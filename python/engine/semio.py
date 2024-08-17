from functools import lru_cache
from os import remove
from pathlib import Path
from typing import List, Optional, Union
from urllib.parse import urlparse
from sqlmodel import (
    SQLModel,
    Field as ModelField,
    Relationship,
    Column,
)
from sqlalchemy import (
    CheckConstraint,
    UniqueConstraint,
    String as SQLString,
    Integer as SQLInteger,
    create_engine,
)
from sqlalchemy.orm import sessionmaker, Session, validates, object_session
from pydantic import computed_field
from fastapi import FastAPI
from graphene import (
    Schema,
    ObjectType,
    Field as GraphField,
    NonNull as GraphNonNull,
    String as GraphString,
    List as GraphList,
)
import graphene
from graphene_pydantic import PydanticInputObjectType, PydanticObjectType
from graphene_sqlalchemy import SQLAlchemyObjectType
from uvicorn import run
from starlette.applications import Starlette
from starlette_graphene3 import GraphQLApp, make_graphiql_handler

HOST = "127.0.0.1"
PORT = 5052

NAME_LENGTH_MAX = 100


@lru_cache(maxsize=100)
def getLocalSession(path: str) -> Session:
    engine = create_engine("sqlite:///" + path, echo=True)
    SQLModel.metadata.create_all(engine)
    return sessionmaker(bind=engine)()


class ArtifactModel(SQLModel):

    name: str
    description: str = ""
    icon: str = ""


class Tag(SQLModel, table=True):
    """🏷️ A tag is meta-data for grouping representations."""

    __tablename__ = "tag"

    value: str = ModelField(primary_key=True)
    representationPk: int = ModelField(
        foreign_key=("representation.id"), primary_key=True, exclude=True
    )
    representation: "Representation" = Relationship(back_populates="_tags")


class RepresentationBase(SQLModel):
    """💾 A representation is a link to a file that describes a type for a unique combination of level of detail, tags and mime."""

    url: str
    lod: str = ""


class Representation(RepresentationBase, table=True):

    __tablename__ = "representation"

    pk: Optional[int] = ModelField(
        sa_column=Column(
            "id",
            SQLInteger,
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    # Can't use the name 'id' because of bug
    # https://github.com/graphql-python/graphene-sqlalchemy/issues/412
    id_: str = ModelField(
        alias="id",
        sa_column=Column(
            "localId",
            SQLString(NAME_LENGTH_MAX),
        ),
    )

    _tags: list[Tag] = Relationship(
        back_populates="representation", cascade_delete=True
    )
    typePk: Optional[int] = ModelField(default=None, foreign_key=("type.id"))
    type: Union["Type", None] = Relationship(back_populates="representations")
    __table_args__ = (UniqueConstraint("url"),)

    @property
    def tags(self) -> list[str]:
        return [tag.value for tag in self._tags or []]

    @tags.setter
    def tags(self, tags: list[str]):
        self._tags = [Tag(value=tag) for tag in tags]

    @validates("url")
    def validate_url(self, key: str, url: str):
        return url


class RepresentationOutput(RepresentationBase):

    class Config:
        title = "Representation"

    id: str = ""
    tags: list[str] = ModelField(default_factory=list)


class TypeBase(ArtifactModel):
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    variant: str = ""
    # __table_args__ = (UniqueConstraint("name", "variant", "kitId"),)


class Type(TypeBase, table=True):

    __tablename__ = "type"

    pk: Optional[int] = ModelField(
        sa_column=Column(
            "id",
            SQLInteger,
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    group: str = ModelField(
        "", sa_column=Column("group_name", SQLString(NAME_LENGTH_MAX))
    )
    representations: list[Representation] = Relationship(
        back_populates="type",
        cascade_delete=True,
    )


class TypeOutput(TypeBase):

    class Config:
        title = "Type"

    representations: list[RepresentationOutput] = ModelField(default_factory=list)


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
    engine = create_engine("sqlite:///" + str(path))
    SQLModel.metadata.create_all(engine)
    with Session(engine) as session:
        session.add(t1)
        [r1n, r2n, r3n] = t1.representations
        r2n.tags = ["tag5", "tag6"]
        session.commit()
        pass
    pass


create_db_and_tables()


def getAllTypes():
    session = getLocalSession("engine2.sqlite3")
    types = session.query(Type).all()
    # types_dict = [type.model_dump() for type in types]
    # return types_dict
    return types


# ---GraphQL---


class RepresentationNode(SQLAlchemyObjectType):

    class Meta:
        model = Representation
        name = "Representation"
        exclude_fields = ("pk", "id_", "_tags", "typePk")

    id = GraphString()

    def resolve_id(self, info):
        return self.id_


class RepresentationInput(PydanticInputObjectType):
    class Meta:
        model = Representation


class TypeNode(SQLAlchemyObjectType):
    class Meta:
        model = Type
        name = "Type"
        exclude_fields = ("pk",)


class TypeInput(PydanticInputObjectType):
    class Meta:
        model = Type


class CreateType(graphene.Mutation):
    class Arguments:
        type = GraphNonNull(TypeInput)

    type = GraphField(TypeNode)

    def mutate(self, info, directory, type: TypeInput):
        session = getLocalSession("engine2.sqlite3")
        type = Type(**type.dict())
        session.add(type)
        session.commit()
        return CreateType(type=type)


class TypesResponse(ObjectType):
    types = GraphList(TypeNode)


class Query(ObjectType):
    types = GraphField(TypesResponse)

    def resolve_types(self, info):
        return TypesResponse(types=getAllTypes())


class Mutation(ObjectType):
    create_type = CreateType.Field()


schema = Schema(
    query=Query,
    mutation=Mutation,
)
with open("schema.graphql", "w", encoding="utf-8") as f:
    f.write(str(schema))

fastapi_app = FastAPI()


@fastapi_app.get("/types")
async def read_root() -> list[TypeOutput]:
    return getAllTypes()


engine = Starlette()
engine.mount("/graphql", GraphQLApp(schema, on_get=make_graphiql_handler()))
engine.mount("/", fastapi_app)

run(
    engine,
    host=HOST,
    port=PORT,
    log_level="info",
    access_log=False,
    log_config=None,
)
