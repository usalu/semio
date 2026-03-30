# region Header
# [👤semio📚engine💻engine](repo://p/u/semio/b/l/engine/f/engine.py)

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

# endregion Header

# region Imports
# [👤semio📚engine💻engine🔖imports](repo://p/u/semio/b/l/engine/f/engine.py/s/Imports)
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
import typing
import uuid
import zipfile

import fastapi
import fastapi.openapi
import graphene
import jinja2
import lark
import pydantic
import requests
import starlette.applications
import starlette.middleware.cors
import starlette_graphene3
import uvicorn
from mcp.server.fastmcp import Context, FastMCP
from mcp.types import CallToolResult, EmbeddedResource, TextContent, TextResourceContents

try:
    import openai  # type: ignore
except Exception:  # pragma: no cover
    openai = None

_semio_core_path = str(pathlib.Path(__file__).parent.parent / "py" / "main.py")
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
    AttributeNode,
    AuthenticationError,
    Author,
    AuthorNode,
    AuthTokenNotFound,
    ClientError,
    CodeUnreachable,
    Connection,
    ConnectionNode,
    Connector,
    ConnectorNode,
    Coord,
    CoordNode,
    Design,
    DesignContext,
    DesignInput,
    DesignNode,
    DesignOutput,
    DesignPrediction,
    Error,
    FeatureNotYetSupported,
    InvalidAuthToken,
    Kit,
    KitAlreadyExists,
    KitContext,
    KitInput,
    KitInputNode,
    KitNode,
    KitNotFound,
    KitOutput,
    KitZipDoesNotContainSemioFolder,
    LocalKitUriIsNotAbsolute,
    Location,
    LocationNode,
    Model,
    ModelNode,
    OnlyRemoteKitsCanBeCached,
    Piece,
    PieceNode,
    Plane,
    PlaneNode,
    Point,
    PointNode,
    RelayNode,
    RemoteKitsNotYetSupported,
    RemoteKitUriNotValid,
    ServerError,
    ServerUnreachable,
    Side,
    SideNode,
    Type,
    TypeContext,
    TypeHasNotAllUsedConnectors,
    TypeInput,
    TypeNode,
    TypeOutput,
    ValidationResult,
    Vector,
    VectorNode,
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

# endregion Imports

# region Store
# [👤semio📚engine💻engine🔖store](repo://p/u/semio/b/l/engine/f/engine.py/s/Store)
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
    """The kind of a store operation.
    [👤semio📚engine💻engine🔖store🛠️operationkind](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/OperationKind)
    """

    KITS = "kits"
    KIT = "kit"
    DESIGNS = "designs"
    DESIGN = "design"
    TYPES = "types"
    TYPE = "type"


class Operation(typing.TypedDict, total=False):
    """Typed operation dict produced by OperationBuilder from parsed code grammar.
    `kind` is always present. Other fields depend on the kind.
    [👤semio📚engine💻engine🔖store🛠️operation](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/Operation)
    """

    kind: typing.Required[OperationKind]
    kitUri: str
    designName: str
    designVariant: str
    designView: str
    typeName: str
    typeVariant: str


class TransactionChange(typing.TypedDict):
    """A single recorded change within a transaction.
    [👤semio📚engine💻engine🔖store🛠️transactionchange](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/TransactionChange)
    """

    kind: str
    before_has_kit: bool
    after_has_kit: bool
    forward_diff: dict | None
    backward_diff: dict | None


class Transaction(typing.TypedDict):
    """An active MCP session transaction tracking kit changes for rollback.
    [👤semio📚engine💻engine🔖store🛠️transaction](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/Transaction)
    """

    active: bool
    started_at: str
    changes: list[TransactionChange]


class OperationBuilder(lark.Transformer):
    """Lark transformer that builds operation dicts from parsed code grammar trees.
    Callers MUST pass a valid parse tree from codeParser.
    [👤semio📚engine💻engine🔖store🛠️operationbuilder](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/OperationBuilder)
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
    [👤semio📚engine💻engine🔖store🛠️storekind](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/StoreKind)
    """

    DATABASE = "database"
    REST = "rest"
    GRAPHQL = "graphql"


class CommandKind(enum.Enum):
    """🔧 The kind of the command.
    Callers MUST use a valid CommandKind when calling Store.execute.
    [👤semio📚engine💻engine🔖store🛠️commandkind](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/CommandKind)
    """

    QUERY = "query"
    PUT = "put"
    UPDATE = "update"
    DELETE = "delete"


class Store(abc.ABC):
    """Abstract base class for all store backends.
    Subclasses MUST implement initialize, get, put, update, and delete methods.
    [👤semio📚engine💻engine🔖store🛠️store](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/Store)
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
    """Abstract database-backed store using raw SQL via sqlite3.
    Stores kit data as JSON blobs. No ORM.
    Subclasses MUST implement the fromUri classmethod to construct from a URI.
    [👤semio📚engine💻engine🔖store🛠️databasestore](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/DatabaseStore)
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
                return KitOutput.model_validate(kit_data)
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
            dump = input.model_dump()
            dump["uri"] = kitUri
            with self._connect() as conn:
                cursor = conn.execute("SELECT 1 FROM kit WHERE uri = ?", (kitUri,))
                if cursor.fetchone() is not None:
                    raise KitAlreadyExists(kitUri)
                conn.execute(
                    "INSERT INTO kit (uri, data) VALUES (?, ?)",
                    (kitUri, json.dumps(dump)),
                )
            return KitOutput.model_validate(dump)

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
                    design_dump = input.model_dump()
                    designs = kit_data.get("designs", [])
                    designs = [d for d in designs if not (d.get("name") == input.name and d.get("variant") == input.variant and d.get("view") == input.view)]
                    designs.append(design_dump)
                    kit_data["designs"] = designs
                    conn.execute(
                        "UPDATE kit SET data = ? WHERE uri = ?",
                        (json.dumps(kit_data), kitUri),
                    )
                case OperationKind.TYPE:
                    type_dump = input.model_dump()
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
        """Apply a kit diff directly via SQL. Loads kit JSON, applies diff, stores back.
        Returns the updated kit dict.
        [👤semio📚engine💻engine🔖store🛠️applydiff](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/apply_diff)
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
    [👤semio📚engine💻engine🔖store🛠️sslmode](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/SSLMode)
    """

    DISABLE = "disable"
    ALLOW = "allow"
    PREFER = "prefer"
    REQUIRE = "require"
    VERIFY_CA = "verify-ca"
    VERIFY_FULL = "verify-full"


def cacheDir(remoteUri: str) -> str:
    """Returns the local cache directory path for a remote kit URI.
    Callers MUST provide a valid remote URI string.
    [👤semio📚engine💻engine🔖store🛠️cachedir](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/cacheDir)
    """
    cacheDir = os.path.expanduser("~/.semio/cache")
    encodedUri = encode(remoteUri)
    return os.path.join(cacheDir, encodedUri)


def cache(remoteUri: str) -> str:
    """📦Cache a remote kit and delete the existing cache if it was already cached.
    Callers MUST provide a URI starting with http and ending with .zip.
    [👤semio📚engine💻engine🔖store🛠️cache](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/cache)
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
    """SQLite-backed store that persists kit data as JSON in a local .semio database file.
    Callers MUST use fromUri to construct instances with a valid local path.
    [👤semio📚engine💻engine🔖store🛠️sqlitestore](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/SqliteStore)
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
    """PostgreSQL-backed store for remote database connections.
    Callers MUST NOT use this class until PostgreSQL support is implemented.
    [👤semio📚engine💻engine🔖store🛠️postgresstore](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/PostgresStore)
    """

    @classmethod
    def fromUri(cls, uri: str):
        raise FeatureNotYetSupported()

    def initialize(self) -> None:
        raise FeatureNotYetSupported()


# region Auth
# [👤semio📚engine💻engine🔖store🔖auth](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth)
# Auth MUST provide credential management for remote server authentication using Bearer tokens.

AUTH_FILE = os.path.join(os.path.expanduser(USER_FOLDER), "auth.json")


def _load_auth() -> dict:
    """Load auth credentials from the auth file.
    Returns dict mapping serverUrl -> {token, email}.
    [👤semio📚engine💻engine🔖store🔖auth🛠️loadauth](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/_load_auth)
    """
    if os.path.exists(AUTH_FILE):
        with open(AUTH_FILE, "r", encoding="utf-8") as f:
            return json.load(f)
    return {}


def _save_auth(auth: dict) -> None:
    """Save auth credentials to the auth file.
    Callers MUST provide a dict mapping serverUrl -> {token, email}.
    [👤semio📚engine💻engine🔖store🔖auth🛠️saveauth](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/_save_auth)
    """
    os.makedirs(os.path.dirname(AUTH_FILE), exist_ok=True)
    with open(AUTH_FILE, "w", encoding="utf-8") as f:
        json.dump(auth, f, indent=2)


def login(serverUrl: str, email: str, password: str) -> dict:
    """🔐 Login to a remote server and store the auth token.
    Callers MUST provide a valid server URL, email and password.
    Returns {ok, serverUrl, email, token} on success.
    [👤semio📚engine💻engine🔖store🔖auth🛠️login](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/login)
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
    [👤semio📚engine💻engine🔖store🔖auth🛠️logout](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/logout)
    """
    serverUrl = serverUrl.rstrip("/")
    auth = _load_auth()
    auth.pop(serverUrl, None)
    _save_auth(auth)
    return {"ok": True, "serverUrl": serverUrl}


def getAuthToken(serverUrl: str) -> str:
    """🔑 Get the stored auth token for a server.
    Raises AuthTokenNotFound if no token is stored.
    [👤semio📚engine💻engine🔖store🔖auth🛠️getauthtoken](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/getAuthToken)
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
    [👤semio📚engine💻engine🔖store🔖auth🛠️getauthstatus](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/getAuthStatus)
    """
    serverUrl = serverUrl.rstrip("/")
    auth = _load_auth()
    entry = auth.get(serverUrl)
    if entry and entry.get("token"):
        return {"authenticated": True, "serverUrl": serverUrl, "email": entry.get("email", "")}
    return {"authenticated": False, "serverUrl": serverUrl, "email": ""}


