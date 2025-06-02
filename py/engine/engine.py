#!/usr/bin/env python

# engine.py
# 2020-2025 Ueli Saluz

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


"""
engine.py
"""


# TODOs #

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

# Copilot #


## Dictionary ##


# Symbol,Code,Abbreviation,Name,Description
# 👥,Bs,Bas,Base,The shared base props for {{NAME}} models.
# 🧲,Cd,Cnd,Connected,The connected side of the piece of the connection.
# 🧲,Cg,Cng,Connecting,The connecting side of the piece of the connection.
# 🖇️,Co,Con,Connection,A connection between two pieces in a design.
# 🖇️,Co*,Cons,Connections,The optional connections of a design.
# ⌚,CA,CAt,Created At,The time when the {{NAME}} was created.
# 💬,Dc?,Dsc,Description,The optional human-readable description of the {{NAME}}.
# 📖,Df,Def,Definition,The optional definition [ text | uri ] of the quality.
# ✏️,Dg,Dgm,Diagram,The diagram of the design.
# 📁,Di?,Dir,Directory,The optional directory where to find the kit.
# 🏅,Dl,Dfl,Default,Whether it is the default representation of the type. There can be only one default representation per type.
# ➡️,Dr,Drn,Direction,The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned.
# 🏙️,Dn,Dsn,Design,A design is a collection of pieces that are connected.
# 🏙️,Dn*,Dsns,Designs,The optional designs of the kit.
# 🚌,Dt,DTO,Data Transfer Object, The Data Transfer Object (DTO) base of the {{NAME}}.
# 🪣,Em,Emp,Empty,Empty all props and children of the {{NAME}}.
# ▢,En,Ent,Entity,An entity is a collection of properties and children.
# 🔑,FK,FKy,Foreign Key, The foreign primary key of the parent {{PARENT_NAME}} of the {{NAME}} in the database.
# ↕️,Gp?,Gap,Gap,The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece.
# 🆔,GI,GID,Globally Unique Identifier,A Globally Unique Identifier (GUID) of the entity.
# 👪,Gr,Grp,Group,The group of the locator.
# 🏠,Hp?,Hmp,Homepage,The optional url of the homepage of the kit.
# 🪙,Ic?,Ico,Icon,The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. {{NAME}}.
# 🆔,Id,Id,Identifier,The local identifier of the {{NAME}} within the {{PARENT_NAME}}.
# 🆔,Id?,Id,Identifier,The optional local identifier of the {{NAME}} within the {{PARENT_NAME}}. No id means the default {{NAME}}.
# 🪪,Id,Id,Identifier,The props to identify the {{NAME}} within the parent {{PARENT_NAME}}.
# ↘️,In,Inp,Input,The input for a {{NAME}}.
# 🗃️,Kt,Kit,Kit,A kit is a collection of designs that use types.
# 🔍,Ld?,Lod,Level of Detail,The optional Level of Detail/Development/Design (LoD) of the representation. No lod means the default lod.
# 📛,Na,Nam,Name,The name of the {{NAME}}.
# ✉️,Mm,Mim,Mime,The Multipurpose Internet Mail Extensions (MIME) type of the content of the resource of the representation.
# ⌱,Og,Org,Origin,The origin of the plane.
# ↗️,Ou,Out,Output,The output for a {{NAME}}.
# 👪,Pa,Par,Parent,The parent of {{NAME}}.
# ⚒️,Pr,Prs,Parse,Parse the {{NAME}} from an input.
# 🔢,Pl,Plu,Plural,The plural of the singular of the entity name.
# ⭕,Pc,Pce,Piece,A piece is a 3d-instance of a type in a design.
# ⭕,Pc?,Pces,Pieces,The optional pieces of the design.
# 🔑,PK,PKy,Primary Key, The {{PROP_NAME}} is the primary key of the {{NAME}} in the database.
# 🔌,Po,Por,Port,A port is a connection point (with a direction) of a type.
# 🔌,Po+,Pors,Ports,The ports of the type.
# 🎫,Pp,Prp,Props,The props are all values of an entity without its children.
# ◳,Pn,Pln,Plane,A plane is an origin (point) and an orientation (x-axis and y-axis).
# ◳,Pn?,Pln,Plane,The optional plane of the piece. When pieces are connected only one piece can have a plane.
# ✖️,Pt,Pnt,Point,A 3d-point (xyz) of floating point numbers.
# ✖️,Pt,Pnt,Point,The connection point of the port that is attracted to another connection point.
# 📏,Ql,Qal,Quality,A quality is meta-data for decision making.
# 📏,Ql*,Qals,Qualities,The optional qualities of the {{NAME}}.
# 🍾,Rl,Rel,Release,The release of the engine that created this database.
# ☁️,Rm?,Rmt,Remote,The optional Unique Resource Locator (URL) where to fetch the kit remotely.
# 💾,Rp,Rep,Representation,A representation is a link to a resource that describes a type for a certain level of detail and tags.
# 🔄,Rt?,Rot,Rotation,The optional horizontal rotation in port direction between the connected and the connecting piece in degrees.
# 🧱,Sd,Sde,Side,A side of a piece in a connection.
# ↔️,Sf,Sft,Shift,The optional lateral shift (applied after the rotation, the turn and the tilt in the plane) between the connected and the connecting piece.
# 📌,SG?,SGr,Subgroup,The optional sub-group of the locator. No sub-group means true.
# 📺,SP,SPt,Diagram Point,A 2d-point (xy) of floats in the diagram. One unit is equal the width of a piece icon.
# ✅,Su,Suc,Success,{{NAME}} was successful.
# 🏷️,Tg*,Tags,Tags,The optional tags to group representations. No tags means default.
# ↗️,Tl?,Tlt,Tilt,The optional horizontal tilt perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.
# ▦,Tf,Trf,Transform,A 4x4 translation and rotation transformation matrix (no scaling or shearing).
# 🧩,Ty,Typ,Type,A type is a reusable element that can be connected with other types over ports.
# 🧩,Ty,Typ,Type,The type-related information of the side.
# 🧩,Ty*,Typs,Types,The optional types of the kit.
# 🔗,Ur,Url,Unique Resource Locator,The Unique Resource Locator (URL) to the resource of the representation.
# Ⓜ️,Ut,Unt,Unit,The length unit for all distance-related information of the {{PARENT_NAME}}.
# Ⓜ️,Ut,Unt,Unit,The optional unit of the value of the quality.
# 🔄,Up,Upd,Update,Update the props of the {{NAME}}. Optionally empty the {{NAME}} before.
# ➡️,Vc,Vec,Vector,A 3d-vector (xyz) of floating point numbers.
# 🛂,Vd,Vld,Validate,Check if the {{NAME}} is valid.
# 🏷️,Vl,Val,Value,The value of the tag.
# 🔢,Vl?,Val,Value,The optional value [ text | url ] of the quality. No value is equivalent to true for the name.
# 🔀,Vn?,Vnt,Variant,The optional variant of the {{NAME}}. No variant means the default variant.
# 🏁,X,X,X,The x-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.
# 🎚️,X,X,X,The x-coordinate of the point.
# ➡️,XA,XAx,XAxis,The x-axis of the plane.
# 🏁,Y,Y,Y,The y-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon.
# 🎚️,Y,Y,Y,The y-coordinate of the point.
# ➡️,YA,YAx,YAxis,The y-axis of the plane.
# 🏁,Z,Z,Z,The z-coordinate of the screen point.
# 🎚️,Z,Z,Z,The z-coordinate of the point.
# ➡️,ZA,ZAx,ZAxis,The z-axis of the plane.

# Imports #


import abc
import argparse
import base64
import datetime
import difflib
import enum
import functools
import inspect
import json
import logging
import multiprocessing
import os
import pathlib
import sqlite3
import time
import typing
import urllib
import zipfile
import io
import shutil
import stat
import signal
import sys

import dotenv
import fastapi
import fastapi.openapi
import graphene
import graphene_pydantic
import graphene_sqlalchemy
import lark
import loguru
import jinja2
import openai
import pydantic
import PySide6.QtCore
import PySide6.QtGui
import PySide6.QtWidgets
import requests
import sqlalchemy
import sqlalchemy.exc
import sqlmodel
import starlette
import starlette_graphene3
import uvicorn


# Type Hints #


RecursiveAnyList = typing.Any | list["RecursiveAnyList"]
"""🔁 A recursive any list is either any or a list where the items are recursive any list."""


# Constants #

NAME = "semio"
EMAIL = "mail@semio-tech.com"
RELEASE = "r25.03-1"
VERSION = "4.3.0-beta"
HOST = "127.0.0.1"
PORT = 2503
ADDRESS = "http://127.0.0.1:2503"
NAME_LENGTH_LIMIT = 64
ID_LENGTH_LIMIT = 128
URL_LENGTH_LIMIT = 1024
URI_LENGTH_LIMIT = 2048
QUALITIES_MAX = 64
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
MAX_REQUEST_BODY_SIZE = 50 * 1024 * 1024  # 50MB
dotenv.load_dotenv()
ENVS = {key: value for key, value in os.environ.items() if key.startswith("SEMIO_")}


# Utility #


def encode(value: str) -> str:
    """ᗒ Encode a string to be url safe."""
    return urllib.parse.quote(value, safe="")


def decode(value: str) -> str:
    """ᗕ Decode a url safe string."""
    return urllib.parse.unquote(value)


def encodeList(list: list[str]) -> str:
    return ",".join([encode(t) for t in list])


def decodeList(encodedList: str) -> list[str]:
    return [decode(t) for t in encodedList.split(",")]


def encodeRecursiveAnyList(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Encode a `RecursiveAnyList` to a url encoded string."""
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return encode(",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList]))


# I would just have to prove Applicative <=>. I miss you Haskell (。﹏。)
def id(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Turn any into `encoded(str(any))` or a recursive list into a flat comma [,] separated encoded list."""
    if not isinstance(recursiveAnyList, list):
        return encode(str(recursiveAnyList))
    return ",".join([encodeRecursiveAnyList(item) for item in recursiveAnyList])


def pretty(number: float) -> str:
    """🦋 Pretty print a floating point number."""
    if number == -0.0:
        number = 0.0
    return f"{number:.5f}".rstrip("0").rstrip(".")


def changeValues(c: dict | list, key: str, func: callable) -> None:
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


def changeKeys(c: dict | list, func: callable) -> None:
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


# Logging #


logger = loguru.logger


# Exceptions #


# All exceptions define __str__ as a message for the user.


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
        return f"🔜 Remote kits are not yet supported."


class NotFound(ClientError, abc.ABC):
    """🔍 The base for not found errors."""


class PortNotFound(NotFound):

    def __init__(self, parent: "Type", id: "PortId") -> None:
        self.parent = parent
        self.id = id

    def __str__(self):
        variant = f", {self.parent.variant}" if self.parent.variant else ""
        return f"🔍 Couldn't find the port ({self.id.id_}) inside the parent type ({self.parent.name}{variant})."


class TypeNotFound(NotFound):

    def __init__(self, id: "TypeId") -> None:
        self.id = id

    def __str__(self):
        variant = f", {self.id.variant}" if self.id.variant else ""
        return f"🔍 Couldn't find the type ({self.id.name}{variant})."


# class DesignNotFound(NotFound):

#     def __init__(self, name: str, variant: str = "") -> None:
#         self.name = name
#         self.variant = variant

#     def __str__(self):
#         variant = f", {self.variant}" if self.variant else ""
#         return f"🔍 Couldn't find the design ({self.name}{variant})."


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


class SpecificationError(ClientError, abc.ABC):
    """🚫 The base for all specification errors."""


class NoParentAssigned(SpecificationError, abc.ABC):
    """👪 The base for all no parent assigned errors."""


class NoRepresentationAssigned(NoParentAssigned):

    def __str__(self):
        return "👪 The entity has no parent representation assigned."


class NoTypeAssigned(NoParentAssigned):

    def __str__(self):
        return "👪 The entity has no parent type assigned."


class NoDesignAssigned(NoParentAssigned):

    def __str__(self):
        return "👪 The entity has no parent design assigned."


class NoTypeOrDesignAssigned(NoTypeAssigned, NoDesignAssigned):

    def __str__(self):
        return "👪 The entity has no parent type or design assigned."


class NoKitAssigned(NoParentAssigned):

    def __str__(self):
        return "👪 The entity has no parent kit assigned."


class NoRepresentationOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned(
    NoRepresentationAssigned,
    NoTypeAssigned,
    NoDesignAssigned,
    NoKitAssigned,
):

    def __str__(self):
        return "👪 The entity has no parent representation, port, type, piece, connection, design or kit assigned."


class AlreadyExists(SpecificationError, abc.ABC):
    """♊ The entity already exists in the store."""


class KitAlreadyExists(AlreadyExists, abc.ABC):

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self) -> str:
        return f"♊ A kit under uri ({self.uri}) already exists."


class TypeHasNotAllUsedPorts(SpecificationError):

    def __init__(self, missingPorts: set[str]) -> None:
        self.missingPorts = missingPorts

    def __str__(self) -> str:
        return f"🚫 A design is using some ports of the type. The new type is missing the following ports: {', '.join(self.missingPorts)}."


class Semio(sqlmodel.SQLModel, table=True):
    """ℹ️ Metadata about the database."""

    __tablename__ = "semio"

    release: str = sqlmodel.Field(
        default=RELEASE,
        primary_key=True,
        description="🍾 The current release of semio.",
    )
    """🍾 The current release of semio."""
    engine: str = sqlmodel.Field(
        default=VERSION,
        description="⚙️ The version of the engine that created this database.",
    )
    """⚙️ The version of the engine that created this database."""
    created: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="⌚ The time when the database was created.",
    )
    """⌚ The time when the database was created."""


# Models #


class Model(sqlmodel.SQLModel, abc.ABC):
    """⚪ The base for models."""

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


## Fields ##


# Composition over inheritance. Literally.


class Field(Model, abc.ABC):
    """🎫 The base for a field of a model."""


class RealField(Field, abc.ABC):
    """🧑 The base for a real field of a model. No lie."""


class MaskedField(Field, abc.ABC):
    """🎭 The base for a mask of a field of a model. WYSIWYG but don't expect it to be there."""


## Bases ##


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


## Entities ##


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
        return id(self.idMembers())

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
    # @abc.abstractmethod
    def empty(self) -> "Entity":
        """🪣 Empty all props and children of the entity."""

    # TODO: Automatic updating based on props.
    # @abc.abstractmethod
    def update(self, other: "Entity") -> "Entity":
        """🔄 Update the props of the entity."""


class Table(Model, abc.ABC):
    """▦ The base for tables. All resources that are stored in the database."""


class TableEntity(Entity, Table, abc.ABC):
    """▢ The base for table entities."""

    __tablename__: typing.ClassVar[str]
    """📛 The lowercase name of the table in the database."""


### Qualities ###


class QualityNameField(RealField, abc.ABC):
    """📏 The name of the quality."""

    name: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="📏 The name of the quality.",
    )
    """📏 The name of the quality."""


class QualityValueField(RealField, abc.ABC):
    """📏 The optional value [ text | url ] of the quality. No value is equivalent to true for the name."""

    value: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="📏 The optional value [ text | url ] of the quality. No value is equivalent to true for the name.",
    )
    """📏 The optional value [ text | url ] of the quality. No value is equivalent to true for the name."""


class QualityUnitField(RealField, abc.ABC):
    """📏 The optional unit of the value of the quality."""

    unit: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="📏 The optional unit of the value of the quality.",
    )
    """📏 The optional unit of the value of the quality."""


class QualityDefinitionField(RealField, abc.ABC):
    """📏 The optional definition [ text | uri ] of the quality."""

    definition: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="📏 The optional definition [ text | uri ] of the quality.",
    )
    """📏 The optional definition [ text | uri ] of the quality."""


class QualityId(QualityNameField, Id):
    """🪪 The props to identify the quality within the parent type."""


