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
from functools import lru_cache
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
    Session,
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

URL_LENGTH_MAX = 1000
URI_SEPARATOR = "/"
NAME_LENGTH_MAX = 100
SYMBOL_LENGTH_MAX = 1
SCRIPT_KIND_LENGTH_MAX = 100
KIT_FILENAME = "kit.semio"

# SQLAlchemy


class Base(DeclarativeBase):
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
    url: Mapped[Optional[str]] = mapped_column(String(URL_LENGTH_MAX))
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
        return f"Script(id={self.id!r}, name={self.name!r}, kind={self.kind!r}, kit_id={self.kit_id!r})"

    # TODO: Implement URI mechanism for transient ids and knowledge graph ids
    # @property
    # def uri(self) -> str:
    #     return URI_SEPARATOR.join(
    #         self.kit.uri, self.__tablename__, self.kind, self.name
    #     )


# TODO: Implement two level inheritance based on PropertyOwnerKind and PropertyDatatype
# class PropertyOwnerKind(Enum):
#     TYPE = "type"
#     PORT = "port"
class PropertyDatatype(Enum):
    DECIMAL = "decimal"
    INTEGER = "integer"
    NATURAL = "natural"
    BOOLEAN = "boolean"
    FUZZY = "fuzzy"
    DESCRIPTION = "description"
    CHOICE = "choice"
    BLOB = "blob"


class Property(Base):
    __tablename__ = "property"

    id: Mapped[int] = mapped_column(primary_key=True)
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    datatype: Mapped[str] = mapped_column(sqlalchemy.Enum(PropertyDatatype))
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

    # __mapper_args__ = {
    #     "polymorphic_identity": "property",
    #     "polymorphic_on": "datatype",
    # }

    # @property
    # def uri(self) -> str:
    #     if self.type_id is not None:
    #         return URI_SEPARATOR.join(
    #         self.type.uri, self.name
    #     )
    #     return URI_SEPARATOR.join(
    #         self.port.uri, self.name
    #     )


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


# TODO: Rename type to type_ in order to avoid name clash with python type keyword
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
        return f"Type(id={self.id!r}, name={self.name!r}, kit_id={self.kit_id!r})"


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
    pieces: Mapped[Optional[List[Piece]]] = relationship(
        back_populates="formation", cascade="all, delete-orphan"
    )
    attractions: Mapped[Optional[List[Attraction]]] = relationship(
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
    name: Mapped[str] = mapped_column(String(NAME_LENGTH_MAX))
    explanation: Mapped[Optional[str]] = mapped_column(Text())
    symbol: Mapped[Optional[str]] = mapped_column(String(SYMBOL_LENGTH_MAX))
    url: Mapped[Optional[str]] = mapped_column(String(URL_LENGTH_MAX))
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
    def __init__(self, directory: String):
        self.directory = directory


class DirectoryDoesNotExistException(DirectoryException):
    def __str__(self):
        return "Directory does not exist: " + self.directory


class DirectoryIsNotADirectoryException(DirectoryException):
    def __str__(self):
        return "Directory is not a directory: " + self.directory


def assertDirectory(directory: String) -> Path:
    directory = Path(directory)
    if not directory.exists():
        raise DirectoryDoesNotExistException(directory)
    if not directory.is_dir():
        raise DirectoryIsNotADirectoryException(directory)
    return directory.resolve()


@lru_cache(maxsize=100)
def getLocalSession(directory: String) -> Session:
    directory = assertDirectory(directory)
    engine = create_engine("sqlite:///" + str(directory.joinpath(KIT_FILENAME)))
    Base.metadata.create_all(engine)
    # Create instance of session factory
    return sessionmaker(bind=engine)()


# Graphene

# Possible interface for a unified ui
# class Artifact(graphene.Interface):
#     id = graphene.ID(required=True)
#     name = graphene.String(required=True)
#     explanation = graphene.String()
#     symbol = graphene.String()
#     relatedTo = graphene.List(lambda: Artifact)

ScriptKindEnum = graphene.Enum.from_enum(ScriptKind)


class ScriptNode(SQLAlchemyObjectType):
    class Meta:
        model = Script
        exclude_fields = ("id", "kit_id")


class PropertyNode(SQLAlchemyObjectType):
    class Meta:
        model = Property
        exclude_fields = ("id", "synthesis_script_id", "type_id", "port_id")


class PortNode(SQLAlchemyObjectType):
    class Meta:
        model = Port
        exclude_fields = ("id", "type_id")


class PieceNode(SQLAlchemyObjectType):
    class Meta:
        model = Piece
        exclude_fields = ("id", "formation_id")


class AttractionNode(SQLAlchemyObjectType):
    class Meta:
        model = Attraction
        exclude_fields = (
            "attracting_piece_id",
            "attracting_piece_type_port_id",
            "attracted_piece_id",
            "attracted_piece_type_port_id",
            "formation_id",
        )


class FormationNode(SQLAlchemyObjectType):
    class Meta:
        model = Formation
        exclude_fields = ("id", "kit_id")


class TypeNode(SQLAlchemyObjectType):
    class Meta:
        model = Type
        exclude_fields = ("id", "kit_id")


class KitNode(SQLAlchemyObjectType):
    class Meta:
        model = Kit
        exclude_fields = ("id",)


class Query(ObjectType):
    localScripts = graphene.List(ScriptNode, directory=graphene.String(required=True))
    localTypes = graphene.List(TypeNode, directory=graphene.String(required=True))
    localFormations = graphene.List(
        FormationNode, directory=graphene.String(required=True)
    )
    localKit = graphene.Field(KitNode, directory=graphene.String(required=True))

    def resolve_localScripts(self, info, directory: graphene.String):
        session = getLocalSession(directory)
        return session.query(Script).all()

    def resolve_localTypes(self, info, directory: graphene.String):
        session = getLocalSession(directory)
        return session.query(Type).all()

    def resolve_localFormations(self, info, directory: graphene.String):
        session = getLocalSession(directory)
        return session.query(Formation).all()

    def resolve_localKit(self, info, directory: graphene.String):
        session = getLocalSession(directory)
        return session.query(Kit).first()


class CharacterizationInput(InputObjectType):
    name = graphene.String(required=True)
    explanation = graphene.String()
    symbol = graphene.String()


class ScriptInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput, required=True)
    kind = graphene.Field(ScriptKindEnum, required=True)
    url = graphene.String()


