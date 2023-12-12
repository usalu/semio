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
Graphql API for semio.
"""

from typing import Optional, List
from enum import Enum
from decimal import Decimal
from dataclasses import dataclass, field
from argparse import ArgumentParser
import sqlalchemy
from sqlalchemy import String, Text, Numeric, ForeignKey, create_engine
from sqlalchemy.orm import (
    DeclarativeBase,
    Mapped,
    mapped_column,
    relationship,
    scoped_session,
    sessionmaker,
    QueryPropertyDescriptor,
)
import graphene
from graphene import Schema
from graphene.relay import Node
from graphene_sqlalchemy import (
    SQLAlchemyObjectType,
    SQLAlchemyConnectionField,
    SQLAlchemyInterface,
)
from flask import Flask
from graphql_server.flask import GraphQLView

NAME_LENGTH_MAX = 100
PROPERTY_DATATYPE_LENGTH_MAX = 100
SCRIPT_URI_LENGTH_MAX = 1000
SCRIPT_KIND_LENGTH_MAX = 100

# engine = create_engine("sqlite:///:memory:", echo=True)
engine = create_engine("sqlite:///semio.db", echo=True)
Session = scoped_session(sessionmaker(autocommit=False, autoflush=False, bind=engine))
# Session = sessionmaker(autocommit=False, autoflush=False, bind=engine)

# SQLAlchemy


class Base(DeclarativeBase):
    # Query: QueryPropertyDescriptor = Session.query_property()
    pass


class ScriptKindEnum(Enum):
    PROTOTYPE = "prototype"
    MODIFICATION = "modification"
    CHOREOGRAPHY = "choreography"
    TRANSFORMATION = "transformation"


class Script(Base):
    __tablename__ = "script"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    uri: Mapped[str] = mapped_column(String(SCRIPT_URI_LENGTH_MAX))
    kind: Mapped[ScriptKindEnum] = mapped_column(sqlalchemy.Enum(ScriptKindEnum))
    kit_id: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="scripts")


class Property(Base):
    __tablename__ = "property"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    datatype: Mapped[str] = mapped_column(String(PROPERTY_DATATYPE_LENGTH_MAX))
    value: Mapped[str] = mapped_column(Text())
    type_id: Mapped[Optional[int]] = mapped_column(ForeignKey("type.id"))
    type: Mapped[Optional["Type"]] = relationship("Type", back_populates="properties")
    port_id: Mapped[Optional[int]] = mapped_column(ForeignKey("port.id"))
    port: Mapped[Optional["Port"]] = relationship("Port", back_populates="properties")


class Port(Base):
    __tablename__ = "port"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    origin_x: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    origin_y: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    origin_z: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    x_axis_x: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    x_axis_y: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    x_axis_z: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    y_axis_x: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    y_axis_y: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    y_axis_z: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    z_axis_x: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    z_axis_y: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    z_axis_z: Mapped[Optional[Decimal]] = mapped_column(Numeric())
    type_id: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped[Optional["Type"]] = relationship("Type", back_populates="ports")
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
    properties: Mapped[List[Property]] = relationship(
        "Property", back_populates="port", cascade="all, delete-orphan"
    )


class Type(Base):
    __tablename__ = "type"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    ports: Mapped[List[Port]] = relationship("Port", back_populates="type")
    kit_id: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="types")
    properties: Mapped[List[Property]] = relationship(
        "Property", back_populates="type", cascade="all, delete-orphan"
    )

    def __repr__(self) -> str:
        return f"Type(id={self.id!r}, name={self.name!r})"


class Piece(Base):
    __tablename__ = "piece"

    id: Mapped[int] = mapped_column(primary_key=True)
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
    formation_id: Mapped[int] = mapped_column(ForeignKey("formation.id"))
    formation: Mapped["Formation"] = relationship("Formation", back_populates="pieces")

    def __repr__(self) -> str:
        return f"Piece(id={self.id!r}, formation_id={self.formation_id!r})"


class Attraction(Base):
    __tablename__ = "attraction"

    attracting_piece_id: Mapped[int] = mapped_column(
        ForeignKey("piece.id"), primary_key=True
    )
    attracting_piece: Mapped[Piece] = relationship(
        "Piece", foreign_keys=[attracting_piece_id], back_populates="attractings"
    )
    attracting_piece_type_port_id = mapped_column(
        ForeignKey("port.id"), primary_key=True
    )
    attracting_piece_type_port: Mapped[Port] = relationship(
        "Port",
        foreign_keys=[attracting_piece_type_port_id],
        back_populates="attractings",
    )
    attracted_piece_id: Mapped[int] = mapped_column(
        ForeignKey("piece.id"), primary_key=True
    )
    attracted_piece: Mapped[Piece] = relationship(
        "Piece", foreign_keys=[attracted_piece_id], back_populates="attracteds"
    )
    attracted_piece_type_port_id = mapped_column(
        ForeignKey("port.id"), primary_key=True
    )
    attracted_piece_type_port: Mapped[Port] = relationship(
        "Port",
        foreign_keys=[attracted_piece_type_port_id],
        back_populates="attracteds",
    )
    formation_id: Mapped[int] = mapped_column(
        ForeignKey("formation.id"), primary_key=True
    )
    formation: Mapped["Formation"] = relationship(
        "Formation", back_populates="attractions"
    )

    def __repr__(self) -> str:
        return f"Piece(id={self.id!r}, formation_id={self.formation_id!r})"


class Formation(Base):
    __tablename__ = "formation"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    pieces: Mapped[List[Piece]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    attractions: Mapped[List[Attraction]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    kit_id: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="formations")

    def __repr__(self) -> str:
        return f"Formation(id={self.id!r}, name={self.name!r})"


class Kit(Base):
    __tablename__ = "kit"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    scripts: Mapped[List[Script]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )
    types: Mapped[List[Type]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )
    formations: Mapped[List[Formation]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )


# Graphene


class ScriptNode(SQLAlchemyObjectType):
    class Meta:
        model = Script


class PropertyNode(SQLAlchemyObjectType):
    class Meta:
        model = Property


class PortNode(SQLAlchemyObjectType):
    class Meta:
        model = Port


class PieceNode(SQLAlchemyObjectType):
    class Meta:
        model = Piece


class AttractionNode(SQLAlchemyObjectType):
    class Meta:
        model = Attraction


class FormationNode(SQLAlchemyObjectType):
    class Meta:
        model = Formation


class TypeNode(SQLAlchemyObjectType):
    class Meta:
        model = Type


class KitNode(SQLAlchemyObjectType):
    class Meta:
        model = Kit


class Query(graphene.ObjectType):
    kits = graphene.List(KitNode)
    types = graphene.List(TypeNode)

    def resolve_kits(self, info):
        query = KitNode.get_query(info)
        return query.all()

    def resolve_types(self, info):
        query = TypeNode.get_query(info)
        return query.all()


schema = Schema(query=Query)

app = Flask(__name__)
app.add_url_rule(
    "/graphql",
    view_func=GraphQLView.as_view(
        "graphql",
        schema=schema,
        graphiql=True,
        get_context=lambda: {"session": Session},
    ),
)


@app.teardown_appcontext
def shutdown_session(exception=None):
    Session.remove()


def initialize_database():
    Base.metadata.create_all(engine)
    metabolism = Kit(
        name="metabolism", explanation="An archive for metabolistic architecture."
    )
    Session.add(metabolism)
    quadraticcapsuleshaft = Type(
        name="quadraticcapsuleshaft",
        explanation="A quadratic shaft with a central development (elevator at the core and stairs around it) to hold capsules.",
        kit=metabolism,
    )
    Session.add(quadraticcapsuleshaft)
    quadraticcapsuleshaft_length = Property(
        name="length", datatype="decimal", value="6", type=quadraticcapsuleshaft
    )
    Session.add(quadraticcapsuleshaft_length)
    Session.commit()


def main():
    initialize_database()
    app.run()


if __name__ == "__main__":
    main()
    # parser = ArgumentParser(description="Process some integers.")
    # parser.add_argument("--argument", metavar="N", type=str)
    # args = parser.parse_args()
    # main.run(args)
    # engine = create_engine("sqlite:///metabolism.db", echo=True)
    # Base.metadata.create_all(engine)
