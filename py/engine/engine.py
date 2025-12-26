# region Header

# py/engine/engine.py

# 2025 Ueli Saluz <ueli@semio-tech.com>

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as
# published by the Free Software Foundation, either version 3 of the
# License, or (at your option) any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.

# You should have received a copy of the GNU Lesser General Public License
# along with this program.  If not, see <https://www.gnu.org/licenses/>.

# endregion Header
# region TODOs
# TODO: Make loguru work on extra uvicorn engine process.
# TODO: Replace prototype healing with one that makes more for every single property.
# TODO: Try closest embedding instead of smallest Levenshtein distance.
# TODO: Automatic derive from Id model.
# TODO: Automatic emptying.
# TODO: Automatic updating based on props.
# TODO: Check how to automate docstring duplication, table=True and PLURAL and __tablename__.
# TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
# TODO: Proper mechanism of nullable fields.
# TODO: Generalize to non-zip kits.
# TODO: Think of using memory sqlite for caching.
# TODO: Get rid of id_ because of bug https://github.com/graphql-python/graphene-sqlalchemy/issues/412
# endregion TODOs
# region Imports
import abc
import argparse
import dataclasses
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
import urllib
import zipfile

import dotenv
import fastapi
import fastapi.openapi
import graphene

if sys.version_info >= (3, 13):
    import graphene_pydantic.util

    def _patched_evaluate_forward_ref(type_: typing.ForwardRef, globalns: typing.Any, localns: typing.Any) -> typing.Any:
        return typing.cast(typing.Any, type_)._evaluate(globalns, localns, recursive_guard=frozenset())

    graphene_pydantic.util.evaluate_forward_ref = _patched_evaluate_forward_ref
    import graphene_pydantic.converters

    graphene_pydantic.converters.evaluate_forward_ref = _patched_evaluate_forward_ref
    import sqlmodel._compat

    _original_get_relationship_to = sqlmodel._compat.get_relationship_to

    def _patched_get_relationship_to(name: str, rel_info: typing.Any, annotation: typing.Any) -> typing.Any:
        if isinstance(annotation, str):
            import re

            def strip_quotes(s: str) -> str:
                if (s.startswith("'") and s.endswith("'")) or (s.startswith('"') and s.endswith('"')):
                    return s[1:-1]
                return s

            annotation = strip_quotes(annotation)
            list_match = re.match(r"list\[(.+)\]", annotation)
            if list_match:
                annotation = strip_quotes(list_match.group(1))
            return annotation
        return _original_get_relationship_to(name=name, rel_info=rel_info, annotation=annotation)

    sqlmodel._compat.get_relationship_to = _patched_get_relationship_to
    import sqlmodel.main

    sqlmodel.main.get_relationship_to = _patched_get_relationship_to
import graphene_pydantic
import graphene_sqlalchemy
import jinja2
import lark
import loguru
import networkx
import numpy
import openai
import pydantic
import PySide6.QtCore
import PySide6.QtGui
import PySide6.QtWidgets
import pytransform3d.rotations
import requests
import sqlalchemy
import sqlalchemy.exc
import sqlalchemy.orm
import sqlmodel
import starlette
import starlette_graphene3
import uvicorn

# endregion Imports

# region Type Hints


RecursiveAnyList = typing.Any | list["RecursiveAnyList"]
"""🔁 A recursive any list is either any or a list where the items are recursive any list."""


# endregion Type Hints

# region Constants

NAME = "semio"
EMAIL = "mail@semio-tech.com"
RELEASE = "r25.07-1"
VERSION = "4.3.0-beta"
HOST = "127.0.0.1"
PORT = 2507
ADDRESS = "http://127.0.0.1:2507"
NAME_LENGTH_LIMIT = 64
ID_LENGTH_LIMIT = 128
URL_LENGTH_LIMIT = 1024
URI_LENGTH_LIMIT = 2048
EXPRESSION_LENGTH_LIMIT = 4096
VALUE_LENGTH_LIMIT = 512
ATTRIBUTES_MAX = 64
QUALITIES_MAX = 1024
TAGS_MAX = 8
REPRESENTATIONS_MAX = 32
TYPES_MAX = 256
PIECES_MAX = 512
DESIGNS_MAX = 128
KITS_MAX = 64
DESCRIPTION_LENGTH_LIMIT = 512
ENCODING_ALPHABET_REGEX = r"[a-zA-Z0-9\-._~%]"
ENCODING_REGEX = ENCODING_ALPHABET_REGEX + "+"
KIT_LOCAL_FOLDERNAME = ".semio"
KIT_LOCAL_FILENAME = "kit.db"
KIT_LOCAL_SUFFIX = str(pathlib.Path(KIT_LOCAL_FOLDERNAME) / pathlib.Path(KIT_LOCAL_FILENAME))
USER_FOLDER = str(pathlib.Path.home() / ".semio")
CACHE_FOLDER = str(pathlib.Path(USER_FOLDER) / "cache")
LOG_FOLDER = str(pathlib.Path(USER_FOLDER) / "logs")
DEBUG_LOG_FILE = str(pathlib.Path(LOG_FOLDER) / "debug.log")
TOLERANCE = 1e-5
SIGNIFICANT_DIGITS = 5
MIMES = {
    ".stl": "model/stl",
    ".obj": "model/obj",
    ".glb": "model/gltf-binary",
    ".gltf": "model/gltf+json",
    ".3dm": "model/vnd.3dm",
    ".png": "image/png",
    ".jpg": "image/jpeg",
    ".jpeg": "image/jpeg",
    ".svg": "image/svg+xml",
    ".pdf": "application/pdf",
    ".zip": "application/zip",
    ".json": "application/json",
    ".csv": "text/csv",
    ".txt": "text/plain",
}
ENCODED_PATH = typing.Annotated[str, fastapi.Path(pattern=ENCODING_REGEX)]
ENCODED_NAME_AND_VARIANT_PATH = typing.Annotated[str, fastapi.Path(pattern=ENCODING_REGEX + "," + ENCODING_ALPHABET_REGEX + "*")]
ENCODED_NAME_AND_VARIANT_AND_VIEW_PATH = typing.Annotated[str, fastapi.Path(pattern=ENCODING_REGEX + "," + ENCODING_ALPHABET_REGEX + "*" + "," + ENCODING_ALPHABET_REGEX + "*")]
MAX_REQUEST_BODY_SIZE = 50 * 1024 * 1024
dotenv.load_dotenv()
ENVS = {key: value for key, value in os.environ.items() if key.startswith("SEMIO_")}


# endregion Constants

# region Utility


def encode(value: str) -> str:
    """ᗒ Encode a string to be url safe."""
    return urllib.parse.quote(value, safe="")


def decode(value: str) -> str:
    """ᗕ Decode a url safe string."""
    return urllib.parse.unquote(value)


def encodeList(items: list[str]) -> str:
    return ",".join([encode(t) for t in items])


def decodeList(encodedList: str) -> list[str]:
    return [decode(t) for t in encodedList.split(",")]


def encodeRecursiveAnyList(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Encode a `RecursiveAnyList` to a url encoded string."""
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return encode(",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList]))


def create_id(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Turn any into `encoded(str(any))` or a recursive list into a flat comma [,] separated encoded list."""
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return ",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList])


def pretty(number: float) -> str:
    """🦋 Pretty print a floating point number."""
    if number == -0.0:
        number = 0.0
    return f"{number:.5f}".rstrip("0").rstrip(".")


def changeValues(c: dict | list, key: str, func: typing.Callable[[typing.Any], typing.Any]) -> None:
    if isinstance(c, dict):
        if key in c:
            c[key] = func(c[key])
        for v in c.values():
            if isinstance(v, dict) or isinstance(v, list):
                changeValues(v, key, func)
    if isinstance(c, list):
        for v in c:
            if isinstance(v, dict) or isinstance(v, list):
                changeValues(v, key, func)


def changeKeys(c: dict | list, func: typing.Callable[[typing.Any], typing.Any]) -> None:
    if isinstance(c, dict):
        for k in list(c.keys()):
            newKey = func(k)
            v = c.pop(k)
            c[newKey] = v
            if isinstance(v, dict) or isinstance(v, list):
                changeKeys(v, func)
    if isinstance(c, list):
        for v in c:
            if isinstance(v, dict) or isinstance(v, list):
                changeKeys(v, func)


def normalizeAngle(angle: float) -> float:
    """🔃 Normalize an angle to be greater or equal to 0 and smaller than 360 degrees."""
    return (angle % 360 + 360) % 360


# endregion Utility

# region Logging


logger = loguru.logger


# endregion Logging

# region Exceptions


class Error(Exception, abc.ABC):
    """❗ The base for all exceptions."""

    def __str__(self):
        return "❗ " + self.__class__.__name__


class ServerError(Error, abc.ABC):
    """🖥️ The base for all server errors."""


class ClientError(Error, abc.ABC):
    """👩‍💼 The base for all client errors."""


class CodeUnreachable(ServerError):
    def __str__(self):
        return "🤷 This code should be unreachable."


class FeatureNotYetSupported(ServerError):
    def __str__(self):
        return "🔜 This feature is not yet supported."


class RemoteKitsNotYetSupported(FeatureNotYetSupported):
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return "🔜 Remote kits are not yet supported."


class NotFound(ClientError, abc.ABC):
    """🔍 The base for not found errors."""


class SpecificationError(ClientError, abc.ABC):
    """📋 The base for all specification errors."""


class NoParentAssigned(SpecificationError, abc.ABC):
    """👪 The base for all no parent assigned errors."""


class NoTypeOrDesignAssigned(NoParentAssigned):
    def __str__(self):
        return "👪 The entity has no parent type or design assigned."


class NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned(NoParentAssigned):
    def __str__(self):
        return "👪 The entity has no parent model, connector, type, piece, connection, design, kit or folder assigned."


class AlreadyExists(SpecificationError, abc.ABC):
    """♊ The entity already exists in the store."""


class Semio(sqlmodel.SQLModel, table=True):
    """ℹ️ Metadata about the database."""

    __tablename__ = "semio"

    release: str = sqlmodel.Field(default=RELEASE, primary_key=True)
    """🍾 The current release of semio."""
    engine: str = sqlmodel.Field(default=VERSION)
    """⚙️ The version of the engine that created this database."""
    created_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)
    """⌚ The time when the database was created."""


# endregion Exceptions

# region Modeling

# region Primitives


class Model(sqlmodel.SQLModel, abc.ABC):
    """⚪ The base for models."""

    model_config = pydantic.ConfigDict(arbitrary_types_allowed=True)

    @classmethod
    def parse(cls, input: str | dict | typing.Any | None) -> "Model":
        """⚒️ Parse the entity from an input."""
        if input is None:
            return cls()
        if isinstance(input, str):
            return cls.model_validate_json(input)
        return cls.model_validate(input)

    def dump(self) -> "Output":
        """📦 Dump the entity to a dictionary."""
        return self.model_dump()


BaseModel = Model


class Field(Model, abc.ABC):
    """🎫 The base for a field of a model."""


class RealField(Field, abc.ABC):
    """🧑 The base for a real field of a model. No lie."""


class MaskedField(Field, abc.ABC):
    """🎭 The base for a mask of a field of a model. WYSIWYG but don't expect it to be there."""


class Base(Model, abc.ABC):
    """👥 The base for models."""


class Id(Base, abc.ABC):
    """🪪 The base for ids. All fields that identify the entity here."""


class Props(Base, abc.ABC):
    """🎫 The base for props. All fields except input-only, output-only or child entities."""


class Input(Base, abc.ABC):
    """↘️ The base for inputs. All fields that are required to create the entity."""


class Context(Base, abc.ABC):
    """📑 The base for contexts. All fields that are required to understand the entity by an llm."""


class Output(Base, abc.ABC):
    """↗️ The base for outputs. All fields that are returned when the entity is fetched."""


class Prediction(Base, abc.ABC):
    """🔮 The base for predictions. All fields that are required to predict the entity by a llm."""


