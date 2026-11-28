#!/usr/bin/env python3
"""H75 wiring patch for `repo bench latency`: `patch-wire.py check|apply`. Anchored text edits of the operations catalogue, the bench dispatch and option names and the two bench tests; the help snapshot fixture is regenerated afterwards with `help-regen.ts` (it equals the renderer's output, verified against the unchanged catalogue)."""
import os, sys
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
edits = []
def edit(path, old, new, count=1):
    edits.append((path, old, new, count))

CAT = B + "/⚙️operations/🔣️catalog.json"
edit(CAT, '"synopsis": "bench list|run|compare|acceptance",', '"synopsis": "bench list|run|compare|acceptance|latency",')
SUB = ('        { "name": "latency", "synopsis": "bench latency [--commands=<ids|all>] [--runs=<n>] [--strict]", "flags": ["--commands", "--runs", "--strict", "--root", "--file", "--json"], '
 '"summary": { "en": "Run the developer commands (inputs, run --exclude-dependencies --dry-run, test-file, typecheck, fleet status) on a fixed small target with the phase profile on, and judge the time per phase group and the round trips (graph service requests, child processes, cache lookups) against the declared budgets; budgets exceeded are warnings, only --strict turns them into a failing exit code.", '
 '"de": "Führt die Entwicklerbefehle (inputs, run --exclude-dependencies --dry-run, test-file, typecheck, fleet status) auf einem festen kleinen Ziel mit eingeschaltetem Phasenprofil aus und beurteilt die Zeit je Phasengruppe und die Rundreisen (Graphdienst-Anfragen, Kindprozesse, Cache-Abfragen) anhand der deklarierten Budgets; überschrittene Budgets sind Warnungen, erst --strict macht daraus einen fehlschlagenden Exit-Code." } }')
edit(CAT, '\n      ],\n      "flags": ["--slo", "--runs", "--baseline", "--save-baseline", "--current", "--history", "--markdown", "--max-load", "--prepare", "--implementations", "--root", "--file", "--json", "--scenario", "--in-place"],',
 ',\n' + SUB + '\n      ],\n      "flags": ["--slo", "--runs", "--baseline", "--save-baseline", "--current", "--history", "--markdown", "--max-load", "--prepare", "--implementations", "--root", "--file", "--json", "--scenario", "--in-place", "--commands", "--strict"],')
edit(CAT, 'Abnahmeszenario in diesem Checkout aus und schlägt fehl, wenn ein verbindliches Budget verletzt ist." } }',
 'Abnahmeszenario in diesem Checkout aus und schlägt fehl, wenn ein verbindliches Budget verletzt ist." } },\n        { "command": "repo bench latency --commands=inputs,typecheck --runs=3", "description": { "en": "Measure the phases and round trips of two developer commands and warn about every exceeded budget.", "de": "Misst die Phasen und Rundreisen zweier Entwicklerbefehle und warnt vor jedem überschrittenen Budget." } }')
edit(CAT, '    {\n      "name": "--scope",\n',
 '    {\n      "name": "--commands",\n      "value": "required",\n      "valueName": "ids|all",\n      "operations": ["bench"],\n      "description": { "en": "bench latency: comma list of the budgeted developer commands (inputs, run, test-file, typecheck, fleet-status), or all.", "de": "bench latency: Kommaliste der budgetierten Entwicklerbefehle (inputs, run, test-file, typecheck, fleet-status) oder all." }\n    },\n    {\n      "name": "--scope",\n')
edit(CAT, '"operations": ["doctor"],\n      "description": { "en": "doctor: exit with status 1 for warnings as well as errors.", "de": "doctor: beendet sich mit Status 1 auch bei Warnungen, nicht nur bei Fehlern." }',
 '"operations": ["doctor", "bench"],\n      "description": { "en": "doctor, bench latency: exit with status 1 for warnings as well as errors.", "de": "doctor, bench latency: beendet sich mit Status 1 auch bei Warnungen, nicht nur bei Fehlern." }')
edit(CAT, '"en": "bench run: samples per probe instead of the authored count', '"en": "bench run|latency: samples per probe instead of the authored count')
edit(CAT, '"de": "bench run: Stichproben je Messung statt der verfassten Anzahl', '"de": "bench run|latency: Stichproben je Messung statt der verfassten Anzahl')

