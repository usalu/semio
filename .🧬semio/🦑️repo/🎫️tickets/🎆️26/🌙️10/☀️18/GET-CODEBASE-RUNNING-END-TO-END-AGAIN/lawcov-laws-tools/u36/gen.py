import sys, os, re, json
sys.path.insert(0, os.path.dirname(os.path.dirname(os.path.abspath(__file__))))
from lib import *

HERE = os.path.dirname(os.path.abspath(__file__))
read = lambda n: open(os.path.join(HERE, "..", n), encoding="utf8").read()
oracle = read("pylib.py") + "\n" + open(os.path.join(HERE, "oracle.py"), encoding="utf8").read()
js = lambda o: json.dumps(o, separators=(",", ":"), ensure_ascii=False)

PLAY = "✏s/🧑‍💻dev/🧩catalog/🤖generated/🚀playgrounds.json"
REG = "🧰framework/🛍products/🦑repo/🔨modules/🎛dashboard/🎮registry/🦀.rs"
REGS = "🧰framework/🛍products/🦑repo/🔨modules/🎛dashboard/🧬schema/🎮registry/🔣.json"
RM = "📋project.json"
F = "✏s/🔌plugins/alpha/🔣.json"
E = "✏s/🔌plugins/alpha/🧩extensions/ext1/🔣.json"
LBL = {"native": {"en": "A", "de": "B"}}
BAD = "lacks equivalent en/de labels or declares a default language"
NOCOV = "has no owner-qualified playground command with React + WGPU Wasm + WGPU native surfaces"
REND = ["react", "wgpu-wasm", "wgpu-native"]
RSRC = 'const RENDERERS: &[&str] = &["react", "wgpu-wasm", "wgpu-native"];'
GOOD_M = {"name": "workspace", "targets": {"verify": {"options": {"command": "bun 📜script.ts verify", "forwardAllArgs": True}, "metadata": {"semio": {"dashboard": {"parameters": [{"kind": "text", "valuePositional": True}]}}}}}}
GOOD_S = {"$defs": {"Parameter": {"properties": {"kind": {"enum": ["text", "number"]}, "valuePositional": {"type": "boolean"}}}}}
GATE_ROWS = [{"id": "interactivity", "check": ["interactivity"]}, {"id": "interactivity-tool-jobs", "check": ["interactivity", "tool-jobs"]}, {"id": "interactivity-apps", "check": ["interactivity", "apps"]}, {"id": "interactivity-apps-actions", "check": ["interactivity", "apps", "--actions"]}, {"id": "dependencies", "check": ["dependencies"]}, {"id": "dependencies-literal-external", "check": ["dependencies", "literal-external"]}, {"id": "composed-child-refs", "check": ["composed-child-refs"]}]
cases = []


def add(id, rule, expect, args=None, files=None, **kw):
    row = {"id": id, "rule": rule}
    if args is not None:
        row["args"] = args
    if files is not None:
        row["files"] = files
    row.update(kw)
    row["expect"] = expect
    cases.append(row)


def desc(pid="p.alpha", apps=None, version=1, role="plugin"):
    return js({"descriptorVersion": version, "role": role, "manifest": {"pluginId": pid, "apps": apps if apps is not None else []}})


def app(id, label=LBL, windows=None):
    out = {"id": id, "label": label}
    if windows is not None:
        out["windowKinds"] = windows
    return out


def act(id, label=LBL, job=None):
    out = {"id": id, "label": label}
    if job is not None:
        out["semantics"] = {"execution": {"interactiveJob": job}}
    return out


def arow(file, pid, ids, actions=None):
    return {"file": file, "kind": "app", "pluginId": pid, "appIds": ids, "actions": actions or []}


def ract(a, w, i, d=None):
    out = {"appId": a, "windowId": w, "actionId": i}
    if d is not None:
        out["disposition"] = d
    return out


add("descriptor-valid-app", "descriptor", {"row": arow(F, "p.alpha", ["a#editor"], [ract("a#editor", "w", "open", "migrated"), ract("a#editor", "w", "close")]), "failures": []},
    [F, desc(apps=[app("a#editor", windows=[{"id": "w", "actions": [act("open", job="migrated"), act("close")]}])])])
add("descriptor-empty-source-is-invalid-json", "descriptor", {"failures": [F + ": descriptor is not valid JSON"]}, [F, ""])
add("descriptor-truncated-json-is-invalid", "descriptor", {"failures": [F + ": descriptor is not valid JSON"]}, [F, "{"])
add("descriptor-version-and-role-are-checked-together", "descriptor", {"row": arow(F, "p.alpha", ["a1"]), "failures": [F + ": descriptorVersion must be 1", F + ": role must be plugin"]},
    [F, desc(apps=[app("a1")], version=2, role="extension")])
