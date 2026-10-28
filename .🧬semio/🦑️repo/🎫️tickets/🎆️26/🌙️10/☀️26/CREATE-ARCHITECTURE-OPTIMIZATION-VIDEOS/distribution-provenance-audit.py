"""🔗 Verify compact delivery provenance chains."""
import sys
sys.dont_write_bytecode=True
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
import json,hashlib,importlib
r=importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
base=Path(".🧬semio/🎓️teaching/⚡️cache/videos/de/speaker/delivery").resolve()
manifest=json.loads((base/"delivery.json").read_text(encoding="utf8"))
videos={v["id"]:v for v in r.load_manifest() if v["id"].startswith("optimization_")}
assert manifest["schema"]=="semio.architecture.optimization.delivery/v1"
assert (manifest["language"],manifest["audience"],manifest["width"],manifest["height"],manifest["fps"])==("de","speaker",1920,1080,30)
assert len(manifest["videos"])==6
assert {v["id"] for v in manifest["videos"]}==set(videos)
assert len({v["file"] for v in manifest["videos"]})==6
assert manifest["video"]==dict(codec="H.264",profile="High",profileIdc=100,levelIdc=41,pixelFormat="yuv420p",crf=18,preset="fast",threads=4,faststart=True)
assert manifest["audio"]==dict(codec="AAC LC",sampleRate=48000,channels=2,copy=True,spokenNarration=False)
digest=lambda p:hashlib.sha256(p.read_bytes()).hexdigest()
rows=[]
for v in manifest["videos"]:
 expected=videos[v["id"]]["output"]+"_speaker_de_1080p30.mp4"
 assert v["file"]==expected
 target=(base/v["file"]).resolve()
 master=(base/v["master"]).resolve()
 cert=(base/v["masterCertificate"]).resolve()
 assert target.parent==base
 assert master==base.parent/"rendered"/expected
 assert cert==master.with_suffix(".render.json")
 assert digest(target)==v["sha256"]
 assert target.stat().st_size==v["bytes"]
 assert digest(master)==v["masterSha256"]
 assert digest(cert)==v["masterCertificateSha256"]
 state=json.loads(cert.read_text(encoding="utf8"))
 assert state["content"]["video"]==state["presentation"]["video"]==v["id"]
 assert r.state_inputs_current(state)
 assert r.reusable_clip(master,state)
 assert v["frames"]>0 and abs(v["frames"]/30-v["durationSeconds"])<0.000001
 rows.append("| "+v["id"]+" | Pass | "+str(v["bytes"])+" |")
p=Path(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-OPTIMIZATION-VIDEOS/distribution-provenance-audit.md")
p.write_text("# Distribution Provenance Audit\n\nAll six published compact delivery files pass the delivery.json provenance chain audit. Each filename and video identity matches its distinct canonical curriculum entry. Independent SHA-256 recomputation matches the final delivery file, canonical master and canonical render certificate recorded in the manifest; final byte counts also match. All master certificates identify the correct video and pass current nested authored-source hash validation through the runtime interface.\n\n| Video | Delivery → Master → Certificate | Delivery Bytes |\n|---|---|---:|\n"+"\n".join(rows)+"\n\nThe manifest declares German speaker delivery, 1920×1080 at 30 fps, H.264 High level 4.1, yuv420p, CRF 18, fast preset, four encoder threads and faststart. It declares copied AAC LC stereo at 48 kHz with no spoken narration. Manifest frame counts are positive and agree with their stated durations at 30 fps. These are manifest checks; independent stream decode/profile and copied-AAC verification are covered by the encoding audit.\n\nAll master and certificate paths resolve to the matching rendered filenames, and every delivery file stays inside the published delivery directory. No duplicate or missing video entry was accepted. Canonical movie certificate validation remains recorded separately in final-provenance-audit.md.\n\nRead-only verification; no media or source edits. distribution-provenance-audit.py is the retained audit input.\n",encoding="utf8")
print("PASS: all six published delivery/master/certificate chains and current source hashes")
