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

# endregion Imports

# region Type Hints
# [👤semio📚py💻semio🔖typehints](repo://p/u/semio/b/l/py/f/semio.py/s/Type%20Hints)
# Custom type hint aliases used throughout the module.

RecursiveAnyList = typing.Any | list["RecursiveAnyList"]
"""🔁 A recursive any list is either any or a list where the items are recursive any list."""

# endregion Type Hints

# region Constants
# [👤semio📚py💻semio🔖constants](repo://p/u/semio/b/l/py/f/semio.py/s/Constants)
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
# [👤semio📚py💻semio🔖utility](repo://p/u/semio/b/l/py/f/semio.py/s/Utility)
# General-purpose utility functions for encoding, formatting and transformation.


def encode(value: str) -> str:
    """ᗒ Encode a string to be url safe.
    encode MUST return a percent-encoded string safe for URL paths.
    [👤semio📚py💻semio🔖utility🛠️encode](repo://p/u/semio/b/l/py/f/semio.py/s/Utility/d/i/encode)
    """
    return urllib.parse.quote(value, safe="")


def decode(value: str) -> str:
    """ᗕ Decode a url safe string.
    decode MUST return the original string from a percent-encoded input.
    [👤semio📚py💻semio🔖utility🛠️decode](repo://p/u/semio/b/l/py/f/semio.py/s/Utility/d/i/decode)
    """
    return urllib.parse.unquote(value)


def encodeList(items: list[str]) -> str:
    """Encode a list of strings into a comma-separated URL-safe string.
    encodeList MUST encode each item and join them with commas.
    [👤semio📚py💻semio🔖utility🛠️encodelist](repo://p/u/semio/b/l/py/f/semio.py/s/Utility/d/i/encodeList)
    """
    return ",".join([encode(t) for t in items])


def decodeList(encodedList: str) -> list[str]:
    """Decode a comma-separated URL-safe string into a list of strings.
    decodeList MUST split by comma and decode each item.
    [👤semio📚py💻semio🔖utility🛠️decodelist](repo://p/u/semio/b/l/py/f/semio.py/s/Utility/d/i/decodeList)
    """
    return [decode(t) for t in encodedList.split(",")]


def encodeRecursiveAnyList(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Encode a `RecursiveAnyList` to a url encoded string.
    encodeRecursiveAnyList MUST recursively encode nested lists into a flat string.
    [👤semio📚py💻semio🔖utility🛠️encoderecursiveanylist](repo://p/u/semio/b/l/py/f/semio.py/s/Utility/d/i/encodeRecursiveAnyList)
    """
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return encode(",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList]))


def create_id(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Turn any into `encoded(str(any))` or a recursive list into a flat comma [,] separated encoded list.
    create_id MUST produce a deterministic identifier from any value or nested list.
    [👤semio📚py💻semio🔖utility🛠️createid](repo://p/u/semio/b/l/py/f/semio.py/s/Utility/d/i/create_id)
    """
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return ",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList])


def pretty(number: float) -> str:
    """🦋 Pretty print a floating point number.
    pretty MUST format the number with up to 5 significant digits.
    [👤semio📚py💻semio🔖utility🛠️pretty](repo://p/u/semio/b/l/py/f/semio.py/s/Utility/d/i/pretty)
    """
    if number == -0.0:
        number = 0.0
    return f"{number:.5f}".rstrip("0").rstrip(".")


def changeValues(c: dict | list, key: str, func: typing.Callable[[typing.Any], typing.Any]) -> None:
    """Recursively change values for a given key in nested dicts and lists.
    changeValues MUST apply the function to all occurrences of the key recursively.
    [👤semio📚py💻semio🔖utility🛠️changevalues](repo://p/u/semio/b/l/py/f/semio.py/s/Utility/d/i/changeValues)
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
    [👤semio📚py💻semio🔖utility🛠️changekeys](repo://p/u/semio/b/l/py/f/semio.py/s/Utility/d/i/changeKeys)
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
    [👤semio📚py💻semio🔖utility🛠️normalizeangle](repo://p/u/semio/b/l/py/f/semio.py/s/Utility/d/i/normalizeAngle)
    """
    return (angle % 360 + 360) % 360


# endregion Utility

# region Logging
# [👤semio📚py💻semio🔖logging](repo://p/u/semio/b/l/py/f/semio.py/s/Logging)
# Module-level logger configuration.

logger = loguru.logger

# endregion Logging

# region Exceptions
# [👤semio📚py💻semio🔖exceptions](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions)
# Custom exception hierarchy for server, client and specification errors.


class Error(Exception, abc.ABC):
    """❗ The base for all exceptions.
    Error MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️error](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/Error)
    """

    def __str__(self):
        return "❗ " + self.__class__.__name__


class ServerError(Error, abc.ABC):
    """🖥 The base for all server errors.
    ServerError MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️servererror](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/ServerError)
    """


class ClientError(Error, abc.ABC):
    """👩‍💼 The base for all client errors.
    ClientError MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️clienterror](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/ClientError)
    """


class CodeUnreachable(ServerError):
    """Exception for code paths that should never be reached.
    CodeUnreachable MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️codeunreachable](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/CodeUnreachable)
    """

    def __str__(self):
        return "🤷 This code should be unreachable."


class FeatureNotYetSupported(ServerError):
    """Exception for unimplemented features.
    FeatureNotYetSupported MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️featurenotyetsupported](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/FeatureNotYetSupported)
    """

    def __str__(self):
        return "🔜 This feature is not yet supported."


class RemoteKitsNotYetSupported(FeatureNotYetSupported):
    """Exception for unsupported remote kit access.
    RemoteKitsNotYetSupported MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️remotekitsnotyetsupported](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/RemoteKitsNotYetSupported)
    """

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return "🔜 Remote kits are not yet supported."


class AuthenticationError(ClientError):
    """🔐 Base error for authentication failures.
    AuthenticationError MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️authenticationerror](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/AuthenticationError)
    """

    def __str__(self):
        return "🔐 Authentication failed."


class InvalidAuthToken(AuthenticationError):
    """🔑 The auth token is invalid or expired.
    InvalidAuthToken MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️invalidauthtoken](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/InvalidAuthToken)
    """

    def __init__(self, serverUrl: str) -> None:
        self.serverUrl = serverUrl

    def __str__(self):
        return f"🔑 The auth token for server ({self.serverUrl}) is invalid or expired."


class AuthTokenNotFound(AuthenticationError):
    """🔑 No auth token found for the server.
    AuthTokenNotFound MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️authtokennotfound](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/AuthTokenNotFound)
    """

    def __init__(self, serverUrl: str) -> None:
        self.serverUrl = serverUrl

    def __str__(self):
        return f"🔑 No auth token found for server ({self.serverUrl}). Call login first."


class ServerUnreachable(ClientError):
    """🌐 The remote server is not reachable.
    ServerUnreachable MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️serverunreachable](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/ServerUnreachable)
    """

    def __init__(self, serverUrl: str) -> None:
        self.serverUrl = serverUrl

    def __str__(self):
        return f"🌐 The remote server ({self.serverUrl}) is not reachable."


class RemoteKitUriNotValid(ClientError):
    """🌐 The remote kit URI is not valid.
    RemoteKitUriNotValid MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️remotekiturinotvalid](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/RemoteKitUriNotValid)
    """

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🌐 The remote kit URI ({self.uri}) is not valid. Expected format: http(s)://server/api/kits/encodedKitUri"


class NotFound(ClientError, abc.ABC):
    """🔍 The base for not found errors.
    NotFound MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️notfound](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/NotFound)
    """


class SpecificationError(ClientError, abc.ABC):
    """📋 The base for all specification errors.
    SpecificationError MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️specificationerror](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/SpecificationError)
    """


class NoParentAssigned(SpecificationError, abc.ABC):
    """👪 The base for all no parent assigned errors.
    NoParentAssigned MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖exceptions🛠️noparentassigned](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/NoParentAssigned)
    """


class NoTypeOrDesignAssigned(NoParentAssigned):
    """No Type Or Design Assigned definition.
    NoTypeOrDesignAssigned MUST fulfill its documented contract.
    [👤semio📚py💻semio🔖exceptions🛠️notypeordesignassigned](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/NoTypeOrDesignAssigned)
    """

    def __str__(self):
        return "👪 The entity has no parent type or design assigned."


class NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned(NoParentAssigned):
    """No Model Or Port Or Type Or Piece Or Connection Or Design Or Kit Assigned definition.
    NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned MUST fulfill its documented contract.
    [👤semio📚py💻semio🔖exceptions🛠️nomodelorportortypeorpieceorconnectionordesignorkitassigned](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/NoModelOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned)
    """

    def __str__(self):
        return "👪 The entity has no parent model, connector, type, piece, connection, design, kit or folder assigned."


class AlreadyExists(SpecificationError, abc.ABC):
    """♊ The entity already exists in the store.
    AlreadyExists MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖exceptions🛠️alreadyexists](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/AlreadyExists)
    """


class Semio(pydantic.BaseModel):
    """ℹ Metadata about the database.
    Semio MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖exceptions🛠️semio](repo://p/u/semio/b/l/py/f/semio.py/s/Exceptions/d/i/Semio)
    """

    release: str = pydantic.Field(default=RELEASE)
    """🍾 The current release of semio."""
    engine: str = pydantic.Field(default=VERSION)
    """⚙️The version of the engine that created this database."""
    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)
    """⌚ The time when the database was created."""


# endregion Exceptions

# region Modeling
# [🔖semio/py/semio.py#Modeling](repo://section/semio/py/semio.py/MODELING)

# region Primitives
# [👤semio📚py💻semio🔖modeling](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling)
# Abstract base classes for models, fields, ids, inputs, outputs and entities.


class SModel(pydantic.BaseModel, abc.ABC):
    """⚪ The base for models.
    SModel MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️smodel](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/SModel)
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
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️field](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/Field)
    """


class RealField(Field, abc.ABC):
    """🧑 The base for a real field of a model. No lie.
    RealField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️realfield](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/RealField)
    """


class MaskedField(Field, abc.ABC):
    """🎭 The base for a mask of a field of a model. WYSIWYG but don't expect it to be there.
    MaskedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️maskedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/MaskedField)
    """


class Base(SModel, abc.ABC):
    """👥 The base for models.
    Base MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️base](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/Base)
    """


class Id(Base, abc.ABC):
    """🪪 The base for ids. All fields that identify the entity here.
    Id MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️id](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/Id)
    """


class Props(Base, abc.ABC):
    """🎫 The base for props. All fields except input-only, output-only or child entities.
    Props MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️props](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/Props)
    """


class Input(Base, abc.ABC):
    """↘ The base for inputs. All fields that are required to create the entity.
    Input MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️input](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/Input)
    """


class Context(Base, abc.ABC):
    """📑 The base for contexts. All fields that are required to understand the entity by an llm.
    Context MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️context](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/Context)
    """


class Output(Base, abc.ABC):
    """↗ The base for outputs. All fields that are returned when the entity is fetched.
    Output MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️output](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/Output)
    """


class Prediction(Base, abc.ABC):
    """🔮 The base for predictions. All fields that are required to predict the entity by a llm.
    Prediction MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️prediction](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/Prediction)
    """


class Entity(SModel, abc.ABC):
    """▢ The base for entities. All fields and behavior of the entity.
    Entity MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️entity](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/Entity)
    """

    PLURAL: typing.ClassVar[str]
    """🔢 The plural of the singular of the entity name."""

    def parent_entity(self) -> typing.Optional["Entity"]:
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
        parent = self.parent_entity()
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
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️table](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/Table)
    """


class TableEntity(Entity, Table, abc.ABC):
    """▢ The base for table entities.
    TableEntity MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖modeling🔖primitives🛠️tableentity](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Primitives/d/i/TableEntity)
    """

    """📛 The lowercase name of the table in the database."""


# endregion Primitives

# region Graphql
# [👤semio📚py💻semio🔖modeling🔖graphql](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Graphql)
# GraphQL node base classes for pydantic, sqlalchemy and relay integration.


class Node(graphene_pydantic.PydanticObjectType):
    """A base class for all nodes that are not a table in the database.
    Node MUST expose the model via Meta.
    [👤semio📚py💻semio🔖modeling🔖graphql🛠️node](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Graphql/d/i/Node)
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
    [👤semio📚py💻semio🔖modeling🔖graphql🛠️inputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Graphql/d/i/InputNode)
    """

    class Meta:
        abstract = True


class RelayNode(graphene.relay.Node):
    """Relay-compliant GraphQL node interface.
    RelayNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖modeling🔖graphql🛠️relaynode](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Graphql/d/i/RelayNode)
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


class TableNode(graphene_pydantic.PydanticObjectType):
    """A base class for all nodes that are a table in the database.
    It automatically excludes the fields that are defined in the table.
    Resolvers to all @properties are added.
    Child relationships are by default included.
    TableNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖modeling🔖graphql🛠️tablenode](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Graphql/d/i/TableNode)
    """

    class Meta:
        abstract = True

    @classmethod
    def __init_subclass_with_meta__(cls, model=None, **options):
        excludedFields = tuple(k for k, v in model.model_fields.items() if v.exclude or v.default_factory is not None)
        if "exclude_fields" in options:
            options["exclude_fields"] += excludedFields
        else:
            options["exclude_fields"] = excludedFields
        if "name" not in options:
            options["name"] = model.__name__

        super().__init_subclass_with_meta__(model=model, **options)


class TableEntityNode(TableNode):
    """A base class for all nodes that are a table in the database and are entities.
    It automatically complies to the Relay Node interface.
    TableEntityNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖modeling🔖graphql🛠️tableentitynode](repo://p/u/semio/b/l/py/f/semio.py/s/Modeling/s/Graphql/d/i/TableEntityNode)
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
# [🔖semio/py/semio.py#Domain](repo://section/semio/py/semio.py/DOMAIN)

# region Attribute
# [👤semio📚py💻semio🔖domain🔖attribute](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute)
# Attribute entity with key-value pairs and definitions.


class AttributeKeyField(RealField, abc.ABC):
    """Field mixin for the key of a attribute.
    AttributeKeyField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖attribute🛠️attributekeyfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute/d/i/AttributeKeyField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class AttributeValueField(RealField, abc.ABC):
    """Field mixin for the value of a attribute.
    AttributeValueField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖attribute🛠️attributevaluefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute/d/i/AttributeValueField)
    """

    value: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class AttributeDefinitionField(RealField, abc.ABC):
    """Field mixin for the definition of a attribute.
    AttributeDefinitionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖attribute🛠️attributedefinitionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute/d/i/AttributeDefinitionField)
    """

    definition: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class AttributeId(AttributeKeyField, Id):
    """Identity fields for uniquely identifying a attribute.
    AttributeId MUST contain all fields that uniquely identify a attribute.
    [👤semio📚py💻semio🔖domain🔖attribute🛠️attributeid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute/d/i/AttributeId)
    """

    pass


class AttributeProps(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Props):
    """Property fields for a attribute.
    AttributeProps MUST contain all non-relational property fields.
    [👤semio📚py💻semio🔖domain🔖attribute🛠️attributeprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute/d/i/AttributeProps)
    """

    pass


class AttributeInput(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Input):
    """Input fields for creating or updating a attribute.
    AttributeInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖attribute🛠️attributeinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute/d/i/AttributeInput)
    """

    pass


class AttributeContext(AttributeValueField, AttributeKeyField, Context):
    """Context fields for understanding a attribute by an LLM.
    AttributeContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖attribute🛠️attributecontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute/d/i/AttributeContext)
    """

    pass


class AttributeOutput(AttributeDefinitionField, AttributeValueField, AttributeKeyField, Output):
    """Output fields returned when fetching a attribute.
    AttributeOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖attribute🛠️attributeoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute/d/i/AttributeOutput)
    """

    pass


class Attribute(
    AttributeDefinitionField,
    AttributeValueField,
    AttributeKeyField,
    TableEntity,
):
    """Attribute entity storing a key-value pair with an optional definition.
    Attribute MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖attribute🛠️attribute](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute/d/i/Attribute)
    """

    PLURAL = "attributes"

    def parent_entity(
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
    [👤semio📚py💻semio🔖domain🔖attribute🛠️attributeinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Attribute/d/i/AttributeInputNode)
    """

    class Meta:
        model = AttributeInput


# endregion Attribute

# region Tag
# [👤semio📚py💻semio🔖domain🔖tag](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Tag)
# Tag entity for categorizing and labeling kit elements.


class TagGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a tag.
    TagGuidField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖tag🛠️tagguidfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Tag/d/i/TagGuidField)
    """

    guid: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class TagNameField(RealField, abc.ABC):
    """Field mixin for the name of a tag.
    TagNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖tag🛠️tagnamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Tag/d/i/TagNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class TagDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a tag.
    TagDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖tag🛠️tagdescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Tag/d/i/TagDescriptionField)
    """

    description: typing.Optional[str] = pydantic.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)


class TagIconField(RealField, abc.ABC):
    """Field mixin for the icon of a tag.
    TagIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖tag🛠️tagiconfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Tag/d/i/TagIconField)
    """

    icon: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class TagOrderField(RealField, abc.ABC):
    """Field mixin for the order of a tag.
    TagOrderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖tag🛠️tagorderfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Tag/d/i/TagOrderField)
    """

    order: int = pydantic.Field(default=0)


class TagId(TagGuidField, Id):
    """Identity fields for uniquely identifying a tag.
    TagId MUST contain all fields that uniquely identify a tag.
    [👤semio📚py💻semio🔖domain🔖tag🛠️tagid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Tag/d/i/TagId)
    """

    pass


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
    [👤semio📚py💻semio🔖domain🔖tag🛠️tag](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Tag/d/i/Tag)
    """


# endregion Tag

# region Concept
# [👤semio📚py💻semio🔖domain🔖concept](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Concept)
# Concept entity for semantic grouping of design elements.


class ConceptGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a concept.
    ConceptGuidField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖concept🛠️conceptguidfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Concept/d/i/ConceptGuidField)
    """

    guid: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class ConceptNameField(RealField, abc.ABC):
    """Field mixin for the name of a concept.
    ConceptNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖concept🛠️conceptnamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Concept/d/i/ConceptNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class ConceptDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a concept.
    ConceptDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖concept🛠️conceptdescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Concept/d/i/ConceptDescriptionField)
    """

    description: typing.Optional[str] = pydantic.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)


class ConceptIconField(RealField, abc.ABC):
    """Field mixin for the icon of a concept.
    ConceptIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖concept🛠️concepticonfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Concept/d/i/ConceptIconField)
    """

    icon: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class ConceptOrderField(RealField, abc.ABC):
    """Field mixin for the order of a concept.
    ConceptOrderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖concept🛠️conceptorderfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Concept/d/i/ConceptOrderField)
    """

    order: int = pydantic.Field(default=0)


class ConceptId(ConceptGuidField, Id):
    """Identity fields for uniquely identifying a concept.
    ConceptId MUST contain all fields that uniquely identify a concept.
    [👤semio📚py💻semio🔖domain🔖concept🛠️conceptid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Concept/d/i/ConceptId)
    """

    pass


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
    [👤semio📚py💻semio🔖domain🔖concept🛠️concept](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Concept/d/i/Concept)
    """


# endregion Concept

# region Coord
# [👤semio📚py💻semio🔖domain🔖coord](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Coord)
# Coordinate primitive for three-dimensional values.


class Coord(SModel):
    """Three-dimensional coordinate with x, y and z values.
    Coord MUST contain all coordinate or geometry fields.
    [👤semio📚py💻semio🔖domain🔖coord🛠️coord](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Coord/d/i/Coord)
    """

    u: float = pydantic.Field()
    v: float = pydantic.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.u)}, {pretty(self.v)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.u)}, {pretty(self.v)}]"


class CoordInput(Coord, Input):
    """Input fields for creating or updating a coord.
    CoordInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖coord🛠️coordinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Coord/d/i/CoordInput)
    """

    pass


class CoordContext(Coord, Context):
    """Context fields for understanding a coord by an LLM.
    CoordContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖coord🛠️coordcontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Coord/d/i/CoordContext)
    """

    pass


class CoordOutput(Coord, Output):
    """Output fields returned when fetching a coord.
    CoordOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖coord🛠️coordoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Coord/d/i/CoordOutput)
    """

    pass


class CoordPrediction(Coord, Prediction):
    """Prediction fields for LLM-based coord inference.
    CoordPrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semio🔖domain🔖coord🛠️coordprediction](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Coord/d/i/CoordPrediction)
    """

    pass


class CoordNode(Node):
    """GraphQL node exposing coord data.
    CoordNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖coord🛠️coordnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Coord/d/i/CoordNode)
    """

    class Meta:
        model = Coord


class CoordInputNode(InputNode):
    """GraphQL input node for coord mutations.
    CoordInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖coord🛠️coordinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Coord/d/i/CoordInputNode)
    """

    class Meta:
        model = CoordInput


# endregion Coord

# region Point
# [👤semio📚py💻semio🔖domain🔖point](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Point)
# Point primitive representing a position in 3D space.


class Point(SModel):
    """Point in 3D space with x, y and z coordinates.
    Point MUST contain all coordinate or geometry fields.
    [👤semio📚py💻semio🔖domain🔖point🛠️point](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Point/d/i/Point)
    """

    x: float = pydantic.Field()
    y: float = pydantic.Field()
    z: float = pydantic.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"


class PointInput(Point, Input):
    """Input fields for creating or updating a point.
    PointInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖point🛠️pointinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Point/d/i/PointInput)
    """

    pass


class PointContext(Point, Context):
    """Context fields for understanding a point by an LLM.
    PointContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖point🛠️pointcontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Point/d/i/PointContext)
    """

    pass


class PointOutput(Point, Output):
    """Output fields returned when fetching a point.
    PointOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖point🛠️pointoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Point/d/i/PointOutput)
    """

    pass


class PointPrediction(Point, Prediction):
    """Prediction fields for LLM-based point inference.
    PointPrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semio🔖domain🔖point🛠️pointprediction](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Point/d/i/PointPrediction)
    """

    pass


class PointNode(Node):
    """GraphQL node exposing point data.
    PointNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖point🛠️pointnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Point/d/i/PointNode)
    """

    class Meta:
        model = Point


class PointInputNode(InputNode):
    """GraphQL input node for point mutations.
    PointInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖point🛠️pointinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Point/d/i/PointInputNode)
    """

    class Meta:
        model = PointInput


# endregion Point

# region Vector
# [👤semio📚py💻semio🔖domain🔖vector](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Vector)
# Vector primitive representing a direction in 3D space.


class Vector(SModel):
    """Direction vector in 3D space with x, y and z components.
    Vector MUST contain all coordinate or geometry fields.
    [👤semio📚py💻semio🔖domain🔖vector🛠️vector](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Vector/d/i/Vector)
    """

    x: float = pydantic.Field()
    y: float = pydantic.Field()
    z: float = pydantic.Field()

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"


class VectorInput(Vector, Input):
    """Input fields for creating or updating a vector.
    VectorInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖vector🛠️vectorinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Vector/d/i/VectorInput)
    """

    pass


class VectorContext(Vector, Context):
    """Context fields for understanding a vector by an LLM.
    VectorContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖vector🛠️vectorcontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Vector/d/i/VectorContext)
    """

    pass


class VectorOutput(Vector, Output):
    """Output fields returned when fetching a vector.
    VectorOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖vector🛠️vectoroutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Vector/d/i/VectorOutput)
    """

    pass


class VectorPrediction(Vector, Prediction):
    """Prediction fields for LLM-based vector inference.
    VectorPrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semio🔖domain🔖vector🛠️vectorprediction](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Vector/d/i/VectorPrediction)
    """

    pass


class VectorNode(Node):
    """GraphQL node exposing vector data.
    VectorNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖vector🛠️vectornode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Vector/d/i/VectorNode)
    """

    class Meta:
        model = Vector


class VectorInputNode(InputNode):
    """GraphQL input node for vector mutations.
    VectorInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖vector🛠️vectorinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Vector/d/i/VectorInputNode)
    """

    class Meta:
        model = VectorInput


# endregion Vector

# region Plane
# [👤semio📚py💻semio🔖domain🔖plane](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Plane)
# Plane primitive representing an oriented coordinate frame in 3D space.


class PlaneOriginField(MaskedField, abc.ABC):
    """Field mixin for the origin of a plane.
    PlaneOriginField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖plane🛠️planeoriginfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Plane/d/i/PlaneOriginField)
    """

    origin: Point = pydantic.Field()


class PlaneXAxisField(MaskedField, abc.ABC):
    """Field mixin for the x axis of a plane.
    PlaneXAxisField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖plane🛠️planexaxisfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Plane/d/i/PlaneXAxisField)
    """

    xAxis: Vector = pydantic.Field()


class PlaneYAxisField(MaskedField, abc.ABC):
    """Field mixin for the y axis of a plane.
    PlaneYAxisField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖plane🛠️planeyaxisfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Plane/d/i/PlaneYAxisField)
    """

    yAxis: Vector = pydantic.Field()


class PlaneInput(Input):
    """Input fields for creating or updating a plane.
    PlaneInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖plane🛠️planeinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Plane/d/i/PlaneInput)
    """

    origin: PointInput = pydantic.Field()
    xAxis: VectorInput = pydantic.Field()
    yAxis: VectorInput = pydantic.Field()


class PlaneContext(Context):
    """Context fields for understanding a plane by an LLM.
    PlaneContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖plane🛠️planecontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Plane/d/i/PlaneContext)
    """

    origin: PointContext = pydantic.Field()
    xAxis: VectorContext = pydantic.Field()
    yAxis: VectorContext = pydantic.Field()


class PlaneOutput(PlaneYAxisField, PlaneXAxisField, PlaneOriginField, Output):
    """Output fields returned when fetching a plane.
    PlaneOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖plane🛠️planeoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Plane/d/i/PlaneOutput)
    """

    pass


class Plane(Table):
    """Oriented coordinate frame in 3D space with origin and axes.
    Plane MUST contain all coordinate or geometry fields.
    [👤semio📚py💻semio🔖domain🔖plane🛠️plane](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Plane/d/i/Plane)
    """

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
    [👤semio📚py💻semio🔖domain🔖plane🛠️planeinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Plane/d/i/PlaneInputNode)
    """

    class Meta:
        model = PlaneInput


# endregion Plane

# region Location
# [👤semio📚py💻semio🔖domain🔖location](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location)
# Location entity for geographic coordinates with longitude, latitude and altitude.


class LocationGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a location.
    LocationGuidField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationguidfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationGuidField)
    """

    guid: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class LocationLongitudeField(RealField, abc.ABC):
    """Field mixin for the longitude of a location.
    LocationLongitudeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationlongitudefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationLongitudeField)
    """

    longitude: float = pydantic.Field()


class LocationLatitudeField(RealField, abc.ABC):
    """Field mixin for the latitude of a location.
    LocationLatitudeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationlatitudefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationLatitudeField)
    """

    latitude: float = pydantic.Field()


class LocationAltitudeField(RealField, abc.ABC):
    """Field mixin for the altitude of a location.
    LocationAltitudeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationaltitudefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationAltitudeField)
    """

    altitude: typing.Optional[float] = pydantic.Field(default=None)


class LocationId(LocationGuidField, Id):
    """Identity fields for uniquely identifying a location.
    LocationId MUST contain all fields that uniquely identify a location.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationId)
    """

    pass


class Location(
    LocationAltitudeField,
    LocationLatitudeField,
    LocationLongitudeField,
    LocationGuidField,
    TableEntity,
):
    """Geographic location with longitude, latitude and altitude.
    Location MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖location🛠️location](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/Location)
    """

    PLURAL = "locations"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


class LocationInput(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Input):
    """Input fields for creating or updating a location.
    LocationInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationInput)
    """

    pass


class LocationOutput(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Output):
    """Output fields returned when fetching a location.
    LocationOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationOutput)
    """

    pass


class LocationContext(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Context):
    """Context fields for understanding a location by an LLM.
    LocationContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationcontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationContext)
    """

    pass


class LocationPrediction(LocationAltitudeField, LocationLatitudeField, LocationLongitudeField, Prediction):
    """Prediction fields for LLM-based location inference.
    LocationPrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationprediction](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationPrediction)
    """

    pass


class LocationNode(Node):
    """GraphQL node exposing location data.
    LocationNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationNode)
    """

    class Meta:
        model = LocationOutput


class LocationInputNode(InputNode):
    """GraphQL input node for location mutations.
    LocationInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖location🛠️locationinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Location/d/i/LocationInputNode)
    """

    class Meta:
        model = LocationInput


# endregion Location

# region Author
# [👤semio📚py💻semio🔖domain🔖author](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author)
# Author entity for tracking contributor identity and rank.


class AuthorNameField(RealField, abc.ABC):
    """Field mixin for the name of a author.
    AuthorNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖author🛠️authornamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author/d/i/AuthorNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class AuthorEmailField(RealField, abc.ABC):
    """Field mixin for the email of a author.
    AuthorEmailField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖author🛠️authoremailfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author/d/i/AuthorEmailField)
    """

    email: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class AuthorRankField(RealField, abc.ABC):
    """Field mixin for the rank of a author.
    AuthorRankField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖author🛠️authorrankfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author/d/i/AuthorRankField)
    """

    rank: int = pydantic.Field(default=0)


