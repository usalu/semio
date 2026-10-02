"""🔎️ Work package B probe: judges `🗑️generated/wp-b/probe.json` (written by `probe_kinematics.ts`) with numpy and Python's exact rationals.

Run from the repository root after the TypeScript probe:
`.venv/Scripts/python.exe .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️02/QUIZ-PETS/probe_kinematics.py [--full]`
(`--full` holds every sample to the 60-digit reference instead of every tenth; it takes a few minutes).

The accuracy of the trigonometry is measured twice: against `numpy.sin`/`numpy.cos` of `2π·t` (the registered oracle,
itself limited by the rounding of `2π·t`), and against a 60-digit reference (`decimal` Taylor series of the exactly
reduced angle), which is what the ulp figures refer to.
"""

import json
import math
import sys
from decimal import Decimal, getcontext
from fractions import Fraction
from pathlib import Path

import numpy

getcontext().prec = 60
PI = Decimal("3.14159265358979323846264338327950288419716939937510582097494459")
HERE = Path(__file__).resolve().parent
probe = json.loads((HERE / "🗑️generated" / "wp-b" / "probe.json").read_text(encoding="utf-8"))


def reference(turns: float) -> tuple[Decimal, Decimal]:
    """📚️ Sine and cosine of an angle in turns to 60 digits: the fraction of the turn is taken exactly, then summed as a Taylor series."""
    exact = Fraction(turns)
    phase = exact - math.floor(exact)
    angle = (Decimal(phase.numerator) / Decimal(phase.denominator)) * 2 * PI
    sine, cosine, term, index = Decimal(0), Decimal(0), Decimal(1), 0
    while abs(term) > Decimal(10) ** -58 or index < 4:
        if index % 2 == 0:
            cosine += term if index % 4 == 0 else -term
        else:
            sine += term if index % 4 == 1 else -term
        index += 1
        term = term * angle / index
    return sine, cosine


turns = numpy.array(probe["turns"], dtype=numpy.float64)
sines = numpy.array(probe["sines"], dtype=numpy.float64)
cosines = numpy.array(probe["cosines"], dtype=numpy.float64)
moderate = numpy.abs(turns) <= 8.0
numpy_sine = numpy.max(numpy.abs(sines[moderate] - numpy.sin(2.0 * numpy.pi * turns[moderate])))
numpy_cosine = numpy.max(numpy.abs(cosines[moderate] - numpy.cos(2.0 * numpy.pi * turns[moderate])))
print(f"samples={turns.size} within-8-turns={int(moderate.sum())}")
print(f"max |sinTurns - numpy.sin(2*pi*t)| over [-8, 8] = {numpy_sine:.3e}")
print(f"max |cosTurns - numpy.cos(2*pi*t)| over [-8, 8] = {numpy_cosine:.3e}")

worst_sine, worst_cosine, worst_ulp = Decimal(0), Decimal(0), Decimal(0)
stride = 1 if "--full" in sys.argv else max(1, turns.size // 40000)
chosen = list(range(0, 400001, stride)) + list(range(400001, turns.size))
for index in chosen:
    sine, cosine = reference(float(turns[index]))
    for produced, exact, name in ((float(sines[index]), sine, "sin"), (float(cosines[index]), cosine, "cos")):
        error = abs(Decimal(produced) - exact)
        if name == "sin":
            worst_sine = max(worst_sine, error)
        else:
            worst_cosine = max(worst_cosine, error)
        if abs(exact) > Decimal(10) ** -40:
            worst_ulp = max(worst_ulp, error / Decimal(math.ulp(float(exact))))
        elif error > Decimal(10) ** -40:
            raise AssertionError(f"{name}Turns({turns[index]!r}) = {produced!r} where the exact value is {exact}")
print(f"reference samples={len(chosen)}")
print(f"max |sinTurns - exact| = {float(worst_sine):.3e}")
print(f"max |cosTurns - exact| = {float(worst_cosine):.3e}")
print(f"max error in ulps of the exact value (values beyond 1e-40; exact zeros are reproduced as 0) = {float(worst_ulp):.3f}")

mismatches = 0
for key, words, unit, pick in zip(probe["keys"], probe["words"], probe["units"], probe["picks"]):
    expected = numpy.random.SeedSequence(key).generate_state(len(words)).tolist() if key else None
    if expected is None:
        expected = numpy.random.SeedSequence(numpy.zeros(0, dtype=numpy.uint32)).generate_state(len(words)).tolist()
    if expected != words:
        mismatches += 1
        print("MISMATCH words", key, words, expected)
    if unit != expected[0] / 4294967296.0:
        mismatches += 1
        print("MISMATCH unit", key, unit)
    weights = numpy.array([0, 2, 0.5, -1, 3, 0], dtype=numpy.float64)
    running = numpy.cumsum(numpy.where(weights > 0, weights, 0.0))
    chosen_index = int(numpy.searchsorted(running, unit * running[-1], side="right"))
    if chosen_index != pick:
        mismatches += 1
        print("MISMATCH pick", key, pick, chosen_index)
print(f"keys={len(probe['keys'])} randomness mismatches={mismatches}")
sys.exit(1 if mismatches or numpy_sine > 1e-12 or numpy_cosine > 1e-12 else 0)
