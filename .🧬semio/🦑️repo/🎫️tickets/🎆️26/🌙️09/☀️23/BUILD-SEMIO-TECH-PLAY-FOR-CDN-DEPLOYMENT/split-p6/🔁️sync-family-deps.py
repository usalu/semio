#!/usr/bin/env python3
"""🔁️ Re-derives the family dependencies of every member of one artifact workspace from the sources it compiles.

For each member manifest the lib closure (production cfg) and the test closure (`test` cfg) are read with the tolerant
canonical closure reader (`📏️unit-files.ts`). A family crate (package name with the family prefix) is a dependency
when its identifier appears in the stripped production closure and a dev-dependency when it appears only in the test
closure. A dev edge that would close a cycle over production edges is reported (those tests must move up) and left
out. External dependencies are not touched. Prints the production crate graph and any cycle.

usage: 🔁️sync-family-deps.py <workspace dir> <family package prefix> [--write]
"""
import json, os, re, subprocess, sys, importlib.util

HERE = os.path.dirname(os.path.abspath(__file__))
REPO = os.path.abspath(os.path.join(HERE, *[".."] * 8))
spec = importlib.util.spec_from_file_location("relocate", os.path.join(HERE, "🧭️relocate.py"))
relocate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(relocate)
workspace_dir, prefix = os.path.join(REPO, sys.argv[1]), sys.argv[2]
write = "--write" in sys.argv
ident_prefix = prefix.replace("-", "_")


def toml_load(path):
    out = subprocess.run(["bun", "-e", "console.log(JSON.stringify(Bun.TOML.parse(await Bun.file(process.argv[1]).text())))", path], capture_output=True, text=True, check=True)
    return json.loads(out.stdout)


def closure(root, mode):
    out = subprocess.run(["bun", os.path.join(HERE, "📏️unit-files.ts"), root, mode, "--files"], capture_output=True, text=True, cwd=HERE)
    return [line.split("\t")[4] for line in out.stdout.splitlines() if line.startswith("F\t") and line.split("\t")[3] == "source"]


TEST_ATTR = re.compile(r"#\s*\[\s*cfg\s*\(\s*test\s*\)\s*\]")


def lib_only(text):
    out = list(text)
    for m in TEST_ATTR.finditer(text):
        j, depth = m.end(), 0
        while j < len(text):
            c = text[j]
            if c in "([":
                depth += 1
            elif c in ")]":
                depth -= 1
            elif c == "{":
                j = relocate.close_of(text, j) + 1
                if depth == 0:
                    break
                continue
            elif c == ";" and depth == 0:
                j += 1
                break
            j += 1
        out[m.start():j] = " " * (j - m.start())
    return "".join(out)


workspace = toml_load(os.path.join(workspace_dir, "Cargo.toml"))
members = {}
for member in workspace["workspace"]["members"]:
    path = os.path.join(workspace_dir, member, "Cargo.toml")
    manifest = toml_load(path)
    name = manifest["package"]["name"]
    if not name.startswith(prefix):
        continue
    root = os.path.normpath(os.path.join(workspace_dir, member, manifest.get("lib", {}).get("path", "src/lib.rs")))
    members[name] = {"path": path, "root": root, "manifest": manifest}
idents = {name.replace("-", "_"): name for name in members}
pattern = re.compile(r"(?<![\w])(" + "|".join(sorted(map(re.escape, idents), key=len, reverse=True)) + r")(?![\w])")
for name, info in members.items():
    lib_files, test_files = closure(info["root"], "native"), closure(info["root"], "test")
    lib_text = "\n".join(lib_only(relocate.strip(open(os.path.join(REPO, f), encoding="utf-8").read())) for f in lib_files)
    test_text = "\n".join(relocate.strip(open(os.path.join(REPO, f), encoding="utf-8").read()) for f in test_files)
    info["deps"] = {idents[m] for m in pattern.findall(lib_text)} - {name}
    info["dev"] = {idents[m] for m in pattern.findall(test_text)} - {name} - info["deps"]


def reaches(a, b, seen=None):
    seen = seen or set()
    if a == b:
        return True
    seen.add(a)
    return any(reaches(n, b, seen) for n in members[a]["deps"] if n not in seen)


cycles = [(a, b) for a in members for b in members[a]["deps"] if reaches(b, a)]
for a, b in cycles:
    print(f"CYCLE {a} -> {b}")
for name, info in sorted(members.items()):
    upward = sorted(d for d in info["dev"] if reaches(d, name))
    info["dev"] -= set(upward)
    print(f"{name}: deps {sorted(d[len(prefix):] or '<agg>' for d in info['deps'])} dev {sorted(d[len(prefix):] or '<agg>' for d in info['dev'])}" + (f"  DISPLACED-TESTS {sorted(d[len(prefix):] or '<agg>' for d in upward)}" if upward else ""))
    text = open(info["path"], encoding="utf-8").read()

    def rewrite(table, wanted, text):
        m = re.search(r"(?ms)^\[" + re.escape(table) + r"\]\n(.*?)(?=^\[|\Z)", text)
        body = m.group(1) if m else ""
        kept = [line for line in body.split("\n") if line.strip() and not re.match(re.escape(prefix) + r"[\w-]*\s*=", line.strip()) and not line.strip().startswith(prefix + " ")]
        kept += [f"{d} = {{ workspace = true }}" for d in sorted(wanted)]
        kept.sort(key=lambda line: line.split("=")[0].strip())
        block = f"[{table}]\n" + "\n".join(kept) + "\n\n" if kept else ""
        if m:
            return text[:m.start()] + block + text[m.end():]
        return text.rstrip("\n") + "\n\n" + block if block else text

    new = rewrite("dependencies", info["deps"], text)
    new = rewrite("dev-dependencies", info["dev"], new).rstrip("\n") + "\n"
    if write and new != text:
        open(info["path"], "w", encoding="utf-8").write(new)
