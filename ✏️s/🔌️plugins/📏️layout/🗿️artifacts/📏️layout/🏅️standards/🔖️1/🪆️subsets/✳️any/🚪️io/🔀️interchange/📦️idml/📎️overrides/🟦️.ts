/** 📎️ IDML master-item override semantics over owned Layout and XML values. */
import type { Frame, PageOverride, ParentPage, LayoutBounds } from "../../../../🧬️schema/🟦️.ts";
import type { XmlNode } from "../../../../../../../../../../../🗄️stdio/🗿️artifacts/📰️xml/🏅️standards/🔖️1.0/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🟦️.ts";
import { binary64, binary64Value } from "../../../../../../../../../../../../../🧰️framework/🔨️modules/🌱️value/🔢️ieee754/🟦️.ts";
export type { Frame, PageOverride, ParentPage, LayoutBounds, XmlNode };
const sourceKey = "semio:override-source";
const fieldsKey = "semio:override-fields";
const fail = (code: string): never => { throw new Error("layout.idml." + code); };
function attr(node: XmlNode, key: string): string | null { return node.kind === "element" ? node.attrs.find((entry) => entry.name === key)?.value ?? null : null; }
function nodes(node: XmlNode, name: string): XmlNode[] { return node.kind === "element" ? [...(node.name.split(":").at(-1) === name ? [node] : []), ...node.children.flatMap((child) => nodes(child, name))] : []; }
function label(node: XmlNode, key: string): string | null { const entries = nodes(node, "KeyValuePair").filter((entry) => attr(entry, "Key") === key); if (entries.length > 1) fail("override-label"); return entries.length ? attr(entries[0], "Value") : null; }
function element(name: string, values: Record<string, string>, children: XmlNode[] = []): XmlNode { return { kind: "element", name, attrs: Object.entries(values).map(([name, value]) => ({ name, value })), children }; }
function checkSource(parent: ParentPage | null, change: PageOverride): Frame {
  if (!change.objectId || change.objectId.length > 256 || /\s/u.test(change.objectId)) fail("override-envelope");
  const frame = parent?.frames.find((frame) => frame.id === change.objectId); if (!frame || !parent) return fail("override-source");
  if ((frame.kind === "text" && frame.threadNext !== null) || parent.frames.some((candidate) => candidate.kind === "text" && candidate.threadNext === frame.id)) fail("threaded-master-override");
  if (change.bounds && ([change.bounds.x, change.bounds.y, change.bounds.w, change.bounds.h, change.bounds.rotation].some((number) => !Number.isFinite(binary64Value(number))) || binary64Value(change.bounds.w) < 0 || binary64Value(change.bounds.h) < 0)) fail("override-bounds");
  return frame;
}
/** 🧬️ Materializes one override without changing the inherited source or owning asset bytes. */
export function resolveOverride(parent: ParentPage | null, change: PageOverride): Frame {
  const frame = structuredClone(checkSource(parent, change));
  if (change.bounds !== null) frame.bounds = structuredClone(change.bounds);
  if (change.visible !== null) frame.visible = change.visible;
  if (change.locked !== null) frame.locked = change.locked;
  return frame;
}
/** 📃️ Writes actual master Self references in native override order. */
export function overrideList(parent: ParentPage | null, changes: readonly PageOverride[]): string {
  if (changes.length > 64) fail("override-envelope");
  const seen = new Set<string>(); return changes.map((change) => { checkSource(parent, change); if (seen.has(change.objectId)) fail("duplicate-override"); seen.add(change.objectId); return change.objectId; }).join(" ");
}
/** 📎️ Adds only copy identity, inherited source identity and sparse presence metadata to actual editable markup. */
export function tagOverrideCopy(node: XmlNode, change: PageOverride, copyId: string): XmlNode {
  const copy = structuredClone(node); if (copy.kind !== "element") return fail("override-copy");
  const self = copy.attrs.find((entry) => entry.name === "Self"); if (!self || !copyId) return fail("override-copy"); self.value = copyId;
  const fields = (["bounds", "visible", "locked"] as const).filter((name) => change[name] !== null);
  const tag = element("Label", {}, [element("KeyValuePair", { Key: sourceKey, Value: change.objectId }), element("KeyValuePair", { Key: fieldsKey, Value: fields.join(" ") })]);
  const properties = copy.children.find((node) => node.kind === "element" && node.name === "Properties");
  if (properties?.kind === "element") properties.children.push(tag); else copy.children.unshift(element("Properties", {}, [tag]));
  return copy;
}
/** 📖️ External OverrideList entries suppress inherited items even when no replacement copy exists. */
export function readPageOverrides(parent: ParentPage | null, node: XmlNode, showMasterItems = true): PageOverride[] {
  const transform = (attr(node, "MasterPageTransform") ?? "1 0 0 1 0 0").trim().split(/\s+/u).map(Number);
  if (transform.length !== 6 || transform.some((number) => !Number.isFinite(number)) || transform[0] !== 1 || transform[1] !== 0 || transform[2] !== 0 || transform[3] !== 1) fail("master-page-transform");
  const ids = (attr(node, "OverrideList") ?? "").trim().split(/\s+/u).filter(Boolean); if (ids.length > 64) fail("override-envelope"); const seen = new Set<string>();
  const changes: PageOverride[] = ids.map((id) => { if (seen.has(id)) fail("duplicate-override"); seen.add(id); if (!parent?.frames.some((frame) => frame.id === id)) fail("override-source"); return { objectId: id, bounds: null, visible: false, locked: null }; });
  if (showMasterItems && transform[4] === 0 && transform[5] === 0) return changes;
  for (const frame of parent?.frames ?? []) {
    if (seen.has(frame.id)) continue; if (changes.length >= 64) fail("override-envelope");
    const box = structuredClone(frame.bounds); box.x = binary64(binary64Value(box.x) + transform[4]); box.y = binary64(binary64Value(box.y) + transform[5]);
    changes.push({ objectId: frame.id, bounds: showMasterItems ? box : null, visible: showMasterItems ? null : false, locked: null });
  }
  return changes;
}

