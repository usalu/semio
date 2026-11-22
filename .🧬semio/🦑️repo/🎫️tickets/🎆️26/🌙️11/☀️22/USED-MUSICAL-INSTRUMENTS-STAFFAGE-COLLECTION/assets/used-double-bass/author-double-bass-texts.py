"""📜 Texts, masses, materials, thermal model, structure, decisions and the poses and clips of the used double bass, executed into the namespace of author-double-bass.py."""

BASS_PARTS_KG = [("top plate, spruce 4.5 mm", 1.10, 0.50), ("flat back, maple 4 mm", 1.40, 0.50), ("ribs, maple 2 mm", 0.80, 0.50), ("blocks, linings, bass bar and back bars", 1.30, 0.55),
                 ("neck, heel, pegbox and scroll", 1.10, 1.45), ("ebony fingerboard", 1.70, 1.08), ("nut", 0.02, 1.53), ("ebony tailpiece", 0.35, 0.18), ("saddle and tail wire", 0.10, 0.01),
                 ("bridge", 0.24, 0.48), ("machine heads", 0.80, 1.62), ("strings", 0.16, 0.95), ("endpin socket", 0.20, -0.012), ("wheel clamp", 0.12, -0.036), ("varnish", 0.08, 0.50)]
M_BASS_BONE = sum(m for _, m, _ in BASS_PARTS_KG)
COM_BASS = sum(m * u for _, m, u in BASS_PARTS_KG) / M_BASS_BONE
MASS = {"bass": (M_BASS_BONE, COM_BASS, 0.0), "endpin": (0.28, 0.22, 0.0), "wheel": (0.25, 0.0, 0.0), "cover": (2.40, 0.62, 0.0), "cover-flap": (0.56, 0.55, 0.36), "stool": (2.86, 0.0, 0.22), "stool-seat": (1.61, 0.0, 0.02)}
M_TOTAL = sum(m for m, _, _ in MASS.values())
MASS_SOURCE = {"bass": ("authored-estimate", "bass-making"), "endpin": ("literature", "en-10088"), "wheel": ("authored-estimate", "bass-accessories"), "cover": ("authored-estimate", "bass-accessories"),
               "cover-flap": ("authored-estimate", "bass-accessories"), "stool": ("literature", "en-10088"), "stool-seat": ("authored-estimate", "bass-accessories")}
MASS_ENTRY = {k: {"kg": r6(m), "comX": r6(cx), "comY": r6(cy), "distribution": "point", "source": MASS_SOURCE[k][0], "reference": MASS_SOURCE[k][1]} for k, (m, cx, cy) in MASS.items()}
SECTIONS = {
    "bass": {"areaM2": 0.0063, "secondMomentM4": 4.2525e-06, "extremeFibreM": 0.045, "material": "worn-maple", "title": T("Neck at the heel, maple 70 x 90 mm", "Hals am Fuß, Ahorn 70 x 90 mm")},
    "endpin": {"areaM2": 7.854e-05, "secondMomentM4": 4.909e-10, "extremeFibreM": 0.005, "material": "steel", "title": T("Steel endpin rod 10 mm", "Stahlstachel 10 mm")},
    "stool": {"areaM2": 9.66e-05, "secondMomentM4": 5.12e-09, "extremeFibreM": 0.011, "material": "chrome-steel", "title": T("One leg of the star base, steel tube 22 x 1.5 mm", "Ein Bein des Sternfußes, Stahlrohr 22 x 1,5 mm")},
}

AREA_PROBE = bool(os.environ.get("BASS_AREA_PROBE"))
AREA = {"cover": 5.789, "top": 1.109, "back-ribs": 1.692, "neck-scroll": 0.246, "ebony": 0.257, "bridge": 0.057, "strings": 0.033, "fittings": 0.073, "wheels": 0.029, "stool-frame": 0.251, "stool-seat": 0.243}
if AREA_PROBE:
    AREA = {k: 100.0 for k in AREA}

SUMMARY = T(
    f"A used 3/4 orchestral double bass of the flat-back workshop type as a used-instrument dealer offers it: 1.80 m from the endpin socket to the scroll, body 1.10 m with bouts of 0.68, 0.38 and 0.51 m and 0.20 m ribs, 1.05 m string length, about 10 kg, set up with four steel strings over a 160 mm bridge; it lies on its bass-side rib in its unzipped 20 mm padded cover with the twin bass wheel clamped on the endpin and its height-adjustable bass stool beside it. Rosin dust under the strings, varnish worn through at the upper bout and the thumb position, a repaired crack in the lower bout, an edge chip, tape markers on the fingerboard, a taped and scuffed cover and a torn stool seat are what its use left behind.",
    f"Ein gebrauchter 3/4-Orchesterkontrabass der Werkstattbauart mit flachem Boden, wie ihn ein Gebrauchtinstrumentenhandel anbietet: 1,80 m von der Stachelbuchse bis zur Schnecke, Korpus 1,10 m mit Bügen von 0,68, 0,38 und 0,51 m und 0,20 m Zargen, 1,05 m Mensur, etwa 10 kg, eingerichtet mit vier Stahlsaiten über einem 160-mm-Steg; er liegt auf der Bass-Zarge in seiner aufgezippten 20-mm-Polsterhülle mit dem am Stachel festgeklemmten Zwillings-Kontrabassrad, daneben sein höhenverstellbarer Kontrabasshocker. Kolophoniumstaub unter den Saiten, an Oberbug und Daumenlage durchgegriffener Lack, ein reparierter Riss im Unterbug, eine Randabplatzung, Lagenmarkierungen aus Klebeband auf dem Griffbrett, eine geflickte und abgestoßene Hülle und ein eingerissener Hockersitz sind, was sein Gebrauch hinterlassen hat.")

PROVENANCE = {
    "author": "Semio staffage instrument authors (used instruments wave, row 07)", "method": "handcrafted", "created": "2026-11-22",
    "references": [
        {"id": "hornbostel-sachs", "citation": "von Hornbostel, E. M.; Sachs, C. (1914): Systematik der Musikinstrumente. Zeitschrift für Ethnologie 46, 553-590; English: Galpin Society Journal 14 (1961), 3-29. Double bass: 321.322-71, necked box lute sounded by a bow."},
        {"id": "brun-1989", "citation": "Brun, P. (1989): A New History of the Double Bass. Villeneuve d'Ascq: Paul Brun Productions (viol-cornered flat-back basses, sloping shoulders, string lengths of 3/4 orchestral basses 1.04 to 1.06 m)."},
        {"id": "bass-making", "citation": "Workshop and catalogue dimensions of 3/4 orchestral double basses of the German workshop tradition (Saxon and Franconian workshops, laminated and hybrid student and orchestra models): body length 1.08 to 1.12 m, upper / middle / lower bout 0.50-0.52 / 0.37-0.39 / 0.66-0.69 m, ribs 0.19-0.21 m, total length about 1.80 m, 9 to 11 kg; bridge 150-165 mm, fingerboard 0.88-0.92 m, after-length about one sixth of the string length."},
        {"id": "string-tension", "citation": "Published tension charts of steel-core orchestral double bass strings for 3/4 basses at 1.05 m: E 30, A 28, D 28, G 25 kgf (about 1.1 kN in all); core diameters 1.6 to 2.6 mm."},
        {"id": "bass-accessories", "citation": "Catalogue data of double bass accessories: padded gig bags 15-20 mm foam with nylon shell, 2.5 to 3.5 kg, backpack straps and carry handles; twin-wheel bass wheels clamped on a 10 mm endpin, 70-80 mm wheels; height-adjustable bass stools 0.66-0.80 m seat height with star base and footrest ring, 4 to 6 kg."},
        {"id": "wood-handbook", "citation": "Forest Products Laboratory (2010): Wood Handbook, Wood as an Engineering Material, FPL-GTR-190, USDA (spruce about 450 kg/m3, E 10 GPa along the grain; maple 650 kg/m3, 12 GPa; thermal conductivity across the grain 0.11 to 0.17 W/mK; specific heat about 1.6 kJ/kgK)."},
        {"id": "wegst-2006", "citation": "Wegst, U. G. K. (2006): Wood for sound. American Journal of Botany 93(10), 1439-1448 (spruce, maple, ebony densities and moduli of instrument woods; ebony about 1100 kg/m3, 17 GPa)."},
        {"id": "en-10088", "citation": "EN 10088-1:2014 Stainless steels, list of stainless steels (density 7900 kg/m3, E 200 GPa) and EN 10305 precision steel tubes for the stool frame (22 x 1.5 mm and 40 x 1.5 mm)."},
        {"id": "incropera-2018", "citation": "Bergman, Lavine, Incropera, DeWitt: Fundamentals of Heat and Mass Transfer, 8th ed., Wiley 2018 (emissivities: varnished wood 0.90, black rubber 0.92, polished chrome 0.10, tarnished chrome 0.25, brass 0.30, nylon fabric 0.85-0.90)."},
        {"id": "en-iso-10456", "citation": "EN ISO 10456:2007 Building materials and products, hygrothermal properties (PE foam 0.035 W/mK, plywood 0.13 W/mK, softwood and hardwood across the grain 0.13 to 0.18 W/mK)."},
        {"id": "din-iso-128-50", "citation": "DIN ISO 128-50:2002 Technical drawings, basic conventions for representing areas on cuts and sections."},
    ],
    "notes": T(
        "The bass is a type, not a maker's instrument: every dimension is a catalogue-typical value of 3/4 orchestral flat-back basses (authored estimates, nothing is measured). The body is modelled as a closed shell of ribs and back with the arched top on it; the inside air, the bass bar, the sound post and the linings are inside and not modelled, their mass is counted. The cover, the bass wheel and the stool are typical accessories without brand.",
        "Der Bass ist ein Typ, kein Instrument eines Herstellers: jedes Maß ist ein katalogtypischer Wert von 3/4-Orchesterbässen mit flachem Boden (Schätzwerte, nichts ist gemessen). Der Korpus ist als geschlossene Schale aus Zargen und Boden mit aufgesetzter gewölbter Decke modelliert; die Innenluft, der Bassbalken, der Stimmstock und die Reifchen liegen innen und sind nicht modelliert, ihre Masse ist mitgezählt. Hülle, Kontrabassrad und Hocker sind typische Zubehörteile ohne Marke."),
}

