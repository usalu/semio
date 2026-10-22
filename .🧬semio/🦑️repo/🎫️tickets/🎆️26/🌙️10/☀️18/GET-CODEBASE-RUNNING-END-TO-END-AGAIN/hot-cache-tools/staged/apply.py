#!/usr/bin/env python3
"""🚚 apply.py: stage the resident repo service (hot-cache, phases 1 to 3) into the repository.

usage: apply.py [--root REPO] [--write] [--no-edits] [--list]
Dry run by default. Refuses on drift: a target that exists with other content, an anchor that is missing or not unique.
Groups whose precondition is not in the tree yet (the Go launcher module, the tool registry) report `waits` and are applied by a later run.
<here>/dev is the ASCII development tree (resident/ and bench/ run as they are; core/ needs the real library and runs in an applied copy, see verify.sh), <here>/edits.json holds every anchored edit as data.
Group 3 (service invocation): dev/core -> B/caller/service and L/laws/service-invocation, edits service-* in edits.json.
"""
import argparse, hashlib, json, os, re, sys

HERE = os.path.dirname(os.path.abspath(__file__))
V = "️"


def e(emoji, name=""):
    return emoji + V + name


L = "/".join([e("\U0001F9F0", "framework"), e("\U0001F6CD", "products"), e("\U0001F991", "repo"), e("\U0001F528", "modules"), e("\U0001F4DA", "library")])
MODULES = "/".join([e("\U0001F9F0", "framework"), e("\U0001F6CD", "products"), e("\U0001F991", "repo"), e("\U0001F528", "modules")])
B = L + "/" + e("\U0001F3D7", "build")
R = B + "/" + e("\U0001F3E1", "resident")
GO = MODULES + "/" + e("\U0001F680", "launcher") + "/" + e("\U0001F4E6", "packages") + "/" + e("\U0001F439", "go")
LAUNCHER = MODULES + "/" + e("\U0001F680", "launcher")
JS, DTS, TS, JSON_, RS = e("\U0001F7E8") + ".mjs", e("\U0001F7E8") + ".d.mts", e("\U0001F7E6") + ".ts", e("\U0001F523") + ".json", e("\U0001F980") + ".rs"
T = e("\U0001F9EA", "tests")
PROTOCOL = e("\U0001F4E8", "protocol")
WATCH = e("\U0001F52D", "watch")
CLIENT = e("\U0001F6AA", "client")
SERVICE = e("\U0001F3DB", "service")
HOOKS = e("\U0001F570", "graph-hooks")
EXECUTOR = e("\U0001F3ED", "executor")


def member(*parts):
    return "/".join(parts)


