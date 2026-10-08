/** 🧊️ Third-party shape oracle of the generation3d analysis tests: rebuilds the fixture shape recipes as triangle meshes with manifold-3d (solids, booleans, compounds) and three (faces, edges, points) and measures them with plain mesh integrals, independent of the B-Rep kernel. */
import Module from "manifold-3d";
import { Box3, Ray, Triangle, Vector3 } from "three";

export type Recipe = { recipe: string; [key: string]: unknown };
type Manifold = ReturnType<Awaited<ReturnType<typeof Module>>["Manifold"]["cube"]>;

/** 🧱️ A closed triangle mesh from a solid or compound, a planar patch of one, a segment, a circle or a point. */
export type Body =
  | { kind: "solid"; manifold: Manifold; triangles: Triangle[] }
  | { kind: "face"; triangles: Triangle[]; normal: Vector3 }
  | { kind: "edge"; from: Vector3; to: Vector3 }
  | { kind: "circle"; radius: number }
  | { kind: "wire"; perimeter: number }
  | { kind: "vertex"; point: Vector3 };

const wasm = await Module();
wasm.setup();
const { Manifold, Mesh } = wasm;
export { Manifold, Mesh };

const SEGMENTS = 256;
const triple = (value: unknown): Vector3 => new Vector3(...(value as [number, number, number]));

const trianglesOf = (manifold: Manifold): Triangle[] => {
  const mesh = manifold.getMesh();
  const vertex = (index: number) => new Vector3(mesh.vertProperties[index * mesh.numProp]!, mesh.vertProperties[index * mesh.numProp + 1]!, mesh.vertProperties[index * mesh.numProp + 2]!);
  const triangles: Triangle[] = [];
  for (let at = 0; at < mesh.triVerts.length; at += 3) triangles.push(new Triangle(vertex(mesh.triVerts[at]!), vertex(mesh.triVerts[at + 1]!), vertex(mesh.triVerts[at + 2]!)));
  return triangles;
};

export const trianglesOfManifold = trianglesOf;

const solid = (manifold: Manifold): Body => ({ kind: "solid", manifold, triangles: trianglesOf(manifold) });

/** 🧊️ The manifold a solid-valued recipe names. */
export const manifoldOf = (recipe: Recipe): Manifold => {
  switch (recipe.recipe) {
    case "box": return Manifold.cube(triple(recipe.size).toArray() as [number, number, number]);
    case "sphere": return Manifold.sphere(recipe.radius as number, SEGMENTS);
    case "cylinder": return Manifold.cylinder(recipe.height as number, recipe.radius as number, recipe.radius as number, SEGMENTS);
    case "translate": return manifoldOf(recipe.of as Recipe).translate(triple(recipe.by).toArray() as [number, number, number]);
    case "rotate": {
      const axis = triple(recipe.axis);
      if (axis.x !== 0 || axis.y !== 0) throw new Error("the oracle rotates about Z only");
      return manifoldOf(recipe.of as Recipe).rotate([0, 0, ((recipe.angle as number) * 180) / Math.PI]);
    }
    case "cut": return manifoldOf(recipe.target as Recipe).subtract(manifoldOf(recipe.tool as Recipe));
    case "compound": return Manifold.compose((recipe.of as Recipe[]).map(manifoldOf));
    default: throw new Error(`${recipe.recipe} is not a solid recipe`);
  }
};

const faceOf = (recipe: Recipe): Body => {
  const whole = solid(manifoldOf(recipe.of as Recipe));
  if (whole.kind !== "solid") throw new Error("a face is picked from a solid");
  const wanted = triple(recipe.normal).normalize();
  const triangles = whole.triangles.filter(triangle => triangle.getNormal(new Vector3()).distanceTo(wanted) < 1e-6);
  if (triangles.length === 0) throw new Error("no face has that normal");
  return { kind: "face", triangles, normal: wanted };
};

/** 🧊️ The body a fixture recipe names. */
export const bodyOf = (recipe: Recipe): Body => {
  switch (recipe.recipe) {
    case "face": return faceOf(recipe);
    case "edge": return { kind: "edge", from: triple(recipe.from), to: triple(recipe.to) };
    case "line": return { kind: "edge", from: triple(recipe.start), to: triple(recipe.end) };
    case "circle": return { kind: "circle", radius: recipe.radius as number };
    case "curvedEdge": return { kind: "circle", radius: (recipe.of as Recipe).radius as number };
    case "rectangleWire": return { kind: "wire", perimeter: 2 * ((recipe.width as number) + (recipe.height as number)) };
    case "vertex": return { kind: "vertex", point: triple(recipe.point) };
    default: return solid(manifoldOf(recipe));
  }
};

//#region 🔖️Integrals
export const area = (triangles: readonly Triangle[]): number => triangles.reduce((sum, triangle) => sum + triangle.getArea(), 0);

/** ⚖️ Signed volume, centroid and the second moment about the origin of a closed mesh by signed tetrahedra against the origin. */
export const volumetric = (triangles: readonly Triangle[]) => {
  let volume = 0;
  const moment = new Vector3();
  const second = [[0, 0, 0], [0, 0, 0], [0, 0, 0]];
  for (const { a, b, c } of triangles) {
    const tetra = a.dot(new Vector3().crossVectors(b, c)) / 6;
    volume += tetra;
    const sum = new Vector3().add(a).add(b).add(c);
    moment.addScaledVector(sum, tetra / 4);
    const [pa, pb, pc, ps] = [a.toArray(), b.toArray(), c.toArray(), sum.toArray()];
    for (let i = 0; i < 3; i++) for (let j = 0; j < 3; j++) second[i]![j]! += (tetra / 20) * (pa[i]! * pa[j]! + pb[i]! * pb[j]! + pc[i]! * pc[j]! + ps[i]! * ps[j]!);
  }
  const centroid = moment.clone().divideScalar(volume);
  const about = second.map((row, i) => row.map((entry, j) => entry - volume * centroid.getComponent(i) * centroid.getComponent(j)));
  const trace = about[0]![0]! + about[1]![1]! + about[2]![2]!;
  const inertia = about.map((row, i) => row.map((entry, j) => (i === j ? trace : 0) - entry));
  return { volume, centroid, inertia };
};