class PropertyInput(InputObjectType):
    name = graphene.String(required=True)
    datatype = graphene.String(required=True)
    value = graphene.String(required=True)


class PortBaseInput(InputObjectType):
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
    base = graphene.Field(PortBaseInput, required=True)
    properties = graphene.List(PropertyInput)


class PieceInput(InputObjectType):
    # Transient ID: Temporary ID to identify the piece in the formation
    transient_id = graphene.String()
    # ID Replacement: Instead of id use name to identify the type
    type_name = graphene.String(required=True)


class AttractionInput(InputObjectType):
    # Transient ID
    attracting_piece_transient_id = graphene.String(required=True)
    # ID Replacement: Instead of a port id use properties to identify the port
    attracting_piece_type_port_properties = graphene.List(PropertyInput, required=True)
    # Transient ID
    attracted_piece_transient_id = graphene.String(required=True)
    # ID Replacement: Instead of a port id use properties to identify the port
    attracted_piece_type_port_properties = graphene.List(PropertyInput, required=True)


class FormationBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput, required=True)
    # ID Replacement: Instead of id use name to identify the choreography script
    choreography_script_name = graphene.String()
    # ID Replacement: Instead of id use name to identify the transformation script
    transformation_script_name = graphene.String()


class FormationInput(InputObjectType):
    base = graphene.Field(FormationBaseInput, required=True)
    pieces = graphene.List(PieceInput)
    attractions = graphene.List(AttractionInput)


class TypeBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput, required=True)
    # ID Replacement: Instead of id use name to identify the prototype script
    prototype_script_name = graphene.String()


class TypeInput(InputObjectType):
    base = graphene.Field(TypeBaseInput, required=True)
    ports = graphene.List(PortInput)
    properties = graphene.List(PropertyInput)


class KitBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput, required=True)
    url = graphene.String()


class KitInput(InputObjectType):
    base = graphene.Field(KitBaseInput, required=True)
    scripts = graphene.List(ScriptInput)
    types = graphene.List(TypeInput)
    formations = graphene.List(FormationInput)


class NotFoundException(Exception):
    def __init__(self, name: String):
        self.name = name


class ScriptNotFoundException(NotFoundException):
    def __str__(self):
        return "Script not found: " + self.name


class PortNotFoundException(NotFoundException):
    def __str__(self):
        return "Port not found: " + self.name


class TypeNotFoundException(NotFoundException):
    def __str__(self):
        return "Type not found: " + self.name


