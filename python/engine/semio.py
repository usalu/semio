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
# TODO: Make sure that kit guid has url not remote url as base.
# TODO: Check how to automate docstring duplication, table=True and PLURAL and __tablename__.
# class KitBase(sqlmodel.SQLModel):
#     '''🗃️ A kit is a collection of types and designs.'''
# class Kit(KitBase, Model, table=True):
#     '''🗃️ A kit is a collection of types and designs.'''

#     PLURAL = 'kits'
#     __tablename__ = 'kit'
# to:
# class KitBase(sqlmodel.SQLModel):
#     '''🗃️ A kit is a collection of types and designs.'''
# class Kit(KitBase, Model):
# TODO: Automatic emptying.
# TODO: Automatic updating based on props.

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


# constants

RELEASE = "r24.11-1"
VERSION = "3.0.0"
HOST = "127.0.0.1"
PORT = 24111
NAME_LENGTH_LIMIT = 64
ID_LENGTH_LIMIT = 128
URL_LENGTH_LIMIT = 1024
DESCRIPTION_LENGTH_LIMIT = 4096
ENCODING_REGEX = r"[a-zA-ZZ0-9_-]+={0,2}"
KIT_FOLDERNAME = ".semio"
KIT_FILENAME = "kit.sqlite3"
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

ureg = pint.UnitRegistry()


# utility


def encodeString(value: str) -> str:
    encoded_bytes = base64.urlsafe_b64encode(value.encode("utf-8"))
    encoded_str = encoded_bytes.decode("utf-8")
    return encoded_str


def decodeString(value: str) -> str:
    value += "=" * (-len(value) % 4)
    decoded_bytes = base64.urlsafe_b64decode(value.encode("utf-8"))
    decoded_str = decoded_bytes.decode("utf-8")
    return decoded_str


def prettyNumber(number: float) -> str:
    if number == -0.0:
        number = 0.0
    return f"{number:.5f}".rstrip("0").rstrip(".")


# exceptions


class Error(Exception):
    """❗ The base class for all exceptions in"""

    def __str__(self):
        return self.__class__.__name__


class ServerError(Error):
    """🖥️ The base class for all server errors."""

    pass


class CodeUnreachable(ServerError):

    def __str__(self):
        return "This code should be unreachable 🤷"


class FeatureNotYetImplemented(ServerError):

    def __str__(self):
        return "This feature is not yet implemented 🔜"


class InvalidDatabase(ServerError):
    """💾 The state of the database is somehow invalid.
    Check the constraints and the insert validators.
    """

    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + "\n The database is invalid."


class ClientError(Error):
    """👩‍💼 The base class for all client errors."""

    pass


class NotFound(ClientError):
    """🔍 The entity was not found."""

    pass


class TypeNotFound(NotFound):

    def __init__(self, id: "TypeId") -> None:
        self.id = id

    def __str__(self):
        variant = f", {self.id.variant}" if self.id.variant else ""
        return f"Couldn't find the type ({id.name}{variant})."


# class DesignNotFound(NotFound):

#     def __init__(self, id: "DesignId") -> None:
#         self.id = id

#     def __str__(self):
#         variant = f", {self.id.variant})" if self.id.variant else ""
#         return f"Couldn't find the design ({id.name}{variant})."


class KitNotFound(NotFound):

    def __init__(self, url: str) -> None:
        self.url = url

    def __str__(self):
        return f"Couldn't find a local or remote kit under url:\n {self.url}."


class SpecificationError(ClientError):
    """🚫 The base class for all specification errors.
    A specification error is when the user input does not respect the specification."""

    pass


class NoParentAssigned(SpecificationError):
    """👪 The entity has no parent assigned."""

    pass


class NoRepresentationAssigned(NoParentAssigned):
    """👪 The entity has no representation assigned."""

    pass


class NoTypeAssigned(NoParentAssigned):
    """👪 The entity has no type assigned."""

    pass


class NoKitAssigned(NoParentAssigned):
    """👪 The entity has no kit assigned."""

    pass


class InvalidURL(SpecificationError):

    def __init__(self, url: str) -> None:
        self.url = url

    def __str__(self) -> str:
        return f"{self.url} is not a valid URL. \n A url must be a path or a link."


class InvalidGuid(SpecificationError):
    """🆔 The guid is not valid. A guid looks like this:
    ENCODED_KIT_URL/types/ENCODED_TYPE_NAME,ENCODED_TYPE_VARIANT/..."""

    pass


class InvalidQuery(InvalidGuid):
    """🔍 The query is not valid. A query looks like this:
    ENCODED_KIT_URL/types/ENCODED_TYPE_NAME,ENCODED_TYPE_VARIANT/..."""

    pass


class AlreadyExists(SpecificationError):
    """♊ The entity already exists in the store."""

    pass


class KitAlreadyExists(AlreadyExists):

    def __init__(self, url: str) -> None:
        self.url = url

    def __str__(self) -> str:
        return f"The kit under url {self.url} already exists."


# models


class Semio(sqlmodel.SQLModel):
    """ℹ️ Metadata about the semio database."""

    __tablename__ = "semio"

    release: str = sqlmodel.Field(default=RELEASE, primary_key=True)
    createdAt: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)


