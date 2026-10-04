#!/usr/bin/env python3
"""⛰️ Regenerates every fixture the quiz challenge levels change — all but the seeded randomness and the shared presence —
with the quiz product's vector generator, leaving those two as other sessions hold them.

Run from the repository root: ``.venv/Scripts/python.exe <this file>`` (``.venv/bin/python`` elsewhere).
"""

import glob
import importlib.util
import os
import sys

sys.dont_write_bytecode = True
GENERATOR = glob.glob(os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "🌙️09", "☀️28", "*", "generate_quiz_vectors.py"))[0]

spec = importlib.util.spec_from_file_location("generate_quiz_vectors", GENERATOR)
generator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(generator)
for name in sys.argv[1:] or ["identity", "sheets", "sorting", "matching", "classification", "validation", "badges", "lifecycle", "leaderboard", "rejected", "crowd", "challenge"]:
    getattr(generator, name)()
