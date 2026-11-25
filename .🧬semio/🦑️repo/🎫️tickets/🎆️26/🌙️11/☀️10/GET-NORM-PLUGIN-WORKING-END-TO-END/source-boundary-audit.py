#!/usr/bin/env python3
"""📏️ Reproduces the Rust source-boundary count per crate: files and bytes of the closure admitted before the compiler, ranked against 32 files and 262144 bytes.
@see 🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/📥️source/🟦️.ts (readRustSourceClosureV1)
@see 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/📦️units/📥️inputs/🟦️.ts (readRustUnitSourceClosureV1)
"""

import argparse
import glob
import os
import re
import sys
from pathlib import Path

FILE_LIMIT = 32
BYTE_LIMIT = 262144
THRESHOLD = 0.85
HIGH = 0.95
SCAN_ROOTS = ["✏️s/🔌️plugins/📕️norm", "🌎️hub/🧩️compositions/📕️norm"]
FRAMEWORK_MANIFESTS = [
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📄️docs/📦️packages/🦀️rust/Cargo.toml",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🔄️turn/📦️packages/🦀️rust/Cargo.toml",
    "🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust/Cargo.toml",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/📦️packages/🦀️rust/Cargo.toml",
]
PROFILES = {
    "macos": ["unix", 'target_family="unix"', 'target_os="macos"', 'target_vendor="apple"', 'target_arch="aarch64"', 'target_pointer_width="64"', 'target_endian="little"', 'target_env=""', 'panic="unwind"', "debug_assertions"],
    "linux": ["unix", 'target_family="unix"', 'target_os="linux"', 'target_vendor="unknown"', 'target_arch="x86_64"', 'target_pointer_width="64"', 'target_endian="little"', 'target_env="gnu"', 'panic="unwind"', "debug_assertions"],
    "windows": ["windows", 'target_family="windows"', 'target_os="windows"', 'target_vendor="pc"', 'target_arch="x86_64"', 'target_pointer_width="64"', 'target_endian="little"', 'target_env="msvc"', 'panic="unwind"', "debug_assertions"],
}
INCLUDE_MACROS = {"include": False, "include_str": True, "include_bytes": True}
SKIP_DIRS = {"target", "node_modules", ".cache", ".git"}
RAW_STRING = re.compile(r'b?r(#*)"')
IDENT = re.compile(r"r#[^\W\d]\w*|[^\W\d]\w*")
LIFETIME = re.compile(r"'[^\W\d]\w*")
NUMBER = re.compile(r"\d\w*")


class Tok:
    __slots__ = ("kind", "value")

    def __init__(self, kind, value):
        self.kind = kind
        self.value = value


def is_punct(tok, chars):
    return tok.kind == "p" and tok.value == chars


def is_word(tok, word):
    return tok.kind == "id" and tok.value == word


def tokenize(src):
    toks = []
    i = 0
    n = len(src)
    while i < n:
        c = src[i]
        if c.isspace():
            i += 1
            continue
        if src.startswith("//", i):
            j = src.find("\n", i)
            i = n if j < 0 else j + 1
            continue
        if src.startswith("/*", i):
            depth = 0
            while i < n:
                if src.startswith("/*", i):
                    depth += 1
                    i += 2
                elif src.startswith("*/", i):
                    depth -= 1
                    i += 2
                    if depth == 0:
                        break
                else:
                    i += 1
            continue
        if c in "rb" and (i == 0 or not (src[i - 1].isalnum() or src[i - 1] == "_")):
            m = RAW_STRING.match(src, i)
            if m:
                closing = '"' + m.group(1)
                j = src.find(closing, m.end())
                j = n if j < 0 else j
                toks.append(Tok("str", src[m.end():j]))
                i = j + len(closing)
                continue
            if c == "b" and src.startswith('"', i + 1):
                i += 1
                c = '"'
        if c == '"':
            j = i + 1
            buf = []
            while j < n and src[j] != '"':
                if src[j] == "\\" and j + 1 < n:
                    buf.append(src[j + 1])
                    j += 2
                else:
                    buf.append(src[j])
                    j += 1
            toks.append(Tok("str", "".join(buf)))
            i = j + 1
            continue
        if c == "'":
            if src.startswith("\\", i + 1):
                j = i + 3
                while j < n and src[j] != "'":
                    j += 2 if src[j] == "\\" else 1
                toks.append(Tok("char", ""))
                i = j + 1
                continue
            if i + 2 < n and src[i + 2] == "'":
                toks.append(Tok("char", ""))
                i += 3
                continue
            m = LIFETIME.match(src, i)
            if m:
                toks.append(Tok("life", m.group(0)))
                i = m.end()
                continue
        if c.isalpha() or c == "_":
            m = IDENT.match(src, i)
            name = m.group(0)
            toks.append(Tok("id", name[2:] if name.startswith("r#") else name))
            i = m.end()
            continue
        if c.isdigit():
            m = NUMBER.match(src, i)
            toks.append(Tok("num", m.group(0)))
            i = m.end()
            continue
        toks.append(Tok("p", c))
        i += 1
    return toks


