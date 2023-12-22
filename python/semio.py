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


def assertDirectory(directory):
    directory = Path(directory)
    if not directory.exists():
        raise DirectoryDoesNotExistException(directory)
    if not directory.is_dir():
        raise DirectoryIsNotADirectoryException(directory)
    return directory.resolve()


@lru_cache(maxsize=100)
def getLocalSession(directory):
    directory = assertDirectory(directory)
    engine = create_engine("sqlite:///" + str(directory.joinpath(KIT_FILENAME)))
    Base.metadata.create_all(engine)
    # Delete instance of session factory
    return sessionmaker(bind=engine)()


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
    localScripts = graphene.List(ScriptNode, directory=graphene.String(required=True))
    localTypes = graphene.List(TypeNode, directory=graphene.String(required=True))
    localFormations = graphene.List(
        FormationNode, directory=graphene.String(required=True)
    )
    localKit = graphene.Field(KitNode, directory=graphene.String(required=True))

    def resolve_localScripts(self, info, directory):
        session = getLocalSession(directory)
        return session.query(Script).all()

    def resolve_localTypes(self, info, directory):
        session = getLocalSession(directory)
        return session.query(Type).all()

    def resolve_localFormations(self, info, directory):
        session = getLocalSession(directory)
        return session.query(Formation).all()

    def resolve_localKit(self, info, directory):
        session = getLocalSession(directory)
        return session.query(Kit).first()


class CharacterizationInput(InputObjectType):
    name = graphene.String(required=True)
    explanation = graphene.String()
    symbol = graphene.String()


class ScriptInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput, required=True)
    kind = graphene.String(required=True)
    url = graphene.String(required=True)


class PropertyInput(InputObjectType):
    name = graphene.String(required=True)
    datatype = graphene.String(required=True)
    value = graphene.String(required=True)


class PortBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput, required=True)
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
    type_id = graphene.Int(required=True)


class AttractionInput(InputObjectType):
    attracting_piece_id = graphene.Int(required=True)
    attracting_piece_type_port_id = graphene.Int(required=True)
    attracted_piece_id = graphene.Int(required=True)
    attracted_piece_type_port_id = graphene.Int(required=True)


class FormationBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput, required=True)
    choreography_script_id = graphene.Int()
    transformation_script_id = graphene.Int()


class FormationInput(InputObjectType):
    base = graphene.Field(FormationBaseInput, required=True)
    pieces = graphene.List(PieceInput)
    attractions = graphene.List(AttractionInput)


class TypeBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput, required=True)
    prototype_script_id = graphene.Int()


class TypeInput(InputObjectType):
    base = graphene.Field(TypeBaseInput, required=True)
    ports = graphene.List(PortInput)
    properties = graphene.List(PropertyInput)


class KitBaseInput(InputObjectType):
    characterization = graphene.Field(CharacterizationInput, required=True)
    uri = graphene.String(required=True)


class KitInput(InputObjectType):
    base = graphene.Field(KitBaseInput, required=True)
    scripts = graphene.List(ScriptInput)
    types = graphene.List(TypeInput)
    formations = graphene.List(FormationInput)


def addScriptInputToKit(kit: Kit, scriptInput):
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
    )
    kit.scripts.append(script)
    return script


def addPropertyInputToKit(kit: Kit, propertyInput: PropertyInput):
    property = Property(
        name=propertyInput.name,
        datatype=propertyInput.datatype,
        value=propertyInput.value,
    )
    kit.properties.append(property)
    return property


def addPortInputToKit(kit: Kit, portInput: PortInput):
    try:
        explanation = portInput.base.characterization.explanation
    except AttributeError:
        explanation = None
    try:
        symbol = portInput.base.characterization.symbol
    except AttributeError:
        symbol = None
    port = Port(
        name=portInput.base.characterization.name,
        explanation=explanation,
        symbol=symbol,
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
    )
    for propertyInput in portInput.properties or []:
        property = addPropertyInputToKit(kit, propertyInput)
        property.port = port
    kit.ports.append(port)
    return port


