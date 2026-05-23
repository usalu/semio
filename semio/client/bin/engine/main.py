# #region 📊Header

# 2025 Ueli Saluz <ueli@semio-tech.com>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Affero General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.
# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Affero General Public License for more details.
# You should have received a copy of the GNU Affero General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# #endregion 📊Header

# #region ⭐Imports
# Imports MUST include all dependencies for store, assistant, GraphQL, REST, MCP, and engine modules.
from __future__ import annotations

import abc
import argparse
import contextlib
import copy
import datetime
import difflib
import enum
import functools
import importlib.util as _ilu
import io
import json
import logging
import multiprocessing
import os
import pathlib
import shutil
import subprocess
import signal
import sqlite3
import sys
import tempfile
import typing
import uuid
import zipfile

import fastapi
import jinja2
import lark
import pydantic
import requests
import starlette.applications
import starlette.middleware.cors
import uvicorn
from ariadne import InterfaceType, MutationType, ObjectType, QueryType, ScalarType, load_schema_from_path, make_executable_schema
from ariadne.asgi import GraphQL
from mcp.server.fastmcp import Context, FastMCP
from mcp.types import CallToolResult, EmbeddedResource, TextContent, TextResourceContents

try:
    import openai  # type: ignore
except Exception:  # pragma: no cover
    openai = None

_semio_core_path = str(pathlib.Path(__file__).parent.parent.parent / "lib" / "py" / "main.py")
_semio_core_spec = _ilu.spec_from_file_location("semio_core", _semio_core_path)
_semio_core = _ilu.module_from_spec(_semio_core_spec)
sys.modules["semio_core"] = _semio_core
_semio_core_spec.loader.exec_module(_semio_core)
from semio_core import (
    DEBUG_LOG_FILE,
    ENCODED_NAME_AND_VARIANT_AND_VIEW_PATH,
    ENCODED_NAME_AND_VARIANT_PATH,
    ENCODED_PATH,
    ENCODING_REGEX,
    HOST,
    KIT_LOCAL_FILENAME,
    KIT_LOCAL_FOLDERNAME,
    MAX_REQUEST_BODY_SIZE,
    PORT,
    RELEASE,
    USER_FOLDER,
    VERSION,
    Attribute,
    AuthenticationError,
    Author,
    AuthTokenNotFound,
    ClientError,
    CodeUnreachable,
    Connection,
    Connector,
    Coordinate,
    Design,
    DesignContext,
    DesignInput,
    DesignOutput,
    DesignPrediction,
    Error,
    FeatureNotYetSupported,
    InvalidAuthToken,
    Kit,
    KitAlreadyExists,
    KitContext,
    KitInput,
    KitNotFound,
    KitOutput,
    KitZipDoesNotContainSemioFolder,
    LocalKitUriIsNotAbsolute,
    Location,
    Representation,
    OnlyRemoteKitsCanBeCached,
    Piece,
    Plane,
    Point,
    RemoteKitsNotYetSupported,
    RemoteKitUriNotValid,
    ServerError,
    ServerUnreachable,
    Side,
    Type,
    TypeContext,
    TypeHasNotAllUsedConnectors,
    TypeInput,
    TypeOutput,
    ValidationResult,
    Vector,
    applyKitDiffDict,
    areDesignsInSameFamilyDict,
    areKitDiffsDictEqual,
    areKitsDictEqual,
    areTypesInSameFamilyDict,
    areValidationResultsEqual,
    canUseDesignAsPieceDict,
    changeKeys,
    changeToDict,
    changeValues,
    createClusteredDesignDict,
    decode,
    deletePiecesAndConnectionsInDesignDict,
    flattenDesignReportDict,
    encode,
    expandDesignPiecesDict,
    findAttributeValueDict,
    findPieceTypeInDesignDict,
    findReplaceableTypesForPieceInDesignDict,
    findReplaceableTypesForPiecesInDesignDict,
    findSameFamilyDesignPiecesDict,
    findUsedConnectorsByPieceInDesignDict,
    flattenDesignDict,
    getClusterableGroupsDict,
    getDesignChange,
    getDesignChildrenDict,
    getDesignFamilyDict,
    getDesignSiblingsDict,
    getKitChange,
    getKitDiffDict,
    getPrimitiveDesignDict,
    getPrimitiveTypeDict,
    getTypeChildrenDict,
    getTypeFamilyDict,
    getTypeSiblingsDict,
    designWithDiffDict,
    inverseKitDiffDict,
    logger,
    normalizeAngle,
    parseValidationResult,
    piecesMetadataDict,
    planeFromYAxis,
    replaceClusterWithDesignDict,
    sumQualityInDesignDict,
    validateKitDict,
)

# #endregion ⭐Imports

# #region 🪨Store
# Store MUST provide the data access layer for kit operations via code-based routing.

codeGrammar = (
    """
    code: (ENCODED_STRING)? ("/" (design | type))?
    type: "types" ("/" ENCODED_STRING "," ENCODED_STRING?)?
    design: "designs" ("/" ENCODED_STRING "," ENCODED_STRING? "," ENCODED_STRING?)?
    ENCODED_STRING: /"""
    + ENCODING_REGEX
    + "/"
)

codeParser = lark.Lark(codeGrammar, start="code")


class OperationKind(enum.Enum):
    """🏷️The kind of a store operation.
    """

    KITS = "kits"
    KIT = "kit"
    DESIGNS = "designs"
    DESIGN = "design"
    TYPES = "types"
    TYPE = "type"


class Operation(typing.TypedDict, total=False):
    """👷Typed operation dict produced by OperationBuilder from parsed code grammar.
    `kind` is always present. Other fields depend on the kind.
    """

    kind: typing.Required[OperationKind]
    kitUri: str
    designName: str
    designVariant: str
    designView: str
    typeName: str
    typeVariant: str


class TransactionChange(typing.TypedDict):
    """💿A single recorded change within a transaction.
    """

    kind: str
    before_has_kit: bool
    after_has_kit: bool
    forward_diff: dict | None
    backward_diff: dict | None


class Transaction(typing.TypedDict):
    """🪪An active MCP session transaction tracking kit changes for rollback.
    """

    active: bool
    started_at: str
    changes: list[TransactionChange]


class OperationBuilder(lark.Transformer):
    """📐Lark transformer that builds operation dicts from parsed code grammar trees.
    Callers MUST pass a valid parse tree from codeParser.
    """

    def code(self, children) -> Operation:
        if len(children) == 0:
            return Operation(kind=OperationKind.KITS)
        kitUri = decode(children[0].value)
        if len(children) == 1:
            return Operation(kind=OperationKind.KIT, kitUri=kitUri)
        code = children[1]
        code["kitUri"] = kitUri
        return code

    def design(self, children) -> Operation:
        if len(children) == 0:
            return Operation(kind=OperationKind.DESIGNS)
        return Operation(
            kind=OperationKind.DESIGN,
            designName=decode(children[0].value),
            designVariant=(decode(children[1].value) if len(children) == 2 else ""),
            designView=(decode(children[2].value) if len(children) == 3 else ""),
        )

    def type(self, children) -> Operation:
        if len(children) == 0:
            return Operation(kind=OperationKind.TYPES)
        return Operation(
            kind=OperationKind.TYPE,
            typeName=decode(children[0].value),
            typeVariant=(decode(children[1].value) if len(children) == 2 else ""),
        )


class StoreKind(enum.Enum):
    """🏪The kind of the store.
    Callers MUST use one of the defined store kinds when selecting a backend.
    """

    DATABASE = "database"
    REST = "rest"
    GRAPHQL = "graphql"


class CommandKind(enum.Enum):
    """🔧 The kind of the command.
    Callers MUST use a valid CommandKind when calling Store.execute.
    """

    QUERY = "query"
    PUT = "put"
    UPDATE = "update"
    DELETE = "delete"


class Store(abc.ABC):
    """🏛️Abstract base class for all store backends.
    Subclasses MUST implement initialize, get, put, update, and delete methods.
    """

    uri: str

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def execute(self, command: CommandKind = CommandKind.QUERY, code: str = "", input: str = "") -> typing.Any:
        """❕ Execute a command on the store."""
        codeTree = codeParser.parse(code)
        operation = OperationBuilder().transform(codeTree)
        if command == CommandKind.QUERY:
            return self.get(operation)
        elif command == CommandKind.PUT:
            return self.put(operation, input)
        elif command == CommandKind.UPDATE:
            return self.update(operation, input)
        elif command == CommandKind.DELETE:
            return self.delete(operation)
        else:
            raise CodeUnreachable()

    @abc.abstractmethod
    def initialize(self: "Store") -> None:
        """🏗️Initialize the store and perform nothing if was already initialized."""
        pass

    @abc.abstractmethod
    def get(cls: "Store", operation: Operation) -> typing.Any:
        """🔍 Get an entity from the store."""
        pass

    @abc.abstractmethod
    def put(cls: "Store", operation: Operation, input: str) -> typing.Any:
        """📥 Put an entity in the store."""
        pass

    @abc.abstractmethod
    def update(cls: "Store", operation: Operation, input: str) -> typing.Any:
        """🔄 Update an entity in the store."""
        pass

    @abc.abstractmethod
    def delete(cls: "Store", operation: Operation) -> typing.Any:
        """🗑 Delete an entity from the store."""
        pass


class DatabaseStore(Store, abc.ABC):
    """🗄️Abstract database-backed store using raw SQL via sqlite3.
    Stores kit data as JSON blobs. No ORM.
    Subclasses MUST implement the fromUri classmethod to construct from a URI.
    """

    db_path: str

    def __init__(self, uri: str, db_path: str) -> None:
        super().__init__(uri)
        self.db_path = db_path

    def _connect(self) -> sqlite3.Connection:
        return sqlite3.connect(self.db_path)

    def initialized(self) -> bool:
        if not os.path.exists(self.db_path):
            return False
        try:
            with self._connect() as conn:
                cursor = conn.execute("SELECT name FROM sqlite_master WHERE type='table' AND name='kit'")
                return cursor.fetchone() is not None
        except sqlite3.OperationalError:
            return False

    @classmethod
    @abc.abstractmethod
    def fromUri(cls, uri: str) -> "DatabaseStore":
        """🔧 Get a store from the uri."""
        pass

    def postDeleteKit(self) -> None:
        return None

    def get(self, operation: Operation) -> typing.Any:
        kitUri = operation["kitUri"]
        kind = operation["kind"]
        if not self.initialized():
            raise KitNotFound(kitUri)
        try:
            with self._connect() as conn:
                cursor = conn.execute("SELECT data FROM kit WHERE uri = ?", (kitUri,))
                row = cursor.fetchone()
        except sqlite3.OperationalError:
            raise KitNotFound(kitUri)
        if row is None:
            raise KitNotFound(kitUri)
        kit_data = json.loads(row[0])
        match kind:
            case OperationKind.KIT:
                return KitOutput.representation_validate(kit_data)
            case OperationKind.DESIGN:
                raise FeatureNotYetSupported()
            case OperationKind.TYPE:
                raise FeatureNotYetSupported()
            case _:
                raise FeatureNotYetSupported()

    def put(
        self,
        operation: Operation,
        input: KitInput | DesignInput | TypeInput,
    ) -> typing.Any:
        kitUri = operation["kitUri"]
        kind = operation["kind"]

        if kind == OperationKind.KIT:
            self.initialize()
            dump = input.representation_dump()
            dump["uri"] = kitUri
            with self._connect() as conn:
                cursor = conn.execute("SELECT 1 FROM kit WHERE uri = ?", (kitUri,))
                if cursor.fetchone() is not None:
                    raise KitAlreadyExists(kitUri)
                conn.execute(
                    "INSERT INTO kit (uri, data) VALUES (?, ?)",
                    (kitUri, json.dumps(dump)),
                )
            return KitOutput.representation_validate(dump)

        if not self.initialized():
            raise KitNotFound(kitUri)

        with self._connect() as conn:
            cursor = conn.execute("SELECT data FROM kit WHERE uri = ?", (kitUri,))
            row = cursor.fetchone()
            if row is None:
                raise KitNotFound(kitUri)
            kit_data = json.loads(row[0])

            match kind:
                case OperationKind.DESIGN:
                    design_dump = input.representation_dump()
                    designs = kit_data.get("designs", [])
                    designs = [d for d in designs if not (d.get("name") == input.name and d.get("variant") == input.variant and d.get("view") == input.view)]
                    designs.append(design_dump)
                    kit_data["designs"] = designs
                    conn.execute(
                        "UPDATE kit SET data = ? WHERE uri = ?",
                        (json.dumps(kit_data), kitUri),
                    )
                case OperationKind.TYPE:
                    type_dump = input.representation_dump()
                    types = kit_data.get("types", [])
                    types = [t for t in types if not (t.get("name") == input.name and t.get("variant") == input.variant)]
                    types.append(type_dump)
                    kit_data["types"] = types
                    conn.execute(
                        "UPDATE kit SET data = ? WHERE uri = ?",
                        (json.dumps(kit_data), kitUri),
                    )
                case _:
                    raise FeatureNotYetSupported()

    def update(self, operation: Operation, input: str) -> typing.Any:
        raise FeatureNotYetSupported()

    def delete(self, operation: Operation) -> typing.Any:
        kitUri = operation["kitUri"]
        kind = operation["kind"]
        if not self.initialized():
            raise KitNotFound(kitUri)

        with self._connect() as conn:
            cursor = conn.execute("SELECT data FROM kit WHERE uri = ?", (kitUri,))
            row = cursor.fetchone()
            if row is None:
                raise KitNotFound(kitUri)

            match kind:
                case OperationKind.KIT:
                    conn.execute("DELETE FROM kit WHERE uri = ?", (kitUri,))
                case OperationKind.DESIGN:
                    kit_data = json.loads(row[0])
                    designs = kit_data.get("designs", [])
                    kit_data["designs"] = [d for d in designs if not (d.get("name") == operation["designName"] and d.get("variant") == operation["designVariant"] and d.get("view") == operation["designView"])]
                    conn.execute(
                        "UPDATE kit SET data = ? WHERE uri = ?",
                        (json.dumps(kit_data), kitUri),
                    )
                case OperationKind.TYPE:
                    kit_data = json.loads(row[0])
                    types = kit_data.get("types", [])
                    kit_data["types"] = [t for t in types if not (t.get("name") == operation["typeName"] and t.get("variant") == operation["typeVariant"])]
                    conn.execute(
                        "UPDATE kit SET data = ? WHERE uri = ?",
                        (json.dumps(kit_data), kitUri),
                    )
                case _:
                    raise FeatureNotYetSupported()

    def apply_diff(self, kitUri: str, diff: dict) -> dict:
        """🔷Apply a kit diff directly via SQL. Loads kit JSON, applies diff, stores back.
        Returns the updated kit dict.
        """
        with self._connect() as conn:
            cursor = conn.execute("SELECT data FROM kit WHERE uri = ?", (kitUri,))
            row = cursor.fetchone()
            if row is None:
                raise KitNotFound(kitUri)
            kit_data = json.loads(row[0])
            updated = applyKitDiffDict(kit_data, diff)
            conn.execute(
                "UPDATE kit SET data = ? WHERE uri = ?",
                (json.dumps(updated), kitUri),
            )
            return updated


class SSLMode(enum.Enum):
    """🔒 The security level of the session
    Callers MUST select the appropriate SSL mode for the target database security policy.
    """

    DISABLE = "disable"
    ALLOW = "allow"
    PREFER = "prefer"
    REQUIRE = "require"
    VERIFY_CA = "verify-ca"
    VERIFY_FULL = "verify-full"


def cacheDir(remoteUri: str) -> str:
    """📂Returns the local cache directory path for a remote kit URI.
    Callers MUST provide a valid remote URI string.
    """
    cacheDir = os.path.expanduser("~/.semio/cache")
    encodedUri = encode(remoteUri)
    return os.path.join(cacheDir, encodedUri)


def cache(remoteUri: str) -> str:
    """📦Cache a remote kit and delete the existing cache if it was already cached.
    Callers MUST provide a URI starting with http and ending with .zip.
    """
    if not (remoteUri.startswith("http") and remoteUri.endswith(".zip")):
        raise OnlyRemoteKitsCanBeCached(remoteUri)

    path = cacheDir(remoteUri)
    os.makedirs(path, exist_ok=True)
    if os.path.exists(path):
        shutil.rmtree(path)
    os.makedirs(path)

    # TODO: Generalize to non-zip kits.

    try:
        response = requests.get(remoteUri)
        response.raise_for_status()
    except requests.exceptions.HTTPError:
        # TODO: Better error message.
        raise KitNotFound(remoteUri)

    with zipfile.ZipFile(io.BytesIO(response.content)) as zip:
        zip.extractall(path)
        paths = os.listdir(path)
        while ".semio" not in paths:
            if len(paths) != 1:
                raise KitZipDoesNotContainSemioFolder()
            nestedPath = os.path.join(path, paths[0])
            nestedDirectories = os.listdir(nestedPath)
            for nestedDirectory in nestedDirectories:
                shutil.move(os.path.join(nestedPath, nestedDirectory), path)
            os.rmdir(nestedPath)
            paths = os.listdir(path)

    return path


class SqliteStore(DatabaseStore):
    """📄SQLite-backed store that persists kit data as JSON in a local .semio database file.
    Callers MUST use fromUri to construct instances with a valid local path.
    """

    kit_dir: pathlib.Path

    def __init__(self, uri: str, db_path: str, kit_dir: pathlib.Path) -> None:
        super().__init__(uri, db_path)
        self.kit_dir = kit_dir

    @classmethod
    def fromUri(cls, uri: str, path: str = "") -> "SqliteStore":
        if path == "":
            path = uri
        kit_dir = pathlib.Path(path) / pathlib.Path(KIT_LOCAL_FOLDERNAME)
        db_path = str(kit_dir / pathlib.Path(KIT_LOCAL_FILENAME))
        store = SqliteStore(uri, db_path, kit_dir)
        if store.initialized():
            try:
                with store._connect() as conn:
                    cursor = conn.execute("SELECT uri FROM kit LIMIT 1")
                    row = cursor.fetchone()
                    if row and row[0] != uri:
                        conn.execute("UPDATE kit SET uri = ? WHERE uri = ?", (uri, row[0]))
            except sqlite3.OperationalError:
                pass
        return store

    def initialize(self) -> None:
        os.makedirs(str(self.kit_dir), exist_ok=True)
        with self._connect() as conn:
            conn.execute("""
                CREATE TABLE IF NOT EXISTS kit (
                    uri TEXT NOT NULL PRIMARY KEY,
                    data TEXT NOT NULL
                )
            """)

    def postDeleteKit(self) -> None:
        os.kill(os.getpid(), signal.SIGTERM)


class PostgresStore(DatabaseStore):
    """🔌PostgreSQL-backed store for remote database connections.
    Callers MUST NOT use this class until PostgreSQL support is implemented.
    """

    @classmethod
    def fromUri(cls, uri: str):
        raise FeatureNotYetSupported()

    def initialize(self) -> None:
        raise FeatureNotYetSupported()


# #region 🪩Auth
# Auth MUST provide credential management for remote server authentication using Bearer tokens.

AUTH_FILE = os.path.join(os.path.expanduser(USER_FOLDER), "auth.json")


def _load_auth() -> dict:
    """🔶Load auth credentials from the auth file.
    Returns dict mapping serverUrl -> {token, email}.
    """
    if os.path.exists(AUTH_FILE):
        with open(AUTH_FILE, "r", encoding="utf-8") as f:
            return json.load(f)
    return {}


def _save_auth(auth: dict) -> None:
    """🔹Save auth credentials to the auth file.
    Callers MUST provide a dict mapping serverUrl -> {token, email}.
    """
    os.makedirs(os.path.dirname(AUTH_FILE), exist_ok=True)
    with open(AUTH_FILE, "w", encoding="utf-8") as f:
        json.dump(auth, f, indent=2)


def login(serverUrl: str, email: str, password: str) -> dict:
    """🔐 Login to a remote server and store the auth token.
    Callers MUST provide a valid server URL, email and password.
    Returns {ok, serverUrl, email, token} on success.
    """
    serverUrl = serverUrl.rstrip("/")
    try:
        response = requests.post(
            f"{serverUrl}/auth/login",
            json={"email": email, "password": password},
            timeout=30,
        )
        response.raise_for_status()
        data = response.json()
        token = data.get("token", "")
        if not token:
            raise AuthenticationError()
        auth = _load_auth()
        auth[serverUrl] = {"token": token, "email": email}
        _save_auth(auth)
        return {"ok": True, "serverUrl": serverUrl, "email": email, "token": token}
    except requests.exceptions.ConnectionError:
        raise ServerUnreachable(serverUrl)
    except requests.exceptions.HTTPError as e:
        if e.response is not None and e.response.status_code in (401, 403):
            raise InvalidAuthToken(serverUrl)
        raise ServerUnreachable(serverUrl)


def logout(serverUrl: str) -> dict:
    """🔓 Logout from a remote server and remove the stored token.
    Callers MUST provide a valid server URL.
    Returns {ok, serverUrl} on success.
    """
    serverUrl = serverUrl.rstrip("/")
    auth = _load_auth()
    auth.pop(serverUrl, None)
    _save_auth(auth)
    return {"ok": True, "serverUrl": serverUrl}


def getAuthToken(serverUrl: str) -> str:
    """🔑 Get the stored auth token for a server.
    Raises AuthTokenNotFound if no token is stored.
    """
    serverUrl = serverUrl.rstrip("/")
    auth = _load_auth()
    entry = auth.get(serverUrl)
    if not entry or not entry.get("token"):
        raise AuthTokenNotFound(serverUrl)
    return entry["token"]


def getAuthStatus(serverUrl: str) -> dict:
    """📋 Get the auth status for a server.
    Returns {authenticated, serverUrl, email} without raising.
    """
    serverUrl = serverUrl.rstrip("/")
    auth = _load_auth()
    entry = auth.get(serverUrl)
    if entry and entry.get("token"):
        return {"authenticated": True, "serverUrl": serverUrl, "email": entry.get("email", "")}
    return {"authenticated": False, "serverUrl": serverUrl, "email": ""}


# #endregion 🪩Auth


class RemoteStore(Store):
    """🖥️REST-backed store that proxies kit operations to a remote semio hub.
    Callers MUST call login() first to authenticate with the remote hub.
    """

    serverUrl: str
    kitUri: str

    def __init__(self, uri: str, serverUrl: str, kitUri: str) -> None:
        super().__init__(uri)
        self.serverUrl = serverUrl
        self.kitUri = kitUri

    @classmethod
    def fromUri(cls, uri: str) -> "RemoteStore":
        """🔧 Construct a RemoteStore from a remote URI.
        URI format: serverUrl + /api/kits/ + encodedKitUri
        """
        if "/api/kits/" not in uri:
            raise RemoteKitUriNotValid(uri)
        idx = uri.index("/api/kits/")
        serverUrl = uri[:idx]
        encodedKitUri = uri[idx + len("/api/kits/") :]
        kitUri = decode(encodedKitUri)
        return cls(uri, serverUrl, kitUri)

    def _headers(self) -> dict:
        """📨Get authorization headers for remote requests."""
        token = getAuthToken(self.serverUrl)
        return {"Authorization": f"Bearer {token}", "Content-Type": "application/json"}

    def _api_url(self, path: str = "") -> str:
        """🌐Build API URL for a kit operation."""
        base = f"{self.serverUrl}/api/kits/{encode(self.kitUri)}"
        if path:
            return f"{base}/{path}"
        return base

    def initialize(self) -> None:
        """💻Remote kits are initialized on the server side."""
        pass

    def get(self, operation: Operation) -> typing.Any:
        """🔍 Get an entity from the remote store."""
        kind = operation["kind"]
        try:
            if kind == OperationKind.KIT:
                response = requests.get(self._api_url(), headers=self._headers(), timeout=30)
                response.raise_for_status()
                return KitOutput.representation_validate(response.json())
            else:
                raise FeatureNotYetSupported()
        except requests.exceptions.ConnectionError:
            raise ServerUnreachable(self.serverUrl)
        except requests.exceptions.HTTPError as e:
            if e.response is not None and e.response.status_code == 401:
                raise InvalidAuthToken(self.serverUrl)
            if e.response is not None and e.response.status_code == 404:
                raise KitNotFound(self.kitUri)
            raise ServerUnreachable(self.serverUrl)

    def put(self, operation: Operation, input: KitInput | DesignInput | TypeInput) -> typing.Any:
        """📥 Put an entity in the remote store."""
        kind = operation["kind"]
        try:
            if kind == OperationKind.KIT:
                response = requests.put(
                    self._api_url(),
                    json=input.representation_dump() if hasattr(input, "representation_dump") else input,
                    headers=self._headers(),
                    timeout=30,
                )
                response.raise_for_status()
                return None
            elif kind == OperationKind.TYPE:
                typeName = encode(operation.get("typeName", ""))
                typeVariant = encode(operation.get("typeVariant", ""))
                path = f"types/{typeName},{typeVariant}"
                response = requests.put(
                    self._api_url(path),
                    json=input.representation_dump() if hasattr(input, "representation_dump") else input,
                    headers=self._headers(),
                    timeout=30,
                )
                response.raise_for_status()
                return None
            elif kind == OperationKind.DESIGN:
                designName = encode(operation.get("designName", ""))
                designVariant = encode(operation.get("designVariant", ""))
                designView = encode(operation.get("designView", ""))
                path = f"designs/{designName},{designVariant},{designView}"
                response = requests.put(
                    self._api_url(path),
                    json=input.representation_dump() if hasattr(input, "representation_dump") else input,
                    headers=self._headers(),
                    timeout=30,
                )
                response.raise_for_status()
                return None
            else:
                raise FeatureNotYetSupported()
        except requests.exceptions.ConnectionError:
            raise ServerUnreachable(self.serverUrl)
        except requests.exceptions.HTTPError as e:
            if e.response is not None and e.response.status_code == 401:
                raise InvalidAuthToken(self.serverUrl)
            raise ServerUnreachable(self.serverUrl)

    def update(self, operation: Operation, input: str) -> typing.Any:
        """🔄 Update an entity in the remote store."""
        raise FeatureNotYetSupported()

    def delete(self, operation: Operation) -> typing.Any:
        """🗑 Delete an entity from the remote store."""
        kind = operation["kind"]
        try:
            if kind == OperationKind.KIT:
                response = requests.delete(self._api_url(), headers=self._headers(), timeout=30)
                response.raise_for_status()
                return None
            elif kind == OperationKind.TYPE:
                typeName = encode(operation.get("typeName", ""))
                typeVariant = encode(operation.get("typeVariant", ""))
                path = f"types/{typeName},{typeVariant}"
                response = requests.delete(self._api_url(path), headers=self._headers(), timeout=30)
                response.raise_for_status()
                return None
            elif kind == OperationKind.DESIGN:
                designName = encode(operation.get("designName", ""))
                designVariant = encode(operation.get("designVariant", ""))
                designView = encode(operation.get("designView", ""))
                path = f"designs/{designName},{designVariant},{designView}"
                response = requests.delete(self._api_url(path), headers=self._headers(), timeout=30)
                response.raise_for_status()
                return None
            else:
                raise FeatureNotYetSupported()
        except requests.exceptions.ConnectionError:
            raise ServerUnreachable(self.serverUrl)
        except requests.exceptions.HTTPError as e:
            if e.response is not None and e.response.status_code == 401:
                raise InvalidAuthToken(self.serverUrl)
            raise ServerUnreachable(self.serverUrl)


