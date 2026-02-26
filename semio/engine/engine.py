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
from mcp.server.fastmcp import FastMCP

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
    VERSION,
    Attribute,
    AttributeNode,
    Author,
    AuthorNode,
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
    Semio,
    ServerError,
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
    changeValues,
    decode,
    encode,
    flattenDesignDict,
    getKitDiffDict,
    inverseKitDiffDict,
    logger,
    normalizeAngle,
    parseValidationResult,
    planeFromYAxis,
    validateKitDict,
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


@functools.lru_cache
def StoreFactory(uri: str) -> Store:
    """🏭 Get a store from the uri. This store doesn't need to exist yet as long as it can be created.
    Callers MUST provide either an absolute local path or an http URL.
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
        raise RemoteKitsNotYetSupported(uri)
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

# endregion Rest

# region Mcp
# [👤semio📚engine💻engine🔖mcp](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp)
# Mcp MUST expose kit, type, design, validation, and diff tools via Model Context Protocol.

mcp = FastMCP("semio", stateless_http=True, json_response=True)


@mcp.tool()
def get_kit(uri: str) -> dict:
    """Get a kit from a URI. The URI can be a file path or a URL.
    Callers MUST provide a valid file path or URL as the URI.
    [👤semio📚engine💻engine🔖mcp🛠️getkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_kit)
    """
    try:
        result = get(encode(uri))
        return result.model_dump() if hasattr(result, "model_dump") else result
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def put_kit(uri: str, kit: dict) -> dict:
    """Put a kit at a URI. Creates or updates the kit.
    Callers MUST provide a valid URI and a dict matching KitInput schema.
    [👤semio📚engine💻engine🔖mcp🛠️putkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/put_kit)
    """
    try:
        kitInput = KitInput.model_validate(kit)
        put(encode(uri), kitInput)
        return {"success": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def mcp_delete_kit(uri: str) -> dict:
    """Delete a kit at a URI.
    Callers MUST provide a URI referencing an existing kit.
    [👤semio📚engine💻engine🔖mcp🛠️mcpdeletekit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/mcp_delete_kit)
    """
    try:
        delete(encode(uri))
        return {"success": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def get_type_from_kit(uri: str, name: str, variant: str = "") -> dict:
    """Get a type from a kit by name and variant.
    Callers MUST provide a valid kit URI and type name.
    [👤semio📚engine💻engine🔖mcp🛠️gettypefromkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_type_from_kit)
    """
    try:
        code = encode(uri) + "/types/" + encode(f"{name}~{variant}" if variant else name)
        result = get(code)
        return result.model_dump() if hasattr(result, "model_dump") else result
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def put_type_in_kit(uri: str, name: str, variant: str, type_data: dict) -> dict:
    """Put a type in a kit.
    Callers MUST provide a valid URI, name, variant, and TypeInput-compatible dict.
    [👤semio📚engine💻engine🔖mcp🛠️puttypeinkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/put_type_in_kit)
    """
    try:
        code = encode(uri) + "/types/" + encode(f"{name}~{variant}" if variant else name)
        typeInput = TypeInput.model_validate(type_data)
        put(code, typeInput)
        return {"success": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def delete_type_from_kit(uri: str, name: str, variant: str = "") -> dict:
    """Delete a type from a kit.
    Callers MUST provide a valid kit URI and existing type name.
    [👤semio📚engine💻engine🔖mcp🛠️deletetypefromkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/delete_type_from_kit)
    """
    try:
        code = encode(uri) + "/types/" + encode(f"{name}~{variant}" if variant else name)
        delete(code)
        return {"success": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def get_design_from_kit(uri: str, name: str, variant: str = "", view: str = "") -> dict:
    """Get a design from a kit by name, variant, and view.
    Callers MUST provide a valid kit URI and design name.
    [👤semio📚engine💻engine🔖mcp🛠️getdesignfromkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_design_from_kit)
    """
    try:
        nameVariantView = name
        if variant:
            nameVariantView += f"~{variant}"
        if view:
            nameVariantView += f"@{view}"
        code = encode(uri) + "/designs/" + encode(nameVariantView)
        result = get(code)
        return result.model_dump() if hasattr(result, "model_dump") else result
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def put_design_in_kit(uri: str, name: str, variant: str, view: str, design_data: dict) -> dict:
    """Put a design in a kit.
    Callers MUST provide a valid URI, name, variant, view, and DesignInput-compatible dict.
    [👤semio📚engine💻engine🔖mcp🛠️putdesigninkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/put_design_in_kit)
    """
    try:
        nameVariantView = name
        if variant:
            nameVariantView += f"~{variant}"
        if view:
            nameVariantView += f"@{view}"
        code = encode(uri) + "/designs/" + encode(nameVariantView)
        designInput = DesignInput.model_validate(design_data)
        put(code, designInput)
        return {"success": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def delete_design_from_kit(uri: str, name: str, variant: str = "", view: str = "") -> dict:
    """Delete a design from a kit.
    Callers MUST provide a valid kit URI and existing design name.
    [👤semio📚engine💻engine🔖mcp🛠️deletedesignfromkit](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/delete_design_from_kit)
    """
    try:
        nameVariantView = name
        if variant:
            nameVariantView += f"~{variant}"
        if view:
            nameVariantView += f"@{view}"
        code = encode(uri) + "/designs/" + encode(nameVariantView)
        delete(code)
        return {"success": True}
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
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


@mcp.tool()
def flatten_design(kit: dict, design_guid: str) -> dict:
    """Flatten a design by computing absolute planes for all pieces.
    Callers MUST provide a valid kit dict and an existing design GUID.
    [👤semio📚engine💻engine🔖mcp🛠️flattendesign](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/flatten_design)
    """
    try:
        return flattenDesignDict(kit, design_guid)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def get_kit_diff(before: dict, after: dict) -> dict:
    """Get the diff between two kit states.
    Callers MUST provide two valid kit dicts for comparison.
    [👤semio📚engine💻engine🔖mcp🛠️getkitdiff](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/get_kit_diff)
    """
    try:
        return getKitDiffDict(before, after)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def apply_kit_diff(base: dict, diff: dict) -> dict:
    """Apply a diff to a kit.
    Callers MUST provide a valid base kit dict and a compatible diff dict.
    [👤semio📚engine💻engine🔖mcp🛠️applykitdiff](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/apply_kit_diff)
    """
    try:
        return applyKitDiffDict(base, diff)
    except Exception as e:
        return {"error": str(e)}


@mcp.tool()
def inverse_kit_diff(original: dict, applied_diff: dict) -> dict:
    """Get the inverse of a diff (for undo operations).
    Callers MUST provide the original kit dict and the applied diff dict.
    [👤semio📚engine💻engine🔖mcp🛠️inversekitdiff](semiorepo://p/u/semio/b/l/engine/f/engine.py/s/Mcp/d/i/inverse_kit_diff)
    """
    try:
        return inverseKitDiffDict(original, applied_diff)
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
