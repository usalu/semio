#!/usr/bin/env python3
# 🚚 Applies the staged worker pool set: copies each staged file to its repository path, refusing when an existing target drifted from the content the staging was built against.
# usage: apply.py [--dry-run | --write] [--root DIR] [--record] [--force]   (--force skips the drift check and is only for a scratch overlay root)
import json, os, sys
import lib

W, P, B, Z = lib.WORKERS, lib.PROTOCOL, lib.BUILD, lib.BAZEL
E_TESTS = lib.TESTS + "/" + lib.TS
E_FIX = lib.TESTS + "/" + lib.FIXTURES + "/" + lib.JSON_NAME
SCHEMA = lib.SCHEMA + "/" + lib.JSON_NAME
TARGETS = [
    ("workers-schema.json", W + "/" + SCHEMA),
    ("workers.ts", W + "/" + lib.TS),
    ("workers-protocol.ts", P + "/" + lib.TS),
    ("workers-test.ts", W + "/" + E_TESTS),
    ("workers-fixture.json", W + "/" + E_FIX),
    ("policy.json", B + "/\U0001F523️policy.json"),
    ("catalog.json", B + "/⚙️operations/\U0001F523️catalog.json"),
    ("help-fixture.json", B + "/⚙️operations/" + lib.TESTS + "/" + lib.FIXTURES + "/\U0001F4D6️help/" + lib.JSON_NAME),
    ("invocation.ts", B + "/\U0001F9ED️invocation/" + lib.TS),
    ("selection.ts", B + "/\U0001F9ED️selection/" + lib.TS),
    ("selection-test.ts", B + "/\U0001F9ED️selection/" + E_TESTS),
    ("build-script.ts", B + "/" + lib.SCRIPT),
    ("script.ts", Z + "/" + lib.SCRIPT),
    ("bazel-workers.ts", Z + "/\U0001F477️workers/" + lib.TS),
    ("bazel-workers-test.ts", Z + "/\U0001F477️workers/" + E_TESTS),
    ("bazel-workers-fixture.json", Z + "/\U0001F477️workers/" + E_FIX),
    ("tests-actions-fixture.json", Z + "/" + lib.TESTS + "/\U0001F4E5️actions/" + lib.JSON_NAME),
    ("tests-implementation-fixture.json", Z + "/" + lib.TESTS + "/\U0001F50F️implementation/" + lib.JSON_NAME),
    ("nx-workers.ts", lib.NXWORKERS + "/" + lib.TS),
    ("nx-workers-test.ts", lib.NXWORKERS + "/" + E_TESTS),
    ("nx-workers-fixture.json", lib.NXWORKERS + "/" + E_FIX),
    ("nx-bootstrap.ts", lib.BOOT + "/" + lib.SCRIPT),
]


def main():
    args = sys.argv[1:]
    root = lib.ROOT
    if "--root" in args:
        root = args[args.index("--root") + 1]
    bases_path = os.path.join(lib.TOOLS, "bases.json")
    bases = json.load(open(bases_path)) if os.path.exists(bases_path) else {}
    record = "--record" in args
    force = "--force" in args
    write = "--write" in args
    problems, actions = [], []
    for name, target in TARGETS:
        source = os.path.join(lib.STAGED, name)
        if not os.path.exists(source):
            actions.append(("missing-staged", name, target))
            continue
        staged_bytes = open(source, "rb").read()
        path = os.path.join(root, target)
        if len(path) > 256 and root == lib.ROOT:
            problems.append(f"path longer than 256: {target}")
        live = open(path, "rb").read() if os.path.exists(path) else None
        if live is None:
            actions.append(("create", name, target))
        elif live == staged_bytes:
            actions.append(("same", name, target))
        else:
            expected = bases.get(target)
            if record:
                bases[target] = lib.sha(live)
                expected = bases[target]
            if force:
                pass
            elif expected is None:
                problems.append(f"existing target without a recorded base: {target}")
            elif lib.sha(live) != expected:
                problems.append(f"drift (live changed since staging): {target}")
            actions.append(("replace", name, target))
    if record:
        json.dump(bases, open(bases_path, "w"), indent=1, ensure_ascii=False)
    for kind, name, target in actions:
        print(f"{kind:15} {name:36} {target.split('/')[-2]}/{target.split('/')[-1]}")
    if problems:
        print("REFUSED:")
        for problem in problems:
            print("  " + problem)
        sys.exit(1)
    if not write:
        print("dry run: nothing written")
        return
    for kind, name, target in actions:
        if kind in ("create", "replace"):
            path = os.path.join(root, target)
            os.makedirs(os.path.dirname(path), exist_ok=True)
            with open(path, "wb") as handle:
                handle.write(open(os.path.join(lib.STAGED, name), "rb").read())
    print("written")


main()
