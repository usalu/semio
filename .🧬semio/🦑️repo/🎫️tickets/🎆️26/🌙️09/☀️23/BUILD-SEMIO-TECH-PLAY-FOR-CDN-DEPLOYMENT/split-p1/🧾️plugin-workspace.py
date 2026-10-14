#!/usr/bin/env python3
"""🧾️ Writes the energy plugin workspace root `✏️s/🔌️plugins/🔋️energy/Cargo.toml` (lane P1, slice S1b).

Members: every engine module crate and the epw extension crate. The lints and profiles are copied verbatim from the
artifact workspace root so the two energy workspaces build with one profile set. Rows for the framework crates the
members name, plus one row per member.
"""
import os, re, json
PLUGIN = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🔋️energy"
ART = PLUGIN + "/🗿️artifacts/🔋️model/Cargo.toml"
ENGINE = "🔨️modules/⚡️simulation/⚙️engine"
art = open(ART, encoding="utf-8").read()
lints_profiles = art[art.index("[workspace.lints.rust]"):]
members = sorted(os.path.relpath(r, PLUGIN) for r, _, fs in os.walk(f"{PLUGIN}/{ENGINE}") if r.endswith("📦️packages/🦀️rust") and "Cargo.toml" in fs)
members.append("🧩️extensions/🌦️epw/📦️packages/🦀️rust")
FW = "../../../🧰️framework"
framework = {
    "semio-framework-dsl-record": f"{FW}/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust",
    "semio-framework-dsl-record-derive": f"{FW}/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust",
    "semio-framework-job": f"{FW}/🔨️modules/🧵️job/📦️packages/🦀️rust",
    "semio-framework-pack-json": f"{FW}/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust",
    "semio-framework-plugin": f"{FW}/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
    "semio-framework-value": f"{FW}/🔨️modules/🌱️value/📦️packages/🦀️rust",
    "semio-framework-value-derive": f"{FW}/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust",
    "semio-s-artifact-stdio-epw": "../🗄️stdio/🗿️artifacts/🌦️epw/📦️packages/🦀️rust",
}
rows = []
for member in members:
    name = re.search(r'^name = "([^"]+)"', open(f"{PLUGIN}/{member}/Cargo.toml", encoding="utf-8").read(), re.M).group(1)
    rows.append((name, f'[workspace.dependencies.{name}]\npath = "{member}"\n'))
for name, path in framework.items():
    rows.append((name, f'[workspace.dependencies.{name}]\npath = "{path}"\n'))
rows.append(("geometry", f'[workspace.dependencies.geometry]\npath = "{FW}/🔨️modules/📐️geometry/📦️packages/🦀️rust"\npackage = "semio-framework-geometry"\n'))
rows.append(("serde", '[workspace.dependencies.serde]\nversion = "1.0.228"\nfeatures = [ "derive" ]\n'))
rows.append(("serde_json", '[workspace.dependencies.serde_json]\nversion = "1.0.149"\nfeatures = [ "raw_value" ]\n'))
EXCLUDE = ['"**/🏭️bridge/**"', '"**/👽️guest/**"', '"**/🔮️oracles/**"', '"🗿️artifacts/**"']
text = ('cargo-features = [ "trim-paths" ]\n\n'
        "# 🔋️ Energy plugin workspace: the simulation engine module crates and the plugin's extension crates. The model\n"
        "# artifact keeps its own workspace (`🗿️artifacts/🔋️model`), and `🏭️bridge` its own.\n"
        "[workspace]\nresolver = \"2\"\nmembers = [\n" + "".join(f'  "{m}",\n' for m in members) + "]\n"
        "exclude = [ " + ", ".join(EXCLUDE) + " ]\n\n"
        "[workspace.metadata.semio.repository]\nschema-version = 1\nexclude-patterns = [ " + ", ".join(EXCLUDE) + " ]\n"
        'member-manifests = [ "**/📦️packages/🦀️rust/Cargo.toml" ]\n\n'
        '  [workspace.package]\n  version = "0.1.0"\n  edition = "2021"\n  rust-version = "1.95"\n\n'
        + "\n".join(r for _, r in sorted(rows)) + "\n" + lints_profiles)
open(f"{PLUGIN}/Cargo.toml", "w", encoding="utf-8").write(text)
print(len(members), "members")
