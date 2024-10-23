#!/usr/bin/env python

# semio-engine.py
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
import typing
import inspect
import argparse
import os
import logging  # for uvicorn in pyinstaller
import multiprocessing
import pathlib
import sqlite3
import sqlmodel
import sqlalchemy
import pydantic
import fastapi
import graphene
import graphene_pydantic
import graphene_sqlalchemy
import uvicorn
import starlette.applications
import starlette_graphene3
import semio

logging.basicConfig(level=logging.INFO)  # for uvicorn in pyinstaller

VERSION = "3.0.0"
HOST = "127.0.0.1"
PORT = 24111
GRAPHQLTYPES = {
    str: graphene.NonNull(graphene.String),
    int: graphene.NonNull(graphene.Int),
    float: graphene.NonNull(graphene.Float),
    bool: graphene.NonNull(graphene.Boolean),
    list[str]: graphene.NonNull(graphene.List(graphene.NonNull(graphene.String))),
    semio.Point: graphene.NonNull(lambda: PointNode),
    semio.Vector: graphene.NonNull(lambda: VectorNode),
    semio.Representation: graphene.NonNull(lambda: RepresentationNode),
    list[semio.Representation]: graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: RepresentationNode))
    ),
    semio.Port: graphene.NonNull(lambda: PortNode),
    list[semio.Port]: graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: PortNode))
    ),
    semio.Quality: graphene.NonNull(lambda: QualityNode),
    list[semio.Quality]: graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: QualityNode))
    ),
    semio.Type: graphene.NonNull(lambda: TypeNode),
    list[semio.Type]: graphene.NonNull(
        graphene.List(graphene.NonNull(lambda: TypeNode))
    ),
    # semio.Plane: graphene.NonNull(lambda: PlaneNode),
    # semio.PieceDiagram: graphene.NonNull(lambda: PieceDiagramNode),
    # semio.SidePieceType: graphene.NonNull(lambda: SidePieceTypeNode),
    # semio.SidePiece: graphene.NonNull(lambda: SidePieceNode),
    # semio.Side: graphene.NonNull(lambda: SideNode),
    # semio.Connection: graphene.NonNull(lambda: ConnectionNode),
    # semio.Design: graphene.NonNull(lambda: DesignNode),
    semio.Kit: graphene.NonNull(lambda: KitNode),
}
ENCODED_PATH = typing.Annotated[str, fastapi.Path(pattern=semio.ENCODING_REGEX)]


class NodeNode(graphene.relay.Node):

    class Meta:
        name = "Node"

    @staticmethod
    def to_global_id(type_, id):
        return id

    @staticmethod
    def get_node_from_global_id(info, global_id, only_type=None):
        entity = semio.entityByGuid(global_id)
        return entity


