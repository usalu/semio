import cytoscape from 'cytoscape'
import { Matrix4, Vector3 } from 'three'
import {
    Point as IPoint,
    Vector as IVector,
    Plane as IPlane,
    Formation,
    FormationInput,
    Port,
    Piece,
    PieceInput,
} from './semio.d'
import { i } from 'vitest/dist/reporters-yx5ZTtEV'

export const TOLERANCE = 1e-5

// class Point(BaseModel):
//     """✖️ A 3d-point (xyz) of floating point numbers."""

//     x: float = 0.0
//     y: float = 0.0
//     z: float = 0.0

//     def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
//         super().__init__(x=x, y=y, z=z)

//     def __str__(self) -> str:
//         return f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"

//     def __repr__(self) -> str:
//         return f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"

//     def __len__(self):
//         return 3

//     def __getitem__(self, key):
//         if key == 0:
//             return self.x
//         elif key == 1:
//             return self.y
//         elif key == 2:
//             return self.z
//         else:
//             raise IndexError("Index out of range")

//     def __iter__(self):
//         return iter((self.x, self.y, self.z))

//     def isCloseTo(self, other: "Point", tol: float = TOLERANCE) -> bool:
//         return (
//             abs(self.x - other.x) < tol
//             and abs(self.y - other.y) < tol
//             and abs(self.z - other.z) < tol
//         )

//     def transform(self, transform: "Transform") -> "Point":
//         return Transform.transformPoint(transform, self)

//     def toVector(self) -> "Vector":
//         return Vector(self.x, self.y, self.z)


// class Vector(BaseModel):
//     """➡️ A 3d-vector (xyz) of floating point numbers."""

//     x: float = 0.0
//     y: float = 0.0
//     z: float = 0.0

//     def __init__(self, x: float = 0.0, y: float = 0.0, z: float = 0.0):
//         super().__init__(x=x, y=y, z=z)

//     def __str__(self) -> str:
//         return f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"

//     def __repr__(self) -> str:
//         return f"[{prettyNumber(self.x)}, {prettyNumber(self.y)}, {prettyNumber(self.z)}]"

//     def __len__(self):
//         return 3

//     def __getitem__(self, key):
//         if key == 0:
//             return self.x
//         elif key == 1:
//             return self.y
//         elif key == 2:
//             return self.z
//         else:
//             raise IndexError("Index out of range")

//     def __iter__(self):
//         return iter((self.x, self.y, self.z))

//     def __add__(self, other):
//         return Vector(self.x + other.x, self.y + other.y, self.z + other.z)

//     @property
//     def length(self) -> float:
//         return (self.x**2 + self.y**2 + self.z**2) ** 0.5

//     def revert(self) -> "Vector":
//         return Vector(-self.x, -self.y, -self.z)

//     def amplify(self, factor: float) -> "Vector":
//         return Vector(self.x * factor, self.y * factor, self.z * factor)

//     def isCloseTo(self, other: "Vector", tol: float = TOLERANCE) -> bool:
//         return (
//             abs(self.x - other.x) < tol
//             and abs(self.y - other.y) < tol
//             and abs(self.z - other.z) < tol
//         )

//     def normalize(self) -> "Vector":
//         length = self.length
//         return Vector(x=self.x / length, y=self.y / length, z=self.z / length)

//     def dot(self, other: "Vector") -> float:
//         return dot(self, other)

//     def cross(self, other: "Vector") -> "Vector":
//         return Vector(*cross(self, other))

//     def transform(self, transform: "Transform") -> "Vector":
//         return Transform.transformVector(transform, self)

//     def toPoint(self) -> "Point":
//         return Point(self.x, self.y, self.z)

//     def toTransform(self) -> "Transform":
//         return Transform.fromTranslation(self)

//     @staticmethod
//     def X() -> "Vector":
//         return Vector(x=1)

//     @staticmethod
//     def Y() -> "Vector":
//         return Vector(y=1)

//     @staticmethod
//     def Z() -> "Vector":
//         return Vector(z=1)


