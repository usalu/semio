# region Header
# [👤semio📚engine💻engine](semiorepo://p/u/semio/b/l/engine/f/engine.py)

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
# [👤semio📚engine💻engine🔖imports](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Imports)
# Imports MUST include all dependencies for store, assistant, GraphQL, REST, MCP, and engine modules.
from __future__ import annotations
import abc
import argparse
import copy
import contextlib
import datetime
import difflib
import enum
import functools
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
import sqlalchemy
import sqlalchemy.orm
import sqlmodel
import starlette.applications
import starlette_graphene3
import uvicorn
from mcp.server.fastmcp import Context, FastMCP

sys.path.insert(0, str(pathlib.Path(__file__).parent.parent / "semio"))
from semio import (
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
    Author,
    AuthorNode,
    AuthenticationError,
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
    RemoteKitUriNotValid,
    RemoteKitsNotYetSupported,
    Semio,
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
    areKitDiffsDictEqual,
    areKitsDictEqual,
    areValidationResultsEqual,
    changeKeys,
    changeToDict,
    getDesignChange,
    getKitChange,
    changeValues,
    createClusteredDesignDict,
    decode,
    encode,
    expandDesignPiecesDict,
    findAttributeValueDict,
    findReplaceableTypesForPieceInDesignDict,
    findReplaceableTypesForPiecesInDesignDict,
    findSameFamilyDesignPiecesDict,
    findUsedConnectorsByPieceInDesignDict,
    flattenDesignDict,
    getClusterableGroupsDict,
    getDesignChildrenDict,
    getDesignFamilyDict,
    getDesignSiblingsDict,
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
    validateKitDict,
    areDesignsInSameFamilyDict,
    areTypesInSameFamilyDict,
    canUseDesignAsPieceDict,
    findPieceTypeInDesignDict,
    sumQualityInDesignDict,
)

# endregion Imports

# region Store
# [👤semio📚engine💻engine🔖store](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store)
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


class OperationBuilder(lark.Transformer):
    """Lark transformer that builds operation dicts from parsed code grammar trees.
    Callers MUST pass a valid parse tree from codeParser.
    [👤semio📚engine💻engine🔖store🛠️operationbuilder](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/OperationBuilder)
    """

    def code(self, children):
        if len(children) == 0:
            return {"kind": "kits"}
        kitUri = decode(children[0].value)
        if len(children) == 1:
            return {"kind": "kit", "kitUri": kitUri}
        code = children[1]
        code["kitUri"] = kitUri
        return code

    def design(self, children):
        if len(children) == 0:
            return {"kind": "designs"}
        return {
            "kind": "design",
            "designName": decode(children[0].value),
            "designVariant": (decode(children[1].value) if len(children) == 2 else ""),
            "designView": (decode(children[2].value) if len(children) == 3 else ""),
        }

    def type(self, children):
        if len(children) == 0:
            return {"kind": "types"}
        return {
            "kind": "type",
            "typeName": decode(children[0].value),
            "typeVariant": (decode(children[1].value) if len(children) == 2 else ""),
        }


class StoreKind(enum.Enum):
    """🏪The kind of the store.
    Callers MUST use one of the defined store kinds when selecting a backend.
    [👤semio📚engine💻engine🔖store🛠️storekind](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/StoreKind)
    """

    DATABASE = "database"
    REST = "rest"
    GRAPHQL = "graphql"


class CommandKind(enum.Enum):
    """🔧 The kind of the command.
    Callers MUST use a valid CommandKind when calling Store.execute.
    [👤semio📚engine💻engine🔖store🛠️commandkind](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/CommandKind)
    """

    QUERY = "query"
    PUT = "put"
    UPDATE = "update"
    DELETE = "delete"