class Entity(Model, abc.ABC):
    """▢ The base for entities. All fields and behavior of the entity."""

    PLURAL: typing.ClassVar[str]
    """🔢 The plural of the singular of the entity name."""

    def parent(self) -> typing.Optional["Entity"]:
        """👪 The parent entity of the entity."""
        return None

    # TODO: Automatic derive from Id model.
    @abc.abstractmethod
    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the entity within its parent."""

    def id(self) -> str:
        """🆔 The id of the entity within its parent."""
        return create_id(self.idMembers())

    def guid(self) -> str:
        """🆔 A Globally Unique Identifier (GUID) of the entity."""
        localId = f"{self.__class__.PLURAL.lower()}/{self.id()}"
        parent = self.parent()
        parentId = f"{parent.guid()}/" if parent is not None else ""
        return parentId + localId

    def clientId(self) -> str:
        """🆔 The client id of the entity."""
        return self.id()

    # TODO: Automatic emptying.

    def empty(self) -> "Entity":
        """🪣 Empty all props and children of the entity."""
        return self.__class__()

    # TODO: Automatic updating based on props.

    def update(self, other: "Entity") -> "Entity":
        """🔄 Update the props of the entity."""
        return self


class Table(Model, abc.ABC):
    """▦ The base for tables. All resources that are stored in the database."""


class TableEntity(Entity, Table, abc.ABC):
    """▢ The base for table entities."""

    __tablename__: typing.ClassVar[str]
    """📛 The lowercase name of the table in the database."""


# endregion Primitives

# region Graphql


class Node(graphene_pydantic.PydanticObjectType):
    """A base class for all nodes that are not a table in the database."""

    class Meta:
        abstract = True

    @classmethod
    def __init_subclass_with_meta__(cls, model=None, **options):
        if "name" not in options:
            options["name"] = model.__name__

        super().__init_subclass_with_meta__(model=model, **options)


class InputNode(graphene_pydantic.PydanticInputObjectType):
    """A base class for all input nodes."""

    class Meta:
        abstract = True


class RelayNode(graphene.relay.Node):
    class Meta:
        name = "Node"

    @staticmethod
    def to_global_id(type_, id):
        return id

    @staticmethod
    def get_node_from_global_id(info, global_id, only_type=None):
        entity = get(global_id)
        return entity


class TableNode(graphene_sqlalchemy.SQLAlchemyObjectType):
    """A base class for all nodes that are a table in the database.
    It automatically excludes the fields that are defined in the table.
    Resolvers to all @properties are added.
    Child relationships are by default included."""

    class Meta:
        abstract = True

    @classmethod
    def __init_subclass_with_meta__(cls, model=None, **options):
        excludedFields = tuple(k for k, v in model.model_fields.items() if v.exclude)
        if "exclude_fields" in options:
            options["exclude_fields"] += excludedFields
        else:
            options["exclude_fields"] = excludedFields

        super().__init_subclass_with_meta__(model=model, **options)


class TableEntityNode(TableNode):
    """A base class for all nodes that are a table in the database and are entities.
    It automatically complies to the Relay Node interface."""

    class Meta:
        abstract = True

    @classmethod
    def __init_subclass_with_meta__(cls, model=None, **options):
        if "interfaces" not in options:
            options["interfaces"] = (RelayNode,)

        def resolve_id(self, info):
            return self.guid()

        setattr(cls, "resolve_id", resolve_id)

        super().__init_subclass_with_meta__(model=model, **options)


# endregion Graphql

# endregion Modeling

# region Domain

# region Attribute


class AttributeKeyField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class AttributeValueField(RealField, abc.ABC):
    value: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class AttributeDefinitionField(RealField, abc.ABC):
    definition: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class AttributeId(AttributeKeyField, Id):
    pass


class AttributeProps(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Props):
    pass


class AttributeInput(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Input):
    pass


class AttributeContext(AttributeValueField, AttributeKeyField, Context):
    pass


class AttributeOutput(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Output):
    pass


class Attribute(AttributeDefinitionField, AttributeValueField, AttributeKeyField, TableEntity, table=True):
    PLURAL = "attributes"
    __tablename__ = "attributes"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    modelPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("model_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("models.id")), default=None, exclude=True)
    model: Model = sqlmodel.Relationship(back_populates="attributes")
    connectorPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("connector_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("connectors.id")), default=None, exclude=True)
    connector: Connector = sqlmodel.Relationship(back_populates="attributes")
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id")), default=None, exclude=True)
    type: Type = sqlmodel.Relationship(back_populates="attributes")
    piecePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("pieces.id")), default=None, exclude=True)
    piece: Piece = sqlmodel.Relationship(back_populates="attributes")
    connectionPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("connection_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("connections.id")), default=None, exclude=True)
    connection: Connection = sqlmodel.Relationship(back_populates="attributes")
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: Design = sqlmodel.Relationship(back_populates="attributes")
    kitPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    kit: Kit = sqlmodel.Relationship(back_populates="attributes")
    qualityPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("quality_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("qualities.id")), default=None, exclude=True)
    quality: Quality = sqlmodel.Relationship(back_populates="attributes")
    propPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("prop_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("props.id")), default=None, exclude=True)
    prop: Prop = sqlmodel.Relationship(back_populates="attributes")
    authorPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("author_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("authors.id")), default=None, exclude=True)
    author: Author = sqlmodel.Relationship(back_populates="attributes")
    locationPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("location_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("locations.id")), default=None, exclude=True)
    location: Location = sqlmodel.Relationship(back_populates="attributes")
    benchmarkPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("benchmark_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("benchmarks.id")), default=None, exclude=True)
    benchmark: Benchmark = sqlmodel.Relationship(back_populates="attributes")
    folderPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("folder_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("folders.id")), default=None, exclude=True)
    folder: Folder = sqlmodel.Relationship(back_populates="attributes")
    interfacePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("interface_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("interfaces.id")), default=None, exclude=True)
    interface_: Interface = sqlmodel.Relationship(back_populates="attributes")

    __table_args__ = (
        sqlalchemy.CheckConstraint(
            """
        (
            (model_id IS NOT NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NOT NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NOT NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NOT NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NOT NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NOT NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NOT NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NOT NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NOT NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NOT NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NOT NULL AND benchmark_id IS NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NOT NULL AND folder_id IS NULL)
        OR
            (model_id IS NULL AND connector_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL AND folder_id IS NOT NULL)
        )
        """,
            name="ck_attributes_parent_set",
        ),
        sqlalchemy.UniqueConstraint("name", "type_id", "design_id", name="uq_attributes_name_type_id_design_id"),
    )

    def parent(self) -> typing.Union["Model", "Connector", "Type", "Piece", "Connection", "Design", "Kit", "Quality", "Prop", "Author", "Location", "Benchmark", "Folder", None]:
        if self.model is not None:
            return self.model
        if self.connector is not None:
            return self.connector
        if self.type is not None:
            return self.type
        if self.piece is not None:
            return self.piece
        if self.connection is not None:
            return self.connection
        if self.design is not None:
            return self.design
        if self.kit is not None:
            return self.kit
        if self.quality is not None:
            return self.quality
        if self.prop is not None:
            return self.prop
        if self.author is not None:
            return self.author
        if self.location is not None:
            return self.location
        if self.benchmark is not None:
            return self.benchmark
        if self.folder is not None:
            return self.folder
        raise NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.name

    @classmethod
    def parse(cls, input: str | dict | typing.Any | None) -> "Attribute":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        return cls(
            name=obj.get("name", obj.get("key", "")),
            value=obj.get("value", ""),
            definition=obj.get("definition", ""),
        )


class AttributeInputNode(InputNode):
    class Meta:
        model = AttributeInput


# endregion Attribute

# region Tag


class TagGuidField(RealField, abc.ABC):
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)


class TagNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class TagDescriptionField(RealField, abc.ABC):
    description: typing.Optional[str] = sqlmodel.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)


class TagIconField(RealField, abc.ABC):
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)


class TagOrderField(RealField, abc.ABC):
    order: int = sqlmodel.Field(default=0)


class TagId(TagGuidField, Id):
    pass


class Tag(TagIconField, TagDescriptionField, TagOrderField, TagNameField, TagGuidField, Table, table=True):
    __tablename__ = "tags"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    modelPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("model_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("models.id")), default=None, exclude=True)
    model: Model = sqlmodel.Relationship(back_populates="tags_")


# endregion Tag

# region Concept


class ConceptGuidField(RealField, abc.ABC):
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)


class ConceptNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class ConceptDescriptionField(RealField, abc.ABC):
    description: typing.Optional[str] = sqlmodel.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)


class ConceptIconField(RealField, abc.ABC):
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)


class ConceptOrderField(RealField, abc.ABC):
    order: int = sqlmodel.Field(default=0)


class ConceptId(ConceptGuidField, Id):
    pass


class Concept(ConceptIconField, ConceptDescriptionField, ConceptOrderField, ConceptNameField, ConceptGuidField, Table, table=True):
    __tablename__ = "concepts"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id")), default=None, exclude=True)
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    kit: Kit = sqlmodel.Relationship(back_populates="concepts_")
    type: Type = sqlmodel.Relationship(back_populates="concepts_")
    design: Design = sqlmodel.Relationship(back_populates="concepts_")


# endregion Concept

# region Coord


class Coord(Model):
    u: float = sqlmodel.Field()
    v: float = sqlmodel.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.u)}, {pretty(self.v)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.u)}, {pretty(self.v)}]"


class CoordInput(Coord, Input):
    pass


class CoordContext(Coord, Context):
    pass


class CoordOutput(Coord, Output):
    pass


class CoordPrediction(Coord, Prediction):
    pass


class CoordNode(Node):
    class Meta:
        model = Coord


class CoordInputNode(InputNode):
    class Meta:
        model = CoordInput


# endregion Coord

# region Point


class Point(Model):
    x: float = sqlmodel.Field()
    y: float = sqlmodel.Field()
    z: float = sqlmodel.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"


class PointInput(Point, Input):
    pass


class PointContext(Point, Context):
    pass


class PointOutput(Point, Output):
    pass


class PointPrediction(Point, Prediction):
    pass


class PointNode(Node):
    class Meta:
        model = Point


class PointInputNode(InputNode):
    class Meta:
        model = PointInput


# endregion Point

# region Vector


class Vector(Model):
    x: float = sqlmodel.Field()
    y: float = sqlmodel.Field()
    z: float = sqlmodel.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"


class VectorInput(Vector, Input):
    pass


class VectorContext(Vector, Context):
    pass


class VectorOutput(Vector, Output):
    pass


class VectorPrediction(Vector, Prediction):
    pass


class VectorNode(Node):
    class Meta:
        model = Vector


class VectorInputNode(InputNode):
    class Meta:
        model = VectorInput


# endregion Vector

# region Plane


class PlaneOriginField(MaskedField, abc.ABC):
    origin: Point = sqlmodel.Field()


class PlaneXAxisField(MaskedField, abc.ABC):
    xAxis: Vector = sqlmodel.Field()


class PlaneYAxisField(MaskedField, abc.ABC):
    yAxis: Vector = sqlmodel.Field()


class PlaneInput(Input):
    origin: PointInput = sqlmodel.Field()
    xAxis: VectorInput = sqlmodel.Field()
    yAxis: VectorInput = sqlmodel.Field()


class PlaneContext(Context):
    origin: PointContext = sqlmodel.Field()
    xAxis: VectorContext = sqlmodel.Field()
    yAxis: VectorContext = sqlmodel.Field()


class PlaneOutput(PlaneYAxisField, PlaneXAxisField, PlaneOriginField, Output):
    pass


class Plane(Table, table=True):
    __tablename__ = "planes"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    originX: float = sqlmodel.Field(sa_column=sqlmodel.Column("origin_x", sqlalchemy.Float()), exclude=True)
    originY: float = sqlmodel.Field(sa_column=sqlmodel.Column("origin_y", sqlalchemy.Float()), exclude=True)
    originZ: float = sqlmodel.Field(sa_column=sqlmodel.Column("origin_z", sqlalchemy.Float()), exclude=True)
    xAxisX: float = sqlmodel.Field(sa_column=sqlmodel.Column("x_axis_x", sqlalchemy.Float()), exclude=True)
    xAxisY: float = sqlmodel.Field(sa_column=sqlmodel.Column("x_axis_y", sqlalchemy.Float()), exclude=True)
    xAxisZ: float = sqlmodel.Field(sa_column=sqlmodel.Column("x_axis_z", sqlalchemy.Float()), exclude=True)
    yAxisX: float = sqlmodel.Field(sa_column=sqlmodel.Column("y_axis_x", sqlalchemy.Float()), exclude=True)
    yAxisY: float = sqlmodel.Field(sa_column=sqlmodel.Column("y_axis_y", sqlalchemy.Float()), exclude=True)
    yAxisZ: float = sqlmodel.Field(sa_column=sqlmodel.Column("y_axis_z", sqlalchemy.Float()), exclude=True)
    piece: Piece = sqlmodel.Relationship(back_populates="plane")

    @property
    def origin(self) -> Point:
        return Point(
            x=self.originX,
            y=self.originY,
            z=self.originZ,
        )

    @origin.setter
    def origin(self, origin: Point):
        self.originX = origin.x
        self.originY = origin.y
        self.originZ = origin.z

    @property
    def xAxis(self) -> Vector:
        return Vector(
            x=self.xAxisX,
            y=self.xAxisY,
            z=self.xAxisZ,
        )

    @xAxis.setter
    def xAxis(self, xAxis: Vector):
        self.xAxisX = xAxis.x
        self.xAxisY = xAxis.y
        self.xAxisZ = xAxis.z

    @property
    def yAxis(self) -> Vector:
        return Vector(
            x=self.yAxisX,
            y=self.yAxisY,
            z=self.yAxisZ,
        )

    @yAxis.setter
    def yAxis(self, yAxis: Vector):
        self.yAxisX = yAxis.x
        self.yAxisY = yAxis.y
        self.yAxisZ = yAxis.z

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Plane", input: str | dict | PlaneInput | typing.Any | None) -> "Plane":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        origin = Point.model_validate(obj["origin"])
        xAxis = Vector.model_validate(obj["xAxis"])
        yAxis = Vector.model_validate(obj["yAxis"])
        entity = Plane()
        entity.origin = origin
        entity.xAxis = xAxis
        entity.yAxis = yAxis

        return entity

    def dump(self) -> PlaneOutput:
        entity = {**PlaneOriginField.model_validate(self).model_dump()}
        entity["xAxis"] = self.xAxis
        entity["yAxis"] = self.yAxis
        return PlaneOutput(**entity)


class PlaneInputNode(InputNode):
    class Meta:
        model = PlaneInput


# endregion Plane

# region Location


class LocationGuidField(RealField, abc.ABC):
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)


class LocationLongitudeField(RealField, abc.ABC):
    longitude: float = sqlmodel.Field()


class LocationLatitudeField(RealField, abc.ABC):
    latitude: float = sqlmodel.Field()


class LocationAltitudeField(RealField, abc.ABC):
    altitude: typing.Optional[float] = sqlmodel.Field(default=None)


class LocationId(LocationGuidField, Id):
    pass


class Location(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, LocationGuidField, TableEntity, table=True):
    PLURAL = "locations"
    __tablename__ = "locations"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="location", cascade_delete=True)


class LocationInput(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Input):
    pass


class LocationOutput(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Output):
    pass


class LocationContext(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Context):
    pass


class LocationPrediction(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Prediction):
    pass


class LocationNode(Node):
    class Meta:
        model = LocationOutput


class LocationInputNode(InputNode):
    class Meta:
        model = LocationInput


# endregion Location

# region Author


class AuthorNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class AuthorEmailField(RealField, abc.ABC):
    email: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)


class AuthorRankField(RealField, abc.ABC):
    rank: int = sqlmodel.Field(default=0)


class AuthorId(AuthorEmailField, Id):
    pass


class AuthorProps(AuthorEmailField, AuthorNameField, Props):
    pass


class AuthorInput(AuthorEmailField, AuthorNameField, Input):
    pass


class AuthorOutput(AuthorEmailField, AuthorNameField, Output):
    pass


class Author(AuthorRankField, AuthorEmailField, AuthorNameField, TableEntity, table=True):
    PLURAL = "authors"
    __tablename__ = "authors"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    kit: Kit = sqlmodel.Relationship(back_populates="authors_")
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="author", cascade_delete=True)

    __table_args__ = (sqlalchemy.UniqueConstraint("email", "kit_id", name="uq_authors_email_kit_id"),)

    def parent(self) -> "Kit":
        if self.kit is not None:
            return self.kit
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.email


class AuthorInputNode(InputNode):
    class Meta:
        model = AuthorInput


# endregion Author

# region ArtifactAuthor


class ArtifactAuthorEmailField(RealField, abc.ABC):
    author_email: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)


class ArtifactAuthor(ArtifactAuthorEmailField, TableEntity, table=True):
    PLURAL = "artifact_authors"
    __tablename__ = "artifact_authors"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id")), default=None, exclude=True)
    type: Type = sqlmodel.Relationship(back_populates="artifact_authors")
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: Design = sqlmodel.Relationship(back_populates="artifact_authors")

    __table_args__ = (
        sqlalchemy.CheckConstraint("(type_id IS NOT NULL AND design_id IS NULL) OR (type_id IS NULL AND design_id IS NOT NULL)", name="ck_artifact_authors_parent_set"),
        sqlalchemy.UniqueConstraint("author_email", "type_id", "design_id", name="uq_artifact_authors_email_type_id_design_id"),
    )

    def parent(self) -> typing.Union["Type", "Design", None]:
        if self.type is not None:
            return self.type
        if self.design is not None:
            return self.design
        raise NoTypeOrDesignAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return [self.author_email, self.type.idMembers() if self.type else self.design.idMembers()]


# endregion ArtifactAuthor

# region File


class FileGuidField(RealField, abc.ABC):
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)


class FileNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class FileMimeField(RealField, abc.ABC):
    mime: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class FileRemoteField(RealField, abc.ABC):
    remote: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)


class FileFolderField(RealField, abc.ABC):
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)


class FileSizeField(RealField, abc.ABC):
    size: typing.Optional[int] = sqlmodel.Field(default=None)


class FileHashField(RealField, abc.ABC):
    hash: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class FileCreatedAtField(RealField, abc.ABC):
    createdAt: datetime.datetime = sqlmodel.Field()


class FileCreatedByField(RealField, abc.ABC):
    createdBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FileUpdatedAtField(RealField, abc.ABC):
    updatedAt: datetime.datetime = sqlmodel.Field()


class FileUpdatedByField(RealField, abc.ABC):
    updatedBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FileId(FileGuidField, Id):
    pass


class FileProps(FileUpdatedByField, FileUpdatedAtField, FileCreatedByField, FileCreatedAtField, FileHashField, FileSizeField, FileFolderField, FileRemoteField, FileMimeField, FileNameField, FileGuidField, Props):
    pass


class FileInput(FileUpdatedByField, FileUpdatedAtField, FileCreatedByField, FileCreatedAtField, FileHashField, FileSizeField, FileFolderField, FileRemoteField, FileMimeField, FileNameField, FileGuidField, Input):
    pass


class FileContext(FileNameField, FileGuidField, Context):
    pass


class FileOutput(FileUpdatedByField, FileUpdatedAtField, FileCreatedByField, FileCreatedAtField, FileHashField, FileSizeField, FileFolderField, FileRemoteField, FileMimeField, FileNameField, FileGuidField, Output):
    pass


class File(FileUpdatedByField, FileUpdatedAtField, FileCreatedByField, FileCreatedAtField, FileHashField, FileSizeField, FileFolderField, FileRemoteField, FileMimeField, FileNameField, FileGuidField, TableEntity, table=True):
    PLURAL = "files"
    __tablename__ = "files"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    kit: Kit = sqlmodel.Relationship(back_populates="files_")

    __table_args__ = (sqlalchemy.UniqueConstraint("guid", "kit_id", name="uq_files_guid_kit_id"),)

    def parent(self) -> "Kit":
        if self.kit is not None:
            return self.kit
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.guid


class FileInputNode(InputNode):
    class Meta:
        model = FileInput


# endregion File

# region Folder


class FolderGuidField(RealField, abc.ABC):
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)


class FolderNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class FolderParentField(RealField, abc.ABC):
    parent: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FolderDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class FolderCreatedAtField(RealField, abc.ABC):
    createdAt: datetime.datetime = sqlmodel.Field()


class FolderCreatedByField(RealField, abc.ABC):
    createdBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FolderUpdatedAtField(RealField, abc.ABC):
    updatedAt: datetime.datetime = sqlmodel.Field()


class FolderUpdatedByField(RealField, abc.ABC):
    updatedBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FolderId(FolderGuidField, Id):
    pass


class FolderProps(FolderUpdatedByField, FolderUpdatedAtField, FolderCreatedByField, FolderCreatedAtField, FolderDescriptionField, FolderParentField, FolderNameField, FolderGuidField, Props):
    pass


class FolderInput(FolderUpdatedByField, FolderUpdatedAtField, FolderCreatedByField, FolderCreatedAtField, FolderDescriptionField, FolderParentField, FolderNameField, FolderGuidField, Input):
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)


class FolderContext(FolderNameField, FolderGuidField, Context):
    pass


class FolderOutput(FolderUpdatedByField, FolderUpdatedAtField, FolderCreatedByField, FolderCreatedAtField, FolderDescriptionField, FolderParentField, FolderNameField, FolderGuidField, Output):
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)


class Folder(FolderUpdatedByField, FolderUpdatedAtField, FolderCreatedByField, FolderCreatedAtField, FolderDescriptionField, FolderParentField, FolderNameField, FolderGuidField, TableEntity, table=True):
    PLURAL = "folders"
    __tablename__ = "folders"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    kit: Kit = sqlmodel.Relationship(back_populates="folders_")
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="folder", cascade_delete=True)

    __table_args__ = (sqlalchemy.UniqueConstraint("guid", "kit_id", name="uq_folders_guid_kit_id"),)

    def parent(self) -> "Kit":
        if self.kit is not None:
            return self.kit
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.guid

    @classmethod
    def parse(cls, input: str | dict | FolderInput | typing.Any | None) -> "Folder":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        props = FolderProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            entity.attributes = [typing.cast(Attribute, Attribute.parse(attribute)) for attribute in obj["attributes"]]
        except KeyError:
            pass
        return entity

    def dump(self) -> "FolderOutput":
        entity = {**FolderProps.model_validate(self).model_dump()}
        entity["attributes"] = [q.dump() for q in self.attributes]
        return FolderOutput(**entity)

    def empty(self) -> "Folder":
        props = FolderProps()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.attributes = []
        return self

    def update(self, other: "Folder", empty: bool = False) -> "Folder":
        if empty:
            self.empty()
        props = FolderProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        return self


class FolderInputNode(InputNode):
    class Meta:
        model = FolderInput


# endregion Folder

# region Benchmark


class BenchmarkNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class BenchmarkIconField(RealField, abc.ABC):
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class BenchmarkMinField(RealField, abc.ABC):
    min: typing.Optional[float] = sqlmodel.Field(default=None)


class BenchmarkMinExcludedField(RealField, abc.ABC):
    min_excluded: bool = sqlmodel.Field(default=False)


class BenchmarkMaxField(RealField, abc.ABC):
    max: typing.Optional[float] = sqlmodel.Field(default=None)


class BenchmarkMaxExcludedField(RealField, abc.ABC):
    max_excluded: bool = sqlmodel.Field(default=False)


class BenchmarkId(BenchmarkNameField, Id):
    pass


class BenchmarkProps(BenchmarkMaxExcludedField, BenchmarkMaxField, BenchmarkMinExcludedField, BenchmarkMinField, BenchmarkIconField, BenchmarkNameField, Props):
    pass


class BenchmarkInput(BenchmarkMaxExcludedField, BenchmarkMaxField, BenchmarkMinExcludedField, BenchmarkMinField, BenchmarkIconField, BenchmarkNameField, Input):
    pass


class BenchmarkOutput(BenchmarkMaxExcludedField, BenchmarkMaxField, BenchmarkMinExcludedField, BenchmarkMinField, BenchmarkIconField, BenchmarkNameField, Output):
    pass


class Benchmark(BenchmarkMaxExcludedField, BenchmarkMaxField, BenchmarkMinExcludedField, BenchmarkMinField, BenchmarkIconField, BenchmarkNameField, TableEntity, table=True):
    PLURAL = "benchmarks"
    __tablename__ = "benchmarks"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    qualityPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("quality_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("qualities.id")), default=None, exclude=True)
    quality: Quality = sqlmodel.Relationship(back_populates="benchmarks")
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="benchmark", cascade_delete=True)


# endregion Benchmark

# region Quality


class QualityKeyField(RealField, abc.ABC):
    key: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT, primary_key=True)


class QualityNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class QualityDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class QualityUriField(RealField, abc.ABC):
    uri: str = sqlmodel.Field(default="", max_length=URI_LENGTH_LIMIT)


class QualityScalableField(RealField, abc.ABC):
    scalable: bool = sqlmodel.Field(default=False)


class QualityKindField(RealField, abc.ABC):
    kind: int = sqlmodel.Field(default=0)


class QualitySiField(RealField, abc.ABC):
    si: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class QualityImperialField(RealField, abc.ABC):
    imperial: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class QualityMinField(RealField, abc.ABC):
    min: typing.Optional[float] = sqlmodel.Field(default=None)


class QualityMinExcludedField(RealField, abc.ABC):
    min_excluded: bool = sqlmodel.Field(default=True)


class QualityMaxField(RealField, abc.ABC):
    max: typing.Optional[float] = sqlmodel.Field(default=None)


class QualityMaxExcludedField(RealField, abc.ABC):
    max_excluded: bool = sqlmodel.Field(default=True)


class QualityDefaultField(RealField, abc.ABC):
    default: typing.Optional[float] = sqlmodel.Field(default=None)


class QualityFormulaField(RealField, abc.ABC):
    formula: str = sqlmodel.Field(default="", max_length=EXPRESSION_LENGTH_LIMIT)


class QualityFolderField(RealField, abc.ABC):
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class QualityIconField(RealField, abc.ABC):
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)


class QualityImageField(RealField, abc.ABC):
    image: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)


class QualityUnitField(RealField, abc.ABC):
    unit: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class QualityCreatedField(RealField, abc.ABC):
    created_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class QualityUpdatedField(RealField, abc.ABC):
    updated_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class QualityId(QualityKeyField, Id):
    pass


class QualityProps(
    QualityUnitField,
    QualityImageField,
    QualityIconField,
    QualityFolderField,
    QualityFormulaField,
    QualityDefaultField,
    QualityMaxExcludedField,
    QualityMaxField,
    QualityMinExcludedField,
    QualityMinField,
    QualityImperialField,
    QualitySiField,
    QualityKindField,
    QualityScalableField,
    QualityUriField,
    QualityDescriptionField,
    QualityNameField,
    QualityKeyField,
    Props,
):
    pass


class QualityInput(
    QualityUnitField,
    QualityImageField,
    QualityIconField,
    QualityFolderField,
    QualityFormulaField,
    QualityDefaultField,
    QualityMaxExcludedField,
    QualityMaxField,
    QualityMinExcludedField,
    QualityMinField,
    QualityImperialField,
    QualitySiField,
    QualityKindField,
    QualityScalableField,
    QualityUriField,
    QualityDescriptionField,
    QualityNameField,
    QualityKeyField,
    Input,
):
    pass


class QualityContext(QualityDescriptionField, QualityNameField, QualityKeyField, Context):
    pass


class QualityOutput(
    QualityUpdatedField,
    QualityCreatedField,
    QualityUnitField,
    QualityImageField,
    QualityIconField,
    QualityFolderField,
    QualityFormulaField,
    QualityDefaultField,
    QualityMaxExcludedField,
    QualityMaxField,
    QualityMinExcludedField,
    QualityMinField,
    QualityImperialField,
    QualitySiField,
    QualityKindField,
    QualityScalableField,
    QualityUriField,
    QualityDescriptionField,
    QualityNameField,
    QualityKeyField,
    Output,
):
    benchmarks: list["BenchmarkOutput"] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)


class Quality(
    QualityUpdatedField,
    QualityCreatedField,
    QualityUnitField,
    QualityImageField,
    QualityIconField,
    QualityFolderField,
    QualityFormulaField,
    QualityDefaultField,
    QualityMaxExcludedField,
    QualityMaxField,
    QualityMinExcludedField,
    QualityMinField,
    QualityImperialField,
    QualitySiField,
    QualityKindField,
    QualityScalableField,
    QualityUriField,
    QualityDescriptionField,
    QualityNameField,
    QualityKeyField,
    TableEntity,
    table=True,
):
    PLURAL = "qualities"
    __tablename__ = "qualities"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    kit: Kit = sqlmodel.Relationship(back_populates="qualities")

    benchmarks: list["Benchmark"] = sqlmodel.Relationship(back_populates="quality", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="quality", cascade_delete=True)

    __table_args__ = (
        sqlalchemy.CheckConstraint("kind >= 0 AND kind <= 63", name="ck_qualities_kind_range"),
        sqlalchemy.UniqueConstraint("key", "kit_id", name="uq_qualities_key_kit_id"),
    )


# endregion Quality

# region Prop


class PropKeyField(RealField, abc.ABC):
    key: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class PropValueField(RealField, abc.ABC):
    value: str = sqlmodel.Field(max_length=VALUE_LENGTH_LIMIT)


class PropUnitField(RealField, abc.ABC):
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class PropCreatedField(RealField, abc.ABC):
    created_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class PropUpdatedField(RealField, abc.ABC):
    updated_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class PropId(PropKeyField, Id):
    pass


class PropProps(PropUpdatedField, PropCreatedField, PropUnitField, PropValueField, PropKeyField, Props):
    pass


class PropInput(PropUnitField, PropValueField, PropKeyField, Input):
    pass


class PropOutput(PropUpdatedField, PropCreatedField, PropUnitField, PropValueField, PropKeyField, Output):
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)


class Prop(PropUpdatedField, PropCreatedField, PropUnitField, PropValueField, PropKeyField, TableEntity, table=True):
    PLURAL = "props"
    __tablename__ = "props"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    connectorPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("connector_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("connectors.id")), default=None, exclude=True)
    connector: Connector = sqlmodel.Relationship(back_populates="props")
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id")), default=None, exclude=True)
    type: Type = sqlmodel.Relationship(back_populates="props")
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: Design = sqlmodel.Relationship(back_populates="props")

    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="prop", cascade_delete=True)

    __table_args__ = (
        sqlalchemy.CheckConstraint(
            """
        (
            (connector_id IS NOT NULL AND type_id IS NULL AND design_id IS NULL)
        OR
            (connector_id IS NULL AND type_id IS NOT NULL AND design_id IS NULL)
        OR
            (connector_id IS NULL AND type_id IS NULL AND design_id IS NOT NULL)
        )
        """,
            name="ck_props_parent_set",
        ),
    )

    def parent(self) -> typing.Union["Connector", "Type", "Design"]:
        if self.connector is not None:
            return self.connector
        if self.type is not None:
            return self.type
        if self.design is not None:
            return self.design
        raise NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.key

    @classmethod
    def parse(cls, input: str | dict | PropInput | typing.Any | None) -> "Prop":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        props = PropProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            entity.attributes = [typing.cast(Attribute, Attribute.parse(attribute)) for attribute in obj["attributes"]]
        except KeyError:
            pass
        return entity

    def dump(self) -> "PropOutput":
        entity = {**PropProps.model_validate(self).model_dump()}
        entity["attributes"] = [q.dump() for q in self.attributes]
        return PropOutput(**entity)


class PropInputNode(InputNode):
    class Meta:
        model = PropInput


# endregion Prop

# region Model


class ModelNameField(RealField, abc.ABC):
    name: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class ModelUrlField(RealField, abc.ABC):
    url: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT)


class ModelFileField(RealField, abc.ABC):
    file: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)


class ModelDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ModelTagsField(MaskedField, abc.ABC):
    tags: list[str] = sqlmodel.Field(default_factory=list)


class ModelId(ModelTagsField, Id):
    pass


class ModelProps(ModelTagsField, ModelDescriptionField, ModelNameField, ModelFileField, ModelUrlField, Props):
    pass


class ModelInput(ModelTagsField, ModelDescriptionField, ModelNameField, ModelFileField, ModelUrlField, Input):
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)


class ModelContext(ModelTagsField, ModelDescriptionField, ModelNameField, Context):
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)


class ModelOutput(ModelTagsField, ModelDescriptionField, ModelNameField, ModelFileField, ModelUrlField, Output):
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)


class Model(ModelDescriptionField, ModelNameField, ModelFileField, ModelUrlField, TableEntity, table=True):
    PLURAL = "models"
    __tablename__ = "models"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    tags_: list[Tag] = sqlmodel.Relationship(back_populates="model", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="model", cascade_delete=True)
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id")), default=None, exclude=True)
    type: Type = sqlmodel.Relationship(back_populates="models")

    @property
    def tags(self: "Model") -> list[str]:
        return [tag.name for tag in sorted(self.tags_, key=lambda x: x.order)]

    @tags.setter
    def tags(self: "Model", tags: list[str]):
        self.tags_ = [Tag(name=tag, order=i) for i, tag in enumerate(tags)]

    def parent(self: "Model") -> "Type":
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls, input: str | dict | ModelInput | typing.Any | None) -> "Model":
        if input is None:
            return cls(url="", file="")
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        props = ModelProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            entity.tags = obj["tags"]
        except KeyError:
            pass
        try:
            entity.attributes = [typing.cast(Attribute, Attribute.parse(attribute)) for attribute in obj["attributes"]]
        except KeyError:
            pass
        return entity

    def dump(self) -> "ModelOutput":
        entity = {**ModelProps.model_validate(self).model_dump()}
        #  TODO: Fix bug with tags not being dumped correctly.

        entity["attributes"] = [q.dump() for q in self.attributes]
        return ModelOutput(**entity)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return [self.tags]


class NoModelAssigned(NoParentAssigned):
    def __str__(self):
        return "👪 The entity has no parent model assigned."


class ModelInputNode(InputNode):
    class Meta:
        model = ModelInput


# endregion Model

# region Interface


class InterfaceNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class InterfaceDescriptionField(RealField, abc.ABC):
    description: typing.Optional[str] = sqlmodel.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)


class InterfaceIconField(RealField, abc.ABC):
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)


class InterfaceCompatibleInterfacesField(MaskedField, abc.ABC):
    compatibleInterfaces: list[str] = sqlmodel.Field(default_factory=list)


class InterfaceId(InterfaceNameField, Id):
    pass


class InterfaceProps(InterfaceCompatibleInterfacesField, InterfaceIconField, InterfaceDescriptionField, InterfaceNameField, Props):
    pass


class InterfaceInput(InterfaceCompatibleInterfacesField, InterfaceIconField, InterfaceDescriptionField, InterfaceNameField, Input):
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)


class InterfaceOutput(InterfaceCompatibleInterfacesField, InterfaceIconField, InterfaceDescriptionField, InterfaceNameField, Output):
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)


class Interface(InterfaceIconField, InterfaceDescriptionField, InterfaceNameField, TableEntity, table=True):
    PLURAL = "interfaces"
    __tablename__ = "interfaces"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="interface_", cascade_delete=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    kit: Kit = sqlmodel.Relationship(back_populates="interfaces")


# TODO: Fix InterfaceNode - was incorrectly changed to TableEntityNode in latest commit


class InterfaceInputNode(InputNode):
    class Meta:
        model = InterfaceInput


# endregion Interface

# region Connector


# region CompatibleInterface


class CompatibleInterfaceNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class CompatibleInterfaceOrderField(RealField, abc.ABC):
    order: int = sqlmodel.Field()


class CompatibleInterface(CompatibleInterfaceOrderField, CompatibleInterfaceNameField, Table, table=True):
    __tablename__ = "compatible_interfaces"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    connectorPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("connector_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("connectors.id")), default=None, exclude=True)
    connector: Connector = sqlmodel.Relationship(back_populates="compatibleInterfaces_")


# endregion CompatibleInterface


class ConnectorIdField(MaskedField, abc.ABC):
    id_: str = sqlmodel.Field(default="", max_length=ID_LENGTH_LIMIT)


class ConnectorDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ConnectorMandatoryField(RealField, abc.ABC):
    is_mandatory: bool = sqlmodel.Field(default=False)


class ConnectorInterfaceField(RealField, abc.ABC):
    interface: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class ConnectorCompatibleInterfacesField(MaskedField, abc.ABC):
    compatibleInterfaces: list[str] = sqlmodel.Field(default_factory=list)


class ConnectorPointField(MaskedField, abc.ABC):
    point: Point = sqlmodel.Field()


class ConnectorDirectionField(MaskedField, abc.ABC):
    direction: Vector = sqlmodel.Field()


class ConnectorTField(RealField, abc.ABC):
    t: float = sqlmodel.Field(default=0.0)


class ConnectorId(ConnectorIdField, Id):
    pass


class ConnectorProps(ConnectorTField, ConnectorCompatibleInterfacesField, ConnectorInterfaceField, ConnectorMandatoryField, ConnectorDescriptionField, ConnectorIdField, Props):
    pass


class ConnectorInput(ConnectorTField, ConnectorCompatibleInterfacesField, ConnectorInterfaceField, ConnectorMandatoryField, ConnectorDescriptionField, ConnectorIdField, Input):
    point: PointInput = sqlmodel.Field()
    direction: VectorInput = sqlmodel.Field()
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)


class ConnectorContext(ConnectorTField, ConnectorDirectionField, ConnectorPointField, ConnectorCompatibleInterfacesField, ConnectorInterfaceField, ConnectorMandatoryField, ConnectorDescriptionField, ConnectorIdField, Context):
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)


class ConnectorOutput(ConnectorTField, ConnectorDirectionField, ConnectorPointField, ConnectorCompatibleInterfacesField, ConnectorInterfaceField, ConnectorMandatoryField, ConnectorDescriptionField, ConnectorIdField, Output):
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)


class Connector(ConnectorTField, ConnectorInterfaceField, ConnectorMandatoryField, ConnectorDescriptionField, TableEntity, table=True):
    PLURAL = "connectors"
    __tablename__ = "connectors"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)

    id_: str = sqlmodel.Field(
        # alias="id",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column("local_id", sqlalchemy.String(ID_LENGTH_LIMIT)),
        default="",
    )
    compatibleInterfaces_: list[CompatibleInterface] = sqlmodel.Relationship(back_populates="connector", cascade_delete=True)
    pointX: float = sqlmodel.Field(sa_column=sqlmodel.Column("point_x", sqlalchemy.String(ID_LENGTH_LIMIT)), exclude=True)
    pointY: float = sqlmodel.Field(sa_column=sqlmodel.Column("point_y", sqlalchemy.Float()), exclude=True)
    pointZ: float = sqlmodel.Field(sa_column=sqlmodel.Column("point_z", sqlalchemy.Float()), exclude=True)
    directionX: float = sqlmodel.Field(sa_column=sqlmodel.Column("direction_x", sqlalchemy.Float()), exclude=True)
    directionY: float = sqlmodel.Field(sa_column=sqlmodel.Column("direction_y", sqlalchemy.Float()), exclude=True)
    directionZ: float = sqlmodel.Field(sa_column=sqlmodel.Column("direction_z", sqlalchemy.Float()), exclude=True)
    attributes: list["Attribute"] = sqlmodel.Relationship(back_populates="connector", cascade_delete=True)
    props: list["Prop"] = sqlmodel.Relationship(back_populates="connector", cascade_delete=True)
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id")), default=None, exclude=True)
    type: Type = sqlmodel.Relationship(back_populates="connectors")
    connecteds: list["Connection"] = sqlmodel.Relationship(back_populates="connectedConnector", sa_relationship_kwargs={"foreign_keys": "Connection.connectedConnectorPk"})
    connectings: list["Connection"] = sqlmodel.Relationship(back_populates="connectingConnector", sa_relationship_kwargs={"foreign_keys": "Connection.connectingConnectorPk"})

    __table_args__ = (sqlalchemy.UniqueConstraint("local_id", "type_id", name="uq_connectors_local_id_type_id"),)

    @property
    def compatibleInterfaces(self) -> list[str]:
        return sorted([cf.name for cf in self.compatibleInterfaces_], key=lambda cf: cf.order)

    @compatibleInterfaces.setter
    def compatibleInterfaces(self, compatibleInterfaces: list[str]):
        self.compatibleInterfaces_ = [CompatibleInterface(name=cf, order=i) for i, cf in enumerate(compatibleInterfaces)]

    @property
    def point(self) -> Point:
        return Point(x=self.pointX, y=self.pointY, z=self.pointZ)

    @point.setter
    def point(self, point: Point):
        self.pointX = point.x
        self.pointY = point.y
        self.pointZ = point.z

    @property
    def direction(self) -> Vector:
        return Vector(x=self.directionX, y=self.directionY, z=self.directionZ)

    @direction.setter
    def direction(self, direction: Vector):
        self.directionX = direction.x
        self.directionY = direction.y
        self.directionZ = direction.z

    @property
    def connections(self) -> list["Connection"]:
        return self.connecteds + self.connectings

    def parent(self) -> "Type":
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Connector", input: str | dict | ConnectorInput | typing.Any | None) -> "Connector":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        interface_obj = obj.get("interface")
        interface_guid = interface_obj.get("guid") if isinstance(interface_obj, dict) else interface_obj if isinstance(interface_obj, str) else None
        entity = cls(
            id_=obj.get("id_", obj.get("name", "")),
            description=obj.get("description", ""),
            is_mandatory=obj.get("mandatory", False),
            interface=interface_guid,
            t=obj.get("t", 0.0),
        )
        point = Point.parse(obj["point"])
        direction = Vector.parse(obj["direction"])
        entity.point = point
        entity.direction = direction
        try:
            entity.compatibleInterfaces = obj["compatibleInterfaces"]
        except KeyError:
            pass
        try:
            attrs = [Attribute.parse(attr) for attr in obj.get("attributes", [])]
            if attrs:
                entity.attributes = attrs
        except KeyError:
            pass
        return entity

    def dump(self) -> "ConnectorOutput":
        entity = {**ConnectorProps.model_validate(self).model_dump()}
        entity["point"] = self.point.dump()
        entity["direction"] = self.direction.dump()
        entity["compatibleInterfaces"] = self.compatibleInterfaces
        entity["attributes"] = [q.dump() for q in self.attributes]
        return ConnectorOutput(**entity)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return self.id_


class ConnectorNotFound(NotFound):
    def __init__(self, parent: "Type", id: "ConnectorId") -> None:
        self.parent = parent
        self.id = id

    def __str__(self):
        variant = f", {self.parent.variant}" if self.parent.variant else ""
        return f"🔍 Couldn't find the connector ({self.id.id_}) inside the parent type ({self.parent.name}{variant})."


class ConnectorInputNode(InputNode):
    class Meta:
        model = ConnectorInput


class ConnectorIdInputNode(InputNode):
    class Meta:
        model = ConnectorId


# endregion Connector

# region Type


class TypeNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class TypeDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class TypeIconField(RealField, abc.ABC):
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class TypeImageField(RealField, abc.ABC):
    image: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class TypeVariantField(RealField, abc.ABC):
    variant: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class TypeParentField(RealField, abc.ABC):
    parent: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)


class TypeIsAbstractField(RealField, abc.ABC):
    is_abstract: bool = sqlmodel.Field(default=False)


class TypeFolderField(RealField, abc.ABC):
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)


class TypeStockField(RealField, abc.ABC):
    stock: int = sqlmodel.Field(default=2147483647)


class TypeVirtualField(RealField, abc.ABC):
    is_virtual: bool = sqlmodel.Field(default=False)


class TypeScalableField(RealField, abc.ABC):
    can_scale: bool = sqlmodel.Field(default=True)


class TypeMirrborableField(RealField, abc.ABC):
    can_mirror: bool = sqlmodel.Field(default=True)


class TypeUnitField(RealField, abc.ABC):
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class TypeLocationField(MaskedField, abc.ABC):
    location: typing.Optional[Location] = sqlmodel.Field(default=None)


class TypeCreatedField(RealField, abc.ABC):
    created_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class TypeUpdatedField(RealField, abc.ABC):
    updated_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class TypeId(TypeVariantField, TypeNameField, Id):
    pass


class TypeProps(TypeUnitField, TypeLocationField, TypeFolderField, TypeIsAbstractField, TypeParentField, TypeVirtualField, TypeStockField, TypeVariantField, TypeImageField, TypeIconField, TypeDescriptionField, TypeNameField, Props):
    pass


class TypeInput(TypeUnitField, TypeVirtualField, TypeStockField, TypeVariantField, TypeImageField, TypeIconField, TypeDescriptionField, TypeNameField, Input):
    parent: typing.Optional[str] = sqlmodel.Field(default=None)
    is_abstract: typing.Optional[bool] = sqlmodel.Field(default=None)
    folder: typing.Optional[str] = sqlmodel.Field(default=None)
    location: typing.Optional[LocationInput] = sqlmodel.Field(default=None)
    models: list[ModelInput] = sqlmodel.Field(default_factory=list)
    connectors: list[ConnectorInput] = sqlmodel.Field(default_factory=list)
    props: list[PropInput] = sqlmodel.Field(default_factory=list)
    authors: list[str] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class TypeOutput(TypeUpdatedField, TypeCreatedField, TypeUnitField, TypeVirtualField, TypeStockField, TypeVariantField, TypeImageField, TypeIconField, TypeDescriptionField, TypeNameField, Output):
    parent: typing.Optional[str] = sqlmodel.Field(default=None)
    is_abstract: typing.Optional[bool] = sqlmodel.Field(default=None)
    folder: typing.Optional[str] = sqlmodel.Field(default=None)
    location: typing.Optional[LocationOutput] = sqlmodel.Field(default=None)
    models: list[ModelOutput] = sqlmodel.Field(default_factory=list)
    connectors: list[ConnectorOutput] = sqlmodel.Field(default_factory=list)
    props: list[PropOutput] = sqlmodel.Field(default_factory=list)
    authors: list[str] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class TypeContext(TypeUnitField, TypeVirtualField, TypeStockField, TypeVariantField, TypeDescriptionField, TypeNameField, Context):
    location: typing.Optional[LocationContext] = sqlmodel.Field(default=None)
    connectors: list[ConnectorContext] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class Type(
    TypeUpdatedField,
    TypeCreatedField,
    TypeUnitField,
    TypeMirrborableField,
    TypeScalableField,
    TypeVirtualField,
    TypeStockField,
    TypeVariantField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    TypeFolderField,
    TypeIsAbstractField,
    TypeParentField,
    TableEntity,
    table=True,
):
    PLURAL = "types"
    __tablename__ = "types"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)

    locationLongitude: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("location_longitude", sqlalchemy.Float()), exclude=True, default=None)

    locationLatitude: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("location_latitude", sqlalchemy.Float()), exclude=True, default=None)

    models: list[Model] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    connectors: list[Connector] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    props: list["Prop"] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    artifact_authors: list[ArtifactAuthor] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    kitPk: typing.Optional[int] = sqlmodel.Field(
        # alias="kitId", # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")),
        default=None,
        exclude=True,
    )

    kit: Kit = sqlmodel.Relationship(back_populates="types")

    pieces: list["Piece"] = sqlmodel.Relationship(back_populates="type")

    concepts_: list[Concept] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    __table_args__ = (sqlalchemy.UniqueConstraint("name", "variant", "kit_id", name="uq_types_name_variant_kit_id"),)

    @property
    def location(self) -> typing.Optional[Location]:
        if self.locationLongitude is None and self.locationLatitude is None:
            return None
        if self.locationLongitude is None:
            raise ValueError("Location longitude is required")
        if self.locationLatitude is None:
            raise ValueError("Location latitude is required")
        return Location(
            longitude=self.locationLongitude,
            latitude=self.locationLatitude,
        )

    @location.setter
    def location(self, location: typing.Optional[Location]):
        if location is None:
            self.locationLongitude = None
            self.locationLatitude = None
        else:
            self.locationLongitude = location.longitude
            self.locationLatitude = location.latitude

    @property
    def authors(self) -> list[str]:
        return [artifact_author.author_email for artifact_author in self.artifact_authors]

    @authors.setter
    def authors(self, author_emails: list[str]):
        self.artifact_authors = [ArtifactAuthor(author_email=email) for email in author_emails]

    @property
    def concepts(self: "Type") -> list[str]:
        return [concept.name for concept in sorted(self.concepts_, key=lambda x: x.order)]

    @concepts.setter
    def concepts(self: "Type", concepts: list[str]):
        self.concepts_ = [Concept(name=concept, order=i) for i, concept in enumerate(concepts)]

    def parent(self) -> "Kit":
        if self.kit is None:
            raise NoKitAssigned()
        return self.kit

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Type", input: str | dict | TypeInput | typing.Any | None) -> "Type":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        parent_obj = obj.get("parent")
        parent_guid = parent_obj.get("guid") if isinstance(parent_obj, dict) else parent_obj if isinstance(parent_obj, str) else None
        folder_obj = obj.get("folder")
        folder_guid = folder_obj.get("guid") if isinstance(folder_obj, dict) else folder_obj if isinstance(folder_obj, str) else None
        entity = cls(
            name=obj.get("name", ""),
            variant=obj.get("variant", ""),
            description=obj.get("description", ""),
            icon=obj.get("icon", ""),
            image=obj.get("image", ""),
            isAbstract=obj.get("isAbstract", False),
            isVirtual=obj.get("isVirtual", False),
            stock=obj.get("stock"),
            unit=obj.get("unit", ""),
            parent=parent_guid,
            folder=folder_guid,
        )
        try:
            location_obj = obj.get("location")
            if location_obj:
                entity.location = Location.parse(location_obj) if isinstance(location_obj, dict) else location_obj
        except (KeyError, AttributeError):
            pass
        try:
            models = [Model.parse(r) for r in obj["models"]]
            entity.models = models
        except KeyError:
            pass
        try:
            connectors = [Connector.parse(p) for p in obj["connectors"]]
            entity.connectors = connectors
        except KeyError:
            pass
        try:
            props = [Prop.parse(p) for p in obj["props"]]
            entity.props = props
        except KeyError:
            pass
        try:
            entity.attributes = [Attribute.parse(q) for q in obj["attributes"]]
        except KeyError:
            pass
        try:
            author_emails = obj["authors"]
            entity.authors = author_emails
        except KeyError:
            pass
        try:
            concepts = obj["concepts"]
            entity.concepts = concepts
        except KeyError:
            pass

        return entity

    def dump(self) -> "TypeOutput":
        entity = {**TypeProps.model_validate(self).model_dump()}
        entity["models"] = [r.dump() for r in self.models]
        entity["connectors"] = [p.dump() for p in self.connectors]
        entity["props"] = [p.dump() for p in self.props]
        entity["attributes"] = [q.dump() for q in self.attributes]
        entity["authors"] = self.authors
        entity["concepts"] = self.concepts
        return TypeOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        props = TypeProps()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.types = []
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Type", empty: bool = False) -> "Type":
        if empty:
            self.empty()
        props = TypeProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return [self.name, self.variant]


class TypeNotFound(NotFound):
    def __init__(self, id: "TypeId") -> None:
        self.id = id

    def __str__(self):
        variant = f", {self.id.variant}" if self.id.variant else ""
        return f"🔍 Couldn't find the type ({self.id.name}{variant})."


class NoTypeAssigned(NoParentAssigned):
    def __str__(self):
        return "👪 The entity has no parent type assigned."


class TypeHasNotAllUsedConnectors(SpecificationError):
    def __init__(self, missingConnectors: set[str]) -> None:
        self.missingConnectors = missingConnectors

    def __str__(self) -> str:
        return f"🚫 A design is using some connectors of the type. The new type is missing the following connectors: {', '.join(self.missingConnectors)}."


class TypeInputNode(InputNode):
    class Meta:
        model = TypeInput


class TypeIdInputNode(InputNode):
    class Meta:
        model = TypeId


# endregion Type

# region Layer


class LayerNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class LayerDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class LayerColorField(RealField, abc.ABC):
    color: str = sqlmodel.Field(default="", max_length=7)


class LayerIsHiddenField(RealField, abc.ABC):
    is_hidden: bool = sqlmodel.Field(default=False)


class LayerIsLockedField(RealField, abc.ABC):
    is_locked: bool = sqlmodel.Field(default=False)


class LayerId(LayerNameField, Id):
    pass


class LayerProps(LayerIsLockedField, LayerIsHiddenField, LayerColorField, LayerDescriptionField, LayerNameField, Props):
    pass


class LayerInput(LayerIsLockedField, LayerIsHiddenField, LayerColorField, LayerDescriptionField, LayerNameField, Input):
    pass


class LayerOutput(LayerIsLockedField, LayerIsHiddenField, LayerColorField, LayerDescriptionField, LayerNameField, Output):
    pass


class Layer(LayerIsLockedField, LayerIsHiddenField, LayerColorField, LayerDescriptionField, LayerNameField, TableEntity, table=True):
    PLURAL = "layers"
    __tablename__ = "layers"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: Design = sqlmodel.Relationship(back_populates="layers")


# endregion Layer

# region Piece


class PieceIdField(MaskedField, abc.ABC):
    id_: str = sqlmodel.Field(
        default="",
        # alias="id", # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        max_length=ID_LENGTH_LIMIT,
    )


class PieceDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class PieceTypeField(MaskedField, abc.ABC):
    type: typing.Optional[TypeId] = sqlmodel.Field(default=None)


class PieceDesignField(MaskedField, abc.ABC):
    designPiece: typing.Optional["DesignId"] = sqlmodel.Field(default=None)


class PiecePlaneField(MaskedField, abc.ABC):
    plane: typing.Optional[Plane] = sqlmodel.Field(default=None)


class PieceCenterField(MaskedField, abc.ABC):
    center: typing.Optional[Coord] = sqlmodel.Field(default=None)


class PieceScaleField(RealField, abc.ABC):
    scale: float = sqlmodel.Field(default=1.0)


class PieceMirrorPlaneField(MaskedField, abc.ABC):
    mirrorPlane: typing.Optional[Plane] = sqlmodel.Field(default=None)


class PieceHiddenField(RealField, abc.ABC):
    is_hidden: bool = sqlmodel.Field(default=False)


class PieceLockedField(RealField, abc.ABC):
    is_locked: bool = sqlmodel.Field(default=False)


class PieceColorField(RealField, abc.ABC):
    color: str = sqlmodel.Field(default="", max_length=7)


class PieceId(PieceIdField, Id):
    pass


class PieceProps(PieceCenterField, PiecePlaneField, PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Props):
    pass


class PieceInput(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Input):
    plane: typing.Optional[PlaneInput] = sqlmodel.Field(default=None)
    center: typing.Optional[CoordInput] = sqlmodel.Field(default=None)
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)


class PieceContext(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Context):
    plane: typing.Optional[PlaneContext] = sqlmodel.Field(default=None)
    center: typing.Optional[CoordContext] = sqlmodel.Field(default=None)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)


class PieceOutput(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Output):
    plane: typing.Optional[PlaneOutput] = sqlmodel.Field(default=None)
    center: typing.Optional[CoordOutput] = sqlmodel.Field(default=None)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)


class PiecePrediction(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Prediction):
    pass


class Piece(PieceIdField, PieceHiddenField, PieceLockedField, PieceColorField, PieceScaleField, TableEntity, table=True):
    PLURAL = "pieces"
    __tablename__ = "pieces"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    id_: str = sqlmodel.Field(sa_column=sqlmodel.Column("local_id", sqlalchemy.String(ID_LENGTH_LIMIT)), default="")
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id"), nullable=True), default=None, exclude=True)
    type: Type = sqlmodel.Relationship(back_populates="pieces")
    designPiecePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id"), nullable=True), default=None, exclude=True)
    designPiece: Design = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Design", foreign_keys="[Piece.designPiecePk]"))
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: Design = sqlmodel.Relationship(back_populates="pieces", sa_relationship_kwargs={"foreign_keys": "[Piece.designPk]"})
    planePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("plane_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("planes.id"), nullable=True), default=None, exclude=True)
    plane: Plane = sqlmodel.Relationship(back_populates="piece")
    centerU: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("center_x", sqlalchemy.Float()), exclude=True)
    centerV: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("center_y", sqlalchemy.Float()), exclude=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="piece", cascade_delete=True)
    connecteds: list["Connection"] = sqlmodel.Relationship(back_populates="connectedPiece", sa_relationship_kwargs={"foreign_keys": "Connection.connectedPiecePk"})
    connectings: list["Connection"] = sqlmodel.Relationship(back_populates="connectingPiece", sa_relationship_kwargs={"foreign_keys": "Connection.connectingPiecePk"})

    __table_args__ = (sqlalchemy.UniqueConstraint("local_id", "design_id", name="uq_pieces_local_id_design_id"),)

    @property
    def center(self) -> typing.Optional[Coord]:
        if self.centerU is None or self.centerV is None:
            return None
        return Coord(u=self.centerU, v=self.centerV)

    @center.setter
    def center(self, center: typing.Optional[Coord]):
        if center is None:
            self.centerU = None
            self.centerV = None
            return
        self.centerU = center.u
        self.centerV = center.v

    @property
    def connections(self) -> list["Connection"]:
        return self.connecteds + self.connectings

    def parent(self) -> "Design":
        if self.design is None:
            raise NoParentAssigned()
        return self.design

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Piece", input: str | dict | PieceInput | typing.Any | None, types: dict[str, dict[str, Type]], designs: typing.Optional[dict[str, dict[str, dict[str, "Design"]]]] = None) -> "Piece":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        entity = cls(id_=obj["id_"])
        typeObj = obj.get("type", None)
        designObj = obj.get("designPiece", None)
        if (typeObj is None and designObj is None) or (typeObj is not None and designObj is not None):
            raise ValueError("Exactly one of 'type' or 'designPiece' must be provided for a Piece.")
        if typeObj is not None:
            typeId = TypeId.parse(typeObj)
            try:
                entity.type = types[typeId.name][typeId.variant]
            except KeyError:
                raise TypeNotFound(typeId)
        else:
            if designs is None:
                raise FeatureNotYetSupported()
            designId = DesignId.parse(designObj)
            try:
                entity.designPiece = designs[designId.name][designId.variant][designId.view]
            except KeyError:
                raise FeatureNotYetSupported()
        try:
            if obj["plane"] is not None:
                plane = Plane.parse(obj["plane"])
                # TODO: Proper mechanism of nullable fields.
                if plane.originX is not None:
                    entity.plane = plane
        except KeyError:
            pass
        try:
            if obj["center"] is not None:
                center = Coord.parse(obj["center"])
                entity.center = center
        except KeyError:
            pass
        return entity

    def dump(self) -> "PieceOutput":
        entity = {**PieceProps.model_validate(self).model_dump()}
        entity["plane"] = self.plane.dump() if self.plane is not None else None
        entity["center"] = self.center.dump() if self.center is not None else None
        entity["attributes"] = [q.dump() for q in self.attributes]
        return PieceOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Piece":
        props = PieceProps()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Piece", empty: bool = False) -> "Piece":
        if empty:
            self.empty()
        props = PieceProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return self.id_


class PieceInputNode(InputNode):
    class Meta:
        model = PieceInput
        exclude_fields = ("type", "designPiece")

    type = TypeIdInputNode()
    designPiece = graphene.Field(lambda: DesignIdInputNode)


class PieceIdInputNode(InputNode):
    class Meta:
        model = PieceId


# endregion Piece

# region Group


class GroupNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class GroupDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class GroupColorField(RealField, abc.ABC):
    color: str = sqlmodel.Field(default="", max_length=7)


class GroupId(GroupNameField, Id):
    pass


class GroupProps(GroupColorField, GroupDescriptionField, GroupNameField, Props):
    pass


class GroupInput(GroupColorField, GroupDescriptionField, GroupNameField, Input):
    pass


class GroupOutput(GroupColorField, GroupDescriptionField, GroupNameField, Output):
    pieces: list["PieceOutput"] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)


class Group(GroupColorField, GroupDescriptionField, GroupNameField, TableEntity, table=True):
    PLURAL = "groups"
    __tablename__ = "groups"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: Design = sqlmodel.Relationship(back_populates="groups")


# endregion Group

# region Side


class Side(BaseModel):
    piece: PieceId = sqlmodel.Field()
    designPiece: typing.Optional[PieceId] = sqlmodel.Field(default=None)
    connector: ConnectorId = sqlmodel.Field()

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Side", input: str | dict | typing.Any | None) -> "Side":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        piece = PieceId.parse(obj["piece"])
        connector = ConnectorId.parse(obj["connector"])
        try:
            designPieceObj = obj["designPiece"]
            designPiece = PieceId.parse(designPieceObj) if designPieceObj is not None else None
        except KeyError:
            designPiece = None
        return cls(piece=piece, designPiece=designPiece, connector=connector)


class SideInput(Side, Input):
    pass


class SideContext(Side, Context):
    pass


class SideOutput(Side, Output):
    pass


class SidePrediction(Side, Prediction):
    pass


class SideNode(Node):
    class Meta:
        model = Side

    exclude_fields = ("piece", "connector")

    piece = graphene.NonNull(lambda: PieceNode)
    designPiece = graphene.Field(lambda: PieceNode)
    connector = graphene.NonNull(lambda: ConnectorNode)

    def resolve_piece(self, info):
        return self.piece

    def resolve_designPiece(self, info):
        return self.designPiece

    def resolve_connector(self, info):
        return self.connector


class SideInputNode(InputNode):
    class Meta:
        model = SideInput

    exclude_fields = ("piece", "connector")

    piece = graphene.NonNull(PieceIdInputNode)
    designPiece = PieceIdInputNode()
    connector = graphene.NonNull(ConnectorIdInputNode)


# endregion Side

# region Connection


class ConnectionConnectedField(MaskedField, abc.ABC):
    connected: Side = sqlmodel.Field()


class ConnectionConnectingField(MaskedField, abc.ABC):
    connecting: Side = sqlmodel.Field()


class ConnectionDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ConnectionGapField(RealField, abc.ABC):
    gap: float = sqlmodel.Field(default=0)


class ConnectionShiftField(RealField, abc.ABC):
    shift: float = sqlmodel.Field(default=0)


class ConnectionRiseField(MaskedField, abc.ABC):
    rise: float = sqlmodel.Field(default=0)


class ConnectionRotationField(RealField, abc.ABC):
    rotation: float = sqlmodel.Field(ge=0, lt=360, default=0)


class ConnectionTurnField(RealField, abc.ABC):
    turn: float = sqlmodel.Field(ge=0, lt=360, default=0)


class ConnectionTiltField(RealField, abc.ABC):
    tilt: float = sqlmodel.Field(ge=0, lt=360, default=0)


class ConnectionUField(RealField, abc.ABC):
    u: float = sqlmodel.Field(default=0)


class ConnectionVField(RealField, abc.ABC):
    v: float = sqlmodel.Field(default=0)


class ConnectionId(ConnectionConnectedField, ConnectionConnectingField, Id):
    pass


class ConnectionProps(ConnectionVField, ConnectionUField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, Props):
    pass


class ConnectionInput(ConnectionVField, ConnectionUField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, Input):
    pass

    connected: SideInput = sqlmodel.Field()
    connecting: SideInput = sqlmodel.Field()


class ConnectionContext(ConnectionVField, ConnectionUField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, Context):
    pass

    connected: SideContext = sqlmodel.Field()
    connecting: SideContext = sqlmodel.Field()


class ConnectionOutput(ConnectionVField, ConnectionUField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, Output):
    pass

    connected: SideOutput = sqlmodel.Field()
    connecting: SideOutput = sqlmodel.Field()


class ConnectionPrediction(ConnectionVField, ConnectionUField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, Prediction):
    pass

    connected: SidePrediction = sqlmodel.Field()
    connecting: SidePrediction = sqlmodel.Field()


class Connection(ConnectionVField, ConnectionUField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, TableEntity, table=True):
    PLURAL = "connections"
    __tablename__ = "connections"

    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    connectedPiecePk: typing.Optional[int] = sqlmodel.Field(alias="connectedPieceId", sa_column=sqlmodel.Column("connected_piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("pieces.id")), default=None, exclude=True)
    connectedPiece: Piece = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Piece", back_populates="connecteds", foreign_keys="[Connection.connectedPiecePk]"))
    connectedConnectorPk: typing.Optional[int] = sqlmodel.Field(alias="connectedConnectorId", sa_column=sqlmodel.Column("connected_connector_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("connectors.id")), default=None, exclude=True)
    connectedConnector: Connector = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Connector", back_populates="connecteds", foreign_keys="[Connection.connectedConnectorPk]"))
    connectedDesignPiecePk: typing.Optional[int] = sqlmodel.Field(
        alias="connectedDesignPieceId", sa_column=sqlmodel.Column("connected_design_piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("pieces.id"), nullable=True), default=None, exclude=True
    )
    connectedDesignPiece: Piece = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Piece", foreign_keys="[Connection.connectedDesignPiecePk]"))
    connectingPiecePk: typing.Optional[int] = sqlmodel.Field(alias="connectingPieceId", sa_column=sqlmodel.Column("connecting_piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("pieces.id")), exclude=True, default=None)
    connectingPiece: Piece = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Piece", back_populates="connectings", foreign_keys="[Connection.connectingPiecePk]"))
    connectingConnectorPk: typing.Optional[int] = sqlmodel.Field(alias="connectingConnectorId", sa_column=sqlmodel.Column("connecting_connector_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("connectors.id")), default=None, exclude=True)
    connectingConnector: Connector = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Connector", back_populates="connectings", foreign_keys="[Connection.connectingConnectorPk]"))
    connectingDesignPiecePk: typing.Optional[int] = sqlmodel.Field(
        alias="connectingDesignPieceId", sa_column=sqlmodel.Column("connecting_design_piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("pieces.id"), nullable=True), default=None, exclude=True
    )
    connectingDesignPiece: Piece = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Piece", foreign_keys="[Connection.connectingDesignPiecePk]"))
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="connection", cascade_delete=True)
    designPk: typing.Optional[int] = sqlmodel.Field(alias="designId", sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: Design = sqlmodel.Relationship(back_populates="connections")
    __table_args__ = (
        sqlalchemy.UniqueConstraint(
            "connected_piece_id", "connected_design_piece_id", "connecting_piece_id", "connecting_design_piece_id", name="uq_connections_connected_piece_id_connected_design_piece_id_connecting_piece_id_connecting_design_piece_id"
        ),
        sqlalchemy.CheckConstraint("connected_piece_id != connecting_piece_id", name="ck_connections_not_reflexive"),
    )

    @property
    def connected(self) -> Side:
        return Side(
            piece=self.connectedPiece,
            designPiece=(PieceId(id_=self.connectedDesignPiece.id_) if self.connectedDesignPiece is not None else None),
            connector=self.connectedConnector,
        )

    @property
    def connecting(self) -> Side:
        return Side(
            piece=self.connectingPiece,
            designPiece=(PieceId(id_=self.connectingDesignPiece.id_) if self.connectingDesignPiece is not None else None),
            connector=self.connectingConnector,
        )

    def parent(self) -> "Design":
        if self.design is None:
            raise NoDesignAssigned()
        return self.design

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Connection", input: str | dict | ConnectionInput | typing.Any | None, pieces: list[Piece], designsById: typing.Optional[dict[str, dict[str, dict[str, Design]]]] = None) -> "Connection":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        piecesDict = {p.id_: p for p in pieces}
        connected = Side.parse(obj["connected"])
        connecting = Side.parse(obj["connecting"])
        connectedPiece = piecesDict[connected.piece.id_]
        connectedType = connectedPiece.type
        if connectedType is None:
            raise FeatureNotYetSupported()
        connectedConnector = [p for p in connectedType.connectors if p.id_ == connected.connector.id_]
        if len(connectedConnector) == 0:
            raise ConnectorNotFound(connectedType, connected.connector)
        else:
            connectedConnector = connectedConnector[0]
        connectingPiece = piecesDict[connecting.piece.id_]
        connectingType = connectingPiece.type
        if connectingType is None:
            raise FeatureNotYetSupported()
        connectingConnector = [p for p in connectingType.connectors if p.id_ == connecting.connector.id_]
        if len(connectingConnector) == 0:
            raise ConnectorNotFound(connectingType, connecting.connector)
        else:
            connectingConnector = connectingConnector[0]
        entity = cls(
            connectedPiece=connectedPiece,
            connectedConnector=connectedConnector,
            connectingPiece=connectingPiece,
            connectingConnector=connectingConnector,
        )
        if connected.designPiece is not None:
            if connectedPiece.refDesign is None and designsById is None:
                raise FeatureNotYetSupported()
            refDesign = connectedPiece.refDesign if connectedPiece.refDesign is not None else None
            if refDesign is None and designsById is not None:
                raise FeatureNotYetSupported()
            if refDesign is not None:
                try:
                    designPiece = next(p for p in refDesign.pieces if p.id_ == connected.designPiece.id_)
                except StopIteration:
                    raise ValueError("Design piece not found in referenced design")
                entity.connectedDesignPiece = designPiece
        if connecting.designPiece is not None:
            if connectingPiece.refDesign is None and designsById is None:
                raise FeatureNotYetSupported()
            refDesign = connectingPiece.refDesign if connectingPiece.refDesign is not None else None
            if refDesign is None and designsById is not None:
                raise FeatureNotYetSupported()
            if refDesign is not None:
                try:
                    designPiece = next(p for p in refDesign.pieces if p.id_ == connecting.designPiece.id_)
                except StopIteration:
                    raise ValueError("Design piece not found in referenced design")
                entity.connectingDesignPiece = designPiece
        try:
            entity.description = obj["description"]
        except KeyError:
            pass
        try:
            entity.gap = obj["gap"]
        except KeyError:
            pass
        try:
            entity.shift = obj["shift"]
        except KeyError:
            pass
        try:
            entity.rise = obj["rise"]
        except KeyError:
            pass
        try:
            entity.rotation = obj["rotation"]
        except KeyError:
            pass
        try:
            entity.turn = obj["turn"]
        except KeyError:
            pass
        try:
            entity.tilt = obj["tilt"]
        except KeyError:
            pass
        try:
            entity.x = obj["x"]
        except KeyError:
            pass
        try:
            entity.y = obj["y"]
        except KeyError:
            pass
        return entity

    def dump(self) -> "ConnectionOutput":
        entity = {**ConnectionProps.model_validate(self).model_dump()}
        entity["connected"] = self.connected.dump()
        entity["connecting"] = self.connecting.dump()
        entity["attributes"] = [q.dump() for q in self.attributes]
        return ConnectionOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Connection":
        for key, value in ConnectionProps.model_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Connection", empty: bool = False) -> "Connection":
        if empty:
            self.empty()
        props = ConnectionProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return [
            self.connected.piece.id_,
            self.connected.connector.id_,
            self.connecting.piece.id_,
            self.connecting.connector.id_,
        ]


class ConnectionInputNode(InputNode):
    class Meta:
        model = ConnectionInput


# endregion Connection

# region Stat


class StatKeyField(RealField, abc.ABC):
    key: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class StatUnitField(RealField, abc.ABC):
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class StatMinField(RealField, abc.ABC):
    min: typing.Optional[float] = sqlmodel.Field(default=None)


class StatMinExcludedField(RealField, abc.ABC):
    min_excluded: bool = sqlmodel.Field(default=False)


class StatMaxField(RealField, abc.ABC):
    max: typing.Optional[float] = sqlmodel.Field(default=None)


class StatMaxExcludedField(RealField, abc.ABC):
    max_excluded: bool = sqlmodel.Field(default=False)


class StatCreatedField(RealField, abc.ABC):
    created_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class StatUpdatedField(RealField, abc.ABC):
    updated_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class StatId(StatKeyField, Id):
    pass


class StatProps(StatUpdatedField, StatCreatedField, StatMaxExcludedField, StatMaxField, StatMinExcludedField, StatMinField, StatUnitField, StatKeyField, Props):
    pass


class StatInput(StatMaxExcludedField, StatMaxField, StatMinExcludedField, StatMinField, StatUnitField, StatKeyField, Input):
    pass


class StatOutput(StatUpdatedField, StatCreatedField, StatMaxExcludedField, StatMaxField, StatMinExcludedField, StatMinField, StatUnitField, StatKeyField, Output):
    pass


class Stat(StatUpdatedField, StatCreatedField, StatMaxExcludedField, StatMaxField, StatMinExcludedField, StatMinField, StatUnitField, StatKeyField, TableEntity, table=True):
    PLURAL = "stats"
    __tablename__ = "stats"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: Design = sqlmodel.Relationship(back_populates="stats")


# endregion Stat

# region Design


class DesignNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class DesignDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class DesignIconField(RealField, abc.ABC):
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class DesignImageField(RealField, abc.ABC):
    image: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class DesignVariantField(RealField, abc.ABC):
    variant: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class DesignViewField(RealField, abc.ABC):
    view: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class DesignParentField(RealField, abc.ABC):
    parent: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)


class DesignIsAbstractField(RealField, abc.ABC):
    is_abstract: bool = sqlmodel.Field(default=False)


class DesignFolderField(RealField, abc.ABC):
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)


class DesignActiveLayerField(RealField, abc.ABC):
    activeLayer: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)


class DesignLocationField(MaskedField, abc.ABC):
    location: typing.Optional[Location] = sqlmodel.Field(default=None)


class DesignUnitField(RealField, abc.ABC):
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class DesignScalableField(RealField, abc.ABC):
    can_scale: bool = sqlmodel.Field(default=True)


class DesignMirrorableField(RealField, abc.ABC):
    can_mirror: bool = sqlmodel.Field(default=True)


class DesignCreatedField(RealField, abc.ABC):
    created_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class DesignUpdatedField(RealField, abc.ABC):
    updated_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class DesignId(DesignNameField, DesignVariantField, Id):
    pass


class DesignProps(
    DesignUnitField, DesignViewField, DesignActiveLayerField, DesignFolderField, DesignIsAbstractField, DesignParentField, DesignLocationField, DesignVariantField, DesignImageField, DesignIconField, DesignDescriptionField, DesignNameField, Props
):
    pass


class DesignInput(DesignUnitField, DesignViewField, DesignVariantField, DesignImageField, DesignIconField, DesignDescriptionField, DesignNameField, Input):
    parent: typing.Optional[str] = sqlmodel.Field(default=None)
    is_abstract: typing.Optional[bool] = sqlmodel.Field(default=None)
    folder: typing.Optional[str] = sqlmodel.Field(default=None)
    activeLayer: typing.Optional[str] = sqlmodel.Field(default=None)
    location: typing.Optional[LocationInput] = sqlmodel.Field(default=None)
    pieces: list[PieceInput] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionInput] = sqlmodel.Field(default_factory=list)
    props: list[PropInput] = sqlmodel.Field(default_factory=list)
    authors: list[str] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class DesignContext(DesignUnitField, DesignViewField, DesignVariantField, DesignDescriptionField, DesignNameField, Context):
    pass

    location: typing.Optional[LocationContext] = sqlmodel.Field(default=None)
    pieces: list[PieceContext] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionContext] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class DesignOutput(DesignUpdatedField, DesignCreatedField, DesignUnitField, DesignViewField, DesignVariantField, DesignImageField, DesignIconField, DesignDescriptionField, DesignNameField, Output):
    pass

    parent: typing.Optional[str] = sqlmodel.Field(default=None)
    is_abstract: typing.Optional[bool] = sqlmodel.Field(default=None)
    folder: typing.Optional[str] = sqlmodel.Field(default=None)
    activeLayer: typing.Optional[str] = sqlmodel.Field(default=None)
    location: typing.Optional[LocationOutput] = sqlmodel.Field(default=None)
    pieces: list[PieceOutput] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionOutput] = sqlmodel.Field(default_factory=list)
    props: list[PropOutput] = sqlmodel.Field(default_factory=list)
    authors: list[str] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class DesignPrediction(DesignDescriptionField, Prediction):
    pass

    pieces: list[PiecePrediction] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionPrediction] = sqlmodel.Field(default_factory=list)


class Design(
    DesignNameField,
    DesignVariantField,
    DesignViewField,
    DesignDescriptionField,
    DesignIconField,
    DesignImageField,
    DesignUnitField,
    DesignScalableField,
    DesignMirrorableField,
    DesignUpdatedField,
    DesignCreatedField,
    DesignActiveLayerField,
    DesignFolderField,
    DesignIsAbstractField,
    DesignParentField,
    TableEntity,
    table=True,
):
    PLURAL = "designs"
    __tablename__ = "designs"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    concepts_: list[Concept] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    artifact_authors: list[ArtifactAuthor] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    locationLongitude: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("location_longitude", sqlalchemy.Float()), exclude=True, default=None)
    locationLatitude: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("location_latitude", sqlalchemy.Float()), exclude=True, default=None)
    layers: list[Layer] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    pieces: list[Piece] = sqlmodel.Relationship(back_populates="design", cascade_delete=True, sa_relationship_kwargs={"foreign_keys": "[Piece.designPk]"})
    groups: list[Group] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    connections: list[Connection] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    stats: list[Stat] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    props: list["Prop"] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(alias="kitId", sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    kit: Kit = sqlmodel.Relationship(back_populates="designs")

    __table_args__ = (sqlalchemy.UniqueConstraint("name", "variant", "view", "kit_id", name="uq_designs_name_variant_view_kit_id"),)

    @property
    def location(self) -> typing.Optional[Location]:
        if self.locationLongitude is None or self.locationLatitude is None:
            return None
        return Location(
            longitude=self.locationLongitude,
            latitude=self.locationLatitude,
        )

    @location.setter
    def location(self, location: typing.Optional[Location]):
        if location is None:
            self.locationLongitude = None
            self.locationLatitude = None
        else:
            self.locationLongitude = location.longitude
            self.locationLatitude = location.latitude

    @property
    def authors(self) -> list[str]:
        return [artifact_author.author_email for artifact_author in self.artifact_authors]

    @authors.setter
    def authors(self, author_emails: list[str]):
        self.artifact_authors = [ArtifactAuthor(author_email=email) for email in author_emails]

    @property
    def concepts(self: "Design") -> list[str]:
        return [concept.name for concept in sorted(self.concepts_, key=lambda x: x.order)]

    @concepts.setter
    def concepts(self: "Design", concepts: list[str]):
        self.concepts_ = [Concept(name=concept, order=i) for i, concept in enumerate(concepts)]

    def parent(self) -> "Kit":
        if self.kit is None:
            raise NoKitAssigned()
        return self.kit

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Design", input: str | dict | DesignInput | typing.Any | None, types: list[Type], designsById: typing.Optional[dict[str, dict[str, dict[str, "Design"]]]] = None) -> "Design":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        props = DesignProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            entity.location = props.location
        except KeyError:
            pass
        typesDict = {}
        for type in types:
            if type.name not in typesDict:
                typesDict[type.name] = {}
            if type.variant not in typesDict[type.name]:
                typesDict[type.name][type.variant] = {}
            typesDict[type.name][type.variant] = type
        try:
            pieces = [Piece.parse(p, typesDict, designsById) for p in obj["pieces"]]
            entity.pieces = pieces
        except KeyError:
            pass
        try:
            connections = [Connection.parse(c, pieces, designsById) for c in obj["connections"]]
            entity.connections = connections
        except KeyError:
            pass
        try:
            props = [Prop.parse(p) for p in obj["props"]]
            entity.props = props
        except KeyError:
            pass
        try:
            attributes = [Attribute.parse(q) for q in obj["attributes"]]
            entity.attributes = attributes
        except KeyError:
            pass
        try:
            author_emails = obj["authors"]
            entity.authors = author_emails
        except KeyError:
            pass
        try:
            concepts = obj["concepts"]
            entity.concepts = concepts
        except KeyError:
            pass
        return entity

    def dump(self) -> "DesignOutput":
        entity = {**DesignProps.model_validate(self).model_dump()}
        entity["pieces"] = [p.dump() for p in self.pieces]
        entity["connections"] = [c.dump() for c in self.connections]
        entity["props"] = [p.dump() for p in self.props]
        entity["attributes"] = [q.dump() for q in self.attributes]
        entity["authors"] = self.authors
        entity["concepts"] = self.concepts
        return DesignOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        props = DesignProps()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.designs = []
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Design", empty: bool = False) -> "Design":
        if empty:
            self.empty()
        props = DesignProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return [self.name, self.variant]


class NoDesignAssigned(NoParentAssigned):
    def __str__(self):
        return "👪 The entity has no parent design assigned."


class DesignInputNode(InputNode):
    class Meta:
        model = DesignInput


class DesignIdInputNode(InputNode):
    class Meta:
        model = DesignId


# endregion Design

# region Kit


class KitUriField(RealField, abc.ABC):
    uri: str = sqlmodel.Field(max_length=URI_LENGTH_LIMIT)


class KitNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class KitDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class KitIconField(RealField, abc.ABC):
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitImageField(RealField, abc.ABC):
    image: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitPreviewField(RealField, abc.ABC):
    preview: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitVersionField(RealField, abc.ABC):
    version: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class KitRemoteField(RealField, abc.ABC):
    remote: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitHomepageField(RealField, abc.ABC):
    homepage: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitLicenseField(RealField, abc.ABC):
    license: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitCreatedField(RealField, abc.ABC):
    created_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class KitUpdatedField(RealField, abc.ABC):
    updated_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class KitId(KitUriField, Id):
    pass


class KitProps(KitLicenseField, KitHomepageField, KitRemoteField, KitVersionField, KitPreviewField, KitImageField, KitIconField, KitDescriptionField, KitNameField, KitUriField, Props):
    pass


class KitInput(KitLicenseField, KitHomepageField, KitRemoteField, KitVersionField, KitPreviewField, KitImageField, KitIconField, KitDescriptionField, KitNameField, Input):
    pass

    types: list[TypeInput] = sqlmodel.Field(default_factory=list)
    designs: list[DesignInput] = sqlmodel.Field(default_factory=list)
    folders: list[FolderInput] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class KitContext(KitDescriptionField, KitNameField, Context):
    pass

    types: list[TypeContext] = sqlmodel.Field(default_factory=list)
    designs: list[DesignContext] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)


class KitOutput(KitUpdatedField, KitCreatedField, KitLicenseField, KitHomepageField, KitRemoteField, KitVersionField, KitPreviewField, KitImageField, KitIconField, KitDescriptionField, KitNameField, KitUriField, Output):
    pass

    types: list[TypeOutput] = sqlmodel.Field(default_factory=list)
    designs: list[DesignOutput] = sqlmodel.Field(default_factory=list)
    folders: list[FolderOutput] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class Kit(KitNameField, KitVersionField, KitDescriptionField, KitIconField, KitImageField, KitRemoteField, KitHomepageField, KitLicenseField, KitPreviewField, KitUriField, KitUpdatedField, KitCreatedField, TableEntity, table=True):
    PLURAL = "kits"
    __tablename__ = "kits"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    concepts_: list[Concept] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    authors_: list[Author] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    files_: list[File] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    folders_: list[Folder] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    interfaces: list[Interface] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    types: list[Type] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    designs: list[Design] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    qualities: list[Quality] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)

    @property
    def concepts(self: "Kit") -> list[str]:
        return [concept.name for concept in sorted(self.concepts_, key=lambda x: x.order)]

    @concepts.setter
    def concepts(self: "Kit", concepts: list[str]):
        self.concepts_ = [Concept(name=concept, order=i) for i, concept in enumerate(concepts)]

    @property
    def folders(self: "Kit") -> list[Folder]:
        return self.folders_

    @folders.setter
    def folders(self: "Kit", folders: list[Folder]):
        self.folders_ = folders

    __table_args__ = (sqlalchemy.UniqueConstraint("uri", name="uq_kits_uri"),)

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Kit", input: str | dict | KitInput | typing.Any | None) -> "Kit":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        uri = obj.get("uri", f"memory://{obj.get('name', 'unnamed')}")
        entity = cls(
            name=obj.get("name", ""),
            version=obj.get("version", ""),
            description=obj.get("description", ""),
            icon=obj.get("icon", ""),
            image=obj.get("image", ""),
            remoteUrl=obj.get("remoteUrl", ""),
            homepageUrl=obj.get("homepageUrl", ""),
            license=obj.get("license", ""),
            preview=obj.get("preview", ""),
            uri=uri,
        )
        try:
            types = [Type.parse(t) for t in obj["types"]]
            entity.types = types
        except KeyError:
            pass
        try:
            designs = [Design.parse(d, types) for d in obj["designs"]]
            entity.designs = designs
        except KeyError:
            pass
        try:
            folders = [Folder.parse(f) for f in obj["folders"]]
            entity.folders = folders
        except KeyError:
            pass
        try:
            concepts = obj["concepts"]
            entity.concepts = concepts
        except KeyError:
            pass
        return entity

    def dump(self) -> "KitOutput":
        entity = {**KitProps.model_validate(self).model_dump()}
        entity["types"] = [t.dump() for t in self.types]
        entity["designs"] = [d.dump() for d in self.designs]
        entity["files"] = [f.dump() for f in self.files_]
        entity["folders"] = [f.dump() for f in self.folders_]
        entity["attributes"] = [q.dump() for q in self.attributes]
        entity["concepts"] = self.concepts
        return KitOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        props = KitProps.model_construct()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.types = []
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Kit", empty: bool = False) -> "Kit":
        if empty:
            self.empty()
        props = KitProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return self.uri

    def guid(self) -> str:
        return self.id()

    # region Design Family Helpers

    def find_design_by_guid(self, design_guid: str) -> "Design":
        """
        Finds a design by its GUID.

        Args:
            design_guid: The GUID of the design to find.

        Returns:
            The design with the specified GUID.

        Raises:
            ValueError: If the design is not found.
        """
        for design in self.designs:
            if design.guid == design_guid:
                return design
        raise ValueError(f"Design {design_guid} not found in kit {self.name}")

    def get_primitive_design(self, design_guid: str) -> "Design":
        """
        Gets the primitive (root) design of a design family.
        A primitive design is a design that has no parent.

        Args:
            design_guid: The GUID of any design in the family.

        Returns:
            The primitive design at the root of the family tree.
        """
        current = self.find_design_by_guid(design_guid)
        while current.parent and current.parent.guid:
            current = self.find_design_by_guid(current.parent.guid)
        return current

    def get_design_family(self, design_guid: str) -> list["Design"]:
        """
        Gets all designs in a design family (the entire tree).

        Args:
            design_guid: The GUID of any design in the family.

        Returns:
            All designs in the family tree.
        """
        primitive = self.get_primitive_design(design_guid)
        family: list[Design] = []
        self._collect_design_descendants(primitive.guid, family)
        return family

    def _collect_design_descendants(self, parent_guid: str, family: list["Design"]) -> None:
        """Helper to collect all descendants of a design."""
        parent = self.find_design_by_guid(parent_guid)
        family.append(parent)
        children = [d for d in self.designs if d.parent and d.parent.guid == parent_guid]
        for child in children:
            self._collect_design_descendants(child.guid, family)

    def are_designs_in_same_family(self, design_guid_a: str, design_guid_b: str) -> bool:
        """
        Checks if two designs belong to the same design family.

        Args:
            design_guid_a: The GUID of the first design.
            design_guid_b: The GUID of the second design.

        Returns:
            True if both designs are in the same family tree.
        """
        primitive_a = self.get_primitive_design(design_guid_a)
        primitive_b = self.get_primitive_design(design_guid_b)
        return primitive_a.guid == primitive_b.guid

    def can_use_design_as_piece(self, container_design_guid: str, piece_design_guid: str) -> bool:
        """
        Checks if a design can be used as a design piece in another design.
        A design cannot contain a design piece from the same family.

        Args:
            container_design_guid: The GUID of the design that would contain the piece.
            piece_design_guid: The GUID of the design to be used as a piece.

        Returns:
            True if the design piece can be added without violating the family constraint.
        """
        return not self.are_designs_in_same_family(container_design_guid, piece_design_guid)

    def find_same_family_design_pieces(self, design_guid: str) -> list["Piece"]:
        """
        Finds design pieces in a design that violate the same-family constraint.

        Args:
            design_guid: The GUID of the design to check.

        Returns:
            List of pieces that reference designs in the same family.
        """
        design = self.find_design_by_guid(design_guid)
        return [p for p in design.pieces if p.design and p.design.guid and self.are_designs_in_same_family(design_guid, p.design.guid)]

    # endregion Design Family Helpers

    # region Type Family Helpers

    def find_type_by_guid(self, type_guid: str) -> "Type":
        """
        Finds a type by its GUID.

        Args:
            type_guid: The GUID of the type to find.

        Returns:
            The type with the specified GUID.

        Raises:
            ValueError: If the type is not found.
        """
        for type_ in self.types:
            if type_.guid == type_guid:
                return type_
        raise ValueError(f"Type {type_guid} not found in kit {self.name}")

    def get_primitive_type(self, type_guid: str) -> "Type":
        """
        Gets the primitive (root) type of a type family.
        A primitive type is a type that has no parent.

        Args:
            type_guid: The GUID of any type in the family.

        Returns:
            The primitive type at the root of the family tree.
        """
        current = self.find_type_by_guid(type_guid)
        while current.parent and current.parent.guid:
            current = self.find_type_by_guid(current.parent.guid)
        return current

    def get_type_family(self, type_guid: str) -> list["Type"]:
        """
        Gets all types in a type family (the entire tree).

        Args:
            type_guid: The GUID of any type in the family.

        Returns:
            All types in the family tree.
        """
        primitive = self.get_primitive_type(type_guid)
        family: list[Type] = []
        self._collect_type_descendants(primitive.guid, family)
        return family

    def _collect_type_descendants(self, parent_guid: str, family: list["Type"]) -> None:
        """Helper to collect all descendants of a type."""
        parent = self.find_type_by_guid(parent_guid)
        family.append(parent)
        children = [t for t in self.types if t.parent and t.parent.guid == parent_guid]
        for child in children:
            self._collect_type_descendants(child.guid, family)

    def are_types_in_same_family(self, type_guid_a: str, type_guid_b: str) -> bool:
        """
        Checks if two types belong to the same type family.

        Args:
            type_guid_a: The GUID of the first type.
            type_guid_b: The GUID of the second type.

        Returns:
            True if both types are in the same family tree.
        """
        primitive_a = self.get_primitive_type(type_guid_a)
        primitive_b = self.get_primitive_type(type_guid_b)
        return primitive_a.guid == primitive_b.guid

    # endregion Type Family Helpers

# endregion Kit

# region Moved Graphene Nodes


class AttributeNode(TableEntityNode):
    class Meta:
        model = Attribute


class PlaneNode(TableNode):
    class Meta:
        model = Plane


class AuthorNode(TableEntityNode):
    class Meta:
        model = Author


class ModelNode(TableEntityNode):
    class Meta:
        model = Model
        excludedFields = ("tags_",)


class ConnectorNode(TableEntityNode):
    class Meta:
        model = Connector
        exclude_fields = ("connecteds", "connectings")

    localId = graphene.String()

    def resolve_localId(self, info):
        return getattr(self, "id_", "")


class TypeNode(TableEntityNode):
    class Meta:
        model = Type


class PieceNode(TableEntityNode):
    class Meta:
        model = Piece
        exclude_fields = ("connecteds", "connectings")

    localId = graphene.String()

    def resolve_localId(self, info):
        return getattr(self, "id_", "")


class ConnectionNode(TableEntityNode):
    class Meta:
        model = Connection
        exclude_fields = (
            "connectedPiece",
            "connectedConnector",
            "connectingPiece",
            "connectingConnector",
        )

    connected = graphene.NonNull(lambda: SideNode)
    connecting = graphene.NonNull(lambda: SideNode)

    def resolve_connected(self, info):
        return self.connected

    def resolve_connecting(self, info):
        return self.connecting


class DesignNode(TableEntityNode):
    class Meta:
        model = Design


# endregion Moved Graphene Nodes
class KitNotFound(NotFound):
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't find an local or remote kit under uri:\n {self.uri}."


class NoKitToDelete(KitNotFound):
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't delete the kit because no local or remote kit was found under uri:\n {self.uri}."


class KitZipDoesNotContainSemioFolder(KitNotFound):
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 The remote zip kit ({self.uri}) is not a valid kit."


class OnlyRemoteKitsCanBeCached(ClientError):
    def __init__(self, nonRemoteUri: str) -> None:
        self.nonRemoteUri = nonRemoteUri

    def __str__(self):
        return f"🔍 Only remote kits can be cached. The uri ({self.nonRemoteUri}) doesn't start with http and ends with .zip"


class KitUriNotValid(ClientError, abc.ABC):
    """🆔 The base for all kit uri not valid errors."""


class LocalKitUriNotValid(KitUriNotValid, abc.ABC):
    """📂 The base for all local kit uri not valid errors."""


class LocalKitUriIsNotAbsolute(LocalKitUriNotValid):
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"📂 The local kit uri ({self.uri}) is relative. It needs to be absolute (include the parent folders, drives, ...)."


class LocalKitUriIsNotDirectory(LocalKitUriNotValid):
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"📂 The local kit uri ({self.uri}) is not a directory."


class NoKitAssigned(NoParentAssigned):
    def __str__(self):
        return "👪 The entity has no parent kit assigned."


class KitAlreadyExists(AlreadyExists, abc.ABC):
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self) -> str:
        return f"♊ A kit under uri ({self.uri}) already exists."


class KitInputNode(InputNode):
    class Meta:
        model = KitInput


class KitNode(TableEntityNode):
    class Meta:
        model = Kit


# endregion Domain

# region Validation


class ValidationSeverity(enum.Enum):
    ERROR = "error"
    WARNING = "warning"


@dataclasses.dataclass
class ValidationFix:
    title: str
    diff: dict

    def toDict(self) -> dict:
        return {"title": self.title, "diff": self.diff}


@dataclasses.dataclass
class ValidationIssue:
    constraintId: str
    severity: ValidationSeverity
    message: str
    entityKind: str
    entityGuid: str
    fixes: list[ValidationFix] = dataclasses.field(default_factory=list)

    def toDict(self) -> dict:
        return {
            "constraintId": self.constraintId,
            "severity": self.severity.value,
            "message": self.message,
            "entityKind": self.entityKind,
            "entityGuid": self.entityGuid,
            "fixes": [f.toDict() for f in self.fixes],
        }


@dataclasses.dataclass
class ValidationResult:
    issues: list[ValidationIssue]

    def hasErrors(self) -> bool:
        return any(i.severity == ValidationSeverity.ERROR for i in self.issues)

    def toDict(self) -> dict:
        sortedIssues = sorted(self.issues, key=lambda i: (i.constraintId, i.entityGuid))
        return {"issues": [i.toDict() for i in sortedIssues]}

    def serialize(self) -> str:
        return json.dumps(self.toDict(), indent=2)


def _isGuid(s: str) -> bool:
    import re

    return bool(re.match(r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$", s, re.IGNORECASE))


def _normalizeGuids(obj: typing.Any) -> typing.Any:
    if obj is None:
        return obj
    if isinstance(obj, str) and _isGuid(obj):
        return "<GUID>"
    if isinstance(obj, list):
        return [_normalizeGuids(x) for x in obj]
    if isinstance(obj, dict):
        return {k: _normalizeGuids(v) for k, v in obj.items()}
    return obj


def areValidationResultsEqual(a: ValidationResult, b: ValidationResult) -> bool:
    if len(a.issues) != len(b.issues):
        return False
    sortedA = sorted(a.issues, key=lambda i: (i.constraintId, i.entityGuid))
    sortedB = sorted(b.issues, key=lambda i: (i.constraintId, i.entityGuid))
    for ia, ib in zip(sortedA, sortedB):
        if ia.constraintId != ib.constraintId or ia.severity != ib.severity or ia.message != ib.message or ia.entityKind != ib.entityKind or ia.entityGuid != ib.entityGuid:
            return False
        if len(ia.fixes) != len(ib.fixes):
            return False
        for fa, fb in zip(ia.fixes, ib.fixes):
            if fa.title != fb.title:
                return False

            if ia.constraintId == "guid-unique":
                continue
            if json.dumps(_normalizeGuids(fa.diff), sort_keys=True) != json.dumps(_normalizeGuids(fb.diff), sort_keys=True):
                return False
    return True


def parseValidationResult(jsonStr: str) -> ValidationResult:
    data = json.loads(jsonStr)
    issues = []
    for i in data["issues"]:
        fixes = [ValidationFix(title=f["title"], diff=f["diff"]) for f in i.get("fixes", [])]
        issues.append(ValidationIssue(constraintId=i["constraintId"], severity=ValidationSeverity(i["severity"]), message=i["message"], entityKind=i["entityKind"], entityGuid=i["entityGuid"], fixes=fixes))
    return ValidationResult(issues=issues)


def validateGuidUniqueness(kit: Kit) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    seen: dict[str, str] = {}

    def check(entityKind: str, entityGuid: str) -> None:
        if entityGuid in seen:
            issues.append(ValidationIssue(constraintId="guid-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate GUID "{entityGuid}". First occurrence kept.', entityKind=entityKind, entityGuid=entityGuid))
        else:
            seen[entityGuid] = entityKind

    check("Kit", kit.guid)
    for t in kit.types:
        check("Type", t.guid)
    for d in kit.designs:
        check("Design", d.guid)
        for p in d.pieces:
            check("Piece", p.guid)
        for c in d.connections:
            check("Connection", c.guid)
        for s in d.stats:
            check("Stat", s.guid)
    for q in kit.qualities:
        check("Quality", q.guid)
    for f in kit.files_:
        check("File", f.guid)
    for fo in kit.folders_:
        check("Folder", fo.guid)
    return issues


def validateTypeNameUniqueness(kit: Kit) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    byParent: dict[str | None, list[Type]] = {}
    for t in kit.types:
        parentGuid = t.parent.guid if t.parent else None
        if parentGuid not in byParent:
            byParent[parentGuid] = []
        byParent[parentGuid].append(t)
    for parentGuid, siblings in byParent.items():
        names: dict[str, list[Type]] = {}
        for t in siblings:
            if t.name not in names:
                names[t.name] = []
            names[t.name].append(t)
        for name, group in names.items():
            if len(group) > 1:
                for t in group[1:]:
                    issues.append(ValidationIssue(constraintId="type-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate type name "{name}" among siblings.', entityKind="Type", entityGuid=t.guid))
    return issues


def validateDesignNameUniqueness(kit: Kit) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    byParent: dict[str | None, list[Design]] = {}
    for d in kit.designs:
        parentGuid = d.parent.guid if d.parent else None
        if parentGuid not in byParent:
            byParent[parentGuid] = []
        byParent[parentGuid].append(d)
    for parentGuid, siblings in byParent.items():
        names: dict[str, list[Design]] = {}
        for d in siblings:
            if d.name not in names:
                names[d.name] = []
            names[d.name].append(d)
        for name, group in names.items():
            if len(group) > 1:
                for d in group[1:]:
                    issues.append(ValidationIssue(constraintId="design-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate design name "{name}" among siblings.', entityKind="Design", entityGuid=d.guid))
    return issues


def validatePieceNameUniqueness(kit: Kit) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    for design in kit.designs:
        names: dict[str, list[Piece]] = {}
        for p in design.pieces:
            if p.name_ and p.name_ not in names:
                names[p.name_] = []
            if p.name_:
                names[p.name_].append(p)
        for name, group in names.items():
            if len(group) > 1:
                for p in group[1:]:
                    issues.append(ValidationIssue(constraintId="piece-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate piece name "{name}" in design.', entityKind="Piece", entityGuid=p.guid))
    return issues


def validatePortNameUniqueness(kit: Kit) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    for t in kit.types:
        names: dict[str, list[Connector]] = {}
        for connector in t.connectors:
            if connector.name_ and connector.name_ not in names:
                names[connector.name_] = []
            if connector.name_:
                names[connector.name_].append(connector)
        for name, group in names.items():
            if len(group) > 1:
                for connector in group[1:]:
                    issues.append(ValidationIssue(constraintId="connector-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate connector name "{name}" in type.', entityKind="Connector", entityGuid=connector.guid))
    return issues


def validateModelNameUniqueness(kit: Kit) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    for t in kit.types:
        names: dict[str, list[Model]] = {}
        for model in t.models:
            if model.name and model.name not in names:
                names[model.name] = []
            if model.name:
                names[model.name].append(model)
        for name, group in names.items():
            if len(group) > 1:
                for model in group[1:]:
                    issues.append(ValidationIssue(constraintId="model-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate model name "{name}" in type.', entityKind="Model", entityGuid=model.guid))
    return issues


def validateQualityNameUniqueness(kit: Kit) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    names: dict[str, list[Quality]] = {}
    for q in kit.qualities:
        if q.name not in names:
            names[q.name] = []
        names[q.name].append(q)
    for name, group in names.items():
        if len(group) > 1:
            for q in group[1:]:
                issues.append(ValidationIssue(constraintId="quality-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate quality name "{name}".', entityKind="Quality", entityGuid=q.guid))
    return issues


def validateFileNameUniqueness(kit: Kit) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    names: dict[str, list[File]] = {}
    for f in kit.files_:
        if f.name not in names:
            names[f.name] = []
        names[f.name].append(f)
    for name, group in names.items():
        if len(group) > 1:
            for f in group[1:]:
                issues.append(ValidationIssue(constraintId="file-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate file name "{name}".', entityKind="File", entityGuid=f.guid))
    return issues


def validateFolderNameUniqueness(kit: Kit) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    byParent: dict[str | None, list[Folder]] = {}
    for fo in kit.folders_:
        parentGuid = fo.parent if fo.parent else None
        if parentGuid not in byParent:
            byParent[parentGuid] = []
        byParent[parentGuid].append(fo)
    for parentGuid, siblings in byParent.items():
        names: dict[str, list[Folder]] = {}
        for fo in siblings:
            if fo.name not in names:
                names[fo.name] = []
            names[fo.name].append(fo)
        for name, group in names.items():
            if len(group) > 1:
                for fo in group[1:]:
                    issues.append(ValidationIssue(constraintId="folder-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate folder name "{name}" among siblings.', entityKind="Folder", entityGuid=fo.guid))
    return issues


def validateLayerPathUniqueness(kit: Kit) -> list[ValidationIssue]:
    issues: list[ValidationIssue] = []
    for design in kit.designs:
        paths: dict[str, list[Layer]] = {}
        for layer in design.layers:
            if layer.path not in paths:
                paths[layer.path] = []
            paths[layer.path].append(layer)
        for path, group in paths.items():
            if len(group) > 1:
                for layer in group[1:]:
                    issues.append(ValidationIssue(constraintId="layer-path-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate layer path "{path}" in design.', entityKind="Layer", entityGuid=layer.guid))
    return issues


def validateKit(kit: Kit) -> ValidationResult:
    issues: list[ValidationIssue] = []
    issues.extend(validateGuidUniqueness(kit))
    issues.extend(validateTypeNameUniqueness(kit))
    issues.extend(validateDesignNameUniqueness(kit))
    issues.extend(validatePieceNameUniqueness(kit))
    issues.extend(validatePortNameUniqueness(kit))
    issues.extend(validateModelNameUniqueness(kit))
    issues.extend(validateQualityNameUniqueness(kit))
    issues.extend(validateFileNameUniqueness(kit))
    issues.extend(validateFolderNameUniqueness(kit))
    issues.extend(validateLayerPathUniqueness(kit))
    return ValidationResult(issues=issues)


# region Dict-based Validation


def _makeFix(title: str, diff: dict) -> ValidationFix:
    return ValidationFix(title=title, diff=diff)


def _deepCopy(obj: typing.Any) -> typing.Any:
    return json.loads(json.dumps(obj))


def _newGuid() -> str:
    import uuid

    return str(uuid.uuid4())


def validateKitDict(kit: dict) -> ValidationResult:
    issues: list[ValidationIssue] = []
    seen: dict[str, str] = {}
    seenEntities: dict[str, dict] = {}

    def checkGuid(entityKind: str, entityGuid: str, entity: dict) -> None:
        if entityGuid in seen:
            newGuid = _newGuid()
            entityCopy = _deepCopy(entity)
            entityCopy["guid"] = newGuid
            collectionKey = {
                "Type": "types",
                "Design": "designs",
                "Piece": "pieces",
                "Connection": "connections",
                "Connector": "connectors",
                "Model": "models",
                "Quality": "qualities",
                "Interface": "interfaces",
                "File": "files",
                "Folder": "folders",
                "Stat": "stats",
                "Layer": "layers",
            }.get(entityKind, "")
            if collectionKey:
                diff = {collectionKey: {"removed": [{"guid": entityGuid}], "added": [entityCopy]}}
                fix = _makeFix("Regenerate GUID", diff)
                issues.append(ValidationIssue(constraintId="guid-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate GUID "{entityGuid}". First occurrence kept.', entityKind=entityKind, entityGuid=entityGuid, fixes=[fix]))
            else:
                issues.append(ValidationIssue(constraintId="guid-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate GUID "{entityGuid}". First occurrence kept.', entityKind=entityKind, entityGuid=entityGuid, fixes=[]))
        else:
            seen[entityGuid] = entityKind
            seenEntities[entityGuid] = entity

    checkGuid("Kit", kit.get("guid", ""), kit)
    for t in kit.get("types", []):
        checkGuid("Type", t.get("guid", ""), t)
        for connector in t.get("connectors", []):
            checkGuid("Connector", connector.get("guid", ""), connector)
        for model in t.get("models", []):
            checkGuid("Model", model.get("guid", ""), model)
    for d in kit.get("designs", []):
        checkGuid("Design", d.get("guid", ""), d)
        for p in d.get("pieces", []):
            checkGuid("Piece", p.get("guid", ""), p)
        for c in d.get("connections", []):
            checkGuid("Connection", c.get("guid", ""), c)
        for s in d.get("stats", []):
            checkGuid("Stat", s.get("guid", ""), s)
    for q in kit.get("qualities", []):
        checkGuid("Quality", q.get("guid", ""), q)
    for i in kit.get("interfaces", []):
        checkGuid("Interface", i.get("guid", ""), i)
    for f in kit.get("files", []):
        checkGuid("File", f.get("guid", ""), f)
    for fo in kit.get("folders", []):
        checkGuid("Folder", fo.get("guid", ""), fo)
    byParent: dict[str | None, list[dict]] = {}
    for t in kit.get("types", []):
        parentGuid = t.get("parent", {}).get("guid") if t.get("parent") else None
        if parentGuid not in byParent:
            byParent[parentGuid] = []
        byParent[parentGuid].append(t)
    for parentGuid, siblings in byParent.items():
        names: dict[str, list[dict]] = {}
        for t in siblings:
            name = t.get("name", "")
            if name not in names:
                names[name] = []
            names[name].append(t)
        for name, group in names.items():
            if len(group) > 1:
                for t in group[1:]:
                    fix = _makeFix(f'Rename "{name}"', {"types": {"updated": [{"type": {"guid": t.get("guid", "")}, "diff": {"name": f"{name} 2"}}]}})
                    issues.append(ValidationIssue(constraintId="type-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate type name "{name}" among siblings.', entityKind="Type", entityGuid=t.get("guid", ""), fixes=[fix]))
    byParent = {}
    for d in kit.get("designs", []):
        parentGuid = d.get("parent", {}).get("guid") if d.get("parent") else None
        if parentGuid not in byParent:
            byParent[parentGuid] = []
        byParent[parentGuid].append(d)
    for parentGuid, siblings in byParent.items():
        names: dict[str, list[dict]] = {}
        for d in siblings:
            name = d.get("name", "")
            if name not in names:
                names[name] = []
            names[name].append(d)
        for name, group in names.items():
            if len(group) > 1:
                for d in group[1:]:
                    fix = _makeFix(f'Rename "{name}"', {"designs": {"updated": [{"design": {"guid": d.get("guid", "")}, "diff": {"name": f"{name} 2"}}]}})
                    issues.append(ValidationIssue(constraintId="design-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate design name "{name}" among siblings.', entityKind="Design", entityGuid=d.get("guid", ""), fixes=[fix]))
    for design in kit.get("designs", []):
        designName = design.get("name", "")
        designGuid = design.get("guid", "")
        names = {}
        for p in design.get("pieces", []):
            name = p.get("name", "")
            if name and name not in names:
                names[name] = []
            if name:
                names[name].append(p)
        for name, group in names.items():
            if len(group) > 1:
                for p in group[1:]:
                    fix = _makeFix(f'Rename piece "{name}"', {"designs": {"updated": [{"design": {"guid": designGuid}, "diff": {"pieces": {"updated": [{"piece": {"guid": p.get("guid", "")}, "diff": {"name": f"{name} 2"}}]}}}]}})
                    issues.append(
                        ValidationIssue(constraintId="piece-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate piece name "{name}" inside design "{designName}".', entityKind="Piece", entityGuid=p.get("guid", ""), fixes=[fix])
                    )
    for t in kit.get("types", []):
        typeName = t.get("name", "")
        typeGuid = t.get("guid", "")
        names = {}
        for connector in t.get("connectors", []):
            name = connector.get("name", "")
            if name and name not in names:
                names[name] = []
            if name:
                names[name].append(connector)
        for name, group in names.items():
            if len(group) > 1:
                for connector in group[1:]:
                    fix = _makeFix(f'Rename connector "{name}"', {"types": {"updated": [{"type": {"guid": typeGuid}, "diff": {"connectors": {"updated": [{"connector": {"guid": connector.get("guid", "")}, "diff": {"name": f"{name} 2"}}]}}}]}})
                    issues.append(
                        ValidationIssue(
                            constraintId="connector-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate connector name "{name}" inside type "{typeName}".', entityKind="Connector", entityGuid=connector.get("guid", ""), fixes=[fix]
                        )
                    )
    for t in kit.get("types", []):
        typeName = t.get("name", "")
        typeGuid = t.get("guid", "")
        names = {}
        for model in t.get("models", []):
            name = model.get("name", "")
            if name and name not in names:
                names[name] = []
            if name:
                names[name].append(model)
        for name, group in names.items():
            if len(group) > 1:
                for model in group[1:]:
                    fix = _makeFix(f'Rename model "{name}"', {"types": {"updated": [{"type": {"guid": typeGuid}, "diff": {"models": {"updated": [{"model": {"guid": model.get("guid", "")}, "diff": {"name": f"{name} 2"}}]}}}]}})
                    issues.append(
                        ValidationIssue(constraintId="model-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate model name "{name}" inside type "{typeName}".', entityKind="Model", entityGuid=model.get("guid", ""), fixes=[fix])
                    )
    names = {}
    for q in kit.get("qualities", []):
        name = q.get("name", "")
        if name not in names:
            names[name] = []
        names[name].append(q)
    for name, group in names.items():
        if len(group) > 1:
            for q in group[1:]:
                fix = _makeFix(f'Rename quality "{name}"', {"qualities": {"updated": [{"quality": {"guid": q.get("guid", "")}, "diff": {"name": f"{name} 2"}}]}})
                issues.append(ValidationIssue(constraintId="quality-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate quality name "{name}".', entityKind="Quality", entityGuid=q.get("guid", ""), fixes=[fix]))
    names = {}
    for i in kit.get("interfaces", []):
        name = i.get("name", "")
        if name not in names:
            names[name] = []
        names[name].append(i)
    for name, group in names.items():
        if len(group) > 1:
            for iface in group[1:]:
                fix = _makeFix(f'Rename interface "{name}"', {"interfaces": {"updated": [{"interface": {"guid": iface.get("guid", "")}, "diff": {"name": f"{name} 2"}}]}})
                issues.append(ValidationIssue(constraintId="interface-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate interface name "{name}".', entityKind="Interface", entityGuid=iface.get("guid", ""), fixes=[fix]))
    names = {}
    for f in kit.get("files", []):
        name = f.get("name", "")
        if name not in names:
            names[name] = []
        names[name].append(f)
    for name, group in names.items():
        if len(group) > 1:
            for f in group[1:]:
                fix = _makeFix(f'Rename file "{name}"', {"files": {"updated": [{"file": {"guid": f.get("guid", "")}, "diff": {"name": f"{name} 2"}}]}})
                issues.append(ValidationIssue(constraintId="file-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate file name "{name}".', entityKind="File", entityGuid=f.get("guid", ""), fixes=[fix]))
    byParent = {}
    for fo in kit.get("folders", []):
        parentGuid = fo.get("parent", {}).get("guid") if fo.get("parent") else None
        if parentGuid not in byParent:
            byParent[parentGuid] = []
        byParent[parentGuid].append(fo)
    for parentGuid, siblings in byParent.items():
        names = {}
        for fo in siblings:
            name = fo.get("name", "")
            if name not in names:
                names[name] = []
            names[name].append(fo)
        for name, group in names.items():
            if len(group) > 1:
                for fo in group[1:]:
                    fix = _makeFix(f'Rename folder "{name}"', {"folders": {"updated": [{"folder": {"guid": fo.get("guid", "")}, "diff": {"name": f"{name} 2"}}]}})
                    issues.append(ValidationIssue(constraintId="folder-name-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate folder name "{name}" among siblings.', entityKind="Folder", entityGuid=fo.get("guid", ""), fixes=[fix]))
    for design in kit.get("designs", []):
        designName = design.get("name", "")
        designGuid = design.get("guid", "")
        paths: dict[str, list[dict]] = {}
        for layer in design.get("layers", []):
            path = layer.get("path", "")
            if path not in paths:
                paths[path] = []
            paths[path].append(layer)
        for path, group in paths.items():
            if len(group) > 1:
                for layer in group[1:]:
                    fix = _makeFix(f'Rename layer "{path}"', {"designs": {"updated": [{"design": {"guid": designGuid}, "diff": {"layers": {"updated": [{"layer": {"guid": layer.get("guid", "")}, "diff": {"path": f"{path} 2"}}]}}}]}})
                    issues.append(
                        ValidationIssue(constraintId="layer-path-unique", severity=ValidationSeverity.ERROR, message=f'Duplicate layer path "{path}" inside design "{designName}".', entityKind="Layer", entityGuid=layer.get("guid", ""), fixes=[fix])
                    )
    return ValidationResult(issues=issues)


# endregion Dict-based Validation

# endregion Validation

# region Graph Operations


def buildPieceGraph(design: Design | dict) -> networkx.Graph:
    G = networkx.Graph()
    pieces = design.get("pieces", []) if isinstance(design, dict) else design.pieces
    connections = design.get("connections", []) if isinstance(design, dict) else design.connections
    for piece in pieces:
        pieceGuid = piece["guid"] if isinstance(piece, dict) else piece.guid
        G.add_node(pieceGuid, piece=piece)
    for connection in connections:
        if isinstance(connection, dict):
            sourceId = connection["connected"]["piece"]["guid"]
            targetId = connection["connecting"]["piece"]["guid"]
        else:
            sourceId = connection.connectedPiece.guid
            targetId = connection.connectingPiece.guid
        if G.has_node(sourceId) and G.has_node(targetId):
            G.add_edge(sourceId, targetId, connection=connection)
    return G


def findFixedPieces(design: Design | dict) -> list[str]:
    pieces = design.get("pieces", []) if isinstance(design, dict) else design.pieces
    result = []
    for p in pieces:
        if isinstance(p, dict):
            if p.get("plane") is not None:
                result.append(p["guid"])
        else:
            if p.plane is not None:
                result.append(p.guid)
    return result


def getConnectedComponents(design: Design | dict) -> list[set[str]]:
    G = buildPieceGraph(design)
    return [set(c) for c in networkx.connected_components(G)]


def getPieceHierarchy(design: Design | dict, rootGuid: str) -> dict[str, int]:
    G = buildPieceGraph(design)
    if rootGuid not in G:
        return {}
    return networkx.single_source_shortest_path_length(G, rootGuid)


# endregion Graph Operations

# region FlattenDesign


def getTypeByGuid(kit: dict, guid: str) -> dict | None:
    for t in kit.get("types", []):
        if t.get("guid") == guid:
            return t
    return None


def getConnectorFromType(kit: dict, typeData: dict | None, connectorGuid: str | None) -> dict | None:
    if typeData is None:
        return None
    if connectorGuid is None:
        connectors = typeData.get("connectors", [])
        if connectors:
            return connectors[0]
        parent = typeData.get("parent")
        if parent:
            parentType = getTypeByGuid(kit, parent.get("guid", ""))
            return getConnectorFromType(kit, parentType, connectorGuid)
        return None
    for connector in typeData.get("connectors", []):
        if connector.get("guid") == connectorGuid:
            return connector
    parent = typeData.get("parent")
    if parent:
        parentType = getTypeByGuid(kit, parent.get("guid", ""))
        return getConnectorFromType(kit, parentType, connectorGuid)
    connectors = typeData.get("connectors", [])
    if connectors:
        return connectors[0]
    return None


def computeChildPlaneDict(parentPlane: dict, parentConnector: dict, childConnector: dict, connection: dict) -> dict:
    gap = connection.get("gap", 0)
    shift = connection.get("shift", 0)
    rise = connection.get("rise", 0)
    rotation = connection.get("rotation", 0)
    turn = connection.get("turn", 0)
    tilt = connection.get("tilt", 0)
    pOrigin = numpy.array([parentPlane["origin"]["x"], parentPlane["origin"]["y"], parentPlane["origin"]["z"]])
    pX = numpy.array([parentPlane["xAxis"]["x"], parentPlane["xAxis"]["y"], parentPlane["xAxis"]["z"]])
    pY = numpy.array([parentPlane["yAxis"]["x"], parentPlane["yAxis"]["y"], parentPlane["yAxis"]["z"]])
    pZ = numpy.cross(pX, pY)
    parentMatrix = numpy.eye(4)
    parentMatrix[:3, 0] = pX
    parentMatrix[:3, 1] = pY
    parentMatrix[:3, 2] = pZ
    parentMatrix[:3, 3] = pOrigin
    ppPoint = numpy.array([parentConnector["point"]["x"], parentConnector["point"]["y"], parentConnector["point"]["z"]])
    ppDir = numpy.array([parentConnector["direction"]["x"], parentConnector["direction"]["y"], parentConnector["direction"]["z"]])
    cpPoint = numpy.array([childConnector["point"]["x"], childConnector["point"]["y"], childConnector["point"]["z"]])
    cpDir = numpy.array([childConnector["direction"]["x"], childConnector["direction"]["y"], childConnector["direction"]["z"]])
    ppWorld = parentMatrix[:3, :3] @ ppPoint + parentMatrix[:3, 3]
    ppDirWorld = parentMatrix[:3, :3] @ ppDir
    ppDirWorld = normalizeVector(ppDirWorld)
    translation = ppWorld + gap * ppDirWorld + shift * numpy.cross(ppDirWorld, pZ) + rise * pZ
    targetDir = -ppDirWorld
    cpDirNormalized = normalizeVector(cpDir)
    if numpy.allclose(cpDirNormalized, targetDir, atol=1e-6):
        baseRotation = numpy.eye(3)
    elif numpy.allclose(cpDirNormalized, -targetDir, atol=1e-6):
        axis = numpy.array([1.0, 0.0, 0.0])
        if numpy.allclose(numpy.abs(cpDirNormalized), axis, atol=1e-6):
            axis = numpy.array([0.0, 1.0, 0.0])
        baseRotation = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([axis, [numpy.pi]]))
    else:
        axis = numpy.cross(cpDirNormalized, targetDir)
        axis = normalizeVector(axis)
        angle = numpy.arccos(numpy.clip(numpy.dot(cpDirNormalized, targetDir), -1.0, 1.0))
        baseRotation = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([axis, [angle]]))
    rotRad = numpy.deg2rad(rotation)
    rotationMatrix = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([targetDir, [rotRad]]))
    turnRad = numpy.deg2rad(turn)
    pZWorld = normalizeVector(pZ)
    turnMatrix = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([pZWorld, [turnRad]]))
    tiltRad = numpy.deg2rad(tilt)
    pXWorld = normalizeVector(parentMatrix[:3, :3] @ numpy.array([1, 0, 0]))
    tiltMatrix = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([pXWorld, [tiltRad]]))
    combinedRotation = tiltMatrix @ turnMatrix @ rotationMatrix @ baseRotation
    childOrigin = translation - combinedRotation @ cpPoint
    childX = combinedRotation @ numpy.array([1, 0, 0])
    childY = combinedRotation @ numpy.array([0, 1, 0])
    return {
        "origin": {"x": float(childOrigin[0]), "y": float(childOrigin[1]), "z": float(childOrigin[2])},
        "xAxis": {"x": float(childX[0]), "y": float(childX[1]), "z": float(childX[2])},
        "yAxis": {"x": float(childY[0]), "y": float(childY[1]), "z": float(childY[2])},
    }


def flattenDesignDict(kit: dict, designGuid: str) -> dict:
    design = next((d for d in kit.get("designs", []) if d.get("guid") == designGuid), None)
    if design is None:
        raise ValueError(f"Design {designGuid} not found")
    pieces = design.get("pieces", [])
    if not pieces:
        return {}
    pieceMap = {p["guid"]: dict(p) for p in pieces}
    piecePlanes: dict[str, dict] = {}
    G = buildPieceGraph(design)
    components = list(networkx.connected_components(G))
    for component in components:
        rootNode = None
        for nodeId in component:
            piece = pieceMap.get(nodeId)
            if piece and piece.get("plane") is not None:
                rootNode = nodeId
                break
        if rootNode is None and component:
            rootNode = next(iter(component))
        if rootNode is None:
            continue
        rootPiece = pieceMap[rootNode]
        if rootPiece.get("plane"):
            piecePlanes[rootNode] = rootPiece["plane"]
        else:
            piecePlanes[rootNode] = {"origin": {"x": 0, "y": 0, "z": 0}, "xAxis": {"x": 1, "y": 0, "z": 0}, "yAxis": {"x": 0, "y": 1, "z": 0}}
        for source, target in networkx.bfs_edges(G, rootNode):
            if target in piecePlanes:
                continue
            parentId = source
            childId = target
            parentPlane = piecePlanes.get(parentId)
            if parentPlane is None:
                continue
            edgeData = G.get_edge_data(parentId, childId)
            connection = edgeData.get("connection") if edgeData else None
            if connection is None:
                continue
            parentPiece = pieceMap[parentId]
            childPiece = pieceMap[childId]
            parentType = getTypeByGuid(kit, parentPiece.get("type", {}).get("guid", ""))
            childType = getTypeByGuid(kit, childPiece.get("type", {}).get("guid", ""))
            parentSide = connection["connected"] if connection["connected"]["piece"]["guid"] == parentId else connection["connecting"]
            childSide = connection["connecting"] if connection["connecting"]["piece"]["guid"] == childId else connection["connected"]
            parentConnectorGuid = parentSide.get("connector", {}).get("guid") if parentSide.get("connector") else None
            childConnectorGuid = childSide.get("connector", {}).get("guid") if childSide.get("connector") else None
            parentConnector = getConnectorFromType(kit, parentType, parentConnectorGuid)
            childConnector = getConnectorFromType(kit, childType, childConnectorGuid)
            if parentConnector is None or childConnector is None:
                continue
            childPlane = computeChildPlaneDict(parentPlane, parentConnector, childConnector, connection)
            piecePlanes[childId] = childPlane
    updatedPieces = []
    for piece in pieces:
        newPiece = dict(piece)
        if piece["guid"] in piecePlanes:
            newPiece["plane"] = piecePlanes[piece["guid"]]
        if newPiece.get("center") is None:
            newPiece["center"] = {"u": 0, "v": 0}
        updatedPieces.append(newPiece)
    return {"pieces": {"updated": [{"id": p["guid"], "diff": {"plane": p.get("plane"), "center": p.get("center")}} for p in updatedPieces if p["guid"] in piecePlanes]}}


# endregion FlattenDesign

# region Kit Diff Operations


def _normalizeValue(value: typing.Any) -> typing.Any:
    """Normalize empty values to None for comparison."""
    if value is None or value == "" or value == []:
        return None
    return value


def _normalizeBoolean(value: bool | None) -> bool | None:
    """Normalize boolean: True stays True, False/None become None."""
    return True if value else None


def _normalizeArray(arr: list | None) -> list:
    """Normalize None or single item to list."""
    if arr is None:
        return []
    if not isinstance(arr, list):
        return [arr]
    return arr


def areAttributesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for attrA in arrA:
        attrB = next((x for x in arrB if x.get("guid") == attrA.get("guid")), None)
        if attrB is None:
            return False
        if attrA.get("key") != attrB.get("key"):
            return False
        if _normalizeValue(attrA.get("value")) != _normalizeValue(attrB.get("value")):
            return False
        if _normalizeValue(attrA.get("definition")) != _normalizeValue(attrB.get("definition")):
            return False
        if strict:
            if attrA.get("createdAt") != attrB.get("createdAt"):
                return False
            if attrA.get("updatedAt") != attrB.get("updatedAt"):
                return False
    return True


def arePropsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for propA in arrA:
        propB = next((x for x in arrB if x.get("guid") == propA.get("guid")), None)
        if propB is None:
            return False
        if propA.get("quality", {}).get("guid") != propB.get("quality", {}).get("guid"):
            return False
        if propA.get("value") != propB.get("value"):
            return False
        if _normalizeValue(propA.get("unit")) != _normalizeValue(propB.get("unit")):
            return False
        if not areAttributesEqualDict(propA.get("attributes"), propB.get("attributes"), strict):
            return False
        if strict:
            if propA.get("createdAt") != propB.get("createdAt"):
                return False
            if propA.get("updatedAt") != propB.get("updatedAt"):
                return False
    return True


def arePortsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for connectorA in arrA:
        connectorB = next((x for x in arrB if x.get("guid") == connectorA.get("guid")), None)
        if connectorB is None:
            return False
        if _normalizeValue(connectorA.get("name")) != _normalizeValue(connectorB.get("name")):
            return False
        pointA = connectorA.get("point", {})
        pointB = connectorB.get("point", {})
        if pointA.get("x") != pointB.get("x") or pointA.get("y") != pointB.get("y") or pointA.get("z") != pointB.get("z"):
            return False
        dirA = connectorA.get("direction", {})
        dirB = connectorB.get("direction", {})
        if dirA.get("x") != dirB.get("x") or dirA.get("y") != dirB.get("y") or dirA.get("z") != dirB.get("z"):
            return False
        if connectorA.get("t") != connectorB.get("t"):
            return False
        if _normalizeBoolean(connectorA.get("mandatory")) != _normalizeBoolean(connectorB.get("mandatory")):
            return False
        ifaceA = connectorA.get("interface", {}) if connectorA.get("interface") else {}
        ifaceB = connectorB.get("interface", {}) if connectorB.get("interface") else {}
        if _normalizeValue(ifaceA.get("guid")) != _normalizeValue(ifaceB.get("guid")):
            return False
        if not arePropsEqualDict(connectorA.get("props"), connectorB.get("props"), strict):
            return False
        if not areAttributesEqualDict(connectorA.get("attributes"), connectorB.get("attributes"), strict):
            return False
        if strict:
            if connectorA.get("createdAt") != connectorB.get("createdAt"):
                return False
            if connectorA.get("updatedAt") != connectorB.get("updatedAt"):
                return False
    return True


def areModelsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for modelA in arrA:
        modelB = next((x for x in arrB if x.get("guid") == modelA.get("guid")), None)
        if modelB is None:
            return False
        if _normalizeValue(modelA.get("name")) != _normalizeValue(modelB.get("name")):
            return False

        fileA = modelA.get("file")
        fileB = modelB.get("file")
        fileGuidA = fileA.get("guid") if isinstance(fileA, dict) else fileA
        fileGuidB = fileB.get("guid") if isinstance(fileB, dict) else fileB
        if fileGuidA != fileGuidB:
            return False
        tagsA = [t.get("guid") if isinstance(t, dict) else t for t in _normalizeArray(modelA.get("tags"))]
        tagsB = [t.get("guid") if isinstance(t, dict) else t for t in _normalizeArray(modelB.get("tags"))]
        if len(tagsA) != len(tagsB) or set(tagsA) != set(tagsB):
            return False
        if not areAttributesEqualDict(modelA.get("attributes"), modelB.get("attributes"), strict):
            return False
        if strict:
            if modelA.get("createdAt") != modelB.get("createdAt"):
                return False
            if modelA.get("updatedAt") != modelB.get("updatedAt"):
                return False
    return True


def areTypesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for typeA in arrA:
        typeB = None
        for t in arrB:
            if t.get("guid") != typeA.get("guid"):
                continue
            parentA = typeA.get("parent")
            parentB = t.get("parent")
            if not parentA and not parentB:
                typeB = t
                break
            if not parentA or not parentB:
                continue

            parentGuidA = parentA.get("guid") if isinstance(parentA, dict) else parentA
            parentGuidB = parentB.get("guid") if isinstance(parentB, dict) else parentB
            if parentGuidA == parentGuidB:
                typeB = t
                break
        if typeB is None:
            return False
        if typeA.get("name") != typeB.get("name"):
            return False
        if _normalizeValue(typeA.get("description")) != _normalizeValue(typeB.get("description")):
            return False
        if _normalizeValue(typeA.get("icon")) != _normalizeValue(typeB.get("icon")):
            return False
        if _normalizeValue(typeA.get("image")) != _normalizeValue(typeB.get("image")):
            return False
        if _normalizeValue(typeA.get("folder")) != _normalizeValue(typeB.get("folder")):
            return False
        if _normalizeValue(typeA.get("unit")) != _normalizeValue(typeB.get("unit")):
            return False
        if typeA.get("stock") != typeB.get("stock"):
            return False
        if _normalizeBoolean(typeA.get("isAbstract")) != _normalizeBoolean(typeB.get("isAbstract")):
            return False
        if _normalizeBoolean(typeA.get("virtual")) != _normalizeBoolean(typeB.get("virtual")):
            return False
        locA = typeA.get("location", {}) if typeA.get("location") else {}
        locB = typeB.get("location", {}) if typeB.get("location") else {}
        if _normalizeValue(locA.get("guid")) != _normalizeValue(locB.get("guid")):
            return False

        conceptsA = _normalizeArray(typeA.get("concepts"))
        conceptsB = _normalizeArray(typeB.get("concepts"))
        conceptGuidsA = [c.get("guid") if isinstance(c, dict) else c for c in conceptsA]
        conceptGuidsB = [c.get("guid") if isinstance(c, dict) else c for c in conceptsB]
        if conceptGuidsA != conceptGuidsB:
            return False
        authA = [a.get("guid") if isinstance(a, dict) else a for a in _normalizeArray(typeA.get("authors"))]
        authB = [a.get("guid") if isinstance(a, dict) else a for a in _normalizeArray(typeB.get("authors"))]
        if authA != authB:
            return False
        if not arePropsEqualDict(typeA.get("props"), typeB.get("props"), strict):
            return False
        if not areModelsEqualDict(typeA.get("models"), typeB.get("models"), strict):
            return False
        if not arePortsEqualDict(typeA.get("connectors"), typeB.get("connectors"), strict):
            return False
        if not areAttributesEqualDict(typeA.get("attributes"), typeB.get("attributes"), strict):
            return False
        if strict:
            if typeA.get("createdAt") != typeB.get("createdAt"):
                return False
            if typeA.get("updatedAt") != typeB.get("updatedAt"):
                return False
    return True


def arePiecesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for pieceA in arrA:
        pieceB = next((x for x in arrB if x.get("guid") == pieceA.get("guid")), None)
        if pieceB is None:
            return False
        if _normalizeValue(pieceA.get("name")) != _normalizeValue(pieceB.get("name")):
            return False

        typeA = pieceA.get("type")
        typeB = pieceB.get("type")
        typeGuidA = typeA.get("guid") if isinstance(typeA, dict) else typeA
        typeGuidB = typeB.get("guid") if isinstance(typeB, dict) else typeB
        if typeGuidA != typeGuidB:
            return False

        designA = pieceA.get("design")
        designB = pieceB.get("design")
        designGuidA = designA.get("guid") if isinstance(designA, dict) else designA
        designGuidB = designB.get("guid") if isinstance(designB, dict) else designB
        if designGuidA != designGuidB:
            return False
        planeA = pieceA.get("plane")
        planeB = pieceB.get("plane")
        if planeA and planeB:
            if planeA.get("origin", {}).get("x") != planeB.get("origin", {}).get("x"):
                return False
            if planeA.get("origin", {}).get("y") != planeB.get("origin", {}).get("y"):
                return False
            if planeA.get("origin", {}).get("z") != planeB.get("origin", {}).get("z"):
                return False
            if planeA.get("xAxis", {}).get("x") != planeB.get("xAxis", {}).get("x"):
                return False
            if planeA.get("xAxis", {}).get("y") != planeB.get("xAxis", {}).get("y"):
                return False
            if planeA.get("xAxis", {}).get("z") != planeB.get("xAxis", {}).get("z"):
                return False
            if planeA.get("yAxis", {}).get("x") != planeB.get("yAxis", {}).get("x"):
                return False
            if planeA.get("yAxis", {}).get("y") != planeB.get("yAxis", {}).get("y"):
                return False
            if planeA.get("yAxis", {}).get("z") != planeB.get("yAxis", {}).get("z"):
                return False
        elif planeA or planeB:
            return False
        centerA = pieceA.get("center")
        centerB = pieceB.get("center")
        if centerA and centerB:
            if centerA.get("u") != centerB.get("u") or centerA.get("v") != centerB.get("v"):
                return False
        elif centerA or centerB:
            return False
        if pieceA.get("scale") != pieceB.get("scale"):
            return False
        if _normalizeBoolean(pieceA.get("isHidden")) != _normalizeBoolean(pieceB.get("isHidden")):
            return False
        if _normalizeBoolean(pieceA.get("isLocked")) != _normalizeBoolean(pieceB.get("isLocked")):
            return False
        if _normalizeValue(pieceA.get("color")) != _normalizeValue(pieceB.get("color")):
            return False
        if _normalizeValue(pieceA.get("description")) != _normalizeValue(pieceB.get("description")):
            return False
        if not arePropsEqualDict(pieceA.get("props"), pieceB.get("props"), strict):
            return False
        if not areAttributesEqualDict(pieceA.get("attributes"), pieceB.get("attributes"), strict):
            return False
        if strict:
            if pieceA.get("createdAt") != pieceB.get("createdAt"):
                return False
            if pieceA.get("updatedAt") != pieceB.get("updatedAt"):
                return False
    return True


def _getGuidFromRef(ref: typing.Any) -> str | None:
    """Extract guid from either a string (Input format) or dict with guid (Output format)."""
    if ref is None:
        return None
    if isinstance(ref, dict):
        return ref.get("guid")
    return ref


def areConnectionsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for connA in arrA:
        connB = next((x for x in arrB if x.get("guid") == connA.get("guid")), None)
        if connB is None:
            return False
        connectedA = connA.get("connected", {})
        connectedB = connB.get("connected", {})

        if _getGuidFromRef(connectedA.get("piece")) != _getGuidFromRef(connectedB.get("piece")):
            return False
        if _getGuidFromRef(connectedA.get("designPiece")) != _getGuidFromRef(connectedB.get("designPiece")):
            return False
        if _getGuidFromRef(connectedA.get("connector")) != _getGuidFromRef(connectedB.get("connector")):
            return False
        connectingA = connA.get("connecting", {})
        connectingB = connB.get("connecting", {})
        if _getGuidFromRef(connectingA.get("piece")) != _getGuidFromRef(connectingB.get("piece")):
            return False
        if _getGuidFromRef(connectingA.get("designPiece")) != _getGuidFromRef(connectingB.get("designPiece")):
            return False
        if _getGuidFromRef(connectingA.get("connector")) != _getGuidFromRef(connectingB.get("connector")):
            return False
        if connA.get("gap") != connB.get("gap"):
            return False
        if connA.get("shift") != connB.get("shift"):
            return False
        if connA.get("rise") != connB.get("rise"):
            return False
        if connA.get("rotation") != connB.get("rotation"):
            return False
        if connA.get("turn") != connB.get("turn"):
            return False
        if connA.get("tilt") != connB.get("tilt"):
            return False
        if connA.get("u") != connB.get("u"):
            return False
        if connA.get("v") != connB.get("v"):
            return False
        if _normalizeValue(connA.get("description")) != _normalizeValue(connB.get("description")):
            return False
        if not areAttributesEqualDict(connA.get("attributes"), connB.get("attributes"), strict):
            return False
        if strict:
            if connA.get("createdAt") != connB.get("createdAt"):
                return False
            if connA.get("updatedAt") != connB.get("updatedAt"):
                return False
    return True


def areDesignsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for designA in arrA:
        designB = None
        for d in arrB:
            if d.get("guid") != designA.get("guid"):
                continue
            parentA = designA.get("parent")
            parentB = d.get("parent")
            if not parentA and not parentB:
                designB = d
                break
            if not parentA or not parentB:
                continue

            parentGuidA = _getGuidFromRef(parentA)
            parentGuidB = _getGuidFromRef(parentB)
            if parentGuidA == parentGuidB:
                designB = d
                break
        if designB is None:
            return False
        if designA.get("name") != designB.get("name"):
            return False
        if _normalizeValue(designA.get("description")) != _normalizeValue(designB.get("description")):
            return False
        if _normalizeValue(designA.get("icon")) != _normalizeValue(designB.get("icon")):
            return False
        if _normalizeValue(designA.get("image")) != _normalizeValue(designB.get("image")):
            return False

        conceptsA = _normalizeArray(designA.get("concepts"))
        conceptsB = _normalizeArray(designB.get("concepts"))
        conceptGuidsA = [_getGuidFromRef(c) for c in conceptsA]
        conceptGuidsB = [_getGuidFromRef(c) for c in conceptsB]
        if conceptGuidsA != conceptGuidsB:
            return False
        if not arePiecesEqualDict(designA.get("pieces"), designB.get("pieces"), strict):
            return False
        if not areConnectionsEqualDict(designA.get("connections"), designB.get("connections"), strict):
            return False
        if not areAttributesEqualDict(designA.get("attributes"), designB.get("attributes"), strict):
            return False
        if strict:
            if designA.get("createdAt") != designB.get("createdAt"):
                return False
            if designA.get("updatedAt") != designB.get("updatedAt"):
                return False
    return True


def areInterfacesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for ifaceA in arrA:
        ifaceB = next((x for x in arrB if x.get("guid") == ifaceA.get("guid")), None)
        if ifaceB is None:
            return False
        if ifaceA.get("name") != ifaceB.get("name"):
            return False
        if _normalizeValue(ifaceA.get("description")) != _normalizeValue(ifaceB.get("description")):
            return False
        if not areAttributesEqualDict(ifaceA.get("attributes"), ifaceB.get("attributes"), strict):
            return False
        if strict:
            if ifaceA.get("createdAt") != ifaceB.get("createdAt"):
                return False
            if ifaceA.get("updatedAt") != ifaceB.get("updatedAt"):
                return False
    return True


def areQualitiesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for qualA in arrA:
        qualB = next((x for x in arrB if x.get("guid") == qualA.get("guid")), None)
        if qualB is None:
            return False
        if qualA.get("key") != qualB.get("key"):
            return False
        if qualA.get("name") != qualB.get("name"):
            return False
        if not areAttributesEqualDict(qualA.get("attributes"), qualB.get("attributes"), strict):
            return False
        if strict:
            if qualA.get("createdAt") != qualB.get("createdAt"):
                return False
            if qualA.get("updatedAt") != qualB.get("updatedAt"):
                return False
    return True


def areFilesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for fileA in arrA:
        fileB = next((x for x in arrB if x.get("guid") == fileA.get("guid")), None)
        if fileB is None:
            return False
        if fileA.get("name") != fileB.get("name"):
            return False
        if strict:
            if fileA.get("createdAt") != fileB.get("createdAt"):
                return False
            if fileA.get("updatedAt") != fileB.get("updatedAt"):
                return False
    return True


def areFoldersEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for folderA in arrA:
        folderB = next((x for x in arrB if x.get("guid") == folderA.get("guid")), None)
        if folderB is None:
            return False
        if folderA.get("name") != folderB.get("name"):
            return False
        if not areAttributesEqualDict(folderA.get("attributes"), folderB.get("attributes"), strict):
            return False
        if strict:
            if folderA.get("createdAt") != folderB.get("createdAt"):
                return False
            if folderA.get("updatedAt") != folderB.get("updatedAt"):
                return False
    return True


def areAuthorsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for authorA in arrA:
        authorB = next((x for x in arrB if x.get("guid") == authorA.get("guid")), None)
        if authorB is None:
            return False
        if authorA.get("name") != authorB.get("name"):
            return False
        if _normalizeValue(authorA.get("email")) != _normalizeValue(authorB.get("email")):
            return False
        if not areAttributesEqualDict(authorA.get("attributes"), authorB.get("attributes"), strict):
            return False
        if strict:
            if authorA.get("createdAt") != authorB.get("createdAt"):
                return False
            if authorA.get("updatedAt") != authorB.get("updatedAt"):
                return False
    return True


def areConceptsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for conceptA in arrA:
        conceptB = next((x for x in arrB if x.get("guid") == conceptA.get("guid")), None)
        if conceptB is None:
            return False
        if conceptA.get("name") != conceptB.get("name"):
            return False
        if _normalizeValue(conceptA.get("description")) != _normalizeValue(conceptB.get("description")):
            return False
        if _normalizeValue(conceptA.get("icon")) != _normalizeValue(conceptB.get("icon")):
            return False
        if strict:
            if conceptA.get("createdAt") != conceptB.get("createdAt"):
                return False
            if conceptA.get("updatedAt") != conceptB.get("updatedAt"):
                return False
    return True


def areTagsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for tagA in arrA:
        tagB = next((x for x in arrB if x.get("guid") == tagA.get("guid")), None)
        if tagB is None:
            return False
        if tagA.get("name") != tagB.get("name"):
            return False
        if _normalizeValue(tagA.get("description")) != _normalizeValue(tagB.get("description")):
            return False
        if _normalizeValue(tagA.get("icon")) != _normalizeValue(tagB.get("icon")):
            return False
        if strict:
            if tagA.get("createdAt") != tagB.get("createdAt"):
                return False
            if tagA.get("updatedAt") != tagB.get("updatedAt"):
                return False
    return True


def areKitsDictEqual(a: dict, b: dict, strict: bool = False) -> bool:
    """Deep equality check for kits (dict-based) - recursively compares all properties including nested entities.

    Args:
        a: First kit dict
        b: Second kit dict
        strict: If True, also compare timestamps (createdAt, updatedAt). Default False.

    Returns:
        True if kits are equal, False otherwise.
    """
    if a.get("guid") != b.get("guid"):
        return False
    if a.get("name") != b.get("name"):
        return False
    if _normalizeValue(a.get("version")) != _normalizeValue(b.get("version")):
        return False
    if _normalizeValue(a.get("description")) != _normalizeValue(b.get("description")):
        return False
    if _normalizeValue(a.get("icon")) != _normalizeValue(b.get("icon")):
        return False
    if _normalizeValue(a.get("image")) != _normalizeValue(b.get("image")):
        return False
    if _normalizeValue(a.get("preview")) != _normalizeValue(b.get("preview")):
        return False
    if _normalizeValue(a.get("remote")) != _normalizeValue(b.get("remote")):
        return False
    if _normalizeValue(a.get("homepage")) != _normalizeValue(b.get("homepage")):
        return False
    if _normalizeValue(a.get("license")) != _normalizeValue(b.get("license")):
        return False
    if not areConceptsEqualDict(a.get("concepts"), b.get("concepts"), strict):
        return False
    if not areTagsEqualDict(a.get("tags"), b.get("tags"), strict):
        return False
    if not areTypesEqualDict(a.get("types"), b.get("types"), strict):
        return False
    if not areDesignsEqualDict(a.get("designs"), b.get("designs"), strict):
        return False
    if not areInterfacesEqualDict(a.get("interfaces"), b.get("interfaces"), strict):
        return False
    if not areQualitiesEqualDict(a.get("qualities"), b.get("qualities"), strict):
        return False
    if not areFilesEqualDict(a.get("files"), b.get("files"), strict):
        return False
    if not areFoldersEqualDict(a.get("folders"), b.get("folders"), strict):
        return False
    if not areAuthorsEqualDict(a.get("authors"), b.get("authors"), strict):
        return False
    if not areAttributesEqualDict(a.get("attributes"), b.get("attributes"), strict):
        return False
    if strict:
        if a.get("createdAt") != b.get("createdAt"):
            return False
        if a.get("updatedAt") != b.get("updatedAt"):
            return False
    return True


def _getCollectionDiff(before: list, after: list, getItemDiff: typing.Callable[[dict, dict], dict], entityKey: str = "") -> dict:
    """Get diff for a collection of items identified by guid.

    Args:
        before: The before collection
        after: The after collection
        getItemDiff: Function to get item-level diff
        entityKey: The key name for the entity ID in the updated array (e.g., "type", "design", "piece")
    """
    diff: dict = {}
    beforeGuids = {item.get("guid") for item in before}
    afterGuids = {item.get("guid") for item in after}

    removed = [{"guid": item.get("guid")} for item in before if item.get("guid") not in afterGuids]
    if removed:
        diff["removed"] = removed
    updated = []
    for item in before:
        if item.get("guid") in afterGuids:
            afterItem = next(a for a in after if a.get("guid") == item.get("guid"))
            itemDiff = getItemDiff(item, afterItem)
            if itemDiff:
                if entityKey:
                    updated.append({entityKey: {"guid": item.get("guid")}, "diff": itemDiff})
                else:
                    updated.append({"id": item.get("guid"), "diff": itemDiff})
    if updated:
        diff["updated"] = updated
    added = [item for item in after if item.get("guid") not in beforeGuids]
    if added:
        diff["added"] = added
    return diff


def _applyCollectionDiff(base: list, diff: dict | None, applyItemDiff: typing.Callable[[dict, dict], dict], entityKey: str = "") -> list:
    """Apply diff to a collection of items.

    Args:
        base: The base collection
        diff: The diff to apply (with removed, updated, added)
        applyItemDiff: Function to apply item-level diff
        entityKey: The key name for the entity ID in the updated array (e.g., "type", "design", "piece")
    """
    if not diff:
        return base
    result = [dict(item) for item in base]
    if diff.get("removed"):
        removedGuids = [r["guid"] if isinstance(r, dict) else r for r in diff["removed"]]
        result = [item for item in result if item.get("guid") not in removedGuids]
    if diff.get("updated"):
        for update in diff["updated"]:
            updateGuid = update[entityKey]["guid"] if entityKey and entityKey in update else update.get("id", "")
            idx = next((i for i, item in enumerate(result) if item.get("guid") == updateGuid), -1)
            if idx >= 0:
                result[idx] = applyItemDiff(result[idx], update["diff"])
    if diff.get("added"):
        result.extend(diff["added"])
    return result


def _getTypeDiff(before: dict, after: dict) -> dict:
    """Get diff between two type dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    if _normalizeValue(before.get("image")) != _normalizeValue(after.get("image")):
        diff["image"] = after.get("image")
    if _normalizeValue(before.get("unit")) != _normalizeValue(after.get("unit")):
        diff["unit"] = after.get("unit")
    if _normalizeBoolean(before.get("isAbstract")) != _normalizeBoolean(after.get("isAbstract")):
        diff["isAbstract"] = after.get("isAbstract")
    if _normalizeBoolean(before.get("virtual")) != _normalizeBoolean(after.get("virtual")):
        diff["virtual"] = after.get("virtual")
    connectorsDiff = _getCollectionDiff(before.get("connectors", []), after.get("connectors", []), _getConnectorDiff, "connector")
    if connectorsDiff:
        diff["connectors"] = connectorsDiff
    modelsDiff = _getCollectionDiff(before.get("models", []), after.get("models", []), _getModelDiff, "model")
    if modelsDiff:
        diff["models"] = modelsDiff
    return diff