P = {
    "index.mjs": JS, "index.d.mts": DTS, "schema/schema.json": e("\U0001F9EC", "schema") + "/" + JSON_,
    "protocol/frames.mjs": PROTOCOL + "/" + JS, "protocol/policy.mjs": member(PROTOCOL, e("\U0001F39A", "policy"), JS),
    "protocol/capability.mjs": member(PROTOCOL, e("\U0001F511", "capability"), JS), "protocol/endpoint.mjs": member(PROTOCOL, e("\U0001F4CD", "endpoint"), JS),
    "state/index.mjs": member(e("\U0001F5C3", "state"), JS), "watch/index.mjs": member(WATCH, JS),
    "watch/provision/index.mjs": member(WATCH, e("\U0001F3D7", "provision"), JS), "watch/provision/tools/index.mjs": member(WATCH, e("\U0001F3D7", "provision"), e("\U0001F50C", "tools"), JS),
    "service/index.mjs": member(SERVICE, JS), "service/governor.mjs": member(SERVICE, e("\U0001F6A6", "governor"), JS),
    "client/index.mjs": member(CLIENT, JS), "client/repo.mjs": member(CLIENT, e("\U0001F501", "repo"), JS), "client/main.mjs": member(CLIENT, e("⌨", "main"), JS),
    "delegate/index.mjs": member(e("\U0001F3CA", "delegate"), JS), "delegate/executor.mjs": member(e("\U0001F3CA", "delegate"), EXECUTOR, JS),
    "executor/index.mjs": member(EXECUTOR, JS),
    "ports/graph.mjs": member(e("\U0001F50C", "ports"), e("\U0001F5FA", "graph"), JS), "ports/listing.mjs": member(e("\U0001F50C", "ports"), e("\U0001F4CB", "listing"), JS),
    "graph-hooks/index.mjs": member(HOOKS, JS), "graph-hooks/host/index.mjs": member(HOOKS, e("\U0001F50C", "host"), JS),
    "command/index.mjs": member(e("⌨", "command"), JS), "composition/index.mjs": member(e("\U0001F3D7", "composition"), JS), "composition/warm.json": member(e("\U0001F3D7", "composition"), JSON_),
    "tests/common.ts": member(T, TS), "tests/oracle.py": member(T, e("\U0001F40D") + "oracle.py"),
    "tests/fixtures/vectors.json": member(T, e("\U0001F9EB", "fixtures"), JSON_), "tests/fixtures/eligibility.json": member(T, e("\U0001F9EB", "fixtures"), e("\U0001F4CB") + "eligibility.json"),
    "tests/executor-fixture.mjs": member(T, EXECUTOR, e("\U0001F3D7", "fixture"), JS),
}
for name, emoji in [("client", "\U0001F6AA"), ("protocol", "\U0001F4E8"), ("state", "\U0001F5C3"), ("watch", "\U0001F52D"), ("service", "\U0001F3DB"), ("graph", "\U0001F570"), ("delegate", "\U0001F3CA"), ("command", "⌨"), ("provision", "\U0001F3D7"), ("executor", "\U0001F3ED")]:
    P["tests/%s.test.ts" % name] = member(T, e(emoji, {"graph": "graph-hooks"}.get(name, name)), TS)
for stem in ["protocol/frames", "protocol/policy", "protocol/capability", "protocol/endpoint", "state/index", "watch/index", "watch/provision/index", "watch/provision/tools/index", "service/index", "service/governor", "client/index", "client/repo", "delegate/index", "delegate/executor", "executor/index", "ports/graph", "ports/listing", "graph-hooks/index", "graph-hooks/host/index", "command/index", "composition/index"]:
    P[stem + ".d.mts"] = P[stem + ".mjs"].rsplit("/", 1)[0] + "/" + DTS
M = {"resident/" + k: v for k, v in P.items()}
PIN_TARGET = member(WATCH, e("\U0001F980", "rust"), JSON_)
M["native/watch.rs"] = member(WATCH, e("\U0001F980", "rust"), RS)
M["native/watch.json"] = PIN_TARGET
M["bench/workspace.mjs"] = member(T, e("\U0001F5A5", "fixture-server"), e("\U0001F3D7", "workspace"), JS)
M["bench/ports.mjs"] = member(T, e("\U0001F5A5", "fixture-server"), e("\U0001F50C", "ports"), JS)
M["bench/serve.mjs"] = member(T, e("\U0001F5A5", "fixture-server"), JS)
M["bench/environment.mjs"] = member(T, e("\U0001F5A5", "fixture-server"), e("\U0001F9EA", "environment"), JS)
CALLER = B + "/" + e("\U0001F4E5", "caller")
SVC = CALLER + "/" + e("\U0001F6F0", "service")
LAW = L + "/" + e("\u2696", "laws") + "/" + e("\U0001F6F0", "service-invocation")
M["core/session.mjs"] = member(SVC, e("\U0001F511", "session"), JS)
M["core/session.d.mts"] = member(SVC, e("\U0001F511", "session"), DTS)
M["core/service.ts"] = member(SVC, TS)
M["core/service.schema.json"] = member(SVC, e("\U0001F9EC", "schema"), JSON_)
M["core/tests/service.test.ts"] = member(SVC, T, TS)
M["core/tests/fixtures/admission.json"] = member(SVC, T, JSON_)
M["core/tests/oracle.py"] = member(SVC, T, e("\U0001F40D") + "oracle.py")
M["core/law.ts"] = member(LAW, TS)
M["core/tests/law.test.ts"] = member(LAW, T, TS)
M = {key: (value if key.startswith("core/") else R + "/" + value) for key, value in M.items()}
GENERATED = {"native/watch.json"}