class AuthorId(AuthorEmailField, Id):
    """Identity fields for uniquely identifying a author.
    AuthorId MUST contain all fields that uniquely identify a author.
    [👤semio📚py💻semio🔖domain🔖author🛠️authorid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author/d/i/AuthorId)
    """

    pass


class AuthorProps(AuthorEmailField, AuthorNameField, Props):
    """Property fields for a author.
    AuthorProps MUST contain all non-relational property fields.
    [👤semio📚py💻semio🔖domain🔖author🛠️authorprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author/d/i/AuthorProps)
    """

    pass


class AuthorInput(AuthorEmailField, AuthorNameField, Input):
    """Input fields for creating or updating a author.
    AuthorInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖author🛠️authorinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author/d/i/AuthorInput)
    """

    pass


class AuthorOutput(AuthorEmailField, AuthorNameField, Output):
    """Output fields returned when fetching a author.
    AuthorOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖author🛠️authoroutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author/d/i/AuthorOutput)
    """

    pass


class Author(
    AuthorRankField,
    AuthorEmailField,
    AuthorNameField,
    TableEntity,
):
    """Author entity with name, email and contribution rank.
    Author MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖author🛠️author](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author/d/i/Author)
    """

    PLURAL = "authors"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def parent_entity(self) -> "Kit":
        if self.kit is not None:
            return self.kit
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.email


class AuthorInputNode(InputNode):
    """GraphQL input node for author mutations.
    AuthorInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖author🛠️authorinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Author/d/i/AuthorInputNode)
    """

    class Meta:
        model = AuthorInput


# endregion Author

# region ArtifactAuthor
# [👤semio📚py💻semio🔖domain🔖artifactauthor](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/ArtifactAuthor)
# Artifact-author association entity linking artifacts to authors by email.


class ArtifactAuthorEmailField(RealField, abc.ABC):
    """Field mixin for the email of a artifact author.
    ArtifactAuthorEmailField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖artifactauthor🛠️artifactauthoremailfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/ArtifactAuthor/d/i/ArtifactAuthorEmailField)
    """

    author_email: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class ArtifactAuthor(ArtifactAuthorEmailField, TableEntity):
    """Association entity linking an artifact to an author by email.
    ArtifactAuthor MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖artifactauthor🛠️artifactauthor](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/ArtifactAuthor/d/i/ArtifactAuthor)
    """

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


# endregion ArtifactAuthor

# region File
# [👤semio📚py💻semio🔖domain🔖file](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File)
# File entity for managing binary assets with metadata and hashing.


class FileGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a file.
    FileGuidField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️fileguidfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileGuidField)
    """

    guid: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class FileNameField(RealField, abc.ABC):
    """Field mixin for the name of a file.
    FileNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️filenamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class FileRemoteField(RealField, abc.ABC):
    """Field mixin for the remote of a file.
    FileRemoteField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️fileremotefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileRemoteField)
    """

    remote: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class FileFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a file.
    FileFolderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️filefolderfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileFolderField)
    """

    folder: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class FileSizeField(RealField, abc.ABC):
    """Field mixin for the size of a file.
    FileSizeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️filesizefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileSizeField)
    """

    size: typing.Optional[int] = pydantic.Field(default=None)


class FileHashField(RealField, abc.ABC):
    """Field mixin for the hash of a file.
    FileHashField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️filehashfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileHashField)
    """

    hash: typing.Optional[str] = pydantic.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class FileBlobField(RealField, abc.ABC):
    """Field mixin for the blob of a file.
    FileBlobField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️fileblobfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileBlobField)
    """

    blob: typing.Optional[str] = pydantic.Field(default=None)


class FileCreatedAtField(RealField, abc.ABC):
    """Field mixin for the created at of a file.
    FileCreatedAtField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️filecreatedatfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileCreatedAtField)
    """

    createdAt: datetime.datetime = pydantic.Field()


class FileCreatedByField(RealField, abc.ABC):
    """Field mixin for the created by of a file.
    FileCreatedByField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️filecreatedbyfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileCreatedByField)
    """

    createdBy: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FileUpdatedAtField(RealField, abc.ABC):
    """Field mixin for the updated at of a file.
    FileUpdatedAtField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️fileupdatedatfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileUpdatedAtField)
    """

    updatedAt: datetime.datetime = pydantic.Field()


class FileUpdatedByField(RealField, abc.ABC):
    """Field mixin for the updated by of a file.
    FileUpdatedByField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖file🛠️fileupdatedbyfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileUpdatedByField)
    """

    updatedBy: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FileId(FileGuidField, Id):
    """Identity fields for uniquely identifying a file.
    FileId MUST contain all fields that uniquely identify a file.
    [👤semio📚py💻semio🔖domain🔖file🛠️fileid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileId)
    """

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
    FileGuidField,
    Props,
):
    """Property fields for a file.
    FileProps MUST contain all non-relational property fields.
    [👤semio📚py💻semio🔖domain🔖file🛠️fileprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileProps)
    """

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
    FileGuidField,
    Input,
):
    """Input fields for creating or updating a file.
    FileInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖file🛠️fileinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileInput)
    """

    pass


class FileContext(FileNameField, FileGuidField, Context):
    """Context fields for understanding a file by an LLM.
    FileContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖file🛠️filecontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileContext)
    """

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
    FileGuidField,
    Output,
):
    """Output fields returned when fetching a file.
    FileOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖file🛠️fileoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileOutput)
    """

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
    FileGuidField,
    TableEntity,
):
    """File entity for binary assets with metadata, hashing and timestamps.
    File MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖file🛠️file](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/File)
    """

    PLURAL = "files"

    def parent_entity(self) -> "Kit":
        if self.kit is not None:
            return self.kit
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.guid


class FileInputNode(InputNode):
    """GraphQL input node for file mutations.
    FileInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖file🛠️fileinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/File/d/i/FileInputNode)
    """

    class Meta:
        model = FileInput


# endregion File

# region Folder
# [👤semio📚py💻semio🔖domain🔖folder](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder)
# Folder entity for hierarchical organization of kit content.


class FolderGuidField(RealField, abc.ABC):
    """Field mixin for the guid of a folder.
    FolderGuidField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖folder🛠️folderguidfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderGuidField)
    """

    guid: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class FolderNameField(RealField, abc.ABC):
    """Field mixin for the name of a folder.
    FolderNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖folder🛠️foldernamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class FolderParentField(RealField, abc.ABC):
    """Field mixin for the parent of a folder.
    FolderParentField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖folder🛠️folderparentfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderParentField)
    """

    parent: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FolderDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a folder.
    FolderDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖folder🛠️folderdescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class FolderCreatedAtField(RealField, abc.ABC):
    """Field mixin for the created at of a folder.
    FolderCreatedAtField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖folder🛠️foldercreatedatfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderCreatedAtField)
    """

    createdAt: datetime.datetime = pydantic.Field()


class FolderCreatedByField(RealField, abc.ABC):
    """Field mixin for the created by of a folder.
    FolderCreatedByField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖folder🛠️foldercreatedbyfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderCreatedByField)
    """

    createdBy: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FolderUpdatedAtField(RealField, abc.ABC):
    """Field mixin for the updated at of a folder.
    FolderUpdatedAtField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖folder🛠️folderupdatedatfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderUpdatedAtField)
    """

    updatedAt: datetime.datetime = pydantic.Field()


class FolderUpdatedByField(RealField, abc.ABC):
    """Field mixin for the updated by of a folder.
    FolderUpdatedByField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖folder🛠️folderupdatedbyfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderUpdatedByField)
    """

    updatedBy: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class FolderId(FolderGuidField, Id):
    """Identity fields for uniquely identifying a folder.
    FolderId MUST contain all fields that uniquely identify a folder.
    [👤semio📚py💻semio🔖domain🔖folder🛠️folderid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderId)
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
    [👤semio📚py💻semio🔖domain🔖folder🛠️folderprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderProps)
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
    [👤semio📚py💻semio🔖domain🔖folder🛠️folderinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderInput)
    """

    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class FolderContext(FolderNameField, FolderGuidField, Context):
    """Context fields for understanding a folder by an LLM.
    FolderContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖folder🛠️foldercontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderContext)
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
    [👤semio📚py💻semio🔖domain🔖folder🛠️folderoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderOutput)
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
    [👤semio📚py💻semio🔖domain🔖folder🛠️folder](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/Folder)
    """

    PLURAL = "folders"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def parent_entity(self) -> "Kit":
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
    [👤semio📚py💻semio🔖domain🔖folder🛠️folderinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Folder/d/i/FolderInputNode)
    """

    class Meta:
        model = FolderInput


# endregion Folder

# region Benchmark
# [👤semio📚py💻semio🔖domain🔖benchmark](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark)
# Benchmark entity for defining performance metrics with min-max bounds.


class BenchmarkNameField(RealField, abc.ABC):
    """Field mixin for the name of a benchmark.
    BenchmarkNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmarknamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/BenchmarkNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class BenchmarkIconField(RealField, abc.ABC):
    """Field mixin for the icon of a benchmark.
    BenchmarkIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmarkiconfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/BenchmarkIconField)
    """

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class BenchmarkMinField(RealField, abc.ABC):
    """Field mixin for the min of a benchmark.
    BenchmarkMinField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmarkminfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/BenchmarkMinField)
    """

    min: typing.Optional[float] = pydantic.Field(default=None)


class BenchmarkMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a benchmark.
    BenchmarkMinExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmarkminexcludedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/BenchmarkMinExcludedField)
    """

    min_excluded: bool = pydantic.Field(default=False)


class BenchmarkMaxField(RealField, abc.ABC):
    """Field mixin for the max of a benchmark.
    BenchmarkMaxField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmarkmaxfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/BenchmarkMaxField)
    """

    max: typing.Optional[float] = pydantic.Field(default=None)


class BenchmarkMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a benchmark.
    BenchmarkMaxExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmarkmaxexcludedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/BenchmarkMaxExcludedField)
    """

    max_excluded: bool = pydantic.Field(default=False)


class BenchmarkId(BenchmarkNameField, Id):
    """Identity fields for uniquely identifying a benchmark.
    BenchmarkId MUST contain all fields that uniquely identify a benchmark.
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmarkid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/BenchmarkId)
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
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmarkprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/BenchmarkProps)
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
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmarkinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/BenchmarkInput)
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
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmarkoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/BenchmarkOutput)
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
):
    """Benchmark entity for performance metrics with min-max bounds.
    Benchmark MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖benchmark🛠️benchmark](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Benchmark/d/i/Benchmark)
    """

    PLURAL = "benchmarks"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# endregion Benchmark

# region Quality
# [👤semio📚py💻semio🔖domain🔖quality](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality)
# Quality entity for defining measurable properties with units and constraints.


class QualityKeyField(RealField, abc.ABC):
    """Field mixin for the key of a quality.
    QualityKeyField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualitykeyfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityKeyField)
    """

    key: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class QualityNameField(RealField, abc.ABC):
    """Field mixin for the name of a quality.
    QualityNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualitynamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class QualityDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a quality.
    QualityDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualitydescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class QualityUriField(RealField, abc.ABC):
    """Field mixin for the uri of a quality.
    QualityUriField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityurifield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityUriField)
    """

    uri: str = pydantic.Field(default="", max_length=URI_LENGTH_LIMIT)


class QualityScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a quality.
    QualityScalableField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityscalablefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityScalableField)
    """

    scalable: bool = pydantic.Field(default=False)


class QualityKindField(RealField, abc.ABC):
    """Field mixin for the kind of a quality.
    QualityKindField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualitykindfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityKindField)
    """

    kind: int = pydantic.Field(default=0)


class QualitySiField(RealField, abc.ABC):
    """Field mixin for the si of a quality.
    QualitySiField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualitysifield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualitySiField)
    """

    si: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class QualityImperialField(RealField, abc.ABC):
    """Field mixin for the imperial of a quality.
    QualityImperialField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityimperialfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityImperialField)
    """

    imperial: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class QualityMinField(RealField, abc.ABC):
    """Field mixin for the min of a quality.
    QualityMinField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityminfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityMinField)
    """

    min: typing.Optional[float] = pydantic.Field(default=None)


class QualityMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a quality.
    QualityMinExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityminexcludedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityMinExcludedField)
    """

    min_excluded: bool = pydantic.Field(default=True)


class QualityMaxField(RealField, abc.ABC):
    """Field mixin for the max of a quality.
    QualityMaxField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualitymaxfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityMaxField)
    """

    max: typing.Optional[float] = pydantic.Field(default=None)


class QualityMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a quality.
    QualityMaxExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualitymaxexcludedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityMaxExcludedField)
    """

    max_excluded: bool = pydantic.Field(default=True)


class QualityDefaultField(RealField, abc.ABC):
    """Field mixin for the default of a quality.
    QualityDefaultField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualitydefaultfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityDefaultField)
    """

    default: typing.Optional[float] = pydantic.Field(default=None)


class QualityFormulaField(RealField, abc.ABC):
    """Field mixin for the formula of a quality.
    QualityFormulaField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityformulafield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityFormulaField)
    """

    formula: str = pydantic.Field(default="", max_length=EXPRESSION_LENGTH_LIMIT)


class QualityFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a quality.
    QualityFolderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityfolderfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityFolderField)
    """

    folder: typing.Optional[str] = pydantic.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class QualityIconField(RealField, abc.ABC):
    """Field mixin for the icon of a quality.
    QualityIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityiconfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityIconField)
    """

    icon: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class QualityImageField(RealField, abc.ABC):
    """Field mixin for the image of a quality.
    QualityImageField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityimagefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityImageField)
    """

    image: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class QualityUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a quality.
    QualityUnitField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityunitfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityUnitField)
    """

    unit: typing.Optional[str] = pydantic.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class QualityCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a quality.
    QualityCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualitycreatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityCreatedField)
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class QualityUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a quality.
    QualityUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityupdatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityUpdatedField)
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class QualityId(QualityKeyField, Id):
    """Identity fields for uniquely identifying a quality.
    QualityId MUST contain all fields that uniquely identify a quality.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityId)
    """

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
    """Property fields for a quality.
    QualityProps MUST contain all non-relational property fields.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityProps)
    """

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
    """Input fields for creating or updating a quality.
    QualityInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityInput)
    """

    pass


class QualityContext(QualityDescriptionField, QualityNameField, QualityKeyField, Context):
    """Context fields for understanding a quality by an LLM.
    QualityContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualitycontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityContext)
    """

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
    """Output fields returned when fetching a quality.
    QualityOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖quality🛠️qualityoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/QualityOutput)
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
    [👤semio📚py💻semio🔖domain🔖quality🛠️quality](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Quality/d/i/Quality)
    """

    PLURAL = "qualities"

    benchmarks: list["Benchmark"] = pydantic.Field(default_factory=list)
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# endregion Quality

# region Prop
# [👤semio📚py💻semio🔖domain🔖prop](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop)
# Prop entity for key-value property pairs with units.


class PropKeyField(RealField, abc.ABC):
    """Field mixin for the key of a prop.
    PropKeyField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖prop🛠️propkeyfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/PropKeyField)
    """

    key: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class PropValueField(RealField, abc.ABC):
    """Field mixin for the value of a prop.
    PropValueField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖prop🛠️propvaluefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/PropValueField)
    """

    value: str = pydantic.Field(max_length=VALUE_LENGTH_LIMIT)


class PropUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a prop.
    PropUnitField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖prop🛠️propunitfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/PropUnitField)
    """

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class PropCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a prop.
    PropCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖prop🛠️propcreatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/PropCreatedField)
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class PropUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a prop.
    PropUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖prop🛠️propupdatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/PropUpdatedField)
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class PropId(PropKeyField, Id):
    """Identity fields for uniquely identifying a prop.
    PropId MUST contain all fields that uniquely identify a prop.
    [👤semio📚py💻semio🔖domain🔖prop🛠️propid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/PropId)
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
    [👤semio📚py💻semio🔖domain🔖prop🛠️propprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/PropProps)
    """

    pass


class PropInput(PropUnitField, PropValueField, PropKeyField, Input):
    """Input fields for creating or updating a prop.
    PropInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖prop🛠️propinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/PropInput)
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
    [👤semio📚py💻semio🔖domain🔖prop🛠️propoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/PropOutput)
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
    [👤semio📚py💻semio🔖domain🔖prop🛠️prop](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/Prop)
    """

    PLURAL = "props"

    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    def parent_entity(self) -> typing.Union["Connector", "Type", "Design"]:
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
    [👤semio📚py💻semio🔖domain🔖prop🛠️propinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Prop/d/i/PropInputNode)
    """

    class Meta:
        model = PropInput


# endregion Prop

# region Model
# [👤semio📚py💻semio🔖domain🔖model](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model)
# Model entity for 3D geometry representations linked to files.


class ModelNameField(RealField, abc.ABC):
    """Field mixin for the name of a model.
    ModelNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖model🛠️modelnamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelNameField)
    """

    name: typing.Optional[str] = pydantic.Field(default=None, max_length=NAME_LENGTH_LIMIT)


class ModelUrlField(RealField, abc.ABC):
    """Field mixin for the url of a model.
    ModelUrlField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖model🛠️modelurlfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelUrlField)
    """

    url: str = pydantic.Field(max_length=URL_LENGTH_LIMIT)


class ModelFileField(RealField, abc.ABC):
    """Field mixin for the file of a model.
    ModelFileField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖model🛠️modelfilefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelFileField)
    """

    file: str = pydantic.Field(max_length=ID_LENGTH_LIMIT)


class ModelDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a model.
    ModelDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖model🛠️modeldescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ModelTagsField(MaskedField, abc.ABC):
    """Field mixin for the tags of a model.
    ModelTagsField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖model🛠️modeltagsfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelTagsField)
    """

    tags: list[str] = pydantic.Field(default_factory=list)


class ModelId(ModelTagsField, Id):
    """Identity fields for uniquely identifying a model.
    ModelId MUST contain all fields that uniquely identify a model.
    [👤semio📚py💻semio🔖domain🔖model🛠️modelid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelId)
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
    [👤semio📚py💻semio🔖domain🔖model🛠️modelprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelProps)
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
    [👤semio📚py💻semio🔖domain🔖model🛠️modelinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelInput)
    """

    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class ModelContext(ModelTagsField, ModelDescriptionField, ModelNameField, Context):
    """Context fields for understanding a model by an LLM.
    ModelContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖model🛠️modelcontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelContext)
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
    [👤semio📚py💻semio🔖domain🔖model🛠️modeloutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelOutput)
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
    [👤semio📚py💻semio🔖domain🔖model🛠️model](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/Model)
    """

    PLURAL = "models"
    tags_: list[Tag] = pydantic.Field(default_factory=list)
    attributes: list[Attribute] = pydantic.Field(default_factory=list)

    @property
    def tags(self: "Model") -> list[str]:
        return [tag.name for tag in sorted(self.tags_, key=lambda x: x.order)]

    @tags.setter
    def tags(self: "Model", tags: list[str]):
        self.tags_ = [Tag(name=tag, order=i) for i, tag in enumerate(tags)]

    def parent_entity(self: "Model") -> "Type":
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
        except KeyError, AttributeError, Exception:
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
    [👤semio📚py💻semio🔖domain🔖model🛠️nomodelassigned](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/NoModelAssigned)
    """

    def __str__(self):
        return " The entity has no parent model assigned."


class ModelInputNode(InputNode):
    """GraphQL input node for model mutations.
    ModelInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖model🛠️modelinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Model/d/i/ModelInputNode)
    """

    class Meta:
        model = ModelInput


# endregion Model

# region Port
# [👤semio📚py💻semio🔖domain🔖port](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port)
# Port entity for defining connection interfaces on types.


class PortNameField(RealField, abc.ABC):
    """Field mixin for the name of a port.
    PortNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖port🛠️portnamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port/d/i/PortNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class PortDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a port.
    PortDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖port🛠️portdescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port/d/i/PortDescriptionField)
    """

    description: typing.Optional[str] = pydantic.Field(default=None, max_length=DESCRIPTION_LENGTH_LIMIT)


class PortIconField(RealField, abc.ABC):
    """Field mixin for the icon of a port.
    PortIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖port🛠️porticonfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port/d/i/PortIconField)
    """

    icon: typing.Optional[str] = pydantic.Field(default=None, max_length=URL_LENGTH_LIMIT)


class PortCompatiblePortsField(MaskedField, abc.ABC):
    """Field mixin for the compatible ports of a port.
    PortCompatiblePortsField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖port🛠️portcompatibleportsfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port/d/i/PortCompatiblePortsField)
    """

    compatiblePorts: list[str] = pydantic.Field(default_factory=list)


class PortId(PortNameField, Id):
    """Identity fields for uniquely identifying a port.
    PortId MUST contain all fields that uniquely identify a port.
    [👤semio📚py💻semio🔖domain🔖port🛠️portid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port/d/i/PortId)
    """

    pass


class PortProps(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Props):
    """Property fields for a port.
    PortProps MUST contain all non-relational property fields.
    [👤semio📚py💻semio🔖domain🔖port🛠️portprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port/d/i/PortProps)
    """

    pass


class PortInput(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Input):
    """Input fields for creating or updating a port.
    PortInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖port🛠️portinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port/d/i/PortInput)
    """

    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class PortOutput(PortCompatiblePortsField, PortIconField, PortDescriptionField, PortNameField, Output):
    """Output fields returned when fetching a port.
    PortOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖port🛠️portoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port/d/i/PortOutput)
    """

    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Port(PortIconField, PortDescriptionField, PortNameField, TableEntity):
    """Port entity defining a named connection interface on a type.
    Port MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖port🛠️port](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port/d/i/Port)
    """

    PLURAL = "ports"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# TODO: Fix PortNode - was incorrectly changed to TableEntityNode in latest commit


class PortInputNode(InputNode):
    """GraphQL input node for port mutations.
    PortInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖port🛠️portinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Port/d/i/PortInputNode)
    """

    class Meta:
        model = PortInput


# endregion Port

# region Connector
# [🔖semio/py/semio.py#Connector](repo://section/semio/py/semio.py/CONNECTOR)

# region CompatiblePort
# [👤semio📚py💻semio🔖domain🔖connector🔖compatibleport](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/s/CompatiblePort)
# Compatible port entity for specifying allowed port pairings on connectors.


class CompatiblePortNameField(RealField, abc.ABC):
    """Field mixin for the name of a compatible port.
    CompatiblePortNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connector🔖compatibleport🛠️compatibleportnamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/s/CompatiblePort/d/i/CompatiblePortNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class CompatiblePortOrderField(RealField, abc.ABC):
    """Field mixin for the order of a compatible port.
    CompatiblePortOrderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connector🔖compatibleport🛠️compatibleportorderfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/s/CompatiblePort/d/i/CompatiblePortOrderField)
    """

    order: int = pydantic.Field()


class CompatiblePort(CompatiblePortOrderField, CompatiblePortNameField, Table):
    """Compatible port entity specifying an allowed port pairing.
    CompatiblePort MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖connector🔖compatibleport🛠️compatibleport](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/s/CompatiblePort/d/i/CompatiblePort)
    """


# endregion CompatiblePort


class ConnectorIdField(MaskedField, abc.ABC):
    """Field mixin for the id of a connector.
    ConnectorIdField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectoridfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorIdField)
    """

    id_: str = pydantic.Field(default="", max_length=ID_LENGTH_LIMIT)


class ConnectorDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a connector.
    ConnectorDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectordescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ConnectorMandatoryField(RealField, abc.ABC):
    """Field mixin for the mandatory of a connector.
    ConnectorMandatoryField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectormandatoryfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorMandatoryField)
    """

    is_mandatory: bool = pydantic.Field(default=False)


class ConnectorPortField(RealField, abc.ABC):
    """Field mixin for the port of a connector.
    ConnectorPortField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectorportfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorPortField)
    """

    port: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class ConnectorCompatiblePortsField(MaskedField, abc.ABC):
    """Field mixin for the compatible ports of a connector.
    ConnectorCompatiblePortsField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectorcompatibleportsfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorCompatiblePortsField)
    """

    compatiblePorts: list[str] = pydantic.Field(default_factory=list)


class ConnectorPointField(MaskedField, abc.ABC):
    """Field mixin for the point of a connector.
    ConnectorPointField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectorpointfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorPointField)
    """

    point: Point = pydantic.Field()


class ConnectorDirectionField(MaskedField, abc.ABC):
    """Field mixin for the direction of a connector.
    ConnectorDirectionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectordirectionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorDirectionField)
    """

    direction: Vector = pydantic.Field()


class ConnectorTField(RealField, abc.ABC):
    """Field mixin for the t of a connector.
    ConnectorTField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectortfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorTField)
    """

    t: float = pydantic.Field(default=0.0)


class ConnectorId(ConnectorIdField, Id):
    """Identity fields for uniquely identifying a connector.
    ConnectorId MUST contain all fields that uniquely identify a connector.
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectorid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorId)
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
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectorprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorProps)
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
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectorinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorInput)
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
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectorcontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorContext)
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
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectoroutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorOutput)
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
    [👤semio📚py💻semio🔖domain🔖connector🛠️connector](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/Connector)
    """

    PLURAL = "connectors"

    compatiblePorts_: list[CompatiblePort] = pydantic.Field(default_factory=list)
    attributes: list["Attribute"] = pydantic.Field(default_factory=list)
    props: list["Prop"] = pydantic.Field(default_factory=list)
    connecteds: list["Connection"] = pydantic.Field(default_factory=list)
    connectings: list["Connection"] = pydantic.Field(default_factory=list)

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

    def parent_entity(self) -> "Type":
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
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectornotfound](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorNotFound)
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
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectorinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorInputNode)
    """

    class Meta:
        model = ConnectorInput


class ConnectorIdInputNode(InputNode):
    """GraphQL input node for connector id mutations.
    ConnectorIdInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖connector🛠️connectoridinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connector/d/i/ConnectorIdInputNode)
    """

    class Meta:
        model = ConnectorId


# endregion Connector

# region Type
# [👤semio📚py💻semio🔖domain🔖type](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type)
# Type entity for defining reusable parametric building blocks.


class TypeNameField(RealField, abc.ABC):
    """Field mixin for the name of a type.
    TypeNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typenamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class TypeDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a type.
    TypeDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typedescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class TypeIconField(RealField, abc.ABC):
    """Field mixin for the icon of a type.
    TypeIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typeiconfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeIconField)
    """

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class TypeImageField(RealField, abc.ABC):
    """Field mixin for the image of a type.
    TypeImageField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typeimagefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeImageField)
    """

    image: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class TypeVariantField(RealField, abc.ABC):
    """Field mixin for the variant of a type.
    TypeVariantField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typevariantfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeVariantField)
    """

    variant: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class TypeParentField(RealField, abc.ABC):
    """Field mixin for the parent of a type.
    TypeParentField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typeparentfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeParentField)
    """

    parent: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class TypeIsAbstractField(RealField, abc.ABC):
    """Field mixin for the is abstract of a type.
    TypeIsAbstractField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typeisabstractfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeIsAbstractField)
    """

    is_abstract: bool = pydantic.Field(default=False)


class TypeFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a type.
    TypeFolderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typefolderfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeFolderField)
    """

    folder: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class TypeStockField(RealField, abc.ABC):
    """Field mixin for the stock of a type.
    TypeStockField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typestockfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeStockField)
    """

    stock: int = pydantic.Field(default=2147483647)


class TypeVirtualField(RealField, abc.ABC):
    """Field mixin for the virtual of a type.
    TypeVirtualField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typevirtualfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeVirtualField)
    """

    is_virtual: bool = pydantic.Field(default=False)


class TypeScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a type.
    TypeScalableField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typescalablefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeScalableField)
    """

    can_scale: bool = pydantic.Field(default=True)


class TypeMirrborableField(RealField, abc.ABC):
    """Field mixin for the mirrborable of a type.
    TypeMirrborableField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typemirrborablefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeMirrborableField)
    """

    can_mirror: bool = pydantic.Field(default=True)


class TypeUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a type.
    TypeUnitField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typeunitfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeUnitField)
    """

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class TypeLocationField(MaskedField, abc.ABC):
    """Field mixin for the location of a type.
    TypeLocationField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typelocationfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeLocationField)
    """

    location: typing.Optional[Location] = pydantic.Field(default=None)


class TypeCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a type.
    TypeCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typecreatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeCreatedField)
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class TypeUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a type.
    TypeUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖type🛠️typeupdatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeUpdatedField)
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class TypeId(TypeVariantField, TypeNameField, Id):
    """Identity fields for uniquely identifying a type.
    TypeId MUST contain all fields that uniquely identify a type.
    [👤semio📚py💻semio🔖domain🔖type🛠️typeid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeId)
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
    [👤semio📚py💻semio🔖domain🔖type🛠️typeprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeProps)
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
    [👤semio📚py💻semio🔖domain🔖type🛠️typeinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeInput)
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
    TypeVariantField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Output,
):
    """Output fields returned when fetching a type.
    TypeOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖type🛠️typeoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeOutput)
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
    [👤semio📚py💻semio🔖domain🔖type🛠️typecontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeContext)
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
    [👤semio📚py💻semio🔖domain🔖type🛠️type](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/Type)
    """

    PLURAL = "types"

    models: list[Model] = pydantic.Field(default_factory=list)

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

    def parent_entity(self) -> "Kit":
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
            stock=2147483647 if obj.get("stock") is None else obj.get("stock"),
            unit=obj.get("unit", ""),
            parent=parent_guid,
            folder=folder_guid,
        )
        try:
            location_obj = obj.get("location")
            if location_obj:
                entity.location = Location.parse(location_obj) if isinstance(location_obj, dict) else location_obj
        except KeyError, AttributeError:
            pass
        try:
            models = [Model.parse(r) for r in obj["models"]]
            entity.models = models
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
    [👤semio📚py💻semio🔖domain🔖type🛠️typenotfound](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeNotFound)
    """

    def __init__(self, id: "TypeId") -> None:
        self.id = id

    def __str__(self):
        variant = f", {self.id.variant}" if self.id.variant else ""
        return f"Couldn't find the type ({self.id.name}{variant})."


class NoTypeAssigned(NoParentAssigned):
    """No Type Assigned definition.
    NoTypeAssigned MUST fulfill its documented contract.
    [👤semio📚py💻semio🔖domain🔖type🛠️notypeassigned](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/NoTypeAssigned)
    """

    def __str__(self):
        return " The entity has no parent type assigned."


class TypeHasNotAllUsedConnectors(SpecificationError):
    """Type Has Not All Used Connectors definition.
    TypeHasNotAllUsedConnectors MUST fulfill its documented contract.
    [👤semio📚py💻semio🔖domain🔖type🛠️typehasnotallusedconnectors](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeHasNotAllUsedConnectors)
    """

    def __init__(self, missingConnectors: set[str]) -> None:
        self.missingConnectors = missingConnectors

    def __str__(self) -> str:
        return f" A design is using some connectors of the type. The new type is missing the following connectors: {', '.join(self.missingConnectors)}."


class TypeInputNode(InputNode):
    """GraphQL input node for type mutations.
    TypeInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖type🛠️typeinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeInputNode)
    """

    class Meta:
        model = TypeInput


class TypeIdInputNode(InputNode):
    """GraphQL input node for type id mutations.
    TypeIdInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖type🛠️typeidinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Type/d/i/TypeIdInputNode)
    """

    class Meta:
        model = TypeId


# endregion Type

# region Layer
# [👤semio📚py💻semio🔖domain🔖layer](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer)
# Layer entity for organizing design elements into visibility groups.


class LayerNameField(RealField, abc.ABC):
    """Field mixin for the name of a layer.
    LayerNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖layer🛠️layernamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer/d/i/LayerNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class LayerDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a layer.
    LayerDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖layer🛠️layerdescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer/d/i/LayerDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class LayerColorField(RealField, abc.ABC):
    """Field mixin for the color of a layer.
    LayerColorField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖layer🛠️layercolorfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer/d/i/LayerColorField)
    """

    color: str = pydantic.Field(default="", max_length=7)


class LayerIsHiddenField(RealField, abc.ABC):
    """Field mixin for the is hidden of a layer.
    LayerIsHiddenField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖layer🛠️layerishiddenfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer/d/i/LayerIsHiddenField)
    """

    is_hidden: bool = pydantic.Field(default=False)


class LayerIsLockedField(RealField, abc.ABC):
    """Field mixin for the is locked of a layer.
    LayerIsLockedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖layer🛠️layerislockedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer/d/i/LayerIsLockedField)
    """

    is_locked: bool = pydantic.Field(default=False)


class LayerId(LayerNameField, Id):
    """Identity fields for uniquely identifying a layer.
    LayerId MUST contain all fields that uniquely identify a layer.
    [👤semio📚py💻semio🔖domain🔖layer🛠️layerid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer/d/i/LayerId)
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
    [👤semio📚py💻semio🔖domain🔖layer🛠️layerprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer/d/i/LayerProps)
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
    [👤semio📚py💻semio🔖domain🔖layer🛠️layerinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer/d/i/LayerInput)
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
    [👤semio📚py💻semio🔖domain🔖layer🛠️layeroutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer/d/i/LayerOutput)
    """

    pass


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
    [👤semio📚py💻semio🔖domain🔖layer🛠️layer](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Layer/d/i/Layer)
    """

    PLURAL = "layers"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)


# endregion Layer

# region Piece
# [👤semio📚py💻semio🔖domain🔖piece](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece)
# Piece entity for placed instances of types within a design.


class PieceIdField(MaskedField, abc.ABC):
    """Field mixin for the id of a piece.
    PieceIdField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️pieceidfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceIdField)
    """

    id_: str = pydantic.Field(
        default="",
        max_length=ID_LENGTH_LIMIT,
    )


class PieceDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a piece.
    PieceDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️piecedescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class PieceTypeField(MaskedField, abc.ABC):
    """Field mixin for the type of a piece.
    PieceTypeField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️piecetypefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceTypeField)
    """

    type: typing.Optional[TypeId] = pydantic.Field(default=None)


class PieceDesignField(MaskedField, abc.ABC):
    """Field mixin for the design of a piece.
    PieceDesignField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️piecedesignfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceDesignField)
    """

    designPiece: typing.Optional["DesignId"] = pydantic.Field(default=None)


class PiecePlaneField(MaskedField, abc.ABC):
    """Field mixin for the plane of a piece.
    PiecePlaneField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️pieceplanefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PiecePlaneField)
    """

    plane: typing.Optional[Plane] = pydantic.Field(default=None)


class PieceCenterField(MaskedField, abc.ABC):
    """Field mixin for the center of a piece.
    PieceCenterField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️piececenterfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceCenterField)
    """

    center: typing.Optional[Coord] = pydantic.Field(default=None)


class PieceScaleField(RealField, abc.ABC):
    """Field mixin for the scale of a piece.
    PieceScaleField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️piecescalefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceScaleField)
    """

    scale: float = pydantic.Field(default=1.0)


class PieceMirrorPlaneField(MaskedField, abc.ABC):
    """Field mixin for the mirror plane of a piece.
    PieceMirrorPlaneField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️piecemirrorplanefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceMirrorPlaneField)
    """

    mirrorPlane: typing.Optional[Plane] = pydantic.Field(default=None)


class PieceHiddenField(RealField, abc.ABC):
    """Field mixin for the hidden of a piece.
    PieceHiddenField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️piecehiddenfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceHiddenField)
    """

    is_hidden: bool = pydantic.Field(default=False)


class PieceLockedField(RealField, abc.ABC):
    """Field mixin for the locked of a piece.
    PieceLockedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️piecelockedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceLockedField)
    """

    is_locked: bool = pydantic.Field(default=False)


class PieceColorField(RealField, abc.ABC):
    """Field mixin for the color of a piece.
    PieceColorField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖piece🛠️piececolorfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceColorField)
    """

    color: str = pydantic.Field(default="", max_length=7)


class PieceId(PieceIdField, Id):
    """Identity fields for uniquely identifying a piece.
    PieceId MUST contain all fields that uniquely identify a piece.
    [👤semio📚py💻semio🔖domain🔖piece🛠️pieceid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceId)
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
    [👤semio📚py💻semio🔖domain🔖piece🛠️pieceprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceProps)
    """

    pass


class PieceInput(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Input):
    """Input fields for creating or updating a piece.
    PieceInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖piece🛠️pieceinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceInput)
    """

    plane: typing.Optional[PlaneInput] = pydantic.Field(default=None)
    center: typing.Optional[CoordInput] = pydantic.Field(default=None)
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)


class PieceContext(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Context):
    """Context fields for understanding a piece by an LLM.
    PieceContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖piece🛠️piececontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceContext)
    """

    plane: typing.Optional[PlaneContext] = pydantic.Field(default=None)
    center: typing.Optional[CoordContext] = pydantic.Field(default=None)
    attributes: list[AttributeContext] = pydantic.Field(default_factory=list)


class PieceOutput(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Output):
    """Output fields returned when fetching a piece.
    PieceOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖piece🛠️pieceoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceOutput)
    """

    plane: typing.Optional[PlaneOutput] = pydantic.Field(default=None)
    center: typing.Optional[CoordOutput] = pydantic.Field(default=None)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class PiecePrediction(PieceDesignField, PieceTypeField, PieceDescriptionField, PieceIdField, Prediction):
    """Prediction fields for LLM-based piece inference.
    PiecePrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semio🔖domain🔖piece🛠️pieceprediction](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PiecePrediction)
    """

    pass


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
    [👤semio📚py💻semio🔖domain🔖piece🛠️piece](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/Piece)
    """

    PLURAL = "pieces"
    attributes: list[Attribute] = pydantic.Field(default_factory=list)
    connecteds: list["Connection"] = pydantic.Field(default_factory=list)
    connectings: list["Connection"] = pydantic.Field(default_factory=list)

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

    def parent_entity(self) -> "Design":
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
    [👤semio📚py💻semio🔖domain🔖piece🛠️pieceinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceInputNode)
    """

    class Meta:
        model = PieceInput
        exclude_fields = ("type", "designPiece")

    type = TypeIdInputNode()
    designPiece = graphene.Field(lambda: DesignIdInputNode)


class PieceIdInputNode(InputNode):
    """GraphQL input node for piece id mutations.
    PieceIdInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖piece🛠️pieceidinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Piece/d/i/PieceIdInputNode)
    """

    class Meta:
        model = PieceId


# endregion Piece

# region Group
# [👤semio📚py💻semio🔖domain🔖group](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Group)
# Group entity for named collections of pieces in a design.


class GroupNameField(RealField, abc.ABC):
    """Field mixin for the name of a group.
    GroupNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖group🛠️groupnamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Group/d/i/GroupNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class GroupDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a group.
    GroupDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖group🛠️groupdescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Group/d/i/GroupDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class GroupColorField(RealField, abc.ABC):
    """Field mixin for the color of a group.
    GroupColorField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖group🛠️groupcolorfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Group/d/i/GroupColorField)
    """

    color: str = pydantic.Field(default="", max_length=7)


class GroupId(GroupNameField, Id):
    """Identity fields for uniquely identifying a group.
    GroupId MUST contain all fields that uniquely identify a group.
    [👤semio📚py💻semio🔖domain🔖group🛠️groupid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Group/d/i/GroupId)
    """

    pass


class GroupProps(GroupColorField, GroupDescriptionField, GroupNameField, Props):
    """Property fields for a group.
    GroupProps MUST contain all non-relational property fields.
    [👤semio📚py💻semio🔖domain🔖group🛠️groupprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Group/d/i/GroupProps)
    """

    pass


class GroupInput(GroupColorField, GroupDescriptionField, GroupNameField, Input):
    """Input fields for creating or updating a group.
    GroupInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖group🛠️groupinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Group/d/i/GroupInput)
    """

    pass


class GroupOutput(GroupColorField, GroupDescriptionField, GroupNameField, Output):
    """Output fields returned when fetching a group.
    GroupOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖group🛠️groupoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Group/d/i/GroupOutput)
    """

    pieces: list["PieceOutput"] = pydantic.Field(default_factory=list)
    attributes: list[AttributeOutput] = pydantic.Field(default_factory=list)


class Group(GroupColorField, GroupDescriptionField, GroupNameField, TableEntity):
    """Group entity for named collections of pieces.
    Group MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖group🛠️group](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Group/d/i/Group)
    """

    PLURAL = "groups"


# endregion Group

# region Side
# [👤semio📚py💻semio🔖domain🔖side](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Side)
# Side primitive for identifying a specific connector on a specific piece.


class Side(BaseModel):
    """Side primitive identifying a specific connector on a specific piece.
    Side MUST contain all coordinate or geometry fields.
    [👤semio📚py💻semio🔖domain🔖side🛠️side](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Side/d/i/Side)
    """

    piece: PieceId = pydantic.Field()
    designPiece: typing.Optional[PieceId] = pydantic.Field(default=None)
    connector: typing.Optional[ConnectorId] = pydantic.Field(default=None)

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Side", input: str | dict | typing.Any | None) -> "Side":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        piece = PieceId.parse(obj["piece"])
        try:
            connectorObj = obj.get("connector")
            connector = ConnectorId.parse(connectorObj) if connectorObj is not None else None
        except KeyError, TypeError:
            connector = None
        try:
            designPieceObj = obj.get("designPiece")
            designPiece = PieceId.parse(designPieceObj) if designPieceObj is not None else None
        except KeyError, TypeError:
            designPiece = None
        return cls(piece=piece, designPiece=designPiece, connector=connector)


class SideInput(Side, Input):
    """Input fields for creating or updating a side.
    SideInput MUST contain all fields required for creation.
    [👤semio📚py💻semio🔖domain🔖side🛠️sideinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Side/d/i/SideInput)
    """

    pass


class SideContext(Side, Context):
    """Context fields for understanding a side by an LLM.
    SideContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖side🛠️sidecontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Side/d/i/SideContext)
    """

    pass


class SideOutput(Side, Output):
    """Output fields returned when fetching a side.
    SideOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖side🛠️sideoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Side/d/i/SideOutput)
    """

    pass


class SidePrediction(Side, Prediction):
    """Prediction fields for LLM-based side inference.
    SidePrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semio🔖domain🔖side🛠️sideprediction](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Side/d/i/SidePrediction)
    """

    pass


class SideNode(Node):
    """GraphQL node exposing side data.
    SideNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖side🛠️sidenode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Side/d/i/SideNode)
    """

    class Meta:
        model = Side

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
    """GraphQL input node for side mutations.
    SideInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖side🛠️sideinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Side/d/i/SideInputNode)
    """

    class Meta:
        model = SideInput

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
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionconnectedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionConnectedField)
    """

    connected: Side = pydantic.Field()


class ConnectionConnectingField(MaskedField, abc.ABC):
    """Field mixin for the connecting of a connection.
    ConnectionConnectingField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionconnectingfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionConnectingField)
    """

    connecting: Side = pydantic.Field()


class ConnectionDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a connection.
    ConnectionDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectiondescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class ConnectionGapField(RealField, abc.ABC):
    """Field mixin for the gap of a connection.
    ConnectionGapField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectiongapfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionGapField)
    """

    gap: float = pydantic.Field(default=0)


class ConnectionShiftField(RealField, abc.ABC):
    """Field mixin for the shift of a connection.
    ConnectionShiftField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionshiftfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionShiftField)
    """

    shift: float = pydantic.Field(default=0)


class ConnectionRiseField(MaskedField, abc.ABC):
    """Field mixin for the rise of a connection.
    ConnectionRiseField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionrisefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionRiseField)
    """

    rise: float = pydantic.Field(default=0)


class ConnectionRotationField(RealField, abc.ABC):
    """Field mixin for the rotation of a connection.
    ConnectionRotationField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionrotationfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionRotationField)
    """

    rotation: float = pydantic.Field(ge=0, lt=360, default=0)


class ConnectionTurnField(RealField, abc.ABC):
    """Field mixin for the turn of a connection.
    ConnectionTurnField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionturnfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionTurnField)
    """

    turn: float = pydantic.Field(ge=0, lt=360, default=0)


class ConnectionTiltField(RealField, abc.ABC):
    """Field mixin for the tilt of a connection.
    ConnectionTiltField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectiontiltfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionTiltField)
    """

    tilt: float = pydantic.Field(ge=0, lt=360, default=0)


class ConnectionUField(RealField, abc.ABC):
    """Field mixin for the u of a connection.
    ConnectionUField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionufield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionUField)
    """

    u: float = pydantic.Field(default=0)


class ConnectionVField(RealField, abc.ABC):
    """Field mixin for the v of a connection.
    ConnectionVField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionvfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionVField)
    """

    v: float = pydantic.Field(default=0)


class ConnectionId(ConnectionConnectedField, ConnectionConnectingField, Id):
    """Identity fields for uniquely identifying a connection.
    ConnectionId MUST contain all fields that uniquely identify a connection.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionId)
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
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionProps)
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
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectioninput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionInput)
    """

    pass

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
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectioncontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionContext)
    """

    pass

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
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionOutput)
    """

    pass

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
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectionprediction](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionPrediction)
    """

    pass

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
    [👤semio📚py💻semio🔖domain🔖connection🛠️connection](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/Connection)
    """

    PLURAL = "connections"

    attributes: list[Attribute] = pydantic.Field(default_factory=list)

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

    def parent_entity(self) -> "Design":
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
        connectedConnector = None
        if connected.connector is not None:
            connectedConnectorList = [p for p in connectedType.connectors if p.id_ == connected.connector.id_]
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
            connectingConnectorList = [p for p in connectingType.connectors if p.id_ == connecting.connector.id_]
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
            (self.connected.connector.id_ if self.connected.connector is not None else ""),
            self.connecting.piece.id_,
            (self.connecting.connector.id_ if self.connecting.connector is not None else ""),
        ]


class ConnectionInputNode(InputNode):
    """GraphQL input node for connection mutations.
    ConnectionInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖connection🛠️connectioninputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Connection/d/i/ConnectionInputNode)
    """

    class Meta:
        model = ConnectionInput


# endregion Connection

# region Stat
# [👤semio📚py💻semio🔖domain🔖stat](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat)
# Stat entity for recording computed statistics with bounds.


class StatKeyField(RealField, abc.ABC):
    """Field mixin for the key of a stat.
    StatKeyField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖stat🛠️statkeyfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatKeyField)
    """

    key: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class StatUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a stat.
    StatUnitField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖stat🛠️statunitfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatUnitField)
    """

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class StatMinField(RealField, abc.ABC):
    """Field mixin for the min of a stat.
    StatMinField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖stat🛠️statminfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatMinField)
    """

    min: typing.Optional[float] = pydantic.Field(default=None)


class StatMinExcludedField(RealField, abc.ABC):
    """Field mixin for the min excluded of a stat.
    StatMinExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖stat🛠️statminexcludedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatMinExcludedField)
    """

    min_excluded: bool = pydantic.Field(default=False)


class StatMaxField(RealField, abc.ABC):
    """Field mixin for the max of a stat.
    StatMaxField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖stat🛠️statmaxfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatMaxField)
    """

    max: typing.Optional[float] = pydantic.Field(default=None)


class StatMaxExcludedField(RealField, abc.ABC):
    """Field mixin for the max excluded of a stat.
    StatMaxExcludedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖stat🛠️statmaxexcludedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatMaxExcludedField)
    """

    max_excluded: bool = pydantic.Field(default=False)


class StatCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a stat.
    StatCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖stat🛠️statcreatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatCreatedField)
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class StatUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a stat.
    StatUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖stat🛠️statupdatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatUpdatedField)
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class StatId(StatKeyField, Id):
    """Identity fields for uniquely identifying a stat.
    StatId MUST contain all fields that uniquely identify a stat.
    [👤semio📚py💻semio🔖domain🔖stat🛠️statid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatId)
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
    [👤semio📚py💻semio🔖domain🔖stat🛠️statprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatProps)
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
    [👤semio📚py💻semio🔖domain🔖stat🛠️statinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatInput)
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
    [👤semio📚py💻semio🔖domain🔖stat🛠️statoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/StatOutput)
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
):
    """Stat entity for recording computed statistics with bounds.
    Stat MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖stat🛠️stat](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Stat/d/i/Stat)
    """

    PLURAL = "stats"


# endregion Stat

# region Design
# [👤semio📚py💻semio🔖domain🔖design](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design)
# Design entity for composing pieces and connections into assemblies.


class DesignNameField(RealField, abc.ABC):
    """Field mixin for the name of a design.
    DesignNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designnamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class DesignDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a design.
    DesignDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designdescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class DesignIconField(RealField, abc.ABC):
    """Field mixin for the icon of a design.
    DesignIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designiconfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignIconField)
    """

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class DesignImageField(RealField, abc.ABC):
    """Field mixin for the image of a design.
    DesignImageField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designimagefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignImageField)
    """

    image: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class DesignVariantField(RealField, abc.ABC):
    """Field mixin for the variant of a design.
    DesignVariantField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designvariantfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignVariantField)
    """

    variant: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class DesignViewField(RealField, abc.ABC):
    """Field mixin for the view of a design.
    DesignViewField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designviewfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignViewField)
    """

    view: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class DesignParentField(RealField, abc.ABC):
    """Field mixin for the parent of a design.
    DesignParentField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designparentfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignParentField)
    """

    parent: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class DesignIsAbstractField(RealField, abc.ABC):
    """Field mixin for the is abstract of a design.
    DesignIsAbstractField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designisabstractfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignIsAbstractField)
    """

    is_abstract: bool = pydantic.Field(default=False)


class DesignFolderField(RealField, abc.ABC):
    """Field mixin for the folder of a design.
    DesignFolderField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designfolderfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignFolderField)
    """

    folder: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class DesignActiveLayerField(RealField, abc.ABC):
    """Field mixin for the active layer of a design.
    DesignActiveLayerField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designactivelayerfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignActiveLayerField)
    """

    activeLayer: typing.Optional[str] = pydantic.Field(default=None, max_length=ID_LENGTH_LIMIT)


class DesignLocationField(MaskedField, abc.ABC):
    """Field mixin for the location of a design.
    DesignLocationField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designlocationfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignLocationField)
    """

    location: typing.Optional[Location] = pydantic.Field(default=None)


class DesignUnitField(RealField, abc.ABC):
    """Field mixin for the unit of a design.
    DesignUnitField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designunitfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignUnitField)
    """

    unit: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class DesignScalableField(RealField, abc.ABC):
    """Field mixin for the scalable of a design.
    DesignScalableField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designscalablefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignScalableField)
    """

    can_scale: bool = pydantic.Field(default=True)


class DesignMirrorableField(RealField, abc.ABC):
    """Field mixin for the mirrorable of a design.
    DesignMirrorableField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designmirrorablefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignMirrorableField)
    """

    can_mirror: bool = pydantic.Field(default=True)


class DesignCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a design.
    DesignCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designcreatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignCreatedField)
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class DesignUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a design.
    DesignUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖design🛠️designupdatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignUpdatedField)
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class DesignId(DesignNameField, DesignVariantField, Id):
    """Identity fields for uniquely identifying a design.
    DesignId MUST contain all fields that uniquely identify a design.
    [👤semio📚py💻semio🔖domain🔖design🛠️designid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignId)
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
    [👤semio📚py💻semio🔖domain🔖design🛠️designprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignProps)
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
    [👤semio📚py💻semio🔖domain🔖design🛠️designinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignInput)
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
    DesignViewField,
    DesignVariantField,
    DesignDescriptionField,
    DesignNameField,
    Context,
):
    """Context fields for understanding a design by an LLM.
    DesignContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖design🛠️designcontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignContext)
    """

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
    [👤semio📚py💻semio🔖domain🔖design🛠️designoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignOutput)
    """

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
    """Prediction fields for LLM-based design inference.
    DesignPrediction MUST contain all fields for LLM inference.
    [👤semio📚py💻semio🔖domain🔖design🛠️designprediction](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignPrediction)
    """

    pass

    pieces: list[PiecePrediction] = pydantic.Field(default_factory=list)
    connections: list[ConnectionPrediction] = pydantic.Field(default_factory=list)


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
):
    """Design entity composing pieces and connections into an assembly.
    Design MUST implement idMembers and inherit from the appropriate field mixins.
    [👤semio📚py💻semio🔖domain🔖design🛠️design](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/Design)
    """

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

    def parent_entity(self) -> "Kit":
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
            connections = [Connection.parse(c, pieces, designsById) for c in obj["connections"]]
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
    [👤semio📚py💻semio🔖domain🔖design🛠️nodesignassigned](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/NoDesignAssigned)
    """

    def __str__(self):
        return "👪 The entity has no parent design assigned."


class DesignInputNode(InputNode):
    """GraphQL input node for design mutations.
    DesignInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖design🛠️designinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignInputNode)
    """

    class Meta:
        model = DesignInput


