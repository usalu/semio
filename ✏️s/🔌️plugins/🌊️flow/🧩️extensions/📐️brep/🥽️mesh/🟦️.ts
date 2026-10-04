/** 🥽️ Portable indexed-polygon contract and geometric analysis. */
export type MeshPoint = [number, number, number];
export type MeshAttributeValue = null | boolean | number | string | MeshAttributeValue[] | { [key: string]: MeshAttributeValue };
export interface MeshAttribute { domain: "vertex" | "corner" | "face" | "edge"; semantic: "normal" | "uv" | "color" | "material" | "custom"; interpolation: "linear" | "nearest" | "constant"; values: MeshAttributeValue[]; indices?: number[] }
export interface PolygonMesh { vertices: MeshPoint[]; faces: number[][]; attributes?: Record<string, MeshAttribute>; materials?: Record<string, Record<string, MeshAttributeValue>>; textures?: Record<string, { mime: string; bytes: number[] }> }
export interface MeshAnalysis { vertices: number; faces: number; edges: number; triangles: number; boundaryEdges: number; nonManifoldEdges: number; inconsistentEdges: number; degenerateTriangles: number; area: number; volume?: number; minimum: MeshPoint; maximum: MeshPoint }
const sub = (a: MeshPoint, b: MeshPoint): MeshPoint => [a[0] - b[0], a[1] - b[1], a[2] - b[2]];
const cross = (a: MeshPoint, b: MeshPoint): MeshPoint => [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]];
const dot = (a: MeshPoint, b: MeshPoint) => a.reduce((sum, value, axis) => sum + value * b[axis], 0);

/** 🧬️ Validates the language-neutral mesh schema and cross-field index constraints. */
export function parsePolygonMesh(text: string): PolygonMesh {
  if (text.length > 16_000_000) throw new Error("mesh input exceeds 16 MB");
  const mesh = JSON.parse(text) as PolygonMesh;
  if (!mesh || Object.keys(mesh).some(key => !["vertices", "faces", "attributes", "materials", "textures"].includes(key))) throw new Error("unknown mesh field");
  if (!Array.isArray(mesh.vertices) || mesh.vertices.length < 3 || mesh.vertices.length > 100_000 || !Array.isArray(mesh.faces) || !mesh.faces.length || mesh.faces.length > 100_000) throw new Error("invalid mesh size");
  for (const point of mesh.vertices) if (!Array.isArray(point) || point.length !== 3 || point.some(value => typeof value !== "number" || !Number.isFinite(Math.fround(value)))) throw new Error("vertex must have three finite coordinates");
  let corners = 0;
  for (const face of mesh.faces) {
    if (!Array.isArray(face) || face.length < 3 || new Set(face).size !== face.length || face.some(index => !Number.isInteger(index) || index < 0 || index >= mesh.vertices.length)) throw new Error("invalid polygon indices");
    corners += face.length;
    if (corners > 600_000) throw new Error("mesh exceeds 600000 polygon corners");
  }
  validateMeshAttributes(mesh, corners);
  return mesh;
}