@functools.lru_cache
def StoreFactory(uri: str) -> Store:
    """🏭 Get a store from the uri. This store doesn't need to exist yet as long as it can be created.
    Callers MUST provide either an absolute local path, an http URL ending in .zip (cached), or a remote server URI.
    Remote server URIs have the format: http(s)://server/api/kits/encodedKitUri
    """
    if os.path.isabs(uri):
        return SqliteStore.fromUri(uri)
    if uri.startswith("http"):
        if uri.endswith(".zip"):
            path = cacheDir(uri)
            if not os.path.exists(path):
                cache(uri)
            return SqliteStore.fromUri(uri, path)
        if "/api/kits/" in uri:
            return RemoteStore.fromUri(uri)
        raise RemoteKitUriNotValid(uri)
    raise LocalKitUriIsNotAbsolute(uri)


def storeAndOperationFromCode(code: str) -> tuple[Store, dict]:
    """🔤Parses a code string into a store instance and operation dict.
    Callers MUST provide a valid code string matching the code grammar.
    """
    codeTree = codeParser.parse(code)
    operation = OperationBuilder().transform(codeTree)
    store = StoreFactory(operation["kitUri"])
    return store, operation


def get(code: str, cache=False) -> typing.Any:
    """🔍 Get an entity from the store.
    Callers MUST provide a valid code string with an encoded kit URI.
    """
    store, operation = storeAndOperationFromCode(code)
    return store.get(operation)


def put(code: str, input: str) -> typing.Any:
    """📥 Put an entity in the store.
    Callers MUST provide a valid code string and matching input data.
    """
    store, operation = storeAndOperationFromCode(code)
    return store.put(operation, input)


def delete(code: str) -> typing.Any:
    """🗑 Delete an entity from the store.
    Callers MUST provide a valid code string referencing an existing entity.
    """
    store, operation = storeAndOperationFromCode(code)
    return store.delete(operation)


# #endregion 🪨Store

# #region 🎗️Assistant
# Assistant MUST provide AI-powered design prediction using OpenAI structured outputs.


def encodeForPrompt(context: str):
    """📝Sanitizes a context string for use in AI prompts by replacing delimiters.
    Callers MUST pass a string that will be embedded in a prompt template.
    """
    return context.replace(";", ",").replace("\n", " ")


def replaceDefault(context: str, default: str):
    """🔸Substitutes an empty context string with the provided default value.
    Callers MUST provide a non-None default string.
    """
    if context == "":
        return context.replace("", default)
    return context


def encodeType(type: TypeContext):
    """🎨Encodes a TypeContext for prompt rendering by replacing empty values with defaults.
    Callers MUST provide a valid TypeContext with populated connectors.
    """
    typeClone = type.representation_copy(deep=True)
    typeClone.variant = replaceDefault(typeClone.variant, "DEFAULT")
    typeClone.description = encodeForPrompt(typeClone.description) if typeClone.description != "" else "NO_DESCRIPTION"
    for connector in typeClone.connectors:
        connector.id_ = replaceDefault(connector.id_, "DEFAULT")

    return typeClone


def decodeDesign(design: dict):
    """📩Decodes a raw AI response dict into a DesignPrediction representation.
    Callers MUST provide a dict with pieces and connections arrays.
    """
    decodedDesign = {
        "pieces": [
            {
                "id_": p["id"] if p["id"] != "DEFAULT" else "",
                "type": {
                    "name": p["typeName"],
                    "variant": (p["typeVariant"] if p["typeVariant"] != "DEFAULT" else ""),
                },
            }
            for p in design["pieces"]
        ],
        "connections": [
            {
                "parent": {
                    "piece": {
                        "id_": (c["parentPieceId"] if c["parentPieceId"] != "DEFAULT" else ""),
                    },
                    "connector": {
                        "id_": (c["parentPieceTypePortId"] if c["parentPieceTypePortId"] != "DEFAULT" else ""),
                    },
                },
                "child": {
                    "piece": {
                        "id_": (c["childPieceId"] if c["childPieceId"] != "DEFAULT" else ""),
                    },
                    "connector": {
                        "id_": (c["childPieceTypePortId"] if c["childPieceTypePortId"] != "DEFAULT" else ""),
                    },
                },
                "gap": c["gap"],
                "shift": c["shift"],
                "rise": c["rise"],
                "rotation": normalizeAngle(c["rotation"]),
                "turn": normalizeAngle(c["turn"]),
                "tilt": normalizeAngle(c["tilt"]),
                "x": c["x"],
                "y": c["y"],
            }
            for c in design["connections"]
        ],
    }
    return DesignPrediction.parse(decodedDesign)


def healDesign(design: DesignPrediction, types: list[TypeContext]):
    """🩺 Heal a design by replacing missing type variants with the first variant.
    TODO: Replace prototype healing with one that makes more for every single property.
    Callers MUST provide a design with pieces referencing types available in the types list.
    """
    designClone = design.representation_copy(deep=True)
    typeD = {}
    connectorD = {}
    pieceD = {}
    for type in types:
        if type.name not in typeD:
            typeD[type.name] = {}
            connectorD[type.name] = {}
        typeD[type.name][type.variant] = type
        if type.variant not in connectorD[type.name]:
            connectorD[type.name][type.variant] = {}
        for connector in type.connectors:
            connectorD[type.name][type.variant][connector.id_] = connector
    # TODO: Try closest embedding instead of smallest Levenshtein distance.
    for piece in designClone.pieces:
        pieceD[piece.id_] = piece
        if piece.type and piece.type.name not in typeD:
            # TODO: Remove piece if type name is not found instead of taking the first.
            try:
                piece.type.name = difflib.get_close_matches(piece.type.name, typeD.keys(), n=1)[0]
            except Error:
                piece.type.name = list(typeD.keys())[0]
        if piece.type and piece.type.name and piece.type.variant not in typeD[piece.type.name]:
            try:
                piece.type.variant = difflib.get_close_matches(piece.type.variant, typeD[piece.type.name].keys(), n=1)[0]
            except Error:
                piece.type.variant = list(typeD[piece.type.name].keys())[0]

    validConnections = []
    for connection in designClone.connections:
        if connection.parent.piece.id_ not in pieceD:
            try:
                connection.parent.piece.id_ = difflib.get_close_matches(connection.parent.piece.id_, pieceD.keys(), n=1)[0]
            except Error:
                continue
        if connection.child.piece.id_ not in pieceD:
            try:
                connection.child.piece.id_ = difflib.get_close_matches(connection.child.piece.id_, pieceD.keys(), n=1)[0]
            except Error:
                continue
        parentType = typeD[pieceD[connection.parent.piece.id_].type.name][pieceD[connection.parent.piece.id_].type.variant]
        childType = typeD[pieceD[connection.child.piece.id_].type.name][pieceD[connection.child.piece.id_].type.variant]

        if connection.parent.connector is not None and connection.parent.connector.id_ not in connectorD[parentType.name][parentType.variant]:
            connection.parent.connector.id_ = difflib.get_close_matches(
                connection.parent.connector.id_,
                connectorD[parentType.name][parentType.variant].keys(),
                n=1,
            )[0]
        if connection.child.connector is not None and connection.child.connector.id_ not in connectorD[childType.name][childType.variant]:
            connection.child.connector.id_ = difflib.get_close_matches(
                connection.child.connector.id_,
                connectorD[childType.name][childType.variant].keys(),
                n=1,
            )[0]
        validConnections.append(connection)
    designClone.connections = validConnections

    designClone.connections = [c for c in designClone.connections if c.parent.piece.id_ != c.child.piece.id_]

    designClone.pieces = [p for p in designClone.pieces if any(c for c in designClone.connections if c.parent.piece.id_ == p.id_ or c.child.piece.id_ == p.id_)]
    return designClone


try:
    openaiClient = openai.Client() if openai is not None else None
except Exception:
    openaiClient = None

systemPrompt = """You are a kit-of-parts design assistant.
Constraints:
Every piece MUST have a type that exists. The type name and type variant MUST match.
Two pieces are different when they have a different type name or type variant.
Two types are different when they have a different name or different variant.
Every parent-side and child-side piece MUST be part of the pieces of the design. The ids MUST match.
The connector on each side MUST exist in the kind of that piece. The ids MUST match.
The two connectors SHOULD match.
If the connectors have a port, they should be compatible.
If one connector has the other connector as ocompatible that's enough.
Every piece in the design MUST be connected to at least one other piece.
One piece is the root piece of the design. The connections MUST form a tree.
Ids SHOULD be abreviated and don't have to be globally unique.
Rotation, tilt, gap, shift SHOULD NOT be added unless specifically instructed.
The diagram is only a nice 2D representation of the design and does not change the design.
When a piece is [on, next to, above, below, ...] another piece, there SHOULD be a connection with that piece as parent and the other as child when the relationship is asymmetric.
When a piece fits to a connector of another piece, encode parent and child so the attachment direction matches the physical intent."""

designGenerationPromptTemplate = jinja2.Template(
    """Your task is to help to puzzle together a design.

TYPE{NAME;VARIANT;DESCRIPTION;CONNECTORS}
CONNECTORECTOR{ID;DESCRIPTION,FAMILY,COMPATIBLEFAMILIES}
COMPATIBLEFAMILY{NAME}

Available types:
{% for type in types %}
{% raw %}{{% endraw -%}
{{ type.name }};{{ type.variant }};{{ type.description }};
{%- for connector in type.connectors %}
{%- raw %}{{% endraw -%}{{ connector.id_ }};{{ connector.description }};{{ connector.port }}
{%- for compatiblePort in connector.compatiblePorts %}
{%- raw %}{{% endraw -%}
{{ compatiblePort }}
{%- endfor -%}
{%- raw %}}{% endraw -%}
{%- endfor -%}
{%- raw %}}{% endraw -%}
{% endfor %}

The generated design should match this description:
{{ description }}"""
)

designResponseFormat = json.loads(
    """
{
    "name": "design",
    "strict": true,
    "schema": {
        "type": "object",
        "description": "A design is a collection of pieces that are connected.",
        "properties": {
            "pieces": {
                "type": "array",
                "items": {
                    "type": "object",
                    "description": " A piece is a 3d-instance of a type in a design.",
                    "properties": {
                        "id": {
                            "type": "string"
                        },
                        "typeName": {
                            "type": "string"
                        },
                        "typeVariant": {
                            "type": "string"
                        }
                    },
                    "required": [
                        "id",
                        "typeName",
                        "typeVariant"
                    ],
                    "additionalProperties": false
                }
            },
            "connections": {
                "type": "array",
                "items": {
                    "type": "object",
                    "description": "A directed connection: parent side attaches to child side.",
                    "properties": {
                        "parentPieceId": {
                            "type": "string"
                        },
                        "parentPieceTypePortId": {
                            "type": "string"
                        },
                        "childPieceId": {
                            "type": "string"
                        },
                        "childPieceTypePortId": {
                            "type": "string"
                        },
                        "gap": {
                            "type": "number",
                            "description": "The optional longitudinal gap (applied after rotation and tilt in connector direction) from parent toward child. "
                        },
                        "shift": {
                            "type": "number",
                            "description": "The optional lateral shift (applied after the rotation, the turn and the tilt in the plane) between parent and child sides."
                        },
                        "rise": {
                            "type": "number",
                            "description": "The optional vertical rise in connector direction between parent and child. Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."
                        },
                        "rotation": {
                            "type": "number",
                            "description": "The optional horizontal rotation in connector direction between parent and child in degrees."
                        },
                        "turn": {
                            "type": "number",
                            "description": "The optional turn perpendicular to the connector direction (applied after rotation and the turn) between parent and child in degrees.  Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."
                        },
                        "tilt": {
                            "type": "number",
                            "description": "The optional horizontal tilt perpendicular to the connector direction (applied after rotation and the turn) between parent and child in degrees."
                        },
                        "x": {
                            "description": "The optional offset in x direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.",
                            "type": "number"
                        },
                        "y": {
                            "description": "The optional offset in y direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.",
                            "type": "number"
                        }
                    },
                    "required": [
                        "parentPieceId",
                        "parentPieceTypePortId",
                        "childPieceId",
                        "childPieceTypePortId",
                        "gap",
                        "shift",
                        "rise",
                        "rotation",
                        "turn",
                        "tilt",
                        "x",
                        "y"
                    ],
                    "additionalProperties": false
                }
            }
        },
        "required": [
            "pieces",
            "connections"
        ],
        "additionalProperties": false
    }
}"""
)


def predictDesign(description: str, types: list[TypeContext], design: DesignInput | None = None) -> DesignPrediction:
    """🔮 Predict a design based on a description, the types that should be used and an optional base design.
    Callers MUST ensure the openaiClient is initialized before calling.
    """
    if openaiClient is None:
        raise FeatureNotYetSupported("OpenAI client not available")

    prompt = designGenerationPromptTemplate.render(description=description, types=[encodeType(t) for t in types])
    logger.debug("Generated prompt: {}", prompt)
    try:
        response = openaiClient.chat.completions.create(
            representation="gpt-4o",
            messages=[
                {
                    "role": "system",
                    "content": [
                        {
                            "type": "text",
                            "text": systemPrompt,
                        }
                    ],
                },
                {
                    "role": "user",
                    "content": [
                        {
                            "type": "text",
                            "text": prompt,
                        }
                    ],
                },
            ],
            response_format={
                "type": "json_schema",
                "json_schema": designResponseFormat,
            },
        )
        if response.usage:
            responseDump = {
                "id": response.id,
                "created": response.created,
                "representation": response.representation,
                "object": response.object,
                "system_fingerprint": response.system_fingerprint,
                "usage": {
                    "completion_tokens": response.usage.completion_tokens,
                    "prompt_tokens": response.usage.prompt_tokens,
                    "total_tokens": response.usage.total_tokens,
                },
                "_request_id": response._request_id,
                "choices": [
                    {
                        "finish_reason": c.finish_reason,
                        "message": {
                            "content": c.message.content,
                            "refusal": c.message.refusal,
                            "role": c.message.role,
                        },
                    }
                    for c in response.choices
                ],
            }
            logger.debug("Received response: {}", responseDump)
    except Error:
        logger.error("Error occurred during OpenAI request")
        raise FeatureNotYetSupported("OpenAI request failed")

    logger.debug("Schema: {}", json.dumps(designResponseFormat, indent=4))
    logger.debug("Prompt: {}", prompt)
    logger.debug("System Prompt: {}", systemPrompt)

    result = response.choices[0] if response.choices else None
    if result and result.message.content:
        logger.debug(
            "Predicted Design Raw: {}",
            json.dumps(json.loads(result.message.content), indent=4),
        )

    if result and result.finish_reason == "stop" and result.message.refusal is None and result.message.content:
        design = decodeDesign(json.loads(result.message.content))

        if hasattr(design, "representation_dump"):
            logger.debug("Predicted Design: {}", json.dumps(design.representation_dump(), indent=4))

        healedDesign = healDesign(typing.cast(DesignPrediction, design), types)
        logger.debug(
            "Predicted Design Healed: {}",
            json.dumps(healedDesign.representation_dump(), indent=4),
        )
        return healedDesign

    raise FeatureNotYetSupported("OpenAI response was invalid or incomplete")


# #endregion 🎗️Assistant

# #region 🎬Graphql
# Graphql MUST serve the hand-written SDL next to this module (semio/engine/schema.graphql) with bound resolvers (schema-first).


def _engine_bundle_dir() -> pathlib.Path:
    # Directory containing engine entrypoint and bundled schema.graphql (source tree).
    return pathlib.Path(__file__).resolve().parent


def _graphql_schema_file() -> pathlib.Path:
    # Path to the engine HTTP GraphQL SDL (PyInstaller places schema.graphql at bundle root).
    if getattr(sys, "frozen", False):
        return pathlib.Path(sys._MEIPASS) / "schema.graphql"
    return _engine_bundle_dir() / "schema.graphql"


def _openapi_schema_file() -> pathlib.Path:
    # Path to the canonical OpenAPI document under semio/openapi/ (PyInstaller keeps openapi/ prefix).
    if getattr(sys, "frozen", False):
        return pathlib.Path(sys._MEIPASS) / "openapi" / "schema.json"
    return _engine_bundle_dir().parent / "openapi" / "schema.json"


graphql_datetime_scalar = ScalarType("DateTime")


@graphql_datetime_scalar.serializer
def _serialize_graphql_datetime(value: typing.Any) -> typing.Any:
    if value is None:
        return None
    if isinstance(value, datetime.datetime):
        return value.isoformat()
    return value


graphql_node_iface = InterfaceType("Node")


@graphql_node_iface.type_resolver
def _resolve_graphql_node_type(obj: typing.Any, info: typing.Any, abstract_type: typing.Any) -> str:
    return "Kit"


graphql_kit_type = ObjectType("Kit")


@graphql_kit_type.field("id")
def _resolve_kit_graphql_id(obj: typing.Any, info: typing.Any) -> str:
    if hasattr(obj, "id") and callable(obj.id):
        return typing.cast(str, obj.id())
    u = getattr(obj, "uri", None)
    if u is not None:
        return str(u)
    return ""


graphql_query = QueryType()


@graphql_query.field("kit")
def _resolve_graphql_kit(_: typing.Any, info: typing.Any, uri: str) -> typing.Any:
    return get(encode(uri))


@graphql_query.field("node")
def _resolve_graphql_node(_: typing.Any, info: typing.Any, id: str) -> typing.Any:
    return get(id)


graphql_mutation = MutationType()


@graphql_mutation.field("createKit")
def _resolve_graphql_create_kit(_: typing.Any, info: typing.Any, kit: dict[str, typing.Any]) -> typing.Any:
    ki = KitInput.representation_validate(kit)
    parent = os.path.join(os.path.expanduser(USER_FOLDER), "graphql-kits")
    os.makedirs(parent, exist_ok=True)
    kit_dir = tempfile.mkdtemp(dir=parent)
    code = encode(str(pathlib.Path(kit_dir).resolve()))
    return put(code, ki)


graphql_type_defs = load_schema_from_path(str(_graphql_schema_file()))
graphql_schema = make_executable_schema(
    graphql_type_defs,
    graphql_query,
    graphql_mutation,
    graphql_node_iface,
    graphql_kit_type,
    graphql_datetime_scalar,
)
graphqlSchema = graphql_schema
graphql_http_app = GraphQL(graphql_schema, debug=True)


# #endregion 🎬Graphql

# #region 🌟Rest
# Rest MUST expose kit, type, design, and assistant endpoints via FastAPI.

rest = fastapi.FastAPI(max_request_body_size=MAX_REQUEST_BODY_SIZE)