add("descriptor-boolean-version-is-not-one", "descriptor", {"row": arow(F, "p.alpha", ["a1"]), "failures": [F + ": descriptorVersion must be 1"]},
    [F, '{"descriptorVersion":true,"role":"plugin","manifest":{"pluginId":"p.alpha","apps":[{"id":"a1","label":' + js(LBL) + '}]}}'])
add("descriptor-without-manifest", "descriptor", {"failures": [F + ": manifest is missing"]}, [F, '{"descriptorVersion":1,"role":"plugin"}'])
add("descriptor-with-scalar-manifest", "descriptor", {"failures": [F + ": manifest is missing"]}, [F, '{"descriptorVersion":1,"role":"plugin","manifest":"x"}'])
add("descriptor-with-array-manifest-lacks-apps", "descriptor", {"failures": [F + ": manifest.apps is missing"]}, [F, '{"descriptorVersion":1,"role":"plugin","manifest":[]}'])
add("descriptor-with-non-list-apps", "descriptor", {"failures": [F + ": manifest.apps is missing"]}, [F, '{"descriptorVersion":1,"role":"plugin","manifest":{"apps":{}}}'])
add("descriptor-scalar-top-level-has-no-manifest", "descriptor", {"failures": [F + ": descriptorVersion must be 1", F + ": role must be plugin", F + ": manifest is missing"]}, [F, "5"])
add("descriptor-null-top-level-is-invalid", "descriptor", {"failures": [F + ": descriptor is not valid JSON"]}, [F, "null"],
    defect="🟦.ts:99 reads descriptor.descriptorVersion on a parsed null, so a descriptor file holding the literal null throws a TypeError instead of reporting a failure", oracle=False)
add("descriptor-placeholder-plugin-id-and-no-apps", "descriptor", {"row": arow(F, "empty", []), "failures": [F + ": pluginId must be a non-placeholder owned identifier", F + ": manifest.apps is empty and has no parent activation evidence"]}, [F, desc(pid="empty")])
add("descriptor-plugin-id-is-trimmed", "descriptor", {"row": arow(F, "p", ["a1"]), "failures": []}, [F, desc(pid="  p  ", apps=[app("a1")])])
add("descriptor-missing-plugin-id", "descriptor", {"row": arow(F, "", ["a1"]), "failures": [F + ": pluginId must be a non-placeholder owned identifier"]}, [F, js({"descriptorVersion": 1, "role": "plugin", "manifest": {"apps": [app("a1")]}})])
add("descriptor-non-object-apps", "descriptor", {"row": arow(F, "p.alpha", ["ok"]), "failures": [
    F + ": app 0 is not an object", F + ": app 1 is not an object", F + ": app 2 is not an object",
    F + ": app 4 has no owned id", F + ": app 4 " + BAD, F + ": app 5 has no owned id", F + ": app 5 " + BAD]},
    [F, desc(apps=[1, None, "x", app("ok"), {}, []])])
labels = [("l1", {"native": {"en": "A", "de": "B"}}, True), ("l2", {"reuse": {"en": "A", "de": "B"}}, True), ("l3", {"native": {"en": "A", "de": "B"}, "reuse": {"en": "C", "de": "D"}}, True),
          ("l4", {"default": "X", "native": {"en": "A", "de": "B"}}, False), ("l5", {"native": {"en": "A"}}, False), ("l6", {"native": {"en": "", "de": "B"}}, False),
          ("l7", {"native": {"en": "A", "de": "B"}, "reuse": {"en": "A", "de": "B", "default": "x"}}, False), ("l8", [1], False), ("l9", None, False),
          ("l11", {"native": {"en": "A", "de": 3}}, False), ("l12", {"native": []}, False), ("l13", {"native": {"en": "A", "de": "B"}, "reuse": None}, True)]
apps = [app(i, lb) for i, lb, _ in labels] + [{"id": "l10"}]
add("descriptor-label-variants", "descriptor", {"row": arow(F, "p.alpha", [i for i, _, _ in labels] + ["l10"]), "failures": [F + ": app " + i + " " + BAD for i, _, ok in labels if not ok] + [F + ": app l10 " + BAD]},
    [F, desc(apps=apps)])
