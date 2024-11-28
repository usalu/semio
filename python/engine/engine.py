#!/usr/bin/env python

# engine.py
# Copyright (C) 2024 Ueli Saluz

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


# TODOs


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
# 💬,Dc?,Dsc,Description,The optional human description of the {{NAME}}.
# 📖,Df,Def,Definition,The optional definition [ text | url ] of the quality.
# ✏️,Dg,Dgm,Diagram,The diagram of the design.
# 📁,Di?,Dir,Directory,The optional directory where to find the kit.
# 🏅,Dl,Dfl,Default,Whether it is the default representation of the type. There can be only one default representation per type.
# ➡️,Dr,Drn,Direction,The direction of the port. The direction of the other port will be flipped and then the pieces will be aligned.
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
# 🖼️,Ic?,Ico,Icon,The optional icon [ emoji | name | url ] of the {{NAME}}.
# 🆔,Id,Id,Identifier,The local identifier of the {{NAME}} within the {{PARENT_NAME}}.
# 🆔,Id?,Id,Identifier,The optional local identifier of the {{NAME}} within the {{PARENT_NAME}}. No id means the default {{NAME}}.
# 🪪,Id,Id,Identifier,The props to identify the {{NAME}} within the parent {{PARENT_NAME}}.
# ↘️,In,Inp,Input,The input for a {{NAME}}.
# 🗃️,Kt,Kit,Kit,A kit is a collection of designs that use types.
# 🗺️,Lc,Loc,Locator,A locator is metadata for grouping ports.
# 🗺️,Lc*,Locs,Locators,The optional locators of the port.
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
# 🔄,Rt?,Rot,Rotation,The optional rotation between the connected and the connecting piece in degrees.
# 🧱,Sd,Sde,Side,A side of a piece in a connection.
# ↔️,Sf,Sft,Shift,The optional lateral shift (applied after rotation and tilt in the plane) between the connected and the connecting piece.
# 📌,SG?,SGr,Subgroup,The optional sub-group of the locator. No sub-group means true.
# 📺,SP,SPt,Screen Point,The 2d-point (xy) of integers in screen plane of the center of the icon in the diagram of the piece.
# ✅,Su,Suc,Success,{{NAME}} was successful.
# 🏷️,Tg*,Tags,Tags,The optional tags to group representations. No tags means default.
# ↗️,Tl?,Tlt,Tilt,The optional tilt (applied after rotation) between the connected and the connecting piece in degrees.
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
# 🏁,X,X,X,The x-coordinate of the screen point.
# 🎚️,X,X,X,The x-coordinate of the point.
# ➡️,XA,XAx,XAxis,The x-axis of the plane.
# 🏁,Y,Y,Y,The y-coordinate of the screen point.
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
import enum
import functools
import inspect
import json
import logging
import multiprocessing
import os
import pathlib
import sqlite3
import typing
import urllib
import zipfile
import io
import shutil
import stat
import signal

import fastapi
import graphene
import graphene_pydantic
import graphene_sqlalchemy
import lark
import networkx
import numpy
import pint
import pytransform3d
import pydantic
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


RELEASE = "r24.12-1"
VERSION = "4.0.2"
HOST = "127.0.0.1"
PORT = 24121
NAME_LENGTH_LIMIT = 64
ID_LENGTH_LIMIT = 128
URL_LENGTH_LIMIT = 1024
URI_LENGTH_LIMIT = 4 * URL_LENGTH_LIMIT
MAX_TAGS = 16
MAX_HIERARCHY = 16
DESCRIPTION_LENGTH_LIMIT = 4096
ENCODING_ALPHABET_REGEX = r"[a-zA-Z0-9\-._~%]"
ENCODING_REGEX = ENCODING_ALPHABET_REGEX + "+"
KIT_LOCAL_FOLDERNAME = ".semio"
KIT_LOCAL_FILENAME = "kit.sqlite3"
KIT_LOCAL_SUFFIX = str(
    pathlib.Path(KIT_LOCAL_FOLDERNAME) / pathlib.Path(KIT_LOCAL_FILENAME)
)
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
MAX_REQUEST_BODY_SIZE = 50 * 1024 * 1024  # 50MB
ENVS = {key: value for key, value in os.environ.items() if key.startswith("SEMIO_")}

ureg = pint.UnitRegistry()


# Utility


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
        description="🍾 The release of the engine that created this database.",
    )
    """🍾 The release of the engine that created this database."""
    createdAt: datetime.datetime = sqlmodel.Field(
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
    """↘️ The base for inputs.  All fields that are required to create the entity."""


class Output(Base, abc.ABC):
    """↗️ The base for outputs. All fields that are returned when the entity is fetched."""


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


### Representations


class RepresentationMimeField(RealField, abc.ABC):
    """✉️ The Multipurpose Internet Mail Extensions (MIME) type of the content of the resource of the representation."""

    mime: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="✉️ The Multipurpose Internet Mail Extensions (MIME) type of the content of the resource of the representation.",
    )
    """✉️ The Multipurpose Internet Mail Extensions (MIME) type of the content of the resource of the representation."""


class RepresentationLodField(RealField, abc.ABC):
    """🔍 The optional Level of Detail/Development/Design (LoD) of the representation. No lod means the default lod."""

    lod: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="🔍 The optional Level of Detail/Development/Design (LoD) of the representation. No lod means the default lod.",
    )
    """🔍 The optional Level of Detail/Development/Design (LoD) of the representation. No lod means the default lod."""


class RepresentationUrlField(RealField, abc.ABC):
    """🔗 The Unique Resource Locator (URL) to the resource of the representation."""

    url: str = sqlmodel.Field(
        max_length=URL_LENGTH_LIMIT,
        description="🔗 The Unique Resource Locator (URL) to the resource of the representation.",
    )
    """🔗 The Unique Resource Locator (URL) to the resource of the representation."""


class RepresentationTagsField(MaskedField, abc.ABC):
    """🏷️ The optional tags to group representations. No tags means default."""

    tags: list[str] = sqlmodel.Field(
        default_factory=list,
        description="🏷️ The optional tags to group representations. No tags means default.",
    )
    """🏷️ The optional tags to group representations. No tags means default."""


class RepresentationId(
    RepresentationTagsField, RepresentationLodField, RepresentationMimeField, Id
):
    """🪪 The props to identify the representation within the parent type."""


class RepresentationProps(
    RepresentationUrlField,
    RepresentationTagsField,
    RepresentationLodField,
    RepresentationMimeField,
    Props,
):
    """🎫 The props of a representation."""


class RepresentationInput(
    RepresentationUrlField,
    RepresentationTagsField,
    RepresentationLodField,
    RepresentationMimeField,
    Input,
):
    """↘️ The input for a representation."""


class RepresentationOutput(
    RepresentationUrlField,
    RepresentationTagsField,
    RepresentationLodField,
    RepresentationMimeField,
    Output,
):
    """↗️ The output of a representation."""