class QualityProps(
    QualityDefinitionField,
    QualityUnitField,
    QualityValueField,
    QualityNameField,
    Props,
):
    """📏 A quality is a named value with a unit and a definition."""


class QualityInput(
    QualityDefinitionField, QualityUnitField, QualityValueField, QualityNameField, Input
):
    """📏 A quality is a named value with a unit and a definition."""


class QualityContext(QualityUnitField, QualityValueField, QualityNameField, Context):
    """📏 A quality is a named value with a unit and a definition."""


class QualityOutput(
    QualityDefinitionField,
    QualityUnitField,
    QualityValueField,
    QualityNameField,
    Output,
):
    """📏 A quality is a named value with a unit and a definition."""


class Quality(
    QualityDefinitionField,
    QualityUnitField,
    QualityValueField,
    QualityNameField,
    TableEntity,
    table=True,
):
    """📏 A quality is a named value with a unit and a definition."""

    PLURAL = "qualities"
    __tablename__ = "quality"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the quality in the database."""
    representationPk: typing.Optional[int] = sqlmodel.Field(
        # alias="representationId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "representation_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("representation.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent representation of the quality in the database."""
    representation: typing.Optional["Representation"] = sqlmodel.Relationship(
        back_populates="qualities"
    )
    """👪 The parent representation of the quality."""
    portPk: typing.Optional[int] = sqlmodel.Field(
        # alias="portId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "port_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("port.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent port of the quality in the database."""
    port: typing.Optional["Port"] = sqlmodel.Relationship(back_populates="qualities")
    """👪 The parent port of the quality."""
    typePk: typing.Optional[int] = sqlmodel.Field(
        # alias="typeId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "type_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent type of the quality in the database."""
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="qualities")
    """👪 The parent type of the quality."""
    piecePk: typing.Optional[int] = sqlmodel.Field(
        # alias="pieceId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "piece_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("piece.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent piece of the quality in the database."""
    piece: typing.Optional["Piece"] = sqlmodel.Relationship(back_populates="qualities")
    """👪 The parent piece of the quality."""
    connectionPk: typing.Optional[int] = sqlmodel.Field(
        # alias="connectionId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "connection_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("connection.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent connection of the quality in the database."""
    connection: typing.Optional["Connection"] = sqlmodel.Relationship(
        back_populates="qualities"
    )
    """👪 The parent connection of the quality."""
    designPk: typing.Optional[int] = sqlmodel.Field(
        # alias="designId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "design_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("design.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent design of the quality in the database."""
    design: typing.Optional["Design"] = sqlmodel.Relationship(
        back_populates="qualities"
    )
    kitPk: typing.Optional[int] = sqlmodel.Field(
        # alias="kitId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "kit_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("kit.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent kit of the quality in the database."""
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="qualities")
    """👪 The parent kit of the quality."""
    __tableargs__ = (
        sqlalchemy.CheckConstraint(
            """
        (
            (representation_id IS NOT NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NOT NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NOT NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NOT NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NOT NULL AND design_id IS NULL AND kit_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NOT NULL AND kit_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NOT NULL)
        )
        """,
            name="parent set",
        ),
        sqlalchemy.UniqueConstraint("name", "type_id", "design_id"),
    )

    def parent(
        self,
    ) -> typing.Union[
        "Representation", "Port", "Type", "Piece", "Connection", "Design", "Kit", None
    ]:
        """👪 The parent type or design of the quality or otherwise `NoRepresentationOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned` is raised."""
        if self.representation is not None:
            return self.representation
        if self.port is not None:
            return self.port
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
        raise NoRepresentationOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the quality within its parent type."""
        return self.name


### Tags


class TagNameField(RealField, abc.ABC):
    """📛 The name of the tag."""

    name: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="📛 The name of the tag.",
    )
    """📛 The name of the tag."""


class TagOrderField(RealField, abc.ABC):
    """🔢 The order of the tag."""

    order: int = sqlmodel.Field(
        default=0,
        description="🔢 The order of the tag.",
    )
    """🔢 The order of the tag."""


class Tag(TagOrderField, TagNameField, Table, table=True):
    """🏷️ A tag is a label to group representations."""

    __tablename__ = "tag"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the tag in the database."""
    representationPk: typing.Optional[int] = sqlmodel.Field(
        # alias="representationId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "representation_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("representation.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent representation of the tag in the database."""
    representation: typing.Optional["Representation"] = sqlmodel.Relationship(
        back_populates="tags_"
    )
    """👪 The parent type of the representation."""


### Representations


class RepresentationUrlField(RealField, abc.ABC):
    """🔗 The Unique Resource Locator (URL) to the resource of the representation."""

    url: str = sqlmodel.Field(
        max_length=URL_LENGTH_LIMIT,
        description="🔗 The Unique Resource Locator (URL) to the resource of the representation.",
    )
    """🔗 The Unique Resource Locator (URL) to the resource of the representation."""


class RepresentationDescriptionField(RealField, abc.ABC):
    """💬 The optional human-readable description of the representation."""

    description: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="💬 The optional human-readable description of the representation.",
    )
    """💬 The optional human-readable description of the representation."""


class RepresentationTagsField(MaskedField, abc.ABC):
    """🏷️ The optional tags to group representations. No tags means default."""

    tags: list[str] = sqlmodel.Field(
        default_factory=list,
        description="🏷️ The optional tags to group representations. No tags means default.",
    )
    """🏷️ The optional tags to group representations. No tags means default."""


class RepresentationId(RepresentationTagsField, Id):
    """💾 A representation is a link to a resource that describes a type for a certain level of detail and tags."""

    pass


class RepresentationProps(
    RepresentationTagsField,
    RepresentationDescriptionField,
    RepresentationUrlField,
    Props,
):
    """🎫 The props of a representation."""


class RepresentationInput(
    RepresentationTagsField,
    RepresentationDescriptionField,
    RepresentationUrlField,
    Input,
):
    """💾 A representation is a link to a resource that describes a type for a certain level of detail and tags."""

    qualities: list[QualityInput] = sqlmodel.Field(
        default_factory=list,
        description="📏 The qualities of the representation.",
    )
    """📏 The qualities of the representation."""


class RepresentationContext(
    RepresentationTagsField,
    RepresentationDescriptionField,
    Context,
):
    """💾 A representation is a link to a resource that describes a type for a certain level of detail and tags."""

    qualities: list[QualityContext] = sqlmodel.Field(
        default_factory=list,
        description="📏 The qualities of the representation.",
    )
    """📏 The qualities of the representation."""


class RepresentationOutput(
    RepresentationTagsField,
    RepresentationDescriptionField,
    RepresentationUrlField,
    Output,
):
    """💾 A representation is a link to a resource that describes a type for a certain level of detail and tags."""

    qualities: list[QualityOutput] = sqlmodel.Field(
        default_factory=list,
        description="📏 The qualities of the representation.",
    )
    """📏 The qualities of the representation."""


class Representation(
    RepresentationDescriptionField,
    RepresentationUrlField,
    TableEntity,
    table=True,
):
    """💾 A representation is a link to a resource that describes a type for a certain level of detail and tags."""

    PLURAL = "representations"
    __tablename__ = "representation"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the representation in the database."""
    tags_: list[Tag] = sqlmodel.Relationship(
        back_populates="representation", cascade_delete=True
    )
    """🧑 The real tags of the representation in the database."""
    qualities: list[Quality] = sqlmodel.Relationship(
        back_populates="representation", cascade_delete=True
    )
    """📏 The qualities of the type."""
    typePk: typing.Optional[int] = sqlmodel.Field(
        # alias="typeId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "type_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent type of the representation in the database."""
    type: typing.Optional["Type"] = sqlmodel.Relationship(
        back_populates="representations"
    )
    """👪 The parent type of the representation."""

    @property
    def tags(self: "Representation") -> list[str]:
        """↗️ Get the masked tags of the representation."""
        return sorted([tag.name for tag in self.tags_], key=lambda x: x.order)

    @tags.setter
    def tags(self: "Representation", tags: list[str]):
        """↘️ Set the masked tags of the representation."""
        self.tags_ = [Tag(name=tag, order=i) for i, tag in enumerate(tags)]

    def parent(self: "Representation") -> "Type":
        """👪 The parent type of the representation or otherwise `NoTypeAssigned` is raised."""
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(
        cls: "Representation",
        input: str | dict | RepresentationInput | typing.Any | None,
    ) -> "Representation":
        """⚒️ Parse the representation from an input."""
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
        props = RepresentationProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            entity.tags = obj["tags"]
        except KeyError:
            pass
        try:
            entity.qualities = [Quality.parse(quality) for quality in obj["qualities"]]
        except KeyError:
            pass
        return entity

    def dump(self) -> "RepresentationOutput":
        entity = {**RepresentationProps.model_validate(self).model_dump()}
        #  TODO: Fix bug with tags not being dumped correctly.
        # Probably some sqlmodel issue with transient objects that are never written to the database.
        # 'str' object has no attribute 'order'
        # entity["tags"] = self.tags
        entity["qualities"] = [q.dump() for q in self.qualities]
        return RepresentationOutput(**entity)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the representation within its parent type."""
        return [self.tags]


### Diagram Points ###


class DiagramPoint(Model):
    """📺 A 2d-point (xy) of floats in the diagram. One unit is equal the width of a piece icon."""

    x: float = sqlmodel.Field(
        description="🏁 The x-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon."
    )
    """🏁 The x-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon."""
    y: float = sqlmodel.Field(
        description="🏁 The y-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon."
    )
    """🏁 The y-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon."""

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}]"

    # def __init__(self, x: int = 0, y: int = 0):
    #     super().__init__(x=x, y=y)

    # def __len__(self):
    #     return 2

    # def __getitem__(self, key):
    #     if key == 0:
    #         return self.x
    #     elif key == 1:
    #         return self.y
    #     else:
    #         raise IndexError("Index out of range")

    # def __iter__(self):
    #     return iter((self.x, self.y))


class DiagramPointInput(DiagramPoint, Input):
    """📺 A 2d-point (xy) of integers in screen coordinate system."""


class DiagramPointContext(DiagramPoint, Context):
    """📺 A 2d-point (xy) of integers in screen coordinate system."""


class DiagramPointOutput(DiagramPoint, Output):
    """📺 A 2d-point (xy) of integers in screen coordinate system."""


class DiagramPointPrediction(DiagramPoint, Prediction):
    """📺 A 2d-point (xy) of integers in screen coordinate system."""


### Points ###


class Point(Model):
    """✖️ A 3d-point (xyz) of floating point numbers."""

    x: float = sqlmodel.Field(description="🎚️ The x-coordinate of the point.")
    """🎚️ The x-coordinate of the point."""
    y: float = sqlmodel.Field(description="🎚️ The y-coordinate of the point.")
    """🎚️ The y-coordinate of the point."""
    z: float = sqlmodel.Field(description="🎚️ The z-coordinate of the point.")
    """🎚️ The z-coordinate of the point."""

    # def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
    #     super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    # def __len__(self):
    #     return 3

    # def __getitem__(self, key):
    #     if key == 0:
    #         return self.x
    #     elif key == 1:
    #         return self.y
    #     elif key == 2:
    #         return self.z
    #     else:
    #         raise IndexError("Index out of range")

    # def __iter__(self):
    #     return iter((self.x, self.y, self.z))

    # def isCloseTo(self, other: "Point", tol: float = TOLERANCE) -> bool:
    #     return (
    #         abs(self.x - other.x) < tol
    #         and abs(self.y - other.y) < tol
    #         and abs(self.z - other.z) < tol
    #     )

    # def transform(self, transform: "Transform") -> "Point":
    #     return Transform.transformPoint(transform, self)

    # def toVector(self) -> "Vector":
    #     return Vector(self.x, self.y, self.z)


class PointInput(Point, Input):
    """✖️ A 3d-point (xyz) of floating point numbers."""


class PointContext(Point, Context):
    """✖️ A 3d-point (xyz) of floating point numbers."""


class PointOutput(Point, Output):
    """✖️ A 3d-point (xyz) of floating point numbers."""


### Vectors ###


class Vector(Model):
    """➡️ A 3d-vector (xyz) of floating point numbers."""

    x: float = sqlmodel.Field(description="🎚️ The x-coordinate of the vector.")
    """🎚️ The x-coordinate of the vector."""
    y: float = sqlmodel.Field(description="🎚️ The y-coordinate of the vector.")
    """🎚️ The y-coordinate of the vector."""
    z: float = sqlmodel.Field(description="🎚️ The z-coordinate of the vector.")
    """🎚️ The z-coordinate of the vector."""

    # def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
    #     super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    # def __len__(self):
    #     return 3

    # def __getitem__(self, key):
    #     if key == 0:
    #         return self.x
    #     elif key == 1:
    #         return self.y
    #     elif key == 2:
    #         return self.z
    #     else:
    #         raise IndexError("Index out of range")

    # def __iter__(self):
    #     return iter((self.x, self.y, self.z))

    # def __add__(self, other):
    #     return Vector(self.x + other.x, self.y + other.y, self.z + other.z)

    # @property
    # def length(self) -> float:
    #     return (self.x**2 + self.y**2 + self.z**2) ** 0.5

    # def revert(self) -> "Vector":
    #     return Vector(-self.x, -self.y, -self.z)

    # def amplify(self, factor: float) -> "Vector":
    #     return Vector(self.x * factor, self.y * factor, self.z * factor)

    # def isCloseTo(self, other: "Vector", tol: float = TOLERANCE) -> bool:
    #     return (
    #         abs(self.x - other.x) < tol
    #         and abs(self.y - other.y) < tol
    #         and abs(self.z - other.z) < tol
    #     )

    # def normalize(self) -> "Vector":
    #     length = self.length
    #     return Vector(x=self.x / length, y=self.y / length, z=self.z / length)

    # def dot(self, other: "Vector") -> float:
    #     return numpy.dot(self, other)

    # def cross(self, other: "Vector") -> "Vector":
    #     return Vector(*numpy.cross(self, other))

    # def transform(self, transform: "Transform") -> "Vector":
    #     return Transform.transformVector(transform, self)

    # def toPoint(self) -> "Point":
    #     return Point(self.x, self.y, self.z)

    # def toTransform(self) -> "Transform":
    #     return Transform.fromTranslation(self)

    # @staticmethod
    # def X() -> "Vector":
    #     return Vector(x=1)

    # @staticmethod
    # def Y() -> "Vector":
    #     return Vector(y=1)

    # @staticmethod
    # def Z() -> "Vector":
    #     return Vector(z=1)


class VectorInput(Vector, Input):
    """➡️ A 3d-vector (xyz) of floating point numbers."""


class VectorContext(Vector, Context):
    """➡️ A 3d-vector (xyz) of floating point numbers."""


class VectorOutput(Vector, Output):
    """➡️ A 3d-vector (xyz) of floating point numbers."""


### Planes ### TODO


class PlaneOriginField(MaskedField, abc.ABC):
    """⌱ The origin of the plane."""

    origin: Point = sqlmodel.Field(description="⌱ The origin of the plane.")
    """⌱ The origin of the plane."""


class PlaneXAxisField(MaskedField, abc.ABC):
    """➡️ The x-axis of the plane."""

    xAxis: Vector = sqlmodel.Field(description="➡️ The x-axis of the plane.")
    """➡️ The x-axis of the plane."""


class PlaneYAxisField(MaskedField, abc.ABC):
    """➡️ The y-axis of the plane."""

    yAxis: Vector = sqlmodel.Field(description="➡️ The y-axis of the plane.")
    """➡️ The y-axis of the plane."""


class PlaneInput(Input):
    """◳ A plane is an origin (point) and an orientation (x-axis and y-axis)."""

    origin: PointInput = sqlmodel.Field(description="⌱ The origin of the plane.")
    """⌱ The origin of the plane."""
    xAxis: VectorInput = sqlmodel.Field(description="➡️ The x-axis of the plane.")
    """➡️ The x-axis of the plane."""
    yAxis: VectorInput = sqlmodel.Field(description="➡️ The y-axis of the plane.")
    """➡️ The y-axis of the plane."""


class PlaneContext(Context):
    """◳ A plane is an origin (point) and an orientation (x-axis and y-axis)."""

    origin: PointContext = sqlmodel.Field(description="⌱ The origin of the plane.")
    """⌱ The origin of the plane."""
    xAxis: VectorContext = sqlmodel.Field(description="➡️ The x-axis of the plane.")
    """➡️ The x-axis of the plane."""
    yAxis: VectorContext = sqlmodel.Field(description="➡️ The y-axis of the plane.")
    """➡️ The y-axis of the plane."""


class PlaneOutput(PlaneYAxisField, PlaneXAxisField, PlaneOriginField, Output):
    """◳ A plane is an origin (point) and an orientation (x-axis and y-axis)."""


class Plane(Table, table=True):
    """◳ A plane is an origin (point) and an orientation (x-axis and y-axis)."""

    __tablename__ = "plane"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the plane in the database."""
    originX: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "origin_x",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The x-coordinate of the origin point of the plane."""
    originY: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "origin_y",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The y-coordinate of the origin point of the plane."""
    originZ: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "origin_z",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The z-coordinate of the origin point of the plane."""
    xAxisX: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "x_axis_x",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The x-coordinate of the x-axis vector of the plane."""
    xAxisY: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "x_axis_y",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The y-coordinate of the x-axis vector of the plane."""
    xAxisZ: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "x_axis_z",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The z-coordinate of the x-axis vector of the plane."""
    yAxisX: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "y_axis_x",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The x-coordinate of the y-axis vector of the plane."""
    yAxisY: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "y_axis_y",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The y-coordinate of the y-axis vector of the plane."""
    yAxisZ: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "y_axis_z",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    piece: typing.Optional["Piece"] = sqlmodel.Relationship(back_populates="plane")
    """👪 The parent piece of the plane."""

    # def __init__(
    #     self, origin: Point = None, xAxis: Vector = None, yAxis: Vector = None
    # ):
    #     if origin is None:
    #         origin = Point()
    #     if xAxis is None and yAxis is None:
    #         xAxis = Vector.X()
    #         yAxis = Vector.Y()
    #     if xAxis is None:
    #         xAxis = Vector()
    #     if yAxis is None:
    #         yAxis = Vector()
    #     if abs(xAxis.length - 1) > TOLERANCE:
    #         raise ValidationError("The x-axis must be normalized.")
    #     if abs(yAxis.length - 1) > TOLERANCE:
    #         raise ValidationError("The y-axis must be normalized.")
    #     if abs(xAxis.dot(yAxis)) > TOLERANCE:
    #         raise ValidationError("The x-axis and y-axis must be orthogonal.")
    #     super().__init__(origin=origin, xAxis=xAxis, yAxis=yAxis)

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

    # def isCloseTo(self, other: "Plane", tol: float = TOLERANCE) -> bool:
    #     return (
    #         self.origin.isCloseTo(other.origin, tol)
    #         and self.xAxis.isCloseTo(other.xAxis, tol)
    #         and self.yAxis.isCloseTo(other.yAxis, tol)
    #     )

    # def transform(self, transform: "Transform") -> "Plane":
    #     return Transform.transformPlane(transform, self)

    # def toTransform(self) -> "Transform":
    #     return Transform.fromPlane(self)

    # @staticmethod
    # def XY() -> "Plane":
    #     return Plane(
    #         origin=Point(),
    #         xAxis=Vector.X(),
    #         yAxis=Vector.Y(),
    #     )

    # @staticmethod
    # def fromYAxis(yAxis: Vector, theta: float = 0.0, origin: Point = None) -> "Plane":
    #     if abs(yAxis.length - 1) > TOLERANCE:
    #         raise SpecificationError("The yAxis must be normalized.")
    #     if origin is None:
    #         origin = Point()
    #     orientation = Transform.fromDirections(Vector.Y(), yAxis)
    #     rotation = Transform.fromAngle(yAxis, theta)
    #     xAxis = Vector.X().transform(rotation.after(orientation))
    #     return Plane(origin=origin, xAxis=xAxis, yAxis=yAxis)

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(
        cls: "Plane", input: str | dict | PlaneInput | typing.Any | None
    ) -> "Plane":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
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


# ### Rotations ### TODO


# class Rotation(Model):
#     """🔄 A rotation is an axis and an angle."""

#     axis: Vector
#     angle: float

#     def __init__(self, axis: Vector, angle: float):
#         super().__init__(axis=axis, angle=angle)

#     def toTransform(self) -> "Transform":
#         return Transform.fromRotation(self)


# ### Transforms ### TODO


# class Transform(numpy.ndarray):
#     """▦ A 4x4 translation and rotation transformation matrix (no scaling or shearing)."""

#     def __new__(cls, input_array=None):
#         if input_array is None:
#             input_array = numpy.eye(4, dtype=float)
#         else:
#             input_array = numpy.asarray(input_array).astype(float)
#         obj = input_array.view(cls)
#         return obj

#     def __array_finalize__(self, obj):
#         if obj is None:
#             return

#     def __str__(self) -> str:
#         rounded_self = self.round()
#         return f"Transform(Rotation={rounded_self.rotation}, Translation={rounded_self.translation})"

#     def __repr__(self) -> str:
#         rounded_self = self.round()
#         return f"Transform(Rotation={rounded_self.rotation}, Translation={rounded_self.translation})"

#     @property
#     def rotation(self) -> Rotation | None:
#         """🔄 The rotation part of the transform."""
#         rotationMatrix = self[:3, :3]
#         axisAngle = axis_angle_from_matrix(rotationMatrix)
#         if axisAngle[3] == 0:
#             return None
#         return Rotation(
#             axis=Vector(float(axisAngle[0]), float(axisAngle[1]), float(axisAngle[2])),
#             angle=float(numpy.degrees(axisAngle[3])),
#         )

#     @property
#     def translation(self) -> Vector:
#         """➡️ The translation part of the transform."""
#         return Vector(*self[:3, 3])

#     # for pydantic
#     def dict(self) -> typing.Dict[str, typing.Union[Rotation, Vector]]:
#         return {
#             "rotation": self.rotation,
#             "translation": self.translation,
#         }

#     def after(self, before: "Transform") -> "Transform":
#         """✖️ Apply this transform after another transform.

#         Args:
#             before (Transform): Transform to apply before this transform.

#         Returns:
#             Transform: New transform.
#         """
#         return Transform(concat(before, self))

#     def invert(self) -> "Transform":
#         return Transform(invert_transform(self))

#     def transformPoint(self, point: Point) -> Point:
#         transformedPoint = transform(self, vector_to_point(point))
#         return Point(*transformedPoint[:3])

#     def transformVector(self, vector: Vector) -> Vector:
#         transformedVector = transform(self, vector_to_direction(vector))
#         return Vector(*transformedVector[:3])

#     def transformPlane(self, plane: Plane) -> Plane:
#         planeTransform = Transform.fromPlane(plane)
#         planeTransformed = planeTransform.after(self)
#         return Transform.toPlane(planeTransformed)

#     def transform(
#         self, geometry: typing.Union[Point, Vector, Plane]
#     ) -> typing.Union[Point, Vector, Plane]:
#         if isinstance(geometry, Point):
#             return self.transformPoint(geometry)
#         elif isinstance(geometry, Vector):
#             return self.transformVector(geometry)
#         elif isinstance(geometry, Plane):
#             return self.transformPlane(geometry)
#         else:
#             raise FeatureNotYetSupported()

#     def round(self, decimals: int = SIGNIFICANT_DIGITS) -> "Transform":
#         return Transform(super().round(decimals=decimals))

#     @staticmethod
#     def fromTranslation(vector: Vector) -> "Transform":
#         return Transform(
#             transform_from(
#                 [
#                     [1, 0, 0],
#                     [0, 1, 0],
#                     [0, 0, 1],
#                 ],
#                 vector,
#             )
#         )

#     @staticmethod
#     def fromRotation(rotation: Rotation) -> "Transform":
#         return Transform(
#             transform_from(
#                 matrix_from_axis_angle((*rotation.axis, radians(rotation.angle))),
#                 Vector(),
#             )
#         )

#     @staticmethod
#     def fromPlane(plane: Plane) -> "Transform":
#         # Assumes plane is normalized
#         return Transform(
#             transform_from(
#                 [
#                     [
#                         plane.xAxis.x,
#                         plane.yAxis.x,
#                         plane.zAxis.x,
#                     ],
#                     [
#                         plane.xAxis.y,
#                         plane.yAxis.y,
#                         plane.zAxis.y,
#                     ],
#                     [
#                         plane.xAxis.z,
#                         plane.yAxis.z,
#                         plane.zAxis.z,
#                     ],
#                 ],
#                 plane.origin,
#             )
#         )

#     @staticmethod
#     def fromAngle(axis: Vector, angle: float) -> "Transform":
#         return Transform(
#             transform_from(matrix_from_axis_angle((*axis, radians(angle))), Vector())
#         )

#     @staticmethod
#     def fromDirections(startDirection: Vector, endDirection: Vector) -> "Transform":
#         if startDirection.isCloseTo(endDirection):
#             return Transform()
#         axisAngle = axis_angle_from_two_directions(startDirection, endDirection)
#         return Transform(transform_from(matrix_from_axis_angle(axisAngle), Vector()))

#     def toPlane(self) -> Plane:
#         return Plane(
#             origin=Point(*self[:3, 3]),
#             xAxis=Vector(
#                 self[0, 0],
#                 self[1, 0],
#                 self[2, 0],
#             ),
#             yAxis=Vector(
#                 self[0, 1],
#                 self[1, 1],
#                 self[2, 1],
#             ),
#         )


### CompatibleFamily


class CompatibleFamilyNameField(RealField, abc.ABC):
    """📛 The name of the compatible port family."""

    name: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="📛 The name of the compatible port family.",
    )
    """📛 The name of the compatible port family."""


class CompatibleFamilyOrderField(RealField, abc.ABC):
    """🔢 The order of the compatible port family."""

    order: int = sqlmodel.Field(
        description="🔢 The order of the compatible port family.",
    )
    """🔢 The order of the compatible port family."""


class CompatibleFamily(
    CompatibleFamilyOrderField, CompatibleFamilyNameField, Table, table=True
):
    """✅ A compatible family is a label to group representations."""

    __tablename__ = "compatible_family"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the compatible port family in the database."""
    portPk: typing.Optional[int] = sqlmodel.Field(
        # alias="portId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "port_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("port.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent port of the compatible port family in the database."""
    port: typing.Optional["Port"] = sqlmodel.Relationship(
        back_populates="compatibleFamilies_"
    )
    """👪 The parent type of the compatible port family."""


### Ports ###


class PortIdField(MaskedField, abc.ABC):
    """🆔 The id of the port."""

    id_: str = sqlmodel.Field(
        default="",
        # alias="id", # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        max_length=ID_LENGTH_LIMIT,
        description="🆔 The id of the port.",
    )
    """🆔 The id of the port."""


class PortDescriptionField(RealField, abc.ABC):
    """💬 The optional human-readable description of the port."""

    description: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="💬 The optional human-readable description of the port.",
    )
    """💬 The optional human-readable description of the port."""


class PortMandatoryField(RealField, abc.ABC):
    """💯 Whether the port is mandatory. A mandatory port must be connected in a design."""

    mandatory: bool = sqlmodel.Field(
        default=False,
        description="💯 Whether the port is mandatory. A mandatory port must be connected in a design.",
    )
    """💯 Whether the port is mandatory. A mandatory port must be connected in a design."""


class PortFamilyField(RealField, abc.ABC):
    """👨‍👩‍👧‍👦 The optional family of the port. This allows to define explicit compatibility with other ports."""

    family: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="👨‍👩‍👧‍👦 The optional family of the port. This allows to define explicit compatibility with other ports.",
    )
    """👨‍👩‍👧‍👦 The optional family of the port. This allows to define explicit compatibility with other ports."""


class PortCompatibleFamiliesField(MaskedField, abc.ABC):
    """✅ The optional other compatible families of the port. An empty list means this port is compatible with all other ports."""

    compatibleFamilies: list[str] = sqlmodel.Field(
        default_factory=list,
        description="✅ The optional other compatible families of the port. An empty list means this port is compatible with all other ports.",
    )
    """✅ The optional other compatible families of the port. An empty list means this port is compatible with all other ports."""


class PortPointField(MaskedField, abc.ABC):
    """✖️ The connection point of the port that is attracted to another connection point."""

    point: Point = sqlmodel.Field(
        description="✖️ The connection point of the port that is attracted to another connection point."
    )
    """✖️ The connection point of the port that is attracted to another connection point."""


class PortDirectionField(MaskedField, abc.ABC):
    """➡️ The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned."""

    direction: Vector = sqlmodel.Field(
        description="➡️ The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned."
    )
    """➡️ The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned."""


class PortTField(RealField, abc.ABC):
    """💍 The parameter t [0,1[ where the port will be shown on the ring of a piece in the diagram. It starts at 12 o`clock and turns clockwise."""

    t: float = sqlmodel.Field(
        default=0.0,
        description="💍 The parameter t [0,1[ where the port will be shown on the ring of a piece in the diagram. It starts at 12 o`clock and turns clockwise.",
    )
    """💍 The parameter t [0,1[ where the port will be shown on the ring of a piece in the diagram. It starts at 12 o`clock and turns clockwise."""


class PortId(PortIdField, Id):
    """🪪 The props to identify the port within the parent type."""


class PortProps(
    PortTField,
    PortCompatibleFamiliesField,
    PortFamilyField,
    PortMandatoryField,
    PortDescriptionField,
    PortIdField,
    Props,
):
    """🎫 The props of a port."""


class PortInput(
    PortTField,
    PortCompatibleFamiliesField,
    PortFamilyField,
    PortMandatoryField,
    PortDescriptionField,
    PortIdField,
    Input,
):
    """🔌 A port is a connection point (with a direction) of a type."""

    point: PointInput = sqlmodel.Field(
        description="✖️ The connection point of the port that is attracted to another connection point."
    )
    """✖️ The connection point of the port that is attracted to another connection point."""
    direction: VectorInput = sqlmodel.Field(
        description="➡️ The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned."
    )
    """➡️ The direction of the port. When another piece connects the direction of the other port is flipped and then the pieces are aligned."""
    qualities: list[QualityInput] = sqlmodel.Field(
        default_factory=list,
        description="📏 The qualities of the port.",
    )
    """📏 The qualities of the port."""


class PortContext(
    PortTField,
    PortDirectionField,
    PortPointField,
    PortCompatibleFamiliesField,
    PortFamilyField,
    PortMandatoryField,
    PortDescriptionField,
    PortIdField,
    Context,
):
    """🔌 A port is a connection point (with a direction) of a type."""

    qualities: list[QualityContext] = sqlmodel.Field(
        default_factory=list,
        description="📏 The qualities of the port.",
    )
    """📏 The qualities of the port."""


class PortOutput(
    PortTField,
    PortDirectionField,
    PortPointField,
    PortCompatibleFamiliesField,
    PortFamilyField,
    PortMandatoryField,
    PortDescriptionField,
    PortIdField,
    Output,
):
    """🔌 A port is a connection point (with a direction) of a type."""

    qualities: list[QualityOutput] = sqlmodel.Field(
        default_factory=list,
        description="📏 The qualities of the port.",
    )
    """📏 The qualities of the port."""


class Port(
    PortTField,
    PortFamilyField,
    PortMandatoryField,
    PortDescriptionField,
    TableEntity,
    table=True,
):
    """🔌 A port is a connection point (with a direction) of a type."""

    PLURAL = "ports"
    __tablename__ = "port"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the port in the database."""
    # Can't use the name 'id' because of bug
    # https://github.com/graphql-python/graphene-sqlalchemy/issues/412
    id_: str = sqlmodel.Field(
        # alias="id",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "local_id",
            sqlalchemy.String(ID_LENGTH_LIMIT),
        ),
        default="",
    )
    """🆔 The id of the port within the type."""
    compatibleFamilies_: list[CompatibleFamily] = sqlmodel.Relationship(
        back_populates="port", cascade_delete=True
    )
    """✅ The compatible families of the port."""
    pointX: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "point_x",
            sqlalchemy.String(ID_LENGTH_LIMIT),
        ),
        exclude=True,
    )
    """🎚️ The x-coordinate of the connection point of the port."""
    pointY: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "point_y",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The y-coordinate of the connection point of the port."""
    pointZ: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "point_z",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The z-coordinate of the connection point of the port."""
    directionX: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "direction_x",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The x-coordinate of the direction of the port."""
    directionY: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "direction_y",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The y-coordinate of the direction of the port."""
    directionZ: float = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "direction_z",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The z-coordinate of the direction of the port."""
    qualities: list["Quality"] = sqlmodel.Relationship(
        back_populates="port", cascade_delete=True
    )
    """📏 The qualities of the port."""
    typePk: typing.Optional[int] = sqlmodel.Field(
        # alias="typeId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "type_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent type of the port in the database."""
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="ports")
    """👪 The parent type of the port."""
    connecteds: list["Connection"] = sqlmodel.Relationship(
        back_populates="connectedPort",
        sa_relationship_kwargs={"foreign_keys": "Connection.connectedPortPk"},
    )
    connectings: list["Connection"] = sqlmodel.Relationship(
        back_populates="connectingPort",
        sa_relationship_kwargs={"foreign_keys": "Connection.connectingPortPk"},
    )

    __table_args__ = (
        sqlalchemy.UniqueConstraint("local_id", "type_id", name="Unique local_id"),
    )

    @property
    def compatibleFamilies(self) -> list[str]:
        return sorted(
            [cf.name for cf in self.compatibleFamilies_], key=lambda cf: cf.order
        )

    @compatibleFamilies.setter
    def compatibleFamilies(self, compatibleFamilies: list[str]):
        self.compatibleFamilies_ = [
            CompatibleFamily(name=cf, order=i)
            for i, cf in enumerate(compatibleFamilies)
        ]

    @property
    def point(self) -> Point:
        """↗️ Get the masked point of the port."""
        return Point(x=self.pointX, y=self.pointY, z=self.pointZ)

    @point.setter
    def point(self, point: Point):
        """↘️ Set the masked point of the port."""
        self.pointX = point.x
        self.pointY = point.y
        self.pointZ = point.z

    @property
    def direction(self) -> Vector:
        """↗️ Get the masked direction of the port."""
        return Vector(x=self.directionX, y=self.directionY, z=self.directionZ)

    @direction.setter
    def direction(self, direction: Vector):
        """↘️ Set the masked direction of the port."""
        self.directionX = direction.x
        self.directionY = direction.y
        self.directionZ = direction.z

    @property
    def connections(self) -> list["Connection"]:
        """🔗 Get the connections of the port."""
        return self.connecteds + self.connectings

    def parent(self) -> "Type":
        """👪 The parent type of the port or otherwise `NoTypeAssigned` is raised."""
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Port", input: str | dict | PortInput | typing.Any | None) -> "Port":
        """🧪 Parse the input to a port."""
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
        props = PortProps.model_validate(obj)
        entity = cls(**props.model_dump())
        point = Point.parse(obj["point"])
        direction = Vector.parse(obj["direction"])
        entity.point = point
        entity.direction = direction
        try:
            entity.compatibleFamilies = obj["compatibleFamilies"]
        except KeyError:
            pass
        try:
            entity.qualities = [Quality.parse(q) for q in obj["qualities"]]
        except KeyError:
            pass
        return entity

    def dump(self) -> "PortOutput":
        entity = {**PortProps.model_validate(self).model_dump()}
        entity["point"] = self.point.dump()
        entity["direction"] = self.direction.dump()
        entity["compatibleFamilies"] = self.compatibleFamilies
        entity["qualities"] = [q.dump() for q in self.qualities]
        return PortOutput(**entity)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the port within its parent type."""
        return self.id_


### Authors ###


class AuthorNameField(RealField, abc.ABC):
    """📛 The name of the author."""

    name: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="📛 The name of the author.",
    )
    """📛 The name of the author."""


class AuthorEmailField(RealField, abc.ABC):
    """📧 The email of the author."""

    email: str = sqlmodel.Field(
        max_length=ID_LENGTH_LIMIT,
        description="📧 The email of the author.",
    )
    """📧 The email of the author."""


class AuthorRankField(RealField, abc.ABC):
    """🔢 The rank of the author."""

    rank: int = sqlmodel.Field(
        default=0,
        description="🔢 The rank of the author.",
    )
    """🔢 The rank of the author."""


class AuthorId(AuthorEmailField, Id):
    """🪪 The props to identify the author."""


class AuthorProps(AuthorEmailField, AuthorNameField, Props):
    """🎫 The props of an author."""


class AuthorInput(AuthorEmailField, AuthorNameField, Input):
    """👤 The input for an author."""


class AuthorOutput(AuthorEmailField, AuthorNameField, Output):
    """📑 The output of an author."""


class Author(
    AuthorRankField, AuthorEmailField, AuthorNameField, TableEntity, table=True
):
    """👤 The information about the author."""

    PLURAL = "authors"
    __tablename__ = "author"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the author in the database."""
    typePk: typing.Optional[int] = sqlmodel.Field(
        # alias="typeId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "type_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The optional foreign primary key of the parent type of the author in the database."""
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="authors_")
    """👪 The optional parent type of the author."""
    designPk: typing.Optional[int] = sqlmodel.Field(
        # alias="designId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "design_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("design.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The optional foreign primary key of the parent design of the author in the database."""
    design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="authors_")
    """👪 The optional parent design of the author."""

    __tableargs__ = (
        sqlalchemy.CheckConstraint(
            "typeId IS NOT NULL AND designId IS NULL OR typeId IS NULL AND designId IS NOT NULL",
            name="typeOrDesignSet",
        ),
        sqlalchemy.UniqueConstraint("email", "typeId", "designId"),
    )

    def parent(self) -> "Type":
        """👪 The parent type or design of the author or otherwise `NoTypeOrDesignAssigned` is raised."""
        if self.type is not None:
            return self.type
        if self.design is not None:
            return self.design
        raise NoTypeOrDesignAssigned()

    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the author within its parent type."""
        return self.email


### Locations ###


class Location(Model):
    """📍 A location on the earth surface (longitude, latitude)."""

    longitude: float = sqlmodel.Field(
        description="↔️ The longitude of the location in degrees.",
    )
    """↔️ The longitude of the location in degrees."""
    latitude: float = sqlmodel.Field(
        description="↕️ The latitude of the location in degrees."
    )
    """↕️ The latitude of the location in degrees."""


class LocationInput(Location, Input):
    """📍 The input for a location."""


class LocationOutput(Location, Output):
    """📍 The output of a location."""


class LocationContext(Location, Context):
    """📍 The context of a location."""


class LocationPrediction(Location, Prediction):
    """📍 The prediction of a location."""


### Types ###


class TypeNameField(RealField, abc.ABC):
    """📛 The name of the type."""

    name: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="📛 The name of the type.",
    )
    """📛 The name of the type."""


class TypeDescriptionField(RealField, abc.ABC):
    """💬 The optional human-readable description of the type."""

    description: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="💬 The optional human-readable description of the type.",
    )
    """💬 The optional human-readable description of the type."""


class TypeIconField(RealField, abc.ABC):
    """🪙 The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB."""

    icon: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🪙 The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB.",
    )
    """🪙 The optional icon [ emoji | logogram | url ] of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB."""


class TypeImageField(RealField, abc.ABC):
    """🖼️ The optional url to the image of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB."""

    image: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🖼️ The optional url to the image of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.",
    )
    """🖼️ The optional url to the image of the type. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB."""


class TypeVariantField(RealField, abc.ABC):
    """🔀 The optional variant of the type. No variant means the default variant."""

    variant: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="🔀 The optional variant of the type. No variant means the default variant.",
    )
    """🔀 The optional variant of the type. No variant means the default variant."""


class TypeStockField(RealField, abc.ABC):
    """📦 The number of items in stock."""

    stock: float = sqlmodel.Field(
        default=float("inf"), description="📦 The number of items in stock."
    )
    """📦 The number of items in stock."""


class TypeVirtualField(RealField, abc.ABC):
    """👻 Whether the type is virtual. A virtual type is not physically present but is used in conjunction with other virtual types to form a larger physical type."""

    virtual: bool = sqlmodel.Field(
        default=False,
        description="👻 Whether the type is virtual. A virtual type is not physically present but is used in conjunction with other virtual types to form a larger physical type.",
    )
    """👻 Whether the type is virtual. A virtual type is not physically present but is used in conjunction with other virtual types to form a larger physical type."""


class TypeUnitField(RealField, abc.ABC):
    """Ⓜ️ The length unit of the point and the direction of the ports of the type."""

    unit: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="Ⓜ️ The length unit of the point and the direction of the ports of the type.",
    )
    """Ⓜ️ The length unit of the point and the direction of the ports of the type."""


class TypeLocationField(MaskedField, abc.ABC):
    """📍 The optional location of the type."""

    location: typing.Optional[Location] = sqlmodel.Field(
        default=None,
        description="📍 The optional location of the type.",
    )
    """📍 The optional location of the type."""


class TypeCreatedField(RealField, abc.ABC):
    """🕒 The creation date of the type."""

    created: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The creation date of the type.",
    )
    """🕒 The creation date of the type."""


class TypeUpdatedField(RealField, abc.ABC):
    """🕒 The last update date of the type."""

    updated: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The last update date of the type.",
    )
    """🕒 The last update date of the type."""


class TypeId(TypeVariantField, TypeNameField, Id):
    """🪪 The props to identify the type."""


class TypeProps(
    TypeUnitField,
    TypeLocationField,
    TypeVirtualField,
    TypeStockField,
    TypeVariantField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Props,
):
    """🎫 The props of a type."""


class TypeInput(
    TypeUnitField,
    TypeLocationField,
    TypeVirtualField,
    TypeStockField,
    TypeVariantField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Input,
):
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    representations: list[RepresentationInput] = sqlmodel.Field(default_factory=list)
    ports: list[PortInput] = sqlmodel.Field(default_factory=list)
    authors: list[AuthorInput] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityInput] = sqlmodel.Field(default_factory=list)


class TypeOutput(
    TypeUpdatedField,
    TypeCreatedField,
    TypeUnitField,
    TypeLocationField,
    TypeVirtualField,
    TypeStockField,
    TypeVariantField,
    TypeImageField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Output,
):
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    representations: list[RepresentationOutput] = sqlmodel.Field(default_factory=list)
    ports: list[PortOutput] = sqlmodel.Field(default_factory=list)
    authors: list[AuthorOutput] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityOutput] = sqlmodel.Field(default_factory=list)


class TypeContext(
    TypeUnitField,
    TypeLocationField,
    TypeVirtualField,
    TypeStockField,
    TypeVariantField,
    TypeDescriptionField,
    TypeNameField,
    Context,
):
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    ports: list[PortContext] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityContext] = sqlmodel.Field(default_factory=list)


class Type(
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
    TableEntity,
    table=True,
):
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    PLURAL = "types"
    __tablename__ = "type"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the type in the database."""
    locationLongitude: typing.Optional[float] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "location_longitude",
            sqlalchemy.Float(),
        ),
        exclude=True,
        default=None,
    )
    """↔️ The longitude of the location in degrees."""
    locationLatitude: typing.Optional[float] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "location_latitude",
            sqlalchemy.Float(),
        ),
        exclude=True,
        default=None,
    )
    """↕️ The latitude of the location in degrees."""
    representations: list[Representation] = sqlmodel.Relationship(
        back_populates="type",
        cascade_delete=True,
    )
    """💾 The representations of the type."""
    ports: list[Port] = sqlmodel.Relationship(
        back_populates="type", cascade_delete=True
    )
    """🔌 The ports of the type."""
    authors_: list[Author] = sqlmodel.Relationship(
        back_populates="type", cascade_delete=True
    )
    """👤 The authors of the type."""
    qualities: list[Quality] = sqlmodel.Relationship(
        back_populates="type", cascade_delete=True
    )
    """📏 The qualities of the type."""
    kitPk: typing.Optional[int] = sqlmodel.Field(
        # alias="kitId", # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "kit_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("kit.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent kit of the type in the database."""
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="types")
    """👪 The parent kit of the type."""
    pieces: list["Piece"] = sqlmodel.Relationship(back_populates="type")

    __table_args__ = (
        sqlalchemy.UniqueConstraint(
            "name", "variant", "kit_id", name="Unique name and variant"
        ),
    )

    @property
    def location(self) -> Location:
        """📍 The location of the type."""
        return Location(
            longitude=self.locationLongitude,
            latitude=self.locationLatitude,
        )

    @location.setter
    def location(self, location: Location):
        """📍 Set the location of the type."""
        self.locationLongitude = location.longitude
        self.locationLatitude = location.latitude

    @property
    def authors(self) -> list[Author]:
        """👤 Get the authors of the type."""
        return sorted(self.authors_, key=lambda a: a.rank)

    @authors.setter
    def authors(self, authors: list[Author]):
        """👤 Set the authors of the type."""
        self.authors_ = authors
        for i, author in enumerate(self.authors_):
            author.rank = i

    def parent(self) -> "Kit":
        """👪 The parent kit of the type or otherwise `NoKitAssigned` is raised."""
        if self.kit is None:
            raise NoKitAssigned()
        return self.kit

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Type", input: str | dict | TypeInput | typing.Any | None) -> "Type":
        """🧪 Parse the input to a type."""
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
        props = TypeProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            representations = [Representation.parse(r) for r in obj["representations"]]
            entity.representations = representations
        except KeyError:
            pass
        try:
            ports = [Port.parse(p) for p in obj["ports"]]
            entity.ports = ports
        except KeyError:
            pass
        try:
            entity.qualities = [Quality.parse(q) for q in obj["qualities"]]
        except KeyError:
            pass
        try:
            authors = [Author.parse(a) for a in obj["authors"]]
            for i, author in enumerate(authors):
                author.rank = i
            entity.authors = authors
        except KeyError:
            pass
        return entity

    def dump(self) -> "TypeOutput":
        entity = {**TypeProps.model_validate(self).model_dump()}
        entity["representations"] = [r.dump() for r in self.representations]
        entity["ports"] = [p.dump() for p in self.ports]
        entity["qualities"] = [q.dump() for q in self.qualities]
        entity["authors"] = [a.dump() for a in self.authors]
        return TypeOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        """🪣 Empty the type."""
        props = TypeProps()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.types = []
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Type", empty: bool = False) -> "Type":
        """🔄 Update the props of the type. Optionally empty the type before."""
        if empty:
            self.empty()
        props = TypeProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the type within the parent kit."""
        return [self.name, self.variant]


### Pieces ###


class PieceIdField(MaskedField, abc.ABC):
    """🆔 The id of the piece."""

    id_: str = sqlmodel.Field(
        default="",
        # alias="id", # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        max_length=ID_LENGTH_LIMIT,
        description="🆔 The id of the piece.",
    )
    """🆔 The id of the piece."""


class PieceDescriptionField(RealField, abc.ABC):
    """💬 The optional human-readable description of the piece."""

    description: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="💬 The optional human-readable description of the piece.",
    )
    """💬 The optional human-readable description of the piece."""


class PieceTypeField(MaskedField, abc.ABC):
    """🧩 The type of the piece."""

    type: TypeId = sqlmodel.Field(
        description="🧩 The type of the piece.",
    )
    """🧩 The type of the piece."""


class PiecePlaneField(MaskedField, abc.ABC):
    """◳ The optional plane of the piece. When pieces are connected only one piece can have a plane."""

    plane: typing.Optional[Plane] = sqlmodel.Field(
        default=None,
        description="◳ The optional plane of the piece. When pieces are connected only one piece can have a plane.",
    )
    """◳ The optional plane of the piece. When pieces are connected only one piece can have a plane."""


class PieceCenterField(MaskedField, abc.ABC):
    """📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center."""

    center: typing.Optional[DiagramPoint] = sqlmodel.Field(
        default=None,
        description="📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.",
    )
    """📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center."""


class PieceId(PieceIdField, Id):
    """🪪 The props to identify the piece within the parent design."""


class PieceProps(
    PieceCenterField,
    PiecePlaneField,
    PieceTypeField,
    PieceDescriptionField,
    PieceIdField,
    Props,
):
    """🎫 The props of a piece."""


class PieceInput(PieceTypeField, PieceDescriptionField, PieceIdField, Input):
    """⭕ A piece is a 3d-instance of a type in a design."""

    plane: typing.Optional[PlaneInput] = sqlmodel.Field(
        default=None,
        description="◳ The optional plane of the piece. When pieces are connected only one piece can have a plane.",
    )
    """◳ The optional plane of the piece. When pieces are connected only one piece can have a plane."""
    center: typing.Optional[DiagramPointInput] = sqlmodel.Field(
        default=None,
        description="📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.",
    )
    """📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center."""
    qualities: list[QualityInput] = sqlmodel.Field(
        default_factory=list,
        description="📏 The qualities of the piece.",
    )
    """📏 The qualities of the piece."""


class PieceContext(PieceTypeField, PieceDescriptionField, PieceIdField, Context):
    """⭕ A piece is a 3d-instance of a type in a design."""

    plane: typing.Optional[PlaneContext] = sqlmodel.Field(
        default=None,
        description="◳ The optional plane of the piece. When pieces are connected only one piece can have a plane.",
    )
    """◳ The optional plane of the piece. When pieces are connected only one piece can have a plane."""
    center: typing.Optional[DiagramPointContext] = sqlmodel.Field(
        default=None,
        description="📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.",
    )
    """📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center."""
    qualities: list[QualityContext] = sqlmodel.Field(
        default_factory=list,
        description="📏 The qualities of the piece.",
    )
    """📏 The qualities of the piece."""


class PieceOutput(PieceTypeField, PieceDescriptionField, PieceIdField, Output):
    """⭕ A piece is a 3d-instance of a type in a design."""

    plane: typing.Optional[PlaneOutput] = sqlmodel.Field(
        default=None,
        description="◳ The optional plane of the piece. When pieces are connected only one piece can have a plane.",
    )
    """◳ The optional plane of the piece. When pieces are connected only one piece can have a plane."""
    center: typing.Optional[DiagramPointOutput] = sqlmodel.Field(
        default=None,
        description="📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.",
    )
    """📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center."""
    qualities: list[QualityOutput] = sqlmodel.Field(
        default_factory=list,
        description="📏 The qualities of the piece.",
    )
    """📏 The qualities of the piece."""


class PiecePrediction(PieceTypeField, PieceDescriptionField, PieceIdField, Prediction):
    """⭕ A piece is a 3d-instance of a type in a design."""

    # center: typing.Optional[DiagramPointPrediction] = sqlmodel.Field(
    #     default=None,
    #     description="📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center.",
    # )
    # """📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center."""


class Piece(PieceDescriptionField, TableEntity, table=True):
    """⭕ A piece is a 3d-instance of a type in a design."""

    PLURAL = "pieces"
    __tablename__ = "piece"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the piece in the database."""
    id_: str = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "local_id",
            sqlalchemy.String(ID_LENGTH_LIMIT),
        ),
        default="",
    )
    typePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "type_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign key of the type of the piece in the database."""
    type: Type = sqlmodel.Relationship(back_populates="pieces")
    """🆔 The id of the piece within the design."""
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
    """🔑 The optional foreign primary key of the plane of the piece in the database."""
    plane: typing.Optional[Plane] = sqlmodel.Relationship(back_populates="piece")
    """◳ The optional plane of the piece. When pieces are connected only one piece can have a plane."""
    centerX: typing.Optional[float] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "center_x",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The x-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon."""
    centerY: typing.Optional[float] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "center_y",
            sqlalchemy.Float(),
        ),
        exclude=True,
    )
    """🎚️ The y-coordinate of the icon of the piece in the diagram. One unit is equal the width of a piece icon."""
    qualities: list[Quality] = sqlmodel.Relationship(
        back_populates="piece", cascade_delete=True
    )
    """📏 The qualities of the type."""
    designPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "design_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("design.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent design of the piece in the database."""
    design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="pieces")
    """👪 The parent design of the piece."""
    connecteds: list["Connection"] = sqlmodel.Relationship(
        back_populates="connectedPiece",
        sa_relationship_kwargs={"foreign_keys": "Connection.connectedPiecePk"},
    )
    """🖇️ The connections where the piece is the connected to another piece."""
    connectings: list["Connection"] = sqlmodel.Relationship(
        back_populates="connectingPiece",
        sa_relationship_kwargs={"foreign_keys": "Connection.connectingPiecePk"},
    )
    """🖇️ The connections where the piece is the connecting from another piece."""

    __table_args__ = (sqlalchemy.UniqueConstraint("local_id", "design_id"),)

    @property
    def center(self) -> typing.Optional[DiagramPoint]:
        """↗️ Get the masked screen point of the piece."""
        if self.centerX is None or self.centerY is None:
            return None
        return DiagramPoint(x=self.centerX, y=self.centerY)

    @center.setter
    def center(self, center: typing.Optional[DiagramPoint]):
        """↘️ Set the masked screen point of the piece."""
        if center is None:
            self.centerX = None
            self.centerY = None
            return
        self.centerX = center.x
        self.centerY = center.y

    @property
    def connections(self) -> list["Connection"]:
        """🔗 Get the connections of the piece."""
        return self.connecteds + self.connectings

    def parent(self) -> "Design":
        """👪 The parent design of the piece or otherwise `NoParentAssigned` is raised."""
        if self.design is None:
            raise NoParentAssigned()
        return self.design

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(
        cls: "Piece",
        input: str | dict | PieceInput | typing.Any | None,
        types: dict[str, dict[str, Type]],
    ) -> "Piece":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
        entity = cls(id_=obj["id_"])
        type = TypeId.parse(obj["type"])
        try:
            entity.type = types[type.name][type.variant]
        except KeyError:
            raise TypeNotFound(type)
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
                center = DiagramPoint.parse(obj["center"])
                entity.center = center
        except KeyError:
            pass
        return entity

    def dump(self) -> "PieceOutput":
        entity = {**PieceProps.model_validate(self).model_dump()}
        entity["plane"] = self.plane.dump() if self.plane is not None else None
        entity["center"] = self.center.dump() if self.center is not None else None
        entity["qualities"] = [q.dump() for q in self.qualities]
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
        """🪪 The members that form the id of the piece within the parent design."""
        return self.id_