/** 🎨️ Checks declared domains, finite interpolation values and owned material/texture references. */
function validateMeshAttributes(mesh: PolygonMesh, corners: number): void {
  const record = (value: unknown): value is Record<string, unknown> => !!value && typeof value === "object" && !Array.isArray(value);
  const entries = <T>(value: Record<string, T> | undefined, limit: number): [string, T][] => {
    if (value === undefined) return [];
    if (!record(value)) throw new Error("mesh channel declarations must be objects");
    const result = Object.entries(value);
    if (result.length > limit || result.some(([name]) => !name.length || name.length > 256 || [...name].length > 128)) throw new Error("mesh channel declaration limit exceeded");
    return result;
  };
  const finite = (value: unknown): value is number => typeof value === "number" && Number.isFinite(Math.fround(value));
  const numeric = (value: unknown): value is number | number[] => finite(value) || (Array.isArray(value) && value.length > 0 && value.length <= 16 && value.every(finite));
  const tuple = (value: unknown, width: number): value is number[] => Array.isArray(value) && value.length === width && value.every(finite);
  let textureBytes = 0;
  for (const [, texture] of entries(mesh.textures, 256)) {
    if (!record(texture) || Object.keys(texture).some(key => key !== "mime" && key !== "bytes") || typeof texture.mime !== "string" || !texture.mime.length || (texture.mime.length > 256 || [...texture.mime].length > 128) || !Array.isArray(texture.bytes) || texture.bytes.length > 16_000_000 || texture.bytes.some(value => !Number.isInteger(value) || value < 0 || value > 255)) throw new Error("invalid owned mesh texture");
    textureBytes += texture.bytes.length; if (textureBytes > 16_000_000) throw new Error("owned mesh textures exceed 16 MB");
  }
  for (const [, material] of entries(mesh.materials, 10_000)) {
    if (!record(material)) throw new Error("invalid owned mesh material");
    if (material.baseColor !== undefined && (!tuple(material.baseColor, 4) || material.baseColor.some(value => value < 0 || value > 1))) throw new Error("invalid material base color");
    for (const field of ["metallic", "roughness", "occlusionStrength"]) if (material[field] !== undefined && (!finite(material[field]) || (material[field] as number) < 0 || (material[field] as number) > 1)) throw new Error("invalid material coefficient");
    if (material.alphaCutoff !== undefined && (!finite(material.alphaCutoff) || material.alphaCutoff < 0)) throw new Error("invalid material alpha cutoff");
    if (material.emissive !== undefined && (!tuple(material.emissive, 3) || material.emissive.some(value => value < 0))) throw new Error("invalid material emissive");
    if (material.alphaMode !== undefined && !["OPAQUE", "MASK", "BLEND"].includes(material.alphaMode as string)) throw new Error("invalid material alpha mode");
    if (material.normalScale !== undefined && !(finite(material.normalScale) || Array.isArray(material.normalScale) && material.normalScale.length === 2 && material.normalScale.every(finite))) throw new Error("invalid material normal scale");
    if (material.textureCoordinates !== undefined && (!record(material.textureCoordinates) || Object.entries(material.textureCoordinates).some(([field,set]) => !["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"].includes(field) || !Number.isInteger(set) || (set as number)<0 || (set as number)>63))) throw new Error("invalid material UV selection");
    if (material.textureSamplers !== undefined && (!record(material.textureSamplers) || Object.entries(material.textureSamplers).some(([field,sampler])=>!["baseColorTexture","metallicRoughnessTexture","normalTexture","occlusionTexture","emissiveTexture"].includes(field) || !record(sampler) || Object.entries(sampler).some(([field,value])=>!({wrapS:[33071,33648,10497],wrapT:[33071,33648,10497],magFilter:[9728,9729],minFilter:[9728,9729,9984,9985,9986,9987]} as Record<string,number[]>)[field]?.includes(value as number))))) throw new Error("invalid material texture sampler");
    if (material.doubleSided !== undefined && typeof material.doubleSided !== "boolean") throw new Error("invalid material sidedness");
    for (const [field, value] of Object.entries(material)) if (field.endsWith("Texture") && (typeof value !== "string" || !mesh.textures || !Object.hasOwn(mesh.textures, value))) throw new Error("undefined mesh material texture");
  }
  for (const [name, attribute] of entries(mesh.attributes, 64)) {
    if (!record(attribute) || Object.keys(attribute).some(key => !["domain", "semantic", "interpolation", "values", "indices"].includes(key)) || !["vertex", "corner", "face", "edge"].includes(attribute.domain) || !["normal", "uv", "color", "material", "custom"].includes(attribute.semantic) || !["linear", "nearest", "constant"].includes(attribute.interpolation) || !Array.isArray(attribute.values)) throw new Error("invalid mesh attribute declaration");
    const length = attribute.domain === "vertex" ? mesh.vertices.length : attribute.domain === "face" ? mesh.faces.length : corners;
    if (attribute.values.length > 600_000 || (attribute.indices === undefined ? attribute.values.length !== length : !Array.isArray(attribute.indices) || attribute.indices.length !== length || attribute.indices.some(index => !Number.isInteger(index) || index < 0 || index >= attribute.values.length))) throw new Error("mesh attribute cardinality does not match its domain");
    if (attribute.interpolation === "linear") {
      const width = Array.isArray(attribute.values[0]) ? attribute.values[0].length : 0;
      if (attribute.values.some(value => !numeric(value) || (Array.isArray(value) ? value.length : 0) !== width)) throw new Error("linear mesh attribute requires compatible finite numeric values");
    }
    const width = attribute.semantic === "normal" ? 3 : attribute.semantic === "uv" ? 2 : attribute.semantic === "color" ? 4 : 0;
    if (width && (!["vertex", "corner", "face"].includes(attribute.domain) || attribute.values.some(value => !tuple(value, width)))) throw new Error("invalid mesh attribute semantic domain or dimensions");
    if (name === "tangent" && (attribute.semantic !== "custom" || !["vertex", "corner", "face"].includes(attribute.domain) || attribute.values.some(value => !tuple(value, 4) || (value as number[]).slice(0, 3).every(coordinate => coordinate === 0) || ![-1, 1].includes((value as number[])[3])))) throw new Error("invalid canonical tangent basis");
    if (attribute.semantic === "normal" && attribute.values.some(value => (value as number[]).every(coordinate => coordinate === 0))) throw new Error("mesh normal cannot be zero");
    if (attribute.semantic === "material" && (attribute.domain !== "face" || attribute.interpolation === "linear" || attribute.values.some(value => typeof value !== "string" || !mesh.materials || !Object.hasOwn(mesh.materials, value)))) throw new Error("undefined mesh face material");
  }
}