class Representation(
    RepresentationUrlField,
    RepresentationLodField,
    RepresentationMimeField,
    TableEntity,
    table=True,
):
    """💾 A representation is a link to a resource that describes a type for a certain level of detail and tags."""

    PLURAL = "representations"
    __tablename__ = "representation"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the representation in the database."""
    encodedTags: str = sqlmodel.Field(
        max_length=(NAME_LENGTH_LIMIT + 1) * MAX_TAGS - 1,
        default="",
        exclude=True,
    )
    """🧑 The real tags in the database."""
    typePk: typing.Optional[int] = sqlmodel.Field(
        # alias="typeId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "typeId",
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
        return decodeList(self.encodedTags)

    @tags.setter
    def tags(self: "Representation", tags: list[str]):
        """↘️ Set the masked tags of the representation."""
        self.encodedTags = encodeList(tags)

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
        return entity

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the representation within its parent type."""
        return [self.mime, self.lod, self.tags]


### Locators ###


class LocatorGroupField(MaskedField, abc.ABC):
    """👪 The group of the locator."""

    group: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT, description="👪 The group of the locator."
    )
    """👪 The group of the locator."""


class LocatorSubgroupField(RealField, abc.ABC):
    """📌 The optional sub-group of the locator. No sub-group means true."""

    subgroup: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="📌 The optional sub-group of the locator. No sub-group means true.",
    )
    """📌 The optional sub-group of the locator. No sub-group means true."""


class LocatorId(LocatorGroupField, Id):
    """🪪 The props to identify the locator within the parent port."""


class LocatorProps(LocatorSubgroupField, LocatorGroupField, Props):
    """🎫 The props of a locator."""


class LocatorInput(LocatorSubgroupField, LocatorGroupField, Input):
    """↘️ The input for a locator."""


class LocatorOutput(LocatorSubgroupField, LocatorGroupField, Output):
    """↗️ The output of a locator."""


class Locator(LocatorSubgroupField, Table, table=True):
    """🗺️ A locator is meta-data for grouping ports."""

    __tablename__ = "locator"
    group: str = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "groupName",  # group is a reserved word in SQL
            sqlalchemy.String(NAME_LENGTH_LIMIT),
            primary_key=True,
        ),
    )
    """🧑 The real group in the database."""
    portPk: typing.Optional[int] = sqlmodel.Field(
        # alias="portId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "portId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("port.id"),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent port of the locator in the database."""
    port: typing.Optional["Port"] = sqlmodel.Relationship(back_populates="locators")
    """👪 The parent port of the locator."""


### Screen Points ###


class ScreenPoint(Model):
    """📺 A 2d-point (xy) of integers in screen coordinate system."""

    x: int = sqlmodel.Field(description="🏁 The x-coordinate of the screen point.")
    """🏁 The x-coordinate of the screen point."""
    y: int = sqlmodel.Field(description="🏁 The y-coordinate of the screen point.")
    """🏁 The y-coordinate of the screen point."""

    def __init__(self, x: int = 0, y: int = 0):
        super().__init__(x=x, y=y)

    def __len__(self):
        return 2

    def __getitem__(self, key):
        if key == 0:
            return self.x
        elif key == 1:
            return self.y
        else:
            raise IndexError("Index out of range")

    def __iter__(self):
        return iter((self.x, self.y))


class ScreenPointInput(ScreenPoint, Input):
    """↘️ The input for a screen point."""


class ScreenPointOutput(ScreenPoint, Output):
    """↗️ The output of a screen point."""


### Points ###


class Point(Model):
    """✖️ A 3d-point (xyz) of floating point numbers."""

    x: float = sqlmodel.Field(description="🎚️ The x-coordinate of the point.")
    """🎚️ The x-coordinate of the point."""
    y: float = sqlmodel.Field(description="🎚️ The y-coordinate of the point.")
    """🎚️ The y-coordinate of the point."""
    z: float = sqlmodel.Field(description="🎚️ The z-coordinate of the point.")
    """🎚️ The z-coordinate of the point."""

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __len__(self):
        return 3

    def __getitem__(self, key):
        if key == 0:
            return self.x
        elif key == 1:
            return self.y
        elif key == 2:
            return self.z
        else:
            raise IndexError("Index out of range")

    def __iter__(self):
        return iter((self.x, self.y, self.z))

    def isCloseTo(self, other: "Point", tol: float = TOLERANCE) -> bool:
        return (
            abs(self.x - other.x) < tol
            and abs(self.y - other.y) < tol
            and abs(self.z - other.z) < tol
        )

    def transform(self, transform: "Transform") -> "Point":
        return Transform.transformPoint(transform, self)

    def toVector(self) -> "Vector":
        return Vector(self.x, self.y, self.z)


class PointInput(Point, Input):
    """↘️ The input for a point."""


class PointOutput(Point, Output):
    """↗️ The output of a point."""


### Vectors ###


class Vector(Model):
    """➡️ A 3d-vector (xyz) of floating point numbers."""

    x: float = sqlmodel.Field(description="🎚️ The x-coordinate of the vector.")
    """🎚️ The x-coordinate of the vector."""
    y: float = sqlmodel.Field(description="🎚️ The y-coordinate of the vector.")
    """🎚️ The y-coordinate of the vector."""
    z: float = sqlmodel.Field(description="🎚️ The z-coordinate of the vector.")
    """🎚️ The z-coordinate of the vector."""

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __repr__(self) -> str:
        return f"[{pretty(self.x)}, {pretty(self.y)}, {pretty(self.z)}]"

    def __len__(self):
        return 3

    def __getitem__(self, key):
        if key == 0:
            return self.x
        elif key == 1:
            return self.y
        elif key == 2:
            return self.z
        else:
            raise IndexError("Index out of range")

    def __iter__(self):
        return iter((self.x, self.y, self.z))

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y, self.z + other.z)

    @property
    def length(self) -> float:
        return (self.x**2 + self.y**2 + self.z**2) ** 0.5

    def revert(self) -> "Vector":
        return Vector(-self.x, -self.y, -self.z)

    def amplify(self, factor: float) -> "Vector":
        return Vector(self.x * factor, self.y * factor, self.z * factor)

    def isCloseTo(self, other: "Vector", tol: float = TOLERANCE) -> bool:
        return (
            abs(self.x - other.x) < tol
            and abs(self.y - other.y) < tol
            and abs(self.z - other.z) < tol
        )

    def normalize(self) -> "Vector":
        length = self.length
        return Vector(x=self.x / length, y=self.y / length, z=self.z / length)

    def dot(self, other: "Vector") -> float:
        return numpy.dot(self, other)

    def cross(self, other: "Vector") -> "Vector":
        return Vector(*numpy.cross(self, other))

    def transform(self, transform: "Transform") -> "Vector":
        return Transform.transformVector(transform, self)

    def toPoint(self) -> "Point":
        return Point(self.x, self.y, self.z)

    def toTransform(self) -> "Transform":
        return Transform.fromTranslation(self)

    @staticmethod
    def X() -> "Vector":
        return Vector(x=1)

    @staticmethod
    def Y() -> "Vector":
        return Vector(y=1)

    @staticmethod
    def Z() -> "Vector":
        return Vector(z=1)


class VectorInput(Vector, Input):
    """↘️ The input for a vector."""


class VectorOutput(Vector, Output):
    """↗️ The output of a vector."""


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
    """↘️ The input for a plane."""

    origin: PointInput = sqlmodel.Field(description="⌱ The origin of the plane.")
    """⌱ The origin of the plane."""
    xAxis: VectorInput = sqlmodel.Field(description="➡️ The x-axis of the plane.")
    """➡️ The x-axis of the plane."""
    yAxis: VectorInput = sqlmodel.Field(description="➡️ The y-axis of the plane.")
    """➡️ The y-axis of the plane."""


