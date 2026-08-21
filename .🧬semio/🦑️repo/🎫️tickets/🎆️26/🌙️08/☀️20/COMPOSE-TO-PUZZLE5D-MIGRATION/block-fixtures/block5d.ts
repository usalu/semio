// 🖐️ block5d — 41 handcrafted mutation fixtures.
import { join } from "node:path";
import { REPO, BLOCK5D_DIFF_FIELDS, applyBlock5d, fullDiff, clone, writeCase } from "./emit.ts";
import { renderRust } from "./rust.ts";

const ROOT = join(REPO, "✏️s/🔌️plugins/🧱️block/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations");

const PART_KIND = { id: "pk-capsule", name: "capsule", label: "Capsule", variant: "a", description: "One habitation capsule.", icon: "icon://capsule", unit: "m" };
const REP_SHELL = { id: "rep-shell", name: "shell", meshUrl: "mesh://capsule/shell", tags: ["lod0", "printable"], lod: "lod0", description: "Outer shell mesh.", attributes: [{ key: "finish", value: "matte" }] };
const GK_SOCKET = { id: "gk-socket", name: "socket", label: "Socket", color: "#00aa88", defaultRopeKind: "rope.link" };
const GK_PLUG = { id: "gk-plug", name: "plug", label: "Plug", color: "#ff8800", defaultRopeKind: "rope.link" };
const GRIP_NORTH = { id: "grip-north", gripKind: "gk-plug", angle: 0.0, radius2d: 0.3, position: [0.0, 1.0, 0.0], direction: [0.0, 1.0, 0.0], radius3d: 0.25 };
const RULE = { id: "compat-plug-plug", source: "gk-plug", target: "gk-plug", bidirectional: true };
const ATTRIBUTE = { key: "material", value: "concrete" };
const AUTHOR = { id: "author-ada", name: "Ada", email: "ada@example.org" };

const BASE = {
  schema: "block.5d",
  partKind: PART_KIND,
  "2d": { shape: "circle", radius: 0.5, color: "#3366ff", iconKind: "icon.capsule" },
  "3d": { orientation: [0.0, 0.0, 0.0, 1.0], scale: [1.0, 1.0, 1.0] },
  representations: [REP_SHELL],
  gripKinds: [GK_SOCKET, GK_PLUG],
  grips: [GRIP_NORTH],
  compatibility: [RULE],
  attributes: [ATTRIBUTE],
  authors: [AUTHOR],
  camera2d: { x: 0.0, y: 0.0, zoom: 1.0 },
  camera3d: { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], zoom: 1.0 },
  meta: { description: "Fixture base for the block5d mutation vocabulary." },
};

const kind = (patch: Record<string, unknown>) => ({ ...PART_KIND, ...patch });
const repPatch = (patch: Record<string, unknown>) => ({ patched: [{ id: "rep-shell", patch: { replacement: { ...clone(REP_SHELL), ...patch } } }] });
const gkPatch = (patch: Record<string, unknown>) => ({ patched: [{ id: "gk-plug", patch: { replacement: { ...GK_PLUG, ...patch } } }] });
const gripPatch = (patch: Record<string, unknown>) => ({ patched: [{ id: "grip-north", patch: { replacement: { ...clone(GRIP_NORTH), ...patch } } }] });

type Case = { leaf: string; name: string; summary: string; mutation: any; diff: any; assertion: string };

