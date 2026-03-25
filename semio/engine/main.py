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
import base64
import contextlib
import copy
import datetime
import difflib
import enum
import functools
import html as htmlmodule
import importlib.util as _ilu
import io
import json
import logging
import multiprocessing
import os
import pathlib
import shutil
import signal
import sqlite3
import sys
import typing
import zipfile

import fastapi
import fastapi.openapi
import graphene
import jinja2
import lark
import openai
import pydantic
import requests
import starlette.applications
import starlette_graphene3
import uvicorn
from mcp.server.fastmcp import Context, FastMCP
from mcp.types import CallToolResult, ImageContent, TextContent

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
    authorToMeta,
    connectionToMeta,
    connectorToMeta,
    conceptToMeta,
    createClusteredDesignDict,
    decode,
    designToMeta,
    designToShallow,
    encode,
    expandDesignPiecesDict,
    fileToMeta,
    findAttributeValueDict,
    findPieceTypeInDesignDict,
    findReplaceableTypesForPieceInDesignDict,
    findReplaceableTypesForPiecesInDesignDict,
    findSameFamilyDesignPiecesDict,
    findUsedConnectorsByPieceInDesignDict,
    flattenDesignDict,
    folderToMeta,
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
    kitToShallow,
    logger,
    modelToMeta,
    normalizeAngle,
    parseValidationResult,
    piecesMetadataDict,
    pieceToMeta,
    planeFromYAxis,
    portToMeta,
    qualityToMeta,
    replaceClusterWithDesignDict,
    sumQualityInDesignDict,
    tagToMeta,
    typeToMeta,
    typeToShallow,
    validateKitDict,
)

# endregion Imports


# region Shallow Diff Helpers

def _shallowifyCollectionDiff(collDiff: dict, metaFn, nestedDiffHandler=None) -> dict:
    """Convert added items in a collection diff to meta/shallow, and optionally process nested updated diffs."""
    if not collDiff or not isinstance(collDiff, dict):
        return collDiff
    result = dict(collDiff)
    if "added" in result:
        result["added"] = [metaFn(item) for item in result["added"]]
    if nestedDiffHandler and "updated" in result:
        result["updated"] = [
            {**entry, "diff": nestedDiffHandler(entry["diff"])} if "diff" in entry else entry
            for entry in result["updated"]
        ]
    return result


def _shallowifyTypeDiff(typeDiff: dict) -> dict:
    """Convert full entities in a type diff to meta."""
    if not typeDiff or not isinstance(typeDiff, dict):
        return typeDiff
    result = dict(typeDiff)
    if "connectors" in result:
        result["connectors"] = _shallowifyCollectionDiff(result["connectors"], connectorToMeta)
    if "models" in result:
        result["models"] = _shallowifyCollectionDiff(result["models"], modelToMeta)
    return result


def _shallowifyDesignDiff(designDiff: dict) -> dict:
    """Convert full entities in a design diff to meta."""
    if not designDiff or not isinstance(designDiff, dict):
        return designDiff
    result = dict(designDiff)
    if "pieces" in result:
        result["pieces"] = _shallowifyCollectionDiff(result["pieces"], pieceToMeta)
    if "connections" in result:
        result["connections"] = _shallowifyCollectionDiff(result["connections"], connectionToMeta)
    return result


def _shallowifyKitDiff(kitDiff: dict) -> dict:
    """Convert full entities in a kit diff to shallow/meta."""
    if not kitDiff or not isinstance(kitDiff, dict):
        return kitDiff
    result = dict(kitDiff)
    if "types" in result:
        result["types"] = _shallowifyCollectionDiff(result["types"], typeToShallow, _shallowifyTypeDiff)
    if "designs" in result:
        result["designs"] = _shallowifyCollectionDiff(result["designs"], designToShallow, _shallowifyDesignDiff)
    for key, metaFn in [("tags", tagToMeta), ("concepts", conceptToMeta), ("ports", portToMeta),
                        ("files", fileToMeta), ("folders", folderToMeta), ("qualities", qualityToMeta),
                        ("authors", authorToMeta)]:
        if key in result:
            result[key] = _shallowifyCollectionDiff(result[key], metaFn)
    return result


def _shallowifyChange(changeDict: dict) -> dict:
    """Convert full entities in a change dict (forward/backward diffs + before/after) to shallow."""
    if not changeDict or not isinstance(changeDict, dict):
        return changeDict
    result = dict(changeDict)
    if "forward" in result:
        result["forward"] = _shallowifyKitDiff(result["forward"])
    if "backward" in result:
        result["backward"] = _shallowifyKitDiff(result["backward"])
    if "before" in result:
        result["before"] = kitToShallow(result["before"])
    if "after" in result:
        result["after"] = kitToShallow(result["after"])
    return result


def _shallowifyDesignChange(changeDict: dict) -> dict:
    """Convert full entities in a design change dict to shallow."""
    if not changeDict or not isinstance(changeDict, dict):
        return changeDict
    result = dict(changeDict)
    if "forward" in result:
        result["forward"] = _shallowifyDesignDiff(result["forward"])
    if "backward" in result:
        result["backward"] = _shallowifyDesignDiff(result["backward"])
    if "before" in result:
        result["before"] = designToShallow(result["before"])
    if "after" in result:
        result["after"] = designToShallow(result["after"])
    return result


# endregion Shallow Diff Helpers

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
    openaiClient = openai.Client()
except openai.OpenAIError:
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

# Session-scoped state. Keyed by session id for isolation.
_mcp_session_kits: dict[int, dict[str, typing.Any]] = {}
_mcp_session_designs: dict[int, dict[str, typing.Any]] = {}
_mcp_session_types: dict[int, dict[str, typing.Any]] = {}
_mcp_session_kit_mode: dict[int, str] = {}
_mcp_session_kit_source: dict[int, str] = {}
_mcp_session_transactions: dict[int, Transaction] = {}
_mcp_session_transaction_rollback: set[int] = set()


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


