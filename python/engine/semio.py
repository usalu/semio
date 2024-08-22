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

'''
semio.
'''
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
import abc
import base64
import enum
import os
import pathlib
import functools
import typing
import urllib
from datetime import datetime
import bcrypt
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
import sqlmodel
import sqlalchemy


RELEASE = 'r24.09-1'
NAME_LENGTH_MAX = 512
DESCRIPTION_LENGTH_MAX = 4096
URL_LENGTH_MAX = 1024
KIT_FOLDERNAME = '.semio'
KIT_FILENAME = 'kit.sqlite3'

TOLERANCE = 1e-5
SIGNIFICANT_DIGITS = 5

MIMES = {
    '.stl': 'model/stl',
    '.obj': 'model/obj',
    '.glb': 'model/gltf-binary',
    '.gltf': 'model/gltf+json',
    '.3dm': 'model/vnd.3dm',
    '.png': 'image/png',
    '.jpg': 'image/jpeg',
    '.jpeg': 'image/jpeg',
    '.svg': 'image/svg+xml',
    '.pdf': 'application/pdf',
    '.zip': 'application/zip',
    '.json': 'application/json',
    '.csv': 'text/csv',
    '.txt': 'text/plain',
}

ureg = pint.UnitRegistry()


class SemioException(Exception):
    '''❗ The base class for all exceptions in semio.'''

    pass


class SpecificationError(SemioException):
    '''🚫 The base class for all specification errors.
    A specification error is when the user input does not respect the specification.'''

    pass


class NoParentAssigned(SpecificationError):
    '''👪 The entity has no parent assigned.'''

    pass

class NoRepresentationAssigned(NoParentAssigned):
    '''👪 The entity has no representation assigned.'''

    pass

class NoTypeAssigned(NoParentAssigned):
    '''👪 The entity has no type assigned.'''

    pass

class NoKitAssigned(NoParentAssigned):
    '''👪 The entity has no kit assigned.'''

    pass

class InvalidURL(ValueError, SpecificationError):
    '''🔗 The URL is not valid. An url must have the form:
    scheme://netloc/path;parameters?query#fragment.'''

    def __init__(self, url: str) -> None:
        self.url = url

    def __str__(self) -> str:
        return f'{self.url} is not a valid URL.'


class InvalidDatabase(SemioException):
    '''💾 The state of the database is somehow invalid.
    Check the constraints and the insert validators.
    '''

    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + '\n The database is invalid. Please report this bug.'


class InvalidBackend(SemioException):
    '''🖥️ The backend processed something wrong. Check the order of operations.'''

    def __init__(self, message: str) -> None:
        self.message = message

    def __str__(self) -> str:
        return self.message + '\n The backend is invalid. Please report this bug.'

class InvalidGuid(SemioException):
    '''🆔 The guid is not valid. A guid must have the form:
    KIND_ABBREVIATION:base64encoded(ENTITY_LOCAL_ID).REPEATED_PARENT_ENTITY_KIND_ABBREVIATION:base64encoded(PARENT_ENTITY_LOCAL_ID)'''

    pass

def encodeToBase64(data: str) -> str:
    byteData = data.encode("utf-8")
    base64Bytes = base64.b64encode(byteData)
    base64String = base64Bytes.decode("utf-8")
    return base64String

def decodeFromBase64(encoded: str) -> str:
    base64Bytes = encoded.encode("utf-8")
    byteData = base64.b64decode(base64Bytes)
    return byteData.decode("utf-8")


class StoreKind(enum.Enum):
    """🏪 The kind of the store."""

    DATABASE = "database"
    REST = "rest"
    GRAPHQL = "graphql"


class SSLMode(enum.Enum):
    """🔒 The security level of the session"""

    DISABLE = "disable"
    ALLOW = "allow"
    PREFER = "prefer"
    REQUIRE = "require"
    VERIFY_CA = "verify-ca"
    VERIFY_FULL = "verify-full"


