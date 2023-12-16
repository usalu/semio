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

from os import remove
from os.path import exists
from pathlib import Path
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
from graphene import Schema, Mutation, ObjectType, InputObjectType, Field
from graphene.relay import Node
from graphene_sqlalchemy import (
    SQLAlchemyObjectType,
    SQLAlchemyConnectionField,
    SQLAlchemyInterface,
)
from flask import Flask
from graphql_server.flask import GraphQLView

URI_LENGTH_MAX = 1000
NAME_LENGTH_MAX = 100
SYMBOL_LENGTH_MAX = 1
PROPERTY_DATATYPE_LENGTH_MAX = 100
SCRIPT_URI_LENGTH_MAX = 1000
SCRIPT_KIND_LENGTH_MAX = 100
KIT_FILENAME = "kit.semio"

# SQLAlchemy


class Base(DeclarativeBase):
    # Query: QueryPropertyDescriptor = Session.query_property()
    pass


class ScriptKind(Enum):
    SYNTHESIS = "synthesis"
    PROTOTYPE = "prototype"
    MODIFICATION = "modification"
    CHOREOGRAPHY = "choreography"
    TRANSFORMATION = "transformation"


class Script(Base):
    __tablename__ = "script"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    symbol: Mapped[Optional[str]] = mapped_column(String(SYMBOL_LENGTH_MAX))
    kind: Mapped[ScriptKind] = mapped_column(sqlalchemy.Enum(ScriptKind))
    url: Mapped[str] = mapped_column(String(SCRIPT_URI_LENGTH_MAX))
    kit_id: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="scripts")
    synthesized_properties: Mapped[Optional[List["Property"]]] = relationship(
        "Property",
        foreign_keys="[Property.synthesis_script_id]",
        back_populates="synthesis_script",
    )
    prototyped_types: Mapped[Optional[List["Type"]]] = relationship(
        "Type",
        foreign_keys="[Type.prototype_script_id]",
        back_populates="prototype_script",
    )
    choreographed_formations: Mapped[Optional[List["Formation"]]] = relationship(
        "Formation",
        foreign_keys="[Formation.choreography_script_id]",
        back_populates="choreography_script",
    )
    transformed_formations: Mapped[Optional[List["Formation"]]] = relationship(
        "Formation",
        foreign_keys="[Formation.transformation_script_id]",
        back_populates="transformation_script",
    )

    def __repr__(self) -> str:
        return f"Script(id={self.id!r}, name={self.name!r}, uri={self.uri!r}, kind={self.kind!r}, kit_id={self.kit_id!r})"


class Property(Base):
    __tablename__ = "property"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    datatype: Mapped[str] = mapped_column(String(PROPERTY_DATATYPE_LENGTH_MAX))
    value: Mapped[str] = mapped_column(Text())
    synthesis_script_id: Mapped[Optional[int]] = mapped_column(ForeignKey("script.id"))
    synthesis_script: Mapped[Optional[Script]] = relationship(
        Script, foreign_keys=[synthesis_script_id]
    )
    type_id: Mapped[Optional[int]] = mapped_column(ForeignKey("type.id"))
    type: Mapped[Optional["Type"]] = relationship("Type", back_populates="properties")
    port_id: Mapped[Optional[int]] = mapped_column(ForeignKey("port.id"))
    port: Mapped[Optional["Port"]] = relationship("Port", back_populates="properties")

    def __repr__(self) -> str:
        if self.type_id is not None:
            return f"Property(id={self.id!r}, name={self.name!r}, datatype={self.datatype!r}, type_id={self.type_id!r})"
        return f"Property(id={self.id!r}, name={self.name!r}, datatype={self.datatype!r}, port_id={self.port_id!r})"