# region MCP Kit Path Resolution
def _resolve_local_kit_path(path: str) -> pathlib.Path:
    """Resolve a local kit path, including shorthand asset paths that omit the `semio` namespace folder.
    [👤semio📚engine💻engine🔖mcp🔖kitpathresolution](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/Kit%20Path%20Resolution)
    """
    candidate = pathlib.Path(path).expanduser()
    if candidate.exists():
        return candidate.resolve()

    parent = candidate.parent
    name = candidate.name
    stem = candidate.stem
    suffix = candidate.suffix
    fallback_candidates: list[pathlib.Path] = []

    if name and parent.name != "semio":
        fallback_candidates.append(parent / "semio" / name)
        if suffix == "":
            fallback_candidates.append(parent / "semio" / f"{name}.kit.json")
            fallback_candidates.append(parent / "semio" / f"kit_{name}.json")
            fallback_candidates.append(parent / "semio" / name / "kit.json")
            fallback_candidates.append(parent / "semio" / name / "kit_metabolism.json")
        elif suffix == ".json":
            fallback_candidates.append(parent / "semio" / name)
            fallback_candidates.append(parent / "semio" / f"{stem}.kit.json")
            fallback_candidates.append(parent / "semio" / f"kit_{stem}.json")

    for fallback_candidate in fallback_candidates:
        if fallback_candidate.exists():
            return fallback_candidate.resolve()

    return candidate.resolve()


def _load_kit_from_path(path: str) -> dict:
    """Load kit dict from path (JSON file or folder with .semio/kit.sqlite3 or kit JSON).
    [👤semio📚engine💻engine🔖mcp🛠️loadkitfrompath](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/_load_kit_from_path)
    """
    p = _resolve_local_kit_path(path)
    if p.is_file() and p.suffix == ".json":
        with open(p, "r", encoding="utf-8") as f:
            return json.load(f)
    if p.is_dir():
        local_json_path = p / KIT_LOCAL_FOLDERNAME / "kit.json"
        if local_json_path.exists():
            with open(local_json_path, "r", encoding="utf-8") as f:
                return json.load(f)
        sqlite_path = p / KIT_LOCAL_FOLDERNAME / KIT_LOCAL_FILENAME
        if sqlite_path.exists():
            store = StoreFactory(str(p))
            kit = store.get({"kind": "kit", "kitUri": str(p)})
            return kit.model_dump() if hasattr(kit, "model_dump") else KitOutput.model_validate(kit).model_dump()
        for name in ("kit_metabolism.json", "kit.json"):
            json_path = p / name
            if json_path.exists():
                with open(json_path, "r", encoding="utf-8") as f:
                    return json.load(f)
        parent_json = p.parent / "kit_metabolism.json"
        if parent_json.exists():
            with open(parent_json, "r", encoding="utf-8") as f:
                return json.load(f)
    raise FileNotFoundError(f"Kit not found at path: {path}")
# endregion MCP Kit Path Resolution


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


def _get_session_design(ctx) -> dict[str, typing.Any]:
    """Get current design from session. Raises if start_working_in_design was not called."""
    sid = _session_id(ctx)
    if sid is None or sid not in _mcp_session_designs:
        raise ValueError("Call start_working_in_design(guid) first to set the design for this session.")
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


@mcp.tool()
def start_working_in_local_kit(path: str, ctx: Context) -> dict:
    """Start working in a local kit for this MCP session. MUST be called first.
    Path: absolute path to kit folder (with .semio/kit.sqlite3) or JSON file, or folder containing kit_metabolism.json.
    [👤semio📚engine💻engine🔖mcp🛠️startworkinginlocalkit](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/start_working_in_local_kit)
    """
    try:
        kit = _load_kit_from_path(path)
        sid = _session_id(ctx)
        _set_session_kit(ctx, kit)
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        _mcp_session_kit_mode[sid] = "local"
        _mcp_session_kit_source[sid] = path
        return kitToShallow(kit)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_new_kit(name: str, version: str, ctx: Context) -> dict:
    """Start a new in-memory kit for this MCP session with flat top-level fields only."""
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
        return kitToShallow(kit)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_working_in_remote_kit(serverUrl: str, kitUri: str, ctx: Context) -> dict:
    """Start working in a remote kit for this MCP session. MUST be called first.
    Requires prior login() to the server. Fetches the kit from the remote server.
    [👤semio📚engine💻engine🔖mcp🛠️startworkinginremotekit](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/start_working_in_remote_kit)
    """
    try:
        kit = _load_kit_from_remote(serverUrl, kitUri)
        sid = _session_id(ctx)
        _set_session_kit(ctx, kit)
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        _mcp_session_kit_mode[sid] = "remote"
        _mcp_session_kit_source[sid] = f"{serverUrl}/api/kits/{encode(kitUri)}"
        return kitToShallow(kit)
    except Exception as e:
        return {"error": str(e)}


# region MCP Auth Tools
# [👤semio📚engine💻engine🔖mcp🔖mcpauthtools](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Auth%20Tools)
# MCP Auth Tools MUST expose login, logout and status for remote server authentication.


def mcp_login(serverUrl: str, email: str, password: str) -> dict:
    """🔐 Login to a remote semio server. Stores the auth token for subsequent remote kit operations.
    [👤semio📚engine💻engine🔖mcp🔖mcpauthtools🛠️mcplogin](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Auth%20Tools/d/i/mcp_login)
    """
    try:
        return login(serverUrl, email, password)
    except Exception as e:
        return {"error": str(e)}


def mcp_logout(serverUrl: str) -> dict:
    """🔓 Logout from a remote semio server. Removes the stored token.
    [👤semio📚engine💻engine🔖mcp🔖mcpauthtools🛠️mcplogout](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Auth%20Tools/d/i/mcp_logout)
    """
    try:
        return logout(serverUrl)
    except Exception as e:
        return {"error": str(e)}


def mcp_auth_status(serverUrl: str) -> dict:
    """📋 Get the auth status for a remote semio server.
    [👤semio📚engine💻engine🔖mcp🔖mcpauthtools🛠️mcpauthstatus](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Auth%20Tools/d/i/mcp_auth_status)
    """
    try:
        return getAuthStatus(serverUrl)
    except Exception as e:
        return {"error": str(e)}


# endregion MCP Auth Tools


