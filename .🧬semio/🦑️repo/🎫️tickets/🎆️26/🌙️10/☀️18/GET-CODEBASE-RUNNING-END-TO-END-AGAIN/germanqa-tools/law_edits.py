#!/usr/bin/env python3
"""⚖️ Edit law of lane germanqa (independent): every staged JSON change keeps the structure and all English leaves byte-identical and changes only German leaves;
every staged text/source change differs from its base only inside German-looking segments. usage: python3 -I law_edits.py <staged-dir>..."""
import json, os, re, sys

MARK = re.compile(r"(?:(?<=\s)|^)(EN|DE): ")


def walk(a, b, path, bad, in_de=False):
    if type(a) is not type(b): bad.append((path, "type")); return
    if isinstance(a, dict):
        if set(a) != set(b): bad.append((path, "keys")); return
        env = a.get("environment") if isinstance(a.get("environment"), dict) else a.get("env") if isinstance(a.get("env"), dict) else {}
        german_here = a.get("language") == "de" or a.get("lang") == "de" or any(str(env.get(k, "")).startswith("de") for k in ("SEMIO_LANGUAGE", "LANGUAGE", "LC_ALL", "LC_MESSAGES", "LANG"))
        for k in a: walk(a[k], b[k], path + "/" + k, bad, in_de or (k == "de" and "en" in a) or (german_here and k not in ("language", "lang", "row", "environment", "env", "name")))
    elif isinstance(a, list):
        if len(a) != len(b): bad.append((path, "length")); return
        for i, (x, y) in enumerate(zip(a, b)): walk(x, y, f"{path}/{i}", bad, in_de)
    elif isinstance(a, str):
        if a == b: return
        if in_de: return
        if MARK.search(a) and MARK.search(b):
            ea = [t for k, t in split(a) if k != "DE"]; eb = [t for k, t in split(b) if k != "DE"]
            if ea == eb: return
        pa, pb = a.split(" / "), b.split(" / ")
        if len(pa) == len(pb) and len(pa) > 1 and pa[0] == pb[0] and any(x != y for x, y in zip(pa, pb)) and pa[:len(pa) // 2] == pb[:len(pb) // 2]: return
        bad.append((path, "non-German leaf changed: " + a[:60]))
    elif a != b: bad.append((path, "value"))


def split(text):
    parts, last, kind = [], 0, None
    for m in MARK.finditer(text):
        parts.append((kind, text[last:m.start()])); kind, last = m.group(1), m.end()
    parts.append((kind, text[last:]))
    return parts


def main():
    problems = 0
    for staged in sys.argv[1:]:
        man = json.load(open(os.path.join(staged, "manifest.json"), encoding="utf-8"))
        for rel, row in man.items():
            base = open(os.path.join(staged, "base", row["file"]), "rb").read().decode("utf-8"); new = open(os.path.join(staged, "files", row["file"]), "rb").read().decode("utf-8")
            if rel.endswith(".json"):
                bad = []; walk(json.loads(base), json.loads(new), "", bad)
                for p, why in bad: problems += 1; print("EDIT-LAW", rel[-70:], p, why)
            if base.count("\n") != new.count("\n") and not rel.endswith(".json"): problems += 1; print("EDIT-LAW line count changed", rel[-70:])
    print("LAW edits:", "GREEN" if not problems else f"RED {problems}")
    sys.exit(1 if problems else 0)


main()
