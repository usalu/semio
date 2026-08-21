// 🧊️ block3d — 37 handcrafted mutation fixtures. The vortex-kind catalogue is a COMPOSED CHILD
// (`s.stdio.semio@v1/kit`): the persisted snapshot keeps only the content-addressed `catalog` handle
// plus the block-owned `vortexKindExtra` overflow, so every vortex-kind case's `before()` seeds the
// artifact's own working-scene cache exactly as a real loader does.
import { join } from "node:path";
import { REPO, BLOCK3D_DIFF_FIELDS, applyBlock3d, fullDiff, clone, writeCase } from "./emit.ts";
import { catalogChildId } from "./siphash.ts";
import { renderRust } from "./rust.ts";

const ROOT = join(REPO, "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");

const DOOR = { id: "door", name: "door", label: "Door", color: "hsl(206 52% 48%)", defaultCableKind: "cable.link" };
const HATCH = { id: "hatch", name: "hatch", label: "Hatch", color: "hsl(37 52% 48%)", defaultCableKind: "cable.bus" };
const SEEDED = [DOOR, HATCH];

const handle = (kinds: any[]) => {
  const id = catalogChildId(kinds.map((k) => ({ id: k.id, name: k.name, category: "vortex-kind" })));
  return { childId: id, target: { artifactId: id, dialect: { artifactKind: "s.stdio.semio", standard: "v1", subset: "kit" } } };
};
const extras = (kinds: any[]) => kinds.map((k) => ({ id: k.id, label: k.label, color: k.color, defaultCableKind: k.defaultCableKind }));

const OBJECT_KIND = { id: "ok-capsule", name: "capsule", label: "Capsule", variant: "a", description: "One habitation capsule shell.", icon: "icon://capsule", unit: "m" };
const REP_SHELL = { id: "rep-shell", name: "shell", meshUrl: "mesh://capsule/shell", tags: ["lod0", "printable"], lod: "lod0", description: "Outer shell mesh.", attributes: [{ key: "finish", value: "matte" }] };
const VORTEX_FRONT = { id: "vortex-front", vortexKind: "door", position: [0.0, -1.6, 1.2], direction: [0.0, -1.0, 0.0], radius: 0.3, label: "front door" };
const RULE = { id: "compat-door-door", source: "door", target: "door", bidirectional: true };
const AUTHOR = { id: "author-ada", name: "Ada", email: "ada@example.org" };

const BASE = {
  schema: "block.3d",
  objectKind: OBJECT_KIND,
  representations: [REP_SHELL],
  catalog: handle(SEEDED),
  vortexKindExtra: extras(SEEDED),
  vortices: [VORTEX_FRONT],
  compatibility: [RULE],
  attributes: [{ key: "material", value: "concrete" }],
  authors: [AUTHOR],
  camera3d: { position: [10.0, -10.0, 6.0], target: [0.0, 0.0, 1.0], zoom: 1.0 },
  meta: { description: "Fixture base for the block3d mutation vocabulary." },
};

const kind = (patch: Record<string, unknown>) => ({ ...OBJECT_KIND, ...patch });
const repPatch = (patch: Record<string, unknown>) => ({ patched: [{ id: "rep-shell", patch: { replacement: { ...clone(REP_SHELL), ...patch } } }] });
const vkPatch = (id: string, patch: Record<string, unknown>) => ({ patched: [{ id, patch: { replacement: { ...(id === "door" ? DOOR : HATCH), ...patch } } }] });
const vxPatch = (patch: Record<string, unknown>) => ({ patched: [{ id: "vortex-front", patch: { replacement: { ...clone(VORTEX_FRONT), ...patch } } }] });

const PRELUDE = `    crate::artifacts::block3d::seed_vortex_kind_catalog_scratch(&[
        crate::artifacts::block3d::Block3dVortexKind { id: "door".into(), name: "door".into(), label: "Door".into(), color: "hsl(206 52% 48%)".into(), default_cable_kind: "cable.link".into() },
        crate::artifacts::block3d::Block3dVortexKind { id: "hatch".into(), name: "hatch".into(), label: "Hatch".into(), color: "hsl(37 52% 48%)".into(), default_cable_kind: "cable.bus".into() },
    ]);
`;

type Case = { leaf: string; name: string; summary: string; mutation: any; diff: any; assertion: string };