def validate_kit(kit: dict) -> dict:
    """Validate a kit and return any validation problems.
    Callers MUST provide a dict matching the Kit schema.
    [👤semio📚engine💻engine🔖mcp🛠️validatekit](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/validate_kit)
    """
    try:
        result = validateKitDict(kit)
        return result.model_dump() if hasattr(result, "model_dump") else {"problems": []}
    except Exception as e:
        return {"error": str(e)}


def flatten_design(kit: dict, design_guid: str) -> dict:
    """Flatten a design by computing absolute planes for all pieces.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️flattendesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/flatten_design)
    """
    try:
        return designToShallow(flattenDesignDict(kit, design_guid))
    except Exception as e:
        return {"error": str(e)}


def get_kit_diff(before: dict, after: dict) -> dict:
    """Get the diff between two kit states.
    Callers MUST provide two valid kit dicts for comparison.
    [👤semio📚engine💻engine🔖mcp🛠️getkitdiff](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_kit_diff)
    """
    try:
        return _shallowifyKitDiff(getKitDiffDict(before, after))
    except Exception as e:
        return {"error": str(e)}


def apply_kit_diff(base: dict, diff: dict) -> dict:
    """Apply a diff to a kit.
    Callers MUST provide a valid base kit dict and a compatible diff dict.
    [👤semio📚engine💻engine🔖mcp🛠️applykitdiff](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/apply_kit_diff)
    """
    try:
        return kitToShallow(applyKitDiffDict(base, diff))
    except Exception as e:
        return {"error": str(e)}


def inverse_kit_diff(original: dict, applied_diff: dict) -> dict:
    """Get the inverse of a diff (for undo operations).
    Callers MUST provide the original kit dict and the applied diff dict.
    [👤semio📚engine💻engine🔖mcp🛠️inversekitdiff](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/inverse_kit_diff)
    """
    try:
        return _shallowifyKitDiff(inverseKitDiffDict(original, applied_diff))
    except Exception as e:
        return {"error": str(e)}


def get_kit_change(before: dict, after: dict) -> dict:
    """Get the change (forward and backward diffs) between two kit states for undo/redo.
    Callers MUST provide two valid kit dicts for comparison.
    [👤semio📚engine💻engine🔖mcp🛠️getkitchange](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_kit_change)
    """
    try:
        return _shallowifyChange(changeToDict(getKitChange(before, after)))
    except Exception as e:
        return {"error": str(e)}


def get_design_change(before: dict, after: dict) -> dict:
    """Get the change (forward and backward diffs) between two design states for undo/redo.
    Callers MUST provide two valid design dicts for comparison.
    [👤semio📚engine💻engine🔖mcp🛠️getdesignchange](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_design_change)
    """
    try:
        return _shallowifyDesignChange(changeToDict(getDesignChange(before, after)))
    except Exception as e:
        return {"error": str(e)}


def pieces_metadata(kit: dict, design_guid: str) -> dict:
    """Get metadata for all pieces in a design (plane, center, fixedPieceId, parentPieceId, depth).
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️piecesmetadata](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/pieces_metadata)
    """
    try:
        return piecesMetadataDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_primitive_design(kit: dict, design_guid: str) -> dict:
    """Get the root/primitive design of a design family.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️getprimitivedesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_primitive_design)
    """
    try:
        return designToShallow(getPrimitiveDesignDict(kit, design_guid))
    except Exception as e:
        return {"error": str(e)}


def get_design_family(kit: dict, design_guid: str) -> list:
    """Get all designs in a design family tree.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️getdesignfamily](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_design_family)
    """
    try:
        return [designToShallow(d) for d in getDesignFamilyDict(kit, design_guid)]
    except Exception as e:
        return {"error": str(e)}


def get_design_siblings(kit: dict, design_guid: str) -> list:
    """Get all sibling designs (same parent, excluding self).
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️getdesignsiblings](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_design_siblings)
    """
    try:
        return [designToShallow(d) for d in getDesignSiblingsDict(kit, design_guid)]
    except Exception as e:
        return {"error": str(e)}


def get_design_children(kit: dict, design_guid: str) -> list:
    """Get all direct children of a design.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️getdesignchildren](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_design_children)
    """
    try:
        return [designToShallow(d) for d in getDesignChildrenDict(kit, design_guid)]
    except Exception as e:
        return {"error": str(e)}


def are_designs_in_same_family(kit: dict, design_guid_a: str, design_guid_b: str) -> dict:
    """Check if two designs belong to the same family.
    Callers MUST provide a valid kit dict and two existing design GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️aredesignsinsamefamily](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/are_designs_in_same_family)
    """
    try:
        return {"result": areDesignsInSameFamilyDict(kit, design_guid_a, design_guid_b)}
    except Exception as e:
        return {"error": str(e)}


def can_use_design_as_piece(kit: dict, container_design_guid: str, piece_design_guid: str) -> dict:
    """Check if a design can be used as a piece in another design.
    Callers MUST provide a valid kit dict and two existing design GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️canusedesignaspiece](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/can_use_design_as_piece)
    """
    try:
        return {"result": canUseDesignAsPieceDict(kit, container_design_guid, piece_design_guid)}
    except Exception as e:
        return {"error": str(e)}


def find_same_family_design_pieces(kit: dict, design_guid: str) -> list:
    """Find pieces in a design that reference designs from the same family.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️findsamefamilydesignpieces](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_same_family_design_pieces)
    """
    try:
        return [pieceToMeta(p) for p in findSameFamilyDesignPiecesDict(kit, design_guid)]
    except Exception as e:
        return {"error": str(e)}


def get_primitive_type(kit: dict, type_guid: str) -> dict:
    """Get the root/primitive type of a type family.
    Callers MUST provide a valid kit dict and an existing type GUID.
    [👤semio📚engine💻engine🔖mcp🛠️getprimitivetype](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_primitive_type)
    """
    try:
        return typeToShallow(getPrimitiveTypeDict(kit, type_guid))
    except Exception as e:
        return {"error": str(e)}