def match_close(toks, i):
    depth = 0
    for k in range(i, len(toks)):
        t = toks[k]
        if t.kind != "p":
            continue
        if t.value in "([{":
            depth += 1
        elif t.value in ")]}":
            depth -= 1
            if depth == 0:
                return k
    raise ValueError("unbalanced delimiter")


def split_top(toks, s, e):
    parts = []
    depth = 0
    start = s
    for k in range(s, e):
        t = toks[k]
        if t.kind != "p":
            continue
        if t.value in "([{":
            depth += 1
        elif t.value in ")]}":
            depth -= 1
        elif t.value == "," and depth == 0:
            parts.append((start, k))
            start = k + 1
    if start < e:
        parts.append((start, e))
    return [p for p in parts if p[0] < p[1]]


def eval_pred(toks, s, e, cfg):
    if s >= e or toks[s].kind != "id":
        raise ValueError("cfg predicate required")
    head = toks[s]
    if s + 1 < e and is_punct(toks[s + 1], "("):
        close = match_close(toks, s + 1)
        values = [eval_pred(toks, a, b, cfg) for a, b in split_top(toks, s + 2, close)]
        if head.value == "all":
            return all(values)
        if head.value == "any":
            return any(values)
        if head.value == "not":
            return not values[0]
        raise ValueError("unknown cfg combinator " + head.value)
    if s + 2 < e and is_punct(toks[s + 1], "=") and toks[s + 2].kind == "str":
        return f'{head.value}="{toks[s + 2].value}"' in cfg
    return head.value in cfg


def attr_effect(toks, s, e, cfg, acc):
    if s >= e or toks[s].kind != "id":
        return
    head = toks[s].value
    if head == "cfg" and s + 1 < e and is_punct(toks[s + 1], "("):
        close = match_close(toks, s + 1)
        if not eval_pred(toks, s + 2, close, cfg):
            acc["active"] = False
        return
    if head == "cfg_attr" and s + 1 < e and is_punct(toks[s + 1], "("):
        close = match_close(toks, s + 1)
        parts = split_top(toks, s + 2, close)
        if parts and eval_pred(toks, parts[0][0], parts[0][1], cfg):
            for a, b in parts[1:]:
                attr_effect(toks, a, b, cfg, acc)
        return
    if head == "path" and s + 2 < e and is_punct(toks[s + 1], "=") and toks[s + 2].kind == "str":
        acc["mount"] = toks[s + 2].value
    if head in ("test", "bench") and e == s + 1 and "test" not in cfg:
        acc["active"] = False


def outer_attrs(toks, i, e):
    attrs = []
    j = i
    while j + 1 < e and is_punct(toks[j], "#") and is_punct(toks[j + 1], "["):
        close = match_close(toks, j + 1)
        attrs.append((j + 2, close))
        j = close + 1
    return attrs, j


def skip_vis(toks, j, e):
    if j < e and is_word(toks[j], "pub"):
        j += 1
        if j < e and is_punct(toks[j], "("):
            j = match_close(toks, j) + 1
    return j


def skip_item(toks, j, e):
    k = j
    while k < e:
        t = toks[k]
        if t.kind == "p":
            if t.value == "{":
                k = match_close(toks, k) + 1
                if k < e and toks[k].kind == "p" and toks[k].value in ";,":
                    k += 1
                return k
            if t.value in "([":
                k = match_close(toks, k) + 1
                continue
            if t.value in ";,":
                return k + 1
            if t.value in ")]}":
                return k
        k += 1
    return e