def addTypeInputToKit(kit: Kit, typeInput: TypeInput):
    try:
        explanation = typeInput.base.characterization.explanation
    except AttributeError:
        explanation = None
    try:
        symbol = typeInput.base.characterization.symbol
    except AttributeError:
        symbol = None
    type = Type(
        name=typeInput.base.characterization.name,
        explanation=explanation,
        symbol=symbol,
    )
    for portInput in typeInput.ports or []:
        port = addPortInputToKit(kit, portInput)
        port.type = type
    for propertyInput in typeInput.properties or []:
        property = addPropertyInputToKit(kit, propertyInput)
        property.type = type
    kit.types.append(type)
    return type


def addPieceInputToKit(kit: Kit, pieceInput: PieceInput):
    piece = Piece(type_id=pieceInput.type_id)
    kit.pieces.append(piece)
    return piece


def addAttractionInputToKit(kit: Kit, attractionInput: AttractionInput):
    attraction = Attraction(
        attracting_piece_id=attractionInput.attracting_piece_id,
        attracting_piece_type_port_id=attractionInput.attracting_piece_type_port_id,
        attracted_piece_id=attractionInput.attracted_piece_id,
        attracted_piece_type_port_id=attractionInput.attracted_piece_type_port_id,
    )
    kit.attractions.append(attraction)
    return attraction


def addFormationInputToKit(kit: Kit, formationInput: FormationInput):
    try:
        explanation = formationInput.base.characterization.explanation
    except AttributeError:
        explanation = None
    try:
        symbol = formationInput.base.characterization.symbol
    except AttributeError:
        symbol = None
    formation = Formation(
        name=formationInput.base.characterization.name,
        explanation=explanation,
        symbol=symbol,
        choreography_script_id=formationInput.base.choreography_script_id,
        transformation_script_id=formationInput.base.transformation_script_id,
    )
    for pieceInput in formationInput.pieces or []:
        piece = addPieceInputToKit(kit, pieceInput)
        piece.formation = formation
    for attractionInput in formationInput.attractions or []:
        attraction = addAttractionInputToKit(kit, attractionInput)
        attraction.formation = formation
    kit.formations.append(formation)
    return formation


def kitInputToKit(kitInput: KitInput):
    try:
        explanation = kitInput.base.characterization.explanation
    except AttributeError:
        explanation = None
    try:
        symbol = kitInput.base.characterization.symbol
    except AttributeError:
        symbol = None
    kit = Kit(
        uri=kitInput.base.uri,
        name=kitInput.base.characterization.name,
        explanation=explanation,
        symbol=symbol,
    )
    for scriptInput in kitInput.scripts or []:
        addScriptInputToKit(kit, scriptInput)
    for typeInput in kitInput.types or []:
        addTypeInputToKit(kit, typeInput)
    for formationInput in kitInput.formations or []:
        addFormationInputToKit(kit, formationInput)
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
# class LocalMutation(Mutation):
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


class CreateLocalKitMutationNodes(graphene.Union):
    class Meta:
        types = (KitNode, CreateLocalKitErrorNode)


deleted_engines = {}


class CreateLocalKitMutation(Mutation):
    class Arguments:
        directory = graphene.String(required=True)
        kitInput = KitInput(required=True)

    Output = CreateLocalKitMutationNodes

    def mutate(self, info, directory, kitInput):
        directory = Path(directory)
        if not directory.exists():
            try:
                directory.mkdir(parents=True)
            except PermissionError:
                return CreateLocalKitErrorNode(
                    error=CreateLocalKitError.NO_PERMISSION_TO_CREATE_DIRECTORY
                )
            except OSError:
                return CreateLocalKitErrorNode(
                    error=CreateLocalKitError.DIRECTORY_IS_NOT_A_DIRECTORY
                )

        kitFile = directory.joinpath(KIT_FILENAME)
        if kitFile.exists():
            return CreateLocalKitErrorNode(
                error=CreateLocalKitError.DIRECTORY_ALREADY_CONTAINS_A_KIT
            )

        kitFileFullPath = kitFile.resolve()
        if kitFileFullPath in deleted_engines:
            raise Exception(
                "Can't create a new kit in a directory where this process already deleted an engine. Restart the server and try again."
            )

        session = getLocalSession(directory)
        kit = kitInputToKit(kitInput)
        session.add(kit)
        session.commit()
        return kit


