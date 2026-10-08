/** 🧰️ Third-party oracle of the construction computes (`brep.curve`, `brep.surface`, `brep.solid`, `brep.boolean`): rebuilds every fixture widget recipe from its inputs with three (curves, shape areas, quaternions, triangles, lathe geometry) and manifold-3d (booleans), measures the result with plain sums and compares it with what the Rust kernel computed. Independent of the B-Rep kernel by construction. */
import { Box3, CatmullRomCurve3, EllipseCurve, LatheGeometry, LineCurve3, Matrix4, Plane, Quaternion, ShapeUtils, Triangle, Vector2, Vector3 } from "three";
import { manifoldOf, Manifold, trianglesOfManifold, type Recipe } from "../🧰️oracle-support/🧊️shapes/🟦️.ts";
import type { FixtureCase, OracleResult } from "../🧰️oracle-support/🟦️.ts";

const PREFIX = "generation3d.geometry.";
const TAU = Math.PI * 2;
const Z = new Vector3(0, 0, 1);

/** 🚫️ A refusal the Rust compute states as a fault: code and port. */
export class Refusal extends Error {
  constructor(readonly code: string, readonly port?: string) {
    super(code);
  }
}
const refuse = (code: string, port?: string): never => {
  throw new Refusal(`${PREFIX}${code}`, port);
};

type Mesh3 = ReturnType<typeof Manifold.cube>;
type Triple = [number, number, number];
type Inputs = Record<string, any>;
const vec = (value: unknown): Vector3 => new Vector3(...(value as Triple));
const vecs = (value: unknown): Vector3[] => (value as Triple[]).map(vec);
const same = (a: Vector3, b: Vector3) => a.distanceTo(b) <= 1e-9;

/** 🧱️ What a recipe builds. A curve or wire is a sampled polyline, a face a planar outline, a surface a sampler, a solid a closed triangle soup. */
export type Body =
  | { kind: "curve" | "wire"; points: Vector3[]; closed: boolean; bounds?: [number, number] }
  | { kind: "face"; loop: Vector3[] }
  | { kind: "surface"; sample: (u: number, v: number) => Vector3; plane?: Plane; samples?: Vector3[] }
  | { kind: "solid"; triangles: Triangle[]; faces?: number; manifold?: Mesh3; analytic?: { volume?: number; area?: number } };

//#region 🔖️Plane
const newell = (loop: Vector3[]): Vector3 => {
  const normal = new Vector3();
  loop.forEach((a, index) => {
    const b = loop[(index + 1) % loop.length]!;
    normal.x += (a.y - b.y) * (a.z + b.z);
    normal.y += (a.z - b.z) * (a.x + b.x);
    normal.z += (a.x - b.x) * (a.y + b.y);
  });
  return normal;
};

/** 📐️ The unit winding normal of an outline; refuses an outline of no area or beyond its plane. */
const outlineNormal = (loop: Vector3[], degenerate: string, planar: string, port?: string): Vector3 => {
  const twice = newell(loop);
  const centroid = loop.reduce((sum, point) => sum.add(point), new Vector3()).divideScalar(loop.length);
  const extent = Math.max(...loop.map(point => point.distanceTo(centroid)));
  if (twice.length() <= 1e-12 * Math.max(1, extent) ** 2) refuse(degenerate, port);
  const unit = twice.clone().normalize();
  if (loop.some(point => Math.abs(point.clone().sub(centroid).dot(unit)) > 1e-6 * Math.max(1, extent))) refuse(planar, port);
  return unit;
};

/** 📏️ The area of a planar outline: three's ShapeUtils on the outline turned flat by a quaternion. */
const outlineArea = (loop: Vector3[]): number => {
  const flat = new Quaternion().setFromUnitVectors(newell(loop).normalize(), Z);
  return Math.abs(ShapeUtils.area(loop.map(point => { const turned = point.clone().applyQuaternion(flat); return new Vector2(turned.x, turned.y); })));
};
//#endregion 🔖️Plane

//#region 🔖️Measures
const pathLength = (points: Vector3[]) => points.slice(1).reduce((sum, point, index) => sum + point.distanceTo(points[index]!), 0);

const signedVolume = (triangles: Triangle[]) => triangles.reduce((sum, { a, b, c }) => sum + a.dot(new Vector3().crossVectors(b, c)) / 6, 0);

