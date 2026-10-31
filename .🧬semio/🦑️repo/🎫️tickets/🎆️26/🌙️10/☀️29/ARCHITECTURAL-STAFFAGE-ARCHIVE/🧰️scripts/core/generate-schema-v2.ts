import { writeFileSync } from "node:fs";
import { ANCHOR_KINDS, CATEGORIES, COMPLETION, EASES, EVENT_KINDS, EXPORT_FORMATS, JOINT_TYPES, PARAMETER_SOURCES, SCALE_DENOMINATORS, SCHEMA_VERSION, SHAPE_ROLES, SUPPORT_TYPES, THERMAL_MODELS, VIEWPOINTS } from "/home/deck/semio/🎓️teaching/🏛️architecture/🧍️staffage/🧬️schema/🟦️.ts";

const ref = (name: string) => ({ $ref: `#/$defs/${name}` });
const num = (description?: string, extra: object = {}) => ({ type: "number", ...extra, ...(description ? { description } : {}) });
const str = (extra: object = {}) => ({ type: "string", ...extra });
const arr = (items: object, extra: object = {}) => ({ type: "array", items, ...extra });
const rec = (values: object) => ({ type: "object", propertyNames: ref("Slug"), additionalProperties: values });
const obj = (properties: Record<string, object>, required: string[] = Object.keys(properties), description?: string) => ({
  ...(description ? { description } : {}), type: "object", additionalProperties: false, required, properties,
});
const en = (values: readonly (string | number)[]) => ({ enum: [...values] });
const nonNeg = { minimum: 0 };
const pos = { exclusiveMinimum: 0 };
const unit = { minimum: 0, maximum: 1 };
const nullable = (inner: object) => ({ oneOf: [inner, { type: "null" }] });

const geometry = {
  CapsuleShape: ["capsule", { x0: num(), y0: num(), x1: num(), y1: num(), r0: num(undefined, nonNeg), r1: num(undefined, nonNeg) }, ["x0", "y0", "x1", "y1", "r0", "r1"], "A stadium between two local points with radii r0 and r1 (tangent hull of two circles)."],
  EllipseShape: ["ellipse", { cx: num(), cy: num(), rx: num(undefined, pos), ry: num(undefined, pos), rotationDeg: num() }, ["cx", "cy", "rx", "ry"], "An ellipse in local coordinates."],
  RectShape: ["rect", { x: num(), y: num(), w: num(undefined, pos), h: num(undefined, pos), r: num(undefined, nonNeg) }, ["x", "y", "w", "h"], "A rectangle with optional corner radius, lower-left at (x, y)."],
  PolyShape: ["poly", { points: arr({ type: "array", items: num(), minItems: 2, maxItems: 2 }, { minItems: 2 }), closed: { type: "boolean" } }, ["points", "closed"], "A polygon or polyline through local points."],
  PathShape: ["path", { d: str({ minLength: 1 }) }, ["d"], "An SVG-style path (M L H V C Q A Z, absolute or relative) in local metres, y up."],
} as const;
const common = {
  id: ref("Slug"), bone: ref("Slug"), role: ref("ShapeRole"), z: num(), filled: { type: "boolean" }, tone: num(undefined, unit), region: ref("Slug"), regions: arr(ref("Slug"), { minItems: 1 }), material: ref("Slug"), hatch: ref("Slug"), penMm: num(undefined, pos),
};
const commonRequired = ["id", "bone", "role", "z", "filled", "tone"];
const geometryDef = ([kind, props, required, description]: readonly [string, Record<string, object>, string[], string]) => obj({ kind: { const: kind }, ...props }, ["kind", ...required], description);
const shapeBranch = ([kind, props, required]: readonly [string, Record<string, object>, string[], string]) => obj({ kind: { const: kind }, ...common, ...props }, ["kind", ...commonRequired, ...required]);
const shapeSets = obj({ elevation: arr(ref("Shape"), { minItems: 1 }), plan: arr(ref("Shape")) }, ["elevation"]);
const denominator = ref("ScaleDenominator");
const scalarTrackTarget = "^(activityMet|externalWorkW|breathing|emitter:[a-z0-9-]+|operating:[a-z0-9-]+|load:[a-z0-9-]+|region:[a-z0-9-]+:(exposedFraction|sunlitFraction|wettedness))$";
const speciesList = ["human", "dog", "cat", "horse", "cow", "deer", "bird", "sheep", "goat", "pig", "fox", "squirrel", "hedgehog", "duck", "gull"];
const layerKinds = ["tissue", "clothing", "fur", "feather", "hair", "air-gap", "shell", "insulation", "glass", "masonry", "timber", "metal", "polymer"];
const regionKinds = ["skin", "clothing", "hair", "fur", "feather", "foliage", "bark", "stem", "surface", "engine", "exhaust", "glazing", "tyre", "wall", "heater", "lamp", "water"];
const emitterKinds = ["breath", "exhaust", "vent", "flue", "fan", "transpiration", "plume"];