export const CASES: Case[] = [
  //#region 🔖️PartKindIdentity
  { leaf: "✏️rename-part-kind", name: "renames-part-kind-to-pod", summary: "the part kind's `name` becomes `pod`",
    mutation: { mutation: "renamePartKind", newName: "pod" }, diff: { partKind: kind({ name: "pod" }) },
    assertion: `assert_eq!(snapshot.part_kind.name, "pod", "rename-part-kind must rewrite only the identity name");` },
  { leaf: "🏷️change-part-kind-label", name: "relabels-part-kind", summary: "the part kind's `label` becomes `Habitation Capsule`",
    mutation: { mutation: "changePartKindLabel", newLabel: "Habitation Capsule" }, diff: { partKind: kind({ label: "Habitation Capsule" }) },
    assertion: `assert_eq!(snapshot.part_kind.label, "Habitation Capsule", "change-part-kind-label must rewrite only the identity label");` },
  { leaf: "🔀️change-part-kind-variant", name: "switches-variant-to-b", summary: "the part kind's `variant` moves from `a` to `b`",
    mutation: { mutation: "changePartKindVariant", newVariant: "b" }, diff: { partKind: kind({ variant: "b" }) },
    assertion: `assert_eq!(snapshot.part_kind.variant.as_deref(), Some("b"), "change-part-kind-variant must rewrite only the identity variant");` },
  { leaf: "📃️change-part-kind-description", name: "rewrites-part-kind-description", summary: "the part kind's own `description` is rewritten",
    mutation: { mutation: "changePartKindDescription", newDescription: "One habitation capsule with a plug rim." }, diff: { partKind: kind({ description: "One habitation capsule with a plug rim." }) },
    assertion: `assert_eq!(snapshot.part_kind.description, "One habitation capsule with a plug rim.", "change-part-kind-description must rewrite the kind description, never the session meta description");` },
  { leaf: "🖼️change-part-kind-icon", name: "repoints-part-kind-icon", summary: "the part kind's `icon` is repointed",
    mutation: { mutation: "changePartKindIcon", newIcon: "icon://pod" }, diff: { partKind: kind({ icon: "icon://pod" }) },
    assertion: `assert_eq!(snapshot.part_kind.icon.as_deref(), Some("icon://pod"), "change-part-kind-icon must rewrite the identity icon, not the 2D iconKind");` },
  { leaf: "📐change-part-kind-unit", name: "switches-unit-to-centimeter", summary: "the part kind's `unit` moves from metres to centimetres",
    mutation: { mutation: "changePartKindUnit", newUnit: "cm" }, diff: { partKind: kind({ unit: "cm" }) },
    assertion: `assert_eq!(snapshot.part_kind.unit.as_deref(), Some("cm"), "change-part-kind-unit must rewrite only the identity unit");` },
  //#endregion 🔖️PartKindIdentity
  //#region 🔖️Presentation
  { leaf: "🖌️update-part-2d", name: "circle-to-rectangle", summary: "the whole 2D presentation facet is replaced, circle → rectangle",
    mutation: { mutation: "updatePart2d", newShape: "rectangle", newRadius: null, newWidth: 1.2, newHeight: 0.8, newColor: "#112233", newIconKind: null },
    diff: { part2d: { shape: "rectangle", width: 1.2, height: 0.8, color: "#112233" } },
    assertion: `assert_eq!((snapshot.part_2d.shape.as_deref(), snapshot.part_2d.radius, snapshot.part_2d.width), (Some("rectangle"), None, Some(1.2)), "update-part-2d replaces the whole 2D facet, so the old radius must be cleared");` },
  { leaf: "🧊update-part-3d", name: "reorients-and-rescales-part", summary: "the whole 3D pose facet is replaced with a new orientation and scale",
    mutation: { mutation: "updatePart3d", newOrientation: [0.0, 0.0, 1.0, 0.0], newScale: [2.0, 1.0, 0.5] },
    diff: { part3d: { orientation: [0.0, 0.0, 1.0, 0.0], scale: [2.0, 1.0, 0.5] } },
    assertion: `assert_eq!((snapshot.part_3d.orientation, snapshot.part_3d.scale), (Some([0.0, 0.0, 1.0, 0.0]), Some([2.0, 1.0, 0.5])), "update-part-3d must replace the pose facet wholesale");` },
  //#endregion 🔖️Presentation
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
  //#region 🔖️GripKinds
  { leaf: "🌱create-grip-kind", name: "appends-hook-grip-kind", summary: "a third grip kind `gk-hook` is appended to the catalog",
    mutation: { mutation: "createGripKind", gripKind: { id: "gk-hook", name: "hook", label: "Hook", color: "#5533aa", defaultRopeKind: "rope.link" } },
    diff: { gripKinds: { added: [{ id: "gk-hook", name: "hook", label: "Hook", color: "#5533aa", defaultRopeKind: "rope.link" }] } },
    assertion: `assert_eq!(snapshot.grip_kinds.last().map(|k| k.id.as_str()), Some("gk-hook"), "create-grip-kind must append the new kind last");` },
  { leaf: "❌delete-grip-kind", name: "removes-plug-grip-kind", summary: "the `gk-plug` grip-kind row is removed from the catalog",
    mutation: { mutation: "deleteGripKind", id: "gk-plug" }, diff: { gripKinds: { removed: ["gk-plug"] } },
    assertion: `assert_eq!(snapshot.grip_kinds.iter().map(|k| k.id.as_str()).collect::<Vec<_>>(), vec!["gk-socket"], "delete-grip-kind must remove only gk-plug, and must NOT cascade into the grips that reference it");` },
  { leaf: "🖋rename-grip-kind", name: "renames-plug-to-coupler", summary: "the `gk-plug` grip kind's `name` becomes `coupler`",
    mutation: { mutation: "renameGripKind", id: "gk-plug", newName: "coupler" }, diff: { gripKinds: gkPatch({ name: "coupler" }) },
    assertion: `assert_eq!(snapshot.grip_kinds[1].name, "coupler", "rename-grip-kind must rewrite the name in place, keeping the row's position");` },
  { leaf: "🎫change-grip-kind-label", name: "relabels-plug-grip-kind", summary: "the `gk-plug` grip kind's `label` becomes `Plug Rim`",
    mutation: { mutation: "changeGripKindLabel", id: "gk-plug", newLabel: "Plug Rim" }, diff: { gripKinds: gkPatch({ label: "Plug Rim" }) },
    assertion: `assert_eq!((snapshot.grip_kinds[1].label.as_str(), snapshot.grip_kinds[1].name.as_str()), ("Plug Rim", "plug"), "change-grip-kind-label must move the label and leave the name untouched");` },
  { leaf: "🎨change-grip-kind-color", name: "recolors-plug-grip-kind", summary: "the `gk-plug` grip kind's `color` becomes `#101010`",
    mutation: { mutation: "changeGripKindColor", id: "gk-plug", newColor: "#101010" }, diff: { gripKinds: gkPatch({ color: "#101010" }) },
    assertion: `assert_eq!(snapshot.grip_kinds[1].color, "#101010", "change-grip-kind-color must rewrite only that row's color");` },
  { leaf: "🪢change-grip-kind-default-rope-kind", name: "swaps-plug-default-rope-kind", summary: "the `gk-plug` grip kind's `defaultRopeKind` becomes `rope.heavy`",
    mutation: { mutation: "changeGripKindDefaultRopeKind", id: "gk-plug", newDefaultRopeKind: "rope.heavy" }, diff: { gripKinds: gkPatch({ defaultRopeKind: "rope.heavy" }) },
    assertion: `assert_eq!((snapshot.grip_kinds[1].default_rope_kind.as_str(), snapshot.grip_kinds[0].default_rope_kind.as_str()), ("rope.heavy", "rope.link"), "change-grip-kind-default-rope-kind must touch only the addressed row");` },
  //#endregion 🔖️GripKinds
  //#region 🔖️Grips
  { leaf: "🌿create-grip", name: "appends-south-grip", summary: "a second rim-grip template `grip-south` is appended",
    mutation: { mutation: "createGrip", grip: { id: "grip-south", gripKind: "gk-plug", angle: 3.0, radius2d: 0.3, position: [0.0, -1.0, 0.0], direction: [0.0, -1.0, 0.0], radius3d: 0.25 } },
    diff: { grips: { added: [{ id: "grip-south", gripKind: "gk-plug", angle: 3.0, radius2d: 0.3, position: [0.0, -1.0, 0.0], direction: [0.0, -1.0, 0.0], radius3d: 0.25 }] } },
    assertion: `assert_eq!(snapshot.grips.last().map(|g| g.id.as_str()), Some("grip-south"), "create-grip must append the new template last");` },
  { leaf: "🕳delete-grip", name: "removes-north-grip", summary: "the `grip-north` rim-grip template is removed",
    mutation: { mutation: "deleteGrip", id: "grip-north" }, diff: { grips: { removed: ["grip-north"] } },
    assertion: `assert!(snapshot.grips.is_empty() && snapshot.grip_kinds.len() == 2, "delete-grip must drop the template without touching the grip-kind catalog");` },
  { leaf: "📍move-grip-2d", name: "swings-north-grip-along-the-rim", summary: "the `grip-north` template's 2D half (`angle` + `radius2d`) is moved",
    mutation: { mutation: "moveGrip2d", id: "grip-north", newAngle: 1.5, newRadius2d: 0.45 }, diff: { grips: gripPatch({ angle: 1.5, radius2d: 0.45 }) },
    assertion: `assert_eq!((snapshot.grips[0].angle, snapshot.grips[0].radius_2d, snapshot.grips[0].position), (1.5, 0.45, [0.0, 1.0, 0.0]), "move-grip-2d must move only the 2D half, leaving the 3D position where it was");` },
  { leaf: "🧭move-grip-3d", name: "repositions-north-grip-in-world", summary: "the `grip-north` template's 3D half (`position` + `direction`) is moved",
    mutation: { mutation: "moveGrip3d", id: "grip-north", newPosition: [1.0, 2.0, 3.0], newDirection: [1.0, 0.0, 0.0] }, diff: { grips: gripPatch({ position: [1.0, 2.0, 3.0], direction: [1.0, 0.0, 0.0] }) },
    assertion: `assert_eq!((snapshot.grips[0].position, snapshot.grips[0].direction, snapshot.grips[0].angle), ([1.0, 2.0, 3.0], [1.0, 0.0, 0.0], 0.0), "move-grip-3d must move only the 3D half, leaving the 2D angle where it was");` },
  { leaf: "📏resize-grip-3d", name: "widens-north-grip-radius", summary: "the `grip-north` template's `radius3d` widens to 0.9",
    mutation: { mutation: "resizeGrip3d", id: "grip-north", newRadius3d: 0.9 }, diff: { grips: gripPatch({ radius3d: 0.9 }) },
    assertion: `assert_eq!((snapshot.grips[0].radius_3d, snapshot.grips[0].radius_2d), (0.9, 0.3), "resize-grip-3d must widen the 3D radius only, never the 2D one");` },
  { leaf: "🧷change-grip-grip-kind", name: "rekinds-north-grip-as-socket", summary: "the `grip-north` template is re-pointed at the `gk-socket` kind",
    mutation: { mutation: "changeGripGripKind", id: "grip-north", newGripKind: "gk-socket" }, diff: { grips: gripPatch({ gripKind: "gk-socket" }) },
    assertion: `assert_eq!((snapshot.grips[0].grip_kind.as_str(), snapshot.grip_kinds.len()), ("gk-socket", 2), "change-grip-grip-kind must re-point the template without adding or removing a catalog row");` },
  //#endregion 🔖️Grips
  //#region 🔖️CompatibilityAttributesAuthors
  { leaf: "➕add-compatibility-rule", name: "allows-plug-to-socket", summary: "a `compat-plug-socket` rule is appended to the compatibility table",
    mutation: { mutation: "addCompatibilityRule", rule: { id: "compat-plug-socket", source: "gk-plug", target: "gk-socket", bidirectional: false } },
    diff: { compatibility: { added: [{ id: "compat-plug-socket", source: "gk-plug", target: "gk-socket", bidirectional: false }] } },
    assertion: `assert_eq!(snapshot.compatibility.last().map(|r| (r.id.as_str(), r.bidirectional)), Some(("compat-plug-socket", false)), "add-compatibility-rule must append the one-way rule verbatim");` },
  { leaf: "✂remove-compatibility-rule", name: "revokes-plug-to-plug", summary: "the `compat-plug-plug` rule is removed from the compatibility table",
    mutation: { mutation: "removeCompatibilityRule", id: "compat-plug-plug" }, diff: { compatibility: { removed: ["compat-plug-plug"] } },
    assertion: `assert!(snapshot.compatibility.is_empty() && snapshot.grip_kinds.len() == 2, "remove-compatibility-rule must drop the row without disturbing the grip-kind catalog it names");` },
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
  //#region 🔖️CamerasAndMeta
  { leaf: "🎥move-camera2d", name: "pans-2d-camera", summary: "the 2D camera pans to (12, -4)",
    mutation: { mutation: "moveCamera2d", newX: 12.0, newY: -4.0 }, diff: { camera2d: { x: 12.0, y: -4.0, zoom: 1.0 } },
    assertion: `assert_eq!((snapshot.camera2d.x, snapshot.camera2d.y, snapshot.camera2d.zoom), (12.0, -4.0, 1.0), "move-camera2d must pan without changing zoom");` },
  { leaf: "🔍scale-camera2d", name: "zooms-2d-camera-in", summary: "the 2D camera zoom rises to 2.5",
    mutation: { mutation: "scaleCamera2d", newZoom: 2.5 }, diff: { camera2d: { x: 0.0, y: 0.0, zoom: 2.5 } },
    assertion: `assert_eq!((snapshot.camera2d.zoom, snapshot.camera2d.x), (2.5, 0.0), "scale-camera2d must change zoom without panning");` },
  { leaf: "🎬move-camera3d", name: "orbits-3d-camera", summary: "the 3D camera moves to (5, 5, 5) looking at (0, 1, 0)",
    mutation: { mutation: "moveCamera3d", newPosition: [5.0, 5.0, 5.0], newTarget: [0.0, 1.0, 0.0] }, diff: { camera3d: { position: [5.0, 5.0, 5.0], target: [0.0, 1.0, 0.0], zoom: 1.0 } },
    assertion: `assert_eq!((snapshot.camera3d.position, snapshot.camera3d.target, snapshot.camera3d.zoom), ([5.0, 5.0, 5.0], [0.0, 1.0, 0.0], 1.0), "move-camera3d must move position and target while keeping zoom");` },
  { leaf: "🔎scale-camera3d", name: "zooms-3d-camera-out", summary: "the 3D camera zoom falls to 0.5",
    mutation: { mutation: "scaleCamera3d", newZoom: 0.5 }, diff: { camera3d: { position: [0.0, 0.0, 0.0], target: [0.0, 0.0, 0.0], zoom: 0.5 } },
    assertion: `assert_eq!((snapshot.camera3d.zoom, snapshot.camera2d.zoom), (0.5, 1.0), "scale-camera3d must leave the 2D camera's zoom untouched");` },
  { leaf: "💬change-meta-description", name: "rewrites-session-notes", summary: "the document's session `meta.description` is rewritten",
    mutation: { mutation: "changeMetaDescription", newDescription: "Reviewed during the fixture pass." }, diff: { meta: { description: "Reviewed during the fixture pass." } },
    assertion: `assert_eq!((snapshot.meta.description.as_str(), snapshot.part_kind.description.as_str()), ("Reviewed during the fixture pass.", "One habitation capsule."), "change-meta-description must rewrite the session note, never the kind's own description");` },
  //#endregion 🔖️CamerasAndMeta
];

export function emitBlock5d(): { written: string[]; leaves: { leaf: string; name: string }[] } {
  const written: string[] = [];
  const leaves: { leaf: string; name: string }[] = [];
  for (const entry of CASES) {
    const diff = fullDiff(BLOCK5D_DIFF_FIELDS, new Set(["representations", "gripKinds", "grips", "compatibility", "attributes"]), new Set(["authors"]), entry.diff);
    const before = clone(BASE);
    const after = applyBlock5d(before, diff);
    const rust = renderRust({
      artifact: "block5d", snapshotType: "Block5dSnapshot", mutationType: "Block5dMutation", diffType: "Block5dDiff",
      applyFn: "apply_block5d_mutation", inverseFn: "inverse_block5d_mutation",
      leaf: entry.leaf.replace(/[^\x20-\x7e]/g, ""),
      caseName: entry.name, summary: entry.summary, beforePrelude: "", stateAssertion: entry.assertion,
    });
    written.push(...writeCase(join(ROOT, entry.leaf), entry.name, { before, after, mutation: entry.mutation, diff, outcome: { status: "applied" }, rust }, () => true));
    leaves.push({ leaf: entry.leaf, name: entry.name });
  }
  return { written, leaves };
}
