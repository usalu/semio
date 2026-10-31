"""NW-MA shared references, materials and environment."""
from lib import *

REFS = {
    "incropera-2011": "Incropera, F. P., DeWitt, D. P., Bergman, T. L., Lavine, A. S. (2011). Fundamentals of Heat and Mass Transfer, 7th ed., Wiley. Ch. 7 and 9 (external and natural convection), Appendix A (thermophysical properties of steel, glass, rubber, aluminium).",
    "ashrae-2021": "ASHRAE Handbook Fundamentals (2021), Chapter 4 Heat Transfer and Chapter 25 Heat, Air and Moisture Control (surface emissivity and solar absorptivity of painted and bare metals, glass).",
    "heywood-2018": "Heywood, J. B. (2018). Internal Combustion Engine Fundamentals, 2nd ed., McGraw-Hill. Ch. 4 and 12 (air flow, exhaust gas temperature, mass flow and composition of diesel engines).",
    "directive-96-53-ec": "Council Directive 96/53/EC of 25 July 1996 laying down maximum authorised dimensions in national and international traffic (width 2.55 m for road vehicles).",
    "iso-6746": "ISO 6746-1:2003 Earth-moving machinery, definitions of dimensions and codes (wheel loader dimensions, bucket tipping angles).",
    "iso-9244": "ISO 3411:2007 Earth-moving machinery, physical dimensions of operators and minimum operator space envelope; ISO 7457 turning circles of wheeled machines.",
    "iso-8528": "ISO 8528-1:2018 Reciprocating internal combustion engine driven alternating current generating sets, application, ratings and performance (prime power, standby power, load classes).",
    "en-14511": "EN 14511-2:2022 Air conditioners, liquid chilling packages and heat pumps for space heating and cooling, test conditions (A7/W35 heating, A35/W7 cooling, outdoor air flow definitions).",
    "en-280": "EN 280:2013 Mobile elevating work platforms, design calculations, stability criteria, construction, safety, examinations and tests (platform load, guard rail height 1.1 m).",
    "class-data-note": "Manufacturer class data (public data sheets of the product class, quoted as orders of magnitude from memory of the data sheet families, not retrieved for this asset); every number derived from it is tagged authored-estimate.",
}

DEFAULT_ENV = {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 8, "groundTempC": 30, "relativeHumidity": 0.5, "windMS": 0.5, "windDirectionDeg": 180, "solarWM2": 400, "diffuseFraction": 0.2, "pressurePa": 101325}


def references(ids, extra=None):
    out = [{"id": i, "citation": REFS[i]} for i in ids]
    if extra:
        out += extra
    return out


def mat_paint(id, en, de, alpha=0.55):
    return material(id, en, de, 0.9, alpha, 7850, 50, 460, "literature", "ashrae-2021")


MAT_GLASS = material("glass", "Safety glass", "Sicherheitsglas", 0.84, 0.2, 2500, 1.0, 840, "literature", "ashrae-2021")
MAT_RUBBER = material("rubber", "Tyre rubber", "Reifengummi", 0.93, 0.9, 1100, 0.15, 1700, "literature", "incropera-2011")
MAT_IRON = material("cast-iron", "Cast iron", "Gusseisen", 0.8, 0.7, 7200, 46, 460, "literature", "ashrae-2021")
MAT_EXHAUST = material("exhaust-steel", "Oxidised exhaust steel", "Oxidierter Auspuffstahl", 0.8, 0.7, 7850, 45, 460, "literature", "ashrae-2021")
MAT_ALU = material("aluminium", "Aluminium sheet", "Aluminiumblech", 0.25, 0.4, 2700, 200, 900, "literature", "incropera-2011")
MAT_GALV = material("galvanised-steel", "Galvanised steel", "Verzinkter Stahl", 0.3, 0.55, 7850, 50, 460, "literature", "ashrae-2021")


def steel_layer(id="plate", thk=0.004, mat="paint-steel"):
    return layer(id, f"Steel plate {int(thk*1000)} mm", f"Stahlblech {int(thk*1000)} mm", "metal", mat, thk, round(thk / 50, 6), round(7850 * 460 * thk), None, "literature", "incropera-2011")


def structure(en, de):
    return {"gravityMS2": 9.80665, "dynamics": "quasi-static", "supports": [], "assumptions": [tx(en, de)], "clinical": False}