def get_type_family(kit: dict, type_guid: str) -> list:
    """Get all types in a type family tree.
    Callers MUST provide a valid kit dict and an existing type GUID.
    [👤semio📚engine💻engine🔖mcp🛠️gettypefamily](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_type_family)
    """
    try:
        return [typeToShallow(t) for t in getTypeFamilyDict(kit, type_guid)]
    except Exception as e:
        return {"error": str(e)}


def get_type_siblings(kit: dict, type_guid: str) -> list:
    """Get all sibling types (same parent, excluding self).
    Callers MUST provide a valid kit dict and an existing type GUID.
    [👤semio📚engine💻engine🔖mcp🛠️gettypesiblings](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_type_siblings)
    """
    try:
        return [typeToShallow(t) for t in getTypeSiblingsDict(kit, type_guid)]
    except Exception as e:
        return {"error": str(e)}


def get_type_children(kit: dict, type_guid: str) -> list:
    """Get all direct children of a type.
    Callers MUST provide a valid kit dict and an existing type GUID.
    [👤semio📚engine💻engine🔖mcp🛠️gettypechildren](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_type_children)
    """
    try:
        return [typeToShallow(t) for t in getTypeChildrenDict(kit, type_guid)]
    except Exception as e:
        return {"error": str(e)}


def are_types_in_same_family(kit: dict, type_guid_a: str, type_guid_b: str) -> dict:
    """Check if two types belong to the same family.
    Callers MUST provide a valid kit dict and two existing type GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️aretypesinsamefamily](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/are_types_in_same_family)
    """
    try:
        return {"result": areTypesInSameFamilyDict(kit, type_guid_a, type_guid_b)}
    except Exception as e:
        return {"error": str(e)}


def find_piece_type_in_design(kit: dict, design_guid: str, piece_guid: str) -> dict:
    """Get the type of a piece in a design.
    Callers MUST provide a valid kit dict, design GUID, and piece GUID.
    [👤semio📚engine💻engine🔖mcp🛠️findpiecetypeindesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_piece_type_in_design)
    """
    try:
        return typeToShallow(findPieceTypeInDesignDict(kit, design_guid, piece_guid))
    except Exception as e:
        return {"error": str(e)}


def find_used_connectors_by_piece_in_design(kit: dict, design_guid: str, piece_guid: str) -> list:
    """Get all connectors of a piece that are used in connections.
    Callers MUST provide a valid kit dict, design GUID, and piece GUID.
    [👤semio📚engine💻engine🔖mcp🛠️findusedconnectorsbypieceindesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_used_connectors_by_piece_in_design)
    """
    try:
        return [connectorToMeta(c) for c in findUsedConnectorsByPieceInDesignDict(kit, design_guid, piece_guid)]
    except Exception as e:
        return {"error": str(e)}


def find_replaceable_types_for_piece_in_design(kit: dict, design_guid: str, piece_guid: str, variants: list[str] = None) -> list:
    """Find all types that can replace a piece while maintaining connection compatibility.
    Callers MUST provide a valid kit dict, design GUID, and piece GUID. Optionally filter by variant parent GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️findreplaceabletypesforpieceindesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_replaceable_types_for_piece_in_design)
    """
    try:
        return [typeToShallow(t) for t in findReplaceableTypesForPieceInDesignDict(kit, design_guid, piece_guid, variants)]
    except Exception as e:
        return {"error": str(e)}


def find_replaceable_types_for_pieces_in_design(kit: dict, design_guid: str, piece_guids: list[str], variants: list[str] = None) -> list:
    """Find types that can replace multiple pieces while maintaining all external connections.
    Callers MUST provide a valid kit dict, design GUID, and list of piece GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️findreplaceabletypesforpiecesindesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_replaceable_types_for_pieces_in_design)
    """
    try:
        return [typeToShallow(t) for t in findReplaceableTypesForPiecesInDesignDict(kit, design_guid, piece_guids, variants)]
    except Exception as e:
        return {"error": str(e)}


def create_clustered_design(original_design: dict, cluster_piece_ids: list[str], design_name: str) -> dict:
    """Create a new design from a subset of pieces (cluster).
    Returns clusteredDesign and externalConnections.
    Callers MUST provide a valid design dict, list of piece GUIDs, and a name for the new design.
    [👤semio📚engine💻engine🔖mcp🛠️createclustereddesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/create_clustered_design)
    """
    try:
        result = createClusteredDesignDict(original_design, cluster_piece_ids, design_name)
        if "clusteredDesign" in result:
            result["clusteredDesign"] = designToShallow(result["clusteredDesign"])
        if "externalConnections" in result:
            result["externalConnections"] = [connectionToMeta(c) for c in result["externalConnections"]]
        return result
    except Exception as e:
        return {"error": str(e)}


def replace_cluster_with_design(original_design: dict, cluster_piece_ids: list[str], clustered_design: dict, external_connections: list[dict]) -> dict:
    """Get a DesignDiff that replaces clustered pieces with a design reference.
    Callers MUST provide the original design, cluster piece IDs, the new clustered design, and external connections.
    [👤semio📚engine💻engine🔖mcp🛠️replaceclusterwithdesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/replace_cluster_with_design)
    """
    try:
        return _shallowifyDesignDiff(replaceClusterWithDesignDict(original_design, cluster_piece_ids, clustered_design, external_connections))
    except Exception as e:
        return {"error": str(e)}


def get_clusterable_groups(design: dict, selected_piece_ids: list[str]) -> list:
    """Get clusterable groups of selected pieces.
    Callers MUST provide a valid design dict and list of selected piece GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️getclusterablegroups](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_clusterable_groups)
    """
    try:
        return getClusterableGroupsDict(design, selected_piece_ids)
    except Exception as e:
        return {"error": str(e)}


def expand_design_pieces(design: dict, kit: dict) -> dict:
    """Recursively expand design references by inlining their pieces and connections.
    Callers MUST provide a valid design dict and kit dict.
    [👤semio📚engine💻engine🔖mcp🛠️expanddesignpieces](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/expand_design_pieces)
    """
    try:
        return designToShallow(expandDesignPiecesDict(design, kit))
    except Exception as e:
        return {"error": str(e)}