add("descriptor-window-and-action-shapes", "descriptor", {"row": arow(F, "p.alpha", ["w1"], [ract("w1", "w", "ok"), ract("w1", "", ""), ract("w1", "", "x"), ract("w1", "", ""), ract("w1", "", "y", "migrated")]), "failures": [
    F + ": app w1 window 0 is not an object", F + ": app w1 window 3 action 0 is not an object", F + ": app w1 window 3 action 1 has no owned id",
    F + ": action w1/3/x " + BAD, F + ": app w1 window 3 action 3 has no owned id", F + ": action w1/3/3 " + BAD]},
    [F, desc(apps=[app("w1", windows=[1, {"id": "w", "actions": [act("ok")]}, {}, {"id": "", "actions": [None, {"label": LBL}, {"id": "x"}, [], act("y", job="migrated")]}])])])
add("descriptor-interactive-job-must-be-a-non-empty-string", "descriptor", {"row": arow(F, "p.alpha", ["d1"], [ract("d1", "w", "a1"), ract("d1", "w", "a2"), ract("d1", "w", "a3"), ract("d1", "w", "a4"), ract("d1", "w", "a5", "batch")]), "failures": []},
    [F, desc(apps=[app("d1", windows=[{"id": "w", "actions": [act("a1", job=""), act("a2", job=5), {"id": "a3", "label": LBL, "semantics": []}, {"id": "a4", "label": LBL, "semantics": {"execution": []}}, act("a5", job="batch")]}])])])
add("descriptor-65-apps-exceed-capacity", "descriptorCounts", {"failures": [F + ": 65 apps exceed fixed capacity 64"], "appIds": 64, "actions": 0}, [F, desc(apps=[app("a%02d" % i) for i in range(65)])])
add("descriptor-64-apps-fit-capacity", "descriptorCounts", {"failures": [], "appIds": 64, "actions": 0}, [F, desc(apps=[app("a%02d" % i) for i in range(64)])])
add("descriptor-600-actions-in-one-window-are-capped", "descriptorCounts", {"failures": [F + ": app big has 600 action rows exceeding fixed capacity 512"], "appIds": 1, "actions": 512},
    [F, desc(apps=[app("big", windows=[{"id": "w", "actions": [act("x%d" % i) for i in range(600)]}])])])
add("descriptor-513-actions-across-windows-are-capped-per-window", "descriptorCounts", {"failures": [F + ": app big has 513 action rows exceeding fixed capacity 512"], "appIds": 1, "actions": 513},
    [F, desc(apps=[app("big", windows=[{"id": "w1", "actions": [act("x%d" % i) for i in range(300)]}, {"id": "w2", "actions": [act("y%d" % i) for i in range(213)]}])])])
EXTSRC = ['const BUNDLE: &str = "p.ext";', 'let b = ExtensionBundle::new(BUNDLE, 1).extends("p.alpha");']
EXTD = '{"descriptorVersion":1,"role":"extension","manifest":{"apps":[]}}'
ext_row = lambda pid, parent=None: {"file": E, "kind": "extension", "pluginId": pid, **({"parentPluginId": parent} if parent else {}), "appIds": [], "actions": []}
add("extension-resolves-constant-bundle-and-parent", "descriptor", {"row": ext_row("p.ext", "p.alpha"), "failures": []}, [E, EXTD, "\n".join(EXTSRC)])
add("extension-resolves-literal-bundle-with-spacing", "descriptor", {"row": ext_row("p.lit", "p.alpha"), "failures": []}, [E, EXTD, 'ExtensionBundle::new( " p.lit " , 2).extends( "p.alpha" )'])
add("extension-without-source-reports-both-gaps", "descriptor", {"row": ext_row(""), "failures": [E + ": extension source declares no resolvable ExtensionBundle::new(<pluginId>, …)", E + ": extension source has no parent .extends(...) activation"]}, [E, EXTD, ""])
add("extension-with-unresolved-constant-has-no-id", "descriptor", {"row": ext_row("", "p.alpha"), "failures": [E + ": extension source declares no resolvable ExtensionBundle::new(<pluginId>, …)"]}, [E, EXTD, 'ExtensionBundle::new(MISSING, 1).extends("p.alpha")'])
add("extension-must-not-duplicate-apps-or-wear-the-plugin-role", "descriptor", {"row": ext_row("p.ext", "p.alpha"), "failures": [E + ": role must be extension", E + ": extension descriptor must delegate app activation to its parent instead of duplicating apps"]},
    [E, '{"descriptorVersion":1,"role":"plugin","manifest":{"apps":[{}]}}', "\n".join(EXTSRC)])

