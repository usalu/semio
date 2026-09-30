#!/usr/bin/env python3
"""🏷️ W2-R norm: writes the `x-semio-ui` input annotations (design §6, manifest `$defs/InputUi`) onto every norm leaf payload
schema (`✏️s/🔌️plugins/📕️norm/**/🧬️mutations/<leaf>/🧬️schema/🔣️.json`) — every top-level input and every nested record field
the framework glossary does not label — with the official German terminology of the respective standard and SI units.

Stored units follow the norm snapshots (`#[dsl(unit)]`, SI base: m, Pa, N, N·m, K, s, J/m²); `displayUnit`/`displayFactor`
give the engineering display unit (display = stored × factor). Hard bounds are added only where the standard or the Rust
diff makes them certain. Idempotent: existing annotations are replaced, payload structure is never touched.

    python3 🧪️w2-r-norm-annotate-inputs.py [--check] [--baseline <leaf-schemas-before.json>]

`--baseline` rebuilds every leaf from its pre-rollout text (a `{path: text}` snapshot), so a hard bound dropped from a table
row is dropped from the schema too; a leaf that changed beyond annotations and bounds since the snapshot is refused.
"""
import glob
import json
import os
import sys

REPO = "/Users/ueli/Documents/semio"
SCOPE = "✏️s/🔌️plugins/📕️norm"
GLOSSARY = "🧰️framework/🔨️modules/🛂️manifest/🔣️input-labels.json"
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")
BOUNDS = "__bounds__"

#region helpers


def L(en, de):
    return {"en": en, "de": de}


def ui(widget, label, role="value", desc=None, group=None, bounds=None, **extra):
    body = {"widget": widget, "role": role, "label": label, "description": desc, "group": group, **extra}
    out = {key: body[key] for key in KEY_ORDER if body.get(key) is not None}
    if bounds:
        out[BOUNDS] = bounds
    return out


DISPLAY = {
    "kN": ("kN", 0.001),
    "kN·m": ("kN·m", 0.001),
    "kN/m": ("kN/m", 0.001),
    "kN/m²": ("kN/m²", 0.001),
    "kN/m³": ("kN/m³", 0.001),
    "N/mm²": ("N/mm²", 1e-6),
    "mm": ("mm", 1000),
    "cm²": ("cm²", 10000),
    "cm²/m": ("cm²/m", 10000),
    "min": ("min", 1 / 60),
    "MJ/m²": ("MJ/m²", 1e-6),
    "%": ("%", 100),
    "kW/m²": ("kW/m²", 0.001),
}


def q(en, de, unit, group, disp=None, step=None, prec=None, desc=None, lo=None, xlo=None, hi=None, widget="stepper", soft=None, snaps=None):
    bounds = {key: value for key, value in (("minimum", lo), ("exclusiveMinimum", xlo), ("maximum", hi)) if value is not None}
    display, factor = DISPLAY[disp] if disp else (None, None)
    return ui(widget, L(en, de), desc=desc, group=group, bounds=bounds, unit=unit, displayUnit=display, displayFactor=factor, step=step, precision=prec, softMin=soft[0] if soft else None, softMax=soft[1] if soft else None, snaps=snaps)


def ratio(en, de, group, desc=None, step=0.01):
    return q(en, de, "1", group, disp="%", step=step, prec=0, desc=desc, lo=0, hi=1, widget="slider")


def count(en, de, group, desc=None, lo=0, hi=None, unit=None):
    return q(en, de, unit, group, step=1, prec=0, desc=desc, lo=lo, hi=hi)


def factor(en, de, group, desc=None, step=0.01, prec=2, lo=None, xlo=None, hi=None):
    return q(en, de, None, group, step=step, prec=prec, desc=desc, lo=lo, xlo=xlo, hi=hi)


def ref(kind, en, de, desc=None, role="target", group="target"):
    return ui("reference", L(en, de), role=role, desc=desc, group=group, ref={"kind": kind})


def idx(en, de, desc, group="target"):
    return ui("stepper", L(en, de), desc=desc, group=group, bounds={"minimum": 0}, step=1, precision=0)


def txt(en, de, group, desc=None, widget="text"):
    return ui(widget, L(en, de), desc=desc, group=group)


def tog(en, de, group, desc=None):
    return ui("toggle", L(en, de), desc=desc, group=group)


def sel(en, de, group, options, desc=None, widget="select"):
    return ui(widget, L(en, de), desc=desc, group=group, options={value: L(*labels) for value, labels in options.items()})


def rec(en, de, desc=None, group="record"):
    return ui(None, L(en, de), desc=desc, group=group)


def D(en, de):
    return L(en, de)


INSERT_AT = D("Zero-based list position at which the record is inserted.", "Nullbasierte Listenposition, an der der Datensatz eingefügt wird.")


def at_insert(group="target"):
    return idx("Position", "Position", INSERT_AT, group)


def at_entry(en, de, what_en, what_de):
    return idx(en, de, D(f"Zero-based position of the {what_en} in its list.", f"Nullbasierte Position {what_de} in der Liste."))


ANNEX_TEXT = txt("National annex", "Nationaler Anhang", "annex", D("De applies the German national annex (DIN EN), En the recommended CEN values.", "De wendet den deutschen Nationalen Anhang (DIN EN) an, En die empfohlenen CEN-Werte."))
ANNEX_OPTIONS = {"En": ("Recommended CEN values (EN)", "Empfohlene CEN-Werte (EN)"), "De": ("German national annex (DIN EN)", "Deutscher Nationaler Anhang (DIN EN)")}
ASSUMED = D("Value claimed by the design; the check compares it with the value derived from the standard.", "Vom Entwurf angesetzter Wert; der Nachweis vergleicht ihn mit dem nach der Norm ermittelten Wert.")

#endregion helpers

#region din16798

ZONE16798 = ref("zone", "Zone", "Zone", D("The ventilation zone this mutation addresses.", "Die Lüftungszone, die diese Mutation adressiert."))
VENT16798 = ref("ventSystem", "Ventilation system", "Lüftungsanlage", D("The ventilation system this mutation addresses.", "Die Lüftungsanlage, die diese Mutation adressiert."))
DIN16798 = {
    "change-zone-t-op-summer": {"zoneId": ZONE16798, "newTOpSummerC": q("Operative temperature in summer θ_o", "Operative Temperatur im Sommer θ_o", "°C", "comfort", step=0.5, prec=1)},
    "change-vent-heat-recovery": {"ventId": VENT16798, "newHeatRecoveryEta": ratio("Heat recovery temperature ratio η_t", "Temperaturänderungsgrad der Wärmerückgewinnung η_t", "ventilation")},
    "change-vent-system-type": {"ventId": VENT16798, "newSystemType": txt("Ventilation system type", "Art der Lüftungsanlage", "ventilation", D("central_mech (central mechanical), decentral_mech (decentral mechanical) or natural.", "central_mech (zentral mechanisch), decentral_mech (dezentral mechanisch) oder natural (freie Lüftung)."))},
    "change-zone-t-op-winter": {"zoneId": ZONE16798, "newTOpWinterC": q("Operative temperature in winter θ_o", "Operative Temperatur im Winter θ_o", "°C", "comfort", step=0.5, prec=1)},
    "insert-zone": {"index": at_insert(), "zone": rec("Zone", "Zone", D("The complete ventilation zone record to insert.", "Der vollständige einzufügende Lüftungszonen-Datensatz."))},
    "remove-zone": {"zoneId": ZONE16798},
    "insert-vent-system": {"index": at_insert(), "vent": rec("Ventilation system", "Lüftungsanlage", D("The complete ventilation system record to insert.", "Der vollständige einzufügende Datensatz der Lüftungsanlage."))},
    "change-cellar-ventilation": {"newCellarVentilationM3H": q("Cellar airflow", "Kellerluftvolumenstrom", "m³/h", "building", step=1, prec=1, lo=0)},
    "change-vent-sfp": {"ventId": VENT16798, "newSfpWM3S": q("Specific fan power SFP", "Spezifische Ventilatorleistung SFP", "W/(m³/s)", "ventilation", step=50, prec=0, lo=0)},
    "change-annex": {"newAnnex": ANNEX_TEXT},
    "change-night-setback": {"newNightSetbackK": q("Night setback Δθ", "Nachtabsenkung Δθ", "K", "building", step=0.5, prec=1, lo=0)},
    "change-outdoor-co2": {"newOutdoorCo2Ppm": q("Outdoor CO₂ concentration", "CO₂-Konzentration der Außenluft", "ppm", "air-quality", step=10, prec=0, lo=0)},
    "change-vent-design-airflow": {"ventId": VENT16798, "newDesignAirflowM3H": q("Design outdoor airflow q_V", "Auslegungs-Außenluftvolumenstrom q_V", "m³/h", "ventilation", step=10, prec=0, lo=0)},
    "change-vent-sfp-class": {"ventId": VENT16798, "newSfpRequiredClass": count("Required SFP class", "Geforderte SFP-Klasse", "ventilation", D("EN 16798-3 class SFP 0 (≤ 300 W/(m³/s)) to SFP 7 (≤ 6500 W/(m³/s)).", "Klasse nach EN 16798-3 von SFP 0 (≤ 300 W/(m³/s)) bis SFP 7 (≤ 6500 W/(m³/s))."), hi=7)},
    "change-zone-metabolic-rate": {"zoneId": ZONE16798, "newMetabolicRateMet": q("Metabolic rate M", "Energieumsatz M", "met", "comfort", step=0.1, prec=1, lo=0)},
    "change-cellar-area": {"newCellarAreaM2": q("Cellar floor area", "Kellergrundfläche", "m²", "building", step=1, prec=1, lo=0)},
    "change-vent-oda-class": {"ventId": VENT16798, "newOdaClass": txt("Outdoor air class ODA", "Außenluftklasse ODA", "air-quality", D("ODA1 (clean) to ODA4 (extremely high concentrations).", "ODA1 (rein) bis ODA4 (extrem hohe Konzentrationen)."))},
    "change-envelope-n50": {"newEnvelopeN50HInv": q("Air change rate at 50 Pa n₅₀", "Luftwechselrate bei 50 Pa n₅₀", "1/h", "building", step=0.1, prec=2, lo=0)},
    "change-zone-usage-type": {"zoneId": ZONE16798, "newUsageType": txt("Usage type", "Nutzungsart", "comfort", D("office, residential, classroom, meeting or retail.", "office (Büro), residential (Wohnen), classroom (Unterrichtsraum), meeting (Besprechung) oder retail (Handel)."))},
    "change-zone-pollution-class": {"zoneId": ZONE16798, "newPollutionClass": txt("Building pollution class", "Emissionsklasse des Gebäudes", "air-quality", D("very_low, low or non_low (EN 16798-1 Annex B).", "very_low (sehr schadstoffarm), low (schadstoffarm) oder non_low (nicht schadstoffarm) nach EN 16798-1 Anhang B."))},
    "change-zone-clothing": {"zoneId": ZONE16798, "newClothingClo": q("Clothing insulation I_cl", "Wärmeisolation der Bekleidung I_cl", "clo", "comfort", step=0.1, prec=1, lo=0)},
    "change-zone-occupants": {"zoneId": ZONE16798, "newOccupants": count("Number of occupants", "Personenanzahl", "comfort")},
    "change-zone-illuminance": {"zoneId": ZONE16798, "newIlluminanceLx": q("Maintained illuminance Ē_m", "Wartungswert der Beleuchtungsstärke Ē_m", "lx", "lighting", step=50, prec=0, lo=0)},
    "change-zone-rh": {"zoneId": ZONE16798, "newRhPercent": q("Relative humidity φ", "Relative Luftfeuchte φ", "%", "comfort", step=1, prec=0, lo=0, hi=100, widget="slider")},
    "change-zone-air-speed": {"zoneId": ZONE16798, "newAirSpeedMS": q("Air speed v_a", "Luftgeschwindigkeit v_a", "m/s", "comfort", step=0.05, prec=2, lo=0)},
    "change-zone-turbulence": {"zoneId": ZONE16798, "newTurbulenceIntensityPercent": q("Turbulence intensity Tu", "Turbulenzgrad Tu", "%", "comfort", step=1, prec=0, lo=0, hi=100, widget="slider")},
    "change-zone-outdoor-air": {"zoneId": ZONE16798, "newOutdoorAirSuppliedM3H": q("Supplied outdoor airflow", "Zugeführter Außenluftvolumenstrom", "m³/h", "air-quality", step=10, prec=0, lo=0)},
    "change-vent-inspection": {"ventId": VENT16798, "newYearsSinceInspection": count("Years since last inspection", "Jahre seit der letzten Inspektion", "ventilation", unit="a")},
    "change-zone-floor-area": {"zoneId": ZONE16798, "newFloorAreaM2": q("Floor area A", "Grundfläche A", "m²", "comfort", step=1, prec=1, lo=0)},
    "change-zone-vent-method": {"zoneId": ZONE16798, "newVentMethod": txt("Ventilation design method", "Verfahren zur Lüftungsauslegung", "air-quality", D("method_1_perceived_air_quality, method_2_limit_concentration or method_3_predefined_rates (EN 16798-1 §6.3).", "method_1_perceived_air_quality (empfundene Luftqualität), method_2_limit_concentration (Grenzkonzentration) oder method_3_predefined_rates (vorgegebene Außenluftraten) nach EN 16798-1 §6.3."))},
    "change-envelope-volume": {"newEnvelopeVolumeM3": q("Internal air volume V", "Innenluftvolumen V", "m³", "building", step=1, prec=1, lo=0)},
    "change-theta-rm": {"newThetaRmC": q("Running mean outdoor temperature θ_rm", "Gleitender Mittelwert der Außenlufttemperatur θ_rm", "°C", "comfort", step=0.5, prec=1)},
    "change-zone-noise": {"zoneId": ZONE16798, "newNoiseDb": q("Sound pressure level L_p", "Schalldruckpegel L_p", "dB", "acoustics", step=1, prec=0)},
    "change-zone-vent-system-id": {"zoneId": ZONE16798, "newVentSystemId": ref("ventSystem", "Serving ventilation system", "Versorgende Lüftungsanlage", D("The ventilation system that supplies the zone.", "Die Lüftungsanlage, die die Zone versorgt."), role="value", group="ventilation")},
    "change-vent-duct-leakage": {"ventId": VENT16798, "newDuctLeakageM3SM2": q("Duct air leakage rate f", "Leckluftrate der Luftleitungen f", "m³/(s·m²)", "ventilation", step=0.0001, prec=5, lo=0)},
    "remove-vent-system": {"ventId": VENT16798},
    "change-zone-comfort-category": {"zoneId": ZONE16798, "newComfortCategory": txt("Indoor environment category", "Kategorie des Innenraumklimas", "comfort", D("I (high), II (normal), III (moderate) or IV (low expectation), EN 16798-1.", "I (hohes), II (normales), III (mäßiges) oder IV (geringes Maß an Erwartungen) nach EN 16798-1."))},
    "change-zone-comfort-model": {"zoneId": ZONE16798, "newComfortModel": txt("Thermal comfort model", "Behaglichkeitsmodell", "comfort", D("fixed_hvac (PMV/PPD) or adaptive (EN 16798-1 Annex B.2).", "fixed_hvac (PMV/PPD) oder adaptive (adaptives Modell nach EN 16798-1 Anhang B.2)."))},
    "change-vent-duct-class": {"ventId": VENT16798, "newDuctClass": txt("Duct airtightness class", "Luftdichtheitsklasse der Luftleitungen", "ventilation", D("A, B, C or D.", "A, B, C oder D."))},
    "change-vent-filter-sup": {"ventId": VENT16798, "newFilterSupClass": txt("Supply air filter class", "Filterklasse der Zuluft", "air-quality", D("ISO 16890 class, e.g. ePM1_80_G, ePM1_80, ePM1_55, ePM2_5_65, ePM10_50 or coarse.", "Klasse nach ISO 16890, z. B. ePM1_80_G, ePM1_80, ePM1_55, ePM2_5_65, ePM10_50 oder coarse (Grobstaub)."))},
    "change-zone-co2": {"zoneId": ZONE16798, "newCo2Ppm": q("Indoor CO₂ concentration", "CO₂-Konzentration der Raumluft", "ppm", "air-quality", step=10, prec=0, lo=0)},
}

#endregion din16798

#region din18599

DIN18599 = {
    "update-renewables": {"newRenewables": rec("Renewable energy systems", "Anlagen zur Nutzung erneuerbarer Energien", D("Photovoltaic area and efficiency and the solar thermal yield.", "Photovoltaikfläche und -wirkungsgrad sowie der solarthermische Ertrag."), group="systems")},
    "geg-qp-factor": {"newGegQpFactor": factor("GEG primary energy requirement factor", "GEG-Anforderungsfaktor für den Jahres-Primärenergiebedarf", "requirements", D("Factor applied to the reference building's annual primary energy demand Q_P (GEG § 15).", "Faktor auf den Jahres-Primärenergiebedarf Q_P des Referenzgebäudes (GEG § 15)."), xlo=0)},
    "update-cooling": {"newCooling": rec("Cooling system", "Kühlsystem", D("Cooling plant with EER and energy carrier.", "Kälteerzeugung mit EER und Energieträger."), group="systems")},
    "delta-u-wb": {"newDeltaUWbWM2K": q("Thermal bridge surcharge ΔU_WB", "Wärmebrückenzuschlag ΔU_WB", "W/(m²·K)", "envelope", step=0.01, prec=3, snaps=[0.03, 0.05, 0.1], desc=D("Flat-rate 0.10, 0.05 (Supplement 2) or 0.03 W/(m²·K), or a detailed value (DIN V 18599-2).", "Pauschal 0,10, 0,05 (Beiblatt 2) oder 0,03 W/(m²·K) oder ein detailliert ermittelter Wert (DIN V 18599-2)."))},
    "change-element-u": {"elementId": ref("element", "Envelope element", "Bauteil der Gebäudehülle", D("The envelope element this mutation addresses.", "Das Bauteil der Gebäudehülle, das diese Mutation adressiert.")), "newUValueWM2K": q("Thermal transmittance U", "Wärmedurchgangskoeffizient U", "W/(m²·K)", "envelope", step=0.01, prec=3, lo=0)},
    "update-climate": {"newClimate": rec("Monthly climate", "Monatsklima", D("Monthly mean outdoor temperatures and global irradiances (DIN V 18599-10).", "Monatsmittelwerte der Außentemperatur und der Globalstrahlung (DIN V 18599-10)."), group="climate")},
    "update-ventilation": {"newVentilation": rec("Ventilation system", "Lüftungsanlage", D("Airflow, heat recovery and fan power.", "Volumenstrom, Wärmerückgewinnung und Ventilatorleistung."), group="systems")},
    "automation-class": {"newAutomationClass": rec("Building automation class", "Gebäudeautomationsklasse", D("Class A, B, C or D (DIN V 18599-11).", "Klasse A, B, C oder D (DIN V 18599-11)."), group="systems")},
    "building-category": {"newBuildingCategory": rec("Building category", "Gebäudekategorie", D("Residential or non-residential building.", "Wohngebäude oder Nichtwohngebäude."), group="building")},
    "use-class": {"newUseClass": rec("Usage profile", "Nutzungsprofil", D("Residential, office or school (DIN V 18599-10).", "Wohnen, Büro oder Schule (DIN V 18599-10)."), group="building")},
    "update-lighting": {"newLighting": rec("Lighting", "Beleuchtung", D("Lighting system with its control factor.", "Beleuchtungsanlage mit ihrem Steuerungsfaktor."), group="systems")},
    "net-floor-area-m2": {"newNetFloorAreaM2": q("Net floor area A_NGF", "Nettogrundfläche A_NGF", "m²", "building", step=1, prec=1, xlo=0)},
    "heated-volume-m3": {"newHeatedVolumeM3": q("Heated building volume V_e", "Beheiztes Gebäudevolumen V_e", "m³", "building", step=1, prec=1, xlo=0)},
    "specify-heating-system": {"newHeating": rec("Heating system", "Heizungsanlage", D("Generation, distribution, storage and transfer efficiencies and the energy carrier.", "Erzeuger-, Verteil-, Speicher- und Übergabewirkungsgrad sowie der Energieträger."), group="systems")},
    "replace-zones": {"newZones": rec("Zones", "Zonen", D("The complete list of building zones replacing the current one.", "Die vollständige Liste der Gebäudezonen, die die aktuelle ersetzt."))},
    "specify-dhw-system": {"newDhw": rec("Domestic hot water system", "Trinkwarmwassersystem", D("Specific demand, storage and distribution losses and the energy carrier.", "Spezifischer Bedarf, Speicher- und Verteilverluste sowie der Energieträger."), group="systems")},
    "replace-elements": {"newElements": rec("Envelope elements", "Bauteile der Gebäudehülle", D("The complete list of envelope elements replacing the current one.", "Die vollständige Liste der Hüllbauteile, die die aktuelle ersetzt."))},
    "method": {"newMethod": rec("Calculation method", "Berechnungsverfahren", D("Detailed monthly balance or tabular method.", "Detailliertes Monatsbilanzverfahren oder Tabellenverfahren."), group="building")},
    "attachment": {"newAttachment": rec("Building attachment", "Anbausituation des Gebäudes", D("Detached, semi-detached, end terrace or mid terrace.", "Freistehend, Doppelhaushälfte, Reihenendhaus oder Reihenmittelhaus."), group="building")},
}
DIN18599_NESTED = {
    "update-climate": {
        "newClimate/thetaEC": ui(None, L("Monthly mean outdoor temperature θ_e", "Monatsmittelwert der Außentemperatur θ_e"), desc=D("Twelve values, January to December.", "Zwölf Werte, Januar bis Dezember."), group="climate", unit="°C", step=0.1),
        "newClimate/gHWM2": ui(None, L("Monthly mean global irradiance on the horizontal", "Monatsmittelwert der Globalstrahlung auf die Horizontale"), desc=D("Twelve values, January to December.", "Zwölf Werte, Januar bis Dezember."), group="climate", unit="W/m²", step=1),
    },
}

