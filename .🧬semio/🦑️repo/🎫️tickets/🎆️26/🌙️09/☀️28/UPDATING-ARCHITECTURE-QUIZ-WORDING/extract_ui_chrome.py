"""🪟️ Moves the window chrome and the surface-chrome appearance controller out of the ui-react barrel.

One-shot refactor for the quiz card grid (2026-09-29): the site imports a slim `@semio-tech/ui-react/chrome` subpath
instead of the 9,700-line barrel. Every block is located by exact, unique marker lines; the script refuses to write
if a marker is missing or ambiguous, so a concurrent edit of the barrel stops it instead of corrupting it.

Moves:
- `createDOMEventBinding` (ContextMenu element) -> `🔨️modules/👂️dom-event-binding/🟦️.ts`
- `applyUiFormControlBrowserDefaults` (barrel) -> `🔨️modules/📝️form-control-presentation/🟦️.ts`
- `uiSpacingLen` (Tree element) -> `🎨️styling/🌓️theme/🟦️.ts`
- silhouette border kinds, geometry hook, chrome classes, `WindowChrome` (barrel) -> `🧱️elements/🗂️WindowChrome/🟦️.tsx`
- surface chrome, chrome reveal, appearance/layout prefs, `useMediaQuery`, browser-default suppression (barrel)
  -> `🎯️targets/⚛️react/🌓️appearance/🟦️.ts`
The barrel, the ContextMenu and the Tree element import and re-export every moved name unchanged.
"""

import io
import os
import sys

UI = r"C:\git\semio\🧰️framework\🔨️modules\🖱️ui"
BARREL = os.path.join(UI, "🎯️targets", "⚛️react", "🟦️.tsx")
CONTEXT_MENU = os.path.join(UI, "🧱️elements", "🖱️ContextMenu", "🟦️.tsx")
TREE = os.path.join(UI, "🧱️elements", "🌳️Tree", "🟦️.tsx")
THEME = os.path.join(UI, "🎨️styling", "🌓️theme", "🟦️.ts")
FORM = os.path.join(UI, "🔨️modules", "📝️form-control-presentation", "🟦️.ts")
DOM_EVENTS = os.path.join(UI, "🔨️modules", "👂️dom-event-binding", "🟦️.ts")
CHROME = os.path.join(UI, "🧱️elements", "🗂️WindowChrome", "🟦️.tsx")
APPEARANCE = os.path.join(UI, "🎯️targets", "⚛️react", "🌓️appearance", "🟦️.ts")


def read(path):
    with io.open(path, encoding="utf-8", newline="") as handle:
        return handle.read()


def write(path, text):
    os.makedirs(os.path.dirname(path), exist_ok=True)
    with io.open(path, "w", encoding="utf-8", newline="") as handle:
        handle.write(text)


class Lines:
    def __init__(self, text):
        assert "\r" not in text, "expected LF line endings"
        self.lines = text.split("\n")

    def index(self, marker, prefix=False, after=0):
        hits = [i for i, line in enumerate(self.lines) if i >= after and (line.startswith(marker) if prefix else line == marker)]
        if len(hits) != 1:
            sys.exit(f"marker {marker!r} found {len(hits)} times")
        return hits[0]

    def cut(self, start, end_inclusive):
        block = self.lines[start : end_inclusive + 1]
        del self.lines[start : end_inclusive + 1]
        return block

    def insert(self, at, new_lines):
        self.lines[at:at] = new_lines

    def text(self):
        return "\n".join(self.lines)


def closing_brace(lines, start):
    for i in range(start, len(lines.lines)):
        if lines.lines[i] == "}":
            return i
    sys.exit(f"no closing brace after line {start}")


STAGED = {}


def stage(path, text):
    STAGED[path] = text


def trim_blank(block):
    while block and block[0].strip() == "":
        block = block[1:]
    while block and block[-1].strip() == "":
        block = block[:-1]
    return block


for path in (DOM_EVENTS, CHROME, APPEARANCE):
    if os.path.exists(path):
        sys.exit(f"{path} already exists")

barrel = Lines(read(BARREL))
context_menu = Lines(read(CONTEXT_MENU))
tree = Lines(read(TREE))
theme_text = read(THEME)
form_text = read(FORM)

