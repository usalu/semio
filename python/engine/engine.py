#!/usr/bin/env python

# semio engine.
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
semio engine.
"""
# TODO: Refactoring Error handling by only exposing client__str__ and not __str__.
#       Write better error messages.
# TODO: Check if sqlmodel can replace SQLAlchemy:
#       ✅Constraints
#       ❔Polymorphism
#       ❔graphene_sqlalchemy
#       ❔graphene_pydantic
# TODO: Uniformize naming.
# TODO: Check graphene_pydantic until the pull request for pydantic>2 is merged.
# TODO: Add constraint to formations that at least 2 pieces and 1 connection are required.
# TODO: Make uvicorn pyinstaller multiprocessing work. Then qt can be integrated again for system tray.

from argparse import ArgumentParser
import os
import logging  # for uvicorn in pyinstaller
from os import remove
from pathlib import Path
from multiprocessing import freeze_support
from functools import lru_cache
from typing import Optional, Dict, Protocol, List, Union
from datetime import datetime
from urllib.parse import urlparse
from json import dumps
from numpy import ndarray, asarray, eye, dot, cross, radians, degrees
from pytransform3d.transformations import (
    concat,
    invert_transform,
    transform_from,
    transform,
    vector_to_point,
    vector_to_direction,
)
from pytransform3d.rotations import (
    matrix_from_axis_angle,
    axis_angle_from_matrix,
    axis_angle_from_two_directions,
)
from networkx import (
    Graph,
    bfs_tree,
    connected_components,
)
from pint import UnitRegistry
from pydantic import BaseModel, ValidationError, field_serializer
from sqlalchemy import (
    String,
    Text,
    Float,
    DateTime,
    ForeignKey,
    create_engine,
    CheckConstraint,
    UniqueConstraint,
    event,
    inspect,
)
from sqlalchemy.orm import (
    DeclarativeBase,
    Mapped,
    mapped_column,
    relationship,
    sessionmaker,
    Session,
    validates,
)
from sqlalchemy.exc import IntegrityError, MultipleResultsFound
import graphene
from graphene import Schema, Mutation, ObjectType, InputObjectType, Field, NonNull
from graphene_sqlalchemy import (
    SQLAlchemyObjectType,
)
from graphene_pydantic import PydanticObjectType, PydanticInputObjectType
from uvicorn import run
from starlette.applications import Starlette
from starlette_graphene3 import GraphQLApp, make_graphiql_handler

# from PySide6.QtWidgets import (
#     QApplication,
#     QSystemTrayIcon,
#     QMenu,
# )
# from PySide6.QtCore import QSize
# from PySide6.QtGui import (
#     QIcon,
#     QAction,
# )

logging.basicConfig(level=logging.INFO)  # for uvicorn in pyinstaller

NAME_LENGTH_MAX = 100
URL_LENGTH_MAX = 1000
KIT_FOLDERNAME = ".semio"
KIT_FILENAME = "kit.sqlite3"
HOST = "127.0.0.1"
PORT = 5052
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

ureg = UnitRegistry()


class SemioException(Exception):
    """❗ The base class for all exceptions in semio."""

    pass


class SpecificationError(SemioException):
    """🚫 The base class for all specification errors.
    A specification error is when the user input does not respect the specification."""

    pass


class InvalidURL(ValueError, SpecificationError):
    """🚫🔗 The URL is not valid. An url must have the form:
    scheme://netloc/path;parameters?query#fragment."""

    def __init__(self, url: str) -> None:
        self.url = url

    def __str__(self) -> str:
        return f"{self.url} is not a valid URL."


class InvalidDatabase(SemioException):
    """🚫💾 The state of the database is somehow invalid.
    Check the constraints and the insert validators.
    """

    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + "\n The database is invalid. Please report this bug."


class InvalidBackend(SemioException):
    """🚫🖥️ The backend processed something wrong. Check the order of operations."""

    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + "\n The backend is invalid. Please report this bug."


class Entity(Protocol):
    """💾 An entity is anything that is captured in the persistance layer."""

    @property
    def parent(self) -> Union["Entity", None]:
        return None

    @property
    def children(self) -> List["Entity"]:
        return []

    @property
    def references(self) -> List["Entity"]:
        return []

    @property
    def referencedBy(self) -> List["Entity"]:
        return []

    @property
    def relatedTo(self) -> List["Entity"]:
        return (
            ([self.parent] if self.parent else [])
            + self.children
            + self.references
            + self.referencedBy
        )

    def client__str__(self) -> str:
        """🥸 A string representation of the entitity for the client."""
        pass


def client__str__List(entities: Entity) -> str:
    """🥸 Get a string representation of a list of entitities for the client.

    Args:
        entities (Entity): The list of entitities.

    Returns:
        str: String representation.
    """
    return f"[{', '.join([e.client__str__() for e in entities])}]"


# TODO: Refactor Protocol to ABC and make it work with SQLAlchemy
class Artifact(Entity):
    """♻️ An artifact is anything that is worth to be reused."""

    name: str
    description: Optional[str] = None
    icon: Optional[str] = None


class Base(DeclarativeBase):
    pass


class Tag(Base):
    """🏷️ A tag is meta-data for grouping representations."""

    __tablename__ = "tag"

    value: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(value) > 0", name="valueSet"),
        primary_key=True,
    )
    representationId: Mapped[int] = mapped_column(
        ForeignKey("representation.id"), primary_key=True
    )
    representation: Mapped["Representation"] = relationship(
        "Representation", back_populates="_tags"
    )

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Tag):
    #         raise NotImplementedError()
    #     return self.value == other.value

    # def __hash__(self) -> int:
    #     return hash(self.value)

    def __repr__(self) -> str:
        return f"Tag(value={self.value!r}, representationId={self.representationId!r})"

    def __str__(self) -> str:
        return f"Tag(value={self.value}, representationId={str(self.representationId)})"

    def client__str__(self) -> str:
        return f"{self.value}"

    # @property
    # def parent(self) -> Entity:
    #     return self.representation

    # @property
    # def children(self) -> List[Entity]:
    #     return []

    # @property
    # def references(self) -> List[Entity]:
    #     return []

    # @property
    # def referencedBy(self) -> List[Entity]:
    #     return []

    # @property
    # def relatedTo(self) -> List[Entity]:
    #     return [self.parent]

def parseMimeFromUrl(url: str) -> str:
    """🔍 Parse the mime type from the URL.

    Args:
        url (str): The URL.

    Returns:
        str: The mime type.
    """
    try:
        return MIMES[Path(url).suffix]
    except KeyError:
        return "application/octet-stream"

class Representation(Base):
    """💾 A representation is a link to a file that describes a type for a certain level of detail and tags."""

    __tablename__ = "representation"
    id: Mapped[int] = mapped_column(primary_key=True)
    url: Mapped[str] = mapped_column(
        String(URL_LENGTH_MAX),
        CheckConstraint("length(url) > 0", name="urlSet"),
    )
    mime: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(mime) > 0", name="mimeSet"),
    )
    # level of detail/development/design/...
    # "" means the defaut lod.
    lod: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    typeId: Mapped[int] = mapped_column(ForeignKey("type.id"), nullable=False)
    type: Mapped["Type"] = relationship("Type", back_populates="representations")
    _tags: Mapped[List[Tag]] = relationship(
        Tag, back_populates="representation", cascade="all, delete-orphan"
    )

    __table_args__ = (UniqueConstraint("typeId", "url"),)

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Representation):
    #         raise NotImplementedError()
    #     return self.url == other.url

    # def __hash__(self) -> int:
    #     return hash(self.url)

    def __repr__(self) -> str:
        return f"Representation(id={self.id!r}, url={self.url!r}, mime={self.mime!r}, lod={self.lod!r}, typeId={self.typeId!r}, tags={self.tags!r})"

    def __str__(self) -> str:
        return f"Representation(id={str(self.id)}, typeId={str(self.typeId)})"

    def client__str__(self) -> str:
        return f"Representation(url={self.url})"

    @validates("url")
    def validate_url(self, key: str, url: str):
        parsed = urlparse(url)
        if not parsed.path:
            raise InvalidURL(url)
        return url

    @property
    def tags(self) -> List[str]:
        return [tag.value for tag in self._tags or []]

    @tags.setter
    def tags(self, tags: List[str]):
        self._tags = [Tag(value=tag) for tag in tags]

    # @property
    # def parent(self) -> Entity:
    #     return self.type

    # @property
    # def children(self) -> List[Entity]:
    #     return self._tags  # type: ignore

    # @property
    # def references(self) -> List[Entity]:
    #     return []

    # @property
    # def referencedBy(self) -> List[Entity]:
    #     return []

    # @property
    # def relatedTo(self) -> List[Entity]:
    #     return [self.parent] + self.children if self.children else []


class Locator(Base):
    """🗺️ A locator is meta-data for grouping ports."""

    __tablename__ = "locator"

    group: Mapped[str] = mapped_column(
        "group_name",  # group is a reserved keyword in SQL
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(group_name) > 0", name="groupSet"),
        primary_key=True,
        key="group",
    )
    # Optional. "" means true.
    subgroup: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    portId: Mapped[int] = mapped_column(ForeignKey("port.id"), primary_key=True)
    port: Mapped["Port"] = relationship("Port", back_populates="locators")

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Locator):
            raise NotImplementedError()
        return self.group == other.group and self.subgroup == other.subgroup

    def __hash__(self) -> int:
        return hash((self.group, self.subgroup))

    def __repr__(self) -> str:
        return f"Locator(group={self.group!r}, subgroup={self.subgroup!r}, portId={self.portId!r})"

    def __str__(self) -> str:
        return f"Locator(group={self.group}, portId={str(self.portId)})"

    def client__str__(self) -> str:
        return f"Locator(group={self.group}, subgroup={self.subgroup})"

    # @property
    # def parent(self) -> Entity:
    #     return self.port

    # @property
    # def children(self) -> List[Entity]:
    #     return []

    # @property
    # def references(self) -> List[Entity]:
    #     return []

    # @property
    # def referencedBy(self) -> List[Entity]:
    #     return []

    # @property
    # def relatedTo(self) -> List[Entity]:
    #     return [self.parent]

def prettyNumber(number: float) -> str:
    if number == -0.0:
        number = 0.0
    return f"{number:.5f}".rstrip("0").rstrip(".")

class ScreenPoint(BaseModel):
    """📺 A 2d-point (xy) of integers in screen coordinate system."""

    x: int = 0
    y: int = 0

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


class Point(BaseModel):
    """✖️ A 3d-point (xyz) of floating point numbers."""

    x: float = 0.0
    y: float = 0.0
    z: float = 0.0

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"
    
    def __repr__(self) -> str:
        return f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"

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


class Vector(BaseModel):
    """➡️ A 3d-vector (xyz) of floating point numbers."""

    x: float = 0.0
    y: float = 0.0
    z: float = 0.0

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"
    
    def __repr__(self) -> str:
        return f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"

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
        return dot(self, other)

    def cross(self, other: "Vector") -> "Vector":
        return Vector(*cross(self, other))

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


class Plane(BaseModel):
    """◳ A plane is an origin (point) and an orientation (x-axis and y-axis)."""

    origin: Point
    xAxis: Vector
    yAxis: Vector

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

    def isCloseTo(self, other: "Plane", tol: float = TOLERANCE) -> bool:
        return (
            self.origin.isCloseTo(other.origin, tol)
            and self.xAxis.isCloseTo(other.xAxis, tol)
            and self.yAxis.isCloseTo(other.yAxis, tol)
        )

    @property
    def zAxis(self) -> Vector:
        return self.xAxis.cross(self.yAxis)

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


class Rotation(BaseModel):
    """🔄 A rotation is an axis and an angle."""

    axis: Vector
    angle: float

    def __init__(self, axis: Vector, angle: float):
        super().__init__(axis=axis, angle=angle)

    def toTransform(self) -> "Transform":
        return Transform.fromRotation(self)


