"""Compile and run the production through-bore certificate and the published bore member verifier against canonical analytic arenas."""
from pathlib import Path
import json
import subprocess
from tempfile import TemporaryDirectory

ticket = Path(__file__).resolve().parent
workspace = Path.cwd()
brep = workspace / "🧰️framework/🔨️modules/🧊️3d/📐️brep"
fixtures = workspace / "✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧪️tests/🔬️unit/🔣️retained-defeature.json"
data = json.loads(fixtures.read_text())
one = data["throughBore"]
two = data["twoThroughBores"]
calls = []
for size, tools in [(one["size"], [dict(origin=one["boreOrigin"], radius=one["radius"])]), (two["size"], two["tools"])]:
    dimensions = str([float(v) for v in size])
    profiles = ",".join(f"({[float(v) for v in t['origin'][:2]]},{t['radius']})" for t in tools)
    calls.append(f"restore({dimensions},&[{profiles}]);")
for row in [one, *data["blindBores"]]:
    dimensions = str([float(v) for v in row["size"]])
    placement = str([float(v) for v in row["boreOrigin"]])
    top = str(row.get("openEnd") != "bottom").lower()
    floor = f'Some({row["floorZ"]})' if "floorZ" in row else "None"
    calls.append(f'restore_published({dimensions},{placement},{row["radius"]},{float(row["boreHeight"])},{top},{floor});')

box_source = (brep / "🛠️operations/🧱️primitives/🦀️.rs").read_text()
box_start = box_source.index("    pub const EDGES:", box_source.index("impl BoxGeometry"))
box_end = box_source.index("    pub fn corners", box_start)
bore_source = (brep / "🛠️operations/🔀️boolean/🧱️certified-boundaries/🕳️bore-boundary/🦀️.rs").read_text()
bore = bore_source[bore_source.index("    pub fn new"):bore_source.index("    pub fn vertex")]
contract = (brep / "📸️representation/🔖️contract/🦀️.rs").read_text()
start = contract.index("pub struct BorePreviewSource")
end = contract.index("    ///", contract.index("pub fn face_count", start))
label = contract.index("    pub fn face_label", end)
source = "#[derive(Clone, Copy)]\n" + contract[start:end] + contract[label:contract.index("\n", label)] + "\n}"
published = (brep / "⚙️engine/🏗️solid-topology/🕳️published-bore-stock/🦀️.rs").read_text()
proof = (brep / "⚙️engine/🏗️solid-topology/🕳️bore-stock-proof/🦀️.rs").read_text().split("#[cfg(test)]", 1)[0]
script = (ticket / "through-bore-witness.rs").read_text()
script = script.replace("/* BOX_TOPOLOGY */", box_source[box_start:box_end])
script = script.replace("/* FIXTURE_CALLS */", "\n".join(calls))
script += "\nimpl BoreBoundary {\n" + bore + "\n}\nmod proof {\n" + proof + "\n}\n"
script += "\nmod contract {\n" + source + "\n}\nmod published {\n" + published + "\n}\n"
generated = ticket / "🗑️generated"
generated.mkdir(exist_ok=True)
with TemporaryDirectory(dir=generated) as work:
    output = str(Path(work) / "bore-witness")
    subprocess.run(
        ["repo", "tools", "exec", "--light", "rustup", "--", "run", "nightly-2026-07-20",
         "rustc", "--edition=2021", "-A", "dead_code", "-o", output, "-"],
        input=script, text=True, check=True,
    )
    subprocess.run([output], check=True)
print("[DEBUG] Production certificates passed through and blind bore arenas, reordered selections, and invalid member identities")
