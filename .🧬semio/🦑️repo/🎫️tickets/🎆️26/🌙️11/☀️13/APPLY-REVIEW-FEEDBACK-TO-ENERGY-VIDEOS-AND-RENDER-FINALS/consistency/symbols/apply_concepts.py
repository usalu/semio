"""📖 Fix forbidden and changed symbols plus wording in the concepts reference (read-modify-write in one pass)."""
ROOT = r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy"
PATH = ROOT + r"\📖️concepts\📝️.md"
QD = "Q\u0307"
PAIRS = [
    ("ΔT_eq", "Δθ_eq"),
    ("ΔT", "Δθ"),
    ("x_s(ϑ)", "x_s(θ)"),
    ("ϑ", "θ"),
    ("Φ_V = " + QD + "_V,sens", QD + "_V = " + QD + "_V,sens"),
    ("**Φ_V = V · n · c_Luft", "**" + QD + "_V = V · n · c_Luft"),
    ("**Φ_V = V · n · (1", "**" + QD + "_V = V · n · (1"),
    ("ergeben Φ_V, das", "ergeben " + QD + "_V, das"),
    ("Φ_T ≈ 2 300 W plus Φ_V ≈ 1 500 W", QD + "_T ≈ 2 300 W plus " + QD + "_V ≈ 1 500 W"),
    ("**Φ_T = Σ (U_i", "**" + QD + "_T = Σ (U_i"),
    (QD + "_L = sensibel + latent", QD + "_V = sensibel + latent"),
    (QD + "_L = " + QD + "_V,sens + " + QD + "_V,lat", QD + "_V = " + QD + "_V,sens + " + QD + "_V,lat"),
    (QD + "_Pers", QD + "_i,p"),
    (QD + "_Geräte", QD + "_i,e"),
    (QD + "_Licht", QD + "_i,l"),
    ("R_ges", "R_T"),
    ("I_S,max", "I_s,max"),
    ("I_reduziert", "I_s,red"),
    ("**F_V** | Verschattungsfaktor (Außen-/Innenschatten)", "**F_C** | Sonnenschutzfaktor (Außen-/Innenschatten)"),
    ("F_F · F_V", "F_F · F_C"),
    ("I_s,max · F_V", "I_s,max · F_C"),
    ("**Wärmeleitung (" + QD + "_k)**", "**Wärmeleitung (Φ_k)**"),
    ("**Konvektion (" + QD + "_c)**", "**Konvektion (Φ_c)**"),
    ("**Strahlung (" + QD + "_r)**", "**Strahlung (Φ_r)**"),
    ("(R, U, " + QD + ")", "(R, U, Φ)"),
    (
        "**" + QD + "_c,i + " + QD + "_r,i = " + QD + "_k = " + QD + "_c,e + " + QD + "_r,e**",
        "**Φ_c,si + Φ_r,si = Φ_k = Φ_c,se + Φ_r,se**",
    ),
    ("**" + QD + " = U · A · Δθ**", "**Φ = U · A · Δθ**"),
    ("**R steigt**, **" + QD + " sinkt**", "**R steigt**, **Φ sinkt**"),
    ("1 000 ppm Pettenkoferwert", "1 000 ppm Grenzwert nach Pettenkofer"),
    ("Pettenkoferwert", "Grenzwert nach Pettenkofer"),
    ("Übergabe an Heizung und Kühlung", "Übergabe an Heizung und Kühltechnik"),
    ("was Heizung und Kühlung übernehmen", "was Heizung und Kühltechnik übernehmen"),
    ("weil sie Heizung und Kühlung gleichermaßen betrifft", "weil sie Heizung und Kühltechnik gleichermaßen betrifft"),
    ("Heizung und Kühlung setzen danach", "Heizung und Kühltechnik setzen danach"),
    ("steht die Lüftung vor Heizung und Kühlung", "steht die Lüftung vor Heizung und Kühltechnik"),
    ("Heizung zählt nur den sensiblen Anteil, Kühlung beide", "Heizung zählt nur den sensiblen Anteil, Kühltechnik beide"),
    ("| Δθ, Δθ |", "| Δθ |"),
]


def main():
    with open(PATH, encoding="utf-8", newline="") as h:
        text = h.read()
    for old, new in PAIRS:
        count = text.count(old)
        print(count, old[:60].replace("\u0307", "^"))
        text = text.replace(old, new)
    with open(PATH, "w", encoding="utf-8", newline="") as h:
        h.write(text)


main()
