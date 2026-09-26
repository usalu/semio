"""🧮️ D1 offline census over the committed plugin descriptors — mirrors `🗂️catalog/🦀️.rs` `compile` +
`audit_source` exactly: per app, the distinct verbs (window-kind actions first, then app-scope actions),
framework-injected ids skipped; app/mode/plugin commands; audience = declared or derived."""
import json, os, sys, collections
ROOT = "/Users/ueli/Documents/semio"
REG = json.load(open(f"{ROOT}/🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/🤖️generated/🔌️plugins.json"))
FRAMEWORK_VIEW_SHELL = {"setActiveUtility", "setActiveTool", "startIntroduction", "setHistoryCommandFilter", "noteShellCommand"}

def owner_root(entry):
    crate = entry["cratePath"]
    return os.path.join(ROOT, os.path.dirname(os.path.dirname(crate)))

def audience(row):
    declared = row.get("semantics", {}).get("audience")
    if declared:
        return declared, True
    kind = row["kind"]
    if kind == "interaction":
        return "input", False
    if kind == "view" and not row.get("inPalette", False):
        return "chrome", False
    return "agent", False

def framework_injected(action):
    return action["kind"] in ("history", "clipboard", "interaction") or action["id"] in FRAMEWORK_VIEW_SHELL

def label(row, lang):
    lab = row.get("label") or {}
    native = lab.get("native") or {}
    return native.get(lang, "")

def description(row, lang):
    d = row.get("semantics", {}).get("description")
    if not d:
        return ""
    return (d.get("native") or {}).get(lang, "")

def rows():
    for entry in REG:
        path = os.path.join(owner_root(entry), "🔣️.json")
        descriptor = json.load(open(path))
        manifest = descriptor["manifest"]
        plugin = manifest["pluginId"]
        for app in manifest.get("apps", []):
            seen = {}
            verbs = []
            for wk in app.get("windowKinds", []):
                for action in wk.get("actions", []):
                    if action["id"] in seen:
                        continue
                    seen[action["id"]] = action
                    verbs.append((action, wk["id"]))
            for action in app.get("actions", []):
                if action["id"] in seen:
                    continue
                seen[action["id"]] = action
                verbs.append((action, "*"))
            for action, wk in verbs:
                if framework_injected(action):
                    continue
                yield plugin, app["id"], f"{plugin}.{app['id']}.{action['id']}", "action", action
            for command in app.get("commands", []):
                yield plugin, app["id"], f"{plugin}.{app['id']}.cmd.{command['id']}", "command", command
            for mode in app.get("modes", []):
                for command in mode.get("commands", []):
                    yield plugin, app["id"], f"{plugin}.{app['id']}.mode.{mode['id']}.{command['id']}", "command", command
        for command in manifest.get("commands", []):
            yield plugin, None, f"{plugin}.cmd.{command['id']}", "command", command

if __name__ == "__main__":
    per_plugin = collections.defaultdict(lambda: collections.Counter())
    out = []
    for plugin, app, cid, kind, row in rows():
        aud, declared = audience(row)
        en, de = description(row, "en"), description(row, "de")
        per_plugin[plugin][aud] += 1
        if aud == "agent":
            per_plugin[plugin]["agent_described" if en and de else "agent_empty"] += 1
        out.append({"id": cid, "plugin": plugin, "app": app, "shape": kind, "kind": row["kind"], "audience": aud, "declared": declared,
                    "destructive": row.get("semantics", {}).get("effects", {}).get("destructive", False),
                    "title_en": label(row, "en"), "title_de": label(row, "de"), "desc_en": en, "desc_de": de, "inPalette": row.get("inPalette", False)})
    json.dump(out, open(sys.argv[1], "w"), ensure_ascii=False, indent=0)
    total = collections.Counter()
    for plugin in sorted(per_plugin):
        c = per_plugin[plugin]; total.update(c)
        print(f"{plugin:40s} agent={c['agent']:4d} empty={c['agent_empty']:4d} described={c['agent_described']:4d} input={c['input']:4d} chrome={c['chrome']:4d}")
    print("TOTAL", dict(total))
