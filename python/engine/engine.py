#!/usr/bin/env python

# semio.py
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
semio.py
"""


# TODOs


# TODO: Think of revertable encoding for ids.
# TODO: Automatic derive from Id model.
# TODO: Automatic emptying.
# TODO: Automatic updating based on props.
# TODO: Check how to automate docstring duplication, table=True and PLURAL and __tablename__.
# TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374


# Copilot


## Dictionary


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
# ➡️,Dr,Drn,Direction,The direction of the port.
# 🏙️,Dn,Dsn,Design,A design is a collection of pieces that are connected.
# 🏙️,Dn*,Dsns,Designs,The optional designs of the kit.
# 🚌,Dt,DTO,Data Transfer Object, The Data Transfer Object (DTO) base of the {{NAME}}.
# 🪣,Em,Emp,Empty,Empty all props and children of the {{NAME}}.
# ▢,En,Ent,Entity,An entity is a collection of properties and children.
# 🔑,FK,FKy,Foreign Key, The foreign primary key of the parent {{PARENT_NAME}} of the {{NAME}} in the database.
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
# ↕️,Of?,Ofs,Offset,The optional offset distance (applied after rotation and tilt in port direction) between the connected and the connecting piece.
# ⌱,Og,Org,Origin,The origin of the plane.
# ↗️,Ou,Out,Output,The output for a {{NAME}}.
# 👪,Pa,Par,Parent,The parent of {{NAME}}.
# ⚒️,Pr,Prs,Parse,Parse the {{NAME}} from an input.
# 🔢,Pl,Plu,Plural,The plural of the singular of the entity name.
# ⭕,Pc,Pce,Piece,A piece is a 3d-instance of a type in a design.
# ⭕,Pc+,Pces,Pieces,The pieces of the design.
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

# Imports


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

import fastapi
import graphene
import graphene_pydantic
import graphene_sqlalchemy
import lark
import pint
import pydantic
import sqlalchemy
import sqlmodel
import starlette
import starlette_graphene3
import uvicorn
import networkx
import numpy
import pytransform3d


# Type Hints #


RecursiveAnyList = typing.Any | list["RecursiveAnyList"]
"""🔁 A recursive any list is either any or a list where the items are recursive any list."""


# Constants #


RELEASE = "r24.11-1"
VERSION = "3.0.0"
HOST = "127.0.0.1"
PORT = 24111
NAME_LENGTH_LIMIT = 64
ID_LENGTH_LIMIT = 128
URL_LENGTH_LIMIT = 1024
URI_LENGTH_LIMIT = 4 * URL_LENGTH_LIMIT
MAX_TAGS = 16
MAX_HIERARCHY = 16
DESCRIPTION_LENGTH_LIMIT = 4096
ENCODING_REGEX = r"[a-zA-ZZ0-9_-]+={0,2}"
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
MAX_REQUEST_BODY_SIZE = 50 * 1024 * 1024  # 50MB
ENVS = {key: value for key, value in os.environ.items() if key.startswith("SEMIO_")}

ureg = pint.UnitRegistry()


# Utility


def encode(value: str) -> str:
    """ᗒ Encode a string to a base64 url-safe (uses - and _ instead of + and /)."""
    encoded_bytes = base64.urlsafe_b64encode(value.encode("utf-8"))
    encoded_str = encoded_bytes.decode("utf-8")
    return encoded_str


def decode(value: str) -> str:
    """ᗕ Decode a base64 url-safe (uses - and _ instead of + and /) string."""
    value += "=" * (-len(value) % 4)
    decoded_bytes = base64.urlsafe_b64decode(value.encode("utf-8"))
    decoded_str = decoded_bytes.decode("utf-8")
    return decoded_str


def encodeList(list: list[str]) -> str:
    return ",".join([encode(t) for t in list])


def decodeList(encodedList: str) -> list[str]:
    return [decode(t) for t in encodedList.split(",")]


def encodeRecursiveAnyList(recursiveAnyList: RecursiveAnyList) -> str:
    """🆔 Encode a `RecursiveAnyList` to a base64 url-safe (uses - and _ instead of + and /) string."""
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


class TypeNotFound(NotFound):

    def __init__(self, name: str, variant: str = "") -> None:
        self.name = name
        self.variant = variant

    def __str__(self):
        variant = f", {self.variant}" if self.variant else ""
        return f"🔍 Couldn't find the type ({self.name}{variant})."


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
        return f"🔍 Couldn't find a local or remote kit under uri:\n {self.uri}."


class NoKitToDelete(KitNotFound):

    def __init__(self, uri: str) -> None:
        self.uri = uri

    def __str__(self):
        return f"🔍 Couldn't delete the kit because no local or remote kit was found under uri:\n {self.uri}."


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
    def parse(cls, input: str | dict | typing.Any) -> "Model":
        """⚒️ Parse the entity from an input."""
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
    RepresentationMimeField, RepresentationLodField, RepresentationTagsField, Id
):
    """🪪 The props to identify the representation within the parent type."""


class RepresentationProps(
    RepresentationMimeField,
    RepresentationLodField,
    RepresentationTagsField,
    RepresentationUrlField,
    Props,
):
    """🎫 The props of a representation."""


class RepresentationInput(
    RepresentationMimeField,
    RepresentationLodField,
    RepresentationTagsField,
    RepresentationUrlField,
    Input,
):
    """↘️ The input for a representation."""


class RepresentationOutput(
    RepresentationMimeField,
    RepresentationLodField,
    RepresentationTagsField,
    RepresentationUrlField,
    Output,
):
    """↗️ The output of a representation."""


class Representation(
    RepresentationMimeField,
    RepresentationLodField,
    RepresentationUrlField,
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
        cls: "Representation", input: str | dict | typing.Any
    ) -> "Representation":
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


class LocatorProps(LocatorGroupField, LocatorSubgroupField, Props):
    """🎫 The props of a locator."""


class LocatorInput(LocatorGroupField, LocatorSubgroupField, Input):
    """↘️ The input for a locator."""


class LocatorOutput(LocatorGroupField, LocatorSubgroupField, Output):
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


class PlaneInput(PlaneOriginField, PlaneXAxisField, PlaneYAxisField, Input):
    """↘️ The input for a plane."""


class PlaneOutput(PlaneOriginField, PlaneXAxisField, PlaneYAxisField, Output):
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
    # piece: typing.Optional["Piece"] = sqlmodel.Relationship(back_populates="plane")
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

    @property
    def zAxis(self) -> Vector:
        return self.xAxis.cross(self.yAxis)

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
        max_length=NAME_LENGTH_LIMIT,
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
    PortIdField, PortPointField, PortDirectionField, PortLocatorsField, Props
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
            sqlalchemy.String(NAME_LENGTH_LIMIT),
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
    # connecteds: list["Connection"] = sqlmodel.Relationship(
    #     back_populates="connectedPieceTypePort",
    #     sa_relationship_kwargs={"foreign_keys": "Connection.connectedPieceTypePortPk"},
    # )
    # connectings: list["Connection"] = sqlmodel.Relationship(
    #     back_populates="connectingPieceTypePort",
    #     sa_relationship_kwargs={"foreign_keys": "Connection.connectingPieceTypePortPk"},
    # )

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

    def parent(self) -> "Type":
        """👪 The parent type of the port or otherwise `NoTypeAssigned` is raised."""
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        """🪪 The members that form the id of the port within its parent type."""
        return self.id_

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Port", input: str | dict | typing.Any) -> "Port":
        """🧪 Parse the input to a port."""
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
        point = Point(**obj["point"])
        direction = Vector(**obj["direction"])
        entity = cls(id_=obj["id_"])
        entity.point = point
        entity.direction = direction
        try:
            locators = [Locator.parse(l) for l in obj["locators"]]
            entity.locators = locators
        except KeyError:
            pass
        return entity


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
    QualityNameField,
    QualityValueField,
    QualityDefinitionField,
    QualityUnitField,
    Props,
):
    """🎫 The props of a quality."""


class QualityInput(
    QualityNameField, QualityValueField, QualityDefinitionField, QualityUnitField, Input
):
    """↘️ The input for a quality."""


class QualityOutput(
    QualityNameField,
    QualityValueField,
    QualityDefinitionField,
    QualityUnitField,
    Output,
):
    """↗️ The output of a quality."""


class Quality(
    QualityNameField,
    QualityValueField,
    QualityDefinitionField,
    QualityUnitField,
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

    def parent(self) -> "Type":
        """👪 The parent type of the quality or otherwise `NoTypeAssigned` is raised."""
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

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


class TypeId(TypeNameField, TypeVariantField, Id):
    """🪪 The props to identify the type."""


class TypeProps(
    TypeNameField,
    TypeDescriptionField,
    TypeIconField,
    TypeVariantField,
    TypeUnitField,
    Props,
):
    """🎫 The props of a type."""


class TypeInput(
    TypeNameField,
    TypeDescriptionField,
    TypeIconField,
    TypeVariantField,
    TypeUnitField,
    Input,
):
    """↘️ The input for a type."""

    representations: list[RepresentationInput] = sqlmodel.Field(default_factory=list)
    ports: list[PortInput] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityInput] = sqlmodel.Field(default_factory=list)


class TypeOutput(
    TypeNameField,
    TypeDescriptionField,
    TypeIconField,
    TypeVariantField,
    TypeUnitField,
    Output,
):
    """↗️ The output of a type."""

    representations: list[RepresentationOutput] = sqlmodel.Field(default_factory=list)
    ports: list[PortOutput] = sqlmodel.Field(default_factory=list)
    qualities: list[QualityOutput] = sqlmodel.Field(default_factory=list)


class Type(
    TypeNameField,
    TypeDescriptionField,
    TypeIconField,
    TypeVariantField,
    TypeUnitField,
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
    def parse(cls: "Type", input: str | dict | typing.Any) -> "Type":
        """🧪 Parse the input to a type."""
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
        return [self.name, self.variant]


### Pieces ###


# class PieceIdField(MaskedField, abc.ABC):
#     """🆔 The id of the piece."""

#     id_: str = sqlmodel.Field(
#         default="",
#         # alias="id", # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
#         max_length=NAME_LENGTH_LIMIT,
#         description="🆔 The id of the piece.",
#     )
#     """🆔 The id of the piece."""


# class PieceTypeField(MaskedField, abc.ABC):
#     """🧩 The type of the piece."""

#     type: TypeId = sqlmodel.Field(
#         description="🧩 The type of the piece.",
#     )
#     """🧩 The type of the piece."""


# class PiecePlaneField(MaskedField, abc.ABC):
#     """◳ The plane of the piece."""

#     plane: Plane = sqlmodel.Field(
#         description="◳ The plane of the piece.",
#     )
#     """◳ The plane of the piece."""


# class PieceScreenPointField(MaskedField, abc.ABC):
#     """📺 The screen point of the piece."""

#     screenPoint: ScreenPoint = sqlmodel.Field(
#         description="📺 The screen point of the piece.",
#     )
#     """📺 The screen point of the piece."""


# class PieceId(PieceIdField, Id):
#     """🪪 The props to identify the piece within the parent design."""


# class PieceProps(
#     PieceIdField, PieceTypeField, PiecePlaneField, PieceScreenPointField, Props
# ):
#     """🎫 The props of a piece."""


# class PieceInput(PieceIdField, PieceTypeField, Input):
#     """↘️ The input for a piece."""

#     plane: PlaneInput = sqlmodel.Field(
#         description="◳ The plane of the piece.",
#     )
#     """◳ The plane of the piece."""
#     screenPoint: ScreenPointInput = sqlmodel.Field(
#         description="📺 The screen point of the piece.",
#     )
#     """📺 The screen point of the piece."""


# class PieceOutput(PieceIdField, PieceTypeField, Output):
#     """↗️ The output of a piece."""

#     plane: PlaneOutput = sqlmodel.Field(
#         description="◳ The plane of the piece.",
#     )
#     """◳ The plane of the piece."""
#     screenPoint: ScreenPointOutput = sqlmodel.Field(
#         description="📺 The screen point of the piece.",
#     )
#     """📺 The screen point of the piece."""


# class Piece(TableEntity, table=True):
#     """⭕ A piece is a 3d-instance of a type in a design."""

#     PLURAL = "pieces"
#     __tablename__ = "piece"
#     pk: typing.Optional[int] = sqlmodel.Field(
#         sa_column=sqlmodel.Column(
#             "id",
#             sqlalchemy.Integer(),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     """🔑 The primary key of the piece in the database."""
#     id_: str = sqlmodel.Field(
#         sa_column=sqlmodel.Column(
#             "localId",
#             sqlalchemy.String(NAME_LENGTH_LIMIT),
#         ),
#         default="",
#         exclude=True,
#     )
#     """🆔 The id of the piece within the design."""
#     planePk: typing.Optional[int] = sqlmodel.Field(
#         sa_column=sqlmodel.Column(
#             "planeId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("plane.id"),
#         ),
#         default=None,
#         exclude=True,
#     )
#     """🔑 The foreign primary key of the plane of the piece in the database."""
#     plane: typing.Optional[Plane] = sqlmodel.Relationship(back_populates="piece")
#     """◳ The plane of the piece."""
#     screenPointX: int = sqlmodel.Field(exclude=True)
#     """📏 The x-coordinate of the screen point of the piece."""
#     screenPointY: int = sqlmodel.Field(exclude=True)
#     """📏 The y-coordinate of the screen point of the piece."""
#     designPk: typing.Optional[int] = sqlmodel.Field(
#         sa_column=sqlmodel.Column(
#             "designId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("design.id"),
#         ),
#         default=None,
#         exclude=True,
#     )
#     """🔑 The foreign primary key of the parent design of the piece in the database."""
#     design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="pieces")
#     """👪 The parent design of the piece."""
#     connecteds: list["Connection"] = sqlmodel.Relationship(
#         back_populates="connectedPiece",
#         sa_relationship_kwargs={"foreign_keys": "Connection.connectedPiecePk"},
#     )
#     """🖇️ The connections where the piece is the connected to another piece."""
#     connectings: list["Connection"] = sqlmodel.Relationship(
#         back_populates="connectingPiece",
#         sa_relationship_kwargs={"foreign_keys": "Connection.connectingPiecePk"},
#     )
#     """🖇️ The connections where the piece is the connecting from another piece."""

#     __table_args__ = (sqlalchemy.UniqueConstraint("localId", "designId"),)

#     @property
#     def screenPoint(self) -> ScreenPoint:
#         return ScreenPoint(self.screenPointPointX, self.screenPointPointY)

#     @screenPoint.setter
#     def screenPoint(self, screenPoint: ScreenPoint):
#         self.screenPointPointX = screenPoint.x
#         self.screenPointPointY = screenPoint.y

#     def parent(self) -> "Design":
#         if self.design is None:
#             raise NoParentAssigned()
#         return self.design


# class SidePieceType(sqlmodel.SQLModel):
#     port: Port = sqlmodel.Field()


# class SidePieceTypeOutput(sqlmodel.SQLModel):
#     port: PortIdOutput = sqlmodel.Field()


# class SidePiece(sqlmodel.SQLModel):
#     id_: str = sqlmodel.Field(alias="id")
#     type: SidePieceType = sqlmodel.Field()


# class SidePieceOutput(sqlmodel.SQLModel):
#     id_: str = sqlmodel.Field(alias="id")
#     type: SidePieceTypeOutput = sqlmodel.Field()


# class Side(sqlmodel.SQLModel):
#     piece: SidePiece = sqlmodel.Field()


# class SideOutput(sqlmodel.SQLModel):
#     piece: SidePieceOutput = sqlmodel.Field()


# class ConnectionBase(sqlmodel.SQLModel):
#     rotation: float = sqlmodel.Field(ge=0, lt=360)
#     offset: float = sqlmodel.Field()


# class Connection(ConnectionBase, TableEntity, table=True):
#     """🖇️ A connection between two pieces of a design."""

#     PLURAL = "connections"
#     __tablename__ = "connection"

#     connectedPiecePk: typing.Optional[int] = sqlmodel.Field(
#         alias="connectedPieceId",
#         sa_column=sqlmodel.Column(
#             "connectedPieceId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("piece.id"),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     connectedPiece: Piece = sqlmodel.Relationship(
#         sa_relationship=sqlalchemy.orm.relationship(
#             "Piece",
#             back_populates="connecteds",
#             foreign_keys="[Connection.connectedPiecePk]",
#         )
#     )
#     connectedPieceTypePortPk: typing.Optional[int] = sqlmodel.Field(
#         alias="connectedPieceTypePortId",
#         sa_column=sqlmodel.Column(
#             "connectedPieceTypePortId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("port.id"),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     connectedPieceTypePort: Port = sqlmodel.Relationship(
#         sa_relationship=sqlalchemy.orm.relationship(
#             "Port",
#             back_populates="connecteds",
#             foreign_keys="[Connection.connectedPieceTypePortPk]",
#         )
#     )
#     connectingPiecePk: typing.Optional[int] = sqlmodel.Field(
#         alias="connectingPieceId",
#         sa_column=sqlmodel.Column(
#             "connectingPieceId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("piece.id"),
#             primary_key=True,
#         ),
#         exclude=True,
#         default=None,
#     )
#     connectingPiece: Piece = sqlmodel.Relationship(
#         sa_relationship=sqlalchemy.orm.relationship(
#             "Piece",
#             back_populates="connectings",
#             foreign_keys="[Connection.connectingPiecePk]",
#         )
#     )
#     connectingPieceTypePortPk: typing.Optional[int] = sqlmodel.Field(
#         alias="connectingPieceTypePortId",
#         sa_column=sqlmodel.Column(
#             "connectingPieceTypePortId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("port.id"),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     connectingPieceTypePort: Port = sqlmodel.Relationship(
#         sa_relationship=sqlalchemy.orm.relationship(
#             "Port",
#             back_populates="connectings",
#             foreign_keys="[Connection.connectingPieceTypePortPk]",
#         )
#     )
#     designPk: typing.Optional[int] = sqlmodel.Field(
#         alias="designId",
#         sa_column=sqlmodel.Column(
#             "designId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("design.id"),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     design: "Design" = sqlmodel.Relationship(back_populates="connections")
#     __table_args__ = (
#         sqlalchemy.CheckConstraint(
#             "connectingPieceId != connectedPieceId",
#             name="noReflexiveConnection",
#         ),
#     )

#     @property
#     def connected(self) -> Side:
#         return Side(
#             piece=SidePiece(
#                 id_=self.connectedPiece.id,
#                 type=SidePieceType(
#                     port=PortIdOutput(id=self.connectedPieceTypePort.id_)
#                 ),
#             )
#         )

#     @property
#     def connecting(self) -> Side:
#         return Side(
#             piece=SidePiece(
#                 id_=self.connectingPiece.id,
#                 type=SidePieceType(
#                     port=PortIdOutput(id=self.connectingPieceTypePort.id_)
#                 ),
#             )
#         )


# class ConnectionOutput(ConnectionBase):
#     class Config:
#         title = "Connection"

#     connected: Side = sqlmodel.Field()
#     connecting: Side = sqlmodel.Field()


# class DesignBase(VariableArtifact):
#     pass


# class Design(DesignBase, TableEntity, table=True):
#     """🏙️ A design is a collection of pieces that are connected."""

#     PLURAL = "designs"
#     __tablename__ = "design"
#     pk: typing.Optional[int] = sqlmodel.Field(
#         sa_column=sqlmodel.Column(
#             "id",
#             sqlalchemy.Integer(),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     pieces: list[Piece] = sqlmodel.Relationship(
#         back_populates="design", cascade_delete=True
#     )
#     connections: list[Connection] = sqlmodel.Relationship(
#         back_populates="design", cascade_delete=True
#     )
#     qualities: list[Quality] = sqlmodel.Relationship(
#         back_populates="design", cascade_delete=True
#     )
#     kitPk: typing.Optional[int] = sqlmodel.Field(
#         alias="kitId",
#         sa_column=sqlmodel.Column(
#             "kitId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("kit.id"),
#         ),
#         default=None,
#         exclude=True,
#     )
#     kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="designs")

#     # __table_args__ = (sqlalchemy.UniqueConstraint('name', 'variant', 'kitPk'),)

#     def parent(self) -> "Kit":
#         if self.kit is None:
#             raise NoKitAssigned()
#         return self.kit

#     def getByLocalId(
#         self, session: sqlalchemy.orm.Session, localId: tuple, decode: bool = False
#     ) -> "Design":
#         return (
#             session.query(Design)
#             .filter(Design.name == decode(localId[0]) if decode else localId[0])
#             .first()
#         )


# class DesignOutput(DesignBase):
#     class Config:
#         title = "Design"

#     pieces: list[PieceOutput] = sqlmodel.Field(default_factory=list)
#     connections: list[ConnectionOutput] = sqlmodel.Field(default_factory=list)


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
    KitUriField,
    KitNameField,
    KitDescriptionField,
    KitIconField,
    KitRemoteField,
    KitHomepage,
    Props,
):
    """🎫 The props of a kit."""


class KitInput(
    KitNameField,
    KitDescriptionField,
    KitIconField,
    KitRemoteField,
    KitHomepage,
    Input,
):
    """↘️ The input for a kit."""

    types: list[TypeInput] = sqlmodel.Field(
        default_factory=list, description="🧩 The types of the kit."
    )
    """🧩 The types of the kit."""


class KitOutput(
    KitUriField,
    KitNameField,
    KitDescriptionField,
    KitIconField,
    KitRemoteField,
    KitHomepage,
    KitCreatedAtField,
    KitLastUpdateAtField,
    Output,
):
    """↗️ The output of a kit."""

    types: list[TypeOutput] = sqlmodel.Field(
        default_factory=list, description="🧩 The types of the kit."
    )
    """🧩 The types of the kit."""


class Kit(
    KitUriField,
    KitNameField,
    KitDescriptionField,
    KitIconField,
    KitRemoteField,
    KitHomepage,
    KitCreatedAtField,
    KitLastUpdateAtField,
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
    # designs: list[Design] = sqlmodel.Relationship(
    #     back_populates="kit", cascade_delete=True
    # )

    __table_args__ = (sqlalchemy.UniqueConstraint("uri"),)

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Kit", input: str | dict | KitInput | typing.Any) -> "Kit":
        """🧪 Parse the input to a kit."""
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


codeGrammar = r"""
    code: (ENCODED_STRING)? ("/" (type | representation | port))?
    type: "types" ("/" ENCODED_STRING "," ENCODED_STRING?)?
    representation: type "/representations" ("/" ENCODED_STRING)?
    port: type "/ports" ("/" ENCODED_STRING)?
    ENCODED_STRING: /[a-zA-ZZ0-9_-]+={0,2}/
