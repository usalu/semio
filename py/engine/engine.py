#!/usr/bin/env python

# region Header

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

from __future__ import annotations

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
import datetime
import difflib
import enum
import functools
import inspect
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
import graphene_pydantic
import graphene_sqlalchemy
import jinja2
import lark
import loguru
import openai
import pydantic
import PySide6.QtCore
import PySide6.QtGui
import PySide6.QtWidgets
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
MAX_REQUEST_BODY_SIZE = 50 * 1024 * 1024  # 50MB
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


# I would just have to prove Applicative <=>. I miss you Haskell (。﹏。)
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
        return "🔜 Remote kits are not yet supported."


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


class NoRepresentationOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned(NoRepresentationAssigned, NoTypeAssigned, NoDesignAssigned, NoKitAssigned):
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


# Composition over inheritance. Literally.


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
    # @abc.abstractmethod
    def empty(self) -> "Entity":
        """🪣 Empty all props and children of the entity."""
        return self.__class__()

    # TODO: Automatic updating based on props.
    # @abc.abstractmethod
    def update(self, other: "Entity") -> "Entity":
        """🔄 Update the props of the entity."""
        return self
        return self


class Table(Model, abc.ABC):
    """▦ The base for tables. All resources that are stored in the database."""


class TableEntity(Entity, Table, abc.ABC):
    """▢ The base for table entities."""

    __tablename__: typing.ClassVar[str]
    """📛 The lowercase name of the table in the database."""


# endregion Primitives

# region Domain

# region Attribute
# https://github.com/usalu/semio-attribute-


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
    representationPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("representation_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("representations.id")), default=None, exclude=True)
    representation: typing.Optional["Representation"] = sqlmodel.Relationship(back_populates="attributes")
    portPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("port_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("ports.id")), default=None, exclude=True)
    port: typing.Optional["Port"] = sqlmodel.Relationship(back_populates="attributes")
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id")), default=None, exclude=True)
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="attributes")
    piecePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("pieces.id")), default=None, exclude=True)
    piece: typing.Optional["Piece"] = sqlmodel.Relationship(back_populates="attributes")
    connectionPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("connection_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("connections.id")), default=None, exclude=True)
    connection: typing.Optional["Connection"] = sqlmodel.Relationship(back_populates="attributes")
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="attributes")
    kitPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="attributes")
    qualityPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("quality_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("qualities.id")), default=None, exclude=True)
    quality: typing.Optional["Quality"] = sqlmodel.Relationship(back_populates="attributes")
    propPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("prop_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("props.id")), default=None, exclude=True)
    prop: typing.Optional["Prop"] = sqlmodel.Relationship(back_populates="attributes")
    authorPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("author_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("authors.id")), default=None, exclude=True)
    author: typing.Optional["Author"] = sqlmodel.Relationship(back_populates="attributes")
    locationPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("location_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("locations.id")), default=None, exclude=True)
    location: typing.Optional["Location"] = sqlmodel.Relationship(back_populates="attributes")
    benchmarkPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("benchmark_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("benchmarks.id")), default=None, exclude=True)
    benchmark: typing.Optional["Benchmark"] = sqlmodel.Relationship(back_populates="attributes")

    __table_args__ = (
        sqlalchemy.CheckConstraint(
            """
        (
            (representation_id IS NOT NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NOT NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NOT NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NOT NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NOT NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NOT NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NOT NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NOT NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NOT NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NOT NULL AND location_id IS NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NOT NULL AND benchmark_id IS NULL)
        OR
            (representation_id IS NULL AND port_id IS NULL AND type_id IS NULL AND piece_id IS NULL AND connection_id IS NULL AND design_id IS NULL AND kit_id IS NULL AND quality_id IS NULL AND prop_id IS NULL AND author_id IS NULL AND location_id IS NULL AND benchmark_id IS NOT NULL)
        )
        """,
            name="ck_attributes_parent_set",
        ),
        sqlalchemy.UniqueConstraint("name", "type_id", "design_id", name="uq_attributes_name_type_id_design_id"),
    )

    def parent(self) -> typing.Union["Representation", "Port", "Type", "Piece", "Connection", "Design", "Kit", "Quality", "Prop", "Author", "Location", "Benchmark", None]:
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
        raise NoRepresentationOrPortOrTypeOrPieceOrConnectionOrDesignOrKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.name


# endregion Attribute

# region Tag
# https://github.com/usalu/semio-tag-


class TagNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class TagOrderField(RealField, abc.ABC):
    order: int = sqlmodel.Field(default=0)


class Tag(TagOrderField, TagNameField, Table, table=True):
    __tablename__ = "tags"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    representationPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("representation_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("representations.id")), default=None, exclude=True)
    representation: typing.Optional["Representation"] = sqlmodel.Relationship(back_populates="tags_")


# endregion Tag

# region Concept


class Concept(TagOrderField, TagNameField, Table, table=True):
    __tablename__ = "concepts"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id")), default=None, exclude=True)
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="concepts_")
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="concepts_")
    design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="concepts_")


# endregion Concept

# region Representation
# https://github.com/usalu/semio-representation-


class RepresentationUrlField(RealField, abc.ABC):
    url: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT)


class RepresentationDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class RepresentationTagsField(MaskedField, abc.ABC):
    tags: list[str] = sqlmodel.Field(default_factory=list)


class RepresentationId(RepresentationTagsField, Id):
    pass


class RepresentationProps(RepresentationTagsField, RepresentationDescriptionField, RepresentationUrlField, Props):
    pass


class RepresentationInput(RepresentationTagsField, RepresentationDescriptionField, RepresentationUrlField, Input):
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)


class RepresentationContext(RepresentationTagsField, RepresentationDescriptionField, Context):
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)