class Closure:
    def __init__(self, cfg, cache):
        self.cfg = cfg
        self.cache = cache
        self.files = {}
        self.visited = set()
        self.issues = []

    def visit(self, path, base, embedded, origin):
        key = (path, base, embedded)
        if key in self.visited:
            return
        self.visited.add(key)
        if not os.path.isfile(path):
            self.issues.append("missing " + origin + " -> " + path)
            return
        self.files[path] = os.path.getsize(path)
        if embedded:
            return
        toks = self.cache.get(path)
        if toks is None:
            toks = tokenize(Path(path).read_bytes().decode("utf8", "replace"))
            self.cache[path] = toks
        self.scope(toks, 0, len(toks), path, base, True)

    def mount_module(self, name, mount, path, base, file_scope):
        if mount is None:
            cands = [os.path.join(base, name + ".rs"), os.path.join(base, name, "mod.rs")]
        else:
            cands = [os.path.join(os.path.dirname(path) if file_scope else base, mount)]
        present = [os.path.normpath(c) for c in cands if os.path.isfile(os.path.normpath(c))]
        if len(present) != 1:
            self.issues.append("unresolved mod " + name + " in " + path)
            return
        child = present[0]
        child_base = os.path.join(os.path.dirname(child), name) if mount is None and child.endswith(name + ".rs") else os.path.dirname(child)
        self.visit(child, os.path.normpath(child_base), False, path)

    def scope(self, toks, s, e, path, base, file_scope):
        i = s
        while i + 2 < e and is_punct(toks[i], "#") and is_punct(toks[i + 1], "!") and is_punct(toks[i + 2], "["):
            close = match_close(toks, i + 2)
            acc = {"active": True, "mount": None}
            attr_effect(toks, i + 3, close, self.cfg, acc)
            if not acc["active"]:
                return
            i = close + 1
        while i < e:
            attrs, j = outer_attrs(toks, i, e)
            acc = {"active": True, "mount": None}
            for a, b in attrs:
                attr_effect(toks, a, b, self.cfg, acc)
            j = skip_vis(toks, j, e)
            if j + 1 < e and is_word(toks[j], "mod") and toks[j + 1].kind == "id":
                name = toks[j + 1].value
                k = j + 2
                if k < e and is_punct(toks[k], ";"):
                    if acc["active"]:
                        self.mount_module(name, acc["mount"], path, base, file_scope)
                    i = k + 1
                    continue
                if k < e and is_punct(toks[k], "{"):
                    close = match_close(toks, k)
                    if acc["active"]:
                        self.scope(toks, k + 1, close, path, os.path.normpath(os.path.join(base, acc["mount"] or name)), False)
                    i = close + 1
                    continue
            if attrs:
                i = j if acc["active"] else skip_item(toks, j, e)
                continue
            if is_word(toks[i], "macro_rules") and i + 3 < e and is_punct(toks[i + 1], "!") and toks[i + 2].kind == "id" and toks[i + 3].kind == "p" and toks[i + 3].value in "({[":
                i = match_close(toks, i + 3) + 1
                continue
            if toks[i].kind == "id" and toks[i].value in INCLUDE_MACROS and i + 1 < e and is_punct(toks[i + 1], "!"):
                if i + 4 < e and is_punct(toks[i + 2], "(") and toks[i + 3].kind == "str" and is_punct(toks[i + 4], ")"):
                    target = os.path.normpath(os.path.join(os.path.dirname(path), toks[i + 3].value))
                    self.visit(target, os.path.dirname(target), INCLUDE_MACROS[toks[i].value], path)
                    i += 5
                    continue
                self.issues.append("unparsed " + toks[i].value + "! in " + path)
            i += 1


def closure(entry, cfg, cache):
    c = Closure(cfg, cache)
    entry = os.path.normpath(entry)
    c.visit(entry, os.path.dirname(entry), False, entry)
    return len(c.files), sum(c.files.values()), c.issues


def parse_sections(text):
    entries = [("", [])]
    for line in text.splitlines():
        s = line.strip()
        if s.startswith("[") and s.endswith("]") and "=" not in s:
            name = s.strip("[]").strip()
            entries.append((name + ("[]" if s.startswith("[[") else ""), []))
        else:
            entries[-1][1].append(line)
    return entries


