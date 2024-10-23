# semio.py
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
semio.
"""
# TODO: Check how to automate docstring duplication, table=True and PLURAL and __tablename__.
# class KitBase(sqlmodel.SQLModel):
#     '''🗃️ A kit is a collection of types and designs.'''
# class Kit(KitBase, Row, table=True):
#     '''🗃️ A kit is a collection of types and designs.'''

#     PLURAL = 'kits'
#     __tablename__ = 'kit'
# to:
# class KitBase(sqlmodel.SQLModel):
#     '''🗃️ A kit is a collection of types and designs.'''
# class Kit(KitBase, Row):


import abc
import base64
import enum
import os
import pathlib
import functools
import typing
import urllib
from datetime import datetime
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
import pint
import pydantic
import lark
import sqlalchemy
import sqlalchemy.orm
import sqlalchemy.ext.hybrid
import sqlmodel


RELEASE = "r24.11-1"
NAME_LENGTH_LIMIT = 64
ID_LENGTH_LIMIT = 128
URL_LENGTH_LIMIT = 1024
DESCRIPTION_LENGTH_LIMIT = 4096
ENCODING_REGEX = r"[a-zA-ZZ0-9_-]+={0,2}"
KIT_FOLDERNAME = ".semio"
KIT_FILENAME = "kit.sqlite3"
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

ureg = pint.UnitRegistry()


class SemioException(Exception):
    """❗ The base class for all exceptions in semio."""

    pass


class SpecificationError(SemioException):
    """🚫 The base class for all specification errors.
    A specification error is when the user input does not respect the specification."""

    pass


class NoParentAssigned(SpecificationError):
    """👪 The entity has no parent assigned."""

    pass


class NoRepresentationAssigned(NoParentAssigned):
    """👪 The entity has no representation assigned."""

    pass


class NoTypeAssigned(NoParentAssigned):
    """👪 The entity has no type assigned."""

    pass


class NoKitAssigned(NoParentAssigned):
    """👪 The entity has no kit assigned."""

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


class InvalidGuid(SemioException):
    """🆔 The guid is not valid. A guid looks like this:
    ENCODED_KIT_URL/types/ENCODED_TYPE_NAME,ENCODED_TYPE_VARIANT/..."""

    pass


class InvalidQuery(InvalidGuid):
    """🔍 The query is not valid. A query looks like this:
    ENCODED_KIT_URL/types/ENCODED_TYPE_NAME,ENCODED_TYPE_VARIANT/..."""

    pass


def encodeString(value: str) -> str:
    encoded_bytes = base64.urlsafe_b64encode(value.encode("utf-8"))
    encoded_str = encoded_bytes.decode("utf-8")
    return encoded_str


def decodeString(value: str) -> str:
    value += "=" * (-len(value) % 4)
    decoded_bytes = base64.urlsafe_b64decode(value.encode("utf-8"))
    decoded_str = decoded_bytes.decode("utf-8")
    return decoded_str


queryGrammar = r"""
    query: (ENCODED_STRING)? ("/" (type | representation | port))?
    type: "types" ("/" ENCODED_STRING "," ENCODED_STRING?)?
    representation: type "/representations" ("/" ENCODED_STRING)?
    port: type "/ports" ("/" ENCODED_STRING)?
    ENCODED_STRING: /[a-zA-ZZ0-9_-]+={0,2}/