/** 🧭️ Turns every triangle of a convex solid so that its normal points away from the solid's centre. */
const outward = (triangles: Triangle[]): Triangle[] => {
  const points = triangles.flatMap(triangle => [triangle.a, triangle.b, triangle.c]);
  const centre = points.reduce((sum, point) => sum.add(point), new Vector3()).divideScalar(points.length);
  return triangles.map(triangle => {
    const middle = new Vector3().add(triangle.a).add(triangle.b).add(triangle.c).divideScalar(3);
    return triangle.getNormal(new Vector3()).dot(middle.sub(centre)) >= 0 ? triangle : new Triangle(triangle.a, triangle.c, triangle.b);
  });
};

const fan = (loop: Vector3[]): Triangle[] => loop.slice(1, -1).map((point, index) => new Triangle(loop[0]!, point, loop[index + 2]!));

const box = (points: Vector3[]): [Triple, Triple] => {
  const bounds = new Box3().setFromPoints(points);
  return [bounds.min.toArray() as Triple, bounds.max.toArray() as Triple];
};

/** ⚖️ The measures a fixture states for a body, each recomputed independently; `expected` carries the stated values where an oracle can only bound them (splines). */
export const measure = (body: Body, wanted: string[], expected: Record<string, any>): Record<string, unknown> => {
  const out: Record<string, unknown> = {};
  for (const key of wanted) {
    switch (key) {
      case "kind": out[key] = body.kind; break;
      case "length": {
        if (body.kind !== "curve" && body.kind !== "wire") throw new Error("length of a non-curve");
        const length = pathLength(body.points);
        out[key] = body.bounds === undefined ? length : length >= body.bounds[0] && length <= body.bounds[1] ? expected.length : length;
        break;
      }
      case "closed": out[key] = (body as { closed: boolean }).closed; break;
      case "start": out[key] = (body as { points: Vector3[] }).points[0]!.toArray(); break;
      case "end": { const { points } = body as { points: Vector3[] }; out[key] = points[points.length - 1]!.toArray(); break; }
      case "through": {
        const { points } = body as { points: Vector3[] };
        out[key] = (expected.through as Triple[]).every(target => Math.min(...points.map(point => point.distanceTo(vec(target)))) < 0.02) ? expected.through : [];
        break;
      }
      case "bbox":
        out[key] = body.kind === "solid" ? box(body.triangles.flatMap(triangle => [triangle.a, triangle.b, triangle.c])) : body.kind === "face" ? box(body.loop) : body.kind === "surface" ? box(body.samples ?? []) : box(body.points);
        break;
      case "area":
        out[key] = body.kind === "solid" ? body.analytic?.area ?? (body.manifold ? body.manifold.surfaceArea() : body.triangles.reduce((sum, triangle) => sum + triangle.getArea(), 0)) : outlineArea((body as { loop: Vector3[] }).loop);
        break;
      case "volume": out[key] = body.kind === "solid" ? body.analytic?.volume ?? (body.manifold ? body.manifold.volume() : Math.abs(signedVolume(body.triangles))) : NaN; break;
      case "faces": out[key] = (body as { faces?: number }).faces; break;
      case "corners": { const { sample } = body as Extract<Body, { kind: "surface" }>; out[key] = [sample(0, 0), sample(1, 0), sample(0, 1), sample(1, 1)].map(point => point.toArray()); break; }
      case "center": out[key] = (body as Extract<Body, { kind: "surface" }>).sample(0.5, 0.5).toArray(); break;
      case "plane": {
        const { plane, samples } = body as Extract<Body, { kind: "surface" }>;
        out[key] = plane && samples!.every(point => Math.abs(plane.distanceToPoint(point)) < 1e-9) ? expected.plane : null;
        break;
      }
      default: throw new Error(`no measure for ${key}`);
    }
  }
  return out;
};
//#endregion 🔖️Measures

//#region 🔖️Curves
/** 🧭️ The local X axis of a plane: the world X axis projected into it, the world Y axis when the normal lies along X. */
const localX = (normal: Vector3): Vector3 => {
  const reference = Math.abs(normal.x) < 0.9 ? new Vector3(1, 0, 0) : new Vector3(0, 1, 0);
  return reference.sub(normal.clone().multiplyScalar(reference.dot(normal))).normalize();
};