# 👂️ createDOMEventBinding
start = context_menu.index("type DOMListenerTarget = Pick<EventTarget, \"addEventListener\" | \"removeEventListener\">;")
end = closing_brace(context_menu, context_menu.index("export function createDOMEventBinding() {"))
dom_block = context_menu.cut(start, end)
context_menu.insert(start, ['export { createDOMEventBinding, type DOMListenerTarget } from "../../🔨️modules/👂️dom-event-binding/🟦️.ts";'])
adapter_end = context_menu.index("// #endregion 🔌️Adapters")
context_menu.insert(adapter_end, ['import { createDOMEventBinding } from "../../🔨️modules/👂️dom-event-binding/🟦️.ts";'])
dom_block = [line.replace("type DOMListenerTarget", "export type DOMListenerTarget") if line.startswith("type DOMListenerTarget") else line for line in dom_block]
stage(
    DOM_EVENTS,
    "\n".join(
        [
            "/** 👂️ One owner for a set of DOM listeners: `listen` registers a listener and remembers how to remove it, `dispose`",
            " * removes every one in reverse order — so a controller installs many listeners and tears them down in one call.",
            " *",
            " * @see https://developer.mozilla.org/en-US/docs/Web/API/EventTarget/addEventListener",
            " */",
            "",
            "/** 🎯️ Anything that takes and removes DOM event listeners (elements, documents, windows, media query lists). */",
        ]
        + [line for line in dom_block if not line.startswith("export type DOMListenerTarget")]
        + [""]
    ).replace(
        "/** 🎯️ Anything that takes and removes DOM event listeners (elements, documents, windows, media query lists). */\n",
        "/** 🎯️ Anything that takes and removes DOM event listeners (elements, documents, windows, media query lists). */\nexport type DOMListenerTarget = Pick<EventTarget, \"addEventListener\" | \"removeEventListener\">;\n",
        1,
    ).replace("export function createDOMEventBinding() {", "/** 🎧️ A fresh listener owner; see the module docs. */\nexport function createDOMEventBinding() {", 1),
)

# 🚫️ applyUiFormControlBrowserDefaults
start = barrel.index("/** @emoji 🚫️ Applies {@link uiFormControlBrowserDefaultProps} to a live form control (idempotent). */")
end = closing_brace(barrel, start)
form_block = barrel.cut(start, end)
form_import = barrel.index('import { formControlFocusBorderClass, uiFormControlBrowserDefaultProps } from "../../🔨️modules/📝️form-control-presentation/🟦️.ts";')
barrel.lines[form_import] = 'import { applyUiFormControlBrowserDefaults, formControlFocusBorderClass, uiFormControlBrowserDefaultProps } from "../../🔨️modules/📝️form-control-presentation/🟦️.ts";'
barrel.lines[form_import + 1] = "export { applyUiFormControlBrowserDefaults, formControlFocusBorderClass, uiFormControlBrowserDefaultProps };"
form_text = form_text.rstrip("\n") + "\n\n" + "\n".join(form_block) + "\n"

# 📐️ uiSpacingLen
spacing = tree.index("export const uiSpacingLen = (multiplier: number): string => `calc(${multiplier} * var(--ui-spacing))`;")
del tree.lines[spacing]
styling_import = tree.index('import { STYLING_DOM, STYLING_METRICS, STYLING_COMPACT_ROOT_PX, domSizePx, sizeVar, uiSpacingPx } from "@semio-tech/ui-styling";')
tree.lines[styling_import] = 'import { STYLING_DOM, STYLING_METRICS, STYLING_COMPACT_ROOT_PX, domSizePx, sizeVar, uiSpacingLen, uiSpacingPx } from "@semio-tech/ui-styling";'
tree.insert(styling_import + 1, ["export { uiSpacingLen };"])
anchor = "/** 📐️ Converts a ui-spacing multiplier to px at the compact reference root. */"
assert theme_text.count(anchor) == 1
theme_text = theme_text.replace(
    anchor,
    "/** 📏️ A ui-spacing multiplier as a CSS length that follows the live `--ui-spacing` (compact or touch). */\n"
    "export function uiSpacingLen(multiplier: number): string {\n"
    "  return `calc(${multiplier} * var(--ui-spacing))`;\n"
    "}\n\n" + anchor,
    1,
)

