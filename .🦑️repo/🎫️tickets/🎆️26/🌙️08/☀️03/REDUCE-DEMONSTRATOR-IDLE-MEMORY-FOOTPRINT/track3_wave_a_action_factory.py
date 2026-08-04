#!/usr/bin/env python3
"""Track 3 Wave A item #8: replaces each app's hand-rolled single-line
`ActionDescriptor { controller_id: X_CONST.into(), action: action.into(), args:
[semio_framework_plugin::]optional_json_to_dsl(args) }` body with
`semio_framework_plugin::ActionFactory::new(X_CONST).action(action, args)` — call sites (the
wrapper function's own name) are untouched since only the body changes. Only touches the
fixed-CONST-controller-id shape (skips the few crates using a runtime `controller_id` parameter or
a differently-shaped args encoding — those need individual handling, not this mechanical pass)."""
import re
import subprocess

ROOT = "/Users/ueli/Documents/semio/"

LINE_RE = re.compile(
    r'^(?P<indent>[ \t]*)ActionDescriptor \{ controller_id: (?P<const>[A-Z][A-Z0-9_]*)\.into\(\), action: action\.into\(\), '
    r'args: (?:semio_framework_plugin::)?optional_json_to_dsl\(args\) \}$'
)


def process(path):
    with open(path, "r", encoding="utf-8") as f:
        lines = f.readlines()
    n = 0
    for i, line in enumerate(lines):
        m = LINE_RE.match(line.rstrip("\n"))
        if not m:
            continue
        indent, const = m.group("indent"), m.group("const")
        lines[i] = f"{indent}semio_framework_plugin::ActionFactory::new({const}).action(action, args)\n"
        n += 1
    if n:
        with open(path, "w", encoding="utf-8") as f:
            f.writelines(lines)
    return n


def main():
    result = subprocess.run(
        ["grep", "-rl", "-E", r"ActionDescriptor \{ controller_id: [A-Z][A-Z0-9_]*\.into\(\)", ROOT + "✏️s"],
        capture_output=True, text=True,
    )
    files = [f for f in result.stdout.splitlines() if f.endswith(".rs") and "node_modules" not in f]
    total = 0
    for f in sorted(files):
        n = process(f)
        rel = f[len(ROOT):]
        if n:
            print(f"  FIXED ({n}): {rel}")
            total += n
        else:
            print(f"  NOOP: {rel}")
    print(f"\ntotal replacements: {total} across {len(files)} files")


if __name__ == "__main__":
    main()
