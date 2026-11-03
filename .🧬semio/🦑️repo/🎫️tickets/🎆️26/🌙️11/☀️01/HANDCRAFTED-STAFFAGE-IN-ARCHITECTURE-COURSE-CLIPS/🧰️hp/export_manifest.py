"""🎭 Export every manifest cast once with the cast CLI under `repo tools exec --light` and record wall time, peak resident memory and frame count per cast.

One exporter process per cast, so the `[peak]` line the exporter prints is the peak of that cast alone. Output goes to the ticket generated folder, not the shared cache.

@see ../../🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts — the cast exporter
"""

import csv
import json
import shutil
import subprocess
import sys
import time
from pathlib import Path

HERE = Path(__file__).resolve()
W = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
EXPORTER = W / "🎓️teaching/🏛️architecture/🧍️staffage/🔨️modules/📤️export/🎭️cast/🟦️.ts"
MANIFEST = W / "🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧍️staffage/🔣️.json"
GENERATED = HERE.parent.parent / "🗑️generated" / "hp"
OUT = GENERATED / "export"
LOGS = GENERATED / "export-logs"
REPORT = GENERATED / "export-per-cast.tsv"
FRAME_CAP = 600
SVG_LIMIT = 200 * 1024


def main():
    """🚀 Run one exporter call per cast and write a TSV row per cast."""
    repo = shutil.which("repo") or shutil.which("repo.exe")
    if repo is None:
        raise FileNotFoundError("repo is not on PATH")
    casts = list(json.loads(MANIFEST.read_text(encoding="utf-8"))["casts"])
    if OUT.exists():
        shutil.rmtree(OUT)
    OUT.mkdir(parents=True)
    LOGS.mkdir(parents=True, exist_ok=True)
    rows = []
    for number, cast in enumerate(casts, 1):
        started = time.perf_counter()
        result = subprocess.run([repo, "tools", "exec", "--light", "bun", "--", str(EXPORTER), "--manifest", str(MANIFEST), "--ids", cast, "--out", str(OUT), "--lang", "en"], cwd=W, capture_output=True, text=True, encoding="utf-8", errors="replace")
        wall = time.perf_counter() - started
        (LOGS / f"{cast}.log").write_text(result.stderr + result.stdout, encoding="utf-8")
        peak = None
        for line in result.stderr.splitlines():
            if line.startswith("[peak]"):
                peak = int(line.split()[1])
        folder = OUT / cast
        index_path = folder / "🔣️.json"
        frames = unique = svg_bytes = svg_count = 0
        if index_path.is_file():
            frames = len(json.loads(index_path.read_text(encoding="utf-8"))["frames"])
            unique = len(list(folder.glob("*.svg")))
            svg_files = list(folder.glob("*.svg"))
            svg_bytes = sum(f.stat().st_size for f in svg_files)
            svg_count = len(svg_files)
        row = {"cast": cast, "exit": result.returncode, "wall_s": round(wall, 2), "peak_mb": peak, "frames": frames, "unique_svgs": svg_count, "svg_kib": round(svg_bytes / 1024, 1), "at_cap": frames >= FRAME_CAP, "svg_over_200k": svg_bytes > SVG_LIMIT}
        rows.append(row)
        print(f"{number}/{len(casts)} {cast} exit={row['exit']} wall={row['wall_s']}s peak={peak}MB frames={frames} svg={row['svg_kib']}KiB", flush=True)
    with REPORT.open("w", newline="", encoding="utf-8") as handle:
        writer = csv.DictWriter(handle, fieldnames=list(rows[0]), delimiter="\t")
        writer.writeheader()
        writer.writerows(rows)
    failed = [r["cast"] for r in rows if r["exit"] != 0]
    print(f"total wall {round(sum(r['wall_s'] for r in rows), 1)} s, max peak {max(r['peak_mb'] or 0 for r in rows)} MB, failed {failed}")
    return 0 if not failed else 1


if __name__ == "__main__":
    sys.exit(main())