rest.add_middleware(
    starlette.middleware.cors.CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


class NativeAlgorithmExecuteBody(pydantic.BaseRepresentation):
    """🔺Request body for POST /api/native-algorithms/execute.
    """

    language: typing.Literal["python", "go", "rust", "csharp"]
    operation: typing.Literal["flatten", "delete"]
    kit: dict[str, typing.Any]
    design: dict[str, typing.Any]
    designId: str
    pieceIds: list[str] = []
    connectionIds: list[str] = []


def _semio_repo_root() -> pathlib.Path:
    """🔻Return the semio/ directory containing go, rs, py bundles.
    """
    return pathlib.Path(__file__).resolve().parent.parent


def _normalize_csharp_json_keys(value: typing.Any) -> typing.Any:
    """🔠Match Storybook: first character of each object key lowercased for C# Newtonsoft payloads.
    """
    if isinstance(value, list):
        return [_normalize_csharp_json_keys(v) for v in value]
    if not isinstance(value, dict):
        return value
    out: dict[str, typing.Any] = {}
    for k, v in value.items():
        nk = (k[0].lower() + k[1:]) if k else k
        out[nk] = _normalize_csharp_json_keys(v)
    return out


def _go_native_bridge(payload: dict[str, typing.Any]) -> typing.Any:
    """🔬Run semio/algorithms/native-bridges/go and return parsed JSON result.
    """
    go_root = _semio_repo_root() / "algorithms" / "native-bridges" / "go"
    proc = subprocess.run(
        ["go", "run", "-mod=mod", "."],
        cwd=str(go_root),
        input=json.dumps(payload).encode("utf-8"),
        capture_output=True,
        timeout=300,
        check=False,
        env={**os.environ, "GOWORK": "off"},
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.decode("utf-8", errors="replace") or "go native bridge failed")
    out = json.loads(proc.stdout.decode("utf-8"))
    if not out.get("ok"):
        raise RuntimeError(out.get("error", "go native bridge error"))
    return out.get("result")


def _rust_native_bridge(payload: dict[str, typing.Any]) -> typing.Any:
    """⬛Run semio/algorithms/native-bridges/rs and return parsed JSON result.
    """
    rs_root = _semio_repo_root() / "algorithms" / "native-bridges" / "rs"
    proc = subprocess.run(
        ["cargo", "run", "-q"],
        cwd=str(rs_root),
        input=json.dumps(payload).encode("utf-8"),
        capture_output=True,
        timeout=600,
        check=False,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.decode("utf-8", errors="replace") or "rust native bridge failed")
    out = json.loads(proc.stdout.decode("utf-8"))
    if not out.get("ok"):
        raise RuntimeError(out.get("error", "rust native bridge error"))
    return out.get("result")


def _csharp_native_bridge(payload: dict[str, typing.Any]) -> typing.Any:
    """🔷Run semio/algorithms/native-bridges/csharp and return parsed JSON result.
    """
    cs_root = _semio_repo_root() / "algorithms" / "native-bridges" / "csharp"
    proc = subprocess.run(
        ["dotnet", "run", "--project", "./csharp-native-bridge.csproj", "-q"],
        cwd=str(cs_root),
        input=json.dumps(payload).encode("utf-8"),
        capture_output=True,
        timeout=600,
        check=False,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.decode("utf-8", errors="replace") or "csharp native bridge failed")
    text = proc.stdout.decode("utf-8", errors="replace")
    json_line = next((ln for ln in reversed(text.splitlines()) if ln.strip().startswith("{")), None)
    if not json_line:
        raise RuntimeError("csharp native bridge: no JSON line on stdout")
    out = json.loads(json_line)
    if not out.get("ok"):
        raise RuntimeError(out.get("error", "csharp native bridge error"))
    return _normalize_csharp_json_keys(out.get("result"))


def _dispatch_native_algorithm(body: NativeAlgorithmExecuteBody) -> typing.Any:
    """🗣️Dispatch native algorithm execution by language and operation.
    """
    kit = body.kit
    design = body.design
    dg = body.designId
    lang = body.language
    op = body.operation
    bridge_payload: dict[str, typing.Any] = {
        "op": op,
        "kit": kit,
        "design": design,
        "designId": dg,
        "pieceIds": list(body.pieceIds),
        "connectionIds": list(body.connectionIds),
    }
    if lang == "python":
        if op == "flatten":
            return flattenDesignReportDict(kit, dg)
        return deletePiecesAndConnectionsInDesignDict(kit, design, list(body.pieceIds), list(body.connectionIds))
    if lang == "go":
        return _go_native_bridge(bridge_payload)
    if lang == "rust":
        bridge_in = dict(bridge_payload)
        if op != "delete":
            bridge_in["design"] = None
        return _rust_native_bridge(bridge_in)
    if lang == "csharp":
        return _csharp_native_bridge(bridge_payload)
    raise RuntimeError(f"unsupported native language: {lang}")


@rest.post("/native-algorithms/execute")
async def native_algorithms_execute(body: NativeAlgorithmExecuteBody) -> fastapi.responses.JSONResponse:
    """Execute flatten or delete in python, go, or rust native stacks (typescript runs in the browser).
    """
    try:
        result = _dispatch_native_algorithm(body)
        return fastapi.responses.JSONResponse(content={"result": result})
    except Exception as e:
        return fastapi.responses.JSONResponse(content={"error": str(e)}, status_code=500)


def _build_design_viewer_html() -> str:
    """👁️Build the embeddable MCP App HTML from the built bundle.
    Callers MUST use the returned HTML for the /app/design-viewer endpoint.
    The MCP App is built from mcp-app.tsx which uses @semio/ui components exclusively.
    """
    app_html_path = os.path.join(os.path.dirname(__file__), "dist", "mcp-app.html")
    if os.path.exists(app_html_path):
        with open(app_html_path, "r", encoding="utf-8") as f:
            return f.read()
    return """<!doctype html><html><body><p>MCP App not built. Run: npm run build:mcp-app in semio/engine</p></body></html>"""


def _build_kit_viewer_html() -> str:
    """⬜Build the embeddable kit viewer HTML (SemioKit-only MCP shell) from the same bundle as the design viewer.
    Sets #root data-mcp-viewer to kit so mcp-app.tsx mounts McpKitViewer from @semio/ui.
    Callers MUST use the returned HTML for the /app/kit-viewer endpoint and ui://semio/kit-viewer resource.
    """
    html = _build_design_viewer_html()
    if "MCP App not built" in html:
        return html.replace("MCP App not built", "MCP kit app not built", 1)
    return html.replace('data-mcp-viewer="design"', 'data-mcp-viewer="kit"', 1)


def _build_scene_viewer_html() -> str:
    """🟥Build the embeddable scene viewer HTML from the same bundle, mounting McpSceneViewer (3D only).
    """
    html = _build_design_viewer_html()
    if "MCP App not built" in html:
        return html.replace("MCP App not built", "MCP scene app not built", 1)
    return html.replace('data-mcp-viewer="design"', 'data-mcp-viewer="scene"', 1)


def _build_diagram_viewer_html() -> str:
    """🟧Build the embeddable diagram viewer HTML from the same bundle, mounting McpDiagramViewer (2D only).
    """
    html = _build_design_viewer_html()
    if "MCP App not built" in html:
        return html.replace("MCP App not built", "MCP diagram app not built", 1)
    return html.replace('data-mcp-viewer="design"', 'data-mcp-viewer="diagram"', 1)


@rest.get("/app/design-viewer")
async def app_design_viewer() -> fastapi.Response:
    """Return the embeddable design viewer HTML shell.
    Callers MUST use this endpoint to embed the semio design viewer in an iframe.
    """
    return fastapi.Response(
        content=_build_design_viewer_html(),
        media_type="text/html",
        headers={
            "Content-Security-Policy": "default-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval' blob:; frame-ancestors *; connect-src * data: blob:; img-src * data: blob:; worker-src blob:;",
        },
    )


@rest.get("/app/kit-viewer")
async def app_kit_viewer() -> fastapi.Response:
    """Return the embeddable kit viewer HTML shell (SemioKit from @semio/ui).
    """
    return fastapi.Response(
        content=_build_kit_viewer_html(),
        media_type="text/html",
        headers={
            "Content-Security-Policy": "default-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval' blob:; frame-ancestors *; connect-src * data: blob:; img-src * data: blob:; worker-src blob:;",
        },
    )


@rest.get("/app/scene-viewer")
async def app_scene_viewer() -> fastapi.Response:
    """Return the embeddable scene viewer HTML shell (SemioScene 3D only from @semio/ui).
    """
    return fastapi.Response(
        content=_build_scene_viewer_html(),
        media_type="text/html",
        headers={
            "Content-Security-Policy": "default-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval' blob:; frame-ancestors *; connect-src * data: blob:; img-src * data: blob:; worker-src blob:;",
        },
    )


@rest.get("/app/diagram-viewer")
async def app_diagram_viewer() -> fastapi.Response:
    """Return the embeddable diagram viewer HTML shell (SemioDiagram 2D only from @semio/ui).
    """
    return fastapi.Response(
        content=_build_diagram_viewer_html(),
        media_type="text/html",
        headers={
            "Content-Security-Policy": "default-src 'self' 'unsafe-inline'; script-src 'self' 'unsafe-inline' 'wasm-unsafe-eval' blob:; frame-ancestors *; connect-src * data: blob:; img-src * data: blob:; worker-src blob:;",
        },
    )


@rest.get("/app/payload/{token}")
async def app_payload(token: str) -> fastapi.responses.JSONResponse:
    """Return the full MCP app payload by token. Used by MCP app iframes to bypass host truncation."""
    payload = _mcp_app_payloads.get(token)
    if payload is None:
        return fastapi.responses.JSONResponse({"error": "Payload not found or expired"}, status_code=404)
    return fastapi.responses.JSONResponse(payload, headers={"Access-Control-Allow-Origin": "*"})


@rest.get("/app/files/{file_id}")
async def app_file(file_id: str) -> fastapi.Response:
    """Serve a kit file blob by id. Used by MCP app iframes to load 3D representations."""
    blob = _mcp_app_file_blobs.get(file_id)
    if blob is None:
        return fastapi.Response(content="File not found", status_code=404)
    if blob.startswith("data:"):
        parts = blob.split(",", 1)
        header = parts[0]
        encoded = parts[1] if len(parts) > 1 else ""
        mime = header.split(":")[1].split(";")[0] if ":" in header else "application/octet-stream"
        import base64
        return fastapi.Response(content=base64.b64decode(encoded), media_type=mime, headers={"Access-Control-Allow-Origin": "*"})
    import base64
    return fastapi.Response(content=base64.b64decode(blob), media_type="application/octet-stream", headers={"Access-Control-Allow-Origin": "*"})


@rest.get("/kits/{encodedKitUri}")
async def kit(
    request: fastapi.Request,
    encodedKitUri: ENCODED_PATH,
) -> KitOutput:
    """Retrieves a kit by its encoded URI path.
    Callers MUST provide a valid encoded kit URI in the URL path.
    """
    try:
        return get(request.url.path.removeprefix("/api/kits/"))
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.put("/kits/{encodedKitUri}")
async def create_kit(
    request: fastapi.Request,
    input: KitInput,
    encodedKitUri: ENCODED_PATH,
) -> None:
    """Creates a new kit at the specified encoded URI.
    Callers MUST provide a valid KitInput body and encoded URI.
    """
    try:
        put(request.url.path.removeprefix("/api/kits/"), input)
        return None
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.delete("/kits/{encodedKitUri}")
async def delete_kit(
    request: fastapi.Request,
    encodedKitUri: ENCODED_PATH,
) -> None:
    """Deletes an existing kit at the specified encoded URI.
    Callers MUST provide a valid encoded URI for an existing kit.
    """
    try:
        delete(request.url.path.removeprefix("/api/kits/"))
        return None
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.put("/kits/{encodedKitUri}/types/{encodedTypeNameAndVariant}")
async def put_type(
    request: fastapi.Request,
    input: TypeInput,
    encodedKitUri: ENCODED_PATH,
    encodedTypeNameAndVariant: ENCODED_NAME_AND_VARIANT_PATH,
) -> None:
    """Creates or replaces a type in a kit by encoded URI and type identifier.
    Callers MUST provide a valid TypeInput body with matching name and variant.
    """
    try:
        put(request.url.path.removeprefix("/api/kits/"), input)
        return None
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.delete("/kits/{encodedKitUri}/types/{encodedTypeNameAndVariant}")
async def delete_type(
    request: fastapi.Request,
    encodedKitUri: ENCODED_PATH,
    encodedTypeNameAndVariant: ENCODED_NAME_AND_VARIANT_PATH,
) -> None:
    """Deletes a type from a kit by encoded URI and type identifier.
    Callers MUST provide a valid encoded URI and type name with variant.
    """
    try:
        delete(request.url.path.removeprefix("/api/kits/"))
        return None
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.put("/kits/{encodedKitUri}/designs/{encodedDesignNameAndVariantAndView}")
async def put_design(
    request: fastapi.Request,
    input: DesignInput,
    encodedKitUri: ENCODED_PATH,
    encodedDesignNameAndVariantAndView: ENCODED_NAME_AND_VARIANT_AND_VIEW_PATH,
) -> None:
    """Creates or replaces a design in a kit by encoded URI and design identifier.
    Callers MUST provide a valid DesignInput body with matching name, variant, and view.
    """
    try:
        put(request.url.path.removeprefix("/api/kits/"), input)
        return None
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.delete("/kits/{encodedKitUri}/designs/{encodedDesignNameAndVariantAndView}")
async def delete_design(
    request: fastapi.Request,
    encodedKitUri: ENCODED_PATH,
    encodedDesignNameAndVariantAndView: ENCODED_NAME_AND_VARIANT_AND_VIEW_PATH,
) -> None:
    """Deletes a design from a kit by encoded URI and design identifier.
    Callers MUST provide a valid encoded URI and design name with variant and view.
    """
    try:
        delete(request.url.path.removeprefix("/api/kits/"))
        return None
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.get("/assistant/predictDesign")
async def predict_design(
    request: fastapi.Request,
    description: str = fastapi.Body(...),
    types: list[TypeContext] = fastapi.Body(...),
    design: DesignContext | None = None,
) -> DesignPrediction:
    """Predicts a design via the assistant based on a description and available types.
    Callers MUST provide a description and at least one TypeContext in the request body.
    """
    try:
        return predictDesign(description, types, design)
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.post("/prepare/kit")
async def prepare_kit(request: fastapi.Request, kit: KitInput = fastapi.Body(...)) -> KitContext:
    """Validates and returns a KitContext from the provided KitInput body.
    Callers MUST provide a valid KitInput in the request body.
    """
    try:
        return kit
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


def custom_openapi():
    """Loads the hand-written OpenAPI document from semio/openapi/schema.json and caches it on the app.
    Callers MUST NOT call this directly; it is assigned to rest.openapi.
    """
    if rest.openapi_schema:
        return rest.openapi_schema
    with open(_openapi_schema_file(), encoding="utf-8") as f:
        openapi_schema = json.load(f)
    openapi_schema.setdefault("info", {})["version"] = VERSION
    rest.openapi_schema = openapi_schema
    return rest.openapi_schema


rest.openapi = custom_openapi


# #region 🧩Auth Endpoints
# Auth endpoints MUST expose login, logout and status for remote server authentication.


class LoginRequest(pydantic.BaseRepresentation):
    """📜Login request body.
    """

    serverUrl: str
    email: str
    password: str


class LoginResponse(pydantic.BaseRepresentation):
    """🟪Login response body.
    """

    ok: bool
    serverUrl: str
    email: str
    token: str


class LogoutRequest(pydantic.BaseRepresentation):
    """🟫Logout request body.
    """

    serverUrl: str


class AuthStatusResponse(pydantic.BaseRepresentation):
    """💠Auth status response body.
    """

    authenticated: bool
    serverUrl: str
    email: str


@rest.post("/auth/login")
async def rest_login(request: LoginRequest) -> LoginResponse:
    """Login to a remote server and store the auth token.
    Callers MUST provide serverUrl, email and password.
    """
    try:
        result = login(request.serverUrl, request.email, request.password)
        return LoginResponse(**result)
    except ClientError as e:
        return fastapi.Response(content=str(e), status_code=400)
    except Exception as e:
        return fastapi.Response(content=str(e), status_code=500)


@rest.post("/auth/logout")
async def rest_logout(request: LogoutRequest) -> dict:
    """Logout from a remote server and remove the stored token.
    Callers MUST provide serverUrl.
    """
    try:
        return logout(request.serverUrl)
    except Exception as e:
        return fastapi.Response(content=str(e), status_code=500)


@rest.get("/auth/status")
async def rest_auth_status(serverUrl: str) -> AuthStatusResponse:
    """Get the auth status for a remote server.
    Callers MUST provide serverUrl as a query parameter.
    """
    try:
        result = getAuthStatus(serverUrl)
        return AuthStatusResponse(**result)
    except Exception as e:
        return fastapi.Response(content=str(e), status_code=500)


# #endregion 🧩Auth Endpoints

# #endregion 🌟Rest

# #region ⛩️Mcp
# Mcp MUST expose stateful kit operations via Representation Context Protocol.
# Call start_working_in_local_kit(path) first; then use start_working_in_design/start_working_in_type to scope further.

mcp = FastMCP("semio", stateless_http=False, json_response=True)

_APP_RESOURCE_URI = "ui://semio/design-viewer"
_APP_RESOURCE_META = {"ui": {"resourceUri": _APP_RESOURCE_URI}, "ui/resourceUri": _APP_RESOURCE_URI}
_KIT_APP_RESOURCE_URI = "ui://semio/kit-viewer"
_KIT_APP_RESOURCE_META = {"ui": {"resourceUri": _KIT_APP_RESOURCE_URI}, "ui/resourceUri": _KIT_APP_RESOURCE_URI}
_SCENE_APP_RESOURCE_URI = "ui://semio/scene-viewer"
_SCENE_APP_RESOURCE_META = {"ui": {"resourceUri": _SCENE_APP_RESOURCE_URI}, "ui/resourceUri": _SCENE_APP_RESOURCE_URI}
_DIAGRAM_APP_RESOURCE_URI = "ui://semio/diagram-viewer"
_DIAGRAM_APP_RESOURCE_META = {"ui": {"resourceUri": _DIAGRAM_APP_RESOURCE_URI}, "ui/resourceUri": _DIAGRAM_APP_RESOURCE_URI}


def _mcp_app_html_resource_meta() -> dict[str, typing.Any]:
    """🎁Resource _meta for MCP App HTML: hosts apply _meta.ui.csp to the sandbox (see .repo/✍️/mcp-app.md)."""
    origins = [
        f"http://127.0.0.1:{PORT}",
        f"http://localhost:{PORT}",
        f"http://[::1]:{PORT}",
        f"ws://127.0.0.1:{PORT}",
        f"ws://localhost:{PORT}",
    ]
    csp = {"connectDomains": origins, "resourceDomains": origins}
    return {"ui": {"csp": csp}, "ui/csp": csp}


# Session-scoped state. Keyed by `ctx.session` for isolation.
#
# Specs:
# - Use plain dict/set instead of WeakKeyDictionary: MCP hosts (and our tests) may provide
#   session identifiers that are not weakref-able (e.g., plain `object()`), and we must
#   still isolate state correctly.
_mcp_session_kits: dict[typing.Any, dict[str, typing.Any]] = {}
_mcp_session_designs: dict[typing.Any, dict[str, typing.Any]] = {}
_mcp_session_types: dict[typing.Any, dict[str, typing.Any]] = {}
_mcp_session_kit_mode: dict[typing.Any, str] = {}
_mcp_session_kit_source: dict[typing.Any, str] = {}
_mcp_session_transactions: dict[typing.Any, Transaction] = {}
_mcp_session_transaction_rollback: set[typing.Any] = set()
_mcp_session_selection: dict[typing.Any, dict[str, list[str]]] = {}
_mcp_session_camera: dict[typing.Any, dict[str, typing.Any]] = {}
import collections
_mcp_app_payloads: collections.OrderedDict[str, dict[str, typing.Any]] = collections.OrderedDict()
_MCP_APP_PAYLOADS_MAX_SIZE = 100
_mcp_app_file_blobs: dict[str, str] = {}


def _load_kit_from_remote(serverUrl: str, kitUri: str) -> dict:
    """🔳Load kit dict from a remote server via REST API.
    Callers MUST have called login() first to authenticate with the server.
    """
    token = getAuthToken(serverUrl)
    encodedKitUri = encode(kitUri)
    try:
        response = requests.get(
            f"{serverUrl}/api/kits/{encodedKitUri}",
            headers={"Authorization": f"Bearer {token}"},
            timeout=30,
        )
        response.raise_for_status()
        return response.json()
    except requests.exceptions.ConnectionError:
        raise ServerUnreachable(serverUrl)
    except requests.exceptions.HTTPError as e:
        if e.response is not None and e.response.status_code == 401:
            raise InvalidAuthToken(serverUrl)
        if e.response is not None and e.response.status_code == 404:
            raise KitNotFound(kitUri)
        raise ServerUnreachable(serverUrl)


def _load_reference_kit_json_for_folder(folder: pathlib.Path) -> dict | None:
    """🧭Load nearby canonical kit JSON used to restore links omitted by folder stores."""
    for json_path in (folder / "metabolism.kit.semio.json", folder / "kit.json", folder.parent / "metabolism.kit.semio.json"):
        if json_path.exists():
            with open(json_path, "r", encoding="utf-8") as f:
                return json.load(f)
    return None


def _merge_reference_collection_links(current_items: list[dict], reference_items: list[dict], link_keys: tuple[str, ...]) -> list[dict]:
    """🔗Copy relationship fields from canonical JSON when imported folder records omit them."""
    reference_by_id = {item.get("id"): item for item in reference_items if isinstance(item, dict) and item.get("id")}
    merged: list[dict] = []
    for item in current_items:
        if not isinstance(item, dict):
            continue
        ref = reference_by_id.get(item.get("id"), {})
        next_item = copy.deepcopy(item)
        for key in link_keys:
            if not next_item.get(key) and ref.get(key):
                next_item[key] = copy.deepcopy(ref[key])
                if key == "connections" and ref.get("pieces"):
                    next_item["pieces"] = copy.deepcopy(ref["pieces"])
        merged.append(next_item)
    return merged


def _merge_reference_kit_links(current: dict, reference: dict | None) -> dict:
    """🧩Merge canonical relationship data into a folder-loaded kit without replacing live records."""
    if not reference:
        return current
    merged = copy.deepcopy(current)
    for key in ("description", "createdAt", "updatedAt", "homepage", "remote", "preview", "icon", "image", "license"):
        if not merged.get(key) and reference.get(key):
            merged[key] = copy.deepcopy(reference[key])
    merged["designs"] = _merge_reference_collection_links(merged.get("designs", []) or [], reference.get("designs", []) or [], ("parent", "connections", "authors", "concepts", "props", "layers", "groups"))
    merged["types"] = _merge_reference_collection_links(merged.get("types", []) or [], reference.get("types", []) or [], ("parent", "families", "authors", "concepts", "representations", "connectors", "props"))
    return merged


def _load_kit_from_path(path: str) -> dict:
    """📁Load kit dict from path (JSON file or folder with .semio/kit.db or kit JSON).
    """
    p = pathlib.Path(path).resolve()
    if p.is_file() and p.suffix == ".json":
        with open(p, "r", encoding="utf-8") as f:
            return json.load(f)
    if p.is_dir():
        sqlite_path = p / KIT_LOCAL_FOLDERNAME / KIT_LOCAL_FILENAME
        if sqlite_path.exists():
            kit, _files = _semio_core.import_folder_kit(str(p))
            if hasattr(kit, "representation_dump"):
                loaded_kit = kit.representation_dump()
            elif hasattr(kit, "to_dict"):
                loaded_kit = kit.to_dict()
            else:
                loaded_kit = KitOutput.representation_validate(kit).representation_dump()
            return _merge_reference_kit_links(loaded_kit, _load_reference_kit_json_for_folder(p))
        for name in ("metabolism.kit.semio.json", "kit.json"):
            json_path = p / name
            if json_path.exists():
                with open(json_path, "r", encoding="utf-8") as f:
                    return json.load(f)
        parent_json = p.parent / "metabolism.kit.semio.json"
        if parent_json.exists():
            with open(parent_json, "r", encoding="utf-8") as f:
                return json.load(f)
    raise FileNotFoundError(f"Kit not found at path: {path}")


def _session_id(ctx) -> typing.Any | None:
    """🔲Get session identifier from context for per-session isolation."""
    return ctx.session if ctx and hasattr(ctx, "session") else None


def _get_session_kit(ctx) -> dict[str, typing.Any]:
    """▶️Get kit from session. Raises if start_working_in_local_kit or start_working_in_remote_kit was not called."""
    sid = _session_id(ctx)
    if sid is None or sid not in _mcp_session_kits:
        raise ValueError("Call start_working_in_local_kit(path) or start_working_in_remote_kit(serverUrl, kitUri) first to set the kit for this session.")
    return _mcp_session_kits[sid]


def _get_session_kit_mode(ctx) -> str:
    """▪️Get kit mode from session. Returns 'local' or 'remote'."""
    sid = _session_id(ctx)
    return _mcp_session_kit_mode.get(sid, "local")


def _hydrate_design_from_kit_disk_if_shallow(design: dict[str, typing.Any], kit_source: str | None, design_id: str) -> dict[str, typing.Any]:
    """▫️If the kit only lists design metadata (no pieces), load a sibling `*.design.semio.json` with the same id.
    """
    pieces = design.get("pieces") or []
    if len(pieces) > 0:
        return design
    if not kit_source or kit_source in ("<memory>",):
        return design
    if kit_source.startswith(("http://", "https://")):
        return design
    try:
        base = pathlib.Path(kit_source).resolve()
        search_roots: list[pathlib.Path] = []
        if base.is_file():
            search_roots.append(base.parent)
        elif base.is_dir():
            search_roots.append(base)
            search_roots.append(base.parent)
        else:
            return design
        seen_dirs: set[pathlib.Path] = set()
        best: dict[str, typing.Any] | None = None
        best_piece_count = -1
        for search_root in search_roots:
            if not search_root.is_dir():
                continue
            rp = search_root.resolve()
            if rp in seen_dirs:
                continue
            seen_dirs.add(rp)
            for candidate in sorted(search_root.glob("*.design.semio.json")):
                try:
                    with open(candidate, "r", encoding="utf-8") as f:
                        data = json.load(f)
                except OSError:
                    continue
                if not isinstance(data, dict) or data.get("id") != design_id:
                    continue
                n = len(data.get("pieces") or [])
                if n > best_piece_count:
                    best_piece_count = n
                    best = data
        if best is not None:
            return best
    except OSError:
        return design
    return design


def _get_session_design(ctx) -> dict[str, typing.Any]:
    """💼Get current design from session. Raises if start_working_in_design was not called."""
    sid = _session_id(ctx)
    if sid is None or sid not in _mcp_session_designs:
        raise ValueError("Call start_working_in_design(id) first to set the design for this session.")
    design = _mcp_session_designs[sid]
    id = design.get("id")
    if not isinstance(id, str) or not id:
        return design
    kit_src = _mcp_session_kit_source.get(sid)
    merged = _hydrate_design_from_kit_disk_if_shallow(design, kit_src, id)
    if merged is not design:
        _mcp_session_designs[sid] = merged
    return _mcp_session_designs[sid]


def _get_session_type(ctx) -> dict[str, typing.Any]:
    """◾Get current type from session. Raises if start_working_in_type was not called."""
    sid = _session_id(ctx)
    if sid is None or sid not in _mcp_session_types:
        raise ValueError("Call start_working_in_type(id) first to set the type for this session.")
    return _mcp_session_types[sid]


def _clone_kit(kit: dict | None) -> dict | None:
    """📸Create a deep copy of a kit dict for safe transaction snapshots."""
    if kit is None:
        return None
    return copy.deepcopy(kit)


def _sync_session_design_and_type(sid: int | None):
    """◽Realign current design and type selections with the current session kit after mutations or rollbacks."""
    if sid is None:
        return
    kit = _mcp_session_kits.get(sid)
    current_design = _mcp_session_designs.get(sid)
    if kit is None:
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        return
    if current_design is not None:
        design_id = current_design.get("id")
        synced_design = next((d for d in kit.get("designs", []) if d.get("id") == design_id), None)
        if synced_design is None:
            _mcp_session_designs.pop(sid, None)
        else:
            cur_n = len((current_design.get("pieces") or []))
            sync_n = len((synced_design.get("pieces") or []))
            if sync_n >= cur_n:
                _mcp_session_designs[sid] = synced_design
    current_type = _mcp_session_types.get(sid)
    if current_type is not None:
        type_id = current_type.get("id")
        synced_type = next((t for t in kit.get("types", []) if t.get("id") == type_id), None)
        if synced_type is None:
            _mcp_session_types.pop(sid, None)
        else:
            _mcp_session_types[sid] = synced_type


def _get_active_transaction(sid: int | None) -> Transaction | None:
    """◻️Return the active transaction for a session, if any."""
    if sid is None:
        return None
    transaction = _mcp_session_transactions.get(sid)
    if transaction is None or not transaction.get("active"):
        return None
    return transaction


def _record_transaction_kit_change(sid: int | None, before_kit: dict | None, after_kit: dict | None):
    """◼️Record a kit change in the active transaction using forward/backward diffs."""
    if sid is None or sid in _mcp_session_transaction_rollback:
        return
    transaction = _get_active_transaction(sid)
    if transaction is None:
        return
    if before_kit is None and after_kit is None:
        return
    before = _clone_kit(before_kit)
    after = _clone_kit(after_kit)
    if before is not None and after is not None:
        change = getKitChange(before, after)
        forward_diff = change.forward
        backward_diff = change.backward
    else:
        forward_diff = after
        backward_diff = before
    transaction["changes"].append(
        TransactionChange(
            kind="kit_change",
            before_has_kit=before is not None,
            after_has_kit=after is not None,
            forward_diff=forward_diff,
            backward_diff=backward_diff,
        )
    )


def _apply_kit_diff_to_copy(base: dict, diff: dict) -> dict:
    """🧮Apply a kit diff and return a dict even when the core applier mutates in place."""
    target = _clone_kit(base)
    applied = applyKitDiffDict(target, diff)
    return target if applied is None else applied


def _set_session_kit(ctx, kit: dict):
    """🗃️Set session kit and record the change if a transaction is active."""
    sid = _session_id(ctx)
    before = _mcp_session_kits.get(sid)
    _record_transaction_kit_change(sid, before, kit)
    _mcp_session_kits[sid] = kit
    _sync_session_design_and_type(sid)


def _clear_session_kit(ctx):
    """🔵Clear session kit and record the change if a transaction is active."""
    sid = _session_id(ctx)
    before = _mcp_session_kits.get(sid)
    _record_transaction_kit_change(sid, before, None)
    _mcp_session_kits.pop(sid, None)
    _sync_session_design_and_type(sid)


def _replace_design_in_session_kit(ctx: Context, design: dict) -> dict:
    """➡️Replace or append a design in the current session kit and keep the current design selection synced."""
    sid = _session_id(ctx)
    kit = _clone_kit(_get_session_kit(ctx))
    designs = list(kit.get("designs", []))
    replaced = False
    for index, existing_design in enumerate(designs):
        if existing_design.get("id") == design.get("id"):
            designs[index] = design
            replaced = True
            break
    if not replaced:
        designs.append(design)
    kit["designs"] = designs
    _set_session_kit(ctx, kit)
    synced_design = next((item for item in _mcp_session_kits[sid].get("designs", []) if item.get("id") == design.get("id")), None)
    if synced_design is not None:
        _mcp_session_designs[sid] = synced_design
        return synced_design
    raise ValueError(f"Design with id {design.get('id')} could not be stored in the current kit.")


def _mutate_current_design(ctx: Context, mutator: typing.Callable[[dict], None]) -> dict:
    """🔴Clone, mutate, and persist the current design in the current session kit."""
    design = copy.deepcopy(_get_session_design(ctx))
    mutator(design)
    return _replace_design_in_session_kit(ctx, design)


def _rollback_session_transaction(sid: int):
    """🟠Rollback all transaction changes in reverse order."""
    transaction = _get_active_transaction(sid)
    if transaction is None:
        return
    for change in reversed(transaction.get("changes", [])):
        if change.get("kind") != "kit_change":
            continue
        before_has_kit = bool(change.get("before_has_kit"))
        after_has_kit = bool(change.get("after_has_kit"))
        backward_diff = change.get("backward_diff")
        if not before_has_kit and after_has_kit:
            _mcp_session_kits.pop(sid, None)
            continue
        if before_has_kit and not after_has_kit:
            if backward_diff is not None:
                _mcp_session_kits[sid] = _clone_kit(backward_diff)
            continue
        if backward_diff is not None:
            current = _clone_kit(_mcp_session_kits.get(sid, {}))
            _mcp_session_kits[sid] = _apply_kit_diff_to_copy(current, backward_diff)
    _sync_session_design_and_type(sid)


@mcp.tool(meta=_KIT_APP_RESOURCE_META)
def start_working_in_local_kit(path: str, ctx: Context) -> CallToolResult:
    """🟡Load a local kit into the session. Must be called before any kit operations.

    Accepts an absolute path to a kit folder containing .semio/kit.db, a JSON file, or a folder containing metabolism.kit.semio.json.
    """
    try:
        kit = _load_kit_from_path(path)
        sid = _session_id(ctx)
        _set_session_kit(ctx, kit)
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        _mcp_session_kit_mode[sid] = "local"
        _mcp_session_kit_source[sid] = path
        return _build_kit_only_app_response(kit)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_KIT_APP_RESOURCE_META)
