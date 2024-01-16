#!/usr/bin/env python

# semio
# Copyright (C) 2023 Ueli Saluz

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
API for semio.
"""

# TODO: Check if sqlmodel can replace SQLAlchemy:
#       ✅Constraints
#       ❔Polymorphism
#       ❔graphene_sqlalchemy
# TODO: Uniformize naming.
# TODO: Check graphene_pydantic until the pull request for pydantic>2 is merged.
# TODO: Check if @staticmethod can be removed in graphene resolvers because implicit.


from os import remove
from pathlib import Path
from functools import lru_cache
from typing import Optional, Dict, Protocol, List, Union
from decimal import Decimal
from datetime import datetime
from urllib.parse import urlparse
from pint import UnitRegistry
from pydantic import BaseModel
from sqlalchemy import (
    String,
    Text,
    Numeric,
    DateTime,
    ForeignKey,
    create_engine,
    CheckConstraint,
    and_,
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
import graphene
from graphene import Schema, Mutation, ObjectType, InputObjectType, Field, NonNull
from graphene_sqlalchemy import (
    SQLAlchemyObjectType,
)
from graphene_pydantic import PydanticObjectType, PydanticInputObjectType
from flask import Flask
from graphql_server.flask import GraphQLView

NAME_LENGTH_MAX = 100
URL_LENGTH_MAX = 1000
KIT_FILENAME = "kit.semio"

ureg = UnitRegistry()


def canonicalize_name(name: str) -> str:
    return name.strip().lower().replace(" ", "_")


def canonicalize_number(number: Decimal) -> Decimal:
    return number.quantize(Decimal("0.000001"))


class SemioException(Exception):
    pass


class SpecificationError(SemioException):
    pass


class InvalidURL(ValueError, SpecificationError):
    def __init__(self, url: str) -> None:
        self.url = url

    def __str__(self) -> str:
        return f"{self.url} is not a valid URL."


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


class Artifact(Protocol):
    @property
    def parent(self) -> Union["Artifact", None]:
        return None

    @property
    def children(self) -> List["Artifact"]:
        return []

    @property
    def references(self) -> List["Artifact"]:
        return []

    @property
    def referenced_by(self) -> List["Artifact"]:
        return []

    @property
    def related_to(self) -> List["Artifact"]:
        return (
            ([self.parent] if self.parent else [])
            + self.children
            + self.references
            + self.referenced_by
        )


# TODO: Refactor Protocol to ABC and make it work with SQLAlchemy
class Document(Artifact):
    name: str
    explanation: str
    icon: str


class Point(BaseModel):
    x: Decimal
    y: Decimal
    z: Decimal


class Vector(BaseModel):
    x: Decimal
    y: Decimal
    z: Decimal


class Plane(BaseModel):
    origin: Point
    x_axis: Vector
    y_axis: Vector


class Base(DeclarativeBase):
    pass


class Tag(Base):
    __tablename__ = "tag"

    value: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX), primary_key=True)
    representation_id: Mapped[int] = mapped_column(ForeignKey("representation.id"))
    representation: Mapped["Representation"] = relationship(
        "Representation", back_populates="_tags"
    )

    def __repr__(self) -> str:
        return (
            f"Tag(value={self.value!r}, representation_id={self.representation_id!r})"
        )

    def __str__(self) -> str:
        return (
            f"Tag(value={self.value}, representation_id={str(self.representation_id)})"
        )

    @property
    def parent(self) -> Artifact:
        return self.representation

    @property
    def children(self) -> List[Artifact]:
        return []

    @property
    def references(self) -> List[Artifact]:
        return []

    @property
    def referenced_by(self) -> List[Artifact]:
        return []

    @property
    def related_to(self) -> List[Artifact]:
        return [self.parent]


class Representation(Base):
    __tablename__ = "representation"
    id: Mapped[int] = mapped_column(primary_key=True)
    url: Mapped[str] = mapped_column(String(URL_LENGTH_MAX))
    # level of detail
    lod: Mapped[Optional[str]] = mapped_column(String(NAME_LENGTH_MAX))
    type_id: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped["Type"] = relationship("Type", back_populates="representations")
    _tags: Mapped[List[Tag]] = relationship(
        Tag, back_populates="representation", cascade="all, delete-orphan"
    )

    def __repr__(self) -> str:
        return f"Representation(id={self.id!r}, url={self.url!r}, lod={self.lod!r}, type_id={self.type_id!r}, tags={self.tags!r})"

    def __str__(self) -> str:
        return f"Representation(id={str(self.id)}, type_id={str(self.type_id)})"

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

    @property
    def parent(self) -> Artifact:
        return self.type

    @property
    def children(self) -> List[Artifact]:
        return self._tags  # type: ignore

    @property
    def references(self) -> List[Artifact]:
        return []

    @property
    def referenced_by(self) -> List[Artifact]:
        return []

    @property
    def related_to(self) -> List[Artifact]:
        return [self.parent] + self.children if self.children else []


class Specifier(Base):
    __tablename__ = "specifier"

    context: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX), primary_key=True)
    group: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    port_id: Mapped[int] = mapped_column(ForeignKey("port.id"), primary_key=True)
    port: Mapped["Port"] = relationship("Port", back_populates="specifiers")

    def __repr__(self) -> str:
        return f"Specifier(context={self.context!r}, group={self.group!r}, port_id={self.port_id!r})"

    def __str__(self) -> str:
        return f"Specifier(context={self.context}, port_id={str(self.port_id)})"

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Specifier):
            return NotImplemented
        return self.context == other.context and self.group == other.group

    def __hash__(self) -> int:
        return hash((self.context, self.group))

    @property
    def parent(self) -> Artifact:
        return self.port

    @property
    def children(self) -> List[Artifact]:
        return []

    @property
    def references(self) -> List[Artifact]:
        return []

    @property
    def referenced_by(self) -> List[Artifact]:
        return []

    @property
    def related_to(self) -> List[Artifact]:
        return [self.parent]


class Port(Base):
    __tablename__ = "port"

    id: Mapped[int] = mapped_column(primary_key=True)
    origin_x: Mapped[Decimal] = mapped_column(Numeric())
    origin_y: Mapped[Decimal] = mapped_column(Numeric())
    origin_z: Mapped[Decimal] = mapped_column(Numeric())
    x_axis_x: Mapped[Decimal] = mapped_column(Numeric())
    x_axis_y: Mapped[Decimal] = mapped_column(Numeric())
    x_axis_z: Mapped[Decimal] = mapped_column(Numeric())
    y_axis_x: Mapped[Decimal] = mapped_column(Numeric())
    y_axis_y: Mapped[Decimal] = mapped_column(Numeric())
    y_axis_z: Mapped[Decimal] = mapped_column(Numeric())
    type_id: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped["Type"] = relationship("Type", back_populates="ports")
    specifiers: Mapped[List[Specifier]] = relationship(
        Specifier, back_populates="port", cascade="all, delete-orphan"
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

    def __repr__(self) -> str:
        return f"Port(id={self.id!r}, origin_x={self.origin_x!r}, origin_y={self.origin_y!r}, origin_z={self.origin_z!r}, x_axis_x={self.x_axis_x!r}, x_axis_y={self.x_axis_y!r}, x_axis_z={self.x_axis_z!r}, y_axis_x={self.y_axis_x!r}, y_axis_y={self.y_axis_y!r}, y_axis_z={self.y_axis_z!r}, type_id={self.type_id!r}, specifiers={self.specifiers!r}, attractings={self.attractings!r}, attracteds={self.attracteds!r})"

    def __str__(self) -> str:
        return f"Port(id={str(self.id)}, type_id={str(self.type_id)})"

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
    def children(self) -> List[Artifact]:
        return self.specifiers  # type: ignore

    @property
    def references(self) -> List[Artifact]:
        return []

    @property
    def referenced_by(self) -> List[Artifact]:
        return self.attractings + self.attracteds  # type: ignore

    @property
    def related_to(self) -> List[Artifact]:
        return [self.parent] + self.children + self.referenced_by


class Quality(Base):
    __tablename__ = "quality"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    value: Mapped[str] = mapped_column(Text())
    unit: Mapped[Optional[str]] = mapped_column(String(NAME_LENGTH_MAX))
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
    )

    def __eq__(self, other: object) -> bool:
        if not isinstance(other, Quality):
            return NotImplemented
        if self.name == other.name:
            if self.unit == other.unit:
                return self.value == other.value
            # TODO: use pint to compare values with different units
            raise NotImplementedError(
                "Comparing values with different units is not implemented yet."
            )

        return False

    def __hash__(self) -> int:
        # TODO: Implement unit normalization for consistent hashing
        return hash((self.name, self.value, self.unit))

    def __repr__(self) -> str:
        return f"Quality(name={self.name!r}, value={self.value!r}, unit={self.unit!r}, type_id={self.type_id!r}, formation_id={self.formation_id!r})"

    def __str__(self) -> str:
        return f"Quality(name={self.name}, type_id={str(self.type_id)}, formation_id={str(self.formation_id)})"

    @property
    def parent(self) -> Artifact:
        return self.type if self.type_id else self.formation

    @property
    def children(self) -> List[Artifact]:
        return []

    @property
    def references(self) -> List[Artifact]:
        return []

    @property
    def referenced_by(self) -> List[Artifact]:
        return []

    @property
    def related_to(self) -> List[Artifact]:
        return [self.parent]


class Type(Base):
    __tablename__ = "type"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    icon: Mapped[Optional[str]] = mapped_column(Text())
    created_at: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.utcnow, nullable=False
    )
    modified_at: Mapped[datetime] = mapped_column(
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

    def __repr__(self) -> str:
        return f"Type(id={self.id!r}, name={self.name!r}, explanation={self.explanation!r}, icon={self.icon!r}, kit_id={self.kit_id!r}, representations={self.representations!r}, ports={self.ports!r}, qualities={self.qualities!r}, pieces={self.pieces!r})"

    def __str__(self) -> str:
        return f"Type(id={str(self.id)}, kit_id={str(self.kit_id)})"

    @property
    def parent(self) -> Artifact:
        return self.kit

    @property
    def children(self) -> List[Artifact]:
        return self.representations + self.ports + self.qualities  # type: ignore

    @property
    def references(self) -> List[Artifact]:
        return []

    @property
    def referenced_by(self) -> List[Artifact]:
        return [self.pieces]  # type: ignore

    @property
    def related_to(self) -> List[Artifact]:
        return [self.parent] + self.children + self.referenced_by


class Transient(BaseModel):
    id: str


class Piece(Base):
    __tablename__ = "piece"

    id: Mapped[int] = mapped_column(primary_key=True)
    type_id: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped["Type"] = relationship("Type", back_populates="pieces")
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

    def __repr__(self) -> str:
        return f"Piece(id={self.id!r}, type_id={self.type_id!r}, formation_id={self.formation_id!r}, attractings={self.attractings!r}, attracteds={self.attracteds!r})"

    def __str__(self) -> str:
        return f"Piece(id={str(self.id)}, formation_id={str(self.formation_id)})"

    @property
    def transient(self) -> Transient:
        return Transient(id=str(self.id))

    @property
    def parent(self) -> Artifact:
        return self.formation

    @property
    def children(self) -> List[Artifact]:
        return []

    @property
    def references(self) -> List[Artifact]:
        return self.type  # type: ignore

    @property
    def referenced_by(self) -> List[Artifact]:
        return self.attractings + self.attracteds  # type: ignore

    @property
    def related_to(self) -> List[Artifact]:
        return [self.parent] + self.references + self.referenced_by


class TypePieceSide(BaseModel):
    class Config:
        arbitrary_types_allowed = True

    port: Port


class PieceSide(BaseModel):
    class Config:
        arbitrary_types_allowed = True

    transient: Transient
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
        return f"Attraction(attracting_piece_id={self.attracting_piece_id!r}, attracting_piece_type_port_id={self.attracting_piece_type_port_id!r}, attracted_piece_id={self.attracted_piece_id!r}, attracted_piece_type_port_id={self.attracted_piece_type_port_id!r}, formation_id={self.formation_id!r})"

    def __str__(self) -> str:
        return f"Attraction(attracting_piece_id={str(self.attracting_piece_id)}, attracted_piece_id={str(self.attracted_piece_id)}, formation_id={str(self.formation_id)})"

    @property
    def attracting(self) -> Side:
        return Side(
            piece=PieceSide(
                transient=self.attracting_piece.transient,
                type=TypePieceSide(
                    port=self.attracting_piece_type_port,
                ),
            )
        )

    @property
    def attracted(self) -> Side:
        return Side(
            piece=PieceSide(
                transient=self.attracted_piece.transient,
                type=TypePieceSide(
                    port=self.attracted_piece_type_port,
                ),
            )
        )

    @property
    def parent(self) -> Artifact:
        return self.formation

    @property
    def children(self) -> List[Artifact]:
        return []

    @property
    def references(self) -> List[Artifact]:
        return [
            self.attracting_piece,
            self.attracted_piece,
            self.attracting_piece_type_port,
            self.attracted_piece_type_port,
        ]

    @property
    def referenced_by(self) -> List[Artifact]:
        return []

    @property
    def related_to(self) -> List[Artifact]:
        return [self.parent] + self.references


class Formation(Base):
    __tablename__ = "formation"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    icon: Mapped[Optional[str]] = mapped_column(Text())
    created_at: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.utcnow, nullable=False
    )
    modified_at: Mapped[datetime] = mapped_column(
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

    def __repr__(self) -> str:
        return f"Formation(id={self.id!r}, name={self.name!r}, explanation={self.explanation!r}, icon={self.icon!r}, kit_id={self.kit_id!r}, pieces={self.pieces!r}, attractions={self.attractions!r}, qualities={self.qualities!r})"

    def __str__(self) -> str:
        return f"Formation(id={str(self.id)}, kit_id={str(self.kit_id)})"

    @property
    def parent(self) -> Artifact:
        return self.kit

    @property
    def children(self) -> List[Artifact]:
        return self.pieces + self.attractions  # type: ignore

    @property
    def references(self) -> List[Artifact]:
        return []

    @property
    def referenced_by(self) -> List[Artifact]:
        return []

    @property
    def related_to(self) -> List[Artifact]:
        return [self.parent] + self.children


class Kit(Base):
    __tablename__ = "kit"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    icon: Mapped[Optional[str]] = mapped_column(Text())
    created_at: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.utcnow, nullable=False
    )
    modified_at: Mapped[datetime] = mapped_column(
        DateTime(), default=datetime.utcnow, nullable=False, onupdate=datetime.utcnow
    )
    url: Mapped[Optional[str]] = mapped_column(String(URL_LENGTH_MAX))
    types: Mapped[List[Type]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )
    formations: Mapped[List[Formation]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )

    def __repr__(self) -> str:
        return f"Kit(id={self.id!r}, name={self.name!r}), explanation={self.explanation!r}, icon={self.icon!r}, url={self.url!r}, types={self.types!r}, formations={self.formations!r})"

    def __str__(self) -> str:
        return f"Kit(id={str(self.id)})"

    @property
    def parent(self) -> None:
        return None

    @property
    def children(self) -> List[Artifact]:
        return self.types + self.formations  # type: ignore

    @property
    def references(self) -> List[Artifact]:
        return []

    @property
    def referenced_by(self) -> List[Artifact]:
        return []

    @property
    def related_to(self) -> List[Artifact]:
        return self.children


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
    engine = create_engine("sqlite:///" + str(directory_path.joinpath(KIT_FILENAME)))
    Base.metadata.create_all(engine)
    # Create instance of session factory
    return sessionmaker(bind=engine)()


class ArtifactNode(graphene.Interface):
    class Meta:
        name = "Artifact"

    name = NonNull(graphene.String)
    explanation = graphene.String()
    icon = graphene.String()
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


class RepresentationNode(SQLAlchemyObjectType):
    class Meta:
        model = Representation
        name = "Representation"
        exclude_fields = (
            "id",
            "_tags",
            "type_id",
        )

    tags = graphene.List(graphene.String)

    @staticmethod
    def resolve_tags(representation: Representation, info):
        return representation.tags


class PointNode(PydanticObjectType):
    class Meta:
        model = Point
        name = "Point"


class VectorNode(PydanticObjectType):
    class Meta:
        model = Vector
        name = "Vector"


class SpecifierNode(SQLAlchemyObjectType):
    class Meta:
        model = Specifier
        name = "Specifier"
        exclude_fields = ("port_id",)


class PlaneNode(PydanticObjectType):
    class Meta:
        model = Plane
        name = "Plane"


class PortNode(SQLAlchemyObjectType):
    class Meta:
        model = Port
        name = "Port"
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


class QualityNode(SQLAlchemyObjectType):
    class Meta:
        model = Quality
        name = "Quality"
        exclude_fields = ("type_id", "formation_id")


class TypeNode(SQLAlchemyObjectType):
    class Meta:
        model = Type
        name = "Type"
        interfaces = (ArtifactNode,)
        exclude_fields = (
            "id",
            "kit_id",
        )


class TransientNode(PydanticObjectType):
    class Meta:
        model = Transient
        name = "Transient"


class PieceNode(SQLAlchemyObjectType):
    class Meta:
        model = Piece
        name = "Piece"
        exclude_fields = ("id", "type_id", "formation_id")

    transient = graphene.Field(NonNull(TransientNode))

    @staticmethod
    def resolve_transient(piece: Piece, info):
        return piece.transient


class TypePieceSideNode(PydanticObjectType):
    class Meta:
        model = TypePieceSide
        name = "TypePieceSide"
        exclude_fields = ("port",)

    port = graphene.Field(PortNode)

    @staticmethod
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

    @staticmethod
    def resolve_attracting(attraction: Attraction, info):
        return attraction.attracting

    @staticmethod
    def resolve_attracted(attraction: Attraction, info):
        return attraction.attracted


class FormationNode(SQLAlchemyObjectType):
    class Meta:
        model = Formation
        name = "Formation"
        interfaces = (ArtifactNode,)
        exclude_fields = (
            "id",
            "kit_id",
        )


class KitNode(SQLAlchemyObjectType):
    class Meta:
        model = Kit
        name = "Kit"
        interfaces = (ArtifactNode,)
        exclude_fields = ("id",)


class RepresentationInput(InputObjectType):
    url = NonNull(graphene.String)
    lod = graphene.String()
    tags = NonNull(graphene.List(NonNull(graphene.String)))


class SpecifierInput(InputObjectType):
    context = NonNull(graphene.String)
    group = NonNull(graphene.String)


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
    specifiers = NonNull(graphene.List(NonNull(SpecifierInput)))


class PortIdInput(InputObjectType):
    specifiers = NonNull(graphene.List(NonNull(SpecifierInput)))


class QualityInput(InputObjectType):
    name = NonNull(graphene.String)
    value = NonNull(graphene.String)
    unit = graphene.String()


class TypeInput(InputObjectType):
    name = NonNull(graphene.String)
    explanation = graphene.String()
    icon = graphene.String()
    representations = NonNull(graphene.List(NonNull(RepresentationInput)))
    ports = NonNull(graphene.List(NonNull(PortInput)))
    qualities = graphene.List(QualityInput)


class TypeIdInput(InputObjectType):
    name = NonNull(graphene.String)
    qualities = graphene.List(QualityInput)


class TransientInput(InputObjectType):
    id = NonNull(graphene.String)


class PieceInput(InputObjectType):
    transient = NonNull(TransientInput)
    type = NonNull(TypeIdInput)


class TypePieceSideInput(InputObjectType):
    port = NonNull(PortIdInput)


class PieceSideInput(InputObjectType):
    transient = NonNull(TransientInput)
    type = NonNull(TypePieceSideInput)


class SideInput(InputObjectType):
    piece = NonNull(PieceSideInput)


class AttractionInput(InputObjectType):
    attracting = NonNull(SideInput)
    attracted = NonNull(SideInput)


class FormationInput(InputObjectType):
    name = NonNull(graphene.String)
    explanation = graphene.String()
    icon = graphene.String()
    pieces = NonNull(graphene.List(NonNull(PieceInput)))
    attractions = NonNull(graphene.List(NonNull(AttractionInput)))
    qualities = graphene.List(QualityInput)


class KitInput(InputObjectType):
    name = NonNull(graphene.String)
    explanation = graphene.String()
    icon = graphene.String()
    url = graphene.String()
    types = graphene.List(TypeInput)
    formations = graphene.List(FormationInput)


class NotFound(SpecificationError):
    def __init__(self, id) -> None:
        self.id = id

    def __str__(self):
        return f"{self.id} not found."


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
        return f"Qualities ({self.qualityInputs}) don't match any type with name {self.name}: {str(self.types)}"


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


def getMainKit(session: Session) -> Kit:
    kit = session.query(Kit).first()
    if not kit:
        raise NoMainKit()
    return kit


def qualityInputToTransientQualityForEquality(qualityInput: QualityInput) -> Quality:
    return Quality(
        name=qualityInput.name,
        value=qualityInput.value,
        unit=qualityInput.unit,
    )


def specifierInputToTransientSpecifierForEquality(
    specifierInput: SpecifierInput,
) -> Specifier:
    return Specifier(
        context=specifierInput.context,
        group=specifierInput.group,
    )


def getTypeByNameAndQualities(
    session: Session, name: String, qualityInputs: List[QualityInput]
) -> Type:
    typesWithSameName = session.query(Type).filter_by(name=name)
    if typesWithSameName.count() < 1:
        raise TypeNotFound(name)
    typesWithSameName = typesWithSameName.all()
    qualities = [
        qualityInputToTransientQualityForEquality(qualityInput)
        for qualityInput in qualityInputs
    ]
    typesWithSameQualities = [
        type for type in typesWithSameName if set(type.qualities) == set(qualities)
    ]
    if len(typesWithSameQualities) != 1:
        raise QualitiesDontMatchType(name, qualityInputs, typesWithSameQualities)
    return typesWithSameQualities[0]


def getPortBySpecifiers(
    session: Session, type: Type, specifierInputs: List[SpecifierInput]
) -> Port:
    ports = session.query(Port).filter_by(type_id=type.id)
    specifiers = [
        specifierInputToTransientSpecifierForEquality(specifierInput)
        for specifierInput in specifierInputs
    ]
    portsWithSameSpecifier = [
        port for port in ports if set(port.specifiers) == set(specifiers)
    ]
    if len(portsWithSameSpecifier) != 1:
        raise PortNotFound(type, specifierInputs)
    return portsWithSameSpecifier[0]


def addRepresentationInputToSession(
    session: Session, type: Type, representationInput: RepresentationInput
) -> Representation:
    representation = Representation(
        name=representationInput.name,
        format=representationInput.format,
        blob=representationInput.blob,
        type_id=type.id,
    )
    try:
        representation.explanation = representationInput.explanation
    except AttributeError:
        pass
    try:
        representation.icon = representationInput.icon
    except AttributeError:
        pass
    try:
        representation.lod = representationInput.lod
    except AttributeError:
        pass
    session.add(representation)
    session.flush()
    for tagInput in representationInput.tags or []:
        tag = Tag(
            value=tagInput.value,
            representation_id=representation.id,
        )
        session.add(tag)
        session.flush()
    return representation


def addSpecifierInputToSession(
    session: Session, port: Port, specifierInput: SpecifierInput
) -> Specifier:
    specifier = Specifier(
        context=specifierInput.context,
        group=specifierInput.group,
        port_id=port.id,
    )
    session.add(specifier)
    session.flush()
    return specifier


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
    for specifierInput in portInput.specifiers or []:
        specifier = addSpecifierInputToSession(session, port, specifierInput)
    return port


def addQualityInputToSession(
    session: Session,
    owner: Type | Formation,
    qualityInput: QualityInput,
) -> Quality:
    typeId = owner.id if isinstance(owner, Type) else None
    formationId = owner.id if isinstance(owner, Formation) else None
    quality = Quality(
        name=qualityInput.name,
        value=qualityInput.value,
        type_id=typeId,
        formation_id=formationId,
    )
    try:
        quality.unit = qualityInput.unit
    except AttributeError:
        pass
    session.add(quality)
    session.flush()
    return quality


def addTypeInputToSession(
    session: Session, kit: Kit, typeInput: TypeInput, replace: bool = False
) -> Type:
    try:
        type = getTypeByNameAndQualities(session, typeInput.name, typeInput.qualities)
    except TypeNotFound:
        type = None
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
        type.icon = typeInput.icon
    except AttributeError:
        pass
    session.flush()
    for portInput in typeInput.ports or []:
        port = addPortInputToSession(session, type, portInput)
    for qualityInput in typeInput.qualities or []:
        quality = addQualityInputToSession(session, type, qualityInput)
    return type


def addPieceInputToSession(
    session: Session, formation: Formation, pieceInput: PieceInput
) -> Piece:
    type = getTypeByNameAndQualities(
        session, pieceInput.type.name, pieceInput.type.qualities
    )
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
    attractingPieceTypePort = getPortBySpecifiers(
        session,
        attractingPiece.type,
        attractionInput.attracting.piece.type.port.specifiers,
    )
    attractedPieceTypePort = getPortBySpecifiers(
        session,
        attractedPiece.type,
        attractionInput.attracted.piece.type.port.specifiers,
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
        formation.icon = formationInput.icon
    except AttributeError:
        icon = None
    transientIdToPiece: Dict[str, Piece] = {}
    for pieceInput in formationInput.pieces or []:
        piece = addPieceInputToSession(session, formation, pieceInput)
        transientIdToPiece[pieceInput.transient.id] = piece

    for attractionInput in formationInput.attractions or []:
        attraction = addAttractionInputToSession(
            session, formation, attractionInput, transientIdToPiece
        )
    for qualityInput in formationInput.qualities or []:
        quality = addQualityInputToSession(session, formation, qualityInput)
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
        kit.icon = kitInput.icon
    except AttributeError:
        pass
    try:
        kit.url = kitInput.url
    except AttributeError:
        pass
    session.flush()
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


class LoadLocalKitError(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_READ_KIT = "no_permission_to_read_kit"


class LoadLocalTypesResponse(ObjectType):
    types = graphene.List(TypeNode)
    error = Field(LoadLocalKitError)


class LoadLocalFormationsResponse(ObjectType):
    formations = graphene.List(FormationNode)
    error = Field(LoadLocalKitError)


class LoadLocalArtifactsResponse(ObjectType):
    artifacts = graphene.List(ArtifactNode)
    error = Field(LoadLocalKitError)


class LoadLocalKitResponse(ObjectType):
    kit = Field(KitNode)
    error = Field(LoadLocalKitError)


class Query(ObjectType):
    loadLocalTypes = graphene.Field(
        LoadLocalTypesResponse, directory=NonNull(graphene.String)
    )
    loadLocalFormations = graphene.Field(
        LoadLocalFormationsResponse, directory=NonNull(graphene.String)
    )
    loadLocalArtifacts = graphene.Field(
        LoadLocalArtifactsResponse, directory=NonNull(graphene.String)
    )
    loadLocalKit = graphene.Field(
        LoadLocalKitResponse, directory=NonNull(graphene.String)
    )

    def resolve_loadLocalTypes(self, info, directory: graphene.String):
        session = getLocalSession(directory)
        return session.query(Type).all()

    def resolve_loadLocalFormations(self, info, directory: graphene.String):
        session = getLocalSession(directory)
        return session.query(Formation).all()

    def resolve_loadLocalArtifacts(self, info, directory: graphene.String):
        session = getLocalSession(directory)
        artifacts = []
        artifacts.extend(session.query(Type).all())
        artifacts.extend(session.query(Formation).all())
        artifacts.extend(session.query(Kit).all())
        return artifacts

    def resolve_loadLocalKit(self, info, directory: graphene.String):
        directory = Path(directory)
        if not directory.exists():
            return LoadLocalKitResponse(
                kit=None,
                error=LoadLocalKitError(
                    code=LoadLocalKitError.DIRECTORY_DOES_NOT_EXIST
                ),
            )
        if not directory.is_dir():
            return LoadLocalKitResponse(
                kit=None,
                error=LoadLocalKitError(
                    code=LoadLocalKitError.DIRECTORY_IS_NOT_A_DIRECTORY
                ),
            )
        try:
            session = getLocalSession(directory)
        except PermissionError:
            return LoadLocalKitResponse(
                kit=None,
                error=LoadLocalKitError(
                    code=LoadLocalKitError.NO_PERMISSION_TO_READ_KIT
                ),
            )
        try:
            kit = getMainKit(session)
        except NoMainKit:
            return LoadLocalKitResponse(
                kit=None,
                error=LoadLocalKitError(code=LoadLocalKitError.DIRECTORY_HAS_NO_KIT),
            )
        return LoadLocalKitResponse(kit=kit, error=None)


class Mutation(ObjectType):
    createLocalKit = CreateLocalKitMutation.Field()
    updateLocalKit = UpdateLocalKitMutation.Field()
    deleteLocalKit = DeleteLocalKitMutation.Field()


schema = Schema(
    query=Query,
    mutation=Mutation,
)

with open("schema.graphql", "w") as f:
    f.write(str(schema))

engine = create_engine("sqlite:///semio.db")
Base.metadata.create_all(engine)

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