class DesignIdInputNode(InputNode):
    """GraphQL input node for design id mutations.
    DesignIdInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖design🛠️designidinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Design/d/i/DesignIdInputNode)
    """

    class Meta:
        model = DesignId


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
    - FILE: Self-contained JSON file
    - FOLDER: Local folder with .semio/kit.db SQLite and asset files
    - ARCHIVE: ZIP file packaging a FolderKit structure
    - REMOTE: URL-addressable kit served over HTTP(S)
    - TEMPORARY: In-memory ephemeral kit (no persistence)
    """

    FILE = "file"
    FOLDER = "folder"
    ARCHIVE = "archive"
    REMOTE = "remote"
    TEMPORARY = "temporary"


ALL_KIT_KINDS: list[KitKind] = list(KitKind)

# #endregion 🔖KitKind


class KitUriField(RealField, abc.ABC):
    """Field mixin for the uri of a kit.
    KitUriField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kiturifield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitUriField)
    """

    uri: str = pydantic.Field(max_length=URI_LENGTH_LIMIT)


class KitNameField(RealField, abc.ABC):
    """Field mixin for the name of a kit.
    KitNameField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitnamefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitNameField)
    """

    name: str = pydantic.Field(max_length=NAME_LENGTH_LIMIT)


class KitDescriptionField(RealField, abc.ABC):
    """Field mixin for the description of a kit.
    KitDescriptionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitdescriptionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitDescriptionField)
    """

    description: str = pydantic.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class KitIconField(RealField, abc.ABC):
    """Field mixin for the icon of a kit.
    KitIconField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kiticonfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitIconField)
    """

    icon: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitImageField(RealField, abc.ABC):
    """Field mixin for the image of a kit.
    KitImageField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitimagefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitImageField)
    """

    image: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitPreviewField(RealField, abc.ABC):
    """Field mixin for the preview of a kit.
    KitPreviewField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitpreviewfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitPreviewField)
    """

    preview: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitVersionField(RealField, abc.ABC):
    """Field mixin for the version of a kit.
    KitVersionField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitversionfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitVersionField)
    """

    version: str = pydantic.Field(default="", max_length=NAME_LENGTH_LIMIT)


class KitRemoteField(RealField, abc.ABC):
    """Field mixin for the remote of a kit.
    KitRemoteField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitremotefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitRemoteField)
    """

    remote: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitHomepageField(RealField, abc.ABC):
    """Field mixin for the homepage of a kit.
    KitHomepageField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kithomepagefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitHomepageField)
    """

    homepage: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitLicenseField(RealField, abc.ABC):
    """Field mixin for the license of a kit.
    KitLicenseField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitlicensefield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitLicenseField)
    """

    license: str = pydantic.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitCreatedField(RealField, abc.ABC):
    """Field mixin for the created of a kit.
    KitCreatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitcreatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitCreatedField)
    """

    created: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class KitUpdatedField(RealField, abc.ABC):
    """Field mixin for the updated of a kit.
    KitUpdatedField MUST declare exactly one field with appropriate constraints.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitupdatedfield](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitUpdatedField)
    """

    updated: datetime.datetime = pydantic.Field(default_factory=datetime.datetime.now)


class KitId(KitUriField, Id):
    """Identity fields for uniquely identifying a kit.
    KitId MUST contain all fields that uniquely identify a kit.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitId)
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
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitprops](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitProps)
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
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitinput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitInput)
    """

    pass

    types: list[TypeInput] = pydantic.Field(default_factory=list)
    designs: list[DesignInput] = pydantic.Field(default_factory=list)
    folders: list[FolderInput] = pydantic.Field(default_factory=list)
    attributes: list[AttributeInput] = pydantic.Field(default_factory=list)
    concepts: list[str] = pydantic.Field(default_factory=list)


class KitContext(KitDescriptionField, KitNameField, Context):
    """Context fields for understanding a kit by an LLM.
    KitContext MUST contain all fields needed for LLM understanding.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitcontext](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitContext)
    """

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
    """Output fields returned when fetching a kit.
    KitOutput MUST contain all fields returned on fetch.
    [👤semio📚py💻semio🔖domain🔖kit🛠️kitoutput](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/KitOutput)
    """

    pass

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
    [👤semio📚py💻semio🔖domain🔖kit🛠️kit](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/d/i/Kit)
    """

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

    def get_design_siblings(self, design_guid: str) -> list["Design"]:
        """Returns all designs with the same parent, excluding self."""
        design = self.find_design_by_guid(design_guid)
        parent_guid = design.parent.guid if design.parent else None
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

    def get_type_siblings(self, type_guid: str) -> list["Type"]:
        """Returns all types with the same parent, excluding self."""
        type_ = self.find_type_by_guid(type_guid)
        parent_guid = type_.parent.guid if type_.parent else None
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
                return port
        raise ValueError(f"Port {port_guid} not found in kit {self.name}")

    def find_piece_in_design(self, design_guid: str, piece_guid: str) -> "Piece":
        """Finds a piece by GUID in a design."""
        design = self.find_design_by_guid(design_guid)
        for piece in design.pieces or []:
            if piece.guid == piece_guid:
                return piece
        raise ValueError(f"Piece {piece_guid} not found in design {design_guid}")

    def find_connection_in_design(self, design_guid: str, connection_guid: str) -> "Connection":
        """Finds a connection by GUID in a design."""
        design = self.find_design_by_guid(design_guid)
        for connection in design.connections or []:
            if connection.guid == connection_guid:
                return connection
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
                return connector
        raise ValueError(f"Connector {connector_guid} not found in type {type_guid}")

    def find_connector_for_piece_in_connection(self, type_guid: str, connection: "Connection", piece_guid: str) -> typing.Optional["Connector"]:
        """Gets the connector used by a piece in a connection."""
        if connection.connected.piece.guid == piece_guid:
            connector_guid = connection.connected.connector.guid if connection.connected.connector else None
        else:
            connector_guid = connection.connecting.connector.guid if connection.connecting.connector else None
        if not connector_guid:
            return None
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
        return result

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
                other_piece_guid = connection.connecting.piece.guid if connection.connected.piece.guid == piece_guid else connection.connected.piece.guid
                other_piece = self.find_piece_in_design(design_guid, other_piece_guid)
                if not other_piece.type or not other_piece.type.guid:
                    continue
                if connection.connected.piece.guid == piece_guid:
                    other_connector_guid = connection.connecting.connector.guid if connection.connecting.connector else None
                else:
                    other_connector_guid = connection.connected.connector.guid if connection.connected.connector else None
                if not other_connector_guid:
                    continue
                other_connector = self.find_connector_in_type(other_piece.type.guid, other_connector_guid)
                required_connectors.append(other_connector)
            except ValueError, AttributeError:
                continue
        result = []
        for replacement_type in self.types or []:
            if replacement_type.isAbstract:
                continue
            if variants is not None and (replacement_type.parent.guid if replacement_type.parent else "") not in variants:
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
                other_piece_guid = connection.connecting.piece.guid if connection.connected.piece.guid == piece_guid else connection.connected.piece.guid
                if other_piece_guid not in piece_guids:
                    try:
                        other_piece = self.find_piece_in_design(design_guid, other_piece_guid)
                        if not other_piece.type or not other_piece.type.guid:
                            continue
                        if connection.connected.piece.guid == piece_guid:
                            other_connector_guid = connection.connecting.connector.guid if connection.connecting.connector else None
                        else:
                            other_connector_guid = connection.connected.connector.guid if connection.connected.connector else None
                        if not other_connector_guid:
                            continue
                        other_connector = self.find_connector_in_type(other_piece.type.guid, other_connector_guid)
                        external_connectors.append(other_connector)
                    except ValueError, AttributeError:
                        continue
        result = []
        for replacement_type in self.types or []:
            if replacement_type.isAbstract:
                continue
            if variants is not None and (replacement_type.parent.guid if replacement_type.parent else "") not in variants:
                continue
            type_connectors = replacement_type.connectors or []
            if len(type_connectors) == 0:
                if len(external_connectors) == 0:
                    result.append(replacement_type)
                continue
            if all(any(True for _ in type_connectors) for _ in external_connectors):
                result.append(replacement_type)
        return result

    # endregion Kit Query Helpers

    # region Filter
    # [👤semio📚py💻semio🔖domain🔖kit🔖filter](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/s/Filter)
    # Filter MUST provide functions to produce a minimal kit subset scoped to a single design.

    @staticmethod
    def _select_best_model_filter(models: list, resolved_tag_guids: list[str]):
        """Selects the best model based on tag matching using Jaccard similarity.
        [👤semio📚py💻semio🔖domain🔖kit🔖filter🛠️selectbestmodelfilter](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/s/Filter/d/i/_select_best_model_filter)
        """
        if not models:
            return None
        if not resolved_tag_guids:
            for m in models:
                if not getattr(m, "tags", None):
                    return m
            return models[0]
        filtered = []
        for m in models:
            model_tag_guids = {t.guid for t in (getattr(m, "tags", None) or [])}
            if all(g in model_tag_guids for g in resolved_tag_guids):
                filtered.append(m)
        if not filtered:
            return None

        def jaccard(m):
            model_tag_guids = {t.guid for t in (getattr(m, "tags", None) or [])}
            sel = set(resolved_tag_guids)
            union = model_tag_guids | sel
            if not union:
                return 0.0
            return len(model_tag_guids & sel) / len(union)

        return max(filtered, key=jaccard)

    @staticmethod
    def _matches_glob_filter(name: str, glob_filter: typing.Optional[dict] = None) -> bool:
        """Checks if a name passes a glob filter with include/exclude patterns.
        [👤semio📚py💻semio🔖domain🔖kit🔖filter🛠️matchesglobfilter](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/s/Filter/d/i/_matches_glob_filter)
        """
        if glob_filter is None:
            return True
        include = glob_filter.get("include") or []
        exclude = glob_filter.get("exclude") or []
        if include and not any(fnmatch.fnmatch(name.lower(), p.lower()) for p in include):
            return False
        if any(fnmatch.fnmatch(name.lower(), p.lower()) for p in exclude):
            return False
        return True

    def filter_kit(self: "Kit", filter_spec: dict) -> "Kit":
        """General-purpose kit filter combining optional design-based transitive filtering with glob-based name filtering.
        When design_guid is set, first performs transitive design-scoped subset extraction.
        Glob filters (include/exclude patterns on names) are applied to each entity kind afterwards.
        [👤semio📚py💻semio🔖domain🔖kit🔖filter🛠️filterkit](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/s/Filter/d/i/filter_kit)
        """
        design_guid = filter_spec.get("design_guid")
        model_tags = filter_spec.get("model_tags")

        if design_guid:
            base = self._filter_kit_by_design(design_guid, model_tags)
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

        return result

    def _filter_kit_by_design(self: "Kit", design_guid: str, tags: typing.Optional[list[str]] = None) -> "Kit":
        """Filters a kit to only include entities related to a specific design.
        Removes types not used by pieces, designs not the target, ports not used by connectors of used types,
        files not used by selected models, tags/concepts only if referenced, and selects one model per type based on tags.
        [👤semio📚py💻semio🔖domain🔖kit🔖filter🛠️filterkitbydesign](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit/s/Filter/d/i/_filter_kit_by_design)
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
            found = False
            for tag in all_tags:
                if tag.guid == tag_value:
                    resolved_tag_guids.append(tag.guid)
                    found = True
                    break
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
                continue
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
                    selected_models[type_guid] = best
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
                continue
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

        return result

    # endregion Filter


# endregion Kit

# region Moved Graphene Nodes
# [👤semio📚py💻semio🔖domain🔖movedgraphenenodes](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes)
# Graphene node definitions moved here due to forward-reference resolution order.


class AttributeNode(TableEntityNode):
    """GraphQL node exposing attribute data.
    AttributeNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️attributenode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/AttributeNode)
    """

    class Meta:
        model = Attribute


class PlaneNode(TableNode):
    """GraphQL node exposing plane data.
    PlaneNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️planenode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/PlaneNode)
    """

    class Meta:
        model = Plane


class AuthorNode(TableEntityNode):
    """GraphQL node exposing author data.
    AuthorNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️authornode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/AuthorNode)
    """

    class Meta:
        model = Author


class ModelNode(TableEntityNode):
    """GraphQL node exposing model data.
    ModelNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️modelnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/ModelNode)
    """

    class Meta:
        model = Model
        excludedFields = ("tags_",)


class ConnectorNode(TableEntityNode):
    """GraphQL node exposing connector data.
    ConnectorNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️connectornode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/ConnectorNode)
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
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️typenode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/TypeNode)
    """

    class Meta:
        model = Type


class PieceNode(TableEntityNode):
    """GraphQL node exposing piece data.
    PieceNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️piecenode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/PieceNode)
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
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️connectionnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/ConnectionNode)
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
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️designnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/DesignNode)
    """

    class Meta:
        model = Design


class KitNotFound(NotFound):
    """endregion Moved Graphene Nodes
    KitNotFound MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️kitnotfound](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/KitNotFound)
    """

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't find an local or remote kit under uri:\n {self.uri}."


class NoKitToDelete(KitNotFound):
    """No Kit To Delete definition.
    NoKitToDelete MUST fulfill its documented contract.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️nokittodelete](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/NoKitToDelete)
    """

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't delete the kit because no local or remote kit was found under uri:\n {self.uri}."


class KitZipDoesNotContainSemioFolder(KitNotFound):
    """Kit Zip Does Not Contain Semio Folder definition.
    KitZipDoesNotContainSemioFolder MUST fulfill its documented contract.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️kitzipdoesnotcontainsemiofolder](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/KitZipDoesNotContainSemioFolder)
    """

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 The remote zip kit ({self.uri}) is not a valid kit."


class OnlyRemoteKitsCanBeCached(ClientError):
    """Only Remote Kits Can Be Cached definition.
    OnlyRemoteKitsCanBeCached MUST fulfill its documented contract.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️onlyremotekitscanbecached](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/OnlyRemoteKitsCanBeCached)
    """

    def __init__(self, nonRemoteUri: str) -> None:
        self.nonRemoteUri = nonRemoteUri

    def __str__(self):
        return f"🔍 Only remote kits can be cached. The uri ({self.nonRemoteUri}) doesn't start with http and ends with .zip"


class KitUriNotValid(ClientError, abc.ABC):
    """🆔 The base for all kit uri not valid errors.
    KitUriNotValid MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️kiturinotvalid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/KitUriNotValid)
    """


class LocalKitUriNotValid(KitUriNotValid, abc.ABC):
    """📂 The base for all local kit uri not valid errors.
    LocalKitUriNotValid MUST be subclassed and MUST NOT be instantiated directly.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️localkiturinotvalid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/LocalKitUriNotValid)
    """


class LocalKitUriIsNotAbsolute(LocalKitUriNotValid):
    """Local Kit Uri Is Not Absolute definition.
    LocalKitUriIsNotAbsolute MUST fulfill its documented contract.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️localkituriisnotabsolute](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/LocalKitUriIsNotAbsolute)
    """

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"📂 The local kit uri ({self.uri}) is relative. It needs to be absolute (include the parent folders, drives, ...)."


class LocalKitUriIsNotDirectory(LocalKitUriNotValid):
    """Local Kit Uri Is Not Directory definition.
    LocalKitUriIsNotDirectory MUST fulfill its documented contract.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️localkituriisnotdirectory](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/LocalKitUriIsNotDirectory)
    """

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"📂 The local kit uri ({self.uri}) is not a directory."


class NoKitAssigned(NoParentAssigned):
    """No Kit Assigned definition.
    NoKitAssigned MUST fulfill its documented contract.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️nokitassigned](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/NoKitAssigned)
    """

    def __str__(self):
        return "👪 The entity has no parent kit assigned."


class KitAlreadyExists(AlreadyExists, abc.ABC):
    """Exception for attempting to create a kit that already exists.
    KitAlreadyExists MUST provide a descriptive error message via __str__.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️kitalreadyexists](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/KitAlreadyExists)
    """

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self) -> str:
        return f"♊ A kit under uri ({self.uri}) already exists."


class KitInputNode(InputNode):
    """GraphQL input node for kit mutations.
    KitInputNode MUST expose the input model via Meta.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️kitinputnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/KitInputNode)
    """

    class Meta:
        model = KitInput


class KitNode(TableEntityNode):
    """GraphQL node exposing kit data.
    KitNode MUST expose the model via Meta.
    [👤semio📚py💻semio🔖domain🔖movedgraphenenodes🛠️kitnode](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Moved%20Graphene%20Nodes/d/i/KitNode)
    """

    class Meta:
        model = Kit


# #endregion 🔖Moved Graphene Nodes

# region Validation
# [👤semio📚py💻semio🔖domain🔖validation](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation)
# Validation logic for checking kit constraints and uniqueness rules.


@dataclasses.dataclass
class ValidationFix:
    """A proposed fix for a validation problem with a title and diff.
    ValidationFix MUST contain a non-empty title and a valid diff dictionary.
    [👤semio📚py💻semio🔖domain🔖validation🛠️validationfix](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/ValidationFix)
    """

    title: str
    diff: dict

    def toDict(self) -> dict:
        return {"title": self.title, "diff": self.diff}


@dataclasses.dataclass
class Problem:
    """A validation problem with a constraint identifier and message.
    Problem MUST contain a non-empty constraint identifier and message.
    [👤semio📚py💻semio🔖domain🔖validation🛠️problem](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/Problem)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validationresult](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/ValidationResult)
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
    """_isGuid performs the _isGuid operation.
    [👤semio📚py💻semio🔖domain🔖validation🛠️isguid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/_isGuid)
    _isGuid MUST perform the _isGuid operation.
    """
    import re

    return bool(
        re.match(
            r"^[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}$",
            s,
            re.IGNORECASE,
        )
    )


def _normalizeGuids(obj: typing.Any) -> typing.Any:
    """_normalizeGuids performs the _normalizeGuids operation.
    [👤semio📚py💻semio🔖domain🔖validation🛠️normalizeguids](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/_normalizeGuids)
    _normalizeGuids MUST perform the _normalizeGuids operation.
    """
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️arevalidationresultsequal](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/areValidationResultsEqual)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️parsevalidationresult](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/parseValidationResult)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validateguiduniqueness](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validateGuidUniqueness)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validatetypenameuniqueness](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validateTypeNameUniqueness)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validatedesignnameuniqueness](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validateDesignNameUniqueness)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validatepiecenameuniqueness](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validatePieceNameUniqueness)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validateportnameuniqueness](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validatePortNameUniqueness)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validatemodelnameuniqueness](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validateModelNameUniqueness)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validatequalitynameuniqueness](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validateQualityNameUniqueness)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validatefilenameuniqueness](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validateFileNameUniqueness)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validatefoldernameuniqueness](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validateFolderNameUniqueness)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validatelayerpathuniqueness](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validateLayerPathUniqueness)
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
    [👤semio📚py💻semio🔖domain🔖validation🛠️validatekit](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/d/i/validateKit)
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
    """_makeFix performs the _makeFix operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖dictbasedvalidation🛠️makefix](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Dict-based%20Validation/d/i/_makeFix)
    _makeFix MUST perform the _makeFix operation.
    """
    return ValidationFix(title=title, diff=diff)


def _deepCopy(obj: typing.Any) -> typing.Any:
    """_deepCopy performs the _deepCopy operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖dictbasedvalidation🛠️deepcopy](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Dict-based%20Validation/d/i/_deepCopy)
    _deepCopy MUST perform the _deepCopy operation.
    """
    return json.loads(json.dumps(obj))


def _newGuid() -> str:
    """_newGuid performs the _newGuid operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖dictbasedvalidation🛠️newguid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Dict-based%20Validation/d/i/_newGuid)
    _newGuid MUST perform the _newGuid operation.
    """
    import uuid

    return str(uuid.uuid4())


def validateKitDict(kit: dict) -> ValidationResult:
    """Validate a kit dictionary against all constraint rules.
    validateKitDict MUST validate a kit dictionary and return results.
    [👤semio📚py💻semio🔖domain🔖validation🔖dictbasedvalidation🛠️validatekitdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Dict-based%20Validation/d/i/validateKitDict)
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


# #endregion 🔖Dict-based Validation

# region Graph Operations
# [👤semio📚py💻semio🔖domain🔖validation🔖graphoperations](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Graph%20Operations)
# Graph construction and traversal for piece connectivity analysis.


def buildPieceGraph(design: Design | dict) -> networkx.Graph:
    """Build a networkx graph from pieces and connections.
    buildPieceGraph MUST return a networkx graph with pieces as nodes.
    [👤semio📚py💻semio🔖domain🔖validation🔖graphoperations🛠️buildpiecegraph](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Graph%20Operations/d/i/buildPieceGraph)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖graphoperations🛠️findfixedpieces](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Graph%20Operations/d/i/findFixedPieces)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖graphoperations🛠️getconnectedcomponents](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Graph%20Operations/d/i/getConnectedComponents)
    """
    G = buildPieceGraph(design)


def getPieceHierarchy(design: Design | dict, rootGuid: str) -> dict[str, int]:
    """Get the hierarchical ordering of pieces from root to leaf.
    getPieceHierarchy MUST return a topological ordering of pieces.
    [👤semio📚py💻semio🔖domain🔖validation🔖graphoperations🛠️getpiecehierarchy](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Graph%20Operations/d/i/getPieceHierarchy)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️gettypebyguid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/getTypeByGuid)
    """
    for t in kit.get("types", []):
        if t.get("guid") == guid:
            return t
    return None


def getConnectorFromType(kit: dict, typeData: dict | None, connectorGuid: str | None) -> dict | None:
    """Look up a connector by name from a type dictionary.
    getConnectorFromType MUST return the matching connector dict.
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️getconnectorfromtype](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/getConnectorFromType)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️planetomatrixdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/planeToMatrixDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️matrixtoplanedict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/matrixToPlaneDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️quaternionfromunitvectorsdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/quaternionFromUnitVectorsDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️quaternionfromaxisangledict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/quaternionFromAxisAngleDict)
    """
    halfAngle = angle / 2
    s = numpy.sin(halfAngle)
    return numpy.array([axis[0] * s, axis[1] * s, axis[2] * s, numpy.cos(halfAngle)])


def quaternionToMatrixDict(q: numpy.ndarray) -> numpy.ndarray:
    """Convert a quaternion to a 3x3 rotation matrix.
    quaternionToMatrixDict MUST produce a valid 3x3 rotation matrix.
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️quaterniontomatrixdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/quaternionToMatrixDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️makerotationaxisdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/makeRotationAxisDict)
    """
    return quaternionToMatrixDict(quaternionFromAxisAngleDict(axis, angle))


def makeTranslationDict(x: float, y: float, z: float) -> numpy.ndarray:
    """Create a 4x4 translation matrix from a displacement vector.
    makeTranslationDict MUST return a 4x4 translation matrix.
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️maketranslationdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/makeTranslationDict)
    """
    m = numpy.eye(4)
    m[0, 3] = x
    m[1, 3] = y
    m[2, 3] = z
    return m


def applyMatrix4ToVec3Dict(m: numpy.ndarray, v: numpy.ndarray) -> numpy.ndarray:
    """Apply a 4x4 matrix to a 3D vector dictionary.
    applyMatrix4ToVec3Dict MUST apply the full affine transformation.
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️applymatrix4tovec3dict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/applyMatrix4ToVec3Dict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️computechildplanedict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/computeChildPlaneDict)
    """
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


def flattenDesignDict(kit: dict, designGuid: str) -> dict:
    """Flatten a nested design hierarchy into a single flat coordinate space.
    flattenDesignDict MUST resolve all nested designs into world coordinates.
    [👤semio📚py💻semio🔖domain🔖validation🔖flattendesign🛠️flattendesigndict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/FlattenDesign/d/i/flattenDesignDict)
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
        piecePaths[rootNode] = rootNode
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
            childCenter = {
                "u": round(childU / TOLERANCE) * TOLERANCE,
                "v": round(childV / TOLERANCE) * TOLERANCE,
            }
            pieceMap[childId]["center"] = childCenter
            piecePaths[childId] = piecePaths.get(parentId, parentId) + "," + childId
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
    [👤semio📚py💻semio🔖domain🔖kitoperations🛠️findattributevaluedict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/d/i/findAttributeValueDict)
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
    return value if value is not None else (defaultValue if defaultValue is not ... else "")


def _findDesignInKitDict(kit: dict, design_guid: str) -> dict:
    """Finds a design by GUID in a kit dict."""
    for d in kit.get("designs", []):
        if d.get("guid") == design_guid:
            return d
    raise ValueError(f"Design {design_guid} not found in kit")


def _findTypeInKitDict(kit: dict, type_guid: str) -> dict:
    """Finds a type by GUID in a kit dict."""
    for t in kit.get("types", []):
        if t.get("guid") == type_guid:
            return t
    raise ValueError(f"Type {type_guid} not found in kit")


def _findPieceInDesignDict(design: dict, piece_guid: str) -> dict:
    """Finds a piece by GUID in a design dict."""
    for p in design.get("pieces", []):
        if p.get("guid") == piece_guid:
            return p
    raise ValueError(f"Piece {piece_guid} not found in design")


def _findPieceConnectionsInDesignDict(design: dict, piece_guid: str) -> list[dict]:
    """Finds all connections involving a piece in a design dict."""
    return [c for c in design.get("connections", []) if c.get("connected", {}).get("piece", {}).get("guid") == piece_guid or c.get("connecting", {}).get("piece", {}).get("guid") == piece_guid]


def _findConnectorInTypeDict(type_dict: dict, connector_guid: str) -> dict:
    """Finds a connector by GUID in a type dict."""
    for c in type_dict.get("connectors", []):
        if c.get("guid") == connector_guid:
            return c
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
            removed_guid = removed.get("guid") if isinstance(removed, dict) else removed
            pieces = [p for p in pieces if p.get("guid") != removed_guid]
        for updated in pieces_diff.get("updated", []):
            piece_id = updated.get("id") or updated.get("piece", {}).get("guid")
            piece_diff = updated.get("diff", {})
            for i, p in enumerate(pieces):
                if p.get("guid") == piece_id:
                    pieces[i] = {
                        **p,
                        **{k: v for k, v in piece_diff.items() if v is not None},
                    }
                    break
        result["pieces"] = pieces
    connections_diff = diff.get("connections")
    if connections_diff:
        connections = list(result.get("connections", []))
        for added in connections_diff.get("added", []):
            connections.append(added)
        for removed in connections_diff.get("removed", []):
            removed_guid = removed.get("guid") if isinstance(removed, dict) else removed
            connections = [c for c in connections if c.get("guid") != removed_guid]
        for updated in connections_diff.get("updated", []):
            conn_id = updated.get("id") or updated.get("connection", {}).get("guid")
            conn_diff = updated.get("diff", {})
            for i, c in enumerate(connections):
                if c.get("guid") == conn_id:
                    connections[i] = {
                        **c,
                        **{k: v for k, v in conn_diff.items() if v is not None},
                    }
                    break
        result["connections"] = connections
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
            result[key] = diff[key]
    return result


def piecesMetadataDict(kit: dict, design_guid: str) -> dict:
    """Returns metadata for all pieces in a design.
    Each entry contains plane, center, fixedPieceId, parentPieceId, depth, and path.
    [👤semio📚py💻semio🔖domain🔖kitoperations🛠️piecesmetadatadict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/d/i/piecesMetadataDict)
    """
    design = _findDesignInKitDict(kit, design_guid)
    flatten_diff = flattenDesignDict(kit, design_guid)
    piece_paths = flatten_diff.pop("_piecePaths", {})
    flat_design = _applyDesignDiffDict(design, flatten_diff)
    result = {}
    for p in flat_design.get("pieces", []):
        guid = p.get("guid", "")
        path_raw = piece_paths.get(guid, guid)
        result[guid] = {
            "plane": p.get("plane"),
            "center": p.get("center", {"u": 0, "v": 0}),
            "fixedPieceId": findAttributeValueDict(p, "semio.fixedPieceId", guid) or guid,
            "parentPieceId": findAttributeValueDict(p, "semio.parentPieceId", None),
            "depth": int(findAttributeValueDict(p, "semio.depth", "0") or "0"),
            "path": [s for s in path_raw.split(",") if s],
        }
    return result


# region 🔖Clustering
# [👤semio📚py💻semio🔖domain🔖kitoperations🔖clustering](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Clustering)
# Functions for clustering and expanding design pieces.


def createClusteredDesignDict(original_design: dict, cluster_piece_ids: list[str], design_name: str) -> dict:
    """Creates a new design from a subset of pieces (cluster).
    Returns a dict with 'clusteredDesign' and 'externalConnections'.
    [👤semio📚py💻semio🔖domain🔖kitoperations🔖clustering🛠️createclustereddesigndict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Clustering/d/i/createClusteredDesignDict)
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
    clustered_design = {
        "guid": str(uuid.uuid4()),
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
    """Returns a DesignDiff that replaces clustered pieces with a design reference.
    [👤semio📚py💻semio🔖domain🔖kitoperations🔖clustering🛠️replaceclusterwithdesigndict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Clustering/d/i/replaceClusterWithDesignDict)
    """
    cluster_set = set(cluster_piece_ids)
    pieces_to_remove = [{"guid": guid} for guid in cluster_piece_ids]
    connections = original_design.get("connections", [])
    connections_to_remove = [{"guid": c.get("guid")} for c in connections if c.get("connected", {}).get("piece", {}).get("guid") in cluster_set or c.get("connecting", {}).get("piece", {}).get("guid") in cluster_set]
    updated_external = []
    for connection in external_connections:
        connected_in_cluster = connection.get("connected", {}).get("piece", {}).get("guid") in cluster_set
        connecting_in_cluster = connection.get("connecting", {}).get("piece", {}).get("guid") in cluster_set
        import copy

        new_conn = copy.deepcopy(connection)
        if connected_in_cluster:
            new_conn.setdefault("connected", {})["designPiece"] = {"guid": clustered_design.get("guid")}
        elif connecting_in_cluster:
            new_conn.setdefault("connecting", {})["designPiece"] = {"guid": clustered_design.get("guid")}
        updated_external.append(new_conn)
    return {
        "pieces": {"removed": pieces_to_remove},
        "connections": {"removed": connections_to_remove, "added": updated_external},
    }


def getClusterableGroupsDict(design: dict, selected_piece_ids: list[str]) -> list[list[str]]:
    """Returns clusterable groups of selected pieces using DFS on connection graph.
    [👤semio📚py💻semio🔖domain🔖kitoperations🔖clustering🛠️getclusterablegroupsdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Clustering/d/i/getClusterableGroupsDict)
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
    piece_guid_set = set(p.get("guid", "") for p in design.get("pieces", []))
    has_design_nodes = any(pid not in piece_guid_set for pid in selected_piece_ids)
    has_multiple_components = len(connected_groups) > 1
    has_large_connected_group = any(len(g) > 1 for g in connected_groups)
    if has_design_nodes or has_multiple_components or has_large_connected_group:
        return [selected_piece_ids]
    return []