"""
queryParser = lark.Lark(queryGrammar, start="query")


class QueryBuilder(lark.Transformer):
    # E.g:
    # a2l0LnNxbGl0ZTM=
    # a2l0LnNxbGl0ZTM=/types
    # a2l0LnNxbGl0ZTM=/types/Q2Fwc3VsZQ==,
    # a2l0LnNxbGl0ZTM=/types/Q2Fwc3VsZQ==,/representations
    # a2l0LnNxbGl0ZTM=/types/Q2Fwc3VsZQ==,/representations/aHR0cHM6Ly9hcHAuc3BlY2tsZS5zeXN0ZW1zL3Byb2plY3RzL2U3ZGUxYTJmOGYvbW9kZWxzL2IzYzIwZGI5NzA=
    # kit.sqlite3/types/Capsule,/representations/https://app.speckle.systems/projects/e7de1a2f8f/models/b3c20db970
    # url: kit.sqlite3
    # kind: type
    # typeName: Capsule
    # typeVariant: ""
    # representationUrl: https://app.speckle.systems/projects/e7de1a2f8f/models/b3c20db970

    def query(self, children):
        if len(children) == 0:
            return {"kind": "kits"}
        kitUrl = decodeString(children[0].value)
        if len(children) == 1:
            return {"kind": "kit", "kitUrl": kitUrl}
        query = children[1]
        query["kitUrl"] = kitUrl
        return query

    def type(self, children):
        if len(children) == 0:
            return {"kind": "types"}
        return {
            "kind": "type",
            "typeName": decodeString(children[0].value),
            "typeVariant": (
                decodeString(children[1].value) if len(children) == 2 else ""
            ),
        }

    def representation(self, children):
        type = children[0]
        query = {
            "typeName": type["typeName"],
            "typeVariant": type["typeVariant"],
        }
        if len(children) == 1:
            query["kind"] = "representations"
        else:
            query["kind"] = "representation"
            query["representationUrl"] = decodeString(children[1].value)

        return query

    def port(self, children):
        type = children[0]
        query = {
            "typeName": type["typeName"],
            "typeVariant": type["typeVariant"],
        }
        if len(children) == 1:
            query["kind"] = "ports"
        else:
            query["kind"] = "port"
            query["portUrl"] = decodeString(children[1].value)
        return query


class StoreKind(enum.Enum):
    """🏪 The kind of the store."""

    DATABASE = "database"
    REST = "rest"
    GRAPHQL = "graphql"


class Store(abc.ABC):

    @classmethod
    @abc.abstractmethod
    def query(cls: "Store", query: str) -> typing.Any:
        """🔍 Query the store. Outputs are either entities or collections of entities."""
        pass


class DatabaseStore(Store, abc.ABC):

    @classmethod
    @abc.abstractmethod
    def sessionByUrl(cls: "DatabaseStore", url: str) -> sqlalchemy.orm.Session:
        """🔧 Get a session from the url."""
        pass

    @classmethod
    @abc.abstractmethod
    def kitNameByUrl(cls: "DatabaseStore", url: str) -> str:
        """📛 Get the name of the kit from the url."""
        pass

    @classmethod
    def query(cls: "DatabaseStore", query: str) -> typing.Any:
        queryTree = queryParser.parse(query)
        query = QueryBuilder().transform(queryTree)
        kitUrl = query["kitUrl"]
        kind = query["kind"]
        match kind:
            case "kit":
                return Kit.specific(kitUrl)
            case "kits":
                return Kit.all()
            case "type":
                return Type.specific(kitUrl, query["typeName"], query["typeVariant"])
            case "types":
                return Type.all(kitUrl)
            case "representation":
                return Representation.specific(
                    kitUrl,
                    query["typeName"],
                    query["typeVariant"],
                    query["representationUrl"],
                )
            case "representations":
                return Representation.all(
                    kitUrl,
                    query["typeName"],
                    query["typeVariant"],
                )

            case _:
                raise InvalidBackend(f"Unknown kind: {kind}")


class SSLMode(enum.Enum):
    """🔒 The security level of the session"""

    DISABLE = "disable"
    ALLOW = "allow"
    PREFER = "prefer"
    REQUIRE = "require"
    VERIFY_CA = "verify-ca"
    VERIFY_FULL = "verify-full"


class SqliteStore(DatabaseStore):

    @classmethod
    def sessionByUrl(cls, url: str):
        parsedUrl = urllib.parse.urlparse(url)
        if not parsedUrl.path.endswith(".sqlite3"):
            if not parsedUrl.path.endswith(".semio"):
                path = pathlib.Path(parsedUrl.path) / KIT_FOLDERNAME / KIT_FILENAME
            else:
                path = pathlib.Path(parsedUrl.path) / KIT_FILENAME
        else:
            path = pathlib.Path(parsedUrl.path)
        path = str(path)
        connectionString = (
            url
            if parsedUrl.scheme == "sqlite"
            else f"sqlite:///{path}" if not path.startswith("/") else f"sqlite://{path}"
        )
        engine = sqlalchemy.create_engine(connectionString, echo=True)
        sqlmodel.SQLModel.metadata.create_all(engine)
        session = sqlalchemy.orm.sessionmaker(bind=engine)()
        return session

    @classmethod
    def kitNameByUrl(cls: "SqliteStore", url: str) -> str:
        session = cls.sessionByUrl(url)
        return session.query(Kit).one_or_none().name


class PostgresStore(DatabaseStore):

    @classmethod
    def sessionByUrl(cls, url: str):
        parsedUrl = urllib.parse.urlparse(url)
        connection_string = sqlalchemy.URL.create(
            "postgresql+psycopg",
            username=parsedUrl.username,
            password=parsedUrl.password,
            host=parsedUrl.hostname,
            database=parsedUrl.path[1:],  # Remove the leading '/'
        )
        engine = sqlalchemy.create_engine(
            connection_string,
            connect_args={"sslmode": parsedUrl.query.get("sslmode", SSLMode.REQUIRE)},
        )
        sqlmodel.SQLModel.metadata.create_all(engine)
        session = sqlalchemy.orm.sessionmaker(bind=engine)()
        return session

    @classmethod
    def kitNameByUrl(cls: "PostgresStore", url: str) -> str:
        raise NotImplementedError()


# class ApiStore(Store, abc.ABC):

#     @classmethod
#     @abc.abstractmethod
#     def kit(cls: "ApiStore", url: str) -> "Kit":
#         """📦 Get a kit from the url."""
#         pass

#     @classmethod
#     def query(cls, url: str, body={}) -> "Row":
#         # TODO: Implement
#         # fetch kit
#         # cache kit locally under encoded url
#         # run SQLiteStore.query
#         raise NotImplementedError()


# class RestStore(ApiStore):

#     @classmethod
#     def kit(cls, url: str) -> "Kit":
#         raise NotImplementedError()


# class GraphqlStore(Store):

#     @classmethod
#     def kit(cls, url: str) -> "Kit":
#         raise NotImplementedError()


def query(query: str) -> typing.Any:
    try:
        return SqliteStore.query(query)
    except Exception as e:
        pass
    try:
        return PostgresStore.query(query)
    except Exception as e:
        pass
    # try:
    #     return RestStore.query(query)
    # except Exception as e:
    #     pass
    # try:
    #     return GraphqlStore.query(query)
    # except Exception as e:
    #     pass
    raise InvalidQuery(query)


def sessionByUrl(url: str) -> sqlalchemy.orm.Session:
    """🔧 Get a session from the url."""
    try:
        return SqliteStore.sessionByUrl(url)
    except Exception as e:
        pass
    try:
        return PostgresStore.sessionByUrl(url)
    except Exception as e:
        pass
    # try:
    #     return RestStore.sessionByUrl(url)
    # except Exception as e:
    #     pass
    # try:
    #     return GraphqlStore.sessionByUrl(url)
    # except Exception as e:
    #     pass
    raise InvalidURL(url)


def kitNameByUrl(url: str) -> str:
    """📛 Get the name of the kit from the url."""
    try:
        return SqliteStore.kitNameByUrl(url)
    except Exception as e:
        pass
    try:
        return PostgresStore.kitNameByUrl(url)
    except Exception as e:
        pass
    # try:
    #     return RestStore.kitNameByUrl(url)
    # except Exception as e:
    #     pass
    # try:
    #     return GraphqlStore.kitNameByUrl(url)
    # except Exception as e:
    #     pass
    raise InvalidURL(url)


class Semio(sqlmodel.SQLModel):
    """ℹ️ Metadata about the semio database."""

    __tablename__ = "semio"

    release: str = sqlmodel.Field(default=RELEASE, primary_key=True)
    createdAt: datetime = sqlmodel.Field(default_factory=datetime.now)


class Row(sqlmodel.SQLModel):
    """Base class for all rows in semio."""

    PLURAL: typing.ClassVar[str]
    """🔢 The plural of the entity."""

    def parent(self) -> typing.Optional["Row"]:
        """👪 The parent of the entity."""
        return None

    # @abc.abstractmethod
    def localId(self, encode: bool = False) -> tuple:
        """🆔 A tuple that identifies the entity within it's parent."""
        pass

    def humanId(self) -> str:
        """🪪 A string that let's the user identify the entity within it's parent."""
        return f"{self.__class__.__name__}({", ".join(self.localId())})"

    def guid(self) -> str:
        """🆔 A guid that let's relay identify the entity."""
        localId = f"{self.__class__.PLURAL.lower()}/{self.localId(encode=True)}"
        parent = self.parent()
        parentId = f"{parent.guid()}/" if parent is not None else ""
        return parentId + localId

    # @property
    # def id(self) -> str:
    #     return self.guid()