const framed = (center: Vector3, normal: Vector3, points: Vector2[]): Vector3[] => {
  const x = localX(normal);
  const turn = new Quaternion().setFromRotationMatrix(new Matrix4().makeBasis(x, new Vector3().crossVectors(normal, x), normal));
  return points.map(point => new Vector3(point.x, point.y, 0).applyQuaternion(turn).add(center));
};

const unitNormal = (value: unknown, port: string): Vector3 => {
  const normal = vec(value);
  if (normal.length() <= 1e-12) refuse("degenerate-direction", port);
  return normal.normalize();
};

const positive = (value: number, port: string) => {
  if (!(value > 0)) refuse("degenerate-size", port);
  return value;
};

const adjacentCoincide = (points: Vector3[]) => points.some((point, index) => index > 0 && same(point, points[index - 1]!));

const splineBody = (points: Vector3[]): Body => {
  const spline = new CatmullRomCurve3(points, false, "centripetal");
  spline.arcLengthDivisions = 4000;
  const sampled = spline.getPoints(2048);
  const length = spline.getLength();
  return { kind: "curve", points: [points[0]!, ...sampled.slice(1, -1), points[points.length - 1]!], closed: false, bounds: [length * 0.85, length * 1.15] };
};

const curveBody = (kind: string, inputs: Inputs): Body => {
  switch (kind) {
    case "line": {
      const [start, end] = [vec(inputs.start), vec(inputs.end)];
      if (same(start, end)) refuse("degenerate-curve", "end");
      return { kind: "curve", points: [start, end], closed: false };
    }
    case "circle": {
      const normal = unitNormal(inputs.normal, "normal");
      const radius = positive(inputs.radius, "radius");
      const points = framed(vec(inputs.center), normal, new EllipseCurve(0, 0, radius, radius, 0, TAU, false, 0).getPoints(4096));
      return { kind: "curve", points, closed: false };
    }
    case "arc": {
      const normal = unitNormal(inputs.normal, "normal");
      const radius = positive(inputs.radius, "radius");
      const span = inputs.endAngle - inputs.startAngle;
      if (Math.abs(span) <= 1e-12) refuse("degenerate-curve", "endAngle");
      if (Math.abs(span) > TAU + 1e-9) refuse("arc-span", "endAngle");
      const end = span < 0 ? inputs.endAngle + TAU * Math.ceil(-span / TAU) : inputs.endAngle;
      const points = framed(vec(inputs.center), normal, new EllipseCurve(0, 0, radius, radius, inputs.startAngle, end, false, 0).getPoints(4096));
      return { kind: "curve", points, closed: false };
    }
    case "ellipse": {
      const normal = unitNormal(inputs.normal, "normal");
      const [major, minor] = [positive(inputs.semiMajor, "semiMajor"), positive(inputs.semiMinor, "semiMinor")];
      return { kind: "curve", points: framed(vec(inputs.center), normal, new EllipseCurve(0, 0, major, minor, 0, TAU, false, 0).getPoints(4096)), closed: false };
    }
    case "polyline": {
      const points = vecs(inputs.points);
      if (points.length < 2) refuse("too-few-points", "points");
      const closed = points.length >= 4 && same(points[0]!, points[points.length - 1]!);
      const corners = closed ? points.slice(0, -1) : points;
      if (adjacentCoincide(corners) || (closed && same(corners[0]!, corners[corners.length - 1]!))) refuse("degenerate-curve", "points");
      return { kind: "wire", points: closed ? [...corners, corners[0]!] : corners, closed };
    }
    case "rectangle": {
      const [width, height] = [positive(inputs.width, "width"), positive(inputs.height, "height")];
      return { kind: "wire", points: [[0, 0], [width, 0], [width, height], [0, height], [0, 0]].map(([x, y]) => new Vector3(x, y, 0)), closed: true };
    }
    case "polygon": {
      const radius = positive(inputs.radius, "radius");
      if (!(inputs.sides >= 3 && inputs.sides <= 256)) refuse("degenerate-size", "sides");
      const corners = Array.from({ length: inputs.sides }, (_, index) => new Vector3(radius * Math.cos((TAU * index) / inputs.sides), radius * Math.sin((TAU * index) / inputs.sides), 0));
      return { kind: "wire", points: [...corners, corners[0]!], closed: true };
    }
    case "interpolate": {
      const points = vecs(inputs.points);
      if (points.length < 2) refuse("too-few-points", "points");
      if (adjacentCoincide(points)) refuse("degenerate-curve", "points");
      if (inputs.degree < 1 || inputs.degree >= points.length) refuse("degree-points", "degree");
      return inputs.degree === 1 ? { kind: "curve", points, closed: false } : splineBody(points);
    }
    case "approximate": {
      const points = vecs(inputs.points);
      if (points.length < 2) refuse("too-few-points", "points");
      if (adjacentCoincide(points)) refuse("degenerate-curve", "points");
      if (inputs.degree < 1 || inputs.controlPoints <= inputs.degree || inputs.controlPoints > points.length) refuse("degree-points", "controlPoints");
      const chord = points[0]!.distanceTo(points[points.length - 1]!);
      return { kind: "curve", points: [points[0]!, points[points.length - 1]!], closed: false, bounds: [chord, pathLength(points)] };
    }
    case "helix": {
      const axis = unitNormal(inputs.axis, "axis");
      const [radius, pitch, turns] = [positive(inputs.radius, "radius"), positive(inputs.pitch, "pitch"), positive(inputs.turns, "turns")];
      const count = Math.max(Math.ceil(turns * 32), 8);
      const turn = new Quaternion().setFromUnitVectors(Z, axis);
      const points = Array.from({ length: count + 1 }, (_, index) => {
        const t = (index / count) * turns;
        return new Vector3(radius * Math.cos(t * TAU), radius * Math.sin(t * TAU), pitch * t).applyQuaternion(turn).add(vec(inputs.origin));
      });
      return { kind: "wire", points, closed: false };
    }
    default: throw new Error(`no curve oracle for ${kind}`);
  }
};
//#endregion 🔖️Curves