class Model(sqlmodel.SQLModel):
    """Base class for all models in"""

    PLURAL: typing.ClassVar[str]
    """🔢 The plural of the entity."""

    def parent(self) -> typing.Optional["Model"]:
        """👪 The parent of the entity."""
        return None

    # @abc.abstractmethod
    def localId(self, encode: bool = False) -> tuple:
        """🆔 A tuple that identifies the entity within it's parent."""
        pass

    def humanId(self) -> str:
        """🪪 A string that let's the user identify the entity within it's parent."""
        return f"{self.__class__.__name__}({", ".join(self.localId())})"

    def guid(self) -> str:
        """🆔 A guid that let's relay identify the entity."""
        localId = f"{self.__class__.PLURAL.lower()}/{self.localId(encode=True)}"
        parent = self.parent()
        parentId = f"{parent.guid()}/" if parent is not None else ""
        return parentId + localId

    @classmethod
    def parse(cls, input: str | dict | typing.Any) -> "Model":
        """⚒️ Parse the model from an input."""
        if isinstance(input, str):
            return cls.model_validate_json(input)
        return cls.model_validate(input)

    # TODO: Automatic emptying.
    # @abc.abstractmethod
    def empty(self) -> "Model":
        """🪣 Empty the model."""
        pass

    # TODO: Automatic updating based on props.
    # @abc.abstractmethod
    def update(self, other: "Model", empty: bool = False) -> "Model":
        """🔄 Update the model. Optionally empty the model before."""
        pass


class Id(Model):
    pass


class Dto(Model):
    pass


class Props(Dto):
    pass


class Input(Dto):
    pass


class Output(Dto):
    pass
    # class Config:
    #     title = __name__[:6]  # len('Output')=6 E.g. 'PieceOutput'->'Piece'


class IdentifiedModel(Model):
    id_: str = sqlmodel.Field(alias="id")

    def localId(self, encode: bool = False) -> str:
        return encodeString(self.id_) if encode else self.id_


class IdentifiedId(Id):
    id_: str = sqlmodel.Field(alias="id")


class UrledModel(Model):
    url: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT)

    def localId(self, encode: bool = False) -> str:
        return encodeString(self.url) if encode else self.url


class UrledId(Id):
    url: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT)


class NamedModel(Model):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

    def localId(self, encode: bool = False) -> str:
        return encodeString(self.name) if encode else self.name


class NamedId(Id):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class ArtifactBase(NamedModel):
    """♻️ An artifact is anything that is worth to be reused."""

    # Optional. Set to '' for None.
    description: str = sqlmodel.Field(max_length=DESCRIPTION_LENGTH_LIMIT, default="")
    # Optional. Set to '' for None.
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)


class Artifact(ArtifactBase):
    createdAt: datetime.datetime = sqlmodel.Field(default_factory=datetime.datetime.now)
    lastUpdateAt: datetime.datetime = sqlmodel.Field(
        default_factory=datetime.datetime.now
    )


class VariableArtifact(ArtifactBase):
    """🎚️ A variable artifact is an artifact that has variants (at least one default)."""

    variant: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT, default="")

    def localId(self, encode: bool = False) -> str:
        return f"{super().localId(encode)},{(encodeString(self.variant) if encode else self.variant)}"


class VariableArtifactId(Id):
    """🎚️ A variable artifact is an artifact that has variants (at least one default)."""

    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)
    variant: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT, default="")