### Sides ###


class Side(Model):
    """🧱 A side of a piece in a connection."""

    piece: PieceId = sqlmodel.Field()
    port: PortId = sqlmodel.Field()

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Side", input: str | dict | typing.Any | None) -> "Side":
        """🧪 Parse the input to a side."""
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
        piece = PieceId.parse(obj["piece"])
        port = PortId.parse(obj["port"])
        return cls(piece=piece, port=port)


class SideInput(Side, Input):
    """🧱 A side of a piece in a connection."""


class SideContext(Side, Context):
    """🧱 A side of a piece in a connection."""


class SideOutput(Side, Output):
    """🧱 A side of a piece in a connection."""


class SidePrediction(Side, Prediction):
    """🧱 A side of a piece in a connection."""


### Connections ###


class ConnectionConnectedField(MaskedField, abc.ABC):
    """🧲 The connected side of the connection."""

    connected: Side = sqlmodel.Field(
        description="🧲 The connected side of the connection."
    )
    """🧲 The connected side of the connection."""


class ConnectionConnectingField(MaskedField, abc.ABC):
    """🧲 The connecting side of the connection."""

    connecting: Side = sqlmodel.Field(
        description="🧲 The connecting side of the connection."
    )
    """🧲 The connecting side of the connection."""


class ConnectionDescriptionField(RealField, abc.ABC):
    """💬 The optional human-readable description of the connection."""

    description: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="💬 The optional human-readable description of the connection.",
    )
    """💬 The optional human-readable description of the connection."""