class PlaneOutput(PlaneYAxisField, PlaneXAxisField, PlaneOriginField, Output):
    """↗️ The output of a plane."""


class Plane(Table, table=True):
    """◳ A plane is an origin (point) and an orientation (x-axis and y-axis)."""

    __tablename__ = "plane"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the plane in the database."""
    originX: float = sqlmodel.Field(exclude=True)
    """🎚️ The x-coordinate of the origin point of the plane."""
    originY: float = sqlmodel.Field(exclude=True)
    """🎚️ The y-coordinate of the origin point of the plane."""
    originZ: float = sqlmodel.Field(exclude=True)
    """🎚️ The z-coordinate of the origin point of the plane."""
    xAxisX: float = sqlmodel.Field(exclude=True)
    """🎚️ The x-coordinate of the x-axis vector of the plane."""
    xAxisY: float = sqlmodel.Field(exclude=True)
    """🎚️ The y-coordinate of the x-axis vector of the plane."""
    xAxisZ: float = sqlmodel.Field(exclude=True)
    """🎚️ The z-coordinate of the x-axis vector of the plane."""
    yAxisX: float = sqlmodel.Field(exclude=True)
    """🎚️ The x-coordinate of the y-axis vector of the plane."""
    yAxisY: float = sqlmodel.Field(exclude=True)
    """🎚️ The y-coordinate of the y-axis vector of the plane."""
    yAxisZ: float = sqlmodel.Field(exclude=True)
    """🎚️ The z-coordinate of the y-axis vector of the plane."""
    piece: typing.Optional["Piece"] = sqlmodel.Relationship(back_populates="plane")
    """👪 The parent piece of the plane."""
    __table_args__ = (
        sqlalchemy.CheckConstraint(
            """
            (
                (originX IS NULL AND originY IS NULL AND originZ IS NULL AND
                 xAxisX IS NULL AND xAxisY IS NULL AND xAxisZ IS NULL AND
                 yAxisX IS NULL AND yAxisY IS NULL AND yAxisZ IS NULL)
            OR
                (originX IS NOT NULL AND originY IS NOT NULL AND originZ IS NOT NULL AND
                 xAxisX IS NOT NULL AND xAxisY IS NOT NULL AND xAxisZ IS NOT NULL AND
                 yAxisX IS NOT NULL AND yAxisY IS NOT NULL AND yAxisZ IS NOT NULL)
            )
            """,
            name="planeSetOrNotSet",
        ),
    )

    def __init__(
        self, origin: Point = None, xAxis: Vector = None, yAxis: Vector = None
    ):
        if origin is None:
            origin = Point()
        if xAxis is None and yAxis is None:
            xAxis = Vector.X()
            yAxis = Vector.Y()
        if xAxis is None:
            xAxis = Vector()
        if yAxis is None:
            yAxis = Vector()
        if abs(xAxis.length - 1) > TOLERANCE:
            raise ValidationError("The x-axis must be normalized.")
        if abs(yAxis.length - 1) > TOLERANCE:
            raise ValidationError("The y-axis must be normalized.")
        if abs(xAxis.dot(yAxis)) > TOLERANCE:
            raise ValidationError("The x-axis and y-axis must be orthogonal.")
        super().__init__(origin=origin, xAxis=xAxis, yAxis=yAxis)

    @property
    def origin(self) -> Point:
        return Point(
            self.originX,
            self.originY,
            self.originZ,
        )

    @origin.setter
    def origin(self, origin: Point):
        self.originX = origin.x
        self.originY = origin.y
        self.originZ = origin.z

    @property
    def xAxis(self) -> Vector:
        return Vector(
            self.xAxisX,
            self.xAxisY,
            self.xAxisZ,
        )

    @xAxis.setter
    def xAxis(self, xAxis: Vector):
        self.xAxisX = xAxis.x
        self.xAxisY = xAxis.y
        self.xAxisZ = xAxis.z

    @property
    def yAxis(self) -> Vector:
        return Vector(
            self.yAxisX,
            self.yAxisY,
            self.yAxisZ,
        )

    @yAxis.setter
    def yAxis(self, yAxis: Vector):
        self.yAxisX = yAxis.x
        self.yAxisY = yAxis.y
        self.yAxisZ = yAxis.z

    def isCloseTo(self, other: "Plane", tol: float = TOLERANCE) -> bool:
        return (
            self.origin.isCloseTo(other.origin, tol)
            and self.xAxis.isCloseTo(other.xAxis, tol)
            and self.yAxis.isCloseTo(other.yAxis, tol)
        )

    def transform(self, transform: "Transform") -> "Plane":
        return Transform.transformPlane(transform, self)

    def toTransform(self) -> "Transform":
        return Transform.fromPlane(self)

    @staticmethod
    def XY() -> "Plane":
        return Plane(
            origin=Point(),
            xAxis=Vector.X(),
            yAxis=Vector.Y(),
        )

    @staticmethod
    def fromYAxis(yAxis: Vector, theta: float = 0.0, origin: Point = None) -> "Plane":
        if abs(yAxis.length - 1) > TOLERANCE:
            raise SpecificationError("The yAxis must be normalized.")
        if origin is None:
            origin = Point()
        orientation = Transform.fromDirections(Vector.Y(), yAxis)
        rotation = Transform.fromAngle(yAxis, theta)
        xAxis = Vector.X().transform(rotation.after(orientation))
        return Plane(origin=origin, xAxis=xAxis, yAxis=yAxis)

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
        origin = PointInput.model_validate(obj["origin"])
        xAxis = VectorInput.model_validate(obj["xAxis"])
        yAxis = VectorInput.model_validate(obj["yAxis"])
        entity = Plane()
        entity.origin = origin
        entity.xAxis = xAxis
        entity.yAxis = yAxis
        return entity


### Rotations ### TODO


class Rotation(Model):
    """🔄 A rotation is an axis and an angle."""

    axis: Vector
    angle: float

    def __init__(self, axis: Vector, angle: float):
        super().__init__(axis=axis, angle=angle)

    def toTransform(self) -> "Transform":
        return Transform.fromRotation(self)


### Transforms ### TODO


class Transform(numpy.ndarray):
    """▦ A 4x4 translation and rotation transformation matrix (no scaling or shearing)."""

    def __new__(cls, input_array=None):
        if input_array is None:
            input_array = numpy.eye(4, dtype=float)
        else:
            input_array = numpy.asarray(input_array).astype(float)
        obj = input_array.view(cls)
        return obj

    def __array_finalize__(self, obj):
        if obj is None:
            return

    def __str__(self) -> str:
        rounded_self = self.round()
        return f"Transform(Rotation={rounded_self.rotation}, Translation={rounded_self.translation})"

    def __repr__(self) -> str:
        rounded_self = self.round()
        return f"Transform(Rotation={rounded_self.rotation}, Translation={rounded_self.translation})"

    @property
    def rotation(self) -> Rotation | None:
        """🔄 The rotation part of the transform."""
        rotationMatrix = self[:3, :3]
        axisAngle = axis_angle_from_matrix(rotationMatrix)
        if axisAngle[3] == 0:
            return None
        return Rotation(
            axis=Vector(float(axisAngle[0]), float(axisAngle[1]), float(axisAngle[2])),
            angle=float(numpy.degrees(axisAngle[3])),
        )

    @property
    def translation(self) -> Vector:
        """➡️ The translation part of the transform."""
        return Vector(*self[:3, 3])

    # for pydantic
    def dict(self) -> typing.Dict[str, typing.Union[Rotation, Vector]]:
        return {
            "rotation": self.rotation,
            "translation": self.translation,
        }

    def after(self, before: "Transform") -> "Transform":
        """✖️ Apply this transform after another transform.

        Args:
            before (Transform): Transform to apply before this transform.

        Returns:
            Transform: New transform.
        """
        return Transform(concat(before, self))

    def invert(self) -> "Transform":
        return Transform(invert_transform(self))

    def transformPoint(self, point: Point) -> Point:
        transformedPoint = transform(self, vector_to_point(point))
        return Point(*transformedPoint[:3])

    def transformVector(self, vector: Vector) -> Vector:
        transformedVector = transform(self, vector_to_direction(vector))
        return Vector(*transformedVector[:3])

    def transformPlane(self, plane: Plane) -> Plane:
        planeTransform = Transform.fromPlane(plane)
        planeTransformed = planeTransform.after(self)
        return Transform.toPlane(planeTransformed)

    def transform(
        self, geometry: typing.Union[Point, Vector, Plane]
    ) -> typing.Union[Point, Vector, Plane]:
        if isinstance(geometry, Point):
            return self.transformPoint(geometry)
        elif isinstance(geometry, Vector):
            return self.transformVector(geometry)
        elif isinstance(geometry, Plane):
            return self.transformPlane(geometry)
        else:
            raise FeatureNotYetSupported()

    def round(self, decimals: int = SIGNIFICANT_DIGITS) -> "Transform":
        return Transform(super().round(decimals=decimals))

    @staticmethod
    def fromTranslation(vector: Vector) -> "Transform":
        return Transform(
            transform_from(
                [
                    [1, 0, 0],
                    [0, 1, 0],
                    [0, 0, 1],
                ],
                vector,
            )
        )

    @staticmethod
    def fromRotation(rotation: Rotation) -> "Transform":
        return Transform(
            transform_from(
                matrix_from_axis_angle((*rotation.axis, radians(rotation.angle))),
                Vector(),
            )
        )

    @staticmethod
    def fromPlane(plane: Plane) -> "Transform":
        # Assumes plane is normalized
        return Transform(
            transform_from(
                [
                    [
                        plane.xAxis.x,
                        plane.yAxis.x,
                        plane.zAxis.x,
                    ],
                    [
                        plane.xAxis.y,
                        plane.yAxis.y,
                        plane.zAxis.y,
                    ],
                    [
                        plane.xAxis.z,
                        plane.yAxis.z,
                        plane.zAxis.z,
                    ],
                ],
                plane.origin,
            )
        )

    @staticmethod
    def fromAngle(axis: Vector, angle: float) -> "Transform":
        return Transform(
            transform_from(matrix_from_axis_angle((*axis, radians(angle))), Vector())
        )

    @staticmethod
    def fromDirections(startDirection: Vector, endDirection: Vector) -> "Transform":
        if startDirection.isCloseTo(endDirection):
            return Transform()
        axisAngle = axis_angle_from_two_directions(startDirection, endDirection)
        return Transform(transform_from(matrix_from_axis_angle(axisAngle), Vector()))

    def toPlane(self) -> Plane:
        return Plane(
            origin=Point(*self[:3, 3]),
            xAxis=Vector(
                self[0, 0],
                self[1, 0],
                self[2, 0],
            ),
            yAxis=Vector(
                self[0, 1],
                self[1, 1],
                self[2, 1],
            ),
        )


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


class PortPointField(MaskedField, abc.ABC):
    """✖️ The connection point of the port that is attracted to another connection point."""

    point: Point = sqlmodel.Field(
        description="✖️ The connection point of the port that is attracted to another connection point."
    )
    """✖️ The connection point of the port that is attracted to another connection point."""


class PortDirectionField(MaskedField, abc.ABC):
    """➡️ The direction of the port. The direction of the other port will be flipped and then the pieces will be aligned."""

    direction: Vector = sqlmodel.Field(
        description="➡️ The direction of the port. The direction of the other port will be flipped and then the pieces will be aligned."
    )
    """➡️ The direction of the port. The direction of the other port will be flipped and then the pieces will be aligned."""


class PortLocatorsField(MaskedField, abc.ABC):
    """🗺️ The locators of the port."""

    locators: list[Locator] = sqlmodel.Field(
        default_factory=list,
        description="🗺️ The locators of the port.",
    )
    """🗺️ The locators of the port."""


class PortId(PortIdField, Id):
    """🪪 The props to identify the port within the parent type."""


class PortProps(
    PortLocatorsField, PortDirectionField, PortPointField, PortIdField, Props
):
    """🎫 The props of a port."""


class PortInput(PortIdField, Input):
    """↘️ The input for a port."""

    point: PointInput = sqlmodel.Field(
        description="✖️ The connection point of the port that is attracted to another connection point."
    )
    """✖️ The connection point of the port that is attracted to another connection point."""
    direction: VectorInput = sqlmodel.Field(
        description="➡️ The direction of the port. The direction of the other port will be flipped and then the pieces will be aligned."
    )
    """➡️ The direction of the port. The direction of the other port will be flipped and then the pieces will be aligned."""
    locators: list[LocatorInput] = sqlmodel.Field(
        default_factory=list,
        description="🗺️ The locators of the port.",
    )
    """🗺️ The locators of the port."""


class PortOutput(PortIdField, PortPointField, PortDirectionField, Output):
    """↗️ The output of a port."""

    locators: list[LocatorOutput] = sqlmodel.Field(
        default_factory=list,
        description="🗺️ The locators of the port.",
    )
    """🗺️ The locators of the port."""


class Port(TableEntity, table=True):
    """🔌 A port is a connection point (with a direction) of a type."""

    PLURAL = "ports"
    __tablename__ = "port"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the port in the database."""
    # Can't use the name 'id' because of bug
    # https://github.com/graphql-python/graphene-sqlalchemy/issues/412
    id_: str = sqlmodel.Field(
        # alias="id",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "localId",
            sqlalchemy.String(ID_LENGTH_LIMIT),
        ),
        default="",
    )
    """🆔 The id of the port within the type."""
    pointX: float = sqlmodel.Field(exclude=True)
    """🎚️ The x-coordinate of the connection point of the port."""
    pointY: float = sqlmodel.Field(exclude=True)
    """🎚️ The y-coordinate of the connection point of the port."""
    pointZ: float = sqlmodel.Field(exclude=True)
    """🎚️ The z-coordinate of the connection point of the port."""
    directionX: float = sqlmodel.Field(exclude=True)
    """🎚️ The x-coordinate of the direction of the port."""
    directionY: float = sqlmodel.Field(exclude=True)
    """🎚️ The y-coordinate of the direction of the port."""
    directionZ: float = sqlmodel.Field(exclude=True)
    """🎚️ The z-coordinate of the direction of the port."""
    typePk: typing.Optional[int] = sqlmodel.Field(
        # alias="typeId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "typeId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent type of the port in the database."""
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="ports")
    """👪 The parent type of the port."""
    locators: list[Locator] = sqlmodel.Relationship(
        back_populates="port", cascade_delete=True
    )
    """🗺️ The locators of the port."""
    connecteds: list["Connection"] = sqlmodel.Relationship(
        back_populates="connectedPort",
        sa_relationship_kwargs={"foreign_keys": "Connection.connectedPortPk"},
    )
    connectings: list["Connection"] = sqlmodel.Relationship(
        back_populates="connectingPort",
        sa_relationship_kwargs={"foreign_keys": "Connection.connectingPortPk"},
    )

    __table_args__ = (
        sqlalchemy.UniqueConstraint("localId", "typeId", name="Unique localId"),
    )

    @property
    def point(self) -> Point:
        """↗️ Get the masked point of the port."""
        return Point(self.pointX, self.pointY, self.pointZ)

    @point.setter
    def point(self, point: Point):
        """↘️ Set the masked point of the port."""
        self.pointX = point.x
        self.pointY = point.y
        self.pointZ = point.z

    @property
    def direction(self) -> Vector:
        """↗️ Get the masked direction of the port."""
        return Vector(self.directionX, self.directionY, self.directionZ)

    @direction.setter
    def direction(self, direction: Vector):
        """↘️ Set the masked direction of the port."""
        self.directionX = direction.x
        self.directionY = direction.y
        self.directionZ = direction.z

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
        point = Point.parse(obj["point"])
        direction = Vector.parse(obj["direction"])
        entity = cls(id_=obj["id_"])
        entity.point = point
        entity.direction = direction
        try:
            locators = [Locator.parse(l) for l in obj["locators"]]
            entity.locators = locators
        except KeyError:
            pass
        return entity

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the port within its parent type."""
        return self.id_


### Qualities ### TODO


class QualityNameField(RealField, abc.ABC):
    """📏 The name of the quality."""

    name: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="📏 The name of the quality.",
    )
    """📏 The name of the quality."""


class QualityValueField(RealField, abc.ABC):
    """📏 The value of the quality."""

    value: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="📏 The value of the quality.",
    )
    """📏 The value of the quality."""


class QualityDefinitionField(RealField, abc.ABC):
    """📏 The definition of the quality."""

    definition: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="📏 The definition of the quality.",
    )
    """📏 The definition of the quality."""


class QualityUnitField(RealField, abc.ABC):
    """📏 The unit of the quality."""

    unit: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="📏 The unit of the quality.",
    )
    """📏 The unit of the quality."""


class QualityId(QualityNameField, Id):
    """🪪 The props to identify the quality within the parent type."""


class QualityProps(
    QualityUnitField,
    QualityDefinitionField,
    QualityValueField,
    QualityNameField,
    Props,
):
    """🎫 The props of a quality."""


class QualityInput(
    QualityUnitField, QualityDefinitionField, QualityValueField, QualityNameField, Input
):
    """↘️ The input for a quality."""


class QualityOutput(
    QualityUnitField,
    QualityDefinitionField,
    QualityValueField,
    QualityNameField,
    Output,
):
    """↗️ The output of a quality."""


class Quality(
    QualityUnitField,
    QualityDefinitionField,
    QualityValueField,
    QualityNameField,
    TableEntity,
    table=True,
):
    """📏 A quality is a named value with a definition and a unit."""

    PLURAL = "qualities"
    __tablename__ = "quality"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the quality in the database."""
    typePk: typing.Optional[int] = sqlmodel.Field(
        # alias="typeId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "typeId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the parent type of the quality in the database."""
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="qualities")
    """👪 The parent type of the quality."""
    designPk: typing.Optional[int] = sqlmodel.Field(
        # alias="designId",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "designId",
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
    __tableargs__ = (
        sqlalchemy.CheckConstraint(
            "typeId IS NOT NULL AND designId IS NULL OR typeId IS NULL AND designId IS NOT NULL",
            name="typeOrDesignSet",
        ),
        sqlalchemy.UniqueConstraint("name", "typeId", "designId"),
    )

    def parent(self) -> "Type":
        """👪 The parent type or design of the quality or otherwise `NoTypeOrDesignAssigned` is raised."""
        if self.type is not None:
            return self.type
        if self.design is not None:
            return self.design
        raise NoTypeOrDesignAssigned()

    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the quality within its parent type."""
        return self.name


### Types ###


class TypeNameField(RealField, abc.ABC):
    """📛 The name of the type."""

    name: str = sqlmodel.Field(
        max_length=NAME_LENGTH_LIMIT,
        description="📛 The name of the type.",
    )
    """📛 The name of the type."""


class TypeDescriptionField(RealField, abc.ABC):
    """💬 The description of the type."""

    description: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="💬 The description of the type.",
    )
    """💬 The description of the type."""


class TypeIconField(RealField, abc.ABC):
    """🖼️ The icon of the type."""

    icon: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🖼️ The icon of the type.",
    )
    """🖼️ The icon of the type."""


class TypeVariantField(RealField, abc.ABC):
    """🔀 The variant of the type."""

    variant: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="🔀 The variant of the type.",
    )
    """🔀 The variant of the type."""


class TypeUnitField(RealField, abc.ABC):
    """📏 The unit of the type."""

    unit: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="📏 The unit of the type.",
    )
    """📏 The unit of the type."""


class TypeCreatedAtField(RealField, abc.ABC):
    """🕒 The creation date of the type."""

    createdAt: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The creation date of the type.",
    )
    """🕒 The creation date of the type."""


class TypeLastUpdateAtField(RealField, abc.ABC):
    """🕒 The last update date of the type."""

    lastUpdateAt: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The last update date of the type.",
    )
    """🕒 The last update date of the type."""


class TypeId(TypeVariantField, TypeNameField, Id):
    """🪪 The props to identify the type."""


class TypeProps(
    TypeUnitField,
    TypeVariantField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Props,
):
    """🎫 The props of a type."""


class TypeInput(
    TypeUnitField,
    TypeVariantField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Input,
):
    """↘️ The input for a type."""

    representations: list[RepresentationInput] = sqlmodel.Field(default_factory=list)
    ports: list[PortInput] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityInput] = sqlmodel.Field(default_factory=list)


class TypeOutput(
    TypeLastUpdateAtField,
    TypeCreatedAtField,
    TypeUnitField,
    TypeVariantField,
    TypeIconField,
    TypeDescriptionField,
    TypeNameField,
    Output,
):
    """↗️ The output of a type."""

    representations: list[RepresentationOutput] = sqlmodel.Field(default_factory=list)
    ports: list[PortOutput] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityOutput] = sqlmodel.Field(default_factory=list)


class Type(
    TypeLastUpdateAtField,
    TypeCreatedAtField,
    TypeUnitField,
    TypeVariantField,
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
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the type in the database."""
    representations: list[Representation] = sqlmodel.Relationship(
        back_populates="type",
        cascade_delete=True,
    )
    """💾 The representations of the type."""
    ports: list[Port] = sqlmodel.Relationship(
        back_populates="type", cascade_delete=True
    )
    """🔌 The ports of the type."""
    qualities: list[Quality] = sqlmodel.Relationship(
        back_populates="type", cascade_delete=True
    )
    """📏 The qualities of the type."""
    kitPk: typing.Optional[int] = sqlmodel.Field(
        # alias="kitId", # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column(
            "kitId",
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
            "name", "variant", "kitId", name="Unique name and variant"
        ),
    )

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
            qualities = [Quality.parse(q) for q in obj["qualities"]]
            entity.qualities = qualities
        except KeyError:
            pass
        return entity

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


