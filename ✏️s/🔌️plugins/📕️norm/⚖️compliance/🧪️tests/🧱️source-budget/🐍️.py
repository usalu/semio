"""🧱️ Every Cargo crate under the norm plugins stays inside the repo source boundary: 32 files, 262144 bytes and 4000 lines of its module closure (`mod`, `#[path]`, `include_str!`; `cfg(test)` code excluded).

`python -I 🐍️.py [-v]` warns from 85 % of a limit and exits non-zero above 100 %; `-v` lists every crate above 75 %.
"""
import pathlib, re, sys, tomllib

ROOT = pathlib.Path(__file__).resolve().parents[3]
LIMITS = {"bytes": 262144, "files": 32, "lines": 4000}
WARN, SPLIT = 0.85, 0.75
MOD = re.compile(r'((?:#\[path\s*=\s*"([^"]+)"\]\s*)?)(?:pub(?:\([^)]*\))?\s+)?mod\s+(\w+)\s*;')
INCLUDE = re.compile(r'include(?:_str|_bytes)?!\(\s*"([^"]+)"')
TEST_ITEM = re.compile(r'#\[cfg\(test\)\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?(?:mod\s+\w+\s*(;|\{)|(?:fn|use|const|static|struct|enum|impl|type)\b)')

def without_tests(text):
    out, i = [], 0
    while True:
        m = TEST_ITEM.search(text, i)
        if not m:
            out.append(text[i:])
            return "".join(out)
        out.append(text[i:m.start()])
        j = m.end()
        if m.group(1) == ";":
            i = j
            continue
        brace, semi = text.find("{", j - 1 if m.group(1) == "{" else j), text.find(";", j)
        if m.group(1) != "{" and semi != -1 and (brace == -1 or semi < brace):
            i = semi + 1
            continue
        depth = 0
        while brace < len(text):
            depth += (text[brace] == "{") - (text[brace] == "}")
            if depth == 0:
                break
            brace += 1
        i = brace + 1

def closure(entry):
    seen = {}
    stack = [entry]
    while stack:
        path = stack.pop().resolve()
        if path in seen or not path.is_file():
            continue
        raw = path.read_text(encoding="utf-8", errors="replace")
        text = without_tests(raw)
        seen[path] = (len(raw.encode()), text.count("\n") + 1)
        base = path.stem
        for m in MOD.finditer(text):
            if m.group(2):
                stack.append(path.parent / m.group(2))
            elif base in ("lib", "main", "mod"):
                stack += [path.parent / (m.group(3) + ".rs"), path.parent / m.group(3) / "mod.rs"]
            else:
                stack.append(path.parent / base / (m.group(3) + ".rs"))
        stack += [path.parent / m.group(1) for m in INCLUDE.finditer(text)]
    return seen

def crates(root=ROOT):
    for manifest in sorted(root.rglob("Cargo.toml")):
        if {"target", "node_modules"} & set(manifest.parts):
            continue
        config = tomllib.loads(manifest.read_text(encoding="utf-8"))
        if "package" not in config:
            continue
        files = closure(manifest.parent / config.get("lib", {}).get("path", "src/lib.rs"))
        yield config["package"]["name"], manifest.parent, {"bytes": sum(v[0] for v in files.values()), "files": len(files), "lines": sum(v[1] for v in files.values())}

def main(argv):
    verbose, over, warned = "-v" in argv, 0, 0
    for name, where, use in crates():
        ratio = max(use[k] / LIMITS[k] for k in LIMITS)
        if ratio <= (SPLIT if verbose else WARN):
            continue
        state = "OVER" if ratio > 1 else "warn" if ratio >= WARN else "info"
        over += state == "OVER"
        warned += state == "warn"
        print(f"{state:<4} {ratio:>4.0%} {use['bytes']:>7} B {use['files']:>2} files {use['lines']:>5} lines {name} {where.relative_to(ROOT)}")
    print(f"{over} crates over the source boundary, {warned} above {WARN:.0%}")
    return 1 if over else 0

if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