def getSession(
    scheme: str, username="", password="", host="", database="", sslMode=SSLMode.REQUIRE
) -> sqlalchemy.orm.Session:
    match scheme:
        case "postgresql":
            schemeWithAdapter = "postgresql+psycopg"
        case _:
            schemeWithAdapter = scheme
    connection_string = sqlalchemy.URL.create(
        schemeWithAdapter,
        username=username,
        password=password,
        host=host,
        database=database,
    )
    match scheme:
        case "sqlite":
            engine = sqlalchemy.create_engine(connection_string)
        case "postgresql":
            engine = sqlalchemy.create_engine(
                connection_string, connect_args={"sslmode": sslMode.value}
            )
    sqlmodel.SQLModel.metadata.create_all(engine)
    session = sqlalchemy.orm.sessionmaker(bind=engine)()
    return session


def getSessionByUrl(url: str) -> sqlalchemy.orm.Session:
    parsedUrl = urllib.parse.urlparse(url)
    match parsedUrl.scheme:
        case "":
            if not parsedUrl.path.endswith(".sqlite3"):
                if not parsedUrl.path.endswith(".semio"):
                    path = pathlib.Path(parsedUrl.path) / KIT_FOLDERNAME / KIT_FILENAME
                else:
                    path = pathlib.Path(parsedUrl.path) / KIT_FILENAME
            else:
                path = pathlib.Path(parsedUrl.path)
            return getSession(
                "sqlite", parsedUrl.username, parsedUrl.password, database=str(path)
            )
        case "sqlite":
            return getSession(
                "sqlite",
                parsedUrl.username,
                parsedUrl.password,
                database=parsedUrl.path[1:],
            )
        case _:
            return getSession(
                parsedUrl.scheme,
                parsedUrl.username,
                parsedUrl.password,
                parsedUrl.hostname,
                parsedUrl.path[1:],
            )


def getStoreKindByUrl(url: str) -> StoreKind:
    parsedUrl = urllib.parse.urlparse(url)
    if parsedUrl.path.endswith("/graphql"):
        return StoreKind.GRAPHQL
    if parsedUrl.scheme == "http" or parsedUrl.scheme == "https":
        return StoreKind.REST
    return StoreKind.DATABASE


def getEntityByGuid(guid: str) -> 'Entity':
    kindAbbreviation = guid[0:2]
    enitiesGuids = guid.split(".")
    kitGuid = enitiesGuids.pop()
    kitAbbreviation, kitUrl = kitGuid.split(":")
    if kitAbbreviation != Kit.ABBREVIATION:
        raise InvalidGuid()
    session = getSessionByUrl(kitUrl)


class Semio(sqlmodel.SQLModel):
    '''ℹ️ Metadata about the semio database.'''

    __tablename__ = 'semio'

    release: str = sqlmodel.Field(default=RELEASE, primary_key=True)
    createdAt: datetime = sqlmodel.Field(default_factory=datetime.now)


class Entity(sqlmodel.SQLModel):
    '''Base class for all entitites in semio.'''

    ABBREVIATION: typing.ClassVar[str]
    '''🤏 A unique abbreviation for the entity.'''

    @abc.abstractmethod
    def parent(self) -> typing.Union['Entity', None]:
        '''👪 The parent of the entity.'''
        pass

    @abc.abstractmethod
    def localId(self) -> tuple:
        '''🆔 A tuple that identifies the entity within it's parent.'''
        pass

    def humanId(self) -> str:
        '''🪪 A string that let's the user identify the entity within it's parent.'''
        return f'{self.__class__.__name__}({', '.join(self.localId())})'

    def guid(self) -> str:
        '''🆔 A guid that let's relay identify the entity.'''
        localId = f'{self.ABBREVIATION}:{','.join([encodeToBase64(field) for field in self.localId()])}'
        parentId = f".{self.parent().guid()}" if self.parent() is not None else ""
        return localId + parentId
    
    def fetchFromGuid(guid: str) -> 'Entity':
        '''🔍 Fetch the entity from the guid.'''
        pass

    def existsError(self) -> SemioException:
        '''🔍 An error that is raised when the entity already exists.'''
        pass

    def notFoundError(self) -> SemioException:
        pass


class ArtifactModel(Entity):
    '''♻️ An artifact is anything that is worth to be reused.'''

    name: str = sqlmodel.Field(max_length=NAME_LENGTH_MAX)
    # Optional. Set to '' for None.
    description: str = sqlmodel.Field(max_length=DESCRIPTION_LENGTH_MAX, default='')
    # Optional. Set to '' for None.
    icon: str = sqlmodel.Field(default='', max_length=URL_LENGTH_MAX)
    createdAt: datetime = sqlmodel.Field(default_factory=datetime.now)
    lastUpdateAt: datetime = sqlmodel.Field(default_factory=datetime.now)