class Transform(ndarray):
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
            axis=Vector(
                float(axisAngle[0]), 
                float(axisAngle[1]), 
                float(axisAngle[2])
            ), 
            angle=float(degrees(axisAngle[3]))
        )
    
    @property
    def translation(self) -> Vector:
        """➡️ The translation part of the transform."""
        return Vector(*self[:3, 3])
    
    # for pydantic
    def dict(self) -> Dict[str, Union[Rotation, Vector]]:
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
        self, geometry: Union[Point, Vector, Plane]
    ) -> Union[Point, Vector, Plane]:
        if isinstance(geometry, Point):
            return self.transformPoint(geometry)
        elif isinstance(geometry, Vector):
            return self.transformVector(geometry)
        elif isinstance(geometry, Plane):
            return self.transformPlane(geometry)
        else:
            raise NotImplementedError()

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
            transform_from(matrix_from_axis_angle((*rotation.axis, radians(rotation.angle))), Vector())
        )

    @staticmethod
    def fromPlane(plane: Plane) -> "Transform":
        # Assumes plane is normalized
        return Transform(
            transform_from(
                [
                    [plane.xAxis.x, plane.yAxis.x, plane.zAxis.x],
                    [plane.xAxis.y, plane.yAxis.y, plane.zAxis.y],
                    [plane.xAxis.z, plane.yAxis.z, plane.zAxis.z],
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


class Port(Base):
    """🔌 A port is a conceptual connection point (with a direction) of a type."""

    __tablename__ = "port"

    id: Mapped[int] = mapped_column(primary_key=True)
    # "" means the default port.
    localId: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    pointX: Mapped[float] = mapped_column(Float())
    pointY: Mapped[float] = mapped_column(Float())
    pointZ: Mapped[float] = mapped_column(Float())
    directionX: Mapped[float] = mapped_column(Float())
    directionY: Mapped[float] = mapped_column(Float())
    directionZ: Mapped[float] = mapped_column(Float())
    typeId: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped["Type"] = relationship("Type", back_populates="ports")
    locators: Mapped[List[Locator]] = relationship(
        Locator, back_populates="port", cascade="all, delete-orphan"
    )
    connecteds: Mapped[List["Connection"]] = relationship(
        "Connection",
        foreign_keys="[Connection.connectedPieceTypePortId]",
        back_populates="connectedPieceTypePort",
    )
    connectings: Mapped[List["Connection"]] = relationship(
        "Connection",
        foreign_keys="[Connection.connectingPieceTypePortId]",
        back_populates="connectingPieceTypePort",
    )

    # TODO: Add a constraint that the plane normal must be normalized.
    __table_args__ = (UniqueConstraint("localId", "typeId"),)

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Port):
    #         raise NotImplementedError()
    #     return self.origin == other.origin and self.normal == other.normal

    # def __hash__(self) -> int:
    #     return hash((self.origin, self.normal))

    def __repr__(self) -> str:
        return f"Port(id={self.id!r}, localId={self.localId!r}, planeOriginX={self.planeOriginX!r}, planeOriginY={self.planeOriginY!r}, planeOriginZ={self.planeOriginZ!r}, planeYAxisX={self.planeYAxisX!r}, planeYAxisY={self.planeYAxisY!r}, planeYAxisZ={self.planeYAxisZ!r}, typeId={self.typeId!r}, locators={self.locators!r})"

    def __str__(self) -> str:
        return f"Port(id={str(self.id)}, typeId={str(self.typeId)})"

    def client__str__(self) -> str:
        return f"Port(id={self.localId})"

    @property
    def point(self) -> Point:
        return Point(self.pointX, self.pointY, self.pointZ)

    @property
    def direction(self) -> Vector:
        return Vector(self.directionX, self.directionY, self.directionZ)

    @property
    def plane(self) -> Plane:
        return Plane.fromYAxis(self.direction, origin=self.point)

    # @property
    # def parent(self) -> Entity:
    #     return self.type

    # @property
    # def children(self) -> List[Entity]:
    #     return self.locators  # type: ignore

    # @property
    # def references(self) -> List[Entity]:
    #     return []

    # @property
    # def referencedBy(self) -> List[Entity]:
    #     return self.connectings + self.connecteds  # type: ignore

    # @property
    # def relatedTo(self) -> List[Entity]:
    #     return [self.parent] + self.children + self.referencedBy


class Quality(Base):
    """📏 A quality is meta-data for decision making."""

    __tablename__ = "quality"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(name) > 0", name="nameSet"),
    )
    # Optional. "" means true.
    value: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    # Optional. Set to "" for None.
    unit: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    # Optional. Set to "" for None.
    definition: Mapped[str] = mapped_column(Text())
    typeId: Mapped[Optional[int]] = mapped_column(ForeignKey("type.id"), nullable=True)
    type: Mapped["Type"] = relationship("Type", back_populates="qualities")
    formationId: Mapped[Optional[int]] = mapped_column(
        ForeignKey("formation.id"), nullable=True
    )
    formation: Mapped["Formation"] = relationship(
        "Formation", back_populates="qualities"
    )

    __table_args__ = (
        CheckConstraint(
            "typeId IS NOT NULL AND formationId IS NULL OR typeId IS NULL AND formationId IS NOT NULL",
            name="typeOrFormationSet",
        ),
        UniqueConstraint("name", "typeId", "formationId"),
    )

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Quality):
    #         raise NotImplementedError()
    #     if self.name == other.name:
    #         if self.unit == other.unit:
    #             return self.value == other.value
    #         # TODO: use pint to compare values with different units
    #         raise NotImplementedError(
    #             "Comparing values with different units is not implemented yet."
    #         )

    #     return False

    # def __hash__(self) -> int:
    #     # TODO: Implement unit normalization for consistent hashing
    #     return hash((self.name, self.value, self.unit))

    def __repr__(self) -> str:
        return f"Quality(id={self.id!r}, name={self.name}, value={self.value}, unit={self.unit}, definition={self.definition} typeId={self.typeId!r}, formationId={self.formationId!r})"

    def __str__(self) -> str:
        return f"Quality(id={str(self.id)}, typeId={str(self.typeId)}, formationId={str(self.formationId)})"

    def client__str__(self) -> str:
        return f"Quality(name={self.name})"

    # @property
    # def parent(self) -> Entity:
    #     return self.type if self.typeId else self.formation

    # @property
    # def children(self) -> List[Entity]:
    #     return []

    # @property
    # def references(self) -> List[Entity]:
    #     return []

    # @property
    # def referencedBy(self) -> List[Entity]:
    #     return []

    # @property
    # def relatedTo(self) -> List[Entity]:
    #     return [self.parent]


class Type(Base):
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    __tablename__ = "type"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(name) > 0", name="nameSet"),
    )
    # Optional. Set to "" for None.
    description: Mapped[str] = mapped_column(Text())
    # Optional. Set to "" for None.
    icon: Mapped[str] = mapped_column(Text())
    # Set to "" for default variant.
    variant: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    # The unit of the ports.
    unit: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(unit) > 0", name="unitSet"),
    )
    createdAt: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.now(), nullable=False
    )
    lastUpdateAt: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.now(), nullable=False, onupdate=datetime.now()
    )
    kitId: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="types")
    representations: Mapped[List[Representation]] = relationship(
        Representation, back_populates="type", cascade="all, delete-orphan"
    )
    ports: Mapped[List[Port]] = relationship(
        "Port", back_populates="type", cascade="all, delete-orphan"
    )
    qualities: Mapped[List[Quality]] = relationship(
        Quality, back_populates="type", cascade="all, delete-orphan"
    )
    pieces: Mapped[List["Piece"]] = relationship("Piece", back_populates="type")

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Type):
    #         raise NotImplementedError()
    #     return self.name == other.name and self.variant == other.variant

    # def __hash__(self) -> int:
    #     return hash((self.name, self.variant))

    def __repr__(self) -> str:
        return f"Type(id={self.id!r}, name={self.name}, description={self.description}, icon={self.icon}, variant={self.variant} unit={self.unit}, kitId={self.kitId!r}, representations={self.representations!r}, ports={self.ports!r}, qualities={self.qualities!r}, pieces={self.pieces!r})"

    def __str__(self) -> str:
        return f"Type(id={str(self.id)}, kitId={str(self.kitId)})"

    def client__str__(self) -> str:
        return f"Type(name={self.name}, variant={self.variant})"

    # @property
    # def parent(self) -> Entity:
    #     return self.kit

    # @property
    # def children(self) -> List[Entity]:
    #     return self.representations + self.ports + self.qualities  # type: ignore

    # @property
    # def references(self) -> List[Entity]:
    #     return []

    # @property
    # def referencedBy(self) -> List[Entity]:
    #     return [self.pieces]  # type: ignore

    # @property
    # def relatedTo(self) -> List[Entity]:
    #     return [self.parent] + self.children + self.referencedBy


@event.listens_for(Representation, "after_update")
def receive_after_update(mapper, connection, target):
    target.type.lastUpdateAt = datetime.now()


@event.listens_for(Port, "after_update")
def receive_after_update(mapper, connection, target):
    target.type.lastUpdateAt = datetime.now()

class TypeId(BaseModel):
    """🧩 A type is identified by a name and variant (empty=default)."""

    name: str
    variant: str = ""

class PieceRoot(BaseModel):
    """🌱 The root information of a piece."""

    plane: Plane


class PieceDiagram(BaseModel):
    """✏️ The diagram information of a piece."""

    point: ScreenPoint


