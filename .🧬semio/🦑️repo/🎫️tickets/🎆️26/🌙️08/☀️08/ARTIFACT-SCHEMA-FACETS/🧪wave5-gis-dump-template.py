#!/usr/bin/env python3
from pathlib import Path

ticket = next(Path("/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets").rglob("ARTIFACT-SCHEMA-FACETS"))
base = Path("/Users/ueli/Documents/semio") / "\U0001f4dd\ufe0fs" / "\U0001f50c\ufe0fplugins"
# discover via name fragments
plugins = Path("/Users/ueli/Documents/semio")
# find procedural plugin dir
proc = None
for p in plugins.rglob("📦️glue.rs"):
    if "procedural" in str(p) and "plugins" in str(p):
        proc = p.parents[2]
        break
print("proc", proc)
art = None
for x in (proc / "🗿️artifacts").iterdir():
    if "2d" in x.name:
        art = x
        break
print("art", art)

parts = []
for p in sorted(art.rglob("*")):
    if not p.is_file():
        continue
    rel = str(p.relative_to(art))
    if "schema" in rel and p.suffix in {".rs", ".ts", ".graphql", ".json", ".proto"}:
        parts.append("===== %s =====\n%s\n" % (rel, p.read_text()))
    elif rel.endswith("component.protocol.semio") and "pack" in rel:
        parts.append("===== %s =====\n%s\n" % (rel, p.read_text()))

out = ticket / "🧪wave5-gis-procedural2d-template-dump.txt"
out.write_text("\n".join(parts))
print("schema dump", out.stat().st_size)

chunks = []
for name in ["component_root", "engine", "diff", "mutations"]:
    pass

# find by suffix path parts
root = next(art.glob("*component.rs"))
engine = next((art / "⚙️engine").glob("*component.rs"))
diff = next((art / "🔺️diff").glob("*component.rs"))
mutations = next((art / "🧬️mutations").glob("*component.rs"))
for label, p, trim in [
    ("ROOT", root, None),
    ("ENGINE", engine, -5000),
    ("DIFF", diff, None),
    ("MUTATIONS", mutations, 12000),
]:
    text = p.read_text()
    if trim is not None:
        text = text[trim:] if trim < 0 else text[:trim]
    chunks.append("===== %s =====\n%s\n" % (label, text))

glue = next((proc / "📦️packages").rglob("📦️glue.rs"))
gtext = glue.read_text()
idx = gtext.find("pub mod artifacts")
chunks.append("===== GLUE =====\n%s\n" % gtext[idx:idx+5000])
ts = next((proc / "📦️packages").rglob("📦️index.ts"))
chunks.append("===== TS =====\n%s\n" % ts.read_text())

out2 = ticket / "🧪wave5-gis-procedural2d-runtime-dump.txt"
out2.write_text("\n".join(chunks))
print("runtime dump", out2.stat().st_size)
