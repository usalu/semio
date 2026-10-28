/** 🧪️ Neutral master-override laws run through an independently supplied XML parser and writer. */
import assert from "node:assert/strict";
import { binary64, binary64Value, binary32 } from "../../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
import fixture from "../🧪️fixtures/🔣️.json";
import * as mapping from "../🟦️.ts";
export interface OverrideXmlOracle { read(source: string): mapping.XmlNode; write(node: mapping.XmlNode): string }
const bounds = (box: { x: number; y: number; width: number; height: number; rotation: number }): mapping.LayoutBounds => ({ x: binary64(box.x), y: binary64(box.y), w: binary64(box.width), h: binary64(box.height), rotation: binary64(box.rotation) });
function source(kind: string): mapping.Frame {
  const common = { id: "master-" + kind, layerId: "master-layer", bounds: bounds({ x: 5, y: 8, width: 50, height: 30, rotation: 0 }), visible: null, locked: null };
  if (kind === "text") return { ...common, kind, storyId: "shared-story", threadNext: null, columns: 1, inset: { x: binary64(0), y: binary64(0), w: binary64(0), h: binary64(0) }, wrapMode: "none" };
  if (kind === "image") return { ...common, kind, linkId: "embedded-image" };
  return { ...common, kind: "rect", fill: [binary32(0.2), binary32(0.3), binary32(0.4), binary32(0.5)], stroke: null };
}
function parent(frame: mapping.Frame): mapping.ParentPage { return { id: "master", name: "Parent", width: binary64(200), height: binary64(300), layerIds: ["master-layer"], layers: [], frames: [frame] }; }
function attr(node: mapping.XmlNode, key: string): string { if (node.kind !== "element") throw new Error("expected element"); const value = node.attrs.find((entry) => entry.name === key)?.value; if (value === undefined) throw new Error("missing attribute " + key); return value; }
function copyNode(frame: mapping.Frame): mapping.XmlNode {
  const b = frame.bounds, w = binary64Value(b.w), h = binary64Value(b.h), a = binary64Value(b.rotation), s = Math.sin(a), c = Math.cos(a), x = binary64Value(b.x), y = binary64Value(b.y);
  return { kind: "element", name: frame.kind === "text" ? "TextFrame" : "Rectangle", attrs: Object.entries({ Self: frame.id, ItemTransform: [c, s, -s, c, x + w / 2 - c * w / 2 + s * h / 2, y + h / 2 - s * w / 2 - c * h / 2].join(" "), GeometricBounds: [0, 0, h, w].join(" "), Visible: String(frame.visible ?? true), Locked: String(frame.locked ?? false) }).map(([name, value]) => ({ name, value })), children: [] };
}
function fromNode(node: mapping.XmlNode, source: mapping.Frame): mapping.Frame {
  const [a, b, , , tx, ty] = attr(node, "ItemTransform").split(" ").map(Number); const [top, left, bottom, right] = attr(node, "GeometricBounds").split(" ").map(Number); const w = right - left, h = bottom - top;
  return { ...structuredClone(source), id: attr(node, "Self"), bounds: bounds({ x: a * w / 2 - b * h / 2 + tx - w / 2, y: b * w / 2 + a * h / 2 + ty - h / 2, width: w, height: h, rotation: Math.atan2(b, a) }), visible: attr(node, "Visible") === "true", locked: attr(node, "Locked") === "true" };
}
/** ⚖️ Compares sparse native overrides with actual independently rewritten OverrideList and body-copy markup. */
export function verifyIdmlOverrides(oracle: OverrideXmlOracle): { cases: number } {
  let cases = 0;
  for (const row of fixture.cases) {
    const frame = source(row.kind ?? "rect"), master = parent(frame), saved = structuredClone(master);
    if (row.inputXml) { const run = () => mapping.readPageOverrides(master, oracle.read(row.inputXml!), row.spreadShowMasterItems ?? true); if (row.error) assert.throws(run, { message: row.error }); else assert.deepEqual(run(), row.expected!.overrides!.map((change) => ({ objectId: change.objectId, bounds: "bounds" in change ? bounds(change.bounds) : null, visible: "visible" in change ? change.visible : null, locked: null }))); cases++; continue; }
    if (!row.override) throw new Error("missing neutral override");
    const change: mapping.PageOverride = { objectId: row.override.objectId, bounds: row.override.bounds ? bounds(row.override.bounds) : null, visible: row.override.visible ?? null, locked: row.override.locked ?? null };
    const resolved = mapping.resolveOverride(master, change); const list = mapping.overrideList(master, [change]); assert.equal(list, frame.id);
    const page = oracle.read('<Page Self="body" AppliedMaster="master" OverrideList="' + list + '"/>'); const hidden = mapping.readPageOverrides(master, page); assert.equal(hidden[0].visible, false);
    const tagged = mapping.tagOverrideCopy(copyNode(resolved), change, "body-override-0"); const independent = oracle.read(oracle.write(tagged)); const candidate = fromNode(independent, resolved); assert.equal(attr(independent, "Self"), "body-override-0");
    if (row.rewrite && candidate.kind === "rect") candidate.fill = [binary32(0), binary32(0), binary32(0), binary32(1)];
    const recover = () => mapping.recoverOverride(master, hidden, independent, candidate);
    if (row.error) assert.throws(recover, { message: row.error }); else { const actual = recover()!; assert.equal(actual.objectId, change.objectId); assert.equal(actual.visible, change.visible); assert.equal(actual.locked, change.locked); if (change.bounds) { assert.ok(actual.bounds); for (const key of ["x", "y", "w", "h", "rotation"] as const) assert.ok(Math.abs(binary64Value(actual.bounds[key]) - binary64Value(change.bounds[key])) < 1e-8); } else assert.equal(actual.bounds, null); }
    assert.deepEqual(master, saved); assert.deepEqual(mapping.readPageOverrides(master, oracle.read('<Page Self="unaffected" AppliedMaster="master"/>')), []); cases++;
  }
  return { cases };
}