class RowNode(graphene_sqlalchemy.SQLAlchemyObjectType):
    """A base class for all graphql nodes.
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
        if "interfaces" not in options:
            options["interfaces"] = (NodeNode,)

        own_properties = [
            name
            for name, value in model.__dict__.items()
            if isinstance(value, property)
        ]

        def make_resolver(name):
            def resolver(self, info):
                return getattr(self, name)

            return resolver

        # Dynamically add resolvers for all properties
        for name in own_properties:
            prop = getattr(model, name)
            prop_getter = prop.fget
            prop_return_type = inspect.signature(prop_getter).return_annotation
            setattr(cls, name, GRAPHQLTYPES[prop_return_type])
            setattr(cls, f"resolve_{name}", make_resolver(name))

        def resolver_guid(self, info):
            return self.guid()

        setattr(cls, "resolve_id", resolver_guid)

        super().__init_subclass_with_meta__(model=model, **options)


class RepresentationNode(RowNode):
    class Meta:
        model = semio.Representation


class RepresentationInput(graphene_pydantic.PydanticInputObjectType):
    class Meta:
        model = semio.RepresentationSkeleton


class LocatorNode(RowNode):
    class Meta:
        model = semio.Locator


class LocatorInput(graphene_pydantic.PydanticInputObjectType):
    class Meta:
        model = semio.LocatorSkeleton


# class ScreenPointNode(graphene_pydantic.PydanticObjectType):
#     class Meta:
#         model = semio.ScreenPoint


# class ScreenPointInput(graphene_pydantic.PydanticInputObjectType):
#     class Meta:
#         model = semio.ScreenPoint


class PointNode(graphene_pydantic.PydanticObjectType):
    class Meta:
        model = semio.Point


class PointInput(graphene_pydantic.PydanticInputObjectType):
    class Meta:
        model = semio.Point


class VectorNode(graphene_pydantic.PydanticObjectType):
    class Meta:
        model = semio.Vector


class VectorInput(graphene_pydantic.PydanticInputObjectType):
    class Meta:
        model = semio.Vector


# class PlaneNode(graphene_pydantic.PydanticObjectType):
#     class Meta:
#         model = semio.Plane


# class PlaneInput(graphene_pydantic.PydanticInputObjectType):
#     class Meta:
#         model = semio.Plane


class PortNode(RowNode):
    class Meta:
        model = semio.Port


class PortInput(graphene_pydantic.PydanticInputObjectType):
    class Meta:
        model = semio.PortSkeleton


class PortIdInput(graphene_pydantic.PydanticInputObjectType):
    class Meta:
        model = semio.PortIdSkeleton


class QualityNode(RowNode):
    class Meta:
        model = semio.Quality


class QualityInput(graphene_pydantic.PydanticInputObjectType):
    class Meta:
        model = semio.QualitySkeleton


class TypeNode(RowNode):
    class Meta:
        model = semio.Type


class TypeInput(graphene_pydantic.PydanticInputObjectType):
    class Meta:
        model = semio.TypeSkeleton


# class PieceDiagramNode(graphene_pydantic.PydanticObjectType):
#     class Meta:
#         model = semio.PieceDiagram


# class PieceDiagramInput(graphene_pydantic.PydanticInputObjectType):
#     class Meta:
#         model = semio.PieceDiagram


# class PieceNode(RowNode):
#     class Meta:
#         model = semio.Piece

#     coordinateSystem = graphene.NonNull(PlaneNode)
#     diagram = graphene.NonNull(PieceDiagramNode)

#     def resolve_coordinateSystem(self, info):
#         return self.coordinateSystem

#     def resolve_diagram(self, info):
#         return self.diagram


# class PieceInput(graphene_pydantic.PydanticInputObjectType):
#     class Meta:
#         model = semio.PieceSkeleton


# class SidePieceTypeNode(graphene_pydantic.PydanticObjectType):
#     class Meta:
#         model = semio.SidePieceType
#         # port is none Pydanctic model and needs to be resolved manually
#         exclude_fields = ("port",)

#     port = graphene.Field(PortNode)

#     def resolve_port(type: semio.SidePieceType, info):
#         return type.port


# class SidePieceTypeInput(graphene_pydantic.PydanticInputObjectType):
#     class Meta:
#         model = semio.SidePieceTypeSkeleton


# class SidePieceNode(graphene_pydantic.PydanticObjectType):
#     class Meta:
#         model = semio.SidePiece


# class SidePieceInput(graphene_pydantic.PydanticInputObjectType):
#     class Meta:
#         model = semio.SidePieceSkeleton


# class SideNode(graphene_pydantic.PydanticObjectType):
#     class Meta:
#         model = semio.Side


# class SideInput(graphene_pydantic.PydanticInputObjectType):
#     class Meta:
#         model = semio.SideSkeleton


# class ConnectionNode(RowNode):
#     class Meta:
#         model = semio.Connection

#     connected = graphene.NonNull(SideNode)
#     connecting = graphene.NonNull(SideNode)

#     def resolve_connected(self, info):
#         return self.connected

#     def resolve_connecting(self, info):
#         return self.connecting


# class ConnectionInput(graphene_pydantic.PydanticInputObjectType):
#     class Meta:
#         model = semio.ConnectionSkeleton


# class DesignNode(RowNode):
#     class Meta:
#         model = semio.Design


# class DesignInput(graphene_pydantic.PydanticInputObjectType):
#     class Meta:
#         model = semio.DesignSkeleton


class KitNode(RowNode):
    class Meta:
        model = semio.Kit


# # Can't use SQLAlchemyConnectionField because only supports one database.
# # https://github.com/graphql-python/graphene-sqlalchemy/issues/180
# class KitConnection(graphene.relay.Connection):
#     url = graphene.String()

#     class Meta:
#         node = KitNode
#         # name = "KitConnection"


class KitInput(graphene_pydantic.PydanticInputObjectType):
    class Meta:
        model = semio.KitSkeleton


class Query(graphene.ObjectType):
    node = NodeNode.Field()
    kit = graphene.Field(KitNode, url=graphene.String(required=True))
    # kit = graphene_sqlalchemy.SQLAlchemyConnectionField(KitNode.connection)
    # kits = graphene.relay.ConnectionField(KitConnection)

    def resolve_kit(self, info, url):
        return semio.Kit.specific(url)


class Mutation(graphene.ObjectType):
    createKit = graphene.Field(KitNode, kit=KitInput(required=True))


def start_engine(debug: bool = False):

    rest = fastapi.FastAPI()

    @rest.get("/kits")
    async def kits(request: fastapi.Request) -> list[semio.KitSkeleton]:
        return semio.query("")

    @rest.get("/kits/{encodedKitUrl}")
    async def kit(
        encodedKitUrl: ENCODED_PATH,
        request: fastapi.Request,
    ) -> semio.KitSkeleton:
        return semio.query(request.url.path.removeprefix("/kits/"))

    @rest.get("/kits/{encodedKitUrl}/types")
    async def types(
        encodedKitUrl: ENCODED_PATH, request: fastapi.Request
    ) -> list[semio.TypeSkeleton]:
        return semio.query(request.url.path.removeprefix("/kits/"))

    @rest.get("/kits/{encodedKitUrl}/types/{encodedTypeName},{encodedTypeVariant}")
    async def type(
        encodedKitUrl: ENCODED_PATH,
        encodedTypeName: ENCODED_PATH,
        encodedTypeVariant: ENCODED_PATH,
        request: fastapi.Request,
    ) -> semio.TypeSkeleton:
        return semio.query(request.url.path.removeprefix("/kits/"))

    @rest.get(
        "/kits/{encodedKitUrl}/types/{encodedTypeName},{encodedTypeVariant}/representations"
    )
    async def representations(
        encodedKitUrl: ENCODED_PATH,
        encodedTypeName: ENCODED_PATH,
        encodedTypeVariant: ENCODED_PATH,
        request: fastapi.Request,
    ) -> list[semio.RepresentationSkeleton]:
        return semio.query(request.url.path.removeprefix("/kits/"))

    @rest.get(
        "/kits/{encodedKitUrl}/types/{encodedTypeName},{encodedTypeVariant}/representations/{encodedRepresentationUrl}"
    )
    async def representation(
        encodedKitUrl: ENCODED_PATH,
        encodedTypeName: ENCODED_PATH,
        encodedTypeVariant: ENCODED_PATH,
        encodedRepresentationUrl: ENCODED_PATH,
        request: fastapi.Request,
    ) -> semio.RepresentationSkeleton:
        return semio.query(request.url.path.removeprefix("/kits/"))

    schema = graphene.Schema(
        query=Query,
        mutation=Mutation,
    )

    if debug:

        def createDbAndTables():
            path = pathlib.Path("kit.sqlite3")
            try:
                os.remove(path)
            except:
                pass
            r1 = semio.Representation(
                url="https://app.speckle.systems/projects/e7de1a2f8f/models/b3c20db970"
            )
            # print(r1.guid())
            r2 = semio.Representation(
                url="https://app.speckle.systems/projects/e7de1a2f8f/models/6f52c1e6b1",
                lod="1to500",
            )
            r2.tags = ["tag1", "tag2"]
            r2.tags = ["tag3", "tag4", "tag5"]
            r3 = semio.Representation(
                id="3",
                url="https://app.speckle.systems/projects/e7de1a2f8f/models/45ab357369",
                lod="1to200",
            )
            t1 = semio.Type(name="Capsule")
            t1.representations = [r1, r2, r3]
            k1 = semio.Kit(name="Metabolism", url=str(path), types=[t1])
            print(r1.guid())
            engine = sqlalchemy.create_engine("sqlite:///" + str(path))
            sqlmodel.SQLModel.metadata.create_all(engine)
            with sqlalchemy.orm.Session(engine) as session:
                session.add(k1)
                [r1n, r2n, r3n] = t1.representations
                r2n.tags = ["volume"]
                session.commit()
                pass
            pass

        createDbAndTables()

        with open("../../graphql/schema.graphql", "w", encoding="utf-8") as f:
            f.write(str(schema))
        sqliteSchemaPath = "../../sqlite/schema.sql"
        if os.path.exists(sqliteSchemaPath):
            os.remove(sqliteSchemaPath)
        metadata_engine = sqlalchemy.create_engine("sqlite:///debug/semio.db")
        sqlmodel.SQLModel.metadata.create_all(metadata_engine)
        if not os.path.exists("debug"):
            os.makedirs("debug")
        extract_schema("debug/semio.db", "../../sqlite/schema.sql")

    engine = starlette.applications.Starlette()
    engine.mount(
        "/graphql",
        starlette_graphene3.GraphQLApp(
            schema, on_get=starlette_graphene3.make_graphiql_handler()
        ),
    )
    engine.mount("/", rest)

    uvicorn.run(
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
    parser = argparse.ArgumentParser()
    parser.add_argument("--debug", action="store_true", help="Enable debug mode")
    args = parser.parse_args()
    start_engine(args.debug)


if __name__ == "__main__":
    multiprocessing.freeze_support()  # needed for pyinstaller on Windows
    main()
