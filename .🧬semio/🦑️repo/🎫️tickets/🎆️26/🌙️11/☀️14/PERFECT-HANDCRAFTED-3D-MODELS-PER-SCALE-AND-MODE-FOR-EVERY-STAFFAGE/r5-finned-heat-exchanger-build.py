"""🧲 Assemble the finned exchanger with the correct header pen and digital coil geometry through 1:500.

Usage: python3 r5-finned-heat-exchanger-build.py
"""
import importlib.util
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))


def main():
    spec = importlib.util.spec_from_file_location("finned_build", os.path.join(HERE, "r3-finned-heat-exchanger-build.py"))
    r3 = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(r3)
    r3.main()
    cand = json.load(open(r3.OUT, encoding="utf-8"))
    json.dump(cand, open(r3.OUT, "w", encoding="utf-8"), indent=2, ensure_ascii=False)


if __name__ == "__main__":
    main()