class PieceTypeField(MaskedField, abc.ABC):
    """🧩 The type of the piece."""

    type: TypeId = sqlmodel.Field(
        description="🧩 The type of the piece.",
    )
    """🧩 The type of the piece."""


class PiecePlaneField(MaskedField, abc.ABC):
    """◳ The plane of the piece."""

    plane: Plane = sqlmodel.Field(
        description="◳ The plane of the piece.",
    )
    """◳ The plane of the piece."""


class PieceScreenPointField(MaskedField, abc.ABC):
    """📺 The screen point of the piece."""

    screenPoint: ScreenPoint = sqlmodel.Field(
        description="📺 The screen point of the piece.",
    )
    """📺 The screen point of the piece."""


class PieceId(PieceIdField, Id):
    """🪪 The props to identify the piece within the parent design."""


class PieceProps(
    PieceScreenPointField, PiecePlaneField, PieceTypeField, PieceIdField, Props
):
    """🎫 The props of a piece."""


class PieceInput(PieceTypeField, PieceIdField, Input):
    """↘️ The input for a piece."""

    plane: typing.Optional[PlaneInput] = sqlmodel.Field(
        description="◳ The plane of the piece.",
    )
    """◳ The plane of the piece."""
    screenPoint: ScreenPointInput = sqlmodel.Field(
        description="📺 The screen point of the piece.",
    )
    """📺 The screen point of the piece."""


