"""Leak-cycle harness of the lifecycle owner: open -> preview play -> seek -> profile switch -> export-like frames -> cancel mid-play -> close, repeated in ONE process.

Usage (from the repo root, through the shared runner env): python <this file> [--warmup 3] [--cycles 10] [--out <dir>]
Writes <ticket>/LIFECYCLE.md and <ticket>/🗑️generated/lifecycle/cycles.json; exit code 1 when a budget is exceeded.
Unit-scale only: Manim at 32x18, 10 fps, no movie encode; anything heavier goes through QUEUE.md.
"""

from __future__ import annotations

import argparse
import gc
import importlib
import json
import os
import random
import statistics
import sys
import tempfile
import time
import tracemalloc
import weakref
from collections import Counter
from pathlib import Path

HERE = Path(__file__).resolve().parent
TICKET = HERE.parent
ROOT = next(p for p in HERE.parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
os.environ.setdefault("PYTHONUTF8", "1")

import numpy as np
import psutil

manim = importlib.import_module("manim")
stage_module = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🎭️stage.🐍️")
preview_module = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.🖼️preview.🐍️")
instrument = importlib.import_module("🎓️teaching.🏛️architecture.🔀️morph.📊️instrument.🐍️")
CLIP = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip"
profile_module = importlib.import_module(CLIP + ".🎚️profile.🐍️")
cache_module = importlib.import_module(CLIP + ".🎚️profile.🗃️cache.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧮️model.🐍️")

DOCUMENT = ROOT / "🎓️teaching/🏛️architecture/🔀️morph/🧪️tests/🎭️stage/🧫️cases/🔣️.json"
CONFIG = {"write_to_movie": False, "disable_caching": True, "verbosity": "ERROR", "progress_bar": "none", "save_last_frame": False, "pixel_width": 32, "pixel_height": 18, "frame_rate": 10}
BUDGET = {
    "cache_entries_growth": 0,
    "cache_bytes_growth": 0,
    "mobjects_growth": 0,
    "updaters_growth": 0,
    "handles_growth": 2,
    "children_growth": 0,
    "heap_growth_bytes": 512 * 1024,
    "heap_slope_bytes_per_cycle": 64 * 1024,
}
TRANSITION = "demo.blocks-to-tiles"
JOBS = cache_module.BoundedCache("harness.job", max_entries=8, max_bytes=1 << 20, sizeof=lambda value: len(value))
DESIGN = {"orientation": 180.0, "wwr": 0.4, "overhang": 0.3, "insulation": 0.2}


def tile(x, y, color=manim.RED, size=1.0):
    return manim.Square(size, fill_color=color, fill_opacity=0.9, stroke_color=color).move_to([x, y, 0])


class Definitions:
    """Immutable entity definitions shared by every job and every cycle."""

    def __init__(self):
        self.blocks = {"a": tile(-4, -2, manim.BLUE), "b": tile(-2, -2, manim.BLUE)}
        self.bounds = [[2.5, 1.5, 0], [5.5, 2.5, 0]]
        self.part_bounds = {"0": [[2.5, 1.5, 0], [3.5, 2.5, 0]], "1": [[4.5, 1.5, 0], [5.5, 2.5, 0]]}

    def tiles(self, lod):
        extra = 5 if lod == "full" else 0
        group = manim.VGroup(tile(3, 2), tile(5, 2))
        group.add(*[manim.Dot([3 + i * 0.01, 2, 0], radius=0.01, color=manim.RED) for i in range(extra)])
        return manim.VGroup(manim.VGroup(group[0], *group[2:]), group[1]) if extra else group

    def snapshot(self):
        return [(m.points.copy(), m.get_fill_color(), m.get_fill_opacity()) for part in self.blocks.values() for m in part.get_family() if len(m.points)]


def same(a, b):
    return len(a) == len(b) and all(np.array_equal(x[0], y[0]) and x[1:] == y[1:] for x, y in zip(a, b))


def select(name):
    os.environ["SEMIO_TEACHING_PROFILE"] = name
    os.environ["SEMIO_TEACHING_JOB"] = f"cycle-{name}"
    profile_module.reset()
    instrument.reset()
    return profile_module.job()


def open_stage(definitions):
    scene = manim.Scene()
    stage = stage_module.stage(scene, DOCUMENT)
    stage.bind("demo.blocks", definitions.blocks)
    stage.bind("demo.tiles", definitions.tiles, bounds=definitions.bounds, part_bounds=definitions.part_bounds)
    scene.add(stage.mobject("demo.blocks"))
    return scene, stage


def probes():
    """Entries and bytes of every cache the harness can see: the BoundedCache registry plus module-level production caches."""
    found = {f"bounded:{name}": (item["entries"], item["bytes"]) for name, item in cache_module.registry().items()}
    for name in ("_evaluate", "_beam_samples", "_sun_samples", "_sun_point"):
        found[f"lru:optimization.{name}"] = (getattr(model, name).cache_info().currsize, 0)
    found["dict:stage._REPORTED"] = (len(stage_module._REPORTED), 0)
    found["dict:instrument._COUNTS"] = (len(instrument._COUNTS), 0)
    return found


def mobject_census():
    census = Counter()
    updaters = suspended = 0
    for item in gc.get_objects():
        if isinstance(item, manim.Mobject):
            census[type(item).__name__] += 1
            updaters += len(item.updaters)
            suspended += bool(item.updating_suspended)
    return census, updaters, suspended


def sample(label, started, work):
    gc.collect()
    gc.collect()
    census, updaters, suspended = mobject_census()
    current, peak = tracemalloc.get_traced_memory()
    process = psutil.Process()
    handles = process.num_handles() if os.name == "nt" else process.num_fds()
    tracemalloc.reset_peak()
    caches = probes()
    return {
        "cycle": label, "seconds": round(time.perf_counter() - started, 3), "caches": caches,
        "cache_entries": sum(entries for entries, _ in caches.values()), "cache_bytes": sum(size for _, size in caches.values()),
        "mobjects": sum(census.values()), "census": dict(census.most_common(6)), "updaters": updaters, "suspended": suspended,
        "heap_current": current, "heap_peak": peak, "handles": handles, "children": len(process.children(recursive=True)),
        "threads": process.num_threads(), "work": work,
    }


def play_preview(definitions, rng_key=0):
    job = select("preview")
    scene, stage = open_stage(definitions)
    frames = []
    original = scene.update_to_time

    def spy(t):
        original(t)
        frames.append(len(scene.mobjects))

    scene.update_to_time = spy
    scene.play(stage.transition(TRANSITION))
    assert list(scene.mobjects) == [stage.mobject("demo.tiles")], "scene must hold exactly the target after the preview swap"
    assert len(frames) == 10
    counts = dict(instrument.snapshot()["counts"])
    assert counts.get("full.prepare", 0) == counts.get("full.evaluate", 0) == 0, "a preview job must not construct a full morph"
    assert counts["preview.swap"] == 1
    JOBS.put(job.key("harness", TRANSITION, rng_key), b"x" * 64)
    stage.close()
    return counts, job


def seek(definitions, rng):
    select("preview")
    scene, stage = open_stage(definitions)
    animation = stage.transition(TRANSITION)
    animation._setup_scene(scene)
    animation.begin()
    for alpha in [rng.random() for _ in range(8)] + [0.0, 1.0]:
        animation.interpolate(alpha)
    animation.finish()
    animation.clean_up_from_scene(scene)
    assert list(scene.mobjects) == [stage.mobject("demo.tiles")], "seek to alpha 1 must leave exactly the target"
    stage.close()


def switch_and_export(definitions, preview_job, directory, index):
    final_job = select("final")
    assert final_job.key("k") != preview_job.key("k"), "profile switch must never share a cache key"
    JOBS.put(final_job.key("harness", TRANSITION, index), b"y" * 64)
    scene, stage = open_stage(definitions)
    scene.play(stage.transition(TRANSITION))
    counts = dict(instrument.snapshot()["counts"])
    assert counts.get("full.prepare", 0) > 0 and counts.get("full.evaluate", 0) > 0 and counts.get("preview.swap", 0) == 0
    written = []
    from PIL import Image

    for frame in range(3):
        scene.renderer.update_frame(scene)
        path = directory / f"cycle{index}-{frame}.png"
        Image.fromarray(np.asarray(scene.renderer.get_frame())).save(path)
        written.append(path)
    stage.close()
    for path in written:
        path.unlink()
    return counts


def cancel_mid_play(definitions):
    select("preview")
    scene, stage = open_stage(definitions)
    source = stage.mobject("demo.blocks")
    before = [(m.points.copy(), m.updating_suspended) for m in source.get_family()]
    original = scene.update_to_time

    def interrupt(t):
        original(t)
        if t > 0.35:
            raise KeyboardInterrupt("cancelled mid-play")

    scene.update_to_time = interrupt
    try:
        scene.play(stage.transition(TRANSITION))
    except KeyboardInterrupt:
        scene.renderer.file_writer.abort_encode_jobs()
    else:
        raise AssertionError("the play must have been interrupted")
    held = any(m.updating_suspended for m in source.get_family())
    stage.close()
    stage.close()
    after = [(m.points.copy(), m.updating_suspended) for m in source.get_family()]
    assert not any(m.updating_suspended for m in source.get_family()), "close() must release the suspended source updaters"
    assert all(np.array_equal(a[0], b[0]) and a[1] == b[1] for a, b in zip(before, after)), "close() must restore the source pose and updater flags"
    return held


def shared_ownership():
    """Resources shared by two owners stay valid until the last one releases them, then become collectable."""
    report = {}
    definitions = Definitions()
    snapshot = definitions.snapshot()
    select("preview")
    scene_a, stage_a = open_stage(definitions)
    select("final")
    scene_b, stage_b = open_stage(definitions)
    watch = weakref.ref(stage_b.mobject("demo.tiles"))
    stage_a.close()
    report["definitions_valid_after_first_owner_closed"] = same(snapshot, definitions.snapshot())
    scene_b.play(stage_b.transition(TRANSITION))
    report["second_owner_still_plays_final"] = list(scene_b.mobjects) == [stage_b.mobject("demo.tiles")] and instrument.snapshot()["counts"].get("full.evaluate", 0) > 0
    stage_b.close()
    report["definitions_unmutated_after_last_owner_closed"] = same(snapshot, definitions.snapshot())
    part = weakref.ref(definitions.blocks["a"])
    del scene_a, scene_b, stage_a, stage_b
    gc.collect()
    report["stage_owned_target_released_after_close"] = watch() is None
    report["definition_alive_while_owner_holds_it"] = part() is not None
    definitions = None
    gc.collect()
    report["definition_releasable_after_last_owner"] = part() is None

    entry = type("Entry", (), {})()
    holder = entry
    reference = weakref.ref(entry)
    bounded = cache_module.BoundedCache("harness.shared", max_entries=1, max_bytes=1 << 20, sizeof=lambda _: 1)
    bounded.put("a", entry)
    bounded.put("b", object())
    report["evicted_entry_valid_while_held"] = reference() is holder and bounded.get("a") is None
    entry = holder = None
    gc.collect()
    report["evicted_entry_releasable_after_last_owner"] = reference() is None
    return report


def slope(values):
    return float(np.polyfit(np.arange(len(values)), values, 1)[0]) if len(values) > 1 else 0.0


def verdict(rows, warmup):
    base, tail = rows[warmup - 1], rows[warmup:]
    checks = []

    def check(name, observed, allowed, detail=""):
        checks.append({"check": name, "observed": observed, "budget": allowed, "ok": observed <= allowed, "detail": detail})

    check("cache entries growth after warm-up", max(r["cache_entries"] for r in tail) - base["cache_entries"], BUDGET["cache_entries_growth"])
    check("cache bytes growth after warm-up", max(r["cache_bytes"] for r in tail) - base["cache_bytes"], BUDGET["cache_bytes_growth"])
    check("Mobject instances growth (gc census)", max(r["mobjects"] for r in tail) - base["mobjects"], BUDGET["mobjects_growth"])
    check("live updaters growth", max(r["updaters"] for r in tail) - base["updaters"], BUDGET["updaters_growth"])
    check("suspended updating flags after close", max(r["suspended"] for r in tail), 0)
    check("open handles growth", max(r["handles"] for r in tail) - base["handles"], BUDGET["handles_growth"])
    check("child processes growth", max(r["children"] for r in tail) - base["children"], BUDGET["children_growth"])
    check("heap (tracemalloc current) growth", rows[-1]["heap_current"] - base["heap_current"], BUDGET["heap_growth_bytes"])
    check("heap slope per cycle (least squares)", round(slope([r["heap_current"] for r in tail])), BUDGET["heap_slope_bytes_per_cycle"])
    return checks


def write_report(rows, checks, ownership, warmup, args, top):
    lines = [
        "# Lifecycle: leak-cycle results", "",
        f"Harness `$T/lifecycle/cycle.py`, one process, {warmup} warm-up + {len(rows) - warmup} measured cycles, Manim 32x18 at 10 fps, "
        "Python " + sys.version.split()[0] + ", " + sys.platform + ". Each cycle: open stage -> preview play (real `scene.play`) -> seek (8 random alphas + 0 and 1 on `PreviewSwap.interpolate`) -> "
        "profile switch to final (Job keys differ, real full morph) -> export-like capture (3 frames rendered and PNG-encoded) -> cancel mid-play (`KeyboardInterrupt` inside `scene.play`) -> `Stage.close()` twice. "
        "Per-cycle sampling happens after two `gc.collect()` calls. The cancel step mirrors `manim.Manager.render`, which calls `file_writer.abort_encode_jobs()` on any `BaseException` raised mid-play; "
        "a bare `scene.play` interrupted without that call leaks one unsealed encoder worker and about 13 native encoder threads per cycle (measured with this harness before the abort was added), which is Manim's contract, not a semio defect.", "",
        "## Plateau verdict", "", "| check | observed | budget | result |", "| --- | --- | --- | --- |",
        *[f"| {c['check']} | {c['observed']} | <= {c['budget']} | {'PASS' if c['ok'] else 'FAIL'} |" for c in checks], "",
        "## Per cycle", "", "| cycle | s | cache entries | cache bytes | Mobjects | updaters | suspended | heap current B | heap peak B | handles | children | threads |", "| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |",
        *[f"| {r['cycle']} | {r['seconds']} | {r['cache_entries']} | {r['cache_bytes']} | {r['mobjects']} | {r['updaters']} | {r['suspended']} | {r['heap_current']} | {r['heap_peak']} | {r['handles']} | {r['children']} | {r['threads']} |" for r in rows], "",
        "Caches seen at the last cycle (entries, bytes): " + ", ".join(f"`{k}` {v}" for k, v in sorted(rows[-1]["caches"].items())) + ".", "",
        "Work counters of the last cycle: preview `" + json.dumps(rows[-1]["work"]["preview"], sort_keys=True) + "`, final `" + json.dumps(rows[-1]["work"]["final"], sort_keys=True) + "`.", "",
        "Top allocation growth sites after warm-up (tracemalloc, bytes): " + ("; ".join(top) if top else "none above 1 KiB") + ".", "",
        "## Shared resources and last-owner release", "", "| property | holds |", "| --- | --- |",
        *[f"| {key.replace('_', ' ')} | {'yes' if value else 'NO'} |" for key, value in ownership.items()], "",
        "## Instrumentation gaps (honest list)", "",
        "- GPU allocations, browser object URLs, DOM listeners and web workers are not observable here: this harness covers the Python render side only (player/browser lifecycle belongs to the playback owner).",
        "- `tracemalloc` sees Python allocations (including NumPy buffers) but not native Cairo/FFmpeg memory; handles come from `psutil` (`num_handles` on Windows, `num_fds` elsewhere).",
        "- The Mobject census counts gc-tracked instances only; a leak held in a C extension would show in the process working set, which is not sampled per cycle.",
        "- Disk caches (nakagin orders, BIM slabs, structure solves, staffage casts) are bounded and pruned by `🗃️cache/💾️disk` and covered by unit tests, not exercised in this loop.",
        "- Staffage lock waits and the exporter process tree need the exporter command; they are covered by unit tests with fake commands (see the report), not by this loop.",
    ]
    (TICKET / "LIFECYCLE.md").write_text("\n".join(lines) + "\n", encoding="utf-8")
    generated = TICKET / "🗑️generated" / "lifecycle"
    generated.mkdir(parents=True, exist_ok=True)
    (generated / "cycles.json").write_text(json.dumps({"rows": rows, "checks": checks, "ownership": ownership}, indent=2, default=str), encoding="utf-8")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--warmup", type=int, default=4)
    parser.add_argument("--cycles", type=int, default=10)
    args = parser.parse_args()
    tracemalloc.start(3)
    rng = random.Random(7)
    definitions = Definitions()
    reference = definitions.snapshot()
    rows, snapshot_after_warmup = [], None
    for _ in range(2 * 2048 + 8):
        model.evaluate({**DESIGN, "orientation": rng.uniform(0, 360)})
    for _ in range(2 * 1024 + 8):
        model.sun_samples(rng.uniform(0, 360), "summer")
    with tempfile.TemporaryDirectory() as scratch, manim.tempconfig(CONFIG):
        directory = Path(scratch)
        for index in range(args.warmup + args.cycles):
            started = time.perf_counter()
            preview_counts, job = play_preview(definitions, index)
            seek(definitions, rng)
            for _ in range(40):
                model.evaluate({**DESIGN, "orientation": rng.uniform(0, 360)})
                model.sun_samples(rng.uniform(0, 360), "summer")
            model.orientation_study()
            final_counts = switch_and_export(definitions, job, directory, index)
            cancel_mid_play(definitions)
            assert same(reference, definitions.snapshot()), "shared definitions were mutated"
            rows.append(sample(index, started, {"preview": preview_counts, "final": final_counts}))
            if index == args.warmup - 1:
                gc.collect()
                snapshot_after_warmup = tracemalloc.take_snapshot()
            print(f"cycle {index}: {rows[-1]['seconds']} s, heap {rows[-1]['heap_current']} B, Mobjects {rows[-1]['mobjects']}, entries {rows[-1]['cache_entries']}", flush=True)
        leftover = list(directory.glob("*"))
        assert not leftover, f"scratch files left behind: {leftover}"
        gc.collect()
        grown = tracemalloc.take_snapshot().compare_to(snapshot_after_warmup, "lineno")
        ownership = shared_ownership()
    top = [f"{Path(item.traceback[0].filename).name}:{item.traceback[0].lineno} {item.size_diff:+d}" for item in grown[:5] if item.size_diff > 1024]
    checks = verdict(rows, args.warmup)
    write_report(rows, checks, ownership, args.warmup, args, top)
    failed = [c for c in checks if not c["ok"]] + [{"check": key} for key, value in ownership.items() if not value]
    print("FAILED:" if failed else "PASS: all budgets held", json.dumps(failed, default=str) if failed else "")
    return 1 if failed else 0


if __name__ == "__main__":
    sys.exit(main())