/** ✂️ Ear-clips planar simple polygons, including concave faces, with original winding. */
export function triangulateMeshFace(mesh: PolygonMesh, face: number[]): number[][] {
  if (face.length === 3) return [[...face]];
  let normal: MeshPoint = [0, 0, 0];
  for (let i = 0; i < face.length; i++) {
    const a = mesh.vertices[face[i]], b = mesh.vertices[face[(i + 1) % face.length]];
    normal = [normal[0] + (a[1] - b[1]) * (a[2] + b[2]), normal[1] + (a[2] - b[2]) * (a[0] + b[0]), normal[2] + (a[0] - b[0]) * (a[1] + b[1])];
  }
  const axis = normal.map(Math.abs).indexOf(Math.max(...normal.map(Math.abs)));
  if (normal[axis] === 0) throw new Error("degenerate polygon");
  const point = (id: number) => mesh.vertices[id].filter((_, i) => i !== axis);
  const orient = (a: number, b: number, c: number) => { const [x, y] = point(a), [u, v] = point(b), [p, q] = point(c); return (u - x) * (q - y) - (v - y) * (p - x); };
  const winding = Math.sign(normal[axis]) * (axis === 1 ? -1 : 1);
  const pending = [...face], triangles: number[][] = [];
  while (pending.length > 3) {
    let clipped = false;
    for (let i = 0; i < pending.length; i++) {
      const a = pending[(i + pending.length - 1) % pending.length], b = pending[i], c = pending[(i + 1) % pending.length];
      if (orient(a, b, c) * winding <= 0) continue;
      if (pending.some(p => p !== a && p !== b && p !== c && orient(a, b, p) * winding >= 0 && orient(b, c, p) * winding >= 0 && orient(c, a, p) * winding >= 0)) continue;
      triangles.push([a, b, c]); pending.splice(i, 1); clipped = true; break;
    }
    if (!clipped) throw new Error("polygon is self-intersecting or degenerate");
  }
  triangles.push(pending);
  return triangles;
}

/** 🎯️ Expands indexed mesh components into one sorted set of affected vertices. */
export function componentVertexIds(mesh: PolygonMesh, mode: "vertex" | "edge" | "face", selection: number[]): number[] {
  if (!Array.isArray(selection) || !selection.length || selection.length > 600_000) throw new Error("select mesh components");
  if (!["vertex", "edge", "face"].includes(mode)) throw new Error("unknown mesh component mode");
  const halfedges = mode === "edge" ? mesh.faces.flatMap(face => face.map((a, i) => [a, face[(i + 1) % face.length]])) : [];
  const limit = mode === "vertex" ? mesh.vertices.length : mode === "face" ? mesh.faces.length : halfedges.length;
  const vertices = new Set<number>();
  for (const id of selection) {
    if (!Number.isInteger(id) || id < 0 || id >= limit) throw new Error("component index out of range");
    for (const vertex of mode === "vertex" ? [id] : mode === "face" ? mesh.faces[id] : halfedges[id]) vertices.add(vertex);
  }
  return [...vertices].sort((a, b) => a - b);
}

