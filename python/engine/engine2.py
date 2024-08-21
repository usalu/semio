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
# TODO: Add constraint to designs that at least 2 pieces and 1 connection are required.
# TODO: Make uvicorn pyinstaller multiprocessing work. Then qt can be integrated again for system tray.
import inspect
from abc import abstractclassmethod
from argparse import ArgumentParser
import os
import logging  # for uvicorn in pyinstaller
from os import remove
from pathlib import Path
from multiprocessing import freeze_support
from functools import lru_cache
import sqlite3
from typing import Optional, Dict, Protocol, List, Union, get_type_hints
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

import sqlmodel
from sqlalchemy import (
    CheckConstraint,
    UniqueConstraint,
    String as SQLString,
    Integer as SQLInteger,
    create_engine,
)
from sqlalchemy.orm import sessionmaker, Session, validates, object_session
from pydantic import BaseModel, computed_field
from fastapi import FastAPI
import graphene

import graphene
from graphene_pydantic import PydanticInputObjectType, PydanticObjectType
from graphene_sqlalchemy import SQLAlchemyObjectType
from uvicorn import run
from starlette.applications import Starlette
from starlette_graphene3 import GraphQLApp, make_graphiql_handler


logging.basicConfig(level=logging.INFO)  # for uvicorn in pyinstaller

VERSION = "3.0.0"
RELEASE = "r24.09-1"
NAME_LENGTH_MAX = 100
DESCRIPTION_LENGTH_MAX = 5000
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
    """🔗 The URL is not valid. An url must have the form:
    scheme://netloc/path;parameters?query#fragment."""

    def __init__(self, url: str) -> None:
        self.url = url

    def __str__(self) -> str:
        return f"{self.url} is not a valid URL."


class InvalidDatabase(SemioException):
    """💾 The state of the database is somehow invalid.
    Check the constraints and the insert validators.
    """

    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + "\n The database is invalid. Please report this bug."


class InvalidBackend(SemioException):
    """🖥️ The backend processed something wrong. Check the order of operations."""

    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + "\n The backend is invalid. Please report this bug."


@lru_cache(maxsize=100)
def getLocalSession(path: str) -> Session:
    engine = create_engine("sqlite:///" + path, echo=True)
    sqlmodel.SQLModel.metadata.create_all(engine)
    return sessionmaker(bind=engine)()


class Semio(sqlmodel.SQLModel):
    """ℹ️ Metadata about the semio database."""

    __tablename__ = "semio"

    engineVersion: str = sqlmodel.Field(default=VERSION, primary_key=True)
    release: str = sqlmodel.Field(default=RELEASE)
    # createdAt: datetime = sqlmodel.Field(default_factory=datetime.now)


class SemioEntity(sqlmodel.SQLModel):
    """Base class for all entitites in semio."""

    def humanId(self) -> str:
        """🪪 A string that let's the user identify the entity."""
        pass

    def notFoundMessage(self) -> str:
        """A message"""
        return

    def notFoundError(self) -> SemioException:
        return SemioException(self.notFoundMessage())


class ArtifactModel(sqlmodel.SQLModel):
    """♻️ An artifact is anything that is worth to be reused."""

    name: str = sqlmodel.Field(max_length=NAME_LENGTH_MAX)
    # Optional. Set to "" for None.
    description: str = sqlmodel.Field(max_length=DESCRIPTION_LENGTH_MAX, default="")
    # Optional. Set to "" for None.
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_MAX)
    createdAt: datetime = sqlmodel.Field(default_factory=datetime.now)
    lastUpdateAt: datetime = sqlmodel.Field(default_factory=datetime.now)


class VariableArtifactModel(ArtifactModel):
    """🎚️ A variable artifact is an artifact that has variants (at least one default)."""

    variant: str = ""


class Tag(sqlmodel.SQLModel, table=True):
    """🏷️ A tag is meta-data for grouping representations."""

    __tablename__ = "tag"
    value: str = sqlmodel.Field(primary_key=True)
    representationPk: int = sqlmodel.Field(
        foreign_key=("representation.id"), primary_key=True, exclude=True
    )
    representation: "Representation" = sqlmodel.Relationship(back_populates="_tags")


class RepresentationBase(sqlmodel.SQLModel):
    url: str
    lod: str = ""


class Representation(RepresentationBase, table=True):
    """💾 A representation is a link to a file that describes a type for a unique combination of level of detail, tags and mime."""

    __tablename__ = "representation"
    pk: Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            SQLInteger,
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
            SQLString(NAME_LENGTH_MAX),
        ),
    )
    _tags: list[Tag] = sqlmodel.Relationship(
        back_populates="representation", cascade_delete=True
    )
    typePk: Optional[int] = sqlmodel.Field(default=None, foreign_key=("type.id"))
    type: Union["Type", None] = sqlmodel.Relationship(back_populates="representations")

    __table_args__ = (UniqueConstraint("url"),)

    @property
    def tags(self) -> list[str]:
        return [tag.value for tag in self._tags or []]

    @tags.setter
    def tags(self, tags: list[str]):
        self._tags = [Tag(value=tag) for tag in tags]

    @property
    def id(self) -> str:
        return self.id_

    @validates("url")
    def validate_url(self, key: str, url: str):
        return url


class RepresentationSkeleton(RepresentationBase):
    class Config:
        title = "Representation"

    id: str = ""
    tags: list[str] = sqlmodel.Field(default_factory=list)


class TypeBase(VariableArtifactModel):
    pass


class Type(TypeBase, table=True):
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    __tablename__ = "type"
    pk: Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            SQLInteger,
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    representations: list[Representation] = sqlmodel.Relationship(
        back_populates="type",
        cascade_delete=True,
    )
    kitPk: Optional[int] = sqlmodel.Field(default=None, foreign_key=("kit.id2"))
    kit: Union["Kit", None] = sqlmodel.Relationship(back_populates="types")

    # __table_args__ = (UniqueConstraint("name", "variant", "kitPk"),)


