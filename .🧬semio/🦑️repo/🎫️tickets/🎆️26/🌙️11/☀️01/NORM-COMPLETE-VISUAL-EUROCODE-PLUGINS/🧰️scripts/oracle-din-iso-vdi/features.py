"""Writes the `.feature` file of the cross-check vectors of each artifact (rows tagged @known-divergence are expected to fail until the Rust side follows the standard)."""
import pathlib

ROOT = pathlib.Path("/home/deck/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts")
SUB = "🏅️standards/🔖️1/🪆️subsets/✳️any"
ART = {"din4108": "🧱️din4108", "din18599": "⚡️din18599", "din16798": "🌬️din16798", "iso16757": "📇️iso16757", "vdi3805": "🏭️vdi3805"}

HEAD = """@capability-{n}-1-evaluate
@oracle-{n}-1-cross-check
@comparison-numerical-v1
Feature: {title}
  The independent oracle `🔮️oracles/🌐️cross-checks/🐍️.py` ({libs}) writes the vectors in `🧫️fixtures/🔮️cross-checks/🔣️.json`; every implementation must reproduce the rows tagged @agree.
  Rows tagged @known-divergence document a difference between the Rust source and the standard (fixture section `divergences`) and are not asserted until the owner decides.

"""

SC = """  @id-{id}
  @level-quick
  @mode-differential
  @{tag}
  Scenario Outline: {text}
    Given the vector "<vector>" of the section "{section}"
    When {when}
    Then {then}
    Examples:
      | vector | tolerance |
{rows}
"""


def sc(id, tag, text, section, when, then, vectors, tol):
    return SC.format(id=id, tag=tag, text=text, section=section, when=when, then=then, rows="\n".join(f"      | {v} | {tol} |" for v in vectors))


