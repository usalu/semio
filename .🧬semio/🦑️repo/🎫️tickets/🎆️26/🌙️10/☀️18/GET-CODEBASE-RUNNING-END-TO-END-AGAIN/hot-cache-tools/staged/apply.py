#!/usr/bin/env python3
"""apply.py: stage the resident repo service into the repository.

usage: apply.py [--root REPO] [--write] [--no-edits] [--list]
Dry run by default. Refuses on drift: a target that exists with other content, an anchor that is missing or not unique.
Layout: <here>/dev holds the ASCII development tree (runnable as is), <here>/plan.json maps it to the emoji taxonomy tree.
"""
import argparse, json, os, re, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
V = "️"


def e(emoji, name=""):
    return emoji + V + name


L = "/".join([e("\U0001F9F0", "framework"), e("\U0001F6CD", "products"), e("\U0001F991", "repo"), e("\U0001F528", "modules"), e("\U0001F4DA", "library")])
B = L + "/" + e("\U0001F3D7", "build")
R = B + "/" + e("\U0001F3E1", "resident")
JS = e("\U0001F7E8") + ".mjs"
DTS = e("\U0001F7E8") + ".d.mts"
TS = e("\U0001F7E6") + ".ts"
JSON_ = e("\U0001F523") + ".json"
RS = e("\U0001F980") + ".rs"
T = e("\U0001F9EA", "tests")

P = {
    "index.mjs": JS, "schema/schema.json": e("\U0001F9EC", "schema") + "/" + JSON_,
    "protocol/frames.mjs": e("\U0001F4E8", "protocol") + "/" + JS,
    "protocol/policy.mjs": e("\U0001F4E8", "protocol") + "/" + e("\U0001F39A", "policy") + "/" + JS,
    "protocol/capability.mjs": e("\U0001F4E8", "protocol") + "/" + e("\U0001F511", "capability") + "/" + JS,
    "protocol/endpoint.mjs": e("\U0001F4E8", "protocol") + "/" + e("\U0001F4CD", "endpoint") + "/" + JS,
    "state/index.mjs": e("\U0001F5C3", "state") + "/" + JS,
    "watch/index.mjs": e("\U0001F52D", "watch") + "/" + JS,
    "service/index.mjs": e("\U0001F3DB", "service") + "/" + JS,
    "service/governor.mjs": e("\U0001F3DB", "service") + "/" + e("\U0001F6A6", "governor") + "/" + JS,
    "client/index.mjs": e("\U0001F6AA", "client") + "/" + JS,
    "client/repo.mjs": e("\U0001F6AA", "client") + "/" + e("\U0001F501", "repo") + "/" + JS,
    "client/main.mjs": e("\U0001F6AA", "client") + "/" + e("\u2328", "main") + "/" + JS,
    "delegate/index.mjs": e("\U0001F3CA", "delegate") + "/" + JS,
    "ports/graph.mjs": e("\U0001F50C", "ports") + "/" + e("\U0001F5FA", "graph") + "/" + JS,
    "ports/listing.mjs": e("\U0001F50C", "ports") + "/" + e("\U0001F4CB", "listing") + "/" + JS,
    "graph-hooks/index.mjs": e("\U0001F570", "graph-hooks") + "/" + JS,
    "command/index.mjs": e("\u2328", "command") + "/" + JS,
    "composition/index.mjs": e("\U0001F3D7", "composition") + "/" + JS,
    "composition/warm.json": e("\U0001F3D7", "composition") + "/" + JSON_,
    "tests/common.ts": T + "/" + TS,
    "tests/oracle.py": T + "/" + e("\U0001F40D") + "oracle.py",
    "tests/fixtures/vectors.json": T + "/" + e("\U0001F9EB", "fixtures") + "/" + JSON_,
}
for name, member in [("client", "\U0001F6AA"), ("protocol", "\U0001F4E8"), ("state", "\U0001F5C3"), ("watch", "\U0001F52D"), ("service", "\U0001F3DB"), ("graph", "\U0001F570"), ("delegate", "\U0001F3CA"), ("command", "\u2328")]:
    P["tests/%s.test.ts" % name] = T + "/" + e(member, {"graph": "graph-hooks"}.get(name, name)) + "/" + TS