class Piece(Base):
    """⭕ A piece is a 3d-instance of a type in a formation."""

    __tablename__ = "piece"

    id: Mapped[int] = mapped_column(primary_key=True)
    localId: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(localId) > 0", name="localIdSet"),
    )
    typeId: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped["Type"] = relationship("Type", back_populates="pieces")
    # When the piece is a root piece, the root plane is set.
    # Plane coordinates are in the units of the formation.
    rootPlaneOriginX: Mapped[Optional[float]] = mapped_column(Float(), nullable=True)
    rootPlaneOriginY: Mapped[Optional[float]] = mapped_column(Float(), nullable=True)
    rootPlaneOriginZ: Mapped[Optional[float]] = mapped_column(Float(), nullable=True)
    rootPlaneXAxisX: Mapped[Optional[float]] = mapped_column(Float(), nullable=True)
    rootPlaneXAxisY: Mapped[Optional[float]] = mapped_column(Float(), nullable=True)
    rootPlaneXAxisZ: Mapped[Optional[float]] = mapped_column(Float(), nullable=True)
    rootPlaneYAxisX: Mapped[Optional[float]] = mapped_column(Float(), nullable=True)
    rootPlaneYAxisY: Mapped[Optional[float]] = mapped_column(Float(), nullable=True)
    rootPlaneYAxisZ: Mapped[Optional[float]] = mapped_column(Float(), nullable=True)
    diagramPointX: Mapped[int] = mapped_column()
    diagramPointY: Mapped[int] = mapped_column()
    formationId: Mapped[int] = mapped_column(ForeignKey("formation.id"))
    formation: Mapped["Formation"] = relationship("Formation", back_populates="pieces")
    connectings: Mapped[List["Connection"]] = relationship(
        "Connection",
        foreign_keys="[Connection.connectingPieceId]",
        back_populates="connectingPiece",
    )
    connecteds: Mapped[List["Connection"]] = relationship(
        "Connection",
        foreign_keys="[Connection.connectedPieceId]",
        back_populates="connectedPiece",
    )

    __table_args__ = (
        UniqueConstraint("localId", "formationId"),
        CheckConstraint(
            """
            (
                (rootPlaneOriginX IS NULL AND rootPlaneOriginY IS NULL AND rootPlaneOriginZ IS NULL AND
                 rootPlaneXAxisX IS NULL AND rootPlaneXAxisY IS NULL AND rootPlaneXAxisZ IS NULL AND
                 rootPlaneYAxisX IS NULL AND rootPlaneYAxisY IS NULL AND rootPlaneYAxisZ IS NULL)
            OR
                (rootPlaneOriginX IS NOT NULL AND rootPlaneOriginY IS NOT NULL AND rootPlaneOriginZ IS NOT NULL AND
                 rootPlaneXAxisX IS NOT NULL AND rootPlaneXAxisY IS NOT NULL AND rootPlaneXAxisZ IS NOT NULL AND
                 rootPlaneYAxisX IS NOT NULL AND rootPlaneYAxisY IS NOT NULL AND rootPlaneYAxisZ IS NOT NULL)
            )
            """,
            name="rootPlaneSetOrNotSet",
        ),
    )

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Piece):
    #         raise NotImplementedError()
    #     return self.localId == other.localId

    # def __hash__(self) -> int:
    #     return hash(self.localId)

    def __repr__(self) -> str:
        return f"Piece(id={self.id!r}, localId={self.localId}, typeId={self.typeId!r}, rootPlaneOriginX={self.rootPlaneOriginX!r}, rootPlaneOriginY={self.rootPlaneOriginY!r}, rootPlaneOriginZ={self.rootPlaneOriginZ!r}, rootPlaneXAxisX={self.rootPlaneXAxisX!r}, rootPlaneXAxisY={self.rootPlaneXAxisY!r}, rootPlaneXAxisZ={self.rootPlaneXAxisZ!r}, rootPlaneYAxisX={self.rootPlaneYAxisX!r}, rootPlaneYAxisY={self.rootPlaneYAxisY!r}, rootPlaneYAxisZ={self.rootPlaneYAxisZ!r}, diagramPointX={self.diagramPointX!r}, diagramPointY={self.diagramPointY!r}, formationId={self.formationId!r})"

    def __str__(self) -> str:
        return f"Piece(id={str(self.id)}, formationId={str(self.formationId)})"

    def client__str__(self) -> str:
        return f"Piece(id={self.localId})"

    @property
    def root(self) -> PieceRoot | None:
        if self.rootPlaneOriginX is not None:
            return PieceRoot(
                plane=Plane(
                    Point(
                        self.rootPlaneOriginX,
                        self.rootPlaneOriginY,
                        self.rootPlaneOriginZ,
                    ),
                    Vector(
                        self.rootPlaneXAxisX,
                        self.rootPlaneXAxisY,
                        self.rootPlaneXAxisZ,
                    ),
                    Vector(
                        self.rootPlaneYAxisX,
                        self.rootPlaneYAxisY,
                        self.rootPlaneYAxisZ,
                    ),
                )
            )
        return None

    @property
    def diagram(self) -> PieceDiagram:
        return PieceDiagram(point=ScreenPoint(self.diagramPointX, self.diagramPointY))

    # for pydantic
    def dict(self) -> Dict[str, Union[PieceRoot, PieceDiagram]]:
        return {
            "id": self.localId,
            "type": TypeId(name=self.type.name, variant=self.type.variant),
            "root": self.root if self.root else None,
            "diagram": self.diagram,
        }
    
    # @property
    # def parent(self) -> Entity:
    #     return self.formation

    # @property
    # def children(self) -> List[Entity]:
    #     return []

    # @property
    # def references(self) -> List[Entity]:
    #     return self.type  # type: ignore

    # @property
    # def referencedBy(self) -> List[Entity]:
    #     return self.connectings + self.connecteds  # type: ignore

    # @property
    # def relatedTo(self) -> List[Entity]:
    #     return [self.parent] + self.references + self.referencedBy


class SidePieceType(BaseModel):
    """🧩 The type information of a piece of a side."""

    class Config:
        arbitrary_types_allowed = True

    port: Port


class SidePiece(BaseModel):
    """⭕ The piece information of a side. A piece is identified by an id (emtpy=default))."""

    id: str
    type: SidePieceType


class Side(BaseModel):
    """🧱 A side of a piece in a connection."""

    piece: SidePiece


class Connection(Base):
    """🖇️ A connection between two pieces of a formation."""

    __tablename__ = "connection"

    connectedPieceId: Mapped[int] = mapped_column(
        ForeignKey("piece.id"), primary_key=True
    )
    connectedPiece: Mapped[Piece] = relationship(
        Piece, foreign_keys=[connectedPieceId], back_populates="connecteds"
    )
    connectedPieceTypePortId = mapped_column(ForeignKey("port.id"))
    connectedPieceTypePort: Mapped[Port] = relationship(
        Port,
        foreign_keys=[connectedPieceTypePortId],
        back_populates="connecteds",
    )
    connectingPieceId: Mapped[int] = mapped_column(
        ForeignKey("piece.id"), primary_key=True
    )
    connectingPiece: Mapped[Piece] = relationship(
        Piece, foreign_keys=[connectingPieceId], back_populates="connectings"
    )
    connectingPieceTypePortId = mapped_column(ForeignKey("port.id"))
    connectingPieceTypePort: Mapped[Port] = relationship(
        Port,
        foreign_keys=[connectingPieceTypePortId],
        back_populates="connectings",
    )
    # Offset (unit of formation) in normal direction of the connected piece.
    offset: Mapped[float] = mapped_column(Float())
    # Rotation (degree) around the normal of the connected piece.
    rotation: Mapped[float] = mapped_column(
        Float(),
        CheckConstraint("rotation >= 0 AND rotation < 360", name="normalisedRotation"),
    )
    formationId: Mapped[int] = mapped_column(ForeignKey("formation.id"))
    formation: Mapped["Formation"] = relationship(
        "Formation", back_populates="connections"
    )

    __table_args__ = (
        CheckConstraint(
            "connectingPieceId != connectedPieceId",
            name="noReflexiveConnection",
        ),
    )

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Connection):
    #         raise NotImplementedError()
    #     return (
    #         self.connectingPiece == other.connectingPiece
    #         and self.connectedPiece == other.connectedPiece
    #     )

    # def __hash__(self) -> int:
    #     return hash((self.connectingPiece, self.connectedPiece))

    def __repr__(self) -> str:
        return f"Connection(connectedPieceId={self.connectedPieceId!r}, connectingPieceTypePortId={self.connectingPieceTypePortId!r}, connectingPieceId={self.connectingPieceId!r}, connectedPieceTypePortId={self.connectedPieceTypePortId!r}, offset={self.offset!r}, rotation={self.rotation!r}, formationId={self.formationId!r})"

    def __str__(self) -> str:
        return f"Connection(connectedPieceId={str(self.connectedPieceId)}, connectingPieceId={str(self.connectingPieceId)}, formationId={str(self.formationId)})"

    def client__str__(self) -> str:
        return f"Connection(connectedPieceId={self.connected.piece.id}, connectingPieceId={self.connecting.piece.id})"

    @property
    def connected(self) -> Side:
        return Side(
            piece=SidePiece(
                id=self.connectedPiece.localId,
                type=SidePieceType(
                    port=self.connectedPieceTypePort,
                ),
            )
        )
    
    @property
    def connecting(self) -> Side:
        return Side(
            piece=SidePiece(
                id=self.connectingPiece.localId,
                type=SidePieceType(
                    port=self.connectingPieceTypePort,
                ),
            )
        )


    # @property
    # def parent(self) -> Entity:
    #     return self.formation

    # @property
    # def children(self) -> List[Entity]:
    #     return []

    # @property
    # def references(self) -> List[Entity]:
    #     return [
    #         self.connectingPiece,
    #         self.connectedPiece,
    #         self.connectingPieceTypePort,
    #         self.connectedPieceTypePort,
    #     ]

    # @property
    # def referencedBy(self) -> List[Entity]:
    #     return []

    # @property
    # def relatedTo(self) -> List[Entity]:
    #     return [self.parent] + self.references


# TODO: Add complex validation before insert with networkx such as:
#       - only root pieces can have a plane.
class Formation(Base):
    """🏙️ A formation is a collection of pieces that are connected."""

    __tablename__ = "formation"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(name) > 0", name="nameSet"),
    )
    # Optional. Set to "" for None.
    description: Mapped[str] = mapped_column(Text())
    # Optional. Set to "" for None.
    icon: Mapped[str] = mapped_column(Text())
    # Set to "" for default variant.
    variant: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    # Unit of the root planes of the pieces.
    unit: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(unit) > 0", name="unitSet"),
    )
    createdAt: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.now(), nullable=False
    )
    lastUpdateAt: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.now(), nullable=False, onupdate=datetime.now()
    )
    kitId: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="formations")
    pieces: Mapped[List[Piece]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    connections: Mapped[List[Connection]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    qualities: Mapped[List[Quality]] = relationship(
        Quality, back_populates="formation", cascade="all, delete-orphan"
    )

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Formation):
    #         raise NotImplementedError()
    #     return is_isomorphic(self, other)

    def __repr__(self) -> str:
        return f"Formation(id={self.id!r}, name={self.name!r}, description={self.description!r}, icon={self.icon!r}, variant={self.variant!r}, kitId={self.kitId!r}, pieces={self.pieces!r}, connections={self.connections!r}, qualities={self.qualities!r})"

    def __str__(self) -> str:
        return f"Formation(id={str(self.id)}, kitId={str(self.kitId)})"

    def client__str__(self) -> str:
        return f"Formation(name={self.name}, qualities={client__str__List(self.qualities)})"

    # @property
    # def parent(self) -> Entity:
    #     return self.kit

    # @property
    # def children(self) -> List[Entity]:
    #     return self.pieces + self.connections  # type: ignore

    # @property
    # def references(self) -> List[Entity]:
    #     return []

    # @property
    # def referencedBy(self) -> List[Entity]:
    #     return []

    # @property
    # def relatedTo(self) -> List[Entity]:
    #     return [self.parent] + self.children


@event.listens_for(Piece, "after_update")
def receive_after_update(mapper, connection, target):
    target.formation.lastUpdateAt = datetime.now()


@event.listens_for(Connection, "after_update")
def receive_after_update(mapper, connection, target):
    target.formation.lastUpdateAt = datetime.now()


# Both Type and Formation can own qualities
@event.listens_for(Quality, "after_update")
def receive_after_update(mapper, connection, target):
    if target.typeId:
        target.type.lastUpdateAt = datetime.now()
    else:
        target.formation.lastUpdateAt = datetime.now()


class Hierarchy(BaseModel):
    class Config:
        arbitrary_types_allowed = True

    piece: Piece
    transform: Transform
    children: Optional[List["Hierarchy"]]

    @field_serializer('piece')
    def serialize_piece(self, piece: Piece, _info):
        return piece.dict()
    
    @field_serializer('transform')
    def serialize_transform(self, transform: Transform, _info):
        return transform.dict()


class Object(BaseModel):
    class Config:
        arbitrary_types_allowed = True

    piece: Piece
    plane: Plane
    parent: Optional["Object"]


class Scene(BaseModel):
    class Config:
        arbitrary_types_allowed = True

    formation: Formation
    objects: List[Object]


class Kit(Base):
    """🗃️ A kit is a collection of types and formations."""

    __tablename__ = "kit"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(name) > 0", name="nameSet"),
    )
    # Optional. Set to "" for None.
    description: Mapped[str] = mapped_column(Text())
    # Optional. Set to "" for None.
    icon: Mapped[str] = mapped_column(Text())
    createdAt: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.now(), nullable=False
    )
    lastUpdateAt: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.now(), nullable=False, onupdate=datetime.now()
    )
    # Optional. Set to "" for None.
    url: Mapped[str] = mapped_column(String(URL_LENGTH_MAX))
    # Optional. Set to "" for None.
    homepage: Mapped[str] = mapped_column(String(URL_LENGTH_MAX))
    types: Mapped[List[Type]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )
    formations: Mapped[List[Formation]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )

    __table_args__ = (UniqueConstraint("name"), UniqueConstraint("url"))

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Kit):
    #         raise NotImplementedError()
    #     return self.name == other.name

    # def __hash__(self) -> int:
    #     return hash(self.name)

    def __repr__(self) -> str:
        return f"Kit(id={self.id!r}, name={self.name!r}), description={self.description!r}, icon={self.icon!r}, url={self.url!r}, types={self.types!r}, formations={self.formations!r})"

    def __str__(self) -> str:
        return f"Kit(id={str(self.id)})"

    def client__str__(self) -> str:
        return f"Kit(name={self.name})"

    # @property
    # def parent(self) -> None:
    #     return None

    # @property
    # def children(self) -> List[Entity]:
    #     return self.types + self.formations  # type: ignore

    # @property
    # def references(self) -> List[Entity]:
    #     return []

    # @property
    # def referencedBy(self) -> List[Entity]:
    #     return []

    # @property
    # def relatedTo(self) -> List[Entity]:
    #     return self.children