class PieceOutput(PieceTypeField, PieceIdField, Output):
    """↗️ The output of a piece."""

    plane: typing.Optional[PlaneOutput] = sqlmodel.Field(
        default=None,
        description="◳ The plane of the piece.",
    )
    """◳ The plane of the piece."""
    screenPoint: ScreenPointOutput = sqlmodel.Field(
        description="📺 The screen point of the piece.",
    )
    """📺 The screen point of the piece."""


class Piece(TableEntity, table=True):
    """⭕ A piece is a 3d-instance of a type in a design."""

    PLURAL = "pieces"
    __tablename__ = "piece"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    """🔑 The primary key of the piece in the database."""
    id_: str = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "localId",
            sqlalchemy.String(ID_LENGTH_LIMIT),
        ),
        default="",
        exclude=True,
    )
    typePk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "typeId",
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
            "planeId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("plane.id"),
            nullable=True,
        ),
        default=None,
        exclude=True,
    )
    """🔑 The foreign primary key of the plane of the piece in the database."""
    plane: typing.Optional[Plane] = sqlmodel.Relationship(back_populates="piece")
    """◳ The plane of the piece."""
    screenPointX: int = sqlmodel.Field(exclude=True)
    """📏 The x-coordinate of the screen point of the piece."""
    screenPointY: int = sqlmodel.Field(exclude=True)
    """📏 The y-coordinate of the screen point of the piece."""
    designPk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "designId",
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

    __table_args__ = (sqlalchemy.UniqueConstraint("localId", "designId"),)

    @property
    def screenPoint(self) -> ScreenPoint:
        """↗️ Get the masked screen point of the piece."""
        return ScreenPoint(self.screenPointX, self.screenPointY)

    @screenPoint.setter
    def screenPoint(self, screenPoint: ScreenPoint):
        """↘️ Set the masked screen point of the piece."""
        self.screenPointX = screenPoint.x
        self.screenPointY = screenPoint.y

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
        screenPoint = ScreenPoint.parse(obj["screenPoint"])
        entity.screenPoint = screenPoint
        try:
            plane = Plane.parse(obj["plane"])
            # TODO: Proper mechanism of nullable fields.
            if plane.originX is not None:
                entity.plane = plane
        except KeyError:
            pass
        return entity

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