GO_FILES = {
    "go/resident.go": GO + "/" + e("\U0001F6F0", "resident") + "/" + e("\U0001F439") + ".go",
    "go/resident_unix.go": GO + "/" + e("\U0001F6F0", "resident-unix") + "/" + e("\U0001F439") + ".go",
    "go/resident_windows.go": GO + "/" + e("\U0001F6F0", "resident-windows") + "/" + e("\U0001F439") + ".go",
    "go/tests/contract.go": LAUNCHER + "/" + e("\U0001F9EA", "tests") + "/" + e("\U0001F6F0", "resident-contract") + "/" + e("\U0001F439") + ".go",
    "go/tests/contract.feature": LAUNCHER + "/" + e("\U0001F9EA", "tests") + "/" + e("\U0001F6F0", "resident-contract") + "/" + e("\U0001F952") + ".feature",
}
GO_REQUIRES = GO + "/" + e("\U0001F439") + ".go"

REWRITES = {
    "resident/tests/common.ts": [
        ('process.env.SEMIO_REPO_NATIVE_DIR??""', 'process.env.SEMIO_REPO_NATIVE_DIR??""'),
        ('join(HERE,"oracle.py")', 'join(HERE,"%s")' % (e("\U0001F40D") + "oracle.py")),
    ],
    "resident/tests/protocol.test.ts": [('join(HERE,"fixtures/vectors.json")', 'join(HERE,"%s")' % member(e("\U0001F9EB", "fixtures"), JSON_))],
    "resident/tests/client.test.ts": [('join(HERE,"fixtures/eligibility.json")', 'join(HERE,"%s")' % member(e("\U0001F9EB", "fixtures"), e("\U0001F4CB") + "eligibility.json"))],
    "resident/tests/oracle.py": [('os.path.join(HERE, "..", "schema", "schema.json")', 'os.path.join(HERE, "..", "%s", "%s")' % (e("\U0001F9EC", "schema"), JSON_))],
    "resident/tests/graph.test.ts": [('join(workspaceRoot(),"' + B + "/" + e("\U0001F578", "graph") + "/" + e("\U0001F570", "generations") + "/" + JS + '")', 'new URL("../../../%s/%s/%s",import.meta.url).pathname' % (e("\U0001F578", "graph"), e("\U0001F570", "generations"), JS))],
    "resident/tests/service.test.ts": [('join(import.meta.dir,"..","..","bench","serve.mjs")', 'join(import.meta.dir,"..","%s","%s")' % (e("\U0001F5A5", "fixture-server"), JS)), ('join(WATCHER,"..")', 'process.env.SEMIO_REPO_NATIVE_DIR??""'), ('join(import.meta.dir,"..","client","main.mjs")', 'join(import.meta.dir,"..","..","%s","%s","%s")' % (CLIENT, e("⌨", "main"), JS))],
    "resident/tests/executor.test.ts": [('join(import.meta.dir,"executor-fixture.mjs")', 'join(import.meta.dir,"%s","%s")' % (e("\U0001F3D7", "fixture"), JS))],
}

SPEC = re.compile(r'(["\'])(\.{1,2}/[^"\'\n$\{]+)\1')


def rel_import(src_target, dst_target):
    r = os.path.relpath(dst_target, os.path.dirname(src_target))
    return r if r.startswith(".") else "./" + r


def rewrite(dev_path, text):
    if not dev_path.endswith((".mjs", ".ts", ".d.mts")):
        return text
    src_target, base = M[dev_path], os.path.dirname(dev_path)

    def sub(m):
        spec = m.group(2)
        resolved = os.path.normpath(os.path.join(base, spec))
        if resolved in M:
            return m.group(1) + rel_import(src_target, M[resolved]) + m.group(1)
        if os.path.exists(os.path.join(HERE, "dev", resolved)):
            raise SystemExit("unmapped import %s in %s" % (spec, dev_path))
        return m.group(0)

    return SPEC.sub(sub, text)


def generate(dev_path):
    if dev_path == "native/watch.json":
        source = open(os.path.join(HERE, "dev", "native", "watch.rs"), "rb").read()
        return json.dumps({"version": "1", "sources": [{"platform": "any", "architecture": "any", "sha256": hashlib.sha256(source).hexdigest(), "source": e("\U0001F980") + ".rs", "bytes": len(source)}]}, ensure_ascii=False, indent=1) + "\n"
    raise KeyError(dev_path)