class DirectoryError(SemioException):
    def __init__(self, directory: str):
        self.directory = directory


class DirectoryDoesNotExist(DirectoryError):
    def __str__(self) -> str:
        return "📁 Directory does not exist: " + self.directory


class DirectoryIsNotADirectory(DirectoryError):
    def __str__(self) -> str:
        return "📁 Directory is not a directory: " + self.directory


def assertDirectory(directory: Union[Path, str]) -> Path:
    if isinstance(directory, str):
        directory = Path(directory)
    if not directory.exists():
        raise DirectoryDoesNotExist(directory)  # type: ignore
    if not directory.is_dir():
        raise DirectoryIsNotADirectory(directory)  # type: ignore
    return directory.resolve()


@lru_cache(maxsize=100)
def getLocalSession(directory: str) -> Session:
    directory_path = assertDirectory(directory)
    engine = create_engine(
        "sqlite:///"
        + str(directory_path.joinpath(KIT_FOLDERNAME).joinpath(KIT_FILENAME)),
        # echo=True,
    )
    Base.metadata.create_all(engine)
    # Create instance of session factory
    return sessionmaker(bind=engine)()


# class ArtifactNode(graphene.Interface):
#     class Meta:
#         name = "Artifact"

#     name = NonNull(graphene.String)
#     description = NonNull(graphene.String)
#     icon = NonNull(graphene.String)
#     parent = graphene.Field(lambda: ArtifactNode)
#     children = NonNull(graphene.List(NonNull(lambda: ArtifactNode)))
#     references = NonNull(graphene.List(NonNull(lambda: ArtifactNode)))
#     referenced_by = NonNull(graphene.List(NonNull(lambda: ArtifactNode)))
#     related_to = NonNull(graphene.List(NonNull(lambda: ArtifactNode)))

#     def resolve_parent(artifact: "ArtifactNode", info):
#         return artifact.parent

#     def resolve_children(artifact: "ArtifactNode", info):
#         return artifact.children

#     def resolve_references(artifact: "ArtifactNode", info):
#         return artifact.references

#     def resolve_referenced_by(artifact: "ArtifactNode", info):
#         return artifact.referenced_by

#     def resolve_related_to(artifact: "ArtifactNode", info):
#         return artifact.related_to