class ConnectionGapField(RealField, abc.ABC):
    """↕️ The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece."""

    gap: float = sqlmodel.Field(
        default=0,
        description="↕️ The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece. ",
    )
    """↕️ The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece. """


class ConnectionShiftField(RealField, abc.ABC):
    """↔️ The optional lateral shift (applied after the rotation, the turn and the tilt in the plane) between the connected and the connecting piece.."""

    shift: float = sqlmodel.Field(
        default=0,
        description="↔️ The optional lateral shift (applied after the rotation, the turn and the tilt in the plane) between the connected and the connecting piece..",
    )
    """↔️ The optional lateral shift (applied after the rotation, the turn and the tilt in the plane) between the connected and the connecting piece.."""


class ConnectionRaiseField(MaskedField, abc.ABC):
    """🪜 The optional vertical raise in port direction between the connected and the connecting piece. Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."""

    raise_: float = sqlmodel.Field(
        alias="raise",
        default=0,
        description="🪜 The optional vertical raise in port direction between the connected and the connecting piece. Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result.",
    )
    """🪜 The optional vertical raise in port direction between the connected and the connecting piece. Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."""


class ConnectionRotationField(RealField, abc.ABC):
    """🔄 The optional horizontal rotation in port direction between the connected and the connecting piece in degrees."""

    rotation: float = sqlmodel.Field(
        ge=0,
        lt=360,
        default=0,
        description="🔄 The optional horizontal rotation in port direction between the connected and the connecting piece in degrees.",
    )
    """🔄 The optional horizontal rotation in port direction between the connected and the connecting piece in degrees."""