class Skeleton(sqlmodel.SQLModel):
    class Config:
        title = __name__[:8]  # len('Skeleton')=8 E.g. 'PieceSkeleton'->'Piece'


class IdentifiedRow(Row):
    id_: str = sqlmodel.Field(alias="id")

    def localId(self, encode: bool = False) -> str:
        return encodeString(self.id_) if encode else self.id_


class UrledRow(Row):
    url: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT)

    def localId(self, encode: bool = False) -> str:
        return encodeString(self.url) if encode else self.url


class NamedRow(Row):
    name: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT)

    def localId(self, encode: bool = False) -> str:
        return encodeString(self.name) if encode else self.name


class ArtifactRow(NamedRow):
    """♻️ An artifact is anything that is worth to be reused."""

    # Optional. Set to '' for None.
    description: str = sqlmodel.Field(max_length=DESCRIPTION_LENGTH_LIMIT, default="")
    # Optional. Set to '' for None.
    icon: str = sqlmodel.Field(default="", max_length=URL_LENGTH_LIMIT)
    createdAt: datetime = sqlmodel.Field(default_factory=datetime.now)
    lastUpdateAt: datetime = sqlmodel.Field(default_factory=datetime.now)


class VariableArtifactRow(ArtifactRow):
    """🎚️ A variable artifact is an artifact that has variants (at least one default)."""

    variant: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT, default="")

    def localId(self, encode: bool = False) -> str:
        return f"{super().localId(encode)},{(encodeString(self.variant) if encode else self.variant)}"