def read_text(path):
    with open(path, encoding="utf-8") as handle:
        return handle.read()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--root", default=os.getcwd())
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--no-edits", action="store_true")
    ap.add_argument("--list", action="store_true")
    ap.add_argument("--skip-missing", action="store_true", help="edits whose target file is absent wait instead of failing (scratch roots)")
    ns = ap.parse_args()
    root = os.path.abspath(ns.root)
    dev = os.path.join(HERE, "dev")
    problems, waits, plan = [], [], []
    launcher_present = os.path.exists(os.path.join(root, GO_REQUIRES))

    def target_row(dev_path, target, text):
        full = os.path.join(root, target)
        units = len((root + "/" + target).encode("utf-16-le")) // 2
        if units > 256:
            problems.append("path too long (%d): %s" % (units, target))
        action = "create"
        if os.path.exists(full):
            action = "same" if read_text(full) == text else "DRIFT"
            if action == "DRIFT":
                problems.append("drift: " + target)
        plan.append((action, target, text))

    for dev_path, target in sorted(M.items()):
        if dev_path in GENERATED:
            text = generate(dev_path)
        else:
            src = os.path.join(dev, dev_path)
            if not os.path.exists(src):
                problems.append("missing dev file " + dev_path)
                continue
            text = read_text(src)
            for old, new in REWRITES.get(dev_path, []):
                if old not in text:
                    if old != new:
                        problems.append("rewrite anchor missing in %s: %s" % (dev_path, old[:60]))
                    continue
                text = text.replace(old, new)
            text = rewrite(dev_path, text)
        target_row(dev_path, target, text)
    for dev_path, target in sorted(GO_FILES.items()):
        if not launcher_present:
            waits.append("launcher module (" + os.path.basename(target) + ")")
            continue
        src = os.path.join(dev, dev_path)
        if not os.path.exists(src):
            problems.append("missing dev file " + dev_path)
            continue
        target_row(dev_path, target, read_text(src))
    edits = []
    if not ns.no_edits:
        for edit in json.load(open(os.path.join(HERE, "edits.json"), encoding="utf-8")):
            if "requires" in edit and not os.path.exists(os.path.join(root, edit["requires"])):
                waits.append("edit " + edit["id"] + " waits for " + os.path.basename(os.path.dirname(edit["requires"])))
                continue
            full = os.path.join(root, edit["file"])
            if not os.path.exists(full):
                if ns.skip_missing:
                    waits.append("edit " + edit["id"] + " (file absent)")
                else:
                    problems.append("edit target missing: " + edit["file"])
                continue
            text = read_text(full)
            if edit["mode"] == "replace" and edit["text"] in text and edit["anchor"] not in text.replace(edit["text"], ""):
                edits.append(("same", edit))
                continue
            if edit["mode"] != "replace" and edit["text"] in text:
                edits.append(("same", edit))
                continue
            if text.count(edit["anchor"]) != 1:
                problems.append("anchor not unique/missing (%d) for %s in %s" % (text.count(edit["anchor"]), edit["id"], edit["file"]))
                continue
            edits.append(("edit", edit))
    for action, target, _ in plan:
        if ns.list or action != "same":
            print("%-7s %s" % (action, target.replace(L + "/", "L/").replace(MODULES + "/", "M/")))
    for action, edit in edits:
        print("%-7s edit %s" % (action, edit["id"]))
    for wait in waits:
        print("waits   " + wait)
    longest = max((len((root + "/" + t).encode("utf-16-le")) // 2 for _, t, _ in plan), default=0)
    print("%d files, %d edits, %d waiting, longest path %d UTF-16 units, %d problems" % (len(plan), len(edits), len(waits), longest, len(problems)))
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
    for action, edit in edits:
        if action == "edit":
            by_file.setdefault(edit["file"], []).append(edit)
    for file, items in by_file.items():
        full = os.path.join(root, file)
        text = read_text(full)
        for edit in items:
            if edit["mode"] == "replace":
                text = text.replace(edit["anchor"], edit["text"])
            elif edit["mode"] == "insert_after":
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