def start_new_kit(name: str, version: str, ctx: Context) -> CallToolResult:
    """📌Create a new in-memory kit for the session with the given name and version."""
    try:
        sid = _session_id(ctx)
        kit = {
            "name": name,
            "version": version,
            "authors": [],
            "qualities": [],
            "types": [],
            "designs": [],
        }
        _set_session_kit(ctx, kit)
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        _mcp_session_kit_mode[sid] = "local"
        _mcp_session_kit_source[sid] = "<memory>"
        return _build_kit_only_app_response(kit)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_KIT_APP_RESOURCE_META)
def start_working_in_remote_kit(serverUrl: str, kitUri: str, ctx: Context) -> CallToolResult:
    """🟢Load a remote kit into the session. Requires a prior login call. Must be called before any kit operations."""
    try:
        kit = _load_kit_from_remote(serverUrl, kitUri)
        sid = _session_id(ctx)
        _set_session_kit(ctx, kit)
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        _mcp_session_kit_mode[sid] = "remote"
        _mcp_session_kit_source[sid] = f"{serverUrl}/api/kits/{encode(kitUri)}"
        return _build_kit_only_app_response(kit)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


# #region 🎎MCP Auth Tools
# MCP Auth Tools MUST expose login, logout and status for remote server authentication.


def mcp_login(serverUrl: str, email: str, password: str) -> dict:
    """🎟️Login to a remote semio hub and store the auth token for subsequent remote kit operations."""
    try:
        return login(serverUrl, email, password)
    except Exception as e:
        return {"error": str(e)}


def mcp_logout(serverUrl: str) -> dict:
    """➖Logout from a remote semio hub and remove the stored token."""
    try:
        return logout(serverUrl)
    except Exception as e:
        return {"error": str(e)}


def mcp_auth_status(serverUrl: str) -> dict:
    """🟣Get the authentication status for a remote semio hub."""
    try:
        return getAuthStatus(serverUrl)
    except Exception as e:
        return {"error": str(e)}


# #endregion 🎎MCP Auth Tools


def validate_kit(kit: dict) -> dict:
    """🟤Validate a kit dict and return any validation problems."""
    try:
        result = validateKitDict(kit)
        return result.representation_dump() if hasattr(result, "representation_dump") else {"problems": []}
    except Exception as e:
        return {"error": str(e)}


def flatten_design(kit: dict, design_id: str) -> dict:
    """⚪Flatten a design by computing absolute planes for all pieces."""
    try:
        return flattenDesignDict(kit, design_id)
    except Exception as e:
        return {"error": str(e)}


def get_kit_diff(before: dict, after: dict) -> dict:
    """⚫Compute the diff between two kit states."""
    try:
        return getKitDiffDict(before, after)
    except Exception as e:
        return {"error": str(e)}


def apply_kit_diff(base: dict, diff: dict) -> dict:
    """🩵Apply a diff to a kit dict."""
    try:
        return _apply_kit_diff_to_copy(base, diff)
    except Exception as e:
        return {"error": str(e)}


def inverse_kit_diff(original: dict, applied_diff: dict) -> dict:
    """🩶Compute the inverse of a diff for undo operations."""
    try:
        return inverseKitDiffDict(original, applied_diff)
    except Exception as e:
        return {"error": str(e)}


def get_kit_change(before: dict, after: dict) -> dict:
    """🩷Compute forward and backward diffs between two kit states for undo/redo."""
    try:
        return changeToDict(getKitChange(before, after))
    except Exception as e:
        return {"error": str(e)}


def get_design_change(before: dict, after: dict) -> dict:
    """💜Compute forward and backward diffs between two design states for undo/redo."""
    try:
        return changeToDict(getDesignChange(before, after))
    except Exception as e:
        return {"error": str(e)}


def pieces_metadata(kit: dict, design_id: str) -> dict:
    """💙Get metadata for all pieces in a design including plane, center, fixedPieceId, parentPieceId, depth, and path."""
    try:
        return piecesMetadataDict(kit, design_id)
    except Exception as e:
        return {"error": str(e)}


def get_primitive_design(kit: dict, design_id: str) -> dict:
    """💚Get the root design of a design family."""
    try:
        return getPrimitiveDesignDict(kit, design_id)
    except Exception as e:
        return {"error": str(e)}


def get_design_family(kit: dict, design_id: str) -> list:
    """🌳Get all designs in a design family tree."""
    try:
        return getDesignFamilyDict(kit, design_id)
    except Exception as e:
        return {"error": str(e)}


def get_design_siblings(kit: dict, design_id: str) -> list:
    """💛Get all sibling designs sharing the same parent, excluding the given design."""
    try:
        return getDesignSiblingsDict(kit, design_id)
    except Exception as e:
        return {"error": str(e)}


def get_design_children(kit: dict, design_id: str) -> list:
    """🧡Get all direct child designs of a design."""
    try:
        return getDesignChildrenDict(kit, design_id)
    except Exception as e:
        return {"error": str(e)}


def are_designs_in_same_family(kit: dict, design_id_a: str, design_id_b: str) -> dict:
    """✔️Check if two designs belong to the same family."""
    try:
        return {"result": areDesignsInSameFamilyDict(kit, design_id_a, design_id_b)}
    except Exception as e:
        return {"error": str(e)}


def can_use_design_as_piece(kit: dict, container_design_id: str, piece_design_id: str) -> dict:
    """❤️Check if a design can be used as a piece in another design without creating circular references."""
    try:
        return {"result": canUseDesignAsPieceDict(kit, container_design_id, piece_design_id)}
    except Exception as e:
        return {"error": str(e)}


def find_same_family_design_pieces(kit: dict, design_id: str) -> list:
    """🤍Find pieces in a design that reference designs from the same family."""
    try:
        return findSameFamilyDesignPiecesDict(kit, design_id)
    except Exception as e:
        return {"error": str(e)}


def get_primitive_type(kit: dict, type_id: str) -> dict:
    """🖤Get the root type of a type family."""
    try:
        return getPrimitiveTypeDict(kit, type_id)
    except Exception as e:
        return {"error": str(e)}


def get_type_family(kit: dict, type_id: str) -> list:
    """🤎Get all types in a type family tree."""
    try:
        return getTypeFamilyDict(kit, type_id)
    except Exception as e:
        return {"error": str(e)}


def get_type_siblings(kit: dict, type_id: str) -> list:
    """💗Get all sibling types sharing the same parent, excluding the given type."""
    try:
        return getTypeSiblingsDict(kit, type_id)
    except Exception as e:
        return {"error": str(e)}


def get_type_children(kit: dict, type_id: str) -> list:
    """💖Get all direct child types of a type."""
    try:
        return getTypeChildrenDict(kit, type_id)
    except Exception as e:
        return {"error": str(e)}


def are_types_in_same_family(kit: dict, type_id_a: str, type_id_b: str) -> dict:
    """💝Check if two types belong to the same family."""
    try:
        return {"result": areTypesInSameFamilyDict(kit, type_id_a, type_id_b)}
    except Exception as e:
        return {"error": str(e)}


def find_piece_type_in_design(kit: dict, design_id: str, piece_id: str) -> dict:
    """💘Get the type of a specific piece in a design."""
    try:
        return findPieceTypeInDesignDict(kit, design_id, piece_id)
    except Exception as e:
        return {"error": str(e)}


def find_used_connectors_by_piece_in_design(kit: dict, design_id: str, piece_id: str) -> list:
    """💕Get all connectors of a piece that are used in connections."""
    try:
        return findUsedConnectorsByPieceInDesignDict(kit, design_id, piece_id)
    except Exception as e:
        return {"error": str(e)}


def find_replaceable_types_for_piece_in_design(kit: dict, design_id: str, piece_id: str, variants: list[str] = None) -> list:
    """🧹Find all types that can replace a piece while maintaining connection compatibility. Optionally filter by variant parent IDs."""
    try:
        return findReplaceableTypesForPieceInDesignDict(kit, design_id, piece_id, variants)
    except Exception as e:
        return {"error": str(e)}


def find_replaceable_types_for_pieces_in_design(kit: dict, design_id: str, piece_ids: list[str], variants: list[str] = None) -> list:
    """🔖Find types that can replace multiple pieces while maintaining all external connections."""
    try:
        return findReplaceableTypesForPiecesInDesignDict(kit, design_id, piece_ids, variants)
    except Exception as e:
        return {"error": str(e)}


def create_clustered_design(original_design: dict, cluster_piece_ids: list[str], design_name: str) -> dict:
    """🆕Create a new design from a subset of pieces. Returns the clustered design and external connections."""
    try:
        return createClusteredDesignDict(original_design, cluster_piece_ids, design_name)
    except Exception as e:
        return {"error": str(e)}


def replace_cluster_with_design(original_design: dict, cluster_piece_ids: list[str], clustered_design: dict, external_connections: list[dict]) -> dict:
    """🔖Compute a design diff that replaces clustered pieces with a single design reference."""
    try:
        return replaceClusterWithDesignDict(original_design, cluster_piece_ids, clustered_design, external_connections)
    except Exception as e:
        return {"error": str(e)}


def get_clusterable_groups(design: dict, selected_piece_ids: list[str]) -> list:
    """🔖Get groups of selected pieces that can be clustered into new designs."""
    try:
        return getClusterableGroupsDict(design, selected_piece_ids)
    except Exception as e:
        return {"error": str(e)}


def expand_design_pieces(design: dict, kit: dict) -> dict:
    """🔖Recursively expand design references by inlining their pieces and connections."""
    try:
        return expandDesignPiecesDict(design, kit)
    except Exception as e:
        return {"error": str(e)}


def find_attribute_value(entity: dict, name: str, default_value: str = None) -> dict:
    """🔖Find an attribute value on an entity by key name."""
    try:
        sentinel = ... if default_value is None else default_value
        result = findAttributeValueDict(entity, name, sentinel)
        return {"value": result}
    except Exception as e:
        return {"error": str(e)}


@functools.lru_cache(maxsize=1024)
def _find_bundled_design_metadata(id: str) -> dict | None:
    """🧷Find bundled design metadata by id for reconstructing stateful tool payloads."""
    assets_dir = _engine_bundle_dir().parent / "assets" / "semio"
    for kit_path in (assets_dir / "metabolism.kit.semio.json", assets_dir / "metabolism.shallow.kit.semio.json", assets_dir / "metabolism.meta.kit.semio.json"):
        if not kit_path.exists():
            continue
        try:
            with open(kit_path, "r", encoding="utf-8") as f:
                kit = json.load(f)
        except Exception:
            continue
        design = next((item for item in kit.get("designs", []) or [] if isinstance(item, dict) and item.get("id") == id), None)
        if design is not None:
            return design
    return None


@mcp.tool()
def read_current_kit(ctx: Context) -> dict:
    """📖Read the current session kit."""
    try:
        return _get_session_kit(ctx)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_new_design(
    id: str,
    name: str,
    description: str,
    unit: str,
    icon: str,
    image: str,
    created_at: str,
    updated_at: str,
    ctx: Context,
) -> dict:
    """Create and select a new design in the current kit with the given metadata."""
    try:
        bundled_design = _find_bundled_design_metadata(id)
        design = {
            "id": id,
            "name": name,
            "description": description,
            "unit": unit,
            "icon": icon,
            "image": image,
            "createdAt": created_at,
            "updatedAt": updated_at,
            "authors": [],
            "props": [],
            "pieces": [],
            "connections": [],
        }
        if bundled_design and bundled_design.get("families"):
            design["families"] = copy.deepcopy(bundled_design["families"])
        stored_design = _replace_design_in_session_kit(ctx, design)
        return {"ok": True, "id": stored_design["id"], "name": stored_design["name"]}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def add_current_design_author(id: str, ctx: Context) -> dict:
    """✍️Add an author reference to the current design by ID."""
    try:
        design = _mutate_current_design(ctx, lambda current_design: current_design.setdefault("authors", []).append({"id": id}))
        return {"ok": True, "authorCount": len(design.get("authors", []))}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def add_current_design_prop(id: str, quality_id: str, value: str, unit: str, ctx: Context) -> dict:
    """📍Add a prop entry to the current design."""
    try:

        def mutate(current_design: dict):
            current_design.setdefault("props", []).append(
                {
                    "id": id,
                    "quality": {"id": quality_id},
                    "value": value,
                    "unit": unit,
                }
            )

        design = _mutate_current_design(ctx, mutate)
        return {"ok": True, "propCount": len(design.get("props", []))}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def add_current_design_piece(
    id: str,
    name: str,
    kind_id: str,
    ctx: Context,
    description: str = "",
    is_hidden: bool = False,
    is_locked: bool = False,
) -> dict:
    """Add a piece to the current design without placement fields."""
    try:

        def mutate(current_design: dict):
            current_design.setdefault("pieces", []).append(
                {
                    "id": id,
                    "name": name,
                    "description": description,
                    "isHidden": is_hidden,
                    "isLocked": is_locked,
                    "type": {"id": kind_id},
                }
            )

        design = _mutate_current_design(ctx, mutate)
        return {"ok": True, "pieceCount": len(design.get("pieces", []))}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def add_current_design_piece_with_plane(
    id: str,
    name: str,
    kind_id: str,
    center_u: float,
    center_v: float,
    origin_x: float,
    origin_y: float,
    origin_z: float,
    x_axis_x: float,
    x_axis_y: float,
    x_axis_z: float,
    y_axis_x: float,
    y_axis_y: float,
    y_axis_z: float,
    ctx: Context,
    description: str = "",
    is_hidden: bool = False,
    is_locked: bool = False,
) -> dict:
    """Add a piece to the current design with explicit placement plane and center."""
    try:

        def mutate(current_design: dict):
            current_design.setdefault("pieces", []).append(
                {
                    "id": id,
                    "name": name,
                    "description": description,
                    "isHidden": is_hidden,
                    "isLocked": is_locked,
                    "type": {"id": kind_id},
                    "center": {"u": center_u, "v": center_v},
                    "plane": {
                        "origin": {"x": origin_x, "y": origin_y, "z": origin_z},
                        "xAxis": {"x": x_axis_x, "y": x_axis_y, "z": x_axis_z},
                        "yAxis": {"x": y_axis_x, "y": y_axis_y, "z": y_axis_z},
                    },
                }
            )

        design = _mutate_current_design(ctx, mutate)
        return {"ok": True, "pieceCount": len(design.get("pieces", []))}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def add_current_design_connection(
    id: str,
    parent_piece_id: str,
    parent_connector_id: str,
    child_piece_id: str,
    child_connector_id: str,
    rotation: float,
    u: float,
    v: float,
    shift: float,
    ctx: Context,
    description: str = "",
    gap: float = 0,
    rise: float = 0,
    tilt: float = 0,
    turn: float = 0,
) -> dict:
    """Add a connection between two pieces in the current design."""
    try:

        def mutate(current_design: dict):
            current_design.setdefault("connections", []).append(
                {
                    "id": id,
                    "gap": gap,
                    "description": description,
                    "parent": {
                        "piece": {"id": parent_piece_id},
                        "connector": {"id": parent_connector_id},
                    },
                    "tilt": tilt,
                    "rotation": rotation,
                    "rise": rise,
                    "turn": turn,
                    "child": {
                        "piece": {"id": child_piece_id},
                        "connector": {"id": child_connector_id},
                    },
                    "shift": shift,
                    "u": u,
                    "v": v,
                }
            )

        design = _mutate_current_design(ctx, mutate)
        return {"ok": True, "connectionCount": len(design.get("connections", []))}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool(meta=_APP_RESOURCE_META)
def start_working_in_design(id: str, ctx: Context) -> CallToolResult:
    """🔖Select a design by ID within the current kit. Requires start_working_in_local_kit to have been called first."""
    try:
        kit = _get_session_kit(ctx)
        design = next((d for d in kit.get("designs", []) if d.get("id") == id), None)
        if design is None:
            return _as_mcp_app_tool_result({"error": f"Design with id {id} not found in kit."}, is_error=True)
        sid = _session_id(ctx)
        _mcp_session_designs[sid] = design
        return _build_app_response("show-design", ctx)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


def _read_current_design(ctx: Context) -> dict:
    """🔖Read the current design set via start_working_in_design."""
    try:
        return _get_session_design(ctx)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def read_current_design(ctx: Context) -> dict:
    """🔖Read the current design that was set via start_working_in_design or start_new_design."""
    return _read_current_design(ctx)


@mcp.tool()
def finish_working_in_design(ctx: Context) -> dict:
    """🔖Clear the current design from session state."""
    try:
        sid = _session_id(ctx)
        _mcp_session_designs.pop(sid, None)
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_working_in_type(id: str, ctx: Context) -> dict:
    """🔖Select a type by ID within the current kit. Requires start_working_in_local_kit to have been called first."""
    try:
        kit = _get_session_kit(ctx)
        t = next((t for t in kit.get("types", []) if t.get("id") == id), None)
        if t is None:
            return {"error": f"Type with id {id} not found in kit."}
        sid = _session_id(ctx)
        _mcp_session_types[sid] = t
        return {"ok": True, "id": id, "name": t.get("name", "")}
    except Exception as e:
        return {"error": str(e)}


def _read_current_type(ctx: Context) -> dict:
    """🔖Read the current type set via start_working_in_type."""
    try:
        return _get_session_type(ctx)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def read_current_type(ctx: Context) -> dict:
    """🔖Read the current type that was set via start_working_in_type."""
    return _read_current_type(ctx)


@mcp.tool()
def finish_working_in_type(ctx: Context) -> dict:
    """🔖Clear the current type from session state."""
    try:
        sid = _session_id(ctx)
        _mcp_session_types.pop(sid, None)
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def finish_working_in_kit(ctx: Context) -> dict:
    """🔖Clear the current kit, design, type, mode, and source from session state."""
    try:
        sid = _session_id(ctx)
        _clear_session_kit(ctx)
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        _mcp_session_kit_mode.pop(sid, None)
        _mcp_session_kit_source.pop(sid, None)
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_transaction(ctx: Context) -> dict:
    """🔭Start a session-scoped transaction. Only one active transaction is allowed per session."""
    try:
        sid = _session_id(ctx)
        if _get_active_transaction(sid) is not None:
            return {"error": "A transaction is already active for this session."}
        _mcp_session_transactions[sid] = Transaction(
            active=True,
            started_at=datetime.datetime.now(datetime.UTC).isoformat(),
            changes=[],
        )
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


def finalize_transaction(ctx: Context) -> dict:
    """🔖Finalize the active session transaction and keep all applied changes."""
    try:
        sid = _session_id(ctx)
        transaction = _get_active_transaction(sid)
        if transaction is None:
            return {"error": "No active transaction for this session."}
        change_count = len(transaction.get("changes", []))
        _mcp_session_transactions.pop(sid, None)
        return {"ok": True, "changeCount": change_count}
    except Exception as e:
        return {"error": str(e)}


def abort_transaction(ctx: Context) -> dict:
    """🔖Abort the active session transaction and rollback all recorded changes in reverse order."""
    try:
        sid = _session_id(ctx)
        transaction = _get_active_transaction(sid)
        if transaction is None:
            return {"error": "No active transaction for this session."}
        rolled_back = len(transaction.get("changes", []))
        _mcp_session_transaction_rollback.add(sid)
        try:
            _rollback_session_transaction(sid)
        finally:
            _mcp_session_transaction_rollback.discard(sid)
            _mcp_session_transactions.pop(sid, None)
        return {"ok": True, "rolledBackChangeCount": rolled_back}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def transaction_finalize(ctx: Context) -> dict:
    """🔖Finalize the active session transaction and keep all applied changes."""
    return finalize_transaction(ctx)


@mcp.tool()
def transaction_abort(ctx: Context) -> dict:
    """🔖Abort the active session transaction and rollback all recorded changes in reverse order."""
    return abort_transaction(ctx)


@mcp.tool()
def sum_quality_in_design(design_id: str, quality_id: str, ctx: Context) -> dict:
    """🔖Sum the values of a quality across all pieces in a design, using piece-level props with fallback to type-level props."""
    try:
        kit = _get_session_kit(ctx)
        return {"result": sumQualityInDesignDict(kit, design_id, quality_id)}
    except Exception as e:
        return {"error": str(e)}


# #region 🔬MCP Selection Tools
# MCP Selection Tools MUST manage session-scoped piece/connection selection state.


def _get_session_selection(ctx) -> dict[str, list[str]]:
    """🔖Get current selection from session."""
    sid = _session_id(ctx)
    return _mcp_session_selection.get(sid, {"pieceIds": [], "connectionIds": []})


def _set_session_selection(ctx, selection: dict[str, list[str]]):
    """🔖Set selection in session."""
    sid = _session_id(ctx)
    _mcp_session_selection[sid] = selection


@mcp.tool()
def read_current_selection(ctx: Context) -> dict:
    """🔖Read the current piece and connection selection for this session. Returns pieceIds and connectionIds."""
    try:
        return _get_session_selection(ctx)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def set_current_selection(ctx: Context, piece_ids: list[str] | None = None, connection_ids: list[str] | None = None) -> dict:
    """🔖Set the current piece and connection selection for this session."""
    try:
        _set_session_selection(
            ctx,
            {
                "pieceIds": piece_ids or [],
                "connectionIds": connection_ids or [],
            },
        )
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def clear_current_selection(ctx: Context) -> dict:
    """🔖Clear the current piece and connection selection for this session."""
    try:
        sid = _session_id(ctx)
        _mcp_session_selection.pop(sid, None)
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


# #endregion 🔬MCP Selection Tools

# #region 👓MCP App Tools
# MCP App Tools MUST expose kit/design/diagram/scene visualization and selection intents as MCP tools.
# Each tool declares the resource URI matching its viewer: kit-viewer, design-viewer, scene-viewer, or diagram-viewer.
# Both nested (_meta.ui.resourceUri) and flat (_meta["ui/resourceUri"]) keys are required for
# host compatibility, matching the registerAppTool normalization from @representationcontextprotocol/ext-apps/server.


def _as_mcp_app_tool_result(payload: dict[str, typing.Any], *, is_error: bool = False) -> CallToolResult:
    """🔖Build tools/call result with full payload in text content and a fetchUrl fallback for hosts that truncate.
    """
    token = uuid.uuid4().hex
    _mcp_app_payloads[token] = payload
    while len(_mcp_app_payloads) > _MCP_APP_PAYLOADS_MAX_SIZE:
        _mcp_app_payloads.popitem(last=False)
    # NOTE: MCP hosts often embed the design-viewer from a different origin/port than the engine
    # (e.g. host UI at 127.0.0.1:6274, engine at 127.0.0.1:{PORT}). A relative URL would resolve
    # against the host origin and may be blocked by the MCP App CSP. Therefore we always emit an
    # absolute engine URL, which is declared in the MCP App CSP allowlist via _mcp_app_html_resource_meta().
    fetch_url = f"http://127.0.0.1:{PORT}/api/app/payload/{token}"
    payload["fetchUrl"] = fetch_url
    text = json.dumps(payload)
    hint: dict[str, typing.Any] = {"fetchUrl": fetch_url, "mode": payload.get("mode")}
    if payload.get("surface"):
        hint["surface"] = payload["surface"]
    if "points" in payload:
        hint["points"] = payload["points"]
        hint["lines"] = payload.get("lines", [])
    return CallToolResult(
        content=[
            TextContent(type="text", text=json.dumps(hint)),
            TextContent(type="text", text=text),
            EmbeddedResource(
                type="resource",
                resource=TextResourceContents(
                    uri="semio://mcp-app/tool-payload",
                    mimeType="application/json",
                    text=text,
                ),
            ),
        ],
        structuredContent=payload,
        isError=is_error,
    )