for stem in ["protocol/frames", "protocol/policy", "protocol/capability", "protocol/endpoint", "state/index", "watch/index", "service/index", "service/governor", "client/index", "client/repo", "delegate/index", "ports/graph", "ports/listing", "graph-hooks/index", "command/index", "composition/index"]:
    P[stem + ".d.mts"] = P[stem + ".mjs"].rsplit("/", 1)[0] + "/" + DTS
P["index.d.mts"] = DTS
M = {"resident/" + k: v for k, v in P.items()}
M["native/watch.rs"] = e("\U0001F52D", "watch") + "/" + e("\U0001F980", "rust") + "/" + RS
M["native/forward.rs"] = e("\U0001F6AA", "client") + "/" + e("\U0001F980", "rust") + "/" + RS
M["bench/workspace.mjs"] = T + "/" + e("\U0001F5A5", "fixture-server") + "/" + e("\U0001F3D7", "workspace") + "/" + JS
M["bench/ports.mjs"] = T + "/" + e("\U0001F5A5", "fixture-server") + "/" + e("\U0001F50C", "ports") + "/" + JS
M["bench/serve.mjs"] = T + "/" + e("\U0001F5A5", "fixture-server") + "/" + JS

FIXTURES_PATH = e("\U0001F9EB", "fixtures") + "/" + JSON_
ORACLE_NAME = e("\U0001F40D") + "oracle.py"
GENERATIONS = e("\U0001F578", "graph") + "/" + e("\U0001F570", "generations") + "/" + JS
REWRITES = {
    "resident/tests/common.ts": [
        ('process.env.SEMIO_REPO_WATCH_BINARY??"/private/tmp/hot-cache/bin/semio-repo-watch"', 'process.env.SEMIO_REPO_WATCH_BINARY??""'),
        ('process.env.SEMIO_REPO_FORWARD_BINARY??"/private/tmp/hot-cache/bin/semio-repo-forward"', 'process.env.SEMIO_REPO_FORWARD_BINARY??""'),
        ('process.env.SEMIO_REPO_FAKETOOL_BINARY??"/private/tmp/hot-cache/bin/faketool"', 'process.env.SEMIO_REPO_FAKETOOL_BINARY??""'),
        ('join(HERE,"oracle.py")', 'join(HERE,"%s")' % ORACLE_NAME),
    ],
    "resident/tests/protocol.test.ts": [('join(HERE,"fixtures/vectors.json")', 'join(HERE,"%s")' % FIXTURES_PATH)],
    "resident/tests/oracle.py": [('os.path.join(HERE, "..", "schema", "schema.json")', 'os.path.join(HERE, "..", "%s", "%s")' % (e("\U0001F9EC", "schema"), JSON_))],
    "resident/tests/graph.test.ts": [('process.env.SEMIO_GENERATIONS_MODULE??"/Users/ueli/Documents/semio/' + B + '/' + GENERATIONS.replace(e("\U0001F578", "graph") + "/", e("\U0001F578", "graph") + "/") + '"', 'process.env.SEMIO_GENERATIONS_MODULE??new URL("../../../%s",import.meta.url).pathname' % GENERATIONS)],
    "resident/tests/service.test.ts": [('join(import.meta.dir,"..","..","bench","serve.mjs")', 'join(import.meta.dir,"..","%s","%s")' % (e("\U0001F5A5", "fixture-server"), JS)), ('join(WATCHER,"..")', 'process.env.SEMIO_REPO_NATIVE_DIR??""'), ('join(import.meta.dir,"..","client","main.mjs")', 'join(import.meta.dir,"..","..","%s","%s","%s")' % (e("\U0001F6AA", "client"), e("\u2328", "main"), JS))],
    "bench/serve.mjs": [('flag("--bin","/private/tmp/hot-cache/bin")', 'flag("--bin",process.env.SEMIO_REPO_NATIVE_DIR??"")'), ('"/private/tmp/hot-cache/stamps"', 'tmpdir()+"/semio-resident-stamps"'), ('import {realpathSync} from "node:fs";', 'import {realpathSync} from "node:fs";\nimport {tmpdir} from "node:os";')],
}

