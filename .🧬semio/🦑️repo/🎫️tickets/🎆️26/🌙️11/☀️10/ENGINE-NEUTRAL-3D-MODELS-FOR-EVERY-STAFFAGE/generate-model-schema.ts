import { writeFileSync } from "node:fs";
const out = process.argv[2]!;
const ref = (name: string) => ({ $ref: `#/$defs/${name}` });
const num = { type: "number" };
const nonneg = { type: "number", minimum: 0 };
const pos = { type: "number", exclusiveMinimum: 0 };
const int = (minimum: number) => ({ type: "integer", minimum });
const arr = (items: object, extra: object = {}) => ({ type: "array", items, ...extra });
const fixed = (items: object, n: number) => arr(items, { minItems: n, maxItems: n });
const obj = (properties: Record<string, object>, required: string[], description?: string) => ({ ...(description ? { description } : {}), type: "object", additionalProperties: false, required, properties });
const slug = ref("Slug");
const vec = (n: number) => ref(`Vec${n}`);
const transform = obj({ translation: vec(3), rotation: vec(4), scale: vec(3) }, [], "Optional translation, rotation quaternion [x, y, z, w] and scale; absent = identity.");
const common = { id: slug, node: slug, material: slug, tags: arr(slug, { uniqueItems: true }), transform: ref("TransformPatch"), repeat: ref("Repeat"), weights: arr(obj({ node: slug, weight: nonneg }, ["node", "weight"]), { minItems: 1, maxItems: 4 }) };
const commonRequired = ["id", "kind", "node", "material"];
const solid = (kind: string, description: string, properties: Record<string, object>, required: string[]) =>
  obj({ kind: { const: kind }, ...common, ...properties }, [...commonRequired, ...required], description);