def _build_kit_only_app_payload(kit: dict) -> dict[str, typing.Any]:
    """🏺Serializable kit viewer payload (diagram lists empty; kitArtifacts populated)."""
    return {
        "points": [],
        "lines": [],
        "capabilities": {"pieceSelection": False, "connectionSelection": False},
        "kitArtifacts": _build_kit_artifact_data(kit),
        "kit": _strip_kit_blobs(kit),
    }


def _build_kit_only_app_response(kit: dict) -> CallToolResult:
    """🔖MCP Apps kit-viewer tool response with kit artifact data only (no diagram)."""
    return _as_mcp_app_tool_result(_build_kit_only_app_payload(kit))


def _connector_port_ref_string(value: object) -> str:
    """🔖Normalize connector.port (string or PortId object) to a id string for kit artifact JSON."""
    if value is None:
        return ""
    if isinstance(value, str):
        return value
    if isinstance(value, dict):
        g = value.get("id")
        if g is not None:
            return str(g)
    return ""


def _entity_id_ref(value: object) -> dict | None:
    """🔖Normalize an entity reference into a id object for artifact links."""
    if isinstance(value, dict) and value.get("id"):
        return {"id": value.get("id")}
    if isinstance(value, str) and value:
        return {"id": value}
    return None


def _infer_design_parent_ref(design: dict, designs: list[dict]) -> dict | None:
    """🪢Infer omitted design parent links for exported flat variants."""
    explicit = _entity_id_ref(design.get("parent"))
    if explicit:
        return explicit
    if design.get("name") != "Flat":
        return None
    parent = next((item for item in designs if item.get("id") != design.get("id") and item.get("name") != "Flat" and not item.get("parent")), None)
    return {"id": parent.get("id")} if parent and parent.get("id") else None


def _infer_type_parent_ref(kind: dict, kinds: list[dict]) -> dict | None:
    """🧬Infer omitted type parent links from shared family roots."""
    explicit = _entity_id_ref(kind.get("parent"))
    if explicit:
        return explicit
    if kind.get("name") == "Capsule":
        return None
    family_ids = {family.get("id") for family in kind.get("families", []) or [] if isinstance(family, dict) and family.get("id")}
    if not family_ids:
        return None
    parent = next(
        (
            item
            for item in kinds
            if item.get("id") != kind.get("id")
            and item.get("name") == "Capsule"
            and family_ids.intersection({family.get("id") for family in item.get("families", []) or [] if isinstance(family, dict) and family.get("id")})
        ),
        None,
    )
    return {"id": parent.get("id")} if parent and parent.get("id") else None


def _build_kit_artifact_data(kit: dict) -> dict:
    """🔖Build a minimal kit artifact payload for UI selection (designs, kinds, kit ports, connectors).

    Specs: ``ports`` MUST list kit-level Port entities only; ``connectors`` MUST list flattened
    type Connector rows (never label connectors as ports).
    """
    meta: dict = {
        "name": kit.get("name") or "",
        "version": kit.get("version") or "",
    }
    if kit.get("id"):
        meta["id"] = kit.get("id")
    for key in ("description", "createdAt", "updatedAt", "homepage", "remote", "preview", "icon", "image", "license"):
        value = kit.get(key)
        if value:
            meta[key] = value
    designs = []
    kit_designs = [d for d in kit.get("designs", []) or [] if isinstance(d, dict)]
    for d in kit_designs:
        id = d.get("id")
        if not id:
            continue
        design_payload = {"id": id, "name": d.get("name", ""), "variant": d.get("variant", ""), "view": d.get("view", "")}
        parent = _infer_design_parent_ref(d, kit_designs)
        if parent:
            design_payload["parent"] = parent
        for key in ("description", "createdAt", "updatedAt", "unit", "icon", "image"):
            value = d.get(key)
            if value:
                design_payload[key] = value
        designs.append(design_payload)

    types = []
    kit_ports: list[dict] = []
    for p in kit.get("ports", []) or []:
        pg = p.get("id")
        if not pg:
            continue
        port_payload: dict = {"id": pg, "name": p.get("name", "")}
        for key in ("description", "icon"):
            val = p.get(key)
            if val:
                port_payload[key] = val
        kit_ports.append(port_payload)

    connectors: list[dict] = []
    kit_types = [t for t in kit.get("types", []) or [] if isinstance(t, dict)]
    for t in kit_types:
        t_id = t.get("id")
        if not t_id:
            continue
        type_payload = {"id": t_id, "name": t.get("name", ""), "variant": t.get("variant", "")}
        parent = _infer_type_parent_ref(t, kit_types)
        if parent:
            type_payload["parent"] = parent
        for key in ("description", "createdAt", "updatedAt", "icon", "image"):
            value = t.get(key)
            if value:
                type_payload[key] = value
        types.append(type_payload)
        for c in t.get("connectors", []) or []:
            c_id = c.get("id")
            if not c_id:
                continue
            port_s = _connector_port_ref_string(c.get("port"))
            connectors.append(
                {
                    "id": c_id,
                    "typeId": t_id,
                    "id": c.get("id", ""),
                    "port": port_s,
                    "name": c.get("name", "") or c.get("id", "") or port_s or "connector",
                    "description": c.get("description", ""),
                    "mandatory": bool(c.get("mandatory", False)),
                }
            )

    meta["designs"] = designs
    meta["types"] = types
    meta["ports"] = kit_ports
    meta["connectors"] = connectors
    return meta


def _build_diagram_data(kit: dict, design_id: str, design_diff: dict | None = None, design: dict | None = None) -> dict:
    """🔖Compute pre-rendered diagram points and lines from kit/design data.
    """
    if design is None:
        design = next((d for d in kit.get("designs", []) if d.get("id") == design_id), None)
    if design is None:
        return {"points": [], "lines": []}
    design_for_diagram = designWithDiffDict(design, design_diff) if design_diff else design

    # Flatten the design to get absolute piece positions.
    # Inject the full design into kit so flattenDesignDict can find all pieces.
    try:
        kit_for_flatten = dict(kit)
        kit_for_flatten["designs"] = [d for d in kit.get("designs", []) if d.get("id") != design_id] + [design_for_diagram]
        flatten_result = flattenDesignDict(kit_for_flatten, design_id)
    except Exception:
        flatten_result = {}

    # Build piece center map from flatten result
    piece_centers: dict[str, dict] = {}
    for update in flatten_result.get("pieces", {}).get("updated", []):
        pid = update.get("id")
        center = update.get("diff", {}).get("center")
        if pid and center:
            piece_centers[pid] = center

    # Build piece map with positions
    pieces = design_for_diagram.get("pieces", [])
    piece_map: dict[str, dict] = {}
    for p in pieces:
        id = p.get("id")
        if not id:
            continue
        center = piece_centers.get(id, p.get("center") or {"u": 0, "v": 0})
        piece_map[id] = {"id": id, "id": p.get("id", ""), "center": center}

    # Determine diff statuses
    removed_piece_ids: set[str] = set()
    added_piece_ids: set[str] = set()
    modified_piece_ids: set[str] = set()
    removed_conn_ids: set[str] = set()
    added_conn_ids: set[str] = set()
    modified_conn_ids: set[str] = set()

    if design_diff:
        for p in design_diff.get("pieces", {}).get("removed", []):
            removed_piece_ids.add(p.get("id", ""))
        for p in design_diff.get("pieces", {}).get("added", []):
            id = p.get("id", "")
            added_piece_ids.add(id)
            # Include added pieces in the map with their centers
            center = p.get("center") or {"u": 0, "v": 0}
            piece_map[id] = {"id": id, "id": p.get("id", ""), "center": center}
        for p in design_diff.get("pieces", {}).get("updated", []):
            id = p.get("piece", {}).get("id", "")
            modified_piece_ids.add(id)
            center = p.get("diff", {}).get("center")
            if id and center:
                if id in piece_map:
                    piece_map[id]["center"] = center
                else:
                    piece_map[id] = {"id": id, "id": p.get("piece", {}).get("id", ""), "center": center}
        for c in design_diff.get("connections", {}).get("removed", []):
            removed_conn_ids.add(c.get("id", ""))
        for c in design_diff.get("connections", {}).get("added", []):
            added_conn_ids.add(c.get("id", ""))
        for c in design_diff.get("connections", {}).get("updated", []):
            modified_conn_ids.add(c.get("connection", {}).get("id", ""))

    # Build points
    points = []
    for id, pdata in piece_map.items():
        status = "default"
        if id in removed_piece_ids:
            status = "removed"
        elif id in added_piece_ids:
            status = "added"
        elif id in modified_piece_ids:
            status = "modified"
        center = pdata.get("center", {"u": 0, "v": 0})
        points.append(
            {
                "id": id,
                "id": pdata.get("id", ""),
                "u": center.get("u", 0),
                "v": center.get("v", 0),
                "status": status,
            }
        )

    # Build lines from connections
    connections = design.get("connections", []) or design.get("_connections", []) or []
    # Also include added connections from diff
    if design_diff:
        for c in design_diff.get("connections", {}).get("added", []):
            connections = list(connections) + [c]

    lines = []
    for c in connections:
        id = c.get("id")
        if not id:
            continue
        source_id = c.get("parent", {}).get("piece", {}).get("id")
        target_id = c.get("child", {}).get("piece", {}).get("id")
        source = piece_map.get(source_id)
        target = piece_map.get(target_id)
        if not source or not target:
            continue
        source_center = source.get("center", {"u": 0, "v": 0})
        target_center = target.get("center", {"u": 0, "v": 0})
        status = "default"
        if id in removed_conn_ids:
            status = "removed"
        elif id in added_conn_ids:
            status = "added"
        elif id in modified_conn_ids:
            status = "modified"
        lines.append(
            {
                "id": id,
                "sourceU": source_center.get("u", 0),
                "sourceV": source_center.get("v", 0),
                "targetU": target_center.get("u", 0),
                "targetV": target_center.get("v", 0),
                "status": status,
            }
        )

    return {"points": points, "lines": lines}


def _enrich_design(kit: dict, design: dict, design_diff: dict | None = None) -> dict:
    """🔖Enrich design pieces with flattened plane/center data from flattenDesignDict."""
    design_id = design.get("id")
    design_for_enrichment = designWithDiffDict(design, design_diff) if design_diff else design
    try:
        # Inject the full session design into the kit for flattening.
        # The kit's designs list may have a shallow entry (no pieces) when the
        # session design was hydrated from a sibling .design.semio.json file.
        # flattenDesignDict reads the design from kit["designs"], so it must
        # contain the full design with all pieces.
        kit_for_flatten = dict(kit)
        kit_for_flatten["designs"] = [d for d in kit.get("designs", []) if d.get("id") != design_id] + [design_for_enrichment]
        flatten_result = flattenDesignDict(kit_for_flatten, design_id)
        flatten_by_id: dict[str, dict] = {}
        for update in flatten_result.get("pieces", {}).get("updated", []):
            pid = update.get("id")
            if pid:
                flatten_by_id[pid] = update.get("diff", {})
        enriched_pieces = []
        for p in design_for_enrichment.get("pieces", []):
            id = p.get("id")
            flat = flatten_by_id.get(id) if id else None
            if flat:
                ep = dict(p)
                if flat.get("plane"):
                    ep["plane"] = flat["plane"]
                if flat.get("center"):
                    ep["center"] = flat["center"]
                enriched_pieces.append(ep)
            else:
                enriched_pieces.append(p)
        if design_diff:
            updated_centers_by_id = {
                update.get("piece", {}).get("id"): update.get("diff", {}).get("center")
                for update in design_diff.get("pieces", {}).get("updated", [])
                if update.get("piece", {}).get("id") and update.get("diff", {}).get("center")
            }
            if updated_centers_by_id:
                enriched_pieces = [
                    ({**piece, "center": updated_centers_by_id[piece.get("id")]} if piece.get("id") in updated_centers_by_id else piece)
                    for piece in enriched_pieces
                ]
        enriched_design = dict(design_for_enrichment)
        enriched_design["pieces"] = enriched_pieces
        return enriched_design
    except Exception:
        return design_for_enrichment


def _is_gltf_file(file: dict) -> bool:
    """🔖Return True if this kit file is a GLB or GLTF by name."""
    name = (file.get("name") or "").lower()
    return name.endswith(".glb") or name.endswith(".gltf")


def _select_best_representation_file_ids(kit: dict, design: dict | None) -> set[str]:
    """🔖Return file IDs for the GLB/GLTF representation file to inline per type used in the design.
    Mirrors JS buildScenePieceAssets: picks the untagged (or first) representation, then falls back
    to any representation with a GLB/GLTF file if the best representation's file isn't a GLB/GLTF."""
    if not design:
        return set()
    files_by_id = {f.get("id"): f for f in kit.get("files", []) if f.get("id")}
    type_ids = {(p.get("type") or {}).get("id") for p in design.get("pieces", [])}
    type_ids.discard(None)
    result: set[str] = set()
    for typ in kit.get("types", []):
        if typ.get("id") not in type_ids:
            continue
        representations = typ.get("representations") or []
        # Mirror JS selectBestRepresentation: prefer first untagged representation, else first representation.
        best = next((m for m in representations if not m.get("tags")), representations[0] if representations else None)
        best_file_id = (best.get("file") or {}).get("id") if best else None
        best_file = files_by_id.get(best_file_id) if best_file_id else None
        if best_file and _is_gltf_file(best_file):
            result.add(best_file_id)
            continue
        # Fallback: mirror JS buildScenePieceAssets lines 2110-2118: find any representation with a GLB file.
        for m in representations:
            fid = (m.get("file") or {}).get("id")
            f = files_by_id.get(fid) if fid else None
            if f and _is_gltf_file(f):
                result.add(fid)
                break
    return result


def _strip_kit_blobs(kit: dict, design: dict | None = None) -> dict:
    """💾Deep copy kit, cache file blobs in _mcp_app_file_blobs, and replace blob with url for UI transport.
    GLB/GLTF blobs for the design's selected type representations are kept inline as data URLs to avoid CSP/HTTP
    issues in sandboxed iframes. All other blobs are stripped and served via HTTP endpoint."""
    inline_ids = _select_best_representation_file_ids(kit, design)
    kit_for_ui = copy.deepcopy(kit)
    for f in kit_for_ui.get("files", []):
        id = f.get("id")
        name = (f.get("name") or "").lower()
        is_gltf = name.endswith(".glb") or name.endswith(".gltf")
        keep_inline = is_gltf and id in inline_ids
        if keep_inline:
            blob = f.get("blob")
        else:
            blob = f.pop("blob", None)
        if blob and id:
            _mcp_app_file_blobs[id] = blob
            if not keep_inline:
                f["url"] = f"http://127.0.0.1:{PORT}/api/app/files/{id}"
    return kit_for_ui


# Diagram-only modes: precomputed 2D geometry only.
_DIAGRAM_MODES = {"show-diagram", "show-diagram-diff", "select-pieces", "select-connections", "select-pieces-and-connections"}
# Split view: design+kit for scene plus points/lines for the diagram panel (start_working_in_design / show_design / show_scene).
_SPLIT_SCENE_DIAGRAM_MODES = {"show-design", "show-scene"}


def _mcp_app_surface_for_mode(mode: str) -> str:
    """📊Stable viewer surface for MCP Apps: design = SemioDesign (scene+diagram); scene = SemioScene; diagram = SemioDiagram only."""
    if mode in ("show-design", "show-diff"):
        return "design"
    if mode == "show-scene":
        return "scene"
    return "diagram"


def _build_app_payload(mode: str, ctx, design_diff: dict | None = None, capabilities: dict | None = None) -> dict[str, typing.Any]:
    """🔖Build mode-appropriate payload: diagram data for diagram modes, design/kit for scene/design modes.
    Diagram-only modes omit kit (~2.3MB of GLB blobs) to stay under host payload truncation limits.
    The JS diagram renderer uses Python-enriched piece centers from enriched_design instead."""
    kit = _get_session_kit(ctx)
    design = _get_session_design(ctx)
    enriched_design = _enrich_design(kit, design, design_diff)

    payload: dict[str, typing.Any] = {
        "mode": mode,
        "surface": _mcp_app_surface_for_mode(mode),
        "capabilities": capabilities or {
            "pieceSelection": mode in ("select-pieces", "select-pieces-and-connections"),
            "connectionSelection": mode in ("select-connections", "select-pieces-and-connections"),
        },
        "kitArtifacts": _build_kit_artifact_data(kit),
        "design": enriched_design,
    }

    # For diagram-only modes, omit kit (saves ~2.3MB of GLB blobs).
    # The diagram uses Python-precomputed centers from enriched_design — no kit GLBs needed.
    if mode not in _DIAGRAM_MODES:
        kit_for_ui = _strip_kit_blobs(kit, design=enriched_design)
        kit_for_ui.pop("designs", None)
        payload["kit"] = kit_for_ui

    if mode in _DIAGRAM_MODES or mode in _SPLIT_SCENE_DIAGRAM_MODES:
        diagram_data = _build_diagram_data(kit, design.get("id"), design_diff, design=design)
        payload["points"] = diagram_data["points"]
        payload["lines"] = diagram_data["lines"]

    if design_diff is not None:
        payload["designDiff"] = design_diff

    return payload


def _build_app_response(mode: str, ctx, design_diff: dict | None = None, capabilities: dict | None = None) -> CallToolResult:
    """🧱MCP Apps tool response with pre-computed diagram data and structuredContent for the appropriate viewer.
    """
    return _as_mcp_app_tool_result(_build_app_payload(mode, ctx, design_diff=design_diff, capabilities=capabilities))


@mcp.resource(
    _APP_RESOURCE_URI,
    name="semio design viewer",
    description="Interactive SVG diagram viewer for semio designs. Renders piece-connection diagrams with pan, zoom, and selection support.",
    mime_type="text/html;profile=mcp-app",
    meta=_mcp_app_html_resource_meta(),
)
def design_viewer_resource() -> str:
    """🔖Serve the MCP App design viewer HTML built from @semio/ui.
    """
    return _build_design_viewer_html()


@mcp.resource(
    _KIT_APP_RESOURCE_URI,
    name="semio kit viewer",
    description="Kit artifact browser for semio kits. Renders SemioKit (designs, kinds, connectors) from @semio/ui.",
    mime_type="text/html;profile=mcp-app",
    meta=_mcp_app_html_resource_meta(),
)
def kit_viewer_resource() -> str:
    """🔖Serve the MCP kit viewer HTML (SemioKit-only shell) built from @semio/ui.
    """
    return _build_kit_viewer_html()


@mcp.resource(
    _SCENE_APP_RESOURCE_URI,
    name="semio scene viewer",
    description="3D scene viewer for semio designs. Renders pieces with GLTF representations in 3D from @semio/ui.",
    mime_type="text/html;profile=mcp-app",
    meta=_mcp_app_html_resource_meta(),
)
def scene_viewer_resource() -> str:
    """🔖Serve the MCP scene viewer HTML (SemioScene 3D only shell) built from @semio/ui.
    """
    return _build_scene_viewer_html()


@mcp.resource(
    _DIAGRAM_APP_RESOURCE_URI,
    name="semio diagram viewer",
    description="2D diagram viewer for semio designs. Renders piece-connection diagrams with pan and zoom from @semio/ui.",
    mime_type="text/html;profile=mcp-app",
    meta=_mcp_app_html_resource_meta(),
)
def diagram_viewer_resource() -> str:
    """🔖Serve the MCP diagram viewer HTML (SemioDiagram 2D only shell) built from @semio/ui.
    """
    return _build_diagram_viewer_html()


@mcp.tool(meta=_APP_RESOURCE_META)
def show_design(ctx: Context) -> CallToolResult:
    """🔖Show the current design in the split design viewer (scene + 2D diagram). Requires an active kit and design session."""
    try:
        return _build_app_response("show-design", ctx)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_DIAGRAM_APP_RESOURCE_META)
def show_diagram(ctx: Context) -> CallToolResult:
    """🔖Show the current design as a 2D diagram only (no 3D scene panel). Requires an active kit and design session."""
    try:
        return _build_app_response("show-diagram", ctx)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_SCENE_APP_RESOURCE_META)
def show_scene(ctx: Context) -> CallToolResult:
    """🔖Show the current design in the 3D scene viewer. Requires an active kit and design session."""
    try:
        return _build_app_response("show-scene", ctx)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_APP_RESOURCE_META)
def show_diff(ctx: Context, design_diff: dict | None = None) -> CallToolResult:
    """🔖Show a diff of the current design in the split design viewer (scene + 2D diagram) with diff coloring. Uses an empty diff if none is provided. Requires an active kit and design session."""
    try:
        return _build_app_response("show-diff", ctx, design_diff=design_diff)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_DIAGRAM_APP_RESOURCE_META)
def show_diagram_diff(ctx: Context, design_diff: dict | None = None) -> CallToolResult:
    """🔖Show a diff of the current design as a 2D diagram only with diff coloring. Uses an empty diff if none is provided. Requires an active kit and design session."""
    try:
        return _build_app_response("show-diagram-diff", ctx, design_diff=design_diff)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_DIAGRAM_APP_RESOURCE_META)
def select_pieces(ctx: Context) -> CallToolResult:
    """📬Open a piece selection view where only pieces can be selected. Requires an active kit and design session."""
    try:
        return _build_app_response(
            "select-pieces",
            ctx,
            capabilities={
                "pieceSelection": True,
                "connectionSelection": False,
            },
        )
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_DIAGRAM_APP_RESOURCE_META)
def select_connections(ctx: Context) -> CallToolResult:
    """🔖Open a connection selection view where only connections can be selected. Requires an active kit and design session."""
    try:
        return _build_app_response(
            "select-connections",
            ctx,
            capabilities={
                "pieceSelection": False,
                "connectionSelection": True,
            },
        )
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_DIAGRAM_APP_RESOURCE_META)
def select_pieces_and_connections(ctx: Context) -> CallToolResult:
    """🔖Open a combined selection view where both pieces and connections can be selected. Requires an active kit and design session."""
    try:
        return _build_app_response(
            "select-pieces-and-connections",
            ctx,
            capabilities={
                "pieceSelection": True,
                "connectionSelection": True,
            },
        )
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


# #endregion 👓MCP App Tools


# #endregion ⛩️Mcp

# #region 🔔Engine
# Engine MUST mount REST, GraphQL, and MCP sub-applications and manage the server lifecycle.


@contextlib.asynccontextmanager
async def engineLifespan(app):
    """Manages the MCP session lifecycle during engine startup and shutdown.
    Callers MUST use this as the lifespan parameter for the Starlette application.
    """
    async with mcp.session_manager.run():
        yield


mcp.settings.streamable_http_path = "/"
engine = starlette.applications.Starlette(lifespan=engineLifespan)
engine.mount("/api", rest)
engine.mount("/graphql", graphql_http_app)
engine.mount("/mcp", mcp.streamable_http_app())


def start_engine():
    """🔖Starts the uvicorn server hosting the engine application.
    Callers MUST invoke this in a separate process to avoid blocking the UI.
    """
    # TODO: Make loguru work on extra uvicorn engine process.
    logging.basicConfig(level=logging.INFO)
    uvicorn.run(
        engine,
        host=HOST,
        port=PORT,
        log_level="info",
        access_log=False,
        log_config=None,
    )


def restart_engine():
    """🔖Terminates the running engine process and starts a new one.
    Callers MUST ensure a PySide6 QApplication instance is running.
    """
    import PySide6.QtWidgets

    ui_instance = PySide6.QtWidgets.QApplication.instance()
    engine_process = ui_instance.engine_process
    if engine_process.is_alive():
        engine_process.terminate()
    ui_instance.engine_process = multiprocessing.Process(target=start_engine)
    ui_instance.engine_process.start()


def run(dev_mode: bool | None = None):
    """🔖Main entry point that starts the engine with optional dev mode and system tray UI.
    Callers MUST invoke this from the __main__ block or dev function.
    """
    logger.debug("Starting engine")
    multiprocessing.freeze_support()

    parser = argparse.ArgumentParser(description="semio ⋅ engine")
    parser.add_argument("-d", "--debug", help="debug mode", action="store_true")
    parser.add_argument("--dev", help="dev mode", action="store_true")
    parser.add_argument("--mcp-stdio", help="start mcp server over stdio", action="store_true")

    args = parser.parse_args()
    if dev_mode is None:
        dev_mode = args.dev or args.debug
    if dev_mode:
        logger.debug("Starting debugpy for semio engine")
        import debugpy

        debugpy.listen(("0.0.0.0" if os.environ.get("DEVCONTAINER") == "true" else "127.0.0.1", 5678))
        logger.debug("Waiting for debugger to attach to semio engine")
        debugpy.wait_for_client()
        preDev()
        logger.add(sys.stderr, level="INFO")
        logger.add(DEBUG_LOG_FILE, level="DEBUG", rotation="10 MB")
    if args.mcp_stdio:
        mcp.run()
        return

    import PySide6.QtCore
    import PySide6.QtGui
    import PySide6.QtWidgets

    ui = PySide6.QtWidgets.QApplication(sys.argv)
    ui.setQuitOnLastWindowClosed(False)

    if getattr(sys, "frozen", False):
        basedir = sys._MEIPASS
    else:
        basedir = "../assets"

    icon = PySide6.QtGui.QIcon()
    icon.addFile(os.path.join(basedir, "icons/semio_512x512.png"), PySide6.QtCore.QSize(512, 512))

    tray = PySide6.QtWidgets.QSystemTrayIcon()
    tray.setIcon(icon)
    tray.setVisible(True)

    menu = PySide6.QtWidgets.QMenu()
    restart = PySide6.QtGui.QAction("Restart")
    restart.triggered.connect(restart_engine)
    menu.addAction(restart)

    quit = PySide6.QtGui.QAction("Quit")
    quit.triggered.connect(lambda: ui.engine_process.terminate() or ui.quit())
    menu.addAction(quit)

    tray.setContextMenu(menu)

    ui.engine_process = multiprocessing.Process(target=start_engine)
    ui.engine_process.start()

    sys.exit(ui.exec())