SPEC = re.compile(r'(["\'])(\.{1,2}/[^"\'\n$\{]+)\1')


def rel_import(src_target, dst_target):
    a = os.path.dirname(src_target)
    r = os.path.relpath(dst_target, a)
    return r if r.startswith(".") else "./" + r


def rewrite(dev_path, text):
    if not dev_path.endswith((".mjs", ".ts", ".d.mts")):
        return text
    src_target = M[dev_path]
    base = os.path.dirname(dev_path)

    def sub(m):
        spec = m.group(2)
        resolved = os.path.normpath(os.path.join(base, spec))
        if resolved in M:
            return m.group(1) + rel_import(src_target, M[resolved]) + m.group(1)
        if os.path.exists(os.path.join(HERE, "dev", resolved)):
            raise SystemExit("unmapped import %s in %s" % (spec, dev_path))
        return m.group(0)

    return SPEC.sub(sub, text)


EDITS = [
    {
        "id": "script-hook",
        "file": B + "/" + e("\U0001F4DC") + "script.ts",
        "anchor": ' if(selected.args[0]==="concurrency"){const{runConcurrencyV1}=await import("./' + e("\U0001F4CF") + "incremental/" + e("\U0001F91D") + "concurrent/" + e("⌨") + "command/" + TS + '");await runConcurrencyV1(root,selected.args,parent);return;}\n',
        "insert_after": True,
        "text": ' if(selected.args[0]==="service"){const{resolveLanguageV1}=await import("../../../../../' + e("\U0001F528") + "modules/" + e("\U0001F3C3") + "process/" + e("\U0001F3D7") + "build/" + e("\U0001F5E3") + "language/" + TS + '"),{runResidentCommandV1}=await import("./' + e("\U0001F3E1") + "resident/" + e("⌨") + "command/" + JS + '"),{createForegroundRunnerV1}=await import("./' + e("\U0001F3E1") + "resident/" + e("\U0001F3D7") + "composition/" + JS + '"),{loadBuildInvocationGraphV1}=await import("./' + e("\U0001F9ED") + "invocation/" + TS + '"),{fileURLToPath}=await import("node:url"),{createHash}=await import("node:crypto"),{readFileSync}=await import("node:fs");const language=resolveLanguageV1(["en","de"],selected.args,process.env).language,script=fileURLToPath(import.meta.url),library=resolve(dirname(script),".."),warm=JSON.parse(readFileSync(new URL("./' + e("\U0001F3E1") + "resident/" + e("\U0001F3D7") + "composition/" + JSON_ + '",import.meta.url),"utf8")).modules.map((module:string)=>join(library,module));process.exitCode=await runResidentCommandV1({root,args:selected.args.slice(1),languages:language==="de"?["de","en"]:["en","de"],environment:process.env,scriptArguments:[script,"repo"],runForeground:createForegroundRunnerV1({loadGraph:()=>loadBuildInvocationGraphV1(root,["graph","--graph-policy=current"],parent,parent,phaseControl,true),script,warmModules:warm,watcherBinary:process.env.SEMIO_REPO_WATCH_BINARY,salt:()=>createHash("sha256").update(JSON.stringify([process.env.SEMIO_BUILD_TOOL??"",(parent.capabilities as {tool?:string}).tool??""])).digest("hex"),environmentPaths:["package.json","bun.lock","nx.json","tsconfig.json","MODULE.bazel","Cargo.toml","Cargo.lock","rust-toolchain.toml","go.work","bunfig.toml"]})});return;}\n',
    },
    {"id": "gitignore", "file": ".gitignore", "anchor": ".\U0001F9EC" + "semio/" + e("\U0001F991", "repo") + "/" + e("⚡", "cache") + "/\n", "insert_after": True, "text": ".\U0001F9EC" + "semio/" + e("\U0001F991", "repo") + "/" + e("\U0001F3E1", "resident-fence") + "/\n"},
]
CATALOG = B + "/" + e("⚙", "operations") + "/" + JSON_.replace(e("\U0001F523"), e("\U0001F523")).replace(".json", "") + "catalog.json"
CATALOG = B + "/" + e("⚙", "operations") + "/" + e("\U0001F523") + "catalog.json"
OPERATION = open(os.path.join(HERE, "catalog-operation.json.txt"), encoding="utf-8").read() if os.path.exists(os.path.join(HERE, "catalog-operation.json.txt")) else ""
FLAGS = open(os.path.join(HERE, "catalog-flags.json.txt"), encoding="utf-8").read() if os.path.exists(os.path.join(HERE, "catalog-flags.json.txt")) else ""
EDITS += [
    {"id": "catalog-operation", "file": CATALOG, "anchor": '    {\n      "name": "remote-execution",\n', "insert_before": True, "text": OPERATION},
    {"id": "catalog-flags", "file": CATALOG, "anchor": '    { "name": "--help", "aliases": ["-h"],', "insert_before": True, "text": FLAGS},
    {"id": "catalog-schema-operation", "file": B + "/" + e("\u2699", "operations") + "/" + e("\U0001F9EC", "schema") + "/" + e("\U0001F523") + ".json", "anchor": '"remote-execution"]\n    },\n    "Operation": {', "replace": '"remote-execution", "service"]\n    },\n    "Operation": {'},
    {"id": "catalog-json-flag", "file": CATALOG, "anchor": '"operations": ["graph", "show", "incremental", "concurrency", "capacity", "leases", "cache"]', "replace": '"operations": ["graph", "show", "incremental", "concurrency", "capacity", "leases", "cache", "service"]'},
]