MATERIALS = [
    {"id": "varnished-spruce", "title": T("Spruce top, aged amber varnish, worn at the edges", "Fichtendecke, gealterter Bernsteinlack, an den Rändern abgegriffen"), "emissivity": 0.90, "solarAbsorptivity": 0.70, "densityKgM3": 450, "conductivityWmK": 0.12, "specificHeatJKgK": 1700, "elasticModulusPa": 10000000000, "strengthPa": 40000000, "source": "literature", "reference": "wood-handbook"},
    {"id": "varnished-maple", "title": T("Flamed maple back and ribs, aged varnish, scratched", "Geflammter Ahornboden und Zargen, gealterter Lack, verkratzt"), "emissivity": 0.90, "solarAbsorptivity": 0.72, "densityKgM3": 650, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "elasticModulusPa": 12000000000, "strengthPa": 100000000, "source": "literature", "reference": "wood-handbook"},
    {"id": "worn-maple", "title": T("Maple neck and scroll, varnish worn to bare wood at the thumb position", "Ahornhals und Schnecke, Lack an der Daumenlage bis aufs Holz abgegriffen"), "emissivity": 0.88, "solarAbsorptivity": 0.65, "densityKgM3": 650, "conductivityWmK": 0.16, "specificHeatJKgK": 1600, "elasticModulusPa": 12000000000, "strengthPa": 100000000, "source": "literature", "reference": "wood-handbook"},
    {"id": "bridge-maple", "title": T("Unvarnished maple bridge, dark with rosin and hand grease", "Unlackierter Ahornsteg, dunkel von Kolophonium und Handschweiß"), "emissivity": 0.90, "solarAbsorptivity": 0.55, "densityKgM3": 700, "conductivityWmK": 0.17, "specificHeatJKgK": 1600, "elasticModulusPa": 12000000000, "source": "literature", "reference": "wegst-2006"},
    {"id": "ebony", "title": T("Ebony fingerboard, nut, saddle and tailpiece, grooved by the strings", "Ebenholz-Griffbrett, Sattel, Untersattel und Saitenhalter, von den Saiten eingekerbt"), "emissivity": 0.90, "solarAbsorptivity": 0.90, "densityKgM3": 1100, "conductivityWmK": 0.20, "specificHeatJKgK": 1500, "elasticModulusPa": 17000000000, "source": "literature", "reference": "wegst-2006"},
    {"id": "string-steel", "title": T("Steel-core strings with flat chrome winding, dulled under the fingers", "Stahlkernsaiten mit Chrom-Flachumspinnung, unter den Fingern matt"), "emissivity": 0.30, "solarAbsorptivity": 0.50, "densityKgM3": 7850, "conductivityWmK": 40, "specificHeatJKgK": 470, "elasticModulusPa": 200000000000, "source": "literature", "reference": "incropera-2018"},
    {"id": "steel", "title": T("Steel endpin, tail wire, clamp and screws, scratched", "Stahlstachel, Saitenhalterdraht, Klemme und Schrauben, verkratzt"), "emissivity": 0.35, "solarAbsorptivity": 0.55, "densityKgM3": 7900, "conductivityWmK": 16, "specificHeatJKgK": 500, "elasticModulusPa": 200000000000, "strengthPa": 500000000, "source": "literature", "reference": "en-10088"},
    {"id": "brass", "title": T("Brass machine-head plates and keys, tarnished", "Messing-Mechanikplatten und -flügel, angelaufen"), "emissivity": 0.30, "solarAbsorptivity": 0.60, "densityKgM3": 8500, "conductivityWmK": 110, "specificHeatJKgK": 380, "source": "literature", "reference": "incropera-2018"},
    {"id": "rubber", "title": T("Black rubber of the wheels, the endpin stopper and the stool glides, worn flat", "Schwarzer Gummi der Rollen, des Stachelstoppers und der Hockergleiter, abgelaufen"), "emissivity": 0.92, "solarAbsorptivity": 0.90, "densityKgM3": 1150, "conductivityWmK": 0.20, "specificHeatJKgK": 1800, "source": "literature", "reference": "incropera-2018"},
    {"id": "fhole-shadow", "title": T("Dark inside of the body seen through the f-holes", "Dunkles Korpusinnere, durch die F-Löcher gesehen"), "emissivity": 0.90, "densityKgM3": 450, "conductivityWmK": 0.12, "specificHeatJKgK": 1700, "source": "illustrative"},
    {"id": "padded-nylon", "title": T("Black 600 D nylon over 20 mm polyethylene foam, faded and scuffed", "Schwarzes 600-D-Nylon über 20 mm Polyethylenschaum, verblichen und abgestoßen"), "emissivity": 0.88, "solarAbsorptivity": 0.85, "densityKgM3": 45, "conductivityWmK": 0.036, "specificHeatJKgK": 1500, "source": "literature", "reference": "en-iso-10456"},
    {"id": "webbing", "title": T("Polypropylene webbing of handles and straps", "Polypropylen-Gurtband der Griffe und Riemen"), "emissivity": 0.88, "solarAbsorptivity": 0.85, "densityKgM3": 900, "conductivityWmK": 0.22, "specificHeatJKgK": 1900, "source": "literature", "reference": "incropera-2018"},
    {"id": "chrome-steel", "title": T("Chromed steel tube of the stool, tarnished and worn at the footrest", "Verchromtes Stahlrohr des Hockers, angelaufen und am Fußring abgerieben"), "emissivity": 0.25, "solarAbsorptivity": 0.40, "densityKgM3": 7850, "conductivityWmK": 50, "specificHeatJKgK": 470, "elasticModulusPa": 210000000000, "strengthPa": 235000000, "source": "literature", "reference": "en-10088"},
    {"id": "black-plastic", "title": T("Black polyamide clamp knob", "Schwarzer Polyamid-Klemmknauf"), "emissivity": 0.92, "densityKgM3": 1140, "conductivityWmK": 0.25, "specificHeatJKgK": 1700, "source": "literature", "reference": "incropera-2018"},
    {"id": "plywood", "title": T("Beech plywood seat board 18 mm", "Buchensperrholz-Sitzplatte 18 mm"), "emissivity": 0.90, "densityKgM3": 680, "conductivityWmK": 0.13, "specificHeatJKgK": 1600, "elasticModulusPa": 9000000000, "source": "literature", "reference": "en-iso-10456"},
    {"id": "worn-vinyl", "title": T("Black vinyl over foam, torn and taped", "Schwarzes Kunstleder über Schaum, eingerissen und geklebt"), "emissivity": 0.90, "solarAbsorptivity": 0.88, "densityKgM3": 120, "conductivityWmK": 0.05, "specificHeatJKgK": 1500, "source": "literature", "reference": "en-iso-10456"},
]


