#!/usr/bin/env python

# semio
# Copyright (C) 2023 Ueli Saluz

# This program is free software: you can redistribute it and/or modify
# it under the terms of the GNU Lesser General Public License as published by
# the Free Software Foundation, either version 3 of the License, or
# any later version.

# This program is distributed in the hope that it will be useful,
# but WITHOUT ANY WARRANTY; without even the implied warranty of
# MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
# GNU Lesser General Public License for more details.

# You should have received a copy of the GNU Lesser General Public License
# along with this program.  If not, see <http://www.gnu.org/licenses/>.

"""
API for semio.
"""

# TODO: Refactor Properties polymorphism to a cleaner solution.
#       Try to mainly get rid of Open-Closed Principle violations.
# TODO: Check if sqlmodel can replace SQLAlchemy:
#       ✅Constraints
#       ❔Polymorphism
#       ❔graphene_sqlalchemy


from os import remove
from pathlib import Path
from functools import lru_cache
import typing
from typing import Optional, Dict, Protocol
from dataclasses import dataclass
from enum import Enum
import decimal
from decimal import Decimal
from hashlib import sha256
from pydantic import BaseModel
import sqlalchemy
from sqlalchemy import (
    String,
    Text,
    Numeric,
    Integer,
    ForeignKey,
    create_engine,
    UniqueConstraint,
    CheckConstraint,
)
from sqlalchemy.orm import (
    DeclarativeBase,
    Mapped,
    mapped_column,
    relationship,
    backref,
    sessionmaker,
    Session,
)
import graphene
from graphene import Schema, Mutation, ObjectType, InputObjectType, Field, NonNull
from graphene_sqlalchemy import (
    SQLAlchemyObjectType,
    SQLAlchemyInterface,
)
from graphene_pydantic import PydanticObjectType, PydanticInputObjectType
from flask import Flask
from graphql_server.flask import GraphQLView

NAME_LENGTH_MAX = 100
SYMBOL_LENGTH_MAX = 1
URL_LENGTH_MAX = 1000
KIT_FILENAME = "kit.semio"


def canonicalize_name(name: str) -> str:
    return name.strip().lower().replace(" ", "_")


def canonicalize_number(number: decimal.Decimal) -> str:
    return format(number, ".6f")


class SemioException(Exception):
    pass


class InvalidDatabase(SemioException):
    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + "\n The database is invalid. Please report this bug."


class InvalidBackend(SemioException):
    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + "\n The backend is invalid. Please report this bug."


# TODO: Refactor Protocol to ABC and make it work with SQLAlchemy
class Artifact(Protocol):
    name: str
    explanation: str
    symbol: str

    @property
    def parent(self):
        pass

    @property
    def children(self):
        pass

    @property
    def references(self):
        pass

    @property
    def referenced_by(self):
        pass

    @property
    def related_to(self):
        (
            ([self.parent] if self.parent else [])
            + self.children
            + self.references
            + self.referenced_by
        )


class Geometry(BaseModel):
    pass


class Point(Geometry):
    x: decimal.Decimal
    y: decimal.Decimal
    z: decimal.Decimal


class Vector(Geometry):
    x: decimal.Decimal
    y: decimal.Decimal
    z: decimal.Decimal


class Plane(Geometry):
    origin: Point
    x_axis: Vector
    y_axis: Vector


# TODO inherit from speckle
class Brep(Geometry):
    pass


class Base(DeclarativeBase):
    pass


class PropertyDatatype(Enum):
    # 001 - 099: numbers
    DECIMAL = 1
    INTEGER = 2
    NATURAL = 3
    # 011 - 019: logical
    BOOLEAN = 11
    # 101 - 199: text
    DESCRIPTION = 101
    CHOICE = 102
    # 201 - 299: geometry
    BREP = 201
    # 1001 - 9999: files
    GLTF = 1001
    # 10001 - 10099: containers
    LIST = 10001


class Property(Base):
    __tablename__ = "property"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    datatype: Mapped[PropertyDatatype] = mapped_column(
        sqlalchemy.Enum(PropertyDatatype)
    )
    type_id: Mapped[Optional[int]] = mapped_column(ForeignKey("type.id"))
    type: Mapped[Optional["Type"]] = relationship("Type", back_populates="properties")
    port_id: Mapped[Optional[int]] = mapped_column(ForeignKey("port.id"))
    port: Mapped[Optional["Port"]] = relationship("Port", back_populates="properties")
    list_id: Mapped[Optional[int]] = mapped_column(ForeignKey("list_property.id"))
    list_index: Mapped[Optional[int]] = mapped_column(
        Integer(), CheckConstraint("list_index>=0")
    )
    list = relationship(
        "ListProperty",
        back_populates="properties",
        # remote_side=[id],
        foreign_keys=[list_id],
    )

    __mapper_args__ = {"polymorphic_on": datatype, "polymorphic_abstract": True}

    __table_args__ = (
        CheckConstraint(
            "(type_id IS NOT NULL AND port_id IS NULL AND list_id IS NULL) OR "
            "(type_id IS NULL AND port_id IS NOT NULL AND list_id IS NULL) OR "
            "(type_id IS NULL AND port_id IS NULL AND list_id IS NOT NULL)",
            name="owner_present_and_unique_constraint",
        ),
        CheckConstraint(
            "(list_id IS NOT NULL AND list_index IS NOT NULL) OR "
            "(list_id IS NULL AND list_index IS NULL)",
            name="list_index_present_per_list_constraint",
        ),
        UniqueConstraint(
            "list_id", "list_index", name="list_index_unique_per_list_constraint"
        ),
    )

    def __repr__(self) -> str:
        return f"Property(id={self.id!r}, name={self.name!r}, {self.owner_kind_name!r}_id={self.owner_id!r})"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Property):
            return NotImplemented
        return (
            canonicalize_name(self.name) == canonicalize_name(other.name)
            and self.value == other.value
        )

    @property
    def owner_id(self) -> Artifact:
        if self.type_id is not None:
            return self.type_id
        if self.port_id is not None:
            return self.port_id
        if self.list_id is not None:
            return self.list_id
        raise InvalidDatabase(f"Property{self!r} has no owner.")

    @property
    def owner_kind_name(self) -> str:
        if self.type_id is not None:
            return "type"
        if self.port_id is not None:
            return "port"
        if self.list_id is not None:
            return "list"
        raise InvalidDatabase(f"Property{self!r} has no owner.")

    @property
    def parent(self) -> Artifact:
        if self.type_id is not None:
            return self.type
        if self.port_id is not None:
            return self.port
        if self.list_id is not None:
            return self.list
        raise InvalidDatabase(f"Property{self!r} has no owner.")

    @property
    def children(self) -> typing.List[Artifact]:
        return []

    @property
    def references(self) -> typing.List[Artifact]:
        return []

    @property
    def referenced_by(self) -> typing.List[Artifact]:
        return []

    @property
    def related_to(self) -> typing.List[Artifact]:
        return [self.parent] + self.children + self.references + self.referenced_by


