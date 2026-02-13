# region Header

# [💻semio/py/semio.py](semiorepo://file/semio/py/semio.py)

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

# [🔖semio/py/semio.py#Imports](semiorepo://section/semio/py/semio.py/IMPORTS)
# Standard library, third-party and framework imports.
from __future__ import annotations
import abc
import dataclasses
import datetime
import json
import os
import pathlib
import shutil
import sys
import tempfile
import typing
import urllib
import uuid
import zipfile

import dotenv
import fastapi
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
        return _original_get_relationship_to(name, rel_info, annotation)
        return _original_get_relationship_to(name=name, rel_info=rel_info, annotation=annotation)

    sqlmodel._compat.get_relationship_to = _patched_get_relationship_to
    import sqlmodel.main

    sqlmodel.main.get_relationship_to = _patched_get_relationship_to
import graphene_pydantic
import graphene_sqlalchemy
import loguru
import networkx
import numpy
import pydantic
import pytransform3d.rotations
import sqlalchemy
import sqlalchemy.orm
import sqlmodel

# endregion Imports

# region Type Hints

# [🔖semio/py/semio.py#Type Hints](semiorepo://section/semio/py/semio.py/TYPE-HINTS)
# Custom type hint aliases used throughout the module.

RecursiveAnyList = typing.Any | list["RecursiveAnyList"]
"""🔁 A recursive any list is either any or a list where the items are recursive any list."""

# endregion Type Hints

# region Constants

# [🔖semio/py/semio.py#Constants](semiorepo://section/semio/py/semio.py/CONSTANTS)
# Global constants for limits, paths, encodings and configuration.

NAME = "semio"
EMAIL = "mail@semio-tech.com"
RELEASE = "r25.07-1"
VERSION = "4.3.0-beta"
HOST = "0.0.0.0"
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
ENCODED_NAME_AND_VARIANT_AND_VIEW_PATH = typing.Annotated[
    str,
    fastapi.Path(pattern=ENCODING_REGEX + "," + ENCODING_ALPHABET_REGEX + "*" + "," + ENCODING_ALPHABET_REGEX + "*"),
]
MAX_REQUEST_BODY_SIZE = 50 * 1024 * 1024
dotenv.load_dotenv()
ENVS = {key: value for key, value in os.environ.items() if key.startswith("SEMIO_")}

# endregion Constants

# region Utility

# [🔖semio/py/semio.py#Utility](semiorepo://section/semio/py/semio.py/UTILITY)
# General-purpose utility functions for encoding, formatting and transformation.

def encode(value: str) -> str:
    """ᗒ Encode a string to be url safe.
    encode MUST return a percent-encoded string safe for URL paths.
    [🛠️semio/py/semio.py#Utility§encode](semiorepo://definition/semio/py/semio.py/UTILITY/ENCODE)
    """
    return urllib.parse.quote(value, safe="")

def decode(value: str) -> str:
    """ᗕ Decode a url safe string.
    decode MUST return the original string from a percent-encoded input.
    [🛠️semio/py/semio.py#Utility§decode](semiorepo://definition/semio/py/semio.py/UTILITY/DECODE)
    """
    return urllib.parse.unquote(value)

def encodeList(items: list[str]) -> str:
    """Encode a list of strings into a comma-separated URL-safe string.
    encodeList MUST encode each item and join them with commas.
    [🛠️semio/py/semio.py#Utility§encodeList](semiorepo://definition/semio/py/semio.py/UTILITY/ENCODELIST)
    """
    return ",".join([encode(t) for t in items])

def decodeList(encodedList: str) -> list[str]:
    """Decode a comma-separated URL-safe string into a list of strings.
    decodeList MUST split by comma and decode each item.
    [🛠️semio/py/semio.py#Utility§decodeList](semiorepo://definition/semio/py/semio.py/UTILITY/DECODELIST)
    """
    return [decode(t) for t in encodedList.split(",")]

def encodeRecursiveAnyList(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Encode a `RecursiveAnyList` to a url encoded string.
    encodeRecursiveAnyList MUST recursively encode nested lists into a flat string.
    [🛠️semio/py/semio.py#Utility§encodeRecursiveAnyList](semiorepo://definition/semio/py/semio.py/UTILITY/ENCODERECURSIVEANYLIST)
    """
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return encode(",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList]))

def create_id(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Turn any into `encoded(str(any))` or a recursive list into a flat comma [,] separated encoded list.
    create_id MUST produce a deterministic identifier from any value or nested list.
    [🛠️semio/py/semio.py#Utility§create_id](semiorepo://definition/semio/py/semio.py/UTILITY/CREATE-ID)
    """
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return ",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList])

def pretty(number: float) -> str:
    """🦋 Pretty print a floating point number.
    pretty MUST format the number with up to 5 significant digits.
    [🛠️semio/py/semio.py#Utility§pretty](semiorepo://definition/semio/py/semio.py/UTILITY/PRETTY)
    """
    if number == -0.0:
        number = 0.0
    return f"{number:.5f}".rstrip("0").rstrip(".")

def changeValues(c: dict | list, key: str, func: typing.Callable[[typing.Any], typing.Any]) -> None:
    """Recursively change values for a given key in nested dicts and lists.
    changeValues MUST apply the function to all occurrences of the key recursively.
    [🛠️semio/py/semio.py#Utility§changeValues](semiorepo://definition/semio/py/semio.py/UTILITY/CHANGEVALUES)
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
    [🛠️semio/py/semio.py#Utility§changeKeys](semiorepo://definition/semio/py/semio.py/UTILITY/CHANGEKEYS)
    """
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
    """🔃 Normalize an angle to be greater or equal to 0 and smaller than 360 degrees.
    normalizeAngle MUST return an angle in the range [0, 360).
    [🛠️semio/py/semio.py#Utility§normalizeAngle](semiorepo://definition/semio/py/semio.py/UTILITY/NORMALIZEANGLE)
    """
    return (angle % 360 + 360) % 360

# endregion Utility

# region Logging

# [🔖semio/py/semio.py#Logging](semiorepo://section/semio/py/semio.py/LOGGING)
# Module-level logger configuration.

logger = loguru.logger

# endregion Logging

# region Exceptions

# [🔖semio/py/semio.py#Exceptions](semiorepo://section/semio/py/semio.py/EXCEPTIONS)
# Custom exception hierarchy for server, client and specification errors.

class Error(Exception, abc.ABC):
    """❗ The base for all exceptions.
    Error MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Exceptions§Error](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/ERROR)
    """

    def __str__(self):
        return "❗ " + self.__class__.__name__

class ServerError(Error, abc.ABC):
    """🖥 The base for all server errors.
    ServerError MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Exceptions§ServerError](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/SERVERERROR)
    """

class ClientError(Error, abc.ABC):
    """👩‍💼 The base for all client errors.
    ClientError MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Exceptions§ClientError](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/CLIENTERROR)
    """

class CodeUnreachable(ServerError):
    """Exception for code paths that should never be reached.
    CodeUnreachable MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Exceptions§CodeUnreachable](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/CODEUNREACHABLE)
    """
    def __str__(self):
        return "🤷 This code should be unreachable."

class FeatureNotYetSupported(ServerError):
    """Exception for unimplemented features.
    FeatureNotYetSupported MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Exceptions§FeatureNotYetSupported](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/FEATURENOTYETSUPPORTED)
    """
    def __str__(self):
        return "🔜 This feature is not yet supported."

class RemoteKitsNotYetSupported(FeatureNotYetSupported):
    """Exception for unsupported remote kit access.
    RemoteKitsNotYetSupported MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Exceptions§RemoteKitsNotYetSupported](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/REMOTEKITSNOTYETSUPPORTED)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return "🔜 Remote kits are not yet supported."

class NotFound(ClientError, abc.ABC):
    """🔍 The base for not found errors.
    NotFound MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Exceptions§NotFound](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/NOTFOUND)
    """

class SpecificationError(ClientError, abc.ABC):
    """📋 The base for all specification errors.
    SpecificationError MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Exceptions§SpecificationError](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/SPECIFICATIONERROR)
    """

class NoParentAssigned(SpecificationError, abc.ABC):
    """👪 The base for all no parent assigned errors.
    NoParentAssigned MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Exceptions§NoParentAssigned](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/NOPARENTASSIGNED)
    """

class NoTypeOrDesignAssigned(NoParentAssigned):
    """No Type Or Design Assigned definition.
    NoTypeOrDesignAssigned MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Exceptions§NoTypeOrDesignAssigned](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/NOTYPEORDESIGNASSIGNED)
    """
    def __str__(self):
        return "👪 The entity has no parent type or design assigned."

class NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned(NoParentAssigned):
    """No Model Or Port Or Type Or Piece Or Connection Or Design Or Kit Assigned definition.
    NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Exceptions§NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/NOMODELORPORTORTYPEORPIECEORCONNECTIONORDESIGNORKITASSIGNED)
    """
    def __str__(self):
        return "👪 The entity has no parent model, connector, type, piece, connection, design, kit or folder assigned."

class AlreadyExists(SpecificationError, abc.ABC):
    """♊ The entity already exists in the store.
    AlreadyExists MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Exceptions§AlreadyExists](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/ALREADYEXISTS)
    """

class Semio(sqlmodel.SQLModel, table=True):
    """ℹ Metadata about the database.
    Semio MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Exceptions§Semio](semiorepo://definition/semio/py/semio.py/EXCEPTIONS/SEMIO)
    """

    __tablename__ = "semio"

    release: str = sqlmodel.Field(default=RELEASE, primary_key=True)
    """🍾 The current release of semio."""
    engine: str = sqlmodel.Field(default=VERSION)
    """⚙️The version of the engine that created this database."""
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)
    """⌚ The time when the database was created."""

# endregion Exceptions

# region Modeling

# [🔖semio/py/semio.py#Modeling](semiorepo://section/semio/py/semio.py/MODELING)

# region Primitives

# [🔖semio/py/semio.py#Primitives](semiorepo://section/semio/py/semio.py/PRIMITIVES)
# Abstract base classes for models, fields, ids, inputs, outputs and entities.

class SModel(sqlmodel.SQLModel, abc.ABC):
    """⚪ The base for models.
    SModel MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§SModel](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/SMODEL)
    """

    model_config = pydantic.ConfigDict(arbitrary_types_allowed=True)

    @classmethod
    def parse(cls, input: str | dict | typing.Any | None) -> "SModel":
        """⚒ Parse the entity from an input."""
        if input is None:
            return cls()
        if isinstance(input, str):
            return cls.model_validate_json(input)
        return cls.model_validate(input)

    def dump(self) -> "Output":
        """📦Dump the entity to a dictionary."""
        return self.model_dump()

BaseModel = SModel

class Field(SModel, abc.ABC):
    """🎫 The base for a field of a model.
    Field MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Modeling#Primitives§Field](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/FIELD)
    """

class RealField(Field, abc.ABC):
    """🧑 The base for a real field of a model. No lie.
    RealField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Modeling#Primitives§RealField](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/REALFIELD)
    """

class MaskedField(Field, abc.ABC):
    """🎭 The base for a mask of a field of a model. WYSIWYG but don't expect it to be there.
    MaskedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Modeling#Primitives§MaskedField](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/MASKEDFIELD)
    """

class Base(SModel, abc.ABC):
    """👥 The base for models.
    Base MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§Base](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/BASE)
    """

class Id(Base, abc.ABC):
    """🪪 The base for ids. All fields that identify the entity here.
    Id MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§Id](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/ID)
    """

class Props(Base, abc.ABC):
    """🎫 The base for props. All fields except input-only, output-only or child entities.
    Props MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§Props](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/PROPS)
    """

class Input(Base, abc.ABC):
    """↘ The base for inputs. All fields that are required to create the entity.
    Input MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§Input](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/INPUT)
    """

class Context(Base, abc.ABC):
    """📑 The base for contexts. All fields that are required to understand the entity by an llm.
    Context MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§Context](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/CONTEXT)
    """

class Output(Base, abc.ABC):
    """↗ The base for outputs. All fields that are returned when the entity is fetched.
    Output MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§Output](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/OUTPUT)
    """

class Prediction(Base, abc.ABC):
    """🔮 The base for predictions. All fields that are required to predict the entity by a llm.
    Prediction MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§Prediction](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/PREDICTION)
    """

class Entity(SModel, abc.ABC):
    """▢ The base for entities. All fields and behavior of the entity.
    Entity MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§Entity](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/ENTITY)
    """

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

class Table(SModel, abc.ABC):
    """▦ The base for tables. All resources that are stored in the database.
    Table MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§Table](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/TABLE)
    """

class TableEntity(Entity, Table, abc.ABC):
    """▢ The base for table entities.
    TableEntity MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Modeling#Primitives§TableEntity](semiorepo://definition/semio/py/semio.py/MODELING/PRIMITIVES/TABLEENTITY)
    """

    __tablename__: typing.ClassVar[str]
    """📛 The lowercase name of the table in the database."""

# endregion Primitives

# region Graphql

# [🔖semio/py/semio.py#Graphql](semiorepo://section/semio/py/semio.py/GRAPHQL)
# GraphQL node base classes for pydantic, sqlalchemy and relay integration.

class Node(graphene_pydantic.PydanticObjectType):
    """A base class for all nodes that are not a table in the database.
    Node MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Modeling#Graphql§Node](semiorepo://definition/semio/py/semio.py/MODELING/GRAPHQL/NODE)
    """

    class Meta:
        abstract = True

    @classmethod
    def __init_subclass_with_meta__(cls, model=None, **options):
        if "name" not in options:
            options["name"] = model.__name__

        super().__init_subclass_with_meta__(model=model, **options)

class InputNode(graphene_pydantic.PydanticInputObjectType):
    """A base class for all input nodes.
    InputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Modeling#Graphql§InputNode](semiorepo://definition/semio/py/semio.py/MODELING/GRAPHQL/INPUTNODE)
    """

    class Meta:
        abstract = True

class RelayNode(graphene.relay.Node):
    """Relay-compliant GraphQL node interface.
    RelayNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Modeling#Graphql§RelayNode](semiorepo://definition/semio/py/semio.py/MODELING/GRAPHQL/RELAYNODE)
    """
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
    Child relationships are by default included.
    TableNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Modeling#Graphql§TableNode](semiorepo://definition/semio/py/semio.py/MODELING/GRAPHQL/TABLENODE)
    """

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
    It automatically complies to the Relay Node interface.
    TableEntityNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Modeling#Graphql§TableEntityNode](semiorepo://definition/semio/py/semio.py/MODELING/GRAPHQL/TABLEENTITYNODE)
    """

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

# [🔖semio/py/semio.py#Domain](semiorepo://section/semio/py/semio.py/DOMAIN)

# region Attribute

# [🔖semio/py/semio.py#Attribute](semiorepo://section/semio/py/semio.py/ATTRIBUTE)
# Attribute entity with key-value pairs and definitions.

class AttributeKeyField(RealField, abc.ABC):
    """Field mixin for the key of a attribute.
    AttributeKeyField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Attribute§AttributeKeyField](semiorepo://definition/semio/py/semio.py/DOMAIN/ATTRIBUTE/ATTRIBUTEKEYFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class AttributeValueField(RealField, abc.ABC):
    """Field mixin for the value of a attribute.
    AttributeValueField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Attribute§AttributeValueField](semiorepo://definition/semio/py/semio.py/DOMAIN/ATTRIBUTE/ATTRIBUTEVALUEFIELD)
    """
    value: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class AttributeDefinitionField(RealField, abc.ABC):
    """Field mixin for the definition of a attribute.
    AttributeDefinitionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Attribute§AttributeDefinitionField](semiorepo://definition/semio/py/semio.py/DOMAIN/ATTRIBUTE/ATTRIBUTEDEFINITIONFIELD)
    """
    definition: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class AttributeId(AttributeKeyField, Id):
    """Identity fields for uniquely identifying a attribute.
    AttributeId MUST contain all fields that uniquely identify a attribute.
    [🛠️semio/py/semio.py#Domain#Attribute§AttributeId](semiorepo://definition/semio/py/semio.py/DOMAIN/ATTRIBUTE/ATTRIBUTEID)
    """
    pass

class AttributeProps(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Props):
    """Property fields for a attribute.
    AttributeProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Attribute§AttributeProps](semiorepo://definition/semio/py/semio.py/DOMAIN/ATTRIBUTE/ATTRIBUTEPROPS)
    """
    pass

class AttributeInput(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Input):
    """Input fields for creating or updating a attribute.
    AttributeInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Attribute§AttributeInput](semiorepo://definition/semio/py/semio.py/DOMAIN/ATTRIBUTE/ATTRIBUTEINPUT)
    """
    pass

class AttributeContext(AttributeValueField, AttributeKeyField, Context):
    """Context fields for understanding a attribute by an LLM.
    AttributeContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Attribute§AttributeContext](semiorepo://definition/semio/py/semio.py/DOMAIN/ATTRIBUTE/ATTRIBUTECONTEXT)
    """
    pass

class AttributeOutput(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Output):
    """Output fields returned when fetching a attribute.
    AttributeOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Attribute§AttributeOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/ATTRIBUTE/ATTRIBUTEOUTPUT)
    """
    pass

class Attribute(
    AttributeDefinitionField,
    AttributeValueField,
    AttributeKeyField,
    TableEntity,
    table=True,
):
    """Attribute entity storing a key-value pair with an optional definition.
    Attribute MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Attribute§Attribute](semiorepo://definition/semio/py/semio.py/DOMAIN/ATTRIBUTE/ATTRIBUTE)
    """
    PLURAL = "attributes"
    __tablename__ = "attribute"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    modelPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("model_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("model.id")),
        default=None,
        exclude=True,
    )
    model: "Model" = sqlmodel.Relationship(back_populates="attributes")
    connectorPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("connector_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("connector.id")),
        default=None,
        exclude=True,
    )
    connector: "Connector" = sqlmodel.Relationship(back_populates="attributes")
    typePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("type.id")),
        default=None,
        exclude=True,
    )
    type: "Type" = sqlmodel.Relationship(back_populates="attributes")
    piecePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("piece.id")),
        default=None,
        exclude=True,
    )
    piece: "Piece" = sqlmodel.Relationship(back_populates="attributes")
    connectionPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "connection_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("connection.id"),
        ),
        default=None,
        exclude=True,
    )
    connection: "Connection" = sqlmodel.Relationship(back_populates="attributes")
    designPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("design.id")),
        default=None,
        exclude=True,
    )
    design: "Design" = sqlmodel.Relationship(back_populates="attributes")
    kitPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kit.id")),
        default=None,
        exclude=True,
    )
    kit: "Kit" = sqlmodel.Relationship(back_populates="attributes")
    qualityPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("quality_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("quality.id")),
        default=None,
        exclude=True,
    )
    quality: "Quality" = sqlmodel.Relationship(back_populates="attributes")
    propPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("prop_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("prop.id")),
        default=None,
        exclude=True,
    )
    prop: "Prop" = sqlmodel.Relationship(back_populates="attributes")
    authorPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("author_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("author.id")),
        default=None,
        exclude=True,
    )
    author: "Author" = sqlmodel.Relationship(back_populates="attributes")
    locationPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("location_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("location.id")),
        default=None,
        exclude=True,
    )
    location: "Location" = sqlmodel.Relationship(back_populates="attributes")
    benchmarkPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("benchmark_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("benchmark.id")),
        default=None,
        exclude=True,
    )
    benchmark: "Benchmark" = sqlmodel.Relationship(back_populates="attributes")
    folderPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("folder_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("folder.id")),
        default=None,
        exclude=True,
    )
    folder: "Folder" = sqlmodel.Relationship(back_populates="attributes")
    portPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("port_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("port.id")),
        default=None,
        exclude=True,
    )
    port_: "Port" = sqlmodel.Relationship(back_populates="attributes")

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

    def parent(
        self,
    ) -> typing.Union[
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
    """GraphQL input node for attribute mutations.
    AttributeInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Attribute§AttributeInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/ATTRIBUTE/ATTRIBUTEINPUTNODE)
    """
    class Meta:
        model = AttributeInput

# endregion Attribute

# region Tag

# [🔖semio/py/semio.py#Tag](semiorepo://section/semio/py/semio.py/TAG)
# Tag entity for categorizing and labeling kit elements.

class TagGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a tag.
    TagGuidField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Tag§TagGuidField](semiorepo://definition/semio/py/semio.py/DOMAIN/TAG/TAGGUIDFIELD)
    """
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class TagNameField(RealField, abc.ABC):
    """Field mixin for the name of a tag.
    TagNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Tag§TagNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/TAG/TAGNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class TagDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a tag.
    TagDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Tag§TagDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/TAG/TAGDESCRIPTIONFIELD)
    """
    description: typing.Optional[str] = sqlmodel.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)

class TagIconField(RealField, abc.ABC):
    """Field mixin for the icon of a tag.
    TagIconField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Tag§TagIconField](semiorepo://definition/semio/py/semio.py/DOMAIN/TAG/TAGICONFIELD)
    """
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class TagOrderField(RealField, abc.ABC):
    """Field mixin for the order of a tag.
    TagOrderField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Tag§TagOrderField](semiorepo://definition/semio/py/semio.py/DOMAIN/TAG/TAGORDERFIELD)
    """
    order: int = sqlmodel.Field(default=0)

class TagId(TagGuidField, Id):
    """Identity fields for uniquely identifying a tag.
    TagId MUST contain all fields that uniquely identify a tag.
    [🛠️semio/py/semio.py#Domain#Tag§TagId](semiorepo://definition/semio/py/semio.py/DOMAIN/TAG/TAGID)
    """
    pass

class Tag(
    TagIconField,
    TagDescriptionField,
    TagOrderField,
    TagNameField,
    TagGuidField,
    Table,
    table=True,
):
    """Tag entity for labeling kit elements with name, icon and order.
    Tag MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Tag§Tag](semiorepo://definition/semio/py/semio.py/DOMAIN/TAG/TAG)
    """
    __tablename__ = "tag"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    modelPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("model_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("model.id")),
        default=None,
        exclude=True,
    )
    model: Model = sqlmodel.Relationship(back_populates="tags_")

# endregion Tag

# region Concept

# [🔖semio/py/semio.py#Concept](semiorepo://section/semio/py/semio.py/CONCEPT)
# Concept entity for semantic grouping of design elements.

class ConceptGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a concept.
    ConceptGuidField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Concept§ConceptGuidField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONCEPT/CONCEPTGUIDFIELD)
    """
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class ConceptNameField(RealField, abc.ABC):
    """Field mixin for the name of a concept.
    ConceptNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Concept§ConceptNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONCEPT/CONCEPTNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class ConceptDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a concept.
    ConceptDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Concept§ConceptDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONCEPT/CONCEPTDESCRIPTIONFIELD)
    """
    description: typing.Optional[str] = sqlmodel.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)

class ConceptIconField(RealField, abc.ABC):
    """Field mixin for the icon of a concept.
    ConceptIconField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Concept§ConceptIconField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONCEPT/CONCEPTICONFIELD)
    """
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class ConceptOrderField(RealField, abc.ABC):
    """Field mixin for the order of a concept.
    ConceptOrderField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Concept§ConceptOrderField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONCEPT/CONCEPTORDERFIELD)
    """
    order: int = sqlmodel.Field(default=0)

class ConceptId(ConceptGuidField, Id):
    """Identity fields for uniquely identifying a concept.
    ConceptId MUST contain all fields that uniquely identify a concept.
    [🛠️semio/py/semio.py#Domain#Concept§ConceptId](semiorepo://definition/semio/py/semio.py/DOMAIN/CONCEPT/CONCEPTID)
    """
    pass

class Concept(
    ConceptIconField,
    ConceptDescriptionField,
    ConceptOrderField,
    ConceptNameField,
    ConceptGuidField,
    Table,
    table=True,
):
    """Concept entity for semantic grouping with name, icon and order.
    Concept MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Concept§Concept](semiorepo://definition/semio/py/semio.py/DOMAIN/CONCEPT/CONCEPT)
    """
    __tablename__ = "concept"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    kitPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kit.id")),
        default=None,
        exclude=True,
    )
    typePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("type.id")),
        default=None,
        exclude=True,
    )
    designPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("design.id")),
        default=None,
        exclude=True,
    )
    kit: Kit = sqlmodel.Relationship(back_populates="concepts_")
    type: Type = sqlmodel.Relationship(back_populates="concepts_")
    design: Design = sqlmodel.Relationship(back_populates="concepts_")

