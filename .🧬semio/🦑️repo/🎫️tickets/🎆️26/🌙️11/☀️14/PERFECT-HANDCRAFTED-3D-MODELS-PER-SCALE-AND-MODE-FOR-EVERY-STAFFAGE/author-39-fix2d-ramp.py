import os
__here = os.path.dirname(os.path.abspath(__file__))
exec(open(os.path.join(__here, "author-39-lib.py")).read())


def mutate(reps):
    for s in reps["500"]["geometry"]["elevation"]:
        if s["id"] == "ramp":
            s["points"] = [[1.5, 0], [5.1, 0.3], [6.6, 0.3], [10.2, 0.6], [11.7, 0.6], [11.7, 0]]


patch_representations("wheelchair-ramp", mutate)
