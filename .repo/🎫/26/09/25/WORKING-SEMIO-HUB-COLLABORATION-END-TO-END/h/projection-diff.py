"""🔬 Reports every input kit projection path missing from or different in an exported projection (entity arrays matched by id, `hash` keys ignored). Usage: `python3 projection-diff.py <input.json> <export.json>` (files may wrap the kit as `{kit}` / `{name, kit}`)."""
import collections
import json
import re
import sys


def load(path):
    value = json.load(open(path))
    return value.get("kit", value) if isinstance(value, dict) and "kit" in value and isinstance(value["kit"], dict) else value


def items(value):
    return value.get("items") if isinstance(value, dict) and isinstance(value.get("items"), list) else value


missing, different, extra = collections.Counter(), collections.Counter(), collections.Counter()
examples = {}


def pattern(path):
    return re.sub(r"\[[^\]]*\]", "[]", path)


def walk(a, b, path):
    a, b = items(a), items(b)
    if isinstance(a, dict):
        if not isinstance(b, dict):
            different[pattern(path)] += 1
            examples.setdefault(pattern(path), (a, b))
            return
        for key, value in a.items():
            if key == "hash":
                continue
            if key not in b:
                missing[pattern(f"{path}.{key}")] += 1
                examples.setdefault(pattern(f"{path}.{key}"), (value, None))
            else:
                walk(value, b[key], f"{path}.{key}")
        for key in b:
            if key not in a and key != "hash":
                extra[pattern(f"{path}.{key}")] += 1
    elif isinstance(a, list):
        if not isinstance(b, list):
            different[pattern(path)] += 1
            return
        if a and all(isinstance(x, dict) and "id" in x for x in a):
            by_id = {x.get("id"): x for x in b if isinstance(x, dict)}
            for x in a:
                if x["id"] not in by_id:
                    missing[pattern(f"{path}[{x['id']}]")] += 1
                else:
                    walk(x, by_id[x["id"]], f"{path}[{x['id']}]")
        elif a != b:
            different[pattern(path)] += 1
            examples.setdefault(pattern(path), (a[:3], b[:3]))
    elif a != b:
        different[pattern(path)] += 1
        examples.setdefault(pattern(path), (a, b))


walk(load(sys.argv[1]), load(sys.argv[2]), "kit")
for title, counter in (("MISSING", missing), ("DIFFERENT", different), ("EXTRA", extra)):
    print(f"== {title} ({sum(counter.values())})")
    for key, count in sorted(counter.items()):
        print(f"{count:6d} {key}  e.g. {json.dumps(examples.get(key))[:160] if key in examples else ''}")
