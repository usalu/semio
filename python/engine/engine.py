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
# TODO: Add constraint to formations that at least 2 pieces and 1 attraction are required.

from argparse import ArgumentParser
import os
import sys
import logging  # for uvicorn in pyinstaller
from os import remove
from pathlib import Path
from multiprocessing import Process, freeze_support
from functools import lru_cache
from time import sleep
from typing import Optional, Dict, Protocol, List, Union
from datetime import datetime
from urllib.parse import urlparse
from numpy import ndarray
from pytransform3d.transformations import (
    concat,
    invert_transform,
    transform_from,
)
from networkx import (
    DiGraph,
    bfs_tree,
    connected_components,
    weakly_connected_components,
)
from pint import UnitRegistry
from pydantic import BaseModel
from sqlalchemy import (
    Boolean,
    String,
    Text,
    Float,
    DateTime,
    ForeignKey,
    create_engine,
    CheckConstraint,
    UniqueConstraint,
    event,
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
from PySide6.QtWidgets import (
    QApplication,
    QSystemTrayIcon,
    QMenu,
)
from PySide6.QtCore import QSize
from PySide6.QtGui import (
    QIcon,
    QAction,
)

logging.basicConfig(level=logging.INFO)  # for uvicorn in pyinstaller

NAME_LENGTH_MAX = 100
URL_LENGTH_MAX = 1000
KIT_FOLDERNAME = ".semio"
KIT_FILENAME = "kit.sqlite3"
HOST = "127.0.0.1"
PORT = 5052

ureg = UnitRegistry()


class SemioException(Exception):
    """The base class for all exceptions in semio."""

    pass


class SpecificationError(SemioException):
    """The base class for all specification errors.
    A specification error is when the user input does not respect the specification."""

    pass


class InvalidURL(ValueError, SpecificationError):
    """The URL is not valid. An url must have the form:
    scheme://netloc/path;parameters?query#fragment."""

    def __init__(self, url: str) -> None:
        self.url = url

    def __str__(self) -> str:
        return f"{self.url} is not a valid URL."


class InvalidDatabase(SemioException):
    """The state of the database is somehow invalid.
    Check the constraints and the insert validators.
    """

    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + "\n The database is invalid. Please report this bug."


class InvalidBackend(SemioException):
    """The backend processed something wrong. Check the order of operations."""

    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + "\n The backend is invalid. Please report this bug."


class Entity(Protocol):
    """An entity is anything that is captured in the persistance layer."""

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
    def referenced_by(self) -> List["Entity"]:
        return []

    @property
    def related_to(self) -> List["Entity"]:
        return (
            ([self.parent] if self.parent else [])
            + self.children
            + self.references
            + self.referenced_by
        )

    def client__str__(self) -> str:
        """A string representation of the entitity for the client."""
        pass


def list_client__str__(entities: Entity) -> str:
    """Get a string representation of a list of entitities for the client.

    Args:
        entities (Entity): The list of entitities.

    Returns:
        str: String representation.
    """
    return f"[{', '.join([e.client__str__() for e in entities])}]"


# TODO: Refactor Protocol to ABC and make it work with SQLAlchemy
class Artifact(Entity):
    """An artifact is anything that is worth to be reused."""

    name: str
    description: str
    icon: str


class Base(DeclarativeBase):
    pass


class Tag(Base):
    """A tag helps to categorize a representation.
    It enables to select subgroups of representations that belong together."""

    __tablename__ = "tag"

    value: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(value) > 0", name="value_not_empty_constraint"),
        primary_key=True,
    )
    representation_id: Mapped[int] = mapped_column(
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
        return (
            f"Tag(value={self.value!r}, representation_id={self.representation_id!r})"
        )

    def __str__(self) -> str:
        return (
            f"Tag(value={self.value}, representation_id={str(self.representation_id)})"
        )

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
    # def referenced_by(self) -> List[Entity]:
    #     return []

    # @property
    # def related_to(self) -> List[Entity]:
    #     return [self.parent]


class Representation(Base):
    """A representation is a link to a file that describes a type from a certain point of view."""

    __tablename__ = "representation"
    id: Mapped[int] = mapped_column(primary_key=True)
    url: Mapped[str] = mapped_column(
        String(URL_LENGTH_MAX),
        CheckConstraint("length(url) > 0", name="url_not_empty_constraint"),
    )
    # level of detail/development/design/...
    # "" means the defaut lod.
    lod: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    type_id: Mapped[int] = mapped_column(ForeignKey("type.id"), nullable=False)
    type: Mapped["Type"] = relationship("Type", back_populates="representations")
    _tags: Mapped[List[Tag]] = relationship(
        Tag, back_populates="representation", cascade="all, delete-orphan"
    )

    __table_args__ = (UniqueConstraint("type_id", "url"),)

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Representation):
    #         raise NotImplementedError()
    #     return self.url == other.url

    # def __hash__(self) -> int:
    #     return hash(self.url)

    def __repr__(self) -> str:
        return f"Representation(id={self.id!r}, url={self.url!r}, lod={self.lod!r}, type_id={self.type_id!r}, tags={self.tags!r})"

    def __str__(self) -> str:
        return f"Representation(id={str(self.id)}, type_id={str(self.type_id)})"

    def client__str__(self) -> str:
        return f"Representation(url={self.url}, tags={list_client__str__(self.tags)})"

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
    # def referenced_by(self) -> List[Entity]:
    #     return []

    # @property
    # def related_to(self) -> List[Entity]:
    #     return [self.parent] + self.children if self.children else []


class Locator(Base):
    """A locator helps to categorize a port from a type.
    It enables to select a single port from all ports based on groups and subgroups."""

    __tablename__ = "locator"

    group: Mapped[str] = mapped_column(
        "group_name",  # group is a reserved keyword in SQL
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(group_name) > 0", name="group_not_empty_constraint"),
        primary_key=True,
        key="group",
    )
    # Optional. "" means true.
    subgroup: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    port_id: Mapped[int] = mapped_column(ForeignKey("port.id"), primary_key=True)
    port: Mapped["Port"] = relationship("Port", back_populates="locators")

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Locator):
            raise NotImplementedError()
        return self.group == other.group and self.subgroup == other.subgroup

    def __hash__(self) -> int:
        return hash((self.group, self.subgroup))

    def __repr__(self) -> str:
        return f"Locator(group={self.group!r}, subgroup={self.subgroup!r}, port_id={self.port_id!r})"

    def __str__(self) -> str:
        return f"Locator(group={self.group}, port_id={str(self.port_id)})"

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
    # def referenced_by(self) -> List[Entity]:
    #     return []

    # @property
    # def related_to(self) -> List[Entity]:
    #     return [self.parent]


class ScreenPoint(BaseModel):
    x: int
    y: int


class Point(BaseModel):
    x: float
    y: float
    z: float

    # def transform(self, transform: "Transform") -> "Point":
    #     return Transform.transformPoint(transform, self)


class Vector(BaseModel):
    x: float
    y: float
    z: float

    # def transform(self, transform: "Transform") -> "Vector":
    #     return Transform.transformVector(transform, self)