class Store(abc.ABC):
    """Abstract base class for all store backends.
    Subclasses MUST implement initialize, get, put, update, and delete methods.
    [👤semio📚engine💻engine🔖store🛠️store](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/Store)
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
    def get(cls: "Store", operation: dict) -> typing.Any:
        """🔍 Get an entity from the store."""
        pass

    @abc.abstractmethod
    def put(cls: "Store", operation: dict, input: str) -> typing.Any:
        """📥 Put an entity in the store."""
        pass

    @abc.abstractmethod
    def update(cls: "Store", operation: dict, input: str) -> typing.Any:
        """🔄 Update an entity in the store."""
        pass

    @abc.abstractmethod
    def delete(cls: "Store", operation: dict) -> typing.Any:
        """🗑 Delete an entity from the store."""
        pass


class DatabaseStore(Store, abc.ABC):
    """Abstract database-backed store using SQLAlchemy engine and session.
    Subclasses MUST implement the fromUri classmethod to construct from a URI.
    [👤semio📚engine💻engine🔖store🛠️databasestore](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/DatabaseStore)
    """

    engine: sqlalchemy.engine.Engine

    def __init__(self, uri: str, engine: sqlalchemy.engine.Engine) -> None:
        super().__init__(uri)
        self.engine = engine

    @functools.cached_property
    def session(self: "DatabaseStore") -> sqlalchemy.orm.Session:
        return sqlalchemy.orm.sessionmaker(bind=self.engine)()

    def initialized(self: "DatabaseStore") -> bool:
        try:
            inspector = sqlalchemy.inspect(self.engine)
            if "semio" in inspector.get_table_names():
                return True
        except sqlalchemy.exc.OperationalError:
            return False

    @classmethod
    @abc.abstractmethod
    def fromUri(cls: "DatabaseStore", uri: str) -> "DatabaseStore":
        """🔧 Get a store from the uri."""
        pass

    def postDeleteKit(self: "SqliteStore") -> None:
        return None

    def get(self: "DatabaseStore", operation: dict) -> typing.Any:
        kitUri = operation["kitUri"]
        kind = operation["kind"]
        try:
            kit = self.session.query(Kit).filter(Kit.uri == kitUri).one_or_none()
        except sqlalchemy.exc.OperationalError:
            raise KitNotFound(kitUri)
        if kit is None:
            raise KitNotFound(kitUri)
        match kind:
            case "kit":
                return kit
            case "design":
                raise FeatureNotYetSupported()
            case "type":
                raise FeatureNotYetSupported()
            case _:
                raise FeatureNotYetSupported()

    def put(
        self: "DatabaseStore",
        operation: dict,
        input: KitInput | DesignInput | TypeInput,
    ) -> typing.Any:
        kitUri = operation["kitUri"]
        kind = operation["kind"]

        if kind == "kit":
            self.initialize()
            dump = input.model_dump()
            dump["uri"] = kitUri
            kit = Kit.parse(dump)
            existingKit = self.session.query(Kit).filter(Kit.uri == kitUri).one_or_none()
            if existingKit is not None:
                raise KitAlreadyExists(kitUri)
            try:
                self.session.add(kit)
                self.session.commit()
            except Exception as e:
                self.session.rollback()
                raise e
            return kit

        if not self.initialized():
            raise KitNotFound(kitUri)
        kit = self.session.query(Kit).filter(Kit.uri == kitUri).one_or_none()
        match kind:
            case "design":
                types = [u.Type for u in self.session.query(Type, Kit).filter(Kit.uri == kitUri).all()]
                existingDesigns = [d for d, _ in self.session.query(Design, Kit).filter(Kit.uri == kitUri).all()]
                designsById: dict[str, dict[str, dict[str, Design]]] = {}
                for d in existingDesigns:
                    if d.name not in designsById:
                        designsById[d.name] = {}
                    if d.variant not in designsById[d.name]:
                        designsById[d.name][d.variant] = {}
                    designsById[d.name][d.variant][d.view] = d
                existingDesignUnion = (
                    self.session.query(Design, Kit)
                    .filter(
                        Kit.uri == kitUri,
                        Design.name == input.name,
                        Design.variant == input.variant,
                        Design.view == input.view,
                    )
                    .one_or_none()
                )
                try:
                    if existingDesignUnion is not None:
                        existingDesign = existingDesignUnion.Design
                        self.session.delete(existingDesign)
                        design = Design.parse(input, types, designsById)
                        design.kit = kit
                        self.session.add(design)
                        self.session.commit()
                    else:
                        design = Design.parse(input, types, designsById)
                        design.kit = kit
                        self.session.add(design)
                        self.session.commit()
                except Exception as e:
                    self.session.rollback()
                    raise e
            case "type":
                type = Type.parse(input)
                type.kit = kit
                existingTypeUnion = (
                    self.session.query(Type, Kit)
                    .filter(
                        Kit.uri == kitUri,
                        Type.name == type.name,
                        Type.variant == type.variant,
                    )
                    .one_or_none()
                )
                try:
                    if existingTypeUnion is not None:
                        existingType = existingTypeUnion.Type
                        existingConnectors = {p.id_: p for p in existingType.connectors}
                        usedConnectors = {}
                        for connector in list(existingType.connectors):
                            for connection in connector.connections:
                                if connection.connectedPiece.type == existingType:
                                    usedConnectors[connection.connectedConnector.id_] = connection.connectedConnector
                                if connection.connectingPiece.type == existingType:
                                    usedConnectors[connection.connectingConnector.id_] = connection.connectingConnector
                        newPorts = {p.id_: p for p in type.connectors}
                        missingConnectors = set(usedConnectors.keys()) - set(newPorts.keys())
                        if missingConnectors:
                            raise TypeHasNotAllUsedConnectors(missingConnectors)
                        unusedConnectors = set(existingConnectors.keys()) - set(usedConnectors.keys())

                        existingType.icon = type.icon
                        existingType.image = type.image
                        existingType.description = type.description
                        existingType.unit = type.unit
                        existingType.updated = datetime.datetime.now()
                        for usedConnectorId, usedConnector in usedConnectors.items():
                            usedConnector.point = newPorts[usedConnectorId].point
                            usedConnector.direction = newPorts[usedConnectorId].direction

                            for attribute in list(usedConnector.attributes):
                                self.session.delete(attribute)
                            usedConnector.attributes = []
                            self.session.flush()

                            newAttributes = []
                            for newAttribute in list(newPorts[usedConnectorId].attributes):
                                newAttribute.connector = usedConnector
                                self.session.add(newAttribute)
                                newAttributes.append(newAttribute)
                            usedConnector.attributes = newAttributes
                            self.session.flush()

                        for unusedConnector in list(unusedConnectors):
                            self.session.delete(existingConnectors[unusedConnector])
                        existingType.connectors = [p for p in existingType.connectors if p.id_ not in unusedConnectors]
                        self.session.flush()

                        for newPortId, newPort in newPorts.items():
                            if newPortId not in usedConnectors:
                                newPort.type = existingType
                                self.session.add(newPort)
                        self.session.flush()

                        existingType.models = []
                        for model in list(type.models):
                            model.type = existingType
                            self.session.add(model)
                        self.session.flush()

                        existingType.attributes = []
                        for attribute in list(type.attributes):
                            attribute.type = existingType
                            self.session.add(attribute)
                        self.session.flush()

                        existingType.authors = []
                        for author in list(type.authors):
                            author.type = existingType
                            self.session.add(author)
                        self.session.flush()

                        self.session.commit()
                    else:
                        self.session.add(type)
                        self.session.commit()
                except Exception as e:
                    self.session.rollback()
                    raise e
                return type
            case _:
                raise FeatureNotYetSupported()

    def update(self: "DatabaseStore", operation: dict, input: str) -> typing.Any:
        raise FeatureNotYetSupported()

    def delete(self: "DatabaseStore", operation: dict) -> typing.Any:
        kitUri = operation["kitUri"]
        kind = operation["kind"]
        try:
            kit = self.session.query(Kit).filter(Kit.uri == kitUri).one_or_none()
        except sqlalchemy.exc.OperationalError:
            raise KitNotFound(kitUri)
        if kit is None:
            raise KitNotFound(kitUri)
        match kind:
            case "kit":
                try:
                    self.session.delete(kit)
                    self.session.commit()
                except Exception as e:
                    self.session.rollback()
                    raise e
            case "design":
                try:
                    self.session.query(Design, Kit).filter(
                        Kit.uri == kitUri,
                        Design.name == operation["designName"],
                        Design.variant == operation["designVariant"],
                        Design.view == operation["designView"],
                    ).delete()
                    self.session.commit()
                except Exception as e:
                    self.session.rollback()
                    raise e
            case "type":
                try:
                    self.session.query(Type, Kit).filter(
                        Kit.uri == kitUri,
                        Type.name == operation["typeName"],
                        Type.variant == operation["typeVariant"],
                    ).delete()
                    self.session.commit()
                except Exception as e:
                    self.session.rollback()
                    raise e
            case _:
                raise FeatureNotYetSupported()


class SSLMode(enum.Enum):
    """🔒 The security level of the session
    Callers MUST select the appropriate SSL mode for the target database security policy.
    [👤semio📚engine💻engine🔖store🛠️sslmode](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/SSLMode)
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
    [👤semio📚engine💻engine🔖store🛠️cachedir](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/cacheDir)
    """
    cacheDir = os.path.expanduser("~/.semio/cache")
    encodedUri = encode(remoteUri)
    return os.path.join(cacheDir, encodedUri)


def cache(remoteUri: str) -> str:
    """📦Cache a remote kit and delete the existing cache if it was already cached.
    Callers MUST provide a URI starting with http and ending with .zip.
    [👤semio📚engine💻engine🔖store🛠️cache](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/cache)
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
    """SQLite-backed store that persists kit data to a local .semio database file.
    Callers MUST use fromUri to construct instances with a valid local path.
    [👤semio📚engine💻engine🔖store🛠️sqlitestore](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/SqliteStore)
    """

    path: pathlib.Path

    def __init__(self, uri: str, engine: sqlalchemy.engine.Engine, path: pathlib.Path) -> None:
        super().__init__(uri, engine)
        self.path = path

    @classmethod
    def fromUri(cls, uri: str, path: str = "") -> "SqliteStore":
        if path == "":
            path = uri
        sqlitePath = pathlib.Path(path) / pathlib.Path(KIT_LOCAL_FOLDERNAME) / pathlib.Path(KIT_LOCAL_FILENAME)
        connectionString = f"sqlite:///{sqlitePath}"
        engine = sqlalchemy.create_engine(connectionString, echo=True)
        SessionMaker = sqlalchemy.orm.sessionmaker(bind=engine)
        try:
            with SessionMaker() as session:
                kit = session.query(Kit).first()
                if kit:
                    kit.uri = uri
                    session.commit()
        except sqlalchemy.exc.OperationalError:
            pass
        return SqliteStore(uri, engine, sqlitePath)

    def initialize(self: "DatabaseStore") -> None:
        os.makedirs(
            str(pathlib.Path(self.uri) / pathlib.Path(KIT_LOCAL_FOLDERNAME)),
            exist_ok=True,
        )
        sqlmodel.SQLModel.metadata.create_all(self.engine)
        SessionMaker = sqlalchemy.orm.sessionmaker(bind=self.engine)
        with SessionMaker() as session:
            existingSemio = session.query(Semio).one_or_none()
            if not existingSemio:
                session.add(Semio())
                session.commit()

    def postDeleteKit(self: "SqliteStore") -> None:
        os.kill(os.getpid(), signal.SIGTERM)


