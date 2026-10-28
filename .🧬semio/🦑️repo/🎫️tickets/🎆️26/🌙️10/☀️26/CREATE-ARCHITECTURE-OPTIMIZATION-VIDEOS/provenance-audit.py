"""🔎 Verify current optimization delivery provenance."""
import sys
sys.dont_write_bytecode=True
import importlib,json
from pathlib import Path
sys.path.insert(0,str(Path.cwd()))
r=importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
vs=[v for v in r.load_manifest() if v["id"].startswith("optimization_")]
assert len(vs)==6
for key in ("id","script","output"):
 assert len({v[key] for v in vs})==6
settings=dict(language="de",quality="-qh",resolution=None,fps=30,audience="speaker")
rows=[]
for v in vs:
 w=importlib.import_module("🎓️teaching."+v["script"].removesuffix(".py").replace("/","."))
 assert w.VIDEO_ID==v["id"]
 o=Path(".🧬semio/🎓️teaching/⚡️cache/videos/de/speaker/rendered")/(v["output"]+"_speaker_de_1080p30.mp4")
 if not o.exists() or not o.with_suffix(".render.json").exists():
  rows.append(dict(video=v["id"],status="pending"))
  continue
 s=json.loads(o.with_suffix(".render.json").read_text(encoding="utf8"))
 assert r.reusable_clip(o,s)
 assert s["content"]["video"]==s["presentation"]["video"]==v["id"]
 assert s["presentation"]["language"]=="de"
 assert s["content"]["settings"]==s["presentation"]["settings"]==settings
 assert r.state_inputs_current(s),v["id"]+" stale hashes"
 assert s["presentation"]==r.presentation_state(v["id"],settings,"de")
 assert len(s["content"]["chapters"])==len(v["chapters"])
 n=0
 for c,a in zip(s["content"]["chapters"],v["chapters"]):
  assert c["scene"]==a["scene"]
  assert c["settings"]==settings
  assert len(c["segments"])==len(a["segments"])
  for t,b in zip(c["segments"],a["segments"]):
   assert t==r.segment_state(b,settings)
   n+=len(t["sources"])
 rows.append(dict(video=v["id"],status="pass",sources=n,bytes=o.stat().st_size))
print(json.dumps(rows))
if all(x["status"]=="pass" for x in rows):
 p=Path(".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-OPTIMIZATION-VIDEOS/final-provenance-audit.md")
 p.write_text("# Final Provenance Audit\n\nAll six German speaker movies pass current provenance verification through the existing runtime interface. Settings are explicit German, speaker audience, high quality (-qh), default 1080p resolution and 30 fps. Each complete movie has its own matching content and presentation identity.\n\n| Video | Certificate | Source Entries | Bytes |\n|---|---|---:|---:|\n"+"".join("| "+x["video"]+" | Pass | "+str(x["sources"])+" | "+str(x["bytes"])+" |\n" for x in rows)+"\nThe six curriculum entries have distinct IDs, wrapper paths and output names. Each imported wrapper declares the correct VIDEO_ID. Each certificate matches current presentation_state and freshly recomputed segment_state, including chapter identities and settings. All nested SHA-256 source fingerprints pass state_inputs_current. No stale or incomplete provenance was accepted.\n\nRead-only verification: no rendering, encoding, broad tests or source edits. Visual and decode/metadata audits are handled separately. provenance-audit.py is retained as the verification input.\n",encoding="utf8")