class Tag(sqlmodel.SQLModel, table=True):
    """🏷️ A tag is meta-data for grouping representations."""

    # __tablename__ = 'tag'
    value: str = sqlmodel.Field(primary_key=True)
    representationPk: int = sqlmodel.Field(
        alias="representationId",
        sa_column=sqlmodel.Column(
            "representationId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("representation.id"),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    representation: "Representation" = sqlmodel.Relationship(back_populates="_tags")

    def parent(self) -> typing.Optional["Representation"]:
        if self.representation is None:
            raise NoRepresentationAssigned()
        return self.representation

    def localId(self, encode: bool = False) -> tuple:
        return (encodeString(self.value) if encode else self.value,)


class RepresentationBase(UrledRow):
    lod: str = ""


class Representation(RepresentationBase, table=True):
    """💾 A representation is a link to a file that describes a type for a unique combination of level of detail, tags and mime."""

    PLURAL = "representations"
    __tablename__ = "representation"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    _tags: list[Tag] = sqlmodel.Relationship(
        back_populates="representation", cascade_delete=True
    )
    typePk: typing.Optional[int] = sqlmodel.Field(
        alias="typeId",
        sa_column=sqlmodel.Column(
            "typeId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    type: typing.Optional["Type"] = sqlmodel.Relationship(
        back_populates="representations"
    )

    @property
    def tags(self) -> list[str]:
        return [tag.value for tag in self._tags or []]

    @tags.setter
    def tags(self, tags: list[str]):
        self._tags = [Tag(value=tag) for tag in tags]

    def parent(self) -> "Type":
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    @classmethod
    def specific(
        cls: "Representation",
        kitUrl: str,
        typeName: str,
        typeVariant: str,
        representationUrl: str,
    ):
        session = sessionByUrl(kitUrl)
        kitName = kitNameByUrl(kitUrl)
        return (
            session.query(Representation, Type, Kit)
            .filter(
                Kit.name == kitName,
                Type.name == typeName,
                Type.variant == typeVariant,
                Representation.url == representationUrl,
            )
            .one_or_none()
            .Representation
        )

    @classmethod
    def all(
        cls: "Representation",
        kitUrl: str,
        typeName: str,
        typeVariant: str,
    ) -> list["Representation"]:
        session = sessionByUrl(kitUrl)
        kitName = kitNameByUrl(kitUrl)
        return [
            r.Representation
            for r in session.query(Representation, Type, Kit)
            .filter(
                Kit.name == kitName,
                Type.name == typeName,
                Type.variant == typeVariant,
            )
            .all()
        ]


class RepresentationSkeleton(RepresentationBase):
    class Config:
        title = "Representation"

    tags: list[str] = sqlmodel.Field(default_factory=list)


class LocatorBase(Row):
    # Optional. '' means true.
    subgroup: str = sqlmodel.Field(default="", max_length=NAME_LENGTH_LIMIT)


class Locator(LocatorBase, table=True):
    """🗺️ A locator is meta-data for grouping ports."""

    PLURAL = "locators"
    __tablename__ = "locator"
    group: str = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "groupName",
            sqlalchemy.String(NAME_LENGTH_LIMIT),
            primary_key=True,
        ),
    )
    portPk: typing.Optional[int] = sqlmodel.Field(
        alias="portId",
        sa_column=sqlmodel.Column(
            "portId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("port.id"),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    port: typing.Optional["Port"] = sqlmodel.Relationship(back_populates="locators")


class LocatorSkeleton(LocatorBase):
    class Config:
        title = "Locator"


def prettyNumber(number: float) -> str:
    if number == -0.0:
        number = 0.0
    return f"{number:.5f}".rstrip("0").rstrip(".")


# class ScreenPoint(sqlmodel.SQLModel):
#     """📺 A 2d-point (xy) of integers in screen coordinate system."""

#     x: int = 0
#     y: int = 0

#     def __init__(self, x: int = 0, y: int = 0):
#         super().__init__(x=x, y=y)

#     def __len__(self):
#         return 2

#     def __getitem__(self, key):
#         if key == 0:
#             return self.x
#         elif key == 1:
#             return self.y
#         else:
#             raise IndexError("Index out of range")

#     def __iter__(self):
#         return iter((self.x, self.y))


class Point(sqlmodel.SQLModel):
    """✖️ A 3d-point (xyz) of floating point numbers."""

    x: float = 0.0
    y: float = 0.0
    z: float = 0.0

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return (
            f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"
        )

    def __repr__(self) -> str:
        return (
            f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"
        )

    def __len__(self):
        return 3

    def __getitem__(self, key):
        if key == 0:
            return self.x
        elif key == 1:
            return self.y
        elif key == 2:
            return self.z
        else:
            raise IndexError("Index out of range")

    def __iter__(self):
        return iter((self.x, self.y, self.z))

    def isCloseTo(self, other: "Point", tol: float = TOLERANCE) -> bool:
        return (
            abs(self.x - other.x) < tol
            and abs(self.y - other.y) < tol
            and abs(self.z - other.z) < tol
        )

    def transform(self, transform: "Transform") -> "Point":
        return Transform.transformPoint(transform, self)

    def toVector(self) -> "Vector":
        return Vector(self.x, self.y, self.z)


class Vector(sqlmodel.SQLModel):
    """➡️ A 3d-vector (xyz) of floating point numbers."""

    x: float = 0.0
    y: float = 0.0
    z: float = 0.0

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return (
            f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"
        )

    def __repr__(self) -> str:
        return (
            f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"
        )

    def __len__(self):
        return 3

    def __getitem__(self, key):
        if key == 0:
            return self.x
        elif key == 1:
            return self.y
        elif key == 2:
            return self.z
        else:
            raise IndexError("Index out of range")

    def __iter__(self):
        return iter((self.x, self.y, self.z))

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y, self.z + other.z)

    @property
    def length(self) -> float:
        return (self.x**2 + self.y**2 + self.z**2) ** 0.5

    def revert(self) -> "Vector":
        return Vector(-self.x, -self.y, -self.z)

    def amplify(self, factor: float) -> "Vector":
        return Vector(self.x * factor, self.y * factor, self.z * factor)

    def isCloseTo(self, other: "Vector", tol: float = TOLERANCE) -> bool:
        return (
            abs(self.x - other.x) < tol
            and abs(self.y - other.y) < tol
            and abs(self.z - other.z) < tol
        )

    def normalize(self) -> "Vector":
        length = self.length
        return Vector(x=self.x / length, y=self.y / length, z=self.z / length)

    def dot(self, other: "Vector") -> float:
        return dot(self, other)

    def cross(self, other: "Vector") -> "Vector":
        return Vector(*cross(self, other))

    def transform(self, transform: "Transform") -> "Vector":
        return Transform.transformVector(transform, self)

    def toPoint(self) -> "Point":
        return Point(self.x, self.y, self.z)

    def toTransform(self) -> "Transform":
        return Transform.fromTranslation(self)

    @staticmethod
    def X() -> "Vector":
        return Vector(x=1)

    @staticmethod
    def Y() -> "Vector":
        return Vector(y=1)

    @staticmethod
    def Z() -> "Vector":
        return Vector(z=1)


# class PlaneBase(sqlmodel.SQLModel):

#     planeOriginX: float = sqlmodel.Field(exclude=True)
#     planeOriginY: float = sqlmodel.Field(exclude=True)
#     planeOriginZ: float = sqlmodel.Field(exclude=True)
#     xAxisX: float = sqlmodel.Field(exclude=True)
#     xAxisY: float = sqlmodel.Field(exclude=True)
#     xAxisZ: float = sqlmodel.Field(exclude=True)
#     yAxisX: float = sqlmodel.Field(exclude=True)
#     yAxisY: float = sqlmodel.Field(exclude=True)
#     yAxisZ: float = sqlmodel.Field(exclude=True)

#     def __init__(
#         self, origin: Point = None, xAxis: Vector = None, yAxis: Vector = None
#     ):
#         if origin is None:
#             origin = Point()
#         if xAxis is None and yAxis is None:
#             xAxis = Vector.X()
#             yAxis = Vector.Y()
#         if xAxis is None:
#             xAxis = Vector()
#         if yAxis is None:
#             yAxis = Vector()
#         if abs(xAxis.length - 1) > TOLERANCE:
#             raise ValidationError("The x-axis must be normalized.")
#         if abs(yAxis.length - 1) > TOLERANCE:
#             raise ValidationError("The y-axis must be normalized.")
#         if abs(xAxis.dot(yAxis)) > TOLERANCE:
#             raise ValidationError("The x-axis and y-axis must be orthogonal.")
#         super().__init__(origin=origin, xAxis=xAxis, yAxis=yAxis)

#     @property
#     def origin(self) -> Point:
#         return Point(
#             self.planeOriginX,
#             self.planeOriginY,
#             self.planeOriginZ,
#         )

#     @property
#     def xAxis(self) -> Vector:
#         return Vector(
#             self.xAxisX,
#             self.xAxisY,
#             self.xAxisZ,
#         )

#     @property
#     def zAxis(self) -> Vector:
#         return self.xAxis.cross(self.yAxis)

#     def isCloseTo(self, other: "Plane", tol: float = TOLERANCE) -> bool:
#         return (
#             self.origin.isCloseTo(other.origin, tol)
#             and self.xAxis.isCloseTo(other.xAxis, tol)
#             and self.yAxis.isCloseTo(other.yAxis, tol)
#         )

#     def transform(self, transform: "Transform") -> "Plane":
#         return Transform.transformPlane(transform, self)

#     def toTransform(self) -> "Transform":
#         return Transform.fromPlane(self)

#     @staticmethod
#     def XY() -> "Plane":
#         return Plane(
#             origin=Point(),
#             xAxis=Vector.X(),
#             yAxis=Vector.Y(),
#         )

#     @staticmethod
#     def fromYAxis(yAxis: Vector, theta: float = 0.0, origin: Point = None) -> "Plane":
#         if abs(yAxis.length - 1) > TOLERANCE:
#             raise SpecificationError("The yAxis must be normalized.")
#         if origin is None:
#             origin = Point()
#         orientation = Transform.fromDirections(Vector.Y(), yAxis)
#         rotation = Transform.fromAngle(yAxis, theta)
#         xAxis = Vector.X().transform(rotation.after(orientation))
#         return Plane(origin=origin, xAxis=xAxis, yAxis=yAxis)


# class Plane(PlaneBase, Row, table=True):
#     """◳ A coordinate system is an origin (point) and an orientation (x-axis and y-axis)."""

#     PLURAL = "planes"
#     __tablename__ = "plane"

#     pk: typing.Optional[int] = sqlmodel.Field(
#         sa_column=sqlmodel.Column(
#             "id",
#             sqlalchemy.Integer(),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     piece: typing.Optional["Piece"] = sqlmodel.Relationship(back_populates="plane")


# class Rotation(sqlmodel.SQLModel):
#     """🔄 A rotation is an axis and an angle."""

#     axis: Vector
#     angle: float

#     def __init__(self, axis: Vector, angle: float):
#         super().__init__(axis=axis, angle=angle)

#     def toTransform(self) -> "Transform":
#         return Transform.fromRotation(self)


# class Transform(ndarray):
#     """▦ A 4x4 translation and rotation transformation matrix (no scaling or shearing)."""

#     def __new__(cls, input_array=None):
#         if input_array is None:
#             input_array = eye(4, dtype=float)
#         else:
#             input_array = asarray(input_array).astype(float)
#         obj = input_array.view(cls)
#         return obj

#     def __array_finalize__(self, obj):
#         if obj is None:
#             return

#     def __str__(self) -> str:
#         rounded_self = self.round()
#         return f"Transform(Rotation={rounded_self.rotation}, Translation={rounded_self.translation})"

#     def __repr__(self) -> str:
#         rounded_self = self.round()
#         return f"Transform(Rotation={rounded_self.rotation}, Translation={rounded_self.translation})"

#     @property
#     def rotation(self) -> Rotation | None:
#         """🔄 The rotation part of the transform."""
#         rotationMatrix = self[:3, :3]
#         axisAngle = axis_angle_from_matrix(rotationMatrix)
#         if axisAngle[3] == 0:
#             return None
#         return Rotation(
#             axis=Vector(float(axisAngle[0]), float(axisAngle[1]), float(axisAngle[2])),
#             angle=float(degrees(axisAngle[3])),
#         )

#     @property
#     def translation(self) -> Vector:
#         """➡️ The translation part of the transform."""
#         return Vector(*self[:3, 3])

#     # for pydantic
#     def dict(self) -> typing.Dict[str, typing.Union[Rotation, Vector]]:
#         return {
#             "rotation": self.rotation,
#             "translation": self.translation,
#         }

#     def after(self, before: "Transform") -> "Transform":
#         """✖️ Apply this transform after another transform.

#         Args:
#             before (Transform): Transform to apply before this transform.

#         Returns:
#             Transform: New transform.
#         """
#         return Transform(concat(before, self))

#     def invert(self) -> "Transform":
#         return Transform(invert_transform(self))

#     def transformPoint(self, point: Point) -> Point:
#         transformedPoint = transform(self, vector_to_point(point))
#         return Point(*transformedPoint[:3])

#     def transformVector(self, vector: Vector) -> Vector:
#         transformedVector = transform(self, vector_to_direction(vector))
#         return Vector(*transformedVector[:3])

#     def transformPlane(self, plane: Plane) -> Plane:
#         planeTransform = Transform.fromPlane(plane)
#         planeTransformed = planeTransform.after(self)
#         return Transform.toPlane(planeTransformed)

#     def transform(
#         self, geometry: typing.Union[Point, Vector, Plane]
#     ) -> typing.Union[Point, Vector, Plane]:
#         if isinstance(geometry, Point):
#             return self.transformPoint(geometry)
#         elif isinstance(geometry, Vector):
#             return self.transformVector(geometry)
#         elif isinstance(geometry, Plane):
#             return self.transformPlane(geometry)
#         else:
#             raise NotImplementedError()

#     def round(self, decimals: int = SIGNIFICANT_DIGITS) -> "Transform":
#         return Transform(super().round(decimals=decimals))

#     @staticmethod
#     def fromTranslation(vector: Vector) -> "Transform":
#         return Transform(
#             transform_from(
#                 [
#                     [1, 0, 0],
#                     [0, 1, 0],
#                     [0, 0, 1],
#                 ],
#                 vector,
#             )
#         )

#     @staticmethod
#     def fromRotation(rotation: Rotation) -> "Transform":
#         return Transform(
#             transform_from(
#                 matrix_from_axis_angle((*rotation.axis, radians(rotation.angle))),
#                 Vector(),
#             )
#         )

#     @staticmethod
#     def fromPlane(plane: Plane) -> "Transform":
#         # Assumes plane is normalized
#         return Transform(
#             transform_from(
#                 [
#                     [
#                         plane.xAxis.x,
#                         plane.yAxis.x,
#                         plane.zAxis.x,
#                     ],
#                     [
#                         plane.xAxis.y,
#                         plane.yAxis.y,
#                         plane.zAxis.y,
#                     ],
#                     [
#                         plane.xAxis.z,
#                         plane.yAxis.z,
#                         plane.zAxis.z,
#                     ],
#                 ],
#                 plane.origin,
#             )
#         )

#     @staticmethod
#     def fromAngle(axis: Vector, angle: float) -> "Transform":
#         return Transform(
#             transform_from(matrix_from_axis_angle((*axis, radians(angle))), Vector())
#         )

#     @staticmethod
#     def fromDirections(startDirection: Vector, endDirection: Vector) -> "Transform":
#         if startDirection.isCloseTo(endDirection):
#             return Transform()
#         axisAngle = axis_angle_from_two_directions(startDirection, endDirection)
#         return Transform(transform_from(matrix_from_axis_angle(axisAngle), Vector()))

#     def toPlane(self) -> Plane:
#         return Plane(
#             origin=Point(*self[:3, 3]),
#             xAxis=Vector(
#                 self[0, 0],
#                 self[1, 0],
#                 self[2, 0],
#             ),
#             yAxis=Vector(
#                 self[0, 1],
#                 self[1, 1],
#                 self[2, 1],
#             ),
#         )


class PortBase(IdentifiedRow):
    """🔌 A port is a connection point (with a direction) of a type."""

    pass


class Port(PortBase, table=True):
    """🔌 A port is a connection point (with a direction) of a type."""

    PLURAL = "ports"
    __tablename__ = "port"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
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
            sqlalchemy.String(NAME_LENGTH_LIMIT),
        ),
    )
    pointX: float = sqlmodel.Field(exclude=True)
    pointY: float = sqlmodel.Field(exclude=True)
    pointZ: float = sqlmodel.Field(exclude=True)
    directionX: float = sqlmodel.Field(exclude=True)
    directionY: float = sqlmodel.Field(exclude=True)
    directionZ: float = sqlmodel.Field(exclude=True)
    typePk: typing.Optional[int] = sqlmodel.Field(
        alias="typeId",
        sa_column=sqlmodel.Column(
            "typeId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="ports")
    locators: list[Locator] = sqlmodel.Relationship(
        back_populates="port", cascade_delete=True
    )
    # connecteds: list["Connection"] = sqlmodel.Relationship(
    #     back_populates="connectedPieceTypePort",
    #     sa_relationship_kwargs={"foreign_keys": "Connection.connectedPieceTypePortPk"},
    # )
    # connectings: list["Connection"] = sqlmodel.Relationship(
    #     back_populates="connectingPieceTypePort",
    #     sa_relationship_kwargs={"foreign_keys": "Connection.connectingPieceTypePortPk"},
    # )

    __table_args__ = (sqlalchemy.UniqueConstraint("localId", "typeId"),)

    @property
    def point(self) -> Point:
        return Point(self.pointX, self.pointY, self.pointZ)

    @point.setter
    def point(self, point: Point):
        self.pointX = point.x
        self.pointY = point.y
        self.pointZ = point.z

    @property
    def direction(self) -> Vector:
        return Vector(self.directionX, self.directionY, self.directionZ)

    @direction.setter
    def direction(self, direction: Vector):
        self.directionX = direction.x
        self.directionY = direction.y
        self.directionZ = direction.z

    def parent(self) -> "Type":
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    @classmethod
    def all(
        cls: "Port",
        kitUrl: str,
        typeName: str,
        typeVariant: str,
    ) -> list["Port"]:
        session = sessionByUrl(kitUrl)
        kitName = kitNameByUrl(kitUrl)
        return [
            p.Port
            for p in session.query(Port, Type, Kit)
            .filter(
                Kit.name == kitName,
                Type.name == typeName,
                Type.variant == typeVariant,
            )
            .all()
        ]

    @classmethod
    def specific(
        cls: "Port",
        kitUrl: str,
        typeName: str,
        typeVariant: str,
        portId: str,
    ):
        session = sessionByUrl(kitUrl)
        kitName = kitNameByUrl(kitUrl)
        return (
            session.query(Port, Type, Kit)
            .filter(
                Kit.name == kitName,
                Type.name == typeName,
                Type.variant == typeVariant,
                Port.id_ == portId,
            )
            .one_or_none()
        )


class PortSkeleton(PortBase):
    class Config:
        title = "Port"

    id_: str = sqlmodel.Field(alias="id")
    point: Point = sqlmodel.Field()
    direction: Vector = sqlmodel.Field()
    locators: list[LocatorSkeleton] = sqlmodel.Field(default_factory=list)


class PortIdSkeleton(PortBase):
    pass


class QualityBase(NamedRow):

    # Optional. '' means true.
    value: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT, default="")
    # Optional. Set to '' for None.
    definition: str = sqlmodel.Field(max_length=DESCRIPTION_LENGTH_LIMIT, default="")
    # Optional. Set to '' for None.
    unit: str = sqlmodel.Field(max_length=NAME_LENGTH_LIMIT, default="")


class Quality(QualityBase, table=True):
    """📏 A quality is meta-data for decision making."""

    PLURAL = "qualities"
    __tablename__ = "quality"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    typePk: typing.Optional[int] = sqlmodel.Field(
        alias="typeId",
        sa_column=sqlmodel.Column(
            "typeId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("type.id"),
        ),
        default=None,
        exclude=True,
    )
    type: typing.Optional["Type"] = sqlmodel.Relationship(back_populates="qualities")
    # designPk: typing.Optional[int] = sqlmodel.Field(
    #     alias="designId",
    #     sa_column=sqlmodel.Column(
    #         "designId",
    #         sqlalchemy.Integer(),
    #         sqlalchemy.ForeignKey("design.id"),
    #     ),
    #     default=None,
    #     exclude=True,
    # )
    # design: typing.Optional["Design"] = sqlmodel.Relationship(
    #     back_populates="qualities"
    # )
    # __table_args__ = (
    #     sqlalchemy.CheckConstraint(
    #         "typeId IS NOT NULL AND designId IS NULL OR typeId IS NULL AND designId IS NOT NULL",
    #         name="typeOrDesignSet",
    #     ),
    #     sqlalchemy.UniqueConstraint("name", "typeId", "designId"),
    # )

    def parent(self) -> "Type":
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    @classmethod
    def all(
        cls: "Quality",
        kitUrl: str,
        typeName: str,
        typeVariant: str,
    ) -> list["Quality"]:
        session = sessionByUrl(kitUrl)
        kitName = kitNameByUrl(kitUrl)
        return [
            q.Quality
            for q in session.query(Quality, Type, Kit)
            .filter(
                Kit.name == kitName,
                Type.name == typeName,
                Type.variant == typeVariant,
            )
            .all()
        ]

    @classmethod
    def specific(
        cls: "Quality",
        kitUrl: str,
        typeName: str,
        typeVariant: str,
        qualityName: str,
    ):
        session = sessionByUrl(kitUrl)
        kitName = kitNameByUrl(kitUrl)
        return (
            session.query(Quality, Type, Kit)
            .filter(
                Kit.name == kitName,
                Type.name == typeName,
                Type.variant == typeVariant,
                Quality.name == qualityName,
            )
            .one_or_none()
        )


class QualitySkeleton(QualityBase):
    class Config:
        title = "Quality"

    pass


class TypeBase(VariableArtifactRow):
    pass


class Type(TypeBase, Row, table=True):
    """🧩 A type is a reusable element that can be connected with other types over ports."""

    PLURAL = "types"
    __tablename__ = "type"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    representations: list[Representation] = sqlmodel.Relationship(
        back_populates="type",
        cascade_delete=True,
    )
    ports: list[Port] = sqlmodel.Relationship(
        back_populates="type", cascade_delete=True
    )
    qualities: list[Quality] = sqlmodel.Relationship(
        back_populates="type", cascade_delete=True
    )
    kitPk: typing.Optional[int] = sqlmodel.Field(
        alias="kitId",
        sa_column=sqlmodel.Column(
            "kitId",
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey("kit.id"),
        ),
        default=None,
        exclude=True,
    )
    kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="types")

    # __table_args__ = (sqlalchemy.UniqueConstraint('name', 'variant', 'kitPk'),)

    def parent(self) -> "Kit":
        if self.kit is None:
            raise NoKitAssigned()
        return self.kit

    @classmethod
    def all(cls: "Type", kitUrl: str) -> list["Type"]:
        session = sessionByUrl(kitUrl)
        kitName = kitNameByUrl(kitUrl)
        return [
            t.Type
            for t in session.query(Type, Kit)
            .filter(
                Kit.name == kitName,
            )
            .all()
        ]

    @classmethod
    def specific(
        cls: "Type",
        kitUrl: str,
        typeName: str,
        typeVariant: str,
    ):
        session = sessionByUrl(kitUrl)
        kitName = kitNameByUrl(kitUrl)
        return (
            session.query(Type, Kit)
            .filter(
                Kit.name == kitName,
                Type.name == typeName,
                Type.variant == typeVariant,
            )
            .one_or_none()
        )


