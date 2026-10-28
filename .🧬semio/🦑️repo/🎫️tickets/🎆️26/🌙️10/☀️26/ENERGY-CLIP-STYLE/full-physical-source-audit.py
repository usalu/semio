import importlib, json, sys, hashlib, datetime
sys.stdout.reconfigure(encoding="utf-8")
from pathlib import Path
sys.dont_write_bytecode = True
import os
root = Path.cwd()
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(Path(__file__).parent / "🗑️generated/full-physical")
sys.path.insert(0, str(root))
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
ticket = Path(__file__).parent
report = ticket / "full-physical-source-audit.md"
settings = {"language": "de", "quality": "-ql", "resolution": "640,360", "fps": 6.0, "audience": "spectator"}
video = next(v for v in runtime.load_manifest(source_ids=["physical_fundamentals"]) if v["id"] == "physical_fundamentals")
states = {s["scene"]: runtime.segment_state({k:s[k] for k in ("script", "scene")}, settings) for c in video["chapters"] for s in c["segments"]}
files = {p:h for state in states.values() for p,h in state["sources"].items()}
manifest = root / "🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json"
files[manifest.as_posix()] = hashlib.sha256(manifest.read_bytes()).hexdigest()
now = datetime.datetime.now(datetime.timezone.utc).isoformat()
if len(sys.argv) > 1 and sys.argv[1] == "compare":
    prior = json.loads(report.read_text(encoding="utf-8").split("<!-- snapshot\n",1)[1].split("\n-->",1)[0])
    changed = [p for p in sorted(set(prior["files"]) | set(files)) if prior["files"].get(p) != files.get(p)]
    with report.open("a", encoding="utf-8") as f:
        f.write("\n## Comparison " + now + "\n\n" + ("No dependency hash changes across the physical source closure.\n" if not changed else "Changed dependencies:\n\n" + "\n".join("- " + p for p in changed) + "\n"))
    print(json.dumps({"changed": changed}, ensure_ascii=False))
else:
    payload = {"time":now, "settings":settings, "files":files, "scenes":list(states)}
    report.write_text("# Full Physical Source Audit\n\nSnapshot: " + now + "\n\nSettings: " + json.dumps(settings) + "\n\n" + str(len(states)) + " physical scenes; " + str(len(files)) + " distinct dependency paths including the curriculum manifest. Hashes come from production segment_state, with a separate manifest hash.\n\nShared runtime, interaction, theme, audience, fonts, visuals and source-owned assets can invalidate all physical beats. Because asset enumeration is recursive under each dependency owner, edits to any qualifying asset under those owners can invalidate a source even when semantically unrelated. Catalog imports may also expose shared lesson changes. JSON changes outside the enumerated owner trees only affect this audit if explicitly imported or listed by source_files. The manifest is separately tracked for selection changes; it is not directly included in segment_state.\n\n<!-- snapshot\n" + json.dumps(payload, ensure_ascii=False, indent=2) + "\n-->\n", encoding="utf-8")
    print(json.dumps({"scenes":list(states), "dependencies":len(files), "report":str(report)}, ensure_ascii=False))
