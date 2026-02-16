# region Header

# [👤semio📚py💻semiopy](semiorepo://file/SEMIO/PY/SEMIO.PY)

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

# [👤semio📚py💻semiopy🔖imports](semiorepo://section/SEMIO/PY/SEMIO.PY/IMPORTS)
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

# [👤semio📚py💻semiopy🔖typehints](semiorepo://section/SEMIO/PY/SEMIO.PY/TYPE-HINTS)
# Custom type hint aliases used throughout the module.

RecursiveAnyList = typing.Any | list["RecursiveAnyList"]
"""🔁 A recursive any list is either any or a list where the items are recursive any list."""

# endregion Type Hints

# region Constants

# [👤semio📚py💻semiopy🔖constants](semiorepo://section/SEMIO/PY/SEMIO.PY/CONSTANTS)
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

# [👤semio📚py💻semiopy🔖utility](semiorepo://section/SEMIO/PY/SEMIO.PY/UTILITY)
# General-purpose utility functions for encoding, formatting and transformation.

def encode(value: str) -> str:
    """ᗒ Encode a string to be url safe.
    encode MUST return a percent-encoded string safe for URL paths.
    [👤semio📚py💻semiopy🔖utility🛠️encode](semiorepo://definition/SEMIO/PY/SEMIO.PY/UTILITY/ENCODE)
    """
    return urllib.parse.quote(value, safe="")

def decode(value: str) -> str:
    """ᗕ Decode a url safe string.
    decode MUST return the original string from a percent-encoded input.
    [👤semio📚py💻semiopy🔖utility🛠️decode](semiorepo://definition/SEMIO/PY/SEMIO.PY/UTILITY/DECODE)
    """
    return urllib.parse.unquote(value)

def encodeList(items: list[str]) -> str:
    """Encode a list of strings into a comma-separated URL-safe string.
    encodeList MUST encode each item and join them with commas.
    [👤semio📚py💻semiopy🔖utility🛠️encodelist](semiorepo://definition/SEMIO/PY/SEMIO.PY/UTILITY/ENCODE-LIST)
    """
    return ",".join([encode(t) for t in items])

def decodeList(encodedList: str) -> list[str]:
    """Decode a comma-separated URL-safe string into a list of strings.
    decodeList MUST split by comma and decode each item.
    [👤semio📚py💻semiopy🔖utility🛠️decodelist](semiorepo://definition/SEMIO/PY/SEMIO.PY/UTILITY/DECODE-LIST)
    """
    return [decode(t) for t in encodedList.split(",")]

def encodeRecursiveAnyList(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Encode a `RecursiveAnyList` to a url encoded string.
    encodeRecursiveAnyList MUST recursively encode nested lists into a flat string.
    [👤semio📚py💻semiopy🔖utility🛠️encoderecursiveanylist](semiorepo://definition/SEMIO/PY/SEMIO.PY/UTILITY/ENCODE-RECURSIVE-ANY-LIST)
    """
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return encode(",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList]))

def create_id(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Turn any into `encoded(str(any))` or a recursive list into a flat comma [,] separated encoded list.
    create_id MUST produce a deterministic identifier from any value or nested list.
    [👤semio📚py💻semiopy🔖utility🛠️createid](semiorepo://definition/SEMIO/PY/SEMIO.PY/UTILITY/CREATE-ID)
    """
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return ",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList])

def pretty(number: float) -> str:
    """🦋 Pretty print a floating point number.
    pretty MUST format the number with up to 5 significant digits.
    [👤semio📚py💻semiopy🔖utility🛠️pretty](semiorepo://definition/SEMIO/PY/SEMIO.PY/UTILITY/PRETTY)
    """
    if number == -0.0:
        number = 0.0
    return f"{number:.5f}".rstrip("0").rstrip(".")

def changeValues(c: dict | list, key: str, func: typing.Callable[[typing.Any], typing.Any]) -> None:
    """Recursively change values for a given key in nested dicts and lists.
    changeValues MUST apply the function to all occurrences of the key recursively.
    [👤semio📚py💻semiopy🔖utility🛠️changevalues](semiorepo://definition/SEMIO/PY/SEMIO.PY/UTILITY/CHANGE-VALUES)
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
    [👤semio📚py💻semiopy🔖utility🛠️changekeys](semiorepo://definition/SEMIO/PY/SEMIO.PY/UTILITY/CHANGE-KEYS)
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
    [👤semio📚py💻semiopy🔖utility🛠️normalizeangle](semiorepo://definition/SEMIO/PY/SEMIO.PY/UTILITY/NORMALIZE-ANGLE)
    """
    return (angle % 360 + 360) % 360

# endregion Utility

# region Logging

# [👤semio📚py💻semiopy🔖logging](semiorepo://section/SEMIO/PY/SEMIO.PY/LOGGING)
# Module-level logger configuration.

logger = loguru.logger

# endregion Logging

# region Exceptions

# [👤semio📚py💻semiopy🔖exceptions](semiorepo://section/SEMIO/PY/SEMIO.PY/EXCEPTIONS)
# Custom exception hierarchy for server, client and specification errors.

class Error(Exception, abc.ABC):
    """❗ The base for all exceptions.
    Error MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖exceptions🛠️error](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/ERROR)
    """

    def __str__(self):
        return "❗ " + self.__class__.__name__

class ServerError(Error, abc.ABC):
    """🖥 The base for all server errors.
    ServerError MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖exceptions🛠️servererror](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/SERVER-ERROR)
    """

class ClientError(Error, abc.ABC):
    """👩‍💼 The base for all client errors.
    ClientError MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖exceptions🛠️clienterror](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/CLIENT-ERROR)
    """

class CodeUnreachable(ServerError):
    """Exception for code paths that should never be reached.
    CodeUnreachable MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖exceptions🛠️codeunreachable](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/CODE-UNREACHABLE)
    """
    def __str__(self):
        return "🤷 This code should be unreachable."

class FeatureNotYetSupported(ServerError):
    """Exception for unimplemented features.
    FeatureNotYetSupported MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖exceptions🛠️featurenotyetsupported](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/FEATURE-NOT-YET-SUPPORTED)
    """
    def __str__(self):
        return "🔜 This feature is not yet supported."

class RemoteKitsNotYetSupported(FeatureNotYetSupported):
    """Exception for unsupported remote kit access.
    RemoteKitsNotYetSupported MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖exceptions🛠️remotekitsnotyetsupported](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/REMOTE-KITS-NOT-YET-SUPPORTED)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return "🔜 Remote kits are not yet supported."

class NotFound(ClientError, abc.ABC):
    """🔍 The base for not found errors.
    NotFound MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖exceptions🛠️notfound](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/NOT-FOUND)
    """

class SpecificationError(ClientError, abc.ABC):
    """📋 The base for all specification errors.
    SpecificationError MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖exceptions🛠️specificationerror](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/SPECIFICATION-ERROR)
    """

class NoParentAssigned(SpecificationError, abc.ABC):
    """👪 The base for all no parent assigned errors.
    NoParentAssigned MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖exceptions🛠️noparentassigned](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/NO-PARENT-ASSIGNED)
    """

class NoTypeOrDesignAssigned(NoParentAssigned):
    """No Type Or Design Assigned definition.
    NoTypeOrDesignAssigned MUST fulfill its documented contract.
    [👤semio📚py💻semiopy🔖exceptions🛠️notypeordesignassigned](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/NO-TYPE-OR-DESIGN-ASSIGNED)
    """
    def __str__(self):
        return "👪 The entity has no parent type or design assigned."

class NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned(NoParentAssigned):
    """No Model Or Port Or Type Or Piece Or Connection Or Design Or Kit Assigned definition.
    NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned MUST fulfill its documented contract.
    [👤semio📚py💻semiopy🔖exceptions🛠️nomodelorportortypeorpieceorconnectionordesignorkitassigned](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/NO-MODEL-OR-PORT-OR-TYPE-OR-PIECE-OR-CONNECTION-OR-DESIGN-OR-KIT-ASSIGNED)
    """
    def __str__(self):
        return "👪 The entity has no parent model, connector, type, piece, connection, design, kit or folder assigned."

class AlreadyExists(SpecificationError, abc.ABC):
    """♊ The entity already exists in the store.
    AlreadyExists MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖exceptions🛠️alreadyexists](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/ALREADY-EXISTS)
    """

class Semio(sqlmodel.SQLModel, table=True):
    """ℹ Metadata about the database.
    Semio MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semiopy🔖exceptions🛠️semio](semiorepo://definition/SEMIO/PY/SEMIO.PY/EXCEPTIONS/SEMIO)
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

# [👤semio📚py💻semiopy🔖modeling🔖primitives](semiorepo://section/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES)
# Abstract base classes for models, fields, ids, inputs, outputs and entities.

class SModel(sqlmodel.SQLModel, abc.ABC):
    """⚪ The base for models.
    SModel MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️smodel](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/S-MODEL)
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
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️field](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/FIELD)
    """

class RealField(Field, abc.ABC):
    """🧑 The base for a real field of a model. No lie.
    RealField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️realfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/REAL-FIELD)
    """

class MaskedField(Field, abc.ABC):
    """🎭 The base for a mask of a field of a model. WYSIWYG but don't expect it to be there.
    MaskedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️maskedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/MASKED-FIELD)
    """

class Base(SModel, abc.ABC):
    """👥 The base for models.
    Base MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️base](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/BASE)
    """

class Id(Base, abc.ABC):
    """🪪 The base for ids. All fields that identify the entity here.
    Id MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️id](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/ID)
    """

class Props(Base, abc.ABC):
    """🎫 The base for props. All fields except input-only, output-only or child entities.
    Props MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️props](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/PROPS)
    """

class Input(Base, abc.ABC):
    """↘ The base for inputs. All fields that are required to create the entity.
    Input MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️input](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/INPUT)
    """

class Context(Base, abc.ABC):
    """📑 The base for contexts. All fields that are required to understand the entity by an llm.
    Context MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️context](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/CONTEXT)
    """

class Output(Base, abc.ABC):
    """↗ The base for outputs. All fields that are returned when the entity is fetched.
    Output MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️output](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/OUTPUT)
    """

class Prediction(Base, abc.ABC):
    """🔮 The base for predictions. All fields that are required to predict the entity by a llm.
    Prediction MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️prediction](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/PREDICTION)
    """

class Entity(SModel, abc.ABC):
    """▢ The base for entities. All fields and behavior of the entity.
    Entity MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️entity](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/ENTITY)
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
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️table](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/TABLE)
    """

class TableEntity(Entity, Table, abc.ABC):
    """▢ The base for table entities.
    TableEntity MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖modeling🔖primitives🛠️tableentity](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/PRIMITIVES/TABLE-ENTITY)
    """

    __tablename__: typing.ClassVar[str]
    """📛 The lowercase name of the table in the database."""

# endregion Primitives

# region Graphql

# [👤semio📚py💻semiopy🔖modeling🔖graphql](semiorepo://section/SEMIO/PY/SEMIO.PY/MODELING/GRAPHQL)
# GraphQL node base classes for pydantic, sqlalchemy and relay integration.

class Node(graphene_pydantic.PydanticObjectType):
    """A base class for all nodes that are not a table in the database.
    Node MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖modeling🔖graphql🛠️node](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/GRAPHQL/NODE)
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
    [👤semio📚py💻semiopy🔖modeling🔖graphql🛠️inputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/GRAPHQL/INPUT-NODE)
    """

    class Meta:
        abstract = True

class RelayNode(graphene.relay.Node):
    """Relay-compliant GraphQL node interface.
    RelayNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖modeling🔖graphql🛠️relaynode](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/GRAPHQL/RELAY-NODE)
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
    [👤semio📚py💻semiopy🔖modeling🔖graphql🛠️tablenode](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/GRAPHQL/TABLE-NODE)
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
    [👤semio📚py💻semiopy🔖modeling🔖graphql🛠️tableentitynode](semiorepo://definition/SEMIO/PY/SEMIO.PY/MODELING/GRAPHQL/TABLE-ENTITY-NODE)
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

# [👤semio📚py💻semiopy🔖domain](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN)
# Attribute entity with key-value pairs and definitions.

class AttributeKeyField(RealField, abc.ABC):
    """Field mixin for the key of a attribute.
    AttributeKeyField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖attribute🛠️attributekeyfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ATTRIBUTE/ATTRIBUTE-KEY-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class AttributeValueField(RealField, abc.ABC):
    """Field mixin for the value of a attribute.
    AttributeValueField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖attribute🛠️attributevaluefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ATTRIBUTE/ATTRIBUTE-VALUE-FIELD)
    """
    value: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class AttributeDefinitionField(RealField, abc.ABC):
    """Field mixin for the definition of a attribute.
    AttributeDefinitionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖attribute🛠️attributedefinitionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ATTRIBUTE/ATTRIBUTE-DEFINITION-FIELD)
    """
    definition: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class AttributeId(AttributeKeyField, Id):
    """Identity fields for uniquely identifying a attribute.
    AttributeId MUST contain all fields that uniquely identify a attribute.
    [👤semio📚py💻semiopy🔖domain🔖attribute🛠️attributeid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ATTRIBUTE/ATTRIBUTE-ID)
    """
    pass

class AttributeProps(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Props):
    """Property fields for a attribute.
    AttributeProps MUST contain all non-relational property fields.
    [👤semio📚py💻semiopy🔖domain🔖attribute🛠️attributeprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ATTRIBUTE/ATTRIBUTE-PROPS)
    """
    pass

class AttributeInput(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Input):
    """Input fields for creating or updating a attribute.
    AttributeInput MUST contain all fields required for creation.
    [👤semio📚py💻semiopy🔖domain🔖attribute🛠️attributeinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ATTRIBUTE/ATTRIBUTE-INPUT)
    """
    pass

class AttributeContext(AttributeValueField, AttributeKeyField, Context):
    """Context fields for understanding a attribute by an LLM.
    AttributeContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖attribute🛠️attributecontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ATTRIBUTE/ATTRIBUTE-CONTEXT)
    """
    pass

class AttributeOutput(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Output):
    """Output fields returned when fetching a attribute.
    AttributeOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖attribute🛠️attributeoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ATTRIBUTE/ATTRIBUTE-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖attribute🛠️attribute](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ATTRIBUTE/ATTRIBUTE)
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
    [👤semio📚py💻semiopy🔖domain🔖attribute🛠️attributeinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ATTRIBUTE/ATTRIBUTE-INPUT-NODE)
    """
    class Meta:
        model = AttributeInput

# endregion Attribute

# region Tag

# [👤semio📚py💻semiopy🔖domain🔖tag](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/TAG)
# Tag entity for categorizing and labeling kit elements.

class TagGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a tag.
    TagGuidField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖tag🛠️tagguidfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TAG/TAG-GUID-FIELD)
    """
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class TagNameField(RealField, abc.ABC):
    """Field mixin for the name of a tag.
    TagNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖tag🛠️tagnamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TAG/TAG-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class TagDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a tag.
    TagDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖tag🛠️tagdescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TAG/TAG-DESCRIPTION-FIELD)
    """
    description: typing.Optional[str] = sqlmodel.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)

class TagIconField(RealField, abc.ABC):
    """Field mixin for the icon of a tag.
    TagIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖tag🛠️tagiconfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TAG/TAG-ICON-FIELD)
    """
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class TagOrderField(RealField, abc.ABC):
    """Field mixin for the order of a tag.
    TagOrderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖tag🛠️tagorderfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TAG/TAG-ORDER-FIELD)
    """
    order: int = sqlmodel.Field(default=0)

class TagId(TagGuidField, Id):
    """Identity fields for uniquely identifying a tag.
    TagId MUST contain all fields that uniquely identify a tag.
    [👤semio📚py💻semiopy🔖domain🔖tag🛠️tagid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TAG/TAG-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖tag🛠️tag](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TAG/TAG)
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

