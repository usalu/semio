"""🧩️ S18 §14c (C13 P1): the browser-actor patch corpus and its law cover the reserved refresh-section surfaces — every step
states `sectionsChanged` and the section values the shell decodes; five section steps (en + de values, packed attributes,
stale reset, a section alone never opens a document).
usage: python3 s18-14c-actor-sections-corpus.py [--dry-run]"""
import json
import sys
from pathlib import Path

HELPERS = Path("/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers")
CORPUS = HELPERS / "🧫️fixtures/🎭️browser-actor-panels/🔣️.json"
LAW = HELPERS / "🧪️tests/🎭️browser-actor-panels/🟦️.ts"
DRY = "--dry-run" in sys.argv

STYLE = {"variant": "plain", "size": "md", "density": "standard", "tone": "neutral", "emphasis": "regular"}
ACCESSIBILITY = {"label": None, "description": None, "live": "off", "shortcut": None, "hidden": False}
STACK = {"kind": "stack", "axis": "vertical", "gap": "none", "padding": {"all": "none"}, "align": "stretch", "justify": "start", "grow": False, "wrap": False}


def node(node_id, key, component, layout, children):
    return {"type": "upsert", "id": node_id, "key": key, "component": component, "layout": layout, "style": STYLE, "activity": "idle", "disabled": False, "transition": None, "accessibility": ACCESSIBILITY, "bindings": [], "menu": None, "children": children}


def text(value, attributes=None):
    return {"type": "text", "value": value, "emphasize": None, "dataAttributes": attributes}


def section(body_key, leaves):
    ops = [node(2 + index, f"c{2 + index}", text(value, attributes), {"kind": "leaf", "width": "hug", "height": "hug"}, []) for index, (value, attributes) in enumerate(leaves)]
    ops.append(node(1, body_key, {"type": "container", "role": "plain"}, STACK, [2 + index for index in range(len(leaves))]))
    ops.append({"type": "setRoot", "id": 1})
    return {"surface": body_key, "baseRevision": 0, "revision": 1, "ops": ops}


ENGAGEMENTS_EN = {"note-navigator": {"status": [{"id": "words", "text": "2 words"}]}}
ENGAGEMENTS_DE = {"note-navigator": {"status": [{"id": "words", "text": "3 Wörter"}]}}
MEASURES = {"note-navigator": [{"kind": "toggle", "id": "wrap", "label": "Wrap", "value": True, "onChange": {"controllerId": "note", "action": "setWrap"}}]}
MEASURES_LEAVES = [('{"note-navigator":[{"kind":"toggle","id":"wrap","label":"Wrap",', None), ('"value":true,', {"01": '"onChange":{"controllerId":"note",', "02": '"action":"setWrap"}}]}'})]
assert json.loads("".join(value + "".join((attributes or {}).values()) for value, attributes in MEASURES_LEAVES)) == MEASURES
compact = lambda value: json.dumps(value, ensure_ascii=False, separators=(",", ":"))

corpus = json.loads(CORPUS.read_text(encoding="utf-8"))
if any("sectionsChanged" in step for step in corpus["steps"]):
    raise SystemExit("corpus already covers sections")
corpus["description"] = "Per-surface verdicts of one browser-actor patch offer on the shell (G-P1-4, S15 finding b, C13 P1). Every window kind of the app, every bodied panel and every reserved refresh section (`framework.section.*`, keyed by its body key) is a surface of the document's one actor. Each step applies `patches` to the stores the previous steps retained; `retained: false` starts an opening. `sectionsChanged` says an acknowledged patch moved a section; `sections` is what the shell decodes from the retained section stores after the step (the local refresh's own carrier: packed text leaves, `value` then sorted `dataAttributes`)."
for step in corpus["steps"]:
    step["sectionsChanged"] = False
    step["sections"] = {}
last = corpus["steps"][-1]["revisions"]
if last != {"note-navigator": 1}:
    raise SystemExit(f"corpus tail moved: {last}")