class TypeSkeleton(TypeBase):
    class Config:
        title = "Type"

    representations: list[RepresentationSkeleton] = sqlmodel.Field(default_factory=list)


# class PieceBase(IdentifiedRow):

#     pass


# class Piece(PieceBase, Row, table=True):
#     """⭕ A piece is a 3d-instance of a type in a design."""

#     PLURAL = "pieces"
#     __tablename__ = "piece"
#     pk: typing.Optional[int] = sqlmodel.Field(
#         sa_column=sqlmodel.Column(
#             "id",
#             sqlalchemy.Integer(),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     id_: str = sqlmodel.Field(
#         alias="id",
#         sa_column=sqlmodel.Column(
#             "localId",
#             sqlalchemy.String(NAME_LENGTH_LIMIT),
#         ),
#     )
#     planePk: typing.Optional[int] = sqlmodel.Field(
#         alias="planeId",
#         sa_column=sqlmodel.Column(
#             "planeId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("plane.id"),
#         ),
#         default=None,
#         exclude=True,
#     )
#     plane: typing.Optional[Plane] = sqlmodel.Relationship(back_populates="piece")
#     screenPointX: int = sqlmodel.Field(exclude=True)
#     screenPointY: int = sqlmodel.Field(exclude=True)
#     designPk: typing.Optional[int] = sqlmodel.Field(
#         alias="designId",
#         sa_column=sqlmodel.Column(
#             "designId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("design.id"),
#         ),
#         default=None,
#         exclude=True,
#     )
#     design: typing.Optional["Design"] = sqlmodel.Relationship(back_populates="pieces")
#     connecteds: list["Connection"] = sqlmodel.Relationship(
#         back_populates="connectedPiece",
#         sa_relationship_kwargs={"foreign_keys": "Connection.connectedPiecePk"},
#     )
#     connectings: list["Connection"] = sqlmodel.Relationship(
#         back_populates="connectingPiece",
#         sa_relationship_kwargs={"foreign_keys": "Connection.connectingPiecePk"},
#     )

