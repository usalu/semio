#!/usr/bin/env python3
"""🗺️ EX1: every ArtifactEditor of the non-stdio plugins — crate, module file, app id (DIALECT), create fn, published example
ids (its `examples()` or its subset's `📚️examples/` leaves) — as JSON for the per-plugin catalog fixtures and surface laws."""
import json, os, re, sys
R = "/Users/ueli/Documents/semio/✏️s/🔌️plugins"
out = []
for plugin in sorted(os.listdir(R)):
    if plugin.startswith(("🗄️", ".")) or not os.path.isdir(os.path.join(R, plugin)):
        continue
    texts = {}
    for dp, dns, fns in os.walk(os.path.join(R, plugin)):
        dns[:] = [d for d in dns if d not in ("🧪️tests", "target", "node_modules")]
        for fn in fns:
            if fn.endswith(".rs"):
                texts[os.path.join(dp, fn)] = open(os.path.join(dp, fn), encoding="utf-8").read()
    alltext = "\n".join(texts.values())
    for path, text in texts.items():
        for m in re.finditer(r"impl (?:semio_framework_plugin::)?ArtifactEditor for (\w+) \{", text):
            name = m.group(1)
            depth, at = 1, m.end()
            while depth and at < len(text):
                depth += {"{": 1, "}": -1}.get(text[at], 0)
                at += 1
            impl = text[m.end():at]
            dc = re.search(r"const DIALECT: (?:semio_framework_plugin::)?Dialect = ([^;]+);", impl) or re.search(r"const DIALECT: (?:semio_framework_plugin::)?Dialect = ([^;]+);", text)
            app = None
            if dc:
                expr = dc.group(1).strip()
                if re.match(r"[\w:]*Dialect \{", expr):
                    body = expr
                else:
                    const = expr.split("::")[-1]
                    d = re.search(rf"{const}: [\w:]*Dialect = ([^;]+);", alltext)
                    body = d.group(1) if d else ""
                k = re.search(r'artifact_kind: "?([\w.-]+)"?', body)
                st = re.search(r'StandardId\("([^"]+)"\)', body)
                sb = re.search(r'SubsetId\("([^"]+)"\)', body)
                if k and st:
                    kind = k.group(1)
                    if "." not in kind:
                        kc = re.search(rf'const {kind}: &str = "([^"]+)"', alltext)
                        kind = kc.group(1) if kc else kind
                    app = f"{kind}@{st.group(1)}/{sb.group(1) if sb else '*'}#editor"
            ex = re.search(r"fn examples\(\) -> Vec<[\w:]*ExampleSource> \{\s*(.*?)\s*\}\n", impl, re.S)
            ids = None
            if ex:
                ids = []
                for module in [x.split("::")[-1] for x in re.findall(r"([\w:]+)::source\(\)", ex.group(1))]:
                    leaf = re.search(rf'#\[path = "([^"]*📚️examples/[^"]*)"\]\s*pub mod {module};', alltext)
                    found = None
                    if leaf:
                        leafdir = leaf.group(1).split("📚️examples/")[1].split("/")[0]
                        for candidate, ctext in texts.items():
                            if f"/📚️examples/{leafdir}/🦀️.rs" in candidate:
                                i = re.search(r'pub const ID: &str = "([^"]+)"', ctext)
                                found = i and i.group(1)
                    ids.append(found or "~" + module.replace("_", "-"))
                if not ids:
                    ids = ["?" + " ".join(ex.group(1).split())[:120]]
            else:
                subset = os.path.dirname(os.path.dirname(path)) if path.endswith("✏️editor/🦀️.rs") else os.path.dirname(path)
                leaves = os.path.join(subset, "📚️examples")
                ids = []
                if os.path.isdir(leaves):
                    for leaf in sorted(os.listdir(leaves)):
                        src = os.path.join(leaves, leaf, "🦀️.rs")
                        if os.path.isfile(src):
                            i = re.search(r'pub const ID: &str = "([^"]+)"', open(src, encoding="utf-8").read())
                            if i: ids.append("=" + i.group(1))
            create = re.findall(r"pub fn (create_\w+)\(\) -> (?:semio_framework_plugin::)?AppDefinition", text)
            out.append({"plugin": plugin, "editor": name, "file": os.path.relpath(path, R), "app": app, "create": create, "examples": ids})
json.dump(out, sys.stdout, ensure_ascii=False, indent=1)