# [👤semio📚py💻semiopy🔖domain🔖concept](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/CONCEPT)
# Concept entity for semantic grouping of design elements.

class ConceptGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a concept.
    ConceptGuidField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖concept🛠️conceptguidfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONCEPT/CONCEPT-GUID-FIELD)
    """
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class ConceptNameField(RealField, abc.ABC):
    """Field mixin for the name of a concept.
    ConceptNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖concept🛠️conceptnamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONCEPT/CONCEPT-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class ConceptDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a concept.
    ConceptDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖concept🛠️conceptdescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONCEPT/CONCEPT-DESCRIPTION-FIELD)
    """
    description: typing.Optional[str] = sqlmodel.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)

class ConceptIconField(RealField, abc.ABC):
    """Field mixin for the icon of a concept.
    ConceptIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖concept🛠️concepticonfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONCEPT/CONCEPT-ICON-FIELD)
    """
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class ConceptOrderField(RealField, abc.ABC):
    """Field mixin for the order of a concept.
    ConceptOrderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖concept🛠️conceptorderfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONCEPT/CONCEPT-ORDER-FIELD)
    """
    order: int = sqlmodel.Field(default=0)

class ConceptId(ConceptGuidField, Id):
    """Identity fields for uniquely identifying a concept.
    ConceptId MUST contain all fields that uniquely identify a concept.
    [👤semio📚py💻semiopy🔖domain🔖concept🛠️conceptid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONCEPT/CONCEPT-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖concept🛠️concept](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONCEPT/CONCEPT)
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

# [👤semio📚py💻semiopy🔖domain🔖coord](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/COORD)
# Coordinate primitive for three-dimensional values.

class Coord(SModel):
    """Three-dimensional coordinate with x, y and z values.
    Coord MUST contain all coordinate or geometry fields.
    [👤semio📚py💻semiopy🔖domain🔖coord🛠️coord](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/COORD/COORD)
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
    [👤semio📚py💻semiopy🔖domain🔖coord🛠️coordinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/COORD/COORD-INPUT)
    """
    pass

class CoordContext(Coord, Context):
    """Context fields for understanding a coord by an LLM.
    CoordContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖coord🛠️coordcontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/COORD/COORD-CONTEXT)
    """
    pass

class CoordOutput(Coord, Output):
    """Output fields returned when fetching a coord.
    CoordOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖coord🛠️coordoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/COORD/COORD-OUTPUT)
    """
    pass

class CoordPrediction(Coord, Prediction):
    """Prediction fields for LLM-based coord inference.
    CoordPrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semiopy🔖domain🔖coord🛠️coordprediction](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/COORD/COORD-PREDICTION)
    """
    pass

class CoordNode(Node):
    """GraphQL node exposing coord data.
    CoordNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖coord🛠️coordnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/COORD/COORD-NODE)
    """
    class Meta:
        model = Coord

class CoordInputNode(InputNode):
    """GraphQL input node for coord mutations.
    CoordInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖coord🛠️coordinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/COORD/COORD-INPUT-NODE)
    """
    class Meta:
        model = CoordInput

# endregion Coord

# region Point

# [👤semio📚py💻semiopy🔖domain🔖point](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/POINT)
# Point primitive representing a position in 3D space.

class Point(SModel):
    """Point in 3D space with x, y and z coordinates.
    Point MUST contain all coordinate or geometry fields.
    [👤semio📚py💻semiopy🔖domain🔖point🛠️point](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/POINT/POINT)
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
    [👤semio📚py💻semiopy🔖domain🔖point🛠️pointinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/POINT/POINT-INPUT)
    """
    pass

class PointContext(Point, Context):
    """Context fields for understanding a point by an LLM.
    PointContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖point🛠️pointcontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/POINT/POINT-CONTEXT)
    """
    pass

class PointOutput(Point, Output):
    """Output fields returned when fetching a point.
    PointOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖point🛠️pointoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/POINT/POINT-OUTPUT)
    """
    pass

class PointPrediction(Point, Prediction):
    """Prediction fields for LLM-based point inference.
    PointPrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semiopy🔖domain🔖point🛠️pointprediction](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/POINT/POINT-PREDICTION)
    """
    pass

class PointNode(Node):
    """GraphQL node exposing point data.
    PointNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖point🛠️pointnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/POINT/POINT-NODE)
    """
    class Meta:
        model = Point

class PointInputNode(InputNode):
    """GraphQL input node for point mutations.
    PointInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖point🛠️pointinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/POINT/POINT-INPUT-NODE)
    """
    class Meta:
        model = PointInput

# endregion Point

# region Vector

# [👤semio📚py💻semiopy🔖domain🔖vector](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/VECTOR)
# Vector primitive representing a direction in 3D space.

class Vector(SModel):
    """Direction vector in 3D space with x, y and z components.
    Vector MUST contain all coordinate or geometry fields.
    [👤semio📚py💻semiopy🔖domain🔖vector🛠️vector](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VECTOR/VECTOR)
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
    [👤semio📚py💻semiopy🔖domain🔖vector🛠️vectorinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VECTOR/VECTOR-INPUT)
    """
    pass

class VectorContext(Vector, Context):
    """Context fields for understanding a vector by an LLM.
    VectorContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖vector🛠️vectorcontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VECTOR/VECTOR-CONTEXT)
    """
    pass

class VectorOutput(Vector, Output):
    """Output fields returned when fetching a vector.
    VectorOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖vector🛠️vectoroutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VECTOR/VECTOR-OUTPUT)
    """
    pass

class VectorPrediction(Vector, Prediction):
    """Prediction fields for LLM-based vector inference.
    VectorPrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semiopy🔖domain🔖vector🛠️vectorprediction](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VECTOR/VECTOR-PREDICTION)
    """
    pass

class VectorNode(Node):
    """GraphQL node exposing vector data.
    VectorNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖vector🛠️vectornode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VECTOR/VECTOR-NODE)
    """
    class Meta:
        model = Vector

class VectorInputNode(InputNode):
    """GraphQL input node for vector mutations.
    VectorInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖vector🛠️vectorinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VECTOR/VECTOR-INPUT-NODE)
    """
    class Meta:
        model = VectorInput

# endregion Vector

# region Plane

# [👤semio📚py💻semiopy🔖domain🔖plane](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/PLANE)
# Plane primitive representing an oriented coordinate frame in 3D space.

class PlaneOriginField(MaskedField, abc.ABC):
    """Field mixin for the origin of a plane.
    PlaneOriginField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖plane🛠️planeoriginfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PLANE/PLANE-ORIGIN-FIELD)
    """
    origin: Point = sqlmodel.Field()

class PlaneXAxisField(MaskedField, abc.ABC):
    """Field mixin for the x axis of a plane.
    PlaneXAxisField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖plane🛠️planexaxisfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PLANE/PLANE-X-AXIS-FIELD)
    """
    xAxis: Vector = sqlmodel.Field()