// class Plane(BaseModel):
//     """◳ A plane is an origin (point) and an orientation (x-axis and y-axis)."""

//     origin: Point
//     xAxis: Vector
//     yAxis: Vector

//     def __init__(
//         self, origin: Point = None, xAxis: Vector = None, yAxis: Vector = None
//     ):
//         if origin is None:
//             origin = Point()
//         if xAxis is None and yAxis is None:
//             xAxis = Vector.X()
//             yAxis = Vector.Y()
//         if xAxis is None:
//             xAxis = Vector()
//         if yAxis is None:
//             yAxis = Vector()
//         if abs(xAxis.length - 1) > TOLERANCE:
//             raise ValidationError("The x-axis must be normalized.")
//         if abs(yAxis.length - 1) > TOLERANCE:
//             raise ValidationError("The y-axis must be normalized.")
//         if abs(xAxis.dot(yAxis)) > TOLERANCE:
//             raise ValidationError("The x-axis and y-axis must be orthogonal.")
//         super().__init__(origin=origin, xAxis=xAxis, yAxis=yAxis)

//     def isCloseTo(self, other: "Plane", tol: float = TOLERANCE) -> bool:
//         return (
//             self.origin.isCloseTo(other.origin, tol)
//             and self.xAxis.isCloseTo(other.xAxis, tol)
//             and self.yAxis.isCloseTo(other.yAxis, tol)
//         )

//     @property
//     def zAxis(self) -> Vector:
//         return self.xAxis.cross(self.yAxis)

//     def transform(self, transform: "Transform") -> "Plane":
//         return Transform.transformPlane(transform, self)

//     def toTransform(self) -> "Transform":
//         return Transform.fromPlane(self)

//     @staticmethod
//     def XY() -> "Plane":
//         return Plane(
//             origin=Point(),
//             xAxis=Vector.X(),
//             yAxis=Vector.Y(),
//         )

//     @staticmethod
//     def fromYAxis(yAxis: Vector, theta: float = 0.0, origin: Point = None) -> "Plane":
//         if abs(yAxis.length - 1) > TOLERANCE:
//             raise SpecificationError("The yAxis must be normalized.")
//         if origin is None:
//             origin = Point()
//         orientation = Transform.fromDirections(Vector.Y(), yAxis)
//         rotation = Transform.fromAngle(yAxis, theta)
//         xAxis = Vector.X().transform(rotation.after(orientation))
//         return Plane(origin=origin, xAxis=xAxis, yAxis=yAxis)


// class Rotation(BaseModel):
//     """🔄 A rotation is an axis and an angle."""

//     axis: Vector
//     angle: float

//     def __init__(self, axis: Vector, angle: float):
//         super().__init__(axis=axis, angle=angle)

//     def toTransform(self) -> "Transform":
//         return Transform.fromRotation(self)


// class Transform(ndarray):
//     """▦ A 4x4 translation and rotation transformation matrix (no scaling or shearing)."""

//     def __new__(cls, input_array=None):
//         if input_array is None:
//             input_array = eye(4, dtype=float)
//         else:
//             input_array = asarray(input_array).astype(float)
//         obj = input_array.view(cls)
//         return obj

//     def __array_finalize__(self, obj):
//         if obj is None:
//             return

//     def __str__(self) -> str:
//         rounded_self = self.round()
//         return f"Transform(Rotation={rounded_self.rotation}, Translation={rounded_self.translation})"

//     def __repr__(self) -> str:
//         rounded_self = self.round()
//         return f"Transform(Rotation={rounded_self.rotation}, Translation={rounded_self.translation})"

//     @property
//     def rotation(self) -> Rotation:
//         """🔄 The rotation part of the transform."""
//         rotationMatrix = self[:3, :3]
//         axisAngle = axis_angle_from_matrix(rotationMatrix)
//         return Rotation(
//             axis=Vector(
//                 float(axisAngle[0]), 
//                 float(axisAngle[1]), 
//                 float(axisAngle[2])
//             ), 
//             angle=float(degrees(axisAngle[3]))
//         )