const profile = arr(fixed(num, 2), { minItems: 2 });
const defs: Record<string, object> = {
  Slug: { description: "A kebab-case identifier, unique within its scope.", type: "string", minLength: 1, maxLength: 64, pattern: "^[a-z0-9]+(?:-[a-z0-9]+)*$" },
  Text: obj({ en: { type: "string", minLength: 1 }, de: { type: "string", minLength: 1 } }, ["en", "de"], "A human-readable text in every supported language; there is no default language."),
  Vec2: fixed(num, 2),
  Vec3: fixed(num, 3),
  Vec4: fixed(num, 4),
  Mat4: { description: "Column-major 4x4 matrix.", ...fixed(num, 16) },
  TransformPatch: transform,
  ModelDocument: obj({
    $schema: { type: "string" },
    version: { const: "semio.model/v1" },
    id: slug,
    title: ref("Text"),
    provenance: obj({ generator: { type: "string" }, source: { type: "string" }, notes: arr(ref("Text")) }, [], "Free provenance metadata."),
    nodes: arr(ref("Node")),
    solids: arr(ref("Solid")),
    meshes: arr(ref("Mesh")),
    skins: arr(ref("Skin")),
    materials: arr(ref("Material")),
    variants: arr(ref("Variant")),
    poses: arr(ref("Pose")),
    animations: arr(ref("Animation")),
  }, ["version", "id", "title", "nodes", "solids", "meshes", "skins", "materials", "variants", "poses", "animations"], "A neutral 3D model: right-handed, +Y up, +Z ahead, +X the asset's left, metres, radians, quaternions [x, y, z, w]. Procedural when it carries solids and no meshes, resolved after tessellation."),
  Node: obj({ id: slug, parent: { oneOf: [slug, { type: "null" }] }, translation: vec(3), rotation: vec(4), scale: vec(3), joint: ref("Joint"), mesh: slug, skin: slug, tags: arr(slug, { uniqueItems: true }) }, ["id", "parent", "translation", "rotation", "scale"], "A transform node; parents precede children."),
  Joint: obj({ type: { enum: ["fixed", "revolute", "prismatic", "spherical", "free"] }, axis: vec(3), min: num, max: num, twistMin: num, twistMax: num }, ["type", "axis"], "The mechanism: limits in rad or m about a node-local axis."),
  Solid: { description: "Procedural declarative geometry in the node-local frame.", oneOf: ["BoxSolid", "EllipsoidSolid", "CapsuleSolid", "RevolveSolid", "ExtrudeSolid", "LoftSolid"].map(ref) },
  BoxSolid: solid("box", "A centred box with optionally rounded edges.", { size: vec(3), cornerRadius: nonneg }, ["size"]),
  EllipsoidSolid: solid("ellipsoid", "A centred ellipsoid.", { radii: vec(3) }, ["radii"]),
  CapsuleSolid: solid("capsule", "The hull of two spheres (p0, r0) and (p1, r1).", { p0: vec(3), p1: vec(3), r0: nonneg, r1: nonneg, lateralScale: pos }, ["p0", "p1", "r0", "r1"]),
  RevolveSolid: solid("revolve", "A profile [axial, radial] revolved about an axis through origin.", { profile, axis: vec(3), origin: vec(3), sweepRad: { type: "number", exclusiveMinimum: 0, maximum: 6.283185307179586 }, lateralScale: pos }, ["profile", "axis", "origin"]),
  ExtrudeSolid: solid("extrude", "A polygon (with holes) on a plane extruded from one coordinate to another, optionally tapered.", { profile, holes: arr(profile), plane: { enum: ["xy", "yz", "zx"] }, from: num, to: num, taperTo: fixed(nonneg, 2) }, ["profile", "plane", "from", "to"]),
  LoftSection: obj({ at: vec(3), rx: pos, ry: pos, exponent: pos, rotation: vec(4) }, ["at", "rx", "ry"], "One section: centre at, semi-axes rx along x and ry along y, optional superellipse exponent and optional rotation quaternion [x, y, z, w]; without rotation the section is perpendicular to the loft path."),
  LoftSolid: solid("loft", "Superelliptic sections |x/rx|^n + |y/ry|^n = 1 (n = exponent, default 2 = ellipse) lofted in order. A section with rotation lies in the x, y plane of that rotation (absolute); a section without rotation lies perpendicular to the path tangent at its at (central difference of the neighbouring sections, one-sided at the ends), its x axis being the node X axis projected perpendicular to the tangent (fallback Z) and its y axis tangent x x, the same frame lateralScale uses on capsules and revolves. Sections with and without rotation may be mixed.", { sections: arr(ref("LoftSection"), { minItems: 2 }), capStart: { type: "boolean" }, capEnd: { type: "boolean" } }, ["sections", "capStart", "capEnd"]),
  Repeat: obj({ count: int(1), translation: vec(3), rotationAxis: vec(3), rotationRad: num, origin: vec(3) }, ["count"], "The k-th copy applies the step k times."),
  Mesh: obj({ id: slug, primitives: arr(ref("Primitive")) }, ["id", "primitives"]),
  PrimitiveSolid: obj({ id: slug, first: int(0), count: int(1) }, ["id", "first", "count"], "Provenance range: the solid id and the index range [first, first + count) of the primitive that it produced."),
  Primitive: obj({ mode: { enum: ["triangles", "lines"] }, material: slug, positions: arr(num), normals: arr(num), colors: arr(num), joints: arr(int(0)), weights: arr(nonneg), indices: arr(int(0)), solids: arr(ref("PrimitiveSolid")) }, ["mode", "material", "positions", "indices"], "Triangle or line geometry. Optional solids: which solid produced which index range; the ranges are contiguous, in solid order and cover all indices (a multiple of 3 for triangles, of 2 for lines)."),
  Skin: obj({ id: slug, joints: arr(slug, { minItems: 1 }), inverseBind: arr(vec(16 as never)) }, ["id", "joints", "inverseBind"]),
  Material: obj({ id: slug, title: ref("Text"), baseColor: fixed(num, 4), metallic: { type: "number", minimum: 0, maximum: 1 }, roughness: { type: "number", minimum: 0, maximum: 1 }, emissive: fixed(num, 3), alphaMode: { enum: ["opaque", "blend"] }, doubleSided: { type: "boolean" }, unlit: { type: "boolean" } }, ["id", "title", "baseColor", "metallic", "roughness", "emissive", "alphaMode", "doubleSided", "unlit"]),
  Variant: obj({ id: slug, title: ref("Text"), bindings: { type: "object", propertyNames: { pattern: "^[a-z0-9]+(?:-[a-z0-9]+)*$" }, additionalProperties: slug } }, ["id", "title", "bindings"], "A named look of one geometry."),
  Pose: obj({ id: slug, title: ref("Text"), nodes: { type: "object", propertyNames: { pattern: "^[a-z0-9]+(?:-[a-z0-9]+)*$" }, additionalProperties: transform } }, ["id", "title", "nodes"]),
  Animation: obj({ id: slug, title: ref("Text"), durationS: pos, loop: { type: "boolean" }, channels: arr(ref("Channel")), events: arr(obj({ t: nonneg, id: slug }, ["t", "id"])) }, ["id", "title", "durationS", "loop", "channels", "events"]),
  Channel: obj({
    target: { oneOf: [
      obj({ node: slug, path: { enum: ["translation", "rotation", "scale"] } }, ["node", "path"]),
      obj({ material: slug, path: { enum: ["baseColor", "emissive"] } }, ["material", "path"]),
    ] },
    interpolation: { enum: ["step", "linear"] },
    times: arr(nonneg, { minItems: 1 }),
    values: arr(num),
  }, ["target", "interpolation", "times", "values"]),
  Operation: { description: "Progress and cancellation of one expensive operation. A behavioural interface, not data: it never appears in a document and is described here only so the schema names every exported contract of the kernel. report(fraction) receives a number in [0, 1] and returns nothing; cancelled() returns a boolean that the operation polls at a bounded interval, and a true answer makes it stop with a CancelledError and no partial result.", type: "object", additionalProperties: false, required: ["report", "cancelled"], properties: { report: { description: "Function (fraction: number) => void; fraction is monotone in [0, 1]." }, cancelled: { description: "Function () => boolean; true once the caller wants the operation to stop." } } },
  Quality: obj({ chordM: pos, minSegments: int(3), maxSegments: int(3) }, ["chordM", "minSegments", "maxSegments"], "Tessellation quality: maximum chord deviation in metres and the segment bounds of one full circle; deterministic."),
  Issue: obj({ severity: { enum: ["error", "warning"] }, code: { type: "string", minLength: 1 }, path: { type: "string" }, message: ref("Text") }, ["severity", "code", "path", "message"], "One validation or pose finding; path is a JSON pointer into the document."),
};
(defs.Skin as any).properties.inverseBind = arr(ref("Mat4"));
const schema = { $schema: "http://json-schema.org/draft-07/schema#", $id: "semio.model/v1", title: "Model", description: "Domain-neutral, engine-neutral 3D model: nodes, mechanism, procedural solids, meshes, skins, materials, variants, poses and animations.", ...ref("ModelDocument"), $defs: defs };
writeFileSync(out, JSON.stringify(schema, null, 2) + "\n");