def preDev():
    """🔖Runs before dev()
    Callers MUST NOT add blocking operations in this hook.
    """


def dev():
    """🐛Starts the engine in development mode with debugging enabled.
    Callers MUST have debugpy available when using this entry point.
    """
    run(dev_mode=True)


# #region 🥼Tests
# Pytest suite lives in this module so the engine and tests share one unit of compilation.
import tempfile
from unittest.mock import MagicMock, patch

import pytest
from starlette.testclient import TestClient

engine = sys.modules[__name__]
sys.modules["engine"] = engine


def _mcp_app_tool_payload(result: object) -> dict:
    """🔖Unpack kit/design MCP app tool returns (CallToolResult with structuredContent)."""
    assert isinstance(result, CallToolResult), result
    assert result.structuredContent is not None
    return result.structuredContent


# #region 👓Constants
ASSETS_DIR = pathlib.Path(__file__).parent.parent / "assets" / "semio"
KIT_METABOLISM_PATH = ASSETS_DIR / "metabolism.kit.semio.json"
METABOLISM_DIR = ASSETS_DIR / "metabolism"

# #endregion 👓Constants


# #region 🧸Fixtures
@pytest.fixture
def kitMetabolismJson() -> dict:
    with open(KIT_METABOLISM_PATH, "r", encoding="utf-8") as f:
        return json.load(f)


@pytest.fixture
def minimalKitJson() -> dict:
    return {
        "name": "TestKit",
        "version": "1.0.0",
    }


@pytest.fixture
def tempKitPath() -> pathlib.Path:
    tmpDir = tempfile.mkdtemp()
    yield pathlib.Path(tmpDir)
    shutil.rmtree(tmpDir, ignore_errors=True)


@pytest.fixture
def restClient() -> TestClient:
    return TestClient(engine.rest)


@pytest.fixture
def graphqlClient() -> TestClient:
    return TestClient(engine.engine)


# #endregion 🧸Fixtures


# #region 📹Encoding Tests
class TestEncoding:
    def test_encode_basic(self):
        assert engine.encode("hello") == "hello"
        assert engine.encode("hello world") == "hello%20world"
        assert engine.encode("/path/to/file") == "%2Fpath%2Fto%2Ffile"

    def test_decode_basic(self):
        assert engine.decode("hello") == "hello"
        assert engine.decode("hello%20world") == "hello world"
        assert engine.decode("%2Fpath%2Fto%2Ffile") == "/path/to/file"

    def test_encode_decode_roundtrip(self):
        testStrings = [
            "hello",
            "hello world",
            "/path/to/file",
            "special!@#$%",
            "C:\\Windows\\path",
        ]
        for s in testStrings:
            assert engine.decode(engine.encode(s)) == s


# #endregion 📹Encoding Tests


# #region 💧OperationBuilder Tests
class TestOperationBuilder:
    def test_parse_kit_operation(self):
        code = "C%3A%5Ctest%5Ckit"
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == engine.OperationKind.KIT
        assert "kitUri" in operation

    def test_parse_kits_operation(self):
        code = ""
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == engine.OperationKind.KITS

    def test_parse_types_operation(self):
        code = "C%3A%5Ctest%5Ckit/types"
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == engine.OperationKind.TYPES
        assert "kitUri" in operation

    def test_parse_designs_operation(self):
        code = "C%3A%5Ctest%5Ckit/designs"
        tree = engine.codeParser.parse(code)
        operation = engine.OperationBuilder().transform(tree)
        assert operation["kind"] == engine.OperationKind.DESIGNS
        assert "kitUri" in operation


# #endregion 💧OperationBuilder Tests


# #region 🎲Store Tests
class TestSqliteStore:
    def test_store_factory_absolute_path(self, tempKitPath: pathlib.Path):
        engine.StoreFactory.cache_clear()
        store = engine.StoreFactory(str(tempKitPath))
        assert isinstance(store, engine.SqliteStore)
        assert store.uri == str(tempKitPath)

    def test_store_factory_caching(self, tempKitPath: pathlib.Path):
        engine.StoreFactory.cache_clear()
        store1 = engine.StoreFactory(str(tempKitPath))
        store2 = engine.StoreFactory(str(tempKitPath))
        assert store1 is store2

    def test_store_initialize(self, tempKitPath: pathlib.Path):
        store = engine.SqliteStore.fromUri(str(tempKitPath))
        store.initialize()
        semioFolder = tempKitPath / ".semio"
        assert semioFolder.exists()
        assert (semioFolder / "kit.db").exists()

    def test_store_initialized_check(self, tempKitPath: pathlib.Path):
        store = engine.SqliteStore.fromUri(str(tempKitPath))
        assert not store.initialized()
        store.initialize()
        assert store.initialized()


# #endregion 🎲Store Tests


# #region 🔔StoreKind Tests
class TestStoreKind:
    def test_store_kind_values(self):
        assert engine.StoreKind.DATABASE.value == "database"
        assert engine.StoreKind.REST.value == "rest"
        assert engine.StoreKind.GRAPHQL.value == "graphql"


# #endregion 🔔StoreKind Tests


# #region 📍CommandKind Tests
class TestCommandKind:
    def test_command_kind_values(self):
        assert engine.CommandKind.QUERY.value == "query"
        assert engine.CommandKind.PUT.value == "put"
        assert engine.CommandKind.UPDATE.value == "update"
        assert engine.CommandKind.DELETE.value == "delete"


# #endregion 📍CommandKind Tests


# #region 🔊REST API Tests
class TestRestApi:
    def test_get_kit_not_found(self, restClient: TestClient, tempKitPath: pathlib.Path):
        nonExistentPath = str(tempKitPath / "nonexistent")
        encodedUri = engine.encode(nonExistentPath)
        response = restClient.get(f"/kits/{encodedUri}")
        assert response.status_code in [400, 404, 500]


# #endregion 🔊REST API Tests


# #region 🥁GraphQL Tests
class TestGraphQL:
    def test_graphql_schema_exists(self):
        assert engine.graphqlSchema is not None
        assert engine.graphql_http_app is not None

    def test_graphql_query_fields(self):
        qt = engine.graphql_schema.query_type
        assert qt is not None
        assert "kit" in qt.fields
        assert "node" in qt.fields


# #endregion 🥁GraphQL Tests


