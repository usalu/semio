
from pathlib import Path
import re
import sys

root = Path("/Users/ueli/Documents/semio")
tick = Path(sys.argv[1])
fw = next(p for p in root.iterdir() if p.is_dir() and "framework" in p.name)
core = next(p for p in (fw / "🔨️modules").iterdir() if "core" in p.name)
comp_path = next(core.glob("*component.ts"))
comp = comp_path.read_text()

if "UiPresence as GeneratedUiPresence" not in comp:
    m = re.search(r'  DialogDefinition as GeneratedDialogDefinition,\n\} from "(\./[^"]+manifest\.ts)";', comp)
    if not m:
        raise SystemExit("manifest import close not found")
    path = m.group(1)
    comp = comp.replace(
        '  DialogDefinition as GeneratedDialogDefinition,\n} from "' + path + '";',
        '  DialogDefinition as GeneratedDialogDefinition,\n'
        "  UiPresence as GeneratedUiPresence,\n"
        "  UiState as GeneratedUiState,\n"
        "  UiStatus as GeneratedUiStatus,\n"
        '} from "' + path + '";',
        1,
    )
    anchor = "export type ActionDescriptor = GeneratedActionDescriptor;"
    if anchor not in comp:
        raise SystemExit("ActionDescriptor alias missing")
    comp = comp.replace(
        anchor,
        anchor
        + "\n\n"
        + "export type UiPresence = GeneratedUiPresence;\n"
        + "export type UiState = GeneratedUiState;\n"
        + "export type UiStatus = GeneratedUiStatus;",
        1,
    )
    print("added UiPresence type imports/aliases")
else:
    print("UiPresence types already present")


def add_presence_field(type_name: str, text: str) -> str:
    start = text.find(f"export type {type_name} = {{")
    if start < 0:
        raise SystemExit(f"{type_name} missing")
    end = text.find("\n};", start)
    block = text[start : end + 3]
    if "presence?" in block:
        return text
    new_block = block[:-3] + "  readonly presence?: UiPresence;\n};"
    return text[:start] + new_block + text[end + 3 :]


comp = add_presence_field("UiStackNode", comp)
comp = add_presence_field("UiTreeNode", comp)
print("presence fields ensured")

region = """
//#region 🧭️UiPresence
const DEFAULT_UI_PRESENCE: UiPresence = { state: "normal", status: "idle", hover: false, selected: false };

/** @emoji 🧭️ Resolves optional wire-format `presence` to the shared default inert model. */
export function resolveUiPresence(presence?: UiPresence): UiPresence {
  return presence ?? DEFAULT_UI_PRESENCE;
}

/** @emoji 🧭️ True when the element should show a skeleton instead of its content. */
export function uiPresenceShowsSkeleton(presence?: UiPresence): boolean {
  const status = resolveUiPresence(presence).status;
  return status === "loading" || status === "waiting";
}

/** @emoji 🧭️ Maps measure chrome booleans to the shared status axis until generated `WindowMeasure` gains `presence`. */
export function windowMeasureChromeStatus(measure: { readonly loading?: boolean; readonly waiting?: boolean }): UiStatus {
  if (measure.loading) return "loading";
  if (measure.waiting) return "waiting";
  return "idle";
}

/** @emoji 🧭️ Shared presence stamp for shell surfaces waiting on `refreshUi`. */
export const UI_PENDING_PRESENCE: UiPresence = { state: "normal", status: "loading", hover: false, selected: false };

/** @emoji 🦴 Declarative placeholder node while a window body is still loading. */
export function pendingWindowUiNode(): UiStackNode {
  return { type: "stack", direction: "column", children: [], presence: UI_PENDING_PRESENCE };
}

/** @emoji 🦴 Declarative placeholder node while a panel tab body is still loading. */
export function pendingPanelUiNode(): UiTreeNode {
  return { type: "tree", sections: [], presence: UI_PENDING_PRESENCE };
}
//#endregion 🧭️UiPresence
"""

if "export function resolveUiPresence" not in comp:
    alias_anchor = "export type UiStatus = GeneratedUiStatus;"
    if alias_anchor not in comp:
        raise SystemExit("UiStatus alias missing for insert")
    comp = comp.replace(alias_anchor, alias_anchor + "\n" + region, 1)
    print("inserted UiPresence helpers")
else:
    print("helpers already present")

comp_path.write_text(comp)
print("wrote core component")

ui_idx = next(
    p
    for p in fw.rglob("📦️index.tsx")
    if "🖱️ui" in str(p) and "🎯️targets" in str(p) and "⚛️react" in str(p)
)
ui = ui_idx.read_text()
# Ensure import includes windowElementId
import_chunk_match = re.search(
    r'panelTabFirstDraggableElementId,\n',
    ui,
)
if "windowElementId," not in ui:
    if "panelTabFirstDraggableElementId," not in ui:
        raise SystemExit("panelTabFirstDraggableElementId import not found")
    ui = ui.replace(
        "panelTabFirstDraggableElementId,",
        "panelTabFirstDraggableElementId,\n  panelTabElementId,\n  windowElementId,",
        1,
    )
    print("added windowElementId to ui-react import")

export_line = 'export { windowElementId, panelTabElementId, panelTabFirstDraggableElementId } from "@semio-tech/framework-core";'
if export_line not in ui:
    reexport = 'export { CANVAS_HOVER_SOURCE_CANVAS, CANVAS_HOVER_SOURCE_PICK_MENU, canvasHoverFocusFromTarget, canvasPickTargetKey, pickMostSpecificCanvasTarget, sortCanvasPickTargetsGeneralFirst } from "@semio-tech/framework-core";'
    if reexport in ui:
        ui = ui.replace(reexport, reexport + "\n" + export_line, 1)
    else:
        ui += "\n// #region 🆔️ElementIds\n" + export_line + "\n// #endregion 🆔️ElementIds\n"
    print("added ui-react element id export")
else:
    print("ui-react element id export present")

ui_idx.write_text(ui)

helper = next(p for p in fw.rglob("*component.tsx") if "ShellHelpers" in str(p) and "elements" in str(p))
ht = helper.read_text()
if "export async function loadPluginModuleResilient" not in ht:
    if "async function loadPluginModuleResilient" not in ht:
        raise SystemExit("loadPluginModuleResilient not found")
    ht = ht.replace("async function loadPluginModuleResilient", "export async function loadPluginModuleResilient", 1)
    helper.write_text(ht)
    print("exported loadPluginModuleResilient")
else:
    print("loadPluginModuleResilient already exported")

(tick / "🧪export-fixes-summary.txt").write_text("ok\n")
print("DONE")