class Plane(Base):
    """A plane with an origin and two axis. Synonyms are reference frame or coordinate system."""

    __tablename__ = "plane"

    id: Mapped[int] = mapped_column(primary_key=True)
    origin_x: Mapped[float] = mapped_column(Float())
    origin_y: Mapped[float] = mapped_column(Float())
    origin_z: Mapped[float] = mapped_column(Float())
    x_axis_x: Mapped[float] = mapped_column(Float())
    x_axis_y: Mapped[float] = mapped_column(Float())
    x_axis_z: Mapped[float] = mapped_column(Float())
    y_axis_x: Mapped[float] = mapped_column(Float())
    y_axis_y: Mapped[float] = mapped_column(Float())
    y_axis_z: Mapped[float] = mapped_column(Float())

    port: Mapped["Port"] = relationship("Port", back_populates="plane", uselist=False)
    root_piece: Mapped["Piece"] = relationship(
        "Piece", back_populates="root_plane", uselist=False
    )

    @property
    def origin(self) -> Point:
        return Point(x=self.origin_x, y=self.origin_y, z=self.origin_z)

    @property
    def x_axis(self) -> Vector:
        return Vector(x=self.x_axis_x, y=self.x_axis_y, z=self.x_axis_z)

    @property
    def y_axis(self) -> Vector:
        return Vector(x=self.y_axis_x, y=self.y_axis_y, z=self.y_axis_z)

    @property
    def normal(self) -> Vector:
        return Vector(
            x=self.x_axis_y * self.y_axis_z - self.x_axis_z * self.y_axis_y,
            y=self.x_axis_z * self.y_axis_x - self.x_axis_x * self.y_axis_z,
            z=self.x_axis_x * self.y_axis_y - self.x_axis_y * self.y_axis_x,
        )

    def transform(self, transform: "Transform") -> "Plane":
        return Transform.transformPlane(transform, self)

    def toTransform(self) -> "Transform":
        return Transform.fromPlane(self)

    @staticmethod
    def XY() -> "Plane":
        return Plane(
            origin_x=0,
            origin_y=0,
            origin_z=0,
            x_axis_x=1,
            x_axis_y=0,
            x_axis_z=0,
            y_axis_x=0,
            y_axis_y=1,
            y_axis_z=0,
        )


class Transform(BaseModel):
    """A 4x4 transformation matrix. It is only used for translation and rotation.
    It is not used for scaling or shearing."""

    class Config:
        arbitrary_types_allowed = True

    # An indirect import of numpy, just for type hinting. Really?
    matrix: ndarray

    def __str__(self) -> str:
        return f"Transform(Rotation={self.rotation}, Translation={self.translation})"

    @property
    def rotation(self) -> ndarray:
        """The 3x3 rotation part of the transformation matrix."""
        return self.matrix[:3, :3]

    @property
    def translation(self) -> ndarray:
        """The 3x1 translation part of the transformation matrix."""
        return self.matrix[:3, 3]

    def after(self, before: "Transform") -> "Transform":
        """Apply this transform after another transform.

        Args:
            before (Transform): Transform to apply before this transform.

        Returns:
            Transform: New transform.
        """
        return Transform(matrix=concat(before.matrix, self.matrix))

    def invert(self) -> "Transform":
        return Transform(matrix=invert_transform(self.matrix))

    # def transformPoint(self, point: Point) -> Point:
    #     transformedPoint = transform(
    #         self.matrix, vector_to_point([point.x, point.y, point.z])
    #     )
    #     return Point(
    #         x=transformedPoint[0], y=transformedPoint[1], z=transformedPoint[2]
    #     )

    # def transformVector(self, vector: Vector) -> Vector:
    #     transformedVector = transform(
    #         self.matrix, vector_to_point([vector.x, vector.y, vector.z])
    #     )
    #     return Vector(
    #         x=transformedVector[0], y=transformedVector[1], z=transformedVector[2]
    #     )

    def transformPlane(self, plane: Plane) -> Plane:
        planeTransform = Transform.fromPlane(plane)
        planeTransformed = planeTransform.after(self)
        return Transform.toPlane(planeTransformed.round())

    def round(self, decimals: int = 6) -> "Transform":
        return Transform(matrix=self.matrix.round(decimals=decimals))

    @staticmethod
    def identity() -> "Transform":
        return Transform(
            matrix=transform_from(
                [
                    [1, 0, 0],
                    [0, 1, 0],
                    [0, 0, 1],
                ],
                [0, 0, 0],
            )
        )

    @staticmethod
    def fromPlane(plane: Plane) -> "Transform":
        return Transform(
            matrix=transform_from(
                [
                    [plane.x_axis.x, plane.y_axis.x, plane.normal.x],
                    [plane.x_axis.y, plane.y_axis.y, plane.normal.y],
                    [plane.x_axis.z, plane.y_axis.z, plane.normal.z],
                ],
                [plane.origin.x, plane.origin.y, plane.origin.z],
            )
        )

    @staticmethod
    def toPlane(transform: "Transform") -> Plane:
        return Plane(
            origin_x=transform.translation[0],
            origin_y=transform.translation[1],
            origin_z=transform.translation[2],
            x_axis_x=transform.rotation[0, 0],
            x_axis_y=transform.rotation[1, 0],
            x_axis_z=transform.rotation[2, 0],
            y_axis_x=transform.rotation[0, 1],
            y_axis_y=transform.rotation[1, 1],
            y_axis_z=transform.rotation[2, 1],
        )


class Port(Base):
    """A port is specified plane in a type. It is used to connect types together."""

    __tablename__ = "port"

    id: Mapped[int] = mapped_column(primary_key=True)
    # "" means the default port.
    local_id: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    plane_id: Mapped[int] = mapped_column(ForeignKey("plane.id"))
    plane: Mapped["Plane"] = relationship(
        "Plane", back_populates="port", uselist=False, cascade="all, delete"
    )
    type_id: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped["Type"] = relationship("Type", back_populates="ports")
    locators: Mapped[List[Locator]] = relationship(
        Locator, back_populates="port", cascade="all, delete-orphan"
    )
    attractings: Mapped[List["Attraction"]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracting_piece_type_port_id]",
        back_populates="attracting_piece_type_port",
    )
    attracteds: Mapped[List["Attraction"]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracted_piece_type_port_id]",
        back_populates="attracted_piece_type_port",
    )

    __table_args__ = (UniqueConstraint("local_id", "type_id"),)

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Port):
    #         raise NotImplementedError()
    #     return set(self.qualities) == set(other.qualities)

    # def __hash__(self) -> int:
    #     return hash(set(self.qualities))

    def __repr__(self) -> str:
        return f"Port(id={self.id!r}, local_id={self.local_id!r}, plane_id={self.plane_id!r}, type_id={self.type_id!r}, locators={self.locators!r})"

    def __str__(self) -> str:
        return f"Port(id={str(self.id)}, type_id={str(self.type_id)})"

    def client__str__(self) -> str:
        return f"Port(id={self.local_id})"

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
    # def referenced_by(self) -> List[Entity]:
    #     return self.attractings + self.attracteds  # type: ignore

    # @property
    # def related_to(self) -> List[Entity]:
    #     return [self.parent] + self.children + self.referenced_by