engagements = section("framework.section.engagements", [(compact(ENGAGEMENTS_EN), None)])
measures = section("framework.section.measures", MEASURES_LEAVES)
corpus["steps"] += [
    {
        "name": "reserved sections are retained beside the windows and decode to the refresh values",
        "retained": True,
        "patches": [engagements, measures],
        "verdicts": [{"surface": "framework.section.engagements", "outcome": "acknowledged", "revision": 1}, {"surface": "framework.section.measures", "outcome": "acknowledged", "revision": 1}],
        "stored": True,
        "surfacesAdded": True,
        "revisions": {"note-navigator": 1, "framework.section.engagements": 1, "framework.section.measures": 1},
        "sectionsChanged": True,
        "sections": {"engagements": ENGAGEMENTS_EN, "measures": MEASURES},
    },
    {
        "name": "a committed edit moves only the engagements section, in the human's locale",
        "retained": True,
        "patches": [{"surface": "framework.section.engagements", "baseRevision": 1, "revision": 2, "ops": [{"type": "setComponent", "id": 2, "component": text(compact(ENGAGEMENTS_DE))}]}],
        "verdicts": [{"surface": "framework.section.engagements", "outcome": "acknowledged", "revision": 2}],
        "stored": True,
        "surfacesAdded": False,
        "revisions": {"note-navigator": 1, "framework.section.engagements": 2, "framework.section.measures": 1},
        "sectionsChanged": True,
        "sections": {"engagements": ENGAGEMENTS_DE, "measures": MEASURES},
    },
    {
        "name": "a window-only frame leaves every section as it was",
        "retained": True,
        "patches": [{"surface": "note-navigator", "baseRevision": 1, "revision": 2, "ops": [{"type": "setComponent", "id": 1, "component": text("note-navigator.body 2")}]}],
        "verdicts": [{"surface": "note-navigator", "outcome": "acknowledged", "revision": 2}],
        "stored": True,
        "surfacesAdded": False,
        "revisions": {"note-navigator": 2, "framework.section.engagements": 2, "framework.section.measures": 1},
        "sectionsChanged": False,
        "sections": {"engagements": ENGAGEMENTS_DE, "measures": MEASURES},
    },
    {
        "name": "a stale section is refused and reset, and the shell holds no value for it until the resend",
        "retained": True,
        "patches": [{"surface": "framework.section.engagements", "baseRevision": 1, "revision": 2, "ops": [{"type": "setComponent", "id": 2, "component": text(compact(ENGAGEMENTS_EN))}]}],
        "verdicts": [{"surface": "framework.section.engagements", "outcome": "rejected", "revision": 0, "reason": "revisionMismatch"}],
        "stored": True,
        "surfacesAdded": False,
        "revisions": {"note-navigator": 2, "framework.section.engagements": 0, "framework.section.measures": 1},
        "sectionsChanged": False,
        "sections": {"measures": MEASURES},
    },
    {
        "name": "a section alone never opens a document",
        "retained": False,
        "patches": [section("framework.section.tools", [("{}", None)])],
        "verdicts": [{"surface": "framework.section.tools", "outcome": "rejected", "revision": 0, "reason": "window-surface-unpainted"}],
        "stored": False,
        "surfacesAdded": False,
        "revisions": {},
        "sectionsChanged": False,
        "sections": {},
    },
]

law = LAW.read_text(encoding="utf-8")


def swap(old, new):
    global law
    if law.count(old) != 1:
        raise SystemExit(f"law anchor count {law.count(old)}: {old[:80]!r}")
    law = law.replace(old, new)


swap(''' * over a document shown in two windows plus its panels, and Ajv checks every verdict the shell answers against the
 * private patch-handoff schema the worker's reader enforces. */''', ''' * over a document shown in two windows plus its panels and reserved refresh sections, and Ajv checks every verdict the
 * shell answers against the private patch-handoff schema the worker's reader enforces. The section values the shell
 * decodes (C13 P1) are compared with fast-deep-equal, and the host's section keys with the worker's visible-surface
 * contract, so the two ends of the actor can never disagree about which sections exist. */''')
