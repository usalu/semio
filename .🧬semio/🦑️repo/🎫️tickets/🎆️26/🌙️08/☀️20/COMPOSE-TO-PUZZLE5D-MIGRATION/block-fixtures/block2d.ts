// ◻ block2d — 26 handcrafted mutation fixtures.
import { join } from "node:path";
import { REPO, BLOCK2D_DIFF_FIELDS, applyBlock2d, fullDiff, clone, writeCase } from "./emit.ts";
import { renderRust } from "./rust.ts";

const ROOT = join(REPO, "✏️s/🔌️plugins/🧱️block/🗿️artifacts/◻2d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");

const NODE_KIND = { id: "nk-valve", name: "valve", label: "Valve", variant: "a", description: "One inline valve node.", icon: "icon://valve", unit: "mm" };
const HK_SIGNAL = { id: "hk-signal", name: "signal", label: "Signal", color: "#ffcc00", defaultWireKind: "wire.link" };
const HK_POWER = { id: "hk-power", name: "power", label: "Power", color: "#cc2222", defaultWireKind: "wire.bus" };
const HANDLE_IN = { id: "handle-in", handleKind: "hk-signal", angle: 0.0, radius: 0.5 };
const RULE = { id: "compat-signal-signal", source: "hk-signal", target: "hk-signal", bidirectional: true };
const AUTHOR = { id: "author-ada", name: "Ada", email: "ada@example.org" };

const BASE = {
  schema: "block.2d",
  nodeKind: NODE_KIND,
  presentation: { shape: "circle", radius: 0.5, color: "#2288cc", iconKind: "icon.valve" },
  handleKinds: [HK_SIGNAL, HK_POWER],
  handles: [HANDLE_IN],
  compatibility: [RULE],
  attributes: [{ key: "material", value: "brass" }],
  authors: [AUTHOR],
  camera2d: { x: 0.0, y: 0.0, zoom: 1.0 },
  meta: { description: "Fixture base for the block2d mutation vocabulary." },
};

const kind = (patch: Record<string, unknown>) => ({ ...NODE_KIND, ...patch });
const hkPatch = (patch: Record<string, unknown>) => ({ patched: [{ id: "hk-power", patch: { replacement: { ...HK_POWER, ...patch } } }] });
const handlePatch = (patch: Record<string, unknown>) => ({ patched: [{ id: "handle-in", patch: { replacement: { ...HANDLE_IN, ...patch } } }] });

type Case = { leaf: string; name: string; summary: string; mutation: any; diff: any; assertion: string };

