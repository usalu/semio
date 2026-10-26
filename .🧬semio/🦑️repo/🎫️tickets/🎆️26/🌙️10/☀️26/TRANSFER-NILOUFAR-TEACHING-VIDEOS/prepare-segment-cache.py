"""📚 Seed fresh private segment caches from complete validated first-pass SVG bytes."""

from concurrent.futures import ThreadPoolExecutor
import json
import os
from pathlib import Path
import shutil
import sys
from xml.etree import ElementTree

sys.dont_write_bytecode = True
ticket = Path(__file__).resolve().parent
root = Path.cwd() / "🎓️teaching"
sys.path.insert(0, str(root))
from video_runtime import segment_plan

generated = ticket / "🗑️generated"
output = generated / "g"
old = json.loads((generated / "pre/jobs.json").read_text(encoding="utf-8"))
old_media = {(job["video"], job["scene"]): Path(job["media"]) for job in old}
groups = {}
for job in segment_plan():
    source = old_media.get((job["video_id"], job["chapter_scene"]), generated / "videos" / str(job["video_index"])) / "t"
    groups.setdefault(source, []).append(output / "s" / str(job["ordinal"]) / "t")
report = []
for source, targets in groups.items():
    valid = []
    for svg in source.glob("*.svg"):
        if svg.is_symlink() or svg.stem.endswith("_"):
            continue
        try:
            data = svg.read_bytes()
            ElementTree.fromstring(data)
            valid.append((svg.name, data))
        except (OSError, ElementTree.ParseError):
            continue
    required = sum(len(data) for _, data in valid) * len(targets)
    if required > shutil.disk_usage(generated).free // 2:
        print(f"[DEBUG] Seed skipped for {source}; native rendering will populate its caches", flush=True)
        continue

    def copy_private(target):
        target.mkdir(parents=True, exist_ok=True)
        copied = 0
        for name, data in valid:
            try:
                descriptor = os.open(target / name, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600)
            except FileExistsError:
                continue
            with os.fdopen(descriptor, "wb") as destination:
                destination.write(data)
            if (target / name).stat().st_nlink != 1:
                raise RuntimeError("Seeded SVG is not private")
            copied += 1
        return copied

    with ThreadPoolExecutor(max_workers=8) as pool:
        copied = sum(pool.map(copy_private, targets))
    report.append({"source": str(source), "targets": len(targets), "valid_svgs": len(valid), "copied": copied})
    print(f"[DEBUG] Seeded {copied} private SVG copies for {len(targets)} segments", flush=True)
(generated / "segment-seed.json").write_text(json.dumps(report, indent=2) + "\n", encoding="utf-8")
print("[DEBUG] All private segment cache seeds completed", flush=True)