class ConnectionTurnField(RealField, abc.ABC):
    """🛞 The optional turn perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.  Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."""

    turn: float = sqlmodel.Field(
        ge=0,
        lt=360,
        default=0,
        description="🛞 The optional turn perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.  Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result.",
    )
    """🛞 The optional turn perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.  Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."""


class ConnectionTiltField(RealField, abc.ABC):
    """↗️ The optional horizontal tilt perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees."""

    tilt: float = sqlmodel.Field(
        ge=0,
        lt=360,
        default=0,
        description="↗️ The optional horizontal tilt perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.",
    )
    """↗️ The optional horizontal tilt perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees."""


# class ConnectionOrientationFirstField(RealField, abc.ABC):
#     """🥇 Wheather the orientation (rotation, turn, tilt) is applied before the translation (gap, shift, raise). By default the translation happens before the orientation."""

#     orientationFirst: bool = sqlmodel.Field(
#         default=False,
#         description="🥇 Wheather the orientation (rotation, turn, tilt) is applied before the translation (gap, shift, raise). By default the translation happens before the orientation.",
#     )
#     """🥇 Wheather the orientation (rotation, turn, tilt) is applied before the translation (gap, shift, raise). By default the translation happens before the orientation."""


class ConnectionXField(RealField, abc.ABC):
    """➡️ The optional offset in x direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon."""

    x: float = sqlmodel.Field(
        default=0,
        description="➡️ The optional offset in x direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.",
    )
    """➡️ The optional offset in x direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon."""


class ConnectionYField(RealField, abc.ABC):
    """⬆️ The optional offset in y direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon."""

    y: float = sqlmodel.Field(
        default=0,
        description="⬆️ The optional offset in y direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon.",
    )
    """⬆️ The optional offset in y direction between the icons of the child and the parent piece in the diagram. One unit is equal the width of a piece icon."""


class ConnectionId(ConnectionConnectedField, ConnectionConnectingField, Id):
    """🪪 The props to identify the connection."""


class ConnectionProps(
    ConnectionYField,
    ConnectionXField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRaiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    Props,
):
    """🎫 The props of a connection."""


class ConnectionInput(
    ConnectionYField,
    ConnectionXField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRaiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    Input,
):
    """🖇️ A bidirectional connection between two pieces of a design."""

    connected: SideInput = sqlmodel.Field(
        description="🧲 The connected side of the connection."
    )
    """🧲 The connected side of the connection."""
    connecting: SideInput = sqlmodel.Field(
        description="🧲 The connecting side of the connection."
    )
    """🧲 The connecting side of the connection."""


class ConnectionContext(
    ConnectionYField,
    ConnectionXField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRaiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    Context,
):
    """🖇️ A bidirectional connection between two pieces of a design."""

    connected: SideContext = sqlmodel.Field(
        description="🧲 The connected side of the connection."
    )
    """🧲 The connected side of the connection."""
    connecting: SideContext = sqlmodel.Field(
        description="🧲 The connecting side of the connection."
    )
    """🧲 The connecting side of the connection."""


class ConnectionOutput(
    ConnectionYField,
    ConnectionXField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRaiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    Output,
):
    """🖇️ A bidirectional connection between two pieces of a design."""

    connected: SideOutput = sqlmodel.Field(
        description="🧲 The connected side of the connection."
    )
    """🧲 The connected side of the connection."""
    connecting: SideOutput = sqlmodel.Field(
        description="🧲 The connecting side of the connection."
    )
    """🧲 The connecting side of the connection."""


class ConnectionPrediction(
    ConnectionYField,
    ConnectionXField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRaiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    Prediction,
):
    """🖇️ A bidirectional connection between two pieces of a design."""

    connected: SidePrediction = sqlmodel.Field(
        description="🧲 The connected side of the connection."
    )
    """🧲 The connected side of the connection."""
    connecting: SidePrediction = sqlmodel.Field(
        description="🧲 The connecting side of the connection."
    )
    """🧲 The connecting side of the connection."""


