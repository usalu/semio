#!/usr/bin/env python3
"""🔁️ Rewrites every Rust path that reaches an os store item through an interim store-root re-export onto the item's
owning module (wave E1b): `store::ArtifactDsl` -> `store::os_store::serialization::ArtifactDsl`,
`crate::ArtifactEnvelope` (inside the kernel) -> `crate::os_store::store_schema::ArtifactEnvelope`, and the same inside
`use` trees (`use store::{A, ArtifactDsl}` -> `use store::{A, os_store::serialization::ArtifactDsl}`).

The name map is read from the store root's `pub use <module>::{..}` / `pub(crate) use <module>::{..}` lines (the interim
re-exports), so it must run while they still exist. Only qualifiers that resolve to the kernel root or to `os_store` are
touched; everything else (other crates' `store`, `super::`, glob users) is left for the compiler.
Usage: owning-paths.py --kernel-files LIST [--map JSON | --dump-map JSON] [--write] [--files F ...]   (default: every *.rs)
"""
import argparse, json, os, re, subprocess, sys

REPO = "/Users/ueli/Documents/semio"
ROOT = f"{REPO}/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs"
MODS = ("document_store_owners|ledger|serialization|document_materialize|edit_retirement|presence|snapshot_read|"
        "one_item_publication|one_item|store_schema|command_format|replay|artifact_child|backbone|space_history_document|"
        "owned_schema_decode|text_format|codec_registry|composition_coordinator")
EXTERNAL_ROOTS = {"store::", "semio_framework_os_kernel::", "::semio_framework_os_kernel::", "directory::", "::directory::",
                  "kernel::", "__kernel::", "crate::__kernel::", "crate::kernel::", "crate::store::", "fixture_store::", "dsl_lsp::",
                  "semio_framework_plugin::plugin_app_close_prelude::store::", "semio_framework_plugin::__kernel::", "store_sync::",
                  "protocol::", "dsl::", "vcs::", "app_store::"}
KERNEL_ROOTS = {"crate::", "store::", "semio_framework_os_kernel::", "::semio_framework_os_kernel::", "dsl::", "vcs::", "spr::",
                "dsl_grammar::", "semio_format::", "$crate::"}


def name_map():
    out = {}
    lines = open(ROOT, encoding="utf-8").read().split("\n")
    pat = re.compile(r"^(pub(?:\(crate\))?) use (" + MODS + r")::(?:\{([^}]*)\}|(\w+));$")
    for l in lines:
        m = pat.match(l)
        if not m: continue
        for n in [x.strip() for x in (m.group(3) if m.group(3) is not None else m.group(4)).split(",") if x.strip()]:
            if n in out and out[n] != m.group(2): sys.exit(f"{n} re-exported from two modules")
            out[n] = m.group(2)
    return out