# endregion Auth


class RemoteStore(Store):
    """REST-backed store that proxies kit operations to a remote semio server.
    Callers MUST call login() first to authenticate with the remote server.
    [👤semio📚engine💻engine🔖store🛠️remotestore](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/RemoteStore)
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
        """Get authorization headers for remote requests."""
        token = getAuthToken(self.serverUrl)
        return {"Authorization": f"Bearer {token}", "Content-Type": "application/json"}

    def _api_url(self, path: str = "") -> str:
        """Build API URL for a kit operation."""
        base = f"{self.serverUrl}/api/kits/{encode(self.kitUri)}"
        if path:
            return f"{base}/{path}"
        return base

    def initialize(self) -> None:
        """Remote kits are initialized on the server side."""
        pass

    def get(self, operation: Operation) -> typing.Any:
        """🔍 Get an entity from the remote store."""
        kind = operation["kind"]
        try:
            if kind == OperationKind.KIT:
                response = requests.get(self._api_url(), headers=self._headers(), timeout=30)
                response.raise_for_status()
                return KitOutput.model_validate(response.json())
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
                    json=input.model_dump() if hasattr(input, "model_dump") else input,
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
                    json=input.model_dump() if hasattr(input, "model_dump") else input,
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
                    json=input.model_dump() if hasattr(input, "model_dump") else input,
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
    [👤semio📚engine💻engine🔖store🛠️storefactory](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/StoreFactory)
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
    """Parses a code string into a store instance and operation dict.
    Callers MUST provide a valid code string matching the code grammar.
    [👤semio📚engine💻engine🔖store🛠️storeandoperationfromcode](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/storeAndOperationFromCode)
    """
    codeTree = codeParser.parse(code)
    operation = OperationBuilder().transform(codeTree)
    store = StoreFactory(operation["kitUri"])
    return store, operation


def get(code: str, cache=False) -> typing.Any:
    """🔍 Get an entity from the store.
    Callers MUST provide a valid code string with an encoded kit URI.
    [👤semio📚engine💻engine🔖store🛠️get](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/get)
    """
    store, operation = storeAndOperationFromCode(code)
    return store.get(operation)


def put(code: str, input: str) -> typing.Any:
    """📥 Put an entity in the store.
    Callers MUST provide a valid code string and matching input data.
    [👤semio📚engine💻engine🔖store🛠️put](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/put)
    """
    store, operation = storeAndOperationFromCode(code)
    return store.put(operation, input)


def delete(code: str) -> typing.Any:
    """🗑 Delete an entity from the store.
    Callers MUST provide a valid code string referencing an existing entity.
    [👤semio📚engine💻engine🔖store🛠️delete](repo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/delete)
    """
    store, operation = storeAndOperationFromCode(code)
    return store.delete(operation)


# endregion Store

# region Assistant
# [👤semio📚engine💻engine🔖assistant](repo://p/u/semio/b/l/engine/f/engine.py/s/Assistant)
# Assistant MUST provide AI-powered design prediction using OpenAI structured outputs.


def encodeForPrompt(context: str):
    """Sanitizes a context string for use in AI prompts by replacing delimiters.
    Callers MUST pass a string that will be embedded in a prompt template.
    [👤semio📚engine💻engine🔖assistant🛠️encodeforprompt](repo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/encodeForPrompt)
    """
    return context.replace(";", ",").replace("\n", " ")


def replaceDefault(context: str, default: str):
    """Substitutes an empty context string with the provided default value.
    Callers MUST provide a non-None default string.
    [👤semio📚engine💻engine🔖assistant🛠️replacedefault](repo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/replaceDefault)
    """
    if context == "":
        return context.replace("", default)
    return context


def encodeType(type: TypeContext):
    """Encodes a TypeContext for prompt rendering by replacing empty values with defaults.
    Callers MUST provide a valid TypeContext with populated connectors.
    [👤semio📚engine💻engine🔖assistant🛠️encodetype](repo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/encodeType)
    """
    typeClone = type.model_copy(deep=True)
    typeClone.variant = replaceDefault(typeClone.variant, "DEFAULT")
    typeClone.description = encodeForPrompt(typeClone.description) if typeClone.description != "" else "NO_DESCRIPTION"
    for connector in typeClone.connectors:
        connector.id_ = replaceDefault(connector.id_, "DEFAULT")

    return typeClone


def decodeDesign(design: dict):
    """Decodes a raw AI response dict into a DesignPrediction model.
    Callers MUST provide a dict with pieces and connections arrays.
    [👤semio📚engine💻engine🔖assistant🛠️decodedesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/decodeDesign)
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
                "connected": {
                    "piece": {
                        "id_": (c["connectedPieceId"] if c["connectedPieceId"] != "DEFAULT" else ""),
                    },
                    "connector": {
                        "id_": (c["connectedPieceTypePortId"] if c["connectedPieceTypePortId"] != "DEFAULT" else ""),
                    },
                },
                "connecting": {
                    "piece": {
                        "id_": (c["connectingPieceId"] if c["connectingPieceId"] != "DEFAULT" else ""),
                    },
                    "connector": {
                        "id_": (c["connectingPieceTypePortId"] if c["connectingPieceTypePortId"] != "DEFAULT" else ""),
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
    [👤semio📚engine💻engine🔖assistant🛠️healdesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/healDesign)
    """
    designClone = design.model_copy(deep=True)
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
        if connection.connected.piece.id_ not in pieceD:
            try:
                connection.connected.piece.id_ = difflib.get_close_matches(connection.connected.piece.id_, pieceD.keys(), n=1)[0]
            except Error:
                continue
        if connection.connecting.piece.id_ not in pieceD:
            try:
                connection.connecting.piece.id_ = difflib.get_close_matches(connection.connecting.piece.id_, pieceD.keys(), n=1)[0]
            except Error:
                continue
        connectedType = typeD[pieceD[connection.connected.piece.id_].type.name][pieceD[connection.connected.piece.id_].type.variant]
        connectingType = typeD[pieceD[connection.connecting.piece.id_].type.name][pieceD[connection.connecting.piece.id_].type.variant]

        if connection.connected.connector.id_ not in connectorD[connectedType.name][connectedType.variant]:
            connection.connected.connector.id_ = difflib.get_close_matches(
                connection.connected.connector.id_,
                connectorD[connectedType.name][connectedType.variant].keys(),
                n=1,
            )[0]
        if connection.connecting.connector.id_ not in connectorD[connectingType.name][connectingType.variant]:
            connection.connecting.connector.id_ = difflib.get_close_matches(
                connection.connecting.connector.id_,
                connectorD[connectingType.name][connectingType.variant].keys(),
                n=1,
            )[0]
        validConnections.append(connection)
    designClone.connections = validConnections

    designClone.connections = [c for c in designClone.connections if c.connected.piece.id_ != c.connecting]

    designClone.pieces = [p for p in designClone.pieces if any(c for c in designClone.connections if c.connected.piece.id_ == p.id_ or c.connecting.piece.id_ == p.id_)]
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
Every connected and connecting piece MUST be part of the pieces of the design. The ids MUST match.
The connector of connected and connecting pieces MUST exist in the type of the piece. The ids MUST match.
The connector of connected and connecting pieces SHOULD match.
If the connectors of connected and connecting pieces have a port, they should be compatible.
If one connector has the other connector as ocompatible that's enough.
Every piece in the design MUST be connected to at least one other piece.
One piece is the root piece of the design. The connections MUST form a tree.
Ids SHOULD be abreviated and don't have to be globally unique.
Rotation, tilt, gap, shift SHOULD NOT be added unless specifically instructed.
The diagram is only a nice 2D model of the design and does not change the design.
When a piece is [on, next to, above, below, ...] another piece, there SHOULD be a connected between the pieces.
When a piece fits to a connector of another piece, there SHOULD be a connecting between the pieces."""

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
                    "description": "A bidirectional connection between two pieces of a design.",
                    "properties": {
                        "connectedPieceId": {
                            "type": "string"
                        },
                        "connectedPieceTypePortId": {
                            "type": "string"
                        },
                        "connectingPieceId": {
                            "type": "string"
                        },
                        "connectingPieceTypePortId": {
                            "type": "string"
                        },
                        "gap": {
                            "type": "number",
                            "description": "The optional longitudinal gap (applied after rotation and tilt in connector direction) between the connected and the connecting piece. "
                        },
                        "shift": {
                            "type": "number",
                            "description": "The optional lateral shift (applied after the rotation, the turn and the tilt in the plane) between the connected and the connecting piece.."
                        },
                        "rise": {
                            "type": "number",
                            "description": "The optional vertical rise in connector direction between the connected and the connecting piece. Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."
                        },
                        "rotation": {
                            "type": "number",
                            "description": "The optional horizontal rotation in connector direction between the connected and the connecting piece in degrees."
                        },
                        "turn": {
                            "type": "number",
                            "description": "The optional turn perpendicular to the connector direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.  Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."
                        },
                        "tilt": {
                            "type": "number",
                            "description": "The optional horizontal tilt perpendicular to the connector direction (applied after rotation and the turn) between the connected and the connecting piece in degrees."
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
                        "connectedPieceId",
                        "connectedPieceTypePortId",
                        "connectingPieceId",
                        "connectingPieceTypePortId",
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
    [👤semio📚engine💻engine🔖assistant🛠️predictdesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/predictDesign)
    """
    if openaiClient is None:
        raise FeatureNotYetSupported("OpenAI client not available")

    prompt = designGenerationPromptTemplate.render(description=description, types=[encodeType(t) for t in types])
    logger.debug("Generated prompt: {}", prompt)
    try:
        response = openaiClient.chat.completions.create(
            model="gpt-4o",
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
                "model": response.model,
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

        if hasattr(design, "model_dump"):
            logger.debug("Predicted Design: {}", json.dumps(design.model_dump(), indent=4))

        healedDesign = healDesign(typing.cast(DesignPrediction, design), types)
        logger.debug(
            "Predicted Design Healed: {}",
            json.dumps(healedDesign.model_dump(), indent=4),
        )
        return healedDesign

    raise FeatureNotYetSupported("OpenAI response was invalid or incomplete")


# endregion Assistant

# region Graphql
# [👤semio📚engine💻engine🔖graphql](repo://p/u/semio/b/l/engine/f/engine.py/s/Graphql)
# Graphql MUST map semio domain types to Graphene schema nodes for query and mutation.

GRAPHQLTYPES = {
    "str": graphene.NonNull(graphene.String),
    "int": graphene.NonNull(graphene.Int),
    "float": graphene.NonNull(graphene.Float),
    "bool": graphene.NonNull(graphene.Boolean),
    "list[str]": graphene.NonNull(graphene.List(graphene.NonNull(graphene.String))),
    "Attribute": graphene.NonNull(lambda: AttributeNode),
    "list[Attribute]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AttributeNode))),
    "list[__main__.Attribute]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AttributeNode))),
    "list[__mp_main__.Attribute]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AttributeNode))),
    "list[main.Attribute]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AttributeNode))),
    "Coord": graphene.NonNull(lambda: CoordNode),
    "typing.Optional[__main__.Coord]": lambda: CoordNode,
    "typing.Optional[__mp_main__.Coord]": lambda: CoordNode,
    "typing.Optional[main.Coord]": lambda: CoordNode,
    "Location": graphene.NonNull(lambda: LocationNode),
    "typing.Optional[__main__.Location]": lambda: LocationNode,
    "typing.Optional[__mp_main__.Location]": lambda: LocationNode,
    "typing.Optional[main.Location]": lambda: LocationNode,
    "Point": graphene.NonNull(lambda: PointNode),
    "Vector": graphene.NonNull(lambda: VectorNode),
    "Plane": graphene.NonNull(lambda: PlaneNode),
    "Connector": graphene.NonNull(lambda: ConnectorNode),
    "ConnectorId": graphene.NonNull(lambda: ConnectorNode),
    "list[Connector]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectorNode))),
    "list[__main__.Connector]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectorNode))),
    "list[__mp_main__.Connector]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectorNode))),
    "list[main.Connector]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectorNode))),
    "Model": graphene.NonNull(lambda: ModelNode),
    "list[Model]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ModelNode))),
    "list[__main__.Model]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ModelNode))),
    "list[__mp_main__.Model]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ModelNode))),
    "list[main.Model]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ModelNode))),
    "Author": graphene.NonNull(lambda: AuthorNode),
    "list[Author]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AuthorNode))),
    "list[__main__.Author]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AuthorNode))),
    "list[__mp_main__.Author]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AuthorNode))),
    "list[main.Author]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AuthorNode))),
    "Type": graphene.NonNull(lambda: TypeNode),
    "TypeId": graphene.NonNull(lambda: TypeNode),
    "DesignId": graphene.NonNull(lambda: DesignNode),
    "list[Type]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    "list[__main__.Type]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    "list[__mp_main__.Type]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    "list[main.Type]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    "Piece": graphene.NonNull(lambda: PieceNode),
    "PieceId": graphene.NonNull(lambda: PieceNode),
    "typing.Optional[__main__.PieceId]": lambda: PieceNode,
    "typing.Optional[__mp_main__.PieceId]": lambda: PieceNode,
    "typing.Optional[main.PieceId]": lambda: PieceNode,
    "typing.Optional[__main__.DesignId]": lambda: DesignNode,
    "typing.Optional[__mp_main__.DesignId]": lambda: DesignNode,
    "typing.Optional[main.DesignId]": lambda: DesignNode,
    "Side": graphene.NonNull(lambda: SideNode),
    "Connection": graphene.NonNull(lambda: ConnectionNode),
    "list['Connection']": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectionNode))),
    "list[__main__.Connection]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectionNode))),
    "list[__mp_main__.Connection]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectionNode))),
    "list[main.Connection]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectionNode))),
    "Design": graphene.NonNull(lambda: DesignNode),
    "Kit": graphene.NonNull(lambda: KitNode),
}


class Query(graphene.ObjectType):
    """GraphQL root query type exposing kit retrieval by URI.
    Callers MUST provide a valid URI when resolving kit queries.
    [👤semio📚engine💻engine🔖graphql🛠️query](repo://p/u/semio/b/l/engine/f/engine.py/s/Graphql/d/i/Query)
    """

    node = RelayNode.Field()
    kit = graphene.Field(KitNode, uri=graphene.String(required=True))

    def resolve_kit(self, info, uri):
        return get(encode(uri))


class Mutation(graphene.ObjectType):
    """GraphQL root mutation type exposing kit creation.
    Callers MUST provide a valid KitInput when creating kits.
    [👤semio📚engine💻engine🔖graphql🛠️mutation](repo://p/u/semio/b/l/engine/f/engine.py/s/Graphql/d/i/Mutation)
    """

    createKit = graphene.Field(KitNode, kit=KitInputNode(required=True))


graphqlSchema = graphene.Schema(
    query=Query,
    mutation=Mutation,
)

# endregion Graphql

# region Rest
# [👤semio📚engine💻engine🔖rest](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest)
# Rest MUST expose kit, type, design, and assistant endpoints via FastAPI.

rest = fastapi.FastAPI(max_request_body_size=MAX_REQUEST_BODY_SIZE)

rest.add_middleware(
    starlette.middleware.cors.CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


class NativeAlgorithmExecuteBody(pydantic.BaseModel):
    """Request body for POST /api/native-algorithms/execute.
    [👤semio📚engine💻engine🔖rest🛠️nativealgorithmexecutebody](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/NativeAlgorithmExecuteBody)
    """

    language: typing.Literal["python", "go", "rust"]
    operation: typing.Literal["flatten", "delete"]
    kit: dict[str, typing.Any]
    design: dict[str, typing.Any]
    designGuid: str
    pieceGuids: list[str] = []
    connectionGuids: list[str] = []


def _semio_repo_root() -> pathlib.Path:
    """Return the semio/ directory containing go, rs, py bundles.
    [👤semio📚engine💻engine🔖rest🛠️semioreporoot](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/_semio_repo_root)
    """
    return pathlib.Path(__file__).resolve().parent.parent


def _python_native_flatten_to_design_change(kit: dict[str, typing.Any], design_guid: str) -> dict[str, typing.Any]:
    """Map flattenDesignDict output to a DesignChange-shaped dict for the algorithms adapter.
    [👤semio📚engine💻engine🔖rest🛠️pythonnativeflattentodesignchange](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/_python_native_flatten_to_design_change)
    """
    raw = flattenDesignDict(kit, design_guid)
    updates: list[dict[str, typing.Any]] = []
    for u in raw.get("pieces", {}).get("updated", []):
        pid = u.get("id")
        diff = u.get("diff", {})
        if pid:
            updates.append({"piece": {"guid": pid}, "diff": diff})
    forward: dict[str, typing.Any] = {}
    if updates:
        forward["pieces"] = {"updated": updates}
    return {"forward": forward, "backward": {}}


def _go_native_bridge(payload: dict[str, typing.Any], operation: str) -> typing.Any:
    """Run semio/go/cmd/nativebridge and return parsed JSON result.
    [👤semio📚engine💻engine🔖rest🛠️gonativebridge](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/_go_native_bridge)
    """
    go_root = _semio_repo_root() / "go"
    proc = subprocess.run(
        ["go", "run", "./cmd/nativebridge"],
        cwd=str(go_root),
        input=json.dumps(payload).encode("utf-8"),
        capture_output=True,
        timeout=300,
        check=False,
    )
    if proc.returncode != 0:
        raise RuntimeError(proc.stderr.decode("utf-8", errors="replace") or "go native bridge failed")
    out = json.loads(proc.stdout.decode("utf-8"))
    if not out.get("ok"):
        raise RuntimeError(out.get("error", "go native bridge error"))
    result = out.get("result")
    if operation == "flatten":
        return {"forward": result, "backward": {}}
    return result


def _rust_native_bridge(payload: dict[str, typing.Any], operation: str) -> typing.Any:
    """Run semio/rs native_bridge and return parsed JSON result.
    [👤semio📚engine💻engine🔖rest🛠️rustnativebridge](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/_rust_native_bridge)
    """
    rs_root = _semio_repo_root() / "rs"
    proc = subprocess.run(
        ["cargo", "run", "-q", "--bin", "native_bridge"],
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


def _dispatch_native_algorithm(body: NativeAlgorithmExecuteBody) -> typing.Any:
    """Dispatch native algorithm execution by language and operation.
    [👤semio📚engine💻engine🔖rest🛠️dispatchnativealgorithm](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/_dispatch_native_algorithm)
    """
    kit = body.kit
    design = body.design
    dg = body.designGuid
    lang = body.language
    op = body.operation
    bridge_payload: dict[str, typing.Any] = {
        "op": op,
        "kit": kit,
        "design": design,
        "designGuid": dg,
        "pieceGuids": list(body.pieceGuids),
        "connectionGuids": list(body.connectionGuids),
    }
    if lang == "python":
        if op == "flatten":
            return _python_native_flatten_to_design_change(kit, dg)
        return deletePiecesAndConnectionsInDesignDict(design, list(body.pieceGuids), list(body.connectionGuids))
    if lang == "go":
        return _go_native_bridge(bridge_payload, op)
    if lang == "rust":
        return _rust_native_bridge(bridge_payload, op)
    raise RuntimeError(f"unsupported native language: {lang}")


@rest.post("/native-algorithms/execute")
async def native_algorithms_execute(body: NativeAlgorithmExecuteBody) -> fastapi.responses.JSONResponse:
    """Execute flatten or delete in python, go, or rust native stacks (typescript runs in the browser).
    [👤semio📚engine💻engine🔖rest🛠️nativealgorithmsexecute](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/native_algorithms_execute)
    """
    try:
        result = _dispatch_native_algorithm(body)
        return fastapi.responses.JSONResponse(content={"result": result})
    except Exception as e:
        return fastapi.responses.JSONResponse(content={"error": str(e)}, status_code=500)


def _build_design_viewer_html() -> str:
    """Build the embeddable design viewer HTML from the built MCP App bundle.
    Callers MUST use the returned HTML for the /app/design-viewer endpoint.
    The MCP App is built from mcp-app.tsx which uses @semio/ui components exclusively.
    [👤semio📚engine💻engine🔖rest🛠️builddesignviewerhtml](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/_build_design_viewer_html)
    """
    app_html_path = os.path.join(os.path.dirname(__file__), "dist", "mcp-app.html")
    if os.path.exists(app_html_path):
        with open(app_html_path, "r", encoding="utf-8") as f:
            return f.read()
    return """<!doctype html><html><body><p>MCP App not built. Run: npm run build:mcp-app in semio/engine</p></body></html>"""


def _build_kit_viewer_html() -> str:
    """Build the embeddable kit viewer HTML (SemioKit-only MCP shell) from the same bundle as the design viewer.
    Sets #root data-mcp-viewer to kit so mcp-app.tsx mounts McpKitViewer from @semio/ui.
    Callers MUST use the returned HTML for the /app/kit-viewer endpoint and ui://semio/kit-viewer resource.
    [👤semio📚engine💻engine🔖rest🛠️buildkitviewerhtml](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/_build_kit_viewer_html)
    """
    html = _build_design_viewer_html()
    if "MCP App not built" in html:
        return html.replace("MCP App not built", "MCP kit app not built", 1)
    return html.replace('data-mcp-viewer="design"', 'data-mcp-viewer="kit"', 1).replace("<title>semio design viewer</title>", "<title>semio kit viewer</title>", 1)


@rest.get("/app/design-viewer")
async def app_design_viewer() -> fastapi.Response:
    """Return the embeddable design viewer HTML shell.
    Callers MUST use this endpoint to embed the semio design viewer in an iframe.
    [👤semio📚engine💻engine🔖rest🛠️appdesignviewer](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/app_design_viewer)
    """
    return fastapi.Response(
        content=_build_design_viewer_html(),
        media_type="text/html",
        headers={
            "Content-Security-Policy": "default-src 'self' 'unsafe-inline'; frame-ancestors *; connect-src * data: blob:; img-src * data: blob:;",
        },
    )


@rest.get("/app/kit-viewer")
async def app_kit_viewer() -> fastapi.Response:
    """Return the embeddable kit viewer HTML shell (SemioKit from @semio/ui).
    [👤semio📚engine💻engine🔖rest🛠️appkitviewer](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/app_kit_viewer)
    """
    return fastapi.Response(
        content=_build_kit_viewer_html(),
        media_type="text/html",
        headers={
            "Content-Security-Policy": "default-src 'self' 'unsafe-inline'; frame-ancestors *; connect-src * data: blob:; img-src * data: blob:;",
        },
    )


@rest.get("/app/payload/{token}")
async def app_payload(token: str) -> fastapi.responses.JSONResponse:
    """Return the full MCP app payload by token. Used by MCP app iframes to bypass host truncation."""
    payload = _mcp_app_payloads.get(token)
    if payload is None:
        return fastapi.responses.JSONResponse({"error": "Payload not found or expired"}, status_code=404)
    return fastapi.responses.JSONResponse(payload, headers={"Access-Control-Allow-Origin": "*"})


@rest.get("/app/files/{file_guid}")
async def app_file(file_guid: str) -> fastapi.Response:
    """Serve a kit file blob by guid. Used by MCP app iframes to load 3D models."""
    blob = _mcp_app_file_blobs.get(file_guid)
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
    [👤semio📚engine💻engine🔖rest🛠️kit](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/kit)
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
    [👤semio📚engine💻engine🔖rest🛠️createkit](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/create_kit)
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
    [👤semio📚engine💻engine🔖rest🛠️deletekit](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/delete_kit)
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
    [👤semio📚engine💻engine🔖rest🛠️puttype](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/put_type)
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
    [👤semio📚engine💻engine🔖rest🛠️deletetype](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/delete_type)
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
    [👤semio📚engine💻engine🔖rest🛠️putdesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/put_design)
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
    [👤semio📚engine💻engine🔖rest🛠️deletedesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/delete_design)
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
    [👤semio📚engine💻engine🔖rest🛠️predictdesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/predict_design)
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
    [👤semio📚engine💻engine🔖rest🛠️preparekit](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/prepare_kit)
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


class ContextGenerateJsonSchema(pydantic.json_schema.GenerateJsonSchema):
    """JSON schema generator that strips Context suffixes from type references.
    Callers MUST use this generator when exporting context model schemas.
    [👤semio📚engine💻engine🔖rest🛠️contextgeneratejsonschema](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/ContextGenerateJsonSchema)
    """

    def generate(self, schema, mode="validation"):
        json_schema = super().generate(schema, mode=mode)
        changeValues(json_schema, "$ref", lambda x: x.removesuffix("Context"))
        changeValues(json_schema, "title", lambda x: x.removesuffix("Context"))
        changeKeys(json_schema, lambda x: x.removesuffix("Context"))
        return json_schema


class OutputGenerateJsonSchema(pydantic.json_schema.GenerateJsonSchema):
    """JSON schema generator that strips Output suffixes from type references.
    Callers MUST use this generator when exporting output model schemas.
    [👤semio📚engine💻engine🔖rest🛠️outputgeneratejsonschema](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/OutputGenerateJsonSchema)
    """

    def generate(self, schema, mode="validation"):
        json_schema = super().generate(schema, mode=mode)
        changeValues(json_schema, "$ref", lambda x: x.removesuffix("Output"))
        changeValues(json_schema, "title", lambda x: x.removesuffix("Output"))
        changeKeys(json_schema, lambda x: x.removesuffix("Output"))
        return json_schema


class PredictionGenerateJsonSchema(pydantic.json_schema.GenerateJsonSchema):
    """JSON schema generator that strips Prediction suffixes from type references.
    Callers MUST use this generator when exporting prediction model schemas.
    [👤semio📚engine💻engine🔖rest🛠️predictiongeneratejsonschema](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/PredictionGenerateJsonSchema)
    """

    def generate(self, schema, mode="validation"):
        json_schema = super().generate(schema, mode=mode)
        changeValues(json_schema, "$ref", lambda x: x.removesuffix("Prediction"))
        changeValues(json_schema, "title", lambda x: x.removesuffix("Prediction"))
        changeKeys(json_schema, lambda x: x.removesuffix("Prediction"))
        return json_schema


def custom_openapi():
    """Generates a custom OpenAPI schema with /api path prefix and cleaned type names.
    Callers MUST NOT call this directly; it is assigned to rest.openapi.
    [👤semio📚engine💻engine🔖rest🛠️customopenapi](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/custom_openapi)
    """
    if rest.openapi_schema:
        return rest.openapi_schema
    openapi_schema = fastapi.openapi.utils.get_openapi(
        title="semio REST API",
        version=VERSION,
        summary="This is the local rest API of the semio engine.",
        routes=rest.routes,
    )

    updated_paths = {}
    for path, path_item in openapi_schema["paths"].items():
        updated_paths[f"/api{path}"] = path_item
    openapi_schema["paths"] = updated_paths

    changeValues(openapi_schema, "$ref", lambda x: x.removesuffix("Output"))
    changeValues(openapi_schema, "title", lambda x: x.removesuffix("Output"))
    changeKeys(openapi_schema, lambda x: x.removesuffix("Output"))
    rest.openapi_schema = openapi_schema
    return rest.openapi_schema


rest.openapi = custom_openapi


# region Auth Endpoints
# [👤semio📚engine💻engine🔖rest🔖authendpoints](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints)
# Auth endpoints MUST expose login, logout and status for remote server authentication.


class LoginRequest(pydantic.BaseModel):
    """Login request body.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️loginrequest](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/LoginRequest)
    """

    serverUrl: str
    email: str
    password: str


class LoginResponse(pydantic.BaseModel):
    """Login response body.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️loginresponse](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/LoginResponse)
    """

    ok: bool
    serverUrl: str
    email: str
    token: str


class LogoutRequest(pydantic.BaseModel):
    """Logout request body.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️logoutrequest](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/LogoutRequest)
    """

    serverUrl: str


class AuthStatusResponse(pydantic.BaseModel):
    """Auth status response body.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️authstatusresponse](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/AuthStatusResponse)
    """

    authenticated: bool
    serverUrl: str
    email: str


@rest.post("/auth/login")
async def rest_login(request: LoginRequest) -> LoginResponse:
    """Login to a remote server and store the auth token.
    Callers MUST provide serverUrl, email and password.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️restlogin](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/rest_login)
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
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️restlogout](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/rest_logout)
    """
    try:
        return logout(request.serverUrl)
    except Exception as e:
        return fastapi.Response(content=str(e), status_code=500)


@rest.get("/auth/status")
async def rest_auth_status(serverUrl: str) -> AuthStatusResponse:
    """Get the auth status for a remote server.
    Callers MUST provide serverUrl as a query parameter.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️restauthstatus](repo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/rest_auth_status)
    """
    try:
        result = getAuthStatus(serverUrl)
        return AuthStatusResponse(**result)
    except Exception as e:
        return fastapi.Response(content=str(e), status_code=500)


# endregion Auth Endpoints

# endregion Rest

# region Mcp
# [👤semio📚engine💻engine🔖mcp](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp)
# Mcp MUST expose stateful kit operations via Model Context Protocol.
# Call start_working_in_local_kit(path) first; then use start_working_in_design/start_working_in_type to scope further.

mcp = FastMCP("semio", stateless_http=False, json_response=True)

_APP_RESOURCE_URI = "ui://semio/design-viewer"
_APP_RESOURCE_META = {"ui": {"resourceUri": _APP_RESOURCE_URI}, "ui/resourceUri": _APP_RESOURCE_URI}
_KIT_APP_RESOURCE_URI = "ui://semio/kit-viewer"
_KIT_APP_RESOURCE_META = {"ui": {"resourceUri": _KIT_APP_RESOURCE_URI}, "ui/resourceUri": _KIT_APP_RESOURCE_URI}


def _mcp_app_html_resource_meta() -> dict[str, typing.Any]:
    """Resource _meta for MCP App HTML: hosts apply _meta.ui.csp to the sandbox (see .repo/✍️/mcp-app.md)."""
    origins = [
        f"http://127.0.0.1:{PORT}",
        f"http://localhost:{PORT}",
        f"http://[::1]:{PORT}",
        f"ws://127.0.0.1:{PORT}",
        f"ws://localhost:{PORT}",
    ]
    csp = {"connectDomains": origins, "resourceDomains": origins}
    return {"ui": {"csp": csp}, "ui/csp": csp}


# Session-scoped state. Keyed by session id for isolation.
_mcp_session_kits: dict[int, dict[str, typing.Any]] = {}
_mcp_session_designs: dict[int, dict[str, typing.Any]] = {}
_mcp_session_types: dict[int, dict[str, typing.Any]] = {}
_mcp_session_kit_mode: dict[int, str] = {}
_mcp_session_kit_source: dict[int, str] = {}
_mcp_session_transactions: dict[int, Transaction] = {}
_mcp_session_transaction_rollback: set[int] = set()
_mcp_session_selection: dict[int, dict[str, list[str]]] = {}
_mcp_session_camera: dict[int, dict[str, typing.Any]] = {}
_mcp_app_payloads: dict[str, dict[str, typing.Any]] = {}
_mcp_app_file_blobs: dict[str, str] = {}


def _load_kit_from_remote(serverUrl: str, kitUri: str) -> dict:
    """Load kit dict from a remote server via REST API.
    Callers MUST have called login() first to authenticate with the server.
    [👤semio📚engine💻engine🔖mcp🛠️loadkitfromremote](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/_load_kit_from_remote)
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