DESCS = [{"file": F, "kind": "app", "pluginId": "p", "appIds": ["a"], "actions": [ract("a", "w", "x", "migrated"), ract("a", "w", "y"), ract("a", "w", "z", "batch")]}]
add("disposition-lists-every-action-that-is-not-migrated", "disposition", [F + ': a/w/y has interactiveJob="missing"', F + ': a/w/z has interactiveJob="batch"'], [DESCS])
add("disposition-of-no-descriptors-is-empty", "disposition", [], [[]])
PD = [{"file": F, "kind": "app", "pluginId": "p", "appIds": ["a"], "actions": [ract("a", "w", "x", "migrated"), ract("a", "w", "y", "migrated"), ract("a", "w", "z", "migrated"), ract("a", "w", "k", "batch")]}]
add("production-accepts-owner-local-commands-and-shared-routes", "production", [F + ": a/w/y declares migrated without an accepted owner-local production command or shared reserved route"],
    [PD, [{"file": "✏s/🔌plugins/alpha/🦀.rs", "id": "x"}, {"file": "✏s/🔌plugins/beta/🦀.rs", "id": "y"}], ["z"]])
HUB = "🌎hub/🧩compositions/c1/🔣.json"
add("production-owner-is-the-first-three-path-segments", "production", [HUB + ": a/w/x declares migrated without an accepted owner-local production command or shared reserved route"],
    [[{"file": HUB, "kind": "app", "pluginId": "c", "appIds": ["a"], "actions": [ract("a", "w", "x", "migrated"), ract("a", "w", "y", "migrated")]}], [{"file": "🌎hub/🧩compositions/c1/deep/🦀.rs", "id": "y"}, {"file": "🌎hub/🧩compositions/c2/🦀.rs", "id": "x"}], []])

pg = lambda v, p, r=1, w=1, a=None: {"variant": v, "pluginId": p, "ports": {"react": r, "wgpu": w}, **({"app": a} if a is not None else {})}
prow = lambda v, p, r=1, w=1, a=None: {"variant": v, "pluginId": p, "react": r, "wgpu": w, **({"appId": a} if a else {})}
add("playgrounds-valid-catalog", "playgrounds", {"rows": [prow("v1", "p", 1, 2, "a#editor"), prow("v2", "p", 0, 0), prow("v3", "q", 0, 0)], "failures": []},
    [js([pg("v1", "p", 1, 2, "a#editor"), pg("v2", "p", 0, 0, ""), {"variant": "v3", "pluginId": "q"}])])
add("playgrounds-invalid-json", "playgrounds", {"rows": [], "failures": [PLAY + ": invalid JSON"]}, ["{"])
add("playgrounds-not-a-list", "playgrounds", {"rows": [], "failures": [PLAY + ": catalog is not a list"]}, ["{}"])
add("playgrounds-empty-list", "playgrounds", {"rows": [], "failures": [PLAY + ": no playground rows discovered"]}, ["[]"])
add("playgrounds-malformed-rows", "playgrounds", {"rows": [prow("ok", "p", 0, 0)], "failures": [PLAY + ": malformed playground row"] * 5},
    [js([{"variant": "", "pluginId": "p"}, {"variant": "v"}, 5, None, [], {"variant": "ok", "pluginId": "p", "ports": {"react": "1", "wgpu": True}}])])
