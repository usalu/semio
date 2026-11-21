import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { EllipseCurve, Line3, Triangle, Vector3 } from "three";

const fixture = JSON.parse(readFileSync("✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️mixed-section-distance.json", "utf8"));
const vector = (value) => new Vector3(...value);
const subdivisions = 8192;

function section(row) {
  const center = vector(row.center);
  const u = vector(row.axisU);
  const v = vector(row.axisV);
  const scale = 1 / u.x;
  const world = (x, y) => center.clone().addScaledVector(u, x).addScaledVector(v, y);
  const ring = [];
  if (row.kind === "cylinder") {
    const intervals = row.height === 4 ? [[-2 * Math.PI / 3, 2 * Math.PI / 3]]
      : [[Math.PI / 3, 2 * Math.PI / 3], [4 * Math.PI / 3, 5 * Math.PI / 3]];
    for (const [lo, hi] of intervals) {
      const arc = new EllipseCurve(0, 0, scale * row.radius, row.radius, lo, hi);
      for (const point of arc.getPoints(subdivisions)) ring.push(world(point.x, point.y));
    }
  } else {
    const slope = row.normal[0];
    const cap = (row.planeOrigin[2] - row.origin[2]) / slope;
    const halfWidth = Math.sqrt(row.radius ** 2 - cap ** 2);
    const zeroRadius = row.radius * (1 - (row.planeOrigin[2] - row.origin[2]) / row.height);
    const taper = slope * row.radius / row.height;
    const a = taper * taper - 1;
    const b = 2 * taper * zeroRadius;
    for (let i = 0; i <= subdivisions; i++) {
      const y = -halfWidth + 2 * halfWidth * i / subdivisions;
      const c = zeroRadius * zeroRadius - y * y;
      const x = a === 0 ? -c / b : -2 * c / (b + Math.sqrt(b * b - 4 * a * c));
      ring.push(world(scale * x, y));
    }
    const capChord = new Line3(ring.at(-1), ring[0]);
    assert.ok(capChord.distance() > 0);
  }
  return ring.map((point, i) => new Triangle(center, point, ring[(i + 1) % ring.length]));
}

for (const row of fixture.sections) {
  const triangles = section(row);
  const area = triangles.reduce((sum, triangle) => sum + triangle.getArea(), 0);
  assert.ok(Math.abs(area - row.area) < 2e-6, row.kind + " area");
  for (const query of row.queries) {
    const target = vector(query.target);
    let best;
    for (const triangle of triangles) {
      const point = triangle.closestPointToPoint(target, new Vector3());
      if (!best || point.distanceToSquared(target) < best.distanceToSquared(target)) best = point;
    }
    assert.ok(best.distanceTo(vector(query.point)) < 2e-6, query.id);
    assert.ok(Math.abs(best.distanceTo(target) - query.distance) < 2e-7, query.id);
    if (query.id.startsWith("near external")) assert.ok(best.distanceTo(target) > query.distance * 0.5, query.id);
  }
}
