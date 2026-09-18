#!/usr/bin/env python3
"""One-off repair for the 2026-09-03 rename-plan codemod incident (T1).

Signature (memory project-codex-rename-plan-codemod-incident):
an emoji + VS16 was spliced INTO an ASCII identifier of jco/wit-bindgen output, e.g.

    export type * as WasiCliEnvironmen<U+1F52C><U+FE0F>t029 from './interfaces/wasi-cli-environment.js';

jco names an import namespace `PascalCase(ns+pkg+iface) + version-digits`, which the
uncorrupted sibling lines in the same file prove (`WasiCliStderr029`, `WasiIoPoll029`).
So stripping the spliced emoji restores the byte-exact generator output — no wasm
rebuild needed, and no peer's 24 MB .core.wasm artifacts are touched.

Scope: ONLY gitignored generated build output (jco/wasm-component bindings,
storybook-static, jcoprobe bundles). Never rewrites tracked source.

Usage:
    python3 <ticket>/🔨️t1-repair-spliced-emoji.py --check     # report only
    python3 <ticket>/🔨️t1-repair-spliced-emoji.py --apply
"""
from __future__ import annotations
import io, os, re, subprocess, sys

ROOT = subprocess.run(
    ["git", "rev-parse", "--show-toplevel"],
    cwd=os.path.dirname(os.path.abspath(__file__)),
    capture_output=True, text=True, check=True,
).stdout.strip()
SKIP = {"node_modules", ".git", "target", "⚡️cache", ".venv", ".nx"}
EXT = (".ts", ".tsx", ".js", ".mjs", ".cjs", ".jsx", ".d.ts")

# word-char, emoji(+VS16/ZWJ/skin-tone), word-char  ->  drop the emoji
SPLICE = re.compile(
    r"(?<=[A-Za-z0-9_])"
    r"([\U0001F000-\U0001FAFF☀-➿⬀-⯿]"
    r"(?:️|︎|‍[\U0001F000-\U0001FAFF]|[\U0001F3FB-\U0001F3FF])*)"
    r"(?=[A-Za-z0-9_])"
)

GENERATED = (
    re.compile(r"(^|/)storybook-static/"),
    re.compile(r"jcoprobe"),
    re.compile(r"_component\.d\.ts$"),
    re.compile(r"(^|/)(\U0001F9E9️extension-modules|\U0001F50C️plugin-modules|dist|out-[a-z-]+)/"),
)


def is_generated(rel: str) -> bool:
    return any(p.search(rel) for p in GENERATED)


def is_ignored(path: str) -> bool:
    return subprocess.run(["git", "check-ignore", "-q", path], cwd=ROOT).returncode == 0


def main() -> int:
    apply = "--apply" in sys.argv
    changed, occ, skipped = [], 0, []
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in SKIP]
        for name in filenames:
            if not name.endswith(EXT):
                continue
            p = os.path.join(dirpath, name)
            rel = os.path.relpath(p, ROOT)
            try:
                src = io.open(p, encoding="utf-8").read()
            except (UnicodeDecodeError, OSError):
                continue
            out = SPLICE.sub("", src)
            if out == src:
                continue
            n = len(SPLICE.findall(src))
            if not is_generated(rel) or not is_ignored(p):
                skipped.append((rel, n))
                continue
            occ += n
            changed.append((rel, n))
            if apply:
                io.open(p, "w", encoding="utf-8").write(out)

    print(f"generated files with spliced emoji: {len(changed)}  occurrences: {occ}")
    for rel, n in sorted(changed):
        print(f"  {'repaired' if apply else 'would repair'} {n:3d}  {rel}")
    if skipped:
        print(f"\nNOT generated / not gitignored — left untouched ({len(skipped)}):")
        for rel, n in sorted(skipped):
            print(f"  skip {n:3d}  {rel}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
