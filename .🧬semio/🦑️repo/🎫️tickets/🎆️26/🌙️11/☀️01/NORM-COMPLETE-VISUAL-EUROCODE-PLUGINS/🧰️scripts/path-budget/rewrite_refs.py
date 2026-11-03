"""🧭 One-shot: rewrite the fixture role paths in text files from the nested layout to the flat one.

Nested `<case>/📸️snapshot/⬅️before/🔣️.json`, `<case>/🦠️mutation/🔣️.json` become flat `<case>/🔣️.before.json`, `<case>/🔣️.mutation.json`.

`python -I rewrite_refs.py [--apply] [--skip-schema] <root>...` prints every changed line; lines naming a schema folder are only reported.
"""
import os, re, sys

ROLE = re.compile(r"(?:📸️snapshot/)?(?:(⬅️before)|(➡️after))/🔣️\.json|(🦠️mutation|🎯️outcome|🔺️diff)/🔣️\.json")
NAMES = {"⬅️before": "before", "➡️after": "after", "🦠️mutation": "mutation", "🎯️outcome": "outcome", "🔺️diff": "diff"}
ABSENT = re.compile(r"🔺️diff/🚫️\.absent")
SKIP_DIRS = {"target", "node_modules", "__pycache__", ".git"}
EXT = (".rs", ".py", ".ts", ".feature", ".json", ".md", ".sh", ".mjs")


def flat(match):
    role = match.group(1) or match.group(2) or match.group(3)
    return "🔣️." + NAMES[role] + ".json"


def schema_line(line):
    return "🧬️schema/" in line and "🧫️fixtures" not in line and "shared://" not in line


def rewrite(text, skip_schema_lines=True):
    out, changed = [], []
    for number, line in enumerate(text.split("\n"), 1):
        new = ROLE.sub(flat, line)
        if new != line and skip_schema_lines and schema_line(line):
            changed.append((number, "SCHEMA-SKIPPED", line.strip()[:160]))
            new = line
        elif new != line:
            changed.append((number, "ok", new.strip()[:160]))
        out.append(new)
    return "\n".join(out), changed


def main(argv):
    apply = "--apply" in argv
    roots = [a for a in argv if not a.startswith("--")]
    total = 0
    for root in roots:
        for directory, dirs, files in os.walk(root):
            dirs[:] = [d for d in dirs if d not in SKIP_DIRS]
            if directory.endswith("🧪️tests/🧫️fixtures/🧬️mutations"):
                dirs[:] = []
                continue
            for name in files:
                if not name.endswith(EXT):
                    continue
                path = os.path.join(directory, name)
                try:
                    text = open(path, encoding="utf-8").read()
                except (UnicodeDecodeError, OSError):
                    continue
                new, changed = rewrite(text)
                if not changed:
                    continue
                print(path, len([c for c in changed if c[1] == "ok"]), "changed", len([c for c in changed if c[1] != "ok"]), "skipped")
                for number, kind, line in changed:
                    if kind != "ok" or "--verbose" in argv:
                        print("   ", number, kind, line)
                total += 1
                if apply and new != text:
                    open(path, "w", encoding="utf-8", newline="").write(new)
    print("files", total, "applied" if apply else "dry-run")


if __name__ == "__main__":
    main(sys.argv[1:])