def _load_kit_from_path(path: str) -> dict:
    """Load kit dict from path (JSON file or folder with .semio/kit.db or kit JSON).
    [👤semio📚engine💻engine🔖mcp🛠️loadkitfrompath](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/_load_kit_from_path)
    """
    p = pathlib.Path(path).resolve()
    if p.is_file() and p.suffix == ".json":
        with open(p, "r", encoding="utf-8") as f:
            return json.load(f)
    if p.is_dir():
        sqlite_path = p / KIT_LOCAL_FOLDERNAME / KIT_LOCAL_FILENAME
        if sqlite_path.exists():
            kit, _files = _semio_core.import_folder_kit(str(p))
            if hasattr(kit, "model_dump"):
                return kit.model_dump()
            if hasattr(kit, "to_dict"):
                return kit.to_dict()
            return KitOutput.model_validate(kit).model_dump()
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


def _session_id(ctx) -> int:
    """Get session id from context."""
    return id(ctx.session) if ctx and hasattr(ctx, "session") else None


def _get_session_kit(ctx) -> dict[str, typing.Any]:
    """Get kit from session. Raises if start_working_in_local_kit or start_working_in_remote_kit was not called."""
    sid = _session_id(ctx)
    if sid is None or sid not in _mcp_session_kits:
        raise ValueError("Call start_working_in_local_kit(path) or start_working_in_remote_kit(serverUrl, kitUri) first to set the kit for this session.")
    return _mcp_session_kits[sid]