class Connection(
    ConnectionYField,
    ConnectionXField,
    ConnectionTiltField,
    ConnectionTurnField,
    ConnectionRotationField,
    ConnectionRaiseField,
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionDescriptionField,
    TableEntity,
    table=True,
):
    """🖇️ A bidirectional connection between two pieces of a design."""

    PLURAL = "connections"
    __tablename__ = "connection"

    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the connection in the database."""
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
    connectedPortPk: typing.Optional[int] = sqlmodel.Field(
        alias="connectedPortId",
        sa_column=sqlmodel.Column(
            "connected_port_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("port.id"),
        ),
        default=None,
        exclude=True,
    )
    connectedPort: Port = sqlmodel.Relationship(
        sa_relationship=sqlalchemy.orm.relationship(
            "Port",
            back_populates="connecteds",
            foreign_keys="[Connection.connectedPortPk]",
        )
    )
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
    connectingPortPk: typing.Optional[int] = sqlmodel.Field(
        alias="connectingPortId",
        sa_column=sqlmodel.Column(
            "connecting_port_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("port.id"),
        ),
        default=None,
        exclude=True,
    )
    connectingPort: Port = sqlmodel.Relationship(
        sa_relationship=sqlalchemy.orm.relationship(
            "Port",
            back_populates="connectings",
            foreign_keys="[Connection.connectingPortPk]",
        )
    )
    qualities: list[Quality] = sqlmodel.Relationship(
        back_populates="connection", cascade_delete=True
    )
    """📏 The qualities of the type."""
    designPk: typing.Optional[int] = sqlmodel.Field(
        alias="designId",
        sa_column=sqlmodel.Column(
            "design_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("design.id"),
        ),
        default=None,
        exclude=True,
    )
    design: "Design" = sqlmodel.Relationship(back_populates="connections")
    __table_args__ = (
        sqlalchemy.CheckConstraint(
            "connecting_piece_id != connected_piece_id",
            name="no reflexive connection",
        ),
    )

    @property
    def connected(self) -> Side:
        return Side(
            piece=self.connectedPiece,
            port=self.connectedPort,
        )

    @property
    def connecting(self) -> Side:
        return Side(
            piece=self.connectingPiece,
            port=self.connectingPort,
        )

    def parent(self) -> "Design":
        """👪 The parent design of the connection or otherwise `NoDesignAssigned` is raised."""
        if self.design is None:
            raise NoDesignAssigned()
        return self.design

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(
        cls: "Connection",
        input: str | dict | ConnectionInput | typing.Any | None,
        pieces: list[Piece],
    ) -> "Connection":
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
        piecesDict = {p.id_: p for p in pieces}
        connected = Side.parse(obj["connected"])
        connecting = Side.parse(obj["connecting"])
        connectedPiece = piecesDict[connected.piece.id_]
        connectedType = connectedPiece.type
        connectedPort = [p for p in connectedType.ports if p.id_ == connected.port.id_]
        if len(connectedPort) == 0:
            raise PortNotFound(connectedType, connected.port)
        else:
            connectedPort = connectedPort[0]
        connectingPiece = piecesDict[connecting.piece.id_]
        connectingType = connectingPiece.type
        connectingPort = [
            p for p in connectingType.ports if p.id_ == connecting.port.id_
        ]
        if len(connectingPort) == 0:
            raise PortNotFound(connectingType, connecting.port)
        else:
            connectingPort = connectingPort[0]
        entity = cls(
            connectedPiece=connectedPiece,
            connectedPort=connectedPort,
            connectingPiece=connectingPiece,
            connectingPort=connectingPort,
        )
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
            entity.raise_ = obj["raise"]
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
        entity["qualities"] = [q.dump() for q in self.qualities]
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
        """🪪 The members that form the id of the connection within the parent design."""
        return [
            self.connected.piece.id_,
            self.connected.port.id_,
            self.connecting.piece.id_,
            self.connecting.port.id_,
        ]


### Designs ###


class DesignNameField(RealField, abc.ABC):
    """📛 The name of the design."""

    name: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="📛 The name of the design.",
    )
    """📛 The name of the design."""


class DesignDescriptionField(RealField, abc.ABC):
    """💬 The optional human-readable description of the design."""

    description: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="💬 The optional human-readable description of the design.",
    )
    """💬 The optional human-readable description of the design."""


class DesignIconField(RealField, abc.ABC):
    """🪙 The optional icon [ emoji | logogram | url ] of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. The image must be at least 256x256 pixels and smaller than 1 MB."""

    icon: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🪙 The optional icon [ emoji | logogram | url ] of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. The image must be at least 256x256 pixels and smaller than 1 MB.",
    )
    """🪙 The optional icon [ emoji | logogram | url ] of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. The image must be at least 256x256 pixels and smaller than 1 MB."""


class DesignImageField(RealField, abc.ABC):
    """🖼️ The optional url to the image of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB."""

    image: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🖼️ The optional url to the image of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.",
    )
    """🖼️ The optional url to the image of the design. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB."""


class DesignVariantField(RealField, abc.ABC):
    """🔀 The optional variant of the design. No variant means the default variant."""

    variant: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="🔀 The optional variant of the design. No variant means the default variant.",
    )
    """🔀 The optional variant of the design. No variant means the default variant."""


class DesignViewField(RealField, abc.ABC):
    """🥽 The optional view of the design. No view means the default view."""

    view: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="🥽 The optional view of the design. No view means the default view.",
    )
    """🥽 The optional view of the design. No view means the default view."""


class DesignLocationField(MaskedField, abc.ABC):
    """📍 The optional location of the design."""

    location: typing.Optional[Location] = sqlmodel.Field(
        default=None,
        description="📍 The optional location of the design.",
    )
    """📍 The optional location of the design."""


class DesignUnitField(RealField, abc.ABC):
    """📏 The unit of the design."""

    unit: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="📏 The unit of the design.",
    )
    """📏 The unit of the design."""


class DesignCreatedField(RealField, abc.ABC):
    """🕒 The creation date of the design."""

    created: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The creation date of the design.",
    )
    """🕒 The creation date of the design."""


class DesignUpdatedField(RealField, abc.ABC):
    """🕒 The last update date of the design."""

    updated: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The last update date of the design.",
    )
    """🕒 The last update date of the design."""


class DesignId(DesignNameField, DesignVariantField, Id):
    """🪪 The props to identify the design."""


class DesignProps(
    DesignUnitField,
    DesignViewField,
    DesignLocationField,
    DesignVariantField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Props,
):
    """🎫 The props of a design."""


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
    """🏙️ A design is a collection of pieces that are connected."""

    location: typing.Optional[LocationInput] = sqlmodel.Field(default=None)
    pieces: list[PieceInput] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionInput] = sqlmodel.Field(default_factory=list)
    authors: list[AuthorInput] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityInput] = sqlmodel.Field(default_factory=list)


class DesignContext(
    DesignUnitField,
    DesignViewField,
    DesignVariantField,
    DesignDescriptionField,
    DesignNameField,
    Context,
):
    """🏙️ A design is a collection of pieces that are connected."""

    location: typing.Optional[LocationContext] = sqlmodel.Field(default=None)
    pieces: list[PieceContext] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionContext] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityContext] = sqlmodel.Field(default_factory=list)


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
    """🏙️ A design is a collection of pieces that are connected."""

    location: typing.Optional[LocationOutput] = sqlmodel.Field(default=None)
    pieces: list[PieceOutput] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionOutput] = sqlmodel.Field(default_factory=list)
    authors: list[AuthorOutput] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityOutput] = sqlmodel.Field(default_factory=list)


class DesignPrediction(
    DesignDescriptionField,
    Prediction,
):
    """🏙️ A design is a collection of pieces that are connected."""

    pieces: list[PiecePrediction] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionPrediction] = sqlmodel.Field(default_factory=list)


class Design(
    DesignUpdatedField,
    DesignCreatedField,
    DesignUnitField,
    DesignViewField,
    DesignVariantField,
    DesignImageField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    TableEntity,
    table=True,
):
    """🏙️ A design is a collection of pieces that are connected."""

    PLURAL = "designs"
    __tablename__ = "design"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    locationLongitude: typing.Optional[float] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "location_longitude",
            sqlalchemy.Float(),
        ),
        exclude=True,
        default=None,
    )
    """↔️ The longitude of the location in degrees."""
    locationLatitude: typing.Optional[float] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "location_latitude",
            sqlalchemy.Float(),
        ),
        exclude=True,
        default=None,
    )
    """↕️ The latitude of the location in degrees."""
    pieces: list[Piece] = sqlmodel.Relationship(
        back_populates="design", cascade_delete=True
    )
    connections: list[Connection] = sqlmodel.Relationship(
        back_populates="design", cascade_delete=True
    )
    authors_: list[Author] = sqlmodel.Relationship(
        back_populates="design", cascade_delete=True
    )
    qualities: list[Quality] = sqlmodel.Relationship(
        back_populates="design", cascade_delete=True
    )
    kitPk: typing.Optional[int] = sqlmodel.Field(
        alias="kitId",
        sa_column=sqlmodel.Column(
            "kit_id",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("kit.id"),
        ),
        default=None,
        exclude=True,
    )
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="designs")

    __table_args__ = (sqlalchemy.UniqueConstraint("name", "variant", "view", "kit_id"),)

    @property
    def location(self) -> Location:
        """📍 The location of the design."""
        return Location(
            longitude=self.locationLongitude,
            latitude=self.locationLatitude,
        )

    @location.setter
    def location(self, location: Location):
        """📍 Set the location of the design."""
        self.locationLongitude = location.longitude
        self.locationLatitude = location.latitude

    @property
    def authors(self) -> list[Author]:
        """👤 Get the authors of the design."""
        return sorted(self.authors_, key=lambda a: a.rank)

    @authors.setter
    def authors(self, authors: list[Author]):
        self.authors_ = authors
        for i, author in enumerate(authors):
            author.rank = i

    def parent(self) -> "Kit":
        """👪 The parent kit of the design or otherwise `NoKitAssigned` is raised."""
        if self.kit is None:
            raise NoKitAssigned()
        return self.kit

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(
        cls: "Design",
        input: str | dict | DesignInput | typing.Any | None,
        types: list[Type],
    ) -> "Design":
        """🧪 Parse the input to a design."""
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
        props = DesignProps.model_validate(obj)
        entity = cls(**props.model_dump())
        typesDict = {}
        for type in types:
            if type.name not in typesDict:
                typesDict[type.name] = {}
            if type.variant not in typesDict[type.name]:
                typesDict[type.name][type.variant] = {}
            typesDict[type.name][type.variant] = type
        try:
            pieces = [Piece.parse(p, typesDict) for p in obj["pieces"]]
            entity.pieces = pieces
        except KeyError:
            pass
        try:
            connections = [Connection.parse(c, pieces) for c in obj["connections"]]
            entity.connections = connections
        except KeyError:
            pass
        try:
            qualities = [Quality.parse(q) for q in obj["qualities"]]
            entity.qualities = qualities
        except KeyError:
            pass
        try:
            authors = [Author.parse(a) for a in obj["authors"]]
            entity.authors = authors
        except KeyError:
            pass
        return entity

    def dump(self) -> "DesignOutput":
        entity = {**DesignProps.model_validate(self).model_dump()}
        entity["pieces"] = [p.dump() for p in self.pieces]
        entity["connections"] = [c.dump() for c in self.connections]
        entity["qualities"] = [q.dump() for q in self.qualities]
        entity["authors"] = [a.dump() for a in self.authors]
        return DesignOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        """🪣 Empty the design."""
        props = DesignProps()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.designs = []
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Design", empty: bool = False) -> "Design":
        """🔄 Update the props of the design. Optionally empty the design before."""
        if empty:
            self.empty()
        props = DesignProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return [self.name, self.variant]


### Kits ###


class KitUriField(RealField, abc.ABC):
    """🆔 The uri of the kit."""

    uri: str = sqlmodel.Field(
        max_length=URI_LENGTH_LIMIT,
        description="🆔 The uri of the kit.",
    )
    """🆔 The uri of the kit."""


class KitNameField(RealField, abc.ABC):
    """📛 The name of the kit."""

    name: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="📛 The name of the kit.",
    )
    """📛 The name of the kit."""


class KitDescriptionField(RealField, abc.ABC):
    """💬 The optional human-readable description of the kit."""

    description: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="💬 The optional human-readable description of the kit.",
    )
    """💬 The optional human-readable description of the kit."""


class KitIconField(RealField, abc.ABC):
    """🪙 The optional icon [ emoji | logogram | url ] of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. kit."""

    icon: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🪙 The optional icon [ emoji | logogram | url ] of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. kit.",
    )
    """🪙 The optional icon [ emoji | logogram | url ] of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 256x256 pixels and smaller than 1 MB. kit."""


class KitImageField(RealField, abc.ABC):
    """🖼️ The optional url to the image of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB."""

    image: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🖼️ The optional url to the image of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB.",
    )
    """🖼️ The optional url to the image of the kit. The url must point to a quadratic image [ png | jpg | svg ] which will be cropped by a circle. The image must be at least 720x720 pixels and smaller than 5 MB."""


class KitPreviewField(RealField, abc.ABC):
    """🔮 The optional url of the preview image of the kit. The url must point to a landscape image [ png | jpg | svg ] which will be cropped by a 2x1 rectangle. The image must be at least 1920x960 pixels and smaller than 15 MB."""

    preview: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🔮 The optional url of the preview image of the kit. The url must point to a landscape image [ png | jpg | svg ] which will be cropped by a 2x1 rectangle. The image must be at least 1920x960 pixels and smaller than 15 MB.",
    )
    """🔮 The optional url of the preview image of the kit. The url must point to a landscape image [ png | jpg | svg ] which will be cropped by a 2x1 rectangle. The image must be at least 1920x960 pixels and smaller than 15 MB."""


class KitVersionField(RealField, abc.ABC):
    """🔀 The optional version of the kit. No version means the latest version."""

    version: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="🔀 The optional version of the kit. No version means the latest version.",
    )
    """🔀 The optional version of the kit. No version means the latest version."""


class KitRemoteField(RealField, abc.ABC):
    """☁️ The optional Unique Resource Locator (URL) where to fetch the kit remotely."""

    remote: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="☁️ The optional Unique Resource Locator (URL) where to fetch the kit remotely.",
    )
    """☁️ The optional Unique Resource Locator (URL) where to fetch the kit remotely."""


class KitHomepage(RealField, abc.ABC):
    """🏠 The optional url of the homepage of the kit."""

    homepage: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🏠 The optional url of the homepage of the kit.",
    )
    """🏠 The optional url of the homepage of the kit."""


class KitLicenseField(RealField, abc.ABC):
    """⚖️ The optional license [ spdx id | url ] of the kit."""

    license: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="⚖️ The optional license [ spdx id | url ] of the kit.",
    )
    """⚖️ The optional license [ spdx id | url ] of the kit."""


class KitCreatedField(RealField, abc.ABC):
    """🕒 The creation date of the kit."""

    created: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The creation date of the kit.",
    )
    """🕒 The creation date of the kit."""


class KitUpdatedField(RealField, abc.ABC):
    """🕒 The last update date of the kit."""

    updated: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The last update date of the kit.",
    )
    """🕒 The last update date of the kit."""


class KitId(KitUriField, Id):
    """🪪 The props to identify the kit."""


class KitProps(
    KitLicenseField,
    KitHomepage,
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
    """🎫 The props of a kit."""


class KitInput(
    KitLicenseField,
    KitHomepage,
    KitRemoteField,
    KitVersionField,
    KitPreviewField,
    KitImageField,
    KitIconField,
    KitDescriptionField,
    KitNameField,
    Input,
):
    """↘️ The input for a kit."""

    types: list[TypeInput] = sqlmodel.Field(
        default_factory=list, description="🧩 The types of the kit."
    )
    """🧩 The types of the kit."""
    designs: list[DesignInput] = sqlmodel.Field(
        default_factory=list, description="🏙️ The designs of the kit."
    )
    """🏙️ The designs of the kit."""
    qualities: list[QualityInput] = sqlmodel.Field(
        default_factory=list, description="📏 The qualities of the kit."
    )
    """📏 The qualities of the kit."""


class KitContext(
    KitDescriptionField,
    KitNameField,
    Context,
):
    """🗃️ A kit is a collection of types and designs."""

    types: list[TypeContext] = sqlmodel.Field(
        default_factory=list, description="🧩 The types of the kit."
    )
    """🧩 The types of the kit."""
    designs: list[DesignContext] = sqlmodel.Field(
        default_factory=list, description="🏙️ The designs of the kit."
    )
    """🏙️ The designs of the kit."""
    qualities: list[QualityContext] = sqlmodel.Field(
        default_factory=list, description="📏 The qualities of the kit."
    )
    """📏 The qualities of the kit."""


class KitOutput(
    KitUpdatedField,
    KitCreatedField,
    KitLicenseField,
    KitHomepage,
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
    """🗃️ A kit is a collection of types and designs."""

    types: list[TypeOutput] = sqlmodel.Field(
        default_factory=list, description="🧩 The types of the kit."
    )
    """🧩 The types of the kit."""
    designs: list[DesignOutput] = sqlmodel.Field(
        default_factory=list, description="🏙️ The designs of the kit."
    )
    """🏙️ The designs of the kit."""
    qualities: list[QualityOutput] = sqlmodel.Field(
        default_factory=list, description="📏 The qualities of the kit."
    )
    """📏 The qualities of the kit."""


class Kit(
    KitUpdatedField,
    KitCreatedField,
    KitLicenseField,
    KitHomepage,
    KitRemoteField,
    KitVersionField,
    KitPreviewField,
    KitImageField,
    KitIconField,
    KitDescriptionField,
    KitNameField,
    KitUriField,
    TableEntity,
    table=True,
):
    """🗃️ A kit is a collection of types and designs."""

    PLURAL = "kits"
    __tablename__ = "kit"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the kit in the database."""

    types: list[Type] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    """🧩 The types of the kit."""
    designs: list[Design] = sqlmodel.Relationship(
        back_populates="kit", cascade_delete=True
    )
    """🏙️ The designs of the kit."""
    qualities: list[Quality] = sqlmodel.Relationship(
        back_populates="kit", cascade_delete=True
    )
    """📏 The qualities of the kit."""

    __table_args__ = (sqlalchemy.UniqueConstraint("uri"),)

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Kit", input: str | dict | KitInput | typing.Any | None) -> "Kit":
        """🧪 Parse the input to a kit."""
        if input is None:
            return cls()
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
        props = KitProps.model_validate(obj)
        entity = cls(**props.model_dump())
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
        return entity

    def dump(self) -> "KitOutput":
        entity = {**KitProps.model_validate(self).model_dump()}
        entity["types"] = [t.dump() for t in self.types]
        entity["designs"] = [d.dump() for d in self.designs]
        entity["qualities"] = [q.dump() for q in self.qualities]
        return KitOutput(**entity)

    # TODO: Automatic emptying.
    def empty(self) -> "Kit":
        """🪣 Empty the kit."""
        props = KitProps.model_construct()
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        self.types = []
        return self

    # TODO: Automatic updating based on props.
    def update(self, other: "Kit", empty: bool = False) -> "Kit":
        """🔄 Update the props of the kit. Optionally empty the kit before."""
        if empty:
            self.empty()
        props = KitProps.model_validate(other)
        for key, value in props.model_dump().items():
            setattr(self, key, value)
        return self

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the kit."""
        return self.uri

    def guid(self) -> str:
        """🔗 The guid of the kit."""
        return self.id()


# Store #


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
    # E.g:
    # QzpcZ2l0XHNlbWlvXGV4YW1wbGVzXG1ldGFib2xpc20=
    # QzpcZ2l0XHNlbWlvXGV4YW1wbGVzXG1ldGFib2xpc20=/types
    # QzpcZ2l0XHNlbWlvXGV4YW1wbGVzXG1ldGFib2xpc20=/types/Q2Fwc3VsZQ==,
    # QzpcZ2l0XHNlbWlvXGV4YW1wbGVzXG1ldGFib2xpc20=/types/Q2Fwc3VsZQ==,/representations
    # QzpcZ2l0XHNlbWlvXGV4YW1wbGVzXG1ldGFib2xpc20=/types/Q2Fwc3VsZQ==,/representations/aHR0cHM6Ly9hcHAuc3BlY2tsZS5zeXN0ZW1zL3Byb2plY3RzL2U3ZGUxYTJmOGYvbW9kZWxzL2IzYzIwZGI5NzA=
    # C:\git\semio\examples\metabolism/types/Capsule,/representations/https://app.speckle.systems/projects/e7de1a2f8f/models/b3c20db970
    # uri: C:\git\semio\examples\metabolism
    # kind: type
    # typeName: Capsule
    # typeVariant: ""
    # representationUrl: https://app.speckle.systems/projects/e7de1a2f8f/models/b3c20db970

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

    # def representation(self, children):
    #     type = children[0]
    #     code = {
    #         "typeName": type["typeName"],
    #         "typeVariant": type["typeVariant"],
    #     }
    #     if len(children) == 1:
    #         code["kind"] = "representations"
    #     else:
    #         code["kind"] = "representation"
    #         code["representationUrl"] = decode(children[1].value)

    #     return code

    # def port(self, children):
    #     type = children[0]
    #     code = {
    #         "typeName": type["typeName"],
    #         "typeVariant": type["typeVariant"],
    #     }
    #     if len(children) == 1:
    #         code["kind"] = "ports"
    #     else:
    #         code["kind"] = "port"
    #         code["portUrl"] = decode(children[1].value)
    #     return code


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

    def execute(
        self, command: CommandKind = CommandKind.QUERY, code: str = "", input: str = ""
    ) -> typing.Any:
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

    def put(
        self: "DatabaseStore",
        operation: dict,
        input: KitInput | DesignInput | TypeInput,
    ) -> typing.Any:
        # General:
        # - Wrap iteration over relationships in list() to avoid iterator bugs
        # - When deleting relationships, set property = [] after deleting all items and then add the new ones

        kitUri = operation["kitUri"]
        kind = operation["kind"]

        if kind == "kit":
            self.initialize()
            dump = input.model_dump()
            dump["uri"] = kitUri
            kit = Kit.parse(dump)
            existingKit = (
                self.session.query(Kit).filter(Kit.uri == kitUri).one_or_none()
            )
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
                types = [
                    u.Type
                    for u in self.session.query(Type, Kit)
                    .filter(Kit.uri == kitUri)
                    .all()
                ]
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
                        design = Design.parse(input, types)
                        design.kit = kit
                        self.session.add(design)
                        self.session.commit()
                    else:
                        design = Design.parse(input, types)
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

                        # gather
                        existingType = existingTypeUnion.Type
                        existingPorts = {p.id_: p for p in existingType.ports}
                        usedPorts = {}
                        for port in list(existingType.ports):
                            for connection in port.connections:
                                if connection.connectedPiece.type == existingType:
                                    usedPorts[connection.connectedPort.id_] = (
                                        connection.connectedPort
                                    )
                                if connection.connectingPiece.type == existingType:
                                    usedPorts[connection.connectingPort.id_] = (
                                        connection.connectingPort
                                    )
                        newPorts = {p.id_: p for p in type.ports}
                        missingPorts = set(usedPorts.keys()) - set(newPorts.keys())
                        if missingPorts:
                            raise TypeHasNotAllUsedPorts(missingPorts)
                        unusedPorts = set(existingPorts.keys()) - set(usedPorts.keys())

                        # update
                        existingType.icon = type.icon
                        existingType.image = type.image
                        existingType.description = type.description
                        existingType.unit = type.unit
                        existingType.updated = datetime.datetime.now()
                        for usedPortId, usedPort in usedPorts.items():
                            usedPort.point = newPorts[usedPortId].point
                            usedPort.direction = newPorts[usedPortId].direction

                            for quality in list(usedPort.qualities):
                                self.session.delete(quality)
                            usedPort.qualities = []
                            self.session.flush()

                            newQualities = []
                            for newQuality in list(newPorts[usedPortId].qualities):
                                newQuality.port = usedPort
                                self.session.add(newQuality)
                                newQualities.append(newQuality)
                            usedPort.qualities = newQualities
                            self.session.flush()

                        for unusedPort in list(unusedPorts):
                            self.session.delete(existingPorts[unusedPort])
                        existingType.ports = [
                            p for p in existingType.ports if p.id_ not in unusedPorts
                        ]
                        self.session.flush()

                        for newPortId, newPort in newPorts.items():
                            if newPortId not in usedPorts:
                                newPort.type = existingType
                                self.session.add(newPort)
                        self.session.flush()

                        existingType.representations = []
                        for representation in list(type.representations):
                            representation.type = existingType
                            self.session.add(representation)
                        self.session.flush()

                        existingType.qualities = []
                        for quality in list(type.qualities):
                            quality.type = existingType
                            self.session.add(quality)
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
    except requests.exceptions.HTTPError as e:
        # TODO: Better error message.
        raise KitNotFound(remoteUri)

    with zipfile.ZipFile(io.BytesIO(response.content)) as zip:
        zip.extractall(path)
        paths = os.listdir(path)
        while not ".semio" in paths:
            if len(paths) != 1:
                raise KitZipDoesNotContainSemioFolder()
            nestedPath = os.path.join(path, paths[0])
            nestedDirectories = os.listdir(nestedPath)
            for nestedDirectory in nestedDirectories:
                shutil.move(os.path.join(nestedPath, nestedDirectory), path)
            os.rmdir(nestedPath)
            paths = os.listdir(path)
        # for directory in paths:
        #     if directory != ".semio":
        #         if os.path.isfile(os.path.join(path, directory)):
        #             os.remove(os.path.join(path, directory))
        #         else:
        #             shutil.rmtree(os.path.join(path, directory))
    return path


class SqliteStore(DatabaseStore):
    path: pathlib.Path

    def __init__(
        self, uri: str, engine: sqlalchemy.engine.Engine, path: pathlib.Path
    ) -> None:
        super().__init__(uri, engine)
        self.path = path

    @classmethod
    def fromUri(cls, uri: str, path: str = "") -> "SqliteStore":
        if path == "":
            path = uri
        sqlitePath = (
            pathlib.Path(path)
            / pathlib.Path(KIT_LOCAL_FOLDERNAME)
            / pathlib.Path(KIT_LOCAL_FILENAME)
        )
        connectionString = f"sqlite:///{sqlitePath}"
        engine = sqlalchemy.create_engine(connectionString, echo=True)
        session = sqlalchemy.orm.sessionmaker(bind=engine)()
        try:  # change uri if local kit is already created
            kit = session.query(Kit).first()
            kit.uri = uri
            session.commit()
            session.close()
        except sqlalchemy.exc.OperationalError as e:
            pass
        return SqliteStore(uri, engine, sqlitePath)

    def initialize(self: "DatabaseStore") -> None:
        os.makedirs(
            str(pathlib.Path(self.uri) / pathlib.Path(KIT_LOCAL_FOLDERNAME)),
            exist_ok=True,
        )
        sqlmodel.SQLModel.metadata.create_all(self.engine)
        session = sqlalchemy.orm.sessionmaker(bind=self.engine)()
        existingSemio = session.query(Semio).one_or_none()
        if not existingSemio:
            session.add(Semio())
            session.commit()
        session.close()

    def postDeleteKit(self: "SqliteStore") -> None:
        # sqlachemy can't maintain the connection to the database after the file is deleted.
        # Therefore, the process is terminated and will be restarted by the client.
        os.kill(os.getpid(), signal.SIGTERM)


class PostgresStore(DatabaseStore):

    @classmethod
    def fromUri(cls, uri: str):
        # TODO: Get connection string from environment variable.
        # dbAliases = {key: value for key, value in ENVS.items() if key.contains("_POSTGRES_ALIAS_")}
        # uris = {key: value for key, value in dbAliases.items() if key.endswith("_URI")}
        # connection_string = sqlalchemy.URL.create(
        #     "postgresql+psycopg",
        #     username=parsedUri.username,
        #     password=parsedUri.password,
        #     host=parsedUri.hostname,
        #     database=parsedUri.path[1:],  # Remove the leading '/'
        # )
        # engine = sqlalchemy.create_engine(
        #     connection_string,
        #     connect_args={"sslmode": parsedUri.query.get("sslmode", SSLMode.REQUIRE)},
        # )
        # return PostgresStore(uri, engine)
        raise FeatureNotYetSupported()

    def initialize(self: "DatabaseStore") -> None:
        sqlmodel.SQLModel.metadata.create_all(self.engine)


# class ApiStore(Store, abc.ABC):
# pass


# class RestStore(ApiStore):
# pass


# class GraphqlStore(Store):
# pass


# The cache is necessary to persist the session!
# An other option would be to eager load the relationships.
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
        raise RemoteKitsNotYetSupported()
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


# Assistant #


def encodeForPrompt(context: str):
    return context.replace(";", ",").replace("\n", " ")


def replaceDefault(context: str, default: str):
    if context == "":
        return context.replace("", default)
    return context


def encodeType(type: TypeContext):
    typeClone = type.model_copy(deep=True)
    typeClone.variant = replaceDefault(typeClone.variant, "DEFAULT")
    typeClone.description = (
        encodeForPrompt(typeClone.description)
        if typeClone.description != ""
        else "NO_DESCRIPTION"
    )
    for port in typeClone.ports:
        port.id_ = replaceDefault(port.id_, "DEFAULT")
        # for quality in port.qualities:
        #     quality.value = replaceDefault(quality.value, "TRUE")
    return typeClone


def decodeDesign(design: dict):
    decodedDesign = {
        "pieces": [
            {
                "id_": p["id"] if p["id"] != "DEFAULT" else "",
                "type": {
                    "name": p["typeName"],
                    "variant": (
                        p["typeVariant"] if p["typeVariant"] != "DEFAULT" else ""
                    ),
                },
            }
            for p in design["pieces"]
        ],
        "connections": [
            {
                "connected": {
                    "piece": {
                        "id_": (
                            c["connectedPieceId"]
                            if c["connectedPieceId"] != "DEFAULT"
                            else ""
                        ),
                    },
                    "port": {
                        "id_": (
                            c["connectedPieceTypePortId"]
                            if c["connectedPieceTypePortId"] != "DEFAULT"
                            else ""
                        ),
                    },
                },
                "connecting": {
                    "piece": {
                        "id_": (
                            c["connectingPieceId"]
                            if c["connectingPieceId"] != "DEFAULT"
                            else ""
                        ),
                    },
                    "port": {
                        "id_": (
                            c["connectingPieceTypePortId"]
                            if c["connectingPieceTypePortId"] != "DEFAULT"
                            else ""
                        ),
                    },
                },
                "gap": c["gap"],
                "shift": c["shift"],
                "raise": c["raise"],
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
    portD = {}
    pieceD = {}
    for type in types:
        if type.name not in typeD:
            typeD[type.name] = {}
            portD[type.name] = {}
        typeD[type.name][type.variant] = type
        if type.variant not in portD[type.name]:
            portD[type.name][type.variant] = {}
        for port in type.ports:
            portD[type.name][type.variant][port.id_] = port
    # TODO: Try closest embedding instead of smallest Levenshtein distance.
    for piece in designClone.pieces:
        pieceD[piece.id_] = piece
        if piece.type.name not in typeD:
            # TODO: Remove piece if type name is not found instead of taking the first.
            try:
                piece.type.name = difflib.get_close_matches(
                    piece.type.name, typeD.keys(), n=1
                )[0]
            except Error as e:  # TODO: Make more specific
                piece.type.name = typeD.keys()[0]
        if not (piece.type.variant in typeD[piece.type.name]):
            try:
                piece.type.variant = difflib.get_close_matches(
                    piece.type.variant, typeD[piece.type.name].keys(), n=1
                )[0]
            except Error as e:  # TODO: Make more specific
                piece.type.variant = typeD[piece.type.name].keys()[0]

    validConnections = []
    for connection in designClone.connections:
        if connection.connected.piece.id_ not in pieceD:
            try:
                connection.connected.piece.id_ = difflib.get_close_matches(
                    connection.connected.piece.id_, pieceD.keys(), n=1
                )[0]
            except Error as e:
                continue
        if connection.connecting.piece.id_ not in pieceD:
            try:
                connection.connecting.piece.id_ = difflib.get_close_matches(
                    connection.connecting.piece.id_, pieceD.keys(), n=1
                )[0]
            except Error as e:
                continue
        connectedType = typeD[pieceD[connection.connected.piece.id_].type.name][
            pieceD[connection.connected.piece.id_].type.variant
        ]
        connectingType = typeD[pieceD[connection.connecting.piece.id_].type.name][
            pieceD[connection.connecting.piece.id_].type.variant
        ]

        if (
            connection.connected.port.id_
            not in portD[connectedType.name][connectedType.variant]
        ):
            connection.connected.port.id_ = difflib.get_close_matches(
                connection.connected.port.id_,
                portD[connectedType.name][connectedType.variant].keys(),
                n=1,
            )[0]
        if (
            connection.connecting.port.id_
            not in portD[connectingType.name][connectingType.variant]
        ):
            connection.connecting.port.id_ = difflib.get_close_matches(
                connection.connecting.port.id_,
                portD[connectingType.name][connectingType.variant].keys(),
                n=1,
            )[0]
        validConnections.append(connection)
    designClone.connections = validConnections
    # remove invalid connections
    designClone.connections = [
        c for c in designClone.connections if c.connected.piece.id_ != c.connecting
    ]
    # remove pieces with no connections
    designClone.pieces = [
        p
        for p in designClone.pieces
        if any(
            c
            for c in designClone.connections
            if c.connected.piece.id_ == p.id_ or c.connecting.piece.id_ == p.id_
        )
    ]
    return designClone


try:
    openaiClient = openai.Client()
except openai.OpenAIError as e:
    pass

systemPrompt = """You are a kit-of-parts design assistant.
Rules:
Every piece MUST have a type that exists. The type name and type variant MUST match.
Two pieces are different when they have a different type name or type variant.
Two types are different when they have a different name or different variant.
Every connected and connecting piece MUST be part of the pieces of the design. The ids MUST match.
The port of connected and connecting pieces MUST exist in the type of the piece. The ids MUST match.
The port of connected and connecting pieces SHOULD match.
If the ports of connected and connecting pieces have a family, they should be compatible.
If one port has the other port as ocompatible that's enough.
Every piece in the design MUST be connected to at least one other piece.
One piece is the root piece of the design. The connections MUST form a tree.
Ids SHOULD be abreviated and don't have to be globally unique.
Rotation, tilt, gap, shift SHOULD NOT be added unless specifically instructed.
The diagram is only a nice 2D representation of the design and does not change the design.
When a piece is [on, next to, above, below, ...] another piece, there SHOULD be a connected between the pieces.
When a piece fits to a port of another piece, there SHOULD be a connecting between the pieces."""
# logger.debug("System prompt: {}", systemPrompt)

designGenerationPromptTemplate = jinja2.Template(
    """Your task is to help to puzzle together a design.