class PostgresStore(DatabaseStore):
    """PostgreSQL-backed store for remote database connections.
    Callers MUST NOT use this class until PostgreSQL support is implemented.
    [👤semio📚engine💻engine🔖store🛠️postgresstore](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/PostgresStore)
    """

    @classmethod
    def fromUri(cls, uri: str):
        # TODO: Get connection string from environment variable.

        raise FeatureNotYetSupported()

    def initialize(self: "DatabaseStore") -> None:
        sqlmodel.SQLModel.metadata.create_all(self.engine)


# region Auth
# [👤semio📚engine💻engine🔖store🔖auth](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth)
# Auth MUST provide credential management for remote server authentication using Bearer tokens.

AUTH_FILE = os.path.join(os.path.expanduser(USER_FOLDER), "auth.json")


def _load_auth() -> dict:
    """Load auth credentials from the auth file.
    Returns dict mapping serverUrl -> {token, email}.
    [👤semio📚engine💻engine🔖store🔖auth🛠️loadauth](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/_load_auth)
    """
    if os.path.exists(AUTH_FILE):
        with open(AUTH_FILE, "r", encoding="utf-8") as f:
            return json.load(f)
    return {}


def _save_auth(auth: dict) -> None:
    """Save auth credentials to the auth file.
    Callers MUST provide a dict mapping serverUrl -> {token, email}.
    [👤semio📚engine💻engine🔖store🔖auth🛠️saveauth](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/_save_auth)
    """
    os.makedirs(os.path.dirname(AUTH_FILE), exist_ok=True)
    with open(AUTH_FILE, "w", encoding="utf-8") as f:
        json.dump(auth, f, indent=2)


def login(serverUrl: str, email: str, password: str) -> dict:
    """🔐 Login to a remote server and store the auth token.
    Callers MUST provide a valid server URL, email and password.
    Returns {ok, serverUrl, email, token} on success.
    [👤semio📚engine💻engine🔖store🔖auth🛠️login](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/login)
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
    [👤semio📚engine💻engine🔖store🔖auth🛠️logout](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/logout)
    """
    serverUrl = serverUrl.rstrip("/")
    auth = _load_auth()
    auth.pop(serverUrl, None)
    _save_auth(auth)
    return {"ok": True, "serverUrl": serverUrl}


def getAuthToken(serverUrl: str) -> str:
    """🔑 Get the stored auth token for a server.
    Raises AuthTokenNotFound if no token is stored.
    [👤semio📚engine💻engine🔖store🔖auth🛠️getauthtoken](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/getAuthToken)
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
    [👤semio📚engine💻engine🔖store🔖auth🛠️getauthstatus](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/s/Auth/d/i/getAuthStatus)
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
    [👤semio📚engine💻engine🔖store🛠️remotestore](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/RemoteStore)
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
        encodedKitUri = uri[idx + len("/api/kits/"):]
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

    def get(self, operation: dict) -> typing.Any:
        """🔍 Get an entity from the remote store."""
        kind = operation["kind"]
        try:
            if kind == "kit":
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

    def put(self, operation: dict, input: KitInput | DesignInput | TypeInput) -> typing.Any:
        """📥 Put an entity in the remote store."""
        kind = operation["kind"]
        try:
            if kind == "kit":
                response = requests.put(
                    self._api_url(),
                    json=input.model_dump() if hasattr(input, "model_dump") else input,
                    headers=self._headers(),
                    timeout=30,
                )
                response.raise_for_status()
                return None
            elif kind == "type":
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
            elif kind == "design":
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

    def update(self, operation: dict, input: str) -> typing.Any:
        """🔄 Update an entity in the remote store."""
        raise FeatureNotYetSupported()

    def delete(self, operation: dict) -> typing.Any:
        """🗑 Delete an entity from the remote store."""
        kind = operation["kind"]
        try:
            if kind == "kit":
                response = requests.delete(self._api_url(), headers=self._headers(), timeout=30)
                response.raise_for_status()
                return None
            elif kind == "type":
                typeName = encode(operation.get("typeName", ""))
                typeVariant = encode(operation.get("typeVariant", ""))
                path = f"types/{typeName},{typeVariant}"
                response = requests.delete(self._api_url(path), headers=self._headers(), timeout=30)
                response.raise_for_status()
                return None
            elif kind == "design":
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
    [👤semio📚engine💻engine🔖store🛠️storefactory](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/StoreFactory)
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
    [👤semio📚engine💻engine🔖store🛠️storeandoperationfromcode](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/storeAndOperationFromCode)
    """
    codeTree = codeParser.parse(code)
    operation = OperationBuilder().transform(codeTree)
    store = StoreFactory(operation["kitUri"])
    return store, operation