#endregion din18599

#region din4108

ELEMENT4108 = ref("element", "Envelope element", "Bauteil der Gebäudehülle", D("The envelope element this mutation addresses.", "Das Bauteil der Gebäudehülle, das diese Mutation adressiert."))
ZONE4108 = ref("zone", "Zone", "Zone", D("The thermal zone this mutation addresses.", "Die thermische Zone, die diese Mutation adressiert."))
WINDOW4108 = ref("window", "Window", "Fenster", D("The window of the zone this mutation addresses.", "Das Fenster der Zone, das diese Mutation adressiert."))
BRIDGE4108 = ref("thermalBridge", "Thermal bridge", "Wärmebrücke", D("The thermal bridge this mutation addresses.", "Die Wärmebrücke, die diese Mutation adressiert."))
LAYER4108 = at_entry("Layer", "Schicht", "layer, counted from the interior", "der Schicht, von innen gezählt,")
U_CORR = D("Correction to the thermal transmittance per DIN EN ISO 6946 Annex F.", "Korrektur des Wärmedurchgangskoeffizienten nach DIN EN ISO 6946 Anhang F.")
DIN4108 = {
    "change-element-delta-uf": {"elementId": ELEMENT4108, "newDeltaUF": q("Correction for mechanical fasteners ΔU_f", "Korrekturwert für mechanische Befestigungsteile ΔU_f", "W/(m²·K)", "envelope", step=0.001, prec=3, desc=U_CORR, lo=0)},
    "change-element-delta-ug": {"elementId": ELEMENT4108, "newDeltaUG": q("Correction for air voids ΔU_g", "Korrekturwert für Luftspalte ΔU_g", "W/(m²·K)", "envelope", step=0.001, prec=3, desc=U_CORR, lo=0)},
    "change-element-delta-ur": {"elementId": ELEMENT4108, "newDeltaUR": q("Correction for inverted roofs ΔU_r", "Korrekturwert für Umkehrdächer ΔU_r", "W/(m²·K)", "envelope", step=0.001, prec=3, desc=U_CORR, lo=0)},
    "change-element-adjacent": {"elementId": ELEMENT4108, "newAdjacent": txt("Adjacent space", "Angrenzender Bereich", "envelope", D("exterior, ground, unheated or otherHeated.", "exterior (Außenluft), ground (Erdreich), unheated (unbeheizter Raum) oder otherHeated (anderer beheizter Raum)."))},
    "change-thermal-bridge-length": {"bridgeId": BRIDGE4108, "newLengthM": q("Length l", "Länge l", "m", "thermal-bridges", step=0.1, prec=2, lo=0)},
    "change-zone-window-g-value": {"zoneId": ZONE4108, "windowId": WINDOW4108, "newGValue": ratio("Total solar energy transmittance g", "Gesamtenergiedurchlassgrad g", "summer")},
    "change-zone-window-shading-fc": {"zoneId": ZONE4108, "windowId": WINDOW4108, "newShadingFc": ratio("Shading reduction factor F_C", "Abminderungsfaktor für Sonnenschutzvorrichtungen F_C", "summer")},
    "change-bb2-details-conform": {"newBb2DetailsConform": tog("Details conform to Supplement 2", "Anschlussdetails gleichwertig nach Beiblatt 2", "thermal-bridges", D("Whether every junction detail is equivalent to DIN 4108 Supplement 2.", "Ob alle Anschlussdetails gleichwertig zu DIN 4108 Beiblatt 2 ausgeführt sind."))},
    "insert-layer": {"elementId": ELEMENT4108, "index": at_insert(), "layer": rec("Layer", "Schicht", D("The complete construction layer record to insert.", "Der vollständige einzufügende Schichtdatensatz."))},
    "insert-zone": {"index": at_insert(), "zone": rec("Zone", "Zone", D("The complete thermal zone record to insert.", "Der vollständige einzufügende Zonendatensatz."))},
    "remove-layer": {"elementId": ELEMENT4108, "index": LAYER4108},
    "remove-zone": {"index": at_entry("Zone", "Zone", "zone", "der Zone"), "zone": rec("Zone", "Zone", D("The removed zone record, kept for the inverse.", "Der entfernte Zonendatensatz, für die Umkehrung vorgehalten."))},
    "insert-thermal-bridge": {"index": at_insert(), "bridge": rec("Thermal bridge", "Wärmebrücke", D("The complete thermal bridge record to insert.", "Der vollständige einzufügende Wärmebrückendatensatz."))},
    "change-zone-night-ventilation": {"zoneId": ZONE4108, "newNightVentilation": txt("Night ventilation", "Nachtlüftung", "summer", D("none, moderate or high (DIN 4108-2 summer heat protection).", "none (keine), moderate (erhöhte) oder high (hohe Nachtlüftung) nach DIN 4108-2."))},
    "change-layer-lambda": {"elementId": ELEMENT4108, "index": LAYER4108, "newLambda": q("Design thermal conductivity λ", "Bemessungswert der Wärmeleitfähigkeit λ", "W/(m·K)", "layer", step=0.001, prec=3, xlo=0)},
    "change-t-int-c": {"newTIntC": q("Indoor air temperature θ_i", "Raumlufttemperatur θ_i", "°C", "climate", step=0.5, prec=1)},
    "change-climate-zone": {"newClimateZone": rec("Summer climate region", "Sommerklimaregion", D("Summer climate region A, B or C per DIN 4108-2.", "Sommerklimaregion A, B oder C nach DIN 4108-2."), group="summer")},
    "change-has-mechanical-ventilation": {"newHasMechanicalVentilation": tog("Mechanical ventilation", "Mechanische Lüftung", "climate")},
    "insert-element": {"index": at_insert(), "element": rec("Envelope element", "Bauteil der Gebäudehülle", D("The complete envelope element record to insert.", "Der vollständige einzufügende Bauteildatensatz."))},
    "change-thermal-bridge-bb2-type": {"bridgeId": BRIDGE4108, "newBb2Type": txt("Supplement 2 equivalence category", "Gleichwertigkeitskategorie nach Beiblatt 2", "thermal-bridges", D("categoryA, categoryB or detailed (individual ψ).", "categoryA (Kategorie A), categoryB (Kategorie B) oder detailed (detaillierter ψ-Wert)."))},
    "change-element-kind": {"elementId": ELEMENT4108, "newKind": txt("Element type", "Bauteilart", "envelope", D("wall, roof, floor, window, door, frameOpaque or rollerShutterBox.", "wall (Wand), roof (Dach), floor (Boden), window (Fenster), door (Tür), frameOpaque (opaker Rahmen) oder rollerShutterBox (Rollladenkasten)."))},
    "change-layer-application-type": {"elementId": ELEMENT4108, "index": LAYER4108, "newApplicationType": txt("Application type", "Anwendungsgebiet", "layer", D("DIN 4108-10 code, e.g. DAD, DAA, DUK, DZ, DI, DEO, WAB, WAP, WI or PW.", "Kurzzeichen nach DIN 4108-10, z. B. DAD, DAA, DUK, DZ, DI, DEO, WAB, WAP, WI oder PW."))},
    "change-layer-compressive-class": {"elementId": ELEMENT4108, "index": LAYER4108, "newCompressiveClass": txt("Compressive load class", "Druckbelastbarkeit", "layer", D("DIN 4108-10 class dh, ds, dm, dk or dx.", "Kurzzeichen nach DIN 4108-10: dh, ds, dm, dk oder dx."))},
    "change-layer-mu": {"elementId": ELEMENT4108, "index": LAYER4108, "newMu": q("Water vapour diffusion resistance factor μ", "Wasserdampf-Diffusionswiderstandszahl μ", None, "layer", step=1, prec=0, lo=0)},
    "change-rh-int": {"newRhInt": ratio("Indoor relative humidity φ_i", "Relative Raumluftfeuchte φ_i", "climate")},
    "change-airtightness-n50": {"newAirtightnessN50": q("Air change rate at 50 Pa n₅₀", "Luftwechselrate bei 50 Pa n₅₀", "1/h", "climate", step=0.1, prec=2, lo=0)},
    "change-zone-window-area": {"zoneId": ZONE4108, "windowId": WINDOW4108, "newAreaM2": q("Window area A_w", "Fensterfläche A_w", "m²", "summer", step=0.1, prec=2, lo=0)},
    "change-layer-thickness": {"elementId": ELEMENT4108, "index": LAYER4108, "newThicknessM": q("Layer thickness d", "Schichtdicke d", "m", "layer", disp="mm", step=0.001, prec=0, lo=0)},
    "change-element-inclination-deg": {"elementId": ELEMENT4108, "newInclinationDeg": q("Inclination", "Neigung", "°", "envelope", step=1, prec=0, lo=0, hi=180, widget="dial", desc=D("Angle to the horizontal: 0° horizontal (roof, floor), 90° vertical (wall).", "Winkel zur Horizontalen: 0° waagerecht (Dach, Boden), 90° senkrecht (Wand)."))},
    "change-zone-window-inclination-deg": {"zoneId": ZONE4108, "windowId": WINDOW4108, "newInclinationDeg": q("Window inclination", "Fensterneigung", "°", "summer", step=1, prec=0, lo=0, hi=180, widget="dial", desc=D("Angle to the horizontal: 90° is a vertical window.", "Winkel zur Horizontalen: 90° ist ein senkrechtes Fenster."))},
    "change-element-area": {"elementId": ELEMENT4108, "newAreaM2": q("Element area A", "Bauteilfläche A", "m²", "envelope", step=0.1, prec=2, lo=0)},
    "change-zone-floor-area": {"zoneId": ZONE4108, "newFloorAreaM2": q("Net floor area A_G", "Nettogrundfläche A_G", "m²", "summer", step=1, prec=1, lo=0)},
    "reorder-layers": {"elementId": ELEMENT4108, "from": at_entry("From position", "Von Position", "layer to move", "der zu verschiebenden Schicht"), "to": idx("To position", "Nach Position", D("Zero-based target position of the moved layer.", "Nullbasierte Zielposition der verschobenen Schicht."))},
    "change-thermal-bridge-psi": {"bridgeId": BRIDGE4108, "newPsi": q("Linear thermal transmittance ψ", "Längenbezogener Wärmedurchgangskoeffizient ψ", "W/(m·K)", "thermal-bridges", step=0.001, prec=3)},
    "change-usage": {"newUsage": txt("Building usage", "Gebäudenutzung", "climate", D("residential or nonResidential.", "residential (Wohngebäude) oder nonResidential (Nichtwohngebäude)."))},
    "remove-element": {"index": at_entry("Envelope element", "Bauteil der Gebäudehülle", "envelope element", "des Hüllbauteils")},
    "remove-zone-window": {"zoneId": ZONE4108, "index": at_entry("Window", "Fenster", "window within the zone", "des Fensters in der Zone")},
    "remove-thermal-bridge": {"index": at_entry("Thermal bridge", "Wärmebrücke", "thermal bridge", "der Wärmebrücke")},
    "change-element-orientation-deg": {"elementId": ELEMENT4108, "newOrientationDeg": q("Orientation (azimuth)", "Orientierung (Azimut)", "°", "envelope", step=1, prec=0, soft=(0, 360), widget="dial", desc=D("Azimuth of the outward normal, clockwise from north (0° north, 180° south).", "Azimut der äußeren Flächennormalen, im Uhrzeigersinn ab Nord (0° Nord, 180° Süd)."))},
    "change-zone-window-orientation": {"zoneId": ZONE4108, "windowId": WINDOW4108, "newOrientation": txt("Window orientation", "Fensterorientierung", "summer", D("Compass direction N, NE, E, SE, S, SW, W or NW.", "Himmelsrichtung N, NE (NO), E (O), SE (SO), S, SW, W oder NW."))},
    "change-zone-heaviness": {"zoneId": ZONE4108, "newHeaviness": txt("Construction type", "Bauart", "summer", D("light, medium or heavy (DIN 4108-2).", "light (leicht), medium (mittel) oder heavy (schwer) nach DIN 4108-2."))},
    "change-layer-material-id": {"elementId": ELEMENT4108, "index": LAYER4108, "newMaterialId": ref("material", "Building material", "Baustoff", D("Catalogue material of the layer.", "Katalogbaustoff der Schicht."), role="value", group="layer")},
    "insert-zone-window": {"zoneId": ZONE4108, "index": at_insert(), "window": rec("Window", "Fenster", D("The complete window record to insert into the zone.", "Der vollständige in die Zone einzufügende Fensterdatensatz."))},
}

#endregion din4108

#region en1991


def assumed(en, de, unit, group, disp=None, step=None, prec=None):
    return q(en, de, unit, group, disp=disp, step=step, prec=prec, desc=ASSUMED, lo=0)


