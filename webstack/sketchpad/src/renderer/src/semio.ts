import cytoscape from 'cytoscape'
import { Matrix4, Vector3 as ThreeVector } from 'three'
import { Vector, VectorInput, Formation, FormationInput, Plane, PlaneInput, Port, Point, PointInput } from './semio.d'

type Hierarchy = {
    pieceId: string
    transform: Matrix4
    children: Hierarchy[]
}

const semioToThreeRotation = (): Matrix4 => {
    return new Matrix4().set(
        1, 0, 0, 0,
        0, 0, 1, 0,
        0, -1, 0, 0,
        0, 0, 0, 1
    )
}

const threeToSemioRotation = (): Matrix4 => {
    return new Matrix4().set(
        1, 0, 0, 0,
        0, 0, -1, 0,
        0, 1, 0, 0,
        0, 0, 0, 1
    )
}

const convertPointToThreeVector = (point: Point | PointInput): ThreeVector => {
    return new ThreeVector(point.x, point.z, -point.y)
}

const convertVectorToThreeVector = (vector: Vector | VectorInput): ThreeVector => {
    return new ThreeVector(vector.x, vector.z, -vector.y)
}

const convertThreeVectorToPoint = (vector: ThreeVector): Point => {
    return { x: vector.x, y: -vector.z, z: vector.y }
}

const convertThreeVectorToVector = (vector: ThreeVector): Vector => {
    return { x: vector.x, y: -vector.z, z: vector.y }
}

/**
 * Convert a semio plane to a three.js transform.
 * @param plane plane in the semio coordinate system.
 * @returns transform in the three.js coordinate system.
 */
export const convertPlaneToTransform = (plane: Plane | PlaneInput): Matrix4 => {
    const inverseExportRotation = threeToSemioRotation()
    const newOrigin = convertPointToThreeVector(plane.origin)
    const newXAxis = convertVectorToThreeVector(plane.xAxis).applyMatrix4(inverseExportRotation)
    const newYAxis = convertVectorToThreeVector(plane.yAxis).applyMatrix4(inverseExportRotation)
    const newZAxis = newXAxis.clone().cross(newYAxis)
    return new Matrix4().set(
        newXAxis.x, newYAxis.x, newZAxis.x, newOrigin.x,
        newXAxis.y, newYAxis.y, newZAxis.y, newOrigin.y,
        newXAxis.z, newYAxis.z, newZAxis.z, newOrigin.z,
        0, 0, 0, 1
    )
}

export const convertTransformToPlane = (transform: Matrix4): Plane => {
    const exportRotation = semioToThreeRotation()
    const origin = convertThreeVectorToPoint(new ThreeVector(transform.elements[12], transform.elements[13], transform.elements[14]))
    const xAxis = convertThreeVectorToVector(new ThreeVector(transform.elements[0], transform.elements[1], transform.elements[2]).applyMatrix4(exportRotation))
    const yAxis = convertThreeVectorToVector(new ThreeVector(transform.elements[4], transform.elements[5], transform.elements[6]).applyMatrix4(exportRotation))
    return { origin, xAxis, yAxis }
}

export const formationToHierarchies = (
    formation: Formation | FormationInput,
    ports: Map<string, Map<string, Map<string,Port>>> // typeName -> typeVariant -> portId -> port
): Hierarchy[] => {
    if (formation.pieces.length === 0) return []
    const cy = cytoscape({
        elements: {
            nodes: formation.pieces.map((piece) => ({
                data: { id: piece.id, label: piece.id }
            })),
            edges: formation.attractions.map((attraction) => ({
                data: {
                    id: `${attraction.attracting.piece.id}->${attraction.attracted.piece.id}`,
                    source: attraction.attracting.piece.id,
                    target: attraction.attracted.piece.id
                }
            }))
        }
    })
    const hierarchies: Hierarchy[] = []
    const components = cy.elements().components()
    components.forEach((component) => {
        const roots = component.roots()
        const root = roots.length === 0 ? component.nodes()[0] : roots[0]
        const { path } = cy.elements().bfs({
            root,
            directed: true
        })
        // path[i] are the nodes. path[i-1] are the edges.
        // path[0] is the root. path[length-1] is the last node.
        const rootHierarchy: Hierarchy = {
            pieceId: path[0].id(),
            transform: convertPlaneToTransform(
                formation.pieces.find((p) => p.id === path[0].id()).root?.plane ?? {
                    origin: { x: 0, y: 0, z: 0 },
                    xAxis: { x: 1, y: 0, z: 0 },
                    yAxis: { x: 0, y: 1, z: 0 }
                }
            ),
            children: []
        }
        hierarchies.push(rootHierarchy)
        const pieceIdToHierarchy: { [key: string]: Hierarchy } = {}
        pieceIdToHierarchy[rootHierarchy.pieceId] = rootHierarchy
        for (let i = 2; i < path.length; i += 2) {
            const edge = path[i - 1]
            const parentPieceId = edge.source().id()
            const parentPiece = formation.pieces.find((p) => p.id === parentPieceId)
            const pieceId = edge.target().id()
            const piece = formation.pieces.find((p) => p.id === pieceId)
            const attraction = formation.attractions.find(
                (attraction) =>
                attraction.attracting.piece.id === parentPieceId &&
                attraction.attracted.piece.id === pieceId
                )
            const parentPort = ports.get(parentPiece.type.name)?.get(parentPiece.type.variant ?? '')?.get(attraction.attracting.piece?.type?.port?.id ?? '')
            const childPort = ports.get(piece.type.name)?.get(piece.type.variant ?? '')?.get(attraction.attracted.piece?.type?.port?.id ?? '')
            const parentTransform = convertPlaneToTransform(parentPort.plane)
            const childTransform = convertPlaneToTransform(childPort.plane).invert()
            const hierarchy = {
                pieceId,
                transform: parentTransform.multiply(childTransform),
                children: []
            }
            pieceIdToHierarchy[pieceId] = hierarchy
            pieceIdToHierarchy[parentPiece.id].children.push(hierarchy)
        }
    })
    // console.log('test', 
    // convertTransformToPlane(
    //     convertThreeTransformToSemioTransform(
    //         convertSemioTransformToThreeTransform(
    //             convertPlaneToTransform({origin: {x: 0, y: 0, z: 0}, xAxis: {x: 1, y: 0, z: 0}, yAxis: {x: 0, y: 1, z: 0}})))))
    return hierarchies
}