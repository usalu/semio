# region Header
# [👤semio📚py💻semio](repo://p/u/semio/b/l/py/f/semio.py)

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

# endregion Header

# region Imports
# [👤semio📚py💻semio🔖imports](repo://p/u/semio/b/l/py/f/semio.py/s/Imports)
# Standard library, third-party and framework imports.
from __future__ import annotations

import abc
import base64
import copy
import dataclasses
import datetime
import enum
import fnmatch
import hashlib
import json
import os
import pathlib
import shutil
import struct
import sys
import tempfile
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

    def _patched_evaluate_forward_ref(type_: typing.ForwardRef, globalns: typing.Any, localns: typing.Any) -> typing.Any:
        return typing.cast(typing.Any, type_)._evaluate(globalns, localns, recursive_guard=frozenset())

    import graphene_pydantic.converters


import graphene_pydantic
import loguru
import networkx
import numpy
import pydantic
import pytest
import pytransform3d.rotations

# endregion Imports

# region Type Hints
# [👤semio📚py💻semio🔖typehints](repo://p/u/semio/b/l/py/f/semio.py/s/Type%20Hints)
# Custom type hint aliases used throughout the module.

RecursiveAnyList = typing.Any | list["RecursiveAnyList"]
""" A recursive any list is either any or a list where the items are recursive any list."""

# endregion Type Hints

# region Constants
# [👤semio📚py💻semio🔖constants](repo://p/u/semio/b/l/py/f/semio.py/s/Constants)
# Global constants for limits, paths, encodings and configuration.

KIT_LOCAL_SUFFIX = str(pathlib.Path(KIT_LOCAL_FOLDERNAME) / pathlib.Path(KIT_LOCAL_FILENAME))
USER_FOLDER = str(pathlib.Path.home() / ".semio")
CACHE_FOLDER = str(pathlib.Path(USER_FOLDER) / "cache")
LOG_FOLDER = str(pathlib.Path(USER_FOLDER) / "logs")
DEBUG_LOG_FILE = str(pathlib.Path(LOG_FOLDER) / "debug.log")
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
    str,
    fastapi.Path(pattern=ENCODING_REGEX + "," + ENCODING_ALPHABET_REGEX + "*" + "," + ENCODING_ALPHABET_REGEX + "*"),
]
dotenv.load_dotenv()
ENVS = {key: value for key, value in os.environ.items() if key.startswith("SEMIO_")}

# endregion Constants

# region Utility
# [👤semio📚py💻semio🔖utility](repo://p/u/semio/b/l/py/f/semio.py/s/Utility)
# General-purpose utility functions for encoding, formatting and transformation.


def encode(value: str) -> str:
    """ᗒ Encode a string to be url safe.
    encode MUST return a percent-encoded string safe for URL paths.
    """
    return urllib.parse.quote(value, safe="")


def decode(value: str) -> str:
    """ᗕ Decode a url safe string.
    decode MUST return the original string from a percent-encoded input.
    """
    return urllib.parse.unquote(value)


def encodeList(items: list[str]) -> str:
    """Encode a list of strings into a comma-separated URL-safe string.
    encodeList MUST encode each item and join them with commas.
    """
    return ",".join([encode(t) for t in items])


def decodeList(encodedList: str) -> list[str]:
    """Decode a comma-separated URL-safe string into a list of strings.
    decodeList MUST split by comma and decode each item.
    """
    return [decode(t) for t in encodedList.split(",")]


def encodeRecursiveAnyList(recursiveAnyList: RecursiveAnyList) -> str:
    """ Encode a `RecursiveAnyList` to a url encoded string.
    encodeRecursiveAnyList MUST recursively encode nested lists into a flat string.
    """
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return encode(",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList]))


def create_id(recursiveAnyList: RecursiveAnyList) -> str:
    """ Turn any into `encoded(str(any))` or a recursive list into a flat comma [,] separated encoded list.
    create_id MUST produce a deterministic identifier from any value or nested list.
    """
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return ",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList])


def pretty(number: float) -> str:
    """🦋 Pretty print a floating point number.
    pretty MUST format the number with up to 5 significant digits.
    """
    if number == -0.0:
    return f"{number:.5f}".rstrip("0").rstrip(".")


def changeValues(c: dict | list, key: str, func: typing.Callable[[typing.Any], typing.Any]) -> None:
    """Recursively change values for a given key in nested dicts and lists.
    changeValues MUST apply the function to all occurrences of the key recursively.
    """
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
    """Recursively transform all keys in nested dicts and lists.
    changeKeys MUST apply the function to all dictionary keys recursively.
    """
    if isinstance(c, dict):
        for k in list(c.keys()):
            newKey = func(k)
            v = c.pop(k)
            if isinstance(v, dict) or isinstance(v, list):
                changeKeys(v, func)
    if isinstance(c, list):
        for v in c:
            if isinstance(v, dict) or isinstance(v, list):
                changeKeys(v, func)


def normalizeAngle(angle: float) -> float:
    """ Normalize an angle to be greater or equal to 0 and smaller than 360 degrees.
    normalizeAngle MUST return an angle in the range [0, 360).
    """


# endregion Utility

# region Logging
# [👤semio📚py💻semio🔖logging](repo://p/u/semio/b/l/py/f/semio.py/s/Logging)
# Module-level logger configuration.


# endregion Logging

# region Exceptions
# [👤semio📚py💻semio🔖exceptions](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions)
# Custom exception hierarchy for server, client and specification errors.


class Error(Exception, abc.ABC):
    """ The base for all exceptions.
    Error MUST provide a descriptive error message via __str__.
    """

    def __str__(self):


class ServerError(Error, abc.ABC):
    """ The base for all server errors.
    ServerError MUST provide a descriptive error message via __str__.
    """


class ClientError(Error, abc.ABC):
    """‍ The base for all client errors.
    ClientError MUST provide a descriptive error message via __str__.
    """


class CodeUnreachable(ServerError):
    """Exception for code paths that should never be reached.
    CodeUnreachable MUST provide a descriptive error message via __str__.
    """

    def __str__(self):


class FeatureNotYetSupported(ServerError):
    """Exception for unimplemented features.
    FeatureNotYetSupported MUST provide a descriptive error message via __str__.
    """

    def __str__(self):


class RemoteKitsNotYetSupported(FeatureNotYetSupported):
    """Exception for unsupported remote kit access.
    RemoteKitsNotYetSupported MUST provide a descriptive error message via __str__.
    """

    def __init__(self, uri: str) -> None:

    def __str__(self):


class AuthenticationError(ClientError):
    """ Base error for authentication failures.
    AuthenticationError MUST provide a descriptive error message via __str__.
    """

    def __str__(self):


class InvalidAuthToken(AuthenticationError):
    """ The auth token is invalid or expired.
    InvalidAuthToken MUST provide a descriptive error message via __str__.
    """

    def __init__(self, serverUrl: str) -> None:

    def __str__(self):


class AuthTokenNotFound(AuthenticationError):
    """ No auth token found for the server.
    AuthTokenNotFound MUST provide a descriptive error message via __str__.
    """

    def __init__(self, serverUrl: str) -> None:

    def __str__(self):


class ServerUnreachable(ClientError):
    """ The remote server is not reachable.
    ServerUnreachable MUST provide a descriptive error message via __str__.
    """

    def __init__(self, serverUrl: str) -> None:

    def __str__(self):


class RemoteKitUriNotValid(ClientError):
    """ The remote kit URI is not valid.
    RemoteKitUriNotValid MUST provide a descriptive error message via __str__.
    """

    def __init__(self, uri: str) -> None:

    def __str__(self):


class NotFound(ClientError, abc.ABC):
    """ The base for not found errors.
    NotFound MUST provide a descriptive error message via __str__.
    """


class SpecificationError(ClientError, abc.ABC):
    """ The base for all specification errors.
    SpecificationError MUST provide a descriptive error message via __str__.
    """


class NoParentAssigned(SpecificationError, abc.ABC):
    """ The base for all no parent assigned errors.
    NoParentAssigned MUST be subclassed and MUST NOT be instantiated directly.
    """


class NoTypeOrDesignAssigned(NoParentAssigned):
    """No Type Or Design Assigned definition.
    NoTypeOrDesignAssigned MUST fulfill its documented contract.
    """

    def __str__(self):


class NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned(NoParentAssigned):
    """No Model Or Port Or Type Or Piece Or Connection Or Design Or Kit Assigned definition.
    NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned MUST fulfill its documented contract.
    """

    def __str__(self):


class AlreadyExists(SpecificationError, abc.ABC):
    """ The entity already exists in the store.
    AlreadyExists MUST provide a descriptive error message via __str__.
    """


class Semio(pydantic.BaseModel):
    """ℹ Metadata about the database.
    Semio MUST implement idMembers and inherit from the appropriate field mixins.
    """

    release: str = pydantic.Field(default=RELEASE)
    """ The current release of semio."""
    engine: str = pydantic.Field(default=VERSION)
    """The version of the engine that created this database."""
    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)
    """⌚ The time when the database was created."""


# endregion Exceptions

# region Modeling
# [🔖semio/py/semio.py#Modeling](repo://section/semio/py/semio.py/MODELING)

# region Primitives
# [👤semio📚py💻semio🔖modeling](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling)
# Abstract base classes for models, fields, ids, inputs, outputs and entities.


class SModel(pydantic.BaseModel, abc.ABC):
    """ The base for models.
    SModel MUST be subclassed and MUST NOT be instantiated directly.
    """

    model_config = pydantic.ConfigDict(arbitrary_types_allowed=True)

    def parse(cls, input: str | dict | typing.Any | None) -> "SModel":
        """ Parse the entity from an input."""
        if input is None:
            return cls()
        if isinstance(input, str):
            return cls.model_validate_json(input)
        return cls.model_validate(input)

    def dump(self) -> "Output":
        """Dump the entity to a dictionary."""
        return self.model_dump()




class Field(SModel, abc.ABC):
    """ The base for a field of a model.
    Field MUST declare exactly one field with appropriate constraints.
    """


class RealField(Field, abc.ABC):
    """🧑 The base for a real field of a model. No lie.
    RealField MUST declare exactly one field with appropriate constraints.
    """


class MaskedField(Field, abc.ABC):
    """ The base for a mask of a field of a model. WYSIWYG but don't expect it to be there.
    MaskedField MUST declare exactly one field with appropriate constraints.
    """


class Base(SModel, abc.ABC):
    """ The base for models.
    Base MUST be subclassed and MUST NOT be instantiated directly.
    """


class Id(Base, abc.ABC):
    """🪪 The base for ids. All fields that identify the entity here.
    Id MUST be subclassed and MUST NOT be instantiated directly.
    """


class Props(Base, abc.ABC):
    """ The base for props. All fields except input-only, output-only or child entities.
    Props MUST be subclassed and MUST NOT be instantiated directly.
    """


class Input(Base, abc.ABC):
    """↘ The base for inputs. All fields that are required to create the entity.
    Input MUST be subclassed and MUST NOT be instantiated directly.
    """


class Context(Base, abc.ABC):
    """ The base for contexts. All fields that are required to understand the entity by an llm.
    Context MUST be subclassed and MUST NOT be instantiated directly.
    """


class Output(Base, abc.ABC):
    """↗ The base for outputs. All fields that are returned when the entity is fetched.
    Output MUST be subclassed and MUST NOT be instantiated directly.
    """


class Prediction(Base, abc.ABC):
    """ The base for predictions. All fields that are required to predict the entity by a llm.
    Prediction MUST be subclassed and MUST NOT be instantiated directly.
    """


class Entity(SModel, abc.ABC):
    """ The base for entities. All fields and behavior of the entity.
    Entity MUST be subclassed and MUST NOT be instantiated directly.
    """

    PLURAL: typing.ClassVar[str]
    """ The plural of the singular of the entity name."""

    def parent_entity(self) -> typing.Optional["Entity"]:
        """ The parent entity of the entity."""

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the entity within its parent."""

    def id(self) -> str:
        """ The id of the entity within its parent."""
        return create_id(self.idMembers())

    def guid(self) -> str:
        """ A Globally Unique Identifier (GUID) of the entity."""
        parent = self.parent_entity()

    def clientId(self) -> str:
        """ The client id of the entity."""
        return self.id()

    # TODO: Automatic emptying.

    def empty(self) -> "Entity":
        """🪣 Empty all props and children of the entity."""
        return self.__class__()

    # TODO: Automatic updating based on props.

    def update(self, other: "Entity") -> "Entity":
        """ Update the props of the entity."""


class Table(SModel, abc.ABC):
    """ The base for tables. All resources that are stored in the database.
    Table MUST be subclassed and MUST NOT be instantiated directly.
    """


class TableEntity(Entity, Table, abc.ABC):
    """ The base for table entities.
    TableEntity MUST be subclassed and MUST NOT be instantiated directly.
    """

    """ The lowercase name of the table in the database."""


# endregion Primitives

# region Graphql
# [👤semio📚py💻semio🔖modeling🔖graphql](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Graphql)
# GraphQL node base classes for pydantic, sqlalchemy and relay integration.


class Node(graphene_pydantic.PydanticObjectType):
    """A base class for all nodes that are not a table in the database.
    Node MUST expose the model via Meta.
    """

    class Meta:

    def __init_subclass_with_meta__(cls, model=None, **options):
        if "name" not in options:

        super().__init_subclass_with_meta__(model=model, **options)


class InputNode(graphene_pydantic.PydanticInputObjectType):
    """A base class for all input nodes.
    InputNode MUST expose the input model via Meta.
    """

    class Meta:


class RelayNode(graphene.relay.Node):
    """Relay-compliant GraphQL node interface.
    RelayNode MUST expose the model via Meta.
    """

    class Meta:

    def to_global_id(type_, id):

    def get_node_from_global_id(info, global_id, only_type=None):
        entity = get(global_id)


class TableNode(graphene_pydantic.PydanticObjectType):
    """A base class for all nodes that are a table in the database.
    It automatically excludes the fields that are defined in the table.
    Resolvers to all @properties are added.
    Child relationships are by default included.
    TableNode MUST expose the model via Meta.
    """

    class Meta:

    def __init_subclass_with_meta__(cls, model=None, **options):
        excludedFields = tuple(k for k, v in model.model_fields.items() if v.exclude or v.default_factory is not None)
        if "exclude_fields" in options:
        else:
        if "name" not in options:

        super().__init_subclass_with_meta__(model=model, **options)


class TableEntityNode(TableNode):
    """A base class for all nodes that are a table in the database and are entities.
    It automatically complies to the Relay Node interface.
    TableEntityNode MUST expose the model via Meta.
    """

    class Meta:

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
# [🔖semio/py/semio.py#Domain](repo://section/semio/py/semio.py/DOMAIN)

# region Attribute
# [👤semio📚py💻semio🔖domain🔖attribute](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute)
# Attribute entity with key-value pairs and definitions.


class AttributeKeyField(RealField, abc.ABC):
    """Field mixin for the key of a attribute.
    AttributeKeyField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class AttributeValueField(RealField, abc.ABC):
    """Field mixin for the value of a attribute.
    AttributeValueField MUST declare exactly one field with appropriate constraints.
    """

    value: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class AttributeDefinitionField(RealField, abc.ABC):
    """Field mixin for the definition of a attribute.
    AttributeDefinitionField MUST declare exactly one field with appropriate constraints.
    """

    definition: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class AttributeId(AttributeKeyField, Id):
    """Identity fields for uniquely identifying a attribute.
    AttributeId MUST contain all fields that uniquely identify a attribute.
    """



class AttributeProps(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Props):
    """Property fields for a attribute.
    AttributeProps MUST contain all non-relational property fields.
    """



class AttributeInput(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Input):
    """Input fields for creating or updating a attribute.
    AttributeInput MUST contain all fields required for creation.
    """



class AttributeContext(AttributeValueField, AttributeKeyField, Context):
    """Context fields for understanding a attribute by an LLM.
    AttributeContext MUST contain all fields needed for LLM understanding.
    """



class AttributeOutput(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Output):
    """Output fields returned when fetching a attribute.
    AttributeOutput MUST contain all fields returned on fetch.
    """



class Attribute(
    AttributeDefinitionField,
    AttributeValueField,
    AttributeKeyField,
    TableEntity,
):
    """Attribute entity storing a key-value pair with an optional definition.
    Attribute MUST implement idMembers and inherit from the appropriate field mixins.
    """


    def parent_entity(
        self,
        "Model",
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
        if self.model is not None:
        if self.connector is not None:
        if self.type is not None:
        if self.piece is not None:
        if self.connection is not None:
        if self.design is not None:
        if self.kit is not None:
        if self.quality is not None:
        if self.prop is not None:
        if self.author is not None:
        if self.location is not None:
        if self.benchmark is not None:
        if self.folder is not None:
        raise NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned()

    def idMembers(self) -> RecursiveAnyList:

    def parse(cls, input: str | dict | typing.Any | None) -> "Attribute":
        if input is None:
            return cls()
            name=obj.get("name", obj.get("key", "")),
            value=obj.get("value", ""),
            definition=obj.get("definition", ""),
        )


class AttributeInputNode(InputNode):
    """GraphQL input node for attribute mutations.
    AttributeInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Attribute

# region Tag
# [👤semio📚py💻semio🔖domain🔖tag](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Tag)
# Tag entity for categorizing and labeling kit elements.


class TagGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a tag.
    TagGuidField MUST declare exactly one field with appropriate constraints.
    """

    guid: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class TagNameField(RealField, abc.ABC):
    """Field mixin for the name of a tag.
    TagNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class TagDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a tag.
    TagDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: typing.Optional[str] = pydantic.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)


class TagIconField(RealField, abc.ABC):
    """Field mixin for the icon of a tag.
    TagIconField MUST declare exactly one field with appropriate constraints.
    """

    icon: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class TagOrderField(RealField, abc.ABC):
    """Field mixin for the order of a tag.
    TagOrderField MUST declare exactly one field with appropriate constraints.
    """

    order: int = pydantic.Field(default=0)


class TagId(TagGuidField, Id):
    """Identity fields for uniquely identifying a tag.
    TagId MUST contain all fields that uniquely identify a tag.
    """



class Tag(
    TagIconField,
    TagDescriptionField,
    TagOrderField,
    TagNameField,
    TagGuidField,
    Table,
):
    """Tag entity for labeling kit elements with name, icon and order.
    Tag MUST implement idMembers and inherit from the appropriate field mixins.
    """


# endregion Tag

# region Concept
# [👤semio📚py💻semio🔖domain🔖concept](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Concept)
# Concept entity for semantic grouping of design elements.


class ConceptGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a concept.
    ConceptGuidField MUST declare exactly one field with appropriate constraints.
    """

    guid: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class ConceptNameField(RealField, abc.ABC):
    """Field mixin for the name of a concept.
    ConceptNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class ConceptDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a concept.
    ConceptDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: typing.Optional[str] = pydantic.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)


class ConceptIconField(RealField, abc.ABC):
    """Field mixin for the icon of a concept.
    ConceptIconField MUST declare exactly one field with appropriate constraints.
    """

    icon: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class ConceptOrderField(RealField, abc.ABC):
    """Field mixin for the order of a concept.
    ConceptOrderField MUST declare exactly one field with appropriate constraints.
    """

    order: int = pydantic.Field(default=0)


class ConceptId(ConceptGuidField, Id):
    """Identity fields for uniquely identifying a concept.
    ConceptId MUST contain all fields that uniquely identify a concept.
    """



class Concept(
    ConceptIconField,
    ConceptDescriptionField,
    ConceptOrderField,
    ConceptNameField,
    ConceptGuidField,
    Table,
):
    """Concept entity for semantic grouping with name, icon and order.
    Concept MUST implement idMembers and inherit from the appropriate field mixins.
    """


# endregion Concept

# region Coord
# [👤semio📚py💻semio🔖domain🔖coord](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Coord)
# Coordinate primitive for three-dimensional values.


class Coord(SModel):
    """Three-dimensional coordinate with x, y and z values.
    Coord MUST contain all coordinate or geometry fields.
    """

    u: float = pydantic.Field()
    v: float = pydantic.Field()

    def __str__(self) -> str:

    def __repr__(self) -> str:


class CoordInput(Coord, Input):
    """Input fields for creating or updating a coord.
    CoordInput MUST contain all fields required for creation.
    """



class CoordContext(Coord, Context):
    """Context fields for understanding a coord by an LLM.
    CoordContext MUST contain all fields needed for LLM understanding.
    """



class CoordOutput(Coord, Output):
    """Output fields returned when fetching a coord.
    CoordOutput MUST contain all fields returned on fetch.
    """



class CoordPrediction(Coord, Prediction):
    """Prediction fields for LLM-based coord inference.
    CoordPrediction MUST contain all fields for LLM inference.
    """



class CoordNode(Node):
    """GraphQL node exposing coord data.
    CoordNode MUST expose the model via Meta.
    """

    class Meta:


class CoordInputNode(InputNode):
    """GraphQL input node for coord mutations.
    CoordInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Coord

# region Point
# [👤semio📚py💻semio🔖domain🔖point](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Point)
# Point primitive representing a position in 3D space.


class Point(SModel):
    """Point in 3D space with x, y and z coordinates.
    Point MUST contain all coordinate or geometry fields.
    """

    x: float = pydantic.Field()
    y: float = pydantic.Field()
    z: float = pydantic.Field()

    def __str__(self) -> str:

    def __repr__(self) -> str:


class PointInput(Point, Input):
    """Input fields for creating or updating a point.
    PointInput MUST contain all fields required for creation.
    """



class PointContext(Point, Context):
    """Context fields for understanding a point by an LLM.
    PointContext MUST contain all fields needed for LLM understanding.
    """



class PointOutput(Point, Output):
    """Output fields returned when fetching a point.
    PointOutput MUST contain all fields returned on fetch.
    """



class PointPrediction(Point, Prediction):
    """Prediction fields for LLM-based point inference.
    PointPrediction MUST contain all fields for LLM inference.
    """



class PointNode(Node):
    """GraphQL node exposing point data.
    PointNode MUST expose the model via Meta.
    """

    class Meta:


class PointInputNode(InputNode):
    """GraphQL input node for point mutations.
    PointInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Point

# region Vector
# [👤semio📚py💻semio🔖domain🔖vector](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Vector)
# Vector primitive representing a direction in 3D space.


class Vector(SModel):
    """Direction vector in 3D space with x, y and z components.
    Vector MUST contain all coordinate or geometry fields.
    """

    x: float = pydantic.Field()
    y: float = pydantic.Field()
    z: float = pydantic.Field()

    def __str__(self) -> str:

    def __repr__(self) -> str:


class VectorInput(Vector, Input):
    """Input fields for creating or updating a vector.
    VectorInput MUST contain all fields required for creation.
    """



class VectorContext(Vector, Context):
    """Context fields for understanding a vector by an LLM.
    VectorContext MUST contain all fields needed for LLM understanding.
    """



class VectorOutput(Vector, Output):
    """Output fields returned when fetching a vector.
    VectorOutput MUST contain all fields returned on fetch.
    """



class VectorPrediction(Vector, Prediction):
    """Prediction fields for LLM-based vector inference.
    VectorPrediction MUST contain all fields for LLM inference.
    """



class VectorNode(Node):
    """GraphQL node exposing vector data.
    VectorNode MUST expose the model via Meta.
    """

    class Meta:


class VectorInputNode(InputNode):
    """GraphQL input node for vector mutations.
    VectorInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Vector

# region Plane
# [👤semio📚py💻semio🔖domain🔖plane](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Plane)
# Plane primitive representing an oriented coordinate frame in 3D space.


class PlaneOriginField(MaskedField, abc.ABC):
    """Field mixin for the origin of a plane.
    PlaneOriginField MUST declare exactly one field with appropriate constraints.
    """

    origin: Point = pydantic.Field()


class PlaneXAxisField(MaskedField, abc.ABC):
    """Field mixin for the x axis of a plane.
    PlaneXAxisField MUST declare exactly one field with appropriate constraints.
    """

    xAxis: Vector = pydantic.Field()


class PlaneYAxisField(MaskedField, abc.ABC):
    """Field mixin for the y axis of a plane.
    PlaneYAxisField MUST declare exactly one field with appropriate constraints.
    """

    yAxis: Vector = pydantic.Field()


class PlaneInput(Input):
    """Input fields for creating or updating a plane.
    PlaneInput MUST contain all fields required for creation.
    """

    origin: PointInput = pydantic.Field()
    xAxis: VectorInput = pydantic.Field()
    yAxis: VectorInput = pydantic.Field()


class PlaneContext(Context):
    """Context fields for understanding a plane by an LLM.
    PlaneContext MUST contain all fields needed for LLM understanding.
    """

    origin: PointContext = pydantic.Field()
    xAxis: VectorContext = pydantic.Field()
    yAxis: VectorContext = pydantic.Field()


class PlaneOutput(PlaneYAxisField, PlaneXAxisField, PlaneOriginField, Output):
    """Output fields returned when fetching a plane.
    PlaneOutput MUST contain all fields returned on fetch.
    """



class Plane(Table):
    """Oriented coordinate frame in 3D space with origin and axes.
    Plane MUST contain all coordinate or geometry fields.
    """

    def origin(self) -> Point:
            x=self.originX,
            y=self.originY,
            z=self.originZ,
        )

    def origin(self, origin: Point):

    def xAxis(self) -> Vector:
            x=self.xAxisX,
            y=self.xAxisY,
            z=self.xAxisZ,
        )

    def xAxis(self, xAxis: Vector):

    def yAxis(self) -> Vector:
            x=self.yAxisX,
            y=self.yAxisY,
            z=self.yAxisZ,
        )

    def yAxis(self, yAxis: Vector):

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    def parse(cls, input: str | dict | PlaneInput | typing.Any | None) -> "Plane":
        if input is None:
            return cls()
        origin = Point.model_validate(obj["origin"])
        xAxis = Vector.model_validate(obj["xAxis"])
        yAxis = Vector.model_validate(obj["yAxis"])
        entity = Plane()


    def dump(self) -> PlaneOutput:
        entity = {**PlaneOriginField.model_validate(self).model_dump()}
        return PlaneOutput(**entity)


class PlaneInputNode(InputNode):
    """GraphQL input node for plane mutations.
    PlaneInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Plane

# region Location
# [👤semio📚py💻semio🔖domain🔖location](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location)
# Location entity for geographic coordinates with longitude, latitude and altitude.


class LocationGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a location.
    LocationGuidField MUST declare exactly one field with appropriate constraints.
    """

    guid: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class LocationLongitudeField(RealField, abc.ABC):
    """Field mixin for the longitude of a location.
    LocationLongitudeField MUST declare exactly one field with appropriate constraints.
    """

    longitude: float = pydantic.Field()


class LocationLatitudeField(RealField, abc.ABC):
    """Field mixin for the latitude of a location.
    LocationLatitudeField MUST declare exactly one field with appropriate constraints.
    """

    latitude: float = pydantic.Field()


class LocationAltitudeField(RealField, abc.ABC):
    """Field mixin for the altitude of a location.
    LocationAltitudeField MUST declare exactly one field with appropriate constraints.
    """

    altitude: typing.Optional[float] = pydantic.Field(default=None)


class LocationId(LocationGuidField, Id):
    """Identity fields for uniquely identifying a location.
    LocationId MUST contain all fields that uniquely identify a location.
    """



class Location(
    LocationAltitudeField,
    LocationLatitudeField,
    LocationLongitudeField,
    LocationGuidField,
    TableEntity,
):
    """Geographic location with longitude, latitude and altitude.
    Location MUST implement idMembers and inherit from the appropriate field mixins.
    """

    attributes: list[Attribute] = pydantic.Field(default_factory=list)


class LocationInput(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Input):
    """Input fields for creating or updating a location.
    LocationInput MUST contain all fields required for creation.
    """



class LocationOutput(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Output):
    """Output fields returned when fetching a location.
    LocationOutput MUST contain all fields returned on fetch.
    """



class LocationContext(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Context):
    """Context fields for understanding a location by an LLM.
    LocationContext MUST contain all fields needed for LLM understanding.
    """



class LocationPrediction(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Prediction):
    """Prediction fields for LLM-based location inference.
    LocationPrediction MUST contain all fields for LLM inference.
    """



class LocationNode(Node):
    """GraphQL node exposing location data.
    LocationNode MUST expose the model via Meta.
    """

    class Meta:


class LocationInputNode(InputNode):
    """GraphQL input node for location mutations.
    LocationInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Location

# region Author
# [👤semio📚py💻semio🔖domain🔖author](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author)
# Author entity for tracking contributor identity and rank.


class AuthorNameField(RealField, abc.ABC):
    """Field mixin for the name of a author.
    AuthorNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class AuthorEmailField(RealField, abc.ABC):
    """Field mixin for the email of a author.
    AuthorEmailField MUST declare exactly one field with appropriate constraints.
    """

    email: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class AuthorRankField(RealField, abc.ABC):
    """Field mixin for the rank of a author.
    AuthorRankField MUST declare exactly one field with appropriate constraints.
    """

    rank: int = pydantic.Field(default=0)


class AuthorId(AuthorEmailField, Id):
    """Identity fields for uniquely identifying a author.
    AuthorId MUST contain all fields that uniquely identify a author.
    """



class AuthorProps(AuthorEmailField, AuthorNameField, Props):
    """Property fields for a author.
    AuthorProps MUST contain all non-relational property fields.
    """



class AuthorInput(AuthorEmailField, AuthorNameField, Input):
    """Input fields for creating or updating a author.
    AuthorInput MUST contain all fields required for creation.
    """



class AuthorOutput(AuthorEmailField, AuthorNameField, Output):
    """Output fields returned when fetching a author.
    AuthorOutput MUST contain all fields returned on fetch.
    """



class Author(
    AuthorRankField,
    AuthorEmailField,
    AuthorNameField,
    TableEntity,
):
    """Author entity with name, email and contribution rank.
    Author MUST implement idMembers and inherit from the appropriate field mixins.
    """

    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def parent_entity(self) -> "Kit":
        if self.kit is not None:
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:


class AuthorInputNode(InputNode):
    """GraphQL input node for author mutations.
    AuthorInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Author

# region ArtifactAuthor
# [👤semio📚py💻semio🔖domain🔖artifactauthor](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/ArtifactAuthor)
# Artifact-author association entity linking artifacts to authors by email.


class ArtifactAuthorEmailField(RealField, abc.ABC):
    """Field mixin for the email of a artifact author.
    ArtifactAuthorEmailField MUST declare exactly one field with appropriate constraints.
    """

    author_email: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class ArtifactAuthor(ArtifactAuthorEmailField, TableEntity):
    """Association entity linking an artifact to an author by email.
    ArtifactAuthor MUST implement idMembers and inherit from the appropriate field mixins.
    """


    def parent_entity(self) -> typing.Union["Type", "Design", None]:
        if self.type is not None:
        if self.design is not None:
        raise NoTypeOrDesignAssigned()

    def idMembers(self) -> RecursiveAnyList:
            self.author_email,
            self.type.idMembers() if self.type else self.design.idMembers(),
        ]


# endregion ArtifactAuthor

# region File
# [👤semio📚py💻semio🔖domain🔖file](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File)
# File entity for managing binary assets with metadata and hashing.


class FileGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a file.
    FileGuidField MUST declare exactly one field with appropriate constraints.
    """

    guid: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class FileNameField(RealField, abc.ABC):
    """Field mixin for the name of a file.
    FileNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class FileRemoteField(RealField, abc.ABC):
    """Field mixin for the remote of a file.
    FileRemoteField MUST declare exactly one field with appropriate constraints.
    """

    remote: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class FileFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a file.
    FileFolderField MUST declare exactly one field with appropriate constraints.
    """

    folder: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class FileSizeField(RealField, abc.ABC):
    """Field mixin for the size of a file.
    FileSizeField MUST declare exactly one field with appropriate constraints.
    """

    size: typing.Optional[int] = pydantic.Field(default=None)


class FileHashField(RealField, abc.ABC):
    """Field mixin for the hash of a file.
    FileHashField MUST declare exactly one field with appropriate constraints.
    """

    hash: typing.Optional[str] = pydantic.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class FileBlobField(RealField, abc.ABC):
    """Field mixin for the blob of a file.
    FileBlobField MUST declare exactly one field with appropriate constraints.
    """

    blob: typing.Optional[str] = pydantic.Field(default=None)


class FileCreatedAtField(RealField, abc.ABC):
    """Field mixin for the created at of a file.
    FileCreatedAtField MUST declare exactly one field with appropriate constraints.
    """

    createdAt: datetime.datetime = pydantic.Field()


class FileCreatedByField(RealField, abc.ABC):
    """Field mixin for the created by of a file.
    FileCreatedByField MUST declare exactly one field with appropriate constraints.
    """

    createdBy: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FileUpdatedAtField(RealField, abc.ABC):
    """Field mixin for the updated at of a file.
    FileUpdatedAtField MUST declare exactly one field with appropriate constraints.
    """

    updatedAt: datetime.datetime = pydantic.Field()


class FileUpdatedByField(RealField, abc.ABC):
    """Field mixin for the updated by of a file.
    FileUpdatedByField MUST declare exactly one field with appropriate constraints.
    """

    updatedBy: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FileId(FileGuidField, Id):
    """Identity fields for uniquely identifying a file.
    FileId MUST contain all fields that uniquely identify a file.
    """



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
    FileGuidField,
    Props,
):
    """Property fields for a file.
    FileProps MUST contain all non-relational property fields.
    """



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
    FileGuidField,
    Input,
):
    """Input fields for creating or updating a file.
    FileInput MUST contain all fields required for creation.
    """



class FileContext(FileNameField, FileGuidField, Context):
    """Context fields for understanding a file by an LLM.
    FileContext MUST contain all fields needed for LLM understanding.
    """



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
    FileGuidField,
    Output,
):
    """Output fields returned when fetching a file.
    FileOutput MUST contain all fields returned on fetch.
    """



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
    FileGuidField,
    TableEntity,
):
    """File entity for binary assets with metadata, hashing and timestamps.
    File MUST implement idMembers and inherit from the appropriate field mixins.
    """


    def parent_entity(self) -> "Kit":
        if self.kit is not None:
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:


class FileInputNode(InputNode):
    """GraphQL input node for file mutations.
    FileInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion File

# region Folder
# [👤semio📚py💻semio🔖domain🔖folder](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder)
# Folder entity for hierarchical organization of kit content.


class FolderGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a folder.
    FolderGuidField MUST declare exactly one field with appropriate constraints.
    """

    guid: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class FolderNameField(RealField, abc.ABC):
    """Field mixin for the name of a folder.
    FolderNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class FolderParentField(RealField, abc.ABC):
    """Field mixin for the parent of a folder.
    FolderParentField MUST declare exactly one field with appropriate constraints.
    """

    parent: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FolderDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a folder.
    FolderDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class FolderCreatedAtField(RealField, abc.ABC):
    """Field mixin for the created at of a folder.
    FolderCreatedAtField MUST declare exactly one field with appropriate constraints.
    """

    createdAt: datetime.datetime = pydantic.Field()


class FolderCreatedByField(RealField, abc.ABC):
    """Field mixin for the created by of a folder.
    FolderCreatedByField MUST declare exactly one field with appropriate constraints.
    """

    createdBy: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FolderUpdatedAtField(RealField, abc.ABC):
    """Field mixin for the updated at of a folder.
    FolderUpdatedAtField MUST declare exactly one field with appropriate constraints.
    """

    updatedAt: datetime.datetime = pydantic.Field()


class FolderUpdatedByField(RealField, abc.ABC):
    """Field mixin for the updated by of a folder.
    FolderUpdatedByField MUST declare exactly one field with appropriate constraints.
    """

    updatedBy: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FolderId(FolderGuidField, Id):
    """Identity fields for uniquely identifying a folder.
    FolderId MUST contain all fields that uniquely identify a folder.
    """



class FolderProps(
    FolderUpdatedByField,
    FolderUpdatedAtField,
    FolderCreatedByField,
    FolderCreatedAtField,
    FolderDescriptionField,
    FolderParentField,
    FolderNameField,
    FolderGuidField,
    Props,
):
    """Property fields for a folder.
    FolderProps MUST contain all non-relational property fields.
    """



class FolderInput(
    FolderUpdatedByField,
    FolderUpdatedAtField,
    FolderCreatedByField,
    FolderCreatedAtField,
    FolderDescriptionField,
    FolderParentField,
    FolderNameField,
    FolderGuidField,
    Input,
):
    """Input fields for creating or updating a folder.
    FolderInput MUST contain all fields required for creation.
    """

    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class FolderContext(FolderNameField, FolderGuidField, Context):
    """Context fields for understanding a folder by an LLM.
    FolderContext MUST contain all fields needed for LLM understanding.
    """



class FolderOutput(
    FolderUpdatedByField,
    FolderUpdatedAtField,
    FolderCreatedByField,
    FolderCreatedAtField,
    FolderDescriptionField,
    FolderParentField,
    FolderNameField,
    FolderGuidField,
    Output,
):
    """Output fields returned when fetching a folder.
    FolderOutput MUST contain all fields returned on fetch.
    """

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Folder(
    FolderUpdatedByField,
    FolderUpdatedAtField,
    FolderCreatedByField,
    FolderCreatedAtField,
    FolderDescriptionField,
    FolderParentField,
    FolderNameField,
    FolderGuidField,
    TableEntity,
):
    """Folder entity for hierarchical content organization.
    Folder MUST implement idMembers and inherit from the appropriate field mixins.
    """

    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def parent_entity(self) -> "Kit":
        if self.kit is not None:
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:

    def parse(cls, input: str | dict | FolderInput | typing.Any | None) -> "Folder":
        if input is None:
            return cls()
        props = FolderProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            entity.attributes = [typing.cast(Attribute, Attribute.parse(attribute)) for attribute in obj["attributes"]]
        except KeyError:

    def dump(self) -> "FolderOutput":
        entity = {**FolderProps.model_validate(self).model_dump()}
        entity["attributes"] = [q.dump() for q in self.attributes]
        return FolderOutput(**entity)

    def empty(self) -> "Folder":
        props = FolderProps()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.attributes = []

    def update(self, other: "Folder", empty: bool = False) -> "Folder":
        if empty:
            self.empty()
        props = FolderProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)


class FolderInputNode(InputNode):
    """GraphQL input node for folder mutations.
    FolderInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Folder

# region Benchmark
# [👤semio📚py💻semio🔖domain🔖benchmark](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark)
# Benchmark entity for defining performance metrics with min-max bounds.


class BenchmarkNameField(RealField, abc.ABC):
    """Field mixin for the name of a benchmark.
    BenchmarkNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class BenchmarkIconField(RealField, abc.ABC):
    """Field mixin for the icon of a benchmark.
    BenchmarkIconField MUST declare exactly one field with appropriate constraints.
    """

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class BenchmarkMinField(RealField, abc.ABC):
    """Field mixin for the min of a benchmark.
    BenchmarkMinField MUST declare exactly one field with appropriate constraints.
    """

    min: typing.Optional[float] = pydantic.Field(default=None)


class BenchmarkMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a benchmark.
    BenchmarkMinExcludedField MUST declare exactly one field with appropriate constraints.
    """

    min_excluded: bool = pydantic.Field(default=False)


class BenchmarkMaxField(RealField, abc.ABC):
    """Field mixin for the max of a benchmark.
    BenchmarkMaxField MUST declare exactly one field with appropriate constraints.
    """

    max: typing.Optional[float] = pydantic.Field(default=None)


class BenchmarkMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a benchmark.
    BenchmarkMaxExcludedField MUST declare exactly one field with appropriate constraints.
    """

    max_excluded: bool = pydantic.Field(default=False)


class BenchmarkId(BenchmarkNameField, Id):
    """Identity fields for uniquely identifying a benchmark.
    BenchmarkId MUST contain all fields that uniquely identify a benchmark.
    """



class BenchmarkProps(
    BenchmarkMaxExcludedField,
    BenchmarkMaxField,
    BenchmarkMinExcludedField,
    BenchmarkMinField,
    BenchmarkIconField,
    BenchmarkNameField,
    Props,
):
    """Property fields for a benchmark.
    BenchmarkProps MUST contain all non-relational property fields.
    """



class BenchmarkInput(
    BenchmarkMaxExcludedField,
    BenchmarkMaxField,
    BenchmarkMinExcludedField,
    BenchmarkMinField,
    BenchmarkIconField,
    BenchmarkNameField,
    Input,
):
    """Input fields for creating or updating a benchmark.
    BenchmarkInput MUST contain all fields required for creation.
    """



class BenchmarkOutput(
    BenchmarkMaxExcludedField,
    BenchmarkMaxField,
    BenchmarkMinExcludedField,
    BenchmarkMinField,
    BenchmarkIconField,
    BenchmarkNameField,
    Output,
):
    """Output fields returned when fetching a benchmark.
    BenchmarkOutput MUST contain all fields returned on fetch.
    """



class Benchmark(
    BenchmarkMaxExcludedField,
    BenchmarkMaxField,
    BenchmarkMinExcludedField,
    BenchmarkMinField,
    BenchmarkIconField,
    BenchmarkNameField,
    TableEntity,
):
    """Benchmark entity for performance metrics with min-max bounds.
    Benchmark MUST implement idMembers and inherit from the appropriate field mixins.
    """

    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# endregion Benchmark

# region Quality
# [👤semio📚py💻semio🔖domain🔖quality](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality)
# Quality entity for defining measurable properties with units and constraints.


class QualityKeyField(RealField, abc.ABC):
    """Field mixin for the key of a quality.
    QualityKeyField MUST declare exactly one field with appropriate constraints.
    """

    key: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class QualityNameField(RealField, abc.ABC):
    """Field mixin for the name of a quality.
    QualityNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class QualityDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a quality.
    QualityDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class QualityUriField(RealField, abc.ABC):
    """Field mixin for the uri of a quality.
    QualityUriField MUST declare exactly one field with appropriate constraints.
    """

    uri: str = pydantic.Field(default="", max_length=URI_LENGTH_LIMIT)


class QualityScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a quality.
    QualityScalableField MUST declare exactly one field with appropriate constraints.
    """

    scalable: bool = pydantic.Field(default=False)


class QualityKindField(RealField, abc.ABC):
    """Field mixin for the kind of a quality.
    QualityKindField MUST declare exactly one field with appropriate constraints.
    """

    kind: int = pydantic.Field(default=0)


class QualitySiField(RealField, abc.ABC):
    """Field mixin for the si of a quality.
    QualitySiField MUST declare exactly one field with appropriate constraints.
    """

    si: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class QualityImperialField(RealField, abc.ABC):
    """Field mixin for the imperial of a quality.
    QualityImperialField MUST declare exactly one field with appropriate constraints.
    """

    imperial: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class QualityMinField(RealField, abc.ABC):
    """Field mixin for the min of a quality.
    QualityMinField MUST declare exactly one field with appropriate constraints.
    """

    min: typing.Optional[float] = pydantic.Field(default=None)


class QualityMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a quality.
    QualityMinExcludedField MUST declare exactly one field with appropriate constraints.
    """

    min_excluded: bool = pydantic.Field(default=True)


class QualityMaxField(RealField, abc.ABC):
    """Field mixin for the max of a quality.
    QualityMaxField MUST declare exactly one field with appropriate constraints.
    """

    max: typing.Optional[float] = pydantic.Field(default=None)


class QualityMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a quality.
    QualityMaxExcludedField MUST declare exactly one field with appropriate constraints.
    """

    max_excluded: bool = pydantic.Field(default=True)


class QualityDefaultField(RealField, abc.ABC):
    """Field mixin for the default of a quality.
    QualityDefaultField MUST declare exactly one field with appropriate constraints.
    """

    default: typing.Optional[float] = pydantic.Field(default=None)


class QualityFormulaField(RealField, abc.ABC):
    """Field mixin for the formula of a quality.
    QualityFormulaField MUST declare exactly one field with appropriate constraints.
    """

    formula: str = pydantic.Field(default="", max_length=EXPRESSION_LENGTH_LIMIT)


class QualityFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a quality.
    QualityFolderField MUST declare exactly one field with appropriate constraints.
    """

    folder: typing.Optional[str] = pydantic.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class QualityIconField(RealField, abc.ABC):
    """Field mixin for the icon of a quality.
    QualityIconField MUST declare exactly one field with appropriate constraints.
    """

    icon: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class QualityImageField(RealField, abc.ABC):
    """Field mixin for the image of a quality.
    QualityImageField MUST declare exactly one field with appropriate constraints.
    """

    image: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class QualityUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a quality.
    QualityUnitField MUST declare exactly one field with appropriate constraints.
    """

    unit: typing.Optional[str] = pydantic.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class QualityCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a quality.
    QualityCreatedField MUST declare exactly one field with appropriate constraints.
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class QualityUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a quality.
    QualityUpdatedField MUST declare exactly one field with appropriate constraints.
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class QualityId(QualityKeyField, Id):
    """Identity fields for uniquely identifying a quality.
    QualityId MUST contain all fields that uniquely identify a quality.
    """



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
    """Property fields for a quality.
    QualityProps MUST contain all non-relational property fields.
    """



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
    """Input fields for creating or updating a quality.
    QualityInput MUST contain all fields required for creation.
    """



class QualityContext(QualityDescriptionField, QualityNameField, QualityKeyField, Context):
    """Context fields for understanding a quality by an LLM.
    QualityContext MUST contain all fields needed for LLM understanding.
    """



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
    """Output fields returned when fetching a quality.
    QualityOutput MUST contain all fields returned on fetch.
    """

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
    """Quality entity with units, constraints, formula and folder classification.
    Quality MUST implement idMembers and inherit from the appropriate field mixins.
    """


    benchmarks: list["Benchmark"] = pydantic.Field(default_factory=list)
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# endregion Quality

# region Prop
# [👤semio📚py💻semio🔖domain🔖prop](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop)
# Prop entity for key-value property pairs with units.


class PropKeyField(RealField, abc.ABC):
    """Field mixin for the key of a prop.
    PropKeyField MUST declare exactly one field with appropriate constraints.
    """

    key: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class PropValueField(RealField, abc.ABC):
    """Field mixin for the value of a prop.
    PropValueField MUST declare exactly one field with appropriate constraints.
    """

    value: str = pydantic.Field(max_length=VALUE_LENGTH_LIMIT)


class PropUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a prop.
    PropUnitField MUST declare exactly one field with appropriate constraints.
    """

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class PropCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a prop.
    PropCreatedField MUST declare exactly one field with appropriate constraints.
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class PropUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a prop.
    PropUpdatedField MUST declare exactly one field with appropriate constraints.
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class PropId(PropKeyField, Id):
    """Identity fields for uniquely identifying a prop.
    PropId MUST contain all fields that uniquely identify a prop.
    """



class PropProps(
    PropUpdatedField,
    PropCreatedField,
    PropUnitField,
    PropValueField,
    PropKeyField,
    Props,
):
    """Property fields for a prop.
    PropProps MUST contain all non-relational property fields.
    """



class PropInput(PropUnitField, PropValueField, PropKeyField, Input):
    """Input fields for creating or updating a prop.
    PropInput MUST contain all fields required for creation.
    """



class PropOutput(
    PropUpdatedField,
    PropCreatedField,
    PropUnitField,
    PropValueField,
    PropKeyField,
    Output,
):
    """Output fields returned when fetching a prop.
    PropOutput MUST contain all fields returned on fetch.
    """

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Prop(
    PropUpdatedField,
    PropCreatedField,
    PropUnitField,
    PropValueField,
    PropKeyField,
    TableEntity,
):
    """Prop entity for key-value properties with optional units.
    Prop MUST implement idMembers and inherit from the appropriate field mixins.
    """


    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def parent_entity(self) -> typing.Union["Connector", "Type", "Design"]:
        if self.connector is not None:
        if self.type is not None:
        if self.design is not None:
        raise NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned()

    def idMembers(self) -> RecursiveAnyList:

    def parse(cls, input: str | dict | PropInput | typing.Any | None) -> "Prop":
        if input is None:
            return cls()
        props = PropProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            entity.attributes = [typing.cast(Attribute, Attribute.parse(attribute)) for attribute in obj["attributes"]]
        except KeyError:

    def dump(self) -> "PropOutput":
        entity = {**PropProps.model_validate(self).model_dump()}
        entity["attributes"] = [q.dump() for q in self.attributes]
        return PropOutput(**entity)


class PropInputNode(InputNode):
    """GraphQL input node for prop mutations.
    PropInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Prop

# region Model
# [👤semio📚py💻semio🔖domain🔖model](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model)
# Model entity for 3D geometry representations linked to files.


class ModelNameField(RealField, abc.ABC):
    """Field mixin for the name of a model.
    ModelNameField MUST declare exactly one field with appropriate constraints.
    """

    name: typing.Optional[str] = pydantic.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class ModelUrlField(RealField, abc.ABC):
    """Field mixin for the url of a model.
    ModelUrlField MUST declare exactly one field with appropriate constraints.
    """

    url: str = pydantic.Field(max_length=URL_LENGTH_LIMIT)


class ModelFileField(RealField, abc.ABC):
    """Field mixin for the file of a model.
    ModelFileField MUST declare exactly one field with appropriate constraints.
    """

    file: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class ModelDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a model.
    ModelDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ModelTagsField(MaskedField, abc.ABC):
    """Field mixin for the tags of a model.
    ModelTagsField MUST declare exactly one field with appropriate constraints.
    """

    tags: list[str] = pydantic.Field(default_factory=list)


class ModelId(ModelTagsField, Id):
    """Identity fields for uniquely identifying a model.
    ModelId MUST contain all fields that uniquely identify a model.
    """



class ModelProps(
    ModelTagsField,
    ModelDescriptionField,
    ModelNameField,
    ModelFileField,
    ModelUrlField,
    Props,
):
    """Property fields for a model.
    ModelProps MUST contain all non-relational property fields.
    """



class ModelInput(
    ModelTagsField,
    ModelDescriptionField,
    ModelNameField,
    ModelFileField,
    ModelUrlField,
    Input,
):
    """Input fields for creating or updating a model.
    ModelInput MUST contain all fields required for creation.
    """

    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class ModelContext(ModelTagsField, ModelDescriptionField, ModelNameField, Context):
    """Context fields for understanding a model by an LLM.
    ModelContext MUST contain all fields needed for LLM understanding.
    """

    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)


class ModelOutput(
    ModelTagsField,
    ModelDescriptionField,
    ModelNameField,
    ModelFileField,
    ModelUrlField,
    Output,
):
    """Output fields returned when fetching a model.
    ModelOutput MUST contain all fields returned on fetch.
    """

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Model(
    ModelDescriptionField,
    ModelNameField,
    ModelFileField,
    ModelUrlField,
    TableEntity,
):
    """Model entity for 3D geometry with name, URL and file reference.
    Model MUST implement idMembers and inherit from the appropriate field mixins.
    """

    tags_: list[Tag] = pydantic.Field(default_factory=list)
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def tags(self: "Model") -> list[str]:
        return [tag.name for tag in sorted(self.tags_, key=lambda x: x.order)]

    def tags(self: "Model", tags: list[str]):
        self.tags_ = [Tag(name=tag, order=i) for i, tag in enumerate(tags)]

    def parent_entity(self: "Model") -> "Type":
        if self.type is None:
            raise NoTypeAssigned()

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    def parse(cls, input: str | dict | ModelInput | typing.Any | None) -> "Model":
        if input is None:
            return cls(url="", file="")
        props = ModelProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            entity.tags = obj["tags"]
        except KeyError, AttributeError, Exception:
        try:
            entity.attributes = [typing.cast(Attribute, Attribute.parse(attribute)) for attribute in obj["attributes"]]
        except KeyError:

    def dump(self) -> "ModelOutput":
        entity = {**ModelProps.model_validate(self).model_dump()}

        entity["attributes"] = [q.dump() for q in self.attributes]
        return ModelOutput(**entity)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return [self.tags]


class NoModelAssigned(NoParentAssigned):
    """No Model Assigned definition.
    NoModelAssigned MUST fulfill its documented contract.
    """

    def __str__(self):


class ModelInputNode(InputNode):
    """GraphQL input node for model mutations.
    ModelInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Model

# region Port
# [👤semio📚py💻semio🔖domain🔖port](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port)
# Port entity for defining connection interfaces on types.


class PortNameField(RealField, abc.ABC):
    """Field mixin for the name of a port.
    PortNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class PortDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a port.
    PortDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: typing.Optional[str] = pydantic.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)


class PortIconField(RealField, abc.ABC):
    """Field mixin for the icon of a port.
    PortIconField MUST declare exactly one field with appropriate constraints.
    """

    icon: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class PortCompatiblePortsField(MaskedField, abc.ABC):
    """Field mixin for the compatible ports of a port.
    PortCompatiblePortsField MUST declare exactly one field with appropriate constraints.
    """

    compatiblePorts: list[str] = pydantic.Field(default_factory=list)


class PortId(PortNameField, Id):
    """Identity fields for uniquely identifying a port.
    PortId MUST contain all fields that uniquely identify a port.
    """



class PortProps(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Props):
    """Property fields for a port.
    PortProps MUST contain all non-relational property fields.
    """



class PortInput(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Input):
    """Input fields for creating or updating a port.
    PortInput MUST contain all fields required for creation.
    """

    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class PortOutput(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Output):
    """Output fields returned when fetching a port.
    PortOutput MUST contain all fields returned on fetch.
    """

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Port(PortIconField, PortDescriptionField, PortNameField, TableEntity):
    """Port entity defining a named connection interface on a type.
    Port MUST implement idMembers and inherit from the appropriate field mixins.
    """

    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# TODO: Fix PortNode - was incorrectly changed to TableEntityNode in latest commit


class PortInputNode(InputNode):
    """GraphQL input node for port mutations.
    PortInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Port

# region Connector
# [🔖semio/py/semio.py#Connector](repo://section/semio/py/semio.py/CONNECTOR)

# region CompatiblePort
# [👤semio📚py💻semio🔖domain🔖connector🔖compatibleport](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/s/CompatiblePort)
# Compatible port entity for specifying allowed port pairings on connectors.


class CompatiblePortNameField(RealField, abc.ABC):
    """Field mixin for the name of a compatible port.
    CompatiblePortNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class CompatiblePortOrderField(RealField, abc.ABC):
    """Field mixin for the order of a compatible port.
    CompatiblePortOrderField MUST declare exactly one field with appropriate constraints.
    """

    order: int = pydantic.Field()


class CompatiblePort(CompatiblePortOrderField, CompatiblePortNameField, Table):
    """Compatible port entity specifying an allowed port pairing.
    CompatiblePort MUST implement idMembers and inherit from the appropriate field mixins.
    """


# endregion CompatiblePort


class ConnectorIdField(MaskedField, abc.ABC):
    """Field mixin for the id of a connector.
    ConnectorIdField MUST declare exactly one field with appropriate constraints.
    """

    id_: str = pydantic.Field(default="", max_length=ID_LENGTH_LIMIT)


class ConnectorDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a connector.
    ConnectorDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ConnectorMandatoryField(RealField, abc.ABC):
    """Field mixin for the mandatory of a connector.
    ConnectorMandatoryField MUST declare exactly one field with appropriate constraints.
    """

    is_mandatory: bool = pydantic.Field(default=False)


class ConnectorPortField(RealField, abc.ABC):
    """Field mixin for the port of a connector.
    ConnectorPortField MUST declare exactly one field with appropriate constraints.
    """

    port: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class ConnectorCompatiblePortsField(MaskedField, abc.ABC):
    """Field mixin for the compatible ports of a connector.
    ConnectorCompatiblePortsField MUST declare exactly one field with appropriate constraints.
    """

    compatiblePorts: list[str] = pydantic.Field(default_factory=list)


class ConnectorPointField(MaskedField, abc.ABC):
    """Field mixin for the point of a connector.
    ConnectorPointField MUST declare exactly one field with appropriate constraints.
    """

    point: Point = pydantic.Field()


class ConnectorDirectionField(MaskedField, abc.ABC):
    """Field mixin for the direction of a connector.
    ConnectorDirectionField MUST declare exactly one field with appropriate constraints.
    """

    direction: Vector = pydantic.Field()


class ConnectorTField(RealField, abc.ABC):
    """Field mixin for the t of a connector.
    ConnectorTField MUST declare exactly one field with appropriate constraints.
    """

    t: float = pydantic.Field(default=0.0)


class ConnectorId(ConnectorIdField, Id):
    """Identity fields for uniquely identifying a connector.
    ConnectorId MUST contain all fields that uniquely identify a connector.
    """



class ConnectorProps(
    ConnectorTField,
    ConnectorCompatiblePortsField,
    ConnectorPortField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    ConnectorIdField,
    Props,
):
    """Property fields for a connector.
    ConnectorProps MUST contain all non-relational property fields.
    """



class ConnectorInput(
    ConnectorTField,
    ConnectorCompatiblePortsField,
    ConnectorPortField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    ConnectorIdField,
    Input,
):
    """Input fields for creating or updating a connector.
    ConnectorInput MUST contain all fields required for creation.
    """

    point: PointInput = pydantic.Field()
    direction: VectorInput = pydantic.Field()
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class ConnectorContext(
    ConnectorTField,
    ConnectorDirectionField,
    ConnectorPointField,
    ConnectorCompatiblePortsField,
    ConnectorPortField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    ConnectorIdField,
    Context,
):
    """Context fields for understanding a connector by an LLM.
    ConnectorContext MUST contain all fields needed for LLM understanding.
    """

    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)


class ConnectorOutput(
    ConnectorTField,
    ConnectorDirectionField,
    ConnectorPointField,
    ConnectorCompatiblePortsField,
    ConnectorPortField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    ConnectorIdField,
    Output,
):
    """Output fields returned when fetching a connector.
    ConnectorOutput MUST contain all fields returned on fetch.
    """

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Connector(
    ConnectorTField,
    ConnectorPortField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    TableEntity,
):
    """Connector entity defining a localized connection point on a type.
    Connector MUST implement idMembers and inherit from the appropriate field mixins.
    """


    compatiblePorts_: list[CompatiblePort] = pydantic.Field(default_factory=list)
    attributes: list["Attribute"] = pydantic.Field(default_factory=list)
    props: list["Prop"] = pydantic.Field(default_factory=list)
    connecteds: list["Connection"] = pydantic.Field(default_factory=list)
    connectings: list["Connection"] = pydantic.Field(default_factory=list)

    def compatiblePorts(self) -> list[str]:
        return sorted([cf.name for cf in self.compatiblePorts_], key=lambda cf: cf.order)

    def compatiblePorts(self, compatiblePorts: list[str]):
        self.compatiblePorts_ = [CompatiblePort(name=cf, order=i) for i, cf in enumerate(compatiblePorts)]

    def point(self) -> Point:
        return Point(x=self.pointX, y=self.pointY, z=self.pointZ)

    def point(self, point: Point):

    def direction(self) -> Vector:
        return Vector(x=self.directionX, y=self.directionY, z=self.directionZ)

    def direction(self, direction: Vector):

    def connections(self) -> list["Connection"]:

    def parent_entity(self) -> "Type":
        if self.type is None:
            raise NoTypeAssigned()

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    def parse(cls, input: str | dict | ConnectorInput | typing.Any | None) -> "Connector":
        if input is None:
            return cls()
        port_obj = obj.get("port")
            id_=obj.get("id_", obj.get("name", "")),
            description=obj.get("description", ""),
            is_mandatory=obj.get("mandatory", False),
            port=port_guid,
            t=obj.get("t", 0.0),
        )
        point = Point.parse(obj["point"])
        direction = Vector.parse(obj["direction"])
        try:
            entity.compatiblePorts = obj["compatiblePorts"]
        except KeyError:
        try:
            attrs = [Attribute.parse(attr) for attr in obj.get("attributes", [])]
            if attrs:
        except KeyError:

    def dump(self) -> "ConnectorOutput":
        entity = {**ConnectorProps.model_validate(self).model_dump()}
        entity["point"] = self.point.dump()
        entity["direction"] = self.direction.dump()
        entity["attributes"] = [q.dump() for q in self.attributes]
        return ConnectorOutput(**entity)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:


class ConnectorNotFound(NotFound):
    """Exception for a connector not found on a type.
    ConnectorNotFound MUST provide a descriptive error message via __str__.
    """

    def __init__(self, parent: "Type", id: "ConnectorId") -> None:

    def __str__(self):


class ConnectorInputNode(InputNode):
    """GraphQL input node for connector mutations.
    ConnectorInputNode MUST expose the input model via Meta.
    """

    class Meta:


class ConnectorIdInputNode(InputNode):
    """GraphQL input node for connector id mutations.
    ConnectorIdInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Connector

# region Type
# [👤semio📚py💻semio🔖domain🔖type](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type)
# Type entity for defining reusable parametric building blocks.


class TypeNameField(RealField, abc.ABC):
    """Field mixin for the name of a type.
    TypeNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class TypeDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a type.
    TypeDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class TypeIconField(RealField, abc.ABC):
    """Field mixin for the icon of a type.
    TypeIconField MUST declare exactly one field with appropriate constraints.
    """

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class TypeImageField(RealField, abc.ABC):
    """Field mixin for the image of a type.
    TypeImageField MUST declare exactly one field with appropriate constraints.
    """

    image: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class TypeParentField(RealField, abc.ABC):
    """Field mixin for the parent of a type.
    TypeParentField MUST declare exactly one field with appropriate constraints.
    """

    parent: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class TypeIsAbstractField(RealField, abc.ABC):
    """Field mixin for the is abstract of a type.
    TypeIsAbstractField MUST declare exactly one field with appropriate constraints.
    """

    is_abstract: bool = pydantic.Field(default=False)


class TypeFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a type.
    TypeFolderField MUST declare exactly one field with appropriate constraints.
    """

    folder: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class TypeStockField(RealField, abc.ABC):
    """Field mixin for the stock of a type.
    TypeStockField MUST declare exactly one field with appropriate constraints.
    """

    stock: int = pydantic.Field(default=2147483647)


class TypeVariantField(RealField, abc.ABC):
    """Field mixin for the variant of a type.
    TypeVariantField MUST declare exactly one field with appropriate constraints.
    """

    variant: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class TypeVirtualField(RealField, abc.ABC):
    """Field mixin for the virtual of a type.
    TypeVirtualField MUST declare exactly one field with appropriate constraints.
    """

    is_virtual: bool = pydantic.Field(default=False)


class TypeScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a type.
    TypeScalableField MUST declare exactly one field with appropriate constraints.
    """

    can_scale: bool = pydantic.Field(default=True)


class TypeMirrborableField(RealField, abc.ABC):
    """Field mixin for the mirrborable of a type.
    TypeMirrborableField MUST declare exactly one field with appropriate constraints.
    """

    can_mirror: bool = pydantic.Field(default=True)


class TypeUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a type.
    TypeUnitField MUST declare exactly one field with appropriate constraints.
    """

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class TypeLocationField(MaskedField, abc.ABC):
    """Field mixin for the location of a type.
    TypeLocationField MUST declare exactly one field with appropriate constraints.
    """

    location: typing.Optional[Location] = pydantic.Field(default=None)


class TypeCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a type.
    TypeCreatedField MUST declare exactly one field with appropriate constraints.
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class TypeUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a type.
    TypeUpdatedField MUST declare exactly one field with appropriate constraints.
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class TypeId(TypeNameField, TypeVariantField, Id):
    """Identity fields for uniquely identifying a type.
    TypeId MUST contain all fields that uniquely identify a type.
    """



class TypeProps(
    TypeUnitField,
    TypeLocationField,
    TypeFolderField,
    TypeIsAbstractField,
    TypeParentField,
    TypeVirtualField,
    TypeStockField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Props,
):
    """Property fields for a type.
    TypeProps MUST contain all non-relational property fields.
    """



class TypeInput(
    TypeUnitField,
    TypeVirtualField,
    TypeStockField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Input,
):
    """Input fields for creating or updating a type.
    TypeInput MUST contain all fields required for creation.
    """

    parent: typing.Optional[str] = pydantic.Field(default=None)
    is_abstract: typing.Optional[bool] = pydantic.Field(default=None)
    folder: typing.Optional[str] = pydantic.Field(default=None)
    location: typing.Optional[LocationInput] = pydantic.Field(default=None)
    models: list[ModelInput] = pydantic.Field(default_factory=list)
    connectors: list[ConnectorInput] = pydantic.Field(default_factory=list)
    props: list[PropInput] = pydantic.Field(default_factory=list)
    authors: list[str] = pydantic.Field(default_factory=list)
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


class TypeOutput(
    TypeUpdatedField,
    TypeCreatedField,
    TypeUnitField,
    TypeVirtualField,
    TypeStockField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Output,
):
    """Output fields returned when fetching a type.
    TypeOutput MUST contain all fields returned on fetch.
    """

    parent: typing.Optional[str] = pydantic.Field(default=None)
    is_abstract: typing.Optional[bool] = pydantic.Field(default=None)
    folder: typing.Optional[str] = pydantic.Field(default=None)
    location: typing.Optional[LocationOutput] = pydantic.Field(default=None)
    models: list[ModelOutput] = pydantic.Field(default_factory=list)
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
    """Context fields for understanding a type by an LLM.
    TypeContext MUST contain all fields needed for LLM understanding.
    """

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
    TableEntity,
):
    """Type entity defining a reusable parametric building block.
    Type MUST implement idMembers and inherit from the appropriate field mixins.
    """


    models: list[Model] = pydantic.Field(default_factory=list)

    connectors: list[Connector] = pydantic.Field(default_factory=list)

    props: list["Prop"] = pydantic.Field(default_factory=list)

    artifact_authors: list[ArtifactAuthor] = pydantic.Field(default_factory=list)

    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    pieces: list["Piece"] = pydantic.Field(default_factory=list)

    concepts_: list[Concept] = pydantic.Field(default_factory=list)

    def location(self) -> typing.Optional[Location]:
        if self.locationLongitude is None and self.locationLatitude is None:
        if self.locationLongitude is None:
            raise ValueError("Location longitude is required")
        if self.locationLatitude is None:
            raise ValueError("Location latitude is required")
            longitude=self.locationLongitude,
            latitude=self.locationLatitude,
        )

    def location(self, location: typing.Optional[Location]):
        if location is None:
        else:

    def authors(self) -> list[str]:
        return [artifact_author.author_email for artifact_author in self.artifact_authors]

    def authors(self, author_emails: list[str]):
        self.artifact_authors = [ArtifactAuthor(author_email=email) for email in author_emails]

    def concepts(self: "Type") -> list[str]:
        return [concept.name for concept in sorted(self.concepts_, key=lambda x: x.order)]

    def concepts(self: "Type", concepts: list[str]):
        self.concepts_ = [Concept(name=concept, order=i) for i, concept in enumerate(concepts)]

    def parent_entity(self) -> "Kit":
        if self.kit is None:
            raise NoKitAssigned()

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    def parse(cls, input: str | dict | TypeInput | typing.Any | None) -> "Type":
        if input is None:
            return cls()
        parent_obj = obj.get("parent")
        folder_obj = obj.get("folder")
            name=obj.get("name", ""),
            variant=obj.get("variant", ""),
            description=obj.get("description", ""),
            icon=obj.get("icon", ""),
            image=obj.get("image", ""),
            isAbstract=obj.get("isAbstract", False),
            isVirtual=obj.get("isVirtual", False),
            stock=2147483647 if obj.get("stock") is None else obj.get("stock"),
            unit=obj.get("unit", ""),
            parent=parent_guid,
            folder=folder_guid,
        )
        try:
            location_obj = obj.get("location")
            if location_obj:
        except KeyError, AttributeError:
        try:
            models = [Model.parse(r) for r in obj["models"]]
        except KeyError, AttributeError, Exception:
        try:
            connectors = [Connector.parse(p) for p in obj["connectors"]]
        except KeyError, AttributeError, Exception:
        try:
            props = [Prop.parse(p) for p in obj["props"]]
        except KeyError, AttributeError, Exception:
        try:
            entity.attributes = [Attribute.parse(q) for q in obj["attributes"]]
        except KeyError, AttributeError, Exception:
        try:
            author_emails = obj["authors"]
        except KeyError, AttributeError, Exception:
        try:
            concepts = obj["concepts"]
        except KeyError, AttributeError, Exception:


    def dump(self) -> "TypeOutput":
        entity = {**TypeProps.model_validate(self).model_dump()}
        entity["models"] = [r.dump() for r in self.models]
        entity["connectors"] = [p.dump() for p in self.connectors]
        entity["props"] = [p.dump() for p in self.props]
        entity["attributes"] = [q.dump() for q in self.attributes]
        return TypeOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        props = TypeProps()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.types = []

    # TODO: Automatic updating based on props.
    def update(self, other: "Type", empty: bool = False) -> "Type":
        if empty:
            self.empty()
        props = TypeProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return [self.name, self.variant]


class TypeNotFound(NotFound):
    """Exception for a type not found in the kit.
    TypeNotFound MUST provide a descriptive error message via __str__.
    """

    def __init__(self, id: "TypeId") -> None:

    def __str__(self):


class NoTypeAssigned(NoParentAssigned):
    """No Type Assigned definition.
    NoTypeAssigned MUST fulfill its documented contract.
    """

    def __str__(self):


class TypeHasNotAllUsedConnectors(SpecificationError):
    """Type Has Not All Used Connectors definition.
    TypeHasNotAllUsedConnectors MUST fulfill its documented contract.
    """

    def __init__(self, missingConnectors: set[str]) -> None:

    def __str__(self) -> str:


class TypeInputNode(InputNode):
    """GraphQL input node for type mutations.
    TypeInputNode MUST expose the input model via Meta.
    """

    class Meta:


class TypeIdInputNode(InputNode):
    """GraphQL input node for type id mutations.
    TypeIdInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Type

# region Layer
# [👤semio📚py💻semio🔖domain🔖layer](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer)
# Layer entity for organizing design elements into visibility groups.


class LayerNameField(RealField, abc.ABC):
    """Field mixin for the name of a layer.
    LayerNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class LayerDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a layer.
    LayerDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class LayerColorField(RealField, abc.ABC):
    """Field mixin for the color of a layer.
    LayerColorField MUST declare exactly one field with appropriate constraints.
    """

    color: str = pydantic.Field(default="", max_length=7)


class LayerIsHiddenField(RealField, abc.ABC):
    """Field mixin for the is hidden of a layer.
    LayerIsHiddenField MUST declare exactly one field with appropriate constraints.
    """

    is_hidden: bool = pydantic.Field(default=False)


class LayerIsLockedField(RealField, abc.ABC):
    """Field mixin for the is locked of a layer.
    LayerIsLockedField MUST declare exactly one field with appropriate constraints.
    """

    is_locked: bool = pydantic.Field(default=False)


class LayerId(LayerNameField, Id):
    """Identity fields for uniquely identifying a layer.
    LayerId MUST contain all fields that uniquely identify a layer.
    """



class LayerProps(
    LayerIsLockedField,
    LayerIsHiddenField,
    LayerColorField,
    LayerDescriptionField,
    LayerNameField,
    Props,
):
    """Property fields for a layer.
    LayerProps MUST contain all non-relational property fields.
    """



class LayerInput(
    LayerIsLockedField,
    LayerIsHiddenField,
    LayerColorField,
    LayerDescriptionField,
    LayerNameField,
    Input,
):
    """Input fields for creating or updating a layer.
    LayerInput MUST contain all fields required for creation.
    """



class LayerOutput(
    LayerIsLockedField,
    LayerIsHiddenField,
    LayerColorField,
    LayerDescriptionField,
    LayerNameField,
    Output,
):
    """Output fields returned when fetching a layer.
    LayerOutput MUST contain all fields returned on fetch.
    """



class Layer(
    LayerIsLockedField,
    LayerIsHiddenField,
    LayerColorField,
    LayerDescriptionField,
    LayerNameField,
    TableEntity,
):
    """Layer entity for grouping design elements with visibility and locking.
    Layer MUST implement idMembers and inherit from the appropriate field mixins.
    """

    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# endregion Layer

# region Piece
# [👤semio📚py💻semio🔖domain🔖piece](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece)
# Piece entity for placed instances of types within a design.


class PieceIdField(MaskedField, abc.ABC):
    """Field mixin for the id of a piece.
    PieceIdField MUST declare exactly one field with appropriate constraints.
    """

        default="",
        max_length=ID_LENGTH_LIMIT,
    )


class PieceDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a piece.
    PieceDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class PieceTypeField(MaskedField, abc.ABC):
    """Field mixin for the type of a piece.
    PieceTypeField MUST declare exactly one field with appropriate constraints.
    """

    type: typing.Optional[TypeId] = pydantic.Field(default=None)


class PieceDesignField(MaskedField, abc.ABC):
    """Field mixin for the design of a piece.
    PieceDesignField MUST declare exactly one field with appropriate constraints.
    """

    designPiece: typing.Optional["DesignId"] = pydantic.Field(default=None)


class PiecePlaneField(MaskedField, abc.ABC):
    """Field mixin for the plane of a piece.
    PiecePlaneField MUST declare exactly one field with appropriate constraints.
    """

    plane: typing.Optional[Plane] = pydantic.Field(default=None)


class PieceCenterField(MaskedField, abc.ABC):
    """Field mixin for the center of a piece.
    PieceCenterField MUST declare exactly one field with appropriate constraints.
    """

    center: typing.Optional[Coord] = pydantic.Field(default=None)


class PieceScaleField(RealField, abc.ABC):
    """Field mixin for the scale of a piece.
    PieceScaleField MUST declare exactly one field with appropriate constraints.
    """

    scale: float = pydantic.Field(default=1.0)


class PieceMirrorPlaneField(MaskedField, abc.ABC):
    """Field mixin for the mirror plane of a piece.
    PieceMirrorPlaneField MUST declare exactly one field with appropriate constraints.
    """

    mirrorPlane: typing.Optional[Plane] = pydantic.Field(default=None)


class PieceHiddenField(RealField, abc.ABC):
    """Field mixin for the hidden of a piece.
    PieceHiddenField MUST declare exactly one field with appropriate constraints.
    """

    is_hidden: bool = pydantic.Field(default=False)


class PieceLockedField(RealField, abc.ABC):
    """Field mixin for the locked of a piece.
    PieceLockedField MUST declare exactly one field with appropriate constraints.
    """

    is_locked: bool = pydantic.Field(default=False)


class PieceColorField(RealField, abc.ABC):
    """Field mixin for the color of a piece.
    PieceColorField MUST declare exactly one field with appropriate constraints.
    """

    color: str = pydantic.Field(default="", max_length=7)


class PieceId(PieceIdField, Id):
    """Identity fields for uniquely identifying a piece.
    PieceId MUST contain all fields that uniquely identify a piece.
    """



class PieceProps(
    PieceCenterField,
    PiecePlaneField,
    PieceDesignField,
    PieceTypeField,
    PieceDescriptionField,
    PieceIdField,
    Props,
):
    """Property fields for a piece.
    PieceProps MUST contain all non-relational property fields.
    """



class PieceInput(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Input):
    """Input fields for creating or updating a piece.
    PieceInput MUST contain all fields required for creation.
    """

    plane: typing.Optional[PlaneInput] = pydantic.Field(default=None)
    center: typing.Optional[CoordInput] = pydantic.Field(default=None)
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class PieceContext(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Context):
    """Context fields for understanding a piece by an LLM.
    PieceContext MUST contain all fields needed for LLM understanding.
    """

    plane: typing.Optional[PlaneContext] = pydantic.Field(default=None)
    center: typing.Optional[CoordContext] = pydantic.Field(default=None)
    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)


class PieceOutput(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Output):
    """Output fields returned when fetching a piece.
    PieceOutput MUST contain all fields returned on fetch.
    """

    plane: typing.Optional[PlaneOutput] = pydantic.Field(default=None)
    center: typing.Optional[CoordOutput] = pydantic.Field(default=None)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class PiecePrediction(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Prediction):
    """Prediction fields for LLM-based piece inference.
    PiecePrediction MUST contain all fields for LLM inference.
    """



class Piece(
    PieceIdField,
    PieceHiddenField,
    PieceLockedField,
    PieceColorField,
    PieceScaleField,
    TableEntity,
):
    """Piece entity for a placed instance of a type within a design.
    Piece MUST implement idMembers and inherit from the appropriate field mixins.
    """

    attributes: list[Attribute] = pydantic.Field(default_factory=list)
    connecteds: list["Connection"] = pydantic.Field(default_factory=list)
    connectings: list["Connection"] = pydantic.Field(default_factory=list)

    def center(self) -> typing.Optional[Coord]:
        if self.centerU is None or self.centerV is None:
        return Coord(u=self.centerU, v=self.centerV)

    def center(self, center: typing.Optional[Coord]):
        if center is None:

    def connections(self) -> list["Connection"]:

    def parent_entity(self) -> "Design":
        if self.design is None:
            raise NoParentAssigned()

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    def parse(
        cls: "Piece",
        input: str | dict | PieceInput | typing.Any | None,
        types: dict[str, dict[str, Type]],
        designs: typing.Optional[dict[str, dict[str, dict[str, "Design"]]]] = None,
    ) -> "Piece":
        if input is None:
            return cls()
        piece_id = obj.get("id_", obj.get("guid", ""))
        entity = cls(id_=piece_id)
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
        except KeyError:
        try:
            if obj["center"] is not None:
                center = Coord.parse(obj["center"])
        except KeyError:

    def dump(self) -> "PieceOutput":
        entity = {**PieceProps.model_validate(self).model_dump()}
        entity["attributes"] = [q.dump() for q in self.attributes]
        return PieceOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Piece":
        props = PieceProps()
        for key, value in props.model_dump().items():
            setattr(self, key, value)

    # TODO: Automatic updating based on props.
    def update(self, other: "Piece", empty: bool = False) -> "Piece":
        if empty:
            self.empty()
        props = PieceProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:


class PieceInputNode(InputNode):
    """GraphQL input node for piece mutations.
    PieceInputNode MUST expose the input model via Meta.
    """

    class Meta:
        exclude_fields = ("type", "designPiece")

    type = TypeIdInputNode()
    designPiece = graphene.Field(lambda: DesignIdInputNode)


class PieceIdInputNode(InputNode):
    """GraphQL input node for piece id mutations.
    PieceIdInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Piece

# region Group
# [👤semio📚py💻semio🔖domain🔖group](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Group)
# Group entity for named collections of pieces in a design.


class GroupNameField(RealField, abc.ABC):
    """Field mixin for the name of a group.
    GroupNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class GroupDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a group.
    GroupDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class GroupColorField(RealField, abc.ABC):
    """Field mixin for the color of a group.
    GroupColorField MUST declare exactly one field with appropriate constraints.
    """

    color: str = pydantic.Field(default="", max_length=7)


class GroupId(GroupNameField, Id):
    """Identity fields for uniquely identifying a group.
    GroupId MUST contain all fields that uniquely identify a group.
    """



class GroupProps(GroupColorField, GroupDescriptionField, GroupNameField, Props):
    """Property fields for a group.
    GroupProps MUST contain all non-relational property fields.
    """



class GroupInput(GroupColorField, GroupDescriptionField, GroupNameField, Input):
    """Input fields for creating or updating a group.
    GroupInput MUST contain all fields required for creation.
    """



class GroupOutput(GroupColorField, GroupDescriptionField, GroupNameField, Output):
    """Output fields returned when fetching a group.
    GroupOutput MUST contain all fields returned on fetch.
    """

    pieces: list["PieceOutput"] = pydantic.Field(default_factory=list)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Group(GroupColorField, GroupDescriptionField, GroupNameField, TableEntity):
    """Group entity for named collections of pieces.
    Group MUST implement idMembers and inherit from the appropriate field mixins.
    """



# endregion Group

# region Side
# [👤semio📚py💻semio🔖domain🔖side](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Side)
# Side primitive for identifying a specific connector on a specific piece.


class Side(BaseModel):
    """Side primitive identifying a specific connector on a specific piece.
    Side MUST contain all coordinate or geometry fields.
    """

    piece: PieceId = pydantic.Field()
    designPiece: typing.Optional[PieceId] = pydantic.Field(default=None)
    connector: typing.Optional[ConnectorId] = pydantic.Field(default=None)

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    def parse(cls: "Side", input: str | dict | typing.Any | None) -> "Side":
        if input is None:
            return cls()
        piece = PieceId.parse(obj["piece"])
        try:
            connectorObj = obj.get("connector")
        except KeyError, TypeError:
        try:
            designPieceObj = obj.get("designPiece")
        except KeyError, TypeError:
        return cls(piece=piece, designPiece=designPiece, connector=connector)


class SideInput(Side, Input):
    """Input fields for creating or updating a side.
    SideInput MUST contain all fields required for creation.
    """



class SideContext(Side, Context):
    """Context fields for understanding a side by an LLM.
    SideContext MUST contain all fields needed for LLM understanding.
    """



class SideOutput(Side, Output):
    """Output fields returned when fetching a side.
    SideOutput MUST contain all fields returned on fetch.
    """



class SidePrediction(Side, Prediction):
    """Prediction fields for LLM-based side inference.
    SidePrediction MUST contain all fields for LLM inference.
    """



class SideNode(Node):
    """GraphQL node exposing side data.
    SideNode MUST expose the model via Meta.
    """

    class Meta:

    exclude_fields = ("piece", "connector")

    piece = graphene.NonNull(lambda: PieceNode)
    designPiece = graphene.Field(lambda: PieceNode)
    connector = graphene.Field(lambda: ConnectorNode)

    def resolve_piece(self, info):

    def resolve_designPiece(self, info):

    def resolve_connector(self, info):


class SideInputNode(InputNode):
    """GraphQL input node for side mutations.
    SideInputNode MUST expose the input model via Meta.
    """

    class Meta:

    exclude_fields = ("piece", "connector")

    piece = graphene.NonNull(PieceIdInputNode)
    designPiece = PieceIdInputNode()
    connector = ConnectorIdInputNode()


# endregion Side

# region Connection
# [👤semio📚py💻semio🔖domain🔖connection](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection)
# Connection entity for linking two pieces through their connectors.


class ConnectionConnectedField(MaskedField, abc.ABC):
    """Field mixin for the connected of a connection.
    ConnectionConnectedField MUST declare exactly one field with appropriate constraints.
    """

    connected: Side = pydantic.Field()


class ConnectionConnectingField(MaskedField, abc.ABC):
    """Field mixin for the connecting of a connection.
    ConnectionConnectingField MUST declare exactly one field with appropriate constraints.
    """

    connecting: Side = pydantic.Field()


class ConnectionDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a connection.
    ConnectionDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ConnectionGapField(RealField, abc.ABC):
    """Field mixin for the gap of a connection.
    ConnectionGapField MUST declare exactly one field with appropriate constraints.
    """

    gap: float = pydantic.Field(default=0)


class ConnectionShiftField(RealField, abc.ABC):
    """Field mixin for the shift of a connection.
    ConnectionShiftField MUST declare exactly one field with appropriate constraints.
    """

    shift: float = pydantic.Field(default=0)


class ConnectionRiseField(MaskedField, abc.ABC):
    """Field mixin for the rise of a connection.
    ConnectionRiseField MUST declare exactly one field with appropriate constraints.
    """

    rise: float = pydantic.Field(default=0)


class ConnectionRotationField(RealField, abc.ABC):
    """Field mixin for the rotation of a connection.
    ConnectionRotationField MUST declare exactly one field with appropriate constraints.
    """

    rotation: float = pydantic.Field(ge=0, lt=360, default=0)


class ConnectionTurnField(RealField, abc.ABC):
    """Field mixin for the turn of a connection.
    ConnectionTurnField MUST declare exactly one field with appropriate constraints.
    """

    turn: float = pydantic.Field(ge=0, lt=360, default=0)


class ConnectionTiltField(RealField, abc.ABC):
    """Field mixin for the tilt of a connection.
    ConnectionTiltField MUST declare exactly one field with appropriate constraints.
    """

    tilt: float = pydantic.Field(ge=0, lt=360, default=0)


class ConnectionUField(RealField, abc.ABC):
    """Field mixin for the u of a connection.
    ConnectionUField MUST declare exactly one field with appropriate constraints.
    """

    u: float = pydantic.Field(default=0)


class ConnectionVField(RealField, abc.ABC):
    """Field mixin for the v of a connection.
    ConnectionVField MUST declare exactly one field with appropriate constraints.
    """

    v: float = pydantic.Field(default=0)


class ConnectionId(ConnectionConnectedField, ConnectionConnectingField, Id):
    """Identity fields for uniquely identifying a connection.
    ConnectionId MUST contain all fields that uniquely identify a connection.
    """



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
    """Property fields for a connection.
    ConnectionProps MUST contain all non-relational property fields.
    """



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
    """Input fields for creating or updating a connection.
    ConnectionInput MUST contain all fields required for creation.
    """


    connected: SideInput = pydantic.Field()
    connecting: SideInput = pydantic.Field()


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
    """Context fields for understanding a connection by an LLM.
    ConnectionContext MUST contain all fields needed for LLM understanding.
    """


    connected: SideContext = pydantic.Field()
    connecting: SideContext = pydantic.Field()


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
    """Output fields returned when fetching a connection.
    ConnectionOutput MUST contain all fields returned on fetch.
    """


    connected: SideOutput = pydantic.Field()
    connecting: SideOutput = pydantic.Field()


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
    """Prediction fields for LLM-based connection inference.
    ConnectionPrediction MUST contain all fields for LLM inference.
    """


    connected: SidePrediction = pydantic.Field()
    connecting: SidePrediction = pydantic.Field()


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
    """Connection entity linking two pieces through their connectors.
    Connection MUST implement idMembers and inherit from the appropriate field mixins.
    """


    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def connected(self) -> Side:
            piece=self.connectedPiece,
            designPiece=(PieceId(id_=self.connectedDesignPiece.id_) if self.connectedDesignPiece is not None else None),
            connector=self.connectedConnector,
        )

    def connecting(self) -> Side:
            piece=self.connectingPiece,
            designPiece=(PieceId(id_=self.connectingDesignPiece.id_) if self.connectingDesignPiece is not None else None),
            connector=self.connectingConnector,
        )

    def parent_entity(self) -> "Design":
        if self.design is None:
            raise NoDesignAssigned()

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    def parse(
        cls: "Connection",
        input: str | dict | ConnectionInput | typing.Any | None,
        pieces: list[Piece],
        designsById: typing.Optional[dict[str, dict[str, dict[str, Design]]]] = None,
    ) -> "Connection":
        if input is None:
            return cls()
        piecesDict = {p.id_: p for p in pieces}
        connected = Side.parse(obj["connected"])
        connecting = Side.parse(obj["connecting"])
        connectedPiece = piecesDict[connected.piece.id_]
        if connectedType is None:
            raise FeatureNotYetSupported()
        if connected.connector is not None:
            connectedConnectorList = [p for p in connectedType.connectors if p.id_ == connected.connector.id_]
            if len(connectedConnectorList) == 0:
                raise ConnectorNotFound(connectedType, connected.connector)
            else:
                connectedConnector = connectedConnectorList[0]
        connectingPiece = piecesDict[connecting.piece.id_]
        if connectingType is None:
            raise FeatureNotYetSupported()
        if connecting.connector is not None:
            connectingConnectorList = [p for p in connectingType.connectors if p.id_ == connecting.connector.id_]
            if len(connectingConnectorList) == 0:
                raise ConnectorNotFound(connectingType, connecting.connector)
            else:
                connectingConnector = connectingConnectorList[0]
            connectedPiece=connectedPiece,
            connectedConnector=connectedConnector,
            connectingPiece=connectingPiece,
            connectingConnector=connectingConnector,
        )
        if connected.designPiece is not None:
            if connectedPiece.refDesign is None and designsById is None:
                raise FeatureNotYetSupported()
            if refDesign is None and designsById is not None:
                raise FeatureNotYetSupported()
            if refDesign is not None:
                try:
                    designPiece = next(p for p in refDesign.pieces if p.id_ == connected.designPiece.id_)
                except StopIteration:
                    raise ValueError("Design piece not found in referenced design")
        if connecting.designPiece is not None:
            if connectingPiece.refDesign is None and designsById is None:
                raise FeatureNotYetSupported()
            if refDesign is None and designsById is not None:
                raise FeatureNotYetSupported()
            if refDesign is not None:
                try:
                    designPiece = next(p for p in refDesign.pieces if p.id_ == connecting.designPiece.id_)
                except StopIteration:
                    raise ValueError("Design piece not found in referenced design")
        try:
            entity.description = obj["description"]
        except KeyError:
        try:
            entity.gap = obj["gap"]
        except KeyError:
        try:
            entity.shift = obj["shift"]
        except KeyError:
        try:
            entity.rise = obj["rise"]
        except KeyError:
        try:
            entity.rotation = obj["rotation"]
        except KeyError:
        try:
            entity.turn = obj["turn"]
        except KeyError:
        try:
            entity.tilt = obj["tilt"]
        except KeyError:
        try:
            entity.x = obj["x"]
        except KeyError:
        try:
            entity.y = obj["y"]
        except KeyError:

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

    # TODO: Automatic updating based on props.
    def update(self, other: "Connection", empty: bool = False) -> "Connection":
        if empty:
            self.empty()
        props = ConnectionProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
            self.connected.piece.id_,
            (self.connected.connector.id_ if self.connected.connector is not None else ""),
            self.connecting.piece.id_,
            (self.connecting.connector.id_ if self.connecting.connector is not None else ""),
        ]


class ConnectionInputNode(InputNode):
    """GraphQL input node for connection mutations.
    ConnectionInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Connection

# region Stat
# [👤semio📚py💻semio🔖domain🔖stat](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat)
# Stat entity for recording computed statistics with bounds.


class StatKeyField(RealField, abc.ABC):
    """Field mixin for the key of a stat.
    StatKeyField MUST declare exactly one field with appropriate constraints.
    """

    key: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class StatUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a stat.
    StatUnitField MUST declare exactly one field with appropriate constraints.
    """

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class StatMinField(RealField, abc.ABC):
    """Field mixin for the min of a stat.
    StatMinField MUST declare exactly one field with appropriate constraints.
    """

    min: typing.Optional[float] = pydantic.Field(default=None)


class StatMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a stat.
    StatMinExcludedField MUST declare exactly one field with appropriate constraints.
    """

    min_excluded: bool = pydantic.Field(default=False)


class StatMaxField(RealField, abc.ABC):
    """Field mixin for the max of a stat.
    StatMaxField MUST declare exactly one field with appropriate constraints.
    """

    max: typing.Optional[float] = pydantic.Field(default=None)


class StatMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a stat.
    StatMaxExcludedField MUST declare exactly one field with appropriate constraints.
    """

    max_excluded: bool = pydantic.Field(default=False)


class StatCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a stat.
    StatCreatedField MUST declare exactly one field with appropriate constraints.
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class StatUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a stat.
    StatUpdatedField MUST declare exactly one field with appropriate constraints.
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class StatId(StatKeyField, Id):
    """Identity fields for uniquely identifying a stat.
    StatId MUST contain all fields that uniquely identify a stat.
    """



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
    """Property fields for a stat.
    StatProps MUST contain all non-relational property fields.
    """



class StatInput(
    StatMaxExcludedField,
    StatMaxField,
    StatMinExcludedField,
    StatMinField,
    StatUnitField,
    StatKeyField,
    Input,
):
    """Input fields for creating or updating a stat.
    StatInput MUST contain all fields required for creation.
    """



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
    """Output fields returned when fetching a stat.
    StatOutput MUST contain all fields returned on fetch.
    """



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
    """Stat entity for recording computed statistics with bounds.
    Stat MUST implement idMembers and inherit from the appropriate field mixins.
    """



# endregion Stat

# region Design
# [👤semio📚py💻semio🔖domain🔖design](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design)
# Design entity for composing pieces and connections into assemblies.


class DesignNameField(RealField, abc.ABC):
    """Field mixin for the name of a design.
    DesignNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class DesignDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a design.
    DesignDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class DesignIconField(RealField, abc.ABC):
    """Field mixin for the icon of a design.
    DesignIconField MUST declare exactly one field with appropriate constraints.
    """

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class DesignImageField(RealField, abc.ABC):
    """Field mixin for the image of a design.
    DesignImageField MUST declare exactly one field with appropriate constraints.
    """

    image: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)




class DesignParentField(RealField, abc.ABC):
    """Field mixin for the parent of a design.
    DesignParentField MUST declare exactly one field with appropriate constraints.
    """

    parent: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class DesignIsAbstractField(RealField, abc.ABC):
    """Field mixin for the is abstract of a design.
    DesignIsAbstractField MUST declare exactly one field with appropriate constraints.
    """

    is_abstract: bool = pydantic.Field(default=False)


class DesignFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a design.
    DesignFolderField MUST declare exactly one field with appropriate constraints.
    """

    folder: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class DesignActiveLayerField(RealField, abc.ABC):
    """Field mixin for the active layer of a design.
    DesignActiveLayerField MUST declare exactly one field with appropriate constraints.
    """

    activeLayer: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class DesignLocationField(MaskedField, abc.ABC):
    """Field mixin for the location of a design.
    DesignLocationField MUST declare exactly one field with appropriate constraints.
    """

    location: typing.Optional[Location] = pydantic.Field(default=None)


class DesignUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a design.
    DesignUnitField MUST declare exactly one field with appropriate constraints.
    """

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class DesignScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a design.
    DesignScalableField MUST declare exactly one field with appropriate constraints.
    """

    can_scale: bool = pydantic.Field(default=True)


class DesignMirrorableField(RealField, abc.ABC):
    """Field mixin for the mirrorable of a design.
    DesignMirrorableField MUST declare exactly one field with appropriate constraints.
    """

    can_mirror: bool = pydantic.Field(default=True)


class DesignCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a design.
    DesignCreatedField MUST declare exactly one field with appropriate constraints.
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class DesignUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a design.
    DesignUpdatedField MUST declare exactly one field with appropriate constraints.
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class DesignId(DesignNameField, Id):
    """Identity fields for uniquely identifying a design.
    DesignId MUST contain all fields that uniquely identify a design.
    """



class DesignProps(
    DesignUnitField,
    DesignActiveLayerField,
    DesignFolderField,
    DesignIsAbstractField,
    DesignParentField,
    DesignLocationField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Props,
):
    """Property fields for a design.
    DesignProps MUST contain all non-relational property fields.
    """



class DesignInput(
    DesignUnitField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Input,
):
    """Input fields for creating or updating a design.
    DesignInput MUST contain all fields required for creation.
    """

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
    """Context fields for understanding a design by an LLM.
    DesignContext MUST contain all fields needed for LLM understanding.
    """


    location: typing.Optional[LocationContext] = pydantic.Field(default=None)
    pieces: list[PieceContext] = pydantic.Field(default_factory=list)
    connections: list[ConnectionContext] = pydantic.Field(default_factory=list)
    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


class DesignOutput(
    DesignUpdatedField,
    DesignCreatedField,
    DesignUnitField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Output,
):
    """Output fields returned when fetching a design.
    DesignOutput MUST contain all fields returned on fetch.
    """


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
    """Prediction fields for LLM-based design inference.
    DesignPrediction MUST contain all fields for LLM inference.
    """


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
    TableEntity,
):
    """Design entity composing pieces and connections into an assembly.
    Design MUST implement idMembers and inherit from the appropriate field mixins.
    """

    concepts_: list[Concept] = pydantic.Field(default_factory=list)
    artifact_authors: list[ArtifactAuthor] = pydantic.Field(default_factory=list)
    layers: list[Layer] = pydantic.Field(default_factory=list)
    pieces: list[Piece] = pydantic.Field(default_factory=list)
    groups: list[Group] = pydantic.Field(default_factory=list)
    connections: list[Connection] = pydantic.Field(default_factory=list)
    stats: list[Stat] = pydantic.Field(default_factory=list)
    props: list["Prop"] = pydantic.Field(default_factory=list)
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def location(self) -> typing.Optional[Location]:
        if self.locationLongitude is None or self.locationLatitude is None:
            longitude=self.locationLongitude,
            latitude=self.locationLatitude,
        )

    def location(self, location: typing.Optional[Location]):
        if location is None:
        else:

    def authors(self) -> list[str]:
        return [artifact_author.author_email for artifact_author in self.artifact_authors]

    def authors(self, author_emails: list[str]):
        self.artifact_authors = [ArtifactAuthor(author_email=email) for email in author_emails]

    def concepts(self: "Design") -> list[str]:
        return [concept.name for concept in sorted(self.concepts_, key=lambda x: x.order)]

    def concepts(self: "Design", concepts: list[str]):
        self.concepts_ = [Concept(name=concept, order=i) for i, concept in enumerate(concepts)]

    def parent_entity(self) -> "Kit":
        if self.kit is None:
            raise NoKitAssigned()

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    def parse(
        cls: "Design",
        input: str | dict | DesignInput | typing.Any | None,
        types: list[Type],
        designsById: typing.Optional[dict[str, dict[str, dict[str, "Design"]]]] = None,
    ) -> "Design":
        if input is None:
            return cls()
        props = DesignProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
        except KeyError, AttributeError, Exception:
        typesDict = {}
        for type in types:
            if type.name not in typesDict:
                typesDict[type.name] = {}
            if type.variant not in typesDict[type.name]:
                typesDict[type.name][type.variant] = {}
        try:
            pieces = [Piece.parse(p, typesDict, designsById) for p in obj["pieces"]]
        except KeyError, AttributeError, Exception:
        try:
            connections = [Connection.parse(c, pieces, designsById) for c in obj["connections"]]
        except KeyError, AttributeError, Exception:
        try:
            props = [Prop.parse(p) for p in obj["props"]]
        except KeyError, AttributeError, Exception:
        try:
            attributes = [Attribute.parse(q) for q in obj["attributes"]]
        except KeyError, AttributeError, Exception:
        try:
            author_emails = obj["authors"]
        except KeyError, AttributeError, Exception:
        try:
            concepts = obj["concepts"]
        except KeyError, AttributeError, Exception:

    def dump(self) -> "DesignOutput":
        entity = {**DesignProps.model_validate(self).model_dump()}
        entity["pieces"] = [p.dump() for p in self.pieces]
        entity["connections"] = [c.dump() for c in self.connections]
        entity["props"] = [p.dump() for p in self.props]
        entity["attributes"] = [q.dump() for q in self.attributes]
        return DesignOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        props = DesignProps()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.designs = []

    # TODO: Automatic updating based on props.
    def update(self, other: "Design", empty: bool = False) -> "Design":
        if empty:
            self.empty()
        props = DesignProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return [self.name, self.variant]


class NoDesignAssigned(NoParentAssigned):
    """No Design Assigned definition.
    NoDesignAssigned MUST fulfill its documented contract.
    """

    def __str__(self):


class DesignInputNode(InputNode):
    """GraphQL input node for design mutations.
    DesignInputNode MUST expose the input model via Meta.
    """

    class Meta:


class DesignIdInputNode(InputNode):
    """GraphQL input node for design id mutations.
    DesignIdInputNode MUST expose the input model via Meta.
    """

    class Meta:


# endregion Design

# region Kit
# [👤semio📚py💻semio🔖domain🔖kit](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit)
# Kit entity for packaging types, designs, qualities and metadata.


# #region 🔖KitKind
# [👤semio📚py💻semio🔖modeltypeskit🔖kitkind](repo://p/u/semio/b/l/py/f/semio.py/s/Model%20Types%20-%20Kit/s/KitKind)
# KitKind discriminates the five persistence/transport forms of a Kit.


class KitKind(str, enum.Enum):
    """Discriminator for the five kit persistence/transport forms.

    Specs: Exactly five kit kinds exist:
    - REMOTE: URL-addressable kit served over HTTP(S)
    - TEMPORARY: In-memory ephemeral kit (no persistence)
    """



ALL_KIT_KINDS: list[KitKind] = list(KitKind)

# #endregion 🔖KitKind


class KitUriField(RealField, abc.ABC):
    """Field mixin for the uri of a kit.
    KitUriField MUST declare exactly one field with appropriate constraints.
    """

    uri: str = pydantic.Field(max_length=URI_LENGTH_LIMIT)


class KitNameField(RealField, abc.ABC):
    """Field mixin for the name of a kit.
    KitNameField MUST declare exactly one field with appropriate constraints.
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class KitDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a kit.
    KitDescriptionField MUST declare exactly one field with appropriate constraints.
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class KitIconField(RealField, abc.ABC):
    """Field mixin for the icon of a kit.
    KitIconField MUST declare exactly one field with appropriate constraints.
    """

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitImageField(RealField, abc.ABC):
    """Field mixin for the image of a kit.
    KitImageField MUST declare exactly one field with appropriate constraints.
    """

    image: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitPreviewField(RealField, abc.ABC):
    """Field mixin for the preview of a kit.
    KitPreviewField MUST declare exactly one field with appropriate constraints.
    """

    preview: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitVersionField(RealField, abc.ABC):
    """Field mixin for the version of a kit.
    KitVersionField MUST declare exactly one field with appropriate constraints.
    """

    version: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class KitRemoteField(RealField, abc.ABC):
    """Field mixin for the remote of a kit.
    KitRemoteField MUST declare exactly one field with appropriate constraints.
    """

    remote: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitHomepageField(RealField, abc.ABC):
    """Field mixin for the homepage of a kit.
    KitHomepageField MUST declare exactly one field with appropriate constraints.
    """

    homepage: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitLicenseField(RealField, abc.ABC):
    """Field mixin for the license of a kit.
    KitLicenseField MUST declare exactly one field with appropriate constraints.
    """

    license: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a kit.
    KitCreatedField MUST declare exactly one field with appropriate constraints.
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class KitUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a kit.
    KitUpdatedField MUST declare exactly one field with appropriate constraints.
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class KitId(KitUriField, Id):
    """Identity fields for uniquely identifying a kit.
    KitId MUST contain all fields that uniquely identify a kit.
    """



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
    """Property fields for a kit.
    KitProps MUST contain all non-relational property fields.
    """



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
    """Input fields for creating or updating a kit.
    KitInput MUST contain all fields required for creation.
    """


    types: list[TypeInput] = pydantic.Field(default_factory=list)
    designs: list[DesignInput] = pydantic.Field(default_factory=list)
    folders: list[FolderInput] = pydantic.Field(default_factory=list)
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


class KitContext(KitDescriptionField, KitNameField, Context):
    """Context fields for understanding a kit by an LLM.
    KitContext MUST contain all fields needed for LLM understanding.
    """


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
    """Output fields returned when fetching a kit.
    KitOutput MUST contain all fields returned on fetch.
    """


    types: list[TypeOutput] = pydantic.Field(default_factory=list)
    designs: list[DesignOutput] = pydantic.Field(default_factory=list)
    folders: list[FolderOutput] = pydantic.Field(default_factory=list)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


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
    """Kit entity packaging types, designs, qualities and metadata.
    Kit MUST implement idMembers and inherit from the appropriate field mixins.
    """

    concepts_: list[Concept] = pydantic.Field(default_factory=list)
    authors_: list[Author] = pydantic.Field(default_factory=list)
    files_: list[File] = pydantic.Field(default_factory=list)
    folders_: list[Folder] = pydantic.Field(default_factory=list)
    ports: list[Port] = pydantic.Field(default_factory=list)
    types: list[Type] = pydantic.Field(default_factory=list)
    designs: list[Design] = pydantic.Field(default_factory=list)
    qualities: list[Quality] = pydantic.Field(default_factory=list)
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def concepts(self: "Kit") -> list[str]:
        if self.concepts_ is None:
            return []
        return [concept.name for concept in sorted(self.concepts_, key=lambda x: x.order)]

    def concepts(self: "Kit", concepts: list[str]):
        self.concepts_ = [Concept(name=concept, order=i) for i, concept in enumerate(concepts)]

    def folders(self: "Kit") -> list[Folder]:

    def folders(self: "Kit", folders: list[Folder]):

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    def parse(cls: "Kit", input: str | dict | KitInput | typing.Any | None) -> "Kit":
        if input is None:
            return cls()
        guid = obj.get("guid", str(uuid.uuid4()))
        uri = obj.get("uri", f"memory://{obj.get('name', 'unnamed')}")
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
        except KeyError, AttributeError, Exception:
        try:
            designs = [Design.parse(d, types) for d in obj["designs"]]
        except KeyError, AttributeError, Exception:
        try:
            folders = [Folder.parse(f) for f in obj["folders"]]
        except KeyError, AttributeError, Exception:
        try:
            concepts = obj["concepts"]
        except KeyError, AttributeError, Exception:

    def dump(self) -> "KitOutput":
        entity = {**KitProps.model_validate(self).model_dump()}
        entity["types"] = [t.dump() for t in (self.types or [])]
        entity["designs"] = [d.dump() for d in (self.designs or [])]
        entity["files"] = [f.dump() for f in (self.files_ or [])]
        entity["folders"] = [f.dump() for f in (self.folders_ or [])]
        entity["attributes"] = [q.dump() for q in (self.attributes or [])]
        entity["concepts"] = self.concepts or []
        return KitOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        props = KitProps.model_construct()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.types = []

    # TODO: Automatic updating based on props.
    def update(self, other: "Kit", empty: bool = False) -> "Kit":
        if empty:
            self.empty()
        props = KitProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:

    def guid(self) -> str:
        return self.id()

    # region Design Family Helpers
    # [👤semio📚py💻semio🔖domain🔖kit🔖designfamilyhelpers](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/s/Design%20Family%20Helpers)
    # Helper functions for querying design hierarchies and families.

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

    def get_design_siblings(self, design_guid: str) -> list["Design"]:
        """Returns all designs with the same parent, excluding self."""
        design = self.find_design_by_guid(design_guid)
        return [d for d in self.designs if (d.parent.guid if d.parent else None) == parent_guid and d.guid != design_guid]

    def get_design_children(self, design_guid: str) -> list["Design"]:
        """Returns all direct children of a design."""
        return [d for d in self.designs if d.parent and d.parent.guid == design_guid]

    # endregion Design Family Helpers

    # region Type Family Helpers
    # [👤semio📚py💻semio🔖domain🔖kit🔖typefamilyhelpers](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/s/Type%20Family%20Helpers)
    # Helper functions for querying type hierarchies and families.

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

    def get_type_siblings(self, type_guid: str) -> list["Type"]:
        """Returns all types with the same parent, excluding self."""
        type_ = self.find_type_by_guid(type_guid)
        return [t for t in self.types if (t.parent.guid if t.parent else None) == parent_guid and t.guid != type_guid]

    def get_type_children(self, type_guid: str) -> list["Type"]:
        """Returns all direct children of a type."""
        return [t for t in self.types if t.parent and t.parent.guid == type_guid]

    # endregion Type Family Helpers

    # region Kit Query Helpers
    # [👤semio📚py💻semio🔖domain🔖kit🔖kitqueryhelpers](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/s/Kit%20Query%20Helpers)
    # Helper functions for querying entities in kits.

    def find_port_in_kit(self, port_guid: str) -> "Port":
        """Finds a port by GUID in the kit."""
        for port in self.ports or []:
            if port.guid == port_guid:
        raise ValueError(f"Port {port_guid} not found in kit {self.name}")

    def find_piece_in_design(self, design_guid: str, piece_guid: str) -> "Piece":
        """Finds a piece by GUID in a design."""
        design = self.find_design_by_guid(design_guid)
        for piece in design.pieces or []:
            if piece.guid == piece_guid:
        raise ValueError(f"Piece {piece_guid} not found in design {design_guid}")

    def find_connection_in_design(self, design_guid: str, connection_guid: str) -> "Connection":
        """Finds a connection by GUID in a design."""
        design = self.find_design_by_guid(design_guid)
        for connection in design.connections or []:
            if connection.guid == connection_guid:
        raise ValueError(f"Connection {connection_guid} not found in design {design_guid}")

    def find_piece_connections_in_design(self, design_guid: str, piece_guid: str) -> list["Connection"]:
        """Finds all connections involving a piece in a design."""
        design = self.find_design_by_guid(design_guid)
        return [c for c in (design.connections or []) if c.connected.piece.guid == piece_guid or c.connecting.piece.guid == piece_guid]

    def find_piece_type_in_design(self, design_guid: str, piece_guid: str) -> "Type":
        """Gets the type of a piece in a design."""
        piece = self.find_piece_in_design(design_guid, piece_guid)
        if not piece.type or not piece.type.guid:
            raise ValueError(f"Piece {piece_guid} has no type")
        return self.find_type_by_guid(piece.type.guid)

    def find_connector_in_type(self, type_guid: str, connector_guid: str) -> "Connector":
        """Finds a connector by GUID in a type."""
        type_ = self.find_type_by_guid(type_guid)
        for connector in type_.connectors or []:
            if connector.guid == connector_guid:
        raise ValueError(f"Connector {connector_guid} not found in type {type_guid}")

    def find_connector_for_piece_in_connection(self, type_guid: str, connection: "Connection", piece_guid: str) -> typing.Optional["Connector"]:
        """Gets the connector used by a piece in a connection."""
        if connection.connected.piece.guid == piece_guid:
        else:
        if not connector_guid:
        return self.find_connector_in_type(type_guid, connector_guid)

    def find_used_connectors_by_piece_in_design(self, design_guid: str, piece_guid: str) -> list["Connector"]:
        """Returns all connectors of a piece that are used in connections."""
        piece = self.find_piece_in_design(design_guid, piece_guid)
        if not piece.type or not piece.type.guid:
            return []
        connections = self.find_piece_connections_in_design(design_guid, piece_guid)
        result = []
        for c in connections:
            connector = self.find_connector_for_piece_in_connection(piece.type.guid, c, piece_guid)
            if connector is not None:
                result.append(connector)

    def find_replaceable_types_for_piece_in_design(
        self,
        design_guid: str,
        piece_guid: str,
        variants: typing.Optional[list[str]] = None,
    ) -> list["Type"]:
        """Finds all types that can replace a piece while maintaining connection compatibility."""
        design = self.find_design_by_guid(design_guid)
        connections = self.find_piece_connections_in_design(design_guid, piece_guid)
        required_connectors: list["Connector"] = []
        for connection in connections:
            try:
                other_piece = self.find_piece_in_design(design_guid, other_piece_guid)
                if not other_piece.type or not other_piece.type.guid:
                if connection.connected.piece.guid == piece_guid:
                else:
                if not other_connector_guid:
                other_connector = self.find_connector_in_type(other_piece.type.guid, other_connector_guid)
                required_connectors.append(other_connector)
            except ValueError, AttributeError:
        result = []
        for replacement_type in self.types or []:
            if replacement_type.isAbstract:
            if variants is not None and (replacement_type.parent.guid if replacement_type.parent else "") not in variants:
            type_connectors = replacement_type.connectors or []
            if len(type_connectors) == 0:
                if len(required_connectors) == 0:
                    result.append(replacement_type)
            if all(any(True for _ in type_connectors) for _ in required_connectors):
                result.append(replacement_type)

    def find_replaceable_types_for_pieces_in_design(
        self,
        design_guid: str,
        piece_guids: list[str],
        variants: typing.Optional[list[str]] = None,
    ) -> list["Type"]:
        """Finds types that can replace multiple pieces while maintaining all external connections."""
        design = self.find_design_by_guid(design_guid)
        external_connectors: list["Connector"] = []
        for piece_guid in piece_guids:
            connections = self.find_piece_connections_in_design(design_guid, piece_guid)
            for connection in connections:
                if other_piece_guid not in piece_guids:
                    try:
                        other_piece = self.find_piece_in_design(design_guid, other_piece_guid)
                        if not other_piece.type or not other_piece.type.guid:
                        if connection.connected.piece.guid == piece_guid:
                        else:
                        if not other_connector_guid:
                        other_connector = self.find_connector_in_type(other_piece.type.guid, other_connector_guid)
                        external_connectors.append(other_connector)
                    except ValueError, AttributeError:
        result = []
        for replacement_type in self.types or []:
            if replacement_type.isAbstract:
            if variants is not None and (replacement_type.parent.guid if replacement_type.parent else "") not in variants:
            type_connectors = replacement_type.connectors or []
            if len(type_connectors) == 0:
                if len(external_connectors) == 0:
                    result.append(replacement_type)
            if all(any(True for _ in type_connectors) for _ in external_connectors):
                result.append(replacement_type)

    # endregion Kit Query Helpers

    # region Filter
    # [👤semio📚py💻semio🔖domain🔖kit🔖filter](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/s/Filter)
    # Filter MUST provide functions to produce a minimal kit subset scoped to a single design.

    def _select_best_model_filter(models: list, resolved_tag_guids: list[str]):
        """Selects the best model based on tag matching using Jaccard similarity.
        """
        if not models:
        if not resolved_tag_guids:
            for m in models:
                if not getattr(m, "tags", None):
            return models[0]
        filtered = []
        for m in models:
            model_tag_guids = {t.guid for t in (getattr(m, "tags", None) or [])}
            if all(g in model_tag_guids for g in resolved_tag_guids):
                filtered.append(m)
        if not filtered:

        def jaccard(m):
            model_tag_guids = {t.guid for t in (getattr(m, "tags", None) or [])}
            sel = set(resolved_tag_guids)
            if not union:
            return len(model_tag_guids & sel) / len(union)

        return max(filtered, key=jaccard)

    def _matches_glob_filter(name: str, glob_filter: typing.Optional[dict] = None) -> bool:
        """Checks if a name passes a glob filter with include/exclude patterns.
        """
        if glob_filter is None:
        include = glob_filter.get("include") or []
        exclude = glob_filter.get("exclude") or []
        if include and not any(fnmatch.fnmatch(name.lower(), p.lower()) for p in include):
        if any(fnmatch.fnmatch(name.lower(), p.lower()) for p in exclude):

    def filter_kit(self: "Kit", filter_spec: dict) -> "Kit":
        """General-purpose kit filter combining optional design-based transitive filtering with glob-based name filtering.
        When design_guid is set, first performs transitive design-scoped subset extraction.
        Glob filters (include/exclude patterns on names) are applied to each entity kind afterwards.
        """
        design_guid = filter_spec.get("design_guid")
        model_tags = filter_spec.get("model_tags")

        if design_guid:
            base = self._filter_kit_by_design(design_guid, model_tags)
        else:

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

        result = copy.copy(base)

        if filter_spec.get("types") is not None:
            result.types = [t for t in (base.types or []) if Kit._matches_glob_filter(t.name, filter_spec["types"])]
        if filter_spec.get("designs") is not None:
            result.designs = [d for d in (base.designs or []) if Kit._matches_glob_filter(d.name, filter_spec["designs"])]
        if filter_spec.get("ports") is not None:
            result.ports = [p for p in (base.ports or []) if Kit._matches_glob_filter(p.name, filter_spec["ports"])]
        if filter_spec.get("files") is not None:
            result.files_ = [f for f in (base.files_ or []) if Kit._matches_glob_filter(f.name, filter_spec["files"])]
        if filter_spec.get("tags") is not None:
            if hasattr(base, "tags_") and base.tags_ is not None:
                result.tags_ = [t for t in base.tags_ if Kit._matches_glob_filter(t.name, filter_spec["tags"])]
        if filter_spec.get("concepts") is not None:
            if hasattr(base, "concepts_") and base.concepts_ is not None:
                result.concepts_ = [c for c in base.concepts_ if Kit._matches_glob_filter(c.name, filter_spec["concepts"])]
        if filter_spec.get("qualities") is not None:
            result.qualities = [q for q in (base.qualities or []) if Kit._matches_glob_filter(q.name, filter_spec["qualities"])]
        if filter_spec.get("authors") is not None:
            result.authors_ = [a for a in (base.authors_ or []) if Kit._matches_glob_filter(a.name, filter_spec["authors"])]
        if filter_spec.get("folders") is not None:
            result.folders_ = [f for f in (base.folders_ or []) if Kit._matches_glob_filter(f.name, filter_spec["folders"])]


    def _filter_kit_by_design(self: "Kit", design_guid: str, tags: typing.Optional[list[str]] = None) -> "Kit":
        """Filters a kit to only include entities related to a specific design.
        Removes types not used by pieces, designs not the target, ports not used by connectors of used types,
        files not used by selected models, tags/concepts only if referenced, and selects one model per type based on tags.
        """
        design = self.find_design_by_guid(design_guid)
        pieces = design.pieces or []

        used_type_guids: set[str] = set()
        used_design_guids: set[str] = {design_guid}

        for piece in pieces:
            if piece.type and piece.type.guid:
                used_type_guids.add(piece.type.guid)
            if piece.design and piece.design.guid:
                used_design_guids.add(piece.design.guid)

        all_types = self.types or []
        type_by_guid = {t.guid: t for t in all_types}

        def collect_type_ancestors(type_guid: str):
            t = type_by_guid.get(type_guid)
            if t and t.parent and t.parent.guid and t.parent.guid not in used_type_guids:
                used_type_guids.add(t.parent.guid)
                collect_type_ancestors(t.parent.guid)

        for guid in list(used_type_guids):
            collect_type_ancestors(guid)

        all_tags = list(getattr(self, "tags_", None) or []) if hasattr(self, "tags_") else []
        resolved_tag_guids: list[str] = []
        for tag_value in tags or []:
            for tag in all_tags:
                if tag.guid == tag_value:
                    resolved_tag_guids.append(tag.guid)
            if not found:
                for tag in all_tags:
                    if tag.name == tag_value:
                        resolved_tag_guids.append(tag.guid)

        used_port_guids: set[str] = set()
        used_file_guids: set[str] = set()
        used_tag_guids: set[str] = set()
        used_concept_guids: set[str] = set()
        used_quality_guids: set[str] = set()
        used_author_guids: set[str] = set()
        used_folder_names: set[str] = set()

        def collect_quality_from_props(props):
            for prop in props or []:
                if hasattr(prop, "quality") and prop.quality and hasattr(prop.quality, "guid"):
                    used_quality_guids.add(prop.quality.guid)

        selected_models: dict[str, typing.Any] = {}
        for type_guid in used_type_guids:
            t = type_by_guid.get(type_guid)
            if not t:
            if getattr(t, "folder", None):
                used_folder_names.add(t.folder)
            for connector in t.connectors or []:
                if connector.port and connector.port.guid:
                    used_port_guids.add(connector.port.guid)
                collect_quality_from_props(getattr(connector, "props", None))
            collect_quality_from_props(getattr(t, "props", None))
            for author_id in getattr(t, "authors", None) or []:
                if hasattr(author_id, "guid"):
                    used_author_guids.add(author_id.guid)
            for concept_id in getattr(t, "concepts", None) or []:
                if hasattr(concept_id, "guid"):
                    used_concept_guids.add(concept_id.guid)

            models = getattr(t, "models", None) or []
            if models:
                best = Kit._select_best_model_filter(models, resolved_tag_guids)
                if best:
                    if hasattr(best, "file") and best.file and hasattr(best.file, "guid"):
                        used_file_guids.add(best.file.guid)
                    for tag_id in getattr(best, "tags", None) or []:
                        used_tag_guids.add(tag_id.guid)

        for piece in pieces:
            collect_quality_from_props(getattr(piece, "props", None))

        for concept_id in getattr(design, "concepts", None) or []:
            if hasattr(concept_id, "guid"):
                used_concept_guids.add(concept_id.guid)
        for author_id in getattr(design, "authors", None) or []:
            if hasattr(author_id, "guid"):
                used_author_guids.add(author_id.guid)

        port_snapshot = list(used_port_guids)
        for port_guid in port_snapshot:
            for port in self.ports or []:
                if port.guid == port_guid:
                    for compat in getattr(port, "compatiblePorts", None) or getattr(port, "compatible_ports", None) or []:
                        if hasattr(compat, "guid"):
                            used_port_guids.add(compat.guid)

        for tag_guid in resolved_tag_guids:
            used_tag_guids.add(tag_guid)

        import copy

        result = copy.copy(self)
        result.types = []
        for t in all_types:
            if t.guid not in used_type_guids:
            t_copy = copy.copy(t)
            if t.guid in selected_models:
                t_copy.models = [selected_models[t.guid]]
            else:
                t_copy.models = []
            result.types.append(t_copy)

        result.designs = [d for d in (self.designs or []) if d.guid in used_design_guids]
        result.ports = [p for p in (self.ports or []) if p.guid in used_port_guids]
        result.files_ = [f for f in (self.files_ or []) if f.guid in used_file_guids]
        result.qualities = [q for q in (self.qualities or []) if q.guid in used_quality_guids]
        result.authors_ = [a for a in (self.authors_ or []) if a.guid in used_author_guids]
        result.folders_ = [f for f in (self.folders_ or []) if f.name in used_folder_names]
        if hasattr(self, "tags_") and self.tags_ is not None:
            result.tags_ = [t for t in self.tags_ if t.guid in used_tag_guids]
        if hasattr(self, "concepts_") and self.concepts_ is not None:
            result.concepts_ = [c for c in self.concepts_ if c.guid in used_concept_guids]


    # endregion Filter


# endregion Kit

# region Moved Graphene Nodes
# [👤semio📚py💻semio🔖domain🔖movedgraphenenodes](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes)
# Graphene node definitions moved here due to forward-reference resolution order.


class AttributeNode(TableEntityNode):
    """GraphQL node exposing attribute data.
    AttributeNode MUST expose the model via Meta.
    """

    class Meta:


class PlaneNode(TableNode):
    """GraphQL node exposing plane data.
    PlaneNode MUST expose the model via Meta.
    """

    class Meta:


class AuthorNode(TableEntityNode):
    """GraphQL node exposing author data.
    AuthorNode MUST expose the model via Meta.
    """

    class Meta:


class ModelNode(TableEntityNode):
    """GraphQL node exposing model data.
    ModelNode MUST expose the model via Meta.
    """

    class Meta:
        excludedFields = ("tags_",)


class ConnectorNode(TableEntityNode):
    """GraphQL node exposing connector data.
    ConnectorNode MUST expose the model via Meta.
    """

    class Meta:
        exclude_fields = ("connecteds", "connectings")

    localId = graphene.String()

    def resolve_localId(self, info):
        return getattr(self, "id_", "")


class TypeNode(TableEntityNode):
    """GraphQL node exposing type data.
    TypeNode MUST expose the model via Meta.
    """

    class Meta:


class PieceNode(TableEntityNode):
    """GraphQL node exposing piece data.
    PieceNode MUST expose the model via Meta.
    """

    class Meta:
        exclude_fields = ("connecteds", "connectings")

    localId = graphene.String()

    def resolve_localId(self, info):
        return getattr(self, "id_", "")


class ConnectionNode(TableEntityNode):
    """GraphQL node exposing connection data.
    ConnectionNode MUST expose the model via Meta.
    """

    class Meta:
            "connectedPiece",
            "connectedConnector",
            "connectingPiece",
            "connectingConnector",
        )

    connected = graphene.NonNull(lambda: SideNode)
    connecting = graphene.NonNull(lambda: SideNode)

    def resolve_connected(self, info):

    def resolve_connecting(self, info):


class DesignNode(TableEntityNode):
    """GraphQL node exposing design data.
    DesignNode MUST expose the model via Meta.
    """

    class Meta:


class KitNotFound(NotFound):
    """endregion Moved Graphene Nodes
    KitNotFound MUST provide a descriptive error message via __str__.
    """

    def __init__(self, uri: str) -> None:

    def __str__(self):


class NoKitToDelete(KitNotFound):
    """No Kit To Delete definition.
    NoKitToDelete MUST fulfill its documented contract.
    """

    def __init__(self, uri: str) -> None:

    def __str__(self):


class KitZipDoesNotContainSemioFolder(KitNotFound):
    """Kit Zip Does Not Contain Semio Folder definition.
    KitZipDoesNotContainSemioFolder MUST fulfill its documented contract.
    """

    def __init__(self, uri: str) -> None:

    def __str__(self):


class OnlyRemoteKitsCanBeCached(ClientError):
    """Only Remote Kits Can Be Cached definition.
    OnlyRemoteKitsCanBeCached MUST fulfill its documented contract.
    """

    def __init__(self, nonRemoteUri: str) -> None:

    def __str__(self):


class KitUriNotValid(ClientError, abc.ABC):
    """ The base for all kit uri not valid errors.
    KitUriNotValid MUST be subclassed and MUST NOT be instantiated directly.
    """


class LocalKitUriNotValid(KitUriNotValid, abc.ABC):
    """ The base for all local kit uri not valid errors.
    LocalKitUriNotValid MUST be subclassed and MUST NOT be instantiated directly.
    """


class LocalKitUriIsNotAbsolute(LocalKitUriNotValid):
    """Local Kit Uri Is Not Absolute definition.
    LocalKitUriIsNotAbsolute MUST fulfill its documented contract.
    """

    def __init__(self, uri: str) -> None:

    def __str__(self):


class LocalKitUriIsNotDirectory(LocalKitUriNotValid):
    """Local Kit Uri Is Not Directory definition.
    LocalKitUriIsNotDirectory MUST fulfill its documented contract.
    """

    def __init__(self, uri: str) -> None:

    def __str__(self):


class NoKitAssigned(NoParentAssigned):
    """No Kit Assigned definition.
    NoKitAssigned MUST fulfill its documented contract.
    """

    def __str__(self):


class KitAlreadyExists(AlreadyExists, abc.ABC):
    """Exception for attempting to create a kit that already exists.
    KitAlreadyExists MUST provide a descriptive error message via __str__.
    """

    def __init__(self, uri: str) -> None:

    def __str__(self) -> str:


class KitInputNode(InputNode):
    """GraphQL input node for kit mutations.
    KitInputNode MUST expose the input model via Meta.
    """

    class Meta:


class KitNode(TableEntityNode):
    """GraphQL node exposing kit data.
    KitNode MUST expose the model via Meta.
    """

    class Meta:


# #endregion 🔖Moved Graphene Nodes

# region Validation
# [👤semio📚py💻semio🔖domain🔖validation](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation)
# Validation logic for checking kit constraints and uniqueness rules.


class ValidationFix:
    """A proposed fix for a validation problem with a title and diff.
    ValidationFix MUST contain a non-empty title and a valid diff dictionary.
    """


    def toDict(self) -> dict:
        return {"title": self.title, "diff": self.diff}


class Problem:
    """A validation problem with a constraint identifier and message.
    Problem MUST contain a non-empty constraint identifier and message.
    """

    fixes: list[ValidationFix] = dataclasses.field(default_factory=list)

    def toDict(self) -> dict:
            "constraintId": self.constraintId,
            "message": self.message,
            "entityKind": self.entityKind,
            "entityGuid": self.entityGuid,
            "fixes": [f.toDict() for f in self.fixes],
        }


class ValidationResult:
    """A validation result aggregating problems and fixes for an entity.
    ValidationResult MUST aggregate all problems and fixes for a single entity.
    """

    problems: list[Problem]

    def hasErrors(self) -> bool:

    def toDict(self) -> dict:
        sortedProblems = sorted(self.problems, key=lambda i: (i.constraintId, i.entityGuid))
        return {"problems": [i.toDict() for i in sortedProblems]}

    def serialize(self) -> str:
        return json.dumps(self.toDict(), indent=2)


def _isGuid(s: str) -> bool:
    """
    """
    import re

            r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
            s,
            re.IGNORECASE,
        )
    )


def _normalizeGuids(obj: typing.Any) -> typing.Any:
    """
    if obj is None:
    if isinstance(obj, str) and _isGuid(obj):
    if isinstance(obj, list):
        return [_normalizeGuids(x) for x in obj]
    if isinstance(obj, dict):
        return {k: _normalizeGuids(v) for k, v in obj.items()}


def areValidationResultsEqual(a: ValidationResult, b: ValidationResult) -> bool:
    """Check whether two validation results are semantically equal.
    """
    if len(a.problems) != len(b.problems):
    sortedA = sorted(a.problems, key=lambda i: (i.constraintId, i.entityGuid))
    sortedB = sorted(b.problems, key=lambda i: (i.constraintId, i.entityGuid))
    for ia, ib in zip(sortedA, sortedB):
        if ia.constraintId != ib.constraintId or ia.message != ib.message or ia.entityKind != ib.entityKind or ia.entityGuid != ib.entityGuid:
        if len(ia.fixes) != len(ib.fixes):
        for fa, fb in zip(ia.fixes, ib.fixes):
            if fa.title != fb.title:

            if ia.constraintId == "guid-unique":
            if json.dumps(_normalizeGuids(fa.diff), sort_keys=True) != json.dumps(_normalizeGuids(fb.diff), sort_keys=True):


def parseValidationResult(jsonStr: str) -> ValidationResult:
    """Parse a validation result from a dictionary representation.
    parseValidationResult MUST return a ValidationResult from a dict.
    """
    data = json.loads(jsonStr)
    problems = []
    for i in data["problems"]:
        fixes = [ValidationFix(title=f["title"], diff=f["diff"]) for f in i.get("fixes", [])]
                constraintId=i["constraintId"],
                message=i["message"],
                entityKind=i["entityKind"],
                entityGuid=i["entityGuid"],
                fixes=fixes,
            )
        )
    return ValidationResult(problems=problems)


def validateGuidUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all GUIDs within a collection are unique.
    validateGuidUniqueness MUST report duplicate GUIDs as problems.
    """
    problems: list[Problem] = []
    seen: dict[str, str] = {}

    def check(entityKind: str, entityGuid: str) -> None:
        if entityGuid in seen:
                    constraintId="guid-unique",
                    message=f'Duplicate GUID "{entityGuid}". First occurrence kept.',
                    entityKind=entityKind,
                    entityGuid=entityGuid,
                )
            )
        else:

    check("Kit", kit.guid)
    for t in kit.types or []:
        check("Type", t.guid)
    for d in kit.designs or []:
        check("Design", d.guid)
        for p in d.pieces or []:
            check("Piece", p.guid)
        for c in d.connections or []:
            check("Connection", c.guid)
        for s in d.stats or []:
            check("Stat", s.guid)
    for q in kit.qualities or []:
        check("Quality", q.guid)
    for f in kit.files_ or []:
        check("File", f.guid)
    for fo in kit.folders_ or []:
        check("Folder", fo.guid)


def validateTypeNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all type names within a kit are unique.
    validateTypeNameUniqueness MUST report duplicate type names as problems.
    """
    problems: list[Problem] = []
    byParent: dict[str | None, list[Type]] = {}
    for t in kit.types or []:
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
                            constraintId="type-name-unique",
                            message=f'Duplicate type name "{name}" among siblings.',
                            entityKind="Type",
                            entityGuid=t.guid,
                        )
                    )


def validateDesignNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all design names within a kit are unique.
    validateDesignNameUniqueness MUST report duplicate design names as problems.
    """
    problems: list[Problem] = []
    byParent: dict[str | None, list[Design]] = {}
    for d in kit.designs or []:
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
                            constraintId="design-name-unique",
                            message=f'Duplicate design name "{name}" among siblings.',
                            entityKind="Design",
                            entityGuid=d.guid,
                        )
                    )


def validatePieceNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all piece names within a design are unique.
    validatePieceNameUniqueness MUST report duplicate piece names as problems.
    """
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
                            constraintId="piece-name-unique",
                            message=f'Duplicate piece name "{name}" in design.',
                            entityKind="Piece",
                            entityGuid=p.guid,
                        )
                    )


def validatePortNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all port names within a type are unique.
    validatePortNameUniqueness MUST report duplicate port names as problems.
    """
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
                            constraintId="connector-name-unique",
                            message=f'Duplicate connector name "{name}" in type.',
                            entityKind="Connector",
                            entityGuid=connector.guid,
                        )
                    )


def validateModelNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all model names within a type are unique.
    validateModelNameUniqueness MUST report duplicate model names as problems.
    """
    problems: list[Problem] = []
    for t in kit.types or []:
        names: dict[str, list[Model]] = {}
        for model in t.models or []:
            if model.name and model.name not in names:
                names[model.name] = []
            if model.name:
                names[model.name].append(model)
        for name, group in names.items():
            if len(group) > 1:
                for model in group[1:]:
                            constraintId="model-name-unique",
                            message=f'Duplicate model name "{name}" in type.',
                            entityKind="Model",
                            entityGuid=model.guid,
                        )
                    )


def validateQualityNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all quality names within a kit are unique.
    validateQualityNameUniqueness MUST report duplicate quality names as problems.
    """
    problems: list[Problem] = []
    names: dict[str, list[Quality]] = {}
    for q in kit.qualities or []:
        if q.name not in names:
            names[q.name] = []
        names[q.name].append(q)
    for name, group in names.items():
        if len(group) > 1:
            for q in group[1:]:
                        constraintId="quality-name-unique",
                        message=f'Duplicate quality name "{name}".',
                        entityKind="Quality",
                        entityGuid=q.guid,
                    )
                )


def validateFileNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all file names within a kit are unique.
    validateFileNameUniqueness MUST report duplicate file names as problems.
    """
    problems: list[Problem] = []
    names: dict[str, list[File]] = {}
    for f in kit.files_ or []:
        if f.name not in names:
            names[f.name] = []
        names[f.name].append(f)
    for name, group in names.items():
        if len(group) > 1:
            for f in group[1:]:
                        constraintId="file-name-unique",
                        message=f'Duplicate file name "{name}".',
                        entityKind="File",
                        entityGuid=f.guid,
                    )
                )


def validateFolderNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all folder names within a kit are unique.
    validateFolderNameUniqueness MUST report duplicate folder names as problems.
    """
    problems: list[Problem] = []
    byParent: dict[str | None, list[Folder]] = {}
    for fo in kit.folders_ or []:
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
                            constraintId="folder-name-unique",
                            message=f'Duplicate folder name "{name}" among siblings.',
                            entityKind="Folder",
                            entityGuid=fo.guid,
                        )
                    )


def validateLayerPathUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all layer paths within a design are unique.
    validateLayerPathUniqueness MUST report duplicate layer paths as problems.
    """
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
                            constraintId="layer-path-unique",
                            message=f'Duplicate layer path "{path}" in design.',
                            entityKind="Layer",
                            entityGuid=layer.guid,
                        )
                    )


def validateKit(kit: Kit) -> ValidationResult:
    """Validate a kit entity against all constraint rules.
    validateKit MUST run all validation checks and return aggregated results.
    """
    problems: list[Problem] = []
    problems.extend(validateGuidUniqueness(kit))
    problems.extend(validateTypeNameUniqueness(kit))
    problems.extend(validateDesignNameUniqueness(kit))
    problems.extend(validatePieceNameUniqueness(kit))
    problems.extend(validatePortNameUniqueness(kit))
    problems.extend(validateModelNameUniqueness(kit))
    problems.extend(validateQualityNameUniqueness(kit))
    problems.extend(validateFolderNameUniqueness(kit))
    problems.extend(validateLayerPathUniqueness(kit))
    return ValidationResult(problems=problems)


# region Dict-based Validation
# [👤semio📚py💻semio🔖domain🔖validation🔖dictbasedvalidation](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Dict-based%20Validation)
# Dictionary-based validation functions for kit data integrity.


def _makeFix(title: str, diff: dict) -> ValidationFix:
    """
    return ValidationFix(title=title, diff=diff)


def _deepCopy(obj: typing.Any) -> typing.Any:
    """
    return json.loads(json.dumps(obj))


def _newGuid() -> str:
    """
    import uuid

    return str(uuid.uuid4())


def validateKitDict(kit: dict) -> ValidationResult:
    """Validate a kit dictionary against all constraint rules.
    validateKitDict MUST validate a kit dictionary and return results.
    """
    problems: list[Problem] = []
    seen: dict[str, str] = {}
    seenEntities: dict[str, dict] = {}

    def checkGuid(entityKind: str, entityGuid: str, entity: dict) -> None:
        if entityGuid in seen:
            newGuid = _newGuid()
            entityCopy = _deepCopy(entity)
                "Type": "types",
                "Design": "designs",
                "Piece": "pieces",
                "Connection": "connections",
                "Connector": "connectors",
                "Model": "models",
                "Quality": "qualities",
                "Port": "ports",
                "File": "files",
                "Folder": "folders",
                "Stat": "stats",
                "Layer": "layers",
            }.get(entityKind, "")
            if collectionKey:
                        "removed": [{"guid": entityGuid}],
                        "added": [entityCopy],
                    }
                }
                fix = _makeFix("Regenerate GUID", diff)
                        constraintId="guid-unique",
                        message=f'Duplicate GUID "{entityGuid}". First occurrence kept.',
                        entityKind=entityKind,
                        entityGuid=entityGuid,
                        fixes=[fix],
                    )
                )
            else:
                        constraintId="guid-unique",
                        message=f'Duplicate GUID "{entityGuid}". First occurrence kept.',
                        entityKind=entityKind,
                        entityGuid=entityGuid,
                        fixes=[],
                    )
                )
        else:

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
    for i in kit.get("ports", []):
        checkGuid("Port", i.get("guid", ""), i)
    for f in kit.get("files", []):
        checkGuid("File", f.get("guid", ""), f)
    for fo in kit.get("folders", []):
        checkGuid("Folder", fo.get("guid", ""), fo)
    byParent: dict[str | None, list[dict]] = {}
    for t in kit.get("types", []):
        parent = t.get("parent")
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
                        f'Rename "{name}"',
                                        "type": {"guid": t.get("guid", "")},
                                        "diff": {"name": f"{name} 2"},
                                    }
                                ]
                            }
                        },
                    )
                            constraintId="type-name-unique",
                            message=f'Duplicate type name "{name}" among siblings.',
                            entityKind="Type",
                            entityGuid=t.get("guid", ""),
                            fixes=[fix],
                        )
                    )
    byParent = {}
    for d in kit.get("designs", []):
        parent = d.get("parent")
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
                        f'Rename "{name}"',
                                        "design": {"guid": d.get("guid", "")},
                                        "diff": {"name": f"{name} 2"},
                                    }
                                ]
                            }
                        },
                    )
                            constraintId="design-name-unique",
                            message=f'Duplicate design name "{name}" among siblings.',
                            entityKind="Design",
                            entityGuid=d.get("guid", ""),
                            fixes=[fix],
                        )
                    )
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
                        f'Rename piece "{name}"',
                                        "design": {"guid": designGuid},
                                                        "piece": {"guid": p.get("guid", "")},
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
                            constraintId="piece-name-unique",
                            message=f'Duplicate piece name "{name}" inside design "{designName}".',
                            entityKind="Piece",
                            entityGuid=p.get("guid", ""),
                            fixes=[fix],
                        )
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
                        f'Rename connector "{name}"',
                                        "type": {"guid": typeGuid},
                                                        "connector": {"guid": connector.get("guid", "")},
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
                            constraintId="connector-name-unique",
                            message=f'Duplicate connector name "{name}" inside type "{typeName}".',
                            entityKind="Connector",
                            entityGuid=connector.get("guid", ""),
                            fixes=[fix],
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
                        f'Rename model "{name}"',
                                        "type": {"guid": typeGuid},
                                                        "model": {"guid": model.get("guid", "")},
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
                            constraintId="model-name-unique",
                            message=f'Duplicate model name "{name}" inside type "{typeName}".',
                            entityKind="Model",
                            entityGuid=model.get("guid", ""),
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
                    f'Rename quality "{name}"',
                                    "quality": {"guid": q.get("guid", "")},
                                    "diff": {"name": f"{name} 2"},
                                }
                            ]
                        }
                    },
                )
                        constraintId="quality-name-unique",
                        message=f'Duplicate quality name "{name}".',
                        entityKind="Quality",
                        entityGuid=q.get("guid", ""),
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
                    f'Rename port "{name}"',
                                    "port": {"guid": iface.get("guid", "")},
                                    "diff": {"name": f"{name} 2"},
                                }
                            ]
                        }
                    },
                )
                        constraintId="port-name-unique",
                        message=f'Duplicate port name "{name}".',
                        entityKind="Port",
                        entityGuid=iface.get("guid", ""),
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
                    f'Rename file "{name}"',
                                    "file": {"guid": f.get("guid", "")},
                                    "diff": {"name": f"{name} 2"},
                                }
                            ]
                        }
                    },
                )
                        constraintId="file-name-unique",
                        message=f'Duplicate file name "{name}".',
                        entityKind="File",
                        entityGuid=f.get("guid", ""),
                        fixes=[fix],
                    )
                )
    byParent = {}
    for fo in kit.get("folders", []):
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
                        f'Rename folder "{name}"',
                                        "folder": {"guid": fo.get("guid", "")},
                                        "diff": {"name": f"{name} 2"},
                                    }
                                ]
                            }
                        },
                    )
                            constraintId="folder-name-unique",
                            message=f'Duplicate folder name "{name}" among siblings.',
                            entityKind="Folder",
                            entityGuid=fo.get("guid", ""),
                            fixes=[fix],
                        )
                    )
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
                        f'Rename layer "{path}"',
                                        "design": {"guid": designGuid},
                                                        "layer": {"guid": layer.get("guid", "")},
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
                            constraintId="layer-path-unique",
                            message=f'Duplicate layer path "{path}" inside design "{designName}".',
                            entityKind="Layer",
                            entityGuid=layer.get("guid", ""),
                            fixes=[fix],
                        )
                    )
    return ValidationResult(problems=problems)


# #endregion 🔖Dict-based Validation

# region Graph Operations
# [👤semio📚py💻semio🔖domain🔖validation🔖graphoperations](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Graph%20Operations)
# Graph construction and traversal for piece connectivity analysis.


def buildPieceGraph(design: Design | dict) -> networkx.Graph:
    """Build a networkx graph from pieces and connections.
    buildPieceGraph MUST return a networkx graph with pieces as nodes.
    """
    G = networkx.Graph()
    for piece in pieces:
        G.add_node(pieceGuid, piece=piece)
    for connection in connections:
        if isinstance(connection, dict):
            sourceId = connection["connected"]["piece"]["guid"]
            targetId = connection["connecting"]["piece"]["guid"]
        else:
        if G.has_node(sourceId) and G.has_node(targetId):
            G.add_edge(sourceId, targetId, connection=connection)


def findFixedPieces(design: Design | dict) -> list[str]:
    """Find all pieces that are fixed in the design hierarchy.
    findFixedPieces MUST return pieces that have both plane and center defined.
    """
    result = []
    for p in pieces:
        if isinstance(p, dict):
            if hasPlane != hasCenter:
                raise ValueError(f"Piece {p.get('guid')} has inconsistent plane and center")
            if hasPlane:
                result.append(p["guid"])
        else:
            if hasPlane != hasCenter:
                raise ValueError(f"Piece {p.guid} has inconsistent plane and center")
            if hasPlane:
                result.append(p.guid)


def getConnectedComponents(design: Design | dict) -> list[set[str]]:
    """Get connected components of the piece graph.
    getConnectedComponents MUST return disjoint piece groups.
    """
    G = buildPieceGraph(design)


def getPieceHierarchy(design: Design | dict, rootGuid: str) -> dict[str, int]:
    """Get the hierarchical ordering of pieces from root to leaf.
    getPieceHierarchy MUST return a topological ordering of pieces.
    """
    G = buildPieceGraph(design)
    if rootGuid not in G:
        return {}


# endregion Graph Operations

# region FlattenDesign
# [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign)
# Design flattening to resolve nested sub-designs into a single coordinate space.


def getTypeByGuid(kit: dict, guid: str) -> dict | None:
    """Look up a type by its GUID within a kit dictionary.
    getTypeByGuid MUST return the type dict or raise if not found.
    """
    for t in kit.get("types", []):
        if t.get("guid") == guid:


def getConnectorFromType(kit: dict, typeData: dict | None, connectorGuid: str | None) -> dict | None:
    """Look up a connector by name from a type dictionary.
    getConnectorFromType MUST return the matching connector dict.
    """
    if typeData is None:
    if connectorGuid is None:
        connectors = typeData.get("connectors", [])
        if connectors:
            return connectors[0]
        parent = typeData.get("parent")
        if parent:
            parentType = getTypeByGuid(kit, parent.get("guid", ""))
            return getConnectorFromType(kit, parentType, connectorGuid)
    for connector in typeData.get("connectors", []):
        if connector.get("guid") == connectorGuid:
    parent = typeData.get("parent")
    if parent:
        parentType = getTypeByGuid(kit, parent.get("guid", ""))
        return getConnectorFromType(kit, parentType, connectorGuid)
    connectors = typeData.get("connectors", [])
    if connectors:
        return connectors[0]


def planeToMatrixDict(plane: dict) -> numpy.ndarray:
    """Convert a plane dictionary to a 4x4 transformation matrix.
    planeToMatrixDict MUST produce a valid 4x4 homogeneous matrix.
    """
    origin = numpy.array([plane["origin"]["x"], plane["origin"]["y"], plane["origin"]["z"]])
    xAxis = numpy.array([plane["xAxis"]["x"], plane["xAxis"]["y"], plane["xAxis"]["z"]])
    yAxis = numpy.array([plane["yAxis"]["x"], plane["yAxis"]["y"], plane["yAxis"]["z"]])
    zAxis = numpy.cross(xAxis, yAxis)
    zAxis = normalizeVector(zAxis)
    matrix = numpy.eye(4)


def matrixToPlaneDict(matrix: numpy.ndarray) -> dict:
    """Convert a 4x4 transformation matrix to a plane dictionary.
    matrixToPlaneDict MUST extract origin, xAxis and yAxis from the matrix.
    """
    origin = matrix[:3, 3]
    xAxis = matrix[:3, 0]
    yAxis = matrix[:3, 1]
        "origin": {"x": float(origin[0]), "y": float(origin[1]), "z": float(origin[2])},
        "xAxis": {"x": float(xAxis[0]), "y": float(xAxis[1]), "z": float(xAxis[2])},
        "yAxis": {"x": float(yAxis[0]), "y": float(yAxis[1]), "z": float(yAxis[2])},
    }


def quaternionFromUnitVectorsDict(vFrom: numpy.ndarray, vTo: numpy.ndarray) -> numpy.ndarray:
    """Compute a quaternion rotating one unit vector onto another.
    quaternionFromUnitVectorsDict MUST compute the shortest rotation quaternion.
    """
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
    """Compute a quaternion from an axis-angle representation.
    quaternionFromAxisAngleDict MUST compute the quaternion for the given rotation.
    """
    s = numpy.sin(halfAngle)
    return numpy.array([axis[0] * s, axis[1] * s, axis[2] * s, numpy.cos(halfAngle)])


def quaternionToMatrixDict(q: numpy.ndarray) -> numpy.ndarray:
    """Convert a quaternion to a 3x3 rotation matrix.
    quaternionToMatrixDict MUST produce a valid 3x3 rotation matrix.
    """
    m = numpy.eye(4)
    m[0, 0] = 1 - (yy + zz)
    m[1, 1] = 1 - (xx + zz)
    m[2, 2] = 1 - (xx + yy)


def makeRotationAxisDict(axis: numpy.ndarray, angle: float) -> numpy.ndarray:
    """Create a 4x4 rotation matrix around an arbitrary axis.
    makeRotationAxisDict MUST return a 4x4 rotation matrix around the axis.
    """
    return quaternionToMatrixDict(quaternionFromAxisAngleDict(axis, angle))


def makeTranslationDict(x: float, y: float, z: float) -> numpy.ndarray:
    """Create a 4x4 translation matrix from a displacement vector.
    makeTranslationDict MUST return a 4x4 translation matrix.
    """
    m = numpy.eye(4)


def applyMatrix4ToVec3Dict(m: numpy.ndarray, v: numpy.ndarray) -> numpy.ndarray:
    """Apply a 4x4 matrix to a 3D vector dictionary.
    applyMatrix4ToVec3Dict MUST apply the full affine transformation.
    """
            m[0, 0] * v[0] + m[0, 1] * v[1] + m[0, 2] * v[2],
            m[1, 0] * v[0] + m[1, 1] * v[1] + m[1, 2] * v[2],
            m[2, 0] * v[0] + m[2, 1] * v[1] + m[2, 2] * v[2],
        ]
    )


def computeChildPlaneDict(parentPlane: dict, parentConnector: dict, childConnector: dict, connection: dict) -> dict:
    """Compute the world-space plane of a child piece from parent and local planes.
    computeChildPlaneDict MUST compose parent and local transformations correctly.
    """
    parentMatrix = planeToMatrixDict(parentPlane)
            parentConnector["point"]["x"],
            parentConnector["point"]["y"],
            parentConnector["point"]["z"],
        ]
    )
                parentConnector["direction"]["x"],
                parentConnector["direction"]["y"],
                parentConnector["direction"]["z"],
            ]
        )
    )
            childConnector["point"]["x"],
            childConnector["point"]["y"],
            childConnector["point"]["z"],
        ]
    )
                childConnector["direction"]["x"],
                childConnector["direction"]["y"],
                childConnector["direction"]["z"],
            ]
        )
    )
    rotationRad = numpy.deg2rad(rotation)
    turnRad = numpy.deg2rad(turn)
    tiltRad = numpy.deg2rad(tilt)
    crossVec = numpy.cross(parentDirection, reverseChildDirection)
    crossLen = numpy.linalg.norm(crossVec)
    if crossLen < 0.01:
        if abs(parentDirection[2]) < TOLERANCE:
            alignQuat = quaternionFromAxisAngleDict(numpy.array([0.0, 0.0, 1.0]), numpy.pi)
        else:
            axis = normalizeVector(numpy.cross(numpy.array([0.0, 0.0, 1.0]), parentDirection))
            alignQuat = quaternionFromAxisAngleDict(axis, numpy.pi)
    else:
        alignQuat = quaternionFromUnitVectorsDict(reverseChildDirection, parentDirection)
    directionT = quaternionToMatrixDict(alignQuat)
    yAxis = numpy.array([0.0, 1.0, 0.0])
    parentConnectorQuat = quaternionFromUnitVectorsDict(yAxis, parentDirection)
    parentRotationT = quaternionToMatrixDict(parentConnectorQuat)
    gapDirection = applyMatrix4ToVec3Dict(parentRotationT, numpy.array([0.0, 1.0, 0.0]))
    shiftDirection = applyMatrix4ToVec3Dict(parentRotationT, numpy.array([1.0, 0.0, 0.0]))
    raiseDirection = applyMatrix4ToVec3Dict(parentRotationT, numpy.array([0.0, 0.0, 1.0]))
    turnAxis = applyMatrix4ToVec3Dict(parentRotationT, numpy.array([0.0, 0.0, 1.0]))
    tiltAxis = applyMatrix4ToVec3Dict(parentRotationT, numpy.array([1.0, 0.0, 0.0]))
    orientationT = directionT.copy()
    rotateT = makeRotationAxisDict(parentDirection, -rotationRad)
    turnAxis = applyMatrix4ToVec3Dict(rotateT, turnAxis)
    tiltAxis = applyMatrix4ToVec3Dict(rotateT, tiltAxis)
    turnT = makeRotationAxisDict(turnAxis, turnRad)
    tiltT = makeRotationAxisDict(tiltAxis, tiltRad)
    centerChildT = makeTranslationDict(-childPoint[0], -childPoint[1], -childPoint[2])
    gapTransform = makeTranslationDict(gapDirection[0] * gap, gapDirection[1] * gap, gapDirection[2] * gap)
    shiftTransform = makeTranslationDict(shiftDirection[0] * shift, shiftDirection[1] * shift, shiftDirection[2] * shift)
    raiseTransform = makeTranslationDict(raiseDirection[0] * rise, raiseDirection[1] * rise, raiseDirection[2] * rise)
    moveToParentT = makeTranslationDict(parentPoint[0], parentPoint[1], parentPoint[2])
    result = matrixToPlaneDict(finalMatrix)
            "x": round(result["origin"]["x"] / TOLERANCE) * TOLERANCE,
            "y": round(result["origin"]["y"] / TOLERANCE) * TOLERANCE,
            "z": round(result["origin"]["z"] / TOLERANCE) * TOLERANCE,
        },
            "x": round(result["xAxis"]["x"] / TOLERANCE) * TOLERANCE,
            "y": round(result["xAxis"]["y"] / TOLERANCE) * TOLERANCE,
            "z": round(result["xAxis"]["z"] / TOLERANCE) * TOLERANCE,
        },
            "x": round(result["yAxis"]["x"] / TOLERANCE) * TOLERANCE,
            "y": round(result["yAxis"]["y"] / TOLERANCE) * TOLERANCE,
            "z": round(result["yAxis"]["z"] / TOLERANCE) * TOLERANCE,
        },
    }


def flattenDesignDict(kit: dict, designGuid: str) -> dict:
    """Flatten a nested design hierarchy into a single flat coordinate space.
    flattenDesignDict MUST resolve all nested designs into world coordinates.
    """
    design = next((d for d in kit.get("designs", []) if d.get("guid") == designGuid), None)
    if design is None:
        raise ValueError(f"Design {designGuid} not found")
    pieces = design.get("pieces", [])
    if not pieces:
        return {}
    pieceMap = {p["guid"]: dict(p) for p in pieces}
    piecePlanes: dict[str, dict] = {}
    piecePaths: dict[str, str] = {}
    G = buildPieceGraph(design)
    components = list(networkx.connected_components(G))
    for component in components:
        for nodeId in component:
            piece = pieceMap.get(nodeId)
            if piece and piece.get("plane") is not None and piece.get("center") is not None:
        if rootNode is None and component:
            rootNode = next(iter(component))
        if rootNode is None:
        rootPiece = pieceMap[rootNode]
        if rootPiece.get("plane") and rootPiece.get("center") is not None:
            piecePlanes[rootNode] = rootPiece["plane"]
        else:
                "origin": {"x": 0, "y": 0, "z": 0},
                "xAxis": {"x": 1, "y": 0, "z": 0},
                "yAxis": {"x": 0, "y": 1, "z": 0},
            }
        for source, target in networkx.bfs_edges(G, rootNode):
            if target in piecePlanes:
            parentPlane = piecePlanes.get(parentId)
            if parentPlane is None:
            edgeData = G.get_edge_data(parentId, childId)
            if connection is None:
            parentPiece = pieceMap[parentId]
            childPiece = pieceMap[childId]
            parentType = getTypeByGuid(kit, parentPiece.get("type", {}).get("guid", ""))
            childType = getTypeByGuid(kit, childPiece.get("type", {}).get("guid", ""))
            parentSide = connection["connected"] if connection["connected"]["piece"]["guid"] == parentId else connection["connecting"]
            childSide = connection["connecting"] if connection["connecting"]["piece"]["guid"] == childId else connection["connected"]
            parentConnector = getConnectorFromType(kit, parentType, parentConnectorGuid)
            childConnector = getConnectorFromType(kit, childType, childConnectorGuid)
            if parentConnector is None or childConnector is None:
            childPlane = computeChildPlaneDict(parentPlane, parentConnector, childConnector, connection)
            parentCenter = parentPiece.get("center") or {"u": 0, "v": 0}
            if parentCenter["u"] == 0 and parentCenter["v"] == 0:
                childU = radius * numpy.sin(angle)
                childV = radius * numpy.cos(angle)
            else:
                if isVerticalConnection:
                else:
                "u": round(childU / TOLERANCE) * TOLERANCE,
                "v": round(childV / TOLERANCE) * TOLERANCE,
            }
    updatedPieces = []
    for piece in pieces:
        newPiece = dict(piece)
        if piece["guid"] in piecePlanes:
            newPiece["plane"] = piecePlanes[piece["guid"]]
        if piece["guid"] in pieceMap and pieceMap[piece["guid"]].get("center"):
            newPiece["center"] = pieceMap[piece["guid"]]["center"]
        elif newPiece.get("center") is None:
            newPiece["center"] = {"u": 0, "v": 0}
        updatedPieces.append(newPiece)
                    "id": p["guid"],
                    "diff": {"plane": p.get("plane"), "center": p.get("center")},
                }
            ]
        },
        "_piecePaths": piecePaths,
    }


# endregion FlattenDesign

# region Kit Operations
# [👤semio📚py💻semio🔖domain🔖kitoperations](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations)
# Dict-based pure functions for kit operations exposed via MCP.


def findAttributeValueDict(entity: dict, name: str, defaultValue: typing.Any = ...) -> typing.Optional[str]:
    """Finds an attribute value on an entity by key.
    Returns default if not found, raises ValueError if no default provided.
    """
    attributes = entity.get("attributes") or []
    attribute = next((a for a in attributes if a.get("key") == name), None)
    if attribute is None and defaultValue is ...:
        raise ValueError(f"Attribute {name} not found")
    if attribute is None:
    value = attribute.get("value")
    if value is None and defaultValue is None:
    return value if value is not None else (defaultValue if defaultValue is not ... else "")


def _findDesignInKitDict(kit: dict, design_guid: str) -> dict:
    """Finds a design by GUID in a kit dict."""
    for d in kit.get("designs", []):
        if d.get("guid") == design_guid:
    raise ValueError(f"Design {design_guid} not found in kit")


def _findTypeInKitDict(kit: dict, type_guid: str) -> dict:
    """Finds a type by GUID in a kit dict."""
    for t in kit.get("types", []):
        if t.get("guid") == type_guid:
    raise ValueError(f"Type {type_guid} not found in kit")


def _findPieceInDesignDict(design: dict, piece_guid: str) -> dict:
    """Finds a piece by GUID in a design dict."""
    for p in design.get("pieces", []):
        if p.get("guid") == piece_guid:
    raise ValueError(f"Piece {piece_guid} not found in design")


def _findPieceConnectionsInDesignDict(design: dict, piece_guid: str) -> list[dict]:
    """Finds all connections involving a piece in a design dict."""
    return [c for c in design.get("connections", []) if c.get("connected", {}).get("piece", {}).get("guid") == piece_guid or c.get("connecting", {}).get("piece", {}).get("guid") == piece_guid]


def _findConnectorInTypeDict(type_dict: dict, connector_guid: str) -> dict:
    """Finds a connector by GUID in a type dict."""
    for c in type_dict.get("connectors", []):
        if c.get("guid") == connector_guid:
    raise ValueError(f"Connector {connector_guid} not found in type")


def _applyDesignDiffDict(base: dict, diff: dict) -> dict:
    """Applies a design diff to a base design dict, returning a new design dict."""
    import copy

    result = copy.deepcopy(base)
    pieces_diff = diff.get("pieces")
    if pieces_diff:
        pieces = list(result.get("pieces", []))
        for added in pieces_diff.get("added", []):
            pieces.append(added)
        for removed in pieces_diff.get("removed", []):
            pieces = [p for p in pieces if p.get("guid") != removed_guid]
        for updated in pieces_diff.get("updated", []):
            piece_id = updated.get("id") or updated.get("piece", {}).get("guid")
            piece_diff = updated.get("diff", {})
            for i, p in enumerate(pieces):
                if p.get("guid") == piece_id:
                        **p,
                        **{k: v for k, v in piece_diff.items() if v is not None},
                    }
    connections_diff = diff.get("connections")
    if connections_diff:
        connections = list(result.get("connections", []))
        for added in connections_diff.get("added", []):
            connections.append(added)
        for removed in connections_diff.get("removed", []):
            connections = [c for c in connections if c.get("guid") != removed_guid]
        for updated in connections_diff.get("updated", []):
            conn_id = updated.get("id") or updated.get("connection", {}).get("guid")
            conn_diff = updated.get("diff", {})
            for i, c in enumerate(connections):
                if c.get("guid") == conn_id:
                        **c,
                        **{k: v for k, v in conn_diff.items() if v is not None},
                    }
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
            result[key] = diff[key]


def piecesMetadataDict(kit: dict, design_guid: str) -> dict:
    """Returns metadata for all pieces in a design.
    Each entry contains plane, center, fixedPieceId, parentPieceId, depth, and path.
    """
    design = _findDesignInKitDict(kit, design_guid)
    flatten_diff = flattenDesignDict(kit, design_guid)
    piece_paths = flatten_diff.pop("_piecePaths", {})
    flat_design = _applyDesignDiffDict(design, flatten_diff)
    result = {}
    for p in flat_design.get("pieces", []):
        guid = p.get("guid", "")
        path_raw = piece_paths.get(guid, guid)
            "plane": p.get("plane"),
            "center": p.get("center", {"u": 0, "v": 0}),
            "fixedPieceId": findAttributeValueDict(p, "semio.fixedPieceId", guid) or guid,
            "parentPieceId": findAttributeValueDict(p, "semio.parentPieceId", None),
            "depth": int(findAttributeValueDict(p, "semio.depth", "0") or "0"),
            "path": [s for s in path_raw.split(",") if s],
        }


# region 🔖Clustering
# [👤semio📚py💻semio🔖domain🔖kitoperations🔖clustering](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Clustering)
# Functions for clustering and expanding design pieces.


def createClusteredDesignDict(original_design: dict, cluster_piece_ids: list[str], design_name: str) -> dict:
    """Creates a new design from a subset of pieces (cluster).
    Returns a dict with 'clusteredDesign' and 'externalConnections'.
    """
    pieces = original_design.get("pieces", [])
    if not pieces:
        raise ValueError("Original design has no pieces to cluster")
    if not cluster_piece_ids:
        raise ValueError("No piece IDs provided for clustering")
    cluster_set = set(cluster_piece_ids)
    clustered_pieces = [p for p in pieces if p.get("guid") in cluster_set]
    if not clustered_pieces:
        raise ValueError("No pieces found matching the provided IDs")
    connections = original_design.get("connections", [])
    internal_connections = [c for c in connections if c.get("connected", {}).get("piece", {}).get("guid") in cluster_set and c.get("connecting", {}).get("piece", {}).get("guid") in cluster_set]
    external_connections = [c for c in connections if (c.get("connected", {}).get("piece", {}).get("guid") in cluster_set) != (c.get("connecting", {}).get("piece", {}).get("guid") in cluster_set)]
    import datetime as dt
    import uuid

    now = dt.datetime.now(dt.timezone.utc).isoformat()
        "guid": str(uuid.uuid4()),
        "name": design_name,
        "unit": original_design.get("unit"),
        "description": f"Clustered design with {len(clustered_pieces)} pieces",
        "pieces": clustered_pieces,
        "connections": internal_connections,
        "createdAt": now,
        "updatedAt": now,
    }
        "clusteredDesign": clustered_design,
        "externalConnections": external_connections,
    }


def replaceClusterWithDesignDict(
    original_design: dict,
    cluster_piece_ids: list[str],
    clustered_design: dict,
    external_connections: list[dict],
) -> dict:
    """Returns a DesignDiff that replaces clustered pieces with a design reference.
    """
    cluster_set = set(cluster_piece_ids)
    pieces_to_remove = [{"guid": guid} for guid in cluster_piece_ids]
    connections = original_design.get("connections", [])
    connections_to_remove = [{"guid": c.get("guid")} for c in connections if c.get("connected", {}).get("piece", {}).get("guid") in cluster_set or c.get("connecting", {}).get("piece", {}).get("guid") in cluster_set]
    updated_external = []
    for connection in external_connections:
        import copy

        new_conn = copy.deepcopy(connection)
        if connected_in_cluster:
            new_conn.setdefault("connected", {})["designPiece"] = {"guid": clustered_design.get("guid")}
        elif connecting_in_cluster:
            new_conn.setdefault("connecting", {})["designPiece"] = {"guid": clustered_design.get("guid")}
        updated_external.append(new_conn)
        "pieces": {"removed": pieces_to_remove},
        "connections": {"removed": connections_to_remove, "added": updated_external},
    }


def getClusterableGroupsDict(design: dict, selected_piece_ids: list[str]) -> list[list[str]]:
    """Returns clusterable groups of selected pieces using DFS on connection graph.
    """
    if len(selected_piece_ids) < 2:
        return []
    adjacency: dict[str, set[str]] = {}
    for connection in design.get("connections", []):
        source_id = connection.get("connecting", {}).get("piece", {}).get("guid", "")
        target_id = connection.get("connected", {}).get("piece", {}).get("guid", "")
        adjacency.setdefault(source_id, set()).add(target_id)
        adjacency.setdefault(target_id, set()).add(source_id)
    selected_set = set(selected_piece_ids)
    visited: set[str] = set()
    connected_groups: list[list[str]] = []

    def dfs(piece_id: str, current_group: list[str]) -> None:
        if piece_id in visited:
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
    piece_guid_set = set(p.get("guid", "") for p in design.get("pieces", []))
    has_design_nodes = any(pid not in piece_guid_set for pid in selected_piece_ids)
    has_large_connected_group = any(len(g) > 1 for g in connected_groups)
    if has_design_nodes or has_multiple_components or has_large_connected_group:
        return [selected_piece_ids]
    return []


def expandDesignPiecesDict(design: dict, kit: dict) -> dict:
    """Recursively expands design references (designPiece) by inlining their pieces and connections.
    """
    import copy

    connections = design.get("connections", [])
    has_design_connections = any(c.get("connected", {}).get("designPiece") or c.get("connecting", {}).get("designPiece") for c in connections)
    if not has_design_connections:
    expanded = copy.deepcopy(design)
    design_ids: set[str] = set()
    for conn in connections:
        dp = conn.get("connected", {}).get("designPiece")
        if dp:
            design_ids.add(dp.get("guid", ""))
        dp = conn.get("connecting", {}).get("designPiece")
        if dp:
            design_ids.add(dp.get("guid", ""))
    if not design_ids:
    for design_ref_guid in design_ids:
            (d for d in kit.get("designs", []) if d.get("guid") == design_ref_guid),
            None,
        )
        if not referenced:
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
            connected_dp = new_conn.get("connected", {}).get("designPiece")
            if connected_dp and connected_dp.get("guid") == design_ref_guid:
                new_conn["connected"].pop("designPiece", None)
            connecting_dp = new_conn.get("connecting", {}).get("designPiece")
            if connecting_dp and connecting_dp.get("guid") == design_ref_guid:
                new_conn["connecting"].pop("designPiece", None)
            updated_connections.append(new_conn)


# endregion 🔖Clustering

# region 🔖Kit Query Helpers Dict
# [👤semio📚py💻semio🔖domain🔖kitoperations🔖kitqueryhelpersdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Kit%20Query%20Helpers%20Dict)
# Dict-based kit query helper functions.


def getPrimitiveDesignDict(kit: dict, design_guid: str) -> dict:
    """Gets the primitive (root) design of a design family."""
    current = _findDesignInKitDict(kit, design_guid)
    while current.get("parent", {}).get("guid"):
        current = _findDesignInKitDict(kit, current["parent"]["guid"])


def getDesignFamilyDict(kit: dict, design_guid: str) -> list[dict]:
    """Gets all designs in a design family (the entire tree)."""
    primitive = getPrimitiveDesignDict(kit, design_guid)
    family: list[dict] = []

    def collect(parent_guid: str) -> None:
        parent = _findDesignInKitDict(kit, parent_guid)
        family.append(parent)
        children = [d for d in kit.get("designs", []) if d.get("parent", {}).get("guid") == parent_guid]
        for child in children:
            collect(child["guid"])

    collect(primitive["guid"])


def getDesignSiblingsDict(kit: dict, design_guid: str) -> list[dict]:
    """Returns all designs with the same parent, excluding self."""
    design = _findDesignInKitDict(kit, design_guid)
    parent_guid = design.get("parent", {}).get("guid")
    return [d for d in kit.get("designs", []) if d.get("parent", {}).get("guid") == parent_guid and d.get("guid") != design_guid]


def getDesignChildrenDict(kit: dict, design_guid: str) -> list[dict]:
    """Returns all direct children of a design."""
    return [d for d in kit.get("designs", []) if d.get("parent", {}).get("guid") == design_guid]


def areDesignsInSameFamilyDict(kit: dict, design_guid_a: str, design_guid_b: str) -> bool:
    """Checks if two designs share the same primitive ancestor."""
    return getPrimitiveDesignDict(kit, design_guid_a).get("guid") == getPrimitiveDesignDict(kit, design_guid_b).get("guid")


def canUseDesignAsPieceDict(kit: dict, container_design_guid: str, piece_design_guid: str) -> bool:
    """Returns true if a design can be used as a piece (must NOT be in same family)."""
    return not areDesignsInSameFamilyDict(kit, container_design_guid, piece_design_guid)


def findSameFamilyDesignPiecesDict(kit: dict, design_guid: str) -> list[dict]:
    """Returns all pieces in a design that reference designs from the same family."""
    design = _findDesignInKitDict(kit, design_guid)
    return [p for p in design.get("pieces", []) if p.get("design", {}).get("guid") and areDesignsInSameFamilyDict(kit, design_guid, p["design"]["guid"])]


def getPrimitiveTypeDict(kit: dict, type_guid: str) -> dict:
    """Gets the primitive (root) type of a type family."""
    current = _findTypeInKitDict(kit, type_guid)
    while current.get("parent", {}).get("guid"):
        current = _findTypeInKitDict(kit, current["parent"]["guid"])


def getTypeFamilyDict(kit: dict, type_guid: str) -> list[dict]:
    """Gets all types in a type family (the entire tree)."""
    primitive = getPrimitiveTypeDict(kit, type_guid)
    family: list[dict] = []

    def collect(parent_guid: str) -> None:
        parent = _findTypeInKitDict(kit, parent_guid)
        family.append(parent)
        children = [t for t in kit.get("types", []) if t.get("parent", {}).get("guid") == parent_guid]
        for child in children:
            collect(child["guid"])

    collect(primitive["guid"])


def getTypeSiblingsDict(kit: dict, type_guid: str) -> list[dict]:
    """Returns all types with the same parent, excluding self."""
    type_ = _findTypeInKitDict(kit, type_guid)
    parent_guid = type_.get("parent", {}).get("guid")
    return [t for t in kit.get("types", []) if t.get("parent", {}).get("guid") == parent_guid and t.get("guid") != type_guid]


def getTypeChildrenDict(kit: dict, type_guid: str) -> list[dict]:
    """Returns all direct children of a type."""
    return [t for t in kit.get("types", []) if t.get("parent", {}).get("guid") == type_guid]


def areTypesInSameFamilyDict(kit: dict, type_guid_a: str, type_guid_b: str) -> bool:
    """Checks if two types share the same primitive ancestor."""
    return getPrimitiveTypeDict(kit, type_guid_a).get("guid") == getPrimitiveTypeDict(kit, type_guid_b).get("guid")


def findPieceTypeInDesignDict(kit: dict, design_guid: str, piece_guid: str) -> dict:
    """Gets the type of a piece in a design."""
    design = _findDesignInKitDict(kit, design_guid)
    piece = _findPieceInDesignDict(design, piece_guid)
    type_ref = piece.get("type", {})
    if not type_ref or not type_ref.get("guid"):
        raise ValueError(f"Piece {piece_guid} has no type")
    return _findTypeInKitDict(kit, type_ref["guid"])


def findUsedConnectorsByPieceInDesignDict(kit: dict, design_guid: str, piece_guid: str) -> list[dict]:
    """Returns all connectors of a piece that are used in connections."""
    design = _findDesignInKitDict(kit, design_guid)
    piece = _findPieceInDesignDict(design, piece_guid)
    type_ref = piece.get("type", {})
    if not type_ref or not type_ref.get("guid"):
        return []
    type_dict = _findTypeInKitDict(kit, type_ref["guid"])
    connections = _findPieceConnectionsInDesignDict(design, piece_guid)
    result = []
    for c in connections:
        if c.get("connected", {}).get("piece", {}).get("guid") == piece_guid:
            connector_guid = (c.get("connected", {}).get("connector") or {}).get("guid")
        else:
            connector_guid = (c.get("connecting", {}).get("connector") or {}).get("guid")
        if connector_guid:
            try:
                result.append(_findConnectorInTypeDict(type_dict, connector_guid))
            except ValueError:


def findReplaceableTypesForPieceInDesignDict(
    kit: dict,
    design_guid: str,
    piece_guid: str,
    variants: typing.Optional[list[str]] = None,
) -> list[dict]:
    """Finds all types that can replace a piece while maintaining connection compatibility.
    """
    design = _findDesignInKitDict(kit, design_guid)
    connections = _findPieceConnectionsInDesignDict(design, piece_guid)
    required_connectors: list[dict] = []
    for connection in connections:
        try:
            connected_guid = connection.get("connected", {}).get("piece", {}).get("guid")
            connecting_guid = connection.get("connecting", {}).get("piece", {}).get("guid")
            other_piece = _findPieceInDesignDict(design, other_piece_guid)
            other_type_guid = (other_piece.get("type") or {}).get("guid")
            if not other_type_guid:
            other_type = _findTypeInKitDict(kit, other_type_guid)
            if connected_guid == piece_guid:
                other_connector_guid = (connection.get("connecting", {}).get("connector") or {}).get("guid")
            else:
                other_connector_guid = (connection.get("connected", {}).get("connector") or {}).get("guid")
            if not other_connector_guid:
            other_connector = _findConnectorInTypeDict(other_type, other_connector_guid)
            required_connectors.append(other_connector)
        except ValueError, AttributeError, KeyError:
    result = []
    for replacement_type in kit.get("types", []):
        if replacement_type.get("isAbstract"):
        if variants is not None:
            parent_guid = (replacement_type.get("parent") or {}).get("guid", "")
            if parent_guid not in variants:
        type_connectors = replacement_type.get("connectors") or []
        if len(type_connectors) == 0:
            if len(required_connectors) == 0:
                result.append(replacement_type)
        if all(len(type_connectors) > 0 for _ in required_connectors):
            result.append(replacement_type)


def findReplaceableTypesForPiecesInDesignDict(
    kit: dict,
    design_guid: str,
    piece_guids: list[str],
    variants: typing.Optional[list[str]] = None,
) -> list[dict]:
    """Finds types that can replace multiple pieces while maintaining all external connections.
    """
    design = _findDesignInKitDict(kit, design_guid)
    piece_set = set(piece_guids)
    external_connectors: list[dict] = []
    for piece_guid in piece_guids:
        connections = _findPieceConnectionsInDesignDict(design, piece_guid)
        for connection in connections:
            connected_guid = connection.get("connected", {}).get("piece", {}).get("guid")
            connecting_guid = connection.get("connecting", {}).get("piece", {}).get("guid")
            if other_piece_guid not in piece_set:
                try:
                    other_piece = _findPieceInDesignDict(design, other_piece_guid)
                    other_type_guid = (other_piece.get("type") or {}).get("guid")
                    if not other_type_guid:
                    other_type = _findTypeInKitDict(kit, other_type_guid)
                    if connected_guid == piece_guid:
                        other_connector_guid = (connection.get("connecting", {}).get("connector") or {}).get("guid")
                    else:
                        other_connector_guid = (connection.get("connected", {}).get("connector") or {}).get("guid")
                    if not other_connector_guid:
                    other_connector = _findConnectorInTypeDict(other_type, other_connector_guid)
                    external_connectors.append(other_connector)
                except ValueError, AttributeError, KeyError:
    result = []
    for replacement_type in kit.get("types", []):
        if replacement_type.get("isAbstract"):
        if variants is not None:
            parent_guid = (replacement_type.get("parent") or {}).get("guid", "")
            if parent_guid not in variants:
        type_connectors = replacement_type.get("connectors") or []
        if len(type_connectors) == 0:
            if len(external_connectors) == 0:
                result.append(replacement_type)
        if all(len(type_connectors) > 0 for _ in external_connectors):
            result.append(replacement_type)


def sumQualityInDesignDict(kit: dict, design_guid: str, quality_guid: str) -> float:
    """Sums up the values of a quality across all pieces in a design.
    For each piece, uses the piece-level prop if present, otherwise falls back to the type-level prop.
    """
    design = _findDesignInKitDict(kit, design_guid)
    for piece in design.get("pieces", []):
            (p for p in piece.get("props", []) if p.get("quality", {}).get("guid") == quality_guid),
            None,
        )
        if piece_prop is not None:
            total += float(piece_prop.get("value", 0))
        type_ref = piece.get("type", {})
        if type_ref and type_ref.get("guid"):
            try:
                type_dict = _findTypeInKitDict(kit, type_ref["guid"])
                    (p for p in type_dict.get("props", []) if p.get("quality", {}).get("guid") == quality_guid),
                    None,
                )
                if type_prop is not None:
                    total += float(type_prop.get("value", 0))
            except ValueError:


# endregion 🔖Kit Query Helpers Dict

# endregion Kit Operations

# region Kit Diff Operations
# [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations)
# Diffing and patching operations for comparing and merging kit versions.


def _normalizeValue(value: typing.Any) -> typing.Any:
    """Normalize empty values to None for comparison.
    """
    if value is None or value == "" or value == []:


def _normalizeBoolean(value: bool | None) -> bool | None:
    """Normalize boolean: True stays True, False/None become None.
    """


def _normalizeArray(arr: list | None) -> list:
    """Normalize None or single item to list.
    """
    if arr is None:
        return []
    if not isinstance(arr, list):
        return [arr]


def areAttributesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two attribute dictionaries are equal.
    areAttributesEqualDict MUST compare all attribute fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for attrA in arrA:
        attrB = next((x for x in arrB if x.get("guid") == attrA.get("guid")), None)
        if attrB is None:
        if attrA.get("key") != attrB.get("key"):
        if _normalizeValue(attrA.get("value")) != _normalizeValue(attrB.get("value")):
        if _normalizeValue(attrA.get("definition")) != _normalizeValue(attrB.get("definition")):
        if strict:
            if attrA.get("createdAt") != attrB.get("createdAt"):
            if attrA.get("updatedAt") != attrB.get("updatedAt"):


def arePropsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two prop dictionaries are equal.
    arePropsEqualDict MUST compare all prop fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for propA in arrA:
        propB = next((x for x in arrB if x.get("guid") == propA.get("guid")), None)
        if propB is None:
        if propA.get("quality", {}).get("guid") != propB.get("quality", {}).get("guid"):
        if propA.get("value") != propB.get("value"):
        if _normalizeValue(propA.get("unit")) != _normalizeValue(propB.get("unit")):
        if not areAttributesEqualDict(propA.get("attributes"), propB.get("attributes"), strict):
        if strict:
            if propA.get("createdAt") != propB.get("createdAt"):
            if propA.get("updatedAt") != propB.get("updatedAt"):


def areConnectorsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two port dictionaries are equal.
    arePortsEqualDict MUST compare all port fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for connectorA in arrA:
        connectorB = next((x for x in arrB if x.get("guid") == connectorA.get("guid")), None)
        if connectorB is None:
        if _normalizeValue(connectorA.get("name")) != _normalizeValue(connectorB.get("name")):
        pointA = connectorA.get("point", {})
        pointB = connectorB.get("point", {})
        if not _floatEqual(pointA.get("x"), pointB.get("x")) or not _floatEqual(pointA.get("y"), pointB.get("y")) or not _floatEqual(pointA.get("z"), pointB.get("z")):
        dirA = connectorA.get("direction", {})
        dirB = connectorB.get("direction", {})
        if not _floatEqual(dirA.get("x"), dirB.get("x")) or not _floatEqual(dirA.get("y"), dirB.get("y")) or not _floatEqual(dirA.get("z"), dirB.get("z")):
        if not _floatEqual(connectorA.get("t"), connectorB.get("t")):
        if _normalizeBoolean(connectorA.get("mandatory")) != _normalizeBoolean(connectorB.get("mandatory")):
        ifaceA = connectorA.get("port", {}) if connectorA.get("port") else {}
        ifaceB = connectorB.get("port", {}) if connectorB.get("port") else {}
        if _normalizeValue(ifaceA.get("guid")) != _normalizeValue(ifaceB.get("guid")):
        if not arePropsEqualDict(connectorA.get("props"), connectorB.get("props"), strict):
        if not areAttributesEqualDict(connectorA.get("attributes"), connectorB.get("attributes"), strict):
        if strict:
            if connectorA.get("createdAt") != connectorB.get("createdAt"):
            if connectorA.get("updatedAt") != connectorB.get("updatedAt"):


def areModelsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two model dictionaries are equal.
    areModelsEqualDict MUST compare all model fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for modelA in arrA:
        modelB = next((x for x in arrB if x.get("guid") == modelA.get("guid")), None)
        if modelB is None:
        if _normalizeValue(modelA.get("name")) != _normalizeValue(modelB.get("name")):

        fileA = modelA.get("file")
        fileB = modelB.get("file")
        if fileGuidA != fileGuidB:
        tagsA = [t.get("guid") if isinstance(t, dict) else t for t in _normalizeArray(modelA.get("tags"))]
        tagsB = [t.get("guid") if isinstance(t, dict) else t for t in _normalizeArray(modelB.get("tags"))]
        if len(tagsA) != len(tagsB) or set(tagsA) != set(tagsB):
        if not areAttributesEqualDict(modelA.get("attributes"), modelB.get("attributes"), strict):
        if strict:
            if modelA.get("createdAt") != modelB.get("createdAt"):
            if modelA.get("updatedAt") != modelB.get("updatedAt"):


def areTypesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two type dictionaries are equal.
    areTypesEqualDict MUST compare all type fields including children for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for typeA in arrA:
        for t in arrB:
            if t.get("guid") != typeA.get("guid"):
            parentA = typeA.get("parent")
            parentB = t.get("parent")
            if not parentA and not parentB:
            if not parentA or not parentB:

            if parentGuidA == parentGuidB:
        if typeB is None:
        if typeA.get("name") != typeB.get("name"):
        if _normalizeValue(typeA.get("description")) != _normalizeValue(typeB.get("description")):
        if _normalizeValue(typeA.get("icon")) != _normalizeValue(typeB.get("icon")):
        if _normalizeValue(typeA.get("image")) != _normalizeValue(typeB.get("image")):
        if _normalizeValue(typeA.get("folder")) != _normalizeValue(typeB.get("folder")):
        if _normalizeValue(typeA.get("unit")) != _normalizeValue(typeB.get("unit")):
        if typeA.get("stock") != typeB.get("stock"):
        if _normalizeBoolean(typeA.get("isAbstract")) != _normalizeBoolean(typeB.get("isAbstract")):
        if _normalizeBoolean(typeA.get("virtual")) != _normalizeBoolean(typeB.get("virtual")):
        locA = typeA.get("location", {}) if typeA.get("location") else {}
        locB = typeB.get("location", {}) if typeB.get("location") else {}
        if _normalizeValue(locA.get("guid")) != _normalizeValue(locB.get("guid")):

        conceptsA = _normalizeArray(typeA.get("concepts"))
        conceptsB = _normalizeArray(typeB.get("concepts"))
        conceptGuidsA = [c.get("guid") if isinstance(c, dict) else c for c in conceptsA]
        conceptGuidsB = [c.get("guid") if isinstance(c, dict) else c for c in conceptsB]
        if conceptGuidsA != conceptGuidsB:
        authA = [a.get("guid") if isinstance(a, dict) else a for a in _normalizeArray(typeA.get("authors"))]
        authB = [a.get("guid") if isinstance(a, dict) else a for a in _normalizeArray(typeB.get("authors"))]
        if authA != authB:
        if not arePropsEqualDict(typeA.get("props"), typeB.get("props"), strict):
        if not areModelsEqualDict(typeA.get("models"), typeB.get("models"), strict):
        if not areConnectorsEqualDict(typeA.get("connectors"), typeB.get("connectors"), strict):
        if not areAttributesEqualDict(typeA.get("attributes"), typeB.get("attributes"), strict):
        if strict:
            if typeA.get("createdAt") != typeB.get("createdAt"):
            if typeA.get("updatedAt") != typeB.get("updatedAt"):


def arePiecesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two piece dictionaries are equal.
    arePiecesEqualDict MUST compare all piece fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for pieceA in arrA:
        pieceB = next((x for x in arrB if x.get("guid") == pieceA.get("guid")), None)
        if pieceB is None:
        if _normalizeValue(pieceA.get("name")) != _normalizeValue(pieceB.get("name")):

        typeA = pieceA.get("type")
        typeB = pieceB.get("type")
        if typeGuidA != typeGuidB:

        designA = pieceA.get("design")
        designB = pieceB.get("design")
        if designGuidA != designGuidB:
        planeA = pieceA.get("plane")
        planeB = pieceB.get("plane")
        if planeA and planeB:
            if planeA.get("origin", {}).get("x") != planeB.get("origin", {}).get("x"):
            if planeA.get("origin", {}).get("y") != planeB.get("origin", {}).get("y"):
            if planeA.get("origin", {}).get("z") != planeB.get("origin", {}).get("z"):
            if planeA.get("xAxis", {}).get("x") != planeB.get("xAxis", {}).get("x"):
            if planeA.get("xAxis", {}).get("y") != planeB.get("xAxis", {}).get("y"):
            if planeA.get("xAxis", {}).get("z") != planeB.get("xAxis", {}).get("z"):
            if planeA.get("yAxis", {}).get("x") != planeB.get("yAxis", {}).get("x"):
            if planeA.get("yAxis", {}).get("y") != planeB.get("yAxis", {}).get("y"):
            if planeA.get("yAxis", {}).get("z") != planeB.get("yAxis", {}).get("z"):
        elif planeA or planeB:
        centerA = pieceA.get("center")
        centerB = pieceB.get("center")
        if centerA and centerB:
            if centerA.get("u") != centerB.get("u") or centerA.get("v") != centerB.get("v"):
        elif centerA or centerB:
        if pieceA.get("scale") != pieceB.get("scale"):
        if _normalizeBoolean(pieceA.get("isHidden")) != _normalizeBoolean(pieceB.get("isHidden")):
        if _normalizeBoolean(pieceA.get("isLocked")) != _normalizeBoolean(pieceB.get("isLocked")):
        if _normalizeValue(pieceA.get("color")) != _normalizeValue(pieceB.get("color")):
        if _normalizeValue(pieceA.get("description")) != _normalizeValue(pieceB.get("description")):
        if not arePropsEqualDict(pieceA.get("props"), pieceB.get("props"), strict):
        if not areAttributesEqualDict(pieceA.get("attributes"), pieceB.get("attributes"), strict):
        if strict:
            if pieceA.get("createdAt") != pieceB.get("createdAt"):
            if pieceA.get("updatedAt") != pieceB.get("updatedAt"):


def _getGuidFromRef(ref: typing.Any) -> str | None:
    """Extract guid from either a string (Input format) or dict with guid (Output format).
    """
    if ref is None:
    if isinstance(ref, dict):
        return ref.get("guid")


def _floatEqual(a, b, epsilon=1e-9):
    """Compare two float values with epsilon tolerance.
    """
    if a is None and b is None:
    if a is None or b is None:
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):


def areConnectionsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two connection dictionaries are equal.
    areConnectionsEqualDict MUST compare all connection fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for connA in arrA:
        connB = next((x for x in arrB if x.get("guid") == connA.get("guid")), None)
        if connB is None:
        connectedA = connA.get("connected", {})
        connectedB = connB.get("connected", {})

        if _getGuidFromRef(connectedA.get("piece")) != _getGuidFromRef(connectedB.get("piece")):
        if _getGuidFromRef(connectedA.get("designPiece")) != _getGuidFromRef(connectedB.get("designPiece")):
        if _getGuidFromRef(connectedA.get("connector")) != _getGuidFromRef(connectedB.get("connector")):
        connectingA = connA.get("connecting", {})
        connectingB = connB.get("connecting", {})
        if _getGuidFromRef(connectingA.get("piece")) != _getGuidFromRef(connectingB.get("piece")):
        if _getGuidFromRef(connectingA.get("designPiece")) != _getGuidFromRef(connectingB.get("designPiece")):
        if _getGuidFromRef(connectingA.get("connector")) != _getGuidFromRef(connectingB.get("connector")):
        if not _floatEqual(connA.get("gap"), connB.get("gap")):
        if not _floatEqual(connA.get("shift"), connB.get("shift")):
        if not _floatEqual(connA.get("rise"), connB.get("rise")):
        if not _floatEqual(connA.get("rotation"), connB.get("rotation")):
        if not _floatEqual(connA.get("turn"), connB.get("turn")):
        if not _floatEqual(connA.get("tilt"), connB.get("tilt")):
        if not _floatEqual(connA.get("u"), connB.get("u")):
        if not _floatEqual(connA.get("v"), connB.get("v")):
        if _normalizeValue(connA.get("description")) != _normalizeValue(connB.get("description")):
        if not areAttributesEqualDict(connA.get("attributes"), connB.get("attributes"), strict):
        if strict:
            if connA.get("createdAt") != connB.get("createdAt"):
            if connA.get("updatedAt") != connB.get("updatedAt"):


def areDesignsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two design dictionaries are equal.
    areDesignsEqualDict MUST compare all design fields including children for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for designA in arrA:
        for d in arrB:
            if d.get("guid") != designA.get("guid"):
            parentA = designA.get("parent")
            parentB = d.get("parent")
            if not parentA and not parentB:
            if not parentA or not parentB:

            parentGuidA = _getGuidFromRef(parentA)
            parentGuidB = _getGuidFromRef(parentB)
            if parentGuidA == parentGuidB:
        if designB is None:
        if designA.get("name") != designB.get("name"):
        if _normalizeValue(designA.get("description")) != _normalizeValue(designB.get("description")):
        if _normalizeValue(designA.get("icon")) != _normalizeValue(designB.get("icon")):
        if _normalizeValue(designA.get("image")) != _normalizeValue(designB.get("image")):

        conceptsA = _normalizeArray(designA.get("concepts"))
        conceptsB = _normalizeArray(designB.get("concepts"))
        conceptGuidsA = [_getGuidFromRef(c) for c in conceptsA]
        conceptGuidsB = [_getGuidFromRef(c) for c in conceptsB]
        if conceptGuidsA != conceptGuidsB:
        if not arePiecesEqualDict(designA.get("pieces"), designB.get("pieces"), strict):
        if not areConnectionsEqualDict(designA.get("connections"), designB.get("connections"), strict):
        if not areAttributesEqualDict(designA.get("attributes"), designB.get("attributes"), strict):
        if strict:
            if designA.get("createdAt") != designB.get("createdAt"):
            if designA.get("updatedAt") != designB.get("updatedAt"):


def arePortsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two port dictionaries are equal.
    arePortsEqualDict MUST compare all port fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for ifaceA in arrA:
        ifaceB = next((x for x in arrB if x.get("guid") == ifaceA.get("guid")), None)
        if ifaceB is None:
        if ifaceA.get("name") != ifaceB.get("name"):
        if _normalizeValue(ifaceA.get("description")) != _normalizeValue(ifaceB.get("description")):
        if not areAttributesEqualDict(ifaceA.get("attributes"), ifaceB.get("attributes"), strict):
        if strict:
            if ifaceA.get("createdAt") != ifaceB.get("createdAt"):
            if ifaceA.get("updatedAt") != ifaceB.get("updatedAt"):


def areQualitiesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two quality dictionaries are equal.
    areQualitiesEqualDict MUST compare all quality fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for qualA in arrA:
        qualB = next((x for x in arrB if x.get("guid") == qualA.get("guid")), None)
        if qualB is None:
        if qualA.get("key") != qualB.get("key"):
        if qualA.get("name") != qualB.get("name"):
        if not areAttributesEqualDict(qualA.get("attributes"), qualB.get("attributes"), strict):
        if strict:
            if qualA.get("createdAt") != qualB.get("createdAt"):
            if qualA.get("updatedAt") != qualB.get("updatedAt"):


def areFilesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two file dictionaries are equal.
    areFilesEqualDict MUST compare all file fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for fileA in arrA:
        fileB = next((x for x in arrB if x.get("guid") == fileA.get("guid")), None)
        if fileB is None:
        if fileA.get("name") != fileB.get("name"):
        if strict:
            if fileA.get("createdAt") != fileB.get("createdAt"):
            if fileA.get("updatedAt") != fileB.get("updatedAt"):


def areFoldersEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two folder dictionaries are equal.
    areFoldersEqualDict MUST compare all folder fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for folderA in arrA:
        folderB = next((x for x in arrB if x.get("guid") == folderA.get("guid")), None)
        if folderB is None:
        if folderA.get("name") != folderB.get("name"):
        if not areAttributesEqualDict(folderA.get("attributes"), folderB.get("attributes"), strict):
        if strict:
            if folderA.get("createdAt") != folderB.get("createdAt"):
            if folderA.get("updatedAt") != folderB.get("updatedAt"):


def areAuthorsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two author dictionaries are equal.
    areAuthorsEqualDict MUST compare all author fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for authorA in arrA:
        authorB = next((x for x in arrB if x.get("guid") == authorA.get("guid")), None)
        if authorB is None:
        if authorA.get("name") != authorB.get("name"):
        if _normalizeValue(authorA.get("email")) != _normalizeValue(authorB.get("email")):
        if not areAttributesEqualDict(authorA.get("attributes"), authorB.get("attributes"), strict):
        if strict:
            if authorA.get("createdAt") != authorB.get("createdAt"):
            if authorA.get("updatedAt") != authorB.get("updatedAt"):


def areConceptsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two concept dictionaries are equal.
    areConceptsEqualDict MUST compare all concept fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for conceptA in arrA:
        conceptB = next((x for x in arrB if x.get("guid") == conceptA.get("guid")), None)
        if conceptB is None:
        if conceptA.get("name") != conceptB.get("name"):
        if _normalizeValue(conceptA.get("description")) != _normalizeValue(conceptB.get("description")):
        if _normalizeValue(conceptA.get("icon")) != _normalizeValue(conceptB.get("icon")):
        if strict:
            if conceptA.get("createdAt") != conceptB.get("createdAt"):
            if conceptA.get("updatedAt") != conceptB.get("updatedAt"):


def areTagsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two tag dictionaries are equal.
    areTagsEqualDict MUST compare all tag fields for equality.
    """
    arrA = _normalizeArray(a)
    arrB = _normalizeArray(b)
    if len(arrA) != len(arrB):
    for tagA in arrA:
        tagB = next((x for x in arrB if x.get("guid") == tagA.get("guid")), None)
        if tagB is None:
        if tagA.get("name") != tagB.get("name"):
        if _normalizeValue(tagA.get("description")) != _normalizeValue(tagB.get("description")):
        if _normalizeValue(tagA.get("icon")) != _normalizeValue(tagB.get("icon")):
        if strict:
            if tagA.get("createdAt") != tagB.get("createdAt"):
            if tagA.get("updatedAt") != tagB.get("updatedAt"):


def areKitsDictEqual(a: dict, b: dict, strict: bool = False) -> bool:
    """Deep equality check for kits (dict-based) - recursively compares all properties including nested entities.
    Args:
    strict: If True, also compare timestamps (createdAt, updatedAt). Default False.
    Returns:
    True if kits are equal, False otherwise.
    areKitsDictEqual MUST compare all kit fields and children recursively.
    """
    if a.get("guid") != b.get("guid"):
    if a.get("name") != b.get("name"):
    if _normalizeValue(a.get("version")) != _normalizeValue(b.get("version")):
    if _normalizeValue(a.get("description")) != _normalizeValue(b.get("description")):
    if _normalizeValue(a.get("icon")) != _normalizeValue(b.get("icon")):
    if _normalizeValue(a.get("image")) != _normalizeValue(b.get("image")):
    if _normalizeValue(a.get("preview")) != _normalizeValue(b.get("preview")):
    if _normalizeValue(a.get("remote")) != _normalizeValue(b.get("remote")):
    if _normalizeValue(a.get("homepage")) != _normalizeValue(b.get("homepage")):
    if _normalizeValue(a.get("license")) != _normalizeValue(b.get("license")):
    if not areConceptsEqualDict(a.get("concepts"), b.get("concepts"), strict):
    if not areTagsEqualDict(a.get("tags"), b.get("tags"), strict):
    if not areTypesEqualDict(a.get("types"), b.get("types"), strict):
    if not areDesignsEqualDict(a.get("designs"), b.get("designs"), strict):
    if not arePortsEqualDict(a.get("ports"), b.get("ports"), strict):
    if not areQualitiesEqualDict(a.get("qualities"), b.get("qualities"), strict):
    if not areFilesEqualDict(a.get("files"), b.get("files"), strict):
    if not areFoldersEqualDict(a.get("folders"), b.get("folders"), strict):
    if not areAuthorsEqualDict(a.get("authors"), b.get("authors"), strict):
    if not areAttributesEqualDict(a.get("attributes"), b.get("attributes"), strict):
    if strict:
        if a.get("createdAt") != b.get("createdAt"):
        if a.get("updatedAt") != b.get("updatedAt"):


def _getCollectionDiff(
    before: list,
    after: list,
    getItemDiff: typing.Callable[[dict, dict], dict],
    entityKey: str = "",
) -> dict:
    """Get diff for a collection of items identified by guid.

    Args:
        entityKey: The key name for the entity ID in the updated array (e.g., "type", "design", "piece")
    """
    diff: dict = {}
    beforeGuids = {item.get("guid") for item in before}
    afterGuids = {item.get("guid") for item in after}

    removed = [{"guid": item.get("guid")} for item in before if item.get("guid") not in afterGuids]
    if removed:
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
    added = [item for item in after if item.get("guid") not in beforeGuids]
    if added:


def _applyCollectionDiff(
    base: list,
    diff: dict | None,
    applyItemDiff: typing.Callable[[dict, dict], dict],
    entityKey: str = "",
) -> list:
    """Apply diff to a collection of items.

    Args:
        diff: The diff to apply (with removed, updated, added)
        entityKey: The key name for the entity ID in the updated array (e.g., "type", "design", "piece")
    """
    if not diff:
    result = [dict(item) for item in base]
    if diff.get("removed"):
        removedGuids = [r["guid"] if isinstance(r, dict) else r for r in diff["removed"]]
        result = [item for item in result if item.get("guid") not in removedGuids]
    if diff.get("updated"):
        for update in diff["updated"]:
            if entityKey and entityKey in update:
                updateGuid = update[entityKey]["guid"]
            elif "id" in update:
                updateGuid = update["id"]
            if not updateGuid:
                (i for i, item in enumerate(result) if item.get("guid") == updateGuid),
                -1,
            )
            if idx >= 0:
                result[idx] = applyItemDiff(result[idx], update["diff"])
    if diff.get("added"):
        result.extend(diff["added"])


def _getTypeDiff(before: dict, after: dict) -> dict:
    """Get diff between two type dicts.
    """
    diff: dict = {}
    for key in ["name", "description", "icon", "image", "folder", "unit", "stock"]:
        if _normalizeValue(before.get(key)) != _normalizeValue(after.get(key)):
            diff[key] = after.get(key)
    for key in ["isAbstract", "virtual"]:
        if _normalizeBoolean(before.get(key)) != _normalizeBoolean(after.get(key)):
            diff[key] = after.get(key)
    for refKey in ["location", "parent"]:
        if _normalizeValue(bGuid) != _normalizeValue(aGuid):
            diff[refKey] = after.get(refKey)
            before.get("concepts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
            after.get("concepts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["concepts"] = after.get("concepts")
            before.get("authors", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
            after.get("authors", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["authors"] = after.get("authors")
        before.get("connectors", []),
        after.get("connectors", []),
        _getConnectorDiff,
        "connector",
    )
    if connectorsDiff:
    modelsDiff = _getCollectionDiff(before.get("models", []), after.get("models", []), _getModelDiff, "model")
    if modelsDiff:
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyTypeDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a type dict.
    """
    result = dict(base)
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
            result[key] = diff[key]
    for refKey in ["location", "parent"]:
        if refKey in diff:
            result[refKey] = diff[refKey]
    if "concepts" in diff:
        result["concepts"] = diff["concepts"]
    if "authors" in diff:
        result["authors"] = diff["authors"]
    if diff.get("connectors"):
            base.get("connectors", []),
            diff["connectors"],
            _applyConnectorDiff,
            "connector",
        )
    if diff.get("models"):
        result["models"] = _applyCollectionDiff(base.get("models", []), diff["models"], _applyModelDiff, "model")
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getConnectorDiff(before: dict, after: dict) -> dict:
    """Get diff between two connector dicts.
    """
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if before.get("t") != after.get("t"):
        diff["t"] = after.get("t")
    if _normalizeBoolean(before.get("mandatory")) != _normalizeBoolean(after.get("mandatory")):
        diff["mandatory"] = after.get("mandatory")
    if _normalizeValue(bPortGuid) != _normalizeValue(aPortGuid):
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
    bDir = before.get("direction", {})
    aDir = after.get("direction", {})
    if bDir and aDir and isinstance(bDir, dict) and isinstance(aDir, dict):
        dx = (aDir.get("x", 0) or 0) - (bDir.get("x", 0) or 0)
        dy = (aDir.get("y", 0) or 0) - (bDir.get("y", 0) or 0)
        dz = (aDir.get("z", 0) or 0) - (bDir.get("z", 0) or 0)
        if abs(dx) > 1e-10 or abs(dy) > 1e-10 or abs(dz) > 1e-10:
            diff["direction"] = {"x": dx, "y": dy, "z": dz}
    elif aDir and not bDir:
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyConnectorDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a connector dict.
    """
    result = dict(base)
    for key in ["name", "description", "t", "mandatory"]:
        if key in diff:
            result[key] = diff[key]
    if "port" in diff:
        result["port"] = diff["port"]
    if "point" in diff:
        bPoint = base.get("point", {})
        if bPoint and isinstance(bPoint, dict):
                "x": (bPoint.get("x", 0) or 0) + (diff["point"].get("x", 0) or 0),
                "y": (bPoint.get("y", 0) or 0) + (diff["point"].get("y", 0) or 0),
                "z": (bPoint.get("z", 0) or 0) + (diff["point"].get("z", 0) or 0),
            }
        else:
            result["point"] = diff["point"]
    if "direction" in diff:
        bDir = base.get("direction", {})
        if bDir and isinstance(bDir, dict):
                "x": (bDir.get("x", 0) or 0) + (diff["direction"].get("x", 0) or 0),
                "y": (bDir.get("y", 0) or 0) + (diff["direction"].get("y", 0) or 0),
                "z": (bDir.get("z", 0) or 0) + (diff["direction"].get("z", 0) or 0),
            }
        else:
            result["direction"] = diff["direction"]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getModelDiff(before: dict, after: dict) -> dict:
    """Get diff between two model dicts.
    """
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if _normalizeValue(bFileGuid) != _normalizeValue(aFileGuid):
        diff["file"] = after.get("file")
            before.get("tags", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
            after.get("tags", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["tags"] = after.get("tags")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyModelDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a model dict.
    """
    result = dict(base)
    for key in ["name", "description"]:
        if key in diff:
            result[key] = diff[key]
    if "file" in diff:
        result["file"] = diff["file"]
    if "tags" in diff:
        result["tags"] = diff["tags"]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getDesignDiff(before: dict, after: dict) -> dict:
    """Get diff between two design dicts.
    """
    diff: dict = {}
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
        if _normalizeValue(bGuid) != _normalizeValue(aGuid):
            diff[refKey] = after.get(refKey)
            before.get("concepts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
            after.get("concepts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["concepts"] = after.get("concepts")
            before.get("authors", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
            after.get("authors", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["authors"] = after.get("authors")
    piecesDiff = _getCollectionDiff(before.get("pieces", []), after.get("pieces", []), _getPieceDiff, "piece")
    if piecesDiff:
        before.get("connections", []),
        after.get("connections", []),
        _getConnectionDiff,
        "connection",
    )
    if connectionsDiff:
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyDesignDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a design dict.
    """
    result = dict(base)
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
            result[key] = diff[key]
    for refKey in ["activeLayer", "parent", "location"]:
        if refKey in diff:
            result[refKey] = diff[refKey]
    if "concepts" in diff:
        result["concepts"] = diff["concepts"]
    if "authors" in diff:
        result["authors"] = diff["authors"]
    if diff.get("pieces"):
        result["pieces"] = _applyCollectionDiff(base.get("pieces", []), diff["pieces"], _applyPieceDiff, "piece")
    if diff.get("connections"):
            base.get("connections", []),
            diff["connections"],
            _applyConnectionDiff,
            "connection",
        )
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def designWithDiffDict(base: dict, diff: dict) -> dict:
    """Create a mixed design keeping old entities with diff status annotations.
    designWithDiffDict MUST maintain all old pieces and connections (same parameters),
    annotate each with a semio.diffStatus attribute (unchanged/modified/removed/added),
    keep deleted entities in place marked as removed, and append added entities marked as added.
    """
    import copy

    def status_attr(status: str) -> dict:
            "guid": f"semio.diffStatus.{status}",
            "key": "semio.diffStatus",
            "value": status,
        }

    pieces_diff = diff.get("pieces", {})
    removed_piece_guids = {r["guid"] for r in pieces_diff.get("removed", [])}
    updated_piece_guids = {u.get("piece", {}).get("guid") for u in pieces_diff.get("updated", [])}

    conns_diff = diff.get("connections", {})
    removed_conn_guids = {r["guid"] for r in conns_diff.get("removed", [])}
    updated_conn_guids = {u.get("connection", {}).get("guid") for u in conns_diff.get("updated", [])}

    result_pieces = []
    for p in base.get("pieces", []):
        pc = copy.deepcopy(p)
        attrs = pc.get("attributes", []) or []
        if pc["guid"] in removed_piece_guids:
            attrs.append(status_attr("removed"))
        elif pc["guid"] in updated_piece_guids:
            attrs.append(status_attr("modified"))
        else:
            attrs.append(status_attr("unchanged"))
        result_pieces.append(pc)
    for added in pieces_diff.get("added", []):
        ac = copy.deepcopy(added)
        attrs = ac.get("attributes", []) or []
        attrs.append(status_attr("added"))
        result_pieces.append(ac)

    result_conns = []
    for c in base.get("connections", []):
        cc = copy.deepcopy(c)
        attrs = cc.get("attributes", []) or []
        if cc["guid"] in removed_conn_guids:
            attrs.append(status_attr("removed"))
        elif cc["guid"] in updated_conn_guids:
            attrs.append(status_attr("modified"))
        else:
            attrs.append(status_attr("unchanged"))
        result_conns.append(cc)
    for added in conns_diff.get("added", []):
        ac = copy.deepcopy(added)
        attrs = ac.get("attributes", []) or []
        attrs.append(status_attr("added"))
        result_conns.append(ac)

    result = copy.deepcopy(base)


def _getPieceDiff(before: dict, after: dict) -> dict:
    """Get diff between two piece dicts.
    """
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    for refKey in ["type", "design"]:
        if _normalizeValue(bGuid) != _normalizeValue(aGuid):
            diff[refKey] = after.get(refKey)
    if before.get("plane") != after.get("plane"):
        diff["plane"] = after.get("plane")
    if before.get("center") != after.get("center"):
        diff["center"] = after.get("center")
    if before.get("scale") != after.get("scale"):
        diff["scale"] = after.get("scale")
    if _normalizeValue(before.get("color")) != _normalizeValue(after.get("color")):
        diff["color"] = after.get("color")
    for key in ["isHidden", "isLocked"]:
        if before.get(key) != after.get(key):
            diff[key] = after.get(key)
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyPieceDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a piece dict.
    """
    result = dict(base)
        "name",
        "description",
        "scale",
        "plane",
        "center",
        "color",
        "isHidden",
        "isLocked",
    ]:
        if key in diff:
            result[key] = diff[key]
    for refKey in ["type", "design"]:
        if refKey in diff:
            result[refKey] = diff[refKey]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getConnectionDiff(before: dict, after: dict) -> dict:
    """Get diff between two connection dicts.
    """
    diff: dict = {}
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        if abs(delta) > 1e-10:
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if before.get("connecting") != after.get("connecting"):
        diff["connecting"] = after.get("connecting")
    if before.get("connected") != after.get("connected"):
        diff["connected"] = after.get("connected")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyConnectionDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a connection dict.
    """
    result = dict(base)
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        if key in diff:
            result[key] = (base.get(key, 0) or 0) + (diff[key] or 0)
    for key in ["description"]:
        if key in diff:
            result[key] = diff[key]
    for key in ["connecting", "connected"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getTagDiff(before: dict, after: dict) -> dict:
    """Get diff between two tag dicts.
    """
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyTagDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a tag dict.
    """
    result = dict(base)
    for key in ["name", "description", "icon"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getConceptDiff(before: dict, after: dict) -> dict:
    """Get diff between two concept dicts.
    """
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyConceptDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a concept dict.
    """
    result = dict(base)
    for key in ["name", "description", "icon"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getPortDiff(before: dict, after: dict) -> dict:
    """Get diff between two port dicts.
    """
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
            before.get("compatiblePorts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
            after.get("compatiblePorts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["compatiblePorts"] = after.get("compatiblePorts")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyPortDiff(base: dict, diff: dict) -> dict:
    """Apply diff to an port dict.
    """
    result = dict(base)
    for key in ["name", "description", "icon"]:
        if key in diff:
            result[key] = diff[key]
    if "compatiblePorts" in diff:
        result["compatiblePorts"] = diff["compatiblePorts"]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getFileDiff(before: dict, after: dict) -> dict:
    """Get diff between two file dicts.
    """
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("remote")) != _normalizeValue(after.get("remote")):
        diff["remote"] = after.get("remote")
    if before.get("size") != after.get("size"):
        diff["size"] = after.get("size")
    if _normalizeValue(before.get("hash")) != _normalizeValue(after.get("hash")):
        diff["hash"] = after.get("hash")
    if _normalizeValue(before.get("blob")) != _normalizeValue(after.get("blob")):
        diff["blob"] = after.get("blob")
    if _normalizeValue(bFolderGuid) != _normalizeValue(aFolderGuid):
        diff["folder"] = after.get("folder")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyFileDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a file dict.
    """
    result = dict(base)
    for key in ["name", "description", "remote", "size", "hash", "blob"]:
        if key in diff:
            result[key] = diff[key]
    if "folder" in diff:
        result["folder"] = diff["folder"]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getFolderDiff(before: dict, after: dict) -> dict:
    """Get diff between two folder dicts.
    """
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyFolderDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a folder dict.
    """
    result = dict(base)
    for key in ["name", "description"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getQualityDiff(before: dict, after: dict) -> dict:
    """Get diff between two quality dicts.
    """
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


def _applyQualityDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a quality dict.
    """
    result = dict(base)
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
            result[key] = diff[key]


def _getAuthorDiff(before: dict, after: dict) -> dict:
    """Get diff between two author dicts.
    """
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("email")) != _normalizeValue(after.get("email")):
        diff["email"] = after.get("email")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def _applyAuthorDiff(base: dict, diff: dict) -> dict:
    """Apply diff to an author dict.
    """
    result = dict(base)
    for key in ["name", "email"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _getAttributeDiff(before: dict, after: dict) -> dict:
    """Get diff between two attribute dicts - used for individual attribute update diffs.
    """
    diff: dict = {}
    if _normalizeValue(before.get("key")) != _normalizeValue(after.get("key")):
        diff["key"] = after.get("key")
    if _normalizeValue(before.get("value")) != _normalizeValue(after.get("value")):
        diff["value"] = after.get("value")
    if _normalizeValue(before.get("definition")) != _normalizeValue(after.get("definition")):
        diff["definition"] = after.get("definition")


def _applyAttributeDiff(base: dict, diff: dict) -> dict:
    """Apply diff to an attribute dict.
    """
    result = dict(base)
    for key in ["key", "value", "definition"]:
        if key in diff:
            result[key] = diff[key]


def _getAttributesDiff(before: list, after: list) -> dict:
    """Get diff for attributes collection - uses GUID for identification with EntityId format.
    """
    diff: dict = {}
    beforeGuids = {a.get("guid") for a in before}
    afterGuids = {a.get("guid") for a in after}

    removed = [{"guid": a.get("guid")} for a in before if a.get("guid") not in afterGuids]
    if removed:
    updated = []
    for afterAttr in after:
        guid = afterAttr.get("guid")
        if guid in beforeGuids:
            beforeAttr = next(a for a in before if a.get("guid") == guid)
            attrDiff = _getAttributeDiff(beforeAttr, afterAttr)
            if attrDiff:
                updated.append({"attribute": {"guid": guid}, "diff": attrDiff})
    if updated:
    added = [a for a in after if a.get("guid") not in beforeGuids]
    if added:


def _applyAttributesDiff(base: list, diff: dict | None) -> list:
    """Apply diff to attributes collection - uses GUID for identification with EntityId format.
    """
    if not diff:
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


def _inverseAttributesDiff(original: list, appliedDiff: dict) -> dict:
    """Compute inverse of attributes collection diff - uses GUID with EntityId format.
    """
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
                (u for u in appliedDiff.get("updated", []) if (u.get("attribute", {}).get("guid") if "attribute" in u else u.get("id")) == guid),
                None,
            )
            if origAttr and upd:
                        "attribute": {"guid": guid},
                        "diff": _inverseAttributeDiff(origAttr, upd["diff"]),
                    }
                )
    if removedGuids:
        inverse["added"] = [a for a in original if a.get("guid") in removedGuids]


def _inverseAttributeDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of an attribute diff.
    """
    inverse: dict = {}
    for key in ["key", "value", "definition"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)


def getKitDiffDict(before: dict, after: dict) -> dict:
    """Compute the diff between two kit dicts.
    getKitDiffDict MUST identify all added, removed and changed entities.
    """
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
    designsDiff = _getCollectionDiff(before.get("designs", []), after.get("designs", []), _getDesignDiff, "design")
    if designsDiff:
    tagsDiff = _getCollectionDiff(before.get("tags", []), after.get("tags", []), _getTagDiff, "tag")
    if tagsDiff:
        before.get("concepts", []),
        after.get("concepts", []),
        _getConceptDiff,
        "concept",
    )
    if conceptsDiff:
    portsDiff = _getCollectionDiff(before.get("ports", []), after.get("ports", []), _getPortDiff, "port")
    if portsDiff:
    filesDiff = _getCollectionDiff(before.get("files", []), after.get("files", []), _getFileDiff, "file")
    if filesDiff:
    foldersDiff = _getCollectionDiff(before.get("folders", []), after.get("folders", []), _getFolderDiff, "folder")
    if foldersDiff:
        before.get("qualities", []),
        after.get("qualities", []),
        _getQualityDiff,
        "quality",
    )
    if qualitiesDiff:
    authorsDiff = _getCollectionDiff(before.get("authors", []), after.get("authors", []), _getAuthorDiff, "author")
    if authorsDiff:
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:


def applyKitDiffDict(base: dict, diff: dict) -> dict:
    """Apply a diff to a kit dict.
    applyKitDiffDict MUST apply additions, removals and changes correctly.
    """
    result = dict(base)
    result["guid"] = base.get("guid")
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
    if diff.get("ports") or base.get("ports"):
        result["ports"] = _applyCollectionDiff(base.get("ports", []), diff.get("ports"), _applyPortDiff, "port")
    if diff.get("files") or base.get("files"):
        result["files"] = _applyCollectionDiff(base.get("files", []), diff.get("files"), _applyFileDiff, "file")
    if diff.get("folders") or base.get("folders"):
        result["folders"] = _applyCollectionDiff(base.get("folders", []), diff.get("folders"), _applyFolderDiff, "folder")
    if diff.get("qualities") or base.get("qualities"):
            base.get("qualities", []),
            diff.get("qualities"),
            _applyQualityDiff,
            "quality",
        )
    if diff.get("authors") or base.get("authors"):
        result["authors"] = _applyCollectionDiff(base.get("authors", []), diff.get("authors"), _applyAuthorDiff, "author")
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))


def _inverseCollectionDiff(
    original: list,
    appliedDiff: dict,
    inverseItemDiff: typing.Callable[[dict, dict], dict],
    entityKey: str = "",
) -> dict:
    """Compute inverse of a collection diff.

    Args:
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
                            entityKey: {"guid": updateGuid},
                            "diff": inverseItemDiff(origItem, update["diff"]),
                        }
                    )
                else:
                            "id": updateGuid,
                            "diff": inverseItemDiff(origItem, update["diff"]),
                        }
                    )


def _inverseTypeDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a type diff.
    """
    inverse: dict = {}
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
            original.get("connectors", []),
            appliedDiff["connectors"],
            _inverseConnectorDiff,
            "connector",
        )
    if appliedDiff.get("models"):
            original.get("models", []),
            appliedDiff["models"],
            _inverseModelDiff,
            "model",
        )
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseConnectorDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a connector diff.
    """
    inverse: dict = {}
    for key in ["name", "description", "t", "mandatory"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "port" in appliedDiff:
        inverse["port"] = original.get("port")
    if "point" in appliedDiff:
        p = appliedDiff["point"]
            "x": -(p.get("x", 0) or 0),
            "y": -(p.get("y", 0) or 0),
            "z": -(p.get("z", 0) or 0),
        }
    if "direction" in appliedDiff:
        d = appliedDiff["direction"]
            "x": -(d.get("x", 0) or 0),
            "y": -(d.get("y", 0) or 0),
            "z": -(d.get("z", 0) or 0),
        }
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseModelDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a model diff.
    """
    inverse: dict = {}
    for key in ["name", "description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "file" in appliedDiff:
        inverse["file"] = original.get("file")
    if "tags" in appliedDiff:
        inverse["tags"] = original.get("tags")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseConnectionDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a connection diff (negate numeric deltas).
    """
    inverse: dict = {}
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        if key in appliedDiff:
            inverse[key] = -(appliedDiff[key] or 0)
    for key in ["description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    for key in ["connecting", "connected"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseModelDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a model diff.
    """
    inverse: dict = {}
    for key in ["name", "description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "file" in appliedDiff:
        inverse["file"] = original.get("file")
    if "tags" in appliedDiff:
        inverse["tags"] = original.get("tags")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseConnectionDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a connection diff (negate numeric deltas).
    """
    inverse: dict = {}
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        if key in appliedDiff:
            inverse[key] = -(appliedDiff[key] or 0)
    for key in ["description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    for key in ["connecting", "connected"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseDesignDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a design diff.
    """
    inverse: dict = {}
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
            original.get("pieces", []),
            appliedDiff["pieces"],
            _inversePieceDiff,
            "piece",
        )
    if appliedDiff.get("connections"):
            original.get("connections", []),
            appliedDiff["connections"],
            _inverseConnectionDiff,
            "connection",
        )
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inversePieceDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a piece diff.
    """
    inverse: dict = {}
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
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseTagDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a tag diff.
    """
    inverse: dict = {}
    for key in ["name", "description", "icon"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseConceptDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a concept diff.
    """
    inverse: dict = {}
    for key in ["name", "description", "icon"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inversePortDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of an port diff.
    """
    inverse: dict = {}
    for key in ["name", "description", "icon"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "compatiblePorts" in appliedDiff:
        inverse["compatiblePorts"] = original.get("compatiblePorts")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseFileDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a file diff.
    """
    inverse: dict = {}
    for key in ["name", "description", "remote", "size", "hash", "blob"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "folder" in appliedDiff:
        inverse["folder"] = original.get("folder")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseFolderDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a folder diff.
    """
    inverse: dict = {}
    for key in ["name", "description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def _inverseQualityDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a quality diff.
    """
    inverse: dict = {}
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


def _inverseAuthorDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of an author diff.
    """
    inverse: dict = {}
    for key in ["name", "email"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


def inverseKitDiffDict(original: dict, appliedDiff: dict) -> dict:
    """Compute the inverse of a kit diff.
    inverseKitDiffDict MUST swap additions and removals to reverse the diff.
    """
    inverse: dict = {}
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
        inverse["types"] = _inverseCollectionDiff(original.get("types", []), appliedDiff["types"], _inverseTypeDiff, "type")
    if appliedDiff.get("designs"):
            original.get("designs", []),
            appliedDiff["designs"],
            _inverseDesignDiff,
            "design",
        )
    if appliedDiff.get("tags"):
        inverse["tags"] = _inverseCollectionDiff(original.get("tags", []), appliedDiff["tags"], _inverseTagDiff, "tag")
    if appliedDiff.get("concepts"):
            original.get("concepts", []),
            appliedDiff["concepts"],
            _inverseConceptDiff,
            "concept",
        )
    if appliedDiff.get("ports"):
        inverse["ports"] = _inverseCollectionDiff(original.get("ports", []), appliedDiff["ports"], _inversePortDiff, "port")
    if appliedDiff.get("files"):
        inverse["files"] = _inverseCollectionDiff(original.get("files", []), appliedDiff["files"], _inverseFileDiff, "file")
    if appliedDiff.get("folders"):
            original.get("folders", []),
            appliedDiff["folders"],
            _inverseFolderDiff,
            "folder",
        )
    if appliedDiff.get("qualities"):
            original.get("qualities", []),
            appliedDiff["qualities"],
            _inverseQualityDiff,
            "quality",
        )
    if appliedDiff.get("authors"):
            original.get("authors", []),
            appliedDiff["authors"],
            _inverseAuthorDiff,
            "author",
        )
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])


class Change:
    """Change holds the data fields for a Change record.
    """



def changeToDict(change: Change) -> dict:
    """
    result: dict = {"forward": change.forward, "backward": change.backward}
    if change.author is not None:
    if change.time is not None:
        result["time"] = change.time.isoformat()
    if change.before is not None:
    if change.after is not None:


class AttributeChange(Change):
    """AttributeChange holds the data fields for a AttributeChange record.
    """



class AuthorChange(Change):
    """AuthorChange holds the data fields for a AuthorChange record.
    """



class FileChange(Change):
    """FileChange holds the data fields for a FileChange record.
    """



class FolderChange(Change):
    """FolderChange holds the data fields for a FolderChange record.
    """



class QualityChange(Change):
    """QualityChange holds the data fields for a QualityChange record.
    """



class PortChange(Change):
    """PortChange holds the data fields for a PortChange record.
    """



class PropChange(Change):
    """PropChange holds the data fields for a PropChange record.
    """



class TagChange(Change):
    """TagChange holds the data fields for a TagChange record.
    """



class ConceptChange(Change):
    """ConceptChange holds the data fields for a ConceptChange record.
    """



class ModelChange(Change):
    """ModelChange holds the data fields for a ModelChange record.
    """



class ConnectorChange(Change):
    """ConnectorChange holds the data fields for a ConnectorChange record.
    """



class TypeChange(Change):
    """TypeChange holds the data fields for a TypeChange record.
    """



class LayerChange(Change):
    """LayerChange holds the data fields for a LayerChange record.
    """



class PieceChange(Change):
    """PieceChange holds the data fields for a PieceChange record.
    """



class GroupChange(Change):
    """GroupChange holds the data fields for a GroupChange record.
    """



class ConnectionChange(Change):
    """ConnectionChange holds the data fields for a ConnectionChange record.
    """



class StatChange(Change):
    """StatChange holds the data fields for a StatChange record.
    """



class DesignChange(Change):
    """DesignChange holds the data fields for a DesignChange record.
    """



class KitChange(Change):
    """KitChange holds the data fields for a KitChange record.
    """



def deletePiecesAndConnectionsInDesignDict(kit: dict, design: dict, pieceGuids: list[str], connectionGuids: list[str]) -> dict:
    """Deletes pieces and connections from a design dict, returning a DesignDiff dict.
    Removes stale connections referencing deleted pieces.
    Updates pieces that become fixed (parent connection removed) with flat plane and center from the flattened design.
    """
    deletedPieceSet = set(pieceGuids)
    connections = design.get("connections", [])

    # Find stale connections: connections referencing any deleted piece
    staleConnectionGuids = set()
    for conn in connections:
        connectedGuid = conn.get("connected", {}).get("piece", {}).get("guid", "")
        connectingGuid = conn.get("connecting", {}).get("piece", {}).get("guid", "")
        if connectedGuid in deletedPieceSet or connectingGuid in deletedPieceSet:
            staleConnectionGuids.add(conn["guid"])

    # All removed connections = explicit + stale

    # Find pieces that become fixed
    fixedPieceGuids: list[str] = []
    for connGuid in allRemovedConnectionGuids:
        conn = next((c for c in connections if c["guid"] == connGuid), None)
        if conn is None:
        connectingGuid = conn.get("connecting", {}).get("piece", {}).get("guid", "")
        if connectingGuid in deletedPieceSet:
        # Check if this piece has another parent connection not in the removed set
        hasOtherParent = any(c.get("connecting", {}).get("piece", {}).get("guid", "") == connectingGuid and c["guid"] not in allRemovedConnectionGuids for c in connections)
        if not hasOtherParent and connectingGuid not in fixedPieceGuids:
            fixedPieceGuids.append(connectingGuid)

        "origin": {"x": 0, "y": 0, "z": 0},
        "xAxis": {"x": 1, "y": 0, "z": 0},
        "yAxis": {"x": 0, "y": 1, "z": 0},
    }
    zeroCenter = {"u": 0, "v": 0}

    # Flatten the design to get absolute plane and center for each piece
    flatResult = flattenDesignDict(kit, design.get("guid", ""))
    flatPieceMap: dict[str, dict] = {}
    for piece in design.get("pieces", []):
        if piece.get("plane"):
                "plane": piece["plane"],
                "center": piece.get("center"),
            }
    for update in flatResult.get("pieces", {}).get("updated", []):
        guid = update.get("piece", {}).get("guid", update.get("id", ""))
        existing = flatPieceMap.get(guid, {})
        diff = update.get("diff", {})
        if diff.get("plane"):
            existing["plane"] = diff["plane"]
        if diff.get("center"):
            existing["center"] = diff["center"]

    diff: dict = {}

    piecesRemoved = [{"guid": g} for g in pieceGuids]
    piecesUpdated = []
    for g in fixedPieceGuids:
        flat = flatPieceMap.get(g, {})
                "piece": {"guid": g},
                    "plane": flat.get("plane", flatPlane),
                    "center": flat.get("center", zeroCenter),
                },
            }
        )
    if piecesRemoved or piecesUpdated:
        piecesDiff: dict = {}
        if piecesRemoved:
        if piecesUpdated:

    connectionsRemoved = [{"guid": g} for g in sorted(allRemovedConnectionGuids)]
    if connectionsRemoved:
        diff["connections"] = {"removed": connectionsRemoved}



def getDesignChange(
    before: dict,
    after: dict,
    author: typing.Optional[str] = None,
    time: typing.Optional[datetime.datetime] = None,
) -> DesignChange:
    """
    forward_diff = _getDesignDiff(before, after)
    backward_diff = _inverseDesignDiff(before, forward_diff)
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
    """
    forward_diff = getKitDiffDict(before, after)
    backward_diff = inverseKitDiffDict(before, forward_diff)
        forward=forward_diff,
        backward=backward_diff,
        author=author,
        time=time,
        before=before,
        after=after,
    )


def _extractUpdateGuid(update: dict, entityKeys: list[str]) -> str:
    """Extract guid from an updated entry which might use EntityId format or old id format.
    """
    for key in entityKeys:
        if key in update and isinstance(update[key], dict):
            return update[key].get("guid", "")
    return update.get("id", "")




def _areDiffDictsEqual(a: dict, b: dict) -> bool:
    """Deep equality check for diff dicts with float epsilon tolerance.
    _areDiffDictsEqual MUST recursively compare dict values with float tolerance.
    """
    if a is b:
    if type(a) != type(b):
        if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        return _normalizeValue(a) == _normalizeValue(b)
    if isinstance(a, dict):
        keysA = {k for k, v in a.items() if _normalizeValue(v) is not None}
        keysB = {k for k, v in b.items() if _normalizeValue(v) is not None}
        if keysA != keysB:
        for key in keysA:
            if not _areDiffDictsEqual(a[key], b[key]):
    if isinstance(a, list):
        if len(a) != len(b):
        for i in range(len(a)):
            if not _areDiffDictsEqual(a[i], b[i]):
    if isinstance(a, float):
    return _normalizeValue(a) == _normalizeValue(b)


def areKitDiffsDictEqual(a: dict, b: dict) -> bool:
    """Deep equality check for kit diffs.
    areKitDiffsDictEqual MUST compare all diff entries for equality.
    """
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

        removedA = {r["guid"] if isinstance(r, dict) else r for r in diffA.get("removed", [])}
        removedB = {r["guid"] if isinstance(r, dict) else r for r in diffB.get("removed", [])}
        if removedA != removedB:
        addedA = {item.get("guid"): item for item in diffA.get("added", [])}
        addedB = {item.get("guid"): item for item in diffB.get("added", [])}
        if set(addedA.keys()) != set(addedB.keys()):

        updatedA = {_extractUpdateGuid(u, [entityKey]): u["diff"] for u in diffA.get("updated", [])}
        updatedB = {_extractUpdateGuid(u, [entityKey]): u["diff"] for u in diffB.get("updated", [])}
        if set(updatedA.keys()) != set(updatedB.keys()):

        for guid in addedA:
            if not _areDiffDictsEqual(addedA[guid], addedB[guid]):

        for guid in updatedA:
            if not _areDiffDictsEqual(updatedA[guid], updatedB[guid]):



# endregion Kit Diff Operations

# region Kit Import/Export
# [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export)
# Import and export utilities for kit serialization and deserialization.


class KitData:
    """Simple in-memory kit representation that supports attribute access.
    KitData MUST hold all kit entities in memory for import and export operations.
    """

    def __init__(self, data: dict):
        self.guid = data.get("guid")
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

    def filter_kit(self, filter_spec: dict) -> "KitData":
        """General-purpose kit filter with glob support.
        """
        design_guid = filter_spec.get("design_guid")
        tags = filter_spec.get("model_tags")

        if design_guid:
            base = self._filter_kit_by_design(design_guid, tags)
        else:

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

        import fnmatch as _fnmatch

        def _matches(name: str, glob_filter: typing.Optional[dict]) -> bool:
            if glob_filter is None:
            include = glob_filter.get("include") or []
            exclude = glob_filter.get("exclude") or []
            if include and not any(_fnmatch.fnmatch(name.lower(), p.lower()) for p in include):
            if any(_fnmatch.fnmatch(name.lower(), p.lower()) for p in exclude):

        filtered = dict(base_data)
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
                filtered[entity_key] = [e for e in filtered.get(entity_key, []) if _matches(e.get(name_key, ""), spec)]

        return KitData(filtered)

    def _filter_kit_by_design(self, design_guid: str, tags: typing.Optional[list[str]] = None) -> "KitData":
        design = next((d for d in kit.get("designs", []) if d.get("guid") == design_guid), None)
        if design is None:
                    "guid": kit.get("guid"),
                    "name": kit.get("name", ""),
                    "version": kit.get("version", ""),
                }
            )

        used_type_guids: set[str] = set()
        used_design_guids: set[str] = {design_guid}
        for piece in design.get("pieces", []):
            piece_kind_guid = piece.get("type", {}).get("guid")
            if piece_kind_guid:
                used_type_guids.add(piece_kind_guid)
            child_design_guid = piece.get("design", {}).get("guid")
            if child_design_guid:
                used_design_guids.add(child_design_guid)

        type_by_guid = {type_item.get("guid"): type_item for type_item in kit.get("types", [])}

        def collect_ancestors(type_guid: str) -> None:
            parent_guid = (type_by_guid.get(type_guid) or {}).get("parent", {}).get("guid")
            if parent_guid and parent_guid not in used_type_guids:
                used_type_guids.add(parent_guid)
                collect_ancestors(parent_guid)

        for type_guid in list(used_type_guids):
            collect_ancestors(type_guid)

        resolved_tag_guids: list[str] = []
        for tag_value in tags or []:
                (tag for tag in kit.get("tags", []) if tag.get("guid") == tag_value),
                None,
            )
            if by_guid is not None:
                resolved_tag_guids.append(by_guid["guid"])
            resolved_tag_guids.extend(tag["guid"] for tag in kit.get("tags", []) if tag.get("name") == tag_value)

        used_port_guids: set[str] = set()
        used_file_guids: set[str] = set()
        used_tag_guids: set[str] = set()
        used_concept_guids: set[str] = set()
        used_quality_guids: set[str] = set()
        used_author_guids: set[str] = set()
        used_folder_names: set[str] = set()
        selected_models: dict[str, dict] = {}

        def collect_quality_from_props(props: typing.Optional[list[dict]]) -> None:
            for prop in props or []:
                quality_guid = prop.get("quality", {}).get("guid")
                if quality_guid:
                    used_quality_guids.add(quality_guid)

        def select_best_model(models: list[dict]) -> typing.Optional[dict]:
            if not models:
            if not resolved_tag_guids:
                return next((model for model in models if not model.get("tags")), models[0])
            filtered = [model for model in models if all(selected in {tag.get("guid") for tag in model.get("tags", [])} for selected in resolved_tag_guids)]
            if not filtered:

            def score(model: dict) -> float:
                model_tags = {tag.get("guid") for tag in model.get("tags", [])}
                selected = set(resolved_tag_guids)
                return 0.0 if not union else len(model_tags & selected) / len(union)

            return max(filtered, key=score)

        for type_guid in used_type_guids:
            type_item = type_by_guid.get(type_guid)
            if not type_item:
            if type_item.get("folder"):
                used_folder_names.add(type_item["folder"])
            for connector in type_item.get("connectors", []):
                port_guid = connector.get("port", {}).get("guid")
                if port_guid:
                    used_port_guids.add(port_guid)
                collect_quality_from_props(connector.get("props"))
            collect_quality_from_props(type_item.get("props"))
            for author in type_item.get("authors", []):
                if author.get("guid"):
                    used_author_guids.add(author["guid"])
            for concept in type_item.get("concepts", []):
                if concept.get("guid"):
                    used_concept_guids.add(concept["guid"])
            selected_model = select_best_model(type_item.get("models", []))
            if selected_model:
                file_guid = selected_model.get("file", {}).get("guid")
                if file_guid:
                    used_file_guids.add(file_guid)
                for tag in selected_model.get("tags", []):
                    if tag.get("guid"):
                        used_tag_guids.add(tag["guid"])

        for piece in design.get("pieces", []):
            collect_quality_from_props(piece.get("props"))
        for concept in design.get("concepts", []):
            if concept.get("guid"):
                used_concept_guids.add(concept["guid"])
        for author in design.get("authors", []):
            if author.get("guid"):
                used_author_guids.add(author["guid"])
        for port_guid in list(used_port_guids):
                (candidate for candidate in kit.get("ports", []) if candidate.get("guid") == port_guid),
                None,
            )
            for compatible in (port or {}).get("compatiblePorts", []):
                if compatible.get("guid"):
                    used_port_guids.add(compatible["guid"])
        used_tag_guids.update(resolved_tag_guids)

            for key, value in kit.items()
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
            if type_item.get("guid") not in used_type_guids:
            filtered_type = dict(type_item)
            selected_model = selected_models.get(type_item["guid"])
            filtered_type["models"] = [selected_model] if selected_model else []
            filtered["types"].append(filtered_type)
        filtered["designs"] = [candidate for candidate in kit.get("designs", []) if candidate.get("guid") in used_design_guids]
        filtered["ports"] = [port for port in kit.get("ports", []) if port.get("guid") in used_port_guids]
        filtered["files"] = [file for file in kit.get("files", []) if file.get("guid") in used_file_guids]
        filtered["tags"] = [tag for tag in kit.get("tags", []) if tag.get("guid") in used_tag_guids]
        filtered["concepts"] = [concept for concept in kit.get("concepts", []) if concept.get("guid") in used_concept_guids]
        filtered["qualities"] = [quality for quality in kit.get("qualities", []) if quality.get("guid") in used_quality_guids]
        filtered["authors"] = [author for author in kit.get("authors", []) if author.get("guid") in used_author_guids]
        filtered["folders"] = [folder for folder in kit.get("folders", []) if folder.get("name") in used_folder_names]
        return KitData(filtered)


def _parse_connector_from_sqlite(row: dict) -> dict:
    """
        "guid": row.get("guid"),
        "name": row.get("name"),
            "x": row.get("point_x", 0.0),
            "y": row.get("point_y", 0.0),
            "z": row.get("point_z", 0.0),
        },
            "x": row.get("direction_x", 0.0),
            "y": row.get("direction_y", 1.0),
            "z": row.get("direction_z", 0.0),
        },
        "t": row.get("t", 0.0),
        "mandatory": bool(row.get("mandatory", False)),
        "port": row.get("port_guid"),
        "description": row.get("description"),
    }


def _parse_model_from_sqlite(row: dict) -> dict:
    """
        "guid": row.get("guid"),
        "name": row.get("name"),
        "file": row.get("file_guid"),
        "description": row.get("description"),
    }


def _parse_type_from_sqlite(row: dict, connectors: list[dict], models: list[dict]) -> dict:
    """
        "guid": row.get("guid"),
        "name": row.get("name"),
        "parent": row.get("parent_guid"),
        "isAbstract": bool(row.get("is_abstract", False)),
        "isVirtual": bool(row.get("virtual", False)),
        "folder": row.get("folder"),
        "stock": row.get("stock"),
        "unit": row.get("unit"),
        "location": row.get("location_guid"),
        "description": row.get("description"),
        "icon": row.get("icon"),
        "image": row.get("image"),
        "connectors": connectors,
        "models": models,
    }


def _parse_piece_from_sqlite(row: dict) -> dict:
    """
    if row.get("plane_origin_x") is not None:
                "x": row.get("plane_origin_x", 0.0),
                "y": row.get("plane_origin_y", 0.0),
                "z": row.get("plane_origin_z", 0.0),
            },
                "x": row.get("plane_x_axis_x", 1.0),
                "y": row.get("plane_x_axis_y", 0.0),
                "z": row.get("plane_x_axis_z", 0.0),
            },
                "x": row.get("plane_y_axis_x", 0.0),
                "y": row.get("plane_y_axis_y", 1.0),
                "z": row.get("plane_y_axis_z", 0.0),
            },
        }
    if row.get("mirror_plane_origin_x") is not None:
                "x": row.get("mirror_plane_origin_x", 0.0),
                "y": row.get("mirror_plane_origin_y", 0.0),
                "z": row.get("mirror_plane_origin_z", 0.0),
            },
                "x": row.get("mirror_plane_x_axis_x", 1.0),
                "y": row.get("mirror_plane_x_axis_y", 0.0),
                "z": row.get("mirror_plane_x_axis_z", 0.0),
            },
                "x": row.get("mirror_plane_y_axis_x", 0.0),
                "y": row.get("mirror_plane_y_axis_y", 1.0),
                "z": row.get("mirror_plane_y_axis_z", 0.0),
            },
        }
    if row.get("center_u") is not None or row.get("center_v") is not None:
            "u": row.get("center_u", 0.0),
            "v": row.get("center_v", 0.0),
        }
        "guid": row.get("guid"),
        "id": row.get("name"),
        "type": row.get("type_guid"),
        "design": row.get("design_guid_ref"),
        "plane": plane,
        "center": center,
        "scale": row.get("scale"),
        "mirrorPlane": mirror_plane,
        "isHidden": bool(row.get("is_hidden", False)),
        "isLocked": bool(row.get("is_locked", False)),
        "color": row.get("color"),
        "description": row.get("description"),
    }


def _parse_connection_from_sqlite(row: dict) -> dict:
    """
        "guid": row.get("guid"),
            "piece": row.get("connected_piece_guid"),
            "designPiece": row.get("connected_design_piece_guid"),
            "connector": row.get("connected_connector_guid"),
        },
            "piece": row.get("connecting_piece_guid"),
            "designPiece": row.get("connecting_design_piece_guid"),
            "connector": row.get("connecting_connector_guid"),
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


def _parse_design_from_sqlite(row: dict, pieces: list[dict], connections: list[dict]) -> dict:
    """
    if row.get("view_center_u") is not None or row.get("view_center_v") is not None or row.get("view_zoom") is not None:
                "u": row.get("view_center_u", 0.0),
                "v": row.get("view_center_v", 0.0),
            },
            "zoom": row.get("view_zoom", 1.0),
        }
        "guid": row.get("guid"),
        "name": row.get("name"),
        "parent": row.get("parent_guid"),
        "variant": row.get("variant"),
        "view": view,
        "unit": row.get("unit"),
        "location": row.get("location_guid"),
        "activeLayer": row.get("active_layer_guid"),
        "isAbstract": bool(row.get("is_abstract", False)),
        "folder": row.get("folder"),
        "canScale": (bool(row.get("can_scale", False)) if row.get("can_scale") is not None else None),
        "canMirror": (bool(row.get("can_mirror", False)) if row.get("can_mirror") is not None else None),
        "description": row.get("description"),
        "icon": row.get("icon"),
        "image": row.get("image"),
        "pieces": pieces,
        "connections": connections,
    }


def _build_folder_path(kit_dict: dict, folder_guid: str) -> str:
    """Build folder path from folder hierarchy.
    """
    for f in kit_dict.get("folders", []):
        if f.get("guid") == folder_guid:
            parent = f.get("parent")
            if parent:
                parent_path = _build_folder_path(kit_dict, parent.get("guid", ""))
                if parent_path:
                    return parent_path + "/" + f.get("name", "")
            return f.get("name", "")


def _build_file_path(kit_dict: dict, file_dict: dict) -> str:
    """Build file path from folder hierarchy and file name.
    """
    folder = file_dict.get("folder")
    if folder:
        folder_path = _build_folder_path(kit_dict, folder.get("guid", ""))
        if folder_path:
            return folder_path + "/" + file_dict.get("name", "")
    return file_dict.get("name", "")


# region Kit Workflow Helpers


def _kit_to_dict(kit: KitData | dict) -> dict:
    """Return the underlying kit dictionary.
    _kit_to_dict MUST normalize KitData and dict inputs for shared workflow helpers.
    """


def _kit_without_file_blobs(kit: KitData | dict) -> dict:
    """Return a deep copy of a kit dictionary without embedded file blobs.
    _kit_without_file_blobs MUST remove file blob payloads before SQLite and archive persistence.
    """
    kit_copy = copy.deepcopy(_kit_to_dict(kit))
    for file_entry in kit_copy.get("files", []):
        file_entry.pop("blob", None)


def _decode_kit_file_blob(blob: str) -> bytes:
    """Decode a kit file blob into raw bytes.
    _decode_kit_file_blob MUST support data URLs and raw base64 payloads.
    """
    return base64.b64decode(encoded)


def _attach_file_blobs_to_kit(kit_dict: dict, files: dict[str, bytes]) -> dict:
    """Attach file blobs from asset bytes to a kit dictionary.
    _attach_file_blobs_to_kit MUST populate file blobs using canonical kit file paths.
    """
    for file_entry in kit_dict.get("files", []):
        file_path = _build_file_path(kit_dict, file_entry)
        if file_path in files:
            encoded = base64.b64encode(files[file_path]).decode("ascii")


def _collect_kit_asset_files(kit: KitData | dict, files: typing.Optional[dict[str, bytes]] = None) -> dict[str, bytes]:
    """Collect asset bytes for the current kit file entries.
    _collect_kit_asset_files MUST prefer embedded blobs and fall back to provided file bytes.
    """
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


def _merge_sqlite_entity(parsed: dict, payload_entity: typing.Optional[dict]) -> dict:
    """Merge a structured SQLite entity with payload metadata.
    _merge_sqlite_entity MUST keep SQLite fields authoritative while preserving unsupported payload fields.
    """
    if payload_entity is None:
    merged = copy.deepcopy(payload_entity)
    for key, value in parsed.items():
        if key in {"connectors", "models", "pieces", "connections"} or value is not None or key not in merged:


def _read_kit_from_sqlite(db_path: str) -> dict:
    """Read a kit dictionary from the folder SQLite database.
    _read_kit_from_sqlite MUST rebuild types and designs using the existing SQLite parsing helpers.
    """
    import sqlite3

    if not os.path.exists(db_path):
        raise FileNotFoundError(f"File not found: {db_path}")

    conn = sqlite3.connect(db_path)
    try:
        cursor = conn.cursor()
        payload_dict: dict = {}
        try:
            payload_row = cursor.execute("SELECT data FROM kit_payload WHERE id = 1").fetchone()
            if payload_row and payload_row["data"]:
                payload_dict = json.loads(payload_row["data"])
        except sqlite3.OperationalError:
            payload_dict = {}

        kit_row = cursor.execute("SELECT * FROM kit LIMIT 1").fetchone()
        if kit_row is None:
            if payload_dict:
            raise ValueError(f"Invalid kit database: no kit row found in {db_path}")

        payload_types_by_guid = {item.get("guid"): item for item in payload_dict.get("types", []) if item.get("guid")}
        payload_designs_by_guid = {item.get("guid"): item for item in payload_dict.get("designs", []) if item.get("guid")}

        connectors_by_type: dict[str, list[dict]] = {}
        for row in cursor.execute("SELECT * FROM connector ORDER BY guid").fetchall():
            connector = _parse_connector_from_sqlite(dict(row))
            connectors_by_type.setdefault(row["type_guid"], []).append(connector)

        models_by_type: dict[str, list[dict]] = {}
        model_tags_by_model: dict[str, list[dict]] = {}
        try:
            for row in cursor.execute("SELECT * FROM model_tag ORDER BY model_guid").fetchall():
                r = dict(row)
                model_tags_by_model.setdefault(r["model_guid"], []).append({"guid": r["tag_guid"]})
        except sqlite3.OperationalError:

        for row in cursor.execute("SELECT * FROM model ORDER BY guid").fetchall():
            model = _parse_model_from_sqlite(dict(row))
            model["tags"] = model_tags_by_model.get(row["guid"], [])
            models_by_type.setdefault(row["type_guid"], []).append(model)

        types: list[dict] = []
        for row in cursor.execute("SELECT * FROM type ORDER BY row_id, name, guid").fetchall():
                dict(row),
                connectors_by_type.get(row["guid"], []),
                models_by_type.get(row["guid"], []),
            )
            if type_dict.get("parent"):
                type_dict["parent"] = {"guid": type_dict["parent"]}
            if type_dict.get("location"):
                type_dict["location"] = {"guid": type_dict["location"]}
            types.append(_merge_sqlite_entity(type_dict, payload_types_by_guid.get(type_dict.get("guid"))))

        pieces_by_design: dict[str, list[dict]] = {}
        for row in cursor.execute("SELECT * FROM piece ORDER BY guid").fetchall():
            piece = _parse_piece_from_sqlite(dict(row))
            if piece.get("type"):
                piece["type"] = {"guid": piece["type"]}
            if piece.get("design"):
                piece["design"] = {"guid": piece["design"]}
            pieces_by_design.setdefault(row["design_guid"], []).append(piece)

        connections_by_design: dict[str, list[dict]] = {}
        for row in cursor.execute("SELECT * FROM connection ORDER BY guid").fetchall():
            connection = _parse_connection_from_sqlite(dict(row))
            for side in ["connected", "connecting"]:
                for key in ["piece", "designPiece", "connector"]:
                    ref = connection.get(side, {}).get(key)
                    if ref:
                        connection[side][key] = {"guid": ref}
            connections_by_design.setdefault(row["design_guid"], []).append(connection)

        designs: list[dict] = []
        for row in cursor.execute("SELECT * FROM design ORDER BY row_id, name, guid").fetchall():
                dict(row),
                pieces_by_design.get(row["guid"], []),
                connections_by_design.get(row["guid"], []),
            )
            if design_dict.get("parent"):
                design_dict["parent"] = {"guid": design_dict["parent"]}
            if design_dict.get("location"):
                design_dict["location"] = {"guid": design_dict["location"]}
            if design_dict.get("activeLayer"):
                design_dict["activeLayer"] = {"guid": design_dict["activeLayer"]}
            designs.append(_merge_sqlite_entity(design_dict, payload_designs_by_guid.get(design_dict.get("guid"))))

        seen_type_guids = {item.get("guid") for item in types}
        for payload_type in payload_dict.get("types", []):
            if payload_type.get("guid") not in seen_type_guids:
                types.append(copy.deepcopy(payload_type))

        seen_design_guids = {item.get("guid") for item in designs}
        for payload_design in payload_dict.get("designs", []):
            if payload_design.get("guid") not in seen_design_guids:
                designs.append(copy.deepcopy(payload_design))

        folders: list[dict] = []
        try:
            for row in cursor.execute("SELECT * FROM folder ORDER BY guid").fetchall():
                r = dict(row)
                folder_dict: dict = {"guid": r.get("guid"), "name": r.get("name")}
                if r.get("parent_guid"):
                    folder_dict["parent"] = {"guid": r["parent_guid"]}
                folders.append(folder_dict)
        except sqlite3.OperationalError:

        files: list[dict] = []
        try:
            for row in cursor.execute("SELECT * FROM file ORDER BY guid").fetchall():
                r = dict(row)
                file_dict: dict = {"guid": r.get("guid"), "name": r.get("name")}
                if r.get("mime"):
                    file_dict["mime"] = r["mime"]
                if r.get("size"):
                    file_dict["size"] = r["size"]
                if r.get("hash"):
                    file_dict["hash"] = r["hash"]
                if r.get("remote_url"):
                    file_dict["remote"] = r["remote_url"]
                if r.get("folder_guid"):
                    file_dict["folder"] = {"guid": r["folder_guid"]}
                files.append(file_dict)
        except sqlite3.OperationalError:

        result = {key: copy.deepcopy(value) for key, value in payload_dict.items() if key not in {"types", "designs", "folders", "files"}}
                "guid": kit_row["guid"],
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
    finally:
        conn.close()


def import_file_kit(path: str) -> KitData:
    """Import a JSON file kit.
    import_file_kit MUST deserialize a JSON kit file into KitData.
    """
    with open(path, "r", encoding="utf-8") as handle:
        return KitData(json.load(handle))


def export_file_kit(kit: KitData | dict, path: str) -> None:
    """Export a JSON file kit.
    export_file_kit MUST persist the in-memory kit dictionary as JSON.
    """
    parent = os.path.dirname(path)
    if parent:
        os.makedirs(parent, exist_ok=True)
    with open(path, "w", encoding="utf-8") as handle:
        json.dump(_kit_to_dict(kit), handle, ensure_ascii=False)


def import_folder_kit(folder_path: str) -> tuple[KitData, dict[str, bytes]]:
    """Import a folder kit backed by .semio/kit.db.
    import_folder_kit MUST rebuild the kit from SQLite and load asset files from the folder tree.
    """
    db_path = os.path.join(folder_path, KIT_LOCAL_SUFFIX)
    kit_dict = _read_kit_from_sqlite(db_path)
    files: dict[str, bytes] = {}
    for file_entry in kit_dict.get("files", []):
        relative_path = _build_file_path(kit_dict, file_entry)
        asset_path = os.path.join(folder_path, relative_path)
        if os.path.isfile(asset_path):
            with open(asset_path, "rb") as handle:
                files[relative_path] = handle.read()
    _attach_file_blobs_to_kit(kit_dict, files)


def export_folder_kit(kit: KitData | dict, files: dict[str, bytes], folder_path: str) -> None:
    """Export a folder kit backed by .semio/kit.db.
    export_folder_kit MUST write the SQLite kit database and synchronize asset files into the folder tree.
    """
    data = _kit_to_dict(kit)
    asset_files = _collect_kit_asset_files(data, files)
    os.makedirs(folder_path, exist_ok=True)
    for entry_name in os.listdir(folder_path):
        if entry_name == KIT_LOCAL_FOLDERNAME:
        entry_path = os.path.join(folder_path, entry_name)
        if os.path.isdir(entry_path):
            shutil.rmtree(entry_path)
        else:
            os.remove(entry_path)

    db_folder = os.path.join(folder_path, KIT_LOCAL_FOLDERNAME)
    os.makedirs(db_folder, exist_ok=True)
    db_path = os.path.join(db_folder, KIT_LOCAL_FILENAME)
    if os.path.exists(db_path):
        os.remove(db_path)
    _write_kit_to_sqlite(data, db_path)

    for relative_path, content in asset_files.items():
        asset_path = os.path.join(folder_path, relative_path)
        os.makedirs(os.path.dirname(asset_path), exist_ok=True)
        with open(asset_path, "wb") as handle:
            handle.write(content)


def _read_remote_kit_bytes(uri: str) -> tuple[str, bytes, str]:
    """Read remote kit bytes and detect JSON or ZIP format.
    _read_remote_kit_bytes MUST support HTTP(S) JSON and ZIP responses using urllib.request only.
    """
    parsed = urllib.parse.urlparse(uri)
    if parsed.scheme not in {"http", "https"} or not parsed.netloc:
        raise RemoteKitUriNotValid(uri)
    try:
        with urllib.request.urlopen(uri) as response:
            body = response.read()
            content_type = response.headers.get_content_type()
    except urllib.error.URLError as error:
        raise ServerUnreachable(server_url) from error



def import_remote_kit(uri: str) -> tuple[KitData, dict[str, bytes]]:
    """Import a remote kit from JSON or ZIP.
    import_remote_kit MUST support remote JSON and ZIP kit payloads over HTTP(S).
    """
    remote_kind, body, _ = _read_remote_kit_bytes(uri)
    if remote_kind == "archive":
        with tempfile.NamedTemporaryFile(suffix=".zip", delete=False) as handle:
            handle.write(body)
        try:
            return import_kit(archive_path)
        finally:
            if os.path.exists(archive_path):
                os.remove(archive_path)

    kit_dict = json.loads(body.decode("utf-8"))
    files = _collect_kit_asset_files(kit_dict)


def edit_temporary_kit(kit: KitData | dict, diff: dict) -> KitData:
    """Edit an in-memory temporary kit with a diff.
    edit_temporary_kit MUST applyKitDiffDict and return the updated KitData instance.
    """
    return KitData(applyKitDiffDict(_kit_to_dict(kit), diff))


def edit_file_kit(path: str, diff: dict) -> KitData:
    """Edit a JSON file kit in place.
    edit_file_kit MUST import, apply the diff, persist the JSON file, and return the updated kit.
    """
    updated = edit_temporary_kit(import_file_kit(path), diff)
    export_file_kit(updated, path)


def edit_folder_kit(folder_path: str, diff: dict) -> KitData:
    """Edit a folder kit in place.
    edit_folder_kit MUST import, apply the diff, persist the SQLite database and asset files, and return the updated kit.
    """
    kit, files = import_folder_kit(folder_path)
    updated = edit_temporary_kit(kit, diff)
    export_folder_kit(updated, _collect_kit_asset_files(updated, files), folder_path)


def edit_archive_kit(path: str, diff: dict) -> KitData:
    """Edit an archive kit in place.
    edit_archive_kit MUST import, apply the diff, persist the archive, and return the updated kit.
    """
    kit, files = import_kit(path)
    updated = edit_temporary_kit(kit, diff)
    export_kit(updated, _collect_kit_asset_files(updated, files), path)


def _write_remote_kit_bytes(uri: str, body: bytes, content_type: str) -> None:
    """Write remote kit bytes back to their source URI.
    _write_remote_kit_bytes MUST persist edited remote kit content using HTTP PUT.
    """
    parsed = urllib.parse.urlparse(uri)
    if parsed.scheme not in {"http", "https"} or not parsed.netloc:
        raise RemoteKitUriNotValid(uri)
    request = urllib.request.Request(uri, data=body, method="PUT", headers={"Content-Type": content_type})
    try:
        with urllib.request.urlopen(request):
    except urllib.error.URLError as error:
        raise ServerUnreachable(server_url) from error


def edit_remote_kit(uri: str, diff: dict) -> KitData:
    """Edit a remote JSON or ZIP kit in place.
    edit_remote_kit MUST import, apply the diff, persist the edited remote representation, and return the updated kit.
    """
    remote_kind, _, content_type = _read_remote_kit_bytes(uri)
    kit, files = import_remote_kit(uri)
    updated = edit_temporary_kit(kit, diff)

    if remote_kind == "archive":
        with tempfile.NamedTemporaryFile(suffix=".zip", delete=False) as handle:
        try:
            export_kit(updated, _collect_kit_asset_files(updated, files), archive_path)
            with open(archive_path, "rb") as handle:
                body = handle.read()
        finally:
            if os.path.exists(archive_path):
                os.remove(archive_path)
        _write_remote_kit_bytes(uri, body, "application/zip")

    body = json.dumps(_kit_to_dict(updated), ensure_ascii=False).encode("utf-8")
    _write_remote_kit_bytes(uri, body, content_type or "application/json")


# endregion Kit Workflow Helpers


def import_kit(path: str) -> tuple[KitData, dict[str, bytes]]:
    """Import a kit from a .zip file (containing kit.json and actual files).
    import_kit MUST read kit.json from zip and populate blob from actual files.
    """
    if not os.path.exists(path):
        raise FileNotFoundError(f"File not found: {path}")

    files: dict[str, bytes] = {}
    with zipfile.ZipFile(path, "r") as zip_ref:
        for file_info in zip_ref.infolist():
            if file_info.is_dir():
            with zip_ref.open(file_info) as f:
                data = f.read()
            if name == "kit.json":
            elif not name.startswith(".semio/"):

    if kit_json_data is None:
        raise ValueError(f"Invalid kit: kit.json not found in {path}")

    kit_dict = json.loads(kit_json_data)
    _attach_file_blobs_to_kit(kit_dict, files)


def _write_kit_to_sqlite(kit_data: KitData | dict, db_path: str) -> None:
    """Write kit data to SQLite database using the TypeScript schema.
    """
    import sqlite3
    from datetime import datetime


    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    cursor.execute("""
            guid VARCHAR(36) PRIMARY KEY,
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
        )
    """)

    cursor.execute("""
            guid VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256) NOT NULL,
            parent_guid VARCHAR(36),
            is_abstract BOOLEAN DEFAULT 0,
            folder VARCHAR(256),
            stock INTEGER,
            virtual BOOLEAN DEFAULT 0,
            unit VARCHAR(64),
            location_guid VARCHAR(36),
            description TEXT,
            icon TEXT,
            image TEXT,
            created DATETIME NOT NULL,
            updated DATETIME NOT NULL,
            kit_guid VARCHAR(36) NOT NULL,
        )
    """)

    cursor.execute("""
            guid VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256),
            point_x FLOAT NOT NULL,
            point_y FLOAT NOT NULL,
            point_z FLOAT NOT NULL,
            direction_x FLOAT NOT NULL,
            direction_y FLOAT NOT NULL,
            direction_z FLOAT NOT NULL,
            t FLOAT NOT NULL,
            mandatory BOOLEAN DEFAULT 0,
            port_guid VARCHAR(36),
            description TEXT,
        )
    """)

    cursor.execute("""
            guid VARCHAR(36) PRIMARY KEY,
            file_guid VARCHAR(36) NOT NULL,
            name VARCHAR(256),
            description TEXT,
        )
    """)

    cursor.execute("""
            guid VARCHAR(36),
            name VARCHAR(256) NOT NULL,
            parent_guid VARCHAR(36),
            variant VARCHAR(256),
            view_center_u FLOAT,
            view_center_v FLOAT,
            view_zoom FLOAT,
            unit VARCHAR(64),
            location_guid VARCHAR(36),
            active_layer_guid VARCHAR(36),
            is_abstract BOOLEAN DEFAULT 0,
            folder VARCHAR(256),
            can_scale BOOLEAN,
            can_mirror BOOLEAN,
            description TEXT,
            icon TEXT,
            image TEXT,
            created DATETIME NOT NULL,
            updated DATETIME NOT NULL,
            kit_guid VARCHAR(36) NOT NULL,
        )
    """)

    cursor.execute("""
            guid VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256),
            type_guid VARCHAR(36),
            design_guid_ref VARCHAR(36),
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
        )
    """)

    cursor.execute("""
            guid VARCHAR(36) PRIMARY KEY,
            connected_piece_guid VARCHAR(36) NOT NULL,
            connected_design_piece_guid VARCHAR(36),
            connected_connector_guid VARCHAR(36),
            connecting_piece_guid VARCHAR(36) NOT NULL,
            connecting_design_piece_guid VARCHAR(36),
            connecting_connector_guid VARCHAR(36),
            gap FLOAT DEFAULT 0,
            shift FLOAT DEFAULT 0,
            rise FLOAT DEFAULT 0,
            rotation FLOAT DEFAULT 0,
            turn FLOAT DEFAULT 0,
            tilt FLOAT DEFAULT 0,
            u FLOAT,
            v FLOAT,
            description TEXT,
        )
    """)

    cursor.execute("""
            guid VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256) NOT NULL,
            parent_guid VARCHAR(36)
        )
    """)

    cursor.execute("""
            guid VARCHAR(36) PRIMARY KEY,
            name VARCHAR(256) NOT NULL,
            mime VARCHAR(256),
            size INTEGER,
            hash VARCHAR(256),
            remote_url TEXT,
            folder_guid VARCHAR(36)
        )
    """)

    cursor.execute("""
            model_guid VARCHAR(36) NOT NULL,
            tag_guid VARCHAR(36) NOT NULL,
            PRIMARY KEY (model_guid, tag_guid)
        )
    """)

    cursor.execute("""
            id INTEGER PRIMARY KEY CHECK (id = 1),
        )
    """)

    now = datetime.now().isoformat()
    kit_guid = data.get("guid", str(uuid.uuid4()))

        """
        INSERT INTO kit (guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    """,
            kit_guid,
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
        type_guid = t.get("guid", str(uuid.uuid4()))
            """
            INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, location_guid, description, icon, image, created, updated, kit_guid)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
                type_guid,
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
                kit_guid,
            ),
        )

        for c in t.get("connectors", []):
            point = c.get("point", {})
            direction = c.get("direction", {})
                """
                INSERT INTO connector (guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description, type_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                    c.get("guid", str(uuid.uuid4())),
                    c.get("name"),
                    point.get("x", 0.0),
                    point.get("y", 0.0),
                    point.get("z", 0.0),
                    direction.get("x", 0.0),
                    direction.get("y", 1.0),
                    direction.get("z", 0.0),
                    c.get("t", 0.0),
                    1 if c.get("mandatory") else 0,
                    _getGuidFromRef(c.get("port")),
                    c.get("description"),
                    type_guid,
                ),
            )

        for m in t.get("models", []):
                """
                INSERT INTO model (guid, file_guid, name, description, type_guid)
                VALUES (?, ?, ?, ?, ?)
            """,
                    m.get("guid", str(uuid.uuid4())),
                    _getGuidFromRef(m.get("file")) or "",
                    m.get("name"),
                    m.get("description"),
                    type_guid,
                ),
            )

    for d in data.get("designs", []):
        design_guid = d.get("guid", str(uuid.uuid4()))
        view = d.get("view") or {}
        view_center = view.get("center") or {}
            """
            INSERT INTO design (guid, name, parent_guid, variant, view_center_u, view_center_v, view_zoom, unit, location_guid, active_layer_guid, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated, kit_guid)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
                design_guid,
                d.get("name", ""),
                _getGuidFromRef(d.get("parent")),
                d.get("variant"),
                view_center.get("u"),
                view_center.get("v"),
                view.get("zoom"),
                d.get("unit"),
                _getGuidFromRef(d.get("location")),
                _getGuidFromRef(d.get("activeLayer")),
                1 if d.get("isAbstract") else 0,
                d.get("folder"),
                1 if d.get("canScale") else (0 if d.get("canScale") is False else None),
                (1 if d.get("canMirror") else (0 if d.get("canMirror") is False else None)),
                d.get("description", ""),
                d.get("icon", ""),
                d.get("image", ""),
                now,
                now,
                kit_guid,
            ),
        )

        for p in d.get("pieces", []):
            plane = p.get("plane") or {}
            plane_origin = plane.get("origin") or {}
            plane_x_axis = plane.get("xAxis") or {}
            plane_y_axis = plane.get("yAxis") or {}
            mirror_plane = p.get("mirrorPlane") or {}
            mirror_origin = mirror_plane.get("origin") or {}
            mirror_x_axis = mirror_plane.get("xAxis") or {}
            mirror_y_axis = mirror_plane.get("yAxis") or {}
            center = p.get("center") or {}
                """
                INSERT INTO piece (guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z,
                    plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z,
                    center_u, center_v, scale, mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z,
                    mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z,
                    mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z,
                    is_hidden, is_locked, color, description, design_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                    p.get("guid", str(uuid.uuid4())),
                    p.get("name") or p.get("id"),
                    _getGuidFromRef(p.get("type")),
                    _getGuidFromRef(p.get("design")),
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
                    design_guid,
                ),
            )

        for c in d.get("connections", []):
            connected = c.get("connected", {})
            connecting = c.get("connecting", {})
            connected_piece = connected.get("piece")
            connected_design_piece = connected.get("designPiece")
            connected_connector = connected.get("connector")
            connecting_piece = connecting.get("piece")
            connecting_design_piece = connecting.get("designPiece")
            connecting_connector = connecting.get("connector")
                """
                INSERT INTO connection (guid, connected_piece_guid, connected_design_piece_guid, connected_connector_guid,
                    connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid,
                    gap, shift, rise, rotation, turn, tilt, u, v, description, design_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                    c.get("guid", str(uuid.uuid4())),
                    connected_piece_guid,
                    connected_design_piece_guid,
                    connected_connector_guid,
                    connecting_piece_guid,
                    connecting_design_piece_guid,
                    connecting_connector_guid,
                    c.get("gap", 0.0),
                    c.get("shift", 0.0),
                    c.get("rise", 0.0),
                    c.get("rotation", 0.0),
                    c.get("turn", 0.0),
                    c.get("tilt", 0.0),
                    c.get("u"),
                    c.get("v"),
                    c.get("description"),
                    design_guid,
                ),
            )

    for folder_entry in data.get("folders", []):
            """
            INSERT INTO folder (guid, name, parent_guid)
            VALUES (?, ?, ?)
        """,
                folder_entry.get("guid", str(uuid.uuid4())),
                folder_entry.get("name", ""),
                _getGuidFromRef(folder_entry.get("parent")),
            ),
        )

    for file_entry in data.get("files", []):
            """
            INSERT INTO file (guid, name, mime, size, hash, remote_url, folder_guid)
            VALUES (?, ?, ?, ?, ?, ?, ?)
        """,
                file_entry.get("guid", str(uuid.uuid4())),
                file_entry.get("name", ""),
                file_entry.get("mime"),
                file_entry.get("size"),
                file_entry.get("hash"),
                file_entry.get("remote"),
                _getGuidFromRef(file_entry.get("folder")),
            ),
        )

    for t in data.get("types", []):
        for m in t.get("models", []):
            model_guid = m.get("guid")
            for tag in m.get("tags", []):
                if model_guid and tag_guid:
                        "INSERT OR IGNORE INTO model_tag (model_guid, tag_guid) VALUES (?, ?)",
                        (model_guid, tag_guid),
                    )

        "INSERT INTO kit_payload (id, data) VALUES (1, ?)",
        (json.dumps(_kit_without_file_blobs(data), ensure_ascii=False),),
    )

    conn.commit()
    conn.close()


def export_kit(kit: KitData, files: dict[str, bytes], path: str) -> None:
    """Export a kit to a .zip file (containing kit.json and actual files).
    export_kit MUST write kit.json (without blob) and actual files to the target path.
    """
    import copy


    kit_for_zip = copy.deepcopy(data)
    for file_entry in kit_for_zip.get("files", []):
        file_entry.pop("blob", None)

    kit_json = json.dumps(kit_for_zip, ensure_ascii=False)

    with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as zip_ref:
        zip_ref.writestr("kit.json", kit_json)
        for filename, content in files.items():
            zip_ref.writestr(filename, content)


# endregion Kit Import/Export

# region Kit Model Export
# [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export)
# 3D model export utilities for designs. Exports design scene graphs as GLB, GLTF, OBJ, STL, PLY, OFF, IFC.

    ".glb": "model/gltf-binary",
    ".gltf": "model/gltf+json",
    ".obj": "model/obj",
    ".stl": "model/stl",
    ".ply": "application/x-ply",
    ".off": "application/x-off",
    ".ifc": "application/x-ifc",
}
"""Supported 3D export formats with their MIME types.
EXPORT_MODEL_FORMATS MUST map file extension to MIME type.
"""


def _plane_to_matrix_4x4(plane: "Plane") -> numpy.ndarray:
    """Convert a Plane to a 4x4 column-major transformation matrix.
    _plane_to_matrix_4x4 MUST produce an orthonormal basis with z = cross(x, y).
    """
    origin = numpy.array([plane.origin.x, plane.origin.y, plane.origin.z])
    x_axis = numpy.array([plane.xAxis.x, plane.xAxis.y, plane.xAxis.z])
    y_axis = numpy.array([plane.yAxis.x, plane.yAxis.y, plane.yAxis.z])
    z_axis = numpy.cross(x_axis, y_axis)
    nz = numpy.linalg.norm(z_axis)
    if nz > 1e-10:
    nx = numpy.linalg.norm(x_axis)
    if nx > 1e-10:
    y_axis = numpy.cross(z_axis, x_axis)
    ny = numpy.linalg.norm(y_axis)
    if ny > 1e-10:
    mat = numpy.eye(4)


def _semio_matrix_to_gltf_matrix(matrix: numpy.ndarray) -> numpy.ndarray:
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, -1.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ]
    )
    basis_inv = numpy.linalg.inv(basis)


def _identity_plane() -> "Plane":
    """Create an identity plane at the world origin with standard axes.
    _identity_plane MUST return a plane with origin=(0,0,0), xAxis=(1,0,0), yAxis=(0,1,0).
    """
    p = Plane()
    p.origin = Point(x=0.0, y=0.0, z=0.0)
    p.xAxis = Vector(x=1.0, y=0.0, z=0.0)
    p.yAxis = Vector(x=0.0, y=1.0, z=0.0)


def _type_key_from_id(type_id: "TypeId") -> str:
    """Build a unique string key from a TypeId (name:variant).
    _type_key_from_id MUST produce a consistent key for type matching.
    """


def _type_key_from_type(t: "Type") -> str:
    """Build a unique string key from a Type (name:variant).
    _type_key_from_type MUST produce a consistent key for type matching.
    """


def _find_matching_model(kit: "Kit", type_obj: "Type", tags: list[str]) -> typing.Optional["Model"]:
    """Find the best matching model for a type given requested tags.
    _find_matching_model MUST return the first model whose tags are all in the requested set, or the first model as fallback.
    """
    if not type_obj.models or len(type_obj.models) == 0:
    if not tags or len(tags) == 0:
        default_model = next((model for model in type_obj.models if len(model.tags or []) == 0), None)
        return default_model if default_model is not None else type_obj.models[0]
    tags_set = set(tags)
    for model in type_obj.models:
        if model_tag_names and all(t in tags_set for t in model_tag_names):
    return type_obj.models[0]


def _load_glb_mesh_from_bytes(raw: bytes, mesh_name: str | None = None) -> "typing.Any | None":
    """Load a mesh directly from GLB bytes by reading accessors.
    _load_glb_mesh_from_bytes MUST rebuild triangle faces from GLB accessor data without relying on trimesh GLB scene interpretation.
    """
    import struct as _struct

    import trimesh as _trimesh

    if len(raw) < 20 or raw[0:4] != b"glTF":

    while offset + 8 <= len(raw):
        chunk_length, chunk_kind = _struct.unpack_from("<II", raw, offset)
        chunk = raw[offset : offset + chunk_length]
        if chunk_kind == 0x4E4F534A:
        elif chunk_kind == 0x004E4942:
    if json_chunk is None:

    try:
        gltf = json.loads(json_chunk.decode("utf-8").rstrip(" \t\r\n\x00"))
    except Exception:

    accessors = gltf.get("accessors", []) or []
    buffer_views = gltf.get("bufferViews", []) or []
    meshes = gltf.get("meshes", []) or []

        5120: ("b", 1),
        5121: ("B", 1),
        5122: ("h", 2),
        5123: ("H", 2),
        5125: ("I", 4),
        5126: ("f", 4),
    }
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
        accessor = accessors[accessor_index]
        buffer_view_index = accessor.get("bufferView")
        if not isinstance(buffer_view_index, int) or buffer_view_index < 0 or buffer_view_index >= len(buffer_views):
        buffer_view = buffer_views[buffer_view_index]
        component_type = accessor.get("componentType")
        accessor_kind = accessor.get("type")
        count = accessor.get("count")
        if component_type not in component_formats or accessor_kind not in type_widths or not isinstance(count, int):
        if buffer_view.get("buffer", 0) != 0:
        fmt_char, component_size = component_formats[component_type]
        element_width = type_widths[accessor_kind]
        stride = buffer_view.get("byteStride") or (component_size * element_width)
        byte_offset = buffer_view.get("byteOffset", 0) + accessor.get("byteOffset", 0)
        values: list[tuple[typing.Any, ...]] = []
        for item_index in range(count):
            if end > len(bin_chunk):
            values.append(_struct.unpack_from("<" + fmt_char * element_width, bin_chunk, start))
        return numpy.array(values)

    vertex_blocks: list[numpy.ndarray] = []
    normal_blocks: list[numpy.ndarray] = []
    face_blocks: list[numpy.ndarray] = []

    for mesh in meshes:
        primitives = mesh.get("primitives", []) or []
        for primitive in primitives:
            attributes = primitive.get("attributes", {}) or {}
            position_accessor_index = attributes.get("POSITION")
            if not isinstance(position_accessor_index, int):
            positions = _read_accessor(position_accessor_index)
            if positions is None or positions.ndim != 2 or positions.shape[1] < 3:
            positions = positions[:, :3].astype(numpy.float64)
            normal_accessor_index = attributes.get("NORMAL")
            if isinstance(normal_accessor_index, int):
                normals = _read_accessor(normal_accessor_index)
                if normals is not None and normals.ndim == 2 and normals.shape[1] >= 3:
                    normals = normals[:, :3].astype(numpy.float64)
                else:
            if normals is None or len(normals) != len(positions):
            if isinstance(primitive.get("indices"), int):
                indices = _read_accessor(primitive.get("indices"))
                if indices is None:
                index_values = indices.reshape(-1).astype(numpy.int64)
            else:
                index_values = numpy.arange(len(positions), dtype=numpy.int64)
            if triangle_value_count == 0:
            triangle_faces = index_values[:triangle_value_count].reshape((-1, 3))
            vertex_offset = sum(len(block) for block in vertex_blocks)
            vertex_blocks.append(positions)
            if normals is not None and len(normals) == len(positions):
                normal_blocks.append(normals)
            face_blocks.append(triangle_faces + vertex_offset)

    if len(vertex_blocks) == 0 or len(face_blocks) == 0:

    combined_vertices = numpy.vstack(vertex_blocks)
    combined_faces = numpy.vstack(face_blocks)
        vertices=combined_vertices,
        faces=combined_faces,
        process=False,
        maintain_order=True,
    )
    if has_normals and len(normal_blocks) == len(vertex_blocks):
        combined_normals = numpy.vstack(normal_blocks)
        if len(combined_normals) == len(combined_vertices):
    if mesh_name:


def _load_type_mesh(kit: "Kit", type_obj: "Type", tags: list[str]) -> "typing.Any | None":
    """Load the 3D mesh for a type from its best-matching model blob.
    _load_type_mesh MUST decode the base64 blob, load with trimesh, and return a single Trimesh.
    """
    import base64 as _base64

    import trimesh as _trimesh

    model = _find_matching_model(kit, type_obj, tags)
    if model is None:
    files_list = kit.files_ or []
    file_obj = next((f for f in files_list if f.name == file_id or f.guid == file_id), None)
    if file_obj is None or not file_obj.blob:
    if blob.startswith("data:"):
        raw = _base64.b64decode(blob.split(",", 1)[1])
    else:
        raw = _base64.b64decode(blob)
    direct_mesh = _load_glb_mesh_from_bytes(raw, file_obj.name)
    if direct_mesh is not None:
    try:
            _trimesh.util.wrap_as_stream(raw),
            file_type="glb",
        )
    except Exception:
    if isinstance(loaded, _trimesh.Scene):
        if len(loaded.geometry) == 0:
        meshes = [geometry.copy() for geometry in loaded.geometry.values() if isinstance(geometry, _trimesh.Trimesh) and len(getattr(geometry, "faces", [])) > 0]
        if not meshes:
        if len(meshes) == 1:
            return meshes[0]
        return _trimesh.util.concatenate(meshes)
    if isinstance(loaded, _trimesh.Trimesh) and len(getattr(loaded, "faces", [])) > 0:


def export_design_model(
    kit: "Kit",
    design_id: str,
    format: str = ".glb",
    tags: list[str] | None = None,
    options: dict | None = None,
) -> bytes:
    """Export the 3D model of a design to a specified format.
    export_design_model MUST produce a valid 3D file. Uses block definitions for types and instances for pieces.
    Connection hierarchy is translated into a scene graph; planes become relative transformation matrices.
    """
    import trimesh as _trimesh

    if tags is None:
        tags = []
    if options is None:
        options = {}
    if format not in EXPORT_MODEL_FORMATS:
        raise ValueError(f"Unsupported export format '{format}'. Supported: {list(EXPORT_MODEL_FORMATS.keys())}")

    if isinstance(kit, dict):
        designs = kit.get("designs", []) or []
            (d for d in designs if d.get("name") == design_id or d.get("guid") == design_id),
            None,
        )
        if design is None:
            raise ValueError(f"Design '{design_id}' not found in kit")
        pieces = design.get("pieces", []) or []
        connections = design.get("connections", []) or []
        if len(pieces) == 0:
            return _export_empty_scene(format)

        types_by_guid = {type_obj.get("guid"): type_obj for type_obj in (kit.get("types", []) or []) if type_obj.get("guid")}

        def _find_type_for_piece_dict(piece_dict: dict) -> dict | None:
            type_ref = piece_dict.get("type")
            if not isinstance(type_ref, dict):
            return types_by_guid.get(type_ref.get("guid"))

        def _find_connector_dict(type_obj: dict | None, connector_guid: str | None) -> dict | None:
            while current is not None:
                connectors = current.get("connectors", []) or []
                if connector_guid is None:
                for connector in connectors:
                    if connector.get("guid") == connector_guid:
                parent_ref = current.get("parent")

        piece_by_guid = {piece.get("guid"): piece for piece in pieces if piece.get("guid")}
        adjacency: dict[str, list[tuple[dict, str]]] = {piece_guid: [] for piece_guid in piece_by_guid}
        for connection in connections:
            connected_guid = connection.get("connected", {}).get("piece", {}).get("guid")
            connecting_guid = connection.get("connecting", {}).get("piece", {}).get("guid")
            if connected_guid in adjacency:
                adjacency[connected_guid].append((connection, connecting_guid))
            if connecting_guid in adjacency:
                adjacency[connecting_guid].append((connection, connected_guid))

        def _identity_plane_dict() -> dict:
                "origin": {"x": 0.0, "y": 0.0, "z": 0.0},
                "xAxis": {"x": 1.0, "y": 0.0, "z": 0.0},
                "yAxis": {"x": 0.0, "y": 1.0, "z": 0.0},
            }

        def _plane_dict_to_matrix(plane_dict: dict) -> numpy.ndarray:
                    plane_dict["origin"]["x"],
                    plane_dict["origin"]["y"],
                    plane_dict["origin"]["z"],
                ],
                dtype=numpy.float64,
            )
                    plane_dict["xAxis"]["x"],
                    plane_dict["xAxis"]["y"],
                    plane_dict["xAxis"]["z"],
                ],
                dtype=numpy.float64,
            )
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

        piece_planes: dict[str, dict] = {}
        parent_of: dict[str, str] = {}
        children_of: dict[str, list[str]] = {piece_guid: [] for piece_guid in piece_by_guid}
        visited: set[str] = set()
        roots: list[str] = []
        queue: list[str] = []

        for piece in pieces:
            piece_guid = piece.get("guid")
            if piece_guid is None:
            if piece.get("plane") is not None and piece.get("center") is not None:
                piece_planes[piece_guid] = piece.get("plane")
                visited.add(piece_guid)
                queue.append(piece_guid)
                roots.append(piece_guid)
        if len(queue) == 0 and len(pieces) > 0 and pieces[0].get("guid") is not None:
            first_guid = pieces[0].get("guid")
            piece_planes[first_guid] = _identity_plane_dict()
            visited.add(first_guid)
            queue.append(first_guid)
            roots.append(first_guid)

        while queue:
            current_guid = queue.pop(0)
            current_plane = piece_planes[current_guid]
            for connection, neighbor_guid in adjacency.get(current_guid, []):
                if neighbor_guid in visited:
                if connection.get("connected", {}).get("piece", {}).get("guid") != current_guid:
                parent_piece = piece_by_guid[current_guid]
                child_piece = piece_by_guid[neighbor_guid]
                parent_type = _find_type_for_piece_dict(parent_piece)
                child_type = _find_type_for_piece_dict(child_piece)
                    parent_type,
                    connection.get("connected", {}).get("connector", {}).get("guid"),
                )
                    child_type,
                    connection.get("connecting", {}).get("connector", {}).get("guid"),
                )
                if parent_connector is not None and child_connector is not None:
                    piece_planes[neighbor_guid] = computeChildPlaneDict(current_plane, parent_connector, child_connector, connection)
                else:
                children_of[current_guid].append(neighbor_guid)
                visited.add(neighbor_guid)
                queue.append(neighbor_guid)

        for piece in pieces:
            piece_guid = piece.get("guid")
            if piece_guid is None:
            if piece_guid not in visited:
                piece_planes[piece_guid] = _identity_plane_dict()
                roots.append(piece_guid)

        if format == ".ifc":
            return _export_ifc_from_dict(kit, design_id, piece_planes, parent_of, children_of, roots, tags)

        def _select_model_dict(type_obj: dict) -> dict | None:
            models = type_obj.get("models", []) or []
            if len(models) == 0:
            tag_lookup = {tag.get("guid"): tag for tag in (kit.get("tags", []) or []) if tag.get("guid")}
            if len(tags) == 0:
                    (model for model in models if len(model.get("tags", []) or []) == 0),
                    None,
                )
                return default_model if default_model is not None else models[0]
            selected_tag_guids: set[str] = set()
            for tag_value in tags:
                if tag_value in tag_lookup:
                    selected_tag_guids.add(tag_value)
                for tag in tag_lookup.values():
                    if tag.get("name") == tag_value:
                        selected_tag_guids.add(tag.get("guid"))
            for model in models:
                model_tag_guids = {tag.get("guid") for tag in (model.get("tags", []) or []) if tag.get("guid")}
                if not selected_tag_guids.issubset(model_tag_guids):
                union = len(model_tag_guids.union(selected_tag_guids))
                intersection = len(model_tag_guids.intersection(selected_tag_guids))
                if score > best_score:
            return best_model if best_model is not None else models[0]

        scene = _trimesh.Scene()
        type_meshes: dict[str, str] = {}
        files_by_guid = {file_entry.get("guid"): file_entry for file_entry in (kit.get("files", []) or []) if file_entry.get("guid")}
        for piece in pieces:
            if type_guid is None or type_guid in type_meshes:
            type_obj = types_by_guid.get(type_guid)
            if type_obj is None:
            selected_model = _select_model_dict(type_obj)
            if selected_file is not None and selected_file.get("blob"):
                try:
                    blob = selected_file.get("blob")
                    raw = base64.b64decode(blob.split(",", 1)[1] if isinstance(blob, str) and blob.startswith("data:") else blob)
                    mesh = _load_glb_mesh_from_bytes(raw, selected_file.get("name"))
                    if mesh is None:
                        loaded = _trimesh.load(_trimesh.util.wrap_as_stream(raw), file_type="glb")
                        if isinstance(loaded, _trimesh.Scene):
                            dumped = [geometry.copy() for geometry in loaded.geometry.values() if isinstance(geometry, _trimesh.Trimesh) and len(getattr(geometry, "faces", [])) > 0]
                            mesh = dumped[0] if len(dumped) == 1 else (_trimesh.util.concatenate(dumped) if len(dumped) > 1 else None)
                        elif isinstance(loaded, _trimesh.Trimesh) and len(getattr(loaded, "faces", [])) > 0:
                    if mesh is not None and selected_file.get("name"):
                        mesh.metadata["name"] = selected_file.get("name")
                except Exception:
            if mesh is None:

        for piece in pieces:
            piece_guid = piece.get("guid")
            world_plane = piece_planes[piece_guid]
            parent_guid = parent_of.get(piece_guid)
            if parent_guid and parent_guid in piece_planes:
                parent_world = _plane_dict_to_matrix(piece_planes[parent_guid])
                child_world = _plane_dict_to_matrix(world_plane)
                frame_from = piece_by_guid[parent_guid].get("name") or parent_guid
            else:
                relative = _plane_dict_to_matrix(world_plane)
                frame_from = scene.graph.base_frame
            relative = _semio_matrix_to_gltf_matrix(relative)
            if type_guid in type_meshes:
                geom_name = type_meshes[type_guid]
                frame_from=frame_from,
                frame_to=piece_frame,
                matrix=relative,
                geometry=geom_name,
            )
        return _export_trimesh_scene(scene, format)

    for d in kit.designs:
        if d.name == design_id or d.id() == design_id:
    if design is None:
        raise ValueError(f"Design '{design_id}' not found in kit")

    pieces = design.pieces or []
    connections = design.connections or []
    types_list = kit.types or []

    if len(pieces) == 0:
        return _export_empty_scene(format)

    types_dict: dict[str, Type] = {}
    for t in types_list:

    pieces_dict: dict[str, Piece] = {}
    for p in pieces:

    adjacency: dict[str, list[tuple[Connection, str]]] = {}
    for p in pieces:
        adjacency[p.id_] = []
    for conn in connections:
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
        return types_dict.get(_type_key_from_id(piece.type))

    def _get_connector(type_obj: Type | None, connector_id: ConnectorId | None) -> Connector | None:
        if type_obj is None:
        if not type_obj.connectors:
        if connector_id is None:
            return type_obj.connectors[0]
        return next((c for c in type_obj.connectors if c.id_ == connector_id.id_), None)

    queue: list[str] = []
    for p in pieces:
        if p.plane is not None and p.center is not None:
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
            if not is_parent:

            parent_piece = pieces_dict[parent_id]
            child_piece = pieces_dict[child_id]
            parent_type = _get_type(parent_piece)
            child_type = _get_type(child_piece)
            parent_connector = _get_connector(parent_type, conn.connected.connector)
            child_connector = _get_connector(child_type, conn.connecting.connector)

            if parent_connector and child_connector:
                child_plane = computeChildPlane(current_plane, parent_connector, child_connector, conn)
            else:

            children_of[parent_id].append(child_id)
            visited.add(child_id)
            queue.append(child_id)

    for p in pieces:
        if p.id_ not in visited:
            piece_planes[p.id_] = _identity_plane()
            roots.append(p.id_)

    if format == ".ifc":
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

    # region Load or create meshes per type
    type_meshes: dict[str, str] = {}
    for piece in pieces:
        if piece.type is None:
        tk = _type_key_from_id(piece.type)
        if tk in type_meshes:
        type_obj = types_dict.get(tk)
        if type_obj is None:
        mesh = _load_type_mesh(kit, type_obj, tags)
        if mesh is None:
        model = _find_matching_model(kit, type_obj, tags)
        if model is not None:
        if not geometry_name:
    # endregion Load or create meshes per type

    # region Build scene graph with connection hierarchy
    def _build_node(piece_id: str) -> None:
        piece = pieces_dict[piece_id]
        world_plane = piece_planes[piece_id]
        p_parent = parent_of.get(piece_id)
        children = children_of.get(piece_id, [])

        if p_parent and p_parent in piece_planes:
            parent_world = _plane_to_matrix_4x4(piece_planes[p_parent])
            child_world = _plane_to_matrix_4x4(world_plane)
            relative = _semio_matrix_to_gltf_matrix(numpy.linalg.inv(parent_world) @ child_world)
            parent_piece = pieces_dict[p_parent]
            frame_from = parent_piece.name or p_parent
        else:
            relative = _semio_matrix_to_gltf_matrix(_plane_to_matrix_4x4(world_plane))
            frame_from = scene.graph.base_frame

        if piece.type is not None:
            tk = _type_key_from_id(piece.type)
            if tk in type_meshes:
                geom_name = type_meshes[tk]

            frame_from=frame_from,
            frame_to=piece_frame,
            matrix=relative,
            geometry=geom_name,
        )

        for child_id in children:
            _build_node(child_id)

    for root_id in roots:
        _build_node(root_id)
    # endregion Build scene graph with connection hierarchy

    return _export_trimesh_scene(scene, format)


def _export_empty_scene(format: str) -> bytes:
    """Export a minimal valid empty scene for the requested format.
    _export_empty_scene MUST return bytes representing a valid but empty 3D file.
    """
    import struct as _struct

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
        json_bytes = json_str.encode("utf-8")
        total_length = 12 + 8 + len(json_bytes)
        result = bytearray(total_length)
        _struct.pack_into("<I", result, 0, 0x46546C67)
        _struct.pack_into("<I", result, 4, 2)
        _struct.pack_into("<I", result, 8, total_length)
        _struct.pack_into("<I", result, 12, len(json_bytes))
        _struct.pack_into("<I", result, 16, 0x4E4F534A)
        return bytes(result)


def _export_trimesh_scene(scene: "typing.Any", format: str) -> bytes:
    """Export a trimesh.Scene to the requested format as bytes.
    _export_trimesh_scene MUST return bytes for all supported formats.
    """
    import base64

    import trimesh as _trimesh

    fmt = format.lstrip(".")

    if fmt == "gltf":
        exported = scene.export(file_type="gltf")
        if isinstance(exported, dict):
            gltf_key = next((key for key in exported.keys() if key.endswith(".gltf")), None)
            if gltf_key is not None:
                gltf_value = exported[gltf_key]
                gltf_json = json.loads(gltf_value.decode("utf-8") if isinstance(gltf_value, bytes) else (json.dumps(gltf_value) if isinstance(gltf_value, dict) else str(gltf_value)))
                for buffer in gltf_json.get("buffers", []) or []:
                    uri = buffer.get("uri")
                    if not uri or uri.startswith("data:") or uri not in exported:
                    buffer["uri"] = "data:application/octet-stream;base64," + base64.b64encode(exported[uri]).decode("ascii")
                for image in gltf_json.get("images", []) or []:
                    uri = image.get("uri")
                    if not uri or uri.startswith("data:") or uri not in exported:
                    mime = image.get("mimeType", "application/octet-stream")
                    image["uri"] = f"data:{mime};base64," + base64.b64encode(exported[uri]).decode("ascii")
                return json.dumps(gltf_json).encode("utf-8")
            for key, value in exported.items():
                if key.endswith(".gltf"):
                    if isinstance(value, bytes):
                    if isinstance(value, dict):
                        return json.dumps(value).encode("utf-8")
                    return str(value).encode("utf-8")
            return json.dumps(exported).encode("utf-8")
        if isinstance(exported, bytes):
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


# region IFC Export
# [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🔖ifcexport](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/s/IFC%20Export)
# IFC exporter mapping semio domain to IFC4 schema via ifcopenshell.


def _gltf_xyz_to_semio_xyz(x: float, y: float, z: float) -> tuple[float, float, float]:
    """Convert glTF coordinates to semio/IFC coordinates.
    _gltf_xyz_to_semio_xyz MUST map glTF +Y up geometry back to semio/IFC +Z up geometry.
    """
    return (float(x), float(-z), float(y))


def _glb_bytes_to_vertices_faces(
    raw: bytes,
) -> tuple[list[tuple[float, float, float]], list[tuple[int, ...]]] | None:
    """Extract vertices and faces from GLB bytes for IFC mesh representation.
    _glb_bytes_to_vertices_faces MUST return (vertices, faces) or None if parsing fails.
    """
    import struct as _struct

    if len(raw) < 20 or raw[0:4] != b"glTF":
    while offset + 8 <= len(raw):
        chunk_length, chunk_kind = _struct.unpack_from("<II", raw, offset)
        chunk = raw[offset : offset + chunk_length]
        if chunk_kind == 0x4E4F534A:
        elif chunk_kind == 0x004E4942:
    if json_chunk is None:
    try:
        gltf = json.loads(json_chunk.decode("utf-8").rstrip(" \t\r\n\x00"))
    except Exception:
    accessors = gltf.get("accessors", []) or []
    buffer_views = gltf.get("bufferViews", []) or []
    meshes = gltf.get("meshes", []) or []
        5120: ("b", 1),
        5121: ("B", 1),
        5122: ("h", 2),
        5123: ("H", 2),
        5125: ("I", 4),
        5126: ("f", 4),
    }
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
        accessor = accessors[accessor_index]
        buffer_view_index = accessor.get("bufferView")
        if not isinstance(buffer_view_index, int) or buffer_view_index < 0 or buffer_view_index >= len(buffer_views):
        buffer_view = buffer_views[buffer_view_index]
        component_type = accessor.get("componentType")
        accessor_kind = accessor.get("type")
        count = accessor.get("count")
        if component_type not in component_formats or accessor_kind not in type_widths or not isinstance(count, int):
        if buffer_view.get("buffer", 0) != 0:
        fmt_char, component_size = component_formats[component_type]
        element_width = type_widths[accessor_kind]
        stride = buffer_view.get("byteStride") or (component_size * element_width)
        byte_offset = buffer_view.get("byteOffset", 0) + accessor.get("byteOffset", 0)
        values: list[tuple[typing.Any, ...]] = []
        for item_index in range(count):
            if end > len(bin_chunk):
            values.append(_struct.unpack_from("<" + fmt_char * element_width, bin_chunk, start))
        return numpy.array(values)

    all_vertices: list[tuple[float, float, float]] = []
    all_faces: list[tuple[int, ...]] = []
    for mesh in meshes:
        primitives = mesh.get("primitives", []) or []
        for primitive in primitives:
            attributes = primitive.get("attributes", {}) or {}
            position_accessor_index = attributes.get("POSITION")
            if not isinstance(position_accessor_index, int):
            positions = _read_accessor(position_accessor_index)
            if positions is None or positions.ndim != 2 or positions.shape[1] < 3:
            vertex_offset = len(all_vertices)
            for row in positions:
                all_vertices.append(_gltf_xyz_to_semio_xyz(float(row[0]), float(row[1]), float(row[2])))
            if isinstance(primitive.get("indices"), int):
                indices = _read_accessor(primitive.get("indices"))
                if indices is None:
                index_values = indices.reshape(-1).astype(int)
            else:
                index_values = numpy.arange(len(positions), dtype=int)
            for tri_idx in range(triangle_count):
                all_faces.append((i0, i1, i2))
    if len(all_vertices) == 0 or len(all_faces) == 0:
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
    """Export a design to IFC4 format from dict-based kit data.
    _export_ifc_from_dict MUST produce a valid IFC4 file with spatial hierarchy, typed occurrences and mesh geometry.
    """
    import ifcopenshell as _ifc
    import ifcopenshell.api as _ifc_api
    import ifcopenshell.guid as _ifc_guid

    # region Step 1: IFC file, project, units, context, spatial tree from layers
    ifc = _ifc_api.run("project.create_file", version="IFC4")
    kit_name = kit.get("name", "semio Kit")
    project = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcProject", name=kit_name)
    _ifc_api.run("unit.assign_unit", ifc)
    model_context = _ifc_api.run("context.add_context", ifc, context_type="Model")
        "context.add_context",
        ifc,
        context_type="Model",
        context_identifier="Body",
        target_view="MODEL_VIEW",
        parent=model_context,
    )
    site = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcSite", name="Site")
    _ifc_api.run("aggregate.assign_object", ifc, relating_object=project, products=[site])

    designs = kit.get("designs", []) or []
        (d for d in designs if d.get("name") == design_name or d.get("guid") == design_name),
        None,
    )
    layers = (design.get("layers", []) or []) if design else []

    def _get_layer_ifc_type(layer: dict) -> str | None:
        for attr in layer.get("attributes", []) or []:
            if attr.get("key") == "ifc.type":
                return attr.get("value")

    # Build spatial hierarchy from layers
    ifc_buildings: dict[str, typing.Any] = {}
    ifc_storeys: dict[str, typing.Any] = {}
    storey_by_number: dict[int, typing.Any] = {}

    for layer in layers:
        layer_path = layer.get("path", "")
        ifc_type = _get_layer_ifc_type(layer)
        if ifc_type == "IfcBuilding":
            building = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuilding", name=layer_path)
                "aggregate.assign_object",
                ifc,
                relating_object=site,
                products=[building],
            )
            if default_building is None:
        elif ifc_type == "IfcBuildingStorey":
            parts = layer_path.rsplit("/", 1)
                "root.create_entity",
                ifc,
                ifc_class="IfcBuildingStorey",
                name=storey_name,
            )
            parent_building = ifc_buildings.get(parent_path)
            if parent_building is not None:
                    "aggregate.assign_object",
                    ifc,
                    relating_object=parent_building,
                    products=[storey],
                )
            try:
                storey_number = int(storey_name)
            except ValueError:
            if default_storey is None:

    # Fallback: create default building and storey if no layers define them
    if default_building is None:
        default_building = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuilding", name="Building")
            "aggregate.assign_object",
            ifc,
            relating_object=site,
            products=[default_building],
        )
    if default_storey is None:
        default_storey = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuildingStorey", name="Storey")
            "aggregate.assign_object",
            ifc,
            relating_object=default_building,
            products=[default_storey],
        )
    # endregion Step 1

    # region Step 2: Piece-to-storey mapping from piece names
    import re as _re

    def _piece_storey(piece_name: str) -> typing.Any:
        m = _re.search(r"_f(\d+)_", piece_name or "")
        if m:
            floor = int(m.group(1))
            if floor in storey_by_number:
                return storey_by_number[floor]

    # endregion Step 2

    pieces = (design.get("pieces", []) or []) if design else []
    connections = (design.get("connections", []) or []) if design else []
    types_by_guid = {t.get("guid"): t for t in (kit.get("types", []) or []) if t.get("guid")}
    files_by_guid = {f.get("guid"): f for f in (kit.get("files", []) or []) if f.get("guid")}
    piece_by_guid = {p.get("guid"): p for p in pieces if p.get("guid")}
    tag_lookup = {tag.get("guid"): tag for tag in (kit.get("tags", []) or []) if tag.get("guid")}

    # region Step 3: Types with geometry
    ifc_types: dict[str, typing.Any] = {}
    for piece in pieces:
        type_ref = piece.get("type")
        if type_guid is None or type_guid in ifc_types:
        type_obj = types_by_guid.get(type_guid)
        if type_obj is None:
        type_name = type_obj.get("name", type_guid)
        type_variant = type_obj.get("variant", "")
            "root.create_entity",
            ifc,
            ifc_class="IfcBuildingElementProxyType",
            name=ifc_type_name,
        )

        # Type-level pset for type attributes
        type_attrs = type_obj.get("attributes", []) or []
        if type_attrs:
            type_pset = _ifc_api.run("pset.add_pset", ifc, product=ifc_type, name="SemioTypeAttributes")
            props = {}
            for attr in type_attrs:
                key = attr.get("key", "")
                value = attr.get("value", "")
                if key:
            if props:
                _ifc_api.run("pset.edit_pset", ifc, pset=type_pset, properties=props)

        # Type-level metadata pset
        type_meta = {}
        if type_obj.get("description"):
            type_meta["description"] = type_obj.get("description")
        if type_obj.get("variant"):
            type_meta["variant"] = type_obj.get("variant")
        if type_obj.get("guid"):
            type_meta["semioGuid"] = type_obj.get("guid")
        if type_meta:
            meta_pset = _ifc_api.run("pset.add_pset", ifc, product=ifc_type, name="SemioTypeMetadata")
            _ifc_api.run("pset.edit_pset", ifc, pset=meta_pset, properties=type_meta)

        # Geometry: find best model, extract GLB mesh
        models = type_obj.get("models", []) or []
        if models:
            selected_tag_guids: set[str] = set()
            for tag_value in tags:
                if tag_value in tag_lookup:
                    selected_tag_guids.add(tag_value)
                else:
                    for tag in tag_lookup.values():
                        if tag.get("name") == tag_value:
                            selected_tag_guids.add(tag.get("guid"))
            if not selected_tag_guids:
                selected_model = next((m for m in models if len(m.get("tags", []) or []) == 0), None) or models[0]
            else:
                for m in models:
                    model_tag_guids = {t.get("guid") if isinstance(t, dict) else t for t in (m.get("tags", []) or [])}
                    if selected_tag_guids.issubset(model_tag_guids):
                if selected_model is None:
                    selected_model = models[0]

        if selected_model is not None:
            file_ref = selected_model.get("file", {})
            file_obj = files_by_guid.get(file_guid)
            if file_obj is not None and file_obj.get("blob"):
                blob = file_obj.get("blob")
                raw = base64.b64decode(blob.split(",", 1)[1] if isinstance(blob, str) and blob.startswith("data:") else blob)
                result = _glb_bytes_to_vertices_faces(raw)
                if result is not None:
                        "geometry.add_mesh_representation",
                        ifc,
                        context=body_context,
                        vertices=[list(vertices)],
                        faces=[list(faces)],
                    )
                        "geometry.assign_representation",
                        ifc,
                        product=ifc_type,
                        representation=rep,
                    )

    # endregion Step 3

    # region Step 4: Pieces as occurrences
    ifc_occurrences: dict[str, typing.Any] = {}
    ifc_connector_ports: dict[str, dict[str, typing.Any]] = {}
    for piece in pieces:
        piece_guid = piece.get("guid")
        if piece_guid is None:
            "root.create_entity",
            ifc,
            ifc_class="IfcBuildingElementProxy",
            name=piece_name,
        )

        type_ref = piece.get("type")
        if type_guid and type_guid in ifc_types:
                "type.assign_type",
                ifc,
                related_objects=[occurrence],
                relating_type=ifc_types[type_guid],
            )

        # World placement from computed planes
        world_plane = piece_planes.get(piece_guid)
        if world_plane is not None:
            origin = world_plane.get("origin", {})
            x_axis = world_plane.get("xAxis", {})
            y_axis = world_plane.get("yAxis", {})
                origin.get("x", 0.0),
                origin.get("y", 0.0),
                origin.get("z", 0.0),
            )
                x_axis.get("x", 1.0),
                x_axis.get("y", 0.0),
                x_axis.get("z", 0.0),
            )
                y_axis.get("x", 0.0),
                y_axis.get("y", 1.0),
                y_axis.get("z", 0.0),
            )
            x_vec = numpy.array([xx, xy, xz], dtype=numpy.float64)
            y_vec = numpy.array([yx, yy, yz], dtype=numpy.float64)
            z_vec = numpy.cross(x_vec, y_vec)
            nz = numpy.linalg.norm(z_vec)
            if nz > 1e-10:
            nx = numpy.linalg.norm(x_vec)
            if nx > 1e-10:
            y_vec = numpy.cross(z_vec, x_vec)
            ny = numpy.linalg.norm(y_vec)
            if ny > 1e-10:
            mat = numpy.eye(4)
            mat[:3, 3] = [ox, oy, oz]
            _ifc_api.run("geometry.edit_object_placement", ifc, product=occurrence, matrix=mat)

        # Assign piece to the correct storey based on its floor number
            "spatial.assign_container",
            ifc,
            relating_structure=_piece_storey(piece_name),
            products=[occurrence],
        )

        # Piece-level pset for piece attributes
        piece_props: dict[str, typing.Any] = {}
        if piece.get("name"):
            piece_props["name"] = piece.get("name")
        if piece.get("guid"):
            piece_props["semioGuid"] = piece.get("guid")
        piece_attrs = piece.get("attributes", []) or []
        for attr in piece_attrs:
            key = attr.get("key", "")
            value = attr.get("value", "")
            if key:
        if piece_props:
            piece_pset = _ifc_api.run("pset.add_pset", ifc, product=occurrence, name="SemioPieceAttributes")
            _ifc_api.run("pset.edit_pset", ifc, pset=piece_pset, properties=piece_props)


        # Connectors as ports
        if type_obj is not None:
            connectors = type_obj.get("connectors", []) or []
            ifc_connector_ports[piece_guid] = {}
            for conn in connectors:
                conn_id = conn.get("guid") or conn.get("id_") or conn.get("name", "")
                    "root.create_entity",
                    ifc,
                    ifc_class="IfcDistributionPort",
                    name=conn_id,
                )
                    "nest.assign_object",
                    ifc,
                    relating_object=occurrence,
                    related_objects=[port],
                )

                # Port placement relative to element (connector point/direction)
                point = conn.get("point", {})
                if point:
                    port_mat = numpy.eye(4)
                        point.get("x", 0.0),
                        point.get("y", 0.0),
                        point.get("z", 0.0),
                    ]
                    direction = conn.get("direction", {})
                    if direction:
                                direction.get("x", 0.0),
                                direction.get("y", 0.0),
                                direction.get("z", 1.0),
                            ]
                        )
                        dn = numpy.linalg.norm(d)
                        if dn > 1e-10:
                            up = numpy.array([0.0, 0.0, 1.0])
                            if abs(numpy.dot(z, up)) > 0.99:
                                up = numpy.array([1.0, 0.0, 0.0])
                            x = numpy.cross(up, z)
                            xn = numpy.linalg.norm(x)
                            if xn > 1e-10:
                            y = numpy.cross(z, x)
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
                    conn_props["semioPort"] = port_val if isinstance(port_val, str) else str(port_val)
                conn_pset = _ifc_api.run("pset.add_pset", ifc, product=port, name="SemioConnector")
                _ifc_api.run("pset.edit_pset", ifc, pset=conn_pset, properties=conn_props)

    # endregion Step 4

    # region Step 5: Connections as port relationships
    for connection in connections:
        connected = connection.get("connected", {})
        connecting = connection.get("connecting", {})
        connected_piece_guid = connected.get("piece", {}).get("guid")
        connecting_piece_guid = connecting.get("piece", {}).get("guid")

        if connected_piece_guid in ifc_connector_ports and connected_connector_guid:
            connected_port = ifc_connector_ports[connected_piece_guid].get(connected_connector_guid)
        if connecting_piece_guid in ifc_connector_ports and connecting_connector_guid:
            connecting_port = ifc_connector_ports[connecting_piece_guid].get(connecting_connector_guid)

        # IfcRelConnectsPorts
        if connected_port is not None and connecting_port is not None:
                "IfcRelConnectsPorts",
                GlobalId=_ifc_guid.new(),
                RelatingPort=connected_port,
                RelatedPort=connecting_port,
            )

        # IfcRelConnectsElements
        connected_elem = ifc_occurrences.get(connected_piece_guid)
        connecting_elem = ifc_occurrences.get(connecting_piece_guid)
        if connected_elem is not None and connecting_elem is not None:
                "IfcRelConnectsElements",
                GlobalId=_ifc_guid.new(),
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
                "pset.add_pset",
                ifc,
                product=connected_elem,
                name="SemioConnectionParams",
            )
            _ifc_api.run("pset.edit_pset", ifc, pset=conn_pset, properties=conn_solver_props)
    # endregion Step 5

    # region Step 6: Kit-level metadata
    kit_meta: dict[str, typing.Any] = {}
    if kit.get("name"):
        kit_meta["name"] = kit.get("name")
    if kit.get("description"):
        kit_meta["description"] = kit.get("description")
    if kit.get("guid"):
        kit_meta["semioGuid"] = kit.get("guid")
    if kit.get("uri"):
        kit_meta["semioUri"] = kit.get("uri")
    authors = kit.get("authors", []) or []
    if authors:
        author_strs = [f"{a.get('name', '')} <{a.get('email', '')}>" for a in authors]
        kit_meta["authors"] = "; ".join(author_strs)
    if kit_meta:
        kit_pset = _ifc_api.run("pset.add_pset", ifc, product=project, name="SemioKitMetadata")
        _ifc_api.run("pset.edit_pset", ifc, pset=kit_pset, properties=kit_meta)
    # endregion Step 6

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
    """Export a design to IFC4 format from entity-based kit data.
    _export_ifc_from_entities MUST produce a valid IFC4 file with spatial hierarchy, typed occurrences and mesh geometry.
    """
    import ifcopenshell as _ifc
    import ifcopenshell.api as _ifc_api
    import ifcopenshell.guid as _ifc_guid

    # region Step 1: IFC file, project, units, context, spatial tree from layers
    ifc = _ifc_api.run("project.create_file", version="IFC4")
    project = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcProject", name=kit_name)
    _ifc_api.run("unit.assign_unit", ifc)
    model_context = _ifc_api.run("context.add_context", ifc, context_type="Model")
        "context.add_context",
        ifc,
        context_type="Model",
        context_identifier="Body",
        target_view="MODEL_VIEW",
        parent=model_context,
    )
    site = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcSite", name="Site")
    _ifc_api.run("aggregate.assign_object", ifc, relating_object=project, products=[site])

    layers = design.layers or [] if hasattr(design, "layers") else []

    def _get_layer_ifc_type_entity(layer: typing.Any) -> str | None:
        if hasattr(layer, "attributes"):
            for attr in layer.attributes or []:
                key = attr.key if hasattr(attr, "key") else attr.get("key", "")
                value = attr.value if hasattr(attr, "value") else attr.get("value", "")
                if key == "ifc.type":

    ifc_buildings: dict[str, typing.Any] = {}
    ifc_storeys: dict[str, typing.Any] = {}
    storey_by_number: dict[int, typing.Any] = {}

    for layer in layers:
        layer_name = layer.name if hasattr(layer, "name") else layer.get("name", "")
        ifc_type_val = _get_layer_ifc_type_entity(layer)
        if ifc_type_val == "IfcBuilding":
            building = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuilding", name=layer_name)
                "aggregate.assign_object",
                ifc,
                relating_object=site,
                products=[building],
            )
            if default_building is None:
        elif ifc_type_val == "IfcBuildingStorey":
            parts = layer_name.rsplit("/", 1)
                "root.create_entity",
                ifc,
                ifc_class="IfcBuildingStorey",
                name=storey_label,
            )
            parent_building = ifc_buildings.get(parent_name)
            if parent_building is not None:
                    "aggregate.assign_object",
                    ifc,
                    relating_object=parent_building,
                    products=[storey_ent],
                )
            try:
            except ValueError:
            if default_storey is None:

    if default_building is None:
        default_building = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuilding", name="Building")
            "aggregate.assign_object",
            ifc,
            relating_object=site,
            products=[default_building],
        )
    if default_storey is None:
        default_storey = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuildingStorey", name="Storey")
            "aggregate.assign_object",
            ifc,
            relating_object=default_building,
            products=[default_storey],
        )
    # endregion Step 1

    # region Step 2: Piece-to-storey mapping
    import re as _re

    def _piece_storey_entity(piece_name: str) -> typing.Any:
        m = _re.search(r"_f(\d+)_", piece_name or "")
        if m:
            floor = int(m.group(1))
            if floor in storey_by_number:
                return storey_by_number[floor]

    # endregion Step 2

    pieces = design.pieces or []
    connections = design.connections or []

    # region Step 3: Types with geometry
    ifc_types: dict[str, typing.Any] = {}
    for piece in pieces:
        if piece.type is None:
        tk = _type_key_from_id(piece.type)
        if tk in ifc_types:
        type_obj = types_dict.get(tk)
        if type_obj is None:
            "root.create_entity",
            ifc,
            ifc_class="IfcBuildingElementProxyType",
            name=ifc_type_name,
        )

        # Type-level geometry
        model = _find_matching_model(kit, type_obj, tags)
        if model is not None:
            files_list = kit.files_ or []
            file_obj = next((f for f in files_list if f.name == file_id or f.guid == file_id), None)
            if file_obj is not None and file_obj.blob:
                raw = base64.b64decode(blob.split(",", 1)[1] if blob.startswith("data:") else blob)
                result = _glb_bytes_to_vertices_faces(raw)
                if result is not None:
                        "geometry.add_mesh_representation",
                        ifc,
                        context=body_context,
                        vertices=[list(vertices)],
                        faces=[list(faces)],
                    )
                        "geometry.assign_representation",
                        ifc,
                        product=ifc_type,
                        representation=rep,
                    )

    # endregion Step 3

    # region Step 4: Pieces as occurrences
    ifc_occurrences: dict[str, typing.Any] = {}
    ifc_connector_ports: dict[str, dict[str, typing.Any]] = {}
    for piece in pieces:
            "root.create_entity",
            ifc,
            ifc_class="IfcBuildingElementProxy",
            name=piece_name,
        )

        if piece.type is not None:
            tk = _type_key_from_id(piece.type)
            if tk in ifc_types:
                    "type.assign_type",
                    ifc,
                    related_objects=[occurrence],
                    relating_type=ifc_types[tk],
                )

        world_plane = piece_planes.get(piece.id_)
        if world_plane is not None:
            mat = _plane_to_matrix_4x4(world_plane)
            _ifc_api.run("geometry.edit_object_placement", ifc, product=occurrence, matrix=mat)

        # Assign piece to the correct storey based on its floor number
            "spatial.assign_container",
            ifc,
            relating_structure=_piece_storey_entity(piece_name),
            products=[occurrence],
        )

        # Connectors as ports
        if type_obj is not None and type_obj.connectors:
            ifc_connector_ports[piece.id_] = {}
            for conn in type_obj.connectors:
                    "root.create_entity",
                    ifc,
                    ifc_class="IfcDistributionPort",
                    name=conn_id,
                )
                    "nest.assign_object",
                    ifc,
                    relating_object=occurrence,
                    related_objects=[port],
                )

                port_mat = numpy.eye(4)
                port_mat[:3, 3] = [point.x, point.y, point.z]
                d = numpy.array([direction.x, direction.y, direction.z])
                dn = numpy.linalg.norm(d)
                if dn > 1e-10:
                    up = numpy.array([0.0, 0.0, 1.0])
                    if abs(numpy.dot(z, up)) > 0.99:
                        up = numpy.array([1.0, 0.0, 0.0])
                    x = numpy.cross(up, z)
                    xn = numpy.linalg.norm(x)
                    if xn > 1e-10:
                    y = numpy.cross(z, x)
                _ifc_api.run("geometry.edit_object_placement", ifc, product=port, matrix=port_mat)

    # endregion Step 4

    # region Step 5: Connections as port relationships
    for conn in connections:

        if connected_id in ifc_connector_ports and connected_connector_id:
            connected_port = ifc_connector_ports[connected_id].get(connected_connector_id)
        if connecting_id in ifc_connector_ports and connecting_connector_id:
            connecting_port = ifc_connector_ports[connecting_id].get(connecting_connector_id)

        if connected_port is not None and connecting_port is not None:
                "IfcRelConnectsPorts",
                GlobalId=_ifc_guid.new(),
                RelatingPort=connected_port,
                RelatedPort=connecting_port,
            )

        connected_elem = ifc_occurrences.get(connected_id)
        connecting_elem = ifc_occurrences.get(connecting_id)
        if connected_elem is not None and connecting_elem is not None:
                "IfcRelConnectsElements",
                GlobalId=_ifc_guid.new(),
                RelatingElement=connected_elem,
                RelatedElement=connecting_elem,
            )

        conn_solver_props: dict[str, typing.Any] = {}
        for param in ("gap", "shift", "rise", "rotation", "turn", "tilt"):
            val = getattr(conn, param, 0)
            if val is not None and val != 0:
                conn_solver_props[param] = float(val)
        if conn.description:
        if conn_solver_props and connected_elem is not None:
                "pset.add_pset",
                ifc,
                product=connected_elem,
                name="SemioConnectionParams",
            )
            _ifc_api.run("pset.edit_pset", ifc, pset=conn_pset, properties=conn_solver_props)
    # endregion Step 5

    return ifc.to_string().encode("utf-8")


# endregion IFC Export

# endregion Kit Model Export

# region Geometric Insights
# [👤semio📚py💻semio🔖domain🔖geometricinsights](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Geometric%20Insights)
# Key performance indicators for GLB/GLTF model geometry. Model MUST be glb/gltf.


class GeometricInsights:
    """Aggregated geometric KPIs for a single mesh or merged scene.
    All geometric data is expressed in the semio coordinate system:
    semio.x = glb.x, semio.y = -glb.x, semio.z = glb.y.
    """

    # Overall size
    # Surface area
    # Volume
    # Compactness
    # Proportion
    # Mass distribution
    # Topology
    # Concavity


def get_geometric_insights_for_model(model: str | bytes) -> GeometricInsights:
    """Compute key performance indicators for the geometry of a GLB/GLTF model.
    Model MUST be glb or gltf (path or raw bytes). Uses trimesh for mesh analysis.
    """
    import trimesh as _trimesh

    if isinstance(model, bytes):
        if len(model) >= 4 and model[:4] == b"glTF":
        elif len(model) > 0 and model.lstrip().startswith(b"{"):
        stream = _trimesh.util.wrap_as_stream(model)
        loaded = _trimesh.load(stream, file_type=file_type)
    else:
        path = pathlib.Path(model)
        if not path.exists():
            raise FileNotFoundError(f"Model file not found: {model}")
        ext = path.suffix.lower()
        if ext not in (".glb", ".gltf"):
            raise ValueError(f"Model MUST be .glb or .gltf, got {ext}")
        loaded = _trimesh.load(str(path), file_type=file_type)

    if isinstance(loaded, _trimesh.Scene):
        meshes = [g.copy() for g in loaded.geometry.values() if isinstance(g, _trimesh.Trimesh) and len(getattr(g, "faces", [])) > 0]
        if not meshes:
            return GeometricInsights()
        mesh = _trimesh.util.concatenate(meshes)
    elif isinstance(loaded, _trimesh.Trimesh) and len(getattr(loaded, "faces", [])) > 0:
    else:
        return GeometricInsights()

    # Transform vertices from GLB to semio coordinate system.
    xs = verts[:, 0]
    ys = verts[:, 1]
    # semio: x = glb.x, y = -glb.x, z = glb.y

    xs_min, xs_max = float(semio_x.min()), float(semio_x.max())
    ys_min, ys_max = float(semio_y.min()), float(semio_y.max())
    zs_min, zs_max = float(semio_z.min()), float(semio_z.max())

    out = GeometricInsights()

    # Overall size in semio coordinates
    out.bounding_box_min = Point(x=xs_min, y=ys_min, z=zs_min)
    out.bounding_box_max = Point(x=xs_max, y=ys_max, z=zs_max)
    out.characteristic_length = float(numpy.cbrt(vol_box) if vol_box > 0 else 0.0)

    # Surface area and volume (topology and integrals are invariant under linear transform)
    out.total_surface_area = float(mesh.area)

    # Volume
    if mesh.is_watertight:
        out.enclosed_volume = float(mesh.volume)
    else:

    # Compactness
    if out.enclosed_volume is not None and out.enclosed_volume > 1e-20:
    if vol > 1e-20 and out.total_surface_area:
        out.sphericity = float((numpy.pi ** (1 / 3)) * (6 * vol) ** (2 / 3) / out.total_surface_area)
        out.sphericity = min(1.0, max(0.0, out.sphericity))

    try:
        if hull is not None and hull.volume > 1e-20 and vol > 0:
            out.convex_hull_volume = float(hull.volume)
            out.hull_fill_ratio = float(vol / hull.volume)
            out.hull_fill_ratio = min(1.0, max(0.0, out.hull_fill_ratio))
        elif hull is not None:
            out.convex_hull_volume = float(hull.volume)
    except Exception:

    # Proportion (semio dimensions)
    if dim_x > 1e-10 and dim_y > 1e-10:
        out.aspect_ratio_xy = float(dim_x / dim_y)
    if dim_x > 1e-10 and dim_z > 1e-10:
        out.aspect_ratio_xz = float(dim_x / dim_z)
    if dim_y > 1e-10 and dim_z > 1e-10:
        out.aspect_ratio_yz = float(dim_y / dim_z)
    max_ext = float(max(dim_x, dim_y, dim_z))
    if max_ext > 1e-10:

    # Mass distribution (trimesh uses density=1)
        float(mesh.centroid[0]),
        float(mesh.centroid[1]),
        float(mesh.centroid[2]),
    )
    # transform centroid as a point
    out.centroid = Point(x=cx_g, y=-cx_g, z=cy_g)
    try:
        if components is not None and vectors is not None:
                float(components[0]),
                float(components[1]),
                float(components[2]),
            )
            # Transform axes from GLB to semio: (vx, vy, vz)_glb -> (vx, -vx, vy)_semio
                    x=float(vectors[0][0]),
                    y=float(-vectors[0][0]),
                    z=float(vectors[0][1]),
                ),
                    x=float(vectors[1][0]),
                    y=float(-vectors[1][0]),
                    z=float(vectors[1][1]),
                ),
                    x=float(vectors[2][0]),
                    y=float(-vectors[2][0]),
                    z=float(vectors[2][1]),
                ),
            ]
    except Exception:

    # Topology
    out.vertex_count = int(len(mesh.vertices))
    out.face_count = int(len(mesh.faces))
    try:
        out.euler_characteristic = int(mesh.euler_number)
        if mesh.is_watertight:
    except Exception:
    out.is_watertight = bool(mesh.is_watertight)

    # Concavity
    if out.convex_hull_volume is not None and out.convex_hull_volume > 1e-20 and out.enclosed_volume is not None:
        out.concavity_index = 1.0 - (out.enclosed_volume / out.convex_hull_volume)
        out.concavity_index = min(1.0, max(0.0, out.concavity_index))



def geometric_insights_to_report_dict(insights: GeometricInsights, round_digits: int = 6) -> dict[str, typing.Any]:
    """Serialize GeometricInsights to a JSON-serializable dict for reports. Uses semio Point/Vector as {x,y,z}."""
    out: dict[str, typing.Any] = {}

    def round_val(v: float | None) -> float | None:

    if insights.bounding_box_min is not None:
            "x": round(p.x, r),
            "y": round(p.y, r),
            "z": round(p.z, r),
        }
    if insights.bounding_box_max is not None:
            "x": round(p.x, r),
            "y": round(p.y, r),
            "z": round(p.z, r),
        }
    if insights.centroid is not None:
        out["centroid"] = {"x": round(p.x, r), "y": round(p.y, r), "z": round(p.z, r)}
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
    if insights.principal_axes is not None:
        out["principal_axes"] = [{"x": round(v.x, r), "y": round(v.y, r), "z": round(v.z, r)} for v in insights.principal_axes]
    if insights.moments_of_inertia is not None:
        out["moments_of_inertia"] = [round(x, r) for x in insights.moments_of_inertia]
    for key in ("vertex_count", "face_count", "euler_characteristic", "genus"):
        val = getattr(insights, key, None)
        if val is not None:
    if insights.is_watertight is not None:


# endregion Geometric Insights

# region Spatial Math
# [👤semio📚py💻semio🔖domain🔖validation🔖spatialmath](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Spatial%20Math)
# Spatial math utilities for vector normalization and plane computation.


def normalizeVector(v: numpy.ndarray) -> numpy.ndarray:
    """Normalize a 3D vector to unit length.
    normalizeVector MUST return a unit-length vector or raise on zero length.
    """
    length = numpy.linalg.norm(v)
    if length < 1e-10:


def planeFromYAxis(yAxis: numpy.ndarray, phiDegrees: float = 0.0, origin: numpy.ndarray | None = None) -> Plane:
    """Construct a plane from an origin point and a Y-axis direction.
    planeFromYAxis MUST derive orthogonal x and z axes from the y axis.
    """
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
    xAxis = normalizeVector(xAxis)
    plane = Plane()
    plane.origin = Point(x=float(origin[0]), y=float(origin[1]), z=float(origin[2]))
    plane.xAxis = Vector(x=float(xAxis[0]), y=float(xAxis[1]), z=float(xAxis[2]))
    plane.yAxis = Vector(x=float(yAxis[0]), y=float(yAxis[1]), z=float(yAxis[2]))


def computeChildPlane(
    parentPlane: Plane,
    parentConnector: Connector,
    childConnector: Connector,
    connection: Connection,
) -> Plane:
    """Compute the world-space plane of a child from parent and local planes.
    computeChildPlane MUST compose parent and local plane transformations.
    """
    pOrigin = numpy.array([parentPlane.origin.x, parentPlane.origin.y, parentPlane.origin.z])
    pX = numpy.array([parentPlane.xAxis.x, parentPlane.xAxis.y, parentPlane.xAxis.z])
    pY = numpy.array([parentPlane.yAxis.x, parentPlane.yAxis.y, parentPlane.yAxis.z])
    pZ = numpy.cross(pX, pY)
    parentMatrix = numpy.eye(4)
    ppPoint = numpy.array([parentConnector.point.x, parentConnector.point.y, parentConnector.point.z])
            parentConnector.direction.x,
            parentConnector.direction.y,
            parentConnector.direction.z,
        ]
    )
    cpPoint = numpy.array([childConnector.point.x, childConnector.point.y, childConnector.point.z])
            childConnector.direction.x,
            childConnector.direction.y,
            childConnector.direction.z,
        ]
    )
    ppWorld = parentMatrix[:3, :3] @ ppPoint + parentMatrix[:3, 3]
    ppDirWorld = normalizeVector(ppDirWorld)
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
    childX = combinedRotation @ numpy.array([1, 0, 0])
    childY = combinedRotation @ numpy.array([0, 1, 0])
    plane = Plane()
    plane.origin = Point(x=float(childOrigin[0]), y=float(childOrigin[1]), z=float(childOrigin[2]))
    plane.xAxis = Vector(x=float(childX[0]), y=float(childX[1]), z=float(childX[2]))
    plane.yAxis = Vector(x=float(childY[0]), y=float(childY[1]), z=float(childY[2]))


# endregion Spatial Math


# region Meta And Shallow Types
# [👤semio📚py💻main🔖metaandshallowtypes](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types)
# Meta And Shallow Types MUST provide lightweight entity representations.

# region 🔖Sub-entity Meta Types

    "AttributeMeta",
    {"guid": str, "name": str, "value": str, "definition": typing.NotRequired[str]},
)
"""AttributeMeta is identical to Attribute (no list fields to omit)."""

    "TagMeta",
        "guid": str,
        "name": str,
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
        "order": typing.NotRequired[int],
    },
)
"""TagMeta is identical to Tag (no list fields to omit)."""

    "ConceptMeta",
        "guid": str,
        "name": str,
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
        "order": typing.NotRequired[int],
    },
)
"""ConceptMeta is identical to Concept (no list fields to omit)."""

    "StatMeta",
        "guid": str,
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

    "PropMeta",
    {"guid": str, "key": str, "value": str, "unit": typing.NotRequired[str]},
)
"""PropMeta is Prop without attributes."""

    "AuthorMeta",
    {"guid": str, "name": str, "email": typing.NotRequired[str]},
)
"""AuthorMeta is Author without attributes."""

    "FileMeta",
        "guid": str,
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

    "FolderMeta",
        "guid": str,
        "name": str,
        "parent": typing.NotRequired[dict],
        "description": typing.NotRequired[str],
        "createdAt": typing.NotRequired[str],
        "updatedAt": typing.NotRequired[str],
    },
)
"""FolderMeta is Folder without attributes."""

    "QualityMeta",
        "guid": str,
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

    "PortMeta",
        "guid": str,
        "name": str,
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
    },
)
"""PortMeta is Port without attributes."""

    "ModelMeta",
        "guid": str,
        "file": typing.NotRequired[dict],
        "name": typing.NotRequired[str],
        "description": typing.NotRequired[str],
    },
)
"""ModelMeta is Model without tags and attributes."""

    "ConnectorMeta",
        "guid": str,
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

    "LayerMeta",
        "guid": str,
        "name": str,
        "isHidden": typing.NotRequired[bool],
        "isLocked": typing.NotRequired[bool],
        "color": typing.NotRequired[str],
        "description": typing.NotRequired[str],
    },
)
"""LayerMeta is Layer without attributes."""

    "PieceMeta",
        "guid": str,
        "name": typing.NotRequired[str],
        "type": typing.NotRequired[dict],
        "designPiece": typing.NotRequired[dict],
        "plane": typing.NotRequired[dict],
        "center": typing.NotRequired[dict],
        "scale": typing.NotRequired[float],
        "mirrorPlane": typing.NotRequired[dict],
        "isHidden": typing.NotRequired[bool],
        "isLocked": typing.NotRequired[bool],
        "color": typing.NotRequired[str],
        "description": typing.NotRequired[str],
    },
)
"""PieceMeta is Piece without props and attributes."""

    "GroupMeta",
        "guid": str,
        "name": typing.NotRequired[str],
        "color": typing.NotRequired[str],
        "description": typing.NotRequired[str],
    },
)
"""GroupMeta is Group without pieces and attributes."""

    "ConnectionMeta",
        "guid": str,
        "connected": dict,
        "connecting": dict,
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

# endregion 🔖Sub-entity Meta Types

# region 🔖Main Entity Meta Types

    "TypeMeta",
        "guid": str,
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

    "DesignMeta",
        "guid": str,
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

    "KitMeta",
        "guid": str,
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

# endregion 🔖Main Entity Meta Types

# region 🔖Shallow Types

    "TypeShallow",
        "guid": str,
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
        "models": typing.NotRequired[list[ModelMeta]],
        "connectors": typing.NotRequired[list[ConnectorMeta]],
        "attributes": typing.NotRequired[list[AttributeMeta]],
    },
)
"""TypeShallow is Type with list fields replaced by Meta item lists."""

    "DesignShallow",
        "guid": str,
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

    "KitShallow",
        "guid": str,
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

# endregion 🔖Shallow Types

# region 🔖Meta And Shallow Conversion Functions


def _strip_none(d: dict) -> dict:
    """Remove keys with None values from a dict.
    _strip_none MUST remove keys with None values.
    """
    return {k: v for k, v in d.items() if v is not None}


def _extract_scalar_fields(d: dict, keys: list[str]) -> dict:
    """Extract only specified keys from a dict, skipping missing keys.
    _extract_scalar_fields MUST return only the specified scalar fields.
    """
    return {k: d[k] for k in keys if k in d}


_ATTRIBUTE_META_KEYS = ["guid", "name", "value", "definition"]
_TAG_META_KEYS = ["guid", "name", "description", "icon", "order"]
_CONCEPT_META_KEYS = ["guid", "name", "description", "icon", "order"]
    "guid",
    "key",
    "unit",
    "min",
    "minExcluded",
    "max",
    "maxExcluded",
    "createdAt",
    "updatedAt",
]
_PROP_META_KEYS = ["guid", "key", "value", "unit"]
_AUTHOR_META_KEYS = ["guid", "name", "email"]
    "guid",
    "name",
    "remote",
    "folder",
    "size",
    "hash",
    "createdAt",
    "updatedAt",
]
_FOLDER_META_KEYS = ["guid", "name", "parent", "description", "createdAt", "updatedAt"]
    "guid",
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
_PORT_META_KEYS = ["guid", "name", "description", "icon"]
_MODEL_META_KEYS = ["guid", "file", "name", "description"]
    "guid",
    "point",
    "direction",
    "t",
    "name",
    "description",
    "mandatory",
    "port",
]
_LAYER_META_KEYS = ["guid", "name", "isHidden", "isLocked", "color", "description"]
    "guid",
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
_GROUP_META_KEYS = ["guid", "name", "color", "description"]
    "guid",
    "connected",
    "connecting",
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

    "guid",
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
    "guid",
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
    "guid",
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
    """Convert an attribute dict to AttributeMeta.
    attributeToMeta MUST extract only AttributeMeta fields.
    """
    return _extract_scalar_fields(d, _ATTRIBUTE_META_KEYS)


def tagToMeta(d: dict) -> TagMeta:
    """Convert a tag dict to TagMeta.
    tagToMeta MUST extract only TagMeta fields.
    """
    return _extract_scalar_fields(d, _TAG_META_KEYS)


def conceptToMeta(d: dict) -> ConceptMeta:
    """Convert a concept dict to ConceptMeta.
    conceptToMeta MUST extract only ConceptMeta fields.
    """
    return _extract_scalar_fields(d, _CONCEPT_META_KEYS)


def statToMeta(d: dict) -> StatMeta:
    """Convert a stat dict to StatMeta.
    statToMeta MUST extract only StatMeta fields.
    """
    return _extract_scalar_fields(d, _STAT_META_KEYS)


def propToMeta(d: dict) -> PropMeta:
    """Convert a prop dict to PropMeta (without attributes).
    propToMeta MUST extract only PropMeta fields.
    """
    return _extract_scalar_fields(d, _PROP_META_KEYS)


def authorToMeta(d: dict) -> AuthorMeta:
    """Convert an author dict to AuthorMeta (without attributes).
    authorToMeta MUST extract only AuthorMeta fields.
    """
    return _extract_scalar_fields(d, _AUTHOR_META_KEYS)


def fileToMeta(d: dict) -> FileMeta:
    """Convert a file dict to FileMeta (without blob).
    fileToMeta MUST extract only FileMeta fields.
    """
    return _extract_scalar_fields(d, _FILE_META_KEYS)


def folderToMeta(d: dict) -> FolderMeta:
    """Convert a folder dict to FolderMeta (without attributes).
    folderToMeta MUST extract only FolderMeta fields.
    """
    return _extract_scalar_fields(d, _FOLDER_META_KEYS)


def qualityToMeta(d: dict) -> QualityMeta:
    """Convert a quality dict to QualityMeta (without benchmarks and attributes).
    qualityToMeta MUST extract only QualityMeta fields.
    """
    return _extract_scalar_fields(d, _QUALITY_META_KEYS)


def portToMeta(d: dict) -> PortMeta:
    """Convert a port dict to PortMeta (without attributes).
    portToMeta MUST extract only PortMeta fields.
    """
    return _extract_scalar_fields(d, _PORT_META_KEYS)


def modelToMeta(d: dict) -> ModelMeta:
    """Convert a model dict to ModelMeta (without tags and attributes).
    modelToMeta MUST extract only ModelMeta fields.
    """
    return _extract_scalar_fields(d, _MODEL_META_KEYS)


def connectorToMeta(d: dict) -> ConnectorMeta:
    """Convert a connector dict to ConnectorMeta (without props and attributes).
    connectorToMeta MUST extract only ConnectorMeta fields.
    """
    return _extract_scalar_fields(d, _CONNECTOR_META_KEYS)


def layerToMeta(d: dict) -> LayerMeta:
    """Convert a layer dict to LayerMeta (without attributes).
    layerToMeta MUST extract only LayerMeta fields.
    """
    return _extract_scalar_fields(d, _LAYER_META_KEYS)


def pieceToMeta(d: dict) -> PieceMeta:
    """Convert a piece dict to PieceMeta (without props and attributes).
    pieceToMeta MUST extract only PieceMeta fields.
    """
    return _extract_scalar_fields(d, _PIECE_META_KEYS)


def groupToMeta(d: dict) -> GroupMeta:
    """Convert a group dict to GroupMeta (without pieces and attributes).
    groupToMeta MUST extract only GroupMeta fields.
    """
    return _extract_scalar_fields(d, _GROUP_META_KEYS)


def connectionToMeta(d: dict) -> ConnectionMeta:
    """Convert a connection dict to ConnectionMeta (without attributes).
    connectionToMeta MUST extract only ConnectionMeta fields.
    """
    return _extract_scalar_fields(d, _CONNECTION_META_KEYS)


def typeToMeta(d: dict) -> TypeMeta:
    """Convert a type dict to TypeMeta (scalar fields only).
    typeToMeta MUST extract only TypeMeta scalar fields.
    """
    return _extract_scalar_fields(d, _TYPE_META_KEYS)


def designToMeta(d: dict) -> DesignMeta:
    """Convert a design dict to DesignMeta (scalar fields only).
    designToMeta MUST extract only DesignMeta scalar fields.
    """
    return _extract_scalar_fields(d, _DESIGN_META_KEYS)


def kitToMeta(d: dict) -> KitMeta:
    """Convert a kit dict to KitMeta (scalar fields only).
    kitToMeta MUST extract only KitMeta scalar fields.
    """
    return _extract_scalar_fields(d, _KIT_META_KEYS)


def _convert_list(items: list | None, converter: typing.Callable) -> list | None:
    """Convert a list of dicts using a converter function, returning None for empty/missing lists.
    _convert_list MUST return None for empty or missing lists.
    """
    if not items:
    return [converter(item) for item in items]


def typeToShallow(d: dict) -> TypeShallow:
    """Convert a type dict to TypeShallow (list fields replaced by Meta items).
    typeToShallow MUST convert list fields to Meta item lists.
    """
    result = _extract_scalar_fields(d, _TYPE_META_KEYS)
    concepts = _convert_list(d.get("concepts"), lambda c: c if isinstance(c, str) else conceptToMeta(c))
    if concepts is not None:
    authors = _convert_list(d.get("authors"), lambda a: a if isinstance(a, str) else authorToMeta(a))
    if authors is not None:
    props = _convert_list(d.get("props"), propToMeta)
    if props is not None:
    models = _convert_list(d.get("models"), modelToMeta)
    if models is not None:
    connectors = _convert_list(d.get("connectors"), connectorToMeta)
    if connectors is not None:
    attributes = _convert_list(d.get("attributes"), attributeToMeta)
    if attributes is not None:


def designToShallow(d: dict) -> DesignShallow:
    """Convert a design dict to DesignShallow (list fields replaced by Meta items).
    designToShallow MUST convert list fields to Meta item lists.
    """
    result = _extract_scalar_fields(d, _DESIGN_META_KEYS)
    concepts = _convert_list(d.get("concepts"), lambda c: c if isinstance(c, str) else conceptToMeta(c))
    if concepts is not None:
    authors = _convert_list(d.get("authors"), lambda a: a if isinstance(a, str) else authorToMeta(a))
    if authors is not None:
    props = _convert_list(d.get("props"), propToMeta)
    if props is not None:
    pieces = _convert_list(d.get("pieces"), pieceToMeta)
    if pieces is not None:
    connections = _convert_list(d.get("connections"), connectionToMeta)
    if connections is not None:
    layers = _convert_list(d.get("layers"), layerToMeta)
    if layers is not None:
    groups = _convert_list(d.get("groups"), groupToMeta)
    if groups is not None:
    stats = _convert_list(d.get("stats"), statToMeta)
    if stats is not None:
    attributes = _convert_list(d.get("attributes"), attributeToMeta)
    if attributes is not None:


def kitToShallow(d: dict) -> KitShallow:
    """Convert a kit dict to KitShallow (list fields replaced by Meta items).
    kitToShallow MUST convert list fields to Meta item lists.
    """
    result = _extract_scalar_fields(d, _KIT_META_KEYS)
    concepts = _convert_list(d.get("concepts"), lambda c: c if isinstance(c, str) else conceptToMeta(c))
    if concepts is not None:
    tags = _convert_list(d.get("tags"), tagToMeta)
    if tags is not None:
    types = _convert_list(d.get("types"), typeToMeta)
    if types is not None:
    designs = _convert_list(d.get("designs"), designToMeta)
    if designs is not None:
    ports = _convert_list(d.get("ports"), portToMeta)
    if ports is not None:
    qualities = _convert_list(d.get("qualities"), qualityToMeta)
    if qualities is not None:
    files = _convert_list(d.get("files"), fileToMeta)
    if files is not None:
    folders = _convert_list(d.get("folders"), folderToMeta)
    if folders is not None:
    authors = _convert_list(d.get("authors"), lambda a: a if isinstance(a, str) else authorToMeta(a))
    if authors is not None:
    attributes = _convert_list(d.get("attributes"), attributeToMeta)
    if attributes is not None:


# endregion 🔖Meta And Shallow Conversion Functions

# endregion Meta And Shallow Types


# region Hash
# [👤semio📚py💻main🔖hash](repo://p/u/semio/b/l/py/f/main.py/s/Hash)
# Merkle hash functions for all semio entities.


# region 🔖HashWriter
def _format_number_for_hash(n) -> str:
    """Format number to match JavaScript Number.toString() behavior.
    Integers (including floats with no fractional part) are formatted without decimal point.
    """
    if isinstance(n, int):
        return str(n)
    if isinstance(n, float) and n.is_integer():
        return str(int(n))
    return str(n)


def _ref_guid(ref) -> str:
    """Extract guid from a reference (dict with 'guid' key or plain string)."""
    if isinstance(ref, dict):
        return ref["guid"]


class HashWriter:
    """Feeds structured data into a SHA-256 hasher for deterministic hashing.
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

    def writeGuidList(self, guids: list[str]):
        sorted_guids = sorted(guids)
        self._parts.extend(struct.pack(">I", len(sorted_guids)))
        for g in sorted_guids:
            self.writeString(g)

    def digest(self) -> str:
        return hashlib.sha256(bytes(self._parts)).hexdigest()


# endregion 🔖HashWriter


# region 🔖Hash Value Types
def hash_coord(c: dict) -> str:
    """Computes SHA-256 hash of a Coord value."""
    w = HashWriter()
    w.writeString("Coord")
    w.writeString("u")
    w.writeNumber(c["u"])
    w.writeString("v")
    w.writeNumber(c["v"])
    return w.digest()


def hash_vec(v: dict) -> str:
    """Computes SHA-256 hash of a Vec value."""
    w = HashWriter()
    w.writeString("Vec")
    w.writeString("u")
    w.writeNumber(v["u"])
    w.writeString("v")
    w.writeNumber(v["v"])
    return w.digest()


def hash_point(p: dict) -> str:
    """Computes SHA-256 hash of a Point value."""
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
    """Computes SHA-256 hash of a Vector value."""
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
    """Computes SHA-256 hash of a Plane value."""
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
    """Computes SHA-256 hash of a Camera value."""
    w = HashWriter()
    w.writeString("Camera")
    w.writeString("forward")
    w.writeHash(hash_vector(c["forward"]))
    w.writeString("position")
    w.writeHash(hash_point(c["position"]))
    w.writeString("up")
    w.writeHash(hash_vector(c["up"]))
    return w.digest()


# endregion 🔖Hash Value Types


# region 🔖Hash Entities
def hash_attribute(a: dict) -> str:
    """Computes SHA-256 hash of an Attribute entity."""
    w = HashWriter()
    w.writeString("Attribute")
    if a.get("definition") is not None:
        w.writeString("definition")
        w.writeString(a["definition"])
    w.writeString("guid")
    w.writeString(a["guid"])
    w.writeString("key")
    w.writeString(a["key"])
    if a.get("value") is not None:
        w.writeString("value")
        w.writeString(a["value"])
    return w.digest()


def hash_location(l: dict) -> str:
    """Computes SHA-256 hash of a Location entity."""
    w = HashWriter()
    w.writeString("Location")
    if l.get("altitude") is not None:
        w.writeString("altitude")
        w.writeNumber(l["altitude"])
    attrs = l.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    w.writeString("guid")
    w.writeString(l["guid"])
    w.writeString("latitude")
    w.writeNumber(l["latitude"])
    w.writeString("longitude")
    w.writeNumber(l["longitude"])
    return w.digest()


def hash_author(a: dict) -> str:
    """Computes SHA-256 hash of an Author entity."""
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
    w.writeString("guid")
    w.writeString(a["guid"])
    w.writeString("name")
    w.writeString(a["name"])
    return w.digest()


def hash_file(f: dict) -> str:
    """Computes SHA-256 hash of a File entity."""
    w = HashWriter()
    w.writeString("File")
    if f.get("blob") is not None:
        w.writeString("blob")
        w.writeString(f["blob"])
    if f.get("folder") is not None:
        w.writeString("folder")
        w.writeString(_ref_guid(f["folder"]))
    w.writeString("guid")
    w.writeString(f["guid"])
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
    """Computes SHA-256 hash of a Folder entity."""
    w = HashWriter()
    w.writeString("Folder")
    attrs = f.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if f.get("description") is not None:
        w.writeString("description")
        w.writeString(f["description"])
    w.writeString("guid")
    w.writeString(f["guid"])
    w.writeString("name")
    w.writeString(f["name"])
    if f.get("parent") is not None:
        w.writeString("parent")
        w.writeString(_ref_guid(f["parent"]))
    return w.digest()


def hash_benchmark(b: dict) -> str:
    """Computes SHA-256 hash of a Benchmark entity."""
    w = HashWriter()
    w.writeString("Benchmark")
    attrs = b.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    w.writeString("guid")
    w.writeString(b["guid"])
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
    """Computes SHA-256 hash of a Quality entity."""
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
    w.writeString("guid")
    w.writeString(q["guid"])
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
    """Computes SHA-256 hash of a Port entity."""
    w = HashWriter()
    w.writeString("Port")
    attrs = p.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    compat = p.get("compatiblePorts")
    if compat and len(compat) > 0:
        w.writeString("compatiblePorts")
        w.writeGuidList([_ref_guid(cp) for cp in compat])
    if p.get("description") is not None:
        w.writeString("description")
        w.writeString(p["description"])
    w.writeString("guid")
    w.writeString(p["guid"])
    if p.get("icon") is not None:
        w.writeString("icon")
        w.writeString(p["icon"])
    w.writeString("name")
    w.writeString(p["name"])
    return w.digest()


def hash_prop(p: dict) -> str:
    """Computes SHA-256 hash of a Prop entity."""
    w = HashWriter()
    w.writeString("Prop")
    attrs = p.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    w.writeString("guid")
    w.writeString(p["guid"])
    w.writeString("quality")
    w.writeString(_ref_guid(p["quality"]))
    if p.get("unit") is not None:
        w.writeString("unit")
        w.writeString(p["unit"])
    w.writeString("value")
    w.writeString(p["value"])
    return w.digest()


def hash_tag(t: dict) -> str:
    """Computes SHA-256 hash of a Tag entity."""
    w = HashWriter()
    w.writeString("Tag")
    attrs = t.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if t.get("description") is not None:
        w.writeString("description")
        w.writeString(t["description"])
    w.writeString("guid")
    w.writeString(t["guid"])
    if t.get("icon") is not None:
        w.writeString("icon")
        w.writeString(t["icon"])
    w.writeString("name")
    w.writeString(t["name"])
    return w.digest()


def hash_concept(c: dict) -> str:
    """Computes SHA-256 hash of a Concept entity."""
    w = HashWriter()
    w.writeString("Concept")
    attrs = c.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if c.get("description") is not None:
        w.writeString("description")
        w.writeString(c["description"])
    w.writeString("guid")
    w.writeString(c["guid"])
    if c.get("icon") is not None:
        w.writeString("icon")
        w.writeString(c["icon"])
    w.writeString("name")
    w.writeString(c["name"])
    return w.digest()


def hash_model(m: dict) -> str:
    """Computes SHA-256 hash of a Model entity."""
    w = HashWriter()
    w.writeString("Model")
    attrs = m.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if m.get("description") is not None:
        w.writeString("description")
        w.writeString(m["description"])
    w.writeString("file")
    w.writeString(_ref_guid(m["file"]))
    w.writeString("guid")
    w.writeString(m["guid"])
    if m.get("name") is not None:
        w.writeString("name")
        w.writeString(m["name"])
    tags = m.get("tags")
    if tags and len(tags) > 0:
        w.writeString("tags")
        w.writeGuidList([_ref_guid(t) for t in tags])
    return w.digest()


def hash_connector(c: dict) -> str:
    """Computes SHA-256 hash of a Connector entity."""
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
    w.writeString("guid")
    w.writeString(c["guid"])
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
        w.writeString(_ref_guid(c["port"]))
    props = c.get("props")
    if props and len(props) > 0:
        w.writeString("props")
        w.writeHashList([hash_prop(p) for p in props])
    w.writeString("t")
    w.writeNumber(c["t"])
    return w.digest()


def hash_type(t: dict) -> str:
    """Computes SHA-256 hash of a Type entity."""
    w = HashWriter()
    w.writeString("Type")
    attrs = t.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    authors = t.get("authors")
    if authors and len(authors) > 0:
        w.writeString("authors")
        w.writeGuidList([_ref_guid(a) for a in authors])
    concepts = t.get("concepts")
    if concepts and len(concepts) > 0:
        w.writeString("concepts")
        w.writeGuidList([_ref_guid(c) for c in concepts])
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
    w.writeString("guid")
    w.writeString(t["guid"])
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
        w.writeString(_ref_guid(t["location"]))
    models = t.get("models")
    if models and len(models) > 0:
        w.writeString("models")
        w.writeHashList([hash_model(m) for m in models])
    w.writeString("name")
    w.writeString(t["name"])
    if t.get("parent") is not None:
        w.writeString("parent")
        w.writeString(_ref_guid(t["parent"]))
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
    """Computes SHA-256 hash of a Layer entity."""
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
    w.writeString("guid")
    w.writeString(l["guid"])
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
    """Computes SHA-256 hash of a Stat entity."""
    w = HashWriter()
    w.writeString("Stat")
    w.writeString("guid")
    w.writeString(s["guid"])
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
    w.writeString(_ref_guid(s["quality"]))
    if s.get("unit") is not None:
        w.writeString("unit")
        w.writeString(s["unit"])
    return w.digest()


def hash_group(g: dict) -> str:
    """Computes SHA-256 hash of a Group entity."""
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
    w.writeString("guid")
    w.writeString(g["guid"])
    if g.get("name") is not None:
        w.writeString("name")
        w.writeString(g["name"])
    w.writeString("pieces")
    w.writeGuidList([_ref_guid(p) for p in g["pieces"]])
    return w.digest()


def hash_side(s: dict) -> str:
    """Computes SHA-256 hash of a Side value."""
    w = HashWriter()
    w.writeString("Side")
    if s.get("connector") is not None:
        w.writeString("connector")
        w.writeString(_ref_guid(s["connector"]))
    if s.get("designPiece") is not None:
        w.writeString("designPiece")
        w.writeString(_ref_guid(s["designPiece"]))
    w.writeString("piece")
    w.writeString(_ref_guid(s["piece"]))
    return w.digest()


def hash_connection(c: dict) -> str:
    """Computes SHA-256 hash of a Connection entity."""
    w = HashWriter()
    w.writeString("Connection")
    attrs = c.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    w.writeString("connected")
    w.writeHash(hash_side(c["connected"]))
    w.writeString("connecting")
    w.writeHash(hash_side(c["connecting"]))
    if c.get("description") is not None:
        w.writeString("description")
        w.writeString(c["description"])
    if c.get("gap") is not None:
        w.writeString("gap")
        w.writeNumber(c["gap"])
    w.writeString("guid")
    w.writeString(c["guid"])
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
    """Computes SHA-256 hash of a Piece entity."""
    w = HashWriter()
    w.writeString("Piece")
    attrs = p.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    if p.get("center") is not None:
        w.writeString("center")
        w.writeHash(hash_coord(p["center"]))
    if p.get("color") is not None:
        w.writeString("color")
        w.writeString(p["color"])
    if p.get("description") is not None:
        w.writeString("description")
        w.writeString(p["description"])
    if p.get("design") is not None:
        w.writeString("design")
        w.writeString(_ref_guid(p["design"]))
    w.writeString("guid")
    w.writeString(p["guid"])
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
    if p.get("plane") is not None:
        w.writeString("plane")
        w.writeHash(hash_plane(p["plane"]))
    if p.get("props") is not None and len(p["props"]) > 0:
        w.writeString("props")
        w.writeHashList([hash_prop(pr) for pr in p["props"]])
    if p.get("scale") is not None:
        w.writeString("scale")
        w.writeNumber(p["scale"])
    if p.get("type") is not None:
        w.writeString("type")
        w.writeString(_ref_guid(p["type"]))
    return w.digest()


def hash_design(d: dict) -> str:
    """Computes SHA-256 hash of a Design entity (Merkle tree)."""
    w = HashWriter()
    w.writeString("Design")
    if d.get("activeLayer") is not None:
        w.writeString("activeLayer")
        w.writeString(_ref_guid(d["activeLayer"]))
    attrs = d.get("attributes")
    if attrs and len(attrs) > 0:
        w.writeString("attributes")
        w.writeHashList([hash_attribute(a) for a in attrs])
    authors = d.get("authors")
    if authors and len(authors) > 0:
        w.writeString("authors")
        w.writeGuidList([_ref_guid(a) for a in authors])
    if d.get("canMirror") is not None:
        w.writeString("canMirror")
        w.writeBool(d["canMirror"])
    if d.get("canScale") is not None:
        w.writeString("canScale")
        w.writeBool(d["canScale"])
    concepts = d.get("concepts")
    if concepts and len(concepts) > 0:
        w.writeString("concepts")
        w.writeGuidList([_ref_guid(c) for c in concepts])
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
    w.writeString("guid")
    w.writeString(d["guid"])
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
        w.writeString(_ref_guid(d["location"]))
    w.writeString("name")
    w.writeString(d["name"])
    if d.get("parent") is not None:
        w.writeString("parent")
        w.writeString(_ref_guid(d["parent"]))
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
    """Computes SHA-256 Merkle hash of a Kit entity."""
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
    w.writeString("guid")
    w.writeString(k["guid"])
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


# endregion 🔖Hash Entities

# region 🔖Hash Diffs
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
            w.writeString(_ref_guid(d[key]))
        else:
            w.writeBool(False)


def _write_diff_id_array(w: HashWriter, key: str, d: dict):
    if key in d:
        val = d[key]
        if val is not None and len(val) > 0:
            w.writeString(key)
            w.writeGuidList([_ref_guid(e) for e in val])
        elif val is not None:
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
        w.writeGuidList([_ref_guid(r) for r in removed])
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
                    uw.writeString(_ref_guid(u[entity_key_name]))
            update_hashes.append(uw.digest())
        w.writeHashList(update_hashes)
    return w.digest()


def hash_coord_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("CoordDiff")
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
    return _hash_collection_diff_generic("AuthorsDiff", "AuthorDiffUpdate", "author", hash_author, hash_author_diff, d)


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
    return _hash_collection_diff_generic("FilesDiff", "FileDiffUpdate", "file", hash_file, hash_file_diff, d)


def hash_folder_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("FolderDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "name", d)
    _write_diff_id(w, "parent", d)
    return w.digest()


def hash_folders_diff(d: dict) -> str:
    return _hash_collection_diff_generic("FoldersDiff", "FolderDiffUpdate", "folder", hash_folder, hash_folder_diff, d)


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
    return _hash_collection_diff_generic("PortsDiff", "PortDiffUpdate", "port", hash_port, hash_port_diff, d)


def hash_prop_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("PropDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_id(w, "quality", d)
    _write_diff_string(w, "unit", d)
    _write_diff_string(w, "value", d)
    return w.digest()


def hash_props_diff(d: dict) -> str:
    return _hash_collection_diff_generic("PropsDiff", "PropDiffUpdate", "prop", hash_prop, hash_prop_diff, d)


def hash_tag_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("TagDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "icon", d)
    _write_diff_string(w, "name", d)
    return w.digest()


def hash_tags_diff(d: dict) -> str:
    return _hash_collection_diff_generic("TagsDiff", "TagDiffUpdate", "tag", hash_tag, hash_tag_diff, d)


def hash_concept_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("ConceptDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "description", d)
    _write_diff_string(w, "icon", d)
    _write_diff_string(w, "name", d)
    return w.digest()


def hash_concepts_diff(d: dict) -> str:
        "ConceptsDiff",
        "ConceptDiffUpdate",
        "concept",
        hash_concept,
        hash_concept_diff,
        d,
    )


def hash_model_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("ModelDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_string(w, "description", d)
    _write_diff_id(w, "file", d)
    _write_diff_string(w, "name", d)
    _write_diff_id_array(w, "tags", d)
    return w.digest()


def hash_models_diff(d: dict) -> str:
    return _hash_collection_diff_generic("ModelsDiff", "ModelDiffUpdate", "model", hash_model, hash_model_diff, d)


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
    _write_diff_hash(w, "models", d, hash_models_diff)
    _write_diff_string(w, "name", d)
    _write_diff_id(w, "parent", d)
    _write_diff_hash(w, "props", d, hash_props_diff)
    _write_diff_number(w, "stock", d)
    _write_diff_string(w, "unit", d)
    _write_diff_bool(w, "virtual", d)
    return w.digest()


def hash_types_diff(d: dict) -> str:
    return _hash_collection_diff_generic("TypesDiff", "TypeDiffUpdate", "type", hash_type, hash_type_diff, d)


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
    return _hash_collection_diff_generic("LayersDiff", "LayerDiffUpdate", "layer", hash_layer, hash_layer_diff, d)


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
    return _hash_collection_diff_generic("GroupsDiff", "GroupDiffUpdate", "group", hash_group, hash_group_diff, d)


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
    return _hash_collection_diff_generic("StatsDiff", "StatDiffUpdate", "stat", hash_stat, hash_stat_diff, d)


def hash_connection_diff(d: dict) -> str:
    w = HashWriter()
    w.writeString("ConnectionDiff")
    _write_diff_hash(w, "attributes", d, hash_attributes_diff)
    _write_diff_hash(w, "connected", d, hash_side_diff)
    _write_diff_hash(w, "connecting", d, hash_side_diff)
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
    _write_diff_hash(w, "center", d, hash_coord)
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
    return _hash_collection_diff_generic("PiecesDiff", "PieceDiffUpdate", "piece", hash_piece, hash_piece_diff, d)


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
    return _hash_collection_diff_generic("DesignsDiff", "DesignDiffUpdate", "design", hash_design, hash_design_diff, d)


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


# endregion 🔖Hash Diffs

# endregion Hash


# region Test
# [👤semio📚py💻main🔖test](repo://p/u/semio/b/l/py/f/main.py/s/Test)
# Tests for the semio py module.



def _test_load_json(filename: str) -> dict:
    path = os.path.join(os.path.dirname(__file__), TEST_ASSETS_DIR, filename)
    if not os.path.exists(path):
        raise FileNotFoundError(f"Asset not found: {path}")
    with open(path, "r", encoding="utf-8") as f:
        return json.load(f)


def _test_load_kit(filename: str) -> dict:
    """Load and normalize kit JSON for Kit.parse (flattens parent/folder refs, etc.)."""
    data = _test_load_json(filename)
    if "guid" in data and "uri" not in data:
        data["uri"] = data["guid"]
        "types",
        "designs",
        "files",
        "folders",
        "authors",
        "concepts",
        "models",
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
                if "parent" in item and isinstance(item["parent"], dict) and "guid" in item["parent"]:
                    item["parent"] = item["parent"]["guid"]
                if "folder" in item and isinstance(item["folder"], dict) and "guid" in item["folder"]:
                    item["folder"] = item["folder"]["guid"]
    if "types" in data:
        for t in data["types"]:
            if "models" in t:
                for m in t["models"]:
                    if "file" in m and isinstance(m["file"], dict) and "guid" in m["file"]:
                        m["file"] = m["file"]["guid"]
                    if "file" not in m or m["file"] is None:
                    if "url" not in m or m["url"] is None:
                    if "tags" in m and isinstance(m["tags"], list):
                        new_tags = [(tag["guid"] if isinstance(tag, dict) and "guid" in tag else tag) for tag in m["tags"]]
                    elif "tags" not in m:
                        m["tags"] = []


def _test_build_workflow_kit() -> dict:
    """Build a compact kit fixture for workflow roundtrip tests."""
    asset_blob = "data:text/plain;base64," + base64.b64encode(b"workflow asset payload").decode("ascii")
        "guid": "11111111-1111-1111-1111-111111111111",
        "name": "Workflow Kit",
        "version": "1.0.0",
        "description": "Kit workflow fixture.",
                "guid": "22222222-2222-2222-2222-222222222222",
                "name": "Workflow Type",
                "connectors": [],
                        "guid": "33333333-3333-3333-3333-333333333333",
                        "name": "Workflow Model",
                        "file": {"guid": "44444444-4444-4444-4444-444444444444"},
                    }
                ],
            }
        ],
                "guid": "55555555-5555-5555-5555-555555555555",
                "name": "Workflow Design",
                        "guid": "66666666-6666-6666-6666-666666666666",
                        "id": "Piece-1",
                        "type": {"guid": "22222222-2222-2222-2222-222222222222"},
                    }
                ],
                "connections": [],
            }
        ],
                "guid": "44444444-4444-4444-4444-444444444444",
                "name": "asset.txt",
                "folder": {"guid": "77777777-7777-7777-7777-777777777777"},
                "blob": asset_blob,
            }
        ],
                "guid": "77777777-7777-7777-7777-777777777777",
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
    """Build a compact diff for workflow edit tests."""
        "name": updated_name,
                    "file": {"guid": "44444444-4444-4444-4444-444444444444"},
                    "diff": {"name": updated_asset_name},
                }
            ]
        },
    }


def _test_build_workflow_archive_bytes(kit_dict: dict, files: dict[str, bytes]) -> bytes:
    """Build archive bytes for remote ZIP workflow tests."""
    with tempfile.TemporaryDirectory() as tmpdir:
        archive_path = os.path.join(tmpdir, "workflow.zip")
        export_kit(KitData(kit_dict), files, archive_path)
        with open(archive_path, "rb") as handle:
            return handle.read()


def _test_remote_kit_server(json_body: bytes, zip_body: bytes):
    """Create a disposable HTTP server for remote kit workflow tests."""
    import http.server
    import threading

    class _WorkflowHandler(http.server.BaseHTTPRequestHandler):
            "/workflow.json": {"content_type": "application/json", "body": json_body},
            "/workflow.zip": {"content_type": "application/zip", "body": zip_body},
        }

        def do_GET(self):
            item = self.store.get(self.path)
            if item is None:
                self.send_error(404)
            self.send_response(200)
            self.send_header("Content-Type", item["content_type"])
            self.send_header("Content-Length", str(len(item["body"])))
            self.end_headers()
            self.wfile.write(item["body"])

        def do_PUT(self):
            item = self.store.get(self.path)
            if item is None:
                self.send_error(404)
            length = int(self.headers.get("Content-Length", "0"))
            item["body"] = self.rfile.read(length)
            item["content_type"] = self.headers.get("Content-Type", item["content_type"])
            self.send_response(204)
            self.end_headers()

        def log_message(self, format: str, *args: typing.Any) -> None:

    server = http.server.ThreadingHTTPServer(("127.0.0.1", 0), _WorkflowHandler)
    thread = threading.Thread(target=server.serve_forever, daemon=True)
    thread.start()


def _test_is_close(a, b):


def _test_vectors_equal(v1, v2):
    if v1 is None or v2 is None:
    return _test_is_close(v1.get("x", 0), v2.get("x", 0)) and _test_is_close(v1.get("y", 0), v2.get("y", 0)) and _test_is_close(v1.get("z", 0), v2.get("z", 0))


def _test_planes_equal(p1, p2):
    if p1 is None or p2 is None:
    if not p1.get("origin") or not p2.get("origin"):
    if not p1.get("xAxis") or not p2.get("xAxis"):
    if not p1.get("yAxis") or not p2.get("yAxis"):
    return _test_vectors_equal(p1.get("origin"), p2.get("origin")) and _test_vectors_equal(p1.get("xAxis"), p2.get("xAxis")) and _test_vectors_equal(p1.get("yAxis"), p2.get("yAxis"))


def _test_centers_equal(c1, c2):
    if c1 is None or c2 is None:
    return _test_is_close(c1.get("u", 0), c2.get("u", 0)) and _test_is_close(c1.get("v", 0), c2.get("v", 0))


def _test_find_design(kit: dict, name: str, parent_name: str = None) -> dict:
    if parent_name:
        for d in kit.get("designs", []):
            if d.get("name") == parent_name:
                parent_guid = d.get("guid")
        if not parent_guid:
            raise ValueError(f"Parent {parent_name} not found")

    for d in kit.get("designs", []):
        if d.get("name") == name:
            p = d.get("parent")
            if parent_guid:
                if p and p.get("guid") == parent_guid:
            else:
                if not p:
    raise ValueError(f"Design {name} not found")


def _test_flatten(design_name, parent_name=None):
    kit_dict = _test_load_json("metabolism.kit.semio.json")
    design = _test_find_design(kit_dict, design_name, parent_name)

        (d for d in kit_dict.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")),
        None,
    )

    flat_design_diff = flattenDesignDict(kit_dict, design.get("guid"))
    flat_design = _applyDesignDiff(design, flat_design_diff)

    for piece in flat_design.get("pieces", []):
            (x for x in expected_design.get("pieces", []) if x.get("name") == piece.get("name")),
            None,
        )
        assert _test_planes_equal(piece.get("plane"), expected_piece.get("plane"))
        assert _test_centers_equal(piece.get("center"), expected_piece.get("center"))


def _test_contains_all_tags(model: dict[str, typing.Any], selected_tag_guids: list[str]) -> bool:
    model_tag_guids = [t.get("guid") if isinstance(t, dict) else t for t in model.get("tags", [])]
    return all(guid in model_tag_guids for guid in selected_tag_guids)


def _test_jaccard_tag_guids(model_tag_guids: list[str], selected_tag_guids: list[str]) -> float:
    if len(model_tag_guids) == 0 and len(selected_tag_guids) == 0:
    set_a = set(model_tag_guids)
    set_b = set(selected_tag_guids)
    if len(union) == 0:
    return len(set_a & set_b) / len(union)


def _test_select_best_model_like_semio_ts(models: list[dict[str, typing.Any]], selected_tag_guids: list[str]) -> dict[str, typing.Any] | None:
    if len(models) == 0:
    if len(selected_tag_guids) == 0:
        default_model = next((model for model in models if len(model.get("tags", [])) == 0), None)
        return default_model if default_model is not None else models[0]
    filtered_models = [model for model in models if _test_contains_all_tags(model, selected_tag_guids)]
    if len(filtered_models) == 0:
            [t.get("guid") if isinstance(t, dict) else t for t in model.get("tags", [])],
            selected_tag_guids,
        )
    ]
    max_score = max(indexed_scores)
    max_score_index = indexed_scores.index(max_score)
    return filtered_models[max_score_index]


def _test_create_glb_blob(vertices: list[tuple[float, float, float]], faces: list[tuple[int, int, int]]) -> str:
    def _pad4(data: bytes, fill: bytes) -> bytes:

        "<" + "f" * (len(vertices) * 3),
        *(value for vertex in vertices for value in vertex),
    )
    index_values = [index for face in faces for index in face]
    index_bytes = struct.pack("<" + "H" * len(index_values), *index_values)
    position_bytes = _pad4(position_bytes, b"\x00")
    index_bytes = _pad4(index_bytes, b"\x00")
    position_length = len(position_bytes)
    index_length = len(index_bytes)
    min_x = min(vertex[0] for vertex in vertices)
    min_y = min(vertex[1] for vertex in vertices)
    min_z = min(vertex[2] for vertex in vertices)
    max_x = max(vertex[0] for vertex in vertices)
    max_y = max(vertex[1] for vertex in vertices)
    max_z = max(vertex[2] for vertex in vertices)
            "asset": {"version": "2.0"},
            "buffers": [{"byteLength": len(binary_chunk)}],
                    "buffer": 0,
                    "byteOffset": 0,
                    "byteLength": position_length,
                    "target": 34962,
                },
                    "buffer": 0,
                    "byteOffset": position_length,
                    "byteLength": index_length,
                    "target": 34963,
                },
            ],
                    "bufferView": 0,
                    "componentType": 5126,
                    "count": len(vertices),
                    "type": "VEC3",
                    "min": [min_x, min_y, min_z],
                    "max": [max_x, max_y, max_z],
                },
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
            struct.pack("<4sII", b"glTF", 2, total_length),
            struct.pack("<I4s", len(json_chunk), b"JSON"),
            json_chunk,
            struct.pack("<I4s", len(binary_chunk), b"BIN\x00"),
            binary_chunk,
        ]
    )
    return "data:model/gltf-binary;base64," + base64.b64encode(glb).decode("ascii")


class TestRoundtrip:
    class TestMetabolism:
        def test_roundtrip(self):
            kit_dict = _test_load_json("metabolism.kit.semio.json")
            serialized = json.dumps(kit_dict)
            deserialized = json.loads(serialized)

            files: dict[str, bytes] = {}
            for file_entry in kit_dict.get("files", []):
                blob = file_entry.get("blob")
                if blob:
                    decoded = base64.b64decode(b64)
                    file_path = _build_file_path(kit_dict, file_entry)

            with tempfile.TemporaryDirectory() as tmpdir:
                roundtrip_path = os.path.join(tmpdir, "metabolism_roundtrip.zip")
                export_kit(KitData(kit_dict), files, roundtrip_path)

                kit2, files2 = import_kit(roundtrip_path)


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


        def test_folder_kit_import_export_edit_roundtrip(self):
            kit_dict = _test_build_workflow_kit()
            files = _collect_kit_asset_files(kit_dict)
            diff = _test_build_workflow_diff("Workflow Folder Edited", "asset-folder.txt")

            with tempfile.TemporaryDirectory() as tmpdir:
                export_folder_kit(KitData(kit_dict), files, tmpdir)
                assert os.path.exists(os.path.join(tmpdir, KIT_LOCAL_SUFFIX))

                imported, imported_files = import_folder_kit(tmpdir)
                assert areKitsDictEqual(kit_dict, imported.to_dict())

                edited = edit_folder_kit(tmpdir, diff)
                roundtrip, roundtrip_files = import_folder_kit(tmpdir)

                assert not os.path.exists(os.path.join(tmpdir, "assets", "asset.txt"))
                assert os.path.exists(os.path.join(tmpdir, "assets", "asset-folder.txt"))

            assert list(roundtrip_files.keys()) == ["assets/asset-folder.txt"]

        def test_archive_kit_import_export_edit_roundtrip(self):
            kit_dict = _test_build_workflow_kit()
            files = _collect_kit_asset_files(kit_dict)
            diff = _test_build_workflow_diff("Workflow Archive Edited", "asset-archive.txt")

            with tempfile.TemporaryDirectory() as tmpdir:
                archive_path = os.path.join(tmpdir, "workflow.zip")
                export_kit(KitData(kit_dict), files, archive_path)

                imported, imported_files = import_kit(archive_path)
                assert areKitsDictEqual(kit_dict, imported.to_dict())

                edited = edit_archive_kit(archive_path, diff)
                roundtrip, roundtrip_files = import_kit(archive_path)

            assert list(roundtrip_files.keys()) == ["assets/asset-archive.txt"]

        def test_remote_kit_import_json_and_zip_then_edit(self):
            kit_dict = _test_build_workflow_kit()
            files = _collect_kit_asset_files(kit_dict)
            json_body = json.dumps(kit_dict, ensure_ascii=False).encode("utf-8")
            zip_body = _test_build_workflow_archive_bytes(kit_dict, files)
            server, thread = _test_remote_kit_server(json_body, zip_body)

            try:

                imported_json, imported_json_files = import_remote_kit(json_uri)
                assert areKitsDictEqual(kit_dict, imported_json.to_dict())

                imported_zip, imported_zip_files = import_remote_kit(zip_uri)
                assert areKitsDictEqual(kit_dict, imported_zip.to_dict())

                    json_uri,
                    _test_build_workflow_diff("Workflow Remote Json Edited", "asset-remote-json.txt"),
                )
                    zip_uri,
                    _test_build_workflow_diff("Workflow Remote Zip Edited", "asset-remote-zip.txt"),
                )

                roundtrip_json, json_files = import_remote_kit(json_uri)
                roundtrip_zip, zip_files = import_remote_kit(zip_uri)
            finally:
                server.shutdown()
                thread.join()

            assert list(json_files.keys()) == ["assets/asset-remote-json.txt"]

            assert list(zip_files.keys()) == ["assets/asset-remote-zip.txt"]

        def test_temporary_kit_edit_via_diff(self):
            kit_dict = _test_build_workflow_kit()
                KitData(kit_dict),
                _test_build_workflow_diff("Workflow Temp Edited", "asset-temp.txt"),
            )



class TestFlatten:
    class TestNakaginCapsuleTower:
        def test_kit_flatten_diff_apply_flat(self):
            _test_flatten("Nakagin Capsule Tower")

        class TestSlanted:
            def test_kit_flatten_diff_apply_flat(self):
                _test_flatten("Slanted", "Nakagin Capsule Tower")

        class TestTwisted:
            def test_kit_flatten_diff_apply_flat(self):
                _test_flatten("Twisted", "Nakagin Capsule Tower")

        class TestDancing:
            def test_kit_flatten_diff_apply_flat(self):
                _test_flatten("Dancing", "Nakagin Capsule Tower")

    class TestCapsuleDream:
        def test_kit_flatten_diff_apply_flat(self):
            _test_flatten("Capsule Dream")


class TestChange:
    class TestMetabolism:
        def test_kit_change_forward_backward_inverse_behavior(self):
            kit_original = _test_load_json("metabolism.kit.semio.json")
            kit_original["designs"] = [d for d in kit_original.get("designs", []) if not d.get("parent")]
            kit_diff = _test_load_json("metabolism.kit.diff.semio.json")
            kit_diff_inverted = _test_load_json("metabolism.kit.diff.inverted.semio.json")
            kit_diffed = _test_load_json("metabolism.kit.diffed.semio.json")

            change = getKitChange(kit_original, kit_diffed)
            computed_diff = getKitDiffDict(kit_original, kit_diffed)
            assert areKitDiffsDictEqual(computed_diff, kit_diff)
            computed_inverse_diff = inverseKitDiffDict(kit_original, change.forward)
            assert areKitDiffsDictEqual(computed_inverse_diff, kit_diff_inverted)
            assert areKitDiffsDictEqual(change.forward, kit_diff)
            assert areKitDiffsDictEqual(change.backward, kit_diff_inverted)
            applied_forward = applyKitDiffDict(kit_original, change.forward)
            assert areKitsDictEqual(applied_forward, kit_diffed)
            applied_inverse = applyKitDiffDict(kit_diffed, change.backward)
            assert areKitsDictEqual(applied_inverse, kit_original)


class TestDelete:
    class TestNakaginCapsuleTower:
        def test_delete_third_tambour_and_first_small_tower_connection(self):
            kit = _test_load_json("metabolism.kit.semio.json")
            design = next(d for d in kit.get("designs", []) if d.get("name") == "Nakagin Capsule Tower")
            selection = _test_load_json("nakagin-capsule-tower.deleted.selection.semio.json")
            expected_diff = _test_load_json("nakagin-capsule-tower.deleted.design.diff.semio.json")

            piece_guids = [p["guid"] for p in selection.get("pieces", [])]
            connection_guids = [c["guid"] for c in selection.get("connections", [])]

            computed_diff = deletePiecesAndConnectionsInDesignDict(kit, design, piece_guids, connection_guids)

            # Verify removed pieces
            computed_removed = computed_diff.get("pieces", {}).get("removed", [])
            expected_removed = expected_diff.get("pieces", {}).get("removed", [])
            for c, e in zip(computed_removed, expected_removed):

            # Verify updated (fixed) pieces
            computed_updated = computed_diff.get("pieces", {}).get("updated", [])
            expected_updated = expected_diff.get("pieces", {}).get("updated", [])
            computed_guids = sorted(u.get("piece", {}).get("guid", "") for u in computed_updated)
            expected_guids = sorted(u.get("piece", {}).get("guid", "") for u in expected_updated)
            computed_sorted = sorted(computed_updated, key=lambda u: u.get("piece", {}).get("guid", ""))
            expected_sorted = sorted(expected_updated, key=lambda u: u.get("piece", {}).get("guid", ""))
            for cu, eu in zip(computed_sorted, expected_sorted):
                cd = cu["diff"]
                ed = eu["diff"]

            # Verify removed connections
            computed_conn_removed = computed_diff.get("connections", {}).get("removed", [])
            expected_conn_removed = expected_diff.get("connections", {}).get("removed", [])
            computed_conn_guids = sorted(r["guid"] for r in computed_conn_removed)
            expected_conn_guids = sorted(r["guid"] for r in expected_conn_removed)


class TestDesignWithDiff:
    class TestNakaginCapsuleTower:
        def test_design_with_diff_preserves_old_entities_and_annotates_status(self):
            kit = _test_load_json("metabolism.kit.semio.json")
            design = next(d for d in kit.get("designs", []) if d.get("name") == "Nakagin Capsule Tower" and not d.get("parent"))
            diff = _test_load_json("nakgin-capsule-tower.diff.design.semio.json")
            expected = _test_load_json("nakagin-capsule-tower.with-diff.design.semio.json")

            computed = designWithDiffDict(design, diff)


            def get_status(attrs):
                for a in attrs or []:
                    if a.get("key") == "semio.diffStatus":
                        return a.get("value")

            piece_statuses = [get_status(p.get("attributes")) for p in computed.get("pieces", [])]

            conn_statuses = [get_status(c.get("attributes")) for c in computed.get("connections", [])]


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
            expected = parseValidationResult(json.dumps(_test_load_json("validation.semio.json")))
            assert areValidationResultsEqual(result, expected)


class TestDesignModel:
    def test_model_selection_from_shared_semio_assets(self):
        payload = _test_load_json("model.selection.semio.json")
        for case in payload.get("cases", []):
                    "guid": model["guid"],
                    "file": {"guid": model["fileGuid"]},
                    "tags": [{"guid": guid} for guid in model.get("tagGuids", [])],
                }
                for model in case.get("models", [])
            ]
            selected = _test_select_best_model_like_semio_ts(models, case.get("selectedTagGuids", []))


class TestKitFilterDesign:
    def test_nakagin_capsule_tower_filter_produces_expected_subset(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        expected = _test_load_json("nakagin-capsule-tower.filtered.kit.semio.json")
        design = _test_find_design(kit_dict, "Nakagin Capsule Tower")

        filtered = KitData(kit_dict).filter_kit({"design_guid": design["guid"]}).to_dict()

        assert len(filtered.get("designs", [])) == len(expected.get("designs", []))
        assert len(filtered.get("types", [])) == len(expected.get("types", []))
        assert len(filtered.get("files", [])) == len(expected.get("files", []))
        assert len(filtered.get("ports", [])) == len(expected.get("ports", []))
        assert len(filtered.get("qualities", [])) == len(expected.get("qualities", []))
        assert len(filtered.get("authors", [])) == len(expected.get("authors", []))

        filtered_design = next(d for d in filtered.get("designs", []) if d.get("guid") == design["guid"])
        assert len(filtered_design.get("pieces", [])) == len(design.get("pieces", []))

        for expected_type in expected.get("types", []):
                (t for t in filtered.get("types", []) if t.get("guid") == expected_type.get("guid")),
                None,
            )
            assert len(filtered_type.get("models", [])) == len(expected_type.get("models", []))

        for piece in filtered_design.get("pieces", []):
            piece_kind_guid = piece.get("type", {}).get("guid")
            if piece_kind_guid:
                assert any(t.get("guid") == piece_kind_guid for t in filtered.get("types", []))

        for kind in filtered.get("types", []):
            for model in kind.get("models", []):
                assert any(file.get("guid") == model.get("file", {}).get("guid") for file in filtered.get("files", []))
            for connector in kind.get("connectors", []):
                connector_guid = connector.get("port", {}).get("guid")
                if connector_guid:
                    assert any(port.get("guid") == connector_guid for port in filtered.get("ports", []))

    def test_nakagin_capsule_tower_filter_preserves_metadata(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        design = _test_find_design(kit_dict, "Nakagin Capsule Tower")

        filtered = KitData(kit_dict).filter_kit({"design_guid": design["guid"]}).to_dict()

        assert filtered.get("guid") == kit_dict.get("guid")
        assert filtered.get("name") == kit_dict.get("name")
        assert filtered.get("version") == kit_dict.get("version")

    def test_glob_filters_types_by_name_include(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        filtered = KitData(kit_dict).filter_kit({"types": {"include": ["Capsule*"]}}).to_dict()
        types = filtered.get("types", [])
        for t in types:
            assert fnmatch.fnmatch(t["name"].lower(), "capsule*")

    def test_glob_filters_types_by_name_exclude(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        total_types = len(kit_dict.get("types", []))
        filtered = KitData(kit_dict).filter_kit({"types": {"exclude": ["Capsule*"]}}).to_dict()
        types = filtered.get("types", [])
        for t in types:
            assert not fnmatch.fnmatch(t["name"].lower(), "capsule*")

    def test_empty_filter_returns_kit_unchanged(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        filtered = KitData(kit_dict).filter_kit({}).to_dict()
        assert len(filtered.get("types", [])) == len(kit_dict.get("types", []))
        assert len(filtered.get("designs", [])) == len(kit_dict.get("designs", []))

    def test_combines_design_guid_with_glob_filters(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        design = _test_find_design(kit_dict, "Nakagin Capsule Tower")
        design_filtered = KitData(kit_dict).filter_kit({"design_guid": design["guid"]}).to_dict()
        combined_filtered = KitData(kit_dict).filter_kit({"design_guid": design["guid"], "types": {"exclude": ["Capsule*"]}}).to_dict()
        assert len(combined_filtered.get("types", [])) < len(design_filtered.get("types", []))
        for t in combined_filtered.get("types", []):
            assert not fnmatch.fnmatch(t["name"].lower(), "capsule*")


class TestDesignQualitySum:
    class TestNakaginCapsuleTower:
        def test_sum_effective_floor_area(self):
            kit_dict = _test_load_json("metabolism.kit.semio.json")
            design = _test_find_design(kit_dict, "Nakagin Capsule Tower")
            quality = next(q for q in kit_dict.get("qualities", []) if q.get("name") == "effective floor area")
            result = sumQualityInDesignDict(kit_dict, design["guid"], quality["guid"])


class TestExportDesignModel:
    def test_export_glb_returns_valid_glb(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".glb")
        assert isinstance(result, bytes)
        assert struct.unpack("<I", result[8:12])[0] == len(result)

    def test_export_gltf_returns_valid_json(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".gltf")
        assert isinstance(result, bytes)
        parsed = json.loads(result.decode("utf-8"))

    def test_export_invalid_format_raises(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        with pytest.raises(ValueError, match="Unsupported export format"):
            export_design_model(kit_dict, "Nakagin Capsule Tower", ".invalid")

    def test_export_scene_graph_report(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".gltf")
        parsed = json.loads(result.decode("utf-8"))
        REPORTS_EXPORT_DIR.mkdir(parents=True, exist_ok=True)
        (REPORTS_EXPORT_DIR / "py.gltf").write_bytes(result)

    def test_export_ifc_returns_valid_ifc(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        assert isinstance(result, bytes)
        ifc_text = result.decode("utf-8")

    def test_export_ifc_contains_types_and_occurrences(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc_text = result.decode("utf-8")

    def test_export_ifc_contains_mesh_geometry(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc_text = result.decode("utf-8")

    def test_export_ifc_converts_gltf_mesh_axes_to_semio_axes(self):
        import ifcopenshell

            "name": "Axis Test Kit",
            "guid": "axis-test-kit",
            "uri": "axis-test-kit",
                    "guid": "axis-test-kind",
                    "name": "Axis Test Kind",
                    "variant": "",
                    "attributes": [],
                    "connectors": [],
                            "guid": "axis-test-model",
                            "file": {"guid": "axis-test-file"},
                            "tags": [],
                        }
                    ],
                }
            ],
                    "guid": "axis-test-design",
                    "name": "Axis Test Design",
                            "guid": "axis-test-piece",
                            "name": "Axis Test Piece",
                            "type": {"guid": "axis-test-kind"},
                        }
                    ],
                    "connections": [],
                }
            ],
                    "guid": "axis-test-file",
                    "name": "axis-test.glb",
                        [(0.0, 0.0, 0.0), (0.0, 1.0, 0.0), (0.0, 0.0, 1.0)],
                        [(0, 1, 2)],
                    ),
                }
            ],
            "tags": [],
            "authors": [],
        }

        result = export_design_model(kit_dict, "Axis Test Design", ".ifc")
        ifc = ifcopenshell.file.from_string(result.decode("utf-8"))
        point_lists = ifc.by_type("IfcCartesianPointList3D")

        coordinates = [tuple(float(value) for value in row) for row in point_lists[0].CoordList]
        assert any(abs(x) < 1e-6 and abs(y) < 1e-6 and z > 0 for x, y, z in coordinates)
        assert any(abs(x) < 1e-6 and y < 0 and abs(z) < 1e-6 for x, y, z in coordinates)
        assert not any(abs(x) < 1e-6 and y > 0 and abs(z) < 1e-6 for x, y, z in coordinates)

    def test_export_ifc_contains_ports_and_connections(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc_text = result.decode("utf-8")

    def test_export_ifc_roundtrip_with_ifcopenshell(self):
        import ifcopenshell

        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc = ifcopenshell.file.from_string(result.decode("utf-8"))
        projects = ifc.by_type("IfcProject")
        sites = ifc.by_type("IfcSite")
        buildings = ifc.by_type("IfcBuilding")
        storeys = ifc.by_type("IfcBuildingStorey")
        storey_names = sorted([s.Name for s in storeys])
        assert storey_names == sorted([str(i) for i in range(11)])
        type_products = ifc.by_type("IfcBuildingElementProxyType")
        occurrences = ifc.by_type("IfcBuildingElementProxy")
        pieces = next(d for d in kit_dict.get("designs", []) if d.get("name") == "Nakagin Capsule Tower").get("pieces", [])
        assert len(occurrences) == len(pieces)
        ports = ifc.by_type("IfcDistributionPort")
        port_connections = ifc.by_type("IfcRelConnectsPorts")
        connections = next(d for d in kit_dict.get("designs", []) if d.get("name") == "Nakagin Capsule Tower").get("connections", [])
        assert len(port_connections) == len(connections)

    def test_export_ifc_layer_spatial_hierarchy(self):
        import ifcopenshell

        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc = ifcopenshell.file.from_string(result.decode("utf-8"))
        # IfcProject -> IfcSite -> IfcBuilding -> IfcBuildingStorey
        project = ifc.by_type("IfcProject")[0]
        site = ifc.by_type("IfcSite")[0]
        building = ifc.by_type("IfcBuilding")[0]
        storeys = ifc.by_type("IfcBuildingStorey")
        # Verify aggregation hierarchy
        project_children = [rel.RelatedObjects for rel in project.IsDecomposedBy]
        site_in_project = any(site in children for children in project_children)
        site_children = [rel.RelatedObjects for rel in site.IsDecomposedBy]
        building_in_site = any(building in children for children in site_children)
        building_children_list = [rel.RelatedObjects for rel in building.IsDecomposedBy]
        building_children = [child for children in building_children_list for child in children]
        for storey in storeys:
        # Each storey should contain pieces
        for storey in storeys:
            contained = [rel.RelatedElements for rel in storey.ContainsElements] if storey.ContainsElements else []
            elements = [e for group in contained for e in group]
        # Verify types have representations (model geometry)
        type_products = ifc.by_type("IfcBuildingElementProxyType")
        types_with_rep = [t for t in type_products if t.RepresentationMaps]

    def test_export_ifc_report(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        REPORTS_EXPORT_DIR.mkdir(parents=True, exist_ok=True)
        (REPORTS_EXPORT_DIR / "py.ifc").write_bytes(result)


class TestGetGeometricInsightsForModel:
    """Model/KPI tests for get_geometric_insights_for_model using nakagin-capsule-tower.gltf."""

    def test_nakagin_capsule_tower_gltf_returns_insights(self):
        model_path = os.path.join(os.path.dirname(__file__), TEST_ASSETS_DIR, "nakagin-capsule-tower.gltf")
        if not os.path.exists(model_path):
            pytest.skip("nakagin-capsule-tower.gltf not found")
        insights = get_geometric_insights_for_model(model_path)
        REPORTS_MODEL_KPI_DIR.mkdir(parents=True, exist_ok=True)
        data = geometric_insights_to_report_dict(insights)
        (REPORTS_MODEL_KPI_DIR / "py.json").write_text(json.dumps(data, indent=2, sort_keys=True), encoding="utf-8")

        canonical_path = os.path.join(os.path.dirname(__file__), TEST_ASSETS_DIR, "nakagin.kpi.model.semio.json")
        with open(canonical_path, "r", encoding="utf-8") as f:
            canonical = json.load(f)
        for key, expected in canonical.items():
        assert isinstance(insights, GeometricInsights)

    def test_nakagin_capsule_tower_from_bytes_gltf(self):
        model_path = os.path.join(os.path.dirname(__file__), TEST_ASSETS_DIR, "nakagin-capsule-tower.gltf")
        if not os.path.exists(model_path):
            pytest.skip("nakagin-capsule-tower.gltf not found")
        with open(model_path, "rb") as f:
            data = f.read()
        insights = get_geometric_insights_for_model(data)
        assert isinstance(insights, GeometricInsights)


class TestTypeMeta:
    """Tests for TypeMeta deserialization from JSON."""

    def test_type_meta(self):
        data = _test_load_json("tambour.meta.type.semio.json")
        assert meta["guid"] == data["guid"]


class TestTypeShallow:
    """Tests for TypeShallow deserialization from JSON."""

    def test_type_shallow(self):
        data = _test_load_json("tambour.shallow.type.semio.json")
        assert isinstance(shallow["connectors"], list)
        first_connector = shallow["connectors"][0]


class TestDesignMeta:
    """Tests for DesignMeta deserialization from JSON."""

    def test_design_meta(self):
        data = _test_load_json("nakagin-capsule-tower.meta.design.semio.json")
        assert meta["guid"] == data["guid"]


class TestDesignShallow:
    """Tests for DesignShallow deserialization from JSON."""

    def test_design_shallow(self):
        data = _test_load_json("nakagin-capsule-tower.shallow.design.semio.json")
        assert isinstance(shallow["pieces"], list)
        first_piece = shallow["pieces"][0]
        if "connections" in shallow:
            assert isinstance(shallow["connections"], list)
            if len(shallow["connections"]) > 0:
                first_conn = shallow["connections"][0]


class TestKitMeta:
    """Tests for KitMeta deserialization from JSON."""

    def test_kit_meta(self):
        data = _test_load_json("metabolism.meta.kit.semio.json")
        assert meta["guid"] == data["guid"]


class TestKitShallow:
    """Tests for KitShallow deserialization from JSON."""

    def test_kit_shallow(self):
        data = _test_load_json("metabolism.shallow.kit.semio.json")
        assert "name" not in data or isinstance(data.get("name"), str)
        assert isinstance(shallow["types"], list)
        first_type = shallow["types"][0]


class TestKitToMetaShallow:
    """Tests for converting a full kit dict to meta and shallow representations."""

    def test_kit_to_meta_shallow(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        expected_meta = _test_load_json("metabolism.meta.kit.semio.json")
        expected_shallow = _test_load_json("metabolism.shallow.kit.semio.json")

        computed_meta = kitToMeta(kit_dict)
        assert computed_meta["guid"] == expected_meta["guid"]
        assert computed_meta["name"] == expected_meta.get("name", computed_meta["name"])
        for key in expected_meta:
            if key in computed_meta:

        computed_shallow = kitToShallow(kit_dict)
        assert computed_shallow["guid"] == expected_shallow["guid"]
        assert isinstance(computed_shallow["types"], list)

        expected_type_guids = {t["guid"] for t in expected_shallow.get("types", [])}
        computed_type_guids = {t["guid"] for t in computed_shallow.get("types", [])}

        for t in computed_shallow.get("types", []):

        expected_type_meta = _test_load_json("tambour.meta.type.semio.json")
        computed_type_meta = typeToMeta(next(t for t in kit_dict["types"] if t["guid"] == expected_type_meta["guid"]))
        for key in expected_type_meta:
            if key in computed_type_meta:

        expected_design_meta = _test_load_json("nakagin-capsule-tower.meta.design.semio.json")
        computed_design_meta = designToMeta(next(d for d in kit_dict["designs"] if d["guid"] == expected_design_meta["guid"]))
        for key in expected_design_meta:
            if key in computed_design_meta:


class TestKitKind:
    """Tests for the KitKind enum."""

    def test_all_kit_kinds_has_five_values(self):

    def test_kit_kind_values(self):

    def test_kit_kind_is_str(self):
        for kind in KitKind:
            assert isinstance(kind, str)

    def test_kit_kind_file_roundtrip(self):
            "name": "FileTest",
            "uri": "file:///test.json",
            "types": [],
            "designs": [],
        }
        kit = Kit.parse(kit_dict)
        kit2 = Kit.parse({"name": kit.name, "uri": kit.uri})

    def test_kit_kind_temporary_in_memory(self):
        kit = Kit.parse({"name": "TempKit"})
        assert kit.uri.startswith("memory://")


class TestHash:
    """Tests for the Merkle hash functions."""

    def test_metabolism_kit_hash(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = hash_kit(kit_dict)

    def test_kit_diff_canonical_hash(self):
        d = {"name": "updated", "description": None}
        result = hash_kit_diff(d)

    def test_kit_diff_deterministic(self):
        d = {"name": "updated", "description": None}
        h1 = hash_kit_diff(d)
        h2 = hash_kit_diff(d)

    def test_kit_diff_different_inputs(self):
        d1 = {"name": "updated", "description": None}
        d2 = {"name": "other"}
        assert hash_kit_diff(d1) != hash_kit_diff(d2)

    def test_kit_diff_empty(self):
        d = {}
        result = hash_kit_diff(d)

    def test_attribute_diff_deterministic(self):
        d = {"key": "newKey", "value": "newValue"}
        h1 = hash_attribute_diff(d)
        h2 = hash_attribute_diff(d)

    def test_coord_diff_deterministic(self):
        d = {"u": 1.0, "v": 2.0}
        h1 = hash_coord_diff(d)
        h2 = hash_coord_diff(d)


# endregion Test

# region Benchmark
# [👤semio📚py💻main🔖benchmark](repo://p/u/semio/b/l/py/f/main.py/s/Benchmark)
# Benchmarks for the semio py module.



def _bench(name: str, func):
    start = time.perf_counter()
    for _ in range(BENCHMARK_ITERATIONS):
        func()
    end = time.perf_counter()
    print(f"{name},{duration:.6f}")


def benchmark_main():
    kit_metabolism = _test_load_kit("metabolism.kit.semio.json")
    kit_invalid = _test_load_kit("invalid.kit.semio.json")

    kit_obj = Kit.parse(kit_metabolism)

    kit_invalid_obj = Kit.parse(kit_invalid)

    def test_roundtrip():
        kit, files = import_kit(os.path.join(TEST_ASSETS_DIR, "metabolism.zip"))

        export_kit(kit, files, "temp_benchmark_metabolism.zip")
        if os.path.exists("temp_benchmark_metabolism.zip"):
            os.remove("temp_benchmark_metabolism.zip")

    _bench("Roundtrip/Metabolism", test_roundtrip)

    diff_forward = _test_load_json("metabolism.kit.diff.semio.json")
    diff_inverse = _test_load_json("metabolism.kit.diff.inverted.semio.json")

    def test_diff_metabolism():
        k2 = applyKitDiffDict(kit_metabolism, diff_forward)
        applyKitDiffDict(k2, diff_inverse)

    _bench("Diff/Metabolism", test_diff_metabolism)

    d1 = _test_find_design(kit_metabolism, "Nakagin Capsule Tower")

    def test_flatten_nakagin():
        flattenDesignDict(kit_metabolism, d1["guid"])

    _bench("Flatten Design/Nakagin Capsule Tower", test_flatten_nakagin)

    d2 = _test_find_design(kit_metabolism, "Slanted", "Nakagin Capsule Tower")

    def test_flatten_nakagin_slanted():
        flattenDesignDict(kit_metabolism, d2["guid"])

    _bench("Flatten Design/Nakagin Capsule Tower/Slanted", test_flatten_nakagin_slanted)

    d3 = _test_find_design(kit_metabolism, "Twisted", "Nakagin Capsule Tower")

    def test_flatten_nakagin_twisted():
        flattenDesignDict(kit_metabolism, d3["guid"])

    _bench("Flatten Design/Nakagin Capsule Tower/Twisted", test_flatten_nakagin_twisted)

    d4 = _test_find_design(kit_metabolism, "Dancing", "Nakagin Capsule Tower")

    def test_flatten_nakagin_dancing():
        flattenDesignDict(kit_metabolism, d4["guid"])

    _bench("Flatten Design/Nakagin Capsule Tower/Dancing", test_flatten_nakagin_dancing)

    d5 = _test_find_design(kit_metabolism, "Capsule Dream")

    def test_flatten_capsule_dream():
        flattenDesignDict(kit_metabolism, d5["guid"])

    _bench("Flatten Design/Capsule Dream", test_flatten_capsule_dream)

    def test_validate_invalid():
        validateKit(kit_invalid_obj)

    _bench("Validation/Invalid Kit", test_validate_invalid)

    def test_validate_metabolism():
        validateKit(kit_obj)

    _bench("Validation/Metabolism", test_validate_metabolism)


if __name__ == "__main__":
    benchmark_main()

# endregion Benchmark