# #region 🎖️MCP Tests
class TestMcp:
    def test_mcp_instance_exists(self):
        assert engine.mcp is not None

    def test_mcp_kit_tools_reference_kit_viewer_resource(self):
        """🧪Kit-loading tools declare ui://semio/kit-viewer; design tools use design-viewer; diagram tools use diagram-viewer; scene tools use scene-viewer."""
        tools = {t.name: t for t in engine.mcp._tool_manager.list_tools()}
        for name in ("start_working_in_local_kit", "start_new_kit", "start_working_in_remote_kit"):
            assert tools[name].meta["ui"]["resourceUri"] == "ui://semio/kit-viewer"
        assert tools["show_design"].meta["ui"]["resourceUri"] == "ui://semio/design-viewer"
        assert tools["show_diff"].meta["ui"]["resourceUri"] == "ui://semio/design-viewer"
        assert tools["show_scene"].meta["ui"]["resourceUri"] == "ui://semio/scene-viewer"
        assert tools["show_diagram"].meta["ui"]["resourceUri"] == "ui://semio/diagram-viewer"
        assert tools["show_diagram_diff"].meta["ui"]["resourceUri"] == "ui://semio/diagram-viewer"
        for name in ("select_pieces", "select_connections", "select_pieces_and_connections"):
            assert tools[name].meta["ui"]["resourceUri"] == "ui://semio/diagram-viewer"

    def test_mcp_app_html_resources_include_ui_csp_meta(self):
        """🔖MCP App HTML resources expose _meta.ui.csp so hosts allow network access to the engine (see .repo//mcp-app.md)."""
        resources = {str(r.uri): r for r in engine.mcp._resource_manager.list_resources()}
        for uri in ("ui://semio/design-viewer", "ui://semio/kit-viewer", "ui://semio/scene-viewer", "ui://semio/diagram-viewer"):
            r = resources[uri]
            assert r.meta is not None
            csp = r.meta.get("ui", {}).get("csp")
            assert csp is not None
            assert isinstance(csp.get("connectDomains"), list) and len(csp["connectDomains"]) > 0
            assert r.meta.get("ui/csp") is csp

    def test_mcp_tool_surface_keeps_only_allowed_prefixes(self):
        names = sorted(tool.name for tool in engine.mcp._tool_manager.list_tools())
        assert names == [
            "add_current_design_author",
            "add_current_design_connection",
            "add_current_design_piece",
            "add_current_design_piece_with_plane",
            "add_current_design_prop",
            "clear_current_selection",
            "finish_working_in_design",
            "finish_working_in_kit",
            "finish_working_in_type",
            "read_current_design",
            "read_current_kit",
            "read_current_selection",
            "read_current_type",
            "select_connections",
            "select_pieces",
            "select_pieces_and_connections",
            "set_current_selection",
            "show_design",
            "show_diagram",
            "show_diagram_diff",
            "show_diff",
            "show_scene",
            "start_new_design",
            "start_new_kit",
            "start_transaction",
            "start_working_in_design",
            "start_working_in_local_kit",
            "start_working_in_remote_kit",
            "start_working_in_type",
            "sum_quality_in_design",
            "transaction_abort",
            "transaction_finalize",
        ]

    def test_flatten_design_tool(self, minimalKitJson: dict):
        result = engine.flatten_design(minimalKitJson, "test-design-id")
        assert isinstance(result, dict)

    def test_pieces_metadata_tool(self, minimalKitJson: dict):
        result = engine.pieces_metadata(minimalKitJson, "test-design-id")
        assert isinstance(result, dict)

    def test_get_primitive_design_tool(self):
        kit = {"name": "test", "designs": [{"id": "d1", "name": "Design1"}]}
        result = engine.get_primitive_design(kit, "d1")
        assert result.get("id") == "d1"

    def test_get_design_family_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {"id": "d1", "name": "Root"},
                {"id": "d2", "name": "Child", "parent": {"id": "d1"}},
            ],
        }
        result = engine.get_design_family(kit, "d2")
        assert isinstance(result, list)
        assert len(result) == 2

    def test_get_design_siblings_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {"id": "d1", "name": "Root"},
                {"id": "d2", "name": "Child1", "parent": {"id": "d1"}},
                {"id": "d3", "name": "Child2", "parent": {"id": "d1"}},
            ],
        }
        result = engine.get_design_siblings(kit, "d2")
        assert isinstance(result, list)
        assert len(result) == 1
        assert result[0].get("id") == "d3"

    def test_get_design_children_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {"id": "d1", "name": "Root"},
                {"id": "d2", "name": "Child", "parent": {"id": "d1"}},
            ],
        }
        result = engine.get_design_children(kit, "d1")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_are_designs_in_same_family_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {"id": "d1", "name": "Root"},
                {"id": "d2", "name": "Child", "parent": {"id": "d1"}},
            ],
        }
        result = engine.are_designs_in_same_family(kit, "d1", "d2")
        assert result.get("result") is True

    def test_can_use_design_as_piece_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {"id": "d1", "name": "Root"},
                {"id": "d2", "name": "Other"},
            ],
        }
        result = engine.can_use_design_as_piece(kit, "d1", "d2")
        assert result.get("result") is True

    def test_find_same_family_design_pieces_tool(self):
        kit = {
            "name": "test",
            "designs": [
                {
                    "id": "d1",
                    "name": "Design1",
                    "pieces": [
                        {"id": "p1", "name": "Piece1", "design": {"id": "d1"}},
                    ],
                },
            ],
        }
        result = engine.find_same_family_design_pieces(kit, "d1")
        assert isinstance(result, list)

    def test_get_primitive_type_tool(self):
        kit = {"name": "test", "types": [{"id": "t1", "name": "Type1"}]}
        result = engine.get_primitive_type(kit, "t1")
        assert result.get("id") == "t1"

    def test_get_type_family_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"id": "t1", "name": "Root"},
                {"id": "t2", "name": "Child", "parent": {"id": "t1"}},
            ],
        }
        result = engine.get_type_family(kit, "t2")
        assert isinstance(result, list)
        assert len(result) == 2

    def test_get_type_siblings_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"id": "t1", "name": "Root"},
                {"id": "t2", "name": "ChildA", "parent": {"id": "t1"}},
                {"id": "t3", "name": "ChildB", "parent": {"id": "t1"}},
            ],
        }
        result = engine.get_type_siblings(kit, "t2")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_get_type_children_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"id": "t1", "name": "Root"},
                {"id": "t2", "name": "Child", "parent": {"id": "t1"}},
            ],
        }
        result = engine.get_type_children(kit, "t1")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_are_types_in_same_family_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"id": "t1", "name": "Root"},
                {"id": "t2", "name": "Child", "parent": {"id": "t1"}},
            ],
        }
        result = engine.are_types_in_same_family(kit, "t1", "t2")
        assert result.get("result") is True

    def test_find_piece_type_in_design_tool(self):
        kit = {
            "name": "test",
            "types": [{"id": "t1", "name": "Type1"}],
            "designs": [
                {
                    "id": "d1",
                    "name": "Design1",
                    "pieces": [
                        {"id": "p1", "name": "Piece1", "type": {"id": "t1"}},
                    ],
                },
            ],
        }
        result = engine.find_piece_type_in_design(kit, "d1", "p1")
        assert result.get("id") == "t1"

    def test_find_used_connectors_by_piece_in_design_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"id": "t1", "name": "Type1", "connectors": [{"id": "c1", "name": "Con1"}]},
            ],
            "designs": [
                {
                    "id": "d1",
                    "name": "Design1",
                    "pieces": [
                        {"id": "p1", "name": "Piece1", "type": {"id": "t1"}},
                        {"id": "p2", "name": "Piece2", "type": {"id": "t1"}},
                    ],
                    "connections": [
                        {"id": "conn1", "parent": {"piece": {"id": "p1"}, "connector": {"id": "c1"}}, "child": {"piece": {"id": "p2"}, "connector": {"id": "c1"}}},
                    ],
                },
            ],
        }
        result = engine.find_used_connectors_by_piece_in_design(kit, "d1", "p1")
        assert isinstance(result, list)
        assert len(result) == 1

    def test_create_clustered_design_tool(self):
        design = {
            "name": "test",
            "pieces": [
                {"id": "p1", "name": "P1"},
                {"id": "p2", "name": "P2"},
            ],
            "connections": [
                {"id": "c1", "parent": {"piece": {"id": "p1"}}, "child": {"piece": {"id": "p2"}}},
            ],
        }
        result = engine.create_clustered_design(design, ["p1", "p2"], "Cluster")
        assert "clusteredDesign" in result
        assert "externalConnections" in result

    def test_get_clusterable_groups_tool(self):
        design = {
            "pieces": [
                {"id": "p1", "name": "P1"},
                {"id": "p2", "name": "P2"},
            ],
            "connections": [
                {"id": "c1", "parent": {"piece": {"id": "p1"}}, "child": {"piece": {"id": "p2"}}},
            ],
        }
        result = engine.get_clusterable_groups(design, ["p1", "p2"])
        assert isinstance(result, list)

    def test_expand_design_pieces_tool(self):
        design = {"name": "test", "pieces": [{"id": "p1"}], "connections": []}
        kit = {"name": "kit", "designs": [design]}
        result = engine.expand_design_pieces(design, kit)
        assert isinstance(result, dict)

    def test_find_attribute_value_tool(self):
        entity = {"attributes": [{"key": "color", "value": "red"}]}
        result = engine.find_attribute_value(entity, "color")
        assert result.get("value") == "red"

    def test_find_replaceable_types_for_piece_in_design_tool(self):
        kit = {
            "name": "test",
            "types": [
                {"id": "t1", "name": "Type1", "connectors": [{"id": "c1"}]},
                {"id": "t2", "name": "Type2", "connectors": [{"id": "c2"}]},
            ],
            "designs": [
                {
                    "id": "d1",
                    "name": "Design1",
                    "pieces": [
                        {"id": "p1", "name": "Piece1", "type": {"id": "t1"}},
                    ],
                    "connections": [],
                },
            ],
        }
        result = engine.find_replaceable_types_for_piece_in_design(kit, "d1", "p1")
        assert isinstance(result, list)

    def test_validate_kit_tool(self, minimalKitJson: dict):
        result = engine.validate_kit(minimalKitJson)
        assert "problems" in result or "error" in result

    def test_get_kit_diff_tool(self, minimalKitJson: dict):
        before = minimalKitJson.copy()
        after = minimalKitJson.copy()
        after["name"] = "ModifiedKit"
        result = engine.get_kit_diff(before, after)
        assert isinstance(result, dict)

    def test_apply_kit_diff_tool(self, minimalKitJson: dict):
        diff = {"name": "ModifiedKit"}
        result = engine.apply_kit_diff(minimalKitJson, diff)
        assert isinstance(result, dict)

    def test_inverse_kit_diff_tool(self, minimalKitJson: dict):
        diff = {"name": "ModifiedKit"}
        result = engine.inverse_kit_diff(minimalKitJson, diff)
        assert isinstance(result, dict)

    def test_sum_quality_in_design_tool(self):
        kit = {
            "name": "test",
            "types": [
                {
                    "id": "t1",
                    "name": "TypeA",
                    "props": [
                        {"id": "p1", "quality": {"id": "q1"}, "value": "10.5"},
                    ],
                },
                {
                    "id": "t2",
                    "name": "TypeB",
                    "props": [
                        {"id": "p2", "quality": {"id": "q1"}, "value": "20.0"},
                    ],
                },
            ],
            "designs": [
                {
                    "id": "d1",
                    "name": "Design1",
                    "pieces": [
                        {"id": "pc1", "name": "Piece1", "type": {"id": "t1"}},
                        {"id": "pc2", "name": "Piece2", "type": {"id": "t2"}},
                        {"id": "pc3", "name": "Piece3", "type": {"id": "t1"}},
                    ],
                },
            ],
        }
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine._mcp_session_kits[mock_ctx.session] = kit
        result = engine.sum_quality_in_design("d1", "q1", mock_ctx)
        assert abs(result.get("result") - 41.0) < 0.001

    def test_start_working_in_local_kit_loads_from_path(self):
        """🔖start_working_in_local_kit loads kit from metabolism JSON path."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.start_working_in_local_kit(str(KIT_METABOLISM_PATH), mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert "kitArtifacts" in payload
        assert mock_ctx.session in engine._mcp_session_kits

    def test_start_working_in_local_kit_loads_from_folder(self):
        """🔖start_working_in_local_kit loads kit from folder containing metabolism.kit.semio.json."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.start_working_in_local_kit(str(ASSETS_DIR), mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert "kitArtifacts" in payload
        kit = engine._mcp_session_kits[mock_ctx.session]
        assert "designs" in kit

    def test_start_working_in_local_kit_loads_from_metabolism_folder(self):
        """🖼️start_working_in_local_kit loads kit from a folder backed by .semio/kit.db (semio/assets/semio/metabolism)."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.start_working_in_local_kit(str(METABOLISM_DIR), mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert "kitArtifacts" in payload
        assert "kit" in payload and isinstance(payload["kit"], dict)
        nakagin = next(design for design in payload["kit"].get("designs", []) if design.get("id") == "9a890dd4-0a9c-48ac-920a-9e62666465ef")
        assert len(nakagin.get("pieces", [])) > 100
        assert payload["kitArtifacts"]["name"] == "Metabolism"
        assert payload["kitArtifacts"].get("version") == "r25.07-1"
        flat_variant = next(design for design in payload["kitArtifacts"]["designs"] if design.get("id") == "019ab4e0-7295-7e1e-bb5f-9dfae8c0c4cf")
        assert flat_variant.get("parent") == {"id": "9a890dd4-0a9c-48ac-920a-9e62666465ef"}
        root_design = next(design for design in payload["kitArtifacts"]["designs"] if design.get("id") == "9a890dd4-0a9c-48ac-920a-9e62666465ef")
        assert "Japanese Metabolism" in root_design.get("description", "")
        assert root_design.get("image") == "images/nakagin-capsule-tower.png"
        ellipsoid = next(kind for kind in payload["kitArtifacts"]["types"] if kind.get("id") == "4ca3b87b-cd76-4228-9f7e-1459b711f0ab")
        assert ellipsoid.get("parent") == {"id": "71749140-9db9-43f6-bd81-d89011667b80"}
        assert ellipsoid.get("name") == "Ellipsoid"
        kit = engine._mcp_session_kits[mock_ctx.session]
        assert "designs" in kit
        assert any(design.get("name") == "Nakagin Capsule Tower" for design in kit.get("designs", []))

    def test_metabolism_folder_path_returns_metabolism_and_nakagin_design_scene_and_diagram(self):
        """🔖start_working_in_local_kit(metabolism dir) exposes Metabolism; start_working_in_design(nakagin id) returns design+kit and diagram points/lines."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        metabolism_path = METABOLISM_DIR.resolve()
        workspace_default = pathlib.Path("/workspaces/semio/semio/assets/semio/metabolism")
        path_arg = str(workspace_default) if workspace_default.is_dir() else str(metabolism_path)

        kit_result = engine.start_working_in_local_kit(path_arg, mock_ctx)
        assert isinstance(kit_result, CallToolResult)
        kit_payload = _mcp_app_tool_payload(kit_result)
        assert "Metabolism" in json.dumps(kit_payload)
        assert kit_payload.get("kitArtifacts", {}).get("name") == "Metabolism"

        nakagin_id = "9a890dd4-0a9c-48ac-920a-9e62666465ef"
        design_result = engine.start_working_in_design(nakagin_id, mock_ctx)
        assert isinstance(design_result, CallToolResult)
        d_payload = _mcp_app_tool_payload(design_result)
        assert d_payload.get("mode") == "show-design"
        assert d_payload.get("surface") == "design"
        assert d_payload.get("design", {}).get("id") == nakagin_id
        assert len(d_payload.get("design", {}).get("pieces", [])) > 0
        assert "kit" in d_payload and isinstance(d_payload["kit"], dict)
        assert "points" in d_payload and isinstance(d_payload["points"], list) and len(d_payload["points"]) > 0
        assert "lines" in d_payload and isinstance(d_payload["lines"], list) and len(d_payload["lines"]) > 0

    def test_build_kit_artifact_data_preserves_parent_dependencies(self):
        """🔖_build_kit_artifact_data keeps nested design and type parent refs for breadcrumb chains."""
        payload = engine._build_kit_artifact_data(
            {
                "id": "kit-id",
                "name": "Metabolism",
                "version": "1",
                "description": "Kit description",
                "homepage": "https://example.com/kit",
                "designs": [
                    {"id": "root-design", "name": "Root", "description": "Root design", "image": "root.png"},
                    {"id": "child-design", "name": "Child", "parent": {"id": "root-design"}, "createdAt": "2026-03-27T00:00:00Z"},
                ],
                "types": [
                    {"id": "root-kind", "name": "Root Kind"},
                    {"id": "child-kind", "name": "Child Kind", "parent": {"id": "root-kind"}, "description": "Child kind", "connectors": []},
                ],
            }
        )

        assert payload["description"] == "Kit description"
        assert payload["homepage"] == "https://example.com/kit"
        assert payload["designs"][0]["description"] == "Root design"
        assert payload["designs"][0]["image"] == "root.png"
        assert payload["designs"][1]["parent"] == {"id": "root-design"}
        assert payload["designs"][1]["createdAt"] == "2026-03-27T00:00:00Z"
        assert payload["types"][1]["parent"] == {"id": "root-kind"}
        assert payload["types"][1]["description"] == "Child kind"

    def test_build_kit_artifact_data_splits_kit_ports_and_type_connectors(self):
        """🔖kitArtifacts.ports lists Port entities; kitArtifacts.connectors lists flattened Connector rows."""
        payload = engine._build_kit_artifact_data(
            {
                "name": "K",
                "version": "1",
                "ports": [{"id": "port-entity", "name": "Wall inlet"}],
                "designs": [],
                "types": [
                    {
                        "id": "t1",
                        "name": "T",
                        "connectors": [
                            {"id": "c1", "name": "C1", "port": {"id": "port-entity"}},
                        ],
                    }
                ],
            }
        )
        assert payload["ports"] == [{"id": "port-entity", "name": "Wall inlet"}]
        assert len(payload["connectors"]) == 1
        assert payload["connectors"][0]["id"] == "c1"
        assert payload["connectors"][0]["typeId"] == "t1"
        assert payload["connectors"][0]["port"] == "port-entity"

    def test_start_working_in_local_kit_clears_design_and_type(self):
        """🔖start_working_in_local_kit clears any previously set design and type."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_designs[sid] = {"id": "old-design"}
        engine._mcp_session_types[sid] = {"id": "old-type"}
        engine.start_working_in_local_kit(str(KIT_METABOLISM_PATH), mock_ctx)
        assert sid not in engine._mcp_session_designs
        assert sid not in engine._mcp_session_types

    def test_start_working_in_local_kit_and_sum_quality_metabolism(self, kitMetabolismJson: dict):
        """🔖start_working_in_local_kit then sum_quality_in_design for Nakagin effective floor area."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine._mcp_session_kits[mock_ctx.session] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        quality = next(q for q in kitMetabolismJson.get("qualities", []) if q.get("name") == "effective floor area")
        result = engine.sum_quality_in_design(design["id"], quality["id"], mock_ctx)
        assert abs(result.get("result") - 2349.53) < 0.01

    def test_start_working_in_design(self, kitMetabolismJson: dict):
        """🔖start_working_in_design selects a design by ID from the session kit and opens the MCP app payload (scene + diagram)."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        result = engine.start_working_in_design(design["id"], mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert payload["mode"] == "show-design"
        assert payload.get("surface") == "design"
        assert "design" in payload and isinstance(payload["design"], dict)
        assert "kit" in payload and isinstance(payload["kit"], dict)
        assert "points" in payload and isinstance(payload["points"], list) and len(payload["points"]) > 0
        assert "lines" in payload and isinstance(payload["lines"], list) and len(payload["lines"]) > 0
        assert "kitArtifacts" in payload
        assert "designs" in payload["kitArtifacts"]
        assert sid in engine._mcp_session_designs
        assert engine._mcp_session_designs[sid]["id"] == design["id"]

    def test_start_working_in_design_not_found(self, kitMetabolismJson: dict):
        """❌start_working_in_design returns error for unknown ID."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine._mcp_session_kits[mock_ctx.session] = kitMetabolismJson
        result = engine.start_working_in_design("nonexistent-id", mock_ctx)
        assert isinstance(result, CallToolResult)
        assert result.isError is True
        payload = _mcp_app_tool_payload(result)
        assert "error" in payload

    def test_read_current_design(self, kitMetabolismJson: dict):
        """🔖read_current_design returns the design set by start_working_in_design."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        result = engine.read_current_design(mock_ctx)
        assert result.get("id") == design["id"]
        assert result.get("name") == "Nakagin Capsule Tower"

    def test_read_current_design_without_start(self):
        """🔖read_current_design returns error if no design was set."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.read_current_design(mock_ctx)
        assert "error" in result

    def test_finish_working_in_design(self, kitMetabolismJson: dict):
        """🔖finish_working_in_design clears the current design from session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        assert sid in engine._mcp_session_designs
        result = engine.finish_working_in_design(mock_ctx)
        assert result.get("ok") is True
        assert sid not in engine._mcp_session_designs

    def test_start_working_in_type(self, kitMetabolismJson: dict):
        """🔖start_working_in_type selects a type by ID from the session kit."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        t = kitMetabolismJson.get("types", [])[0]
        result = engine.start_working_in_type(t["id"], mock_ctx)
        assert result.get("ok") is True
        assert result.get("id") == t["id"]
        assert sid in engine._mcp_session_types
        assert engine._mcp_session_types[sid]["id"] == t["id"]

    def test_start_working_in_type_not_found(self, kitMetabolismJson: dict):
        """🔖start_working_in_type returns error for unknown ID."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine._mcp_session_kits[mock_ctx.session] = kitMetabolismJson
        result = engine.start_working_in_type("nonexistent-id", mock_ctx)
        assert "error" in result

    def test_read_current_type(self, kitMetabolismJson: dict):
        """🔖read_current_type returns the type set by start_working_in_type."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        t = kitMetabolismJson.get("types", [])[0]
        engine.start_working_in_type(t["id"], mock_ctx)
        result = engine.read_current_type(mock_ctx)
        assert result.get("id") == t["id"]

    def test_read_current_type_without_start(self):
        """🔖read_current_type returns error if no type was set."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.read_current_type(mock_ctx)
        assert "error" in result

    def test_finish_working_in_type(self, kitMetabolismJson: dict):
        """🔖finish_working_in_type clears the current type from session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        t = kitMetabolismJson.get("types", [])[0]
        engine.start_working_in_type(t["id"], mock_ctx)
        assert sid in engine._mcp_session_types
        result = engine.finish_working_in_type(mock_ctx)
        assert result.get("ok") is True
        assert sid not in engine._mcp_session_types

    def test_finish_working_in_kit(self, kitMetabolismJson: dict):
        """🔖finish_working_in_kit clears kit, design, and type from session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        t = kitMetabolismJson.get("types", [])[0]
        engine.start_working_in_type(t["id"], mock_ctx)
        result = engine.finish_working_in_kit(mock_ctx)
        assert result.get("ok") is True
        assert sid not in engine._mcp_session_kits
        assert sid not in engine._mcp_session_designs
        assert sid not in engine._mcp_session_types

    def test_start_transaction_rejects_nested_transaction(self):
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        try:
            first = engine.start_transaction(mock_ctx)
            second = engine.start_transaction(mock_ctx)
            assert first.get("ok") is True
            assert "error" in second
        finally:
            engine._mcp_session_transactions.pop(sid, None)

    def test_finalize_transaction_removes_active_transaction(self):
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        started = engine.start_transaction(mock_ctx)
        assert started.get("ok") is True
        result = engine.finalize_transaction(mock_ctx)
        assert result.get("ok") is True
        assert sid not in engine._mcp_session_transactions

    def test_abort_transaction_unwinds_recorded_kit_changes(self):
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        initial_kit = {"name": "Initial", "version": "1.0.0", "designs": [], "types": []}
        changed_kit = {"name": "Changed", "version": "1.0.0", "designs": [], "types": []}
        engine._mcp_session_kits[sid] = initial_kit
        started = engine.start_transaction(mock_ctx)
        assert started.get("ok") is True
        engine._set_session_kit(mock_ctx, changed_kit)
        engine._clear_session_kit(mock_ctx)
        assert sid not in engine._mcp_session_kits
        result = engine.abort_transaction(mock_ctx)
        assert result.get("ok") is True
        assert sid not in engine._mcp_session_transactions
        assert sid in engine._mcp_session_kits
        assert engine._mcp_session_kits[sid].get("name") == "Initial"

    def test_abort_transaction_without_active_transaction_returns_error(self):
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.abort_transaction(mock_ctx)
        assert "error" in result

    def test_stateful_flat_tools_build_nakagin_capsule_tower_flat_payload(self, kitMetabolismJson: dict):
        mock_ctx = type("MockCtx", (), {"session": object()})()
        expected_design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))

        started_kit = engine.start_new_kit("Temporary Kit", "1.0.0", mock_ctx)
        assert isinstance(started_kit, CallToolResult)
        started_kit_payload = _mcp_app_tool_payload(started_kit)
        assert "kitArtifacts" in started_kit_payload

        started_transaction = engine.start_transaction(mock_ctx)
        assert started_transaction.get("ok") is True
        aborted_design = engine.start_new_design(
            "aborted-design",
            "Aborted Draft",
            "should be rolled back",
            "m",
            "icons/aborted.svg",
            "images/aborted.png",
            "2025-01-01T00:00:00.000Z",
            "2025-01-01T00:00:00.000Z",
            mock_ctx,
        )
        assert aborted_design.get("ok") is True
        aborted_piece = engine.add_current_design_piece(
            "aborted-piece",
            "x",
            "aborted-kind",
            mock_ctx,
        )
        assert aborted_piece.get("ok") is True
        aborted = engine.transaction_abort(mock_ctx)
        assert aborted.get("ok") is True
        current_kit_after_abort = engine.read_current_kit(mock_ctx)
        assert current_kit_after_abort.get("designs") == []
        assert "error" in engine.read_current_design(mock_ctx)

        started_transaction = engine.start_transaction(mock_ctx)
        assert started_transaction.get("ok") is True
        created_design = engine.start_new_design(
            expected_design["id"],
            expected_design["name"],
            expected_design["description"],
            expected_design["unit"],
            expected_design["icon"],
            expected_design["image"],
            expected_design["createdAt"],
            expected_design["updatedAt"],
            mock_ctx,
        )
        assert created_design.get("ok") is True

        for author in expected_design.get("authors", []):
            result = engine.add_current_design_author(author["id"], mock_ctx)
            assert result.get("ok") is True

        for prop in expected_design.get("props", []):
            result = engine.add_current_design_prop(
                prop["id"],
                prop["quality"]["id"],
                prop["value"],
                prop["unit"],
                mock_ctx,
            )
            assert result.get("ok") is True

        for piece in expected_design.get("pieces", []):
            pose = piece.get("pose") or {}
            plane = pose.get("plane")
            center = pose.get("center")
            if plane is not None and center is not None:
                result = engine.add_current_design_piece_with_plane(
                    piece["id"],
                    piece["name"],
                    piece["type"]["id"],
                    center["u"],
                    center["v"],
                    plane["origin"]["x"],
                    plane["origin"]["y"],
                    plane["origin"]["z"],
                    plane["xAxis"]["x"],
                    plane["xAxis"]["y"],
                    plane["xAxis"]["z"],
                    plane["yAxis"]["x"],
                    plane["yAxis"]["y"],
                    plane["yAxis"]["z"],
                    mock_ctx,
                    description=piece["description"],
                    is_hidden=piece["isHidden"],
                    is_locked=piece["isLocked"],
                )
            else:
                result = engine.add_current_design_piece(
                    piece["id"],
                    piece["name"],
                    piece["type"]["id"],
                    mock_ctx,
                    description=piece["description"],
                    is_hidden=piece["isHidden"],
                    is_locked=piece["isLocked"],
                )
            assert result.get("ok") is True

        for connection in expected_design.get("connections", []):
            result = engine.add_current_design_connection(
                connection["id"],
                connection["parent"]["piece"]["id"],
                connection["parent"]["connector"]["id"],
                connection["child"]["piece"]["id"],
                connection["child"]["connector"]["id"],
                connection["rotation"],
                connection["u"],
                connection["v"],
                connection["shift"],
                mock_ctx,
                description=connection["description"],
                gap=connection["gap"],
                rise=connection["rise"],
                tilt=connection["tilt"],
                turn=connection["turn"],
            )
            assert result.get("ok") is True

        finalized = engine.transaction_finalize(mock_ctx)
        assert finalized.get("ok") is True
        current_design = engine.read_current_design(mock_ctx)
        expected_flat_design = {key: value for key, value in expected_design.items() if key != "layers"}
        assert current_design == expected_flat_design
        assert "layers" not in current_design

    def test_read_current_selection_default_empty(self):
        """🔖read_current_selection returns empty lists when no selection is set."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.read_current_selection(mock_ctx)
        assert result == {"pieceIds": [], "connectionIds": []}

    def test_set_current_selection_pieces(self):
        """🔖set_current_selection stores piece ids in session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.set_current_selection(mock_ctx, piece_ids=["p1", "p2"])
        assert result.get("ok") is True
        sel = engine.read_current_selection(mock_ctx)
        assert sel["pieceIds"] == ["p1", "p2"]
        assert sel["connectionIds"] == []

    def test_set_current_selection_connections(self):
        """🔖set_current_selection stores connection ids in session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.set_current_selection(mock_ctx, connection_ids=["c1", "c2"])
        assert result.get("ok") is True
        sel = engine.read_current_selection(mock_ctx)
        assert sel["pieceIds"] == []
        assert sel["connectionIds"] == ["c1", "c2"]

    def test_set_current_selection_both(self):
        """🔖set_current_selection stores both piece and connection ids."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.set_current_selection(mock_ctx, piece_ids=["p1"], connection_ids=["c1"])
        assert result.get("ok") is True
        sel = engine.read_current_selection(mock_ctx)
        assert sel["pieceIds"] == ["p1"]
        assert sel["connectionIds"] == ["c1"]

    def test_clear_current_selection(self):
        """🚚clear_current_selection removes selection from session."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        engine.set_current_selection(mock_ctx, piece_ids=["p1"])
        result = engine.clear_current_selection(mock_ctx)
        assert result.get("ok") is True
        sel = engine.read_current_selection(mock_ctx)
        assert sel == {"pieceIds": [], "connectionIds": []}

    def test_show_design_returns_diagram_json(self, kitMetabolismJson: dict):
        """🔖show_design returns CallToolResult with design, kit, mode=show-design, and diagram points/lines for the split viewer."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        result = engine.show_design(mock_ctx)
        assert isinstance(result, CallToolResult)
        data = _mcp_app_tool_payload(result)
        assert data["mode"] == "show-design"
        assert "design" in data and isinstance(data["design"], dict)
        assert "kit" in data and isinstance(data["kit"], dict)
        assert "points" in data and isinstance(data["points"], list) and len(data["points"]) > 0
        assert "lines" in data and isinstance(data["lines"], list) and len(data["lines"]) > 0
        assert "capabilities" in data
        assert isinstance(data.get("fetchUrl"), str)
        assert f":{engine.PORT}/api/app/payload/" in data["fetchUrl"]

    def test_show_diagram_returns_diagram_json(self, kitMetabolismJson: dict):
        """🔖show_diagram returns design, mode=show-diagram and diagram data."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        result = engine.show_diagram(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert data["mode"] == "show-diagram"
        assert "design" in data and isinstance(data["design"], dict)
        assert "points" in data and isinstance(data["points"], list)
        assert "lines" in data and isinstance(data["lines"], list)

    def test_show_scene_returns_scene_data(self, kitMetabolismJson: dict):
        """🔖show_scene returns design, kit, mode=show-scene, and diagram points/lines for context."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        result = engine.show_scene(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert data["mode"] == "show-scene"
        assert "design" in data and isinstance(data["design"], dict)
        assert "kit" in data and isinstance(data["kit"], dict)
        assert "points" in data and isinstance(data["points"], list) and len(data["points"]) > 0
        assert "lines" in data and isinstance(data["lines"], list) and len(data["lines"]) > 0
        assert isinstance(data.get("fetchUrl"), str)
        assert f":{engine.PORT}/api/app/payload/" in data["fetchUrl"]

    def test_show_diff_returns_design_diff(self, kitMetabolismJson: dict):
        """🔖show_diff returns design, kit, designDiff, mode=show-diff (no diagram points/lines)."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        diff = {"pieces": {"added": [], "removed": [], "updated": []}, "connections": {"added": [], "removed": [], "updated": []}}
        result = engine.show_diff(mock_ctx, design_diff=diff)
        data = _mcp_app_tool_payload(result)
        assert data["mode"] == "show-diff"
        assert "design" in data and isinstance(data["design"], dict)
        assert "kit" in data and isinstance(data["kit"], dict)
        assert "designDiff" in data and isinstance(data["designDiff"], dict)
        assert "points" not in data
        assert "lines" not in data

    def test_show_diagram_diff_returns_diagram_diff(self, kitMetabolismJson: dict):
        """🔖show_diagram_diff returns design, designDiff, mode=show-diagram-diff with diagram data."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        diff = {"pieces": {"added": [], "removed": [], "updated": []}, "connections": {"added": [], "removed": [], "updated": []}}
        result = engine.show_diagram_diff(mock_ctx, design_diff=diff)
        data = _mcp_app_tool_payload(result)
        assert data["mode"] == "show-diagram-diff"
        assert "design" in data and isinstance(data["design"], dict)
        assert "designDiff" in data and isinstance(data["designDiff"], dict)
        assert "points" in data and isinstance(data["points"], list)
        assert "lines" in data and isinstance(data["lines"], list)

    def test_shallow_kit_hydrates_nakagin_design_from_disk(self):
        """🔖metabolism.shallow.kit.semio.json lists designs without pieces; load nakagin-capsule-tower.shallow.design.semio.json by id."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        shallow_kit_path = ASSETS_DIR / "metabolism.shallow.kit.semio.json"
        engine.start_working_in_local_kit(str(shallow_kit_path), mock_ctx)
        engine.start_working_in_design("9a890dd4-0a9c-48ac-920a-9e62666465ef", mock_ctx)
        d = engine._get_session_design(mock_ctx)
        assert len(d.get("pieces", [])) > 50

    def test_hydrate_design_searches_parent_of_folder_kit(self):
        """🔎*.design.semio.json for Nakagin lives next to the metabolism folder, not inside it."""
        shallow = {"id": "9a890dd4-0a9c-48ac-920a-9e62666465ef", "name": "Nakagin Capsule Tower", "pieces": []}
        out = engine._hydrate_design_from_kit_disk_if_shallow(
            shallow,
            str(METABOLISM_DIR),
            "9a890dd4-0a9c-48ac-920a-9e62666465ef",
        )
        assert len(out.get("pieces", [])) > 50

    def test_show_diff_returns_diagram_json(self, kitMetabolismJson: dict):
        """🔖show_diff returns design data and default capabilities in structuredContent (no diagram points/lines)."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        result = engine.show_diff(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert "points" not in data
        assert "lines" not in data
        assert data["capabilities"]["pieceSelection"] is False
        assert data["capabilities"]["connectionSelection"] is False

    def test_show_diagram_diff_returns_diagram_json(self, kitMetabolismJson: dict):
        """🔖show_diagram_diff returns diagram data in structuredContent."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        result = engine.show_diagram_diff(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert "points" in data
        assert "lines" in data

    def test_show_diagram_diff_flattens_the_diffed_design(self):
        """🔖show_diagram_diff must flatten the design after applying the diff so diagram centers come from the diffed design."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        kit = {
            "name": "Diff Diagram Kit",
            "types": [],
            "designs": [{"id": "dg-1", "name": "D", "pieces": [{"id": "p-1", "id": "p-1"}], "connections": []}],
        }
        engine._mcp_session_kits[sid] = kit
        engine._mcp_session_designs[sid] = kit["designs"][0]

        diff = {"pieces": {"updated": [{"piece": {"id": "p-1"}, "diff": {"center": {"u": 12, "v": -4}}}]}}
        result = engine.show_diagram_diff(mock_ctx, design_diff=diff)
        data = _mcp_app_tool_payload(result)

        points_by_id = {point["id"]: point for point in data["points"]}
        assert points_by_id["p-1"]["u"] == 12
        assert points_by_id["p-1"]["v"] == -4
        piece = next(piece for piece in data["design"]["pieces"] if piece["id"] == "p-1")
        assert piece["center"] == {"u": 12, "v": -4}

    def test_show_diff_with_design_diff_adds_pieces(self, kitMetabolismJson: dict):
        """➕show_diff with design_diff includes designDiff in payload."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        diff = {"pieces": {"added": [{"id": "new-piece", "id": "added-1", "center": {"u": 10, "v": 20}}]}}
        result = engine.show_diff(mock_ctx, design_diff=diff)
        data = _mcp_app_tool_payload(result)
        assert "designDiff" in data
        added = data["designDiff"]["pieces"]["added"]
        assert any(p["id"] == "added-1" for p in added)

    def test_select_pieces_capabilities(self, kitMetabolismJson: dict):
        """🔖select_pieces sets pieceSelection capability."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        result = engine.select_pieces(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert data["capabilities"]["pieceSelection"] is True
        assert data["capabilities"]["connectionSelection"] is False

    def test_select_connections_capabilities(self, kitMetabolismJson: dict):
        """🔖select_connections sets connectionSelection capability."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        result = engine.select_connections(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert data["capabilities"]["pieceSelection"] is False
        assert data["capabilities"]["connectionSelection"] is True

    def test_select_pieces_and_connections_capabilities(self, kitMetabolismJson: dict):
        """🔖select_pieces_and_connections sets both selection capabilities."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        result = engine.select_pieces_and_connections(mock_ctx)
        data = _mcp_app_tool_payload(result)
        assert data["capabilities"]["pieceSelection"] is True
        assert data["capabilities"]["connectionSelection"] is True

    def test_app_tools_require_kit_and_design(self):
        """🔖All app tools return CallToolResult with error in structuredContent when kit or design is not set."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        # Ensure clean state
        engine._mcp_session_kits.pop(sid, None)
        engine._mcp_session_designs.pop(sid, None)
        for tool_fn in (engine.show_design, engine.show_diagram, engine.show_scene, engine.select_pieces, engine.select_connections, engine.select_pieces_and_connections):
            result = tool_fn(mock_ctx)
            assert isinstance(result, CallToolResult), f"{tool_fn.__name__} should return CallToolResult"
            assert result.isError is True, f"{tool_fn.__name__} should signal error"
            data = _mcp_app_tool_payload(result)
            assert "error" in data, f"{tool_fn.__name__} should require kit+design"

    def test_show_design_pieces_have_required_fields(self, kitMetabolismJson: dict):
        """🔖show_design design pieces contain id field."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        design = next(d for d in kitMetabolismJson.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
        engine.start_working_in_design(design["id"], mock_ctx)
        result = engine.show_design(mock_ctx)
        data = _mcp_app_tool_payload(result)
        for piece in data["design"]["pieces"]:
            assert "id" in piece

    def test_selection_isolated_between_sessions(self):
        """🔖Selection state is isolated between different sessions."""
        ctx_a = type("MockCtx", (), {"session": object()})()
        ctx_b = type("MockCtx", (), {"session": object()})()
        engine.set_current_selection(ctx_a, piece_ids=["p1"])
        engine.set_current_selection(ctx_b, piece_ids=["p2"])
        assert engine.read_current_selection(ctx_a)["pieceIds"] == ["p1"]
        assert engine.read_current_selection(ctx_b)["pieceIds"] == ["p2"]


class TestAppEndpoint:
    def test_app_design_viewer_returns_html(self):
        """🔖GET /app/design-viewer returns the built MCP App HTML that uses @semio/ui."""
        client = TestClient(engine.rest)
        response = client.get("/app/design-viewer")
        assert response.status_code == 200
        assert "text/html" in response.headers["content-type"]
        assert "semio design viewer" in response.text

    def test_app_design_viewer_csp_header(self):
        """🔗The app endpoint includes Content-Security-Policy allowing iframe embedding and wasm-unsafe-eval for Three.js scene."""
        client = TestClient(engine.rest)
        response = client.get("/app/design-viewer")
        csp = response.headers["content-security-policy"]
        assert "frame-ancestors *" in csp
        assert "'wasm-unsafe-eval'" in csp
        assert "script-src" in csp
        assert "worker-src blob:" in csp

    def test_app_kit_viewer_csp_header(self):
        """🔖The kit-viewer endpoint includes the same CSP as design-viewer."""
        client = TestClient(engine.rest)
        response = client.get("/app/kit-viewer")
        csp = response.headers["content-security-policy"]
        assert "frame-ancestors *" in csp
        assert "'wasm-unsafe-eval'" in csp

    def test_app_design_viewer_html_structure(self):
        """🔖The HTML contains root element for the React MCP App from @semio/ui."""
        client = TestClient(engine.rest)
        response = client.get("/app/design-viewer")
        html = response.text
        assert 'id="root"' in html

    def test_app_design_viewer_excludes_embedded_js_tests(self):
        """🔖The MCP App bundle excludes @semio/js embedded tests so sketchpad-only code cannot crash viewer startup."""
        client = TestClient(engine.rest)
        response = client.get("/app/design-viewer")
        html = response.text
        assert "Test on temporary kits" not in html
        assert "createFolderKitStore" not in html
        assert "KIT_DIAGRAM_NODE_SCALE" not in html

    def test_app_kit_viewer_returns_html(self):
        """🔖GET /app/kit-viewer returns the built MCP App HTML that mounts McpKitViewer from @semio/ui."""
        client = TestClient(engine.rest)
        response = client.get("/app/kit-viewer")
        assert response.status_code == 200
        assert "text/html" in response.headers["content-type"]
        assert "semio kit viewer" in response.text
        assert 'data-mcp-viewer="kit"' in response.text

    def test_app_scene_viewer_returns_html(self):
        """🔖GET /app/scene-viewer returns the built MCP App HTML that mounts McpSceneViewer from @semio/ui."""
        client = TestClient(engine.rest)
        response = client.get("/app/scene-viewer")
        assert response.status_code == 200
        assert "text/html" in response.headers["content-type"]
        assert 'data-mcp-viewer="scene"' in response.text

    def test_app_diagram_viewer_returns_html(self):
        """🔖GET /app/diagram-viewer returns the built MCP App HTML that mounts McpDiagramViewer from @semio/ui."""
        client = TestClient(engine.rest)
        response = client.get("/app/diagram-viewer")
        assert response.status_code == 200
        assert "text/html" in response.headers["content-type"]
        assert 'data-mcp-viewer="diagram"' in response.text


# #endregion 🎖️MCP Tests


# #region 🔐Cache Tests
class TestCache:
    def test_cache_dir_encoding(self):
        remoteUri = "https://example.com/kit.zip"
        expectedDir = os.path.join(os.path.expanduser("~/.semio/cache"), engine.encode(remoteUri))
        assert engine.cacheDir(remoteUri) == expectedDir

    def test_cache_rejects_non_remote(self, tempKitPath: pathlib.Path):
        localUri = str(tempKitPath)
        with pytest.raises(engine.OnlyRemoteKitsCanBeCached):
            engine.cache(localUri)

    def test_cache_rejects_non_zip(self):
        nonZipUri = "https://example.com/kit.json"
        with pytest.raises(engine.OnlyRemoteKitsCanBeCached):
            engine.cache(nonZipUri)


# #endregion 🔐Cache Tests


# #region 🔧SSLMode Tests
class TestSSLMode:
    def test_ssl_mode_values(self):
        assert engine.SSLMode.DISABLE.value == "disable"
        assert engine.SSLMode.ALLOW.value == "allow"
        assert engine.SSLMode.PREFER.value == "prefer"
        assert engine.SSLMode.REQUIRE.value == "require"
        assert engine.SSLMode.VERIFY_CA.value == "verify-ca"
        assert engine.SSLMode.VERIFY_FULL.value == "verify-full"


# #endregion 🔧SSLMode Tests


# #region 🌤️Error Classes Tests
class TestErrors:
    def test_kit_not_found_error(self):
        error = engine.KitNotFound("test/path")
        assert "test/path" in str(error)

    def test_kit_already_exists_error(self):
        error = engine.KitAlreadyExists("test/path")
        assert "test/path" in str(error)

    def test_only_remote_kits_can_be_cached_error(self):
        error = engine.OnlyRemoteKitsCanBeCached("/local/path")
        assert "/local/path" in str(error)

    def test_local_kit_uri_is_not_absolute_error(self):
        error = engine.LocalKitUriIsNotAbsolute("relative/path")
        assert "relative/path" in str(error)


# #endregion 🌤️Error Classes Tests


# #region 🖨️Assistant Tests
class TestAssistant:
    def test_encode_for_prompt(self):
        assert engine.encodeForPrompt("hello;world") == "hello,world"
        assert engine.encodeForPrompt("hello\nworld") == "hello world"

    def test_replace_default_empty(self):
        assert engine.replaceDefault("", "DEFAULT") == "DEFAULT"
        assert engine.replaceDefault("value", "DEFAULT") == "value"

    def test_encode_type(self):
        typeContext = engine.TypeContext(name="TestType", variant="", connectors=[])
        encoded = engine.encodeType(typeContext)
        assert encoded.variant == "DEFAULT"

    def test_design_generation_prompt_template_renders(self):
        types = []
        description = "Test description"
        result = engine.designGenerationPromptTemplate.render(description=description, types=types)
        assert "Test description" in result

    def test_design_response_format_is_valid_json(self):
        assert isinstance(engine.designResponseFormat, dict)
        assert "name" in engine.designResponseFormat
        assert engine.designResponseFormat["name"] == "design"


# #endregion 🖨️Assistant Tests


# #region 🌥️Engine Configuration Tests
class TestEngineConfiguration:
    def test_engine_app_exists(self):
        assert engine.engine is not None

    def test_rest_app_exists(self):
        assert engine.rest is not None

    def test_mcp_app_exists(self):
        assert engine.mcp is not None

    def test_graphql_schema_exists(self):
        assert engine.graphqlSchema is not None


# #endregion 🌥️Engine Configuration Tests


# #region 🐍Integration Tests
class TestIntegration:
    def test_store_initialization_and_semio_check(self, tempKitPath: pathlib.Path):
        store = engine.SqliteStore.fromUri(str(tempKitPath))
        store.initialize()
        assert store.initialized()

    def test_store_and_operation_from_code(self, tempKitPath: pathlib.Path):
        engine.StoreFactory.cache_clear()
        code = engine.encode(str(tempKitPath))
        resultStore, resultOperation = engine.storeAndOperationFromCode(code)
        assert resultOperation["kind"] == engine.OperationKind.KIT
        assert resultOperation["kitUri"] == str(tempKitPath)


# #endregion 🐍Integration Tests


# #region 🩻Auth Error Classes Tests
class TestAuthErrors:
    def test_authentication_error(self):
        error = engine.AuthenticationError()
        assert "Authentication failed" in str(error)

    def test_invalid_auth_token_error(self):
        error = engine.InvalidAuthToken("https://example.com")
        assert "https://example.com" in str(error)
        assert "invalid or expired" in str(error)

    def test_auth_token_not_found_error(self):
        error = engine.AuthTokenNotFound("https://example.com")
        assert "https://example.com" in str(error)
        assert "No auth token found" in str(error)

    def test_server_unreachable_error(self):
        error = engine.ServerUnreachable("https://example.com")
        assert "https://example.com" in str(error)
        assert "not reachable" in str(error)

    def test_remote_kit_uri_not_valid_error(self):
        error = engine.RemoteKitUriNotValid("http://bad-uri")
        assert "http://bad-uri" in str(error)
        assert "not valid" in str(error)


# #endregion 🩻Auth Error Classes Tests


# #region ⚗️Auth Credential Management Tests
class TestAuthCredentials:
    def test_load_auth_empty(self, tmp_path):
        """🔖_load_auth returns empty dict when no auth file exists."""
        with patch.object(engine, "AUTH_FILE", str(tmp_path / "auth.json")):
            result = engine._load_auth()
            assert result == {}

    def test_save_and_load_auth(self, tmp_path):
        """✏️_save_auth writes and _load_auth reads auth credentials."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            auth_data = {"https://server.com": {"token": "tok123", "email": "user@test.com"}}
            engine._save_auth(auth_data)
            loaded = engine._load_auth()
            assert loaded == auth_data

    def test_get_auth_token_found(self, tmp_path):
        """🔖getAuthToken returns the stored token for a server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            token = engine.getAuthToken("https://server.com")
            assert token == "tok123"

    def test_get_auth_token_not_found(self, tmp_path):
        """🔖getAuthToken raises AuthTokenNotFound when no token exists."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            with pytest.raises(engine.AuthTokenNotFound):
                engine.getAuthToken("https://server.com")

    def test_get_auth_token_strips_trailing_slash(self, tmp_path):
        """🔖getAuthToken strips trailing slash from server URL."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            token = engine.getAuthToken("https://server.com/")
            assert token == "tok123"

    def test_get_auth_status_authenticated(self, tmp_path):
        """🔖getAuthStatus returns authenticated=True when token exists."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            status = engine.getAuthStatus("https://server.com")
            assert status["authenticated"] is True
            assert status["email"] == "user@test.com"

    def test_get_auth_status_not_authenticated(self, tmp_path):
        """🔖getAuthStatus returns authenticated=False when no token exists."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            status = engine.getAuthStatus("https://server.com")
            assert status["authenticated"] is False
            assert status["email"] == ""

    def test_login_success(self, tmp_path):
        """🔖login stores token on successful server response."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.json.return_value = {"token": "new-token-123"}
        mock_response.raise_for_status.return_value = None
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", return_value=mock_response):
            result = engine.login("https://server.com", "user@test.com", "pass123")
            assert result["ok"] is True
            assert result["token"] == "new-token-123"
            assert result["email"] == "user@test.com"
            loaded = engine._load_auth()
            assert "https://server.com" in loaded
            assert loaded["https://server.com"]["token"] == "new-token-123"

    def test_login_connection_error(self, tmp_path):
        """🔖login raises ServerUnreachable on connection error."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", side_effect=engine.requests.exceptions.ConnectionError):
            with pytest.raises(engine.ServerUnreachable):
                engine.login("https://unreachable.com", "user@test.com", "pass")

    def test_login_401_error(self, tmp_path):
        """🔖login raises InvalidAuthToken on 401 response."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.status_code = 401
        http_error = engine.requests.exceptions.HTTPError(response=mock_response)
        mock_response.raise_for_status.side_effect = http_error
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", return_value=mock_response):
            with pytest.raises(engine.InvalidAuthToken):
                engine.login("https://server.com", "user@test.com", "wrong-pass")

    def test_logout_removes_token(self, tmp_path):
        """🔖logout removes the stored token for a server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            result = engine.logout("https://server.com")
            assert result["ok"] is True
            loaded = engine._load_auth()
            assert "https://server.com" not in loaded

    def test_logout_nonexistent_server(self, tmp_path):
        """🔖logout succeeds even if server was never logged in."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            result = engine.logout("https://nonexistent.com")
            assert result["ok"] is True