class Port(Base):
    __tablename__ = "port"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    symbol: Mapped[Optional[str]] = mapped_column(String(SYMBOL_LENGTH_MAX))
    origin_x: Mapped[Decimal] = mapped_column(Numeric())
    origin_y: Mapped[Decimal] = mapped_column(Numeric())
    origin_z: Mapped[Decimal] = mapped_column(Numeric())
    x_axis_x: Mapped[Decimal] = mapped_column(Numeric())
    x_axis_y: Mapped[Decimal] = mapped_column(Numeric())
    x_axis_z: Mapped[Decimal] = mapped_column(Numeric())
    y_axis_x: Mapped[Decimal] = mapped_column(Numeric())
    y_axis_y: Mapped[Decimal] = mapped_column(Numeric())
    y_axis_z: Mapped[Decimal] = mapped_column(Numeric())
    z_axis_x: Mapped[Decimal] = mapped_column(Numeric())
    z_axis_y: Mapped[Decimal] = mapped_column(Numeric())
    z_axis_z: Mapped[Decimal] = mapped_column(Numeric())
    type_id: Mapped[int] = mapped_column(ForeignKey("type.id"))
    type: Mapped["Type"] = relationship("Type", back_populates="ports")
    properties: Mapped[Optional[List[Property]]] = relationship(
        Property, back_populates="port", cascade="all, delete-orphan"
    )
    attractings: Mapped[Optional[List["Attraction"]]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracting_piece_type_port_id]",
        back_populates="attracting_piece_type_port",
    )
    attracteds: Mapped[Optional[List["Attraction"]]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracted_piece_type_port_id]",
        back_populates="attracted_piece_type_port",
    )

    def __repr__(self) -> str:
        return f"Port(id={self.id!r}, name={self.name!r}, type_id={self.type_id!r})"


class Type(Base):
    __tablename__ = "type"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    symbol: Mapped[Optional[str]] = mapped_column(String(SYMBOL_LENGTH_MAX))
    kit_id: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="types")
    prototype_script_id: Mapped[Optional[int]] = mapped_column(ForeignKey("script.id"))
    prototype_script: Mapped[Optional[Script]] = relationship(
        Script, foreign_keys=[prototype_script_id]
    )
    ports: Mapped[Optional[List[Port]]] = relationship("Port", back_populates="type")
    properties: Mapped[Optional[List[Property]]] = relationship(
        Property, back_populates="type", cascade="all, delete-orphan"
    )

    def __repr__(self) -> str:
        return f"Type(id={self.id!r}, name={self.name!r})"


class Piece(Base):
    __tablename__ = "piece"

    id: Mapped[int] = mapped_column(primary_key=True)
    formation_id: Mapped[int] = mapped_column(ForeignKey("formation.id"))
    formation: Mapped["Formation"] = relationship("Formation", back_populates="pieces")
    attractings: Mapped[Optional[List["Attraction"]]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracting_piece_id]",
        back_populates="attracting_piece",
    )
    attracteds: Mapped[Optional[List["Attraction"]]] = relationship(
        "Attraction",
        foreign_keys="[Attraction.attracted_piece_id]",
        back_populates="attracted_piece",
    )

    def __repr__(self) -> str:
        return f"Piece(id={self.id!r}, formation_id={self.formation_id!r})"


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

    def __repr__(self) -> str:
        return f"Attraction(attracting_piece_id={self.attracting_piece_id!r}, attracted_piece_id={self.attracted_piece_id!r}, formation_id={self.formation_id!r})"