def _get_session_kit_mode(ctx) -> str:
    """Get kit mode from session. Returns 'local' or 'remote'."""
    sid = _session_id(ctx)
    return _mcp_session_kit_mode.get(sid, "local")


def _hydrate_design_from_kit_disk_if_shallow(design: dict[str, typing.Any], kit_source: str | None, design_guid: str) -> dict[str, typing.Any]:
    """If the kit only lists design metadata (no pieces), load a sibling `*.design.semio.json` with the same guid.
    [👤semio📚engine💻engine🔖mcp🛠️hydratedesignfromkitdiskifshallow](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/_hydrate_design_from_kit_disk_if_shallow)
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
                if not isinstance(data, dict) or data.get("guid") != design_guid:
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
    """Get current design from session. Raises if start_working_in_design was not called."""
    sid = _session_id(ctx)
    if sid is None or sid not in _mcp_session_designs:
        raise ValueError("Call start_working_in_design(guid) first to set the design for this session.")
    design = _mcp_session_designs[sid]
    guid = design.get("guid")
    if not isinstance(guid, str) or not guid:
        return design
    kit_src = _mcp_session_kit_source.get(sid)
    merged = _hydrate_design_from_kit_disk_if_shallow(design, kit_src, guid)
    if merged is not design:
        _mcp_session_designs[sid] = merged
    return _mcp_session_designs[sid]


def _get_session_type(ctx) -> dict[str, typing.Any]:
    """Get current type from session. Raises if start_working_in_type was not called."""
    sid = _session_id(ctx)
    if sid is None or sid not in _mcp_session_types:
        raise ValueError("Call start_working_in_type(guid) first to set the type for this session.")
    return _mcp_session_types[sid]


def _clone_kit(kit: dict | None) -> dict | None:
    """Create a deep copy of a kit dict for safe transaction snapshots."""
    if kit is None:
        return None
    return copy.deepcopy(kit)


def _sync_session_design_and_type(sid: int | None):
    """Realign current design and type selections with the current session kit after mutations or rollbacks."""
    if sid is None:
        return
    kit = _mcp_session_kits.get(sid)
    current_design = _mcp_session_designs.get(sid)
    if kit is None:
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        return
    if current_design is not None:
        design_guid = current_design.get("guid")
        synced_design = next((d for d in kit.get("designs", []) if d.get("guid") == design_guid), None)
        if synced_design is None:
            _mcp_session_designs.pop(sid, None)
        else:
            cur_n = len((current_design.get("pieces") or []))
            sync_n = len((synced_design.get("pieces") or []))
            if sync_n >= cur_n:
                _mcp_session_designs[sid] = synced_design
    current_type = _mcp_session_types.get(sid)
    if current_type is not None:
        type_guid = current_type.get("guid")
        synced_type = next((t for t in kit.get("types", []) if t.get("guid") == type_guid), None)
        if synced_type is None:
            _mcp_session_types.pop(sid, None)
        else:
            _mcp_session_types[sid] = synced_type


def _get_active_transaction(sid: int | None) -> Transaction | None:
    """Return the active transaction for a session, if any."""
    if sid is None:
        return None
    transaction = _mcp_session_transactions.get(sid)
    if transaction is None or not transaction.get("active"):
        return None
    return transaction


def _record_transaction_kit_change(sid: int | None, before_kit: dict | None, after_kit: dict | None):
    """Record a kit change in the active transaction using forward/backward diffs."""
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


def _set_session_kit(ctx, kit: dict):
    """Set session kit and record the change if a transaction is active."""
    sid = _session_id(ctx)
    before = _mcp_session_kits.get(sid)
    _record_transaction_kit_change(sid, before, kit)
    _mcp_session_kits[sid] = kit
    _sync_session_design_and_type(sid)


def _clear_session_kit(ctx):
    """Clear session kit and record the change if a transaction is active."""
    sid = _session_id(ctx)
    before = _mcp_session_kits.get(sid)
    _record_transaction_kit_change(sid, before, None)
    _mcp_session_kits.pop(sid, None)
    _sync_session_design_and_type(sid)


def _replace_design_in_session_kit(ctx: Context, design: dict) -> dict:
    """Replace or append a design in the current session kit and keep the current design selection synced."""
    sid = _session_id(ctx)
    kit = _clone_kit(_get_session_kit(ctx))
    designs = list(kit.get("designs", []))
    replaced = False
    for index, existing_design in enumerate(designs):
        if existing_design.get("guid") == design.get("guid"):
            designs[index] = design
            replaced = True
            break
    if not replaced:
        designs.append(design)
    kit["designs"] = designs
    _set_session_kit(ctx, kit)
    synced_design = next((item for item in _mcp_session_kits[sid].get("designs", []) if item.get("guid") == design.get("guid")), None)
    if synced_design is not None:
        _mcp_session_designs[sid] = synced_design
        return synced_design
    raise ValueError(f"Design with guid {design.get('guid')} could not be stored in the current kit.")


def _mutate_current_design(ctx: Context, mutator: typing.Callable[[dict], None]) -> dict:
    """Clone, mutate, and persist the current design in the current session kit."""
    design = copy.deepcopy(_get_session_design(ctx))
    mutator(design)
    return _replace_design_in_session_kit(ctx, design)


def _rollback_session_transaction(sid: int):
    """Rollback all transaction changes in reverse order."""
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
            _mcp_session_kits[sid] = applyKitDiffDict(current, backward_diff)
    _sync_session_design_and_type(sid)


@mcp.tool(meta=_KIT_APP_RESOURCE_META)
def start_working_in_local_kit(path: str, ctx: Context) -> CallToolResult:
    """Load a local kit into the session. Must be called before any kit operations.

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
    """Create a new in-memory kit for the session with the given name and version."""
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
    """Load a remote kit into the session. Requires a prior login call. Must be called before any kit operations."""
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


