import { writeFileSync } from "node:fs";
import { ANCHOR_KINDS, CATEGORIES, COMPLETION, EASES, EVENT_KINDS, EXPORT_FORMATS, JOINT_TYPES, PARAMETER_SOURCES, SCHEMA_VERSION, SHAPE_ROLES, SUPPORT_TYPES, THERMAL_MODELS, VIEWPOINTS } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

const ref = (name: string) => ({ $ref: `#/$defs/${name}` });
const num = (description?: string, extra: object = {}) => ({ type: "number", ...extra, ...(description ? { description } : {}) });
const str = (extra: object = {}) => ({ type: "string", ...extra });
const arr = (items: object, extra: object = {}) => ({ type: "array", items, ...extra });
const rec = (values: object) => ({ type: "object", propertyNames: ref("Slug"), additionalProperties: values });
const obj = (properties: Record<string, object>, required: string[] = Object.keys(properties), description?: string) => ({
  ...(description ? { description } : {}), type: "object", additionalProperties: false, required, properties,
});
const opt = (properties: Record<string, object>, required: string[], description?: string) => obj(properties, required, description);
const en = (values: readonly string[]) => ({ enum: [...values] });
const nonNeg = { minimum: 0 };
const pos = { exclusiveMinimum: 0 };

const geometry = {
  CapsuleShape: ["capsule", { x0: num(), y0: num(), x1: num(), y1: num(), r0: num(undefined, nonNeg), r1: num(undefined, nonNeg) }, ["x0", "y0", "x1", "y1", "r0", "r1"], "A stadium between two local points with radii r0 and r1 (tangent hull of two circles)."],
  EllipseShape: ["ellipse", { cx: num(), cy: num(), rx: num(undefined, pos), ry: num(undefined, pos), rotationDeg: num() }, ["cx", "cy", "rx", "ry"], "An ellipse in local coordinates."],
  RectShape: ["rect", { x: num(), y: num(), w: num(undefined, pos), h: num(undefined, pos), r: num(undefined, nonNeg) }, ["x", "y", "w", "h"], "A rectangle with optional corner radius, lower-left at (x, y)."],
  PolyShape: ["poly", { points: arr({ type: "array", items: num(), minItems: 2, maxItems: 2 }, { minItems: 2 }), closed: { type: "boolean" } }, ["points", "closed"], "A polygon or polyline through local points."],
  PathShape: ["path", { d: str({ minLength: 1 }) }, ["d"], "An SVG-style path (M L H V C S Q T A Z, absolute or relative) in local metres, y up."],
} as const;
const common = {
  id: ref("Slug"), bone: ref("Slug"), role: ref("ShapeRole"), lod: ref("Lod"), z: num(), filled: { type: "boolean" }, tone: num(undefined, { minimum: 0, maximum: 1 }), region: ref("Slug"), material: ref("Slug"),
};
const commonRequired = ["id", "bone", "role", "lod", "z", "filled", "tone"];
const geometryDef = ([kind, props, required, description]: readonly [string, Record<string, object>, string[], string]) => obj({ kind: { const: kind }, ...props }, ["kind", ...required], description);
const shapeBranch = ([kind, props, required]: readonly [string, Record<string, object>, string[], string]) => obj({ kind: { const: kind }, ...common, ...props }, ["kind", ...commonRequired, ...required]);