# endregion Concept

# region Coord

# [🔖semio/py/semio.py#Coord](semiorepo://section/semio/py/semio.py/COORD)
# Coordinate primitive for three-dimensional values.

class Coord(SModel):
    """Three-dimensional coordinate with x, y and z values.
    Coord MUST contain all coordinate or geometry fields.
    [🛠️semio/py/semio.py#Domain#Coord§Coord](semiorepo://definition/semio/py/semio.py/DOMAIN/COORD/COORD)
    """
    u: float = sqlmodel.Field()
    v: float = sqlmodel.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.u)}, {pretty(self.v)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.u)}, {pretty(self.v)}]"

class CoordInput(Coord, Input):
    """Input fields for creating or updating a coord.
    CoordInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Coord§CoordInput](semiorepo://definition/semio/py/semio.py/DOMAIN/COORD/COORDINPUT)
    """
    pass

class CoordContext(Coord, Context):
    """Context fields for understanding a coord by an LLM.
    CoordContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Coord§CoordContext](semiorepo://definition/semio/py/semio.py/DOMAIN/COORD/COORDCONTEXT)
    """
    pass

class CoordOutput(Coord, Output):
    """Output fields returned when fetching a coord.
    CoordOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Coord§CoordOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/COORD/COORDOUTPUT)
    """
    pass

class CoordPrediction(Coord, Prediction):
    """Prediction fields for LLM-based coord inference.
    CoordPrediction MUST contain all fields for LLM inference.
    [🛠️semio/py/semio.py#Domain#Coord§CoordPrediction](semiorepo://definition/semio/py/semio.py/DOMAIN/COORD/COORDPREDICTION)
    """
    pass

class CoordNode(Node):
    """GraphQL node exposing coord data.
    CoordNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Coord§CoordNode](semiorepo://definition/semio/py/semio.py/DOMAIN/COORD/COORDNODE)
    """
    class Meta:
        model = Coord

class CoordInputNode(InputNode):
    """GraphQL input node for coord mutations.
    CoordInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Coord§CoordInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/COORD/COORDINPUTNODE)
    """
    class Meta:
        model = CoordInput

# endregion Coord

# region Point

# [🔖semio/py/semio.py#Point](semiorepo://section/semio/py/semio.py/POINT)
# Point primitive representing a position in 3D space.

class Point(SModel):
    """Point in 3D space with x, y and z coordinates.
    Point MUST contain all coordinate or geometry fields.
    [🛠️semio/py/semio.py#Domain#Point§Point](semiorepo://definition/semio/py/semio.py/DOMAIN/POINT/POINT)
    """
    x: float = sqlmodel.Field()
    y: float = sqlmodel.Field()
    z: float = sqlmodel.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

class PointInput(Point, Input):
    """Input fields for creating or updating a point.
    PointInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Point§PointInput](semiorepo://definition/semio/py/semio.py/DOMAIN/POINT/POINTINPUT)
    """
    pass

class PointContext(Point, Context):
    """Context fields for understanding a point by an LLM.
    PointContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Point§PointContext](semiorepo://definition/semio/py/semio.py/DOMAIN/POINT/POINTCONTEXT)
    """
    pass

class PointOutput(Point, Output):
    """Output fields returned when fetching a point.
    PointOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Point§PointOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/POINT/POINTOUTPUT)
    """
    pass

class PointPrediction(Point, Prediction):
    """Prediction fields for LLM-based point inference.
    PointPrediction MUST contain all fields for LLM inference.
    [🛠️semio/py/semio.py#Domain#Point§PointPrediction](semiorepo://definition/semio/py/semio.py/DOMAIN/POINT/POINTPREDICTION)
    """
    pass

class PointNode(Node):
    """GraphQL node exposing point data.
    PointNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Point§PointNode](semiorepo://definition/semio/py/semio.py/DOMAIN/POINT/POINTNODE)
    """
    class Meta:
        model = Point

class PointInputNode(InputNode):
    """GraphQL input node for point mutations.
    PointInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Point§PointInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/POINT/POINTINPUTNODE)
    """
    class Meta:
        model = PointInput

# endregion Point

# region Vector

# [🔖semio/py/semio.py#Vector](semiorepo://section/semio/py/semio.py/VECTOR)
# Vector primitive representing a direction in 3D space.

class Vector(SModel):
    """Direction vector in 3D space with x, y and z components.
    Vector MUST contain all coordinate or geometry fields.
    [🛠️semio/py/semio.py#Domain#Vector§Vector](semiorepo://definition/semio/py/semio.py/DOMAIN/VECTOR/VECTOR)
    """
    x: float = sqlmodel.Field()
    y: float = sqlmodel.Field()
    z: float = sqlmodel.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

class VectorInput(Vector, Input):
    """Input fields for creating or updating a vector.
    VectorInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Vector§VectorInput](semiorepo://definition/semio/py/semio.py/DOMAIN/VECTOR/VECTORINPUT)
    """
    pass

class VectorContext(Vector, Context):
    """Context fields for understanding a vector by an LLM.
    VectorContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Vector§VectorContext](semiorepo://definition/semio/py/semio.py/DOMAIN/VECTOR/VECTORCONTEXT)
    """
    pass

class VectorOutput(Vector, Output):
    """Output fields returned when fetching a vector.
    VectorOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Vector§VectorOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/VECTOR/VECTOROUTPUT)
    """
    pass

class VectorPrediction(Vector, Prediction):
    """Prediction fields for LLM-based vector inference.
    VectorPrediction MUST contain all fields for LLM inference.
    [🛠️semio/py/semio.py#Domain#Vector§VectorPrediction](semiorepo://definition/semio/py/semio.py/DOMAIN/VECTOR/VECTORPREDICTION)
    """
    pass

class VectorNode(Node):
    """GraphQL node exposing vector data.
    VectorNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Vector§VectorNode](semiorepo://definition/semio/py/semio.py/DOMAIN/VECTOR/VECTORNODE)
    """
    class Meta:
        model = Vector

class VectorInputNode(InputNode):
    """GraphQL input node for vector mutations.
    VectorInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Vector§VectorInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/VECTOR/VECTORINPUTNODE)
    """
    class Meta:
        model = VectorInput

# endregion Vector

# region Plane

# [🔖semio/py/semio.py#Plane](semiorepo://section/semio/py/semio.py/PLANE)
# Plane primitive representing an oriented coordinate frame in 3D space.

class PlaneOriginField(MaskedField, abc.ABC):
    """Field mixin for the origin of a plane.
    PlaneOriginField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Plane§PlaneOriginField](semiorepo://definition/semio/py/semio.py/DOMAIN/PLANE/PLANEORIGINFIELD)
    """
    origin: Point = sqlmodel.Field()

class PlaneXAxisField(MaskedField, abc.ABC):
    """Field mixin for the x axis of a plane.
    PlaneXAxisField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Plane§PlaneXAxisField](semiorepo://definition/semio/py/semio.py/DOMAIN/PLANE/PLANEXAXISFIELD)
    """
    xAxis: Vector = sqlmodel.Field()

class PlaneYAxisField(MaskedField, abc.ABC):
    """Field mixin for the y axis of a plane.
    PlaneYAxisField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Plane§PlaneYAxisField](semiorepo://definition/semio/py/semio.py/DOMAIN/PLANE/PLANEYAXISFIELD)
    """
    yAxis: Vector = sqlmodel.Field()

class PlaneInput(Input):
    """Input fields for creating or updating a plane.
    PlaneInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Plane§PlaneInput](semiorepo://definition/semio/py/semio.py/DOMAIN/PLANE/PLANEINPUT)
    """
    origin: PointInput = sqlmodel.Field()
    xAxis: VectorInput = sqlmodel.Field()
    yAxis: VectorInput = sqlmodel.Field()

class PlaneContext(Context):
    """Context fields for understanding a plane by an LLM.
    PlaneContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Plane§PlaneContext](semiorepo://definition/semio/py/semio.py/DOMAIN/PLANE/PLANECONTEXT)
    """
    origin: PointContext = sqlmodel.Field()
    xAxis: VectorContext = sqlmodel.Field()
    yAxis: VectorContext = sqlmodel.Field()

class PlaneOutput(PlaneYAxisField, PlaneXAxisField, PlaneOriginField, Output):
    """Output fields returned when fetching a plane.
    PlaneOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Plane§PlaneOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/PLANE/PLANEOUTPUT)
    """
    pass

class Plane(Table, table=True):
    """Oriented coordinate frame in 3D space with origin and axes.
    Plane MUST contain all coordinate or geometry fields.
    [🛠️semio/py/semio.py#Domain#Plane§Plane](semiorepo://definition/semio/py/semio.py/DOMAIN/PLANE/PLANE)
    """
    __tablename__ = "plane"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
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
    def parse(cls, input: str | dict | PlaneInput | typing.Any | None) -> "Plane":
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
    """GraphQL input node for plane mutations.
    PlaneInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Plane§PlaneInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/PLANE/PLANEINPUTNODE)
    """
    class Meta:
        model = PlaneInput

# endregion Plane

# region Location

# [🔖semio/py/semio.py#Location](semiorepo://section/semio/py/semio.py/LOCATION)
# Location entity for geographic coordinates with longitude, latitude and altitude.

class LocationGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a location.
    LocationGuidField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Location§LocationGuidField](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONGUIDFIELD)
    """
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class LocationLongitudeField(RealField, abc.ABC):
    """Field mixin for the longitude of a location.
    LocationLongitudeField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Location§LocationLongitudeField](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONLONGITUDEFIELD)
    """
    longitude: float = sqlmodel.Field()

class LocationLatitudeField(RealField, abc.ABC):
    """Field mixin for the latitude of a location.
    LocationLatitudeField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Location§LocationLatitudeField](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONLATITUDEFIELD)
    """
    latitude: float = sqlmodel.Field()

class LocationAltitudeField(RealField, abc.ABC):
    """Field mixin for the altitude of a location.
    LocationAltitudeField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Location§LocationAltitudeField](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONALTITUDEFIELD)
    """
    altitude: typing.Optional[float] = sqlmodel.Field(default=None)

class LocationId(LocationGuidField, Id):
    """Identity fields for uniquely identifying a location.
    LocationId MUST contain all fields that uniquely identify a location.
    [🛠️semio/py/semio.py#Domain#Location§LocationId](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONID)
    """
    pass

class Location(
    LocationAltitudeField,
    LocationLatitudeField,
    LocationLongitudeField,
    LocationGuidField,
    TableEntity,
    table=True,
):
    """Geographic location with longitude, latitude and altitude.
    Location MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Location§Location](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATION)
    """
    PLURAL = "locations"
    __tablename__ = "location"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="location", cascade_delete=True)

class LocationInput(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Input):
    """Input fields for creating or updating a location.
    LocationInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Location§LocationInput](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONINPUT)
    """
    pass

class LocationOutput(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Output):
    """Output fields returned when fetching a location.
    LocationOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Location§LocationOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONOUTPUT)
    """
    pass

class LocationContext(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Context):
    """Context fields for understanding a location by an LLM.
    LocationContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Location§LocationContext](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONCONTEXT)
    """
    pass

class LocationPrediction(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Prediction):
    """Prediction fields for LLM-based location inference.
    LocationPrediction MUST contain all fields for LLM inference.
    [🛠️semio/py/semio.py#Domain#Location§LocationPrediction](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONPREDICTION)
    """
    pass

class LocationNode(Node):
    """GraphQL node exposing location data.
    LocationNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Location§LocationNode](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONNODE)
    """
    class Meta:
        model = LocationOutput

class LocationInputNode(InputNode):
    """GraphQL input node for location mutations.
    LocationInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Location§LocationInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCATION/LOCATIONINPUTNODE)
    """
    class Meta:
        model = LocationInput

# endregion Location

# region Author

# [🔖semio/py/semio.py#Author](semiorepo://section/semio/py/semio.py/AUTHOR)
# Author entity for tracking contributor identity and rank.

class AuthorNameField(RealField, abc.ABC):
    """Field mixin for the name of a author.
    AuthorNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Author§AuthorNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/AUTHOR/AUTHORNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class AuthorEmailField(RealField, abc.ABC):
    """Field mixin for the email of a author.
    AuthorEmailField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Author§AuthorEmailField](semiorepo://definition/semio/py/semio.py/DOMAIN/AUTHOR/AUTHOREMAILFIELD)
    """
    email: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class AuthorRankField(RealField, abc.ABC):
    """Field mixin for the rank of a author.
    AuthorRankField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Author§AuthorRankField](semiorepo://definition/semio/py/semio.py/DOMAIN/AUTHOR/AUTHORRANKFIELD)
    """
    rank: int = sqlmodel.Field(default=0)

class AuthorId(AuthorEmailField, Id):
    """Identity fields for uniquely identifying a author.
    AuthorId MUST contain all fields that uniquely identify a author.
    [🛠️semio/py/semio.py#Domain#Author§AuthorId](semiorepo://definition/semio/py/semio.py/DOMAIN/AUTHOR/AUTHORID)
    """
    pass

class AuthorProps(AuthorEmailField, AuthorNameField, Props):
    """Property fields for a author.
    AuthorProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Author§AuthorProps](semiorepo://definition/semio/py/semio.py/DOMAIN/AUTHOR/AUTHORPROPS)
    """
    pass

class AuthorInput(AuthorEmailField, AuthorNameField, Input):
    """Input fields for creating or updating a author.
    AuthorInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Author§AuthorInput](semiorepo://definition/semio/py/semio.py/DOMAIN/AUTHOR/AUTHORINPUT)
    """
    pass

class AuthorOutput(AuthorEmailField, AuthorNameField, Output):
    """Output fields returned when fetching a author.
    AuthorOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Author§AuthorOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/AUTHOR/AUTHOROUTPUT)
    """
    pass

class Author(
    AuthorRankField,
    AuthorEmailField,
    AuthorNameField,
    TableEntity,
    table=True,
):
    """Author entity with name, email and contribution rank.
    Author MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Author§Author](semiorepo://definition/semio/py/semio.py/DOMAIN/AUTHOR/AUTHOR)
    """
    PLURAL = "authors"
    __tablename__ = "author"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    kitPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kit.id")),
        default=None,
        exclude=True,
    )
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
    """GraphQL input node for author mutations.
    AuthorInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Author§AuthorInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/AUTHOR/AUTHORINPUTNODE)
    """
    class Meta:
        model = AuthorInput

# endregion Author

# region ArtifactAuthor

# [🔖semio/py/semio.py#ArtifactAuthor](semiorepo://section/semio/py/semio.py/ARTIFACTAUTHOR)
# Artifact-author association entity linking artifacts to authors by email.

class ArtifactAuthorEmailField(RealField, abc.ABC):
    """Field mixin for the email of a artifact author.
    ArtifactAuthorEmailField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#ArtifactAuthor§ArtifactAuthorEmailField](semiorepo://definition/semio/py/semio.py/DOMAIN/ARTIFACTAUTHOR/ARTIFACTAUTHOREMAILFIELD)
    """
    author_email: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class ArtifactAuthor(ArtifactAuthorEmailField, TableEntity, table=True):
    """Association entity linking an artifact to an author by email.
    ArtifactAuthor MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#ArtifactAuthor§ArtifactAuthor](semiorepo://definition/semio/py/semio.py/DOMAIN/ARTIFACTAUTHOR/ARTIFACTAUTHOR)
    """
    PLURAL = "artifact_authors"
    __tablename__ = "artifact_author"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    typePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("type.id")),
        default=None,
        exclude=True,
    )
    type: Type = sqlmodel.Relationship(back_populates="artifact_authors")
    designPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("design.id")),
        default=None,
        exclude=True,
    )
    design: Design = sqlmodel.Relationship(back_populates="artifact_authors")

    __table_args__ = (
        sqlalchemy.CheckConstraint(
            "(type_id IS NOT NULL AND design_id IS NULL) OR (type_id IS NULL AND design_id IS NOT NULL)",
            name="ck_artifact_authors_parent_set",
        ),
        sqlalchemy.UniqueConstraint(
            "author_email",
            "type_id",
            "design_id",
            name="uq_artifact_authors_email_type_id_design_id",
        ),
    )

    def parent(self) -> typing.Union["Type", "Design", None]:
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

# endregion ArtifactAuthor

# region File

# [🔖semio/py/semio.py#File](semiorepo://section/semio/py/semio.py/FILE)
# File entity for managing binary assets with metadata and hashing.

class FileGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a file.
    FileGuidField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileGuidField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEGUIDFIELD)
    """
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class FileNameField(RealField, abc.ABC):
    """Field mixin for the name of a file.
    FileNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILENAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class FileMimeField(RealField, abc.ABC):
    """Field mixin for the mime of a file.
    FileMimeField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileMimeField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEMIMEFIELD)
    """
    mime: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)