# region MCP Auth Tools
# [👤semio📚engine💻engine🔖mcp🔖mcpauthtools](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Auth%20Tools)
# MCP Auth Tools MUST expose login, logout and status for remote server authentication.


def mcp_login(serverUrl: str, email: str, password: str) -> dict:
    """Login to a remote semio server and store the auth token for subsequent remote kit operations."""
    try:
        return login(serverUrl, email, password)
    except Exception as e:
        return {"error": str(e)}


def mcp_logout(serverUrl: str) -> dict:
    """Logout from a remote semio server and remove the stored token."""
    try:
        return logout(serverUrl)
    except Exception as e:
        return {"error": str(e)}


def mcp_auth_status(serverUrl: str) -> dict:
    """Get the authentication status for a remote semio server."""
    try:
        return getAuthStatus(serverUrl)
    except Exception as e:
        return {"error": str(e)}


# endregion MCP Auth Tools


def validate_kit(kit: dict) -> dict:
    """Validate a kit dict and return any validation problems."""
    try:
        result = validateKitDict(kit)
        return result.model_dump() if hasattr(result, "model_dump") else {"problems": []}
    except Exception as e:
        return {"error": str(e)}


def flatten_design(kit: dict, design_guid: str) -> dict:
    """Flatten a design by computing absolute planes for all pieces."""
    try:
        return flattenDesignDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_kit_diff(before: dict, after: dict) -> dict:
    """Compute the diff between two kit states."""
    try:
        return getKitDiffDict(before, after)
    except Exception as e:
        return {"error": str(e)}