def find_attribute_value(entity: dict, name: str, default_value: str = None) -> dict:
    """Find an attribute value on an entity by key.
    Callers MUST provide an entity dict (kit, type, design, piece, etc.) and attribute key name.
    [👤semio📚engine💻engine🔖mcp🛠️findattributevalue](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_attribute_value)
    """
    try:
        sentinel = ... if default_value is None else default_value
        result = findAttributeValueDict(entity, name, sentinel)
        return {"value": result}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def read_current_kit(ctx: Context) -> dict:
    """Read the current session kit as a shallow entity (no blobs, child collections as meta)."""
    try:
        kit = _get_session_kit(ctx)
        return kitToShallow(kit)
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
    """Create and select a new current design with flat metadata fields only."""
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
        return designToShallow(stored_design)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def add_current_design_author(guid: str, ctx: Context) -> dict:
    """Append a flat author reference to the current design."""
    try:
        design = _mutate_current_design(ctx, lambda current_design: current_design.setdefault("authors", []).append({"guid": guid}))
        return designToShallow(design)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def add_current_design_prop(guid: str, quality_guid: str, value: str, unit: str, ctx: Context) -> dict:
    """Append a flat prop entry to the current design."""
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
        return designToShallow(design)
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
    """Append a flat piece entry to the current design without placement fields."""
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
        return designToShallow(design)
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
    """Append a flat piece entry to the current design with explicit placement fields."""
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
        return designToShallow(design)
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
    """Append a flat connection entry to the current design without nested arguments."""
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
        return designToShallow(design)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_working_in_design(guid: str, ctx: Context) -> dict:
    """Start working in a design within the current kit.
    Callers MUST have called start_working_in_local_kit first. Selects the design by GUID.
    [👤semio📚engine💻engine🔖mcp🛠️startworkingindesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/start_working_in_design)
    """
    try:
        kit = _get_session_kit(ctx)
        design = next((d for d in kit.get("designs", []) if d.get("guid") == guid), None)
        if design is None:
            return {"error": f"Design with guid {guid} not found in kit."}
        sid = _session_id(ctx)
        _mcp_session_designs[sid] = design
        return designToShallow(design)
    except Exception as e:
        return {"error": str(e)}


def _read_current_design(ctx: Context) -> dict:
    """Read the current design that was set via start_working_in_design.
    [👤semio📚engine💻engine🔖mcp🛠️readcurrentdesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/read_current_design)
    """
    try:
        return designToShallow(_get_session_design(ctx))
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def read_current_design(ctx: Context) -> dict:
    """Read the current design that was set via start_working_in_design or start_new_design."""
    return _read_current_design(ctx)


@mcp.tool()
def finish_working_in_design(ctx: Context) -> dict:
    """Finish working in the current design. Clears the design from session state.
    [👤semio📚engine💻engine🔖mcp🛠️finishworkingindesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/finish_working_in_design)
    """
    try:
        kit = _get_session_kit(ctx)
        sid = _session_id(ctx)
        _mcp_session_designs.pop(sid, None)
        return kitToShallow(kit)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_working_in_type(guid: str, ctx: Context) -> dict:
    """Start working in a type within the current kit.
    Callers MUST have called start_working_in_local_kit first. Selects the type by GUID.
    [👤semio📚engine💻engine🔖mcp🛠️startworkingintype](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/start_working_in_type)
    """
    try:
        kit = _get_session_kit(ctx)
        t = next((t for t in kit.get("types", []) if t.get("guid") == guid), None)
        if t is None:
            return {"error": f"Type with guid {guid} not found in kit."}
        sid = _session_id(ctx)
        _mcp_session_types[sid] = t
        return typeToShallow(t)
    except Exception as e:
        return {"error": str(e)}


def _read_current_type(ctx: Context) -> dict:
    """Read the current type that was set via start_working_in_type.
    [👤semio📚engine💻engine🔖mcp🛠️readcurrenttype](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/read_current_type)
    """
    try:
        return typeToShallow(_get_session_type(ctx))
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def read_current_type(ctx: Context) -> dict:
    """Read the current type that was set via start_working_in_type."""
    return _read_current_type(ctx)


@mcp.tool()
def finish_working_in_type(ctx: Context) -> dict:
    """Finish working in the current type. Clears the type from session state.
    [👤semio📚engine💻engine🔖mcp🛠️finishworkingintype](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/finish_working_in_type)
    """
    try:
        kit = _get_session_kit(ctx)
        sid = _session_id(ctx)
        _mcp_session_types.pop(sid, None)
        return kitToShallow(kit)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def finish_working_in_kit(ctx: Context) -> dict:
    """Finish working in the current kit. Clears kit, design, type, mode and source from session state.
    [👤semio📚engine💻engine🔖mcp🛠️finishworkinginkit](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/finish_working_in_kit)
    """
    try:
        kit = _get_session_kit(ctx)
        shallow = kitToShallow(kit)
        sid = _session_id(ctx)
        _clear_session_kit(ctx)
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        _mcp_session_kit_mode.pop(sid, None)
        _mcp_session_kit_source.pop(sid, None)
        return shallow
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
        sid_check = _session_id(ctx)
        if sid_check is not None and sid_check in _mcp_session_kits:
            return kitToShallow(_mcp_session_kits[sid_check])
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
        _mcp_session_transactions.pop(sid, None)
        if sid is not None and sid in _mcp_session_kits:
            return kitToShallow(_mcp_session_kits[sid])
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


def abort_transaction(ctx: Context) -> dict:
    """Abort the active session transaction and rollback all recorded changes in reverse order."""
    try:
        sid = _session_id(ctx)
        transaction = _get_active_transaction(sid)
        if transaction is None:
            return {"error": "No active transaction for this session."}
        _mcp_session_transaction_rollback.add(sid)
        try:
            _rollback_session_transaction(sid)
        finally:
            _mcp_session_transaction_rollback.discard(sid)
            _mcp_session_transactions.pop(sid, None)
        if sid is not None and sid in _mcp_session_kits:
            return kitToShallow(_mcp_session_kits[sid])
        return {"ok": True}
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
    """Sum up the values of a quality across all pieces in a design.
    For each piece, uses the piece-level prop if present, otherwise falls back to the type-level prop.
    Callers MUST have called start_working_in_local_kit first.
    [👤semio📚engine💻engine🔖mcp🛠️sumqualityindesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/sum_quality_in_design)
    """
    try:
        kit = _get_session_kit(ctx)
        return {"result": sumQualityInDesignDict(kit, design_guid, quality_guid)}
    except Exception as e:
        return {"error": str(e)}