def layer(lid, en, de, kind, material, t, k, rho, c, src="literature", ref="en-iso-10456"):
    """🧶 One thermal layer from thickness, conductivity, density and specific heat."""
    return {"id": lid, "title": T(en, de), "kind": kind, "material": material, "thicknessM": t, "resistanceM2KW": r6(t / k), "heatCapacityJM2K": round(t * rho * c, 1), "source": src, "reference": ref}


def region(rid, en, de, kind, material, length, geometry, lay, extra=None, src="authored-estimate", ref="bass-making"):
    """🔥 One passive thermal region with its area from AREA and its heat capacity from its layer."""
    out = {"id": rid, "title": T(en, de), "kind": kind, "model": "passive", "material": material, "areaM2": AREA[rid], "characteristicLengthM": length, "geometry": geometry, "skyViewFactor": 0.1,
           "layers": [lay], "heatCapacityJM2K": lay["heatCapacityJM2K"], "source": src, "reference": ref}
    out.update(extra or {})
    return out


THERMAL = {
    "environment": {"airTempC": 20, "meanRadiantTempC": 20, "skyTempC": 19, "groundTempC": 19, "relativeHumidity": 0.45, "windMS": 0.1, "windDirectionDeg": 0, "solarWM2": 0, "diffuseFraction": 1, "pressurePa": 101325},
    "passengers": [], "operating": [],
    "regions": [
        region("cover", "Padded cover: nylon shell over polyethylene foam", "Polsterhülle: Nylonhülle über Polyethylenschaum", "surface", "padded-nylon", 0.7, "vertical-plate",
               layer("foam", "Polyethylene foam 20 mm under the nylon", "Polyethylenschaum 20 mm unter dem Nylon", "insulation", "padded-nylon", 0.02, 0.036, 45, 1500), ref="bass-accessories"),
        region("top", "Arched spruce top, varnished", "Gewölbte Fichtendecke, lackiert", "surface", "varnished-spruce", 0.6, "vertical-plate",
               layer("spruce", "Spruce plate 4.5 mm", "Fichtenplatte 4,5 mm", "timber", "varnished-spruce", 0.0045, 0.12, 450, 1700, ref="wood-handbook")),
        region("back-ribs", "Flat maple back and ribs, varnished", "Flacher Ahornboden und Zargen, lackiert", "surface", "varnished-maple", 0.6, "vertical-plate",
               layer("maple", "Maple plate 4 mm", "Ahornplatte 4 mm", "timber", "varnished-maple", 0.004, 0.16, 650, 1600, ref="wood-handbook"), {"ratedPowerW": 0, "ratedPowerBasis": "passive"}),
        region("neck-scroll", "Maple neck, heel, pegbox and scroll", "Ahornhals, Halsfuß, Wirbelkasten und Schnecke", "surface", "worn-maple", 0.05, "cylinder",
               layer("maple", "Solid maple, thermally active 15 mm", "Massiver Ahorn, thermisch wirksam 15 mm", "timber", "worn-maple", 0.015, 0.16, 650, 1600, ref="wood-handbook")),
        region("ebony", "Ebony fingerboard, nut, saddle and tailpiece", "Ebenholz-Griffbrett, Sattel, Untersattel und Saitenhalter", "surface", "ebony", 0.08, "vertical-plate",
               layer("ebony", "Ebony 10 mm", "Ebenholz 10 mm", "timber", "ebony", 0.010, 0.20, 1100, 1500, ref="wegst-2006")),
        region("bridge", "Maple bridge", "Ahornsteg", "surface", "bridge-maple", 0.16, "vertical-plate",
               layer("maple", "Maple 9 mm (half the bridge)", "Ahorn 9 mm (halber Steg)", "timber", "bridge-maple", 0.009, 0.17, 700, 1600, ref="wegst-2006")),
        region("strings", "Four steel-core strings", "Vier Stahlkernsaiten", "surface", "string-steel", 0.0042, "cylinder",
               layer("steel", "Steel string 1 mm (core to winding)", "Stahlsaite 1 mm (Kern bis Umspinnung)", "metal", "string-steel", 0.001, 40, 7850, 470, ref="string-tension")),
        region("fittings", "Endpin, socket, wheel clamp, tail wire and machine heads", "Stachel, Stachelbuchse, Radklemme, Saitenhalterdraht und Mechaniken", "surface", "steel", 0.02, "cylinder",
               layer("metal", "Steel and brass 3 mm", "Stahl und Messing 3 mm", "metal", "steel", 0.003, 16, 7900, 500, ref="en-10088")),
        region("wheels", "Rubber wheels of the bass wheel and endpin stopper", "Gummirollen des Kontrabassrads und Stachelstopper", "tyre", "rubber", 0.075, "cylinder",
               layer("rubber", "Rubber tread 8 mm", "Gummilauffläche 8 mm", "polymer", "rubber", 0.008, 0.20, 1150, 1800, ref="incropera-2018"), ref="bass-accessories"),
        region("stool-frame", "Chromed steel star base, column and footrest ring of the stool", "Verchromter Sternfuß, Säule und Fußring des Hockers", "surface", "chrome-steel", 0.022, "cylinder",
               layer("tube", "Steel tube wall 1.5 mm", "Stahlrohrwand 1,5 mm", "metal", "chrome-steel", 0.0015, 50, 7850, 470, ref="en-10088"), ref="bass-accessories"),
        region("stool-seat", "Padded vinyl seat on its plywood board", "Gepolsterter Kunstledersitz auf Sperrholzplatte", "surface", "worn-vinyl", 0.35, "horizontal-plate-up",
               layer("foam", "Vinyl over foam 40 mm", "Kunstleder über Schaum 40 mm", "insulation", "worn-vinyl", 0.04, 0.05, 120, 1500), ref="bass-accessories"),
    ],
    "emitters": [],
    "assumptions": [
        T("The bass is an acoustic instrument and draws no power (passive, 0 W, stated on the back and ribs); everything sits at the 20 degC and 45 % relative humidity of a heated dealer's showroom without sun. The cover, the bass and the stool are passive bodies with heat capacity; the hollow body is modelled closed, its inside air exchanges through the f-holes only.",
          "Der Bass ist ein akustisches Instrument und nimmt keine Leistung auf (passiv, 0 W, an Boden und Zargen angegeben); alles steht bei 20 °C und 45 % relativer Feuchte eines beheizten Verkaufsraums ohne Sonne. Hülle, Bass und Hocker sind passive Körper mit Wärmekapazität; der hohle Korpus ist geschlossen modelliert, seine Innenluft tauscht nur durch die F-Löcher aus."),
        T("Played, the left hand warms the neck and the strings under the fingers (contact at 33 degC on 6 % and 3 % of their surface), the player's body touches the back at the upper bout (32 degC on 2 %) and the player half-sits on the stool seat (33 degC on 45 %); stored and rolled, only the floor touches the cover, the front panel and the stool glides (19 degC).",
          "Gespielt wärmt die linke Hand den Hals und die Saiten unter den Fingern (Kontakt mit 33 °C auf 6 % und 3 % ihrer Fläche), der Körper des Spielers berührt den Boden am Oberbug (32 °C auf 2 %) und der Spieler sitzt halb auf dem Hockersitz (33 °C auf 45 %); gelagert und gerollt berührt nur der Boden die Hülle, die Vorderseite und die Hockergleiter (19 °C)."),
        T("Aged surfaces differ from new ones: the darkened amber varnish absorbs 0.70 of the light instead of about 0.55 new, the faded black nylon 0.85, tarnished chrome emits 0.25 instead of 0.10 polished, tarnished brass 0.30 (Incropera; authored estimates for the ageing).",
          "Gealterte Oberflächen unterscheiden sich von neuen: der nachgedunkelte Bernsteinlack absorbiert 0,70 des Lichts statt etwa 0,55 neu, das verblichene schwarze Nylon 0,85, angelaufenes Chrom emittiert 0,25 statt 0,10 poliert, angelaufenes Messing 0,30 (Incropera; Schätzwerte für die Alterung)."),
    ],
}

