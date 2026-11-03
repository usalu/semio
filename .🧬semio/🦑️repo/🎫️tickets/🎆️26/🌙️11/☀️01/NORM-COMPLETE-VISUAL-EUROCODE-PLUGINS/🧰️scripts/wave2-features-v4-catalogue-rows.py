"""Wave 2 features v4 (input script): appends catalogue-example rows for the registry entries a catalogue feature does not cover yet. Inputs per entry = the documented keys of the Rust registry (`inputs: &[...]`) with the example values of the catalogue-bridge tsv; an entry is skipped when a key has no example or the independent Python oracle rejects the case. Expected values are the oracle's. Usage: python -I wave2-features-v4-catalogue-rows.py <artifact dir name> <tsv> <annex> [--write]"""
import glob, importlib.util, pathlib, re, sys

art, tsv, annex = sys.argv[1], sys.argv[2], sys.argv[3]
write = "--write" in sys.argv
base = pathlib.Path("✏️s/🔌️plugins/📕️norm/🗿️artifacts") / art / "🏅️standards/🔖️1/🪆️subsets/✳️any"
oracle_path = base / "🔮️oracles/⚖️compliance/🐍️.py"
spec = importlib.util.spec_from_file_location("oracle_mod", oracle_path)
mod = importlib.util.module_from_spec(spec)
sys.modules["oracle_mod"] = mod
spec.loader.exec_module(mod)
examples = {}
choices = {}
aliases = {}
for line in open(tsv, encoding="utf8"):
    if line.startswith("#") or not line.strip():
        continue
    if line.startswith("@alias"):
        alias, base_key = line.strip().split(" ", 1)[1].split("|")
        aliases[alias] = base_key
        continue
    if line.startswith("@choice"):
        _, key = line.split("|")[0].split(" ", 1)
        choices.setdefault(key, line.split("|")[1])
        continue
    key, scope, kind, unit, example, *_ = line.rstrip("\n").split("|")
    examples.setdefault(key, []).append((set(filter(None, scope.split(","))), kind, example))


def example_for(key, entry):
    key = aliases.get(key, key)
    rows = examples.get(key, [])
    scoped = [r for r in rows if entry in r[0]]
    open_rows = [r for r in rows if not r[0]]
    pick = (scoped or open_rows or [None])[0]
    if pick and pick[2] == "" and pick[1] != "n" and key in choices:
        return (pick[0], pick[1], choices[key])
    return pick if pick and pick[2] != "" else None


registry = {}
for rs in glob.glob(str(base / "**/*catalogue*/**/🦀️.rs"), recursive=True):
    if "🧪️tests" in rs or "editor" in rs:
        continue
    text = open(rs, encoding="utf8").read()
    for match in re.finditer(r'Entry \{ id: "([^"]+)".*?inputs: &\[([^\]]*)\]', text):
        registry[match[1]] = re.findall(r'"([^"]+)"', match[2])
feature = next(pathlib.Path(p) for p in glob.glob(str(base / "🧪️tests/*catalogue*/🥒️.feature")))
body = feature.read_text(encoding="utf8")
covered = {entry for entry in registry if re.search(r"[\s|]" + re.escape(entry) + r"\s*\|", body)}
rows, skipped = [], {}
for entry in sorted(set(registry) - covered):
    n, s, parts = {}, {}, []
    missing = [key for key in registry[entry] if example_for(key, entry) is None]
    if missing:
        skipped[entry] = "no example for " + ",".join(missing)
        continue
    for key in registry[entry]:
        _, kind, example = example_for(key, entry)
        if kind == "n":
            n[key] = float(example)
            parts.append("%s=%s" % (key, example))
        else:
            s[key] = example
            parts.append("%s:%s" % (key, example))
    try:
        out = mod.ev(entry, annex, n, s, {})
    except Exception as error:
        skipped[entry] = "oracle: %s %s" % (type(error).__name__, str(error)[:60])
        continue
    for key, value in out.items():
        value = float(value)
        if value != value or abs(value) == float("inf"):
            continue
        slug = re.sub(r"[^a-z0-9]+", "-", entry.lower()).strip("-")
        rows.append("      | %s-%s-%s | %s | %s | %s | %s | %.12g | 1e-6 |" % (slug, re.sub(r"[^a-z0-9]+", "-", key.lower()).strip("-"), annex, entry, annex, ";".join(parts), key, value))
print(len(rows), "rows for", len({r.split("|")[2] for r in rows}), "entries; skipped", len(skipped))
for entry, why in skipped.items():
    print("  skip", entry, why)
if write and rows:
    lines = body.rstrip("\n").split("\n")
    last = max(i for i, line in enumerate(lines) if re.match(r"\s+\|.*\|\s*$", line))
    lines[last + 1 : last + 1] = rows
    feature.write_text("\n".join(lines) + "\n", encoding="utf8")