/** 🧭️ Parameters of a topology-preserving transform around the selection centroid or a chosen point. */
export interface MeshComponentTransform {
  mode: "vertex" | "edge" | "face";
  selection: number[];
  operation: "translate" | "rotate" | "scale";
  vector: MeshPoint;
  angle: number;
  pivot: "selection" | MeshPoint;
}

/** 🪄️ Transforms selected components once while keeping all other positions and polygon indices unchanged. */
export function transformMeshComponents(input: PolygonMesh, transform: MeshComponentTransform): PolygonMesh {
  const mesh = parsePolygonMesh(JSON.stringify(input));
  mesh.vertices = mesh.vertices.map(point => point.map(Math.fround) as MeshPoint);
  const ids = componentVertexIds(mesh, transform.mode, transform.selection);
  const validPoint = (point: MeshPoint) => Array.isArray(point) && point.length === 3 && point.every(value => typeof value === "number" && Number.isFinite(Math.fround(value)));
  if (!validPoint(transform.vector) || !Number.isFinite(Math.fround(transform.angle)) || (transform.pivot !== "selection" && !validPoint(transform.pivot))) throw new Error("transform values must be finite");
  const pivot = (transform.pivot === "selection" ? [0, 1, 2].map(axis => ids.reduce((sum, id) => sum + mesh.vertices[id][axis], 0) / ids.length) as MeshPoint : transform.pivot).map(Math.fround) as MeshPoint;
  const vector = transform.vector.map(Math.fround) as MeshPoint;
  const length = Math.hypot(...vector), axis = vector.map(value => value / length) as MeshPoint;
  if (transform.operation === "rotate" && length === 0) throw new Error("rotation axis cannot be zero");
  if (transform.operation === "scale" && vector.some(value => value === 0)) throw new Error("scale factors cannot be zero");
  if (!["translate", "rotate", "scale"].includes(transform.operation)) throw new Error("unknown component transform");
  const angle = Math.fround(transform.angle), sine = Math.sin(angle), cosine = Math.cos(angle);
  for (const id of ids) {
    const point = mesh.vertices[id], relative = sub(point, pivot);
    const normal = cross(axis, relative), projection = dot(axis, relative);
    const output = [0, 1, 2].map(coordinate => Math.fround(transform.operation === "translate" ? point[coordinate] + vector[coordinate]
      : pivot[coordinate] + (transform.operation === "scale" ? relative[coordinate] * vector[coordinate] : relative[coordinate] * cosine + normal[coordinate] * sine + axis[coordinate] * projection * (1 - cosine)))) as MeshPoint;
    if (output.some(value => !Number.isFinite(value))) throw new Error("component transform exceeds mesh coordinate range");
    mesh.vertices[id] = output;
  }
  if (transform.operation !== "translate") {
    const selected = new Set(ids), corners = mesh.faces.flat();
    for (const [name, attribute] of Object.entries(mesh.attributes ?? {})) {
      const tangent = name === "tangent" && attribute.semantic === "custom";
      if (attribute.semantic !== "normal" && !tangent) continue;
      const values: MeshAttributeValue[] = [], indices: number[] = [], samples = new Map<string, number>();
      const count = attribute.domain === "vertex" ? mesh.vertices.length : attribute.domain === "face" ? mesh.faces.length : corners.length;
      for (let domain = 0; domain < count; domain++) {
        const source = attribute.indices?.[domain] ?? domain;
        const affected = attribute.domain === "face" ? selected.has(mesh.faces[domain][0]) : selected.has(attribute.domain === "vertex" ? domain : corners[domain]);
        if (attribute.domain === "face" && mesh.faces[domain].some(vertex => selected.has(vertex) !== affected)) throw new Error("face direction cannot represent a mixed component transform");
        const key = `${source}:${affected}`;
        let sample = samples.get(key);
        if (sample === undefined) {
          const normal = attribute.values[source] as MeshPoint;
          let output = normal;
          if (affected) {
            const perpendicular = cross(axis, normal), projection = dot(axis, normal);
            output = [0, 1, 2].map(coordinate => transform.operation === "scale" ? (tangent ? normal[coordinate] * vector[coordinate] : normal[coordinate] / vector[coordinate]) : normal[coordinate] * cosine + perpendicular[coordinate] * sine + axis[coordinate] * projection * (1 - cosine)) as MeshPoint;
            const magnitude = Math.hypot(...output); output = output.map(value => Math.fround(value / magnitude)) as MeshPoint;
            if (output.some(value => !Number.isFinite(value))) throw new Error("component normal transform exceeds mesh range");
          }
          sample = values.length; values.push(tangent && affected ? [...output, (normal as number[])[3] * (transform.operation === "scale" ? Math.sign(vector[0] * vector[1] * vector[2]) : 1)] : output); samples.set(key, sample);
        }
        indices.push(sample);
      }
      attribute.values = values; attribute.indices = indices;
    }
  }
  return mesh;
}