add("playgrounds-duplicate-variant", "playgrounds", {"rows": [prow("v", "p"), prow("v", "q")], "failures": [PLAY + ': duplicate variant "v"']}, [js([pg("v", "p"), pg("v", "q")])])
add("playgrounds-port-numbers-are-taken-as-they-are", "playgrounds", {"rows": [prow("v", "p", -1, 3)], "failures": []}, [js([pg("v", "p", -1, 3)])])
ren = lambda names, failures: {"renderers": names, "failures": failures}
RFAIL = lambda found: [REG + ": RENDERERS must offer exactly one React, one Wasm and one native surface, found " + found]
add("renderers-canonical-axis", "renderers", ren(REND, []), [RSRC])
add("renderers-tolerate-spacing-and-visibility", "renderers", ren(["react", "x-wasm", "y-native"], []), ['pub  const  RENDERERS : &[&str] =&[ "react" ,"x-wasm","y-native"]'])
add("renderers-bare-native-suffix-counts", "renderers", ren(["react", "x-wasm", "native"], []), ['const RENDERERS: &[&str] = &["react", "x-wasm", "native"];'])
add("renderers-missing-axis", "renderers", ren([], [REG + ": the RENDERERS axis of the playground commands is not readable"]), ["fn main() {}"])
add("renderers-empty-axis", "renderers", ren([], [REG + ": the RENDERERS axis of the playground commands is not readable"]), ["const RENDERERS: &[&str] = &[];"])
add("renderers-prefixed-name-is-not-the-axis", "renderers", ren([], [REG + ": the RENDERERS axis of the playground commands is not readable"]), ['const MY_RENDERERS: &[&str] = &["react", "a-wasm", "b-native"];'])
add("renderers-two-entries", "renderers", ren(["react", "wgpu-wasm"], RFAIL("react, wgpu-wasm")), ['const RENDERERS: &[&str] = &["react", "wgpu-wasm"];'])
add("renderers-four-entries", "renderers", ren(["react", "a-wasm", "b-native", "c-native"], RFAIL("react, a-wasm, b-native, c-native")), ['const RENDERERS: &[&str] = &["react", "a-wasm", "b-native", "c-native"];'])
add("renderers-two-react-one-wasm", "renderers", ren(["react", "react", "x-wasm"], RFAIL("react, react, x-wasm")), ['const RENDERERS: &[&str] = &["react", "react", "x-wasm"];'])
add("surfaces-all-three", "surfaces", REND, [{"variant": "v", "pluginId": "p", "react": 1, "wgpu": 1}, REND])
add("surfaces-wgpu-only", "surfaces", ["wgpu-wasm", "wgpu-native"], [{"variant": "v", "pluginId": "p", "react": 0, "wgpu": 5}, REND])
add("surfaces-none-for-zero-and-negative-ports", "surfaces", [], [{"variant": "v", "pluginId": "p", "react": -1, "wgpu": 0}, REND])
add("surfaces-of-an-empty-axis", "surfaces", [], [{"variant": "v", "pluginId": "p", "react": 1, "wgpu": 1}, []])
D2 = lambda ids, pid="p", kind="app": {"file": F, "kind": kind, "pluginId": pid, "appIds": ids, "actions": []}
P1 = lambda v, p, r, w, a=None: {"variant": v, "pluginId": p, "react": r, "wgpu": w, **({"appId": a} if a else {})}
add("coverage-role-neutral-owner-qualified", "coverage", [F + ": b " + NOCOV], [[D2(["a#editor", "b"])], [P1("v1", "p", 1, 1, "a#viewer"), P1("v2", "p", 1, 0)], REND])
add("coverage-ignores-other-plugins", "coverage", [F + ": x " + NOCOV], [[D2(["x"])], [P1("v", "q", 1, 1)], REND])
add("coverage-app-id-must-match-up-to-the-role-suffix", "coverage", [F + ": a " + NOCOV, F + ": a#editorx " + NOCOV], [[D2(["a", "a#editorx"])], [P1("v", "p", 1, 1, "other#editor"), P1("w", "p", 1, 1, "a#viewer")], REND])
add("coverage-everything-is-complete-for-an-empty-axis", "coverage", [], [[D2(["a"])], [P1("v", "p", 0, 0)], []])
add("coverage-ignores-extension-descriptors", "coverage", [], [[D2(["z"], "p", "extension")], [], REND])
add("coverage-of-nothing", "coverage", [], [[], [], REND])
GM, GS = js(GOOD_M), js(GOOD_S)
mod = lambda f: (lambda m: (f(m), m)[1])(json.loads(GM))
GPREFIX = RM + ": workspace:verify must forward free arguments to 📜script.ts verify"
add("gates-healthy-manifest-and-schema", "gates", [], [GM, GS])
add("gates-invalid-manifest-json-wins-over-schema", "gates", [RM + ": invalid JSON"], ["{", "{"])
add("gates-invalid-schema-json", "gates", [REGS + ": invalid JSON"], [GM, "["])
add("gates-empty-objects-fail-every-check", "gates", [
    REGS + ": the Parameter schema declares no text kind", REGS + ": the Parameter schema declares no valuePositional", RM + ": the root project must be named workspace",
    RM + ": target workspace:verify is missing, so no verification gate is a dashboard command", GPREFIX,
    RM + ": workspace:verify must declare exactly one positional text parameter in metadata.semio.dashboard, found 0"], ["{}", "{}"])
add("gates-null-documents-are-empty-records", "gates", [
    REGS + ": the Parameter schema declares no text kind", REGS + ": the Parameter schema declares no valuePositional", RM + ": the root project must be named workspace",
    RM + ": target workspace:verify is missing, so no verification gate is a dashboard command", GPREFIX,
    RM + ": workspace:verify must declare exactly one positional text parameter in metadata.semio.dashboard, found 0"], ["null", "[]"])
