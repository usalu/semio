"""🐐 Goat, author 07: the ribcage re-cut by hand at every scale.

The chest was a tube 0.30 m wide from the last rib to the shoulder. A dairy goat of 0.76 m at the withers is 0.34 m deep at the heart girth, 0.26 m wide there and only 0.21 m wide between the shoulders; the ribs spring to 0.30 m at the last rib where the abdominal barrel takes over.
PROFILE rows are (u along the thorax bone, lateral half extent, the loft `rx` of the trunk rotation); depth and back line are kept. Every scale keeps its own stations and reads the profile there.
"""
import importlib.util
import os

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("lib07", os.path.join(HERE, "rework-07-lib.py"))
lib = importlib.util.module_from_spec(spec)
spec.loader.exec_module(lib)
SOURCE = os.path.join(HERE, "volumes", "animals", "goat.json")

PROFILE = [(0.058, 0.148), (0.102, 0.143), (0.145, 0.136), (0.192, 0.128), (0.238, 0.119), (0.279, 0.11), (0.321, 0.098), (0.362, 0.084), (0.398, 0.066), (0.421, 0.045), (0.433, 0.022)]


def read(u):
    if u <= PROFILE[0][0]:
        return PROFILE[0][1]
    for (u0, w0), (u1, w1) in zip(PROFILE, PROFILE[1:]):
        if u <= u1:
            return w0 + (w1 - w0) * (u - u0) / (u1 - u0)
    return PROFILE[-1][1]


def main():
    document = lib.load(SOURCE)
    for scale, volume in document["volumes"].items():
        costal = lib.solid(volume["body"], "costal-barrel")
        for section in costal["sections"]:
            section["rx"] = round(min(read(section["at"][0]), section["rx"]), 4)
    lib.save(SOURCE, document)
    print("[DEBUG] goat ribcage re-cut")


main()