/** ✂️ Cuts connected quad strips using preview halfedge identifiers, without mutating the input. */
export function loopCutMesh(input: PolygonMesh, edges: number[], cuts: number): PolygonMesh {
  const source = parsePolygonMesh(JSON.stringify(input));
  if (!Array.isArray(edges) || !edges.length || edges.length > 600_000) throw new Error("select at least one edge");
  if (!Number.isInteger(cuts) || cuts < 1 || cuts > 256) throw new Error("cuts must be in 1..=256");
  const key = (a: number, b: number) => `${Math.min(a, b)}:${Math.max(a, b)}`;
  const incidence = new Map<string, [number, number][]>();
  const halfedges: [number, number][] = [];
  source.faces.forEach((face, fi) => face.forEach((a, i) => {
    const b = face[(i + 1) % face.length], id = key(a, b);
    halfedges.push([a, b]);
    const uses = incidence.get(id) ?? []; uses.push([fi, i]); incidence.set(id, uses);
  }));
  const marked = new Set<string>(), pending: string[] = [];
  for (const id of edges) {
    if (!Number.isInteger(id) || id < 0 || id >= halfedges.length) throw new Error("edge index out of range");
    const edge = key(...halfedges[id]);
    if (!incidence.get(edge)!.some(([fi]) => source.faces[fi].length === 4)) throw new Error("selected edges must touch a quad");
    if (!marked.has(edge)) { marked.add(edge); pending.push(edge); }
  }
  for (let cursor = 0; cursor < pending.length; cursor++) {
    const uses = incidence.get(pending[cursor])!;
    if (uses.length > 2) throw new Error("loop cut crosses a nonmanifold edge");
    for (const [fi, i] of uses) {
      const face = source.faces[fi];
      if (face.length !== 4) continue;
      const opposite = key(face[(i + 2) % 4], face[(i + 3) % 4]);
      if (!marked.has(opposite)) { marked.add(opposite); pending.push(opposite); }
    }
  }
  const dimensions = (face: number[]) => [marked.has(key(face[0], face[1])) ? cuts + 1 : 1, marked.has(key(face[1], face[2])) ? cuts + 1 : 1];
  let vertexCount = source.vertices.length + marked.size * cuts, faceCount = 0, cornerCount = 0;
  for (const face of source.faces) {
    if (face.length === 4) {
      const [nx, ny] = dimensions(face); vertexCount += (nx - 1) * (ny - 1); faceCount += nx * ny; cornerCount += nx * ny * 4;
    } else {
      faceCount++; cornerCount += face.length + face.filter((a, i) => marked.has(key(a, face[(i + 1) % face.length]))).length * cuts;
    }
  }
  if (vertexCount > 100_000 || faceCount > 100_000 || cornerCount > 600_000) throw new Error("loop cut exceeds mesh capacity");
  const vertices = source.vertices.map(point => point.map(Math.fround) as MeshPoint), faces: number[][] = [];
  const cutVertices = new Map<string, number[]>();
  const ordered = pending.map(edge => edge.split(":").map(Number) as [number, number]).sort(([a, b], [c, d]) => a - c || b - d);
  for (const [a, b] of ordered) {
    const ids: number[] = [];
    let previous = vertices[a];
    for (let step = 1; step <= cuts; step++) {
      const t = step / (cuts + 1), point = vertices[a].map((value, axis) => Math.fround(value * (1 - t) + vertices[b][axis] * t)) as MeshPoint;
      if (point.some(value => !Number.isFinite(value)) || point.every((value, axis) => value === previous[axis]) || point.every((value, axis) => value === vertices[b][axis])) throw new Error("cut vertices collapse at this precision");
      ids.push(vertices.length); vertices.push(point); previous = point;
    }
    cutVertices.set(key(a, b), ids);
  }
  const onEdge = (a: number, b: number, step: number, segments: number) => step === 0 ? a : step === segments ? b : cutVertices.get(key(a, b))![a < b ? step - 1 : segments - step - 1];
  for (const face of source.faces) {
    if (face.length === 4) {
      const [nx, ny] = dimensions(face), grid: number[] = [];
      for (let y = 0; y <= ny; y++) for (let x = 0; x <= nx; x++) {
        if (y === 0) grid.push(onEdge(face[0], face[1], x, nx));
        else if (y === ny) grid.push(onEdge(face[3], face[2], x, nx));
        else if (x === 0) grid.push(onEdge(face[0], face[3], y, ny));
        else if (x === nx) grid.push(onEdge(face[1], face[2], y, ny));
        else {
          const u = x / nx, v = y / ny, weights = [(1 - u) * (1 - v), u * (1 - v), u * v, (1 - u) * v];
          const point = [0, 1, 2].map(axis => Math.fround(face.reduce((sum, id, i) => sum + vertices[id][axis] * weights[i], 0))) as MeshPoint;
          if (point.some(value => !Number.isFinite(value))) throw new Error("non-finite cut vertex");
          grid.push(vertices.length); vertices.push(point);
        }
      }
      for (let y = 0; y < ny; y++) for (let x = 0; x < nx; x++) {
        const i = y * (nx + 1) + x; faces.push([grid[i], grid[i + 1], grid[i + nx + 2], grid[i + nx + 1]]);
      }
    } else {
      const boundary: number[] = [];
      face.forEach((a, i) => {
        const b = face[(i + 1) % face.length]; boundary.push(a);
        if (marked.has(key(a, b))) for (let step = 1; step <= cuts; step++) boundary.push(onEdge(a, b, step, cuts + 1));
      });
      faces.push(boundary);
    }
  }
  return { vertices, faces };
}

