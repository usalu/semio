#!/usr/bin/env python3
"""🔬️ Scoped full-resolution differential (RESOLUTION-ORACLE.md): the touched crate before my edits against the crate as it is now.

Both closures are copied into sandboxes outside the repo (scratchpad), patched alike (extern crates removed, external globs neutralised) and checked with an analysis-only `rustc --emit=metadata -o /dev/null`. Errors are compared as multisets of (code, message, file relative to the sandbox root).
usage: diff_oracle.py <label> <package dir> [--test] [--target T] [--cfg 'feature="x"']... [--show N]
The baseline reads scratchpad/pre/<path> in place of every file this slice edited, hides the files it created (DM2_NEW_FILES) and restores the files it deleted.
"""
import collections, json, os, re, shutil, subprocess, sys

R = "/Users/ueli/Documents/semio"
F = R + "/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split"
SCRATCH = "/private/tmp/claude-501/-Users-ueli--claude-spares-spare-PBncC5/6c384ddf-f4e2-43bf-8d3c-c66daff05e49/scratchpad"
PRE = SCRATCH + "/dm2pre/"
ORC = SCRATCH + "/dm2orc"
NEW_FILES = json.load(open(F + "/dm2-tools/new-files.json", encoding="utf8")) if os.path.exists(F + "/dm2-tools/new-files.json") else []
TOOLCHAIN = "+nightly-2026-07-20"
SAFE_ROOTS = {"crate", "self", "super", "std", "core", "alloc"}


def closure(pkg, base, extra):
    env = dict(os.environ, DM2_NEW_FILES=json.dumps(NEW_FILES))
    cmd = ["bun", F + "/dm2-tools/closure.ts", pkg, "--json"] + (["--base"] if base else []) + extra
    out = subprocess.run(cmd, capture_output=True, text=True, env=env, cwd=R)
    if out.returncode != 0:
        raise SystemExit(out.stderr[-2000:])
    return [json.loads(line) for line in out.stdout.splitlines() if line.startswith("{")]


def source_of(rel, base):
    if base and os.path.exists(PRE + rel):
        return PRE + rel
    return R + "/" + rel


EXTERN_CRATE = re.compile(r"(?m)^\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?extern\s+crate\s+(?!std\b|core\b|alloc\b)[^;]+;")
GLOB_USE = re.compile(r"(?m)^(\s*(?:#\[[^\]]*\]\s*)*)((?:pub(?:\([^)]*\))?\s+)?use\s+(?:::)?([A-Za-z_][A-Za-z0-9_]*)::(?:[A-Za-z0-9_:]*::)?\*;)")


def patch(text, local_names):
    text = EXTERN_CRATE.sub(lambda m: " " * 0 + "", text)

    def glob(m):
        root = m.group(3)
        if root in SAFE_ROOTS or root in local_names:
            return m.group(0)
        return m.group(1) + "const _: () = ();"

    return GLOB_USE.sub(glob, text)


def build(label, side, entries, base):
    root = f"{ORC}/{label}/{side}"
    shutil.rmtree(root, ignore_errors=True)
    files = set()
    for entry in entries:
        files.update(entry["files"])
        files.add(entry["entry"])
    local = set()
    for rel in files:
        src = source_of(rel, base)
        if rel.endswith(".rs") and os.path.exists(src):
            local.update(re.findall(r"\bmod\s+([A-Za-z_][A-Za-z0-9_]*)", open(src, encoding="utf8").read()))
            local.update(re.findall(r"\b(?:enum|struct|type|trait)\s+([A-Za-z_][A-Za-z0-9_]*)", open(src, encoding="utf8").read()))
    for rel in sorted(files):
        src = source_of(rel, base)
        if not os.path.exists(src):
            continue
        dst = root + "/" + rel
        os.makedirs(os.path.dirname(dst), exist_ok=True)
        if rel.endswith(".rs"):
            text = open(src, encoding="utf8").read()
            open(dst, "w", encoding="utf8").write(patch(text, local))
        else:
            shutil.copy(src, dst)
    return root