class ScalarProperty(Property):
    __mapper_args__ = {"polymorphic_abstract": True}

    @property
    def value(self) -> str:
        return str(self.scalar)

    @property
    def hash(self) -> str:
        return sha256(self.value.encode()).hexdigest()


class NumberProperty(ScalarProperty):
    __mapper_args__ = {"polymorphic_abstract": True}

    @property
    def scalar(self) -> decimal.Decimal:
        return decimal.Decimal(self.number)

    def __str__(self) -> str:
        return canonicalize_number(self.scalar)


class DecimalProperty(NumberProperty):
    __tablename__ = "decimal_property"

    id: Mapped[int] = mapped_column(ForeignKey("property.id"), primary_key=True)
    decimal: Mapped[Numeric] = mapped_column(Numeric())

    __mapper_args__ = {
        "polymorphic_identity": PropertyDatatype.DECIMAL,
    }

    def __repr__(self) -> str:
        return f"Decimal(id={self.id!r}, name={self.name!r}, {self.owner_kind_name!r}_id={self.owner_id!r})"

    @property
    def number(self) -> Decimal:
        return self.decimal


class IntegerProperty(NumberProperty):
    __tablename__ = "integer_property"

    id: Mapped[int] = mapped_column(ForeignKey("property.id"), primary_key=True)
    integer: Mapped[int] = mapped_column(Integer())

    __mapper_args__ = {
        "polymorphic_identity": PropertyDatatype.INTEGER,
    }

    def __repr__(self) -> str:
        return f"Integer(id={self.id!r}, name={self.name!r}, {self.owner_kind_name!r}_id={self.owner_id!r})"

    @property
    def number(self) -> int:
        return self.integer


class NaturalProperty(NumberProperty):
    __tablename__ = "natural_property"

    id: Mapped[int] = mapped_column(ForeignKey("property.id"), primary_key=True)
    natural: Mapped[int] = mapped_column(Integer(), CheckConstraint("natural>=0"))
    __mapper_args__ = {
        "polymorphic_identity": PropertyDatatype.NATURAL,
    }

    def __repr__(self) -> str:
        return f"Natural(id={self.id!r}, name={self.name!r}, {self.owner_kind_name!r}_id={self.owner_id!r})"

    @property
    def number(self) -> int:
        return self.natural


class LogicalProperty(ScalarProperty):
    __mapper_args__ = {"polymorphic_abstract": True}

    @property
    def scalar(self) -> decimal.Decimal:
        return decimal.Decimal(self.logical)

    def __str__(self) -> str:
        return canonicalize_number(self.scalar)


class BooleanProperty(LogicalProperty):
    __tablename__ = "boolean_property"

    id: Mapped[int] = mapped_column(ForeignKey("property.id"), primary_key=True)
    boolean: Mapped[bool] = mapped_column(Integer())

    __mapper_args__ = {
        "polymorphic_identity": PropertyDatatype.BOOLEAN,
    }

    def __repr__(self) -> str:
        return f"Boolean(id={self.id!r}, name={self.name!r}, {self.owner_kind_name!r}_id={self.owner_id!r})"

    @property
    def logical(self) -> bool:
        return self.boolean


class TextProperty(ScalarProperty):
    __mapper_args__ = {"polymorphic_abstract": True}

    @property
    def scalar(self) -> str:
        return self.text


class DescriptionProperty(TextProperty):
    __tablename__ = "description_property"

    id: Mapped[int] = mapped_column(ForeignKey("property.id"), primary_key=True)
    description: Mapped[str] = mapped_column(Text())

    __mapper_args__ = {
        "polymorphic_identity": PropertyDatatype.DESCRIPTION,
    }

    def __repr__(self) -> str:
        return f"Description(id={self.id!r}, name={self.name!r}, {self.owner_kind_name!r}_id={self.owner_id!r})"

    @property
    def text(self) -> str:
        return self.description


class ChoiceProperty(TextProperty):
    __tablename__ = "choice_property"

    id: Mapped[int] = mapped_column(ForeignKey("property.id"), primary_key=True)
    choice: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))

    __mapper_args__ = {
        "polymorphic_identity": PropertyDatatype.CHOICE,
    }

    def __repr__(self) -> str:
        return f"Choice(id={self.id!r}, name={self.name!r}, {self.owner_kind_name!r}_id={self.owner_id!r})"

    @property
    def text(self) -> str:
        return self.choice


class GeometryProperty(ScalarProperty):
    __mapper_args__ = {"polymorphic_abstract": True}

    @property
    def scalar(self) -> Geometry:
        return self.geometry


class BrepProperty(GeometryProperty):
    __tablename__ = "brep_property"

    id: Mapped[int] = mapped_column(ForeignKey("property.id"), primary_key=True)
    brep: Mapped[Brep] = mapped_column(Text())

    __mapper_args__ = {
        "polymorphic_identity": PropertyDatatype.BREP,
    }

    def __repr__(self) -> str:
        return f"Brep(id={self.id!r}, name={self.name!r}, {self.owner_kind_name!r}_id={self.owner_id!r})"

    @property
    def geometry(self) -> Brep:
        return self.brep


class ContainerProperty(Property):
    __mapper_args__ = {"polymorphic_abstract": True}

    @property
    def value(self) -> str:
        return str(self.hash)


class ListProperty(ContainerProperty):
    __tablename__ = "list_property"

    id: Mapped[int] = mapped_column(ForeignKey("property.id"), primary_key=True)
    properties: Mapped[typing.List[Property]] = relationship(
        Property,
        back_populates="list",
        cascade="all, delete-orphan",
        foreign_keys=[Property.list_id],
    )

    __mapper_args__ = {
        "polymorphic_identity": PropertyDatatype.LIST,
        "inherit_condition": (id == Property.id),
    }

    def __repr__(self) -> str:
        return f"ListProperty(id={self.id!r}, name={self.name!r}, {self.owner_kind_name!r}_id={self.owner_id!r})"

    @property
    def list(self) -> typing.List[Property]:
        return sorted(self.properties, key=lambda property: property.list_index)

    @property
    def hash(self) -> str:
        return sha256("\n".join([p.hash for p in self.list]).encode()).hexdigest()


