"""🧭 Inspect authored playlists with actual Manim before declaring beat-segment plans."""

import ast
import importlib.util
import json
from pathlib import Path
import sys

sys.dont_write_bytecode = True
ticket = Path(__file__).resolve().parent
root = Path.cwd() / "🎓️teaching"
source = Path("C:/git/semio.worktrees/🦢niloufar/⛳wip/tutorial")
sys.path.insert(0, str(root))
manifest = json.loads((root / "video_manifest.json").read_text(encoding="utf-8"))
videos = []
for video in manifest["videos"]:
    script = root / video["script"]
    original_tree = ast.parse((source / video["script"]).read_text(encoding="utf-8"))
    current_tree = ast.parse(script.read_text(encoding="utf-8"))
    declarations = []
    for tree in (original_tree, current_tree):
        declarations.append(next(ast.dump(node) for node in tree.body if isinstance(node, ast.AnnAssign) and isinstance(node.target, ast.Name) and node.target.id == video["playlist"]))
    if declarations[0] != declarations[1]:
        raise RuntimeError(f"Original playlist order changed: {video['id']}")
    spec = importlib.util.spec_from_file_location("plan_" + video["id"], script)
    module = importlib.util.module_from_spec(spec)
    sys.modules[spec.name] = module
    spec.loader.exec_module(module)
    playlist = getattr(module, video["playlist"])
    logical = getattr(module, "SECTION_SCENES", None) or getattr(module, "CHAPTER_SCENES", None) or [module.PhysicalFundamentals_FullSection]
    chapters = []
    for chapter, (_, beats) in zip(logical, playlist, strict=True):
        segments = [{"script": Path(sys.modules[beat.__module__].__file__).resolve().relative_to(root).as_posix(), "scene": beat.__name__} for beat in beats]
        chapters.append({"scene": chapter.__name__, "segments": segments})
    videos.append({"id": video["id"], "script": video["script"], "playlist": video["playlist"],
                   "intro": {"script": "intro/intro_scene.py", "scene": getattr(module, "SERIES_INTRO_SCENE", None) or module.INTRO_CLASS_NAME},
                   "chapters": chapters, "output": video["output"]})
proposal = {"$schema": "./video_manifest.schema.json", "videos": videos}
generated = ticket / "🗑️generated"
(generated / "segment-plan.json").write_text(json.dumps(proposal, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
counts = [{"video": video["id"], "chapters": len(video["chapters"]), "segments": [len(chapter["segments"]) for chapter in video["chapters"]]} for video in videos]
(generated / "segment-counts.json").write_text(json.dumps(counts, indent=2) + "\n", encoding="utf-8")
print("[DEBUG]", json.dumps(counts), flush=True)