EN1991 = {
    "change-assumed-crane-horizontal": {"newAssumedCraneHorizontal": assumed("Assumed horizontal crane force H_T", "Angesetzte horizontale Kranlast H_T", "N", "crane", disp="kN", step=100, prec=1)},
    "change-fire-duration": {"new_fire_duration": q("Fire duration t", "Branddauer t", "s", "fire", disp="min", step=60, prec=0, lo=0, desc=D("Required fire exposure time, e.g. 30, 60 or 90 min.", "Erforderliche Branddauer, z. B. 30, 60 oder 90 min."))},
    "change-assumed-gas-temperature": {"new_assumed_gas_temperature": assumed("Assumed gas temperature θ_g", "Angesetzte Heißgastemperatur θ_g", "K", "fire", step=10, prec=0)},
    "change-silo-kind": {"newSiloKind": txt("Silo or tank", "Silo oder Tank", "silo", D("silo (bulk solids, EN 1991-4 §5) or tank (liquids, §7).", "silo (Schüttgut, EN 1991-4 §5) oder tank (Flüssigkeit, §7)."))},
    "change-assumed-construction-qk": {"newAssumedConstructionQk": assumed("Assumed construction load q_ca", "Angesetzte Nutzlast im Bauzustand q_ca", "Pa", "execution", disp="kN/m²", step=50, prec=2)},
    "change-fire-load-density-qf": {"new_fire_load_density_qf": q("Characteristic fire load density q_f,k", "Charakteristische Brandlastdichte q_f,k", "J/m²", "fire", disp="MJ/m²", step=1e6, prec=0, lo=0)},
    "change-altitude": {"newAltitude": q("Site altitude A", "Geländehöhe über NN A", "m", "site", step=1, prec=0)},
    "change-hoist-class": {"newHoistClass": txt("Hoisting class", "Hubklasse", "crane", D("HC1 to HC4 (EN 1991-3 Annex B).", "HC1 bis HC4 (EN 1991-3 Anhang B)."))},
    "insert-accidental-cases": {"index": at_insert(), "item": rec("Accidental design situation", "Außergewöhnliche Bemessungssituation", D("Impact or explosion case (EN 1991-1-7) to insert.", "Einzufügender Anprall- oder Explosionsfall (EN 1991-1-7)."))},
    "insert-floors": {"index": at_insert(), "item": rec("Floor area", "Deckenfläche", D("Floor with its imposed load category (EN 1991-1-1) to insert.", "Einzufügende Decke mit ihrer Nutzungskategorie (EN 1991-1-1)."))},
    "insert-roofs": {"index": at_insert(), "item": rec("Roof", "Dach", D("Roof with its shape and snow coefficients (EN 1991-1-3) to insert.", "Einzufügendes Dach mit Dachform und Schneelastbeiwerten (EN 1991-1-3)."))},
    "insert-self-weight-elements": {"index": at_insert(), "item": rec("Self-weight element", "Eigenlastbauteil", D("Building element with material and thickness (EN 1991-1-1 Annex A) to insert.", "Einzufügendes Bauteil mit Baustoff und Dicke (EN 1991-1-1 Anhang A)."))},
    "insert-wind-faces": {"index": at_insert(), "item": rec("Wind-loaded face", "Windbeanspruchte Fläche", D("Face with zone, reference height and pressure coefficients (EN 1991-1-4) to insert.", "Einzufügende Fläche mit Bereich, Bezugshöhe und Druckbeiwerten (EN 1991-1-4)."))},
    "change-assumed-crane-wheel": {"newAssumedCraneWheel": assumed("Assumed crane wheel load Q_r", "Angesetzte Kranradlast Q_r", "N", "crane", disp="kN", step=100, prec=1)},
    "remove-accidental-cases": {"index": at_entry("Accidental design situation", "Außergewöhnliche Bemessungssituation", "accidental design situation", "der außergewöhnlichen Bemessungssituation")},
    "remove-floors": {"index": at_entry("Floor area", "Deckenfläche", "floor", "der Decke")},
    "remove-roofs": {"index": at_entry("Roof", "Dach", "roof", "des Dachs")},
    "remove-self-weight-elements": {"index": at_entry("Self-weight element", "Eigenlastbauteil", "self-weight element", "des Eigenlastbauteils")},
    "remove-wind-faces": {"index": at_entry("Wind-loaded face", "Windbeanspruchte Fläche", "wind-loaded face", "der windbeanspruchten Fläche")},
    "change-accidental-assumed-force": {"index": at_entry("Accidental design situation", "Außergewöhnliche Bemessungssituation", "accidental design situation", "der außergewöhnlichen Bemessungssituation"), "newAssumedForce": assumed("Assumed impact force F_d", "Angesetzte Anprallkraft F_d", "N", "accidental", disp="kN", step=1000, prec=0)},
    "change-bridge-lane-width": {"newBridgeLaneWidth": q("Carriageway width w", "Fahrbahnbreite w", "m", "bridge", step=0.1, prec=2, lo=0)},
    "change-coast-or-island": {"newCoastOrIsland": tog("Coastal or island location", "Küsten- oder Insellage", "site", D("Site at the coast or on an island (DIN EN 1991-1-4/NA wind profile).", "Standort an der Küste oder auf einer Insel (Windprofil nach DIN EN 1991-1-4/NA)."))},
    "change-crane-claimed": {"newCraneClaimed": tog("Crane loads declared", "Kranlasten angesetzt", "crane", D("Whether the structure carries crane runway actions (EN 1991-3).", "Ob das Tragwerk Einwirkungen aus Kranbahnen erhält (EN 1991-3)."))},
    "change-en-sk": {"newEnSk": q("Characteristic ground snow load s_k (EN)", "Charakteristischer Wert der Schneelast auf dem Boden s_k (EN)", "Pa", "site", disp="kN/m²", step=50, prec=2, lo=0)},
    "change-floor-assumed-qk": {"index": at_entry("Floor area", "Deckenfläche", "floor", "der Decke"), "newAssumedQk": assumed("Assumed imposed load q_k", "Angesetzte Nutzlast q_k", "Pa", "floors", disp="kN/m²", step=50, prec=2)},
    "change-hoisting-speed": {"newHoistingSpeed": q("Steady hoisting speed v_h", "Stationäre Hubgeschwindigkeit v_h", "m/s", "crane", step=0.05, prec=2, lo=0)},
    "change-mixed-terrain-distance": {"newMixedTerrainDistance": q("Distance to the terrain category change", "Abstand zum Geländekategoriewechsel", "m", "site", step=10, prec=0, lo=0, desc=D("Upwind distance to the change of roughness (EN 1991-1-4 Annex A.2).", "Abstand in Luv zum Rauigkeitswechsel (EN 1991-1-4 Anhang A.2)."))},
    "change-roof-assumed-sk": {"index": at_entry("Roof", "Dach", "roof", "des Dachs"), "newAssumedSk": assumed("Assumed roof snow load s", "Angesetzte Schneelast auf dem Dach s", "Pa", "roofs", disp="kN/m²", step=50, prec=2)},
    "change-silo-bulk-density": {"newSiloBulkDensity": q("Bulk solid unit weight γ", "Wichte des Schüttguts γ", "N/m³", "silo", disp="kN/m³", step=100, prec=1, lo=0)},
    "change-silo-claimed": {"newSiloClaimed": tog("Silo loads declared", "Silolasten angesetzt", "silo", D("Whether silo or tank actions apply (EN 1991-4).", "Ob Einwirkungen auf Silos oder Tanks angesetzt werden (EN 1991-4)."))},
    "change-silo-hydraulic-radius": {"newSiloHydraulicRadius": q("Hydraulic radius a/U", "Hydraulischer Radius A/U", "m", "silo", step=0.01, prec=2, lo=0)},
    "change-silo-k": {"newSiloK": factor("Lateral pressure ratio K", "Horizontallastverhältnis K", "silo", lo=0)},
    "change-snow-zone": {"newSnowZone": txt("Snow load zone", "Schneelastzone", "site", D("DIN EN 1991-1-3/NA zone 1, 1a, 2, 2a or 3.", "Zone 1, 1a, 2, 2a oder 3 nach DIN EN 1991-1-3/NA."))},
    "change-structure-kind": {"newStructureKind": tog("Structure type", "Tragwerksart", "site", D("Building or bridge; EN 1991-2 traffic loads apply to bridges.", "Gebäude oder Brücke; Verkehrslasten nach EN 1991-2 gelten für Brücken."))},
    "change-terrain-category": {"newTerrainCategory": count("Terrain category", "Geländekategorie", "site", D("0 (sea) and I to IV as 1 to 4 (EN 1991-1-4 Table 4.1).", "0 (See) sowie I bis IV als 1 bis 4 (EN 1991-1-4 Tabelle 4.1)."), hi=4)},
    "change-wind-zone": {"newWindZone": count("Wind zone", "Windzone", "site", D("DIN EN 1991-1-4/NA wind zone 1 to 4.", "Windzone 1 bis 4 nach DIN EN 1991-1-4/NA."), lo=1, hi=4)},
    "change-assumed-silo-pressure": {"newAssumedSiloPressure": assumed("Assumed horizontal wall pressure p_h", "Angesetzter Horizontallastdruck p_h", "Pa", "silo", disp="kN/m²", step=50, prec=2)},
    "change-bridge-lane": {"newBridgeLane": count("Notional lane number", "Nummer des rechnerischen Fahrstreifens", "bridge", D("Notional lane i of load model 1 (EN 1991-2 §4.2.4).", "Rechnerischer Fahrstreifen i des Lastmodells 1 (EN 1991-2 §4.2.4)."), lo=1)},
    "change-thermal-bridge-type": {"new_thermal_bridge_type": count("Bridge deck type", "Überbautyp", "thermal", D("Type 1 steel, 2 composite or 3 concrete deck (EN 1991-1-5 §6.1.1).", "Typ 1 Stahl-, 2 Verbund- oder 3 Betonüberbau (EN 1991-1-5 §6.1.1)."), lo=1, hi=3)},
    "change-annex": {"newAnnex": ANNEX_TEXT},
    "change-assumed-delta-t": {"newAssumedDeltaT": q("Assumed uniform temperature component ΔT_u", "Angesetzter konstanter Temperaturanteil ΔT_u", "K", "thermal", step=1, prec=1, desc=ASSUMED)},
    "change-t-max": {"new_t_max": q("Maximum shade air temperature T_max", "Maximale Außenlufttemperatur im Schatten T_max", "°C", "thermal", step=1, prec=0)},
    "change-en-vb": {"newEnVb": q("Basic wind velocity v_b (EN)", "Basiswindgeschwindigkeit v_b (EN)", "m/s", "site", step=0.5, prec=1, lo=0)},
    "change-assumed-bridge-tandem": {"newAssumedBridgeTandem": assumed("Assumed tandem system axle load Q_ik", "Angesetzte Achslast der Doppelachse Q_ik", "N", "bridge", disp="kN", step=1000, prec=0)},
    "change-exceptional-snow-north-german-lowlands": {"newExceptionalSnowNorthGermanLowlands": tog("Exceptional snow in the North German Lowlands", "Außergewöhnliche Schneelast im Norddeutschen Tiefland", "site", D("Accidental snow load per DIN EN 1991-1-3/NA for the North German Lowlands.", "Schnee als außergewöhnliche Einwirkung nach DIN EN 1991-1-3/NA im Norddeutschen Tiefland."))},
    "change-bridge-span": {"newBridgeSpan": q("Bridge span L", "Stützweite der Brücke L", "m", "bridge", step=0.5, prec=2, lo=0)},
    "change-thermal-element-type": {"new_thermal_element_type": txt("Element type for thermal actions", "Bauteiltyp für Temperatureinwirkungen", "thermal", D("building, bridge1, bridge2 or bridge3.", "building (Gebäudebauteil), bridge1, bridge2 oder bridge3 (Brückentyp 1 bis 3)."))},
    "change-storey-count": {"new_storey_count": count("Number of storeys", "Geschossanzahl", "fire")},
    "change-width": {"newWidth": q("Building width b", "Gebäudebreite b", "m", "building", step=0.1, prec=2, lo=0)},
    "change-air-density": {"newAirDensity": q("Air density ρ", "Luftdichte ρ", "kg/m³", "site", step=0.01, prec=3, lo=0)},
    "change-fire-occupancy": {"new_fire_occupancy": txt("Occupancy for fire design", "Nutzungsart für die Brandschutzbemessung", "fire", D("office, dwelling, hospital, shopping, industrial, warehouse or parking (EN 1991-1-2 Annex E).", "office (Büro), dwelling (Wohnen), hospital (Krankenhaus), shopping (Verkauf), industrial (Industrie), warehouse (Lager) oder parking (Parken) nach EN 1991-1-2 Anhang E."))},
    "change-silo-height": {"newSiloHeight": q("Silo height h_c", "Silohöhe h_c", "m", "silo", step=0.1, prec=2, lo=0)},
    "change-assumed-bridge-lm4": {"new_assumed_bridge_lm4": assumed("Assumed crowd load of load model 4", "Angesetzte Menschengedrängelast des Lastmodells 4", "Pa", "bridge", disp="kN/m²", step=50, prec=2)},
    "change-crane-class": {"newCraneClass": txt("Crane class", "Kranklasse", "crane", D("Hoisting class HC1 to HC4 of the crane.", "Hubklasse HC1 bis HC4 des Krans."))},
    "change-depth": {"newDepth": q("Building depth d", "Gebäudetiefe d", "m", "building", step=0.1, prec=2, lo=0)},
    "change-fire-curve": {"new_fire_curve": txt("Nominal temperature-time curve", "Nominelle Temperaturzeitkurve", "fire", D("Standard (ISO 834), External, Hydrocarbon or Parametric (EN 1991-1-2 §3.2, Annex A).", "Standard (Einheits-Temperaturzeitkurve), External (Außenbrandkurve), Hydrocarbon (Hydrokarbonkurve) oder Parametric (Anhang A)."))},
    "change-linear-temperature-gradient": {"new_delta_t_m": q("Linear temperature difference ΔT_M", "Linear veränderlicher Temperaturanteil ΔT_M", "K", "thermal", step=1, prec=1)},
    "change-fire-compartment-height": {"new_fire_compartment_height": q("Fire compartment height H", "Höhe des Brandabschnitts H", "m", "fire", step=0.1, prec=2, lo=0)},
    "change-orography-factor": {"newOrographyFactor": factor("Orography factor c_o", "Topografiebeiwert c_o", "site", lo=0)},
    "change-assumed-silo-patch": {"newAssumedSiloPatch": assumed("Assumed patch pressure p_p", "Angesetzte Teilflächenlast p_p", "Pa", "silo", disp="kN/m²", step=50, prec=2)},
    "change-bridge-load-group": {"new_bridge_load_group": txt("Traffic load group", "Verkehrslastgruppe", "bridge", D("gr1a, gr1b, gr3, gr4 or gr5 (EN 1991-2 Table 4.4a).", "gr1a, gr1b, gr3, gr4 oder gr5 (EN 1991-2 Tabelle 4.4a)."))},
    "change-assumed-h-net": {"new_assumed_h_net": q("Assumed net heat flux ḣ_net", "Angesetzter Nettowärmestrom ḣ_net", "W/m²", "fire", disp="kW/m²", step=100, prec=1, desc=ASSUMED)},
    "change-assumed-qf-d": {"new_assumed_qf_d": assumed("Assumed design fire load density q_f,d", "Angesetzter Bemessungswert der Brandlastdichte q_f,d", "J/m²", "fire", disp="MJ/m²", step=1e6, prec=0)},
    "change-silo-mu": {"newSiloMu": factor("Wall friction coefficient μ", "Wandreibungskoeffizient μ", "silo", lo=0)},
    "change-construction-activity": {"newConstructionActivity": txt("Construction activity", "Tätigkeit im Bauzustand", "execution", D("scaffolding, formwork, landing or working_platform (EN 1991-1-6).", "scaffolding (Gerüst), formwork (Schalung), landing (Lagerfläche) oder working_platform (Arbeitsbühne) nach EN 1991-1-6."))},
    "change-fire-mode": {"new_fire_mode": sel("Fire design method", "Brandschutzbemessungsverfahren", "fire", {"none": ("None — no fire design", "Keine Brandschutzbemessung"), "nominal": ("Nominal fire curves (§3.2)", "Nominelle Temperaturzeitkurven (§3.2)"), "parametric": ("Parametric fire (Annex A)", "Parametrische Temperaturzeitkurve (Anhang A)")}, widget="segmented")},
    "change-initial-temperature": {"new_t_0": q("Initial temperature T_0", "Aufstelltemperatur T_0", "°C", "thermal", step=1, prec=0)},
    "change-fire-compartment-area": {"new_fire_compartment_area": q("Fire compartment floor area A_f", "Grundfläche des Brandabschnitts A_f", "m²", "fire", step=1, prec=1, lo=0)},
    "change-assumed-bridge-lm2": {"newAssumedBridgeLm2": assumed("Assumed single axle load of load model 2", "Angesetzte Einzelachslast des Lastmodells 2", "N", "bridge", disp="kN", step=1000, prec=0)},
    "change-assumed-bridge-lm3": {"new_assumed_bridge_lm3": assumed("Assumed special vehicle axle load of load model 3", "Angesetzte Achslast des Sonderfahrzeugs (Lastmodell 3)", "N", "bridge", disp="kN", step=1000, prec=0)},
    "change-self-weight-assumed-gk": {"index": at_entry("Self-weight element", "Eigenlastbauteil", "self-weight element", "des Eigenlastbauteils"), "newAssumedGk": assumed("Assumed self-weight g_k", "Angesetzte Eigenlast g_k", "Pa", "self-weight", disp="kN/m²", step=50, prec=2)},
    "change-assumed-bridge-footway": {"newAssumedBridgeFootway": assumed("Assumed footway load q_fk", "Angesetzte Gehweglast q_fk", "Pa", "bridge", disp="kN/m²", step=50, prec=2)},
    "change-wind-face-assumed-wp": {"index": at_entry("Wind-loaded face", "Windbeanspruchte Fläche", "wind-loaded face", "der windbeanspruchten Fläche"), "newAssumedWp": q("Assumed wind pressure w_e", "Angesetzter Winddruck w_e", "Pa", "wind", disp="kN/m²", step=50, prec=2, desc=ASSUMED)},
    "change-assumed-bridge-udl": {"newAssumedBridgeUdl": assumed("Assumed uniformly distributed load q_ik", "Angesetzte gleichmäßig verteilte Last q_ik", "Pa", "bridge", disp="kN/m²", step=50, prec=2)},
    "change-t-min": {"new_t_min": q("Minimum shade air temperature T_min", "Minimale Außenlufttemperatur im Schatten T_min", "°C", "thermal", step=1, prec=0)},
    "change-mixed-terrain-upwind": {"newMixedTerrainUpwind": count("Upwind terrain category", "Geländekategorie in Luv", "site", D("Roughness upwind of the change: 0 and I to IV as 1 to 4.", "Rauigkeit in Luv des Wechsels: 0 sowie I bis IV als 1 bis 4."), hi=4)},
    "change-assumed-silo-wall-friction": {"newAssumedSiloWallFriction": assumed("Assumed wall frictional traction p_w", "Angesetzte Wandreibungsspannung p_w", "Pa", "silo", disp="kN/m²", step=50, prec=2)},
    "change-fire-thermal-inertia": {"new_fire_thermal_inertia": q("Thermal absorptivity b", "Wärmeeindringzahl b", "J/(m²·s^½·K)", "fire", step=10, prec=0, lo=0, desc=D("b = √(ρ·c·λ) of the enclosure (EN 1991-1-2 Annex A).", "b = √(ρ·c·λ) der Umfassungsbauteile (EN 1991-1-2 Anhang A)."))},
    "change-height": {"newHeight": q("Building height h", "Gebäudehöhe h", "m", "building", step=0.1, prec=2, lo=0)},
    "change-fire-opening-factor": {"new_fire_opening_factor": q("Opening factor O", "Öffnungsfaktor O", "m^½", "fire", step=0.01, prec=3, lo=0, desc=D("O = A_v·√h_eq / A_t (EN 1991-1-2 Annex A).", "O = A_v·√h_eq / A_t (EN 1991-1-2 Anhang A)."))},
}

#endregion en1991

#region en1992

MEMBER1992 = ref("member", "Member", "Bauteil", D("The reinforced concrete member this mutation addresses.", "Das Stahlbetonbauteil, das diese Mutation adressiert."))
ACTION1992 = ref("action", "Action", "Einwirkung", D("The characteristic action (load case) of the member.", "Die charakteristische Einwirkung (Lastfall) des Bauteils."))
ANCHOR1992 = ref("anchor", "Anchor", "Dübel", D("The post-installed anchor this mutation addresses.", "Der Dübel, den diese Mutation adressiert."))
EN1992 = {
    "change-bar-layer-count": {"memberId": MEMBER1992, "layerId": ref("barLayer", "Bar layer", "Bewehrungslage", D("The longitudinal reinforcement layer of the member.", "Die Längsbewehrungslage des Bauteils.")), "newCount": count("Number of bars n", "Stabanzahl n", "reinforcement")},
    "change-member-width": {"memberId": MEMBER1992, "newValue": q("Cross-section width b", "Querschnittsbreite b", "m", "geometry", disp="mm", step=0.005, prec=0, lo=0)},
    "change-member-height": {"memberId": MEMBER1992, "newValue": q("Cross-section height h", "Querschnittshöhe h", "m", "geometry", disp="mm", step=0.005, prec=0, lo=0)},
    "change-action-v-ed": {"memberId": MEMBER1992, "actionId": ACTION1992, "newValue": q("Characteristic shear force V_k", "Charakteristische Querkraft V_k", "N", "actions", disp="kN", step=100, prec=1)},
    "insert-anchor": {"index": at_insert(), "anchor": rec("Anchor", "Dübel", D("The complete anchor record (EN 1992-4) to insert.", "Der vollständige einzufügende Dübeldatensatz (EN 1992-4)."))},
    "insert-member": {"index": at_insert(), "member": rec("Member", "Bauteil", D("The complete reinforced concrete member record to insert.", "Der vollständige einzufügende Stahlbetonbauteil-Datensatz."))},
    "remove-member": {"memberId": MEMBER1992},
    "change-action-mk": {"memberId": MEMBER1992, "actionId": ACTION1992, "newValue": q("Characteristic bending moment M_k", "Charakteristisches Biegemoment M_k", "N·m", "actions", disp="kN·m", step=100, prec=1)},
    "change-bar-layer-diameter": {"memberId": MEMBER1992, "layerId": ref("barLayer", "Bar layer", "Bewehrungslage", D("The longitudinal reinforcement layer of the member.", "Die Längsbewehrungslage des Bauteils.")), "newDiameter": q("Bar diameter Ø", "Stabdurchmesser Ø", "m", "reinforcement", disp="mm", step=0.001, prec=0, xlo=0, snaps=[0.006, 0.008, 0.01, 0.012, 0.014, 0.016, 0.02, 0.025, 0.028, 0.032, 0.04])},
    "change-member-span": {"memberId": MEMBER1992, "newValue": q("Effective span l_eff", "Stützweite l_eff", "m", "geometry", step=0.05, prec=2, lo=0)},
    "change-annex": {"newAnnex": ANNEX_TEXT},
    "change-member-exposure": {"memberId": MEMBER1992, "newExposure": txt("Exposure class", "Expositionsklasse", "durability", D("EN 1992-1-1 Table 4.1: X0, Xc1–Xc4, Xd1–Xd3, Xs1–Xs3, Xf1–Xf4 or Xa1–Xa3.", "Nach EN 1992-1-1 Tabelle 4.1: X0, Xc1–Xc4, Xd1–Xd3, Xs1–Xs3, Xf1–Xf4 oder Xa1–Xa3."))},
    "change-action-n-ed": {"memberId": MEMBER1992, "actionId": ACTION1992, "newValue": q("Characteristic axial force N_k", "Charakteristische Normalkraft N_k", "N", "actions", disp="kN", step=100, prec=1)},
    "change-title": {"newTitle": txt("Report title", "Berichtstitel", "report")},
    "change-design-working-life": {"newYears": q("Design working life T_lg", "Geplante Nutzungsdauer T_lg", "a", "durability", step=1, prec=0, xlo=0, snaps=[10, 25, 50, 100])},
    "change-anchor-h-ef": {"anchorId": ANCHOR1992, "newValue": q("Effective embedment depth h_ef", "Effektive Verankerungstiefe h_ef", "m", "anchor", disp="mm", step=0.005, prec=0, xlo=0)},
    "change-delta-c-dev": {"newDeltaCDev": q("Allowance for deviation Δc_dev", "Vorhaltemaß Δc_dev", "m", "durability", disp="mm", step=0.001, prec=0, lo=0, snaps=[0, 0.005, 0.01, 0.015])},
    "change-member-effective-depth": {"memberId": MEMBER1992, "newValue": q("Effective depth d", "Statische Nutzhöhe d", "m", "geometry", disp="mm", step=0.005, prec=0, lo=0)},
    "reorder-members": {"fromIndex": at_entry("From position", "Von Position", "member to move", "des zu verschiebenden Bauteils"), "toIndex": idx("To position", "Nach Position", D("Zero-based target position; a larger value moves the member to the end.", "Nullbasierte Zielposition; ein größerer Wert verschiebt das Bauteil ans Ende."))},
    "change-member-axis-distance": {"memberId": MEMBER1992, "newAxisDistance": q("Axis distance a", "Achsabstand a", "m", "fire", disp="mm", step=0.005, prec=0, lo=0, desc=D("Axis distance of the longitudinal bars to the exposed face (EN 1992-1-2 §5).", "Achsabstand der Längsbewehrung zur beflammten Oberfläche (EN 1992-1-2 §5)."))},
    "change-member-fire-rating": {"memberId": MEMBER1992, "newRating": txt("Fire resistance class", "Feuerwiderstandsklasse", "fire", D("R30, R60, R90 or R120 (EN 1992-1-2).", "R30, R60, R90 oder R120 (EN 1992-1-2)."))},
    "change-reinforcement-f-yk": {"gradeId": ref("reinforcementGrade", "Reinforcing steel grade", "Betonstahlsorte", D("The reinforcing steel grade this mutation addresses.", "Die Betonstahlsorte, die diese Mutation adressiert.")), "newFYk": q("Characteristic yield strength f_yk", "Charakteristische Streckgrenze f_yk", "Pa", "material", disp="N/mm²", step=5e6, prec=0, xlo=0)},
    "remove-anchor": {"anchorId": ANCHOR1992},
    "change-member-cover": {"memberId": MEMBER1992, "newValue": q("Nominal concrete cover c_nom", "Nennmaß der Betondeckung c_nom", "m", "durability", disp="mm", step=0.005, prec=0, lo=0)},
    "change-cement-type": {"newCementType": txt("Cement class", "Zementklasse", "material", D("Strength development class S, N or R (EN 1992-1-1 §3.1.2).", "Festigkeitsentwicklungsklasse S, N oder R (EN 1992-1-1 §3.1.2)."))},
    "change-concrete-f-ck": {"gradeId": ref("concreteGrade", "Concrete strength class", "Betonfestigkeitsklasse", D("The concrete grade this mutation addresses.", "Die Betonfestigkeitsklasse, die diese Mutation adressiert.")), "newFCk": q("Characteristic compressive cylinder strength f_ck", "Charakteristische Zylinderdruckfestigkeit f_ck", "Pa", "material", disp="N/mm²", step=1e6, prec=0, xlo=0, snaps=[1.2e7, 1.6e7, 2e7, 2.5e7, 3e7, 3.5e7, 4e7, 4.5e7, 5e7])},
    "change-anchor-a-s": {"anchorId": ANCHOR1992, "newValue": q("Stressed cross-section A_s", "Spannungsquerschnitt A_s", "m²", "anchor", disp="cm²", step=1e-6, prec=2, xlo=0)},
    "change-member-stirrup-spacing": {"memberId": MEMBER1992, "newSpacing": q("Stirrup spacing s", "Bügelabstand s", "m", "reinforcement", disp="mm", step=0.01, prec=0, xlo=0)},
}

