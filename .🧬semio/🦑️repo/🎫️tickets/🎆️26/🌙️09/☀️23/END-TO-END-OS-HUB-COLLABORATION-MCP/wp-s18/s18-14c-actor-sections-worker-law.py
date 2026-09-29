"""🧩️ S18 §14c (C13 P1): the worker harness law expects the reserved section surfaces beside the windows and panels — all
four on the lifetime's first paint, the three live ones on every later repaint and panel refresh (`🪟️visible-surfaces`).
usage: python3 s18-14c-actor-sections-worker-law.py [--dry-run]"""
import sys
from pathlib import Path

LAW = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🧪️tests/🧪️space-artifact-creation-owner/🟦️.ts")
text = LAW.read_text(encoding="utf-8")


def swap(old: str, new: str) -> None:
    global text
    if text.count(old) != 1:
        raise SystemExit(f"anchor count {text.count(old)}: {old[:90]!r}")
    text = text.replace(old, new)


swap('''      const { panelViewContext, windowViewContext } = await import("../../../../🔨️modules/🛂️manifest/🟦️.ts");
''', '''      const { panelViewContext, sectionViewContext, windowViewContext } = await import("../../../../🔨️modules/🛂️manifest/🟦️.ts");
      const { BROWSER_ACTOR_VISIBLE_SURFACES_V1 } = await import("../../🔨️modules/🏪️store/👷️worker/🪟️visible-surfaces/🟦️.ts");
      const sectionVisible = (view: ResolvedPluginViewState, mount: boolean) => BROWSER_ACTOR_VISIBLE_SURFACES_V1.sections.filter((section) => mount || !section.static).map((section) => ({ tag: "surface-visible", val: { surface: { instance: 0, surface: section.bodyKey }, bodyKey: section.bodyKey, viewState: encodePackValue(sectionViewContext(view)) } }));
''')
swap("              expect(events).toEqual(panelVisible(state.browserActorViewState!));\n",
     "              expect(events).toEqual([...panelVisible(state.browserActorViewState!), ...sectionVisible(state.browserActorViewState!, false)]);\n")
swap("              expect(events.slice(1)).toEqual([secondWindowVisible(state.browserActorViewState!), ...panelVisible(state.browserActorViewState!)]);\n",
     "              expect(events.slice(1)).toEqual([secondWindowVisible(state.browserActorViewState!), ...panelVisible(state.browserActorViewState!), ...sectionVisible(state.browserActorViewState!, visibleViews.length === 1)]);\n")
swap("secondWindowVisible(state.browserActorViewState!), ...panelVisible(state.browserActorViewState!)] : [{ tag: \"wake\" }]);",
     "secondWindowVisible(state.browserActorViewState!), ...panelVisible(state.browserActorViewState!), ...sectionVisible(state.browserActorViewState!, true)] : [{ tag: \"wake\" }]);")
if "--dry-run" in sys.argv:
    print("dry ok")
else:
    LAW.write_text(text, encoding="utf-8")
    print("ok", LAW)
