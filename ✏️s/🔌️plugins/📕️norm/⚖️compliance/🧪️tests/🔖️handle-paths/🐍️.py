"""🧭 Every handle write path literal in a scene dir resolves to an edit rule (`SetFieldRule`) of its artifact, directly, member-relative or as the entity container of one.

`python -I 🐍️.py` exits non-zero and lists the uncovered paths.
"""
import pathlib, re, sys
ARTIFACTS = pathlib.Path(__file__).resolve().parents[3] / "🗿️artifacts"
LITERAL = re.compile(r'"([A-Za-z]+\[(?:id|index)?=?\{[^}]*\}\](?:\.[A-Za-z]+(?:\[[^\]]*\])?)+)"')
RULE = re.compile(r'SetFieldRule \{ path: "([^"]+)"')

def covered(rules):
    paths = set(RULE.findall(rules))
    return paths | {c[m.end():] for c in paths for m in re.finditer(r"\]\.", c)}

def uncovered(subset):
    heads = covered("".join(f.read_text(encoding="utf-8") for f in (subset / "🧬️schema/🧬️mutations/🧭️edit-rules").rglob("*.rs")))
    found = {}
    for file in subset.rglob("*.rs"):
        parts = file.relative_to(subset).parts
        if "🧪" in str(file.relative_to(subset)) or not any("🎬" in p or "scene" in p for p in parts): continue
        for m in LITERAL.finditer(file.read_text(encoding="utf-8")):
            p = re.sub(r"\[[^\]]*\]", "[]", m.group(1))
            if not any(c == p or c.startswith(p + ".") or c.startswith(p + "[].") for c in heads):
                found.setdefault(p, file)
    return found

def main():
    bad = 0
    for subset in sorted(ARTIFACTS.glob("*/🏅️standards/🔖️1/🪆️subsets/✳️any")):
        if not (subset / "🧬️schema/🧬️mutations/🧭️edit-rules/🦀️.rs").exists(): continue
        for p, f in sorted(uncovered(subset).items()):
            print("uncovered handle path", p, f.relative_to(ARTIFACTS)); bad += 1
    if not bad: print("ok: every handle write path has an edit rule")
    return 1 if bad else 0

if __name__ == "__main__":
    sys.exit(main())
