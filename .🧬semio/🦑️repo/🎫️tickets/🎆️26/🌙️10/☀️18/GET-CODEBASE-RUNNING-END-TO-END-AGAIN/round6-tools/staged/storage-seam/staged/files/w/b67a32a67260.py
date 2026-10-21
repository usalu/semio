"""Independent oracle of the native storage backend selection: reads only the schema JSON and the language-agnostic rows, shares no code with the TypeScript implementation, prints one JSON object per row id -> chosen backend id or refusal key."""
import json, sys
schema = json.load(open(sys.argv[1]))
rows = json.load(open(sys.argv[2]))["rows"]
protocol = schema["definitions"]["Protocol"]["const"]
platforms = schema["definitions"]["Platform"]["enum"]
capabilities = schema["definitions"]["Capability"]["enum"]
backends = protocol["backends"]


def choose(selection):
    platform = selection.get("platform")
    if platform not in platforms:
        return "!platform"
    require = selection.get("require", [])
    prefer = selection.get("prefer", protocol["preferred"])
    if any(name not in capabilities for name in list(require) + list(prefer)):
        return "!capability"
    hosted = [b for b in backends if platform in b["platforms"]]
    if "backend" in selection:
        match = [b for b in backends if b["id"] == selection["backend"]]
        if not match or match[0] not in hosted:
            return "!unavailable"
        return match[0]["id"] if set(require) <= set(match[0]["capabilities"]) else "!unsatisfied"
    pool = [b for b in hosted if set(require) <= set(b["capabilities"])]
    if not pool:
        return "!unsatisfied"
    ranked = sorted(range(len(pool)), key=lambda i: (-len([n for n in prefer if n in pool[i]["capabilities"]]), i))
    return pool[ranked[0]]["id"]


print(json.dumps({row["id"]: choose(row["selection"]) for row in rows}, sort_keys=True))