class RepresentationOutput(RepresentationTagsField, RepresentationDescriptionField, RepresentationUrlField, Output):
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)


class Representation(RepresentationDescriptionField, RepresentationUrlField, TableEntity, table=True):
    PLURAL = "representations"
    __tablename__ = "representations"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    tags_: list[Tag] = sqlmodel.Relationship(back_populates="representation", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="representation", cascade_delete=True)
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("type.id")), default=None, exclude=True)
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="representations")

    @property
    def tags(self: "Representation") -> list[str]:
        return [tag.name for tag in sorted(self.tags_, key=lambda x: x.order)]

    @tags.setter
    def tags(self: "Representation", tags: list[str]):
        self.tags_ = [Tag(name=tag, order=i) for i, tag in enumerate(tags)]

    def parent(self: "Representation") -> "Type":
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls, input: str | dict | RepresentationInput | typing.Any | None) -> "Representation":
        if input is None:
            return cls(url="")
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        props = RepresentationProps.model_validate(obj)
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

    def dump(self) -> "RepresentationOutput":
        entity = {**RepresentationProps.model_validate(self).model_dump()}
        #  TODO: Fix bug with tags not being dumped correctly.
        # Probably some sqlmodel issue with transient objects that are never written to the database.
        # 'str' object has no attribute 'order'
        # entity["tags"] = self.tags
        entity["attributes"] = [q.dump() for q in self.attributes]
        return RepresentationOutput(**entity)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return [self.tags]


# endregion Representation

# region Coord
# https://github.com/usalu/semio-coord-


class Coord(Model):
    x: float = sqlmodel.Field()
    y: float = sqlmodel.Field()

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


class CoordInput(Coord, Input):
    pass


class CoordContext(Coord, Context):
    pass


class CoordOutput(Coord, Output):
    pass


class CoordPrediction(Coord, Prediction):
    pass


# endregion Coord

# region Point
# https://github.com/usalu/semio-point-


class Point(Model):
    x: float = sqlmodel.Field()
    y: float = sqlmodel.Field()
    z: float = sqlmodel.Field()

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
    pass


class PointContext(Point, Context):
    pass


class PointOutput(Point, Output):
    pass


# endregion Point

# region Vector
# https://github.com/usalu/semio-vector-


class Vector(Model):
    x: float = sqlmodel.Field()
    y: float = sqlmodel.Field()
    z: float = sqlmodel.Field()

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
    pass


class VectorContext(Vector, Context):
    pass


class VectorOutput(Vector, Output):
    pass


# endregion Vector

# region Plane
# https://github.com/usalu/semio-plane-


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
    piece: typing.Optional["Piece"] = sqlmodel.Relationship(back_populates="plane")

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


# endregion Plane

# region CompatibleFamily
# https://github.com/usalu/semio-compatiblefamily-


class CompatibleFamilyNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class CompatibleFamilyOrderField(RealField, abc.ABC):
    order: int = sqlmodel.Field()


class CompatibleFamily(CompatibleFamilyOrderField, CompatibleFamilyNameField, Table, table=True):
    __tablename__ = "compatible_families"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    portPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("port_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("ports.id")), default=None, exclude=True)
    port: typing.Optional["Port"] = sqlmodel.Relationship(back_populates="compatibleFamilies_")


# endregion CompatibleFamily

# region Port
# https://github.com/usalu/semio-port-


class PortIdField(MaskedField, abc.ABC):
    id_: str = sqlmodel.Field(default="", max_length=ID_LENGTH_LIMIT)


class PortDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class PortMandatoryField(RealField, abc.ABC):
    is_mandatory: bool = sqlmodel.Field(default=False)


class PortFamilyField(RealField, abc.ABC):
    family: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class PortCompatibleFamiliesField(MaskedField, abc.ABC):
    compatibleFamilies: list[str] = sqlmodel.Field(default_factory=list)


class PortPointField(MaskedField, abc.ABC):
    point: Point = sqlmodel.Field()


class PortDirectionField(MaskedField, abc.ABC):
    direction: Vector = sqlmodel.Field()


class PortTField(RealField, abc.ABC):
    t: float = sqlmodel.Field(default=0.0)


class PortId(PortIdField, Id):
    pass


class PortProps(PortTField, PortCompatibleFamiliesField, PortFamilyField, PortMandatoryField, PortDescriptionField, PortIdField, Props):
    pass


class PortInput(PortTField, PortCompatibleFamiliesField, PortFamilyField, PortMandatoryField, PortDescriptionField, PortIdField, Input):
    point: PointInput = sqlmodel.Field()
    direction: VectorInput = sqlmodel.Field()
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)