class SideInput(Side):
    pass


class SideOutput(Side):
    pass


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


class ConnectionRotationField(RealField, abc.ABC):
    """🔄 The rotation of the connection."""

    rotation: float = sqlmodel.Field(
        ge=0, lt=360, default=0, description="🔄 The rotation of the connection."
    )
    """🔄 The rotation of the connection."""


class ConnectionTiltField(RealField, abc.ABC):
    """↗️ The tilt of the connection."""

    tilt: float = sqlmodel.Field(
        ge=0, lt=360, default=0, description="↗️ The tilt of the connection."
    )
    """↗️ The tilt of the connection."""


class ConnectionGapField(RealField, abc.ABC):
    """↕️ The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece."""

    gap: float = sqlmodel.Field(
        default=0,
        description="↕️ The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece. ",
    )
    """↕️ The optional longitudinal gap (applied after rotation and tilt in port direction) between the connected and the connecting piece. """


class ConnectionShiftField(RealField, abc.ABC):
    """↔️ The optional lateral shift (applied after rotation and tilt in the plane) between the connected and the connecting piece.."""

    shift: float = sqlmodel.Field(
        default=0,
        description="↔️ The optional lateral shift (applied after rotation and tilt in the plane) between the connected and the connecting piece..",
    )
    """↔️ The optional lateral shift (applied after rotation and tilt in the plane) between the connected and the connecting piece.."""


class ConnectionId(ConnectionConnectedField, ConnectionConnectingField, Id):
    """🪪 The props to identify the connection."""


class ConnectionProps(
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionTiltField,
    ConnectionRotationField,
    Props,
):
    """🎫 The props of a connection."""


class ConnectionInput(
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionTiltField,
    ConnectionRotationField,
    Input,
):
    """↘️ The input for a connection."""

    connected: SideInput = sqlmodel.Field(
        description="🧲 The connected side of the connection."
    )
    """🧲 The connected side of the connection."""
    connecting: SideInput = sqlmodel.Field(
        description="🧲 The connecting side of the connection."
    )
    """🧲 The connecting side of the connection."""


class ConnectionOutput(
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionTiltField,
    ConnectionRotationField,
    Output,
):
    """↗️ The output of a connection."""

    connected: SideOutput = sqlmodel.Field(
        description="🧲 The connected side of the connection."
    )
    """🧲 The connected side of the connection."""
    connecting: SideOutput = sqlmodel.Field(
        description="🧲 The connecting side of the connection."
    )
    """🧲 The connecting side of the connection."""