//#region 🔖️Surfaces
const faceOf = (loop: Vector3[]): Body => ({ kind: "face", loop });

const wireLoop = (wire: Body, port: string): Vector3[] => {
  if (wire.kind !== "wire") return refuse("shape-kind", port);
  if (!wire.closed) refuse("wire-open", port);
  return wire.points.slice(0, -1);
};

const sampled = (sample: (u: number, v: number) => Vector3): Vector3[] => Array.from({ length: 17 }, (_, i) => Array.from({ length: 17 }, (_, j) => sample(i / 16, j / 16))).flat();

const surfaceBody = (kind: string, inputs: Inputs): Body => {
  switch (kind) {
    case "plane": {
      const normal = vec(inputs.plane.normal);
      if (normal.length() <= 1e-12) refuse("degenerate-direction", "plane");
      const plane = new Plane().setFromNormalAndCoplanarPoint(normal.clone().normalize(), vec(inputs.plane.origin));
      const turn = new Quaternion().setFromUnitVectors(Z, normal.clone().normalize());
      const sample = (u: number, v: number) => new Vector3(2 * u - 1, 2 * v - 1, 0).applyQuaternion(turn).add(vec(inputs.plane.origin));
      return { kind: "surface", sample, plane, samples: sampled(sample) };
    }
    case "planarFace": {
      let loop = vecs(inputs.points);
      if (loop.length >= 4 && same(loop[0]!, loop[loop.length - 1]!)) loop = loop.slice(0, -1);
      if (loop.length < 3) refuse("too-few-points", "points");
      if (loop.some((point, index) => same(point, loop[(index + 1) % loop.length]!))) refuse("points-degenerate", "points");
      outlineNormal(loop, "points-degenerate", "points-not-planar", "points");
      return faceOf(loop);
    }
    case "planarFaceFromWire":
    case "faceFromWire": {
      const loop = wireLoop(inputs.wire, "wire");
      const normal = outlineNormal(loop, "wire-degenerate", "wire-not-planar", "wire");
      if (kind === "planarFaceFromWire" && Math.abs(normal.z) < 1 - 1e-9) refuse("wire-not-xy-parallel", "wire");
      return faceOf(loop);
    }
    case "nurbsGrid": {
      const points = vecs(inputs.points);
      const rows: number = inputs.rows;
      if (rows < 2 || points.length % rows !== 0 || points.length / rows < 2) refuse("grid-shape", "rows");
      const columns = points.length / rows;
      if (inputs.degreeU < 1 || inputs.degreeU >= columns) refuse("degree-points", "degreeU");
      if (inputs.degreeV < 1 || inputs.degreeV >= rows) refuse("degree-points", "degreeV");
      const at = (row: number, column: number) => points[row * columns + column]!;
      const middle = (count: number) => (count % 2 === 1 ? [(count - 1) / 2] : [count / 2 - 1, count / 2]);
      const centre = new Vector3();
      let taken = 0;
      for (const row of middle(rows)) for (const column of middle(columns)) { centre.add(at(row, column)); taken += 1; }
      centre.divideScalar(taken);
      const corner = (u: number, v: number) => at(v === 1 ? rows - 1 : 0, u === 1 ? columns - 1 : 0);
      const sample = (u: number, v: number) => (u === 0.5 && v === 0.5 ? centre : corner(Math.round(u), Math.round(v)));
      return { kind: "surface", sample, samples: points };
    }
    case "coons": {
      const [bottom, right, top, left] = ["bottom", "right", "top", "left"].map(port => {
        const boundary = vecs(inputs[port]);
        if (boundary.length < 2 || adjacentCoincide(boundary)) refuse("degenerate-curve", port);
        return boundary.length === 2 ? new LineCurve3(boundary[0]!, boundary[1]!) : new CatmullRomCurve3(boundary, false, "catmullrom", 0.5);
      }) as (CatmullRomCurve3 | LineCurve3)[];
      const ends = (curve: CatmullRomCurve3 | LineCurve3) => [curve.getPoint(0), curve.getPoint(1)];
      const [[b0, b1], [r0, r1], [t0, t1], [l0, l1]] = [bottom, right, top, left].map(ends) as [Vector3, Vector3][];
      if (!same(b0, l0) || !same(b1, r0) || !same(t0, l1) || !same(t1, r1)) refuse("kernel");
      const sample = (u: number, v: number) => {
        const ruled = bottom.getPoint(u).multiplyScalar(1 - v).add(top.getPoint(u).multiplyScalar(v)).add(left.getPoint(v).multiplyScalar(1 - u)).add(right.getPoint(v).multiplyScalar(u));
        const corners = b0.clone().multiplyScalar((1 - u) * (1 - v)).add(b1.clone().multiplyScalar(u * (1 - v))).add(l1.clone().multiplyScalar((1 - u) * v)).add(t1.clone().multiplyScalar(u * v));
        return ruled.sub(corners);
      };
      return { kind: "surface", sample, samples: sampled(sample) };
    }
    case "offset": {
      const face = inputs.face as Body;
      if (face.kind !== "face") refuse("kernel");
      const loop = (face as { loop: Vector3[] }).loop;
      const shift = newell(loop).normalize().multiplyScalar(inputs.distance);
      return faceOf(loop.map(point => point.clone().add(shift)));
    }
    default: throw new Error(`no surface oracle for ${kind}`);
  }
};
//#endregion 🔖️Surfaces