#endregion en1992

#region en1993

def mpa(en, de, group, desc=None, lo=None, xlo=None):
    return q(en, de, "N/mm²", group, step=5, prec=0, desc=desc, lo=lo, xlo=xlo)


def mm(en, de, group, desc=None, lo=0, step=1, prec=1):
    return q(en, de, "mm", group, step=step, prec=prec, desc=desc, lo=lo)


def kn(en, de, group, desc=None, lo=None):
    return q(en, de, "kN", group, step=1, prec=1, desc=desc, lo=lo)


def knm(en, de, group, desc=None):
    return q(en, de, "kN·m", group, step=1, prec=1, desc=desc)


EN1993 = {
    "update-through-thickness-inputs": {
        "newT10SteelSubgrade": txt("Steel sub-grade", "Gütegruppe", "fracture", D("JR, J0, J2 or K2 (EN 10025, EN 1993-1-10 Table 2.1).", "JR, J0, J2 oder K2 (EN 10025, EN 1993-1-10 Tabelle 2.1).")),
        "newT10ActualThicknessMm": mm("Element thickness t", "Erzeugnisdicke t", "fracture"),
        "newT10TEdC": q("Reference temperature T_Ed", "Bezugstemperatur T_Ed", "°C", "fracture", step=1, prec=0, desc=D("Lowest service temperature at the detail (EN 1993-1-10 §2.2).", "Tiefste Einsatztemperatur am Detail (EN 1993-1-10 §2.2).")),
    },
    "update-stainless-inputs": {
        "newStainlessMEdKnm": knm("Design bending moment M_Ed", "Bemessungswert des Biegemoments M_Ed", "stainless"),
        "newStainlessWPlMm3": q("Plastic section modulus W_pl", "Plastisches Widerstandsmoment W_pl", "mm³", "stainless", step=1000, prec=0, lo=0),
        "newStainlessFYMpa": mpa("0.2 % proof strength f_y", "0,2 %-Dehngrenze f_y", "stainless", xlo=0),
    },
    "change-annex": {"newAnnex": sel("National annex", "Nationaler Anhang", "annex", ANNEX_OPTIONS, widget="segmented")},
    "update-hss-inputs": {
        "newHssWElMm3": q("Elastic section modulus W_el", "Elastisches Widerstandsmoment W_el", "mm³", "high-strength", step=1000, prec=0, lo=0),
        "newHssFYMpa": mpa("Yield strength f_y", "Streckgrenze f_y", "high-strength", xlo=0),
        "newHssSectionClass": count("Cross-section class", "Querschnittsklasse", "high-strength", D("Class 1 to 4 (EN 1993-1-1 §5.5).", "Klasse 1 bis 4 (EN 1993-1-1 §5.5)."), lo=1, hi=4),
        "newHssMEdKnm": knm("Design bending moment M_Ed", "Bemessungswert des Biegemoments M_Ed", "high-strength"),
    },
    "update-bridge-inputs": {
        "newBridgeLambda": factor("Damage equivalence factor λ", "Schadensäquivalenzfaktor λ", "fatigue", lo=0),
        "newBridgePhi2": factor("Dynamic factor φ_2", "Schwingbeiwert φ_2", "fatigue", lo=0),
        "newBridgeDeltaSigmaPMpa": mpa("Stress range Δσ_p", "Spannungsschwingbreite Δσ_p", "fatigue", lo=0),
    },
    "update-crane-inputs": {
        "newCraneFZEdKn": kn("Design wheel load F_z,Ed", "Bemessungswert der Radlast F_z,Ed", "crane", lo=0),
        "newCraneWheelContactLengthMm": mm("Wheel contact length", "Radaufstandslänge", "crane"),
        "newCraneDispersionMm": mm("Load dispersion length", "Lastausbreitungslänge", "crane"),
        "newCraneTWMm": mm("Web thickness t_w", "Stegdicke t_w", "crane"),
    },
    "update-member-properties": {
        "newNEdKn": kn("Design axial force N_Ed", "Bemessungswert der Normalkraft N_Ed", "actions"),
        "newMEdKnm": knm("Design bending moment M_Ed", "Bemessungswert des Biegemoments M_Ed", "actions"),
        "newVEdKn": kn("Design shear force V_Ed", "Bemessungswert der Querkraft V_Ed", "actions"),
        "newAMm2": q("Cross-section area A", "Querschnittsfläche A", "mm²", "section", step=10, prec=0, lo=0),
        "newAVMm2": q("Shear area A_v", "Schubfläche A_v", "mm²", "section", step=10, prec=0, lo=0),
        "newWPlMm3": q("Plastic section modulus W_pl", "Plastisches Widerstandsmoment W_pl", "mm³", "section", step=1000, prec=0, lo=0),
        "newFYMpa": mpa("Yield strength f_y", "Streckgrenze f_y", "material", xlo=0),
        "newFUMpa": mpa("Ultimate tensile strength f_u", "Zugfestigkeit f_u", "material", xlo=0),
        "newChi": factor("Buckling reduction factor χ", "Abminderungsbeiwert für Biegeknicken χ", "section", lo=0, hi=1),
        "newANetMm2": q("Net cross-section area A_net", "Nettoquerschnittsfläche A_net", "mm²", "section", step=10, prec=0, lo=0),
        "newTensionNEdKn": kn("Design tension force N_t,Ed", "Bemessungswert der Zugkraft N_t,Ed", "actions"),
    },
    "update-fatigue-inputs": {
        "newDeltaSigmaMpa": mpa("Stress range Δσ", "Spannungsschwingbreite Δσ", "fatigue", lo=0),
        "newFatigueCategory": count("Detail category Δσ_C", "Kerbfall Δσ_C", "fatigue", D("Detail category in N/mm² (EN 1993-1-9 Tables 8.1–8.10).", "Kerbfall in N/mm² (EN 1993-1-9 Tabellen 8.1–8.10)."), unit="N/mm²"),
        "newFatigueMethod": txt("Assessment method", "Bemessungskonzept", "fatigue", D("damage_tolerant or safe_life (EN 1993-1-9 §3).", "damage_tolerant (Schadenstoleranz) oder safe_life (ausreichende Sicherheit gegen Ermüdungsversagen) nach EN 1993-1-9 §3.")),
    },
    "update-fire-inputs": {
        "newFireThicknessMm": mm("Fire protection thickness d_p", "Dicke der Brandschutzbekleidung d_p", "fire"),
        "newFireRating": txt("Fire resistance class", "Feuerwiderstandsklasse", "fire", D("r30, r60, r90 or r120.", "r30, r60, r90 oder r120 (R 30 bis R 120).")),
        "newFireMassivity": q("Section factor A_m/V", "Profilfaktor A_m/V", "1/m", "fire", step=1, prec=0, lo=0),
        "newFireMu0": factor("Degree of utilisation μ_0", "Ausnutzungsgrad μ_0", "fire", lo=0),
        "newFireDesignTemperatureC": q("Design steel temperature θ_a", "Bemessungstemperatur des Stahls θ_a", "°C", "fire", step=10, prec=0),
    },
    "update-bolt-inputs": {
        "newBoltFEdKn": kn("Design force per bolt F_Ed", "Bemessungswert der Kraft je Schraube F_Ed", "bolts"),
        "newBoltNBolts": count("Number of bolts n", "Schraubenanzahl n", "bolts"),
        "newBoltASMm2": q("Tensile stress area A_s", "Spannungsquerschnitt A_s", "mm²", "bolts", step=1, prec=0, lo=0),
        "newBoltE1Mm": mm("End distance e_1", "Randabstand in Kraftrichtung e_1", "bolts"),
        "newBoltE2Mm": mm("Edge distance e_2", "Randabstand quer zur Kraftrichtung e_2", "bolts"),
        "newBoltD0Mm": mm("Hole diameter d_0", "Lochdurchmesser d_0", "bolts"),
        "newBoltDMm": mm("Bolt diameter d", "Schraubendurchmesser d", "bolts"),
        "newBoltTMm": mm("Plate thickness t", "Blechdicke t", "bolts"),
        "newBoltFUMpa": mpa("Plate ultimate strength f_u", "Zugfestigkeit des Blechs f_u", "bolts", xlo=0),
        "newBoltFUbMpa": mpa("Bolt ultimate strength f_ub", "Zugfestigkeit der Schraube f_ub", "bolts", xlo=0),
    },
    "update-tower-inputs": {
        "newTowerWindFactor": factor("Wind load factor", "Windlastfaktor", "tower", lo=0),
        "newTowerNEdKn": kn("Design leg force N_Ed", "Bemessungswert der Eckstielkraft N_Ed", "tower"),
    },
    "update-silo-shell-inputs": {
        "newSiloTMm": mm("Wall thickness t", "Wanddicke t", "silo"),
        "newSiloRMm": mm("Shell radius r", "Schalenradius r", "silo"),
        "newShellSigmaXEdMpa": mpa("Design meridional stress σ_x,Ed", "Bemessungswert der Meridianspannung σ_x,Ed", "silo"),
        "newSiloK": factor("Lateral pressure ratio K", "Horizontallastverhältnis K", "silo", lo=0),
        "newSiloGammaKnM3": q("Bulk solid unit weight γ", "Wichte des Schüttguts γ", "kN/m³", "silo", step=0.1, prec=1, lo=0),
        "newSiloDepthM": q("Depth below the equivalent surface z", "Tiefe unter der Schüttgutoberfläche z", "m", "silo", step=0.1, prec=2, lo=0),
    },
    "update-cold-formed-inputs": {
        "newCfBBarMm": mm("Notional flat width b_p", "Fiktive ebene Breite b_p", "cold-formed"),
        "newCfTMm": mm("Design core thickness t", "Bemessungskerndicke t", "cold-formed", step=0.1, prec=2),
        "newCfKSigma": factor("Buckling factor k_σ", "Beulwert k_σ", "cold-formed", lo=0),
        "newCfPsi": factor("Stress ratio ψ", "Randspannungsverhältnis ψ", "cold-formed"),
        "newCfNEdKn": kn("Design axial force N_Ed", "Bemessungswert der Normalkraft N_Ed", "cold-formed"),
        "newCfGrossResistanceKn": kn("Gross cross-section resistance", "Tragfähigkeit des Bruttoquerschnitts", "cold-formed", lo=0),
    },
    "update-plated-inputs": {
        "newPlatedLambdaP": factor("Plate slenderness λ̄_p", "Plattenschlankheit λ̄_p", "plated", lo=0),
        "newPlatedSigmaEdMpa": mpa("Design stress σ_Ed", "Bemessungswert der Spannung σ_Ed", "plated"),
    },
    "update-weld-inputs": {
        "newWeldAMm": mm("Throat thickness a", "Kehlnahtdicke a", "welds"),
        "newWeldLMm": mm("Effective weld length l_eff", "Wirksame Nahtlänge l_eff", "welds"),
        "newWeldFUMpa": mpa("Ultimate strength f_u", "Zugfestigkeit f_u", "welds", xlo=0),
        "newWeldSteelGrade": txt("Steel grade", "Stahlsorte", "welds", D("E.g. S235, S275, S355 or S460 (correlation factor β_w).", "Z. B. S235, S275, S355 oder S460 (Korrelationsbeiwert β_w).")),
        "newWeldFEdKn": kn("Design weld force F_w,Ed", "Bemessungswert der Schweißnahtkraft F_w,Ed", "welds"),
    },
    "update-tension-component-inputs": {
        "newTensionComponentFUkKn": kn("Characteristic breaking force F_uk", "Charakteristische Bruchkraft F_uk", "tension", lo=0),
        "newTensionComponentFKKn": kn("Characteristic force F_k", "Charakteristische Zugkraft F_k", "tension"),
        "newTensionComponentNEdKn": kn("Design tension force N_Ed", "Bemessungswert der Zugkraft N_Ed", "tension"),
    },
    "update-pile-inputs": {
        "newPileSigmaMpa": mpa("Design stress σ", "Bemessungswert der Spannung σ", "piles"),
        "newPileKRed": factor("Reduction factor k_red", "Abminderungsfaktor k_red", "piles", lo=0),
        "newPileNEdKn": kn("Design axial force N_Ed", "Bemessungswert der Normalkraft N_Ed", "piles"),
    },
}
for _kind, _field, _en, _de, _what_en, _what_de in (
    ("bridge-fatigue", "bridge_fatigue_item", "Bridge fatigue check", "Ermüdungsnachweis der Brücke", "bridge fatigue check", "des Ermüdungsnachweises der Brücke"),
    ("cold-formed-member", "cold_formed_member", "Cold-formed member", "Kaltprofil-Bauteil", "cold-formed member", "des Kaltprofil-Bauteils"),
    ("crane-runway", "crane_runway", "Crane runway beam", "Kranbahnträger", "crane runway beam", "des Kranbahnträgers"),
    ("fatigue-detail", "fatigue_detail", "Fatigue detail", "Kerbdetail", "fatigue detail", "des Kerbdetails"),
    ("fire-exposure", "fire_exposure", "Fire exposure", "Brandbeanspruchung", "fire exposure", "der Brandbeanspruchung"),
    ("joint", "joint", "Joint", "Anschluss", "joint", "des Anschlusses"),
    ("load-case", "load_case", "Load case", "Lastfall", "load case", "des Lastfalls"),
    ("material", "material", "Steel material", "Stahlwerkstoff", "steel material", "des Stahlwerkstoffs"),
    ("member-action", "member_action", "Member action", "Bauteilbeanspruchung", "member action", "der Bauteilbeanspruchung"),
    ("member", "member", "Member", "Bauteil", "member", "des Bauteils"),
    ("pile", "pile", "Steel pile", "Stahlpfahl", "steel pile", "des Stahlpfahls"),
    ("plated-panel", "plated_panel", "Plated panel", "Beulfeld", "plated panel", "des Beulfelds"),
    ("section", "section", "Cross-section", "Querschnitt", "cross-section", "des Querschnitts"),
    ("silo-shell", "silo_shell", "Silo shell", "Siloschale", "silo shell", "der Siloschale"),
    ("tension-component", "tension_component", "Tension component", "Zugglied", "tension component", "des Zugglieds"),
    ("tower-leg", "tower_leg", "Tower leg", "Turmeckstiel", "tower leg", "des Turmeckstiels"),
):
    EN1993[f"insert-{_kind}"] = {"index": at_insert(), _field: rec(_en, _de, D(f"The complete {_what_en} record to insert.", f"Der vollständige einzufügende Datensatz {_what_de}."))}
    EN1993[f"remove-{_kind}"] = {"index": at_entry(_en, _de, _what_en, _what_de)}

#endregion en1993

#region en1994