class Port(Base):
    __tablename__ = "port"

    id: Mapped[int] = mapped_column(primary_key=True)
    origin_x: Mapped[decimal.Decimal] = mapped_column(Numeric())
    origin_y: Mapped[decimal.Decimal] = mapped_column(Numeric())
    origin_z: Mapped[decimal.Decimal] = mapped_column(Numeric())
    x_axis_x: Mapped[decimal.Decimal] = mapped_column(Numeric())
    x_axis_y: Mapped[decimal.Decimal] = mapped_column(Numeric())
    x_axis_z: Mapped[decimal.Decimal] = mapped_column(Numeric())
    y_axis_x: Mapped[decimal.Decimal] = mapped_column(Numeric())
    y_axis_y: Mapped[decimal.Decimal] = mapped_column(Numeric())
    y_axis_z: Mapped[decimal.Decimal] = mapped_column(Numeric())
    type_id: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped["Type"] = relationship("Type", back_populates="ports")
    properties: Mapped[Optional[typing.List[Property]]] = relationship(
        Property, back_populates="port", cascade="all, delete-orphan"
    )
    attractings: Mapped[Optional[typing.List["Attraction"]]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracting_piece_type_port_id]",
        back_populates="attracting_piece_type_port",
    )
    attracteds: Mapped[Optional[typing.List["Attraction"]]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracted_piece_type_port_id]",
        back_populates="attracted_piece_type_port",
    )

    def __repr__(self) -> str:
        return f"Port(id={self.id!r}, name={self.name!r}, type_id={self.type_id!r})"

    @property
    def plane(self) -> Plane:
        return Plane(
            origin=Point(
                x=self.origin_x,
                y=self.origin_y,
                z=self.origin_z,
            ),
            x_axis=Vector(
                x=self.x_axis_x,
                y=self.x_axis_y,
                z=self.x_axis_z,
            ),
            y_axis=Vector(
                x=self.y_axis_x,
                y=self.y_axis_y,
                z=self.y_axis_z,
            ),
        )

    @plane.setter
    def plane(self, plane: Plane):
        self.origin_x = plane.origin.x
        self.origin_y = plane.origin.y
        self.origin_z = plane.origin.z
        self.x_axis_x = plane.x_axis.x
        self.x_axis_y = plane.x_axis.y
        self.x_axis_z = plane.x_axis.z
        self.y_axis_x = plane.y_axis.x
        self.y_axis_y = plane.y_axis.y
        self.y_axis_z = plane.y_axis.z

    @property
    def parent(self) -> Artifact:
        return self.type

    @property
    def children(self) -> typing.List[Artifact]:
        return self.properties

    @property
    def references(self) -> typing.List[Artifact]:
        return []

    @property
    def referenced_by(self) -> typing.List[Artifact]:
        return self.attractings + self.attracteds

    @property
    def related_to(self) -> typing.List[Artifact]:
        return [self.parent] + self.children + self.references + self.referenced_by


class Type(Base):
    __tablename__ = "type"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    symbol: Mapped[Optional[str]] = mapped_column(String(SYMBOL_LENGTH_MAX))
    kit_id: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="types")
    ports: Mapped[Optional[typing.List[Port]]] = relationship(
        "Port", back_populates="type"
    )
    properties: Mapped[Optional[typing.List[Property]]] = relationship(
        Property, back_populates="type", cascade="all, delete-orphan"
    )
    pieces: Mapped[Optional[typing.List["Piece"]]] = relationship(
        "Piece", back_populates="type"
    )

    def __repr__(self) -> str:
        return f"Type(id={self.id!r}, name={self.name!r}, kit_id={self.kit_id!r})"

    @property
    def parent(self) -> Artifact:
        return self.kit

    @property
    def children(self) -> typing.List[Artifact]:
        return self.ports + self.properties

    @property
    def references(self) -> typing.List[Artifact]:
        return []

    @property
    def referenced_by(self) -> typing.List[Artifact]:
        return []

    @property
    def related_to(self) -> typing.List[Artifact]:
        return [self.parent] + self.children + self.references + self.referenced_by


class TransientId(BaseModel):
    id: str


class Piece(Base):
    __tablename__ = "piece"

    id: Mapped[int] = mapped_column(primary_key=True)
    type_id: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped["Type"] = relationship("Type", back_populates="pieces")
    formation_id: Mapped[int] = mapped_column(ForeignKey("formation.id"))
    formation: Mapped["Formation"] = relationship("Formation", back_populates="pieces")
    attractings: Mapped[Optional[typing.List["Attraction"]]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracting_piece_id]",
        back_populates="attracting_piece",
    )
    attracteds: Mapped[Optional[typing.List["Attraction"]]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracted_piece_id]",
        back_populates="attracted_piece",
    )

    def __repr__(self) -> str:
        return f"Piece(id={self.id!r}, formation_id={self.formation_id!r})"

    @property
    def transient(self) -> TransientId:
        return TransientId(id=str(self.id))

    @property
    def parent(self) -> Artifact:
        return self.formation

    @property
    def children(self) -> typing.List[Artifact]:
        return []

    @property
    def references(self) -> typing.List[Artifact]:
        return self.type

    @property
    def referenced_by(self) -> typing.List[Artifact]:
        return self.attractings + self.attracteds

    @property
    def related_to(self) -> typing.List[Artifact]:
        return [self.parent] + self.children + self.references + self.referenced_by


class PortTypePieceSide(BaseModel):
    class Config:
        arbitrary_types_allowed = True

    properties: typing.List[Property]


class TypePieceSide(BaseModel):
    class Config:
        arbitrary_types_allowed = True

    port: PortTypePieceSide


class PieceSide(BaseModel):
    class Config:
        arbitrary_types_allowed = True

    transient: TransientId
    type: TypePieceSide


class Side(BaseModel):
    class Config:
        arbitrary_types_allowed = True

    piece: PieceSide


class Attraction(Base):
    __tablename__ = "attraction"

    attracting_piece_id: Mapped[int] = mapped_column(
        ForeignKey("piece.id"), primary_key=True
    )
    attracting_piece: Mapped[Piece] = relationship(
        Piece, foreign_keys=[attracting_piece_id], back_populates="attractings"
    )
    attracting_piece_type_port_id = mapped_column(
        ForeignKey("port.id"), primary_key=True
    )
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
    attracted_piece_type_port_id = mapped_column(
        ForeignKey("port.id"), primary_key=True
    )
    attracted_piece_type_port: Mapped[Port] = relationship(
        Port,
        foreign_keys=[attracted_piece_type_port_id],
        back_populates="attracteds",
    )
    formation_id: Mapped[int] = mapped_column(
        ForeignKey("formation.id"), primary_key=True
    )
    formation: Mapped["Formation"] = relationship(
        "Formation", back_populates="attractions"
    )

    __table_args__ = (
        CheckConstraint(
            "attracting_piece_id != attracted_piece_id",
            name="attracting_and_attracted_piece_not_equal_constraint",
        ),
    )

    def __repr__(self) -> str:
        return f"Attraction(attracting_piece_id={self.attracting_piece_id!r}, attracted_piece_id={self.attracted_piece_id!r}, formation_id={self.formation_id!r})"

    @property
    def attracting(self) -> Side:
        return Side(
            piece=PieceSide(
                transient=self.attracting_piece.transient,
                type=TypePieceSide(
                    port=PortTypePieceSide(
                        properties=self.attracting_piece_type_port.properties
                    )
                ),
            )
        )

    @property
    def attracted(self) -> Side:
        return Side(
            piece=PieceSide(
                transient=self.attracted_piece.transient,
                type=TypePieceSide(
                    port=PortTypePieceSide(
                        properties=self.attracted_piece_type_port.properties
                    )
                ),
            )
        )

    @property
    def parent(self) -> Artifact:
        return self.formation

    @property
    def children(self) -> typing.List[Artifact]:
        return []

    @property
    def references(self) -> typing.List[Artifact]:
        return [
            self.attracting_piece,
            self.attracted_piece,
            self.attracting_piece_type_port,
            self.attracted_piece_type_port,
        ]

    @property
    def referenced_by(self) -> typing.List[Artifact]:
        return []

    @property
    def related_to(self) -> typing.List[Artifact]:
        return [self.parent] + self.children + self.references + self.referenced_by