//#region 🔖️Solids
/** 🧱️ A closed convex prism or ruled solid through aligned profile outlines, capped at both ends. */
const ruled = (profiles: Vector3[][]): Body => {
  const triangles: Triangle[] = [...fan(profiles[0]!), ...fan(profiles[profiles.length - 1]!)];
  for (let layer = 0; layer + 1 < profiles.length; layer += 1) {
    const [from, to] = [profiles[layer]!, profiles[layer + 1]!];
    from.forEach((a, index) => {
      const next = (index + 1) % from.length;
      triangles.push(new Triangle(a, from[next]!, to[next]!), new Triangle(a, to[next]!, to[index]!));
    });
  }
  return { kind: "solid", triangles: outward(triangles), faces: profiles[0]!.length + 2 };
};

const profileLoop = (profile: Body, port: string): Vector3[] => {
  if (profile.kind === "face") return profile.loop;
  if (profile.kind !== "wire") return refuse("kernel");
  return wireLoop(profile, port);
};

const direction = (value: unknown, port: string) => {
  const vector = vec(value);
  if (vector.length() <= 1e-12) refuse("degenerate-direction", port);
  return vector;
};

const centroidOf = (loop: Vector3[]) => loop.reduce((sum, point) => sum.add(point), new Vector3()).divideScalar(loop.length);
const rho = (point: Vector3) => Math.hypot(point.x, point.y);

