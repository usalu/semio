"""🔋 Verify the completed wt48zqN Energy release against its published and tested bytes."""
import filecmp
import hashlib
import json
from pathlib import Path
import re
import runpy
import sys

ticket = Path(__file__).resolve().parent
root = ticket.parents[6]
generated = ticket / "🗑️generated"
components = root / ".cache/b/r/m/components"
run = components / "runs/wt48zqN"
report = run / "r"
stages = components / "stages"
module = "semio-hub-energy-component"
base = "semio_hub_energy_component"
helpers = runpy.run_path(str(ticket / "release-receipts.py"))


def require(ok, message):
    if not ok:
        raise ValueError(message)


def digest(path):
    result = hashlib.sha256()
    with path.open("rb") as source:
        for page in iter(lambda: source.read(1024 * 1024), b""):
            result.update(page)
    return result.hexdigest()


def read(path):
    return json.loads(path.read_bytes())


def cache_output(stage, input_name, input_digest, file_name, output_digest=None):
    outputs = set()
    for path in (stages / stage).glob("*/🧾️entry.json"):
        entry = read(path)
        key = entry["key"]
        if key["stage"] != stage or key["inputs"].get(input_name) != input_digest:
            continue
        if entry["digest"] != path.parent.name:
            continue
        for item in entry["files"]:
            if item["path"] == file_name and (output_digest is None or item["sha256"] == output_digest):
                outputs.add(item["sha256"])
    require(len(outputs) == 1, f"Expected a unique {stage} output for {input_digest}")
    return outputs.pop()


def verify(site):
    published = helpers["receipt_path"](root / ".cache/b/p/m/browser/release/modules", "energy")
    deployed = helpers["receipt_path"](site / "🔌️plugin-modules", "energy")
    receipt = helpers["read_receipt"](published, "energy")
    units = read(report / "units.json")
    require(units["units"] and all(unit["exit"] == 0 for unit in units["units"]), "Compiler failure")
    expected = {(module, s) for s in ("stage", "transpile", "describe", "model", "optimize", "browser", "complete", "discard")}
    expected.update({("release-00", "provenance"), ("release-00", "reclaim"), ("release", "provenance-recheck")})
    actual = units["stages"]
    require({(s["component"], s["stage"]) for s in actual} == expected and all(s["ok"] is True for s in actual), "Incomplete build stages")
    custody = read(report / "provenance/reads.json")
    require(custody["closed"] is True and not custody["excess"], "Open compiler custody")
    leaves = read(report / "provenance/leaves.json")["release-00"]
    component = leaves["components"][module]
    require(component["root"] == receipt["provenanceSha256"] and component["sourceTreeDigest"] == receipt["sourceTreeDigest"], "Different source or closure")
    provenance = read(report / "provenance" / leaves["receipt"])
    require(provenance["status"] == 0 and provenance["cancelled"] is False, "Unsuccessful Cargo run")
    matches = [a for u in provenance["units"] if u["message"]["target"]["name"] == base for a in u["artifacts"] if "stagedPath" in a]
    require(len(matches) == 1, "Ambiguous linked component")
    artifact = matches[0]
    raw = Path(provenance["strings"][artifact["stagedPath"]])
    require(run / "w" in raw.parents and raw.name == "component.wasm", "Wrong staged component")
    require(raw.stat().st_size == receipt["wasmBytes"] and digest(raw) == artifact["stagedSha256"] == artifact["sha256"] == receipt["wasmSha256"], "Different raw component")
    require(digest(published.parent / "🛂️.descriptor.semio") == receipt["semanticDescriptorSha256"], "Different declared descriptor")
    core = base + ".core.wasm"
    transpiled = cache_output("transpile", "component", receipt["wasmSha256"], core)
    optimized = cache_output("optimize", "core", transpiled, "core.wasm", digest(published.parent / core))
    names = {p.relative_to(published.parent) for p in published.parent.rglob("*") if p.is_file() and p.name != ".semio-artifact.json"}
    deployed_names = {p.relative_to(deployed.parent) for p in deployed.parent.rglob("*") if p.is_file() and p.name != ".semio-artifact.json"}
    require(names == deployed_names and all(filecmp.cmp(published.parent / n, deployed.parent / n, shallow=False) for n in names), "Different static Energy module")
    log = (generated / "static-verify.log").read_text()
    require(set(re.findall(r"✓\s+\d+ \[(en|de)\].*?› energie boot \(", log)) == {"en", "de"}, "Missing static locale evidence")
    require(helpers["read_receipt"](published, "energy") == receipt, "Receipt changed during verification")
    print(json.dumps({"run": run.name, "units": len(units["units"]), "stages": len(actual), "staticFiles": len(names), "raw": receipt["wasmSha256"], "transpiled": transpiled, "optimized": optimized, "source": component["sourceTreeDigest"], "closure": component["root"]}, indent=2))
    return receipt


if __name__ == "__main__":
    if len(sys.argv) not in (2, 3) or len(sys.argv) == 3 and sys.argv[2] != "--record":
        raise SystemExit("Usage: verify-sealed-energy.py STATIC_SITE [--record]")
    verified = verify(Path(sys.argv[1]).resolve())
    if len(sys.argv) == 3:
        destination = generated / "confirmed/energy.json"
        published = helpers["receipt_path"](root / ".cache/b/p/m/browser/release/modules", "energy")
        helpers["record_confirmation"](published, destination, "energy", expected=verified)