const defs: Record<string, object> = {
  Slug: { description: "A kebab-case identifier, unique within its scope.", type: "string", minLength: 1, maxLength: 64, pattern: "^[a-z0-9]+(?:-[a-z0-9]+)*$" },
  Text: obj({ en: str({ minLength: 1 }), de: str({ minLength: 1 }) }, ["en", "de"], "A human-readable text in every supported language; there is no default language."),
  CategoryId: { description: "One of the eight archive categories.", ...en(CATEGORIES) },
  ParameterSource: { description: "How a physical parameter is known.", ...en(PARAMETER_SOURCES) },
  Viewpoint: { description: "The projection an authored geometry set is drawn in.", ...en(VIEWPOINTS) },
  ExportFormat: { description: "The output kinds an asset can be exported to.", ...en(EXPORT_FORMATS) },
  Completion: { description: "How far an asset has come.", ...en(COMPLETION) },
  Reference: obj({ id: ref("Slug"), citation: str({ minLength: 1 }), url: str({ minLength: 1 }) }, ["id", "citation"], "A source a parameter or a shape relies on."),
  Provenance: obj({ author: str({ minLength: 1 }), method: { const: "handcrafted" }, created: str({ pattern: "^\\d{4}-\\d{2}-\\d{2}$" }), references: arr(ref("Reference")), notes: ref("Text") }, ["author", "method", "created", "references"], "Who made an asset and how."),
  Material: obj({
    id: ref("Slug"), title: ref("Text"), emissivity: num(undefined, { exclusiveMinimum: 0, maximum: 1 }), solarAbsorptivity: num(undefined, unit), densityKgM3: num(undefined, pos), conductivityWmK: num(undefined, pos), specificHeatJKgK: num(undefined, pos),
    elasticModulusPa: num(undefined, pos), strengthPa: num(undefined, pos), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "title", "emissivity", "source"], "A material with the properties the thermal and structural models read."),
  JointType: { description: "How a bone may move relative to its parent.", ...en(JOINT_TYPES) },
  Joint: obj({ type: ref("JointType"), minDeg: num(), maxDeg: num(), axisDeg: num(), minM: num(), maxM: num() }, ["type"], "A joint and its limits; revolute limits are angle offsets from rest, prismatic limits are offsets along axisDeg."),
  SegmentMass: obj({ kg: num(undefined, nonNeg), comX: num(), comY: num(), distribution: en(["point", "uniform"]), source: ref("ParameterSource"), reference: ref("Slug") }, ["kg", "comX", "comY", "distribution", "source"], "A segment mass: point mass at the local centre, or spread uniformly along the bone."),
  Section: obj({ areaM2: num(undefined, pos), secondMomentM4: num(undefined, pos), extremeFibreM: num(undefined, pos), material: ref("Slug"), title: ref("Text") }, ["areaM2", "secondMomentM4", "extremeFibreM", "material"], "A member cross-section for stress."),
  Bone: obj({ id: ref("Slug"), parent: nullable(ref("Slug")), title: ref("Text"), x: num(), y: num(), angleDeg: num(), lengthM: num(undefined, nonNeg), joint: ref("Joint"), mass: ref("SegmentMass"), section: ref("Section") }, ["id", "parent", "title", "x", "y", "angleDeg", "lengthM", "joint"], "A rigid frame placed at (x, y) and rotated by angleDeg in its parent's frame at rest."),
  Rig: obj({ kind: en(["anatomical", "botanical", "mechanical", "structural"]), root: ref("Slug"), bones: arr(ref("Bone"), { minItems: 1 }) }, ["kind", "root", "bones"], "A transform hierarchy with exactly one root."),
  AnchorKind: { description: "What an anchor is for.", ...en(ANCHOR_KINDS) },
  SupportType: { description: "How a support restrains.", ...en(SUPPORT_TYPES) },
  Anchor: obj({ id: ref("Slug"), bone: ref("Slug"), x: num(), y: num(), kind: ref("AnchorKind"), title: ref("Text"), support: ref("SupportType"), normalDeg: num(), footprintLengthM: num(undefined, pos), footprintWidthM: num(undefined, pos), frictionCoefficient: num(undefined, nonNeg) }, ["id", "bone", "x", "y", "kind", "title"], "A point fixed on a bone (local coordinates)."),
  ShapeRole: { description: "The line-hierarchy role of a shape.", ...en(SHAPE_ROLES) },
  ...Object.fromEntries(Object.entries(geometry).map(([name, value]) => [name, geometryDef(value)])),
  ShapeGeometry: { description: "One of the shape primitives.", oneOf: Object.keys(geometry).map(ref) },
  Shape: { description: "An authored shape of one representation, bound to a bone: one geometry plus the common fields. region binds one thermal region, regions a merged form; hatch names a hatch of the representation; penMm overrides the pen of the role.", oneOf: Object.values(geometry).map(shapeBranch) },
  ScaleDenominator: { description: "A preset drawing-scale denominator.", ...en(SCALE_DENOMINATORS) },
  PenSet: obj({ body: num(undefined, pos), detail: nullable(num(undefined, pos)), texture: nullable(num(undefined, pos)), glazing: nullable(num(undefined, pos)), ground: num(undefined, pos) }, undefined, "The pens of a representation per role in paper millimetres; null means the role is not drawn."),
  Hatch: obj({ id: ref("Slug"), title: ref("Text"), pattern: en(["lines", "cross", "dots", "stipple"]), angleDeg: num(), spacingMm: num(undefined, pos), penMm: num(undefined, pos) }, undefined, "A hatch drawn inside shapes that name it: spacing and pen in paper millimetres."),
  RepresentationDecisions: obj(Object.fromEntries(["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"].map((key) => [key, ref("Text")])), undefined, "The authored design decisions of one representation."),
  AuthoredRepresentation: obj({
    status: { const: "authored" }, denominator, validRange: obj({ minDenominator: num(undefined, pos), maxDenominator: num(undefined, pos) }), pens: ref("PenSet"), hatches: arr(ref("Hatch")), geometry: shapeSets, decisions: ref("RepresentationDecisions"),
  }, undefined, "A representation individually authored for one preset scale."),
  Alternative: { description: "What to offer instead of an unavailable representation.", oneOf: [obj({ kind: { const: "scale" }, denominator }), obj({ kind: { const: "symbol" }, symbol: ref("Slug") }), obj({ kind: { const: "asset" }, asset: ref("Slug") })] },
  UnavailableRepresentation: obj({ status: { const: "unavailable" }, denominator, reason: ref("Text"), limitingFeature: ref("Text"), limitingSizeM: num(undefined, pos), alternatives: arr(ref("Alternative"), { minItems: 1 }) }, undefined, "An explicit decision that the asset cannot be meaningfully drawn at this scale."),
  Representation: { description: "The decision for one preset scale.", oneOf: [ref("AuthoredRepresentation"), ref("UnavailableRepresentation")] },
  SymbolRepresentation: obj({
    id: ref("Slug"), title: ref("Text"), description: ref("Text"), semantics: en(["true-extent", "true-position", "nominal"]), units: en(["model-m", "paper-mm"]), denominators: arr(denominator, { minItems: 1 }), pens: ref("PenSet"), hatches: arr(ref("Hatch")), geometry: shapeSets, dimensionalNote: ref("Text"),
  }, undefined, "A conventional symbol, separately identified from the pictorial representations."),
  PrintProfile: obj({
    id: ref("Slug"), title: ref("Text"), description: ref("Text"), minStrokeMm: num(undefined, pos), minGapMm: num(undefined, pos), minFeatureMm: num(undefined, pos), minHatchSpacingMm: num(undefined, pos), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "title", "description", "minStrokeMm", "minGapMm", "minFeatureMm", "minHatchSpacingMm", "source"], "A print profile: smallest stroke, gap, feature width and hatch spacing that reliably print."),
  PrintProfileDocument: obj({ $schema: str(), version: { const: SCHEMA_VERSION }, defaultProfile: ref("Slug"), profiles: arr(ref("PrintProfile"), { minItems: 1 }), references: arr(ref("Reference")) }, undefined, "The print profile document."),
  BoneOffset: obj({ angleDeg: num(), dx: num(), dy: num() }, [], "An offset from rest for one bone."),
  AppliedLoad: obj({
    id: ref("Slug"), kind: en(["carried", "imposed", "live", "wind", "snow", "occupant", "equipment"]), title: ref("Text"), anchor: ref("Slug"), bone: ref("Slug"), x: num(), y: num(), fxN: num(), fyN: num(), momentNm: num(),
    distributed: obj({ bone: ref("Slug"), fromM: num(), toM: num(), wxNPerM: num(), wyNPerM: num() }), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "kind", "title", "fxN", "fyN", "source"], "A load applied at an anchor or a bone point, in world newtons."),
  RegionBinding: obj({ exposedFraction: num(undefined, unit), sunlitFraction: num(undefined, unit), wettedness: num(undefined, unit), contact: obj({ fraction: num(undefined, unit), tempC: num() }) }, ["exposedFraction", "sunlitFraction"], "The region state a pose binds."),
  ThermalBinding: obj({
    passengers: rec(obj({ activityMet: num(undefined, nonNeg), breathing: ref("Slug") }, ["activityMet"])), activityMet: num(undefined, nonNeg), externalWorkW: num(), operating: rec(num()), breathing: ref("Slug"), regions: rec(ref("RegionBinding")), emitters: rec(num()),
  }, ["regions", "emitters"], "The thermal and respiratory state a pose or clip key binds explicitly."),
  Pose: obj({
    id: ref("Slug"), title: ref("Text"), description: ref("Text"), key: { type: "boolean" }, bones: rec(ref("BoneOffset")), root: obj({ x: num(), y: num(), angleDeg: num() }, []), ground: en(["auto", "authored"]), contacts: arr(ref("Slug")), thermal: ref("ThermalBinding"), loads: arr(ref("AppliedLoad")),
  }, ["id", "title", "description", "bones", "ground", "contacts", "thermal", "loads"], "A named configuration or an in-between key pose of a clip."),
  Ease: { description: "How a key eases into the next one. linear, in (t^2), out (1-(1-t)^2) and step blend the two key states by the eased weight; smooth is a C1 cubic Hermite (Catmull-Rom) segment through the neighbouring keys: tangents are the finite differences across the previous and next key (wrapping for looping clips, zero at the ends of non-looping clips), angles in degrees on bone offsets, translations linear in value space, so velocity is continuous across keys. Scalar tracks follow the same rule. Looping clips drop a last key at or beyond the duration that repeats the first.", ...en(EASES) },
  ClipKey: obj({ t: num(undefined, nonNeg), pose: ref("Slug"), ease: ref("Ease"), bones: rec(ref("BoneOffset")) }, ["t", "pose", "ease"], "A clip key."),
  ContactSpan: obj({ anchor: ref("Slug"), fromS: num(undefined, nonNeg), toS: num(undefined, pos) }, undefined, "A time span during which an anchor is in contact."),
  Phase: obj({ id: ref("Slug"), fromS: num(undefined, nonNeg), toS: num(undefined, pos), title: ref("Text"), ballistic: { type: "boolean" } }, ["id", "fromS", "toS", "title"], "A named action phase."),
  ClipEvent: obj({ t: num(undefined, nonNeg), kind: en(EVENT_KINDS), title: ref("Text"), target: ref("Slug") }, ["t", "kind", "title"], "A point event on the clip timeline."),
  ScalarTrack: obj({ target: str({ pattern: scalarTrackTarget }), ease: ref("Ease"), keys: arr(obj({ t: num(undefined, nonNeg), v: num() }), { minItems: 1 }) }, ["target", "ease", "keys"], "A scalar track of a clip."),
  Clip: obj({
    id: ref("Slug"), title: ref("Text"), description: ref("Text"), durationS: num(undefined, pos), fps: num(undefined, { minimum: 1 }), loop: { type: "boolean" }, rootMotion: en(["none", "plant"]), advanceM: num(),
    keys: arr(ref("ClipKey"), { minItems: 1 }), contacts: arr(ref("ContactSpan")), phases: arr(ref("Phase")), events: arr(ref("ClipEvent")), tracks: arr(ref("ScalarTrack")),
  }, ["id", "title", "description", "durationS", "fps", "loop", "rootMotion", "keys", "contacts", "phases", "events", "tracks"], "A deterministic clip over named poses."),
  Environment: obj({
    airTempC: num(), meanRadiantTempC: num(), skyTempC: num(), groundTempC: num(), relativeHumidity: num(undefined, unit), windMS: num(undefined, nonNeg), windDirectionDeg: num(), solarWM2: num(undefined, nonNeg), diffuseFraction: num(undefined, unit), pressurePa: num(undefined, pos),
  }, ["airTempC", "meanRadiantTempC", "relativeHumidity", "windMS", "windDirectionDeg", "solarWM2", "diffuseFraction", "pressurePa"], "Ambient conditions."),
  EnvironmentPreset: obj({ id: ref("Slug"), title: ref("Text"), description: ref("Text"), environment: ref("Environment"), source: ref("ParameterSource"), reference: ref("Slug") }, ["id", "title", "description", "environment", "source"], "A named environment preset."),
  EnvironmentDocument: obj({ $schema: str(), version: { const: SCHEMA_VERSION }, presets: arr(ref("EnvironmentPreset"), { minItems: 1 }), references: arr(ref("Reference")) }, undefined, "The environment preset document."),
  ThermalModelKind: { description: "How a region's surface temperature is obtained.", ...en(THERMAL_MODELS) },
  ThermalLayer: obj({
    id: ref("Slug"), title: ref("Text"), kind: en(layerKinds), material: ref("Slug"), thicknessM: num(undefined, pos), resistanceM2KW: num(undefined, nonNeg), vapourResistanceM2PaW: num(undefined, nonNeg), heatCapacityJM2K: num(undefined, nonNeg), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "title", "kind", "material", "resistanceM2KW", "source"], "One layer of a region, innermost first."),
  ThermalRegion: obj({
    id: ref("Slug"), title: ref("Text"), kind: en(regionKinds), model: ref("ThermalModelKind"), material: ref("Slug"), areaM2: num(undefined, pos), characteristicLengthM: num(undefined, pos), geometry: en(["cylinder", "sphere", "vertical-plate", "horizontal-plate-up", "horizontal-plate-down"]),
    skyViewFactor: num(undefined, unit), layers: arr(ref("ThermalLayer")), heatCapacityJM2K: num(undefined, nonNeg),
    body: obj({ subject: ref("Slug"), segment: en(["head", "trunk", "arm", "hand", "leg", "foot", "tail", "wing"]), metabolicShare: num(undefined, unit), skinFraction: num(undefined, unit) }, ["segment", "metabolicShare", "skinFraction"]),
    leaf: obj({ stomatalConductanceMaxMS: num(undefined, nonNeg), leafAreaIndex: num(undefined, nonNeg), photosynthesis: en(["c3", "c4", "cam"]) }),
    hidden: ref("Text"), operating: obj({ key: ref("Slug"), heatW: num(undefined, nonNeg) }), conditioned: obj({ interiorTempC: num(), interiorFilmM2KW: num(undefined, nonNeg) }), fixed: obj({ surfaceTempC: num() }), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "title", "kind", "model", "material", "areaM2", "characteristicLengthM", "geometry", "skyViewFactor", "layers", "heatCapacityJM2K", "source"], "A thermal region: one surface patch with its layers and model parameters."),
  BodyPhysiology: obj({
    species: en(speciesList), massKg: num(undefined, pos), skinAreaM2: num(undefined, pos), basalMetabolismW: num(undefined, pos), metWM2: num(undefined, pos), coreSetPointC: num(), skinSetPointC: num(), sweating: { type: "boolean" }, panting: { type: "boolean" },
    maxSkinBloodFlowLM2H: num(undefined, nonNeg), minSkinBloodFlowLM2H: num(undefined, nonNeg), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["species", "massKg", "skinAreaM2", "basalMetabolismW", "metWM2", "coreSetPointC", "skinSetPointC", "sweating", "panting", "maxSkinBloodFlowLM2H", "minSkinBloodFlowLM2H", "source"], "The whole-body thermal physiology of a human or animal."),
  BreathingPattern: obj({
    id: ref("Slug"), title: ref("Text"), rateBpm: num(undefined, pos), tidalVolumeL: num(undefined, pos), inhaleFraction: num(undefined, unit), pauseAfterInhaleFraction: num(undefined, unit), exhaleFraction: num(undefined, unit), pauseAfterExhaleFraction: num(undefined, unit),
    route: en(["nose", "mouth", "nose-and-mouth"]), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "title", "rateBpm", "tidalVolumeL", "inhaleFraction", "pauseAfterInhaleFraction", "exhaleFraction", "pauseAfterExhaleFraction", "route", "source"], "A breathing pattern: frequency, tidal volume and the split of one cycle into four phases."),
  Respiration: obj({
    noseEmitter: ref("Slug"), mouthEmitter: ref("Slug"), exhaledTempC: num(), exhaledRelativeHumidity: num(undefined, unit), patterns: arr(ref("BreathingPattern"), { minItems: 1 }),
    motion: arr(obj({ bone: ref("Slug"), angleDeg: num(), dy: num() })), source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["exhaledTempC", "exhaledRelativeHumidity", "patterns", "motion", "source"], "Respiration of a body."),
  Emitter: obj({
    id: ref("Slug"), kind: en(emitterKinds), title: ref("Text"), anchor: ref("Slug"), anchors: arr(ref("Slug")), directionDeg: num(), diameterM: num(undefined, pos), operatingKey: ref("Slug"), volumeFlowM3S: num(undefined, nonNeg), temperatureRiseK: num(), vapourKgPerKg: num(undefined, nonNeg), region: ref("Slug"),
    source: ref("ParameterSource"), reference: ref("Slug"),
  }, ["id", "kind", "title", "anchor", "directionDeg", "diameterM", "source"], "A rig-attached source of air or vapour."),
  Passenger: obj({ id: ref("Slug"), title: ref("Text"), physiology: ref("BodyPhysiology"), respiration: ref("Respiration") }, ["id", "title", "physiology"], "A second living body carried by the asset."),
  OperatingKey: obj({ id: ref("Slug"), title: ref("Text"), levels: arr(obj({ v: num(), title: ref("Text") }), { minItems: 1 }) }, undefined, "An operating state family of a machine."),
  ThermalModel: obj({
    environment: ref("Environment"), baseMet: num(undefined, nonNeg), body: ref("BodyPhysiology"), respiration: ref("Respiration"), passengers: arr(ref("Passenger")), operating: arr(ref("OperatingKey")), regions: arr(ref("ThermalRegion")), emitters: arr(ref("Emitter")), assumptions: arr(ref("Text")),
  }, ["environment", "passengers", "operating", "regions", "emitters", "assumptions"], "The thermal model of an asset."),
  StructuralModel: obj({ gravityMS2: num(undefined, pos), dynamics: en(["quasi-static", "inertial"]), supports: arr(ref("Slug")), assumptions: arr(ref("Text")), clinical: { const: false } }, undefined, "The structural model."),
  Dimensions: obj({ heightM: num(undefined, pos), widthM: num(undefined, pos), depthM: num(undefined, pos), massKg: num(undefined, pos) }, undefined, "Nominal overall real dimensions and total mass."),
  Presentation: obj({ defaultPose: ref("Slug"), defaultClip: ref("Slug"), defaultScale: denominator, viewpoints: arr(ref("Viewpoint"), { minItems: 1 }) }, undefined, "Presentation defaults."),
  Asset: obj({
    id: ref("Slug"), category: ref("CategoryId"), title: ref("Text"), summary: ref("Text"), tags: arr(ref("Slug")), provenance: ref("Provenance"), dimensions: ref("Dimensions"), materials: arr(ref("Material")), rig: ref("Rig"), anchors: arr(ref("Anchor")),
    representations: obj(Object.fromEntries(SCALE_DENOMINATORS.map((d) => [String(d), ref("Representation")]))), symbols: arr(ref("SymbolRepresentation")), poses: arr(ref("Pose"), { minItems: 3 }), clips: arr(ref("Clip"), { minItems: 2 }),
    thermal: ref("ThermalModel"), structure: ref("StructuralModel"), presentation: ref("Presentation"), exports: arr(ref("ExportFormat")), completion: ref("Completion"),
  }, ["id", "category", "title", "summary", "tags", "provenance", "dimensions", "materials", "rig", "anchors", "representations", "symbols", "poses", "clips", "thermal", "structure", "presentation", "exports", "completion"], "One handcrafted staffage asset; representations decides every preset scale."),
  AssetDocument: obj({ $schema: str(), version: { const: SCHEMA_VERSION }, asset: ref("Asset") }, undefined, "One asset file."),
  ManifestEntry: obj({
    id: ref("Slug"), category: ref("CategoryId"), title: ref("Text"), heightM: num(undefined, pos), widthM: num(undefined, pos), poses: arr(ref("Slug")), clips: arr(ref("Slug")),
    representations: obj(Object.fromEntries(SCALE_DENOMINATORS.map((d) => [String(d), en(["authored", "unavailable"])]))), symbols: arr(ref("Slug")), analytics: arr(en(["thermal", "transport", "structural"])), viewpoints: arr(ref("Viewpoint")), exports: arr(ref("ExportFormat")), distinction: ref("Text"), completion: ref("Completion"),
  }, undefined, "A manifest row."),
  CategoryEntry: obj({ id: ref("CategoryId"), emoji: str({ minLength: 1 }), title: ref("Text"), description: ref("Text"), folder: str({ minLength: 1 }) }, undefined, "A category of the archive with its folder of asset folders."),
  Manifest: obj({
    $schema: str(), version: { const: SCHEMA_VERSION }, categories: arr(ref("CategoryEntry")),
    collections: arr(obj({ id: ref("Slug"), title: ref("Text"), description: ref("Text"), assets: arr(ref("Slug")) })), entries: arr(ref("ManifestEntry")),
  }, undefined, "The archive manifest."),
};

const schema = {
  $schema: "http://json-schema.org/draft-07/schema#",
  $id: SCHEMA_VERSION,
  title: "Staffage",
  description: "Handcrafted architectural staffage with real dimensions, rigs, poses, clips, individually authored drawing-scale representations, layered thermal models, respiration and structural models. A document is an asset file, the manifest, the print profile document or the environment document.",
  oneOf: [ref("AssetDocument"), ref("Manifest"), ref("PrintProfileDocument"), ref("EnvironmentDocument")],
  $defs: defs,
};
writeFileSync(process.argv[2]!, JSON.stringify(schema, null, 2) + "\n");
