import cProfile, importlib, pstats, sys, unittest
from pathlib import Path
ROOT = Path.cwd(); sys.path.insert(0, str(ROOT))
name = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.🪟️presentation.🧪️tests.🐍️.PresentationNativeTest.test_encoded_chapter_preserves_body_camera_handoffs_audio_and_contextual_boundaries"
suite = unittest.defaultTestLoader.loadTestsFromName(name)
profile = cProfile.Profile()
profile.enable()
unittest.TextTestRunner(verbosity=0).run(suite)
profile.disable()
out = ROOT / sys.argv[1]
stats = pstats.Stats(profile)
stats.sort_stats("cumulative").dump_stats(str(out / "prof.bin"))
with open(out / "prof.txt", "w", encoding="utf-8") as f:
    pstats.Stats(profile, stream=f).sort_stats("cumulative").print_stats(60)