//     @property
//     def translation(self) -> Vector:
//         """➡️ The translation part of the transform."""
//         return Vector(*self[:3, 3])

//     def after(self, before: "Transform") -> "Transform":
//         """✖️ Apply this transform after another transform.

//         Args:
//             before (Transform): Transform to apply before this transform.

//         Returns:
//             Transform: New transform.
//         """
//         return Transform(concat(before, self))

//     def invert(self) -> "Transform":
//         return Transform(invert_transform(self))

//     def transformPoint(self, point: Point) -> Point:
//         transformedPoint = transform(self, vector_to_point(point))
//         return Point(*transformedPoint[:3])

//     def transformVector(self, vector: Vector) -> Vector:
//         transformedVector = transform(self, vector_to_direction(vector))
//         return Vector(*transformedVector[:3])

//     def transformPlane(self, plane: Plane) -> Plane:
//         planeTransform = Transform.fromPlane(plane)
//         planeTransformed = planeTransform.after(self)
//         return Transform.toPlane(planeTransformed)

//     def transform(
//         self, geometry: Union[Point, Vector, Plane]
//     ) -> Union[Point, Vector, Plane]:
//         if isinstance(geometry, Point):
//             return self.transformPoint(geometry)
//         elif isinstance(geometry, Vector):
//             return self.transformVector(geometry)
//         elif isinstance(geometry, Plane):
//             return self.transformPlane(geometry)
//         else:
//             raise NotImplementedError()

//     def round(self, decimals: int = SIGNIFICANT_DIGITS) -> "Transform":
//         return Transform(super().round(decimals=decimals))

//     @staticmethod
//     def fromTranslation(vector: Vector) -> "Transform":
//         return Transform(
//             transform_from(
//                 [
//                     [1, 0, 0],
//                     [0, 1, 0],
//                     [0, 0, 1],
//                 ],
//                 vector,
//             )
//         )

//     @staticmethod
//     def fromRotation(rotation: Rotation) -> "Transform":
//         return Transform(
//             transform_from(matrix_from_axis_angle((*rotation.axis, radians(rotation.angle))), Vector())
//         )

//     @staticmethod
//     def fromPlane(plane: Plane) -> "Transform":
//         # Assumes plane is normalized
//         return Transform(
//             transform_from(
//                 [
//                     [plane.xAxis.x, plane.yAxis.x, plane.zAxis.x],
//                     [plane.xAxis.y, plane.yAxis.y, plane.zAxis.y],
//                     [plane.xAxis.z, plane.yAxis.z, plane.zAxis.z],
//                 ],
//                 plane.origin,
//             )
//         )

//     @staticmethod
//     def fromAngle(axis: Vector, angle: float) -> "Transform":
//         return Transform(
//             transform_from(matrix_from_axis_angle((*axis, radians(angle))), Vector())
//         )

//     @staticmethod
//     def fromDirections(startDirection: Vector, endDirection: Vector) -> "Transform":
//         if startDirection.isCloseTo(endDirection):
//             return Transform()
//         axisAngle = axis_angle_from_two_directions(startDirection, endDirection)
//         return Transform(transform_from(matrix_from_axis_angle(axisAngle), Vector()))

//     def toPlane(self) -> Plane:
//         return Plane(
//             origin=Point(*self[:3, 3]),
//             xAxis=Vector(
//                 self[0, 0],
//                 self[1, 0],
//                 self[2, 0],
//             ),
//             yAxis=Vector(
//                 self[0, 1],
//                 self[1, 1],
//                 self[2, 1],
//             ),
//         )

export class Point extends Vector3 implements IPoint {

    constructor(x: number = 0, y: number = 0, z: number = 0) {
        super(x, y, z)
    }

    [n: number]: number

    *[Symbol.iterator](): Iterator<number> {
        yield this.x
        yield this.y
        yield this.z
    }

    get 0(): number { return this.x }
    get 1(): number { return this.y }
    get 2(): number { return this.z }

