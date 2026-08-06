import { readdirSync, statSync, readFileSync, writeFileSync, existsSync } from "fs";
import { join } from "path";
const repo = "/Users/ueli/Documents/semio";
function findChild(parent, substr) {
  for (const name of readdirSync(parent)) {
    const p = join(parent, name);
    try { if (statSync(p).isDirectory() && name.includes(substr)) return p; } catch {}
  }
  return null;
}
const fw = findChild(repo, "framework");
const ui = findChild(findChild(fw, "modules"), "ui");
const el = findChild(ui, "elements");
const ticket = "/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️05/UI-ELEMENT-CO-LOCATION-RESTRUCTURE";
const yarn = String.fromCodePoint(0x1f9f6) + String.fromCodePoint(0xfe0f);
const outPath = join(ticket, `${yarn}w6-core-icons-button-context.txt`);

const av = join(el, "Avatar", readdirSync(join(el, "Avatar")).find(n => n.endsWith(".tsx") && n.includes("component")));
const pn = join(el, "Panel", readdirSync(join(el, "Panel")).find(n => n.endsWith(".tsx") && n.includes("component")));
const avT = readFileSync(av, "utf8");
const pnT = readFileSync(pn, "utf8");

const append = `
## Precise definition lines (enrichment — inventory only)

| Symbol | Line | Status |
|---|---|---|
| Icon (type) | L1189 | STILL_IN_BARREL (near IconRenderPort / Icon region ~L1168) |
| IconSource | L1444 | STILL_IN_BARREL |
| ControlIcon | L1447 | STILL_IN_BARREL |
| renderControlIcon | L1554 | STILL_IN_BARREL |
| Icon (component) | L1605 | STILL_IN_BARREL |
| CheckIcon / Chevron* / CloseIcon / Maximize2Icon | L1684+ | STILL_IN_BARREL (createIconComponent cluster) |
| ContextMenuItem | L1881 | STILL_IN_BARREL inside #region ContextMenu L1788-~2764 |
| ContextMenu | L1973 | STILL_IN_BARREL inside #region ContextMenu |
| DragHandle | L8801 | STILL_IN_BARREL (near DnD; NOT in Avatar leaf) |
| PanelTabNode | L9026 | STILL_IN_BARREL |
| PanelTabBar | L9497 | STILL_IN_BARREL |
| Label (component) | L11280 | STILL_IN_BARREL (distinct from core/UiLabel) |
| ButtonGroup | L11921 | STILL_IN_BARREL (no named #region; clustered with Button) |
| ButtonGroupItem | L11953 | STILL_IN_BARREL |
| Button | L12038 | STILL_IN_BARREL (function Button; export cluster L12068) |

### Region stubs already extracted
- #region Icons L11826-11829: only import { Cursor } from elements/Icons + export — Icon cluster NOT moved yet
- #region Avatar L11537-11540: already import-then-exports Avatar/DraggableAvatar/TableAvatar from Avatar leaf
- #region UiLabel L3828-3834: core/UiLabel only (NOT the Label component)

### Leaf probes
- Avatar contains DragHandle text: ${/DragHandle/.test(avT)}; exports DraggableAvatar: ${/export.*DraggableAvatar/.test(avT)}
- Panel contains PanelTabBar: ${/PanelTabBar/.test(pnT)}; PanelTabNode: ${/PanelTabNode/.test(pnT)}

### Co-location notes for when lock frees
- Button/: needs NEW component.tsx (rs-only today). Button+ButtonCycle co-extract; ButtonGroup/Item adjacent (~L11921-12068) -> ButtonGroup/ element dir per task.
- Icons/: EXTEND existing component.tsx beyond Cursor with Icon/ControlIcon/renderControlIcon/IconSource/IconName/individual icons from ~L1189-1735+. Prefer Icons/ not core/Icons unless top-level TDZ cycle.
- ContextMenu/: CREATE; extract full #region ContextMenu (~977 lines).
- DragHandle: NOT in Avatar (Avatar already has DraggableAvatar). CREATE DragHandle/ element (preferred over stuffing Avatar).
- PanelTabBar/PanelTabNode: NOT in Panel leaf. Own PanelTabBar/ element if interim needs them.
- Label/: MISSING; Label component still barrel L11280; extract only if in scope.
- Stories still at .storybook/stories/ui/: Button, ButtonGroup, ContextMenu, Label, PanelTabBar — move after extract homes exist; do NOT change meta.title.

### W3-interim clearance blockers (still barrel-defined)
Button, Icon, ControlIcon, renderControlIcon, IconSource, CheckIcon/Chevrons/Close/Maximize*, ContextMenu, ContextMenuItem, DragHandle, ButtonGroup, ButtonGroupItem, PanelTabBar, PanelTabNode, Label — all STILL_IN_BARREL.

STOP. Awaiting parent barrel lock. No edits to ui-react barrel.
`;

const existing = existsSync(outPath) ? readFileSync(outPath, "utf8") : "";
const body = existing.includes("## Precise definition lines")
  ? existing.replace(/## Precise definition lines[\s\S]*$/, append.trim() + "\n")
  : existing.trimEnd() + "\n" + append;
writeFileSync(outPath, body);
console.log("UPDATED", outPath);
console.log("Avatar DragHandle?", /DragHandle/.test(avT), "DraggableAvatar?", /DraggableAvatar/.test(avT));
console.log("Panel PanelTabBar?", /PanelTabBar/.test(pnT), "PanelTabNode?", /PanelTabNode/.test(pnT));