class FileRemoteField(RealField, abc.ABC):
    """Field mixin for the remote of a file.
    FileRemoteField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileRemoteField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEREMOTEFIELD)
    """
    remote: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class FileFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a file.
    FileFolderField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileFolderField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEFOLDERFIELD)
    """
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class FileSizeField(RealField, abc.ABC):
    """Field mixin for the size of a file.
    FileSizeField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileSizeField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILESIZEFIELD)
    """
    size: typing.Optional[int] = sqlmodel.Field(default=None)

class FileHashField(RealField, abc.ABC):
    """Field mixin for the hash of a file.
    FileHashField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileHashField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEHASHFIELD)
    """
    hash: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)

class FileCreatedAtField(RealField, abc.ABC):
    """Field mixin for the created at of a file.
    FileCreatedAtField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileCreatedAtField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILECREATEDATFIELD)
    """
    createdAt: datetime.datetime = sqlmodel.Field()

class FileCreatedByField(RealField, abc.ABC):
    """Field mixin for the created by of a file.
    FileCreatedByField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileCreatedByField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILECREATEDBYFIELD)
    """
    createdBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class FileUpdatedAtField(RealField, abc.ABC):
    """Field mixin for the updated at of a file.
    FileUpdatedAtField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileUpdatedAtField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEUPDATEDATFIELD)
    """
    updatedAt: datetime.datetime = sqlmodel.Field()

class FileUpdatedByField(RealField, abc.ABC):
    """Field mixin for the updated by of a file.
    FileUpdatedByField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#File§FileUpdatedByField](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEUPDATEDBYFIELD)
    """
    updatedBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class FileId(FileGuidField, Id):
    """Identity fields for uniquely identifying a file.
    FileId MUST contain all fields that uniquely identify a file.
    [🛠️semio/py/semio.py#Domain#File§FileId](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEID)
    """
    pass

class FileProps(
    FileUpdatedByField,
    FileUpdatedAtField,
    FileCreatedByField,
    FileCreatedAtField,
    FileHashField,
    FileSizeField,
    FileFolderField,
    FileRemoteField,
    FileMimeField,
    FileNameField,
    FileGuidField,
    Props,
):
    """Property fields for a file.
    FileProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#File§FileProps](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEPROPS)
    """
    pass

class FileInput(
    FileUpdatedByField,
    FileUpdatedAtField,
    FileCreatedByField,
    FileCreatedAtField,
    FileHashField,
    FileSizeField,
    FileFolderField,
    FileRemoteField,
    FileMimeField,
    FileNameField,
    FileGuidField,
    Input,
):
    """Input fields for creating or updating a file.
    FileInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#File§FileInput](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEINPUT)
    """
    pass

class FileContext(FileNameField, FileGuidField, Context):
    """Context fields for understanding a file by an LLM.
    FileContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#File§FileContext](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILECONTEXT)
    """
    pass

class FileOutput(
    FileUpdatedByField,
    FileUpdatedAtField,
    FileCreatedByField,
    FileCreatedAtField,
    FileHashField,
    FileSizeField,
    FileFolderField,
    FileRemoteField,
    FileMimeField,
    FileNameField,
    FileGuidField,
    Output,
):
    """Output fields returned when fetching a file.
    FileOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#File§FileOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEOUTPUT)
    """
    pass

class File(
    FileUpdatedByField,
    FileUpdatedAtField,
    FileCreatedByField,
    FileCreatedAtField,
    FileHashField,
    FileSizeField,
    FileFolderField,
    FileRemoteField,
    FileMimeField,
    FileNameField,
    FileGuidField,
    TableEntity,
    table=True,
):
    """File entity for binary assets with metadata, hashing and timestamps.
    File MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#File§File](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILE)
    """
    PLURAL = "files"
    __tablename__ = "file"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    kitPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kit.id")),
        default=None,
        exclude=True,
    )
    kit: Kit = sqlmodel.Relationship(back_populates="files_")

    __table_args__ = (sqlalchemy.UniqueConstraint("guid", "kit_id", name="uq_files_guid_kit_id"),)

    def parent(self) -> "Kit":
        if self.kit is not None:
            return self.kit
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.guid

class FileInputNode(InputNode):
    """GraphQL input node for file mutations.
    FileInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#File§FileInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/FILE/FILEINPUTNODE)
    """
    class Meta:
        model = FileInput

# endregion File

# region Folder

# [🔖semio/py/semio.py#Folder](semiorepo://section/semio/py/semio.py/FOLDER)
# Folder entity for hierarchical organization of kit content.

class FolderGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a folder.
    FolderGuidField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Folder§FolderGuidField](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERGUIDFIELD)
    """
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class FolderNameField(RealField, abc.ABC):
    """Field mixin for the name of a folder.
    FolderNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Folder§FolderNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class FolderParentField(RealField, abc.ABC):
    """Field mixin for the parent of a folder.
    FolderParentField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Folder§FolderParentField](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERPARENTFIELD)
    """
    parent: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class FolderDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a folder.
    FolderDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Folder§FolderDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class FolderCreatedAtField(RealField, abc.ABC):
    """Field mixin for the created at of a folder.
    FolderCreatedAtField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Folder§FolderCreatedAtField](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERCREATEDATFIELD)
    """
    createdAt: datetime.datetime = sqlmodel.Field()

class FolderCreatedByField(RealField, abc.ABC):
    """Field mixin for the created by of a folder.
    FolderCreatedByField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Folder§FolderCreatedByField](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERCREATEDBYFIELD)
    """
    createdBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class FolderUpdatedAtField(RealField, abc.ABC):
    """Field mixin for the updated at of a folder.
    FolderUpdatedAtField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Folder§FolderUpdatedAtField](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERUPDATEDATFIELD)
    """
    updatedAt: datetime.datetime = sqlmodel.Field()

class FolderUpdatedByField(RealField, abc.ABC):
    """Field mixin for the updated by of a folder.
    FolderUpdatedByField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Folder§FolderUpdatedByField](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERUPDATEDBYFIELD)
    """
    updatedBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class FolderId(FolderGuidField, Id):
    """Identity fields for uniquely identifying a folder.
    FolderId MUST contain all fields that uniquely identify a folder.
    [🛠️semio/py/semio.py#Domain#Folder§FolderId](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERID)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Folder§FolderProps](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERPROPS)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Folder§FolderInput](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERINPUT)
    """
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)

class FolderContext(FolderNameField, FolderGuidField, Context):
    """Context fields for understanding a folder by an LLM.
    FolderContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Folder§FolderContext](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERCONTEXT)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Folder§FolderOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDEROUTPUT)
    """
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)

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
    table=True,
):
    """Folder entity for hierarchical content organization.
    Folder MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Folder§Folder](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDER)
    """
    PLURAL = "folders"
    __tablename__ = "folder"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    kitPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kit.id")),
        default=None,
        exclude=True,
    )
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
    """GraphQL input node for folder mutations.
    FolderInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Folder§FolderInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/FOLDER/FOLDERINPUTNODE)
    """
    class Meta:
        model = FolderInput

# endregion Folder

# region Benchmark

# [🔖semio/py/semio.py#Benchmark](semiorepo://section/semio/py/semio.py/BENCHMARK)
# Benchmark entity for defining performance metrics with min-max bounds.

class BenchmarkNameField(RealField, abc.ABC):
    """Field mixin for the name of a benchmark.
    BenchmarkNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Benchmark§BenchmarkNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARKNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class BenchmarkIconField(RealField, abc.ABC):
    """Field mixin for the icon of a benchmark.
    BenchmarkIconField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Benchmark§BenchmarkIconField](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARKICONFIELD)
    """
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class BenchmarkMinField(RealField, abc.ABC):
    """Field mixin for the min of a benchmark.
    BenchmarkMinField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Benchmark§BenchmarkMinField](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARKMINFIELD)
    """
    min: typing.Optional[float] = sqlmodel.Field(default=None)

class BenchmarkMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a benchmark.
    BenchmarkMinExcludedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Benchmark§BenchmarkMinExcludedField](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARKMINEXCLUDEDFIELD)
    """
    min_excluded: bool = sqlmodel.Field(default=False)

class BenchmarkMaxField(RealField, abc.ABC):
    """Field mixin for the max of a benchmark.
    BenchmarkMaxField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Benchmark§BenchmarkMaxField](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARKMAXFIELD)
    """
    max: typing.Optional[float] = sqlmodel.Field(default=None)

class BenchmarkMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a benchmark.
    BenchmarkMaxExcludedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Benchmark§BenchmarkMaxExcludedField](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARKMAXEXCLUDEDFIELD)
    """
    max_excluded: bool = sqlmodel.Field(default=False)

class BenchmarkId(BenchmarkNameField, Id):
    """Identity fields for uniquely identifying a benchmark.
    BenchmarkId MUST contain all fields that uniquely identify a benchmark.
    [🛠️semio/py/semio.py#Domain#Benchmark§BenchmarkId](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARKID)
    """
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
    """Property fields for a benchmark.
    BenchmarkProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Benchmark§BenchmarkProps](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARKPROPS)
    """
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
    """Input fields for creating or updating a benchmark.
    BenchmarkInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Benchmark§BenchmarkInput](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARKINPUT)
    """
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
    """Output fields returned when fetching a benchmark.
    BenchmarkOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Benchmark§BenchmarkOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARKOUTPUT)
    """
    pass

class Benchmark(
    BenchmarkMaxExcludedField,
    BenchmarkMaxField,
    BenchmarkMinExcludedField,
    BenchmarkMinField,
    BenchmarkIconField,
    BenchmarkNameField,
    TableEntity,
    table=True,
):
    """Benchmark entity for performance metrics with min-max bounds.
    Benchmark MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Benchmark§Benchmark](semiorepo://definition/semio/py/semio.py/DOMAIN/BENCHMARK/BENCHMARK)
    """
    PLURAL = "benchmarks"
    __tablename__ = "benchmark"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    qualityPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("quality_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("quality.id")),
        default=None,
        exclude=True,
    )
    quality: Quality = sqlmodel.Relationship(back_populates="benchmarks")
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="benchmark", cascade_delete=True)

# endregion Benchmark

# region Quality

# [🔖semio/py/semio.py#Quality](semiorepo://section/semio/py/semio.py/QUALITY)
# Quality entity for defining measurable properties with units and constraints.

class QualityKeyField(RealField, abc.ABC):
    """Field mixin for the key of a quality.
    QualityKeyField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityKeyField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYKEYFIELD)
    """
    key: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT, primary_key=True)

class QualityNameField(RealField, abc.ABC):
    """Field mixin for the name of a quality.
    QualityNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class QualityDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a quality.
    QualityDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class QualityUriField(RealField, abc.ABC):
    """Field mixin for the uri of a quality.
    QualityUriField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityUriField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYURIFIELD)
    """
    uri: str = sqlmodel.Field(default="", max_length=URI_LENGTH_LIMIT)

class QualityScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a quality.
    QualityScalableField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityScalableField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYSCALABLEFIELD)
    """
    scalable: bool = sqlmodel.Field(default=False)

class QualityKindField(RealField, abc.ABC):
    """Field mixin for the kind of a quality.
    QualityKindField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityKindField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYKINDFIELD)
    """
    kind: int = sqlmodel.Field(default=0)

class QualitySiField(RealField, abc.ABC):
    """Field mixin for the si of a quality.
    QualitySiField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualitySiField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYSIFIELD)
    """
    si: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class QualityImperialField(RealField, abc.ABC):
    """Field mixin for the imperial of a quality.
    QualityImperialField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityImperialField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYIMPERIALFIELD)
    """
    imperial: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class QualityMinField(RealField, abc.ABC):
    """Field mixin for the min of a quality.
    QualityMinField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityMinField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYMINFIELD)
    """
    min: typing.Optional[float] = sqlmodel.Field(default=None)

class QualityMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a quality.
    QualityMinExcludedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityMinExcludedField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYMINEXCLUDEDFIELD)
    """
    min_excluded: bool = sqlmodel.Field(default=True)

class QualityMaxField(RealField, abc.ABC):
    """Field mixin for the max of a quality.
    QualityMaxField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityMaxField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYMAXFIELD)
    """
    max: typing.Optional[float] = sqlmodel.Field(default=None)

class QualityMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a quality.
    QualityMaxExcludedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityMaxExcludedField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYMAXEXCLUDEDFIELD)
    """
    max_excluded: bool = sqlmodel.Field(default=True)

class QualityDefaultField(RealField, abc.ABC):
    """Field mixin for the default of a quality.
    QualityDefaultField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityDefaultField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYDEFAULTFIELD)
    """
    default: typing.Optional[float] = sqlmodel.Field(default=None)

class QualityFormulaField(RealField, abc.ABC):
    """Field mixin for the formula of a quality.
    QualityFormulaField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityFormulaField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYFORMULAFIELD)
    """
    formula: str = sqlmodel.Field(default="", max_length=EXPRESSION_LENGTH_LIMIT)

class QualityFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a quality.
    QualityFolderField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityFolderField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYFOLDERFIELD)
    """
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)

class QualityIconField(RealField, abc.ABC):
    """Field mixin for the icon of a quality.
    QualityIconField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityIconField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYICONFIELD)
    """
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class QualityImageField(RealField, abc.ABC):
    """Field mixin for the image of a quality.
    QualityImageField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityImageField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYIMAGEFIELD)
    """
    image: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class QualityUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a quality.
    QualityUnitField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityUnitField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYUNITFIELD)
    """
    unit: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)

class QualityCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a quality.
    QualityCreatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityCreatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYCREATEDFIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class QualityUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a quality.
    QualityUpdatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Quality§QualityUpdatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYUPDATEDFIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class QualityId(QualityKeyField, Id):
    """Identity fields for uniquely identifying a quality.
    QualityId MUST contain all fields that uniquely identify a quality.
    [🛠️semio/py/semio.py#Domain#Quality§QualityId](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYID)
    """
    pass

class QualityProps(
    """Property fields for a quality.
    QualityProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Quality§QualityProps](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYPROPS)
    """
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
    """Input fields for creating or updating a quality.
    QualityInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Quality§QualityInput](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYINPUT)
    """
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
    """Context fields for understanding a quality by an LLM.
    QualityContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Quality§QualityContext](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYCONTEXT)
    """
    pass

class QualityOutput(
    """Output fields returned when fetching a quality.
    QualityOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Quality§QualityOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITYOUTPUT)
    """
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
    """Quality entity with units, constraints, formula and folder classification.
    Quality MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Quality§Quality](semiorepo://definition/semio/py/semio.py/DOMAIN/QUALITY/QUALITY)
    """
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
    __tablename__ = "quality"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    kitPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kit.id")),
        default=None,
        exclude=True,
    )
    kit: Kit = sqlmodel.Relationship(back_populates="qualities")

    benchmarks: list["Benchmark"] = sqlmodel.Relationship(back_populates="quality", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="quality", cascade_delete=True)

    __table_args__ = (
        sqlalchemy.CheckConstraint("kind >= 0 AND kind <= 63", name="ck_qualities_kind_range"),
        sqlalchemy.UniqueConstraint("key", "kit_id", name="uq_qualities_key_kit_id"),
    )

# endregion Quality

# region Prop

# [🔖semio/py/semio.py#Prop](semiorepo://section/semio/py/semio.py/PROP)
# Prop entity for key-value property pairs with units.

class PropKeyField(RealField, abc.ABC):
    """Field mixin for the key of a prop.
    PropKeyField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Prop§PropKeyField](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROPKEYFIELD)
    """
    key: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class PropValueField(RealField, abc.ABC):
    """Field mixin for the value of a prop.
    PropValueField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Prop§PropValueField](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROPVALUEFIELD)
    """
    value: str = sqlmodel.Field(max_length=VALUE_LENGTH_LIMIT)

class PropUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a prop.
    PropUnitField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Prop§PropUnitField](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROPUNITFIELD)
    """
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class PropCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a prop.
    PropCreatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Prop§PropCreatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROPCREATEDFIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class PropUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a prop.
    PropUpdatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Prop§PropUpdatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROPUPDATEDFIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class PropId(PropKeyField, Id):
    """Identity fields for uniquely identifying a prop.
    PropId MUST contain all fields that uniquely identify a prop.
    [🛠️semio/py/semio.py#Domain#Prop§PropId](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROPID)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Prop§PropProps](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROPPROPS)
    """
    pass

class PropInput(PropUnitField, PropValueField, PropKeyField, Input):
    """Input fields for creating or updating a prop.
    PropInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Prop§PropInput](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROPINPUT)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Prop§PropOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROPOUTPUT)
    """
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)

class Prop(
    PropUpdatedField,
    PropCreatedField,
    PropUnitField,
    PropValueField,
    PropKeyField,
    TableEntity,
    table=True,
):
    """Prop entity for key-value properties with optional units.
    Prop MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Prop§Prop](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROP)
    """
    PLURAL = "props"
    __tablename__ = "prop"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    connectorPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("connector_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("connector.id")),
        default=None,
        exclude=True,
    )
    connector: Connector = sqlmodel.Relationship(back_populates="props")
    typePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("type.id")),
        default=None,
        exclude=True,
    )
    type: Type = sqlmodel.Relationship(back_populates="props")
    designPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("design.id")),
        default=None,
        exclude=True,
    )
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
    """GraphQL input node for prop mutations.
    PropInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Prop§PropInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/PROP/PROPINPUTNODE)
    """
    class Meta:
        model = PropInput

# endregion Prop

# region Model

# [🔖semio/py/semio.py#Model](semiorepo://section/semio/py/semio.py/MODEL)
# Model entity for 3D geometry representations linked to files.

class ModelNameField(RealField, abc.ABC):
    """Field mixin for the name of a model.
    ModelNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Model§ModelNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELNAMEFIELD)
    """
    name: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)

class ModelUrlField(RealField, abc.ABC):
    """Field mixin for the url of a model.
    ModelUrlField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Model§ModelUrlField](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELURLFIELD)
    """
    url: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT)

class ModelFileField(RealField, abc.ABC):
    """Field mixin for the file of a model.
    ModelFileField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Model§ModelFileField](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELFILEFIELD)
    """
    file: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class ModelDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a model.
    ModelDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Model§ModelDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class ModelTagsField(MaskedField, abc.ABC):
    """Field mixin for the tags of a model.
    ModelTagsField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Model§ModelTagsField](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELTAGSFIELD)
    """
    tags: list[str] = sqlmodel.Field(default_factory=list)

class ModelId(ModelTagsField, Id):
    """Identity fields for uniquely identifying a model.
    ModelId MUST contain all fields that uniquely identify a model.
    [🛠️semio/py/semio.py#Domain#Model§ModelId](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELID)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Model§ModelProps](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELPROPS)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Model§ModelInput](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELINPUT)
    """
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)

class ModelContext(ModelTagsField, ModelDescriptionField, ModelNameField, Context):
    """Context fields for understanding a model by an LLM.
    ModelContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Model§ModelContext](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELCONTEXT)
    """
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)

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
    [🛠️semio/py/semio.py#Domain#Model§ModelOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELOUTPUT)
    """
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)

class Model(
    ModelDescriptionField,
    ModelNameField,
    ModelFileField,
    ModelUrlField,
    TableEntity,
    table=True,
):
    """Model entity for 3D geometry with name, URL and file reference.
    Model MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Model§Model](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODEL)
    """
    PLURAL = "models"
    __tablename__ = "model"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    tags_: list[Tag] = sqlmodel.Relationship(back_populates="model", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="model", cascade_delete=True)
    typePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("type.id")),
        default=None,
        exclude=True,
    )
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
        except (KeyError, AttributeError, Exception):
            pass
        try:
            entity.attributes = [typing.cast(Attribute, Attribute.parse(attribute)) for attribute in obj["attributes"]]
        except KeyError:
            pass
        return entity

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
    [🛠️semio/py/semio.py#Domain#Model§NoModelAssigned](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/NOMODELASSIGNED)
    """
    def __str__(self):
        return " The entity has no parent model assigned."

class ModelInputNode(InputNode):
    """GraphQL input node for model mutations.
    ModelInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Model§ModelInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/MODEL/MODELINPUTNODE)
    """
    class Meta:
        model = ModelInput

# endregion Model

# region Port

# [🔖semio/py/semio.py#Port](semiorepo://section/semio/py/semio.py/PORT)
# Port entity for defining connection interfaces on types.

class PortNameField(RealField, abc.ABC):
    """Field mixin for the name of a port.
    PortNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Port§PortNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/PORT/PORTNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class PortDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a port.
    PortDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Port§PortDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/PORT/PORTDESCRIPTIONFIELD)
    """
    description: typing.Optional[str] = sqlmodel.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)

class PortIconField(RealField, abc.ABC):
    """Field mixin for the icon of a port.
    PortIconField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Port§PortIconField](semiorepo://definition/semio/py/semio.py/DOMAIN/PORT/PORTICONFIELD)
    """
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class PortCompatiblePortsField(MaskedField, abc.ABC):
    """Field mixin for the compatible ports of a port.
    PortCompatiblePortsField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Port§PortCompatiblePortsField](semiorepo://definition/semio/py/semio.py/DOMAIN/PORT/PORTCOMPATIBLEPORTSFIELD)
    """
    compatiblePorts: list[str] = sqlmodel.Field(default_factory=list)

class PortId(PortNameField, Id):
    """Identity fields for uniquely identifying a port.
    PortId MUST contain all fields that uniquely identify a port.
    [🛠️semio/py/semio.py#Domain#Port§PortId](semiorepo://definition/semio/py/semio.py/DOMAIN/PORT/PORTID)
    """
    pass

class PortProps(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Props):
    """Property fields for a port.
    PortProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Port§PortProps](semiorepo://definition/semio/py/semio.py/DOMAIN/PORT/PORTPROPS)
    """
    pass

class PortInput(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Input):
    """Input fields for creating or updating a port.
    PortInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Port§PortInput](semiorepo://definition/semio/py/semio.py/DOMAIN/PORT/PORTINPUT)
    """
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)

class PortOutput(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Output):
    """Output fields returned when fetching a port.
    PortOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Port§PortOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/PORT/PORTOUTPUT)
    """
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)

class Port(PortIconField, PortDescriptionField, PortNameField, TableEntity, table=True):
    """Port entity defining a named connection interface on a type.
    Port MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Port§Port](semiorepo://definition/semio/py/semio.py/DOMAIN/PORT/PORT)
    """
    PLURAL = "ports"
    __tablename__ = "port"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="port_", cascade_delete=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kit.id")),
        default=None,
        exclude=True,
    )
    kit: Kit = sqlmodel.Relationship(back_populates="ports")

# TODO: Fix PortNode - was incorrectly changed to TableEntityNode in latest commit

class PortInputNode(InputNode):
    """GraphQL input node for port mutations.
    PortInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Port§PortInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/PORT/PORTINPUTNODE)
    """
    class Meta:
        model = PortInput

# endregion Port

# region Connector

# [🔖semio/py/semio.py#Connector](semiorepo://section/semio/py/semio.py/CONNECTOR)

# region CompatiblePort

# [🔖semio/py/semio.py#CompatiblePort](semiorepo://section/semio/py/semio.py/COMPATIBLEPORT)
# Compatible port entity for specifying allowed port pairings on connectors.

class CompatiblePortNameField(RealField, abc.ABC):
    """Field mixin for the name of a compatible port.
    CompatiblePortNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connector#CompatiblePort§CompatiblePortNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/COMPATIBLEPORT/COMPATIBLEPORTNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class CompatiblePortOrderField(RealField, abc.ABC):
    """Field mixin for the order of a compatible port.
    CompatiblePortOrderField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connector#CompatiblePort§CompatiblePortOrderField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/COMPATIBLEPORT/COMPATIBLEPORTORDERFIELD)
    """
    order: int = sqlmodel.Field()

class CompatiblePort(CompatiblePortOrderField, CompatiblePortNameField, Table, table=True):
    """Compatible port entity specifying an allowed port pairing.
    CompatiblePort MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Connector#CompatiblePort§CompatiblePort](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/COMPATIBLEPORT/COMPATIBLEPORT)
    """
    __tablename__ = "compatible_port"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    connectorPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("connector_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("connector.id")),
        default=None,
        exclude=True,
    )
    connector: Connector = sqlmodel.Relationship(back_populates="compatiblePorts_")

# endregion CompatiblePort

class ConnectorIdField(MaskedField, abc.ABC):
    """Field mixin for the id of a connector.
    ConnectorIdField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorIdField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORIDFIELD)
    """
    id_: str = sqlmodel.Field(default="", max_length=ID_LENGTH_LIMIT)

class ConnectorDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a connector.
    ConnectorDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class ConnectorMandatoryField(RealField, abc.ABC):
    """Field mixin for the mandatory of a connector.
    ConnectorMandatoryField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorMandatoryField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORMANDATORYFIELD)
    """
    is_mandatory: bool = sqlmodel.Field(default=False)

class ConnectorPortField(RealField, abc.ABC):
    """Field mixin for the port of a connector.
    ConnectorPortField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorPortField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORPORTFIELD)
    """
    port: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class ConnectorCompatiblePortsField(MaskedField, abc.ABC):
    """Field mixin for the compatible ports of a connector.
    ConnectorCompatiblePortsField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorCompatiblePortsField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORCOMPATIBLEPORTSFIELD)
    """
    compatiblePorts: list[str] = sqlmodel.Field(default_factory=list)

class ConnectorPointField(MaskedField, abc.ABC):
    """Field mixin for the point of a connector.
    ConnectorPointField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorPointField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORPOINTFIELD)
    """
    point: Point = sqlmodel.Field()

class ConnectorDirectionField(MaskedField, abc.ABC):
    """Field mixin for the direction of a connector.
    ConnectorDirectionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorDirectionField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORDIRECTIONFIELD)
    """
    direction: Vector = sqlmodel.Field()

class ConnectorTField(RealField, abc.ABC):
    """Field mixin for the t of a connector.
    ConnectorTField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorTField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORTFIELD)
    """
    t: float = sqlmodel.Field(default=0.0)

class ConnectorId(ConnectorIdField, Id):
    """Identity fields for uniquely identifying a connector.
    ConnectorId MUST contain all fields that uniquely identify a connector.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorId](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORID)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorProps](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORPROPS)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorInput](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORINPUT)
    """
    point: PointInput = sqlmodel.Field()
    direction: VectorInput = sqlmodel.Field()
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)

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
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorContext](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORCONTEXT)
    """
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)

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
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTOROUTPUT)
    """
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)

class Connector(
    ConnectorTField,
    ConnectorPortField,
    ConnectorMandatoryField,
    ConnectorDescriptionField,
    TableEntity,
    table=True,
):
    """Connector entity defining a localized connection point on a type.
    Connector MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Connector§Connector](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTOR)
    """
    PLURAL = "connectors"
    __tablename__ = "connector"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )

    id_: str = sqlmodel.Field(
        sa_column=sqlmodel.Column("local_id", sqlalchemy.String(ID_LENGTH_LIMIT)),
        default="",
    )
    compatiblePorts_: list[CompatiblePort] = sqlmodel.Relationship(back_populates="connector", cascade_delete=True)
    pointX: float = sqlmodel.Field(
        sa_column=sqlmodel.Column("point_x", sqlalchemy.String(ID_LENGTH_LIMIT)),
        exclude=True,
    )
    pointY: float = sqlmodel.Field(sa_column=sqlmodel.Column("point_y", sqlalchemy.Float()), exclude=True)
    pointZ: float = sqlmodel.Field(sa_column=sqlmodel.Column("point_z", sqlalchemy.Float()), exclude=True)
    directionX: float = sqlmodel.Field(sa_column=sqlmodel.Column("direction_x", sqlalchemy.Float()), exclude=True)
    directionY: float = sqlmodel.Field(sa_column=sqlmodel.Column("direction_y", sqlalchemy.Float()), exclude=True)
    directionZ: float = sqlmodel.Field(sa_column=sqlmodel.Column("direction_z", sqlalchemy.Float()), exclude=True)
    attributes: list["Attribute"] = sqlmodel.Relationship(back_populates="connector", cascade_delete=True)
    props: list["Prop"] = sqlmodel.Relationship(back_populates="connector", cascade_delete=True)
    typePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("type.id")),
        default=None,
        exclude=True,
    )
    type: Type = sqlmodel.Relationship(back_populates="connectors")
    connecteds: list["Connection"] = sqlmodel.Relationship(
        back_populates="connectedConnector",
        sa_relationship_kwargs={"foreign_keys": "Connection.connectedConnectorPk"},
    )
    connectings: list["Connection"] = sqlmodel.Relationship(
        back_populates="connectingConnector",
        sa_relationship_kwargs={"foreign_keys": "Connection.connectingConnectorPk"},
    )

    __table_args__ = (sqlalchemy.UniqueConstraint("local_id", "type_id", name="uq_connectors_local_id_type_id"),)

    @property
    def compatiblePorts(self) -> list[str]:
        return sorted([cf.name for cf in self.compatiblePorts_], key=lambda cf: cf.order)

    @compatiblePorts.setter
    def compatiblePorts(self, compatiblePorts: list[str]):
        self.compatiblePorts_ = [CompatiblePort(name=cf, order=i) for i, cf in enumerate(compatiblePorts)]

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
    def parse(cls, input: str | dict | ConnectorInput | typing.Any | None) -> "Connector":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        port_obj = obj.get("port")
        port_guid = port_obj.get("guid") if isinstance(port_obj, dict) else port_obj if isinstance(port_obj, str) else None
        entity = cls(
            id_=obj.get("id_", obj.get("name", "")),
            description=obj.get("description", ""),
            is_mandatory=obj.get("mandatory", False),
            port=port_guid,
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
        entity = {**ConnectorProps.model_validate(self).model_dump()}
        entity["point"] = self.point.dump()
        entity["direction"] = self.direction.dump()
        entity["compatiblePorts"] = self.compatiblePorts
        entity["attributes"] = [q.dump() for q in self.attributes]
        return ConnectorOutput(**entity)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return self.id_

class ConnectorNotFound(NotFound):
    """Exception for a connector not found on a type.
    ConnectorNotFound MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorNotFound](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORNOTFOUND)
    """
    def __init__(self, parent: "Type", id: "ConnectorId") -> None:
        self.parent = parent
        self.id = id

    def __str__(self):
        variant = f", {self.parent.variant}" if self.parent.variant else ""
        return f"Couldn't find the connector ({self.id.id_}) inside the parent type ({self.parent.name}{variant})."

class ConnectorInputNode(InputNode):
    """GraphQL input node for connector mutations.
    ConnectorInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORINPUTNODE)
    """
    class Meta:
        model = ConnectorInput

class ConnectorIdInputNode(InputNode):
    """GraphQL input node for connector id mutations.
    ConnectorIdInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Connector§ConnectorIdInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTOR/CONNECTORIDINPUTNODE)
    """
    class Meta:
        model = ConnectorId

# endregion Connector

# region Type

# [🔖semio/py/semio.py#Type](semiorepo://section/semio/py/semio.py/TYPE)
# Type entity for defining reusable parametric building blocks.

class TypeNameField(RealField, abc.ABC):
    """Field mixin for the name of a type.
    TypeNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPENAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class TypeDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a type.
    TypeDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class TypeIconField(RealField, abc.ABC):
    """Field mixin for the icon of a type.
    TypeIconField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeIconField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEICONFIELD)
    """
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class TypeImageField(RealField, abc.ABC):
    """Field mixin for the image of a type.
    TypeImageField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeImageField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEIMAGEFIELD)
    """
    image: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class TypeVariantField(RealField, abc.ABC):
    """Field mixin for the variant of a type.
    TypeVariantField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeVariantField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEVARIANTFIELD)
    """
    variant: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class TypeParentField(RealField, abc.ABC):
    """Field mixin for the parent of a type.
    TypeParentField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeParentField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEPARENTFIELD)
    """
    parent: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class TypeIsAbstractField(RealField, abc.ABC):
    """Field mixin for the is abstract of a type.
    TypeIsAbstractField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeIsAbstractField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEISABSTRACTFIELD)
    """
    is_abstract: bool = sqlmodel.Field(default=False)

class TypeFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a type.
    TypeFolderField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeFolderField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEFOLDERFIELD)
    """
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class TypeStockField(RealField, abc.ABC):
    """Field mixin for the stock of a type.
    TypeStockField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeStockField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPESTOCKFIELD)
    """
    stock: int = sqlmodel.Field(default=2147483647)

class TypeVirtualField(RealField, abc.ABC):
    """Field mixin for the virtual of a type.
    TypeVirtualField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeVirtualField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEVIRTUALFIELD)
    """
    is_virtual: bool = sqlmodel.Field(default=False)

class TypeScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a type.
    TypeScalableField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeScalableField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPESCALABLEFIELD)
    """
    can_scale: bool = sqlmodel.Field(default=True)

class TypeMirrborableField(RealField, abc.ABC):
    """Field mixin for the mirrborable of a type.
    TypeMirrborableField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeMirrborableField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEMIRRBORABLEFIELD)
    """
    can_mirror: bool = sqlmodel.Field(default=True)

class TypeUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a type.
    TypeUnitField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeUnitField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEUNITFIELD)
    """
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class TypeLocationField(MaskedField, abc.ABC):
    """Field mixin for the location of a type.
    TypeLocationField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeLocationField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPELOCATIONFIELD)
    """
    location: typing.Optional[Location] = sqlmodel.Field(default=None)

class TypeCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a type.
    TypeCreatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeCreatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPECREATEDFIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class TypeUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a type.
    TypeUpdatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Type§TypeUpdatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEUPDATEDFIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class TypeId(TypeVariantField, TypeNameField, Id):
    """Identity fields for uniquely identifying a type.
    TypeId MUST contain all fields that uniquely identify a type.
    [🛠️semio/py/semio.py#Domain#Type§TypeId](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEID)
    """
    pass

class TypeProps(
    TypeUnitField,
    TypeLocationField,
    TypeFolderField,
    TypeIsAbstractField,
    TypeParentField,
    TypeVirtualField,
    TypeStockField,
    TypeVariantField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Props,
):
    """Property fields for a type.
    TypeProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Type§TypeProps](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEPROPS)
    """
    pass

class TypeInput(
    TypeUnitField,
    TypeVirtualField,
    TypeStockField,
    TypeVariantField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Input,
):
    """Input fields for creating or updating a type.
    TypeInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Type§TypeInput](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEINPUT)
    """
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

class TypeOutput(
    TypeUpdatedField,
    TypeCreatedField,
    TypeUnitField,
    TypeVirtualField,
    TypeStockField,
    TypeVariantField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Output,
):
    """Output fields returned when fetching a type.
    TypeOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Type§TypeOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEOUTPUT)
    """
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
    [🛠️semio/py/semio.py#Domain#Type§TypeContext](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPECONTEXT)
    """
    location: typing.Optional[LocationContext] = sqlmodel.Field(default=None)
    connectors: list[ConnectorContext] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)

class Type(
    """Type entity defining a reusable parametric building block.
    Type MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Type§Type](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPE)
    """
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
    __tablename__ = "type"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )

    locationLongitude: typing.Optional[float] = sqlmodel.Field(
        sa_column=sqlmodel.Column("location_longitude", sqlalchemy.Float()),
        exclude=True,
        default=None,
    )

    locationLatitude: typing.Optional[float] = sqlmodel.Field(
        sa_column=sqlmodel.Column("location_latitude", sqlalchemy.Float()),
        exclude=True,
        default=None,
    )

    models: list[Model] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    connectors: list[Connector] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    props: list["Prop"] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    artifact_authors: list[ArtifactAuthor] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    kitPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kit.id")),
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
    def parse(cls, input: str | dict | TypeInput | typing.Any | None) -> "Type":
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
        except (KeyError, AttributeError, Exception):
            pass
        try:
            connectors = [Connector.parse(p) for p in obj["connectors"]]
            entity.connectors = connectors
        except (KeyError, AttributeError, Exception):
            pass
        try:
            props = [Prop.parse(p) for p in obj["props"]]
            entity.props = props
        except (KeyError, AttributeError, Exception):
            pass
        try:
            entity.attributes = [Attribute.parse(q) for q in obj["attributes"]]
        except (KeyError, AttributeError, Exception):
            pass
        try:
            author_emails = obj["authors"]
            entity.authors = author_emails
        except (KeyError, AttributeError, Exception):
            pass
        try:
            concepts = obj["concepts"]
            entity.concepts = concepts
        except (KeyError, AttributeError, Exception):
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
    """Exception for a type not found in the kit.
    TypeNotFound MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Domain#Type§TypeNotFound](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPENOTFOUND)
    """
    def __init__(self, id: "TypeId") -> None:
        self.id = id

    def __str__(self):
        variant = f", {self.id.variant}" if self.id.variant else ""
        return f"Couldn't find the type ({self.id.name}{variant})."

class NoTypeAssigned(NoParentAssigned):
    """No Type Assigned definition.
    NoTypeAssigned MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Domain#Type§NoTypeAssigned](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/NOTYPEASSIGNED)
    """
    def __str__(self):
        return " The entity has no parent type assigned."

class TypeHasNotAllUsedConnectors(SpecificationError):
    """Type Has Not All Used Connectors definition.
    TypeHasNotAllUsedConnectors MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Domain#Type§TypeHasNotAllUsedConnectors](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEHASNOTALLUSEDCONNECTORS)
    """
    def __init__(self, missingConnectors: set[str]) -> None:
        self.missingConnectors = missingConnectors

    def __str__(self) -> str:
        return f" A design is using some connectors of the type. The new type is missing the following connectors: {', '.join(self.missingConnectors)}."

class TypeInputNode(InputNode):
    """GraphQL input node for type mutations.
    TypeInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Type§TypeInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEINPUTNODE)
    """
    class Meta:
        model = TypeInput

class TypeIdInputNode(InputNode):
    """GraphQL input node for type id mutations.
    TypeIdInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Type§TypeIdInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/TYPE/TYPEIDINPUTNODE)
    """
    class Meta:
        model = TypeId

# endregion Type

# region Layer

# [🔖semio/py/semio.py#Layer](semiorepo://section/semio/py/semio.py/LAYER)
# Layer entity for organizing design elements into visibility groups.

class LayerNameField(RealField, abc.ABC):
    """Field mixin for the name of a layer.
    LayerNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Layer§LayerNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/LAYER/LAYERNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class LayerDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a layer.
    LayerDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Layer§LayerDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/LAYER/LAYERDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class LayerColorField(RealField, abc.ABC):
    """Field mixin for the color of a layer.
    LayerColorField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Layer§LayerColorField](semiorepo://definition/semio/py/semio.py/DOMAIN/LAYER/LAYERCOLORFIELD)
    """
    color: str = sqlmodel.Field(default="", max_length=7)

class LayerIsHiddenField(RealField, abc.ABC):
    """Field mixin for the is hidden of a layer.
    LayerIsHiddenField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Layer§LayerIsHiddenField](semiorepo://definition/semio/py/semio.py/DOMAIN/LAYER/LAYERISHIDDENFIELD)
    """
    is_hidden: bool = sqlmodel.Field(default=False)

class LayerIsLockedField(RealField, abc.ABC):
    """Field mixin for the is locked of a layer.
    LayerIsLockedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Layer§LayerIsLockedField](semiorepo://definition/semio/py/semio.py/DOMAIN/LAYER/LAYERISLOCKEDFIELD)
    """
    is_locked: bool = sqlmodel.Field(default=False)

class LayerId(LayerNameField, Id):
    """Identity fields for uniquely identifying a layer.
    LayerId MUST contain all fields that uniquely identify a layer.
    [🛠️semio/py/semio.py#Domain#Layer§LayerId](semiorepo://definition/semio/py/semio.py/DOMAIN/LAYER/LAYERID)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Layer§LayerProps](semiorepo://definition/semio/py/semio.py/DOMAIN/LAYER/LAYERPROPS)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Layer§LayerInput](semiorepo://definition/semio/py/semio.py/DOMAIN/LAYER/LAYERINPUT)
    """
    pass

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
    [🛠️semio/py/semio.py#Domain#Layer§LayerOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/LAYER/LAYEROUTPUT)
    """
    pass

class Layer(
    LayerIsLockedField,
    LayerIsHiddenField,
    LayerColorField,
    LayerDescriptionField,
    LayerNameField,
    TableEntity,
    table=True,
):
    """Layer entity for grouping design elements with visibility and locking.
    Layer MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Layer§Layer](semiorepo://definition/semio/py/semio.py/DOMAIN/LAYER/LAYER)
    """
    PLURAL = "layers"
    __tablename__ = "layer"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    designPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("design.id")),
        default=None,
        exclude=True,
    )
    design: Design = sqlmodel.Relationship(back_populates="layers")

# endregion Layer

# region Piece

# [🔖semio/py/semio.py#Piece](semiorepo://section/semio/py/semio.py/PIECE)
# Piece entity for placed instances of types within a design.

class PieceIdField(MaskedField, abc.ABC):
    """Field mixin for the id of a piece.
    PieceIdField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PieceIdField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEIDFIELD)
    """
    id_: str = sqlmodel.Field(
        default="",
        max_length=ID_LENGTH_LIMIT,
    )

class PieceDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a piece.
    PieceDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PieceDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class PieceTypeField(MaskedField, abc.ABC):
    """Field mixin for the type of a piece.
    PieceTypeField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PieceTypeField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECETYPEFIELD)
    """
    type: typing.Optional[TypeId] = sqlmodel.Field(default=None)

class PieceDesignField(MaskedField, abc.ABC):
    """Field mixin for the design of a piece.
    PieceDesignField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PieceDesignField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEDESIGNFIELD)
    """
    designPiece: typing.Optional["DesignId"] = sqlmodel.Field(default=None)

class PiecePlaneField(MaskedField, abc.ABC):
    """Field mixin for the plane of a piece.
    PiecePlaneField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PiecePlaneField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEPLANEFIELD)
    """
    plane: typing.Optional[Plane] = sqlmodel.Field(default=None)

class PieceCenterField(MaskedField, abc.ABC):
    """Field mixin for the center of a piece.
    PieceCenterField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PieceCenterField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECECENTERFIELD)
    """
    center: typing.Optional[Coord] = sqlmodel.Field(default=None)

class PieceScaleField(RealField, abc.ABC):
    """Field mixin for the scale of a piece.
    PieceScaleField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PieceScaleField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECESCALEFIELD)
    """
    scale: float = sqlmodel.Field(default=1.0)

class PieceMirrorPlaneField(MaskedField, abc.ABC):
    """Field mixin for the mirror plane of a piece.
    PieceMirrorPlaneField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PieceMirrorPlaneField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEMIRRORPLANEFIELD)
    """
    mirrorPlane: typing.Optional[Plane] = sqlmodel.Field(default=None)

class PieceHiddenField(RealField, abc.ABC):
    """Field mixin for the hidden of a piece.
    PieceHiddenField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PieceHiddenField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEHIDDENFIELD)
    """
    is_hidden: bool = sqlmodel.Field(default=False)

class PieceLockedField(RealField, abc.ABC):
    """Field mixin for the locked of a piece.
    PieceLockedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PieceLockedField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECELOCKEDFIELD)
    """
    is_locked: bool = sqlmodel.Field(default=False)

class PieceColorField(RealField, abc.ABC):
    """Field mixin for the color of a piece.
    PieceColorField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Piece§PieceColorField](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECECOLORFIELD)
    """
    color: str = sqlmodel.Field(default="", max_length=7)

class PieceId(PieceIdField, Id):
    """Identity fields for uniquely identifying a piece.
    PieceId MUST contain all fields that uniquely identify a piece.
    [🛠️semio/py/semio.py#Domain#Piece§PieceId](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEID)
    """
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
    """Property fields for a piece.
    PieceProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Piece§PieceProps](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEPROPS)
    """
    pass

class PieceInput(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Input):
    """Input fields for creating or updating a piece.
    PieceInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Piece§PieceInput](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEINPUT)
    """
    plane: typing.Optional[PlaneInput] = sqlmodel.Field(default=None)
    center: typing.Optional[CoordInput] = sqlmodel.Field(default=None)
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)

class PieceContext(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Context):
    """Context fields for understanding a piece by an LLM.
    PieceContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Piece§PieceContext](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECECONTEXT)
    """
    plane: typing.Optional[PlaneContext] = sqlmodel.Field(default=None)
    center: typing.Optional[CoordContext] = sqlmodel.Field(default=None)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)

class PieceOutput(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Output):
    """Output fields returned when fetching a piece.
    PieceOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Piece§PieceOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEOUTPUT)
    """
    plane: typing.Optional[PlaneOutput] = sqlmodel.Field(default=None)
    center: typing.Optional[CoordOutput] = sqlmodel.Field(default=None)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)

class PiecePrediction(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Prediction):
    """Prediction fields for LLM-based piece inference.
    PiecePrediction MUST contain all fields for LLM inference.
    [🛠️semio/py/semio.py#Domain#Piece§PiecePrediction](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEPREDICTION)
    """
    pass

class Piece(
    PieceIdField,
    PieceHiddenField,
    PieceLockedField,
    PieceColorField,
    PieceScaleField,
    TableEntity,
    table=True,
):
    """Piece entity for a placed instance of a type within a design.
    Piece MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Piece§Piece](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECE)
    """
    PLURAL = "pieces"
    __tablename__ = "piece"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    id_: str = sqlmodel.Field(
        sa_column=sqlmodel.Column("local_id", sqlalchemy.String(ID_LENGTH_LIMIT)),
        default="",
    )
    typePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "type_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
            nullable=True,
        ),
        default=None,
        exclude=True,
    )
    type: Type = sqlmodel.Relationship(back_populates="pieces")
    designPiecePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "design_piece_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("design.id"),
            nullable=True,
        ),
        default=None,
        exclude=True,
    )
    designPiece: Design = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Design", foreign_keys="[Piece.designPiecePk]"))
    designPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("design.id")),
        default=None,
        exclude=True,
    )
    design: Design = sqlmodel.Relationship(
        back_populates="pieces",
        sa_relationship_kwargs={"foreign_keys": "[Piece.designPk]"},
    )
    planePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "plane_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("plane.id"),
            nullable=True,
        ),
        default=None,
        exclude=True,
    )
    plane: Plane = sqlmodel.Relationship(back_populates="piece")
    centerU: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("center_x", sqlalchemy.Float()), exclude=True)
    centerV: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("center_y", sqlalchemy.Float()), exclude=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="piece", cascade_delete=True)
    connecteds: list["Connection"] = sqlmodel.Relationship(
        back_populates="connectedPiece",
        sa_relationship_kwargs={"foreign_keys": "Connection.connectedPiecePk"},
    )
    connectings: list["Connection"] = sqlmodel.Relationship(
        back_populates="connectingPiece",
        sa_relationship_kwargs={"foreign_keys": "Connection.connectingPiecePk"},
    )

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
    def parse(
        cls: "Piece",
        input: str | dict | PieceInput | typing.Any | None,
        types: dict[str, dict[str, Type]],
        designs: typing.Optional[dict[str, dict[str, dict[str, "Design"]]]] = None,
    ) -> "Piece":
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
    """GraphQL input node for piece mutations.
    PieceInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Piece§PieceInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEINPUTNODE)
    """
    class Meta:
        model = PieceInput
        exclude_fields = ("type", "designPiece")

    type = TypeIdInputNode()
    designPiece = graphene.Field(lambda: DesignIdInputNode)

class PieceIdInputNode(InputNode):
    """GraphQL input node for piece id mutations.
    PieceIdInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Piece§PieceIdInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/PIECE/PIECEIDINPUTNODE)
    """
    class Meta:
        model = PieceId

# endregion Piece

# region Group

# [🔖semio/py/semio.py#Group](semiorepo://section/semio/py/semio.py/GROUP)
# Group entity for named collections of pieces in a design.

class GroupNameField(RealField, abc.ABC):
    """Field mixin for the name of a group.
    GroupNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Group§GroupNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/GROUP/GROUPNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class GroupDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a group.
    GroupDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Group§GroupDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/GROUP/GROUPDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class GroupColorField(RealField, abc.ABC):
    """Field mixin for the color of a group.
    GroupColorField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Group§GroupColorField](semiorepo://definition/semio/py/semio.py/DOMAIN/GROUP/GROUPCOLORFIELD)
    """
    color: str = sqlmodel.Field(default="", max_length=7)