    isCloseTo(other: Point | Vector | number, tol: number = TOLERANCE): boolean {
        if (typeof other === 'number') {
            return (
                Math.abs(this.x - other) < tol &&
                Math.abs(this.y - other) < tol &&
                Math.abs(this.z - other) < tol
            )
        } else if (other instanceof Vector) {
            return this.isCloseTo(other.toPoint(), tol)
        } else {
            return (
                Math.abs(this.x - other.x) < tol &&
                Math.abs(this.y - other.y) < tol &&
                Math.abs(this.z - other.z) < tol
            )
        }
    }

    toVector(): Vector { return new Vector(this.x, this.y, this.z) }

    static fromVector(vector: Vector): Point { return new Point(vector.x, vector.y, vector.z) }

    static parse(point: IPoint): Point {
        return new Point(point.x, point.y, point.z)
    }
}

export class Vector extends Vector3 implements IVector {

    constructor(x: number = 0, y: number = 0, z: number = 0) {
        super(x, y, z)
    }

    [n: number]: number

    *[Symbol.iterator](): Iterator<number> {
        yield this.x
        yield this.y
        yield this.z
    }

    get 0(): number { return this.x }
    get 1(): number { return this.y }
    get 2(): number { return this.z }

    sqrt(): Vector {
        return new Vector(Math.sqrt(this.x), Math.sqrt(this.y), Math.sqrt(this.z))
    }

    sign(): Vector {
        return new Vector(Math.sign(this.x), Math.sign(this.y), Math.sign(this.z))
    }

    isCloseTo(other: Vector | Point | number, tol: number = TOLERANCE): boolean {
        if (typeof other === 'number') {
            return (
                Math.abs(this.x - other) < tol &&
                Math.abs(this.y - other) < tol &&
                Math.abs(this.z - other) < tol
            )
        } else if (other instanceof Point) {
            return this.isCloseTo(other.toVector(), tol)
        } else {
            return (
                Math.abs(this.x - other.x) < tol &&
                Math.abs(this.y - other.y) < tol &&
                Math.abs(this.z - other.z) < tol
            )
        }
    }

    toPoint(): Point {
        return new Point(this.x, this.y, this.z)
    }

    toTransform(): Transform { return Transform.fromTranslation(this) }

    static X(): Vector {
        return new Vector(1, 0, 0)
    }

    static Y(): Vector {
        return new Vector(0, 1, 0)
    }

    static Z(): Vector {
        return new Vector(0, 0, 1)
    }

    static parse(vector: IVector): Vector {
        return new Vector(vector.x, vector.y, vector.z)
    }
}

export class Plane implements IPlane {
    origin: Point
    xAxis: Vector
    yAxis: Vector

    constructor(origin?: Point, xAxis?: Vector, yAxis?: Vector) {
        this.origin = origin ?? new Point();
        this.xAxis = xAxis ?? Vector.X();
        this.yAxis = yAxis ?? Vector.Y();
        if (this.xAxis.length() - 1 > TOLERANCE) {
            throw new Error("The x-axis must be normalized.")
        }
        if (this.yAxis.length() - 1 > TOLERANCE) {
            throw new Error("The y-axis must be normalized.")
        }
        if (Math.abs(this.xAxis.dot(this.yAxis)) > TOLERANCE) {
            throw new Error("The x-axis and y-axis must be orthogonal.")
        }
    }

    get zAxis(): Vector {
        return new Vector().crossVectors(this.xAxis, this.yAxis)
    }

    isCloseTo(other: Plane, tol: number = TOLERANCE): boolean {
        return (
            this.origin.isCloseTo(other.origin, tol) &&
            this.xAxis.isCloseTo(other.xAxis, tol) &&
            this.yAxis.isCloseTo(other.yAxis, tol)
        )
    }

    transform(transform: Transform): Plane {
        return transform.toPlane()
    }

    toTransform(): Transform {
        return Transform.fromPlane(this)
    }

    static XY(): Plane {
        return new Plane(new Point(), Vector.X(), Vector.Y())
    }