def manifest_info(path):
    base = path.parent
    entries = parse_sections(path.read_text(encoding="utf8"))

    def blocks(key):
        return ["\n".join(lines) for k, lines in entries if k == key]

    pkg_blocks = blocks("package")
    if not pkg_blocks:
        return None
    pkg = pkg_blocks[0]
    name = re.search(r'^\s*name\s*=\s*"([^"]+)"', pkg, re.M).group(1)

    def path_of(block):
        m = re.search(r'^\s*path\s*=\s*"([^"]+)"', block, re.M)
        return m.group(1) if m else None

    lib_block = blocks("lib")
    lib_entry = base / (path_of(lib_block[0]) if lib_block and path_of(lib_block[0]) else "src/lib.rs")
    lib = lib_entry if lib_entry.is_file() else None
    autobins = not re.search(r"^\s*autobins\s*=\s*false", pkg, re.M)
    autotests = not re.search(r"^\s*autotests\s*=\s*false", pkg, re.M)

    bins = [base / p for p in (path_of(b) for b in blocks("bin[]")) if p]
    tests = [base / p for p in (path_of(t) for t in blocks("test[]")) if p]
    if autobins:
        bins += [Path(p) for p in sorted(glob.glob(str(base / "src/main.rs")) + glob.glob(str(base / "src/bin/*.rs")) + glob.glob(str(base / "src/bin/*/main.rs")))]
    if autotests:
        tests += [Path(p) for p in sorted(glob.glob(str(base / "tests/*.rs")) + glob.glob(str(base / "tests/*/main.rs")))]

    build_match = re.search(r'^\s*build\s*=\s*(?:"([^"]+)"|(false))', pkg, re.M)
    if build_match:
        build = None if build_match.group(2) else base / build_match.group(1)
    else:
        build = base / "build.rs" if (base / "build.rs").is_file() else None

    feature_map = {}
    for block in blocks("features"):
        for m in re.finditer(r"^\s*([A-Za-z0-9_.-]+)\s*=\s*\[(.*?)\]", block, re.M | re.S):
            feature_map[m.group(1)] = re.findall(r'"([^"]*)"', m.group(2))
    enabled = set()
    stack = list(feature_map.get("default", []))
    while stack:
        f = stack.pop()
        if f in enabled or "/" in f or f.startswith("dep:") or f.startswith("?"):
            continue
        enabled.add(f)
        stack.extend(feature_map.get(f, []))
    every = set(feature_map) | {m for members in feature_map.values() for m in members if "/" not in m and not m.startswith(("dep:", "?"))}
    every.discard("default")

    def unique(paths):
        seen = []
        for p in paths:
            if p not in seen and p.is_file():
                seen.append(p)
        return seen

    return {
        "name": name,
        "dir": base,
        "lib": lib,
        "bins": unique(bins),
        "tests": unique(tests),
        "build": build if build is not None and build.is_file() else None,
        "features": enabled,
        "all_features": every,
    }


def units_of(info):
    out = []
    base = info["dir"]
    variants = [("", info["features"])]
    if info["all_features"] != info["features"]:
        variants.append((" +all-features", info["all_features"]))
    for suffix, feats in variants:
        if info["lib"]:
            out.append(("build", "lib" + suffix, info["lib"], False, feats))
            out.append(("test", "lib[cfg(test)]" + suffix, info["lib"], True, feats))
        for b in info["bins"]:
            out.append(("build", "bin " + os.path.relpath(b, base) + suffix, b, False, feats))
            out.append(("test", "bin[cfg(test)] " + os.path.relpath(b, base) + suffix, b, True, feats))
        for t in info["tests"]:
            out.append(("test", "test " + os.path.relpath(t, base) + suffix, t, True, feats))
    if info["build"]:
        out.append(("build", "build-script", info["build"], False, info["features"]))
    return out


def unit_cfg(target, features, test):
    cfg = set(PROFILES[target]) | {f'feature="{f}"' for f in features}
    if test:
        cfg.add("test")
    return cfg


def collect_manifests(root):
    found = []
    for scan in SCAN_ROOTS:
        for dirpath, dirnames, filenames in os.walk(root / scan):
            dirnames[:] = [d for d in dirnames if d not in SKIP_DIRS]
            if "Cargo.toml" in filenames:
                found.append(Path(dirpath) / "Cargo.toml")
    for rel in FRAMEWORK_MANIFESTS:
        p = root / rel
        if not p.is_file():
            sys.exit("missing framework manifest: " + rel)
        found.append(p)
    return sorted(set(found))


def audit(root, target):
    cache = {}
    rows = []
    manifests = collect_manifests(root)
    for manifest in manifests:
        info = manifest_info(manifest)
        if info is None:
            continue
        best = None
        by_group = {}
        issues = []
        for group, label, entry, test, feats in units_of(info):
            cfg = unit_cfg(target, feats, test)
            try:
                files, size, problems = closure(str(entry), cfg, cache)
            except ValueError as error:
                files, size, problems = 0, 0, ["parse " + str(error) + " in " + str(entry)]
            ratio = max(files / FILE_LIMIT, size / BYTE_LIMIT)
            unit = {"group": group, "label": label, "files": files, "bytes": size, "ratio": ratio}
            if best is None or ratio > best["ratio"]:
                best = unit
            if group not in by_group or ratio > by_group[group]["ratio"]:
                by_group[group] = unit
            issues.extend(problems)
        if best is None:
            continue
        rows.append({
            "crate": info["name"],
            "manifest": os.path.relpath(manifest, root),
            "unit": best["group"] + ":" + best["label"],
            "files": best["files"],
            "bytes": best["bytes"],
            "ratio": best["ratio"],
            "build_ratio": by_group.get("build", {"ratio": 0.0})["ratio"],
            "build_unit": by_group["build"]["label"] if "build" in by_group else "-",
            "test_ratio": by_group.get("test", {"ratio": 0.0})["ratio"],
            "issues": len(issues),
            "issue_sample": issues[:3],
        })
    rows.sort(key=lambda r: (-r["ratio"], r["crate"]))
    return rows, len(manifests)