def get(code: str, cache=False) -> typing.Any:
    """🔍 Get an entity from the store.
    Callers MUST provide a valid code string with an encoded kit URI.
    [👤semio📚engine💻engine🔖store🛠️get](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/get)
    """
    store, operation = storeAndOperationFromCode(code)
    return store.get(operation)


def put(code: str, input: str) -> typing.Any:
    """📥 Put an entity in the store.
    Callers MUST provide a valid code string and matching input data.
    [👤semio📚engine💻engine🔖store🛠️put](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/put)
    """
    store, operation = storeAndOperationFromCode(code)
    return store.put(operation, input)


def delete(code: str) -> typing.Any:
    """🗑 Delete an entity from the store.
    Callers MUST provide a valid code string referencing an existing entity.
    [👤semio📚engine💻engine🔖store🛠️delete](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Store/d/i/delete)
    """
    store, operation = storeAndOperationFromCode(code)
    return store.delete(operation)


# endregion Store

# region Assistant
# [👤semio📚engine💻engine🔖assistant](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Assistant)
# Assistant MUST provide AI-powered design prediction using OpenAI structured outputs.


def encodeForPrompt(context: str):
    """Sanitizes a context string for use in AI prompts by replacing delimiters.
    Callers MUST pass a string that will be embedded in a prompt template.
    [👤semio📚engine💻engine🔖assistant🛠️encodeforprompt](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/encodeForPrompt)
    """
    return context.replace(";", ",").replace("\n", " ")


def replaceDefault(context: str, default: str):
    """Substitutes an empty context string with the provided default value.
    Callers MUST provide a non-None default string.
    [👤semio📚engine💻engine🔖assistant🛠️replacedefault](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/replaceDefault)
    """
    if context == "":
        return context.replace("", default)
    return context


def encodeType(type: TypeContext):
    """Encodes a TypeContext for prompt rendering by replacing empty values with defaults.
    Callers MUST provide a valid TypeContext with populated connectors.
    [👤semio📚engine💻engine🔖assistant🛠️encodetype](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/encodeType)
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
    [👤semio📚engine💻engine🔖assistant🛠️decodedesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/decodeDesign)
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
    [👤semio📚engine💻engine🔖assistant🛠️healdesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/healDesign)
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
    [👤semio📚engine💻engine🔖assistant🛠️predictdesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Assistant/d/i/predictDesign)
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
# [👤semio📚engine💻engine🔖graphql](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Graphql)
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
    "list[engine.Attribute]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AttributeNode))),
    "Coord": graphene.NonNull(lambda: CoordNode),
    "typing.Optional[__main__.Coord]": lambda: CoordNode,
    "typing.Optional[__mp_main__.Coord]": lambda: CoordNode,
    "typing.Optional[engine.Coord]": lambda: CoordNode,
    "Location": graphene.NonNull(lambda: LocationNode),
    "typing.Optional[__main__.Location]": lambda: LocationNode,
    "typing.Optional[__mp_main__.Location]": lambda: LocationNode,
    "typing.Optional[engine.Location]": lambda: LocationNode,
    "Point": graphene.NonNull(lambda: PointNode),
    "Vector": graphene.NonNull(lambda: VectorNode),
    "Plane": graphene.NonNull(lambda: PlaneNode),
    "Connector": graphene.NonNull(lambda: ConnectorNode),
    "ConnectorId": graphene.NonNull(lambda: ConnectorNode),
    "list[Connector]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectorNode))),
    "list[__main__.Connector]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectorNode))),
    "list[__mp_main__.Connector]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectorNode))),
    "list[engine.Connector]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectorNode))),
    "Model": graphene.NonNull(lambda: ModelNode),
    "list[Model]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ModelNode))),
    "list[__main__.Model]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ModelNode))),
    "list[__mp_main__.Model]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ModelNode))),
    "list[engine.Model]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ModelNode))),
    "Author": graphene.NonNull(lambda: AuthorNode),
    "list[Author]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AuthorNode))),
    "list[__main__.Author]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AuthorNode))),
    "list[__mp_main__.Author]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AuthorNode))),
    "list[engine.Author]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: AuthorNode))),
    "Type": graphene.NonNull(lambda: TypeNode),
    "TypeId": graphene.NonNull(lambda: TypeNode),
    "DesignId": graphene.NonNull(lambda: DesignNode),
    "list[Type]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    "list[__main__.Type]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    "list[__mp_main__.Type]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    "list[engine.Type]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    "Piece": graphene.NonNull(lambda: PieceNode),
    "PieceId": graphene.NonNull(lambda: PieceNode),
    "typing.Optional[__main__.PieceId]": lambda: PieceNode,
    "typing.Optional[__mp_main__.PieceId]": lambda: PieceNode,
    "typing.Optional[engine.PieceId]": lambda: PieceNode,
    "typing.Optional[__main__.DesignId]": lambda: DesignNode,
    "typing.Optional[__mp_main__.DesignId]": lambda: DesignNode,
    "typing.Optional[engine.DesignId]": lambda: DesignNode,
    "Side": graphene.NonNull(lambda: SideNode),
    "Connection": graphene.NonNull(lambda: ConnectionNode),
    "list['Connection']": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectionNode))),
    "list[__main__.Connection]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectionNode))),
    "list[__mp_main__.Connection]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectionNode))),
    "list[engine.Connection]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: ConnectionNode))),
    "Design": graphene.NonNull(lambda: DesignNode),
    "Kit": graphene.NonNull(lambda: KitNode),
}


class Query(graphene.ObjectType):
    """GraphQL root query type exposing kit retrieval by URI.
    Callers MUST provide a valid URI when resolving kit queries.
    [👤semio📚engine💻engine🔖graphql🛠️query](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Graphql/d/i/Query)
    """

    node = RelayNode.Field()
    kit = graphene.Field(KitNode, uri=graphene.String(required=True))

    def resolve_kit(self, info, uri):
        return get(encode(uri))


class Mutation(graphene.ObjectType):
    """GraphQL root mutation type exposing kit creation.
    Callers MUST provide a valid KitInput when creating kits.
    [👤semio📚engine💻engine🔖graphql🛠️mutation](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Graphql/d/i/Mutation)
    """

    createKit = graphene.Field(KitNode, kit=KitInputNode(required=True))


graphqlSchema = graphene.Schema(
    query=Query,
    mutation=Mutation,
)

# endregion Graphql

# region Rest
# [👤semio📚engine💻engine🔖rest](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest)
# Rest MUST expose kit, type, design, and assistant endpoints via FastAPI.

rest = fastapi.FastAPI(max_request_body_size=MAX_REQUEST_BODY_SIZE)


@rest.get("/kits/{encodedKitUri}")
async def kit(
    request: fastapi.Request,
    encodedKitUri: ENCODED_PATH,
) -> KitOutput:
    """Retrieves a kit by its encoded URI path.
    Callers MUST provide a valid encoded kit URI in the URL path.
    [👤semio📚engine💻engine🔖rest🛠️kit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/kit)
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
    [👤semio📚engine💻engine🔖rest🛠️createkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/create_kit)
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
    [👤semio📚engine💻engine🔖rest🛠️deletekit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/delete_kit)
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
    [👤semio📚engine💻engine🔖rest🛠️puttype](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/put_type)
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
    [👤semio📚engine💻engine🔖rest🛠️deletetype](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/delete_type)
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
    [👤semio📚engine💻engine🔖rest🛠️putdesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/put_design)
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
    [👤semio📚engine💻engine🔖rest🛠️deletedesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/delete_design)
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
    [👤semio📚engine💻engine🔖rest🛠️predictdesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/predict_design)
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
    [👤semio📚engine💻engine🔖rest🛠️preparekit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/prepare_kit)
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
    [👤semio📚engine💻engine🔖rest🛠️contextgeneratejsonschema](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/ContextGenerateJsonSchema)
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
    [👤semio📚engine💻engine🔖rest🛠️outputgeneratejsonschema](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/OutputGenerateJsonSchema)
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
    [👤semio📚engine💻engine🔖rest🛠️predictiongeneratejsonschema](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/PredictionGenerateJsonSchema)
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
    [👤semio📚engine💻engine🔖rest🛠️customopenapi](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/d/i/custom_openapi)
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
# [👤semio📚engine💻engine🔖rest🔖authendpoints](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints)
# Auth endpoints MUST expose login, logout and status for remote server authentication.