class TypeSkeleton(TypeBase):
    class Config:
        title = "Type"

    representations: list[RepresentationSkeleton] = sqlmodel.Field(default_factory=list)


class KitBase(ArtifactModel):
    url: str = sqlmodel.Field(max_length=URL_LENGTH_MAX, default="")
    homepage: str = sqlmodel.Field(max_length=URL_LENGTH_MAX, default="")


class Kit(KitBase, table=True):
    """🗃️ A kit is a collection of types and designs."""

    __tablename__ = "kit"
    pk: Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id2",
            SQLInteger,
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    types: list[Type] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    # designs: list[Design] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)

    __table_args__ = (UniqueConstraint("name"), UniqueConstraint("url"))


class KitSkeleton(KitBase):

    class Config:
        title = "Kit"

    types: list[TypeSkeleton] = sqlmodel.Field(default_factory=list)


def create_db_and_tables():
    path = Path("engine2.sqlite3")
    try:
        remove(path)
    except:
        pass
    r1 = Representation(id="g", url="https://www.google.com")
    r2 = Representation(id="y", url="https://www.yahoo.com")
    r2.tags = ["tag1", "tag2"]
    r2.tags = ["tag3", "tag4"]
    r3 = Representation(id="y2", url="https://www.yahoo.com1")
    t1 = Type(name="capsule")
    t1.representations = [r1, r2, r3]
    k1 = Kit(name="kit1", types=[t1])
    engine = create_engine("sqlite:///" + str(path))
    sqlmodel.SQLModel.metadata.create_all(engine)
    with Session(engine) as session:
        session.add(k1)
        [r1n, r2n, r3n] = t1.representations
        r2n.tags = ["tag5", "tag6"]
        session.commit()
        pass
    pass


create_db_and_tables()


def getKit(url: str) -> Kit:
    session = getLocalSession("engine2.sqlite3")
    kit = session.query(Kit).first()
    return kit


# ---GraphQL---

GRAPHQLTYPES = {
    str: graphene.NonNull(graphene.String),
    int: graphene.NonNull(graphene.Int),
    float: graphene.NonNull(graphene.Float),
    bool: graphene.NonNull(graphene.Boolean),
    list[str]: graphene.NonNull(graphene.List(graphene.NonNull(graphene.String))),
}


class SemioNode(SQLAlchemyObjectType):
    """A base class for all nodes in the semio graph.
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

        own_properties = [
            name
            for name, value in model.__dict__.items()
            if isinstance(value, property)
        ]
        # Dynamically add resolvers for all properties
        for name in own_properties:
            prop = getattr(model, name)
            prop_getter = prop.fget
            prop_return_type = inspect.signature(prop_getter).return_annotation
            setattr(cls, name, GRAPHQLTYPES[prop_return_type])

            def make_resolver(name):
                def resolver(self, info):
                    return getattr(self, name)

                return resolver

            resolver_name = f"resolve_{name}"
            setattr(cls, resolver_name, make_resolver(name))

        super().__init_subclass_with_meta__(model=model, **options)


class RepresentationNode(SemioNode):
    class Meta:
        model = Representation


class RepresentationInput(PydanticInputObjectType):
    class Meta:
        model = RepresentationSkeleton


class TypeNode(SemioNode):
    class Meta:
        model = Type


class TypeInput(PydanticInputObjectType):
    class Meta:
        model = TypeSkeleton


class KitNode(SemioNode):
    class Meta:
        model = Kit


class KitInput(PydanticInputObjectType):
    class Meta:
        model = KitSkeleton


class Query(graphene.ObjectType):
    kit = graphene.Field(KitNode, url=graphene.String(required=True))

    def resolve_kit(self, info, url):
        return getKit(url)


class Mutation(graphene.ObjectType):
    createKit = graphene.Field(KitNode, kit=KitInput(required=True))


schema = graphene.Schema(
    query=Query,
    mutation=Mutation,
)


def start_engine():

    fastapi_app = FastAPI()

    @fastapi_app.get("/kits/{url}")
    async def read_kits(url) -> KitSkeleton:
        return getKit(url)

    engine = Starlette()
    engine.mount("/graphql", GraphQLApp(schema, on_get=make_graphiql_handler()))
    engine.mount("/api", fastapi_app)

    run(
        engine,
        host=HOST,
        port=PORT,
        log_level="info",
        access_log=False,
        log_config=None,
    )


def extract_schema(db_path, output_path):
    conn = sqlite3.connect(db_path)
    cursor = conn.cursor()

    cursor.execute("SELECT sql FROM sqlite_master WHERE type='table';")
    schema = cursor.fetchall()

    with open(output_path, "w", encoding="utf-8") as f:
        for table in schema:
            f.write(f"{table[0]};\n")

    conn.close()


def main():
    parser = ArgumentParser()
    parser.add_argument("--debug", action="store_true", help="Enable debug mode")
    args = parser.parse_args()
    if args.debug:
        with open("../../graphql/schema.graphql", "w", encoding="utf-8") as f:
            f.write(str(schema))
        sqliteSchemaPath = "../../sqlite/schema.sql"
        if os.path.exists(sqliteSchemaPath):
            os.remove(sqliteSchemaPath)
        metadata_engine = create_engine("sqlite:///debug/semio.db")
        sqlmodel.SQLModel.metadata.create_all(metadata_engine)
        extract_schema("debug/semio.db", "../../sqlite/schema.sql")

    start_engine()


if __name__ == "__main__":
    freeze_support()  # needed for pyinstaller on Windows
    main()