class FormationNotFoundException(NotFoundException):
    def __str__(self):
        return "Formation not found: " + self.name


def getScriptByName(session: Session, name: String) -> Script:
    script = session.query(Script).filter_by(name=name).first()
    if script is None:
        raise ScriptNotFoundException(name)
    return script


def getTypeByName(session: Session, name: String) -> Type:
    type = session.query(Type).filter_by(name=name).first()
    if type is None:
        raise TypeNotFoundException(name)
    return type


def getPortByProperties(session: Session, properties: List[PropertyInput]) -> Port:
    port = (
        session.query(Port)
        .join(Port.properties)
        .filter(Property.name.in_([property.name for property in properties]))
        .first()
    )
    if port is None:
        raise PortNotFoundException(properties)
    return port


def getFormationByName(session: Session, name: String) -> Formation:
    formation = session.query(Formation).filter_by(name=name).first()
    if formation is None:
        raise FormationNotFoundException(name)
    return formation


def addScriptInputToSession(
    session: Session, kit: Kit, scriptInput: ScriptInput
) -> Script:
    try:
        explanation = scriptInput.characterization.explanation
    except AttributeError:
        explanation = None
    try:
        symbol = scriptInput.characterization.symbol
    except AttributeError:
        symbol = None
    script = Script(
        name=scriptInput.characterization.name,
        explanation=explanation,
        symbol=symbol,
        kind=scriptInput.kind,
        url=scriptInput.url,
        kit_id=kit.id,
    )
    session.add(script)
    session.flush()
    return script


def addPropertyInputToSession(
    session: Session, owner: Type | Port, propertyInput: PropertyInput
) -> Property:
    try:
        synthesisScriptId = getScriptByName(
            session, propertyInput.synthesis_script_name
        ).id
    except ScriptNotFoundException:
        synthesisScriptId = None
    if isinstance(owner, Type):
        typeId = owner.id
        portId = None
    elif isinstance(owner, Port):
        typeId = None
        portId = owner.id
    else:
        raise Exception("Unknown property owner")
    property = Property(
        name=propertyInput.name,
        datatype=propertyInput.datatype,
        value=propertyInput.value,
        synthesis_script_id=synthesisScriptId,
        type_id=typeId,
        port_id=portId,
    )
    session.add(property)
    session.flush()
    return property


def addPortInputToSession(session: Session, type: Type, portInput: PortInput) -> Port:
    port = Port(
        origin_x=portInput.base.origin_x,
        origin_y=portInput.base.origin_y,
        origin_z=portInput.base.origin_z,
        x_axis_x=portInput.base.x_axis_x,
        x_axis_y=portInput.base.x_axis_y,
        x_axis_z=portInput.base.x_axis_z,
        y_axis_x=portInput.base.y_axis_x,
        y_axis_y=portInput.base.y_axis_y,
        y_axis_z=portInput.base.y_axis_z,
        z_axis_x=portInput.base.z_axis_x,
        z_axis_y=portInput.base.z_axis_y,
        z_axis_z=portInput.base.z_axis_z,
        type_id=type.id,
    )
    for propertyInput in portInput.properties or []:
        property = addPropertyInputToSession(session, port, propertyInput)
    session.add(port)
    session.flush()
    return port


def addTypeInputToSession(session: Session, kit: Kit, typeInput: TypeInput) -> Type:
    try:
        explanation = typeInput.base.characterization.explanation
    except AttributeError:
        explanation = None
    try:
        symbol = typeInput.base.characterization.symbol
    except AttributeError:
        symbol = None
    try:
        prototypeScriptId = getScriptByName(
            session, typeInput.base.prototype_script_name
        ).id
    except ScriptNotFoundException:
        prototypeScriptId = None
    type = Type(
        name=typeInput.base.characterization.name,
        explanation=explanation,
        symbol=symbol,
        prototype_script_id=prototypeScriptId,
        kit_id=kit.id,
    )
    for portInput in typeInput.ports or []:
        port = addPortInputToSession(session, type, portInput)
    for propertyInput in typeInput.properties or []:
        property = addPropertyInputToSession(session, type, propertyInput)
    session.add(type)
    session.flush()
    return type