class LoginRequest(pydantic.BaseModel):
    """Login request body.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️loginrequest](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/LoginRequest)
    """
    serverUrl: str
    email: str
    password: str


class LoginResponse(pydantic.BaseModel):
    """Login response body.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️loginresponse](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/LoginResponse)
    """
    ok: bool
    serverUrl: str
    email: str
    token: str


class LogoutRequest(pydantic.BaseModel):
    """Logout request body.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️logoutrequest](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/LogoutRequest)
    """
    serverUrl: str


class AuthStatusResponse(pydantic.BaseModel):
    """Auth status response body.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️authstatusresponse](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/AuthStatusResponse)
    """
    authenticated: bool
    serverUrl: str
    email: str


@rest.post("/auth/login")
async def rest_login(request: LoginRequest) -> LoginResponse:
    """Login to a remote server and store the auth token.
    Callers MUST provide serverUrl, email and password.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️restlogin](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/rest_login)
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
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️restlogout](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/rest_logout)
    """
    try:
        return logout(request.serverUrl)
    except Exception as e:
        return fastapi.Response(content=str(e), status_code=500)


@rest.get("/auth/status")
async def rest_auth_status(serverUrl: str) -> AuthStatusResponse:
    """Get the auth status for a remote server.
    Callers MUST provide serverUrl as a query parameter.
    [👤semio📚engine💻engine🔖rest🔖authendpoints🛠️restauthstatus](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Rest/s/Auth%20Endpoints/d/i/rest_auth_status)
    """
    try:
        result = getAuthStatus(serverUrl)
        return AuthStatusResponse(**result)
    except Exception as e:
        return fastapi.Response(content=str(e), status_code=500)


# endregion Auth Endpoints

# endregion Rest

# region Mcp
# [👤semio📚engine💻engine🔖mcp](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp)
# Mcp MUST expose stateful kit operations via Model Context Protocol.
# Call start_working_in_local_kit(path) first; then use start_working_in_design/start_working_in_type to scope further.

mcp = FastMCP("semio", stateless_http=False, json_response=True)

# Session-scoped state. Keyed by session id for isolation.
_mcp_session_kits: dict[int, dict] = {}
_mcp_session_designs: dict[int, dict] = {}
_mcp_session_types: dict[int, dict] = {}
_mcp_session_kit_mode: dict[int, str] = {}  # "local" or "remote"
_mcp_session_kit_source: dict[int, str] = {}  # path or serverUrl+kitUri
_mcp_session_transactions: dict[int, dict] = {}
_mcp_session_transaction_rollback: set[int] = set()