class PortContext(PortTField, PortDirectionField, PortPointField, PortCompatibleFamiliesField, PortFamilyField, PortMandatoryField, PortDescriptionField, PortIdField, Context):
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)


class PortOutput(PortTField, PortDirectionField, PortPointField, PortCompatibleFamiliesField, PortFamilyField, PortMandatoryField, PortDescriptionField, PortIdField, Output):
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)


class Port(PortTField, PortFamilyField, PortMandatoryField, PortDescriptionField, TableEntity, table=True):
    PLURAL = "ports"
    __tablename__ = "ports"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)

    id_: str = sqlmodel.Field(
        # alias="id",  # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column("local_id", sqlalchemy.String(ID_LENGTH_LIMIT)),
        default="",
    )
    compatibleFamilies_: list[CompatibleFamily] = sqlmodel.Relationship(back_populates="port", cascade_delete=True)
    pointX: float = sqlmodel.Field(sa_column=sqlmodel.Column("point_x", sqlalchemy.String(ID_LENGTH_LIMIT)), exclude=True)
    pointY: float = sqlmodel.Field(sa_column=sqlmodel.Column("point_y", sqlalchemy.Float()), exclude=True)
    pointZ: float = sqlmodel.Field(sa_column=sqlmodel.Column("point_z", sqlalchemy.Float()), exclude=True)
    directionX: float = sqlmodel.Field(sa_column=sqlmodel.Column("direction_x", sqlalchemy.Float()), exclude=True)
    directionY: float = sqlmodel.Field(sa_column=sqlmodel.Column("direction_y", sqlalchemy.Float()), exclude=True)
    directionZ: float = sqlmodel.Field(sa_column=sqlmodel.Column("direction_z", sqlalchemy.Float()), exclude=True)
    attributes: list["Attribute"] = sqlmodel.Relationship(back_populates="port", cascade_delete=True)
    props: list["Prop"] = sqlmodel.Relationship(back_populates="port", cascade_delete=True)
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id")), default=None, exclude=True)
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="ports")
    connecteds: list["Connection"] = sqlmodel.Relationship(back_populates="connectedPort", sa_relationship_kwargs={"foreign_keys": "Connection.connectedPortPk"})
    connectings: list["Connection"] = sqlmodel.Relationship(back_populates="connectingPort", sa_relationship_kwargs={"foreign_keys": "Connection.connectingPortPk"})

    __table_args__ = (sqlalchemy.UniqueConstraint("local_id", "type_id", name="uq_ports_local_id_type_id"),)

    @property
    def compatibleFamilies(self) -> list[str]:
        return sorted([cf.name for cf in self.compatibleFamilies_], key=lambda cf: cf.order)

    @compatibleFamilies.setter
    def compatibleFamilies(self, compatibleFamilies: list[str]):
        self.compatibleFamilies_ = [CompatibleFamily(name=cf, order=i) for i, cf in enumerate(compatibleFamilies)]

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
    def parse(cls: "Port", input: str | dict | PortInput | typing.Any | None) -> "Port":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
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
            entity.attributes = [Attribute.parse(q) for q in obj["attributes"]]
        except KeyError:
            pass
        return entity

    def dump(self) -> "PortOutput":
        entity = {**PortProps.model_validate(self).model_dump()}
        entity["point"] = self.point.dump()
        entity["direction"] = self.direction.dump()
        entity["compatibleFamilies"] = self.compatibleFamilies
        entity["attributes"] = [q.dump() for q in self.attributes]
        return PortOutput(**entity)

    # TODO: Automatic derive from Id model.
    def idMembers(self) -> RecursiveAnyList:
        return self.id_


# endregion Port

# region Author
# https://github.com/usalu/semio-author-


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
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="authors_")
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="author", cascade_delete=True)

    __table_args__ = (sqlalchemy.UniqueConstraint("email", "kit_id", name="uq_authors_email_kit_id"),)

    def parent(self) -> "Kit":
        if self.kit is not None:
            return self.kit
        raise NoKitAssigned()

    def idMembers(self) -> RecursiveAnyList:
        return self.email


# endregion Author

# region ArtifactAuthor


class ArtifactAuthorEmailField(RealField, abc.ABC):
    author_email: str = sqlmodel.Field(max_length=ID_LENGTH_LIMIT)


class ArtifactAuthor(ArtifactAuthorEmailField, TableEntity, table=True):
    PLURAL = "artifact_authors"
    __tablename__ = "artifact_authors"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id")), default=None, exclude=True)
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="artifact_authors")
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="artifact_authors")

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

# region Location
# https://github.com/usalu/semio-location-


class LocationLongitudeField(RealField, abc.ABC):
    longitude: float = sqlmodel.Field()


class LocationLatitudeField(RealField, abc.ABC):
    latitude: float = sqlmodel.Field()


class Location(LocationLatitudeField, LocationLongitudeField, TableEntity, table=True):
    PLURAL = "locations"
    __tablename__ = "locations"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="location", cascade_delete=True)


class LocationInput(Location, Input):
    pass


class LocationOutput(Location, Output):
    pass


class LocationContext(Location, Context):
    pass


class LocationPrediction(Location, Prediction):
    pass