/** 🔪️ A line projected onto one zero-based face defines a surface-preserving cut. */
export interface MeshKnifeCut { face: number; start: MeshPoint; end: MeshPoint }

/** ✂️ Splits convex polygons or their concave triangulation and shares boundary intersections with neighbors. */
export function knifeCutMesh(input: PolygonMesh, cut: MeshKnifeCut): PolygonMesh {
  const source = parsePolygonMesh(JSON.stringify(input));
  source.vertices = source.vertices.map(point => point.map(Math.fround) as MeshPoint);
  const validPoint = (point: MeshPoint) => Array.isArray(point) && point.length === 3 && point.every(value => typeof value === "number" && Number.isFinite(Math.fround(value)));
  if (!Number.isInteger(cut.face) || cut.face < 0 || cut.face >= source.faces.length || !validPoint(cut.start) || !validPoint(cut.end)) throw new Error("invalid knife cut");
  const polygon = source.faces[cut.face], origin = source.vertices[polygon[0]];
  const points = polygon.map(id => source.vertices[id]);
  let normal: MeshPoint = [0, 0, 0];
  for (let i = 0; i < points.length; i++) {
    const contribution = cross(sub(points[i], origin), sub(points[(i + 1) % points.length], origin));
    normal = normal.map((value, axis) => value + contribution[axis]) as MeshPoint;
  }
  const magnitude = Math.hypot(...normal);
  if (!magnitude) throw new Error("degenerate knife face");
  normal = normal.map(value => Math.fround(value / magnitude)) as MeshPoint;
  const start = cut.start.map(Math.fround) as MeshPoint, direction = sub(cut.end.map(Math.fround) as MeshPoint, start);
  const plane = cross(direction, normal), length = Math.hypot(...plane);
  if (length <= Math.hypot(...direction) * 1e-7) throw new Error("knife direction must project onto the face");
  const unit = plane.map(value => value / length) as MeshPoint;
  const scale = points.reduce((maximum, point) => Math.max(maximum, Math.hypot(...sub(point, origin))), 0), tolerance = scale * 1e-7;
  const distances = source.vertices.map(point => { const d = dot(sub(point, start), unit); return Math.abs(d) <= tolerance ? 0 : d; });
  if (!polygon.some(id => distances[id] > 0) || !polygon.some(id => distances[id] < 0)) throw new Error("knife line must cross the face interior");
  const planar = points.every(point => Math.abs(dot(sub(point, origin), normal)) <= tolerance);
  const convex = points.every((a, i) => dot(cross(sub(points[(i + 1) % points.length], a), sub(points[(i + 2) % points.length], points[(i + 1) % points.length])), normal) >= -scale * tolerance);
  const pieces = planar && convex ? [polygon] : triangulateMeshFace(source, polygon);
  const key = (a: number, b: number) => `${Math.min(a, b)}:${Math.max(a, b)}`;
  const vertices = [...source.vertices], intersections = new Map<string, number>(), split: number[][] = [];
  for (const piece of pieces) {
    if (!piece.some(id => distances[id] > 0) || !piece.some(id => distances[id] < 0)) { split.push([...piece]); continue; }
    for (const sign of [1, -1]) {
      const clipped: number[] = [];
      piece.forEach((a, i) => {
        const b = piece[(i + 1) % piece.length], da = distances[a], db = distances[b];
        if (da * sign >= 0) clipped.push(a);
        if ((da > 0 && db < 0) || (da < 0 && db > 0)) {
          const edge = key(a, b);
          let id = intersections.get(edge);
          if (id === undefined) {
            const lo = Math.min(a, b), hi = Math.max(a, b), t = distances[lo] / (distances[lo] - distances[hi]);
            const point = vertices[lo].map((value, axis) => Math.fround((1 - t) * value + t * vertices[hi][axis])) as MeshPoint;
            if (point.some(value => !Number.isFinite(value)) || point.every((value, axis) => value === vertices[lo][axis]) || point.every((value, axis) => value === vertices[hi][axis])) throw new Error("knife intersection collapses at this precision");
            if (vertices.length >= 100_000) throw new Error("knife cut exceeds mesh capacity");
            id = vertices.length; vertices.push(point); intersections.set(edge, id);
          }
          clipped.push(id);
        }
      });
      const unique = clipped.filter((id, i) => i === 0 || id !== clipped[i - 1]);
      if (unique[0] === unique.at(-1)) unique.pop();
      if (unique.length < 3) throw new Error("degenerate knife result");
      split.push(unique);
    }
  }
  const faces = source.faces.flatMap((face, index) => index === cut.face ? split : [face.flatMap((a, i) => {
    const id = intersections.get(key(a, face[(i + 1) % face.length]));
    return id === undefined ? [a] : [a, id];
  })]);
  if (faces.length > 100_000 || faces.reduce((sum, face) => sum + face.length, 0) > 600_000) throw new Error("knife cut exceeds mesh capacity");
  return { vertices, faces };
}