class UpdateLocalKitBaseMutation(Mutation):
    class Arguments:
        directory = graphene.String(required=True)
        characterization = CharacterizationInput(required=True)

    Output = KitNode

    def mutate(self, info, directory, characterization):
        session = getLocalSession(directory)
        kit = session.query(Kit).first()
        try:
            kit.name = characterization.name
        except AttributeError:
            pass
        try:
            kit.explanation = characterization.explanation
        except AttributeError:
            pass
        try:
            kit.symbol = characterization.symbol
        except AttributeError:
            pass
        session.commit()
        return kit


class DeleteLocalKitError(graphene.Enum):
    NO_ERROR = "no_error"
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    NO_PERMISSION_TO_DELETE_KIT = "no_permission_to_delete_kit"


class DeleteLocalKitErrorNode(ObjectType):
    error = graphene.Field(DeleteLocalKitError, required=True)


class DeleteLocalKitMutation(Mutation):
    class Arguments:
        directory = graphene.String(required=True)

    Output = DeleteLocalKitErrorNode

    def mutate(self, info, directory):
        directory = Path(directory)
        if not directory.exists():
            return DeleteLocalKitErrorNode(
                error=DeleteLocalKitError.DIRECTORY_DOES_NOT_EXIST
            )
        kitFile = directory.joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return DeleteLocalKitErrorNode(
                error=DeleteLocalKitError.DIRECTORY_HAS_NO_KIT
            )
        kitFileFullPath = kitFile.resolve()
        deleted_engines[kitFileFullPath] = True
        try:
            remove(kitFileFullPath)
        except PermissionError:
            return DeleteLocalKitErrorNode(
                error=DeleteLocalKitError.NO_PERMISSION_TO_DELETE_KIT
            )
        return DeleteLocalKitErrorNode(error=DeleteLocalKitError.NO_ERROR)


class AddLocalTypeError(graphene.Enum):
    DIRECTORY_DOES_NOT_EXIST = "directory_does_not_exist"
    DIRECTORY_IS_NOT_A_DIRECTORY = "directory_is_not_a_directory"
    DIRECTORY_NO_PERMISSION = "directory_no_permission"
    DIRECTORY_HAS_NO_KIT = "directory_has_no_kit"
    TYPE_ALREADY_EXISTS = "type_already_exists"


class AddLocalTypeErrorNode(ObjectType):
    error = graphene.Field(AddLocalTypeError, required=True)


class AddLocalTypeMutationNodes(graphene.Union):
    class Meta:
        types = (TypeNode, AddLocalTypeErrorNode)


class AddLocalTypeMutation(Mutation):
    class Arguments:
        directory = graphene.String(required=True)
        typeInput = TypeInput(required=True)

    Output = TypeNode

    def mutate(self, info, directory, typeInput):
        directory = Path(directory)
        if not directory.exists():
            return AddLocalTypeErrorNode(
                error=AddLocalTypeError.DIRECTORY_DOES_NOT_EXIST
            )
        if not directory.is_dir():
            return AddLocalTypeErrorNode(
                error=AddLocalTypeError.DIRECTORY_IS_NOT_A_DIRECTORY
            )
        kitFile = directory.joinpath(KIT_FILENAME)
        if not kitFile.exists():
            return AddLocalTypeErrorNode(error=AddLocalTypeError.DIRECTORY_HAS_NO_KIT)

        session = getLocalSession(directory)
        kit = session.query(Kit).first()
        type = addTypeInputToKit(kit, typeInput)
        session.commit()
        return type


class Mutation(ObjectType):
    createLocalKit = CreateLocalKitMutation.Field()
    updateLocalKitBase = UpdateLocalKitBaseMutation.Field()
    deleteLocalKit = DeleteLocalKitMutation.Field()
    addLocalType = AddLocalTypeMutation.Field()


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