# 🗂️ WindowChrome
w1_start = barrel.index("/** @emoji 🪟️ All border effects the silhouette SVG can paint. */")
w1_end = barrel.index("/** @emoji 📏️ Body fill only", prefix=True, after=w1_start) - 1
w1 = barrel.cut(w1_start, w1_end)
w2_start = barrel.index("/** @emoji 📏️ Maximize/controls glass cell — host stamps {@link glassClass}; fill must not span the U-gap. */")
w2_end = barrel.index("export const WINDOW_CHROME_BODY_PLANE_STYLE: React.CSSProperties = { zIndex: 1 };")
w2 = barrel.cut(w2_start, w2_end)
region = barrel.index("//#region 🪟️WindowChrome")
w3_start = barrel.index("/** @emoji 🪟️ Optional right-cap control on {@link WindowChrome} (enlarge / close). */")
w3_end = barrel.index('WindowChrome.displayName = "WindowChrome";')
assert w3_start == region + 2, "WindowChrome region layout changed"
w3 = barrel.cut(w3_start, w3_end)
chrome_names = [
    "WINDOW_CHROME_BODY_PLANE_STYLE",
    "WINDOW_CHROME_CHIP_ROW_STYLE",
    "WINDOW_SILHOUETTE_BORDER_KINDS",
    "WindowChrome",
    "WindowChromeSilhouetteBorder",
    "isWindowChromeIntroducedTarget",
    "measureWindowSilhouetteMetrics",
    "resolveWindowSilhouetteBorderKind",
    "useWindowSilhouetteGeometry",
    "windowCapFrameClass",
    "windowChromeTitleChipClass",
    "windowControlsCapClass",
    "windowGapFrameClass",
    "windowSilhouetteBorderPaint",
    "type WindowChromeControlAction",
    "type WindowChromeProps",
    "type WindowSilhouetteBorderKind",
]
barrel.insert(region + 1, [f'import {{ {", ".join(chrome_names)} }} from "../../🧱️elements/🗂️WindowChrome/🟦️.tsx";', f'export {{ {", ".join(chrome_names)} }};'])
stage(
    CHROME,
    "\n".join(
        [
            "// #region 🧲️Header",
            "// 💻️ framework/ui/elements/🗂️WindowChrome/component.tsx",
            "// 2026 Ueli Saluz <ueli@semio-tech.com>",
            "// 2026 Kinan Sarakbi <kinan.sarak@gmail.com>",
            "// This program is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version. This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the GNU Lesser General Public License for more details. You should have received a copy of the GNU Lesser General Public License along with this program.  If not, see <https://www.gnu.org/licenses/>.",
            "// #endregion 🧲️Header",
            "",
            "// #region 🔌️Adapters",
            'import * as React from "react";',
            'import { reactHostPort } from "../🔌️Ports/🟦️.tsx";',
            'import { SurfaceScope, type Level, type SurfaceActiveBindProps } from "../🌈️Surface/🟦️.tsx";',
            'import { WINDOW_SILHOUETTE_CHIP_EPSILON, createWindowSilhouetteGeometry, normalizeWindowSilhouetteChips, type WindowSilhouetteChip, type WindowSilhouetteEdge, type WindowSilhouetteGeometry, type WindowSilhouetteMetrics } from "../🔲️WindowSilhouette/🟦️.tsx";',
            'import { cn } from "../../🔨️modules/🏷️class-name-composition/🟦️.ts";',
            'import { modeDockTabClassName } from "../../🔨️modules/🎛️chrome-control-presentation/🟦️.ts";',
            'import { interactiveHoverClass } from "../../🔨️modules/🖱️interaction-presentation/🟦️.ts";',
            'import { glassClass } from "../../🔨️modules/🌈️surface-presentation/🟦️.ts";',
            "// #endregion 🔌️Adapters",
            "",
            "//#region 🗂️WindowSilhouetteBorder",
        ]
        + trim_blank(w1)
        + ["", ""]
        + trim_blank(w2)
        + ["//#endregion 🗂️WindowSilhouetteBorder", "", "//#region 🗂️WindowChrome"]
        + trim_blank(w3)
        + ["//#endregion 🗂️WindowChrome", ""]
    ),
)

