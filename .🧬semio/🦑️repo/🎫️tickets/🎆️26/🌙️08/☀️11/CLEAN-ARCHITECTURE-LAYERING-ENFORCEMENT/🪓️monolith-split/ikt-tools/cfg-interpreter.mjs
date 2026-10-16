const E = "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🗣️Interpreter/🟦️.tsx";
const R = (lo, hi) => (i) => i >= lo && i <= hi;
const set = (...xs) => (i) => xs.includes(i);
export const table = [
  ["presence", R(0, 6)],
  ["panelTree", R(7, 15)],
  ["context", (i) => i === 16 || R(34, 36)(i)],
  ["bridge", R(17, 27)],
  ["lanes", R(28, 33)],
  ["vfs", R(37, 38)],
  ["layout", R(39, 48)],
  ["actions", R(49, 52)],
  ["declarative", R(53, 61)],
  ["display", set(62, 64, 65, 72, 133, 139, 140, 141, 142)],
  ["input", (i) => R(66, 71)(i) || R(73, 76)(i)],
  ["treeWalk", (i) => R(77, 79)(i) || i === 122 || i === 123 || i === 132],
  ["domIds", R(80, 83)],
  ["treeWindows", R(85, 121)],
  ["rowActions", R(125, 130)],
  ["surfaceViews", R(134, 138)],
  ["tableWindow", R(143, 148)],
  ["nodeTree", set(63, 84, 124, 131, 149, 150, 151)],
];
export default {
  file: E,
  parentName: "Interpreter",
  keepKey: "parent",
  noBarrel: true,
  assign: (u) => {
    for (const [k, f] of table) if (f(u.idx)) return k;
    return u.idx === 152 || u.idx === 153 || u.idx === 154 ? "parent" : null;
  },
  parentHeader: (res) => {
    const head = res.a.text.slice(res.a.text.indexOf("// #region 🧲️Header"), res.a.text.indexOf("// #endregion 🧲️Header") + "// #endregion 🧲️Header".length);
    const tail = " * doc). Also owns the `ComponentSceneHost` registry (lazily mounts `canvas-2d`/`world-3d`/etc. surface\n * hosts behind `Component::Surface`) and the shared per-surface context-menu flow — both unchanged\n * from the pre-migration renderer, since neither depends on the old `UiNode` shape. */";
    if (head.split(tail).length !== 2) throw new Error("header anchor");
    const kids = res.modules.map((m) => `${m.emoji}${m.dir.slice(m.emoji.length)}`);
    const next = " * doc). The scene hosts behind `Component::Surface` are resolved through the element-host registry\n * (`🗺️element-host-registry`); the concrete hosts are registered by the composition root `🪢️ElementHosts` and are\n * never imported here. This entry module owns `InterpretedUiNode` and `interpretUiNode`; everything else lives in\n * its children, which importers import directly: " + kids.map((k) => "`" + k + "`").join(", ") + ". */";
    return "/// <reference types=\"vitest/importMeta\" />\n" + head.replace(tail, next);
  },
  modules: [
    { key: "presence", dir: "👥️presence-overlay", emoji: "👥️", doc: "This session's own hover, selection and preview marks on a node, keyed by node key, plus the host row-action refusals and the live guest presence merged into the overlay a rendered tree reads." },
    { key: "panelTree", dir: "🌲️tree-panel-boundary", emoji: "🌲️", doc: "The still-supported manifest tree payload mapped onto the owned panel-tree contract: panel tree items, sections, catalogue drag MIME and the declarative drag controller." },
    { key: "context", dir: "🧷️interpreter-context", emoji: "🧷️", doc: "The context every retained view reads: the interpreter context type, the plugin surface actions context and the label resolver." },
    { key: "bridge", dir: "🔌️surface-bridge", emoji: "🔌️", doc: "The bridge from a retained surface node to the scene host registered for its kind: kind to scene field and lane tables, host identity, the scene node assembly, the gesture funnel and the registry-resolved host render." },
    { key: "lanes", dir: "🚚️surface-scene-lanes", emoji: "🚚️", doc: "The last complete text of each surface scene-lane carrier, keyed by the producer's content hash, with its bounded cache and lane text assembly." },
    { key: "vfs", dir: "🗄️virtual-file-system", emoji: "🗄️", doc: "The virtual-file-system surface host: its navigation action and the host that renders the file tree." },
    { key: "layout", dir: "📐️layout-accessibility", emoji: "📐️", doc: "The pure presentation props of a node: layout spec to CSS, style spec data attributes and the accessibility ARIA props." },
    { key: "actions", dir: "⚡️action-dispatch", emoji: "⚡️", doc: "How a retained view dispatches a trigger: the control icon node, the trigger dispatch, the continuous trigger lane and the wire value conversion." },
    { key: "declarative", dir: "🎛️declarative-controls", emoji: "🎛️", doc: "The declarative control boundary: control types, their disabled and activity rules, action dispatch and the slider and stepper controls." },
    { key: "display", dir: "🔤️display-views", emoji: "🔤️", doc: "The display-only retained views: text, button, key-value list, image, progress, extension and unknown component, the busy shell and the activity border." },
    { key: "input", dir: "✍️input-views", emoji: "✍️", doc: "The editable retained views: input with its commit draft and picker release, select, toggle, slider, number stepper, ring and icon select." },
    { key: "treeWalk", dir: "🌳️tree-walk", emoji: "🌳️", doc: "The retained tree walk: tree item collection, row tone classes, control stand-in rule and the status section." },
    { key: "domIds", dir: "🪪️stable-dom-ids", emoji: "🪪️", doc: "The stable DOM ids and React reconciliation keys every interpreted node gets from its authored key." },
    { key: "treeWindows", dir: "🪟️tree-windows", emoji: "🪟️", doc: "Windowed containers: the window context and reporter, row and viewport measurement, request math, the served-memory rule, the window observer and the tree pick and open-state rules." },
    { key: "rowActions", dir: "🧾️row-actions", emoji: "🧾️", doc: "Row actions: the tree and table row action builders, the table row action button and the admission controller." },
    { key: "surfaceViews", dir: "🖼️surface-views", emoji: "🖼️", doc: "The retained surface views: carrier epoch, paged surface, accessibility shell and the surface view." },
    { key: "tableWindow", dir: "📊️table-window", emoji: "📊️", doc: "The windowed table rules: next row and column, scroll offsets, viewport row cap and the column template." },
    { key: "nodeTree", dir: "🧱️node-tree", emoji: "🧱️", doc: "Node rendering and component dispatch: the per-node subscribing view, the component switch and the views that recurse into child nodes (container, tree, table)." },
  ],
};
