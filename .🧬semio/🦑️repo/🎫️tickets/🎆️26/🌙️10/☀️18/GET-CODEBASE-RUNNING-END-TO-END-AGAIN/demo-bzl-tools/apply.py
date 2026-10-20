#!/usr/bin/env python3
"""Applies the demo-bzl change set (anchored block edits, no source copies).

usage: apply.py [--root R] [--group demonstrator|bericht|all]            dry run (default): prints every edit it would make
       apply.py --write [--root R] [--group G] [--accept-drift]          applies all-or-nothing (re-reads each file, refuses on drift)
       apply.py --bases                                                  records the sha256 of every edited file of the live checkout
       apply.py --status [--root R]                                      per file: pending | applied | drift

A file whose edits are all present is `applied` and left alone. A file whose content differs from bases.json is `drift`
and refused unless --accept-drift (every anchor must still occur exactly once)."""
import hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
def _root():
    path = HERE
    while path != os.path.dirname(path):
        if os.path.isfile(os.path.join(path, "bunfig.toml")) and os.path.isfile(os.path.join(path, "bun.lock")):
            return path
        path = os.path.dirname(path)
    raise SystemExit("repository root not found")


LIVE = _root()
D = "♻️mit-bestand/🧺️demonstrator"
DEMO = D + "/📋️project.json"
RUNTIME = D + "/🔨️modules/🧩️runtime/📜️script.ts"
RTEST = D + "/🔨️modules/🧩️runtime/🧪️tests/🟦️.ts"
README = D + "/README.md"
DECK = "♻️mit-bestand/🎤️präsentation/📅️33.projektetage/📦️packages/🟦️typescript/📋️project.json"
BERICHT = "♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📋️project.json"
DRIVER = '"cache": { "version": 1, "class": "driver-owned", "reason": "freshness-owned-by-driver" }'
RESTORE = '      "inputs": [\n        {\n          "dependentTasksOutputFiles": "**/*",\n          "transitive": true\n        }\n      ],\n'


def decision(klass, reason):
    return '"cache": { "version": 1, "class": "%s", "reason": "%s" }' % (klass, reason)


def block(text, name):
    m = re.search(r'\n    "%s": \{\n' % re.escape(name), text)
    if not m:
        raise SystemExit("target block missing: " + name)
    end = re.search(r'\n    \},?\n', text[m.end():])
    return m.start() + 1, m.end() + end.end()


def sub_in_block(text, name, old, new):
    a, b = block(text, name)
    body = text[a:b]
    if body.count(new) and not body.count(old):
        return text
    if body.count(old) != 1:
        raise SystemExit("anchor %r occurs %d times in block %s" % (old, body.count(old), name))
    return text[:a] + body.replace(old, new) + text[b:]


def sub_once(text, old, new):
    if text.count(new) and not text.count(old):
        return text
    if text.count(old) != 1:
        raise SystemExit("anchor %r occurs %d times" % (old[:90], text.count(old)))
    return text.replace(old, new)


def demonstrator(text):
    for name, reason in (("prepare-prototype", "check-without-outputs"), ("prepare-release", "check-without-outputs"), ("activate-prototype", "pure-function-of-inputs")):
        text = sub_in_block(text, name, '"cache": false,', '"cache": true,')
        text = sub_in_block(text, name, DRIVER, decision("deterministic", reason))
    a, b = block(text, "activate-lanes-prototype")
    lanes = re.findall(r'"(@semio-tech/framework-os-dev:activate-[a-z0-9-]+-react-prototype)"', text[a:b])
    if len(lanes) != 3 and '"repo:noop"' not in text[a:b]:
        raise SystemExit("activate-lanes-prototype: expected three lane dependencies")
    if '"repo:noop"' not in text[a:b]:
        new = '    "activate-lanes-prototype": {\n      "executor": "repo:noop",\n      "dependsOn": [\n' + ",\n".join('        "%s"' % lane for lane in lanes) + '\n      ],\n      "metadata": {\n        "semio": {\n          "cache": { "version": 1, "class": "aggregate", "reason": "orders-prerequisites" }\n        }\n      }\n    },\n'
        text = text[:a] + new + text[b:]
    for name in ("serve", "dev", "prepare-e2e", "serve-e2e"):
        a, b = block(text, name)
        if "dependentTasksOutputFiles" not in text[a:b]:
            text = sub_in_block(text, name, '      "executor": "repo:script",\n', '      "executor": "repo:script",\n' + RESTORE)
    for name in ("serve", "dev", "serve-e2e"):
        a, b = block(text, name)
        if '"class": "continuous"' not in text[a:b]:
            text = sub_in_block(text, name, '        "semio": {\n', '        "semio": {\n          ' + decision("continuous", "serves-until-stopped") + ',\n')
    json.loads(text)
    return text


def runtime(text):
    text = sub_once(text, 'import { BundleScript, checkScriptInvocation, ScriptRouter }', 'import { BundleScript, ScriptRouter }')
    text = sub_once(text, '/** 🏁️ Completes the selected component lane prerequisites owned by the build tool. */\nclass CompleteScript extends BundleScript {\n  run(args: string[]): void {\n    if (args.length) throw new Error("Runtime lane completion accepts no arguments");\n    checkScriptInvocation(this.invocation);\n    console.log(`Prepared Demonstrator component lanes: ${readBuildMode(process.env.SEMIO_BUILD_MODE)}`);\n  }\n}\n\n', '')
    return sub_once(text, '.register("prepare", PreparationScript).register("complete", CompleteScript).register("activate"', '.register("prepare", PreparationScript).register("activate"')


