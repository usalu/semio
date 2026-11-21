"""🚿 Round 5, appliances/shower-head: the round-3 model with the 1:50 neck of the hand shower redrawn so that it runs from the handle onto the lower back of the spray disc and sinks into its dome (31 mm wide at most) instead of lying on it as a flat 54 mm plate; writes `volumes/appliances/shower-head.json`.

Neck stations (u along the handle, v off the axis towards the back, rx lateral half width, ry half thickness), checked against the back of the disc: the dome surface lies at v = -0.0017 at u = 0.139, 0.0 at u = 0.152, -0.0008 at u = 0.169 and -0.0055 at u = 0.192, so the last station ends below it.

Usage: python3 r5-shower-head-model.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
_spec = importlib.util.spec_from_file_location("r3_shower_head_model", os.path.join(HERE, "r3-shower-head-model.py"))
M = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(M)

M.NECK_50 = [(0.110, 0.0, 0.0115, 0.0115), (0.126, -0.0003, 0.0130, 0.0122), (0.140, -0.0015, 0.0145, 0.0105), (0.152, -0.0030, 0.0155, 0.0085), (0.162, -0.0045, 0.0140, 0.0065), (0.172, -0.0065, 0.0100, 0.0045)]
OLD_EN = "a neck that flattens and widens from 26 mm to a tongue of 54 mm lying on the back of the spray disc"
NEW_EN = "a neck that flattens and widens from 26 to 31 mm as it runs onto the lower back of the spray disc and sinks into its dome"
OLD_DE = "ein Hals, der sich von 26 mm zu einer Zunge von 54 mm abflacht und verbreitert und auf dem Rücken der Brausescheibe liegt"
NEW_DE = "ein Hals, der sich von 26 auf 31 mm abflacht und verbreitert, auf den unteren Rücken der Brausescheibe läuft und in ihrer Wölbung aufgeht"


def scale_50():
    """🔍 1:50 with the new neck and its decision text."""
    v = M.scale_50()
    d = v["decisions"][0]
    assert OLD_EN in d["en"] and OLD_DE in d["de"]
    d["en"], d["de"] = d["en"].replace(OLD_EN, NEW_EN), d["de"].replace(OLD_DE, NEW_DE)
    return v


def main():
    """📝 Write all four scales."""
    volumes = {"50": scale_50(), "100": M.scale_100(), "200": M.scale_200(), "500": M.scale_500()}
    with open(M.SOURCE, "w", encoding="utf-8") as handle:
        json.dump({"volumes": volumes}, handle, ensure_ascii=False, indent=1)
    print("written", M.SOURCE)


if __name__ == "__main__":
    main()