    static fromYAxis(yAxis: Vector, theta: number = 0, origin?: Point): Plane {
        if (yAxis.length() - 1 > TOLERANCE) {
            throw new Error("The yAxis must be normalized.")
        }
        const orientation = Transform.fromDirections(Vector.Y(), yAxis)
        const rotation = Transform.fromAngle(yAxis, theta)
        const xAxis = Vector.X().applyMatrix4(rotation.multiply(orientation))
        return new Plane(origin ?? new Point(), xAxis, yAxis)
    }

    static parse(plane: IPlane): Plane {
        return new Plane(
            Point.parse(plane.origin),
            Vector.parse(plane.xAxis),
            Vector.parse(plane.yAxis)
        )
    }
}

export class Rotation {
    axis: Vector
    angle: number

    constructor(axis?: Vector, angle: number = 0) {
        this.axis = axis ?? Vector.Z()
        this.angle = angle
    }

    toTransform(): Transform {
        return Transform.fromRotation(this)
    }
}

export class Transform extends Matrix4 {
    constructor() {
        super()
    }
    get rotation(): Rotation {
        //https://github.com/dfki-ric/pytransform3d/blob/c45e817c4a7960108afe9f5259542c8376c0e89a/pytransform3d/rotations/_conversions.py#L1719
        const rotationMatrix = new Matrix4()
        rotationMatrix.extractRotation(this)
        const trace = rotationMatrix.elements[0] + rotationMatrix.elements[5] + rotationMatrix.elements[10]
        const cosAngle = (trace - 1) / 2
        const angle = Math.acos(Math.min(Math.max(-1, cosAngle), 1))
        if (angle === 0) return new Rotation()
        const axisUnnormalized = new Vector(
            rotationMatrix[6] - rotationMatrix[9],
            rotationMatrix[8] - rotationMatrix[2],
            rotationMatrix[1] - rotationMatrix[4]
        )
        let axis: Vector
        if (Math.abs(angle - Math.PI) < 1e-4) {
            const clampedDiagonal = new Vector(rotationMatrix[0], rotationMatrix[5], rotationMatrix[10]).clampScalar(-1, 1)
            const eeTDiag = clampedDiagonal.clone().addScalar(1).multiplyScalar(0.5)
            const signs = axisUnnormalized.clone().sign()
            const unitizedSigns = new Vector(signs.x || 1, signs.y || 1, signs.z || 1)
            axis = eeTDiag.sqrt().multiply(unitizedSigns)
        } else {
            axis = axisUnnormalized
        }
        const normalizedAxis = axis.normalize()
        return new Rotation(normalizedAxis, angle)
    }

    get translation(): Vector {
        return new Vector(this[12], this[13], this[14])
    }

    after(before: Transform): Transform {
        return new Transform().multiplyMatrices(before, this)
    }

    transformPoint(point: Point): Point {
        const transformedPoint = point.clone().applyMatrix4(this)
        return new Point(transformedPoint.x, transformedPoint.y, transformedPoint.z)
    }

    transformVector(vector: Vector): Vector {
        const transformedVector = vector.clone().applyMatrix4(this)
        return new Vector(transformedVector.x, transformedVector.y, transformedVector.z)
    }

    transformPlane(plane: Plane): Plane {
        const planeTransform = Transform.fromPlane(plane)
        const planeTransformed = planeTransform.after(this)
        return planeTransformed.toPlane()
    }

    transform(geometry: Point | Vector | Plane): Point | Vector | Plane {
        if (geometry instanceof Point) {
            return this.transformPoint(geometry)
        } else if (geometry instanceof Vector) {
            return this.transformVector(geometry)
        } else if (geometry instanceof Plane) {
            return this.transformPlane(geometry)
        } else {
            throw new Error("Not implemented")
        }
    }

    static fromTranslation(vector: Vector): Transform {
        return new Transform().makeTranslation(vector.x, vector.y, vector.z)
    }