#     __table_args__ = (
#         sqlalchemy.UniqueConstraint("localId", "designId"),
#         sqlalchemy.CheckConstraint(
#             """
#             (
#                 (planeOriginX IS NULL AND planeOriginY IS NULL AND planeOriginZ IS NULL AND
#                  planeXAxisX IS NULL AND planeXAxisY IS NULL AND planeXAxisZ IS NULL AND
#                  planeYAxisX IS NULL AND planeYAxisY IS NULL AND planeYAxisZ IS NULL)
#             OR
#                 (planeOriginX IS NOT NULL AND planeOriginY IS NOT NULL AND planeOriginZ IS NOT NULL AND
#                  planeXAxisX IS NOT NULL AND planeXAxisY IS NOT NULL AND planeXAxisZ IS NOT NULL AND
#                  planeYAxisX IS NOT NULL AND planeYAxisY IS NOT NULL AND planeYAxisZ IS NOT NULL)
#             )
#             """,
#             name="planeSetOrNotSet",
#         ),
#     )

#     @property
#     def plane(self) -> Plane:
#         return Plane(
#             origin=Point(
#                 self.planeOriginX,
#                 self.planeOriginY,
#                 self.planeOriginZ,
#             ),
#             xAxis=Vector(
#                 self.planeXAxisX,
#                 self.planeXAxisY,
#                 self.planeXAxisZ,
#             ),
#             yAxis=Vector(
#                 self.planeYAxisX,
#                 self.planeYAxisY,
#                 self.planeYAxisZ,
#             ),
#         )

