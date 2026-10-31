"""🧭 Verify the profile against its prior complete visual dependency bracket."""
from datetime import datetime, timezone
import hashlib
import importlib
import importlib.metadata
import importlib.util
import inspect
import json
from pathlib import Path
import platform
import sys

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
GENERATED = TICKET / "🗑️generated"
visual_path = GENERATED / "final-visual-contracts-lining/result.json"
profile_path = GENERATED / "final-profile-contact.json"
verification_paths = (Path(__file__).resolve(), TICKET / "scene-provenance.py")
verification_before = {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in verification_paths}
visual = json.loads(visual_path.read_text(encoding="utf-8"))
profile = json.loads(profile_path.read_text(encoding="utf-8"))
spec = importlib.util.spec_from_file_location("profile_provenance", TICKET / "scene-provenance.py")
provenance = importlib.util.module_from_spec(spec)
spec.loader.exec_module(provenance)
baseline = visual["source_start"]
assert visual["success"] and visual["unchanged"] and visual["tests"] == 22
assert baseline == visual["source_end"]
runtime = visual["runtime_end"]
assert runtime["python"] == sys.version and runtime["executable"] == sys.executable and runtime["platform"] == platform.platform()
assert all(importlib.metadata.version(name) == value for name, value in runtime["packages"].items())
paths = [Path(key.removeprefix("@external:")) if key.startswith("@external:") else ROOT / key for key in baseline]
assert provenance.fingerprints(ROOT, paths) == baseline
raw_before = profile["source_start"]
assert raw_before == {name: hashlib.sha256((ROOT / name).read_bytes()).hexdigest() for name in raw_before}
V = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️")
loaded = []
manifest = list(V.source_files())
for module in tuple(sys.modules.values()):
    filename = getattr(module, "__file__", None)
    if filename:
        path = Path(filename).resolve()
        if path.is_file() and path.is_relative_to(ROOT) and "site-packages" not in path.parts and not path.is_relative_to(TICKET):
            loaded.append(path)
            source_files = getattr(module, "source_files", None)
            if callable(source_files) and not any(parameter.default is inspect.Parameter.empty and parameter.kind not in (inspect.Parameter.VAR_POSITIONAL, inspect.Parameter.VAR_KEYWORD) for parameter in inspect.signature(source_files).parameters.values()):
                manifest.extend(source_files())
from manim._config.utils import config_file_paths
manifest.extend(Path(path) for path in config_file_paths() if Path(path).is_file())
closure = provenance.dependency_paths(ROOT, loaded, manifest, ())
missing = [str(path.relative_to(ROOT)) for path in closure if path.is_relative_to(ROOT) and str(path.relative_to(ROOT)) not in baseline and str(path.relative_to(ROOT)) not in raw_before]
assert not missing, missing
journal = profile_path.with_suffix(".jsonl")
visual_end_time = visual_path.stat().st_mtime
profile_creation_time = getattr(journal.stat(), "st_birthtime", None)
ordering_verified = profile_creation_time is not None and visual_end_time < profile_creation_time
assert profile_creation_time is None or ordering_verified
complete = profile.get("state") == "complete"
if complete:
    assert profile["unchanged"] and raw_before == profile["source_end"]
    assert all(profile[name]["capsules"] == 140 for name in ("cold", "warm"))
row = {
    "observed_at": datetime.now(timezone.utc).isoformat(),
    "profile_complete": complete,
    "visual_result_sha256": hashlib.sha256(visual_path.read_bytes()).hexdigest(),
    "profile_result_sha256": hashlib.sha256(profile_path.read_bytes()).hexdigest(),
    "verification_inputs": verification_before,
    "visual_result_modified_at": datetime.fromtimestamp(visual_end_time, timezone.utc).isoformat(),
    "profile_journal_created_at": datetime.fromtimestamp(profile_creation_time, timezone.utc).isoformat() if profile_creation_time is not None else None,
    "filesystem_ordering_verified": ordering_verified,
    "precursor_semantic_before": baseline,
    "precursor_semantic_end": visual["source_end"],
    "current_semantic": provenance.fingerprints(ROOT, paths),
    "profile_raw_before": raw_before,
    "profile_raw_end": profile.get("source_end"),
    "runtime_precursor": runtime,
    "runtime_current": {"python": sys.version, "executable": sys.executable, "platform": platform.platform(), "packages": {name: importlib.metadata.version(name) for name in runtime["packages"]}},
    "effective_first_party_closure": [str(path.relative_to(ROOT)) for path in closure if path.is_relative_to(ROOT)],
    "missing_dependencies": missing,
}
assert verification_before == {str(path.relative_to(ROOT)): hashlib.sha256(path.read_bytes()).hexdigest() for path in verification_paths}
destination = GENERATED / "profile-dependency-bracket.jsonl"
with destination.open("a", encoding="utf-8") as stream:
    stream.write(json.dumps(row, ensure_ascii=False) + "\n")
text = "# Full Profile Dependency Bracket\n\nThe profiler itself records six raw inputs, not the complete Drafting closure. The completed Visual22 endpoint was written before this fresh profile journal was created. Its complete 35-input semantic map supplies a precursor bracket; the profile independently supplies its six raw inputs. Every effective loaded first-party dependency and declared source_files asset must be in their union. Both maps are rechecked during profiling and after completion. These observations do not retrofit package/configuration capture into the native profiler: package/interpreter identity is compared with the prior visual runtime; the profile does not measure OS peak memory or record its own complete rendering configuration.\n\nThe available file birth times below document ordering of these fresh, uncopied output records. Platforms without file birth times report an unavailable ordering observation instead of substituting metadata-change time as process-start evidence. Endpoint equality establishes the recorded source bracket, not a filesystem mutation watch. Final timing acceptance requires profile_complete=true in the final observation and the terminal command exit.\n\n"
for line in destination.read_text(encoding="utf-8").splitlines():
    text += "```json\n" + json.dumps(json.loads(line), ensure_ascii=False, indent=2) + "\n```\n\n"
(TICKET / "profile-dependency-bracket.md").write_text(text, encoding="utf-8")
print("[DEBUG] Profile dependency bracket verified; complete=" + str(complete))
