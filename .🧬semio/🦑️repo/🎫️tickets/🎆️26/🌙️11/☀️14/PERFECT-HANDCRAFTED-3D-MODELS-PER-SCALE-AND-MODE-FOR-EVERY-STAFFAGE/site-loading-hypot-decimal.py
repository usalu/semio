"""📏 Third-party cross-check of `portable.hypot`: Python's `decimal` computes the Euclidean length of every sample with 80 significant digits and rounds it once to the nearest double; the portable result must be that double.
Usage: python3 site-loading-hypot-decimal.py samples.jsonl"""
import json, sys
from decimal import Decimal, getcontext
getcontext().prec = 80
total = wrong = 0
for line in open(sys.argv[1]):
    values, result = json.loads(line)
    values, result = [float(value) for value in values], float(result)
    exact = sum(Decimal(value) ** 2 for value in values).sqrt()
    total += 1
    if float(exact) != result:
        wrong += 1
        if wrong <= 5:
            print("DIFFERENT", values, result, float(exact))
print(json.dumps({"samples": total, "notTheCorrectlyRoundedDouble": wrong}))
sys.exit(0 if wrong == 0 else 1)