class PlaneYAxisField(MaskedField, abc.ABC):
    """Field mixin for the y axis of a plane.
    PlaneYAxisField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖plane🛠️planeyaxisfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PLANE/PLANE-Y-AXIS-FIELD)
    """
    yAxis: Vector = sqlmodel.Field()

class PlaneInput(Input):
    """Input fields for creating or updating a plane.
    PlaneInput MUST contain all fields required for creation.
    [👤semio📚py💻semiopy🔖domain🔖plane🛠️planeinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PLANE/PLANE-INPUT)
    """
    origin: PointInput = sqlmodel.Field()
    xAxis: VectorInput = sqlmodel.Field()
    yAxis: VectorInput = sqlmodel.Field()

class PlaneContext(Context):
    """Context fields for understanding a plane by an LLM.
    PlaneContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖plane🛠️planecontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PLANE/PLANE-CONTEXT)
    """
    origin: PointContext = sqlmodel.Field()
    xAxis: VectorContext = sqlmodel.Field()
    yAxis: VectorContext = sqlmodel.Field()

class PlaneOutput(PlaneYAxisField, PlaneXAxisField, PlaneOriginField, Output):
    """Output fields returned when fetching a plane.
    PlaneOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖plane🛠️planeoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PLANE/PLANE-OUTPUT)
    """
    pass

class Plane(Table, table=True):
    """Oriented coordinate frame in 3D space with origin and axes.
    Plane MUST contain all coordinate or geometry fields.
    [👤semio📚py💻semiopy🔖domain🔖plane🛠️plane](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PLANE/PLANE)
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
    [👤semio📚py💻semiopy🔖domain🔖plane🛠️planeinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PLANE/PLANE-INPUT-NODE)
    """
    class Meta:
        model = PlaneInput

# endregion Plane

# region Location

# [👤semio📚py💻semiopy🔖domain🔖location](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION)
# Location entity for geographic coordinates with longitude, latitude and altitude.

class LocationGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a location.
    LocationGuidField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationguidfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-GUID-FIELD)
    """
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class LocationLongitudeField(RealField, abc.ABC):
    """Field mixin for the longitude of a location.
    LocationLongitudeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationlongitudefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-LONGITUDE-FIELD)
    """
    longitude: float = sqlmodel.Field()

class LocationLatitudeField(RealField, abc.ABC):
    """Field mixin for the latitude of a location.
    LocationLatitudeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationlatitudefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-LATITUDE-FIELD)
    """
    latitude: float = sqlmodel.Field()

class LocationAltitudeField(RealField, abc.ABC):
    """Field mixin for the altitude of a location.
    LocationAltitudeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationaltitudefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-ALTITUDE-FIELD)
    """
    altitude: typing.Optional[float] = sqlmodel.Field(default=None)

class LocationId(LocationGuidField, Id):
    """Identity fields for uniquely identifying a location.
    LocationId MUST contain all fields that uniquely identify a location.
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖location🛠️location](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION)
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
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-INPUT)
    """
    pass

class LocationOutput(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Output):
    """Output fields returned when fetching a location.
    LocationOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-OUTPUT)
    """
    pass

class LocationContext(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Context):
    """Context fields for understanding a location by an LLM.
    LocationContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationcontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-CONTEXT)
    """
    pass

class LocationPrediction(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Prediction):
    """Prediction fields for LLM-based location inference.
    LocationPrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationprediction](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-PREDICTION)
    """
    pass

class LocationNode(Node):
    """GraphQL node exposing location data.
    LocationNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-NODE)
    """
    class Meta:
        model = LocationOutput

class LocationInputNode(InputNode):
    """GraphQL input node for location mutations.
    LocationInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖location🛠️locationinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LOCATION/LOCATION-INPUT-NODE)
    """
    class Meta:
        model = LocationInput

# endregion Location

# region Author

# [👤semio📚py💻semiopy🔖domain🔖author](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/AUTHOR)
# Author entity for tracking contributor identity and rank.

class AuthorNameField(RealField, abc.ABC):
    """Field mixin for the name of a author.
    AuthorNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖author🛠️authornamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/AUTHOR/AUTHOR-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class AuthorEmailField(RealField, abc.ABC):
    """Field mixin for the email of a author.
    AuthorEmailField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖author🛠️authoremailfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/AUTHOR/AUTHOR-EMAIL-FIELD)
    """
    email: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class AuthorRankField(RealField, abc.ABC):
    """Field mixin for the rank of a author.
    AuthorRankField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖author🛠️authorrankfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/AUTHOR/AUTHOR-RANK-FIELD)
    """
    rank: int = sqlmodel.Field(default=0)

class AuthorId(AuthorEmailField, Id):
    """Identity fields for uniquely identifying a author.
    AuthorId MUST contain all fields that uniquely identify a author.
    [👤semio📚py💻semiopy🔖domain🔖author🛠️authorid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/AUTHOR/AUTHOR-ID)
    """
    pass

class AuthorProps(AuthorEmailField, AuthorNameField, Props):
    """Property fields for a author.
    AuthorProps MUST contain all non-relational property fields.
    [👤semio📚py💻semiopy🔖domain🔖author🛠️authorprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/AUTHOR/AUTHOR-PROPS)
    """
    pass

class AuthorInput(AuthorEmailField, AuthorNameField, Input):
    """Input fields for creating or updating a author.
    AuthorInput MUST contain all fields required for creation.
    [👤semio📚py💻semiopy🔖domain🔖author🛠️authorinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/AUTHOR/AUTHOR-INPUT)
    """
    pass

class AuthorOutput(AuthorEmailField, AuthorNameField, Output):
    """Output fields returned when fetching a author.
    AuthorOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖author🛠️authoroutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/AUTHOR/AUTHOR-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖author🛠️author](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/AUTHOR/AUTHOR)
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
    [👤semio📚py💻semiopy🔖domain🔖author🛠️authorinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/AUTHOR/AUTHOR-INPUT-NODE)
    """
    class Meta:
        model = AuthorInput

# endregion Author

# region ArtifactAuthor

# [👤semio📚py💻semiopy🔖domain🔖artifactauthor](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/ARTIFACT-AUTHOR)
# Artifact-author association entity linking artifacts to authors by email.

class ArtifactAuthorEmailField(RealField, abc.ABC):
    """Field mixin for the email of a artifact author.
    ArtifactAuthorEmailField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖artifactauthor🛠️artifactauthoremailfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ARTIFACT-AUTHOR/ARTIFACT-AUTHOR-EMAIL-FIELD)
    """
    author_email: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class ArtifactAuthor(ArtifactAuthorEmailField, TableEntity, table=True):
    """Association entity linking an artifact to an author by email.
    ArtifactAuthor MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semiopy🔖domain🔖artifactauthor🛠️artifactauthor](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/ARTIFACT-AUTHOR/ARTIFACT-AUTHOR)
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

# [👤semio📚py💻semiopy🔖domain🔖file](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/FILE)
# File entity for managing binary assets with metadata and hashing.

class FileGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a file.
    FileGuidField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️fileguidfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-GUID-FIELD)
    """
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class FileNameField(RealField, abc.ABC):
    """Field mixin for the name of a file.
    FileNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️filenamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class FileMimeField(RealField, abc.ABC):
    """Field mixin for the mime of a file.
    FileMimeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️filemimefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-MIME-FIELD)
    """
    mime: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)

class FileRemoteField(RealField, abc.ABC):
    """Field mixin for the remote of a file.
    FileRemoteField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️fileremotefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-REMOTE-FIELD)
    """
    remote: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class FileFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a file.
    FileFolderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️filefolderfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-FOLDER-FIELD)
    """
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class FileSizeField(RealField, abc.ABC):
    """Field mixin for the size of a file.
    FileSizeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️filesizefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-SIZE-FIELD)
    """
    size: typing.Optional[int] = sqlmodel.Field(default=None)

class FileHashField(RealField, abc.ABC):
    """Field mixin for the hash of a file.
    FileHashField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️filehashfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-HASH-FIELD)
    """
    hash: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)

class FileCreatedAtField(RealField, abc.ABC):
    """Field mixin for the created at of a file.
    FileCreatedAtField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️filecreatedatfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-CREATED-AT-FIELD)
    """
    createdAt: datetime.datetime = sqlmodel.Field()

class FileCreatedByField(RealField, abc.ABC):
    """Field mixin for the created by of a file.
    FileCreatedByField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️filecreatedbyfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-CREATED-BY-FIELD)
    """
    createdBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class FileUpdatedAtField(RealField, abc.ABC):
    """Field mixin for the updated at of a file.
    FileUpdatedAtField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️fileupdatedatfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-UPDATED-AT-FIELD)
    """
    updatedAt: datetime.datetime = sqlmodel.Field()

class FileUpdatedByField(RealField, abc.ABC):
    """Field mixin for the updated by of a file.
    FileUpdatedByField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️fileupdatedbyfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-UPDATED-BY-FIELD)
    """
    updatedBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class FileId(FileGuidField, Id):
    """Identity fields for uniquely identifying a file.
    FileId MUST contain all fields that uniquely identify a file.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️fileid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖file🛠️fileprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖file🛠️fileinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-INPUT)
    """
    pass

class FileContext(FileNameField, FileGuidField, Context):
    """Context fields for understanding a file by an LLM.
    FileContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖file🛠️filecontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-CONTEXT)
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
    [👤semio📚py💻semiopy🔖domain🔖file🛠️fileoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖file🛠️file](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE)
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
    [👤semio📚py💻semiopy🔖domain🔖file🛠️fileinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FILE/FILE-INPUT-NODE)
    """
    class Meta:
        model = FileInput

# endregion File

# region Folder

# [👤semio📚py💻semiopy🔖domain🔖folder](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER)
# Folder entity for hierarchical organization of kit content.

class FolderGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a folder.
    FolderGuidField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folderguidfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-GUID-FIELD)
    """
    guid: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class FolderNameField(RealField, abc.ABC):
    """Field mixin for the name of a folder.
    FolderNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️foldernamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class FolderParentField(RealField, abc.ABC):
    """Field mixin for the parent of a folder.
    FolderParentField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folderparentfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-PARENT-FIELD)
    """
    parent: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class FolderDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a folder.
    FolderDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folderdescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class FolderCreatedAtField(RealField, abc.ABC):
    """Field mixin for the created at of a folder.
    FolderCreatedAtField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️foldercreatedatfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-CREATED-AT-FIELD)
    """
    createdAt: datetime.datetime = sqlmodel.Field()

class FolderCreatedByField(RealField, abc.ABC):
    """Field mixin for the created by of a folder.
    FolderCreatedByField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️foldercreatedbyfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-CREATED-BY-FIELD)
    """
    createdBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class FolderUpdatedAtField(RealField, abc.ABC):
    """Field mixin for the updated at of a folder.
    FolderUpdatedAtField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folderupdatedatfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-UPDATED-AT-FIELD)
    """
    updatedAt: datetime.datetime = sqlmodel.Field()

class FolderUpdatedByField(RealField, abc.ABC):
    """Field mixin for the updated by of a folder.
    FolderUpdatedByField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folderupdatedbyfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-UPDATED-BY-FIELD)
    """
    updatedBy: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class FolderId(FolderGuidField, Id):
    """Identity fields for uniquely identifying a folder.
    FolderId MUST contain all fields that uniquely identify a folder.
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folderid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folderprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folderinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-INPUT)
    """
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)