class Formation(Base):
    __tablename__ = "formation"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    symbol: Mapped[Optional[str]] = mapped_column(String(SYMBOL_LENGTH_MAX))
    pieces: Mapped[Optional[typing.List[Piece]]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    attractions: Mapped[Optional[typing.List[Attraction]]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    kit_id: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="formations")

    def __repr__(self) -> str:
        return (
            f"Formation(id={self.id!r}, name = {self.name!r}, kit_id={self.kit_id!r})"
        )

    @property
    def parent(self) -> Artifact:
        return self.kit

    @property
    def children(self) -> typing.List[Artifact]:
        return self.pieces + self.attractions

    @property
    def references(self) -> typing.List[Artifact]:
        return []

    @property
    def referenced_by(self) -> typing.List[Artifact]:
        return []

    @property
    def related_to(self) -> typing.List[Artifact]:
        return [self.parent] + self.children + self.references + self.referenced_by


class Kit(Base):
    __tablename__ = "kit"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    symbol: Mapped[Optional[str]] = mapped_column(String(SYMBOL_LENGTH_MAX))
    url: Mapped[Optional[str]] = mapped_column(String(URL_LENGTH_MAX))
    types: Mapped[Optional[typing.List[Type]]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )
    formations: Mapped[Optional[typing.List[Formation]]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )

    def __repr__(self) -> str:
        return f"Kit(id={self.id!r}, name={self.name!r})"

    @property
    def parent(self) -> Artifact:
        return None

    @property
    def children(self) -> typing.List[Artifact]:
        return self.scripts + self.types + self.formations

    @property
    def references(self) -> typing.List[Artifact]:
        return []

    @property
    def referenced_by(self) -> typing.List[Artifact]:
        return []

    @property
    def related_to(self) -> typing.List[Artifact]:
        return self.children + self.references + self.referenced_by


class DirectoryError(SemioException):
    def __init__(self, directory: String):
        self.directory = directory


class DirectoryDoesNotExist(DirectoryError):
    def __str__(self):
        return "Directory does not exist: " + self.directory


class DirectoryIsNotADirectory(DirectoryError):
    def __str__(self):
        return "Directory is not a directory: " + self.directory


def assertDirectory(directory: String) -> Path:
    directory = Path(directory)
    if not directory.exists():
        raise DirectoryDoesNotExist(directory)
    if not directory.is_dir():
        raise DirectoryIsNotADirectory(directory)
    return directory.resolve()


@lru_cache(maxsize=100)
def getLocalSession(directory: String) -> Session:
    directory = assertDirectory(directory)
    engine = create_engine("sqlite:///" + str(directory.joinpath(KIT_FILENAME)))
    Base.metadata.create_all(engine)
    # Create instance of session factory
    return sessionmaker(bind=engine)()


class ArtifactNode(graphene.Interface):
    name = NonNull(graphene.String)
    explanation = graphene.String()
    symbol = graphene.String()
    parent = graphene.Field(lambda: ArtifactNode)
    children = NonNull(graphene.List(NonNull(lambda: ArtifactNode)))
    references = NonNull(graphene.List(NonNull(lambda: ArtifactNode)))
    referenced_by = NonNull(graphene.List(NonNull(lambda: ArtifactNode)))
    related_to = NonNull(graphene.List(NonNull(lambda: ArtifactNode)))

    @staticmethod
    def resolve_parent(artifact: "ArtifactNode", info):
        return artifact.parent

    @staticmethod
    def resolve_children(artifact: "ArtifactNode", info):
        return artifact.children

    @staticmethod
    def resolve_references(artifact: "ArtifactNode", info):
        return artifact.references

    @staticmethod
    def resolve_referenced_by(artifact: "ArtifactNode", info):
        return artifact.referenced_by

    @staticmethod
    def resolve_related_to(artifact: "ArtifactNode", info):
        return artifact.related_to


class PropertyNode(SQLAlchemyInterface):
    class Meta:
        model = Property
        exclude_fields = (
            "id",
            "type_id",
            "type",
            "port_id",
            "port",
            "list_id",
            "list_index",
            "list",
        )

    datatype = NonNull(graphene.String)
    value = NonNull(graphene.String)

    @staticmethod
    def resolve_datatype(property: Property, info):
        return property.datatype.name.lower()

    @staticmethod
    def resolve_value(property: Property, info):
        return property.value


class DecimalPropertyNode(SQLAlchemyObjectType):
    class Meta:
        model = DecimalProperty
        exclude_fields = (
            "id",
            "type_id",
            "type",
            "port_id",
            "port",
            "list_id",
            "list_index",
            "list",
        )
        interfaces = (PropertyNode,)

    @staticmethod
    def resolve_decimal(property: DecimalProperty, info):
        return property.decimal

    @classmethod
    def is_type_of(cls, root, info):
        return isinstance(root, (cls, DecimalProperty))


class IntegerPropertyNode(SQLAlchemyObjectType):
    class Meta:
        model = IntegerProperty
        exclude_fields = (
            "id",
            "type_id",
            "type",
            "port_id",
            "port",
            "list_id",
            "list_index",
            "list",
        )
        interfaces = (PropertyNode,)

    @staticmethod
    def resolve_integer(property: IntegerProperty, info):
        return property.integer

    @classmethod
    def is_type_of(cls, root, info):
        return isinstance(root, (cls, IntegerProperty))


class NaturalPropertyNode(SQLAlchemyObjectType):
    class Meta:
        model = NaturalProperty
        exclude_fields = (
            "id",
            "type_id",
            "type",
            "port_id",
            "port",
            "list_id",
            "list_index",
            "list",
        )
        interfaces = (PropertyNode,)

    @staticmethod
    def resolve_natural(property: NaturalProperty, info):
        return property.natural

    @classmethod
    def is_type_of(cls, root, info):
        return isinstance(root, (cls, NaturalProperty))


class BooleanPropertyNode(SQLAlchemyObjectType):
    class Meta:
        model = BooleanProperty
        exclude_fields = (
            "id",
            "type_id",
            "type",
            "port_id",
            "port",
            "list_id",
            "list_index",
            "list",
        )
        interfaces = (PropertyNode,)

    @staticmethod
    def resolve_boolean(property: BooleanProperty, info):
        return property.boolean

    @classmethod
    def is_type_of(cls, root, info):
        return isinstance(root, (cls, BooleanProperty))


class DescriptionPropertyNode(SQLAlchemyObjectType):
    class Meta:
        model = DescriptionProperty
        exclude_fields = (
            "id",
            "type_id",
            "type",
            "port_id",
            "port",
            "list_id",
            "list_index",
            "list",
        )
        interfaces = (PropertyNode,)

    @staticmethod
    def resolve_description(property: DescriptionProperty, info):
        return property.description

    @classmethod
    def is_type_of(cls, root, info):
        return isinstance(root, (cls, DescriptionProperty))


class ChoicePropertyNode(SQLAlchemyObjectType):
    class Meta:
        model = ChoiceProperty
        exclude_fields = (
            "id",
            "type_id",
            "type",
            "port_id",
            "port",
            "list_id",
            "list_index",
            "list",
        )
        interfaces = (PropertyNode,)

    @staticmethod
    def resolve_choice(property: ChoiceProperty, info):
        return property.choice

    @classmethod
    def is_type_of(cls, root, info):
        return isinstance(root, (cls, ChoiceProperty))


class BrepPropertyNode(SQLAlchemyObjectType):
    class Meta:
        model = BrepProperty
        exclude_fields = (
            "id",
            "type_id",
            "type",
            "port_id",
            "port",
            "list_id",
            "list_index",
            "list",
        )
        interfaces = (PropertyNode,)

    @staticmethod
    def resolve_brep(property: BrepProperty, info):
        return property.brep

    @classmethod
    def is_type_of(cls, root, info):
        return isinstance(root, (cls, BrepProperty))


class ListPropertyNode(SQLAlchemyObjectType):
    class Meta:
        model = ListProperty
        exclude_fields = (
            "id",
            "type_id",
            "type",
            "port_id",
            "port",
            "list_id",
            "list_index",
            "list",
            "properties",
        )
        interfaces = (PropertyNode,)

    list = NonNull(graphene.List(lambda: PropertyNode))

    @staticmethod
    def resolve_list(property: ListProperty, info):
        return property.list

    @classmethod
    def is_type_of(cls, root, info):
        return isinstance(root, (cls, ListProperty))


class PointNode(PydanticObjectType):
    class Meta:
        model = Point


class VectorNode(PydanticObjectType):
    class Meta:
        model = Vector


class PlaneNode(PydanticObjectType):
    class Meta:
        model = Plane


class PortNode(SQLAlchemyObjectType):
    class Meta:
        model = Port
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
            "type_id",
        )

    plane = graphene.Field(PlaneNode)

    @staticmethod
    def resolve_plane(port: Port, info):
        return port.plane