#     @plane.setter
#     def plane(self, plane: Plane):
#         self.planeOriginX = plane.origin.x
#         self.planeOriginY = plane.origin.y
#         self.planeOriginZ = plane.origin.z
#         self.planeXAxisX = plane.xAxis.x
#         self.planeXAxisY = plane.xAxis.y
#         self.planeXAxisZ = plane.xAxis.z
#         self.planeYAxisX = plane.yAxis.x
#         self.planeYAxisY = plane.yAxis.y
#         self.planeYAxisZ = plane.yAxis.z

#     @property
#     def screenPoint(self) -> ScreenPoint:
#         return ScreenPoint(self.screenPointPointX, self.screenPointPointY)

#     @screenPoint.setter
#     def screenPoint(self, screenPoint: ScreenPoint):
#         self.screenPointPointX = screenPoint.x
#         self.screenPointPointY = screenPoint.y

#     def parent(self) -> "Design":
#         if self.design is None:
#             raise NoParentAssigned()
#         return self.design


# class PieceSkeleton(PieceBase):
#     id_: str = sqlmodel.Field(alias="id")
#     plane: Plane = sqlmodel.Field(default_factory=Plane)
#     screenPoint: ScreenPoint = sqlmodel.Field(default_factory=ScreenPoint)


# class PieceIdSkeleton(sqlmodel.SQLModel):
#     id_: str = sqlmodel.Field(alias="id")


# class SidePieceType(sqlmodel.SQLModel):
#     port: Port = sqlmodel.Field()


# class SidePieceTypeSkeleton(sqlmodel.SQLModel):
#     port: PortIdSkeleton = sqlmodel.Field()


# class SidePiece(sqlmodel.SQLModel):
#     id_: str = sqlmodel.Field(alias="id")
#     type: SidePieceType = sqlmodel.Field()


# class SidePieceSkeleton(sqlmodel.SQLModel):
#     id_: str = sqlmodel.Field(alias="id")
#     type: SidePieceTypeSkeleton = sqlmodel.Field()


# class Side(sqlmodel.SQLModel):
#     piece: SidePiece = sqlmodel.Field()


# class SideSkeleton(sqlmodel.SQLModel):
#     piece: SidePieceSkeleton = sqlmodel.Field()


# class ConnectionBase(sqlmodel.SQLModel):
#     rotation: float = sqlmodel.Field(ge=0, lt=360)
#     offset: float = sqlmodel.Field()


# class Connection(ConnectionBase, Row, table=True):
#     """🖇️ A connection between two pieces of a design."""

#     PLURAL = "connections"
#     __tablename__ = "connection"