"""
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

    @classmethod
    @abc.abstractmethod
    def fromUri(cls: "DatabaseStore", uri: str) -> "DatabaseStore":
        """🔧 Get a store from the uri."""
        pass

    def get(self: "DatabaseStore", operation: dict) -> typing.Any:
        kitUri = operation["kitUri"]
        kind = operation["kind"]
        kit = self.session.query(Kit).filter(Kit.uri == kitUri).one_or_none()
        if kit is None:
            raise KitNotFound(kitUri)
        match kind:
            case "kit":
                return kit
            case "type":
                return (
                    self.session.Query(Type, Kit)
                    .filter(
                        Kit.uri == kitUri,
                        Type.name == operation["typeName"],
                        Type.variant == operation["typeVariant"],
                    )
                    .one_or_none()
                )
            case _:
                raise FeatureNotYetSupported()

    def put(
        self: "DatabaseStore", operation: dict, input: KitInput | TypeInput
    ) -> typing.Any:
        kitUri = operation["kitUri"]
        kind = operation["kind"]
        match kind:
            case "kit":
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
            case _:
                raise FeatureNotYetSupported()

    def update(self: "DatabaseStore", operation: dict, input: str) -> typing.Any:
        raise FeatureNotYetSupported()

    def delete(self: "DatabaseStore", operation: dict) -> typing.Any:
        kitUri = operation["kitUri"]
        kind = operation["kind"]
        kit = self.session.query(Kit).filter(Kit.uri == kitUri).one_or_none()
        if kit is None:
            raise NoKitToDelete(kitUri)
        match kind:
            case "kit":
                try:
                    self.session.delete(kit)
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


class SqliteStore(DatabaseStore):
    path: pathlib.Path

    def __init__(
        self, uri: str, engine: sqlalchemy.engine.Engine, path: pathlib.Path
    ) -> None:
        super().__init__(uri, engine)
        self.path = path

    @classmethod
    def fromUri(cls, uri: str) -> "SqliteStore":
        if not os.path.isabs(uri):
            raise LocalKitUriIsNotAbsolute(uri)  # Currently unreachable
        path = (
            pathlib.Path(uri)
            / pathlib.Path(KIT_LOCAL_FOLDERNAME)
            / pathlib.Path(KIT_LOCAL_FILENAME)
        )
        connectionString = f"sqlite:///{path}"
        engine = sqlalchemy.create_engine(connectionString, echo=True)
        return SqliteStore(uri, engine, path)

    def kitNameByUri(self: "SqliteStore", uri: str) -> str | None:
        kit = self.session.query(Kit).one_or_none()
        return kit.name if kit is not None else None

    def initialize(self: "DatabaseStore") -> None:
        os.makedirs(
            str(pathlib.Path(self.uri) / pathlib.Path(KIT_LOCAL_FOLDERNAME)),
            exist_ok=True,
        )
        sqlmodel.SQLModel.metadata.create_all(self.engine)


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

    @classmethod
    def kitNameByUri(self: "PostgresStore", uri: str) -> str | None:
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
        raise RemoteKitsNotYetSupported()
    raise LocalKitUriIsNotAbsolute(uri)


def storeAndOperationFromCode(code: str) -> tuple[Store, dict]:
    codeTree = codeParser.parse(code)
    operation = OperationBuilder().transform(codeTree)
    store = StoreFactory(operation["kitUri"])
    return store, operation


def get(code: str) -> typing.Any:
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
    Point: graphene.NonNull(lambda: PointNode),
    Vector: graphene.NonNull(lambda: VectorNode),
    Representation: graphene.NonNull(lambda: RepresentationNode),
    list[Representation]: graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: RepresentationNode))
    ),
    Port: graphene.NonNull(lambda: PortNode),
    list[Port]: graphene.NonNull(graphene.List(graphene.NonNull(lambda: PortNode))),
    Quality: graphene.NonNull(lambda: QualityNode),
    list[Quality]: graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: QualityNode))
    ),
    Type: graphene.NonNull(lambda: TypeNode),
    list[Type]: graphene.NonNull(graphene.List(graphene.NonNull(lambda: TypeNode))),
    # Plane: graphene.NonNull(lambda: PlaneNode),
    # PieceDiagram: graphene.NonNull(lambda: PieceDiagramNode),
    # SidePieceType: graphene.NonNull(lambda: SidePieceTypeNode),
    # SidePiece: graphene.NonNull(lambda: SidePieceNode),
    # Side: graphene.NonNull(lambda: SideNode),
    # Connection: graphene.NonNull(lambda: ConnectionNode),
    # Design: graphene.NonNull(lambda: DesignNode),
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


# class ScreenPointNode(Node):
#     class Meta:
#         model = ScreenPoint


# class ScreenPointInputNode(InputNode):
#     class Meta:
#         model = ScreenPointInput


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


# class PlaneNode(TableNode):
#     class Meta:
#         model = Plane


# class PlaneInputNode(InputNode):
#     class Meta:
#         model = PlaneInput


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


# class PieceNode(TableEntityNode):
#     class Meta:
#         model = Piece

#     coordinateSystem = graphene.NonNull(PlaneNode)
#     diagram = graphene.NonNull(PieceDiagramNode)

#     def resolve_coordinateSystem(self, info):
#         return self.coordinateSystem

#     def resolve_diagram(self, info):
#         return self.diagram


# class PieceInputNode(InputNode):
#     class Meta:
#         model = PieceInput


# class SidePieceTypeNode(Node):
#     class Meta:
#         model = SidePieceType
#         # port is none Pydanctic model and needs to be resolved manually
#         exclude_fields = ("port",)

#     port = graphene.Field(PortNode)

#     def resolve_port(type: SidePieceType, info):
#         return type.port


# class SidePieceTypeInputNode(InputNode):
#     class Meta:
#         model = SidePieceTypeInput


# class SidePieceNode(Node):
#     class Meta:
#         model = SidePiece


# class SidePieceInputNode(InputNode):
#     class Meta:
#         model = SidePieceInput


# class SideNode(Node):
#     class Meta:
#         model = Side


# class SideInputNode(InputNode):
#     class Meta:
#         model = SideInput


# class ConnectionNode(TableEntityNode):
#     class Meta:
#         model = Connection

#     connected = graphene.NonNull(SideNode)
#     connecting = graphene.NonNull(SideNode)

#     def resolve_connected(self, info):
#         return self.connected

#     def resolve_connecting(self, info):
#         return self.connecting


# class ConnectionInputNode(InputNode):
#     class Meta:
#         model = ConnectionInput


# class DesignInputNode(InputNode):
#     class Meta:
#         model = DesignInput


# class DesignNode(TableEntityNode):
#     class Meta:
#         model = Design


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
    encodedKitUri: ENCODED_PATH,
    input: KitInput,
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


@rest.put("/kits/{encodedKitUri}/types/{encodedTypeName},{encodedTypeVariant}")
async def put_type(
    request: fastapi.Request,
    encodedKitUri: ENCODED_PATH,
    encodedTypeName: ENCODED_PATH,
    encodedTypeVariant: ENCODED_PATH,
    input: TypeInput,
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


@rest.delete("/kits/{encodedKitUri}/types/{encodedTypeName},{encodedTypeVariant}")
async def delete_type(
    request: fastapi.Request,
    encodedKitUri: ENCODED_PATH,
    encodedTypeName: ENCODED_PATH,
    encodedTypeVariant: ENCODED_PATH,
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