class TypeNode(SQLAlchemyObjectType):
    class Meta:
        model = Type
        interfaces = (ArtifactNode,)
        exclude_fields = (
            "id",
            "kit_id",
        )


class TransientIdNode(PydanticObjectType):
    class Meta:
        model = TransientId


class PieceNode(SQLAlchemyObjectType):
    class Meta:
        model = Piece
        exclude_fields = ("id", "type_id", "formation_id")

    transient = graphene.Field(NonNull(TransientIdNode))

    @staticmethod
    def resolve_transient(piece: Piece, info):
        return piece.transient


class PortTypePieceSideNode(PydanticObjectType):
    class Meta:
        model = PortTypePieceSide
        exclude_fields = ("properties",)

    properties = graphene.List(PropertyNode)

    @staticmethod
    def resolve_properties(root, info):
        return root.properties


class TypePieceSideNode(PydanticObjectType):
    class Meta:
        model = TypePieceSide
        exclude_fields = ("port",)

    port = graphene.Field(PortTypePieceSideNode)

    @staticmethod
    def resolve_port(root, info):
        return root.port


class PieceSideNode(PydanticObjectType):
    class Meta:
        model = PieceSide
        exclude_fields = ("type",)

    type = graphene.Field(TypePieceSideNode)

    @staticmethod
    def resolve_type(root, info):
        return root.type


class SideNode(PydanticObjectType):
    class Meta:
        model = Side
        exclude_fields = ("piece",)

    piece = graphene.Field(PieceSideNode)

    @staticmethod
    def resolve_piece(root, info):
        return root.piece


class AttractionNode(SQLAlchemyObjectType):
    class Meta:
        model = Attraction
        exclude_fields = (
            "attracting_piece_id",
            "attracting_piece",
            "attracting_piece_type_port_id",
            "attracted_piece_id",
            "attracted_piece",
            "attracted_piece_type_port_id",
            "formation_id",
        )

    attracting = graphene.Field(NonNull(SideNode))
    attracted = graphene.Field(NonNull(SideNode))

    @staticmethod
    def resolve_attracting(attraction: Attraction, info):
        return attraction.attracting

    @staticmethod
    def resolve_attracted(attraction: Attraction, info):
        return attraction.attracted


class FormationNode(SQLAlchemyObjectType):
    class Meta:
        model = Formation
        interfaces = (ArtifactNode,)
        exclude_fields = (
            "id",
            "kit_id",
        )


class KitNode(SQLAlchemyObjectType):
    class Meta:
        model = Kit
        interfaces = (ArtifactNode,)
        exclude_fields = ("id",)


class DecimalPropertyInput(InputObjectType):
    name = NonNull(graphene.String)
    decimal = NonNull(graphene.Decimal)


class IntegerPropertyInput(InputObjectType):
    name = NonNull(graphene.String)
    integer = NonNull(graphene.Int)


class NaturalPropertyInput(InputObjectType):
    name = NonNull(graphene.String)
    natural = NonNull(graphene.Int)


class BooleanPropertyInput(InputObjectType):
    name = NonNull(graphene.String)
    boolean = NonNull(graphene.Boolean)


class DescriptionPropertyInput(InputObjectType):
    name = NonNull(graphene.String)
    description = NonNull(graphene.String)


class ChoicePropertyInput(InputObjectType):
    name = NonNull(graphene.String)
    choice = NonNull(graphene.String)


class BrepPropertyInput(InputObjectType):
    name = NonNull(graphene.String)
    brep = NonNull(graphene.JSONString)


class ListPropertyInput(InputObjectType):
    name = NonNull(graphene.String)
    list = graphene.List(lambda: ListPropertyInput)


class PropertyInput(InputObjectType):
    name = NonNull(graphene.String)
    # oneOf
    decimal = graphene.Decimal()
    integer = graphene.Int()
    natural = graphene.Int()
    boolean = graphene.Boolean()
    description = graphene.String()
    choice = graphene.String()
    brep = graphene.JSONString()
    list = graphene.List(lambda: PropertyInput)


class PointInput(PydanticInputObjectType):
    class Meta:
        model = Point


class VectorInput(PydanticInputObjectType):
    class Meta:
        model = Vector


class PlaneInput(PydanticInputObjectType):
    class Meta:
        model = Plane


class PortInput(InputObjectType):
    plane = NonNull(PlaneInput)
    properties = NonNull(graphene.List(NonNull(PropertyInput)))


class PortIdInput(InputObjectType):
    properties = NonNull(graphene.List(NonNull(PropertyInput)))


class TypeInput(InputObjectType):
    name = NonNull(graphene.String)
    explanation = graphene.String()
    symbol = graphene.String()
    ports = NonNull(graphene.List(NonNull(PortInput)))
    properties = graphene.List(PropertyInput)


class TypeIdInput(InputObjectType):
    name = NonNull(graphene.String)


class TransientIdInput(InputObjectType):
    id = NonNull(graphene.String)