def _applyTypeDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a type dict."""
    result = dict(base)
    for key in ["name", "description", "icon", "image", "unit", "isAbstract", "virtual"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("connectors"):
        result["connectors"] = _applyCollectionDiff(base.get("connectors", []), diff["connectors"], _applyConnectorDiff, "connector")
    if diff.get("models"):
        result["models"] = _applyCollectionDiff(base.get("models", []), diff["models"], _applyModelDiff, "model")
    return result


def _getConnectorDiff(before: dict, after: dict) -> dict:
    """Get diff between two connector dicts."""
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    if before.get("t") != after.get("t"):
        diff["t"] = after.get("t")
    if _normalizeBoolean(before.get("mandatory")) != _normalizeBoolean(after.get("mandatory")):
        diff["mandatory"] = after.get("mandatory")
    return diff


def _applyConnectorDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a connector dict."""
    result = dict(base)
    for key in ["name", "t", "mandatory"]:
        if key in diff:
            result[key] = diff[key]
    return result


def _getModelDiff(before: dict, after: dict) -> dict:
    """Get diff between two model dicts."""
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    return diff


def _applyModelDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a model dict."""
    result = dict(base)
    if "name" in diff:
        result["name"] = diff["name"]
    return result


def _getDesignDiff(before: dict, after: dict) -> dict:
    """Get diff between two design dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    if _normalizeValue(before.get("image")) != _normalizeValue(after.get("image")):
        diff["image"] = after.get("image")
    piecesDiff = _getCollectionDiff(before.get("pieces", []), after.get("pieces", []), _getPieceDiff, "piece")
    if piecesDiff:
        diff["pieces"] = piecesDiff
    connectionsDiff = _getCollectionDiff(before.get("connections", []), after.get("connections", []), _getConnectionDiff, "connection")
    if connectionsDiff:
        diff["connections"] = connectionsDiff
    return diff