class Connection(
    ConnectionShiftField,
    ConnectionGapField,
    ConnectionTiltField,
    ConnectionRotationField,
    TableEntity,
    table=True,
):
    """🖇️ A connection between two pieces of a design."""

    PLURAL = "connections"
    __tablename__ = "connection"

    connectedPiecePk: typing.Optional[int] = sqlmodel.Field(
        alias="connectedPieceId",
        sa_column=sqlmodel.Column(
            "connectedPieceId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("piece.id"),
            primary_key=True,
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
            "connectedPortId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("port.id"),
            primary_key=True,
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
            "connectingPieceId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("piece.id"),
            primary_key=True,
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
            "connectingPortId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("port.id"),
            primary_key=True,
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
    designPk: typing.Optional[int] = sqlmodel.Field(
        alias="designId",
        sa_column=sqlmodel.Column(
            "designId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("design.id"),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    design: "Design" = sqlmodel.Relationship(back_populates="connections")
    __table_args__ = (
        sqlalchemy.CheckConstraint(
            "connectingPieceId != connectedPieceId",
            name="noReflexiveConnection",
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
            entity.rotation = obj["rotation"]
        except KeyError:
            pass
        try:
            entity.tilt = obj["tilt"]
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
        return entity

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
    """💬 The description of the design."""

    description: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="💬 The description of the design.",
    )
    """💬 The description of the design."""


class DesignIconField(RealField, abc.ABC):
    """🖼️ The icon of the design."""

    icon: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🖼️ The icon of the design.",
    )
    """🖼️ The icon of the design."""


class DesignVariantField(RealField, abc.ABC):
    """🔀 The variant of the design."""

    variant: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="🔀 The variant of the design.",
    )
    """🔀 The variant of the design."""


class DesignUnitField(RealField, abc.ABC):
    """📏 The unit of the design."""

    unit: str = sqlmodel.Field(
        default="",
        max_length=NAME_LENGTH_LIMIT,
        description="📏 The unit of the design.",
    )
    """📏 The unit of the design."""


class DesignCreatedAtField(RealField, abc.ABC):
    """🕒 The creation date of the design."""

    createdAt: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The creation date of the design.",
    )
    """🕒 The creation date of the design."""


class DesignLastUpdateAtField(RealField, abc.ABC):
    """🕒 The last update date of the design."""

    lastUpdateAt: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The last update date of the design.",
    )
    """🕒 The last update date of the design."""


class DesignId(DesignNameField, DesignVariantField, Id):
    """🪪 The props to identify the design."""


class DesignProps(
    DesignUnitField,
    DesignVariantField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Props,
):
    """🎫 The props of a design."""


class DesignInput(
    DesignUnitField,
    DesignVariantField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Input,
):
    """↘️ The input for a design."""

    pieces: list[PieceInput] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionInput] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityInput] = sqlmodel.Field(default_factory=list)


class DesignOutput(
    DesignLastUpdateAtField,
    DesignCreatedAtField,
    DesignUnitField,
    DesignVariantField,
    DesignIconField,
    DesignDescriptionField,
    DesignNameField,
    Output,
):
    """↗️ The output of a design."""

    pieces: list[PieceOutput] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionOutput] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityOutput] = sqlmodel.Field(default_factory=list)


class Design(
    DesignLastUpdateAtField,
    DesignCreatedAtField,
    DesignUnitField,
    DesignVariantField,
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
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    pieces: list[Piece] = sqlmodel.Relationship(
        back_populates="design", cascade_delete=True
    )
    connections: list[Connection] = sqlmodel.Relationship(
        back_populates="design", cascade_delete=True
    )
    qualities: list[Quality] = sqlmodel.Relationship(
        back_populates="design", cascade_delete=True
    )
    kitPk: typing.Optional[int] = sqlmodel.Field(
        alias="kitId",
        sa_column=sqlmodel.Column(
            "kitId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("kit.id"),
        ),
        default=None,
        exclude=True,
    )
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="designs")

    __table_args__ = (sqlalchemy.UniqueConstraint("name", "variant", "kitId"),)

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
        return entity

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
    """💬 The description of the kit."""

    description: str = sqlmodel.Field(
        default="",
        max_length=DESCRIPTION_LENGTH_LIMIT,
        description="💬 The description of the kit.",
    )
    """💬 The description of the kit."""


class KitIconField(RealField, abc.ABC):
    """🖼️ The icon of the kit."""

    icon: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🖼️ The icon of the kit.",
    )
    """🖼️ The icon of the kit."""


class KitRemoteField(RealField, abc.ABC):
    """🌐 The remote of the kit."""

    remote: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🌐 The remote of the kit.",
    )
    """🌐 The remote of the kit."""


class KitHomepage(RealField, abc.ABC):
    """🌐 The homepage of the kit."""

    homepage: str = sqlmodel.Field(
        default="",
        max_length=URL_LENGTH_LIMIT,
        description="🌐 The homepage of the kit.",
    )
    """🌐 The homepage of the kit."""


class KitCreatedAtField(RealField, abc.ABC):
    """🕒 The creation date of the kit."""

    createdAt: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The creation date of the kit.",
    )
    """🕒 The creation date of the kit."""


class KitLastUpdateAtField(RealField, abc.ABC):
    """🕒 The last update date of the kit."""

    lastUpdateAt: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now,
        description="🕒 The last update date of the kit.",
    )
    """🕒 The last update date of the kit."""


class KitId(KitUriField, Id):
    """🪪 The props to identify the kit."""


class KitProps(
    KitHomepage,
    KitRemoteField,
    KitIconField,
    KitDescriptionField,
    KitNameField,
    KitUriField,
    Props,
):
    """🎫 The props of a kit."""