class GroupId(GroupNameField, Id):
    """Identity fields for uniquely identifying a group.
    GroupId MUST contain all fields that uniquely identify a group.
    [🛠️semio/py/semio.py#Domain#Group§GroupId](semiorepo://definition/semio/py/semio.py/DOMAIN/GROUP/GROUPID)
    """
    pass

class GroupProps(GroupColorField, GroupDescriptionField, GroupNameField, Props):
    """Property fields for a group.
    GroupProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Group§GroupProps](semiorepo://definition/semio/py/semio.py/DOMAIN/GROUP/GROUPPROPS)
    """
    pass

class GroupInput(GroupColorField, GroupDescriptionField, GroupNameField, Input):
    """Input fields for creating or updating a group.
    GroupInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Group§GroupInput](semiorepo://definition/semio/py/semio.py/DOMAIN/GROUP/GROUPINPUT)
    """
    pass

class GroupOutput(GroupColorField, GroupDescriptionField, GroupNameField, Output):
    """Output fields returned when fetching a group.
    GroupOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Group§GroupOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/GROUP/GROUPOUTPUT)
    """
    pieces: list["PieceOutput"] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)

class Group(GroupColorField, GroupDescriptionField, GroupNameField, TableEntity, table=True):
    """Group entity for named collections of pieces.
    Group MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Group§Group](semiorepo://definition/semio/py/semio.py/DOMAIN/GROUP/GROUP)
    """
    PLURAL = "groups"
    __tablename__ = "group"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    designPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("design.id")),
        default=None,
        exclude=True,
    )
    design: Design = sqlmodel.Relationship(back_populates="groups")

# endregion Group

# region Side

# [🔖semio/py/semio.py#Side](semiorepo://section/semio/py/semio.py/SIDE)
# Side primitive for identifying a specific connector on a specific piece.

class Side(BaseModel):
    """Side primitive identifying a specific connector on a specific piece.
    Side MUST contain all coordinate or geometry fields.
    [🛠️semio/py/semio.py#Domain#Side§Side](semiorepo://definition/semio/py/semio.py/DOMAIN/SIDE/SIDE)
    """
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
    """Input fields for creating or updating a side.
    SideInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Side§SideInput](semiorepo://definition/semio/py/semio.py/DOMAIN/SIDE/SIDEINPUT)
    """
    pass

class SideContext(Side, Context):
    """Context fields for understanding a side by an LLM.
    SideContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Side§SideContext](semiorepo://definition/semio/py/semio.py/DOMAIN/SIDE/SIDECONTEXT)
    """
    pass

class SideOutput(Side, Output):
    """Output fields returned when fetching a side.
    SideOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Side§SideOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/SIDE/SIDEOUTPUT)
    """
    pass

class SidePrediction(Side, Prediction):
    """Prediction fields for LLM-based side inference.
    SidePrediction MUST contain all fields for LLM inference.
    [🛠️semio/py/semio.py#Domain#Side§SidePrediction](semiorepo://definition/semio/py/semio.py/DOMAIN/SIDE/SIDEPREDICTION)
    """
    pass

class SideNode(Node):
    """GraphQL node exposing side data.
    SideNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Side§SideNode](semiorepo://definition/semio/py/semio.py/DOMAIN/SIDE/SIDENODE)
    """
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
    """GraphQL input node for side mutations.
    SideInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Side§SideInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/SIDE/SIDEINPUTNODE)
    """
    class Meta:
        model = SideInput

    exclude_fields = ("piece", "connector")

    piece = graphene.NonNull(PieceIdInputNode)
    designPiece = PieceIdInputNode()
    connector = graphene.NonNull(ConnectorIdInputNode)

# endregion Side

# region Connection

# [🔖semio/py/semio.py#Connection](semiorepo://section/semio/py/semio.py/CONNECTION)
# Connection entity for linking two pieces through their connectors.

class ConnectionConnectedField(MaskedField, abc.ABC):
    """Field mixin for the connected of a connection.
    ConnectionConnectedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionConnectedField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONCONNECTEDFIELD)
    """
    connected: Side = sqlmodel.Field()

class ConnectionConnectingField(MaskedField, abc.ABC):
    """Field mixin for the connecting of a connection.
    ConnectionConnectingField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionConnectingField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONCONNECTINGFIELD)
    """
    connecting: Side = sqlmodel.Field()

class ConnectionDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a connection.
    ConnectionDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class ConnectionGapField(RealField, abc.ABC):
    """Field mixin for the gap of a connection.
    ConnectionGapField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionGapField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONGAPFIELD)
    """
    gap: float = sqlmodel.Field(default=0)

class ConnectionShiftField(RealField, abc.ABC):
    """Field mixin for the shift of a connection.
    ConnectionShiftField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionShiftField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONSHIFTFIELD)
    """
    shift: float = sqlmodel.Field(default=0)

class ConnectionRiseField(MaskedField, abc.ABC):
    """Field mixin for the rise of a connection.
    ConnectionRiseField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionRiseField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONRISEFIELD)
    """
    rise: float = sqlmodel.Field(default=0)

class ConnectionRotationField(RealField, abc.ABC):
    """Field mixin for the rotation of a connection.
    ConnectionRotationField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionRotationField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONROTATIONFIELD)
    """
    rotation: float = sqlmodel.Field(ge=0, lt=360, default=0)

class ConnectionTurnField(RealField, abc.ABC):
    """Field mixin for the turn of a connection.
    ConnectionTurnField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionTurnField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONTURNFIELD)
    """
    turn: float = sqlmodel.Field(ge=0, lt=360, default=0)

class ConnectionTiltField(RealField, abc.ABC):
    """Field mixin for the tilt of a connection.
    ConnectionTiltField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionTiltField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONTILTFIELD)
    """
    tilt: float = sqlmodel.Field(ge=0, lt=360, default=0)

class ConnectionUField(RealField, abc.ABC):
    """Field mixin for the u of a connection.
    ConnectionUField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionUField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONUFIELD)
    """
    u: float = sqlmodel.Field(default=0)

class ConnectionVField(RealField, abc.ABC):
    """Field mixin for the v of a connection.
    ConnectionVField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionVField](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONVFIELD)
    """
    v: float = sqlmodel.Field(default=0)

class ConnectionId(ConnectionConnectedField, ConnectionConnectingField, Id):
    """Identity fields for uniquely identifying a connection.
    ConnectionId MUST contain all fields that uniquely identify a connection.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionId](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONID)
    """
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
    """Property fields for a connection.
    ConnectionProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionProps](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONPROPS)
    """
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
    """Input fields for creating or updating a connection.
    ConnectionInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionInput](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONINPUT)
    """
    pass

    connected: SideInput = sqlmodel.Field()
    connecting: SideInput = sqlmodel.Field()

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
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionContext](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONCONTEXT)
    """
    pass

    connected: SideContext = sqlmodel.Field()
    connecting: SideContext = sqlmodel.Field()

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
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONOUTPUT)
    """
    pass

    connected: SideOutput = sqlmodel.Field()
    connecting: SideOutput = sqlmodel.Field()

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
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionPrediction](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONPREDICTION)
    """
    pass

    connected: SidePrediction = sqlmodel.Field()
    connecting: SidePrediction = sqlmodel.Field()

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
    table=True,
):
    """Connection entity linking two pieces through their connectors.
    Connection MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Connection§Connection](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTION)
    """
    PLURAL = "connections"
    __tablename__ = "connection"

    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    connectedPiecePk: typing.Optional[int] = sqlmodel.Field(
        alias="connectedPieceId",
        sa_column=sqlmodel.Column(
            "connected_piece_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("piece.id"),
        ),
        default=None,
        exclude=True,
    )
    connectedPiece: Piece = sqlmodel.Relationship(
        sa_relationship=sqlalchemy.orm.relationship(
            "Piece",
            back_populates="connecteds",
            foreign_keys="[Connection.connectedPiecePk]",
        )
    )
    connectedConnectorPk: typing.Optional[int] = sqlmodel.Field(
        alias="connectedConnectorId",
        sa_column=sqlmodel.Column(
            "connected_connector_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("connector.id"),
        ),
        default=None,
        exclude=True,
    )
    connectedConnector: Connector = sqlmodel.Relationship(
        sa_relationship=sqlalchemy.orm.relationship(
            "Connector",
            back_populates="connecteds",
            foreign_keys="[Connection.connectedConnectorPk]",
        )
    )
    connectedDesignPiecePk: typing.Optional[int] = sqlmodel.Field(
        alias="connectedDesignPieceId",
        sa_column=sqlmodel.Column(
            "connected_design_piece_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("piece.id"),
            nullable=True,
        ),
        default=None,
        exclude=True,
    )
    connectedDesignPiece: Piece = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Piece", foreign_keys="[Connection.connectedDesignPiecePk]"))
    connectingPiecePk: typing.Optional[int] = sqlmodel.Field(
        alias="connectingPieceId",
        sa_column=sqlmodel.Column(
            "connecting_piece_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("piece.id"),
        ),
        exclude=True,
        default=None,
    )
    connectingPiece: Piece = sqlmodel.Relationship(
        sa_relationship=sqlalchemy.orm.relationship(
            "Piece",
            back_populates="connectings",
            foreign_keys="[Connection.connectingPiecePk]",
        )
    )
    connectingConnectorPk: typing.Optional[int] = sqlmodel.Field(
        alias="connectingConnectorId",
        sa_column=sqlmodel.Column(
            "connecting_connector_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("connector.id"),
        ),
        default=None,
        exclude=True,
    )
    connectingConnector: Connector = sqlmodel.Relationship(
        sa_relationship=sqlalchemy.orm.relationship(
            "Connector",
            back_populates="connectings",
            foreign_keys="[Connection.connectingConnectorPk]",
        )
    )
    connectingDesignPiecePk: typing.Optional[int] = sqlmodel.Field(
        alias="connectingDesignPieceId",
        sa_column=sqlmodel.Column(
            "connecting_design_piece_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("piece.id"),
            nullable=True,
        ),
        default=None,
        exclude=True,
    )
    connectingDesignPiece: Piece = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Piece", foreign_keys="[Connection.connectingDesignPiecePk]"))
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="connection", cascade_delete=True)
    designPk: typing.Optional[int] = sqlmodel.Field(
        alias="designId",
        sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("design.id")),
        default=None,
        exclude=True,
    )
    design: Design = sqlmodel.Relationship(back_populates="connections")
    __table_args__ = (
        sqlalchemy.UniqueConstraint(
            "connected_piece_id",
            "connected_design_piece_id",
            "connecting_piece_id",
            "connecting_design_piece_id",
            name="uq_connections_connected_piece_id_connected_design_piece_id_connecting_piece_id_connecting_design_piece_id",
        ),
        sqlalchemy.CheckConstraint(
            "connected_piece_id != connecting_piece_id",
            name="ck_connections_not_reflexive",
        ),
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
    def parse(
        cls: "Connection",
        input: str | dict | ConnectionInput | typing.Any | None,
        pieces: list[Piece],
        designsById: typing.Optional[dict[str, dict[str, dict[str, Design]]]] = None,
    ) -> "Connection":
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
    """GraphQL input node for connection mutations.
    ConnectionInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Connection§ConnectionInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/CONNECTION/CONNECTIONINPUTNODE)
    """
    class Meta:
        model = ConnectionInput

# endregion Connection

# region Stat

# [🔖semio/py/semio.py#Stat](semiorepo://section/semio/py/semio.py/STAT)
# Stat entity for recording computed statistics with bounds.

class StatKeyField(RealField, abc.ABC):
    """Field mixin for the key of a stat.
    StatKeyField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Stat§StatKeyField](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATKEYFIELD)
    """
    key: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class StatUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a stat.
    StatUnitField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Stat§StatUnitField](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATUNITFIELD)
    """
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class StatMinField(RealField, abc.ABC):
    """Field mixin for the min of a stat.
    StatMinField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Stat§StatMinField](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATMINFIELD)
    """
    min: typing.Optional[float] = sqlmodel.Field(default=None)

class StatMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a stat.
    StatMinExcludedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Stat§StatMinExcludedField](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATMINEXCLUDEDFIELD)
    """
    min_excluded: bool = sqlmodel.Field(default=False)

class StatMaxField(RealField, abc.ABC):
    """Field mixin for the max of a stat.
    StatMaxField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Stat§StatMaxField](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATMAXFIELD)
    """
    max: typing.Optional[float] = sqlmodel.Field(default=None)

class StatMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a stat.
    StatMaxExcludedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Stat§StatMaxExcludedField](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATMAXEXCLUDEDFIELD)
    """
    max_excluded: bool = sqlmodel.Field(default=False)

class StatCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a stat.
    StatCreatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Stat§StatCreatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATCREATEDFIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class StatUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a stat.
    StatUpdatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Stat§StatUpdatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATUPDATEDFIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class StatId(StatKeyField, Id):
    """Identity fields for uniquely identifying a stat.
    StatId MUST contain all fields that uniquely identify a stat.
    [🛠️semio/py/semio.py#Domain#Stat§StatId](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATID)
    """
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
    """Property fields for a stat.
    StatProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Stat§StatProps](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATPROPS)
    """
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
    """Input fields for creating or updating a stat.
    StatInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Stat§StatInput](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATINPUT)
    """
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
    """Output fields returned when fetching a stat.
    StatOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Stat§StatOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STATOUTPUT)
    """
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
    table=True,
):
    """Stat entity for recording computed statistics with bounds.
    Stat MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Stat§Stat](semiorepo://definition/semio/py/semio.py/DOMAIN/STAT/STAT)
    """
    PLURAL = "stats"
    __tablename__ = "stat"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    designPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("design.id")),
        default=None,
        exclude=True,
    )
    design: Design = sqlmodel.Relationship(back_populates="stats")

# endregion Stat

# region Design

# [🔖semio/py/semio.py#Design](semiorepo://section/semio/py/semio.py/DESIGN)
# Design entity for composing pieces and connections into assemblies.

class DesignNameField(RealField, abc.ABC):
    """Field mixin for the name of a design.
    DesignNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class DesignDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a design.
    DesignDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class DesignIconField(RealField, abc.ABC):
    """Field mixin for the icon of a design.
    DesignIconField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignIconField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNICONFIELD)
    """
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class DesignImageField(RealField, abc.ABC):
    """Field mixin for the image of a design.
    DesignImageField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignImageField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNIMAGEFIELD)
    """
    image: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class DesignVariantField(RealField, abc.ABC):
    """Field mixin for the variant of a design.
    DesignVariantField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignVariantField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNVARIANTFIELD)
    """
    variant: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class DesignViewField(RealField, abc.ABC):
    """Field mixin for the view of a design.
    DesignViewField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignViewField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNVIEWFIELD)
    """
    view: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class DesignParentField(RealField, abc.ABC):
    """Field mixin for the parent of a design.
    DesignParentField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignParentField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNPARENTFIELD)
    """
    parent: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class DesignIsAbstractField(RealField, abc.ABC):
    """Field mixin for the is abstract of a design.
    DesignIsAbstractField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignIsAbstractField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNISABSTRACTFIELD)
    """
    is_abstract: bool = sqlmodel.Field(default=False)

class DesignFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a design.
    DesignFolderField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignFolderField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNFOLDERFIELD)
    """
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class DesignActiveLayerField(RealField, abc.ABC):
    """Field mixin for the active layer of a design.
    DesignActiveLayerField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignActiveLayerField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNACTIVELAYERFIELD)
    """
    activeLayer: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class DesignLocationField(MaskedField, abc.ABC):
    """Field mixin for the location of a design.
    DesignLocationField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignLocationField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNLOCATIONFIELD)
    """
    location: typing.Optional[Location] = sqlmodel.Field(default=None)

class DesignUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a design.
    DesignUnitField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignUnitField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNUNITFIELD)
    """
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class DesignScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a design.
    DesignScalableField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignScalableField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNSCALABLEFIELD)
    """
    can_scale: bool = sqlmodel.Field(default=True)

class DesignMirrorableField(RealField, abc.ABC):
    """Field mixin for the mirrorable of a design.
    DesignMirrorableField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignMirrorableField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNMIRRORABLEFIELD)
    """
    can_mirror: bool = sqlmodel.Field(default=True)

class DesignCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a design.
    DesignCreatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignCreatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNCREATEDFIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class DesignUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a design.
    DesignUpdatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Design§DesignUpdatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNUPDATEDFIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class DesignId(DesignNameField, DesignVariantField, Id):
    """Identity fields for uniquely identifying a design.
    DesignId MUST contain all fields that uniquely identify a design.
    [🛠️semio/py/semio.py#Domain#Design§DesignId](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNID)
    """
    pass

class DesignProps(
    DesignUnitField,
    DesignViewField,
    DesignActiveLayerField,
    DesignFolderField,
    DesignIsAbstractField,
    DesignParentField,
    DesignLocationField,
    DesignVariantField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Props,
):
    """Property fields for a design.
    DesignProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Design§DesignProps](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNPROPS)
    """
    pass

class DesignInput(
    DesignUnitField,
    DesignViewField,
    DesignVariantField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Input,
):
    """Input fields for creating or updating a design.
    DesignInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Design§DesignInput](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNINPUT)
    """
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

class DesignContext(
    DesignUnitField,
    DesignViewField,
    DesignVariantField,
    DesignDescriptionField,
    DesignNameField,
    Context,
):
    """Context fields for understanding a design by an LLM.
    DesignContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Design§DesignContext](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNCONTEXT)
    """
    pass

    location: typing.Optional[LocationContext] = sqlmodel.Field(default=None)
    pieces: list[PieceContext] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionContext] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)

class DesignOutput(
    DesignUpdatedField,
    DesignCreatedField,
    DesignUnitField,
    DesignViewField,
    DesignVariantField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Output,
):
    """Output fields returned when fetching a design.
    DesignOutput MUST contain all fields returned on fetch.
    [🛠️semio/py/semio.py#Domain#Design§DesignOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNOUTPUT)
    """
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
    """Prediction fields for LLM-based design inference.
    DesignPrediction MUST contain all fields for LLM inference.
    [🛠️semio/py/semio.py#Domain#Design§DesignPrediction](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNPREDICTION)
    """
    pass

    pieces: list[PiecePrediction] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionPrediction] = sqlmodel.Field(default_factory=list)

