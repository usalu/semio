import cytoscape from 'cytoscape'
import { Matrix4, Vector3 } from 'three'
import {
    Point as IPoint,
    Vector as IVector,
    Plane as IPlane,
    VectorInput,
    Formation,
    FormationInput,
    PlaneInput,
    Port,
    PointInput
} from './semio.d'

export type {
    VectorInput,
    Formation,
    FormationInput,
    PlaneInput,
    Port,
    PointInput
}

export const TOLERANCE = 1e-5

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

    toVector(): Vector { return new Vector(this.x, this.y, this.z) }

    static fromVector(vector: Vector): Point { return new Point(vector.x, vector.y, vector.z) }
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
}

export class Plane implements IPlane {
    origin: Point
    xAxis: Vector
    yAxis: Vector

    constructor(origin: Point, xAxis: Vector, yAxis: Vector) {
        this.origin = origin
        this.xAxis = xAxis
        this.yAxis = yAxis
    }

    get zAxis(): Vector {
        return new Vector().crossVectors(this.xAxis, this.yAxis)
    }

    isClose(other: Plane, tol: number = TOLERANCE): boolean {
        return (
            this.origin.isClose(other.origin, tol) &&
            this.xAxis.isClose(other.xAxis, tol) &&
            this.yAxis.isClose(other.yAxis, tol)
        )
    }

    transform(transform: Transform): Plane {
        return transform.toPlane()
    }

    toTransform(): Transform {
        return Transform.fromPlane(this)
    }

    static XY(): Plane {
        return new Plane(new Point(0, 0, 0), Vector.X(), Vector.Y())
    }

    static fromYAxis(yAxis: Vector, theta: number = 0, origin: Point = new Point(0, 0, 0)): Plane {
        if (yAxis.length() - 1 > TOLERANCE) {
            throw new Error("The yAxis must be normalized.")
        }
        const orientation = Transform.fromDirections(Vector.Y(), yAxis)
        const rotation = Transform.fromAngle(yAxis, theta)
        const xAxis = Vector.X().applyMatrix4(rotation.multiply(orientation))
        return new Plane(origin, xAxis, yAxis)
    }
}

export class Rotation {
    axis: Vector
    angle: number

    constructor(axis: Vector, angle: number) {
        this.axis = axis
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
    pieceId: string
    plane: Plane | PlaneInput
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
        const rootId = root.id()
        const rootHierarchy: Hierarchy = {
            pieceId: rootId,
            plane: formation.pieces.find((p) => p.id === rootId).root?.plane ?? {
                origin: { x: 0, y: 0, z: 0 },
                xAxis: { x: 1, y: 0, z: 0 },
                yAxis: { x: 0, y: 1, z: 0 }
            },
            children: []
        }
        hierarchies.push(rootHierarchy)
        const pieceIdToHierarchy: { [key: string]: Hierarchy } = {}
        pieceIdToHierarchy[rootHierarchy.pieceId] = rootHierarchy
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
                    const invertedChildDirection = vectorToVector3(childPort.direction).negate()
                    const parentDirection = vectorToVector3(parentPort.direction)
                    const rotation = new Matrix4()
                    const orientAxis = new Vector3().crossVectors(invertedChildDirection, parentDirection)
                    if (orientAxis.length() > TOLERANCE) {
                        const orient = new Matrix4().makeRotationAxis(orientAxis.normalize(), invertedChildDirection.angleTo(parentDirection))
                        rotation.premultiply(orient)
                    }
                    if (connection.rotation !== 0) {
                        const rotate = new Matrix4().makeRotationAxis(parentDirection, radians(connection.rotation))
                        rotation.premultiply(rotate)
                    }
                    const centerConnecting = new Matrix4().makeTranslation(pointToVector3(childPort.point).negate())
                    const moveToConnected = new Matrix4().makeTranslation(pointToVector3(parentPort.point))
                    const transform = new Matrix4()
                    transform.premultiply(centerConnecting)
                    transform.premultiply(rotation)
                    if (connection.offset !== 0) {
                        const offset = new Matrix4().makeTranslation(parentDirection.clone().multiplyScalar(connection.offset))
                        transform.premultiply(offset)
                    }
                    transform.premultiply(moveToConnected)
                    const hierarchy = {
                        pieceId: childPiece.id,
                        plane: transformToPlane(transform),
                        children: []
                    }
                    console.log(hierarchy, transformToPlane(transform), rotation, centerConnecting, moveToConnected)
                    pieceIdToHierarchy[childPiece.id] = hierarchy
                    pieceIdToHierarchy[parentPiece.id].children.push(hierarchy)
                }
            }
        )
    })
    return hierarchies
}