def apply_kit_diff(base: dict, diff: dict) -> dict:
    """Apply a diff to a kit dict."""
    try:
        return applyKitDiffDict(base, diff)
    except Exception as e:
        return {"error": str(e)}


def inverse_kit_diff(original: dict, applied_diff: dict) -> dict:
    """Compute the inverse of a diff for undo operations."""
    try:
        return inverseKitDiffDict(original, applied_diff)
    except Exception as e:
        return {"error": str(e)}


def get_kit_change(before: dict, after: dict) -> dict:
    """Compute forward and backward diffs between two kit states for undo/redo."""
    try:
        return changeToDict(getKitChange(before, after))
    except Exception as e:
        return {"error": str(e)}


def get_design_change(before: dict, after: dict) -> dict:
    """Compute forward and backward diffs between two design states for undo/redo."""
    try:
        return changeToDict(getDesignChange(before, after))
    except Exception as e:
        return {"error": str(e)}


def pieces_metadata(kit: dict, design_guid: str) -> dict:
    """Get metadata for all pieces in a design including plane, center, fixedPieceId, parentPieceId, depth, and path."""
    try:
        return piecesMetadataDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_primitive_design(kit: dict, design_guid: str) -> dict:
    """Get the root design of a design family."""
    try:
        return getPrimitiveDesignDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_design_family(kit: dict, design_guid: str) -> list:
    """Get all designs in a design family tree."""
    try:
        return getDesignFamilyDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_design_siblings(kit: dict, design_guid: str) -> list:
    """Get all sibling designs sharing the same parent, excluding the given design."""
    try:
        return getDesignSiblingsDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_design_children(kit: dict, design_guid: str) -> list:
    """Get all direct child designs of a design."""
    try:
        return getDesignChildrenDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def are_designs_in_same_family(kit: dict, design_guid_a: str, design_guid_b: str) -> dict:
    """Check if two designs belong to the same family."""
    try:
        return {"result": areDesignsInSameFamilyDict(kit, design_guid_a, design_guid_b)}
    except Exception as e:
        return {"error": str(e)}


def can_use_design_as_piece(kit: dict, container_design_guid: str, piece_design_guid: str) -> dict:
    """Check if a design can be used as a piece in another design without creating circular references."""
    try:
        return {"result": canUseDesignAsPieceDict(kit, container_design_guid, piece_design_guid)}
    except Exception as e:
        return {"error": str(e)}


def find_same_family_design_pieces(kit: dict, design_guid: str) -> list:
    """Find pieces in a design that reference designs from the same family."""
    try:
        return findSameFamilyDesignPiecesDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_primitive_type(kit: dict, type_guid: str) -> dict:
    """Get the root type of a type family."""
    try:
        return getPrimitiveTypeDict(kit, type_guid)
    except Exception as e:
        return {"error": str(e)}


def get_type_family(kit: dict, type_guid: str) -> list:
    """Get all types in a type family tree."""
    try:
        return getTypeFamilyDict(kit, type_guid)
    except Exception as e:
        return {"error": str(e)}


def get_type_siblings(kit: dict, type_guid: str) -> list:
    """Get all sibling types sharing the same parent, excluding the given type."""
    try:
        return getTypeSiblingsDict(kit, type_guid)
    except Exception as e:
        return {"error": str(e)}


def get_type_children(kit: dict, type_guid: str) -> list:
    """Get all direct child types of a type."""
    try:
        return getTypeChildrenDict(kit, type_guid)
    except Exception as e:
        return {"error": str(e)}


def are_types_in_same_family(kit: dict, type_guid_a: str, type_guid_b: str) -> dict:
    """Check if two types belong to the same family."""
    try:
        return {"result": areTypesInSameFamilyDict(kit, type_guid_a, type_guid_b)}
    except Exception as e:
        return {"error": str(e)}


def find_piece_type_in_design(kit: dict, design_guid: str, piece_guid: str) -> dict:
    """Get the type of a specific piece in a design."""
    try:
        return findPieceTypeInDesignDict(kit, design_guid, piece_guid)
    except Exception as e:
        return {"error": str(e)}


def find_used_connectors_by_piece_in_design(kit: dict, design_guid: str, piece_guid: str) -> list:
    """Get all connectors of a piece that are used in connections."""
    try:
        return findUsedConnectorsByPieceInDesignDict(kit, design_guid, piece_guid)
    except Exception as e:
        return {"error": str(e)}


def find_replaceable_types_for_piece_in_design(kit: dict, design_guid: str, piece_guid: str, variants: list[str] = None) -> list:
    """Find all types that can replace a piece while maintaining connection compatibility. Optionally filter by variant parent GUIDs."""
    try:
        return findReplaceableTypesForPieceInDesignDict(kit, design_guid, piece_guid, variants)
    except Exception as e:
        return {"error": str(e)}


def find_replaceable_types_for_pieces_in_design(kit: dict, design_guid: str, piece_guids: list[str], variants: list[str] = None) -> list:
    """Find types that can replace multiple pieces while maintaining all external connections."""
    try:
        return findReplaceableTypesForPiecesInDesignDict(kit, design_guid, piece_guids, variants)
    except Exception as e:
        return {"error": str(e)}


def create_clustered_design(original_design: dict, cluster_piece_ids: list[str], design_name: str) -> dict:
    """Create a new design from a subset of pieces. Returns the clustered design and external connections."""
    try:
        return createClusteredDesignDict(original_design, cluster_piece_ids, design_name)
    except Exception as e:
        return {"error": str(e)}


def replace_cluster_with_design(original_design: dict, cluster_piece_ids: list[str], clustered_design: dict, external_connections: list[dict]) -> dict:
    """Compute a design diff that replaces clustered pieces with a single design reference."""
    try:
        return replaceClusterWithDesignDict(original_design, cluster_piece_ids, clustered_design, external_connections)
    except Exception as e:
        return {"error": str(e)}


def get_clusterable_groups(design: dict, selected_piece_ids: list[str]) -> list:
    """Get groups of selected pieces that can be clustered into new designs."""
    try:
        return getClusterableGroupsDict(design, selected_piece_ids)
    except Exception as e:
        return {"error": str(e)}


def expand_design_pieces(design: dict, kit: dict) -> dict:
    """Recursively expand design references by inlining their pieces and connections."""
    try:
        return expandDesignPiecesDict(design, kit)
    except Exception as e:
        return {"error": str(e)}


