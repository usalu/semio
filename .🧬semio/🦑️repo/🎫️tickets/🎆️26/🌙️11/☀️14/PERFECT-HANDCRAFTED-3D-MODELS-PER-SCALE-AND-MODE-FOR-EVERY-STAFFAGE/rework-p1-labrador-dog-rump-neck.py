"""🐕 Hand-authored hindquarter and neck-base sections of the labrador: the thigh rises as a gluteal dome out of the croup instead of a capped drum, the neck base is a ball about the cervicothoracic joint so no shelf opens when the neck lifts.

Usage: python3 rework-p1-labrador-dog-rump-neck.py
"""
import json

SOURCE = "volumes/animals/labrador-dog.json"
THIGH_ROTATION = {"n": [0.54749, 0.4475, 0.4475, 0.54749], "f": [0.4475, -0.54749, 0.54749, -0.4475]}
NECK_ROTATION = [0.57516, 0.41132, 0.41132, 0.57516]
THIGH = {
    "50": ("thigh-", 11, [
        (-0.052, -0.0040, 0.012, 0.012, 0.008, 2.0),
        (-0.050, -0.0040, 0.011, 0.024, 0.014, 2.0),
        (-0.046, -0.0042, 0.010, 0.036, 0.020, 2.0),
        (-0.040, -0.0045, 0.008, 0.047, 0.025, 2.0),
        (-0.032, -0.0050, 0.005, 0.057, 0.029, 2.1),
        (-0.022, -0.0056, 0.001, 0.066, 0.033, 2.1),
        (-0.010, -0.0064, -0.005, 0.076, 0.037, 2.2),
        (0.004, -0.0072, -0.011, 0.086, 0.041, 2.2),
        (0.020, -0.0082, -0.017, 0.093, 0.0445, 2.2),
        (0.036, -0.0089, -0.021, 0.097, 0.047, 2.2),
    ]),
    "100": ("thigh-", 9, [
        (-0.050, -0.0040, 0.011, 0.022, 0.013, 2.0),
        (-0.044, -0.0043, 0.009, 0.040, 0.022, 2.0),
        (-0.032, -0.0050, 0.005, 0.057, 0.029, 2.1),
        (-0.012, -0.0063, -0.004, 0.075, 0.036, 2.2),
        (0.012, -0.0078, -0.015, 0.090, 0.043, 2.2),
    ]),
    "200": ("thigh-", 4, [
        (-0.050, -0.0040, 0.011, 0.022, 0.013, 2.0),
        (-0.040, -0.0045, 0.008, 0.047, 0.025, 2.0),
        (-0.012, -0.0063, -0.004, 0.075, 0.036, 2.2),
    ]),
    "500": ("hind-thigh-", 2, [
        (-0.050, -0.0040, 0.011, 0.022, 0.013, 2.0),
        (-0.034, -0.0048, 0.006, 0.055, 0.028, 2.1),
    ]),
}
NECK = {
    "50": [(-0.098, 0.020), (-0.093, 0.040), (-0.083, 0.060), (-0.068, 0.078), (-0.050, 0.092), (-0.030, 0.103), (-0.010, 0.110)],
    "100": [(-0.097, 0.024), (-0.086, 0.055), (-0.066, 0.080), (-0.040, 0.098), (-0.012, 0.110)],
    "200": [(-0.097, 0.024), (-0.082, 0.062), (-0.052, 0.091), (-0.014, 0.110)],
    "500": [(-0.095, 0.030), (-0.070, 0.076), (-0.020, 0.108)],
}


def solid(volumes, scale, identifier):
    return next(entry for entry in volumes[scale]["body"] if entry["id"] == identifier)


def main():
    document = json.load(open(SOURCE))
    volumes = document["volumes"]
    for scale, (prefix, keep, table) in THIGH.items():
        for side, sign in (("n", 1), ("f", -1)):
            thigh = solid(volumes, scale, prefix + side)
            head = [{"at": [x, y, round(sign * z, 5)], "rx": rx, "ry": ry, "exponent": exponent, "rotation": THIGH_ROTATION[side]} for x, y, z, rx, ry, exponent in table]
            thigh["sections"] = head + thigh["sections"][keep:]
    for scale, table in NECK.items():
        neck = solid(volumes, scale, "neck")
        head = [{"at": [x, round(-0.002 - 0.009 * (x + 0.098) / 0.088, 5), 0], "rx": r, "ry": round(0.9 * r, 5), "exponent": 2, "rotation": NECK_ROTATION} for x, r in table]
        neck["sections"] = head + neck["sections"][7:]
    json.dump(document, open(SOURCE, "w"), ensure_ascii=False)


main()