class PieceInput(InputObjectType):
    transient = NonNull(TransientIdInput)
    type = NonNull(TypeIdInput)


class TypePieceSideInput(InputObjectType):
    port = NonNull(PortIdInput)


class PieceSideInput(InputObjectType):
    transient = NonNull(TransientIdInput)
    type = NonNull(TypePieceSideInput)


class SideInput(InputObjectType):
    piece = NonNull(PieceSideInput)


class AttractionInput(InputObjectType):
    attracting = NonNull(SideInput)
    attracted = NonNull(SideInput)


class FormationInput(InputObjectType):
    name = NonNull(graphene.String)
    explanation = graphene.String()
    symbol = graphene.String()
    pieces = NonNull(graphene.List(NonNull(PieceInput)))
    attractions = NonNull(graphene.List(NonNull(AttractionInput)))


class KitInput(InputObjectType):
    name = NonNull(graphene.String)
    explanation = graphene.String()
    symbol = graphene.String()
    url = graphene.String()
    types = graphene.List(TypeInput)
    formations = graphene.List(FormationInput)


class SpecificationError(SemioException):
    pass


class NotFound(SpecificationError):
    def __init__(self, id) -> None:
        self.id = id

    def __str__(self):
        return f"{self.id} not found."


class ArtifactNotFound(NotFound):
    def __init__(self, name) -> None:
        super().__init__(name)
        self.name = name

    def __str__(self):
        return f"Artifact({self.name}) not found."


class PortNotFound(NotFound):
    def __init__(self, properties) -> None:
        super().__init__(properties)
        self.properties = properties

    def __str__(self):
        return f"Port({self.properties}) not found."


class TypeNotFound(NotFound):
    def __init__(self, name) -> None:
        super().__init__(name)
        self.name = name

    def __str__(self):
        return f"Type({self.name}) not found."


class PieceNotFound(NotFound):
    def __init__(self, transient_id) -> None:
        super().__init__(transient_id)
        self.transient_id = transient_id

    def __str__(self):
        return f"Piece({self.transient_id}) not found. Please check that the transient id is correct and that the piece is part of the formation."


class FormationNotFound(NotFound):
    def __init__(self, name) -> None:
        super().__init__(name)
        self.name = name

    def __str__(self):
        return f"Formation({self.name}) not found."


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


class AlreadyExists(SemioException):
    def __init__(self, id, existing) -> None:
        self.id = id
        self.existing = existing

    def __str__(self):
        return f"{self.id} already exists: {str(self.existing)}"


class ArtifactAlreadyExists(AlreadyExists):
    def __init__(self, artifact) -> None:
        super().__init__(artifact.name, artifact)
        self.artifact = artifact

    def __str__(self):
        return f"Artifact ({self.artifact.name}) already exists: {str(self.artifact)}"


class TypeAlreadyExists(AlreadyExists):
    def __init__(self, type) -> None:
        super().__init__(type.name, type)
        self.type = type

    def __str__(self):
        return f"Type ({self.type.name}) already exists: {str(self.type)}"


class FormationAlreadyExists(AlreadyExists):
    def __init__(self, formation) -> None:
        super().__init__(formation.name, formation)
        self.formation = formation

    def __str__(self):
        return (
            f"Formation ({self.formation.name}) already exists: {str(self.formation)}"
        )


class KitAlreadyExists(AlreadyExists):
    def __init__(self, kit) -> None:
        super().__init__(kit.name, kit)
        self.kit = kit

    def __str__(self):
        return f"Kit ({self.kit.name}) already exists: {str(self.kit)}"


def propertyInputToTransientPropertyForEquality(
    propertyInput: PropertyInput,
) -> Property:
    if propertyInput.decimal is not None:
        return DecimalProperty(
            name=propertyInput.name,
            decimal=propertyInput.decimal,
        )
    elif propertyInput.integer is not None:
        return IntegerProperty(
            name=propertyInput.name,
            integer=propertyInput.integer,
        )
    elif propertyInput.natural is not None:
        return NaturalProperty(
            name=propertyInput.name,
            natural=propertyInput.natural,
        )
    elif propertyInput.boolean is not None:
        return BooleanProperty(
            name=propertyInput.name,
            boolean=propertyInput.boolean,
        )
    elif propertyInput.description is not None:
        return DescriptionProperty(
            name=propertyInput.name,
            description=propertyInput.description,
        )
    elif propertyInput.choice is not None:
        return ChoiceProperty(
            name=propertyInput.name,
            choice=propertyInput.choice,
        )
    elif propertyInput.brep is not None:
        return BrepProperty(
            name=propertyInput.name,
            brep=propertyInput.brep,
        )
    elif propertyInput.list is not None:
        return ListProperty(
            name=propertyInput.name,
        )
    else:
        raise InvalidBackend("Unknown property type")


def getMainKit(session: Session) -> Kit:
    kit = session.query(Kit).first()
    if not kit:
        raise NoMainKit()
    return kit


def getTypeByName(session: Session, name: String) -> Type:
    type = session.query(Type).filter_by(name=name).first()
    if type is None:
        raise TypeNotFound(name)
    return type


def getPortByProperties(
    session: Session, type: Type, propertyInputs: typing.List[PropertyInput]
) -> Port:
    typePorts = session.query(Port).filter_by(type_id=type.id)
    port = None
    for typePort in typePorts:
        if all(
            [
                propertyInputToTransientPropertyForEquality(property)
                in typePort.properties
                for property in propertyInputs
            ]
        ):
            port = typePort
            break
    if port is None:
        raise PortNotFound(propertyInputs)
    return port


def getFormationByName(session: Session, name: String) -> Formation:
    formation = session.query(Formation).filter_by(name=name).first()
    if formation is None:
        raise FormationNotFound(name)
    return formation