def _applyDesignDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a design dict."""
    result = dict(base)
    for key in ["name", "description", "icon", "image"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("pieces"):
        result["pieces"] = _applyCollectionDiff(base.get("pieces", []), diff["pieces"], _applyPieceDiff, "piece")
    if diff.get("connections"):
        result["connections"] = _applyCollectionDiff(base.get("connections", []), diff["connections"], _applyConnectionDiff, "connection")
    return result


def _getPieceDiff(before: dict, after: dict) -> dict:
    """Get diff between two piece dicts."""
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    if before.get("scale") != after.get("scale"):
        diff["scale"] = after.get("scale")
    return diff


def _applyPieceDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a piece dict."""
    result = dict(base)
    for key in ["name", "scale", "plane", "center"]:
        if key in diff:
            result[key] = diff[key]
    return result


def _getConnectionDiff(before: dict, after: dict) -> dict:
    """Get diff between two connection dicts."""
    diff: dict = {}
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        if before.get(key) != after.get(key):
            diff[key] = after.get(key)
    return diff


def _applyConnectionDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a connection dict."""
    result = dict(base)
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        if key in diff:
            result[key] = diff[key]
    return result


def _getTagDiff(before: dict, after: dict) -> dict:
    """Get diff between two tag dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    return diff


def _applyTagDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a tag dict."""
    result = dict(base)
    for key in ["name", "description"]:
        if key in diff:
            result[key] = diff[key]
    return result


def _getConceptDiff(before: dict, after: dict) -> dict:
    """Get diff between two concept dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    return diff


