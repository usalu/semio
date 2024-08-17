import json
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
from sqlalchemy.orm import (
    sessionmaker,
    Session,
    validates,
    object_session,
    mapped_column,
    Mapped,
)
from pydantic import BaseModel, Field, computed_field
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


class A(SQLModel, table=True):
    id: Optional[int] = ModelField(default=None, primary_key=True)
    foo: str
    bId: Optional[int] = ModelField(default=None, foreign_key="b.id")
    b: Union["B", None] = Relationship(back_populates="a_s")


class B(SQLModel, table=True):
    id: Optional[int] = ModelField(default=None, primary_key=True)
    bar: str
    a_s: List[A] = Relationship(back_populates="b")


class ANode(SQLAlchemyObjectType):
    class Meta:
        model = A


class BNode(SQLAlchemyObjectType):

    class Meta:
        model = B


class AInput(PydanticInputObjectType):
    class Meta:
        model = A


class BInput(PydanticInputObjectType):

    class Meta:
        model = B


class CreateB(graphene.Mutation):
    class Arguments:
        b = GraphNonNull(BInput)

    b = GraphField(BNode)

    def mutate(self, info, directory, b: BInput):
        session = getLocalSession("engine2.sqlite3")
        b = B(**b.dict())
        session.add(b)
        session.commit()
        return CreateB(b=b)


class C(SQLModel):
    # _hidden_prop: Mapped[int] = mapped_column(primary_key=True)
    # _hidden_prop: Optional[int] = ModelField(default=None, primary_key=True)
    vis_prop: str


class CInput(PydanticInputObjectType):
    class Meta:
        model = C


class CNode(PydanticObjectType):
    class Meta:
        model = C


# b = B(a=[A(foo="bar")])
# try:
#     remove("engine2.sqlite3")
# except:
#     pass
# session = getLocalSession("engine2.sqlite3")
# session.add(b)
# session.commit()
# pass


def getAllBs():
    # session = getLocalSession("engine2.sqlite3")
    # types = session.query(B).all()
    # types_dict = [type.model_dump() for type in types]
    # return types_dict
    # return types
    a1 = A(foo="bar")
    a2 = A(foo="baz")
    b = B(bar="ham", a_s=[a1, a2])
    return [b]


class Query(ObjectType):
    bs = GraphField(GraphList(BNode))
    cs = GraphField(GraphList(CNode))

    def resolve_bs(self, info):
        return getAllBs()

    def resolve_cs(self, info):
        return [C(visible="foo"), C(visible="bar")]


class Mutation(ObjectType):
    create_b = CreateB.Field()


schema = Schema(
    query=Query,
    # mutation=Mutation,
)
with open("schema.graphql", "w", encoding="utf-8") as f:
    f.write(str(schema))

fastapi_app = FastAPI()


@fastapi_app.get("/types")
async def read_root() -> list[B]:
    return getAllBs()


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