def find_attribute_value(entity: dict, name: str, default_value: str = None) -> dict:
    """Find an attribute value on an entity by key name."""
    try:
        sentinel = ... if default_value is None else default_value
        result = findAttributeValueDict(entity, name, sentinel)
        return {"value": result}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def read_current_kit(ctx: Context) -> dict:
    """Read the current session kit."""
    try:
        return _get_session_kit(ctx)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_new_design(
    guid: str,
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
        design = {
            "guid": guid,
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
        stored_design = _replace_design_in_session_kit(ctx, design)
        return {"ok": True, "guid": stored_design["guid"], "name": stored_design["name"]}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def add_current_design_author(guid: str, ctx: Context) -> dict:
    """Add an author reference to the current design by GUID."""
    try:
        design = _mutate_current_design(ctx, lambda current_design: current_design.setdefault("authors", []).append({"guid": guid}))
        return {"ok": True, "authorCount": len(design.get("authors", []))}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def add_current_design_prop(guid: str, quality_guid: str, value: str, unit: str, ctx: Context) -> dict:
    """Add a prop entry to the current design."""
    try:

        def mutate(current_design: dict):
            current_design.setdefault("props", []).append(
                {
                    "guid": guid,
                    "quality": {"guid": quality_guid},
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
    guid: str,
    name: str,
    kind_guid: str,
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
                    "guid": guid,
                    "name": name,
                    "description": description,
                    "isHidden": is_hidden,
                    "isLocked": is_locked,
                    "type": {"guid": kind_guid},
                }
            )

        design = _mutate_current_design(ctx, mutate)
        return {"ok": True, "pieceCount": len(design.get("pieces", []))}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def add_current_design_piece_with_plane(
    guid: str,
    name: str,
    kind_guid: str,
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
                    "guid": guid,
                    "name": name,
                    "description": description,
                    "isHidden": is_hidden,
                    "isLocked": is_locked,
                    "type": {"guid": kind_guid},
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
    guid: str,
    connected_piece_guid: str,
    connected_connector_guid: str,
    connecting_piece_guid: str,
    connecting_connector_guid: str,
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
                    "guid": guid,
                    "gap": gap,
                    "description": description,
                    "connected": {
                        "piece": {"guid": connected_piece_guid},
                        "connector": {"guid": connected_connector_guid},
                    },
                    "tilt": tilt,
                    "rotation": rotation,
                    "rise": rise,
                    "turn": turn,
                    "connecting": {
                        "piece": {"guid": connecting_piece_guid},
                        "connector": {"guid": connecting_connector_guid},
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
def start_working_in_design(guid: str, ctx: Context) -> CallToolResult:
    """Select a design by GUID within the current kit. Requires start_working_in_local_kit to have been called first."""
    try:
        kit = _get_session_kit(ctx)
        design = next((d for d in kit.get("designs", []) if d.get("guid") == guid), None)
        if design is None:
            return _as_mcp_app_tool_result({"error": f"Design with guid {guid} not found in kit."}, is_error=True)
        sid = _session_id(ctx)
        _mcp_session_designs[sid] = design
        return _build_app_response("show-design", ctx)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


def _read_current_design(ctx: Context) -> dict:
    """Read the current design set via start_working_in_design."""
    try:
        return _get_session_design(ctx)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def read_current_design(ctx: Context) -> dict:
    """Read the current design that was set via start_working_in_design or start_new_design."""
    return _read_current_design(ctx)


@mcp.tool()
def finish_working_in_design(ctx: Context) -> dict:
    """Clear the current design from session state."""
    try:
        sid = _session_id(ctx)
        _mcp_session_designs.pop(sid, None)
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_working_in_type(guid: str, ctx: Context) -> dict:
    """Select a type by GUID within the current kit. Requires start_working_in_local_kit to have been called first."""
    try:
        kit = _get_session_kit(ctx)
        t = next((t for t in kit.get("types", []) if t.get("guid") == guid), None)
        if t is None:
            return {"error": f"Type with guid {guid} not found in kit."}
        sid = _session_id(ctx)
        _mcp_session_types[sid] = t
        return {"ok": True, "guid": guid, "name": t.get("name", "")}
    except Exception as e:
        return {"error": str(e)}


def _read_current_type(ctx: Context) -> dict:
    """Read the current type set via start_working_in_type."""
    try:
        return _get_session_type(ctx)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def read_current_type(ctx: Context) -> dict:
    """Read the current type that was set via start_working_in_type."""
    return _read_current_type(ctx)


@mcp.tool()
def finish_working_in_type(ctx: Context) -> dict:
    """Clear the current type from session state."""
    try:
        sid = _session_id(ctx)
        _mcp_session_types.pop(sid, None)
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def finish_working_in_kit(ctx: Context) -> dict:
    """Clear the current kit, design, type, mode, and source from session state."""
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
    """Start a session-scoped transaction. Only one active transaction is allowed per session."""
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
    """Finalize the active session transaction and keep all applied changes."""
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
    """Abort the active session transaction and rollback all recorded changes in reverse order."""
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
    """Finalize the active session transaction and keep all applied changes."""
    return finalize_transaction(ctx)


@mcp.tool()
def transaction_abort(ctx: Context) -> dict:
    """Abort the active session transaction and rollback all recorded changes in reverse order."""
    return abort_transaction(ctx)


@mcp.tool()
def sum_quality_in_design(design_guid: str, quality_guid: str, ctx: Context) -> dict:
    """Sum the values of a quality across all pieces in a design, using piece-level props with fallback to type-level props."""
    try:
        kit = _get_session_kit(ctx)
        return {"result": sumQualityInDesignDict(kit, design_guid, quality_guid)}
    except Exception as e:
        return {"error": str(e)}


# region MCP Selection Tools
# [👤semio📚engine💻engine🔖mcp🔖mcpselectiontools](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Selection%20Tools)
# MCP Selection Tools MUST manage session-scoped piece/connection selection state.


def _get_session_selection(ctx) -> dict[str, list[str]]:
    """Get current selection from session."""
    sid = _session_id(ctx)
    return _mcp_session_selection.get(sid, {"pieceGuids": [], "connectionGuids": []})


def _set_session_selection(ctx, selection: dict[str, list[str]]):
    """Set selection in session."""
    sid = _session_id(ctx)
    _mcp_session_selection[sid] = selection


@mcp.tool()
def read_current_selection(ctx: Context) -> dict:
    """Read the current piece and connection selection for this session. Returns pieceGuids and connectionGuids."""
    try:
        return _get_session_selection(ctx)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def set_current_selection(ctx: Context, piece_guids: list[str] | None = None, connection_guids: list[str] | None = None) -> dict:
    """Set the current piece and connection selection for this session."""
    try:
        _set_session_selection(
            ctx,
            {
                "pieceGuids": piece_guids or [],
                "connectionGuids": connection_guids or [],
            },
        )
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def clear_current_selection(ctx: Context) -> dict:
    """Clear the current piece and connection selection for this session."""
    try:
        sid = _session_id(ctx)
        _mcp_session_selection.pop(sid, None)
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


# endregion MCP Selection Tools

# region MCP App Tools
# [👤semio📚engine💻engine🔖mcp🔖mcpapptools](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools)
# MCP App Tools MUST expose design visualization/selection intents as MCP tools.
# Diagram and kit tools return CallToolResult with text content and structuredContent (MCP Apps template in .repo/✍️/mcp-app.md).
# Kit-loading tools declare ui://semio/kit-viewer; diagram tools declare ui://semio/design-viewer.
# Both nested (_meta.ui.resourceUri) and flat (_meta["ui/resourceUri"]) keys are required for
# host compatibility, matching the registerAppTool normalization from @modelcontextprotocol/ext-apps/server.


def _as_mcp_app_tool_result(payload: dict[str, typing.Any], *, is_error: bool = False) -> CallToolResult:
    """Build tools/call result with full payload in text content and a fetchUrl fallback for hosts that truncate.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️asmcpapptoolresult](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/_as_mcp_app_tool_result)
    """
    token = uuid.uuid4().hex
    _mcp_app_payloads[token] = payload
    payload["fetchUrl"] = f"http://localhost:{PORT}/api/app/payload/{token}"
    text = json.dumps(payload)
    return CallToolResult(
        content=[
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
    """Serializable kit viewer payload (diagram lists empty; kitArtifacts populated)."""
    return {
        "points": [],
        "lines": [],
        "capabilities": {"pieceSelection": False, "connectionSelection": False},
        "kitArtifacts": _build_kit_artifact_data(kit),
    }


def _build_kit_only_app_response(kit: dict) -> CallToolResult:
    """MCP Apps kit-viewer tool response with kit artifact data only (no diagram)."""
    return _as_mcp_app_tool_result(_build_kit_only_app_payload(kit))


def _build_kit_artifact_data(kit: dict) -> dict:
    """Build a minimal kit artifact payload for UI selection (designs, types, connectors)."""
    meta: dict = {
        "name": kit.get("name") or "",
        "version": kit.get("version") or "",
    }
    if kit.get("guid"):
        meta["guid"] = kit.get("guid")
    for key in ("description", "createdAt", "updatedAt", "homepage", "remote", "preview", "icon", "image", "license"):
        value = kit.get(key)
        if value:
            meta[key] = value
    designs = []
    for d in kit.get("designs", []) or []:
        guid = d.get("guid")
        if not guid:
            continue
        design_payload = {"guid": guid, "name": d.get("name", ""), "variant": d.get("variant", ""), "view": d.get("view", "")}
        parent = d.get("parent")
        if isinstance(parent, dict) and parent.get("guid"):
            design_payload["parent"] = {"guid": parent.get("guid")}
        for key in ("description", "createdAt", "updatedAt", "unit", "icon", "image"):
            value = d.get(key)
            if value:
                design_payload[key] = value
        designs.append(design_payload)

    types = []
    ports = []
    for t in kit.get("types", []) or []:
        t_guid = t.get("guid")
        if not t_guid:
            continue
        type_payload = {"guid": t_guid, "name": t.get("name", ""), "variant": t.get("variant", "")}
        parent = t.get("parent")
        if isinstance(parent, dict) and parent.get("guid"):
            type_payload["parent"] = {"guid": parent.get("guid")}
        for key in ("description", "createdAt", "updatedAt", "icon", "image"):
            value = t.get(key)
            if value:
                type_payload[key] = value
        types.append(type_payload)
        for c in t.get("connectors", []) or []:
            c_guid = c.get("guid")
            if not c_guid:
                continue
            ports.append(
                {
                    "guid": c_guid,
                    "typeGuid": t_guid,
                    "id": c.get("id", ""),
                    "port": c.get("port", ""),
                    "name": c.get("name", "") or c.get("id", "") or c.get("port", "") or "port",
                    "description": c.get("description", ""),
                    "mandatory": bool(c.get("mandatory", False)),
                }
            )

    meta["designs"] = designs
    meta["types"] = types
    meta["ports"] = ports
    return meta


def _build_diagram_data(kit: dict, design_guid: str, design_diff: dict | None = None) -> dict:
    """Compute pre-rendered diagram points and lines from kit/design data.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️builddiagramdata](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/_build_diagram_data)
    """
    design = next((d for d in kit.get("designs", []) if d.get("guid") == design_guid), None)
    if design is None:
        return {"points": [], "lines": []}

    # Flatten the design to get absolute piece positions
    try:
        flatten_result = flattenDesignDict(kit, design_guid)
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
    pieces = design.get("pieces", [])
    piece_map: dict[str, dict] = {}
    for p in pieces:
        guid = p.get("guid")
        if not guid:
            continue
        center = piece_centers.get(guid, p.get("center") or {"u": 0, "v": 0})
        piece_map[guid] = {"guid": guid, "id": p.get("id", ""), "center": center}

    # Determine diff statuses
    removed_piece_guids: set[str] = set()
    added_piece_guids: set[str] = set()
    modified_piece_guids: set[str] = set()
    removed_conn_guids: set[str] = set()
    added_conn_guids: set[str] = set()
    modified_conn_guids: set[str] = set()

    if design_diff:
        for p in design_diff.get("pieces", {}).get("removed", []):
            removed_piece_guids.add(p.get("guid", ""))
        for p in design_diff.get("pieces", {}).get("added", []):
            guid = p.get("guid", "")
            added_piece_guids.add(guid)
            # Include added pieces in the map with their centers
            center = p.get("center") or {"u": 0, "v": 0}
            piece_map[guid] = {"guid": guid, "id": p.get("id", ""), "center": center}
        for p in design_diff.get("pieces", {}).get("updated", []):
            modified_piece_guids.add(p.get("piece", {}).get("guid", ""))
        for c in design_diff.get("connections", {}).get("removed", []):
            removed_conn_guids.add(c.get("guid", ""))
        for c in design_diff.get("connections", {}).get("added", []):
            added_conn_guids.add(c.get("guid", ""))
        for c in design_diff.get("connections", {}).get("updated", []):
            modified_conn_guids.add(c.get("connection", {}).get("guid", ""))

    # Build points
    points = []
    for guid, pdata in piece_map.items():
        status = "default"
        if guid in removed_piece_guids:
            status = "removed"
        elif guid in added_piece_guids:
            status = "added"
        elif guid in modified_piece_guids:
            status = "modified"
        center = pdata.get("center", {"u": 0, "v": 0})
        points.append(
            {
                "guid": guid,
                "id": pdata.get("id", ""),
                "u": center.get("u", 0),
                "v": center.get("v", 0),
                "status": status,
            }
        )

    # Build lines from connections
    connections = design.get("connections", [])
    # Also include added connections from diff
    if design_diff:
        for c in design_diff.get("connections", {}).get("added", []):
            connections = list(connections) + [c]

    lines = []
    for c in connections:
        guid = c.get("guid")
        if not guid:
            continue
        source_guid = c.get("connected", {}).get("piece", {}).get("guid")
        target_guid = c.get("connecting", {}).get("piece", {}).get("guid")
        source = piece_map.get(source_guid)
        target = piece_map.get(target_guid)
        if not source or not target:
            continue
        source_center = source.get("center", {"u": 0, "v": 0})
        target_center = target.get("center", {"u": 0, "v": 0})
        status = "default"
        if guid in removed_conn_guids:
            status = "removed"
        elif guid in added_conn_guids:
            status = "added"
        elif guid in modified_conn_guids:
            status = "modified"
        lines.append(
            {
                "guid": guid,
                "sourceU": source_center.get("u", 0),
                "sourceV": source_center.get("v", 0),
                "targetU": target_center.get("u", 0),
                "targetV": target_center.get("v", 0),
                "status": status,
            }
        )

    return {"points": points, "lines": lines}


def _build_app_payload(mode: str, ctx, design_diff: dict | None = None, capabilities: dict | None = None) -> dict[str, typing.Any]:
    """Pre-computed diagram + kit artifact dict for design-viewer tools."""
    kit = _get_session_kit(ctx)
    design = _get_session_design(ctx)
    design_guid = design.get("guid")

    diagram_data = _build_diagram_data(kit, design_guid, design_diff)
    diagram_data["mode"] = mode
    diagram_data["capabilities"] = capabilities or {
        "pieceSelection": mode in ("select-pieces", "select-pieces-and-connections"),
        "connectionSelection": mode in ("select-connections", "select-pieces-and-connections"),
    }
    diagram_data["kitArtifacts"] = _build_kit_artifact_data(kit)

    try:
        flatten_result = flattenDesignDict(kit, design_guid)
        flatten_by_guid: dict[str, dict] = {}
        for update in flatten_result.get("pieces", {}).get("updated", []):
            pid = update.get("id")
            if pid:
                flatten_by_guid[pid] = update.get("diff", {})
        enriched_pieces = []
        for p in design.get("pieces", []):
            guid = p.get("guid")
            flat = flatten_by_guid.get(guid) if guid else None
            if flat:
                ep = dict(p)
                if flat.get("plane") and not ep.get("plane"):
                    ep["plane"] = flat["plane"]
                if flat.get("center") and not ep.get("center"):
                    ep["center"] = flat["center"]
                enriched_pieces.append(ep)
            else:
                enriched_pieces.append(p)
        enriched_design = dict(design)
        enriched_design["pieces"] = enriched_pieces
    except Exception:
        enriched_design = design

    diagram_data["design"] = enriched_design

    kit_for_ui = copy.deepcopy(kit)
    for f in kit_for_ui.get("files", []):
        f.pop("blob", None)
    diagram_data["kit"] = kit_for_ui

    return diagram_data


def _build_app_response(mode: str, ctx, design_diff: dict | None = None, capabilities: dict | None = None) -> CallToolResult:
    """MCP Apps design-viewer tool response with pre-computed diagram data and structuredContent.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️buildappresponse](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/_build_app_response)
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
    """Serve the MCP App design viewer HTML built from @semio/ui.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️designviewerresource](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/design_viewer_resource)
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
    """Serve the MCP kit viewer HTML (SemioKit-only shell) built from @semio/ui.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️kitviewerresource](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/kit_viewer_resource)
    """
    return _build_kit_viewer_html()


@mcp.tool(meta=_APP_RESOURCE_META)
def show_design(ctx: Context) -> CallToolResult:
    """Show the current design in the split design viewer (scene + 2D diagram). Requires an active kit and design session."""
    try:
        return _build_app_response("show-design", ctx)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_APP_RESOURCE_META)
def show_diagram(ctx: Context) -> CallToolResult:
    """Show the current design as a 2D diagram only (no 3D scene panel). Requires an active kit and design session."""
    try:
        return _build_app_response("show-diagram", ctx)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_APP_RESOURCE_META)
def show_scene(ctx: Context) -> CallToolResult:
    """Show the current design in the 3D scene viewer. Requires an active kit and design session."""
    try:
        return _build_app_response("show-scene", ctx)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_APP_RESOURCE_META)
def show_diff(ctx: Context, design_diff: dict | None = None) -> CallToolResult:
    """Show a diff of the current design as a 2D diagram with diff coloring. Uses an empty diff if none is provided. Requires an active kit and design session."""
    try:
        return _build_app_response("show-diff", ctx, design_diff=design_diff)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_APP_RESOURCE_META)
def show_diagram_diff(ctx: Context, design_diff: dict | None = None) -> CallToolResult:
    """Show a diff of the current design as a 2D diagram only with diff coloring. Uses an empty diff if none is provided. Requires an active kit and design session."""
    try:
        return _build_app_response("show-diagram-diff", ctx, design_diff=design_diff)
    except Exception as e:
        return _as_mcp_app_tool_result({"error": str(e)}, is_error=True)


@mcp.tool(meta=_APP_RESOURCE_META)
def select_pieces(ctx: Context) -> CallToolResult:
    """Open a piece selection view where only pieces can be selected. Requires an active kit and design session."""
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


@mcp.tool(meta=_APP_RESOURCE_META)
def select_connections(ctx: Context) -> CallToolResult:
    """Open a connection selection view where only connections can be selected. Requires an active kit and design session."""
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


@mcp.tool(meta=_APP_RESOURCE_META)
def select_pieces_and_connections(ctx: Context) -> CallToolResult:
    """Open a combined selection view where both pieces and connections can be selected. Requires an active kit and design session."""
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


# endregion MCP App Tools


# endregion Mcp

# region Engine
# [👤semio📚engine💻engine🔖engine](repo://p/u/semio/b/l/engine/f/engine.py/s/Engine)
# Engine MUST mount REST, GraphQL, and MCP sub-applications and manage the server lifecycle.


@contextlib.asynccontextmanager
async def engineLifespan(app):
    """Manages the MCP session lifecycle during engine startup and shutdown.
    Callers MUST use this as the lifespan parameter for the Starlette application.
    [👤semio📚engine💻engine🔖engine🛠️enginelifespan](repo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/engineLifespan)
    """
    async with mcp.session_manager.run():
        yield


mcp.settings.streamable_http_path = "/"
engine = starlette.applications.Starlette(lifespan=engineLifespan)
engine.mount("/api", rest)
engine.mount(
    "/graphql",
    starlette_graphene3.GraphQLApp(graphqlSchema, on_get=starlette_graphene3.make_graphiql_handler()),
)
engine.mount("/mcp", mcp.streamable_http_app())


def generateSchemas():
    """Exports OpenAPI, JSON Schema, SQLite schema, and GraphQL schema files to disk.
    Callers MUST run this from the engine directory with write access to output paths.
    [👤semio📚engine💻engine🔖engine🛠️generateschemas](repo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/generateSchemas)
    """
    if os.path.exists("temp"):
        for root, dirs, files in os.walk("temp", topdown=False):
            for name in files:
                os.remove(os.path.join(root, name))
            for name in dirs:
                os.rmdir(os.path.join(root, name))
    else:
        os.makedirs("temp")

    with open("../../openapi/schema.json", "w", encoding="utf-8") as f:
        json.dump(rest.openapi(), f, indent=4)

    with open("../../semioonschema/kit.json", "w", encoding="utf-8") as f:
        json.dump(
            KitOutput.model_json_schema(schema_generator=OutputGenerateJsonSchema),
            f,
            indent=4,
        )

    with open("../../semioonschema/design-context.json", "w", encoding="utf-8") as f:
        json.dump(
            DesignContext.model_json_schema(schema_generator=ContextGenerateJsonSchema),
            f,
            indent=4,
        )

    with open("../../semioonschema/design.json", "w", encoding="utf-8") as f:
        json.dump(
            DesignOutput.model_json_schema(schema_generator=OutputGenerateJsonSchema),
            f,
            indent=4,
        )

    with open("../../semioonschema/design-prediction.json", "w", encoding="utf-8") as f:
        json.dump(
            DesignPrediction.model_json_schema(schema_generator=PredictionGenerateJsonSchema),
            f,
            indent=4,
        )

    with open("../../semioonschema/type.json", "w", encoding="utf-8") as f:
        json.dump(
            TypeOutput.model_json_schema(schema_generator=OutputGenerateJsonSchema),
            f,
            indent=4,
        )

    with open("../../semioonschema/type-context.json", "w", encoding="utf-8") as f:
        json.dump(
            TypeContext.model_json_schema(schema_generator=ContextGenerateJsonSchema),
            f,
            indent=4,
        )

    sqliteSchemaPath = "../../sqlite/schema.sql"
    # SQLite schema is now maintained manually in sqlite/schema.sql
    # No auto-generation from ORM metadata

    with open("../../graphql/schema.graphql", "w", encoding="utf-8") as f:
        f.write(str(graphqlSchema))


def start_engine():
    """Starts the uvicorn server hosting the engine application.
    Callers MUST invoke this in a separate process to avoid blocking the UI.
    [👤semio📚engine💻engine🔖engine🛠️startengine](repo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/start_engine)
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
    """Terminates the running engine process and starts a new one.
    Callers MUST ensure a PySide6 QApplication instance is running.
    [👤semio📚engine💻engine🔖engine🛠️restartengine](repo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/restart_engine)
    """
    import PySide6.QtWidgets

    ui_instance = PySide6.QtWidgets.QApplication.instance()
    engine_process = ui_instance.engine_process
    if engine_process.is_alive():
        engine_process.terminate()
    ui_instance.engine_process = multiprocessing.Process(target=start_engine)
    ui_instance.engine_process.start()


def run(dev_mode: bool | None = None):
    """Main entry point that starts the engine with optional dev mode and system tray UI.
    Callers MUST invoke this from the __main__ block or dev function.
    [👤semio📚engine💻engine🔖engine🛠️run](repo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/run)
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

        debugpy.listen(("0.0.0.0", 5678))
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
    """Runs before dev()
    Callers MUST NOT add blocking operations in this hook.
    [👤semio📚engine💻engine🔖engine🛠️predev](repo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/preDev)
    """


def dev():
    """Starts the engine in development mode with debugging enabled.
    Callers MUST have debugpy available when using this entry point.
    [👤semio📚engine💻engine🔖engine🛠️dev](repo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/dev)
    """
    run(dev_mode=True)


if __name__ == "__main__":
    run()

# endregion Engine