BC = B + "/📏️incremental/🏎️bench/⌨️command/🟦️.ts"
edit(BC, 'import acceptance from "../🛂️acceptance/🧬️schema/🔣️.json" with {type:"json"};\n',
 'import acceptance from "../🛂️acceptance/🧬️schema/🔣️.json" with {type:"json"};\nimport latency from "../⏲️latency/🧬️schema/🔣️.json" with {type:"json"};\n')
edit(BC, 'const own=acceptance.definitions.Policy.const.options;return {valued:[...valuedOptions,...own.valued],plain:[...plainOptions,...own.plain]};}',
 'const own=acceptance.definitions.Policy.const.options,late=latency.definitions.Policy.const.options;return {valued:[...valuedOptions,...own.valued,...late.valued],plain:[...plainOptions,...own.plain,...late.plain]};}')
edit(BC, ' if(rest[0]==="acceptance"){const{runAcceptanceV1}=await import("../🛂️acceptance/⌨️command/🟦️.ts");await runAcceptanceV1(root,rest.slice(1),invocation);return;}\n',
 ' if(rest[0]==="acceptance"){const{runAcceptanceV1}=await import("../🛂️acceptance/⌨️command/🟦️.ts");await runAcceptanceV1(root,rest.slice(1),invocation);return;}\n if(rest[0]==="latency"){const{runLatencyEntryV1}=await import("../⏲️latency/⌨️command/🟦️.ts");await runLatencyEntryV1(root,rest.slice(1),invocation);return;}\n')
edit(BC, '/** 🍜️ The entry the build CLI calls for `repo bench`; `repo bench acceptance` is the acceptance module\'s own command. */',
 '/** 🍜️ The entry the build CLI calls for `repo bench`; `repo bench acceptance` and `repo bench latency` are their modules\' own commands. */')

BT = B + "/📏️incremental/🏎️bench/🧪️tests/🟦️.ts"
edit(BT, 'expect(entry.subcommands.map((row:any)=>row.name)).toEqual(["list","run","compare","acceptance"]);', 'expect(entry.subcommands.map((row:any)=>row.name)).toEqual(["list","run","compare","acceptance","latency"]);')

LT = B + "/📏️incremental/🏎️bench/⏲️latency/🧪️tests/🟦️.ts"
APPEND = """

test("the operations catalogue declares the latency subcommand with exactly the options its parser accepts",async()=>{
 const catalog=(await import("../../../../⚙️operations/🔣️catalog.json")).default as any,entry=catalog.operations.find((row:any)=>row.name==="bench"),names=latencyOptionNamesV1();
 const sub=entry.subcommands.find((row:any)=>row.name==="latency");
 expect(sub).toBeDefined();expect(sub.synopsis.startsWith("bench latency")).toBe(true);expect(sub.summary.en).not.toBe(sub.summary.de);
 expect([...sub.flags].sort()).toEqual([...names.valued,...names.plain,...names.shared.filter(name=>name!=="--lang")].sort());
 for(const name of [...names.valued,...names.plain]){const flag=catalog.flags.find((row:any)=>row.name===name);expect(flag,name).toBeDefined();expect(flag.operations).toContain("bench");expect(flag.value).toBe(names.valued.includes(name)?"required":"none");expect(flag.description.en).not.toBe(flag.description.de);expect(entry.flags).toContain(name);}
 expect(entry.synopsis).toContain("latency");
 expect(entry.examples.some((row:any)=>row.command.startsWith("repo bench latency"))).toBe(true);
});
"""

def main(mode):
    plan = {}
    for path, old, new, count in edits:
        text = plan.get(path)
        if text is None:
            text = open(path, encoding="utf-8").read()
        found = text.count(old)
        if found != count:
            print("ANCHOR", found, "!=", count, path[-60:], repr(old[:80])); sys.exit(1)
        plan[path] = text.replace(old, new)
    plan[LT] = open(LT, encoding="utf-8").read() + APPEND
    print("anchors ok:", len(edits), "edits in", len(plan), "files")
    if mode == "apply":
        for path, text in plan.items():
            tmp = path + ".h75.tmp"
            open(tmp, "w", encoding="utf-8").write(text)
            os.replace(tmp, path)
        print("applied")

main(sys.argv[1] if len(sys.argv) > 1 else "check")