class Quality(Base):
    """A quality helps to categorize a type or a formation.
    Qualities should be high-level information that can be important for decision making.
    """

    __tablename__ = "quality"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(name) > 0", name="name_not_empty_constraint"),
    )
    # Optional. "" means true.
    value: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    # Optional. Set to "" for None.
    unit: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    type_id: Mapped[Optional[int]] = mapped_column(ForeignKey("type.id"), nullable=True)
    type: Mapped["Type"] = relationship("Type", back_populates="qualities")
    formation_id: Mapped[Optional[int]] = mapped_column(
        ForeignKey("formation.id"), nullable=True
    )
    formation: Mapped["Formation"] = relationship(
        "Formation", back_populates="qualities"
    )

    __table_args__ = (
        CheckConstraint(
            "type_id IS NOT NULL AND formation_id IS NULL OR type_id IS NULL AND formation_id IS NOT NULL",
            name="type_or_formation_owner_constraint",
        ),
        UniqueConstraint("name", "type_id", "formation_id"),
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
        return f"Quality(id={self.id}, name={self.name!r}, value={self.value!r}, unit={self.unit!r}, type_id={self.type_id!r}, formation_id={self.formation_id!r})"

    def __str__(self) -> str:
        return f"Quality(id={self.id}, type_id={str(self.type_id)}, formation_id={str(self.formation_id)})"

    def client__str__(self) -> str:
        return f"Quality(name={self.name}, value={self.value}, unit={self.unit})"

    # @property
    # def parent(self) -> Entity:
    #     return self.type if self.type_id else self.formation

    # @property
    # def children(self) -> List[Entity]:
    #     return []

    # @property
    # def references(self) -> List[Entity]:
    #     return []

    # @property
    # def referenced_by(self) -> List[Entity]:
    #     return []

    # @property
    # def related_to(self) -> List[Entity]:
    #     return [self.parent]


class Type(Base):
    """A type is a reusable building block that can be freely combined with other types over ports.
    It is uniquely identified by its name and its variant.
    Representations can be used to visualize or simulate the type.
    Qualities are meta-data that help to categorize the type.
    """

    __tablename__ = "type"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(name) > 0", name="name_not_empty_constraint"),
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
        CheckConstraint("length(unit) > 0", name="unit_not_empty_constraint"),
    )
    created_at: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.utcnow, nullable=False
    )
    last_update_at: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.utcnow, nullable=False, onupdate=datetime.utcnow
    )
    kit_id: Mapped[int] = mapped_column(ForeignKey("kit.id"))
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
        return f"Type(id={self.id!r}, name={self.name!r}, description={self.description!r}, icon={self.icon!r}, variant={self.variant!r} unit={self.unit!r}, kit_id={self.kit_id!r}, representations={self.representations!r}, ports={self.ports!r}, qualities={self.qualities!r}, pieces={self.pieces!r})"

    def __str__(self) -> str:
        return f"Type(id={str(self.id)}, kit_id={str(self.kit_id)})"

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
    # def referenced_by(self) -> List[Entity]:
    #     return [self.pieces]  # type: ignore

    # @property
    # def related_to(self) -> List[Entity]:
    #     return [self.parent] + self.children + self.referenced_by


@event.listens_for(Representation, "after_update")
def receive_after_update(mapper, connection, target):
    target.type.last_update_at = datetime.utcnow()


@event.listens_for(Port, "after_update")
def receive_after_update(mapper, connection, target):
    target.type.last_update_at = datetime.utcnow()


class RootPiece(BaseModel):
    """The plane of the root piece of a formation."""

    class Config:
        arbitrary_types_allowed = True

    plane: Plane


class DiagramPiece(BaseModel):
    """The point of a diagram of a piece."""

    point: ScreenPoint


class Piece(Base):
    """A piece is an instance of a type in a formation."""

    __tablename__ = "piece"

    id: Mapped[int] = mapped_column(primary_key=True)
    local_id: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(local_id) > 0", name="local_id_not_empty_constraint"),
    )
    type_id: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped["Type"] = relationship("Type", back_populates="pieces")
    root_plane_id: Mapped[int] = mapped_column(ForeignKey("plane.id"), nullable=True)
    root_plane: Mapped["Plane"] = relationship(
        "Plane", back_populates="root_piece", uselist=False, cascade="all, delete"
    )
    diagram_point_x: Mapped[int] = mapped_column()
    diagram_point_y: Mapped[int] = mapped_column(Float())
    formation_id: Mapped[int] = mapped_column(ForeignKey("formation.id"))
    formation: Mapped["Formation"] = relationship("Formation", back_populates="pieces")
    attractings: Mapped[List["Attraction"]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracting_piece_id]",
        back_populates="attracting_piece",
    )
    attracteds: Mapped[List["Attraction"]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracted_piece_id]",
        back_populates="attracted_piece",
    )

    __table_args__ = (UniqueConstraint("local_id", "formation_id"),)

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Piece):
    #         raise NotImplementedError()
    #     return self.local_id == other.local_id

    # def __hash__(self) -> int:
    #     return hash(self.local_id)

    def __repr__(self) -> str:
        return f"Piece(id={self.id!r}, local_id={self.local_id!r}, type_id={self.type_id!r}, root_plane_id={self.root_plane_id!r}, diagram_point_x={self.diagram_point_x!r}, diagram_point_y={self.diagram_point_y!r}, formation_id={self.formation_id!r})"

    def __str__(self) -> str:
        return f"Piece(id={str(self.id)}, formation_id={str(self.formation_id)})"

    def client__str__(self) -> str:
        return f"Piece(id={self.local_id})"

    @property
    def root(self) -> RootPiece | None:
        if self.root_plane:
            return RootPiece(plane=self.root_plane)
        return None

    @property
    def diagram(self) -> DiagramPiece:
        return DiagramPiece(
            point=ScreenPoint(x=self.diagram_point_x, y=self.diagram_point_y)
        )

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
    # def referenced_by(self) -> List[Entity]:
    #     return self.attractings + self.attracteds  # type: ignore

    # @property
    # def related_to(self) -> List[Entity]:
    #     return [self.parent] + self.references + self.referenced_by


class TypePieceSide(BaseModel):
    """The port of a type of a piece of a side of an attraction."""

    class Config:
        arbitrary_types_allowed = True

    port: Port


class PieceSide(BaseModel):
    """The piece of a side of an attraction."""

    class Config:
        arbitrary_types_allowed = True

    id: str
    type: TypePieceSide


class Side(BaseModel):
    """A side of an attraction."""

    class Config:
        arbitrary_types_allowed = True

    piece: PieceSide


