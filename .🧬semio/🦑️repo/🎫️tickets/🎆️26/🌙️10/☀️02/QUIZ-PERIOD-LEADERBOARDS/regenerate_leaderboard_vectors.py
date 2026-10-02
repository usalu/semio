#!/usr/bin/env python3
"""🏆️ Regenerates only the fixtures the period leaderboards change — `🏆️leaderboard` and `🪪️identity-shapes` — with
the quiz product's vector generator, leaving every other fixture as other sessions hold it.

Run from the repository root: ``.venv/Scripts/python.exe <this file>`` (``.venv/bin/python`` elsewhere).
"""

import importlib.util
import os
import sys

sys.dont_write_bytecode = True
GENERATOR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "🌙️09", "☀️28", "QUIZ-PRODUCT-AND-TEACHING-PROCTOR", "generate_quiz_vectors.py")

spec = importlib.util.spec_from_file_location("generate_quiz_vectors", GENERATOR)
generator = importlib.util.module_from_spec(spec)
spec.loader.exec_module(generator)
generator.identity()
generator.leaderboard()
