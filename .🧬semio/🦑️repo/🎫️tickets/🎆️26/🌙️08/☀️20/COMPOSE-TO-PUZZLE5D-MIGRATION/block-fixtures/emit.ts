// 🧪️ Fixture writer for the 🧱️block plugin. Each case's `before`/`mutation`/`diff` is hand-authored
// from that mutation's own 🔺️diff/🦀️component.rs; `after` is derived by replaying the committed diff
// through a faithful port of the artifact's own `MutationDiff::apply`.
import { mkdirSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { catalogChildId } from "./siphash.ts";

export const REPO = "/Users/ueli/Documents/semio";

type Json = any;

const clone = (v: Json): Json => JSON.parse(JSON.stringify(v));

/** 📂 Rust `apply_identified_delta`: removed → added(push) → patched(in place) → reordered. */
function applyDelta(items: Json[], delta: Json, key: string): Json[] {
  let next = clone(items);
  for (const id of delta.removed ?? []) {
    const at = next.findIndex((e: Json) => e[key] === id);
    if (at < 0) throw new Error(`removed ${id} missing`);
    next.splice(at, 1);
  }
  for (const item of delta.added ?? []) {
    if (next.some((e: Json) => e[key] === item[key])) throw new Error(`added ${item[key]} exists`);
    next.push(clone(item));
  }
  for (const entry of delta.patched ?? []) {
    const at = next.findIndex((e: Json) => e[key] === entry.id);
    if (at < 0) throw new Error(`patched ${entry.id} missing`);
    if (entry.patch?.replacement == null) throw new Error(`patch ${entry.id} has no replacement`);
    next[at] = clone(entry.patch.replacement);
  }
  if (delta.reordered != null) {
    next = delta.reordered.map((id: string) => {
      const found = next.find((e: Json) => e[key] === id);
      if (!found) throw new Error(`reordered ${id} missing`);
      return found;
    });
  }
  return next;
}

const KIT_DIALECT = { artifactKind: "s.stdio.semio", standard: "v1", subset: "kit" };

/** 🪪️ Rust `catalog_child_handle` + `set_vortex_kinds`. */
function setVortexKinds(snapshot: Json, kinds: Json[]): void {
  const id = catalogChildId(kinds.map((k) => ({ id: k.id, name: k.name, category: "vortex-kind" })));
  snapshot.catalog = { childId: id, target: { artifactId: id, dialect: { ...KIT_DIALECT } } };
  snapshot.vortexKindExtra = kinds.map((k) => ({ id: k.id, label: k.label, color: k.color, defaultCableKind: k.defaultCableKind }));
}

function vortexKindsOf(snapshot: Json, seeded: Json[]): Json[] {
  const id = catalogChildId(seeded.map((k) => ({ id: k.id, name: k.name, category: "vortex-kind" })));
  return snapshot.catalog.childId === id ? clone(seeded) : [];
}

//#region 🔖️DiffShapes
const DELTA_DEFAULT = { added: [], removed: [], patched: [], reordered: null };

/** 🔺️ Expands a hand-authored sparse diff into the exact serde shape (`default`, no skips). */
function fullDiff(fields: string[], deltaFields: Set<string>, listFields: Set<string>, sparse: Json): Json {
  const out: Json = {};
  for (const field of fields) {
    if (!(field in sparse)) {
      out[field] = null;
      continue;
    }
    const value = sparse[field];
    if (value !== null && deltaFields.has(field)) out[field] = { ...DELTA_DEFAULT, ...value };
    else if (value !== null && listFields.has(field)) out[field] = { values: value.values };
    else out[field] = value;
  }
  const unknown = Object.keys(sparse).filter((k) => !fields.includes(k));
  if (unknown.length > 0) throw new Error(`unknown diff fields: ${unknown.join(", ")}`);
  return out;
}
//#endregion 🔖️DiffShapes

//#region 🔖️Apply
export const BLOCK5D_DIFF_FIELDS = ["artifact","schema","partKind","part2d","part3d","representations","gripKinds","grips","compatibility","attributes","authors","camera2d","camera3d","meta","selectedIds","locale"];
export const BLOCK3D_DIFF_FIELDS = ["artifact","schema","objectKind","representations","vortexKinds","vortices","compatibility","attributes","authors","camera3d","meta","selectedIds","activeRepresentationId","wantedTags","locale","windows","brushVortexKindId","brushRadius","brushFlip","brushPreview","camera","hoveredVortexFullId"];
export const BLOCK2D_DIFF_FIELDS = ["artifact","schema","nodeKind","presentation","handleKinds","handles","compatibility","attributes","authors","camera2d","meta","selectedIds","locale"];

export function applyBlock5d(before: Json, diff: Json): Json {
  const next = clone(before);
  if (diff.schema !== null) next.schema = diff.schema;
  if (diff.partKind !== null) next.partKind = clone(diff.partKind);
  if (diff.part2d !== null) next["2d"] = clone(diff.part2d);
  if (diff.part3d !== null) next["3d"] = clone(diff.part3d);
  if (diff.representations !== null) next.representations = applyDelta(next.representations, diff.representations, "id");
  if (diff.gripKinds !== null) next.gripKinds = applyDelta(next.gripKinds, diff.gripKinds, "id");
  if (diff.grips !== null) next.grips = applyDelta(next.grips, diff.grips, "id");
  if (diff.compatibility !== null) next.compatibility = applyDelta(next.compatibility, diff.compatibility, "id");
  if (diff.attributes !== null) next.attributes = applyDelta(next.attributes, diff.attributes, "key");
  if (diff.authors !== null) next.authors = clone(diff.authors.values);
  if (diff.camera2d !== null) next.camera2d = clone(diff.camera2d);
  if (diff.camera3d !== null) next.camera3d = clone(diff.camera3d);
  if (diff.meta !== null) next.meta = clone(diff.meta);
  return next;
}

export function applyBlock3d(before: Json, diff: Json, seeded: Json[]): Json {
  const next = clone(before);
  if (diff.schema !== null) next.schema = diff.schema;
  if (diff.objectKind !== null) next.objectKind = clone(diff.objectKind);
  if (diff.representations !== null) next.representations = applyDelta(next.representations, diff.representations, "id");
  if (diff.vortexKinds !== null) setVortexKinds(next, applyDelta(vortexKindsOf(next, seeded), diff.vortexKinds, "id"));
  if (diff.vortices !== null) next.vortices = applyDelta(next.vortices, diff.vortices, "id");
  if (diff.compatibility !== null) next.compatibility = applyDelta(next.compatibility, diff.compatibility, "id");
  if (diff.attributes !== null) next.attributes = applyDelta(next.attributes, diff.attributes, "key");
  if (diff.authors !== null) next.authors = clone(diff.authors.values);
  if (diff.camera3d !== null) next.camera3d = clone(diff.camera3d);
  if (diff.meta !== null) next.meta = clone(diff.meta);
  return next;
}

export function applyBlock2d(before: Json, diff: Json): Json {
  const next = clone(before);
  if (diff.schema !== null) next.schema = diff.schema;
  if (diff.nodeKind !== null) next.nodeKind = clone(diff.nodeKind);
  if (diff.presentation !== null) next.presentation = clone(diff.presentation);
  if (diff.handleKinds !== null) next.handleKinds = applyDelta(next.handleKinds, diff.handleKinds, "id");
  if (diff.handles !== null) next.handles = applyDelta(next.handles, diff.handles, "id");
  if (diff.compatibility !== null) next.compatibility = applyDelta(next.compatibility, diff.compatibility, "id");
  if (diff.attributes !== null) next.attributes = applyDelta(next.attributes, diff.attributes, "key");
  if (diff.authors !== null) next.authors = clone(diff.authors.values);
  if (diff.camera2d !== null) next.camera2d = clone(diff.camera2d);
  if (diff.meta !== null) next.meta = clone(diff.meta);
  return next;
}
//#endregion 🔖️Apply

//#region 🔖️Json
/** 🔣️ Emits canonical JSON with every f64 carrying a decimal point (serde_json number identity). */
export function encodeJson(value: Json, floatPaths: (path: string) => boolean, path = "", indent = 0): string {
  const pad = "  ".repeat(indent);
  const padIn = "  ".repeat(indent + 1);
  if (value === null) return "null";
  if (typeof value === "boolean") return value ? "true" : "false";
  if (typeof value === "number") {
    if (!Number.isFinite(value)) throw new Error("non-finite number");
    return floatPaths(path) ? formatFloat(value) : String(value);
  }
  if (typeof value === "string") return JSON.stringify(value);
  if (Array.isArray(value)) {
    if (value.length === 0) return "[]";
    const items = value.map((v) => padIn + encodeJson(v, floatPaths, `${path}[]`, indent + 1));
    return `[\n${items.join(",\n")}\n${pad}]`;
  }
  const keys = Object.keys(value);
  if (keys.length === 0) return "{}";
  const items = keys.map((k) => `${padIn}${JSON.stringify(k)}: ${encodeJson(value[k], floatPaths, `${path}.${k}`, indent + 1)}`);
  return `{\n${items.join(",\n")}\n${pad}}`;
}

function formatFloat(value: number): string {
  const text = String(value);
  return /[.eE]/.test(text) ? text : `${text}.0`;
}
//#endregion 🔖️Json

//#region 🔖️Writer
export type CaseFiles = { before: Json; after: Json; mutation: Json; diff: Json; outcome: Json; rust: string };

export function writeCase(leafDir: string, caseName: string, files: CaseFiles, isFloat: (path: string) => boolean): string[] {
  const root = join(leafDir, "🧪️tests", caseName);
  const written: string[] = [];
  const put = (relative: string, text: string) => {
    const target = join(root, relative);
    mkdirSync(join(target, ".."), { recursive: true });
    writeFileSync(target, text);
    written.push(target);
  };
  put("📸️snapshot/⬅️before/🔣️component.json", `${encodeJson(files.before, isFloat)}\n`);
  put("📸️snapshot/➡️after/🔣️component.json", `${encodeJson(files.after, isFloat)}\n`);
  put("🦠️mutation/🔣️component.json", `${encodeJson(files.mutation, isFloat)}\n`);
  put("🔺️diff/🔣️component.json", `${encodeJson(files.diff, isFloat)}\n`);
  put("🎯️outcome/🔣️component.json", `${encodeJson(files.outcome, isFloat)}\n`);
  put("🦀️component.rs", files.rust);
  return written;
}

export { fullDiff, clone };
//#endregion 🔖️Writer