def addPieceInputToSession(
    session: Session, formation: Formation, pieceInput: PieceInput
) -> Piece:
    type = getTypeByName(session, pieceInput.type_name)
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
    attractingPiece = transientIdToPiece[attractionInput.attracting_piece_transient_id]
    attractedPiece = transientIdToPiece[attractionInput.attracted_piece_transient_id]
    attractingPieceTypePort = getPortByProperties(
        session, attractionInput.attracting_piece_type_port_properties
    )
    attractedPieceTypePort = getPortByProperties(
        session, attractionInput.attracted_piece_type_port_properties
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
        explanation = formationInput.base.characterization.explanation
    except AttributeError:
        explanation = None
    try:
        symbol = formationInput.base.characterization.symbol
    except AttributeError:
        symbol = None
    try:
        choreographyScript = getScriptByName(
            session, formationInput.base.choreography_script_name
        )
    except ScriptNotFoundException:
        choreographyScript = None
    try:
        transformationScript = getScriptByName(
            session, formationInput.base.transformation_script_name
        )
    except ScriptNotFoundException:
        transformationScript = None
    formation = Formation(
        name=formationInput.base.characterization.name,
        explanation=explanation,
        symbol=symbol,
        choreography_script_id=choreographyScript.id,
        transformation_script_id=transformationScript.id,
        kit_id=kit.id,
    )
    transientIdToPiece = {}
    for pieceInput in formationInput.pieces or []:
        piece = addPieceInputToSession(session, formation, pieceInput)
        transientIdToPiece[pieceInput.transient_id] = piece
    for attractionInput in formationInput.attractions or []:
        attraction = addAttractionInputToSession(
            session, formation, attractionInput, transientIdToPiece
        )
    session.add(formation)
    session.flush()
    return formation


def addKitInputToSession(session: Session, kitInput: KitInput):
    try:
        explanation = kitInput.base.characterization.explanation
    except AttributeError:
        explanation = None
    try:
        symbol = kitInput.base.characterization.symbol
    except AttributeError:
        symbol = None
    kit = Kit(
        url=kitInput.base.url,
        name=kitInput.base.characterization.name,
        explanation=explanation,
        symbol=symbol,
    )
    session.add(kit)
    session.flush()
    for scriptInput in kitInput.scripts or []:
        script = addScriptInputToSession(session, kit, scriptInput)
    for typeInput in kitInput.types or []:
        type = addTypeInputToSession(session, kit, typeInput)
    for formationInput in kitInput.formations or []:
        formation = addFormationInputToSession(session, kit, formationInput)
    return kit


# graphene directory error handling classes


# class DirectoryError(graphene.Enum):
#     DOES_NOT_EXIST = "does_not_exist"
#     IS_NOT_A_DIRECTORY = "is_not_a_directory"
#     NO_PERMISSION = "no_permission"
#     ALREADY_EXISTS = "already_exists"


# class DirectoryErrorNode(ObjectType):
#     directoryError = graphene.Field(DirectoryError, required=True)


# TODO: Implement BaseMutation mechanism and use it for all local mutations
# which have as first argument directory and return either the output or a
# DirectoryError.
# An example can be found here: https://github.com/graphql-python/graphene-django/blob/main/graphene_django/forms/mutation.py
# class LocalMutation(graphene.Mutation):
#     class Arguments:
#         directory = graphene.String(required=True)


class CreateLocalKitError(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_ALREADY_CONTAINS_A_KIT = "directory_already_contains_a_kit"
    NO_PERMISSION_TO_CREATE_DIRECTORY = "no_permission_to_create_directory"
    NO_PERMISSION_TO_CREATE_KIT = "no_permission_to_create_kit"


class CreateLocalKitErrorNode(ObjectType):
    error = graphene.Field(CreateLocalKitError, required=True)


disposed_engines = {}


class CreateLocalKitMutation(graphene.Mutation):
    class Arguments:
        directory = graphene.String(required=True)
        kitInput = KitInput(required=True)

    kit = graphene.Field(lambda: KitNode)
    error = graphene.Field(lambda: CreateLocalKitError)

    def mutate(self, info, directory, kitInput):
        directory = Path(directory)
        if not directory.exists():
            try:
                directory.mkdir(parents=True)
            except PermissionError:
                return CreateLocalKitMutation(
                    error=CreateLocalKitError.NO_PERMISSION_TO_CREATE_DIRECTORY
                )
            except OSError:
                return CreateLocalKitMutation(
                    error=CreateLocalKitError.DIRECTORY_IS_NOT_A_DIRECTORY
                )

        kitFile = directory.joinpath(KIT_FILENAME)
        if kitFile.exists():
            return CreateLocalKitMutation(
                error=CreateLocalKitError.DIRECTORY_ALREADY_CONTAINS_A_KIT
            )

        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in disposed_engines:
            raise Exception(
                "Can't create a new kit in a directory where this process already deleted an engine. Restart the server and try again."
            )

        session = getLocalSession(directory)
        kit = addKitInputToSession(session, kitInput)
        session.commit()
        return CreateLocalKitMutation(kit=kit, error=None)


# class UpdateLocalKitBaseMutation(graphene.Mutation):
#     class Arguments:
#         directory = graphene.String(required=True)
#         characterization = CharacterizationInput(required=True)

#     Output = KitNode

#     def mutate(self, info, directory, characterization):
#         session = getLocalSession(directory)
#         kit = session.query(Kit).first()
#         try:
#             kit.name = characterization.name
#         except AttributeError:
#             pass
#         try:
#             kit.explanation = characterization.explanation
#         except AttributeError:
#             pass
#         try:
#             kit.symbol = characterization.symbol
#         except AttributeError:
#             pass
#         session.commit()
#         return kit


# class DeleteLocalKitError(graphene.Enum):
#     NO_ERROR = "no_error"
#     DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
#     DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
#     NO_PERMISSION_TO_DELETE_KIT = "no_permission_to_delete_kit"


# class DeleteLocalKitErrorNode(ObjectType):
#     error = graphene.Field(DeleteLocalKitError, required=True)


# class DeleteLocalKitMutation(graphene.Mutation):
#     class Arguments:
#         directory = graphene.String(required=True)

#     Output = DeleteLocalKitErrorNode

#     def mutate(self, info, directory):
#         directory = Path(directory)
#         if not directory.exists():
#             return DeleteLocalKitErrorNode(
#                 error=DeleteLocalKitError.DIRECTORY_DOES_NOT_EXIST
#             )
#         kitFile = directory.joinpath(KIT_FILENAME)
#         if not kitFile.exists():
#             return DeleteLocalKitErrorNode(
#                 error=DeleteLocalKitError.DIRECTORY_HAS_NO_KIT
#             )
#         kitFileFullPath = kitFile.resolve()
#         disposed_engines[kitFileFullPath] = True
#         try:
#             remove(kitFileFullPath)
#         except PermissionError:
#             return DeleteLocalKitErrorNode(
#                 error=DeleteLocalKitError.NO_PERMISSION_TO_DELETE_KIT
#             )
#         return DeleteLocalKitErrorNode(error=DeleteLocalKitError.NO_ERROR)


# class AddLocalError(graphene.Enum):
#     DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
#     DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
#     DIRECTORY_NO_PERMISSION = "directory_no_permission"
#     DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
#     ALREADY_EXISTS = "already_exists"


# class AddLocalTypeErrorNode(ObjectType):
#     error = graphene.Field(AddLocalError, required=True)


# class AddLocalTypeMutationNodes(graphene.Union):
#     class Meta:
#         types = (TypeNode, AddLocalTypeErrorNode)


# class AddLocalTypeMutation(graphene.Mutation):
#     class Arguments:
#         directory = graphene.String(required=True)
#         typeInput = TypeInput(required=True)

#     Output = TypeNode

#     def mutate(self, info, directory, typeInput):
#         directory = Path(directory)
#         if not directory.exists():
#             return AddLocalTypeErrorNode(error=AddLocalError.DIRECTORY_DOES_NOT_EXIST)
#         if not directory.is_dir():
#             return AddLocalTypeErrorNode(
#                 error=AddLocalError.DIRECTORY_IS_NOT_A_DIRECTORY
#             )
#         kitFile = directory.joinpath(KIT_FILENAME)
#         if not kitFile.exists():
#             return AddLocalTypeErrorNode(error=AddLocalError.DIRECTORY_HAS_NO_KIT)

#         session = getLocalSession(directory)
#         kit = session.query(Kit).first()
#         type = addTypeInputToSession(session, typeInput)
#         type.kit = kit
#         session.commit()
#         return type


class Mutation(ObjectType):
    createLocalKit = CreateLocalKitMutation.Field()
    # updateLocalKitBase = UpdateLocalKitBaseMutation.Field()
    # deleteLocalKit = DeleteLocalKitMutation.Field()
    # addLocalType = AddLocalTypeMutation.Field()


schema = Schema(query=Query, mutation=Mutation)
# with open("schema.graphql", "w") as f:
#     f.write(str(schema))

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