class Attraction(Base):
    """An attraction is a connection between two pieces of a formation."""

    __tablename__ = "attraction"

    attracting_piece_id: Mapped[int] = mapped_column(
        ForeignKey("piece.id"), primary_key=True
    )
    attracting_piece: Mapped[Piece] = relationship(
        Piece, foreign_keys=[attracting_piece_id], back_populates="attractings"
    )
    attracting_piece_type_port_id = mapped_column(ForeignKey("port.id"))
    attracting_piece_type_port: Mapped[Port] = relationship(
        Port,
        foreign_keys=[attracting_piece_type_port_id],
        back_populates="attractings",
    )
    attracted_piece_id: Mapped[int] = mapped_column(
        ForeignKey("piece.id"), primary_key=True
    )
    attracted_piece: Mapped[Piece] = relationship(
        Piece, foreign_keys=[attracted_piece_id], back_populates="attracteds"
    )
    attracted_piece_type_port_id = mapped_column(ForeignKey("port.id"))
    attracted_piece_type_port: Mapped[Port] = relationship(
        Port,
        foreign_keys=[attracted_piece_type_port_id],
        back_populates="attracteds",
    )
    formation_id: Mapped[int] = mapped_column(ForeignKey("formation.id"))
    formation: Mapped["Formation"] = relationship(
        "Formation", back_populates="attractions"
    )

    __table_args__ = (
        CheckConstraint(
            "attracting_piece_id != attracted_piece_id",
            name="attracting_and_attracted_piece_not_equal_constraint",
        ),
    )

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Attraction):
    #         raise NotImplementedError()
    #     return (
    #         self.attracting_piece == other.attracting_piece
    #         and self.attracted_piece == other.attracted_piece
    #     )

    # def __hash__(self) -> int:
    #     return hash((self.attracting_piece, self.attracted_piece))

    def __repr__(self) -> str:
        return f"Attraction(attracting_piece_id={self.attracting_piece_id!r}, attracting_piece_type_port_id={self.attracting_piece_type_port_id!r}, attracted_piece_id={self.attracted_piece_id!r}, attracted_piece_type_port_id={self.attracted_piece_type_port_id!r}, formation_id={self.formation_id!r})"

    def __str__(self) -> str:
        return f"Attraction(attracting_piece_id={str(self.attracting_piece_id)}, attracted_piece_id={str(self.attracted_piece_id)}, formation_id={str(self.formation_id)})"

    def client__str__(self) -> str:
        return f"Attraction(attracting_piece_id={self.attracting.piece.id}, attracted_piece_id={self.attracted.piece.id})"

    @property
    def attracting(self) -> Side:
        return Side(
            piece=PieceSide(
                id=self.attracting_piece.local_id,
                type=TypePieceSide(
                    port=self.attracting_piece_type_port,
                ),
            )
        )

    @property
    def attracted(self) -> Side:
        return Side(
            piece=PieceSide(
                id=self.attracted_piece.local_id,
                type=TypePieceSide(
                    port=self.attracted_piece_type_port,
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
    #         self.attracting_piece,
    #         self.attracted_piece,
    #         self.attracting_piece_type_port,
    #         self.attracted_piece_type_port,
    #     ]

    # @property
    # def referenced_by(self) -> List[Entity]:
    #     return []

    # @property
    # def related_to(self) -> List[Entity]:
    #     return [self.parent] + self.references


# TODO: Add complex validation before insert with networkx such as:
#       - only root pieces can have a plane.
class Formation(Base):
    """A formation is a collection of pieces that are connected by attractions.
    It is uniquely identified by its name and its variant.
    Qualities are meta-data that help to categorize the formation.
    """

    __tablename__ = "formation"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(name) > 0", name="name_not_empty_constraint"),
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
        CheckConstraint("length(unit) > 0", name="unit_not_empty_constraint"),
    )
    created_at: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.utcnow, nullable=False
    )
    last_update_at: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.utcnow, nullable=False, onupdate=datetime.utcnow
    )
    kit_id: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="formations")
    pieces: Mapped[List[Piece]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    attractions: Mapped[List[Attraction]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    qualities: Mapped[List[Quality]] = relationship(
        Quality, back_populates="formation", cascade="all, delete-orphan"
    )

    # def __eq__(self, other: object) -> bool:
    #     if not isinstance(other, Formation):
    #         raise NotImplementedError()
    #     return self.local_id == other.local_id

    # def __hash__(self) -> int:
    #     return hash(self.local_id)

    def __repr__(self) -> str:
        return f"Formation(id={self.id!r}, name={self.name!r}, description={self.description!r}, icon={self.icon!r}, variant={self.variant!r}, kit_id={self.kit_id!r}, pieces={self.pieces!r}, attractions={self.attractions!r}, qualities={self.qualities!r})"

    def __str__(self) -> str:
        return f"Formation(id={str(self.id)}, kit_id={str(self.kit_id)})"

    def client__str__(self) -> str:
        return f"Formation(name={self.name}, qualities={list_client__str__(self.qualities)})"

    # @property
    # def parent(self) -> Entity:
    #     return self.kit

    # @property
    # def children(self) -> List[Entity]:
    #     return self.pieces + self.attractions  # type: ignore

    # @property
    # def references(self) -> List[Entity]:
    #     return []

    # @property
    # def referenced_by(self) -> List[Entity]:
    #     return []

    # @property
    # def related_to(self) -> List[Entity]:
    #     return [self.parent] + self.children


@event.listens_for(Piece, "after_update")
def receive_after_update(mapper, connection, target):
    target.formation.last_update_at = datetime.utcnow()


@event.listens_for(Attraction, "after_update")
def receive_after_update(mapper, connection, target):
    target.formation.last_update_at = datetime.utcnow()


# Both Type and Formation can own qualities
@event.listens_for(Quality, "after_update")
def receive_after_update(mapper, connection, target):
    if target.type_id:
        target.type.last_update_at = datetime.utcnow()
    else:
        target.formation.last_update_at = datetime.utcnow()


class Hierarchy(BaseModel):
    class Config:
        arbitrary_types_allowed = True

    piece: Piece
    transform: Transform
    children: Optional[List["Hierarchy"]]


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
    """A kit is a collection of types and formations. It is uniquely identified by its name."""

    __tablename__ = "kit"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(
        String(NAME_LENGTH_MAX),
        CheckConstraint("length(name) > 0", name="name_not_empty_constraint"),
    )
    # Optional. Set to "" for None.
    description: Mapped[str] = mapped_column(Text())
    # Optional. Set to "" for None.
    icon: Mapped[str] = mapped_column(Text())
    created_at: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.utcnow, nullable=False
    )
    last_update_at: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.utcnow, nullable=False, onupdate=datetime.utcnow
    )
    # Optional. Set to "" for None.
    url: Mapped[str] = mapped_column(String(URL_LENGTH_MAX))
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
    # def referenced_by(self) -> List[Entity]:
    #     return []

    # @property
    # def related_to(self) -> List[Entity]:
    #     return self.children


class DirectoryError(SemioException):
    def __init__(self, directory: str):
        self.directory = directory


class DirectoryDoesNotExist(DirectoryError):
    def __str__(self) -> str:
        return "Directory does not exist: " + self.directory