BEAM1994 = at_entry("Composite beam", "Verbundträger", "composite beam", "des Verbundträgers")
COLUMN1994 = at_entry("Composite column", "Verbundstütze", "composite column", "der Verbundstütze")
SLAB1994 = at_entry("Composite slab", "Verbunddecke", "composite slab", "der Verbunddecke")
ACTION1994 = idx("Action", "Einwirkung", D("Zero-based position of the characteristic action in the member's list.", "Nullbasierte Position der charakteristischen Einwirkung in der Liste des Bauteils."))
EN1994 = {
    "change-beam-stud-count": {"index": BEAM1994, "newTotalCount": count("Number of headed studs n", "Anzahl der Kopfbolzendübel n", "shear-connection")},
    "change-beam-transverse-as": {"index": BEAM1994, "newTransverseAsM2PerM": q("Transverse reinforcement A_sf/s_f", "Querbewehrung A_sf/s_f", "m²/m", "shear-connection", disp="cm²/m", step=1e-5, prec=2, lo=0)},
    "change-column-kind": {"index": COLUMN1994, "newKind": txt("Column type", "Stützentyp", "column", D("encased, concrete_filled or partially_encased (EN 1994-1-1 Figure 6.17).", "encased (vollständig einbetoniert), concrete_filled (ausbetoniertes Hohlprofil) oder partially_encased (teilweise einbetoniert) nach EN 1994-1-1 Bild 6.17."))},
    "remove-column": {"index": COLUMN1994},
    "change-beam-stud-spacing-m": {"index": BEAM1994, "newSpacingM": q("Headed stud spacing e_L", "Dübelabstand in Längsrichtung e_L", "m", "shear-connection", disp="mm", step=0.005, prec=0, xlo=0)},
    "insert-slab": {"index": at_insert(), "slab": rec("Composite slab", "Verbunddecke", D("The complete composite slab record to insert.", "Der vollständige einzufügende Verbunddecken-Datensatz."))},
    "insert-beam": {"index": at_insert(), "beam": rec("Composite beam", "Verbundträger", D("The complete composite beam record to insert.", "Der vollständige einzufügende Verbundträger-Datensatz."))},
    "remove-slab": {"index": SLAB1994},
    "remove-beam": {"index": BEAM1994},
    "insert-column": {"index": at_insert(), "column": rec("Composite column", "Verbundstütze", D("The complete composite column record to insert.", "Der vollständige einzufügende Verbundstützen-Datensatz."))},
    "change-column-action-force-n": {"index": COLUMN1994, "actionIndex": ACTION1994, "newNKN": q("Characteristic axial force N_k", "Charakteristische Normalkraft N_k", "N", "actions", disp="kN", step=1000, prec=0)},
    "change-beam-stud-diameter-m": {"index": BEAM1994, "newDiameterM": q("Shank diameter of the headed stud d", "Schaftdurchmesser des Kopfbolzendübels d", "m", "shear-connection", disp="mm", step=0.001, prec=0, xlo=0, snaps=[0.016, 0.019, 0.022, 0.025])},
    "change-beam-action-q-area-pa": {"index": BEAM1994, "actionIndex": ACTION1994, "newQAreaPa": q("Characteristic area load q_k", "Charakteristische Flächenlast q_k", "Pa", "actions", disp="kN/m²", step=50, prec=2)},
    "change-annex": {"newAnnex": ANNEX_TEXT},
    "change-steel-fy-pa": {"newSteelFYPa": q("Yield strength of structural steel f_y", "Streckgrenze des Baustahls f_y", "Pa", "material", disp="N/mm²", step=5e6, prec=0, xlo=0, snaps=[2.35e8, 2.75e8, 3.55e8, 4.6e8])},
    "change-structure-kind": {"newStructureKind": txt("Structure type", "Tragwerksart", "annex", D("building (EN 1994-1-1) or bridge (EN 1994-2).", "building (Hochbau, EN 1994-1-1) oder bridge (Brückenbau, EN 1994-2)."))},
    "change-beam-stud-fu-pa": {"index": BEAM1994, "newFUPa": q("Ultimate tensile strength of the stud f_u", "Zugfestigkeit des Kopfbolzendübels f_u", "Pa", "shear-connection", disp="N/mm²", step=5e6, prec=0, xlo=0)},
    "change-slab-thickness-m": {"index": SLAB1994, "newConcreteThicknessM": q("Overall slab depth h", "Gesamtdicke der Verbunddecke h", "m", "slab", disp="mm", step=0.005, prec=0, lo=0)},
    "change-beam-span-m": {"index": BEAM1994, "newSpanM": q("Beam span L", "Stützweite des Trägers L", "m", "beam", step=0.05, prec=2, lo=0)},
    "change-slab-action-q-area-pa": {"index": SLAB1994, "actionIndex": ACTION1994, "newQAreaPa": q("Characteristic area load q_k", "Charakteristische Flächenlast q_k", "Pa", "actions", disp="kN/m²", step=50, prec=2)},
    "change-fatigue-detail": {"newFatigueDetail": txt("Fatigue detail category", "Kerbfall", "fatigue", D("stud_welded, shear_connector, reinforcement or flange_butt_weld.", "stud_welded (geschweißter Kopfbolzen), shear_connector (Verbundmittel), reinforcement (Bewehrung) oder flange_butt_weld (Flanschstumpfstoß)."))},
    "change-fire-rating": {"newFireRating": txt("Fire resistance class", "Feuerwiderstandsklasse", "fire", D("r30, r60, r90 or r120.", "r30, r60, r90 oder r120 (R 30 bis R 120)."))},
    "change-beam-construction": {"index": BEAM1994, "newConstruction": txt("Construction method", "Herstellungsart", "beam", D("propped or unpropped construction.", "propped (mit Montageunterstützung) oder unpropped (ohne Montageunterstützung)."))},
    "change-insulation-thickness-m": {"newInsulationThicknessM": q("Fire protection thickness d_p", "Dicke der Brandschutzbekleidung d_p", "m", "fire", disp="mm", step=0.001, prec=0, lo=0)},
    "change-beam-slab-thickness-m": {"index": BEAM1994, "newSlabThicknessM": q("Concrete flange thickness h_c", "Dicke des Betongurts h_c", "m", "beam", disp="mm", step=0.005, prec=0, lo=0)},
}

#endregion en1994

#region en1995

MEMBER1995 = ref("member", "Member", "Bauteil", D("The timber member this mutation addresses.", "Das Holzbauteil, das diese Mutation adressiert."))
CONNECTION1995 = ref("connection", "Connection", "Verbindung", D("The timber connection this mutation addresses.", "Die Holzverbindung, die diese Mutation adressiert."))
ACTION1995 = ref("action", "Action", "Einwirkung", D("The characteristic action of the member or connection.", "Die charakteristische Einwirkung des Bauteils oder der Verbindung."))
LOAD_DURATION1995 = D("permanent, long, medium, short or instantaneous (EN 1995-1-1 Table 2.1).", "permanent (ständig), long (lang), medium (mittel), short (kurz) oder instantaneous (sehr kurz) nach EN 1995-1-1 Tabelle 2.1.")
ACTION_KIND1995 = D("permanent, imposed, snow, wind or accidental.", "permanent (ständig), imposed (Nutzlast), snow (Schnee), wind oder accidental (außergewöhnlich).")
STRENGTH1995 = D("E.g. C24, GL24h, GL28c, LVL32 or CLT100 (EN 338, EN 14080).", "Z. B. C24, GL24h, GL28c, LVL32 oder CLT100 (EN 338, EN 14080).")
SERVICE1995 = D("Service class 1, 2 or 3 (EN 1995-1-1 §2.3.1.3).", "Nutzungsklasse 1, 2 oder 3 (EN 1995-1-1 §2.3.1.3).")


def mm1995(en, de, group, lo=0, xlo=None):
    return q(en, de, "m", group, disp="mm", step=0.001, prec=0, lo=None if xlo is not None else lo, xlo=xlo)


EN1995 = {
    "change-connection-diameter": {"connectionId": CONNECTION1995, "newValue": mm1995("Fastener diameter d", "Durchmesser des Verbindungsmittels d", "connection", xlo=0)},
    "change-connection-edge-distance": {"connectionId": CONNECTION1995, "newValue": mm1995("Loaded edge distance a_4,t", "Randabstand a_4,t", "connection")},
    "change-connection-end-distance": {"connectionId": CONNECTION1995, "newValue": mm1995("Loaded end distance a_3,t", "Hirnholzabstand a_3,t", "connection")},
    "change-connection-spacing": {"connectionId": CONNECTION1995, "newValue": mm1995("Spacing parallel to grain a_1", "Abstand parallel zur Faser a_1", "connection")},
    "change-connection-steel-plate-thickness": {"connectionId": CONNECTION1995, "newValue": mm1995("Steel plate thickness t_s", "Stahlblechdicke t_s", "connection")},
    "change-connection-t1": {"connectionId": CONNECTION1995, "newValue": mm1995("Timber thickness t_1", "Holzdicke t_1", "connection")},
    "change-connection-t2": {"connectionId": CONNECTION1995, "newValue": mm1995("Timber thickness t_2", "Holzdicke t_2", "connection")},
    "change-member-b": {"memberId": MEMBER1995, "newValue": mm1995("Cross-section width b", "Querschnittsbreite b", "geometry", xlo=0)},
    "change-member-bearing-length": {"memberId": MEMBER1995, "newValue": mm1995("Contact length l", "Aufstandslänge l", "geometry")},
    "change-member-buckling-y": {"memberId": MEMBER1995, "newValue": q("Buckling length l_ef,y", "Knicklänge l_ef,y", "m", "stability", step=0.05, prec=2, lo=0)},
    "change-member-buckling-z": {"memberId": MEMBER1995, "newValue": q("Buckling length l_ef,z", "Knicklänge l_ef,z", "m", "stability", step=0.05, prec=2, lo=0)},
    "change-member-lateral-restraint": {"memberId": MEMBER1995, "newValue": q("Lateral restraint spacing l_ef", "Abstand der seitlichen Halterungen l_ef", "m", "stability", step=0.05, prec=2, lo=0)},
    "change-member-notch-depth": {"memberId": MEMBER1995, "newValue": mm1995("Notch depth h_ef", "Restquerschnittshöhe an der Ausklinkung h_ef", "geometry")},
    "change-member-notch-distance": {"memberId": MEMBER1995, "newValue": mm1995("Notch distance x", "Abstand der Ausklinkung x", "geometry")},
    "change-member-span": {"memberId": MEMBER1995, "newValue": q("Span l", "Stützweite l", "m", "geometry", step=0.05, prec=2, lo=0)},
    "change-member-support-length": {"memberId": MEMBER1995, "newValue": mm1995("Support length", "Auflagerlänge", "geometry")},
    "change-member-action-vk": {"memberId": MEMBER1995, "actionId": ACTION1995, "newValue": q("Characteristic shear force V_k", "Charakteristische Querkraft V_k", "N", "actions", disp="kN", step=100, prec=1)},
    "change-member-h": {"memberId": MEMBER1995, "newValue": mm1995("Cross-section depth h", "Querschnittshöhe h", "geometry", xlo=0)},
    "change-connection-action-load-duration": {"connectionId": CONNECTION1995, "actionId": ACTION1995, "newValue": txt("Load-duration class", "Klasse der Lasteinwirkungsdauer", "actions", LOAD_DURATION1995)},
    "change-member-action-load-duration": {"memberId": MEMBER1995, "actionId": ACTION1995, "newValue": txt("Load-duration class", "Klasse der Lasteinwirkungsdauer", "actions", LOAD_DURATION1995)},
    "change-connection-action-kind": {"connectionId": CONNECTION1995, "actionId": ACTION1995, "newValue": txt("Action type", "Einwirkungsart", "actions", ACTION_KIND1995)},
    "change-member-action-kind": {"memberId": MEMBER1995, "actionId": ACTION1995, "newValue": txt("Action type", "Einwirkungsart", "actions", ACTION_KIND1995)},
    "change-member-mass-per-m": {"memberId": MEMBER1995, "newValue": q("Mass per unit length m", "Längenbezogene Masse m", "kg/m", "vibration", step=1, prec=1, lo=0)},
    "change-member-mass-per-m2": {"memberId": MEMBER1995, "newValue": q("Mass per unit area m", "Flächenbezogene Masse m", "kg/m²", "vibration", step=1, prec=1, lo=0)},
    "change-member-m-crit": {"memberId": MEMBER1995, "newValue": q("Critical bending moment M_crit", "Ideales Kippmoment M_crit", "N·m", "stability", disp="kN·m", step=100, prec=1, lo=0)},
    "insert-connection-action": {"connectionId": CONNECTION1995, "index": at_insert(), "action": rec("Action", "Einwirkung", D("The complete characteristic fastener action to insert.", "Die vollständige einzufügende charakteristische Einwirkung auf das Verbindungsmittel."))},
    "insert-connection": {"index": at_insert(), "connection": rec("Connection", "Verbindung", D("The complete timber connection record to insert.", "Der vollständige einzufügende Verbindungsdatensatz."))},
    "insert-member-action": {"memberId": MEMBER1995, "index": at_insert(), "action": rec("Action", "Einwirkung", D("The complete characteristic action to insert.", "Die vollständige einzufügende charakteristische Einwirkung."))},
    "insert-member": {"index": at_insert(), "member": rec("Member", "Bauteil", D("The complete timber member record to insert.", "Der vollständige einzufügende Holzbauteil-Datensatz."))},
    "remove-connection-action": {"connectionId": CONNECTION1995, "index": at_entry("Action", "Einwirkung", "action within the connection", "der Einwirkung in der Verbindung")},
    "remove-connection": {"index": at_entry("Connection", "Verbindung", "connection", "der Verbindung")},
    "remove-member-action": {"memberId": MEMBER1995, "index": at_entry("Action", "Einwirkung", "action within the member", "der Einwirkung im Bauteil")},
    "remove-member": {"index": at_entry("Member", "Bauteil", "member", "des Bauteils")},
    "change-member-action-mk": {"memberId": MEMBER1995, "actionId": ACTION1995, "newValue": q("Characteristic bending moment M_k", "Charakteristisches Biegemoment M_k", "N·m", "actions", disp="kN·m", step=100, prec=1)},
    "change-member-action-f-point": {"memberId": MEMBER1995, "actionId": ACTION1995, "newValue": q("Characteristic point load F_k", "Charakteristische Einzellast F_k", "N", "actions", disp="kN", step=100, prec=1)},
    "change-member-action-q-line": {"memberId": MEMBER1995, "actionId": ACTION1995, "newValue": q("Characteristic line load q_k", "Charakteristische Streckenlast q_k", "N/m", "actions", disp="kN/m", step=100, prec=2)},
    "change-member-bridge-a": {"memberId": MEMBER1995, "newValue": factor("Fatigue parameter a", "Ermüdungsparameter a", "fatigue", desc=D("Coefficient a of EN 1995-2 Annex A.", "Beiwert a nach EN 1995-2 Anhang A."))},
    "change-member-bridge-b": {"memberId": MEMBER1995, "newValue": factor("Fatigue parameter b", "Ermüdungsparameter b", "fatigue", desc=D("Coefficient b of EN 1995-2 Annex A.", "Beiwert b nach EN 1995-2 Anhang A."))},
    "change-member-bridge-beta": {"memberId": MEMBER1995, "newValue": factor("Fatigue coefficient β", "Ermüdungsbeiwert β", "fatigue", desc=D("Damage consequence factor β of EN 1995-2 Annex A.", "Beiwert β für die Schadensfolgen nach EN 1995-2 Anhang A."))},
    "change-member-bridge-n-obs": {"memberId": MEMBER1995, "newValue": q("Annual number of constant amplitude stress cycles N_obs", "Jährliche Anzahl der Spannungswechsel N_obs", "1/a", "fatigue", step=1000, prec=0, lo=0)},
    "change-member-bridge-tl-years": {"memberId": MEMBER1995, "newValue": q("Design service life t_L", "Nutzungsdauer t_L", "a", "fatigue", step=1, prec=0, xlo=0)},
    "change-member-damping": {"memberId": MEMBER1995, "newValue": q("Modal damping ratio ζ", "Modaler Dämpfungsgrad ζ", "1", "vibration", disp="%", step=0.001, prec=1, lo=0, hi=1)},
    "change-annex": {"newAnnex": ANNEX_TEXT},
    "change-connection-service-class": {"connectionId": CONNECTION1995, "newValue": count("Service class", "Nutzungsklasse", "material", SERVICE1995, lo=1, hi=3)},
    "change-member-service-class": {"memberId": MEMBER1995, "newValue": count("Service class", "Nutzungsklasse", "material", SERVICE1995, lo=1, hi=3)},
    "change-member-role": {"memberId": MEMBER1995, "newValue": sel("Structural role", "Tragwerksrolle", "geometry", {"beam": ("Beam — bending member", "Träger — Biegebauteil"), "column": ("Column — compression member", "Stütze — Druckbauteil"), "floor": ("Floor — joist or panel with vibration check", "Decke — Balken oder Platte mit Schwingungsnachweis"), "bridge": ("Bridge — EN 1995-2 girder", "Brücke — Träger nach EN 1995-2")}, widget="segmented")},
    "change-member-action-fc90-k": {"memberId": MEMBER1995, "actionId": ACTION1995, "newValue": q("Characteristic compressive force perpendicular to grain F_c,90,k", "Charakteristische Querdruckkraft F_c,90,k", "N", "actions", disp="kN", step=100, prec=1)},
    "change-member-action-nk": {"memberId": MEMBER1995, "actionId": ACTION1995, "newValue": q("Characteristic compressive force N_c,k", "Charakteristische Druckkraft N_c,k", "N", "actions", disp="kN", step=100, prec=1)},
    "change-member-action-ntk": {"memberId": MEMBER1995, "actionId": ACTION1995, "newValue": q("Characteristic tensile force N_t,k", "Charakteristische Zugkraft N_t,k", "N", "actions", disp="kN", step=100, prec=1)},
    "change-member-action-category": {"memberId": MEMBER1995, "actionId": ACTION1995, "newValue": txt("Imposed load category", "Nutzungskategorie", "actions", D("A to H (EN 1991-1-1 Table 6.1); empty when the action is no imposed load.", "A bis H (EN 1991-1-1 Tabelle 6.1); leer, wenn die Einwirkung keine Nutzlast ist."))},
    "change-connection-label-de": {"connectionId": CONNECTION1995, "newValue": txt("German label", "Deutsche Bezeichnung", "labels")},
    "change-connection-label-en": {"connectionId": CONNECTION1995, "newValue": txt("English label", "Englische Bezeichnung", "labels")},
    "change-member-label-de": {"memberId": MEMBER1995, "newValue": txt("German label", "Deutsche Bezeichnung", "labels")},
    "change-member-label-en": {"memberId": MEMBER1995, "newValue": txt("English label", "Englische Bezeichnung", "labels")},
    "change-member-support": {"memberId": MEMBER1995, "newValue": sel("Support conditions", "Lagerungsart", "geometry", {"simplySupported": ("Simply supported", "Einfeldträger"), "cantilever": ("Cantilever", "Kragträger"), "continuousTwoSpan": ("Continuous over two spans", "Zweifeldträger")}, widget="segmented")},
    "change-connection-number": {"connectionId": CONNECTION1995, "newValue": count("Number of fasteners n", "Anzahl der Verbindungsmittel n", "connection")},
    "change-connection-rows": {"connectionId": CONNECTION1995, "newValue": count("Number of rows", "Anzahl der Reihen", "connection")},
    "change-connection-shear-planes": {"connectionId": CONNECTION1995, "newValue": count("Number of shear planes", "Anzahl der Scherfugen", "connection")},
    "change-member-fire-duration": {"memberId": MEMBER1995, "newValue": q("Fire resistance duration t", "Feuerwiderstandsdauer t", "s", "fire", disp="min", step=60, prec=0, lo=0)},
    "change-connection-action-fk": {"connectionId": CONNECTION1995, "actionId": ACTION1995, "newValue": q("Characteristic fastener force F_k", "Charakteristische Kraft je Verbindung F_k", "N", "actions", disp="kN", step=100, prec=1)},
    "change-connection-fastener-type": {"connectionId": CONNECTION1995, "newValue": txt("Fastener type", "Art des Verbindungsmittels", "connection", D("nail, screw, bolt or dowel.", "nail (Nagel), screw (Schraube), bolt (Bolzen) oder dowel (Stabdübel)."))},
    "change-connection-steel-plate": {"connectionId": CONNECTION1995, "newValue": tog("Steel-to-timber connection", "Stahlblech-Holz-Verbindung", "connection")},
    "change-member-bridge-crowd": {"memberId": MEMBER1995, "newValue": q("Pedestrian crowd density", "Fußgängerdichte", "1/m²", "vibration", step=0.1, prec=1, lo=0)},
    "change-connection-fuk": {"connectionId": CONNECTION1995, "newValue": q("Characteristic tensile strength of the fastener f_u,k", "Charakteristische Zugfestigkeit des Verbindungsmittels f_u,k", "Pa", "connection", disp="N/mm²", step=1e7, prec=0, xlo=0)},
    "change-connection-strength-class": {"connectionId": CONNECTION1995, "newValue": txt("Strength class", "Festigkeitsklasse", "material", STRENGTH1995)},
    "change-member-strength-class": {"memberId": MEMBER1995, "newValue": txt("Strength class", "Festigkeitsklasse", "material", STRENGTH1995)},
}

#endregion en1995

#region en1996

WALL1996 = at_entry("Wall", "Wand", "wall", "der Wand")
LOADCASE1996 = at_entry("Load case", "Lastfall", "load case within the wall", "des Lastfalls in der Wand")
CONCENTRATED1996 = at_entry("Concentrated load", "Einzellast", "concentrated load within the load case", "der Einzellast im Lastfall")
OPENING1996 = at_entry("Opening", "Öffnung", "opening within the wall", "der Öffnung in der Wand")
DESIGN_SITUATION1996 = D("Persistent, Transient, Accidental or Seismic (EN 1990 §3.2).", "Persistent (ständig), Transient (vorübergehend), Accidental (außergewöhnlich) oder Seismic (Erdbeben) nach EN 1990 §3.2.")


def wall_mm(en, de, group, xlo=None):
    return q(en, de, "m", group, disp="mm", step=0.005, prec=0, lo=None if xlo is not None else 0, xlo=xlo)