EDITS += [
    {
        "id": "repo-script-forward",
        "file": B + "/" + e("\U0001F4DC") + "script.ts",
        "anchor": '  async run(args:string[]):Promise<void> { await runOwnedBuildInvocationV1(this.repoRoot,args,this.invocation,createIssuedResourceControlV1(this.invocation,"repository:command",0)); }\n',
        "replace": '  async run(args:string[]):Promise<void> { const{forwardRepoCommandV1}=await import("./' + e("\U0001F3E1") + "resident/" + e("\U0001F6AA") + "client/" + e("\U0001F501") + "repo/" + JS + '"),{resolveLanguageV1}=await import("../../../../../' + e("\U0001F528") + "modules/" + e("\U0001F3C3") + "process/" + e("\U0001F3D7") + "build/" + e("\U0001F5E3") + "language/" + TS + '");const forwarded=await forwardRepoCommandV1({root:this.repoRoot,args,environment:process.env,languageOf:()=>resolveLanguageV1(["en","de"],args,process.env).language});if(forwarded!==undefined){process.exitCode=forwarded;return;} await runOwnedBuildInvocationV1(this.repoRoot,args,this.invocation,createIssuedResourceControlV1(this.invocation,"repository:command",0)); }\n',
    },
    {
        "id": "invocation-strips-service",
        "file": B + "/" + e("\U0001F9ED") + "invocation/" + TS,
        "anchor": '    if(argument.startsWith("--workers=")){workers=argument.slice(10);continue;}\n',
        "insert_after": True,
        "text": '    if(argument==="--service"){index++;continue;}\n    if(argument.startsWith("--service=")){continue;}\n',
    },
]


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=os.getcwd())
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--no-edits", action="store_true")
    ap.add_argument("--list", action="store_true")
    ns = ap.parse_args()
    root = os.path.abspath(ns.root)
    dev = os.path.join(HERE, "dev")
    problems, plan = [], []
    for dev_path, target in sorted(M.items()):
        src = os.path.join(dev, dev_path)
        if not os.path.exists(src):
            problems.append("missing dev file " + dev_path)
            continue
        text = open(src, encoding="utf-8").read()
        for old, new in REWRITES.get(dev_path, []):
            if old not in text:
                if old != new:
                    problems.append("rewrite anchor missing in %s: %s" % (dev_path, old[:60]))
                continue
            text = text.replace(old, new)
        text = rewrite(dev_path, text)
        target = R + "/" + target
        full = os.path.join(root, target)
        units = len((root + "/" + target).encode("utf-16-le")) // 2
        if units > 256:
            problems.append("path too long (%d): %s" % (units, target))
        action = "create"
        if os.path.exists(full):
            action = "same" if open(full, encoding="utf-8").read() == text else "DRIFT"
            if action == "DRIFT":
                problems.append("drift: " + target)
        plan.append((action, target, text))
    edits = []
    if not ns.no_edits:
        for edit in EDITS:
            full = os.path.join(root, edit["file"])
            if not os.path.exists(full):
                problems.append("edit target missing: " + edit["file"])
                continue
            text = open(full, encoding="utf-8").read()
            marker = edit.get("text", edit.get("replace", ""))
            if marker and marker in text:
                edits.append(("same", edit, text))
                continue
            if text.count(edit["anchor"]) != 1:
                problems.append("anchor not unique/missing (%d) for %s in %s" % (text.count(edit["anchor"]), edit["id"], edit["file"]))
                continue
            if "replace" in edit:
                new = text.replace(edit["anchor"], edit["replace"])
            elif edit.get("insert_after"):
                new = text.replace(edit["anchor"], edit["anchor"] + edit["text"])
            else:
                new = text.replace(edit["anchor"], edit["text"] + edit["anchor"])
            edits.append(("edit", edit, new))
    for action, target, _ in plan:
        if ns.list or action != "same":
            print("%-7s %s" % (action, target.replace(L + "/", "L/")))
    for action, edit, _ in edits:
        print("%-7s edit %s" % (action, edit["id"]))
    longest = max((len((root + "/" + t).encode("utf-16-le")) // 2 for _, t, _ in plan), default=0)
    print("%d files, %d edits, longest path %d UTF-16 units, %d problems" % (len(plan), len(edits), longest, len(problems)))
    for p in problems:
        print("PROBLEM", p)
    if problems:
        sys.exit(2)
    if not ns.write:
        print("dry run; pass --write to apply")
        return
    for action, target, text in plan:
        if action == "create":
            full = os.path.join(root, target)
            os.makedirs(os.path.dirname(full), exist_ok=True)
            with open(full, "w", encoding="utf-8") as handle:
                handle.write(text)
            os.chmod(full, 0o644)
    by_file = {}
    for action, edit, new in edits:
        if action == "edit":
            by_file.setdefault(edit["file"], []).append(edit)
    for file, items in by_file.items():
        full = os.path.join(root, file)
        text = open(full, encoding="utf-8").read()
        for edit in items:
            if "replace" in edit:
                text = text.replace(edit["anchor"], edit["replace"])
            elif edit.get("insert_after"):
                text = text.replace(edit["anchor"], edit["anchor"] + edit["text"])
            else:
                text = text.replace(edit["anchor"], edit["text"] + edit["anchor"])
        mode = os.stat(full).st_mode & 0o777
        with open(full, "w", encoding="utf-8") as handle:
            handle.write(text)
        os.chmod(full, mode)
    print("written")


if __name__ == "__main__":
    main()
