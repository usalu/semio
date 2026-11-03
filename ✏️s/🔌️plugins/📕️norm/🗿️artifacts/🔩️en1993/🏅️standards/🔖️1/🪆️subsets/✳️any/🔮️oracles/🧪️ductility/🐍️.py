"""🧪️ Independent oracle of the ductility requirements of EN 1993-1-1 3.2.2 (registry rows ndp.en1993.1-1.ductility.*): f_u/f_y >= 1.10, elongation at failure >= 15 % and eps_u >= 15 eps_y with eps_y = f_y / E; the ratio of requirement to value is the governing utilisation of the plastic analysis gate. Run with `python -I`; it prints the JSON that the Rust unit tests embed."""

import json


def gate(fy, fu, e, elongation, eps_u):
    terms = {"fu_fy": 1.10 * fy / fu}
    if elongation > 0:
        terms["elongation"] = 0.15 / elongation
    if eps_u > 0:
        terms["eps_u"] = 15 * fy / e / eps_u
    name = max(terms, key=terms.get)
    return {"terms": terms, "governing": name, "utilisation": terms[name]}


def main():
    out = {
        "s355": gate(355e6, 510e6, 210e9, 0.20, 0.12),
        "s460": gate(460e6, 540e6, 210e9, 0.17, 0.05),
        "low_strain": gate(355e6, 510e6, 210e9, 0.20, 0.02),
        "no_certificate": gate(355e6, 510e6, 210e9, 0.0, 0.0),
        "eps_y_s355": 355e6 / 210e9,
    }
    print(json.dumps(out, indent=1))


if __name__ == "__main__":
    main()