def _applyConceptDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a concept dict."""
    result = dict(base)
    for key in ["name", "description"]:
        if key in diff:
            result[key] = diff[key]
    return result


def _getInterfaceDiff(before: dict, after: dict) -> dict:
    """Get diff between two interface dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    return diff


def _applyInterfaceDiff(base: dict, diff: dict) -> dict:
    """Apply diff to an interface dict."""
    result = dict(base)
    for key in ["name", "description"]:
        if key in diff:
            result[key] = diff[key]
    return result


def _getFileDiff(before: dict, after: dict) -> dict:
    """Get diff between two file dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    return diff


def _applyFileDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a file dict."""
    result = dict(base)
    if "name" in diff:
        result["name"] = diff["name"]
    return result


def _getFolderDiff(before: dict, after: dict) -> dict:
    """Get diff between two folder dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    return diff


def _applyFolderDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a folder dict."""
    result = dict(base)
    if "name" in diff:
        result["name"] = diff["name"]
    return result


def _getQualityDiff(before: dict, after: dict) -> dict:
    """Get diff between two quality dicts."""
    diff: dict = {}
    if before.get("key") != after.get("key"):
        diff["key"] = after.get("key")
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("uri")) != _normalizeValue(after.get("uri")):
        diff["uri"] = after.get("uri")
    if before.get("kind") != after.get("kind"):
        diff["kind"] = after.get("kind")
    if _normalizeBoolean(before.get("canScale")) != _normalizeBoolean(after.get("canScale")):
        diff["canScale"] = after.get("canScale")
    if _normalizeValue(before.get("defaultSiUnit")) != _normalizeValue(after.get("defaultSiUnit")):
        diff["defaultSiUnit"] = after.get("defaultSiUnit")
    if _normalizeValue(before.get("defaultImperialUnit")) != _normalizeValue(after.get("defaultImperialUnit")):
        diff["defaultImperialUnit"] = after.get("defaultImperialUnit")
    if before.get("min") != after.get("min"):
        diff["min"] = after.get("min")
    if _normalizeBoolean(before.get("isMinExcluded")) != _normalizeBoolean(after.get("isMinExcluded")):
        diff["isMinExcluded"] = after.get("isMinExcluded")
    if before.get("max") != after.get("max"):
        diff["max"] = after.get("max")
    if _normalizeBoolean(before.get("isMaxExcluded")) != _normalizeBoolean(after.get("isMaxExcluded")):
        diff["isMaxExcluded"] = after.get("isMaxExcluded")
    if before.get("defaultValue") != after.get("defaultValue"):
        diff["defaultValue"] = after.get("defaultValue")
    if _normalizeValue(before.get("formula")) != _normalizeValue(after.get("formula")):
        diff["formula"] = after.get("formula")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    if _normalizeValue(before.get("image")) != _normalizeValue(after.get("image")):
        diff["image"] = after.get("image")
    if _normalizeValue(before.get("unit")) != _normalizeValue(after.get("unit")):
        diff["unit"] = after.get("unit")
    return diff


