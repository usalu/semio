"""🔎 Every flat fixture role reference in the norm and block plugins resolves to a committed file, and no nested role path is left behind.

`python -I verify_refs.py`: checks `include_str!` paths, literal `shared://🧬️mutations/…` URIs, templated feature outlines (`<dir>`, `<fixture>` expanded with every Examples row) and the absence of the nested spellings.
"""
import os, pathlib, re, sys

REPO = pathlib.Path(__file__).resolve().parents[9]
PLUGINS = REPO / "✏️s/🔌️plugins"
ROLE = r"🔣️\.(?:before|after|mutation|outcome|diff)\.json"
NESTED = re.compile(r"📸️snapshot/(?:⬅️before|➡️after)/🔣️\.json|🦠️mutation/🔣️\.json|🎯️outcome/🔣️\.json")
INCLUDE = re.compile(r'include_(?:str|bytes)!\(\s*"([^"]*' + ROLE + ')"')
SHARED = re.compile(r"shared://(🧬️mutations/[^\s\"'`]*" + ROLE + ")")
SKIP = {"target", "node_modules", "__pycache__"}


def subset_of(path):
    parts = path.parts
    index = parts.index("🪆️subsets")
    return pathlib.Path(*parts[: index + 2])


def table(lines, start):
    rows = []
    for line in lines[start:]:
        if not line.strip().startswith("|"):
            if rows:
                break
            continue
        rows.append([cell.strip() for cell in line.strip().strip("|").split("|")])
    return rows


def main():
    missing, nested, checked = [], [], 0
    for plugin in ("📕️norm", "🧱️block"):
        for directory, dirs, files in os.walk(PLUGINS / plugin):
            dirs[:] = [d for d in dirs if d not in SKIP]
            if directory.endswith("🧪️tests/🧫️fixtures/🧬️mutations"):
                dirs[:] = []
                continue
            for name in files:
                path = pathlib.Path(directory) / name
                if path.suffix not in {".rs", ".py", ".ts", ".feature", ".json"}:
                    continue
                try:
                    text = path.read_text(encoding="utf-8")
                except UnicodeDecodeError:
                    continue
                if path.suffix in {".rs", ".py", ".feature"} and "🧪️tests" in path.parts:
                    for line_number, line in enumerate(text.split("\n"), 1):
                        if NESTED.search(line) and "🧬️schema/" not in line and "<kind>" not in line:
                            nested.append((str(path.relative_to(PLUGINS)), line_number, line.strip()[:140]))
                for match in INCLUDE.finditer(text):
                    checked += 1
                    if not (path.parent / match.group(1)).resolve().is_file():
                        missing.append((str(path.relative_to(PLUGINS)), match.group(1)))
                if "🪆️subsets" not in path.parts:
                    continue
                fixtures = subset_of(path) / "🧪️tests/🧫️fixtures"
                lines = text.split("\n")
                for index, line in enumerate(lines):
                    for match in SHARED.finditer(line):
                        target = match.group(1)
                        if "<" not in target:
                            checked += 1
                            if not (fixtures / target).is_file():
                                missing.append((str(path.relative_to(PLUGINS)), target))
                            continue
                        for later in range(index, len(lines)):
                            if lines[later].strip().startswith("Examples:"):
                                rows = table(lines, later + 1)
                                break
                        else:
                            continue
                        head = rows[0]
                        for row in rows[1:]:
                            values = dict(zip(head, row))
                            expanded = target
                            for key, value in values.items():
                                expanded = expanded.replace("<" + key + ">", value)
                            checked += 1
                            if "<" in expanded:
                                continue
                            if not (fixtures / expanded).is_file():
                                missing.append((str(path.relative_to(PLUGINS)), expanded))
    print("references checked", checked, "missing", len(missing), "nested spellings left", len(nested))
    for row in missing[:400]:
        print("  MISSING", row)
    for row in nested[:25]:
        print("  NESTED", row)
    return 1 if missing or nested else 0


if __name__ == "__main__":
    sys.exit(main())