# region MCP Selection Tools
# [👤semio📚engine💻engine🔖mcp🔖mcpselectiontools](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Selection%20Tools)
# Specs: Session-scoped selection state for pieces and connections, exposed via MCP tools.
# Summary: Three MCP tools for reading, setting, and clearing the current piece/connection selection.

_mcp_session_selection: dict[int, dict[str, list[str]]] = {}
_mcp_session_camera: dict[int, dict[str, typing.Any]] = {}


@mcp.tool()
def read_current_selection(ctx: Context) -> dict:
    """Read the current piece and connection selection for the session.
    [👤semio📚engine💻engine🔖mcp🔖mcpselectiontools🛠️readcurrentselection](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Selection%20Tools/d/i/read_current_selection)
    """
    sid = _session_id(ctx)
    selection = _mcp_session_selection.get(sid, {"pieceGuids": [], "connectionGuids": []})
    return selection


@mcp.tool()
def set_current_selection(ctx: Context, piece_guids: list[str] | None = None, connection_guids: list[str] | None = None) -> dict:
    """Set the current piece and connection selection for the session.
    [👤semio📚engine💻engine🔖mcp🔖mcpselectiontools🛠️setcurrentselection](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Selection%20Tools/d/i/set_current_selection)
    """
    sid = _session_id(ctx)
    selection = {
        "pieceGuids": piece_guids or [],
        "connectionGuids": connection_guids or [],
    }
    _mcp_session_selection[sid] = selection
    return selection


@mcp.tool()
def clear_current_selection(ctx: Context) -> dict:
    """Clear the current selection for the session.
    [👤semio📚engine💻engine🔖mcp🔖mcpselectiontools🛠️clearcurrentselection](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Selection%20Tools/d/i/clear_current_selection)
    """
    sid = _session_id(ctx)
    _mcp_session_selection.pop(sid, None)
    return {"pieceGuids": [], "connectionGuids": []}


# endregion MCP Selection Tools


# region MCP App Tools
# [👤semio📚engine💻engine🔖mcp🔖mcpapptools](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools)
# Specs: MCP App Tools MUST expose eight user-facing intents as separate MCP tools.
# All tools return CallToolResult with an SVG diagram image, text summary, and structuredContent.
# Summary: Eight MCP tools returning rich visual SVG diagrams via CallToolResult.


# region MCP App SVG Generation
# Specs: Generate self-contained SVG diagrams from kit data for embedding in MCP tool results.
# Summary: SVG diagram renderer for pieces (circles) and connections (lines) with diff/selection coloring.


def _svg_status_color(status: str, is_selected: bool = False) -> str:
    """Return SVG fill/stroke color for a diagram entity status.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🔖mcpappsvggeneration🛠️svgstatuscolor](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/s/MCP%20App%20SVG%20Generation/d/i/_svg_status_color)
    """
    if is_selected:
        return "#6366f1"
    if status == "removed":
        return "#ef4444"
    if status == "added":
        return "#22c55e"
    if status == "modified":
        return "#f59e0b"
    return "#a1a1aa"


def _flatten_and_get_flat_design(kit: dict, design_guid: str) -> dict:
    """Flatten a design and return the flat design dict with resolved piece centers.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🔖mcpappsvggeneration🛠️flattenandgetflatdesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/s/MCP%20App%20SVG%20Generation/d/i/_flatten_and_get_flat_design)
    """
    design = None
    for d in kit.get("designs", []):
        if d.get("guid") == design_guid:
            design = d
            break
    if design is None:
        return {"pieces": [], "connections": []}
    flatten_diff = flattenDesignDict(kit, design_guid)
    center_updates = {}
    for update in flatten_diff.get("pieces", {}).get("updated", []):
        uid = update.get("id")
        diff_data = update.get("diff", {})
        if uid and diff_data.get("center"):
            center_updates[uid] = diff_data["center"]
    flat_pieces = []
    for piece in design.get("pieces", []):
        flat_piece = dict(piece)
        if piece.get("guid") in center_updates:
            flat_piece["center"] = center_updates[piece["guid"]]
        elif flat_piece.get("center") is None:
            flat_piece["center"] = {"u": 0, "v": 0}
        flat_pieces.append(flat_piece)
    return {
        "pieces": flat_pieces,
        "connections": design.get("connections", []),
        "guid": design.get("guid"),
        "name": design.get("name", ""),
    }