class FolderContext(FolderNameField, FolderGuidField, Context):
    """Context fields for understanding a folder by an LLM.
    FolderContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️foldercontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-CONTEXT)
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
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folderoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folder](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER)
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
    [👤semio📚py💻semiopy🔖domain🔖folder🛠️folderinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FOLDER/FOLDER-INPUT-NODE)
    """
    class Meta:
        model = FolderInput

# endregion Folder

# region Benchmark

# [👤semio📚py💻semiopy🔖domain🔖benchmark](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK)
# Benchmark entity for defining performance metrics with min-max bounds.

class BenchmarkNameField(RealField, abc.ABC):
    """Field mixin for the name of a benchmark.
    BenchmarkNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmarknamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class BenchmarkIconField(RealField, abc.ABC):
    """Field mixin for the icon of a benchmark.
    BenchmarkIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmarkiconfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK-ICON-FIELD)
    """
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class BenchmarkMinField(RealField, abc.ABC):
    """Field mixin for the min of a benchmark.
    BenchmarkMinField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmarkminfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK-MIN-FIELD)
    """
    min: typing.Optional[float] = sqlmodel.Field(default=None)

class BenchmarkMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a benchmark.
    BenchmarkMinExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmarkminexcludedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK-MIN-EXCLUDED-FIELD)
    """
    min_excluded: bool = sqlmodel.Field(default=False)

class BenchmarkMaxField(RealField, abc.ABC):
    """Field mixin for the max of a benchmark.
    BenchmarkMaxField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmarkmaxfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK-MAX-FIELD)
    """
    max: typing.Optional[float] = sqlmodel.Field(default=None)

class BenchmarkMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a benchmark.
    BenchmarkMaxExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmarkmaxexcludedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK-MAX-EXCLUDED-FIELD)
    """
    max_excluded: bool = sqlmodel.Field(default=False)

class BenchmarkId(BenchmarkNameField, Id):
    """Identity fields for uniquely identifying a benchmark.
    BenchmarkId MUST contain all fields that uniquely identify a benchmark.
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmarkid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmarkprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmarkinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK-INPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmarkoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖benchmark🛠️benchmark](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/BENCHMARK/BENCHMARK)
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

# [👤semio📚py💻semiopy🔖domain🔖quality](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY)
# Quality entity for defining measurable properties with units and constraints.

class QualityKeyField(RealField, abc.ABC):
    """Field mixin for the key of a quality.
    QualityKeyField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualitykeyfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-KEY-FIELD)
    """
    key: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT, primary_key=True)

class QualityNameField(RealField, abc.ABC):
    """Field mixin for the name of a quality.
    QualityNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualitynamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class QualityDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a quality.
    QualityDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualitydescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class QualityUriField(RealField, abc.ABC):
    """Field mixin for the uri of a quality.
    QualityUriField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityurifield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-URI-FIELD)
    """
    uri: str = sqlmodel.Field(default="", max_length=URI_LENGTH_LIMIT)

class QualityScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a quality.
    QualityScalableField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityscalablefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-SCALABLE-FIELD)
    """
    scalable: bool = sqlmodel.Field(default=False)

class QualityKindField(RealField, abc.ABC):
    """Field mixin for the kind of a quality.
    QualityKindField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualitykindfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-KIND-FIELD)
    """
    kind: int = sqlmodel.Field(default=0)

class QualitySiField(RealField, abc.ABC):
    """Field mixin for the si of a quality.
    QualitySiField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualitysifield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-SI-FIELD)
    """
    si: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class QualityImperialField(RealField, abc.ABC):
    """Field mixin for the imperial of a quality.
    QualityImperialField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityimperialfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-IMPERIAL-FIELD)
    """
    imperial: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class QualityMinField(RealField, abc.ABC):
    """Field mixin for the min of a quality.
    QualityMinField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityminfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-MIN-FIELD)
    """
    min: typing.Optional[float] = sqlmodel.Field(default=None)

class QualityMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a quality.
    QualityMinExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityminexcludedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-MIN-EXCLUDED-FIELD)
    """
    min_excluded: bool = sqlmodel.Field(default=True)

class QualityMaxField(RealField, abc.ABC):
    """Field mixin for the max of a quality.
    QualityMaxField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualitymaxfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-MAX-FIELD)
    """
    max: typing.Optional[float] = sqlmodel.Field(default=None)

class QualityMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a quality.
    QualityMaxExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualitymaxexcludedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-MAX-EXCLUDED-FIELD)
    """
    max_excluded: bool = sqlmodel.Field(default=True)

class QualityDefaultField(RealField, abc.ABC):
    """Field mixin for the default of a quality.
    QualityDefaultField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualitydefaultfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-DEFAULT-FIELD)
    """
    default: typing.Optional[float] = sqlmodel.Field(default=None)

class QualityFormulaField(RealField, abc.ABC):
    """Field mixin for the formula of a quality.
    QualityFormulaField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityformulafield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-FORMULA-FIELD)
    """
    formula: str = sqlmodel.Field(default="", max_length=EXPRESSION_LENGTH_LIMIT)

class QualityFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a quality.
    QualityFolderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityfolderfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-FOLDER-FIELD)
    """
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)

class QualityIconField(RealField, abc.ABC):
    """Field mixin for the icon of a quality.
    QualityIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityiconfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-ICON-FIELD)
    """
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class QualityImageField(RealField, abc.ABC):
    """Field mixin for the image of a quality.
    QualityImageField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityimagefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-IMAGE-FIELD)
    """
    image: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class QualityUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a quality.
    QualityUnitField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityunitfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-UNIT-FIELD)
    """
    unit: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)

class QualityCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a quality.
    QualityCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualitycreatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-CREATED-FIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class QualityUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a quality.
    QualityUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityupdatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-UPDATED-FIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class QualityId(QualityKeyField, Id):
    """Identity fields for uniquely identifying a quality.
    QualityId MUST contain all fields that uniquely identify a quality.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-ID)
    """
    pass

class QualityProps(
    """Property fields for a quality.
    QualityProps MUST contain all non-relational property fields.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-INPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualitycontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-CONTEXT)
    """
    pass

class QualityOutput(
    """Output fields returned when fetching a quality.
    QualityOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️qualityoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖quality🛠️quality](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/QUALITY/QUALITY)
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

# [👤semio📚py💻semiopy🔖domain🔖prop](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/PROP)
# Prop entity for key-value property pairs with units.

class PropKeyField(RealField, abc.ABC):
    """Field mixin for the key of a prop.
    PropKeyField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️propkeyfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP-KEY-FIELD)
    """
    key: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class PropValueField(RealField, abc.ABC):
    """Field mixin for the value of a prop.
    PropValueField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️propvaluefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP-VALUE-FIELD)
    """
    value: str = sqlmodel.Field(max_length=VALUE_LENGTH_LIMIT)

class PropUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a prop.
    PropUnitField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️propunitfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP-UNIT-FIELD)
    """
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class PropCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a prop.
    PropCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️propcreatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP-CREATED-FIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class PropUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a prop.
    PropUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️propupdatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP-UPDATED-FIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class PropId(PropKeyField, Id):
    """Identity fields for uniquely identifying a prop.
    PropId MUST contain all fields that uniquely identify a prop.
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️propid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️propprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP-PROPS)
    """
    pass

class PropInput(PropUnitField, PropValueField, PropKeyField, Input):
    """Input fields for creating or updating a prop.
    PropInput MUST contain all fields required for creation.
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️propinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP-INPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️propoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️prop](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP)
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
    [👤semio📚py💻semiopy🔖domain🔖prop🛠️propinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PROP/PROP-INPUT-NODE)
    """
    class Meta:
        model = PropInput

# endregion Prop

# region Model

# [👤semio📚py💻semiopy🔖domain🔖model](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL)
# Model entity for 3D geometry representations linked to files.

class ModelNameField(RealField, abc.ABC):
    """Field mixin for the name of a model.
    ModelNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modelnamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-NAME-FIELD)
    """
    name: typing.Optional[str] = sqlmodel.Field(default=None, max_length=NAME_LENGTH_LIMIT)

class ModelUrlField(RealField, abc.ABC):
    """Field mixin for the url of a model.
    ModelUrlField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modelurlfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-URL-FIELD)
    """
    url: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT)

class ModelFileField(RealField, abc.ABC):
    """Field mixin for the file of a model.
    ModelFileField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modelfilefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-FILE-FIELD)
    """
    file: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)

class ModelDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a model.
    ModelDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modeldescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class ModelTagsField(MaskedField, abc.ABC):
    """Field mixin for the tags of a model.
    ModelTagsField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modeltagsfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-TAGS-FIELD)
    """
    tags: list[str] = sqlmodel.Field(default_factory=list)

class ModelId(ModelTagsField, Id):
    """Identity fields for uniquely identifying a model.
    ModelId MUST contain all fields that uniquely identify a model.
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modelid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modelprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modelinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-INPUT)
    """
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)

class ModelContext(ModelTagsField, ModelDescriptionField, ModelNameField, Context):
    """Context fields for understanding a model by an LLM.
    ModelContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modelcontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-CONTEXT)
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
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modeloutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖model🛠️model](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL)
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
    [👤semio📚py💻semiopy🔖domain🔖model🛠️nomodelassigned](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/NO-MODEL-ASSIGNED)
    """
    def __str__(self):
        return " The entity has no parent model assigned."

class ModelInputNode(InputNode):
    """GraphQL input node for model mutations.
    ModelInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖model🛠️modelinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MODEL/MODEL-INPUT-NODE)
    """
    class Meta:
        model = ModelInput

# endregion Model

# region Port

# [👤semio📚py💻semiopy🔖domain🔖port](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/PORT)
# Port entity for defining connection interfaces on types.

class PortNameField(RealField, abc.ABC):
    """Field mixin for the name of a port.
    PortNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖port🛠️portnamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PORT/PORT-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class PortDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a port.
    PortDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖port🛠️portdescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PORT/PORT-DESCRIPTION-FIELD)
    """
    description: typing.Optional[str] = sqlmodel.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)

class PortIconField(RealField, abc.ABC):
    """Field mixin for the icon of a port.
    PortIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖port🛠️porticonfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PORT/PORT-ICON-FIELD)
    """
    icon: typing.Optional[str] = sqlmodel.Field(default=None, max_length=URL_LENGTH_LIMIT)

class PortCompatiblePortsField(MaskedField, abc.ABC):
    """Field mixin for the compatible ports of a port.
    PortCompatiblePortsField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖port🛠️portcompatibleportsfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PORT/PORT-COMPATIBLE-PORTS-FIELD)
    """
    compatiblePorts: list[str] = sqlmodel.Field(default_factory=list)

class PortId(PortNameField, Id):
    """Identity fields for uniquely identifying a port.
    PortId MUST contain all fields that uniquely identify a port.
    [👤semio📚py💻semiopy🔖domain🔖port🛠️portid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PORT/PORT-ID)
    """
    pass

class PortProps(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Props):
    """Property fields for a port.
    PortProps MUST contain all non-relational property fields.
    [👤semio📚py💻semiopy🔖domain🔖port🛠️portprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PORT/PORT-PROPS)
    """
    pass