/** 📏️ Measures topology and surface area; enclosed volume requires closed, consistently oriented edges. */
export function analyzePolygonMesh(input: PolygonMesh): MeshAnalysis {
  const mesh = parsePolygonMesh(JSON.stringify(input));
  const edges = new Map<string, { count: number; orientation: number }>();
  let area = 0, volume = 0, triangles = 0, degenerateTriangles = 0;
  const minimum: MeshPoint = [Infinity, Infinity, Infinity], maximum: MeshPoint = [-Infinity, -Infinity, -Infinity];
  for (const point of mesh.vertices) for (let axis = 0; axis < 3; axis++) { minimum[axis] = Math.min(minimum[axis], point[axis]); maximum[axis] = Math.max(maximum[axis], point[axis]); }
  for (const face of mesh.faces) {
    for (let i = 0; i < face.length; i++) {
      const a = face[i], b = face[(i + 1) % face.length], key = `${Math.min(a, b)}:${Math.max(a, b)}`;
      const edge = edges.get(key) ?? { count: 0, orientation: 0 }; edge.count++; edge.orientation += a < b ? 1 : -1; edges.set(key, edge);
    }
    for (const triangle of triangulateMeshFace(mesh, face)) {
      const [a, b, c] = triangle.map(id => mesh.vertices[id]);
      const triangleArea = Math.hypot(...cross(sub(b, a), sub(c, a))) / 2;
      area += triangleArea; triangles++; if (triangleArea === 0) degenerateTriangles++;
      volume += dot(sub(a, mesh.vertices[0]), cross(sub(b, mesh.vertices[0]), sub(c, mesh.vertices[0]))) / 6;
    }
  }
  const boundaryEdges = [...edges.values()].filter(edge => edge.count === 1).length;
  const nonManifoldEdges = [...edges.values()].filter(edge => edge.count > 2).length;
  const inconsistentEdges = [...edges.values()].filter(edge => edge.count === 2 && edge.orientation !== 0).length;
  return { vertices: mesh.vertices.length, faces: mesh.faces.length, edges: edges.size, triangles, boundaryEdges, nonManifoldEdges, inconsistentEdges, degenerateTriangles, area, ...(!boundaryEdges && !nonManifoldEdges && !inconsistentEdges && !degenerateTriangles ? { volume: Math.abs(volume) } : {}), minimum, maximum };
}