def _applyQualityDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a quality dict."""
    result = dict(base)
    for key in ["key", "name", "description", "uri", "kind", "canScale", "defaultSiUnit", "defaultImperialUnit", "min", "isMinExcluded", "max", "isMaxExcluded", "defaultValue", "formula", "icon", "image", "unit"]:
        if key in diff:
            result[key] = diff[key]
    return result


def _getAuthorDiff(before: dict, after: dict) -> dict:
    """Get diff between two author dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("email")) != _normalizeValue(after.get("email")):
        diff["email"] = after.get("email")
    return diff


def _applyAuthorDiff(base: dict, diff: dict) -> dict:
    """Apply diff to an author dict."""
    result = dict(base)
    for key in ["name", "email"]:
        if key in diff:
            result[key] = diff[key]
    return result


def _getAttributeDiff(before: dict, after: dict) -> dict:
    """Get diff between two attribute dicts - used for individual attribute update diffs."""
    diff: dict = {}

    if _normalizeValue(before.get("value")) != _normalizeValue(after.get("value")):
        diff["value"] = after.get("value")
    if _normalizeValue(before.get("definition")) != _normalizeValue(after.get("definition")):
        diff["definition"] = after.get("definition")
    return diff


def _applyAttributeDiff(base: dict, diff: dict) -> dict:
    """Apply diff to an attribute dict."""
    result = dict(base)
    for key in ["value", "definition"]:
        if key in diff:
            result[key] = diff[key]
    return result