def run(root, entry, crate, flags):
    cmd = ["rustc", TOOLCHAIN, "--edition", "2021", "--crate-type", "lib", "--crate-name", crate, "--emit=metadata", "-o", "/dev/null", "--error-format=human", "--color=never", "-A", "warnings"] + flags + [root + "/" + entry["entry"]]
    out = subprocess.run(cmd, capture_output=True, text=True, cwd=root)
    return out.stderr


ERR = re.compile(r"^error(?:\[(E\d+)\])?: (.*)$")
LOC = re.compile(r"^\s*--> (.*?):(\d+):(\d+)")


def parse(stderr, root):
    rows = collections.Counter()
    lines = stderr.splitlines()
    for i, line in enumerate(lines):
        m = ERR.match(line)
        if not m or line.startswith("error: aborting") or "could not compile" in line:
            continue
        loc = ""
        for j in range(i + 1, min(i + 4, len(lines))):
            lm = LOC.match(lines[j])
            if lm:
                loc = lm.group(1)
                if loc.startswith(root + "/"):
                    loc = loc[len(root) + 1:]
                break
        rows[(m.group(1) or "-", re.sub(r"\s+", " ", m.group(2))[:160], loc)] += 1
    return rows


def main():
    args = sys.argv[1:]
    label, pkg = args[0], args[1]
    extra, flags, show = [], [], 40
    i = 2
    while i < len(args):
        a = args[i]
        if a == "--test":
            extra.append("--tests")
            flags.append("--test")
        elif a == "--target":
            flags += ["--target", args[i + 1]]
            extra += ["--target", args[i + 1]]
            i += 1
        elif a == "--cfg":
            flags += ["--cfg", args[i + 1]]
            extra += ["--cfg", args[i + 1]]
            i += 1
        elif a == "--cur-only":
            pass
        elif a == "--show":
            show = int(args[i + 1])
            i += 1
        i += 1
    results = {}
    sides = (("cur", False),) if "--cur-only" in args else (("base", True), ("cur", False))
    for side, base in sides:
        entries = [e for e in closure(pkg, base, extra) if e["kind"] == "lib"]
        assert len(entries) == 1, entries
        root = build(label, side, entries, base)
        crate = entries[0]["crate"].replace("-", "_")
        stderr = run(root, entries[0], crate, flags)
        open(f"{F}/🗑️generated/dm2-oracle-{label}-{side}.log", "w", encoding="utf8").write(stderr)
        results[side] = parse(stderr, root)
    if "--cur-only" in args:
        cur = results["cur"]
        noise = re.compile(r"cannot find (?:module or crate|attribute|macro)|unresolved import|failed to resolve|can't find crate|cannot find derive macro")
        rows = [(k, n) for k, n in cur.items() if not noise.search(k[1])]
        print(f"[{label}] cur {sum(cur.values())} errors, {sum(n for _, n in rows)} outside extern noise")
        for (code, msg, loc), n in sorted(rows, key=lambda kv: kv[0][1])[:show]:
            print(f"   {n}x {code} {msg}   @ {loc[-90:]}")
        return
    base, cur = results["base"], results["cur"]

    def flat(c):
        out = collections.Counter()
        for (code, msg, loc), n in c.items():
            out[(code, msg)] += n
        return out

    fb, fc = flat(base), flat(cur)
    only_cur = fc - fb
    only_base = fb - fc
    print(f"[{label}] base {sum(base.values())} cur {sum(cur.values())} only-cur {sum(only_cur.values())} only-base {sum(only_base.values())}")
    where = collections.defaultdict(set)
    for (code, msg, loc), n in cur.items():
        where[(code, msg)].add(loc[-80:])
    for (code, msg), n in sorted(only_cur.items(), key=lambda kv: (kv[0][1], kv[0][0]))[:show]:
        print(f"   CUR+ {n}x {code} {msg}   @ {sorted(where[(code, msg)])[:2]}")
    whereb = collections.defaultdict(set)
    for (code, msg, loc), n in base.items():
        whereb[(code, msg)].add(loc[-80:])
    for (code, msg), n in sorted(only_base.items(), key=lambda kv: (kv[0][1], kv[0][0]))[:show]:
        print(f"   BASE+ {n}x {code} {msg}   @ {sorted(whereb[(code, msg)])[:2]}")


main()