    static fromRotation(rotation: Rotation): Transform {
        return new Transform().makeRotationAxis(rotation.axis, radians(rotation.angle))
    }

    static fromPlane(plane: Plane): Transform {
        return new Transform().makeBasis(plane.xAxis, plane.yAxis, plane.zAxis)
            .setPosition(plane.origin)
    }

    static fromAngle(axis: Vector, angle: number): Transform {
        return new Transform().makeRotationAxis(axis, radians(angle))
    }

    static fromDirections(startDirection: Vector, endDirection: Vector): Transform {
        if (startDirection.isCloseTo(endDirection)) {
            return new Transform()
        }
        const axisAngle = new Vector3().crossVectors(startDirection, endDirection)
        return new Transform().makeRotationAxis(axisAngle.normalize(), startDirection.angleTo(endDirection))
    }

    toPlane(): Plane {
        const origin = new Point(this[12], this[13], this[14])
        const xAxis = new Vector(this[0], this[1], this[2])
        const yAxis = new Vector(this[4], this[5], this[6])
        return new Plane(origin, xAxis, yAxis)
    }
}

export const semioToThreeRotation = (): Matrix4 => {
    return new Matrix4().set(
        1, 0, 0, 0,
        0, 0, 1, 0,
        0, -1, 0, 0,
        0, 0, 0, 1)
}

export const threeToSemioRotation = (): Matrix4 => {
    return new Matrix4().set(
        1, 0, 0, 0,
        0, 0, -1, 0,
        0, 1, 0, 0,
        0, 0, 0, 1)
}


export const radians = (degrees: number): number => {
    return degrees * (Math.PI / 180);
}

export type Hierarchy = {
    piece: Piece | PieceInput
    transform: Transform
    children: Hierarchy[]
}

