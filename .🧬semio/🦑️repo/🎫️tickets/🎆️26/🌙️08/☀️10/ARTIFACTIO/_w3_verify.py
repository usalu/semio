from pathlib import Path
import json

ticket = Path(__file__).resolve().parent
root = ticket
while not (root / "📜️script.ts").exists():
    root = root.parent
print("repo", root)

rust = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs"
t = rust.read_text()
assert 'let io_dir = "🚪️io"' in t, "rust missing io_dir"
assert "IoFacetCompleteness" in t, "rust missing region"
assert "*component == io_dir" in t, "rust missing io filter"
print("rust ok")

ts_pkg = root / "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🟦️typescript"
reg = next(x for x in ts_pkg.iterdir() if x.name.endswith("registry"))
rt = (reg / "📜️script.ts").read_text()
assert "IO_FACET_DIR" in rt
assert "component === IO_FACET_DIR" in rt
assert "TAXONOMY_IO_FORMAT_CHILD_DIRS" in rt
print("registry ok")

tax = json.loads((root / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json").read_text())
assert "ARTIFACT-IO-FACETS" in tax["_comment"]
assert "ARTIFACT-IO-FACETS" in tax["_engineListComment"]
assert "🚪️io" in tax["artifactComponentDirs"]
assert "🚪️io" in tax["artifactChildDirs"]
assert tax["ioFormatChildDirs"] == ["📥️import", "📤️export"]
assert "📥️import" in tax["taxonomyLeafParentDirs"]
assert "📤️export" in tax["taxonomyLeafParentDirs"]
owner = json.loads((ticket / "🧪owner-table.json").read_text())
catalog = {k: v["dir"] for k, v in owner["catalog_formats"].items()}
assert tax["mediaFormatDirs"] == catalog, (set(tax["mediaFormatDirs"].items()) ^ set(catalog.items()))
print("taxonomy ok; formats", len(catalog))

disc = (root / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts").read_text()
assert 'facet === "🚪️io"' in disc
assert "ioFormatChildDirs" in disc
assert "mediaFormatDirs" in disc
assert "IoFacetContract" in disc
print("discovery ok")

script = (root / "📜️script.ts").read_text()
assert "export function policyArtifactIoBreaches" in script
assert "artifact-io/facet-completeness" in script
assert 'child.name === "🚪️io"' in script
print("policy ok")
print("ALL STATIC CHECKS PASSED")