# endregion Location

# region Type
# https://github.com/usalu/semio-type-


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


class TypeProps(TypeUnitField, TypeLocationField, TypeVirtualField, TypeStockField, TypeVariantField, TypeImageField, TypeIconField, TypeDescriptionField, TypeNameField, Props):
    pass


class TypeInput(TypeUnitField, TypeVirtualField, TypeStockField, TypeVariantField, TypeImageField, TypeIconField, TypeDescriptionField, TypeNameField, Input):
    location: typing.Optional[LocationInput] = sqlmodel.Field(default=None)
    representations: list[RepresentationInput] = sqlmodel.Field(default_factory=list)
    ports: list[PortInput] = sqlmodel.Field(default_factory=list)
    authors: list[str] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class TypeOutput(TypeUpdatedField, TypeCreatedField, TypeUnitField, TypeVirtualField, TypeStockField, TypeVariantField, TypeImageField, TypeIconField, TypeDescriptionField, TypeNameField, Output):
    location: typing.Optional[LocationOutput] = sqlmodel.Field(default=None)
    representations: list[RepresentationOutput] = sqlmodel.Field(default_factory=list)
    ports: list[PortOutput] = sqlmodel.Field(default_factory=list)
    authors: list[str] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class TypeContext(TypeUnitField, TypeVirtualField, TypeStockField, TypeVariantField, TypeDescriptionField, TypeNameField, Context):
    location: typing.Optional[LocationContext] = sqlmodel.Field(default=None)
    ports: list[PortContext] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class Type(TypeUpdatedField, TypeCreatedField, TypeUnitField, TypeMirrborableField, TypeScalableField, TypeVirtualField, TypeStockField, TypeVariantField, TypeImageField, TypeIconField, TypeDescriptionField, TypeNameField, TableEntity, table=True):
    PLURAL = "types"
    __tablename__ = "types"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)

    locationLongitude: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("location_longitude", sqlalchemy.Float()), exclude=True, default=None)

    locationLatitude: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("location_latitude", sqlalchemy.Float()), exclude=True, default=None)

    representations: list[Representation] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    ports: list[Port] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    artifact_authors: list[ArtifactAuthor] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="type", cascade_delete=True)

    kitPk: typing.Optional[int] = sqlmodel.Field(
        # alias="kitId", # TODO: Check if alias bug is fixed: https://github.com/fastapi/sqlmodel/issues/374
        sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")),
        default=None,
        exclude=True,
    )

    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="types")

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
        props = TypeProps.model_validate(obj)
        entity = cls(**props.model_dump())
        try:
            entity.location = props.location
        except KeyError:
            pass
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
        entity["representations"] = [r.dump() for r in self.representations]
        entity["ports"] = [p.dump() for p in self.ports]
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


# endregion Type

# region Piece
# https://github.com/usalu/semio-piece-


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
    # center: typing.Optional[CoordPrediction] = sqlmodel.Field(
    #     default=None,
    #     ,
    # )
    # """📺 The optional center of the piece in the diagram. When pieces are connected only one piece can have a center."""


class Piece(PieceIdField, PieceTypeField, PieceDesignField, PiecePlaneField, PieceCenterField, PieceHiddenField, PieceLockedField, PieceColorField, PieceScaleField, PieceMirrorPlaneField, TableEntity, table=True):
    PLURAL = "pieces"
    __tablename__ = "pieces"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    id_: str = sqlmodel.Field(sa_column=sqlmodel.Column("local_id", sqlalchemy.String(ID_LENGTH_LIMIT)), default="")
    typePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("type_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("types.id"), nullable=True), default=None, exclude=True)
    type: typing.Optional[Type] = sqlmodel.Relationship(back_populates="pieces")
    designPiecePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id"), nullable=True), default=None, exclude=True)
    designPiece: typing.Optional["Design"] = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Design", foreign_keys="[Piece.designPiecePk]"))
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="pieces")
    planePk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("plane_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("planes.id"), nullable=True), default=None, exclude=True)
    plane: typing.Optional[Plane] = sqlmodel.Relationship(back_populates="piece")
    centerX: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("center_x", sqlalchemy.Float()), exclude=True)
    centerY: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("center_y", sqlalchemy.Float()), exclude=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="piece", cascade_delete=True)
    connecteds: list["Connection"] = sqlmodel.Relationship(back_populates="connectedPiece", sa_relationship_kwargs={"foreign_keys": "Connection.connectedPiecePk"})
    connectings: list["Connection"] = sqlmodel.Relationship(back_populates="connectingPiece", sa_relationship_kwargs={"foreign_keys": "Connection.connectingPiecePk"})

    __table_args__ = (sqlalchemy.UniqueConstraint("local_id", "design_id", name="uq_pieces_local_id_design_id"),)

    @property
    def center(self) -> typing.Optional[Coord]:
        if self.centerX is None or self.centerY is None:
            return None
        return Coord(x=self.centerX, y=self.centerY)

    @center.setter
    def center(self, center: typing.Optional[Coord]):
        if center is None:
            self.centerX = None
            self.centerY = None
            return
        self.centerX = center.x
        self.centerY = center.y

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


# endregion Piece