STRUCTURE = {"gravityMS2": 9.81, "dynamics": "quasi-static", "supports": [], "assumptions": [
    T(f"No support is permanent: each pose names its contacts. Stored, the cover rests on its lower and upper bout on the floor, the unzipped front panel lies flat beside it, the bass lies in it on its bass-side rib (two contacts on the foam of the band) and the stool stands on four glides; rolled, the {MASS['bass'][0] + MASS['endpin'][0] + MASS['wheel'][0]:.1f} kg bass in its zipped cover stands on the twin wheels (roller) and the hand holds the sleeve grip (pin); played, it stands on the rubber stopper of the endpin extended {PLAYED_E * 1000:.0f} mm, leaning {LEAN:.0f} degrees back against the player's body (roller at the upper back).",
      f"Kein Auflager ist dauerhaft: jede Pose nennt ihre Kontakte. Gelagert liegt die Hülle mit Unter- und Oberbug auf dem Boden, die aufgezippte Vorderseite flach daneben, der Bass darin auf der Bass-Zarge (zwei Kontakte auf dem Schaum des Seitenbands) und der Hocker steht auf vier Gleitern; gerollt steht der {MASS['bass'][0] + MASS['endpin'][0] + MASS['wheel'][0]:.1f} kg schwere Bass in der geschlossenen Hülle auf den Zwillingsrollen (Rolle), die Hand hält den Griff am Halsschlauch (Gelenk); gespielt steht er auf dem Gummistopper des um {PLAYED_E * 1000:.0f} mm ausgezogenen Stachels und lehnt um {LEAN:.0f} Grad zurück am Körper des Spielers (Rolle am oberen Boden)."),
    T("The four strings are tuned to pitch in every pose (E 300, A 280, D 280, G 250 N, 1.11 kN together): they pull the nut and the tailpiece towards the bridge and press the bridge into the top with about 0.67 kN at a deflection of 35 degrees over the bridge; the three loads are self-equilibrated and compress the neck and the body as one beam between the saddle and the nut.",
      "Die vier Saiten sind in jeder Pose gestimmt (E 300, A 280, D 280, G 250 N, zusammen 1,11 kN): sie ziehen Sattel und Saitenhalter zum Steg und drücken den Steg mit etwa 0,67 kN bei 35 Grad Knickwinkel über dem Steg in die Decke; die drei Lasten sind im Gleichgewicht und drücken Hals und Korpus als einen Balken zwischen Untersattel und Sattel zusammen."),
    T("Played, the left hand presses the strings onto the fingerboard with 15 N, the bow presses 3 N on the strings and drags 1.5 N across them, reversing with the bow direction, and the player rests 0.30 kN of his weight on the stool seat; the solver treats the whole asset as one rigid body, so the cover and the stool share the reactions with the bass.",
          "Gespielt drückt die linke Hand die Saiten mit 15 N auf das Griffbrett, der Bogen drückt mit 3 N auf die Saiten und zieht sie mit 1,5 N quer, mit dem Bogenstrich wechselnd, und der Spieler stützt 0,30 kN seines Gewichts auf den Hockersitz; der Löser behandelt das ganze Asset als einen starren Körper, Hülle und Hocker teilen sich also die Reaktionen mit dem Bass."),
], "clinical": False}


def D(silhouette, retained, merged, omitted, thin, hatching, styles, overlays):
    """📝 The eight decisions of one representation, each a pair of English and German sentences."""
    keys = ["silhouette", "retained", "merged", "omitted", "thinElements", "hatching", "styles", "overlays"]
    return {k: T(*v) for k, v in zip(keys, [silhouette, retained, merged, omitted, thin, hatching, styles, overlays])}