/** 🧭️ Eigenvalues (ascending) of a symmetric 3x3 matrix in closed form (trigonometric method). */
export const eigenvalues = (m: number[][]): number[] => {
  const p1 = m[0]![1]! ** 2 + m[0]![2]! ** 2 + m[1]![2]! ** 2;
  const q = (m[0]![0]! + m[1]![1]! + m[2]![2]!) / 3;
  if (p1 === 0) return [m[0]![0]!, m[1]![1]!, m[2]![2]!].sort((x, y) => x - y);
  const p2 = (m[0]![0]! - q) ** 2 + (m[1]![1]! - q) ** 2 + (m[2]![2]! - q) ** 2 + 2 * p1;
  const p = Math.sqrt(p2 / 6);
  const b = m.map((row, i) => row.map((entry, j) => (entry - (i === j ? q : 0)) / p));
  const det = b[0]![0]! * (b[1]![1]! * b[2]![2]! - b[1]![2]! * b[2]![1]!) - b[0]![1]! * (b[1]![0]! * b[2]![2]! - b[1]![2]! * b[2]![0]!) + b[0]![2]! * (b[1]![0]! * b[2]![1]! - b[1]![1]! * b[2]![0]!);
  const phi = Math.acos(Math.max(-1, Math.min(1, det / 2))) / 3;
  const high = q + 2 * p * Math.cos(phi);
  const low = q + 2 * p * Math.cos(phi + (2 * Math.PI) / 3);
  return [low, 3 * q - high - low, high];
};

/** ✅️ Whether `axis` is a unit eigenvector of `m` for `value`. */
export const isEigenvector = (m: number[][], axis: number[], value: number, tolerance: number): boolean => {
  const length = Math.hypot(...axis);
  const image = m.map(row => row.reduce((sum, entry, j) => sum + entry * axis[j]!, 0));
  return Math.abs(length - 1) < tolerance && image.every((component, i) => Math.abs(component - value * axis[i]!) <= tolerance * Math.max(1, Math.abs(value)));
};
//#endregion 🔖️Integrals

//#region 🔖️Queries
const boxOf = (points: Iterable<Vector3>): Box3 => new Box3().setFromPoints([...points]);
export const pointsOf = (triangles: readonly Triangle[]): Vector3[] => triangles.flatMap(triangle => [triangle.a, triangle.b, triangle.c]);

/** 🛤️ The smallest distance from a point to the surface of a triangle soup. */
export const closestOnSurface = (triangles: readonly Triangle[], point: Vector3): { point: Vector3; distance: number } => {
  let best = { point: new Vector3(), distance: Infinity };
  for (const triangle of triangles) {
    const candidate = triangle.closestPointToPoint(point, new Vector3());
    const distance = candidate.distanceTo(point);
    if (distance < best.distance) best = { point: candidate, distance };
  }
  return best;
};

/** 🧭️ Inside, outside or boundary by parity of the crossings of a skew ray, with a surface band for the boundary. */
export const classify = (triangles: readonly Triangle[], point: Vector3): "inside" | "outside" | "boundary" => {
  if (closestOnSurface(triangles, point).distance <= 1e-6) return "boundary";
  const ray = new Ray(point, new Vector3(0.5773502691896258, 0.3141592653589793, 0.7548776662466927).normalize());
  const crossings = triangles.filter(triangle => ray.intersectTriangle(triangle.a, triangle.b, triangle.c, false, new Vector3()) !== null).length;
  return crossings % 2 === 1 ? "inside" : "outside";
};

/** 🛤️ The distance between two bodies: zero when the solids overlap, else the smallest vertex-to-surface distance either way (exact for the axis-aligned and spherical pairs of the fixture). */
export const distance = (a: Body, b: Body): number => {
  const surface = (body: Body): Triangle[] | Vector3 => (body.kind === "vertex" ? body.point : "triangles" in body ? body.triangles : new Vector3());
  const [sa, sb] = [surface(a), surface(b)];
  if (a.kind === "solid" && b.kind === "solid" && a.manifold.intersect(b.manifold).volume() > 1e-12) return 0;
  if (sa instanceof Vector3 && !(sb instanceof Vector3)) return closestOnSurface(sb, sa).distance;
  if (sb instanceof Vector3 && !(sa instanceof Vector3)) return closestOnSurface(sa, sb).distance;
  if (Array.isArray(sa) && Array.isArray(sb)) return Math.min(...pointsOf(sa).map(p => closestOnSurface(sb, p).distance), ...pointsOf(sb).map(p => closestOnSurface(sa, p).distance));
  throw new Error("distance between these bodies is not modelled");
};

export const boundsOf = (body: Body): Box3 => {
  if (body.kind === "vertex") return boxOf([body.point]);
  if ("triangles" in body) return boxOf(pointsOf(body.triangles));
  throw new Error("bounds of this body are not modelled");
};
//#endregion 🔖️Queries