class DirectoryIsNotADirectory(DirectoryError):
    def __str__(self) -> str:
        return "Directory is not a directory: " + self.directory


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
    class Meta:
        model = Representation
        name = "Representation"
        exclude_fields = (
            "id",
            "_tags",
            "type_id",
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


class LocatorNode(SQLAlchemyObjectType):
    class Meta:
        model = Locator
        name = "Locator"
        exclude_fields = ("port_id",)


class PlaneNode(SQLAlchemyObjectType):
    class Meta:
        model = Plane
        name = "Plane"
        exclude_fields = (
            "id",
            "origin_x",
            "origin_y",
            "origin_z",
            "x_axis_x",
            "x_axis_y",
            "x_axis_z",
            "y_axis_x",
            "y_axis_y",
            "y_axis_z",
        )

    origin = NonNull(PointNode)
    x_axis = NonNull(VectorNode)
    y_axis = NonNull(VectorNode)

    def resolve_origin(plane: Plane, info):
        return plane.origin

    def resolve_x_axis(plane: Plane, info):
        return plane.x_axis

    def resolve_y_axis(plane: Plane, info):
        return plane.y_axis


class PortNode(SQLAlchemyObjectType):
    class Meta:
        model = Port
        name = "Port"
        exclude_fields = (
            "id",
            "local_id",
            "plane_id",
            "type_id",
        )

    id = graphene.Field(NonNull(graphene.String))

    def resolve_id(locator: Locator, info):
        return locator.local_id


class QualityNode(SQLAlchemyObjectType):
    class Meta:
        model = Quality
        name = "Quality"
        exclude_fields = ("id", "type_id", "formation_id")


class TypeNode(SQLAlchemyObjectType):
    class Meta:
        model = Type
        name = "Type"
        # interfaces = (ArtifactNode,)
        exclude_fields = (
            "id",
            "kit_id",
        )


class RootPieceNode(PydanticObjectType):
    class Meta:
        model = RootPiece
        name = "RootPiece"
        # plane is not a Pydanctic model and needs to be resolved manually
        exclude_fields = ("plane",)

    plane = graphene.Field(NonNull(PlaneNode))

    def resolve_plane(root, info):
        return root.plane


class DiagramPieceNode(PydanticObjectType):
    class Meta:
        model = DiagramPiece
        name = "DiagramPiece"


class PieceNode(SQLAlchemyObjectType):
    class Meta:
        model = Piece
        name = "Piece"
        exclude_fields = (
            "id",
            "local_id",
            "type_id",
            "root_plane_id",
            "root_plane",
            "diagram_point_x",
            "diagram_point_y",
            "formation_id",
        )

    id = graphene.Field(NonNull(graphene.String))
    root = graphene.Field(RootPieceNode)
    diagram = graphene.Field(NonNull(DiagramPieceNode))

    def resolve_id(piece: Piece, info):
        return piece.local_id

    def resolve_root(piece: Piece, info):
        if piece.root is not None:
            return piece.root

    def resolve_diagram(piece: Piece, info):
        return piece.diagram


class TypePieceSideNode(PydanticObjectType):
    class Meta:
        model = TypePieceSide
        name = "TypePieceSide"
        # port is none Pydanctic model and needs to be resolved manually
        exclude_fields = ("port",)

    port = graphene.Field(PortNode)

    def resolve_port(root, info):
        return root.port


class PieceSideNode(PydanticObjectType):
    class Meta:
        name = "PieceSide"
        model = PieceSide


class SideNode(PydanticObjectType):
    class Meta:
        name = "Side"
        model = Side


class AttractionNode(SQLAlchemyObjectType):
    class Meta:
        model = Attraction
        name = "Attraction"
        exclude_fields = (
            "attracting_piece_id",
            "attracting_piece",
            "attracting_piece_type_port_id",
            "attracting_piece_type_port",
            "attracted_piece_id",
            "attracted_piece",
            "attracted_piece_type_port_id",
            "attracted_piece_type_port",
            "formation_id",
        )

    attracting = graphene.Field(NonNull(SideNode))
    attracted = graphene.Field(NonNull(SideNode))

    def resolve_attracting(attraction: Attraction, info):
        return attraction.attracting

    def resolve_attracted(attraction: Attraction, info):
        return attraction.attracted


class FormationNode(SQLAlchemyObjectType):
    class Meta:
        model = Formation
        name = "Formation"
        # interfaces = (ArtifactNode,)
        exclude_fields = (
            "id",
            "kit_id",
        )


class ObjectNode(PydanticObjectType):
    class Meta:
        model = Object
        name = "Object"
        exclude_fields = ("piece", "plane", "parent")

    piece = graphene.Field(PieceNode)
    plane = graphene.Field(PlaneNode)
    parent = graphene.Field(lambda: ObjectNode)

    def resolve_piece(root, info):
        return root.piece

    def resolve_plane(root, info):
        return root.plane

    def resolve_parent(root, info):
        return root.parent


class SceneNode(PydanticObjectType):
    class Meta:
        model = Scene
        name = "Scene"
        # formation is not a Pydanctic model and needs to be resolved manually
        exclude_fields = ("formation",)

    formation = graphene.Field(FormationNode)

    def resolve_formation(root, info):
        return root.formation


class KitNode(SQLAlchemyObjectType):
    class Meta:
        model = Kit
        name = "Kit"
        # interfaces = (ArtifactNode,)
        exclude_fields = ("id",)


class RepresentationInput(InputObjectType):
    url = NonNull(graphene.String)
    lod = graphene.String()
    tags = graphene.List(NonNull(graphene.String))


class LocatorInput(InputObjectType):
    group = NonNull(graphene.String)
    subgroup = graphene.String()


class ScreenPointInput(PydanticInputObjectType):
    class Meta:
        model = ScreenPoint


class PointInput(PydanticInputObjectType):
    class Meta:
        model = Point


class VectorInput(PydanticInputObjectType):
    class Meta:
        model = Vector


class PlaneInput(InputObjectType):
    origin = NonNull(PointInput)
    x_axis = NonNull(VectorInput)
    y_axis = NonNull(VectorInput)


class PortInput(InputObjectType):
    id = graphene.String()
    plane = NonNull(PlaneInput)
    locators = graphene.List(NonNull(LocatorInput))


class PortIdInput(InputObjectType):
    id = graphene.String()


class QualityInput(InputObjectType):
    name = NonNull(graphene.String)
    value = graphene.String()
    unit = graphene.String()


class TypeInput(InputObjectType):
    name = NonNull(graphene.String)
    description = graphene.String()
    icon = graphene.String()
    variant = graphene.String()
    unit = NonNull(graphene.String)
    representations = NonNull(graphene.List(NonNull(RepresentationInput)))
    ports = NonNull(graphene.List(NonNull(PortInput)))
    qualities = graphene.List(NonNull(QualityInput))


class TypeIdInput(InputObjectType):
    name = NonNull(graphene.String)
    variant = graphene.String()


class RootPieceInput(PydanticInputObjectType):
    class Meta:
        model = RootPiece
        # plane is none Pydanctic model and needs to be resolved manually
        exclude_fields = ("plane",)

    plane = NonNull(PlaneInput)


class DiagramPieceInput(PydanticInputObjectType):
    class Meta:
        model = DiagramPiece


class PieceInput(InputObjectType):
    id = NonNull(graphene.String)
    type = NonNull(TypeIdInput)
    root = graphene.Field(RootPieceInput)
    diagram = NonNull(DiagramPieceInput)


class TypePieceSideInput(InputObjectType):
    port = graphene.Field(PortIdInput)


class PieceSideInput(InputObjectType):
    id = NonNull(graphene.String)
    type = graphene.Field(TypePieceSideInput)


class SideInput(InputObjectType):
    piece = NonNull(PieceSideInput)


class AttractionInput(InputObjectType):
    attracting = NonNull(SideInput)
    attracted = NonNull(SideInput)


class FormationInput(InputObjectType):
    name = NonNull(graphene.String)
    description = graphene.String()
    icon = graphene.String()
    variant = graphene.String()
    unit = NonNull(graphene.String)
    pieces = NonNull(graphene.List(NonNull(PieceInput)))
    attractions = NonNull(graphene.List(NonNull(AttractionInput)))
    qualities = graphene.List(NonNull(QualityInput))


class FormationIdInput(InputObjectType):
    name = NonNull(graphene.String)
    variant = graphene.String()


class KitInput(InputObjectType):
    name = NonNull(graphene.String)
    description = graphene.String()
    icon = graphene.String()
    url = graphene.String()
    types = graphene.List(NonNull(TypeInput))
    formations = graphene.List(NonNull(FormationInput))


class KitMetadataInput(InputObjectType):
    name = graphene.String()
    description = graphene.String()
    icon = graphene.String()
    url = graphene.String()


class NotFound(SpecificationError):
    def __init__(self, id) -> None:
        self.id = id

    def __str__(self):
        return f"{self.id} not found."


class RepresentationNotFound(NotFound):
    def __init__(self, type, url) -> None:
        super().__init__(url)
        self.type = type
        self.url = url

    def __str__(self):
        return f"Representation({self.url}) not found for type: {str(self.type)}"


class PortNotFound(NotFound):
    def __init__(self, qualities) -> None:
        super().__init__(qualities)
        self.qualities = qualities

    def __str__(self):
        return f"Port({self.qualities}) not found."


class TypeNotFound(NotFound):
    def __init__(self, name) -> None:
        super().__init__(name)
        self.name = name

    def __str__(self):
        return f"Type({self.name}) not found."


class QualitiesDontMatchType(TypeNotFound):
    def __init__(
        self, name, qualityInputs: List[QualityInput], types: List[Type]
    ) -> None:
        super().__init__(name)
        self.qualityInputs = qualityInputs
        self.types = types

    def __str__(self):
        return f"Qualities({self.qualityInputs}) don't match any type with name {self.name}: {list_client__str__(self.types)}"


class TooLittleQualitiesToMatchExcactlyType(QualitiesDontMatchType):
    def __str__(self):
        return f"Too little qualities ({self.qualityInputs}) to match exactly one type name {self.name}: {list_client__str__(self.types)}"


class PieceNotFound(NotFound):
    def __init__(self, formation, local_id) -> None:
        super().__init__(local_id)
        self.formation = formation
        self.local_id = local_id

    def __str__(self):
        return f"Piece({self.local_id}) not found. Please check that the local id is correct and that the piece is part of the formation {str(self.formation)}"


class AttractionNotFound(NotFound):
    def __init__(self, formation, attracting, attracted) -> None:
        super().__init__((attracting, attracted))
        self.formation = formation
        self.attracting = attracting
        self.attracted = attracted

    def __str__(self):
        return f"Attraction with attracting piece id ({self.attracting}) and attracted piece id ({self.attracted}) not found in formation {str(self.formation)}"


class FormationNotFound(NotFound):
    def __init__(self, name) -> None:
        super().__init__(name)
        self.name = name

    def __str__(self):
        return f"Formation({self.name}) not found."


class QualitiesDontMatchFormation(FormationNotFound):
    def __init__(
        self, name, qualityInputs: List[QualityInput], formations: List[Formation]
    ) -> None:
        super().__init__(name)
        self.qualityInputs = qualityInputs
        self.formations = formations

    def __str__(self):
        return f"Qualities ({self.qualityInputs}) don't match any formation with name {self.name}: {str(self.formations)}"


class TooLittleQualitiesToMatchExcactlyFormation(QualitiesDontMatchFormation):
    def __str__(self):
        return f"Too little qualities ({self.qualityInputs}) to match exactly one formation name {self.name}: {str(self.formations)}"


class KitNotFound(NotFound):
    def __init__(self, name) -> None:
        super().__init__(name)
        self.name = name

    def __str__(self):
        return f"Kit({self.name}) not found."


class NoMainKit(KitNotFound):
    def __init__(self) -> None:
        super().__init__("main")

    def __str__(self):
        return f"Main kit not found."


class AlreadyExists(SpecificationError):
    def __init__(self, id, existing) -> None:
        self.id = id
        self.existing = existing

    def __str__(self):
        return f"{self.id!r} already exists: {str(self.existing)}"


class RepresentationAlreadyExists(AlreadyExists):
    def __init__(self, representation) -> None:
        super().__init__(representation.url, representation)
        self.representation = representation

    def __str__(self):
        return f"Representation with url: {self.representation.url!r} already exists: {str(self.representation)}"


class PortAlreadyExists(AlreadyExists):
    def __init__(self, id) -> None:
        super().__init__(id, id)

    def __str__(self):
        return f"Port with id: {self.id!r} already exists."


class AttractionAlreadyExists(AlreadyExists):
    def __init__(self, attraction: Attraction, existingAttraction: Attraction) -> None:
        super().__init__(
            (attraction.attracting.piece.id, attraction.attracted.piece.id),
            existingAttraction,
        )
        self.attraction = attraction

    def __str__(self):
        return f"Attraction with attracting piece id ({self.attraction.attracting.piece.id}) and attracted piece id ({self.attraction.attracted.piece.id}) already exists: {self.existing.client__str__()}"


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


class KitAlreadyExists(DocumentAlreadyExists):
    def __init__(self, kit) -> None:
        super().__init__(kit)
        self.kit = kit

    def __str__(self):
        return f"Kit ({self.kit.name}) already exists: {str(self.kit)}"


def getMainKit(session: Session) -> Kit:
    kit = session.query(Kit).first()
    if not kit:
        raise NoMainKit()
    return kit


def qualityInputToTransientQualityForEquality(qualityInput: QualityInput) -> Quality:
    try:
        value = qualityInput.value
    except AttributeError:
        value = ""
    try:
        unit = qualityInput.unit
    except AttributeError:
        unit = ""
    return Quality(
        name=qualityInput.name,
        value=value,
        unit=unit,
    )


# def locatorInputToTransientLocatorForEquality(
#     locatorInput: LocatorInput,
# ) -> Locator:
#     try:
#         subgroup = locatorInput.subgroup
#     except AttributeError:
#         subgroup = ""
#     return Locator(group=locatorInput.group, subgroup=subgroup)


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


# def getTypeByNameAndQualities(
#     session: Session, name: String, qualityInputs: List[QualityInput]
# ) -> Type:
#     typesWithSameName = session.query(Type).filter_by(name=name)
#     if typesWithSameName.count() < 1:
#         raise TypeNotFound(name)
#     typesWithSameName = typesWithSameName.all()
#     qualities = (
#         [
#             qualityInputToTransientQualityForEquality(qualityInput)
#             for qualityInput in qualityInputs
#         ]
#         if qualityInputs
#         else []
#     )
#     typesWithSameQualities = [
#         type
#         for type in typesWithSameName
#         if set(qualities).issubset(set(type.qualities))
#     ]
#     if len(typesWithSameQualities) < 1:
#         raise QualitiesDontMatchType(name, qualityInputs, typesWithSameName)
#     elif len(typesWithSameQualities) > 1:
#         typesWithExactSameQualities = [
#             type
#             for type in typesWithSameQualities
#             if set(type.qualities) == set(qualities)
#         ]
#         if len(typesWithExactSameQualities) < 1:
#             raise TooLittleQualitiesToMatchExcactlyType(
#                 name, qualityInputs, typesWithSameQualities
#             )
#         elif len(typesWithExactSameQualities) > 1:
#             raise InvalidDatabase(
#                 f"Found multiple types {typesWithExactSameQualities!r} for {name} and same qualities: {qualities}"
#             )
#         return typesWithExactSameQualities[0]
#     return typesWithSameQualities[0]


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


# def getFormationByNameAndQualities(
#     session: Session, name: String, qualityInputs: List[QualityInput]
# ) -> Formation:
#     formationsWithSameName = session.query(Formation).filter_by(name=name)
#     if formationsWithSameName.count() < 1:
#         raise FormationNotFound(name)
#     formationsWithSameName = formationsWithSameName.all()
#     qualities = (
#         [
#             qualityInputToTransientQualityForEquality(qualityInput)
#             for qualityInput in qualityInputs
#         ]
#         if qualityInputs
#         else []
#     )
#     formationsWithSameQualities = [
#         formation
#         for formation in formationsWithSameName
#         if set(qualities).issubset(set(formation.qualities))
#     ]
#     if len(formationsWithSameQualities) < 1:
#         raise QualitiesDontMatchFormation(name, qualityInputs, formationsWithSameName)
#     elif len(formationsWithSameQualities) > 1:
#         formationsWithExactSameQualities = [
#             formation
#             for formation in formationsWithSameQualities
#             if set(formation.qualities) == set(qualities)
#         ]
#         if len(formationsWithExactSameQualities) < 1:
#             raise TooLittleQualitiesToMatchExcactlyFormation(
#                 name, qualityInputs, formationsWithSameQualities
#             )
#         elif len(formationsWithExactSameQualities) > 1:
#             raise InvalidDatabase(
#                 f"Found multiple formations {formationsWithExactSameQualities!r} for {name} and same qualities: {qualities}"
#             )
#         return formationsWithExactSameQualities[0]
#     return formationsWithSameQualities[0]


def getFormationByNameAndVariant(
    session: Session, name: str, variant: str
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
        raise FormationNotFound(name)
    return formation


# def getPortByLocators(
#     session: Session, type: Formation, locatorInputs: List[LocatorInput]
# ) -> Port:
#     ports = session.query(Port).filter_by(type_id=type.id)
#     locators = [
#         locatorInputToTransientLocatorForEquality(locatorInput)
#         for locatorInput in locatorInputs
#     ]
#     portsWithSameLocator = [
#         port for port in ports if set(port.locators) == set(locators)
#     ]
#     if len(portsWithSameLocator) != 1:
#         raise PortNotFound(locators)
#     return portsWithSameLocator[0]


def getPortById(session: Session, type: Type, portId: str) -> Port:
    port = session.query(Port).filter_by(type_id=type.id, local_id=portId).first()
    if not port:
        raise PortNotFound(portId)
    return port


def getAttractionByPieceIds(
    session: Session,
    formation: Formation,
    attractingPieceId: str,
    attractedPieceId: str,
) -> Attraction:
    attraction = (
        session.query(Attraction)
        .filter_by(
            attracting_piece_id=attractingPieceId,
            attracted_piece_id=attractedPieceId,
            formation_id=formation.id,
        )
        .first()
    )
    if not attraction:
        raise AttractionNotFound(attractingPieceId, attractedPieceId, formation)
    return attraction


def addRepresentationInputToSession(
    session: Session,
    type: Type,
    representationInput: RepresentationInput,
) -> Representation:
    try:
        lod = representationInput.lod
    except AttributeError:
        lod = ""
    try:
        representation = getRepresentationByUrl(session, type, representationInput.url)
        raise RepresentationAlreadyExists(representation)
    except RepresentationNotFound:
        pass
    representation = Representation(
        url=representationInput.url,
        lod=lod,
        type_id=type.id,
    )
    session.add(representation)
    session.flush()
    for tagInput in representationInput.tags or []:
        tag = Tag(
            value=tagInput,
            representation_id=representation.id,
        )
        session.add(tag)
        session.flush()
    return representation


def addLocatorInputToSession(
    session: Session, port: Port, locatorInput: LocatorInput
) -> Locator:
    try:
        subgroup = locatorInput.subgroup
    except AttributeError:
        subgroup = ""
    locator = Locator(group=locatorInput.group, subgroup=subgroup, port_id=port.id)
    session.add(locator)
    session.flush()
    return locator


def addPortInputToSession(session: Session, type: Type, portInput: PortInput) -> Port:
    try:
        id = portInput.id
    except AttributeError:
        id = ""
    try:
        existingPort = getPortById(session, type, id)
        raise PortAlreadyExists(existingPort)
    except PortNotFound:
        pass
    plane = Plane(
        origin_x=portInput.plane.origin.x,
        origin_y=portInput.plane.origin.y,
        origin_z=portInput.plane.origin.z,
        x_axis_x=portInput.plane.x_axis.x,
        x_axis_y=portInput.plane.x_axis.y,
        x_axis_z=portInput.plane.x_axis.z,
        y_axis_x=portInput.plane.y_axis.x,
        y_axis_y=portInput.plane.y_axis.y,
        y_axis_z=portInput.plane.y_axis.z,
    )
    session.add(plane)
    session.flush()
    port = Port(
        local_id=id,
        plane_id=plane.id,
        type_id=type.id,
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
        unit = qualityInput.unit
    except AttributeError:
        unit = ""
    try:
        value = qualityInput.value
    except AttributeError:
        value = ""
    typeId = owner.id if isinstance(owner, Type) else None
    formationId = owner.id if isinstance(owner, Formation) else None
    quality = Quality(
        name=qualityInput.name,
        value=value,
        unit=unit,
        type_id=typeId,
        formation_id=formationId,
    )
    session.add(quality)
    session.flush()
    return quality


def addTypeInputToSession(session: Session, kit: Kit, typeInput: TypeInput) -> Type:
    try:
        description = typeInput.description
    except AttributeError:
        description = ""
    try:
        icon = typeInput.icon
    except AttributeError:
        icon = ""
    try:
        variant = typeInput.variant
    except AttributeError:
        variant = ""
    try:
        existingType = getTypeByNameAndVariant(session, typeInput.name, variant)
        raise TypeAlreadyExists(existingType)
    except TypeNotFound:
        pass
    type = Type(
        name=typeInput.name,
        description=description,
        icon=icon,
        variant=variant,
        unit=typeInput.unit,
        kit_id=kit.id,
    )
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
        variant = pieceInput.type.variant
    except AttributeError:
        variant = ""
    type = getTypeByNameAndVariant(session, pieceInput.type.name, variant)
    try:
        root_plane = Plane(
            origin_x=pieceInput.root.plane.origin.x,
            origin_y=pieceInput.root.plane.origin.y,
            origin_z=pieceInput.root.plane.origin.z,
            x_axis_x=pieceInput.root.plane.x_axis.x,
            x_axis_y=pieceInput.root.plane.x_axis.y,
            x_axis_z=pieceInput.root.plane.x_axis.z,
            y_axis_x=pieceInput.root.plane.y_axis.x,
            y_axis_y=pieceInput.root.plane.y_axis.y,
            y_axis_z=pieceInput.root.plane.y_axis.z,
        )
        session.add(root_plane)
        session.flush()
        root_plane_id = root_plane.id
    except AttributeError:
        root_plane_id = None
    piece = Piece(
        local_id=pieceInput.id,
        type_id=type.id,
        root_plane_id=root_plane_id,
        diagram_point_x=pieceInput.diagram.point.x,
        diagram_point_y=pieceInput.diagram.point.y,
        formation_id=formation.id,
    )
    session.add(piece)
    session.flush()
    return piece


def addAttractionInputToSession(
    session: Session,
    formation: Formation,
    attractionInput: AttractionInput,
    localIdToPiece: dict,
) -> Attraction:
    try:
        attracting_piece_type_port_id = attractionInput.attracting.piece.type.port.id
    except AttributeError:
        attracting_piece_type_port_id = ""
    try:
        attracted_piece_type_port_id = attractionInput.attracted.piece.type.port.id
    except AttributeError:
        attracted_piece_type_port_id = ""
    try:
        # TODO: Somehow the flushing of the other attractions works but you can't query for them.
        # When I try to look for an existing attraction it finds none but when adding it,
        # it raises a proper IntegrityError
        existingAttraction = getAttractionByPieceIds(
            session,
            formation,
            attractionInput.attracting.piece.id,
            attractionInput.attracted.piece.id,
        )
        raise AttractionAlreadyExists(attractionInput, existingAttraction)
    except AttractionNotFound:
        pass
    try:
        attractingPiece = localIdToPiece[attractionInput.attracting.piece.id]
    except KeyError:
        raise PieceNotFound(formation, attractionInput.attracting.piece.id)
    try:
        attractedPiece = localIdToPiece[attractionInput.attracted.piece.id]
    except KeyError:
        raise PieceNotFound(formation, attractionInput.attracted.piece.id)
    attractingPieceTypePort = getPortById(
        session,
        attractingPiece.type,
        attracting_piece_type_port_id,
    )
    attractedPieceTypePort = getPortById(
        session,
        attractedPiece.type,
        attracted_piece_type_port_id,
    )
    attraction = Attraction(
        attracting_piece_id=attractingPiece.id,
        attracting_piece_type_port_id=attractingPieceTypePort.id,
        attracted_piece_id=attractedPiece.id,
        attracted_piece_type_port_id=attractedPieceTypePort.id,
        formation_id=formation.id,
    )
    session.add(attraction)
    session.flush()
    return attraction


def addFormationInputToSession(
    session: Session, kit: Kit, formationInput: FormationInput
):
    try:
        description = formationInput.description
    except AttributeError:
        description = ""
    try:
        icon = formationInput.icon
    except AttributeError:
        icon = ""
    try:
        variant = formationInput.variant
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
        kit_id=kit.id,
    )
    session.add(formation)
    session.flush()
    localIdToPiece: Dict[str, Piece] = {}
    for pieceInput in formationInput.pieces or []:
        piece = addPieceInputToSession(session, formation, pieceInput)
        localIdToPiece[pieceInput.id] = piece
    for attractionInput in formationInput.attractions or []:
        attraction = addAttractionInputToSession(
            session, formation, attractionInput, localIdToPiece
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
        kit.description = kitInput.description
    except AttributeError:
        pass
    try:
        kit.icon = kitInput.icon
    except AttributeError:
        pass
    try:
        kit.url = kitInput.url
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
        kit.name = kitMetadata.name
    except AttributeError:
        pass
    try:
        kit.description = kitMetadata.description
    except AttributeError:
        pass
    try:
        kit.icon = kitMetadata.icon
    except AttributeError:
        pass
    try:
        kit.url = kitMetadata.url
    except AttributeError:
        pass
    return kit


def hierarchiesFromFormation(formation: Formation) -> List[Hierarchy]:
    nodes = list((piece.local_id, {"piece": piece}) for piece in formation.pieces)
    edges = (
        (
            attraction.attracting.piece.id,
            attraction.attracted.piece.id,
            {"attraction": attraction},
        )
        for attraction in formation.attractions
    )
    graph = DiGraph()
    graph.add_nodes_from(nodes)
    graph.add_edges_from(edges)
    hierarchies = []
    for component in weakly_connected_components(graph):
        connected_subgraph = graph.subgraph(component)
        root = [node for node, degree in connected_subgraph.in_degree() if degree == 0][
            0
        ]
        if not root:
            root = graph.nodes[0]
        rootHierarchy = Hierarchy(
            piece=graph.nodes[root]["piece"],
            transform=Transform.identity(),
            children=[],
        )
        connected_subgraph.nodes[root]["hierarchy"] = rootHierarchy
        for parent, child in bfs_tree(connected_subgraph, source=root).edges():
            parentTransform = connected_subgraph[parent][child][
                "attraction"
            ].attracting.piece.type.port.plane.toTransform()
            childTransform = (
                connected_subgraph[parent][child]["attraction"]
                .attracted.piece.type.port.plane.toTransform()
                .invert()
            )
            transform = parentTransform.after(childTransform)
            hierarchy = Hierarchy(
                piece=connected_subgraph.nodes[child]["piece"],
                transform=transform,
                children=[],
            )
            connected_subgraph.nodes[child]["hierarchy"] = hierarchy
            connected_subgraph.nodes[parent]["hierarchy"].children.append(hierarchy)
        hierarchies.append(rootHierarchy)
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
        variant = formationIdInput.variant
    except AttributeError:
        variant = ""
    formation = getFormationByNameAndVariant(session, formationIdInput.name, variant)
    hierarchies = hierarchiesFromFormation(formation)
    scene = Scene(formation=formation, objects=[])
    for hierarchy in hierarchies:
        addObjectsToScene(
            scene,
            None,
            hierarchy,
            hierarchy.piece.root_plane if hierarchy.piece.root_plane else Plane.XY(),
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
                variant = typeId.variant
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
                variant = formationId.variant
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

engine = Starlette()
engine.mount("/graphql", GraphQLApp(schema, on_get=make_graphiql_handler()))

parser = ArgumentParser()
parser.add_argument("--debug", action="store_true", help="Enable debug mode")
args = parser.parse_args()
if args.debug:
    with open("../../graphql/schema.graphql", "w") as f:
        f.write(str(schema))

    metadata_engine = create_engine("sqlite:///debug/semio.db")
    Base.metadata.create_all(metadata_engine)


def start_engine():
    run(
        engine,
        host=HOST,
        port=PORT,
        log_level="info",
        access_log=False,
        log_config=None,
    )


def restart_engine():
    ui_instance = QApplication.instance()
    engine_process = ui_instance.engine_process
    if engine_process.is_alive():
        engine_process.terminate()
    ui_instance.engine_process = Process(target=start_engine)
    ui_instance.engine_process.start()


def watcher():
    ui_instance = QApplication.instance()
    try:
        engine_process = ui_instance.engine_process
    except AttributeError:
        start_engine()
    while True:
        if not engine_process.is_alive():
            restart_engine()
        sleep(1)


if __name__ == "__main__":
    freeze_support()

    ui = QApplication(sys.argv)
    ui.setQuitOnLastWindowClosed(False)

    # final location of assests when bundeled with PyInstaller
    if getattr(sys, "frozen", False):
        basedir = sys._MEIPASS
    else:
        basedir = "../.."

    icon = QIcon()
    icon.addFile(os.path.join(basedir, "icons/semio_16x16.png"), QSize(16, 16))
    icon.addFile(os.path.join(basedir, "icons/semio_32x32.png"), QSize(32, 32))
    icon.addFile(os.path.join(basedir, "icons/semio_48x48.png"), QSize(48, 48))
    icon.addFile(os.path.join(basedir, "icons/semio_128x128.png"), QSize(128, 128))
    icon.addFile(os.path.join(basedir, "icons/semio_256x256.png"), QSize(256, 256))

    tray = QSystemTrayIcon()
    tray.setIcon(icon)
    tray.setVisible(True)

    menu = QMenu()
    restart = QAction("Restart")
    restart.triggered.connect(restart_engine)
    menu.addAction(restart)

    quit = QAction("Quit")
    # kill engine and quit ui
    quit.triggered.connect(lambda: ui.engine_process.terminate() or ui.quit())
    menu.addAction(quit)

    tray.setContextMenu(menu)

    ui.watcher_process = Process(target=watcher)
    ui.watcher_process.start()

    sys.exit(ui.exec())