EN1996 = {
    "change-concentrated-bearing-length": {"wallIndex": WALL1996, "loadCaseIndex": LOADCASE1996, "index": CONCENTRATED1996, "newBearingLengthM": wall_mm("Bearing length of the concentrated load", "Auflagerlänge der Einzellast", "actions")},
    "change-slab-span": {"wallIndex": WALL1996, "index": LOADCASE1996, "newSlabSpanM": q("Clear span of the slab l_f", "Lichte Deckenstützweite l_f", "m", "actions", step=0.05, prec=2, lo=0)},
    "change-wall-length": {"index": WALL1996, "newLengthM": q("Wall length l", "Wandlänge l", "m", "geometry", step=0.05, prec=2, lo=0)},
    "change-wall-height": {"index": WALL1996, "newHeightM": q("Clear wall height h", "Lichte Wandhöhe h", "m", "geometry", step=0.05, prec=2, lo=0)},
    "change-wall-thickness": {"index": WALL1996, "newThicknessM": wall_mm("Wall thickness t", "Wanddicke t", "geometry", xlo=0)},
    "change-eccentricity-bottom": {"index": WALL1996, "newEccentricityBottomM": q("Eccentricity at the wall base e_u", "Lastausmitte am Wandfuß e_u", "m", "geometry", disp="mm", step=0.001, prec=0)},
    "change-eccentricity-top": {"index": WALL1996, "newEccentricityTopM": q("Eccentricity at the wall head e_o", "Lastausmitte am Wandkopf e_o", "m", "geometry", disp="mm", step=0.001, prec=0)},
    "change-phi-infinity": {"index": WALL1996, "newPhiInfinity": factor("Final creep coefficient φ_∞", "Endkriechzahl φ_∞", "material", lo=0)},
    "change-qk-snow": {"wallIndex": WALL1996, "index": LOADCASE1996, "newQKSnowPa": q("Characteristic snow load s_k", "Charakteristische Schneelast s_k", "Pa", "actions", disp="kN/m²", step=50, prec=2, lo=0)},
    "insert-concentrated": {"wallIndex": WALL1996, "loadCaseIndex": LOADCASE1996, "index": at_insert(), "load": rec("Concentrated load", "Einzellast", D("The complete concentrated load record to insert.", "Der vollständige einzufügende Einzellast-Datensatz."))},
    "insert-load-case": {"wallIndex": WALL1996, "index": at_insert(), "loadCase": rec("Load case", "Lastfall", D("The complete load case record to insert into the wall.", "Der vollständige in die Wand einzufügende Lastfall-Datensatz."))},
    "insert-opening": {"wallIndex": WALL1996, "index": at_insert(), "opening": rec("Opening", "Öffnung", D("The complete opening record to insert into the wall.", "Der vollständige in die Wand einzufügende Öffnungsdatensatz."))},
    "insert-wall": {"index": at_insert(), "wall": rec("Wall", "Wand", D("The complete masonry wall record to insert.", "Der vollständige einzufügende Mauerwerkswand-Datensatz."))},
    "remove-concentrated": {"wallIndex": WALL1996, "loadCaseIndex": LOADCASE1996, "index": CONCENTRATED1996},
    "remove-load-case": {"wallIndex": WALL1996, "index": LOADCASE1996},
    "remove-opening": {"wallIndex": WALL1996, "index": OPENING1996},
    "remove-wall": {"index": WALL1996},
    "change-annex": {"newAnnex": ANNEX_TEXT},
    "change-qp-wind": {"wallIndex": WALL1996, "index": LOADCASE1996, "newQPWindPa": q("Peak velocity pressure q_p", "Böengeschwindigkeitsdruck q_p", "Pa", "actions", disp="kN/m²", step=50, prec=2, lo=0)},
    "change-design-situation": {"newDesignSituation": txt("Design situation", "Bemessungssituation", "annex", DESIGN_SITUATION1996)},
    "change-load-case-situation": {"wallIndex": WALL1996, "loadCaseIndex": LOADCASE1996, "newDesignSituation": txt("Design situation of the load case", "Bemessungssituation des Lastfalls", "actions", DESIGN_SITUATION1996)},
    "change-concentrated-force": {"wallIndex": WALL1996, "loadCaseIndex": LOADCASE1996, "index": CONCENTRATED1996, "newForceN": q("Characteristic concentrated force F_k", "Charakteristische Einzellast F_k", "N", "actions", disp="kN", step=1000, prec=1)},
    "change-gk-slab": {"wallIndex": WALL1996, "index": LOADCASE1996, "newGKSlabN": q("Characteristic permanent slab load G_k", "Charakteristische ständige Deckenauflast G_k", "N", "actions", disp="kN", step=1000, prec=1, lo=0)},
    "change-qk-imposed": {"wallIndex": WALL1996, "index": LOADCASE1996, "newQKImposedPa": q("Characteristic imposed load q_k", "Charakteristische Nutzlast q_k", "Pa", "actions", disp="kN/m²", step=50, prec=2, lo=0)},
    "change-is-basement": {"index": WALL1996, "newIsBasement": tog("Basement wall", "Kellerwand", "geometry", D("Checked for earth pressure per EN 1996-3/NA.", "Nachweis für Erddruck nach EN 1996-3/NA."))},
    "change-storeys": {"newStoreys": count("Number of storeys", "Geschossanzahl", "annex")},
    "change-masonry-class": {"newMasonryClass": txt("Masonry execution class", "Mauerwerksklasse", "annex", D("Class1 to Class5: category of manufacturing control and execution, sets γ_M (EN 1996-1-1 Table 2.3).", "Class1 bis Class5: Kategorie der Herstellungskontrolle und Ausführung, bestimmt γ_M (EN 1996-1-1 Tabelle 2.3)."))},
    "change-imposed-category": {"wallIndex": WALL1996, "index": LOADCASE1996, "newImposedCategory": txt("Imposed load category", "Nutzungskategorie", "actions", D("A to H per EN 1991-1-1 Table 6.1; sets the ψ factors.", "A bis H nach EN 1991-1-1 Tabelle 6.1; bestimmt die ψ-Beiwerte."))},
    "change-wall-label-de": {"index": WALL1996, "newLabelDe": txt("German label", "Deutsche Bezeichnung", "labels")},
    "change-wall-label-en": {"index": WALL1996, "newLabelEn": txt("English label", "Englische Bezeichnung", "labels")},
    "change-exposure": {"index": WALL1996, "newExposure": txt("Exposure class", "Expositionsklasse", "durability", D("Mx1 to Mx5 (EN 1996-2 Annex A).", "Mx1 bis Mx5 (EN 1996-2 Anhang A)."))},
    "change-concentrated-bearing-area": {"wallIndex": WALL1996, "loadCaseIndex": LOADCASE1996, "index": CONCENTRATED1996, "newBearingAreaM2": q("Loaded area A_b", "Belastete Fläche A_b", "m²", "actions", disp="cm²", step=0.0001, prec=0, lo=0)},
    "change-slab-bearing-depth": {"index": WALL1996, "newSlabBearingDepthM": wall_mm("Slab bearing depth a", "Deckenauflagertiefe a", "geometry")},
    "change-tributary-area": {"wallIndex": WALL1996, "index": LOADCASE1996, "newTributaryAreaM2": q("Tributary floor area", "Lasteinzugsfläche", "m²", "actions", step=0.5, prec=2, lo=0)},
    "change-fire-rei": {"index": WALL1996, "newFireReiMin": count("Required fire resistance REI", "Erforderliche Feuerwiderstandsdauer REI", "fire", D("Fire resistance in minutes, e.g. 30, 60, 90 (EN 1996-1-2).", "Feuerwiderstandsdauer in Minuten, z. B. 30, 60, 90 (EN 1996-1-2)."), unit="min")},
    "change-as-horizontal": {"index": WALL1996, "newAsHorizontalM2": q("Bed joint reinforcement area A_s", "Querschnitt der Lagerfugenbewehrung A_s", "m²", "reinforcement", disp="cm²", step=1e-6, prec=2, lo=0)},
    "change-as-vertical": {"index": WALL1996, "newAsVerticalM2": q("Vertical reinforcement area A_s", "Querschnitt der vertikalen Bewehrung A_s", "m²", "reinforcement", disp="cm²", step=1e-6, prec=2, lo=0)},
    "change-f-yd": {"index": WALL1996, "newFYdPa": q("Design yield strength of reinforcement f_yd", "Bemessungswert der Streckgrenze der Bewehrung f_yd", "Pa", "reinforcement", disp="N/mm²", step=5e6, prec=0, lo=0)},
    "change-reinforced": {"index": WALL1996, "newReinforced": tog("Reinforced masonry", "Bewehrtes Mauerwerk", "reinforcement")},
    "change-bed-joint-thickness": {"index": WALL1996, "newBedJointThicknessM": q("Bed joint thickness", "Lagerfugendicke", "m", "material", disp="mm", step=0.001, prec=0, lo=0)},
    "change-fm": {"index": WALL1996, "newMortarStrengthPa": q("Mortar compressive strength f_m", "Mörteldruckfestigkeit f_m", "Pa", "material", disp="N/mm²", step=5e5, prec=1, lo=0)},
    "change-mortar-class": {"index": WALL1996, "newMortarClass": txt("Mortar strength class", "Mörtelklasse", "material", D("M1, M2_5, M5, M10, M15 or M20 (f_m in N/mm²).", "M1, M2_5, M5, M10, M15 oder M20 (f_m in N/mm²)."))},
    "change-mortar-type": {"index": WALL1996, "newMortarType": txt("Mortar type", "Mörtelart", "material", D("GeneralPurpose, ThinLayer or Lightweight.", "GeneralPurpose (Normalmauermörtel), ThinLayer (Dünnbettmörtel) oder Lightweight (Leichtmauermörtel)."))},
    "change-c-pe": {"wallIndex": WALL1996, "index": LOADCASE1996, "newCPe": factor("External pressure coefficient c_pe", "Außendruckbeiwert c_pe", "actions")},
    "change-density": {"index": WALL1996, "newDensityKgM3": q("Masonry density ρ", "Rohdichte des Mauerwerks ρ", "kg/m³", "material", step=10, prec=0, lo=0)},
    "change-support-sides": {"index": WALL1996, "newSupportSides": count("Number of supported edges", "Anzahl der gehaltenen Ränder", "geometry", D("Two-, three- or four-sided support (EN 1996-1-1 §5.5.1.2).", "Zwei-, drei- oder vierseitig gehalten (EN 1996-1-1 §5.5.1.2)."), lo=2, hi=4)},
    "change-unit-fb": {"index": WALL1996, "newFBPa": q("Normalised compressive strength of the unit f_b", "Normierte Steindruckfestigkeit f_b", "Pa", "material", disp="N/mm²", step=5e5, prec=1, xlo=0)},
    "change-unit-group": {"index": WALL1996, "newUnitGroup": txt("Masonry unit group", "Steingruppe", "material", D("Group1 to Group4 (EN 1996-1-1 Table 3.1).", "Group1 bis Group4 (EN 1996-1-1 Tabelle 3.1)."))},
    "change-unit-height": {"index": WALL1996, "newUnitHeightM": wall_mm("Unit height", "Steinhöhe", "material", xlo=0)},
    "change-unit-length": {"index": WALL1996, "newUnitLengthM": wall_mm("Unit length", "Steinlänge", "material", xlo=0)},
    "change-unit-material": {"index": WALL1996, "newUnitMaterial": txt("Unit material", "Steinart", "material", D("Clay, CalciumSilicate, Aerated or Concrete.", "Clay (Mauerziegel), CalciumSilicate (Kalksandstein), Aerated (Porenbeton) oder Concrete (Betonstein)."))},
    "change-unit-width": {"index": WALL1996, "newUnitWidthM": wall_mm("Unit width", "Steinbreite", "material", xlo=0)},
    "change-wall-type": {"index": WALL1996, "newWallType": txt("Wall type", "Wandart", "geometry", D("LoadBearing, Shear or NonLoadBearing.", "LoadBearing (tragend), Shear (aussteifend) oder NonLoadBearing (nichttragend)."))},
    "change-mu": {"index": WALL1996, "newMu": factor("Friction coefficient μ", "Reibungsbeiwert μ", "material", lo=0)},
    "change-opening-height": {"wallIndex": WALL1996, "index": OPENING1996, "newHeightM": q("Opening height", "Öffnungshöhe", "m", "openings", step=0.05, prec=2, lo=0)},
    "change-opening-sill": {"wallIndex": WALL1996, "index": OPENING1996, "newSillHeightM": q("Sill height", "Brüstungshöhe", "m", "openings", step=0.05, prec=2, lo=0)},
    "change-opening-width": {"wallIndex": WALL1996, "index": OPENING1996, "newWidthM": q("Opening width", "Öffnungsbreite", "m", "openings", step=0.05, prec=2, lo=0)},
    "change-hk-earth": {"wallIndex": WALL1996, "index": LOADCASE1996, "newHKEarthN": q("Characteristic earth pressure resultant H_k", "Charakteristische Erddruckresultierende H_k", "N", "actions", disp="kN", step=1000, prec=1, lo=0)},
}

#endregion en1996

#region en1997

FOOTING1997 = ref("footing", "Spread foundation", "Flachgründung", D("The spread foundation this mutation addresses.", "Die Flachgründung, die diese Mutation adressiert."))
LAYER1997 = ref("soilLayer", "Soil layer", "Bodenschicht", D("The soil layer this mutation addresses.", "Die Bodenschicht, die diese Mutation adressiert."))
PILE1997 = ref("pile", "Pile", "Pfahl", D("The pile group this mutation addresses.", "Die Pfahlgruppe, die diese Mutation adressiert."))
EN1997 = {
    "change-footing-width": {"id": FOOTING1997, "newWidth": q("Foundation width b", "Fundamentbreite b", "m", "foundation", step=0.05, prec=2, xlo=0)},
    "change-slope-angle": {"id": ref("slope", "Slope", "Böschung", D("The slope this mutation addresses.", "Die Böschung, die diese Mutation adressiert.")), "newAngleDeg": q("Slope angle β", "Böschungswinkel β", "°", "slope", step=0.5, prec=1, lo=0, hi=90, widget="dial")},
    "insert-footing": {"index": at_insert(), "footing": rec("Spread foundation", "Flachgründung", D("The complete spread foundation record to insert.", "Der vollständige einzufügende Datensatz der Flachgründung."))},
    "insert-layer": {"index": at_insert(), "layer": rec("Soil layer", "Bodenschicht", D("The complete soil layer record to insert, top to bottom.", "Der vollständige einzufügende Bodenschicht-Datensatz, von oben nach unten."))},
    "remove-footing": {"index": at_entry("Spread foundation", "Flachgründung", "spread foundation", "der Flachgründung")},
    "remove-layer": {"index": at_entry("Soil layer", "Bodenschicht", "soil layer", "der Bodenschicht")},
    "change-footing-embedment": {"id": FOOTING1997, "newEmbedment": q("Embedment depth d", "Einbindetiefe d", "m", "foundation", step=0.05, prec=2, lo=0)},
    "change-layer-oedometric-modulus": {"id": LAYER1997, "newOedometricModulus": q("Oedometer modulus E_oed", "Steifemodul E_s", "Pa", "soil", disp="N/mm²", step=1e6, prec=1, xlo=0)},
    "change-annex": {"newAnnex": ANNEX_TEXT},
    "change-groundwater-level": {"newGroundwaterLevel": q("Groundwater level", "Grundwasserstand", "m", "site", step=0.1, prec=2, desc=D("Depth of the design groundwater table below ground level.", "Tiefe des Bemessungswasserstands unter Geländeoberkante."))},
    "change-design-situation": {"newDesignSituation": txt("Design situation", "Bemessungssituation", "annex", D("DIN 1054 design situation bsP, bsT or bsA (BS-P persistent, BS-T transient, BS-A accidental).", "Bemessungssituation nach DIN 1054: bsP (BS-P ständig), bsT (BS-T vorübergehend) oder bsA (BS-A außergewöhnlich)."))},
    "change-pile-length": {"id": PILE1997, "newLength": q("Pile length L", "Pfahllänge L", "m", "piles", step=0.5, prec=2, xlo=0)},
    "change-layer-phi-prime": {"id": LAYER1997, "newPhiPrimeDeg": q("Effective angle of shearing resistance φ′", "Effektiver Reibungswinkel φ′", "°", "soil", step=0.5, prec=1, lo=0, hi=90, widget="dial")},
    "remove-pile": {"index": at_entry("Pile", "Pfahl", "pile group", "der Pfahlgruppe")},
    "insert-pile": {"index": at_insert(), "pile": rec("Pile", "Pfahl", D("The complete pile group record to insert.", "Der vollständige einzufügende Pfahlgruppen-Datensatz."))},
    "change-investigation-depth": {"newInvestigationDepth": q("Ground investigation depth z_a", "Erkundungstiefe z_a", "m", "site", step=0.5, prec=1, lo=0)},
    "change-pile-count": {"id": PILE1997, "newCount": count("Number of piles n", "Pfahlanzahl n", "piles", lo=1)},
    "change-geotechnical-category": {"newGeotechnicalCategory": count("Geotechnical category", "Geotechnische Kategorie", "annex", D("GK 1, 2 or 3 (EN 1997-1 §2.1, DIN 1054).", "GK 1, 2 oder 3 (EN 1997-1 §2.1, DIN 1054)."), lo=1, hi=3)},
    "change-design-approach": {"newDesignApproach": txt("Design approach", "Nachweisverfahren", "annex", D("da1, da2 or da3 (EN 1997-1 §2.4.7.3.4); Germany uses da2 (DIN 1054).", "da1, da2 oder da3 (EN 1997-1 §2.4.7.3.4); in Deutschland gilt da2 (DIN 1054)."))},
    "change-wall-base-width": {"id": ref("retainingWall", "Retaining wall", "Stützwand", D("The retaining wall this mutation addresses.", "Die Stützwand, die diese Mutation adressiert.")), "newBaseWidth": q("Base width b", "Sohlbreite b", "m", "retaining-wall", step=0.05, prec=2, xlo=0)},
}

#endregion en1997

#region en1998