function sameBounds(a: LayoutBounds, b: LayoutBounds): boolean { return (["x", "y", "w", "h", "rotation"] as const).every((key) => Math.abs(binary64Value(a[key]) - binary64Value(b[key])) < 1e-8); }
function sameContent(a: Frame, b: Frame): boolean {
  if (a.kind === "rect" && b.kind === "rect") return JSON.stringify([a.fill, a.stroke]) === JSON.stringify([b.fill, b.stroke]);
  if (a.kind === "image" && b.kind === "image") return a.linkId === b.linkId;
  if (a.kind === "text" && b.kind === "text") return a.storyId === b.storyId && a.threadNext === b.threadNext && a.columns === b.columns && a.wrapMode === b.wrapMode && (["x", "y", "w", "h"] as const).every((key) => a.inset[key].bits === b.inset[key].bits);
  return false;
}
/** 🔁️ Reconstructs supported sparse native overrides and refuses edits the native override cannot represent. */
export function recoverOverride(parent: ParentPage | null, changes: readonly PageOverride[], node: XmlNode, candidate: Frame): PageOverride | null {
  const id = label(node, sourceKey); if (id === null) return null; const change = changes.find((change) => change.objectId === id); if (!change) return fail("override-source"); const base = checkSource(parent, change);
  const fieldsValue = label(node, fieldsKey); if (fieldsValue === null) return fail("override-label"); const fields = fieldsValue.trim().split(/\s+/u).filter(Boolean);
  if (fields.length > 3 || new Set(fields).size !== fields.length || fields.some((field) => !["bounds", "visible", "locked"].includes(field))) fail("override-label");
  if (!sameContent(base, candidate) || (!fields.includes("bounds") && !sameBounds(base.bounds, candidate.bounds)) || (!fields.includes("visible") && (base.visible ?? true) !== (candidate.visible ?? true)) || (!fields.includes("locked") && (base.locked ?? false) !== (candidate.locked ?? false))) fail("override-copy-content");
  return { objectId: id, bounds: fields.includes("bounds") ? structuredClone(candidate.bounds) : null, visible: fields.includes("visible") ? candidate.visible ?? true : null, locked: fields.includes("locked") ? candidate.locked ?? false : null };
}
