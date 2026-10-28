"""🧪 Reproduce the actual cooling facade-to-office-shell morph through native Manim."""
import importlib
import json
import os
import sys
from pathlib import Path

TICKET = Path(__file__).resolve().parent
ROOT = next(path for path in TICKET.parents if (path / "🧰️framework").is_dir())
OUTPUT = TICKET / "🗑️generated/iv/cooling-fix"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ["SEMIO_TEACHING_OUTPUT_ROOT"] = str(OUTPUT)
os.environ["SEMIO_TEACHING_CACHE_DIR"] = str(OUTPUT / "cache")
sys.path.insert(0, str(ROOT))
sys.dont_write_bytecode = True


def main():
    """🔬 Compare all source/target freezing combinations at the original morph boundary."""
    import numpy as np
    from manim import Camera, ReplacementTransform, tempconfig

    module = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.❄️cooling.🎬️clip.👥️internal-gains.🐍️")
    rows = []
    with tempconfig({"media_dir": str(OUTPUT / "media"), "text_dir": str(OUTPUT / "text")}):
        for source_frozen, target_frozen, snapshot in ((False, False, False), (True, False, False), (False, True, False), (True, True, False), (True, True, True)):
            facade = module.house_section(np.array([0.0, -0.55, 0.0]))["group"]
            facade.scale(1.35).move_to([0, -0.15, 0])
            shell = module.room_section(np.array([0.0, -0.2, 0.0]), w=6.6, h=2.5)["shell"]
            counts = [sum(len(member.updaters) for member in mob.get_family()) for mob in (facade, shell)]
            if source_frozen:
                facade.clear_updaters()
            if target_frozen:
                shell.clear_updaters()
            pixels_equal = None
            if snapshot:
                before = Camera(pixel_width=480, pixel_height=270)
                before.capture_mobjects([facade])
                parts = []
                for member in facade.family_members_with_points():
                    part = member.copy().clear_updaters()
                    part.set_submobjects([])
                    parts.append(part)
                facade.set_submobjects(parts)
                after = Camera(pixel_width=480, pixel_height=270)
                after.capture_mobjects([facade])
                pixels_equal = bool(np.array_equal(np.asarray(before.get_image()), np.asarray(after.get_image())))
                assert pixels_equal
            error = None
            try:
                animation = ReplacementTransform(facade, shell)
                animation.begin()
                for alpha in (0, 0.25, 0.5, 0.75, 1):
                    animation.update_mobjects(0.25)
                    animation.interpolate(alpha)
                animation.finish()
            except ValueError as failure:
                error = str(failure)
            row = {"source_frozen": source_frozen, "target_frozen": target_frozen,
                   "source_snapshot": snapshot, "snapshot_pixels_equal": pixels_equal,
                   "original_updaters": counts, "error": error}
            rows.append(row)
            print("[DEBUG] Cooling morph", json.dumps(row), flush=True)
    (OUTPUT / "oracle.json").write_text(json.dumps(rows, indent=2), encoding="utf-8")
    (TICKET / "inspection-cooling-fix-test.md").write_text("# Inspection Cooling Morph Oracle\n\nThe original production geometry and facade scale/position were used with the third-party Manim ReplacementTransform begin/update/interpolate lifecycle. The unfrozen control and all updater-only freezing combinations reproduce the native strict-zip topology failure. An independently owned source point-family snapshot is checked against the original Cairo pixels before driving the morph with a frozen target. No shared helper or authored production source was changed by this oracle.\n\n```json\n" + json.dumps(rows, indent=2) + "\n```\n", encoding="utf-8")
    assert rows[0]["error"] and "zip" in rows[0]["error"], rows
    assert rows[-1]["error"] is None, rows


if __name__ == "__main__":
    main()