def expandDesignPiecesDict(design: dict, kit: dict) -> dict:
    """Recursively expands design references (designPiece) by inlining their pieces and connections.
    [👤semio📚py💻semio🔖domain🔖kitoperations🔖clustering🛠️expanddesignpiecesdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Clustering/d/i/expandDesignPiecesDict)
    """
    import copy

    connections = design.get("connections", [])
    has_design_connections = any(c.get("connected", {}).get("designPiece") or c.get("connecting", {}).get("designPiece") for c in connections)
    if not has_design_connections:
        return design
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
        return expanded
    for design_ref_guid in design_ids:
        referenced = next(
            (d for d in kit.get("designs", []) if d.get("guid") == design_ref_guid),
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
            connected_dp = new_conn.get("connected", {}).get("designPiece")
            if connected_dp and connected_dp.get("guid") == design_ref_guid:
                new_conn["connected"].pop("designPiece", None)
            connecting_dp = new_conn.get("connecting", {}).get("designPiece")
            if connecting_dp and connecting_dp.get("guid") == design_ref_guid:
                new_conn["connecting"].pop("designPiece", None)
            updated_connections.append(new_conn)
        expanded["pieces"] = list(expanded.get("pieces", [])) + transformed_pieces
        expanded["connections"] = updated_connections + transformed_connections
    return expanded


# endregion 🔖Clustering

# region 🔖Kit Query Helpers Dict
# [👤semio📚py💻semio🔖domain🔖kitoperations🔖kitqueryhelpersdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Kit%20Query%20Helpers%20Dict)
# Dict-based kit query helper functions.


def getPrimitiveDesignDict(kit: dict, design_guid: str) -> dict:
    """Gets the primitive (root) design of a design family."""
    current = _findDesignInKitDict(kit, design_guid)
    while current.get("parent", {}).get("guid"):
        current = _findDesignInKitDict(kit, current["parent"]["guid"])
    return current


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
    return family


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
    return current


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
    return family


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
                pass
    return result


def findReplaceableTypesForPieceInDesignDict(
    kit: dict,
    design_guid: str,
    piece_guid: str,
    variants: typing.Optional[list[str]] = None,
) -> list[dict]:
    """Finds all types that can replace a piece while maintaining connection compatibility.
    [👤semio📚py💻semio🔖domain🔖kitoperations🔖kitqueryhelpersdict🛠️findreplaceabletypesforpieceindesigndict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Kit%20Query%20Helpers%20Dict/d/i/findReplaceableTypesForPieceInDesignDict)
    """
    design = _findDesignInKitDict(kit, design_guid)
    connections = _findPieceConnectionsInDesignDict(design, piece_guid)
    required_connectors: list[dict] = []
    for connection in connections:
        try:
            connected_guid = connection.get("connected", {}).get("piece", {}).get("guid")
            connecting_guid = connection.get("connecting", {}).get("piece", {}).get("guid")
            other_piece_guid = connecting_guid if connected_guid == piece_guid else connected_guid
            other_piece = _findPieceInDesignDict(design, other_piece_guid)
            other_type_guid = (other_piece.get("type") or {}).get("guid")
            if not other_type_guid:
                continue
            other_type = _findTypeInKitDict(kit, other_type_guid)
            if connected_guid == piece_guid:
                other_connector_guid = (connection.get("connecting", {}).get("connector") or {}).get("guid")
            else:
                other_connector_guid = (connection.get("connected", {}).get("connector") or {}).get("guid")
            if not other_connector_guid:
                continue
            other_connector = _findConnectorInTypeDict(other_type, other_connector_guid)
            required_connectors.append(other_connector)
        except ValueError, AttributeError, KeyError:
            continue
    result = []
    for replacement_type in kit.get("types", []):
        if replacement_type.get("isAbstract"):
            continue
        if variants is not None:
            parent_guid = (replacement_type.get("parent") or {}).get("guid", "")
            if parent_guid not in variants:
                continue
        type_connectors = replacement_type.get("connectors") or []
        if len(type_connectors) == 0:
            if len(required_connectors) == 0:
                result.append(replacement_type)
            continue
        if all(len(type_connectors) > 0 for _ in required_connectors):
            result.append(replacement_type)
    return result


def findReplaceableTypesForPiecesInDesignDict(
    kit: dict,
    design_guid: str,
    piece_guids: list[str],
    variants: typing.Optional[list[str]] = None,
) -> list[dict]:
    """Finds types that can replace multiple pieces while maintaining all external connections.
    [👤semio📚py💻semio🔖domain🔖kitoperations🔖kitqueryhelpersdict🛠️findreplaceabletypesforpiecesindesigndict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Kit%20Query%20Helpers%20Dict/d/i/findReplaceableTypesForPiecesInDesignDict)
    """
    design = _findDesignInKitDict(kit, design_guid)
    piece_set = set(piece_guids)
    external_connectors: list[dict] = []
    for piece_guid in piece_guids:
        connections = _findPieceConnectionsInDesignDict(design, piece_guid)
        for connection in connections:
            connected_guid = connection.get("connected", {}).get("piece", {}).get("guid")
            connecting_guid = connection.get("connecting", {}).get("piece", {}).get("guid")
            other_piece_guid = connecting_guid if connected_guid == piece_guid else connected_guid
            if other_piece_guid not in piece_set:
                try:
                    other_piece = _findPieceInDesignDict(design, other_piece_guid)
                    other_type_guid = (other_piece.get("type") or {}).get("guid")
                    if not other_type_guid:
                        continue
                    other_type = _findTypeInKitDict(kit, other_type_guid)
                    if connected_guid == piece_guid:
                        other_connector_guid = (connection.get("connecting", {}).get("connector") or {}).get("guid")
                    else:
                        other_connector_guid = (connection.get("connected", {}).get("connector") or {}).get("guid")
                    if not other_connector_guid:
                        continue
                    other_connector = _findConnectorInTypeDict(other_type, other_connector_guid)
                    external_connectors.append(other_connector)
                except ValueError, AttributeError, KeyError:
                    continue
    result = []
    for replacement_type in kit.get("types", []):
        if replacement_type.get("isAbstract"):
            continue
        if variants is not None:
            parent_guid = (replacement_type.get("parent") or {}).get("guid", "")
            if parent_guid not in variants:
                continue
        type_connectors = replacement_type.get("connectors") or []
        if len(type_connectors) == 0:
            if len(external_connectors) == 0:
                result.append(replacement_type)
            continue
        if all(len(type_connectors) > 0 for _ in external_connectors):
            result.append(replacement_type)
    return result


def sumQualityInDesignDict(kit: dict, design_guid: str, quality_guid: str) -> float:
    """Sums up the values of a quality across all pieces in a design.
    For each piece, uses the piece-level prop if present, otherwise falls back to the type-level prop.
    [👤semio📚py💻semio🔖domain🔖kitoperations🔖kitqueryhelpersdict🛠️sumqualityindesigndict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Kit%20Operations/s/Kit%20Query%20Helpers%20Dict/d/i/sumQualityInDesignDict)
    """
    design = _findDesignInKitDict(kit, design_guid)
    total = 0.0
    for piece in design.get("pieces", []):
        piece_prop = next(
            (p for p in piece.get("props", []) if p.get("quality", {}).get("guid") == quality_guid),
            None,
        )
        if piece_prop is not None:
            total += float(piece_prop.get("value", 0))
            continue
        type_ref = piece.get("type", {})
        if type_ref and type_ref.get("guid"):
            try:
                type_dict = _findTypeInKitDict(kit, type_ref["guid"])
                type_prop = next(
                    (p for p in type_dict.get("props", []) if p.get("quality", {}).get("guid") == quality_guid),
                    None,
                )
                if type_prop is not None:
                    total += float(type_prop.get("value", 0))
            except ValueError:
                pass
    return total


# endregion 🔖Kit Query Helpers Dict

# endregion Kit Operations

# region Kit Diff Operations
# [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations)
# Diffing and patching operations for comparing and merging kit versions.


def _normalizeValue(value: typing.Any) -> typing.Any:
    """Normalize empty values to None for comparison.
    _normalizeValue MUST perform the _normalizeValue operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️normalizevalue](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_normalizeValue)
    """
    if value is None or value == "" or value == []:
        return None
    return value


def _normalizeBoolean(value: bool | None) -> bool | None:
    """Normalize boolean: True stays True, False/None become None.
    _normalizeBoolean MUST perform the _normalizeBoolean operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️normalizeboolean](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_normalizeBoolean)
    """
    return True if value else None


def _normalizeArray(arr: list | None) -> list:
    """Normalize None or single item to list.
    _normalizeArray MUST perform the _normalizeArray operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️normalizearray](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_normalizeArray)
    """
    if arr is None:
        return []
    if not isinstance(arr, list):
        return [arr]
    return arr


def areAttributesEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two attribute dictionaries are equal.
    areAttributesEqualDict MUST compare all attribute fields for equality.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️areattributesequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areAttributesEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️arepropsequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/arePropsEqualDict)
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


def areConnectorsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two port dictionaries are equal.
    arePortsEqualDict MUST compare all port fields for equality.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️areconnectorsequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areConnectorsEqualDict)
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
        if not _floatEqual(pointA.get("x"), pointB.get("x")) or not _floatEqual(pointA.get("y"), pointB.get("y")) or not _floatEqual(pointA.get("z"), pointB.get("z")):
            return False
        dirA = connectorA.get("direction", {})
        dirB = connectorB.get("direction", {})
        if not _floatEqual(dirA.get("x"), dirB.get("x")) or not _floatEqual(dirA.get("y"), dirB.get("y")) or not _floatEqual(dirA.get("z"), dirB.get("z")):
            return False
        if not _floatEqual(connectorA.get("t"), connectorB.get("t")):
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️aremodelsequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areModelsEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️aretypesequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areTypesEqualDict)
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
        if not areConnectorsEqualDict(typeA.get("connectors"), typeB.get("connectors"), strict):
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️arepiecesequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/arePiecesEqualDict)
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
    """Extract guid from either a string (Input format) or dict with guid (Output format).
    _getGuidFromRef MUST perform the _getGuidFromRef operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getguidfromref](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getGuidFromRef)
    """
    if ref is None:
        return None
    if isinstance(ref, dict):
        return ref.get("guid")
    return ref


def _floatEqual(a, b, epsilon=1e-9):
    """Compare two float values with epsilon tolerance.
    _floatEqual MUST perform the _floatEqual operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️floatequal](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_floatEqual)
    """
    if a is None and b is None:
        return True
    if a is None or b is None:
        return a == b
    if isinstance(a, (int, float)) and isinstance(b, (int, float)):
        return abs(float(a) - float(b)) < epsilon
    return a == b


def areConnectionsEqualDict(a: list | None, b: list | None, strict: bool = False) -> bool:
    """Check whether two connection dictionaries are equal.
    areConnectionsEqualDict MUST compare all connection fields for equality.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️areconnectionsequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areConnectionsEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️aredesignsequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areDesignsEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️areportsequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/arePortsEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️arequalitiesequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areQualitiesEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️arefilesequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areFilesEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️arefoldersequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areFoldersEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️areauthorsequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areAuthorsEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️areconceptsequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areConceptsEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️aretagsequaldict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areTagsEqualDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️arekitsdictequal](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areKitsDictEqual)
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
    _getCollectionDiff MUST perform the _getCollectionDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getcollectiondiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getCollectionDiff)
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
    _applyCollectionDiff MUST perform the _applyCollectionDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applycollectiondiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyCollectionDiff)
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
    """Get diff between two type dicts.
    _getTypeDiff MUST perform the _getTypeDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️gettypediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getTypeDiff)
    """
    diff: dict = {}
    for key in ["name", "description", "icon", "image", "folder", "unit", "stock"]:
        if _normalizeValue(before.get(key)) != _normalizeValue(after.get(key)):
            diff[key] = after.get(key)
    for key in ["isAbstract", "virtual"]:
        if _normalizeBoolean(before.get(key)) != _normalizeBoolean(after.get(key)):
            diff[key] = after.get(key)
    for refKey in ["location", "parent"]:
        bGuid = before.get(refKey, {}).get("guid") if isinstance(before.get(refKey), dict) else None
        aGuid = after.get(refKey, {}).get("guid") if isinstance(after.get(refKey), dict) else None
        if _normalizeValue(bGuid) != _normalizeValue(aGuid):
            diff[refKey] = after.get(refKey)
    if json.dumps(
        sorted(
            before.get("concepts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("concepts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["concepts"] = after.get("concepts")
    if json.dumps(
        sorted(
            before.get("authors", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("authors", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
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
    modelsDiff = _getCollectionDiff(before.get("models", []), after.get("models", []), _getModelDiff, "model")
    if modelsDiff:
        diff["models"] = modelsDiff
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyTypeDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a type dict.
    _applyTypeDiff MUST perform the _applyTypeDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applytypediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyTypeDiff)
    """
    result = dict(base)
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
            result[key] = diff[key]
    for refKey in ["location", "parent"]:
        if refKey in diff:
            result[refKey] = diff[refKey]
    if "concepts" in diff:
        result["concepts"] = diff["concepts"]
    if "authors" in diff:
        result["authors"] = diff["authors"]
    if diff.get("connectors"):
        result["connectors"] = _applyCollectionDiff(
            base.get("connectors", []),
            diff["connectors"],
            _applyConnectorDiff,
            "connector",
        )
    if diff.get("models"):
        result["models"] = _applyCollectionDiff(base.get("models", []), diff["models"], _applyModelDiff, "model")
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _getConnectorDiff(before: dict, after: dict) -> dict:
    """Get diff between two connector dicts.
    _getConnectorDiff MUST perform the _getConnectorDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getconnectordiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getConnectorDiff)
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
    bPortGuid = before.get("port", {}).get("guid") if isinstance(before.get("port"), dict) else None
    aPortGuid = after.get("port", {}).get("guid") if isinstance(after.get("port"), dict) else None
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
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyConnectorDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a connector dict.
    _applyConnectorDiff MUST perform the _applyConnectorDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applyconnectordiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyConnectorDiff)
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
            result["point"] = {
                "x": (bPoint.get("x", 0) or 0) + (diff["point"].get("x", 0) or 0),
                "y": (bPoint.get("y", 0) or 0) + (diff["point"].get("y", 0) or 0),
                "z": (bPoint.get("z", 0) or 0) + (diff["point"].get("z", 0) or 0),
            }
        else:
            result["point"] = diff["point"]
    if "direction" in diff:
        bDir = base.get("direction", {})
        if bDir and isinstance(bDir, dict):
            result["direction"] = {
                "x": (bDir.get("x", 0) or 0) + (diff["direction"].get("x", 0) or 0),
                "y": (bDir.get("y", 0) or 0) + (diff["direction"].get("y", 0) or 0),
                "z": (bDir.get("z", 0) or 0) + (diff["direction"].get("z", 0) or 0),
            }
        else:
            result["direction"] = diff["direction"]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _getModelDiff(before: dict, after: dict) -> dict:
    """Get diff between two model dicts.
    _getModelDiff MUST perform the _getModelDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getmodeldiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getModelDiff)
    """
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    bFileGuid = before.get("file", {}).get("guid") if isinstance(before.get("file"), dict) else None
    aFileGuid = after.get("file", {}).get("guid") if isinstance(after.get("file"), dict) else None
    if _normalizeValue(bFileGuid) != _normalizeValue(aFileGuid):
        diff["file"] = after.get("file")
    if json.dumps(
        sorted(
            before.get("tags", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("tags", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["tags"] = after.get("tags")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyModelDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a model dict.
    _applyModelDiff MUST perform the _applyModelDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applymodeldiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyModelDiff)
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
    return result


def _getDesignDiff(before: dict, after: dict) -> dict:
    """Get diff between two design dicts.
    _getDesignDiff MUST perform the _getDesignDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getdesigndiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getDesignDiff)
    """
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
        bGuid = before.get(refKey, {}).get("guid") if isinstance(before.get(refKey), dict) else None
        aGuid = after.get(refKey, {}).get("guid") if isinstance(after.get(refKey), dict) else None
        if _normalizeValue(bGuid) != _normalizeValue(aGuid):
            diff[refKey] = after.get(refKey)
    if json.dumps(
        sorted(
            before.get("concepts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("concepts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["concepts"] = after.get("concepts")
    if json.dumps(
        sorted(
            before.get("authors", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("authors", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["authors"] = after.get("authors")
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
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyDesignDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a design dict.
    _applyDesignDiff MUST perform the _applyDesignDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applydesigndiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyDesignDiff)
    """
    result = dict(base)
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
        result["connections"] = _applyCollectionDiff(
            base.get("connections", []),
            diff["connections"],
            _applyConnectionDiff,
            "connection",
        )
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _getPieceDiff(before: dict, after: dict) -> dict:
    """Get diff between two piece dicts.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getpiecediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getPieceDiff)
    _getPieceDiff MUST perform the _getPieceDiff operation.
    """
    diff: dict = {}
    if _normalizeValue(before.get("name")) != _normalizeValue(after.get("name")):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    for refKey in ["type", "design"]:
        bGuid = before.get(refKey, {}).get("guid") if isinstance(before.get(refKey), dict) else None
        aGuid = after.get(refKey, {}).get("guid") if isinstance(after.get(refKey), dict) else None
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
        diff["attributes"] = attributesDiff
    return diff


def _applyPieceDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a piece dict.
    _applyPieceDiff MUST perform the _applyPieceDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applypiecediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyPieceDiff)
    """
    result = dict(base)
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
        if key in diff:
            result[key] = diff[key]
    for refKey in ["type", "design"]:
        if refKey in diff:
            result[refKey] = diff[refKey]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _getConnectionDiff(before: dict, after: dict) -> dict:
    """Get diff between two connection dicts.
    _getConnectionDiff MUST perform the _getConnectionDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getconnectiondiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getConnectionDiff)
    """
    diff: dict = {}
    for key in ["gap", "shift", "rise", "rotation", "turn", "tilt", "u", "v"]:
        bVal = before.get(key, 0) or 0
        aVal = after.get(key, 0) or 0
        delta = aVal - bVal
        if abs(delta) > 1e-10:
            diff[key] = delta
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if before.get("connecting") != after.get("connecting"):
        diff["connecting"] = after.get("connecting")
    if before.get("connected") != after.get("connected"):
        diff["connected"] = after.get("connected")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyConnectionDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a connection dict.
    _applyConnectionDiff MUST perform the _applyConnectionDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applyconnectiondiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyConnectionDiff)
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
    return result


def _getTagDiff(before: dict, after: dict) -> dict:
    """Get diff between two tag dicts.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️gettagdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getTagDiff)
    _getTagDiff MUST perform the _getTagDiff operation.
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
        diff["attributes"] = attributesDiff
    return diff


def _applyTagDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a tag dict.
    _applyTagDiff MUST perform the _applyTagDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applytagdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyTagDiff)
    """
    result = dict(base)
    for key in ["name", "description", "icon"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _getConceptDiff(before: dict, after: dict) -> dict:
    """Get diff between two concept dicts.
    _getConceptDiff MUST perform the _getConceptDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getconceptdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getConceptDiff)
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
        diff["attributes"] = attributesDiff
    return diff


def _applyConceptDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a concept dict.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applyconceptdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyConceptDiff)
    _applyConceptDiff MUST perform the _applyConceptDiff operation.
    """
    result = dict(base)
    for key in ["name", "description", "icon"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _getPortDiff(before: dict, after: dict) -> dict:
    """Get diff between two port dicts.
    _getPortDiff MUST perform the _getPortDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getportdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getPortDiff)
    """
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    if _normalizeValue(before.get("icon")) != _normalizeValue(after.get("icon")):
        diff["icon"] = after.get("icon")
    if json.dumps(
        sorted(
            before.get("compatiblePorts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ) != json.dumps(
        sorted(
            after.get("compatiblePorts", []),
            key=lambda x: x.get("guid", "") if isinstance(x, dict) else str(x),
        )
    ):
        diff["compatiblePorts"] = after.get("compatiblePorts")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyPortDiff(base: dict, diff: dict) -> dict:
    """Apply diff to an port dict.
    _applyPortDiff MUST perform the _applyPortDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applyportdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyPortDiff)
    """
    result = dict(base)
    for key in ["name", "description", "icon"]:
        if key in diff:
            result[key] = diff[key]
    if "compatiblePorts" in diff:
        result["compatiblePorts"] = diff["compatiblePorts"]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _getFileDiff(before: dict, after: dict) -> dict:
    """Get diff between two file dicts.
    _getFileDiff MUST perform the _getFileDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getfilediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getFileDiff)
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
    bFolderGuid = before.get("folder", {}).get("guid") if isinstance(before.get("folder"), dict) else None
    aFolderGuid = after.get("folder", {}).get("guid") if isinstance(after.get("folder"), dict) else None
    if _normalizeValue(bFolderGuid) != _normalizeValue(aFolderGuid):
        diff["folder"] = after.get("folder")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyFileDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a file dict.
    _applyFileDiff MUST perform the _applyFileDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applyfilediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyFileDiff)
    """
    result = dict(base)
    for key in ["name", "description", "remote", "size", "hash", "blob"]:
        if key in diff:
            result[key] = diff[key]
    if "folder" in diff:
        result["folder"] = diff["folder"]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _getFolderDiff(before: dict, after: dict) -> dict:
    """Get diff between two folder dicts.
    _getFolderDiff MUST perform the _getFolderDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getfolderdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getFolderDiff)
    """
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("description")) != _normalizeValue(after.get("description")):
        diff["description"] = after.get("description")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyFolderDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a folder dict.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applyfolderdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyFolderDiff)
    _applyFolderDiff MUST perform the _applyFolderDiff operation.
    """
    result = dict(base)
    for key in ["name", "description"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _getQualityDiff(before: dict, after: dict) -> dict:
    """Get diff between two quality dicts.
    _getQualityDiff MUST perform the _getQualityDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getqualitydiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getQualityDiff)
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
    return diff


def _applyQualityDiff(base: dict, diff: dict) -> dict:
    """Apply diff to a quality dict.
    _applyQualityDiff MUST perform the _applyQualityDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applyqualitydiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyQualityDiff)
    """
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
    """Get diff between two author dicts.
    _getAuthorDiff MUST perform the _getAuthorDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getauthordiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getAuthorDiff)
    """
    diff: dict = {}
    if before.get("name") != after.get("name"):
        diff["name"] = after.get("name")
    if _normalizeValue(before.get("email")) != _normalizeValue(after.get("email")):
        diff["email"] = after.get("email")
    attributesDiff = _getAttributesDiff(before.get("attributes", []), after.get("attributes", []))
    if attributesDiff:
        diff["attributes"] = attributesDiff
    return diff


def _applyAuthorDiff(base: dict, diff: dict) -> dict:
    """Apply diff to an author dict.
    _applyAuthorDiff MUST perform the _applyAuthorDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applyauthordiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyAuthorDiff)
    """
    result = dict(base)
    for key in ["name", "email"]:
        if key in diff:
            result[key] = diff[key]
    if diff.get("attributes") or base.get("attributes"):
        result["attributes"] = _applyAttributesDiff(base.get("attributes", []), diff.get("attributes"))
    return result


def _getAttributeDiff(before: dict, after: dict) -> dict:
    """Get diff between two attribute dicts - used for individual attribute update diffs.
    _getAttributeDiff MUST perform the _getAttributeDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getattributediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getAttributeDiff)
    """
    diff: dict = {}
    if _normalizeValue(before.get("key")) != _normalizeValue(after.get("key")):
        diff["key"] = after.get("key")
    if _normalizeValue(before.get("value")) != _normalizeValue(after.get("value")):
        diff["value"] = after.get("value")
    if _normalizeValue(before.get("definition")) != _normalizeValue(after.get("definition")):
        diff["definition"] = after.get("definition")
    return diff


def _applyAttributeDiff(base: dict, diff: dict) -> dict:
    """Apply diff to an attribute dict.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applyattributediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyAttributeDiff)
    _applyAttributeDiff MUST perform the _applyAttributeDiff operation.
    """
    result = dict(base)
    for key in ["key", "value", "definition"]:
        if key in diff:
            result[key] = diff[key]
    return result


def _getAttributesDiff(before: list, after: list) -> dict:
    """Get diff for attributes collection - uses GUID for identification with EntityId format.
    _getAttributesDiff MUST perform the _getAttributesDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getattributesdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_getAttributesDiff)
    """
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
    """Apply diff to attributes collection - uses GUID for identification with EntityId format.
    _applyAttributesDiff MUST perform the _applyAttributesDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applyattributesdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_applyAttributesDiff)
    """
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
    """Compute inverse of attributes collection diff - uses GUID with EntityId format.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inverseattributesdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseAttributesDiff)
    _inverseAttributesDiff MUST perform the _inverseAttributesDiff operation.
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
    """Compute inverse of an attribute diff.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inverseattributediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseAttributeDiff)
    _inverseAttributeDiff MUST perform the _inverseAttributeDiff operation.
    """
    inverse: dict = {}
    for key in ["key", "value", "definition"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    return inverse


def getKitDiffDict(before: dict, after: dict) -> dict:
    """Compute the diff between two kit dicts.
    getKitDiffDict MUST identify all added, removed and changed entities.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getkitdiffdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/getKitDiffDict)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️applykitdiffdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/applyKitDiffDict)
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
    _inverseCollectionDiff MUST perform the _inverseCollectionDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversecollectiondiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseCollectionDiff)
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
    """Compute inverse of a type diff.
    _inverseTypeDiff MUST perform the _inverseTypeDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversetypediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseTypeDiff)
    """
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
    if appliedDiff.get("models"):
        inverse["models"] = _inverseCollectionDiff(
            original.get("models", []),
            appliedDiff["models"],
            _inverseModelDiff,
            "model",
        )
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def _inverseConnectorDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a connector diff.
    _inverseConnectorDiff MUST perform the _inverseConnectorDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inverseconnectordiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseConnectorDiff)
    """
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
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def _inverseModelDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a model diff.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversemodeldiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseModelDiff)
    _inverseModelDiff MUST perform the _inverseModelDiff operation.
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
    return inverse


def _inverseConnectionDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a connection diff (negate numeric deltas).
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inverseconnectiondiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseConnectionDiff)
    _inverseConnectionDiff MUST perform the _inverseConnectionDiff operation.
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
    return inverse


def _inverseModelDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a model diff.
    _inverseModelDiff MUST perform the _inverseModelDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversemodeldiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseModelDiff)
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
    return inverse


def _inverseConnectionDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a connection diff (negate numeric deltas).
    _inverseConnectionDiff MUST perform the _inverseConnectionDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inverseconnectiondiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseConnectionDiff)
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
    return inverse


def _inverseDesignDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a design diff.
    _inverseDesignDiff MUST perform the _inverseDesignDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversedesigndiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseDesignDiff)
    """
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
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def _inversePieceDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a piece diff.
    _inversePieceDiff MUST perform the _inversePieceDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversepiecediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inversePieceDiff)
    """
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
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def _inverseTagDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a tag diff.
    _inverseTagDiff MUST perform the _inverseTagDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversetagdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseTagDiff)
    """
    inverse: dict = {}
    for key in ["name", "description", "icon"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def _inverseConceptDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a concept diff.
    _inverseConceptDiff MUST perform the _inverseConceptDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inverseconceptdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseConceptDiff)
    """
    inverse: dict = {}
    for key in ["name", "description", "icon"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def _inversePortDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of an port diff.
    _inversePortDiff MUST perform the _inversePortDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inverseportdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inversePortDiff)
    """
    inverse: dict = {}
    for key in ["name", "description", "icon"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "compatiblePorts" in appliedDiff:
        inverse["compatiblePorts"] = original.get("compatiblePorts")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def _inverseFileDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a file diff.
    _inverseFileDiff MUST perform the _inverseFileDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversefilediff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseFileDiff)
    """
    inverse: dict = {}
    for key in ["name", "description", "remote", "size", "hash", "blob"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if "folder" in appliedDiff:
        inverse["folder"] = original.get("folder")
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def _inverseFolderDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a folder diff.
    _inverseFolderDiff MUST perform the _inverseFolderDiff operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversefolderdiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseFolderDiff)
    """
    inverse: dict = {}
    for key in ["name", "description"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def _inverseQualityDiff(original: dict, appliedDiff: dict) -> dict:
    """Compute inverse of a quality diff.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversequalitydiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseQualityDiff)
    _inverseQualityDiff MUST perform the _inverseQualityDiff operation.
    """
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
    """Compute inverse of an author diff.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inverseauthordiff](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_inverseAuthorDiff)
    _inverseAuthorDiff MUST perform the _inverseAuthorDiff operation.
    """
    inverse: dict = {}
    for key in ["name", "email"]:
        if key in appliedDiff:
            inverse[key] = original.get(key)
    if appliedDiff.get("attributes"):
        inverse["attributes"] = _inverseAttributesDiff(original.get("attributes", []), appliedDiff["attributes"])
    return inverse


def inverseKitDiffDict(original: dict, appliedDiff: dict) -> dict:
    """Compute the inverse of a kit diff.
    inverseKitDiffDict MUST swap additions and removals to reverse the diff.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️inversekitdiffdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/inverseKitDiffDict)
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


@dataclasses.dataclass
class Change:
    """Change holds the data fields for a Change record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️change](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/Change)
    Change MUST perform the Change operation.
    """

    forward: dict
    backward: dict
    author: typing.Optional[str] = None
    time: typing.Optional[datetime.datetime] = None
    before: typing.Optional[dict] = None
    after: typing.Optional[dict] = None


def changeToDict(change: Change) -> dict:
    """changeToDict performs the changeToDict operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️changetodict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/changeToDict)
    changeToDict MUST perform the changeToDict operation.
    """
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
    """AttributeChange holds the data fields for a AttributeChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️attributechange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/AttributeChange)
    AttributeChange MUST perform the AttributeChange operation.
    """

    pass


@dataclasses.dataclass
class AuthorChange(Change):
    """AuthorChange holds the data fields for a AuthorChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️authorchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/AuthorChange)
    AuthorChange MUST perform the AuthorChange operation.
    """

    pass


@dataclasses.dataclass
class FileChange(Change):
    """FileChange holds the data fields for a FileChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️filechange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/FileChange)
    FileChange MUST perform the FileChange operation.
    """

    pass


@dataclasses.dataclass
class FolderChange(Change):
    """FolderChange holds the data fields for a FolderChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️folderchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/FolderChange)
    FolderChange MUST perform the FolderChange operation.
    """

    pass


@dataclasses.dataclass
class QualityChange(Change):
    """QualityChange holds the data fields for a QualityChange record.
    QualityChange MUST perform the QualityChange operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️qualitychange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/QualityChange)
    """

    pass


@dataclasses.dataclass
class PortChange(Change):
    """PortChange holds the data fields for a PortChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️portchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/PortChange)
    PortChange MUST perform the PortChange operation.
    """

    pass


@dataclasses.dataclass
class PropChange(Change):
    """PropChange holds the data fields for a PropChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️propchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/PropChange)
    PropChange MUST perform the PropChange operation.
    """

    pass


@dataclasses.dataclass
class TagChange(Change):
    """TagChange holds the data fields for a TagChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️tagchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/TagChange)
    TagChange MUST perform the TagChange operation.
    """

    pass


@dataclasses.dataclass
class ConceptChange(Change):
    """ConceptChange holds the data fields for a ConceptChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️conceptchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/ConceptChange)
    ConceptChange MUST perform the ConceptChange operation.
    """

    pass