export const CASES: Case[] = [
  //#region 🔖️ObjectKindIdentity
  { leaf: "✏️rename-object-kind", name: "renames-object-kind-to-pod", summary: "the object kind's `name` becomes `pod`",
    mutation: { mutation: "renameObjectKind", newName: "pod" }, diff: { objectKind: kind({ name: "pod" }) },
    assertion: `assert_eq!(snapshot.object_kind.name, "pod", "rename-object-kind must rewrite only the identity name");` },
  { leaf: "🏷️change-object-kind-label", name: "relabels-object-kind", summary: "the object kind's `label` becomes `Habitation Capsule`",
    mutation: { mutation: "changeObjectKindLabel", newLabel: "Habitation Capsule" }, diff: { objectKind: kind({ label: "Habitation Capsule" }) },
    assertion: `assert_eq!(snapshot.object_kind.label, "Habitation Capsule", "change-object-kind-label must rewrite only the identity label");` },
  { leaf: "🔀️change-object-kind-variant", name: "switches-variant-to-b", summary: "the object kind's `variant` moves from `a` to `b`",
    mutation: { mutation: "changeObjectKindVariant", newVariant: "b" }, diff: { objectKind: kind({ variant: "b" }) },
    assertion: `assert_eq!(snapshot.object_kind.variant.as_deref(), Some("b"), "change-object-kind-variant must rewrite only the identity variant");` },
  { leaf: "📃️change-object-kind-description", name: "rewrites-object-kind-description", summary: "the object kind's own `description` is rewritten",
    mutation: { mutation: "changeObjectKindDescription", newDescription: "One habitation capsule shell with a door vortex." }, diff: { objectKind: kind({ description: "One habitation capsule shell with a door vortex." }) },
    assertion: `assert_eq!(snapshot.object_kind.description, "One habitation capsule shell with a door vortex.", "change-object-kind-description must rewrite the kind description, never the session meta description");` },
  { leaf: "🖼️change-object-kind-icon", name: "repoints-object-kind-icon", summary: "the object kind's `icon` is repointed",
    mutation: { mutation: "changeObjectKindIcon", newIcon: "icon://pod" }, diff: { objectKind: kind({ icon: "icon://pod" }) },
    assertion: `assert_eq!(snapshot.object_kind.icon.as_deref(), Some("icon://pod"), "change-object-kind-icon must rewrite only the identity icon");` },
  { leaf: "📐change-object-kind-unit", name: "switches-unit-to-centimeter", summary: "the object kind's `unit` moves from metres to centimetres",
    mutation: { mutation: "changeObjectKindUnit", newUnit: "cm" }, diff: { objectKind: kind({ unit: "cm" }) },
    assertion: `assert_eq!(snapshot.object_kind.unit.as_deref(), Some("cm"), "change-object-kind-unit must rewrite only the identity unit");` },
  //#endregion 🔖️ObjectKindIdentity
  //#region 🔖️Representations
  { leaf: "🧱create-representation", name: "appends-frame-representation", summary: "a second representation `rep-frame` is appended to the catalog",
    mutation: { mutation: "createRepresentation", representation: { id: "rep-frame", name: "frame", meshUrl: "mesh://capsule/frame", tags: ["lod1"], lod: "lod1", description: "Structural frame mesh.", attributes: [] } },
    diff: { representations: { added: [{ id: "rep-frame", name: "frame", meshUrl: "mesh://capsule/frame", tags: ["lod1"], lod: "lod1", description: "Structural frame mesh.", attributes: [] }] } },
    assertion: `assert_eq!(snapshot.representations.last().map(|r| r.id.as_str()), Some("rep-frame"), "create-representation must append the new representation last");` },
  { leaf: "🗑delete-representation", name: "removes-shell-representation", summary: "the `rep-shell` representation is removed",
    mutation: { mutation: "deleteRepresentation", id: "rep-shell" }, diff: { representations: { removed: ["rep-shell"] } },
    assertion: `assert!(snapshot.representations.is_empty(), "delete-representation must drop rep-shell and touch no other collection");` },
  { leaf: "✒rename-representation", name: "renames-shell-to-hull", summary: "the `rep-shell` representation's `name` becomes `hull`",
    mutation: { mutation: "renameRepresentation", id: "rep-shell", newName: "hull" }, diff: { representations: repPatch({ name: "hull" }) },
    assertion: `assert_eq!(snapshot.representations[0].name, "hull", "rename-representation must rewrite only that representation's name");` },
  { leaf: "🌐change-representation-mesh-url", name: "repoints-shell-mesh-url", summary: "the `rep-shell` representation points at a new mesh URL",
    mutation: { mutation: "changeRepresentationMeshUrl", id: "rep-shell", newMeshUrl: "mesh://capsule/shell-v2" }, diff: { representations: repPatch({ meshUrl: "mesh://capsule/shell-v2" }) },
    assertion: `assert_eq!(snapshot.representations[0].mesh_url.as_deref(), Some("mesh://capsule/shell-v2"), "change-representation-mesh-url must repoint only meshUrl");` },
  { leaf: "🏔change-representation-lod", name: "promotes-shell-to-lod2", summary: "the `rep-shell` representation's `lod` moves to `lod2`",
    mutation: { mutation: "changeRepresentationLod", id: "rep-shell", newLod: "lod2" }, diff: { representations: repPatch({ lod: "lod2" }) },
    assertion: `assert_eq!((snapshot.representations[0].lod.as_deref(), snapshot.representations[0].tags.len()), (Some("lod2"), 2), "change-representation-lod must move the lod field and leave the tag list alone");` },
  { leaf: "📜change-representation-description", name: "rewrites-shell-description", summary: "the `rep-shell` representation's `description` is rewritten",
    mutation: { mutation: "changeRepresentationDescription", id: "rep-shell", newDescription: "Outer shell mesh, watertight." }, diff: { representations: repPatch({ description: "Outer shell mesh, watertight." }) },
    assertion: `assert_eq!(snapshot.representations[0].description, "Outer shell mesh, watertight.", "change-representation-description must rewrite only that representation's description");` },
  { leaf: "🔖add-representation-tag", name: "tags-shell-as-structural", summary: "`structural` is appended to the `rep-shell` tag list",
    mutation: { mutation: "addRepresentationTag", id: "rep-shell", tag: "structural" }, diff: { representations: repPatch({ tags: ["lod0", "printable", "structural"] }) },
    assertion: `assert_eq!(snapshot.representations[0].tags, vec!["lod0".to_string(), "printable".to_string(), "structural".to_string()], "add-representation-tag must append the tag last and keep the existing ones");` },
  { leaf: "🚫remove-representation-tag", name: "untags-shell-printable", summary: "`printable` is dropped from the `rep-shell` tag list",
    mutation: { mutation: "removeRepresentationTag", id: "rep-shell", tag: "printable" }, diff: { representations: repPatch({ tags: ["lod0"] }) },
    assertion: `assert_eq!(snapshot.representations[0].tags, vec!["lod0".to_string()], "remove-representation-tag must drop only the named tag");` },
  { leaf: "🧩add-representation-attribute", name: "adds-color-attribute-to-shell", summary: "a `color` attribute is appended to `rep-shell`'s nested attribute table",
    mutation: { mutation: "addRepresentationAttribute", id: "rep-shell", attribute: { key: "color", value: "bone" } },
    diff: { representations: repPatch({ attributes: [{ key: "finish", value: "matte" }, { key: "color", value: "bone" }] }) },
    assertion: `assert_eq!(snapshot.representations[0].attributes.last().map(|a| a.key.as_str()), Some("color"), "add-representation-attribute must append to the representation's own attribute table, never the document one");` },
  { leaf: "➖remove-representation-attribute", name: "drops-finish-attribute-from-shell", summary: "the `finish` attribute is dropped from `rep-shell`'s nested attribute table",
    mutation: { mutation: "removeRepresentationAttribute", id: "rep-shell", key: "finish" }, diff: { representations: repPatch({ attributes: [] }) },
    assertion: `assert!(snapshot.representations[0].attributes.is_empty() && snapshot.attributes.len() == 1, "remove-representation-attribute must empty the representation's table and leave the document attributes alone");` },
  //#endregion 🔖️Representations
  //#region 🔖️VortexKindsComposedChild
  { leaf: "🌱create-vortex-kind", name: "appends-vent-vortex-kind", summary: "a third vortex kind `vent` joins the composed catalogue, re-minting the `catalog` child handle",
    mutation: { mutation: "createVortexKind", vortexKind: { id: "vent", name: "vent", label: "Vent", color: "hsl(124 52% 48%)", defaultCableKind: "cable.link" } },
    diff: { vortexKinds: { added: [{ id: "vent", name: "vent", label: "Vent", color: "hsl(124 52% 48%)", defaultCableKind: "cable.link" }] } },
    assertion: `assert_eq!(snapshot.vortex_kind_extra.last().map(|e| e.id.as_str()), Some("vent"), "create-vortex-kind must append the overflow row and re-mint the content-addressed catalog handle");` },
  { leaf: "❌delete-vortex-kind", name: "removes-hatch-vortex-kind", summary: "the `hatch` vortex kind leaves the composed catalogue, re-minting the `catalog` child handle",
    mutation: { mutation: "deleteVortexKind", id: "hatch" }, diff: { vortexKinds: { removed: ["hatch"] } },
    assertion: `assert_eq!(snapshot.vortex_kind_extra.iter().map(|e| e.id.as_str()).collect::<Vec<_>>(), vec!["door"], "delete-vortex-kind must drop the overflow row and must NOT cascade into the vortices that reference it");` },
  { leaf: "🖋rename-vortex-kind", name: "renames-door-to-portal", summary: "the `door` vortex kind's `name` becomes `portal` — the half that lives in the composed kit child, so the handle changes while the overflow row does not",
    mutation: { mutation: "renameVortexKind", id: "door", newName: "portal" }, diff: { vortexKinds: vkPatch("door", { name: "portal" }) },
    assertion: `assert_eq!(snapshot.vortex_kind_extra[0], crate::artifacts::block3d::Block3dVortexKindExtra { id: "door".into(), label: "Door".into(), color: "hsl(206 52% 48%)".into(), default_cable_kind: "cable.link".into() }, "rename-vortex-kind touches the kit half only: the block-owned overflow row must come through byte-identical");` },
  { leaf: "🎫change-vortex-kind-label", name: "relabels-door-vortex-kind", summary: "the `door` vortex kind's `label` becomes `Front Door` — block-owned overflow, so the `catalog` handle is untouched",
    mutation: { mutation: "changeVortexKindLabel", id: "door", newLabel: "Front Door" }, diff: { vortexKinds: vkPatch("door", { label: "Front Door" }) },
    assertion: `assert_eq!((snapshot.vortex_kind_extra[0].label.as_str(), snapshot.catalog.child_id.as_str()), ("Front Door", "${handle(SEEDED).childId}"), "change-vortex-kind-label lives entirely in the overflow half, so the composed catalog handle must not move");` },
  { leaf: "🎨change-vortex-kind-color", name: "recolors-door-vortex-kind", summary: "the `door` vortex kind's `color` becomes `hsl(0 0% 10%)` — block-owned overflow, so the `catalog` handle is untouched",
    mutation: { mutation: "changeVortexKindColor", id: "door", newColor: "hsl(0 0% 10%)" }, diff: { vortexKinds: vkPatch("door", { color: "hsl(0 0% 10%)" }) },
    assertion: `assert_eq!((snapshot.vortex_kind_extra[0].color.as_str(), snapshot.catalog.child_id.as_str()), ("hsl(0 0% 10%)", "${handle(SEEDED).childId}"), "change-vortex-kind-color lives entirely in the overflow half, so the composed catalog handle must not move");` },
  { leaf: "🔌change-vortex-kind-default-cable-kind", name: "swaps-door-default-cable-kind", summary: "the `door` vortex kind's `defaultCableKind` becomes `cable.heavy` — block-owned overflow, so the `catalog` handle is untouched",
    mutation: { mutation: "changeVortexKindDefaultCableKind", id: "door", newDefaultCableKind: "cable.heavy" }, diff: { vortexKinds: vkPatch("door", { defaultCableKind: "cable.heavy" }) },
    assertion: `assert_eq!((snapshot.vortex_kind_extra[0].default_cable_kind.as_str(), snapshot.vortex_kind_extra[1].default_cable_kind.as_str()), ("cable.heavy", "cable.bus"), "change-vortex-kind-default-cable-kind must touch only the addressed overflow row");` },
  //#endregion 🔖️VortexKindsComposedChild
  //#region 🔖️Vortices
  { leaf: "🌀create-vortex", name: "appends-rear-vortex", summary: "a second vortex template `vortex-rear` is appended",
    mutation: { mutation: "createVortex", vortex: { id: "vortex-rear", vortexKind: "hatch", position: [0.0, 1.6, 1.2], direction: [0.0, 1.0, 0.0], radius: 0.2, label: "rear hatch" } },
    diff: { vortices: { added: [{ id: "vortex-rear", vortexKind: "hatch", position: [0.0, 1.6, 1.2], direction: [0.0, 1.0, 0.0], radius: 0.2, label: "rear hatch" }] } },
    assertion: `assert_eq!(snapshot.vortices.last().map(|v| v.id.as_str()), Some("vortex-rear"), "create-vortex must append the new template last");` },
  { leaf: "🕳delete-vortex", name: "removes-front-vortex", summary: "the `vortex-front` template is removed",
    mutation: { mutation: "deleteVortex", id: "vortex-front" }, diff: { vortices: { removed: ["vortex-front"] } },
    assertion: `assert!(snapshot.vortices.is_empty() && snapshot.vortex_kind_extra.len() == 2, "delete-vortex must drop the template without touching the vortex-kind catalogue");` },
  { leaf: "📍move-vortex", name: "repositions-front-vortex", summary: "the `vortex-front` template's `position` and `direction` are moved",
    mutation: { mutation: "moveVortex", id: "vortex-front", newPosition: [1.0, 2.0, 3.0], newDirection: [1.0, 0.0, 0.0] }, diff: { vortices: vxPatch({ position: [1.0, 2.0, 3.0], direction: [1.0, 0.0, 0.0] }) },
    assertion: `assert_eq!((snapshot.vortices[0].position, snapshot.vortices[0].direction, snapshot.vortices[0].radius), ([1.0, 2.0, 3.0], [1.0, 0.0, 0.0], 0.3), "move-vortex must move pose only, leaving the radius alone");` },
  { leaf: "📏resize-vortex", name: "widens-front-vortex", summary: "the `vortex-front` template's `radius` widens to 0.75",
    mutation: { mutation: "resizeVortex", id: "vortex-front", newRadius: 0.75 }, diff: { vortices: vxPatch({ radius: 0.75 }) },
    assertion: `assert_eq!((snapshot.vortices[0].radius, snapshot.vortices[0].position), (0.75, [0.0, -1.6, 1.2]), "resize-vortex must widen the radius without moving the vortex");` },
  { leaf: "🧷change-vortex-vortex-kind", name: "rekinds-front-vortex-as-hatch", summary: "the `vortex-front` template is re-pointed at the `hatch` kind",
    mutation: { mutation: "changeVortexVortexKind", id: "vortex-front", newVortexKind: "hatch" }, diff: { vortices: vxPatch({ vortexKind: "hatch" }) },
    assertion: `assert_eq!((snapshot.vortices[0].vortex_kind.as_str(), snapshot.vortex_kind_extra.len()), ("hatch", 2), "change-vortex-vortex-kind must re-point the template without adding or removing a catalogue row");` },
  { leaf: "🪧change-vortex-label", name: "relabels-front-vortex", summary: "the `vortex-front` template's optional `label` becomes `main hatch`",
    mutation: { mutation: "changeVortexLabel", id: "vortex-front", newLabel: "main hatch" }, diff: { vortices: vxPatch({ label: "main hatch" }) },
    assertion: `assert_eq!(snapshot.vortices[0].label.as_deref(), Some("main hatch"), "change-vortex-label must rewrite only the template's own optional label");` },
  //#endregion 🔖️Vortices
  //#region 🔖️CompatibilityAttributesAuthors
  { leaf: "➕add-compatibility-rule", name: "allows-door-to-hatch", summary: "a `compat-door-hatch` rule is appended to the compatibility table",
    mutation: { mutation: "addCompatibilityRule", rule: { id: "compat-door-hatch", source: "door", target: "hatch", bidirectional: false } },
    diff: { compatibility: { added: [{ id: "compat-door-hatch", source: "door", target: "hatch", bidirectional: false }] } },
    assertion: `assert_eq!(snapshot.compatibility.last().map(|r| (r.id.as_str(), r.bidirectional)), Some(("compat-door-hatch", false)), "add-compatibility-rule must append the one-way rule verbatim");` },
  { leaf: "✂remove-compatibility-rule", name: "revokes-door-to-door", summary: "the `compat-door-door` rule is removed from the compatibility table",
    mutation: { mutation: "removeCompatibilityRule", id: "compat-door-door" }, diff: { compatibility: { removed: ["compat-door-door"] } },
    assertion: `assert!(snapshot.compatibility.is_empty() && snapshot.vortex_kind_extra.len() == 2, "remove-compatibility-rule must drop the row without disturbing the vortex-kind catalogue it names");` },
  { leaf: "🔩add-attribute", name: "adds-weight-attribute", summary: "a document-level `weight` attribute is appended",
    mutation: { mutation: "addAttribute", attribute: { key: "weight", value: "1400" } }, diff: { attributes: { added: [{ key: "weight", value: "1400" }] } },
    assertion: `assert_eq!(snapshot.attributes.last().map(|a| a.key.as_str()), Some("weight"), "add-attribute must append to the document attribute table, not a representation's");` },
  { leaf: "🚷remove-attribute", name: "drops-material-attribute", summary: "the document-level `material` attribute is removed",
    mutation: { mutation: "removeAttribute", key: "material" }, diff: { attributes: { removed: ["material"] } },
    assertion: `assert!(snapshot.attributes.is_empty() && snapshot.representations[0].attributes.len() == 1, "remove-attribute is keyed by attribute key and must leave the representation's own table alone");` },
  { leaf: "👤add-author", name: "credits-bo", summary: "author `Bo` is appended to the credited author list",
    mutation: { mutation: "addAuthor", author: { id: "author-bo", name: "Bo" } }, diff: { authors: { values: [AUTHOR, { id: "author-bo", name: "Bo" }] } },
    assertion: `assert_eq!(snapshot.authors.iter().map(|a| a.id.as_str()).collect::<Vec<_>>(), vec!["author-ada", "author-bo"], "add-author rewrites the whole author list, so the incumbent must survive in place");` },
  { leaf: "🙅remove-author", name: "uncredits-ada", summary: "author `Ada` is dropped from the credited author list",
    mutation: { mutation: "removeAuthor", id: "author-ada" }, diff: { authors: { values: [] } },
    assertion: `assert!(snapshot.authors.is_empty(), "remove-author rewrites the whole author list down to the survivors");` },
  //#endregion 🔖️CompatibilityAttributesAuthors
  //#region 🔖️CameraAndMeta
  { leaf: "🎥move-camera3d", name: "orbits-camera", summary: "the 3D camera moves to (5, 5, 5) looking at (0, 1, 0)",
    mutation: { mutation: "moveCamera3d", newPosition: [5.0, 5.0, 5.0], newTarget: [0.0, 1.0, 0.0] }, diff: { camera3d: { position: [5.0, 5.0, 5.0], target: [0.0, 1.0, 0.0], zoom: 1.0 } },
    assertion: `assert_eq!((snapshot.camera3d.position, snapshot.camera3d.target, snapshot.camera3d.zoom), ([5.0, 5.0, 5.0], [0.0, 1.0, 0.0], 1.0), "move-camera3d must move position and target while keeping zoom");` },
  { leaf: "🔍scale-camera3d", name: "zooms-camera-out", summary: "the 3D camera zoom falls to 0.5",
    mutation: { mutation: "scaleCamera3d", newZoom: 0.5 }, diff: { camera3d: { position: [10.0, -10.0, 6.0], target: [0.0, 0.0, 1.0], zoom: 0.5 } },
    assertion: `assert_eq!((snapshot.camera3d.zoom, snapshot.camera3d.position), (0.5, [10.0, -10.0, 6.0]), "scale-camera3d must change zoom without moving the eye");` },
  { leaf: "💬change-meta-description", name: "rewrites-session-notes", summary: "the document's session `meta.description` is rewritten",
    mutation: { mutation: "changeMetaDescription", newDescription: "Reviewed during the fixture pass." }, diff: { meta: { description: "Reviewed during the fixture pass." } },
    assertion: `assert_eq!((snapshot.meta.description.as_str(), snapshot.object_kind.description.as_str()), ("Reviewed during the fixture pass.", "One habitation capsule shell."), "change-meta-description must rewrite the session note, never the kind's own description");` },
  //#endregion 🔖️CameraAndMeta
];

export function emitBlock3d(): string[] {
  const written: string[] = [];
  for (const entry of CASES) {
    const diff = fullDiff(BLOCK3D_DIFF_FIELDS, new Set(["representations", "vortexKinds", "vortices", "compatibility", "attributes"]), new Set(["authors"]), entry.diff);
    const before = clone(BASE);
    const after = applyBlock3d(before, diff, SEEDED);
    const rust = renderRust({
      artifact: "block3d", snapshotType: "Block3dSnapshot", mutationType: "Block3dMutation", diffType: "Block3dDiff",
      applyFn: "apply_block3d_mutation", inverseFn: "inverse_block3d_mutation",
      leaf: entry.leaf.replace(/[^\x20-\x7e]/g, ""), caseName: entry.name, summary: entry.summary,
      beforePrelude: PRELUDE, stateAssertion: entry.assertion,
    });
    written.push(...writeCase(join(ROOT, entry.leaf), entry.name, { before, after, mutation: entry.mutation, diff, outcome: { status: "applied" }, rust }, () => true));
  }
  return written;
}