def _generate_diagram_svg(kit: dict, design_guid: str, selected_piece_guids: list[str], selected_connection_guids: list[str], design_diff: dict | None = None) -> str:
    """Generate an SVG string representing a 2D diagram of the design.
    Pieces are circles at (u, -v), connections are lines between piece centers.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🔖mcpappsvggeneration🛠️generatediagramsvg](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/s/MCP%20App%20SVG%20Generation/d/i/_generate_diagram_svg)
    """
    flat_design = _flatten_and_get_flat_design(kit, design_guid)
    pieces = flat_design.get("pieces", [])
    connections = flat_design.get("connections", [])

    removed_piece_guids = set()
    added_piece_guids = set()
    modified_piece_guids = set()
    removed_connection_guids = set()
    added_connection_guids = set()
    modified_connection_guids = set()
    if design_diff:
        removed_piece_guids = set(p.get("guid", "") for p in design_diff.get("pieces", {}).get("removed", []))
        added_piece_guids = set(p.get("guid", "") for p in design_diff.get("pieces", {}).get("added", []))
        modified_piece_guids = set(p.get("piece", {}).get("guid", "") for p in design_diff.get("pieces", {}).get("updated", []))
        removed_connection_guids = set(c.get("guid", "") for c in design_diff.get("connections", {}).get("removed", []))
        added_connection_guids = set(c.get("guid", "") for c in design_diff.get("connections", {}).get("added", []))
        modified_connection_guids = set(c.get("connection", {}).get("guid", "") for c in design_diff.get("connections", {}).get("updated", []))

    selected_piece_set = set(selected_piece_guids)
    selected_connection_set = set(selected_connection_guids)

    type_name_map: dict[str, str] = {}
    for t in kit.get("types", []):
        if t.get("guid"):
            type_name_map[t["guid"]] = t.get("name", "")

    point_map: dict[str, dict] = {}
    for piece in pieces:
        guid = piece.get("guid", "")
        center = piece.get("center") or {"u": 0, "v": 0}
        u = center.get("u", 0) or 0
        v = center.get("v", 0) or 0
        status = "default"
        if guid in removed_piece_guids:
            status = "removed"
        elif guid in added_piece_guids:
            status = "added"
        elif guid in modified_piece_guids:
            status = "modified"
        type_guid = piece.get("type", {}).get("guid", "") if piece.get("type") else ""
        type_name = type_name_map.get(type_guid, piece.get("type", {}).get("name", "") if piece.get("type") else "")
        point_map[guid] = {"u": u, "v": v, "status": status, "name": type_name, "selected": guid in selected_piece_set}

    line_list: list[dict] = []
    for conn in connections:
        guid = conn.get("guid", "")
        source_guid = conn.get("connected", {}).get("piece", {}).get("guid", "")
        target_guid = conn.get("connecting", {}).get("piece", {}).get("guid", "")
        if source_guid not in point_map or target_guid not in point_map:
            continue
        status = "default"
        if guid in removed_connection_guids:
            status = "removed"
        elif guid in added_connection_guids:
            status = "added"
        elif guid in modified_connection_guids:
            status = "modified"
        source = point_map[source_guid]
        target = point_map[target_guid]
        line_list.append(
            {
                "guid": guid,
                "x1": source["u"],
                "y1": -source["v"],
                "x2": target["u"],
                "y2": -target["v"],
                "status": status,
                "selected": guid in selected_connection_set,
            }
        )

    points = list(point_map.values())
    if points:
        all_u = [p["u"] for p in points]
        all_y = [-p["v"] for p in points]
        min_u = min(all_u)
        max_u = max(all_u)
        min_y = min(all_y)
        max_y = max(all_y)
    else:
        min_u, max_u, min_y, max_y = -1, 1, -1, 1

    span_u = max(max_u - min_u, 1.0)
    span_y = max(max_y - min_y, 1.0)
    padding = 3.5
    piece_radius = 1.75
    stroke_width = 0.6

    vb_x = min_u - padding
    vb_y = min_y - padding
    vb_w = span_u + padding * 2
    vb_h = span_y + padding * 2

    svg_width = max(400, min(800, int(vb_w * 40)))
    svg_height = max(300, min(600, int(vb_h * 40)))

    svg_parts: list[str] = []
    svg_parts.append(f'<svg xmlns="http://www.w3.org/2000/svg" width="{svg_width}" height="{svg_height}" viewBox="{vb_x:.4f} {vb_y:.4f} {vb_w:.4f} {vb_h:.4f}" style="background:#18181b;border-radius:8px">')

    for line in line_list:
        color = _svg_status_color(line["status"], line["selected"])
        opacity = "1" if line["selected"] else ("0.8" if line["status"] != "default" else "0.45")
        sw = stroke_width + 0.4 if line["selected"] else stroke_width
        svg_parts.append(f'<line x1="{line["x1"]:.4f}" y1="{line["y1"]:.4f}" x2="{line["x2"]:.4f}" y2="{line["y2"]:.4f}" stroke="{color}" stroke-width="{sw:.2f}" stroke-opacity="{opacity}" stroke-linecap="round"/>')

    for guid, point in point_map.items():
        color = _svg_status_color(point["status"], point["selected"])
        r = piece_radius + 0.75 if point["selected"] else piece_radius
        x = point["u"]
        y = -point["v"]
        name = htmlmodule.escape(point["name"]) if point["name"] else ""
        svg_parts.append(f'<circle cx="{x:.4f}" cy="{y:.4f}" r="{r:.2f}" fill="{color}"><title>{name}</title></circle>')
        if point["selected"] and name:
            svg_parts.append(f'<text x="{x:.4f}" y="{y - r - 0.8:.4f}" text-anchor="middle" font-size="1.4" fill="#e4e4e7" font-family="system-ui">{name}</text>')

    svg_parts.append("</svg>")
    return "\n".join(svg_parts)


def _build_text_summary(mode: str, design_name: str, piece_count: int, connection_count: int, selected_piece_count: int, selected_connection_count: int) -> str:
    """Build a concise text summary for a MCP app tool response.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🔖mcpappsvggeneration🛠️buildtextsummary](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/s/MCP%20App%20SVG%20Generation/d/i/_build_text_summary)
    """
    mode_labels = {
        "show-design": "Design (2D+3D)",
        "show-diagram": "Diagram (2D)",
        "show-scene": "Scene (3D)",
        "show-diff": "Diff (2D+3D)",
        "show-diagram-diff": "Diagram Diff (2D)",
        "select-pieces": "Piece Selection",
        "select-connections": "Connection Selection",
        "select-pieces-and-connections": "Piece & Connection Selection",
    }
    label = mode_labels.get(mode, mode)
    parts = [f"**{label}**: {design_name}"]
    parts.append(f"{piece_count} pieces, {connection_count} connections")
    if selected_piece_count > 0 or selected_connection_count > 0:
        sel_parts = []
        if selected_piece_count > 0:
            sel_parts.append(f"{selected_piece_count} pieces selected")
        if selected_connection_count > 0:
            sel_parts.append(f"{selected_connection_count} connections selected")
        parts.append(", ".join(sel_parts))
    return " · ".join(parts)


# endregion MCP App SVG Generation