class KitInput(
    KitHomepage,
    KitRemoteField,
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


class KitOutput(
    KitLastUpdateAtField,
    KitCreatedAtField,
    KitHomepage,
    KitRemoteField,
    KitIconField,
    KitDescriptionField,
    KitNameField,
    KitUriField,
    Output,
):
    """↗️ The output of a kit."""

    types: list[TypeOutput] = sqlmodel.Field(
        default_factory=list, description="🧩 The types of the kit."
    )
    """🧩 The types of the kit."""
    designs: list[DesignOutput] = sqlmodel.Field(
        default_factory=list, description="🏙️ The designs of the kit."
    )
    """🏙️ The designs of the kit."""


class Kit(
    KitLastUpdateAtField,
    KitCreatedAtField,
    KitHomepage,
    KitRemoteField,
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
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
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
    design: "designs" ("/" ENCODED_STRING "," ENCODED_STRING?)?
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
        }

    def type(self, children):
        if len(children) == 0:
            return {"kind": "types"}
        return {
            "kind": "type",
            "typeName": decode(children[0].value),
            "typeVariant": (decode(children[1].value) if len(children) == 2 else ""),
        }

    def representation(self, children):
        type = children[0]
        code = {
            "typeName": type["typeName"],
            "typeVariant": type["typeVariant"],
        }
        if len(children) == 1:
            code["kind"] = "representations"
        else:
            code["kind"] = "representation"
            code["representationUrl"] = decode(children[1].value)

        return code

    def port(self, children):
        type = children[0]
        code = {
            "typeName": type["typeName"],
            "typeVariant": type["typeVariant"],
        }
        if len(children) == 1:
            code["kind"] = "ports"
        else:
            code["kind"] = "port"
            code["portUrl"] = decode(children[1].value)
        return code


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
        self: "DatabaseStore", operation: dict, input: KitInput | TypeInput
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
                            for connection in port.connections():
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
                        existingType.description = type.description
                        existingType.unit = type.unit
                        existingType.lastUpdateAt = datetime.datetime.now()
                        for usedPortId, usedPort in usedPorts.items():
                            usedPort.point = newPorts[usedPortId].point
                            usedPort.direction = newPorts[usedPortId].direction

                            for locator in list(usedPort.locators):
                                self.session.delete(locator)
                            usedPort.locators = []
                            self.session.flush()

                            newLocators = []
                            for newLocator in list(newPorts[usedPortId].locators):
                                newLocator.port = usedPort
                                self.session.add(newLocator)
                                newLocators.append(newLocator)
                            usedPort.locators = newLocators
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
        except sqlalchemy.exc.OperationalError:
            pass
        return SqliteStore(uri, engine, sqlitePath)

    def initialize(self: "DatabaseStore") -> None:
        os.makedirs(
            str(pathlib.Path(self.uri) / pathlib.Path(KIT_LOCAL_FOLDERNAME)),
            exist_ok=True,
        )
        sqlmodel.SQLModel.metadata.create_all(self.engine)

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


# Graphql #


GRAPHQLTYPES = {
    str: graphene.NonNull(graphene.String),
    int: graphene.NonNull(graphene.Int),
    float: graphene.NonNull(graphene.Float),
    bool: graphene.NonNull(graphene.Boolean),
    list[str]: graphene.NonNull(graphene.List(graphene.NonNull(graphene.String))),
    ScreenPoint: graphene.NonNull(lambda: ScreenPointNode),
    Point: graphene.NonNull(lambda: PointNode),
    Vector: graphene.NonNull(lambda: VectorNode),
    Plane: graphene.NonNull(lambda: PlaneNode),
    Representation: graphene.NonNull(lambda: RepresentationNode),
    list[Representation]: graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: RepresentationNode))
    ),
    Port: graphene.NonNull(lambda: PortNode),
    PortId: graphene.NonNull(lambda: PortNode),
    list[Port]: graphene.NonNull(graphene.List(graphene.NonNull(lambda: PortNode))),
    Quality: graphene.NonNull(lambda: QualityNode),
    list[Quality]: graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: QualityNode))
    ),
    Type: graphene.NonNull(lambda: TypeNode),
    TypeId: graphene.NonNull(lambda: TypeNode),
    list[Type]: graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    Piece: graphene.NonNull(lambda: PieceNode),
    PieceId: graphene.NonNull(lambda: PieceNode),
    Side: graphene.NonNull(lambda: SideNode),
    Connection: graphene.NonNull(lambda: ConnectionNode),
    Design: graphene.NonNull(lambda: DesignNode),
    Kit: graphene.NonNull(lambda: KitNode),
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
        if "exclude_fields" not in options:
            options["exclude_fields"] = tuple(
                k for k, v in model.__fields__.items() if v.exclude
            )
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
            setattr(cls, name, GRAPHQLTYPES[prop_return_type])
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


class RepresentationNode(TableEntityNode):
    class Meta:
        model = Representation


class RepresentationInputNode(InputNode):
    class Meta:
        model = RepresentationInput


class LocatorNode(TableNode):
    class Meta:
        model = Locator


class LocatorInputNode(InputNode):
    class Meta:
        model = LocatorInput


class ScreenPointNode(Node):
    class Meta:
        model = ScreenPoint


class ScreenPointInputNode(InputNode):
    class Meta:
        model = ScreenPointInput


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

    locators = graphene.List(graphene.NonNull(lambda: LocatorNode))

    def resolve_locators(self, info):
        return self.locators


class PortInputNode(InputNode):
    class Meta:
        model = PortInput


class PortIdInputNode(InputNode):
    class Meta:
        model = PortId


class QualityNode(TableEntityNode):
    class Meta:
        model = Quality


class QualityInputNode(InputNode):
    class Meta:
        model = QualityInput


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

    # connected = graphene.NonNull(SideNode)
    # connecting = graphene.NonNull(SideNode)

    # def resolve_connected(connection: Connection, info):
    #     return connection.connected

    # def resolve_connecting(connection: Connection, info):
    #     return connection.connecting


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

rest = fastapi.FastAPI(
    title="semio REST API", version=VERSION, max_request_body_size=MAX_REQUEST_BODY_SIZE
)


@rest.get("/kits/{encodedKitUri}")
async def kit(
    request: fastapi.Request,
    encodedKitUri: ENCODED_PATH,
) -> KitOutput:
    try:
        return get(request.url.path.removeprefix("/kits/"))
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
        put(request.url.path.removeprefix("/kits/"), input)
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
        delete(request.url.path.removeprefix("/kits/"))
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
        put(request.url.path.removeprefix("/kits/"), input)
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
        delete(request.url.path.removeprefix("/kits/"))
        return None
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.put("/kits/{encodedKitUri}/designs/{encodedDesignNameAndVariant}")
async def put_design(
    request: fastapi.Request,
    input: DesignInput,
    encodedKitUri: ENCODED_PATH,
    encodedDesignNameAndVariant: ENCODED_NAME_AND_VARIANT_PATH,
) -> None:
    try:
        put(request.url.path.removeprefix("/kits/"), input)
        return None
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.delete("/kits/{encodedKitUri}/designs/{encodedDesignNameAndVariant}")
async def delete_design(
    request: fastapi.Request,
    encodedKitUri: ENCODED_PATH,
    encodedDesignNameAndVariant: ENCODED_NAME_AND_VARIANT_PATH,
) -> None:
    try:
        delete(request.url.path.removeprefix("/kits/"))
        return None
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


# Engine #

engine = starlette.applications.Starlette()
engine.mount(
    "/graphql",
    starlette_graphene3.GraphQLApp(
        graphqlSchema, on_get=starlette_graphene3.make_graphiql_handler()
    ),
)
engine.mount("/", rest)


def start_engine(debug: bool = False):

    if debug:
        if os.path.exists("debug"):
            # delete all files and folders in debug folder
            for root, dirs, files in os.walk("debug", topdown=False):
                for name in files:
                    os.remove(os.path.join(root, name))
                for name in dirs:
                    os.rmdir(os.path.join(root, name))
        else:
            os.makedirs("debug")

        # write openapi schema to file
        with open("../../openapi/schema.json", "w", encoding="utf-8") as f:
            json.dump(rest.openapi(), f, indent=4)

        # write sqlite schema to file
        sqliteSchemaPath = "../../sqlite/schema.sql"
        if os.path.exists(sqliteSchemaPath):
            os.remove(sqliteSchemaPath)
        metadata_engine = sqlalchemy.create_engine("sqlite:///debug/semio.db")
        sqlmodel.SQLModel.metadata.create_all(metadata_engine)
        conn = sqlite3.connect("debug/semio.db")
        cursor = conn.cursor()
        cursor.execute("SELECT sql FROM sqlite_master WHERE type='table';")
        sqliteSchema = cursor.fetchall()
        with open(sqliteSchemaPath, "w", encoding="utf-8") as f:
            for table in sqliteSchema:
                f.write(f"{table[0]};\n")
        conn.close()

        # write graphql schema to file
        with open("../../graphql/schema.graphql", "w", encoding="utf-8") as f:
            f.write(str(graphqlSchema))

    logging.basicConfig(level=logging.INFO)  # for uvicorn in pyinstaller
    uvicorn.run(
        engine,
        host=HOST,
        port=PORT,
        log_level="info",
        access_log=False,
        log_config=None,
    )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--debug", action="store_true", help="Enable debug mode")
    args = parser.parse_args()
    start_engine(args.debug)


if __name__ == "__main__":
    multiprocessing.freeze_support()  # needed for pyinstaller on Windows
    main()