# #endregion ⚗️Auth Credential Management Tests


# #region 🖲️RemoteStore Tests
class TestRemoteStore:
    def test_from_uri_valid(self):
        """🔖RemoteStore.fromUri parses server URL and kit URI from remote URI."""
        uri = "https://server.com/api/kits/my-kit"
        store = engine.RemoteStore.fromUri(uri)
        assert store.serverUrl == "https://server.com"
        assert store.kitUri == "my-kit"
        assert store.uri == uri

    def test_from_uri_with_encoded_kit(self):
        """🔖RemoteStore.fromUri handles encoded kit URI."""
        encodedKit = engine.encode("/path/to/kit")
        uri = f"https://server.com/api/kits/{encodedKit}"
        store = engine.RemoteStore.fromUri(uri)
        assert store.serverUrl == "https://server.com"
        assert store.kitUri == "/path/to/kit"

    def test_from_uri_invalid(self):
        """🔖RemoteStore.fromUri raises RemoteKitUriNotValid for bad URIs."""
        with pytest.raises(engine.RemoteKitUriNotValid):
            engine.RemoteStore.fromUri("https://server.com/bad/path")

    def test_get_kit_success(self, tmp_path):
        """🔖RemoteStore.get retrieves kit from remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.json.return_value = {"uri": "my-kit", "name": "TestKit", "version": "1.0.0"}
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.get", return_value=mock_response):
                result = store.get({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})
                assert result is not None
                assert result.name == "TestKit"

    def test_get_kit_unauthorized(self, tmp_path):
        """🔖RemoteStore.get raises InvalidAuthToken on 401."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "expired-token", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.status_code = 401
            http_error = engine.requests.exceptions.HTTPError(response=mock_response)
            mock_response.raise_for_status.side_effect = http_error
            with patch("engine.requests.get", return_value=mock_response):
                with pytest.raises(engine.InvalidAuthToken):
                    store.get({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})

    def test_get_kit_not_found(self, tmp_path):
        """🔖RemoteStore.get raises KitNotFound on 404."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.status_code = 404
            http_error = engine.requests.exceptions.HTTPError(response=mock_response)
            mock_response.raise_for_status.side_effect = http_error
            with patch("engine.requests.get", return_value=mock_response):
                with pytest.raises(engine.KitNotFound):
                    store.get({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})

    def test_get_kit_connection_error(self, tmp_path):
        """🔖RemoteStore.get raises ServerUnreachable on connection error."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            with patch("engine.requests.get", side_effect=engine.requests.exceptions.ConnectionError):
                with pytest.raises(engine.ServerUnreachable):
                    store.get({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})

    def test_get_kit_no_auth(self, tmp_path):
        """🔖RemoteStore.get raises AuthTokenNotFound when not logged in."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            with pytest.raises(engine.AuthTokenNotFound):
                store.get({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})

    def test_put_kit_success(self, tmp_path):
        """🔖RemoteStore.put creates a kit on the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.put", return_value=mock_response):
                result = store.put({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"}, engine.KitInput(name="TestKit", version="1.0.0"))
                assert result is None

    def test_put_type_success(self, tmp_path):
        """🔖RemoteStore.put creates a type on the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.put", return_value=mock_response) as mock_put:
                result = store.put(
                    {"kind": engine.OperationKind.TYPE, "kitUri": "my-kit", "typeName": "Brick", "typeVariant": ""},
                    engine.TypeInput(name="Brick", variant=""),
                )
                assert result is None
                call_args = mock_put.call_args
                assert "types/" in call_args[0][0]

    def test_put_design_success(self, tmp_path):
        """🔖RemoteStore.put creates a design on the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.put", return_value=mock_response) as mock_put:
                result = store.put(
                    {"kind": engine.OperationKind.DESIGN, "kitUri": "my-kit", "designName": "MyDesign", "designVariant": "", "designView": ""},
                    engine.DesignInput(name="MyDesign", variant="", view=""),
                )
                assert result is None
                call_args = mock_put.call_args
                assert "designs/" in call_args[0][0]

    def test_delete_kit_success(self, tmp_path):
        """🗑️RemoteStore.delete removes a kit from the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.delete", return_value=mock_response):
                result = store.delete({"kind": engine.OperationKind.KIT, "kitUri": "my-kit"})
                assert result is None

    def test_delete_type_success(self, tmp_path):
        """🔖RemoteStore.delete removes a type from the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.delete", return_value=mock_response) as mock_del:
                result = store.delete({"kind": engine.OperationKind.TYPE, "kitUri": "my-kit", "typeName": "Brick", "typeVariant": ""})
                assert result is None
                call_args = mock_del.call_args
                assert "types/" in call_args[0][0]

    def test_delete_design_success(self, tmp_path):
        """🔖RemoteStore.delete removes a design from the remote server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            mock_response = MagicMock()
            mock_response.raise_for_status.return_value = None
            with patch("engine.requests.delete", return_value=mock_response) as mock_del:
                result = store.delete({"kind": engine.OperationKind.DESIGN, "kitUri": "my-kit", "designName": "MyDesign", "designVariant": "", "designView": ""})
                assert result is None
                call_args = mock_del.call_args
                assert "designs/" in call_args[0][0]

    def test_initialize_noop(self, tmp_path):
        """🔖RemoteStore.initialize is a no-op (server-side initialization)."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            store.initialize()  # Should not raise

    def test_update_not_supported(self, tmp_path):
        """🔁RemoteStore.update raises FeatureNotYetSupported."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            store = engine.RemoteStore("https://server.com/api/kits/my-kit", "https://server.com", "my-kit")
            with pytest.raises(engine.FeatureNotYetSupported):
                store.update({}, "")


# #endregion 🖲️RemoteStore Tests


# #region ⛅StoreFactory Remote Tests
class TestStoreFactoryRemote:
    def test_store_factory_remote_uri(self, tmp_path):
        """🔖StoreFactory returns RemoteStore for remote server URIs."""
        engine.StoreFactory.cache_clear()
        uri = "https://server.com/api/kits/my-kit"
        store = engine.StoreFactory(uri)
        assert isinstance(store, engine.RemoteStore)
        assert store.serverUrl == "https://server.com"
        assert store.kitUri == "my-kit"

    def test_store_factory_invalid_remote_uri(self):
        """🔖StoreFactory raises RemoteKitUriNotValid for http URIs without /api/kits/."""
        engine.StoreFactory.cache_clear()
        with pytest.raises(engine.RemoteKitUriNotValid):
            engine.StoreFactory("https://server.com/some/other/path")

    def test_store_factory_local_still_works(self, tempKitPath: pathlib.Path):
        """🔖StoreFactory still returns SqliteStore for local absolute paths."""
        engine.StoreFactory.cache_clear()
        store = engine.StoreFactory(str(tempKitPath))
        assert isinstance(store, engine.SqliteStore)

    def test_store_factory_relative_path_raises(self):
        """🔖StoreFactory raises LocalKitUriIsNotAbsolute for relative paths."""
        engine.StoreFactory.cache_clear()
        with pytest.raises(engine.LocalKitUriIsNotAbsolute):
            engine.StoreFactory("relative/path")


# #endregion ⛅StoreFactory Remote Tests


# #region 🐼MCP Auth Tools Tests
class TestMcpAuth:
    def test_mcp_login(self, tmp_path):
        """🔖mcp_login calls login and returns result."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.json.return_value = {"token": "mcp-token"}
        mock_response.raise_for_status.return_value = None
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", return_value=mock_response):
            result = engine.mcp_login("https://server.com", "user@test.com", "pass")
            assert result["ok"] is True
            assert result["token"] == "mcp-token"

    def test_mcp_login_error(self, tmp_path):
        """🔖mcp_login returns error dict on connection failure."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", side_effect=engine.requests.exceptions.ConnectionError):
            result = engine.mcp_login("https://unreachable.com", "user@test.com", "pass")
            assert "error" in result

    def test_mcp_logout(self, tmp_path):
        """🔖mcp_logout calls logout and returns result."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            result = engine.mcp_logout("https://server.com")
            assert result["ok"] is True

    def test_mcp_auth_status_authenticated(self, tmp_path):
        """🔖mcp_auth_status returns authenticated status."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            result = engine.mcp_auth_status("https://server.com")
            assert result["authenticated"] is True

    def test_mcp_auth_status_not_authenticated(self, tmp_path):
        """🔖mcp_auth_status returns not authenticated status."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            result = engine.mcp_auth_status("https://unknown.com")
            assert result["authenticated"] is False


# #endregion 🐼MCP Auth Tools Tests


# #region 📎MCP Remote Kit Tests
class TestMcpRemoteKit:
    def test_start_working_in_remote_kit_success(self, tmp_path):
        """🔖start_working_in_remote_kit fetches kit from remote server."""
        auth_file = str(tmp_path / "auth.json")
        kit_data = {"name": "RemoteKit", "version": "1.0.0", "designs": [], "types": []}
        mock_response = MagicMock()
        mock_response.json.return_value = kit_data
        mock_response.raise_for_status.return_value = None
        mock_ctx = type("MockCtx", (), {"session": object()})()
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok123", "email": "user@test.com"}})
            result = engine.start_working_in_remote_kit("https://server.com", "my-kit", mock_ctx)
            assert isinstance(result, CallToolResult)
            payload = _mcp_app_tool_payload(result)
            assert "kitArtifacts" in payload
            sid = mock_ctx.session
            assert sid in engine._mcp_session_kits
            assert engine._mcp_session_kit_mode[sid] == "remote"
            assert "/api/kits/" in engine._mcp_session_kit_source[sid]

    def test_start_working_in_remote_kit_no_auth(self, tmp_path):
        """🔖start_working_in_remote_kit returns error when not logged in."""
        auth_file = str(tmp_path / "auth.json")
        mock_ctx = type("MockCtx", (), {"session": object()})()
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            result = engine.start_working_in_remote_kit("https://server.com", "my-kit", mock_ctx)
            assert isinstance(result, CallToolResult)
            assert result.isError is True
            assert "error" in _mcp_app_tool_payload(result)

    def test_start_working_in_remote_kit_connection_error(self, tmp_path):
        """🔖start_working_in_remote_kit returns error on connection failure."""
        auth_file = str(tmp_path / "auth.json")
        mock_ctx = type("MockCtx", (), {"session": object()})()
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", side_effect=engine.requests.exceptions.ConnectionError):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            result = engine.start_working_in_remote_kit("https://server.com", "my-kit", mock_ctx)
            assert isinstance(result, CallToolResult)
            assert result.isError is True
            assert "error" in _mcp_app_tool_payload(result)

    def test_start_working_in_remote_kit_clears_previous_state(self, tmp_path):
        """🔖start_working_in_remote_kit clears design, type, and sets mode to remote."""
        auth_file = str(tmp_path / "auth.json")
        kit_data = {"name": "RemoteKit", "version": "1.0.0", "designs": [], "types": []}
        mock_response = MagicMock()
        mock_response.json.return_value = kit_data
        mock_response.raise_for_status.return_value = None
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_designs[sid] = {"id": "old-design"}
        engine._mcp_session_types[sid] = {"id": "old-type"}
        engine._mcp_session_kit_mode[sid] = "local"
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            engine.start_working_in_remote_kit("https://server.com", "my-kit", mock_ctx)
            assert sid not in engine._mcp_session_designs
            assert sid not in engine._mcp_session_types
            assert engine._mcp_session_kit_mode[sid] == "remote"

    def test_start_working_in_local_kit_sets_mode_local(self):
        """🔖start_working_in_local_kit sets session mode to local."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        result = engine.start_working_in_local_kit(str(KIT_METABOLISM_PATH), mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert "kitArtifacts" in payload
        sid = mock_ctx.session
        assert engine._mcp_session_kit_mode[sid] == "local"

    def test_get_session_kit_mode_default(self):
        """🔖_get_session_kit_mode returns 'local' when not set."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = engine._session_id(mock_ctx)
        engine._mcp_session_kit_mode.pop(sid, None)
        mode = engine._get_session_kit_mode(mock_ctx)
        assert mode == "local"

    def test_get_session_kit_mode_remote(self, tmp_path):
        """🔖_get_session_kit_mode returns 'remote' for remote kit sessions."""
        auth_file = str(tmp_path / "auth.json")
        kit_data = {"name": "RemoteKit", "version": "1.0.0", "designs": [], "types": []}
        mock_response = MagicMock()
        mock_response.json.return_value = kit_data
        mock_response.raise_for_status.return_value = None
        mock_ctx = type("MockCtx", (), {"session": object()})()
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            engine.start_working_in_remote_kit("https://server.com", "my-kit", mock_ctx)
            mode = engine._get_session_kit_mode(mock_ctx)
            assert mode == "remote"

    def test_finish_working_in_kit_clears_mode_and_source(self, kitMetabolismJson: dict):
        """🔖finish_working_in_kit clears mode and source in addition to kit, design, type."""
        mock_ctx = type("MockCtx", (), {"session": object()})()
        sid = mock_ctx.session
        engine._mcp_session_kits[sid] = kitMetabolismJson
        engine._mcp_session_kit_mode[sid] = "remote"
        engine._mcp_session_kit_source[sid] = "https://server.com/api/kits/test"
        result = engine.finish_working_in_kit(mock_ctx)
        assert result["ok"] is True
        assert sid not in engine._mcp_session_kit_mode
        assert sid not in engine._mcp_session_kit_source

    def test_all_mcp_tools_work_after_remote_kit_login(self, tmp_path):
        """🔖All existing MCP tools work after start_working_in_remote_kit (design/type operations)."""
        auth_file = str(tmp_path / "auth.json")
        kit_data = {
            "name": "RemoteKit",
            "version": "1.0.0",
            "designs": [
                {"id": "d1", "name": "Design1", "pieces": [], "connections": []},
            ],
            "types": [
                {"id": "t1", "name": "Type1", "connectors": []},
            ],
        }
        mock_response = MagicMock()
        mock_response.json.return_value = kit_data
        mock_response.raise_for_status.return_value = None
        mock_ctx = type("MockCtx", (), {"session": object()})()
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            engine.start_working_in_remote_kit("https://server.com", "remote-kit", mock_ctx)

        # start_working_in_design works for remote kits
        result = engine.start_working_in_design("d1", mock_ctx)
        assert isinstance(result, CallToolResult)
        payload = _mcp_app_tool_payload(result)
        assert payload["mode"] == "show-design"
        assert "design" in payload and isinstance(payload["design"], dict)
        assert "kitArtifacts" in payload

        # read_current_design works
        design = engine.read_current_design(mock_ctx)
        assert design["id"] == "d1"

        # finish_working_in_design works
        result = engine.finish_working_in_design(mock_ctx)
        assert result["ok"] is True

        # start_working_in_type works for remote kits
        result = engine.start_working_in_type("t1", mock_ctx)
        assert result["ok"] is True

        # read_current_type works
        t = engine.read_current_type(mock_ctx)
        assert t["id"] == "t1"

        # finish_working_in_type works
        result = engine.finish_working_in_type(mock_ctx)
        assert result["ok"] is True

        # finish_working_in_kit clears everything
        result = engine.finish_working_in_kit(mock_ctx)
        assert result["ok"] is True


# #endregion 📎MCP Remote Kit Tests


# #region 🌎REST Auth Endpoints Tests
class TestRestAuthEndpoints:
    def test_rest_login_endpoint(self, restClient: TestClient, tmp_path):
        """🔖POST /auth/login endpoint calls login and returns token."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.json.return_value = {"token": "rest-token"}
        mock_response.raise_for_status.return_value = None
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.post", return_value=mock_response):
            response = restClient.post(
                "/auth/login",
                json={
                    "serverUrl": "https://server.com",
                    "email": "user@test.com",
                    "password": "pass123",
                },
            )
            assert response.status_code == 200
            data = response.json()
            assert data["ok"] is True
            assert data["token"] == "rest-token"

    def test_rest_logout_endpoint(self, restClient: TestClient, tmp_path):
        """🔖POST /auth/logout endpoint removes token."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            response = restClient.post("/auth/logout", json={"serverUrl": "https://server.com"})
            assert response.status_code == 200
            data = response.json()
            assert data["ok"] is True

    def test_rest_auth_status_endpoint(self, restClient: TestClient, tmp_path):
        """🔖GET /auth/status endpoint returns auth status."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            response = restClient.get("/auth/status", params={"serverUrl": "https://server.com"})
            assert response.status_code == 200
            data = response.json()
            assert data["authenticated"] is True
            assert data["email"] == "user@test.com"

    def test_rest_auth_status_not_authenticated(self, restClient: TestClient, tmp_path):
        """🔖GET /auth/status returns not authenticated for unknown server."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            response = restClient.get("/auth/status", params={"serverUrl": "https://unknown.com"})
            assert response.status_code == 200
            data = response.json()
            assert data["authenticated"] is False


# #endregion 🌎REST Auth Endpoints Tests


# #region 🐙Load Kit From Remote Tests
class TestLoadKitFromRemote:
    def test_load_kit_from_remote_success(self, tmp_path):
        """🔖_load_kit_from_remote fetches kit from server."""
        auth_file = str(tmp_path / "auth.json")
        kit_data = {"name": "RemoteKit", "version": "1.0.0"}
        mock_response = MagicMock()
        mock_response.json.return_value = kit_data
        mock_response.raise_for_status.return_value = None
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            result = engine._load_kit_from_remote("https://server.com", "my-kit")
            assert result["name"] == "RemoteKit"

    def test_load_kit_from_remote_connection_error(self, tmp_path):
        """🔖_load_kit_from_remote raises ServerUnreachable on connection error."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", side_effect=engine.requests.exceptions.ConnectionError):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            with pytest.raises(engine.ServerUnreachable):
                engine._load_kit_from_remote("https://server.com", "my-kit")

    def test_load_kit_from_remote_401(self, tmp_path):
        """🔖_load_kit_from_remote raises InvalidAuthToken on 401."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.status_code = 401
        http_error = engine.requests.exceptions.HTTPError(response=mock_response)
        mock_response.raise_for_status.side_effect = http_error
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "expired", "email": "user@test.com"}})
            with pytest.raises(engine.InvalidAuthToken):
                engine._load_kit_from_remote("https://server.com", "my-kit")

    def test_load_kit_from_remote_404(self, tmp_path):
        """🔖_load_kit_from_remote raises KitNotFound on 404."""
        auth_file = str(tmp_path / "auth.json")
        mock_response = MagicMock()
        mock_response.status_code = 404
        http_error = engine.requests.exceptions.HTTPError(response=mock_response)
        mock_response.raise_for_status.side_effect = http_error
        with patch.object(engine, "AUTH_FILE", auth_file), patch("engine.requests.get", return_value=mock_response):
            engine._save_auth({"https://server.com": {"token": "tok", "email": "user@test.com"}})
            with pytest.raises(engine.KitNotFound):
                engine._load_kit_from_remote("https://server.com", "my-kit")

    def test_load_kit_from_remote_no_token(self, tmp_path):
        """🔖_load_kit_from_remote raises AuthTokenNotFound without login."""
        auth_file = str(tmp_path / "auth.json")
        with patch.object(engine, "AUTH_FILE", auth_file):
            engine._save_auth({})
            with pytest.raises(engine.AuthTokenNotFound):
                engine._load_kit_from_remote("https://server.com", "my-kit")


# #endregion 🐙Load Kit From Remote Tests

# #endregion 🥼Tests

if __name__ == "__main__":
    run()

# #endregion 🔔Engine