class PortInput(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Input):
    """Input fields for creating or updating a port.
    PortInput MUST contain all fields required for creation.
    [👤semio📚py💻semiopy🔖domain🔖port🛠️portinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PORT/PORT-INPUT)
    """
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)

class PortOutput(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Output):
    """Output fields returned when fetching a port.
    PortOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖port🛠️portoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PORT/PORT-OUTPUT)
    """
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)

class Port(PortIconField, PortDescriptionField, PortNameField, TableEntity, table=True):
    """Port entity defining a named connection interface on a type.
    Port MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semiopy🔖domain🔖port🛠️port](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PORT/PORT)
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
    [👤semio📚py💻semiopy🔖domain🔖port🛠️portinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PORT/PORT-INPUT-NODE)
    """
    class Meta:
        model = PortInput

# endregion Port

# region Connector

# [🔖semio/py/semio.py#Connector](semiorepo://section/semio/py/semio.py/CONNECTOR)

# region CompatiblePort

# [👤semio📚py💻semiopy🔖domain🔖connector](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR)
# Compatible port entity for specifying allowed port pairings on connectors.

class CompatiblePortNameField(RealField, abc.ABC):
    """Field mixin for the name of a compatible port.
    CompatiblePortNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connector🔖compatibleport🛠️compatibleportnamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/COMPATIBLE-PORT/COMPATIBLE-PORT-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class CompatiblePortOrderField(RealField, abc.ABC):
    """Field mixin for the order of a compatible port.
    CompatiblePortOrderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connector🔖compatibleport🛠️compatibleportorderfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/COMPATIBLE-PORT/COMPATIBLE-PORT-ORDER-FIELD)
    """
    order: int = sqlmodel.Field()

class CompatiblePort(CompatiblePortOrderField, CompatiblePortNameField, Table, table=True):
    """Compatible port entity specifying an allowed port pairing.
    CompatiblePort MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semiopy🔖domain🔖connector🔖compatibleport🛠️compatibleport](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/COMPATIBLE-PORT/COMPATIBLE-PORT)
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
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectoridfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-ID-FIELD)
    """
    id_: str = sqlmodel.Field(default="", max_length=ID_LENGTH_LIMIT)

class ConnectorDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a connector.
    ConnectorDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectordescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class ConnectorMandatoryField(RealField, abc.ABC):
    """Field mixin for the mandatory of a connector.
    ConnectorMandatoryField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectormandatoryfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-MANDATORY-FIELD)
    """
    is_mandatory: bool = sqlmodel.Field(default=False)

class ConnectorPortField(RealField, abc.ABC):
    """Field mixin for the port of a connector.
    ConnectorPortField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectorportfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-PORT-FIELD)
    """
    port: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class ConnectorCompatiblePortsField(MaskedField, abc.ABC):
    """Field mixin for the compatible ports of a connector.
    ConnectorCompatiblePortsField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectorcompatibleportsfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-COMPATIBLE-PORTS-FIELD)
    """
    compatiblePorts: list[str] = sqlmodel.Field(default_factory=list)

class ConnectorPointField(MaskedField, abc.ABC):
    """Field mixin for the point of a connector.
    ConnectorPointField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectorpointfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-POINT-FIELD)
    """
    point: Point = sqlmodel.Field()

class ConnectorDirectionField(MaskedField, abc.ABC):
    """Field mixin for the direction of a connector.
    ConnectorDirectionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectordirectionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-DIRECTION-FIELD)
    """
    direction: Vector = sqlmodel.Field()

class ConnectorTField(RealField, abc.ABC):
    """Field mixin for the t of a connector.
    ConnectorTField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectortfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-T-FIELD)
    """
    t: float = sqlmodel.Field(default=0.0)

class ConnectorId(ConnectorIdField, Id):
    """Identity fields for uniquely identifying a connector.
    ConnectorId MUST contain all fields that uniquely identify a connector.
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectorid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectorprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectorinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-INPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectorcontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-CONTEXT)
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
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectoroutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connector](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR)
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
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectornotfound](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-NOT-FOUND)
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
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectorinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-INPUT-NODE)
    """
    class Meta:
        model = ConnectorInput

class ConnectorIdInputNode(InputNode):
    """GraphQL input node for connector id mutations.
    ConnectorIdInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖connector🛠️connectoridinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTOR/CONNECTOR-ID-INPUT-NODE)
    """
    class Meta:
        model = ConnectorId

# endregion Connector

# region Type

# [👤semio📚py💻semiopy🔖domain🔖type](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE)
# Type entity for defining reusable parametric building blocks.

class TypeNameField(RealField, abc.ABC):
    """Field mixin for the name of a type.
    TypeNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typenamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class TypeDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a type.
    TypeDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typedescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class TypeIconField(RealField, abc.ABC):
    """Field mixin for the icon of a type.
    TypeIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeiconfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-ICON-FIELD)
    """
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class TypeImageField(RealField, abc.ABC):
    """Field mixin for the image of a type.
    TypeImageField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeimagefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-IMAGE-FIELD)
    """
    image: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class TypeVariantField(RealField, abc.ABC):
    """Field mixin for the variant of a type.
    TypeVariantField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typevariantfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-VARIANT-FIELD)
    """
    variant: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class TypeParentField(RealField, abc.ABC):
    """Field mixin for the parent of a type.
    TypeParentField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeparentfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-PARENT-FIELD)
    """
    parent: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class TypeIsAbstractField(RealField, abc.ABC):
    """Field mixin for the is abstract of a type.
    TypeIsAbstractField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeisabstractfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-IS-ABSTRACT-FIELD)
    """
    is_abstract: bool = sqlmodel.Field(default=False)

class TypeFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a type.
    TypeFolderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typefolderfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-FOLDER-FIELD)
    """
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class TypeStockField(RealField, abc.ABC):
    """Field mixin for the stock of a type.
    TypeStockField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typestockfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-STOCK-FIELD)
    """
    stock: int = sqlmodel.Field(default=2147483647)

class TypeVirtualField(RealField, abc.ABC):
    """Field mixin for the virtual of a type.
    TypeVirtualField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typevirtualfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-VIRTUAL-FIELD)
    """
    is_virtual: bool = sqlmodel.Field(default=False)

class TypeScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a type.
    TypeScalableField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typescalablefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-SCALABLE-FIELD)
    """
    can_scale: bool = sqlmodel.Field(default=True)

class TypeMirrborableField(RealField, abc.ABC):
    """Field mixin for the mirrborable of a type.
    TypeMirrborableField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typemirrborablefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-MIRRBORABLE-FIELD)
    """
    can_mirror: bool = sqlmodel.Field(default=True)

class TypeUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a type.
    TypeUnitField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeunitfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-UNIT-FIELD)
    """
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class TypeLocationField(MaskedField, abc.ABC):
    """Field mixin for the location of a type.
    TypeLocationField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typelocationfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-LOCATION-FIELD)
    """
    location: typing.Optional[Location] = sqlmodel.Field(default=None)

class TypeCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a type.
    TypeCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typecreatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-CREATED-FIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class TypeUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a type.
    TypeUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeupdatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-UPDATED-FIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class TypeId(TypeVariantField, TypeNameField, Id):
    """Identity fields for uniquely identifying a type.
    TypeId MUST contain all fields that uniquely identify a type.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-INPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typecontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-CONTEXT)
    """
    location: typing.Optional[LocationContext] = sqlmodel.Field(default=None)
    connectors: list[ConnectorContext] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)

class Type(
    """Type entity defining a reusable parametric building block.
    Type MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️type](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE)
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
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typenotfound](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-NOT-FOUND)
    """
    def __init__(self, id: "TypeId") -> None:
        self.id = id

    def __str__(self):
        variant = f", {self.id.variant}" if self.id.variant else ""
        return f"Couldn't find the type ({self.id.name}{variant})."

class NoTypeAssigned(NoParentAssigned):
    """No Type Assigned definition.
    NoTypeAssigned MUST fulfill its documented contract.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️notypeassigned](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/NO-TYPE-ASSIGNED)
    """
    def __str__(self):
        return " The entity has no parent type assigned."

class TypeHasNotAllUsedConnectors(SpecificationError):
    """Type Has Not All Used Connectors definition.
    TypeHasNotAllUsedConnectors MUST fulfill its documented contract.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typehasnotallusedconnectors](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-HAS-NOT-ALL-USED-CONNECTORS)
    """
    def __init__(self, missingConnectors: set[str]) -> None:
        self.missingConnectors = missingConnectors

    def __str__(self) -> str:
        return f" A design is using some connectors of the type. The new type is missing the following connectors: {', '.join(self.missingConnectors)}."

class TypeInputNode(InputNode):
    """GraphQL input node for type mutations.
    TypeInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-INPUT-NODE)
    """
    class Meta:
        model = TypeInput

class TypeIdInputNode(InputNode):
    """GraphQL input node for type id mutations.
    TypeIdInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖type🛠️typeidinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/TYPE/TYPE-ID-INPUT-NODE)
    """
    class Meta:
        model = TypeId

# endregion Type

# region Layer

# [👤semio📚py💻semiopy🔖domain🔖layer](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER)
# Layer entity for organizing design elements into visibility groups.

class LayerNameField(RealField, abc.ABC):
    """Field mixin for the name of a layer.
    LayerNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖layer🛠️layernamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER/LAYER-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class LayerDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a layer.
    LayerDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖layer🛠️layerdescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER/LAYER-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class LayerColorField(RealField, abc.ABC):
    """Field mixin for the color of a layer.
    LayerColorField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖layer🛠️layercolorfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER/LAYER-COLOR-FIELD)
    """
    color: str = sqlmodel.Field(default="", max_length=7)

class LayerIsHiddenField(RealField, abc.ABC):
    """Field mixin for the is hidden of a layer.
    LayerIsHiddenField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖layer🛠️layerishiddenfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER/LAYER-IS-HIDDEN-FIELD)
    """
    is_hidden: bool = sqlmodel.Field(default=False)

class LayerIsLockedField(RealField, abc.ABC):
    """Field mixin for the is locked of a layer.
    LayerIsLockedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖layer🛠️layerislockedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER/LAYER-IS-LOCKED-FIELD)
    """
    is_locked: bool = sqlmodel.Field(default=False)