swap('import { applyBrowserActorUiPatchesV1, browserActorPanelKeysV1, type BrowserActorUiStoresV1 } from "../../🟦️.tsx";',
     'import equal from "fast-deep-equal";\nimport type { PluginUiRefreshRequest, PluginViewState } from "@semio-tech/framework";\nimport { applyBrowserActorUiPatchesV1, BROWSER_ACTOR_SECTION_KEYS, browserActorPanelKeysV1, browserActorSectionValuesV1, withoutUiRefreshSectionsV1, type BrowserActorUiStoresV1 } from "../../🟦️.tsx";\nimport visibleSurfaces from "../../../../../../🏪️store/👷️worker/🪟️visible-surfaces/🔣️.json";')
swap("readonly surfacesAdded: boolean; readonly revisions: Readonly<Record<string, number>> };",
     "readonly surfacesAdded: boolean; readonly revisions: Readonly<Record<string, number>>; readonly sectionsChanged: boolean; readonly sections: Readonly<Record<string, unknown>> };")
swap('''      expect(applied.surfacesAdded, step.name).toBe(step.surfacesAdded);
      const stores = applied.stores;
      const revisions = stores === null ? {} : Object.fromEntries([...stores.windows, ...stores.panels].map(([key, store]) => [key, store.getRevisionSnapshot()]));
      expect(revisions, step.name).toEqual(step.revisions);
      for (const [key, revision] of Object.entries(step.revisions)) if (revision === 0) expect((stores!.windows.get(key) ?? stores!.panels.get(key)!).getState().root, `${step.name}: ${key} reset`).toBeNull();''', '''      expect(applied.surfacesAdded, step.name).toBe(step.surfacesAdded);
      expect(applied.sectionsChanged, step.name).toBe(step.sectionsChanged);
      const stores = applied.stores;
      const revisions = stores === null ? {} : Object.fromEntries([...stores.windows, ...stores.panels, ...stores.sections].map(([key, store]) => [key, store.getRevisionSnapshot()]));
      expect(revisions, step.name).toEqual(step.revisions);
      for (const [key, revision] of Object.entries(step.revisions)) if (revision === 0) expect((stores!.windows.get(key) ?? stores!.panels.get(key) ?? stores!.sections.get(key)!).getState().root, `${step.name}: ${key} reset`).toBeNull();
      const decoded = stores === null ? {} : Object.fromEntries([...browserActorSectionValuesV1(stores.sections, "corpus")].map(([key, { value }]) => [key, value]));
      expect(equal(decoded, step.sections), `${step.name}: ${JSON.stringify(decoded)}`).toBe(true);''')
swap('''      if (stores !== null) retained = stores;
    }
  });
''', '''      if (stores !== null) retained = stores;
    }
  });

  it("renders exactly the sections the worker's visible-surface contract announces", () => {
    expect([...BROWSER_ACTOR_SECTION_KEYS].sort()).toEqual(visibleSurfaces.sections.map(({ bodyKey }) => bodyKey).sort());
  });

  it("strips the reserved sections from an actor-served session's local refresh, and asks nothing when nothing else is left", () => {
    const viewState = {} as PluginViewState;
    const request: PluginUiRefreshRequest = { viewState, windows: [{ key: "note-navigator", bodyKey: "navigator" }], panels: [], engagements: { hash: "e" }, measures: {}, tools: {}, catalogue: {}, labels: { hash: "l" } };
    const stripped = withoutUiRefreshSectionsV1(request);
    expect(stripped === null ? null : Object.fromEntries(Object.entries(stripped).filter(([, value]) => value !== undefined))).toEqual({ viewState, windows: request.windows, panels: [], labels: { hash: "l" } });
    expect(withoutUiRefreshSectionsV1({ viewState, windows: [], panels: [], engagements: {}, measures: {}, tools: {}, catalogue: {} })).toBeNull();
    expect(withoutUiRefreshSectionsV1(null)).toBeNull();
  });
''')

if DRY:
    print("dry", len(corpus["steps"]), "steps;", len(law), "law chars")
else:
    CORPUS.write_text(json.dumps(corpus, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
    LAW.write_text(law, encoding="utf-8")
    print("ok", CORPUS)
    print("ok", LAW)