BUILDING1998 = at_entry("Building", "Gebäude", "building", "des Gebäudes")
STOREY1998 = at_entry("Storey", "Geschoss", "storey within the building", "des Geschosses im Gebäude")
EN1998 = {
    "change-tower-m-rd-nm": {"index": at_entry("Tower", "Turm", "tower", "des Turms"), "newMRdNm": q("Design moment resistance M_Rd", "Bemessungswert der Biegetragfähigkeit M_Rd", "N·m", "resistance", disp="kN·m", step=1000, prec=0, lo=0)},
    "change-storey-permanent-gk-n": {"buildingIndex": BUILDING1998, "storeyIndex": STOREY1998, "newPermanentGkN": q("Permanent action of the storey ΣG_k", "Ständige Einwirkung des Geschosses ΣG_k", "N", "masses", disp="kN", step=1000, prec=0, lo=0)},
    "change-member-detailing-compatible": {"buildingIndex": BUILDING1998, "memberIndex": at_entry("Member", "Bauteil", "member within the building", "des Bauteils im Gebäude"), "newDetailingCompatibleWithQ": tog("Detailing compatible with q", "Konstruktive Durchbildung passend zu q", "ductility", D("Whether the member detailing meets the ductility class assumed for the behaviour factor q.", "Ob die konstruktive Durchbildung des Bauteils der für den Verhaltensbeiwert q angesetzten Duktilitätsklasse entspricht."))},
    "insert-building": {"index": at_insert(), "building": rec("Building", "Gebäude", D("The complete building record (EN 1998-1) to insert.", "Der vollständige einzufügende Gebäudedatensatz (EN 1998-1)."))},
    "remove-building": {"index": BUILDING1998},
    "insert-bridge": {"index": at_insert(), "bridge": rec("Bridge", "Brücke", D("The complete bridge record (EN 1998-2) to insert.", "Der vollständige einzufügende Brückendatensatz (EN 1998-2)."))},
    "update-site": {"site": rec("Site seismicity", "Standortseismizität", D("Seismic zone, ground conditions, spectrum type and importance class.", "Erdbebenzone, Untergrundverhältnisse, Spektrumtyp und Bedeutungskategorie."), group="site")},
    "change-assessment-rkn": {"index": at_entry("Assessed element", "Bewertetes Bauteil", "assessed element", "des bewerteten Bauteils"), "newRKN": q("Characteristic resistance R_k", "Charakteristischer Widerstand R_k", "N", "resistance", disp="kN", step=1000, prec=0, lo=0)},
    "change-system-base-shear-resistance-n": {"buildingIndex": BUILDING1998, "systemIndex": at_entry("Structural system", "Tragsystem", "structural system within the building", "des Tragsystems im Gebäude"), "newBaseShearResistanceN": q("Base shear resistance V_Rd", "Widerstand gegen die Gesamterdbebenkraft V_Rd", "N", "resistance", disp="kN", step=1000, prec=0, lo=0)},
    "change-annex": {"newAnnex": txt("National annex", "Nationaler Anhang", "annex", D("de applies DIN EN 1998-1/NA (seismic zones), en the recommended CEN values.", "de wendet DIN EN 1998-1/NA an (Erdbebenzonen), en die empfohlenen CEN-Werte."))},
    "change-building-elevation-regular": {"buildingIndex": BUILDING1998, "newElevationRegular": tog("Regular in elevation", "Regelmäßig im Aufriss", "regularity", D("Criteria of EN 1998-1 §4.2.3.3.", "Kriterien nach EN 1998-1 §4.2.3.3."))},
    "change-storey-drift-xm": {"buildingIndex": BUILDING1998, "storeyIndex": STOREY1998, "newDriftXM": q("Design interstorey drift d_r,x", "Bemessungswert der Stockwerksverschiebung d_r,x", "m", "damage-limitation", disp="mm", step=0.001, prec=1)},
    "change-storey-stiffness-x": {"buildingIndex": BUILDING1998, "storeyIndex": STOREY1998, "newStiffnessX": q("Storey stiffness in x", "Geschosssteifigkeit in x-Richtung", "N/m", "regularity", step=1e6, prec=0, lo=0)},
    "insert-assessment": {"index": at_insert(), "assessment": rec("Assessed element", "Bewertetes Bauteil", D("The complete existing-structure assessment record (EN 1998-3) to insert.", "Der vollständige einzufügende Datensatz zur Bewertung des Bestands (EN 1998-3)."))},
    "insert-tower": {"index": at_insert(), "tower": rec("Tower", "Turm", D("The complete tower, mast or chimney record (EN 1998-6) to insert.", "Der vollständige einzufügende Datensatz für Turm, Mast oder Schornstein (EN 1998-6)."))},
    "change-bridge-v-rd-n": {"index": at_entry("Bridge", "Brücke", "bridge", "der Brücke"), "newVRdN": q("Design shear resistance V_Rd", "Bemessungswert der Querkrafttragfähigkeit V_Rd", "N", "resistance", disp="kN", step=1000, prec=0, lo=0)},
    "insert-tank": {"index": at_insert(), "tank": rec("Tank", "Tank", D("The complete tank record (EN 1998-4) to insert.", "Der vollständige einzufügende Tankdatensatz (EN 1998-4)."))},
    "change-building-plan-regular": {"buildingIndex": BUILDING1998, "newPlanRegular": tog("Regular in plan", "Regelmäßig im Grundriss", "regularity", D("Criteria of EN 1998-1 §4.2.3.2.", "Kriterien nach EN 1998-1 §4.2.3.2."))},
    "change-building-masonry-wall-area-ratio": {"buildingIndex": BUILDING1998, "newMasonryWallAreaRatio": factor("Shear wall area ratio", "Wandflächenanteil der Schubwände", "masonry", D("Cross-sectional area of the shear walls per direction relative to the floor area (EN 1998-1 Table 9.3).", "Querschnittsfläche der Schubwände je Richtung bezogen auf die Geschossfläche (EN 1998-1 Tabelle 9.3)."), step=0.001, prec=3, lo=0)},
    "insert-retaining-wall": {"index": at_insert(), "wall": rec("Retaining wall", "Stützwand", D("The complete retaining wall record (EN 1998-5) to insert.", "Der vollständige einzufügende Stützwand-Datensatz (EN 1998-5)."))},
    "insert-foundation": {"index": at_insert(), "foundation": rec("Foundation", "Gründung", D("The complete foundation record (EN 1998-5) to insert.", "Der vollständige einzufügende Gründungsdatensatz (EN 1998-5)."))},
    "insert-silo": {"index": at_insert(), "silo": rec("Silo", "Silo", D("The complete silo record (EN 1998-4) to insert.", "Der vollständige einzufügende Silodatensatz (EN 1998-4)."))},
}

#endregion en1998

#region en1999

MEMBER1999 = ref("member", "Member", "Bauteil", D("The aluminium member this mutation addresses.", "Das Aluminiumbauteil, das diese Mutation adressiert."))
ACTION1999 = ref("action", "Load case", "Lastfall", D("The characteristic action (load case) of the member.", "Die charakteristische Einwirkung (Lastfall) des Bauteils."))
CONNECTION1999 = ref("connection", "Connection", "Verbindung", D("The connection this mutation addresses.", "Die Verbindung, die diese Mutation adressiert."))
EN1999 = {
    "change-material-designation": {"materialId": ref("material", "Material", "Werkstoff", D("The aluminium material this mutation addresses.", "Der Aluminiumwerkstoff, den diese Mutation adressiert.")), "newDesignation": txt("Alloy and temper", "Legierung und Werkstoffzustand", "material", D("EN 573/EN 755 designation, e.g. aw6060-t6, aw6082-t6 or aw5083-o.", "Bezeichnung nach EN 573/EN 755, z. B. aw6060-t6, aw6082-t6 oder aw5083-o."))},
    "change-cold-formed": {"coldFormed": rec("Cold-formed sheeting", "Kaltprofilierte Bleche", D("The complete list of cold-formed sheeting (EN 1999-1-4).", "Die vollständige Liste der kaltprofilierten Bleche (EN 1999-1-4)."))},
    "add-member": {"index": at_insert(), "member": rec("Member", "Bauteil", D("The complete aluminium member record to add.", "Der vollständige hinzuzufügende Aluminiumbauteil-Datensatz."))},
    "remove-member": {"id": MEMBER1999},
    "change-member-my-ed": {"memberId": MEMBER1999, "loadCaseId": ACTION1999, "newMYEd": q("Characteristic bending moment M_y,k", "Charakteristisches Biegemoment M_y,k", "N·m", "actions", disp="kN·m", step=100, prec=1)},
    "change-annex": {"newAnnex": ANNEX_TEXT},
    "change-member-n-ed": {"memberId": MEMBER1999, "loadCaseId": ACTION1999, "newNEd": q("Characteristic axial force N_k", "Charakteristische Normalkraft N_k", "N", "actions", disp="kN", step=100, prec=1)},
    "change-members": {"members": rec("Members", "Bauteile", D("The complete list of aluminium members.", "Die vollständige Liste der Aluminiumbauteile."))},
    "change-member-buckling-length": {"memberId": MEMBER1999, "axis": txt("Buckling axis", "Knickachse", "stability", D("y, z, t (torsional) or ltb (lateral-torsional); anything else sets y.", "y, z, t (Drillknicken) oder ltb (Biegedrillknicken); jeder andere Wert setzt y.")), "newLength": q("Buckling length L_cr", "Knicklänge L_cr", "m", "stability", step=0.05, prec=2, lo=0)},
    "change-sections": {"sections": rec("Cross-sections", "Querschnitte", D("The complete list of aluminium cross-sections.", "Die vollständige Liste der Aluminiumquerschnitte."))},
    "change-fatigue-details": {"fatigueDetails": rec("Fatigue detail categories", "Kerbfälle", D("The complete list of fatigue details (EN 1999-1-3).", "Die vollständige Liste der Kerbfälle (EN 1999-1-3)."))},
    "change-connections": {"connections": rec("Connections", "Verbindungen", D("The complete list of bolted and welded connections.", "Die vollständige Liste der Schrauben- und Schweißverbindungen."))},
    "change-fire-scenarios": {"fireScenarios": rec("Fire scenarios", "Brandszenarien", D("The complete list of fire scenarios (EN 1999-1-2).", "Die vollständige Liste der Brandszenarien (EN 1999-1-2)."))},
    "change-weld-throat": {"connectionId": CONNECTION1999, "newThroat": q("Weld throat thickness a", "Kehlnahtdicke a", "m", "welds", disp="mm", step=0.0005, prec=1, xlo=0)},
    "change-bolt-count": {"connectionId": CONNECTION1999, "newRows": count("Number of bolt rows", "Anzahl der Schraubenreihen", "bolts"), "newBoltsPerRow": count("Bolts per row", "Schrauben je Reihe", "bolts")},
    "change-materials": {"materials": rec("Materials", "Werkstoffe", D("The complete list of aluminium materials.", "Die vollständige Liste der Aluminiumwerkstoffe."))},
    "change-plate-thickness": {"sectionId": ref("section", "Cross-section", "Querschnitt", D("The cross-section containing the plate element.", "Der Querschnitt, der das Blechelement enthält.")), "elementId": ref("element", "Plate element", "Blechelement", D("The plate element of the cross-section.", "Das Blechelement des Querschnitts.")), "newThickness": q("Plate thickness t", "Blechdicke t", "m", "section", disp="mm", step=0.0005, prec=1, xlo=0)},
    "change-shells": {"shells": rec("Shell structures", "Schalentragwerke", D("The complete list of shell structures (EN 1999-1-5).", "Die vollständige Liste der Schalentragwerke (EN 1999-1-5)."))},
}

#endregion en1999

#region iso16757

ISO_ID = {
    "retire-subject": ("subject", "Dictionary subject", "Wörterbuch-Subjekt"),
    "rename-product": ("product", "Product", "Produkt"),
    "rename-product-group": ("productGroup", "Product group", "Produktgruppe"),
    "retire-geometry-object": ("geometryObject", "Geometry object", "Geometrieobjekt"),
    "retire-product-class": ("productClass", "Product class", "Produktklasse"),
    "retire-product-index": ("productIndex", "Product index", "Produktindex"),
    "retire-product-series": ("productSeries", "Product series", "Produktserie"),
    "retire-product": ("product", "Product", "Produkt"),
    "retire-product-group": ("productGroup", "Product group", "Produktgruppe"),
    "retire-property-definition": ("propertyDefinition", "Property definition", "Merkmalsdefinition"),
}


def iso_id(leaf):
    kind, en, de = ISO_ID[leaf]
    return ref(kind, en, de, D(f"The {en.lower()} this mutation addresses.", f"Das Katalogelement ({de}), das diese Mutation adressiert."))


PART_NUMBER_KEY = txt("Part-number input", "Eingabe der Artikelnummernregel", "target", D("Key of the input the part-number rule reads, e.g. height.", "Schlüssel der Eingabe, die die Artikelnummernregel liest, z. B. height."))
ISO16757 = {
    "retire-subject": {"id": iso_id("retire-subject")},
    "introduce-subject": {"subject": rec("Dictionary subject", "Wörterbuch-Subjekt", D("The complete dictionary subject to introduce (ISO 16757-2).", "Das vollständige einzuführende Wörterbuch-Subjekt (ISO 16757-2).")), "index": at_insert()},
    "change-part-number-input": {"key": PART_NUMBER_KEY, "newValue": rec("Input value", "Eingabewert", D("Typed catalogue value (boolean, integer, decimal, text, quantity, range …).", "Typisierter Katalogwert (boolean, integer, decimal, text, quantity, range …)."), group="part-number")},
    "change-selection-class": {"newClassId": ref("productClass", "Product class", "Produktklasse", D("Product class the selection request searches in (ISO 16757-5).", "Produktklasse, in der der Auswahlauftrag sucht (ISO 16757-5)."), role="value", group="selection")},
    "rename-manufacturer": {"newName": txt("Manufacturer name", "Herstellername", "catalogue")},
    "introduce-product-class": {"productClass": rec("Product class", "Produktklasse", D("The complete product class with required and optional properties.", "Die vollständige Produktklasse mit Pflicht- und optionalen Merkmalen.")), "index": at_insert()},
    "rename-product": {"id": iso_id("rename-product"), "newName": txt("Product name", "Produktname", "catalogue")},
    "rename-catalogue": {"newName": txt("Catalogue name", "Katalogname", "catalogue")},
    "introduce-geometry-object": {"geometryObject": rec("Geometry object", "Geometrieobjekt", D("The complete parametric geometry object (ISO 16757-4) to introduce.", "Das vollständige einzuführende parametrische Geometrieobjekt (ISO 16757-4)."))},
    "introduce-property-definition": {"propertyDefinition": rec("Property definition", "Merkmalsdefinition", D("The complete property definition with data type, unit and cardinality.", "Die vollständige Merkmalsdefinition mit Datentyp, Einheit und Kardinalität.")), "index": at_insert()},
    "introduce-product-series": {"productSeries": rec("Product series", "Produktserie", D("The complete product series of a product class to introduce.", "Die vollständige einzuführende Produktserie einer Produktklasse.")), "index": at_insert()},
    "introduce-product": {"product": rec("Product", "Produkt", D("The complete product with parameter domains, variants and static properties.", "Das vollständige Produkt mit Parameterbereichen, Varianten und statischen Merkmalen.")), "index": ui("stepper", L("Position", "Position"), desc=D("Zero-based list position; empty appends at the end.", "Nullbasierte Listenposition; leer hängt am Ende an."), group="target", bounds={"minimum": 0}, step=1, precision=0)},
    "change-exchange-process": {"newExchangeProcess": sel("Exchange process", "Austauschprozess", "catalogue", {"CreateFromDictionary": ("Create from dictionary", "Aus dem Wörterbuch erstellen"), "ProvideCatalogue": ("Provide catalogue", "Katalog bereitstellen"), "DetermineProduct": ("Determine product", "Produkt bestimmen"), "IntegrateIntoSystem": ("Integrate into system", "In das System integrieren"), "ExchangeSystemModel": ("Exchange system model", "Systemmodell austauschen")}, D("ISO 16757-1 exchange process the catalogue serves.", "Austauschprozess nach ISO 16757-1, dem der Katalog dient."))},
    "remove-part-number-input": {"key": PART_NUMBER_KEY},
    "introduce-product-index": {"productIndex": rec("Product index", "Produktindex", D("The complete search index entry of a product variant.", "Der vollständige Suchindexeintrag einer Produktvariante.")), "index": at_insert()},
    "add-selection-constraint": {"constraint": rec("Selection constraint", "Auswahlbedingung", D("Property, comparison operator and value restricting the selection.", "Merkmal, Vergleichsoperator und Wert, die die Auswahl einschränken."), group="selection")},
    "remove-selection-constraint": {"index": at_entry("Selection constraint", "Auswahlbedingung", "selection constraint", "der Auswahlbedingung")},
    "rename-product-group": {"id": iso_id("rename-product-group"), "newName": txt("Product group name", "Name der Produktgruppe", "catalogue")},
    "retire-geometry-object": {"id": iso_id("retire-geometry-object")},
    "retire-product-class": {"id": iso_id("retire-product-class")},
    "retire-product-index": {"id": iso_id("retire-product-index")},
    "retire-product-series": {"id": iso_id("retire-product-series")},
    "change-script-limits": {"newMaxSteps": count("Maximum script steps", "Maximale Skriptschritte", "limits"), "newMaxRecursion": count("Maximum recursion depth", "Maximale Rekursionstiefe", "limits"), "newTimeoutMs": count("Script timeout", "Zeitlimit des Skripts", "limits", unit="ms")},
    "retire-product": {"id": iso_id("retire-product")},
    "replace-part-number-rule": {"newRule": rec("Part-number rule", "Artikelnummernregel", D("Literal, lookup table or script that derives the part number.", "Literal, Nachschlagetabelle oder Skript, das die Artikelnummer ableitet."), group="part-number")},
    "change-selection-series": {"newSeriesId": ref("productSeries", "Product series", "Produktserie", D("Product series the selection request narrows to.", "Produktserie, auf die der Auswahlauftrag eingeschränkt wird."), role="value", group="selection")},
    "retire-product-group": {"id": iso_id("retire-product-group")},
    "introduce-product-group": {"productGroup": rec("Product group", "Produktgruppe", D("The complete product group to introduce.", "Die vollständige einzuführende Produktgruppe.")), "index": at_insert()},
    "retire-property-definition": {"id": iso_id("retire-property-definition")},
}
ISO_NAMES = {
    "locale": txt("Language", "Sprache", "names", D("BCP 47 language tag, e.g. en or de.", "Sprachkennung nach BCP 47, z. B. en oder de.")),
    "text": txt("Text", "Text", "names"),
    "preferred": rec("Preferred name", "Vorzugsbenennung", group="names"),
    "shortName": txt("Short name", "Kurzbezeichnung", "names"),
    "alternatives": rec("Synonyms", "Synonyme", group="names"),
    "names": rec("Names", "Benennungen", group="names"),
    "definition": rec("Definition", "Definition", group="names"),
    "parentId": ref("subject", "Parent", "Übergeordnetes Element", role="value", group="hierarchy"),
    "groupId": ref("productGroup", "Product group", "Produktgruppe", role="value", group="hierarchy"),
    "classId": ref("productClass", "Product class", "Produktklasse", role="value", group="hierarchy"),
    "seriesId": ref("productSeries", "Product series", "Produktserie", role="value", group="hierarchy"),
    "productId": ref("product", "Product", "Produkt", role="value", group="hierarchy"),
    "variantId": ref("variant", "Variant", "Variante", role="value", group="hierarchy"),
    "geometryId": ref("geometryObject", "Geometry object", "Geometrieobjekt", role="value", group="geometry"),
    "requiredPropertyIds": ref("propertyDefinition", "Required properties", "Pflichtmerkmale", role="value", group="properties"),
    "optionalPropertyIds": ref("propertyDefinition", "Optional properties", "Optionale Merkmale", role="value", group="properties"),
    "dictionaryPropertyId": ref("subject", "Dictionary property", "Wörterbuch-Merkmal", role="value", group="properties"),
    "dictionarySubjectId": ref("subject", "Dictionary subject", "Wörterbuch-Subjekt", role="value", group="hierarchy"),
    "definitionId": ref("propertyDefinition", "Property definition", "Merkmalsdefinition", role="value", group="properties"),
    "propertyId": ref("propertyDefinition", "Property", "Merkmal", role="value", group="selection"),
    "parameterId": ref("propertyDefinition", "Parameter", "Parameter", role="value", group="parameters"),
    "functionId": txt("Function", "Funktion", "properties", D("Id of the script function that computes the value.", "Kennung der Skriptfunktion, die den Wert berechnet.")),
    "dataType": txt("Data type", "Datentyp", "properties", D("E.g. boolean, integer, decimal, text, quantity or range.", "Z. B. boolean, integer, decimal, text, quantity oder range.")),
    "unit": rec("Unit", "Einheit", group="properties"),
    "symbol": txt("Unit symbol", "Einheitenzeichen", "properties"),
    "dimension": rec("Dimension", "Dimension", D("Exponents of the SI base quantities.", "Exponenten der SI-Basisgrößen."), group="properties"),
    "siFactor": factor("Factor to SI", "Umrechnungsfaktor zur SI-Einheit", "properties", step=0.001, prec=6),
    "length": count("Length exponent", "Exponent der Länge", "properties", lo=None),
    "mass": count("Mass exponent", "Exponent der Masse", "properties", lo=None),
    "time": count("Time exponent", "Exponent der Zeit", "properties", lo=None),
    "temperature": count("Temperature exponent", "Exponent der Temperatur", "properties", lo=None),
    "cardinality": rec("Cardinality", "Kardinalität", group="properties"),
    "min": count("Minimum occurrences", "Mindestanzahl", "properties"),
    "max": count("Maximum occurrences", "Höchstanzahl", "properties"),
    "parameterDomains": rec("Parameter domains", "Parameterbereiche", group="parameters"),
    "allowedValues": rec("Allowed values", "Zulässige Werte", group="parameters"),
    "defaultValue": rec("Default value", "Vorgabewert", group="parameters"),
    "variants": rec("Variants", "Varianten", group="variants"),
    "parameterValues": rec("Parameter values", "Parameterwerte", group="variants"),
    "propertyValues": rec("Property values", "Merkmalswerte", group="variants"),
    "articleNumber": txt("Article number", "Artikelnummer", "variants"),
    "staticProperties": rec("Static properties", "Statische Merkmale", group="properties"),
    "sharedPropertyValues": rec("Shared property values", "Gemeinsame Merkmalswerte", group="properties"),
    "searchTags": ui(None, L("Search tags", "Suchbegriffe"), group="search"),
    "shape": rec("Shape", "Form", group="geometry"),
    "symbolic": rec("Symbolic representation", "Symbolische Darstellung", group="geometry"),
    "spaces": rec("Spaces", "Räume", D("Overall, operation, access, placement and installation spaces.", "Gesamt-, Bedien-, Zugangs-, Einbring- und Installationsraum."), group="geometry"),
    "surfaces": rec("Surfaces", "Flächen", group="geometry"),
    "ports": rec("Ports", "Anschlüsse", group="geometry"),
    "parameterBindings": rec("Parameter bindings", "Parameterbindungen", group="geometry"),
}
ISO_KIND = {
    "subject/kind": sel("Subject kind", "Subjektart", "hierarchy", {"ProductGroup": ("Product group", "Produktgruppe"), "ProductClass": ("Product class", "Produktklasse"), "ProductSpecialization": ("Product specialization", "Produktspezialisierung"), "CatalogueMetadata": ("Catalogue metadata", "Katalog-Metadaten"), "ManufacturerMetadata": ("Manufacturer metadata", "Hersteller-Metadaten"), "PropertyBlock": ("Property block", "Merkmalsblock"), "Port": ("Port", "Anschluss"), "Inlet": ("Inlet", "Eintritt"), "Outlet": ("Outlet", "Austritt"), "InOutlet": ("Inlet/outlet", "Ein-/Austritt")}),
    "propertyDefinition/kind": sel("Property kind", "Merkmalsart", "properties", {"Static": ("Static", "Statisch"), "Dynamic": ("Dynamic", "Dynamisch"), "Selection": ("Selection", "Auswahl"), "External": ("External", "Extern")}, widget="segmented"),
    "constraint/operator": sel("Comparison operator", "Vergleichsoperator", "selection", {"equal": ("Equal", "Gleich"), "notEqual": ("Not equal", "Ungleich"), "lessThan": ("Less than", "Kleiner als"), "greaterThan": ("Greater than", "Größer als"), "inRange": ("In range", "Im Bereich")}),
    "constraint/value": rec("Value", "Wert", D("Typed catalogue value compared with the property.", "Typisierter Katalogwert, mit dem das Merkmal verglichen wird."), group="selection"),
}