def addPropertyInputToSession(
    session: Session,
    owner: Type | Port | ListProperty,
    propertyInput: PropertyInput,
    listIndex=None,
) -> Property:
    if isinstance(owner, Type):
        type_id = owner.id
        port_id = None
        list_id = None
    elif isinstance(owner, Port):
        type_id = None
        port_id = owner.id
        list_id = None
    elif isinstance(owner, ListProperty):
        type_id = None
        port_id = None
        list_id = owner.id
    else:
        raise InvalidBackend(f"Unknown owner type: {type(owner)}")
    if propertyInput.decimal is not None:
        property = DecimalProperty(
            name=propertyInput.name,
            decimal=propertyInput.decimal,
            type_id=type_id,
            port_id=port_id,
            list_id=list_id,
            list_index=listIndex,
        )
        session.add(property)
        session.flush()
    elif propertyInput.integer is not None:
        property = IntegerProperty(
            name=propertyInput.name,
            integer=propertyInput.integer,
            type_id=type_id,
            port_id=port_id,
            list_id=list_id,
            list_index=listIndex,
        )
        session.add(property)
        session.flush()
    elif propertyInput.natural is not None:
        property = NaturalProperty(
            name=propertyInput.name,
            natural=propertyInput.natural,
            type_id=type_id,
            port_id=port_id,
            list_id=list_id,
            list_index=listIndex,
        )
        session.add(property)
        session.flush()
    elif propertyInput.boolean is not None:
        property = BooleanProperty(
            name=propertyInput.name,
            boolean=propertyInput.boolean,
            type_id=type_id,
            port_id=port_id,
            list_id=list_id,
            list_index=listIndex,
        )
        session.add(property)
        session.flush()
    elif propertyInput.description is not None:
        property = DescriptionProperty(
            name=propertyInput.name,
            description=propertyInput.description,
            type_id=type_id,
            port_id=port_id,
            list_id=list_id,
            list_index=listIndex,
        )
        session.add(property)
        session.flush()
    elif propertyInput.choice is not None:
        property = ChoiceProperty(
            name=propertyInput.name,
            choice=propertyInput.choice,
            type_id=type_id,
            port_id=port_id,
            list_id=list_id,
            list_index=listIndex,
        )
        session.add(property)
        session.flush()
    elif propertyInput.brep is not None:
        property = BrepProperty(
            name=propertyInput.name,
            brep=propertyInput.brep,
            type_id=type_id,
            port_id=port_id,
            list_id=list_id,
            list_index=listIndex,
        )
        session.add(property)
        session.flush()
    elif propertyInput.list is not None:
        property = ListProperty(
            name=propertyInput.name,
            type_id=type_id,
            port_id=port_id,
            list_id=list_id,
            list_index=listIndex,
        )
        session.add(property)
        session.flush()
        for i, childPropertyInput in enumerate(propertyInput.list) or []:
            addPropertyInputToSession(session, property, childPropertyInput, i)
    else:
        raise InvalidBackend("Unknown property type")

    return property


def addPortInputToSession(session: Session, type: Type, portInput: PortInput) -> Port:
    port = Port(
        origin_x=portInput.plane.origin.x,
        origin_y=portInput.plane.origin.y,
        origin_z=portInput.plane.origin.z,
        x_axis_x=portInput.plane.x_axis.x,
        x_axis_y=portInput.plane.x_axis.y,
        x_axis_z=portInput.plane.x_axis.z,
        y_axis_x=portInput.plane.y_axis.x,
        y_axis_y=portInput.plane.y_axis.y,
        y_axis_z=portInput.plane.y_axis.z,
        type_id=type.id,
    )
    session.add(port)
    session.flush()
    for propertyInput in portInput.properties or []:
        property = addPropertyInputToSession(session, port, propertyInput)
    return port


def addTypeInputToSession(
    session: Session, kit: Kit, typeInput: TypeInput, replace: bool = False
) -> Type:
    type = session.query(Type).filter_by(name=typeInput.name).first()
    if type:
        if replace:
            session.delete(type)
        else:
            raise TypeAlreadyExists(type)
    if not type:
        type = Type(name=typeInput.name, kit_id=kit.id)
        session.add(type)
        session.flush()
    try:
        type.explanation = typeInput.explanation
    except AttributeError:
        pass
    try:
        type.symbol = typeInput.symbol
    except AttributeError:
        pass
    for portInput in typeInput.ports or []:
        port = addPortInputToSession(session, type, portInput)
    for propertyInput in typeInput.properties or []:
        property = addPropertyInputToSession(session, type, propertyInput)
    return type


def addPieceInputToSession(
    session: Session, formation: Formation, pieceInput: PieceInput
) -> Piece:
    type = getTypeByName(session, pieceInput.type.name)
    piece = Piece(type_id=type.id, formation_id=formation.id)
    session.add(piece)
    session.flush()
    return piece


def addAttractionInputToSession(
    session: Session,
    formation: Formation,
    attractionInput: AttractionInput,
    transientIdToPiece: dict,
) -> Attraction:
    try:
        attractingPiece = transientIdToPiece[
            attractionInput.attracting.piece.transient.id
        ]
    except KeyError:
        raise PieceNotFound(attractionInput.attracting.piece.transient.id)
    try:
        attractedPiece = transientIdToPiece[
            attractionInput.attracted.piece.transient.id
        ]
    except KeyError:
        raise PieceNotFound(attractionInput.attracted.piece.transient.id)
    attractingPieceTypePort = getPortByProperties(
        session,
        getTypeByName(session, attractingPiece.type.name),
        attractionInput.attracting.piece.type.port.properties,
    )
    attractedPieceTypePort = getPortByProperties(
        session,
        getTypeByName(session, attractedPiece.type.name),
        attractionInput.attracted.piece.type.port.properties,
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
    session: Session, kit: Kit, formationInput: FormationInput, replace: bool = False
):
    formation = session.query(Formation).filter_by(name=formationInput.name).first()
    if formation:
        if replace:
            session.delete(formation)
            formation = None
        else:
            raise FormationAlreadyExists(formation)
    if not formation:
        formation = Formation(
            name=formationInput.name,
            kit_id=kit.id,
        )
        session.add(formation)
        session.flush()
    try:
        formation.explanation = formationInput.explanation
    except AttributeError:
        pass
    try:
        formation.symbol = formationInput.symbol
    except AttributeError:
        symbol = None
    transientIdToPiece: Dict[str, Piece] = {}
    for pieceInput in formationInput.pieces or []:
        piece = addPieceInputToSession(session, formation, pieceInput)
        transientIdToPiece[pieceInput.transient.id] = piece
    for attractionInput in formationInput.attractions or []:
        attraction = addAttractionInputToSession(
            session, formation, attractionInput, transientIdToPiece
        )
    return formation


def addKitInputToSession(session: Session, kitInput: KitInput, replace: bool = False):
    try:
        kit = getMainKit(session)
    except NoMainKit:
        kit = Kit(
            name=kitInput.name,
        )
        session.add(kit)
        session.flush()
    try:
        kit.explanation = kitInput.explanation
    except AttributeError:
        pass
    try:
        kit.symbol = kitInput.symbol
    except AttributeError:
        pass
    for typeInput in kitInput.types or []:
        type = addTypeInputToSession(session, kit, typeInput, replace)
    for formationInput in kitInput.formations or []:
        formation = addFormationInputToSession(session, kit, formationInput, replace)
    return kit


class CreateLocalKitErrorCode(graphene.Enum):
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_ALREADY_CONTAINS_A_KIT = "directory_already_contains_a_kit"
    NO_PERMISSION_TO_CREATE_DIRECTORY = "no_permission_to_create_directory"
    NO_PERMISSION_TO_CREATE_KIT = "no_permission_to_create_kit"
    KIT_INPUT_IS_INVALID = "kit_input_is_invalid"


class CreateLocalKitErrorNode(ObjectType):
    code = NonNull(CreateLocalKitErrorCode)
    message = graphene.String()


disposed_engines = {}