class LayerId(LayerNameField, Id):
    """Identity fields for uniquely identifying a layer.
    LayerId MUST contain all fields that uniquely identify a layer.
    [👤semio📚py💻semiopy🔖domain🔖layer🛠️layerid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER/LAYER-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖layer🛠️layerprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER/LAYER-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖layer🛠️layerinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER/LAYER-INPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖layer🛠️layeroutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER/LAYER-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖layer🛠️layer](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/LAYER/LAYER)
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

# [👤semio📚py💻semiopy🔖domain🔖piece](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE)
# Piece entity for placed instances of types within a design.

class PieceIdField(MaskedField, abc.ABC):
    """Field mixin for the id of a piece.
    PieceIdField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️pieceidfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-ID-FIELD)
    """
    id_: str = sqlmodel.Field(
        default="",
        max_length=ID_LENGTH_LIMIT,
    )

class PieceDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a piece.
    PieceDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piecedescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class PieceTypeField(MaskedField, abc.ABC):
    """Field mixin for the type of a piece.
    PieceTypeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piecetypefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-TYPE-FIELD)
    """
    type: typing.Optional[TypeId] = sqlmodel.Field(default=None)

class PieceDesignField(MaskedField, abc.ABC):
    """Field mixin for the design of a piece.
    PieceDesignField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piecedesignfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-DESIGN-FIELD)
    """
    designPiece: typing.Optional["DesignId"] = sqlmodel.Field(default=None)

class PiecePlaneField(MaskedField, abc.ABC):
    """Field mixin for the plane of a piece.
    PiecePlaneField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️pieceplanefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-PLANE-FIELD)
    """
    plane: typing.Optional[Plane] = sqlmodel.Field(default=None)

class PieceCenterField(MaskedField, abc.ABC):
    """Field mixin for the center of a piece.
    PieceCenterField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piececenterfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-CENTER-FIELD)
    """
    center: typing.Optional[Coord] = sqlmodel.Field(default=None)

class PieceScaleField(RealField, abc.ABC):
    """Field mixin for the scale of a piece.
    PieceScaleField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piecescalefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-SCALE-FIELD)
    """
    scale: float = sqlmodel.Field(default=1.0)

class PieceMirrorPlaneField(MaskedField, abc.ABC):
    """Field mixin for the mirror plane of a piece.
    PieceMirrorPlaneField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piecemirrorplanefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-MIRROR-PLANE-FIELD)
    """
    mirrorPlane: typing.Optional[Plane] = sqlmodel.Field(default=None)

class PieceHiddenField(RealField, abc.ABC):
    """Field mixin for the hidden of a piece.
    PieceHiddenField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piecehiddenfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-HIDDEN-FIELD)
    """
    is_hidden: bool = sqlmodel.Field(default=False)

class PieceLockedField(RealField, abc.ABC):
    """Field mixin for the locked of a piece.
    PieceLockedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piecelockedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-LOCKED-FIELD)
    """
    is_locked: bool = sqlmodel.Field(default=False)

class PieceColorField(RealField, abc.ABC):
    """Field mixin for the color of a piece.
    PieceColorField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piececolorfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-COLOR-FIELD)
    """
    color: str = sqlmodel.Field(default="", max_length=7)

class PieceId(PieceIdField, Id):
    """Identity fields for uniquely identifying a piece.
    PieceId MUST contain all fields that uniquely identify a piece.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️pieceid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️pieceprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-PROPS)
    """
    pass

class PieceInput(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Input):
    """Input fields for creating or updating a piece.
    PieceInput MUST contain all fields required for creation.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️pieceinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-INPUT)
    """
    plane: typing.Optional[PlaneInput] = sqlmodel.Field(default=None)
    center: typing.Optional[CoordInput] = sqlmodel.Field(default=None)
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)

class PieceContext(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Context):
    """Context fields for understanding a piece by an LLM.
    PieceContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piececontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-CONTEXT)
    """
    plane: typing.Optional[PlaneContext] = sqlmodel.Field(default=None)
    center: typing.Optional[CoordContext] = sqlmodel.Field(default=None)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)

class PieceOutput(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Output):
    """Output fields returned when fetching a piece.
    PieceOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️pieceoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-OUTPUT)
    """
    plane: typing.Optional[PlaneOutput] = sqlmodel.Field(default=None)
    center: typing.Optional[CoordOutput] = sqlmodel.Field(default=None)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)

class PiecePrediction(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Prediction):
    """Prediction fields for LLM-based piece inference.
    PiecePrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️pieceprediction](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-PREDICTION)
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
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️piece](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE)
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
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️pieceinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-INPUT-NODE)
    """
    class Meta:
        model = PieceInput
        exclude_fields = ("type", "designPiece")

    type = TypeIdInputNode()
    designPiece = graphene.Field(lambda: DesignIdInputNode)

class PieceIdInputNode(InputNode):
    """GraphQL input node for piece id mutations.
    PieceIdInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖piece🛠️pieceidinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/PIECE/PIECE-ID-INPUT-NODE)
    """
    class Meta:
        model = PieceId

# endregion Piece

# region Group

# [👤semio📚py💻semiopy🔖domain🔖group](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/GROUP)
# Group entity for named collections of pieces in a design.

class GroupNameField(RealField, abc.ABC):
    """Field mixin for the name of a group.
    GroupNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖group🛠️groupnamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GROUP/GROUP-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class GroupDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a group.
    GroupDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖group🛠️groupdescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GROUP/GROUP-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class GroupColorField(RealField, abc.ABC):
    """Field mixin for the color of a group.
    GroupColorField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖group🛠️groupcolorfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GROUP/GROUP-COLOR-FIELD)
    """
    color: str = sqlmodel.Field(default="", max_length=7)

class GroupId(GroupNameField, Id):
    """Identity fields for uniquely identifying a group.
    GroupId MUST contain all fields that uniquely identify a group.
    [👤semio📚py💻semiopy🔖domain🔖group🛠️groupid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GROUP/GROUP-ID)
    """
    pass

class GroupProps(GroupColorField, GroupDescriptionField, GroupNameField, Props):
    """Property fields for a group.
    GroupProps MUST contain all non-relational property fields.
    [👤semio📚py💻semiopy🔖domain🔖group🛠️groupprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GROUP/GROUP-PROPS)
    """
    pass

class GroupInput(GroupColorField, GroupDescriptionField, GroupNameField, Input):
    """Input fields for creating or updating a group.
    GroupInput MUST contain all fields required for creation.
    [👤semio📚py💻semiopy🔖domain🔖group🛠️groupinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GROUP/GROUP-INPUT)
    """
    pass

class GroupOutput(GroupColorField, GroupDescriptionField, GroupNameField, Output):
    """Output fields returned when fetching a group.
    GroupOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖group🛠️groupoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GROUP/GROUP-OUTPUT)
    """
    pieces: list["PieceOutput"] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)

class Group(GroupColorField, GroupDescriptionField, GroupNameField, TableEntity, table=True):
    """Group entity for named collections of pieces.
    Group MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semiopy🔖domain🔖group🛠️group](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GROUP/GROUP)
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

# [👤semio📚py💻semiopy🔖domain🔖side](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/SIDE)
# Side primitive for identifying a specific connector on a specific piece.

class Side(BaseModel):
    """Side primitive identifying a specific connector on a specific piece.
    Side MUST contain all coordinate or geometry fields.
    [👤semio📚py💻semiopy🔖domain🔖side🛠️side](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/SIDE/SIDE)
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
    [👤semio📚py💻semiopy🔖domain🔖side🛠️sideinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/SIDE/SIDE-INPUT)
    """
    pass

class SideContext(Side, Context):
    """Context fields for understanding a side by an LLM.
    SideContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semiopy🔖domain🔖side🛠️sidecontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/SIDE/SIDE-CONTEXT)
    """
    pass

class SideOutput(Side, Output):
    """Output fields returned when fetching a side.
    SideOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semiopy🔖domain🔖side🛠️sideoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/SIDE/SIDE-OUTPUT)
    """
    pass

class SidePrediction(Side, Prediction):
    """Prediction fields for LLM-based side inference.
    SidePrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semiopy🔖domain🔖side🛠️sideprediction](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/SIDE/SIDE-PREDICTION)
    """
    pass

class SideNode(Node):
    """GraphQL node exposing side data.
    SideNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖side🛠️sidenode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/SIDE/SIDE-NODE)
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
    [👤semio📚py💻semiopy🔖domain🔖side🛠️sideinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/SIDE/SIDE-INPUT-NODE)
    """
    class Meta:
        model = SideInput

    exclude_fields = ("piece", "connector")

    piece = graphene.NonNull(PieceIdInputNode)
    designPiece = PieceIdInputNode()
    connector = graphene.NonNull(ConnectorIdInputNode)

# endregion Side

# region Connection

# [👤semio📚py💻semiopy🔖domain🔖connection](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION)
# Connection entity for linking two pieces through their connectors.

class ConnectionConnectedField(MaskedField, abc.ABC):
    """Field mixin for the connected of a connection.
    ConnectionConnectedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionconnectedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-CONNECTED-FIELD)
    """
    connected: Side = sqlmodel.Field()

class ConnectionConnectingField(MaskedField, abc.ABC):
    """Field mixin for the connecting of a connection.
    ConnectionConnectingField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionconnectingfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-CONNECTING-FIELD)
    """
    connecting: Side = sqlmodel.Field()

class ConnectionDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a connection.
    ConnectionDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectiondescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class ConnectionGapField(RealField, abc.ABC):
    """Field mixin for the gap of a connection.
    ConnectionGapField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectiongapfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-GAP-FIELD)
    """
    gap: float = sqlmodel.Field(default=0)

class ConnectionShiftField(RealField, abc.ABC):
    """Field mixin for the shift of a connection.
    ConnectionShiftField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionshiftfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-SHIFT-FIELD)
    """
    shift: float = sqlmodel.Field(default=0)

class ConnectionRiseField(MaskedField, abc.ABC):
    """Field mixin for the rise of a connection.
    ConnectionRiseField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionrisefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-RISE-FIELD)
    """
    rise: float = sqlmodel.Field(default=0)

class ConnectionRotationField(RealField, abc.ABC):
    """Field mixin for the rotation of a connection.
    ConnectionRotationField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionrotationfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-ROTATION-FIELD)
    """
    rotation: float = sqlmodel.Field(ge=0, lt=360, default=0)

class ConnectionTurnField(RealField, abc.ABC):
    """Field mixin for the turn of a connection.
    ConnectionTurnField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionturnfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-TURN-FIELD)
    """
    turn: float = sqlmodel.Field(ge=0, lt=360, default=0)

class ConnectionTiltField(RealField, abc.ABC):
    """Field mixin for the tilt of a connection.
    ConnectionTiltField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectiontiltfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-TILT-FIELD)
    """
    tilt: float = sqlmodel.Field(ge=0, lt=360, default=0)

class ConnectionUField(RealField, abc.ABC):
    """Field mixin for the u of a connection.
    ConnectionUField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionufield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-U-FIELD)
    """
    u: float = sqlmodel.Field(default=0)

class ConnectionVField(RealField, abc.ABC):
    """Field mixin for the v of a connection.
    ConnectionVField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionvfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-V-FIELD)
    """
    v: float = sqlmodel.Field(default=0)

class ConnectionId(ConnectionConnectedField, ConnectionConnectingField, Id):
    """Identity fields for uniquely identifying a connection.
    ConnectionId MUST contain all fields that uniquely identify a connection.
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectioninput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-INPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectioncontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-CONTEXT)
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
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectionprediction](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-PREDICTION)
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
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connection](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION)
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
    [👤semio📚py💻semiopy🔖domain🔖connection🛠️connectioninputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/CONNECTION/CONNECTION-INPUT-NODE)
    """
    class Meta:
        model = ConnectionInput

# endregion Connection

# region Stat

# [👤semio📚py💻semiopy🔖domain🔖stat](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/STAT)
# Stat entity for recording computed statistics with bounds.

class StatKeyField(RealField, abc.ABC):
    """Field mixin for the key of a stat.
    StatKeyField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statkeyfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-KEY-FIELD)
    """
    key: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class StatUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a stat.
    StatUnitField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statunitfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-UNIT-FIELD)
    """
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class StatMinField(RealField, abc.ABC):
    """Field mixin for the min of a stat.
    StatMinField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statminfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-MIN-FIELD)
    """
    min: typing.Optional[float] = sqlmodel.Field(default=None)

class StatMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a stat.
    StatMinExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statminexcludedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-MIN-EXCLUDED-FIELD)
    """
    min_excluded: bool = sqlmodel.Field(default=False)

class StatMaxField(RealField, abc.ABC):
    """Field mixin for the max of a stat.
    StatMaxField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statmaxfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-MAX-FIELD)
    """
    max: typing.Optional[float] = sqlmodel.Field(default=None)

class StatMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a stat.
    StatMaxExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statmaxexcludedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-MAX-EXCLUDED-FIELD)
    """
    max_excluded: bool = sqlmodel.Field(default=False)

class StatCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a stat.
    StatCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statcreatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-CREATED-FIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class StatUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a stat.
    StatUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statupdatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-UPDATED-FIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class StatId(StatKeyField, Id):
    """Identity fields for uniquely identifying a stat.
    StatId MUST contain all fields that uniquely identify a stat.
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-INPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️statoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖stat🛠️stat](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/STAT/STAT)
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

# [👤semio📚py💻semiopy🔖domain🔖design](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN)
# Design entity for composing pieces and connections into assemblies.

class DesignNameField(RealField, abc.ABC):
    """Field mixin for the name of a design.
    DesignNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designnamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class DesignDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a design.
    DesignDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designdescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class DesignIconField(RealField, abc.ABC):
    """Field mixin for the icon of a design.
    DesignIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designiconfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-ICON-FIELD)
    """
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class DesignImageField(RealField, abc.ABC):
    """Field mixin for the image of a design.
    DesignImageField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designimagefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-IMAGE-FIELD)
    """
    image: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class DesignVariantField(RealField, abc.ABC):
    """Field mixin for the variant of a design.
    DesignVariantField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designvariantfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-VARIANT-FIELD)
    """
    variant: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class DesignViewField(RealField, abc.ABC):
    """Field mixin for the view of a design.
    DesignViewField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designviewfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-VIEW-FIELD)
    """
    view: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class DesignParentField(RealField, abc.ABC):
    """Field mixin for the parent of a design.
    DesignParentField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designparentfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-PARENT-FIELD)
    """
    parent: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class DesignIsAbstractField(RealField, abc.ABC):
    """Field mixin for the is abstract of a design.
    DesignIsAbstractField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designisabstractfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-IS-ABSTRACT-FIELD)
    """
    is_abstract: bool = sqlmodel.Field(default=False)

class DesignFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a design.
    DesignFolderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designfolderfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-FOLDER-FIELD)
    """
    folder: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class DesignActiveLayerField(RealField, abc.ABC):
    """Field mixin for the active layer of a design.
    DesignActiveLayerField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designactivelayerfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-ACTIVE-LAYER-FIELD)
    """
    activeLayer: typing.Optional[str] = sqlmodel.Field(default=None, max_length=ID_LENGTH_LIMIT)

class DesignLocationField(MaskedField, abc.ABC):
    """Field mixin for the location of a design.
    DesignLocationField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designlocationfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-LOCATION-FIELD)
    """
    location: typing.Optional[Location] = sqlmodel.Field(default=None)

class DesignUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a design.
    DesignUnitField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designunitfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-UNIT-FIELD)
    """
    unit: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class DesignScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a design.
    DesignScalableField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designscalablefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-SCALABLE-FIELD)
    """
    can_scale: bool = sqlmodel.Field(default=True)

class DesignMirrorableField(RealField, abc.ABC):
    """Field mixin for the mirrorable of a design.
    DesignMirrorableField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designmirrorablefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-MIRRORABLE-FIELD)
    """
    can_mirror: bool = sqlmodel.Field(default=True)

class DesignCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a design.
    DesignCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designcreatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-CREATED-FIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class DesignUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a design.
    DesignUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designupdatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-UPDATED-FIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class DesignId(DesignNameField, DesignVariantField, Id):
    """Identity fields for uniquely identifying a design.
    DesignId MUST contain all fields that uniquely identify a design.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-INPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designcontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-CONTEXT)
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
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designprediction](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-PREDICTION)
    """
    pass

    pieces: list[PiecePrediction] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionPrediction] = sqlmodel.Field(default_factory=list)

class Design(
    """Design entity composing pieces and connections into an assembly.
    Design MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️design](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN)
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
    [👤semio📚py💻semiopy🔖domain🔖design🛠️nodesignassigned](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/NO-DESIGN-ASSIGNED)
    """
    def __str__(self):
        return "👪 The entity has no parent design assigned."

class DesignInputNode(InputNode):
    """GraphQL input node for design mutations.
    DesignInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-INPUT-NODE)
    """
    class Meta:
        model = DesignInput

class DesignIdInputNode(InputNode):
    """GraphQL input node for design id mutations.
    DesignIdInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖design🛠️designidinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/DESIGN/DESIGN-ID-INPUT-NODE)
    """
    class Meta:
        model = DesignId

# endregion Design

# region Kit

# [👤semio📚py💻semiopy🔖domain🔖kit](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/KIT)
# Kit entity for packaging types, designs, qualities and metadata.

class KitUriField(RealField, abc.ABC):
    """Field mixin for the uri of a kit.
    KitUriField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kiturifield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-URI-FIELD)
    """
    uri: str = sqlmodel.Field(max_length=URI_LENGTH_LIMIT)

class KitNameField(RealField, abc.ABC):
    """Field mixin for the name of a kit.
    KitNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitnamefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-NAME-FIELD)
    """
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

class KitDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a kit.
    KitDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitdescriptionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-DESCRIPTION-FIELD)
    """
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)

class KitIconField(RealField, abc.ABC):
    """Field mixin for the icon of a kit.
    KitIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kiticonfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-ICON-FIELD)
    """
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitImageField(RealField, abc.ABC):
    """Field mixin for the image of a kit.
    KitImageField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitimagefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-IMAGE-FIELD)
    """
    image: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitPreviewField(RealField, abc.ABC):
    """Field mixin for the preview of a kit.
    KitPreviewField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitpreviewfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-PREVIEW-FIELD)
    """
    preview: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitVersionField(RealField, abc.ABC):
    """Field mixin for the version of a kit.
    KitVersionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitversionfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-VERSION-FIELD)
    """
    version: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)

class KitRemoteField(RealField, abc.ABC):
    """Field mixin for the remote of a kit.
    KitRemoteField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitremotefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-REMOTE-FIELD)
    """
    remote: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitHomepageField(RealField, abc.ABC):
    """Field mixin for the homepage of a kit.
    KitHomepageField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kithomepagefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-HOMEPAGE-FIELD)
    """
    homepage: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitLicenseField(RealField, abc.ABC):
    """Field mixin for the license of a kit.
    KitLicenseField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitlicensefield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-LICENSE-FIELD)
    """
    license: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)

class KitCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a kit.
    KitCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitcreatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-CREATED-FIELD)
    """
    created: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class KitUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a kit.
    KitUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitupdatedfield](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-UPDATED-FIELD)
    """
    updated: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)

class KitId(KitUriField, Id):
    """Identity fields for uniquely identifying a kit.
    KitId MUST contain all fields that uniquely identify a kit.
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-ID)
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
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitprops](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-PROPS)
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
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitinput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-INPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitcontext](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-CONTEXT)
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
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kitoutput](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT-OUTPUT)
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
    [👤semio📚py💻semiopy🔖domain🔖kit🛠️kit](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/KIT)
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

# [👤semio📚py💻semiopy🔖domain🔖kit🔖designfamilyhelpers](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/DESIGN-FAMILY-HELPERS)
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

# [👤semio📚py💻semiopy🔖domain🔖kit🔖typefamilyhelpers](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/KIT/TYPE-FAMILY-HELPERS)
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

# [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES)
# Graphene node definitions moved here due to forward-reference resolution order.

class AttributeNode(TableEntityNode):
    """GraphQL node exposing attribute data.
    AttributeNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️attributenode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/ATTRIBUTE-NODE)
    """
    class Meta:
        model = Attribute

class PlaneNode(TableNode):
    """GraphQL node exposing plane data.
    PlaneNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️planenode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/PLANE-NODE)
    """
    class Meta:
        model = Plane

class AuthorNode(TableEntityNode):
    """GraphQL node exposing author data.
    AuthorNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️authornode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/AUTHOR-NODE)
    """
    class Meta:
        model = Author

class ModelNode(TableEntityNode):
    """GraphQL node exposing model data.
    ModelNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️modelnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/MODEL-NODE)
    """
    class Meta:
        model = Model
        excludedFields = ("tags_",)

class ConnectorNode(TableEntityNode):
    """GraphQL node exposing connector data.
    ConnectorNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️connectornode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/CONNECTOR-NODE)
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
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️typenode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/TYPE-NODE)
    """
    class Meta:
        model = Type

class PieceNode(TableEntityNode):
    """GraphQL node exposing piece data.
    PieceNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️piecenode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/PIECE-NODE)
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
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️connectionnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/CONNECTION-NODE)
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
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️designnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/DESIGN-NODE)
    """
    class Meta:
        model = Design

class KitNotFound(NotFound):
    """endregion Moved Graphene Nodes
    KitNotFound MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️kitnotfound](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/KIT-NOT-FOUND)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't find an local or remote kit under uri:\n {self.uri}."

class NoKitToDelete(KitNotFound):
    """No Kit To Delete definition.
    NoKitToDelete MUST fulfill its documented contract.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️nokittodelete](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/NO-KIT-TO-DELETE)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't delete the kit because no local or remote kit was found under uri:\n {self.uri}."

class KitZipDoesNotContainSemioFolder(KitNotFound):
    """Kit Zip Does Not Contain Semio Folder definition.
    KitZipDoesNotContainSemioFolder MUST fulfill its documented contract.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️kitzipdoesnotcontainsemiofolder](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/KIT-ZIP-DOES-NOT-CONTAIN-SEMIO-FOLDER)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 The remote zip kit ({self.uri}) is not a valid kit."

class OnlyRemoteKitsCanBeCached(ClientError):
    """Only Remote Kits Can Be Cached definition.
    OnlyRemoteKitsCanBeCached MUST fulfill its documented contract.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️onlyremotekitscanbecached](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/ONLY-REMOTE-KITS-CAN-BE-CACHED)
    """
    def __init__(self, nonRemoteUri: str) -> None:
        self.nonRemoteUri = nonRemoteUri

    def __str__(self):
        return f"🔍 Only remote kits can be cached. The uri ({self.nonRemoteUri}) doesn't start with http and ends with .zip"

class KitUriNotValid(ClientError, abc.ABC):
    """🆔 The base for all kit uri not valid errors.
    KitUriNotValid MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️kiturinotvalid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/KIT-URI-NOT-VALID)
    """

class LocalKitUriNotValid(KitUriNotValid, abc.ABC):
    """📂 The base for all local kit uri not valid errors.
    LocalKitUriNotValid MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️localkiturinotvalid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/LOCAL-KIT-URI-NOT-VALID)
    """

class LocalKitUriIsNotAbsolute(LocalKitUriNotValid):
    """Local Kit Uri Is Not Absolute definition.
    LocalKitUriIsNotAbsolute MUST fulfill its documented contract.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️localkituriisnotabsolute](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/LOCAL-KIT-URI-IS-NOT-ABSOLUTE)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"📂 The local kit uri ({self.uri}) is relative. It needs to be absolute (include the parent folders, drives, ...)."

class LocalKitUriIsNotDirectory(LocalKitUriNotValid):
    """Local Kit Uri Is Not Directory definition.
    LocalKitUriIsNotDirectory MUST fulfill its documented contract.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️localkituriisnotdirectory](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/LOCAL-KIT-URI-IS-NOT-DIRECTORY)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"📂 The local kit uri ({self.uri}) is not a directory."