DECISIONS_2D = {
    50: D(("Side elevation onto the top of the lying bass inside the open band of its cover, front elevation along the bass from the scroll end, plan from above onto the band, the front panel lying open on the floor and the stool.",
           "Seitenansicht auf die Decke des liegenden Basses im offenen Seitenband seiner Hülle, Vorderansicht längs des Basses vom Schneckenende, Draufsicht von oben auf das Band, die offen am Boden liegende Vorderseite und den Hocker."),
          ("Every part of the bass: viol-cornered outline, arched top, f-holes with eyes, bridge with heart and kidneys, tailpiece, saddle and tail wire, four strings, fingerboard and nut, neck with heel, pegbox, scroll and machine heads, endpin socket with screw, endpin, wheel clamp and twin wheels; cover back, band, sleeve, hinge tape, handle, straps and grip; the whole stool.",
           "Jedes Teil des Basses: Umriss mit Gambenecken, gewölbte Decke, F-Löcher mit Augen, Steg mit Herz und Nieren, Saitenhalter, Untersattel und Saitenhalterdraht, vier Saiten, Griffbrett und Sattel, Hals mit Fuß, Wirbelkasten, Schnecke und Mechaniken, Stachelbuchse mit Schraube, Stachel, Radklemme und Zwillingsrollen; Rücken, Band, Halsschlauch, Scharnierband, Griff, Riemen und Halsgriff der Hülle; der ganze Hocker."),
          ("The rollers inside the pegbox merge into its cheeks; the eight worm gears into the two machine-head plates.", "Die Wellen im Wirbelkasten gehen in seinen Wangen auf, die acht Schneckengetriebe in den zwei Mechanikplatten."),
          ("The sound post, bass bar and linings inside the closed body, the string windings inside the pegbox and the zip teeth.", "Stimmstock, Bassbalken und Reifchen im geschlossenen Korpus, die Saitenwicklungen im Wirbelkasten und die Reißverschlusszähne."),
          ("Strings at their true diameters of 1.6 to 2.6 mm, the tail wire 5.6 mm, the stool struts 8 mm and the footrest ring 14 mm are thin filled shapes along their solids.", "Die Saiten in ihren wahren Durchmessern von 1,6 bis 2,6 mm, der Saitenhalterdraht 5,6 mm, die Hockerstreben 8 mm und der Fußring 14 mm sind dünne gefüllte Formen längs ihrer Körper."),
          ("No hatches: nothing is cut.", "Keine Schraffuren: nichts ist geschnitten."),
          ("Body pen 0.35 mm, detail 0.18 mm for purfling, f-holes, crack and zip lines, texture 0.10 mm for rosin dust, worn varnish, tape markers, scuffs, the taped repair and the torn seat; tones from light spruce to dark ebony and black nylon.",
           "Körperstift 0,35 mm, Detail 0,18 mm für Einlage, F-Löcher, Riss und Reißverschlusslinien, Textur 0,10 mm für Kolophoniumstaub, abgegriffenen Lack, Klebebandmarken, Schrammen, die geklebte Reparatur und den eingerissenen Sitz; Töne von heller Fichte bis dunklem Ebenholz und schwarzem Nylon."),
          ("Thermal and structural views colour the same shapes by region and by member.", "Thermische und statische Ansichten färben dieselben Formen nach Region und Bauteil.")),
    100: D(("The same three views as at 1:50 with simplified outlines: the body outline from three points per interval, the bridge without its cut-outs, the head as one pegbox with scroll and two tuner blocks.",
            "Dieselben drei Ansichten wie bei 1:50 mit vereinfachten Umrissen: der Korpusumriss aus drei Punkten je Abschnitt, der Steg ohne Ausschnitte, der Kopf als ein Wirbelkasten mit Schnecke und zwei Mechanikblöcken."),
           ("Body, arched top, f-holes as slots, bridge, tailpiece, the four strings, fingerboard, neck, pegbox, scroll, tuner blocks, endpin with socket, clamp and wheels; the cover with handle, straps and grip; the stool with glides, struts and collar.",
            "Korpus, gewölbte Decke, F-Löcher als Schlitze, Steg, Saitenhalter, die vier Saiten, Griffbrett, Hals, Wirbelkasten, Schnecke, Mechanikblöcke, Stachel mit Buchse, Klemme und Rollen; die Hülle mit Griff, Riemen und Halsgriff; der Hocker mit Gleitern, Streben und Manschette."),
           ("Nut into the fingerboard, saddle and tail wire into the tailpiece and socket, screw and knob into their housings, keys into the tuner blocks, seat board and bracket into the seat.", "Sattel ins Griffbrett, Untersattel und Draht in Saitenhalter und Buchse, Schraube und Knauf in ihre Gehäuse, Flügel in die Mechanikblöcke, Sitzplatte und Halter in den Sitz."),
           ("Crack line, edge chip, tape markers, scuffs and the torn seat: below the 0.13 mm pen at this scale.", "Risslinie, Randabplatzung, Klebebandmarken, Schrammen und der eingerissene Sitz: unter dem 0,13-mm-Stift dieses Maßstabs."),
           ("Strings, endpin rod, struts and footrest ring stay at their true sections as thin filled shapes.", "Saiten, Stachelstange, Streben und Fußring behalten ihre wahren Querschnitte als dünne gefüllte Formen."),
           ("No hatches.", "Keine Schraffuren."),
           ("Body pen 0.25 mm, detail 0.13 mm for purfling, rosin dust, worn varnish and the f-holes; no texture pen.", "Körperstift 0,25 mm, Detail 0,13 mm für Einlage, Kolophoniumstaub, abgegriffenen Lack und die F-Löcher; kein Texturstift."),
           ("Thermal and structural views colour the same shapes.", "Thermische und statische Ansichten färben dieselben Formen.")),
    200: D(("A generalised body outline from every second point, the top as one plate, neck and fingerboard as one tapering bar, the head as one block with the scroll, the bridge as a 30 mm plate, the twin wheels as one roller.",
            "Ein verallgemeinerter Korpusumriss aus jedem zweiten Punkt, die Decke als eine Platte, Hals und Griffbrett als ein sich verjüngender Stab, der Kopf als ein Block mit der Schnecke, der Steg als 30-mm-Platte, die Zwillingsrollen als eine Walze."),
           ("Body, top, neck bar, head, scroll, bridge, tailpiece, socket, clamp, roller and endpin; the cover's back, band, sleeve and hinge tapes; the stool with four legs, glides, column, clamp, footrest ring and seat.",
            "Korpus, Decke, Halsstab, Kopf, Schnecke, Steg, Saitenhalter, Buchse, Klemme, Walze und Stachel; Rücken, Band, Halsschlauch und Scharnierbänder der Hülle; der Hocker mit vier Beinen, Gleitern, Säule, Klemme, Fußring und Sitz."),
           ("Strings into the bridge shape (they keep their slender thermal solids), fingerboard into the neck, machine heads into the head, the two wheels into one roller, handle, straps and grip into the cover.", "Saiten in die Stegform (sie behalten ihre schlanken thermischen Körper), Griffbrett in den Hals, Mechaniken in den Kopf, die zwei Rollen in eine Walze, Griff, Riemen und Halsgriff in die Hülle."),
           ("F-holes, purfling and every trace of use: below the printable feature at 1:200.", "F-Löcher, Einlage und jede Gebrauchsspur: unter dem druckbaren Merkmal bei 1:200."),
           ("The endpin rod, the stool struts and the footrest ring keep their true sections.", "Stachelstange, Hockerstreben und Fußring behalten ihre wahren Querschnitte."),
           ("No hatches.", "Keine Schraffuren."),
           ("Body pen 0.18 mm, detail 0.13 mm for the zip line of the band.", "Körperstift 0,18 mm, Detail 0,13 mm für die Reißverschlusslinie des Bandes."),
           ("Thermal and structural views colour the same shapes.", "Thermische und statische Ansichten färben dieselben Formen.")),
    500: D(("The bass as body with top plate, a 75 mm neck bar with head and a ridge for bridge, fingerboard and strings; the cover as back, band 77 mm wide, sleeve and front panel; the stool as base disc, column, ring and seat.",
            "Der Bass als Korpus mit Deckenplatte, ein 75-mm-Halsstab mit Kopf und ein Grat für Steg, Griffbrett und Saiten; die Hülle als Rücken, 77 mm breites Band, Halsschlauch und Vorderseite; der Hocker als Fußscheibe, Säule, Ring und Sitz."),
           ("A lying bass in an open cover beside a stool, readable at a glance at 3.6 mm of bass length.", "Ein liegender Bass in einer offenen Hülle neben einem Hocker, auf einen Blick lesbar bei 3,6 mm Basslänge."),
           ("Bridge, tailpiece, fingerboard and strings into one ridge, scroll and machine heads into the head, socket and clamp into the roller, the star base with glides and hub into one disc, inner column into the seat.", "Steg, Saitenhalter, Griffbrett und Saiten in einen Grat, Schnecke und Mechaniken in den Kopf, Buchse und Klemme in die Walze, Sternfuß mit Gleitern und Nabe in eine Scheibe, Innensäule in den Sitz."),
           ("Every detail line: there is no detail pen at 1:500.", "Jede Detaillinie: bei 1:500 gibt es keinen Detailstift."),
           ("The endpin rod, the struts and the footrest ring keep their true sections; the front panel and the back of the cover are declared thin sheets.", "Stachelstange, Streben und Fußring behalten ihre wahren Querschnitte; Vorderseite und Rücken der Hülle sind als dünne Platten deklariert."),
           ("No hatches.", "Keine Schraffuren."),
           ("Body pen 0.13 mm only.", "Nur Körperstift 0,13 mm."),
           ("Thermal and structural views colour the same shapes.", "Thermische und statische Ansichten färben dieselben Formen.")),
}