def _build_app_response(mode: str, ctx: Context, design_diff: dict | None = None, capabilities: dict | None = None) -> CallToolResult:
    """Build a CallToolResult with SVG diagram image, text summary, and structuredContent.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️buildappresponse](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/_build_app_response)
    """
    kit = _get_session_kit(ctx)
    sid = _session_id(ctx)
    selection = _mcp_session_selection.get(sid, {"pieceGuids": [], "connectionGuids": []})

    design_guid = None
    design_entry = _mcp_session_designs.get(sid)
    if design_entry is not None:
        design_guid = design_entry.get("guid")

    if design_guid is None:
        designs = kit.get("designs", [])
        if designs:
            design_guid = designs[0].get("guid")

    if design_guid is None:
        return CallToolResult(
            content=[TextContent(type="text", text="No design available. Call start_working_in_design first.")],
            isError=True,
        )

    caps = capabilities or {
        "pieceSelection": mode in ("select-pieces", "select-pieces-and-connections"),
        "connectionSelection": mode in ("select-connections", "select-pieces-and-connections"),
        "diff": mode in ("show-diff", "show-diagram-diff"),
    }

    selected_piece_guids = selection.get("pieceGuids", [])
    selected_connection_guids = selection.get("connectionGuids", [])

    structured_content = {
        "mode": mode,
        "designGuid": design_guid,
        "selectedPieceGuids": selected_piece_guids,
        "selectedConnectionGuids": selected_connection_guids,
        "capabilities": caps,
    }
    if design_diff is not None:
        structured_content["designDiff"] = design_diff

    svg_str = _generate_diagram_svg(kit, design_guid, selected_piece_guids, selected_connection_guids, design_diff)
    svg_b64 = base64.standard_b64encode(svg_str.encode("utf-8")).decode("ascii")

    flat_design = _flatten_and_get_flat_design(kit, design_guid)
    design_name = flat_design.get("name", design_guid)
    piece_count = len(flat_design.get("pieces", []))
    connection_count = len(flat_design.get("connections", []))

    text_summary = _build_text_summary(mode, design_name, piece_count, connection_count, len(selected_piece_guids), len(selected_connection_guids))

    content: list = [
        TextContent(type="text", text=text_summary),
        ImageContent(type="image", data=svg_b64, mimeType="image/svg+xml"),
    ]

    return CallToolResult(
        content=content,
        structuredContent=structured_content,
    )


@mcp.tool()
def show_design(ctx: Context) -> CallToolResult:
    """Show the current design in a combined 2D diagram + 3D scene split view.
    Callers MUST have called start_working_in_local_kit and start_working_in_design first.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️showdesign](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/show_design)
    """
    try:
        return _build_app_response("show-design", ctx)
    except Exception as e:
        return CallToolResult(content=[TextContent(type="text", text=str(e))], isError=True)


@mcp.tool()
def show_diagram(ctx: Context) -> CallToolResult:
    """Show the current design as a 2D diagram only.
    Callers MUST have called start_working_in_local_kit and start_working_in_design first.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️showdiagram](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/show_diagram)
    """
    try:
        return _build_app_response("show-diagram", ctx)
    except Exception as e:
        return CallToolResult(content=[TextContent(type="text", text=str(e))], isError=True)


@mcp.tool()
def show_scene(ctx: Context) -> CallToolResult:
    """Show the current design as a 3D scene only (rendered as 2D diagram fallback).
    Callers MUST have called start_working_in_local_kit and start_working_in_design first.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️showscene](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/show_scene)
    """
    try:
        return _build_app_response("show-scene", ctx)
    except Exception as e:
        return CallToolResult(content=[TextContent(type="text", text=str(e))], isError=True)


@mcp.tool()
def show_diff(ctx: Context, design_diff: dict | None = None) -> CallToolResult:
    """Show the current design diff in a combined 2D diagram + 3D scene split view.
    Callers MUST have called start_working_in_local_kit and start_working_in_design first.
    Optionally pass a design_diff dict; otherwise the current session design is shown without diff.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️showdiff](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/show_diff)
    """
    try:
        return _build_app_response("show-diff", ctx, design_diff=design_diff)
    except Exception as e:
        return CallToolResult(content=[TextContent(type="text", text=str(e))], isError=True)


@mcp.tool()
def show_diagram_diff(ctx: Context, design_diff: dict | None = None) -> CallToolResult:
    """Show the current design diff as a 2D diagram only.
    Callers MUST have called start_working_in_local_kit and start_working_in_design first.
    Optionally pass a design_diff dict; otherwise the current session design is shown without diff.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️showdiagramdiff](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/show_diagram_diff)
    """
    try:
        return _build_app_response("show-diagram-diff", ctx, design_diff=design_diff)
    except Exception as e:
        return CallToolResult(content=[TextContent(type="text", text=str(e))], isError=True)


@mcp.tool()
def select_pieces(ctx: Context) -> CallToolResult:
    """Present a piece selection interface for the current design.
    Callers MUST have called start_working_in_local_kit and start_working_in_design first.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️selectpieces](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/select_pieces)
    """
    try:
        return _build_app_response("select-pieces", ctx)
    except Exception as e:
        return CallToolResult(content=[TextContent(type="text", text=str(e))], isError=True)


@mcp.tool()
def select_connections(ctx: Context) -> CallToolResult:
    """Present a connection selection interface for the current design.
    Callers MUST have called start_working_in_local_kit and start_working_in_design first.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️selectconnections](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/select_connections)
    """
    try:
        return _build_app_response("select-connections", ctx)
    except Exception as e:
        return CallToolResult(content=[TextContent(type="text", text=str(e))], isError=True)


@mcp.tool()
def select_pieces_and_connections(ctx: Context) -> CallToolResult:
    """Present a combined piece and connection selection interface for the current design.
    Callers MUST have called start_working_in_local_kit and start_working_in_design first.
    [👤semio📚engine💻engine🔖mcp🔖mcpapptools🛠️selectpiecesandconnections](repo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20App%20Tools/d/i/select_pieces_and_connections)
    """
    try:
        return _build_app_response("select-pieces-and-connections", ctx)
    except Exception as e:
        return CallToolResult(content=[TextContent(type="text", text=str(e))], isError=True)


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
        import threading

        engine_thread = threading.Thread(target=start_engine, daemon=True)
        engine_thread.start()
        logger.debug(f"[DEBUG] Engine HTTP server started in background on {HOST}:{PORT}")
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