class NoKitAssigned(NoParentAssigned):
    """No Kit Assigned definition.
    NoKitAssigned MUST fulfill its documented contract.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️nokitassigned](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/NO-KIT-ASSIGNED)
    """
    def __str__(self):
        return "👪 The entity has no parent kit assigned."

class KitAlreadyExists(AlreadyExists, abc.ABC):
    """Exception for attempting to create a kit that already exists.
    KitAlreadyExists MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️kitalreadyexists](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/KIT-ALREADY-EXISTS)
    """
    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self) -> str:
        return f"♊ A kit under uri ({self.uri}) already exists."

class KitInputNode(InputNode):
    """GraphQL input node for kit mutations.
    KitInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️kitinputnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/KIT-INPUT-NODE)
    """
    class Meta:
        model = KitInput

class KitNode(TableEntityNode):
    """GraphQL node exposing kit data.
    KitNode MUST expose the model via Meta.
    [👤semio📚py💻semiopy🔖domain🔖movedgraphenenodes🛠️kitnode](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/MOVED-GRAPHENE-NODES/KIT-NODE)
    """
    class Meta:
        model = Kit

# #endregion 🔖Moved Graphene Nodes

# region Validation

# [👤semio📚py💻semiopy🔖domain🔖validation](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION)
# Validation logic for checking kit constraints and uniqueness rules.

@dataclasses.dataclass
class ValidationFix:
    """A proposed fix for a validation problem with a title and diff.
    ValidationFix MUST contain a non-empty title and a valid diff dictionary.
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validationfix](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATION-FIX)
    """
    title: str
    diff: dict

    def toDict(self) -> dict:
        return {"title": self.title, "diff": self.diff}

@dataclasses.dataclass
class Problem:
    """A validation problem with a constraint identifier and message.
    Problem MUST contain a non-empty constraint identifier and message.
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️problem](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/PROBLEM)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validationresult](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATION-RESULT)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️arevalidationresultsequal](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/ARE-VALIDATION-RESULTS-EQUAL)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️parsevalidationresult](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/PARSE-VALIDATION-RESULT)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validateguiduniqueness](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-GUID-UNIQUENESS)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validatetypenameuniqueness](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-TYPE-NAME-UNIQUENESS)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validatedesignnameuniqueness](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-DESIGN-NAME-UNIQUENESS)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validatepiecenameuniqueness](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-PIECE-NAME-UNIQUENESS)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validateportnameuniqueness](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-PORT-NAME-UNIQUENESS)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validatemodelnameuniqueness](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-MODEL-NAME-UNIQUENESS)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validatequalitynameuniqueness](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-QUALITY-NAME-UNIQUENESS)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validatefilenameuniqueness](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-FILE-NAME-UNIQUENESS)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validatefoldernameuniqueness](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-FOLDER-NAME-UNIQUENESS)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validatelayerpathuniqueness](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-LAYER-PATH-UNIQUENESS)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🛠️validatekit](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/VALIDATE-KIT)
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

# [👤semio📚py💻semiopy🔖domain🔖validation🔖dictbasedvalidation](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/DICT-BASED-VALIDATION)
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
    [👤semio📚py💻semiopy🔖domain🔖validation🔖dictbasedvalidation🛠️validatekitdict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/VALIDATION/DICT-BASED-VALIDATION/VALIDATE-KIT-DICT)
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

# [👤semio📚py💻semiopy🔖domain🔖graphoperations](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/GRAPH-OPERATIONS)
# Graph construction and traversal for piece connectivity analysis.

def buildPieceGraph(design: Design | dict) -> networkx.Graph:
    """Build a networkx graph from pieces and connections.
    buildPieceGraph MUST return a networkx graph with pieces as nodes.
    [👤semio📚py💻semiopy🔖domain🔖graphoperations🛠️buildpiecegraph](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GRAPH-OPERATIONS/BUILD-PIECE-GRAPH)
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
    [👤semio📚py💻semiopy🔖domain🔖graphoperations🛠️findfixedpieces](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GRAPH-OPERATIONS/FIND-FIXED-PIECES)
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
    [👤semio📚py💻semiopy🔖domain🔖graphoperations🛠️getconnectedcomponents](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GRAPH-OPERATIONS/GET-CONNECTED-COMPONENTS)
    """
    G = buildPieceGraph(design)
    return [set(c) for c in networkx.connected_components(G)]

def getPieceHierarchy(design: Design | dict, rootGuid: str) -> dict[str, int]:
    """Get the hierarchical ordering of pieces from root to leaf.
    getPieceHierarchy MUST return a topological ordering of pieces.
    [👤semio📚py💻semiopy🔖domain🔖graphoperations🛠️getpiecehierarchy](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/GRAPH-OPERATIONS/GET-PIECE-HIERARCHY)
    """
    G = buildPieceGraph(design)
    if rootGuid not in G:
        return {}
    return networkx.single_source_shortest_path_length(G, rootGuid)

# endregion Graph Operations

# region FlattenDesign

# [👤semio📚py💻semiopy🔖domain🔖flattendesign](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN)
# Design flattening to resolve nested sub-designs into a single coordinate space.

def getTypeByGuid(kit: dict, guid: str) -> dict | None:
    """Look up a type by its GUID within a kit dictionary.
    getTypeByGuid MUST return the type dict or raise if not found.
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️gettypebyguid](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/GET-TYPE-BY-GUID)
    """
    for t in kit.get("types", []):
        if t.get("guid") == guid:
            return t
    return None

def getConnectorFromType(kit: dict, typeData: dict | None, connectorGuid: str | None) -> dict | None:
    """Look up a connector by name from a type dictionary.
    getConnectorFromType MUST return the matching connector dict.
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️getconnectorfromtype](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/GET-CONNECTOR-FROM-TYPE)
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
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️planetomatrixdict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/PLANE-TO-MATRIX-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️matrixtoplanedict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/MATRIX-TO-PLANE-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️quaternionfromunitvectorsdict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/QUATERNION-FROM-UNIT-VECTORS-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️quaternionfromaxisangledict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/QUATERNION-FROM-AXIS-ANGLE-DICT)
    """
    halfAngle = angle / 2
    s = numpy.sin(halfAngle)
    return numpy.array([axis[0] * s, axis[1] * s, axis[2] * s, numpy.cos(halfAngle)])

def quaternionToMatrixDict(q: numpy.ndarray) -> numpy.ndarray:
    """Convert a quaternion to a 3x3 rotation matrix.
    quaternionToMatrixDict MUST produce a valid 3x3 rotation matrix.
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️quaterniontomatrixdict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/QUATERNION-TO-MATRIX-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️makerotationaxisdict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/MAKE-ROTATION-AXIS-DICT)
    """
    return quaternionToMatrixDict(quaternionFromAxisAngleDict(axis, angle))

def makeTranslationDict(x: float, y: float, z: float) -> numpy.ndarray:
    """Create a 4x4 translation matrix from a displacement vector.
    makeTranslationDict MUST return a 4x4 translation matrix.
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️maketranslationdict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/MAKE-TRANSLATION-DICT)
    """
    m = numpy.eye(4)
    m[0, 3] = x
    m[1, 3] = y
    m[2, 3] = z
    return m

def applyMatrix4ToVec3Dict(m: numpy.ndarray, v: numpy.ndarray) -> numpy.ndarray:
    """Apply a 4x4 matrix to a 3D vector dictionary.
    applyMatrix4ToVec3Dict MUST apply the full affine transformation.
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️applymatrix4tovec3dict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/APPLY-MATRIX4TO-VEC3DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️computechildplanedict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/COMPUTE-CHILD-PLANE-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖flattendesign🛠️flattendesigndict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/FLATTEN-DESIGN/FLATTEN-DESIGN-DICT)
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

# [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️areattributesequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-ATTRIBUTES-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️arepropsequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-PROPS-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️areportsequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-PORTS-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️aremodelsequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-MODELS-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️aretypesequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-TYPES-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️arepiecesequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-PIECES-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️areconnectionsequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-CONNECTIONS-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️aredesignsequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-DESIGNS-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️areportsequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-PORTS-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️arequalitiesequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-QUALITIES-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️arefilesequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-FILES-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️arefoldersequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-FOLDERS-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️areauthorsequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-AUTHORS-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️areconceptsequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-CONCEPTS-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️aretagsequaldict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-TAGS-EQUAL-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️arekitsdictequal](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-KITS-DICT-EQUAL)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️getkitdiffdict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/GET-KIT-DIFF-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️applykitdiffdict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/APPLY-KIT-DIFF-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️inversekitdiffdict](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/INVERSE-KIT-DIFF-DICT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitdiffoperations🛠️arekitdiffsdictequal](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-DIFF-OPERATIONS/ARE-KIT-DIFFS-DICT-EQUAL)
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

# [👤semio📚py💻semiopy🔖domain🔖kitimport🔖export](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-IMPORT-EXPORT)
# Import and export utilities for kit serialization and deserialization.

class KitData:
    """Simple in-memory kit representation that supports attribute access.
    KitData MUST hold all kit entities in memory for import and export operations.
    [👤semio📚py💻semiopy🔖domain🔖kitimport🔖export🛠️kitdata](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-IMPORT-EXPORT/KIT-DATA)
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
    [👤semio📚py💻semiopy🔖domain🔖kitimport🔖export🛠️importkit](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-IMPORT-EXPORT/IMPORT-KIT)
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
    [👤semio📚py💻semiopy🔖domain🔖kitimport🔖export🛠️exportkit](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/KIT-IMPORT-EXPORT/EXPORT-KIT)
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

# [👤semio📚py💻semiopy🔖domain🔖spatialmath](semiorepo://section/SEMIO/PY/SEMIO.PY/DOMAIN/SPATIAL-MATH)
# Spatial math utilities for vector normalization and plane computation.

def normalizeVector(v: numpy.ndarray) -> numpy.ndarray:
    """Normalize a 3D vector to unit length.
    normalizeVector MUST return a unit-length vector or raise on zero length.
    [👤semio📚py💻semiopy🔖domain🔖spatialmath🛠️normalizevector](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/SPATIAL-MATH/NORMALIZE-VECTOR)
    """
    length = numpy.linalg.norm(v)
    if length < 1e-10:
        return v
    return v / length

def planeFromYAxis(yAxis: numpy.ndarray, phiDegrees: float = 0.0, origin: numpy.ndarray | None = None) -> Plane:
    """Construct a plane from an origin point and a Y-axis direction.
    planeFromYAxis MUST derive orthogonal x and z axes from the y axis.
    [👤semio📚py💻semiopy🔖domain🔖spatialmath🛠️planefromyaxis](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/SPATIAL-MATH/PLANE-FROM-Y-AXIS)
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
    [👤semio📚py💻semiopy🔖domain🔖spatialmath🛠️computechildplane](semiorepo://definition/SEMIO/PY/SEMIO.PY/DOMAIN/SPATIAL-MATH/COMPUTE-CHILD-PLANE)
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