/** 🌀️ Pappus: volume = section area x the path of the section's centroid, surface = sum of boundary length x the path of its midpoint, both around the Z axis through `angle`, plus the two end sections. */
const pappus = (loop: Vector3[], angle: number): { volume: number; area: number } => {
  const section = outlineArea(loop);
  const lateral = loop.reduce((sum, a, index) => {
    const b = loop[(index + 1) % loop.length]!;
    return sum + a.distanceTo(b) * rho(a.clone().add(b).multiplyScalar(0.5)) * angle;
  }, 0);
  return { volume: section * rho(centroidOf(loop)) * angle, area: lateral + 2 * section };
};

const latheBody = (loop: Vector3[], angle: number): Body => {
  const profile = loop.map(point => new Vector2(rho(point), point.z));
  const lathe = new LatheGeometry([...profile, profile[0]!], 512, 0, angle);
  const position = lathe.getAttribute("position");
  const mapped = Array.from({ length: position.count }, (_, index) => new Vector3(position.getZ(index), position.getX(index), position.getY(index)));
  const indexed = lathe.getIndex()!;
  const triangles = Array.from({ length: indexed.count / 3 }, (_, at) => new Triangle(mapped[indexed.getX(3 * at)]!, mapped[indexed.getX(3 * at + 1)]!, mapped[indexed.getX(3 * at + 2)]!));
  const lateral = triangles.reduce((sum, triangle) => sum + triangle.getArea(), 0);
  return { kind: "solid", triangles, analytic: { volume: pappus(loop, angle).volume, area: angle >= TAU - 1e-9 ? lateral : lateral + 2 * outlineArea(loop) } };
};

const solidBody = (kind: string, inputs: Inputs): Body => {
  switch (kind) {
    case "extrudeWire": {
      const vector = direction(inputs.vector, "vector");
      const loop = wireLoop(inputs.wire, "wire");
      const normal = outlineNormal(loop, "wire-degenerate", "wire-not-planar", "wire");
      if (Math.abs(normal.dot(vector.clone().normalize())) <= 1e-9) refuse("extrusion-in-plane", "vector");
      return ruled([loop, loop.map(point => point.clone().add(vector))]);
    }
    case "extrudeFace": {
      const vector = direction(inputs.vector, "vector");
      const loop = profileLoop(inputs.face, "face");
      return ruled([loop, loop.map(point => point.clone().add(vector))]);
    }
    case "revolve": {
      const axis = direction(inputs.axisDirection, "axisDirection");
      if (!(inputs.angle > 0 && inputs.angle <= TAU + 1e-9)) refuse("revolve-angle", "angle");
      const face = inputs.face as Body;
      if (face.kind !== "face") return refuse("kernel");
      if (Math.abs(newell(face.loop).normalize().dot(axis.normalize())) > 1 - 1e-9) refuse("revolve-degenerate", "face");
      return latheBody(face.loop, inputs.angle);
    }
    case "loft": {
      const profiles = (inputs.profiles as Body[]).map(profile => profileLoop(profile, "profiles"));
      if (profiles.length < 2) refuse("too-few-profiles", "profiles");
      return ruled(profiles);
    }
    case "sweep":
    case "pipe": {
      const path = inputs.path as Body;
      if (path.kind !== "wire" && path.kind !== "curve") return refuse("shape-kind", "path");
      const loop = profileLoop(inputs.profile, "profile");
      const { points } = path;
      if (points.length === 2) return ruled([loop, loop.map(point => point.clone().add(points[1]!.clone().sub(points[0]!)))]);
      const angle = Math.abs(Math.atan2(points[points.length - 1]!.y, points[points.length - 1]!.x) - Math.atan2(points[0]!.y, points[0]!.x));
      return { kind: "solid", triangles: [], analytic: pappus(loop, angle) };
    }
    case "helicalSweep": {
      direction(inputs.axisDirection, "axisDirection");
      for (const port of ["radius", "pitch", "turns"]) positive(inputs[port], port);
      const loop = profileLoop(inputs.profile, "profile");
      return { kind: "solid", triangles: [], analytic: { volume: outlineArea(loop) * TAU * rho(centroidOf(loop)) * inputs.turns } };
    }
    default: throw new Error(`no solid oracle for ${kind}`);
  }
};
//#endregion 🔖️Solids