# region Side
# https://github.com/usalu/semio-side-


class Side(Model):
    piece: PieceId = sqlmodel.Field()
    designPiece: typing.Optional[PieceId] = sqlmodel.Field(default=None)
    port: PortId = sqlmodel.Field()

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Side", input: str | dict | typing.Any | None) -> "Side":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
        piece = PieceId.parse(obj["piece"])
        port = PortId.parse(obj["port"])
        try:
            designPieceObj = obj["designPiece"]
            designPiece = PieceId.parse(designPieceObj) if designPieceObj is not None else None
        except KeyError:
            designPiece = None
        return cls(piece=piece, designPiece=designPiece, port=port)


class SideInput(Side, Input):
    pass


class SideContext(Side, Context):
    pass


class SideOutput(Side, Output):
    pass


class SidePrediction(Side, Prediction):
    pass


# endregion Side

# region Connection
# https://github.com/usalu/semio-connection-


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


# class ConnectionOrientationFirstField(RealField, abc.ABC):
#     """🥇 Wheather the orientation (rotation, turn, tilt) is applied before the translation (gap, shift, raise). By default the translation happens before the orientation."""

#     orientationFirst: bool = sqlmodel.Field(
#         default=False,
#         ,
#     )
#     """🥇 Wheather the orientation (rotation, turn, tilt) is applied before the translation (gap, shift, raise). By default the translation happens before the orientation."""


class ConnectionXField(RealField, abc.ABC):
    x: float = sqlmodel.Field(default=0)


class ConnectionYField(RealField, abc.ABC):
    y: float = sqlmodel.Field(default=0)


class ConnectionId(ConnectionConnectedField, ConnectionConnectingField, Id):
    pass


class ConnectionProps(ConnectionYField, ConnectionXField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, Props):
    pass


class ConnectionInput(ConnectionYField, ConnectionXField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, Input):
    pass

    connected: SideInput = sqlmodel.Field()
    connecting: SideInput = sqlmodel.Field()


class ConnectionContext(ConnectionYField, ConnectionXField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, Context):
    pass

    connected: SideContext = sqlmodel.Field()
    connecting: SideContext = sqlmodel.Field()


class ConnectionOutput(ConnectionYField, ConnectionXField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, Output):
    pass

    connected: SideOutput = sqlmodel.Field()
    connecting: SideOutput = sqlmodel.Field()


class ConnectionPrediction(ConnectionYField, ConnectionXField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, Prediction):
    pass

    connected: SidePrediction = sqlmodel.Field()
    connecting: SidePrediction = sqlmodel.Field()