TYPE{NAME;VARIANT;DESCRIPTION;PORTS}
PORT{ID;DESCRIPTION,FAMILY,COMPATIBLEFAMILIES}
COMPATIBLEFAMILY{NAME}

Available types:
{% for type in types %}
{% raw %}{{% endraw -%}
{{ type.name }};{{ type.variant }};{{ type.description }};
{%- for port in type.ports %}
{%- raw %}{{% endraw -%}{{ port.id_ }};{{ port.description }};{{ port.family }}
{%- for compatibleFamily in port.compatibleFamilies %}
{%- raw %}{{% endraw -%}
{{ compatibleFamily }}
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
                            "description": "The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece. "
                        },
                        "shift": {
                            "type": "number",
                            "description": "The optional lateral shift (applied after the rotation, the turn and the tilt in the plane) between the connected and the connecting piece.."
                        },
                        "raise": {
                            "type": "number",
                            "description": "The optional vertical raise in port direction between the connected and the connecting piece. Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."
                        },
                        "rotation": {
                            "type": "number",
                            "description": "The optional horizontal rotation in port direction between the connected and the connecting piece in degrees."
                        },
                        "turn": {
                            "type": "number",
                            "description": "The optional turn perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees.  Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."
                        },
                        "tilt": {
                            "type": "number",
                            "description": "The optional horizontal tilt perpendicular to the port direction (applied after rotation and the turn) between the connected and the connecting piece in degrees."
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
                        "raise",
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
# with open("../../jsonschema/design-prediction-openai.json", "r") as f:
#     designResponseFormat = json.load(f)


def predictDesign(
    description: str, types: list[TypeContext], design: DesignInput | None = None
) -> DesignPrediction:
    """🔮 Predict a design based on a description, the types that should be used and an optional base design."""
    prompt = designGenerationPromptTemplate.render(
        description=description, types=[encodeType(t) for t in types]
    )
    logger.debug("Generated prompt: {}", prompt)
    try:
        response = openaiClient.chat.completions.create(
            # model="o1-mini",
            model="gpt-4o",
            # model="gpt-4o-mini",
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
            # temperature=1,
            # max_completion_tokens=16383,
            # top_p=1,
            # frequency_penalty=0,
            # presence_penalty=0,
        )
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
    except Error as e:
        logger.error("Error occurred: {}", e)
        pass

    logger.debug("Schema: {}", json.dumps(designResponseFormat, indent=4))
    logger.debug("Prompt: {}", prompt)
    logger.debug("System Prompt: {}", systemPrompt)
    logger.debug(
        "Predicted Design Raw: {}",
        json.dumps(json.loads(response.choices[0].message.content), indent=4),
    )

    result = response.choices[0]
    if result.finish_reason == "stop" and result.message.refusal is None:
        design = decodeDesign(json.loads(result.message.content))

        logger.debug("Predicted Design: {}", json.dumps(design.model_dump(), indent=4))

        # piece healing of variants that do not exist
        healedDesign = healDesign(design, types)
        logger.debug(
            "Predicted Design Healed: {}",
            json.dumps(healedDesign.model_dump(), indent=4),
        )

        return healedDesign


# Graphql #


GRAPHQLTYPES = {
    "str": graphene.NonNull(graphene.String),
    "int": graphene.NonNull(graphene.Int),
    "float": graphene.NonNull(graphene.Float),
    "bool": graphene.NonNull(graphene.Boolean),
    "list[str]": graphene.NonNull(graphene.List(graphene.NonNull(graphene.String))),
    "Quality": graphene.NonNull(lambda: QualityNode),
    "list[Quality]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: QualityNode))
    ),
    "list[__main__.Quality]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: QualityNode))
    ),
    "list[__mp_main__.Quality]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: QualityNode))
    ),
    "list[engine.Quality]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: QualityNode))
    ),
    "DiagramPoint": graphene.NonNull(lambda: DiagramPointNode),
    "typing.Optional[__main__.DiagramPoint]": lambda: DiagramPointNode,
    "typing.Optional[__mp_main__.DiagramPoint]": lambda: DiagramPointNode,
    "typing.Optional[engine.DiagramPoint]": lambda: DiagramPointNode,
    "Location": graphene.NonNull(lambda: LocationNode),
    "typing.Optional[__main__.Location]": lambda: LocationNode,
    "typing.Optional[__mp_main__.Location]": lambda: LocationNode,
    "typing.Optional[engine.Location]": lambda: LocationNode,
    "Point": graphene.NonNull(lambda: PointNode),
    "Vector": graphene.NonNull(lambda: VectorNode),
    "Plane": graphene.NonNull(lambda: PlaneNode),
    "Port": graphene.NonNull(lambda: PortNode),
    "PortId": graphene.NonNull(lambda: PortNode),
    "list[Port]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: PortNode))),
    "list[__main__.Port]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: PortNode))
    ),
    "list[__mp_main__.Port]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: PortNode))
    ),
    "list[engine.Port]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: PortNode))
    ),
    "Representation": graphene.NonNull(lambda: RepresentationNode),
    "list[Representation]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: RepresentationNode))
    ),
    "list[__main__.Representation]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: RepresentationNode))
    ),
    "list[__mp_main__.Representation]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: RepresentationNode))
    ),
    "list[engine.Representation]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: RepresentationNode))
    ),
    "Author": graphene.NonNull(lambda: AuthorNode),
    "list[Author]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: AuthorNode))
    ),
    "list[__main__.Author]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: AuthorNode))
    ),
    "list[__mp_main__.Author]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: AuthorNode))
    ),
    "list[engine.Author]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: AuthorNode))
    ),
    "Type": graphene.NonNull(lambda: TypeNode),
    "TypeId": graphene.NonNull(lambda: TypeNode),
    "list[Type]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    "list[__main__.Type]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: TypeNode))
    ),
    "list[__mp_main__.Type]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: TypeNode))
    ),
    "list[engine.Type]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: TypeNode))
    ),
    "Piece": graphene.NonNull(lambda: PieceNode),
    "PieceId": graphene.NonNull(lambda: PieceNode),
    "Side": graphene.NonNull(lambda: SideNode),
    "Connection": graphene.NonNull(lambda: ConnectionNode),
    "list['Connection']": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: ConnectionNode))
    ),
    "list[__main__.Connection]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: ConnectionNode))
    ),
    "list[__mp_main__.Connection]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: ConnectionNode))
    ),
    "list[engine.Connection]": graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: ConnectionNode))
    ),
    "Design": graphene.NonNull(lambda: DesignNode),
    "Kit": graphene.NonNull(lambda: KitNode),
}


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
        if "name" not in options:
            options["name"] = model.__name__

        own_properties = [
            name
            for name, value in model.__dict__.items()
            if isinstance(value, property)
        ]

        def make_resolve(name):
            def resolve(self, info):
                return getattr(self, name)

            return resolve

        # Dynamically add resolvers for all properties
        for name in own_properties:
            prop = getattr(model, name)
            prop_getter = prop.fget
            prop_return_type = inspect.signature(prop_getter).return_annotation
            if prop_return_type.__name__.startswith("Optional"):
                graphqlTypeName = prop_return_type.__args__[0].__name__
            elif prop_return_type.__name__.startswith("list"):
                graphqlTypeName = str(prop_return_type)
            else:
                graphqlTypeName = prop_return_type.__name__
            setattr(
                cls,
                name,
                GRAPHQLTYPES[graphqlTypeName],
            )
            setattr(cls, f"resolve_{name}", make_resolve(name))

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