v = lambda f: js(mod(f))
add("gates-command-must-end-with-the-script-verify", "gates", [GPREFIX], [v(lambda m: m["targets"]["verify"]["options"].update(command="bun 📜script.ts verify --x")), GS])
add("gates-command-may-carry-a-runner-prefix", "gates", [], [v(lambda m: m["targets"]["verify"]["options"].update(command="node tools/📜script.ts verify")), GS])
add("gates-forward-all-args-must-be-boolean-true", "gates", [GPREFIX], [v(lambda m: m["targets"]["verify"]["options"].update(forwardAllArgs="true")), GS])
add("gates-command-must-be-a-string", "gates", [GPREFIX], [v(lambda m: m["targets"]["verify"]["options"].update(command=5)), GS])
add("gates-two-positional-text-parameters", "gates", [RM + ": workspace:verify must declare exactly one positional text parameter in metadata.semio.dashboard, found 2"],
    [v(lambda m: m["targets"]["verify"]["metadata"]["semio"]["dashboard"]["parameters"].append({"kind": "text", "valuePositional": True})), GS])
add("gates-positional-parameter-must-be-text", "gates", [RM + ": workspace:verify must declare exactly one positional text parameter in metadata.semio.dashboard, found 0"],
    [v(lambda m: m["targets"]["verify"]["metadata"]["semio"]["dashboard"].update(parameters=[{"kind": "number", "valuePositional": True}, {"kind": "text", "valuePositional": "yes"}])), GS])
add("gates-wrong-root-name", "gates", [RM + ": the root project must be named workspace"], [v(lambda m: m.update(name="other")), GS])
add("gates-schema-kind-enum-must-offer-text", "gates", [REGS + ": the Parameter schema declares no text kind"], [GM, js({"$defs": {"Parameter": {"properties": {"kind": {"enum": ["number"]}, "valuePositional": None}}}})])
add("gates-schema-kind-enum-must-be-a-list", "gates", [REGS + ": the Parameter schema declares no text kind"], [GM, js({"$defs": {"Parameter": {"properties": {"kind": {"enum": "text"}, "valuePositional": 1}}}})])
add("gates-custom-gate-rows-are-validated", "gates", ["gate b: duplicate check \"x y\"", "gate c: check must be a list of non-empty words", "gate d: check must be a list of non-empty words", "gate e: check must be a list of non-empty words", "gate f: check must be a list of non-empty words", "gate f: duplicate check \"\""],
    [GM, GS, [{"id": "a", "check": ["x", "y"]}, {"id": "b", "check": ["x", "y"]}, {"id": "c", "check": []}, {"id": "d", "check": ["", "z"]}, {"id": "e", "check": ["a b"]}, {"id": "f", "check": []}]])
add("constants-capacities-and-required-gates", "constants", {"apps": 64, "actions": 512, "gates": GATE_ROWS})


def tree(plugins, hub=(), play=None, extra=None):
    files = {RM: [js(GOOD_M)], REGS: [js(GOOD_S)], REG: [RSRC]}
    pgs = []
    for name, pid, apps_, complete in plugins:
        files["✏s/🔌plugins/%s/🔣.json" % name] = [desc(pid, apps_)]
        pgs.append(pg(name + "-v", pid, 1, 1 if complete else 0))
    for name, pid, apps_ in hub:
        files["🌎hub/🧩compositions/%s/🔣.json" % name] = [desc(pid, apps_)]
        pgs.append(pg(name + "-v", pid))
    files[PLAY] = [js(play if play is not None else pgs)]
    files.update(extra or {})
    return files


AF, BF, GF = "✏s/🔌plugins/alpha/🔣.json", "✏s/🔌plugins/beta/🔣.json", "🌎hub/🧩compositions/gamma/🔣.json"
files = tree([("alpha", "p.alpha", [app("a#editor", windows=[{"id": "w", "actions": [act("open", job="migrated"), act("close", job="migrated")]}]), app("b")], True),
              ("beta", "p.beta", [app("c", windows=[{"id": "w", "actions": [act("run")]}])], False)], hub=[("gamma", "p.gamma", [app("g")])])
add("discovery-healthy-tree-with-one-incomplete-surface", "discovery", {
    "descriptors": [arow(AF, "p.alpha", ["a#editor", "b"], [ract("a#editor", "w", "open", "migrated"), ract("a#editor", "w", "close", "migrated")]), arow(BF, "p.beta", ["c"], [ract("c", "w", "run")]), arow(GF, "p.gamma", ["g"])],
    "appCount": 4, "surfaceCount": 7, "actionCount": 3, "migratedActionCount": 2, "missingActionCount": 1, "surfaceCoveredAppCount": 3, "surfaceMissingAppCount": 1,
    "duplicateAppIds": [], "playgrounds": 3, "renderers": REND, "gates": 7, "failures": [BF + ": c " + NOCOV]}, files=files)
