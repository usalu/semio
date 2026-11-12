import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).parent))
from patch import apply

CLIP = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir()) / "🎓️teaching/🏛️architecture/⚡️energy/📊️demand/🎬️clip/🏷️certificate/🐍️.py"

P = "demand.certificate."
apply(CLIP, [
    ("self.play(ReplacementTransform(sheet, bedarf_sheet), run_time=1.4)",
     f'self.play(stage.transition("{P}kinds.sheet-to-demand-sheet", source=sheet, target=bedarf_sheet), run_time=1.4)'),
    ("self.play(ReplacementTransform(cyan_sheet, cyan_left), FadeOut(row_c), FadeOut(box_c), run_time=1.4)",
     f'self.play(stage.transition("{P}when.demand-sheet-to-left", source=cyan_sheet, target=cyan_left), FadeOut(row_c), FadeOut(box_c), run_time=1.4)'),
    ("self.play(ReplacementTransform(big_sheet, small_sheet), run_time=1.4)",
     f'self.play(stage.transition("{P}mandatory.sheet-to-listing-size", source=big_sheet, target=small_sheet), run_time=1.4)'),
    ("self.play(ReplacementTransform(flight, badge), run_time=1.6)",
     f'self.play(stage.transition("{P}mandatory.band-to-class-badge", source=flight, target=badge), run_time=1.6)'),
    ("ReplacementTransform(recommend, none_chip), *_heat(warm_old, 1.4), run_time=1.4)",
     f'stage.transition("{P}mandatory.recommendation-to-none", source=recommend, target=none_chip), *_heat(warm_old, 1.4), run_time=1.4)'),
], scenes=("Beat1_BedarfVsVerbrauch", "Beat3_PflichtangabenUndFolgen"))