@dataclasses.dataclass
class ModelChange(Change):
    """ModelChange holds the data fields for a ModelChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️modelchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/ModelChange)
    ModelChange MUST perform the ModelChange operation.
    """

    pass


@dataclasses.dataclass
class ConnectorChange(Change):
    """ConnectorChange holds the data fields for a ConnectorChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️connectorchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/ConnectorChange)
    ConnectorChange MUST perform the ConnectorChange operation.
    """

    pass


@dataclasses.dataclass
class TypeChange(Change):
    """TypeChange holds the data fields for a TypeChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️typechange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/TypeChange)
    TypeChange MUST perform the TypeChange operation.
    """

    pass


@dataclasses.dataclass
class LayerChange(Change):
    """LayerChange holds the data fields for a LayerChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️layerchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/LayerChange)
    LayerChange MUST perform the LayerChange operation.
    """

    pass


@dataclasses.dataclass
class PieceChange(Change):
    """PieceChange holds the data fields for a PieceChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️piecechange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/PieceChange)
    PieceChange MUST perform the PieceChange operation.
    """

    pass


@dataclasses.dataclass
class GroupChange(Change):
    """GroupChange holds the data fields for a GroupChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️groupchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/GroupChange)
    GroupChange MUST perform the GroupChange operation.
    """

    pass


@dataclasses.dataclass
class ConnectionChange(Change):
    """ConnectionChange holds the data fields for a ConnectionChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️connectionchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/ConnectionChange)
    ConnectionChange MUST perform the ConnectionChange operation.
    """

    pass


@dataclasses.dataclass
class StatChange(Change):
    """StatChange holds the data fields for a StatChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️statchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/StatChange)
    StatChange MUST perform the StatChange operation.
    """

    pass


@dataclasses.dataclass
class DesignChange(Change):
    """DesignChange holds the data fields for a DesignChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️designchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/DesignChange)
    DesignChange MUST perform the DesignChange operation.
    """

    pass


@dataclasses.dataclass
class KitChange(Change):
    """KitChange holds the data fields for a KitChange record.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️kitchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/KitChange)
    KitChange MUST perform the KitChange operation.
    """

    pass


def deletePiecesAndConnectionsInDesignDict(design: dict, pieceGuids: list[str], connectionGuids: list[str]) -> dict:
    """Deletes pieces and connections from a design dict, returning a DesignDiff dict.
    Removes stale connections referencing deleted pieces.
    Updates pieces that become fixed (parent connection removed) with flat plane and zero center.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️deletepiecesandconnectionsindesigndict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/deletePiecesAndConnectionsInDesignDict)
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
    allRemovedConnectionGuids = set(connectionGuids) | staleConnectionGuids

    # Find pieces that become fixed
    fixedPieceGuids: list[str] = []
    for connGuid in allRemovedConnectionGuids:
        conn = next((c for c in connections if c["guid"] == connGuid), None)
        if conn is None:
            continue
        connectingGuid = conn.get("connecting", {}).get("piece", {}).get("guid", "")
        if connectingGuid in deletedPieceSet:
            continue
        # Check if this piece has another parent connection not in the removed set
        hasOtherParent = any(c.get("connecting", {}).get("piece", {}).get("guid", "") == connectingGuid and c["guid"] not in allRemovedConnectionGuids for c in connections)
        if not hasOtherParent and connectingGuid not in fixedPieceGuids:
            fixedPieceGuids.append(connectingGuid)

    flatPlane = {
        "origin": {"x": 0, "y": 0, "z": 0},
        "xAxis": {"x": 1, "y": 0, "z": 0},
        "yAxis": {"x": 0, "y": 1, "z": 0},
    }
    zeroCenter = {"u": 0, "v": 0}

    diff: dict = {}

    piecesRemoved = [{"guid": g} for g in pieceGuids]
    piecesUpdated = [{"piece": {"guid": g}, "diff": {"plane": flatPlane, "center": zeroCenter}} for g in fixedPieceGuids]
    if piecesRemoved or piecesUpdated:
        piecesDiff: dict = {}
        if piecesRemoved:
            piecesDiff["removed"] = piecesRemoved
        if piecesUpdated:
            piecesDiff["updated"] = piecesUpdated
        diff["pieces"] = piecesDiff

    connectionsRemoved = [{"guid": g} for g in sorted(allRemovedConnectionGuids)]
    if connectionsRemoved:
        diff["connections"] = {"removed": connectionsRemoved}

    return diff


def getDesignChange(
    before: dict,
    after: dict,
    author: typing.Optional[str] = None,
    time: typing.Optional[datetime.datetime] = None,
) -> DesignChange:
    """getDesignChange performs the getDesignChange operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getdesignchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/getDesignChange)
    getDesignChange MUST perform the getDesignChange operation.
    """
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
    """getKitChange performs the getKitChange operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️getkitchange](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/getKitChange)
    getKitChange MUST perform the getKitChange operation.
    """
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


def _extractUpdateGuid(update: dict, entityKeys: list[str]) -> str:
    """Extract guid from an updated entry which might use EntityId format or old id format.
    _extractUpdateGuid MUST perform the _extractUpdateGuid operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️extractupdateguid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_extractUpdateGuid)
    """
    for key in entityKeys:
        if key in update and isinstance(update[key], dict):
            return update[key].get("guid", "")
    return update.get("id", "")


FLOAT_EPSILON = 1e-10


def _areDiffDictsEqual(a: dict, b: dict) -> bool:
    """Deep equality check for diff dicts with float epsilon tolerance.
    _areDiffDictsEqual MUST recursively compare dict values with float tolerance.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️arediffdictsequal](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/_areDiffDictsEqual)
    """
    if a is b:
        return True
    if type(a) != type(b):
        if isinstance(a, (int, float)) and isinstance(b, (int, float)):
            return abs(float(a) - float(b)) < FLOAT_EPSILON
        return _normalizeValue(a) == _normalizeValue(b)
    if isinstance(a, dict):
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
    """Deep equality check for kit diffs.
    areKitDiffsDictEqual MUST compare all diff entries for equality.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitdiffoperations🛠️arekitdiffsdictequal](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Diff%20Operations/d/i/areKitDiffsDictEqual)
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

        for guid in addedA:
            if not _areDiffDictsEqual(addedA[guid], addedB[guid]):
                return False

        for guid in updatedA:
            if not _areDiffDictsEqual(updatedA[guid], updatedB[guid]):
                return False

    return True


# endregion Kit Diff Operations

# region Kit Import/Export
# [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export)
# Import and export utilities for kit serialization and deserialization.


class KitData:
    """Simple in-memory kit representation that supports attribute access.
    KitData MUST hold all kit entities in memory for import and export operations.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️kitdata](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/KitData)
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

    def filter_kit(self, filter_spec: dict) -> "KitData":
        """General-purpose kit filter with glob support.
        [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️kitdata🛠️filterkit](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/KitData/filter_kit)
        """
        design_guid = filter_spec.get("design_guid")
        tags = filter_spec.get("model_tags")

        if design_guid:
            base = self._filter_kit_by_design(design_guid, tags)
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
            if include and not any(_fnmatch.fnmatch(name.lower(), p.lower()) for p in include):
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
                filtered[entity_key] = [e for e in filtered.get(entity_key, []) if _matches(e.get(name_key, ""), spec)]

        return KitData(filtered)

    def _filter_kit_by_design(self, design_guid: str, tags: typing.Optional[list[str]] = None) -> "KitData":
        kit = self._data
        design = next((d for d in kit.get("designs", []) if d.get("guid") == design_guid), None)
        if design is None:
            return KitData(
                {
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
            by_guid = next(
                (tag for tag in kit.get("tags", []) if tag.get("guid") == tag_value),
                None,
            )
            if by_guid is not None:
                resolved_tag_guids.append(by_guid["guid"])
                continue
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
                return None
            if not resolved_tag_guids:
                return next((model for model in models if not model.get("tags")), models[0])
            filtered = [model for model in models if all(selected in {tag.get("guid") for tag in model.get("tags", [])} for selected in resolved_tag_guids)]
            if not filtered:
                return None

            def score(model: dict) -> float:
                model_tags = {tag.get("guid") for tag in model.get("tags", [])}
                selected = set(resolved_tag_guids)
                union = model_tags | selected
                return 0.0 if not union else len(model_tags & selected) / len(union)

            return max(filtered, key=score)

        for type_guid in used_type_guids:
            type_item = type_by_guid.get(type_guid)
            if not type_item:
                continue
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
                selected_models[type_guid] = selected_model
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
            port = next(
                (candidate for candidate in kit.get("ports", []) if candidate.get("guid") == port_guid),
                None,
            )
            for compatible in (port or {}).get("compatiblePorts", []):
                if compatible.get("guid"):
                    used_port_guids.add(compatible["guid"])
        used_tag_guids.update(resolved_tag_guids)

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
            if type_item.get("guid") not in used_type_guids:
                continue
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
    """_parse_connector_from_sqlite performs the _parse_connector_from_sqlite operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️parseconnectorfromsqlite](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/_parse_connector_from_sqlite)
    _parse_connector_from_sqlite MUST perform the _parse_connector_from_sqlite operation.
    """
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
    """_parse_model_from_sqlite performs the _parse_model_from_sqlite operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️parsemodelfromsqlite](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/_parse_model_from_sqlite)
    _parse_model_from_sqlite MUST perform the _parse_model_from_sqlite operation.
    """
    return {
        "guid": row.get("guid"),
        "name": row.get("name"),
        "file": row.get("file_guid"),
        "description": row.get("description"),
    }


def _parse_type_from_sqlite(row: dict, connectors: list[dict], models: list[dict]) -> dict:
    """_parse_type_from_sqlite performs the _parse_type_from_sqlite operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️parsetypefromsqlite](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/_parse_type_from_sqlite)
    _parse_type_from_sqlite MUST perform the _parse_type_from_sqlite operation.
    """
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
    """_parse_piece_from_sqlite performs the _parse_piece_from_sqlite operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️parsepiecefromsqlite](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/_parse_piece_from_sqlite)
    _parse_piece_from_sqlite MUST perform the _parse_piece_from_sqlite operation.
    """
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
    """_parse_connection_from_sqlite performs the _parse_connection_from_sqlite operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️parseconnectionfromsqlite](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/_parse_connection_from_sqlite)
    _parse_connection_from_sqlite MUST perform the _parse_connection_from_sqlite operation.
    """
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
    """_parse_design_from_sqlite performs the _parse_design_from_sqlite operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️parsedesignfromsqlite](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/_parse_design_from_sqlite)
    _parse_design_from_sqlite MUST perform the _parse_design_from_sqlite operation.
    """
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
    _build_folder_path MUST perform the _build_folder_path operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️buildfolderpath](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/_build_folder_path)
    """
    for f in kit_dict.get("folders", []):
        if f.get("guid") == folder_guid:
            parent = f.get("parent")
            if parent:
                parent_path = _build_folder_path(kit_dict, parent.get("guid", ""))
                if parent_path:
                    return parent_path + "/" + f.get("name", "")
            return f.get("name", "")
    return ""


def _build_file_path(kit_dict: dict, file_dict: dict) -> str:
    """Build file path from folder hierarchy and file name.
    _build_file_path MUST perform the _build_file_path operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️buildfilepath](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/_build_file_path)
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
    return kit.to_dict() if isinstance(kit, KitData) else kit


def _kit_without_file_blobs(kit: KitData | dict) -> dict:
    """Return a deep copy of a kit dictionary without embedded file blobs.
    _kit_without_file_blobs MUST remove file blob payloads before SQLite and archive persistence.
    """
    kit_copy = copy.deepcopy(_kit_to_dict(kit))
    for file_entry in kit_copy.get("files", []):
        file_entry.pop("blob", None)
    return kit_copy


def _decode_kit_file_blob(blob: str) -> bytes:
    """Decode a kit file blob into raw bytes.
    _decode_kit_file_blob MUST support data URLs and raw base64 payloads.
    """
    encoded = blob.split(",", 1)[1] if blob.startswith("data:") else blob
    return base64.b64decode(encoded)


def _attach_file_blobs_to_kit(kit_dict: dict, files: dict[str, bytes]) -> dict:
    """Attach file blobs from asset bytes to a kit dictionary.
    _attach_file_blobs_to_kit MUST populate file blobs using canonical kit file paths.
    """
    for file_entry in kit_dict.get("files", []):
        file_path = _build_file_path(kit_dict, file_entry)
        if file_path in files:
            encoded = base64.b64encode(files[file_path]).decode("ascii")
            file_entry["blob"] = f"data:application/octet-stream;base64,{encoded}"
    return kit_dict


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
    return collected


def _merge_sqlite_entity(parsed: dict, payload_entity: typing.Optional[dict]) -> dict:
    """Merge a structured SQLite entity with payload metadata.
    _merge_sqlite_entity MUST keep SQLite fields authoritative while preserving unsupported payload fields.
    """
    if payload_entity is None:
        return parsed
    merged = copy.deepcopy(payload_entity)
    for key, value in parsed.items():
        if key in {"connectors", "models", "pieces", "connections"} or value is not None or key not in merged:
            merged[key] = value
    return merged


def _read_kit_from_sqlite(db_path: str) -> dict:
    """Read a kit dictionary from the folder SQLite database.
    _read_kit_from_sqlite MUST rebuild types and designs using the existing SQLite parsing helpers.
    """
    import sqlite3

    if not os.path.exists(db_path):
        raise FileNotFoundError(f"File not found: {db_path}")

    conn = sqlite3.connect(db_path)
    conn.row_factory = sqlite3.Row
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
                return payload_dict
            raise ValueError(f"Invalid kit database: no kit row found in {db_path}")

        payload_types_by_guid = {item.get("guid"): item for item in payload_dict.get("types", []) if item.get("guid")}
        payload_designs_by_guid = {item.get("guid"): item for item in payload_dict.get("designs", []) if item.get("guid")}

        connectors_by_type: dict[str, list[dict]] = {}
        for row in cursor.execute("SELECT * FROM connector ORDER BY guid").fetchall():
            connector = _parse_connector_from_sqlite(dict(row))
            connector["port"] = {"guid": connector["port"]} if connector.get("port") else None
            connectors_by_type.setdefault(row["type_guid"], []).append(connector)

        models_by_type: dict[str, list[dict]] = {}
        for row in cursor.execute("SELECT * FROM model ORDER BY guid").fetchall():
            model = _parse_model_from_sqlite(dict(row))
            model["file"] = {"guid": model["file"]} if model.get("file") else None
            models_by_type.setdefault(row["type_guid"], []).append(model)

        types: list[dict] = []
        for row in cursor.execute("SELECT * FROM type ORDER BY row_id, name, guid").fetchall():
            type_dict = _parse_type_from_sqlite(
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
            design_dict = _parse_design_from_sqlite(
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

        result = {key: copy.deepcopy(value) for key, value in payload_dict.items() if key not in {"types", "designs"}}
        result.update(
            {
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
            }
        )
        return result
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
    return KitData(kit_dict), files


def export_folder_kit(kit: KitData | dict, files: dict[str, bytes], folder_path: str) -> None:
    """Export a folder kit backed by .semio/kit.db.
    export_folder_kit MUST write the SQLite kit database and synchronize asset files into the folder tree.
    """
    data = _kit_to_dict(kit)
    asset_files = _collect_kit_asset_files(data, files)
    os.makedirs(folder_path, exist_ok=True)
    for entry_name in os.listdir(folder_path):
        if entry_name == KIT_LOCAL_FOLDERNAME:
            continue
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
    server_url = f"{parsed.scheme}://{parsed.netloc}"
    try:
        with urllib.request.urlopen(uri) as response:
            body = response.read()
            content_type = response.headers.get_content_type()
    except urllib.error.URLError as error:
        raise ServerUnreachable(server_url) from error

    is_zip = body.startswith(b"PK\x03\x04") or uri.lower().endswith(".zip") or content_type == "application/zip"
    return ("archive" if is_zip else "file"), body, content_type


def import_remote_kit(uri: str) -> tuple[KitData, dict[str, bytes]]:
    """Import a remote kit from JSON or ZIP.
    import_remote_kit MUST support remote JSON and ZIP kit payloads over HTTP(S).
    """
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
    return updated


def edit_folder_kit(folder_path: str, diff: dict) -> KitData:
    """Edit a folder kit in place.
    edit_folder_kit MUST import, apply the diff, persist the SQLite database and asset files, and return the updated kit.
    """
    kit, files = import_folder_kit(folder_path)
    updated = edit_temporary_kit(kit, diff)
    export_folder_kit(updated, _collect_kit_asset_files(updated, files), folder_path)
    return updated


def edit_archive_kit(path: str, diff: dict) -> KitData:
    """Edit an archive kit in place.
    edit_archive_kit MUST import, apply the diff, persist the archive, and return the updated kit.
    """
    kit, files = import_kit(path)
    updated = edit_temporary_kit(kit, diff)
    export_kit(updated, _collect_kit_asset_files(updated, files), path)
    return updated


def _write_remote_kit_bytes(uri: str, body: bytes, content_type: str) -> None:
    """Write remote kit bytes back to their source URI.
    _write_remote_kit_bytes MUST persist edited remote kit content using HTTP PUT.
    """
    parsed = urllib.parse.urlparse(uri)
    if parsed.scheme not in {"http", "https"} or not parsed.netloc:
        raise RemoteKitUriNotValid(uri)
    server_url = f"{parsed.scheme}://{parsed.netloc}"
    request = urllib.request.Request(uri, data=body, method="PUT", headers={"Content-Type": content_type})
    try:
        with urllib.request.urlopen(request):
            pass
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


# endregion Kit Workflow Helpers


def import_kit(path: str) -> tuple[KitData, dict[str, bytes]]:
    """📦Import a kit from a .zip file (containing kit.json and actual files).
    import_kit MUST read kit.json from zip and populate blob from actual files.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️importkit](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/import_kit)
    """
    if not os.path.exists(path):
        raise FileNotFoundError(f"File not found: {path}")

    kit_json_data = None
    files: dict[str, bytes] = {}
    with zipfile.ZipFile(path, "r") as zip_ref:
        for file_info in zip_ref.infolist():
            if file_info.is_dir():
                continue
            name = file_info.filename
            with zip_ref.open(file_info) as f:
                data = f.read()
            if name == "kit.json":
                kit_json_data = data
            elif not name.startswith(".semio/"):
                files[name] = data

    if kit_json_data is None:
        raise ValueError(f"Invalid kit: kit.json not found in {path}")

    kit_dict = json.loads(kit_json_data)
    _attach_file_blobs_to_kit(kit_dict, files)
    return KitData(kit_dict), files


def _write_kit_to_sqlite(kit_data: KitData | dict, db_path: str) -> None:
    """Write kit data to SQLite database using the TypeScript schema.
    _write_kit_to_sqlite MUST perform the _write_kit_to_sqlite operation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️writekittosqlite](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/_write_kit_to_sqlite)
    """
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
            design_guid VARCHAR(36) NOT NULL
        )
    """)

    cursor.execute("""
        CREATE TABLE IF NOT EXISTS kit_payload (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            data TEXT NOT NULL
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
                    _getGuidFromRef(c.get("port")),
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
        cursor.execute(
            """
            INSERT INTO design (guid, name, parent_guid, variant, view_center_u, view_center_v, view_zoom, unit, location_guid, active_layer_guid, is_abstract, folder, can_scale, can_mirror, description, icon, image, created, updated, kit_guid)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        """,
            (
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
            connected_piece_guid = connected_piece.get("guid") if isinstance(connected_piece, dict) else connected_piece
            connected_design_piece = connected.get("designPiece")
            connected_design_piece_guid = connected_design_piece.get("guid") if isinstance(connected_design_piece, dict) else connected_design_piece
            connected_connector = connected.get("connector")
            connected_connector_guid = connected_connector.get("guid") if isinstance(connected_connector, dict) else connected_connector
            connecting_piece = connecting.get("piece")
            connecting_piece_guid = connecting_piece.get("guid") if isinstance(connecting_piece, dict) else connecting_piece
            connecting_design_piece = connecting.get("designPiece")
            connecting_design_piece_guid = connecting_design_piece.get("guid") if isinstance(connecting_design_piece, dict) else connecting_design_piece
            connecting_connector = connecting.get("connector")
            connecting_connector_guid = connecting_connector.get("guid") if isinstance(connecting_connector, dict) else connecting_connector
            cursor.execute(
                """
                INSERT INTO connection (guid, connected_piece_guid, connected_design_piece_guid, connected_connector_guid,
                    connecting_piece_guid, connecting_design_piece_guid, connecting_connector_guid,
                    gap, shift, rise, rotation, turn, tilt, u, v, description, design_guid)
                VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
            """,
                (
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

    cursor.execute(
        "INSERT INTO kit_payload (id, data) VALUES (1, ?)",
        (json.dumps(_kit_without_file_blobs(data), ensure_ascii=False),),
    )

    conn.commit()
    conn.close()


def export_kit(kit: KitData, files: dict[str, bytes], path: str) -> None:
    """📦Export a kit to a .zip file (containing kit.json and actual files).
    export_kit MUST write kit.json (without blob) and actual files to the target path.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitimport🔖export🛠️exportkit](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Import/Export/d/i/export_kit)
    """
    import copy

    data = kit.to_dict() if isinstance(kit, KitData) else kit

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

EXPORT_MODEL_FORMATS: dict[str, str] = {
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
[👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🛠️exportmodelformats](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/c/EXPORT_MODEL_FORMATS)
"""


def _plane_to_matrix_4x4(plane: "Plane") -> numpy.ndarray:
    """Convert a Plane to a 4x4 column-major transformation matrix.
    _plane_to_matrix_4x4 MUST produce an orthonormal basis with z = cross(x, y).
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🛠️planetomatrix4x4](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/i/_plane_to_matrix_4x4)
    """
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
    """Create an identity plane at the world origin with standard axes.
    _identity_plane MUST return a plane with origin=(0,0,0), xAxis=(1,0,0), yAxis=(0,1,0).
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🛠️identityplane](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/i/_identity_plane)
    """
    p = Plane()
    p.origin = Point(x=0.0, y=0.0, z=0.0)
    p.xAxis = Vector(x=1.0, y=0.0, z=0.0)
    p.yAxis = Vector(x=0.0, y=1.0, z=0.0)
    return p


def _type_key_from_id(type_id: "TypeId") -> str:
    """Build a unique string key from a TypeId (name:variant).
    _type_key_from_id MUST produce a consistent key for type matching.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🛠️typekeyfromid](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/i/_type_key_from_id)
    """
    return f"{type_id.name}:{type_id.variant}"


def _type_key_from_type(t: "Type") -> str:
    """Build a unique string key from a Type (name:variant).
    _type_key_from_type MUST produce a consistent key for type matching.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🛠️typekeyfromtype](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/i/_type_key_from_type)
    """
    return f"{t.name}:{t.variant}"


def _find_matching_model(kit: "Kit", type_obj: "Type", tags: list[str]) -> typing.Optional["Model"]:
    """Find the best matching model for a type given requested tags.
    _find_matching_model MUST return the first model whose tags are all in the requested set, or the first model as fallback.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🛠️findmatchingmodel](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/i/_find_matching_model)
    """
    if not type_obj.models or len(type_obj.models) == 0:
        return None
    if not tags or len(tags) == 0:
        default_model = next((model for model in type_obj.models if len(model.tags or []) == 0), None)
        return default_model if default_model is not None else type_obj.models[0]
    tags_set = set(tags)
    for model in type_obj.models:
        model_tag_names = model.tags
        if model_tag_names and all(t in tags_set for t in model_tag_names):
            return model
    return type_obj.models[0]


def _load_glb_mesh_from_bytes(raw: bytes, mesh_name: str | None = None) -> "typing.Any | None":
    """Load a mesh directly from GLB bytes by reading accessors.
    _load_glb_mesh_from_bytes MUST rebuild triangle faces from GLB accessor data without relying on trimesh GLB scene interpretation.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🛠️loadglbmeshfrombytes](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/i/_load_glb_mesh_from_bytes)
    """
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
        if not isinstance(buffer_view_index, int) or buffer_view_index < 0 or buffer_view_index >= len(buffer_views):
            return None
        buffer_view = buffer_views[buffer_view_index]
        component_type = accessor.get("componentType")
        accessor_kind = accessor.get("type")
        count = accessor.get("count")
        if component_type not in component_formats or accessor_kind not in type_widths or not isinstance(count, int):
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
            values.append(_struct.unpack_from("<" + fmt_char * element_width, bin_chunk, start))
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


