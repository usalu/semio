"""📏 Round 5 root fix of `humans/running-child`: the valid range of the 1:500 drawing ends at 1:550 instead of 1:1000. A six-year-old's forearm (50 mm), sock cuff and toe cap fall below the printable 0.2 mm feature beyond about 1:550, as for the other slim figures of the catalog (standing-phone-user 650, tour-guide 700). Idempotent: states the final value.

Usage: python3 r5-running-child-roots.py
"""
import importlib.util
import os

here = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("splice", os.path.join(here, "r3-running-child-splice.py"))
splice = importlib.util.module_from_spec(spec)
spec.loader.exec_module(splice)


def change(asset):
    asset["representations"]["500"]["validRange"]["maxDenominator"] = 550
    return ["representations"]


print(splice.replace_members(change))