DECISIONS_3D = {
    50: [T("Body as a closed shell of ribs and flat back with the 10 degree back break over the upper bout, the arched top as its own solid of four arching rings; neck and heel as a loft of seven D sections, pegbox cheeks, back and floor, scroll, tuner plates and keys; bridge with heart and kidneys, tailpiece and fingerboard as sloped eight-corner solids; strings as slender capsules at their true diameters; f-holes, rosin dust and worn varnish as texture inlays flush with the top.",
           "Korpus als geschlossene Schale aus Zargen und flachem Boden mit dem 10-Grad-Bodenknick über dem Oberbug, die gewölbte Decke als eigener Körper aus vier Wölbungsringen; Hals und Fuß als Loft aus sieben D-Querschnitten, Wirbelkastenwangen, -rücken und -boden, Schnecke, Mechanikplatten und Flügel; Steg mit Herz und Nieren, Saitenhalter und Griffbrett als geneigte Achtecker; Saiten als schlanke Kapseln in wahren Durchmessern; F-Löcher, Kolophoniumstaub und abgegriffener Lack als bündige Texturauflagen."),
         T("Cover as back panel, two halves of the gusset band (endpin slot and neck opening between them), a loft of nine sections for the neck sleeve, two hinge tapes on the floor, handle, straps and grip; the unzipped front panel lies flat on its own hinged bone. Stool with four leg tubes, glides, hub, column, clamp, knob, footrest ring on four struts and collar, inner column, bracket, board and padded seat.",
           "Hülle als Rückenteil, zwei Hälften des Seitenbands (Stachelschlitz und Halsöffnung dazwischen), ein Loft aus neun Schnitten für den Halsschlauch, zwei Scharnierbänder am Boden, Griff, Riemen und Halsgriff; die aufgezippte Vorderseite liegt flach auf ihrem eigenen angelenkten Knochen. Hocker mit vier Beinrohren, Gleitern, Nabe, Säule, Klemme, Knauf, Fußring auf vier Streben mit Manschette, Innensäule, Halter, Platte und Polstersitz.")],
    100: [T("Body shell and a top of one arching ring, neck as a four-section loft, pegbox as one block with scroll and two tuner blocks, bridge without cut-outs, nut, saddle and tail wire merged, f-holes and traces of use left to the drawing; strings, endpin rod and stool struts keep their true sections as slender solids.",
            "Korpusschale und eine Decke aus einem Wölbungsring, Hals als Loft aus vier Schnitten, Wirbelkasten als ein Block mit Schnecke und zwei Mechanikblöcken, Steg ohne Ausschnitte, Sattel, Untersattel und Draht zusammengefasst, F-Löcher und Gebrauchsspuren der Zeichnung überlassen; Saiten, Stachelstange und Hockerstreben behalten als schlanke Körper ihre wahren Querschnitte."),
          T("Cover with back, band halves, a nine-section sleeve, hinge tapes, solid handle hump, straps and grip; stool without knob, bracket and separate seat board.", "Hülle mit Rücken, Bandhälften, einem Halsschlauch aus neun Schnitten, Scharnierbändern, vollem Griffbuckel, Riemen und Halsgriff; Hocker ohne Knauf, Halter und eigene Sitzplatte.")],
    200: [T("Body shell with back break, the top as a declared 12 mm plate, neck and fingerboard as one tapering 75 mm deep bar, head and machine heads as one block with the scroll, bridge thickened to 30 mm, tailpiece 30 mm, socket and clamp, the twin wheels as one roller; strings stay as slender thermal solids only.",
            "Korpusschale mit Bodenknick, die Decke als deklarierte 12-mm-Platte, Hals und Griffbrett als ein sich verjüngender 75 mm tiefer Stab, Kopf und Mechaniken als ein Block mit der Schnecke, Steg auf 30 mm verdickt, Saitenhalter 30 mm, Buchse und Klemme, die Zwillingsrollen als eine Walze; die Saiten bleiben nur als schlanke thermische Körper."),
          T("Cover band 8 mm thicker inwards so its 33 mm wall reaches the feature, back and front panel as declared sheets, the sleeve from five sections; stool legs thickened to 30 mm, glides 30 mm high, struts and footrest ring at their true sections.",
            "Seitenband der Hülle 8 mm nach innen verdickt, damit seine 33-mm-Wand das Merkmal erreicht, Rücken und Vorderseite als deklarierte Platten, der Halsschlauch aus fünf Schnitten; Hockerbeine auf 30 mm verdickt, Gleiter 30 mm hoch, Streben und Fußring in wahren Querschnitten.")],
    500: [T("Body as one slab without the back break (55 mm, under the 75 mm feature), the top as a declared plate, neck and head as 75 mm bars, bridge, tailpiece, fingerboard and strings as one ridge on the top (strings and ebony keep thermal solids), the twin wheels as one 75 mm roller; socket and clamp merged into it.",
            "Korpus als eine Platte ohne Bodenknick (55 mm, unter dem 75-mm-Merkmal), die Decke als deklarierte Platte, Hals und Kopf als 75-mm-Stäbe, Steg, Saitenhalter, Griffbrett und Saiten als ein Grat auf der Decke (Saiten und Ebenholz behalten thermische Körper), die Zwillingsrollen als eine 75-mm-Walze; Buchse und Klemme darin aufgegangen."),
          T("Cover band 52 mm thicker inwards (77 mm wall), back and front panel as sheets, the sleeve from four sections; stool star base with glides and hub as one disc 75 mm high, column 75 mm, struts and footrest ring true, seat 75 mm with the inner column merged.",
            "Seitenband der Hülle 52 mm nach innen verdickt (77-mm-Wand), Rücken und Vorderseite als Platten, der Halsschlauch aus vier Schnitten; Sternfuß des Hockers mit Gleitern und Nabe als eine 75 mm hohe Scheibe, Säule 75 mm, Streben und Fußring wahr, Sitz 75 mm mit eingeschlossener Innensäule.")],
}


def local_dir_to_world(frames, bone, d):
    """🧭 A bone-local direction in world coordinates."""
    o, m = frames[bone]
    return apply(m, d)


def unit(a, b):
    """➡ The unit vector from a to b."""
    d = [b[i] - a[i] for i in range(3)]
    L = math.sqrt(sum(x * x for x in d))
    return [x / L for x in d]


TENSION_N = 1110.0


def string_loads(frames):
    """🎼 The self-equilibrated string loads of a pose in world newtons: nut and tailpiece pulled towards the bridge, the bridge pressed into the top."""
    (u0, v0, w0, u1, v1, w1, _), (_, _, _, u2, v2, w2, _) = string_span("d", 50)
    tail, bridge, nut = (u0, 0.0, w0), (u1, 0.0, w1), (u2, 0.0, w2)
    to_nut, to_tail = unit(bridge, nut), unit(bridge, tail)
    f_nut = [-TENSION_N * x for x in to_nut]
    f_tail = [-TENSION_N * x for x in to_tail]
    f_bridge = [-(f_nut[i] + f_tail[i]) for i in range(3)]
    out = []
    for lid, anchor, f, en, de in (("strings-at-nut", "string-nut", f_nut, "Four strings pulling the nut towards the bridge, 1.11 kN", "Vier Saiten ziehen den Sattel zum Steg, 1,11 kN"),
                                    ("strings-at-tailpiece", "string-tailpiece", f_tail, "Four strings pulling the tailpiece towards the bridge, 1.11 kN", "Vier Saiten ziehen den Saitenhalter zum Steg, 1,11 kN"),
                                    ("strings-on-bridge", "bridge-crown", f_bridge, "Down-bearing of the strings on the bridge, 0.67 kN", "Druck der Saiten auf den Steg, 0,67 kN")):
        wv = local_dir_to_world(frames, "bass", f)
        out.append({"id": lid, "kind": "imposed", "title": T(en, de), "anchor": anchor, "fxN": r6(wv[0]), "fyN": r6(wv[1]), "fzN": r6(wv[2]), "source": "authored-estimate", "reference": "string-tension"})
    return out


def played_loads(frames):
    """🎻 The playing loads: left hand on the strings, bow pressure and bow friction, the player half-sitting on the stool."""
    press = local_dir_to_world(frames, "bass", (0, 0, 1))
    across = local_dir_to_world(frames, "bass", (0, 1, 0))
    out = string_loads(frames)
    out.append({"id": "left-hand", "kind": "live", "title": T("Left hand stopping the strings on the fingerboard, 15 N", "Linke Hand greift die Saiten aufs Griffbrett, 15 N"), "anchor": "neck-hand", "fxN": r6(15 * press[0]), "fyN": r6(15 * press[1]), "fzN": r6(15 * press[2]), "source": "authored-estimate", "reference": "bass-making"})
    out.append({"id": "bow-pressure", "kind": "live", "title": T("Bow pressing on the strings, 3 N", "Bogen drückt auf die Saiten, 3 N"), "anchor": "bow-point", "fxN": r6(3 * press[0]), "fyN": r6(3 * press[1]), "fzN": r6(3 * press[2]), "source": "authored-estimate", "reference": "bass-making"})
    out.append({"id": "bow-friction", "kind": "live", "title": T("Bow dragging the strings across, 1.5 N (reverses with the stroke)", "Bogen zieht die Saiten quer, 1,5 N (wechselt mit dem Strich)"), "anchor": "bow-point", "fxN": r6(1.5 * across[0]), "fyN": r6(1.5 * across[1]), "fzN": r6(1.5 * across[2]), "source": "authored-estimate", "reference": "bass-making"})
    out.append({"id": "player-on-stool", "kind": "occupant", "title": T("Player half-sitting on the stool, 0.30 kN", "Spieler sitzt halb auf dem Hocker, 0,30 kN"), "anchor": "seat-top", "fxN": 0, "fyN": -300, "source": "authored-estimate", "reference": "bass-accessories"})
    return out


def binding(regions, mode):
    """🌬 Region binding of a pose: indoors without sun; floor contacts stored and rolled, hand, body and seat contacts played."""
    out = {}
    for r in regions:
        b = {"exposedFraction": 1, "sunlitFraction": 0}
        if r == "cover" and mode in ("stored", "played", "zipped"):
            b = {"exposedFraction": 0.97, "sunlitFraction": 0, "contact": {"fraction": 0.03, "tempC": 19}}
        if r == "cover" and mode == "carried":
            b = {"exposedFraction": 0.98, "sunlitFraction": 0, "contact": {"fraction": 0.02, "tempC": 33}}
        if r == "stool-frame":
            b = {"exposedFraction": 0.99, "sunlitFraction": 0, "contact": {"fraction": 0.01, "tempC": 19}}
        if r == "wheels" and mode in ("carried", "played"):
            b = {"exposedFraction": 0.98, "sunlitFraction": 0, "contact": {"fraction": 0.02, "tempC": 19}}
        if mode == "played" and r == "neck-scroll":
            b = {"exposedFraction": 0.94, "sunlitFraction": 0, "contact": {"fraction": 0.06, "tempC": 33}}
        if mode == "played" and r == "strings":
            b = {"exposedFraction": 0.97, "sunlitFraction": 0, "contact": {"fraction": 0.03, "tempC": 33}}
        if mode == "played" and r == "back-ribs":
            b = {"exposedFraction": 0.98, "sunlitFraction": 0, "contact": {"fraction": 0.02, "tempC": 32}}
        if mode == "played" and r == "stool-seat":
            b = {"exposedFraction": 0.55, "sunlitFraction": 0, "contact": {"fraction": 0.45, "tempC": 33}}
        out[r] = b
    return out