class VariableArtifactModel(ArtifactModel):
    '''🎚️ A variable artifact is an artifact that has variants (at least one default).'''

    variant: str = sqlmodel.Field(max_length=NAME_LENGTH_MAX, default='')


class Tag(Entity, table=True):
    '''🏷️ A tag is meta-data for grouping representations.'''

    __tablename__ = 'tag'
    value: str = sqlmodel.Field(primary_key=True)
    representationPk: int = sqlmodel.Field(
        alias='representationId',
        sa_column=sqlmodel.Column(
            'representationId',
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey('representation.id'),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    representation: 'Representation' = sqlmodel.Relationship(back_populates='_tags')

    def parent(self) -> typing.Union['Representation', None]:
        if self.representation is None:
            raise NoRepresentationAssigned()
        return self.representation

    def localId(self) -> tuple:
        return (self.value,)
    
    def fetchFromGuid(guid: str) -> Entity:
        return 


class RepresentationBase(Entity):
    url: str
    lod: str = ''


class Representation(RepresentationBase, table=True):
    '''💾 A representation is a link to a file that describes a type for a unique combination of level of detail, tags and mime.'''

    ABBREVIATION = 'Rp'
    __tablename__ = 'representation'
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            'id',
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    _tags: list[Tag] = sqlmodel.Relationship(
        back_populates='representation', cascade_delete=True
    )
    typePk: typing.Optional[int] = sqlmodel.Field(
        alias='typeId',
        sa_column=sqlmodel.Column(
            'typeId',
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey('type.id'),
        ),
        default=None,
        exclude=True,
    )
    type: typing.Union['Type', None] = sqlmodel.Relationship(back_populates='representations')

    @property
    def tags(self) -> list[str]:
        return [tag.value for tag in self._tags or []]

    @tags.setter
    def tags(self, tags: list[str]):
        self._tags = [Tag(value=tag) for tag in tags]

    def parent(self) -> 'Type':
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    def localId(self) -> tuple:
        return (self.url,)


class RepresentationSkeleton(RepresentationBase):
    class Config:
        title = 'Representation'

    id: str = ''
    tags: list[str] = sqlmodel.Field(default_factory=list)


class LocatorBase(Entity):
    # Optional. '' means true.
    subgroup: str = sqlmodel.Field(default='', max_length=NAME_LENGTH_MAX)


class Locator(LocatorBase, table=True):
    '''🗺️ A locator is meta-data for grouping ports.'''

    ABBREVIATION = 'Lc'
    __tablename__ = 'locator'
    group: str = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            'groupName',
            sqlalchemy.String(NAME_LENGTH_MAX),
            primary_key=True,
        ),
    )
    portPk: typing.Optional[int] = sqlmodel.Field(
        alias='portId',
        sa_column=sqlmodel.Column(
            'portId',
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey('port.id'),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    port: typing.Union['Port', None] = sqlmodel.Relationship(back_populates='locators')


class LocatorSkeleton(LocatorBase):
    class Config:
        title = 'Locator'


def prettyNumber(number: float) -> str:
    if number == -0.0:
        number = 0.0
    return f'{number:.5f}'.rstrip('0').rstrip('.')


class ScreenPoint(Entity):
    '''📺 A 2d-point (xy) of integers in screen coordinate system.'''

    x: int = 0
    y: int = 0

    def __init__(self, x: int = 0, y: int = 0):
        super().__init__(x=x, y=y)

    def __len__(self):
        return 2

    def __getitem__(self, key):
        if key == 0:
            return self.x
        elif key == 1:
            return self.y
        else:
            raise IndexError('Index out of range')

    def __iter__(self):
        return iter((self.x, self.y))


class Point(Entity):
    '''✖️ A 3d-point (xyz) of floating point numbers.'''

    x: float = 0.0
    y: float = 0.0
    z: float = 0.0

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return (
            f'[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]'
        )

    def __repr__(self) -> str:
        return (
            f'[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]'
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
            raise IndexError('Index out of range')

    def __iter__(self):
        return iter((self.x, self.y, self.z))

    def isCloseTo(self, other: 'Point', tol: float = TOLERANCE) -> bool:
        return (
            abs(self.x - other.x) < tol
            and abs(self.y - other.y) < tol
            and abs(self.z - other.z) < tol
        )

    def transform(self, transform: 'Transform') -> 'Point':
        return Transform.transformPoint(transform, self)

    def toVector(self) -> 'Vector':
        return Vector(self.x, self.y, self.z)


class Vector(Entity):
    '''➡️ A 3d-vector (xyz) of floating point numbers.'''

    x: float = 0.0
    y: float = 0.0
    z: float = 0.0

    def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
        super().__init__(x=x, y=y, z=z)

    def __str__(self) -> str:
        return (
            f'[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]'
        )

    def __repr__(self) -> str:
        return (
            f'[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]'
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
            raise IndexError('Index out of range')

    def __iter__(self):
        return iter((self.x, self.y, self.z))

    def __add__(self, other):
        return Vector(self.x + other.x, self.y + other.y, self.z + other.z)

    @property
    def length(self) -> float:
        return (self.x**2 + self.y**2 + self.z**2) ** 0.5

    def revert(self) -> 'Vector':
        return Vector(-self.x, -self.y, -self.z)

    def amplify(self, factor: float) -> 'Vector':
        return Vector(self.x * factor, self.y * factor, self.z * factor)

    def isCloseTo(self, other: 'Vector', tol: float = TOLERANCE) -> bool:
        return (
            abs(self.x - other.x) < tol
            and abs(self.y - other.y) < tol
            and abs(self.z - other.z) < tol
        )

    def normalize(self) -> 'Vector':
        length = self.length
        return Vector(x=self.x / length, y=self.y / length, z=self.z / length)

    def dot(self, other: 'Vector') -> float:
        return dot(self, other)

    def cross(self, other: 'Vector') -> 'Vector':
        return Vector(*cross(self, other))

    def transform(self, transform: 'Transform') -> 'Vector':
        return Transform.transformVector(transform, self)

    def toPoint(self) -> 'Point':
        return Point(self.x, self.y, self.z)

    def toTransform(self) -> 'Transform':
        return Transform.fromTranslation(self)

    @staticmethod
    def X() -> 'Vector':
        return Vector(x=1)

    @staticmethod
    def Y() -> 'Vector':
        return Vector(y=1)

    @staticmethod
    def Z() -> 'Vector':
        return Vector(z=1)


class CoordinateSystem(Entity):
    '''◳ A coordinate system is an origin (point) and an orientation (x-axis and y-axis).'''

    origin: Point
    xAxis: Vector
    yAxis: Vector

    def __init__(
        self, origin: Point = None, xAxis: Vector = None, yAxis: Vector = None
    ):
        if origin is None:
            origin = Point()
        if xAxis is None and yAxis is None:
            xAxis = Vector.X()
            yAxis = Vector.Y()
        if xAxis is None:
            xAxis = Vector()
        if yAxis is None:
            yAxis = Vector()
        if abs(xAxis.length - 1) > TOLERANCE:
            raise ValidationError('The x-axis must be normalized.')
        if abs(yAxis.length - 1) > TOLERANCE:
            raise ValidationError('The y-axis must be normalized.')
        if abs(xAxis.dot(yAxis)) > TOLERANCE:
            raise ValidationError('The x-axis and y-axis must be orthogonal.')
        super().__init__(origin=origin, xAxis=xAxis, yAxis=yAxis)

    def isCloseTo(self, other: 'CoordinateSystem', tol: float = TOLERANCE) -> bool:
        return (
            self.origin.isCloseTo(other.origin, tol)
            and self.xAxis.isCloseTo(other.xAxis, tol)
            and self.yAxis.isCloseTo(other.yAxis, tol)
        )

    @property
    def zAxis(self) -> Vector:
        return self.xAxis.cross(self.yAxis)

    def transform(self, transform: 'Transform') -> 'CoordinateSystem':
        return Transform.transformCoordinateSystem(transform, self)

    def toTransform(self) -> 'Transform':
        return Transform.fromCoordinateSystem(self)

    @staticmethod
    def XY() -> 'CoordinateSystem':
        return CoordinateSystem(
            origin=Point(),
            xAxis=Vector.X(),
            yAxis=Vector.Y(),
        )

    @staticmethod
    def fromYAxis(
        yAxis: Vector, theta: float = 0.0, origin: Point = None
    ) -> 'CoordinateSystem':
        if abs(yAxis.length - 1) > TOLERANCE:
            raise SpecificationError('The yAxis must be normalized.')
        if origin is None:
            origin = Point()
        orientation = Transform.fromDirections(Vector.Y(), yAxis)
        rotation = Transform.fromAngle(yAxis, theta)
        xAxis = Vector.X().transform(rotation.after(orientation))
        return CoordinateSystem(origin=origin, xAxis=xAxis, yAxis=yAxis)


class Rotation(Entity):
    '''🔄 A rotation is an axis and an angle.'''

    axis: Vector
    angle: float

    def __init__(self, axis: Vector, angle: float):
        super().__init__(axis=axis, angle=angle)

    def toTransform(self) -> 'Transform':
        return Transform.fromRotation(self)


class Transform(ndarray):
    '''▦ A 4x4 translation and rotation transformation matrix (no scaling or shearing).'''

    def __new__(cls, input_array=None):
        if input_array is None:
            input_array = eye(4, dtype=float)
        else:
            input_array = asarray(input_array).astype(float)
        obj = input_array.view(cls)
        return obj

    def __array_finalize__(self, obj):
        if obj is None:
            return

    def __str__(self) -> str:
        rounded_self = self.round()
        return f'Transform(Rotation={rounded_self.rotation}, Translation={rounded_self.translation})'

    def __repr__(self) -> str:
        rounded_self = self.round()
        return f'Transform(Rotation={rounded_self.rotation}, Translation={rounded_self.translation})'

    @property
    def rotation(self) -> Rotation | None:
        '''🔄 The rotation part of the transform.'''
        rotationMatrix = self[:3, :3]
        axisAngle = axis_angle_from_matrix(rotationMatrix)
        if axisAngle[3] == 0:
            return None
        return Rotation(
            axis=Vector(float(axisAngle[0]), float(axisAngle[1]), float(axisAngle[2])),
            angle=float(degrees(axisAngle[3])),
        )

    @property
    def translation(self) -> Vector:
        '''➡️ The translation part of the transform.'''
        return Vector(*self[:3, 3])

    # for pydantic
    def dict(self) -> typing.Dict[str, typing.Union[Rotation, Vector]]:
        return {
            'rotation': self.rotation,
            'translation': self.translation,
        }

    def after(self, before: 'Transform') -> 'Transform':
        '''✖️ Apply this transform after another transform.

        Args:
            before (Transform): Transform to apply before this transform.

        Returns:
            Transform: New transform.
        '''
        return Transform(concat(before, self))

    def invert(self) -> 'Transform':
        return Transform(invert_transform(self))

    def transformPoint(self, point: Point) -> Point:
        transformedPoint = transform(self, vector_to_point(point))
        return Point(*transformedPoint[:3])

    def transformVector(self, vector: Vector) -> Vector:
        transformedVector = transform(self, vector_to_direction(vector))
        return Vector(*transformedVector[:3])

    def transformCoordinateSystem(
        self, coordinateSystem: CoordinateSystem
    ) -> CoordinateSystem:
        coordinateSystemTransform = Transform.fromCoordinateSystem(coordinateSystem)
        coordinateSystemTransformed = coordinateSystemTransform.after(self)
        return Transform.toCoordinateSystem(coordinateSystemTransformed)

    def transform(
        self, geometry: typing.Union[Point, Vector, CoordinateSystem]
    ) -> typing.Union[Point, Vector, CoordinateSystem]:
        if isinstance(geometry, Point):
            return self.transformPoint(geometry)
        elif isinstance(geometry, Vector):
            return self.transformVector(geometry)
        elif isinstance(geometry, CoordinateSystem):
            return self.transformCoordinateSystem(geometry)
        else:
            raise NotImplementedError()

    def round(self, decimals: int = SIGNIFICANT_DIGITS) -> 'Transform':
        return Transform(super().round(decimals=decimals))

    @staticmethod
    def fromTranslation(vector: Vector) -> 'Transform':
        return Transform(
            transform_from(
                [
                    [1, 0, 0],
                    [0, 1, 0],
                    [0, 0, 1],
                ],
                vector,
            )
        )

    @staticmethod
    def fromRotation(rotation: Rotation) -> 'Transform':
        return Transform(
            transform_from(
                matrix_from_axis_angle((*rotation.axis, radians(rotation.angle))),
                Vector(),
            )
        )

    @staticmethod
    def fromCoordinateSystem(coordinateSystem: CoordinateSystem) -> 'Transform':
        # Assumes coordinateSystem is normalized
        return Transform(
            transform_from(
                [
                    [
                        coordinateSystem.xAxis.x,
                        coordinateSystem.yAxis.x,
                        coordinateSystem.zAxis.x,
                    ],
                    [
                        coordinateSystem.xAxis.y,
                        coordinateSystem.yAxis.y,
                        coordinateSystem.zAxis.y,
                    ],
                    [
                        coordinateSystem.xAxis.z,
                        coordinateSystem.yAxis.z,
                        coordinateSystem.zAxis.z,
                    ],
                ],
                coordinateSystem.origin,
            )
        )

    @staticmethod
    def fromAngle(axis: Vector, angle: float) -> 'Transform':
        return Transform(
            transform_from(matrix_from_axis_angle((*axis, radians(angle))), Vector())
        )

    @staticmethod
    def fromDirections(startDirection: Vector, endDirection: Vector) -> 'Transform':
        if startDirection.isCloseTo(endDirection):
            return Transform()
        axisAngle = axis_angle_from_two_directions(startDirection, endDirection)
        return Transform(transform_from(matrix_from_axis_angle(axisAngle), Vector()))

    def toCoordinateSystem(self) -> CoordinateSystem:
        return CoordinateSystem(
            origin=Point(*self[:3, 3]),
            xAxis=Vector(
                self[0, 0],
                self[1, 0],
                self[2, 0],
            ),
            yAxis=Vector(
                self[0, 1],
                self[1, 1],
                self[2, 1],
            ),
        )


class PortBase(Entity):
    '''🔌 A port is a connection point (with a direction) of a type.'''

    pass


class Port(PortBase, table=True):
    '''🔌 A port is a connection point (with a direction) of a type.'''

    ABBREVIATION = 'Po'
    __tablename__ = 'port'
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            'id',
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    # Can't use the name 'id' because of bug
    # https://github.com/graphql-python/graphene-sqlalchemy/issues/412
    id_: str = sqlmodel.Field(
        alias='id',
        sa_column=sqlmodel.Column(
            'localId',
            sqlalchemy.String(NAME_LENGTH_MAX),
        ),
    )
    pointX: float = sqlmodel.Field(exclude=True)
    pointY: float = sqlmodel.Field(exclude=True)
    pointZ: float = sqlmodel.Field(exclude=True)
    directionX: float = sqlmodel.Field(exclude=True)
    directionY: float = sqlmodel.Field(exclude=True)
    directionZ: float = sqlmodel.Field(exclude=True)
    typePk: typing.Optional[int] = sqlmodel.Field(
        alias='typeId',
        sa_column=sqlmodel.Column(
            'typeId',
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey('type.id'),
        ),
        default=None,
        exclude=True,
    )
    type: typing.Union['Type', None] = sqlmodel.Relationship(back_populates='ports')
    locators: list[Locator] = sqlmodel.Relationship(
        back_populates='port', cascade_delete=True
    )

    __table_args__ = (sqlalchemy.UniqueConstraint('localId', 'typeId'),)

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

    def parent(self) -> 'Type':
        if self.type is None:
            raise NoTypeAssigned()
        return self.type

    def localId(self) -> tuple:
        return (self.id_,)


class TypeBase(VariableArtifactModel):
    pass


class Type(TypeBase, table=True):
    '''🧩 A type is a reusable element that can be connected with other types over ports.'''

    ABBREVIATION = 'Ty'
    __tablename__ = 'type'
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            'id',
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    representations: list[Representation] = sqlmodel.Relationship(
        back_populates='type',
        cascade_delete=True,
    )
    ports: list[Port] = sqlmodel.Relationship(back_populates='type', cascade_delete=True)
    kitPk: typing.Optional[int] = sqlmodel.Field(
        alias='kitId',
        sa_column=sqlmodel.Column(
            'kitId',
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey('kit.id'),
        ),
        default=None,
        exclude=True,
    )
    kit: typing.Union['Kit', None] = sqlmodel.Relationship(back_populates='types')

    # __table_args__ = (sqlalchemy.UniqueConstraint('name', 'variant', 'kitPk'),)

    def parent(self) -> 'Kit':
        if self.kit is None:
            raise NoKitAssigned()
        return self.kit
    
    def localId(self) -> tuple:
        return (self.name, self.variant)


class TypeSkeleton(TypeBase):
    class Config:
        title = 'Type'

    representations: list[RepresentationSkeleton] = sqlmodel.Field(default_factory=list)


class KitBase(ArtifactModel):
    url: str = sqlmodel.Field(max_length=URL_LENGTH_MAX, default='')
    homepage: str = sqlmodel.Field(max_length=URL_LENGTH_MAX, default='')


class Kit(KitBase, table=True):
    '''🗃️ A kit is a collection of types and designs.'''

    ABBREVIATION = 'Kt'
    __tablename__ = 'kit'
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            'id',
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    types: list[Type] = sqlmodel.Relationship(back_populates='kit', cascade_delete=True)
    # designs: list[Design] = sqlmodel.Relationship(back_populates='kit', cascade_delete=True)
    accountPk: typing.Optional[int] = sqlmodel.Field(
        alias='accountId',
        sa_column=sqlmodel.Column(
            'accountId',
            sqlalchemy.Integer(),
            sqlalchemy.ForeignKey('account.id'),
        ),
        default=None,
        exclude=True,
    )
    account: typing.Union['Account', None] = sqlmodel.Relationship(back_populates='kits')
    __table_args__ = (sqlalchemy.UniqueConstraint('name'), sqlalchemy.UniqueConstraint('url'))

    def parent(self) -> 'Account':
        return None
    
    def localId(self) -> tuple:
        return (self.url,)

class KitSkeleton(KitBase):

    class Config:
        title = 'Kit'

    types: list[TypeSkeleton] = sqlmodel.Field(default_factory=list)


class AccountBase(Entity):

    pass


class Account(AccountBase, table=True):
    '''👤 An account for semio.'''

    ABBREVIATION = 'Ac'
    __tablename__ = 'account'
    pk: typing.Optional[int] = sqlmodel.Field(
        sa_column=sqlmodel.Column(
            'id',
            sqlalchemy.Integer(),
            primary_key=True,
        ),
        default=None,
        exclude=True,
    )
    email: str = sqlmodel.Field(max_length=NAME_LENGTH_MAX)
    _password: str = sqlmodel.Field(max_length=NAME_LENGTH_MAX, name="password")
    kits: list[Kit] = sqlmodel.Relationship(back_populates='account', cascade_delete=True)

    __table_args__ = (sqlalchemy.UniqueConstraint('email'),)

    def parent(self) -> None:
        return None

    def localId(self) -> tuple:
        return (self.email,)

    @property
    def password(self) -> str:
        raise AttributeError("Password is not readable")

    @password.setter
    def password(self, rawPassword: str) -> None:
        self._password = bcrypt.hashpw(rawPassword.encode('utf-8'), bcrypt.gensalt()).decode('utf-8')

    def verify_password(self, rawPassword: str) -> bool:
        return bcrypt.checkpw(rawPassword.encode('utf-8'), self._password.encode('utf-8'))

def create_db_and_tables():
    path = pathlib.Path('engine2.sqlite3')
    try:
        os.remove(path)
    except:
        pass
    r1 = Representation(url='https://www.google.com')
    # print(r1.guid())
    r2 = Representation(url='https://www.yahoo.com')
    r2.tags = ['tag1', 'tag2']
    r2.tags = ['tag3', 'tag4']
    r3 = Representation(id='y2', url='https://www.yahoo.com1')
    t1 = Type(name='capsule')
    t1.representations = [r1, r2, r3]
    k1 = Kit(name='kit1', url=str(path), types=[t1])
    print(r1.guid())
    engine = sqlalchemy.create_engine('sqlite:///' + str(path))
    sqlmodel.SQLModel.metadata.create_all(engine)
    with sqlalchemy.orm.Session(engine) as session:
        session.add(k1)
        [r1n, r2n, r3n] = t1.representations
        r2n.tags = ['tag5', 'tag6']
        session.commit()
        pass
    pass

create_db_and_tables()
