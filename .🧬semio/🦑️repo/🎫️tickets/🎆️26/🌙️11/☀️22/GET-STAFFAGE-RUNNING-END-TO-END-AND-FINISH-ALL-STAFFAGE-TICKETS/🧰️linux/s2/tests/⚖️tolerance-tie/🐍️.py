"""⚖️ Oracle of the tolerance tie (🧪️tests/⚖️tolerance-tie): exact rational arithmetic on the decimal strings of the vectors.

Input: the path of the fixture `{d18: {absoluteM, relative}, cases: [{measuredM, nominalM}]}`. Output: JSON `[true | false]`, whether the deviation exceeds D18 (the larger of the absolute and the relative tolerance); the verdict is the same in metres and in paper millimetres because both sides scale by the same exact factor.
"""
import json
import sys
from fractions import Fraction

data = json.load(open(sys.argv[1], encoding="utf8"))
absolute = Fraction(data["d18"]["absoluteM"])
relative = Fraction(data["d18"]["relative"])
print(json.dumps([abs(Fraction(c["measuredM"]) - Fraction(c["nominalM"])) > max(absolute, relative * Fraction(c["nominalM"])) for c in data["cases"]]))