class Connection(ConnectionYField, ConnectionXField, ConnectionTiltField, ConnectionTurnField, ConnectionRotationField, ConnectionRiseField, ConnectionShiftField, ConnectionGapField, ConnectionDescriptionField, TableEntity, table=True):
    PLURAL = "connections"
    __tablename__ = "connections"

    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    connectedPiecePk: typing.Optional[int] = sqlmodel.Field(alias="connectedPieceId", sa_column=sqlmodel.Column("connected_piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("pieces.id")), default=None, exclude=True)
    connectedPiece: Piece = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Piece", back_populates="connecteds", foreign_keys="[Connection.connectedPiecePk]"))
    connectedPortPk: typing.Optional[int] = sqlmodel.Field(alias="connectedPortId", sa_column=sqlmodel.Column("connected_port_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("ports.id")), default=None, exclude=True)
    connectedPort: Port = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Port", back_populates="connecteds", foreign_keys="[Connection.connectedPortPk]"))
    connectedDesignPiecePk: typing.Optional[int] = sqlmodel.Field(
        alias="connectedDesignPieceId", sa_column=sqlmodel.Column("connected_design_piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("pieces.id"), nullable=True), default=None, exclude=True
    )
    connectedDesignPiece: typing.Optional[Piece] = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Piece", foreign_keys="[Connection.connectedDesignPiecePk]"))
    connectingPiecePk: typing.Optional[int] = sqlmodel.Field(alias="connectingPieceId", sa_column=sqlmodel.Column("connecting_piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("pieces.id")), exclude=True, default=None)
    connectingPiece: Piece = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Piece", back_populates="connectings", foreign_keys="[Connection.connectingPiecePk]"))
    connectingPortPk: typing.Optional[int] = sqlmodel.Field(alias="connectingPortId", sa_column=sqlmodel.Column("connecting_port_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("ports.id")), default=None, exclude=True)
    connectingPort: Port = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Port", back_populates="connectings", foreign_keys="[Connection.connectingPortPk]"))
    connectingDesignPiecePk: typing.Optional[int] = sqlmodel.Field(
        alias="connectingDesignPieceId", sa_column=sqlmodel.Column("connecting_design_piece_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("pieces.id"), nullable=True), default=None, exclude=True
    )
    connectingDesignPiece: typing.Optional[Piece] = sqlmodel.Relationship(sa_relationship=sqlalchemy.orm.relationship("Piece", foreign_keys="[Connection.connectingDesignPiecePk]"))
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="connection", cascade_delete=True)
    designPk: typing.Optional[int] = sqlmodel.Field(alias="designId", sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: "Design" = sqlmodel.Relationship(back_populates="connections")
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
            port=self.connectedPort,
        )

    @property
    def connecting(self) -> Side:
        return Side(
            piece=self.connectingPiece,
            designPiece=(PieceId(id_=self.connectingDesignPiece.id_) if self.connectingDesignPiece is not None else None),
            port=self.connectingPort,
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
        connectedPort = [p for p in connectedType.ports if p.id_ == connected.port.id_]
        if len(connectedPort) == 0:
            raise PortNotFound(connectedType, connected.port)
        else:
            connectedPort = connectedPort[0]
        connectingPiece = piecesDict[connecting.piece.id_]
        connectingType = connectingPiece.type
        if connectingType is None:
            raise FeatureNotYetSupported()
        connectingPort = [p for p in connectingType.ports if p.id_ == connecting.port.id_]
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
        if connected.designPiece is not None:
            if connectedPiece.refDesign is None and designsById is None:
                raise FeatureNotYetSupported()
            refDesign = connectedPiece.refDesign if connectedPiece.refDesign is not None else None
            if refDesign is None and designsById is not None:
                # best-effort lookup by connected piece's type/design is not possible; require refDesign
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
            self.connected.port.id_,
            self.connecting.piece.id_,
            self.connecting.port.id_,
        ]


# endregion Connection

# region Design
# https://github.com/usalu/semio-design-


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


class DesignProps(DesignUnitField, DesignViewField, DesignLocationField, DesignVariantField, DesignImageField, DesignIconField, DesignDescriptionField, DesignNameField, Props):
    pass


class DesignInput(DesignUnitField, DesignViewField, DesignVariantField, DesignImageField, DesignIconField, DesignDescriptionField, DesignNameField, Input):
    pass

    location: typing.Optional[LocationInput] = sqlmodel.Field(default=None)
    pieces: list[PieceInput] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionInput] = sqlmodel.Field(default_factory=list)
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

    location: typing.Optional[LocationOutput] = sqlmodel.Field(default=None)
    pieces: list[PieceOutput] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionOutput] = sqlmodel.Field(default_factory=list)
    authors: list[str] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class DesignPrediction(DesignDescriptionField, Prediction):
    pass

    pieces: list[PiecePrediction] = sqlmodel.Field(default_factory=list)
    connections: list[ConnectionPrediction] = sqlmodel.Field(default_factory=list)


class Design(
    DesignNameField, DesignVariantField, DesignViewField, DesignDescriptionField, DesignIconField, DesignImageField, DesignUnitField, DesignScalableField, DesignMirrorableField, DesignUpdatedField, DesignCreatedField, TableEntity, table=True
):
    PLURAL = "designs"
    __tablename__ = "designs"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    concepts_: list[Concept] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    artifact_authors: list[ArtifactAuthor] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    locationLongitude: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("location_longitude", sqlalchemy.Float()), exclude=True, default=None)
    locationLatitude: typing.Optional[float] = sqlmodel.Field(sa_column=sqlmodel.Column("location_latitude", sqlalchemy.Float()), exclude=True, default=None)
    layers: list[Layer] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    pieces: list[Piece] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    groups: list[Group] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    connections: list[Connection] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    stats: list[Stat] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="design", cascade_delete=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(alias="kitId", sa_column=sqlmodel.Column("kit_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("kits.id")), default=None, exclude=True)
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="designs")

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


# endregion Design

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


class QualityCreatedField(RealField, abc.ABC):
    created_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class QualityUpdatedField(RealField, abc.ABC):
    updated_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class QualityId(QualityKeyField, Id):
    pass


class QualityProps(
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
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="qualities")

    benchmarks: list["Benchmark"] = sqlmodel.Relationship(back_populates="quality", cascade_delete=True)
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="quality", cascade_delete=True)

    __table_args__ = (
        sqlalchemy.CheckConstraint("kind >= 0 AND kind <= 63", name="ck_qualities_kind_range"),
        sqlalchemy.UniqueConstraint("key", "kit_id", name="uq_qualities_key_kit_id"),
    )


# endregion Quality

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
    quality: typing.Optional[Quality] = sqlmodel.Relationship(back_populates="benchmarks")
    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="benchmark", cascade_delete=True)


# endregion Benchmark

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
    portPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("port_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("ports.id")), default=None, exclude=True)
    port: typing.Optional["Port"] = sqlmodel.Relationship(back_populates="props")

    attributes: list[Attribute] = sqlmodel.Relationship(back_populates="prop", cascade_delete=True)


# endregion Prop

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
    design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="stats")


# endregion Stat

# region Layer


class LayerNameField(RealField, abc.ABC):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class LayerDescriptionField(RealField, abc.ABC):
    description: str = sqlmodel.Field(default="", max_length=DESCRIPTION_LENGTH_LIMIT)


class LayerColorField(RealField, abc.ABC):
    color: str = sqlmodel.Field(default="", max_length=7)


class LayerId(LayerNameField, Id):
    pass


class LayerProps(LayerColorField, LayerDescriptionField, LayerNameField, Props):
    pass


class LayerInput(LayerColorField, LayerDescriptionField, LayerNameField, Input):
    pass


class LayerOutput(LayerColorField, LayerDescriptionField, LayerNameField, Output):
    pass


class Layer(LayerColorField, LayerDescriptionField, LayerNameField, TableEntity, table=True):
    PLURAL = "layers"
    __tablename__ = "layers"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    designPk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("design_id", sqlalchemy.Integer(), sqlalchemy.ForeignKey("designs.id")), default=None, exclude=True)
    design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="layers")


# endregion Layer

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
    design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="groups")


# endregion Group

# region Kit
# https://github.com/usalu/semio-kit-


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


class KitHomepage(RealField, abc.ABC):
    homepage: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitLicenseField(RealField, abc.ABC):
    license: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class KitCreatedField(RealField, abc.ABC):
    created_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class KitUpdatedField(RealField, abc.ABC):
    updated_at: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class KitId(KitUriField, Id):
    pass


class KitProps(KitLicenseField, KitHomepage, KitRemoteField, KitVersionField, KitPreviewField, KitImageField, KitIconField, KitDescriptionField, KitNameField, KitUriField, Props):
    pass


class KitInput(KitLicenseField, KitHomepage, KitRemoteField, KitVersionField, KitPreviewField, KitImageField, KitIconField, KitDescriptionField, KitNameField, Input):
    pass

    types: list[TypeInput] = sqlmodel.Field(default_factory=list)
    designs: list[DesignInput] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeInput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class KitContext(KitDescriptionField, KitNameField, Context):
    pass

    types: list[TypeContext] = sqlmodel.Field(default_factory=list)
    designs: list[DesignContext] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeContext] = sqlmodel.Field(default_factory=list)


class KitOutput(KitUpdatedField, KitCreatedField, KitLicenseField, KitHomepage, KitRemoteField, KitVersionField, KitPreviewField, KitImageField, KitIconField, KitDescriptionField, KitNameField, KitUriField, Output):
    pass

    types: list[TypeOutput] = sqlmodel.Field(default_factory=list)
    designs: list[DesignOutput] = sqlmodel.Field(default_factory=list)
    attributes: list[AttributeOutput] = sqlmodel.Field(default_factory=list)
    concepts: list[str] = sqlmodel.Field(default_factory=list)


class Kit(KitNameField, KitVersionField, KitDescriptionField, KitIconField, KitImageField, KitRemoteField, KitHomepage, KitLicenseField, KitPreviewField, KitUriField, KitUpdatedField, KitCreatedField, TableEntity, table=True):
    PLURAL = "kits"
    __tablename__ = "kits"
    pk: typing.Optional[int] = sqlmodel.Field(sa_column=sqlmodel.Column("id", sqlalchemy.Integer(), primary_key=True), default=None, exclude=True)
    concepts_: list[Concept] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    authors_: list[Author] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
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

    __table_args__ = (sqlalchemy.UniqueConstraint("uri", name="uq_kits_uri"),)

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Kit", input: str | dict | KitInput | typing.Any | None) -> "Kit":
        if input is None:
            return cls()
        obj = json.loads(input) if isinstance(input, str) else input if isinstance(input, dict) else input.__dict__
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


# endregion Models

# endregion Domain

# endregion Modeling

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
                        # gather
                        existingType = existingTypeUnion.Type
                        existingPorts = {p.id_: p for p in existingType.ports}
                        usedPorts = {}
                        for port in list(existingType.ports):
                            for connection in port.connections:
                                if connection.connectedPiece.type == existingType:
                                    usedPorts[connection.connectedPort.id_] = connection.connectedPort
                                if connection.connectingPiece.type == existingType:
                                    usedPorts[connection.connectingPort.id_] = connection.connectingPort
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

                            for attribute in list(usedPort.attributes):
                                self.session.delete(attribute)
                            usedPort.attributes = []
                            self.session.flush()

                            newAttributes = []
                            for newAttribute in list(newPorts[usedPortId].attributes):
                                newAttribute.port = usedPort
                                self.session.add(newAttribute)
                                newAttributes.append(newAttribute)
                            usedPort.attributes = newAttributes
                            self.session.flush()

                        for unusedPort in list(unusedPorts):
                            self.session.delete(existingPorts[unusedPort])
                        existingType.ports = [p for p in existingType.ports if p.id_ not in unusedPorts]
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
        # for directory in paths:
        #     if directory != ".semio":
        #         if os.path.isfile(os.path.join(path, directory)):
        #             os.remove(os.path.join(path, directory))
        #         else:
        #             shutil.rmtree(os.path.join(path, directory))
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
        try:  # change uri if local kit is already created
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
    for port in typeClone.ports:
        port.id_ = replaceDefault(port.id_, "DEFAULT")
        # for attribute in port.attributes:
        #     attribute.value = replaceDefault(attribute.value, "TRUE")
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
                    "port": {
                        "id_": (c["connectedPieceTypePortId"] if c["connectedPieceTypePortId"] != "DEFAULT" else ""),
                    },
                },
                "connecting": {
                    "piece": {
                        "id_": (c["connectingPieceId"] if c["connectingPieceId"] != "DEFAULT" else ""),
                    },
                    "port": {
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

        if connection.connected.port.id_ not in portD[connectedType.name][connectedType.variant]:
            connection.connected.port.id_ = difflib.get_close_matches(
                connection.connected.port.id_,
                portD[connectedType.name][connectedType.variant].keys(),
                n=1,
            )[0]
        if connection.connecting.port.id_ not in portD[connectingType.name][connectingType.variant]:
            connection.connecting.port.id_ = difflib.get_close_matches(
                connection.connecting.port.id_,
                portD[connectingType.name][connectingType.variant].keys(),
                n=1,
            )[0]
        validConnections.append(connection)
    designClone.connections = validConnections
    # remove invalid connections
    designClone.connections = [c for c in designClone.connections if c.connected.piece.id_ != c.connecting]
    # remove pieces with no connections
    designClone.pieces = [p for p in designClone.pieces if any(c for c in designClone.connections if c.connected.piece.id_ == p.id_ or c.connecting.piece.id_ == p.id_)]
    return designClone


try:
    openaiClient = openai.Client()
except openai.OpenAIError:
    openaiClient = None

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
                        "rise": {
                            "type": "number",
                            "description": "The optional vertical rise in port direction between the connected and the connecting piece. Set this only when necessary as it is not a symmetric property which means that when the parent piece and child piece are flipped it yields a different result."
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
# with open("../../jsonschema/design-prediction-openai.json", "r") as f:
#     designResponseFormat = json.load(f)


def predictDesign(description: str, types: list[TypeContext], design: DesignInput | None = None) -> DesignPrediction:
    """🔮 Predict a design based on a description, the types that should be used and an optional base design."""
    if openaiClient is None:
        raise FeatureNotYetSupported("OpenAI client not available")

    prompt = designGenerationPromptTemplate.render(description=description, types=[encodeType(t) for t in types])
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

        # piece healing of variants that do not exist
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
    "Port": graphene.NonNull(lambda: PortNode),
    "PortId": graphene.NonNull(lambda: PortNode),
    "list[Port]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: PortNode))),
    "list[__main__.Port]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: PortNode))),
    "list[__mp_main__.Port]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: PortNode))),
    "list[engine.Port]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: PortNode))),
    "Representation": graphene.NonNull(lambda: RepresentationNode),
    "list[Representation]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: RepresentationNode))),
    "list[__main__.Representation]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: RepresentationNode))),
    "list[__mp_main__.Representation]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: RepresentationNode))),
    "list[engine.Representation]": graphene.NonNull(graphene.List(graphene.NonNull(lambda: RepresentationNode))),
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

        own_properties = [name for name, value in model.__dict__.items() if isinstance(value, property)]

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


class AttributeNode(TableEntityNode):
    class Meta:
        model = Attribute


class AttributeInputNode(InputNode):
    class Meta:
        model = AttributeInput


class LocationNode(Node):
    class Meta:
        model = Location


class LocationInputNode(InputNode):
    class Meta:
        model = LocationInput


class RepresentationNode(TableEntityNode):
    class Meta:
        model = Representation
        excludedFields = ("tags_",)

    # attributes = graphene.List(graphene.NonNull(lambda: AttributeNode))

    # def resolve_attributes(self, info):
    #     return self.attributes


class RepresentationInputNode(InputNode):
    class Meta:
        model = RepresentationInput


class CoordNode(Node):
    class Meta:
        model = Coord


class CoordInputNode(InputNode):
    class Meta:
        model = CoordInput


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

    # Add localId field to follow GraphQL naming conventions instead of id_
    localId = graphene.String()

    def resolve_localId(self, info):
        return getattr(self, "id_", "")

    # attributes = graphene.List(graphene.NonNull(lambda: AttributeNode))

    # def resolve_attributes(self, info):
    #     return self.attributes


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

    # Add localId field to follow GraphQL naming conventions instead of id_
    localId = graphene.String()

    def resolve_localId(self, info):
        return getattr(self, "id_", "")


class PieceInputNode(InputNode):
    class Meta:
        model = PieceInput

    type = TypeIdInputNode()
    designPiece = graphene.Field(lambda: DesignIdInputNode)


class PieceIdInputNode(InputNode):
    class Meta:
        model = PieceId


class SideNode(Node):
    class Meta:
        model = Side

    exclude_fields = ("piece", "port")

    piece = graphene.NonNull(PieceNode)
    designPiece = graphene.Field(PieceNode)
    port = graphene.NonNull(PortNode)

    def resolve_piece(self, info):
        return self.piece

    def resolve_designPiece(self, info):
        return self.designPiece

    def resolve_port(self, info):
        return self.port


class SideInputNode(InputNode):
    class Meta:
        model = SideInput

    exclude_fields = ("piece", "port")

    piece = graphene.NonNull(PieceIdInputNode)
    designPiece = PieceIdInputNode()
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


class DesignIdInputNode(InputNode):
    class Meta:
        model = DesignId


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
    logging.basicConfig(level=logging.INFO)  # for uvicorn in pyinstaller
    uvicorn.run(engine, host=HOST, port=PORT, log_level="info", access_log=False, log_config=None)


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

    parser = argparse.ArgumentParser(description="semio ⋅ engine")
    parser.add_argument("-d", "--debug", help="debug mode", action="store_true")

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

# endregion Engine