#     connectedPiecePk: typing.Optional[int] = sqlmodel.Field(
#         alias="connectedPieceId",
#         sa_column=sqlmodel.Column(
#             "connectedPieceId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("piece.id"),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     connectedPiece: Piece = sqlmodel.Relationship(
#         sa_relationship=sqlalchemy.orm.relationship(
#             "Piece",
#             back_populates="connecteds",
#             foreign_keys="[Connection.connectedPiecePk]",
#         )
#     )
#     connectedPieceTypePortPk: typing.Optional[int] = sqlmodel.Field(
#         alias="connectedPieceTypePortId",
#         sa_column=sqlmodel.Column(
#             "connectedPieceTypePortId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("port.id"),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     connectedPieceTypePort: Port = sqlmodel.Relationship(
#         sa_relationship=sqlalchemy.orm.relationship(
#             "Port",
#             back_populates="connecteds",
#             foreign_keys="[Connection.connectedPieceTypePortPk]",
#         )
#     )
#     connectingPiecePk: typing.Optional[int] = sqlmodel.Field(
#         alias="connectingPieceId",
#         sa_column=sqlmodel.Column(
#             "connectingPieceId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("piece.id"),
#             primary_key=True,
#         ),
#         exclude=True,
#         default=None,
#     )
#     connectingPiece: Piece = sqlmodel.Relationship(
#         sa_relationship=sqlalchemy.orm.relationship(
#             "Piece",
#             back_populates="connectings",
#             foreign_keys="[Connection.connectingPiecePk]",
#         )
#     )
#     connectingPieceTypePortPk: typing.Optional[int] = sqlmodel.Field(
#         alias="connectingPieceTypePortId",
#         sa_column=sqlmodel.Column(
#             "connectingPieceTypePortId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("port.id"),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     connectingPieceTypePort: Port = sqlmodel.Relationship(
#         sa_relationship=sqlalchemy.orm.relationship(
#             "Port",
#             back_populates="connectings",
#             foreign_keys="[Connection.connectingPieceTypePortPk]",
#         )
#     )
#     designPk: typing.Optional[int] = sqlmodel.Field(
#         alias="designId",
#         sa_column=sqlmodel.Column(
#             "designId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("design.id"),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     design: "Design" = sqlmodel.Relationship(back_populates="connections")
#     __table_args__ = (
#         sqlalchemy.CheckConstraint(
#             "connectingPieceId != connectedPieceId",
#             name="noReflexiveConnection",
#         ),
#     )

#     @property
#     def connected(self) -> Side:
#         return Side(
#             piece=SidePiece(
#                 id_=self.connectedPiece.id,
#                 type=SidePieceType(
#                     port=PortIdSkeleton(id=self.connectedPieceTypePort.id_)
#                 ),
#             )
#         )

#     @property
#     def connecting(self) -> Side:
#         return Side(
#             piece=SidePiece(
#                 id_=self.connectingPiece.id,
#                 type=SidePieceType(
#                     port=PortIdSkeleton(id=self.connectingPieceTypePort.id_)
#                 ),
#             )
#         )


# class ConnectionSkeleton(ConnectionBase):
#     class Config:
#         title = "Connection"

#     connected: Side = sqlmodel.Field()
#     connecting: Side = sqlmodel.Field()


# class DesignBase(VariableArtifactRow):
#     pass


# class Design(DesignBase, Row, table=True):
#     """🏙️ A design is a collection of pieces that are connected."""

#     PLURAL = "designs"
#     __tablename__ = "design"
#     pk: typing.Optional[int] = sqlmodel.Field(
#         sa_column=sqlmodel.Column(
#             "id",
#             sqlalchemy.Integer(),
#             primary_key=True,
#         ),
#         default=None,
#         exclude=True,
#     )
#     pieces: list[Piece] = sqlmodel.Relationship(
#         back_populates="design", cascade_delete=True
#     )
#     connections: list[Connection] = sqlmodel.Relationship(
#         back_populates="design", cascade_delete=True
#     )
#     qualities: list[Quality] = sqlmodel.Relationship(
#         back_populates="design", cascade_delete=True
#     )
#     kitPk: typing.Optional[int] = sqlmodel.Field(
#         alias="kitId",
#         sa_column=sqlmodel.Column(
#             "kitId",
#             sqlalchemy.Integer(),
#             sqlalchemy.ForeignKey("kit.id"),
#         ),
#         default=None,
#         exclude=True,
#     )
#     kit: typing.Optional["Kit"] = sqlmodel.Relationship(back_populates="designs")

#     # __table_args__ = (sqlalchemy.UniqueConstraint('name', 'variant', 'kitPk'),)

#     def parent(self) -> "Kit":
#         if self.kit is None:
#             raise NoKitAssigned()
#         return self.kit

#     def getByLocalId(
#         self, session: sqlalchemy.orm.Session, localId: tuple, decode: bool = False
#     ) -> "Design":
#         return (
#             session.query(Design)
#             .filter(Design.name == decodeString(localId[0]) if decode else localId[0])
#             .first()
#         )


# class DesignSkeleton(DesignBase):
#     class Config:
#         title = "Design"

#     pieces: list[PieceSkeleton] = sqlmodel.Field(default_factory=list)
#     connections: list[ConnectionSkeleton] = sqlmodel.Field(default_factory=list)


class KitBase(ArtifactRow):
    url: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT, default="")
    homepage: str = sqlmodel.Field(max_length=URL_LENGTH_LIMIT, default="")


class Kit(KitBase, Row, table=True):
    """🗃️ A kit is a collection of types and designs."""

    PLURAL = "kits"
    __tablename__ = "kit"
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            "id",
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    types: list[Type] = sqlmodel.Relationship(back_populates="kit", cascade_delete=True)
    # designs: list[Design] = sqlmodel.Relationship(
    #     back_populates="kit", cascade_delete=True
    # )

    __table_args__ = (sqlalchemy.UniqueConstraint("name"),)

    def guid(self) -> str:
        return encodeString(self.url)

    @classmethod
    def specific(
        cls: "Kit",
        url: str,
    ):
        session = sessionByUrl(url)
        name = kitNameByUrl(url)
        return session.query(Kit).filter(Kit.name == name).one_or_none()

    @classmethod
    def all(
        cls: "Kit",
    ) -> list["Kit"]:
        raise NotImplementedError()


class KitSkeleton(KitBase):

    class Config:
        title = "Kit"

    types: list[TypeSkeleton] = sqlmodel.Field(default_factory=list)


ENTITIES = [Kit, Type, Port, Quality, Representation]