def _getAttributesDiff(before: list, after: list) -> dict:
    """Get diff for attributes collection - uses GUID for identification with EntityId format."""
    diff: dict = {}
    beforeGuids = {a.get("guid") for a in before}
    afterGuids = {a.get("guid") for a in after}

    removed = [{"guid": a.get("guid")} for a in before if a.get("guid") not in afterGuids]
    if removed:
        diff["removed"] = removed
    updated = []
    for afterAttr in after:
        guid = afterAttr.get("guid")
        if guid in beforeGuids:
            beforeAttr = next(a for a in before if a.get("guid") == guid)
            attrDiff = _getAttributeDiff(beforeAttr, afterAttr)
            if attrDiff:
                updated.append({"attribute": {"guid": guid}, "diff": attrDiff})
    if updated:
        diff["updated"] = updated
    added = [a for a in after if a.get("guid") not in beforeGuids]
    if added:
        diff["added"] = added
    return diff


def _applyAttributesDiff(base: list, diff: dict | None) -> list:
    """Apply diff to attributes collection - uses GUID for identification with EntityId format."""
    if not diff:
        return base
    result = [dict(a) for a in base]
    if diff.get("removed"):
        removedGuids = {r["guid"] if isinstance(r, dict) else r for r in diff["removed"]}
        result = [a for a in result if a.get("guid") not in removedGuids]
    if diff.get("updated"):
        for update in diff["updated"]:
            updateGuid = update["attribute"]["guid"] if "attribute" in update else update.get("id", "")
            idx = next((i for i, a in enumerate(result) if a.get("guid") == updateGuid), -1)
            if idx >= 0:
                result[idx] = _applyAttributeDiff(result[idx], update["diff"])
    if diff.get("added"):
        result.extend(diff["added"])
    return result


def _inverseAttributesDiff(original: list, appliedDiff: dict) -> dict:
    """Compute inverse of attributes collection diff - uses GUID with EntityId format."""
    inverse: dict = {}

    removedGuids = [r["guid"] if isinstance(r, dict) else r for r in appliedDiff.get("removed", [])]

    updatedGuids = []
    for u in appliedDiff.get("updated", []):
        if "attribute" in u:
            updatedGuids.append(u["attribute"]["guid"])
        else:
            updatedGuids.append(u.get("id", ""))
    addedGuids = [a.get("guid") for a in appliedDiff.get("added", [])]
    if addedGuids:
        inverse["removed"] = [{"guid": guid} for guid in addedGuids]
    if updatedGuids:
        inverse["updated"] = []
        for guid in updatedGuids:
            origAttr = next((a for a in original if a.get("guid") == guid), None)
            upd = next((u for u in appliedDiff.get("updated", []) if (u.get("attribute", {}).get("guid") if "attribute" in u else u.get("id")) == guid), None)
            if origAttr and upd:
                inverse["updated"].append({"attribute": {"guid": guid}, "diff": _inverseAttributeDiff(origAttr, upd["diff"])})
    if removedGuids:
        inverse["added"] = [a for a in original if a.get("guid") in removedGuids]
    return inverse