class Formation(Base):
    __tablename__ = "formation"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    symbol: Mapped[Optional[str]] = mapped_column(String(SYMBOL_LENGTH_MAX))
    pieces: Mapped[List[Piece]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    attractions: Mapped[List[Attraction]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    choreography_script_id: Mapped[Optional[int]] = mapped_column(
        ForeignKey("script.id")
    )
    choreography_script: Mapped[Optional[Script]] = relationship(
        Script, foreign_keys=[choreography_script_id]
    )
    transformation_script_id: Mapped[Optional[int]] = mapped_column(
        ForeignKey("script.id")
    )
    transformation_script: Mapped[Optional[Script]] = relationship(
        Script, foreign_keys=[transformation_script_id]
    )
    kit_id: Mapped[int] = mapped_column(ForeignKey("kit.id"))
    kit: Mapped["Kit"] = relationship("Kit", back_populates="formations")

    def __repr__(self) -> str:
        return (
            f"Formation(id={self.id!r}, name = {self.name!r}, kit_id={self.kit_id!r})"
        )


class Kit(Base):
    __tablename__ = "kit"

    id: Mapped[int] = mapped_column(primary_key=True)
    uri: Mapped[str] = mapped_column(String(URI_LENGTH_MAX))
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[str] = mapped_column(Text())
    symbol: Mapped[Optional[str]] = mapped_column(String(SYMBOL_LENGTH_MAX))
    scripts: Mapped[Optional[List[Script]]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )
    types: Mapped[Optional[List[Type]]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )
    formations: Mapped[Optional[List[Formation]]] = relationship(
        back_populates="kit", cascade="all, delete-orphan"
    )

    def __repr__(self) -> str:
        return f"Kit(id={self.id!r}, name={self.name!r})"


class DirectoryException(Exception):
    def __init__(self, directory):
        self.directory = directory


class DirectoryDoesNotExistException(DirectoryException):
    def __str__(self):
        return "Directory does not exist: " + self.directory


class DirectoryIsNotADirectoryException(DirectoryException):
    def __str__(self):
        return "Directory is not a directory: " + self.directory


def sanitizeAndCheckAndAbsolutizeDirectory(directory):
    directory = Path(directory)
    if not directory.exists():
        raise DirectoryDoesNotExistException(directory)
    if not directory.is_dir():
        raise DirectoryIsNotADirectoryException(directory)
    return directory.resolve()


def getLocalSession(directory):
    engine = create_engine("sqlite:///" + directory + "/" + KIT_FILENAME)
    Session = scoped_session(sessionmaker(bind=engine))
    return Session


def getLocalKit(directory):
    session = getLocalSession(directory)
    assert session.query(Kit).count() == 1
    return session.query(Kit).first()


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


class Query(ObjectType):
    kits = graphene.List(KitNode)

    def resolve_kits(self, info):
        query = KitNode.get_query(info)
        return query.all()


class CharacterizationInput(InputObjectType):
    name = graphene.String(required=True)
    explanation = graphene.String()
    symbol = graphene.String()


class ScriptInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput)
    kind = graphene.String(required=True)
    url = graphene.String(required=True)


class PropertyInput(InputObjectType):
    name = graphene.Field(CharacterizationInput)
    datatype = graphene.String(required=True)
    value = graphene.String(required=True)


class PortBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput)
    origin_x = graphene.Decimal(required=True)
    origin_y = graphene.Decimal(required=True)
    origin_z = graphene.Decimal(required=True)
    x_axis_x = graphene.Decimal(required=True)
    x_axis_y = graphene.Decimal(required=True)
    x_axis_z = graphene.Decimal(required=True)
    y_axis_x = graphene.Decimal(required=True)
    y_axis_y = graphene.Decimal(required=True)
    y_axis_z = graphene.Decimal(required=True)
    z_axis_x = graphene.Decimal(required=True)
    z_axis_y = graphene.Decimal(required=True)
    z_axis_z = graphene.Decimal(required=True)


class PortInput(InputObjectType):
    base = graphene.Field(PortBaseInput)
    properties = graphene.List(PropertyInput)


class PieceInput(InputObjectType):
    type_id = graphene.Int(required=True)


class AttractionInput(InputObjectType):
    attracting_piece_id = graphene.Int(required=True)
    attracting_piece_type_port_id = graphene.Int(required=True)
    attracted_piece_id = graphene.Int(required=True)
    attracted_piece_type_port_id = graphene.Int(required=True)


class FormationBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput)
    choreography_script_id = graphene.Int()
    transformation_script_id = graphene.Int()


class FormationInput(InputObjectType):
    base = graphene.Field(FormationBaseInput)
    pieces = graphene.List(PieceInput)
    attractions = graphene.List(AttractionInput)


class TypeBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput)
    prototype_script_id = graphene.Int()


class TypeInput(InputObjectType):
    base = graphene.Field(TypeBaseInput)
    ports = graphene.List(PortInput)
    properties = graphene.List(PropertyInput)


class KitBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput)
    uri = graphene.String(required=True)