def markdown(rows, scanned, target):
    over = [r for r in rows if r["ratio"] > 1]
    high = [r for r in rows if r["ratio"] > HIGH]
    listed = [r for r in rows if r["ratio"] > THRESHOLD]
    out = [
        "# Source boundary audit 2026-11-25",
        "",
        "Read-only reproduction of the Rust source-boundary count from `readRustUnitSourceClosureV1` and `readRustSourceClosureV1`: files and bytes of the closure reachable from each unit entry, admitted before the compiler against 32 files and 262144 bytes.",
        "",
        "Counting rules reproduced by `source-boundary-audit.py`:",
        "",
        "- The entry is the lib, each bin, each integration test and the build script (`build.rs`). Each unit is counted on its own.",
        "- Closure: `mod x;` resolves to `x.rs` or `x/mod.rs` (or `#[path]`), inline `mod x { }` moves the module base, `include!` is parsed as source, `include_str!` and `include_bytes!` count as files and bytes without being parsed.",
        "- Bytes are whole-file sizes, not active bytes. Files are unique paths. Items gated by a false `cfg` (including `cfg(test)` for build units) are dropped with their `mod` and `include!`.",
        "- Build units use cfg `" + target + "` debug without `test`. Test units add `test`. Each unit is counted twice: with default features, and with all features (`+all-features`). Dependents enable features such as `test-support` through feature unification, which the default pass cannot see, so the all-features pass is the upper bound.",
        "- Not counted: `maxActiveLines` (4000), `env!`/`concat!`/`OUT_DIR` generated inputs, and files that no `mod` or `include!` reaches.",
        "",
        f"Scanned {scanned} manifests with a `[package]` section. Crates listed below are above {int(THRESHOLD * 100)}% of either limit, ranked by the worse of the two ratios.",
        "",
        f"Over the limit: {len(over)}. Above {int(HIGH * 100)}%: {len(high)}. Above {int(THRESHOLD * 100)}%: {len(listed)}.",
        "",
        "| Rank | Crate | Manifest | Worst unit | Files (of 32) | Bytes (of 262144) | Worst % | Build % | Test % | Issues |",
        "| ---: | --- | --- | --- | ---: | ---: | ---: | ---: | ---: | ---: |",
    ]
    for rank, r in enumerate(listed, 1):
        out.append(f"| {rank} | {r['crate']} | `{r['manifest']}` | `{r['unit']}` | {r['files']} | {r['bytes']:,} | {r['ratio'] * 100:.1f}% | {r['build_ratio'] * 100:.1f}% | {r['test_ratio'] * 100:.1f}% | {r['issues']} |")
    if not listed:
        out.append("| - | none | - | - | - | - | - | - |")
    out.append("")
    return "\n".join(out)


def main():
    ap = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    ap.add_argument("--root", default=str(Path(__file__).resolve().parents[7]))
    ap.add_argument("--target", default="macos", choices=sorted(PROFILES))
    ap.add_argument("--report", default=str(Path(__file__).resolve().parent / "source-boundary-audit-2026-11-25.md"))
    args = ap.parse_args()
    root = Path(args.root)
    rows, scanned = audit(root, args.target)
    listed = [r for r in rows if r["ratio"] > THRESHOLD]
    for rank, r in enumerate(listed, 1):
        flag = "OVER" if r["ratio"] > 1 else ("HIGH" if r["ratio"] > HIGH else "WARN")
        print(f"{rank:>3} {flag:<4} {r['ratio'] * 100:6.1f}% files={r['files']:>2}/32 bytes={r['bytes']:>7}/262144 issues={r['issues']} {r['crate']} [{r['unit']}] {r['manifest']}")
    print(f"scanned={scanned} above_85={len(listed)} above_95={sum(1 for r in rows if r['ratio'] > HIGH)} over_limit={sum(1 for r in rows if r['ratio'] > 1)}")
    Path(args.report).write_text(markdown(rows, scanned, args.target), encoding="utf8")
    print("report=" + args.report)


if __name__ == "__main__":
    main()
