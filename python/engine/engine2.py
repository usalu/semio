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

from semio import RepresentationModel, TypeModel, getLocalSession

HOST = "127.0.0.1"
PORT = 5052


class RepresentationNode(SQLAlchemyObjectType):

    class Meta:
        model = RepresentationModel
        name = "Representation"
        exclude_fields = ("_tags",)

    tags = GraphNonNull(GraphList(GraphNonNull(GraphString)))

    def resolve_tags(representation: RepresentationModel, info):
        return representation.tags


class RepresentationInput(PydanticInputObjectType):

    class Meta:
        model = RepresentationModel


class TypeNode(SQLAlchemyObjectType):
    class Meta:
        model = TypeModel
        name = "Type"


class TypeInput(PydanticInputObjectType):
    class Meta:
        model = TypeModel


class TypesResponse(ObjectType):
    types = GraphList(TypeNode)


class Query(ObjectType):
    types = GraphField(TypesResponse)

    def resolve_types(self, info):
        session = getLocalSession("test.sqlite3")
        types = session.query(TypeModel).all()
        return TypesResponse(types=types)


class CreateType(graphene.Mutation):
    class Arguments:
        type = GraphNonNull(TypeInput)

    type = GraphField(TypeNode)

    def mutate(self, info, directory, type: TypeInput):
        session = getLocalSession("test.sqlite3")
        type = RepresentationModel(**type.dict())
        session.add(type)
        session.commit()
        return CreateType(type=type)


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
async def read_root() -> list[TypeModel]:
    session = getLocalSession("test.sqlite3")
    types = session.query(TypeModel).all()
    return types


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
