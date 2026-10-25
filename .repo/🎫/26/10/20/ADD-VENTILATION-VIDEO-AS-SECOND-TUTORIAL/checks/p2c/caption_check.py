"""[DEBUG] build every caption_bar of the given scene files and report clauses that wrap to too many lines."""
import importlib.util, sys
from pathlib import Path

root = Path(__file__).resolve().parents[8]
sys.path.insert(0, str(root / "tutorial"))
from manim_visuals import caption_bar  # noqa: E402

bad = 0
for i, path in enumerate(sys.argv[1:]):
    spec = importlib.util.spec_from_file_location(f"chk{i}", root / path)
    mod = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = mod
    spec.loader.exec_module(mod)
    for name, cls in vars(mod).items():
        narration = getattr(cls, "NARRATION", None)
        if not isinstance(cls, type) or not narration:
            continue
        for key, _en, de in narration:
            try:
                caption_bar(de)
            except ValueError as exc:
                bad += 1
                print(f"[DEBUG] {path}:{name}:{key}: {exc}")
print(f"[DEBUG] caption check done, {bad} problems")
