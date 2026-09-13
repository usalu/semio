#!/usr/bin/env python3
"""🔭️ Move every remaining `Viewport2d` consumer forward onto `semio_framework_os_kernel`.

The type now lives in `semio-framework-ui-viewport` and is re-exported only by
`semio_framework_os_kernel` (`🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs:344`);
`semio_framework` and `semio_framework_plugin` no longer name it. No re-export is added
back into the old crates — every call site is rewritten onto the new home.
"""

import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[7]
SCAN = [ROOT / "✏️s", ROOT / "🧰️framework"]
NEW = "semio_framework_os_kernel::Viewport2d"
FORWARD_USE = "use semio_framework_os_kernel::Viewport2d;\n"

BRACE = re.compile(r"use (semio_framework|semio_framework_plugin)::\{", re.MULTILINE)


def rewrite(text: str) -> str:
    text = text.replace("semio_framework::Viewport2d", NEW)
    text = text.replace("semio_framework_plugin::Viewport2d", NEW)
    lines = text.split("\n")
    out = []
    i = 0
    injected = False
    while i < len(lines):
        line = lines[i]
        if BRACE.search(line) and "}" not in line:
            block = [line]
            j = i + 1
            while j < len(lines):
                block.append(lines[j])
                if lines[j].strip().startswith("};") or lines[j].strip() == "}":
                    break
                j += 1
            blob = "\n".join(block)
            if re.search(r"(?<![A-Za-z0-9_])Viewport2d(?![A-Za-z0-9_])", blob):
                blob = re.sub(r"(?<![A-Za-z0-9_])Viewport2d, ", "", blob)
                blob = re.sub(r", (?<![A-Za-z0-9_])Viewport2d(?![A-Za-z0-9_])", "", blob)
                out.extend(blob.split("\n"))
                out.append(FORWARD_USE.rstrip("\n"))
                injected = True
            else:
                out.extend(block)
            i = j + 1
            continue
        if BRACE.search(line) and "}" in line and re.search(r"(?<![A-Za-z0-9_])Viewport2d(?![A-Za-z0-9_])", line):
            line = re.sub(r"(?<![A-Za-z0-9_])Viewport2d, ", "", line)
            line = re.sub(r", (?<![A-Za-z0-9_])Viewport2d(?![A-Za-z0-9_])", "", line)
            out.append(line)
            out.append(FORWARD_USE.rstrip("\n"))
            injected = True
            i += 1
            continue
        out.append(line)
        i += 1
    _ = injected
    return "\n".join(out)


def main() -> int:
    touched = []
    for base in SCAN:
        for path in base.rglob("*.rs"):
            original = path.read_text(encoding="utf-8")
            if "Viewport2d" not in original:
                continue
            updated = rewrite(original)
            if updated != original:
                path.write_text(updated, encoding="utf-8")
                touched.append(str(path.relative_to(ROOT)))
    for name in sorted(touched):
        print(name)
    print(f"touched={len(touched)}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