export const CASES: Case[] = [
  //#region 🔖️NodeKindIdentity
  { leaf: "✏️rename-node-kind", name: "renames-node-kind-to-gate", summary: "the node kind's `name` becomes `gate`",
    mutation: { mutation: "renameNodeKind", newName: "gate" }, diff: { nodeKind: kind({ name: "gate" }) },
    assertion: `assert_eq!(snapshot.node_kind.name, "gate", "rename-node-kind must rewrite only the identity name");` },
  { leaf: "🏷️change-node-kind-label", name: "relabels-node-kind", summary: "the node kind's `label` becomes `Inline Valve`",
    mutation: { mutation: "changeNodeKindLabel", newLabel: "Inline Valve" }, diff: { nodeKind: kind({ label: "Inline Valve" }) },
    assertion: `assert_eq!(snapshot.node_kind.label, "Inline Valve", "change-node-kind-label must rewrite only the identity label");` },
  { leaf: "🔀️change-node-kind-variant", name: "switches-variant-to-b", summary: "the node kind's `variant` moves from `a` to `b`",
    mutation: { mutation: "changeNodeKindVariant", newVariant: "b" }, diff: { nodeKind: kind({ variant: "b" }) },
    assertion: `assert_eq!(snapshot.node_kind.variant.as_deref(), Some("b"), "change-node-kind-variant must rewrite only the identity variant");` },
  { leaf: "📃️change-node-kind-description", name: "rewrites-node-kind-description", summary: "the node kind's own `description` is rewritten",
    mutation: { mutation: "changeNodeKindDescription", newDescription: "One inline valve node with a signal rim." }, diff: { nodeKind: kind({ description: "One inline valve node with a signal rim." }) },
    assertion: `assert_eq!(snapshot.node_kind.description, "One inline valve node with a signal rim.", "change-node-kind-description must rewrite the kind description, never the session meta description");` },
  { leaf: "🖼️change-node-kind-icon", name: "repoints-node-kind-icon", summary: "the node kind's `icon` is repointed",
    mutation: { mutation: "changeNodeKindIcon", newIcon: "icon://gate" }, diff: { nodeKind: kind({ icon: "icon://gate" }) },
    assertion: `assert_eq!((snapshot.node_kind.icon.as_deref(), snapshot.presentation.icon_kind.as_deref()), (Some("icon://gate"), Some("icon.valve")), "change-node-kind-icon must rewrite the identity icon, never the presentation iconKind");` },
  { leaf: "📐️change-node-kind-unit", name: "switches-unit-to-metre", summary: "the node kind's `unit` moves from millimetres to metres",
    mutation: { mutation: "changeNodeKindUnit", newUnit: "m" }, diff: { nodeKind: kind({ unit: "m" }) },
    assertion: `assert_eq!(snapshot.node_kind.unit.as_deref(), Some("m"), "change-node-kind-unit must rewrite only the identity unit");` },
  //#endregion 🔖️NodeKindIdentity
  //#region 🔖️Presentation
  { leaf: "🖌️update-presentation", name: "circle-to-rectangle", summary: "the whole rim presentation facet is replaced, circle → rectangle",
    mutation: { mutation: "updatePresentation", newShape: "rectangle", newRadius: null, newWidth: 1.2, newHeight: 0.8, newColor: "#112233", newIconKind: null },
    diff: { presentation: { shape: "rectangle", width: 1.2, height: 0.8, color: "#112233" } },
    assertion: `assert_eq!((snapshot.presentation.shape.as_deref(), snapshot.presentation.radius, snapshot.presentation.icon_kind.as_deref()), (Some("rectangle"), None, None), "update-presentation replaces the whole facet, so the old radius and iconKind must be cleared");` },
  //#endregion 🔖️Presentation
  //#region 🔖️HandleKinds
  { leaf: "🌱️create-handle-kind", name: "appends-ground-handle-kind", summary: "a third handle kind `hk-ground` is appended to the catalog",
    mutation: { mutation: "createHandleKind", handleKind: { id: "hk-ground", name: "ground", label: "Ground", color: "#448844", defaultWireKind: "wire.link" } },
    diff: { handleKinds: { added: [{ id: "hk-ground", name: "ground", label: "Ground", color: "#448844", defaultWireKind: "wire.link" }] } },
    assertion: `assert_eq!(snapshot.handle_kinds.last().map(|k| k.id.as_str()), Some("hk-ground"), "create-handle-kind must append the new kind last");` },
  { leaf: "🗑️delete-handle-kind", name: "removes-power-handle-kind", summary: "the `hk-power` handle-kind row is removed from the catalog",
    mutation: { mutation: "deleteHandleKind", id: "hk-power" }, diff: { handleKinds: { removed: ["hk-power"] } },
    assertion: `assert_eq!(snapshot.handle_kinds.iter().map(|k| k.id.as_str()).collect::<Vec<_>>(), vec!["hk-signal"], "delete-handle-kind must remove only hk-power, and must NOT cascade into the handles that reference it");` },
  { leaf: "✒️rename-handle-kind", name: "renames-power-to-mains", summary: "the `hk-power` handle kind's `name` becomes `mains`",
    mutation: { mutation: "renameHandleKind", id: "hk-power", newName: "mains" }, diff: { handleKinds: hkPatch({ name: "mains" }) },
    assertion: `assert_eq!(snapshot.handle_kinds[1].name, "mains", "rename-handle-kind must rewrite the name in place, keeping the row's position");` },
  { leaf: "🔖️change-handle-kind-label", name: "relabels-power-handle-kind", summary: "the `hk-power` handle kind's `label` becomes `Mains Power`",
    mutation: { mutation: "changeHandleKindLabel", id: "hk-power", newLabel: "Mains Power" }, diff: { handleKinds: hkPatch({ label: "Mains Power" }) },
    assertion: `assert_eq!((snapshot.handle_kinds[1].label.as_str(), snapshot.handle_kinds[1].name.as_str()), ("Mains Power", "power"), "change-handle-kind-label must move the label and leave the name untouched");` },
  { leaf: "🎨️change-handle-kind-color", name: "recolors-power-handle-kind", summary: "the `hk-power` handle kind's `color` becomes `#101010`",
    mutation: { mutation: "changeHandleKindColor", id: "hk-power", newColor: "#101010" }, diff: { handleKinds: hkPatch({ color: "#101010" }) },
    assertion: `assert_eq!((snapshot.handle_kinds[1].color.as_str(), snapshot.handle_kinds[0].color.as_str()), ("#101010", "#ffcc00"), "change-handle-kind-color must recolor only the addressed row");` },
  { leaf: "🔌️change-handle-kind-default-wire-kind", name: "swaps-power-default-wire-kind", summary: "the `hk-power` handle kind's `defaultWireKind` becomes `wire.heavy`",
    mutation: { mutation: "changeHandleKindDefaultWireKind", id: "hk-power", newDefaultWireKind: "wire.heavy" }, diff: { handleKinds: hkPatch({ defaultWireKind: "wire.heavy" }) },
    assertion: `assert_eq!((snapshot.handle_kinds[1].default_wire_kind.as_str(), snapshot.handle_kinds[0].default_wire_kind.as_str()), ("wire.heavy", "wire.link"), "change-handle-kind-default-wire-kind must touch only the addressed row");` },
  //#endregion 🔖️HandleKinds
  //#region 🔖️Handles
  { leaf: "🌿️create-handle", name: "appends-out-handle", summary: "a second rim-handle template `handle-out` is appended",
    mutation: { mutation: "createHandle", handle: { id: "handle-out", handleKind: "hk-power", angle: 3.0, radius: 0.5 } },
    diff: { handles: { added: [{ id: "handle-out", handleKind: "hk-power", angle: 3.0, radius: 0.5 }] } },
    assertion: `assert_eq!(snapshot.handles.last().map(|h| h.id.as_str()), Some("handle-out"), "create-handle must append the new template last");` },
  { leaf: "❌️delete-handle", name: "removes-in-handle", summary: "the `handle-in` rim-handle template is removed",
    mutation: { mutation: "deleteHandle", id: "handle-in" }, diff: { handles: { removed: ["handle-in"] } },
    assertion: `assert!(snapshot.handles.is_empty() && snapshot.handle_kinds.len() == 2, "delete-handle must drop the template without touching the handle-kind catalog");` },
  { leaf: "📍️move-handle", name: "swings-in-handle-along-the-rim", summary: "the `handle-in` template's `angle` and `radius` are moved",
    mutation: { mutation: "moveHandle", id: "handle-in", newAngle: 1.5, newRadius: 0.75 }, diff: { handles: handlePatch({ angle: 1.5, radius: 0.75 }) },
    assertion: `assert_eq!((snapshot.handles[0].angle, snapshot.handles[0].radius, snapshot.handles[0].handle_kind.as_str()), (1.5, 0.75, "hk-signal"), "move-handle must move the rim placement without re-kinding the handle");` },
  { leaf: "🧷️change-handle-handle-kind", name: "rekinds-in-handle-as-power", summary: "the `handle-in` template is re-pointed at the `hk-power` kind",
    mutation: { mutation: "changeHandleHandleKind", id: "handle-in", newHandleKind: "hk-power" }, diff: { handles: handlePatch({ handleKind: "hk-power" }) },
    assertion: `assert_eq!((snapshot.handles[0].handle_kind.as_str(), snapshot.handles[0].angle), ("hk-power", 0.0), "change-handle-handle-kind must re-point the template without moving it");` },
  //#endregion 🔖️Handles
  //#region 🔖️CompatibilityAttributesAuthors
  { leaf: "➕️add-compatibility-rule", name: "allows-signal-to-power", summary: "a `compat-signal-power` rule is appended to the compatibility table",
    mutation: { mutation: "addCompatibilityRule", rule: { id: "compat-signal-power", source: "hk-signal", target: "hk-power", bidirectional: false } },
    diff: { compatibility: { added: [{ id: "compat-signal-power", source: "hk-signal", target: "hk-power", bidirectional: false }] } },
    assertion: `assert_eq!(snapshot.compatibility.last().map(|r| (r.id.as_str(), r.bidirectional)), Some(("compat-signal-power", false)), "add-compatibility-rule must append the one-way rule verbatim");` },
  { leaf: "➖️remove-compatibility-rule", name: "revokes-signal-to-signal", summary: "the `compat-signal-signal` rule is removed from the compatibility table",
    mutation: { mutation: "removeCompatibilityRule", id: "compat-signal-signal" }, diff: { compatibility: { removed: ["compat-signal-signal"] } },
    assertion: `assert!(snapshot.compatibility.is_empty() && snapshot.handle_kinds.len() == 2, "remove-compatibility-rule must drop the row without disturbing the handle-kind catalog it names");` },
  { leaf: "🧩️add-attribute", name: "adds-pressure-attribute", summary: "a document-level `pressure` attribute is appended",
    mutation: { mutation: "addAttribute", attribute: { key: "pressure", value: "16" } }, diff: { attributes: { added: [{ key: "pressure", value: "16" }] } },
    assertion: `assert_eq!(snapshot.attributes.last().map(|a| (a.key.as_str(), a.value.as_str())), Some(("pressure", "16")), "add-attribute must append the key/value pair verbatim");` },
  { leaf: "🚫️remove-attribute", name: "drops-material-attribute", summary: "the document-level `material` attribute is removed",
    mutation: { mutation: "removeAttribute", key: "material" }, diff: { attributes: { removed: ["material"] } },
    assertion: `assert!(snapshot.attributes.is_empty(), "remove-attribute is keyed by attribute key, not by id");` },
  { leaf: "👤️add-author", name: "credits-bo", summary: "author `Bo` is appended to the credited author list",
    mutation: { mutation: "addAuthor", author: { id: "author-bo", name: "Bo" } }, diff: { authors: { values: [AUTHOR, { id: "author-bo", name: "Bo" }] } },
    assertion: `assert_eq!(snapshot.authors.iter().map(|a| a.id.as_str()).collect::<Vec<_>>(), vec!["author-ada", "author-bo"], "add-author rewrites the whole author list, so the incumbent must survive in place");` },
  { leaf: "🚷️remove-author", name: "uncredits-ada", summary: "author `Ada` is dropped from the credited author list",
    mutation: { mutation: "removeAuthor", id: "author-ada" }, diff: { authors: { values: [] } },
    assertion: `assert!(snapshot.authors.is_empty(), "remove-author rewrites the whole author list down to the survivors");` },
  //#endregion 🔖️CompatibilityAttributesAuthors
  //#region 🔖️CameraAndMeta
  { leaf: "🎥️move-camera2d", name: "pans-camera", summary: "the 2D camera pans to (12, -4)",
    mutation: { mutation: "moveCamera2d", newX: 12.0, newY: -4.0 }, diff: { camera2d: { x: 12.0, y: -4.0, zoom: 1.0 } },
    assertion: `assert_eq!((snapshot.camera2d.x, snapshot.camera2d.y, snapshot.camera2d.zoom), (12.0, -4.0, 1.0), "move-camera2d must pan without changing zoom");` },
  { leaf: "🔍️scale-camera2d", name: "zooms-camera-in", summary: "the 2D camera zoom rises to 2.5",
    mutation: { mutation: "scaleCamera2d", newZoom: 2.5 }, diff: { camera2d: { x: 0.0, y: 0.0, zoom: 2.5 } },
    assertion: `assert_eq!((snapshot.camera2d.zoom, snapshot.camera2d.x), (2.5, 0.0), "scale-camera2d must change zoom without panning");` },
  { leaf: "💬️change-meta-description", name: "rewrites-session-notes", summary: "the document's session `meta.description` is rewritten",
    mutation: { mutation: "changeMetaDescription", newDescription: "Reviewed during the fixture pass." }, diff: { meta: { description: "Reviewed during the fixture pass." } },
    assertion: `assert_eq!((snapshot.meta.description.as_str(), snapshot.node_kind.description.as_str()), ("Reviewed during the fixture pass.", "One inline valve node."), "change-meta-description must rewrite the session note, never the kind's own description");` },
  //#endregion 🔖️CameraAndMeta
];

export function emitBlock2d(): string[] {
  const written: string[] = [];
  for (const entry of CASES) {
    const diff = fullDiff(BLOCK2D_DIFF_FIELDS, new Set(["handleKinds", "handles", "compatibility", "attributes"]), new Set(["authors"]), entry.diff);
    const before = clone(BASE);
    const after = applyBlock2d(before, diff);
    const rust = renderRust({
      artifact: "block2d", snapshotType: "Block2dSnapshot", mutationType: "Block2dMutation", diffType: "Block2dDiff",
      applyFn: "apply_block2d_mutation", inverseFn: "inverse_block2d_mutation",
      leaf: entry.leaf.replace(/[^\x20-\x7e]/g, ""), caseName: entry.name, summary: entry.summary,
      beforePrelude: "", stateAssertion: entry.assertion,
    });
    written.push(...writeCase(join(ROOT, entry.leaf), entry.name, { before, after, mutation: entry.mutation, diff, outcome: { status: "applied" }, rust }, () => true));
  }
  return written;
}