def anchors_of(carry_frames):
    """📍 Floor contacts, the bass resting in the band, the endpin tip, the wheel on the floor when rolled, the grip, the player's body and the load points of strings, hands, bow and seat."""
    (a1, b1), (a2, b2) = hull_contacts()
    fp = flap_profile(50)
    out = [
        {"id": "cover-floor-lower-bout", "bone": "cover", "x": r6(a1), "y": r6(b1), "z": -0.09, "kind": "contact", "support": "contact", "footprintLengthM": 0.12, "footprintWidthM": 0.42, "frictionCoefficient": 0.6, "title": T("Cover on the floor under the lower bout", "Hülle unter dem Unterbug am Boden")},
        {"id": "cover-floor-upper-bout", "bone": "cover", "x": r6(a2), "y": r6(b2), "z": -0.09, "kind": "contact", "support": "contact", "footprintLengthM": 0.10, "footprintWidthM": 0.42, "frictionCoefficient": 0.6, "title": T("Cover on the floor under the upper bout", "Hülle unter dem Oberbug am Boden")},
        {"id": "flap-floor", "bone": "cover-flap", "x": r6(sum(p[0] for p in fp) / len(fp)), "y": r6(sum(p[1] for p in fp) / len(fp)), "z": 0, "kind": "contact", "support": "contact", "footprintLengthM": 1.0, "footprintWidthM": 0.6, "frictionCoefficient": 0.6, "title": T("Open front panel lying on the floor", "Offene Vorderseite am Boden")},
        {"id": "bass-rest-lower-bout", "bone": "bass", "x": 0.27, "y": -0.340, "z": 0, "kind": "support", "support": "contact", "title": T("Bass-side rib of the lower bout on the foam of the band", "Bass-Zarge des Unterbugs auf dem Schaum des Bandes")},
        {"id": "bass-rest-upper-bout", "bone": "bass", "x": 0.88, "y": -0.255, "z": 0, "kind": "support", "support": "contact", "title": T("Bass-side rib of the upper bout on the foam of the band", "Bass-Zarge des Oberbugs auf dem Schaum des Bandes")},
        {"id": "endpin-tip", "bone": "endpin", "x": r6(ROD_L + TIP_R), "y": 0, "z": 0, "kind": "contact", "support": "contact", "footprintLengthM": 0.012, "footprintWidthM": 0.012, "frictionCoefficient": 0.8, "title": T("Rubber stopper of the endpin on the floor", "Gummistopper des Stachels am Boden")},
    ]
    axle = to_world(carry_frames, "bass", (AXLE_U, 0, 0))
    o, m = carry_frames["bass"]
    p = (axle[0] - o[0], axle[1] - WHEEL_R - o[1], axle[2] - o[2])
    loc = tuple(sum(m[k][i] * p[k] for k in range(3)) for i in range(3))
    out.append({"id": "wheel-floor", "bone": "bass", "x": r6(loc[0]), "y": r6(loc[1]), "z": r6(loc[2]), "kind": "contact", "support": "roller", "footprintLengthM": 0.01, "footprintWidthM": 0.044, "frictionCoefficient": 0.02, "title": T("Twin wheels on the floor while the bass is rolled", "Zwillingsrollen am Boden, während der Bass gerollt wird")})
    out.append({"id": "carry-grip", "bone": "cover", "x": 1.37, "y": 0.105, "z": SLEEVE_WC, "kind": "support", "support": "fixed", "title": T("Hand on the grip of the neck sleeve", "Hand am Griff des Halsschlauchs")})
    out.append({"id": "player-body", "bone": "bass", "x": 0.95, "y": 0.0, "z": r6(w_back(0.95)), "kind": "support", "support": "roller", "normalDeg": r6(LEAN), "title": T("Player's body against the back at the upper bout", "Körper des Spielers am Boden des Oberbugs")})
    (u0, v0, w0, u1, v1, w1, _), (_, _, _, u2, v2, w2, _) = string_span("d", 50)
    out.append({"id": "string-nut", "bone": "bass", "x": r6(u2), "y": 0, "z": r6(w2), "kind": "load", "title": T("Strings on the nut", "Saiten auf dem Sattel")})
    out.append({"id": "string-tailpiece", "bone": "bass", "x": r6(u0), "y": 0, "z": r6(w0), "kind": "load", "title": T("String ends in the tailpiece", "Saitenenden im Saitenhalter")})
    out.append({"id": "bridge-crown", "bone": "bass", "x": BRIDGE_U, "y": 0, "z": r6(w1), "kind": "load", "title": T("Strings over the bridge crown", "Saiten über der Stegkrone")})
    out.append({"id": "neck-hand", "bone": "bass", "x": 1.30, "y": 0, "z": r6(w_ft(1.30) - 0.004), "kind": "load", "title": T("Left hand stopping the strings in thumb position", "Linke Hand greift die Saiten in Daumenlage")})
    bu = 0.555
    out.append({"id": "bow-point", "bone": "bass", "x": bu, "y": 0, "z": r6(w1 + (w2 - w1) * (bu - u1) / (u2 - u1) - 0.002), "kind": "load", "title": T("Bow hair on the strings between fingerboard and bridge", "Bogenhaare auf den Saiten zwischen Griffbrett und Steg")})
    out.append({"id": "seat-top", "bone": "stool-seat", "x": 0, "y": 0.060, "z": 0, "kind": "load", "title": T("Middle of the stool seat", "Mitte des Hockersitzes")})
    for k, az in enumerate(STOOL_LEG_AZ):
        ca, sa = math.cos(math.radians(az)), math.sin(math.radians(az))
        out.append({"id": f"stool-foot-{k}", "bone": "stool", "x": r6(0.262 * ca), "y": 0, "z": r6(0.262 * sa), "kind": "contact", "support": "contact", "footprintLengthM": 0.032, "footprintWidthM": 0.032, "frictionCoefficient": 0.5, "title": T(f"Stool glide {k + 1} on the floor", f"Hockergleiter {k + 1} am Boden")})
    return out


