"""🎹️ Independent Python oracle of the `repo cache` argument grammar (standard library only): the operation, flags and valued options, repeated or unknown options, valued options without values, and legal area ids. A refusal is answered as a message key with its variables. Requests arrive as JSON on stdin."""
import json, sys

VALUED = ["--area", "--lang"]
PLAIN = ["--json", "--dry-run"]


def parse(args, ids):
    op = args[0] if args else None
    if op not in ("status", "gc"):
        return {"key": "error.syntax", "variables": {}}
    seen, values, flags, index = set(), {}, {"json": False, "dryRun": False}, 1
    while index < len(args):
        argument = args[index]
        cut = argument.find("=")
        name = argument if cut < 0 else argument[:cut]
        if name in seen:
            return {"key": "error.repeated", "variables": {"option": name}}
        seen.add(name)
        if name in VALUED:
            if cut < 0:
                index += 1
                value = args[index] if index < len(args) else None
            else:
                value = argument[cut + 1:]
            if not value or value.startswith("--"):
                return {"key": "error.value", "variables": {"option": name}}
            values[name] = value
        elif name in PLAIN and cut < 0 and (name != "--dry-run" or op == "gc"):
            flags["json" if name == "--json" else "dryRun"] = True
        else:
            return {"key": "error.option", "variables": {"option": argument}}
        index += 1
    only = values["--area"].split(",") if "--area" in values else None
    for area in dict.fromkeys(only or []):
        if area == "":
            return {"key": "error.areaEmpty", "variables": {}}
        if area not in ids:
            return {"key": "error.area", "variables": {"id": area}}
    return {"parsed": {"operation": op, "json": flags["json"], "dryRun": flags["dryRun"], "only": list(dict.fromkeys(only)) if only is not None else None, "language": values.get("--lang")}}


request = json.load(sys.stdin)
print(json.dumps([parse(args, set(request["ids"])) for args in request["vectors"]]))