class CreateLocalKitMutation(graphene.Mutation):
    class Arguments:
        directory = NonNull(graphene.String)
        kitInput = NonNull(KitInput)

    kit = Field(KitNode)
    error = Field(CreateLocalKitErrorNode)

    def mutate(self, info, directory, kitInput):
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

        kitFile = directory.joinpath(KIT_FILENAME)
        if kitFile.exists():
            return CreateLocalKitMutation(
                error=CreateLocalKitErrorNode(
                    code=CreateLocalKitErrorCode.DIRECTORY_ALREADY_CONTAINS_A_KIT
                )
            )

        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in disposed_engines:
            raise Exception(
                "Can't create a new kit in a directory where this process already deleted an engine. Restart the server and try again."
            )

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
        return CreateLocalKitMutation(kit=kit, error=None)


class UpdateMode(graphene.Enum):
    REPLACE = "replace"
    APPEND = "append"


class UpdateLocalKitErrorCode(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_UPDATE_KIT = "no_permission_to_update_kit"
    ARTIFACT_ALREADY_EXISTS = "artifact_already_exists"
    KIT_INPUT_IS_INVALID = "kit_input_is_invalid"


class UpdateLocalKitErrorNode(ObjectType):
    code = NonNull(UpdateLocalKitErrorCode)
    message = graphene.String()


class UpdateLocalKitMutation(graphene.Mutation):
    class Arguments:
        directory = NonNull(graphene.String)
        kitInput = NonNull(KitInput)
        mode = NonNull(UpdateMode, default_value=UpdateMode.REPLACE)

    kit = Field(KitNode)
    error = Field(UpdateLocalKitErrorNode)

    def mutate(self, info, directory, kitInput, mode):
        directory = Path(directory)
        if not directory.exists():
            return UpdateLocalKitMutation(
                error=UpdateLocalKitErrorNode(
                    code=UpdateLocalKitErrorCode.DIRECTORY_DOES_NOT_EXIST
                )
            )
        if not directory.is_dir():
            return UpdateLocalKitMutation(
                error=UpdateLocalKitErrorNode(
                    code=UpdateLocalKitErrorCode.DIRECTORY_IS_NOT_A_DIRECTORY
                )
            )
        kitFile = directory.joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return UpdateLocalKitMutation(
                error=UpdateLocalKitErrorNode(
                    code=UpdateLocalKitErrorCode.DIRECTORY_HAS_NO_KIT
                )
            )
        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in disposed_engines:
            raise Exception(
                "Can't update a kit in a directory where this process already deleted an engine. Restart the server and try again."
            )
        session = getLocalSession(directory)
        try:
            kit = addKitInputToSession(session, kitInput, mode == UpdateMode.REPLACE)
        except SpecificationError as e:
            session.rollback()
            return UpdateLocalKitMutation(
                error=UpdateLocalKitErrorNode(
                    code=UpdateLocalKitErrorCode.KIT_INPUT_IS_INVALID, message=str(e)
                )
            )
        except ArtifactAlreadyExists as e:
            session.rollback()
            return UpdateLocalKitMutation(
                error=UpdateLocalKitErrorNode(
                    code=UpdateLocalKitErrorCode.ARTIFACT_ALREADY_EXISTS,
                    message=str(e),
                )
            )
        session.commit()
        return UpdateLocalKitMutation(kit=kit, error=None)


class DeleteLocalKitError(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_DELETE_KIT = "no_permission_to_delete_kit"


class DeleteLocalKitMutation(graphene.Mutation):
    class Arguments:
        directory = NonNull(graphene.String)

    error = DeleteLocalKitError()

    def mutate(self, info, directory):
        directory = Path(directory)
        if not directory.exists():
            return DeleteLocalKitError(
                error=DeleteLocalKitError.DIRECTORY_DOES_NOT_EXIST
            )
        kitFile = directory.joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return DeleteLocalKitError(error=DeleteLocalKitError.DIRECTORY_HAS_NO_KIT)
        kitFileFullPath = kitFile.resolve()
        disposed_engines[kitFileFullPath] = True
        try:
            remove(kitFileFullPath)
        except PermissionError:
            return DeleteLocalKitError(
                error=DeleteLocalKitError.NO_PERMISSION_TO_DELETE_KIT
            )
        return DeleteLocalKitError()


class ReadLocalKitError(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_READ_KIT = "no_permission_to_read_kit"


class ReadLocalKitResponse(ObjectType):
    kit = Field(KitNode)
    error = Field(ReadLocalKitError)


class Query(ObjectType):
    localTypes = graphene.List(TypeNode, directory=NonNull(graphene.String))
    localFormations = graphene.List(FormationNode, directory=NonNull(graphene.String))
    localArtifacts = graphene.List(ArtifactNode, directory=NonNull(graphene.String))
    localKit = graphene.Field(ReadLocalKitResponse, directory=NonNull(graphene.String))

    def resolve_localTypes(self, info, directory: graphene.String):
        session = getLocalSession(directory)
        return session.query(Type).all()

    def resolve_localFormations(self, info, directory: graphene.String):
        session = getLocalSession(directory)
        return session.query(Formation).all()

    def resolve_localArtifacts(self, info, directory: graphene.String):
        session = getLocalSession(directory)
        artifacts = []
        artifacts.extend(session.query(Type).all())
        artifacts.extend(session.query(Formation).all())
        artifacts.extend(session.query(Kit).all())
        return artifacts

    def resolve_localKit(self, info, directory: graphene.String):
        directory = Path(directory)
        if not directory.exists():
            return ReadLocalKitResponse(
                kit=None,
                error=ReadLocalKitError(
                    code=ReadLocalKitError.DIRECTORY_DOES_NOT_EXIST
                ),
            )
        if not directory.is_dir():
            return ReadLocalKitResponse(
                kit=None,
                error=ReadLocalKitError(
                    code=ReadLocalKitError.DIRECTORY_IS_NOT_A_DIRECTORY
                ),
            )
        try:
            session = getLocalSession(directory)
        except PermissionError:
            return ReadLocalKitResponse(
                kit=None,
                error=ReadLocalKitError(
                    code=ReadLocalKitError.NO_PERMISSION_TO_READ_KIT
                ),
            )
        try:
            kit = getMainKit(session)
        except NoMainKit:
            return ReadLocalKitResponse(
                kit=None,
                error=ReadLocalKitError(code=ReadLocalKitError.DIRECTORY_HAS_NO_KIT),
            )
        return ReadLocalKitResponse(kit=kit, error=None)


class Mutation(ObjectType):
    createLocalKit = CreateLocalKitMutation.Field()
    updateLocalKit = UpdateLocalKitMutation.Field()
    deleteLocalKit = DeleteLocalKitMutation.Field()


schema = Schema(
    query=Query,
    mutation=Mutation,
    types=[
        DecimalPropertyNode,
        IntegerPropertyNode,
        NaturalPropertyNode,
        BooleanPropertyNode,
        DescriptionPropertyNode,
        ChoicePropertyNode,
        BrepPropertyNode,
        ListPropertyNode,
    ],
)
with open("schema.graphql", "w") as f:
    f.write(str(schema))

app = Flask(__name__)
app.add_url_rule(
    "/graphql",
    view_func=GraphQLView.as_view(
        "graphql",
        schema=schema,
        graphiql=True,
    ),
)


def main():
    app.run()


if __name__ == "__main__":
    main()