//#region 🔖️Booleans
const isSolidRecipe = (recipe: Recipe): boolean => ["box", "sphere", "cylinder", "translate", "rotate", "cut", "compound"].includes(recipe.recipe);

const manifoldOfOperand = (recipe: Recipe, port: string): Mesh3 => {
  if (!isSolidRecipe(recipe)) return refuse("shape-kind", port);
  if (recipe.recipe === "compound") return Manifold.union((recipe.of as Recipe[]).map(member => manifoldOfOperand(member, port)));
  return manifoldOf(recipe);
};

const booleanBody = (kind: string, inputs: Inputs): Body => {
  let result: Mesh3;
  switch (kind) {
    case "fuse": result = manifoldOfOperand(inputs.a, "a").add(manifoldOfOperand(inputs.b, "b")); break;
    case "cut": result = manifoldOfOperand(inputs.a, "a").subtract(manifoldOfOperand(inputs.b, "b")); break;
    case "intersect": result = manifoldOfOperand(inputs.a, "a").intersect(manifoldOfOperand(inputs.b, "b")); break;
    case "compoundCut": {
      const tools = inputs.tools as Recipe[];
      if (tools.length === 0) refuse("too-few-tools", "tools");
      result = manifoldOfOperand(inputs.target, "target").subtract(Manifold.union(tools.map(tool => manifoldOfOperand(tool, "tools"))));
      break;
    }
    default: throw new Error(`no boolean oracle for ${kind}`);
  }
  if (result.volume() < 1e-9) refuse("boolean-empty");
  return { kind: "solid", triangles: trianglesOfManifold(result), manifold: result };
};
//#endregion 🔖️Booleans

//#region 🔖️Recipes
const SHAPE_PORTS = new Set(["wire", "face", "profile", "path", "guide"]);

type WidgetRecipe = Recipe & { kind?: string; inputs?: Inputs };

/** 🧊️ The body of any recipe a fixture can name as a shape input; oracle recipes of solids matter only as operands of booleans and as wrong inputs of the others. */
const shapeBody = (recipe: WidgetRecipe): Body => {
  if (recipe.recipe === "widget") return build(recipe);
  if (recipe.recipe === "line") return curveBody("line", recipe as Inputs);
  return { kind: "solid", triangles: [] };
};

const withBodies = (inputs: Inputs): Inputs => {
  const out: Inputs = { ...inputs };
  for (const [port, value] of Object.entries(inputs)) {
    if (SHAPE_PORTS.has(port) && value && typeof value === "object") out[port] = shapeBody(value);
    if (port === "profiles") out[port] = (value as WidgetRecipe[]).map(shapeBody);
  }
  return out;
};

/** 🧊️ Builds the body a widget recipe names from the bodies of its own shape inputs. */
export const build = (recipe: WidgetRecipe): Body => {
  const [, category, name] = (recipe.kind as string).split(".") as [string, string, string];
  const inputs = recipe.inputs ?? {};
  switch (category) {
    case "curve": return curveBody(name, inputs);
    case "surface": return surfaceBody(name, withBodies(inputs));
    case "solid": return solidBody(name, withBodies(inputs));
    case "boolean": return booleanBody(name, inputs);
    default: throw new Error(`no oracle for ${recipe.kind}`);
  }
};
//#endregion 🔖️Recipes

//#region 🔖️Oracle
/** 🔬️ The oracle of one fixture case: builds the widget from its inputs and measures exactly the keys the case states. */
export const oracleCase = (fixtureCase: FixtureCase): OracleResult => {
  try {
    const body = build({ recipe: "widget", kind: fixtureCase.kind, inputs: fixtureCase.inputs });
    const expected = (fixtureCase.outputs?.shape ?? {}) as Record<string, any>;
    return { outputs: { shape: measure(body, Object.keys(expected), expected) } };
  } catch (error) {
    if (error instanceof Refusal) return { fault: { code: error.code, ...(error.port ? { port: error.port } : {}) } };
    throw error;
  }
};
//#endregion 🔖️Oracle
