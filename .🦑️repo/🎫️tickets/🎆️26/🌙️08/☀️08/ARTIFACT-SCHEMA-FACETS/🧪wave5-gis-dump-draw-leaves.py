from pathlib import Path
draw = next(p for p in Path("/Users/ueli/Documents/semio/✏️s/🔌️plugins").iterdir() if "draw" in p.name)
dart = next((draw/"🗿️artifacts").iterdir())
out = []
for p in sorted(dart.rglob("*")):
    if not p.is_file():
        continue
    rel = str(p.relative_to(dart))
    if "schema" in rel and p.suffix in {".graphql", ".proto", ".ts", ".json", ".rs"}:
        if "diff" in rel or rel.startswith("🧬️schema"):
            out.append("==== %s ====\n%s\n" % (rel, p.read_text()))
mut = (dart/"🧬️mutations"/"🦀️component.rs").read_text()
i = mut.find("impl Mutation")
out.append("==== MUTATION IMPL ====\n%s\n" % mut[i:i+4000])
glue = next((draw/"📦️packages").rglob("📦️glue.rs")).read_text()
i = glue.find("pub mod snapshot")
out.append("==== SNAPSHOT GLUE ====\n%s\n" % glue[i:i+900])
# also how collection_diff maps to ObjectsDelta in draw diff runtime
diff = (dart/"🔺️diff"/"🦀️component.rs").read_text()
out.append("==== DIFF RT ====\n%s\n" % diff)
ticket = next(Path("/Users/ueli/Documents/semio/.🦑️repo/🎫️tickets").rglob("ARTIFACT-SCHEMA-FACETS"))
path = ticket/"🧪wave5-gis-draw-leaves.txt"
path.write_text("\n".join(out))
print(path, path.stat().st_size)