add("discovery-empty-tree-reports-every-missing-source", "discovery", {
    "descriptors": [], "appCount": 0, "surfaceCount": 0, "actionCount": 0, "migratedActionCount": 0, "missingActionCount": 0, "surfaceCoveredAppCount": 0, "surfaceMissingAppCount": 0,
    "duplicateAppIds": [], "playgrounds": 0, "renderers": [], "gates": 0,
    "failures": sorted(["no plugin descriptors discovered", RM + ": invalid JSON", REG + ": the RENDERERS axis of the playground commands is not readable", PLAY + ": invalid JSON"], key=lambda s: s.encode("utf-16-be"))}, files={"README.md": ["x"]})
BF2 = "✏s/🔌plugins/beta/🔣.json"
files = tree([("alpha", "p.alpha", [app("a1"), app("shared")], True), ("beta", "p.alpha", [app("b1"), app("shared")], True)], extra={
    "✏s/🔌plugins/alpha/🧩extensions/ext1/🔣.json": [EXTD], "✏s/🔌plugins/alpha/🧩extensions/ext1/🦀.rs": ['ExtensionBundle::new("p.ext1", 1).extends("p.missing");'],
    "✏s/🔌plugins/alpha/🧩extensions/ext2/🔣.json": [EXTD], "✏s/🔌plugins/alpha/🧩extensions/ext2/🦀.rs": ['ExtensionBundle::new("p.ext2", 1).extends("p.alpha");'],
    "✏s/🔌plugins/alpha/sub/🔣.json": ["{"], "✏s/🔌plugins/alpha/🧩extensions/🧫fixtures/🔣.json": ["{"], "✏s/🔌plugins/alpha/🧩extensions/🧪tests/🔣.json": ["{"],
    "✏s/🔌plugins/node_modules/🔣.json": ["{"], "✏s/🔌plugins/.hidden/🔣.json": ["{"], "✏s/🔌plugins/target/🔣.json": ["{"], "✏s/🔌plugins/🤖generated/🔣.json": ["{"],
    "✏s/🔌plugins/alpha/readme.json": ["{"], "other/🔌plugins/zzz/🔣.json": ["{"]})
X1, X2 = "✏s/🔌plugins/alpha/🧩extensions/ext1/🔣.json", "✏s/🔌plugins/alpha/🧩extensions/ext2/🔣.json"
xr = lambda f, pid, parent: {"file": f, "kind": "extension", "pluginId": pid, "parentPluginId": parent, "appIds": [], "actions": []}
add("discovery-ambiguity-duplicates-extensions-and-skipped-locations", "discovery", {
    "descriptors": [arow(AF, "p.alpha", ["a1", "shared"]), xr(X1, "p.ext1", "p.missing"), xr(X2, "p.ext2", "p.alpha"), arow(BF2, "p.alpha", ["b1", "shared"])],
    "appCount": 4, "surfaceCount": 6, "actionCount": 0, "migratedActionCount": 0, "missingActionCount": 0, "surfaceCoveredAppCount": 4, "surfaceMissingAppCount": 0,
    "duplicateAppIds": [{"id": "shared", "files": [AF, BF2]}], "playgrounds": 2, "renderers": REND, "gates": 7,
    "failures": sorted(['pluginId "p.alpha" is ambiguous across %s, %s' % (AF, BF2), X1 + ': parent "p.missing" has no discovered app activation'], key=lambda s: s.encode("utf-16-be"))}, files=files)
files = tree([("pn1", "", [app("x1")], True), ("pn2", "", [app("x2")], True)], play=[pg("q-v", "q")])
P1F, P2F = "✏s/🔌plugins/pn1/🔣.json", "✏s/🔌plugins/pn2/🔣.json"
add("discovery-blank-plugin-ids-are-never-ambiguous-but-uncovered", "discovery", {
    "descriptors": [arow(P1F, "", ["x1"]), arow(P2F, "", ["x2"])], "appCount": 2, "surfaceCount": 3, "actionCount": 0, "migratedActionCount": 0, "missingActionCount": 0, "surfaceCoveredAppCount": 0, "surfaceMissingAppCount": 2,
    "duplicateAppIds": [], "playgrounds": 1, "renderers": REND, "gates": 7,
    "failures": sorted([P1F + ": pluginId must be a non-placeholder owned identifier", P2F + ": pluginId must be a non-placeholder owned identifier", P1F + ": x1 " + NOCOV, P2F + ": x2 " + NOCOV], key=lambda s: s.encode("utf-16-be"))}, files=files)
many = tree([("p%03d" % i, "p%03d" % i, [app("a%03d" % i)], True) for i in range(257)])
add("discovery-257-descriptors-exceed-the-fixed-capacity", "discoverySummary", {
    "descriptors": 256, "appCount": 256, "surfaceCount": 771, "actionCount": 0, "migratedActionCount": 0, "missingActionCount": 0, "surfaceCoveredAppCount": 256, "surfaceMissingAppCount": 0,
    "duplicateAppIds": [], "playgrounds": 257, "renderers": REND, "gates": 7, "failures": ["257 descriptors exceed fixed capacity 256"]}, files=many)