def _inverseAttributeDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of an attribute diff."""
    inverse: dict = {}
    if "value" in appliedDiff:
        inverse["value"] = original.get("value")
    if "definition" in appliedDiff:
        inverse["definition"] = original.get("definition")
    return inverse


def getKitDiffDict(before: dict, after: dict) -> dict:
    """Compute the diff between two kit dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if before.get("version") != after.get("version"):
        diff["version"] = after.get("version")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    if _normalizeValue(before.get("image")) != _normalizeValue(after.get("image")):
        diff["image"] = after.get("image")
    if _normalizeValue(before.get("remote")) != _normalizeValue(after.get("remote")):
        diff["remote"] = after.get("remote")
    if _normalizeValue(before.get("homepage")) != _normalizeValue(after.get("homepage")):
        diff["homepage"] = after.get("homepage")
    if _normalizeValue(before.get("license")) != _normalizeValue(after.get("license")):
        diff["license"] = after.get("license")
    if _normalizeValue(before.get("preview")) != _normalizeValue(after.get("preview")):
        diff["preview"] = after.get("preview")
    typesDiff = _getCollectionDiff(before.get("types", []), after.get("types", []), _getTypeDiff, "type")
    if typesDiff:
        diff["types"] = typesDiff
    designsDiff = _getCollectionDiff(before.get("designs", []), after.get("designs", []), _getDesignDiff, "design")
    if designsDiff:
        diff["designs"] = designsDiff
    tagsDiff = _getCollectionDiff(before.get("tags", []), after.get("tags", []), _getTagDiff, "tag")
    if tagsDiff:
        diff["tags"] = tagsDiff
    conceptsDiff = _getCollectionDiff(before.get("concepts", []), after.get("concepts", []), _getConceptDiff, "concept")
    if conceptsDiff:
        diff["concepts"] = conceptsDiff
    interfacesDiff = _getCollectionDiff(before.get("interfaces", []), after.get("interfaces", []), _getInterfaceDiff, "interface")
    if interfacesDiff:
        diff["interfaces"] = interfacesDiff
    filesDiff = _getCollectionDiff(before.get("files", []), after.get("files", []), _getFileDiff, "file")
    if filesDiff:
        diff["files"] = filesDiff
    foldersDiff = _getCollectionDiff(before.get("folders", []), after.get("folders", []), _getFolderDiff, "folder")
    if foldersDiff:
        diff["folders"] = foldersDiff
    qualitiesDiff = _getCollectionDiff(before.get("qualities", []), after.get("qualities", []), _getQualityDiff, "quality")
    if qualitiesDiff:
        diff["qualities"] = qualitiesDiff
    authorsDiff = _getCollectionDiff(before.get("authors", []), after.get("authors", []), _getAuthorDiff, "author")
    if authorsDiff:
        diff["authors"] = authorsDiff
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def applyKitDiffDict(base: dict, diff: dict) -> dict:
    """Apply a diff to a kit dict."""
    result = dict(base)
    result["guid"] = base.get("guid")
    for key in ["name", "version", "description", "icon", "image", "remote", "homepage", "license", "preview"]:
        if key in diff:
            value = diff[key]
            if value is not None:
                result[key] = value
            elif key in result:
                del result[key]
        elif key in base:
            result[key] = base[key]
    if diff.get("types") or base.get("types"):
        result["types"] = _applyCollectionDiff(base.get("types", []), diff.get("types"), _applyTypeDiff, "type")
    if diff.get("designs") or base.get("designs"):
        result["designs"] = _applyCollectionDiff(base.get("designs", []), diff.get("designs"), _applyDesignDiff, "design")
    if diff.get("tags") or base.get("tags"):
        result["tags"] = _applyCollectionDiff(base.get("tags", []), diff.get("tags"), _applyTagDiff, "tag")
    if diff.get("concepts") or base.get("concepts"):
        result["concepts"] = _applyCollectionDiff(base.get("concepts", []), diff.get("concepts"), _applyConceptDiff, "concept")
    if diff.get("interfaces") or base.get("interfaces"):
        result["interfaces"] = _applyCollectionDiff(base.get("interfaces", []), diff.get("interfaces"), _applyInterfaceDiff, "interface")
    if diff.get("files") or base.get("files"):
        result["files"] = _applyCollectionDiff(base.get("files", []), diff.get("files"), _applyFileDiff, "file")
    if diff.get("folders") or base.get("folders"):
        result["folders"] = _applyCollectionDiff(base.get("folders", []), diff.get("folders"), _applyFolderDiff, "folder")
    if diff.get("qualities") or base.get("qualities"):
        result["qualities"] = _applyCollectionDiff(base.get("qualities", []), diff.get("qualities"), _applyQualityDiff, "quality")
    if diff.get("authors") or base.get("authors"):
        result["authors"] = _applyCollectionDiff(base.get("authors", []), diff.get("authors"), _applyAuthorDiff, "author")
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _inverseCollectionDiff(original: list, appliedDiff: dict, inverseItemDiff: typing.Callable[[dict, dict], dict], entityKey: str = "") -> dict:
    """Compute inverse of a collection diff.

    Args:
        original: The original collection before diff was applied
        appliedDiff: The diff that was applied
        inverseItemDiff: Function to compute inverse of item-level diff
        entityKey: The key name for the entity ID (e.g., "type", "design", "piece")
    """
    inverse: dict = {}
    if appliedDiff.get("removed"):
        removedGuids = [r["guid"] if isinstance(r, dict) else r for r in appliedDiff["removed"]]
        inverse["added"] = [item for item in original if item.get("guid") in removedGuids]
    if appliedDiff.get("added"):
        inverse["removed"] = [{"guid": item.get("guid")} for item in appliedDiff["added"]]
    if appliedDiff.get("updated"):
        inverse["updated"] = []
        for update in appliedDiff["updated"]:
            updateGuid = update[entityKey]["guid"] if entityKey and entityKey in update else update.get("id", "")
            origItem = next((item for item in original if item.get("guid") == updateGuid), None)
            if origItem:
                if entityKey:
                    inverse["updated"].append({entityKey: {"guid": updateGuid}, "diff": inverseItemDiff(origItem, update["diff"])})
                else:
                    inverse["updated"].append({"id": updateGuid, "diff": inverseItemDiff(origItem, update["diff"])})
    return inverse


def _inverseTypeDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a type diff."""
    inverse: dict = {}
    for key in ["name", "description", "icon", "image", "unit", "isAbstract", "virtual"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("connectors"):
        inverse["connectors"] = _inverseCollectionDiff(original.get("connectors", []), appliedDiff["connectors"], _inversePortDiff, "connector")
    return inverse


def _inversePortDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a connector diff."""
    inverse: dict = {}
    for key in ["name", "t", "mandatory"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse


def _inverseDesignDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a design diff."""
    inverse: dict = {}
    for key in ["name", "description", "icon", "image"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("pieces"):
        inverse["pieces"] = _inverseCollectionDiff(original.get("pieces", []), appliedDiff["pieces"], _inversePieceDiff, "piece")
    return inverse


def _inversePieceDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a piece diff."""
    inverse: dict = {}
    for key in ["name", "scale", "plane", "center"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse


def _inverseTagDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a tag diff."""
    inverse: dict = {}
    for key in ["name", "description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse


def _inverseConceptDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a concept diff."""
    inverse: dict = {}
    for key in ["name", "description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse


def _inverseInterfaceDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of an interface diff."""
    inverse: dict = {}
    for key in ["name", "description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse


def _inverseFileDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a file diff."""
    inverse: dict = {}
    if "name" in appliedDiff:
        inverse["name"] = original.get("name")
    return inverse


def _inverseFolderDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a folder diff."""
    inverse: dict = {}
    if "name" in appliedDiff:
        inverse["name"] = original.get("name")
    return inverse


def _inverseQualityDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a quality diff."""
    inverse: dict = {}
    for key in ["key", "name", "description", "uri", "kind", "canScale", "defaultSiUnit", "defaultImperialUnit", "min", "isMinExcluded", "max", "isMaxExcluded", "defaultValue", "formula", "icon", "image", "unit"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse


def _inverseAuthorDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of an author diff."""
    inverse: dict = {}
    for key in ["name", "email"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse


def inverseKitDiffDict(original: dict, appliedDiff: dict) -> dict:
    """Compute the inverse of a kit diff."""
    inverse: dict = {}
    for key in ["name", "version", "description", "icon", "image", "remote", "homepage", "license", "preview"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("types"):
        inverse["types"] = _inverseCollectionDiff(original.get("types", []), appliedDiff["types"], _inverseTypeDiff, "type")
    if appliedDiff.get("designs"):
        inverse["designs"] = _inverseCollectionDiff(original.get("designs", []), appliedDiff["designs"], _inverseDesignDiff, "design")
    if appliedDiff.get("tags"):
        inverse["tags"] = _inverseCollectionDiff(original.get("tags", []), appliedDiff["tags"], _inverseTagDiff, "tag")
    if appliedDiff.get("concepts"):
        inverse["concepts"] = _inverseCollectionDiff(original.get("concepts", []), appliedDiff["concepts"], _inverseConceptDiff, "concept")
    if appliedDiff.get("interfaces"):
        inverse["interfaces"] = _inverseCollectionDiff(original.get("interfaces", []), appliedDiff["interfaces"], _inverseInterfaceDiff, "interface")
    if appliedDiff.get("files"):
        inverse["files"] = _inverseCollectionDiff(original.get("files", []), appliedDiff["files"], _inverseFileDiff, "file")
    if appliedDiff.get("folders"):
        inverse["folders"] = _inverseCollectionDiff(original.get("folders", []), appliedDiff["folders"], _inverseFolderDiff, "folder")
    if appliedDiff.get("qualities"):
        inverse["qualities"] = _inverseCollectionDiff(original.get("qualities", []), appliedDiff["qualities"], _inverseQualityDiff, "quality")
    if appliedDiff.get("authors"):
        inverse["authors"] = _inverseCollectionDiff(original.get("authors", []), appliedDiff["authors"], _inverseAuthorDiff, "author")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def _extractUpdateGuid(update: dict, entityKeys: list[str]) -> str:
    """Extract guid from an updated entry which might use EntityId format or old id format."""
    for key in entityKeys:
        if key in update and isinstance(update[key], dict):
            return update[key].get("guid", "")
    return update.get("id", "")


def areKitDiffsDictEqual(a: dict, b: dict) -> bool:
    """Deep equality check for kit diffs."""
    keys = ["name", "version", "description", "icon", "image", "remote", "homepage", "license", "preview"]
    for key in keys:
        if _normalizeValue(a.get(key)) != _normalizeValue(b.get(key)):
            return False

    collectionConfig = [
        ("types", "type"),
        ("designs", "design"),
        ("tags", "tag"),
        ("concepts", "concept"),
        ("interfaces", "interface"),
        ("files", "file"),
        ("folders", "folder"),
        ("attributes", "attribute"),
    ]
    for collectionKey, entityKey in collectionConfig:
        diffA = a.get(collectionKey, {})
        diffB = b.get(collectionKey, {})

        removedA = {r["guid"] if isinstance(r, dict) else r for r in diffA.get("removed", [])}
        removedB = {r["guid"] if isinstance(r, dict) else r for r in diffB.get("removed", [])}
        if removedA != removedB:
            return False
        addedA = {item.get("guid"): item for item in diffA.get("added", [])}
        addedB = {item.get("guid"): item for item in diffB.get("added", [])}
        if set(addedA.keys()) != set(addedB.keys()):
            return False

        updatedA = {_extractUpdateGuid(u, [entityKey]): u["diff"] for u in diffA.get("updated", [])}
        updatedB = {_extractUpdateGuid(u, [entityKey]): u["diff"] for u in diffB.get("updated", [])}
        if set(updatedA.keys()) != set(updatedB.keys()):
            return False
    return True


# endregion Kit Diff Operations

# region Spatial Math


def normalizeVector(v: numpy.ndarray) -> numpy.ndarray:
    length = numpy.linalg.norm(v)
    if length < 1e-10:
        return v
    return v / length


def planeFromYAxis(yAxis: numpy.ndarray, phiDegrees: float = 0.0, origin: numpy.ndarray | None = None) -> Plane:
    if origin is None:
        origin = numpy.array([0.0, 0.0, 0.0])
    yAxis = normalizeVector(yAxis)
    worldY = numpy.array([0.0, 1.0, 0.0])
    if numpy.allclose(yAxis, worldY, atol=1e-6):
        rotationToY = numpy.eye(3)
    elif numpy.allclose(yAxis, -worldY, atol=1e-6):
        rotationToY = pytransform3d.rotations.matrix_from_axis_angle([1, 0, 0, numpy.pi])
    else:
        axis = numpy.cross(worldY, yAxis)
        axis = normalizeVector(axis)
        angle = numpy.arccos(numpy.clip(numpy.dot(worldY, yAxis), -1.0, 1.0))
        rotationToY = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([axis, [angle]]))
    phiRadians = numpy.deg2rad(phiDegrees)
    rotationAroundY = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([yAxis, [phiRadians]]))
    worldX = numpy.array([1.0, 0.0, 0.0])
    xAxis = rotationAroundY @ rotationToY @ worldX
    xAxis = normalizeVector(xAxis)
    plane = Plane()
    plane.origin = Point(x=float(origin[0]), y=float(origin[1]), z=float(origin[2]))
    plane.xAxis = Vector(x=float(xAxis[0]), y=float(xAxis[1]), z=float(xAxis[2]))
    plane.yAxis = Vector(x=float(yAxis[0]), y=float(yAxis[1]), z=float(yAxis[2]))
    return plane


def computeChildPlane(parentPlane: Plane, parentConnector: Connector, childConnector: Connector, connection: Connection) -> Plane:
    gap = connection.gap or 0
    shift = connection.shift or 0
    rise = connection.rise or 0
    rotation = connection.rotation or 0
    turn = connection.turn or 0
    tilt = connection.tilt or 0
    pOrigin = numpy.array([parentPlane.origin.x, parentPlane.origin.y, parentPlane.origin.z])
    pX = numpy.array([parentPlane.xAxis.x, parentPlane.xAxis.y, parentPlane.xAxis.z])
    pY = numpy.array([parentPlane.yAxis.x, parentPlane.yAxis.y, parentPlane.yAxis.z])
    pZ = numpy.cross(pX, pY)
    parentMatrix = numpy.eye(4)
    parentMatrix[:3, 0] = pX
    parentMatrix[:3, 1] = pY
    parentMatrix[:3, 2] = pZ
    parentMatrix[:3, 3] = pOrigin
    ppPoint = numpy.array([parentConnector.point.x, parentConnector.point.y, parentConnector.point.z])
    ppDir = numpy.array([parentConnector.direction.x, parentConnector.direction.y, parentConnector.direction.z])
    cpPoint = numpy.array([childConnector.point.x, childConnector.point.y, childConnector.point.z])
    cpDir = numpy.array([childConnector.direction.x, childConnector.direction.y, childConnector.direction.z])
    ppWorld = parentMatrix[:3, :3] @ ppPoint + parentMatrix[:3, 3]
    ppDirWorld = parentMatrix[:3, :3] @ ppDir
    ppDirWorld = normalizeVector(ppDirWorld)
    translation = ppWorld + gap * ppDirWorld + shift * numpy.cross(ppDirWorld, pZ) + rise * pZ
    targetDir = -ppDirWorld
    cpDirNormalized = normalizeVector(cpDir)
    if numpy.allclose(cpDirNormalized, targetDir, atol=1e-6):
        baseRotation = numpy.eye(3)
    elif numpy.allclose(cpDirNormalized, -targetDir, atol=1e-6):
        axis = numpy.array([1.0, 0.0, 0.0])
        if numpy.allclose(numpy.abs(cpDirNormalized), axis, atol=1e-6):
            axis = numpy.array([0.0, 1.0, 0.0])
        baseRotation = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([axis, [numpy.pi]]))
    else:
        axis = numpy.cross(cpDirNormalized, targetDir)
        axis = normalizeVector(axis)
        angle = numpy.arccos(numpy.clip(numpy.dot(cpDirNormalized, targetDir), -1.0, 1.0))
        baseRotation = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([axis, [angle]]))
    rotRad = numpy.deg2rad(rotation)
    rotationMatrix = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([targetDir, [rotRad]]))
    turnRad = numpy.deg2rad(turn)
    pZWorld = normalizeVector(pZ)
    turnMatrix = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([pZWorld, [turnRad]]))
    tiltRad = numpy.deg2rad(tilt)
    pXWorld = normalizeVector(parentMatrix[:3, :3] @ numpy.array([1, 0, 0]))
    tiltMatrix = pytransform3d.rotations.matrix_from_axis_angle(numpy.concatenate([pXWorld, [tiltRad]]))
    combinedRotation = tiltMatrix @ turnMatrix @ rotationMatrix @ baseRotation
    childOrigin = translation - combinedRotation @ cpPoint
    childX = combinedRotation @ numpy.array([1, 0, 0])
    childY = combinedRotation @ numpy.array([0, 1, 0])
    plane = Plane()
    plane.origin = Point(x=float(childOrigin[0]), y=float(childOrigin[1]), z=float(childOrigin[2]))
    plane.xAxis = Vector(x=float(childX[0]), y=float(childX[1]), z=float(childX[2]))
    plane.yAxis = Vector(x=float(childY[0]), y=float(childY[1]), z=float(childY[2]))
    return plane


# endregion Spatial Math

# region Store


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
    """🏪 The kind of the store."""

    DATABASE = "database"
    REST = "rest"
    GRAPHQL = "graphql"


class CommandKind(enum.Enum):
    """🔧 The kind of the command."""

    QUERY = "query"
    PUT = "put"
    UPDATE = "update"
    DELETE = "delete"


class Store(abc.ABC):
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
        """🏗️ Initialize the store and perform nothing if was already initialized."""
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
        """🗑️ Delete an entity from the store."""
        pass


class DatabaseStore(Store, abc.ABC):
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

    def put(self: "DatabaseStore", operation: dict, input: KitInput | DesignInput | TypeInput) -> typing.Any:
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
    """🔒 The security level of the session"""

    DISABLE = "disable"
    ALLOW = "allow"
    PREFER = "prefer"
    REQUIRE = "require"
    VERIFY_CA = "verify-ca"
    VERIFY_FULL = "verify-full"


def cacheDir(remoteUri: str) -> str:
    cacheDir = os.path.expanduser("~/.semio/cache")
    encodedUri = encode(remoteUri)
    return os.path.join(cacheDir, encodedUri)


def cache(remoteUri: str) -> str:
    """📦 Cache a remote kit and delete the existing cache if it was already cached."""
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
            existingsemio = session.query(Semio).one_or_none()
            if not existingSemio:
                session.add(Semio())
                session.commit()

    def postDeleteKit(self: "SqliteStore") -> None:
        os.kill(os.getpid(), signal.SIGTERM)


class PostgresStore(DatabaseStore):
    @classmethod
    def fromUri(cls, uri: str):
        # TODO: Get connection string from environment variable.

        raise FeatureNotYetSupported()

    def initialize(self: "DatabaseStore") -> None:
        sqlmodel.SQLModel.metadata.create_all(self.engine)


@functools.lru_cache
def StoreFactory(uri: str) -> Store:
    """🏭 Get a store from the uri. This store doesn't need to exist yet as long as it can be created."""
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
    codeTree = codeParser.parse(code)
    operation = OperationBuilder().transform(codeTree)
    store = StoreFactory(operation["kitUri"])
    return store, operation


def get(code: str, cache=False) -> typing.Any:
    """🔍 Get an entity from the store."""
    store, operation = storeAndOperationFromCode(code)
    return store.get(operation)


def put(code: str, input: str) -> typing.Any:
    """📥 Put an entity in the store."""
    store, operation = storeAndOperationFromCode(code)
    return store.put(operation, input)


def delete(code: str) -> typing.Any:
    """🗑️ Delete an entity from the store."""
    store, operation = storeAndOperationFromCode(code)
    return store.delete(operation)


# endregion Store

# region Assistant


def encodeForPrompt(context: str):
    return context.replace(";", ",").replace("\n", " ")


def replaceDefault(context: str, default: str):
    if context == "":
        return context.replace("", default)
    return context


def encodeType(type: TypeContext):
    typeClone = type.model_copy(deep=True)
    typeClone.variant = replaceDefault(typeClone.variant, "DEFAULT")
    typeClone.description = encodeForPrompt(typeClone.description) if typeClone.description != "" else "NO_DESCRIPTION"
    for connector in typeClone.connectors:
        connector.id_ = replaceDefault(connector.id_, "DEFAULT")

    return typeClone


def decodeDesign(design: dict):
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


# TODO: Replace prototype healing with one that makes more for every single property.
def healDesign(design: DesignPrediction, types: list[TypeContext]):
    """🩺 Heal a design by replacing missing type variants with the first variant."""
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
            except Error:  # TODO: Make more specific
                piece.type.name = list(typeD.keys())[0]
        if piece.type and piece.type.name and piece.type.variant not in typeD[piece.type.name]:
            try:
                piece.type.variant = difflib.get_close_matches(piece.type.variant, typeD[piece.type.name].keys(), n=1)[0]
            except Error:  # TODO: Make more specific
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
If the connectors of connected and connecting pieces have a interface, they should be compatible.
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
{%- raw %}{{% endraw -%}{{ connector.id_ }};{{ connector.description }};{{ connector.interface }}
{%- for compatibleInterface in connector.compatibleInterfaces %}
{%- raw %}{{% endraw -%}
{{ compatibleInterface }}
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
    """🔮 Predict a design based on a description, the types that should be used and an optional base design."""
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
        logger.debug("Predicted Design Raw: {}", json.dumps(json.loads(result.message.content), indent=4))

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
    node = RelayNode.Field()
    kit = graphene.Field(KitNode, uri=graphene.String(required=True))

    def resolve_kit(self, info, uri):
        return get(encode(uri))


class Mutation(graphene.ObjectType):
    createKit = graphene.Field(KitNode, kit=KitInputNode(required=True))


graphqlSchema = graphene.Schema(
    query=Query,
    mutation=Mutation,
)

# endregion Graphql

# region Rest

rest = fastapi.FastAPI(max_request_body_size=MAX_REQUEST_BODY_SIZE)


@rest.get("/kits/{encodedKitUri}")
async def kit(
    request: fastapi.Request,
    encodedKitUri: ENCODED_PATH,
) -> KitOutput:
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
    def generate(self, schema, mode="validation"):
        json_schema = super().generate(schema, mode=mode)
        changeValues(json_schema, "$ref", lambda x: x.removesuffix("Context"))
        changeValues(json_schema, "title", lambda x: x.removesuffix("Context"))
        changeKeys(json_schema, lambda x: x.removesuffix("Context"))
        return json_schema


class OutputGenerateJsonSchema(pydantic.json_schema.GenerateJsonSchema):
    def generate(self, schema, mode="validation"):
        json_schema = super().generate(schema, mode=mode)
        changeValues(json_schema, "$ref", lambda x: x.removesuffix("Output"))
        changeValues(json_schema, "title", lambda x: x.removesuffix("Output"))
        changeKeys(json_schema, lambda x: x.removesuffix("Output"))
        return json_schema


class PredictionGenerateJsonSchema(pydantic.json_schema.GenerateJsonSchema):
    def generate(self, schema, mode="validation"):
        json_schema = super().generate(schema, mode=mode)
        changeValues(json_schema, "$ref", lambda x: x.removesuffix("Prediction"))
        changeValues(json_schema, "title", lambda x: x.removesuffix("Prediction"))
        changeKeys(json_schema, lambda x: x.removesuffix("Prediction"))
        return json_schema


def custom_openapi():
    if rest.openapi_schema:
        return rest.openapi_schema
    openapi_schema = fastapi.openapi.utils.get_openapi(title="semio REST API", version=VERSION, summary="This is the local rest API of the semio engine.", routes=rest.routes)

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

# region Engine

engine = starlette.applications.Starlette()
engine.mount("/api", rest)
engine.mount(
    "/graphql",
    starlette_graphene3.GraphQLApp(graphqlSchema, on_get=starlette_graphene3.make_graphiql_handler()),
)


def generateSchemas():
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

    with open("../../jsonschema/kit.json", "w", encoding="utf-8") as f:
        json.dump(
            KitOutput.model_json_schema(schema_generator=OutputGenerateJsonSchema),
            f,
            indent=4,
        )

    with open("../../jsonschema/design-context.json", "w", encoding="utf-8") as f:
        json.dump(
            DesignContext.model_json_schema(schema_generator=ContextGenerateJsonSchema),
            f,
            indent=4,
        )

    with open("../../jsonschema/design.json", "w", encoding="utf-8") as f:
        json.dump(
            DesignOutput.model_json_schema(schema_generator=OutputGenerateJsonSchema),
            f,
            indent=4,
        )

    with open("../../jsonschema/design-prediction.json", "w", encoding="utf-8") as f:
        json.dump(
            DesignPrediction.model_json_schema(schema_generator=PredictionGenerateJsonSchema),
            f,
            indent=4,
        )

    with open("../../jsonschema/type.json", "w", encoding="utf-8") as f:
        json.dump(
            TypeOutput.model_json_schema(schema_generator=OutputGenerateJsonSchema),
            f,
            indent=4,
        )

    with open("../../jsonschema/type-context.json", "w", encoding="utf-8") as f:
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
    # TODO: Make loguru work on extra uvicorn engine process.
    logging.basicConfig(level=logging.INFO)
    uvicorn.run(engine, host=HOST, connector=PORT, log_level="info", access_log=False, log_config=None)


def restart_engine():
    ui_instance = PySide6.QtWidgets.QApplication.instance()
    engine_process = ui_instance.engine_process
    if engine_process.is_alive():
        engine_process.terminate()
    ui_instance.engine_process = multiprocessing.Process(target=start_engine)
    ui_instance.engine_process.start()


def run():
    logger.debug("Starting engine")
    multiprocessing.freeze_support()

    parser = argparse.ArgumentParser(description="semio ⋅ engine")
    parser.add_argument("-d", "--debug", help="debug mode", action="store_true")

    args = parser.parse_args()
    if args.debug:
        logger.add(sys.stderr, level="INFO")
        logger.add(DEBUG_LOG_FILE, level="DEBUG", rotation="10 MB")

    ui = PySide6.QtWidgets.QApplication(sys.argv)
    ui.setQuitOnLastWindowClosed(False)

    if getattr(sys, "frozen", False):
        basedir = sys._MEIPASS
    else:
        basedir = "../../assets"

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
    """Runs before dev()"""


def dev():
    logger.debug("Starting debugpy for semio engine")
    import debugpy

    debugpy.listen(("0.0.0.0", 5678))
    logger.debug("Waiting for debugger to attach to semio engine")
    debugpy.wait_for_client()
    preDev()
    run()


if __name__ == "__main__":
    run()

# endregion Engine