class KitInput(InputObjectType):
    base = graphene.Field(KitBaseInput)
    scripts = graphene.List(ScriptInput)
    types = graphene.List(TypeInput)
    formations = graphene.List(FormationInput)


# graphene directory error handling classes


class DirectoryError(graphene.Enum):
    DOES_NOT_EXIST = "does_not_exist"
    NO_PERMISSION = "no_permission"


class LocalKitMutation(graphene.Union):
    class Meta:
        types = (KitNode, DirectoryError)


class CreateLocalKitMutation(Mutation):
    class Arguments:
        directory = graphene.String(required=True)
        kitInput = KitInput(required=True)

    Output = LocalKitMutation

    def mutate(self, info, directory, kitInput):
        directory = sanitizeAndCheckAndAbsolutizeDirectory(directory)

        engine = create_engine("sqlite:///" + directory + KIT_FILENAME)
        Base.base.create_all(engine)
        session = getLocalSession(directory)
        kit = Kit(
            uri=kitInput.uri, name=kitInput.name, explanation=kitInput.explanation
        )
        session.add(kit)
        session.commit()
        return kit


class UpdateLocalKitBaseMutation(Mutation):
    class Arguments:
        directory = graphene.String(required=True)
        kitBaseInput = KitBaseInput(required=True)

    Output = KitNode

    def mutate(self, info, directory, kitBaseInput):
        # try:
        #     directory = sanitizeAndCheckDirectory(directory)
        # except DirectoryException as directoryException:
        #     if isinstance(directoryException, DirectoryDoesNotExistException):
        #         return UpdateLocalKitBaseMutation(ok=False)
        session = getLocalSession(directory)
        kit = getLocalKit(directory)
        if kitBaseInput.uri:
            kit.uri = kitBaseInput.uri
        if kitBaseInput.name:
            kit.name = kitBaseInput.name
        if kitBaseInput.explanation:
            kit.explanation = kitBaseInput.explanation
        session.commit()
        return kit


class DeleteLocalKitMutation(Mutation):
    class Arguments:
        directory = graphene.String(required=True)

    ok = graphene.Boolean()

    def mutate(self, info, directory):
        sanitizeDirectory(directory)
        remove(directory + KIT_FILENAME)
        return DeleteLocalKitMutation(ok=True)


class UpdateMode(graphene.Enum):
    MERGE = "merge"
    REPLACE = "replace"


class CreateLocalTypeMutation(Mutation):
    class Arguments:
        directory = graphene.String(required=True)
        type = TypeInput(required=True)

    Output = TypeNode

    def mutate(self, info, directory, typeInput):
        session = getLocalSession(directory)
        type = Type(
            name=typeInput.name, explanation=typeInput.explanation, kit_id=kit_id
        )
        session.add(type)
        session.commit()
        return type


class UpdateTypeByNameMutation(Mutation):
    class Arguments:
        directory = graphene.String(required=True)
        name = graphene.String(required=True)
        type = TypeInput(required=True)

    Output = TypeNode


class DeleteTypeMutation(Mutation):
    class Arguments:
        id = graphene.Int(required=True)

    ok = graphene.Boolean()

    def mutate(self, info, id, directory):
        session = getLocalSession(directory)
        type = session.get(Type, id)
        session.delete(type)
        session.commit()
        return DeleteTypeMutation(ok=True)


class Mutation(ObjectType):
    create_kit = CreateKitMutation.Field()
    update_kit = UpdateKitMutation.Field()
    delete_kit = DeleteKitMutation.Field()
    create_type = CreateTypeMutation.Field()
    update_type = UpdateTypeMutation.Field()
    delete_type = DeleteTypeMutation.Field()


schema = Schema(query=Query, mutation=Mutation)
with open("schema.graphql", "w") as f:
    f.write(str(schema))

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
    if exists(SQLITE_PATH):
        remove(SQLITE_PATH)
    Base.base.create_all(engine)
    metabolism = Kit(
        uri="https://github.com/usalu/semio/tree/2.x/examples/metabolism",
        name="metabolism",
        explanation="A kit for metabolistic architecture.",
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
    # Base.base.create_all(engine)
