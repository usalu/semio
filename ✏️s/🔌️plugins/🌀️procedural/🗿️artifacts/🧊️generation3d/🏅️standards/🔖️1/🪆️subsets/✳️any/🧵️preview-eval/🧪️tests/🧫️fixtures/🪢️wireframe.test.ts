import { expect, test } from "bun:test";
import { BufferGeometry, Float32BufferAttribute, Points, PointsMaterial, WireframeGeometry } from "three";
import fixture from "./🪢️wireframe.json";

test("wireframe keeps isolated points and matches the indexed surface edges", () => {
    const pointGeometry = new BufferGeometry();
    const pointMaterial = new PointsMaterial();
    const surface = new BufferGeometry();
    let wire: WireframeGeometry | undefined;
    try {
        pointGeometry.setAttribute("position", new Float32BufferAttribute(fixture.point.positions, 3));
        const point = new Points(pointGeometry, pointMaterial);
        point.geometry.computeBoundingBox();
        expect(point.geometry.boundingBox?.min.toArray()).toEqual(fixture.point.positions);
        expect(point.geometry.boundingBox?.max.toArray()).toEqual(fixture.point.positions);
        expect(fixture.point.componentRefs.vertex.map(value => BigInt(value).toString())).toEqual(fixture.point.componentRefs.vertex);
        surface.setAttribute("position", new Float32BufferAttribute(fixture.positions, 3));
        surface.setIndex(fixture.indices);
        wire = new WireframeGeometry(surface);
        const undirected = (positions: ArrayLike<number>) => {
            const segments = [];
            for (let offset = 0; offset < positions.length; offset += 6) {
                const vertex = (start: number) => Array.from({ length: 3 }, (_, axis) => positions[start + axis]).join(",");
                segments.push([vertex(offset), vertex(offset + 3)].sort().join("|"));
            }
            return segments.sort();
        };
        expect(undirected(wire.getAttribute("position").array)).toEqual(undirected(fixture.edges));
    } finally {
        pointGeometry.dispose();
        pointMaterial.dispose();
        surface.dispose();
        wire?.dispose();
    }
});