def _load_kit_from_remote(serverUrl: str, kitUri: str) -> dict:
    """Load kit dict from a remote server via REST API.
    Callers MUST have called login() first to authenticate with the server.
    [👤semio📚engine💻engine🔖mcp🛠️loadkitfromremote](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/_load_kit_from_remote)
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
    """Load kit dict from path (JSON file or folder with .semio/kit.sqlite3 or kit JSON).
    [👤semio📚engine💻engine🔖mcp🛠️loadkitfrompath](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/_load_kit_from_path)
    """
    p = pathlib.Path(path).resolve()
    if p.is_file() and p.suffix == ".json":
        with open(p, "r", encoding="utf-8") as f:
            return json.load(f)
    if p.is_dir():
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


def _session_id(ctx) -> int:
    """Get session id from context."""
    return id(ctx.session) if ctx and hasattr(ctx, "session") else None


def _get_session_kit(ctx) -> dict:
    """Get kit from session. Raises if start_working_in_local_kit or start_working_in_remote_kit was not called."""
    sid = _session_id(ctx)
    if sid is None or sid not in _mcp_session_kits:
        raise ValueError("Call start_working_in_local_kit(path) or start_working_in_remote_kit(serverUrl, kitUri) first to set the kit for this session.")
    return _mcp_session_kits[sid]


def _get_session_kit_mode(ctx) -> str:
    """Get kit mode from session. Returns 'local' or 'remote'."""
    sid = _session_id(ctx)
    return _mcp_session_kit_mode.get(sid, "local")


def _get_session_design(ctx) -> dict:
    """Get current design from session. Raises if start_working_in_design was not called."""
    sid = _session_id(ctx)
    if sid is None or sid not in _mcp_session_designs:
        raise ValueError("Call start_working_in_design(guid) first to set the design for this session.")
    return _mcp_session_designs[sid]


def _get_session_type(ctx) -> dict:
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


def _get_active_transaction(sid: int | None) -> dict | None:
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
        {
            "kind": "kit_change",
            "before_has_kit": before is not None,
            "after_has_kit": after is not None,
            "forward_diff": forward_diff,
            "backward_diff": backward_diff,
        }
    )


def _set_session_kit(ctx, kit: dict):
    """Set session kit and record the change if a transaction is active."""
    sid = _session_id(ctx)
    before = _mcp_session_kits.get(sid)
    _record_transaction_kit_change(sid, before, kit)
    _mcp_session_kits[sid] = kit


def _clear_session_kit(ctx):
    """Clear session kit and record the change if a transaction is active."""
    sid = _session_id(ctx)
    before = _mcp_session_kits.get(sid)
    _record_transaction_kit_change(sid, before, None)
    _mcp_session_kits.pop(sid, None)


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


@mcp.tool()
def start_working_in_local_kit(path: str, ctx: Context) -> dict:
    """Start working in a local kit for this MCP session. MUST be called first.
    Path: absolute path to kit folder (with .semio/kit.sqlite3) or JSON file, or folder containing kit_metabolism.json.
    [👤semio📚engine💻engine🔖mcp🛠️startworkinginlocalkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/start_working_in_local_kit)
    """
    try:
        kit = _load_kit_from_path(path)
        sid = _session_id(ctx)
        _set_session_kit(ctx, kit)
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        _mcp_session_kit_mode[sid] = "local"
        _mcp_session_kit_source[sid] = path
        return {"ok": True, "mode": "local", "path": path}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_working_in_remote_kit(serverUrl: str, kitUri: str, ctx: Context) -> dict:
    """Start working in a remote kit for this MCP session. MUST be called first.
    Requires prior login() to the server. Fetches the kit from the remote server.
    [👤semio📚engine💻engine🔖mcp🛠️startworkinginremotekit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/start_working_in_remote_kit)
    """
    try:
        kit = _load_kit_from_remote(serverUrl, kitUri)
        sid = _session_id(ctx)
        _set_session_kit(ctx, kit)
        _mcp_session_designs.pop(sid, None)
        _mcp_session_types.pop(sid, None)
        _mcp_session_kit_mode[sid] = "remote"
        _mcp_session_kit_source[sid] = f"{serverUrl}/api/kits/{encode(kitUri)}"
        return {"ok": True, "mode": "remote", "serverUrl": serverUrl, "kitUri": kitUri}
    except Exception as e:
        return {"error": str(e)}


# region MCP Auth Tools
# [👤semio📚engine💻engine🔖mcp🔖mcpauthtools](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Auth%20Tools)
# MCP Auth Tools MUST expose login, logout and status for remote server authentication.

def mcp_login(serverUrl: str, email: str, password: str) -> dict:
    """🔐 Login to a remote semio server. Stores the auth token for subsequent remote kit operations.
    [👤semio📚engine💻engine🔖mcp🔖mcpauthtools🛠️mcplogin](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Auth%20Tools/d/i/mcp_login)
    """
    try:
        return login(serverUrl, email, password)
    except Exception as e:
        return {"error": str(e)}


def mcp_logout(serverUrl: str) -> dict:
    """🔓 Logout from a remote semio server. Removes the stored token.
    [👤semio📚engine💻engine🔖mcp🔖mcpauthtools🛠️mcplogout](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Auth%20Tools/d/i/mcp_logout)
    """
    try:
        return logout(serverUrl)
    except Exception as e:
        return {"error": str(e)}


def mcp_auth_status(serverUrl: str) -> dict:
    """📋 Get the auth status for a remote semio server.
    [👤semio📚engine💻engine🔖mcp🔖mcpauthtools🛠️mcpauthstatus](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/s/MCP%20Auth%20Tools/d/i/mcp_auth_status)
    """
    try:
        return getAuthStatus(serverUrl)
    except Exception as e:
        return {"error": str(e)}

# endregion MCP Auth Tools


def validate_kit(kit: dict) -> dict:
    """Validate a kit and return any validation problems.
    Callers MUST provide a dict matching the Kit schema.
    [👤semio📚engine💻engine🔖mcp🛠️validatekit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/validate_kit)
    """
    try:
        result = validateKitDict(kit)
        return result.model_dump() if hasattr(result, "model_dump") else {"problems": []}
    except Exception as e:
        return {"error": str(e)}


def flatten_design(kit: dict, design_guid: str) -> dict:
    """Flatten a design by computing absolute planes for all pieces.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️flattendesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/flatten_design)
    """
    try:
        return flattenDesignDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_kit_diff(before: dict, after: dict) -> dict:
    """Get the diff between two kit states.
    Callers MUST provide two valid kit dicts for comparison.
    [👤semio📚engine💻engine🔖mcp🛠️getkitdiff](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_kit_diff)
    """
    try:
        return getKitDiffDict(before, after)
    except Exception as e:
        return {"error": str(e)}


def apply_kit_diff(base: dict, diff: dict) -> dict:
    """Apply a diff to a kit.
    Callers MUST provide a valid base kit dict and a compatible diff dict.
    [👤semio📚engine💻engine🔖mcp🛠️applykitdiff](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/apply_kit_diff)
    """
    try:
        return applyKitDiffDict(base, diff)
    except Exception as e:
        return {"error": str(e)}


def inverse_kit_diff(original: dict, applied_diff: dict) -> dict:
    """Get the inverse of a diff (for undo operations).
    Callers MUST provide the original kit dict and the applied diff dict.
    [👤semio📚engine💻engine🔖mcp🛠️inversekitdiff](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/inverse_kit_diff)
    """
    try:
        return inverseKitDiffDict(original, applied_diff)
    except Exception as e:
        return {"error": str(e)}


def get_kit_change(before: dict, after: dict) -> dict:
    """Get the change (forward and backward diffs) between two kit states for undo/redo.
    Callers MUST provide two valid kit dicts for comparison.
    [👤semio📚engine💻engine🔖mcp🛠️getkitchange](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_kit_change)
    """
    try:
        return changeToDict(getKitChange(before, after))
    except Exception as e:
        return {"error": str(e)}


def get_design_change(before: dict, after: dict) -> dict:
    """Get the change (forward and backward diffs) between two design states for undo/redo.
    Callers MUST provide two valid design dicts for comparison.
    [👤semio📚engine💻engine🔖mcp🛠️getdesignchange](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_design_change)
    """
    try:
        return changeToDict(getDesignChange(before, after))
    except Exception as e:
        return {"error": str(e)}


def pieces_metadata(kit: dict, design_guid: str) -> dict:
    """Get metadata for all pieces in a design (plane, center, fixedPieceId, parentPieceId, depth).
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️piecesmetadata](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/pieces_metadata)
    """
    try:
        return piecesMetadataDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_primitive_design(kit: dict, design_guid: str) -> dict:
    """Get the root/primitive design of a design family.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️getprimitivedesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_primitive_design)
    """
    try:
        return getPrimitiveDesignDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_design_family(kit: dict, design_guid: str) -> list:
    """Get all designs in a design family tree.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️getdesignfamily](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_design_family)
    """
    try:
        return getDesignFamilyDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_design_siblings(kit: dict, design_guid: str) -> list:
    """Get all sibling designs (same parent, excluding self).
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️getdesignsiblings](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_design_siblings)
    """
    try:
        return getDesignSiblingsDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_design_children(kit: dict, design_guid: str) -> list:
    """Get all direct children of a design.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️getdesignchildren](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_design_children)
    """
    try:
        return getDesignChildrenDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def are_designs_in_same_family(kit: dict, design_guid_a: str, design_guid_b: str) -> dict:
    """Check if two designs belong to the same family.
    Callers MUST provide a valid kit dict and two existing design GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️aredesignsinsamefamily](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/are_designs_in_same_family)
    """
    try:
        return {"result": areDesignsInSameFamilyDict(kit, design_guid_a, design_guid_b)}
    except Exception as e:
        return {"error": str(e)}


def can_use_design_as_piece(kit: dict, container_design_guid: str, piece_design_guid: str) -> dict:
    """Check if a design can be used as a piece in another design.
    Callers MUST provide a valid kit dict and two existing design GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️canusedesignaspiece](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/can_use_design_as_piece)
    """
    try:
        return {"result": canUseDesignAsPieceDict(kit, container_design_guid, piece_design_guid)}
    except Exception as e:
        return {"error": str(e)}


def find_same_family_design_pieces(kit: dict, design_guid: str) -> list:
    """Find pieces in a design that reference designs from the same family.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️findsamefamilydesignpieces](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_same_family_design_pieces)
    """
    try:
        return findSameFamilyDesignPiecesDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


def get_primitive_type(kit: dict, type_guid: str) -> dict:
    """Get the root/primitive type of a type family.
    Callers MUST provide a valid kit dict and an existing type GUID.
    [👤semio📚engine💻engine🔖mcp🛠️getprimitivetype](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_primitive_type)
    """
    try:
        return getPrimitiveTypeDict(kit, type_guid)
    except Exception as e:
        return {"error": str(e)}


def get_type_family(kit: dict, type_guid: str) -> list:
    """Get all types in a type family tree.
    Callers MUST provide a valid kit dict and an existing type GUID.
    [👤semio📚engine💻engine🔖mcp🛠️gettypefamily](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_type_family)
    """
    try:
        return getTypeFamilyDict(kit, type_guid)
    except Exception as e:
        return {"error": str(e)}


def get_type_siblings(kit: dict, type_guid: str) -> list:
    """Get all sibling types (same parent, excluding self).
    Callers MUST provide a valid kit dict and an existing type GUID.
    [👤semio📚engine💻engine🔖mcp🛠️gettypesiblings](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_type_siblings)
    """
    try:
        return getTypeSiblingsDict(kit, type_guid)
    except Exception as e:
        return {"error": str(e)}


def get_type_children(kit: dict, type_guid: str) -> list:
    """Get all direct children of a type.
    Callers MUST provide a valid kit dict and an existing type GUID.
    [👤semio📚engine💻engine🔖mcp🛠️gettypechildren](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_type_children)
    """
    try:
        return getTypeChildrenDict(kit, type_guid)
    except Exception as e:
        return {"error": str(e)}


def are_types_in_same_family(kit: dict, type_guid_a: str, type_guid_b: str) -> dict:
    """Check if two types belong to the same family.
    Callers MUST provide a valid kit dict and two existing type GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️aretypesinsamefamily](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/are_types_in_same_family)
    """
    try:
        return {"result": areTypesInSameFamilyDict(kit, type_guid_a, type_guid_b)}
    except Exception as e:
        return {"error": str(e)}


def find_piece_type_in_design(kit: dict, design_guid: str, piece_guid: str) -> dict:
    """Get the type of a piece in a design.
    Callers MUST provide a valid kit dict, design GUID, and piece GUID.
    [👤semio📚engine💻engine🔖mcp🛠️findpiecetypeindesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_piece_type_in_design)
    """
    try:
        return findPieceTypeInDesignDict(kit, design_guid, piece_guid)
    except Exception as e:
        return {"error": str(e)}


def find_used_connectors_by_piece_in_design(kit: dict, design_guid: str, piece_guid: str) -> list:
    """Get all connectors of a piece that are used in connections.
    Callers MUST provide a valid kit dict, design GUID, and piece GUID.
    [👤semio📚engine💻engine🔖mcp🛠️findusedconnectorsbypieceindesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_used_connectors_by_piece_in_design)
    """
    try:
        return findUsedConnectorsByPieceInDesignDict(kit, design_guid, piece_guid)
    except Exception as e:
        return {"error": str(e)}


def find_replaceable_types_for_piece_in_design(kit: dict, design_guid: str, piece_guid: str, variants: list[str] = None) -> list:
    """Find all types that can replace a piece while maintaining connection compatibility.
    Callers MUST provide a valid kit dict, design GUID, and piece GUID. Optionally filter by variant parent GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️findreplaceabletypesforpieceindesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_replaceable_types_for_piece_in_design)
    """
    try:
        return findReplaceableTypesForPieceInDesignDict(kit, design_guid, piece_guid, variants)
    except Exception as e:
        return {"error": str(e)}


def find_replaceable_types_for_pieces_in_design(kit: dict, design_guid: str, piece_guids: list[str], variants: list[str] = None) -> list:
    """Find types that can replace multiple pieces while maintaining all external connections.
    Callers MUST provide a valid kit dict, design GUID, and list of piece GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️findreplaceabletypesforpiecesindesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_replaceable_types_for_pieces_in_design)
    """
    try:
        return findReplaceableTypesForPiecesInDesignDict(kit, design_guid, piece_guids, variants)
    except Exception as e:
        return {"error": str(e)}


def create_clustered_design(original_design: dict, cluster_piece_ids: list[str], design_name: str) -> dict:
    """Create a new design from a subset of pieces (cluster).
    Returns clusteredDesign and externalConnections.
    Callers MUST provide a valid design dict, list of piece GUIDs, and a name for the new design.
    [👤semio📚engine💻engine🔖mcp🛠️createclustereddesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/create_clustered_design)
    """
    try:
        return createClusteredDesignDict(original_design, cluster_piece_ids, design_name)
    except Exception as e:
        return {"error": str(e)}


def replace_cluster_with_design(original_design: dict, cluster_piece_ids: list[str], clustered_design: dict, external_connections: list[dict]) -> dict:
    """Get a DesignDiff that replaces clustered pieces with a design reference.
    Callers MUST provide the original design, cluster piece IDs, the new clustered design, and external connections.
    [👤semio📚engine💻engine🔖mcp🛠️replaceclusterwithdesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/replace_cluster_with_design)
    """
    try:
        return replaceClusterWithDesignDict(original_design, cluster_piece_ids, clustered_design, external_connections)
    except Exception as e:
        return {"error": str(e)}


def get_clusterable_groups(design: dict, selected_piece_ids: list[str]) -> list:
    """Get clusterable groups of selected pieces.
    Callers MUST provide a valid design dict and list of selected piece GUIDs.
    [👤semio📚engine💻engine🔖mcp🛠️getclusterablegroups](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_clusterable_groups)
    """
    try:
        return getClusterableGroupsDict(design, selected_piece_ids)
    except Exception as e:
        return {"error": str(e)}


def expand_design_pieces(design: dict, kit: dict) -> dict:
    """Recursively expand design references by inlining their pieces and connections.
    Callers MUST provide a valid design dict and kit dict.
    [👤semio📚engine💻engine🔖mcp🛠️expanddesignpieces](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/expand_design_pieces)
    """
    try:
        return expandDesignPiecesDict(design, kit)
    except Exception as e:
        return {"error": str(e)}


def find_attribute_value(entity: dict, name: str, default_value: str = None) -> dict:
    """Find an attribute value on an entity by key.
    Callers MUST provide an entity dict (kit, type, design, piece, etc.) and attribute key name.
    [👤semio📚engine💻engine🔖mcp🛠️findattributevalue](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/find_attribute_value)
    """
    try:
        sentinel = ... if default_value is None else default_value
        result = findAttributeValueDict(entity, name, sentinel)
        return {"value": result}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_working_in_design(guid: str, ctx: Context) -> dict:
    """Start working in a design within the current kit.
    Callers MUST have called start_working_in_local_kit first. Selects the design by GUID.
    [👤semio📚engine💻engine🔖mcp🛠️startworkingindesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/start_working_in_design)
    """
    try:
        kit = _get_session_kit(ctx)
        design = next((d for d in kit.get("designs", []) if d.get("guid") == guid), None)
        if design is None:
            return {"error": f"Design with guid {guid} not found in kit."}
        sid = _session_id(ctx)
        _mcp_session_designs[sid] = design
        return {"ok": True, "guid": guid, "name": design.get("name", "")}
    except Exception as e:
        return {"error": str(e)}


def read_current_design(ctx: Context) -> dict:
    """Read the current design that was set via start_working_in_design.
    [👤semio📚engine💻engine🔖mcp🛠️readcurrentdesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/read_current_design)
    """
    try:
        return _get_session_design(ctx)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def finish_working_in_design(ctx: Context) -> dict:
    """Finish working in the current design. Clears the design from session state.
    [👤semio📚engine💻engine🔖mcp🛠️finishworkingindesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/finish_working_in_design)
    """
    try:
        sid = _session_id(ctx)
        _mcp_session_designs.pop(sid, None)
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def start_working_in_type(guid: str, ctx: Context) -> dict:
    """Start working in a type within the current kit.
    Callers MUST have called start_working_in_local_kit first. Selects the type by GUID.
    [👤semio📚engine💻engine🔖mcp🛠️startworkingintype](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/start_working_in_type)
    """
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


def read_current_type(ctx: Context) -> dict:
    """Read the current type that was set via start_working_in_type.
    [👤semio📚engine💻engine🔖mcp🛠️readcurrenttype](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/read_current_type)
    """
    try:
        return _get_session_type(ctx)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def finish_working_in_type(ctx: Context) -> dict:
    """Finish working in the current type. Clears the type from session state.
    [👤semio📚engine💻engine🔖mcp🛠️finishworkingintype](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/finish_working_in_type)
    """
    try:
        sid = _session_id(ctx)
        _mcp_session_types.pop(sid, None)
        return {"ok": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def finish_working_in_kit(ctx: Context) -> dict:
    """Finish working in the current kit. Clears kit, design, type, mode and source from session state.
    [👤semio📚engine💻engine🔖mcp🛠️finishworkinginkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/finish_working_in_kit)
    """
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
        _mcp_session_transactions[sid] = {
            "active": True,
            "started_at": datetime.datetime.now(datetime.UTC).isoformat(),
            "changes": [],
        }
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
    """Sum up the values of a quality across all pieces in a design.
    For each piece, uses the piece-level prop if present, otherwise falls back to the type-level prop.
    Callers MUST have called start_working_in_local_kit first.
    [👤semio📚engine💻engine🔖mcp🛠️sumqualityindesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/sum_quality_in_design)
    """
    try:
        kit = _get_session_kit(ctx)
        return {"result": sumQualityInDesignDict(kit, design_guid, quality_guid)}
    except Exception as e:
        return {"error": str(e)}


# endregion Mcp

# region Engine
# [👤semio📚engine💻engine🔖engine](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Engine)
# Engine MUST mount REST, GraphQL, and MCP sub-applications and manage the server lifecycle.


@contextlib.asynccontextmanager
async def engineLifespan(app):
    """Manages the MCP session lifecycle during engine startup and shutdown.
    Callers MUST use this as the lifespan parameter for the Starlette application.
    [👤semio📚engine💻engine🔖engine🛠️enginelifespan](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/engineLifespan)
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
    [👤semio📚engine💻engine🔖engine🛠️generateschemas](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/generateSchemas)
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
    if os.path.exists(sqliteSchemaPath):
        os.remove(sqliteSchemaPath)
    metadata_engine = sqlalchemy.create_engine("sqlite:///temp/semio.db")
    sqlmodel.SQLModel.metadata.create_all(metadata_engine)
    conn = sqlite3.connect("temp/semio.db")
    cursor = conn.cursor()
    cursor.execute("SELECT sql FROM sqlite_master WHERE type='table';")
    sqliteSchema = cursor.fetchall()
    with open(sqliteSchemaPath, "w", encoding="utf-8") as f:
        for table in sqliteSchema:
            f.write(f"{table[0]};\n")
    conn.close()

    with open("../../graphql/schema.graphql", "w", encoding="utf-8") as f:
        f.write(str(graphqlSchema))


def start_engine():
    """Starts the uvicorn server hosting the engine application.
    Callers MUST invoke this in a separate process to avoid blocking the UI.
    [👤semio📚engine💻engine🔖engine🛠️startengine](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/start_engine)
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
    [👤semio📚engine💻engine🔖engine🛠️restartengine](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/restart_engine)
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
    [👤semio📚engine💻engine🔖engine🛠️run](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/run)
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
    [👤semio📚engine💻engine🔖engine🛠️predev](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/preDev)
    """


def dev():
    """Starts the engine in development mode with debugging enabled.
    Callers MUST have debugpy available when using this entry point.
    [👤semio📚engine💻engine🔖engine🛠️dev](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Engine/d/i/dev)
    """
    run(dev_mode=True)


if __name__ == "__main__":
    run()

# endregion Engine