// Reference in Python:
// def formationToHierarchies(formation: Formation) -> List[Hierarchy]:
//     nodes = list((piece.localId, {"piece": piece}) for piece in formation.pieces)
//     edges = (
//         (
//             connection.connecting.piece.id,
//             connection.connected.piece.id,
//             {"connection": connection},
//         )
//         for connection in formation.connections
//     )
//     graph = Graph()
//     graph.add_nodes_from(nodes)
//     graph.add_edges_from(edges)
//     hierarchies = []
//     for componentGenerator in connected_components(graph):
//         component = graph.subgraph(componentGenerator)
//         try:
//             root = [
//                 node for node in component.nodes() if graph.nodes[node]["piece"].root
//             ][0]
//         except IndexError:
//             root = next(iter(component.nodes))
//         rootHierarchy = Hierarchy(
//             piece=graph.nodes[root]["piece"],
//             transform=Transform(),
//             children=[],
//         )
//         component.nodes[root]["hierarchy"] = rootHierarchy
//         for parent, child in bfs_tree(component, source=root).edges():
//             connection = component[parent][child]["connection"]
//             connectedIsParent = connection.connected.piece.id == parent
//             parentPort = connection.connected.piece.type.port if connectedIsParent else connection.connecting.piece.type.port
//             childPort = connection.connecting.piece.type.port if connectedIsParent else connection.connected.piece.type.port
//             orient = Transform.fromDirections(childPort.direction.revert(), parentPort.direction)
//             rotation = orient
//             if connection.rotation != 0.0:
//                 rotate = Transform.fromAngle(parentPort.direction, connection.rotation)
//                 rotation = rotate.after(orient)
//             centerConnecting = childPort.point.toVector().revert().toTransform()
//             moveToConnected = parentPort.point.toVector().toTransform()
//             transform = rotation.after(centerConnecting)
//             if connection.offset != 0.0:
//                 offset = parentPort.direction.amplify(connection.offset).toTransform()
//                 transform = offset.after(transform)
//             transform = moveToConnected.after(transform)
//             hierarchy = Hierarchy(
//                 piece=component.nodes[child]["piece"],
//                 transform=transform,
//                 children=[],
//             )
//             component.nodes[child]["hierarchy"] = hierarchy
//             component.nodes[parent]["hierarchy"].children.append(hierarchy)
//         hierarchies.append(rootHierarchy)
//     return hierarchies
export const formationToHierarchies = (
    formation: Formation | FormationInput,
    ports: Map<string, Map<string, Map<string, Port>>> // typeName -> typeVariant -> portId -> port
): Hierarchy[] => {
    if (formation.pieces.length === 0) return []
    const cy = cytoscape({
        elements: {
            nodes: formation.pieces.map((piece) => ({
                data: { id: piece.id, label: piece.id }
            })),
            edges: formation.connections.map((connection) => ({
                data: {
                    id: `${connection.connecting.piece.id}-${connection.connected.piece.id}`,
                    source: connection.connected.piece.id,
                    target: connection.connecting.piece.id
                }
            }))
        }
    })
    const hierarchies: Hierarchy[] = []
    const components = cy.elements().components()
    components.forEach((component) => {
        const roots = component
            .nodes()
            .filter((node) => formation.pieces.find((p) => p.id === node.id()).root)
        const root = roots.length === 0 ? component.nodes()[0] : roots[0]
        const rootPiece = formation.pieces.find((p) => p.id === root.id())
        const rootHierarchy: Hierarchy = {
            piece: rootPiece,
            transform: new Transform(),
            children: []
        }
        hierarchies.push(rootHierarchy)
        const pieceIdToHierarchy: { [key: string]: Hierarchy } = {}
        pieceIdToHierarchy[rootHierarchy.piece.id] = rootHierarchy
        cy.elements().bfs(
            {
                root,
                visit: (v, e, u, i, depth) => {
                    if (depth === 0) return
                    const parentId = u.id()
                    const childId = v.id()
                    const connection = formation.connections.find(
                        (connection) =>
                            connection.connected.piece.id === e.source().id() &&
                            connection.connecting.piece.id === e.target().id()
                    )
                    const connectedIsParent = connection.connected.piece.id === parentId
                    const parentPiece = formation.pieces.find((p) => p.id === parentId)
                    const childPiece = formation.pieces.find((p) => p.id === childId)
                    const parentPortId = connectedIsParent
                        ? connection.connected.piece?.type.port?.id
                        : connection.connecting.piece?.type.port?.id
                    const childPortId = connectedIsParent
                        ? connection.connecting.piece?.type.port?.id
                        : connection.connected.piece?.type.port?.id
                    const parentPort = ports
                        .get(parentPiece.type.name)
                        ?.get(parentPiece.type.variant ?? '')
                        ?.get(parentPortId ?? '')
                    const childPort = ports
                        .get(childPiece.type.name)
                        ?.get(childPiece.type.variant ?? '')
                        ?.get(childPortId ?? '')
                    const parentDirection = Vector.parse(parentPort.direction)
                    const childDirection = Vector.parse(childPort.direction)
                    const parentPoint = Point.parse(parentPort.point)
                    const childPoint = Point.parse(childPort.point)
                    const orient = Transform.fromDirections(childDirection.negate(), parentDirection)
                    let rotation = orient
                    if (connection.rotation !== 0) {
                        const rotate = Transform.fromAngle(parentDirection, connection.rotation)
                        rotation = rotate.after(orient)
                    }
                    const centerChild = childPoint.toVector().negate().toTransform()
                    const moveToParent = parentPoint.toVector().toTransform()
                    let transform = new Transform()
                    transform = transform.after(rotation)
                    if (connection.offset !== 0) {
                        const offset = parentDirection.clone().multiplyScalar(connection.offset).toTransform()
                        transform = transform.after(offset)
                    }
                    transform.after(moveToParent)
                    const hierarchy = {
                        piece: childPiece,
                        transform,
                        children: []
                    }
                    console.log(hierarchy, transformToPlane(transform), rotation, centerChild, moveToParent)
                    pieceIdToHierarchy[childPiece.id] = hierarchy
                    pieceIdToHierarchy[parentPiece.id].children.push(hierarchy)
                }
            }
        )
    })
    return hierarchies
}
