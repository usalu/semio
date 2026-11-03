"""🧭 One-shot: moves the ticket's fixture generators to the flat role files, on lines that write or read a mutation vector (`✅apply`, `art.fix`, fixtures) and never on schema facet paths.

`python -I rewrite_generators.py [--apply]` prints every changed line for review.
"""
import os, pathlib, re, sys

TICKET = pathlib.Path(__file__).resolve().parents[2]
ROLE = re.compile(r"(?:📸️snapshot/)?(?:(⬅️before)|(➡️after))/🔣️\.json|(🦠️mutation|🎯️outcome|🔺️diff)/🔣️\.json")
NAMES = {"⬅️before": "before", "➡️after": "after", "🦠️mutation": "mutation", "🎯️outcome": "outcome", "🔺️diff": "diff"}
CONTEXT = re.compile(r"✅apply|art\.fix|🧫️fixtures|fixtures|\bfix\b|\bdst\b|🧬️mutations/")
SCHEMA = re.compile(r"🧬️schema|\bSCH\b|\bSCHEMA\b|DIFF =|ANY \+")


def flat(match):
    return "🔣️." + NAMES[match.group(1) or match.group(2) or match.group(3)] + ".json"


def main(argv):
    apply = "--apply" in argv
    files = 0
    for directory, dirs, names in os.walk(TICKET):
        dirs[:] = [d for d in dirs if d not in {"🗑️generated", "__pycache__", "path-budget"}]
        for name in names:
            if not name.endswith((".py", ".sh", ".ts", ".mjs")):
                continue
            path = pathlib.Path(directory) / name
            try:
                text = path.read_text(encoding="utf-8")
            except UnicodeDecodeError:
                continue
            out, shown = [], []
            for number, line in enumerate(text.split("\n"), 1):
                new = ROLE.sub(flat, line) if CONTEXT.search(line) and not SCHEMA.search(line) else line
                if new != line:
                    shown.append((number, new.strip()[:150]))
                out.append(new)
            if shown:
                files += 1
                print(path.relative_to(TICKET), len(shown))
                for number, line in shown[:3]:
                    print("   ", number, line)
                if apply:
                    path.write_text("\n".join(out), encoding="utf-8", newline="")
    print("files", files, "applied" if apply else "dry-run")


if __name__ == "__main__":
    main(sys.argv[1:])
