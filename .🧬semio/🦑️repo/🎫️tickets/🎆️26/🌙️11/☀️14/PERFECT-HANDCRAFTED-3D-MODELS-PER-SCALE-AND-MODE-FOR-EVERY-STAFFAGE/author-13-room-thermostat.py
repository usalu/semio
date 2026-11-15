import sys; sys.path.insert(0, ".")
import importlib; L = importlib.import_module("author-13-lib"); from importlib import import_module
globals().update({k: getattr(L, k) for k in dir(L) if not k.startswith("_")})
ABS, LCD, LED = "abs-plastic", "lcd-glass", "led-lens"

def housing_secs(sc, n=10):
    return [sec((0, 0, -0.0015 + o), 0.045 * f, 0.045 * f, n) for o, f in sc]

def volumes():
    sec50 = [(0, 0.955), (0.001, 0.99), (0.0015, 1.0), (0.015, 1.0)]
    b50 = [
        loft("housing", "housing", ABS, "body", 0.9, housing_secs(sec50, 10)),
        box("display", "housing", LCD, "glazing", 0.12, (0.036, 0.021, 0.0016), (0, 0.0265, -0.0023)),
        ell("led", "housing", LED, "glazing", 0.35, (0.0035, 0.0035, 0.002), (0.0315, 0.0265, -0.0015)),
        rev("dial", "dial", ABS, "body", 0.62, [(-0.0015, 0.0205), (-0.0035, 0.021), (-0.0112, 0.021), (-0.0123, 0.0195), (-0.0125, 0.0165), (-0.0125, 0)]),
        box("pointer", "dial", ABS, "detail", 0.95, (0.011, 0.0145, 0.0075), (0, 0.01175, -0.00975)),
    ]
    cup = loft("th-casing", "housing", ABS, "body", 0.9, housing_secs(sec50, 10), True, False, region="casing", layer=None)
    cup.pop("layer")
    glass = loft("th-display", "housing", LCD, "glazing", 0.12, [sec((0, 0.0265, -0.0031), 0.01775, 0.01025, 24), sec((0, 0.0265, -0.0021), 0.01775, 0.01025, 24)], True, False, region="display", layer="glass")
    t50 = [cup, glass, th("th-led", "led", "led", "lens"), loft("th-dial", "dial", ABS, "body", 0.62, [sec((0, 0, w), r, r, 2) for w, r in [(0.0, 0.0205), (-0.0035, 0.021), (-0.0112, 0.021), (-0.0123, 0.0195), (-0.0125, 0.0165)]], False, True, region="dial", layer="knob")]
    s50 = [
        st(box("member-housing", "housing", ABS, "body", 0.5, (0.082, 0.082, 0.011), (0, 0, 0.0045), r=0.003), "housing", "member"),
        st(rev("member-dial", "dial", ABS, "body", 0.5, [(-0.0015, 0.006), (0.0105, 0.006), (0.0105, 0)], at=(0, 0, 0)), "dial", "member"),
        st(rev("joint-dial", "dial", ABS, "detail", 0.35, [(0.0005, 0.0095), (0.0105, 0.0095), (0.0105, 0)]), "dial", "joint"),
        st(rev("support-wall-screw-top", "housing", ABS, "detail", 0.25, [(0.0, 0.004), (0.0135, 0.004), (0.0135, 0)], at=(0, 0.034, 0)), "housing", "support"),
        st(cap("support-wall-screw-bottom", "housing", ABS, "detail", 0.25, (-0.006, -0.034, 0.0095), (0.006, -0.034, 0.0095), 0.004), "housing", "support"),
        st(ell("mass-housing", "housing", ABS, "detail", 0.15, (0.02, 0.02, 0.0045), (0, 0, 0.0045)), "housing", "mass"),
        st(ell("mass-dial", "dial", ABS, "detail", 0.15, (0.011, 0.011, 0.0045), (0, 0, 0.001)), "dial", "mass"),
    ]
    en50 = ["Housing: a lofted 90 mm square slab 15 mm deep with a 1.5 mm front bevel and rounded corners (not a box); the front of the loft stands at w = -0.0015 where the dial and the 1.6 mm glass window sit. Display, lamp (7 mm lens, glazing role) and the 11 mm pointer, which is a 7.5 mm thick inlay sunk into the dial so that it stays a printable solid while only 1 mm shows.",
            "Thermal: the casing is the exposed skin only, a front-capped loft open towards the wall (the back lies on the wall and exchanges no heat, so areaM2 = 0.012 m2 is the front and the four sides); display is a 1 mm glass plate, lamp and dial reuse their body solids. Every layer has one shell, the thickness of each is the layer's own.",
            "Structure: the housing core (82 x 82 x 11 mm), the 12 mm spindle and 19 mm bearing hub of the dial, two wall supports (round pin at the upper screw, 8 mm roller lying along u at the lower screw, B3) and one compact mass per bone at its centre of mass."]
    de50 = ["Gehäuse: eine gelofte quadratische Platte von 90 mm und 15 mm Tiefe mit 1,5 mm Frontfase und gerundeten Ecken (keine Box); die Front des Lofts steht bei w = -0,0015, dort sitzen Drehknopf und das 1,6 mm dicke Glasfenster. Anzeige, Lampe (7-mm-Linse, Rolle Verglasung) und der 11 mm breite Zeiger, der als 7,5 mm dicke Einlage im Drehknopf steckt und damit ein druckbarer Körper bleibt, obwohl nur 1 mm herausragt.",
            "Thermik: das Gehäuse ist nur die freie Haut, ein vorne geschlossener, zur Wand offener Loft (die Rückseite liegt an der Wand und tauscht keine Wärme aus, deshalb sind 0,012 m2 Front plus vier Seiten); die Anzeige ist eine 1 mm dicke Glasplatte, Lampe und Drehknopf verwenden ihre Körper. Jede Region hat eine Schicht, also eine Schale mit deren eigener Dicke.",
            "Tragwerk: der Gehäusekern (82 x 82 x 11 mm), die 12-mm-Spindel und die 19-mm-Lagernabe des Drehknopfs, zwei Wandauflager (runder Stift an der oberen Schraube, 8-mm-Rolle entlang u an der unteren, B3) und je eine kompakte Masse pro Knochen im Schwerpunkt."]
    v50 = scale_vol(50, b50, t50, s50, en50, de50)

    sec100 = [(0, 0.97), (0.001, 1.0), (0.02, 1.0)]
    hs = [sec((0, 0, -0.0065 + o), 0.045 * f, 0.045 * f, 8) for o, f in sec100]
    b100 = [
        loft("housing", "housing", ABS, "body", 0.9, hs),
        rev("dial", "dial", ABS, "body", 0.45, [(-0.0065, 0.024), (-0.0245, 0.024), (-0.0265, 0.021), (-0.0265, 0)]),
    ]
    cup = loft("th-casing", "housing", ABS, "body", 0.9, [sec((0, 0, -0.0065 + o), 0.045 * f, 0.045 * f, 8) for o, f in [(0, 0.97), (0.001, 1.0), (0.018, 1.0)]], True, False, region="casing")
    glass = loft("th-display", "housing", LCD, "glazing", 0.12, [sec((0, 0.0265, -0.0075), 0.01775, 0.01025, 16), sec((0, 0.0265, -0.0065), 0.01775, 0.01025, 16)], True, False, region="display", layer="glass")
    led = ell("th-led", "housing", LED, "glazing", 0.35, (0.0035, 0.0035, 0.0025), (0.0315, 0.0265, -0.0065), region="led", layer="lens")
    dial = loft("th-dial", "dial", ABS, "body", 0.45, [sec((0, 0, w), r, r, 2) for w, r in [(-0.0015, 0.0245), (-0.0225, 0.0245), (-0.0275, 0.019)]], False, True, region="dial", layer="knob")
    t100 = [cup, glass, led, dial]
    s100 = [
        st(box("member-housing", "housing", ABS, "body", 0.5, (0.078, 0.078, 0.016), (0, 0, 0.0035), r=0.004), "housing", "member"),
        st(rev("member-dial", "dial", ABS, "body", 0.5, [(-0.0065, 0.0095), (0.0135, 0.0095), (0.0135, 0)]), "dial", "member"),
        st(box("support-wall-screw-top", "housing", ABS, "detail", 0.25, (0.015, 0.015, 0.015), (0, 0.034, 0.0055)), "housing", "support"),
        st(box("support-wall-screw-bottom", "housing", ABS, "detail", 0.25, (0.015, 0.015, 0.015), (0, -0.034, 0.0055)), "housing", "support"),
        st(ell("mass-housing", "housing", ABS, "detail", 0.15, (0.022, 0.022, 0.008), (0, 0, 0.0035)), "housing", "mass"),
        st(ell("mass-dial", "dial", ABS, "detail", 0.15, (0.012, 0.012, 0.008), (0, 0, 0.0)), "dial", "mass"),
    ]
    en100 = ["Housing: a lofted slab of 90 mm by 20 mm depth with a 1 mm front bevel; the dial is one revolve with a rounded shoulder, 20 mm proud and 48 mm wide as in the 1:100 drawings. Display, lamp and pointer are no longer separate bodies (15 mm smallest feature): they are carried by the housing and the dial.",
              "Thermal: all four regions keep a solid, the casing as front-capped skin open to the wall, the 1 mm glass plate and 7 mm lens as thin glazing parts, the dial as its open-based skin 15 mm deep at the true 42 mm diameter, so the areas stay near areaM2.",
              "Structure: housing core and dial spindle as members, the dial hub is merged into the spindle, supports are 15 mm pads at both screws (pin and roller no longer distinguishable at this size) and the two centre-of-mass solids."]
    de100 = ["Gehäuse: eine geloftete Platte von 90 mm mit 20 mm Tiefe und 1 mm Frontfase; der Drehknopf ist ein einziger Drehkörper mit gerundeter Schulter, 20 mm hoch und 48 mm breit wie in den 1:100-Zeichnungen. Anzeige, Lampe und Zeiger sind keine eigenen Körper mehr (Mindestmaß 15 mm), das Gehäuse und der Knopf tragen sie.",
              "Thermik: alle vier Regionen behalten einen Körper, das Gehäuse als vorne geschlossene Haut mit Öffnung zur Wand, die 1 mm Glasplatte und die 7-mm-Linse als dünne Verglasungsteile, der Drehknopf als offene Haut von 15 mm Tiefe beim echten Durchmesser von 42 mm, damit die Flächen nahe an areaM2 bleiben.",
              "Tragwerk: Gehäusekern und Knopfspindel als Glieder, die Knopfnabe ist in der Spindel zusammengefasst, die Auflager sind 15-mm-Pads an beiden Schrauben (Stift und Rolle sind in dieser Grösse nicht mehr unterscheidbar) sowie die zwei Schwerpunktkörper."]
    return {"50": v50, "100": scale_vol(100, b100, t100, s100, en100, de100), "200": unavailable(200), "500": unavailable(500)}

if __name__ == "__main__":
    write("appliances/room-thermostat", volumes())
    print(run("check", "appliances/room-thermostat", "--quiet"))