FEATURES = {
    "din4108": (HEAD.format(n="din4108", title="DIN 4108 U-value, temperature factor and Glaser checks reproduce the third-party cross-checks", libs="exact fractions per EN ISO 6946 6.7.2, scipy brentq, IAPWS-IF97 and R14-08 vapour pressure, ISO 13788 condensation-plane iteration")
        + sc("u-combined", "agree", "the combined-method resistance equals the exact fraction result", "uValues", "the total resistance of the layer stack is evaluated", "it equals rTotal within the relative tolerance <tolerance>", [0, 1, 2, 3, 5, 6, 7], "1e-9")
        + sc("u-two-layers", "known-divergence", "two inhomogeneous layers with different fractions", "uValues", "the total resistance of the layer stack is evaluated", "it equals rTotal within the relative tolerance <tolerance>", [4], "1e-9")
        + sc("frsi-formula", "agree", "the temperature factor follows 1 - R_si / R_T for the stated surface resistance", "fRsi", "the temperature factor is evaluated with the surface resistance rSiRust", "it equals fRsiRust within the absolute tolerance <tolerance>", [0, 1, 3], "1e-12")
        + sc("frsi-standard", "known-divergence", "the temperature factor uses R_si = 0.25 m2K/W", "fRsi", "the temperature factor is evaluated", "it equals fRsiStandard within the absolute tolerance <tolerance>", [0, 1, 2, 3, 4], "1e-12")
        + sc("glaser-mass", "known-divergence", "the Glaser condensed and evaporated mass follow DIN 4108-3 Annex A", "glaser", "the winter condensed mass and the summer evaporated mass are evaluated", "they equal reference within the relative tolerance <tolerance>", [0, 1, 2, 3, 4], "1e-2")),
    "din18599": (HEAD.format(n="din18599", title="DIN V 18599-2 monthly balance reproduces the third-party cross-checks", libs="mpmath at 50 digits for the utilisation factor, EN ISO 13790 monthly method with a = a0 + tau / tau0 and calendar month hours")
        + sc("eta-heating", "known-divergence", "the gain utilisation factor of the heating balance", "utilisationFactor", "the utilisation factor of the gain-to-loss ratio gamma and the parameter a is evaluated", "it equals etaHeating within the relative tolerance <tolerance>", list(range(35)), "1e-9")
        + sc("eta-cooling", "agree", "the loss utilisation factor of the cooling balance", "utilisationFactor", "the cooling utilisation factor of gamma and a is evaluated", "it equals etaCooling within the relative tolerance <tolerance>", list(range(35)), "1e-9")
        + sc("month-standard", "known-divergence", "the monthly heating demand follows the standard method", "monthsStandard", "the monthly balance of the reference zone is evaluated", "Q_T, Q_V, Q_i, Q_s, eta, Q_H and Q_C equal the values within the relative tolerance <tolerance>", list(range(12)), "1e-6")
        + sc("zero-gain", "agree", "without gains the heating demand equals transmission plus ventilation loss", "zeroGain", "the monthly balance of the reference zone without internal and solar gains is evaluated", "Q_H equals Q_T + Q_V within the relative tolerance <tolerance>", [0], "1e-12")),
    "din16798": (HEAD.format(n="din16798", title="DIN EN 16798-1 comfort and ventilation reproduce the third-party cross-checks", libs="scipy brentq for PMV and its inversion, scipy solve_ivp for the CO2 balance, IAPWS-IF97 vapour pressure")
        + sc("pmv-brent", "agree", "the predicted mean vote equals the clothing heat balance root", "pmv", "the predicted mean vote is evaluated", "it equals pmvIso7730Vapour within the absolute tolerance <tolerance>", list(range(8)), "5e-3")
        + sc("invert", "agree", "the operative temperature at the PMV limit equals the root", "inversion", "the temperature at the category PMV limit is evaluated", "it equals referenceC within the absolute tolerance <tolerance>", [i for i in range(40) if i not in (19, 21, 23)], "1e-3")
        + sc("invert-bracket", "known-divergence", "the temperature at the PMV limit outside the bracket 16 to 30 C", "inversion", "the temperature at the category PMV limit is evaluated", "it equals referenceC within the absolute tolerance <tolerance>", [19, 21, 23], "1e-3")
        + sc("co2", "agree", "the steady-state CO2 concentration equals the integrated mass balance", "co2", "the required outdoor air of method 2 is evaluated", "it equals flowM3H within the relative tolerance <tolerance>", [0, 1, 2], "1e-6")),
    "iso16757": (HEAD.format(n="iso16757", title="ISO 16757-2 geometry volumes reproduce the third-party cross-checks", libs="scipy ConvexHull, numpy rotation and closed forms")
        + sc("volume-box", "agree", "the box volume equals the convex hull volume", "volumes", "the geometry volume is evaluated", "it equals solidVolumeM3 within the relative tolerance <tolerance>", [0], "1e-9")
        + sc("volume-solid", "known-divergence", "the volume of curved, rotated and boolean geometry", "volumes", "the geometry volume is evaluated", "it equals solidVolumeM3 within the relative tolerance <tolerance>", [1, 2, 3, 4, 5, 6], "1e-3")),
    "vdi3805": (HEAD.format(n="vdi3805", title="VDI 3805 radiator and valve laws reproduce the third-party cross-checks", libs="scipy brentq for the operating point, the EN 442 logarithmic mean excess temperature, the Kell water density")
        + sc("radiator", "agree", "the EN 442 power law at the logarithmic mean excess temperature", "radiators", "the radiator output is evaluated at the excess temperature deltaTLogK", "it equals outputW within the relative tolerance <tolerance>", list(range(5)), "2e-4")
        + sc("valve", "agree", "the valve pressure drop follows the square law of the kvs value", "valves", "the pressure drop at the flow is evaluated", "it equals dropBarRustLaw within the relative tolerance <tolerance>", list(range(4)), "1e-9")),
}

for name, text in FEATURES.items():
    out = ROOT / ART[name] / SUB / "🧪️tests/🔮️cross-checks" / "🥒️.feature"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(text, encoding="utf8")
    print(out.relative_to(ROOT), len(str(out.relative_to("/home/deck/semio")).encode()))