class Design(
    """Design entity composing pieces and connections into an assembly.
    Design MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Design§Design](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGN)
    """
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
    __tablename__ = "design"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    concepts_: list[Concept] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    artifact_authors: list[ArtifactAuthor] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    locationLongitude: typing.Optional[float] = sqlmodel.Field(
        sa_column=sqlmodel.Column("location_longitude", sqlalchemy.Float()),
        exclude=True,
        default=None,
    )
    locationLatitude: typing.Optional[float] = sqlmodel.Field(
        sa_column=sqlmodel.Column("location_latitude", sqlalchemy.Float()),
        exclude=True,
        default=None,
    )
    layers: list[Layer] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    pieces: list[Piece] = sqlmodel.Relationship(
        back_populates="design",
        cascade_delete=True,
        sa_relationship_kwargs={"foreign_keys": "[Piece.designPk]"},
    )
    groups: list[Group] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    connections: list[Connection] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    stats: list[Stat] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    props: list["Prop"] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(
        alias="kitId",
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kit.id")),
        default=None,
        exclude=True,
    )
    kit: Kit = sqlmodel.Relationship(back_populates="designs")

    __table_args__ = (
        sqlalchemy.UniqueConstraint(
            "name",
            "variant",
            "view",
            "kit_id",
            name="uq_designs_name_variant_view_kit_id",
        ),
    )

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
    def parse(
        cls: "Design",
        input: str | dict | DesignInput | typing.Any | None,
        types: list[Type],
        designsById: typing.Optional[dict[str, dict[str, dict[str, "Design"]]]] = None,
    ) -> "Design":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        props = DesignProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            entity.location = props.location
        except (KeyError, AttributeError, Exception):
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
        except (KeyError, AttributeError, Exception):
            pass
        try:
            connections = [Connection.parse(c, pieces, designsById) for c in obj["connections"]]
            entity.connections = connections
        except (KeyError, AttributeError, Exception):
            pass
        try:
            props = [Prop.parse(p) for p in obj["props"]]
            entity.props = props
        except (KeyError, AttributeError, Exception):
            pass
        try:
            attributes = [Attribute.parse(q) for q in obj["attributes"]]
            entity.attributes = attributes
        except (KeyError, AttributeError, Exception):
            pass
        try:
            author_emails = obj["authors"]
            entity.authors = author_emails
        except (KeyError, AttributeError, Exception):
            pass
        try:
            concepts = obj["concepts"]
            entity.concepts = concepts
        except (KeyError, AttributeError, Exception):
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
    """No Design Assigned definition.
    NoDesignAssigned MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Domain#Design§NoDesignAssigned](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/NODESIGNASSIGNED)
    """
    def __str__(self):
        return "👪 The entity has no parent design assigned."

class DesignInputNode(InputNode):
    """GraphQL input node for design mutations.
    DesignInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Design§DesignInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNINPUTNODE)
    """
    class Meta:
        model = DesignInput

class DesignIdInputNode(InputNode):
    """GraphQL input node for design id mutations.
    DesignIdInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain#Design§DesignIdInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/DESIGN/DESIGNIDINPUTNODE)
    """
    class Meta:
        model = DesignId

# endregion Design

# region Kit

# [🔖semio/py/semio.py#Kit](semiorepo://section/semio/py/semio.py/KIT)
# Kit entity for packaging types, designs, qualities and metadata.

class KitUriField(RealField, abc.ABC):
    """Field mixin for the uri of a kit.
    KitUriField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitUriField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITURIFIELD)
    """
    uri: str = sqlmodel.Field(max_length=URI_LENGTH_LIMIT)

class KitNameField(RealField, abc.ABC):
    """Field mixin for the name of a kit.
    KitNameField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitNameField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITNAMEFIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class KitDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a kit.
    KitDescriptionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitDescriptionField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITDESCRIPTIONFIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class KitIconField(RealField, abc.ABC):
    """Field mixin for the icon of a kit.
    KitIconField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitIconField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITICONFIELD)
    """
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitImageField(RealField, abc.ABC):
    """Field mixin for the image of a kit.
    KitImageField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitImageField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITIMAGEFIELD)
    """
    image: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitPreviewField(RealField, abc.ABC):
    """Field mixin for the preview of a kit.
    KitPreviewField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitPreviewField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITPREVIEWFIELD)
    """
    preview: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitVersionField(RealField, abc.ABC):
    """Field mixin for the version of a kit.
    KitVersionField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitVersionField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITVERSIONFIELD)
    """
    version: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class KitRemoteField(RealField, abc.ABC):
    """Field mixin for the remote of a kit.
    KitRemoteField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitRemoteField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITREMOTEFIELD)
    """
    remote: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitHomepageField(RealField, abc.ABC):
    """Field mixin for the homepage of a kit.
    KitHomepageField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitHomepageField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITHOMEPAGEFIELD)
    """
    homepage: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitLicenseField(RealField, abc.ABC):
    """Field mixin for the license of a kit.
    KitLicenseField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitLicenseField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITLICENSEFIELD)
    """
    license: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a kit.
    KitCreatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitCreatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITCREATEDFIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class KitUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a kit.
    KitUpdatedField MUST declare exactly one field with appropriate constraints.
    [🛠️semio/py/semio.py#Domain#Kit§KitUpdatedField](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITUPDATEDFIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class KitId(KitUriField, Id):
    """Identity fields for uniquely identifying a kit.
    KitId MUST contain all fields that uniquely identify a kit.
    [🛠️semio/py/semio.py#Domain#Kit§KitId](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITID)
    """
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
    """Property fields for a kit.
    KitProps MUST contain all non-relational property fields.
    [🛠️semio/py/semio.py#Domain#Kit§KitProps](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITPROPS)
    """
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
    """Input fields for creating or updating a kit.
    KitInput MUST contain all fields required for creation.
    [🛠️semio/py/semio.py#Domain#Kit§KitInput](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITINPUT)
    """
    pass

    types: list[TypeInput] = sqlmodel.Field(default_factory=list)
    designs: list[DesignInput] = sqlmodel.Field(default_factory=list)
    folders: list[FolderInput] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)

class KitContext(KitDescriptionField, KitNameField, Context):
    """Context fields for understanding a kit by an LLM.
    KitContext MUST contain all fields needed for LLM understanding.
    [🛠️semio/py/semio.py#Domain#Kit§KitContext](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITCONTEXT)
    """
    pass

    types: list[TypeContext] = sqlmodel.Field(default_factory=list)
    designs: list[DesignContext] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)

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
    [🛠️semio/py/semio.py#Domain#Kit§KitOutput](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KITOUTPUT)
    """
    pass

    types: list[TypeOutput] = sqlmodel.Field(default_factory=list)
    designs: list[DesignOutput] = sqlmodel.Field(default_factory=list)
    folders: list[FolderOutput] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)

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
    table=True,
):
    """Kit entity packaging types, designs, qualities and metadata.
    Kit MUST implement idMembers and inherit from the appropriate field mixins.
    [🛠️semio/py/semio.py#Domain#Kit§Kit](semiorepo://definition/semio/py/semio.py/DOMAIN/KIT/KIT)
    """
    PLURAL = "kits"
    __tablename__ = "kit"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    concepts_: list[Concept] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    authors_: list[Author] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    files_: list[File] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    folders_: list[Folder] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    ports: list[Port] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    types: list[Type] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    designs: list[Design] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    qualities: list[Quality] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)

    @property
    def concepts(self: "Kit") -> list[str]:
        if self.concepts_ is None:
            return []
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
        guid = obj.get("guid", str(uuid.uuid4()))
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
        except (KeyError, AttributeError, Exception):
            pass
        try:
            designs = [Design.parse(d, types) for d in obj["designs"]]
            entity.designs = designs
        except (KeyError, AttributeError, Exception):
            pass
        try:
            folders = [Folder.parse(f) for f in obj["folders"]]
            entity.folders = folders
        except (KeyError, AttributeError, Exception):
            pass
        try:
            concepts = obj["concepts"]
            entity.concepts = concepts
        except (KeyError, AttributeError, Exception):
            pass
        return entity

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

# [🔖semio/py/semio.py#Design Family Helpers](semiorepo://section/semio/py/semio.py/DESIGN-FAMILY-HELPERS)
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

# [🔖semio/py/semio.py#Type Family Helpers](semiorepo://section/semio/py/semio.py/TYPE-FAMILY-HELPERS)
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

# [🔖semio/py/semio.py#Moved Graphene Nodes](semiorepo://section/semio/py/semio.py/MOVED-GRAPHENE-NODES)
# Graphene node definitions moved here due to forward-reference resolution order.

class AttributeNode(TableEntityNode):
    """GraphQL node exposing attribute data.
    AttributeNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Moved Graphene Nodes§AttributeNode](semiorepo://definition/semio/py/semio.py/DOMAIN/MOVED-GRAPHENE-NODES/ATTRIBUTENODE)
    """
    class Meta:
        model = Attribute

class PlaneNode(TableNode):
    """GraphQL node exposing plane data.
    PlaneNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Moved Graphene Nodes§PlaneNode](semiorepo://definition/semio/py/semio.py/DOMAIN/MOVED-GRAPHENE-NODES/PLANENODE)
    """
    class Meta:
        model = Plane

class AuthorNode(TableEntityNode):
    """GraphQL node exposing author data.
    AuthorNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Moved Graphene Nodes§AuthorNode](semiorepo://definition/semio/py/semio.py/DOMAIN/MOVED-GRAPHENE-NODES/AUTHORNODE)
    """
    class Meta:
        model = Author

class ModelNode(TableEntityNode):
    """GraphQL node exposing model data.
    ModelNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Moved Graphene Nodes§ModelNode](semiorepo://definition/semio/py/semio.py/DOMAIN/MOVED-GRAPHENE-NODES/MODELNODE)
    """
    class Meta:
        model = Model
        excludedFields = ("tags_",)

class ConnectorNode(TableEntityNode):
    """GraphQL node exposing connector data.
    ConnectorNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Moved Graphene Nodes§ConnectorNode](semiorepo://definition/semio/py/semio.py/DOMAIN/MOVED-GRAPHENE-NODES/CONNECTORNODE)
    """
    class Meta:
        model = Connector
        exclude_fields = ("connecteds", "connectings")

    localId = graphene.String()

    def resolve_localId(self, info):
        return getattr(self, "id_", "")

class TypeNode(TableEntityNode):
    """GraphQL node exposing type data.
    TypeNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Moved Graphene Nodes§TypeNode](semiorepo://definition/semio/py/semio.py/DOMAIN/MOVED-GRAPHENE-NODES/TYPENODE)
    """
    class Meta:
        model = Type

class PieceNode(TableEntityNode):
    """GraphQL node exposing piece data.
    PieceNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Moved Graphene Nodes§PieceNode](semiorepo://definition/semio/py/semio.py/DOMAIN/MOVED-GRAPHENE-NODES/PIECENODE)
    """
    class Meta:
        model = Piece
        exclude_fields = ("connecteds", "connectings")

    localId = graphene.String()

    def resolve_localId(self, info):
        return getattr(self, "id_", "")

class ConnectionNode(TableEntityNode):
    """GraphQL node exposing connection data.
    ConnectionNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Moved Graphene Nodes§ConnectionNode](semiorepo://definition/semio/py/semio.py/DOMAIN/MOVED-GRAPHENE-NODES/CONNECTIONNODE)
    """
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
    """GraphQL node exposing design data.
    DesignNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain#Moved Graphene Nodes§DesignNode](semiorepo://definition/semio/py/semio.py/DOMAIN/MOVED-GRAPHENE-NODES/DESIGNNODE)
    """
    class Meta:
        model = Design

class KitNotFound(NotFound):
    """endregion Moved Graphene Nodes
    KitNotFound MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Domain§KitNotFound](semiorepo://definition/semio/py/semio.py/DOMAIN/KITNOTFOUND)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't find an local or remote kit under uri:\n {self.uri}."

class NoKitToDelete(KitNotFound):
    """No Kit To Delete definition.
    NoKitToDelete MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Domain§NoKitToDelete](semiorepo://definition/semio/py/semio.py/DOMAIN/NOKITTODELETE)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't delete the kit because no local or remote kit was found under uri:\n {self.uri}."

class KitZipDoesNotContainSemioFolder(KitNotFound):
    """Kit Zip Does Not Contain Semio Folder definition.
    KitZipDoesNotContainSemioFolder MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Domain§KitZipDoesNotContainSemioFolder](semiorepo://definition/semio/py/semio.py/DOMAIN/KITZIPDOESNOTCONTAINSEMIOFOLDER)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 The remote zip kit ({self.uri}) is not a valid kit."

class OnlyRemoteKitsCanBeCached(ClientError):
    """Only Remote Kits Can Be Cached definition.
    OnlyRemoteKitsCanBeCached MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Domain§OnlyRemoteKitsCanBeCached](semiorepo://definition/semio/py/semio.py/DOMAIN/ONLYREMOTEKITSCANBECACHED)
    """
    def __init__(self, nonRemoteUri: str) -> None:
        self.nonRemoteUri = nonRemoteUri

    def __str__(self):
        return f"🔍 Only remote kits can be cached. The uri ({self.nonRemoteUri}) doesn't start with http and ends with .zip"

class KitUriNotValid(ClientError, abc.ABC):
    """🆔 The base for all kit uri not valid errors.
    KitUriNotValid MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Domain§KitUriNotValid](semiorepo://definition/semio/py/semio.py/DOMAIN/KITURINOTVALID)
    """

class LocalKitUriNotValid(KitUriNotValid, abc.ABC):
    """📂 The base for all local kit uri not valid errors.
    LocalKitUriNotValid MUST be subclassed and MUST NOT be instantiated directly.
    [🛠️semio/py/semio.py#Domain§LocalKitUriNotValid](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCALKITURINOTVALID)
    """

class LocalKitUriIsNotAbsolute(LocalKitUriNotValid):
    """Local Kit Uri Is Not Absolute definition.
    LocalKitUriIsNotAbsolute MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Domain§LocalKitUriIsNotAbsolute](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCALKITURIISNOTABSOLUTE)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"📂 The local kit uri ({self.uri}) is relative. It needs to be absolute (include the parent folders, drives, ...)."

class LocalKitUriIsNotDirectory(LocalKitUriNotValid):
    """Local Kit Uri Is Not Directory definition.
    LocalKitUriIsNotDirectory MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Domain§LocalKitUriIsNotDirectory](semiorepo://definition/semio/py/semio.py/DOMAIN/LOCALKITURIISNOTDIRECTORY)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"📂 The local kit uri ({self.uri}) is not a directory."

class NoKitAssigned(NoParentAssigned):
    """No Kit Assigned definition.
    NoKitAssigned MUST fulfill its documented contract.
    [🛠️semio/py/semio.py#Domain§NoKitAssigned](semiorepo://definition/semio/py/semio.py/DOMAIN/NOKITASSIGNED)
    """
    def __str__(self):
        return "👪 The entity has no parent kit assigned."

class KitAlreadyExists(AlreadyExists, abc.ABC):
    """Exception for attempting to create a kit that already exists.
    KitAlreadyExists MUST provide a descriptive error message via __str__.
    [🛠️semio/py/semio.py#Domain§KitAlreadyExists](semiorepo://definition/semio/py/semio.py/DOMAIN/KITALREADYEXISTS)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self) -> str:
        return f"♊ A kit under uri ({self.uri}) already exists."

class KitInputNode(InputNode):
    """GraphQL input node for kit mutations.
    KitInputNode MUST expose the input model via Meta.
    [🛠️semio/py/semio.py#Domain§KitInputNode](semiorepo://definition/semio/py/semio.py/DOMAIN/KITINPUTNODE)
    """
    class Meta:
        model = KitInput

class KitNode(TableEntityNode):
    """GraphQL node exposing kit data.
    KitNode MUST expose the model via Meta.
    [🛠️semio/py/semio.py#Domain§KitNode](semiorepo://definition/semio/py/semio.py/DOMAIN/KITNODE)
    """
    class Meta:
        model = Kit

# #endregion 🔖Moved Graphene Nodes

# region Validation

# [🔖semio/py/semio.py#Validation](semiorepo://section/semio/py/semio.py/VALIDATION)
# Validation logic for checking kit constraints and uniqueness rules.

@dataclasses.dataclass
class ValidationFix:
    """A proposed fix for a validation problem with a title and diff.
    ValidationFix MUST contain a non-empty title and a valid diff dictionary.
    [🛠️semio/py/semio.py#Validation§ValidationFix](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATIONFIX)
    """
    title: str
    diff: dict

    def toDict(self) -> dict:
        return {"title": self.title, "diff": self.diff}

@dataclasses.dataclass
class Problem:
    """A validation problem with a constraint identifier and message.
    Problem MUST contain a non-empty constraint identifier and message.
    [🛠️semio/py/semio.py#Validation§Problem](semiorepo://definition/semio/py/semio.py/VALIDATION/PROBLEM)
    """
    constraintId: str
    message: str
    entityKind: str
    entityGuid: str
    fixes: list[ValidationFix] = dataclasses.field(default_factory=list)

    def toDict(self) -> dict:
        return {
            "constraintId": self.constraintId,
            "message": self.message,
            "entityKind": self.entityKind,
            "entityGuid": self.entityGuid,
            "fixes": [f.toDict() for f in self.fixes],
        }

@dataclasses.dataclass
class ValidationResult:
    """A validation result aggregating problems and fixes for an entity.
    ValidationResult MUST aggregate all problems and fixes for a single entity.
    [🛠️semio/py/semio.py#Validation§ValidationResult](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATIONRESULT)
    """
    problems: list[Problem]

    def hasErrors(self) -> bool:
        return len(self.problems) > 0

    def toDict(self) -> dict:
        sortedProblems = sorted(self.problems, key=lambda i: (i.constraintId, i.entityGuid))
        return {"problems": [i.toDict() for i in sortedProblems]}

    def serialize(self) -> str:
        return json.dumps(self.toDict(), indent=2)

def _isGuid(s: str) -> bool:
    import re

    return bool(
        re.match(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
            s,
            re.IGNORECASE,
        )
    )

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
    """Check whether two validation results are semantically equal.
    areValidationResultsEqual MUST compare all problems and fixes.
    [🛠️semio/py/semio.py#Validation§areValidationResultsEqual](semiorepo://definition/semio/py/semio.py/VALIDATION/AREVALIDATIONRESULTSEQUAL)
    """
    if len(a.problems) != len(b.problems):
        return False
    sortedA = sorted(a.problems, key=lambda i: (i.constraintId, i.entityGuid))
    sortedB = sorted(b.problems, key=lambda i: (i.constraintId, i.entityGuid))
    for ia, ib in zip(sortedA, sortedB):
        if ia.constraintId != ib.constraintId or ia.message != ib.message or ia.entityKind != ib.entityKind or ia.entityGuid != ib.entityGuid:
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
    """Parse a validation result from a dictionary representation.
    parseValidationResult MUST return a ValidationResult from a dict.
    [🛠️semio/py/semio.py#Validation§parseValidationResult](semiorepo://definition/semio/py/semio.py/VALIDATION/PARSEVALIDATIONRESULT)
    """
    data = json.loads(jsonStr)
    problems = []
    for i in data["problems"]:
        fixes = [ValidationFix(title=f["title"], diff=f["diff"]) for f in i.get("fixes", [])]
        problems.append(
            Problem(
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
    [🛠️semio/py/semio.py#Validation§validateGuidUniqueness](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATEGUIDUNIQUENESS)
    """
    problems: list[Problem] = []
    seen: dict[str, str] = {}

    def check(entityKind: str, entityGuid: str) -> None:
        if entityGuid in seen:
            problems.append(
                Problem(
                    constraintId="guid-unique",
                    message=f'Duplicate GUID "{entityGuid}". First occurrence kept.',
                    entityKind=entityKind,
                    entityGuid=entityGuid,
                )
            )
        else:
            seen[entityGuid] = entityKind

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
    return problems

def validateTypeNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all type names within a kit are unique.
    validateTypeNameUniqueness MUST report duplicate type names as problems.
    [🛠️semio/py/semio.py#Validation§validateTypeNameUniqueness](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATETYPENAMEUNIQUENESS)
    """
    problems: list[Problem] = []
    byParent: dict[str | None, list[Type]] = {}
    for t in kit.types or []:
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
                    problems.append(
                        Problem(
                            constraintId="type-name-unique",
                            message=f'Duplicate type name "{name}" among siblings.',
                            entityKind="Type",
                            entityGuid=t.guid,
                        )
                    )
    return problems

def validateDesignNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all design names within a kit are unique.
    validateDesignNameUniqueness MUST report duplicate design names as problems.
    [🛠️semio/py/semio.py#Validation§validateDesignNameUniqueness](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATEDESIGNNAMEUNIQUENESS)
    """
    problems: list[Problem] = []
    byParent: dict[str | None, list[Design]] = {}
    for d in kit.designs or []:
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
                    problems.append(
                        Problem(
                            constraintId="design-name-unique",
                            message=f'Duplicate design name "{name}" among siblings.',
                            entityKind="Design",
                            entityGuid=d.guid,
                        )
                    )
    return problems

def validatePieceNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all piece names within a design are unique.
    validatePieceNameUniqueness MUST report duplicate piece names as problems.
    [🛠️semio/py/semio.py#Validation§validatePieceNameUniqueness](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATEPIECENAMEUNIQUENESS)
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
                    problems.append(
                        Problem(
                            constraintId="piece-name-unique",
                            message=f'Duplicate piece name "{name}" in design.',
                            entityKind="Piece",
                            entityGuid=p.guid,
                        )
                    )
    return problems

def validatePortNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all port names within a type are unique.
    validatePortNameUniqueness MUST report duplicate port names as problems.
    [🛠️semio/py/semio.py#Validation§validatePortNameUniqueness](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATEPORTNAMEUNIQUENESS)
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
                    problems.append(
                        Problem(
                            constraintId="connector-name-unique",
                            message=f'Duplicate connector name "{name}" in type.',
                            entityKind="Connector",
                            entityGuid=connector.guid,
                        )
                    )
    return problems

def validateModelNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all model names within a type are unique.
    validateModelNameUniqueness MUST report duplicate model names as problems.
    [🛠️semio/py/semio.py#Validation§validateModelNameUniqueness](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATEMODELNAMEUNIQUENESS)
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
                    problems.append(
                        Problem(
                            constraintId="model-name-unique",
                            message=f'Duplicate model name "{name}" in type.',
                            entityKind="Model",
                            entityGuid=model.guid,
                        )
                    )
    return problems

def validateQualityNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all quality names within a kit are unique.
    validateQualityNameUniqueness MUST report duplicate quality names as problems.
    [🛠️semio/py/semio.py#Validation§validateQualityNameUniqueness](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATEQUALITYNAMEUNIQUENESS)
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
                problems.append(
                    Problem(
                        constraintId="quality-name-unique",
                        message=f'Duplicate quality name "{name}".',
                        entityKind="Quality",
                        entityGuid=q.guid,
                    )
                )
    return problems

def validateFileNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all file names within a kit are unique.
    validateFileNameUniqueness MUST report duplicate file names as problems.
    [🛠️semio/py/semio.py#Validation§validateFileNameUniqueness](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATEFILENAMEUNIQUENESS)
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
                problems.append(
                    Problem(
                        constraintId="file-name-unique",
                        message=f'Duplicate file name "{name}".',
                        entityKind="File",
                        entityGuid=f.guid,
                    )
                )
    return problems

def validateFolderNameUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all folder names within a kit are unique.
    validateFolderNameUniqueness MUST report duplicate folder names as problems.
    [🛠️semio/py/semio.py#Validation§validateFolderNameUniqueness](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATEFOLDERNAMEUNIQUENESS)
    """
    problems: list[Problem] = []
    byParent: dict[str | None, list[Folder]] = {}
    for fo in kit.folders_ or []:
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
                    problems.append(
                        Problem(
                            constraintId="folder-name-unique",
                            message=f'Duplicate folder name "{name}" among siblings.',
                            entityKind="Folder",
                            entityGuid=fo.guid,
                        )
                    )
    return problems

def validateLayerPathUniqueness(kit: Kit) -> list[Problem]:
    """Validate that all layer paths within a design are unique.
    validateLayerPathUniqueness MUST report duplicate layer paths as problems.
    [🛠️semio/py/semio.py#Validation§validateLayerPathUniqueness](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATELAYERPATHUNIQUENESS)
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
                    problems.append(
                        Problem(
                            constraintId="layer-path-unique",
                            message=f'Duplicate layer path "{path}" in design.',
                            entityKind="Layer",
                            entityGuid=layer.guid,
                        )
                    )
    return problems

def validateKit(kit: Kit) -> ValidationResult:
    """Validate a kit entity against all constraint rules.
    validateKit MUST run all validation checks and return aggregated results.
    [🛠️semio/py/semio.py#Validation§validateKit](semiorepo://definition/semio/py/semio.py/VALIDATION/VALIDATEKIT)
    """
    problems: list[Problem] = []
    problems.extend(validateGuidUniqueness(kit))
    problems.extend(validateTypeNameUniqueness(kit))
    problems.extend(validateDesignNameUniqueness(kit))
    problems.extend(validatePieceNameUniqueness(kit))
    problems.extend(validatePortNameUniqueness(kit))
    problems.extend(validateModelNameUniqueness(kit))
    problems.extend(validateQualityNameUniqueness(kit))
    problems.extend(validateFileNameUniqueness(kit))
    problems.extend(validateFolderNameUniqueness(kit))
    problems.extend(validateLayerPathUniqueness(kit))
    return ValidationResult(problems=problems)

# region Dict-based Validation

# [🔖semio/py/semio.py#Dict-based Validation](semiorepo://section/semio/py/semio.py/DICT-BASED-VALIDATION)
# Dictionary-based validation functions for kit data integrity.

def _makeFix(title: str, diff: dict) -> ValidationFix:
    return ValidationFix(title=title, diff=diff)

def _deepCopy(obj: typing.Any) -> typing.Any:
    return json.loads(json.dumps(obj))

def _newGuid() -> str:
    import uuid

    return str(uuid.uuid4())

def validateKitDict(kit: dict) -> ValidationResult:
    """Validate a kit dictionary against all constraint rules.
    validateKitDict MUST validate a kit dictionary and return results.
    [🛠️semio/py/semio.py#Validation#Dict-based Validation§validateKitDict](semiorepo://definition/semio/py/semio.py/VALIDATION/DICT-BASED-VALIDATION/VALIDATEKITDICT)
    """
    problems: list[Problem] = []
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
                "Port": "ports",
                "File": "files",
                "Folder": "folders",
                "Stat": "stats",
                "Layer": "layers",
            }.get(entityKind, "")
            if collectionKey:
                diff = {
                    collectionKey: {
                        "removed": [{"guid": entityGuid}],
                        "added": [entityCopy],
                    }
                }
                fix = _makeFix("Regenerate GUID", diff)
                problems.append(
                    Problem(
                        constraintId="guid-unique",
                        message=f'Duplicate GUID "{entityGuid}". First occurrence kept.',
                        entityKind=entityKind,
                        entityGuid=entityGuid,
                        fixes=[fix],
                    )
                )
            else:
                problems.append(
                    Problem(
                        constraintId="guid-unique",
                        message=f'Duplicate GUID "{entityGuid}". First occurrence kept.',
                        entityKind=entityKind,
                        entityGuid=entityGuid,
                        fixes=[],
                    )
                )
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
    for i in kit.get("ports", []):
        checkGuid("Port", i.get("guid", ""), i)
    for f in kit.get("files", []):
        checkGuid("File", f.get("guid", ""), f)
    for fo in kit.get("folders", []):
        checkGuid("Folder", fo.get("guid", ""), fo)
    byParent: dict[str | None, list[dict]] = {}
    for t in kit.get("types", []):
        parent = t.get("parent")
        parentGuid = parent.get("guid") if isinstance(parent, dict) else parent if parent else None
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
                    fix = _makeFix(
                        f'Rename "{name}"',
                        {
                            "types": {
                                "updated": [
                                    {
                                        "type": {"guid": t.get("guid", "")},
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
                            entityGuid=t.get("guid", ""),
                            fixes=[fix],
                        )
                    )
    byParent = {}
    for d in kit.get("designs", []):
        parent = d.get("parent")
        parentGuid = parent.get("guid") if isinstance(parent, dict) else parent if parent else None
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
                    fix = _makeFix(
                        f'Rename "{name}"',
                        {
                            "designs": {
                                "updated": [
                                    {
                                        "design": {"guid": d.get("guid", "")},
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
                    fix = _makeFix(
                        f'Rename piece "{name}"',
                        {
                            "designs": {
                                "updated": [
                                    {
                                        "design": {"guid": designGuid},
                                        "diff": {
                                            "pieces": {
                                                "updated": [
                                                    {
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
                    problems.append(
                        Problem(
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
                    fix = _makeFix(
                        f'Rename connector "{name}"',
                        {
                            "types": {
                                "updated": [
                                    {
                                        "type": {"guid": typeGuid},
                                        "diff": {
                                            "connectors": {
                                                "updated": [
                                                    {
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
                    problems.append(
                        Problem(
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
                    fix = _makeFix(
                        f'Rename model "{name}"',
                        {
                            "types": {
                                "updated": [
                                    {
                                        "type": {"guid": typeGuid},
                                        "diff": {
                                            "models": {
                                                "updated": [
                                                    {
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
                    problems.append(
                        Problem(
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
                fix = _makeFix(
                    f'Rename quality "{name}"',
                    {
                        "qualities": {
                            "updated": [
                                {
                                    "quality": {"guid": q.get("guid", "")},
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
                fix = _makeFix(
                    f'Rename port "{name}"',
                    {
                        "ports": {
                            "updated": [
                                {
                                    "port": {"guid": iface.get("guid", "")},
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
                fix = _makeFix(
                    f'Rename file "{name}"',
                    {
                        "files": {
                            "updated": [
                                {
                                    "file": {"guid": f.get("guid", "")},
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
                        entityGuid=f.get("guid", ""),
                        fixes=[fix],
                    )
                )
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
                    fix = _makeFix(
                        f'Rename folder "{name}"',
                        {
                            "folders": {
                                "updated": [
                                    {
                                        "folder": {"guid": fo.get("guid", "")},
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
                    fix = _makeFix(
                        f'Rename layer "{path}"',
                        {
                            "designs": {
                                "updated": [
                                    {
                                        "design": {"guid": designGuid},
                                        "diff": {
                                            "layers": {
                                                "updated": [
                                                    {
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
                    problems.append(
                        Problem(
                            constraintId="layer-path-unique",
                            message=f'Duplicate layer path "{path}" inside design "{designName}".',
                            entityKind="Layer",
                            entityGuid=layer.get("guid", ""),
                            fixes=[fix],
                        )
                    )
    return ValidationResult(problems=problems)

# endregion Dict-based Validation

# endregion Validation

# region Graph Operations

# [🔖semio/py/semio.py#Graph Operations](semiorepo://section/semio/py/semio.py/GRAPH-OPERATIONS)
# Graph construction and traversal for piece connectivity analysis.

def buildPieceGraph(design: Design | dict) -> networkx.Graph:
    """Build a networkx graph from pieces and connections.
    buildPieceGraph MUST return a networkx graph with pieces as nodes.
    [🛠️semio/py/semio.py#Graph Operations§buildPieceGraph](semiorepo://definition/semio/py/semio.py/GRAPH-OPERATIONS/BUILDPIECEGRAPH)
    """
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
    """Find all pieces that are fixed in the design hierarchy.
    findFixedPieces MUST return pieces that have no incoming connections.
    [🛠️semio/py/semio.py#Graph Operations§findFixedPieces](semiorepo://definition/semio/py/semio.py/GRAPH-OPERATIONS/FINDFIXEDPIECES)
    """
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
    """Get connected components of the piece graph.
    getConnectedComponents MUST return disjoint piece groups.
    [🛠️semio/py/semio.py#Graph Operations§getConnectedComponents](semiorepo://definition/semio/py/semio.py/GRAPH-OPERATIONS/GETCONNECTEDCOMPONENTS)
    """
    G = buildPieceGraph(design)
    return [set(c) for c in networkx.connected_components(G)]

def getPieceHierarchy(design: Design | dict, rootGuid: str) -> dict[str, int]:
    """Get the hierarchical ordering of pieces from root to leaf.
    getPieceHierarchy MUST return a topological ordering of pieces.
    [🛠️semio/py/semio.py#Graph Operations§getPieceHierarchy](semiorepo://definition/semio/py/semio.py/GRAPH-OPERATIONS/GETPIECEHIERARCHY)
    """
    G = buildPieceGraph(design)
    if rootGuid not in G:
        return {}
    return networkx.single_source_shortest_path_length(G, rootGuid)

# endregion Graph Operations

# region FlattenDesign

# [🔖semio/py/semio.py#FlattenDesign](semiorepo://section/semio/py/semio.py/FLATTENDESIGN)
# Design flattening to resolve nested sub-designs into a single coordinate space.

def getTypeByGuid(kit: dict, guid: str) -> dict | None:
    """Look up a type by its GUID within a kit dictionary.
    getTypeByGuid MUST return the type dict or raise if not found.
    [🛠️semio/py/semio.py#FlattenDesign§getTypeByGuid](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/GETTYPEBYGUID)
    """
    for t in kit.get("types", []):
        if t.get("guid") == guid:
            return t
    return None

def getConnectorFromType(kit: dict, typeData: dict | None, connectorGuid: str | None) -> dict | None:
    """Look up a connector by name from a type dictionary.
    getConnectorFromType MUST return the matching connector dict.
    [🛠️semio/py/semio.py#FlattenDesign§getConnectorFromType](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/GETCONNECTORFROMTYPE)
    """
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

def planeToMatrixDict(plane: dict) -> numpy.ndarray:
    """Convert a plane dictionary to a 4x4 transformation matrix.
    planeToMatrixDict MUST produce a valid 4x4 homogeneous matrix.
    [🛠️semio/py/semio.py#FlattenDesign§planeToMatrixDict](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/PLANETOMATRIXDICT)
    """
    origin = numpy.array([plane["origin"]["x"], plane["origin"]["y"], plane["origin"]["z"]])
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
    """Convert a 4x4 transformation matrix to a plane dictionary.
    matrixToPlaneDict MUST extract origin, xAxis and yAxis from the matrix.
    [🛠️semio/py/semio.py#FlattenDesign§matrixToPlaneDict](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/MATRIXTOPLANEDICT)
    """
    origin = matrix[:3, 3]
    xAxis = matrix[:3, 0]
    yAxis = matrix[:3, 1]
    return {
        "origin": {"x": float(origin[0]), "y": float(origin[1]), "z": float(origin[2])},
        "xAxis": {"x": float(xAxis[0]), "y": float(xAxis[1]), "z": float(xAxis[2])},
        "yAxis": {"x": float(yAxis[0]), "y": float(yAxis[1]), "z": float(yAxis[2])},
    }

def quaternionFromUnitVectorsDict(vFrom: numpy.ndarray, vTo: numpy.ndarray) -> numpy.ndarray:
    """Compute a quaternion rotating one unit vector onto another.
    quaternionFromUnitVectorsDict MUST compute the shortest rotation quaternion.
    [🛠️semio/py/semio.py#FlattenDesign§quaternionFromUnitVectorsDict](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/QUATERNIONFROMUNITVECTORSDICT)
    """
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
    """Compute a quaternion from an axis-angle representation.
    quaternionFromAxisAngleDict MUST compute the quaternion for the given rotation.
    [🛠️semio/py/semio.py#FlattenDesign§quaternionFromAxisAngleDict](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/QUATERNIONFROMAXISANGLEDICT)
    """
    halfAngle = angle / 2
    s = numpy.sin(halfAngle)
    return numpy.array([axis[0] * s, axis[1] * s, axis[2] * s, numpy.cos(halfAngle)])

def quaternionToMatrixDict(q: numpy.ndarray) -> numpy.ndarray:
    """Convert a quaternion to a 3x3 rotation matrix.
    quaternionToMatrixDict MUST produce a valid 3x3 rotation matrix.
    [🛠️semio/py/semio.py#FlattenDesign§quaternionToMatrixDict](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/QUATERNIONTOMATRIXDICT)
    """
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
    """Create a 4x4 rotation matrix around an arbitrary axis.
    makeRotationAxisDict MUST return a 4x4 rotation matrix around the axis.
    [🛠️semio/py/semio.py#FlattenDesign§makeRotationAxisDict](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/MAKEROTATIONAXISDICT)
    """
    return quaternionToMatrixDict(quaternionFromAxisAngleDict(axis, angle))

def makeTranslationDict(x: float, y: float, z: float) -> numpy.ndarray:
    """Create a 4x4 translation matrix from a displacement vector.
    makeTranslationDict MUST return a 4x4 translation matrix.
    [🛠️semio/py/semio.py#FlattenDesign§makeTranslationDict](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/MAKETRANSLATIONDICT)
    """
    m = numpy.eye(4)
    m[0, 3] = x
    m[1, 3] = y
    m[2, 3] = z
    return m

def applyMatrix4ToVec3Dict(m: numpy.ndarray, v: numpy.ndarray) -> numpy.ndarray:
    """Apply a 4x4 matrix to a 3D vector dictionary.
    applyMatrix4ToVec3Dict MUST apply the full affine transformation.
    [🛠️semio/py/semio.py#FlattenDesign§applyMatrix4ToVec3Dict](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/APPLYMATRIX4TOVEC3DICT)
    """
    return numpy.array(
        [
            m[0, 0] * v[0] + m[0, 1] * v[1] + m[0, 2] * v[2],
            m[1, 0] * v[0] + m[1, 1] * v[1] + m[1, 2] * v[2],
            m[2, 0] * v[0] + m[2, 1] * v[1] + m[2, 2] * v[2],
        ]
    )

def computeChildPlaneDict(parentPlane: dict, parentConnector: dict, childConnector: dict, connection: dict) -> dict:
    """Compute the world-space plane of a child piece from parent and local planes.
    computeChildPlaneDict MUST compose parent and local transformations correctly.
    [🛠️semio/py/semio.py#FlattenDesign§computeChildPlaneDict](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/COMPUTECHILDPLANEDICT)
    """
    parentMatrix = planeToMatrixDict(parentPlane)
    parentPoint = numpy.array([parentConnector["point"]["x"], parentConnector["point"]["y"], parentConnector["point"]["z"]])
    parentDirection = normalizeVector(numpy.array([parentConnector["direction"]["x"], parentConnector["direction"]["y"], parentConnector["direction"]["z"]]))
    childPoint = numpy.array([childConnector["point"]["x"], childConnector["point"]["y"], childConnector["point"]["z"]])
    childDirection = normalizeVector(numpy.array([childConnector["direction"]["x"], childConnector["direction"]["y"], childConnector["direction"]["z"]]))
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
    orientationT = rotateT @ orientationT
    turnAxis = applyMatrix4ToVec3Dict(rotateT, turnAxis)
    tiltAxis = applyMatrix4ToVec3Dict(rotateT, tiltAxis)
    turnT = makeRotationAxisDict(turnAxis, turnRad)
    orientationT = turnT @ orientationT
    tiltT = makeRotationAxisDict(tiltAxis, tiltRad)
    orientationT = tiltT @ orientationT
    centerChildT = makeTranslationDict(-childPoint[0], -childPoint[1], -childPoint[2])
    transform = orientationT @ centerChildT
    gapTransform = makeTranslationDict(gapDirection[0] * gap, gapDirection[1] * gap, gapDirection[2] * gap)
    shiftTransform = makeTranslationDict(shiftDirection[0] * shift, shiftDirection[1] * shift, shiftDirection[2] * shift)
    raiseTransform = makeTranslationDict(raiseDirection[0] * rise, raiseDirection[1] * rise, raiseDirection[2] * rise)
    translationT = raiseTransform @ shiftTransform @ gapTransform
    transform = translationT @ transform
    moveToParentT = makeTranslationDict(parentPoint[0], parentPoint[1], parentPoint[2])
    transform = moveToParentT @ transform
    finalMatrix = parentMatrix @ transform
    result = matrixToPlaneDict(finalMatrix)
    return {
        "origin": {"x": round(result["origin"]["x"] / TOLERANCE) * TOLERANCE, "y": round(result["origin"]["y"] / TOLERANCE) * TOLERANCE, "z": round(result["origin"]["z"] / TOLERANCE) * TOLERANCE},
        "xAxis": {"x": round(result["xAxis"]["x"] / TOLERANCE) * TOLERANCE, "y": round(result["xAxis"]["y"] / TOLERANCE) * TOLERANCE, "z": round(result["xAxis"]["z"] / TOLERANCE) * TOLERANCE},
        "yAxis": {"x": round(result["yAxis"]["x"] / TOLERANCE) * TOLERANCE, "y": round(result["yAxis"]["y"] / TOLERANCE) * TOLERANCE, "z": round(result["yAxis"]["z"] / TOLERANCE) * TOLERANCE},
    }

def flattenDesignDict(kit: dict, designGuid: str) -> dict:
    """Flatten a nested design hierarchy into a single flat coordinate space.
    flattenDesignDict MUST resolve all nested designs into world coordinates.
    [🛠️semio/py/semio.py#FlattenDesign§flattenDesignDict](semiorepo://definition/semio/py/semio.py/FLATTENDESIGN/FLATTENDESIGNDICT)
    """
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
            piecePlanes[rootNode] = {
                "origin": {"x": 0, "y": 0, "z": 0},
                "xAxis": {"x": 1, "y": 0, "z": 0},
                "yAxis": {"x": 0, "y": 1, "z": 0},
            }
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
            radius = 2.697
            verticalVExtra = 1.0
            horizontalScale = 3.0633
            parentCenter = parentPiece.get("center") or {"u": 0, "v": 0}
            connectionU = connection.get("u", 0) or 0
            connectionV = connection.get("v", 0) or 0
            if parentCenter["u"] == 0 and parentCenter["v"] == 0:
                t = parentConnector.get("t", 0) or 0
                angle = 2 * numpy.pi * t
                childU = radius * numpy.sin(angle)
                childV = radius * numpy.cos(angle)
            else:
                parentDirZ = (parentConnector.get("direction") or {}).get("z", 0) or 0
                isVerticalConnection = abs(parentDirZ) > 0.5
                if isVerticalConnection:
                    childU = parentCenter["u"] + connectionU
                    childV = parentCenter["v"] + connectionV + verticalVExtra
                else:
                    childU = parentCenter["u"] + connectionU * horizontalScale
                    childV = parentCenter["v"] + connectionV * horizontalScale
            childCenter = {"u": round(childU / TOLERANCE) * TOLERANCE, "v": round(childV / TOLERANCE) * TOLERANCE}
            pieceMap[childId]["center"] = childCenter
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
    return {
        "pieces": {
            "updated": [
                {
                    "id": p["guid"],
                    "diff": {"plane": p.get("plane"), "center": p.get("center")},
                }
                for p in updatedPieces
                if p["guid"] in piecePlanes
            ]
        }
    }

# endregion FlattenDesign

# region Kit Diff Operations

# [🔖semio/py/semio.py#Kit Diff Operations](semiorepo://section/semio/py/semio.py/KIT-DIFF-OPERATIONS)
# Diffing and patching operations for comparing and merging kit versions.

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
    """Check whether two attribute dictionaries are equal.
    areAttributesEqualDict MUST compare all attribute fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areAttributesEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREATTRIBUTESEQUALDICT)
    """
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
    """Check whether two prop dictionaries are equal.
    arePropsEqualDict MUST compare all prop fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§arePropsEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREPROPSEQUALDICT)
    """
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
    """Check whether two port dictionaries are equal.
    arePortsEqualDict MUST compare all port fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§arePortsEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREPORTSEQUALDICT)
    """
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
        ifaceA = connectorA.get("port", {}) if connectorA.get("port") else {}
        ifaceB = connectorB.get("port", {}) if connectorB.get("port") else {}
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
    """Check whether two model dictionaries are equal.
    areModelsEqualDict MUST compare all model fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areModelsEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREMODELSEQUALDICT)
    """
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
    """Check whether two type dictionaries are equal.
    areTypesEqualDict MUST compare all type fields including children for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areTypesEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/ARETYPESEQUALDICT)
    """
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
    """Check whether two piece dictionaries are equal.
    arePiecesEqualDict MUST compare all piece fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§arePiecesEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREPIECESEQUALDICT)
    """
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
    """Check whether two connection dictionaries are equal.
    areConnectionsEqualDict MUST compare all connection fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areConnectionsEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/ARECONNECTIONSEQUALDICT)
    """
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
    """Check whether two design dictionaries are equal.
    areDesignsEqualDict MUST compare all design fields including children for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areDesignsEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREDESIGNSEQUALDICT)
    """
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

def arePortsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two port dictionaries are equal.
    arePortsEqualDict MUST compare all port fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§arePortsEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREPORTSEQUALDICT)
    """
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
    """Check whether two quality dictionaries are equal.
    areQualitiesEqualDict MUST compare all quality fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areQualitiesEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREQUALITIESEQUALDICT)
    """
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
    """Check whether two file dictionaries are equal.
    areFilesEqualDict MUST compare all file fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areFilesEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREFILESEQUALDICT)
    """
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
    """Check whether two folder dictionaries are equal.
    areFoldersEqualDict MUST compare all folder fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areFoldersEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREFOLDERSEQUALDICT)
    """
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
    """Check whether two author dictionaries are equal.
    areAuthorsEqualDict MUST compare all author fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areAuthorsEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREAUTHORSEQUALDICT)
    """
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
    """Check whether two concept dictionaries are equal.
    areConceptsEqualDict MUST compare all concept fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areConceptsEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/ARECONCEPTSEQUALDICT)
    """
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
    """Check whether two tag dictionaries are equal.
    areTagsEqualDict MUST compare all tag fields for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areTagsEqualDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/ARETAGSEQUALDICT)
    """
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
    areKitsDictEqual MUST compare all kit fields and children recursively.
    [🛠️semio/py/semio.py#Kit Diff Operations§areKitsDictEqual](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREKITSDICTEQUAL)
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

def _applyCollectionDiff(
    base: list,
    diff: dict | None,
    applyItemDiff: typing.Callable[[dict, dict], dict],
    entityKey: str = "",
) -> list:
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
            updateGuid = None
            if entityKey and entityKey in update:
                updateGuid = update[entityKey]["guid"]
            elif "id" in update:
                updateGuid = update["id"]
            if not updateGuid:
                continue
            idx = next(
                (i for i, item in enumerate(result) if item.get("guid") == updateGuid),
                -1,
            )
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
    connectorsDiff = _getCollectionDiff(
        before.get("connectors", []),
        after.get("connectors", []),
        _getConnectorDiff,
        "connector",
    )
    if connectorsDiff:
        diff["connectors"] = connectorsDiff
    modelsDiff = _getCollectionDiff(before.get("models", []), after.get("models", []), _getModelDiff, "model")
    if modelsDiff:
        diff["models"] = modelsDiff
    return diff

def _applyTypeDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a type dict."""
    result = dict(base)
    for key in [
        "name",
        "description",
        "icon",
        "image",
        "unit",
        "isAbstract",
        "virtual",
    ]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("connectors"):
        result["connectors"] = _applyCollectionDiff(
            base.get("connectors", []),
            diff["connectors"],
            _applyConnectorDiff,
            "connector",
        )
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
    connectionsDiff = _getCollectionDiff(
        before.get("connections", []),
        after.get("connections", []),
        _getConnectionDiff,
        "connection",
    )
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
        result["connections"] = _applyCollectionDiff(
            base.get("connections", []),
            diff["connections"],
            _applyConnectionDiff,
            "connection",
        )
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

def _getPortDiff(before: dict, after: dict) -> dict:
    """Get diff between two port dicts."""
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    return diff

def _applyPortDiff(base: dict, diff: dict) -> dict:
    """Apply diff to an port dict."""
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
            upd = next(
                (u for u in appliedDiff.get("updated", []) if (u.get("attribute", {}).get("guid") if "attribute" in u else u.get("id")) == guid),
                None,
            )
            if origAttr and upd:
                inverse["updated"].append(
                    {
                        "attribute": {"guid": guid},
                        "diff": _inverseAttributeDiff(origAttr, upd["diff"]),
                    }
                )
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
    """Compute the diff between two kit dicts.
    getKitDiffDict MUST identify all added, removed and changed entities.
    [🛠️semio/py/semio.py#Kit Diff Operations§getKitDiffDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/GETKITDIFFDICT)
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
        diff["types"] = typesDiff
    designsDiff = _getCollectionDiff(before.get("designs", []), after.get("designs", []), _getDesignDiff, "design")
    if designsDiff:
        diff["designs"] = designsDiff
    tagsDiff = _getCollectionDiff(before.get("tags", []), after.get("tags", []), _getTagDiff, "tag")
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
    portsDiff = _getCollectionDiff(before.get("ports", []), after.get("ports", []), _getPortDiff, "port")
    if portsDiff:
        diff["ports"] = portsDiff
    filesDiff = _getCollectionDiff(before.get("files", []), after.get("files", []), _getFileDiff, "file")
    if filesDiff:
        diff["files"] = filesDiff
    foldersDiff = _getCollectionDiff(before.get("folders", []), after.get("folders", []), _getFolderDiff, "folder")
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
    authorsDiff = _getCollectionDiff(before.get("authors", []), after.get("authors", []), _getAuthorDiff, "author")
    if authorsDiff:
        diff["authors"] = authorsDiff
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff

def applyKitDiffDict(base: dict, diff: dict) -> dict:
    """Apply a diff to a kit dict.
    applyKitDiffDict MUST apply additions, removals and changes correctly.
    [🛠️semio/py/semio.py#Kit Diff Operations§applyKitDiffDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/APPLYKITDIFFDICT)
    """
    result = dict(base)
    result["guid"] = base.get("guid")
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
    if diff.get("ports") or base.get("ports"):
        result["ports"] = _applyCollectionDiff(base.get("ports", []), diff.get("ports"), _applyPortDiff, "port")
    if diff.get("files") or base.get("files"):
        result["files"] = _applyCollectionDiff(base.get("files", []), diff.get("files"), _applyFileDiff, "file")
    if diff.get("folders") or base.get("folders"):
        result["folders"] = _applyCollectionDiff(base.get("folders", []), diff.get("folders"), _applyFolderDiff, "folder")
    if diff.get("qualities") or base.get("qualities"):
        result["qualities"] = _applyCollectionDiff(
            base.get("qualities", []),
            diff.get("qualities"),
            _applyQualityDiff,
            "quality",
        )
    if diff.get("authors") or base.get("authors"):
        result["authors"] = _applyCollectionDiff(base.get("authors", []), diff.get("authors"), _applyAuthorDiff, "author")
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result

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
                    inverse["updated"].append(
                        {
                            entityKey: {"guid": updateGuid},
                            "diff": inverseItemDiff(origItem, update["diff"]),
                        }
                    )
                else:
                    inverse["updated"].append(
                        {
                            "id": updateGuid,
                            "diff": inverseItemDiff(origItem, update["diff"]),
                        }
                    )
    return inverse

def _inverseTypeDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a type diff."""
    inverse: dict = {}
    for key in [
        "name",
        "description",
        "icon",
        "image",
        "unit",
        "isAbstract",
        "virtual",
    ]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("connectors"):
        inverse["connectors"] = _inverseCollectionDiff(
            original.get("connectors", []),
            appliedDiff["connectors"],
            _inverseConnectorDiff,
            "connector",
        )
    return inverse

def _inverseConnectorDiff(original: dict, appliedDiff: dict) -> dict:
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
        inverse["pieces"] = _inverseCollectionDiff(
            original.get("pieces", []),
            appliedDiff["pieces"],
            _inversePieceDiff,
            "piece",
        )
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

def _inversePortDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of an port diff."""
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
    """Compute inverse of an author diff."""
    inverse: dict = {}
    for key in ["name", "email"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse

def inverseKitDiffDict(original: dict, appliedDiff: dict) -> dict:
    """Compute the inverse of a kit diff.
    inverseKitDiffDict MUST swap additions and removals to reverse the diff.
    [🛠️semio/py/semio.py#Kit Diff Operations§inverseKitDiffDict](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/INVERSEKITDIFFDICT)
    """
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
        inverse["types"] = _inverseCollectionDiff(original.get("types", []), appliedDiff["types"], _inverseTypeDiff, "type")
    if appliedDiff.get("designs"):
        inverse["designs"] = _inverseCollectionDiff(
            original.get("designs", []),
            appliedDiff["designs"],
            _inverseDesignDiff,
            "design",
        )
    if appliedDiff.get("tags"):
        inverse["tags"] = _inverseCollectionDiff(original.get("tags", []), appliedDiff["tags"], _inverseTagDiff, "tag")
    if appliedDiff.get("concepts"):
        inverse["concepts"] = _inverseCollectionDiff(
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
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse

def _extractUpdateGuid(update: dict, entityKeys: list[str]) -> str:
    """Extract guid from an updated entry which might use EntityId format or old id format."""
    for key in entityKeys:
        if key in update and isinstance(update[key], dict):
            return update[key].get("guid", "")
    return update.get("id", "")

def areKitDiffsDictEqual(a: dict, b: dict) -> bool:
    """Deep equality check for kit diffs.
    areKitDiffsDictEqual MUST compare all diff entries for equality.
    [🛠️semio/py/semio.py#Kit Diff Operations§areKitDiffsDictEqual](semiorepo://definition/semio/py/semio.py/KIT-DIFF-OPERATIONS/AREKITDIFFSDICTEQUAL)
    """
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

# region Kit Import/Export

# [🔖semio/py/semio.py#Kit Import/Export](semiorepo://section/semio/py/semio.py/KIT-IMPORT-EXPORT)
# Import and export utilities for kit serialization and deserialization.

class KitData:
    """Simple in-memory kit representation that supports attribute access.
    KitData MUST hold all kit entities in memory for import and export operations.
    [🛠️semio/py/semio.py#Kit Import/Export§KitData](semiorepo://definition/semio/py/semio.py/KIT-IMPORT-EXPORT/KITDATA)
    """

    def __init__(self, data: dict):
        self._data = data
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
        return self._data

def _parse_connector_from_sqlite(row: dict) -> dict:
    return {
        "guid": row.get("guid"),
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
        "port": row.get("port_guid"),
        "description": row.get("description"),
    }

def _parse_model_from_sqlite(row: dict) -> dict:
    return {
        "guid": row.get("guid"),
        "name": row.get("name"),
        "file": row.get("file_guid"),
        "description": row.get("description"),
    }

def _parse_type_from_sqlite(row: dict, connectors: list[dict], models: list[dict]) -> dict:
    return {
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
    plane = None
    if row.get("plane_origin_x") is not None:
        plane = {
            "origin": {
                "x": row.get("plane_origin_x", 0.0),
                "y": row.get("plane_origin_y", 0.0),
                "z": row.get("plane_origin_z", 0.0),
            },
            "xAxis": {
                "x": row.get("plane_x_axis_x", 1.0),
                "y": row.get("plane_x_axis_y", 0.0),
                "z": row.get("plane_x_axis_z", 0.0),
            },
            "yAxis": {
                "x": row.get("plane_y_axis_x", 0.0),
                "y": row.get("plane_y_axis_y", 1.0),
                "z": row.get("plane_y_axis_z", 0.0),
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
    if row.get("center_u") is not None or row.get("center_v") is not None:
        center = {
            "u": row.get("center_u", 0.0),
            "v": row.get("center_v", 0.0),
        }
    return {
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
    return {
        "guid": row.get("guid"),
        "connected": {
            "piece": row.get("connected_piece_guid"),
            "designPiece": row.get("connected_design_piece_guid"),
            "connector": row.get("connected_connector_guid"),
        },
        "connecting": {
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
    view = None
    if row.get("view_center_u") is not None or row.get("view_center_v") is not None or row.get("view_zoom") is not None:
        view = {
            "center": {
                "u": row.get("view_center_u", 0.0),
                "v": row.get("view_center_v", 0.0),
            },
            "zoom": row.get("view_zoom", 1.0),
        }
    return {
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
        "canScale": bool(row.get("can_scale", False)) if row.get("can_scale") is not None else None,
        "canMirror": bool(row.get("can_mirror", False)) if row.get("can_mirror") is not None else None,
        "description": row.get("description"),
        "icon": row.get("icon"),
        "image": row.get("image"),
        "pieces": pieces,
        "connections": connections,
    }

def import_kit(path: str) -> tuple[KitData, dict[str, bytes]]:
    """📦Import a kit from a .zip file (containing a .semio/kit.db sqlite database).
    import_kit MUST handle both local directories and remote zip archives.
    [🛠️semio/py/semio.py#Kit Import/Export§import_kit](semiorepo://definition/semio/py/semio.py/KIT-IMPORT-EXPORT/IMPORT-KIT)
    """
    if not os.path.exists(path):
        raise FileNotFoundError(f"File not found: {path}")

    files = {}
    with tempfile.TemporaryDirectory() as tmpdirname:
        with zipfile.ZipFile(path, "r") as zip_ref:
            zip_ref.extractall(tmpdirname)
            for file_info in zip_ref.infolist():
                if not file_info.is_dir() and not file_info.filename.startswith(".semio/"):
                    with zip_ref.open(file_info) as f:
                        files[file_info.filename] = f.read()

        db_path = os.path.join(tmpdirname, ".semio", "kit.db")
        if not os.path.exists(db_path):
            raise ValueError(f"Invalid kit: .semio/kit.db not found in {path}")

        import sqlite3

        conn = sqlite3.connect(db_path)
        conn.row_factory = sqlite3.Row
        cursor = conn.cursor()

        cursor.execute("SELECT * FROM kit LIMIT 1")
        kit_row = cursor.fetchone()
        if not kit_row:
            conn.close()
            raise ValueError("No Kit found in database")

        kit_dict = dict(kit_row)
        kit_guid = kit_dict.get("guid", str(uuid.uuid4()))
        uri = f"memory://{kit_dict.get('name', 'unnamed')}"

        cursor.execute("SELECT * FROM type WHERE kit_guid = ?", (kit_guid,))
        type_rows = cursor.fetchall()
        types_list = []
        for t_row in type_rows:
            t = dict(t_row)
            type_guid = t["guid"]
            cursor.execute("SELECT * FROM connector WHERE type_guid = ?", (type_guid,))
            connectors = [_parse_connector_from_sqlite(dict(c)) for c in cursor.fetchall()]
            cursor.execute("SELECT * FROM model WHERE type_guid = ?", (type_guid,))
            models = [_parse_model_from_sqlite(dict(m)) for m in cursor.fetchall()]
            types_list.append(_parse_type_from_sqlite(t, connectors, models))

        cursor.execute("SELECT * FROM design WHERE kit_guid = ?", (kit_guid,))
        design_rows = cursor.fetchall()
        designs_list = []
        for d_row in design_rows:
            d = dict(d_row)
            design_guid = d["guid"]
            cursor.execute("SELECT * FROM piece WHERE design_guid = ?", (design_guid,))
            pieces = [_parse_piece_from_sqlite(dict(p)) for p in cursor.fetchall()]
            cursor.execute("SELECT * FROM connection WHERE design_guid = ?", (design_guid,))
            connections = [_parse_connection_from_sqlite(dict(c)) for c in cursor.fetchall()]
            designs_list.append(_parse_design_from_sqlite(d, pieces, connections))

        conn.close()

        kit_data_dict = {
            "guid": kit_guid,
            "uri": uri,
            "name": kit_dict.get("name", ""),
            "version": kit_dict.get("version", ""),
            "description": kit_dict.get("description", ""),
            "icon": kit_dict.get("icon", ""),
            "image": kit_dict.get("image", ""),
            "remote": kit_dict.get("remote", ""),
            "homepage": kit_dict.get("homepage", ""),
            "license": kit_dict.get("license", ""),
            "preview": kit_dict.get("preview", ""),
            "types": types_list,
            "designs": designs_list,
        }

    return KitData(kit_data_dict), files

def _write_kit_to_sqlite(kit_data: KitData | dict, db_path: str) -> None:
    """Write kit data to SQLite database using the TypeScript schema."""
    import sqlite3
    from datetime import datetime

    data = kit_data.to_dict() if isinstance(kit_data, KitData) else kit_data

    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS kit (
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
            updated DATETIME NOT NULL
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS type (
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
            row_id INTEGER
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS connector (
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
            type_guid VARCHAR(36) NOT NULL
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS model (
            guid VARCHAR(36) PRIMARY KEY,
            file_guid VARCHAR(36) NOT NULL,
            name VARCHAR(256),
            description TEXT,
            type_guid VARCHAR(36) NOT NULL
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS design (
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
            row_id INTEGER PRIMARY KEY
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS piece (
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
            design_guid VARCHAR(36) NOT NULL
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS connection (
            guid VARCHAR(36) PRIMARY KEY,
            connected_piece_guid VARCHAR(36) NOT NULL,
            connected_design_piece_guid VARCHAR(36),
            connected_connector_guid VARCHAR(36) NOT NULL,
            connecting_piece_guid VARCHAR(36) NOT NULL,
            connecting_design_piece_guid VARCHAR(36),
            connecting_connector_guid VARCHAR(36) NOT NULL,
            gap FLOAT DEFAULT 0,
            shift FLOAT DEFAULT 0,
            rise FLOAT DEFAULT 0,
            rotation FLOAT DEFAULT 0,
            turn FLOAT DEFAULT 0,
            tilt FLOAT DEFAULT 0,
            u FLOAT,
            v FLOAT,
            description TEXT,
            design_guid VARCHAR(36) NOT NULL
        )
    """)

    now = datetime.now().isoformat()
    kit_guid = data.get("guid", str(uuid.uuid4()))

    cursor.execute(
        """
        INSERT INTO kit (guid, name, version, description, icon, image, preview, remote, homepage, license, created, updated)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
    """,
        (
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
        cursor.execute(
            """
            INSERT INTO type (guid, name, parent_guid, is_abstract, folder, stock, virtual, unit, location_guid, description, icon, image, created, updated, kit_guid)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
            (
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
            cursor.execute(
                """
                INSERT INTO connector (guid, name, point_x, point_y, point_z, direction_x, direction_y, direction_z, t, mandatory, port_guid, description, type_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                (
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
                    c.get("port"),
                    c.get("description"),
                    type_guid,
                ),
            )

        for m in t.get("models", []):
            cursor.execute(
                """
                INSERT INTO model (guid, file_guid, name, description, type_guid)
                VALUES (?, ?, ?, ?, ?)
            """,
                (
                    m.get("guid", str(uuid.uuid4())),
                    m.get("file", ""),
                    m.get("name"),
                    m.get("description"),
                    type_guid,
                ),
            )

    for d in data.get("designs", []):
        design_guid = d.get("guid", str(uuid.uuid4()))
        view = d.get("view") or {}
        view_center = view.get("center") or {}
        cursor.execute(
            """
            INSERT INTO design (guid, name, parent_guid, variant, view_center_u, view_center_v, view_zoom, unit, location_guid, active_layer_guid, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated, kit_guid)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
            (
                design_guid,
                d.get("name", ""),
                d.get("parent"),
                d.get("variant"),
                view_center.get("u"),
                view_center.get("v"),
                view.get("zoom"),
                d.get("unit"),
                d.get("location"),
                d.get("activeLayer"),
                1 if d.get("isAbstract") else 0,
                d.get("folder"),
                1 if d.get("canScale") else (0 if d.get("canScale") is False else None),
                1 if d.get("canMirror") else (0 if d.get("canMirror") is False else None),
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
            cursor.execute(
                """
                INSERT INTO piece (guid, name, type_guid, design_guid_ref, plane_origin_x, plane_origin_y, plane_origin_z,
                    plane_x_axis_x, plane_x_axis_y, plane_x_axis_z, plane_y_axis_x, plane_y_axis_y, plane_y_axis_z,
                    center_u, center_v, scale, mirror_plane_origin_x, mirror_plane_origin_y, mirror_plane_origin_z,
                    mirror_plane_x_axis_x, mirror_plane_x_axis_y, mirror_plane_x_axis_z,
                    mirror_plane_y_axis_x, mirror_plane_y_axis_y, mirror_plane_y_axis_z,
                    is_hidden, is_locked, color, description, design_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                (
                    p.get("guid", str(uuid.uuid4())),
                    p.get("id"),
                    p.get("type"),
                    p.get("design"),
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
            cursor.execute(
                """
                INSERT INTO connection (guid, connected_piece_guid, connected_design_piece_guid, connected_connector_guid,
                    connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid,
                    gap, shift, rise, rotation, turn, tilt, u, v, description, design_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                (
                    c.get("guid", str(uuid.uuid4())),
                    connected.get("piece"),
                    connected.get("designPiece"),
                    connected.get("connector"),
                    connecting.get("piece"),
                    connecting.get("designPiece"),
                    connecting.get("connector"),
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

    conn.commit()
    conn.close()

def export_kit(kit: KitData, files: dict[str, bytes], path: str) -> None:
    """📦Export a kit to a .zip file (containing a .semio/kit.db sqlite database).
    export_kit MUST write the kit database and files to the target path.
    [🛠️semio/py/semio.py#Kit Import/Export§export_kit](semiorepo://definition/semio/py/semio.py/KIT-IMPORT-EXPORT/EXPORT-KIT)
    """
    with tempfile.TemporaryDirectory() as tmpdirname:
        semio_dir = os.path.join(tmpdirname, ".semio")
        os.makedirs(semio_dir, exist_ok=True)
        db_path = os.path.join(semio_dir, "kit.db")

        _write_kit_to_sqlite(kit, db_path)

        with zipfile.ZipFile(path, "w", zipfile.ZIP_DEFLATED) as zip_ref:
            zip_ref.write(db_path, ".semio/kit.db")

            for filename, content in files.items():
                zip_ref.writestr(filename, content)

# endregion Kit Import/Export

# region Spatial Math

# [🔖semio/py/semio.py#Spatial Math](semiorepo://section/semio/py/semio.py/SPATIAL-MATH)
# Spatial math utilities for vector normalization and plane computation.

def normalizeVector(v: numpy.ndarray) -> numpy.ndarray:
    """Normalize a 3D vector to unit length.
    normalizeVector MUST return a unit-length vector or raise on zero length.
    [🛠️semio/py/semio.py#Spatial Math§normalizeVector](semiorepo://definition/semio/py/semio.py/SPATIAL-MATH/NORMALIZEVECTOR)
    """
    length = numpy.linalg.norm(v)
    if length < 1e-10:
        return v
    return v / length

def planeFromYAxis(yAxis: numpy.ndarray, phiDegrees: float = 0.0, origin: numpy.ndarray | None = None) -> Plane:
    """Construct a plane from an origin point and a Y-axis direction.
    planeFromYAxis MUST derive orthogonal x and z axes from the y axis.
    [🛠️semio/py/semio.py#Spatial Math§planeFromYAxis](semiorepo://definition/semio/py/semio.py/SPATIAL-MATH/PLANEFROMYAXIS)
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
    """Compute the world-space plane of a child from parent and local planes.
    computeChildPlane MUST compose parent and local plane transformations.
    [🛠️semio/py/semio.py#Spatial Math§computeChildPlane](semiorepo://definition/semio/py/semio.py/SPATIAL-MATH/COMPUTECHILDPLANE)
    """
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
    ppDir = numpy.array(
        [
            parentConnector.direction.x,
            parentConnector.direction.y,
            parentConnector.direction.z,
        ]
    )
    cpPoint = numpy.array([childConnector.point.x, childConnector.point.y, childConnector.point.z])
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
