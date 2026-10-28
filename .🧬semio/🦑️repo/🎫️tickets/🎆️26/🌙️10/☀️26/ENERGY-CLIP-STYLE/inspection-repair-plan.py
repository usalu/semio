"""📋 Inventory selected canonical sources with production provenance guards."""
from __future__ import annotations
import importlib
import json
import os
from pathlib import Path
import sys
import time

sys.dont_write_bytecode = True
TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
OUTPUT = TICKET / "🗑️generated" / "i"
os.environ['SEMIO_TEACHING_OUTPUT_ROOT'] = str(OUTPUT)
runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
selected = set(sys.argv[1:])
videos = runtime.load_manifest(source_ids=[])
assert selected and selected.issubset({video["id"] for video in videos})
indexes = {index for index, video in enumerate(videos, 1) if video["id"] in selected}
plan = runtime.segment_plan(videos)
sources = runtime.source_plan([job for job in plan if job["video_index"] in indexes])
partial = {job["source_ordinal"] for job in plan if "clauses" in job["segment"]}
settings = dict(language="de", quality="-ql", resolution="480,270", fps=4.0, audience="spectator")
stale = []
for job in sources:
    segment = {key: job["segment"][key] for key in ("script", "scene")}
    clip = OUTPUT / "de/spectator/s" / str(job["source_ordinal"]) / "v" / f"{segment['scene']}.mp4"
    state = runtime.segment_state(segment, settings)
    trace = job["source_ordinal"] in partial or bool(runtime.source_narration(segment, language="de"))
    reusable = runtime.reusable_clip(clip, state) and clip.with_suffix(".cursor.json").is_file()
    if not reusable or trace and runtime.source_marks(clip, state) is None:
        stale.append(dict(ordinal=job["source_ordinal"], **segment))
result = dict(selected=sorted(selected), canonical_sources=len(sources), stale_count=len(stale),
              stale=stale, checked=time.time())
(OUTPUT / "repair-plan.json").write_text(json.dumps(result, ensure_ascii=False, indent=2), encoding="utf-8")
report = TICKET / "inspection-render.md"
with report.open("a", encoding="utf-8") as stream:
    stream.write("\n\n## Selected Recovery Inventory\n\n")
    stream.write(f"Production provenance guards identified {len(stale)} stale canonical sources among "
                 f"{len(sources)} sources selected by {', '.join(sorted(selected))}.\n\n")
    stream.write("| Ordinal | Scene | Source |\n| ---: | --- | --- |\n")
    stream.write("\n".join(f"| {job['ordinal']} | {job['scene']} | {job['script']} |" for job in stale) + "\n")
print(f"[DEBUG] Recovery plan: {len(stale)}/{len(sources)} selected canonical sources require fresh rendering", flush=True)