def rewrite(src, names, kernel, stats):
    alt = "|".join(sorted(map(re.escape, names), key=len, reverse=True))
    roots = KERNEL_ROOTS if kernel else EXTERNAL_ROOTS

    def target(q, n):
        if q.endswith("os_store::component::"): return q[:-len("component::")] + names[n] + "::"
        if q.endswith("os_store::"): return q + names[n] + "::"
        if q in roots:
            if kernel: return ("$crate::" if q == "$crate::" else "crate::") + "os_store::" + names[n] + "::"
            if q == "store_sync::": return None
            return q + "os_store::" + names[n] + "::"
        return None

    def qualified(m):
        q, n = m.group(1), m.group(2)
        t = target(q, n)
        if t is None: return m.group(0)
        stats["paths"] += 1
        return t + n

    src = re.sub(r"(?<![\w$])(?<!::)((?:\$crate::|::)?(?:[A-Za-z_][A-Za-z0-9_]*::)+)(" + alt + r")\b(?!\s*!)", qualified, src)

    def tree(body, mode):
        out, i, depth, start, parts = [], 0, 0, 0, []
        while i < len(body):
            c = body[i]
            if c == "{": depth += 1
            elif c == "}": depth -= 1
            elif c == "," and depth == 0: parts.append(body[start:i]); start = i + 1
            i += 1
        parts.append(body[start:])
        for p in parts:
            lead = p[:len(p) - len(p.lstrip())]; core = p.strip(); trail = p[len(p.rstrip()):] if p.strip() else ""
            if not core: out.append(p); continue
            m = re.match(r"^(\w+)(\s+as\s+\w+)?$", core)
            if m and m.group(1) in names:
                stats["trees"] += 1
                prefix = "os_store::" if mode == "root" else ""
                out.append(lead + prefix + names[m.group(1)] + "::" + core + trail); continue
            m = re.match(r"^os_store::\{(.*)\}$", core, re.S)
            if m and mode == "root":
                out.append(lead + "os_store::{" + tree(m.group(1), "store") + "}" + trail); continue
            m = re.match(r"^os_store::component::\{(.*)\}$", core, re.S)
            if m and mode == "root":
                out.append(lead + "os_store::{" + tree(m.group(1), "store") + "}" + trail); continue
            out.append(p)
        return ",".join(out)

    def use_tree(m):
        q = m.group(2)
        mode = "store" if q.endswith("os_store::") else ("root" if q in roots and q != "store_sync::" else None)
        if mode is None: return m.group(0)
        j = m.end() - 1
        depth, k = 0, j
        while k < len(src_ref[0]):
            if src_ref[0][k] == "{": depth += 1
            elif src_ref[0][k] == "}":
                depth -= 1
                if depth == 0: break
            k += 1
        return m.group(0)

    out, pos = [], 0
    for m in re.finditer(r"(\buse\s+)((?:\$crate::|::)?(?:[A-Za-z_][A-Za-z0-9_]*::)+)\{", src):
        if m.start() < pos: continue
        q = m.group(2)
        mode = "store" if (q.endswith("os_store::") or q.endswith("os_store::component::")) else ("root" if q in roots and q != "store_sync::" else None)
        if mode is None: continue
        j = m.end() - 1; depth = 0; k = j
        while k < len(src):
            if src[k] == "{": depth += 1
            elif src[k] == "}":
                depth -= 1
                if depth == 0: break
            k += 1
        q2 = q[:-len("component::")] if q.endswith("os_store::component::") else q
        out.append(src[pos:m.start()] + m.group(1) + q2 + "{" + tree(src[j + 1:k], mode) + "}")
        pos = k + 1
    out.append(src[pos:])
    return "".join(out)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--kernel-files", required=True)
    ap.add_argument("--write", action="store_true")
    ap.add_argument("--files", nargs="*")
    ap.add_argument("--dump-map")
    ap.add_argument("--map", help="owning-module JSON written by --dump-map while the re-exports still existed")
    a = ap.parse_args()
    names = json.load(open(a.map)) if a.map else name_map()
    if a.dump_map: json.dump(names, open(a.dump_map, "w"), indent=1, sort_keys=True)
    kernel = {os.path.join(REPO, l.strip()) for l in open(a.kernel_files) if l.strip()}
    files = a.files
    if not files:
        r = subprocess.run(["rg", "-l", "-g", "*.rs", "-g", "!**/target/**", "-g", "!.🧬semio/**", "-g", "!**/node_modules/**",
                            r"\b(" + "|".join(names) + r")\b", REPO], capture_output=True, text=True)
        files = [f for f in r.stdout.split("\n") if f]
    total, changed = {"paths": 0, "trees": 0}, 0
    for f in files:
        f = os.path.abspath(f)
        src = open(f, encoding="utf-8").read()
        stats = {"paths": 0, "trees": 0}
        new = rewrite(src, names, f in kernel or f == ROOT, stats)
        if new != src:
            changed += 1
            for k in total: total[k] += stats[k]
            if a.write: open(f, "w", encoding="utf-8").write(new)
            else: print(f"{stats['paths']:4} {stats['trees']:4} {os.path.relpath(f, REPO)}")
    print(f"[owning-paths] {len(names)} names; {changed} files; {total['paths']} qualified paths, {total['trees']} use-tree entries")


if __name__ == "__main__":
    main()