export interface MeshInspectionQuery { kind: "vertex" | "edge" | "face"; index: number }
export interface MeshInspection { point?: MeshPoint; start?: MeshPoint; end?: MeshPoint; length?: number; vertices?: number[]; normal?: MeshPoint; center?: MeshPoint }

/** 🔎️ Inspects zero-based polygon components; edge indices use the preview halfedge convention. */
export function inspectMeshComponent(mesh: PolygonMesh, query: MeshInspectionQuery): MeshInspection {
  mesh = parsePolygonMesh(JSON.stringify(mesh));
  if (!query || Object.keys(query).some(key => key !== "kind" && key !== "index") || !Number.isInteger(query.index) || query.index < 0) throw new Error("invalid mesh inspection query");
  const point = (id: number): MeshPoint => {
    if (!mesh.vertices[id]) throw new Error("vertex index out of range");
    return mesh.vertices[id].map(Math.fround) as MeshPoint;
  };
  if (query.kind === "vertex") return { point: point(query.index) };
  if (query.kind === "edge") {
    let offset = 0;
    for (const face of mesh.faces) {
      if (query.index < offset + face.length) {
        const index = query.index - offset, start = point(face[index]), end = point(face[(index + 1) % face.length]);
        return { start, end, length: Math.hypot(...sub(end, start)) };
      }
      offset += face.length;
    }
    throw new Error("edge index out of range");
  }
  if (query.kind !== "face" || !mesh.faces[query.index]) throw new Error("face index out of range");
  const vertices = mesh.faces[query.index], points = vertices.map(point), sum: MeshPoint = [0, 0, 0], center: MeshPoint = [0, 0, 0];
  for (let index = 0; index < points.length; index++) {
    const a = points[index], b = points[(index + 1) % points.length];
    sum[0] += (a[1] - b[1]) * (a[2] + b[2]); sum[1] += (a[2] - b[2]) * (a[0] + b[0]); sum[2] += (a[0] - b[0]) * (a[1] + b[1]);
    for (let axis = 0; axis < 3; axis++) center[axis] += a[axis] / points.length;
  }
  const length = Math.hypot(...sum);
  if (!length) throw new Error("face normal is degenerate");
  return { vertices: [...vertices], normal: sum.map(value => value / length) as MeshPoint, center };
}
