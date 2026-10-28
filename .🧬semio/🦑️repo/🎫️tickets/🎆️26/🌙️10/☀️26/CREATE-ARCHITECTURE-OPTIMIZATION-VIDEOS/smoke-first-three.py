"""🧪 Exercise every first-three scene beat and capture bilingual verification keyframes."""

import argparse
import importlib
import os
import sys
from pathlib import Path

sys.dont_write_bytecode = True
ROOT = next(parent for parent in Path(__file__).resolve().parents if (parent / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
OUTPUT = Path(__file__).parent / "🗑️generated" / "first-three"
OUTPUT.mkdir(parents=True, exist_ok=True)
os.environ.update(SEMIO_TEACHING_OUTPUT_ROOT=str(OUTPUT), SEMIO_TEACHING_CACHE_ROOT=str(OUTPUT / "cache"),
                  SEMIO_TEACHING_CACHE_DIR=str(OUTPUT / "fonts"), PYTHONDONTWRITEBYTECODE="1")

from manim import tempconfig

TOPICS = [
    ("📉️extrema", "Beat1_LocalGlobalExtrema", {"global", "negate", "feasible"}),
    ("🎲️sampling", "Beat2_GridRandomSampling", {"grid", "gaps", "marginals"}),
    ("🧪️experiments", "Beat3_FactorialExperiments", {"plan", "difference", "confirm"}),
]


def verify(languages, audiences):
    """📸 Run animations without writing movies and save content-rich boundary frames."""
    for language in languages:
        for audience in audiences:
            os.environ.update(SEMIO_TEACHING_LANGUAGE=language, SEMIO_TEACHING_AUDIENCE=audience)
            for topic, class_name, keyframes in TOPICS:
                module = importlib.import_module(f"🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.{topic}.🐍️")
                with tempconfig({"dry_run":True, "skip_animations":True,"write_to_movie":False,"save_last_frame":False,
                                 "media_dir":str(OUTPUT / "media"), "pixel_width":854,"pixel_height":480,
                                 "frame_rate":4,"verbosity":"ERROR", "progress_bar":"none"}):
                    scene = getattr(module, class_name)()
                    original = scene.step

                    def capture(key, *animations, seconds=1.2):
                        original(key, *animations, seconds=seconds)
                        if key in keyframes and audience == "speaker":
                            scene.renderer.update_frame(scene, ignore_skipping=True)
                            scene.renderer.camera.get_image().save(OUTPUT / f"{class_name}-{language}-{key}.png")

                    scene.step = capture
                    scene.render()
                print(f"[DEBUG] complete {class_name} {language}/{audience}", flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--language", action="append", choices=("en", "de"))
    parser.add_argument("--audience", action="append", choices=("speaker", "spectator"))
    arguments = parser.parse_args()
    verify(arguments.language or ("en", "de"), arguments.audience or ("speaker", "spectator"))