class QualityNode(TableEntityNode):
    class Meta:
        model = Quality


class QualityInputNode(InputNode):
    class Meta:
        model = QualityInput


class LocationNode(Node):
    class Meta:
        model = Location


class LocationInputNode(InputNode):
    class Meta:
        model = Location


class RepresentationNode(TableEntityNode):
    class Meta:
        model = Representation
        excludedFields = ("tags_",)

    # qualities = graphene.List(graphene.NonNull(lambda: QualityNode))

    # def resolve_qualities(self, info):
    #     return self.qualities


class RepresentationInputNode(InputNode):
    class Meta:
        model = RepresentationInput


class DiagramPointNode(Node):
    class Meta:
        model = DiagramPoint


class DiagramPointInputNode(InputNode):
    class Meta:
        model = DiagramPointInput


class PointNode(Node):
    class Meta:
        model = Point


class PointInputNode(InputNode):
    class Meta:
        model = PointInput


class VectorNode(Node):
    class Meta:
        model = Vector


class VectorInputNode(InputNode):
    class Meta:
        model = VectorInput


class PlaneNode(TableNode):
    class Meta:
        model = Plane


class PlaneInputNode(InputNode):
    class Meta:
        model = PlaneInput


class PortNode(TableEntityNode):
    class Meta:
        model = Port
        exclude_fields = ("connecteds", "connectings")

    # qualities = graphene.List(graphene.NonNull(lambda: QualityNode))

    # def resolve_qualities(self, info):
    #     return self.qualities


class PortInputNode(InputNode):
    class Meta:
        model = PortInput


class PortIdInputNode(InputNode):
    class Meta:
        model = PortId


class AuthorNode(TableEntityNode):
    class Meta:
        model = Author


class AuthorInputNode(InputNode):
    class Meta:
        model = AuthorInput


class TypeNode(TableEntityNode):
    class Meta:
        model = Type


class TypeInputNode(InputNode):
    class Meta:
        model = TypeInput


class TypeIdInputNode(InputNode):
    class Meta:
        model = TypeId


class PieceNode(TableEntityNode):
    class Meta:
        model = Piece
        exclude_fields = ("connecteds", "connectings")


class PieceInputNode(InputNode):
    class Meta:
        model = PieceInput
        exclude_fields = "type"

    type = graphene.NonNull(TypeIdInputNode)


class PieceIdInputNode(InputNode):
    class Meta:
        model = PieceId


class SideNode(Node):
    class Meta:
        model = Side
        exclude_fields = ("piece", "port")

    piece = graphene.NonNull(PieceNode)
    port = graphene.NonNull(PortNode)

    def resolve_piece(self, info):
        return self.piece

    def resolve_port(self, info):
        return self.port


class SideInputNode(InputNode):
    class Meta:
        model = SideInput
        exclude_fields = ("piece", "port")

    piece = graphene.NonNull(PieceIdInputNode)
    port = graphene.NonNull(PortIdInputNode)


class ConnectionNode(TableEntityNode):
    class Meta:
        model = Connection
        exclude_fields = (
            "connectedPiece",
            "connectedPort",
            "connectingPiece",
            "connectingPort",
        )

    connected = graphene.NonNull(lambda: SideNode)
    connecting = graphene.NonNull(lambda: SideNode)

    def resolve_connected(self, info):
        return self.connected

    def resolve_connecting(self, info):
        return self.connecting


class ConnectionInputNode(InputNode):
    class Meta:
        model = ConnectionInput


class DesignInputNode(InputNode):
    class Meta:
        model = DesignInput


class DesignNode(TableEntityNode):
    class Meta:
        model = Design


class KitInputNode(InputNode):
    class Meta:
        model = KitInput


class KitNode(TableEntityNode):
    class Meta:
        model = Kit


# # Can't use SQLAlchemyConnectionField because only supports one database.
# # https://github.com/graphql-python/graphene-sqlalchemy/issues/180
# class KitConnection(graphene.relay.Connection):
#     class Meta:
#         node = KitNode


class Query(graphene.ObjectType):
    node = RelayNode.Field()
    kit = graphene.Field(KitNode, uri=graphene.String(required=True))
    # kits = graphene.relay.ConnectionField(KitConnection)

    def resolve_kit(self, info, uri):
        return get(encode(uri))


class Mutation(graphene.ObjectType):
    createKit = graphene.Field(KitNode, kit=KitInputNode(required=True))


graphqlSchema = graphene.Schema(
    query=Query,
    mutation=Mutation,
)

# Rest #

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
async def prepare_kit(
    request: fastapi.Request, kit: KitInput = fastapi.Body(...)
) -> KitContext:
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
    openapi_schema = fastapi.openapi.utils.get_openapi(
        title="semio REST API",
        version=VERSION,
        summary="This is the local rest API of the semio engine.",
        routes=rest.routes,
    )
    # Prepend `/api` to all paths in the OpenAPI schema
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

# Engine #

engine = starlette.applications.Starlette()
engine.mount("/api", rest)
engine.mount(
    "/graphql",
    starlette_graphene3.GraphQLApp(
        graphqlSchema, on_get=starlette_graphene3.make_graphiql_handler()
    ),
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
            DesignPrediction.model_json_schema(
                schema_generator=PredictionGenerateJsonSchema
            ),
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
    logging.basicConfig(level=logging.INFO)  # for uvicorn in pyinstaller
    uvicorn.run(
        engine,
        host=HOST,
        port=PORT,
        log_level="info",
        access_log=False,
        log_config=None,
    )


def restart_engine():
    ui_instance = PySide6.QtWidgets.QApplication.instance()
    engine_process = ui_instance.engine_process
    if engine_process.is_alive():
        engine_process.terminate()
    ui_instance.engine_process = multiprocessing.Process(target=start_engine)
    ui_instance.engine_process.start()


def run():
    logger.debug("Starting engine")
    multiprocessing.freeze_support()  # needed for pyinstaller on Windows

    parser = argparse.ArgumentParser(description="semio engine")
    parser.add_argument(
        "-d",
        "--debug",
        help="debug mode",
        action="store_true",
    )

    args = parser.parse_args()
    if args.debug:
        logger.add(sys.stderr, level="INFO")
        logger.add(DEBUG_LOG_FILE, level="DEBUG", rotation="10 MB")

    ui = PySide6.QtWidgets.QApplication(sys.argv)
    ui.setQuitOnLastWindowClosed(False)

    # Final location of assets when bundled with PyInstaller
    if getattr(sys, "frozen", False):
        basedir = sys._MEIPASS
    else:
        basedir = "../../assets"

    icon = PySide6.QtGui.QIcon()
    icon.addFile(
        os.path.join(basedir, "icons/semio_512x512.png"), PySide6.QtCore.QSize(512, 512)
    )

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
    # testCaseDict = json.load(open("temp/test-case.json", "r"))
    # testCaseDict["uri"] = "test-case"
    # kit = Kit.parse(testCaseDict)
    # dumpedKit = kit.dump()
    # testDesign = DesignContext(**dumpedKit.designs[0].model_dump())
    # with open("temp/test-case-cleaned.json", "w") as f:
    #     json.dump(testDesign.model_dump(), f)


def dev():
    logger.debug("Starting debugpy for semio engine")
    import debugpy

    debugpy.listen(("0.0.0.0", 5678))  # Start debug server
    logger.debug("Waiting for debugger to attach to semio engine")
    debugpy.wait_for_client()
    preDev()
    run()


if __name__ == "__main__":
    run()
