import json
import re
import sys

LINE = re.compile(r'^(\s*"[a-z0-9-]+": )(\{ "asset".*\})(,?)(\s*)$')


def fmt(value):
    if isinstance(value, dict):
        return "{ " + ", ".join(f'{json.dumps(k)}: {fmt(v)}' for k, v in value.items()) + " }"
    if isinstance(value, list):
        return "[" + ", ".join(fmt(v) for v in value) + "]"
    return json.dumps(value, ensure_ascii=False)


def fix(entry):
    entry = dict(entry)
    entry.setdefault("scale", 50)
    if ("clip" in entry or "timeline" in entry) and "fps" not in entry:
        entry["fps"] = 15
    if "timeline" in entry:
        entry["timeline"] = [{**{k: key[k] for k in ("t", "environment")}, "ease": key.get("ease", "linear")} for key in entry["timeline"]]
    order = ["asset", "pose", "clip", "view", "viewpoint", "theme", "scale", "fps"]
    head = {k: entry[k] for k in order if k in entry}
    return {**head, **{k: v for k, v in entry.items() if k not in head}}


for path in sys.argv[1:]:
    raw = open(path, encoding="utf-8", newline="").read()
    lines = raw.split("\n")
    changed = 0
    for i, line in enumerate(lines):
        crlf = line.endswith("\r")
        m = LINE.match(line.rstrip("\r"))
        if not m:
            continue
        entry = json.loads(m.group(2))
        fixed = fix(entry)
        if fixed == entry:
            continue
        keep = {k: v for k, v in entry.items()}
        new = fixed if list(fixed) != list(entry) and False else {**entry, **{k: fixed[k] for k in fixed if k not in entry}, **({"timeline": fixed["timeline"]} if "timeline" in fixed else {})}
        lines[i] = m.group(1) + fmt(new) + m.group(3) + m.group(4).rstrip("\r") + ("\r" if crlf else "")
        changed += 1
    open(path, "w", encoding="utf-8", newline="").write("\n".join(lines))
    print(path, changed)