for i, (src, out) in enumerate([('{ "a" : 1 ,\n "b":[true,null,"x"] }', '{"a":1,"b":[true,null,"x"]}'), ('{"nested":{"k":[1,2,{"z":"🧪"}]}}', '{"nested":{"k":[1,2,{"z":"🧪"}]}}'), ('{"s":"a\\nb"}', '{"s":"a\\nb"}'), ("{", None)]):
    add("oracle-json-%d" % i, "oracleJson", out, [src], oracle=False)

props = {"expect": True, "args": {"type": "array"}}
schema = val_schema("Interactivity All App Corpus", ["descriptor", "descriptorCounts", "disposition", "production", "playgrounds", "renderers", "surfaces", "coverage", "gates", "discovery", "discoverySummary", "constants", "oracleJson"])
rules = '''  descriptor: (_root, file: string, source: string, ext = "") => audit.interactivityAllAppDescriptorFromSource(file, source, ext),
  descriptorCounts: (_root, file: string, source: string) => { const r = audit.interactivityAllAppDescriptorFromSource(file, source); return { failures: r.failures, appIds: r.row?.appIds.length ?? null, actions: r.row?.actions.length ?? null }; },
  disposition: (_root, descriptors: any[]) => audit.interactivityAllAppActionDispositionFailures(descriptors),
  production: (_root, descriptors: any[], commands: any[], routes: string[]) => audit.interactivityAllAppActionProductionFailures(descriptors, commands, routes),
  playgrounds: (_root, source: string) => audit.interactivityAllAppPlaygroundsFromSource(source),
  renderers: (_root, source: string) => audit.interactivityAllAppRenderersFromSource(source),
  surfaces: (_root, playground: any, renderers: string[]) => audit.interactivityAllAppPlaygroundSurfaces(playground, renderers),
  coverage: (_root, descriptors: any[], playgrounds: any[], renderers: string[]) => audit.interactivityAllAppSurfaceCoverageFailures(descriptors, playgrounds, renderers),
  gates: (_root, manifest: string, schema: string, gates?: any[]) => (gates === undefined ? audit.interactivityAllAppGateFailures(manifest, schema) : audit.interactivityAllAppGateFailures(manifest, schema, gates)),
  discovery: (root) => { const r = audit.interactivityAllAppDiscovery(root); return { ...r, descriptors: [...r.descriptors].sort((a, b) => (a.file < b.file ? -1 : a.file > b.file ? 1 : 0)) }; },
  discoverySummary: (root) => { const r = audit.interactivityAllAppDiscovery(root); return { ...r, descriptors: r.descriptors.length }; },
  constants: () => ({ apps: audit.INTERACTIVITY_ALL_APP_APPS_PER_DESCRIPTOR_CAPACITY, actions: audit.INTERACTIVITY_ALL_APP_ACTIONS_PER_APP_CAPACITY, gates: audit.INTERACTIVITY_ALL_APP_REQUIRED_GATES.map((gate) => ({ id: gate.id, check: [...gate.check] })) }),
  oracleJson: () => { throw new Error("oracleJson rows run in the dedicated test"); },'''
extra = '''
for (const row of (corpus.cases as any[]).filter((candidate) => candidate.rule === "oracleJson")) {
  test(`oracleJson: ${row.id}`, async () => {
    if (row.expect === null) {
      await expect(audit.interactivityAllAppOracleJson(row.args[0])).rejects.toThrow();
      expect(() => JSON.parse(row.args[0])).toThrow();
      return;
    }
    expect(await audit.interactivityAllAppOracleJson(row.args[0])).toBe(row.expect);
    expect(JSON.stringify(JSON.parse(row.args[0]))).toBe(row.expect);
  });
}
'''
ts = val_ts("🧭 The all-app interactivity gate reads app descriptors, extension bundles, playground catalogs, the renderer axis and the verify gate manifest exactly as their language-agnostic corpus says, and an independent Python port reproduces every row; the JSON oracle agrees with the built-in JSON parser.", 'import * as audit from "../../🟦.ts";', rules, extra)
ts = ts.replace("for (const row of corpus.cases as any[]) {", 'for (const row of (corpus.cases as any[]).filter((candidate) => candidate.rule !== "oracleJson")) {')
folder, plen = write_test("⚖laws/🔬interactivity-audit/🧭all-app", "🧭all-app", ts, {"version": 1, "cases": cases}, schema, oracle)
print(folder, plen, len(cases))