def document():
    """📄 The whole asset document."""
    bones = rig_bones()
    regions = [r["id"] for r in THERMAL["regions"]]
    cf = solve_rig(bones, carried_offsets())
    pf = solve_rig(bones, played_offsets())
    rf = solve_rig(bones, {})
    zf = solve_rig(bones, {"cover-flap": {"twistDeg": -90.0}})
    anchors = anchors_of(cf)
    stool = [f"stool-foot-{k}" for k in range(4)]
    floor = ["cover-floor-lower-bout", "cover-floor-upper-bout"]
    rest_bass = ["bass-rest-lower-bout", "bass-rest-upper-bout"]
    poses = [
        {"id": "stored", "title": T("Stored in the open cover", "In der offenen Hülle gelagert"), "description": T(
            "The bass lies on its bass-side rib in its padded cover on the dealer's floor, the front panel unzipped and folded down onto the floor so the instrument can be seen; the bass wheel stays clamped on the retracted endpin and the stool stands beside the endpin end.",
            "Der Bass liegt auf der Bass-Zarge in seiner Polsterhülle auf dem Boden des Händlers, die Vorderseite aufgezippt und auf den Boden geklappt, damit man das Instrument sieht; das Kontrabassrad bleibt am eingeschobenen Stachel festgeklemmt, der Hocker steht neben dem Stachelende."),
         "bones": {}, "ground": "authored", "contacts": floor + ["flap-floor"] + rest_bass + stool, "thermal": {"regions": binding(regions, "stored"), "emitters": {}}, "loads": string_loads(rf)},
        {"id": "zipped", "title": T("Zipped in the cover", "In der Hülle zugezippt"), "description": T(
            "The front panel is folded up and zipped shut over the strings; the bass lies closed in its cover, ready to be rolled.",
            "Die Vorderseite ist hochgeklappt und über den Saiten zugezippt; der Bass liegt geschlossen in seiner Hülle, bereit zum Rollen."),
         "bones": {"cover-flap": {"twistDeg": -90.0}}, "ground": "authored", "contacts": floor + rest_bass + stool, "thermal": {"regions": binding(regions, "zipped"), "emitters": {}}, "loads": string_loads(zf)},
        {"id": "carried", "title": T("Rolled on the bass wheel", "Auf dem Kontrabassrad gerollt"), "description": T(
            f"Zipped in its cover, the bass is tipped up {CARRY_DEG:.0f} degrees onto the twin wheels clamped on the endpin and rolled along by one hand on the grip of the neck sleeve at shoulder height; the stool stays behind.",
            f"In der Hülle zugezippt ist der Bass um {CARRY_DEG:.0f} Grad auf die am Stachel festgeklemmten Zwillingsrollen gekippt und wird von einer Hand am Griff des Halsschlauchs in Schulterhöhe gerollt; der Hocker bleibt stehen."),
         "bones": carried_offsets(), "ground": "authored", "contacts": ["wheel-floor", "carry-grip"] + stool, "thermal": {"regions": binding(regions, "carried"), "emitters": {}}, "loads": string_loads(cf)},
        {"id": "played", "title": T("Played standing at the stool", "Stehend am Hocker gespielt"), "description": T(
            f"Taken out of the cover, the bass stands on its endpin pulled out {PLAYED_E * 1000:.0f} mm, its top turned to the front and leaning {LEAN:.0f} degrees back against the player, who half-sits on the stool; the left hand stops the strings, the bow plays between fingerboard and bridge, the open cover stays on the floor.",
            f"Aus der Hülle genommen steht der Bass auf dem um {PLAYED_E * 1000:.0f} mm ausgezogenen Stachel, die Decke nach vorn gedreht und um {LEAN:.0f} Grad zurück an den Spieler gelehnt, der halb auf dem Hocker sitzt; die linke Hand greift die Saiten, der Bogen streicht zwischen Griffbrett und Steg, die offene Hülle bleibt am Boden."),
         "bones": played_offsets(), "ground": "authored", "contacts": ["endpin-tip", "player-body"] + floor + ["flap-floor"] + stool, "thermal": {"regions": binding(regions, "played"), "emitters": {}}, "loads": played_loads(pf)},
    ]
    rock_fwd, rock_back = played_offsets(-1.0), played_offsets(0.8)
    clips = [
        {"id": "carry", "title": T("Zip up and roll away", "Zuzippen und wegrollen"), "description": T(
            "The front panel is folded up and zipped in two seconds; after a pause the bass in its cover is tipped up onto the bass wheel in three seconds and held ready to roll.",
            "Die Vorderseite wird in zwei Sekunden hochgeklappt und zugezippt; nach einer Pause wird der Bass in seiner Hülle in drei Sekunden auf das Kontrabassrad gekippt und rollbereit gehalten."),
         "durationS": 7, "fps": 25, "loop": False, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "stored", "ease": "smooth"}, {"t": 2, "pose": "zipped", "ease": "smooth"}, {"t": 2.5, "pose": "zipped", "ease": "smooth"}, {"t": 5.5, "pose": "carried", "ease": "smooth"}, {"t": 7, "pose": "carried", "ease": "smooth"}],
         "contacts": [{"anchor": "flap-floor", "fromS": 0, "toS": 0.3}] + [{"anchor": a, "fromS": 0, "toS": 2.7} for a in floor + rest_bass] + [{"anchor": "carry-grip", "fromS": 2.5, "toS": 7}, {"anchor": "wheel-floor", "fromS": 5.3, "toS": 7}] + [{"anchor": a, "fromS": 0, "toS": 7} for a in stool],
         "phases": [{"id": "zip-up", "fromS": 0, "toS": 2, "title": T("Folding up and zipping the front panel", "Vorderseite hochklappen und zuzippen")}, {"id": "grip", "fromS": 2, "toS": 2.5, "title": T("Taking the grip", "Griff fassen")},
                    {"id": "tip-up", "fromS": 2.5, "toS": 5.5, "title": T("Tipping up onto the wheel", "Auf das Rad kippen")}, {"id": "ready", "fromS": 5.5, "toS": 7, "title": T("Ready to roll", "Rollbereit")}],
         "events": [{"t": 0.3, "kind": "contact-off", "title": T("Front panel leaves the floor", "Vorderseite verlässt den Boden"), "target": "cover-flap"}, {"t": 2, "kind": "motion", "title": T("Zip closed", "Reißverschluss zu"), "target": "cover-flap"},
                    {"t": 2.7, "kind": "contact-off", "title": T("Cover leaves the floor", "Hülle verlässt den Boden"), "target": "cover"}, {"t": 5.3, "kind": "contact-on", "title": T("Wheels touch down", "Rollen setzen auf"), "target": "wheel"}],
         "tracks": []},
        {"id": "bowing", "title": T("Bowing a long note", "Einen langen Ton streichen"), "description": T(
            "A slow down-bow and up-bow of two seconds each on the D string: the bass rocks one degree towards the player on the down-bow and back on the up-bow about its endpin, the bow friction reverses with the stroke.",
            "Ein langsamer Ab- und Aufstrich von je zwei Sekunden auf der D-Saite: der Bass wiegt sich beim Abstrich um ein Grad zum Spieler und beim Aufstrich zurück um seinen Stachel, die Bogenreibung wechselt mit dem Strich."),
         "durationS": 4, "fps": 25, "loop": True, "rootMotion": "none",
         "keys": [{"t": 0, "pose": "played", "ease": "smooth"}, {"t": 1, "pose": "played", "ease": "smooth", "bones": {k: rock_back[k] for k in ("bass-tilt", "bass-slide", "bass-lift")}},
                  {"t": 2, "pose": "played", "ease": "smooth"}, {"t": 3, "pose": "played", "ease": "smooth", "bones": {k: rock_fwd[k] for k in ("bass-tilt", "bass-slide", "bass-lift")}}],
         "contacts": [{"anchor": a, "fromS": 0, "toS": 4} for a in ["endpin-tip", "player-body"] + floor + ["flap-floor"] + stool],
         "phases": [{"id": "down-bow", "fromS": 0, "toS": 2, "title": T("Down-bow", "Abstrich")}, {"id": "up-bow", "fromS": 2, "toS": 4, "title": T("Up-bow", "Aufstrich")}],
         "events": [{"t": 2, "kind": "motion", "title": T("Bow change", "Bogenwechsel"), "target": "bass"}],
         "tracks": [{"target": "load:bow-friction", "ease": "step", "keys": [{"t": 0, "v": 1}, {"t": 2, "v": -1}, {"t": 4, "v": 1}]}]},
    ]
    asset = {
        "id": SLUG, "category": "objects", "title": T("Used Double Bass", "Gebrauchter Kontrabass"), "summary": SUMMARY,
        "tags": ["used", "instrument", "music", "second-hand", "strings", "double-bass", "bowed", "orchestral", "spruce", "maple", "ebony", "gig-bag", "bass-wheel", "stool", "passive"],
        "provenance": PROVENANCE, "dimensions": DIMENSIONS, "materials": MATERIALS, "rig": {"kind": "mechanical", "root": "floor", "bones": bones},
        "volumes": {str(s): volume(s) for s in SCALES}, "anchors": anchors,
        "representations": {str(s): representation(s) for s in SCALES},
        "symbols": [], "poses": poses, "clips": clips, "thermal": THERMAL, "structure": STRUCTURE,
        "presentation": {"defaultPose": "stored", "defaultClip": "carry", "defaultScale": 50},
        "exports": ["svg", "pdf", "dxf", "dwg", "png-sequence", "mp4", "webm", "component", "glb", "fbx", "model", "model-zip"],
        "completion": "authored",
    }
    return {"$schema": "../../../🧬️schema/🔣️.json#/$defs/AssetDocument", "version": "semio.staffage/v2", "asset": asset}