class RepresentationNode(SQLAlchemyObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """💾 A representation is a link to a file that describes a type for a certain level of detail and tags."""

    class Meta:
        model = Representation
        name = "Representation"
        exclude_fields = (
            "id",
            "_tags",
            "typeId",
        )

    tags = NonNull(graphene.List(NonNull(graphene.String)))

    def resolve_tags(representation: Representation, info):
        return representation.tags


class ScreenPointNode(PydanticObjectType):

    class Meta:
        model = ScreenPoint
        name = "ScreenPoint"


class PointNode(PydanticObjectType):

    class Meta:
        model = Point
        name = "Point"


class VectorNode(PydanticObjectType):

    class Meta:
        model = Vector
        name = "Vector"


class PlaneNode(PydanticObjectType):

    class Meta:
        model = Plane
        name = "Plane"

    zAxis: VectorNode

    def resolve_port(plane: Plane, info):
        return plane.zAxis


class LocatorNode(SQLAlchemyObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """🗺️ A locator is meta-data for grouping ports."""

    class Meta:
        model = Locator
        name = "Locator"
        exclude_fields = ("portId",)


class PortNode(SQLAlchemyObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """🔌 A port is a conceptual connection point (with a direction) of a type."""

    class Meta:
        model = Port
        name = "Port"
        exclude_fields = (
            "id",
            "localId",
            "pointX",
            "pointY",
            "pointZ",
            "directionX",
            "directionY",
            "directionZ",
            "typeId",
        )

    id = graphene.Field(NonNull(graphene.String))
    point = graphene.Field(NonNull(PointNode))
    direction = graphene.Field(NonNull(VectorNode))
    plane = graphene.Field(NonNull(PlaneNode))

    def resolve_id(port: Port, info):
        return port.localId

    def resolve_point(port: Port, info):
        return port.point

    def resolve_direction(port: Port, info):
        return port.direction

    def resolve_plane(port: Port, info):
        return port.plane


class QualityNode(SQLAlchemyObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """📏 A quality is meta-data for decision making."""

    class Meta:
        model = Quality
        name = "Quality"
        exclude_fields = ("id", "typeId", "formationId")


class TypeNode(SQLAlchemyObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    class Meta:
        model = Type
        name = "Type"
        # interfaces = (ArtifactNode,)
        exclude_fields = (
            "id",
            "kitId",
        )


class SidePieceTypeNode(PydanticObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """🧩 The type information of a piece of a side."""

    class Meta:
        model = SidePieceType
        name = "SidePieceType"
        # port is none Pydanctic model and needs to be resolved manually
        exclude_fields = ("port",)

    port = graphene.Field(PortNode)

    def resolve_port(type: SidePieceType, info):
        return type.port


class PieceRootNode(PydanticObjectType):

    class Meta:
        model = PieceRoot
        name = "PieceRoot"


class PieceDiagramNode(PydanticObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """✏️ The diagram information of a piece."""

    class Meta:
        model = PieceDiagram
        name = "PieceDiagram"


class PieceNode(SQLAlchemyObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """⭕ A piece is a 3d-instance of a type in a formation."""

    class Meta:
        model = Piece
        name = "Piece"
        exclude_fields = (
            "id",
            "localId",
            "typeId",
            "rootPlaneOriginX",
            "rootPlaneOriginY",
            "rootPlaneOriginZ",
            "rootPlaneXAxisX",
            "rootPlaneXAxisY",
            "rootPlaneXAxisZ",
            "rootPlaneYAxisX",
            "rootPlaneYAxisY",
            "rootPlaneYAxisZ",
            "diagramPointX",
            "diagramPointY",
            "formationId",
        )

    id = graphene.Field(NonNull(graphene.String))
    root = graphene.Field(PieceRootNode)
    diagram = graphene.Field(NonNull(PieceDiagramNode))

    def resolve_id(piece: Piece, info):
        return piece.localId

    def resolve_root(piece: Piece, info):
        if piece.root is not None:
            return piece.root

    def resolve_diagram(piece: Piece, info):
        return piece.diagram


class SidePieceNode(PydanticObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """⭕ The piece information of a side. A piece is identified by an id (emtpy=default))."""

    class Meta:
        name = "SidePiece"
        model = SidePiece


class SideNode(PydanticObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """🧱 A side of a piece in a connection."""

    class Meta:
        name = "Side"
        model = Side


class ConnectionNode(SQLAlchemyObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """🖇️ A connection between two pieces of a formation."""

    class Meta:
        model = Connection
        name = "Connection"
        exclude_fields = (
            "connectedPieceId",
            "connectedPiece",
            "connectedPieceTypePortId",
            "connectedPieceTypePort",
            "connectingPieceId",
            "connectingPiece",
            "connectingPieceTypePortId",
            "connectingPieceTypePort",
            "formationId",
        )

    connected = NonNull(SideNode)
    connecting = NonNull(SideNode)

    def resolve_connected(connection: Connection, info):
        return connection.connected

    def resolve_connecting(connection: Connection, info):
        return connection.connecting


class FormationNode(SQLAlchemyObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """🏙️ A formation is a collection of pieces that are connected."""

    class Meta:
        model = Formation
        name = "Formation"
        # interfaces = (ArtifactNode,)
        exclude_fields = (
            "id",
            "kitId",
        )


class ObjectNode(PydanticObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """🗿 An object is a piece with a plane and a parent object (unless the piece is a root)."""

    class Meta:
        model = Object
        name = "Object"
        exclude_fields = ("piece", "parent")

    piece = graphene.Field(PieceNode)
    parent = graphene.Field(lambda: ObjectNode)

    def resolve_piece(object: Object, info):
        return object.piece

    def resolve_plane(object, info):
        return object.plane

    def resolve_parent(object, info):
        return object.parent


class SceneNode(PydanticObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """🌆 A scene is a collection of objects."""

    class Meta:
        model = Scene
        name = "Scene"
        # formation is not a Pydanctic model and needs to be resolved manually
        exclude_fields = ("formation",)

    formation = graphene.Field(FormationNode)

    def resolve_formation(scene: Scene, info):
        return scene.formation


class KitNode(SQLAlchemyObjectType):
    # Duplicate docstring because not automatically generated by SQLAlchemyObjectType
    """🗃️ A kit is a collection of types and formations."""

    class Meta:
        model = Kit
        name = "Kit"
        # interfaces = (ArtifactNode,)
        exclude_fields = ("id",)


class RepresentationInput(InputObjectType):
    # Duplicate docstring because not automatically generated by InputObjectType
    """💾 A representation is a link to a file that describes a type for a certain level of detail and tags."""

    url = NonNull(graphene.String)
    mime = graphene.String()
    lod = graphene.String()
    tags = graphene.List(NonNull(graphene.String))


class LocatorInput(InputObjectType):
    # Duplicate docstring because not automatically generated by InputObjectType
    """🗺️ A locator is meta-data for grouping ports."""

    group = NonNull(graphene.String)
    subgroup = graphene.String()


class ScreenPointInput(PydanticInputObjectType):
    # Duplicate docstring because not automatically generated by PydanticInputObjectType
    """📺 A 2d-point (xy) of integers in screen coordinate system."""

    class Meta:
        model = ScreenPoint


class PointInput(PydanticInputObjectType):
    # Duplicate docstring because not automatically generated by PydanticInputObjectType
    """✖️ A 3d-point (xyz) of floating point numbers."""

    class Meta:
        model = Point


class VectorInput(PydanticInputObjectType):
    # Duplicate docstring because not automatically generated by PydanticInputObjectType
    """➡️ A 3d-vector (xyz) of floating point numbers."""

    class Meta:
        model = Vector


class PlaneInput(PydanticInputObjectType):
    # Duplicate docstring because not automatically generated by PydanticInputObjectType
    """◳ A plane is an origin (point) and an orientation (x-axis and y-axis)."""

    class Meta:
        model = Plane


class PortInput(InputObjectType):
    # Duplicate docstring because not automatically generated by InputObjectType
    """🔌 A port is a conceptual connection point (with a direction) of a type."""

    id = graphene.String(default_value="")
    point = NonNull(PointInput)
    direction = NonNull(VectorInput)
    locators = graphene.List(NonNull(LocatorInput))


class PortIdInput(InputObjectType):
    """🔌 A port is identified by an id (emtpy=default))."""

    id = graphene.String(default_value="")


class QualityInput(InputObjectType):
    # Duplicate docstring because not automatically generated by InputObjectType
    """📏 A quality is meta-data for decision making."""

    name = NonNull(graphene.String)
    value = graphene.String()
    unit = graphene.String()
    definition = graphene.String()


class TypeInput(InputObjectType):
    # Duplicate docstring because not automatically generated by InputObjectType
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    name = NonNull(graphene.String)
    description = graphene.String()
    icon = graphene.String()
    variant = graphene.String(default_value="")
    unit = NonNull(graphene.String)
    representations = NonNull(graphene.List(NonNull(RepresentationInput)))
    ports = NonNull(graphene.List(NonNull(PortInput)))
    qualities = graphene.List(NonNull(QualityInput))


class TypeIdInput(PydanticInputObjectType):
    """🧩 A type is identified by a name and variant (empty=default)."""

    class Meta:
        model = TypeId


class PieceRootInput(PydanticInputObjectType):
    """🌱 The root information of a piece."""

    # Duplicate docstring because not automatically generated by PydanticInputObjectType

    class Meta:
        model = PieceRoot


class PieceDiagramInput(PydanticInputObjectType):
    # Duplicate docstring because not automatically generated by PydanticInputObjectType
    """✏️ The diagram information of a piece."""

    class Meta:
        model = PieceDiagram


class PieceInput(InputObjectType):
    # Duplicate docstring because not automatically generated by nputObjectType
    """⭕ A piece is a 3d-instance of a type in a formation."""

    id = NonNull(graphene.String)
    type = NonNull(TypeIdInput)
    root = graphene.Field(PieceRootInput)
    diagram = NonNull(PieceDiagramInput)


class SidePieceTypeInput(InputObjectType):
    # Duplicate docstring because not automatically generated by InputObjectType
    """🧩 The type information of a piece of a side."""
    port = graphene.Field(PortIdInput)


class SidePieceInput(InputObjectType):
    # Duplicate docstring because not automatically generated by InputObjectType
    """⭕ The piece information of a side. A piece is identified by an id (emtpy=default))."""

    id = NonNull(graphene.String)
    type = graphene.Field(SidePieceTypeInput)


class SideInput(InputObjectType):
    # Duplicate docstring because not automatically generated by InputObjectType
    """🧱 A side of a piece in a connection."""

    piece = NonNull(SidePieceInput)


class ConnectionInput(InputObjectType):
    # Duplicate docstring because not automatically generated by PydanticInputObjectType
    """🖇️ A connection between two pieces of a formation."""

    connecting = NonNull(SideInput)
    connected = NonNull(SideInput)
    offset = graphene.Float(default_value=0.0)
    rotation = graphene.Float(default_value=0.0)


class FormationInput(InputObjectType):
    # Duplicate docstring because not automatically generated by PydanticInputObjectType
    """🏙️ A formation is a collection of pieces that are connected."""

    name = NonNull(graphene.String)
    description = graphene.String()
    icon = graphene.String()
    variant = graphene.String(default_value="")
    unit = NonNull(graphene.String)
    pieces = NonNull(graphene.List(NonNull(PieceInput)))
    connections = NonNull(graphene.List(NonNull(ConnectionInput)))
    qualities = graphene.List(NonNull(QualityInput))


class FormationIdInput(InputObjectType):
    # Duplicate docstring because not automatically generated by InputObjectType
    """🏙️ A formation is identified by a name and optional variant."""

    name = NonNull(graphene.String)
    variant = graphene.String(default_value="")


class KitInput(InputObjectType):
    # Duplicate docstring because not automatically generated by InputObjectType
    """🗃️ A kit is a collection of types and formations."""

    name = NonNull(graphene.String)
    description = graphene.String()
    icon = graphene.String()
    url = graphene.String()
    homepage = graphene.String()
    types = graphene.List(NonNull(TypeInput))
    formations = graphene.List(NonNull(FormationInput))


class KitMetadataInput(InputObjectType):
    """🗃️ Meta-data of a kit."""

    name = graphene.String()
    description = graphene.String()
    icon = graphene.String()
    url = graphene.String()
    homepage = graphene.String()


RepresentationLike = Representation | RepresentationNode | RepresentationInput
ScreenPointLike = ScreenPoint | ScreenPointNode | ScreenPointInput
PointLike = Point | PointNode | PointInput
VectorLike = Vector | VectorNode | VectorInput
PlaneLike = Plane | PlaneNode | PlaneInput
LocatorLike = Locator | LocatorNode | LocatorInput
PortLike = Port | PortNode | PortInput
QualityLike = Quality | QualityNode | QualityInput
TypeLike = Type | TypeNode | TypeInput
PieceRootLike = PieceRoot | PieceRootNode | PieceRootInput
PieceDiagramLike = PieceDiagram | PieceDiagramNode | PieceDiagramInput
PieceLike = Piece | PieceNode | PieceInput
SidePieceTypeLike = SidePieceType | SidePieceTypeNode | SidePieceTypeInput
SidePieceLike = SidePiece | SidePieceNode | SidePieceInput
SideLike = Side | SideNode | SideInput
ConnectionLike = Connection | ConnectionNode | ConnectionInput
FormationLike = Formation | FormationNode | FormationInput
KitLike = Kit | KitNode | KitInput

class NotFound(SpecificationError):
    def __init__(self, id, pythonType) -> None:
        self.id = id
        self.pythonType = pythonType

    def __str__(self):
        return f"{self.id} ({self.pythonType}) not found."


class RepresentationNotFound(NotFound):
    def __init__(self, type, url) -> None:
        super().__init__(url, Representation)
        self.type = type
        self.url = url

    def __str__(self):
        return f"Representation({self.url}) not found for type: {self.type.client__str__()}"


class PortNotFound(NotFound):
    def __init__(self, type, id) -> None:
        super().__init__(id, Port)
        self.type = type
        self.id = id

    def __str__(self):
        return f"Port({self.id}) not found for type: {self.type.client__str__()}."


class TypeNotFound(NotFound):
    def __init__(self, name) -> None:
        super().__init__(name, Type)
        self.name = name

    def __str__(self):
        return f"Type({self.name}) not found."


class QualitiesDontMatchType(TypeNotFound):
    def __init__(
        self, name, qualityInputs: List[QualityInput], types: List[Type]
    ) -> None:
        super().__init__(name, Quality)
        self.qualityInputs = qualityInputs
        self.types = types

    def __str__(self):
        return f"Qualities({self.qualityInputs}) don't match any type with name {self.name}: {client__str__List(self.types)}"


class TooLittleQualitiesToMatchExcactlyType(QualitiesDontMatchType):
    def __str__(self):
        return f"Too little qualities ({self.qualityInputs}) to match exactly one type name {self.name}: {client__str__List(self.types)}"


class PieceNotFound(NotFound):
    def __init__(self, formation, localId) -> None:
        super().__init__(localId, Piece)
        self.formation = formation
        self.localId = localId

    def __str__(self):
        return f"Piece({self.localId}) not found. Please check that the local id is correct and that the piece is part of the formation {self.formation.client__str__()}."


class ConnectionNotFound(NotFound):
    def __init__(self, formation, connected, connecting) -> None:
        super().__init__((connected,connecting), Connection)
        self.formation = formation
        self.connected = connected
        self.connecting = connecting

    def __str__(self):
        return f"Connection with connected piece id ({self.connected}) and connecting piece id ({self.connecting}) not found in formation {self.formation.client__str__()}"


class FormationNotFound(NotFound):
    def __init__(self, name, variant = "") -> None:
        super().__init__(name, Formation)
        self.name = name
        self.variant = variant

    def __str__(self):
        return f"Formation({(self.name + ":" + self.variant) if self.variant!="" else self.name}) not found."

class KitNotFound(NotFound):
    def __init__(self, name) -> None:
        super().__init__(name, Kit)
        self.name = name

    def __str__(self):
        return f"Kit({self.name}) not found."


class NoMainKit(KitNotFound):
    def __init__(self) -> None:
        super().__init__("main")

    def __str__(self):
        return f"Main kit not found."


class AlreadyExists(SpecificationError):
    def __init__(self, new, existing) -> None:
        self.new = new
        self.existing = existing

    def __str__(self):
        return f"{str(self.new)} already exists: {str(self.existing)}"


class RepresentationAlreadyExists(AlreadyExists):
    def __init__(self, newRepresentation : RepresentationLike, oldRepresentation) -> None:
        super().__init__(newRepresentation, oldRepresentation)

    def __str__(self):
        return f"Representation with url: {self.new.url!r} already exists: {str(self.existing)}"


class PortAlreadyExists(AlreadyExists):
    def __init__(self, id) -> None:
        super().__init__(id, id)

    def __str__(self):
        return f"Port with id: {self.id!r} already exists."


class ConnectionAlreadyExists(AlreadyExists):
    def __init__(self, connection: Connection, existingConnection: Connection) -> None:
        super().__init__(
            (connection.connecting.piece.id, connection.connected.piece.id),
            existingConnection,
        )
        self.connection = connection

    def __str__(self):
        return f"Connection with connecting piece id ({self.connection.connecting.piece.id}) and connected piece id ({self.connection.connected.piece.id}) already exists: {self.existing.client__str__()}"


class DocumentAlreadyExists(AlreadyExists):
    def __init__(self, document) -> None:
        super().__init__(document.name, document)
        self.document = document

    def __str__(self):
        return f"Artifact ({self.document.name}) already exists: {str(self.document)}"


class TypeAlreadyExists(DocumentAlreadyExists):
    def __init__(self, type) -> None:
        super().__init__(type)
        self.type = type

    def __str__(self):
        return f"Type ({self.type.name}) already exists: {str(self.type)}"


class FormationAlreadyExists(DocumentAlreadyExists):
    def __init__(self, formation) -> None:
        super().__init__(formation)
        self.formation = formation

    def __str__(self):
        return (
            f"Formation ({self.formation.name}) already exists: {str(self.formation)}"
        )


def getMainKit(session: Session) -> Kit:
    kit = session.query(Kit).first()
    if not kit:
        raise NoMainKit()
    return kit


def getRepresentationByUrl(session: Session, type: Type, url: str) -> Representation:
    representationsWithSameUrl = session.query(Representation).filter_by(url=url)
    match representationsWithSameUrl.count():
        case 0:
            raise RepresentationNotFound(type, url)
        case 1:
            return representationsWithSameUrl.first()
        case _:
            raise InvalidDatabase(
                f"Found multiple representations {representationsWithSameUrl.all()!r} for {str(type)} and url: {url}"
            )


def getTypeByNameAndVariant(session: Session, name: str, variant: str) -> Type:
    try:
        type = session.query(Type).filter_by(name=name, variant=variant).one_or_none()
    except MultipleResultsFound as e:
        raise InvalidDatabase(
            f"Found multiple types with name {name} and variant {variant}"
        ) from e
    if not type:
        raise TypeNotFound(name)
    return type


def getFormationByNameAndVariant(
    session: Session, name: str, variant: str = ""
) -> Formation:
    try:
        formation = (
            session.query(Formation).filter_by(name=name, variant=variant).one_or_none()
        )
    except MultipleResultsFound as e:
        raise InvalidDatabase(
            f"Found multiple formations with name {name} and variant {variant}"
        ) from e
    if not formation:
        raise FormationNotFound(name, variant)
    return formation


def getPortById(session: Session, type: Type, portId: str) -> Port:
    port = session.query(Port).filter_by(typeId=type.id, localId=portId).first()
    if not port:
        raise PortNotFound(type, portId)
    return port


def getConnectionByPieceIds(
    session: Session,
    formation: Formation,
    connectedPieceId: str,
    connectingPieceId: str,
) -> Connection:
    connection = (
        session.query(Connection)
        .filter_by(
            connectedPieceId=connectedPieceId,
            connectingPieceId=connectingPieceId,
            formationId=formation.id,
        )
        .first()
    )
    if not connection:
        raise ConnectionNotFound(connectedPieceId, connectingPieceId, formation)
    return connection


def addRepresentationInputToSession(
    session: Session,
    type: Type,
    representationInput: RepresentationInput,
) -> Representation:
    try:
        lod = representationInput.lod if representationInput.lod is not None else ""
    except AttributeError:
        lod = ""
    try:
        mime = representationInput.mime if representationInput.mime is not None else ""
    except AttributeError:
        mime = ""
    if mime == "":
        mime = parseMimeFromUrl(representationInput.url)
    try:
        representation = getRepresentationByUrl(session, type, representationInput.url)
        raise RepresentationAlreadyExists(representationInput,representation)
    except RepresentationNotFound:
        pass
    representation = Representation(
        url=representationInput.url,
        mime=mime,
        lod=lod,
        typeId=type.id,
    )
    session.add(representation)
    session.flush()
    for tagInput in representationInput.tags or []:
        tag = Tag(
            value=tagInput,
            representationId=representation.id,
        )
        session.add(tag)
        session.flush()
    return representation


def addLocatorInputToSession(
    session: Session, port: Port, locatorInput: LocatorInput
) -> Locator:
    try:
        subgroup = locatorInput.subgroup if locatorInput.subgroup is not None else ""
    except AttributeError:
        subgroup = ""
    locator = Locator(group=locatorInput.group, subgroup=subgroup, portId=port.id)
    session.add(locator)
    session.flush()
    return locator


def addPortInputToSession(session: Session, type: Type, portInput: PortInput) -> Port:
    try:
        id = portInput.id if portInput.id is not None else ""
    except AttributeError:
        id = ""
    try:
        existingPort = getPortById(session, type, id)
        raise PortAlreadyExists(existingPort)
    except PortNotFound:
        pass
    port = Port(
        localId=id,
        pointX=portInput.point.x,
        pointY=portInput.point.y,
        pointZ=portInput.point.z,
        directionX=portInput.direction.x,
        directionY=portInput.direction.y,
        directionZ=portInput.direction.z,
        typeId=type.id,
    )
    session.add(port)
    session.flush()
    for locatorInput in portInput.locators or []:
        locator = addLocatorInputToSession(session, port, locatorInput)
    return port


def addQualityInputToSession(
    session: Session,
    owner: Type | Formation,
    qualityInput: QualityInput,
) -> Quality:
    try:
        unit = qualityInput.unit if qualityInput.unit is not None else ""
    except AttributeError:
        unit = ""
    try:
        value = qualityInput.value if qualityInput.value is not None else ""
    except AttributeError:
        value = ""
    try:
        definition = (
            qualityInput.definition if qualityInput.definition is not None else ""
        )
    except AttributeError:
        definition = ""
    typeId = owner.id if isinstance(owner, Type) else None
    formationId = owner.id if isinstance(owner, Formation) else None
    quality = Quality(
        name=qualityInput.name,
        value=value,
        unit=unit,
        definition=definition,
        typeId=typeId,
        formationId=formationId,
    )
    session.add(quality)
    session.flush()
    return quality


def addTypeInputToSession(session: Session, kit: Kit, typeInput: TypeInput) -> Type:
    try:
        description = typeInput.description if typeInput.description is not None else ""
    except AttributeError:
        description = ""
    try:
        icon = typeInput.icon if typeInput.icon is not None else ""
    except AttributeError:
        icon = ""
    try:
        variant = typeInput.variant if typeInput.variant is not None else ""
    except AttributeError:
        variant = ""
    type = Type(
        name=typeInput.name,
        description=description,
        icon=icon,
        variant=variant,
        unit=typeInput.unit,
        kitId=kit.id,
    )
    try:
        existingType = getTypeByNameAndVariant(session, typeInput.name, variant)
        raise TypeAlreadyExists(type, existingType)
    except TypeNotFound:
        pass
    session.add(type)
    session.flush()
    for representationInput in typeInput.representations or []:
        representation = addRepresentationInputToSession(
            session, type, representationInput
        )
    for portInput in typeInput.ports or []:
        port = addPortInputToSession(session, type, portInput)
    for qualityInput in typeInput.qualities or []:
        quality = addQualityInputToSession(session, type, qualityInput)
    return type


def addPieceInputToSession(
    session: Session, formation: Formation, pieceInput: PieceInput
) -> Piece:
    try:
        variant = pieceInput.type.variant if pieceInput.type.variant is not None else ""
    except AttributeError:
        variant = ""
    type = getTypeByNameAndVariant(session, pieceInput.type.name, variant)
    piece = Piece(
        localId=pieceInput.id,
        typeId=type.id,
        diagramPointX=pieceInput.diagram.point.x,
        diagramPointY=pieceInput.diagram.point.y,
        formationId=formation.id,
    )
    try:
        piece.rootPlaneOriginX = pieceInput.root.plane.origin.x
        piece.rootPlaneOriginY = pieceInput.root.plane.origin.y
        piece.rootPlaneOriginZ = pieceInput.root.plane.origin.z
        piece.rootPlaneXAxisX = pieceInput.root.plane.xAxis.x
        piece.rootPlaneXAxisY = pieceInput.root.plane.xAxis.y
        piece.rootPlaneXAxisZ = pieceInput.root.plane.xAxis.z
        piece.rootPlaneYAxisX = pieceInput.root.plane.yAxis.x
        piece.rootPlaneYAxisY = pieceInput.root.plane.yAxis.y
        piece.rootPlaneYAxisZ = pieceInput.root.plane.yAxis.z
    except AttributeError:
        pass
    session.add(piece)
    session.flush()
    return piece


def addConnectionInputToSession(
    session: Session,
    formation: Formation,
    connectionInput: ConnectionInput,
    localIdToPiece: dict,
) -> Connection:
    try:
        connectedPieceTypePortId = (
            connectionInput.connected.piece.type.port.id
            if connectionInput.connected.piece.type.port.id is not None
            else ""
        )
    except AttributeError:
        connectedPieceTypePortId = ""
    try:
        connectingPieceTypePortId = (
            connectionInput.connecting.piece.type.port.id
            if connectionInput.connecting.piece.type.port.id is not None
            else ""
        )
    except AttributeError:
        connectingPieceTypePortId = ""
    try:
        # TODO: Somehow the flushing of the other connections works but you can't query for them.
        # When I try to look for an existing connection it finds none but when adding it,
        # it raises a proper IntegrityError
        existingConnection = getConnectionByPieceIds(
            session,
            formation,
            connectionInput.connected.piece.id,
            connectionInput.connecting.piece.id,
        )
        raise ConnectionAlreadyExists(connectionInput, existingConnection)
    except ConnectionNotFound:
        pass
    try:
        connectingPiece = localIdToPiece[connectionInput.connecting.piece.id]
    except KeyError:
        raise PieceNotFound(formation, connectionInput.connecting.piece.id)
    try:
        connectedPiece = localIdToPiece[connectionInput.connected.piece.id]
    except KeyError:
        raise PieceNotFound(formation, connectionInput.connected.piece.id)
    connectingPieceTypePort = getPortById(
        session,
        connectingPiece.type,
        connectingPieceTypePortId,
    )
    connectedPieceTypePort = getPortById(
        session,
        connectedPiece.type,
        connectedPieceTypePortId,
    )
    try:
        offset = connectionInput.offset if connectionInput.offset is not None else 0.0
    except AttributeError:
        offset = 0.0
    try:
        rotation = (
            connectionInput.rotation % 360 if connectionInput.rotation is not None else 0.0
        )
        if rotation < 0.0:
            rotation = rotation + 360.0
    except AttributeError:
        rotation = 0.0
    connection = Connection(
        connectedPieceId=connectedPiece.id,
        connectedPieceTypePortId=connectedPieceTypePort.id,
        connectingPieceId=connectingPiece.id,
        connectingPieceTypePortId=connectingPieceTypePort.id,
        offset=offset,
        rotation=rotation,
        formationId=formation.id,
    )
    session.add(connection)
    session.flush()
    return connection


def addFormationInputToSession(
    session: Session, kit: Kit, formationInput: FormationInput
):
    try:
        description = (
            formationInput.description if formationInput.description is not None else ""
        )
    except AttributeError:
        description = ""
    try:
        icon = formationInput.icon if formationInput.icon is not None else ""
    except AttributeError:
        icon = ""
    try:
        variant = formationInput.variant if formationInput.variant is not None else ""
    except AttributeError:
        variant = ""
    try:
        existingFormation = getFormationByNameAndVariant(
            session, formationInput.name, variant
        )
        raise FormationAlreadyExists(existingFormation)
    except FormationNotFound:
        pass
    formation = Formation(
        name=formationInput.name,
        description=description,
        icon=icon,
        variant=variant,
        unit=formationInput.unit,
        kitId=kit.id,
    )
    session.add(formation)
    session.flush()
    localIdToPiece: Dict[str, Piece] = {}
    for pieceInput in formationInput.pieces or []:
        piece = addPieceInputToSession(session, formation, pieceInput)
        localIdToPiece[pieceInput.id] = piece
    for connectionInput in formationInput.connections or []:
        connection = addConnectionInputToSession(
            session, formation, connectionInput, localIdToPiece
        )
    for qualityInput in formationInput.qualities or []:
        quality = addQualityInputToSession(session, formation, qualityInput)
    return formation


def addKitInputToSession(session: Session, kitInput: KitInput):
    try:
        kit = getMainKit(session)
    except NoMainKit:
        kit = Kit(
            name=kitInput.name,
        )
    try:
        kit.description = kitInput.description or ""
    except AttributeError:
        pass
    try:
        kit.icon = kitInput.icon or ""
    except AttributeError:
        pass
    try:
        kit.url = kitInput.url or ""
    except AttributeError:
        pass
    try:
        kit.homepage = kitInput.homepage or ""
    except AttributeError:
        pass
    session.add(kit)
    session.flush()
    for typeInput in kitInput.types or []:
        type = addTypeInputToSession(session, kit, typeInput)
    for formationInput in kitInput.formations or []:
        formation = addFormationInputToSession(session, kit, formationInput)
    return kit


def updateKitMetadataInSession(session: Session, kitMetadata: KitMetadataInput):
    kit = getMainKit(session)
    try:
        kit.name = kitMetadata.name or ""
    except AttributeError:
        pass
    try:
        kit.description = kitMetadata.description or ""
    except AttributeError:
        pass
    try:
        kit.icon = kitMetadata.icon or ""
    except AttributeError:
        pass
    try:
        kit.url = kitMetadata.url or ""
    except AttributeError:
        pass
    try:
        kit.homepage = kitMetadata.homepage or ""
    except AttributeError:
        pass
    return kit


def formationToHierarchies(formation: Formation) -> List[Hierarchy]:
    nodes = list((piece.localId, {"piece": piece}) for piece in formation.pieces)
    edges = (
        (
            connection.connecting.piece.id,
            connection.connected.piece.id,
            {"connection": connection},
        )
        for connection in formation.connections
    )
    graph = Graph()
    graph.add_nodes_from(nodes)
    graph.add_edges_from(edges)
    hierarchies = []
    for componentGenerator in connected_components(graph):
        component = graph.subgraph(componentGenerator)
        try:
            root = [
                node for node in component.nodes() if graph.nodes[node]["piece"].root
            ][0]
        except IndexError:
            root = next(iter(component.nodes))
        rootHierarchy = Hierarchy(
            piece=graph.nodes[root]["piece"],
            transform=Transform(),
            children=[],
        )
        component.nodes[root]["hierarchy"] = rootHierarchy
        for parent, child in bfs_tree(component, source=root).edges():
            connection = component[parent][child]["connection"]
            connectedIsParent = connection.connected.piece.id == parent
            parentPort = connection.connected.piece.type.port if connectedIsParent else connection.connecting.piece.type.port
            childPort = connection.connecting.piece.type.port if connectedIsParent else connection.connected.piece.type.port
            orient = Transform.fromDirections(childPort.direction.revert(), parentPort.direction)
            rotation = orient
            if connection.rotation != 0.0:
                rotate = Transform.fromAngle(parentPort.direction, connection.rotation)
                rotation = rotate.after(orient)
            centerChild = childPort.point.toVector().revert().toTransform()
            moveToParent = parentPort.point.toVector().toTransform()
            transform = rotation.after(centerChild)
            if connection.offset != 0.0:
                offset = parentPort.direction.amplify(connection.offset).toTransform()
                transform = offset.after(transform)
            transform = moveToParent.after(transform)
            hierarchy = Hierarchy(
                piece=component.nodes[child]["piece"],
                transform=transform,
                children=[],
            )
            component.nodes[child]["hierarchy"] = hierarchy
            component.nodes[parent]["hierarchy"].children.append(hierarchy)
        hierarchies.append(rootHierarchy)
        with open("../../local/engine_hierarchy.json", "w") as file:
            file.write(rootHierarchy.model_dump_json())

    return hierarchies


def addObjectsToScene(
    scene: "Scene",
    parent: Object,
    hierarchy: Hierarchy,
    plane: Plane,
) -> None:
    transformedPlane = plane.transform(hierarchy.transform)
    object = Object(
        piece=hierarchy.piece,
        plane=transformedPlane,
        parent=parent,
    )
    scene.objects.append(object)
    for child in hierarchy.children:
        addObjectsToScene(scene, object, child, transformedPlane)


def sceneFromFormationInSession(
    session: Session, formationIdInput: FormationIdInput
) -> "Scene":
    try:
        variant = (
            formationIdInput.variant if formationIdInput.variant is not None else ""
        )
    except AttributeError:
        variant = ""
    formation = getFormationByNameAndVariant(session, formationIdInput.name, variant)
    hierarchies = formationToHierarchies(formation)
    scene = Scene(formation=formation, objects=[])
    for hierarchy in hierarchies:
        addObjectsToScene(
            scene,
            None,
            hierarchy,
            hierarchy.piece.root.plane if hierarchy.piece.root else Plane.XY(),
        )
    return scene


class CreateLocalKitErrorCode(graphene.Enum):
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_ALREADY_CONTAINS_A_KIT = "directory_already_contains_a_kit"
    NO_PERMISSION_TO_CREATE_DIRECTORY = "no_permission_to_create_directory"
    NO_PERMISSION_TO_CREATE_KIT = "no_permission_to_create_kit"
    KIT_INPUT_IS_INVALID = "kit_input_is_invalid"


class CreateLocalKitErrorNode(ObjectType):
    class Meta:
        name = "CreateLocalKitError"

    code = NonNull(CreateLocalKitErrorCode)
    message = graphene.String()


disposed_engines = {}


class CreateLocalKitMutation(graphene.Mutation):
    class Arguments:
        directory = NonNull(graphene.String)
        kitInput = NonNull(KitInput)

    kit = Field(KitNode)
    error = Field(CreateLocalKitErrorNode)

    def mutate(self, info, directory, kitInput: KitInput):
        directory = Path(directory)
        if not directory.exists():
            try:
                directory.mkdir(parents=True)
            except PermissionError:
                return CreateLocalKitMutation(
                    error=CreateLocalKitErrorNode(
                        code=CreateLocalKitErrorCode.NO_PERMISSION_TO_CREATE_DIRECTORY
                    )
                )
            except OSError:
                return CreateLocalKitMutation(
                    error=CreateLocalKitErrorNode(
                        code=CreateLocalKitErrorCode.DIRECTORY_IS_NOT_A_DIRECTORY
                    )
                )
        kitFile = directory.joinpath(KIT_FOLDERNAME).joinpath(KIT_FILENAME)
        if kitFile.exists():
            return CreateLocalKitMutation(
                error=CreateLocalKitErrorNode(
                    code=CreateLocalKitErrorCode.DIRECTORY_ALREADY_CONTAINS_A_KIT
                )
            )
        else:
            kitFile.parent.mkdir(parents=True, exist_ok=True)

        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in disposed_engines:
            # Can't update a kit in a directory where this process already deleted an engine.
            # Ending the process and let the watcher restart it is the only way to handle this.
            logging.debug("Engine already disposed. Exiting.")
            os._exit(1)

        session = getLocalSession(directory)
        try:
            kit = addKitInputToSession(session, kitInput)
        except SpecificationError as e:
            session.rollback()
            return CreateLocalKitMutation(
                error=CreateLocalKitErrorNode(
                    code=CreateLocalKitErrorCode.KIT_INPUT_IS_INVALID, message=str(e)
                )
            )
        session.commit()
        return CreateLocalKitMutation(kit=kit)


class UpdateLocalKitMetadataErrorCode(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_UPDATE_KIT = "no_permission_to_update_kit"
    KIT_METADATA_IS_INVALID = "kit_metadata_is_invalid"


class UpdateLocalKitMetadataErrorNode(ObjectType):
    class Meta:
        name = "UpdateLocalKitMetadataError"

    code = NonNull(UpdateLocalKitMetadataErrorCode)
    message = graphene.String()


class UpdateLocalKitMetadataMutation(graphene.Mutation):
    class Arguments:
        directory = NonNull(graphene.String)
        kitMetadataInput = NonNull(KitMetadataInput)

    kit = Field(KitNode)
    error = Field(UpdateLocalKitMetadataErrorNode)

    def mutate(self, info, directory, kitMetadataInput, mode):
        directory = Path(directory)
        if not directory.exists():
            return UpdateLocalKitMetadataMutation(
                error=UpdateLocalKitMetadataErrorNode(
                    code=UpdateLocalKitMetadataErrorCode.DIRECTORY_DOES_NOT_EXIST
                )
            )
        if not directory.is_dir():
            return UpdateLocalKitMetadataMutation(
                error=UpdateLocalKitMetadataErrorNode(
                    code=UpdateLocalKitMetadataErrorCode.DIRECTORY_IS_NOT_A_DIRECTORY
                )
            )
        kitFile = directory.joinpath(KIT_FOLDERNAME).joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return UpdateLocalKitMetadataMutation(
                error=UpdateLocalKitMetadataErrorNode(
                    code=UpdateLocalKitMetadataErrorCode.DIRECTORY_HAS_NO_KIT
                )
            )
        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in disposed_engines:
            # Can't update a kit in a directory where this process already deleted an engine.
            # Ending the process and let the watcher restart it is the only way to handle this.
            logging.debug("Engine already disposed. Exiting.")
            os._exit(1)
        session = getLocalSession(directory)
        try:
            kit = updateKitMetadataInSession(session, kitMetadataInput)
        except SpecificationError as e:
            session.rollback()
            return UpdateLocalKitMetadataMutation(
                error=UpdateLocalKitMetadataErrorNode(
                    code=UpdateLocalKitMetadataErrorCode.KIT_METADATA_IS_INVALID,
                    message=str(e),
                )
            )
        session.commit()
        return UpdateLocalKitMetadataMutation(kit=kit)


class DeleteLocalKitError(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_DELETE_KIT = "no_permission_to_delete_kit"


class DeleteLocalKitMutation(graphene.Mutation):
    class Arguments:
        directory = NonNull(graphene.String)

    error = Field(DeleteLocalKitError)

    def mutate(self, info, directory):
        directory = Path(directory)
        if not directory.exists():
            return DeleteLocalKitMutation(
                error=DeleteLocalKitError.DIRECTORY_DOES_NOT_EXIST
            )
        kitFile = directory.joinpath(KIT_FOLDERNAME).joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return DeleteLocalKitMutation(
                error=DeleteLocalKitError.DIRECTORY_HAS_NO_KIT
            )
        kitFileFullPath = kitFile.resolve()
        disposed_engines[kitFileFullPath] = True
        try:
            remove(kitFileFullPath)
        except PermissionError:
            return DeleteLocalKitMutation(
                error=DeleteLocalKitError.NO_PERMISSION_TO_DELETE_KIT
            )
        return DeleteLocalKitMutation()


class AddTypeToLocalKitErrorCode(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_MODIFY_KIT = "no_permission_to_modify_kit"
    TYPE_INPUT_IS_INVALID = "type_input_is_invalid"


class AddTypeToLocalKitErrorNode(ObjectType):
    class Meta:
        name = "AddTypeToLocalKitError"

    code = NonNull(AddTypeToLocalKitErrorCode)
    message = graphene.String()


class AddTypeToLocalKitMutation(graphene.Mutation):
    class Arguments:
        directory = NonNull(graphene.String)
        typeInput = NonNull(TypeInput)

    type = Field(TypeNode)
    error = Field(AddTypeToLocalKitErrorNode)

    def mutate(self, info, directory, typeInput):
        directory = Path(directory)
        if not directory.exists():
            return AddTypeToLocalKitMutation(
                error=AddTypeToLocalKitErrorNode(
                    code=AddTypeToLocalKitErrorCode.DIRECTORY_DOES_NOT_EXIST
                )
            )
        if not directory.is_dir():
            return AddTypeToLocalKitMutation(
                error=AddTypeToLocalKitErrorNode(
                    code=AddTypeToLocalKitErrorCode.DIRECTORY_IS_NOT_A_DIRECTORY
                )
            )
        kitFile = directory.joinpath(KIT_FOLDERNAME).joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return AddTypeToLocalKitMutation(
                error=AddTypeToLocalKitErrorNode(
                    code=AddTypeToLocalKitErrorCode.DIRECTORY_HAS_NO_KIT
                )
            )
        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in disposed_engines:
            # Can't update a kit in a directory where this process already deleted an engine.
            # Ending the process and let the watcher restart it is the only way to handle this.
            logging.debug("Engine already disposed. Exiting.")
            os._exit(1)
        session = getLocalSession(directory)
        try:
            kit = getMainKit(session)
        except NoMainKit:
            raise Exception("Main kit not found.")
        try:
            type = addTypeInputToSession(session, kit, typeInput)
        except SpecificationError as e:
            session.rollback()
            return AddTypeToLocalKitMutation(
                error=AddTypeToLocalKitErrorNode(
                    code=AddTypeToLocalKitErrorCode.TYPE_INPUT_IS_INVALID,
                    message=str(e),
                )
            )
        except IntegrityError as e:
            session.rollback()
            return AddTypeToLocalKitMutation(
                error=AddTypeToLocalKitErrorNode(
                    code=AddTypeToLocalKitErrorCode.TYPE_INPUT_IS_INVALID,
                    message=str(
                        "Sorry, I didn't have time to write you a nice error message. For now I can only give you the technical description of what is wrong: "
                        + str(e)
                    ),
                )
            )
        session.commit()
        return AddTypeToLocalKitMutation(type=type)


class RemoveTypeFromLocalKitErrorCode(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_MODIFY_KIT = "no_permission_to_modify_kit"
    TYPE_DOES_NOT_EXIST = "type_does_not_exist"
    FORMATION_DEPENDS_ON_TYPE = "formation_depends_on_type"


class RemoveTypeFromLocalKitErrorNode(ObjectType):
    class Meta:
        name = "RemoveTypeFromLocalKitError"

    code = NonNull(RemoveTypeFromLocalKitErrorCode)
    message = graphene.String()


class RemoveTypeFromLocalKitMutation(graphene.Mutation):
    class Arguments:
        directory = NonNull(graphene.String)
        typeId = NonNull(TypeIdInput)

    error = Field(RemoveTypeFromLocalKitErrorNode)

    def mutate(self, info, directory, typeId):
        directory = Path(directory)
        if not directory.exists():
            return RemoveTypeFromLocalKitMutation(
                error=RemoveTypeFromLocalKitErrorNode(
                    code=RemoveTypeFromLocalKitErrorCode.DIRECTORY_DOES_NOT_EXIST
                ),
            )
        if not directory.is_dir():
            return RemoveTypeFromLocalKitMutation(
                error=RemoveTypeFromLocalKitErrorNode(
                    code=RemoveTypeFromLocalKitErrorCode.DIRECTORY_IS_NOT_A_DIRECTORY,
                )
            )
        kitFile = directory.joinpath(KIT_FOLDERNAME).joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return RemoveTypeFromLocalKitMutation(
                error=RemoveTypeFromLocalKitErrorNode(
                    code=RemoveTypeFromLocalKitErrorCode.DIRECTORY_HAS_NO_KIT,
                )
            )
        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in disposed_engines:
            # Can't update a kit in a directory where this process already deleted an engine.
            # Ending the process and let the watcher restart it is the only way to handle this.
            logging.debug("Engine already disposed. Exiting.")
            os._exit(1)
        session = getLocalSession(directory)
        try:
            kit = getMainKit(session)
        except NoMainKit:
            raise Exception("Main kit not found.")
        try:
            try:
                variant = typeId.variant if typeId.variant is not None else ""
            except AttributeError:
                variant = ""
            type = getTypeByNameAndVariant(session, typeId.name, variant)
        except TypeNotFound:
            return RemoveTypeFromLocalKitMutation(
                error=RemoveTypeFromLocalKitErrorNode(
                    code=RemoveTypeFromLocalKitErrorCode.TYPE_DOES_NOT_EXIST
                ),
            )
        if type.pieces:
            return RemoveTypeFromLocalKitMutation(
                error=RemoveTypeFromLocalKitErrorNode(
                    code=RemoveTypeFromLocalKitErrorCode.FORMATION_DEPENDS_ON_TYPE
                ),
            )
        session.delete(type)
        session.commit()
        return RemoveTypeFromLocalKitMutation()


class AddFormationToLocalKitErrorCode(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_MODIFY_KIT = "no_permission_to_modify_kit"
    FORMATION_INPUT_IS_INVALID = "formation_input_is_invalid"


class AddFormationToLocalKitErrorNode(ObjectType):
    class Meta:
        name = "AddFormationToLocalKitError"

    code = NonNull(AddFormationToLocalKitErrorCode)
    message = graphene.String()


class AddFormationToLocalKitMutation(graphene.Mutation):
    class Arguments:
        directory = NonNull(graphene.String)
        formationInput = NonNull(FormationInput)

    formation = Field(FormationNode)
    error = Field(AddFormationToLocalKitErrorNode)

    def mutate(self, info, directory, formationInput):
        directory = Path(directory)
        if not directory.exists():
            return AddFormationToLocalKitMutation(
                error=AddFormationToLocalKitErrorNode(
                    code=AddFormationToLocalKitErrorCode.DIRECTORY_DOES_NOT_EXIST
                )
            )
        if not directory.is_dir():
            return AddFormationToLocalKitMutation(
                error=AddFormationToLocalKitErrorNode(
                    code=AddFormationToLocalKitErrorCode.DIRECTORY_IS_NOT_A_DIRECTORY
                )
            )
        kitFile = directory.joinpath(KIT_FOLDERNAME).joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return AddFormationToLocalKitMutation(
                error=AddFormationToLocalKitErrorNode(
                    code=AddFormationToLocalKitErrorCode.DIRECTORY_HAS_NO_KIT
                )
            )
        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in disposed_engines:
            # Can't update a kit in a directory where this process already deleted an engine.
            # Ending the process and let the watcher restart it is the only way to handle this.
            logging.debug("Engine already disposed. Exiting.")
            os._exit(1)
        session = getLocalSession(directory)
        try:
            kit = getMainKit(session)
        except NoMainKit:
            raise Exception("Main kit not found.")
        try:
            formation = addFormationInputToSession(session, kit, formationInput)
        except SpecificationError as e:
            session.rollback()
            return AddFormationToLocalKitMutation(
                error=AddFormationToLocalKitErrorNode(
                    code=AddFormationToLocalKitErrorCode.FORMATION_INPUT_IS_INVALID,
                    message=str(e),
                )
            )
        except IntegrityError as e:
            session.rollback()
            return AddFormationToLocalKitMutation(
                error=AddFormationToLocalKitErrorNode(
                    code=AddFormationToLocalKitErrorCode.FORMATION_INPUT_IS_INVALID,
                    message=str(
                        "Sorry, I didn't have time to write you a nice error message. For now I can only give you the technical description of what is wrong: "
                        + str(e)
                    ),
                )
            )
        session.commit()
        return AddFormationToLocalKitMutation(formation=formation)


class RemoveFormationFromLocalKitErrorCode(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_MODIFY_KIT = "no_permission_to_modify_kit"
    FORMATION_DOES_NOT_EXIST = "formation_does_not_exist"


class RemoveFormationFromLocalKitErrorNode(ObjectType):
    class Meta:
        name = "RemoveFormationFromLocalKitError"

    code = NonNull(RemoveFormationFromLocalKitErrorCode)
    message = graphene.String()


class RemoveFormationFromLocalKitMutation(graphene.Mutation):
    class Arguments:
        directory = NonNull(graphene.String)
        formationId = NonNull(FormationIdInput)

    error = Field(RemoveFormationFromLocalKitErrorNode)

    def mutate(self, info, directory, formationId):
        directory = Path(directory)
        if not directory.exists():
            return RemoveFormationFromLocalKitMutation(
                error=RemoveFormationFromLocalKitErrorNode(
                    code=RemoveFormationFromLocalKitErrorCode.DIRECTORY_DOES_NOT_EXIST,
                )
            )
        if not directory.is_dir():
            return RemoveFormationFromLocalKitMutation(
                error=RemoveFormationFromLocalKitErrorNode(
                    code=RemoveFormationFromLocalKitErrorCode.DIRECTORY_IS_NOT_A_DIRECTORY,
                )
            )
        kitFile = directory.joinpath(KIT_FOLDERNAME).joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return RemoveFormationFromLocalKitMutation(
                error=RemoveFormationFromLocalKitErrorNode(
                    code=RemoveFormationFromLocalKitErrorCode.DIRECTORY_HAS_NO_KIT
                ),
            )
        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in disposed_engines:
            # Can't update a kit in a directory where this process already deleted an engine.
            # Ending the process and let the watcher restart it is the only way to handle this.
            logging.debug("Engine already disposed. Exiting.")
            os._exit(1)
        session = getLocalSession(directory)
        try:
            kit = getMainKit(session)
        except NoMainKit:
            raise Exception("Main kit not found.")
        try:
            try:
                variant = formationId.variant if formationId.variant is not None else ""
            except AttributeError:
                variant = ""
            formation = getFormationByNameAndVariant(session, formationId.name, variant)
        except FormationNotFound:
            return RemoveFormationFromLocalKitMutation(
                error=RemoveFormationFromLocalKitErrorNode(
                    code=RemoveFormationFromLocalKitErrorCode.FORMATION_DOES_NOT_EXIST
                ),
            )
        session.delete(formation)
        session.commit()
        return RemoveFormationFromLocalKitMutation()


class LoadLocalKitError(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_READ_KIT = "no_permission_to_read_kit"


class LoadLocalKitResponse(ObjectType):
    kit = Field(KitNode)
    error = Field(LoadLocalKitError)


class FormationToSceneFromLocalKitResponseErrorCode(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_READ_KIT = "no_permission_to_read_kit"
    FORMATION_DOES_NOT_EXIST = "formation_does_not_exist"


class FormationToSceneFromLocalKitResponseErrorNode(ObjectType):
    class Meta:
        name = "FormationToSceneFromLocalKitResponseError"

    code = NonNull(FormationToSceneFromLocalKitResponseErrorCode)
    message = graphene.String()


class FormationToSceneFromLocalKitResponse(ObjectType):
    scene = Field(SceneNode)
    error = Field(FormationToSceneFromLocalKitResponseErrorNode)


class Query(ObjectType):
    loadLocalKit = Field(LoadLocalKitResponse, directory=NonNull(graphene.String))
    formationToSceneFromLocalKit = Field(
        FormationToSceneFromLocalKitResponse,
        directory=NonNull(graphene.String),
        formationIdInput=NonNull(FormationIdInput),
    )

    def resolve_loadLocalKit(self, info, directory: graphene.String):
        directory = Path(directory)
        if not directory.exists():
            return LoadLocalKitResponse(
                error=LoadLocalKitError.DIRECTORY_DOES_NOT_EXIST
            )
        if not directory.is_dir():
            return LoadLocalKitResponse(
                error=LoadLocalKitError.DIRECTORY_IS_NOT_A_DIRECTORY
            )
        try:
            session = getLocalSession(directory)
        except PermissionError:
            return LoadLocalKitResponse(
                error=LoadLocalKitError.NO_PERMISSION_TO_READ_KIT
            )
        try:
            kit = getMainKit(session)
        except NoMainKit:
            return LoadLocalKitResponse(error=LoadLocalKitError.DIRECTORY_HAS_NO_KIT)
        return LoadLocalKitResponse(kit=kit)

    def resolve_formationToSceneFromLocalKit(
        self, info, directory, formationIdInput: FormationIdInput
    ):
        directory = Path(directory)
        if not directory.exists():
            return FormationToSceneFromLocalKitResponse(
                error=FormationToSceneFromLocalKitResponseErrorNode(
                    code=FormationToSceneFromLocalKitResponseErrorCode.DIRECTORY_DOES_NOT_EXIST
                )
            )
        if not directory.is_dir():
            return FormationToSceneFromLocalKitResponse(
                error=FormationToSceneFromLocalKitResponseErrorNode(
                    code=FormationToSceneFromLocalKitResponseErrorCode.DIRECTORY_IS_NOT_A_DIRECTORY
                )
            )
        kitFile = directory.joinpath(KIT_FOLDERNAME).joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return FormationToSceneFromLocalKitResponse(
                error=FormationToSceneFromLocalKitResponseErrorNode(
                    code=FormationToSceneFromLocalKitResponseErrorCode.DIRECTORY_HAS_NO_KIT
                )
            )
        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in disposed_engines:
            # Can't update a kit in a directory where this process already deleted an engine.
            # Ending the process and let the watcher restart it is the only way to handle this.
            logging.debug("Engine already disposed. Exiting.")
            os._exit(1)
        session = getLocalSession(directory)
        try:
            scene = sceneFromFormationInSession(session, formationIdInput)
        except FormationNotFound:
            return FormationToSceneFromLocalKitResponse(
                error=FormationToSceneFromLocalKitResponseErrorNode(
                    code=FormationToSceneFromLocalKitResponseErrorCode.FORMATION_DOES_NOT_EXIST
                )
            )
        return FormationToSceneFromLocalKitResponse(scene=scene)


class Mutation(ObjectType):
    createLocalKit = CreateLocalKitMutation.Field()
    updateLocalKitMetadata = UpdateLocalKitMetadataMutation.Field()
    deleteLocalKit = DeleteLocalKitMutation.Field()
    addTypeToLocalKit = AddTypeToLocalKitMutation.Field()
    removeTypeFromLocalKit = RemoveTypeFromLocalKitMutation.Field()
    addFormationToLocalKit = AddFormationToLocalKitMutation.Field()
    removeFormationFromLocalKit = RemoveFormationFromLocalKitMutation.Field()


schema = Schema(
    query=Query,
    mutation=Mutation,
)


def start_engine():
    engine = Starlette()
    engine.mount("/graphql", GraphQLApp(schema, on_get=make_graphiql_handler()))

    run(
        engine,
        host=HOST,
        port=PORT,
        log_level="info",
        access_log=False,
        log_config=None,
    )


# def restart_engine():
#     ui_instance = QApplication.instance()
#     engine_process = ui_instance.engine_process
#     if engine_process.is_alive():
#         engine_process.terminate()
#     ui_instance.engine_process = Process(target=start_engine)
#     ui_instance.engine_process.start()


def main():
    parser = ArgumentParser()
    parser.add_argument("--debug", action="store_true", help="Enable debug mode")
    args = parser.parse_args()
    if args.debug:
        with open("../../graphql/schema.graphql", "w", encoding="utf-8") as f:
            f.write(str(schema))

        metadata_engine = create_engine("sqlite:///debug/semio.db")
        Base.metadata.create_all(metadata_engine)
    start_engine()


if __name__ == "__main__":
    freeze_support()  # needed for pyinstaller on Windows
    main()

    # ui = QApplication(sys.argv)
    # ui.setQuitOnLastWindowClosed(False)

    # # Final location of assets when bundled with PyInstaller
    # if getattr(sys, "frozen", False):
    #     basedir = sys._MEIPASS
    # else:
    #     basedir = "../.."

    # icon = QIcon()
    # icon.addFile(os.path.join(basedir, "icons/semio_16x16.png"), QSize(16, 16))
    # icon.addFile(os.path.join(basedir, "icons/semio_32x32.png"), QSize(32, 32))
    # icon.addFile(os.path.join(basedir, "icons/semio_48x48.png"), QSize(48, 48))
    # icon.addFile(os.path.join(basedir, "icons/semio_128x128.png"), QSize(128, 128))
    # icon.addFile(os.path.join(basedir, "icons/semio_256x256.png"), QSize(256, 256))

    # tray = QSystemTrayIcon()
    # tray.setIcon(icon)
    # tray.setVisible(True)

    # menu = QMenu()
    # restart = QAction("Restart")
    # restart.triggered.connect(restart_engine)
    # menu.addAction(restart)

    # quit = QAction("Quit")
    # quit.triggered.connect(lambda: ui.engine_process.terminate() or ui.quit())
    # menu.addAction(quit)

    # tray.setContextMenu(menu)

    # ui.engine_process = Process(target=start_engine)
    # ui.engine_process.start()

    # sys.exit(ui.exec_())