def _load_type_mesh(kit: "Kit", type_obj: "Type", tags: list[str]) -> "typing.Any | None":
    """Load the 3D mesh for a type from its best-matching model blob.
    _load_type_mesh MUST decode the base64 blob, load with trimesh, and return a single Trimesh.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🛠️loadtypemesh](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/i/_load_type_mesh)
    """
    import base64 as _base64

    import trimesh as _trimesh

    model = _find_matching_model(kit, type_obj, tags)
    if model is None:
        return None
    files_list = kit.files_ or []
    file_id = model.file.guid if hasattr(model.file, "guid") else model.file
    file_obj = next((f for f in files_list if f.name == file_id or f.guid == file_id), None)
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
        meshes = [geometry.copy() for geometry in loaded.geometry.values() if isinstance(geometry, _trimesh.Trimesh) and len(getattr(geometry, "faces", [])) > 0]
        if not meshes:
            return None
        if len(meshes) == 1:
            return meshes[0]
        return _trimesh.util.concatenate(meshes)
    if isinstance(loaded, _trimesh.Trimesh) and len(getattr(loaded, "faces", [])) > 0:
        return loaded
    return None


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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🪨exportdesignmodel](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/i/export_design_model)
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
        design = next(
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
                return None
            return types_by_guid.get(type_ref.get("guid"))

        def _find_connector_dict(type_obj: dict | None, connector_guid: str | None) -> dict | None:
            current = type_obj
            while current is not None:
                connectors = current.get("connectors", []) or []
                if connector_guid is None:
                    return connectors[0] if connectors else None
                for connector in connectors:
                    if connector.get("guid") == connector_guid:
                        return connector
                parent_ref = current.get("parent")
                current = types_by_guid.get(parent_ref.get("guid")) if isinstance(parent_ref, dict) else None
            return None

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
        children_of: dict[str, list[str]] = {piece_guid: [] for piece_guid in piece_by_guid}
        visited: set[str] = set()
        roots: list[str] = []
        queue: list[str] = []

        for piece in pieces:
            piece_guid = piece.get("guid")
            if piece_guid is None:
                continue
            if piece.get("plane") is not None:
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
                    continue
                if connection.get("connected", {}).get("piece", {}).get("guid") != current_guid:
                    continue
                parent_piece = piece_by_guid[current_guid]
                child_piece = piece_by_guid[neighbor_guid]
                parent_type = _find_type_for_piece_dict(parent_piece)
                child_type = _find_type_for_piece_dict(child_piece)
                parent_connector = _find_connector_dict(
                    parent_type,
                    connection.get("connected", {}).get("connector", {}).get("guid"),
                )
                child_connector = _find_connector_dict(
                    child_type,
                    connection.get("connecting", {}).get("connector", {}).get("guid"),
                )
                if parent_connector is not None and child_connector is not None:
                    piece_planes[neighbor_guid] = computeChildPlaneDict(current_plane, parent_connector, child_connector, connection)
                else:
                    piece_planes[neighbor_guid] = current_plane
                parent_of[neighbor_guid] = current_guid
                children_of[current_guid].append(neighbor_guid)
                visited.add(neighbor_guid)
                queue.append(neighbor_guid)

        for piece in pieces:
            piece_guid = piece.get("guid")
            if piece_guid is None:
                continue
            if piece_guid not in visited:
                piece_planes[piece_guid] = _identity_plane_dict()
                roots.append(piece_guid)

        if format == ".ifc":
            return _export_ifc_from_dict(kit, design_id, piece_planes, parent_of, children_of, roots, tags)

        def _select_model_dict(type_obj: dict) -> dict | None:
            models = type_obj.get("models", []) or []
            if len(models) == 0:
                return None
            tag_lookup = {tag.get("guid"): tag for tag in (kit.get("tags", []) or []) if tag.get("guid")}
            if len(tags) == 0:
                default_model = next(
                    (model for model in models if len(model.get("tags", []) or []) == 0),
                    None,
                )
                return default_model if default_model is not None else models[0]
            selected_tag_guids: set[str] = set()
            for tag_value in tags:
                if tag_value in tag_lookup:
                    selected_tag_guids.add(tag_value)
                    continue
                for tag in tag_lookup.values():
                    if tag.get("name") == tag_value:
                        selected_tag_guids.add(tag.get("guid"))
            best_model = None
            best_score = -1.0
            for model in models:
                model_tag_guids = {tag.get("guid") for tag in (model.get("tags", []) or []) if tag.get("guid")}
                if not selected_tag_guids.issubset(model_tag_guids):
                    continue
                union = len(model_tag_guids.union(selected_tag_guids))
                intersection = len(model_tag_guids.intersection(selected_tag_guids))
                score = float(intersection) / float(union) if union > 0 else 0.0
                if score > best_score:
                    best_score = score
                    best_model = model
            return best_model if best_model is not None else models[0]

        scene = _trimesh.Scene()
        type_meshes: dict[str, str] = {}
        files_by_guid = {file_entry.get("guid"): file_entry for file_entry in (kit.get("files", []) or []) if file_entry.get("guid")}
        for piece in pieces:
            type_guid = piece.get("type", {}).get("guid") if isinstance(piece.get("type"), dict) else None
            if type_guid is None or type_guid in type_meshes:
                continue
            type_obj = types_by_guid.get(type_guid)
            if type_obj is None:
                continue
            selected_model = _select_model_dict(type_obj)
            selected_file = files_by_guid.get(selected_model.get("file", {}).get("guid")) if selected_model is not None else None
            mesh = None
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
                            mesh = loaded
                    if mesh is not None and selected_file.get("name"):
                        mesh.metadata["name"] = selected_file.get("name")
                except Exception:
                    mesh = None
            if mesh is None:
                continue
            geometry_name = selected_file.get("name") if selected_file is not None and selected_file.get("name") else type_guid
            type_meshes[type_guid] = geometry_name
            scene.geometry[geometry_name] = mesh

        for piece in pieces:
            piece_guid = piece.get("guid")
            world_plane = piece_planes[piece_guid]
            parent_guid = parent_of.get(piece_guid)
            piece_frame = piece.get("name") or piece_guid
            if parent_guid and parent_guid in piece_planes:
                parent_world = _plane_dict_to_matrix(piece_planes[parent_guid])
                child_world = _plane_dict_to_matrix(world_plane)
                relative = numpy.linalg.inv(parent_world) @ child_world
                frame_from = piece_by_guid[parent_guid].get("name") or parent_guid
            else:
                relative = _plane_dict_to_matrix(world_plane)
                frame_from = scene.graph.base_frame
            relative = _semio_matrix_to_gltf_matrix(relative)
            geom_name = None
            type_guid = piece.get("type", {}).get("guid") if isinstance(piece.get("type"), dict) else None
            if type_guid in type_meshes:
                geom_name = type_meshes[type_guid]
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
        connected_id = conn.connected.piece.id_
        connecting_id = conn.connecting.piece.id_
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

    def _get_connector(type_obj: Type | None, connector_id: ConnectorId | None) -> Connector | None:
        if type_obj is None:
            return None
        if not type_obj.connectors:
            return None
        if connector_id is None:
            return type_obj.connectors[0]
        return next((c for c in type_obj.connectors if c.id_ == connector_id.id_), None)

    queue: list[str] = []
    for p in pieces:
        if p.plane is not None:
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
            is_parent = conn.connected.piece.id_ == current_id
            if not is_parent:
                continue

            parent_id = current_id
            child_id = neighbor_id
            parent_piece = pieces_dict[parent_id]
            child_piece = pieces_dict[child_id]
            parent_type = _get_type(parent_piece)
            child_type = _get_type(child_piece)
            parent_connector = _get_connector(parent_type, conn.connected.connector)
            child_connector = _get_connector(child_type, conn.connecting.connector)

            if parent_connector and child_connector:
                child_plane = computeChildPlane(current_plane, parent_connector, child_connector, conn)
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

    # region Load or create meshes per type
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
        model = _find_matching_model(kit, type_obj, tags)
        if model is not None:
            geometry_name = model.file if isinstance(model.file, str) else None
        if not geometry_name:
            geometry_name = tk
        type_meshes[tk] = geometry_name
        scene.geometry[geometry_name] = mesh
    # endregion Load or create meshes per type

    # region Build scene graph with connection hierarchy
    def _build_node(piece_id: str) -> None:
        piece = pieces_dict[piece_id]
        world_plane = piece_planes[piece_id]
        p_parent = parent_of.get(piece_id)
        children = children_of.get(piece_id, [])
        piece_frame = piece.name or piece.id_

        if p_parent and p_parent in piece_planes:
            parent_world = _plane_to_matrix_4x4(piece_planes[p_parent])
            child_world = _plane_to_matrix_4x4(world_plane)
            relative = _semio_matrix_to_gltf_matrix(numpy.linalg.inv(parent_world) @ child_world)
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
    # endregion Build scene graph with connection hierarchy

    return _export_trimesh_scene(scene, format)


def _export_empty_scene(format: str) -> bytes:
    """Export a minimal valid empty scene for the requested format.
    _export_empty_scene MUST return bytes representing a valid but empty 3D file.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🛠️exportemptyscene](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/i/_export_empty_scene)
    """
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
    """Export a trimesh.Scene to the requested format as bytes.
    _export_trimesh_scene MUST return bytes for all supported formats.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🛠️exporttrimeshscene](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/d/i/_export_trimesh_scene)
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
                        continue
                    buffer["uri"] = "data:application/octet-stream;base64," + base64.b64encode(exported[uri]).decode("ascii")
                for image in gltf_json.get("images", []) or []:
                    uri = image.get("uri")
                    if not uri or uri.startswith("data:") or uri not in exported:
                        continue
                    mime = image.get("mimeType", "application/octet-stream")
                    image["uri"] = f"data:{mime};base64," + base64.b64encode(exported[uri]).decode("ascii")
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


# region IFC Export
# [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🔖ifcexport](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/s/IFC%20Export)
# IFC exporter mapping semio domain to IFC4 schema via ifcopenshell.


def _gltf_xyz_to_semio_xyz(x: float, y: float, z: float) -> tuple[float, float, float]:
    """Convert glTF coordinates to semio/IFC coordinates.
    _gltf_xyz_to_semio_xyz MUST map glTF +Y up geometry back to semio/IFC +Z up geometry.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🔖ifcexport🛠️gltfxyztosemioxyz](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/s/IFC%20Export/d/i/_gltf_xyz_to_semio_xyz)
    """
    return (float(x), float(-z), float(y))


def _glb_bytes_to_vertices_faces(
    raw: bytes,
) -> tuple[list[tuple[float, float, float]], list[tuple[int, ...]]] | None:
    """Extract vertices and faces from GLB bytes for IFC mesh representation.
    _glb_bytes_to_vertices_faces MUST return (vertices, faces) or None if parsing fails.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🔖ifcexport🛠️glbbytestoverticesfaces](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/s/IFC%20Export/d/i/_glb_bytes_to_vertices_faces)
    """
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
        if not isinstance(buffer_view_index, int) or buffer_view_index < 0 or buffer_view_index >= len(buffer_views):
            return None
        buffer_view = buffer_views[buffer_view_index]
        component_type = accessor.get("componentType")
        accessor_kind = accessor.get("type")
        count = accessor.get("count")
        if component_type not in component_formats or accessor_kind not in type_widths or not isinstance(count, int):
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
                continue
            positions = _read_accessor(position_accessor_index)
            if positions is None or positions.ndim != 2 or positions.shape[1] < 3:
                continue
            vertex_offset = len(all_vertices)
            for row in positions:
                all_vertices.append(_gltf_xyz_to_semio_xyz(float(row[0]), float(row[1]), float(row[2])))
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
    """Export a design to IFC4 format from dict-based kit data.
    _export_ifc_from_dict MUST produce a valid IFC4 file with spatial hierarchy, typed occurrences and mesh geometry.
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🔖ifcexport🛠️exportifcfromdict](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/s/IFC%20Export/d/i/_export_ifc_from_dict)
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
    body_context = _ifc_api.run(
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
    design = next(
        (d for d in designs if d.get("name") == design_name or d.get("guid") == design_name),
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
            building = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuilding", name=layer_path)
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
        default_building = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuilding", name="Building")
        _ifc_api.run(
            "aggregate.assign_object",
            ifc,
            relating_object=site,
            products=[default_building],
        )
    if default_storey is None:
        default_storey = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuildingStorey", name="Storey")
        _ifc_api.run(
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
        return default_storey

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
        type_guid = type_ref.get("guid") if isinstance(type_ref, dict) else None
        if type_guid is None or type_guid in ifc_types:
            continue
        type_obj = types_by_guid.get(type_guid)
        if type_obj is None:
            continue
        type_name = type_obj.get("name", type_guid)
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
            type_pset = _ifc_api.run("pset.add_pset", ifc, product=ifc_type, name="SemioTypeAttributes")
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
        if type_obj.get("guid"):
            type_meta["semioGuid"] = type_obj.get("guid")
        if type_meta:
            meta_pset = _ifc_api.run("pset.add_pset", ifc, product=ifc_type, name="SemioTypeMetadata")
            _ifc_api.run("pset.edit_pset", ifc, pset=meta_pset, properties=type_meta)

        # Geometry: find best model, extract GLB mesh
        models = type_obj.get("models", []) or []
        selected_model = None
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
                        selected_model = m
                        break
                if selected_model is None:
                    selected_model = models[0]

        if selected_model is not None:
            file_ref = selected_model.get("file", {})
            file_guid = file_ref.get("guid") if isinstance(file_ref, dict) else file_ref
            file_obj = files_by_guid.get(file_guid)
            if file_obj is not None and file_obj.get("blob"):
                blob = file_obj.get("blob")
                raw = base64.b64decode(blob.split(",", 1)[1] if isinstance(blob, str) and blob.startswith("data:") else blob)
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

        ifc_types[type_guid] = ifc_type
    # endregion Step 3

    # region Step 4: Pieces as occurrences
    ifc_occurrences: dict[str, typing.Any] = {}
    ifc_connector_ports: dict[str, dict[str, typing.Any]] = {}
    for piece in pieces:
        piece_guid = piece.get("guid")
        if piece_guid is None:
            continue
        piece_name = piece.get("name") or piece_guid
        occurrence = _ifc_api.run(
            "root.create_entity",
            ifc,
            ifc_class="IfcBuildingElementProxy",
            name=piece_name,
        )

        type_ref = piece.get("type")
        type_guid = type_ref.get("guid") if isinstance(type_ref, dict) else None
        if type_guid and type_guid in ifc_types:
            _ifc_api.run(
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
            _ifc_api.run("geometry.edit_object_placement", ifc, product=occurrence, matrix=mat)

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
        if piece.get("guid"):
            piece_props["semioGuid"] = piece.get("guid")
        piece_attrs = piece.get("attributes", []) or []
        for attr in piece_attrs:
            key = attr.get("key", "")
            value = attr.get("value", "")
            if key:
                piece_props[key] = value
        if piece_props:
            piece_pset = _ifc_api.run("pset.add_pset", ifc, product=occurrence, name="SemioPieceAttributes")
            _ifc_api.run("pset.edit_pset", ifc, pset=piece_pset, properties=piece_props)

        ifc_occurrences[piece_guid] = occurrence

        # Connectors as ports
        type_obj = types_by_guid.get(type_guid) if type_guid else None
        if type_obj is not None:
            connectors = type_obj.get("connectors", []) or []
            ifc_connector_ports[piece_guid] = {}
            for conn in connectors:
                conn_id = conn.get("guid") or conn.get("id_") or conn.get("name", "")
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
                    conn_props["semioPort"] = port_val if isinstance(port_val, str) else str(port_val)
                conn_pset = _ifc_api.run("pset.add_pset", ifc, product=port, name="SemioConnector")
                _ifc_api.run("pset.edit_pset", ifc, pset=conn_pset, properties=conn_props)

                ifc_connector_ports[piece_guid][conn_id] = port
    # endregion Step 4

    # region Step 5: Connections as port relationships
    for connection in connections:
        connected = connection.get("connected", {})
        connecting = connection.get("connecting", {})
        connected_piece_guid = connected.get("piece", {}).get("guid")
        connecting_piece_guid = connecting.get("piece", {}).get("guid")
        connected_connector_guid = connected.get("connector", {}).get("guid") if connected.get("connector") else None
        connecting_connector_guid = connecting.get("connector", {}).get("guid") if connecting.get("connector") else None

        connected_port = None
        connecting_port = None
        if connected_piece_guid in ifc_connector_ports and connected_connector_guid:
            connected_port = ifc_connector_ports[connected_piece_guid].get(connected_connector_guid)
        if connecting_piece_guid in ifc_connector_ports and connecting_connector_guid:
            connecting_port = ifc_connector_ports[connecting_piece_guid].get(connecting_connector_guid)

        # IfcRelConnectsPorts
        if connected_port is not None and connecting_port is not None:
            ifc.create_entity(
                "IfcRelConnectsPorts",
                GlobalId=_ifc_guid.new(),
                RelatingPort=connected_port,
                RelatedPort=connecting_port,
            )

        # IfcRelConnectsElements
        connected_elem = ifc_occurrences.get(connected_piece_guid)
        connecting_elem = ifc_occurrences.get(connecting_piece_guid)
        if connected_elem is not None and connecting_elem is not None:
            ifc.create_entity(
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
            conn_pset = _ifc_api.run(
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
    [👤semio📚py💻semio🔖domain🔖validation🔖kitmodelexport🔖ifcexport🛠️exportifcfromentities](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Kit%20Model%20Export/s/IFC%20Export/d/i/_export_ifc_from_entities)
    """
    import ifcopenshell as _ifc
    import ifcopenshell.api as _ifc_api
    import ifcopenshell.guid as _ifc_guid

    # region Step 1: IFC file, project, units, context, spatial tree from layers
    ifc = _ifc_api.run("project.create_file", version="IFC4")
    kit_name = kit.name if hasattr(kit, "name") and kit.name else "semio Kit"
    project = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcProject", name=kit_name)
    _ifc_api.run("unit.assign_unit", ifc)
    model_context = _ifc_api.run("context.add_context", ifc, context_type="Model")
    body_context = _ifc_api.run(
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
            building = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuilding", name=layer_name)
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
        default_building = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuilding", name="Building")
        _ifc_api.run(
            "aggregate.assign_object",
            ifc,
            relating_object=site,
            products=[default_building],
        )
    if default_storey is None:
        default_storey = _ifc_api.run("root.create_entity", ifc, ifc_class="IfcBuildingStorey", name="Storey")
        _ifc_api.run(
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
        return default_storey

    # endregion Step 2

    pieces = design.pieces or []
    connections = design.connections or []

    # region Step 3: Types with geometry
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
        ifc_type_name = f"{type_obj.name}:{type_obj.variant}" if type_obj.variant else type_obj.name
        ifc_type = _ifc_api.run(
            "root.create_entity",
            ifc,
            ifc_class="IfcBuildingElementProxyType",
            name=ifc_type_name,
        )

        # Type-level geometry
        model = _find_matching_model(kit, type_obj, tags)
        if model is not None:
            files_list = kit.files_ or []
            file_id = model.file.guid if hasattr(model.file, "guid") else model.file
            file_obj = next((f for f in files_list if f.name == file_id or f.guid == file_id), None)
            if file_obj is not None and file_obj.blob:
                blob = file_obj.blob
                raw = base64.b64decode(blob.split(",", 1)[1] if blob.startswith("data:") else blob)
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
    # endregion Step 3

    # region Step 4: Pieces as occurrences
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
            _ifc_api.run("geometry.edit_object_placement", ifc, product=occurrence, matrix=mat)

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
                _ifc_api.run("geometry.edit_object_placement", ifc, product=port, matrix=port_mat)

                ifc_connector_ports[piece.id_][conn_id] = port
    # endregion Step 4

    # region Step 5: Connections as port relationships
    for conn in connections:
        connected_id = conn.connected.piece.id_
        connecting_id = conn.connecting.piece.id_
        connected_connector_id = conn.connected.connector.id_ if conn.connected.connector else None
        connecting_connector_id = conn.connecting.connector.id_ if conn.connecting.connector else None

        connected_port = None
        connecting_port = None
        if connected_id in ifc_connector_ports and connected_connector_id:
            connected_port = ifc_connector_ports[connected_id].get(connected_connector_id)
        if connecting_id in ifc_connector_ports and connecting_connector_id:
            connecting_port = ifc_connector_ports[connecting_id].get(connecting_connector_id)

        if connected_port is not None and connecting_port is not None:
            ifc.create_entity(
                "IfcRelConnectsPorts",
                GlobalId=_ifc_guid.new(),
                RelatingPort=connected_port,
                RelatedPort=connecting_port,
            )

        connected_elem = ifc_occurrences.get(connected_id)
        connecting_elem = ifc_occurrences.get(connecting_id)
        if connected_elem is not None and connecting_elem is not None:
            ifc.create_entity(
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
            conn_solver_props["description"] = conn.description
        if conn_solver_props and connected_elem is not None:
            conn_pset = _ifc_api.run(
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


@dataclasses.dataclass
class GeometricInsights:
    """Aggregated geometric KPIs for a single mesh or merged scene.
    All geometric data is expressed in the semio coordinate system:
    semio.x = glb.x, semio.y = -glb.x, semio.z = glb.y.
    [👤semio📚py💻semio🔖domain🔖geometricinsights🪨geometricinsights](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Geometric%20Insights/d/i/GeometricInsights)
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


def get_geometric_insights_for_model(model: str | bytes) -> GeometricInsights:
    """Compute key performance indicators for the geometry of a GLB/GLTF model.
    Model MUST be glb or gltf (path or raw bytes). Uses trimesh for mesh analysis.
    [👤semio📚py💻semio🔖domain🔖geometricinsights🛠️getgeometricinsightsformodel](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Geometric%20Insights/d/i/get_geometric_insights_for_model)
    """
    import trimesh as _trimesh

    if isinstance(model, bytes):
        file_type = "glb"
        if len(model) >= 4 and model[:4] == b"glTF":
            file_type = "glb"
        elif len(model) > 0 and model.lstrip().startswith(b"{"):
            file_type = "gltf"
        stream = _trimesh.util.wrap_as_stream(model)
        loaded = _trimesh.load(stream, file_type=file_type)
    else:
        path = pathlib.Path(model)
        if not path.exists():
            raise FileNotFoundError(f"Model file not found: {model}")
        ext = path.suffix.lower()
        if ext not in (".glb", ".gltf"):
            raise ValueError(f"Model MUST be .glb or .gltf, got {ext}")
        file_type = "glb" if ext == ".glb" else "gltf"
        loaded = _trimesh.load(str(path), file_type=file_type)

    if isinstance(loaded, _trimesh.Scene):
        meshes = [g.copy() for g in loaded.geometry.values() if isinstance(g, _trimesh.Trimesh) and len(getattr(g, "faces", [])) > 0]
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
        out.sphericity = float((numpy.pi ** (1 / 3)) * (6 * vol) ** (2 / 3) / out.total_surface_area)
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
        out.slenderness = max_ext / float(numpy.cbrt(mesh.area * max_ext)) if mesh.area > 0 else None

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
            out.genus = (2 - out.euler_characteristic) // 2 if out.euler_characteristic is not None else None
    except Exception:
        pass
    out.is_watertight = bool(mesh.is_watertight)

    # Concavity
    if out.convex_hull_volume is not None and out.convex_hull_volume > 1e-20 and out.enclosed_volume is not None:
        out.concavity_index = 1.0 - (out.enclosed_volume / out.convex_hull_volume)
        out.concavity_index = min(1.0, max(0.0, out.concavity_index))

    return out


def geometric_insights_to_report_dict(insights: GeometricInsights, round_digits: int = 6) -> dict[str, typing.Any]:
    """Serialize GeometricInsights to a JSON-serializable dict for reports. Uses semio Point/Vector as {x,y,z}."""
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
        out["principal_axes"] = [{"x": round(v.x, r), "y": round(v.y, r), "z": round(v.z, r)} for v in insights.principal_axes]
    if insights.moments_of_inertia is not None:
        out["moments_of_inertia"] = [round(x, r) for x in insights.moments_of_inertia]
    for key in ("vertex_count", "face_count", "euler_characteristic", "genus"):
        val = getattr(insights, key, None)
        if val is not None:
            out[key] = val
    if insights.is_watertight is not None:
        out["is_watertight"] = insights.is_watertight
    return out


# endregion Geometric Insights

# region Spatial Math
# [👤semio📚py💻semio🔖domain🔖validation🔖spatialmath](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Spatial%20Math)
# Spatial math utilities for vector normalization and plane computation.


def normalizeVector(v: numpy.ndarray) -> numpy.ndarray:
    """Normalize a 3D vector to unit length.
    normalizeVector MUST return a unit-length vector or raise on zero length.
    [👤semio📚py💻semio🔖domain🔖validation🔖spatialmath🛠️normalizevector](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Spatial%20Math/d/i/normalizeVector)
    """
    length = numpy.linalg.norm(v)
    if length < 1e-10:
        return v
    return v / length


def planeFromYAxis(yAxis: numpy.ndarray, phiDegrees: float = 0.0, origin: numpy.ndarray | None = None) -> Plane:
    """Construct a plane from an origin point and a Y-axis direction.
    planeFromYAxis MUST derive orthogonal x and z axes from the y axis.
    [👤semio📚py💻semio🔖domain🔖validation🔖spatialmath🛠️planefromyaxis](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Spatial%20Math/d/i/planeFromYAxis)
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
    [👤semio📚py💻semio🔖domain🔖validation🔖spatialmath🛠️computechildplane](repo://p/u/semio/b/l/py/f/semio.py/s/Domain/s/Validation/s/Spatial%20Math/d/i/computeChildPlane)
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


# region Meta And Shallow Types
# [👤semio📚py💻main🔖metaandshallowtypes](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types)
# Meta And Shallow Types MUST provide lightweight entity representations.

# region 🔖Sub-entity Meta Types

AttributeMeta = typing.TypedDict(
    "AttributeMeta",
    {"guid": str, "name": str, "value": str, "definition": typing.NotRequired[str]},
)
"""AttributeMeta is identical to Attribute (no list fields to omit)."""

TagMeta = typing.TypedDict(
    "TagMeta",
    {
        "guid": str,
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
        "guid": str,
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

PropMeta = typing.TypedDict(
    "PropMeta",
    {"guid": str, "key": str, "value": str, "unit": typing.NotRequired[str]},
)
"""PropMeta is Prop without attributes."""

AuthorMeta = typing.TypedDict(
    "AuthorMeta",
    {"guid": str, "name": str, "email": typing.NotRequired[str]},
)
"""AuthorMeta is Author without attributes."""

FileMeta = typing.TypedDict(
    "FileMeta",
    {
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

FolderMeta = typing.TypedDict(
    "FolderMeta",
    {
        "guid": str,
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

PortMeta = typing.TypedDict(
    "PortMeta",
    {
        "guid": str,
        "name": str,
        "description": typing.NotRequired[str],
        "icon": typing.NotRequired[str],
    },
)
"""PortMeta is Port without attributes."""

ModelMeta = typing.TypedDict(
    "ModelMeta",
    {
        "guid": str,
        "file": typing.NotRequired[dict],
        "name": typing.NotRequired[str],
        "description": typing.NotRequired[str],
    },
)
"""ModelMeta is Model without tags and attributes."""

ConnectorMeta = typing.TypedDict(
    "ConnectorMeta",
    {
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

LayerMeta = typing.TypedDict(
    "LayerMeta",
    {
        "guid": str,
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

GroupMeta = typing.TypedDict(
    "GroupMeta",
    {
        "guid": str,
        "name": typing.NotRequired[str],
        "color": typing.NotRequired[str],
        "description": typing.NotRequired[str],
    },
)
"""GroupMeta is Group without pieces and attributes."""

ConnectionMeta = typing.TypedDict(
    "ConnectionMeta",
    {
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

TypeMeta = typing.TypedDict(
    "TypeMeta",
    {
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

DesignMeta = typing.TypedDict(
    "DesignMeta",
    {
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

KitMeta = typing.TypedDict(
    "KitMeta",
    {
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

TypeShallow = typing.TypedDict(
    "TypeShallow",
    {
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

DesignShallow = typing.TypedDict(
    "DesignShallow",
    {
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

KitShallow = typing.TypedDict(
    "KitShallow",
    {
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
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️stripnone](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/_strip_none)
    """
    return {k: v for k, v in d.items() if v is not None}


def _extract_scalar_fields(d: dict, keys: list[str]) -> dict:
    """Extract only specified keys from a dict, skipping missing keys.
    _extract_scalar_fields MUST return only the specified scalar fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️extractscalarfields](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/_extract_scalar_fields)
    """
    return {k: d[k] for k in keys if k in d}


_ATTRIBUTE_META_KEYS = ["guid", "name", "value", "definition"]
_TAG_META_KEYS = ["guid", "name", "description", "icon", "order"]
_CONCEPT_META_KEYS = ["guid", "name", "description", "icon", "order"]
_STAT_META_KEYS = [
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
_FILE_META_KEYS = [
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
_QUALITY_META_KEYS = [
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
_CONNECTOR_META_KEYS = [
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
_PIECE_META_KEYS = [
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
_CONNECTION_META_KEYS = [
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

_TYPE_META_KEYS = [
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
_DESIGN_META_KEYS = [
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
_KIT_META_KEYS = [
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
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️attributetometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/attributeToMeta)
    """
    return _extract_scalar_fields(d, _ATTRIBUTE_META_KEYS)


def tagToMeta(d: dict) -> TagMeta:
    """Convert a tag dict to TagMeta.
    tagToMeta MUST extract only TagMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️tagtometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/tagToMeta)
    """
    return _extract_scalar_fields(d, _TAG_META_KEYS)


def conceptToMeta(d: dict) -> ConceptMeta:
    """Convert a concept dict to ConceptMeta.
    conceptToMeta MUST extract only ConceptMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️concepttometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/conceptToMeta)
    """
    return _extract_scalar_fields(d, _CONCEPT_META_KEYS)


def statToMeta(d: dict) -> StatMeta:
    """Convert a stat dict to StatMeta.
    statToMeta MUST extract only StatMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️stattometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/statToMeta)
    """
    return _extract_scalar_fields(d, _STAT_META_KEYS)


def propToMeta(d: dict) -> PropMeta:
    """Convert a prop dict to PropMeta (without attributes).
    propToMeta MUST extract only PropMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️proptometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/propToMeta)
    """
    return _extract_scalar_fields(d, _PROP_META_KEYS)


def authorToMeta(d: dict) -> AuthorMeta:
    """Convert an author dict to AuthorMeta (without attributes).
    authorToMeta MUST extract only AuthorMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️authortometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/authorToMeta)
    """
    return _extract_scalar_fields(d, _AUTHOR_META_KEYS)


def fileToMeta(d: dict) -> FileMeta:
    """Convert a file dict to FileMeta (without blob).
    fileToMeta MUST extract only FileMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️filetometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/fileToMeta)
    """
    return _extract_scalar_fields(d, _FILE_META_KEYS)


def folderToMeta(d: dict) -> FolderMeta:
    """Convert a folder dict to FolderMeta (without attributes).
    folderToMeta MUST extract only FolderMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️foldertometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/folderToMeta)
    """
    return _extract_scalar_fields(d, _FOLDER_META_KEYS)


def qualityToMeta(d: dict) -> QualityMeta:
    """Convert a quality dict to QualityMeta (without benchmarks and attributes).
    qualityToMeta MUST extract only QualityMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️qualitytometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/qualityToMeta)
    """
    return _extract_scalar_fields(d, _QUALITY_META_KEYS)


def portToMeta(d: dict) -> PortMeta:
    """Convert a port dict to PortMeta (without attributes).
    portToMeta MUST extract only PortMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️porttometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/portToMeta)
    """
    return _extract_scalar_fields(d, _PORT_META_KEYS)


def modelToMeta(d: dict) -> ModelMeta:
    """Convert a model dict to ModelMeta (without tags and attributes).
    modelToMeta MUST extract only ModelMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️modeltometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/modelToMeta)
    """
    return _extract_scalar_fields(d, _MODEL_META_KEYS)


def connectorToMeta(d: dict) -> ConnectorMeta:
    """Convert a connector dict to ConnectorMeta (without props and attributes).
    connectorToMeta MUST extract only ConnectorMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️connectortometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/connectorToMeta)
    """
    return _extract_scalar_fields(d, _CONNECTOR_META_KEYS)


def layerToMeta(d: dict) -> LayerMeta:
    """Convert a layer dict to LayerMeta (without attributes).
    layerToMeta MUST extract only LayerMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️layertometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/layerToMeta)
    """
    return _extract_scalar_fields(d, _LAYER_META_KEYS)


def pieceToMeta(d: dict) -> PieceMeta:
    """Convert a piece dict to PieceMeta (without props and attributes).
    pieceToMeta MUST extract only PieceMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️piecetometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/pieceToMeta)
    """
    return _extract_scalar_fields(d, _PIECE_META_KEYS)


def groupToMeta(d: dict) -> GroupMeta:
    """Convert a group dict to GroupMeta (without pieces and attributes).
    groupToMeta MUST extract only GroupMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️grouptometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/groupToMeta)
    """
    return _extract_scalar_fields(d, _GROUP_META_KEYS)


def connectionToMeta(d: dict) -> ConnectionMeta:
    """Convert a connection dict to ConnectionMeta (without attributes).
    connectionToMeta MUST extract only ConnectionMeta fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️connectiontometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/connectionToMeta)
    """
    return _extract_scalar_fields(d, _CONNECTION_META_KEYS)


def typeToMeta(d: dict) -> TypeMeta:
    """Convert a type dict to TypeMeta (scalar fields only).
    typeToMeta MUST extract only TypeMeta scalar fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️typetometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/typeToMeta)
    """
    return _extract_scalar_fields(d, _TYPE_META_KEYS)


def designToMeta(d: dict) -> DesignMeta:
    """Convert a design dict to DesignMeta (scalar fields only).
    designToMeta MUST extract only DesignMeta scalar fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️designtometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/designToMeta)
    """
    return _extract_scalar_fields(d, _DESIGN_META_KEYS)


def kitToMeta(d: dict) -> KitMeta:
    """Convert a kit dict to KitMeta (scalar fields only).
    kitToMeta MUST extract only KitMeta scalar fields.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️kittometa](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/kitToMeta)
    """
    return _extract_scalar_fields(d, _KIT_META_KEYS)


def _convert_list(items: list | None, converter: typing.Callable) -> list | None:
    """Convert a list of dicts using a converter function, returning None for empty/missing lists.
    _convert_list MUST return None for empty or missing lists.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️convertlist](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/_convert_list)
    """
    if not items:
        return None
    return [converter(item) for item in items]


def typeToShallow(d: dict) -> TypeShallow:
    """Convert a type dict to TypeShallow (list fields replaced by Meta items).
    typeToShallow MUST convert list fields to Meta item lists.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️typetoshallow](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/typeToShallow)
    """
    result = _extract_scalar_fields(d, _TYPE_META_KEYS)
    concepts = _convert_list(d.get("concepts"), lambda c: c if isinstance(c, str) else conceptToMeta(c))
    if concepts is not None:
        result["concepts"] = concepts
    authors = _convert_list(d.get("authors"), lambda a: a if isinstance(a, str) else authorToMeta(a))
    if authors is not None:
        result["authors"] = authors
    props = _convert_list(d.get("props"), propToMeta)
    if props is not None:
        result["props"] = props
    models = _convert_list(d.get("models"), modelToMeta)
    if models is not None:
        result["models"] = models
    connectors = _convert_list(d.get("connectors"), connectorToMeta)
    if connectors is not None:
        result["connectors"] = connectors
    attributes = _convert_list(d.get("attributes"), attributeToMeta)
    if attributes is not None:
        result["attributes"] = attributes
    return result


def designToShallow(d: dict) -> DesignShallow:
    """Convert a design dict to DesignShallow (list fields replaced by Meta items).
    designToShallow MUST convert list fields to Meta item lists.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️designtoshallow](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/designToShallow)
    """
    result = _extract_scalar_fields(d, _DESIGN_META_KEYS)
    concepts = _convert_list(d.get("concepts"), lambda c: c if isinstance(c, str) else conceptToMeta(c))
    if concepts is not None:
        result["concepts"] = concepts
    authors = _convert_list(d.get("authors"), lambda a: a if isinstance(a, str) else authorToMeta(a))
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
    """Convert a kit dict to KitShallow (list fields replaced by Meta items).
    kitToShallow MUST convert list fields to Meta item lists.
    [👤semio📚py💻main🔖metaandshallowtypes🔖metaandshallowconversionfunctions🛠️kittoshallow](repo://p/u/semio/b/l/py/f/main.py/s/Meta%20And%20Shallow%20Types/s/Meta%20And%20Shallow%20Conversion%20Functions/d/i/kitToShallow)
    """
    result = _extract_scalar_fields(d, _KIT_META_KEYS)
    concepts = _convert_list(d.get("concepts"), lambda c: c if isinstance(c, str) else conceptToMeta(c))
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
    authors = _convert_list(d.get("authors"), lambda a: a if isinstance(a, str) else authorToMeta(a))
    if authors is not None:
        result["authors"] = authors
    attributes = _convert_list(d.get("attributes"), attributeToMeta)
    if attributes is not None:
        result["attributes"] = attributes
    return result


# endregion 🔖Meta And Shallow Conversion Functions

# endregion Meta And Shallow Types


# region Test
# [👤semio📚py💻main🔖test](repo://p/u/semio/b/l/py/f/main.py/s/Test)
# Tests for the semio py module.

TEST_TOLERANCE = 0.001
TEST_ASSETS_DIR = "../assets/semio"
REPORTS_EXPORT_DIR = pathlib.Path(__file__).resolve().parents[2] / "reports" / "export-design-model"
REPORTS_MODEL_KPI_DIR = pathlib.Path(__file__).resolve().parents[2] / "reports" / "model-kpi"


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
    for key in [
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
                        m["file"] = ""
                    if "url" not in m or m["url"] is None:
                        m["url"] = ""
                    if "tags" in m and isinstance(m["tags"], list):
                        new_tags = [(tag["guid"] if isinstance(tag, dict) and "guid" in tag else tag) for tag in m["tags"]]
                        m["tags"] = new_tags
                    elif "tags" not in m:
                        m["tags"] = []
    return data


def _test_build_workflow_kit() -> dict:
    """Build a compact kit fixture for workflow roundtrip tests."""
    asset_blob = "data:text/plain;base64," + base64.b64encode(b"workflow asset payload").decode("ascii")
    return {
        "guid": "11111111-1111-1111-1111-111111111111",
        "name": "Workflow Kit",
        "version": "1.0.0",
        "description": "Kit workflow fixture.",
        "types": [
            {
                "guid": "22222222-2222-2222-2222-222222222222",
                "name": "Workflow Type",
                "connectors": [],
                "models": [
                    {
                        "guid": "33333333-3333-3333-3333-333333333333",
                        "name": "Workflow Model",
                        "file": {"guid": "44444444-4444-4444-4444-444444444444"},
                    }
                ],
            }
        ],
        "designs": [
            {
                "guid": "55555555-5555-5555-5555-555555555555",
                "name": "Workflow Design",
                "pieces": [
                    {
                        "guid": "66666666-6666-6666-6666-666666666666",
                        "id": "Piece-1",
                        "type": {"guid": "22222222-2222-2222-2222-222222222222"},
                    }
                ],
                "connections": [],
            }
        ],
        "files": [
            {
                "guid": "44444444-4444-4444-4444-444444444444",
                "name": "asset.txt",
                "folder": {"guid": "77777777-7777-7777-7777-777777777777"},
                "blob": asset_blob,
            }
        ],
        "folders": [
            {
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
    return {
        "name": updated_name,
        "files": {
            "updated": [
                {
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
            item["content_type"] = self.headers.get("Content-Type", item["content_type"])
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
    return _test_is_close(v1.get("x", 0), v2.get("x", 0)) and _test_is_close(v1.get("y", 0), v2.get("y", 0)) and _test_is_close(v1.get("z", 0), v2.get("z", 0))


def _test_planes_equal(p1, p2):
    if p1 is None or p2 is None:
        return False
    if not p1.get("origin") or not p2.get("origin"):
        return False
    if not p1.get("xAxis") or not p2.get("xAxis"):
        return False
    if not p1.get("yAxis") or not p2.get("yAxis"):
        return False
    return _test_vectors_equal(p1.get("origin"), p2.get("origin")) and _test_vectors_equal(p1.get("xAxis"), p2.get("xAxis")) and _test_vectors_equal(p1.get("yAxis"), p2.get("yAxis"))


def _test_centers_equal(c1, c2):
    if c1 is None or c2 is None:
        return c1 == c2
    return _test_is_close(c1.get("u", 0), c2.get("u", 0)) and _test_is_close(c1.get("v", 0), c2.get("v", 0))


def _test_find_design(kit: dict, name: str, parent_name: str = None) -> dict:
    parent_guid = None
    if parent_name:
        for d in kit.get("designs", []):
            if d.get("name") == parent_name:
                parent_guid = d.get("guid")
                break
        if not parent_guid:
            raise ValueError(f"Parent {parent_name} not found")

    for d in kit.get("designs", []):
        if d.get("name") == name:
            p = d.get("parent")
            if parent_guid:
                if p and p.get("guid") == parent_guid:
                    return d
            else:
                if not p:
                    return d
    raise ValueError(f"Design {name} not found")


def _test_flatten(design_name, parent_name=None):
    kit_dict = _test_load_json("metabolism.kit.semio.json")
    design = _test_find_design(kit_dict, design_name, parent_name)

    expected_design = next(
        (d for d in kit_dict.get("designs", []) if d.get("name") == "Flat" and d.get("parent", {}).get("guid") == design.get("guid")),
        None,
    )
    assert expected_design is not None, f"Expected Flat design for {design_name} not found"

    flat_design_diff = flattenDesignDict(kit_dict, design.get("guid"))
    flat_design = _applyDesignDiff(design, flat_design_diff)

    for piece in flat_design.get("pieces", []):
        expected_piece = next(
            (x for x in expected_design.get("pieces", []) if x.get("name") == piece.get("name")),
            None,
        )
        assert expected_piece is not None, f"Piece {piece.get('name')} not found in expected design"
        assert piece.get("plane") is not None
        assert piece.get("center") is not None
        assert _test_planes_equal(piece.get("plane"), expected_piece.get("plane"))
        assert _test_centers_equal(piece.get("center"), expected_piece.get("center"))


def _test_contains_all_tags(model: dict[str, typing.Any], selected_tag_guids: list[str]) -> bool:
    model_tag_guids = [t.get("guid") if isinstance(t, dict) else t for t in model.get("tags", [])]
    return all(guid in model_tag_guids for guid in selected_tag_guids)


def _test_jaccard_tag_guids(model_tag_guids: list[str], selected_tag_guids: list[str]) -> float:
    if len(model_tag_guids) == 0 and len(selected_tag_guids) == 0:
        return 1.0
    set_a = set(model_tag_guids)
    set_b = set(selected_tag_guids)
    union = set_a | set_b
    if len(union) == 0:
        return 0.0
    return len(set_a & set_b) / len(union)


def _test_select_best_model_like_semio_ts(models: list[dict[str, typing.Any]], selected_tag_guids: list[str]) -> dict[str, typing.Any] | None:
    if len(models) == 0:
        return None
    if len(selected_tag_guids) == 0:
        default_model = next((model for model in models if len(model.get("tags", [])) == 0), None)
        return default_model if default_model is not None else models[0]
    filtered_models = [model for model in models if _test_contains_all_tags(model, selected_tag_guids)]
    if len(filtered_models) == 0:
        return None
    indexed_scores = [
        _test_jaccard_tag_guids(
            [t.get("guid") if isinstance(t, dict) else t for t in model.get("tags", [])],
            selected_tag_guids,
        )
        for model in filtered_models
    ]
    max_score = max(indexed_scores)
    max_score_index = indexed_scores.index(max_score)
    return filtered_models[max_score_index]


def _test_create_glb_blob(vertices: list[tuple[float, float, float]], faces: list[tuple[int, int, int]]) -> str:
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
    return "data:model/gltf-binary;base64," + base64.b64encode(glb).decode("ascii")


class TestRoundtrip:
    class TestMetabolism:
        def test_roundtrip(self):
            kit_dict = _test_load_json("metabolism.kit.semio.json")
            serialized = json.dumps(kit_dict)
            deserialized = json.loads(serialized)
            assert areKitsDictEqual(kit_dict, deserialized), "JSON -> Memory -> JSON: serialized and deserialized kit should be equal"

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

            assert areKitsDictEqual(kit_dict, kit2.to_dict()), "ZIP -> JSON: roundtrip kit should be equal"
            assert len(files2) == len(files), f"Expected {len(files)} files, got {len(files2)}"

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
            diff = _test_build_workflow_diff("Workflow Folder Edited", "asset-folder.txt")

            with tempfile.TemporaryDirectory() as tmpdir:
                export_folder_kit(KitData(kit_dict), files, tmpdir)
                assert os.path.exists(os.path.join(tmpdir, KIT_LOCAL_SUFFIX))

                imported, imported_files = import_folder_kit(tmpdir)
                assert areKitsDictEqual(kit_dict, imported.to_dict())
                assert imported_files == files

                edited = edit_folder_kit(tmpdir, diff)
                roundtrip, roundtrip_files = import_folder_kit(tmpdir)

                assert not os.path.exists(os.path.join(tmpdir, "assets", "asset.txt"))
                assert os.path.exists(os.path.join(tmpdir, "assets", "asset-folder.txt"))

            assert edited.name == "Workflow Folder Edited"
            assert roundtrip.name == "Workflow Folder Edited"
            assert roundtrip.to_dict()["files"][0]["name"] == "asset-folder.txt"
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
                    _test_build_workflow_diff("Workflow Remote Json Edited", "asset-remote-json.txt"),
                )
                edited_zip = edit_remote_kit(
                    zip_uri,
                    _test_build_workflow_diff("Workflow Remote Zip Edited", "asset-remote-zip.txt"),
                )

                roundtrip_json, json_files = import_remote_kit(json_uri)
                roundtrip_zip, zip_files = import_remote_kit(zip_uri)
            finally:
                server.shutdown()
                thread.join()

            assert edited_json.name == "Workflow Remote Json Edited"
            assert roundtrip_json.name == "Workflow Remote Json Edited"
            assert roundtrip_json.to_dict()["files"][0]["name"] == "asset-remote-json.txt"
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

            computed_diff = deletePiecesAndConnectionsInDesignDict(design, piece_guids, connection_guids)

            # Verify removed pieces
            computed_removed = computed_diff.get("pieces", {}).get("removed", [])
            expected_removed = expected_diff.get("pieces", {}).get("removed", [])
            assert len(computed_removed) == len(expected_removed), f"Removed pieces count mismatch: {len(computed_removed)} vs {len(expected_removed)}"
            for c, e in zip(computed_removed, expected_removed):
                assert c["guid"] == e["guid"], f"Removed piece guid mismatch: {c['guid']} vs {e['guid']}"

            # Verify updated (fixed) pieces
            computed_updated = computed_diff.get("pieces", {}).get("updated", [])
            expected_updated = expected_diff.get("pieces", {}).get("updated", [])
            assert len(computed_updated) == len(expected_updated), f"Updated pieces count mismatch: {len(computed_updated)} vs {len(expected_updated)}"
            computed_guids = sorted(u.get("piece", {}).get("guid", "") for u in computed_updated)
            expected_guids = sorted(u.get("piece", {}).get("guid", "") for u in expected_updated)
            assert computed_guids == expected_guids, f"Updated piece guids mismatch"
            for u in computed_updated:
                diff = u["diff"]
                plane = diff["plane"]
                center = diff["center"]
                assert plane["origin"] == {"x": 0, "y": 0, "z": 0}
                assert plane["xAxis"] == {"x": 1, "y": 0, "z": 0}
                assert plane["yAxis"] == {"x": 0, "y": 1, "z": 0}
                assert center == {"u": 0, "v": 0}

            # Verify removed connections
            computed_conn_removed = computed_diff.get("connections", {}).get("removed", [])
            expected_conn_removed = expected_diff.get("connections", {}).get("removed", [])
            assert len(computed_conn_removed) == len(expected_conn_removed), f"Removed connections count mismatch: {len(computed_conn_removed)} vs {len(expected_conn_removed)}"
            computed_conn_guids = sorted(r["guid"] for r in computed_conn_removed)
            expected_conn_guids = sorted(r["guid"] for r in expected_conn_removed)
            assert computed_conn_guids == expected_conn_guids, "Removed connection guids mismatch"


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
            models = [
                {
                    "guid": model["guid"],
                    "file": {"guid": model["fileGuid"]},
                    "tags": [{"guid": guid} for guid in model.get("tagGuids", [])],
                }
                for model in case.get("models", [])
            ]
            selected = _test_select_best_model_like_semio_ts(models, case.get("selectedTagGuids", []))
            selected_guid = selected.get("guid") if selected else None
            assert selected_guid == case.get("expectedGuid"), f"Case {case.get('name')} failed"


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
            filtered_type = next(
                (t for t in filtered.get("types", []) if t.get("guid") == expected_type.get("guid")),
                None,
            )
            assert filtered_type is not None
            assert len(filtered_type.get("models", [])) == len(expected_type.get("models", []))

        for piece in filtered_design.get("pieces", []):
            piece_kind_guid = piece.get("type", {}).get("guid")
            if piece_kind_guid:
                assert any(t.get("guid") == piece_kind_guid for t in filtered.get("types", []))

        for kind in filtered.get("types", []):
            assert len(kind.get("models", [])) <= 1
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
        assert len(types) > 0
        for t in types:
            assert fnmatch.fnmatch(t["name"].lower(), "capsule*")

    def test_glob_filters_types_by_name_exclude(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        total_types = len(kit_dict.get("types", []))
        filtered = KitData(kit_dict).filter_kit({"types": {"exclude": ["Capsule*"]}}).to_dict()
        types = filtered.get("types", [])
        assert len(types) < total_types
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
            assert abs(result - 2349.53) < TEST_TOLERANCE


class TestExportDesignModel:
    def test_export_glb_returns_valid_glb(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".glb")
        assert isinstance(result, bytes)
        assert len(result) > 0
        assert result[:4] == b"glTF"
        assert struct.unpack("<I", result[4:8])[0] == 2
        assert struct.unpack("<I", result[8:12])[0] == len(result)

    def test_export_gltf_returns_valid_json(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".gltf")
        assert isinstance(result, bytes)
        assert len(result) > 0
        parsed = json.loads(result.decode("utf-8"))
        assert "asset" in parsed
        assert "scenes" in parsed

    def test_export_invalid_format_raises(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        with pytest.raises(ValueError, match="Unsupported export format"):
            export_design_model(kit_dict, "Nakagin Capsule Tower", ".invalid")

    def test_export_scene_graph_report(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".gltf")
        parsed = json.loads(result.decode("utf-8"))
        assert "nodes" in parsed
        assert "scenes" in parsed
        REPORTS_EXPORT_DIR.mkdir(parents=True, exist_ok=True)
        (REPORTS_EXPORT_DIR / "py.gltf").write_bytes(result)

    def test_export_ifc_returns_valid_ifc(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
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
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc_text = result.decode("utf-8")
        assert "IFCBUILDINGELEMENTPROXYTYPE" in ifc_text
        assert "IFCBUILDINGELEMENTPROXY(" in ifc_text

    def test_export_ifc_contains_mesh_geometry(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc_text = result.decode("utf-8")
        assert "IFCSHAPEREPRESENTATION" in ifc_text

    def test_export_ifc_converts_gltf_mesh_axes_to_semio_axes(self):
        import ifcopenshell

        kit_dict = {
            "name": "Axis Test Kit",
            "guid": "axis-test-kit",
            "uri": "axis-test-kit",
            "types": [
                {
                    "guid": "axis-test-kind",
                    "name": "Axis Test Kind",
                    "variant": "",
                    "attributes": [],
                    "connectors": [],
                    "models": [
                        {
                            "guid": "axis-test-model",
                            "file": {"guid": "axis-test-file"},
                            "tags": [],
                        }
                    ],
                }
            ],
            "designs": [
                {
                    "guid": "axis-test-design",
                    "name": "Axis Test Design",
                    "pieces": [
                        {
                            "guid": "axis-test-piece",
                            "name": "Axis Test Piece",
                            "type": {"guid": "axis-test-kind"},
                        }
                    ],
                    "connections": [],
                }
            ],
            "files": [
                {
                    "guid": "axis-test-file",
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

        result = export_design_model(kit_dict, "Axis Test Design", ".ifc")
        ifc = ifcopenshell.file.from_string(result.decode("utf-8"))
        point_lists = ifc.by_type("IfcCartesianPointList3D")

        assert len(point_lists) == 1
        coordinates = [tuple(float(value) for value in row) for row in point_lists[0].CoordList]
        assert any(abs(x) < 1e-6 and abs(y) < 1e-6 and z > 0 for x, y, z in coordinates)
        assert any(abs(x) < 1e-6 and y < 0 and abs(z) < 1e-6 for x, y, z in coordinates)
        assert not any(abs(x) < 1e-6 and y > 0 and abs(z) < 1e-6 for x, y, z in coordinates)

    def test_export_ifc_contains_ports_and_connections(self):
        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
        ifc_text = result.decode("utf-8")
        assert "IFCDISTRIBUTIONPORT" in ifc_text
        assert "IFCRELCONNECTSPORTS" in ifc_text
        assert "IFCRELCONNECTSELEMENTS" in ifc_text

    def test_export_ifc_roundtrip_with_ifcopenshell(self):
        import ifcopenshell

        kit_dict = _test_load_json("metabolism.kit.semio.json")
        result = export_design_model(kit_dict, "Nakagin Capsule Tower", ".ifc")
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
        pieces = next(d for d in kit_dict.get("designs", []) if d.get("name") == "Nakagin Capsule Tower").get("pieces", [])
        assert len(occurrences) == len(pieces)
        ports = ifc.by_type("IfcDistributionPort")
        assert len(ports) > 0
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
        assert site_in_project
        site_children = [rel.RelatedObjects for rel in site.IsDecomposedBy]
        building_in_site = any(building in children for children in site_children)
        assert building_in_site
        building_children_list = [rel.RelatedObjects for rel in building.IsDecomposedBy]
        building_children = [child for children in building_children_list for child in children]
        for storey in storeys:
            assert storey in building_children, f"Storey {storey.Name} not aggregated under building"
        # Each storey should contain pieces
        for storey in storeys:
            contained = [rel.RelatedElements for rel in storey.ContainsElements] if storey.ContainsElements else []
            elements = [e for group in contained for e in group]
            assert len(elements) > 0, f"Storey {storey.Name} has no contained elements"
        # Verify types have representations (model geometry)
        type_products = ifc.by_type("IfcBuildingElementProxyType")
        types_with_rep = [t for t in type_products if t.RepresentationMaps]
        assert len(types_with_rep) > 0

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
            assert key in data, f"missing key {key}"
            assert data[key] == expected, f"mismatch for {key}: {data[key]!r} != {expected!r}"
        assert isinstance(insights, GeometricInsights)
        assert insights.bounding_box_min is not None
        assert insights.bounding_box_max is not None
        assert insights.dimension_x is not None and insights.dimension_x >= 0
        assert insights.dimension_y is not None and insights.dimension_y >= 0
        assert insights.dimension_z is not None and insights.dimension_z >= 0
        assert insights.characteristic_length is not None and insights.characteristic_length >= 0
        assert insights.total_surface_area is not None and insights.total_surface_area >= 0
        assert insights.vertex_count is not None and insights.vertex_count > 0
        assert insights.face_count is not None and insights.face_count > 0
        assert insights.centroid is not None
        assert insights.euler_characteristic is not None

    def test_nakagin_capsule_tower_from_bytes_gltf(self):
        model_path = os.path.join(os.path.dirname(__file__), TEST_ASSETS_DIR, "nakagin-capsule-tower.gltf")
        if not os.path.exists(model_path):
            pytest.skip("nakagin-capsule-tower.gltf not found")
        with open(model_path, "rb") as f:
            data = f.read()
        insights = get_geometric_insights_for_model(data)
        assert isinstance(insights, GeometricInsights)
        assert insights.face_count is not None and insights.face_count > 0


class TestTypeMeta:
    """Tests for TypeMeta deserialization from JSON."""

    def test_type_meta(self):
        data = _test_load_json("tambour.meta.type.semio.json")
        assert "guid" in data
        assert "name" in data
        assert data["name"] == "Tambour"
        meta: TypeMeta = data
        assert meta["guid"] == data["guid"]
        assert meta["name"] == "Tambour"
        assert "connectors" not in meta
        assert "models" not in meta
        assert "props" not in meta
        assert "attributes" not in meta


class TestTypeShallow:
    """Tests for TypeShallow deserialization from JSON."""

    def test_type_shallow(self):
        data = _test_load_json("tambour.shallow.type.semio.json")
        assert "guid" in data
        shallow: TypeShallow = data
        assert "connectors" in shallow
        assert isinstance(shallow["connectors"], list)
        assert len(shallow["connectors"]) > 0
        first_connector = shallow["connectors"][0]
        assert "guid" in first_connector
        assert "point" in first_connector
        assert "direction" in first_connector
        assert "attributes" not in first_connector
        assert "props" not in first_connector


class TestDesignMeta:
    """Tests for DesignMeta deserialization from JSON."""

    def test_design_meta(self):
        data = _test_load_json("nakagin-capsule-tower.meta.design.semio.json")
        assert "guid" in data
        assert "name" in data
        assert data["name"] == "Nakagin Capsule Tower"
        meta: DesignMeta = data
        assert meta["guid"] == data["guid"]
        assert "pieces" not in meta
        assert "connections" not in meta
        assert "layers" not in meta


class TestDesignShallow:
    """Tests for DesignShallow deserialization from JSON."""

    def test_design_shallow(self):
        data = _test_load_json("nakagin-capsule-tower.shallow.design.semio.json")
        assert "guid" in data
        assert "name" in data
        shallow: DesignShallow = data
        assert "pieces" in shallow
        assert isinstance(shallow["pieces"], list)
        assert len(shallow["pieces"]) > 0
        first_piece = shallow["pieces"][0]
        assert "guid" in first_piece
        assert "attributes" not in first_piece
        if "connections" in shallow:
            assert isinstance(shallow["connections"], list)
            if len(shallow["connections"]) > 0:
                first_conn = shallow["connections"][0]
                assert "guid" in first_conn
                assert "connected" in first_conn
                assert "connecting" in first_conn


class TestKitMeta:
    """Tests for KitMeta deserialization from JSON."""

    def test_kit_meta(self):
        data = _test_load_json("metabolism.meta.kit.semio.json")
        assert "guid" in data
        assert "name" in data
        assert data["name"] == "Metabolism"
        meta: KitMeta = data
        assert meta["guid"] == data["guid"]
        assert "types" not in meta
        assert "designs" not in meta
        assert "files" not in meta
        assert "folders" not in meta


class TestKitShallow:
    """Tests for KitShallow deserialization from JSON."""

    def test_kit_shallow(self):
        data = _test_load_json("metabolism.shallow.kit.semio.json")
        assert "guid" in data
        assert "name" not in data or isinstance(data.get("name"), str)
        shallow: KitShallow = data
        assert "types" in shallow
        assert isinstance(shallow["types"], list)
        assert len(shallow["types"]) > 0
        first_type = shallow["types"][0]
        assert "guid" in first_type
        assert "name" in first_type
        assert "connectors" not in first_type
        assert "models" not in first_type


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
                assert computed_meta[key] == expected_meta[key], f"KitMeta mismatch for key '{key}': {computed_meta[key]!r} != {expected_meta[key]!r}"

        computed_shallow = kitToShallow(kit_dict)
        assert computed_shallow["guid"] == expected_shallow["guid"]
        assert "types" in computed_shallow
        assert isinstance(computed_shallow["types"], list)

        expected_type_guids = {t["guid"] for t in expected_shallow.get("types", [])}
        computed_type_guids = {t["guid"] for t in computed_shallow.get("types", [])}
        assert expected_type_guids == computed_type_guids, "TypeMeta guids in shallow kit must match"

        for t in computed_shallow.get("types", []):
            assert "connectors" not in t, "TypeMeta in shallow kit must not have connectors"
            assert "models" not in t, "TypeMeta in shallow kit must not have models"

        expected_type_meta = _test_load_json("tambour.meta.type.semio.json")
        computed_type_meta = typeToMeta(next(t for t in kit_dict["types"] if t["guid"] == expected_type_meta["guid"]))
        for key in expected_type_meta:
            if key in computed_type_meta:
                assert computed_type_meta[key] == expected_type_meta[key], f"TypeMeta mismatch for key '{key}'"

        expected_design_meta = _test_load_json("nakagin-capsule-tower.meta.design.semio.json")
        computed_design_meta = designToMeta(next(d for d in kit_dict["designs"] if d["guid"] == expected_design_meta["guid"]))
        for key in expected_design_meta:
            if key in computed_design_meta:
                assert computed_design_meta[key] == expected_design_meta[key], f"DesignMeta mismatch for key '{key}'"


class TestKitKind:
    """Tests for the KitKind enum."""

    def test_all_kit_kinds_has_five_values(self):
        assert len(ALL_KIT_KINDS) == 5

    def test_kit_kind_values(self):
        assert KitKind.FILE.value == "file"
        assert KitKind.FOLDER.value == "folder"
        assert KitKind.ARCHIVE.value == "archive"
        assert KitKind.REMOTE.value == "remote"
        assert KitKind.TEMPORARY.value == "temporary"

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

    def test_kit_kind_temporary_in_memory(self):
        kit = Kit.parse({"name": "TempKit"})
        assert kit.name == "TempKit"
        assert kit.uri.startswith("memory://")


# endregion Test

# region Benchmark
# [👤semio📚py💻main🔖benchmark](repo://p/u/semio/b/l/py/f/main.py/s/Benchmark)
# Benchmarks for the semio py module.

BENCHMARK_ITERATIONS = 3


def _bench(name: str, func):
    start = time.perf_counter()
    for _ in range(BENCHMARK_ITERATIONS):
        func()
    end = time.perf_counter()
    duration = (end - start) / BENCHMARK_ITERATIONS
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