const defs: Record<string, object> = {
  Slug: { description: "A kebab-case identifier, unique within its scope.", type: "string", minLength: 1, maxLength: 64, pattern: "^[a-z0-9]+(?:-[a-z0-9]+)*$" },
  Text: obj({ en: str({ minLength: 1 }), de: str({ minLength: 1 }) }, ["en", "de"], "A human-readable text in every supported language; there is no default language."),
  Category: { description: "One of the six archive categories.", ...en(CATEGORIES) },
  ParameterSource: { description: "How a physical parameter is known.", ...en(PARAMETER_SOURCES) },
  Viewpoint: { description: "The projection an authored geometry set is drawn in.", ...en(VIEWPOINTS) },
  ExportFormat: { description: "The output kinds an asset can be exported to.", ...en(EXPORT_FORMATS) },
  Completion: { description: "How far an asset has come.", ...en(COMPLETION) },
  Reference: obj({ id: ref("Slug"), citation: str({ minLength: 1 }), url: str({ minLength: 1 }) }, ["id", "citation"], "A source a parameter or a shape relies on."),
  Provenance: obj({ author: str({ minLength: 1 }), method: { const: "handcrafted" }, created: str({ pattern: "^\\d{4}-\\d{2}-\\d{2}$" }), references: arr(ref("Reference")), notes: ref("Text") }, ["author", "method", "created", "references"], "Who made an asset and how."),
  Material: obj({
    id: ref("Slug"), title: ref("Text"), emissivity: num(undefined, { exclusiveMinimum: 0, maximum: 1 }), solarAbsorptivity: num(undefined, { minimum: 0, maximum: 1 }), densityKgM3: num(undefined, pos), conductivityWmK: num(undefined, pos),
    elasticModulusPa: num(undefined, pos), strengthPa: num(undefined, pos), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "title", "emissivity", "source"], "A material with the properties the thermal and structural models read."),
  JointType: { description: "How a bone may move relative to its parent.", ...en(JOINT_TYPES) },
  Joint: obj({ type: ref("JointType"), minDeg: num(), maxDeg: num(), axisDeg: num(), minM: num(), maxM: num() }, ["type"], "A joint and its limits; revolute limits are angle offsets from rest, prismatic limits are offsets along axisDeg."),
  SegmentMass: obj({ kg: num(undefined, nonNeg), comX: num(), comY: num(), distribution: en(["point", "uniform"]), source: ref("ParameterSource"), reference: ref("Slug") }, ["kg", "comX", "comY", "distribution", "source"], "A segment mass: point mass at the local centre, or spread uniformly along the bone."),
  Section: obj({ areaM2: num(undefined, pos), secondMomentM4: num(undefined, pos), extremeFibreM: num(undefined, pos), material: ref("Slug"), title: ref("Text") }, ["areaM2", "secondMomentM4", "extremeFibreM", "material"], "A member cross-section for stress."),
  Bone: obj({ id: ref("Slug"), parent: { oneOf: [ref("Slug"), { type: "null" }] }, title: ref("Text"), x: num(), y: num(), angleDeg: num(), lengthM: num(undefined, nonNeg), joint: ref("Joint"), mass: ref("SegmentMass"), section: ref("Section") }, ["id", "parent", "title", "x", "y", "angleDeg", "lengthM", "joint"], "A rigid frame placed at (x, y) and rotated by angleDeg in its parent's frame at rest."),
  Rig: obj({ kind: en(["anatomical", "botanical", "mechanical", "structural"]), root: ref("Slug"), bones: arr(ref("Bone"), { minItems: 1 }) }, ["kind", "root", "bones"], "A transform hierarchy with exactly one root."),
  AnchorKind: { description: "What an anchor is for.", ...en(ANCHOR_KINDS) },
  SupportType: { description: "How a support restrains.", ...en(SUPPORT_TYPES) },
  Anchor: obj({ id: ref("Slug"), bone: ref("Slug"), x: num(), y: num(), kind: ref("AnchorKind"), title: ref("Text"), support: ref("SupportType"), normalDeg: num(), footprintLengthM: num(undefined, pos), footprintWidthM: num(undefined, pos), frictionCoefficient: num(undefined, nonNeg) }, ["id", "bone", "x", "y", "kind", "title"], "A point fixed on a bone (local coordinates)."),
  ShapeRole: { description: "The line-hierarchy role of a shape.", ...en(SHAPE_ROLES) },
  Lod: { description: "The level of detail a shape needs.", type: "integer", enum: [0, 1, 2, 3] },
  ...Object.fromEntries(Object.entries(geometry).map(([name, value]) => [name, geometryDef(value)])),
  ShapeGeometry: { description: "One of the shape primitives.", oneOf: Object.keys(geometry).map(ref) },
  Shape: { description: "An authored shape bound to a bone: one geometry plus the common fields.", oneOf: Object.values(geometry).map(shapeBranch) },
  BoneOffset: obj({ angleDeg: num(), dx: num(), dy: num() }, [], "An offset from rest for one bone."),
  AppliedLoad: obj({
    id: ref("Slug"), kind: en(["carried", "imposed", "live", "wind", "snow", "occupant", "equipment"]), title: ref("Text"), anchor: ref("Slug"), bone: ref("Slug"), x: num(), y: num(), fxN: num(), fyN: num(), momentNm: num(),
    distributed: obj({ bone: ref("Slug"), fromM: num(), toM: num(), wxNPerM: num(), wyNPerM: num() }), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "kind", "title", "fxN", "fyN", "source"], "A load applied at an anchor or a bone point, in world newtons."),
  ThermalBinding: obj({
    activityMet: num(undefined, nonNeg), operating: rec(num()),
    regions: rec(obj({ exposedFraction: num(undefined, { minimum: 0, maximum: 1 }), surfaceTempC: num(), coreTempC: num(), wettedness: num(undefined, { minimum: 0, maximum: 1 }) }, [])),
    emitters: rec(num()),
  }, [], "The thermal state a pose binds."),
  Pose: obj({
    id: ref("Slug"), title: ref("Text"), description: ref("Text"), bones: rec(ref("BoneOffset")), root: obj({ x: num(), y: num(), angleDeg: num() }, []), ground: en(["auto", "authored"]), contacts: arr(ref("Slug")), thermal: ref("ThermalBinding"), loads: arr(ref("AppliedLoad")),
  }, ["id", "title", "description", "bones", "ground", "contacts", "thermal", "loads"], "A named configuration."),
  Ease: { description: "How a key eases into the next one. linear, in (t^2), out (1-(1-t)^2) and step blend the two key states by the eased weight; smooth is a C1 cubic Hermite (Catmull-Rom) segment through the neighbouring keys: tangents are the finite differences across the previous and next key (wrapping for looping clips, zero at the ends of non-looping clips), angles in degrees on bone offsets, translations linear in value space, so velocity is continuous across keys. Scalar tracks follow the same rule. Looping clips drop a last key at or beyond the duration that repeats the first.", ...en(EASES) },
  ClipKey: obj({ t: num(undefined, nonNeg), pose: ref("Slug"), ease: ref("Ease"), bones: rec(ref("BoneOffset")) }, ["t", "pose", "ease"], "A clip key."),
  ContactSpan: obj({ anchor: ref("Slug"), fromS: num(undefined, nonNeg), toS: num(undefined, pos) }, undefined, "A time span during which an anchor is in contact."),
  Phase: obj({ id: ref("Slug"), fromS: num(undefined, nonNeg), toS: num(undefined, pos), title: ref("Text") }, undefined, "A named action phase."),
  ClipEvent: obj({ t: num(undefined, nonNeg), kind: en(EVENT_KINDS), title: ref("Text"), target: ref("Slug") }, ["t", "kind", "title"], "A point event on the clip timeline."),
  ScalarTrack: obj({ target: str({ pattern: "^(activityMet|emitter:[a-z0-9-]+|operating:[a-z0-9-]+|load:[a-z0-9-]+|region:[a-z0-9-]+:(exposedFraction|surfaceTempC|coreTempC|wettedness))$" }), ease: ref("Ease"), keys: arr(obj({ t: num(undefined, nonNeg), v: num() }), { minItems: 1 }) }, ["target", "ease", "keys"], "A scalar track of a clip."),
  Clip: obj({
    id: ref("Slug"), title: ref("Text"), description: ref("Text"), durationS: num(undefined, pos), fps: num(undefined, { minimum: 1 }), loop: { type: "boolean" }, rootMotion: en(["none", "plant"]), advanceM: num(),
    keys: arr(ref("ClipKey"), { minItems: 1 }), contacts: arr(ref("ContactSpan")), phases: arr(ref("Phase")), events: arr(ref("ClipEvent")), tracks: arr(ref("ScalarTrack")),
  }, ["id", "title", "description", "durationS", "fps", "loop", "rootMotion", "keys", "contacts", "phases", "events", "tracks"], "A deterministic clip over named poses."),
  Environment: obj({ airTempC: num(), meanRadiantTempC: num(), relativeHumidity: num(undefined, { minimum: 0, maximum: 1 }), windMS: num(undefined, nonNeg), windDirectionDeg: num(), solarWM2: num(undefined, nonNeg), pressurePa: num(undefined, pos) }, undefined, "Ambient conditions."),
  ThermalModelKind: { description: "How a region's surface temperature is obtained.", ...en(THERMAL_MODELS) },
  ThermalRegion: obj({
    id: ref("Slug"), title: ref("Text"), kind: en(["skin", "clothing", "hair", "fur", "feather", "foliage", "bark", "surface", "engine", "exhaust", "glazing", "tyre", "wall"]), material: ref("Slug"), model: ref("ThermalModelKind"), areaM2: num(undefined, pos), characteristicLengthM: num(undefined, pos),
    surfaceTempC: num(), coreTempC: num(), insulationM2KW: num(undefined, nonNeg), wettedness: num(undefined, { minimum: 0, maximum: 1 }), stomatalConductanceMS: num(undefined, nonNeg), operatingKey: ref("Slug"), riseK: num(), metabolicShare: num(undefined, { minimum: 0, maximum: 1 }), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "title", "kind", "material", "model", "areaM2", "characteristicLengthM", "source"], "A thermal region: one surface patch with its material, area and model parameters."),
  Emitter: obj({
    id: ref("Slug"), kind: en(["breath", "exhaust", "transpiration", "vent", "plume"]), title: ref("Text"), anchor: ref("Slug"), directionDeg: num(), carries: en(["airflow", "vapour"]), temperatureC: num(), velocityMS: num(undefined, nonNeg), diameterM: num(undefined, pos),
    vapourKgPerKg: num(undefined, nonNeg), periodS: num(undefined, pos), dutyFrom: num(undefined, { minimum: 0, maximum: 1 }), dutyTo: num(undefined, { minimum: 0, maximum: 1 }), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "kind", "title", "anchor", "directionDeg", "carries", "temperatureC", "velocityMS", "diameterM", "source"], "A rig-attached source of air or vapour."),
  ThermalModel: obj({ environment: ref("Environment"), baseMet: num(undefined, nonNeg), regions: arr(ref("ThermalRegion")), emitters: arr(ref("Emitter")), assumptions: arr(ref("Text")) }, ["environment", "regions", "emitters", "assumptions"], "The thermal model of an asset."),
  StructuralModel: obj({ gravityMS2: num(undefined, pos), dynamics: en(["quasi-static", "inertial"]), supports: arr(ref("Slug")), assumptions: arr(ref("Text")), clinical: { const: false } }, undefined, "The structural model."),
  Dimensions: obj({ heightM: num(undefined, pos), widthM: num(undefined, pos), depthM: num(undefined, pos), massKg: num(undefined, pos) }, undefined, "Nominal overall real dimensions and total mass."),
  Presentation: obj({ defaultPose: ref("Slug"), defaultClip: ref("Slug"), viewpoints: arr(ref("Viewpoint"), { minItems: 1 }) }, undefined, "Presentation defaults."),
  Asset: obj({
    id: ref("Slug"), category: ref("Category"), title: ref("Text"), summary: ref("Text"), tags: arr(ref("Slug")), provenance: ref("Provenance"), dimensions: ref("Dimensions"), materials: arr(ref("Material")), rig: ref("Rig"), anchors: arr(ref("Anchor")),
    geometry: obj({ elevation: arr(ref("Shape"), { minItems: 1 }), plan: arr(ref("Shape")) }, ["elevation"]), poses: arr(ref("Pose"), { minItems: 3 }), clips: arr(ref("Clip"), { minItems: 2 }), thermal: ref("ThermalModel"), structure: ref("StructuralModel"), presentation: ref("Presentation"), exports: arr(ref("ExportFormat")), completion: ref("Completion"),
  }, ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "geometry", "poses", "clips", "thermal", "structure", "presentation", "exports", "completion"], "One handcrafted staffage asset."),
  CategoryDocument: obj({ $schema: str(), version: { const: SCHEMA_VERSION }, category: ref("Category"), assets: arr(ref("Asset")) }, undefined, "One category file."),
  ManifestEntry: obj({
    id: ref("Slug"), category: ref("Category"), title: ref("Text"), heightM: num(undefined, pos), widthM: num(undefined, pos), poses: arr(ref("Slug")), clips: arr(ref("Slug")), analytics: arr(en(["thermal", "structural"])), viewpoints: arr(ref("Viewpoint")), exports: arr(ref("ExportFormat")), distinction: ref("Text"), completion: ref("Completion"),
  }, undefined, "A manifest row."),
  CategoryEntry: obj({ id: ref("Category"), emoji: str({ minLength: 1 }), title: ref("Text"), description: ref("Text"), file: str({ minLength: 1 }) }, undefined, "A category of the archive with its file."),
  Manifest: obj({
    $schema: str(), version: { const: SCHEMA_VERSION }, categories: arr(ref("CategoryEntry")),
    collections: arr(obj({ id: ref("Slug"), title: ref("Text"), description: ref("Text"), assets: arr(ref("Slug")) })), entries: arr(ref("ManifestEntry")),
  }, undefined, "The archive manifest."),
};

const schema = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: SCHEMA_VERSION,
  title: "Staffage",
  description: "Handcrafted architectural staffage with real dimensions, rigs, poses, clips, thermal and structural models. A document is a category file or the manifest.",
  oneOf: [ref("CategoryDocument"), ref("Manifest")],
  $defs: defs,
};
writeFileSync(process.argv[2]!, JSON.stringify(schema, null, 2) + "\n");