class Tag(sqlmodel.SQLModel, table=True):
    """🏷️ A tag is meta-data for grouping representations."""

    # __tablename__ = 'tag'
    value: str = sqlmodel.Field(primary_key=True)
    representationPk: int = sqlmodel.Field(
        alias="representationId",
        sa_column=sqlmodel.Column(
            "representationId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("representation.id"),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    representation: "Representation" = sqlmodel.Relationship(back_populates="_tags")

    def parent(self) -> typing.Optional["Representation"]:
        if self.representation is None:
            raise NoRepresentationAssigned()
        return self.representation

    def localId(self, encode: bool = False) -> tuple:
        return (encodeString(self.value) if encode else self.value,)


class RepresentationBase(UrledModel):
    lod: str = ""


class Representation(RepresentationBase, table=True):
    """💾 A representation is a link to a file that describes a type for a unique combination of level of detail, tags and mime."""

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
    _tags: list[Tag] = sqlmodel.Relationship(
        back_populates="representation",
        cascade_delete=True,
    )
    typePk: typing.Optional[int] = sqlmodel.Field(
        alias="typeId",
        sa_column=sqlmodel.Column(
            "typeId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    type: typing.Optional["Type"] = sqlmodel.Relationship(
        back_populates="representations"
    )

    @property
    def tags(self) -> list[str]:
        return [tag.value for tag in self._tags or []]

    @tags.setter
    def tags(self, tags: list[str]):
        self._tags = [Tag(value=tag) for tag in tags]

    def parent(self) -> "Type":
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
        model = cls(**props.model_dump())
        try:
            model.tags = obj["tags"]
        except KeyError:
            pass
        return model


class RepresentationId(UrledId):
    pass


class RepresentationDto(RepresentationBase):
    tags: list[str] = sqlmodel.Field(default_factory=list)


class RepresentationProps(RepresentationBase, Props):
    pass


class RepresentationInput(RepresentationDto, Input):
    pass


class RepresentationOutput(RepresentationDto, Output):
    pass


class LocatorBase(Model):
    # Optional. '' means true.
    subgroup: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class Locator(LocatorBase, table=True):
    """🗺️ A locator is meta-data for grouping ports."""

    PLURAL = "locators"
    __tablename__ = "locator"
    group: str = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "groupName",  # group is a reserved word in SQL
            sqlalchemy.String(NAME_LENGTH_LIMIT),
            primary_key=True,
        ),
    )
    portPk: typing.Optional[int] = sqlmodel.Field(
        alias="portId",
        sa_column=sqlmodel.Column(
            "portId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("port.id"),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    port: typing.Optional["Port"] = sqlmodel.Relationship(back_populates="locators")


class LocatorId(Id):
    group: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class LocatorDto(LocatorBase, Dto):
    group: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)


class LocatorInput(LocatorDto, Input):
    pass


class LocatorOutput(LocatorDto, Output):
    pass


# class ScreenPoint(sqlmodel.SQLModel):
#     """📺 A 2d-point (xy) of integers in screen coordinate system."""

#     x: int = 0
#     y: int = 0

#     def __init__(self, x: int = 0, y: int = 0):
#         super().__init__(x=x, y=y)

#     def __len__(self):
#         return 2

#     def __getitem__(self, key):
#         if key == 0:
#             return self.x
#         elif key == 1:
#             return self.y
#         else:
#             raise IndexError("Index out of range")

#     def __iter__(self):
#         return iter((self.x, self.y))


class PointBase(sqlmodel.SQLModel):
    x: float = sqlmodel.Field(default=0.0)
    y: float = sqlmodel.Field(default=0.0)
    z: float = sqlmodel.Field(default=0.0)


class Point(PointBase):
    """✖️ A 3d-point (xyz) of floating point numbers."""

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return (
            f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"
        )

    def __repr__(self) -> str:
        return (
            f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"
        )

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


class PointProps(PointBase, Props):
    pass


class PointDto(PointBase, Dto):
    pass


class PointInput(PointDto, Input):
    pass


class PointOutput(PointDto, Output):
    pass


class VectorBase(sqlmodel.SQLModel):
    """➡️ A 3d-vector (xyz) of floating point numbers."""

    x: float = 0.0
    y: float = 0.0
    z: float = 0.0


class Vector(VectorBase):
    """➡️ A 3d-vector (xyz) of floating point numbers."""

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return (
            f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"
        )

    def __repr__(self) -> str:
        return (
            f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"
        )

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


class VectorProps(VectorBase, Props):
    pass


class VectorDto(VectorBase, Input):
    pass


class VectorInput(VectorDto, Input):
    pass


class VectorOutput(VectorDto, Output):
    pass


class PlaneBase(Model):
    pass


class Plane(PlaneBase, table=True):
    """◳ A coordinate system is an origin (point) and an orientation (x-axis and y-axis)."""

    PLURAL = "planes"
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

    originX: float = sqlmodel.Field(exclude=True)
    originY: float = sqlmodel.Field(exclude=True)
    originZ: float = sqlmodel.Field(exclude=True)
    xAxisX: float = sqlmodel.Field(exclude=True)
    xAxisY: float = sqlmodel.Field(exclude=True)
    xAxisZ: float = sqlmodel.Field(exclude=True)
    yAxisX: float = sqlmodel.Field(exclude=True)
    yAxisY: float = sqlmodel.Field(exclude=True)
    yAxisZ: float = sqlmodel.Field(exclude=True)

    # piece: typing.Optional["Piece"] = sqlmodel.Relationship(back_populates="plane")

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

    @property
    def xAxis(self) -> Vector:
        return Vector(
            self.xAxisX,
            self.xAxisY,
            self.xAxisZ,
        )

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


class PlaneDto(PlaneBase, Dto):
    origin: Point = sqlmodel.Field()
    xAxis: Vector = sqlmodel.Field()
    yAxis: Vector = sqlmodel.Field()


class PlaneInput(PlaneDto, Input):
    pass


class PlaneOutput(PlaneDto, Output):
    pass


class Rotation(sqlmodel.SQLModel):
    """🔄 A rotation is an axis and an angle."""

    axis: Vector
    angle: float

    def __init__(self, axis: Vector, angle: float):
        super().__init__(axis=axis, angle=angle)

    def toTransform(self) -> "Transform":
        return Transform.fromRotation(self)


class Transform(numpy.ndarray):
    """▦ A 4x4 translation and rotation transformation matrix (no scaling or shearing)."""

    def __new__(cls, input_array=None):
        if input_array is None:
            input_array = eye(4, dtype=float)
        else:
            input_array = asarray(input_array).astype(float)
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
            angle=float(degrees(axisAngle[3])),
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
            raise FeatureNotYetImplemented()

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


class PortBase(IdentifiedModel):
    """🔌 A port is a connection point (with a direction) of a type."""

    pass


class Port(PortBase, table=True):
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
    # Can't use the name 'id' because of bug
    # https://github.com/graphql-python/graphene-sqlalchemy/issues/412
    id_: str = sqlmodel.Field(
        alias="id",
        sa_column=sqlmodel.Column(
            "localId",
            sqlalchemy.String(NAME_LENGTH_LIMIT),
        ),
    )
    pointX: float = sqlmodel.Field(exclude=True)
    pointY: float = sqlmodel.Field(exclude=True)
    pointZ: float = sqlmodel.Field(exclude=True)
    directionX: float = sqlmodel.Field(exclude=True)
    directionY: float = sqlmodel.Field(exclude=True)
    directionZ: float = sqlmodel.Field(exclude=True)
    typePk: typing.Optional[int] = sqlmodel.Field(
        alias="typeId",
        sa_column=sqlmodel.Column(
            "typeId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="ports")
    locators: list[Locator] = sqlmodel.Relationship(
        back_populates="port", cascade_delete=True
    )
    # connecteds: list["Connection"] = sqlmodel.Relationship(
    #     back_populates="connectedPieceTypePort",
    #     sa_relationship_kwargs={"foreign_keys": "Connection.connectedPieceTypePortPk"},
    # )
    # connectings: list["Connection"] = sqlmodel.Relationship(
    #     back_populates="connectingPieceTypePort",
    #     sa_relationship_kwargs={"foreign_keys": "Connection.connectingPieceTypePortPk"},
    # )

    __table_args__ = (sqlalchemy.UniqueConstraint("localId", "typeId"),)

    @property
    def point(self) -> Point:
        return Point(self.pointX, self.pointY, self.pointZ)

    @point.setter
    def point(self, point: Point):
        self.pointX = point.x
        self.pointY = point.y
        self.pointZ = point.z

    @property
    def direction(self) -> Vector:
        return Vector(self.directionX, self.directionY, self.directionZ)

    @direction.setter
    def direction(self, direction: Vector):
        self.directionX = direction.x
        self.directionY = direction.y
        self.directionZ = direction.z

    def parent(self) -> "Type":
        if self.type is None:
            raise NoTypeAssigned()
        return self.type


class PortId(IdentifiedId):
    pass


class PortProps(PortBase, Props):
    pass


class PortDto(PortBase, Dto):
    id_: str = sqlmodel.Field(alias="id")
    point: Point = sqlmodel.Field()
    direction: Vector = sqlmodel.Field()
    locators: list[LocatorOutput] = sqlmodel.Field(default_factory=list)


class PortInput(PortDto, Input):
    pass


class PortOutput(PortDto, Output):
    pass


class QualityBase(NamedModel):

    # Optional. '' means true.
    value: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT, default="")
    # Optional. Set to '' for None.
    definition: str = sqlmodel.Field(max_length=DESCRIPTION_LENGTH_LIMIT, default="")
    # Optional. Set to '' for None.
    unit: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT, default="")


class Quality(QualityBase, table=True):
    """📏 A quality is meta-data for decision making."""

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
    typePk: typing.Optional[int] = sqlmodel.Field(
        alias="typeId",
        sa_column=sqlmodel.Column(
            "typeId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="qualities")
    # designPk: typing.Optional[int] = sqlmodel.Field(
    #     alias="designId",
    #     sa_column=sqlmodel.Column(
    #         "designId",
    #         sqlalchemy.Integer(),
    #         sqlalchemy.ForeignKey("design.id"),
    #     ),
    #     default=None,
    #     exclude=True,
    # )
    # design: typing.Optional["Design"] = sqlmodel.Relationship(
    #     back_populates="qualities"
    # )
    # __table_args__ = (
    #     sqlalchemy.CheckConstraint(
    #         "typeId IS NOT NULL AND designId IS NULL OR typeId IS NULL AND designId IS NOT NULL",
    #         name="typeOrDesignSet",
    #     ),
    #     sqlalchemy.UniqueConstraint("name", "typeId", "designId"),
    # )

    def parent(self) -> "Type":
        if self.type is None:
            raise NoTypeAssigned()
        return self.type


class QualityId(NamedId):
    pass


class QualityDto(QualityBase, Dto):
    pass


class QualityProps(QualityDto, Props):
    pass


class QualityInput(QualityDto, Input):
    pass


class QualityOutput(QualityDto, Output):
    pass


class TypeBase(VariableArtifact):
    pass


class Type(TypeBase, Model, table=True):
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
    representations: list[Representation] = sqlmodel.Relationship(
        back_populates="type",
        cascade_delete=True,
    )
    ports: list[Port] = sqlmodel.Relationship(
        back_populates="type", cascade_delete=True
    )
    qualities: list[Quality] = sqlmodel.Relationship(
        back_populates="type", cascade_delete=True
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
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="types")

    # __table_args__ = (sqlalchemy.UniqueConstraint('name', 'variant', 'kitPk'),)

    def parent(self) -> "Kit":
        if self.kit is None:
            raise NoKitAssigned()
        return self.kit

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Type", input: str | dict | typing.Any) -> "Type":
        obj = (
            json.loads(input)
            if isinstance(input, str)
            else input if isinstance(input, dict) else input.__dict__
        )
        props = TypeProps.model_validate(obj)
        model = cls(**props.model_dump())
        try:
            representations = [Representation.parse(r) for r in obj["representations"]]
            model.representations = representations
        except KeyError:
            pass
        try:
            ports = [Port.parse(p) for p in obj["ports"]]
            model.ports = ports
        except KeyError:
            pass
        try:
            qualities = [Quality.parse(q) for q in obj["qualities"]]
            model.qualities = qualities
        except KeyError:
            pass
        return model

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


class TypeId(VariableArtifactId):
    pass


class TypeDto(TypeBase, Dto):
    pass


class TypeProps(TypeDto, Props):
    pass


class TypeInput(TypeDto, Input):
    representations: list[RepresentationInput] = sqlmodel.Field(default_factory=list)


class TypeOutput(TypeDto, Artifact, Output):
    representations: list[RepresentationOutput] = sqlmodel.Field(default_factory=list)


# class PieceBase(IdentifiedModel):

#     pass


# class Piece(PieceBase, Model, table=True):
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
#     id_: str = sqlmodel.Field(
#         alias="id",
#         sa_column=sqlmodel.Column(
#             "localId",
#             sqlalchemy.String(NAME_LENGTH_LIMIT),
#         ),
#     )
#     planePk: typing.Optional[int] = sqlmodel.Field(
#         alias="planeId",
#         sa_column=sqlmodel.Column(
#             "planeId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("plane.id"),
#         ),
#         default=None,
#         exclude=True,
#     )
#     plane: typing.Optional[Plane] = sqlmodel.Relationship(back_populates="piece")
#     screenPointX: int = sqlmodel.Field(exclude=True)
#     screenPointY: int = sqlmodel.Field(exclude=True)
#     designPk: typing.Optional[int] = sqlmodel.Field(
#         alias="designId",
#         sa_column=sqlmodel.Column(
#             "designId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("design.id"),
#         ),
#         default=None,
#         exclude=True,
#     )
#     design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="pieces")
#     connecteds: list["Connection"] = sqlmodel.Relationship(
#         back_populates="connectedPiece",
#         sa_relationship_kwargs={"foreign_keys": "Connection.connectedPiecePk"},
#     )
#     connectings: list["Connection"] = sqlmodel.Relationship(
#         back_populates="connectingPiece",
#         sa_relationship_kwargs={"foreign_keys": "Connection.connectingPiecePk"},
#     )

#     __table_args__ = (
#         sqlalchemy.UniqueConstraint("localId", "designId"),
#         sqlalchemy.CheckConstraint(
#             """
#             (
#                 (planeOriginX IS NULL AND planeOriginY IS NULL AND planeOriginZ IS NULL AND
#                  planeXAxisX IS NULL AND planeXAxisY IS NULL AND planeXAxisZ IS NULL AND
#                  planeYAxisX IS NULL AND planeYAxisY IS NULL AND planeYAxisZ IS NULL)
#             OR
#                 (planeOriginX IS NOT NULL AND planeOriginY IS NOT NULL AND planeOriginZ IS NOT NULL AND
#                  planeXAxisX IS NOT NULL AND planeXAxisY IS NOT NULL AND planeXAxisZ IS NOT NULL AND
#                  planeYAxisX IS NOT NULL AND planeYAxisY IS NOT NULL AND planeYAxisZ IS NOT NULL)
#             )
#             """,
#             name="planeSetOrNotSet",
#         ),
#     )

#     @property
#     def plane(self) -> Plane:
#         return Plane(
#             origin=Point(
#                 self.planeOriginX,
#                 self.planeOriginY,
#                 self.planeOriginZ,
#             ),
#             xAxis=Vector(
#                 self.planeXAxisX,
#                 self.planeXAxisY,
#                 self.planeXAxisZ,
#             ),
#             yAxis=Vector(
#                 self.planeYAxisX,
#                 self.planeYAxisY,
#                 self.planeYAxisZ,
#             ),
#         )

#     @plane.setter
#     def plane(self, plane: Plane):
#         self.planeOriginX = plane.origin.x
#         self.planeOriginY = plane.origin.y
#         self.planeOriginZ = plane.origin.z
#         self.planeXAxisX = plane.xAxis.x
#         self.planeXAxisY = plane.xAxis.y
#         self.planeXAxisZ = plane.xAxis.z
#         self.planeYAxisX = plane.yAxis.x
#         self.planeYAxisY = plane.yAxis.y
#         self.planeYAxisZ = plane.yAxis.z

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


# class PieceOutput(PieceBase):
#     id_: str = sqlmodel.Field(alias="id")
#     plane: Plane = sqlmodel.Field(default_factory=Plane)
#     screenPoint: ScreenPoint = sqlmodel.Field(default_factory=ScreenPoint)


# class PieceIdOutput(sqlmodel.SQLModel):
#     id_: str = sqlmodel.Field(alias="id")


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


# class Connection(ConnectionBase, Model, table=True):
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


# class Design(DesignBase, Model, table=True):
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
#             .filter(Design.name == decodeString(localId[0]) if decode else localId[0])
#             .first()
#         )


# class DesignOutput(DesignBase):
#     class Config:
#         title = "Design"

#     pieces: list[PieceOutput] = sqlmodel.Field(default_factory=list)
#     connections: list[ConnectionOutput] = sqlmodel.Field(default_factory=list)


class KitBase(ArtifactBase):
    remote: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT, default="")
    homepage: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT, default="")


class Kit(KitBase, Model, table=True):
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
    types: list[Type] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    # designs: list[Design] = sqlmodel.Relationship(
    #     back_populates="kit", cascade_delete=True
    # )

    __table_args__ = (sqlalchemy.UniqueConstraint("name"),)

    # def guid(self) -> str:
    #     return encodeString(self.remote)

    # TODO: Automatic nested parsing (https://github.com/fastapi/sqlmodel/issues/293)
    @classmethod
    def parse(cls: "Kit", input: str | dict | typing.Any) -> "Kit":
        obj = json.loads(input) if isinstance(input, str) else input
        props = KitProps.model_validate(obj)
        model = cls(**props.model_dump())
        types = [Type.parse(t) for t in input.types]
        model.types = types
        return model

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

    # TODO: Make sure that kit guid has url not remote url as base.
    # def guid(self) -> str:
    #     return encodeString(self.remote)


class KitId(UrledModel):
    pass


class KitDto(KitBase, Dto):
    pass


class KitInput(KitDto, Input):
    types: list[TypeInput] = sqlmodel.Field(default_factory=list)


class KitProps(KitDto, Props):
    pass


class KitOutput(KitDto, Artifact, Output):
    types: list[TypeOutput] = sqlmodel.Field(default_factory=list)


# class KitOutput(KitDto, Output):
#     types: list[TypeId] = sqlmodel.Field(default_factory=list)

# class KitFullOutput(KitDto, Output):
#     types: list[TypeOutput] = sqlmodel.Field(default_factory=list)


# store


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
    # a2l0LnNxbGl0ZTM=
    # a2l0LnNxbGl0ZTM=/types
    # a2l0LnNxbGl0ZTM=/types/Q2Fwc3VsZQ==,
    # a2l0LnNxbGl0ZTM=/types/Q2Fwc3VsZQ==,/representations
    # a2l0LnNxbGl0ZTM=/types/Q2Fwc3VsZQ==,/representations/aHR0cHM6Ly9hcHAuc3BlY2tsZS5zeXN0ZW1zL3Byb2plY3RzL2U3ZGUxYTJmOGYvbW9kZWxzL2IzYzIwZGI5NzA=
    # kit.sqlite3/types/Capsule,/representations/https://app.speckle.systems/projects/e7de1a2f8f/models/b3c20db970
    # url: kit.sqlite3
    # kind: type
    # typeName: Capsule
    # typeVariant: ""
    # representationUrl: https://app.speckle.systems/projects/e7de1a2f8f/models/b3c20db970

    def code(self, children):
        if len(children) == 0:
            return {"kind": "kits"}
        kitUrl = decodeString(children[0].value)
        if len(children) == 1:
            return {"kind": "kit", "kitUrl": kitUrl}
        code = children[1]
        code["kitUrl"] = kitUrl
        return code

    def type(self, children):
        if len(children) == 0:
            return {"kind": "types"}
        return {
            "kind": "type",
            "typeName": decodeString(children[0].value),
            "typeVariant": (
                decodeString(children[1].value) if len(children) == 2 else ""
            ),
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
            code["representationUrl"] = decodeString(children[1].value)

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
            code["portUrl"] = decodeString(children[1].value)
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
    url: str

    def __init__(self, url: str) -> None:
        self.url = url

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

    def __init__(self, url: str, engine: sqlalchemy.engine.Engine) -> None:
        super().__init__(url)
        self.engine = engine

    @functools.cached_property
    def session(self: "DatabaseStore") -> sqlalchemy.orm.Session:
        return sqlalchemy.orm.sessionmaker(bind=self.engine)()

    @classmethod
    @abc.abstractmethod
    def fromUrl(cls: "DatabaseStore", url: str) -> "DatabaseStore":
        """🔧 Get a store from the url."""
        pass

    @abc.abstractmethod
    def kitNameByUrl(cls: "DatabaseStore", url: str) -> str | None:
        """📛 Get the name of the kit from the url."""
        pass

    def get(self: "DatabaseStore", operation: dict) -> typing.Any:
        kitUrl = operation["kitUrl"]
        kind = operation["kind"]
        kitName = self.kitNameByUrl(kitUrl)
        if kitName is None:
            raise KitNotFound(kitUrl)
        match kind:
            case "kit":
                return self.session.query(Kit).filter(Kit.name == kitName).one_or_none()
            case "type":
                return (
                    self.session.Query(Type, Kit)
                    .filter(
                        Kit.name == kitName,
                        Type.name == operation["typeName"],
                        Type.variant == operation["typeVariant"],
                    )
                    .one_or_none()
                )
            case _:
                raise ServerError(f"Unknown kind: {kind}")

    def put(self: "DatabaseStore", operation: dict, input: typing.Any) -> typing.Any:
        kitUrl = operation["kitUrl"]
        kind = operation["kind"]
        match kind:
            case "kit":
                self.initialize()
                kit = Kit.parse(input)
                existingKit = (
                    self.session.query(Kit).filter(Kit.name == input.name).one_or_none()
                )
                if existingKit is not None:
                    raise KitAlreadyExists(kitUrl)
                try:
                    self.session.add(kit)
                    self.session.commit()
                except Exception as e:
                    self.session.rollback()
                    raise e
                return kit
            case _:
                raise FeatureNotYetImplemented()

    def update(self: "DatabaseStore", operation: dict, input: str) -> typing.Any:
        raise FeatureNotYetImplemented()

    def delete(self: "DatabaseStore", operation: dict) -> typing.Any:
        kitUrl = operation["kitUrl"]
        kind = operation["kind"]
        kitName = self.kitNameByUrl(kitUrl)
        match kind:
            case "kit":
                kit = self.session.query(Kit).filter(Kit.name == kitName).one_or_none()
                if kit is None:
                    raise KitNotFound(kitUrl)
                try:
                    self.session.delete(kit)
                    self.session.commit()
                except Exception as e:
                    self.session.rollback()
                    raise e


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
        self, url: str, engine: sqlalchemy.engine.Engine, path: pathlib.Path
    ) -> None:
        super().__init__(url, engine)
        self.path = path

    @classmethod
    def fromUrl(cls, url: str) -> "SqliteStore":
        parsedUrl = urllib.parse.urlparse(url)
        if not parsedUrl.path.endswith(".sqlite3"):
            if not parsedUrl.path.endswith(".semio"):
                path = pathlib.Path(parsedUrl.path) / KIT_FOLDERNAME / KIT_FILENAME
            else:
                path = pathlib.Path(parsedUrl.path) / KIT_FILENAME
        else:
            path = pathlib.Path(parsedUrl.path)
        pathString = str(path)
        connectionString = (
            url
            if parsedUrl.scheme == "sqlite"
            else (
                f"sqlite:///{pathString}"
                if not pathString.startswith("/")
                else f"sqlite://{pathString}"
            )
        )
        engine = sqlalchemy.create_engine(connectionString, echo=True)
        return SqliteStore(url, engine, path)

    def kitNameByUrl(self: "SqliteStore", url: str) -> str | None:
        kit = self.session.query(Kit).one_or_none()
        return kit.name if kit is not None else None

    def initialize(self: "DatabaseStore") -> None:
        os.makedirs(
            pathlib.Path(self.url) / pathlib.Path(KIT_FOLDERNAME), exist_ok=True
        )
        sqlmodel.SQLModel.metadata.create_all(self.engine)


class PostgresStore(DatabaseStore):

    @classmethod
    def fromUrl(cls, url: str):
        parsedUrl = urllib.parse.urlparse(url)
        connection_string = sqlalchemy.URL.create(
            "postgresql+psycopg",
            username=parsedUrl.username,
            password=parsedUrl.password,
            host=parsedUrl.hostname,
            database=parsedUrl.path[1:],  # Remove the leading '/'
        )
        engine = sqlalchemy.create_engine(
            connection_string,
            connect_args={"sslmode": parsedUrl.query.get("sslmode", SSLMode.REQUIRE)},
        )
        return PostgresStore(url, engine)

    @classmethod
    def kitNameByUrl(self: "PostgresStore", url: str) -> str | None:
        raise FeatureNotYetImplemented()

    def initialize(self: "DatabaseStore") -> None:
        sqlmodel.SQLModel.metadata.create_all(self.engine)


# class ApiStore(Store, abc.ABC):
# pass


# class RestStore(ApiStore):
# pass


# class GraphqlStore(Store):
# pass


# The cache is necissary to persist the session!
# An other option would be to eager load the relationships.
@functools.lru_cache
def StoreFactory(url: str) -> Store:
    """🏭 Get a store from the url."""
    try:
        return SqliteStore.fromUrl(url)
    except Exception as e:
        pass
    raise KitNotFound(url)


def storeAndOperationFromCode(code: str) -> tuple[Store, dict]:
    codeTree = codeParser.parse(code)
    operation = OperationBuilder().transform(codeTree)
    store = StoreFactory(operation["kitUrl"])
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


# engine

## graphql

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


class NodeNode(graphene.relay.Node):

    class Meta:
        name = "Node"

    @staticmethod
    def to_global_id(type_, id):
        return id

    @staticmethod
    def get_node_from_global_id(info, global_id, only_type=None):
        entity = get(global_id)
        return entity


class ModelNode(graphene_sqlalchemy.SQLAlchemyObjectType):
    """A base class for all graphql nodes.
    It automatically excludes the fields of the base and adds resolvers to all @properties.
    Relationships are by default included.
    """

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
        if "interfaces" not in options:
            options["interfaces"] = (NodeNode,)

        own_properties = [
            name
            for name, value in model.__dict__.items()
            if isinstance(value, property)
        ]

        def make_resolver(name):
            def resolver(self, info):
                return getattr(self, name)

            return resolver

        # Dynamically add resolvers for all properties
        for name in own_properties:
            prop = getattr(model, name)
            prop_getter = prop.fget
            prop_return_type = inspect.signature(prop_getter).return_annotation
            setattr(cls, name, GRAPHQLTYPES[prop_return_type])
            setattr(cls, f"resolve_{name}", make_resolver(name))

        def resolver_guid(self, info):
            return self.guid()

        setattr(cls, "resolve_id", resolver_guid)

        super().__init_subclass_with_meta__(model=model, **options)


class InputNode(graphene_pydantic.PydanticInputObjectType):
    """A base class for all graphql input nodes."""

    class Meta:
        abstract = True


class RepresentationNode(ModelNode):
    class Meta:
        model = Representation


class RepresentationInputNode(InputNode):
    class Meta:
        model = RepresentationInput


class LocatorNode(ModelNode):
    class Meta:
        model = Locator


class LocatorInputNode(InputNode):
    class Meta:
        model = LocatorInput


# class ScreenPointNode(graphene_pydantic.PydanticObjectType):
#     class Meta:
#         model = ScreenPoint


# class ScreenPointInputNode(InputNode):
#     class Meta:
#         model = ScreenPointInput


class PointNode(graphene_pydantic.PydanticObjectType):
    class Meta:
        model = Point


class PointInputNode(InputNode):
    class Meta:
        model = Point


class VectorNode(graphene_pydantic.PydanticObjectType):
    class Meta:
        model = Vector


class VectorInputNode(InputNode):
    class Meta:
        model = Vector


# class PlaneNode(graphene_pydantic.PydanticObjectType):
#     class Meta:
#         model = Plane


# class PlaneInputNode(InputNode):
#     class Meta:
#         model = Plane


class PortNode(ModelNode):
    class Meta:
        model = Port


class PortInputNode(InputNode):
    class Meta:
        model = Port


class PortIdInputNode(InputNode):
    class Meta:
        model = PortId


class QualityNode(ModelNode):
    class Meta:
        model = Quality


class QualityInputNode(InputNode):
    class Meta:
        model = Quality


class TypeNode(ModelNode):
    class Meta:
        model = Type


class TypeInputNode(InputNode):
    class Meta:
        model = TypeInput


# class PieceNode(ModelNode):
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


# class SidePieceTypeNode(graphene_pydantic.PydanticObjectType):
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


# class SidePieceNode(graphene_pydantic.PydanticObjectType):
#     class Meta:
#         model = SidePiece


# class SidePieceInputNode(InputNode):
#     class Meta:
#         model = SidePieceInput


# class SideNode(graphene_pydantic.PydanticObjectType):
#     class Meta:
#         model = Side


# class SideInputNode(InputNode):
#     class Meta:
#         model = SideInput


# class ConnectionNode(ModelNode):
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


# class DesignNode(ModelNode):
#     class Meta:
#         model = Design


# class DesignInputNode(InputNode):
#     class Meta:
#         model = DesignInput


class KitNode(ModelNode):
    class Meta:
        model = Kit


# # Can't use SQLAlchemyConnectionField because only supports one database.
# # https://github.com/graphql-python/graphene-sqlalchemy/issues/180
# class KitConnection(graphene.relay.Connection):
#     class Meta:
#         node = KitNode


class KitInputNode(InputNode):
    class Meta:
        model = KitInput


class Query(graphene.ObjectType):
    node = NodeNode.Field()
    kit = graphene.Field(KitNode, url=graphene.String(required=True))
    # kits = graphene.relay.ConnectionField(KitConnection)

    def resolve_kit(self, info, url):
        return get(encodeString(url))


class Mutation(graphene.ObjectType):
    createKit = graphene.Field(KitNode, kit=KitInputNode(required=True))


## rest

rest = fastapi.FastAPI(max_request_body_size=MAX_REQUEST_BODY_SIZE)


@rest.get("/kits/{encodedKitUrl}")
async def kit(
    request: fastapi.Request,
    encodedKitUrl: ENCODED_PATH,
) -> KitOutput:
    try:
        return get(request.url.path.removeprefix("/kits/"))
    except KitNotFound as e:
        statusCode = 404
        error = e
    except ClientError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.put("/kits/{encodedKitUrl}")
async def create_kit(
    request: fastapi.Request,
    encodedKitUrl: ENCODED_PATH,
    input: KitInput,
) -> None:
    try:
        put(request.url.path.removeprefix("/kits/"), input)
        return None
    except SpecificationError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.delete("/kits/{encodedKitUrl}")
async def delete_kit(
    request: fastapi.Request,
    encodedKitUrl: ENCODED_PATH,
) -> None:
    try:
        delete(request.url.path.removeprefix("/kits/"))
        return None
    except KitNotFound as e:
        statusCode = 404
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.put("/kits/{encodedKitUrl}/types/{encodedTypeName},{encodedTypeVariant}")
async def put_type(
    request: fastapi.Request,
    encodedKitUrl: ENCODED_PATH,
    encodedTypeName: ENCODED_PATH,
    encodedTypeVariant: ENCODED_PATH,
    input: TypeInput,
) -> None:
    try:
        put(request.url.path.removeprefix("/kits/"), input)
        return None
    except KitNotFound as e:
        statusCode = 404
        error = e
    except SpecificationError as e:
        statusCode = 400
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


@rest.delete("/kits/{encodedKitUrl}/types/{encodedTypeName},{encodedTypeVariant}")
async def delete_type(
    request: fastapi.Request,
    encodedKitUrl: ENCODED_PATH,
    encodedTypeName: ENCODED_PATH,
    encodedTypeVariant: ENCODED_PATH,
) -> None:
    try:
        delete(request.url.path.removeprefix("/kits/"))
        return None
    except NotFound as e:
        statusCode = 404
        error = e
    except Exception as e:
        statusCode = 500
        error = e
    return fastapi.Response(content=str(error), status_code=statusCode)


schema = graphene.Schema(
    query=Query,
    mutation=Mutation,
)

engine = starlette.applications.Starlette()
engine.mount(
    "/graphql",
    starlette_graphene3.GraphQLApp(
        schema, on_get=starlette_graphene3.make_graphiql_handler()
    ),
)
engine.mount("/", rest)


def start_engine(debug: bool = False):

    if debug:
        if not os.path.exists("debug"):
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
        schema = cursor.fetchall()
        with open(sqliteSchemaPath, "w", encoding="utf-8") as f:
            for table in schema:
                f.write(f"{table[0]};\n")
        conn.close()

        # write graphql schema to file
        with open("../../graphql/schema.graphql", "w", encoding="utf-8") as f:
            f.write(str(schema))

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
