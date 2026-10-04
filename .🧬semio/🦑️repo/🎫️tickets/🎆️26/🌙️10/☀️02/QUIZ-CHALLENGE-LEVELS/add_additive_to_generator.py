#!/usr/bin/env python3
"""➕️ Gives every ``quantity(...)`` call of the quiz vector generator its ``additive`` argument (design §8.1): amounts of
power, energy, mass and length add up, every other quantity of the generator's quizzes does not.

Run from the repository root: ``.venv/Scripts/python.exe <this file>``.
"""

import os
import re

GENERATOR = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "..", "🌙️09", "☀️28", "UPDATING-ARCHITECTURE-QUIZ-WORDING", "generate_quiz_vectors.py")
ADDITIVE = {"Power", "Energy", "Mass", "Length"}

source = open(GENERATOR, encoding="utf-8").read()
source = source.replace('def quantity(en, de, unit, scale, prefixed):\n    """📐️ A quantity."""\n    return {"label": T(en, de), "unit": unit, "scale": scale, "prefixed": prefixed}', 'def quantity(en, de, unit, scale, prefixed, additive):\n    """📐️ A quantity: whether amounts of it add up is authored (powers, energies, masses and lengths do)."""\n    return {"label": T(en, de), "unit": unit, "scale": scale, "prefixed": prefixed, "additive": additive}')
pattern = re.compile(r'quantity\("([^"]+)", ("[^"]+"), ("[^"]+"), ("[a-z]+"), (True|False)\)')
source, count = pattern.subn(lambda match: 'quantity("%s", %s, %s, %s, %s, %s)' % (match.group(1), match.group(2), match.group(3), match.group(4), match.group(5), match.group(1) in ADDITIVE), source)
print("[DEBUG] replaced %d quantity calls" % count)
with open(GENERATOR, "w", encoding="utf-8", newline="\n") as handle:
    handle.write(source)