def rtest(text):
    anchor = '  for (const name of ["serve", "dev"]) {\n'
    add = ('  assert.equal(project.targets["activate-prototype"]?.cache, true, "the union receipt is a pure function of the lane receipts");\n'
           '  assert.equal(project.targets["activate-lanes-prototype"]?.executor, "repo:noop", "the lane gate is an ordering node without a process");\n'
           '  for (const name of ["serve", "dev", "prepare-e2e", "serve-e2e"]) assert.ok(project.targets[name]?.inputs?.some((input: any) => input?.dependentTasksOutputFiles === "**/*" && input.transitive === true), `${name} restores the transitive closure it serves from`);\n')
    return text if add in text else sub_once(text, anchor, add + anchor)


def readme(text):
    text = sub_once(text, "after `activate-energy-react-prototype`, `activate-fem3d-react-prototype` and `activate-generator-react-prototype`; only logs that the lanes are complete | yes |", "after `activate-energy-react-prototype`, `activate-fem3d-react-prototype` and `activate-generator-react-prototype`; a pure ordering node (`repo:noop`, no process) | not applicable |")
    text = sub_once(text, "`build` is not cached: it stages into the immutable products store rather than into a declared output tree.", "`build` is not cached: it stages into the immutable products store rather than into a declared output tree, and it bundles map tiles fetched from the network (`prefetchMapTiles`), which no declared input pins.")
    text = sub_once(text, "nach `activate-energy-react-prototype`, `activate-fem3d-react-prototype` und `activate-generator-react-prototype`; meldet nur, dass die Lanes vollständig sind | ja |", "nach `activate-energy-react-prototype`, `activate-fem3d-react-prototype` und `activate-generator-react-prototype`; ein reiner Ordnungsknoten (`repo:noop`, kein Prozess) | nicht anwendbar |")
    return sub_once(text, "`build` ist nicht gecacht: Es legt in den unveränderlichen Produkt-Store ab, nicht in einen deklarierten Ausgabebaum.", "`build` ist nicht gecacht: Es legt in den unveränderlichen Produkt-Store ab, nicht in einen deklarierten Ausgabebaum, und bündelt Kartenkacheln, die aus dem Netz geladen werden (`prefetchMapTiles`) und die keine deklarierte Eingabe festschreibt.")


def bericht(text):
    for document in ("zwischenbericht", "forschungsbericht", "kompaktbericht"):
        text = sub_once(text, '"{projectRoot}/dist/documents/%s"' % document, '"{workspaceRoot}/.cache/b/p/m/print/{options.env.SEMIO_BUILD_MODE}/documents/report/%s"' % document)
    json.loads(text)
    return text


def deck(text):
    a, b = block(text, "dev")
    if '"class": "continuous"' not in text[a:b]:
        text = sub_in_block(text, "dev", '        "semio": {\n', '        "semio": {\n          ' + decision("continuous", "serves-until-stopped") + ',\n')
    json.loads(text)
    return text


EDITS = [("demonstrator", DEMO, demonstrator), ("demonstrator", RUNTIME, runtime), ("demonstrator", RTEST, rtest), ("demonstrator", README, readme), ("bericht", BERICHT, bericht), ("deck", DECK, deck)]


def sha(data):
    return hashlib.sha256(data).hexdigest()


def main(argv):
    root = LIVE
    if "--root" in argv:
        root = os.path.abspath(argv[argv.index("--root") + 1])
    group = argv[argv.index("--group") + 1] if "--group" in argv else "all"
    bases_file = os.path.join(HERE, "bases.json")
    if "--bases" in argv:
        data = {path: sha(open(os.path.join(root, path), "rb").read()) for _, path, _ in EDITS}
        json.dump(data, open(bases_file, "w"), indent=1, ensure_ascii=False)
        print("recorded", len(data), "bases")
        return 0
    bases = json.load(open(bases_file)) if os.path.exists(bases_file) else {}
    write, drift_ok, status = "--write" in argv, "--accept-drift" in argv, "--status" in argv
    plan, problems = {}, []
    for name, path, edit in EDITS:
        if group not in ("all", name):
            continue
        raw = open(os.path.join(root, path), "rb").read()
        text = raw.decode("utf8")
        try:
            new = edit(text)
        except SystemExit as error:
            problems.append((path, str(error)))
            continue
        state = "applied" if new == text else "pending"
        if state == "pending" and bases.get(path) and bases[path] != sha(raw) and not drift_ok:
            state = "drift"
            problems.append((path, "drift: base %s now %s (anchors hold; rerun with --accept-drift)" % (bases[path][:12], sha(raw)[:12])))
        print("%-8s %s  %d -> %d bytes" % (state, path.split("/")[-1] if "/" in path else path, len(raw), len(new.encode())))
        if state == "pending":
            plan[path] = (raw, new)
    if problems:
        for path, reason in problems:
            print("REFUSED", path, reason)
        return 3
    if status or not write:
        print("dry run: %d file(s) would change" % len(plan) if not status else "")
        return 0
    for path, (raw, new) in plan.items():
        full = os.path.join(root, path)
        if open(full, "rb").read() != raw:
            print("REFUSED", path, "changed while planning")
            return 4
    for path, (raw, new) in plan.items():
        full = os.path.join(root, path)
        temporary = full + ".demo-bzl~"
        open(temporary, "w", encoding="utf8").write(new)
        os.replace(temporary, full)
    print("written", len(plan), "file(s)")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
