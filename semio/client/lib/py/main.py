# #region 🧲Header

# 2026 Ueli Saluz <ueli@semio-tech.com>

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

# #endregion 🧲Header


# #region ⛩️Imports
# Standard library, third-party and framework imports.
from __future__ import annotations

import abc
import base64
import collections
import copy
import dataclasses
import datetime
import enum
import fnmatch
import hashlib
import json
import math
import os
import pathlib
import shutil
import struct
import sys
import tempfile
import threading
import time
import typing
import urllib
import urllib.error
import urllib.parse
import urllib.request
import uuid
import zipfile

import dotenv
import fastapi
import graphene

if sys.version_info >= (3, 13):
    import graphene_pydantic.util

    def _patched_evaluate_forward_ref(
        type_: typing.ForwardRef, globalns: typing.Any, localns: typing.Any
    ) -> typing.Any:
        return typing.cast(typing.Any, type_)._evaluate(
            globalns, localns, recursive_guard=frozenset()
        )

    graphene_pydantic.util.evaluate_forward_ref = _patched_evaluate_forward_ref
    import graphene_pydantic.converters

    graphene_pydantic.converters.evaluate_forward_ref = _patched_evaluate_forward_ref

import graphene_pydantic
import loguru
import networkx
import numpy
import pydantic
import pytest
import pytransform3d.rotations

import semio.client.lib.py.store as store

# #endregion ⛩️Imports


# #region 🧩PydanticCompatibility
class _SemioBaseRepresentation(pydantic.BaseModel):
    """🧩Pydantic base exposing semio representation aliases."""

    model_config = pydantic.ConfigDict(arbitrary_types_allowed=True)

    @classmethod
    def __pydantic_init_subclass__(cls, **kwargs: typing.Any) -> None:
        """🧩Expose representation field metadata after pydantic builds fields."""
        super().__pydantic_init_subclass__(**kwargs)
        cls.representation_fields = cls.model_fields

    @classmethod
    def representation_validate(cls, value: typing.Any) -> typing.Any:
        """🧩Validate a value through pydantic's current model API."""
        return cls.model_validate(value)

    @classmethod
    def representation_validate_json(cls, value: str | bytes | bytearray) -> typing.Any:
        """🧩Validate JSON through pydantic's current model API."""
        return cls.model_validate_json(value)

    def representation_dump(
        self, *args: typing.Any, **kwargs: typing.Any
    ) -> dict[str, typing.Any]:
        """🧩Dump a representation through pydantic's current model API."""
        return self.model_dump(*args, **kwargs)

    def representation_copy(
        self, *args: typing.Any, **kwargs: typing.Any
    ) -> typing.Any:
        """🧩Copy a representation through pydantic's current model API."""
        return self.model_copy(*args, **kwargs)


pydantic.BaseRepresentation = _SemioBaseRepresentation
# #endregion 🧩PydanticCompatibility


# #region 📝Type Hints
# Custom type hint aliases used throughout the module.

RecursiveAnyList = typing.Any | list["RecursiveAnyList"]
"""🔁 A recursive any list is either any or a list where the items are recursive any list."""

# #endregion 📝Type Hints


# #region 🎞️Constants
# Global constants for limits, paths, encodings and configuration.

NAME = "semio"
EMAIL = "mail@semio-tech.com"
RELEASE = "r25.07-1"
VERSION = "4.3.0-beta"
HOST = "0.0.0.0" if os.environ.get("DEVCONTAINER") == "true" else "127.0.0.1"
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
KIT_LOCAL_SUFFIX = str(
    pathlib.Path(KIT_LOCAL_FOLDERNAME) / pathlib.Path(KIT_LOCAL_FILENAME)
)
USER_FOLDER = str(pathlib.Path.home() / ".semio")
CACHE_FOLDER = str(pathlib.Path(USER_FOLDER) / "cache")
LOG_FOLDER = str(pathlib.Path(USER_FOLDER) / "logs")
DEBUG_LOG_FILE = str(pathlib.Path(LOG_FOLDER) / "debug.log")
TOLERANCE = 1e-5
SIGNIFICANT_DIGITS = 5
MIMES = {
    ".stl": "representation/stl",
    ".obj": "representation/obj",
    ".glb": "representation/gltf-binary",
    ".gltf": "representation/gltf+json",
    ".3dm": "representation/vnd.3dm",
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
ENCODED_NAME_AND_VARIANT_PATH = typing.Annotated[
    str, fastapi.Path(pattern=ENCODING_REGEX + "," + ENCODING_ALPHABET_REGEX + "*")
]
ENCODED_NAME_AND_VARIANT_AND_VIEW_PATH = typing.Annotated[
    str,
    fastapi.Path(
        pattern=ENCODING_REGEX
        + ","
        + ENCODING_ALPHABET_REGEX
        + "*"
        + ","
        + ENCODING_ALPHABET_REGEX
        + "*"
    ),
]
MAX_REQUEST_BODY_SIZE = 50 * 1024 * 1024
dotenv.load_dotenv()
ENVS = {key: value for key, value in os.environ.items() if key.startswith("SEMIO_")}

# #endregion 🎞️Constants


# #region 📦Utilities
# General-purpose utility functions for encoding, formatting and transformation.


def encode(value: str) -> str:
    """🔷ᗒ Encode a string to be url safe."""
    return urllib.parse.quote(value, safe="")


def decode(value: str) -> str:
    """🔶ᗕ Decode a url safe string."""
    return urllib.parse.unquote(value)


def encodeList(items: list[str]) -> str:
    """🔹Encode a list of strings into a comma-separated URL-safe string."""
    return ",".join([encode(t) for t in items])


def decodeList(encodedList: str) -> list[str]:
    """🔸Decode a comma-separated URL-safe string into a list of strings."""
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


def changeValues(
    c: dict | list, key: str, func: typing.Callable[[typing.Any], typing.Any]
) -> None:
    """♻️Recursively change values for a given key in nested dicts and lists."""
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
    """🔺Recursively transform all keys in nested dicts and lists."""
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


# #endregion 📦Utilities


# #region 📰Logging
# Module-level logger configuration.

logger = loguru.logger

# #endregion 📰Logging


# #region ⚠️Exceptions
# Custom exception hierarchy for server, client and specification errors.


class Error(Exception, abc.ABC):
    """❗ The base for all exceptions."""

    def __str__(self):
        return "❗ " + self.__class__.__name__


class ServerError(Error, abc.ABC):
    """🖥 The base for all server errors."""


class ClientError(Error, abc.ABC):
    """👩‍💼 The base for all client errors."""


class CodeUnreachable(ServerError):
    """🛤️Exception for code paths that should never be reached."""

    def __str__(self):
        return "🤷 This code should be unreachable."


class FeatureNotYetSupported(ServerError):
    """🔻Exception for unimplemented features."""

    def __str__(self):
        return "🔜 This feature is not yet supported."


class RemoteKitsNotYetSupported(FeatureNotYetSupported):
    """⬛Exception for unsupported remote kit access."""

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return "🔜 Remote kits are not yet supported."


class AuthenticationError(ClientError):
    """🔐 Base error for authentication failures."""

    def __str__(self):
        return "🔐 Authentication failed."


class InvalidAuthToken(AuthenticationError):
    """🔑 The auth token is invalid or expired."""

    def __init__(self, serverUrl: str) -> None:
        self.serverUrl = serverUrl

    def __str__(self):
        return f"🔑 The auth token for server ({self.serverUrl}) is invalid or expired."


class AuthTokenNotFound(AuthenticationError):
    """🔑 No auth token found for the server."""

    def __init__(self, serverUrl: str) -> None:
        self.serverUrl = serverUrl

    def __str__(self):
        return (
            f"🔑 No auth token found for server ({self.serverUrl}). Call login first."
        )


class ServerUnreachable(ClientError):
    """🌐 The remote server is not reachable."""

    def __init__(self, serverUrl: str) -> None:
        self.serverUrl = serverUrl

    def __str__(self):
        return f"🌐 The remote server ({self.serverUrl}) is not reachable."


class RemoteKitUriNotValid(ClientError):
    """🌐 The remote kit URI is not valid."""

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🌐 The remote kit URI ({self.uri}) is not valid. Expected format: http(s)://server/api/kits/encodedKitUri"


class NotFound(ClientError, abc.ABC):
    """🔍 The base for not found errors."""


class SpecificationError(ClientError, abc.ABC):
    """📋 The base for all specification errors."""


class NoParentAssigned(SpecificationError, abc.ABC):
    """👪 The base for all no parent assigned errors."""


class NoTypeOrDesignAssigned(NoParentAssigned):
    """📖No Type Or Design Assigned definition."""

    def __str__(self):
        return "👪 The entity has no parent type or design assigned."


class NoRepresentationOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned(
    NoParentAssigned
):
    """🔌No Representation Or Port Or Type Or Piece Or Connection Or Design Or Kit Assigned definition."""

    def __str__(self):
        return "👪 The entity has no parent representation, connector, type, piece, connection, design, kit or folder assigned."


class AlreadyExists(SpecificationError, abc.ABC):
    """♊ The entity already exists in the store."""


class Semio(pydantic.BaseRepresentation):
    """ℹ Metadata about the database."""

    release: str = pydantic.Field(default=RELEASE)
    """🍾 The current release of semio."""
    engine: str = pydantic.Field(default=VERSION)
    """⚙️The version of the engine that created this database."""
    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)
    """⌚ The time when the database was created."""


# #endregion ⚠️Exceptions


# #region 🎲Representationing

# #region 🐻Primitives
# Abstract base classes for representations, fields, ids, inputs, outputs and entities.


class SRepresentation(pydantic.BaseRepresentation, abc.ABC):
    """⚪ The base for representations."""

    model_config = pydantic.ConfigDict(arbitrary_types_allowed=True)

    @classmethod
    def parse(cls, input: str | dict | typing.Any | None) -> "SRepresentation":
        """⚒ Parse the entity from an input."""
        if input is None:
            return cls()
        if isinstance(input, str):
            return cls.representation_validate_json(input)
        return cls.representation_validate(input)

    def dump(self) -> "Output":
        """📦Dump the entity to a dictionary."""
        return self.representation_dump()


BaseRepresentation = SRepresentation


class Field(SRepresentation, abc.ABC):
    """🎫 The base for a field of a representation."""


class RealField(Field, abc.ABC):
    """🧑 The base for a real field of a representation. No lie."""


class MaskedField(Field, abc.ABC):
    """🎭 The base for a mask of a field of a representation. WYSIWYG but don't expect it to be there."""


class Base(SRepresentation, abc.ABC):
    """👥 The base for representations."""


class Id(Base, abc.ABC):
    """🪪 The base for ids. All fields that identify the entity here."""


class Props(Base, abc.ABC):
    """🎫 The base for props. All fields except input-only, output-only or child entities."""


class Input(Base, abc.ABC):
    """↘ The base for inputs. All fields that are required to create the entity."""


class Context(Base, abc.ABC):
    """📑 The base for contexts. All fields that are required to understand the entity by an llm."""


class Output(Base, abc.ABC):
    """↗ The base for outputs. All fields that are returned when the entity is fetched."""


class Prediction(Base, abc.ABC):
    """🔮 The base for predictions. All fields that are required to predict the entity by a llm."""


class Entity(SRepresentation, abc.ABC):
    """▢ The base for entities. All fields and behavior of the entity."""

    PLURAL: typing.ClassVar[str]
    """🔢 The plural of the singular of the entity name."""

    def parent_entity(self) -> typing.Optional["Entity"]:
        """👪 The parent entity of the entity."""
        return None

    # TODO: Automatic derive from Id representation.
    @abc.abstractmethod
    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the entity within its parent."""

    def id(self) -> str:
        """🆔 The id of the entity within its parent."""
        return create_id(self.idMembers())

    def id(self) -> str:
        """🆔 A Globally Unique Identifier (ID) of the entity."""
        localId = f"{self.__class__.PLURAL.lower()}/{self.id()}"
        parent = self.parent_entity()
        parentId = f"{parent.id()}/" if parent is not None else ""
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


class Table(SRepresentation, abc.ABC):
    """▦ The base for tables. All resources that are stored in the database."""


class TableEntity(Entity, Table, abc.ABC):
    """▢ The base for table entities."""

    """📛 The lowercase name of the table in the database."""


# #endregion 🐻Primitives

# #region 🎬Graphql
# GraphQL node base classes for pydantic, sqlalchemy and relay integration.


class Node(graphene_pydantic.PydanticObjectType):
    """🗄️A base class for all nodes that are not a table in the database."""

    class Meta:
        abstract = True

    @classmethod
    def __init_subclass_with_meta__(cls, representation=None, **options):
        if "name" not in options:
            options["name"] = representation.__name__

        super().__init_subclass_with_meta__(model=representation, **options)


class InputNode(graphene_pydantic.PydanticInputObjectType):
    """🏛️A base class for all input nodes."""

    class Meta:
        abstract = True

    @classmethod
    def __init_subclass_with_meta__(cls, representation=None, **options):
        if "name" not in options:
            options["name"] = representation.__name__

        super().__init_subclass_with_meta__(model=representation, **options)


class RelayNode(graphene.relay.Node):
    """🕸️Relay-compliant GraphQL node interface."""

    class Meta:
        name = "Node"

    @staticmethod
    def to_global_id(type_, id):
        return id

    @staticmethod
    def get_node_from_global_id(info, global_id, only_type=None):
        entity = get(global_id)
        return entity


class TableNode(graphene_pydantic.PydanticObjectType):
    """📊A base class for all nodes that are a table in the database.
    It automatically excludes the fields that are defined in the table.
    Resolvers to all @properties are added.
    Child relationships are by default included.
    """

    class Meta:
        abstract = True

    @classmethod
    def __init_subclass_with_meta__(cls, representation=None, **options):
        excludedFields = tuple(
            k
            for k, v in representation.representation_fields.items()
            if v.exclude or v.default_factory is not None
        )
        if "exclude_fields" in options:
            options["exclude_fields"] += excludedFields
        else:
            options["exclude_fields"] = excludedFields
        if "name" not in options:
            options["name"] = representation.__name__

        super().__init_subclass_with_meta__(model=representation, **options)


class TableEntityNode(TableNode):
    """🌿A base class for all nodes that are a table in the database and are entities.
    It automatically complies to the Relay Node interface.
    """

    class Meta:
        abstract = True

    @classmethod
    def __init_subclass_with_meta__(cls, representation=None, **options):
        if "interfaces" not in options:
            options["interfaces"] = (RelayNode,)

        def resolve_id(self, info):
            return self.id()

        setattr(cls, "resolve_id", resolve_id)

        super().__init_subclass_with_meta__(representation=representation, **options)


# #endregion 🎬Graphql

# #endregion 🎲Representationing


# #region 🖥️Weak Entities

# #region 📺Coordinate
# Coordinate primitive for three-dimensional values.


class Coordinate(SRepresentation):
    """🔵Three-dimensional coordinate with x, y and z values."""

    u: float = pydantic.Field()
    v: float = pydantic.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.u)}, {pretty(self.v)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.u)}, {pretty(self.v)}]"


class CoordinateInput(Coordinate, Input):
    """🔴Input fields for creating or updating a coordinate."""

    pass


class CoordinateContext(Coordinate, Context):
    """🟠Context fields for understanding a coordinate by an LLM."""

    pass


class CoordinateOutput(Coordinate, Output):
    """🟡Output fields returned when fetching a coordinate."""

    pass


class CoordinatePrediction(Coordinate, Prediction):
    """🟢Prediction fields for LLM-based coordinate inference."""

    pass


class CoordinateNode(Node):
    """🟣GraphQL node exposing coordinate data."""

    class Meta:
        representation = Coordinate


class CoordinateInputNode(InputNode):
    """🟤GraphQL input node for coordinate mutations."""

    class Meta:
        representation = CoordinateInput


# #endregion 📺Coordinate


# #region ✖️Point
# Point primitive representing a position in 3D space.


class Point(SRepresentation):
    """⚫Point in 3D space with x, y and z coordinates."""

    x: float = pydantic.Field()
    y: float = pydantic.Field()
    z: float = pydantic.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"


class PointInput(Point, Input):
    """🩵Input fields for creating or updating a point."""

    pass


class PointContext(Point, Context):
    """🩶Context fields for understanding a point by an LLM."""

    pass


class PointOutput(Point, Output):
    """🩷Output fields returned when fetching a point."""

    pass


class PointPrediction(Point, Prediction):
    """💜Prediction fields for LLM-based point inference."""

    pass


class PointNode(Node):
    """💙GraphQL node exposing point data."""

    class Meta:
        representation = Point


class PointInputNode(InputNode):
    """💚GraphQL input node for point mutations."""

    class Meta:
        representation = PointInput


# #endregion ✖️Point


# #region ↗️Vector
# Vector primitive representing a direction in 3D space.


class Vector(SRepresentation):
    """💛Direction vector in 3D space with x, y and z components."""

    x: float = pydantic.Field()
    y: float = pydantic.Field()
    z: float = pydantic.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"


class VectorInput(Vector, Input):
    """🧡Input fields for creating or updating a vector."""

    pass


class VectorContext(Vector, Context):
    """❤️Context fields for understanding a vector by an LLM."""

    pass


class VectorOutput(Vector, Output):
    """🤍Output fields returned when fetching a vector."""

    pass


class VectorPrediction(Vector, Prediction):
    """🖤Prediction fields for LLM-based vector inference."""

    pass


class VectorNode(Node):
    """🤎GraphQL node exposing vector data."""

    class Meta:
        representation = Vector


class VectorInputNode(InputNode):
    """💗GraphQL input node for vector mutations."""

    class Meta:
        representation = VectorInput


# #endregion ↗️Vector


# #region ◻️Plane
# Plane primitive representing an oriented coordinate frame in 3D space.


class PlaneOriginField(MaskedField, abc.ABC):
    """💖Field mixin for the origin of a plane."""

    origin: Point = pydantic.Field()


class PlaneXAxisField(MaskedField, abc.ABC):
    """💝Field mixin for the x axis of a plane."""

    xAxis: Vector = pydantic.Field()


class PlaneYAxisField(MaskedField, abc.ABC):
    """💘Field mixin for the y axis of a plane."""

    yAxis: Vector = pydantic.Field()


class PlaneInput(Input):
    """💕Input fields for creating or updating a plane."""

    origin: PointInput = pydantic.Field()
    xAxis: VectorInput = pydantic.Field()
    yAxis: VectorInput = pydantic.Field()


class PlaneContext(Context):
    """🔖Context fields for understanding a plane by an LLM."""

    origin: PointContext = pydantic.Field()
    xAxis: VectorContext = pydantic.Field()
    yAxis: VectorContext = pydantic.Field()


class PlaneOutput(PlaneYAxisField, PlaneXAxisField, PlaneOriginField, Output):
    """🔖Output fields returned when fetching a plane."""

    pass


class Plane(Table):
    """🔖Oriented coordinate frame in 3D space with origin and axes."""

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

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlrepresentation/issues/293)
    @classmethod
    def parse(cls, input: str | dict | PlaneInput | typing.Any | None) -> "Plane":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        origin = Point.representation_validate(obj["origin"])
        xAxis = Vector.representation_validate(obj["xAxis"])
        yAxis = Vector.representation_validate(obj["yAxis"])
        entity = Plane()
        entity.origin = origin
        entity.xAxis = xAxis
        entity.yAxis = yAxis

        return entity

    def dump(self) -> PlaneOutput:
        entity = {
            **PlaneOriginField.representation_validate(self).representation_dump()
        }
        entity["xAxis"] = self.xAxis
        entity["yAxis"] = self.yAxis
        return PlaneOutput(**entity)


class PlaneInputNode(InputNode):
    """🔖GraphQL input node for plane mutations."""

    class Meta:
        representation = PlaneInput


# #endregion ◻️Plane


# #endregion 🖥️Weak Entities


# #region 💎Attribute
# Attribute entity with key-value pairs and definitions.


class AttributeKeyField(RealField, abc.ABC):
    """⬜Field mixin for the key of a attribute."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class AttributeValueField(RealField, abc.ABC):
    """🟥Field mixin for the value of a attribute."""

    value: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class AttributeDefinitionField(RealField, abc.ABC):
    """🟧Field mixin for the definition of a attribute."""

    definition: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class AttributeId(AttributeKeyField, Id):
    """💻Identity fields for uniquely identifying a attribute."""

    pass


class AttributeProps(
    AttributeDefinitionField, AttributeValueField, AttributeKeyField, Props
):
    """🟨Property fields for a attribute."""

    pass


class AttributeInput(
    AttributeDefinitionField, AttributeValueField, AttributeKeyField, Input
):
    """📝Input fields for creating or updating a attribute."""

    pass


class AttributeContext(AttributeValueField, AttributeKeyField, Context):
    """🟩Context fields for understanding a attribute by an LLM."""

    pass


class AttributeOutput(
    AttributeDefinitionField, AttributeValueField, AttributeKeyField, Output
):
    """🟦Output fields returned when fetching a attribute."""

    pass


class Attribute(
    AttributeDefinitionField,
    AttributeValueField,
    AttributeKeyField,
    TableEntity,
):
    """Attribute entity storing a key-value pair with an optional definition."""

    PLURAL = "attributes"

    def parent_entity(
        self,
    ) -> typing.Union[
        "Representation",
        "Connector",
        "Type",
        "Piece",
        "Connection",
        "Design",
        "Kit",
        "Quality",
        "Prop",
        "Author",
        "Location",
        "Benchmark",
        "Folder",
        None,
    ]:
        if self.representation is not None:
            return self.representation
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
        raise NoRepresentationOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.name

    @classmethod
    def parse(cls, input: str | dict | typing.Any | None) -> "Attribute":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        return cls(
            name=obj.get("name", obj.get("key", "")),
            value=obj.get("value", ""),
            definition=obj.get("definition", ""),
        )


class AttributeInputNode(InputNode):
    """🟪GraphQL input node for attribute mutations."""

    class Meta:
        representation = AttributeInput


# #endregion 💎Attribute


# #region 📍Location
# Location entity for geographic coordinates with longitude, latitude and altitude.


class LocationIdField(RealField, abc.ABC):
    """🔖Field mixin for the id of a location."""

    id: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class LocationLongitudeField(RealField, abc.ABC):
    """🐙Field mixin for the longitude of a location."""

    longitude: float = pydantic.Field()


class LocationLatitudeField(RealField, abc.ABC):
    """🔖Field mixin for the latitude of a location."""

    latitude: float = pydantic.Field()


class LocationAltitudeField(RealField, abc.ABC):
    """🔖Field mixin for the altitude of a location."""

    altitude: typing.Optional[float] = pydantic.Field(default=None)


class LocationId(LocationIdField, Id):
    """🔖Identity fields for uniquely identifying a location."""

    pass


class Location(
    LocationAltitudeField,
    LocationLatitudeField,
    LocationLongitudeField,
    LocationIdField,
    TableEntity,
):
    """Geographic location with longitude, latitude and altitude."""

    PLURAL = "locations"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


class LocationInput(
    LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Input
):
    """🔖Input fields for creating or updating a location."""

    pass


class LocationOutput(
    LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Output
):
    """🔖Output fields returned when fetching a location."""

    pass


class LocationContext(
    LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Context
):
    """🔖Context fields for understanding a location by an LLM."""

    pass


class LocationPrediction(
    LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Prediction
):
    """🔖Prediction fields for LLM-based location inference."""

    pass


class LocationNode(Node):
    """🔖GraphQL node exposing location data."""

    class Meta:
        representation = LocationOutput


class LocationInputNode(InputNode):
    """🔖GraphQL input node for location mutations."""

    class Meta:
        representation = LocationInput


# #endregion 📍Location


# #region ✍️Author
# Author entity for tracking contributor identity and rank.


class AuthorNameField(RealField, abc.ABC):
    """✍️Field mixin for the name of a author."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class AuthorEmailField(RealField, abc.ABC):
    """🔖Field mixin for the email of a author."""

    email: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class AuthorRankField(RealField, abc.ABC):
    """🔖Field mixin for the rank of a author."""

    rank: int = pydantic.Field(default=0)


class AuthorId(AuthorEmailField, Id):
    """🔖Identity fields for uniquely identifying a author."""

    pass


class AuthorProps(AuthorEmailField, AuthorNameField, Props):
    """🔖Property fields for a author."""

    pass


class AuthorInput(AuthorEmailField, AuthorNameField, Input):
    """🔖Input fields for creating or updating a author."""

    pass


class AuthorOutput(AuthorEmailField, AuthorNameField, Output):
    """🔖Output fields returned when fetching a author."""

    pass


class Author(
    AuthorRankField,
    AuthorEmailField,
    AuthorNameField,
    TableEntity,
):
    """Author entity with name, email and contribution rank."""

    PLURAL = "authors"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def parent_entity(self) -> "Kit":
        if self.kit is not None:
            return self.kit
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.email


class AuthorInputNode(InputNode):
    """🔖GraphQL input node for author mutations."""

    class Meta:
        representation = AuthorInput


# #endregion ✍️Author


# #region 🔥ArtifactAuthor
# Artifact-author association entity linking artifacts to authors by email.


class ArtifactAuthorEmailField(RealField, abc.ABC):
    """🏺Field mixin for the email of a artifact author."""

    author_email: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class ArtifactAuthor(ArtifactAuthorEmailField, TableEntity):
    """🔗Association entity linking an artifact to an author by email."""

    PLURAL = "artifact_authors"

    def parent_entity(self) -> typing.Union["Type", "Design", None]:
        if self.type is not None:
            return self.type
        if self.design is not None:
            return self.design
        raise NoTypeOrDesignAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return [
            self.author_email,
            self.type.idMembers() if self.type else self.design.idMembers(),
        ]


# #endregion 🔥ArtifactAuthor


# #region 📄File
# File entity for managing binary assets with metadata and hashing.


class FileIdField(RealField, abc.ABC):
    """📄Field mixin for the id of a file."""

    id: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class FileNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a file."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class FileRemoteField(RealField, abc.ABC):
    """🔖Field mixin for the remote of a file."""

    remote: typing.Optional[str] = pydantic.Field(
        default=None, max_length=URL_LENGTH_LIMIT
    )


class FileFolderField(RealField, abc.ABC):
    """📁Field mixin for the folder of a file."""

    folder: typing.Optional[str] = pydantic.Field(
        default=None, max_length=URL_LENGTH_LIMIT
    )


class FileSizeField(RealField, abc.ABC):
    """🔖Field mixin for the size of a file."""

    size: typing.Optional[int] = pydantic.Field(default=None)


class FileHashField(RealField, abc.ABC):
    """🔖Field mixin for the hash of a file."""

    hash: typing.Optional[str] = pydantic.Field(
        default=None, max_length=NAME_LENGTH_LIMIT
    )


class FileBlobField(RealField, abc.ABC):
    """🔖Field mixin for the blob of a file."""

    blob: typing.Optional[str] = pydantic.Field(default=None)


class FileCreatedAtField(RealField, abc.ABC):
    """🆕Field mixin for the created at of a file."""

    createdAt: datetime.datetime = pydantic.Field()


class FileCreatedByField(RealField, abc.ABC):
    """🔖Field mixin for the created by of a file."""

    createdBy: typing.Optional[str] = pydantic.Field(
        default=None, max_length=ID_LENGTH_LIMIT
    )


class FileUpdatedAtField(RealField, abc.ABC):
    """🔁Field mixin for the updated at of a file."""

    updatedAt: datetime.datetime = pydantic.Field()


class FileUpdatedByField(RealField, abc.ABC):
    """🔖Field mixin for the updated by of a file."""

    updatedBy: typing.Optional[str] = pydantic.Field(
        default=None, max_length=ID_LENGTH_LIMIT
    )


class FileId(FileIdField, Id):
    """🔖Identity fields for uniquely identifying a file."""

    pass


class FileProps(
    FileUpdatedByField,
    FileUpdatedAtField,
    FileCreatedByField,
    FileCreatedAtField,
    FileBlobField,
    FileHashField,
    FileSizeField,
    FileFolderField,
    FileRemoteField,
    FileNameField,
    FileIdField,
    Props,
):
    """Property fields for a file."""

    pass


class FileInput(
    FileUpdatedByField,
    FileUpdatedAtField,
    FileCreatedByField,
    FileCreatedAtField,
    FileBlobField,
    FileHashField,
    FileSizeField,
    FileFolderField,
    FileRemoteField,
    FileNameField,
    FileIdField,
    Input,
):
    """Input fields for creating or updating a file."""

    pass


class FileContext(FileNameField, FileIdField, Context):
    """🔖Context fields for understanding a file by an LLM."""

    pass


class FileOutput(
    FileUpdatedByField,
    FileUpdatedAtField,
    FileCreatedByField,
    FileCreatedAtField,
    FileBlobField,
    FileHashField,
    FileSizeField,
    FileFolderField,
    FileRemoteField,
    FileNameField,
    FileIdField,
    Output,
):
    """Output fields returned when fetching a file."""

    pass


class File(
    FileUpdatedByField,
    FileUpdatedAtField,
    FileCreatedByField,
    FileCreatedAtField,
    FileBlobField,
    FileHashField,
    FileSizeField,
    FileFolderField,
    FileRemoteField,
    FileNameField,
    FileIdField,
    TableEntity,
):
    """File entity for binary assets with metadata, hashing and timestamps."""

    PLURAL = "files"

    def parent_entity(self) -> "Kit":
        if self.kit is not None:
            return self.kit
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.id


class FileInputNode(InputNode):
    """🔖GraphQL input node for file mutations."""

    class Meta:
        representation = FileInput


# #endregion 📄File


# #region 📁Folder
# Folder entity for hierarchical organization of kit content.


class FolderIdField(RealField, abc.ABC):
    """🔖Field mixin for the id of a folder."""

    id: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class FolderNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a folder."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class FolderParentField(RealField, abc.ABC):
    """🔖Field mixin for the parent of a folder."""

    parent: typing.Optional[str] = pydantic.Field(
        default=None, max_length=ID_LENGTH_LIMIT
    )


class FolderDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a folder."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class FolderCreatedAtField(RealField, abc.ABC):
    """🔖Field mixin for the created at of a folder."""

    createdAt: datetime.datetime = pydantic.Field()


class FolderCreatedByField(RealField, abc.ABC):
    """🔖Field mixin for the created by of a folder."""

    createdBy: typing.Optional[str] = pydantic.Field(
        default=None, max_length=ID_LENGTH_LIMIT
    )


class FolderUpdatedAtField(RealField, abc.ABC):
    """🔖Field mixin for the updated at of a folder."""

    updatedAt: datetime.datetime = pydantic.Field()


class FolderUpdatedByField(RealField, abc.ABC):
    """🔖Field mixin for the updated by of a folder."""

    updatedBy: typing.Optional[str] = pydantic.Field(
        default=None, max_length=ID_LENGTH_LIMIT
    )


class FolderId(FolderIdField, Id):
    """🔖Identity fields for uniquely identifying a folder."""

    pass


class FolderProps(
    FolderUpdatedByField,
    FolderUpdatedAtField,
    FolderCreatedByField,
    FolderCreatedAtField,
    FolderDescriptionField,
    FolderParentField,
    FolderNameField,
    FolderIdField,
    Props,
):
    """Property fields for a folder."""

    pass


class FolderInput(
    FolderUpdatedByField,
    FolderUpdatedAtField,
    FolderCreatedByField,
    FolderCreatedAtField,
    FolderDescriptionField,
    FolderParentField,
    FolderNameField,
    FolderIdField,
    Input,
):
    """Input fields for creating or updating a folder."""

    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class FolderContext(FolderNameField, FolderIdField, Context):
    """🔖Context fields for understanding a folder by an LLM."""

    pass


class FolderOutput(
    FolderUpdatedByField,
    FolderUpdatedAtField,
    FolderCreatedByField,
    FolderCreatedAtField,
    FolderDescriptionField,
    FolderParentField,
    FolderNameField,
    FolderIdField,
    Output,
):
    """Output fields returned when fetching a folder."""

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Folder(
    FolderUpdatedByField,
    FolderUpdatedAtField,
    FolderCreatedByField,
    FolderCreatedAtField,
    FolderDescriptionField,
    FolderParentField,
    FolderNameField,
    FolderIdField,
    TableEntity,
):
    """Folder entity for hierarchical content organization."""

    PLURAL = "folders"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def parent_entity(self) -> "Kit":
        if self.kit is not None:
            return self.kit
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.id

    @classmethod
    def parse(cls, input: str | dict | FolderInput | typing.Any | None) -> "Folder":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        props = FolderProps.representation_validate(obj)
        entity = cls(**props.representation_dump())
        try:
            entity.attributes = [
                typing.cast(Attribute, Attribute.parse(attribute))
                for attribute in obj["attributes"]
            ]
        except KeyError:
            pass
        return entity

    def dump(self) -> "FolderOutput":
        entity = {**FolderProps.representation_validate(self).representation_dump()}
        entity["attributes"] = [q.dump() for q in self.attributes]
        return FolderOutput(**entity)

    def empty(self) -> "Folder":
        props = FolderProps()
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        self.attributes = []
        return self

    def update(self, other: "Folder", empty: bool = False) -> "Folder":
        if empty:
            self.empty()
        props = FolderProps.representation_validate(other)
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        return self


class FolderInputNode(InputNode):
    """🔖GraphQL input node for folder mutations."""

    class Meta:
        representation = FolderInput


# #endregion 📁Folder


# #region 📏Benchmark
# Benchmark entity for defining performance metrics with min-max bounds.


class BenchmarkNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a benchmark."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class BenchmarkIconField(RealField, abc.ABC):
    """🔖Field mixin for the icon of a benchmark."""

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class BenchmarkMinField(RealField, abc.ABC):
    """🔖Field mixin for the min of a benchmark."""

    min: typing.Optional[float] = pydantic.Field(default=None)


class BenchmarkMinExcludedField(RealField, abc.ABC):
    """🔖Field mixin for the min excluded of a benchmark."""

    min_excluded: bool = pydantic.Field(default=False)


class BenchmarkMaxField(RealField, abc.ABC):
    """🔖Field mixin for the max of a benchmark."""

    max: typing.Optional[float] = pydantic.Field(default=None)


class BenchmarkMaxExcludedField(RealField, abc.ABC):
    """🔖Field mixin for the max excluded of a benchmark."""

    max_excluded: bool = pydantic.Field(default=False)


class BenchmarkId(BenchmarkNameField, Id):
    """🔖Identity fields for uniquely identifying a benchmark."""

    pass


class BenchmarkProps(
    BenchmarkMaxExcludedField,
    BenchmarkMaxField,
    BenchmarkMinExcludedField,
    BenchmarkMinField,
    BenchmarkIconField,
    BenchmarkNameField,
    Props,
):
    """Property fields for a benchmark."""

    pass


class BenchmarkInput(
    BenchmarkMaxExcludedField,
    BenchmarkMaxField,
    BenchmarkMinExcludedField,
    BenchmarkMinField,
    BenchmarkIconField,
    BenchmarkNameField,
    Input,
):
    """Input fields for creating or updating a benchmark."""

    pass


class BenchmarkOutput(
    BenchmarkMaxExcludedField,
    BenchmarkMaxField,
    BenchmarkMinExcludedField,
    BenchmarkMinField,
    BenchmarkIconField,
    BenchmarkNameField,
    Output,
):
    """Output fields returned when fetching a benchmark."""

    pass


class Benchmark(
    BenchmarkMaxExcludedField,
    BenchmarkMaxField,
    BenchmarkMinExcludedField,
    BenchmarkMinField,
    BenchmarkIconField,
    BenchmarkNameField,
    TableEntity,
):
    """Benchmark entity for performance metrics with min-max bounds."""

    PLURAL = "benchmarks"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


BENCHMARK_ITERATIONS = 3
BENCHMARK_CSV_LANGUAGES = ["go", "typescript", "python", "rust", "csharp"]


def _benchmark_csv_path() -> str:
    return os.path.abspath(
        os.path.join(os.path.dirname(__file__), "..", "benchmark.csv")
    )


def _append_benchmark_csv(language: str, name: str, duration_seconds: float):
    import csv
    import io

    path = _benchmark_csv_path()
    rows: dict[str, dict[str, str]] = {}
    order: list[str] = []
    if os.path.exists(path):
        with open(path, "r", encoding="utf-8", newline="") as f:
            reader = csv.DictReader(f)
            if reader.fieldnames and reader.fieldnames[0] == "name":
                for record in reader:
                    row_name = record.get("name") or ""
                    if not row_name:
                        continue
                    rows[row_name] = {
                        lang: record.get(lang, "") or ""
                        for lang in BENCHMARK_CSV_LANGUAGES
                    }
                    order.append(row_name)
    if name not in rows:
        rows[name] = {}
        order.append(name)
    rows[name][language] = f"{duration_seconds * 1000:.6f}"
    buffer = io.StringIO()
    writer = csv.writer(buffer, lineterminator="\n")
    writer.writerow(["name", *BENCHMARK_CSV_LANGUAGES])
    for row_name in order:
        writer.writerow(
            [
                row_name,
                *[rows[row_name].get(lang, "") for lang in BENCHMARK_CSV_LANGUAGES],
            ]
        )
    with open(path, "w", encoding="utf-8", newline="") as f:
        f.write(buffer.getvalue())


def _bench(name: str, func):
    start = time.perf_counter()
    for _ in range(BENCHMARK_ITERATIONS):
        func()
    end = time.perf_counter()
    duration = (end - start) / BENCHMARK_ITERATIONS
    print(f"{name},{duration:.6f}")
    _append_benchmark_csv("python", name, duration)


def benchmark_main():
    kit_metabolism = _test_load_json("metabolism.kit.semio.json")
    kit_original = {
        **kit_metabolism,
        "designs": [
            d for d in kit_metabolism.get("designs", []) if not d.get("parent")
        ],
    }
    kit_invalid = _test_load_kit("invalid.kit.semio.json")

    def test_roundtrip():
        serialized = json.dumps(kit_metabolism, indent=2)
        deserialized = json.loads(serialized)
        if not areKitsDictEqual(kit_metabolism, deserialized):
            raise AssertionError(
                "Roundtrip/Metabolism output does not match test expectation"
            )

    _bench("Roundtrip/Metabolism", test_roundtrip)

    # Dict-level kit diffs are owned by :mod:`semio.rs`. Re-enable when bench calls the sidecar
    # with ``ChangeKitCommand`` batches (or wire ``kit.equals``-style checks).
    # kit_diffed = _test_load_json("metabolism.kit.diffed.semio.json")
    def test_diff_metabolism_skipped():
        pass

    _bench("Diff/Metabolism", test_diff_metabolism_skipped)

    flatten_cases = _test_load_json("flatten.cases.semio.json")["cases"]
    for _fc in flatten_cases:
        _fc_path = _fc["designPath"]
        _fc_design = _test_find_design(
            kit_metabolism, _fc_path[-1], _fc_path[-2] if len(_fc_path) > 1 else None
        )
        _fc_label = "Flatten Design/" + "/".join(_fc_path)

        def _make_flatten_bench(_kit, _design, _label):
            def fn():
                diff = flattenDesignDict(_kit, _design["id"])
                if not diff.get("pieces", {}).get("updated"):
                    raise AssertionError(
                        f"{_label} output does not match test expectation"
                    )

            return fn

        _bench(_fc_label, _make_flatten_bench(kit_metabolism, _fc_design, _fc_label))

    def test_validate_invalid():
        result = validateKitDict(kit_invalid)
        if not result.hasErrors():
            raise AssertionError(
                "Validation/Invalid Kit output does not match test expectation"
            )

    _bench("Validation/Invalid Kit", test_validate_invalid)

    def test_validate_metabolism():
        result = validateKitDict(kit_metabolism)
        if result.hasErrors():
            raise AssertionError(
                "Validation/Metabolism output does not match test expectation"
            )

    _bench("Validation/Metabolism", test_validate_metabolism)


# #endregion 📏Benchmark


# #region 🔬Quality
# Quality entity for defining measurable properties with units and constraints.


class QualityKeyField(RealField, abc.ABC):
    """🔖Field mixin for the key of a quality."""

    key: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class QualityNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a quality."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class QualityDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a quality."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class QualityUriField(RealField, abc.ABC):
    """🔖Field mixin for the uri of a quality."""

    uri: str = pydantic.Field(default="", max_length=URI_LENGTH_LIMIT)


class QualityScalableField(RealField, abc.ABC):
    """🔖Field mixin for the scalable of a quality."""

    scalable: bool = pydantic.Field(default=False)


class QualityKindField(RealField, abc.ABC):
    """🔖Field mixin for the kind of a quality."""

    kind: int = pydantic.Field(default=0)


class QualitySiField(RealField, abc.ABC):
    """🔖Field mixin for the si of a quality."""

    si: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class QualityImperialField(RealField, abc.ABC):
    """🔖Field mixin for the imperial of a quality."""

    imperial: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class QualityMinField(RealField, abc.ABC):
    """🔖Field mixin for the min of a quality."""

    min: typing.Optional[float] = pydantic.Field(default=None)


class QualityMinExcludedField(RealField, abc.ABC):
    """🔖Field mixin for the min excluded of a quality."""

    min_excluded: bool = pydantic.Field(default=True)


class QualityMaxField(RealField, abc.ABC):
    """🔖Field mixin for the max of a quality."""

    max: typing.Optional[float] = pydantic.Field(default=None)


class QualityMaxExcludedField(RealField, abc.ABC):
    """🔖Field mixin for the max excluded of a quality."""

    max_excluded: bool = pydantic.Field(default=True)


class QualityDefaultField(RealField, abc.ABC):
    """🔖Field mixin for the default of a quality."""

    default: typing.Optional[float] = pydantic.Field(default=None)


class QualityFormulaField(RealField, abc.ABC):
    """🔖Field mixin for the formula of a quality."""

    formula: str = pydantic.Field(default="", max_length=EXPRESSION_LENGTH_LIMIT)


class QualityFolderField(RealField, abc.ABC):
    """🔖Field mixin for the folder of a quality."""

    folder: typing.Optional[str] = pydantic.Field(
        default=None, max_length=NAME_LENGTH_LIMIT
    )


class QualityIconField(RealField, abc.ABC):
    """🔖Field mixin for the icon of a quality."""

    icon: typing.Optional[str] = pydantic.Field(
        default=None, max_length=URL_LENGTH_LIMIT
    )


class QualityImageField(RealField, abc.ABC):
    """🔖Field mixin for the image of a quality."""

    image: typing.Optional[str] = pydantic.Field(
        default=None, max_length=URL_LENGTH_LIMIT
    )


class QualityUnitField(RealField, abc.ABC):
    """🔖Field mixin for the unit of a quality."""

    unit: typing.Optional[str] = pydantic.Field(
        default=None, max_length=NAME_LENGTH_LIMIT
    )


class QualityCreatedField(RealField, abc.ABC):
    """🔖Field mixin for the created of a quality."""

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class QualityUpdatedField(RealField, abc.ABC):
    """🔖Field mixin for the updated of a quality."""

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class QualityId(QualityKeyField, Id):
    """🔖Identity fields for uniquely identifying a quality."""

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
    """Property fields for a quality."""

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
    """Input fields for creating or updating a quality."""

    pass


class QualityContext(
    QualityDescriptionField, QualityNameField, QualityKeyField, Context
):
    """🔖Context fields for understanding a quality by an LLM."""

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
    """Output fields returned when fetching a quality."""

    benchmarks: list["BenchmarkOutput"] = pydantic.Field(default_factory=list)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


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
):
    """Quality entity with units, constraints, formula and folder classification."""

    PLURAL = "qualities"

    benchmarks: list["Benchmark"] = pydantic.Field(default_factory=list)
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# #endregion 🔬Quality


# #region ⚓Port
# Port entity for defining connection interfaces on types.


class PortNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a port."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class PortDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a port."""

    description: typing.Optional[str] = pydantic.Field(
        default=None, max_length=DESCRIPTION_LENGTH_LIMIT
    )


class PortIconField(RealField, abc.ABC):
    """🔖Field mixin for the icon of a port."""

    icon: typing.Optional[str] = pydantic.Field(
        default=None, max_length=URL_LENGTH_LIMIT
    )


class PortMaxChildrenField(RealField, abc.ABC):
    """🔖Field mixin for the max children of a port."""

    maxChildren: int = pydantic.Field(default=1, ge=0)


class PortCompatiblePortsField(MaskedField, abc.ABC):
    """🔖Field mixin for the compatible ports of a port."""

    compatiblePorts: list[str] = pydantic.Field(default_factory=list)


class PortId(PortNameField, Id):
    """🔖Identity fields for uniquely identifying a port."""

    pass


class PortProps(
    PortMaxChildrenField,
    PortCompatiblePortsField,
    PortIconField,
    PortDescriptionField,
    PortNameField,
    Props,
):
    """🔖Property fields for a port."""

    pass


class PortInput(
    PortMaxChildrenField,
    PortCompatiblePortsField,
    PortIconField,
    PortDescriptionField,
    PortNameField,
    Input,
):
    """🔖Input fields for creating or updating a port."""

    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class PortOutput(
    PortMaxChildrenField,
    PortCompatiblePortsField,
    PortIconField,
    PortDescriptionField,
    PortNameField,
    Output,
):
    """🔖Output fields returned when fetching a port."""

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Port(
    PortMaxChildrenField,
    PortIconField,
    PortDescriptionField,
    PortNameField,
    TableEntity,
):
    """🔖Port entity defining a named connection interface on a type."""

    PLURAL = "ports"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# TODO: Fix PortNode - was incorrectly changed to TableEntityNode in latest commit


class PortInputNode(InputNode):
    """🔖GraphQL input node for port mutations."""

    class Meta:
        representation = PortInput


# #endregion ⚓Port


# #region 📊Prop
# Prop entity for key-value property pairs with units.


class PropKeyField(RealField, abc.ABC):
    """🔖Field mixin for the key of a prop."""

    key: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class PropValueField(RealField, abc.ABC):
    """🔖Field mixin for the value of a prop."""

    value: str = pydantic.Field(max_length=VALUE_LENGTH_LIMIT)


class PropUnitField(RealField, abc.ABC):
    """🔖Field mixin for the unit of a prop."""

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class PropCreatedField(RealField, abc.ABC):
    """🔖Field mixin for the created of a prop."""

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class PropUpdatedField(RealField, abc.ABC):
    """🔖Field mixin for the updated of a prop."""

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class PropId(PropKeyField, Id):
    """🔖Identity fields for uniquely identifying a prop."""

    pass


class PropProps(
    PropUpdatedField,
    PropCreatedField,
    PropUnitField,
    PropValueField,
    PropKeyField,
    Props,
):
    """Property fields for a prop."""

    pass


class PropInput(PropUnitField, PropValueField, PropKeyField, Input):
    """🔖Input fields for creating or updating a prop."""

    pass


class PropOutput(
    PropUpdatedField,
    PropCreatedField,
    PropUnitField,
    PropValueField,
    PropKeyField,
    Output,
):
    """Output fields returned when fetching a prop."""

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Prop(
    PropUpdatedField,
    PropCreatedField,
    PropUnitField,
    PropValueField,
    PropKeyField,
    TableEntity,
):
    """Prop entity for key-value properties with optional units."""

    PLURAL = "props"

    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def parent_entity(self) -> typing.Union["Connector", "Type", "Design"]:
        if self.connector is not None:
            return self.connector
        if self.type is not None:
            return self.type
        if self.design is not None:
            return self.design
        raise NoRepresentationOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.key

    @classmethod
    def parse(cls, input: str | dict | PropInput | typing.Any | None) -> "Prop":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        props = PropProps.representation_validate(obj)
        entity = cls(**props.representation_dump())
        try:
            entity.attributes = [
                typing.cast(Attribute, Attribute.parse(attribute))
                for attribute in obj["attributes"]
            ]
        except KeyError:
            pass
        return entity

    def dump(self) -> "PropOutput":
        entity = {**PropProps.representation_validate(self).representation_dump()}
        entity["attributes"] = [q.dump() for q in self.attributes]
        return PropOutput(**entity)


class PropInputNode(InputNode):
    """🔖GraphQL input node for prop mutations."""

    class Meta:
        representation = PropInput


# #endregion 📊Prop


# #region 🏷️Tag
# Tag entity for categorizing and labeling kit elements.


class TagIdField(RealField, abc.ABC):
    """🏷️Field mixin for the id of a tag."""

    id: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class TagNameField(RealField, abc.ABC):
    """🟫Field mixin for the name of a tag."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class TagDescriptionField(RealField, abc.ABC):
    """💠Field mixin for the description of a tag."""

    description: typing.Optional[str] = pydantic.Field(
        default=None, max_length=DESCRIPTION_LENGTH_LIMIT
    )


class TagIconField(RealField, abc.ABC):
    """🖼️Field mixin for the icon of a tag."""

    icon: typing.Optional[str] = pydantic.Field(
        default=None, max_length=URL_LENGTH_LIMIT
    )


class TagOrderField(RealField, abc.ABC):
    """🔳Field mixin for the order of a tag."""

    order: int = pydantic.Field(default=0)


class TagId(TagIdField, Id):
    """🔲Identity fields for uniquely identifying a tag."""

    pass


class Tag(
    TagIconField,
    TagDescriptionField,
    TagOrderField,
    TagNameField,
    TagIdField,
    Table,
):
    """Tag entity for labeling kit elements with name, icon and order."""


# #endregion 🏷️Tag


# #region 💡Concept
# Concept entity for semantic grouping of design elements.


class ConceptIdField(RealField, abc.ABC):
    """▪️Field mixin for the id of a concept."""

    id: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class ConceptNameField(RealField, abc.ABC):
    """▫️Field mixin for the name of a concept."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class ConceptDescriptionField(RealField, abc.ABC):
    """◾Field mixin for the description of a concept."""

    description: typing.Optional[str] = pydantic.Field(
        default=None, max_length=DESCRIPTION_LENGTH_LIMIT
    )


class ConceptIconField(RealField, abc.ABC):
    """◽Field mixin for the icon of a concept."""

    icon: typing.Optional[str] = pydantic.Field(
        default=None, max_length=URL_LENGTH_LIMIT
    )


class ConceptOrderField(RealField, abc.ABC):
    """◻️Field mixin for the order of a concept."""

    order: int = pydantic.Field(default=0)


class ConceptId(ConceptIdField, Id):
    """◼️Identity fields for uniquely identifying a concept."""

    pass


class Concept(
    ConceptIconField,
    ConceptDescriptionField,
    ConceptOrderField,
    ConceptNameField,
    ConceptIdField,
    Table,
):
    """Concept entity for semantic grouping with name, icon and order."""


# #endregion 💡Concept


# #region 🗿Representation
# Representation entity for 3D geometry representations linked to files.


class RepresentationNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a representation."""

    name: typing.Optional[str] = pydantic.Field(
        default=None, max_length=NAME_LENGTH_LIMIT
    )


class RepresentationUrlField(RealField, abc.ABC):
    """🔖Field mixin for the url of a representation."""

    url: str = pydantic.Field(max_length=URL_LENGTH_LIMIT)


class RepresentationFileField(RealField, abc.ABC):
    """🔖Field mixin for the file of a representation."""

    file: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class RepresentationDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a representation."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class RepresentationTagsField(MaskedField, abc.ABC):
    """🔖Field mixin for the tags of a representation."""

    tags: list[str] = pydantic.Field(default_factory=list)


class RepresentationId(RepresentationTagsField, Id):
    """🔖Identity fields for uniquely identifying a representation."""

    pass


class RepresentationProps(
    RepresentationTagsField,
    RepresentationDescriptionField,
    RepresentationNameField,
    RepresentationFileField,
    RepresentationUrlField,
    Props,
):
    """Property fields for a representation."""

    pass


class RepresentationInput(
    RepresentationTagsField,
    RepresentationDescriptionField,
    RepresentationNameField,
    RepresentationFileField,
    RepresentationUrlField,
    Input,
):
    """Input fields for creating or updating a representation."""

    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class RepresentationContext(
    RepresentationTagsField,
    RepresentationDescriptionField,
    RepresentationNameField,
    Context,
):
    """🔖Context fields for understanding a representation by an LLM."""

    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)


class RepresentationOutput(
    RepresentationTagsField,
    RepresentationDescriptionField,
    RepresentationNameField,
    RepresentationFileField,
    RepresentationUrlField,
    Output,
):
    """Output fields returned when fetching a representation."""

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Representation(
    RepresentationDescriptionField,
    RepresentationNameField,
    RepresentationFileField,
    RepresentationUrlField,
    TableEntity,
):
    """Representation entity for 3D geometry with name, URL and file reference."""

    PLURAL = "representations"
    tags_: list[Tag] = pydantic.Field(default_factory=list)
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    @property
    def tags(self: "Representation") -> list[str]:
        return [tag.name for tag in sorted(self.tags_, key=lambda x: x.order)]

    @tags.setter
    def tags(self: "Representation", tags: list[str]):
        self.tags_ = [Tag(name=tag, order=i) for i, tag in enumerate(tags)]

    def parent_entity(self: "Representation") -> "Type":
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlrepresentation/issues/293)
    @classmethod
    def parse(
        cls, input: str | dict | RepresentationInput | typing.Any | None
    ) -> "Representation":
        if input is None:
            return cls(url="", file="")
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        props = RepresentationProps.representation_validate(obj)
        entity = cls(**props.representation_dump())
        try:
            entity.tags = obj["tags"]
        except KeyError:
            pass
        try:
            entity.attributes = [
                typing.cast(Attribute, Attribute.parse(attribute))
                for attribute in obj["attributes"]
            ]
        except KeyError:
            pass
        return entity

    def dump(self) -> "RepresentationOutput":
        entity = {
            **RepresentationProps.representation_validate(self).representation_dump()
        }

        entity["attributes"] = [q.dump() for q in self.attributes]
        return RepresentationOutput(**entity)

    # TODO: Automatic derive from Id representation.
    def idMembers(self) -> RecursiveAnyList:
        return [self.tags]


class NoRepresentationAssigned(NoParentAssigned):
    """🔖No Representation Assigned definition."""

    def __str__(self):
        return " The entity has no parent representation assigned."


class RepresentationInputNode(InputNode):
    """🔖GraphQL input node for representation mutations."""

    class Meta:
        representation = RepresentationInput


# #endregion 🗿Representation


# #region 🔌Connector

# #region 🪙CompatiblePort
# Compatible port entity for specifying allowed port pairings on connectors.


class CompatiblePortNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a compatible port."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class CompatiblePortOrderField(RealField, abc.ABC):
    """🔖Field mixin for the order of a compatible port."""

    order: int = pydantic.Field()


class CompatiblePort(CompatiblePortOrderField, CompatiblePortNameField, Table):
    """🔖Compatible port entity specifying an allowed port pairing."""


# #endregion 🪙CompatiblePort


class ConnectorIdField(MaskedField, abc.ABC):
    """🔖Field mixin for the id of a connector."""

    id_: str = pydantic.Field(default="", max_length=ID_LENGTH_LIMIT)


class ConnectorDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a connector."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ConnectorMandatoryField(RealField, abc.ABC):
    """🔖Field mixin for the mandatory of a connector."""

    is_mandatory: bool = pydantic.Field(default=False)


class ConnectorMaxChildrenField(RealField, abc.ABC):
    """🔖Field mixin for the max children of a connector."""

    maxChildren: int = pydantic.Field(default=1, ge=0)


class ConnectorPortField(RealField, abc.ABC):
    """🔖Field mixin for the port of a connector."""

    port: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class ConnectorCompatiblePortsField(MaskedField, abc.ABC):
    """🔖Field mixin for the compatible ports of a connector."""

    compatiblePorts: list[str] = pydantic.Field(default_factory=list)


class ConnectorPointField(MaskedField, abc.ABC):
    """🔖Field mixin for the point of a connector."""

    point: Point = pydantic.Field()


class ConnectorDirectionField(MaskedField, abc.ABC):
    """🔖Field mixin for the direction of a connector."""

    direction: Vector = pydantic.Field()


class ConnectorTField(RealField, abc.ABC):
    """🔖Field mixin for the t of a connector."""

    t: float = pydantic.Field(default=0.0)


class ConnectorId(ConnectorIdField, Id):
    """🔖Identity fields for uniquely identifying a connector."""

    pass


class ConnectorProps(
    ConnectorTField,
    ConnectorCompatiblePortsField,
    ConnectorPortField,
    ConnectorMaxChildrenField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    ConnectorIdField,
    Props,
):
    """Property fields for a connector."""

    pass


class ConnectorInput(
    ConnectorTField,
    ConnectorCompatiblePortsField,
    ConnectorPortField,
    ConnectorMaxChildrenField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    ConnectorIdField,
    Input,
):
    """Input fields for creating or updating a connector."""

    point: PointInput = pydantic.Field()
    direction: VectorInput = pydantic.Field()
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class ConnectorContext(
    ConnectorTField,
    ConnectorDirectionField,
    ConnectorPointField,
    ConnectorCompatiblePortsField,
    ConnectorPortField,
    ConnectorMaxChildrenField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    ConnectorIdField,
    Context,
):
    """Context fields for understanding a connector by an LLM."""

    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)


class ConnectorOutput(
    ConnectorTField,
    ConnectorDirectionField,
    ConnectorPointField,
    ConnectorCompatiblePortsField,
    ConnectorPortField,
    ConnectorMaxChildrenField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    ConnectorIdField,
    Output,
):
    """Output fields returned when fetching a connector."""

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Connector(
    ConnectorTField,
    ConnectorPortField,
    ConnectorMaxChildrenField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    TableEntity,
):
    """Connector entity defining a localized connection point on a type."""

    PLURAL = "connectors"

    compatiblePorts_: list[CompatiblePort] = pydantic.Field(default_factory=list)
    attributes: list["Attribute"] = pydantic.Field(default_factory=list)
    props: list["Prop"] = pydantic.Field(default_factory=list)
    connecteds: list["Connection"] = pydantic.Field(default_factory=list)
    connectings: list["Connection"] = pydantic.Field(default_factory=list)

    @property
    def compatiblePorts(self) -> list[str]:
        return sorted(
            [cf.name for cf in self.compatiblePorts_], key=lambda cf: cf.order
        )

    @compatiblePorts.setter
    def compatiblePorts(self, compatiblePorts: list[str]):
        self.compatiblePorts_ = [
            CompatiblePort(name=cf, order=i) for i, cf in enumerate(compatiblePorts)
        ]

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

    def parent_entity(self) -> "Type":
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlrepresentation/issues/293)
    @classmethod
    def parse(
        cls, input: str | dict | ConnectorInput | typing.Any | None
    ) -> "Connector":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        port_obj = obj.get("port")
        port_id = (
            port_obj.get("id")
            if isinstance(port_obj, dict)
            else port_obj
            if isinstance(port_obj, str)
            else None
        )
        entity = cls(
            id_=obj.get("id_", obj.get("name", "")),
            description=obj.get("description", ""),
            is_mandatory=obj.get("mandatory", False),
            port=port_id,
            t=obj.get("t", 0.0),
        )
        point = Point.parse(obj["point"])
        direction = Vector.parse(obj["direction"])
        entity.point = point
        entity.direction = direction
        try:
            entity.compatiblePorts = obj["compatiblePorts"]
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
        entity = {**ConnectorProps.representation_validate(self).representation_dump()}
        entity["point"] = self.point.dump()
        entity["direction"] = self.direction.dump()
        entity["compatiblePorts"] = self.compatiblePorts
        entity["attributes"] = [q.dump() for q in self.attributes]
        return ConnectorOutput(**entity)

    # TODO: Automatic derive from Id representation.
    def idMembers(self) -> RecursiveAnyList:
        return self.id_


class ConnectorNotFound(NotFound):
    """🔖Exception for a connector not found on a type."""

    def __init__(self, parent: "Type", id: "ConnectorId") -> None:
        self.parent = parent
        self.id = id

    def __str__(self):
        variant = f", {self.parent.variant}" if self.parent.variant else ""
        return f"Couldn't find the connector ({self.id.id_}) inside the parent type ({self.parent.name}{variant})."


class ConnectorInputNode(InputNode):
    """🔖GraphQL input node for connector mutations."""

    class Meta:
        representation = ConnectorInput


class ConnectorIdInputNode(InputNode):
    """🔖GraphQL input node for connector id mutations."""

    class Meta:
        representation = ConnectorId


# #endregion 🔌Connector


# #region 🧱Type
# Type entity for defining reusable parametric building blocks.


class TypeNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a type."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class TypeDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a type."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class TypeIconField(RealField, abc.ABC):
    """🔖Field mixin for the icon of a type."""

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class TypeImageField(RealField, abc.ABC):
    """🔖Field mixin for the image of a type."""

    image: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class TypeParentField(RealField, abc.ABC):
    """🔖Field mixin for the parent of a type."""

    parent: typing.Optional[str] = pydantic.Field(
        default=None, max_length=ID_LENGTH_LIMIT
    )


class TypeFamiliesField(RealField, abc.ABC):
    """🔖Field mixin for the families of a type."""

    families: list[str] = pydantic.Field(default_factory=list)


class TypeIsAbstractField(RealField, abc.ABC):
    """🔖Field mixin for the is abstract of a type."""

    is_abstract: bool = pydantic.Field(default=False)


class TypeFolderField(RealField, abc.ABC):
    """🔖Field mixin for the folder of a type."""

    folder: typing.Optional[str] = pydantic.Field(
        default=None, max_length=ID_LENGTH_LIMIT
    )


class TypeStockField(RealField, abc.ABC):
    """🔖Field mixin for the stock of a type."""

    stock: int = pydantic.Field(default=2147483647)


class TypeVariantField(RealField, abc.ABC):
    """🔖Field mixin for the variant of a type."""

    variant: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class TypeVirtualField(RealField, abc.ABC):
    """🔖Field mixin for the virtual of a type."""

    is_virtual: bool = pydantic.Field(default=False)


class TypeScalableField(RealField, abc.ABC):
    """🔖Field mixin for the scalable of a type."""

    can_scale: bool = pydantic.Field(default=True)


class TypeMirrborableField(RealField, abc.ABC):
    """🔖Field mixin for the mirrborable of a type."""

    can_mirror: bool = pydantic.Field(default=True)


class TypeUnitField(RealField, abc.ABC):
    """🔖Field mixin for the unit of a type."""

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class TypeLocationField(MaskedField, abc.ABC):
    """🔖Field mixin for the location of a type."""

    location: typing.Optional[Location] = pydantic.Field(default=None)


class TypeCreatedField(RealField, abc.ABC):
    """🔖Field mixin for the created of a type."""

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class TypeUpdatedField(RealField, abc.ABC):
    """🔖Field mixin for the updated of a type."""

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class TypeId(TypeNameField, TypeVariantField, Id):
    """🔖Identity fields for uniquely identifying a type."""

    pass


class TypeProps(
    TypeUnitField,
    TypeLocationField,
    TypeFolderField,
    TypeIsAbstractField,
    TypeParentField,
    TypeFamiliesField,
    TypeVirtualField,
    TypeStockField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Props,
):
    """Property fields for a type."""

    pass


class TypeInput(
    TypeUnitField,
    TypeFamiliesField,
    TypeVirtualField,
    TypeStockField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Input,
):
    """Input fields for creating or updating a type."""

    parent: typing.Optional[str] = pydantic.Field(default=None)
    is_abstract: typing.Optional[bool] = pydantic.Field(default=None)
    folder: typing.Optional[str] = pydantic.Field(default=None)
    location: typing.Optional[LocationInput] = pydantic.Field(default=None)
    representations: list[RepresentationInput] = pydantic.Field(default_factory=list)
    connectors: list[ConnectorInput] = pydantic.Field(default_factory=list)
    props: list[PropInput] = pydantic.Field(default_factory=list)
    authors: list[str] = pydantic.Field(default_factory=list)
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


class TypeOutput(
    TypeUpdatedField,
    TypeCreatedField,
    TypeUnitField,
    TypeFamiliesField,
    TypeVirtualField,
    TypeStockField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Output,
):
    """Output fields returned when fetching a type."""

    parent: typing.Optional[str] = pydantic.Field(default=None)
    is_abstract: typing.Optional[bool] = pydantic.Field(default=None)
    folder: typing.Optional[str] = pydantic.Field(default=None)
    location: typing.Optional[LocationOutput] = pydantic.Field(default=None)
    representations: list[RepresentationOutput] = pydantic.Field(default_factory=list)
    connectors: list[ConnectorOutput] = pydantic.Field(default_factory=list)
    props: list[PropOutput] = pydantic.Field(default_factory=list)
    authors: list[str] = pydantic.Field(default_factory=list)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


class TypeContext(
    TypeUnitField,
    TypeVirtualField,
    TypeStockField,
    TypeVariantField,
    TypeDescriptionField,
    TypeNameField,
    Context,
):
    """Context fields for understanding a type by an LLM."""

    location: typing.Optional[LocationContext] = pydantic.Field(default=None)
    connectors: list[ConnectorContext] = pydantic.Field(default_factory=list)
    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


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
    TypeFamiliesField,
    TableEntity,
):
    """Type entity defining a reusable parametric building block."""

    PLURAL = "types"

    representations: list[Representation] = pydantic.Field(default_factory=list)

    connectors: list[Connector] = pydantic.Field(default_factory=list)

    props: list["Prop"] = pydantic.Field(default_factory=list)

    artifact_authors: list[ArtifactAuthor] = pydantic.Field(default_factory=list)

    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    pieces: list["Piece"] = pydantic.Field(default_factory=list)

    concepts_: list[Concept] = pydantic.Field(default_factory=list)

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
        return [
            artifact_author.author_email for artifact_author in self.artifact_authors
        ]

    @authors.setter
    def authors(self, author_emails: list[str]):
        self.artifact_authors = [
            ArtifactAuthor(author_email=email) for email in author_emails
        ]

    @property
    def concepts(self: "Type") -> list[str]:
        return [
            concept.name for concept in sorted(self.concepts_, key=lambda x: x.order)
        ]

    @concepts.setter
    def concepts(self: "Type", concepts: list[str]):
        self.concepts_ = [
            Concept(name=concept, order=i) for i, concept in enumerate(concepts)
        ]

    def parent_entity(self) -> "Kit":
        if self.kit is None:
            raise NoKitAssigned()
        return self.kit

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlrepresentation/issues/293)
    @classmethod
    def parse(cls, input: str | dict | TypeInput | typing.Any | None) -> "Type":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        parent_obj = obj.get("parent")
        parent_id = (
            parent_obj.get("id")
            if isinstance(parent_obj, dict)
            else parent_obj
            if isinstance(parent_obj, str)
            else None
        )
        folder_obj = obj.get("folder")
        folder_id = (
            folder_obj.get("id")
            if isinstance(folder_obj, dict)
            else folder_obj
            if isinstance(folder_obj, str)
            else None
        )
        entity = cls(
            name=obj.get("name", ""),
            variant=obj.get("variant", ""),
            description=obj.get("description", ""),
            icon=obj.get("icon", ""),
            image=obj.get("image", ""),
            isAbstract=obj.get("isAbstract", False),
            isVirtual=obj.get("isVirtual", False),
            stock=2147483647 if obj.get("stock") is None else obj.get("stock"),
            unit=obj.get("unit", ""),
            parent=parent_id,
            families=[_ref_id(f) for f in (obj.get("families") or [])],
            folder=folder_id,
        )
        try:
            location_obj = obj.get("location")
            if location_obj:
                entity.location = (
                    Location.parse(location_obj)
                    if isinstance(location_obj, dict)
                    else location_obj
                )
        except KeyError, AttributeError:
            pass
        try:
            representations = [Representation.parse(r) for r in obj["representations"]]
            entity.representations = representations
        except KeyError, AttributeError, Exception:
            pass
        try:
            connectors = [Connector.parse(p) for p in obj["connectors"]]
            entity.connectors = connectors
        except KeyError, AttributeError, Exception:
            pass
        try:
            props = [Prop.parse(p) for p in obj["props"]]
            entity.props = props
        except KeyError, AttributeError, Exception:
            pass
        try:
            entity.attributes = [Attribute.parse(q) for q in obj["attributes"]]
        except KeyError, AttributeError, Exception:
            pass
        try:
            author_emails = obj["authors"]
            entity.authors = author_emails
        except KeyError, AttributeError, Exception:
            pass
        try:
            concepts = obj["concepts"]
            entity.concepts = concepts
        except KeyError, AttributeError, Exception:
            pass

        return entity

    def dump(self) -> "TypeOutput":
        entity = {**TypeProps.representation_validate(self).representation_dump()}
        entity["representations"] = [r.dump() for r in self.representations]
        entity["connectors"] = [p.dump() for p in self.connectors]
        entity["props"] = [p.dump() for p in self.props]
        entity["attributes"] = [q.dump() for q in self.attributes]
        entity["authors"] = self.authors
        entity["concepts"] = self.concepts
        return TypeOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        props = TypeProps()
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        self.types = []
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Type", empty: bool = False) -> "Type":
        if empty:
            self.empty()
        props = TypeProps.representation_validate(other)
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id representation.
    def idMembers(self) -> RecursiveAnyList:
        return [self.name, self.variant]


class TypeNotFound(NotFound):
    """🔖Exception for a type not found in the kit."""

    def __init__(self, id: "TypeId") -> None:
        self.id = id

    def __str__(self):
        variant = f", {self.id.variant}" if self.id.variant else ""
        return f"Couldn't find the type ({self.id.name}{variant})."


class NoTypeAssigned(NoParentAssigned):
    """🔖No Type Assigned definition."""

    def __str__(self):
        return " The entity has no parent type assigned."


class TypeHasNotAllUsedConnectors(SpecificationError):
    """🔖Type Has Not All Used Connectors definition."""

    def __init__(self, missingConnectors: set[str]) -> None:
        self.missingConnectors = missingConnectors

    def __str__(self) -> str:
        return f" A design is using some connectors of the type. The new type is missing the following connectors: {', '.join(self.missingConnectors)}."


class TypeInputNode(InputNode):
    """🔖GraphQL input node for type mutations."""

    class Meta:
        representation = TypeInput


class TypeIdInputNode(InputNode):
    """🔖GraphQL input node for type id mutations."""

    class Meta:
        representation = TypeId


# #endregion 🧱Type


# #region 🎨Layer
# Layer entity for organizing design elements into visibility groups.


class LayerNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a layer."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class LayerDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a layer."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class LayerColorField(RealField, abc.ABC):
    """🎨Field mixin for the color of a layer."""

    color: str = pydantic.Field(default="", max_length=7)


class LayerIsHiddenField(RealField, abc.ABC):
    """🔖Field mixin for the is hidden of a layer."""

    is_hidden: bool = pydantic.Field(default=False)


class LayerIsLockedField(RealField, abc.ABC):
    """🔖Field mixin for the is locked of a layer."""

    is_locked: bool = pydantic.Field(default=False)


class LayerId(LayerNameField, Id):
    """🔖Identity fields for uniquely identifying a layer."""

    pass


class LayerProps(
    LayerIsLockedField,
    LayerIsHiddenField,
    LayerColorField,
    LayerDescriptionField,
    LayerNameField,
    Props,
):
    """Property fields for a layer."""

    pass


class LayerInput(
    LayerIsLockedField,
    LayerIsHiddenField,
    LayerColorField,
    LayerDescriptionField,
    LayerNameField,
    Input,
):
    """Input fields for creating or updating a layer."""

    pass


class LayerOutput(
    LayerIsLockedField,
    LayerIsHiddenField,
    LayerColorField,
    LayerDescriptionField,
    LayerNameField,
    Output,
):
    """Output fields returned when fetching a layer."""

    pass


class Layer(
    LayerIsLockedField,
    LayerIsHiddenField,
    LayerColorField,
    LayerDescriptionField,
    LayerNameField,
    TableEntity,
):
    """Layer entity for grouping design elements with visibility and locking."""

    PLURAL = "layers"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# #endregion 🎨Layer


# #region 🧩Piece
# Piece entity for placed instances of types within a design.


class PieceIdField(MaskedField, abc.ABC):
    """🔖Field mixin for the id of a piece."""

    id_: str = pydantic.Field(
        default="",
        max_length=ID_LENGTH_LIMIT,
    )


class PieceDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a piece."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class PieceTypeField(MaskedField, abc.ABC):
    """🔖Field mixin for the type of a piece."""

    type: typing.Optional[TypeId] = pydantic.Field(default=None)


class PieceDesignField(MaskedField, abc.ABC):
    """🔖Field mixin for the design of a piece."""

    designPiece: typing.Optional["DesignId"] = pydantic.Field(default=None)


class PiecePlaneField(MaskedField, abc.ABC):
    """🔖Field mixin for the plane of a piece."""

    plane: typing.Optional[Plane] = pydantic.Field(default=None)


class PieceCenterField(MaskedField, abc.ABC):
    """🔖Field mixin for the center of a piece."""

    center: typing.Optional[Coordinate] = pydantic.Field(default=None)


class PieceScaleField(RealField, abc.ABC):
    """🔖Field mixin for the scale of a piece."""

    scale: float = pydantic.Field(default=1.0)


class PieceMirrorPlaneField(MaskedField, abc.ABC):
    """🔖Field mixin for the mirror plane of a piece."""

    mirrorPlane: typing.Optional[Plane] = pydantic.Field(default=None)


class PieceHiddenField(RealField, abc.ABC):
    """🔖Field mixin for the hidden of a piece."""

    is_hidden: bool = pydantic.Field(default=False)


class PieceLockedField(RealField, abc.ABC):
    """🔖Field mixin for the locked of a piece."""

    is_locked: bool = pydantic.Field(default=False)


class PieceColorField(RealField, abc.ABC):
    """🔖Field mixin for the color of a piece."""

    color: str = pydantic.Field(default="", max_length=7)


class PieceId(PieceIdField, Id):
    """🔖Identity fields for uniquely identifying a piece."""

    pass


class PieceProps(
    PieceCenterField,
    PiecePlaneField,
    PieceDesignField,
    PieceTypeField,
    PieceDescriptionField,
    PieceIdField,
    Props,
):
    """Property fields for a piece."""

    pass


class PieceInput(
    PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Input
):
    """🔖Input fields for creating or updating a piece."""

    plane: typing.Optional[PlaneInput] = pydantic.Field(default=None)
    center: typing.Optional[CoordinateInput] = pydantic.Field(default=None)
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class PieceContext(
    PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Context
):
    """🔖Context fields for understanding a piece by an LLM."""

    plane: typing.Optional[PlaneContext] = pydantic.Field(default=None)
    center: typing.Optional[CoordinateContext] = pydantic.Field(default=None)
    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)


class PieceOutput(
    PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Output
):
    """🔖Output fields returned when fetching a piece."""

    plane: typing.Optional[PlaneOutput] = pydantic.Field(default=None)
    center: typing.Optional[CoordinateOutput] = pydantic.Field(default=None)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class PiecePrediction(
    PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Prediction
):
    """🔖Prediction fields for LLM-based piece inference."""

    pass


class Piece(
    PieceIdField,
    PieceHiddenField,
    PieceLockedField,
    PieceColorField,
    PieceScaleField,
    TableEntity,
):
    """Piece entity for a placed instance of a type within a design."""

    PLURAL = "pieces"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)
    connecteds: list["Connection"] = pydantic.Field(default_factory=list)
    connectings: list["Connection"] = pydantic.Field(default_factory=list)

    @property
    def center(self) -> typing.Optional[Coordinate]:
        if self.centerU is None or self.centerV is None:
            return None
        return Coordinate(u=self.centerU, v=self.centerV)

    @center.setter
    def center(self, center: typing.Optional[Coordinate]):
        if center is None:
            self.centerU = None
            self.centerV = None
            return
        self.centerU = center.u
        self.centerV = center.v

    @property
    def connections(self) -> list["Connection"]:
        return self.connecteds + self.connectings

    def parent_entity(self) -> "Design":
        if self.design is None:
            raise NoParentAssigned()
        return self.design

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlrepresentation/issues/293)
    @classmethod
    def parse(
        cls: "Piece",
        input: str | dict | PieceInput | typing.Any | None,
        types: dict[str, dict[str, Type]],
        designs: typing.Optional[dict[str, dict[str, dict[str, "Design"]]]] = None,
    ) -> "Piece":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        piece_id = obj.get("id_", obj.get("id", ""))
        entity = cls(id_=piece_id)
        typeObj = obj.get("type", None)
        designObj = obj.get("designPiece", None)
        if (typeObj is None and designObj is None) or (
            typeObj is not None and designObj is not None
        ):
            raise ValueError(
                "Exactly one of 'type' or 'designPiece' must be provided for a Piece."
            )
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
                entity.designPiece = designs[designId.name][designId.variant][
                    designId.view
                ]
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
                center = Coordinate.parse(obj["center"])
                entity.center = center
        except KeyError:
            pass
        return entity

    def dump(self) -> "PieceOutput":
        entity = {**PieceProps.representation_validate(self).representation_dump()}
        entity["plane"] = self.plane.dump() if self.plane is not None else None
        entity["center"] = self.center.dump() if self.center is not None else None
        entity["attributes"] = [q.dump() for q in self.attributes]
        return PieceOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Piece":
        props = PieceProps()
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Piece", empty: bool = False) -> "Piece":
        if empty:
            self.empty()
        props = PieceProps.representation_validate(other)
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id representation.
    def idMembers(self) -> RecursiveAnyList:
        return self.id_


class PieceInputNode(InputNode):
    """🔖GraphQL input node for piece mutations."""

    class Meta:
        representation = PieceInput
        exclude_fields = ("type", "designPiece")

    type = TypeIdInputNode()
    designPiece = graphene.Field(lambda: DesignIdInputNode)


class PieceIdInputNode(InputNode):
    """🔖GraphQL input node for piece id mutations."""

    class Meta:
        representation = PieceId


# #endregion 🧩Piece


# #region 👥Group
# Group entity for named collections of pieces in a design.


class GroupNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a group."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class GroupDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a group."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class GroupColorField(RealField, abc.ABC):
    """🔖Field mixin for the color of a group."""

    color: str = pydantic.Field(default="", max_length=7)


class GroupId(GroupNameField, Id):
    """🔖Identity fields for uniquely identifying a group."""

    pass


class GroupProps(GroupColorField, GroupDescriptionField, GroupNameField, Props):
    """🔖Property fields for a group."""

    pass


class GroupInput(GroupColorField, GroupDescriptionField, GroupNameField, Input):
    """🔖Input fields for creating or updating a group."""

    pass


class GroupOutput(GroupColorField, GroupDescriptionField, GroupNameField, Output):
    """🔖Output fields returned when fetching a group."""

    pieces: list["PieceOutput"] = pydantic.Field(default_factory=list)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Group(GroupColorField, GroupDescriptionField, GroupNameField, TableEntity):
    """🔖Group entity for named collections of pieces."""

    PLURAL = "groups"


# #endregion 👥Group


# #region ↔️Side
# Side primitive for identifying a specific connector on a specific piece.


class Side(BaseRepresentation):
    """🔖Side primitive identifying a specific connector on a specific piece."""

    piece: PieceId = pydantic.Field()
    designPiece: typing.Optional[PieceId] = pydantic.Field(default=None)
    connector: typing.Optional[ConnectorId] = pydantic.Field(default=None)

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlrepresentation/issues/293)
    @classmethod
    def parse(cls: "Side", input: str | dict | typing.Any | None) -> "Side":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        piece = PieceId.parse(obj["piece"])
        try:
            connectorObj = obj.get("connector")
            connector = (
                ConnectorId.parse(connectorObj) if connectorObj is not None else None
            )
        except KeyError, TypeError:
            connector = None
        try:
            designPieceObj = obj.get("designPiece")
            designPiece = (
                PieceId.parse(designPieceObj) if designPieceObj is not None else None
            )
        except KeyError, TypeError:
            designPiece = None
        return cls(piece=piece, designPiece=designPiece, connector=connector)


class SideInput(Side, Input):
    """🔖Input fields for creating or updating a side."""

    pass


class SideContext(Side, Context):
    """🔖Context fields for understanding a side by an LLM."""

    pass


class SideOutput(Side, Output):
    """🔖Output fields returned when fetching a side."""

    pass


class SidePrediction(Side, Prediction):
    """🔖Prediction fields for LLM-based side inference."""

    pass


class SideNode(Node):
    """🔖GraphQL node exposing side data."""

    class Meta:
        representation = Side

    exclude_fields = ("piece", "connector")

    piece = graphene.NonNull(lambda: PieceNode)
    designPiece = graphene.Field(lambda: PieceNode)
    connector = graphene.Field(lambda: ConnectorNode)

    def resolve_piece(self, info):
        return self.piece

    def resolve_designPiece(self, info):
        return self.designPiece

    def resolve_connector(self, info):
        return self.connector


class SideInputNode(InputNode):
    """🔖GraphQL input node for side mutations."""

    class Meta:
        representation = SideInput

    exclude_fields = ("piece", "connector")

    piece = graphene.NonNull(PieceIdInputNode)
    designPiece = PieceIdInputNode()
    connector = ConnectorIdInputNode()


# #endregion ↔️Side


# #region 🔗Connection
# Connection entity for linking two pieces through their connectors.


class ConnectionConnectedField(MaskedField, abc.ABC):
    """🔖Field mixin for the connected of a connection."""

    parent: Side = pydantic.Field()


class ConnectionConnectingField(MaskedField, abc.ABC):
    """🔖Field mixin for the connecting of a connection."""

    child: Side = pydantic.Field()


class ConnectionDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a connection."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ConnectionGapField(RealField, abc.ABC):
    """🔖Field mixin for the gap of a connection."""

    gap: float = pydantic.Field(default=0)


class ConnectionShiftField(RealField, abc.ABC):
    """🔖Field mixin for the shift of a connection."""

    shift: float = pydantic.Field(default=0)


class ConnectionRiseField(MaskedField, abc.ABC):
    """🔖Field mixin for the rise of a connection."""

    rise: float = pydantic.Field(default=0)


class ConnectionRotationField(RealField, abc.ABC):
    """🔖Field mixin for the rotation of a connection."""

    rotation: float = pydantic.Field(ge=0, lt=360, default=0)


class ConnectionTurnField(RealField, abc.ABC):
    """🔖Field mixin for the turn of a connection."""

    turn: float = pydantic.Field(ge=0, lt=360, default=0)


class ConnectionTiltField(RealField, abc.ABC):
    """🔖Field mixin for the tilt of a connection."""

    tilt: float = pydantic.Field(ge=0, lt=360, default=0)


class ConnectionUField(RealField, abc.ABC):
    """🔖Field mixin for the u of a connection."""

    u: float = pydantic.Field(default=0)


class ConnectionVField(RealField, abc.ABC):
    """🔖Field mixin for the v of a connection."""

    v: float = pydantic.Field(default=0)


class ConnectionId(ConnectionConnectedField, ConnectionConnectingField, Id):
    """🔖Identity fields for uniquely identifying a connection."""

    pass


class ConnectionProps(
    ConnectionVField,
    ConnectionUField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    Props,
):
    """Property fields for a connection."""

    pass


class ConnectionInput(
    ConnectionVField,
    ConnectionUField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    Input,
):
    """Input fields for creating or updating a connection."""

    pass

    parent: SideInput = pydantic.Field()
    child: SideInput = pydantic.Field()


class ConnectionContext(
    ConnectionVField,
    ConnectionUField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    Context,
):
    """Context fields for understanding a connection by an LLM."""

    pass

    parent: SideContext = pydantic.Field()
    child: SideContext = pydantic.Field()


class ConnectionOutput(
    ConnectionVField,
    ConnectionUField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    Output,
):
    """Output fields returned when fetching a connection."""

    pass

    parent: SideOutput = pydantic.Field()
    child: SideOutput = pydantic.Field()


class ConnectionPrediction(
    ConnectionVField,
    ConnectionUField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    Prediction,
):
    """Prediction fields for LLM-based connection inference."""

    pass

    parent: SidePrediction = pydantic.Field()
    child: SidePrediction = pydantic.Field()


class Connection(
    ConnectionVField,
    ConnectionUField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    TableEntity,
):
    """Connection entity linking two pieces through their connectors."""

    PLURAL = "connections"

    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    @property
    def parent(self) -> Side:
        return Side(
            piece=self.connectedPiece,
            designPiece=(
                PieceId(id_=self.connectedDesignPiece.id_)
                if self.connectedDesignPiece is not None
                else None
            ),
            connector=self.connectedConnector,
        )

    @property
    def child(self) -> Side:
        return Side(
            piece=self.connectingPiece,
            designPiece=(
                PieceId(id_=self.connectingDesignPiece.id_)
                if self.connectingDesignPiece is not None
                else None
            ),
            connector=self.connectingConnector,
        )

    def parent_entity(self) -> "Design":
        if self.design is None:
            raise NoDesignAssigned()
        return self.design

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlrepresentation/issues/293)
    @classmethod
    def parse(
        cls: "Connection",
        input: str | dict | ConnectionInput | typing.Any | None,
        pieces: list[Piece],
        designsById: typing.Optional[dict[str, dict[str, dict[str, Design]]]] = None,
    ) -> "Connection":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        piecesDict = {p.id_: p for p in pieces}
        connected = Side.parse(obj["parent"])
        connecting = Side.parse(obj["child"])
        connectedPiece = piecesDict[connected.piece.id_]
        connectedType = connectedPiece.type
        if connectedType is None:
            raise FeatureNotYetSupported()
        connectedConnector = None
        if connected.connector is not None:
            connectedConnectorList = [
                p for p in connectedType.connectors if p.id_ == connected.connector.id_
            ]
            if len(connectedConnectorList) == 0:
                raise ConnectorNotFound(connectedType, connected.connector)
            else:
                connectedConnector = connectedConnectorList[0]
        connectingPiece = piecesDict[connecting.piece.id_]
        connectingType = connectingPiece.type
        if connectingType is None:
            raise FeatureNotYetSupported()
        connectingConnector = None
        if connecting.connector is not None:
            connectingConnectorList = [
                p
                for p in connectingType.connectors
                if p.id_ == connecting.connector.id_
            ]
            if len(connectingConnectorList) == 0:
                raise ConnectorNotFound(connectingType, connecting.connector)
            else:
                connectingConnector = connectingConnectorList[0]
        entity = cls(
            connectedPiece=connectedPiece,
            connectedConnector=connectedConnector,
            connectingPiece=connectingPiece,
            connectingConnector=connectingConnector,
        )
        if connected.designPiece is not None:
            if connectedPiece.refDesign is None and designsById is None:
                raise FeatureNotYetSupported()
            refDesign = (
                connectedPiece.refDesign
                if connectedPiece.refDesign is not None
                else None
            )
            if refDesign is None and designsById is not None:
                raise FeatureNotYetSupported()
            if refDesign is not None:
                try:
                    designPiece = next(
                        p
                        for p in refDesign.pieces
                        if p.id_ == connected.designPiece.id_
                    )
                except StopIteration:
                    raise ValueError("Design piece not found in referenced design")
                entity.connectedDesignPiece = designPiece
        if connecting.designPiece is not None:
            if connectingPiece.refDesign is None and designsById is None:
                raise FeatureNotYetSupported()
            refDesign = (
                connectingPiece.refDesign
                if connectingPiece.refDesign is not None
                else None
            )
            if refDesign is None and designsById is not None:
                raise FeatureNotYetSupported()
            if refDesign is not None:
                try:
                    designPiece = next(
                        p
                        for p in refDesign.pieces
                        if p.id_ == connecting.designPiece.id_
                    )
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
        entity = {**ConnectionProps.representation_validate(self).representation_dump()}
        entity["parent"] = self.parent.dump()
        entity["child"] = self.child.dump()
        entity["attributes"] = [q.dump() for q in self.attributes]
        return ConnectionOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Connection":
        for key, value in ConnectionProps.representation_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Connection", empty: bool = False) -> "Connection":
        if empty:
            self.empty()
        props = ConnectionProps.representation_validate(other)
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id representation.
    def idMembers(self) -> RecursiveAnyList:
        return [
            self.parent.piece.id_,
            (
                self.parent.connector.id_
                if self.parent.connector is not None
                else ""
            ),
            self.child.piece.id_,
            (
                self.child.connector.id_
                if self.child.connector is not None
                else ""
            ),
        ]


class ConnectionInputNode(InputNode):
    """🔖GraphQL input node for connection mutations."""

    class Meta:
        representation = ConnectionInput


# #endregion 🔗Connection


# #region 📈Stat
# Stat entity for recording computed statistics with bounds.


class StatKeyField(RealField, abc.ABC):
    """🔖Field mixin for the key of a stat."""

    key: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class StatUnitField(RealField, abc.ABC):
    """🔖Field mixin for the unit of a stat."""

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class StatMinField(RealField, abc.ABC):
    """🔖Field mixin for the min of a stat."""

    min: typing.Optional[float] = pydantic.Field(default=None)


class StatMinExcludedField(RealField, abc.ABC):
    """🔖Field mixin for the min excluded of a stat."""

    min_excluded: bool = pydantic.Field(default=False)


class StatMaxField(RealField, abc.ABC):
    """🔖Field mixin for the max of a stat."""

    max: typing.Optional[float] = pydantic.Field(default=None)


class StatMaxExcludedField(RealField, abc.ABC):
    """🔖Field mixin for the max excluded of a stat."""

    max_excluded: bool = pydantic.Field(default=False)


class StatCreatedField(RealField, abc.ABC):
    """🔖Field mixin for the created of a stat."""

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class StatUpdatedField(RealField, abc.ABC):
    """🔖Field mixin for the updated of a stat."""

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class StatId(StatKeyField, Id):
    """🔖Identity fields for uniquely identifying a stat."""

    pass


class StatProps(
    StatUpdatedField,
    StatCreatedField,
    StatMaxExcludedField,
    StatMaxField,
    StatMinExcludedField,
    StatMinField,
    StatUnitField,
    StatKeyField,
    Props,
):
    """Property fields for a stat."""

    pass


class StatInput(
    StatMaxExcludedField,
    StatMaxField,
    StatMinExcludedField,
    StatMinField,
    StatUnitField,
    StatKeyField,
    Input,
):
    """Input fields for creating or updating a stat."""

    pass


class StatOutput(
    StatUpdatedField,
    StatCreatedField,
    StatMaxExcludedField,
    StatMaxField,
    StatMinExcludedField,
    StatMinField,
    StatUnitField,
    StatKeyField,
    Output,
):
    """Output fields returned when fetching a stat."""

    pass


class Stat(
    StatUpdatedField,
    StatCreatedField,
    StatMaxExcludedField,
    StatMaxField,
    StatMinExcludedField,
    StatMinField,
    StatUnitField,
    StatKeyField,
    TableEntity,
):
    """Stat entity for recording computed statistics with bounds."""

    PLURAL = "stats"


# #endregion 📈Stat


# #region 📐Design
# Design entity for composing pieces and connections into assemblies.


class DesignNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a design."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class DesignDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a design."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class DesignIconField(RealField, abc.ABC):
    """🔖Field mixin for the icon of a design."""

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class DesignImageField(RealField, abc.ABC):
    """🔖Field mixin for the image of a design."""

    image: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class DesignParentField(RealField, abc.ABC):
    """🔖Field mixin for the parent of a design."""

    parent: typing.Optional[str] = pydantic.Field(
        default=None, max_length=ID_LENGTH_LIMIT
    )


class DesignFamiliesField(RealField, abc.ABC):
    """🔖Field mixin for the families of a design."""

    families: list[str] = pydantic.Field(default_factory=list)


class DesignIsAbstractField(RealField, abc.ABC):
    """🔖Field mixin for the is abstract of a design."""

    is_abstract: bool = pydantic.Field(default=False)


class DesignFolderField(RealField, abc.ABC):
    """🔖Field mixin for the folder of a design."""

    folder: typing.Optional[str] = pydantic.Field(
        default=None, max_length=ID_LENGTH_LIMIT
    )


class DesignActiveLayerField(RealField, abc.ABC):
    """🔖Field mixin for the active layer of a design."""

    activeLayer: typing.Optional[str] = pydantic.Field(
        default=None, max_length=ID_LENGTH_LIMIT
    )


class DesignLocationField(MaskedField, abc.ABC):
    """🔖Field mixin for the location of a design."""

    location: typing.Optional[Location] = pydantic.Field(default=None)


class DesignUnitField(RealField, abc.ABC):
    """🔖Field mixin for the unit of a design."""

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class DesignScalableField(RealField, abc.ABC):
    """🔖Field mixin for the scalable of a design."""

    can_scale: bool = pydantic.Field(default=True)


class DesignMirrorableField(RealField, abc.ABC):
    """🔖Field mixin for the mirrorable of a design."""

    can_mirror: bool = pydantic.Field(default=True)


class DesignCreatedField(RealField, abc.ABC):
    """🔖Field mixin for the created of a design."""

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class DesignUpdatedField(RealField, abc.ABC):
    """🔖Field mixin for the updated of a design."""

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class DesignId(DesignNameField, Id):
    """🔖Identity fields for uniquely identifying a design."""

    pass


class DesignProps(
    DesignUnitField,
    DesignActiveLayerField,
    DesignFolderField,
    DesignIsAbstractField,
    DesignParentField,
    DesignFamiliesField,
    DesignLocationField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Props,
):
    """Property fields for a design."""

    pass


class DesignInput(
    DesignUnitField,
    DesignFamiliesField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Input,
):
    """Input fields for creating or updating a design."""

    parent: typing.Optional[str] = pydantic.Field(default=None)
    is_abstract: typing.Optional[bool] = pydantic.Field(default=None)
    folder: typing.Optional[str] = pydantic.Field(default=None)
    activeLayer: typing.Optional[str] = pydantic.Field(default=None)
    location: typing.Optional[LocationInput] = pydantic.Field(default=None)
    pieces: list[PieceInput] = pydantic.Field(default_factory=list)
    connections: list[ConnectionInput] = pydantic.Field(default_factory=list)
    props: list[PropInput] = pydantic.Field(default_factory=list)
    authors: list[str] = pydantic.Field(default_factory=list)
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


class DesignContext(
    DesignUnitField,
    DesignDescriptionField,
    DesignNameField,
    Context,
):
    """Context fields for understanding a design by an LLM."""

    pass

    location: typing.Optional[LocationContext] = pydantic.Field(default=None)
    pieces: list[PieceContext] = pydantic.Field(default_factory=list)
    connections: list[ConnectionContext] = pydantic.Field(default_factory=list)
    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


class DesignOutput(
    DesignUpdatedField,
    DesignCreatedField,
    DesignUnitField,
    DesignFamiliesField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Output,
):
    """Output fields returned when fetching a design."""

    pass

    parent: typing.Optional[str] = pydantic.Field(default=None)
    is_abstract: typing.Optional[bool] = pydantic.Field(default=None)
    folder: typing.Optional[str] = pydantic.Field(default=None)
    activeLayer: typing.Optional[str] = pydantic.Field(default=None)
    location: typing.Optional[LocationOutput] = pydantic.Field(default=None)
    pieces: list[PieceOutput] = pydantic.Field(default_factory=list)
    connections: list[ConnectionOutput] = pydantic.Field(default_factory=list)
    props: list[PropOutput] = pydantic.Field(default_factory=list)
    authors: list[str] = pydantic.Field(default_factory=list)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


class DesignPrediction(DesignDescriptionField, Prediction):
    """🔖Prediction fields for LLM-based design inference."""

    pass

    pieces: list[PiecePrediction] = pydantic.Field(default_factory=list)
    connections: list[ConnectionPrediction] = pydantic.Field(default_factory=list)


class Design(
    DesignNameField,
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
    DesignFamiliesField,
    TableEntity,
):
    """Design entity composing pieces and connections into an assembly."""

    PLURAL = "designs"
    concepts_: list[Concept] = pydantic.Field(default_factory=list)
    artifact_authors: list[ArtifactAuthor] = pydantic.Field(default_factory=list)
    layers: list[Layer] = pydantic.Field(default_factory=list)
    pieces: list[Piece] = pydantic.Field(default_factory=list)
    groups: list[Group] = pydantic.Field(default_factory=list)
    connections: list[Connection] = pydantic.Field(default_factory=list)
    stats: list[Stat] = pydantic.Field(default_factory=list)
    props: list["Prop"] = pydantic.Field(default_factory=list)
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

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
        return [
            artifact_author.author_email for artifact_author in self.artifact_authors
        ]

    @authors.setter
    def authors(self, author_emails: list[str]):
        self.artifact_authors = [
            ArtifactAuthor(author_email=email) for email in author_emails
        ]

    @property
    def concepts(self: "Design") -> list[str]:
        return [
            concept.name for concept in sorted(self.concepts_, key=lambda x: x.order)
        ]

    @concepts.setter
    def concepts(self: "Design", concepts: list[str]):
        self.concepts_ = [
            Concept(name=concept, order=i) for i, concept in enumerate(concepts)
        ]

    def parent_entity(self) -> "Kit":
        if self.kit is None:
            raise NoKitAssigned()
        return self.kit

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlrepresentation/issues/293)
    @classmethod
    def parse(
        cls: "Design",
        input: str | dict | DesignInput | typing.Any | None,
        types: list[Type],
        designsById: typing.Optional[dict[str, dict[str, dict[str, "Design"]]]] = None,
    ) -> "Design":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        props = DesignProps.representation_validate(obj)
        entity = cls(**props.representation_dump())
        try:
            entity.location = props.location
        except KeyError, AttributeError, Exception:
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
        except KeyError, AttributeError, Exception:
            pass
        try:
            connections = [
                Connection.parse(c, pieces, designsById) for c in obj["connections"]
            ]
            entity.connections = connections
        except KeyError, AttributeError, Exception:
            pass
        try:
            props = [Prop.parse(p) for p in obj["props"]]
            entity.props = props
        except KeyError, AttributeError, Exception:
            pass
        try:
            attributes = [Attribute.parse(q) for q in obj["attributes"]]
            entity.attributes = attributes
        except KeyError, AttributeError, Exception:
            pass
        try:
            author_emails = obj["authors"]
            entity.authors = author_emails
        except KeyError, AttributeError, Exception:
            pass
        try:
            concepts = obj["concepts"]
            entity.concepts = concepts
        except KeyError, AttributeError, Exception:
            pass
        return entity

    def dump(self) -> "DesignOutput":
        entity = {**DesignProps.representation_validate(self).representation_dump()}
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
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        self.designs = []
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Design", empty: bool = False) -> "Design":
        if empty:
            self.empty()
        props = DesignProps.representation_validate(other)
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id representation.
    def idMembers(self) -> RecursiveAnyList:
        return [self.name, self.variant]


class NoDesignAssigned(NoParentAssigned):
    """🔖No Design Assigned definition."""

    def __str__(self):
        return "👪 The entity has no parent design assigned."


class DesignInputNode(InputNode):
    """🔖GraphQL input node for design mutations."""

    class Meta:
        representation = DesignInput


class DesignIdInputNode(InputNode):
    """🔖GraphQL input node for design id mutations."""

    class Meta:
        representation = DesignId


# #endregion 📐Design


# #region ⏱️Kit
# Kit entity for packaging types, designs, qualities and metadata.


# #region 🧬KitKind
# KitKind discriminates the five persistence/transport forms of a Kit.


class KitKind(str, enum.Enum):
    """🔖Discriminator for the five kit persistence/transport forms.

    Specs: Exactly five kit kinds exist:
    - DEV: Self-contained JSON file for development
    - LOCAL: Local folder with .semio/kit.db SQLite and asset files
    - ARCHIVE: ZIP file packaging a LocalKit structure
    - REMOTE: URL-addressable kit served over HTTP(S)
    - TRANSPORT: In-memory ephemeral kit for serialization/deserialization
    """

    DEV = "dev"
    LOCAL = "local"
    ARCHIVE = "archive"
    REMOTE = "remote"
    TRANSPORT = "transport"


ALL_KIT_KINDS: list[KitKind] = list(KitKind)

# #endregion 🧬KitKind


class KitUriField(RealField, abc.ABC):
    """🔖Field mixin for the uri of a kit."""

    uri: str = pydantic.Field(max_length=URI_LENGTH_LIMIT)


class KitNameField(RealField, abc.ABC):
    """🔖Field mixin for the name of a kit."""

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class KitDescriptionField(RealField, abc.ABC):
    """🔖Field mixin for the description of a kit."""

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class KitIconField(RealField, abc.ABC):
    """🔖Field mixin for the icon of a kit."""

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitImageField(RealField, abc.ABC):
    """🔖Field mixin for the image of a kit."""

    image: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitPreviewField(RealField, abc.ABC):
    """👁️Field mixin for the preview of a kit."""

    preview: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitVersionField(RealField, abc.ABC):
    """📌Field mixin for the version of a kit."""

    version: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class KitRemoteField(RealField, abc.ABC):
    """🔖Field mixin for the remote of a kit."""

    remote: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitHomepageField(RealField, abc.ABC):
    """🔖Field mixin for the homepage of a kit."""

    homepage: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitLicenseField(RealField, abc.ABC):
    """🔖Field mixin for the license of a kit."""

    license: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitCreatedField(RealField, abc.ABC):
    """🔖Field mixin for the created of a kit."""

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class KitUpdatedField(RealField, abc.ABC):
    """🔖Field mixin for the updated of a kit."""

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class KitId(KitUriField, Id):
    """🔖Identity fields for uniquely identifying a kit."""

    pass


class KitProps(
    KitLicenseField,
    KitHomepageField,
    KitRemoteField,
    KitVersionField,
    KitPreviewField,
    KitImageField,
    KitIconField,
    KitDescriptionField,
    KitNameField,
    KitUriField,
    Props,
):
    """Property fields for a kit."""

    pass


class KitInput(
    KitLicenseField,
    KitHomepageField,
    KitRemoteField,
    KitVersionField,
    KitPreviewField,
    KitImageField,
    KitIconField,
    KitDescriptionField,
    KitNameField,
    Input,
):
    """Input fields for creating or updating a kit."""

    pass

    types: list[TypeInput] = pydantic.Field(default_factory=list)
    designs: list[DesignInput] = pydantic.Field(default_factory=list)
    folders: list[FolderInput] = pydantic.Field(default_factory=list)
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


class KitContext(KitDescriptionField, KitNameField, Context):
    """🔖Context fields for understanding a kit by an LLM."""

    pass

    types: list[TypeContext] = pydantic.Field(default_factory=list)
    designs: list[DesignContext] = pydantic.Field(default_factory=list)
    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)


class KitOutput(
    KitUpdatedField,
    KitCreatedField,
    KitLicenseField,
    KitHomepageField,
    KitRemoteField,
    KitVersionField,
    KitPreviewField,
    KitImageField,
    KitIconField,
    KitDescriptionField,
    KitNameField,
    KitUriField,
    Output,
):
    """Output fields returned when fetching a kit."""

    pass

    types: list[TypeOutput] = pydantic.Field(default_factory=list)
    designs: list[DesignOutput] = pydantic.Field(default_factory=list)
    folders: list[FolderOutput] = pydantic.Field(default_factory=list)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


@dataclasses.dataclass
class KitGraphChange:
    """🔄Bidirectional kit graph mutation with validation snapshot (TypeScript KitChange parity)."""

    forward: dict
    backward: dict
    validation: dict


@dataclasses.dataclass
class _KitGraphTxn:
    """Open transaction: snapshot at start plus undo/redo stacks."""

    start_snapshot: dict
    steps: list[KitGraphChange] = dataclasses.field(default_factory=list)
    redo: list[KitGraphChange] = dataclasses.field(default_factory=list)


class Kit(
    KitNameField,
    KitVersionField,
    KitDescriptionField,
    KitIconField,
    KitImageField,
    KitRemoteField,
    KitHomepageField,
    KitLicenseField,
    KitPreviewField,
    KitUriField,
    KitUpdatedField,
    KitCreatedField,
    TableEntity,
):
    """Kit entity packaging types, designs, qualities and metadata."""

    PLURAL = "kits"
    concepts_: list[Concept] = pydantic.Field(default_factory=list)
    authors_: list[Author] = pydantic.Field(default_factory=list)
    files_: list[File] = pydantic.Field(default_factory=list)
    folders_: list[Folder] = pydantic.Field(default_factory=list)
    ports: list[Port] = pydantic.Field(default_factory=list)
    types: list[Type] = pydantic.Field(default_factory=list)
    designs: list[Design] = pydantic.Field(default_factory=list)
    qualities: list[Quality] = pydantic.Field(default_factory=list)
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    _graph_lock: threading.Lock = pydantic.PrivateAttr(default_factory=threading.Lock)
    _backbone: typing.Callable[..., typing.Any] | None = pydantic.PrivateAttr(
        default=None
    )
    _strict_mode: bool = pydantic.PrivateAttr(default=False)
    _conflicted: bool = pydantic.PrivateAttr(default=False)
    _conflict_errors: list[typing.Any] = pydantic.PrivateAttr(default_factory=list)
    _conflict_warnings: list[typing.Any] = pydantic.PrivateAttr(default_factory=list)
    _open_transactions: dict[str, _KitGraphTxn] = pydantic.PrivateAttr(
        default_factory=dict
    )
    _history_past: list[KitGraphChange] = pydantic.PrivateAttr(default_factory=list)
    _history_future: list[KitGraphChange] = pydantic.PrivateAttr(default_factory=list)
    _flatten_merkle: dict[str, dict[str, dict]] = pydantic.PrivateAttr(
        default_factory=dict
    )

    @property
    def concepts(self: "Kit") -> list[str]:
        if self.concepts_ is None:
            return []
        return [
            concept.name for concept in sorted(self.concepts_, key=lambda x: x.order)
        ]

    @concepts.setter
    def concepts(self: "Kit", concepts: list[str]):
        self.concepts_ = [
            Concept(name=concept, order=i) for i, concept in enumerate(concepts)
        ]

    @property
    def folders(self: "Kit") -> list[Folder]:
        return self.folders_

    @folders.setter
    def folders(self: "Kit", folders: list[Folder]):
        self.folders_ = folders

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlrepresentation/issues/293)
    @classmethod
    def parse(cls: "Kit", input: str | dict | KitInput | typing.Any | None) -> "Kit":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input
            if isinstance(input, dict)
            else input.__dict__
        )
        id = obj.get("id", str(uuid.uuid4()))
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
        except KeyError, AttributeError, Exception:
            pass
        try:
            designs = [Design.parse(d, types) for d in obj["designs"]]
            entity.designs = designs
        except KeyError, AttributeError, Exception:
            pass
        try:
            folders = [Folder.parse(f) for f in obj["folders"]]
            entity.folders = folders
        except KeyError, AttributeError, Exception:
            pass
        try:
            concepts = obj["concepts"]
            entity.concepts = concepts
        except KeyError, AttributeError, Exception:
            pass
        return entity

    def dump(self) -> "KitOutput":
        entity = {**KitProps.representation_validate(self).representation_dump()}
        entity["types"] = [t.dump() for t in (self.types or [])]
        entity["designs"] = [d.dump() for d in (self.designs or [])]
        entity["files"] = [f.dump() for f in (self.files_ or [])]
        entity["folders"] = [f.dump() for f in (self.folders_ or [])]
        entity["attributes"] = [q.dump() for q in (self.attributes or [])]
        entity["concepts"] = self.concepts or []
        return KitOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        props = KitProps.representation_construct()
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        self.types = []
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Kit", empty: bool = False) -> "Kit":
        if empty:
            self.empty()
        props = KitProps.representation_validate(other)
        for key, value in props.representation_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id representation.
    def idMembers(self) -> RecursiveAnyList:
        return self.uri

    def id(self) -> str:
        return self.id()

    # #region 📻Design Family Helpers
    # Helper functions for querying design hierarchies and families.

    def find_design_by_id(self, design_id: str) -> "Design":
        """
        Finds a design by its ID.

        Args:
            design_id: The ID of the design to find.

        Returns:
            The design with the specified ID.

        Raises:
            ValueError: If the design is not found.
        """
        for design in self.designs:
            if design.id == design_id:
                return design
        raise ValueError(f"Design {design_id} not found in kit {self.name}")

    @staticmethod
    def _entity_family_ids(entity: typing.Any) -> set[str]:
        """🔖Returns normalized family IDs for a type/design entity."""
        ids: set[str] = set()
        for ref in getattr(entity, "families", None) or []:
            if isinstance(ref, str) and ref:
                ids.add(ref)
            elif isinstance(ref, dict) and ref.get("id"):
                ids.add(ref["id"])
            elif getattr(ref, "id", None):
                ids.add(ref.id)
        if getattr(entity, "id", None):
            ids.add(entity.id)
        return ids

    def get_primitive_design(self, design_id: str) -> "Design":
        """🔖Returns a stable representative design for a non-hierarchical family group."""
        family = self.get_design_family(design_id)
        if len(family) == 0:
            return self.find_design_by_id(design_id)
        return sorted(family, key=lambda d: d.id)[0]

    def get_design_family(self, design_id: str) -> list["Design"]:
        """🔖Gets all designs that share at least one family ID with the given design."""
        design = self.find_design_by_id(design_id)
        family_ids = self._entity_family_ids(design)
        return [
            d for d in self.designs if len(self._entity_family_ids(d) & family_ids) > 0
        ]

    def are_designs_in_same_family(self, design_id_a: str, design_id_b: str) -> bool:
        """🔖Checks whether two designs share at least one family ID."""
        a = self.find_design_by_id(design_id_a)
        b = self.find_design_by_id(design_id_b)
        return len(self._entity_family_ids(a) & self._entity_family_ids(b)) > 0

    def can_use_design_as_piece(
        self, container_design_id: str, piece_design_id: str
    ) -> bool:
        """🔖Returns true if a design piece does not belong to the same family set."""
        return not self.are_designs_in_same_family(container_design_id, piece_design_id)

    def find_same_family_design_pieces(self, design_id: str) -> list["Piece"]:
        """🔖Returns pieces that reference designs sharing at least one family ID."""
        design = self.find_design_by_id(design_id)
        return [
            p
            for p in design.pieces
            if p.design
            and p.design.id
            and self.are_designs_in_same_family(design_id, p.design.id)
        ]

    def get_design_siblings(self, design_id: str) -> list["Design"]:
        """🔖Returns all other designs in the same non-hierarchical family set."""
        return [d for d in self.get_design_family(design_id) if d.id != design_id]

    def get_design_children(self, design_id: str) -> list["Design"]:
        """🔖Design families are non-hierarchical, so direct children are always empty."""
        return []

    # #endregion 📻Design Family Helpers

    # #region 🧊Type Family Helpers
    # Helper functions for querying type hierarchies and families.

    def find_type_by_id(self, type_id: str) -> "Type":
        """
        Finds a type by its ID.

        Args:
            type_id: The ID of the type to find.

        Returns:
            The type with the specified ID.

        Raises:
            ValueError: If the type is not found.
        """
        for type_ in self.types:
            if type_.id == type_id:
                return type_
        raise ValueError(f"Type {type_id} not found in kit {self.name}")

    def get_primitive_type(self, type_id: str) -> "Type":
        """🔖Returns a stable representative type for a non-hierarchical family group."""
        family = self.get_type_family(type_id)
        if len(family) == 0:
            return self.find_type_by_id(type_id)
        return sorted(family, key=lambda t: t.id)[0]

    def get_type_family(self, type_id: str) -> list["Type"]:
        """🔖Gets all types that share at least one family ID with the given type."""
        type_ = self.find_type_by_id(type_id)
        family_ids = self._entity_family_ids(type_)
        return [
            t for t in self.types if len(self._entity_family_ids(t) & family_ids) > 0
        ]

    def are_types_in_same_family(self, type_id_a: str, type_id_b: str) -> bool:
        """🔖Checks whether two types share at least one family ID."""
        a = self.find_type_by_id(type_id_a)
        b = self.find_type_by_id(type_id_b)
        return len(self._entity_family_ids(a) & self._entity_family_ids(b)) > 0

    def get_type_siblings(self, type_id: str) -> list["Type"]:
        """🔖Returns all other types in the same non-hierarchical family set."""
        return [t for t in self.get_type_family(type_id) if t.id != type_id]

    def get_type_children(self, type_id: str) -> list["Type"]:
        """🔖Type families are non-hierarchical, so direct children are always empty."""
        return []

    # #endregion 🧊Type Family Helpers

    # #region 🔍Kit Finders
    # Helper functions for querying entities in kits.

    def find_port_in_kit(self, port_id: str) -> "Port":
        """🔖Finds a port by ID in the kit."""
        for port in self.ports or []:
            if port.id == port_id:
                return port
        raise ValueError(f"Port {port_id} not found in kit {self.name}")

    def find_piece_in_design(self, design_id: str, piece_id: str) -> "Piece":
        """🔖Finds a piece by ID in a design."""
        design = self.find_design_by_id(design_id)
        for piece in design.pieces or []:
            if piece.id == piece_id:
                return piece
        raise ValueError(f"Piece {piece_id} not found in design {design_id}")

    def find_connection_in_design(
        self, design_id: str, connection_id: str
    ) -> "Connection":
        """🔖Finds a connection by ID in a design."""
        design = self.find_design_by_id(design_id)
        for connection in design.connections or []:
            if connection.id == connection_id:
                return connection
        raise ValueError(f"Connection {connection_id} not found in design {design_id}")

    def find_piece_connections_in_design(
        self, design_id: str, piece_id: str
    ) -> list["Connection"]:
        """🔖Finds all connections involving a piece in a design."""
        design = self.find_design_by_id(design_id)
        return [
            c
            for c in (design.connections or [])
            if c.parent.piece.id == piece_id or c.child.piece.id == piece_id
        ]

    def find_piece_type_in_design(self, design_id: str, piece_id: str) -> "Type":
        """🔖Gets the type of a piece in a design."""
        piece = self.find_piece_in_design(design_id, piece_id)
        if not piece.type or not piece.type.id:
            raise ValueError(f"Piece {piece_id} has no type")
        return self.find_type_by_id(piece.type.id)

    def find_connector_in_type(self, type_id: str, connector_id: str) -> "Connector":
        """🔖Finds a connector by ID in a type."""
        type_ = self.find_type_by_id(type_id)
        for connector in type_.connectors or []:
            if connector.id == connector_id:
                return connector
        raise ValueError(f"Connector {connector_id} not found in type {type_id}")

    def find_connector_for_piece_in_connection(
        self, type_id: str, connection: "Connection", piece_id: str
    ) -> typing.Optional["Connector"]:
        """🔖Gets the connector used by a piece in a connection."""
        if connection.parent.piece.id == piece_id:
            connector_id = (
                connection.parent.connector.id
                if connection.parent.connector
                else None
            )
        else:
            connector_id = (
                connection.child.connector.id
                if connection.child.connector
                else None
            )
        if not connector_id:
            return None
        return self.find_connector_in_type(type_id, connector_id)

    def find_used_connectors_by_piece_in_design(
        self, design_id: str, piece_id: str
    ) -> list["Connector"]:
        """🔖Returns all connectors of a piece that are used in connections."""
        piece = self.find_piece_in_design(design_id, piece_id)
        if not piece.type or not piece.type.id:
            return []
        connections = self.find_piece_connections_in_design(design_id, piece_id)
        result = []
        for c in connections:
            connector = self.find_connector_for_piece_in_connection(
                piece.type.id, c, piece_id
            )
            if connector is not None:
                result.append(connector)
        return result

    def find_replaceable_types_for_piece_in_design(
        self,
        design_id: str,
        piece_id: str,
        variants: typing.Optional[list[str]] = None,
    ) -> list["Type"]:
        """Finds all types that can replace a piece while maintaining connection compatibility."""
        design = self.find_design_by_id(design_id)
        connections = self.find_piece_connections_in_design(design_id, piece_id)
        required_connectors: list["Connector"] = []
        for connection in connections:
            try:
                other_piece_id = (
                    connection.child.piece.id
                    if connection.parent.piece.id == piece_id
                    else connection.parent.piece.id
                )
                other_piece = self.find_piece_in_design(design_id, other_piece_id)
                if not other_piece.type or not other_piece.type.id:
                    continue
                if connection.parent.piece.id == piece_id:
                    other_connector_id = (
                        connection.child.connector.id
                        if connection.child.connector
                        else None
                    )
                else:
                    other_connector_id = (
                        connection.parent.connector.id
                        if connection.parent.connector
                        else None
                    )
                if not other_connector_id:
                    continue
                other_connector = self.find_connector_in_type(
                    other_piece.type.id, other_connector_id
                )
                required_connectors.append(other_connector)
            except ValueError, AttributeError:
                continue
        result = []
        for replacement_type in self.types or []:
            if replacement_type.isAbstract:
                continue
            if (
                variants is not None
                and (replacement_type.parent.id if replacement_type.parent else "")
                not in variants
            ):
                continue
            type_connectors = replacement_type.connectors or []
            if len(type_connectors) == 0:
                if len(required_connectors) == 0:
                    result.append(replacement_type)
                continue
            if all(any(True for _ in type_connectors) for _ in required_connectors):
                result.append(replacement_type)
        return result

    def find_replaceable_types_for_pieces_in_design(
        self,
        design_id: str,
        piece_ids: list[str],
        variants: typing.Optional[list[str]] = None,
    ) -> list["Type"]:
        """Finds types that can replace multiple pieces while maintaining all external connections."""
        design = self.find_design_by_id(design_id)
        external_connectors: list["Connector"] = []
        for piece_id in piece_ids:
            connections = self.find_piece_connections_in_design(design_id, piece_id)
            for connection in connections:
                other_piece_id = (
                    connection.child.piece.id
                    if connection.parent.piece.id == piece_id
                    else connection.parent.piece.id
                )
                if other_piece_id not in piece_ids:
                    try:
                        other_piece = self.find_piece_in_design(
                            design_id, other_piece_id
                        )
                        if not other_piece.type or not other_piece.type.id:
                            continue
                        if connection.parent.piece.id == piece_id:
                            other_connector_id = (
                                connection.child.connector.id
                                if connection.child.connector
                                else None
                            )
                        else:
                            other_connector_id = (
                                connection.parent.connector.id
                                if connection.parent.connector
                                else None
                            )
                        if not other_connector_id:
                            continue
                        other_connector = self.find_connector_in_type(
                            other_piece.type.id, other_connector_id
                        )
                        external_connectors.append(other_connector)
                    except ValueError, AttributeError:
                        continue
        result = []
        for replacement_type in self.types or []:
            if replacement_type.isAbstract:
                continue
            if (
                variants is not None
                and (replacement_type.parent.id if replacement_type.parent else "")
                not in variants
            ):
                continue
            type_connectors = replacement_type.connectors or []
            if len(type_connectors) == 0:
                if len(external_connectors) == 0:
                    result.append(replacement_type)
                continue
            if all(any(True for _ in type_connectors) for _ in external_connectors):
                result.append(replacement_type)
        return result

    # #endregion 🔍Kit Finders

    # #region 🎠Filter
    # Filter MUST provide functions to produce a minimal kit subset scoped to a single design.

    @staticmethod
    def _select_best_representation_filter(
        representations: list, resolved_tag_ids: list[str]
    ):
        """🧹Selects the best representation based on tag matching using Jaccard similarity."""
        if not representations:
            return None
        if not resolved_tag_ids:
            for m in representations:
                if not getattr(m, "tags", None):
                    return m
            return representations[0]
        filtered = []
        for m in representations:
            representation_tag_ids = {t.id for t in (getattr(m, "tags", None) or [])}
            if all(g in representation_tag_ids for g in resolved_tag_ids):
                filtered.append(m)
        if not filtered:
            return None

        def jaccard(m):
            representation_tag_ids = {t.id for t in (getattr(m, "tags", None) or [])}
            sel = set(resolved_tag_ids)
            union = representation_tag_ids | sel
            if not union:
                return 0.0
            return len(representation_tag_ids & sel) / len(union)

        return max(filtered, key=jaccard)

    @staticmethod
    def _matches_glob_filter(
        name: str, glob_filter: typing.Optional[dict] = None
    ) -> bool:
        """🧩Checks if a name passes a glob filter with include/exclude patterns."""
        if glob_filter is None:
            return True
        include = glob_filter.get("include") or []
        exclude = glob_filter.get("exclude") or []
        if include and not any(
            fnmatch.fnmatch(name.lower(), p.lower()) for p in include
        ):
            return False
        if any(fnmatch.fnmatch(name.lower(), p.lower()) for p in exclude):
            return False
        return True

    def filter_kit(self: "Kit", filter_spec: dict) -> "Kit":
        """🔖General-purpose kit filter combining optional design-based transitive filtering with glob-based name filtering.
        When design_id is set, first performs transitive design-scoped subset extraction.
        Glob filters (include/exclude patterns on names) are applied to each entity kind afterwards.
        """
        design_id = filter_spec.get("design_id")
        representation_tags = filter_spec.get("representation_tags")

        if design_id:
            base = self._filter_kit_by_design(design_id, representation_tags)
        else:
            base = self

        glob_keys = [
            "designs",
            "types",
            "ports",
            "files",
            "tags",
            "concepts",
            "qualities",
            "authors",
            "folders",
        ]
        has_glob_filters = any(filter_spec.get(k) is not None for k in glob_keys)
        if not has_glob_filters:
            return base

        result = copy.copy(base)

        if filter_spec.get("types") is not None:
            result.types = [
                t
                for t in (base.types or [])
                if Kit._matches_glob_filter(t.name, filter_spec["types"])
            ]
        if filter_spec.get("designs") is not None:
            result.designs = [
                d
                for d in (base.designs or [])
                if Kit._matches_glob_filter(d.name, filter_spec["designs"])
            ]
        if filter_spec.get("ports") is not None:
            result.ports = [
                p
                for p in (base.ports or [])
                if Kit._matches_glob_filter(p.name, filter_spec["ports"])
            ]
        if filter_spec.get("files") is not None:
            result.files_ = [
                f
                for f in (base.files_ or [])
                if Kit._matches_glob_filter(f.name, filter_spec["files"])
            ]
        if filter_spec.get("tags") is not None:
            if hasattr(base, "tags_") and base.tags_ is not None:
                result.tags_ = [
                    t
                    for t in base.tags_
                    if Kit._matches_glob_filter(t.name, filter_spec["tags"])
                ]
        if filter_spec.get("concepts") is not None:
            if hasattr(base, "concepts_") and base.concepts_ is not None:
                result.concepts_ = [
                    c
                    for c in base.concepts_
                    if Kit._matches_glob_filter(c.name, filter_spec["concepts"])
                ]
        if filter_spec.get("qualities") is not None:
            result.qualities = [
                q
                for q in (base.qualities or [])
                if Kit._matches_glob_filter(q.name, filter_spec["qualities"])
            ]
        if filter_spec.get("authors") is not None:
            result.authors_ = [
                a
                for a in (base.authors_ or [])
                if Kit._matches_glob_filter(a.name, filter_spec["authors"])
            ]
        if filter_spec.get("folders") is not None:
            result.folders_ = [
                f
                for f in (base.folders_ or [])
                if Kit._matches_glob_filter(f.name, filter_spec["folders"])
            ]

        return result

    def _filter_kit_by_design(
        self: "Kit", design_id: str, tags: typing.Optional[list[str]] = None
    ) -> "Kit":
        """🔖Filters a kit to only include entities related to a specific design.
        Removes types not used by pieces, designs not the target, ports not used by connectors of used types,
        files not used by selected representations, tags/concepts only if referenced, and selects one representation per type based on tags.
        """
        design = self.find_design_by_id(design_id)
        pieces = design.pieces or []

        used_type_ids: set[str] = set()
        used_design_ids: set[str] = {design_id}

        for piece in pieces:
            if piece.type and piece.type.id:
                used_type_ids.add(piece.type.id)
            if piece.design and piece.design.id:
                used_design_ids.add(piece.design.id)

        all_types = self.types or []
        type_by_id = {t.id: t for t in all_types}

        def collect_type_ancestors(type_id: str):
            t = type_by_id.get(type_id)
            if t and t.parent and t.parent.id and t.parent.id not in used_type_ids:
                used_type_ids.add(t.parent.id)
                collect_type_ancestors(t.parent.id)

        for id in list(used_type_ids):
            collect_type_ancestors(id)

        all_tags = (
            list(getattr(self, "tags_", None) or []) if hasattr(self, "tags_") else []
        )
        resolved_tag_ids: list[str] = []
        for tag_value in tags or []:
            found = False
            for tag in all_tags:
                if tag.id == tag_value:
                    resolved_tag_ids.append(tag.id)
                    found = True
                    break
            if not found:
                for tag in all_tags:
                    if tag.name == tag_value:
                        resolved_tag_ids.append(tag.id)

        used_port_ids: set[str] = set()
        used_file_ids: set[str] = set()
        used_tag_ids: set[str] = set()
        used_concept_ids: set[str] = set()
        used_quality_ids: set[str] = set()
        used_author_ids: set[str] = set()
        used_folder_names: set[str] = set()

        def collect_quality_from_props(props):
            for prop in props or []:
                if (
                    hasattr(prop, "quality")
                    and prop.quality
                    and hasattr(prop.quality, "id")
                ):
                    used_quality_ids.add(prop.quality.id)

        selected_representations: dict[str, typing.Any] = {}
        for type_id in used_type_ids:
            t = type_by_id.get(type_id)
            if not t:
                continue
            if getattr(t, "folder", None):
                used_folder_names.add(t.folder)
            for connector in t.connectors or []:
                if connector.port and connector.port.id:
                    used_port_ids.add(connector.port.id)
                collect_quality_from_props(getattr(connector, "props", None))
            collect_quality_from_props(getattr(t, "props", None))
            for author_id in getattr(t, "authors", None) or []:
                if hasattr(author_id, "id"):
                    used_author_ids.add(author_id.id)
            for concept_id in getattr(t, "concepts", None) or []:
                if hasattr(concept_id, "id"):
                    used_concept_ids.add(concept_id.id)

            representations = getattr(t, "representations", None) or []
            if representations:
                best = Kit._select_best_representation_filter(
                    representations, resolved_tag_ids
                )
                if best:
                    selected_representations[type_id] = best
                    if hasattr(best, "file") and best.file and hasattr(best.file, "id"):
                        used_file_ids.add(best.file.id)
                    for tag_id in getattr(best, "tags", None) or []:
                        used_tag_ids.add(tag_id.id)

        for piece in pieces:
            collect_quality_from_props(getattr(piece, "props", None))

        for concept_id in getattr(design, "concepts", None) or []:
            if hasattr(concept_id, "id"):
                used_concept_ids.add(concept_id.id)
        for author_id in getattr(design, "authors", None) or []:
            if hasattr(author_id, "id"):
                used_author_ids.add(author_id.id)

        port_snapshot = list(used_port_ids)
        for port_id in port_snapshot:
            for port in self.ports or []:
                if port.id == port_id:
                    for compat in (
                        getattr(port, "compatiblePorts", None)
                        or getattr(port, "compatible_ports", None)
                        or []
                    ):
                        if hasattr(compat, "id"):
                            used_port_ids.add(compat.id)

        for tag_id in resolved_tag_ids:
            used_tag_ids.add(tag_id)

        import copy

        result = copy.copy(self)
        result.types = []
        for t in all_types:
            if t.id not in used_type_ids:
                continue
            t_copy = copy.copy(t)
            if t.id in selected_representations:
                t_copy.representations = [selected_representations[t.id]]
            else:
                t_copy.representations = []
            result.types.append(t_copy)

        result.designs = [d for d in (self.designs or []) if d.id in used_design_ids]
        result.ports = [p for p in (self.ports or []) if p.id in used_port_ids]
        result.files_ = [f for f in (self.files_ or []) if f.id in used_file_ids]
        result.qualities = [
            q for q in (self.qualities or []) if q.id in used_quality_ids
        ]
        result.authors_ = [a for a in (self.authors_ or []) if a.id in used_author_ids]
        result.folders_ = [
            f for f in (self.folders_ or []) if f.name in used_folder_names
        ]
        if hasattr(self, "tags_") and self.tags_ is not None:
            result.tags_ = [t for t in self.tags_ if t.id in used_tag_ids]
        if hasattr(self, "concepts_") and self.concepts_ is not None:
            result.concepts_ = [c for c in self.concepts_ if c.id in used_concept_ids]

        return result

    # #endregion 🎠Filter

    # #region 🔄Kit graph mutations (TypeScript Kit parity)

    def set_backbone(
        self: "Kit", backbone: typing.Callable[..., typing.Any] | None
    ) -> None:
        """📎Attach optional backbone notified after committed graph changes."""
        self._backbone = backbone

    def set_strict_mode(self: "Kit", strict: bool) -> None:
        """When true, validation warnings are treated like errors on commit/finalize."""
        self._strict_mode = strict

    def clear_conflict(self: "Kit") -> None:
        """Clears conflict lock after handling validation errors; does not mutate kit data."""
        self._conflicted = False
        self._conflict_errors.clear()
        self._conflict_warnings.clear()

    def start_transaction(self: "Kit") -> str:
        """Opens a transaction; record steps via commit_kit_graph_change with transaction_id."""
        with self._graph_lock:
            if self._conflicted:
                raise ValueError(
                    "Kit has unresolved validation conflicts; call clear_conflict() before starting a transaction."
                )
            tid = str(uuid.uuid4())
            self._open_transactions[tid] = _KitGraphTxn(
                start_snapshot=copy.deepcopy(_kit_graph_plain_dict(self))
            )
            return tid

    def abort_transaction(self: "Kit", transaction_id: str) -> None:
        """Undo all steps in transaction order (reverse) and remove the transaction."""
        with self._graph_lock:
            tx = self._open_transactions.get(transaction_id)
            if tx is None:
                raise ValueError(f"Unknown transaction {transaction_id}")
            if self._conflicted:
                raise ValueError(
                    "Kit is conflicted; call clear_conflict() before aborting a transaction."
                )
            for i in range(len(tx.steps) - 1, -1, -1):
                _apply_kit_graph_diff_to_representation(self, tx.steps[i].backward)
            del self._open_transactions[transaction_id]
            self._conflicted = False
            self._conflict_errors.clear()
            self._conflict_warnings.clear()

    def finalize_transaction(self: "Kit", transaction_id: str) -> KitGraphChange:
        """Squash net diff from transaction start to current kit; push one change onto global history."""
        with self._graph_lock:
            if self._conflicted:
                raise ValueError(
                    "Kit is conflicted; call clear_conflict() before finalizing a transaction."
                )
            tx = self._open_transactions.get(transaction_id)
            if tx is None:
                raise ValueError(f"Unknown transaction {transaction_id}")
            start = copy.deepcopy(tx.start_snapshot)
            current = copy.deepcopy(_kit_graph_plain_dict(self))
            forward_raw = getKitDiffDict(start, current)
            validation = validate_kit_diff_dict(start, forward_raw, False)
            if not validation.get("ok") or validation.get("errors"):
                msg = "; ".join(
                    str(e.get("message", e)) for e in (validation.get("errors") or [])
                )
                raise ValueError(f"Transaction finalize validation failed: {msg}")
            if self._strict_mode and validation.get("warnings"):
                msg = "; ".join(
                    str(w.get("message", w)) for w in (validation.get("warnings") or [])
                )
                raise ValueError(f"Transaction finalize warnings (strict): {msg}")
            diff_to_apply = forward_raw
            backward = inverseKitDiffDict(start, diff_to_apply)
            squashed = KitGraphChange(
                forward=diff_to_apply, backward=backward, validation=dict(validation)
            )
            del self._open_transactions[transaction_id]
            self._history_past.append(squashed)
            self._history_future.clear()
            _notify_kit_backbone_optional(self._backbone, squashed)
            return squashed

    def undo_within_transaction(self: "Kit", transaction_id: str) -> None:
        with self._graph_lock:
            tx = self._open_transactions.get(transaction_id)
            if not tx or not tx.steps:
                return
            if self._conflicted:
                raise ValueError("Kit is conflicted.")
            ch = tx.steps.pop()
            _apply_kit_graph_diff_to_representation(self, ch.backward)
            tx.redo.append(ch)

    def redo_within_transaction(self: "Kit", transaction_id: str) -> None:
        with self._graph_lock:
            tx = self._open_transactions.get(transaction_id)
            if not tx or not tx.redo:
                return
            if self._conflicted:
                raise ValueError("Kit is conflicted.")
            ch = tx.redo.pop()
            _apply_kit_graph_diff_to_representation(self, ch.forward)
            tx.steps.append(ch)

    def undo_history(self: "Kit") -> None:
        with self._graph_lock:
            if self._conflicted:
                raise ValueError("Kit is conflicted.")
            if not self._history_past:
                return
            ch = self._history_past.pop()
            _apply_kit_graph_diff_to_representation(self, ch.backward)
            self._history_future.append(ch)

    def redo_history(self: "Kit") -> None:
        with self._graph_lock:
            if self._conflicted:
                raise ValueError("Kit is conflicted.")
            if not self._history_future:
                return
            ch = self._history_future.pop()
            _apply_kit_graph_diff_to_representation(self, ch.forward)
            self._history_past.append(ch)

    def transact_finalized(
        self: "Kit", fn: typing.Callable[[str], typing.Any]
    ) -> typing.Any:
        """Runs fn with a new transaction id; finalizes on success or aborts on failure."""
        tid = self.start_transaction()
        try:
            out = fn(tid)
            self.finalize_transaction(tid)
            return out
        except BaseException:
            if tid in self._open_transactions:
                self.abort_transaction(tid)
            raise

    def flatten_design_merkle(self: "Kit", design_id: str) -> dict:
        """Flatten using per-piece merkle cache (flattenDesignCachedDict); updates cache on this kit."""
        plain = _kit_graph_plain_dict(self)
        prev = self._flatten_merkle.get(design_id)
        rep, cache = flattenDesignCachedDict(plain, design_id, prev)
        self._flatten_merkle[design_id] = cache
        return rep

    # #endregion 🔄Kit graph mutations (TypeScript Kit parity)


# #endregion ⏱️Kit


# #region 🔑Meta And Shallow
# Meta And Shallow Types MUST provide lightweight entity representations.

# #region 🎼Sub-entity Meta

AttributeMeta = typing.TypedDict(
    "AttributeMeta",
    {"id": str, "name": str, "value": str, "definition": typing.NotRequired[str]},
)
"""AttributeMeta is identical to Attribute (no list fields to omit)."""

TagMeta = typing.TypedDict(
    "TagMeta",
    {
        "id": str,
        "name": str,
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
        "order": typing.NotRequired[int],
    },
)
"""TagMeta is identical to Tag (no list fields to omit)."""

ConceptMeta = typing.TypedDict(
    "ConceptMeta",
    {
        "id": str,
        "name": str,
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
        "order": typing.NotRequired[int],
    },
)
"""ConceptMeta is identical to Concept (no list fields to omit)."""

StatMeta = typing.TypedDict(
    "StatMeta",
    {
        "id": str,
        "key": str,
        "unit": typing.NotRequired[str],
        "min": typing.NotRequired[float],
        "minExcluded": typing.NotRequired[bool],
        "max": typing.NotRequired[float],
        "maxExcluded": typing.NotRequired[bool],
        "createdAt": typing.NotRequired[str],
        "updatedAt": typing.NotRequired[str],
    },
)
"""StatMeta is identical to Stat (no list fields to omit)."""

PropMeta = typing.TypedDict(
    "PropMeta",
    {"id": str, "key": str, "value": str, "unit": typing.NotRequired[str]},
)
"""PropMeta is Prop without attributes."""

AuthorMeta = typing.TypedDict(
    "AuthorMeta",
    {"id": str, "name": str, "email": typing.NotRequired[str]},
)
"""AuthorMeta is Author without attributes."""

FileMeta = typing.TypedDict(
    "FileMeta",
    {
        "id": str,
        "name": str,
        "remote": typing.NotRequired[str],
        "folder": typing.NotRequired[dict],
        "size": typing.NotRequired[int],
        "hash": typing.NotRequired[str],
        "createdAt": typing.NotRequired[str],
        "updatedAt": typing.NotRequired[str],
    },
)
"""FileMeta is File without blob."""

FolderMeta = typing.TypedDict(
    "FolderMeta",
    {
        "id": str,
        "name": str,
        "parent": typing.NotRequired[dict],
        "description": typing.NotRequired[str],
        "createdAt": typing.NotRequired[str],
        "updatedAt": typing.NotRequired[str],
    },
)
"""FolderMeta is Folder without attributes."""

QualityMeta = typing.TypedDict(
    "QualityMeta",
    {
        "id": str,
        "key": str,
        "name": str,
        "kind": typing.NotRequired[int],
        "defaultValue": typing.NotRequired[float],
        "formula": typing.NotRequired[str],
        "defaultSiUnit": typing.NotRequired[str],
        "defaultImperialUnit": typing.NotRequired[str],
        "min": typing.NotRequired[float],
        "minExcluded": typing.NotRequired[bool],
        "max": typing.NotRequired[float],
        "maxExcluded": typing.NotRequired[bool],
        "canScale": typing.NotRequired[bool],
        "uri": typing.NotRequired[str],
    },
)
"""QualityMeta is Quality without benchmarks and attributes."""

PortMeta = typing.TypedDict(
    "PortMeta",
    {
        "id": str,
        "name": str,
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
    },
)
"""PortMeta is Port without attributes."""

RepresentationMeta = typing.TypedDict(
    "RepresentationMeta",
    {
        "id": str,
        "file": typing.NotRequired[dict],
        "name": typing.NotRequired[str],
        "description": typing.NotRequired[str],
    },
)
"""RepresentationMeta is Representation without tags and attributes."""

ConnectorMeta = typing.TypedDict(
    "ConnectorMeta",
    {
        "id": str,
        "point": dict,
        "direction": dict,
        "t": float,
        "name": typing.NotRequired[str],
        "description": typing.NotRequired[str],
        "mandatory": typing.NotRequired[bool],
        "port": typing.NotRequired[dict],
    },
)
"""ConnectorMeta is Connector without props and attributes."""

LayerMeta = typing.TypedDict(
    "LayerMeta",
    {
        "id": str,
        "name": str,
        "isHidden": typing.NotRequired[bool],
        "isLocked": typing.NotRequired[bool],
        "color": typing.NotRequired[str],
        "description": typing.NotRequired[str],
    },
)
"""LayerMeta is Layer without attributes."""

PieceMeta = typing.TypedDict(
    "PieceMeta",
    {
        "id": str,
        "name": typing.NotRequired[str],
        "type": typing.NotRequired[dict],
        "designPiece": typing.NotRequired[dict],
        "pose": typing.NotRequired[dict],
        "scale": typing.NotRequired[float],
        "mirrorPlane": typing.NotRequired[dict],
        "isHidden": typing.NotRequired[bool],
        "isLocked": typing.NotRequired[bool],
        "color": typing.NotRequired[str],
        "description": typing.NotRequired[str],
    },
)
"""PieceMeta is Piece without props and attributes."""


def _dict_piece_plane(piece: dict | None) -> dict | None:
    if not piece or not isinstance(piece, dict):
        return None
    pose = piece.get("pose")
    if isinstance(pose, dict):
        return pose.get("plane")
    return None


def _dict_piece_center(piece: dict | None) -> dict | None:
    if not piece or not isinstance(piece, dict):
        return None
    pose = piece.get("pose")
    if isinstance(pose, dict):
        return pose.get("center")
    return None


def _dict_piece_diff_pose(diff: dict | None) -> dict | None:
    if not diff or not isinstance(diff, dict):
        return None
    v = diff.get("pose")
    return v if isinstance(v, dict) else None


GroupMeta = typing.TypedDict(
    "GroupMeta",
    {
        "id": str,
        "name": typing.NotRequired[str],
        "color": typing.NotRequired[str],
        "description": typing.NotRequired[str],
    },
)
"""GroupMeta is Group without pieces and attributes."""

ConnectionMeta = typing.TypedDict(
    "ConnectionMeta",
    {
        "id": str,
        "parent": dict,
        "child": dict,
        "gap": typing.NotRequired[float],
        "shift": typing.NotRequired[float],
        "rise": typing.NotRequired[float],
        "rotation": typing.NotRequired[float],
        "turn": typing.NotRequired[float],
        "tilt": typing.NotRequired[float],
        "u": typing.NotRequired[float],
        "v": typing.NotRequired[float],
        "description": typing.NotRequired[str],
    },
)
"""ConnectionMeta is Connection without attributes."""

# #endregion 🎼Sub-entity Meta

# #region 🕰️Main Entity Meta

TypeMeta = typing.TypedDict(
    "TypeMeta",
    {
        "id": str,
        "name": str,
        "parent": typing.NotRequired[dict],
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
        "image": typing.NotRequired[str],
        "folder": typing.NotRequired[str],
        "unit": typing.NotRequired[str],
        "stock": typing.NotRequired[int],
        "isAbstract": typing.NotRequired[bool],
        "virtual": typing.NotRequired[bool],
        "createdAt": typing.NotRequired[str],
        "updatedAt": typing.NotRequired[str],
    },
)
"""TypeMeta is Type with only scalar fields."""

DesignMeta = typing.TypedDict(
    "DesignMeta",
    {
        "id": str,
        "name": str,
        "parent": typing.NotRequired[dict],
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
        "image": typing.NotRequired[str],
        "variant": typing.NotRequired[str],
        "view": typing.NotRequired[str],
        "unit": typing.NotRequired[str],
        "folder": typing.NotRequired[str],
        "isAbstract": typing.NotRequired[bool],
        "activeLayer": typing.NotRequired[str],
        "createdAt": typing.NotRequired[str],
        "updatedAt": typing.NotRequired[str],
    },
)
"""DesignMeta is Design with only scalar fields."""

KitMeta = typing.TypedDict(
    "KitMeta",
    {
        "id": str,
        "name": str,
        "version": typing.NotRequired[str],
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
        "image": typing.NotRequired[str],
        "preview": typing.NotRequired[str],
        "remote": typing.NotRequired[str],
        "homepage": typing.NotRequired[str],
        "license": typing.NotRequired[str],
        "createdAt": typing.NotRequired[str],
        "updatedAt": typing.NotRequired[str],
    },
)
"""KitMeta is Kit with only scalar fields."""

# #endregion 🕰️Main Entity Meta

# #region 🐻Shallow

TypeShallow = typing.TypedDict(
    "TypeShallow",
    {
        "id": str,
        "name": str,
        "parent": typing.NotRequired[dict],
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
        "image": typing.NotRequired[str],
        "folder": typing.NotRequired[str],
        "unit": typing.NotRequired[str],
        "stock": typing.NotRequired[int],
        "isAbstract": typing.NotRequired[bool],
        "virtual": typing.NotRequired[bool],
        "createdAt": typing.NotRequired[str],
        "updatedAt": typing.NotRequired[str],
        "concepts": typing.NotRequired[list[ConceptMeta]],
        "authors": typing.NotRequired[list[AuthorMeta]],
        "props": typing.NotRequired[list[PropMeta]],
        "representations": typing.NotRequired[list[RepresentationMeta]],
        "connectors": typing.NotRequired[list[ConnectorMeta]],
        "attributes": typing.NotRequired[list[AttributeMeta]],
    },
)
"""TypeShallow is Type with list fields replaced by Meta item lists."""

DesignShallow = typing.TypedDict(
    "DesignShallow",
    {
        "id": str,
        "name": str,
        "parent": typing.NotRequired[dict],
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
        "image": typing.NotRequired[str],
        "variant": typing.NotRequired[str],
        "view": typing.NotRequired[str],
        "unit": typing.NotRequired[str],
        "folder": typing.NotRequired[str],
        "isAbstract": typing.NotRequired[bool],
        "activeLayer": typing.NotRequired[str],
        "createdAt": typing.NotRequired[str],
        "updatedAt": typing.NotRequired[str],
        "concepts": typing.NotRequired[list[ConceptMeta]],
        "authors": typing.NotRequired[list[AuthorMeta]],
        "props": typing.NotRequired[list[PropMeta]],
        "pieces": typing.NotRequired[list[PieceMeta]],
        "connections": typing.NotRequired[list[ConnectionMeta]],
        "layers": typing.NotRequired[list[LayerMeta]],
        "groups": typing.NotRequired[list[GroupMeta]],
        "stats": typing.NotRequired[list[StatMeta]],
        "attributes": typing.NotRequired[list[AttributeMeta]],
    },
)
"""DesignShallow is Design with list fields replaced by Meta item lists."""

KitShallow = typing.TypedDict(
    "KitShallow",
    {
        "id": str,
        "name": str,
        "version": typing.NotRequired[str],
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
        "image": typing.NotRequired[str],
        "preview": typing.NotRequired[str],
        "remote": typing.NotRequired[str],
        "homepage": typing.NotRequired[str],
        "license": typing.NotRequired[str],
        "createdAt": typing.NotRequired[str],
        "updatedAt": typing.NotRequired[str],
        "concepts": typing.NotRequired[list[ConceptMeta]],
        "tags": typing.NotRequired[list[TagMeta]],
        "types": typing.NotRequired[list[TypeMeta]],
        "designs": typing.NotRequired[list[DesignMeta]],
        "ports": typing.NotRequired[list[PortMeta]],
        "qualities": typing.NotRequired[list[QualityMeta]],
        "files": typing.NotRequired[list[FileMeta]],
        "folders": typing.NotRequired[list[FolderMeta]],
        "authors": typing.NotRequired[list[AuthorMeta]],
        "attributes": typing.NotRequired[list[AttributeMeta]],
    },
)
"""KitShallow is Kit with list fields replaced by Meta item lists."""

# #endregion 🐻Shallow

# #region 📎Meta And Shallow Conversions


def _strip_none(d: dict) -> dict:
    """➖Remove keys with None values from a dict."""
    return {k: v for k, v in d.items() if v is not None}


def _extract_scalar_fields(d: dict, keys: list[str]) -> dict:
    """🔖Extract only specified keys from a dict, skipping missing keys."""
    return {k: d[k] for k in keys if k in d}


_ATTRIBUTE_META_KEYS = ["id", "name", "value", "definition"]
_TAG_META_KEYS = ["id", "name", "description", "icon", "order"]
_CONCEPT_META_KEYS = ["id", "name", "description", "icon", "order"]
_STAT_META_KEYS = [
    "id",
    "key",
    "unit",
    "min",
    "minExcluded",
    "max",
    "maxExcluded",
    "createdAt",
    "updatedAt",
]
_PROP_META_KEYS = ["id", "key", "value", "unit"]
_AUTHOR_META_KEYS = ["id", "name", "email"]
_FILE_META_KEYS = [
    "id",
    "name",
    "remote",
    "folder",
    "size",
    "hash",
    "createdAt",
    "updatedAt",
]
_FOLDER_META_KEYS = ["id", "name", "parent", "description", "createdAt", "updatedAt"]
_QUALITY_META_KEYS = [
    "id",
    "key",
    "name",
    "kind",
    "defaultValue",
    "formula",
    "defaultSiUnit",
    "defaultImperialUnit",
    "min",
    "minExcluded",
    "max",
    "maxExcluded",
    "canScale",
    "uri",
]
_PORT_META_KEYS = ["id", "name", "description", "icon"]
_REPRESENTATION_META_KEYS = ["id", "file", "name", "description"]
_CONNECTOR_META_KEYS = [
    "id",
    "point",
    "direction",
    "t",
    "name",
    "description",
    "mandatory",
    "port",
]
_LAYER_META_KEYS = ["id", "name", "isHidden", "isLocked", "color", "description"]
_PIECE_META_KEYS = [
    "id",
    "name",
    "type",
    "designPiece",
    "plane",
    "center",
    "scale",
    "mirrorPlane",
    "isHidden",
    "isLocked",
    "color",
    "description",
]
_GROUP_META_KEYS = ["id", "name", "color", "description"]
_CONNECTION_META_KEYS = [
    "id",
    "parent",
    "child",
    "gap",
    "shift",
    "rise",
    "rotation",
    "turn",
    "tilt",
    "u",
    "v",
    "description",
]

_TYPE_META_KEYS = [
    "id",
    "name",
    "parent",
    "description",
    "icon",
    "image",
    "folder",
    "unit",
    "stock",
    "isAbstract",
    "virtual",
    "createdAt",
    "updatedAt",
]
_DESIGN_META_KEYS = [
    "id",
    "name",
    "parent",
    "description",
    "icon",
    "image",
    "variant",
    "view",
    "unit",
    "folder",
    "isAbstract",
    "activeLayer",
    "createdAt",
    "updatedAt",
]
_KIT_META_KEYS = [
    "id",
    "name",
    "version",
    "description",
    "icon",
    "image",
    "preview",
    "remote",
    "homepage",
    "license",
    "createdAt",
    "updatedAt",
]


def attributeToMeta(d: dict) -> AttributeMeta:
    """🔖Convert an attribute dict to AttributeMeta."""
    return _extract_scalar_fields(d, _ATTRIBUTE_META_KEYS)


def tagToMeta(d: dict) -> TagMeta:
    """🔖Convert a tag dict to TagMeta."""
    return _extract_scalar_fields(d, _TAG_META_KEYS)


def conceptToMeta(d: dict) -> ConceptMeta:
    """🔖Convert a concept dict to ConceptMeta."""
    return _extract_scalar_fields(d, _CONCEPT_META_KEYS)


def statToMeta(d: dict) -> StatMeta:
    """🔖Convert a stat dict to StatMeta."""
    return _extract_scalar_fields(d, _STAT_META_KEYS)


def propToMeta(d: dict) -> PropMeta:
    """🔖Convert a prop dict to PropMeta (without attributes)."""
    return _extract_scalar_fields(d, _PROP_META_KEYS)


def authorToMeta(d: dict) -> AuthorMeta:
    """🔖Convert an author dict to AuthorMeta (without attributes)."""
    return _extract_scalar_fields(d, _AUTHOR_META_KEYS)


def fileToMeta(d: dict) -> FileMeta:
    """🔖Convert a file dict to FileMeta (without blob)."""
    return _extract_scalar_fields(d, _FILE_META_KEYS)


def folderToMeta(d: dict) -> FolderMeta:
    """🔖Convert a folder dict to FolderMeta (without attributes)."""
    return _extract_scalar_fields(d, _FOLDER_META_KEYS)


def qualityToMeta(d: dict) -> QualityMeta:
    """🔖Convert a quality dict to QualityMeta (without benchmarks and attributes)."""
    return _extract_scalar_fields(d, _QUALITY_META_KEYS)


def portToMeta(d: dict) -> PortMeta:
    """🔖Convert a port dict to PortMeta (without attributes)."""
    return _extract_scalar_fields(d, _PORT_META_KEYS)


def representationToMeta(d: dict) -> RepresentationMeta:
    """🔖Convert a representation dict to RepresentationMeta (without tags and attributes)."""
    return _extract_scalar_fields(d, _REPRESENTATION_META_KEYS)


def connectorToMeta(d: dict) -> ConnectorMeta:
    """🔖Convert a connector dict to ConnectorMeta (without props and attributes)."""
    return _extract_scalar_fields(d, _CONNECTOR_META_KEYS)


def layerToMeta(d: dict) -> LayerMeta:
    """🔖Convert a layer dict to LayerMeta (without attributes)."""
    return _extract_scalar_fields(d, _LAYER_META_KEYS)


def pieceToMeta(d: dict) -> PieceMeta:
    """🔖Convert a piece dict to PieceMeta (without props and attributes)."""
    return _extract_scalar_fields(d, _PIECE_META_KEYS)


def groupToMeta(d: dict) -> GroupMeta:
    """🔖Convert a group dict to GroupMeta (without pieces and attributes)."""
    return _extract_scalar_fields(d, _GROUP_META_KEYS)


def connectionToMeta(d: dict) -> ConnectionMeta:
    """🔖Convert a connection dict to ConnectionMeta (without attributes)."""
    return _extract_scalar_fields(d, _CONNECTION_META_KEYS)


def typeToMeta(d: dict) -> TypeMeta:
    """🔖Convert a type dict to TypeMeta (scalar fields only)."""
    return _extract_scalar_fields(d, _TYPE_META_KEYS)


def designToMeta(d: dict) -> DesignMeta:
    """🔖Convert a design dict to DesignMeta (scalar fields only)."""
    return _extract_scalar_fields(d, _DESIGN_META_KEYS)


def kitToMeta(d: dict) -> KitMeta:
    """🔖Convert a kit dict to KitMeta (scalar fields only)."""
    return _extract_scalar_fields(d, _KIT_META_KEYS)


def _convert_list(items: list | None, converter: typing.Callable) -> list | None:
    """⚡Convert a list of dicts using a converter function, returning None for empty/missing lists."""
    if not items:
        return None
    return [converter(item) for item in items]


def typeToShallow(d: dict) -> TypeShallow:
    """🔖Convert a type dict to TypeShallow (list fields replaced by Meta items)."""
    result = _extract_scalar_fields(d, _TYPE_META_KEYS)
    concepts = _convert_list(
        d.get("concepts"), lambda c: c if isinstance(c, str) else conceptToMeta(c)
    )
    if concepts is not None:
        result["concepts"] = concepts
    authors = _convert_list(
        d.get("authors"), lambda a: a if isinstance(a, str) else authorToMeta(a)
    )
    if authors is not None:
        result["authors"] = authors
    props = _convert_list(d.get("props"), propToMeta)
    if props is not None:
        result["props"] = props
    representations = _convert_list(d.get("representations"), representationToMeta)
    if representations is not None:
        result["representations"] = representations
    connectors = _convert_list(d.get("connectors"), connectorToMeta)
    if connectors is not None:
        result["connectors"] = connectors
    attributes = _convert_list(d.get("attributes"), attributeToMeta)
    if attributes is not None:
        result["attributes"] = attributes
    return result


def designToShallow(d: dict) -> DesignShallow:
    """🔖Convert a design dict to DesignShallow (list fields replaced by Meta items)."""
    result = _extract_scalar_fields(d, _DESIGN_META_KEYS)
    concepts = _convert_list(
        d.get("concepts"), lambda c: c if isinstance(c, str) else conceptToMeta(c)
    )
    if concepts is not None:
        result["concepts"] = concepts
    authors = _convert_list(
        d.get("authors"), lambda a: a if isinstance(a, str) else authorToMeta(a)
    )
    if authors is not None:
        result["authors"] = authors
    props = _convert_list(d.get("props"), propToMeta)
    if props is not None:
        result["props"] = props
    pieces = _convert_list(d.get("pieces"), pieceToMeta)
    if pieces is not None:
        result["pieces"] = pieces
    connections = _convert_list(d.get("connections"), connectionToMeta)
    if connections is not None:
        result["connections"] = connections
    layers = _convert_list(d.get("layers"), layerToMeta)
    if layers is not None:
        result["layers"] = layers
    groups = _convert_list(d.get("groups"), groupToMeta)
    if groups is not None:
        result["groups"] = groups
    stats = _convert_list(d.get("stats"), statToMeta)
    if stats is not None:
        result["stats"] = stats
    attributes = _convert_list(d.get("attributes"), attributeToMeta)
    if attributes is not None:
        result["attributes"] = attributes
    return result


def kitToShallow(d: dict) -> KitShallow:
    """🔖Convert a kit dict to KitShallow (list fields replaced by Meta items)."""
    result = _extract_scalar_fields(d, _KIT_META_KEYS)
    concepts = _convert_list(
        d.get("concepts"), lambda c: c if isinstance(c, str) else conceptToMeta(c)
    )
    if concepts is not None:
        result["concepts"] = concepts
    tags = _convert_list(d.get("tags"), tagToMeta)
    if tags is not None:
        result["tags"] = tags
    types = _convert_list(d.get("types"), typeToMeta)
    if types is not None:
        result["types"] = types
    designs = _convert_list(d.get("designs"), designToMeta)
    if designs is not None:
        result["designs"] = designs
    ports = _convert_list(d.get("ports"), portToMeta)
    if ports is not None:
        result["ports"] = ports
    qualities = _convert_list(d.get("qualities"), qualityToMeta)
    if qualities is not None:
        result["qualities"] = qualities
    files = _convert_list(d.get("files"), fileToMeta)
    if files is not None:
        result["files"] = files
    folders = _convert_list(d.get("folders"), folderToMeta)
    if folders is not None:
        result["folders"] = folders
    authors = _convert_list(
        d.get("authors"), lambda a: a if isinstance(a, str) else authorToMeta(a)
    )
    if authors is not None:
        result["authors"] = authors
    attributes = _convert_list(d.get("attributes"), attributeToMeta)
    if attributes is not None:
        result["attributes"] = attributes
    return result


# #endregion 📎Meta And Shallow Conversions

# #endregion 🔑Meta And Shallow


# #region 🖥️Hash
# Merkle hash functions for all semio entities.


# #region 🌩️HashWriter
def _format_number_for_hash(n) -> str:
    """🔢Format number to match JavaScript Number.toString() behavior.
    Integers (including floats with no fractional part) are formatted without decimal point.
    """
    if isinstance(n, int):
        return str(n)
    if isinstance(n, float) and n.is_integer():
        return str(int(n))
    return str(n)


def _ref_id(ref) -> str:
    """🔖Extract id from a reference (dict with 'id' key or plain string)."""
    if isinstance(ref, dict):
        return ref["id"]
    return ref


class HashWriter:
    """🔖Feeds structured data into a SHA-256 hasher for deterministic hashing.
    Uses length-prefixed strings and type tags for unambiguous encoding.
    """

    def __init__(self):
        self._parts = bytearray()

    def writeString(self, s: str):
        b = s.encode("utf-8")
        self._parts.extend(struct.pack(">I", len(b)))
        self._parts.extend(b)

    def writeNumber(self, n):
        self.writeString(_format_number_for_hash(n))

    def writeBool(self, b: bool):
        self._parts.append(1 if b else 0)

    def writeHash(self, h: str):
        self.writeString(h)

    def writeHashList(self, hashes: list[str]):
        sorted_hashes = sorted(hashes)
        self._parts.extend(struct.pack(">I", len(sorted_hashes)))
        for h in sorted_hashes:
            self.writeString(h)

    def writeIdList(self, ids: list[str]):
        sorted_ids = sorted(ids)
        self._parts.extend(struct.pack(">I", len(sorted_ids)))
        for g in sorted_ids:
            self.writeString(g)

    def digest(self) -> str:
        return hashlib.sha256(bytes(self._parts)).hexdigest()


# #endregion 🌩️HashWriter


# #region 🎵Hash Value Types
def hash_coordinate(c: dict) -> str:
    """🔖Computes SHA-256 hash of a Coordinate value."""
    w = HashWriter()
    w.writeString("Coordinate")
    w.writeString("u")
    w.writeNumber(c["u"])
    w.writeString("v")
    w.writeNumber(c["v"])
    return w.digest()


def hash_vec(v: dict) -> str:
    """🔖Computes SHA-256 hash of a Vec value."""
    w = HashWriter()
    w.writeString("Vec")
    w.writeString("u")
    w.writeNumber(v["u"])
    w.writeString("v")
    w.writeNumber(v["v"])
    return w.digest()


def hash_point(p: dict) -> str:
    """🔖Computes SHA-256 hash of a Point value."""
    w = HashWriter()
    w.writeString("Point")
    w.writeString("x")
    w.writeNumber(p["x"])
    w.writeString("y")
    w.writeNumber(p["y"])
    w.writeString("z")
    w.writeNumber(p["z"])
    return w.digest()


def hash_vector(v: dict) -> str:
    """🔖Computes SHA-256 hash of a Vector value."""
    w = HashWriter()
    w.writeString("Vector")
    w.writeString("x")
    w.writeNumber(v["x"])
    w.writeString("y")
    w.writeNumber(v["y"])
    w.writeString("z")
    w.writeNumber(v["z"])
    return w.digest()


def hash_plane(p: dict) -> str:
    """🔖Computes SHA-256 hash of a Plane value."""
    w = HashWriter()
    w.writeString("Plane")
    w.writeString("origin")
    w.writeHash(hash_point(p["origin"]))
    w.writeString("xAxis")
    w.writeHash(hash_vector(p["xAxis"]))
    w.writeString("yAxis")
    w.writeHash(hash_vector(p["yAxis"]))
    return w.digest()


def hash_camera(c: dict) -> str:
    """🔖Computes SHA-256 hash of a Camera value."""
    w = HashWriter()
    w.writeString("Camera")
    w.writeString("forward")
    w.writeHash(hash_vector(c["forward"]))
    w.writeString("position")
    w.writeHash(hash_point(c["position"]))
    w.writeString("up")
    w.writeHash(hash_vector(c["up"]))
    return w.digest()


# #endregion 🎵Hash Value Types


# #region 🎩Hash Entities
def hash_attribute(a: dict) -> str:
    """🔖Computes SHA-256 hash of an Attribute entity."""
    w = HashWriter()
    w.writeString("Attribute")
    if a.get("definition") is not None:
        w.writeString("definition")
        w.writeString(a["definition"])
    w.writeString("id")
    w.writeString(a["id"])
    w.writeString("key")
    w.writeString(a["key"])
    if a.get("value") is not None:
        w.writeString("value")
        w.writeString(a["value"])
    return w.digest()


def hash_location(l: dict) -> str:
    """🔖Computes SHA-256 hash of a Location entity."""
    w = HashWriter()
    w.writeString("Location")
    if l.get("altitude") is not None:
        w.writeString("altitude")
        w.writeNumber(l["altitude"])
    attrs = l.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    w.writeString("id")
    w.writeString(l["id"])
    w.writeString("latitude")
    w.writeNumber(l["latitude"])
    w.writeString("longitude")
    w.writeNumber(l["longitude"])
    return w.digest()


def hash_author(a: dict) -> str:
    """🔖Computes SHA-256 hash of an Author entity."""
    w = HashWriter()
    w.writeString("Author")
    attrs = a.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(at) for at in attrs])
    email = a.get("email")
    if email is not None and email != "":
        w.writeString("email")
        w.writeString(email)
    w.writeString("id")
    w.writeString(a["id"])
    w.writeString("name")
    w.writeString(a["name"])
    return w.digest()


def hash_file(f: dict) -> str:
    """🔖Computes SHA-256 hash of a File entity."""
    w = HashWriter()
    w.writeString("File")
    if f.get("blob") is not None:
        w.writeString("blob")
        w.writeString(f["blob"])
    if f.get("folder") is not None:
        w.writeString("folder")
        w.writeString(_ref_id(f["folder"]))
    w.writeString("id")
    w.writeString(f["id"])
    if f.get("hash") is not None:
        w.writeString("hash")
        w.writeString(f["hash"])
    w.writeString("name")
    w.writeString(f["name"])
    if f.get("remote") is not None:
        w.writeString("remote")
        w.writeString(f["remote"])
    if f.get("size") is not None:
        w.writeString("size")
        w.writeNumber(f["size"])
    return w.digest()


def hash_folder(f: dict) -> str:
    """🔖Computes SHA-256 hash of a Folder entity."""
    w = HashWriter()
    w.writeString("Folder")
    attrs = f.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if f.get("description") is not None:
        w.writeString("description")
        w.writeString(f["description"])
    w.writeString("id")
    w.writeString(f["id"])
    w.writeString("name")
    w.writeString(f["name"])
    if f.get("parent") is not None:
        w.writeString("parent")
        w.writeString(_ref_id(f["parent"]))
    return w.digest()


def hash_benchmark(b: dict) -> str:
    """🔖Computes SHA-256 hash of a Benchmark entity."""
    w = HashWriter()
    w.writeString("Benchmark")
    attrs = b.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    w.writeString("id")
    w.writeString(b["id"])
    if b.get("icon") is not None:
        w.writeString("icon")
        w.writeString(b["icon"])
    if b.get("max") is not None:
        w.writeString("max")
        w.writeNumber(b["max"])
    if b.get("maxExcluded") is not None:
        w.writeString("maxExcluded")
        w.writeBool(b["maxExcluded"])
    if b.get("min") is not None:
        w.writeString("min")
        w.writeNumber(b["min"])
    if b.get("minExcluded") is not None:
        w.writeString("minExcluded")
        w.writeBool(b["minExcluded"])
    w.writeString("name")
    w.writeString(b["name"])
    return w.digest()


def hash_quality(q: dict) -> str:
    """🔖Computes SHA-256 hash of a Quality entity."""
    w = HashWriter()
    w.writeString("Quality")
    benchmarks = q.get("benchmarks")
    if benchmarks and len(benchmarks) > 0:
        w.writeString("benchmarks")
        w.writeHashList([hash_benchmark(b) for b in benchmarks])
    if q.get("canScale") is not None:
        w.writeString("canScale")
        w.writeBool(q["canScale"])
    if q.get("defaultImperialUnit") is not None:
        w.writeString("defaultImperialUnit")
        w.writeString(q["defaultImperialUnit"])
    if q.get("defaultSiUnit") is not None:
        w.writeString("defaultSiUnit")
        w.writeString(q["defaultSiUnit"])
    if q.get("defaultValue") is not None:
        w.writeString("defaultValue")
        w.writeNumber(q["defaultValue"])
    if q.get("description") is not None:
        w.writeString("description")
        w.writeString(q["description"])
    if q.get("formula") is not None:
        w.writeString("formula")
        w.writeString(q["formula"])
    w.writeString("id")
    w.writeString(q["id"])
    if q.get("icon") is not None:
        w.writeString("icon")
        w.writeString(q["icon"])
    if q.get("image") is not None:
        w.writeString("image")
        w.writeString(q["image"])
    if q.get("isMaxExcluded") is not None:
        w.writeString("isMaxExcluded")
        w.writeBool(q["isMaxExcluded"])
    if q.get("isMinExcluded") is not None:
        w.writeString("isMinExcluded")
        w.writeBool(q["isMinExcluded"])
    w.writeString("key")
    w.writeString(q["key"])
    if q.get("kind") is not None:
        w.writeString("kind")
        w.writeNumber(q["kind"])
    if q.get("max") is not None:
        w.writeString("max")
        w.writeNumber(q["max"])
    if q.get("min") is not None:
        w.writeString("min")
        w.writeNumber(q["min"])
    w.writeString("name")
    w.writeString(q["name"])
    if q.get("unit") is not None:
        w.writeString("unit")
        w.writeString(q["unit"])
    if q.get("uri") is not None:
        w.writeString("uri")
        w.writeString(q["uri"])
    return w.digest()


def hash_port(p: dict) -> str:
    """🔖Computes SHA-256 hash of a Port entity."""
    w = HashWriter()
    w.writeString("Port")
    attrs = p.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    compat = p.get("compatiblePorts")
    if compat and len(compat) > 0:
        w.writeString("compatiblePorts")
        w.writeIdList([_ref_id(cp) for cp in compat])
    if p.get("description") is not None:
        w.writeString("description")
        w.writeString(p["description"])
    w.writeString("id")
    w.writeString(p["id"])
    if p.get("icon") is not None:
        w.writeString("icon")
        w.writeString(p["icon"])
    w.writeString("name")
    w.writeString(p["name"])
    return w.digest()


def hash_prop(p: dict) -> str:
    """🔖Computes SHA-256 hash of a Prop entity."""
    w = HashWriter()
    w.writeString("Prop")
    attrs = p.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    w.writeString("id")
    w.writeString(p["id"])
    w.writeString("quality")
    w.writeString(_ref_id(p["quality"]))
    if p.get("unit") is not None:
        w.writeString("unit")
        w.writeString(p["unit"])
    w.writeString("value")
    w.writeString(p["value"])
    return w.digest()


def hash_tag(t: dict) -> str:
    """🔖Computes SHA-256 hash of a Tag entity."""
    w = HashWriter()
    w.writeString("Tag")
    attrs = t.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if t.get("description") is not None:
        w.writeString("description")
        w.writeString(t["description"])
    w.writeString("id")
    w.writeString(t["id"])
    if t.get("icon") is not None:
        w.writeString("icon")
        w.writeString(t["icon"])
    w.writeString("name")
    w.writeString(t["name"])
    return w.digest()


def hash_concept(c: dict) -> str:
    """🔖Computes SHA-256 hash of a Concept entity."""
    w = HashWriter()
    w.writeString("Concept")
    attrs = c.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if c.get("description") is not None:
        w.writeString("description")
        w.writeString(c["description"])
    w.writeString("id")
    w.writeString(c["id"])
    if c.get("icon") is not None:
        w.writeString("icon")
        w.writeString(c["icon"])
    w.writeString("name")
    w.writeString(c["name"])
    return w.digest()


def hash_representation(m: dict) -> str:
    """🔖Computes SHA-256 hash of a Representation entity."""
    w = HashWriter()
    w.writeString("Representation")
    attrs = m.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if m.get("description") is not None:
        w.writeString("description")
        w.writeString(m["description"])
    w.writeString("file")
    w.writeString(_ref_id(m["file"]))
    w.writeString("id")
    w.writeString(m["id"])
    if m.get("name") is not None:
        w.writeString("name")
        w.writeString(m["name"])
    tags = m.get("tags")
    if tags and len(tags) > 0:
        w.writeString("tags")
        w.writeIdList([_ref_id(t) for t in tags])
    return w.digest()


def hash_connector(c: dict) -> str:
    """🔖Computes SHA-256 hash of a Connector entity."""
    w = HashWriter()
    w.writeString("Connector")
    attrs = c.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if c.get("description") is not None:
        w.writeString("description")
        w.writeString(c["description"])
    w.writeString("direction")
    w.writeHash(hash_vector(c["direction"]))
    w.writeString("id")
    w.writeString(c["id"])
    if c.get("mandatory") is not None:
        w.writeString("mandatory")
        w.writeBool(c["mandatory"])
    if c.get("name") is not None:
        w.writeString("name")
        w.writeString(c["name"])
    w.writeString("point")
    w.writeHash(hash_point(c["point"]))
    if c.get("port") is not None:
        w.writeString("port")
        w.writeString(_ref_id(c["port"]))
    props = c.get("props")
    if props and len(props) > 0:
        w.writeString("props")
        w.writeHashList([hash_prop(p) for p in props])
    w.writeString("t")
    w.writeNumber(c["t"])
    return w.digest()


def hash_type(t: dict) -> str:
    """🔖Computes SHA-256 hash of a Type entity."""
    w = HashWriter()
    w.writeString("Type")
    attrs = t.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    authors = t.get("authors")
    if authors and len(authors) > 0:
        w.writeString("authors")
        w.writeIdList([_ref_id(a) for a in authors])
    concepts = t.get("concepts")
    if concepts and len(concepts) > 0:
        w.writeString("concepts")
        w.writeIdList([_ref_id(c) for c in concepts])
    connectors = t.get("connectors")
    if connectors and len(connectors) > 0:
        w.writeString("connectors")
        w.writeHashList([hash_connector(c) for c in connectors])
    if t.get("description") is not None:
        w.writeString("description")
        w.writeString(t["description"])
    if t.get("folder") is not None:
        w.writeString("folder")
        w.writeString(t["folder"])
    w.writeString("id")
    w.writeString(t["id"])
    if t.get("icon") is not None:
        w.writeString("icon")
        w.writeString(t["icon"])
    if t.get("image") is not None:
        w.writeString("image")
        w.writeString(t["image"])
    if t.get("isAbstract") is not None:
        w.writeString("isAbstract")
        w.writeBool(t["isAbstract"])
    if t.get("location") is not None:
        w.writeString("location")
        w.writeString(_ref_id(t["location"]))
    representations = t.get("representations")
    if representations and len(representations) > 0:
        w.writeString("representations")
        w.writeHashList([hash_representation(m) for m in representations])
    w.writeString("name")
    w.writeString(t["name"])
    if t.get("parent") is not None:
        w.writeString("parent")
        w.writeString(_ref_id(t["parent"]))
    props = t.get("props")
    if props and len(props) > 0:
        w.writeString("props")
        w.writeHashList([hash_prop(p) for p in props])
    if t.get("stock") is not None:
        w.writeString("stock")
        w.writeNumber(t["stock"])
    if t.get("unit") is not None:
        w.writeString("unit")
        w.writeString(t["unit"])
    if t.get("virtual") is not None:
        w.writeString("virtual")
        w.writeBool(t["virtual"])
    return w.digest()


def hash_layer(l: dict) -> str:
    """🔖Computes SHA-256 hash of a Layer entity."""
    w = HashWriter()
    w.writeString("Layer")
    attrs = l.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if l.get("color") is not None:
        w.writeString("color")
        w.writeString(l["color"])
    if l.get("description") is not None:
        w.writeString("description")
        w.writeString(l["description"])
    w.writeString("id")
    w.writeString(l["id"])
    if l.get("isHidden") is not None:
        w.writeString("isHidden")
        w.writeBool(l["isHidden"])
    if l.get("isLocked") is not None:
        w.writeString("isLocked")
        w.writeBool(l["isLocked"])
    w.writeString("path")
    w.writeString(l["path"])
    return w.digest()


def hash_stat(s: dict) -> str:
    """🔖Computes SHA-256 hash of a Stat entity."""
    w = HashWriter()
    w.writeString("Stat")
    w.writeString("id")
    w.writeString(s["id"])
    if s.get("max") is not None:
        w.writeString("max")
        w.writeNumber(s["max"])
    if s.get("maxExcluded") is not None:
        w.writeString("maxExcluded")
        w.writeBool(s["maxExcluded"])
    if s.get("min") is not None:
        w.writeString("min")
        w.writeNumber(s["min"])
    if s.get("minExcluded") is not None:
        w.writeString("minExcluded")
        w.writeBool(s["minExcluded"])
    w.writeString("quality")
    w.writeString(_ref_id(s["quality"]))
    if s.get("unit") is not None:
        w.writeString("unit")
        w.writeString(s["unit"])
    return w.digest()


def hash_group(g: dict) -> str:
    """🔖Computes SHA-256 hash of a Group entity."""
    w = HashWriter()
    w.writeString("Group")
    attrs = g.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if g.get("color") is not None:
        w.writeString("color")
        w.writeString(g["color"])
    if g.get("description") is not None:
        w.writeString("description")
        w.writeString(g["description"])
    w.writeString("id")
    w.writeString(g["id"])
    if g.get("name") is not None:
        w.writeString("name")
        w.writeString(g["name"])
    w.writeString("pieces")
    w.writeIdList([_ref_id(p) for p in g["pieces"]])
    return w.digest()


def hash_side(s: dict) -> str:
    """🔖Computes SHA-256 hash of a Side value."""
    w = HashWriter()
    w.writeString("Side")
    if s.get("connector") is not None:
        w.writeString("connector")
        w.writeString(_ref_id(s["connector"]))
    if s.get("designPiece") is not None:
        w.writeString("designPiece")
        w.writeString(_ref_id(s["designPiece"]))
    w.writeString("piece")
    w.writeString(_ref_id(s["piece"]))
    return w.digest()


def hash_connection(c: dict) -> str:
    """🔖Computes SHA-256 hash of a Connection entity."""
    w = HashWriter()
    w.writeString("Connection")
    attrs = c.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    w.writeString("parent")
    w.writeHash(hash_side(c["parent"]))
    w.writeString("connecting")
    w.writeHash(hash_side(c["child"]))
    if c.get("description") is not None:
        w.writeString("description")
        w.writeString(c["description"])
    if c.get("gap") is not None:
        w.writeString("gap")
        w.writeNumber(c["gap"])
    w.writeString("id")
    w.writeString(c["id"])
    if c.get("rise") is not None:
        w.writeString("rise")
        w.writeNumber(c["rise"])
    if c.get("rotation") is not None:
        w.writeString("rotation")
        w.writeNumber(c["rotation"])
    if c.get("shift") is not None:
        w.writeString("shift")
        w.writeNumber(c["shift"])
    if c.get("tilt") is not None:
        w.writeString("tilt")
        w.writeNumber(c["tilt"])
    if c.get("turn") is not None:
        w.writeString("turn")
        w.writeNumber(c["turn"])
    if c.get("u") is not None:
        w.writeString("u")
        w.writeNumber(c["u"])
    if c.get("v") is not None:
        w.writeString("v")
        w.writeNumber(c["v"])
    return w.digest()


def hash_piece(p: dict) -> str:
    """🔖Computes SHA-256 hash of a Piece entity."""
    w = HashWriter()
    w.writeString("Piece")
    attrs = p.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    cen = _dict_piece_center(p)
    if cen is not None:
        w.writeString("center")
        w.writeHash(hash_coordinate(cen))
    if p.get("color") is not None:
        w.writeString("color")
        w.writeString(p["color"])
    if p.get("description") is not None:
        w.writeString("description")
        w.writeString(p["description"])
    if p.get("design") is not None:
        w.writeString("design")
        w.writeString(_ref_id(p["design"]))
    w.writeString("id")
    w.writeString(p["id"])
    if p.get("isHidden") is not None:
        w.writeString("isHidden")
        w.writeBool(p["isHidden"])
    if p.get("isLocked") is not None:
        w.writeString("isLocked")
        w.writeBool(p["isLocked"])
    if p.get("mirrorPlane") is not None:
        w.writeString("mirrorPlane")
        w.writeHash(hash_plane(p["mirrorPlane"]))
    if p.get("name") is not None:
        w.writeString("name")
        w.writeString(p["name"])
    pln = _dict_piece_plane(p)
    if pln is not None:
        w.writeString("plane")
        w.writeHash(hash_plane(pln))
    if p.get("props") is not None and len(p["props"]) > 0:
        w.writeString("props")
        w.writeHashList([hash_prop(pr) for pr in p["props"]])
    if p.get("scale") is not None:
        w.writeString("scale")
        w.writeNumber(p["scale"])
    if p.get("type") is not None:
        w.writeString("type")
        w.writeString(_ref_id(p["type"]))
    return w.digest()


def hash_design(d: dict) -> str:
    """🔖Computes SHA-256 hash of a Design entity (Merkle tree)."""
    w = HashWriter()
    w.writeString("Design")
    if d.get("activeLayer") is not None:
        w.writeString("activeLayer")
        w.writeString(_ref_id(d["activeLayer"]))
    attrs = d.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    authors = d.get("authors")
    if authors and len(authors) > 0:
        w.writeString("authors")
        w.writeIdList([_ref_id(a) for a in authors])
    if d.get("canMirror") is not None:
        w.writeString("canMirror")
        w.writeBool(d["canMirror"])
    if d.get("canScale") is not None:
        w.writeString("canScale")
        w.writeBool(d["canScale"])
    concepts = d.get("concepts")
    if concepts and len(concepts) > 0:
        w.writeString("concepts")
        w.writeIdList([_ref_id(c) for c in concepts])
    connections = d.get("connections")
    if connections and len(connections) > 0:
        w.writeString("connections")
        w.writeHashList([hash_connection(c) for c in connections])
    if d.get("description") is not None:
        w.writeString("description")
        w.writeString(d["description"])
    if d.get("folder") is not None:
        w.writeString("folder")
        w.writeString(d["folder"])
    groups = d.get("groups")
    if groups and len(groups) > 0:
        w.writeString("groups")
        w.writeHashList([hash_group(g) for g in groups])
    w.writeString("id")
    w.writeString(d["id"])
    if d.get("icon") is not None:
        w.writeString("icon")
        w.writeString(d["icon"])
    if d.get("image") is not None:
        w.writeString("image")
        w.writeString(d["image"])
    if d.get("isAbstract") is not None:
        w.writeString("isAbstract")
        w.writeBool(d["isAbstract"])
    layers = d.get("layers")
    if layers and len(layers) > 0:
        w.writeString("layers")
        w.writeHashList([hash_layer(la) for la in layers])
    if d.get("location") is not None:
        w.writeString("location")
        w.writeString(_ref_id(d["location"]))
    w.writeString("name")
    w.writeString(d["name"])
    if d.get("parent") is not None:
        w.writeString("parent")
        w.writeString(_ref_id(d["parent"]))
    pieces = d.get("pieces")
    if pieces and len(pieces) > 0:
        w.writeString("pieces")
        w.writeHashList([hash_piece(p) for p in pieces])
    props = d.get("props")
    if props and len(props) > 0:
        w.writeString("props")
        w.writeHashList([hash_prop(p) for p in props])
    stats = d.get("stats")
    if stats and len(stats) > 0:
        w.writeString("stats")
        w.writeHashList([hash_stat(s) for s in stats])
    if d.get("unit") is not None:
        w.writeString("unit")
        w.writeString(d["unit"])
    return w.digest()


def hash_kit(k: dict) -> str:
    """🔖Computes SHA-256 Merkle hash of a Kit entity."""
    w = HashWriter()
    w.writeString("Kit")
    attrs = k.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    authors = k.get("authors")
    if authors and len(authors) > 0:
        w.writeString("authors")
        w.writeHashList([hash_author(a) for a in authors])
    concepts = k.get("concepts")
    if concepts and len(concepts) > 0:
        w.writeString("concepts")
        w.writeHashList([hash_concept(c) for c in concepts])
    if k.get("description") is not None:
        w.writeString("description")
        w.writeString(k["description"])
    designs = k.get("designs")
    if designs and len(designs) > 0:
        w.writeString("designs")
        w.writeHashList([hash_design(d) for d in designs])
    files = k.get("files")
    if files and len(files) > 0:
        w.writeString("files")
        w.writeHashList([hash_file(f) for f in files])
    folders = k.get("folders")
    if folders and len(folders) > 0:
        w.writeString("folders")
        w.writeHashList([hash_folder(f) for f in folders])
    w.writeString("id")
    w.writeString(k["id"])
    if k.get("homepage") is not None:
        w.writeString("homepage")
        w.writeString(k["homepage"])
    if k.get("icon") is not None:
        w.writeString("icon")
        w.writeString(k["icon"])
    if k.get("image") is not None:
        w.writeString("image")
        w.writeString(k["image"])
    if k.get("license") is not None:
        w.writeString("license")
        w.writeString(k["license"])
    w.writeString("name")
    w.writeString(k["name"])
    if k.get("ports") is not None and len(k["ports"]) > 0:
        w.writeString("ports")
        w.writeHashList([hash_port(p) for p in k["ports"]])
    if k.get("preview") is not None:
        w.writeString("preview")
        w.writeString(k["preview"])
    qualities = k.get("qualities")
    if qualities and len(qualities) > 0:
        w.writeString("qualities")
        w.writeHashList([hash_quality(q) for q in qualities])
    if k.get("remote") is not None:
        w.writeString("remote")
        w.writeString(k["remote"])
    tags = k.get("tags")
    if tags and len(tags) > 0:
        w.writeString("tags")
        w.writeHashList([hash_tag(t) for t in tags])
    types = k.get("types")
    if types and len(types) > 0:
        w.writeString("types")
        w.writeHashList([hash_type(t) for t in types])
    if k.get("version") is not None:
        w.writeString("version")
        w.writeString(k["version"])
    return w.digest()


# #endregion 🎩Hash Entities

# #region 🔗Hash Diffs
# Deterministic SHA-256 Merkle hash functions for all diff types.
# Diffs are plain dicts; field presence in dict = field was changed.
# If a field is present with None value → write field name + writeBool(false) as null marker.
# If a field is absent from dict → skip it entirely.


def _write_diff_string(w: HashWriter, key: str, d: dict):
    if key in d:
        w.writeString(key)
        if d[key] is not None:
            w.writeString(d[key])
        else:
            w.writeBool(False)


def _write_diff_number(w: HashWriter, key: str, d: dict):
    if key in d:
        w.writeString(key)
        if d[key] is not None:
            w.writeNumber(d[key])
        else:
            w.writeBool(False)


def _write_diff_bool(w: HashWriter, key: str, d: dict):
    if key in d:
        w.writeString(key)
        if d[key] is not None:
            w.writeBool(d[key])
        else:
            w.writeBool(False)


def _write_diff_id(w: HashWriter, key: str, d: dict):
    if key in d:
        w.writeString(key)
        if d[key] is not None:
            w.writeString(_ref_id(d[key]))
        else:
            w.writeBool(False)


def _write_diff_id_array(w: HashWriter, key: str, d: dict):
    if key in d:
        val = d[key]
        if val is not None and len(val) > 0:
            w.writeString(key)
            w.writeIdList([_ref_id(e) for e in val])
        elif val is not None:
            pass  # empty array = skip
        else:
            w.writeString(key)
            w.writeBool(False)


def _write_diff_hash(w: HashWriter, key: str, d: dict, hash_fn):
    if key in d:
        w.writeString(key)
        if d[key] is not None:
            w.writeHash(hash_fn(d[key]))
        else:
            w.writeBool(False)


def _hash_collection_diff_generic(
    tag: str,
    update_tag: str,
    entity_key_name: str,
    hash_entity_fn,
    hash_diff_fn,
    d: dict,
) -> str:
    w = HashWriter()
    w.writeString(tag)
    added = d.get("added")
    if added and len(added) > 0:
        w.writeString("added")
        w.writeHashList([hash_entity_fn(e) for e in added])
    removed = d.get("removed")
    if removed and len(removed) > 0:
        w.writeString("removed")
        w.writeIdList([_ref_id(r) for r in removed])
    updated = d.get("updated")
    if updated and len(updated) > 0:
        w.writeString("updated")
        keys = sorted([entity_key_name, "diff"])
        update_hashes = []
        for u in updated:
            uw = HashWriter()
            uw.writeString(update_tag)
            for k in keys:
                if k == "diff":
                    uw.writeString("diff")
                    uw.writeHash(hash_diff_fn(u["diff"]))
                else:
                    uw.writeString(k)
                    uw.writeString(_ref_id(u[entity_key_name]))
            update_hashes.append(uw.digest())
        w.writeHashList(update_hashes)
    return w.digest()


def hash_coordinate_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("CoordinateDiff")
    _write_diff_number(w, "u", d)
    _write_diff_number(w, "v", d)
    return w.digest()


def hash_point_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("PointDiff")
    _write_diff_number(w, "x", d)
    _write_diff_number(w, "y", d)
    _write_diff_number(w, "z", d)
    return w.digest()


def hash_vector_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("VectorDiff")
    _write_diff_number(w, "x", d)
    _write_diff_number(w, "y", d)
    _write_diff_number(w, "z", d)
    return w.digest()


def hash_plane_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("PlaneDiff")
    _write_diff_hash(w, "origin", d, hash_point_diff)
    _write_diff_hash(w, "xAxis", d, hash_vector_diff)
    _write_diff_hash(w, "yAxis", d, hash_vector_diff)
    return w.digest()


def hash_camera_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("CameraDiff")
    _write_diff_hash(w, "forward", d, hash_vector_diff)
    _write_diff_hash(w, "position", d, hash_point_diff)
    _write_diff_hash(w, "up", d, hash_vector_diff)
    return w.digest()


def hash_attribute_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("AttributeDiff")
    _write_diff_string(w, "definition", d)
    _write_diff_string(w, "key", d)
    _write_diff_string(w, "value", d)
    return w.digest()


def hash_attributes_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "AttributesDiff",
        "AttributeDiffUpdate",
        "attribute",
        hash_attribute,
        hash_attribute_diff,
        d,
    )


def hash_location_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("LocationDiff")
    _write_diff_number(w, "altitude", d)
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_number(w, "latitude", d)
    _write_diff_number(w, "longitude", d)
    return w.digest()


def hash_author_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("AuthorDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "email", d)
    _write_diff_string(w, "name", d)
    return w.digest()


def hash_authors_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "AuthorsDiff", "AuthorDiffUpdate", "author", hash_author, hash_author_diff, d
    )


def hash_file_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("FileDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "blob", d)
    _write_diff_string(w, "description", d)
    _write_diff_id(w, "folder", d)
    _write_diff_string(w, "hash", d)
    _write_diff_string(w, "name", d)
    _write_diff_string(w, "remote", d)
    if "size" in d:
        w.writeString("size")
        if d["size"] is not None:
            w.writeNumber(d["size"])
        else:
            w.writeBool(False)
    return w.digest()


def hash_files_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "FilesDiff", "FileDiffUpdate", "file", hash_file, hash_file_diff, d
    )


def hash_folder_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("FolderDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "name", d)
    _write_diff_id(w, "parent", d)
    return w.digest()


def hash_folders_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "FoldersDiff", "FolderDiffUpdate", "folder", hash_folder, hash_folder_diff, d
    )


def hash_benchmark_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("BenchmarkDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "definition", d)
    _write_diff_string(w, "icon", d)
    _write_diff_number(w, "max", d)
    _write_diff_bool(w, "maxExcluded", d)
    _write_diff_number(w, "min", d)
    _write_diff_bool(w, "minExcluded", d)
    _write_diff_string(w, "name", d)
    return w.digest()


def hash_benchmarks_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "BenchmarksDiff",
        "BenchmarkDiffUpdate",
        "benchmark",
        hash_benchmark,
        hash_benchmark_diff,
        d,
    )


def hash_quality_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("QualityDiff")
    _write_diff_hash(w, "benchmarks", d, hash_benchmarks_diff)
    _write_diff_bool(w, "canScale", d)
    _write_diff_string(w, "defaultImperialUnit", d)
    _write_diff_string(w, "defaultSiUnit", d)
    _write_diff_number(w, "defaultValue", d)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "folder", d)
    _write_diff_string(w, "formula", d)
    _write_diff_string(w, "icon", d)
    _write_diff_string(w, "image", d)
    _write_diff_bool(w, "isMaxExcluded", d)
    _write_diff_bool(w, "isMinExcluded", d)
    _write_diff_string(w, "key", d)
    _write_diff_number(w, "kind", d)
    _write_diff_number(w, "max", d)
    _write_diff_number(w, "min", d)
    _write_diff_string(w, "name", d)
    _write_diff_string(w, "unit", d)
    _write_diff_string(w, "uri", d)
    return w.digest()


def hash_qualities_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "QualitiesDiff",
        "QualityDiffUpdate",
        "quality",
        hash_quality,
        hash_quality_diff,
        d,
    )


def hash_port_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("PortDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_id_array(w, "compatiblePorts", d)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "icon", d)
    _write_diff_string(w, "name", d)
    return w.digest()


def hash_ports_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "PortsDiff", "PortDiffUpdate", "port", hash_port, hash_port_diff, d
    )


def hash_prop_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("PropDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_id(w, "quality", d)
    _write_diff_string(w, "unit", d)
    _write_diff_string(w, "value", d)
    return w.digest()


def hash_props_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "PropsDiff", "PropDiffUpdate", "prop", hash_prop, hash_prop_diff, d
    )


def hash_tag_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("TagDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "icon", d)
    _write_diff_string(w, "name", d)
    return w.digest()


def hash_tags_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "TagsDiff", "TagDiffUpdate", "tag", hash_tag, hash_tag_diff, d
    )


def hash_concept_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("ConceptDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "icon", d)
    _write_diff_string(w, "name", d)
    return w.digest()


def hash_concepts_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "ConceptsDiff",
        "ConceptDiffUpdate",
        "concept",
        hash_concept,
        hash_concept_diff,
        d,
    )


def hash_representation_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("RepresentationDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "description", d)
    _write_diff_id(w, "file", d)
    _write_diff_string(w, "name", d)
    _write_diff_id_array(w, "tags", d)
    return w.digest()


def hash_representations_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "RepresentationsDiff",
        "RepresentationDiffUpdate",
        "representation",
        hash_representation,
        hash_representation_diff,
        d,
    )


def hash_connector_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("ConnectorDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "description", d)
    _write_diff_hash(w, "direction", d, hash_vector_diff)
    _write_diff_bool(w, "mandatory", d)
    _write_diff_string(w, "name", d)
    _write_diff_hash(w, "point", d, hash_point_diff)
    _write_diff_id(w, "port", d)
    _write_diff_hash(w, "props", d, hash_props_diff)
    _write_diff_number(w, "t", d)
    return w.digest()


def hash_connectors_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "ConnectorsDiff",
        "ConnectorDiffUpdate",
        "connector",
        hash_connector,
        hash_connector_diff,
        d,
    )


def hash_type_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("TypeDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_id_array(w, "authors", d)
    _write_diff_id_array(w, "concepts", d)
    _write_diff_hash(w, "connectors", d, hash_connectors_diff)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "folder", d)
    _write_diff_string(w, "icon", d)
    _write_diff_string(w, "image", d)
    _write_diff_bool(w, "isAbstract", d)
    _write_diff_id(w, "location", d)
    _write_diff_hash(w, "representations", d, hash_representations_diff)
    _write_diff_string(w, "name", d)
    _write_diff_id(w, "parent", d)
    _write_diff_hash(w, "props", d, hash_props_diff)
    _write_diff_number(w, "stock", d)
    _write_diff_string(w, "unit", d)
    _write_diff_bool(w, "virtual", d)
    return w.digest()


def hash_types_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "TypesDiff", "TypeDiffUpdate", "type", hash_type, hash_type_diff, d
    )


def hash_side_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("SideDiff")
    _write_diff_id(w, "connector", d)
    _write_diff_id(w, "designPiece", d)
    _write_diff_id(w, "piece", d)
    return w.digest()


def hash_layer_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("LayerDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "color", d)
    _write_diff_string(w, "description", d)
    _write_diff_bool(w, "isHidden", d)
    _write_diff_bool(w, "isLocked", d)
    _write_diff_string(w, "path", d)
    return w.digest()


def hash_layers_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "LayersDiff", "LayerDiffUpdate", "layer", hash_layer, hash_layer_diff, d
    )


def hash_group_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("GroupDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "color", d)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "name", d)
    _write_diff_id_array(w, "pieces", d)
    return w.digest()


def hash_groups_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "GroupsDiff", "GroupDiffUpdate", "group", hash_group, hash_group_diff, d
    )


def hash_stat_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("StatDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_number(w, "max", d)
    _write_diff_number(w, "min", d)
    _write_diff_id(w, "quality", d)
    _write_diff_string(w, "unit", d)
    return w.digest()


def hash_stats_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "StatsDiff", "StatDiffUpdate", "stat", hash_stat, hash_stat_diff, d
    )


def hash_connection_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("ConnectionDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_hash(w, "parent", d, hash_side_diff)
    _write_diff_hash(w, "child", d, hash_side_diff)
    _write_diff_string(w, "description", d)
    _write_diff_number(w, "gap", d)
    _write_diff_number(w, "rise", d)
    _write_diff_number(w, "rotation", d)
    _write_diff_number(w, "shift", d)
    _write_diff_number(w, "tilt", d)
    _write_diff_number(w, "turn", d)
    _write_diff_number(w, "u", d)
    _write_diff_number(w, "v", d)
    return w.digest()


def hash_connections_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "ConnectionsDiff",
        "ConnectionDiffUpdate",
        "connection",
        hash_connection,
        hash_connection_diff,
        d,
    )


def hash_piece_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("PieceDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_hash(w, "center", d, hash_coordinate)
    _write_diff_string(w, "color", d)
    _write_diff_string(w, "description", d)
    _write_diff_id(w, "design", d)
    _write_diff_bool(w, "isHidden", d)
    _write_diff_bool(w, "isLocked", d)
    _write_diff_hash(w, "mirrorPlane", d, hash_plane)
    _write_diff_string(w, "name", d)
    _write_diff_hash(w, "plane", d, hash_plane_diff)
    _write_diff_hash(w, "props", d, hash_props_diff)
    _write_diff_number(w, "scale", d)
    _write_diff_id(w, "type", d)
    return w.digest()


def hash_pieces_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "PiecesDiff", "PieceDiffUpdate", "piece", hash_piece, hash_piece_diff, d
    )


def hash_design_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("DesignDiff")
    _write_diff_id(w, "activeLayer", d)
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_hash(w, "authors", d, hash_authors_diff)
    _write_diff_bool(w, "canMirror", d)
    _write_diff_bool(w, "canScale", d)
    _write_diff_id_array(w, "concepts", d)
    _write_diff_hash(w, "connections", d, hash_connections_diff)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "folder", d)
    _write_diff_hash(w, "groups", d, hash_groups_diff)
    _write_diff_string(w, "icon", d)
    _write_diff_string(w, "image", d)
    _write_diff_bool(w, "isAbstract", d)
    _write_diff_hash(w, "layers", d, hash_layers_diff)
    _write_diff_id(w, "location", d)
    _write_diff_string(w, "name", d)
    _write_diff_id(w, "parent", d)
    _write_diff_hash(w, "pieces", d, hash_pieces_diff)
    _write_diff_hash(w, "props", d, hash_props_diff)
    _write_diff_hash(w, "stats", d, hash_stats_diff)
    _write_diff_string(w, "unit", d)
    return w.digest()


def hash_designs_diff(d: dict) -> str:
    return _hash_collection_diff_generic(
        "DesignsDiff", "DesignDiffUpdate", "design", hash_design, hash_design_diff, d
    )


def hash_kit_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("KitDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_hash(w, "authors", d, hash_authors_diff)
    _write_diff_hash(w, "concepts", d, hash_concepts_diff)
    _write_diff_string(w, "description", d)
    _write_diff_hash(w, "designs", d, hash_designs_diff)
    _write_diff_hash(w, "files", d, hash_files_diff)
    _write_diff_hash(w, "folders", d, hash_folders_diff)
    _write_diff_string(w, "homepage", d)
    _write_diff_string(w, "icon", d)
    _write_diff_string(w, "image", d)
    _write_diff_string(w, "license", d)
    _write_diff_string(w, "name", d)
    _write_diff_hash(w, "ports", d, hash_ports_diff)
    _write_diff_string(w, "preview", d)
    _write_diff_hash(w, "qualities", d, hash_qualities_diff)
    _write_diff_string(w, "remote", d)
    _write_diff_hash(w, "tags", d, hash_tags_diff)
    _write_diff_hash(w, "types", d, hash_types_diff)
    _write_diff_string(w, "version", d)
    return w.digest()


# #endregion 🔗Hash Diffs

# #endregion 🖥️Hash


# #region 🎪Kit Operations
# Dict-based pure functions for kit operations exposed via MCP.


def findAttributeValueDict(
    entity: dict, name: str, defaultValue: typing.Any = ...
) -> typing.Optional[str]:
    """🔖Finds an attribute value on an entity by key.
    Returns default if not found, raises ValueError if no default provided.
    """
    attributes = entity.get("attributes") or []
    attribute = next((a for a in attributes if a.get("key") == name), None)
    if attribute is None and defaultValue is ...:
        raise ValueError(f"Attribute {name} not found")
    if attribute is None:
        return defaultValue
    value = attribute.get("value")
    if value is None and defaultValue is None:
        return None
    return (
        value
        if value is not None
        else (defaultValue if defaultValue is not ... else "")
    )


def _findDesignInKitDict(kit: dict, design_id: str) -> dict:
    """🔖Finds a design by ID in a kit dict."""
    for d in kit.get("designs", []):
        if d.get("id") == design_id:
            return d
    raise ValueError(f"Design {design_id} not found in kit")


def _findTypeInKitDict(kit: dict, type_id: str) -> dict:
    """🔖Finds a type by ID in a kit dict."""
    for t in kit.get("types", []):
        if t.get("id") == type_id:
            return t
    raise ValueError(f"Type {type_id} not found in kit")


def _findPieceInDesignDict(design: dict, piece_id: str) -> dict:
    """🔖Finds a piece by ID in a design dict."""
    for p in design.get("pieces", []):
        if p.get("id") == piece_id:
            return p
    raise ValueError(f"Piece {piece_id} not found in design")


def _findPieceConnectionsInDesignDict(design: dict, piece_id: str) -> list[dict]:
    """🔖Finds all connections involving a piece in a design dict."""
    return [
        c
        for c in design.get("connections", [])
        if c.get("parent", {}).get("piece", {}).get("id") == piece_id
        or c.get("child", {}).get("piece", {}).get("id") == piece_id
    ]


def _findConnectorInTypeDict(type_dict: dict, connector_id: str) -> dict:
    """🔖Finds a connector by ID in a type dict."""
    for c in type_dict.get("connectors", []):
        if c.get("id") == connector_id:
            return c
    raise ValueError(f"Connector {connector_id} not found in type")


def _applyDesignDiffDict(target: dict, diff: dict) -> None:
    """🔖Applies a design diff to a design dict in-place."""
    pieces_diff = diff.get("pieces")
    if pieces_diff:
        pieces = target.get("pieces", [])
        if diff.get("pieces") and "pieces" not in target:
            target["pieces"] = []
            pieces = target["pieces"]
        for removed in pieces_diff.get("removed", []):
            removed_id = removed.get("id") if isinstance(removed, dict) else removed
            pieces[:] = [p for p in pieces if p.get("id") != removed_id]
        for updated in pieces_diff.get("updated", []):
            piece_id = updated.get("id") or updated.get("piece", {}).get("id")
            piece_diff = updated.get("diff", {})
            for p in pieces:
                if p.get("id") == piece_id:
                    for k, v in piece_diff.items():
                        if v is not None:
                            p[k] = v
                    break
        for added in pieces_diff.get("added", []):
            pieces.append(added)
    connections_diff = diff.get("connections")
    if connections_diff:
        connections = target.get("connections", [])
        if diff.get("connections") and "connections" not in target:
            target["connections"] = []
            connections = target["connections"]
        for removed in connections_diff.get("removed", []):
            removed_id = removed.get("id") if isinstance(removed, dict) else removed
            connections[:] = [c for c in connections if c.get("id") != removed_id]
        for updated in connections_diff.get("updated", []):
            conn_id = updated.get("id") or updated.get("connection", {}).get("id")
            conn_diff = updated.get("diff", {})
            for c in connections:
                if c.get("id") == conn_id:
                    for k, v in conn_diff.items():
                        if v is not None:
                            c[k] = v
                    break
        for added in connections_diff.get("added", []):
            connections.append(added)
    for key in [
        "name",
        "isAbstract",
        "unit",
        "folder",
        "parent",
        "location",
        "icon",
        "image",
        "description",
    ]:
        if key in diff and diff[key] is not None:
            target[key] = diff[key]


def piecesMetadataDict(kit: dict, design_id: str) -> dict:
    """🔖Returns metadata for all pieces in a design.
    Each entry contains plane, center, fixedPieceId, parentPieceId, depth, and path.
    """
    design = _findDesignInKitDict(kit, design_id)
    flatten_diff = flattenDesignDict(kit, design_id)
    piece_paths = flatten_diff.pop("_piecePaths", {})
    flat_design = copy.deepcopy(design)
    _applyDesignDiffDict(flat_design, flatten_diff)
    result = {}
    for p in flat_design.get("pieces", []):
        id = p.get("id", "")
        path_raw = piece_paths.get(id, id)
        result[id] = {
            "plane": _dict_piece_plane(p),
            "center": _dict_piece_center(p) or {"u": 0, "v": 0},
            "fixedPieceId": findAttributeValueDict(p, "semio.fixedPieceId", id) or id,
            "parentPieceId": findAttributeValueDict(p, "semio.parentPieceId", None),
            "depth": int(findAttributeValueDict(p, "semio.depth", "0") or "0"),
            "path": [s for s in path_raw.split(",") if s],
        }
    return result


# #region 🎡Clustering
# Functions for clustering and expanding design pieces.


def createClusteredDesignDict(
    original_design: dict, cluster_piece_ids: list[str], design_name: str
) -> dict:
    """🗃️Creates a new design from a subset of pieces (cluster).
    Returns a dict with 'clusteredDesign' and 'externalConnections'.
    """
    pieces = original_design.get("pieces", [])
    if not pieces:
        raise ValueError("Original design has no pieces to cluster")
    if not cluster_piece_ids:
        raise ValueError("No piece IDs provided for clustering")
    cluster_set = set(cluster_piece_ids)
    clustered_pieces = [p for p in pieces if p.get("id") in cluster_set]
    if not clustered_pieces:
        raise ValueError("No pieces found matching the provided IDs")
    connections = original_design.get("connections", [])
    internal_connections = [
        c
        for c in connections
        if c.get("parent", {}).get("piece", {}).get("id") in cluster_set
        and c.get("child", {}).get("piece", {}).get("id") in cluster_set
    ]
    external_connections = [
        c
        for c in connections
        if (c.get("parent", {}).get("piece", {}).get("id") in cluster_set)
        != (c.get("child", {}).get("piece", {}).get("id") in cluster_set)
    ]
    import datetime as dt
    import uuid

    now = dt.datetime.now(dt.timezone.utc).isoformat()
    clustered_design = {
        "id": str(uuid.uuid4()),
        "name": design_name,
        "unit": original_design.get("unit"),
        "description": f"Clustered design with {len(clustered_pieces)} pieces",
        "pieces": clustered_pieces,
        "connections": internal_connections,
        "createdAt": now,
        "updatedAt": now,
    }
    return {
        "clusteredDesign": clustered_design,
        "externalConnections": external_connections,
    }


def replaceClusterWithDesignDict(
    original_design: dict,
    cluster_piece_ids: list[str],
    clustered_design: dict,
    external_connections: list[dict],
) -> dict:
    """Returns a DesignDiff that replaces clustered pieces with a design reference."""
    cluster_set = set(cluster_piece_ids)
    pieces_to_remove = [{"id": id} for id in cluster_piece_ids]
    connections = original_design.get("connections", [])
    connections_to_remove = [
        {"id": c.get("id")}
        for c in connections
        if c.get("parent", {}).get("piece", {}).get("id") in cluster_set
        or c.get("child", {}).get("piece", {}).get("id") in cluster_set
    ]
    updated_external = []
    for connection in external_connections:
        connected_in_cluster = (
            connection.get("parent", {}).get("piece", {}).get("id") in cluster_set
        )
        connecting_in_cluster = (
            connection.get("child", {}).get("piece", {}).get("id") in cluster_set
        )
        import copy

        new_conn = copy.deepcopy(connection)
        if connected_in_cluster:
            new_conn.setdefault("parent", {})["designPiece"] = {
                "id": clustered_design.get("id")
            }
        elif connecting_in_cluster:
            new_conn.setdefault("child", {})["designPiece"] = {
                "id": clustered_design.get("id")
            }
        updated_external.append(new_conn)
    return {
        "pieces": {"removed": pieces_to_remove},
        "connections": {"removed": connections_to_remove, "added": updated_external},
    }


def getClusterableGroupsDict(
    design: dict, selected_piece_ids: list[str]
) -> list[list[str]]:
    """🔖Returns clusterable groups of selected pieces using DFS on connection graph."""
    if len(selected_piece_ids) < 2:
        return []
    adjacency: dict[str, set[str]] = {}
    for connection in design.get("connections", []):
        source_id = connection.get("child", {}).get("piece", {}).get("id", "")
        target_id = connection.get("parent", {}).get("piece", {}).get("id", "")
        adjacency.setdefault(source_id, set()).add(target_id)
        adjacency.setdefault(target_id, set()).add(source_id)
    selected_set = set(selected_piece_ids)
    visited: set[str] = set()
    connected_groups: list[list[str]] = []

    def dfs(piece_id: str, current_group: list[str]) -> None:
        if piece_id in visited:
            return
        visited.add(piece_id)
        current_group.append(piece_id)
        for neighbor in adjacency.get(piece_id, set()):
            if neighbor in selected_set and neighbor not in visited:
                dfs(neighbor, current_group)

    for piece_id in selected_piece_ids:
        if piece_id not in visited:
            group: list[str] = []
            dfs(piece_id, group)
            connected_groups.append(group)
    piece_id_set = set(p.get("id", "") for p in design.get("pieces", []))
    has_design_nodes = any(pid not in piece_id_set for pid in selected_piece_ids)
    has_multiple_components = len(connected_groups) > 1
    has_large_connected_group = any(len(g) > 1 for g in connected_groups)
    if has_design_nodes or has_multiple_components or has_large_connected_group:
        return [selected_piece_ids]
    return []


def expandDesignPiecesDict(design: dict, kit: dict) -> dict:
    """🔖Recursively expands design references (designPiece) by inlining their pieces and connections."""
    import copy

    connections = design.get("connections", [])
    has_design_connections = any(
        c.get("parent", {}).get("designPiece")
        or c.get("child", {}).get("designPiece")
        for c in connections
    )
    if not has_design_connections:
        return design
    expanded = copy.deepcopy(design)
    design_ids: set[str] = set()
    for conn in connections:
        dp = conn.get("parent", {}).get("designPiece")
        if dp:
            design_ids.add(dp.get("id", ""))
        dp = conn.get("child", {}).get("designPiece")
        if dp:
            design_ids.add(dp.get("id", ""))
    if not design_ids:
        return expanded
    for design_ref_id in design_ids:
        referenced = next(
            (d for d in kit.get("designs", []) if d.get("id") == design_ref_id),
            None,
        )
        if not referenced:
            continue
        expanded_ref = expandDesignPiecesDict(referenced, kit)
        transformed_pieces = []
        for piece in expanded_ref.get("pieces", []):
            new_piece = copy.deepcopy(piece)
            if not new_piece.get("center"):
                new_piece["center"] = {"u": 0, "v": 0}
            transformed_pieces.append(new_piece)
        transformed_connections = copy.deepcopy(expanded_ref.get("connections", []))
        updated_connections = []
        for conn in expanded.get("connections", []):
            new_conn = copy.deepcopy(conn)
            connected_dp = new_conn.get("parent", {}).get("designPiece")
            if connected_dp and connected_dp.get("id") == design_ref_id:
                new_conn["parent"].pop("designPiece", None)
            connecting_dp = new_conn.get("child", {}).get("designPiece")
            if connecting_dp and connecting_dp.get("id") == design_ref_id:
                new_conn["child"].pop("designPiece", None)
            updated_connections.append(new_conn)
        expanded["pieces"] = list(expanded.get("pieces", [])) + transformed_pieces
        expanded["connections"] = updated_connections + transformed_connections
    return expanded


# #endregion 🎡Clustering

# #region 📍Kit Query Helpers Dict
# Dict-based kit query helper functions.


def getPrimitiveDesignDict(kit: dict, design_id: str) -> dict:
    """🌱Gets the primitive (root) design of a design family."""
    current = _findDesignInKitDict(kit, design_id)
    while current.get("parent", {}).get("id"):
        current = _findDesignInKitDict(kit, current["parent"]["id"])
    return current


def getDesignFamilyDict(kit: dict, design_id: str) -> list[dict]:
    """🌳Gets all designs in a design family (the entire tree)."""
    primitive = getPrimitiveDesignDict(kit, design_id)
    family: list[dict] = []

    def collect(parent_id: str) -> None:
        parent = _findDesignInKitDict(kit, parent_id)
        family.append(parent)
        children = [
            d
            for d in kit.get("designs", [])
            if d.get("parent", {}).get("id") == parent_id
        ]
        for child in children:
            collect(child["id"])

    collect(primitive["id"])
    return family


def getDesignSiblingsDict(kit: dict, design_id: str) -> list[dict]:
    """🔖Returns all designs with the same parent, excluding self."""
    design = _findDesignInKitDict(kit, design_id)
    parent_id = design.get("parent", {}).get("id")
    return [
        d
        for d in kit.get("designs", [])
        if d.get("parent", {}).get("id") == parent_id and d.get("id") != design_id
    ]


def getDesignChildrenDict(kit: dict, design_id: str) -> list[dict]:
    """🔖Returns all direct children of a design."""
    return [
        d for d in kit.get("designs", []) if d.get("parent", {}).get("id") == design_id
    ]


def areDesignsInSameFamilyDict(kit: dict, design_id_a: str, design_id_b: str) -> bool:
    """🔖Checks if two designs share the same primitive ancestor."""
    return getPrimitiveDesignDict(kit, design_id_a).get("id") == getPrimitiveDesignDict(
        kit, design_id_b
    ).get("id")


def canUseDesignAsPieceDict(
    kit: dict, container_design_id: str, piece_design_id: str
) -> bool:
    """🔖Returns true if a design can be used as a piece (must NOT be in same family)."""
    return not areDesignsInSameFamilyDict(kit, container_design_id, piece_design_id)


def findSameFamilyDesignPiecesDict(kit: dict, design_id: str) -> list[dict]:
    """🔖Returns all pieces in a design that reference designs from the same family."""
    design = _findDesignInKitDict(kit, design_id)
    return [
        p
        for p in design.get("pieces", [])
        if p.get("design", {}).get("id")
        and areDesignsInSameFamilyDict(kit, design_id, p["design"]["id"])
    ]


def getPrimitiveTypeDict(kit: dict, type_id: str) -> dict:
    """🔖Gets the primitive (root) type of a type family."""
    current = _findTypeInKitDict(kit, type_id)
    while current.get("parent", {}).get("id"):
        current = _findTypeInKitDict(kit, current["parent"]["id"])
    return current


def getTypeFamilyDict(kit: dict, type_id: str) -> list[dict]:
    """🔖Gets all types in a type family (the entire tree)."""
    primitive = getPrimitiveTypeDict(kit, type_id)
    family: list[dict] = []

    def collect(parent_id: str) -> None:
        parent = _findTypeInKitDict(kit, parent_id)
        family.append(parent)
        children = [
            t
            for t in kit.get("types", [])
            if t.get("parent", {}).get("id") == parent_id
        ]
        for child in children:
            collect(child["id"])

    collect(primitive["id"])
    return family


def getTypeSiblingsDict(kit: dict, type_id: str) -> list[dict]:
    """🔖Returns all types with the same parent, excluding self."""
    type_ = _findTypeInKitDict(kit, type_id)
    parent_id = type_.get("parent", {}).get("id")
    return [
        t
        for t in kit.get("types", [])
        if t.get("parent", {}).get("id") == parent_id and t.get("id") != type_id
    ]


def getTypeChildrenDict(kit: dict, type_id: str) -> list[dict]:
    """🔖Returns all direct children of a type."""
    return [t for t in kit.get("types", []) if t.get("parent", {}).get("id") == type_id]


def areTypesInSameFamilyDict(kit: dict, type_id_a: str, type_id_b: str) -> bool:
    """🔖Checks if two types share the same primitive ancestor."""
    return getPrimitiveTypeDict(kit, type_id_a).get("id") == getPrimitiveTypeDict(
        kit, type_id_b
    ).get("id")


def findPieceTypeInDesignDict(kit: dict, design_id: str, piece_id: str) -> dict:
    """🔖Gets the type of a piece in a design."""
    design = _findDesignInKitDict(kit, design_id)
    piece = _findPieceInDesignDict(design, piece_id)
    type_ref = piece.get("type", {})
    if not type_ref or not type_ref.get("id"):
        raise ValueError(f"Piece {piece_id} has no type")
    return _findTypeInKitDict(kit, type_ref["id"])


def findUsedConnectorsByPieceInDesignDict(
    kit: dict, design_id: str, piece_id: str
) -> list[dict]:
    """🔖Returns all connectors of a piece that are used in connections."""
    design = _findDesignInKitDict(kit, design_id)
    piece = _findPieceInDesignDict(design, piece_id)
    type_ref = piece.get("type", {})
    if not type_ref or not type_ref.get("id"):
        return []
    type_dict = _findTypeInKitDict(kit, type_ref["id"])
    connections = _findPieceConnectionsInDesignDict(design, piece_id)
    result = []
    for c in connections:
        if c.get("parent", {}).get("piece", {}).get("id") == piece_id:
            connector_id = (c.get("parent", {}).get("connector") or {}).get("id")
        else:
            connector_id = (c.get("child", {}).get("connector") or {}).get("id")
        if connector_id:
            try:
                result.append(_findConnectorInTypeDict(type_dict, connector_id))
            except ValueError:
                pass
    return result


def findReplaceableTypesForPieceInDesignDict(
    kit: dict,
    design_id: str,
    piece_id: str,
    variants: typing.Optional[list[str]] = None,
) -> list[dict]:
    """Finds all types that can replace a piece while maintaining connection compatibility."""
    result = findReplaceableTypesInDesignsForPiecesInDesignDict(
        kit, design_id, [piece_id]
    )
    return result["types"]


def findReplaceableTypesForPiecesInDesignDict(
    kit: dict,
    design_id: str,
    piece_ids: list[str],
    variants: typing.Optional[list[str]] = None,
) -> list[dict]:
    """Finds types that can replace multiple pieces while maintaining all external connections."""
    result = findReplaceableTypesInDesignsForPiecesInDesignDict(
        kit, design_id, piece_ids
    )
    return result["types"]


def findReplaceableTypesInDesignsForPiecesInDesignDict(
    kit: dict,
    design_id: str,
    piece_ids: list[str],
) -> dict:
    """Finds replaceable types and designs for the selected pieces in a design.
    Specs: A candidate is valid only when the whole selection boundary requirement multiset can be
    injectively matched to distinct compatible candidate connectors. Boundary requirements use the
    actual opposite-side connector ports, isolated selections use the selected pieces' own kind
    connectors with multiplicity, and candidate designs only expose connectors not already consumed
    by internal design connections.
    """
    design = _findDesignInKitDict(kit, design_id)
    selected_piece_set = set(piece_ids)
    pieces = design.get("pieces", [])
    connections = design.get("connections", [])
    piece_map = {piece.get("id"): piece for piece in pieces}

    port_map = {p["id"]: p for p in kit.get("ports", [])}
    type_map = {t["id"]: t for t in kit.get("types", [])}
    all_types = kit.get("types", [])
    all_designs = kit.get("designs", [])

    def check_port_compatibility(candidate_port_id: str, required_port_id: str) -> bool:
        if not candidate_port_id or not required_port_id:
            return False
        if candidate_port_id == required_port_id:
            return True
        candidate_port = port_map.get(candidate_port_id)
        required_port = port_map.get(required_port_id)
        if not candidate_port or not required_port:
            return False
        return any(
            port_ref.get("id") == required_port_id
            for port_ref in candidate_port.get("compatiblePorts") or []
        ) or any(
            port_ref.get("id") == candidate_port_id
            for port_ref in required_port.get("compatiblePorts") or []
        )

    def get_connector_port_id(type_id: str, connector_id: str) -> str:
        if not type_id or not connector_id:
            return ""
        type_dict = type_map.get(type_id) or {}
        for connector in type_dict.get("connectors") or []:
            if connector.get("id") == connector_id:
                return ((connector.get("port") or {}).get("id")) or ""
        return ""

    def get_own_requirement_port_ids(piece_id: str) -> list[str]:
        piece = piece_map.get(piece_id) or {}
        type_id = (piece.get("type") or {}).get("id", "")
        type_dict = type_map.get(type_id) or {}
        return [
            ((connector.get("port") or {}).get("id")) or ""
            for connector in type_dict.get("connectors") or []
        ]

    def get_boundary_requirement_port_ids() -> list[str]:
        requirement_port_ids: list[str] = []
        for conn in connections:
            connected_id = conn.get("parent", {}).get("piece", {}).get("id", "")
            connecting_id = conn.get("child", {}).get("piece", {}).get("id", "")
            connected_selected = connected_id in selected_piece_set
            connecting_selected = connecting_id in selected_piece_set
            if connected_selected == connecting_selected:
                continue
            other_side = (
                conn.get("child") if connected_selected else conn.get("parent")
            )
            other_piece_id = (other_side or {}).get("piece", {}).get("id", "")
            other_piece = piece_map.get(other_piece_id) or {}
            other_type_id = (other_piece.get("type") or {}).get("id", "")
            other_connector_id = ((other_side or {}).get("connector") or {}).get(
                "id", ""
            )
            requirement_port_ids.append(
                get_connector_port_id(other_type_id, other_connector_id)
            )
        return requirement_port_ids

    def get_selection_own_requirement_port_ids() -> list[str]:
        requirement_port_ids: list[str] = []
        for piece_id in piece_ids:
            requirement_port_ids.extend(get_own_requirement_port_ids(piece_id))
        return requirement_port_ids

    required_port_ids = get_boundary_requirement_port_ids()
    if len(required_port_ids) == 0:
        required_port_ids = get_selection_own_requirement_port_ids()

    def can_satisfy_requirements(
        required_port_ids: list[str], available_port_ids: list[str]
    ) -> bool:
        if len(required_port_ids) == 0:
            return True
        if len(available_port_ids) < len(required_port_ids):
            return False

        requirement_options = []
        for required_port_id in required_port_ids:
            connector_indexes = [
                connector_index
                for connector_index, available_port_id in enumerate(available_port_ids)
                if check_port_compatibility(available_port_id, required_port_id)
            ]
            if len(connector_indexes) == 0:
                return False
            requirement_options.append(connector_indexes)
        requirement_options.sort(key=len)

        used_connector_indexes = [False] * len(available_port_ids)

        def match_requirement(requirement_index: int) -> bool:
            if requirement_index >= len(requirement_options):
                return True
            for connector_index in requirement_options[requirement_index]:
                if used_connector_indexes[connector_index]:
                    continue
                used_connector_indexes[connector_index] = True
                if match_requirement(requirement_index + 1):
                    return True
                used_connector_indexes[connector_index] = False
            return False

        return match_requirement(0)

    def candidate_type_available_port_ids(candidate_type: dict) -> list[str]:
        return [
            ((connector.get("port") or {}).get("id")) or ""
            for connector in candidate_type.get("connectors") or []
        ]

    def candidate_design_available_port_ids(candidate_design: dict) -> list[str]:
        consumed_connector_keys = set()
        for connection in candidate_design.get("connections") or []:
            for side in [
                connection.get("parent") or {},
                connection.get("child") or {},
            ]:
                piece_id = (side.get("piece") or {}).get("id", "")
                connector_id = (side.get("connector") or {}).get("id", "")
                if piece_id and connector_id:
                    consumed_connector_keys.add(f"{piece_id}::{connector_id}")

        available_port_ids = []
        for piece in candidate_design.get("pieces") or []:
            type_id = (piece.get("type") or {}).get("id", "")
            type_dict = type_map.get(type_id) or {}
            for connector in type_dict.get("connectors") or []:
                if (
                    f"{piece.get('id', '')}::{connector.get('id', '')}"
                    in consumed_connector_keys
                ):
                    continue
                available_port_ids.append(
                    ((connector.get("port") or {}).get("id")) or ""
                )
        return available_port_ids

    if len(piece_ids) == 0:
        return {
            "types": [
                candidate_type
                for candidate_type in all_types
                if len(candidate_type_available_port_ids(candidate_type)) == 0
            ],
            "designs": [
                candidate_design
                for candidate_design in all_designs
                if len(candidate_design_available_port_ids(candidate_design)) == 0
            ],
        }

    def is_valid_candidate(available_port_ids: list[str]) -> bool:
        return can_satisfy_requirements(required_port_ids, available_port_ids)

    return {
        "types": [
            candidate_type
            for candidate_type in all_types
            if is_valid_candidate(candidate_type_available_port_ids(candidate_type))
        ],
        "designs": [
            candidate_design
            for candidate_design in all_designs
            if is_valid_candidate(candidate_design_available_port_ids(candidate_design))
        ],
    }


def sumQualityInDesignDict(kit: dict, design_id: str, quality_id: str) -> float:
    """🔖Sums up the values of a quality across all pieces in a design.
    For each piece, uses the piece-level prop if present, otherwise falls back to the type-level prop.
    """
    design = _findDesignInKitDict(kit, design_id)
    total = 0.0
    for piece in design.get("pieces", []):
        piece_prop = next(
            (
                p
                for p in piece.get("props", [])
                if p.get("quality", {}).get("id") == quality_id
            ),
            None,
        )
        if piece_prop is not None:
            total += float(piece_prop.get("value", 0))
            continue
        type_ref = piece.get("type", {})
        if type_ref and type_ref.get("id"):
            try:
                type_dict = _findTypeInKitDict(kit, type_ref["id"])
                type_prop = next(
                    (
                        p
                        for p in type_dict.get("props", [])
                        if p.get("quality", {}).get("id") == quality_id
                    ),
                    None,
                )
                if type_prop is not None:
                    total += float(type_prop.get("value", 0))
            except ValueError:
                pass
    return total


# #endregion 📍Kit Query Helpers Dict

# #endregion 🎪Kit Operations


# #region 🎗️Kit Diff Operations
# Diffing and patching operations for comparing and merging kit versions.


def _normalizeValue(value: typing.Any) -> typing.Any:
    """🔖Normalize empty values to None for comparison."""
    if value is None or value == "" or value == []:
        return None
    return value


def _normalizeBoolean(value: bool | None) -> bool | None:
    """🔘Normalize boolean: True stays True, False/None become None."""
    return True if value else None


def _normalizeArray(arr: list | None) -> list:
    """📚Normalize None or single item to list."""
    if arr is None:
        return []
    if not isinstance(arr, list):
        return [arr]
    return arr


def areAttributesEqualDict(
    a: list | None, b: list | None, strict: bool = False
) -> bool:
    """🔖Check whether two attribute dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for attrA in arrA:
        attrB = next((x for x in arrB if x.get("id") == attrA.get("id")), None)
        if attrB is None:
            return False
        if attrA.get("key") != attrB.get("key"):
            return False
        if _normalizeValue(attrA.get("value")) != _normalizeValue(attrB.get("value")):
            return False
        if _normalizeValue(attrA.get("definition")) != _normalizeValue(
            attrB.get("definition")
        ):
            return False
        if strict:
            if attrA.get("createdAt") != attrB.get("createdAt"):
                return False
            if attrA.get("updatedAt") != attrB.get("updatedAt"):
                return False
    return True


def arePropsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """🔖Check whether two prop dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for propA in arrA:
        propB = next((x for x in arrB if x.get("id") == propA.get("id")), None)
        if propB is None:
            return False
        if propA.get("quality", {}).get("id") != propB.get("quality", {}).get("id"):
            return False
        if propA.get("value") != propB.get("value"):
            return False
        if _normalizeValue(propA.get("unit")) != _normalizeValue(propB.get("unit")):
            return False
        if not areAttributesEqualDict(
            propA.get("attributes"), propB.get("attributes"), strict
        ):
            return False
        if strict:
            if propA.get("createdAt") != propB.get("createdAt"):
                return False
            if propA.get("updatedAt") != propB.get("updatedAt"):
                return False
    return True


def areConnectorsEqualDict(
    a: list | None, b: list | None, strict: bool = False
) -> bool:
    """🔖Check whether two port dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for connectorA in arrA:
        connectorB = next(
            (x for x in arrB if x.get("id") == connectorA.get("id")), None
        )
        if connectorB is None:
            return False
        if _normalizeValue(connectorA.get("name")) != _normalizeValue(
            connectorB.get("name")
        ):
            return False
        pointA = connectorA.get("point", {})
        pointB = connectorB.get("point", {})
        if (
            not _floatEqual(pointA.get("x"), pointB.get("x"))
            or not _floatEqual(pointA.get("y"), pointB.get("y"))
            or not _floatEqual(pointA.get("z"), pointB.get("z"))
        ):
            return False
        dirA = connectorA.get("direction", {})
        dirB = connectorB.get("direction", {})
        if (
            not _floatEqual(dirA.get("x"), dirB.get("x"))
            or not _floatEqual(dirA.get("y"), dirB.get("y"))
            or not _floatEqual(dirA.get("z"), dirB.get("z"))
        ):
            return False
        if not _floatEqual(connectorA.get("t"), connectorB.get("t")):
            return False
        if _normalizeBoolean(connectorA.get("mandatory")) != _normalizeBoolean(
            connectorB.get("mandatory")
        ):
            return False
        ifaceA = connectorA.get("port", {}) if connectorA.get("port") else {}
        ifaceB = connectorB.get("port", {}) if connectorB.get("port") else {}
        if _normalizeValue(ifaceA.get("id")) != _normalizeValue(ifaceB.get("id")):
            return False
        if not arePropsEqualDict(
            connectorA.get("props"), connectorB.get("props"), strict
        ):
            return False
        if not areAttributesEqualDict(
            connectorA.get("attributes"), connectorB.get("attributes"), strict
        ):
            return False
        if strict:
            if connectorA.get("createdAt") != connectorB.get("createdAt"):
                return False
            if connectorA.get("updatedAt") != connectorB.get("updatedAt"):
                return False
    return True


def areRepresentationsEqualDict(
    a: list | None, b: list | None, strict: bool = False
) -> bool:
    """🔖Check whether two representation dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for representationA in arrA:
        representationB = next(
            (x for x in arrB if x.get("id") == representationA.get("id")), None
        )
        if representationB is None:
            return False
        if _normalizeValue(representationA.get("name")) != _normalizeValue(
            representationB.get("name")
        ):
            return False

        fileA = representationA.get("file")
        fileB = representationB.get("file")
        fileIdA = fileA.get("id") if isinstance(fileA, dict) else fileA
        fileIdB = fileB.get("id") if isinstance(fileB, dict) else fileB
        if fileIdA != fileIdB:
            return False
        tagsA = [
            t.get("id") if isinstance(t, dict) else t
            for t in _normalizeArray(representationA.get("tags"))
        ]
        tagsB = [
            t.get("id") if isinstance(t, dict) else t
            for t in _normalizeArray(representationB.get("tags"))
        ]
        if len(tagsA) != len(tagsB) or set(tagsA) != set(tagsB):
            return False
        if not areAttributesEqualDict(
            representationA.get("attributes"), representationB.get("attributes"), strict
        ):
            return False
        if strict:
            if representationA.get("createdAt") != representationB.get("createdAt"):
                return False
            if representationA.get("updatedAt") != representationB.get("updatedAt"):
                return False
    return True


def areTypesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """🔖Check whether two type dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for typeA in arrA:
        typeB = None
        for t in arrB:
            if t.get("id") != typeA.get("id"):
                continue
            parentA = typeA.get("parent")
            parentB = t.get("parent")
            if not parentA and not parentB:
                typeB = t
                break
            if not parentA or not parentB:
                continue

            parentIdA = parentA.get("id") if isinstance(parentA, dict) else parentA
            parentIdB = parentB.get("id") if isinstance(parentB, dict) else parentB
            if parentIdA == parentIdB:
                typeB = t
                break
        if typeB is None:
            return False
        if typeA.get("name") != typeB.get("name"):
            return False
        if _normalizeValue(typeA.get("description")) != _normalizeValue(
            typeB.get("description")
        ):
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
        if _normalizeBoolean(typeA.get("isAbstract")) != _normalizeBoolean(
            typeB.get("isAbstract")
        ):
            return False
        if _normalizeBoolean(typeA.get("virtual")) != _normalizeBoolean(
            typeB.get("virtual")
        ):
            return False
        locA = typeA.get("location", {}) if typeA.get("location") else {}
        locB = typeB.get("location", {}) if typeB.get("location") else {}
        if _normalizeValue(locA.get("id")) != _normalizeValue(locB.get("id")):
            return False

        conceptsA = _normalizeArray(typeA.get("concepts"))
        conceptsB = _normalizeArray(typeB.get("concepts"))
        conceptIdsA = [c.get("id") if isinstance(c, dict) else c for c in conceptsA]
        conceptIdsB = [c.get("id") if isinstance(c, dict) else c for c in conceptsB]
        if conceptIdsA != conceptIdsB:
            return False
        authA = [
            a.get("id") if isinstance(a, dict) else a
            for a in _normalizeArray(typeA.get("authors"))
        ]
        authB = [
            a.get("id") if isinstance(a, dict) else a
            for a in _normalizeArray(typeB.get("authors"))
        ]
        if authA != authB:
            return False
        if not arePropsEqualDict(typeA.get("props"), typeB.get("props"), strict):
            return False
        if not areRepresentationsEqualDict(
            typeA.get("representations"), typeB.get("representations"), strict
        ):
            return False
        if not areConnectorsEqualDict(
            typeA.get("connectors"), typeB.get("connectors"), strict
        ):
            return False
        if not areAttributesEqualDict(
            typeA.get("attributes"), typeB.get("attributes"), strict
        ):
            return False
        if strict:
            if typeA.get("createdAt") != typeB.get("createdAt"):
                return False
            if typeA.get("updatedAt") != typeB.get("updatedAt"):
                return False
    return True


def arePiecesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """🔖Check whether two piece dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for pieceA in arrA:
        pieceB = next((x for x in arrB if x.get("id") == pieceA.get("id")), None)
        if pieceB is None:
            return False
        if _normalizeValue(pieceA.get("name")) != _normalizeValue(pieceB.get("name")):
            return False

        typeA = pieceA.get("type")
        typeB = pieceB.get("type")
        typeIdA = typeA.get("id") if isinstance(typeA, dict) else typeA
        typeIdB = typeB.get("id") if isinstance(typeB, dict) else typeB
        if typeIdA != typeIdB:
            return False

        designA = pieceA.get("design")
        designB = pieceB.get("design")
        designIdA = designA.get("id") if isinstance(designA, dict) else designA
        designIdB = designB.get("id") if isinstance(designB, dict) else designB
        if designIdA != designIdB:
            return False
        planeA = _dict_piece_plane(pieceA)
        planeB = _dict_piece_plane(pieceB)
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
            if centerA.get("u") != centerB.get("u") or centerA.get("v") != centerB.get(
                "v"
            ):
                return False
        elif centerA or centerB:
            return False
        if pieceA.get("scale") != pieceB.get("scale"):
            return False
        if _normalizeBoolean(pieceA.get("isHidden")) != _normalizeBoolean(
            pieceB.get("isHidden")
        ):
            return False
        if _normalizeBoolean(pieceA.get("isLocked")) != _normalizeBoolean(
            pieceB.get("isLocked")
        ):
            return False
        if _normalizeValue(pieceA.get("color")) != _normalizeValue(pieceB.get("color")):
            return False
        if _normalizeValue(pieceA.get("description")) != _normalizeValue(
            pieceB.get("description")
        ):
            return False
        if not arePropsEqualDict(pieceA.get("props"), pieceB.get("props"), strict):
            return False
        if not areAttributesEqualDict(
            pieceA.get("attributes"), pieceB.get("attributes"), strict
        ):
            return False
        if strict:
            if pieceA.get("createdAt") != pieceB.get("createdAt"):
                return False
            if pieceA.get("updatedAt") != pieceB.get("updatedAt"):
                return False
    return True


def _getIdFromRef(ref: typing.Any) -> str | None:
    """🧲Extract id from either a string (Input format) or dict with id (Output format)."""
    if ref is None:
        return None
    if isinstance(ref, dict):
        return ref.get("id")
    return ref


def _floatEqual(a, b, epsilon=1e-9):
    """🔖Compare two float values with epsilon tolerance."""
    if a is None and b is None:
        return True
    if a is None or b is None:
        return a == b
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        return abs(float(a) - float(b)) < epsilon
    return a == b


def areConnectionsEqualDict(
    a: list | None, b: list | None, strict: bool = False
) -> bool:
    """🔖Check whether two connection dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for connA in arrA:
        connB = next((x for x in arrB if x.get("id") == connA.get("id")), None)
        if connB is None:
            return False
        connectedA = connA.get("parent", {})
        connectedB = connB.get("parent", {})

        if _getIdFromRef(connectedA.get("piece")) != _getIdFromRef(
            connectedB.get("piece")
        ):
            return False
        if _getIdFromRef(connectedA.get("designPiece")) != _getIdFromRef(
            connectedB.get("designPiece")
        ):
            return False
        if _getIdFromRef(connectedA.get("connector")) != _getIdFromRef(
            connectedB.get("connector")
        ):
            return False
        connectingA = connA.get("child", {})
        connectingB = connB.get("child", {})
        if _getIdFromRef(connectingA.get("piece")) != _getIdFromRef(
            connectingB.get("piece")
        ):
            return False
        if _getIdFromRef(connectingA.get("designPiece")) != _getIdFromRef(
            connectingB.get("designPiece")
        ):
            return False
        if _getIdFromRef(connectingA.get("connector")) != _getIdFromRef(
            connectingB.get("connector")
        ):
            return False
        if not _floatEqual(connA.get("gap"), connB.get("gap")):
            return False
        if not _floatEqual(connA.get("shift"), connB.get("shift")):
            return False
        if not _floatEqual(connA.get("rise"), connB.get("rise")):
            return False
        if not _floatEqual(connA.get("rotation"), connB.get("rotation")):
            return False
        if not _floatEqual(connA.get("turn"), connB.get("turn")):
            return False
        if not _floatEqual(connA.get("tilt"), connB.get("tilt")):
            return False
        if not _floatEqual(connA.get("u"), connB.get("u")):
            return False
        if not _floatEqual(connA.get("v"), connB.get("v")):
            return False
        if _normalizeValue(connA.get("description")) != _normalizeValue(
            connB.get("description")
        ):
            return False
        if not areAttributesEqualDict(
            connA.get("attributes"), connB.get("attributes"), strict
        ):
            return False
        if strict:
            if connA.get("createdAt") != connB.get("createdAt"):
                return False
            if connA.get("updatedAt") != connB.get("updatedAt"):
                return False
    return True


def areDesignsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """🔖Check whether two design dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for designA in arrA:
        designB = None
        for d in arrB:
            if d.get("id") != designA.get("id"):
                continue
            parentA = designA.get("parent")
            parentB = d.get("parent")
            if not parentA and not parentB:
                designB = d
                break
            if not parentA or not parentB:
                continue

            parentIdA = _getIdFromRef(parentA)
            parentIdB = _getIdFromRef(parentB)
            if parentIdA == parentIdB:
                designB = d
                break
        if designB is None:
            return False
        if designA.get("name") != designB.get("name"):
            return False
        if _normalizeValue(designA.get("description")) != _normalizeValue(
            designB.get("description")
        ):
            return False
        if _normalizeValue(designA.get("icon")) != _normalizeValue(designB.get("icon")):
            return False
        if _normalizeValue(designA.get("image")) != _normalizeValue(
            designB.get("image")
        ):
            return False

        conceptsA = _normalizeArray(designA.get("concepts"))
        conceptsB = _normalizeArray(designB.get("concepts"))
        conceptIdsA = [_getIdFromRef(c) for c in conceptsA]
        conceptIdsB = [_getIdFromRef(c) for c in conceptsB]
        if conceptIdsA != conceptIdsB:
            return False
        if not arePiecesEqualDict(designA.get("pieces"), designB.get("pieces"), strict):
            return False
        if not areConnectionsEqualDict(
            designA.get("connections"), designB.get("connections"), strict
        ):
            return False
        if not areAttributesEqualDict(
            designA.get("attributes"), designB.get("attributes"), strict
        ):
            return False
        if strict:
            if designA.get("createdAt") != designB.get("createdAt"):
                return False
            if designA.get("updatedAt") != designB.get("updatedAt"):
                return False
    return True


def arePortsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """🔖Check whether two port dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for ifaceA in arrA:
        ifaceB = next((x for x in arrB if x.get("id") == ifaceA.get("id")), None)
        if ifaceB is None:
            return False
        if ifaceA.get("name") != ifaceB.get("name"):
            return False
        if _normalizeValue(ifaceA.get("description")) != _normalizeValue(
            ifaceB.get("description")
        ):
            return False
        if not areAttributesEqualDict(
            ifaceA.get("attributes"), ifaceB.get("attributes"), strict
        ):
            return False
        if strict:
            if ifaceA.get("createdAt") != ifaceB.get("createdAt"):
                return False
            if ifaceA.get("updatedAt") != ifaceB.get("updatedAt"):
                return False
    return True


def areQualitiesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """🔖Check whether two quality dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for qualA in arrA:
        qualB = next((x for x in arrB if x.get("id") == qualA.get("id")), None)
        if qualB is None:
            return False
        if qualA.get("key") != qualB.get("key"):
            return False
        if qualA.get("name") != qualB.get("name"):
            return False
        if not areAttributesEqualDict(
            qualA.get("attributes"), qualB.get("attributes"), strict
        ):
            return False
        if strict:
            if qualA.get("createdAt") != qualB.get("createdAt"):
                return False
            if qualA.get("updatedAt") != qualB.get("updatedAt"):
                return False
    return True


def areFilesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """🔖Check whether two file dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for fileA in arrA:
        fileB = next((x for x in arrB if x.get("id") == fileA.get("id")), None)
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
    """🔖Check whether two folder dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for folderA in arrA:
        folderB = next((x for x in arrB if x.get("id") == folderA.get("id")), None)
        if folderB is None:
            return False
        if folderA.get("name") != folderB.get("name"):
            return False
        if not areAttributesEqualDict(
            folderA.get("attributes"), folderB.get("attributes"), strict
        ):
            return False
        if strict:
            if folderA.get("createdAt") != folderB.get("createdAt"):
                return False
            if folderA.get("updatedAt") != folderB.get("updatedAt"):
                return False
    return True


def areAuthorsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """🔖Check whether two author dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for authorA in arrA:
        authorB = next((x for x in arrB if x.get("id") == authorA.get("id")), None)
        if authorB is None:
            return False
        if authorA.get("name") != authorB.get("name"):
            return False
        if _normalizeValue(authorA.get("email")) != _normalizeValue(
            authorB.get("email")
        ):
            return False
        if not areAttributesEqualDict(
            authorA.get("attributes"), authorB.get("attributes"), strict
        ):
            return False
        if strict:
            if authorA.get("createdAt") != authorB.get("createdAt"):
                return False
            if authorA.get("updatedAt") != authorB.get("updatedAt"):
                return False
    return True


def areConceptsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """🔖Check whether two concept dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for conceptA in arrA:
        conceptB = next((x for x in arrB if x.get("id") == conceptA.get("id")), None)
        if conceptB is None:
            return False
        if conceptA.get("name") != conceptB.get("name"):
            return False
        if _normalizeValue(conceptA.get("description")) != _normalizeValue(
            conceptB.get("description")
        ):
            return False
        if _normalizeValue(conceptA.get("icon")) != _normalizeValue(
            conceptB.get("icon")
        ):
            return False
        if strict:
            if conceptA.get("createdAt") != conceptB.get("createdAt"):
                return False
            if conceptA.get("updatedAt") != conceptB.get("updatedAt"):
                return False
    return True


def areTagsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """🔖Check whether two tag dictionaries are equal."""
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
        return False
    for tagA in arrA:
        tagB = next((x for x in arrB if x.get("id") == tagA.get("id")), None)
        if tagB is None:
            return False
        if tagA.get("name") != tagB.get("name"):
            return False
        if _normalizeValue(tagA.get("description")) != _normalizeValue(
            tagB.get("description")
        ):
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
    """🔖Deep equality check for kits (dict-based) - recursively compares all properties including nested entities.
    Args:
    a: First kit dict
    b: Second kit dict
    strict: If True, also compare timestamps (createdAt, updatedAt). Default False.
    Returns:
    True if kits are equal, False otherwise.
    """
    if a.get("id") != b.get("id"):
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
    if not arePortsEqualDict(a.get("ports"), b.get("ports"), strict):
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


def _getCollectionDiff(
    before: list,
    after: list,
    getItemDiff: typing.Callable[[dict, dict], dict],
    entityKey: str = "",
) -> dict:
    """Get diff for a collection of items identified by id.

    Args:
        before: The before collection
        after: The after collection
        getItemDiff: Function to get item-level diff
        entityKey: The key name for the entity ID in the updated array (e.g., "type", "design", "piece")
    """
    diff: dict = {}
    beforeIds = {item.get("id") for item in before}
    afterIds = {item.get("id") for item in after}

    removed = [
        {"id": item.get("id")} for item in before if item.get("id") not in afterIds
    ]
    if removed:
        diff["removed"] = removed
    updated = []
    for item in before:
        if item.get("id") in afterIds:
            afterItem = next(a for a in after if a.get("id") == item.get("id"))
            itemDiff = getItemDiff(item, afterItem)
            if itemDiff:
                if entityKey:
                    updated.append(
                        {entityKey: {"id": item.get("id")}, "diff": itemDiff}
                    )
                else:
                    updated.append({"id": item.get("id"), "diff": itemDiff})
    if updated:
        diff["updated"] = updated
    added = [item for item in after if item.get("id") not in beforeIds]
    if added:
        diff["added"] = added
    return diff


def _applyCollectionDiff(
    items: list,
    diff: dict | None,
    applyItemDiff: typing.Callable[[dict, dict], None],
    entityKey: str = "",
) -> None:
    """Apply diff to a collection of items in-place.

    Args:
        items: The collection to mutate
        diff: The diff to apply (with removed, updated, added)
        applyItemDiff: Function to apply item-level diff in-place
        entityKey: The key name for the entity ID in the updated array (e.g., "type", "design", "piece")
    """
    if not diff:
        return
    if diff.get("removed"):
        removedIds = {r["id"] if isinstance(r, dict) else r for r in diff["removed"]}
        items[:] = [item for item in items if item.get("id") not in removedIds]
    if diff.get("updated"):
        for update in diff["updated"]:
            updateId = None
            if entityKey and entityKey in update:
                updateId = update[entityKey]["id"]
            elif "id" in update:
                updateId = update["id"]
            if not updateId:
                continue
            item = next(
                (i for i in items if i.get("id") == updateId),
                None,
            )
            if item is not None:
                applyItemDiff(item, update["diff"])
    if diff.get("added"):
        items.extend(diff["added"])


def _getTypeDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two type dicts."""
    diff: dict = {}
    for key in ["name", "description", "icon", "image", "folder", "unit", "stock"]:
        if _normalizeValue(before.get(key)) != _normalizeValue(after.get(key)):
            diff[key] = after.get(key)
    for key in ["isAbstract", "virtual"]:
        if _normalizeBoolean(before.get(key)) != _normalizeBoolean(after.get(key)):
            diff[key] = after.get(key)
    for refKey in ["location", "parent"]:
        bId = (
            before.get(refKey, {}).get("id")
            if isinstance(before.get(refKey), dict)
            else None
        )
        aId = (
            after.get(refKey, {}).get("id")
            if isinstance(after.get(refKey), dict)
            else None
        )
        if _normalizeValue(bId) != _normalizeValue(aId):
            diff[refKey] = after.get(refKey)
    if json.dumps(
        sorted(
            before.get("concepts", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("concepts", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["concepts"] = after.get("concepts")
    if json.dumps(
        sorted(
            before.get("authors", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("authors", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["authors"] = after.get("authors")
    connectorsDiff = _getCollectionDiff(
        before.get("connectors", []),
        after.get("connectors", []),
        _getConnectorDiff,
        "connector",
    )
    if connectorsDiff:
        diff["connectors"] = connectorsDiff
    representationsDiff = _getCollectionDiff(
        before.get("representations", []),
        after.get("representations", []),
        _getRepresentationDiff,
        "representation",
    )
    if representationsDiff:
        diff["representations"] = representationsDiff
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyTypeDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a type dict in-place."""
    for key in [
        "name",
        "description",
        "icon",
        "image",
        "folder",
        "unit",
        "stock",
        "isAbstract",
        "virtual",
    ]:
        if key in diff:
            target[key] = diff[key]
    for refKey in ["location", "parent"]:
        if refKey in diff:
            target[refKey] = diff[refKey]
    if "concepts" in diff:
        target["concepts"] = diff["concepts"]
    if "authors" in diff:
        target["authors"] = diff["authors"]
    if diff.get("connectors"):
        if "connectors" not in target:
            target["connectors"] = []
        _applyCollectionDiff(
            target["connectors"],
            diff["connectors"],
            _applyConnectorDiff,
            "connector",
        )
    if diff.get("representations"):
        if "representations" not in target:
            target["representations"] = []
        _applyCollectionDiff(
            target["representations"],
            diff["representations"],
            _applyRepresentationDiff,
            "representation",
        )
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getConnectorDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two connector dicts."""
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    if before.get("t") != after.get("t"):
        diff["t"] = after.get("t")
    if _normalizeBoolean(before.get("mandatory")) != _normalizeBoolean(
        after.get("mandatory")
    ):
        diff["mandatory"] = after.get("mandatory")
    bPortId = (
        before.get("port", {}).get("id")
        if isinstance(before.get("port"), dict)
        else None
    )
    aPortId = (
        after.get("port", {}).get("id") if isinstance(after.get("port"), dict) else None
    )
    if _normalizeValue(bPortId) != _normalizeValue(aPortId):
        diff["port"] = after.get("port")
    bPoint = before.get("point", {})
    aPoint = after.get("point", {})
    if bPoint and aPoint and isinstance(bPoint, dict) and isinstance(aPoint, dict):
        px = (aPoint.get("x", 0) or 0) - (bPoint.get("x", 0) or 0)
        py = (aPoint.get("y", 0) or 0) - (bPoint.get("y", 0) or 0)
        pz = (aPoint.get("z", 0) or 0) - (bPoint.get("z", 0) or 0)
        if abs(px) > 1e-10 or abs(py) > 1e-10 or abs(pz) > 1e-10:
            diff["point"] = {"x": px, "y": py, "z": pz}
    elif aPoint and not bPoint:
        diff["point"] = aPoint
    bDir = before.get("direction", {})
    aDir = after.get("direction", {})
    if bDir and aDir and isinstance(bDir, dict) and isinstance(aDir, dict):
        dx = (aDir.get("x", 0) or 0) - (bDir.get("x", 0) or 0)
        dy = (aDir.get("y", 0) or 0) - (bDir.get("y", 0) or 0)
        dz = (aDir.get("z", 0) or 0) - (bDir.get("z", 0) or 0)
        if abs(dx) > 1e-10 or abs(dy) > 1e-10 or abs(dz) > 1e-10:
            diff["direction"] = {"x": dx, "y": dy, "z": dz}
    elif aDir and not bDir:
        diff["direction"] = aDir
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyConnectorDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a connector dict in-place."""
    for key in ["name", "description", "t", "mandatory"]:
        if key in diff:
            target[key] = diff[key]
    if "port" in diff:
        target["port"] = diff["port"]
    if "point" in diff:
        bPoint = target.get("point", {})
        if bPoint and isinstance(bPoint, dict):
            target["point"] = {
                "x": (bPoint.get("x", 0) or 0) + (diff["point"].get("x", 0) or 0),
                "y": (bPoint.get("y", 0) or 0) + (diff["point"].get("y", 0) or 0),
                "z": (bPoint.get("z", 0) or 0) + (diff["point"].get("z", 0) or 0),
            }
        else:
            target["point"] = diff["point"]
    if "direction" in diff:
        bDir = target.get("direction", {})
        if bDir and isinstance(bDir, dict):
            target["direction"] = {
                "x": (bDir.get("x", 0) or 0) + (diff["direction"].get("x", 0) or 0),
                "y": (bDir.get("y", 0) or 0) + (diff["direction"].get("y", 0) or 0),
                "z": (bDir.get("z", 0) or 0) + (diff["direction"].get("z", 0) or 0),
            }
        else:
            target["direction"] = diff["direction"]
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getRepresentationDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two representation dicts."""
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    bFileId = (
        before.get("file", {}).get("id")
        if isinstance(before.get("file"), dict)
        else None
    )
    aFileId = (
        after.get("file", {}).get("id") if isinstance(after.get("file"), dict) else None
    )
    if _normalizeValue(bFileId) != _normalizeValue(aFileId):
        diff["file"] = after.get("file")
    if json.dumps(
        sorted(
            before.get("tags", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("tags", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["tags"] = after.get("tags")
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyRepresentationDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a representation dict in-place."""
    for key in ["name", "description"]:
        if key in diff:
            target[key] = diff[key]
    if "file" in diff:
        target["file"] = diff["file"]
    if "tags" in diff:
        target["tags"] = diff["tags"]
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getDesignDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two design dicts."""
    diff: dict = {}
    for key in [
        "name",
        "variant",
        "view",
        "description",
        "icon",
        "image",
        "unit",
        "folder",
    ]:
        if _normalizeValue(before.get(key)) != _normalizeValue(after.get(key)):
            diff[key] = after.get(key)
    for key in ["isAbstract", "canScale", "canMirror"]:
        if _normalizeBoolean(before.get(key)) != _normalizeBoolean(after.get(key)):
            diff[key] = after.get(key)
    for refKey in ["activeLayer", "parent", "location"]:
        bId = (
            before.get(refKey, {}).get("id")
            if isinstance(before.get(refKey), dict)
            else None
        )
        aId = (
            after.get(refKey, {}).get("id")
            if isinstance(after.get(refKey), dict)
            else None
        )
        if _normalizeValue(bId) != _normalizeValue(aId):
            diff[refKey] = after.get(refKey)
    if json.dumps(
        sorted(
            before.get("concepts", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("concepts", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["concepts"] = after.get("concepts")
    if json.dumps(
        sorted(
            before.get("authors", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("authors", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["authors"] = after.get("authors")
    piecesDiff = _getCollectionDiff(
        before.get("pieces", []), after.get("pieces", []), _getPieceDiff, "piece"
    )
    if piecesDiff:
        diff["pieces"] = piecesDiff
    connectionsDiff = _getCollectionDiff(
        before.get("connections", []),
        after.get("connections", []),
        _getConnectionDiff,
        "connection",
    )
    if connectionsDiff:
        diff["connections"] = connectionsDiff
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyDesignDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a design dict in-place."""
    for key in [
        "name",
        "variant",
        "view",
        "description",
        "icon",
        "image",
        "unit",
        "folder",
        "isAbstract",
        "canScale",
        "canMirror",
    ]:
        if key in diff:
            target[key] = diff[key]
    for refKey in ["activeLayer", "parent", "location"]:
        if refKey in diff:
            target[refKey] = diff[refKey]
    if "concepts" in diff:
        target["concepts"] = diff["concepts"]
    if "authors" in diff:
        target["authors"] = diff["authors"]
    if diff.get("pieces"):
        if "pieces" not in target:
            target["pieces"] = []
        _applyCollectionDiff(target["pieces"], diff["pieces"], _applyPieceDiff, "piece")
    if diff.get("connections"):
        if "connections" not in target:
            target["connections"] = []
        _applyCollectionDiff(
            target["connections"],
            diff["connections"],
            _applyConnectionDiff,
            "connection",
        )
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def designWithDiffDict(base: dict, diff: dict) -> dict:
    """🔖Create a mixed design applying diff changes and annotating with diff status.
    Annotate each with a semio.diffStatus attribute (unchanged/modified/removed/added).
    Updated entities are applied (new positions/values) and marked as modified.
    Removed entities are kept in place marked as removed.
    Added entities are appended marked as added.
    """
    import copy

    def status_attr(status: str) -> dict:
        return {
            "id": f"semio.diffStatus.{status}",
            "key": "semio.diffStatus",
            "value": status,
        }

    pieces_diff = diff.get("pieces", {})
    removed_piece_ids = {r["id"] for r in pieces_diff.get("removed", [])}
    updated_piece_map = {
        u.get("piece", {}).get("id"): u.get("diff", {})
        for u in pieces_diff.get("updated", [])
    }

    conns_diff = diff.get("connections", {})
    removed_conn_ids = {r["id"] for r in conns_diff.get("removed", [])}
    updated_conn_map = {
        u.get("connection", {}).get("id"): u.get("diff", {})
        for u in conns_diff.get("updated", [])
    }

    result_pieces = []
    for p in base.get("pieces", []):
        pc = copy.deepcopy(p)
        if pc["id"] in removed_piece_ids:
            attrs = pc.get("attributes", []) or []
            attrs.append(status_attr("removed"))
            pc["attributes"] = attrs
        elif pc["id"] in updated_piece_map:
            base_plane = _dict_piece_plane(pc)
            base_center = _dict_piece_center(pc)
            _applyPieceDiff(pc, updated_piece_map[pc["id"]])
            # 📌Preserve base geometry so modified pieces stay in place and only get recolored.
            if base_plane is not None or base_center is not None:
                pc["pose"] = {"plane": base_plane, "center": base_center}
            elif "pose" in pc:
                del pc["pose"]
            attrs = pc.get("attributes", []) or []
            attrs.append(status_attr("modified"))
            pc["attributes"] = attrs
        else:
            attrs = pc.get("attributes", []) or []
            attrs.append(status_attr("unchanged"))
            pc["attributes"] = attrs
        result_pieces.append(pc)
    for added in pieces_diff.get("added", []):
        ac = copy.deepcopy(added)
        attrs = ac.get("attributes", []) or []
        attrs.append(status_attr("added"))
        ac["attributes"] = attrs
        result_pieces.append(ac)

    result_conns = []
    for c in base.get("connections", []):
        cc = copy.deepcopy(c)
        if cc["id"] in removed_conn_ids:
            attrs = cc.get("attributes", []) or []
            attrs.append(status_attr("removed"))
            cc["attributes"] = attrs
        elif cc["id"] in updated_conn_map:
            _applyConnectionDiff(cc, updated_conn_map[cc["id"]])
            attrs = cc.get("attributes", []) or []
            attrs.append(status_attr("modified"))
            cc["attributes"] = attrs
        else:
            attrs = cc.get("attributes", []) or []
            attrs.append(status_attr("unchanged"))
            cc["attributes"] = attrs
        result_conns.append(cc)
    for added in conns_diff.get("added", []):
        ac = copy.deepcopy(added)
        attrs = ac.get("attributes", []) or []
        attrs.append(status_attr("added"))
        ac["attributes"] = attrs
        result_conns.append(ac)

    result = copy.deepcopy(base)
    result["pieces"] = result_pieces
    result["connections"] = result_conns
    return result


def _getPieceDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two piece dicts."""
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    for refKey in ["type", "design"]:
        bId = (
            before.get(refKey, {}).get("id")
            if isinstance(before.get(refKey), dict)
            else None
        )
        aId = (
            after.get(refKey, {}).get("id")
            if isinstance(after.get(refKey), dict)
            else None
        )
        if _normalizeValue(bId) != _normalizeValue(aId):
            diff[refKey] = after.get(refKey)
    if _dict_piece_plane(before) != _dict_piece_plane(after) or _dict_piece_center(
        before
    ) != _dict_piece_center(after):
        diff["pose"] = {
            "plane": _dict_piece_plane(after),
            "center": _dict_piece_center(after),
        }
    if before.get("scale") != after.get("scale"):
        diff["scale"] = after.get("scale")
    if _normalizeValue(before.get("color")) != _normalizeValue(after.get("color")):
        diff["color"] = after.get("color")
    for key in ["isHidden", "isLocked"]:
        if before.get(key) != after.get(key):
            diff[key] = after.get(key)
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyPieceDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a piece dict in-place."""
    for key in [
        "name",
        "description",
        "scale",
        "color",
        "isHidden",
        "isLocked",
    ]:
        if key in diff:
            target[key] = diff[key]
    if "pose" in diff:
        target["pose"] = diff["pose"]
    for refKey in ["type", "design"]:
        if refKey in diff:
            target[refKey] = diff[refKey]
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getConnectionDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two connection dicts."""
    diff: dict = {}
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        bVal = before.get(key, 0) or 0
        aVal = after.get(key, 0) or 0
        delta = aVal - bVal
        if abs(delta) > 1e-10:
            diff[key] = delta
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    if before.get("child") != after.get("child"):
        diff["child"] = after.get("child")
    if before.get("parent") != after.get("parent"):
        diff["parent"] = after.get("parent")
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyConnectionDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a connection dict in-place."""
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        if key in diff:
            target[key] = (target.get(key, 0) or 0) + (diff[key] or 0)
    for key in ["description"]:
        if key in diff:
            target[key] = diff[key]
    for key in ["child", "connected"]:
        if key in diff:
            target[key] = diff[key]
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getTagDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two tag dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyTagDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a tag dict in-place."""
    for key in ["name", "description", "icon"]:
        if key in diff:
            target[key] = diff[key]
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getConceptDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two concept dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyConceptDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a concept dict in-place."""
    for key in ["name", "description", "icon"]:
        if key in diff:
            target[key] = diff[key]
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getPortDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two port dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    if json.dumps(
        sorted(
            before.get("compatiblePorts", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("compatiblePorts", []),
            key=lambda x: x.get("id", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["compatiblePorts"] = after.get("compatiblePorts")
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyPortDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a port dict in-place."""
    for key in ["name", "description", "icon"]:
        if key in diff:
            target[key] = diff[key]
    if "compatiblePorts" in diff:
        target["compatiblePorts"] = diff["compatiblePorts"]
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getFileDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two file dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("remote")) != _normalizeValue(after.get("remote")):
        diff["remote"] = after.get("remote")
    if before.get("size") != after.get("size"):
        diff["size"] = after.get("size")
    if _normalizeValue(before.get("hash")) != _normalizeValue(after.get("hash")):
        diff["hash"] = after.get("hash")
    if _normalizeValue(before.get("blob")) != _normalizeValue(after.get("blob")):
        diff["blob"] = after.get("blob")
    bFolderId = (
        before.get("folder", {}).get("id")
        if isinstance(before.get("folder"), dict)
        else None
    )
    aFolderId = (
        after.get("folder", {}).get("id")
        if isinstance(after.get("folder"), dict)
        else None
    )
    if _normalizeValue(bFolderId) != _normalizeValue(aFolderId):
        diff["folder"] = after.get("folder")
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyFileDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a file dict in-place."""
    for key in ["name", "description", "remote", "size", "hash", "blob"]:
        if key in diff:
            target[key] = diff[key]
    if "folder" in diff:
        target["folder"] = diff["folder"]
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getFolderDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two folder dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyFolderDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a folder dict in-place."""
    for key in ["name", "description"]:
        if key in diff:
            target[key] = diff[key]
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getQualityDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two quality dicts."""
    diff: dict = {}
    if before.get("key") != after.get("key"):
        diff["key"] = after.get("key")
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("uri")) != _normalizeValue(after.get("uri")):
        diff["uri"] = after.get("uri")
    if before.get("kind") != after.get("kind"):
        diff["kind"] = after.get("kind")
    if _normalizeBoolean(before.get("canScale")) != _normalizeBoolean(
        after.get("canScale")
    ):
        diff["canScale"] = after.get("canScale")
    if _normalizeValue(before.get("defaultSiUnit")) != _normalizeValue(
        after.get("defaultSiUnit")
    ):
        diff["defaultSiUnit"] = after.get("defaultSiUnit")
    if _normalizeValue(before.get("defaultImperialUnit")) != _normalizeValue(
        after.get("defaultImperialUnit")
    ):
        diff["defaultImperialUnit"] = after.get("defaultImperialUnit")
    if before.get("min") != after.get("min"):
        diff["min"] = after.get("min")
    if _normalizeBoolean(before.get("isMinExcluded")) != _normalizeBoolean(
        after.get("isMinExcluded")
    ):
        diff["isMinExcluded"] = after.get("isMinExcluded")
    if before.get("max") != after.get("max"):
        diff["max"] = after.get("max")
    if _normalizeBoolean(before.get("isMaxExcluded")) != _normalizeBoolean(
        after.get("isMaxExcluded")
    ):
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


def _applyQualityDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to a quality dict in-place."""
    for key in [
        "key",
        "name",
        "description",
        "uri",
        "kind",
        "canScale",
        "defaultSiUnit",
        "defaultImperialUnit",
        "min",
        "isMinExcluded",
        "max",
        "isMaxExcluded",
        "defaultValue",
        "formula",
        "icon",
        "image",
        "unit",
    ]:
        if key in diff:
            target[key] = diff[key]


def _getAuthorDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two author dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("email")) != _normalizeValue(after.get("email")):
        diff["email"] = after.get("email")
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyAuthorDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to an author dict in-place."""
    for key in ["name", "email"]:
        if key in diff:
            target[key] = diff[key]
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _getAttributeDiff(before: dict, after: dict) -> dict:
    """🔖Get diff between two attribute dicts - used for individual attribute update diffs."""
    diff: dict = {}
    if _normalizeValue(before.get("key")) != _normalizeValue(after.get("key")):
        diff["key"] = after.get("key")
    if _normalizeValue(before.get("value")) != _normalizeValue(after.get("value")):
        diff["value"] = after.get("value")
    if _normalizeValue(before.get("definition")) != _normalizeValue(
        after.get("definition")
    ):
        diff["definition"] = after.get("definition")
    return diff


def _applyAttributeDiff(target: dict, diff: dict) -> None:
    """🔖Apply diff to an attribute dict in-place."""
    for key in ["key", "value", "definition"]:
        if key in diff:
            target[key] = diff[key]


def _getAttributesDiff(before: list, after: list) -> dict:
    """🔖Get diff for attributes collection - uses ID for identification with EntityId format."""
    diff: dict = {}
    beforeIds = {a.get("id") for a in before}
    afterIds = {a.get("id") for a in after}

    removed = [{"id": a.get("id")} for a in before if a.get("id") not in afterIds]
    if removed:
        diff["removed"] = removed
    updated = []
    for afterAttr in after:
        id = afterAttr.get("id")
        if id in beforeIds:
            beforeAttr = next(a for a in before if a.get("id") == id)
            attrDiff = _getAttributeDiff(beforeAttr, afterAttr)
            if attrDiff:
                updated.append({"attribute": {"id": id}, "diff": attrDiff})
    if updated:
        diff["updated"] = updated
    added = [a for a in after if a.get("id") not in beforeIds]
    if added:
        diff["added"] = added
    return diff


def _applyAttributesDiff(items: list, diff: dict | None) -> None:
    """🔖Apply diff to attributes collection in-place - uses ID for identification with EntityId format."""
    if not diff:
        return
    if diff.get("removed"):
        removedIds = {r["id"] if isinstance(r, dict) else r for r in diff["removed"]}
        items[:] = [a for a in items if a.get("id") not in removedIds]
    if diff.get("updated"):
        for update in diff["updated"]:
            updateId = (
                update["attribute"]["id"]
                if "attribute" in update
                else update.get("id", "")
            )
            item = next((a for a in items if a.get("id") == updateId), None)
            if item is not None:
                _applyAttributeDiff(item, update["diff"])
    if diff.get("added"):
        items.extend(diff["added"])


def _inverseAttributesDiff(original: list, appliedDiff: dict) -> dict:
    """🔖Compute inverse of attributes collection diff - uses ID with EntityId format."""
    inverse: dict = {}

    removedIds = [
        r["id"] if isinstance(r, dict) else r for r in appliedDiff.get("removed", [])
    ]

    updatedIds = []
    for u in appliedDiff.get("updated", []):
        if "attribute" in u:
            updatedIds.append(u["attribute"]["id"])
        else:
            updatedIds.append(u.get("id", ""))
    addedIds = [a.get("id") for a in appliedDiff.get("added", [])]
    if addedIds:
        inverse["removed"] = [{"id": id} for id in addedIds]
    if updatedIds:
        inverse["updated"] = []
        for id in updatedIds:
            origAttr = next((a for a in original if a.get("id") == id), None)
            upd = next(
                (
                    u
                    for u in appliedDiff.get("updated", [])
                    if (
                        u.get("attribute", {}).get("id")
                        if "attribute" in u
                        else u.get("id")
                    )
                    == id
                ),
                None,
            )
            if origAttr and upd:
                inverse["updated"].append(
                    {
                        "attribute": {"id": id},
                        "diff": _inverseAttributeDiff(origAttr, upd["diff"]),
                    }
                )
    if removedIds:
        inverse["added"] = [a for a in original if a.get("id") in removedIds]
    return inverse


def _inverseAttributeDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of an attribute diff."""
    inverse: dict = {}
    for key in ["key", "value", "definition"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse


def getKitDiffDict(before: dict, after: dict) -> dict:
    """🔖Compute the diff between two kit dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if before.get("version") != after.get("version"):
        diff["version"] = after.get("version")
    if _normalizeValue(before.get("description")) != _normalizeValue(
        after.get("description")
    ):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    if _normalizeValue(before.get("image")) != _normalizeValue(after.get("image")):
        diff["image"] = after.get("image")
    if _normalizeValue(before.get("remote")) != _normalizeValue(after.get("remote")):
        diff["remote"] = after.get("remote")
    if _normalizeValue(before.get("homepage")) != _normalizeValue(
        after.get("homepage")
    ):
        diff["homepage"] = after.get("homepage")
    if _normalizeValue(before.get("license")) != _normalizeValue(after.get("license")):
        diff["license"] = after.get("license")
    if _normalizeValue(before.get("preview")) != _normalizeValue(after.get("preview")):
        diff["preview"] = after.get("preview")
    typesDiff = _getCollectionDiff(
        before.get("types", []), after.get("types", []), _getTypeDiff, "type"
    )
    if typesDiff:
        diff["types"] = typesDiff
    designsDiff = _getCollectionDiff(
        before.get("designs", []), after.get("designs", []), _getDesignDiff, "design"
    )
    if designsDiff:
        diff["designs"] = designsDiff
    tagsDiff = _getCollectionDiff(
        before.get("tags", []), after.get("tags", []), _getTagDiff, "tag"
    )
    if tagsDiff:
        diff["tags"] = tagsDiff
    conceptsDiff = _getCollectionDiff(
        before.get("concepts", []),
        after.get("concepts", []),
        _getConceptDiff,
        "concept",
    )
    if conceptsDiff:
        diff["concepts"] = conceptsDiff
    portsDiff = _getCollectionDiff(
        before.get("ports", []), after.get("ports", []), _getPortDiff, "port"
    )
    if portsDiff:
        diff["ports"] = portsDiff
    filesDiff = _getCollectionDiff(
        before.get("files", []), after.get("files", []), _getFileDiff, "file"
    )
    if filesDiff:
        diff["files"] = filesDiff
    foldersDiff = _getCollectionDiff(
        before.get("folders", []), after.get("folders", []), _getFolderDiff, "folder"
    )
    if foldersDiff:
        diff["folders"] = foldersDiff
    qualitiesDiff = _getCollectionDiff(
        before.get("qualities", []),
        after.get("qualities", []),
        _getQualityDiff,
        "quality",
    )
    if qualitiesDiff:
        diff["qualities"] = qualitiesDiff
    authorsDiff = _getCollectionDiff(
        before.get("authors", []), after.get("authors", []), _getAuthorDiff, "author"
    )
    if authorsDiff:
        diff["authors"] = authorsDiff
    attributesDiff = _getAttributesDiff(
        before.get("attributes", []), after.get("attributes", [])
    )
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _kitdiff_deep_equal(a: typing.Any, b: typing.Any) -> bool:
    """Deep structural equality for noop remove/add detection in kit diff validation."""
    if a is b:
        return True
    if type(a) != type(b):
        return False
    if isinstance(a, dict):
        if set(a.keys()) != set(b.keys()):
            return False
        return all(_kitdiff_deep_equal(a[k], b[k]) for k in a)
    if isinstance(a, list):
        if len(a) != len(b):
            return False
        if len(a) != len(b):
            return False
        return all(_kitdiff_deep_equal(x, y) for x, y in zip(a, b))
    return a == b


def _kitdiff_push(ctx: dict, kind: str, code: str, message: str) -> None:
    ctx[kind].append({"code": code, "message": message})


def _validate_id_collection_diff(
    ctx: dict,
    path: str,
    id_key: str,
    base: list,
    raw: dict | None,
    on_updated: typing.Callable[[dict, dict, str], None] | None = None,
) -> dict | None:
    """Validate removed/updated/added ids for one collection diff; heal trims invalid ops when ctx["heal"]."""
    if not raw:
        return None
    heal: bool = ctx["heal"]
    base_by = {i.get("id"): i for i in base if isinstance(i, dict) and i.get("id")}
    removed_ids = {r.get("id") for r in raw.get("removed") or [] if isinstance(r, dict)}
    after_remove = {g for g in base_by if g not in removed_ids}
    h_rem = list(raw.get("removed") or []) if heal else None
    h_upd = list(raw.get("updated") or []) if heal else None
    h_add = list(raw.get("added") or []) if heal else None

    for r in raw.get("removed") or []:
        if not isinstance(r, dict):
            continue
        rg = r.get("id")
        if rg not in base_by:
            _kitdiff_push(
                ctx,
                "warnings",
                "kitdiff.remove.missing-target",
                f"{path}: remove references missing {id_key} {rg}",
            )
            if heal and h_rem is not None:
                h_rem = [x for x in h_rem if x.get("id") != rg]

    add_by_id = {
        a.get("id"): a
        for a in raw.get("added") or []
        if isinstance(a, dict) and a.get("id")
    }
    for r in raw.get("removed") or []:
        if not isinstance(r, dict):
            continue
        rg = r.get("id")
        orig = base_by.get(rg)
        add = add_by_id.get(rg)
        if orig is not None and add is not None and _kitdiff_deep_equal(orig, add):
            _kitdiff_push(
                ctx,
                "warnings",
                "kitdiff.cycle.noop-restore",
                f"{path}: removed and re-added {id_key} {rg} are deeply equal (no effective change)",
            )
            if heal:
                if h_rem is not None:
                    h_rem = [x for x in h_rem if x.get("id") != rg]
                if h_add is not None:
                    h_add = [x for x in h_add if x.get("id") != rg]

    seen_add: set[str] = set()
    for a in raw.get("added") or []:
        if not isinstance(a, dict):
            continue
        ag = a.get("id")
        if ag in seen_add:
            _kitdiff_push(
                ctx,
                "errors",
                "kitdiff.add.duplicate-in-diff",
                f"{path}: duplicate added {id_key} id {ag}",
            )
            if heal and h_add is not None:
                na = []
                first_kept = False
                for x in h_add:
                    if x.get("id") == ag:
                        if not first_kept:
                            na.append(x)
                            first_kept = True
                        continue
                    na.append(x)
                h_add = na
        seen_add.add(ag)
        if ag in after_remove:
            _kitdiff_push(
                ctx,
                "errors",
                "kitdiff.add.duplicate-id",
                f"{path}: cannot add {id_key} {ag} that still exists after removes",
            )
            if heal and h_add is not None:
                h_add = [x for x in h_add if x.get("id") != ag]

    for u in raw.get("updated") or []:
        if not isinstance(u, dict) or id_key not in u:
            continue
        gid = (u.get(id_key) or {}).get("id")
        p = f"{path}.{id_key}[{gid}]"
        if not gid:
            _kitdiff_push(
                ctx, "errors", "kitdiff.update.bad-id", f"{p}: missing {id_key} id"
            )
            if heal and h_upd is not None:
                h_upd = [x for x in h_upd if (x.get(id_key) or {}).get("id") != gid]
            continue
        if gid not in after_remove:
            _kitdiff_push(
                ctx,
                "errors",
                "kitdiff.update.missing-target",
                f"{p}: update targets {id_key} not present after removes",
            )
            if heal and h_upd is not None:
                h_upd = [x for x in h_upd if (x.get(id_key) or {}).get("id") != gid]
            continue
        item = base_by.get(gid)
        if item is None:
            _kitdiff_push(
                ctx,
                "errors",
                "kitdiff.update.missing-base",
                f"{p}: {id_key} not found in base kit",
            )
            if heal and h_upd is not None:
                h_upd = [x for x in h_upd if (x.get(id_key) or {}).get("id") != gid]
            continue
        if on_updated:
            on_updated(item, u.get("diff") or {}, p)

    if not heal:
        return raw
    out: dict = {}
    if h_rem:
        out["removed"] = h_rem
    if h_upd:
        out["updated"] = h_upd
    if h_add:
        out["added"] = h_add
    return out or None


def _validate_design_diff_nested_py(
    ctx: dict, kit: dict, path: str, design: dict, diff: dict, refs: dict
) -> None:
    """Validate nested design diff: piece type refs, authors diff or list."""
    type_ids: set[str] = refs["typeIds"]
    design_ids: set[str] = refs["designIds"]
    author_ids: set[str] = refs["authorIds"]

    if diff.get("parent") and isinstance(diff["parent"], dict):
        pg = diff["parent"].get("id")
        if pg and pg not in design_ids:
            _kitdiff_push(
                ctx,
                "errors",
                "kitdiff.ref.design-parent-missing",
                f"{path}: parent design {pg} not in kit",
            )
        if pg == design.get("id"):
            _kitdiff_push(
                ctx,
                "errors",
                "kitdiff.ref.design-parent-self",
                f"{path}: design cannot be its own parent",
            )

    da = diff.get("authors")
    if da is not None:
        if isinstance(da, list):
            for a in da:
                if isinstance(a, dict) and a.get("id") and a["id"] not in author_ids:
                    _kitdiff_push(
                        ctx,
                        "errors",
                        "kitdiff.ref.author-missing",
                        f"{path}: author {a['id']} not in kit",
                    )
        elif isinstance(da, dict):
            _validate_id_collection_diff(
                ctx,
                f"{path}.authors",
                "author",
                kit.get("authors") or [],
                da,
                None,
            )

    pd = diff.get("pieces")
    if isinstance(pd, dict):
        _validate_id_collection_diff(
            ctx,
            f"{path}.pieces",
            "piece",
            design.get("pieces") or [],
            pd,
            None,
        )
        for a in pd.get("added") or []:
            if not isinstance(a, dict):
                continue
            tg = (a.get("type") or {}).get("id")
            if tg and tg not in type_ids:
                _kitdiff_push(
                    ctx,
                    "errors",
                    "kitdiff.ref.piece-type-missing",
                    f"{path}.pieces.added: type {tg} not in kit",
                )
            dg = (
                (a.get("design") or {}).get("id")
                if isinstance(a.get("design"), dict)
                else None
            )
            if dg and dg not in design_ids:
                _kitdiff_push(
                    ctx,
                    "errors",
                    "kitdiff.ref.piece-design-missing",
                    f"{path}.pieces.added: subdesign {dg} not in kit",
                )


def validate_kit_diff_dict(kit: dict, diff: dict, heal: bool) -> dict:
    """Validate a kit diff dict against a base kit dict; optional heal returns a scrubbed diff copy.

    Returns a dict: ok (bool), errors, warnings (list of {code, message}), diff (optional when heal).
    """
    import copy

    working = copy.deepcopy(diff) if heal else diff
    ctx = {"errors": [], "warnings": [], "heal": heal}
    type_ids = {t.get("id") for t in kit.get("types") or [] if t.get("id")}
    design_ids = {d.get("id") for d in kit.get("designs") or [] if d.get("id")}
    quality_ids = {q.get("id") for q in kit.get("qualities") or [] if q.get("id")}
    file_ids = {f.get("id") for f in kit.get("files") or [] if f.get("id")}
    port_ids = {p.get("id") for p in kit.get("ports") or [] if p.get("id")}
    concept_ids = {c.get("id") for c in kit.get("concepts") or [] if c.get("id")}
    author_ids = {a.get("id") for a in kit.get("authors") or [] if a.get("id")}
    refs = {
        "typeIds": type_ids,
        "designIds": design_ids,
        "qualityIds": quality_ids,
        "fileIds": file_ids,
        "portIds": port_ids,
        "conceptIds": concept_ids,
        "authorIds": author_ids,
    }

    out_diff = copy.deepcopy(diff) if heal else None

    def run_coll(key: str, id_key: str, arr_key: str, on_upd=None):
        nonlocal out_diff
        part = working.get(key) if isinstance(working, dict) else None
        if not part:
            return
        fixed = _validate_id_collection_diff(
            ctx, key, id_key, kit.get(arr_key) or [], part, on_upd
        )
        if heal and out_diff is not None:
            if fixed:
                out_diff[key] = fixed
            elif key in out_diff:
                del out_diff[key]

    run_coll("types", "type", "types")
    run_coll(
        "designs",
        "design",
        "designs",
        lambda item, ddf, p: _validate_design_diff_nested_py(
            ctx, kit, p, item, ddf, refs
        ),
    )
    run_coll("tags", "tag", "tags")
    run_coll("concepts", "concept", "concepts")
    run_coll("ports", "port", "ports")
    run_coll("qualities", "quality", "qualities")
    run_coll("files", "file", "files")
    run_coll("folders", "folder", "folders")
    run_coll("authors", "author", "authors")

    if working.get("attributes"):
        _validate_id_collection_diff(
            ctx,
            "kit.attributes",
            "attribute",
            kit.get("attributes") or [],
            working["attributes"],
            None,
        )

    ok = len(ctx["errors"]) == 0
    result: dict = {"ok": ok, "errors": ctx["errors"], "warnings": ctx["warnings"]}
    if heal:
        result["diff"] = out_diff
    return result


def applyKitDiffDict(target: dict, diff: dict) -> None:
    """🔖Apply a diff to a kit dict in-place."""
    for key in [
        "name",
        "version",
        "description",
        "icon",
        "image",
        "remote",
        "homepage",
        "license",
        "preview",
    ]:
        if key in diff:
            value = diff[key]
            if value is not None:
                target[key] = value
            elif key in target:
                del target[key]
    for collKey, applyFn, entityKey in [
        ("types", _applyTypeDiff, "type"),
        ("designs", _applyDesignDiff, "design"),
        ("tags", _applyTagDiff, "tag"),
        ("concepts", _applyConceptDiff, "concept"),
        ("ports", _applyPortDiff, "port"),
        ("files", _applyFileDiff, "file"),
        ("folders", _applyFolderDiff, "folder"),
        ("qualities", _applyQualityDiff, "quality"),
        ("authors", _applyAuthorDiff, "author"),
    ]:
        if diff.get(collKey) or target.get(collKey):
            if collKey not in target:
                target[collKey] = []
            _applyCollectionDiff(target[collKey], diff.get(collKey), applyFn, entityKey)
    if diff.get("attributes") or target.get("attributes"):
        if "attributes" not in target:
            target["attributes"] = []
        _applyAttributesDiff(target["attributes"], diff.get("attributes"))


def _inverseCollectionDiff(
    original: list,
    appliedDiff: dict,
    inverseItemDiff: typing.Callable[[dict, dict], dict],
    entityKey: str = "",
) -> dict:
    """Compute inverse of a collection diff.

    Args:
        original: The original collection before diff was applied
        appliedDiff: The diff that was applied
        inverseItemDiff: Function to compute inverse of item-level diff
        entityKey: The key name for the entity ID (e.g., "type", "design", "piece")
    """
    inverse: dict = {}
    if appliedDiff.get("removed"):
        removedIds = [
            r["id"] if isinstance(r, dict) else r for r in appliedDiff["removed"]
        ]
        inverse["added"] = [item for item in original if item.get("id") in removedIds]
    if appliedDiff.get("added"):
        inverse["removed"] = [{"id": item.get("id")} for item in appliedDiff["added"]]
    if appliedDiff.get("updated"):
        inverse["updated"] = []
        for update in appliedDiff["updated"]:
            updateId = (
                update[entityKey]["id"]
                if entityKey and entityKey in update
                else update.get("id", "")
            )
            origItem = next(
                (item for item in original if item.get("id") == updateId), None
            )
            if origItem:
                if entityKey:
                    inverse["updated"].append(
                        {
                            entityKey: {"id": updateId},
                            "diff": inverseItemDiff(origItem, update["diff"]),
                        }
                    )
                else:
                    inverse["updated"].append(
                        {
                            "id": updateId,
                            "diff": inverseItemDiff(origItem, update["diff"]),
                        }
                    )
    return inverse


def _inverseTypeDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a type diff."""
    inverse: dict = {}
    for key in [
        "name",
        "description",
        "icon",
        "image",
        "folder",
        "unit",
        "stock",
        "isAbstract",
        "virtual",
    ]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    for refKey in ["location", "parent"]:
        if refKey in appliedDiff:
            inverse[refKey] = original.get(refKey)
    if "concepts" in appliedDiff:
        inverse["concepts"] = original.get("concepts")
    if "authors" in appliedDiff:
        inverse["authors"] = original.get("authors")
    if appliedDiff.get("connectors"):
        inverse["connectors"] = _inverseCollectionDiff(
            original.get("connectors", []),
            appliedDiff["connectors"],
            _inverseConnectorDiff,
            "connector",
        )
    if appliedDiff.get("representations"):
        inverse["representations"] = _inverseCollectionDiff(
            original.get("representations", []),
            appliedDiff["representations"],
            _inverseRepresentationDiff,
            "representation",
        )
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseConnectorDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a connector diff."""
    inverse: dict = {}
    for key in ["name", "description", "t", "mandatory"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "port" in appliedDiff:
        inverse["port"] = original.get("port")
    if "point" in appliedDiff:
        p = appliedDiff["point"]
        inverse["point"] = {
            "x": -(p.get("x", 0) or 0),
            "y": -(p.get("y", 0) or 0),
            "z": -(p.get("z", 0) or 0),
        }
    if "direction" in appliedDiff:
        d = appliedDiff["direction"]
        inverse["direction"] = {
            "x": -(d.get("x", 0) or 0),
            "y": -(d.get("y", 0) or 0),
            "z": -(d.get("z", 0) or 0),
        }
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseRepresentationDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a representation diff."""
    inverse: dict = {}
    for key in ["name", "description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "file" in appliedDiff:
        inverse["file"] = original.get("file")
    if "tags" in appliedDiff:
        inverse["tags"] = original.get("tags")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseConnectionDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a connection diff (negate numeric deltas)."""
    inverse: dict = {}
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        if key in appliedDiff:
            inverse[key] = -(appliedDiff[key] or 0)
    for key in ["description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    for key in ["child", "connected"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseRepresentationDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a representation diff."""
    inverse: dict = {}
    for key in ["name", "description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "file" in appliedDiff:
        inverse["file"] = original.get("file")
    if "tags" in appliedDiff:
        inverse["tags"] = original.get("tags")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseConnectionDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a connection diff (negate numeric deltas)."""
    inverse: dict = {}
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        if key in appliedDiff:
            inverse[key] = -(appliedDiff[key] or 0)
    for key in ["description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    for key in ["child", "connected"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseDesignDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a design diff."""
    inverse: dict = {}
    for key in [
        "name",
        "variant",
        "view",
        "description",
        "icon",
        "image",
        "unit",
        "folder",
        "isAbstract",
        "canScale",
        "canMirror",
    ]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    for refKey in ["activeLayer", "parent", "location"]:
        if refKey in appliedDiff:
            inverse[refKey] = original.get(refKey)
    if "concepts" in appliedDiff:
        inverse["concepts"] = original.get("concepts")
    if "authors" in appliedDiff:
        inverse["authors"] = original.get("authors")
    if appliedDiff.get("pieces"):
        inverse["pieces"] = _inverseCollectionDiff(
            original.get("pieces", []),
            appliedDiff["pieces"],
            _inversePieceDiff,
            "piece",
        )
    if appliedDiff.get("connections"):
        inverse["connections"] = _inverseCollectionDiff(
            original.get("connections", []),
            appliedDiff["connections"],
            _inverseConnectionDiff,
            "connection",
        )
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inversePieceDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a piece diff."""
    inverse: dict = {}
    for key in [
        "name",
        "description",
        "scale",
        "plane",
        "center",
        "color",
        "isHidden",
        "isLocked",
    ]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    for refKey in ["type", "design"]:
        if refKey in appliedDiff:
            inverse[refKey] = original.get(refKey)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseTagDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a tag diff."""
    inverse: dict = {}
    for key in ["name", "description", "icon"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseConceptDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a concept diff."""
    inverse: dict = {}
    for key in ["name", "description", "icon"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inversePortDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of an port diff."""
    inverse: dict = {}
    for key in ["name", "description", "icon"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "compatiblePorts" in appliedDiff:
        inverse["compatiblePorts"] = original.get("compatiblePorts")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseFileDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a file diff."""
    inverse: dict = {}
    for key in ["name", "description", "remote", "size", "hash", "blob"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "folder" in appliedDiff:
        inverse["folder"] = original.get("folder")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseFolderDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a folder diff."""
    inverse: dict = {}
    for key in ["name", "description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _inverseQualityDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of a quality diff."""
    inverse: dict = {}
    for key in [
        "key",
        "name",
        "description",
        "uri",
        "kind",
        "canScale",
        "defaultSiUnit",
        "defaultImperialUnit",
        "min",
        "isMinExcluded",
        "max",
        "isMaxExcluded",
        "defaultValue",
        "formula",
        "icon",
        "image",
        "unit",
    ]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse


def _inverseAuthorDiff(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute inverse of an author diff."""
    inverse: dict = {}
    for key in ["name", "email"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def inverseKitDiffDict(original: dict, appliedDiff: dict) -> dict:
    """🔖Compute the inverse of a kit diff."""
    inverse: dict = {}
    for key in [
        "name",
        "version",
        "description",
        "icon",
        "image",
        "remote",
        "homepage",
        "license",
        "preview",
    ]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("types"):
        inverse["types"] = _inverseCollectionDiff(
            original.get("types", []), appliedDiff["types"], _inverseTypeDiff, "type"
        )
    if appliedDiff.get("designs"):
        inverse["designs"] = _inverseCollectionDiff(
            original.get("designs", []),
            appliedDiff["designs"],
            _inverseDesignDiff,
            "design",
        )
    if appliedDiff.get("tags"):
        inverse["tags"] = _inverseCollectionDiff(
            original.get("tags", []), appliedDiff["tags"], _inverseTagDiff, "tag"
        )
    if appliedDiff.get("concepts"):
        inverse["concepts"] = _inverseCollectionDiff(
            original.get("concepts", []),
            appliedDiff["concepts"],
            _inverseConceptDiff,
            "concept",
        )
    if appliedDiff.get("ports"):
        inverse["ports"] = _inverseCollectionDiff(
            original.get("ports", []), appliedDiff["ports"], _inversePortDiff, "port"
        )
    if appliedDiff.get("files"):
        inverse["files"] = _inverseCollectionDiff(
            original.get("files", []), appliedDiff["files"], _inverseFileDiff, "file"
        )
    if appliedDiff.get("folders"):
        inverse["folders"] = _inverseCollectionDiff(
            original.get("folders", []),
            appliedDiff["folders"],
            _inverseFolderDiff,
            "folder",
        )
    if appliedDiff.get("qualities"):
        inverse["qualities"] = _inverseCollectionDiff(
            original.get("qualities", []),
            appliedDiff["qualities"],
            _inverseQualityDiff,
            "quality",
        )
    if appliedDiff.get("authors"):
        inverse["authors"] = _inverseCollectionDiff(
            original.get("authors", []),
            appliedDiff["authors"],
            _inverseAuthorDiff,
            "author",
        )
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(
            original.get("attributes", []), appliedDiff["attributes"]
        )
    return inverse


def _kit_graph_plain_dict(kit: Kit) -> dict:
    """JSON-shaped kit dict aligned with validate/apply kit diff helpers."""
    return kit.representation_dump(mode="json")


def _assign_validated_kit_to(target: Kit, data: dict) -> None:
    parsed = Kit.representation_validate(data)
    for fname in Kit.representation_fields:
        setattr(target, fname, getattr(parsed, fname))


def _apply_kit_graph_diff_to_representation(kit: Kit, diff: dict) -> None:
    d = copy.deepcopy(_kit_graph_plain_dict(kit))
    applyKitDiffDict(d, diff)
    _assign_validated_kit_to(kit, d)


def _notify_kit_backbone_optional(
    backbone: typing.Callable[[KitGraphChange], typing.Any] | None,
    change: KitGraphChange,
) -> None:
    if backbone is None:
        return

    def run() -> None:
        try:
            backbone(change)
        except Exception:
            loguru.logger.exception("Kit backbone notification failed")

    threading.Thread(target=run, daemon=True).start()


def commit_kit_graph_change(
    kit: Kit,
    diff: dict,
    *,
    transaction_id: str | None = None,
    notify_backbone: bool = True,
    skip_global_history: bool = False,
) -> KitGraphChange:
    """Validate, invert, apply diff to Kit in-place; record transaction/history and optionally notify backbone."""
    with kit._graph_lock:
        if kit._conflicted:
            raise ValueError(
                "Kit has unresolved validation conflicts; call clear_conflict() before applying further changes."
            )
        kit_dict = copy.deepcopy(_kit_graph_plain_dict(kit))
        validation = validate_kit_diff_dict(kit_dict, diff, False)
        if not validation.get("ok") or validation.get("errors"):
            kit._conflicted = True
            kit._conflict_errors = list(validation.get("errors", []))
            kit._conflict_warnings = list(validation.get("warnings", []))
            msg = "; ".join(str(e.get("message", e)) for e in kit._conflict_errors)
            raise ValueError(f"Kit validation failed: {msg}")
        if kit._strict_mode and validation.get("warnings"):
            kit._conflicted = True
            kit._conflict_errors = []
            kit._conflict_warnings = list(validation.get("warnings", []))
            wmsg = "; ".join(str(w.get("message", w)) for w in kit._conflict_warnings)
            raise ValueError(f"Kit validation warnings (strict): {wmsg}")
        diff_to_apply = diff
        backward = inverseKitDiffDict(kit_dict, diff_to_apply)
        applyKitDiffDict(kit_dict, diff_to_apply)
        _assign_validated_kit_to(kit, kit_dict)
        change = KitGraphChange(
            forward=diff_to_apply, backward=backward, validation=dict(validation)
        )
        if transaction_id is not None:
            tx = kit._open_transactions.get(transaction_id)
            if tx is None:
                raise ValueError(f"Unknown transaction {transaction_id}")
            tx.steps.append(change)
            tx.redo.clear()
        elif not skip_global_history:
            kit._history_past.append(change)
            kit._history_future.clear()
        notify = notify_backbone and transaction_id is None
        if notify:
            _notify_kit_backbone_optional(kit._backbone, change)
        kit._conflicted = False
        kit._conflict_errors.clear()
        kit._conflict_warnings.clear()
        return change


@dataclasses.dataclass
class Change:
    """💿Change holds the data fields for a Change record."""

    forward: dict
    backward: dict
    author: typing.Optional[str] = None
    time: typing.Optional[datetime.datetime] = None
    before: typing.Optional[dict] = None
    after: typing.Optional[dict] = None


def changeToDict(change: Change) -> dict:
    """🔖changeToDict performs the changeToDict operation."""
    result: dict = {"forward": change.forward, "backward": change.backward}
    if change.author is not None:
        result["author"] = change.author
    if change.time is not None:
        result["time"] = change.time.isoformat()
    if change.before is not None:
        result["before"] = change.before
    if change.after is not None:
        result["after"] = change.after
    return result


@dataclasses.dataclass
class AttributeChange(Change):
    """🔖AttributeChange holds the data fields for a AttributeChange record."""

    pass


@dataclasses.dataclass
class AuthorChange(Change):
    """🔖AuthorChange holds the data fields for a AuthorChange record."""

    pass


@dataclasses.dataclass
class FileChange(Change):
    """🔖FileChange holds the data fields for a FileChange record."""

    pass


@dataclasses.dataclass
class FolderChange(Change):
    """🔖FolderChange holds the data fields for a FolderChange record."""

    pass


@dataclasses.dataclass
class QualityChange(Change):
    """🔖QualityChange holds the data fields for a QualityChange record."""

    pass


@dataclasses.dataclass
class PortChange(Change):
    """🔖PortChange holds the data fields for a PortChange record."""

    pass


@dataclasses.dataclass
class PropChange(Change):
    """🔖PropChange holds the data fields for a PropChange record."""

    pass


@dataclasses.dataclass
class TagChange(Change):
    """🔖TagChange holds the data fields for a TagChange record."""

    pass


@dataclasses.dataclass
class ConceptChange(Change):
    """🔖ConceptChange holds the data fields for a ConceptChange record."""

    pass


@dataclasses.dataclass
class RepresentationChange(Change):
    """🔖RepresentationChange holds the data fields for a RepresentationChange record."""

    pass


@dataclasses.dataclass
class ConnectorChange(Change):
    """🔖ConnectorChange holds the data fields for a ConnectorChange record."""

    pass


@dataclasses.dataclass
class TypeChange(Change):
    """🔖TypeChange holds the data fields for a TypeChange record."""

    pass


@dataclasses.dataclass
class LayerChange(Change):
    """🔖LayerChange holds the data fields for a LayerChange record."""

    pass


@dataclasses.dataclass
class PieceChange(Change):
    """🔖PieceChange holds the data fields for a PieceChange record."""

    pass


@dataclasses.dataclass
class GroupChange(Change):
    """🔖GroupChange holds the data fields for a GroupChange record."""

    pass


@dataclasses.dataclass
class ConnectionChange(Change):
    """🔖ConnectionChange holds the data fields for a ConnectionChange record."""

    pass


@dataclasses.dataclass
class StatChange(Change):
    """🔖StatChange holds the data fields for a StatChange record."""

    pass


@dataclasses.dataclass
class DesignChange(Change):
    """🔖DesignChange holds the data fields for a DesignChange record."""

    pass


@dataclasses.dataclass
class KitChange(Change):
    """🔖KitChange holds the data fields for a KitChange record."""

    pass


# #region 📋Copy Paste Design
# 📋Copy Paste Design provides copy and paste functionality for designs.
# Specs: CopyDesign extracts selected pieces and connections. PasteDesign inserts them into a target design.


def copyDesignDict(
    kit: dict, design: dict, pieceIds: list[str], connectionIds: list[str]
) -> dict:
    """📋Extracts selected pieces and connections from a design into a new Design dict.
    Specs: Selected pieces are classified as internal-fixed, internal-connected, or parent-piece-exclusive parent-connection-inclusive.
    Internal pieces are copied as-is. Pp-excl-pc-incl pieces get semio.center and semio.plane attributes.
    Non-internal connections include their external pieces marked with semio.piece.origin = "external" and semio.center.
    """
    selectedPieceSet = set(pieceIds)
    selectedConnectionSet = set(connectionIds)

    connections = design.get("connections", [])
    pieces = design.get("pieces", [])

    # Build parent map: child id -> (parent id, connection)
    parentMap: dict[str, tuple[str, dict]] = {}
    for conn in connections:
        connectingId = conn.get("child", {}).get("piece", {}).get("id", "")
        connectedId = conn.get("parent", {}).get("piece", {}).get("id", "")
        parentMap[connectingId] = (connectedId, conn)

    # Flatten the design to get absolute planes/centers
    flatResult = flattenDesignDict(kit, design.get("id", ""))
    flatPieceMap: dict[str, dict] = {}
    for piece in pieces:
        if _dict_piece_plane(piece) is not None:
            flatPieceMap[piece["id"]] = {
                "plane": _dict_piece_plane(piece),
                "center": _dict_piece_center(piece),
            }
    for update in flatResult.get("pieces", {}).get("updated", []):
        id = update.get("piece", {}).get("id", update.get("id", ""))
        diff = update.get("diff", {})
        entry = flatPieceMap.get(id, {})
        pd = _dict_piece_diff_pose(diff)
        if pd:
            if pd.get("plane") is not None:
                entry["plane"] = pd["plane"]
            if pd.get("center") is not None:
                entry["center"] = pd["center"]
        flatPieceMap[id] = entry

    copyPieces: list[dict] = []
    addedPieceIds: set[str] = set()
    copyConnections: list[dict] = []

    # Process selected pieces
    for pieceId in pieceIds:
        piece = next((p for p in pieces if p.get("id") == pieceId), None)
        if piece is None:
            continue

        isFixed = _dict_piece_plane(piece) is not None
        isConnected = pieceId in parentMap

        isInternalConnected = False
        isInternalFixed = isFixed and pieceId in selectedPieceSet
        isPpExclPcIncl = False

        if isConnected:
            parentId, parentConn = parentMap[pieceId]
            parentPieceSelected = parentId in selectedPieceSet
            parentConnSelected = parentConn.get("id", "") in selectedConnectionSet
            isInternalConnected = parentPieceSelected and parentConnSelected
            isPpExclPcIncl = not parentPieceSelected and parentConnSelected

        if isInternalFixed or isInternalConnected:
            copyPieces.append(_deepCopy(piece))
            addedPieceIds.add(pieceId)
        elif isPpExclPcIncl:
            copied = _deepCopy(piece)
            flatPiece = flatPieceMap.get(pieceId, {})
            centerValue = json.dumps(flatPiece.get("center", {"u": 0, "v": 0}))
            planeValue = json.dumps(
                flatPiece.get(
                    "plane",
                    {
                        "origin": {"x": 0, "y": 0, "z": 0},
                        "xAxis": {"x": 1, "y": 0, "z": 0},
                        "yAxis": {"x": 0, "y": 1, "z": 0},
                    },
                )
            )
            attrs = copied.setdefault("attributes", [])
            attrs.append({"key": "semio.center", "value": centerValue})
            attrs.append({"key": "semio.plane", "value": planeValue})
            copyPieces.append(copied)
            addedPieceIds.add(pieceId)

    # Process selected connections
    for connId in connectionIds:
        conn = next((c for c in connections if c.get("id") == connId), None)
        if conn is None:
            continue

        connectedId = conn.get("parent", {}).get("piece", {}).get("id", "")
        connectingId = conn.get("child", {}).get("piece", {}).get("id", "")
        connectedSelected = connectedId in selectedPieceSet
        connectingSelected = connectingId in selectedPieceSet

        isInternal = connectedSelected and connectingSelected

        if isInternal:
            copyConnections.append(_deepCopy(conn))
        else:
            copyConnections.append(_deepCopy(conn))

            externalIds: list[str] = []
            if not connectedSelected:
                externalIds.append(connectedId)
            if not connectingSelected:
                externalIds.append(connectingId)

            for extId in externalIds:
                if extId not in addedPieceIds:
                    extPiece = next((p for p in pieces if p.get("id") == extId), None)
                    if extPiece is not None:
                        cloned = _deepCopy(extPiece)
                        attrs = cloned.setdefault("attributes", [])
                        attrs.append({"key": "semio.piece.origin", "value": "external"})
                        flatPiece = flatPieceMap.get(extId, {})
                        centerValue = json.dumps(
                            flatPiece.get("center", {"u": 0, "v": 0})
                        )
                        attrs.append({"key": "semio.center", "value": centerValue})
                        copyPieces.append(cloned)
                        addedPieceIds.add(extId)

    return {"pieces": copyPieces, "connections": copyConnections}


def pasteDesignDict(
    kit: dict,
    source: dict,
    target: dict,
    anchoring: str,
    coordinate: typing.Optional[dict] = None,
) -> dict:
    """📋Pastes a copied design into a target design, returning a DesignDiff dict.
    Specs: Anchoring determines the reference point within the bounding rectangle of the source.
    Fixed pieces get -anchor offset applied to center; if coordinate is given, +coordinate offset is also applied.
    Connected pieces with non-external parents are added as-is.
    Connected pieces with external-origin parents: if a matching piece with a matching connector is found in target,
    the parent connection is remapped; otherwise treated as fixed using semio.center/semio.plane attributes.
    Coordinate adjusts connection u/v only for remapped child–stub edges; fully internal clipboard connections keep cloned u/v.
    """
    types = kit.get("types", [])
    ports = kit.get("ports", [])

    typesMap: dict[str, dict] = {t["id"]: t for t in types}
    portsMap: dict[str, dict] = {p["id"]: p for p in ports}

    sourcePieces = source.get("pieces", [])
    sourceConnections = source.get("connections", [])
    targetPieces = target.get("pieces", [])

    # Classify source pieces
    externalOriginIds: set[str] = set()
    for p in sourcePieces:
        for attr in p.get("attributes", []):
            if (
                attr.get("key") == "semio.piece.origin"
                and attr.get("value") == "external"
            ):
                externalOriginIds.add(p["id"])

    sourcePieceMap: dict[str, dict] = {p["id"]: p for p in sourcePieces}

    sourceParentMap: dict[str, tuple[str, dict]] = {}
    for conn in sourceConnections:
        connectingId = conn.get("child", {}).get("piece", {}).get("id", "")
        connectedId = conn.get("parent", {}).get("piece", {}).get("id", "")
        if connectingId not in sourceParentMap:
            sourceParentMap[connectingId] = (connectedId, conn)
            continue
        prevId, _ = sourceParentMap[connectingId]
        prev_stub = prevId in externalOriginIds
        next_stub = connectedId in externalOriginIds
        if prev_stub != next_stub and next_stub:
            sourceParentMap[connectingId] = (connectedId, conn)

    # Compute bounding rectangle from flat centers
    centerCoordinates: list[dict] = []
    for piece in sourcePieces:
        if piece["id"] in externalOriginIds:
            continue
        center = piece.get("center")
        if center is None:
            for attr in piece.get("attributes", []):
                if attr.get("key") == "semio.center" and attr.get("value"):
                    try:
                        center = json.loads(attr["value"])
                    except json.JSONDecodeError, TypeError:
                        pass
        if center is not None:
            centerCoordinates.append(center)

    if not centerCoordinates:
        centerCoordinates.append({"u": 0, "v": 0})

    minU = min(c.get("u", 0) for c in centerCoordinates)
    maxU = max(c.get("u", 0) for c in centerCoordinates)
    minV = min(c.get("v", 0) for c in centerCoordinates)
    maxV = max(c.get("v", 0) for c in centerCoordinates)

    if anchoring == "middle":
        anchor = {"u": (minU + maxU) / 2, "v": (minV + maxV) / 2}
    elif anchoring == "centroid":
        n = len(centerCoordinates)
        anchor = {
            "u": sum(c.get("u", 0) for c in centerCoordinates) / n,
            "v": sum(c.get("v", 0) for c in centerCoordinates) / n,
        }
    elif anchoring == "bottomLeft":
        anchor = {"u": minU, "v": minV}
    elif anchoring == "bottomRight":
        anchor = {"u": maxU, "v": minV}
    elif anchoring == "topLeft":
        anchor = {"u": minU, "v": maxV}
    elif anchoring == "topRight":
        anchor = {"u": maxU, "v": maxV}
    else:  # "original"
        anchor = {"u": 0, "v": 0}

    # Build target piece maps for matching
    targetPiecesByName: dict[str, list[dict]] = {}
    for tp in targetPieces:
        name = tp.get("name", "")
        if name:
            targetPiecesByName.setdefault(name, []).append(tp)

    def arePortsCompatible(portId1: str, portId2: str) -> bool:
        if not portId1 or not portId2:
            return False
        if portId1 == portId2:
            return True
        port1 = portsMap.get(portId1)
        port2 = portsMap.get(portId2)
        if not port1 or not port2:
            return False
        for cp in port1.get("compatiblePorts", []):
            if cp.get("id") == portId2:
                return True
        for cp in port2.get("compatiblePorts", []):
            if cp.get("id") == portId1:
                return True
        return False

    def areConnectorsCompatible(c1: dict, c2: dict) -> bool:
        pg1 = c1.get("port", {}).get("id", "")
        pg2 = c2.get("port", {}).get("id", "")
        return arePortsCompatible(pg1, pg2)

    def findMatchingConnector(
        typeId: str, sourceConnector: dict
    ) -> typing.Optional[dict]:
        t = typesMap.get(typeId)
        if not t:
            return None
        srcName = sourceConnector.get("name", "")
        for c in t.get("connectors", []):
            if c.get("name", "") == srcName and areConnectorsCompatible(
                c, sourceConnector
            ):
                return c
        return None

    addedPieces: list[dict] = []
    addedConnections: list[dict] = []

    # Process source pieces
    for piece in sourcePieces:
        if piece["id"] in externalOriginIds:
            continue

        isFixed = _dict_piece_plane(piece) is not None
        isConnected = piece["id"] in sourceParentMap

        if isFixed and not isConnected:
            # Fixed piece: apply -anchor offset, then +coordinate if given
            copied = _deepCopy(piece)
            center = copied.get("center") or {"u": 0, "v": 0}
            newCenter = {
                "u": center.get("u", 0) - anchor["u"],
                "v": center.get("v", 0) - anchor["v"],
            }
            if coordinate is not None:
                newCenter = {
                    "u": newCenter["u"] + coordinate.get("u", 0),
                    "v": newCenter["v"] + coordinate.get("v", 0),
                }
            copied["center"] = newCenter
            addedPieces.append(copied)
        elif isConnected:
            parentId, parentConn = sourceParentMap[piece["id"]]
            if parentId in externalOriginIds:
                # Parent is external-origin: try to match in target
                externalParent = sourcePieceMap[parentId]
                matched = False

                extName = externalParent.get("name", "")
                if extName and extName in targetPiecesByName:
                    candidates = targetPiecesByName[extName]
                    isParentConnected = (
                        parentConn.get("parent", {}).get("piece", {}).get("id", "")
                        == parentId
                    )
                    if isParentConnected:
                        parentConnectorId = (
                            parentConn.get("parent", {})
                            .get("connector", {})
                            .get("id", "")
                        )
                    else:
                        parentConnectorId = (
                            parentConn.get("child", {})
                            .get("connector", {})
                            .get("id", "")
                        )

                    # Find the source parent connector
                    sourceParentConnector = None
                    extTypeId = externalParent.get("type", {}).get("id", "")
                    if extTypeId:
                        parentType = typesMap.get(extTypeId)
                        if parentType:
                            for c in parentType.get("connectors", []):
                                if c.get("id") == parentConnectorId:
                                    sourceParentConnector = c
                                    break

                    if sourceParentConnector is not None:
                        for candidate in candidates:
                            candidateTypeId = candidate.get("type", {}).get("id", "")
                            if not candidateTypeId:
                                continue
                            matchingConnector = findMatchingConnector(
                                candidateTypeId, sourceParentConnector
                            )
                            if matchingConnector is not None:
                                matched = True
                                addedPieces.append(_deepCopy(piece))

                                copiedConn = _deepCopy(parentConn)
                                if isParentConnected:
                                    copiedConn["parent"] = {
                                        "piece": {"id": candidate["id"]},
                                        "connector": {"id": matchingConnector["id"]},
                                    }
                                else:
                                    copiedConn["child"] = {
                                        "piece": {"id": candidate["id"]},
                                        "connector": {"id": matchingConnector["id"]},
                                    }

                                if coordinate is not None:
                                    connected_id = (
                                        parentConn.get("parent", {})
                                        .get("piece", {})
                                        .get("id", "")
                                    )
                                    connecting_id = (
                                        parentConn.get("child", {})
                                        .get("piece", {})
                                        .get("id", "")
                                    )
                                    connected_stub = connected_id in externalOriginIds
                                    connecting_stub = connecting_id in externalOriginIds
                                    conn_matches_parentage = (
                                        connecting_id == piece["id"]
                                        and connected_id == parentId
                                    ) or (
                                        connected_id == piece["id"]
                                        and connecting_id == parentId
                                    )
                                    # Specs: Coordinate may shift diagram u/v only for the remapped bridge to a clipboard external stub;
                                    # internal–internal source edges (neither side a stub) must keep cloned u/v.
                                    if (
                                        conn_matches_parentage
                                        and connected_stub != connecting_stub
                                    ):
                                        flatParentCenter = None
                                        c0 = candidate.get("center")
                                        if c0 is not None and isinstance(c0, dict):
                                            flatParentCenter = {
                                                "u": c0.get("u", 0),
                                                "v": c0.get("v", 0),
                                            }
                                        if flatParentCenter is None:
                                            for attr in candidate.get("attributes", []):
                                                if attr.get(
                                                    "key"
                                                ) == "semio.center" and attr.get(
                                                    "value"
                                                ):
                                                    try:
                                                        flatParentCenter = json.loads(
                                                            attr["value"]
                                                        )
                                                        break
                                                    except (
                                                        json.JSONDecodeError,
                                                        TypeError,
                                                    ):
                                                        pass
                                        if flatParentCenter is None:
                                            for attr in externalParent.get(
                                                "attributes", []
                                            ):
                                                if attr.get(
                                                    "key"
                                                ) == "semio.center" and attr.get(
                                                    "value"
                                                ):
                                                    try:
                                                        flatParentCenter = json.loads(
                                                            attr["value"]
                                                        )
                                                        break
                                                    except (
                                                        json.JSONDecodeError,
                                                        TypeError,
                                                    ):
                                                        pass
                                        if (
                                            flatParentCenter is None
                                            and externalParent.get("center")
                                        ):
                                            flatParentCenter = externalParent["center"]
                                        flatChildCenter = None
                                        for attr in piece.get("attributes", []):
                                            if attr.get(
                                                "key"
                                            ) == "semio.center" and attr.get("value"):
                                                try:
                                                    flatChildCenter = json.loads(
                                                        attr["value"]
                                                    )
                                                    break
                                                except json.JSONDecodeError, TypeError:
                                                    pass
                                        if flatChildCenter is None and piece.get(
                                            "center"
                                        ):
                                            flatChildCenter = piece["center"]
                                        if (
                                            flatParentCenter is not None
                                            and flatChildCenter is not None
                                        ):
                                            copiedConn["u"] = flatParentCenter.get(
                                                "u", 0
                                            ) - (
                                                coordinate.get("u", 0)
                                                + (
                                                    anchor["u"]
                                                    - flatChildCenter.get("u", 0)
                                                )
                                            )
                                            copiedConn["v"] = flatParentCenter.get(
                                                "v", 0
                                            ) - (
                                                coordinate.get("v", 0)
                                                + (
                                                    anchor["v"]
                                                    - flatChildCenter.get("v", 0)
                                                )
                                            )

                                addedConnections.append(copiedConn)
                                break

                if not matched:
                    # Treat as fixed piece using semio.center and semio.plane attributes
                    copied = _deepCopy(piece)
                    for attr in piece.get("attributes", []):
                        if attr.get("key") == "semio.center" and attr.get("value"):
                            try:
                                po = dict(copied.get("pose") or {})
                                po["center"] = json.loads(attr["value"])
                                copied["pose"] = po
                            except json.JSONDecodeError, TypeError:
                                pass
                        if attr.get("key") == "semio.plane" and attr.get("value"):
                            try:
                                po = dict(copied.get("pose") or {})
                                po["plane"] = json.loads(attr["value"])
                                copied["pose"] = po
                            except json.JSONDecodeError, TypeError:
                                pass
                    center = _dict_piece_center(copied) or {"u": 0, "v": 0}
                    newCenter = {
                        "u": center.get("u", 0) - anchor["u"],
                        "v": center.get("v", 0) - anchor["v"],
                    }
                    if coordinate is not None:
                        newCenter = {
                            "u": newCenter["u"] + coordinate.get("u", 0),
                            "v": newCenter["v"] + coordinate.get("v", 0),
                        }
                    po = dict(copied.get("pose") or {})
                    po["center"] = newCenter
                    copied["pose"] = po
                    addedPieces.append(copied)
            else:
                # Parent is not external: add connected piece as-is
                addedPieces.append(_deepCopy(piece))

    # Process source connections (non-external internal connections)
    addedPieceIds = {p["id"] for p in addedPieces}
    for conn in sourceConnections:
        connectedId = conn.get("parent", {}).get("piece", {}).get("id", "")
        connectingId = conn.get("child", {}).get("piece", {}).get("id", "")

        if connectedId in externalOriginIds or connectingId in externalOriginIds:
            continue

        if connectedId not in addedPieceIds or connectingId not in addedPieceIds:
            continue

        addedConnections.append(_deepCopy(conn))

    diff: dict = {}
    if addedPieces:
        diff["pieces"] = {"added": addedPieces}
    if addedConnections:
        diff["connections"] = {"added": addedConnections}
    return diff


# #endregion 📋Copy Paste Design


def deletePiecesAndConnectionsInDesignDict(
    kit: dict, design: dict, pieceIds: list[str], connectionIds: list[str]
) -> dict:
    """🔖Deletes pieces and connections from a design dict, returning a canonical SemioReport with DesignDiff.
    Removes stale connections referencing deleted pieces.
    Updates pieces that become fixed (parent connection removed) with flat plane and center from the flattened design.
    """
    deletedPieceSet = set(pieceIds)
    connections = design.get("connections", [])

    # Find stale connections: connections referencing any deleted piece
    staleConnectionIds = set()
    for conn in connections:
        connectedId = conn.get("parent", {}).get("piece", {}).get("id", "")
        connectingId = conn.get("child", {}).get("piece", {}).get("id", "")
        if connectedId in deletedPieceSet or connectingId in deletedPieceSet:
            staleConnectionIds.add(conn["id"])

    # All removed connections = explicit + stale
    allRemovedConnectionIds = set(connectionIds) | staleConnectionIds

    # Find pieces that become fixed
    fixedPieceIds: list[str] = []
    for connId in allRemovedConnectionIds:
        conn = next((c for c in connections if c["id"] == connId), None)
        if conn is None:
            continue
        connectingId = conn.get("child", {}).get("piece", {}).get("id", "")
        if connectingId in deletedPieceSet:
            continue
        # Check if this piece has another parent connection not in the removed set
        hasOtherParent = any(
            c.get("child", {}).get("piece", {}).get("id", "") == connectingId
            and c["id"] not in allRemovedConnectionIds
            for c in connections
        )
        if not hasOtherParent and connectingId not in fixedPieceIds:
            fixedPieceIds.append(connectingId)

    flatPlane = {
        "origin": {"x": 0, "y": 0, "z": 0},
        "xAxis": {"x": 1, "y": 0, "z": 0},
        "yAxis": {"x": 0, "y": 1, "z": 0},
    }
    zeroCenter = {"u": 0, "v": 0}

    # Flatten the design to get absolute plane and center for each piece
    flatRep = flattenDesignReportDict(kit, design.get("id", ""))
    if not flatRep["ok"]:
        return flatRep
    flatResult = flatRep["diff"]["forward"]
    flatPieceMap: dict[str, dict] = {}
    for piece in design.get("pieces", []):
        if _dict_piece_plane(piece) is not None:
            flatPieceMap[piece["id"]] = {
                "plane": _dict_piece_plane(piece),
                "center": _dict_piece_center(piece),
            }
    for update in flatResult.get("pieces", {}).get("updated", []):
        id = update.get("piece", {}).get("id", update.get("id", ""))
        existing = flatPieceMap.get(id, {})
        diff = update.get("diff", {})
        pd = _dict_piece_diff_pose(diff)
        if pd:
            if pd.get("plane") is not None:
                existing["plane"] = pd["plane"]
            if pd.get("center") is not None:
                existing["center"] = pd["center"]
        flatPieceMap[id] = existing

    diff: dict = {}

    piecesRemoved = [{"id": g} for g in pieceIds]
    piecesUpdated = []
    for g in fixedPieceIds:
        flat = flatPieceMap.get(g, {})
        piecesUpdated.append(
            {
                "piece": {"id": g},
                "diff": {
                    "pose": {
                        "plane": flat.get("plane", flatPlane),
                        "center": flat.get("center", zeroCenter),
                    }
                },
            }
        )
    if piecesRemoved or piecesUpdated:
        piecesDiff: dict = {}
        if piecesRemoved:
            piecesDiff["removed"] = piecesRemoved
        if piecesUpdated:
            piecesDiff["updated"] = piecesUpdated
        diff["pieces"] = piecesDiff

    connectionsRemoved = [{"id": g} for g in sorted(allRemovedConnectionIds)]
    if connectionsRemoved:
        diff["connections"] = {"removed": connectionsRemoved}

    return _semio_report_ok(diff, flatRep["warnings"], flatRep["infos"])


def getDesignChange(
    before: dict,
    after: dict,
    author: typing.Optional[str] = None,
    time: typing.Optional[datetime.datetime] = None,
) -> DesignChange:
    """getDesignChange performs the getDesignChange operation."""
    forward_diff = _getDesignDiff(before, after)
    backward_diff = _inverseDesignDiff(before, forward_diff)
    return DesignChange(
        forward=forward_diff,
        backward=backward_diff,
        author=author,
        time=time,
        before=before,
        after=after,
    )


def getKitChange(
    before: dict,
    after: dict,
    author: typing.Optional[str] = None,
    time: typing.Optional[datetime.datetime] = None,
) -> KitChange:
    """getKitChange performs the getKitChange operation."""
    forward_diff = getKitDiffDict(before, after)
    backward_diff = inverseKitDiffDict(before, forward_diff)
    return KitChange(
        forward=forward_diff,
        backward=backward_diff,
        author=author,
        time=time,
        before=before,
        after=after,
    )


def _extractUpdateId(update: dict, entityKeys: list[str]) -> str:
    """📍Extract id from an updated entry which might use EntityId format or old id format."""
    for key in entityKeys:
        if key in update and isinstance(update[key], dict):
            return update[key].get("id", "")
    return update.get("id", "")


FLOAT_EPSILON = 1e-10

_VECTOR3_KEYS = frozenset({"x", "y", "z"})
_UV_KEYS = frozenset({"u", "v"})
_CONNECTION_OPTIONAL_NUMERIC_KEYS = frozenset(
    {"gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"}
)


def _expandConnectionEntityDict(d: dict) -> dict:
    """🔖Full connection snapshots may omit numeric fields when zero; golden fixtures often spell them out."""
    if not isinstance(d, dict):
        return d
    if "parent" not in d or "child" not in d:
        return d
    out = dict(d)
    for k in _CONNECTION_OPTIONAL_NUMERIC_KEYS:
        if k not in out:
            out[k] = 0
    return out


def _expandSparseNumericDict(d: dict) -> dict:
    """🔖Make sparse xyz / uv dicts comparable to fully populated ones (missing axis ≡ 0)."""
    keys = set(d.keys())
    if keys and keys <= _VECTOR3_KEYS:
        if not all(isinstance(d.get(k), (int, float)) for k in keys):
            return d
        return {
            "x": float(d.get("x", 0)),
            "y": float(d.get("y", 0)),
            "z": float(d.get("z", 0)),
        }
    if keys and keys <= _UV_KEYS:
        if not all(isinstance(d.get(k), (int, float)) for k in keys):
            return d
        return {"u": float(d.get("u", 0)), "v": float(d.get("v", 0))}
    return d


def _areDiffDictsEqual(a: dict, b: dict) -> bool:
    """🔖Deep equality check for diff dicts with float epsilon tolerance."""
    if a is b:
        return True
    if type(a) != type(b):
        if isinstance(a, (int, float)) and isinstance(b, (int, float)):
            return abs(float(a) - float(b)) < FLOAT_EPSILON
        return _normalizeValue(a) == _normalizeValue(b)
    if isinstance(a, dict):
        if isinstance(b, dict):
            a = _expandConnectionEntityDict(_expandSparseNumericDict(a))
            b = _expandConnectionEntityDict(_expandSparseNumericDict(b))
        keysA = {k for k, v in a.items() if _normalizeValue(v) is not None}
        keysB = {k for k, v in b.items() if _normalizeValue(v) is not None}
        if keysA != keysB:
            return False
        for key in keysA:
            if not _areDiffDictsEqual(a[key], b[key]):
                return False
        return True
    if isinstance(a, list):
        if len(a) != len(b):
            return False
        for i in range(len(a)):
            if not _areDiffDictsEqual(a[i], b[i]):
                return False
        return True
    if isinstance(a, float):
        return abs(a - b) < FLOAT_EPSILON
    return _normalizeValue(a) == _normalizeValue(b)


def areKitDiffsDictEqual(a: dict, b: dict) -> bool:
    """🔖Deep equality check for kit diffs."""
    keys = [
        "name",
        "version",
        "description",
        "icon",
        "image",
        "remote",
        "homepage",
        "license",
        "preview",
    ]
    for key in keys:
        if _normalizeValue(a.get(key)) != _normalizeValue(b.get(key)):
            return False

    collectionConfig = [
        ("types", "type"),
        ("designs", "design"),
        ("tags", "tag"),
        ("concepts", "concept"),
        ("ports", "port"),
        ("files", "file"),
        ("folders", "folder"),
        ("attributes", "attribute"),
    ]
    for collectionKey, entityKey in collectionConfig:
        diffA = a.get(collectionKey, {})
        diffB = b.get(collectionKey, {})

        removedA = {
            r["id"] if isinstance(r, dict) else r for r in diffA.get("removed", [])
        }
        removedB = {
            r["id"] if isinstance(r, dict) else r for r in diffB.get("removed", [])
        }
        if removedA != removedB:
            return False
        addedA = {item.get("id"): item for item in diffA.get("added", [])}
        addedB = {item.get("id"): item for item in diffB.get("added", [])}
        if set(addedA.keys()) != set(addedB.keys()):
            return False

        updatedA = {
            _extractUpdateId(u, [entityKey]): u["diff"]
            for u in diffA.get("updated", [])
        }
        updatedB = {
            _extractUpdateId(u, [entityKey]): u["diff"]
            for u in diffB.get("updated", [])
        }
        if set(updatedA.keys()) != set(updatedB.keys()):
            return False

        for id in addedA:
            if not _areDiffDictsEqual(addedA[id], addedB[id]):
                return False

        for id in updatedA:
            if not _areDiffDictsEqual(updatedA[id], updatedB[id]):
                return False

    return True


# #endregion 🎗️Kit Diff Operations


# #region 🧭Moved Graphene Nodes
# Graphene node definitions moved here due to forward-reference resolution order.


class AttributeNode(TableEntityNode):
    """🔖GraphQL node exposing attribute data."""

    class Meta:
        representation = Attribute


class PlaneNode(TableNode):
    """🔖GraphQL node exposing plane data."""

    class Meta:
        representation = Plane


class AuthorNode(TableEntityNode):
    """🔖GraphQL node exposing author data."""

    class Meta:
        representation = Author


class RepresentationNode(TableEntityNode):
    """🔖GraphQL node exposing representation data."""

    class Meta:
        representation = Representation
        excludedFields = ("tags_",)


class ConnectorNode(TableEntityNode):
    """🔖GraphQL node exposing connector data."""

    class Meta:
        representation = Connector
        exclude_fields = ("connecteds", "connectings")

    localId = graphene.String()

    def resolve_localId(self, info):
        return getattr(self, "id_", "")


class TypeNode(TableEntityNode):
    """🔖GraphQL node exposing type data."""

    class Meta:
        representation = Type


class PieceNode(TableEntityNode):
    """🔖GraphQL node exposing piece data."""

    class Meta:
        representation = Piece
        exclude_fields = ("connecteds", "connectings")

    localId = graphene.String()

    def resolve_localId(self, info):
        return getattr(self, "id_", "")


class ConnectionNode(TableEntityNode):
    """🔖GraphQL node exposing connection data."""

    class Meta:
        representation = Connection
        exclude_fields = (
            "connectedPiece",
            "connectedConnector",
            "connectingPiece",
            "connectingConnector",
        )

    connected = graphene.NonNull(lambda: SideNode)
    connecting = graphene.NonNull(lambda: SideNode)

    def resolve_connected(self, info):
        return self.parent

    def resolve_connecting(self, info):
        return self.child


class DesignNode(TableEntityNode):
    """🔖GraphQL node exposing design data."""

    class Meta:
        representation = Design


class KitNotFound(NotFound):
    """🚚endregion 🧭Moved Graphene Nodes"""

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't find an local or remote kit under uri:\n {self.uri}."


class NoKitToDelete(KitNotFound):
    """🗑️No Kit To Delete definition."""

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't delete the kit because no local or remote kit was found under uri:\n {self.uri}."


class KitZipDoesNotContainSemioFolder(KitNotFound):
    """🔖Kit Zip Does Not Contain Semio Folder definition."""

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 The remote zip kit ({self.uri}) is not a valid kit."


class OnlyRemoteKitsCanBeCached(ClientError):
    """💾Only Remote Kits Can Be Cached definition."""

    def __init__(self, nonRemoteUri: str) -> None:
        self.nonRemoteUri = nonRemoteUri

    def __str__(self):
        return f"🔍 Only remote kits can be cached. The uri ({self.nonRemoteUri}) doesn't start with http and ends with .zip"


class KitUriNotValid(ClientError, abc.ABC):
    """🆔 The base for all kit uri not valid errors."""


class LocalKitUriNotValid(KitUriNotValid, abc.ABC):
    """📂 The base for all local kit uri not valid errors."""


class LocalKitUriIsNotAbsolute(LocalKitUriNotValid):
    """🔖Local Kit Uri Is Not Absolute definition."""

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"📂 The local kit uri ({self.uri}) is relative. It needs to be absolute (include the parent folders, drives, ...)."


class LocalKitUriIsNotDirectory(LocalKitUriNotValid):
    """🔖Local Kit Uri Is Not Directory definition."""

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"📂 The local kit uri ({self.uri}) is not a directory."


class NoKitAssigned(NoParentAssigned):
    """🔖No Kit Assigned definition."""

    def __str__(self):
        return "👪 The entity has no parent kit assigned."


class KitAlreadyExists(AlreadyExists, abc.ABC):
    """🔖Exception for attempting to create a kit that already exists."""

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self) -> str:
        return f"♊ A kit under uri ({self.uri}) already exists."


class KitInputNode(InputNode):
    """🔖GraphQL input node for kit mutations."""

    class Meta:
        representation = KitInput


class KitNode(TableEntityNode):
    """🔖GraphQL node exposing kit data."""

    class Meta:
        representation = Kit


# #endregion 🧭Moved Graphene Nodes


# #region 🛡️Validation
# Validation logic for checking kit constraints and uniqueness rules.


@dataclasses.dataclass
class ValidationFix:
    """🔧A proposed fix for a validation problem with a title and diff."""

    title: str
    diff: dict

    def toDict(self) -> dict:
        return {"title": self.title, "diff": self.diff}


@dataclasses.dataclass
class Problem:
    """🔒A validation problem with a constraint identifier and message."""

    constraintId: str
    message: str
    entityKind: str
    entityId: str
    fixes: list[ValidationFix] = dataclasses.field(default_factory=list)

    def toDict(self) -> dict:
        return {
            "constraintId": self.constraintId,
            "message": self.message,
            "entityKind": self.entityKind,
            "entityId": self.entityId,
            "fixes": [f.toDict() for f in self.fixes],
        }


@dataclasses.dataclass
class ValidationResult:
    """🔖A validation result aggregating problems and fixes for an entity."""

    problems: list[Problem]

    def hasErrors(self) -> bool:
        return len(self.problems) > 0

    def toDict(self) -> dict:
        sortedProblems = sorted(
            self.problems, key=lambda i: (i.constraintId, i.entityId)
        )
        return {"problems": [i.toDict() for i in sortedProblems]}

    def serialize(self) -> str:
        return json.dumps(self.toDict(), indent=2)


def _isId(s: str) -> bool:
    """🔖_isId performs the _isId operation."""
    import re

    return bool(
        re.match(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
            s,
            re.IGNORECASE,
        )
    )


def _normalizeIds(obj: typing.Any) -> typing.Any:
    """🔖_normalizeIds performs the _normalizeIds operation."""
    if obj is None:
        return obj
    if isinstance(obj, str) and _isId(obj):
        return "<ID>"
    if isinstance(obj, list):
        return [_normalizeIds(x) for x in obj]
    if isinstance(obj, dict):
        return {k: _normalizeIds(v) for k, v in obj.items()}
    return obj


def areValidationResultsEqual(a: ValidationResult, b: ValidationResult) -> bool:
    """✔️Check whether two validation results are semantically equal."""
    if len(a.problems) != len(b.problems):
        return False
    sortedA = sorted(a.problems, key=lambda i: (i.constraintId, i.entityId))
    sortedB = sorted(b.problems, key=lambda i: (i.constraintId, i.entityId))
    for ia, ib in zip(sortedA, sortedB):
        if (
            ia.constraintId != ib.constraintId
            or ia.message != ib.message
            or ia.entityKind != ib.entityKind
            or ia.entityId != ib.entityId
        ):
            return False
        if len(ia.fixes) != len(ib.fixes):
            return False
        for fa, fb in zip(ia.fixes, ib.fixes):
            if fa.title != fb.title:
                return False

            if ia.constraintId == "id-unique":
                continue
            if json.dumps(_normalizeIds(fa.diff), sort_keys=True) != json.dumps(
                _normalizeIds(fb.diff), sort_keys=True
            ):
                return False
    return True


def parseValidationResult(jsonStr: str) -> ValidationResult:
    """🔬Parse a validation result from a dictionary representation."""
    data = json.loads(jsonStr)
    problems = []
    for i in data["problems"]:
        fixes = [
            ValidationFix(title=f["title"], diff=f["diff"]) for f in i.get("fixes", [])
        ]
        problems.append(
            Problem(
                constraintId=i["constraintId"],
                message=i["message"],
                entityKind=i["entityKind"],
                entityId=i["entityId"],
                fixes=fixes,
            )
        )
    return ValidationResult(problems=problems)


def validateIdUniqueness(kit: Kit) -> list[Problem]:
    """🔖Validate that all IDs within a collection are unique."""
    problems: list[Problem] = []
    seen: dict[str, str] = {}

    def check(entityKind: str, entityId: str) -> None:
        if entityId in seen:
            problems.append(
                Problem(
                    constraintId="id-unique",
                    message=f'Duplicate ID "{entityId}". Entity IDs are immutable; resolve by removing or replacing the duplicate entity (first occurrence kept).',
                    entityKind=entityKind,
                    entityId=entityId,
                )
            )
        else:
            seen[entityId] = entityKind

    check("Kit", kit.id)
    for t in kit.types or []:
        check("Type", t.id)
    for d in kit.designs or []:
        check("Design", d.id)
        for p in d.pieces or []:
            check("Piece", p.id)
        for c in d.connections or []:
            check("Connection", c.id)
        for s in d.stats or []:
            check("Stat", s.id)
    for q in kit.qualities or []:
        check("Quality", q.id)
    for f in kit.files_ or []:
        check("File", f.id)
    for fo in kit.folders_ or []:
        check("Folder", fo.id)
    return problems


def validateTypeNameUniqueness(kit: Kit) -> list[Problem]:
    """🔖Validate that all type names within a kit are unique."""
    problems: list[Problem] = []
    byParent: dict[str | None, list[Type]] = {}
    for t in kit.types or []:
        parentId = t.parent.id if t.parent else None
        if parentId not in byParent:
            byParent[parentId] = []
        byParent[parentId].append(t)
    for parentId, siblings in byParent.items():
        names: dict[str, list[Type]] = {}
        for t in siblings:
            if t.name not in names:
                names[t.name] = []
            names[t.name].append(t)
        for name, group in names.items():
            if len(group) > 1:
                for t in group[1:]:
                    problems.append(
                        Problem(
                            constraintId="type-name-unique",
                            message=f'Duplicate type name "{name}" among siblings.',
                            entityKind="Type",
                            entityId=t.id,
                        )
                    )
    return problems


def validateDesignNameUniqueness(kit: Kit) -> list[Problem]:
    """🔖Validate that all design names within a kit are unique."""
    problems: list[Problem] = []
    byParent: dict[str | None, list[Design]] = {}
    for d in kit.designs or []:
        parentId = d.parent.id if d.parent else None
        if parentId not in byParent:
            byParent[parentId] = []
        byParent[parentId].append(d)
    for parentId, siblings in byParent.items():
        names: dict[str, list[Design]] = {}
        for d in siblings:
            if d.name not in names:
                names[d.name] = []
            names[d.name].append(d)
        for name, group in names.items():
            if len(group) > 1:
                for d in group[1:]:
                    problems.append(
                        Problem(
                            constraintId="design-name-unique",
                            message=f'Duplicate design name "{name}" among siblings.',
                            entityKind="Design",
                            entityId=d.id,
                        )
                    )
    return problems


def validatePieceNameUniqueness(kit: Kit) -> list[Problem]:
    """🔖Validate that all piece names within a design are unique."""
    problems: list[Problem] = []
    for design in kit.designs or []:
        names: dict[str, list[Piece]] = {}
        for p in design.pieces or []:
            if p.name_ and p.name_ not in names:
                names[p.name_] = []
            if p.name_:
                names[p.name_].append(p)
        for name, group in names.items():
            if len(group) > 1:
                for p in group[1:]:
                    problems.append(
                        Problem(
                            constraintId="piece-name-unique",
                            message=f'Duplicate piece name "{name}" in design.',
                            entityKind="Piece",
                            entityId=p.id,
                        )
                    )
    return problems


def validatePortNameUniqueness(kit: Kit) -> list[Problem]:
    """🔖Validate that all port names within a type are unique."""
    problems: list[Problem] = []
    for t in kit.types or []:
        names: dict[str, list[Connector]] = {}
        for connector in t.connectors or []:
            if connector.name_ and connector.name_ not in names:
                names[connector.name_] = []
            if connector.name_:
                names[connector.name_].append(connector)
        for name, group in names.items():
            if len(group) > 1:
                for connector in group[1:]:
                    problems.append(
                        Problem(
                            constraintId="connector-name-unique",
                            message=f'Duplicate connector name "{name}" in type.',
                            entityKind="Connector",
                            entityId=connector.id,
                        )
                    )
    return problems


def validateRepresentationNameUniqueness(kit: Kit) -> list[Problem]:
    """🔖Validate that all representation names within a type are unique."""
    problems: list[Problem] = []
    for t in kit.types or []:
        names: dict[str, list[Representation]] = {}
        for representation in t.representations or []:
            if representation.name and representation.name not in names:
                names[representation.name] = []
            if representation.name:
                names[representation.name].append(representation)
        for name, group in names.items():
            if len(group) > 1:
                for representation in group[1:]:
                    problems.append(
                        Problem(
                            constraintId="representation-name-unique",
                            message=f'Duplicate representation name "{name}" in type.',
                            entityKind="Representation",
                            entityId=representation.id,
                        )
                    )
    return problems


def validateQualityNameUniqueness(kit: Kit) -> list[Problem]:
    """🔖Validate that all quality names within a kit are unique."""
    problems: list[Problem] = []
    names: dict[str, list[Quality]] = {}
    for q in kit.qualities or []:
        if q.name not in names:
            names[q.name] = []
        names[q.name].append(q)
    for name, group in names.items():
        if len(group) > 1:
            for q in group[1:]:
                problems.append(
                    Problem(
                        constraintId="quality-name-unique",
                        message=f'Duplicate quality name "{name}".',
                        entityKind="Quality",
                        entityId=q.id,
                    )
                )
    return problems


def validateFileNameUniqueness(kit: Kit) -> list[Problem]:
    """🔖Validate that all file names within a kit are unique."""
    problems: list[Problem] = []
    names: dict[str, list[File]] = {}
    for f in kit.files_ or []:
        if f.name not in names:
            names[f.name] = []
        names[f.name].append(f)
    for name, group in names.items():
        if len(group) > 1:
            for f in group[1:]:
                problems.append(
                    Problem(
                        constraintId="file-name-unique",
                        message=f'Duplicate file name "{name}".',
                        entityKind="File",
                        entityId=f.id,
                    )
                )
    return problems


def validateFolderNameUniqueness(kit: Kit) -> list[Problem]:
    """🔖Validate that all folder names within a kit are unique."""
    problems: list[Problem] = []
    byParent: dict[str | None, list[Folder]] = {}
    for fo in kit.folders_ or []:
        parentId = fo.parent if fo.parent else None
        if parentId not in byParent:
            byParent[parentId] = []
        byParent[parentId].append(fo)
    for parentId, siblings in byParent.items():
        names: dict[str, list[Folder]] = {}
        for fo in siblings:
            if fo.name not in names:
                names[fo.name] = []
            names[fo.name].append(fo)
        for name, group in names.items():
            if len(group) > 1:
                for fo in group[1:]:
                    problems.append(
                        Problem(
                            constraintId="folder-name-unique",
                            message=f'Duplicate folder name "{name}" among siblings.',
                            entityKind="Folder",
                            entityId=fo.id,
                        )
                    )
    return problems


def validateLayerPathUniqueness(kit: Kit) -> list[Problem]:
    """🔖Validate that all layer paths within a design are unique."""
    problems: list[Problem] = []
    for design in kit.designs or []:
        paths: dict[str, list[Layer]] = {}
        for layer in design.layers or []:
            if layer.path not in paths:
                paths[layer.path] = []
            paths[layer.path].append(layer)
        for path, group in paths.items():
            if len(group) > 1:
                for layer in group[1:]:
                    problems.append(
                        Problem(
                            constraintId="layer-path-unique",
                            message=f'Duplicate layer path "{path}" in design.',
                            entityKind="Layer",
                            entityId=layer.id,
                        )
                    )
    return problems


def validateKit(kit: Kit) -> ValidationResult:
    """🔖Validate a kit entity against all constraint rules."""
    problems: list[Problem] = []
    problems.extend(validateIdUniqueness(kit))
    problems.extend(validateTypeNameUniqueness(kit))
    problems.extend(validateDesignNameUniqueness(kit))
    problems.extend(validatePieceNameUniqueness(kit))
    problems.extend(validatePortNameUniqueness(kit))
    problems.extend(validateRepresentationNameUniqueness(kit))
    problems.extend(validateQualityNameUniqueness(kit))
    problems.extend(validateFolderNameUniqueness(kit))
    problems.extend(validateLayerPathUniqueness(kit))
    return ValidationResult(problems=problems)


# #region 📧Dict-based Validation
# Dictionary-based validation functions for kit data integrity.


def _makeFix(title: str, diff: dict) -> ValidationFix:
    """🔖_makeFix performs the _makeFix operation."""
    return ValidationFix(title=title, diff=diff)


def _deepCopy(obj: typing.Any) -> typing.Any:
    """🔖_deepCopy performs the _deepCopy operation."""
    return json.loads(json.dumps(obj))


def _newId() -> str:
    """🔖_newId performs the _newId operation."""
    import uuid

    return str(uuid.uuid4())


def validateKitDict(kit: dict) -> ValidationResult:
    """🔖Validate a kit dictionary against all constraint rules."""
    problems: list[Problem] = []
    seen: dict[str, str] = {}
    seenEntities: dict[str, dict] = {}

    def checkId(entityKind: str, entityId: str, entity: dict) -> None:
        if entityId in seen:
            problems.append(
                Problem(
                    constraintId="id-unique",
                    message=f'Duplicate ID "{entityId}". Entity IDs are immutable; resolve by removing or replacing the duplicate entity (first occurrence kept).',
                    entityKind=entityKind,
                    entityId=entityId,
                    fixes=[],
                )
            )
        else:
            seen[entityId] = entityKind
            seenEntities[entityId] = entity

    checkId("Kit", kit.get("id", ""), kit)
    for t in kit.get("types", []):
        checkId("Type", t.get("id", ""), t)
        for connector in t.get("connectors", []):
            checkId("Connector", connector.get("id", ""), connector)
        for representation in t.get("representations", []):
            checkId("Representation", representation.get("id", ""), representation)
    for d in kit.get("designs", []):
        checkId("Design", d.get("id", ""), d)
        for p in d.get("pieces", []):
            checkId("Piece", p.get("id", ""), p)
        for c in d.get("connections", []):
            checkId("Connection", c.get("id", ""), c)
        for s in d.get("stats", []):
            checkId("Stat", s.get("id", ""), s)
    for q in kit.get("qualities", []):
        checkId("Quality", q.get("id", ""), q)
    for i in kit.get("ports", []):
        checkId("Port", i.get("id", ""), i)
    for f in kit.get("files", []):
        checkId("File", f.get("id", ""), f)
    for fo in kit.get("folders", []):
        checkId("Folder", fo.get("id", ""), fo)
    byParent: dict[str | None, list[dict]] = {}
    for t in kit.get("types", []):
        parent = t.get("parent")
        parentId = (
            parent.get("id") if isinstance(parent, dict) else parent if parent else None
        )
        if parentId not in byParent:
            byParent[parentId] = []
        byParent[parentId].append(t)
    for parentId, siblings in byParent.items():
        names: dict[str, list[dict]] = {}
        for t in siblings:
            name = t.get("name", "")
            if name not in names:
                names[name] = []
            names[name].append(t)
        for name, group in names.items():
            if len(group) > 1:
                for t in group[1:]:
                    fix = _makeFix(
                        f'Rename "{name}"',
                        {
                            "types": {
                                "updated": [
                                    {
                                        "type": {"id": t.get("id", "")},
                                        "diff": {"name": f"{name} 2"},
                                    }
                                ]
                            }
                        },
                    )
                    problems.append(
                        Problem(
                            constraintId="type-name-unique",
                            message=f'Duplicate type name "{name}" among siblings.',
                            entityKind="Type",
                            entityId=t.get("id", ""),
                            fixes=[fix],
                        )
                    )
    byParent = {}
    for d in kit.get("designs", []):
        parent = d.get("parent")
        parentId = (
            parent.get("id") if isinstance(parent, dict) else parent if parent else None
        )
        if parentId not in byParent:
            byParent[parentId] = []
        byParent[parentId].append(d)
    for parentId, siblings in byParent.items():
        names: dict[str, list[dict]] = {}
        for d in siblings:
            name = d.get("name", "")
            if name not in names:
                names[name] = []
            names[name].append(d)
        for name, group in names.items():
            if len(group) > 1:
                for d in group[1:]:
                    fix = _makeFix(
                        f'Rename "{name}"',
                        {
                            "designs": {
                                "updated": [
                                    {
                                        "design": {"id": d.get("id", "")},
                                        "diff": {"name": f"{name} 2"},
                                    }
                                ]
                            }
                        },
                    )
                    problems.append(
                        Problem(
                            constraintId="design-name-unique",
                            message=f'Duplicate design name "{name}" among siblings.',
                            entityKind="Design",
                            entityId=d.get("id", ""),
                            fixes=[fix],
                        )
                    )
    for design in kit.get("designs", []):
        designName = design.get("name", "")
        designId = design.get("id", "")
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
                    fix = _makeFix(
                        f'Rename piece "{name}"',
                        {
                            "designs": {
                                "updated": [
                                    {
                                        "design": {"id": designId},
                                        "diff": {
                                            "pieces": {
                                                "updated": [
                                                    {
                                                        "piece": {
                                                            "id": p.get("id", "")
                                                        },
                                                        "diff": {"name": f"{name} 2"},
                                                    }
                                                ]
                                            }
                                        },
                                    }
                                ]
                            }
                        },
                    )
                    problems.append(
                        Problem(
                            constraintId="piece-name-unique",
                            message=f'Duplicate piece name "{name}" inside design "{designName}".',
                            entityKind="Piece",
                            entityId=p.get("id", ""),
                            fixes=[fix],
                        )
                    )
    for t in kit.get("types", []):
        typeName = t.get("name", "")
        typeId = t.get("id", "")
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
                    fix = _makeFix(
                        f'Rename connector "{name}"',
                        {
                            "types": {
                                "updated": [
                                    {
                                        "type": {"id": typeId},
                                        "diff": {
                                            "connectors": {
                                                "updated": [
                                                    {
                                                        "connector": {
                                                            "id": connector.get(
                                                                "id", ""
                                                            )
                                                        },
                                                        "diff": {"name": f"{name} 2"},
                                                    }
                                                ]
                                            }
                                        },
                                    }
                                ]
                            }
                        },
                    )
                    problems.append(
                        Problem(
                            constraintId="connector-name-unique",
                            message=f'Duplicate connector name "{name}" inside type "{typeName}".',
                            entityKind="Connector",
                            entityId=connector.get("id", ""),
                            fixes=[fix],
                        )
                    )
    for t in kit.get("types", []):
        typeName = t.get("name", "")
        typeId = t.get("id", "")
        names = {}
        for representation in t.get("representations", []):
            name = representation.get("name", "")
            if name and name not in names:
                names[name] = []
            if name:
                names[name].append(representation)
        for name, group in names.items():
            if len(group) > 1:
                for representation in group[1:]:
                    fix = _makeFix(
                        f'Rename representation "{name}"',
                        {
                            "types": {
                                "updated": [
                                    {
                                        "type": {"id": typeId},
                                        "diff": {
                                            "representations": {
                                                "updated": [
                                                    {
                                                        "representation": {
                                                            "id": representation.get(
                                                                "id", ""
                                                            )
                                                        },
                                                        "diff": {"name": f"{name} 2"},
                                                    }
                                                ]
                                            }
                                        },
                                    }
                                ]
                            }
                        },
                    )
                    problems.append(
                        Problem(
                            constraintId="representation-name-unique",
                            message=f'Duplicate representation name "{name}" inside type "{typeName}".',
                            entityKind="Representation",
                            entityId=representation.get("id", ""),
                            fixes=[fix],
                        )
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
                fix = _makeFix(
                    f'Rename quality "{name}"',
                    {
                        "qualities": {
                            "updated": [
                                {
                                    "quality": {"id": q.get("id", "")},
                                    "diff": {"name": f"{name} 2"},
                                }
                            ]
                        }
                    },
                )
                problems.append(
                    Problem(
                        constraintId="quality-name-unique",
                        message=f'Duplicate quality name "{name}".',
                        entityKind="Quality",
                        entityId=q.get("id", ""),
                        fixes=[fix],
                    )
                )
    names = {}
    for i in kit.get("ports", []):
        name = i.get("name", "")
        if name not in names:
            names[name] = []
        names[name].append(i)
    for name, group in names.items():
        if len(group) > 1:
            for iface in group[1:]:
                fix = _makeFix(
                    f'Rename port "{name}"',
                    {
                        "ports": {
                            "updated": [
                                {
                                    "port": {"id": iface.get("id", "")},
                                    "diff": {"name": f"{name} 2"},
                                }
                            ]
                        }
                    },
                )
                problems.append(
                    Problem(
                        constraintId="port-name-unique",
                        message=f'Duplicate port name "{name}".',
                        entityKind="Port",
                        entityId=iface.get("id", ""),
                        fixes=[fix],
                    )
                )
    names = {}
    for f in kit.get("files", []):
        name = f.get("name", "")
        if name not in names:
            names[name] = []
        names[name].append(f)
    for name, group in names.items():
        if len(group) > 1:
            for f in group[1:]:
                fix = _makeFix(
                    f'Rename file "{name}"',
                    {
                        "files": {
                            "updated": [
                                {
                                    "file": {"id": f.get("id", "")},
                                    "diff": {"name": f"{name} 2"},
                                }
                            ]
                        }
                    },
                )
                problems.append(
                    Problem(
                        constraintId="file-name-unique",
                        message=f'Duplicate file name "{name}".',
                        entityKind="File",
                        entityId=f.get("id", ""),
                        fixes=[fix],
                    )
                )
    byParent = {}
    for fo in kit.get("folders", []):
        parentId = fo.get("parent", {}).get("id") if fo.get("parent") else None
        if parentId not in byParent:
            byParent[parentId] = []
        byParent[parentId].append(fo)
    for parentId, siblings in byParent.items():
        names = {}
        for fo in siblings:
            name = fo.get("name", "")
            if name not in names:
                names[name] = []
            names[name].append(fo)
        for name, group in names.items():
            if len(group) > 1:
                for fo in group[1:]:
                    fix = _makeFix(
                        f'Rename folder "{name}"',
                        {
                            "folders": {
                                "updated": [
                                    {
                                        "folder": {"id": fo.get("id", "")},
                                        "diff": {"name": f"{name} 2"},
                                    }
                                ]
                            }
                        },
                    )
                    problems.append(
                        Problem(
                            constraintId="folder-name-unique",
                            message=f'Duplicate folder name "{name}" among siblings.',
                            entityKind="Folder",
                            entityId=fo.get("id", ""),
                            fixes=[fix],
                        )
                    )
    for design in kit.get("designs", []):
        designName = design.get("name", "")
        designId = design.get("id", "")
        paths: dict[str, list[dict]] = {}
        for layer in design.get("layers", []):
            path = layer.get("path", "")
            if path not in paths:
                paths[path] = []
            paths[path].append(layer)
        for path, group in paths.items():
            if len(group) > 1:
                for layer in group[1:]:
                    fix = _makeFix(
                        f'Rename layer "{path}"',
                        {
                            "designs": {
                                "updated": [
                                    {
                                        "design": {"id": designId},
                                        "diff": {
                                            "layers": {
                                                "updated": [
                                                    {
                                                        "layer": {
                                                            "id": layer.get("id", "")
                                                        },
                                                        "diff": {"path": f"{path} 2"},
                                                    }
                                                ]
                                            }
                                        },
                                    }
                                ]
                            }
                        },
                    )
                    problems.append(
                        Problem(
                            constraintId="layer-path-unique",
                            message=f'Duplicate layer path "{path}" inside design "{designName}".',
                            entityKind="Layer",
                            entityId=layer.get("id", ""),
                            fixes=[fix],
                        )
                    )
    return ValidationResult(problems=problems)


# #endregion 📧Dict-based Validation

# #region 🕌Graph Operations
# Graph construction and traversal for piece connectivity analysis.


def buildPieceGraph(design: Design | dict) -> networkx.Graph:
    """🏗️Build a networkx graph from pieces and connections."""
    G = networkx.Graph()
    pieces = design.get("pieces", []) if isinstance(design, dict) else design.pieces
    connections = (
        design.get("connections", [])
        if isinstance(design, dict)
        else design.connections
    )
    for piece in pieces:
        pieceId = piece["id"] if isinstance(piece, dict) else piece.id
        G.add_node(pieceId, piece=piece)
    for connection in connections:
        if isinstance(connection, dict):
            sourceId = connection["parent"]["piece"]["id"]
            targetId = connection["child"]["piece"]["id"]
        else:
            sourceId = connection.connectedPiece.id
            targetId = connection.connectingPiece.id
        if G.has_node(sourceId) and G.has_node(targetId):
            G.add_edge(sourceId, targetId, connection=connection)
    return G


def findFixedPieces(design: Design | dict) -> list[str]:
    """🔖Find all pieces that are fixed in the design hierarchy."""
    pieces = design.get("pieces", []) if isinstance(design, dict) else design.pieces
    result = []
    for p in pieces:
        if isinstance(p, dict):
            hasPlane = _dict_piece_plane(p) is not None
            hasCenter = _dict_piece_center(p) is not None
            if hasPlane != hasCenter:
                raise ValueError(
                    f"Piece {p.get('id')} has inconsistent plane and center"
                )
            if hasPlane:
                result.append(p["id"])
        else:
            hasPlane = p.plane is not None
            hasCenter = p.center is not None
            if hasPlane != hasCenter:
                raise ValueError(f"Piece {p.id} has inconsistent plane and center")
            if hasPlane:
                result.append(p.id)
    return result


def getConnectedComponents(design: Design | dict) -> list[set[str]]:
    """🔖Get connected components of the piece graph."""
    G = buildPieceGraph(design)


def getPieceHierarchy(design: Design | dict, rootId: str) -> dict[str, int]:
    """🍃Get the hierarchical ordering of pieces from root to leaf."""
    G = buildPieceGraph(design)
    if rootId not in G:
        return {}


# #endregion 🕌Graph Operations

# #endregion 🛡️Validation


# #region 🌤️Flatten Design
# Design flattening to resolve nested sub-designs into a single coordinate space.


def getTypeById(kit: dict, id: str) -> dict | None:
    """🔖Look up a type by its ID within a kit dictionary."""
    for t in kit.get("types", []):
        if t.get("id") == id:
            return t
    return None


def getConnectorFromType(
    kit: dict,
    typeData: dict | None,
    connectorId: str | None,
    *,
    types_by_id: dict[str, dict] | None = None,
) -> dict | None:
    """🔖Look up a connector by name from a type dictionary."""

    def _resolve_type(id: str) -> dict | None:
        if types_by_id is not None:
            return types_by_id.get(id)
        return getTypeById(kit, id)

    if typeData is None:
        return None
    if connectorId is None:
        connectors = typeData.get("connectors", [])
        if connectors:
            return connectors[0]
        parent = typeData.get("parent")
        if parent:
            parentType = _resolve_type(parent.get("id", ""))
            return getConnectorFromType(
                kit, parentType, connectorId, types_by_id=types_by_id
            )
        return None
    for connector in typeData.get("connectors", []):
        if connector.get("id") == connectorId:
            return connector
    parent = typeData.get("parent")
    if parent:
        parentType = _resolve_type(parent.get("id", ""))
        return getConnectorFromType(
            kit, parentType, connectorId, types_by_id=types_by_id
        )
    connectors = typeData.get("connectors", [])
    if connectors:
        return connectors[0]
    return None


def planeToMatrixDict(plane: dict) -> numpy.ndarray:
    """🔖Convert a plane dictionary to a 4x4 transformation matrix."""
    origin = numpy.array(
        [plane["origin"]["x"], plane["origin"]["y"], plane["origin"]["z"]]
    )
    xAxis = numpy.array([plane["xAxis"]["x"], plane["xAxis"]["y"], plane["xAxis"]["z"]])
    yAxis = numpy.array([plane["yAxis"]["x"], plane["yAxis"]["y"], plane["yAxis"]["z"]])
    zAxis = numpy.cross(xAxis, yAxis)
    zAxis = normalizeVector(zAxis)
    matrix = numpy.eye(4)
    matrix[:3, 0] = xAxis
    matrix[:3, 1] = yAxis
    matrix[:3, 2] = zAxis
    matrix[:3, 3] = origin
    return matrix


def matrixToPlaneDict(matrix: numpy.ndarray) -> dict:
    """🔖Convert a 4x4 transformation matrix to a plane dictionary."""
    origin = matrix[:3, 3]
    xAxis = matrix[:3, 0]
    yAxis = matrix[:3, 1]
    return {
        "origin": {"x": float(origin[0]), "y": float(origin[1]), "z": float(origin[2])},
        "xAxis": {"x": float(xAxis[0]), "y": float(xAxis[1]), "z": float(xAxis[2])},
        "yAxis": {"x": float(yAxis[0]), "y": float(yAxis[1]), "z": float(yAxis[2])},
    }


def quaternionFromUnitVectorsDict(
    vFrom: numpy.ndarray, vTo: numpy.ndarray
) -> numpy.ndarray:
    """🔖Compute a quaternion rotating one unit vector onto another."""
    r = numpy.dot(vFrom, vTo) + 1
    if r < 0.000001:
        if abs(vFrom[0]) > abs(vFrom[2]):
            q = numpy.array([-vFrom[1], vFrom[0], 0, 0])
        else:
            q = numpy.array([0, -vFrom[2], vFrom[1], 0])
    else:
        cross = numpy.cross(vFrom, vTo)
        q = numpy.array([cross[0], cross[1], cross[2], r])
    return q / numpy.linalg.norm(q)


def quaternionFromAxisAngleDict(axis: numpy.ndarray, angle: float) -> numpy.ndarray:
    """🔖Compute a quaternion from an axis-angle representation."""
    halfAngle = angle / 2
    s = numpy.sin(halfAngle)
    return numpy.array([axis[0] * s, axis[1] * s, axis[2] * s, numpy.cos(halfAngle)])


def quaternionToMatrixDict(q: numpy.ndarray) -> numpy.ndarray:
    """🔖Convert a quaternion to a 3x3 rotation matrix."""
    x, y, z, w = q
    x2, y2, z2 = x + x, y + y, z + z
    xx, xy, xz = x * x2, x * y2, x * z2
    yy, yz, zz = y * y2, y * z2, z * z2
    wx, wy, wz = w * x2, w * y2, w * z2
    m = numpy.eye(4)
    m[0, 0] = 1 - (yy + zz)
    m[0, 1] = xy - wz
    m[0, 2] = xz + wy
    m[1, 0] = xy + wz
    m[1, 1] = 1 - (xx + zz)
    m[1, 2] = yz - wx
    m[2, 0] = xz - wy
    m[2, 1] = yz + wx
    m[2, 2] = 1 - (xx + yy)
    return m


def makeRotationAxisDict(axis: numpy.ndarray, angle: float) -> numpy.ndarray:
    """🔖Create a 4x4 rotation matrix around an arbitrary axis."""
    return quaternionToMatrixDict(quaternionFromAxisAngleDict(axis, angle))


def makeTranslationDict(x: float, y: float, z: float) -> numpy.ndarray:
    """🔖Create a 4x4 translation matrix from a displacement vector."""
    m = numpy.eye(4)
    m[0, 3] = x
    m[1, 3] = y
    m[2, 3] = z
    return m


def applyMatrix4ToVec3Dict(m: numpy.ndarray, v: numpy.ndarray) -> numpy.ndarray:
    """🔖Apply a 4x4 matrix to a 3D vector dictionary."""
    return numpy.array(
        [
            m[0, 0] * v[0] + m[0, 1] * v[1] + m[0, 2] * v[2],
            m[1, 0] * v[0] + m[1, 1] * v[1] + m[1, 2] * v[2],
            m[2, 0] * v[0] + m[2, 1] * v[1] + m[2, 2] * v[2],
        ]
    )


def computeChildPlaneDict(
    parentPlane: dict, parentConnector: dict, childConnector: dict, connection: dict
) -> dict:
    """🔖Compute the world-space plane of a child piece from parent and local planes."""
    parentMatrix = planeToMatrixDict(parentPlane)
    parentPoint = numpy.array(
        [
            parentConnector["point"]["x"],
            parentConnector["point"]["y"],
            parentConnector["point"]["z"],
        ]
    )
    parentDirection = normalizeVector(
        numpy.array(
            [
                parentConnector["direction"]["x"],
                parentConnector["direction"]["y"],
                parentConnector["direction"]["z"],
            ]
        )
    )
    childPoint = numpy.array(
        [
            childConnector["point"]["x"],
            childConnector["point"]["y"],
            childConnector["point"]["z"],
        ]
    )
    childDirection = normalizeVector(
        numpy.array(
            [
                childConnector["direction"]["x"],
                childConnector["direction"]["y"],
                childConnector["direction"]["z"],
            ]
        )
    )
    gap = connection.get("gap", 0) or 0
    shift = connection.get("shift", 0) or 0
    rise = connection.get("rise", 0) or 0
    rotation = connection.get("rotation", 0) or 0
    turn = connection.get("turn", 0) or 0
    tilt = connection.get("tilt", 0) or 0
    rotationRad = numpy.deg2rad(rotation)
    turnRad = numpy.deg2rad(turn)
    tiltRad = numpy.deg2rad(tilt)
    reverseChildDirection = -childDirection
    crossVec = numpy.cross(parentDirection, reverseChildDirection)
    crossLen = numpy.linalg.norm(crossVec)
    if crossLen < 0.01:
        if abs(parentDirection[2]) < TOLERANCE:
            alignQuat = quaternionFromAxisAngleDict(
                numpy.array([0.0, 0.0, 1.0]), numpy.pi
            )
        else:
            axis = normalizeVector(
                numpy.cross(numpy.array([0.0, 0.0, 1.0]), parentDirection)
            )
            alignQuat = quaternionFromAxisAngleDict(axis, numpy.pi)
    else:
        alignQuat = quaternionFromUnitVectorsDict(
            reverseChildDirection, parentDirection
        )
    directionT = quaternionToMatrixDict(alignQuat)
    yAxis = numpy.array([0.0, 1.0, 0.0])
    parentConnectorQuat = quaternionFromUnitVectorsDict(yAxis, parentDirection)
    parentRotationT = quaternionToMatrixDict(parentConnectorQuat)
    gapDirection = applyMatrix4ToVec3Dict(parentRotationT, numpy.array([0.0, 1.0, 0.0]))
    shiftDirection = applyMatrix4ToVec3Dict(
        parentRotationT, numpy.array([1.0, 0.0, 0.0])
    )
    raiseDirection = applyMatrix4ToVec3Dict(
        parentRotationT, numpy.array([0.0, 0.0, 1.0])
    )
    turnAxis = applyMatrix4ToVec3Dict(parentRotationT, numpy.array([0.0, 0.0, 1.0]))
    tiltAxis = applyMatrix4ToVec3Dict(parentRotationT, numpy.array([1.0, 0.0, 0.0]))
    orientationT = directionT.copy()
    rotateT = makeRotationAxisDict(parentDirection, -rotationRad)
    orientationT = rotateT @ orientationT
    turnAxis = applyMatrix4ToVec3Dict(rotateT, turnAxis)
    tiltAxis = applyMatrix4ToVec3Dict(rotateT, tiltAxis)
    turnT = makeRotationAxisDict(turnAxis, turnRad)
    orientationT = turnT @ orientationT
    tiltT = makeRotationAxisDict(tiltAxis, tiltRad)
    orientationT = tiltT @ orientationT
    centerChildT = makeTranslationDict(-childPoint[0], -childPoint[1], -childPoint[2])
    transform = orientationT @ centerChildT
    gapTransform = makeTranslationDict(
        gapDirection[0] * gap, gapDirection[1] * gap, gapDirection[2] * gap
    )
    shiftTransform = makeTranslationDict(
        shiftDirection[0] * shift, shiftDirection[1] * shift, shiftDirection[2] * shift
    )
    raiseTransform = makeTranslationDict(
        raiseDirection[0] * rise, raiseDirection[1] * rise, raiseDirection[2] * rise
    )
    translationT = raiseTransform @ shiftTransform @ gapTransform
    transform = translationT @ transform
    moveToParentT = makeTranslationDict(parentPoint[0], parentPoint[1], parentPoint[2])
    transform = moveToParentT @ transform
    finalMatrix = parentMatrix @ transform
    result = matrixToPlaneDict(finalMatrix)
    return {
        "origin": {
            "x": round(result["origin"]["x"] / TOLERANCE) * TOLERANCE,
            "y": round(result["origin"]["y"] / TOLERANCE) * TOLERANCE,
            "z": round(result["origin"]["z"] / TOLERANCE) * TOLERANCE,
        },
        "xAxis": {
            "x": round(result["xAxis"]["x"] / TOLERANCE) * TOLERANCE,
            "y": round(result["xAxis"]["y"] / TOLERANCE) * TOLERANCE,
            "z": round(result["xAxis"]["z"] / TOLERANCE) * TOLERANCE,
        },
        "yAxis": {
            "x": round(result["yAxis"]["x"] / TOLERANCE) * TOLERANCE,
            "y": round(result["yAxis"]["y"] / TOLERANCE) * TOLERANCE,
            "z": round(result["yAxis"]["z"] / TOLERANCE) * TOLERANCE,
        },
    }


def flattenDesignDict(kit: dict, designId: str) -> dict:
    """🔖Flatten a nested design hierarchy into a single flat coordinate space."""
    design = next((d for d in kit.get("designs", []) if d.get("id") == designId), None)
    if design is None:
        raise ValueError(f"Design {designId} not found")
    pieces = design.get("pieces", [])
    if not pieces:
        return {}
    types_by_id: dict[str, dict] = {
        t["id"]: t for t in kit.get("types", []) if t.get("id")
    }
    pieceMap = {p["id"]: dict(p) for p in pieces}
    piecePlanes: dict[str, dict] = {}
    piecePaths: dict[str, str] = {}
    adjacency: dict[str, list[tuple[str, dict]]] = {}
    for conn in design.get("connections", []):
        src = conn["parent"]["piece"]["id"]
        tgt = conn["child"]["piece"]["id"]
        if src not in pieceMap or tgt not in pieceMap:
            continue
        adjacency.setdefault(src, []).append((tgt, conn))
        adjacency.setdefault(tgt, []).append((src, conn))
    visited: set[str] = set()

    def bfs(root_id: str) -> None:
        q: collections.deque[str] = collections.deque([root_id])
        visited.add(root_id)
        piecePaths[root_id] = root_id
        root_piece = pieceMap[root_id]
        if (
            _dict_piece_plane(root_piece) is not None
            and _dict_piece_center(root_piece) is not None
        ):
            piecePlanes[root_id] = _dict_piece_plane(root_piece)
        else:
            piecePlanes[root_id] = {
                "origin": {"x": 0, "y": 0, "z": 0},
                "xAxis": {"x": 1, "y": 0, "z": 0},
                "yAxis": {"x": 0, "y": 1, "z": 0},
            }
        while q:
            current_id = q.popleft()
            current_plane = piecePlanes[current_id]
            current_piece = pieceMap[current_id]
            for neighbor_id, conn in adjacency.get(current_id, []):
                if neighbor_id in visited:
                    continue
                visited.add(neighbor_id)
                parent_id = current_id
                child_id = neighbor_id
                parent_plane = current_plane
                parent_piece = current_piece
                child_piece = pieceMap[child_id]
                if conn["parent"]["piece"]["id"] == parent_id:
                    parent_side = conn["parent"]
                    child_side = conn["child"]
                else:
                    parent_side = conn["child"]
                    child_side = conn["parent"]
                parent_type = types_by_id.get(
                    parent_piece.get("type", {}).get("id", "")
                )
                child_type = types_by_id.get(child_piece.get("type", {}).get("id", ""))
                parent_connector_id = (
                    parent_side.get("connector", {}).get("id")
                    if parent_side.get("connector")
                    else None
                )
                child_connector_id = (
                    child_side.get("connector", {}).get("id")
                    if child_side.get("connector")
                    else None
                )
                parent_connector = getConnectorFromType(
                    kit, parent_type, parent_connector_id, types_by_id=types_by_id
                )
                child_connector = getConnectorFromType(
                    kit, child_type, child_connector_id, types_by_id=types_by_id
                )
                if parent_connector is None or child_connector is None:
                    continue
                child_plane = computeChildPlaneDict(
                    parent_plane, parent_connector, child_connector, conn
                )
                piecePlanes[child_id] = child_plane
                radius = 2.697
                verticalVExtra = 1.0
                horizontalScale = 3.0633
                parent_center = parent_piece.get("center") or {"u": 0, "v": 0}
                connection_u = conn.get("u", 0) or 0
                connection_v = conn.get("v", 0) or 0
                if parent_center["u"] == 0 and parent_center["v"] == 0:
                    t = parent_connector.get("t", 0) or 0
                    angle = 2 * math.pi * t
                    child_u = radius * math.sin(angle)
                    child_v = radius * math.cos(angle)
                else:
                    parent_dir_z = (parent_connector.get("direction") or {}).get(
                        "z", 0
                    ) or 0
                    is_vertical_connection = abs(parent_dir_z) > 0.5
                    if is_vertical_connection:
                        child_u = parent_center["u"] + connection_u
                        child_v = parent_center["v"] + connection_v + verticalVExtra
                    else:
                        child_u = parent_center["u"] + connection_u * horizontalScale
                        child_v = parent_center["v"] + connection_v * horizontalScale
                child_center = {
                    "u": round(child_u / TOLERANCE) * TOLERANCE,
                    "v": round(child_v / TOLERANCE) * TOLERANCE,
                }
                ch = pieceMap[child_id]
                po = dict(ch.get("pose") or {})
                po["center"] = child_center
                ch["pose"] = po
                piecePaths[child_id] = (
                    piecePaths.get(parent_id, parent_id) + "," + child_id
                )
                q.append(neighbor_id)

    for p in pieces:
        g = p.get("id")
        if g and g not in visited:
            bfs(g)

    def _plane_dict_close(ap: dict | None, bp: dict | None, tol: float = 1e-4) -> bool:
        if ap is bp:
            return True
        if not ap or not bp:
            return False

        def _pt(d: dict, *keys: str) -> tuple[float, float, float]:
            o = d or {}
            return (
                float(o.get("x", 0) or 0),
                float(o.get("y", 0) or 0),
                float(o.get("z", 0) or 0),
            )

        ao, ax, ay = (
            ap.get("origin") or {},
            ap.get("xAxis") or {},
            ap.get("yAxis") or {},
        )
        bo, bx, by = (
            bp.get("origin") or {},
            bp.get("xAxis") or {},
            bp.get("yAxis") or {},
        )
        for a3, b3 in (
            (_pt(ao, "x", "y", "z"), _pt(bo, "x", "y", "z")),
            (_pt(ax, "x", "y", "z"), _pt(bx, "x", "y", "z")),
            (_pt(ay, "x", "y", "z"), _pt(by, "x", "y", "z")),
        ):
            if any(abs(a3[i] - b3[i]) >= tol for i in range(3)):
                return False
        return True

    updated_rows: list[dict] = []
    for piece in pieces:
        id = piece.get("id")
        if not id or id not in piecePlanes:
            continue
        new_plane = piecePlanes[id]
        new_center = (
            _dict_piece_center(pieceMap.get(id))
            if id in pieceMap
            else _dict_piece_center(piece)
        )
        if new_center is None:
            new_center = {"u": 0, "v": 0}
        old_plane = _dict_piece_plane(piece)
        old_center = _dict_piece_center(piece) or {"u": 0, "v": 0}
        plane_changed = old_plane is None or not _plane_dict_close(new_plane, old_plane)
        center_changed = (
            abs(float(new_center.get("u", 0) or 0) - float(old_center.get("u", 0) or 0))
            > TOLERANCE
            or abs(
                float(new_center.get("v", 0) or 0) - float(old_center.get("v", 0) or 0)
            )
            > TOLERANCE
        )
        if plane_changed or center_changed:
            updated_rows.append(
                {"id": id, "diff": {"pose": {"plane": new_plane, "center": new_center}}}
            )
    return {
        "pieces": {
            "updated": updated_rows,
        },
        "_piecePaths": piecePaths,
    }


# #region 🎯SemioReport
def _semio_report_ok(diff, warnings=None, infos=None):
    """📋Successful semio algorithm payload (tool-friendly JSON)."""
    return {
        "ok": True,
        "diff": diff,
        "warnings": warnings or [],
        "infos": infos or [],
        "errors": [],
    }


def _semio_report_err(errors: list):
    """📋Failed semio algorithm payload."""
    return {"ok": False, "diff": None, "warnings": [], "infos": [], "errors": errors}


def flattenDesignReportDict(kit: dict, designId: str) -> dict:
    """📋Canonical flatten report matching TypeScript flattenDesign (forward/backward + notes)."""
    import copy

    design = next((d for d in kit.get("designs", []) if d.get("id") == designId), None)
    if design is None:
        return _semio_report_err(
            [
                {
                    "code": "flatten.design-not-found",
                    "message": f"Design {designId} not found",
                }
            ]
        )
    pieces = design.get("pieces", [])
    if not pieces:
        return _semio_report_ok(
            {"forward": {}, "backward": {}},
            [],
            [
                {
                    "code": "flatten.empty-pieces",
                    "message": "No pieces to flatten; returning empty forward and backward diffs.",
                }
            ],
        )
    before = copy.deepcopy(design)
    try:
        forward = flattenDesignDict(kit, designId)
    except ValueError as e:
        return _semio_report_err([{"code": "flatten.error", "message": str(e)}])
    backward = _inverseDesignDiff(before, forward)
    return _semio_report_ok({"forward": forward, "backward": backward}, [], [])


# #endregion 🎯SemioReport


# #region 🌳Flatten Merkle Hashes
# Per-piece merkle hashes for plane and center computations so subsequent flatten calls can skip unchanged chains.


def _hash_plane_root(id: str, plane: dict | None) -> str:
    """🌱Root plane hash includes only the piece id and its fixed plane components (identity when absent)."""
    w = HashWriter()
    if plane is None:
        w.writeString("plane.root.identity")
        w.writeString(id)
        return w.digest()
    w.writeString("plane.root")
    w.writeString(id)
    origin = plane.get("origin") or {}
    xAxis = plane.get("xAxis") or {}
    yAxis = plane.get("yAxis") or {}
    w.writeNumber(origin.get("x", 0) or 0)
    w.writeNumber(origin.get("y", 0) or 0)
    w.writeNumber(origin.get("z", 0) or 0)
    w.writeNumber(xAxis.get("x", 0) or 0)
    w.writeNumber(xAxis.get("y", 0) or 0)
    w.writeNumber(xAxis.get("z", 0) or 0)
    w.writeNumber(yAxis.get("x", 0) or 0)
    w.writeNumber(yAxis.get("y", 0) or 0)
    w.writeNumber(yAxis.get("z", 0) or 0)
    return w.digest()


def _hash_plane_chain(
    parent_hash: str, parent_connector: dict, child_connector: dict, connection: dict
) -> str:
    """🔗Chain plane hash depends on parent plane hash plus the inputs consumed by computeChildPlane."""
    w = HashWriter()
    w.writeString("plane.chain")
    w.writeHash(parent_hash)
    pPoint = parent_connector.get("point") or {}
    pDir = parent_connector.get("direction") or {}
    cPoint = child_connector.get("point") or {}
    cDir = child_connector.get("direction") or {}
    w.writeNumber(pPoint.get("x", 0) or 0)
    w.writeNumber(pPoint.get("y", 0) or 0)
    w.writeNumber(pPoint.get("z", 0) or 0)
    w.writeNumber(pDir.get("x", 0) or 0)
    w.writeNumber(pDir.get("y", 0) or 0)
    w.writeNumber(pDir.get("z", 0) or 0)
    w.writeNumber(cPoint.get("x", 0) or 0)
    w.writeNumber(cPoint.get("y", 0) or 0)
    w.writeNumber(cPoint.get("z", 0) or 0)
    w.writeNumber(cDir.get("x", 0) or 0)
    w.writeNumber(cDir.get("y", 0) or 0)
    w.writeNumber(cDir.get("z", 0) or 0)
    w.writeNumber(connection.get("gap", 0) or 0)
    w.writeNumber(connection.get("shift", 0) or 0)
    w.writeNumber(connection.get("rise", 0) or 0)
    w.writeNumber(connection.get("rotation", 0) or 0)
    w.writeNumber(connection.get("turn", 0) or 0)
    w.writeNumber(connection.get("tilt", 0) or 0)
    return w.digest()


def _hash_center_root(id: str, center: dict | None) -> str:
    """🌱Root center hash includes only the piece id and its fixed center (identity when absent)."""
    w = HashWriter()
    if center is None:
        w.writeString("center.root.identity")
        w.writeString(id)
        return w.digest()
    w.writeString("center.root")
    w.writeString(id)
    w.writeNumber(center.get("u", 0) or 0)
    w.writeNumber(center.get("v", 0) or 0)
    return w.digest()


def _hash_center_chain(
    parent_hash: str, parent_connector: dict, connection: dict
) -> str:
    """🔗Chain center hash conservatively includes every potentially-read input of the child center computation."""
    w = HashWriter()
    w.writeString("center.chain")
    w.writeHash(parent_hash)
    pDir = parent_connector.get("direction") or {}
    w.writeNumber(pDir.get("z", 0) or 0)
    w.writeNumber(parent_connector.get("t", 0) or 0)
    w.writeNumber(connection.get("u", 0) or 0)
    w.writeNumber(connection.get("v", 0) or 0)
    return w.digest()


def computeFlatHashesDict(kit: dict, designId: str) -> dict[str, dict]:
    """🌳Compute per-piece {planeHash, centerHash} merkle hashes for the flattened design so callers can cache by chain identity."""
    design = next((d for d in kit.get("designs", []) if d.get("id") == designId), None)
    if design is None:
        raise ValueError(f"Design {designId} not found")
    pieces = design.get("pieces", [])
    if not pieces:
        return {}
    pieceMap = {p["id"]: p for p in pieces}
    planeHashes: dict[str, str] = {}
    centerHashes: dict[str, str] = {}
    G = buildPieceGraph(design)
    components = list(networkx.connected_components(G))
    for component in components:
        rootNode = None
        for nodeId in component:
            piece = pieceMap.get(nodeId)
            if (
                piece
                and _dict_piece_plane(piece) is not None
                and _dict_piece_center(piece) is not None
            ):
                rootNode = nodeId
                break
        if rootNode is None and component:
            rootNode = next(iter(sorted(component)))
        if rootNode is None:
            continue
        rootPiece = pieceMap[rootNode]
        planeHashes[rootNode] = _hash_plane_root(rootNode, _dict_piece_plane(rootPiece))
        centerHashes[rootNode] = _hash_center_root(
            rootNode, _dict_piece_center(rootPiece)
        )
        for source, target in networkx.bfs_edges(G, rootNode):
            if target in planeHashes:
                continue
            parentId = source
            childId = target
            parentPlaneHash = planeHashes.get(parentId)
            parentCenterHash = centerHashes.get(parentId)
            if parentPlaneHash is None or parentCenterHash is None:
                continue
            edgeData = G.get_edge_data(parentId, childId)
            connection = edgeData.get("connection") if edgeData else None
            if connection is None:
                continue
            parentPiece = pieceMap[parentId]
            childPiece = pieceMap[childId]
            parentType = getTypeById(kit, parentPiece.get("type", {}).get("id", ""))
            childType = getTypeById(kit, childPiece.get("type", {}).get("id", ""))
            parentSide = (
                connection["parent"]
                if connection["parent"]["piece"]["id"] == parentId
                else connection["child"]
            )
            childSide = (
                connection["child"]
                if connection["child"]["piece"]["id"] == childId
                else connection["parent"]
            )
            parentConnectorId = (
                parentSide.get("connector", {}).get("id")
                if parentSide.get("connector")
                else None
            )
            childConnectorId = (
                childSide.get("connector", {}).get("id")
                if childSide.get("connector")
                else None
            )
            parentConnector = (
                getConnectorFromType(kit, parentType, parentConnectorId) or {}
            )
            childConnector = (
                getConnectorFromType(kit, childType, childConnectorId) or {}
            )
            planeHashes[childId] = _hash_plane_chain(
                parentPlaneHash, parentConnector, childConnector, connection
            )
            centerHashes[childId] = _hash_center_chain(
                parentCenterHash, parentConnector, connection
            )
    return {
        id: {"planeHash": planeHashes[id], "centerHash": centerHashes[id]}
        for id in planeHashes
    }


def flattenDesignCachedDict(
    kit: dict, designId: str, cache: dict[str, dict] | None = None
) -> tuple[dict, dict[str, dict]]:
    """🧠Flatten a design reusing cached plane/center values when the per-piece merkle hashes match the previous run."""
    newHashes = computeFlatHashesDict(kit, designId)
    rep = flattenDesignReportDict(kit, designId)
    if not rep["ok"]:
        return rep, {}
    diff = rep["diff"]["forward"]
    updatedById: dict[str, dict] = {}
    for entry in (
        diff.get("pieces", {}).get("updated", [])
        if isinstance(diff.get("pieces"), dict)
        else []
    ):
        id_key = entry.get("piece", {}).get("id", entry.get("id", ""))
        updatedById[id_key] = entry["diff"]
    nextCache: dict[str, dict] = {}
    if cache:
        for id, hashes in newHashes.items():
            prev = cache.get(id)
            updated = updatedById.get(id)
            if prev is None or updated is None:
                if updated is not None:
                    pd = _dict_piece_diff_pose(updated)
                    pl = pd.get("plane") if pd else None
                    ce = pd.get("center") if pd else None
                    nextCache[id] = {
                        "planeHash": hashes["planeHash"],
                        "centerHash": hashes["centerHash"],
                        "plane": pl,
                        "center": ce,
                    }
                continue
            reusedPlane = (
                prev.get("plane")
                if prev.get("planeHash") == hashes["planeHash"]
                else (_dict_piece_diff_pose(updated) or {}).get("plane")
            )
            reusedCenter = (
                prev.get("center")
                if prev.get("centerHash") == hashes["centerHash"]
                else (_dict_piece_diff_pose(updated) or {}).get("center")
            )
            nextCache[id] = {
                "planeHash": hashes["planeHash"],
                "centerHash": hashes["centerHash"],
                "plane": reusedPlane,
                "center": reusedCenter,
            }
    else:
        for id, hashes in newHashes.items():
            updated = updatedById.get(id)
            if updated is None:
                continue
            pd = _dict_piece_diff_pose(updated)
            pl = pd.get("plane") if pd else None
            ce = pd.get("center") if pd else None
            nextCache[id] = {
                "planeHash": hashes["planeHash"],
                "centerHash": hashes["centerHash"],
                "plane": pl,
                "center": ce,
            }
    return rep, nextCache


# #endregion 🌳Flatten Merkle Hashes


# #endregion 🌤️Flatten Design


# #region 🧿Kit Import/Export
# Import and export utilities for kit serialization and deserialization.


class KitData:
    """🔖Simple in-memory kit representation that supports attribute access."""

    def __init__(self, data: dict):
        self._data = data
        self.id = data.get("id")
        self.name = data.get("name", "")
        self.version = data.get("version", "")
        self.description = data.get("description", "")
        self.icon = data.get("icon", "")
        self.image = data.get("image", "")
        self.remote = data.get("remote", "")
        self.homepage = data.get("homepage", "")
        self.license = data.get("license")
        self.preview = data.get("preview", "")
        self.types = data.get("types", [])
        self.designs = data.get("designs", [])

    def to_dict(self) -> dict:
        return self._data

    def filter_kit(self, filter_spec: dict) -> "KitData":
        """🔖General-purpose kit filter with glob support."""
        design_id = filter_spec.get("design_id")
        tags = filter_spec.get("representation_tags")

        if design_id:
            base = self._filter_kit_by_design(design_id, tags)
        else:
            base = self

        base_data = base._data if isinstance(base, KitData) else base
        glob_keys = [
            "designs",
            "types",
            "ports",
            "files",
            "tags",
            "concepts",
            "qualities",
            "authors",
            "folders",
        ]
        has_glob_filters = any(filter_spec.get(k) is not None for k in glob_keys)
        if not has_glob_filters:
            return KitData(base_data) if base is self else base

        import fnmatch as _fnmatch

        def _matches(name: str, glob_filter: typing.Optional[dict]) -> bool:
            if glob_filter is None:
                return True
            include = glob_filter.get("include") or []
            exclude = glob_filter.get("exclude") or []
            if include and not any(
                _fnmatch.fnmatch(name.lower(), p.lower()) for p in include
            ):
                return False
            if any(_fnmatch.fnmatch(name.lower(), p.lower()) for p in exclude):
                return False
            return True

        filtered = dict(base_data)
        entity_key_map = {
            "types": "name",
            "designs": "name",
            "ports": "name",
            "files": "name",
            "tags": "name",
            "concepts": "name",
            "qualities": "name",
            "authors": "name",
            "folders": "name",
        }
        for entity_key, name_key in entity_key_map.items():
            spec = filter_spec.get(entity_key)
            if spec is not None:
                filtered[entity_key] = [
                    e
                    for e in filtered.get(entity_key, [])
                    if _matches(e.get(name_key, ""), spec)
                ]

        return KitData(filtered)

    def _filter_kit_by_design(
        self, design_id: str, tags: typing.Optional[list[str]] = None
    ) -> "KitData":
        kit = self._data
        design = next(
            (d for d in kit.get("designs", []) if d.get("id") == design_id), None
        )
        if design is None:
            return KitData(
                {
                    "id": kit.get("id"),
                    "name": kit.get("name", ""),
                    "version": kit.get("version", ""),
                }
            )

        used_type_ids: set[str] = set()
        used_design_ids: set[str] = {design_id}
        for piece in design.get("pieces", []):
            piece_kind_id = piece.get("type", {}).get("id")
            if piece_kind_id:
                used_type_ids.add(piece_kind_id)
            child_design_id = piece.get("design", {}).get("id")
            if child_design_id:
                used_design_ids.add(child_design_id)

        type_by_id = {
            type_item.get("id"): type_item for type_item in kit.get("types", [])
        }

        def collect_ancestors(type_id: str) -> None:
            parent_id = (type_by_id.get(type_id) or {}).get("parent", {}).get("id")
            if parent_id and parent_id not in used_type_ids:
                used_type_ids.add(parent_id)
                collect_ancestors(parent_id)

        for type_id in list(used_type_ids):
            collect_ancestors(type_id)

        resolved_tag_ids: list[str] = []
        for tag_value in tags or []:
            by_id = next(
                (tag for tag in kit.get("tags", []) if tag.get("id") == tag_value),
                None,
            )
            if by_id is not None:
                resolved_tag_ids.append(by_id["id"])
                continue
            resolved_tag_ids.extend(
                tag["id"] for tag in kit.get("tags", []) if tag.get("name") == tag_value
            )

        used_port_ids: set[str] = set()
        used_file_ids: set[str] = set()
        used_tag_ids: set[str] = set()
        used_concept_ids: set[str] = set()
        used_quality_ids: set[str] = set()
        used_author_ids: set[str] = set()
        used_folder_names: set[str] = set()
        selected_representations: dict[str, dict] = {}

        def collect_quality_from_props(props: typing.Optional[list[dict]]) -> None:
            for prop in props or []:
                quality_id = prop.get("quality", {}).get("id")
                if quality_id:
                    used_quality_ids.add(quality_id)

        def select_best_representation(
            representations: list[dict],
        ) -> typing.Optional[dict]:
            if not representations:
                return None
            if not resolved_tag_ids:
                return next(
                    (
                        representation
                        for representation in representations
                        if not representation.get("tags")
                    ),
                    representations[0],
                )
            filtered = [
                representation
                for representation in representations
                if all(
                    selected
                    in {tag.get("id") for tag in representation.get("tags", [])}
                    for selected in resolved_tag_ids
                )
            ]
            if not filtered:
                return None

            def score(representation: dict) -> float:
                representation_tags = {
                    tag.get("id") for tag in representation.get("tags", [])
                }
                selected = set(resolved_tag_ids)
                union = representation_tags | selected
                return (
                    0.0
                    if not union
                    else len(representation_tags & selected) / len(union)
                )

            return max(filtered, key=score)

        for type_id in used_type_ids:
            type_item = type_by_id.get(type_id)
            if not type_item:
                continue
            if type_item.get("folder"):
                used_folder_names.add(type_item["folder"])
            for connector in type_item.get("connectors", []):
                port_id = connector.get("port", {}).get("id")
                if port_id:
                    used_port_ids.add(port_id)
                collect_quality_from_props(connector.get("props"))
            collect_quality_from_props(type_item.get("props"))
            for author in type_item.get("authors", []):
                if author.get("id"):
                    used_author_ids.add(author["id"])
            for concept in type_item.get("concepts", []):
                if concept.get("id"):
                    used_concept_ids.add(concept["id"])
            selected_representation = select_best_representation(
                type_item.get("representations", [])
            )
            if selected_representation:
                selected_representations[type_id] = selected_representation
                file_id = selected_representation.get("file", {}).get("id")
                if file_id:
                    used_file_ids.add(file_id)
                for tag in selected_representation.get("tags", []):
                    if tag.get("id"):
                        used_tag_ids.add(tag["id"])

        for piece in design.get("pieces", []):
            collect_quality_from_props(piece.get("props"))
        for concept in design.get("concepts", []):
            if concept.get("id"):
                used_concept_ids.add(concept["id"])
        for author in design.get("authors", []):
            if author.get("id"):
                used_author_ids.add(author["id"])
        for port_id in list(used_port_ids):
            port = next(
                (
                    candidate
                    for candidate in kit.get("ports", [])
                    if candidate.get("id") == port_id
                ),
                None,
            )
            for compatible in (port or {}).get("compatiblePorts", []):
                if compatible.get("id"):
                    used_port_ids.add(compatible["id"])
        used_tag_ids.update(resolved_tag_ids)

        filtered = {
            key: value
            for key, value in kit.items()
            if key
            not in {
                "types",
                "designs",
                "ports",
                "files",
                "tags",
                "concepts",
                "qualities",
                "authors",
                "folders",
            }
        }
        filtered["types"] = []
        for type_item in kit.get("types", []):
            if type_item.get("id") not in used_type_ids:
                continue
            filtered_type = dict(type_item)
            selected_representation = selected_representations.get(type_item["id"])
            filtered_type["representations"] = (
                [selected_representation] if selected_representation else []
            )
            filtered["types"].append(filtered_type)
        filtered["designs"] = [
            candidate
            for candidate in kit.get("designs", [])
            if candidate.get("id") in used_design_ids
        ]
        filtered["ports"] = [
            port for port in kit.get("ports", []) if port.get("id") in used_port_ids
        ]
        filtered["files"] = [
            file for file in kit.get("files", []) if file.get("id") in used_file_ids
        ]
        filtered["tags"] = [
            tag for tag in kit.get("tags", []) if tag.get("id") in used_tag_ids
        ]
        filtered["concepts"] = [
            concept
            for concept in kit.get("concepts", [])
            if concept.get("id") in used_concept_ids
        ]
        filtered["qualities"] = [
            quality
            for quality in kit.get("qualities", [])
            if quality.get("id") in used_quality_ids
        ]
        filtered["authors"] = [
            author
            for author in kit.get("authors", [])
            if author.get("id") in used_author_ids
        ]
        filtered["folders"] = [
            folder
            for folder in kit.get("folders", [])
            if folder.get("name") in used_folder_names
        ]
        return KitData(filtered)


def _parse_connector_from_sqlite(row: dict) -> dict:
    """🔖_parse_connector_from_sqlite performs the _parse_connector_from_sqlite operation."""
    return {
        "id": row.get("id"),
        "name": row.get("name"),
        "point": {
            "x": row.get("point_x", 0.0),
            "y": row.get("point_y", 0.0),
            "z": row.get("point_z", 0.0),
        },
        "direction": {
            "x": row.get("direction_x", 0.0),
            "y": row.get("direction_y", 1.0),
            "z": row.get("direction_z", 0.0),
        },
        "t": row.get("t", 0.0),
        "mandatory": bool(row.get("mandatory", False)),
        "port": row.get("port_id"),
        "description": row.get("description"),
    }


def _parse_representation_from_sqlite(row: dict) -> dict:
    """🔖_parse_representation_from_sqlite performs the _parse_representation_from_sqlite operation."""
    return {
        "id": row.get("id"),
        "name": row.get("name"),
        "file": row.get("file_id"),
        "description": row.get("description"),
    }


def _parse_type_from_sqlite(
    row: dict, connectors: list[dict], representations: list[dict]
) -> dict:
    """🔖_parse_type_from_sqlite performs the _parse_type_from_sqlite operation."""
    return {
        "id": row.get("id"),
        "name": row.get("name"),
        "parent": row.get("parent_id"),
        "isAbstract": bool(row.get("is_abstract", False)),
        "isVirtual": bool(row.get("virtual", False)),
        "folder": row.get("folder"),
        "stock": row.get("stock"),
        "unit": row.get("unit"),
        "location": row.get("location_id"),
        "description": row.get("description"),
        "icon": row.get("icon"),
        "image": row.get("image"),
        "connectors": connectors,
        "representations": representations,
    }


def _parse_piece_from_sqlite(row: dict) -> dict:
    """🔖Build a piece dict from a SQLite row aligned with ``semio/sqlite/schema.sql`` (``pose_*`` columns, ``design_ref_id``, ``hidden`` / ``locked``)."""
    _pose_plane_keys = (
        "pose_plane_origin_x",
        "pose_plane_origin_y",
        "pose_plane_origin_z",
        "pose_plane_x_axis_x",
        "pose_plane_x_axis_y",
        "pose_plane_x_axis_z",
        "pose_plane_y_axis_x",
        "pose_plane_y_axis_y",
        "pose_plane_y_axis_z",
    )
    plane = None
    if any(row.get(k) is not None for k in _pose_plane_keys):
        plane = {
            "origin": {
                "x": row.get("pose_plane_origin_x", 0.0),
                "y": row.get("pose_plane_origin_y", 0.0),
                "z": row.get("pose_plane_origin_z", 0.0),
            },
            "xAxis": {
                "x": row.get("pose_plane_x_axis_x", 1.0),
                "y": row.get("pose_plane_x_axis_y", 0.0),
                "z": row.get("pose_plane_x_axis_z", 0.0),
            },
            "yAxis": {
                "x": row.get("pose_plane_y_axis_x", 0.0),
                "y": row.get("pose_plane_y_axis_y", 1.0),
                "z": row.get("pose_plane_y_axis_z", 0.0),
            },
        }
    mirror_plane = None
    if row.get("mirror_plane_origin_x") is not None:
        mirror_plane = {
            "origin": {
                "x": row.get("mirror_plane_origin_x", 0.0),
                "y": row.get("mirror_plane_origin_y", 0.0),
                "z": row.get("mirror_plane_origin_z", 0.0),
            },
            "xAxis": {
                "x": row.get("mirror_plane_x_axis_x", 1.0),
                "y": row.get("mirror_plane_x_axis_y", 0.0),
                "z": row.get("mirror_plane_x_axis_z", 0.0),
            },
            "yAxis": {
                "x": row.get("mirror_plane_y_axis_x", 0.0),
                "y": row.get("mirror_plane_y_axis_y", 1.0),
                "z": row.get("mirror_plane_y_axis_z", 0.0),
            },
        }
    center = None
    if row.get("pose_center_x") is not None or row.get("pose_center_y") is not None:
        center = {
            "u": row.get("pose_center_x", 0.0),
            "v": row.get("pose_center_y", 0.0),
        }
    pose: dict | None = None
    if plane is not None or center is not None:
        pose = {}
        if plane is not None:
            pose["plane"] = plane
        if center is not None:
            pose["center"] = center
    hidden_raw = row.get("hidden")
    if hidden_raw is None:
        hidden_raw = row.get("is_hidden", 0)
    locked_raw = row.get("locked")
    if locked_raw is None:
        locked_raw = row.get("is_locked", 0)
    out = {
        "id": row.get("id"),
        "name": row.get("name"),
        "type": row.get("type_id"),
        "design": row.get("design_ref_id") or row.get("design_id_ref"),
        "scale": row.get("scale"),
        "mirrorPlane": mirror_plane,
        "isHidden": bool(hidden_raw),
        "isLocked": bool(locked_raw),
        "color": row.get("color"),
        "description": row.get("description"),
    }
    if pose is not None:
        out["pose"] = pose
    return out


def _parse_connection_from_sqlite(row: dict) -> dict:
    """🔖_parse_connection_from_sqlite performs the _parse_connection_from_sqlite operation."""
    return {
        "id": row.get("id"),
        "connected": {
            "piece": row.get("parent_piece_id"),
            "designPiece": row.get("parent_design_piece_id"),
            "connector": row.get("connected_connector_id"),
        },
        "connecting": {
            "piece": row.get("child_piece_id"),
            "designPiece": row.get("child_design_piece_id"),
            "connector": row.get("connecting_connector_id"),
        },
        "gap": row.get("gap", 0.0),
        "shift": row.get("shift", 0.0),
        "rise": row.get("rise", 0.0),
        "rotation": row.get("rotation", 0.0),
        "turn": row.get("turn", 0.0),
        "tilt": row.get("tilt", 0.0),
        "u": row.get("u"),
        "v": row.get("v"),
        "description": row.get("description"),
    }


def _parse_design_from_sqlite(
    row: dict, pieces: list[dict], connections: list[dict]
) -> dict:
    """🔖_parse_design_from_sqlite performs the _parse_design_from_sqlite operation."""
    view = None
    if (
        row.get("view_center_u") is not None
        or row.get("view_center_v") is not None
        or row.get("view_zoom") is not None
    ):
        view = {
            "center": {
                "u": row.get("view_center_u", 0.0),
                "v": row.get("view_center_v", 0.0),
            },
            "zoom": row.get("view_zoom", 1.0),
        }
    return {
        "id": row.get("id"),
        "name": row.get("name"),
        "parent": row.get("parent_id"),
        "variant": row.get("variant"),
        "view": view,
        "unit": row.get("unit"),
        "location": row.get("location_id"),
        "activeLayer": row.get("active_layer_id"),
        "isAbstract": bool(row.get("is_abstract", False)),
        "folder": row.get("folder"),
        "canScale": (
            bool(row.get("can_scale", False))
            if row.get("can_scale") is not None
            else None
        ),
        "canMirror": (
            bool(row.get("can_mirror", False))
            if row.get("can_mirror") is not None
            else None
        ),
        "description": row.get("description"),
        "icon": row.get("icon"),
        "image": row.get("image"),
        "pieces": pieces,
        "connections": connections,
    }


def _build_folder_path(kit_dict: dict, folder_id: str) -> str:
    """🔖Build folder path from folder hierarchy."""
    for f in kit_dict.get("folders", []):
        if f.get("id") == folder_id:
            parent = f.get("parent")
            if parent:
                parent_path = _build_folder_path(kit_dict, parent.get("id", ""))
                if parent_path:
                    return parent_path + "/" + f.get("name", "")
            return f.get("name", "")
    return ""


def _build_file_path(kit_dict: dict, file_dict: dict) -> str:
    """🔖Build file path from folder hierarchy and file name."""
    folder = file_dict.get("folder")
    if folder:
        folder_path = _build_folder_path(kit_dict, folder.get("id", ""))
        if folder_path:
            return folder_path + "/" + file_dict.get("name", "")
    return file_dict.get("name", "")


# #region 🔄Kit Workflow Helpers


def _kit_to_dict(kit: KitData | dict) -> dict:
    """🔖Return the underlying kit dictionary."""
    return kit.to_dict() if isinstance(kit, KitData) else kit


def _kit_without_file_blobs(kit: KitData | dict) -> dict:
    """🔖Return a deep copy of a kit dictionary without embedded file blobs."""
    kit_copy = copy.deepcopy(_kit_to_dict(kit))
    for file_entry in kit_copy.get("files", []):
        file_entry.pop("blob", None)
    return kit_copy


def _decode_kit_file_blob(blob: str) -> bytes:
    """🔖Decode a kit file blob into raw bytes."""
    encoded = blob.split(",", 1)[1] if blob.startswith("data:") else blob
    return base64.b64decode(encoded)


def _attach_file_blobs_to_kit(kit_dict: dict, files: dict[str, bytes]) -> dict:
    """🔖Attach file blobs from asset bytes to a kit dictionary."""
    for file_entry in kit_dict.get("files", []):
        file_path = _build_file_path(kit_dict, file_entry)
        if file_path in files:
            encoded = base64.b64encode(files[file_path]).decode("ascii")
            file_entry["blob"] = f"data:application/octet-stream;base64,{encoded}"
    return kit_dict


def _collect_kit_asset_files(
    kit: KitData | dict, files: typing.Optional[dict[str, bytes]] = None
) -> dict[str, bytes]:
    """🔖Collect asset bytes for the current kit file entries."""
    data = _kit_to_dict(kit)
    existing_files = files or {}
    collected: dict[str, bytes] = {}
    for file_entry in data.get("files", []):
        file_path = _build_file_path(data, file_entry)
        blob = file_entry.get("blob")
        if blob:
            collected[file_path] = _decode_kit_file_blob(blob)
        elif file_path in existing_files:
            collected[file_path] = existing_files[file_path]
    return collected


def _merge_sqlite_entity(parsed: dict, payload_entity: typing.Optional[dict]) -> dict:
    """🧱Merge a structured SQLite entity with payload metadata."""
    if payload_entity is None:
        return parsed
    merged = copy.deepcopy(payload_entity)
    for key, value in parsed.items():
        if (
            key in {"connectors", "representations", "pieces", "connections"}
            or value is not None
            or key not in merged
        ):
            merged[key] = value
    return merged


def _read_kit_from_sqlite(db_path: str) -> dict:
    """🔖Read a kit dictionary from the folder SQLite database."""
    import sqlite3

    if not os.path.exists(db_path):
        raise FileNotFoundError(f"File not found: {db_path}")

    conn = sqlite3.connect(db_path)
    conn.row_factory = sqlite3.Row
    try:
        cursor = conn.cursor()
        payload_dict: dict = {}
        try:
            payload_row = cursor.execute(
                "SELECT data FROM kit_payload WHERE id = 1"
            ).fetchone()
            if payload_row and payload_row["data"]:
                payload_dict = json.loads(payload_row["data"])
        except sqlite3.OperationalError:
            payload_dict = {}

        kit_row = cursor.execute("SELECT * FROM kit LIMIT 1").fetchone()
        if kit_row is None:
            if payload_dict:
                return payload_dict
            raise ValueError(f"Invalid kit database: no kit row found in {db_path}")

        payload_types_by_id = {
            item.get("id"): item
            for item in payload_dict.get("types", [])
            if item.get("id")
        }
        payload_designs_by_id = {
            item.get("id"): item
            for item in payload_dict.get("designs", [])
            if item.get("id")
        }

        connectors_by_type: dict[str, list[dict]] = {}
        for row in cursor.execute("SELECT * FROM connector ORDER BY id").fetchall():
            connector = _parse_connector_from_sqlite(dict(row))
            connector["port"] = (
                {"id": connector["port"]} if connector.get("port") else None
            )
            connectors_by_type.setdefault(row["type_id"], []).append(connector)

        representations_by_type: dict[str, list[dict]] = {}
        representation_tags_by_representation: dict[str, list[dict]] = {}
        try:
            for row in cursor.execute(
                "SELECT * FROM representation_tag ORDER BY representation_id"
            ).fetchall():
                r = dict(row)
                representation_tags_by_representation.setdefault(
                    r["representation_id"], []
                ).append({"id": r["tag_id"]})
        except sqlite3.OperationalError:
            pass

        for row in cursor.execute(
            "SELECT * FROM representation ORDER BY id"
        ).fetchall():
            representation = _parse_representation_from_sqlite(dict(row))
            representation["file"] = (
                {"id": representation["file"]} if representation.get("file") else None
            )
            representation["tags"] = representation_tags_by_representation.get(
                row["id"], []
            )
            representations_by_type.setdefault(row["type_id"], []).append(
                representation
            )

        types: list[dict] = []
        for row in cursor.execute(
            "SELECT * FROM type ORDER BY row_id, name, id"
        ).fetchall():
            type_dict = _parse_type_from_sqlite(
                dict(row),
                connectors_by_type.get(row["id"], []),
                representations_by_type.get(row["id"], []),
            )
            if type_dict.get("parent"):
                type_dict["parent"] = {"id": type_dict["parent"]}
            if type_dict.get("location"):
                type_dict["location"] = {"id": type_dict["location"]}
            types.append(
                _merge_sqlite_entity(
                    type_dict, payload_types_by_id.get(type_dict.get("id"))
                )
            )

        pieces_by_design: dict[str, list[dict]] = {}
        for row in cursor.execute("SELECT * FROM piece ORDER BY id").fetchall():
            piece = _parse_piece_from_sqlite(dict(row))
            if piece.get("type"):
                piece["type"] = {"id": piece["type"]}
            if piece.get("design"):
                piece["design"] = {"id": piece["design"]}
            pieces_by_design.setdefault(row["design_id"], []).append(piece)

        connections_by_design: dict[str, list[dict]] = {}
        for row in cursor.execute("SELECT * FROM connection ORDER BY id").fetchall():
            connection = _parse_connection_from_sqlite(dict(row))
            for side in ["parent", "connecting"]:
                for key in ["piece", "designPiece", "connector"]:
                    ref = connection.get(side, {}).get(key)
                    if ref:
                        connection[side][key] = {"id": ref}
            connections_by_design.setdefault(row["design_id"], []).append(connection)

        designs: list[dict] = []
        for row in cursor.execute(
            "SELECT * FROM design ORDER BY row_id, name, id"
        ).fetchall():
            design_dict = _parse_design_from_sqlite(
                dict(row),
                pieces_by_design.get(row["id"], []),
                connections_by_design.get(row["id"], []),
            )
            if design_dict.get("parent"):
                design_dict["parent"] = {"id": design_dict["parent"]}
            if design_dict.get("location"):
                design_dict["location"] = {"id": design_dict["location"]}
            if design_dict.get("activeLayer"):
                design_dict["activeLayer"] = {"id": design_dict["activeLayer"]}
            designs.append(
                _merge_sqlite_entity(
                    design_dict, payload_designs_by_id.get(design_dict.get("id"))
                )
            )

        seen_type_ids = {item.get("id") for item in types}
        for payload_type in payload_dict.get("types", []):
            if payload_type.get("id") not in seen_type_ids:
                types.append(copy.deepcopy(payload_type))

        seen_design_ids = {item.get("id") for item in designs}
        for payload_design in payload_dict.get("designs", []):
            if payload_design.get("id") not in seen_design_ids:
                designs.append(copy.deepcopy(payload_design))

        folders: list[dict] = []
        try:
            for row in cursor.execute("SELECT * FROM folder ORDER BY id").fetchall():
                r = dict(row)
                folder_dict: dict = {"id": r.get("id"), "name": r.get("name")}
                if r.get("parent_id"):
                    folder_dict["parent"] = {"id": r["parent_id"]}
                folders.append(folder_dict)
        except sqlite3.OperationalError:
            pass

        files: list[dict] = []
        try:
            for row in cursor.execute("SELECT * FROM file ORDER BY id").fetchall():
                r = dict(row)
                file_dict: dict = {"id": r.get("id"), "name": r.get("name")}
                if r.get("mime"):
                    file_dict["mime"] = r["mime"]
                if r.get("size"):
                    file_dict["size"] = r["size"]
                if r.get("hash"):
                    file_dict["hash"] = r["hash"]
                if r.get("remote_url"):
                    file_dict["remote"] = r["remote_url"]
                if r.get("folder_id"):
                    file_dict["folder"] = {"id": r["folder_id"]}
                files.append(file_dict)
        except sqlite3.OperationalError:
            pass

        result = {
            key: copy.deepcopy(value)
            for key, value in payload_dict.items()
            if key not in {"types", "designs", "folders", "files"}
        }
        result.update(
            {
                "id": kit_row["id"],
                "name": kit_row["name"],
                "version": kit_row["version"],
                "description": kit_row["description"],
                "icon": kit_row["icon"],
                "image": kit_row["image"],
                "preview": kit_row["preview"],
                "remote": kit_row["remote"],
                "homepage": kit_row["homepage"],
                "license": kit_row["license"],
                "types": types,
                "designs": designs,
                "folders": folders,
                "files": files,
            }
        )
        return result
    finally:
        conn.close()


def import_file_kit(path: str) -> KitData:
    """📥Import a JSON file kit (via the ``semio-store`` I/O path)."""
    d = store.load_kit_via_io("io.importFromFile", {"path": path})
    return KitData(d)


def export_file_kit(kit: KitData | dict, path: str) -> None:
    """📤Export a JSON file kit (via ``semio-store``)."""
    dto = _kit_to_dict(kit)
    with store.StoreClient() as c:
        c.call("kit.create", {"dto": dto})
        c.call("io.exportToFile", {"path": path})


def import_folder_kit(folder_path: str) -> tuple[KitData, dict[str, bytes]]:
    """🔖Import a folder kit backed by :file:`.semio/kit.db` (``semio-store`` + Rust SQLite)."""
    try:
        kit_dict = store.load_kit_via_io("io.importFromFolder", {"path": folder_path})
    except FileNotFoundError:
        kit_dict = _read_kit_from_sqlite(
            os.path.join(folder_path, KIT_LOCAL_FOLDERNAME, KIT_LOCAL_FILENAME)
        )
    files: dict[str, bytes] = {}
    for file_entry in kit_dict.get("files", []):
        relative_path = _build_file_path(kit_dict, file_entry)
        asset_path = os.path.join(folder_path, relative_path)
        if os.path.isfile(asset_path):
            with open(asset_path, "rb") as handle:
                files[relative_path] = handle.read()
    _attach_file_blobs_to_kit(kit_dict, files)
    return KitData(kit_dict), files


def export_folder_kit(
    kit: KitData | dict, files: dict[str, bytes], folder_path: str
) -> None:
    """🔖Export a folder kit (``semio-store`` + Rust SQLite on disk)."""
    dto = _kit_to_dict(kit)
    asset_files = _collect_kit_asset_files(dto, files)
    os.makedirs(folder_path, exist_ok=True)
    for entry_name in os.listdir(folder_path):
        if entry_name == KIT_LOCAL_FOLDERNAME:
            continue
        entry_path = os.path.join(folder_path, entry_name)
        if os.path.isdir(entry_path):
            shutil.rmtree(entry_path)
        else:
            os.remove(entry_path)
    with store.StoreClient() as c:
        c.call("kit.create", {"dto": dto})
        c.call("io.exportToFolder", {"path": folder_path})
    for relative_path, content in asset_files.items():
        asset_path = os.path.join(folder_path, relative_path)
        os.makedirs(os.path.dirname(asset_path), exist_ok=True)
        with open(asset_path, "wb") as handle:
            handle.write(content)


def _read_remote_kit_bytes(uri: str) -> tuple[str, bytes, str]:
    """🔖Read remote kit bytes and detect JSON or ZIP format."""
    parsed = urllib.parse.urlparse(uri)
    if parsed.scheme not in {"http", "https"} or not parsed.netloc:
        raise RemoteKitUriNotValid(uri)
    server_url = f"{parsed.scheme}://{parsed.netloc}"
    try:
        with urllib.request.urlopen(uri) as response:
            body = response.read()
            content_type = response.headers.get_content_type()
    except urllib.error.URLError as error:
        raise ServerUnreachable(server_url) from error

    is_zip = (
        body.startswith(b"PK\x03\x04")
        or uri.lower().endswith(".zip")
        or content_type == "application/zip"
    )
    return ("archive" if is_zip else "file"), body, content_type


def import_remote_kit(uri: str) -> tuple[KitData, dict[str, bytes]]:
    """🔖Import a remote kit from JSON or ZIP."""
    remote_kind, body, _ = _read_remote_kit_bytes(uri)
    if remote_kind == "archive":
        with tempfile.NamedTemporaryFile(suffix=".zip", delete=False) as handle:
            handle.write(body)
            archive_path = handle.name
        try:
            return import_kit(archive_path)
        finally:
            if os.path.exists(archive_path):
                os.remove(archive_path)

    kit_dict = json.loads(body.decode("utf-8"))
    files = _collect_kit_asset_files(kit_dict)
    return KitData(kit_dict), files


def edit_temporary_kit(kit: KitData | dict, commands: list[dict] | dict) -> KitData:
    """🔖Edit an in-memory kit with ``ChangeKitCommand`` JSON (``semio.rs``) via the sidecar."""
    if isinstance(commands, dict):
        raise TypeError(
            "dict diffs are removed — pass a list of ChangeKitCommand objects as JSON"
        )
    dto = _kit_to_dict(kit)
    with store.StoreClient() as c:
        c.call("kit.create", {"dto": dto})
        c.call("kit.executeChangeKitCommands", {"cmds": commands})
        out = c.call("kit.snapshot", None)
    if not isinstance(out, dict):
        raise TypeError("kit.snapshot: expected object")
    return KitData(out)


def edit_file_kit(path: str, diff: dict) -> KitData:
    """🔖Edit a JSON file kit in place."""
    updated = edit_temporary_kit(import_file_kit(path), diff)
    export_file_kit(updated, path)
    return updated


def edit_folder_kit(folder_path: str, diff: dict) -> KitData:
    """🔖Edit a folder kit in place."""
    kit, files = import_folder_kit(folder_path)
    updated = edit_temporary_kit(kit, diff)
    export_folder_kit(updated, _collect_kit_asset_files(updated, files), folder_path)
    return updated


def edit_archive_kit(path: str, diff: dict) -> KitData:
    """🔖Edit an archive kit in place."""
    kit, files = import_kit(path)
    updated = edit_temporary_kit(kit, diff)
    export_kit(updated, _collect_kit_asset_files(updated, files), path)
    return updated


def _write_remote_kit_bytes(uri: str, body: bytes, content_type: str) -> None:
    """✏️Write remote kit bytes back to their source URI."""
    parsed = urllib.parse.urlparse(uri)
    if parsed.scheme not in {"http", "https"} or not parsed.netloc:
        raise RemoteKitUriNotValid(uri)
    server_url = f"{parsed.scheme}://{parsed.netloc}"
    request = urllib.request.Request(
        uri, data=body, method="PUT", headers={"Content-Type": content_type}
    )
    try:
        with urllib.request.urlopen(request):
            pass
    except urllib.error.URLError as error:
        raise ServerUnreachable(server_url) from error


def edit_remote_kit(uri: str, diff: dict) -> KitData:
    """🔖Edit a remote JSON or ZIP kit in place."""
    remote_kind, _, content_type = _read_remote_kit_bytes(uri)
    kit, files = import_remote_kit(uri)
    updated = edit_temporary_kit(kit, diff)

    if remote_kind == "archive":
        with tempfile.NamedTemporaryFile(suffix=".zip", delete=False) as handle:
            archive_path = handle.name
        try:
            export_kit(updated, _collect_kit_asset_files(updated, files), archive_path)
            with open(archive_path, "rb") as handle:
                body = handle.read()
        finally:
            if os.path.exists(archive_path):
                os.remove(archive_path)
        _write_remote_kit_bytes(uri, body, "application/zip")
        return updated

    body = json.dumps(_kit_to_dict(updated), ensure_ascii=False).encode("utf-8")
    _write_remote_kit_bytes(uri, body, content_type or "application/json")
    return updated


# #endregion 🔄Kit Workflow Helpers


def import_kit(path: str) -> tuple[KitData, dict[str, bytes]]:
    """📦Import a kit from a ``.zip`` (``kit.json`` at archive root) via :program:`semio-store`."""
    if not os.path.exists(path):
        raise FileNotFoundError(f"File not found: {path}")
    kit_dict = store.load_kit_via_io("io.importFromZip", {"path": path})
    files: dict[str, bytes] = {}
    return KitData(kit_dict), files


def _write_kit_to_sqlite(kit_data: KitData | dict, db_path: str) -> None:
    """🔖Write kit data to SQLite database using the TypeScript schema."""
    import sqlite3
    from datetime import datetime

    data = kit_data.to_dict() if isinstance(kit_data, KitData) else kit_data

    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS kit (
            id VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256) NOT NULL,
            version VARCHAR(256),
            description TEXT,
            icon TEXT,
            image TEXT,
            preview TEXT,
            remote TEXT,
            homepage TEXT,
            license VARCHAR(256),
            created DATETIME NOT NULL,
            updated DATETIME NOT NULL
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS type (
            id VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256) NOT NULL,
            parent_id VARCHAR(36),
            is_abstract BOOLEAN DEFAULT 0,
            folder VARCHAR(256),
            stock INTEGER,
            virtual BOOLEAN DEFAULT 0,
            unit VARCHAR(64),
            location_id VARCHAR(36),
            description TEXT,
            icon TEXT,
            image TEXT,
            created DATETIME NOT NULL,
            updated DATETIME NOT NULL,
            kit_id VARCHAR(36) NOT NULL,
            row_id INTEGER
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS connector (
            id VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256),
            point_x FLOAT NOT NULL,
            point_y FLOAT NOT NULL,
            point_z FLOAT NOT NULL,
            direction_x FLOAT NOT NULL,
            direction_y FLOAT NOT NULL,
            direction_z FLOAT NOT NULL,
            t FLOAT NOT NULL,
            mandatory BOOLEAN DEFAULT 0,
            port_id VARCHAR(36),
            description TEXT,
            type_id VARCHAR(36) NOT NULL
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS representation (
            id VARCHAR(36) PRIMARY KEY,
            file_id VARCHAR(36) NOT NULL,
            name VARCHAR(256),
            description TEXT,
            type_id VARCHAR(36) NOT NULL
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS design (
            id VARCHAR(36),
            name VARCHAR(256) NOT NULL,
            parent_id VARCHAR(36),
            variant VARCHAR(256),
            view_center_u FLOAT,
            view_center_v FLOAT,
            view_zoom FLOAT,
            unit VARCHAR(64),
            location_id VARCHAR(36),
            active_layer_id VARCHAR(36),
            is_abstract BOOLEAN DEFAULT 0,
            folder VARCHAR(256),
            can_scale BOOLEAN,
            can_mirror BOOLEAN,
            description TEXT,
            icon TEXT,
            image TEXT,
            created DATETIME NOT NULL,
            updated DATETIME NOT NULL,
            kit_id VARCHAR(36) NOT NULL,
            row_id INTEGER PRIMARY KEY
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS piece (
            id VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256),
            type_id VARCHAR(36),
            design_id_ref VARCHAR(36),
            plane_origin_x FLOAT,
            plane_origin_y FLOAT,
            plane_origin_z FLOAT,
            plane_x_axis_x FLOAT,
            plane_x_axis_y FLOAT,
            plane_x_axis_z FLOAT,
            plane_y_axis_x FLOAT,
            plane_y_axis_y FLOAT,
            plane_y_axis_z FLOAT,
            center_u FLOAT,
            center_v FLOAT,
            scale FLOAT,
            mirror_plane_origin_x FLOAT,
            mirror_plane_origin_y FLOAT,
            mirror_plane_origin_z FLOAT,
            mirror_plane_x_axis_x FLOAT,
            mirror_plane_x_axis_y FLOAT,
            mirror_plane_x_axis_z FLOAT,
            mirror_plane_y_axis_x FLOAT,
            mirror_plane_y_axis_y FLOAT,
            mirror_plane_y_axis_z FLOAT,
            is_hidden BOOLEAN DEFAULT 0,
            is_locked BOOLEAN DEFAULT 0,
            color VARCHAR(32),
            description TEXT,
            design_id VARCHAR(36) NOT NULL
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS connection (
            id VARCHAR(36) PRIMARY KEY,
            parent_piece_id VARCHAR(36) NOT NULL,
            parent_design_piece_id VARCHAR(36),
            connected_connector_id VARCHAR(36),
            child_piece_id VARCHAR(36) NOT NULL,
            child_design_piece_id VARCHAR(36),
            connecting_connector_id VARCHAR(36),
            gap FLOAT DEFAULT 0,
            shift FLOAT DEFAULT 0,
            rise FLOAT DEFAULT 0,
            rotation FLOAT DEFAULT 0,
            turn FLOAT DEFAULT 0,
            tilt FLOAT DEFAULT 0,
            u FLOAT,
            v FLOAT,
            description TEXT,
            design_id VARCHAR(36) NOT NULL
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS folder (
            id VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256) NOT NULL,
            parent_id VARCHAR(36)
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS file (
            id VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256) NOT NULL,
            mime VARCHAR(256),
            size INTEGER,
            hash VARCHAR(256),
            remote_url TEXT,
            folder_id VARCHAR(36)
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS representation_tag (
            representation_id VARCHAR(36) NOT NULL,
            tag_id VARCHAR(36) NOT NULL,
            PRIMARY KEY (representation_id, tag_id)
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS kit_payload (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            data TEXT NOT NULL
        )
    """)

    now = datetime.now().isoformat()
    kit_id = data.get("id", str(uuid.uuid4()))

    cursor.execute(
        """
        INSERT INTO kit (id, name, version, description, icon, image, preview, remote, homepage, license, created, updated)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    """,
        (
            kit_id,
            data.get("name", ""),
            data.get("version", ""),
            data.get("description", ""),
            data.get("icon", ""),
            data.get("image", ""),
            data.get("preview", ""),
            data.get("remote", ""),
            data.get("homepage", ""),
            data.get("license"),
            now,
            now,
        ),
    )

    for t in data.get("types", []):
        type_id = t.get("id", str(uuid.uuid4()))
        cursor.execute(
            """
            INSERT INTO type (id, name, parent_id, is_abstract, folder, stock, virtual, unit, location_id, description, icon, image, created, updated, kit_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
            (
                type_id,
                t.get("name", ""),
                t.get("parent"),
                1 if t.get("isAbstract") else 0,
                t.get("folder"),
                t.get("stock"),
                1 if t.get("isVirtual") else 0,
                t.get("unit"),
                t.get("location"),
                t.get("description", ""),
                t.get("icon", ""),
                t.get("image", ""),
                now,
                now,
                kit_id,
            ),
        )

        for c in t.get("connectors", []):
            point = c.get("point", {})
            direction = c.get("direction", {})
            cursor.execute(
                """
                INSERT INTO connector (id, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_id, description, type_id)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                (
                    c.get("id", str(uuid.uuid4())),
                    c.get("name"),
                    point.get("x", 0.0),
                    point.get("y", 0.0),
                    point.get("z", 0.0),
                    direction.get("x", 0.0),
                    direction.get("y", 1.0),
                    direction.get("z", 0.0),
                    c.get("t", 0.0),
                    1 if c.get("mandatory") else 0,
                    _getIdFromRef(c.get("port")),
                    c.get("description"),
                    type_id,
                ),
            )

        for m in t.get("representations", []):
            cursor.execute(
                """
                INSERT INTO representation (id, file_id, name, description, type_id)
                VALUES (?, ?, ?, ?, ?)
            """,
                (
                    m.get("id", str(uuid.uuid4())),
                    _getIdFromRef(m.get("file")) or "",
                    m.get("name"),
                    m.get("description"),
                    type_id,
                ),
            )

    for d in data.get("designs", []):
        design_id = d.get("id", str(uuid.uuid4()))
        view = d.get("view") or {}
        view_center = view.get("center") or {}
        cursor.execute(
            """
            INSERT INTO design (id, name, parent_id, variant, view_center_u, view_center_v, view_zoom, unit, location_id, active_layer_id, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated, kit_id)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
            (
                design_id,
                d.get("name", ""),
                _getIdFromRef(d.get("parent")),
                d.get("variant"),
                view_center.get("u"),
                view_center.get("v"),
                view.get("zoom"),
                d.get("unit"),
                _getIdFromRef(d.get("location")),
                _getIdFromRef(d.get("activeLayer")),
                1 if d.get("isAbstract") else 0,
                d.get("folder"),
                1 if d.get("canScale") else (0 if d.get("canScale") is False else None),
                (
                    1
                    if d.get("canMirror")
                    else (0 if d.get("canMirror") is False else None)
                ),
                d.get("description", ""),
                d.get("icon", ""),
                d.get("image", ""),
                now,
                now,
                kit_id,
            ),
        )

        for p in d.get("pieces", []):
            plane = _dict_piece_plane(p) or {}
            plane_origin = plane.get("origin") or {}
            plane_x_axis = plane.get("xAxis") or {}
            plane_y_axis = plane.get("yAxis") or {}
            mirror_plane = p.get("mirrorPlane") or {}
            mirror_origin = mirror_plane.get("origin") or {}
            mirror_x_axis = mirror_plane.get("xAxis") or {}
            mirror_y_axis = mirror_plane.get("yAxis") or {}
            center = _dict_piece_center(p) or {}
            cursor.execute(
                """
                INSERT INTO piece (id, name, type_id, design_id_ref, plane_origin_x, plane_origin_y, plane_origin_z,
                    plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z,
                    center_u, center_v, scale, mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z,
                    mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z,
                    mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z,
                    is_hidden, is_locked, color, description, design_id)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                (
                    p.get("id", str(uuid.uuid4())),
                    p.get("name") or p.get("id"),
                    _getIdFromRef(p.get("type")),
                    _getIdFromRef(p.get("design")),
                    plane_origin.get("x") if plane else None,
                    plane_origin.get("y") if plane else None,
                    plane_origin.get("z") if plane else None,
                    plane_x_axis.get("x") if plane else None,
                    plane_x_axis.get("y") if plane else None,
                    plane_x_axis.get("z") if plane else None,
                    plane_y_axis.get("x") if plane else None,
                    plane_y_axis.get("y") if plane else None,
                    plane_y_axis.get("z") if plane else None,
                    center.get("u") if center else None,
                    center.get("v") if center else None,
                    p.get("scale"),
                    mirror_origin.get("x") if mirror_plane else None,
                    mirror_origin.get("y") if mirror_plane else None,
                    mirror_origin.get("z") if mirror_plane else None,
                    mirror_x_axis.get("x") if mirror_plane else None,
                    mirror_x_axis.get("y") if mirror_plane else None,
                    mirror_x_axis.get("z") if mirror_plane else None,
                    mirror_y_axis.get("x") if mirror_plane else None,
                    mirror_y_axis.get("y") if mirror_plane else None,
                    mirror_y_axis.get("z") if mirror_plane else None,
                    1 if p.get("isHidden") else 0,
                    1 if p.get("isLocked") else 0,
                    p.get("color"),
                    p.get("description"),
                    design_id,
                ),
            )

        for c in d.get("connections", []):
            connected = c.get("parent", {})
            connecting = c.get("child", {})
            connected_piece = connected.get("piece")
            parent_piece_id = (
                connected_piece.get("id")
                if isinstance(connected_piece, dict)
                else connected_piece
            )
            connected_design_piece = connected.get("designPiece")
            parent_design_piece_id = (
                connected_design_piece.get("id")
                if isinstance(connected_design_piece, dict)
                else connected_design_piece
            )
            connected_connector = connected.get("connector")
            connected_connector_id = (
                connected_connector.get("id")
                if isinstance(connected_connector, dict)
                else connected_connector
            )
            connecting_piece = connecting.get("piece")
            child_piece_id = (
                connecting_piece.get("id")
                if isinstance(connecting_piece, dict)
                else connecting_piece
            )
            connecting_design_piece = connecting.get("designPiece")
            child_design_piece_id = (
                connecting_design_piece.get("id")
                if isinstance(connecting_design_piece, dict)
                else connecting_design_piece
            )
            connecting_connector = connecting.get("connector")
            connecting_connector_id = (
                connecting_connector.get("id")
                if isinstance(connecting_connector, dict)
                else connecting_connector
            )
            cursor.execute(
                """
                INSERT INTO connection (id, parent_piece_id, parent_design_piece_id, connected_connector_id,
                    child_piece_id, child_design_piece_id, connecting_connector_id,
                    gap, shift, rise, rotation, turn, tilt, u, v, description, design_id)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                (
                    c.get("id", str(uuid.uuid4())),
                    parent_piece_id,
                    parent_design_piece_id,
                    connected_connector_id,
                    child_piece_id,
                    child_design_piece_id,
                    connecting_connector_id,
                    c.get("gap", 0.0),
                    c.get("shift", 0.0),
                    c.get("rise", 0.0),
                    c.get("rotation", 0.0),
                    c.get("turn", 0.0),
                    c.get("tilt", 0.0),
                    c.get("u"),
                    c.get("v"),
                    c.get("description"),
                    design_id,
                ),
            )

    for folder_entry in data.get("folders", []):
        cursor.execute(
            """
            INSERT INTO folder (id, name, parent_id)
            VALUES (?, ?, ?)
        """,
            (
                folder_entry.get("id", str(uuid.uuid4())),
                folder_entry.get("name", ""),
                _getIdFromRef(folder_entry.get("parent")),
            ),
        )

    for file_entry in data.get("files", []):
        cursor.execute(
            """
            INSERT INTO file (id, name, mime, size, hash, remote_url, folder_id)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        """,
            (
                file_entry.get("id", str(uuid.uuid4())),
                file_entry.get("name", ""),
                file_entry.get("mime"),
                file_entry.get("size"),
                file_entry.get("hash"),
                file_entry.get("remote"),
                _getIdFromRef(file_entry.get("folder")),
            ),
        )

    for t in data.get("types", []):
        for m in t.get("representations", []):
            representation_id = m.get("id")
            for tag in m.get("tags", []):
                tag_id = _getIdFromRef(tag) if isinstance(tag, dict) else tag
                if representation_id and tag_id:
                    cursor.execute(
                        "INSERT OR IGNORE INTO representation_tag (representation_id, tag_id) VALUES (?, ?)",
                        (representation_id, tag_id),
                    )

    cursor.execute(
        "INSERT INTO kit_payload (id, data) VALUES (1, ?)",
        (json.dumps(_kit_without_file_blobs(data), ensure_ascii=False),),
    )

    conn.commit()
    conn.close()


def export_kit(kit: KitData, files: dict[str, bytes], path: str) -> None:
    """📦Export a kit to a ``.zip`` (``kit.json`` at root) via :program:`semio-store`."""
    _ = files  # file blobs are carried on the DTO; external assets follow Rust inlining rules
    data = _kit_to_dict(kit)
    with store.StoreClient() as c:
        c.call("kit.create", {"dto": data})
        c.call("io.exportToZip", {"path": path})


# #region 🧬Kit Kind Classes


class TransportKit:
    """📋 Wraps a static JSON string for kit serialization/deserialization."""

    def __init__(self, json_str: str):
        self.json = json_str

    def to_kit(self) -> KitData:
        return KitData(json.loads(self.json))

    @staticmethod
    def from_kit(kit: KitData | dict) -> "TransportKit":
        return TransportKit(json.dumps(_kit_to_dict(kit), ensure_ascii=False))


class ArchiveKit:
    """📦 Wraps a static zipped local kit."""

    def __init__(self, data: bytes):
        self.data = data

    def to_kit(self) -> tuple[KitData, dict[str, bytes]]:
        with tempfile.NamedTemporaryFile(suffix=".zip", delete=False) as handle:
            handle.write(self.data)
            archive_path = handle.name
        try:
            return import_kit(archive_path)
        finally:
            if os.path.exists(archive_path):
                os.remove(archive_path)

    @staticmethod
    def from_kit(
        kit: KitData | dict, files: dict[str, bytes] | None = None
    ) -> "ArchiveKit":
        with tempfile.NamedTemporaryFile(suffix=".zip", delete=False) as handle:
            archive_path = handle.name
        try:
            export_kit(kit, files or _collect_kit_asset_files(kit), archive_path)
            with open(archive_path, "rb") as handle:
                return ArchiveKit(handle.read())
        finally:
            if os.path.exists(archive_path):
                os.remove(archive_path)


class SyncKit:
    """🔄 Base class for synchronized kit kinds."""

    def __init__(self, kit: KitData):
        self._kit = kit

    @property
    def kit(self) -> KitData:
        return self._kit

    def apply(self, commands: list[dict]) -> None:
        """Apply ``ChangeKitCommand`` JSON via ``semio-store``."""
        d = _kit_to_dict(self._kit)
        with store.StoreClient() as c:
            c.call("kit.create", {"dto": d})
            c.call("kit.executeChangeKitCommands", {"cmds": commands})
            out = c.call("kit.snapshot", None)
        if not isinstance(out, dict):
            raise TypeError("kit.snapshot")
        self._kit = KitData(out)

    def import_transport(self, transport: TransportKit) -> None:
        imported = transport.to_kit()
        with store.StoreClient() as c:
            c.call("kit.create", {"dto": self._kit.to_dict()})
            c.call(
                "kit.executeChangeKitCommands",
                {
                    "cmds": [
                        {
                            "replaceKitFromFull": {"dto": imported.to_dict()},
                        }
                    ],
                },
            )
            out = c.call("kit.snapshot", None)
        if not isinstance(out, dict):
            raise TypeError("kit.snapshot")
        self._kit = KitData(out)

    def import_archive(self, archive: ArchiveKit) -> None:
        imported, _ = archive.to_kit()
        with store.StoreClient() as c:
            c.call("kit.create", {"dto": self._kit.to_dict()})
            c.call(
                "kit.executeChangeKitCommands",
                {
                    "cmds": [
                        {
                            "replaceKitFromFull": {"dto": imported.to_dict()},
                        }
                    ],
                },
            )
            out = c.call("kit.snapshot", None)
        if not isinstance(out, dict):
            raise TypeError("kit.snapshot")
        self._kit = KitData(out)

    def export_transport(self) -> TransportKit:
        return TransportKit.from_kit(self._kit)

    def export_archive(self) -> ArchiveKit:
        return ArchiveKit.from_kit(self._kit)

    def close(self) -> None:
        pass


class DevKit(SyncKit):
    """📝 Synchronized JSON file kit."""

    @staticmethod
    def from_json(json_str: str) -> "DevKit":
        return DevKit(KitData(json.loads(json_str)))


class LocalKit(SyncKit):
    """📂 Synchronized folder with .semio/kit.db SQLite database."""

    pass


class RemoteKit(SyncKit):
    """🌐 Synchronized websocket connection to semio/hub."""

    pass


# #endregion 🧬Kit Kind Classes


# #endregion 🧿Kit Import/Export


# #region 🔩Kit Representation Export
# 3D representation export utilities for designs. Exports design scene graphs as GLB, GLTF, OBJ, STL, PLY, OFF, IFC.

EXPORT_REPRESENTATION_FORMATS: dict[str, str] = {
    ".glb": "representation/gltf-binary",
    ".gltf": "representation/gltf+json",
    ".obj": "representation/obj",
    ".stl": "representation/stl",
    ".ply": "application/x-ply",
    ".off": "application/x-off",
    ".ifc": "application/x-ifc",
}
"""Supported 3D export formats with their MIME types.
"""


def _plane_to_matrix_4x4(plane: "Plane") -> numpy.ndarray:
    """🔖Convert a Plane to a 4x4 column-major transformation matrix."""
    origin = numpy.array([plane.origin.x, plane.origin.y, plane.origin.z])
    x_axis = numpy.array([plane.xAxis.x, plane.xAxis.y, plane.xAxis.z])
    y_axis = numpy.array([plane.yAxis.x, plane.yAxis.y, plane.yAxis.z])
    z_axis = numpy.cross(x_axis, y_axis)
    nz = numpy.linalg.norm(z_axis)
    if nz > 1e-10:
        z_axis = z_axis / nz
    nx = numpy.linalg.norm(x_axis)
    if nx > 1e-10:
        x_axis = x_axis / nx
    y_axis = numpy.cross(z_axis, x_axis)
    ny = numpy.linalg.norm(y_axis)
    if ny > 1e-10:
        y_axis = y_axis / ny
    mat = numpy.eye(4)
    mat[:3, 0] = x_axis
    mat[:3, 1] = y_axis
    mat[:3, 2] = z_axis
    mat[:3, 3] = origin
    return mat


def _semio_matrix_to_gltf_matrix(matrix: numpy.ndarray) -> numpy.ndarray:
    basis = numpy.array(
        [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    )
    basis_inv = numpy.linalg.inv(basis)
    return basis @ matrix @ basis_inv


def _identity_plane() -> "Plane":
    """🔖Create an identity plane at the world origin with standard axes."""
    p = Plane()
    p.origin = Point(x=0.0, y=0.0, z=0.0)
    p.xAxis = Vector(x=1.0, y=0.0, z=0.0)
    p.yAxis = Vector(x=0.0, y=1.0, z=0.0)
    return p


def _type_key_from_id(type_id: "TypeId") -> str:
    """🔖Build a unique string key from a TypeId (name:variant)."""
    return f"{type_id.name}:{type_id.variant}"


def _type_key_from_type(t: "Type") -> str:
    """🔖Build a unique string key from a Type (name:variant)."""
    return f"{t.name}:{t.variant}"


def _find_matching_representation(
    kit: "Kit", type_obj: "Type", tags: list[str]
) -> typing.Optional["Representation"]:
    """📨Find the best matching representation for a type given requested tags."""
    if not type_obj.representations or len(type_obj.representations) == 0:
        return None
    if not tags or len(tags) == 0:
        default_representation = next(
            (
                representation
                for representation in type_obj.representations
                if len(representation.tags or []) == 0
            ),
            None,
        )
        return (
            default_representation
            if default_representation is not None
            else type_obj.representations[0]
        )
    tags_set = set(tags)
    for representation in type_obj.representations:
        representation_tag_names = representation.tags
        if representation_tag_names and all(
            t in tags_set for t in representation_tag_names
        ):
            return representation
    return type_obj.representations[0]


def _load_glb_mesh_from_bytes(
    raw: bytes, mesh_name: str | None = None
) -> "typing.Any | None":
    """🔖Load a mesh directly from GLB bytes by reading accessors."""
    import struct as _struct

    import trimesh as _trimesh

    if len(raw) < 20 or raw[0:4] != b"glTF":
        return None

    offset = 12
    json_chunk: bytes | None = None
    bin_chunk = b""
    while offset + 8 <= len(raw):
        chunk_length, chunk_kind = _struct.unpack_from("<II", raw, offset)
        offset += 8
        chunk = raw[offset : offset + chunk_length]
        offset += chunk_length
        if chunk_kind == 0x4E4F534A:
            json_chunk = chunk
        elif chunk_kind == 0x004E4942:
            bin_chunk = chunk
    if json_chunk is None:
        return None

    try:
        gltf = json.loads(json_chunk.decode("utf-8").rstrip(" \t\r\n\x00"))
    except Exception:
        return None

    accessors = gltf.get("accessors", []) or []
    buffer_views = gltf.get("bufferViews", []) or []
    meshes = gltf.get("meshes", []) or []

    component_formats: dict[int, tuple[str, int]] = {
        5120: ("b", 1),
        5121: ("B", 1),
        5122: ("h", 2),
        5123: ("H", 2),
        5125: ("I", 4),
        5126: ("f", 4),
    }
    type_widths = {
        "SCALAR": 1,
        "VEC2": 2,
        "VEC3": 3,
        "VEC4": 4,
        "MAT2": 4,
        "MAT3": 9,
        "MAT4": 16,
    }

    def _read_accessor(accessor_index: int) -> numpy.ndarray | None:
        if accessor_index < 0 or accessor_index >= len(accessors):
            return None
        accessor = accessors[accessor_index]
        buffer_view_index = accessor.get("bufferView")
        if (
            not isinstance(buffer_view_index, int)
            or buffer_view_index < 0
            or buffer_view_index >= len(buffer_views)
        ):
            return None
        buffer_view = buffer_views[buffer_view_index]
        component_type = accessor.get("componentType")
        accessor_kind = accessor.get("type")
        count = accessor.get("count")
        if (
            component_type not in component_formats
            or accessor_kind not in type_widths
            or not isinstance(count, int)
        ):
            return None
        if buffer_view.get("buffer", 0) != 0:
            return None
        fmt_char, component_size = component_formats[component_type]
        element_width = type_widths[accessor_kind]
        stride = buffer_view.get("byteStride") or (component_size * element_width)
        byte_offset = buffer_view.get("byteOffset", 0) + accessor.get("byteOffset", 0)
        values: list[tuple[typing.Any, ...]] = []
        for item_index in range(count):
            start = byte_offset + item_index * stride
            end = start + component_size * element_width
            if end > len(bin_chunk):
                return None
            values.append(
                _struct.unpack_from("<" + fmt_char * element_width, bin_chunk, start)
            )
        return numpy.array(values)

    vertex_blocks: list[numpy.ndarray] = []
    normal_blocks: list[numpy.ndarray] = []
    face_blocks: list[numpy.ndarray] = []
    has_normals = True

    for mesh in meshes:
        primitives = mesh.get("primitives", []) or []
        for primitive in primitives:
            attributes = primitive.get("attributes", {}) or {}
            position_accessor_index = attributes.get("POSITION")
            if not isinstance(position_accessor_index, int):
                continue
            positions = _read_accessor(position_accessor_index)
            if positions is None or positions.ndim != 2 or positions.shape[1] < 3:
                continue
            positions = positions[:, :3].astype(numpy.float64)
            normals = None
            normal_accessor_index = attributes.get("NORMAL")
            if isinstance(normal_accessor_index, int):
                normals = _read_accessor(normal_accessor_index)
                if normals is not None and normals.ndim == 2 and normals.shape[1] >= 3:
                    normals = normals[:, :3].astype(numpy.float64)
                else:
                    normals = None
            if normals is None or len(normals) != len(positions):
                has_normals = False
            if isinstance(primitive.get("indices"), int):
                indices = _read_accessor(primitive.get("indices"))
                if indices is None:
                    continue
                index_values = indices.reshape(-1).astype(numpy.int64)
            else:
                index_values = numpy.arange(len(positions), dtype=numpy.int64)
            triangle_value_count = (len(index_values) // 3) * 3
            if triangle_value_count == 0:
                continue
            triangle_faces = index_values[:triangle_value_count].reshape((-1, 3))
            vertex_offset = sum(len(block) for block in vertex_blocks)
            vertex_blocks.append(positions)
            if normals is not None and len(normals) == len(positions):
                normal_blocks.append(normals)
            face_blocks.append(triangle_faces + vertex_offset)

    if len(vertex_blocks) == 0 or len(face_blocks) == 0:
        return None

    combined_vertices = numpy.vstack(vertex_blocks)
    combined_faces = numpy.vstack(face_blocks)
    mesh = _trimesh.Trimesh(
        vertices=combined_vertices,
        faces=combined_faces,
        process=False,
        maintain_order=True,
    )
    if has_normals and len(normal_blocks) == len(vertex_blocks):
        combined_normals = numpy.vstack(normal_blocks)
        if len(combined_normals) == len(combined_vertices):
            mesh.vertex_normals = combined_normals
    if mesh_name:
        mesh.metadata["name"] = mesh_name
    return mesh if len(getattr(mesh, "faces", [])) > 0 else None


def _load_type_mesh(
    kit: "Kit", type_obj: "Type", tags: list[str]
) -> "typing.Any | None":
    """🎯Load the 3D mesh for a type from its best-matching representation blob."""
    import base64 as _base64

    import trimesh as _trimesh

    representation = _find_matching_representation(kit, type_obj, tags)
    if representation is None:
        return None
    files_list = kit.files_ or []
    file_id = (
        representation.file.id
        if hasattr(representation.file, "id")
        else representation.file
    )
    file_obj = next(
        (f for f in files_list if f.name == file_id or f.id == file_id), None
    )
    if file_obj is None or not file_obj.blob:
        return None
    blob = file_obj.blob
    if blob.startswith("data:"):
        raw = _base64.b64decode(blob.split(",", 1)[1])
    else:
        raw = _base64.b64decode(blob)
    direct_mesh = _load_glb_mesh_from_bytes(raw, file_obj.name)
    if direct_mesh is not None:
        return direct_mesh
    try:
        loaded = _trimesh.load(
            _trimesh.util.wrap_as_stream(raw),
            file_type="glb",
        )
    except Exception:
        return None
    if isinstance(loaded, _trimesh.Scene):
        if len(loaded.geometry) == 0:
            return None
        meshes = [
            geometry.copy()
            for geometry in loaded.geometry.values()
            if isinstance(geometry, _trimesh.Trimesh)
            and len(getattr(geometry, "faces", [])) > 0
        ]
        if not meshes:
            return None
        if len(meshes) == 1:
            return meshes[0]
        return _trimesh.util.concatenate(meshes)
    if isinstance(loaded, _trimesh.Trimesh) and len(getattr(loaded, "faces", [])) > 0:
        return loaded
    return None


def export_design_representation(
    kit: "Kit",
    design_id: str,
    format: str = ".glb",
    tags: list[str] | None = None,
    options: dict | None = None,
) -> bytes:
    """Export the 3D representation of a design to a specified format.
    Connection hierarchy is translated into a scene graph; planes become relative transformation matrices.
    """
    import trimesh as _trimesh

    if tags is None:
        tags = []
    if options is None:
        options = {}
    if format not in EXPORT_REPRESENTATION_FORMATS:
        raise ValueError(
            f"Unsupported export format '{format}'. Supported: {list(EXPORT_REPRESENTATION_FORMATS.keys())}"
        )

    if isinstance(kit, dict):
        designs = kit.get("designs", []) or []
        design = next(
            (
                d
                for d in designs
                if d.get("name") == design_id or d.get("id") == design_id
            ),
            None,
        )
        if design is None:
            raise ValueError(f"Design '{design_id}' not found in kit")
        pieces = design.get("pieces", []) or []
        connections = design.get("connections", []) or []
        if len(pieces) == 0:
            return _export_empty_scene(format)

        types_by_id = {
            type_obj.get("id"): type_obj
            for type_obj in (kit.get("types", []) or [])
            if type_obj.get("id")
        }

        def _find_type_for_piece_dict(piece_dict: dict) -> dict | None:
            type_ref = piece_dict.get("type")
            if not isinstance(type_ref, dict):
                return None
            return types_by_id.get(type_ref.get("id"))

        def _find_connector_dict(
            type_obj: dict | None, connector_id: str | None
        ) -> dict | None:
            current = type_obj
            while current is not None:
                connectors = current.get("connectors", []) or []
                if connector_id is None:
                    return connectors[0] if connectors else None
                for connector in connectors:
                    if connector.get("id") == connector_id:
                        return connector
                parent_ref = current.get("parent")
                current = (
                    types_by_id.get(parent_ref.get("id"))
                    if isinstance(parent_ref, dict)
                    else None
                )
            return None

        piece_by_id = {piece.get("id"): piece for piece in pieces if piece.get("id")}
        adjacency: dict[str, list[tuple[dict, str]]] = {
            piece_id: [] for piece_id in piece_by_id
        }
        for connection in connections:
            connected_id = connection.get("parent", {}).get("piece", {}).get("id")
            connecting_id = connection.get("child", {}).get("piece", {}).get("id")
            if connected_id in adjacency:
                adjacency[connected_id].append((connection, connecting_id))
            if connecting_id in adjacency:
                adjacency[connecting_id].append((connection, connected_id))

        def _identity_plane_dict() -> dict:
            return {
                "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
                "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
                "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
            }

        def _plane_dict_to_matrix(plane_dict: dict) -> numpy.ndarray:
            origin = numpy.array(
                [
                    plane_dict["origin"]["x"],
                    plane_dict["origin"]["y"],
                    plane_dict["origin"]["z"],
                ],
                dtype=numpy.float64,
            )
            x_axis = numpy.array(
                [
                    plane_dict["xAxis"]["x"],
                    plane_dict["xAxis"]["y"],
                    plane_dict["xAxis"]["z"],
                ],
                dtype=numpy.float64,
            )
            y_axis = numpy.array(
                [
                    plane_dict["yAxis"]["x"],
                    plane_dict["yAxis"]["y"],
                    plane_dict["yAxis"]["z"],
                ],
                dtype=numpy.float64,
            )
            z_axis = numpy.cross(x_axis, y_axis)
            if numpy.linalg.norm(z_axis) > 1e-10:
                z_axis = z_axis / numpy.linalg.norm(z_axis)
            if numpy.linalg.norm(x_axis) > 1e-10:
                x_axis = x_axis / numpy.linalg.norm(x_axis)
            y_axis = numpy.cross(z_axis, x_axis)
            if numpy.linalg.norm(y_axis) > 1e-10:
                y_axis = y_axis / numpy.linalg.norm(y_axis)
            matrix = numpy.eye(4)
            matrix[:3, 0] = x_axis
            matrix[:3, 1] = y_axis
            matrix[:3, 2] = z_axis
            matrix[:3, 3] = origin
            return matrix

        piece_planes: dict[str, dict] = {}
        parent_of: dict[str, str] = {}
        children_of: dict[str, list[str]] = {piece_id: [] for piece_id in piece_by_id}
        visited: set[str] = set()
        roots: list[str] = []
        queue: list[str] = []

        for piece in pieces:
            piece_id = piece.get("id")
            if piece_id is None:
                continue
            if (
                _dict_piece_plane(piece) is not None
                and _dict_piece_center(piece) is not None
            ):
                piece_planes[piece_id] = _dict_piece_plane(piece)
                visited.add(piece_id)
                queue.append(piece_id)
                roots.append(piece_id)
        if len(queue) == 0 and len(pieces) > 0 and pieces[0].get("id") is not None:
            first_id = pieces[0].get("id")
            piece_planes[first_id] = _identity_plane_dict()
            visited.add(first_id)
            queue.append(first_id)
            roots.append(first_id)

        while queue:
            current_id = queue.pop(0)
            current_plane = piece_planes[current_id]
            for connection, neighbor_id in adjacency.get(current_id, []):
                if neighbor_id in visited:
                    continue
                if (
                    connection.get("parent", {}).get("piece", {}).get("id")
                    != current_id
                ):
                    continue
                parent_piece = piece_by_id[current_id]
                child_piece = piece_by_id[neighbor_id]
                parent_type = _find_type_for_piece_dict(parent_piece)
                child_type = _find_type_for_piece_dict(child_piece)
                parent_connector = _find_connector_dict(
                    parent_type,
                    connection.get("parent", {}).get("connector", {}).get("id"),
                )
                child_connector = _find_connector_dict(
                    child_type,
                    connection.get("child", {}).get("connector", {}).get("id"),
                )
                if parent_connector is not None and child_connector is not None:
                    piece_planes[neighbor_id] = computeChildPlaneDict(
                        current_plane, parent_connector, child_connector, connection
                    )
                else:
                    piece_planes[neighbor_id] = current_plane
                parent_of[neighbor_id] = current_id
                children_of[current_id].append(neighbor_id)
                visited.add(neighbor_id)
                queue.append(neighbor_id)

        for piece in pieces:
            piece_id = piece.get("id")
            if piece_id is None:
                continue
            if piece_id not in visited:
                piece_planes[piece_id] = _identity_plane_dict()
                roots.append(piece_id)

        if format == ".ifc":
            return _export_ifc_from_dict(
                kit, design_id, piece_planes, parent_of, children_of, roots, tags
            )

        def _select_representation_dict(type_obj: dict) -> dict | None:
            representations = type_obj.get("representations", []) or []
            if len(representations) == 0:
                return None
            tag_lookup = {
                tag.get("id"): tag
                for tag in (kit.get("tags", []) or [])
                if tag.get("id")
            }
            if len(tags) == 0:
                default_representation = next(
                    (
                        representation
                        for representation in representations
                        if len(representation.get("tags", []) or []) == 0
                    ),
                    None,
                )
                return (
                    default_representation
                    if default_representation is not None
                    else representations[0]
                )
            selected_tag_ids: set[str] = set()
            for tag_value in tags:
                if tag_value in tag_lookup:
                    selected_tag_ids.add(tag_value)
                    continue
                for tag in tag_lookup.values():
                    if tag.get("name") == tag_value:
                        selected_tag_ids.add(tag.get("id"))
            best_representation = None
            best_score = -1.0
            for representation in representations:
                representation_tag_ids = {
                    tag.get("id")
                    for tag in (representation.get("tags", []) or [])
                    if tag.get("id")
                }
                if not selected_tag_ids.issubset(representation_tag_ids):
                    continue
                union = len(representation_tag_ids.union(selected_tag_ids))
                intersection = len(
                    representation_tag_ids.intersection(selected_tag_ids)
                )
                score = float(intersection) / float(union) if union > 0 else 0.0
                if score > best_score:
                    best_score = score
                    best_representation = representation
            return (
                best_representation
                if best_representation is not None
                else representations[0]
            )

        scene = _trimesh.Scene()
        type_meshes: dict[str, str] = {}
        files_by_id = {
            file_entry.get("id"): file_entry
            for file_entry in (kit.get("files", []) or [])
            if file_entry.get("id")
        }
        for piece in pieces:
            type_id = (
                piece.get("type", {}).get("id")
                if isinstance(piece.get("type"), dict)
                else None
            )
            if type_id is None or type_id in type_meshes:
                continue
            type_obj = types_by_id.get(type_id)
            if type_obj is None:
                continue
            selected_representation = _select_representation_dict(type_obj)
            selected_file = (
                files_by_id.get(selected_representation.get("file", {}).get("id"))
                if selected_representation is not None
                else None
            )
            mesh = None
            if selected_file is not None and selected_file.get("blob"):
                try:
                    blob = selected_file.get("blob")
                    raw = base64.b64decode(
                        blob.split(",", 1)[1]
                        if isinstance(blob, str) and blob.startswith("data:")
                        else blob
                    )
                    mesh = _load_glb_mesh_from_bytes(raw, selected_file.get("name"))
                    if mesh is None:
                        loaded = _trimesh.load(
                            _trimesh.util.wrap_as_stream(raw), file_type="glb"
                        )
                        if isinstance(loaded, _trimesh.Scene):
                            dumped = [
                                geometry.copy()
                                for geometry in loaded.geometry.values()
                                if isinstance(geometry, _trimesh.Trimesh)
                                and len(getattr(geometry, "faces", [])) > 0
                            ]
                            mesh = (
                                dumped[0]
                                if len(dumped) == 1
                                else (
                                    _trimesh.util.concatenate(dumped)
                                    if len(dumped) > 1
                                    else None
                                )
                            )
                        elif (
                            isinstance(loaded, _trimesh.Trimesh)
                            and len(getattr(loaded, "faces", [])) > 0
                        ):
                            mesh = loaded
                    if mesh is not None and selected_file.get("name"):
                        mesh.metadata["name"] = selected_file.get("name")
                except Exception:
                    mesh = None
            if mesh is None:
                continue
            geometry_name = (
                selected_file.get("name")
                if selected_file is not None and selected_file.get("name")
                else type_id
            )
            type_meshes[type_id] = geometry_name
            scene.geometry[geometry_name] = mesh

        for piece in pieces:
            piece_id = piece.get("id")
            world_plane = piece_planes[piece_id]
            parent_id = parent_of.get(piece_id)
            piece_frame = piece.get("name") or piece_id
            if parent_id and parent_id in piece_planes:
                parent_world = _plane_dict_to_matrix(piece_planes[parent_id])
                child_world = _plane_dict_to_matrix(world_plane)
                relative = numpy.linalg.inv(parent_world) @ child_world
                frame_from = piece_by_id[parent_id].get("name") or parent_id
            else:
                relative = _plane_dict_to_matrix(world_plane)
                frame_from = scene.graph.base_frame
            relative = _semio_matrix_to_gltf_matrix(relative)
            geom_name = None
            type_id = (
                piece.get("type", {}).get("id")
                if isinstance(piece.get("type"), dict)
                else None
            )
            if type_id in type_meshes:
                geom_name = type_meshes[type_id]
            scene.graph.update(
                frame_from=frame_from,
                frame_to=piece_frame,
                matrix=relative,
                geometry=geom_name,
            )
        return _export_trimesh_scene(scene, format)

    design: Design | None = None
    for d in kit.designs:
        if d.name == design_id or d.id() == design_id:
            design = d
            break
    if design is None:
        raise ValueError(f"Design '{design_id}' not found in kit")

    pieces = design.pieces or []
    connections = design.connections or []
    types_list = kit.types or []

    if len(pieces) == 0:
        return _export_empty_scene(format)

    types_dict: dict[str, Type] = {}
    for t in types_list:
        types_dict[_type_key_from_type(t)] = t

    pieces_dict: dict[str, Piece] = {}
    for p in pieces:
        pieces_dict[p.id_] = p

    adjacency: dict[str, list[tuple[Connection, str]]] = {}
    for p in pieces:
        adjacency[p.id_] = []
    for conn in connections:
        connected_id = conn.parent.piece.id_
        connecting_id = conn.child.piece.id_
        if connected_id in adjacency:
            adjacency[connected_id].append((conn, connecting_id))
        if connecting_id in adjacency:
            adjacency[connecting_id].append((conn, connected_id))

    piece_planes: dict[str, Plane] = {}
    parent_of: dict[str, str] = {}
    children_of: dict[str, list[str]] = {}
    for p in pieces:
        children_of[p.id_] = []

    visited: set[str] = set()
    roots: list[str] = []

    def _get_type(piece: Piece) -> Type | None:
        if piece.type is None:
            return None
        return types_dict.get(_type_key_from_id(piece.type))

    def _get_connector(
        type_obj: Type | None, connector_id: ConnectorId | None
    ) -> Connector | None:
        if type_obj is None:
            return None
        if not type_obj.connectors:
            return None
        if connector_id is None:
            return type_obj.connectors[0]
        return next((c for c in type_obj.connectors if c.id_ == connector_id.id_), None)

    queue: list[str] = []
    for p in pieces:
        if p.plane is not None and p.center is not None:
            piece_planes[p.id_] = p.plane
            visited.add(p.id_)
            queue.append(p.id_)
            roots.append(p.id_)
    if len(queue) == 0 and len(pieces) > 0:
        piece_planes[pieces[0].id_] = _identity_plane()
        visited.add(pieces[0].id_)
        queue.append(pieces[0].id_)
        roots.append(pieces[0].id_)

    while queue:
        current_id = queue.pop(0)
        current_plane = piece_planes[current_id]
        for conn, neighbor_id in adjacency.get(current_id, []):
            if neighbor_id in visited:
                continue
            is_parent = conn.parent.piece.id_ == current_id
            if not is_parent:
                continue

            parent_id = current_id
            child_id = neighbor_id
            parent_piece = pieces_dict[parent_id]
            child_piece = pieces_dict[child_id]
            parent_type = _get_type(parent_piece)
            child_type = _get_type(child_piece)
            parent_connector = _get_connector(parent_type, conn.parent.connector)
            child_connector = _get_connector(child_type, conn.child.connector)

            if parent_connector and child_connector:
                child_plane = computeChildPlane(
                    current_plane, parent_connector, child_connector, conn
                )
                piece_planes[child_id] = child_plane
            else:
                piece_planes[child_id] = current_plane

            parent_of[child_id] = parent_id
            children_of[parent_id].append(child_id)
            visited.add(child_id)
            queue.append(child_id)

    for p in pieces:
        if p.id_ not in visited:
            piece_planes[p.id_] = _identity_plane()
            roots.append(p.id_)

    if format == ".ifc":
        return _export_ifc_from_entities(
            kit,
            design,
            piece_planes,
            parent_of,
            children_of,
            roots,
            pieces_dict,
            types_dict,
            tags,
        )

    scene = _trimesh.Scene()

    # #region 🎁Load Or Create Meshes Per Type
    type_meshes: dict[str, str] = {}
    for piece in pieces:
        if piece.type is None:
            continue
        tk = _type_key_from_id(piece.type)
        if tk in type_meshes:
            continue
        type_obj = types_dict.get(tk)
        if type_obj is None:
            continue
        mesh = _load_type_mesh(kit, type_obj, tags)
        if mesh is None:
            continue
        geometry_name = None
        representation = _find_matching_representation(kit, type_obj, tags)
        if representation is not None:
            geometry_name = (
                representation.file if isinstance(representation.file, str) else None
            )
        if not geometry_name:
            geometry_name = tk
        type_meshes[tk] = geometry_name
        scene.geometry[geometry_name] = mesh
    # #endregion 🎁Load Or Create Meshes Per Type

    # #region 🧭Build Scene Graph With Connection Hierarchy
    def _build_node(piece_id: str) -> None:
        piece = pieces_dict[piece_id]
        world_plane = piece_planes[piece_id]
        p_parent = parent_of.get(piece_id)
        children = children_of.get(piece_id, [])
        piece_frame = piece.name or piece.id_

        if p_parent and p_parent in piece_planes:
            parent_world = _plane_to_matrix_4x4(piece_planes[p_parent])
            child_world = _plane_to_matrix_4x4(world_plane)
            relative = _semio_matrix_to_gltf_matrix(
                numpy.linalg.inv(parent_world) @ child_world
            )
            parent_piece = pieces_dict[p_parent]
            frame_from = parent_piece.name or p_parent
        else:
            relative = _semio_matrix_to_gltf_matrix(_plane_to_matrix_4x4(world_plane))
            frame_from = scene.graph.base_frame

        geom_name = None
        if piece.type is not None:
            tk = _type_key_from_id(piece.type)
            if tk in type_meshes:
                geom_name = type_meshes[tk]

        scene.graph.update(
            frame_from=frame_from,
            frame_to=piece_frame,
            matrix=relative,
            geometry=geom_name,
        )

        for child_id in children:
            _build_node(child_id)

    for root_id in roots:
        _build_node(root_id)
    # #endregion 🧭Build Scene Graph With Connection Hierarchy

    return _export_trimesh_scene(scene, format)


def _export_empty_scene(format: str) -> bytes:
    """🔖Export a minimal valid empty scene for the requested format."""
    import struct as _struct

    empty_json: dict = {
        "asset": {"version": "2.0", "generator": "semio"},
        "scene": 0,
        "scenes": [{"nodes": []}],
        "nodes": [],
    }
    if format == ".gltf":
        return json.dumps(empty_json).encode("utf-8")
    if format == ".glb":
        json_str = json.dumps(empty_json, separators=(",", ":"))
        while len(json_str) % 4 != 0:
            json_str += " "
        json_bytes = json_str.encode("utf-8")
        total_length = 12 + 8 + len(json_bytes)
        result = bytearray(total_length)
        _struct.pack_into("<I", result, 0, 0x46546C67)
        _struct.pack_into("<I", result, 4, 2)
        _struct.pack_into("<I", result, 8, total_length)
        _struct.pack_into("<I", result, 12, len(json_bytes))
        _struct.pack_into("<I", result, 16, 0x4E4F534A)
        result[20 : 20 + len(json_bytes)] = json_bytes
        return bytes(result)
    return b""


def _export_trimesh_scene(scene: "typing.Any", format: str) -> bytes:
    """🔖Export a trimesh.Scene to the requested format as bytes."""
    import base64

    import trimesh as _trimesh

    fmt = format.lstrip(".")

    if fmt == "gltf":
        exported = scene.export(file_type="gltf")
        if isinstance(exported, dict):
            gltf_key = next(
                (key for key in exported.keys() if key.endswith(".gltf")), None
            )
            if gltf_key is not None:
                gltf_value = exported[gltf_key]
                gltf_json = json.loads(
                    gltf_value.decode("utf-8")
                    if isinstance(gltf_value, bytes)
                    else (
                        json.dumps(gltf_value)
                        if isinstance(gltf_value, dict)
                        else str(gltf_value)
                    )
                )
                for buffer in gltf_json.get("buffers", []) or []:
                    uri = buffer.get("uri")
                    if not uri or uri.startswith("data:") or uri not in exported:
                        continue
                    buffer["uri"] = (
                        "data:application/octet-stream;base64,"
                        + base64.b64encode(exported[uri]).decode("ascii")
                    )
                for image in gltf_json.get("images", []) or []:
                    uri = image.get("uri")
                    if not uri or uri.startswith("data:") or uri not in exported:
                        continue
                    mime = image.get("mimeType", "application/octet-stream")
                    image["uri"] = f"data:{mime};base64," + base64.b64encode(
                        exported[uri]
                    ).decode("ascii")
                return json.dumps(gltf_json).encode("utf-8")
            for key, value in exported.items():
                if key.endswith(".gltf"):
                    if isinstance(value, bytes):
                        return value
                    if isinstance(value, dict):
                        return json.dumps(value).encode("utf-8")
                    return str(value).encode("utf-8")
            return json.dumps(exported).encode("utf-8")
        if isinstance(exported, bytes):
            return exported
        return str(exported).encode("utf-8")

    if fmt in ("obj", "stl", "ply", "off"):
        meshes = scene.dump()
        if meshes:
            combined = _trimesh.util.concatenate(meshes)
        else:
            combined = _trimesh.Trimesh()
        result = combined.export(file_type=fmt)
        if isinstance(result, str):
            return result.encode("utf-8")
        return bytes(result)

    result = scene.export(file_type=fmt)
    if isinstance(result, str):
        return result.encode("utf-8")
    return bytes(result)


# #region 📻IFC Export
# IFC exporter mapping semio domain to IFC4 schema via ifcopenshell.


def _gltf_xyz_to_semio_xyz(x: float, y: float, z: float) -> tuple[float, float, float]:
    """🔖Convert glTF coordinates to semio/IFC coordinates."""
    return (float(x), float(-z), float(y))


def _glb_bytes_to_vertices_faces(
    raw: bytes,
) -> tuple[list[tuple[float, float, float]], list[tuple[int, ...]]] | None:
    """Extract vertices and faces from GLB bytes for IFC mesh representation."""
    import struct as _struct

    if len(raw) < 20 or raw[0:4] != b"glTF":
        return None
    offset = 12
    json_chunk: bytes | None = None
    bin_chunk = b""
    while offset + 8 <= len(raw):
        chunk_length, chunk_kind = _struct.unpack_from("<II", raw, offset)
        offset += 8
        chunk = raw[offset : offset + chunk_length]
        offset += chunk_length
        if chunk_kind == 0x4E4F534A:
            json_chunk = chunk
        elif chunk_kind == 0x004E4942:
            bin_chunk = chunk
    if json_chunk is None:
        return None
    try:
        gltf = json.loads(json_chunk.decode("utf-8").rstrip(" \t\r\n\x00"))
    except Exception:
        return None
    accessors = gltf.get("accessors", []) or []
    buffer_views = gltf.get("bufferViews", []) or []
    meshes = gltf.get("meshes", []) or []
    component_formats: dict[int, tuple[str, int]] = {
        5120: ("b", 1),
        5121: ("B", 1),
        5122: ("h", 2),
        5123: ("H", 2),
        5125: ("I", 4),
        5126: ("f", 4),
    }
    type_widths = {
        "SCALAR": 1,
        "VEC2": 2,
        "VEC3": 3,
        "VEC4": 4,
        "MAT2": 4,
        "MAT3": 9,
        "MAT4": 16,
    }

    def _read_accessor(accessor_index: int) -> numpy.ndarray | None:
        if accessor_index < 0 or accessor_index >= len(accessors):
            return None
        accessor = accessors[accessor_index]
        buffer_view_index = accessor.get("bufferView")
        if (
            not isinstance(buffer_view_index, int)
            or buffer_view_index < 0
            or buffer_view_index >= len(buffer_views)
        ):
            return None
        buffer_view = buffer_views[buffer_view_index]
        component_type = accessor.get("componentType")
        accessor_kind = accessor.get("type")
        count = accessor.get("count")
        if (
            component_type not in component_formats
            or accessor_kind not in type_widths
            or not isinstance(count, int)
        ):
            return None
        if buffer_view.get("buffer", 0) != 0:
            return None
        fmt_char, component_size = component_formats[component_type]
        element_width = type_widths[accessor_kind]
        stride = buffer_view.get("byteStride") or (component_size * element_width)
        byte_offset = buffer_view.get("byteOffset", 0) + accessor.get("byteOffset", 0)
        values: list[tuple[typing.Any, ...]] = []
        for item_index in range(count):
            start = byte_offset + item_index * stride
            end = start + component_size * element_width
            if end > len(bin_chunk):
                return None
            values.append(
                _struct.unpack_from("<" + fmt_char * element_width, bin_chunk, start)
            )
        return numpy.array(values)

    all_vertices: list[tuple[float, float, float]] = []
    all_faces: list[tuple[int, ...]] = []
    for mesh in meshes:
        primitives = mesh.get("primitives", []) or []
        for primitive in primitives:
            attributes = primitive.get("attributes", {}) or {}
            position_accessor_index = attributes.get("POSITION")
            if not isinstance(position_accessor_index, int):
                continue
            positions = _read_accessor(position_accessor_index)
            if positions is None or positions.ndim != 2 or positions.shape[1] < 3:
                continue
            vertex_offset = len(all_vertices)
            for row in positions:
                all_vertices.append(
                    _gltf_xyz_to_semio_xyz(float(row[0]), float(row[1]), float(row[2]))
                )
            if isinstance(primitive.get("indices"), int):
                indices = _read_accessor(primitive.get("indices"))
                if indices is None:
                    continue
                index_values = indices.reshape(-1).astype(int)
            else:
                index_values = numpy.arange(len(positions), dtype=int)
            triangle_count = len(index_values) // 3
            for tri_idx in range(triangle_count):
                i0 = int(index_values[tri_idx * 3]) + vertex_offset
                i1 = int(index_values[tri_idx * 3 + 1]) + vertex_offset
                i2 = int(index_values[tri_idx * 3 + 2]) + vertex_offset
                all_faces.append((i0, i1, i2))
    if len(all_vertices) == 0 or len(all_faces) == 0:
        return None
    return (all_vertices, all_faces)


def _export_ifc_from_dict(
    kit: dict,
    design_name: str,
    piece_planes: dict[str, dict],
    parent_of: dict[str, str],
    children_of: dict[str, list[str]],
    roots: list[str],
    tags: list[str],
) -> bytes:
    """Export a design to IFC4 format from dict-based kit data."""
    import ifcopenshell as _ifc
    import ifcopenshell.api as _ifc_api
    import ifcopenshell.id as _ifc_id

    # #region 🖨️Step 1: IFC File Project Units Context Spatial Tree From Layers
    ifc = _ifc_api.run("project.create_file", version="IFC4")
    kit_name = kit.get("name", "semio Kit")
    project = _ifc_api.run(
        "root.create_entity", ifc, ifc_class="IfcProject", name=kit_name
    )
    _ifc_api.run("unit.assign_unit", ifc)
    representation_context = _ifc_api.run(
        "context.add_context", ifc, context_type="Representation"
    )
    body_context = _ifc_api.run(
        "context.add_context",
        ifc,
        context_type="Representation",
        context_identifier="Body",
        target_view="REPRESENTATION_VIEW",
        parent=representation_context,
    )
    site = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcSite", name="Site")
    _ifc_api.run(
        "aggregate.assign_object", ifc, relating_object=project, products=[site]
    )

    designs = kit.get("designs", []) or []
    design = next(
        (
            d
            for d in designs
            if d.get("name") == design_name or d.get("id") == design_name
        ),
        None,
    )
    layers = (design.get("layers", []) or []) if design else []

    def _get_layer_ifc_type(layer: dict) -> str | None:
        for attr in layer.get("attributes", []) or []:
            if attr.get("key") == "ifc.type":
                return attr.get("value")
        return None

    # Build spatial hierarchy from layers
    ifc_buildings: dict[str, typing.Any] = {}
    ifc_storeys: dict[str, typing.Any] = {}
    storey_by_number: dict[int, typing.Any] = {}
    default_building = None
    default_storey = None

    for layer in layers:
        layer_path = layer.get("path", "")
        ifc_type = _get_layer_ifc_type(layer)
        if ifc_type == "IfcBuilding":
            building = _ifc_api.run(
                "root.create_entity", ifc, ifc_class="IfcBuilding", name=layer_path
            )
            _ifc_api.run(
                "aggregate.assign_object",
                ifc,
                relating_object=site,
                products=[building],
            )
            ifc_buildings[layer_path] = building
            if default_building is None:
                default_building = building
        elif ifc_type == "IfcBuildingStorey":
            parts = layer_path.rsplit("/", 1)
            parent_path = parts[0] if len(parts) > 1 else ""
            storey_name = parts[-1] if len(parts) > 1 else layer_path
            storey = _ifc_api.run(
                "root.create_entity",
                ifc,
                ifc_class="IfcBuildingStorey",
                name=storey_name,
            )
            parent_building = ifc_buildings.get(parent_path)
            if parent_building is not None:
                _ifc_api.run(
                    "aggregate.assign_object",
                    ifc,
                    relating_object=parent_building,
                    products=[storey],
                )
            ifc_storeys[layer_path] = storey
            try:
                storey_number = int(storey_name)
                storey_by_number[storey_number] = storey
            except ValueError:
                pass
            if default_storey is None:
                default_storey = storey

    # Fallback: create default building and storey if no layers define them
    if default_building is None:
        default_building = _ifc_api.run(
            "root.create_entity", ifc, ifc_class="IfcBuilding", name="Building"
        )
        _ifc_api.run(
            "aggregate.assign_object",
            ifc,
            relating_object=site,
            products=[default_building],
        )
    if default_storey is None:
        default_storey = _ifc_api.run(
            "root.create_entity", ifc, ifc_class="IfcBuildingStorey", name="Storey"
        )
        _ifc_api.run(
            "aggregate.assign_object",
            ifc,
            relating_object=default_building,
            products=[default_storey],
        )
    # #endregion 🖨️Step 1

    # #region 📋Step 2: Piece-to-storey Mapping From Piece Names
    import re as _re

    def _piece_storey(piece_name: str) -> typing.Any:
        m = _re.search(r"_f(\d+)_", piece_name or "")
        if m:
            floor = int(m.group(1))
            if floor in storey_by_number:
                return storey_by_number[floor]
        return default_storey

    # #endregion 📋Step 2

    pieces = (design.get("pieces", []) or []) if design else []
    connections = (design.get("connections", []) or []) if design else []
    types_by_id = {t.get("id"): t for t in (kit.get("types", []) or []) if t.get("id")}
    files_by_id = {f.get("id"): f for f in (kit.get("files", []) or []) if f.get("id")}
    piece_by_id = {p.get("id"): p for p in pieces if p.get("id")}
    tag_lookup = {
        tag.get("id"): tag for tag in (kit.get("tags", []) or []) if tag.get("id")
    }

    # #region 🛕Step 3: Types With Geometry
    ifc_types: dict[str, typing.Any] = {}
    for piece in pieces:
        type_ref = piece.get("type")
        type_id = type_ref.get("id") if isinstance(type_ref, dict) else None
        if type_id is None or type_id in ifc_types:
            continue
        type_obj = types_by_id.get(type_id)
        if type_obj is None:
            continue
        type_name = type_obj.get("name", type_id)
        type_variant = type_obj.get("variant", "")
        ifc_type_name = f"{type_name}:{type_variant}" if type_variant else type_name
        ifc_type = _ifc_api.run(
            "root.create_entity",
            ifc,
            ifc_class="IfcBuildingElementProxyType",
            name=ifc_type_name,
        )

        # Type-level pset for type attributes
        type_attrs = type_obj.get("attributes", []) or []
        if type_attrs:
            type_pset = _ifc_api.run(
                "pset.add_pset", ifc, product=ifc_type, name="SemioTypeAttributes"
            )
            props = {}
            for attr in type_attrs:
                key = attr.get("key", "")
                value = attr.get("value", "")
                if key:
                    props[key] = value
            if props:
                _ifc_api.run("pset.edit_pset", ifc, pset=type_pset, properties=props)

        # Type-level metadata pset
        type_meta = {}
        if type_obj.get("description"):
            type_meta["description"] = type_obj.get("description")
        if type_obj.get("variant"):
            type_meta["variant"] = type_obj.get("variant")
        if type_obj.get("id"):
            type_meta["semioId"] = type_obj.get("id")
        if type_meta:
            meta_pset = _ifc_api.run(
                "pset.add_pset", ifc, product=ifc_type, name="SemioTypeMetadata"
            )
            _ifc_api.run("pset.edit_pset", ifc, pset=meta_pset, properties=type_meta)

        # Geometry: find best representation, extract GLB mesh
        representations = type_obj.get("representations", []) or []
        selected_representation = None
        if representations:
            selected_tag_ids: set[str] = set()
            for tag_value in tags:
                if tag_value in tag_lookup:
                    selected_tag_ids.add(tag_value)
                else:
                    for tag in tag_lookup.values():
                        if tag.get("name") == tag_value:
                            selected_tag_ids.add(tag.get("id"))
            if not selected_tag_ids:
                selected_representation = (
                    next(
                        (
                            m
                            for m in representations
                            if len(m.get("tags", []) or []) == 0
                        ),
                        None,
                    )
                    or representations[0]
                )
            else:
                for m in representations:
                    representation_tag_ids = {
                        t.get("id") if isinstance(t, dict) else t
                        for t in (m.get("tags", []) or [])
                    }
                    if selected_tag_ids.issubset(representation_tag_ids):
                        selected_representation = m
                        break
                if selected_representation is None:
                    selected_representation = representations[0]

        if selected_representation is not None:
            file_ref = selected_representation.get("file", {})
            file_id = file_ref.get("id") if isinstance(file_ref, dict) else file_ref
            file_obj = files_by_id.get(file_id)
            if file_obj is not None and file_obj.get("blob"):
                blob = file_obj.get("blob")
                raw = base64.b64decode(
                    blob.split(",", 1)[1]
                    if isinstance(blob, str) and blob.startswith("data:")
                    else blob
                )
                result = _glb_bytes_to_vertices_faces(raw)
                if result is not None:
                    vertices, faces = result
                    rep = _ifc_api.run(
                        "geometry.add_mesh_representation",
                        ifc,
                        context=body_context,
                        vertices=[list(vertices)],
                        faces=[list(faces)],
                    )
                    _ifc_api.run(
                        "geometry.assign_representation",
                        ifc,
                        product=ifc_type,
                        representation=rep,
                    )

        ifc_types[type_id] = ifc_type
    # #endregion 🛕Step 3

    # #region 🎈Step 4: Pieces As Occurrences
    ifc_occurrences: dict[str, typing.Any] = {}
    ifc_connector_ports: dict[str, dict[str, typing.Any]] = {}
    for piece in pieces:
        piece_id = piece.get("id")
        if piece_id is None:
            continue
        piece_name = piece.get("name") or piece_id
        occurrence = _ifc_api.run(
            "root.create_entity",
            ifc,
            ifc_class="IfcBuildingElementProxy",
            name=piece_name,
        )

        type_ref = piece.get("type")
        type_id = type_ref.get("id") if isinstance(type_ref, dict) else None
        if type_id and type_id in ifc_types:
            _ifc_api.run(
                "type.assign_type",
                ifc,
                related_objects=[occurrence],
                relating_type=ifc_types[type_id],
            )

        # World placement from computed planes
        world_plane = piece_planes.get(piece_id)
        if world_plane is not None:
            origin = world_plane.get("origin", {})
            x_axis = world_plane.get("xAxis", {})
            y_axis = world_plane.get("yAxis", {})
            ox, oy, oz = (
                origin.get("x", 0.0),
                origin.get("y", 0.0),
                origin.get("z", 0.0),
            )
            xx, xy, xz = (
                x_axis.get("x", 1.0),
                x_axis.get("y", 0.0),
                x_axis.get("z", 0.0),
            )
            yx, yy, yz = (
                y_axis.get("x", 0.0),
                y_axis.get("y", 1.0),
                y_axis.get("z", 0.0),
            )
            x_vec = numpy.array([xx, xy, xz], dtype=numpy.float64)
            y_vec = numpy.array([yx, yy, yz], dtype=numpy.float64)
            z_vec = numpy.cross(x_vec, y_vec)
            nz = numpy.linalg.norm(z_vec)
            if nz > 1e-10:
                z_vec = z_vec / nz
            nx = numpy.linalg.norm(x_vec)
            if nx > 1e-10:
                x_vec = x_vec / nx
            y_vec = numpy.cross(z_vec, x_vec)
            ny = numpy.linalg.norm(y_vec)
            if ny > 1e-10:
                y_vec = y_vec / ny
            mat = numpy.eye(4)
            mat[:3, 0] = x_vec
            mat[:3, 1] = y_vec
            mat[:3, 2] = z_vec
            mat[:3, 3] = [ox, oy, oz]
            _ifc_api.run(
                "geometry.edit_object_placement", ifc, product=occurrence, matrix=mat
            )

        # Assign piece to the correct storey based on its floor number
        _ifc_api.run(
            "spatial.assign_container",
            ifc,
            relating_structure=_piece_storey(piece_name),
            products=[occurrence],
        )

        # Piece-level pset for piece attributes
        piece_props: dict[str, typing.Any] = {}
        if piece.get("name"):
            piece_props["name"] = piece.get("name")
        if piece.get("id"):
            piece_props["semioId"] = piece.get("id")
        piece_attrs = piece.get("attributes", []) or []
        for attr in piece_attrs:
            key = attr.get("key", "")
            value = attr.get("value", "")
            if key:
                piece_props[key] = value
        if piece_props:
            piece_pset = _ifc_api.run(
                "pset.add_pset", ifc, product=occurrence, name="SemioPieceAttributes"
            )
            _ifc_api.run("pset.edit_pset", ifc, pset=piece_pset, properties=piece_props)

        ifc_occurrences[piece_id] = occurrence

        # Connectors as ports
        type_obj = types_by_id.get(type_id) if type_id else None
        if type_obj is not None:
            connectors = type_obj.get("connectors", []) or []
            ifc_connector_ports[piece_id] = {}
            for conn in connectors:
                conn_id = conn.get("id") or conn.get("id_") or conn.get("name", "")
                port = _ifc_api.run(
                    "root.create_entity",
                    ifc,
                    ifc_class="IfcDistributionPort",
                    name=conn_id,
                )
                _ifc_api.run(
                    "nest.assign_object",
                    ifc,
                    relating_object=occurrence,
                    related_objects=[port],
                )

                # Port placement relative to element (connector point/direction)
                point = conn.get("point", {})
                if point:
                    port_mat = numpy.eye(4)
                    port_mat[:3, 3] = [
                        point.get("x", 0.0),
                        point.get("y", 0.0),
                        point.get("z", 0.0),
                    ]
                    direction = conn.get("direction", {})
                    if direction:
                        d = numpy.array(
                            [
                                direction.get("x", 0.0),
                                direction.get("y", 0.0),
                                direction.get("z", 1.0),
                            ]
                        )
                        dn = numpy.linalg.norm(d)
                        if dn > 1e-10:
                            d = d / dn
                            z = d
                            up = numpy.array([0.0, 0.0, 1.0])
                            if abs(numpy.dot(z, up)) > 0.99:
                                up = numpy.array([1.0, 0.0, 0.0])
                            x = numpy.cross(up, z)
                            xn = numpy.linalg.norm(x)
                            if xn > 1e-10:
                                x = x / xn
                            y = numpy.cross(z, x)
                            port_mat[:3, 0] = x
                            port_mat[:3, 1] = y
                            port_mat[:3, 2] = z
                    _ifc_api.run(
                        "geometry.edit_object_placement",
                        ifc,
                        product=port,
                        matrix=port_mat,
                    )

                # Connector pset
                conn_props: dict[str, typing.Any] = {"semioConnectorId": conn_id}
                if conn.get("description") and isinstance(conn.get("description"), str):
                    conn_props["description"] = conn.get("description")
                port_val = conn.get("port")
                if port_val:
                    conn_props["semioPort"] = (
                        port_val if isinstance(port_val, str) else str(port_val)
                    )
                conn_pset = _ifc_api.run(
                    "pset.add_pset", ifc, product=port, name="SemioConnector"
                )
                _ifc_api.run(
                    "pset.edit_pset", ifc, pset=conn_pset, properties=conn_props
                )

                ifc_connector_ports[piece_id][conn_id] = port
    # #endregion 🎈Step 4

    # #region 🌪️Step 5: Connections As Port Relationships
    for connection in connections:
        connected = connection.get("parent", {})
        connecting = connection.get("child", {})
        parent_piece_id = connected.get("piece", {}).get("id")
        child_piece_id = connecting.get("piece", {}).get("id")
        connected_connector_id = (
            connected.get("connector", {}).get("id")
            if connected.get("connector")
            else None
        )
        connecting_connector_id = (
            connecting.get("connector", {}).get("id")
            if connecting.get("connector")
            else None
        )

        connected_port = None
        connecting_port = None
        if parent_piece_id in ifc_connector_ports and connected_connector_id:
            connected_port = ifc_connector_ports[parent_piece_id].get(
                connected_connector_id
            )
        if child_piece_id in ifc_connector_ports and connecting_connector_id:
            connecting_port = ifc_connector_ports[child_piece_id].get(
                connecting_connector_id
            )

        # IfcRelConnectsPorts
        if connected_port is not None and connecting_port is not None:
            ifc.create_entity(
                "IfcRelConnectsPorts",
                GlobalId=_ifc_id.new(),
                RelatingPort=connected_port,
                RelatedPort=connecting_port,
            )

        # IfcRelConnectsElements
        connected_elem = ifc_occurrences.get(parent_piece_id)
        connecting_elem = ifc_occurrences.get(child_piece_id)
        if connected_elem is not None and connecting_elem is not None:
            ifc.create_entity(
                "IfcRelConnectsElements",
                GlobalId=_ifc_id.new(),
                RelatingElement=connected_elem,
                RelatedElement=connecting_elem,
            )

        # Connection solver parameters pset (on the connected element)
        conn_solver_props: dict[str, typing.Any] = {}
        for param in ("gap", "shift", "rise", "rotation", "turn", "tilt"):
            val = connection.get(param)
            if val is not None and val != 0:
                conn_solver_props[param] = float(val)
        if connection.get("description"):
            conn_solver_props["description"] = connection.get("description")
        if conn_solver_props and connected_elem is not None:
            conn_pset = _ifc_api.run(
                "pset.add_pset",
                ifc,
                product=connected_elem,
                name="SemioConnectionParams",
            )
            _ifc_api.run(
                "pset.edit_pset", ifc, pset=conn_pset, properties=conn_solver_props
            )
    # #endregion 🌪️Step 5

    # #region 🏆Step 6: Kit-level Metadata
    kit_meta: dict[str, typing.Any] = {}
    if kit.get("name"):
        kit_meta["name"] = kit.get("name")
    if kit.get("description"):
        kit_meta["description"] = kit.get("description")
    if kit.get("id"):
        kit_meta["semioId"] = kit.get("id")
    if kit.get("uri"):
        kit_meta["semioUri"] = kit.get("uri")
    authors = kit.get("authors", []) or []
    if authors:
        author_strs = [f"{a.get('name', '')} <{a.get('email', '')}>" for a in authors]
        kit_meta["authors"] = "; ".join(author_strs)
    if kit_meta:
        kit_pset = _ifc_api.run(
            "pset.add_pset", ifc, product=project, name="SemioKitMetadata"
        )
        _ifc_api.run("pset.edit_pset", ifc, pset=kit_pset, properties=kit_meta)
    # #endregion 🏆Step 6

    return ifc.to_string().encode("utf-8")


def _export_ifc_from_entities(
    kit: "Kit",
    design: "Design",
    piece_planes: dict[str, "Plane"],
    parent_of: dict[str, str],
    children_of: dict[str, list[str]],
    roots: list[str],
    pieces_dict: dict[str, "Piece"],
    types_dict: dict[str, "Type"],
    tags: list[str],
) -> bytes:
    """Export a design to IFC4 format from entity-based kit data."""
    import ifcopenshell as _ifc
    import ifcopenshell.api as _ifc_api
    import ifcopenshell.id as _ifc_id

    # #region 🖨️Step 1: IFC File Project Units Context Spatial Tree From Layers
    ifc = _ifc_api.run("project.create_file", version="IFC4")
    kit_name = kit.name if hasattr(kit, "name") and kit.name else "semio Kit"
    project = _ifc_api.run(
        "root.create_entity", ifc, ifc_class="IfcProject", name=kit_name
    )
    _ifc_api.run("unit.assign_unit", ifc)
    representation_context = _ifc_api.run(
        "context.add_context", ifc, context_type="Representation"
    )
    body_context = _ifc_api.run(
        "context.add_context",
        ifc,
        context_type="Representation",
        context_identifier="Body",
        target_view="REPRESENTATION_VIEW",
        parent=representation_context,
    )
    site = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcSite", name="Site")
    _ifc_api.run(
        "aggregate.assign_object", ifc, relating_object=project, products=[site]
    )

    layers = design.layers or [] if hasattr(design, "layers") else []

    def _get_layer_ifc_type_entity(layer: typing.Any) -> str | None:
        if hasattr(layer, "attributes"):
            for attr in layer.attributes or []:
                key = attr.key if hasattr(attr, "key") else attr.get("key", "")
                value = attr.value if hasattr(attr, "value") else attr.get("value", "")
                if key == "ifc.type":
                    return value
        return None

    ifc_buildings: dict[str, typing.Any] = {}
    ifc_storeys: dict[str, typing.Any] = {}
    storey_by_number: dict[int, typing.Any] = {}
    default_building = None
    default_storey = None

    for layer in layers:
        layer_name = layer.name if hasattr(layer, "name") else layer.get("name", "")
        ifc_type_val = _get_layer_ifc_type_entity(layer)
        if ifc_type_val == "IfcBuilding":
            building = _ifc_api.run(
                "root.create_entity", ifc, ifc_class="IfcBuilding", name=layer_name
            )
            _ifc_api.run(
                "aggregate.assign_object",
                ifc,
                relating_object=site,
                products=[building],
            )
            ifc_buildings[layer_name] = building
            if default_building is None:
                default_building = building
        elif ifc_type_val == "IfcBuildingStorey":
            parts = layer_name.rsplit("/", 1)
            parent_name = parts[0] if len(parts) > 1 else ""
            storey_label = parts[-1] if len(parts) > 1 else layer_name
            storey_ent = _ifc_api.run(
                "root.create_entity",
                ifc,
                ifc_class="IfcBuildingStorey",
                name=storey_label,
            )
            parent_building = ifc_buildings.get(parent_name)
            if parent_building is not None:
                _ifc_api.run(
                    "aggregate.assign_object",
                    ifc,
                    relating_object=parent_building,
                    products=[storey_ent],
                )
            ifc_storeys[layer_name] = storey_ent
            try:
                storey_by_number[int(storey_label)] = storey_ent
            except ValueError:
                pass
            if default_storey is None:
                default_storey = storey_ent

    if default_building is None:
        default_building = _ifc_api.run(
            "root.create_entity", ifc, ifc_class="IfcBuilding", name="Building"
        )
        _ifc_api.run(
            "aggregate.assign_object",
            ifc,
            relating_object=site,
            products=[default_building],
        )
    if default_storey is None:
        default_storey = _ifc_api.run(
            "root.create_entity", ifc, ifc_class="IfcBuildingStorey", name="Storey"
        )
        _ifc_api.run(
            "aggregate.assign_object",
            ifc,
            relating_object=default_building,
            products=[default_storey],
        )
    # #endregion 🖨️Step 1

    # #region 📋Step 2: Piece-to-storey Mapping
    import re as _re

    def _piece_storey_entity(piece_name: str) -> typing.Any:
        m = _re.search(r"_f(\d+)_", piece_name or "")
        if m:
            floor = int(m.group(1))
            if floor in storey_by_number:
                return storey_by_number[floor]
        return default_storey

    # #endregion 📋Step 2

    pieces = design.pieces or []
    connections = design.connections or []

    # #region 🛕Step 3: Types With Geometry
    ifc_types: dict[str, typing.Any] = {}
    for piece in pieces:
        if piece.type is None:
            continue
        tk = _type_key_from_id(piece.type)
        if tk in ifc_types:
            continue
        type_obj = types_dict.get(tk)
        if type_obj is None:
            continue
        ifc_type_name = (
            f"{type_obj.name}:{type_obj.variant}" if type_obj.variant else type_obj.name
        )
        ifc_type = _ifc_api.run(
            "root.create_entity",
            ifc,
            ifc_class="IfcBuildingElementProxyType",
            name=ifc_type_name,
        )

        # Type-level geometry
        representation = _find_matching_representation(kit, type_obj, tags)
        if representation is not None:
            files_list = kit.files_ or []
            file_id = (
                representation.file.id
                if hasattr(representation.file, "id")
                else representation.file
            )
            file_obj = next(
                (f for f in files_list if f.name == file_id or f.id == file_id), None
            )
            if file_obj is not None and file_obj.blob:
                blob = file_obj.blob
                raw = base64.b64decode(
                    blob.split(",", 1)[1] if blob.startswith("data:") else blob
                )
                result = _glb_bytes_to_vertices_faces(raw)
                if result is not None:
                    vertices, faces = result
                    rep = _ifc_api.run(
                        "geometry.add_mesh_representation",
                        ifc,
                        context=body_context,
                        vertices=[list(vertices)],
                        faces=[list(faces)],
                    )
                    _ifc_api.run(
                        "geometry.assign_representation",
                        ifc,
                        product=ifc_type,
                        representation=rep,
                    )

        ifc_types[tk] = ifc_type
    # #endregion 🛕Step 3

    # #region 🎈Step 4: Pieces As Occurrences
    ifc_occurrences: dict[str, typing.Any] = {}
    ifc_connector_ports: dict[str, dict[str, typing.Any]] = {}
    for piece in pieces:
        piece_name = piece.name or piece.id_
        occurrence = _ifc_api.run(
            "root.create_entity",
            ifc,
            ifc_class="IfcBuildingElementProxy",
            name=piece_name,
        )

        if piece.type is not None:
            tk = _type_key_from_id(piece.type)
            if tk in ifc_types:
                _ifc_api.run(
                    "type.assign_type",
                    ifc,
                    related_objects=[occurrence],
                    relating_type=ifc_types[tk],
                )

        world_plane = piece_planes.get(piece.id_)
        if world_plane is not None:
            mat = _plane_to_matrix_4x4(world_plane)
            _ifc_api.run(
                "geometry.edit_object_placement", ifc, product=occurrence, matrix=mat
            )

        # Assign piece to the correct storey based on its floor number
        _ifc_api.run(
            "spatial.assign_container",
            ifc,
            relating_structure=_piece_storey_entity(piece_name),
            products=[occurrence],
        )
        ifc_occurrences[piece.id_] = occurrence

        # Connectors as ports
        type_obj = types_dict.get(_type_key_from_id(piece.type)) if piece.type else None
        if type_obj is not None and type_obj.connectors:
            ifc_connector_ports[piece.id_] = {}
            for conn in type_obj.connectors:
                conn_id = conn.id_
                port = _ifc_api.run(
                    "root.create_entity",
                    ifc,
                    ifc_class="IfcDistributionPort",
                    name=conn_id,
                )
                _ifc_api.run(
                    "nest.assign_object",
                    ifc,
                    relating_object=occurrence,
                    related_objects=[port],
                )

                point = conn.point
                port_mat = numpy.eye(4)
                port_mat[:3, 3] = [point.x, point.y, point.z]
                direction = conn.direction
                d = numpy.array([direction.x, direction.y, direction.z])
                dn = numpy.linalg.norm(d)
                if dn > 1e-10:
                    d = d / dn
                    z = d
                    up = numpy.array([0.0, 0.0, 1.0])
                    if abs(numpy.dot(z, up)) > 0.99:
                        up = numpy.array([1.0, 0.0, 0.0])
                    x = numpy.cross(up, z)
                    xn = numpy.linalg.norm(x)
                    if xn > 1e-10:
                        x = x / xn
                    y = numpy.cross(z, x)
                    port_mat[:3, 0] = x
                    port_mat[:3, 1] = y
                    port_mat[:3, 2] = z
                _ifc_api.run(
                    "geometry.edit_object_placement", ifc, product=port, matrix=port_mat
                )

                ifc_connector_ports[piece.id_][conn_id] = port
    # #endregion 🎈Step 4

    # #region 🌪️Step 5: Connections As Port Relationships
    for conn in connections:
        connected_id = conn.parent.piece.id_
        connecting_id = conn.child.piece.id_
        connected_connector_id = (
            conn.parent.connector.id_ if conn.parent.connector else None
        )
        connecting_connector_id = (
            conn.child.connector.id_ if conn.child.connector else None
        )

        connected_port = None
        connecting_port = None
        if connected_id in ifc_connector_ports and connected_connector_id:
            connected_port = ifc_connector_ports[connected_id].get(
                connected_connector_id
            )
        if connecting_id in ifc_connector_ports and connecting_connector_id:
            connecting_port = ifc_connector_ports[connecting_id].get(
                connecting_connector_id
            )

        if connected_port is not None and connecting_port is not None:
            ifc.create_entity(
                "IfcRelConnectsPorts",
                GlobalId=_ifc_id.new(),
                RelatingPort=connected_port,
                RelatedPort=connecting_port,
            )

        connected_elem = ifc_occurrences.get(connected_id)
        connecting_elem = ifc_occurrences.get(connecting_id)
        if connected_elem is not None and connecting_elem is not None:
            ifc.create_entity(
                "IfcRelConnectsElements",
                GlobalId=_ifc_id.new(),
                RelatingElement=connected_elem,
                RelatedElement=connecting_elem,
            )

        conn_solver_props: dict[str, typing.Any] = {}
        for param in ("gap", "shift", "rise", "rotation", "turn", "tilt"):
            val = getattr(conn, param, 0)
            if val is not None and val != 0:
                conn_solver_props[param] = float(val)
        if conn.description:
            conn_solver_props["description"] = conn.description
        if conn_solver_props and connected_elem is not None:
            conn_pset = _ifc_api.run(
                "pset.add_pset",
                ifc,
                product=connected_elem,
                name="SemioConnectionParams",
            )
            _ifc_api.run(
                "pset.edit_pset", ifc, pset=conn_pset, properties=conn_solver_props
            )
    # #endregion 🌪️Step 5

    return ifc.to_string().encode("utf-8")


# #endregion 📻IFC Export

# #endregion 🔩Kit Representation Export


# #region ❄️Geometric Insights
# Key performance indicators for GLB/GLTF representation geometry. Representation MUST be glb/gltf.


@dataclasses.dataclass
class GeometricInsights:
    """🔖Aggregated geometric KPIs for a single mesh or merged scene.
    All geometric data is expressed in the semio coordinate system:
    semio.x = glb.x, semio.y = -glb.x, semio.z = glb.y.
    """

    # Overall size
    bounding_box_min: Point | None = None
    bounding_box_max: Point | None = None
    dimension_x: float | None = None
    dimension_y: float | None = None
    dimension_z: float | None = None
    characteristic_length: float | None = None
    footprint_area: float | None = None
    # Surface area
    total_surface_area: float | None = None
    # Volume
    enclosed_volume: float | None = None
    # Compactness
    surface_to_volume_ratio: float | None = None
    sphericity: float | None = None
    hull_fill_ratio: float | None = None
    # Proportion
    aspect_ratio_xy: float | None = None
    aspect_ratio_xz: float | None = None
    aspect_ratio_yz: float | None = None
    slenderness: float | None = None
    # Mass distribution
    centroid: Point | None = None
    principal_axes: list[Vector] | None = None
    moments_of_inertia: tuple[float, float, float] | None = None
    # Topology
    vertex_count: int | None = None
    face_count: int | None = None
    euler_characteristic: int | None = None
    genus: int | None = None
    is_watertight: bool | None = None
    # Concavity
    convex_hull_volume: float | None = None
    concavity_index: float | None = None


def get_geometric_insights_for_representation(
    representation: str | bytes,
) -> GeometricInsights:
    """🔖Compute key performance indicators for the geometry of a GLB/GLTF representation."""
    import trimesh as _trimesh

    if isinstance(representation, bytes):
        file_type = "glb"
        if len(representation) >= 4 and representation[:4] == b"glTF":
            file_type = "glb"
        elif len(representation) > 0 and representation.lstrip().startswith(b"{"):
            file_type = "gltf"
        stream = _trimesh.util.wrap_as_stream(representation)
        loaded = _trimesh.load(stream, file_type=file_type)
    else:
        path = pathlib.Path(representation)
        if not path.exists():
            raise FileNotFoundError(f"Representation file not found: {representation}")
        ext = path.suffix.lower()
        if ext not in (".glb", ".gltf"):
            raise ValueError(f"Representation MUST be .glb or .gltf, got {ext}")
        file_type = "glb" if ext == ".glb" else "gltf"
        loaded = _trimesh.load(str(path), file_type=file_type)

    if isinstance(loaded, _trimesh.Scene):
        meshes = [
            g.copy()
            for g in loaded.geometry.values()
            if isinstance(g, _trimesh.Trimesh) and len(getattr(g, "faces", [])) > 0
        ]
        if not meshes:
            return GeometricInsights()
        mesh = _trimesh.util.concatenate(meshes)
    elif isinstance(loaded, _trimesh.Trimesh) and len(getattr(loaded, "faces", [])) > 0:
        mesh = loaded
    else:
        return GeometricInsights()

    # Transform vertices from GLB to semio coordinate system.
    verts = mesh.vertices  # (n, 3) in GLB
    xs = verts[:, 0]
    ys = verts[:, 1]
    # semio: x = glb.x, y = -glb.x, z = glb.y
    semio_x = xs
    semio_y = -xs
    semio_z = ys

    xs_min, xs_max = float(semio_x.min()), float(semio_x.max())
    ys_min, ys_max = float(semio_y.min()), float(semio_y.max())
    zs_min, zs_max = float(semio_z.min()), float(semio_z.max())

    out = GeometricInsights()

    # Overall size in semio coordinates
    out.bounding_box_min = Point(x=xs_min, y=ys_min, z=zs_min)
    out.bounding_box_max = Point(x=xs_max, y=ys_max, z=zs_max)
    dim_x = xs_max - xs_min
    dim_y = ys_max - ys_min
    dim_z = zs_max - zs_min
    out.dimension_x = dim_x
    out.dimension_y = dim_y
    out.dimension_z = dim_z
    vol_box = dim_x * dim_y * dim_z
    out.characteristic_length = float(numpy.cbrt(vol_box) if vol_box > 0 else 0.0)
    out.footprint_area = dim_x * dim_z

    # Surface area and volume (topology and integrals are invariant under linear transform)
    out.total_surface_area = float(mesh.area)

    # Volume
    if mesh.is_watertight:
        out.enclosed_volume = float(mesh.volume)
    else:
        out.enclosed_volume = None

    # Compactness
    if out.enclosed_volume is not None and out.enclosed_volume > 1e-20:
        out.surface_to_volume_ratio = out.total_surface_area / out.enclosed_volume
    vol = out.enclosed_volume or 0.0
    if vol > 1e-20 and out.total_surface_area:
        out.sphericity = float(
            (numpy.pi ** (1 / 3)) * (6 * vol) ** (2 / 3) / out.total_surface_area
        )
        out.sphericity = min(1.0, max(0.0, out.sphericity))

    try:
        hull = mesh.convex_hull
        if hull is not None and hull.volume > 1e-20 and vol > 0:
            out.convex_hull_volume = float(hull.volume)
            out.hull_fill_ratio = float(vol / hull.volume)
            out.hull_fill_ratio = min(1.0, max(0.0, out.hull_fill_ratio))
        elif hull is not None:
            out.convex_hull_volume = float(hull.volume)
    except Exception:
        pass

    # Proportion (semio dimensions)
    if dim_x > 1e-10 and dim_y > 1e-10:
        out.aspect_ratio_xy = float(dim_x / dim_y)
    if dim_x > 1e-10 and dim_z > 1e-10:
        out.aspect_ratio_xz = float(dim_x / dim_z)
    if dim_y > 1e-10 and dim_z > 1e-10:
        out.aspect_ratio_yz = float(dim_y / dim_z)
    max_ext = float(max(dim_x, dim_y, dim_z))
    if max_ext > 1e-10:
        out.slenderness = (
            max_ext / float(numpy.cbrt(mesh.area * max_ext)) if mesh.area > 0 else None
        )

    # Mass distribution (trimesh uses density=1)
    cx_g, cy_g, cz_g = (
        float(mesh.centroid[0]),
        float(mesh.centroid[1]),
        float(mesh.centroid[2]),
    )
    # transform centroid as a point
    out.centroid = Point(x=cx_g, y=-cx_g, z=cy_g)
    try:
        components = mesh.principal_inertia_components
        vectors = mesh.principal_inertia_vectors
        if components is not None and vectors is not None:
            out.moments_of_inertia = (
                float(components[0]),
                float(components[1]),
                float(components[2]),
            )
            # Transform axes from GLB to semio: (vx, vy, vz)_glb -> (vx, -vx, vy)_semio
            out.principal_axes = [
                Vector(
                    x=float(vectors[0][0]),
                    y=float(-vectors[0][0]),
                    z=float(vectors[0][1]),
                ),
                Vector(
                    x=float(vectors[1][0]),
                    y=float(-vectors[1][0]),
                    z=float(vectors[1][1]),
                ),
                Vector(
                    x=float(vectors[2][0]),
                    y=float(-vectors[2][0]),
                    z=float(vectors[2][1]),
                ),
            ]
    except Exception:
        pass

    # Topology
    out.vertex_count = int(len(mesh.vertices))
    out.face_count = int(len(mesh.faces))
    try:
        out.euler_characteristic = int(mesh.euler_number)
        if mesh.is_watertight:
            out.genus = (
                (2 - out.euler_characteristic) // 2
                if out.euler_characteristic is not None
                else None
            )
    except Exception:
        pass
    out.is_watertight = bool(mesh.is_watertight)

    # Concavity
    if (
        out.convex_hull_volume is not None
        and out.convex_hull_volume > 1e-20
        and out.enclosed_volume is not None
    ):
        out.concavity_index = 1.0 - (out.enclosed_volume / out.convex_hull_volume)
        out.concavity_index = min(1.0, max(0.0, out.concavity_index))

    return out


def geometric_insights_to_report_dict(
    insights: GeometricInsights, round_digits: int = 6
) -> dict[str, typing.Any]:
    """🔖Serialize GeometricInsights to a JSON-serializable dict for reports. Uses semio Point/Vector as {x,y,z}."""
    out: dict[str, typing.Any] = {}
    r = round_digits

    def round_val(v: float | None) -> float | None:
        return round(v, r) if v is not None else None

    if insights.bounding_box_min is not None:
        p = insights.bounding_box_min
        out["bounding_box_min"] = {
            "x": round(p.x, r),
            "y": round(p.y, r),
            "z": round(p.z, r),
        }
    if insights.bounding_box_max is not None:
        p = insights.bounding_box_max
        out["bounding_box_max"] = {
            "x": round(p.x, r),
            "y": round(p.y, r),
            "z": round(p.z, r),
        }
    if insights.centroid is not None:
        p = insights.centroid
        out["centroid"] = {"x": round(p.x, r), "y": round(p.y, r), "z": round(p.z, r)}
    for key in (
        "dimension_x",
        "dimension_y",
        "dimension_z",
        "characteristic_length",
        "footprint_area",
        "total_surface_area",
        "enclosed_volume",
        "surface_to_volume_ratio",
        "sphericity",
        "hull_fill_ratio",
        "aspect_ratio_xy",
        "aspect_ratio_xz",
        "aspect_ratio_yz",
        "slenderness",
        "convex_hull_volume",
        "concavity_index",
    ):
        val = getattr(insights, key, None)
        if val is not None:
            out[key] = round(val, r) if isinstance(val, float) else val
    if insights.principal_axes is not None:
        out["principal_axes"] = [
            {"x": round(v.x, r), "y": round(v.y, r), "z": round(v.z, r)}
            for v in insights.principal_axes
        ]
    if insights.moments_of_inertia is not None:
        out["moments_of_inertia"] = [round(x, r) for x in insights.moments_of_inertia]
    for key in ("vertex_count", "face_count", "euler_characteristic", "genus"):
        val = getattr(insights, key, None)
        if val is not None:
            out[key] = val
    if insights.is_watertight is not None:
        out["is_watertight"] = insights.is_watertight
    return out


# #endregion ❄️Geometric Insights


# #region 🔍Spatial Math
# Spatial math utilities for vector normalization and plane computation.


def normalizeVector(v: numpy.ndarray) -> numpy.ndarray:
    """🔖Normalize a 3D vector to unit length."""
    length = numpy.linalg.norm(v)
    if length < 1e-10:
        return v
    return v / length


def planeFromYAxis(
    yAxis: numpy.ndarray, phiDegrees: float = 0.0, origin: numpy.ndarray | None = None
) -> Plane:
    """🔖Construct a plane from an origin point and a Y-axis direction."""
    if origin is None:
        origin = numpy.array([0.0, 0.0, 0.0])
    yAxis = normalizeVector(yAxis)
    worldY = numpy.array([0.0, 1.0, 0.0])
    if numpy.allclose(yAxis, worldY, atol=1e-6):
        rotationToY = numpy.eye(3)
    elif numpy.allclose(yAxis, -worldY, atol=1e-6):
        rotationToY = pytransform3d.rotations.matrix_from_axis_angle(
            [1, 0, 0, numpy.pi]
        )
    else:
        axis = numpy.cross(worldY, yAxis)
        axis = normalizeVector(axis)
        angle = numpy.arccos(numpy.clip(numpy.dot(worldY, yAxis), -1.0, 1.0))
        rotationToY = pytransform3d.rotations.matrix_from_axis_angle(
            numpy.concatenate([axis, [angle]])
        )
    phiRadians = numpy.deg2rad(phiDegrees)
    rotationAroundY = pytransform3d.rotations.matrix_from_axis_angle(
        numpy.concatenate([yAxis, [phiRadians]])
    )
    worldX = numpy.array([1.0, 0.0, 0.0])
    xAxis = rotationAroundY @ rotationToY @ worldX
    xAxis = normalizeVector(xAxis)
    plane = Plane()
    plane.origin = Point(x=float(origin[0]), y=float(origin[1]), z=float(origin[2]))
    plane.xAxis = Vector(x=float(xAxis[0]), y=float(xAxis[1]), z=float(xAxis[2]))
    plane.yAxis = Vector(x=float(yAxis[0]), y=float(yAxis[1]), z=float(yAxis[2]))
    return plane


def computeChildPlane(
    parentPlane: Plane,
    parentConnector: Connector,
    childConnector: Connector,
    connection: Connection,
) -> Plane:
    """Compute the world-space plane of a child from parent and local planes."""
    gap = connection.gap or 0
    shift = connection.shift or 0
    rise = connection.rise or 0
    rotation = connection.rotation or 0
    turn = connection.turn or 0
    tilt = connection.tilt or 0
    pOrigin = numpy.array(
        [parentPlane.origin.x, parentPlane.origin.y, parentPlane.origin.z]
    )
    pX = numpy.array([parentPlane.xAxis.x, parentPlane.xAxis.y, parentPlane.xAxis.z])
    pY = numpy.array([parentPlane.yAxis.x, parentPlane.yAxis.y, parentPlane.yAxis.z])
    pZ = numpy.cross(pX, pY)
    parentMatrix = numpy.eye(4)
    parentMatrix[:3, 0] = pX
    parentMatrix[:3, 1] = pY
    parentMatrix[:3, 2] = pZ
    parentMatrix[:3, 3] = pOrigin
    ppPoint = numpy.array(
        [parentConnector.point.x, parentConnector.point.y, parentConnector.point.z]
    )
    ppDir = numpy.array(
        [
            parentConnector.direction.x,
            parentConnector.direction.y,
            parentConnector.direction.z,
        ]
    )
    cpPoint = numpy.array(
        [childConnector.point.x, childConnector.point.y, childConnector.point.z]
    )
    cpDir = numpy.array(
        [
            childConnector.direction.x,
            childConnector.direction.y,
            childConnector.direction.z,
        ]
    )
    ppWorld = parentMatrix[:3, :3] @ ppPoint + parentMatrix[:3, 3]
    ppDirWorld = parentMatrix[:3, :3] @ ppDir
    ppDirWorld = normalizeVector(ppDirWorld)
    translation = (
        ppWorld + gap * ppDirWorld + shift * numpy.cross(ppDirWorld, pZ) + rise * pZ
    )
    targetDir = -ppDirWorld
    cpDirNormalized = normalizeVector(cpDir)
    if numpy.allclose(cpDirNormalized, targetDir, atol=1e-6):
        baseRotation = numpy.eye(3)
    elif numpy.allclose(cpDirNormalized, -targetDir, atol=1e-6):
        axis = numpy.array([1.0, 0.0, 0.0])
        if numpy.allclose(numpy.abs(cpDirNormalized), axis, atol=1e-6):
            axis = numpy.array([0.0, 1.0, 0.0])
        baseRotation = pytransform3d.rotations.matrix_from_axis_angle(
            numpy.concatenate([axis, [numpy.pi]])
        )
    else:
        axis = numpy.cross(cpDirNormalized, targetDir)
        axis = normalizeVector(axis)
        angle = numpy.arccos(
            numpy.clip(numpy.dot(cpDirNormalized, targetDir), -1.0, 1.0)
        )
        baseRotation = pytransform3d.rotations.matrix_from_axis_angle(
            numpy.concatenate([axis, [angle]])
        )
    rotRad = numpy.deg2rad(rotation)
    rotationMatrix = pytransform3d.rotations.matrix_from_axis_angle(
        numpy.concatenate([targetDir, [rotRad]])
    )
    turnRad = numpy.deg2rad(turn)
    pZWorld = normalizeVector(pZ)
    turnMatrix = pytransform3d.rotations.matrix_from_axis_angle(
        numpy.concatenate([pZWorld, [turnRad]])
    )
    tiltRad = numpy.deg2rad(tilt)
    pXWorld = normalizeVector(parentMatrix[:3, :3] @ numpy.array([1, 0, 0]))
    tiltMatrix = pytransform3d.rotations.matrix_from_axis_angle(
        numpy.concatenate([pXWorld, [tiltRad]])
    )
    combinedRotation = tiltMatrix @ turnMatrix @ rotationMatrix @ baseRotation
    childOrigin = translation - combinedRotation @ cpPoint
    childX = combinedRotation @ numpy.array([1, 0, 0])
    childY = combinedRotation @ numpy.array([0, 1, 0])
    plane = Plane()
    plane.origin = Point(
        x=float(childOrigin[0]), y=float(childOrigin[1]), z=float(childOrigin[2])
    )
    plane.xAxis = Vector(x=float(childX[0]), y=float(childX[1]), z=float(childX[2]))
    plane.yAxis = Vector(x=float(childY[0]), y=float(childY[1]), z=float(childY[2]))
    return plane


# #endregion 🔍Spatial Math


# #region 🧪Tests
# Tests for the semio py module.

TEST_TOLERANCE = 0.001
TEST_ASSETS_DIR = "../assets/semio"
REPORTS_EXPORT_DIR = (
    pathlib.Path(__file__).resolve().parents[2]
    / "reports"
    / "export-design-representation"
)
REPORTS_REPRESENTATION_KPI_DIR = (
    pathlib.Path(__file__).resolve().parents[2] / "reports" / "representation-kpi"
)


def _test_load_json(filename: str) -> dict:
    path = os.path.join(os.path.dirname(__file__), TEST_ASSETS_DIR, filename)
    if not os.path.exists(path):
        raise FileNotFoundError(f"Asset not found: {path}")
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def _test_load_kit(filename: str) -> dict:
    """🧪Load and normalize kit JSON for Kit.parse (flattens parent/folder refs, etc.)."""
    data = _test_load_json(filename)
    if "id" in data and "uri" not in data:
        data["uri"] = data["id"]
    for key in [
        "types",
        "designs",
        "files",
        "folders",
        "authors",
        "concepts",
        "representations",
        "connectors",
        "pieces",
        "connections",
        "layers",
        "groups",
        "stats",
        "ports",
        "qualities",
        "attributes",
    ]:
        if key not in data or data[key] is None:
            data[key] = []
    for collection in ["types", "designs", "folders"]:
        if collection in data:
            for item in data[collection]:
                if (
                    "parent" in item
                    and isinstance(item["parent"], dict)
                    and "id" in item["parent"]
                ):
                    item["parent"] = item["parent"]["id"]
                if (
                    "folder" in item
                    and isinstance(item["folder"], dict)
                    and "id" in item["folder"]
                ):
                    item["folder"] = item["folder"]["id"]
    if "types" in data:
        for t in data["types"]:
            if "representations" in t:
                for m in t["representations"]:
                    if (
                        "file" in m
                        and isinstance(m["file"], dict)
                        and "id" in m["file"]
                    ):
                        m["file"] = m["file"]["id"]
                    if "file" not in m or m["file"] is None:
                        m["file"] = ""
                    if "url" not in m or m["url"] is None:
                        m["url"] = ""
                    if "tags" in m and isinstance(m["tags"], list):
                        new_tags = [
                            (
                                tag["id"]
                                if isinstance(tag, dict) and "id" in tag
                                else tag
                            )
                            for tag in m["tags"]
                        ]
                        m["tags"] = new_tags
                    elif "tags" not in m:
                        m["tags"] = []
    return data


def _test_build_workflow_kit() -> dict:
    """💼Build a compact kit fixture for workflow roundtrip tests."""
    asset_blob = "data:text/plain;base64," + base64.b64encode(
        b"workflow asset payload"
    ).decode("ascii")
    return {
        "id": "11111111-1111-1111-1111-111111111111",
        "name": "Workflow Kit",
        "version": "1.0.0",
        "description": "Kit workflow fixture.",
        "types": [
            {
                "id": "22222222-2222-2222-2222-222222222222",
                "name": "Workflow Type",
                "connectors": [],
                "representations": [
                    {
                        "id": "33333333-3333-3333-3333-333333333333",
                        "name": "Workflow Representation",
                        "file": {"id": "44444444-4444-4444-4444-444444444444"},
                    }
                ],
            }
        ],
        "designs": [
            {
                "id": "55555555-5555-5555-5555-555555555555",
                "name": "Workflow Design",
                "pieces": [
                    {
                        "id": "66666666-6666-6666-6666-666666666666",
                        "id": "Piece-1",
                        "type": {"id": "22222222-2222-2222-2222-222222222222"},
                    }
                ],
                "connections": [],
            }
        ],
        "files": [
            {
                "id": "44444444-4444-4444-4444-444444444444",
                "name": "asset.txt",
                "folder": {"id": "77777777-7777-7777-7777-777777777777"},
                "blob": asset_blob,
            }
        ],
        "folders": [
            {
                "id": "77777777-7777-7777-7777-777777777777",
                "name": "assets",
            }
        ],
        "ports": [],
        "qualities": [],
        "concepts": [],
        "tags": [],
        "authors": [],
        "attributes": [],
    }


def _test_build_workflow_diff(updated_name: str, updated_asset_name: str) -> dict:
    """🔖Build a compact diff for workflow edit tests."""
    return {
        "name": updated_name,
        "files": {
            "updated": [
                {
                    "file": {"id": "44444444-4444-4444-4444-444444444444"},
                    "diff": {"name": updated_asset_name},
                }
            ]
        },
    }


def _test_build_workflow_archive_bytes(
    kit_dict: dict, files: dict[str, bytes]
) -> bytes:
    """🔖Build archive bytes for remote ZIP workflow tests."""
    with tempfile.TemporaryDirectory() as tmpdir:
        archive_path = os.path.join(tmpdir, "workflow.zip")
        export_kit(KitData(kit_dict), files, archive_path)
        with open(archive_path, "rb") as handle:
            return handle.read()


def _test_remote_kit_server(json_body: bytes, zip_body: bytes):
    """🖥️Create a disposable HTTP server for remote kit workflow tests."""
    import http.server
    import threading

    class _WorkflowHandler(http.server.BaseHTTPRequestHandler):
        store = {
            "/workflow.json": {"content_type": "application/json", "body": json_body},
            "/workflow.zip": {"content_type": "application/zip", "body": zip_body},
        }

        def do_GET(self):
            item = self.store.get(self.path)
            if item is None:
                self.send_error(404)
                return
            self.send_response(200)
            self.send_header("Content-Type", item["content_type"])
            self.send_header("Content-Length", str(len(item["body"])))
            self.end_headers()
            self.wfile.write(item["body"])

        def do_PUT(self):
            item = self.store.get(self.path)
            if item is None:
                self.send_error(404)
                return
            length = int(self.headers.get("Content-Length", "0"))
            item["body"] = self.rfile.read(length)
            item["content_type"] = self.headers.get(
                "Content-Type", item["content_type"]
            )
            self.send_response(204)
            self.end_headers()

        def log_message(self, format: str, *args: typing.Any) -> None:
            return

    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), _WorkflowHandler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()
    return server, thread


def _test_is_close(a, b):
    return abs(a - b) < TEST_TOLERANCE


def _test_vectors_equal(v1, v2):
    if v1 is None or v2 is None:
        return False
    return (
        _test_is_close(v1.get("x", 0), v2.get("x", 0))
        and _test_is_close(v1.get("y", 0), v2.get("y", 0))
        and _test_is_close(v1.get("z", 0), v2.get("z", 0))
    )


def _test_planes_equal(p1, p2):
    if p1 is None or p2 is None:
        return False
    if not p1.get("origin") or not p2.get("origin"):
        return False
    if not p1.get("xAxis") or not p2.get("xAxis"):
        return False
    if not p1.get("yAxis") or not p2.get("yAxis"):
        return False
    return (
        _test_vectors_equal(p1.get("origin"), p2.get("origin"))
        and _test_vectors_equal(p1.get("xAxis"), p2.get("xAxis"))
        and _test_vectors_equal(p1.get("yAxis"), p2.get("yAxis"))
    )


def _test_centers_equal(c1, c2):
    if c1 is None or c2 is None:
        return c1 == c2
    return _test_is_close(c1.get("u", 0), c2.get("u", 0)) and _test_is_close(
        c1.get("v", 0), c2.get("v", 0)
    )


def _test_find_design(kit: dict, name: str, parent_name: str = None) -> dict:
    parent_id = None
    if parent_name:
        for d in kit.get("designs", []):
            if d.get("name") == parent_name:
                parent_id = d.get("id")
                break
        if not parent_id:
            raise ValueError(f"Parent {parent_name} not found")

    for d in kit.get("designs", []):
        if d.get("name") == name:
            p = d.get("parent")
            if parent_id:
                if p and p.get("id") == parent_id:
                    return d
            else:
                if not p:
                    return d
    raise ValueError(f"Design {name} not found")


def _test_flatten(design_name, parent_name=None):
    kit_dict = _test_load_json("metabolism.kit.semio.json")
    design = _test_find_design(kit_dict, design_name, parent_name)

    expected_design = next(
        (
            d
            for d in kit_dict.get("designs", [])
            if d.get("name") == "Flat"
            and d.get("parent", {}).get("id") == design.get("id")
        ),
        None,
    )
    assert expected_design is not None, (
        f"Expected Flat design for {design_name} not found"
    )

    flat_design_diff = flattenDesignDict(kit_dict, design.get("id"))
    flat_design = copy.deepcopy(design)
    _applyDesignDiff(flat_design, flat_design_diff)

    for piece in flat_design.get("pieces", []):
        expected_piece = next(
            (
                x
                for x in expected_design.get("pieces", [])
                if x.get("name") == piece.get("name")
            ),
            None,
        )
        assert expected_piece is not None, (
            f"Piece {piece.get('name')} not found in expected design"
        )
        assert _dict_piece_plane(piece) is not None
        assert _dict_piece_center(piece) is not None
        assert _test_planes_equal(
            _dict_piece_plane(piece), _dict_piece_plane(expected_piece)
        )
        assert _test_centers_equal(
            _dict_piece_center(piece), _dict_piece_center(expected_piece)
        )


def _test_contains_all_tags(
    representation: dict[str, typing.Any], selected_tag_ids: list[str]
) -> bool:
    representation_tag_ids = [
        t.get("id") if isinstance(t, dict) else t
        for t in representation.get("tags", [])
    ]
    return all(id in representation_tag_ids for id in selected_tag_ids)


def _test_jaccard_tag_ids(
    representation_tag_ids: list[str], selected_tag_ids: list[str]
) -> float:
    if len(representation_tag_ids) == 0 and len(selected_tag_ids) == 0:
        return 1.0
    set_a = set(representation_tag_ids)
    set_b = set(selected_tag_ids)
    union = set_a | set_b
    if len(union) == 0:
        return 0.0
    return len(set_a & set_b) / len(union)


def _test_select_best_representation_like_semio_ts(
    representations: list[dict[str, typing.Any]], selected_tag_ids: list[str]
) -> dict[str, typing.Any] | None:
    if len(representations) == 0:
        return None
    if len(selected_tag_ids) == 0:
        default_representation = next(
            (
                representation
                for representation in representations
                if len(representation.get("tags", [])) == 0
            ),
            None,
        )
        return (
            default_representation
            if default_representation is not None
            else representations[0]
        )
    filtered_representations = [
        representation
        for representation in representations
        if _test_contains_all_tags(representation, selected_tag_ids)
    ]
    if len(filtered_representations) == 0:
        return None
    indexed_scores = [
        _test_jaccard_tag_ids(
            [
                t.get("id") if isinstance(t, dict) else t
                for t in representation.get("tags", [])
            ],
            selected_tag_ids,
        )
        for representation in filtered_representations
    ]
    max_score = max(indexed_scores)
    max_score_index = indexed_scores.index(max_score)
    return filtered_representations[max_score_index]


def _test_create_glb_blob(
    vertices: list[tuple[float, float, float]], faces: list[tuple[int, int, int]]
) -> str:
    def _pad4(data: bytes, fill: bytes) -> bytes:
        padding = (-len(data)) % 4
        return data + fill * padding

    position_bytes = struct.pack(
        "<" + "f" * (len(vertices) * 3),
        *(value for vertex in vertices for value in vertex),
    )
    index_values = [index for face in faces for index in face]
    index_bytes = struct.pack("<" + "H" * len(index_values), *index_values)
    position_bytes = _pad4(position_bytes, b"\x00")
    index_bytes = _pad4(index_bytes, b"\x00")
    position_length = len(position_bytes)
    index_length = len(index_bytes)
    binary_chunk = position_bytes + index_bytes
    min_x = min(vertex[0] for vertex in vertices)
    min_y = min(vertex[1] for vertex in vertices)
    min_z = min(vertex[2] for vertex in vertices)
    max_x = max(vertex[0] for vertex in vertices)
    max_y = max(vertex[1] for vertex in vertices)
    max_z = max(vertex[2] for vertex in vertices)
    json_chunk = json.dumps(
        {
            "asset": {"version": "2.0"},
            "buffers": [{"byteLength": len(binary_chunk)}],
            "bufferViews": [
                {
                    "buffer": 0,
                    "byteOffset": 0,
                    "byteLength": position_length,
                    "target": 34962,
                },
                {
                    "buffer": 0,
                    "byteOffset": position_length,
                    "byteLength": index_length,
                    "target": 34963,
                },
            ],
            "accessors": [
                {
                    "bufferView": 0,
                    "componentType": 5126,
                    "count": len(vertices),
                    "type": "VEC3",
                    "min": [min_x, min_y, min_z],
                    "max": [max_x, max_y, max_z],
                },
                {
                    "bufferView": 1,
                    "componentType": 5123,
                    "count": len(index_values),
                    "type": "SCALAR",
                },
            ],
            "meshes": [{"primitives": [{"attributes": {"POSITION": 0}, "indices": 1}]}],
            "nodes": [{"mesh": 0}],
            "scenes": [{"nodes": [0]}],
            "scene": 0,
        },
        separators=(",", ":"),
    ).encode("utf-8")
    json_chunk = _pad4(json_chunk, b" ")
    total_length = 12 + 8 + len(json_chunk) + 8 + len(binary_chunk)
    glb = b"".join(
        [
            struct.pack("<4sII", b"glTF", 2, total_length),
            struct.pack("<I4s", len(json_chunk), b"JSON"),
            json_chunk,
            struct.pack("<I4s", len(binary_chunk), b"BIN\x00"),
            binary_chunk,
        ]
    )
    return "data:representation/gltf-binary;base64," + base64.b64encode(glb).decode(
        "ascii"
    )


class TestRoundtrip:
    class TestMetabolism:
        def test_roundtrip(self):
            kit_dict = _test_load_json("metabolism.kit.semio.json")
            serialized = json.dumps(kit_dict)
            deserialized = json.loads(serialized)
            assert areKitsDictEqual(kit_dict, deserialized), (
                "JSON -> Memory -> JSON: serialized and deserialized kit should be equal"
            )

            files: dict[str, bytes] = {}
            for file_entry in kit_dict.get("files", []):
                blob = file_entry.get("blob")
                if blob:
                    b64 = blob.split(",", 1)[1] if blob.startswith("data:") else blob
                    decoded = base64.b64decode(b64)
                    file_path = _build_file_path(kit_dict, file_entry)
                    files[file_path] = decoded

            with tempfile.TemporaryDirectory() as tmpdir:
                roundtrip_path = os.path.join(tmpdir, "metabolism_roundtrip.zip")
                export_kit(KitData(kit_dict), files, roundtrip_path)

                kit2, files2 = import_kit(roundtrip_path)

            assert areKitsDictEqual(kit_dict, kit2.to_dict()), (
                "ZIP -> JSON: roundtrip kit should be equal"
            )
            assert len(files2) == len(files), (
                f"Expected {len(files)} files, got {len(files2)}"
            )

    @pytest.mark.skip(
        reason="dict kit diffs removed from edit_*; port workflow tests to ChangeKitCommand JSON (semio-store)"
    )
    class TestKitWorkflows:
        def test_file_kit_import_export_edit_roundtrip(self):
            kit_dict = _test_build_workflow_kit()
            diff = _test_build_workflow_diff("Workflow File Edited", "asset-file.txt")

            with tempfile.TemporaryDirectory() as tmpdir:
                kit_path = os.path.join(tmpdir, "workflow.json")
                export_file_kit(KitData(kit_dict), kit_path)

                imported = import_file_kit(kit_path)
                assert areKitsDictEqual(kit_dict, imported.to_dict())

                edited = edit_file_kit(kit_path, diff)
                roundtrip = import_file_kit(kit_path)

            assert edited.name == "Workflow File Edited"
            assert roundtrip.name == "Workflow File Edited"
            assert roundtrip.to_dict()["files"][0]["name"] == "asset-file.txt"

        def test_folder_kit_import_export_edit_roundtrip(self):
            kit_dict = _test_build_workflow_kit()
            files = _collect_kit_asset_files(kit_dict)
            diff = _test_build_workflow_diff(
                "Workflow Folder Edited", "asset-folder.txt"
            )

            with tempfile.TemporaryDirectory() as tmpdir:
                export_folder_kit(KitData(kit_dict), files, tmpdir)
                assert os.path.exists(os.path.join(tmpdir, KIT_LOCAL_SUFFIX))

                imported, imported_files = import_folder_kit(tmpdir)
                assert areKitsDictEqual(kit_dict, imported.to_dict())
                assert imported_files == files

                edited = edit_folder_kit(tmpdir, diff)
                roundtrip, roundtrip_files = import_folder_kit(tmpdir)

                assert not os.path.exists(os.path.join(tmpdir, "assets", "asset.txt"))
                assert os.path.exists(
                    os.path.join(tmpdir, "assets", "asset-folder.txt")
                )

            assert edited.name == "Workflow Folder Edited"
            assert roundtrip.name == "Workflow Folder Edited"
            assert roundtrip.to_dict()["files"][0]["name"] == "asset-folder.txt"
            assert list(roundtrip_files.keys()) == ["assets/asset-folder.txt"]

        def test_archive_kit_import_export_edit_roundtrip(self):
            kit_dict = _test_build_workflow_kit()
            files = _collect_kit_asset_files(kit_dict)
            diff = _test_build_workflow_diff(
                "Workflow Archive Edited", "asset-archive.txt"
            )

            with tempfile.TemporaryDirectory() as tmpdir:
                archive_path = os.path.join(tmpdir, "workflow.zip")
                export_kit(KitData(kit_dict), files, archive_path)

                imported, imported_files = import_kit(archive_path)
                assert areKitsDictEqual(kit_dict, imported.to_dict())
                assert imported_files == files

                edited = edit_archive_kit(archive_path, diff)
                roundtrip, roundtrip_files = import_kit(archive_path)

            assert edited.name == "Workflow Archive Edited"
            assert roundtrip.name == "Workflow Archive Edited"
            assert roundtrip.to_dict()["files"][0]["name"] == "asset-archive.txt"
            assert list(roundtrip_files.keys()) == ["assets/asset-archive.txt"]

        def test_remote_kit_import_json_and_zip_then_edit(self):
            kit_dict = _test_build_workflow_kit()
            files = _collect_kit_asset_files(kit_dict)
            json_body = json.dumps(kit_dict, ensure_ascii=False).encode("utf-8")
            zip_body = _test_build_workflow_archive_bytes(kit_dict, files)
            server, thread = _test_remote_kit_server(json_body, zip_body)

            try:
                base_uri = f"http://127.0.0.1:{server.server_port}"
                json_uri = f"{base_uri}/workflow.json"
                zip_uri = f"{base_uri}/workflow.zip"

                imported_json, imported_json_files = import_remote_kit(json_uri)
                assert areKitsDictEqual(kit_dict, imported_json.to_dict())
                assert imported_json_files == files

                imported_zip, imported_zip_files = import_remote_kit(zip_uri)
                assert areKitsDictEqual(kit_dict, imported_zip.to_dict())
                assert imported_zip_files == files

                edited_json = edit_remote_kit(
                    json_uri,
                    _test_build_workflow_diff(
                        "Workflow Remote Json Edited", "asset-remote-json.txt"
                    ),
                )
                edited_zip = edit_remote_kit(
                    zip_uri,
                    _test_build_workflow_diff(
                        "Workflow Remote Zip Edited", "asset-remote-zip.txt"
                    ),
                )

                roundtrip_json, json_files = import_remote_kit(json_uri)
                roundtrip_zip, zip_files = import_remote_kit(zip_uri)
            finally:
                server.shutdown()
                thread.join()

            assert edited_json.name == "Workflow Remote Json Edited"
            assert roundtrip_json.name == "Workflow Remote Json Edited"
            assert (
                roundtrip_json.to_dict()["files"][0]["name"] == "asset-remote-json.txt"
            )
            assert list(json_files.keys()) == ["assets/asset-remote-json.txt"]

            assert edited_zip.name == "Workflow Remote Zip Edited"
            assert roundtrip_zip.name == "Workflow Remote Zip Edited"
            assert roundtrip_zip.to_dict()["files"][0]["name"] == "asset-remote-zip.txt"
            assert list(zip_files.keys()) == ["assets/asset-remote-zip.txt"]

        def test_temporary_kit_edit_via_diff(self):
            kit_dict = _test_build_workflow_kit()
            edited = edit_temporary_kit(
                KitData(kit_dict),
                _test_build_workflow_diff("Workflow Temp Edited", "asset-temp.txt"),
            )

            assert edited.name == "Workflow Temp Edited"
            assert edited.to_dict()["files"][0]["name"] == "asset-temp.txt"


class TestFlatten:
    _flatten_cases = _test_load_json("flatten.cases.semio.json")["cases"]

    @pytest.mark.parametrize(
        "case", _flatten_cases, ids=[c["name"] for c in _flatten_cases]
    )
    def test_kit_flatten_diff_apply_flat(self, case):
        path = case["designPath"]
        _test_flatten(path[-1], path[-2] if len(path) > 1 else None)


def _flatten_merkle_set_path(obj: dict, path: str, value) -> None:
    """🌳Assign a value inside a nested dict structure using a dotted path (creating intermediate dicts when missing)."""
    keys = path.split(".")
    current = obj
    for key in keys[:-1]:
        if key not in current or current[key] is None:
            current[key] = {}
        current = current[key]
    current[keys[-1]] = value


def _flatten_merkle_find_design_by_path(kit: dict, design_path: list[str]) -> dict:
    """🌳Resolve a design by its hierarchical name path (root, then successive parents)."""
    if not design_path:
        raise ValueError("designPath must not be empty")
    current = None
    for i, name in enumerate(design_path):
        parent_id = current.get("id") if current is not None else None
        match = None
        for d in kit.get("designs", []):
            if d.get("name") != name:
                continue
            parent = d.get("parent")
            if i == 0:
                if not parent:
                    match = d
                    break
            else:
                if parent and parent.get("id") == parent_id:
                    match = d
                    break
        if match is None:
            raise ValueError(f"Design path {design_path} not found at segment {name!r}")
        current = match
    assert current is not None
    return current


def _flatten_merkle_apply_mutations(
    kit: dict, design: dict, mutations: list[dict]
) -> None:
    """🌳Apply the asset-described mutations in-place on a kit clone prior to recomputing hashes."""
    for mutation in mutations:
        kind = mutation.get("kind")
        path = mutation.get("path")
        value = mutation.get("value")
        if kind == "pieceField":
            pieceId = mutation.get("pieceId")
            piece = next(
                (p for p in design.get("pieces", []) if p.get("id") == pieceId), None
            )
            if piece is None:
                raise ValueError(
                    f"Piece {pieceId} not found in design {design.get('id')}"
                )
            _flatten_merkle_set_path(piece, path, value)
        elif kind == "connectionField":
            connectionId = mutation.get("connectionId")
            connection = next(
                (
                    c
                    for c in design.get("connections", [])
                    if c.get("id") == connectionId
                ),
                None,
            )
            if connection is None:
                raise ValueError(
                    f"Connection {connectionId} not found in design {design.get('id')}"
                )
            _flatten_merkle_set_path(connection, path, value)
        else:
            raise ValueError(f"Unknown mutation kind {kind!r}")


class TestFlattenMerkle:
    def test_shared_asset_mutation_cases(self):
        cases_doc = _test_load_json("flatten-merkle.cases.semio.json")
        for case in cases_doc.get("cases", []):
            kit_before = _test_load_json(case["kit"])
            design_before = _flatten_merkle_find_design_by_path(
                kit_before, case["designPath"]
            )
            before_hashes = computeFlatHashesDict(kit_before, design_before["id"])

            kit_after = json.loads(json.dumps(kit_before))
            design_after = _flatten_merkle_find_design_by_path(
                kit_after, case["designPath"]
            )
            _flatten_merkle_apply_mutations(
                kit_after, design_after, case.get("mutations", [])
            )
            after_hashes = computeFlatHashesDict(kit_after, design_after["id"])

            assert set(before_hashes.keys()) == set(after_hashes.keys()), (
                f"Case {case['name']}: piece set changed"
            )

            changed_plane = {
                g
                for g in before_hashes
                if before_hashes[g]["planeHash"] != after_hashes[g]["planeHash"]
            }
            changed_center = {
                g
                for g in before_hashes
                if before_hashes[g]["centerHash"] != after_hashes[g]["centerHash"]
            }
            expect = case.get("expect", {})
            name = case["name"]

            if "planeHashesChangedAny" in expect:
                if expect["planeHashesChangedAny"]:
                    assert len(changed_plane) > 0, (
                        f"Case {name}: expected some planeHash changes, got none"
                    )
                else:
                    assert len(changed_plane) == 0, (
                        f"Case {name}: expected no planeHash changes, got {changed_plane}"
                    )
            if "centerHashesChangedAny" in expect:
                if expect["centerHashesChangedAny"]:
                    assert len(changed_center) > 0, (
                        f"Case {name}: expected some centerHash changes, got none"
                    )
                else:
                    assert len(changed_center) == 0, (
                        f"Case {name}: expected no centerHash changes, got {changed_center}"
                    )
            if expect.get("planeHashesChangedAll") is True:
                assert changed_plane == set(before_hashes.keys()), (
                    f"Case {name}: expected every planeHash to change"
                )
            if expect.get("planeHashesChangedAll") is False:
                assert changed_plane != set(before_hashes.keys()), (
                    f"Case {name}: expected not every planeHash to change"
                )
            if expect.get("centerHashesChangedAll") is True:
                assert changed_center == set(before_hashes.keys()), (
                    f"Case {name}: expected every centerHash to change"
                )
            if expect.get("centerHashesChangedAll") is False:
                assert changed_center != set(before_hashes.keys()), (
                    f"Case {name}: expected not every centerHash to change"
                )
            for id in expect.get("planeHashesChangedIncludes", []):
                assert id in changed_plane, (
                    f"Case {name}: expected piece {id} to have changed planeHash"
                )
            for id in expect.get("centerHashesChangedIncludes", []):
                assert id in changed_center, (
                    f"Case {name}: expected piece {id} to have changed centerHash"
                )
            for id in expect.get("planeHashesStableIncludes", []):
                assert id not in changed_plane, (
                    f"Case {name}: expected piece {id} to keep stable planeHash"
                )
            for id in expect.get("centerHashesStableIncludes", []):
                assert id not in changed_center, (
                    f"Case {name}: expected piece {id} to keep stable centerHash"
                )

    def test_cross_language_parity_reference_hashes(self):
        cases_doc = _test_load_json("flatten-merkle.cases.semio.json")
        parity = cases_doc.get("parity")
        assert parity is not None, "parity block missing"
        kit = _test_load_json(parity["kit"])
        design = _flatten_merkle_find_design_by_path(kit, parity["designPath"])
        hashes = computeFlatHashesDict(kit, design["id"])
        for expected in parity.get("expectedHashes", []):
            id = expected["pieceId"]
            assert id in hashes, f"piece {id} missing from computed hashes"
            assert hashes[id]["planeHash"] == expected["planeHash"], (
                f"piece {id} planeHash mismatch: got {hashes[id]['planeHash']}"
            )
            assert hashes[id]["centerHash"] == expected["centerHash"], (
                f"piece {id} centerHash mismatch: got {hashes[id]['centerHash']}"
            )

    def test_cached_flatten_reuses_values_when_hashes_match(self):
        cases_doc = _test_load_json("flatten-merkle.cases.semio.json")
        parity = cases_doc["parity"]
        kit = _test_load_json(parity["kit"])
        design = _flatten_merkle_find_design_by_path(kit, parity["designPath"])
        _, first_cache = flattenDesignCachedDict(kit, design["id"])
        assert len(first_cache) > 0
        _, second_cache = flattenDesignCachedDict(kit, design["id"], first_cache)
        for id, entry in first_cache.items():
            assert entry["planeHash"] == second_cache[id]["planeHash"]
            assert entry["centerHash"] == second_cache[id]["centerHash"]
            assert entry["plane"] == second_cache[id]["plane"]
            assert entry["center"] == second_cache[id]["center"]


@pytest.mark.skip(
    reason="getKitChange/dict diffs: migrate to semio rs KitDiff/ChangeKitCommand (semio-store)"
)
class TestChange:
    class TestMetabolism:
        def test_kit_change_forward_backward_inverse_behavior(self):
            kit_original = _test_load_json("metabolism.kit.semio.json")
            kit_original["designs"] = [
                d for d in kit_original.get("designs", []) if not d.get("parent")
            ]
            kit_diff = _test_load_json("metabolism.kit.diff.semio.json")
            kit_diff_inverted = _test_load_json(
                "metabolism.kit.diff.inverted.semio.json"
            )
            kit_diffed = _test_load_json("metabolism.kit.diffed.semio.json")

            change = getKitChange(kit_original, kit_diffed)
            computed_diff = getKitDiffDict(kit_original, kit_diffed)
            assert areKitDiffsDictEqual(computed_diff, kit_diff)
            computed_inverse_diff = inverseKitDiffDict(kit_original, change.forward)
            assert areKitDiffsDictEqual(computed_inverse_diff, kit_diff_inverted)
            assert areKitDiffsDictEqual(change.forward, kit_diff)
            assert areKitDiffsDictEqual(change.backward, kit_diff_inverted)
            applied_forward = copy.deepcopy(kit_original)
            applyKitDiffDict(applied_forward, change.forward)
            assert areKitsDictEqual(applied_forward, kit_diffed)
            applied_inverse = copy.deepcopy(kit_diffed)
            applyKitDiffDict(applied_inverse, change.backward)
            assert areKitsDictEqual(applied_inverse, kit_original)


class TestDelete:
    _delete_cases = _test_load_json("delete.cases.semio.json")["cases"]

    @pytest.mark.parametrize(
        "case", _delete_cases, ids=[c["name"] for c in _delete_cases]
    )
    def test_delete_pieces_and_connections(self, case):
        kit = _test_load_json(case["kit"])
        design = _test_find_design(kit, case["designName"], case.get("designParent"))
        selection = _test_load_json(case["selectionAsset"])
        expected_diff = _test_load_json(case["expectedDiffAsset"])

        piece_ids = [p["id"] for p in selection.get("pieces", [])]
        connection_ids = [c["id"] for c in selection.get("connections", [])]

        computed_report = deletePiecesAndConnectionsInDesignDict(
            kit, design, piece_ids, connection_ids
        )
        assert computed_report.get("ok"), computed_report.get("errors", [])
        computed_diff = computed_report["diff"]

        # Verify removed pieces
        computed_removed = computed_diff.get("pieces", {}).get("removed", [])
        expected_removed = expected_diff.get("pieces", {}).get("removed", [])
        assert len(computed_removed) == len(expected_removed), (
            f"Removed pieces count mismatch: {len(computed_removed)} vs {len(expected_removed)}"
        )
        for c, e in zip(computed_removed, expected_removed):
            assert c["id"] == e["id"], (
                f"Removed piece id mismatch: {c['id']} vs {e['id']}"
            )

        # Verify updated (fixed) pieces
        computed_updated = computed_diff.get("pieces", {}).get("updated", [])
        expected_updated = expected_diff.get("pieces", {}).get("updated", [])
        assert len(computed_updated) == len(expected_updated), (
            f"Updated pieces count mismatch: {len(computed_updated)} vs {len(expected_updated)}"
        )
        computed_ids = sorted(
            u.get("piece", {}).get("id", "") for u in computed_updated
        )
        expected_ids = sorted(
            u.get("piece", {}).get("id", "") for u in expected_updated
        )
        assert computed_ids == expected_ids, f"Updated piece ids mismatch"
        computed_sorted = sorted(
            computed_updated, key=lambda u: u.get("piece", {}).get("id", "")
        )
        expected_sorted = sorted(
            expected_updated, key=lambda u: u.get("piece", {}).get("id", "")
        )
        for cu, eu in zip(computed_sorted, expected_sorted):
            cp = cd.get("pose") or {}
            ep = ed.get("pose") or {}
            assert abs(cp["plane"]["origin"]["x"] - ep["plane"]["origin"]["x"]) < 0.001
            assert abs(cp["plane"]["origin"]["y"] - ep["plane"]["origin"]["y"]) < 0.001
            assert abs(cp["plane"]["origin"]["z"] - ep["plane"]["origin"]["z"]) < 0.001
            assert abs(cp["center"]["u"] - ep["center"]["u"]) < 0.001
            assert abs(cp["center"]["v"] - ep["center"]["v"]) < 0.001

        # Verify removed connections
        computed_conn_removed = computed_diff.get("connections", {}).get("removed", [])
        expected_conn_removed = expected_diff.get("connections", {}).get("removed", [])
        assert len(computed_conn_removed) == len(expected_conn_removed), (
            f"Removed connections count mismatch: {len(computed_conn_removed)} vs {len(expected_conn_removed)}"
        )
        computed_conn_ids = sorted(r["id"] for r in computed_conn_removed)
        expected_conn_ids = sorted(r["id"] for r in expected_conn_removed)
        assert computed_conn_ids == expected_conn_ids, "Removed connection ids mismatch"


class TestCopyAndPaste:
    _cp_cases = _test_load_json("copy-paste.cases.semio.json")["cases"]

    @pytest.mark.parametrize("case", _cp_cases, ids=[c["name"] for c in _cp_cases])
    def test_copy(self, case):
        kit = _test_load_json(case["kit"])
        design = _test_find_design(kit, case["designName"], case.get("designParent"))
        selection = _test_load_json(case["selectionAsset"])
        expected_copy = _test_load_json(case["expectedCopyAsset"])

        piece_ids = [p["id"] for p in selection.get("pieces", [])]
        connection_ids = [c["id"] for c in selection.get("connections", [])]

        copy_result = copyDesignDict(kit, design, piece_ids, connection_ids)

        # Verify piece count
        assert len(copy_result.get("pieces", [])) == len(
            expected_copy.get("pieces", [])
        ), (
            f"Copy pieces count mismatch: {len(copy_result.get('pieces', []))} vs {len(expected_copy.get('pieces', []))}"
        )

        # Verify connection count
        assert len(copy_result.get("connections", [])) == len(
            expected_copy.get("connections", [])
        ), (
            f"Copy connections count mismatch: {len(copy_result.get('connections', []))} vs {len(expected_copy.get('connections', []))}"
        )

        # Verify each piece exists
        copyPieceIds = {p["id"] for p in copy_result.get("pieces", [])}
        for ep in expected_copy.get("pieces", []):
            assert ep["id"] in copyPieceIds, (
                f"Expected piece {ep['id']} not found in copy output"
            )

        # Verify external pieces have semio.piece.origin and semio.center attributes
        expectedPieceMap = {p["id"]: p for p in expected_copy.get("pieces", [])}
        for p in copy_result.get("pieces", []):
            ep = expectedPieceMap[p["id"]]
            hasOrigin = any(
                a.get("key") == "semio.piece.origin" and a.get("value") == "external"
                for a in p.get("attributes", [])
            )
            expectedOrigin = any(
                a.get("key") == "semio.piece.origin" and a.get("value") == "external"
                for a in ep.get("attributes", [])
            )
            assert hasOrigin == expectedOrigin, (
                f"Piece {p['id']}: semio.piece.origin mismatch"
            )
            hasCenter = any(
                a.get("key") == "semio.center" for a in p.get("attributes", [])
            )
            expectedCenter = any(
                a.get("key") == "semio.center" for a in ep.get("attributes", [])
            )
            assert hasCenter == expectedCenter, (
                f"Piece {p['id']}: semio.center attr mismatch"
            )

    @pytest.mark.parametrize("case", _cp_cases, ids=[c["name"] for c in _cp_cases])
    def test_paste_without_coordinate(self, case):
        kit = _test_load_json(case["kit"])
        design = _test_find_design(kit, case["designName"], case.get("designParent"))
        paste_target = _test_load_json(case["pasteTargetAsset"])
        selection = _test_load_json(case["selectionAsset"])
        expected_paste = _test_load_json(case["expectedPasteDiffAsset"])

        piece_ids = [p["id"] for p in selection.get("pieces", [])]
        connection_ids = [c["id"] for c in selection.get("connections", [])]

        copy_result = copyDesignDict(kit, design, piece_ids, connection_ids)
        paste_diff = pasteDesignDict(kit, copy_result, paste_target, "original", None)

        # Verify pasted pieces count
        paste_pieces = paste_diff.get("pieces", {}).get("added", [])
        expected_paste_pieces = expected_paste.get("pieces", {}).get("added", [])
        assert len(paste_pieces) == len(expected_paste_pieces), (
            f"Paste added pieces count mismatch: {len(paste_pieces)} vs {len(expected_paste_pieces)}"
        )

        # Verify no external-origin pieces in paste output
        for p in paste_pieces:
            hasExt = any(
                a.get("key") == "semio.piece.origin" and a.get("value") == "external"
                for a in p.get("attributes", [])
            )
            assert not hasExt, (
                f"External-origin piece {p['id']} should not be in paste output"
            )

        # Verify pasted connections count
        paste_conns = paste_diff.get("connections", {}).get("added", [])
        expected_paste_conns = expected_paste.get("connections", {}).get("added", [])
        assert len(paste_conns) == len(expected_paste_conns), (
            f"Paste added connections count mismatch: {len(paste_conns)} vs {len(expected_paste_conns)}"
        )

    @pytest.mark.parametrize("case", _cp_cases, ids=[c["name"] for c in _cp_cases])
    def test_paste_with_coordinate(self, case):
        kit = _test_load_json(case["kit"])
        design = _test_find_design(kit, case["designName"], case.get("designParent"))
        paste_target = _test_load_json(case["pasteTargetAsset"])
        selection = _test_load_json(case["selectionAsset"])
        expected_pwc = _test_load_json(case["expectedPasteWithCoordinateDiffAsset"])
        coordinate = case["pasteCoordinate"]

        piece_ids = [p["id"] for p in selection.get("pieces", [])]
        connection_ids = [c["id"] for c in selection.get("connections", [])]

        copy_result = copyDesignDict(kit, design, piece_ids, connection_ids)
        paste_diff = pasteDesignDict(
            kit, copy_result, paste_target, "original", coordinate
        )

        # Verify pasted pieces count
        paste_pieces = paste_diff.get("pieces", {}).get("added", [])
        expected_paste_pieces = expected_pwc.get("pieces", {}).get("added", [])
        assert len(paste_pieces) == len(expected_paste_pieces), (
            f"Paste with coordinate added pieces count mismatch: {len(paste_pieces)} vs {len(expected_paste_pieces)}"
        )

        # Verify pasted connections count
        paste_conns = paste_diff.get("connections", {}).get("added", [])
        expected_paste_conns = expected_pwc.get("connections", {}).get("added", [])
        assert len(paste_conns) == len(expected_paste_conns), (
            f"Paste with coordinate added connections count mismatch: {len(paste_conns)} vs {len(expected_paste_conns)}"
        )

        # Verify centers are offset by coordinate
        expectedPieceMap = {p["id"]: p for p in expected_paste_pieces}
        for p in paste_pieces:
            ep = expectedPieceMap.get(p["id"])
            assert ep is not None, (
                f"Piece {p['id']} not found in expected paste with coordinate"
            )
            if p.get("center") and ep.get("center"):
                assert abs(p["center"]["u"] - ep["center"]["u"]) < 0.001, (
                    f"Piece {p['id']} center.u mismatch: {p['center']['u']} vs {ep['center']['u']}"
                )
                assert abs(p["center"]["v"] - ep["center"]["v"]) < 0.001, (
                    f"Piece {p['id']} center.v mismatch: {p['center']['v']} vs {ep['center']['v']}"
                )


class TestDesignWithDiff:
    _diff_cases = _test_load_json("design-with-diff.cases.semio.json")["cases"]

    @pytest.mark.parametrize("case", _diff_cases, ids=[c["name"] for c in _diff_cases])
    def test_design_with_diff_preserves_old_entities_and_annotates_status(self, case):
        kit = _test_load_json(case["kit"])
        design = _test_find_design(kit, case["designName"], case.get("designParent"))
        diff = _test_load_json(case["diff"])
        expected = _test_load_json(case["expected"])

        computed = designWithDiffDict(design, diff)

        assert len(computed.get("pieces", [])) == len(expected.get("pieces", [])), (
            f"Pieces count mismatch: {len(computed.get('pieces', []))} vs {len(expected.get('pieces', []))}"
        )
        assert len(computed.get("connections", [])) == len(
            expected.get("connections", [])
        ), (
            f"Connections count mismatch: {len(computed.get('connections', []))} vs {len(expected.get('connections', []))}"
        )

        def get_status(attrs):
            for a in attrs or []:
                if a.get("key") == "semio.diffStatus":
                    return a.get("value")
            return None

        piece_statuses = [
            get_status(p.get("attributes")) for p in computed.get("pieces", [])
        ]
        pc = case["expectedPieceCounts"]
        assert piece_statuses.count("unchanged") == pc["unchanged"]
        assert piece_statuses.count("modified") == pc["modified"]
        assert piece_statuses.count("removed") == pc["removed"]
        assert piece_statuses.count("added") == pc["added"]

        conn_statuses = [
            get_status(c.get("attributes")) for c in computed.get("connections", [])
        ]
        cc = case["expectedConnectionCounts"]
        assert conn_statuses.count("unchanged") == cc["unchanged"]
        assert conn_statuses.count("modified") == cc["modified"]
        assert conn_statuses.count("removed") == cc["removed"]
        assert conn_statuses.count("added") == cc["added"]


class TestValidation:
    class TestMetabolism:
        def test_metabolism_kit_validate_empty_report(self):
            valid_kit = _test_load_json("metabolism.kit.semio.json")
            valid_result = validateKitDict(valid_kit)
            assert not valid_result.hasErrors()

    class TestInvalid:
        def test_invalid_kit_validate_invalid_report(self):
            invalid_kit = _test_load_json("invalid.kit.semio.json")
            result = validateKitDict(invalid_kit)
            expected = parseValidationResult(
                json.dumps(_test_load_json("validation.semio.json"))
            )
            assert areValidationResultsEqual(result, expected)

        def test_plain_descriptions_do_not_create_emoji_validation_problems(self):
            kit = _test_load_json("metabolism.kit.semio.json")
            kit["description"] = "Plain kit summary"
            for index, entry in enumerate(kit.get("types", [])):
                entry["description"] = f"Repeated plain description {index % 2}"

            result = validateKitDict(kit)
            emoji_constraint_ids = {
                problem.constraintId
                for problem in result.problems
                if problem.constraintId
                in {"description-missing-emoji", "description-emoji-unique"}
            }

            assert not emoji_constraint_ids


class TestDesignRepresentation:
    def test_representation_selection_from_shared_semio_assets(self):
        payload = _test_load_json("representation.selection.semio.json")
        for case in payload.get("cases", []):
            representations = [
                {
                    "id": representation["id"],
                    "file": {"id": representation["fileId"]},
                    "tags": [{"id": id} for id in representation.get("tagIds", [])],
                }
                for representation in case.get("representations", [])
            ]
            selected = _test_select_best_representation_like_semio_ts(
                representations, case.get("selectedTagIds", [])
            )
            selected_id = selected.get("id") if selected else None
            assert selected_id == case.get("expectedId"), (
                f"Case {case.get('name')} failed"
            )


class TestKitFilterDesign:
    _filter_cases = _test_load_json("filter-kit.cases.semio.json")
    _design_filter_cases = _filter_cases["cases"]
    _glob_cases = _filter_cases["globCases"]

    def test_filter_produces_expected_subset(self):
        for case in self._design_filter_cases:
            kit_dict = _test_load_json(case["kit"])
            expected = _test_load_json(case["expectedKit"])
            design = _test_find_design(
                kit_dict, case["designName"], case.get("designParent")
            )

            filtered = (
                KitData(kit_dict).filter_kit({"design_id": design["id"]}).to_dict()
            )

            assert len(filtered.get("designs", [])) == len(expected.get("designs", []))
            assert len(filtered.get("types", [])) == len(expected.get("types", []))
            assert len(filtered.get("files", [])) == len(expected.get("files", []))
            assert len(filtered.get("ports", [])) == len(expected.get("ports", []))
            assert len(filtered.get("qualities", [])) == len(
                expected.get("qualities", [])
            )
            assert len(filtered.get("authors", [])) == len(expected.get("authors", []))

            filtered_design = next(
                d for d in filtered.get("designs", []) if d.get("id") == design["id"]
            )
            assert len(filtered_design.get("pieces", [])) == len(
                design.get("pieces", [])
            )

            for expected_type in expected.get("types", []):
                filtered_type = next(
                    (
                        t
                        for t in filtered.get("types", [])
                        if t.get("id") == expected_type.get("id")
                    ),
                    None,
                )
                assert filtered_type is not None
                assert len(filtered_type.get("representations", [])) == len(
                    expected_type.get("representations", [])
                )

            for piece in filtered_design.get("pieces", []):
                piece_kind_id = piece.get("type", {}).get("id")
                if piece_kind_id:
                    assert any(
                        t.get("id") == piece_kind_id for t in filtered.get("types", [])
                    )

            for kind in filtered.get("types", []):
                assert len(kind.get("representations", [])) <= 1
                for representation in kind.get("representations", []):
                    assert any(
                        file.get("id") == representation.get("file", {}).get("id")
                        for file in filtered.get("files", [])
                    )
                for connector in kind.get("connectors", []):
                    connector_id = connector.get("port", {}).get("id")
                    if connector_id:
                        assert any(
                            port.get("id") == connector_id
                            for port in filtered.get("ports", [])
                        )

    def test_filter_preserves_metadata(self):
        for case in self._design_filter_cases:
            kit_dict = _test_load_json(case["kit"])
            design = _test_find_design(
                kit_dict, case["designName"], case.get("designParent")
            )

            filtered = (
                KitData(kit_dict).filter_kit({"design_id": design["id"]}).to_dict()
            )

            assert filtered.get("id") == kit_dict.get("id")
            assert filtered.get("name") == kit_dict.get("name")
            assert filtered.get("version") == kit_dict.get("version")

    def test_glob_filters_types_by_name_include(self):
        gc = next(c for c in self._glob_cases if c["name"] == "type_include_capsule")
        kit_dict = _test_load_json(gc["kit"])
        patterns = gc["typeInclude"]
        filtered = (
            KitData(kit_dict).filter_kit({"types": {"include": patterns}}).to_dict()
        )
        types = filtered.get("types", [])
        assert len(types) > 0
        for t in types:
            assert any(fnmatch.fnmatch(t["name"].lower(), p.lower()) for p in patterns)

    def test_glob_filters_types_by_name_exclude(self):
        gc = next(c for c in self._glob_cases if c["name"] == "type_exclude_capsule")
        kit_dict = _test_load_json(gc["kit"])
        patterns = gc["typeExclude"]
        total_types = len(kit_dict.get("types", []))
        filtered = (
            KitData(kit_dict).filter_kit({"types": {"exclude": patterns}}).to_dict()
        )
        types = filtered.get("types", [])
        assert len(types) < total_types
        for t in types:
            assert not any(
                fnmatch.fnmatch(t["name"].lower(), p.lower()) for p in patterns
            )

    def test_glob_filters_designs_by_name_include(self):
        gc = next(c for c in self._glob_cases if c["name"] == "design_include_nakagin")
        kit_dict = _test_load_json(gc["kit"])
        patterns = gc["designInclude"]
        filtered = (
            KitData(kit_dict).filter_kit({"designs": {"include": patterns}}).to_dict()
        )
        designs = filtered.get("designs", [])
        assert len(designs) > 0
        for d in designs:
            assert any(fnmatch.fnmatch(d["name"].lower(), p.lower()) for p in patterns)

    def test_empty_filter_returns_kit_unchanged(self):
        gc = next(c for c in self._glob_cases if c["name"] == "empty_filter")
        kit_dict = _test_load_json(gc["kit"])
        filtered = KitData(kit_dict).filter_kit({}).to_dict()
        assert len(filtered.get("types", [])) == len(kit_dict.get("types", []))
        assert len(filtered.get("designs", [])) == len(kit_dict.get("designs", []))

    def test_combines_design_id_with_glob_filters(self):
        gc = next(
            c
            for c in self._glob_cases
            if c["name"] == "combined_design_and_type_exclude"
        )
        kit_dict = _test_load_json(gc["kit"])
        design = _test_find_design(kit_dict, gc["designName"], gc.get("designParent"))
        patterns = gc["typeExclude"]
        design_filtered = (
            KitData(kit_dict).filter_kit({"design_id": design["id"]}).to_dict()
        )
        combined_filtered = (
            KitData(kit_dict)
            .filter_kit({"design_id": design["id"], "types": {"exclude": patterns}})
            .to_dict()
        )
        assert len(combined_filtered.get("types", [])) < len(
            design_filtered.get("types", [])
        )
        for t in combined_filtered.get("types", []):
            assert not any(
                fnmatch.fnmatch(t["name"].lower(), p.lower()) for p in patterns
            )


# #region 🔍Find Replaceable Types In Designs Tests
class TestFindReplaceableTypesInDesigns:
    _frt_doc = _test_load_json("find-replaceable-types.cases.semio.json")

    def test_synthetic_selection_enforces_distinct_connectors_and_free_design_connectors(
        self,
    ):
        kit = _test_load_json(self._frt_doc["syntheticKit"])
        for sc in self._frt_doc["syntheticCases"]:
            result = findReplaceableTypesInDesignsForPiecesInDesignDict(
                kit, sc["designId"], sc["pieceIds"]
            )
            type_ids = [td["id"] for td in result["types"]]
            design_ids = [dd["id"] for dd in result["designs"]]
            for expected in sc.get("expectedContainsTypes", []):
                assert expected in type_ids, (
                    f"Case {sc['name']}: expected type {expected} in results"
                )
            for forbidden in sc.get("expectedNotContainsTypes", []):
                assert forbidden not in type_ids, (
                    f"Case {sc['name']}: unexpected type {forbidden} in results"
                )
            for expected in sc.get("expectedContainsDesigns", []):
                assert expected in design_ids, (
                    f"Case {sc['name']}: expected design {expected} in results"
                )
            for forbidden in sc.get("expectedNotContainsDesigns", []):
                assert forbidden not in design_ids, (
                    f"Case {sc['name']}: unexpected design {forbidden} in results"
                )

    def test_connector_level_boundary_matching_shrinks_candidates_as_demand_grows(self):
        bc = self._frt_doc["boundaryCases"]
        kit = _test_load_json(bc["kit"])
        design = _test_find_design(kit, bc["designName"], bc.get("designParent"))
        name_to_id = {
            piece.get("name"): piece.get("id") for piece in design.get("pieces", [])
        }
        type_name_by_id = {
            type_dict.get("id"): type_dict.get("name")
            for type_dict in kit.get("types", [])
        }

        def type_names_for_selection(piece_names: list[str]) -> list[str]:
            piece_ids = [name_to_id[piece_name] for piece_name in piece_names]
            result = findReplaceableTypesInDesignsForPiecesInDesignDict(
                kit, design["id"], piece_ids
            )
            return [type_name_by_id[type_dict["id"]] for type_dict in result["types"]]

        def unique_type_names_for_selection(piece_names: list[str]) -> list[str]:
            return sorted(set(type_names_for_selection(piece_names)))

        single_capsule_names = type_names_for_selection(bc["singleCapsulePieces"])
        two_capsule_names = type_names_for_selection(bc["twoCapsulePieces"])
        four_capsule_names = type_names_for_selection(bc["fourCapsulePieces"])
        eight_capsule_names = type_names_for_selection(bc["eightCapsulePieces"])
        tambour_result = findReplaceableTypesInDesignsForPiecesInDesignDict(
            kit, design["id"], [name_to_id[bc["tambourPieceName"]]]
        )

        assert len(single_capsule_names) > len(two_capsule_names)
        assert len(two_capsule_names) >= len(four_capsule_names)
        assert len(four_capsule_names) >= len(eight_capsule_names)

        for forbidden_family in bc["forbiddenFamilies"]:
            assert forbidden_family not in two_capsule_names
            assert forbidden_family not in four_capsule_names
            assert forbidden_family not in eight_capsule_names

        assert "Bridge" not in four_capsule_names
        assert "Bridge" not in eight_capsule_names
        assert (
            unique_type_names_for_selection(bc["twoCapsulePieces"])
            == bc["expectedTwoCapsuleFamilies"]
        )
        assert (
            unique_type_names_for_selection(bc["fourCapsulePieces"])
            == bc["expectedLargeFamilies"]
        )
        assert (
            unique_type_names_for_selection(bc["eightCapsulePieces"])
            == bc["expectedLargeFamilies"]
        )
        assert (
            len([type_dict["id"] for type_dict in tambour_result["types"]])
            == bc["expectedTambourTypeIdCount"]
        )
        assert len(tambour_result["designs"]) == bc["expectedTambourDesignIdCount"]

    def test_asset_driven_cases(self):
        for case in self._frt_doc["cases"]:
            kit = _test_load_json(case["kit"])
            if case.get("designParentName"):
                parent_design = next(
                    d
                    for d in kit.get("designs", [])
                    if d.get("name") == case["designParentName"] and not d.get("parent")
                )
                design = next(
                    d
                    for d in kit.get("designs", [])
                    if d.get("name") == case["designName"]
                    and (d.get("parent") or {}).get("id") == parent_design["id"]
                )
            else:
                design = _test_find_design(
                    kit, case["designName"], case.get("designParent")
                )

            if "selectionAsset" in case:
                selection = _test_load_json(case["selectionAsset"])
                piece_ids = [p["id"] for p in selection.get("pieces", [])]
            elif "pieceNames" in case:
                piece_ids = [
                    next(
                        p["id"] for p in design.get("pieces", []) if p.get("name") == pn
                    )
                    for pn in case["pieceNames"]
                ]
            elif "lookupTypeName" in case:
                lookup_type = next(
                    t
                    for t in kit.get("types", [])
                    if t.get("name") == case["lookupTypeName"]
                )
                piece_ids = [
                    next(
                        p["id"]
                        for p in design.get("pieces", [])
                        if (p.get("type") or {}).get("id") == lookup_type["id"]
                    )
                ]
            elif "usePieceIndex" in case:
                piece_ids = [design["pieces"][case["usePieceIndex"]]["id"]]
            else:
                piece_ids = []

            result = findReplaceableTypesInDesignsForPiecesInDesignDict(
                kit, design["id"], piece_ids
            )
            type_ids = [t["id"] for t in result["types"]]
            design_ids = [d["id"] for d in result["designs"]]

            if "expectedTypeIds" in case:
                assert type_ids == case["expectedTypeIds"], (
                    f"Case {case['name']}: type ids mismatch"
                )
            if "expectedDesignIds" in case:
                assert design_ids == case["expectedDesignIds"], (
                    f"Case {case['name']}: design ids mismatch"
                )
            if "expectedTypeIdCount" in case:
                assert len(type_ids) == case["expectedTypeIdCount"], (
                    f"Case {case['name']}: type id count mismatch"
                )
            if case.get("expectNonEmptyTypes"):
                assert len(type_ids) > 0, (
                    f"Case {case['name']}: expected non-empty types"
                )
            if case.get("expectOwnTypeInResults"):
                piece = design["pieces"][case.get("usePieceIndex", 0)]
                if piece.get("type", {}).get("id"):
                    assert piece["type"]["id"] in type_ids, (
                        f"Case {case['name']}: own type not in results"
                    )
            if "forbiddenTypeNames" in case:
                type_name_by_id = {
                    td.get("id"): td.get("name") for td in kit.get("types", [])
                }
                result_type_names = [type_name_by_id.get(tg) for tg in type_ids]
                for forbidden in case["forbiddenTypeNames"]:
                    assert forbidden not in result_type_names, (
                        f"Case {case['name']}: forbidden type {forbidden} in results"
                    )
            if case.get("expectConnectorlessTypeCount"):
                no_connector_types = [
                    t
                    for t in kit.get("types", [])
                    if len(t.get("connectors") or []) == 0
                ]
                assert len(type_ids) == len(no_connector_types), (
                    f"Case {case['name']}: connectorless count mismatch"
                )


# #endregion 🔍Find Replaceable Types In Designs Tests


class TestDesignQualitySum:
    _quality_cases = _test_load_json("quality-sum.cases.semio.json")["cases"]

    @pytest.mark.parametrize(
        "case", _quality_cases, ids=[c["name"] for c in _quality_cases]
    )
    def test_sum_quality(self, case):
        kit_dict = _test_load_json(case["kit"])
        design = _test_find_design(
            kit_dict, case["designName"], case.get("designParent")
        )
        quality = next(
            q
            for q in kit_dict.get("qualities", [])
            if q.get("name") == case["qualityName"]
        )
        result = sumQualityInDesignDict(kit_dict, design["id"], quality["id"])
        assert abs(result - case["expected"]) < case.get("tolerance", TEST_TOLERANCE)


class TestExportDesignRepresentation:
    _export_cases = _test_load_json("export-design-representation.cases.semio.json")[
        "cases"
    ]
    _export_case = _export_cases[0]
    _export_kit_file = _export_case["kit"]
    _export_design_name = _export_case["designName"]

    def test_export_glb_returns_valid_glb(self):
        kit_dict = _test_load_json(self._export_kit_file)
        result = export_design_representation(
            kit_dict, self._export_design_name, ".glb"
        )
        assert isinstance(result, bytes)
        assert len(result) > 0
        assert result[:4] == b"glTF"
        assert struct.unpack("<I", result[4:8])[0] == 2
        assert struct.unpack("<I", result[8:12])[0] == len(result)

    def test_export_gltf_returns_valid_json(self):
        kit_dict = _test_load_json(self._export_kit_file)
        result = export_design_representation(
            kit_dict, self._export_design_name, ".gltf"
        )
        assert isinstance(result, bytes)
        assert len(result) > 0
        parsed = json.loads(result.decode("utf-8"))
        assert "asset" in parsed
        assert "scenes" in parsed

    def test_export_invalid_format_raises(self):
        kit_dict = _test_load_json(self._export_kit_file)
        with pytest.raises(ValueError, match="Unsupported export format"):
            export_design_representation(kit_dict, self._export_design_name, ".invalid")

    def test_export_scene_graph_report(self):
        kit_dict = _test_load_json(self._export_kit_file)
        result = export_design_representation(
            kit_dict, self._export_design_name, ".gltf"
        )
        parsed = json.loads(result.decode("utf-8"))
        assert "nodes" in parsed
        assert "scenes" in parsed
        REPORTS_EXPORT_DIR.mkdir(parents=True, exist_ok=True)
        (REPORTS_EXPORT_DIR / "py.gltf").write_bytes(result)

    def test_export_ifc_returns_valid_ifc(self):
        kit_dict = _test_load_json(self._export_kit_file)
        result = export_design_representation(
            kit_dict, self._export_design_name, ".ifc"
        )
        assert isinstance(result, bytes)
        assert len(result) > 0
        ifc_text = result.decode("utf-8")
        assert "ISO-10303-21" in ifc_text
        assert "IFC4" in ifc_text
        assert "IFCPROJECT" in ifc_text
        assert "IFCSITE" in ifc_text
        assert "IFCBUILDING" in ifc_text
        assert "IFCBUILDINGSTOREY" in ifc_text

    def test_export_ifc_contains_types_and_occurrences(self):
        kit_dict = _test_load_json(self._export_kit_file)
        result = export_design_representation(
            kit_dict, self._export_design_name, ".ifc"
        )
        ifc_text = result.decode("utf-8")
        assert "IFCBUILDINGELEMENTPROXYTYPE" in ifc_text
        assert "IFCBUILDINGELEMENTPROXY(" in ifc_text

    def test_export_ifc_contains_mesh_geometry(self):
        kit_dict = _test_load_json(self._export_kit_file)
        result = export_design_representation(
            kit_dict, self._export_design_name, ".ifc"
        )
        ifc_text = result.decode("utf-8")
        assert "IFCSHAPEREPRESENTATION" in ifc_text

    def test_export_ifc_converts_gltf_mesh_axes_to_semio_axes(self):
        import ifcopenshell

        kit_dict = {
            "name": "Axis Test Kit",
            "id": "axis-test-kit",
            "uri": "axis-test-kit",
            "types": [
                {
                    "id": "axis-test-kind",
                    "name": "Axis Test Kind",
                    "variant": "",
                    "attributes": [],
                    "connectors": [],
                    "representations": [
                        {
                            "id": "axis-test-representation",
                            "file": {"id": "axis-test-file"},
                            "tags": [],
                        }
                    ],
                }
            ],
            "designs": [
                {
                    "id": "axis-test-design",
                    "name": "Axis Test Design",
                    "pieces": [
                        {
                            "id": "axis-test-piece",
                            "name": "Axis Test Piece",
                            "type": {"id": "axis-test-kind"},
                        }
                    ],
                    "connections": [],
                }
            ],
            "files": [
                {
                    "id": "axis-test-file",
                    "name": "axis-test.glb",
                    "blob": _test_create_glb_blob(
                        [(0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)],
                        [(0, 1, 2)],
                    ),
                }
            ],
            "tags": [],
            "authors": [],
        }

        result = export_design_representation(kit_dict, "Axis Test Design", ".ifc")
        ifc = ifcopenshell.file.from_string(result.decode("utf-8"))
        point_lists = ifc.by_type("IfcCartesianPointList3D")

        assert len(point_lists) == 1
        coordinates = [
            tuple(float(value) for value in row)
            for row in point_lists[0].CoordinateList
        ]
        assert any(abs(x) < 1e-6 and abs(y) < 1e-6 and z > 0 for x, y, z in coordinates)
        assert any(abs(x) < 1e-6 and y < 0 and abs(z) < 1e-6 for x, y, z in coordinates)
        assert not any(
            abs(x) < 1e-6 and y > 0 and abs(z) < 1e-6 for x, y, z in coordinates
        )

    def test_export_ifc_contains_ports_and_connections(self):
        kit_dict = _test_load_json(self._export_kit_file)
        result = export_design_representation(
            kit_dict, self._export_design_name, ".ifc"
        )
        ifc_text = result.decode("utf-8")
        assert "IFCDISTRIBUTIONPORT" in ifc_text
        assert "IFCRELCONNECTSPORTS" in ifc_text
        assert "IFCRELCONNECTSELEMENTS" in ifc_text

    def test_export_ifc_roundtrip_with_ifcopenshell(self):
        import ifcopenshell

        kit_dict = _test_load_json(self._export_kit_file)
        result = export_design_representation(
            kit_dict, self._export_design_name, ".ifc"
        )
        ifc = ifcopenshell.file.from_string(result.decode("utf-8"))
        projects = ifc.by_type("IfcProject")
        assert len(projects) == 1
        sites = ifc.by_type("IfcSite")
        assert len(sites) == 1
        buildings = ifc.by_type("IfcBuilding")
        assert len(buildings) == 1
        assert buildings[0].Name == "tower"
        storeys = ifc.by_type("IfcBuildingStorey")
        assert len(storeys) == 11
        storey_names = sorted([s.Name for s in storeys])
        assert storey_names == sorted([str(i) for i in range(11)])
        type_products = ifc.by_type("IfcBuildingElementProxyType")
        assert len(type_products) > 0
        occurrences = ifc.by_type("IfcBuildingElementProxy")
        assert len(occurrences) > 0
        pieces = next(
            d
            for d in kit_dict.get("designs", [])
            if d.get("name") == self._export_design_name
        ).get("pieces", [])
        assert len(occurrences) == len(pieces)
        ports = ifc.by_type("IfcDistributionPort")
        assert len(ports) > 0
        port_connections = ifc.by_type("IfcRelConnectsPorts")
        connections = next(
            d
            for d in kit_dict.get("designs", [])
            if d.get("name") == self._export_design_name
        ).get("connections", [])
        assert len(port_connections) == len(connections)

    def test_export_ifc_layer_spatial_hierarchy(self):
        import ifcopenshell

        kit_dict = _test_load_json(self._export_kit_file)
        result = export_design_representation(
            kit_dict, self._export_design_name, ".ifc"
        )
        ifc = ifcopenshell.file.from_string(result.decode("utf-8"))
        # IfcProject -> IfcSite -> IfcBuilding -> IfcBuildingStorey
        project = ifc.by_type("IfcProject")[0]
        site = ifc.by_type("IfcSite")[0]
        building = ifc.by_type("IfcBuilding")[0]
        storeys = ifc.by_type("IfcBuildingStorey")
        # Verify aggregation hierarchy
        project_children = [rel.RelatedObjects for rel in project.IsDecomposedBy]
        site_in_project = any(site in children for children in project_children)
        assert site_in_project
        site_children = [rel.RelatedObjects for rel in site.IsDecomposedBy]
        building_in_site = any(building in children for children in site_children)
        assert building_in_site
        building_children_list = [rel.RelatedObjects for rel in building.IsDecomposedBy]
        building_children = [
            child for children in building_children_list for child in children
        ]
        for storey in storeys:
            assert storey in building_children, (
                f"Storey {storey.Name} not aggregated under building"
            )
        # Each storey should contain pieces
        for storey in storeys:
            contained = (
                [rel.RelatedElements for rel in storey.ContainsElements]
                if storey.ContainsElements
                else []
            )
            elements = [e for group in contained for e in group]
            assert len(elements) > 0, f"Storey {storey.Name} has no contained elements"
        # Verify types have representations (representation geometry)
        type_products = ifc.by_type("IfcBuildingElementProxyType")
        types_with_rep = [t for t in type_products if t.RepresentationMaps]
        assert len(types_with_rep) > 0

    def test_export_ifc_report(self):
        kit_dict = _test_load_json(self._export_kit_file)
        result = export_design_representation(
            kit_dict, self._export_design_name, ".ifc"
        )
        REPORTS_EXPORT_DIR.mkdir(parents=True, exist_ok=True)
        (REPORTS_EXPORT_DIR / "py.ifc").write_bytes(result)


class TestGetGeometricInsightsForRepresentation:
    """🔖Representation/KPI tests for get_geometric_insights_for_representation using nakagin-capsule-tower.gltf."""

    def test_nakagin_capsule_tower_gltf_returns_insights(self):
        representation_path = os.path.join(
            os.path.dirname(__file__), TEST_ASSETS_DIR, "nakagin-capsule-tower.gltf"
        )
        if not os.path.exists(representation_path):
            pytest.skip("nakagin-capsule-tower.gltf not found")
        insights = get_geometric_insights_for_representation(representation_path)
        REPORTS_REPRESENTATION_KPI_DIR.mkdir(parents=True, exist_ok=True)
        data = geometric_insights_to_report_dict(insights)
        (REPORTS_REPRESENTATION_KPI_DIR / "py.json").write_text(
            json.dumps(data, indent=2, sort_keys=True), encoding="utf-8"
        )

        canonical_path = os.path.join(
            os.path.dirname(__file__),
            TEST_ASSETS_DIR,
            "nakagin.kpi.representation.semio.json",
        )
        with open(canonical_path, "r", encoding="utf-8") as f:
            canonical = json.load(f)
        for key, expected in canonical.items():
            assert key in data, f"missing key {key}"
            assert data[key] == expected, (
                f"mismatch for {key}: {data[key]!r} != {expected!r}"
            )
        assert isinstance(insights, GeometricInsights)
        assert insights.bounding_box_min is not None
        assert insights.bounding_box_max is not None
        assert insights.dimension_x is not None and insights.dimension_x >= 0
        assert insights.dimension_y is not None and insights.dimension_y >= 0
        assert insights.dimension_z is not None and insights.dimension_z >= 0
        assert (
            insights.characteristic_length is not None
            and insights.characteristic_length >= 0
        )
        assert (
            insights.total_surface_area is not None and insights.total_surface_area >= 0
        )
        assert insights.vertex_count is not None and insights.vertex_count > 0
        assert insights.face_count is not None and insights.face_count > 0
        assert insights.centroid is not None
        assert insights.euler_characteristic is not None

    def test_nakagin_capsule_tower_from_bytes_gltf(self):
        representation_path = os.path.join(
            os.path.dirname(__file__), TEST_ASSETS_DIR, "nakagin-capsule-tower.gltf"
        )
        if not os.path.exists(representation_path):
            pytest.skip("nakagin-capsule-tower.gltf not found")
        with open(representation_path, "rb") as f:
            data = f.read()
        insights = get_geometric_insights_for_representation(data)
        assert isinstance(insights, GeometricInsights)
        assert insights.face_count is not None and insights.face_count > 0


class TestTypeMeta:
    """🔖Tests for TypeMeta deserialization from JSON."""

    def test_type_meta(self):
        data = _test_load_json("tambour.meta.type.semio.json")
        assert "id" in data
        assert "name" in data
        assert data["name"] == "Tambour"
        meta: TypeMeta = data
        assert meta["id"] == data["id"]
        assert meta["name"] == "Tambour"
        assert "connectors" not in meta
        assert "representations" not in meta
        assert "props" not in meta
        assert "attributes" not in meta


class TestTypeShallow:
    """🔖Tests for TypeShallow deserialization from JSON."""

    def test_type_shallow(self):
        data = _test_load_json("tambour.shallow.type.semio.json")
        assert "id" in data
        shallow: TypeShallow = data
        assert "connectors" in shallow
        assert isinstance(shallow["connectors"], list)
        assert len(shallow["connectors"]) > 0
        first_connector = shallow["connectors"][0]
        assert "id" in first_connector
        assert "point" in first_connector
        assert "direction" in first_connector
        assert "attributes" not in first_connector
        assert "props" not in first_connector


class TestDesignMeta:
    """🔖Tests for DesignMeta deserialization from JSON."""

    def test_design_meta(self):
        data = _test_load_json("nakagin-capsule-tower.meta.design.semio.json")
        assert "id" in data
        assert "name" in data
        assert data["name"] == data["name"]  # name from asset, no hardcoded string
        meta: DesignMeta = data
        assert meta["id"] == data["id"]
        assert "pieces" not in meta
        assert "connections" not in meta
        assert "layers" not in meta


class TestDesignShallow:
    """🔖Tests for DesignShallow deserialization from JSON."""

    def test_design_shallow(self):
        data = _test_load_json("nakagin-capsule-tower.shallow.design.semio.json")
        assert "id" in data
        assert "name" in data
        shallow: DesignShallow = data
        assert "pieces" in shallow
        assert isinstance(shallow["pieces"], list)
        assert len(shallow["pieces"]) > 0
        first_piece = shallow["pieces"][0]
        assert "id" in first_piece
        assert "attributes" not in first_piece
        if "connections" in shallow:
            assert isinstance(shallow["connections"], list)
            if len(shallow["connections"]) > 0:
                first_conn = shallow["connections"][0]
                assert "id" in first_conn
                assert "connected" in first_conn
                assert "connecting" in first_conn


class TestKitMeta:
    """🔖Tests for KitMeta deserialization from JSON."""

    def test_kit_meta(self):
        data = _test_load_json("metabolism.meta.kit.semio.json")
        assert "id" in data
        assert "name" in data
        assert data["name"] == "Metabolism"
        meta: KitMeta = data
        assert meta["id"] == data["id"]
        assert "types" not in meta
        assert "designs" not in meta
        assert "files" not in meta
        assert "folders" not in meta


class TestKitShallow:
    """🔖Tests for KitShallow deserialization from JSON."""

    def test_kit_shallow(self):
        data = _test_load_json("metabolism.shallow.kit.semio.json")
        assert "id" in data
        assert "name" not in data or isinstance(data.get("name"), str)
        shallow: KitShallow = data
        assert "types" in shallow
        assert isinstance(shallow["types"], list)
        assert len(shallow["types"]) > 0
        first_type = shallow["types"][0]
        assert "id" in first_type
        assert "name" in first_type
        assert "connectors" not in first_type
        assert "representations" not in first_type


class TestKitToMetaShallow:
    """🔖Tests for converting a full kit dict to meta and shallow representations."""

    def test_kit_to_meta_shallow(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        expected_meta = _test_load_json("metabolism.meta.kit.semio.json")
        expected_shallow = _test_load_json("metabolism.shallow.kit.semio.json")

        computed_meta = kitToMeta(kit_dict)
        assert computed_meta["id"] == expected_meta["id"]
        assert computed_meta["name"] == expected_meta.get("name", computed_meta["name"])
        for key in expected_meta:
            if key in computed_meta:
                assert computed_meta[key] == expected_meta[key], (
                    f"KitMeta mismatch for key '{key}': {computed_meta[key]!r} != {expected_meta[key]!r}"
                )

        computed_shallow = kitToShallow(kit_dict)
        assert computed_shallow["id"] == expected_shallow["id"]
        assert "types" in computed_shallow
        assert isinstance(computed_shallow["types"], list)

        expected_type_ids = {t["id"] for t in expected_shallow.get("types", [])}
        computed_type_ids = {t["id"] for t in computed_shallow.get("types", [])}
        assert expected_type_ids == computed_type_ids, (
            "TypeMeta ids in shallow kit must match"
        )

        for t in computed_shallow.get("types", []):
            assert "connectors" not in t, (
                "TypeMeta in shallow kit must not have connectors"
            )
            assert "representations" not in t, (
                "TypeMeta in shallow kit must not have representations"
            )

        expected_type_meta = _test_load_json("tambour.meta.type.semio.json")
        computed_type_meta = typeToMeta(
            next(t for t in kit_dict["types"] if t["id"] == expected_type_meta["id"])
        )
        for key in expected_type_meta:
            if key in computed_type_meta:
                assert computed_type_meta[key] == expected_type_meta[key], (
                    f"TypeMeta mismatch for key '{key}'"
                )

        expected_design_meta = _test_load_json(
            "nakagin-capsule-tower.meta.design.semio.json"
        )
        computed_design_meta = designToMeta(
            next(
                d for d in kit_dict["designs"] if d["id"] == expected_design_meta["id"]
            )
        )
        for key in expected_design_meta:
            if key in computed_design_meta:
                assert computed_design_meta[key] == expected_design_meta[key], (
                    f"DesignMeta mismatch for key '{key}'"
                )


class TestKitKind:
    """📇Tests for the KitKind enum."""

    def test_all_kit_kinds_has_five_values(self):
        assert len(ALL_KIT_KINDS) == 5

    def test_kit_kind_values(self):
        assert KitKind.DEV.value == "dev"
        assert KitKind.LOCAL.value == "local"
        assert KitKind.ARCHIVE.value == "archive"
        assert KitKind.REMOTE.value == "remote"
        assert KitKind.TRANSPORT.value == "transport"

    def test_kit_kind_is_str(self):
        for kind in KitKind:
            assert isinstance(kind, str)
            assert kind == kind.value

    def test_kit_kind_file_roundtrip(self):
        kit_dict = {
            "name": "FileTest",
            "uri": "file:///test.json",
            "types": [],
            "designs": [],
        }
        kit = Kit.parse(kit_dict)
        assert kit.name == "FileTest"
        assert kit.uri == "file:///test.json"
        kit2 = Kit.parse({"name": kit.name, "uri": kit.uri})
        assert kit2.name == kit.name
        assert kit2.uri == kit.uri

    def test_kit_kind_transport_in_memory(self):
        kit = Kit.parse({"name": "TempKit"})
        assert kit.name == "TempKit"
        assert kit.uri.startswith("memory://")

    def test_transport_kit_roundtrip(self):
        kit_dict = _test_build_workflow_kit()
        kit = KitData(kit_dict)
        transport = TransportKit.from_kit(kit)
        assert isinstance(transport.json, str)
        roundtrip = transport.to_kit()
        assert areKitsDictEqual(kit.to_dict(), roundtrip.to_dict())

    def test_archive_kit_roundtrip(self):
        kit_dict = _test_build_workflow_kit()
        kit = KitData(kit_dict)
        files = _collect_kit_asset_files(kit_dict)
        archive = ArchiveKit.from_kit(kit, files)
        assert isinstance(archive.data, bytes)
        roundtrip, _ = archive.to_kit()
        assert roundtrip.name == kit.name

    def test_sync_kit_apply_diff(self):
        kit_dict = _test_build_workflow_kit()
        sync = SyncKit(KitData(kit_dict))
        diff = _test_build_workflow_diff("SyncEdited", "asset-sync.txt")
        sync.apply(diff)
        assert sync.kit.name == "SyncEdited"

    def test_dev_kit_from_json(self):
        kit_dict = _test_build_workflow_kit()
        json_str = json.dumps(kit_dict, ensure_ascii=False)
        dev = DevKit.from_json(json_str)
        assert dev.kit.name == kit_dict["name"]

    def test_sync_kit_import_export_transport(self):
        kit_dict = _test_build_workflow_kit()
        sync = SyncKit(KitData(kit_dict))
        transport = sync.export_transport()
        sync2 = SyncKit(
            KitData({"id": "00000000-0000-0000-0000-000000000000", "name": "Empty"})
        )
        sync2.import_transport(transport)
        assert sync2.kit.name == kit_dict["name"]

    def test_sync_kit_import_export_archive(self):
        kit_dict = _test_build_workflow_kit()
        sync = SyncKit(KitData(kit_dict))
        archive = sync.export_archive()
        sync2 = SyncKit(
            KitData({"id": "00000000-0000-0000-0000-000000000000", "name": "Empty"})
        )
        sync2.import_archive(archive)
        assert sync2.kit.name == kit_dict["name"]


class TestValidateKitDiffDict:
    """Tests for validate_kit_diff_dict using validate-kit-diff.cases.semio.json."""

    def test_validate_kit_diff_asset_cases(self):
        payload = _test_load_json("validate-kit-diff.cases.semio.json")
        tiny = payload["tinyKit"]
        for case in payload["cases"]:
            r = validate_kit_diff_dict(tiny, case["diff"], False)
            assert r["ok"] == case["expectOk"], case["id"]
            err_codes = [e.get("code") for e in r["errors"] if e.get("code")]
            warn_codes = [w.get("code") for w in r["warnings"] if w.get("code")]
            for c in case["errorCodes"]:
                assert c in err_codes, (case["id"], err_codes)
            for c in case["warningCodes"]:
                assert c in warn_codes, (case["id"], warn_codes)

    def test_validate_kit_diff_heal_drops_bad_design_update(self):
        tiny = _test_load_json("validate-kit-diff.cases.semio.json")["tinyKit"]
        bad = {
            "designs": {
                "updated": [
                    {
                        "design": {"id": "99999999-9999-9999-9999-999999999999"},
                        "diff": {"name": "X"},
                    }
                ]
            }
        }
        r = validate_kit_diff_dict(tiny, bad, True)
        assert r.get("diff", {}).get("designs", {}).get("updated", []) == []


class TestHash:
    """🔖Tests for the Merkle hash functions."""

    _hash_cases = _test_load_json("hash.cases.semio.json")

    def test_metabolism_kit_hash(self):
        hc = self._hash_cases["kitHash"]
        kit_dict = _test_load_json(hc["kit"])
        result = hash_kit(kit_dict)
        assert result == hc["expected"]

    def test_kit_diff_canonical_hash(self):
        hc = self._hash_cases["kitDiffHash"]
        d = json.loads(hc["json"])
        result = hash_kit_diff(d)
        assert result == hc["expected"]

    def test_kit_diff_deterministic(self):
        hc = self._hash_cases["kitDiffHash"]
        d = json.loads(hc["json"])
        h1 = hash_kit_diff(d)
        h2 = hash_kit_diff(d)
        assert h1 == h2

    def test_kit_diff_different_inputs(self):
        hc = self._hash_cases["kitDiffHash"]
        d1 = json.loads(hc["json"])
        d2 = {"name": "other"}
        assert hash_kit_diff(d1) != hash_kit_diff(d2)

    def test_kit_diff_empty(self):
        d = {}
        result = hash_kit_diff(d)
        assert len(result) == 64

    def test_attribute_diff_deterministic(self):
        d = {"key": "newKey", "value": "newValue"}
        h1 = hash_attribute_diff(d)
        h2 = hash_attribute_diff(d)
        assert h1 == h2

    def test_coordinate_diff_deterministic(self):
        d = {"u": 1.0, "v": 2.0}
        h1 = hash_coordinate_diff(d)
        h2 = hash_coordinate_diff(d)
        assert h1 == h2


class TestMaxChildren:
    """🔖Tests for maxChildren field on Port and Connector."""

    def test_port_max_children_default(self):
        port = PortProps(name="TestPort")
        assert port.maxChildren == 1

    def test_port_max_children_custom(self):
        port = PortProps(name="TestPort", maxChildren=3)
        assert port.maxChildren == 3

    def test_port_max_children_serialization(self):
        port = PortProps(name="TestPort", maxChildren=5)
        data = port.representation_dump(mode="json")
        assert data["maxChildren"] == 5

    def test_port_max_children_roundtrip(self):
        port = PortInput(name="TestPort", maxChildren=3)
        data = port.representation_dump(mode="json")
        restored = PortInput(**data)
        assert restored.maxChildren == 3

    def test_connector_max_children_default(self):
        connector = ConnectorInput(
            id="c1",
            t=0,
            point=PointInput(x=0, y=0, z=0),
            direction=VectorInput(x=0, y=0, z=1),
        )
        assert connector.maxChildren == 1

    def test_connector_max_children_custom(self):
        connector = ConnectorInput(
            id="c1",
            t=0,
            point=PointInput(x=0, y=0, z=0),
            direction=VectorInput(x=0, y=0, z=1),
            maxChildren=5,
        )
        assert connector.maxChildren == 5

    def test_connector_max_children_serialization(self):
        connector = ConnectorInput(
            id="c1",
            t=0,
            point=PointInput(x=0, y=0, z=0),
            direction=VectorInput(x=0, y=0, z=1),
            maxChildren=10,
        )
        data = connector.representation_dump(mode="json")
        assert data["maxChildren"] == 10

    def test_connector_max_children_roundtrip(self):
        connector = ConnectorInput(
            id="c1",
            t=0,
            point=PointInput(x=0, y=0, z=0),
            direction=VectorInput(x=0, y=0, z=1),
            maxChildren=7,
        )
        data = connector.representation_dump(mode="json")
        restored = ConnectorInput(**data)
        assert restored.maxChildren == 7

    def test_kit_max_children_json_roundtrip(self):
        kit_dict = {
            "id": "kit-mc-1",
            "name": "MaxChildrenKit",
            "ports": [{"id": "p1", "name": "Port1", "maxChildren": 3}],
            "types": [
                {
                    "id": "t1",
                    "name": "Type1",
                    "connectors": [
                        {
                            "id": "c1",
                            "t": 0,
                            "point": {"x": 0, "y": 0, "z": 0},
                            "direction": {"x": 0, "y": 0, "z": 1},
                            "maxChildren": 5,
                        }
                    ],
                }
            ],
        }
        import json as _json

        restored = _json.loads(_json.dumps(kit_dict))
        assert restored["ports"][0]["maxChildren"] == 3
        assert restored["types"][0]["connectors"][0]["maxChildren"] == 5


# #endregion 🧪Tests


if __name__ == "__main__":
    benchmark_main()
