"""🧮️ M5b — per plugin, which agent-published verbs lack a description, lack destructive marking by
id lexicon, or carry typed payload fields that the descriptor does not declare as args."""
import json, glob, re, sys, os
PLUGINS = sys.argv[1:] or ["draw", "note", "raster", "layout", "forms", "cad", "gis"]
DIRS = {os.path.basename(os.path.dirname(p)): p for p in glob.glob("✏️s/🔌️plugins/*/🔣️.json")}
def plugin_file(pid):
    for name, path in DIRS.items():
        if name.endswith(pid):
            return path
def derive(kind, in_palette):
    return "input" if kind == "interaction" else "chrome" if kind == "view" and not in_palette else "agent"
def payload_fields(root):
    rows = {}
    for rs in glob.glob(root + "/**/🦀️.rs", recursive=True):
        text = open(rs, encoding="utf8").read()
        for m in re.finditer(r'"(\w+)"\s+as\s+"[\w:-]+"\s*=>\s*(\w+)::(\w+)', text):
            rows.setdefault(m.group(1), m.group(3))
    structs = {}
    for rs in glob.glob(root + "/**/🎮️commands/**/🦀️.rs", recursive=True):
        text = open(rs, encoding="utf8").read()
        for m in re.finditer(r'pub struct (\w+)\s*\{([^}]*)\}', text):
            fields = re.findall(r'pub (\w+)\s*:', m.group(2))
            structs.setdefault(m.group(1), fields)
    return {verb: structs.get(payload) for verb, payload in rows.items()}
for pid in PLUGINS:
    path = plugin_file(pid)
    d = json.load(open(path))
    root = os.path.dirname(path)
    fields = payload_fields(root)
    seen = set()
    missing_desc, missing_args = [], []
    agent = 0
    for app in d["manifest"]["apps"]:
        acts = list(app.get("actions", [])) + [a for w in app.get("windowKinds", []) for a in w.get("actions", [])]
        for a in acts:
            s = a.get("semantics", {})
            aud = s.get("audience") or derive(a["kind"], a.get("inPalette", False))
            if aud != "agent" or a["id"] in seen:
                continue
            seen.add(a["id"])
            agent += 1
            if not s.get("description"):
                missing_desc.append(a["id"])
            f = fields.get(a["id"])
            if f and not a.get("args"):
                missing_args.append(f"{a['id']}({','.join(f)})")
    print(f"## {pid}: {agent} agent verbs · {len(missing_desc)} undescribed · {len(missing_args)} typed-but-undeclared")
    print("  undescribed:", " ".join(missing_desc))
    print("  typed-but-undeclared:", " ".join(missing_args))
