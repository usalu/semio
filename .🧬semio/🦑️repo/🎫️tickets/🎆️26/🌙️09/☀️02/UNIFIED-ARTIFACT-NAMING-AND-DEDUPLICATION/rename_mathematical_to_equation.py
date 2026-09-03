#!/usr/bin/env python3
"""🔧️ Token-aware rewriter for the mathematical->equation ARTIFACT rename (plugin stays 'mathematical').

Rules applied to every text file under the (already `mv`-renamed) artifact subtree
`✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation/`:
  1. Protect substrings that name the PLUGIN or its crate, never the artifact:
     - "🔌️plugins/➗️mathematical"      (plugin directory path segment)
     - "semio_s_plugin_mathematical"   (crate name, underscore form)
     - "semio-s-plugin-mathematical"   (crate name, hyphen form)
     - "semio:mathematical"            (wasm component package name)
  2. Rename the PRE-EXISTING `EquationSnapshot` (the single-equation expr+label-allocator
     struct under .../schema/snapshot) to `EquationExprSnapshot` BEFORE the blanket pass,
     freeing up the name `EquationSnapshot` for the artifact-level type
     (`MathematicalSnapshot` -> `EquationSnapshot`) without collision.
  3. Blanket literal substring replace: Mathematical->Equation, mathematical->equation,
     MATHEMATICAL->EQUATION.
  4. Restore the protected substrings from step 1.
"""
import os
import re
import sys

ROOT = "✏️s/🔌️plugins/➗️mathematical/🗿️artifacts/➗️equation"

PROTECT = [
    ("🔌️plugins/➗️mathematical", "\x00PLUGINDIRTOK\x00"),
    ("semio_s_plugin_mathematical", "\x00CRATEUSTOK\x00"),
    ("semio-s-plugin-mathematical", "\x00CRATEHYTOK\x00"),
    ("semio:mathematical", "\x00CRATECOMPTOK\x00"),
]

SKIP_DIR_NAMES = {"target"}

TEXT_EXTS = {
    "rs", "ts", "json", "md", "semio", "proto", "graphql", "feature", "toml",
    "lock", "spicy", "ksy", "g4", "ebnf", "abnf", "csv", "jsonc",
}

EQUATION_SNAPSHOT_RE = re.compile(r"\bEquationSnapshot\b")


def should_process(path: str) -> bool:
    parts = path.split(os.sep)
    if any(p in SKIP_DIR_NAMES for p in parts):
        return False
    base = os.path.basename(path)
    if base == "AGENTS.md":
        return False
    if "." not in base:
        return False
    ext = base.rsplit(".", 1)[-1]
    return ext in TEXT_EXTS


def transform(text: str) -> str:
    for orig, placeholder in PROTECT:
        text = text.replace(orig, placeholder)
    text = EQUATION_SNAPSHOT_RE.sub("EquationExprSnapshot", text)
    text = text.replace("Mathematical", "Equation")
    text = text.replace("mathematical", "equation")
    text = text.replace("MATHEMATICAL", "EQUATION")
    for orig, placeholder in PROTECT:
        text = text.replace(placeholder, orig)
    return text


def main() -> int:
    changed = 0
    scanned = 0
    for dirpath, dirnames, filenames in os.walk(ROOT):
        dirnames[:] = [d for d in dirnames if d not in SKIP_DIR_NAMES]
        for fn in filenames:
            full = os.path.join(dirpath, fn)
            if not should_process(full):
                continue
            scanned += 1
            try:
                with open(full, "r", encoding="utf-8") as f:
                    original = f.read()
            except UnicodeDecodeError:
                print(f"[skip-binary] {full}", file=sys.stderr)
                continue
            updated = transform(original)
            if updated != original:
                with open(full, "w", encoding="utf-8") as f:
                    f.write(updated)
                changed += 1
    print(f"scanned={scanned} changed={changed}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