# 🌓️ appearance
a1_start = barrel.index('export type ElementsSurfaceAppearance = "system" | "light" | "dark";') - 1
a1_end = a1_start + 1
a1 = barrel.cut(a1_start, a1_end)
a2_start = barrel.index("export interface ElementsSurfaceChromeInput {")
a2_end = closing_brace(barrel, barrel.index("export function resetElementsSurfaceChromeForTests(): void {"))
a2 = barrel.cut(a2_start, a2_end)
a3_start = barrel.index("/** @emoji 🌓️ Storage key for surface appearance (system/light/dark). */")
a3_end = closing_brace(barrel, barrel.index("export function writeStoredUiChromeLayout(storage: StoragePort, layout: UiChromeLayout): void {"))
a3 = barrel.cut(a3_start, a3_end)
a4_start = barrel.index(" * Hook returning whether a CSS media query currently matches.") - 1
assert barrel.lines[a4_start] == "/**"
a4_end = closing_brace(barrel, barrel.index("export function useMediaQuery(query: string, defaultValue = false): boolean {"))
a4 = barrel.cut(a4_start, a4_end)
a5_start = barrel.index("/** @emoji 🚫️ Capture-phase listeners: native context menu off everywhere; form-control browser defaults on focus.")
a5_end = closing_brace(barrel, barrel.index("export function installElementsSurfaceBrowserDefaultSuppression(bindings: ReturnType<typeof createDOMEventBinding>): void {"))
a5 = barrel.cut(a5_start, a5_end)
appearance_names = [
    "UI_CHROME_APPEARANCE_STORAGE_KEY",
    "UI_CHROME_LAYOUT_STORAGE_KEY",
    "applyChromeRevealAtPoint",
    "applyElementsSurfaceChrome",
    "bootstrapElementsSurfaceChromeDocument",
    "installElementsSurfaceBrowserDefaultSuppression",
    "isElementsSurfaceChromeDarkApplied",
    "readStoredUiChromeAppearance",
    "readStoredUiChromeLayout",
    "resetElementsSurfaceChromeForTests",
    "resolveElementsSurfaceChromeDark",
    "resolveElementsSurfaceChromeRoot",
    "useCanvasAppearanceSync",
    "useElementsSurfaceChrome",
    "useMediaQuery",
    "writeStoredUiChromeAppearance",
    "writeStoredUiChromeLayout",
    "type ElementsSurfaceAppearance",
    "type ElementsSurfaceBrowserDefaults",
    "type ElementsSurfaceChromeInput",
    "type UiChromeLayout",
]
surface_region = barrel.index("// #region 🌈️SurfaceChrome")
barrel.insert(surface_region + 1, [f'import {{ {", ".join(appearance_names)} }} from "./🌓️appearance/🟦️.ts";', f'export {{ {", ".join(appearance_names)} }};'])
a2_text = "\n".join(a2)
a2_text = a2_text.replace("function resolveElementsSurfaceChromeRoot(root?: HTMLElement): HTMLElement | undefined {", "export function resolveElementsSurfaceChromeRoot(root?: HTMLElement): HTMLElement | undefined {", 1)
stage(
    APPEARANCE,
    "\n".join(
        [
            "/** 🌓️ The document-level surface chrome of every semio page on its own: appearance (system, light, dark) as `.dark`",
            " * and base colors, the device as `data-ui-device` and `.touch`, the driver's DOM axes with the hover-reveal controller,",
            " * the persisted appearance and layout preferences, `useMediaQuery`, and the app-shell suppression of browser defaults —",
            " * importable without the rest of the React target (`@semio-tech/ui-react/chrome`); the barrel re-exports it unchanged.",
            " *",
            " * @see ../../../📱️device/🟦️.ts — the device vocabulary and breakpoints",
            " * @see ../../../🧱️elements/🚗️UiDriver/🟦️.tsx — the driver axes",
            " * @see ../🟦️.tsx — the barrel that re-exports this module",
            " */",
            "",
            'import { createBrowserStoragePort, ephemeralBox, ephemeralMap, type StoragePort } from "@semio-tech/framework";',
            'import { clearStylingAppearanceRoot, setStylingAppearanceRoot, stylingAppearanceRootElement, subscribeStylingAppearanceRoot } from "@semio-tech/ui-styling";',
            'import { availableViewportHeightPx, type ElementsSurfaceDevice } from "../../../📱️device/🟦️.ts";',
            'import { reactHostPort } from "../../../🧱️elements/🔌️Ports/🟦️.tsx";',
            'import { readStoredUiDriver, setUiDriverProvider, type UiDriver, type UiDriverReveal } from "../../../🧱️elements/🚗️UiDriver/🟦️.tsx";',
            'import { createDOMEventBinding } from "../../../🔨️modules/👂️dom-event-binding/🟦️.ts";',
            'import { applyUiFormControlBrowserDefaults } from "../../../🔨️modules/📝️form-control-presentation/🟦️.ts";',
            "",
            "// #region 🌈️SurfaceChrome",
        ]
        + trim_blank(a1)
        + [""]
        + trim_blank(a2_text.split("\n"))
        + ["// #endregion 🌈️SurfaceChrome", "", "// #region 🎛️UiChromePrefs"]
        + trim_blank(a3)
        + ["// #endregion 🎛️UiChromePrefs", "", "// #region 📱️MediaQuery"]
        + trim_blank(a4)
        + ["// #endregion 📱️MediaQuery", "", "// #region 🚫️BrowserDefaults"]
        + trim_blank(a5)
        + ["// #endregion 🚫️BrowserDefaults", ""]
    ),
)

for path, text in STAGED.items():
    write(path, text)
write(BARREL, barrel.text())
write(CONTEXT_MENU, context_menu.text())
write(TREE, tree.text())
write(THEME, theme_text)
write(FORM, form_text)
print("moved", len(w1) + len(w2) + len(w3), "window chrome lines and", len(a1) + len(a2) + len(a3) + len(a4) + len(a5), "appearance lines")