#endregion iso16757

#region vdi3805

PRODUCT3805 = ref("product", "Product", "Produkt", D("The catalogue product (article number) this mutation addresses.", "Das Katalogprodukt (Artikelnummer), das diese Mutation adressiert."))
GEOMETRY3805 = ref("geometry", "Geometry", "Geometrie", D("The parametric geometry this mutation addresses.", "Die parametrische Geometrie, die diese Mutation adressiert."))
CURVE3805 = ref("curve", "Characteristic curve", "Kennlinie", D("The characteristic curve this mutation addresses.", "Die Kennlinie, die diese Mutation adressiert."))
SHEET3805 = txt("Sheet", "Blatt", "target", D("VDI 3805 sheet number, e.g. 8.", "Blattnummer der VDI 3805, z. B. 8."))
VDI3805 = {
    "remove-geometry-connection": {"id": GEOMETRY3805, "connectionId": ref("connection", "Connection point", "Anschlusspunkt", D("The connection point of the geometry.", "Der Anschlusspunkt der Geometrie."))},
    "change-product-configuration": {"id": PRODUCT3805, "newConfiguration": rec("Configuration", "Konfiguration", D("Geometry reference, characteristic curve references and sheet attributes of the product.", "Geometriereferenz, Kennlinienreferenzen und Blattattribute des Produkts."), group="product")},
    "change-manufacturer-file": {"newManufacturerFile": rec("Manufacturer file header", "Kopfsatz der Herstellerdatei", D("Header version, manufacturer, building services number, creation date and character set.", "Kopfversion, Hersteller, Gewerknummer, Erstellungsdatum und Zeichensatz."), group="catalogue")},
    "rename-product": {"id": PRODUCT3805, "newTitle": rec("Product title", "Produktbezeichnung", D("Localized product titles, one per language.", "Lokalisierte Produktbezeichnungen, je Sprache eine."), group="product")},
    "change-correction-as-of": {"newCorrectionAsOf": rec("Correction status", "Korrekturstand", D("Year and month of the catalogue correction.", "Jahr und Monat des Katalog-Korrekturstands."), group="catalogue")},
    "add-curve": {"curve": rec("Characteristic curve", "Kennlinie", D("Curve with x/y units and points, e.g. a kv value or pump head curve.", "Kennlinie mit x/y-Einheiten und Punkten, z. B. kv-Wert- oder Förderhöhenkennlinie."))},
    "remove-curve": {"id": CURVE3805},
    "change-curve-points": {"id": CURVE3805, "newPoints": rec("Curve points", "Kennlinienpunkte", D("Ordered (x, y) points in the curve's units.", "Geordnete (x, y)-Punkte in den Einheiten der Kennlinie."), group="curve")},
    "resize-geometry": {"id": GEOMETRY3805, "newBbox": rec("Bounding box", "Hüllquader", D("Minimum and maximum corner in metres.", "Minimal- und Maximalecke in Metern."), group="geometry")},
    "add-product": {"product": rec("Product", "Produkt", D("The complete catalogue product to add.", "Das vollständige hinzuzufügende Katalogprodukt.")), "index": ui("stepper", L("Position", "Position"), desc=D("Zero-based position in the catalogue; a larger value appends.", "Nullbasierte Position im Katalog; ein größerer Wert hängt an."), group="target", bounds={"minimum": 0}, step=1, precision=0)},
    "add-geometry-connection": {"id": GEOMETRY3805, "connection": rec("Connection point", "Anschlusspunkt", D("Connection point with medium, position, direction and nominal diameter.", "Anschlusspunkt mit Medium, Lage, Richtung und Nennweite."), group="geometry")},
    "change-strict-mode": {"newStrictMode": tog("Strict mode", "Strenger Modus", "catalogue", D("Reject records that deviate from the VDI 3805 sheet definition.", "Datensätze ablehnen, die von der Blattdefinition der VDI 3805 abweichen."))},
    "change-edition-profile": {"sheet": SHEET3805, "newChoice": sel("Sheet edition", "Blattausgabe", "catalogue", {"Legacy": ("Legacy edition", "Frühere Ausgabe"), "Current": ("Current edition", "Aktuelle Ausgabe")}, widget="segmented")},
    "remove-product": {"id": PRODUCT3805},
    "change-limits": {"newLimits": rec("Security limits", "Sicherheitsgrenzen", D("Maximum file size, record count, field length and nesting depth when reading.", "Maximale Dateigröße, Satzanzahl, Feldlänge und Verschachtelungstiefe beim Einlesen."), group="limits")},
    "remove-geometry": {"id": GEOMETRY3805},
    "add-geometry": {"geometry": rec("Geometry", "Geometrie", D("Parametric geometry with bounding box, connection points and parameters.", "Parametrische Geometrie mit Hüllquader, Anschlusspunkten und Parametern."))},
    "change-geometry-parameters": {"id": GEOMETRY3805, "newParameters": rec("Geometry parameters", "Geometrieparameter", D("Named numeric parameters of the geometry.", "Benannte Zahlenparameter der Geometrie."), group="geometry")},
    "remove-edition-profile": {"sheet": SHEET3805},
}
UNIT_KIND = {"Dimensionless": ("Dimensionless", "Dimensionslos"), "Length": ("Length", "Länge"), "Area": ("Area", "Fläche"), "Volume": ("Volume", "Volumen"), "Mass": ("Mass", "Masse"), "Time": ("Time", "Zeit"), "Temperature": ("Temperature", "Temperatur"), "Force": ("Force", "Kraft"), "Pressure": ("Pressure", "Druck"), "Stress": ("Stress", "Spannung"), "Moment": ("Moment", "Moment"), "Energy": ("Energy", "Energie"), "Power": ("Power", "Leistung"), "ThermalConductivity": ("Thermal conductivity", "Wärmeleitfähigkeit"), "ThermalResistance": ("Thermal resistance", "Wärmedurchlasswiderstand"), "HeatTransferCoefficient": ("Heat transfer coefficient", "Wärmeübergangskoeffizient"), "AirPermeability": ("Air permeability", "Luftdurchlässigkeit"), "VentilationRate": ("Ventilation rate", "Luftwechselrate"), "Acceleration": ("Acceleration", "Beschleunigung")}


def bbox(axis, corner_en, corner_de):
    return q(f"{corner_en} {axis}", f"{corner_de} {axis}", "m", "geometry", disp="mm", step=0.001, prec=0)


VDI_NAMES = {
    "headerVersion": txt("Header version", "Kopfversion", "catalogue"),
    "manufacturer": txt("Manufacturer code", "Herstellerkennung", "catalogue"),
    "buildingSystemNumber": rec("Building services number", "Gewerknummer", group="catalogue"),
    "systemCode": txt("System code", "Systemkennung", "catalogue"),
    "subsystem": txt("Subsystem", "Teilsystem", "catalogue"),
    "sequence": count("Sequence number", "Laufende Nummer", "catalogue"),
    "created": txt("Creation date", "Erstellungsdatum", "catalogue"),
    "charset": txt("Character set", "Zeichensatz", "catalogue"),
    "recordCount": count("Record count", "Satzanzahl", "catalogue"),
    "extensions": rec("Extensions", "Erweiterungen", group="extensions"),
    "fields": rec("Fields", "Felder", group="extensions"),
    "locale": txt("Language", "Sprache", "product", D("Language tag, e.g. de or en.", "Sprachkennung, z. B. de oder en.")),
    "year": count("Year", "Jahr", "catalogue"),
    "month": count("Month", "Monat", "catalogue", hi=12),
    "xUnit": rec("x-axis unit", "Einheit der x-Achse", group="curve"),
    "yUnit": rec("y-axis unit", "Einheit der y-Achse", group="curve"),
    "symbol": txt("Unit symbol", "Einheitenzeichen", "curve"),
    "delta": tog("Difference quantity", "Differenzgröße", "curve", D("The unit measures a difference, e.g. K instead of °C.", "Die Einheit misst eine Differenz, z. B. K statt °C.")),
    "siFactor": factor("Factor to SI", "Umrechnungsfaktor zur SI-Einheit", "curve", step=0.001, prec=6),
    "points": rec("Curve points", "Kennlinienpunkte", group="curve"),
    "minX": bbox("x", "Minimum", "Minimum"), "minY": bbox("y", "Minimum", "Minimum"), "minZ": bbox("z", "Minimum", "Minimum"),
    "maxX": bbox("x", "Maximum", "Maximum"), "maxY": bbox("y", "Maximum", "Maximum"), "maxZ": bbox("z", "Maximum", "Maximum"),
    "identity": rec("Product identity", "Produktidentität", group="product"),
    "manufacturerCode": txt("Manufacturer code", "Herstellerkennung", "product"),
    "productGroup": txt("Product group", "Produktgruppe", "product"),
    "articleNumber": txt("Article number", "Artikelnummer", "product"),
    "title": rec("Product title", "Produktbezeichnung", group="product"),
    "sheet": count("Sheet", "Blatt", "product", D("VDI 3805 sheet number of the product.", "Blattnummer der VDI 3805 des Produkts.")),
    "records": rec("Native records", "Originalsätze", group="records"),
    "family": txt("Record family", "Satzart", "records"),
    "configuration": rec("Configuration", "Konfiguration", group="product"),
    "geometryRef": ref("geometry", "Geometry", "Geometrie", role="value", group="product"),
    "functionRefs": ref("curve", "Characteristic curves", "Kennlinien", role="value", group="product"),
    "attributes": rec("Sheet attributes", "Blattattribute", group="product"),
    "accessories": rec("Accessories", "Zubehör", group="composition"),
    "accessoryId": ref("product", "Accessory", "Zubehörteil", role="value", group="composition"),
    "required": tog("Required", "Erforderlich", "composition"),
    "quantity": count("Quantity", "Anzahl", "composition"),
    "components": rec("Components", "Komponenten", group="composition"),
    "componentId": ref("product", "Component", "Komponente", role="value", group="composition"),
    "medium": txt("Medium", "Medium", "geometry", D("Conveyed medium, e.g. water, air or gas.", "Gefördertes Medium, z. B. Wasser, Luft oder Gas.")),
    "position": ui("vector", L("Position", "Lage"), group="geometry", unit="m"),
    "direction": ui("vector", L("Direction", "Richtung"), desc=D("Outward unit vector of the connection.", "Nach außen gerichteter Einheitsvektor des Anschlusses."), group="geometry"),
    "diameterMm": q("Nominal diameter", "Nenndurchmesser", "mm", "geometry", step=1, prec=0, lo=0),
    "bbox": rec("Bounding box", "Hüllquader", group="geometry"),
    "connections": rec("Connection points", "Anschlusspunkte", group="geometry"),
    "parameters": rec("Parameters", "Parameter", group="geometry"),
    "maxFileBytes": count("Maximum file size", "Maximale Dateigröße", "limits", unit="B"),
    "maxRecords": count("Maximum number of records", "Maximale Satzanzahl", "limits"),
    "maxFieldLength": count("Maximum field length", "Maximale Feldlänge", "limits", unit="B"),
    "maxNestingDepth": count("Maximum nesting depth", "Maximale Verschachtelungstiefe", "limits"),
}
VDI_KIND = {
    "curve/xUnit/kind": sel("Quantity", "Größenart", "curve", UNIT_KIND),
    "curve/yUnit/kind": sel("Quantity", "Größenart", "curve", UNIT_KIND),
    "curve/points/-/x": factor("x", "x", "curve", step=0.01, prec=3),
    "curve/points/-/y": factor("y", "y", "curve", step=0.01, prec=3),
    "newPoints/-/x": factor("x", "x", "curve", step=0.01, prec=3),
    "newPoints/-/y": factor("y", "y", "curve", step=0.01, prec=3),
}

#endregion vdi3805

#region results

RESULTS = {
    "change-selected-check-index": {"index": ui("stepper", L("Selected check", "Ausgewählter Nachweis"), desc=D("Zero-based index of the check shown in the inspection; empty selects the first check.", "Nullbasierter Index des in der Prüfansicht gezeigten Nachweises; leer wählt den ersten Nachweis."), group="results", step=1, precision=0)},
}

#endregion results

#region registry

ARTIFACTS = {
    "🌬️din16798": {"leaves": DIN16798},
    "⚡️din18599": {"leaves": DIN18599, "paths": DIN18599_NESTED},
    "🧱️din4108": {"leaves": DIN4108},
    "🏋️en1991": {"leaves": EN1991},
    "🏛️en1992": {"leaves": EN1992},
    "🔩️en1993": {"leaves": EN1993},
    "🧩️en1994": {"leaves": EN1994},
    "🪵️en1995": {"leaves": EN1995},
    "🪨️en1996": {"leaves": EN1996},
    "🌍️en1997": {"leaves": EN1997},
    "🫨️en1998": {"leaves": EN1998},
    "🪶️en1999": {"leaves": EN1999},
    "📇️iso16757": {"leaves": ISO16757, "suffixes": ISO_KIND, "names": ISO_NAMES},
    "🏭️vdi3805": {"leaves": VDI3805, "suffixes": VDI_KIND, "names": VDI_NAMES},
    "🪟️results": {"leaves": RESULTS},
}

#endregion registry

#region writer


def kind_of(name):
    for at, character in enumerate(name):
        if character.isascii() and character.isalpha():
            return name[at:]
    return name


def annotated(prop, annotation, order):
    body = {key: value for key, value in prop.items() if key != "x-semio-ui"}
    annotation = dict(annotation)
    body.update(annotation.pop(BOUNDS, {}))
    annotation["order"] = order
    body["x-semio-ui"] = {key: annotation[key] for key in KEY_ORDER if key in annotation}
    return body


def nested_annotation(tables, leaf, child, name):
    exact = tables.get("paths", {}).get(leaf, {}).get(child)
    if exact is not None:
        return exact
    for suffix, annotation in tables.get("suffixes", {}).items():
        if child == suffix or child.endswith("/" + suffix):
            return annotation
    return tables.get("names", {}).get(name)


def nested(node, path, tables, leaf, glossary, where, missing):
    if not isinstance(node, dict):
        return node
    if isinstance(node.get("items"), dict):
        node = {**node, "items": nested(node["items"], path + "/-", tables, leaf, glossary, where, missing)}
    if isinstance(node.get("properties"), dict):
        properties = {}
        for position, (name, prop) in enumerate(node["properties"].items()):
            child = f"{path}/{name}"
            prop = nested(prop, child, tables, leaf, glossary, where, missing)
            annotation = nested_annotation(tables, leaf, child, name)
            if annotation is not None:
                prop = annotated(prop, annotation, 10 * (position + 1))
            elif name not in glossary and not (isinstance(prop, dict) and "const" in prop):
                missing.append(f"{where}{child}")
            properties[name] = prop
        node = {**node, "properties": properties}
    return node


def unannotated(node):
    if isinstance(node, dict):
        return {key: unannotated(value) for key, value in node.items() if key not in ("x-semio-ui", "minimum", "maximum", "exclusiveMinimum")}
    if isinstance(node, list):
        return [unannotated(value) for value in node]
    return node


def main():
    os.chdir(REPO)
    glossary = json.load(open(GLOSSARY, encoding="utf-8"))["labels"]
    baseline = json.load(open(sys.argv[sys.argv.index("--baseline") + 1], encoding="utf-8")) if "--baseline" in sys.argv else {}
    problems = []
    written = 0
    unchanged = 0
    seen = set()
    for path in sorted(glob.glob(f"{SCOPE}/**/🧬️mutations/*/🧬️schema/🔣️.json", recursive=True)):
        artifact = path.split("/🏅️standards")[0].split("/")[-1] if "/🏅️standards" in path else path.split(f"{SCOPE}/")[1].split("/")[0]
        leaf = kind_of(path.split("🧬️mutations/")[1].split("/")[0])
        text = open(path, encoding="utf-8").read()
        schema = json.loads(text)
        if path in baseline:
            pristine = json.loads(baseline[path])
            if unannotated(schema) != unannotated(pristine):
                problems.append(f"{path}: changed since the baseline beyond annotations and bounds")
                continue
            schema = pristine
        properties = schema.get("properties")
        if not properties:
            continue
        if artifact not in ARTIFACTS:
            problems.append(f"{artifact}: no annotation table")
            continue
        tables = ARTIFACTS[artifact]
        fields = tables["leaves"].get(leaf)
        if fields is None:
            problems.append(f"{artifact}/{leaf}: no annotation row")
            continue
        seen.add((artifact, leaf))
        if set(fields) != set(properties):
            problems.append(f"{artifact}/{leaf}: annotated {sorted(fields)} ≠ inputs {sorted(properties)}")
            continue
        out = {}
        for position, (name, prop) in enumerate(properties.items()):
            prop = nested(prop, name, tables, leaf, glossary, f"{artifact}/{leaf}:", problems)
            out[name] = annotated(prop, fields[name], 10 * (position + 1))
        schema["properties"] = out
        rendered = json.dumps(schema, indent=2, ensure_ascii=False) + ("\n" if text.endswith("\n") else "")
        if rendered == text:
            unchanged += 1
            continue
        written += 1
        if "--check" not in sys.argv:
            with open(path, "w", encoding="utf-8") as handle:
                handle.write(rendered)
    for artifact, tables in ARTIFACTS.items():
        for leaf in tables["leaves"]:
            if (artifact, leaf) not in seen:
                problems.append(f"{artifact}/{leaf}: row names no leaf on disk")
    for problem in problems:
        print("PROBLEM", problem)
    print(f"{'would write' if '--check' in sys.argv else 'wrote'} {written} leaf schemas, {unchanged} unchanged, {len(problems)} problems")
    return 1 if problems else 0


if __name__ == "__main__":
    sys.exit(main())

#endregion writer
