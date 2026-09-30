#!/usr/bin/env python3
"""🔋️ W2-R energy: writes the design §6 `x-semio-ui` input annotations onto every energy mutation leaf payload schema
(`✏️s/🔌️plugins/🔋️energy/**/🧬️mutations/<leaf>/🧬️schema/🔣️.json`) and the structural hard bounds of unsigned payload
fields (`minimum: 0`, plus the u8/u16 `maximum`) read from each leaf's Rust payload struct.

Decisions (report `📓️w2-r-energy-report.md`):
- Semantic ranges owned by a leaf diff (`mutation.invariant`) stay out of the schema: their refusal fixtures carry
  out-of-range payloads by design. They reach the UI as a soft slider range or a description instead.
- German terms follow the plugin's own mutation labels (DIN/VDI wording where the plugin has none); the two misleading
  plugin terms were corrected in both places: Zonenverbund (not Thermische Hülle) and Sollwertführung (not Sollwertmanager).

Idempotent: re-running replaces the annotations it owns. The two compact set-camera schemas are edited by hand.
Addressed ids and entity references are integer `role: target` references with `ref {kind, domain: energyModel,
granularity}` (W1-D `ReferenceIdType::Integer`); created ids and plain node numbers stay numeric steppers."""
import glob
import json
import os
import re
import sys

REPO = "/Users/ueli/Documents/semio"
SCOPE = "✏️s/🔌️plugins/🔋️energy"
KEY_ORDER = ("widget", "role", "label", "description", "ref", "unit", "displayUnit", "displayFactor", "step", "precision", "softMin", "softMax", "scale", "snaps", "snapSource", "options", "group", "order")
HAND_EDITED = ("🎥️set-camera",)
PER_HOUR = 3600
M3H10 = 10 / PER_HOUR
M3H1 = 1 / PER_HOUR


def L(en, de):
    return {"en": en, "de": de}


def ui(widget, label, group, description=None, **extra):
    body = {"widget": widget, "role": "value", "label": label, "group": group, **extra}
    if description is not None:
        body["description"] = description
    return body


def stepper(label, group, unit=None, step=1, precision=0, description=None, **extra):
    return ui("stepper", label, group, description, **({"unit": unit} if unit else {}), step=step, precision=precision, **extra)


def quantity(label, group, unit, step, precision, description=None, display=None, **extra):
    shown = {"displayUnit": display[0], "displayFactor": display[1]} if display else {}
    return stepper(label, group, unit, step, precision, description, **shown, **extra)


def ratio(label, group, description=None):
    return ui("slider", label, group, description, step=0.01, precision=2, softMin=0, softMax=1)


def percent(label, group, description=None, step=0.01, precision=0, soft_min=0):
    return ui("slider", label, group, description, displayUnit="%", displayFactor=100, step=step, precision=precision, softMin=soft_min, softMax=1)


def efficiency(label, group, description=None):
    return percent(label, group, description, step=0.001, precision=1, soft_min=0.001)


def temperature(label, group, description=None):
    return quantity(label, group, "°C", 0.5, 1, description)


def power(label, group, description=None, display="kW"):
    return quantity(label, group, "W", 100, 2, description, (display, 0.001))


def airflow(label, group, description=None):
    return quantity(label, group, "m³/s", M3H10, 0, description, ("m³/h", PER_HOUR))


def toggle(label, group, description=None):
    return ui("toggle", label, group, description)


def text(label, group, description=None, widget="text"):
    return ui(widget, label, group, description)


def choice(label, group, options, description=None):
    return ui("segmented" if len(options) <= 3 else "select", label, group, description, options=options)


def listing(label, group, description=None, **extra):
    body = {"role": "value", "label": label, "group": group, **extra}
    if description is not None:
        body["description"] = description
    return body


#region 🔖️Vocabulary
ENTITY = {
    "zone": ("zone", "Zone", "f"),
    "space": ("space", "Raum", "m"),
    "surface": ("surface", "Oberfläche", "f"),
    "fenestration": ("window", "Fenster", "n"),
    "shading-surface": ("shading surface", "Verschattungsfläche", "f"),
    "material": ("material", "Material", "n"),
    "glazing-material": ("glazing material", "Verglasungsmaterial", "n"),
    "gas-material": ("gas gap", "Gasfüllung", "f"),
    "construction": ("construction", "Konstruktion", "f"),
    "construction-layer": ("construction", "Konstruktion", "f"),
    "construction-layers": ("construction", "Konstruktion", "f"),
    "people-gain": ("people gain", "Personenwärmegewinn", "m"),
    "lighting-gain": ("lighting gain", "Beleuchtungswärmegewinn", "m"),
    "equipment-gain": ("equipment gain", "Gerätewärmegewinn", "m"),
    "infiltration": ("infiltration", "Infiltration", "f"),
    "thermostat": ("thermostat", "Thermostat", "m"),
    "humidistat": ("humidistat", "Feuchteregler", "m"),
    "setpoint-manager": ("setpoint manager", "Sollwertführung", "f"),
    "ideal-loads-system": ("ideal loads system", "Ideallastsystem", "n"),
    "zone-equipment": ("zone equipment", "Zonengerät", "n"),
    "air-loop": ("air loop", "Luftkreislauf", "m"),
    "plant-loop": ("plant loop", "Anlagenkreislauf", "m"),
    "outdoor-air-system": ("outdoor air system", "Außenluftsystem", "n"),
    "mechanical-ventilation": ("mechanical ventilation", "Mechanische Lüftung", "f"),
    "pv-system": ("PV system", "PV-Anlage", "f"),
    "battery": ("battery", "Batterie", "f"),
    "electrical-load-center": ("electrical load center", "Stromverteiler", "m"),
    "service-hot-water-system": ("service hot water system", "Trinkwarmwasseranlage", "f"),
    "solar-thermal-system": ("solar thermal system", "Solarthermieanlage", "f"),
    "refrigeration-system": ("refrigeration system", "Kälteanlage", "f"),
    "water-system": ("water system", "Wasseranlage", "f"),
    "fault": ("fault", "Fehler", "m"),
    "daylight-zone": ("daylight zone", "Tageslichtzone", "f"),
    "sizing-object": ("sizing object", "Auslegungsobjekt", "n"),
    "space-list": ("space list", "Raumliste", "f"),
    "thermal-enclosure": ("thermal enclosure", "Zonenverbund", "m"),
    "annual-schedule": ("annual schedule", "Jahreszeitplan", "m"),
    "daily-schedule": ("daily schedule", "Tageszeitplan", "m"),
    "weekly-schedule": ("weekly schedule", "Wochenzeitplan", "m"),
    "time-series-schedule": ("time series schedule", "Zeitreihenzeitplan", "m"),
    "constant-schedule": ("constant schedule", "Konstanter Zeitplan", "m"),
}
RELATIVE = {"m": "den", "f": "die", "n": "das"}
TARGET_VERB = {
    "delete": ("to delete", "löscht"),
    "rename": ("to rename", "umbenennt"),
}

ROUGHNESS = {"VeryRough": L("Very rough", "Sehr rau"), "Rough": L("Rough", "Rau"), "MediumRough": L("Medium rough", "Mittelrau"), "MediumSmooth": L("Medium smooth", "Mittelglatt"), "Smooth": L("Smooth", "Glatt"), "VerySmooth": L("Very smooth", "Sehr glatt")}
SURFACE_CLASS = {"ExteriorWall": L("Exterior wall", "Außenwand"), "InteriorWall": L("Interior wall", "Innenwand"), "Roof": L("Roof", "Dach"), "Ceiling": L("Ceiling", "Decke"), "Floor": L("Floor", "Fußboden"), "Interzone": L("Interzone partition", "Zonentrennbauteil"), "Adiabatic": L("Adiabatic", "Adiabates Bauteil"), "Ground": L("Ground contact", "Erdberührtes Bauteil")}
BOUNDARY = {"OutdoorAir": L("Outdoor air", "Außenluft"), "Ground": L("Ground", "Erdreich"), "OtherSideTemperature": L("Other-side temperature", "Temperatur der Gegenseite"), "Adiabatic": L("Adiabatic", "Adiabat"), "Interzone": L("Adjacent zone", "Nachbarzone")}
LOOP_TYPE = {"Heating": L("Heating", "Heizung"), "Cooling": L("Cooling", "Kühlung"), "Condenser": L("Condenser", "Rückkühlung")}
FAULT_TYPE = {"SensorBias": L("Sensor bias", "Sensorabweichung"), "CoilFouling": L("Coil fouling", "Registerverschmutzung"), "DamperStuck": L("Stuck damper", "Klemmende Klappe"), "ChillerFouling": L("Chiller fouling", "Verschmutzung der Kältemaschine"), "BoilerEfficiencyDegradation": L("Boiler efficiency degradation", "Wirkungsgradverlust des Kessels")}
DESIGN_DAY = {"Heating": L("Heating design day", "Heizauslegungstag"), "Cooling": L("Cooling design day", "Kühlauslegungstag")}
SIZING = {"Heating": L("Heating", "Heizung"), "Cooling": L("Cooling", "Kühlung"), "OutdoorAir": L("Outdoor air", "Außenluft")}
FREQUENCY = {"Timestep": L("Every timestep", "Jeder Zeitschritt"), "Hourly": L("Hourly", "Stündlich"), "Daily": L("Daily", "Täglich"), "Monthly": L("Monthly", "Monatlich"), "RunPeriod": L("Run period", "Simulationszeitraum")}
EQUIPMENT_TYPE = {
    "Baseboard": L("Baseboard heater", "Sockelleistenheizung"),
    "Radiant": L("Radiant system", "Flächenheizung/-kühlung"),
    "FanCoil": L("Fan coil unit", "Gebläsekonvektor"),
    "Ptac": L("Packaged terminal air conditioner", "Kompaktklimagerät"),
    "VrfTerminal": L("VRF terminal unit", "VRF-Inneneinheit"),
    "Erv": L("Energy recovery ventilator", "Lüftungsgerät mit Wärmerückgewinnung"),
    "UnitHeater": L("Unit heater", "Lufterhitzer"),
    "WaterToAirHp": L("Water-to-air heat pump", "Wasser-Luft-Wärmepumpe"),
}
INFILTRATION_METHOD = {"ScheduledAch": L("Scheduled air change rate", "Luftwechsel nach Zeitplan"), "PerExteriorArea": L("Per exterior area", "Pro Außenfläche"), "EffectiveLeakageArea": L("Effective leakage area", "Effektive Leckagefläche"), "WindAndStack": L("Wind and stack", "Wind und thermischer Auftrieb")}
INTERPOLATION = {"Continuous": L("Continuous", "Stetig"), "Discrete": L("Discrete", "Diskret")}
GAS = {"Air": L("Air", "Luft"), "Argon": L("Argon", "Argon"), "Krypton": L("Krypton", "Krypton"), "Xenon": L("Xenon", "Xenon")}
ROOM_AIR = {"WellMixed": L("Well mixed", "Vollständig durchmischt"), "OneNodeDisplacement": L("One-node displacement", "Quelllüftung, Einknotenmodell"), "TwoNodeBuoyancy": L("Two-node buoyancy", "Thermischer Auftrieb, Zweiknotenmodell"), "UnderFloorAirDistribution": L("Underfloor air distribution", "Unterflur-Luftverteilung")}
SPM_KIND_OPTIONS = {"Scheduled": L("Scheduled", "Zeitplangeführt"), "OutdoorAirReset": L("Outdoor air reset", "Außentemperaturgeführt"), "WarmestZone": L("Warmest zone", "Wärmste Zone"), "ColdestZone": L("Coldest zone", "Kälteste Zone")}
RESULT_FIELD = {"conductionLoss": L("Conduction loss", "Transmissionswärmeverlust"), "conductionGain": L("Conduction gain", "Transmissionswärmegewinn"), "solarTransmitted": L("Transmitted solar", "Transmittierte Solarstrahlung"), "solarAbsorbed": L("Absorbed solar", "Absorbierte Solarstrahlung")}
TIMESTEP_SNAPS = [1, 2, 3, 4, 5, 6, 10, 12, 15, 20, 30, 60]
COMPASS_SNAPS = [0, 90, 180, 270, 360]
TILT_SNAPS = [0, 15, 30, 45, 60, 75, 90]

SCHEDULE_DESCRIPTION = L("Fraction schedule scaling the design value over time, by id.", "Anteilszeitplan, der den Bemessungswert zeitlich skaliert, per ID.")
REFERENCES = {
    "zoneId": (L("Zone", "Zone"), L("Assigned zone, by id.", "Zugeordnete Zone, per ID."), "assignment"),
    "spaceId": (L("Space", "Raum"), L("Space to add or remove, by id.", "Hinzuzufügender oder zu entfernender Raum, per ID."), "member"),
    "surfaceId": (L("Host surface", "Trägerfläche"), L("Surface the window sits in, by id.", "Oberfläche, in der das Fenster sitzt, per ID."), "assignment"),
    "constructionId": (L("Construction", "Konstruktion"), L("Layered construction of the surface, by id.", "Schichtaufbau der Oberfläche, per ID."), "construction"),
    "materialId": (L("Layer material", "Schichtmaterial"), L("Material the new layer is made of, by id.", "Material der neuen Schicht, per ID."), "construction"),
    "airLoopId": (L("Air loop", "Luftkreislauf"), L("Air loop the outdoor air system feeds, by id.", "Luftkreislauf, den das Außenluftsystem versorgt, per ID."), "assignment"),
    "pvId": (L("PV system", "PV-Anlage"), L("PV system to attach or detach, by id.", "Anzubindende oder zu lösende PV-Anlage, per ID."), "member"),
    "batteryId": (L("Battery", "Batterie"), L("Battery to attach or detach, by id.", "Anzubindende oder zu lösende Batterie, per ID."), "member"),
    "equipmentId": (L("Plant component", "Anlagenkomponente"), L("Component to add to or remove from the loop, by id.", "Dem Kreislauf hinzuzufügende oder zu entfernende Komponente, per ID."), "member"),
    "targetEquipmentId": (L("Affected system", "Betroffene Anlage"), L("Ideal loads system the fault acts on, by id.", "Ideallastsystem, auf das der Fehler wirkt, per ID."), "assignment"),
    "scheduleId": (L("Schedule", "Zeitplan"), SCHEDULE_DESCRIPTION, "schedule"),
    "activityScheduleId": (L("Activity schedule", "Aktivitätszeitplan"), L("Metabolic heat per person over time, by id.", "Wärmeabgabe je Person im Zeitverlauf, per ID."), "schedule"),
    "heatingSetpointScheduleId": (L("Heating setpoint schedule", "Heiz-Sollwertzeitplan"), L("Heating setpoint in °C over time, by id.", "Heizsollwert in °C im Zeitverlauf, per ID."), "schedule"),
    "coolingSetpointScheduleId": (L("Cooling setpoint schedule", "Kühl-Sollwertzeitplan"), L("Cooling setpoint in °C over time, by id.", "Kühlsollwert in °C im Zeitverlauf, per ID."), "schedule"),
    "humidifyingSetpointScheduleId": (L("Humidifying setpoint schedule", "Befeuchtungs-Sollwertzeitplan"), L("Relative-humidity floor over time, by id.", "Untere Sollgrenze der relativen Feuchte im Zeitverlauf, per ID."), "schedule"),
    "dehumidifyingSetpointScheduleId": (L("Dehumidifying setpoint schedule", "Entfeuchtungs-Sollwertzeitplan"), L("Relative-humidity ceiling over time, by id.", "Obere Sollgrenze der relativen Feuchte im Zeitverlauf, per ID."), "schedule"),
    "defrostScheduleId": (L("Defrost schedule", "Abtauzeitplan"), L("When the display cases defrost, by id.", "Wann die Kühlmöbel abtauen, per ID."), "schedule"),
    "startScheduleId": (L("Onset schedule", "Startzeitplan"), L("When the fault is active, by id.", "Wann der Fehler wirkt, per ID."), "schedule"),
    "transmittanceScheduleId": (L("Transmittance schedule", "Transmissionszeitplan"), L("Solar transmittance of the shade over time, by id; empty means none.", "Solarer Transmissionsgrad der Verschattung im Zeitverlauf, per ID; leer bedeutet keiner."), "schedule"),
    "dailyScheduleId": (L("Daily schedule", "Tageszeitplan"), L("Daily profile the rule applies, by id.", "Tagesprofil, das die Regel anwendet, per ID."), "schedule"),
    "defaultDailyScheduleId": (L("Default daily schedule", "Standard-Tageszeitplan"), L("Daily profile of every day no rule covers, by id.", "Tagesprofil für jeden Tag ohne Regel, per ID."), "schedule"),
    "holidayDailyScheduleId": (L("Holiday daily schedule", "Feiertags-Tageszeitplan"), L("Daily profile of the holidays, by id; empty means none.", "Tagesprofil der Feiertage, per ID; leer bedeutet keiner."), "schedule"),
    "glazingConstructionId": (L("Glazing construction", "Verglasungsaufbau"), L("Layered glazing stack, by id; empty uses the U-value, SHGC and visible transmittance.", "Geschichteter Verglasungsaufbau, per ID; leer verwendet U-Wert, g-Wert und Lichttransmissionsgrad."), "optics"),
    "interzoneSurfaceId": (L("Interzone partner surface", "Partnerfläche der Nachbarzone"), L("Required for the adjacent-zone boundary, empty for every other one.", "Erforderlich bei der Randbedingung Nachbarzone, sonst leer."), "boundary"),
    "surfaceAId": (L("Surface A", "Oberfläche A"), L("First surface of the adjacency pair, by id.", "Erste Oberfläche des Nachbarschaftspaars, per ID."), "assignment"),
    "surfaceBId": (L("Surface B", "Oberfläche B"), L("Second surface of the adjacency pair, by id.", "Zweite Oberfläche des Nachbarschaftspaars, per ID."), "assignment"),
}
REFERENCE_OVERRIDES = {
    ("bind-fenestration-glazing-construction", "constructionId"): (L("Glazing construction", "Verglasungsaufbau"), L("Layered glazing stack the window uses, by id.", "Geschichteter Verglasungsaufbau des Fensters, per ID."), "optics"),
    ("change-setpoint-manager-schedule", "newScheduleId"): (L("Setpoint schedule", "Sollwertzeitplan"), L("Setpoint in °C over time, by id; 0 while no schedule is used.", "Sollwert in °C im Zeitverlauf, per ID; 0, solange kein Zeitplan verwendet wird."), "schedule"),
    ("create-setpoint-manager", "scheduleId"): (L("Setpoint schedule", "Sollwertzeitplan"), L("Setpoint in °C over time, by id; 0 while no schedule is used.", "Sollwert in °C im Zeitverlauf, per ID; 0, solange kein Zeitplan verwendet wird."), "schedule"),
    ("change-weekly-schedule-day", "newDailyScheduleId"): (L("Daily schedule", "Tageszeitplan"), L("Daily profile of this weekday slot, by id.", "Tagesprofil dieses Wochentags, per ID."), "schedule"),
    ("add-thermal-enclosure-zone", "zoneId"): (L("Zone", "Zone"), L("Zone to add, by id.", "Hinzuzufügende Zone, per ID."), "member"),
    ("remove-thermal-enclosure-zone", "zoneId"): (L("Zone", "Zone"), L("Zone to remove, by id.", "Zu entfernende Zone, per ID."), "member"),
    ("add-air-loop-terminal-zone", "zoneId"): (L("Served zone", "Versorgte Zone"), L("Zone the air loop starts serving, by id.", "Zone, die der Luftkreislauf zusätzlich versorgt, per ID."), "member"),
    ("remove-air-loop-terminal-zone", "zoneId"): (L("Served zone", "Versorgte Zone"), L("Zone the air loop stops serving, by id.", "Zone, die der Luftkreislauf nicht mehr versorgt, per ID."), "member"),
    ("create-room-air-model-assignment", "zoneId"): (L("Zone", "Zone"), L("Zone the room air model applies to, by id.", "Zone, für die das Raumluftmodell gilt, per ID."), "target"),
    ("change-room-air-model", "zoneId"): (L("Zone", "Zone"), L("Zone whose room air model changes, by id.", "Zone, deren Raumluftmodell sich ändert, per ID."), "target"),
    ("delete-room-air-model-assignment", "zoneId"): (L("Zone", "Zone"), L("Zone whose room air model assignment is deleted, by id.", "Zone, deren Raumluftmodellzuordnung gelöscht wird, per ID."), "target"),
    ("add-space-list-member", "spaceId"): (L("Space", "Raum"), L("Space to add, by id.", "Hinzuzufügender Raum, per ID."), "member"),
    ("remove-space-list-member", "spaceId"): (L("Space", "Raum"), L("Space to remove, by id.", "Zu entfernender Raum, per ID."), "member"),
    ("add-electrical-load-center-pv", "pvId"): (L("PV system", "PV-Anlage"), L("PV system to attach, by id.", "Anzubindende PV-Anlage, per ID."), "member"),
    ("remove-electrical-load-center-pv", "pvId"): (L("PV system", "PV-Anlage"), L("PV system to detach, by id.", "Zu lösende PV-Anlage, per ID."), "member"),
    ("add-electrical-load-center-battery", "batteryId"): (L("Battery", "Batterie"), L("Battery to attach, by id.", "Anzubindende Batterie, per ID."), "member"),
    ("remove-electrical-load-center-battery", "batteryId"): (L("Battery", "Batterie"), L("Battery to detach, by id.", "Zu lösende Batterie, per ID."), "member"),
    ("add-plant-loop-equipment", "equipmentId"): (L("Plant component", "Anlagenkomponente"), L("Component to add to the loop, by id.", "Dem Kreislauf hinzuzufügende Komponente, per ID."), "member"),
    ("remove-plant-loop-equipment", "equipmentId"): (L("Plant component", "Anlagenkomponente"), L("Component to remove from the loop, by id.", "Aus dem Kreislauf zu entfernende Komponente, per ID."), "member"),
}
ID_LISTS = {
    "zoneIds": (L("Zones", "Zonen"), L("Member zones, by id.", "Zugehörige Zonen, per ID."), "member"),
    "spaceIds": (L("Spaces", "Räume"), L("Member spaces, by id.", "Zugehörige Räume, per ID."), "member"),
    "layerMaterialIds": (L("Layers", "Schichten"), L("Layer materials by id, outside layer first.", "Schichtmaterialien per ID, äußere Schicht zuerst."), "construction"),
    "equipmentIds": (L("Plant components", "Anlagenkomponenten"), L("Component ids in strictly ascending order.", "Komponenten-IDs in streng aufsteigender Reihenfolge."), "member"),
    "generatorIds": (L("Generators", "Generatoren"), L("Attached generators, by id.", "Angebundene Generatoren, per ID."), "member"),
    "pvIds": (L("PV systems", "PV-Anlagen"), L("Attached PV systems, by id.", "Angebundene PV-Anlagen, per ID."), "member"),
    "batteryIds": (L("Batteries", "Batterien"), L("Attached batteries, by id.", "Angebundene Batterien, per ID."), "member"),
    "terminalZoneIds": (L("Served zones", "Versorgte Zonen"), L("Zone ids in strictly ascending order.", "Zonen-IDs in streng aufsteigender Reihenfolge."), "member"),
    "dailyScheduleIds": (L("Daily schedules", "Tageszeitpläne"), L("Seven daily profiles by id, one per weekday slot 0 to 6.", "Sieben Tagesprofile per ID, je eines pro Wochentag 0 bis 6."), "schedule"),
}
ID_LIST_OVERRIDES = {
    ("replace-airflow-network", "zoneIds"): (L("Zones", "Zonen"), L("Zones in the network, by id, paired one-to-one with the zone nodes.", "Zonen im Netz, per ID, paarweise den Zonenknoten zugeordnet."), "network"),
    ("replace-airflow-network", "nodeIds"): (L("Zone nodes", "Zonenknoten"), L("Airflow network node number of each zone, in zone order.", "Knotennummer jeder Zone im Luftströmungsnetz, in Zonenreihenfolge."), "network"),
    ("replace-airflow-network", "linkIds"): (L("Links", "Strömungspfade"), L("Airflow network link numbers.", "Nummern der Strömungspfade im Netz."), "network"),
    ("create-thermal-enclosure", "zoneIds"): (L("Zones", "Zonen"), L("Zones grouped in the enclosure, by id.", "Im Zonenverbund zusammengefasste Zonen, per ID."), "member"),
    ("create-space-list", "spaceIds"): (L("Spaces", "Räume"), L("Spaces in the list, by id.", "Räume der Liste, per ID."), "member"),
    ("reorder-construction-layers", "newLayerMaterialIds"): (L("Layers", "Schichten"), L("The same layer materials in their new order by id, outside layer first.", "Dieselben Schichtmaterialien in neuer Reihenfolge per ID, äußere Schicht zuerst."), "construction"),
}
#endregion 🔖️Vocabulary


def base_name(name):
    if name.startswith("new") and len(name) > 3 and name[3].isupper():
        return name[3].lower() + name[4:]
    return name


def target_id(kind, verb, entity):
    en, de, gender = ENTITY[entity]
    if verb == "create":
        return stepper(L("ID", "ID"), "identity", description=L(f"Id the new {en} is created under; later mutations address it by this id.", "ID, unter der das neue Objekt angelegt wird; spätere Mutationen adressieren es darüber."))
    en_verb, de_verb = TARGET_VERB.get(verb, ("this mutation changes", "ändert"))
    return stepper(L(en[:1].upper() + en[1:], de), "target", description=L(f"The {en} {en_verb}, by id.", f"{de}, {RELATIVE[gender]} diese Mutation {de_verb}, per ID."))


def index_ui(kind, verb, entity):
    if kind == "add-construction-layer":
        return stepper(L("Layer position", "Schichtposition"), "order", description=L("Insertion position; 0 is the outside layer.", "Einfügeposition; 0 ist die äußere Schicht."))
    if kind == "remove-construction-layer":
        return stepper(L("Layer position", "Schichtposition"), "order", description=L("Position of the layer to remove; 0 is the outside layer.", "Position der zu entfernenden Schicht; 0 ist die äußere Schicht."))
    if kind == "insert-annual-schedule-rule":
        return stepper(L("Rule position", "Regelposition"), "order", description=L("Insertion position in the rule list; 0 is first.", "Einfügeposition in der Regelliste; 0 ist die erste."))
    if kind == "remove-annual-schedule-rule":
        return stepper(L("Rule position", "Regelposition"), "order", description=L("Position of the rule to remove; 0 is first.", "Position der zu entfernenden Regel; 0 ist die erste."))
    if kind == "add-annual-schedule-holiday":
        return stepper(L("Position", "Position"), "order", description=L("Insertion position in the holiday list; 0 is first.", "Einfügeposition in der Feiertagsliste; 0 ist die erste."))
    if verb == "add":
        return stepper(L("Position", "Position"), "order", description=L("Insertion position in the member list; 0 is first.", "Einfügeposition in der Mitgliederliste; 0 ist die erste."))
    return stepper(L("Position", "Position"), "order", description=L("Insertion position in the model's list; 0 is first.", "Einfügeposition in der Modellliste; 0 ist die erste."))


def capacity_present(kind, name):
    if "Heating" in name or "heating" in kind:
        return toggle(L("Limit heating capacity", "Heizleistung begrenzen"), "capacity", L("Off means unlimited; the heating capacity is then 0.", "Aus bedeutet unbegrenzt; die Heizleistung ist dann 0."))
    return toggle(L("Limit cooling capacity", "Kühlleistung begrenzen"), "capacity", L("Off means unlimited; the cooling capacity is then 0.", "Aus bedeutet unbegrenzt; die Kühlleistung ist dann 0."))


def vertices(entity):
    if entity == "fenestration":
        return listing(L("Vertices", "Eckpunkte"), "geometry", L("Aperture polygon in the host surface's plane, counter-clockwise seen from outside; empty derives the rectangle from area, height and sill height.", "Öffnungspolygon in der Ebene der Trägerfläche, von außen gesehen gegen den Uhrzeigersinn; leer leitet das Rechteck aus Fläche, Höhe und Brüstungshöhe ab."), unit="m")
    if entity == "shading-surface":
        return listing(L("Vertices", "Eckpunkte"), "geometry", L("Polygon corners in world coordinates, at least three.", "Polygonecken in Weltkoordinaten, mindestens drei."), unit="m")
    return listing(L("Vertices", "Eckpunkte"), "geometry", L("Polygon corners in world coordinates, counter-clockwise seen from outside.", "Polygonecken in Weltkoordinaten, von außen gesehen gegen den Uhrzeigersinn."), unit="m")


def calendar(label_en, label_de, group, soft_max):
    return stepper(L(label_en, label_de), group, softMin=1, softMax=soft_max)


DOMAIN = "energyModel"
GRANULARITY = {
    "zone": "zone", "space": "space", "surface": "surface", "fenestration": "fenestration", "shading-surface": "shading",
    "material": "material", "glazing-material": "glazingMaterial", "gas-material": "gasMaterial",
    "construction": "construction", "construction-layer": "construction", "construction-layers": "construction",
    "thermostat": "thermostat", "people-gain": "load", "lighting-gain": "load", "equipment-gain": "load", "infiltration": "load",
    "ideal-loads-system": "hvac", "annual-schedule": "schedule", "daily-schedule": "schedule", "weekly-schedule": "schedule",
    "time-series-schedule": "schedule", "constant-schedule": "schedule",
}
FIELD_REFERENCE = {
    "zoneId": ("zone", "zone"), "zoneIds": ("zone", "zone"), "terminalZoneIds": ("zone", "zone"),
    "spaceId": ("space", "space"), "spaceIds": ("space", "space"),
    "surfaceId": ("surface", "surface"), "surfaceAId": ("surface", "surface"), "surfaceBId": ("surface", "surface"), "interzoneSurfaceId": ("surface", "surface"),
    "constructionId": ("construction", "construction"), "glazingConstructionId": ("construction", "construction"),
    "materialId": ("material", "material"), "layerMaterialIds": (["material", "glazingMaterial", "gasMaterial"], None),
    "dailyScheduleIds": ("schedule", "schedule"), "targetEquipmentId": ("idealLoadsSystem", "hvac"),
    "airLoopId": ("airLoop", None), "pvId": ("pvSystem", None), "pvIds": ("pvSystem", None), "batteryId": ("battery", None), "batteryIds": ("battery", None),
    "equipmentId": ("plantComponent", None), "equipmentIds": ("plantComponent", None), "generatorIds": ("generator", None),
}


def field_reference(base):
    if base.endswith("ScheduleId") or base == "scheduleId":
        return ("schedule", "schedule")
    return FIELD_REFERENCE.get(base)


def referenced(annotation, kind, granularity, in_domain):
    body = {key: value for key, value in annotation.items() if key not in ("step", "precision", "softMin", "softMax", "unit", "snaps")}
    ref = {"kind": kind, **({"domain": DOMAIN} if in_domain else {}), **({"granularity": granularity} if granularity else {})}
    return {**body, "widget": "reference", "role": "target", "ref": ref}


def value_ui(kind, verb, entity, name):
    base = base_name(name)
    if name == "id":
        annotation = target_id(kind, verb, entity)
        if verb == "create":
            return annotation
        granularity = GRANULARITY.get(entity)
        return referenced(annotation, camel(entity.replace("-", "_")) if entity not in ("construction-layer", "construction-layers") else "construction", granularity, granularity is not None)
    if (kind, name) in REFERENCE_OVERRIDES or base in REFERENCES:
        label, description, group = REFERENCE_OVERRIDES.get((kind, name)) or REFERENCES[base]
        annotation = stepper(label, group, description=description)
        target = field_reference(base)
        return referenced(annotation, target[0], target[1], target[1] is not None) if target else annotation
    if (kind, name) in ID_LIST_OVERRIDES or base in ID_LISTS:
        label, description, group = ID_LIST_OVERRIDES.get((kind, name)) or ID_LISTS[base]
        annotation = listing(label, group, description)
        target = field_reference(base)
        return referenced(annotation, target[0], target[1], True if isinstance(target[0], list) else target[1] is not None) if target else annotation
    if base == "index":
        return index_ui(kind, verb, entity)
    table = {
        "name": text(L("Variable name", "Variablenname"), "output", L("Output variable to report, e.g. Zone Mean Air Temperature.", "Zu berichtende Ausgabevariable, z. B. Zone Mean Air Temperature.")) if entity == "output-variable" else text(L("Name", "Name"), "identity"),
        "key": text(L("Key value", "Schlüsselwert"), "output", L("Object the variable reports for, e.g. a zone name.", "Objekt, für das die Variable berichtet, z. B. ein Zonenname.")),
        "reportingFrequency": choice(L("Reporting frequency", "Berichtsintervall"), "output", FREQUENCY),
        "version": text(L("Version", "Version"), "identity", L("Model version label.", "Versionsbezeichnung des Modells.")),
        "targetUri": text(L("Weather file", "Wetterdatei"), "source", L("Artifact reference of the EPW weather file, e.g. doc!s.stdio.semio@v1/epw.", "Artefaktverweis auf die EPW-Wetterdatei, z. B. doc!s.stdio.semio@v1/epw.")) if kind == "bind-weather-file" else text(L("Referenced model", "Referenziertes Modell"), "source", L("Artifact reference of the model that supplies the geometry, e.g. doc!s.stdio.semio@v1/model.", "Artefaktverweis auf das Modell, das die Geometrie liefert, z. B. doc!s.stdio.semio@v1/model.")),
        "field": choice(L("Result field", "Ergebnisgröße"), "display", RESULT_FIELD, L("Surface result that colours the model.", "Flächenergebnis, nach dem das Modell eingefärbt wird.")),
        "zoneTimestepMinutes": stepper(L("Zone timestep", "Zonenzeitschritt"), "simulation", "min", description=L("Heat balance timestep; divisors of 60 give whole steps per hour.", "Zeitschritt der Wärmebilanz; Teiler von 60 ergeben ganze Schritte pro Stunde."), snaps=TIMESTEP_SNAPS),
        "systemTimestepMinutes": stepper(L("System timestep", "Anlagenzeitschritt"), "simulation", "min", description=L("HVAC system timestep; divisors of 60 give whole steps per hour.", "Zeitschritt der Anlagentechnik; Teiler von 60 ergeben ganze Schritte pro Stunde."), snaps=TIMESTEP_SNAPS),
        "warmupDays": stepper(L("Warm-up days", "Einschwingtage"), "simulation", "d", description=L("Days simulated before the run period to settle the building's thermal mass.", "Vor dem Simulationszeitraum gerechnete Tage, damit die Speichermasse einschwingt.")),
        "volumeM3": quantity(L("Air volume", "Luftvolumen"), "geometry", "m³", 0.1, 1),
        "multiplier": stepper(L("Multiplier", "Multiplikator"), "geometry", description=L("Number of identical instances this object stands for; at least 1.", "Anzahl gleicher Exemplare, die dieses Objekt vertritt; mindestens 1.")),
        "conditioned": toggle(L("Conditioned", "Konditioniert"), "thermal", L("Heated or cooled by the HVAC system.", "Von der Anlagentechnik beheizt oder gekühlt.")),
        "partOfTotalFloorArea": toggle(L("Counts toward total floor area", "Zählt zur Gesamtgeschossfläche"), "geometry"),
        "floorAreaM2": quantity(L("Floor area", "Geschossfläche"), "geometry", "m²", 0.1, 2),
        "class": choice(L("Surface class", "Bauteilart"), "construction", SURFACE_CLASS),
        "verticesM": vertices(entity),
        "boundary": choice(L("Outside boundary condition", "Außenrandbedingung"), "boundary", BOUNDARY),
        "sunExposed": toggle(L("Sun exposed", "Besonnt"), "boundary"),
        "windExposed": toggle(L("Wind exposed", "Windexponiert"), "boundary"),
        "roughness": choice(L("Roughness", "Rauigkeit"), "thermal", ROUGHNESS, L("Outside surface roughness; scales forced convection.", "Rauigkeit der Außenoberfläche; skaliert die erzwungene Konvektion.")),
        "thicknessM": quantity(L("Gap width", "Scheibenabstand"), "geometry", "m", 0.001, 1, display=("mm", 1000)) if entity == "gas-material" else quantity(L("Thickness", "Dicke"), "geometry", "m", 0.001, 1, display=("mm", 1000)),
        "conductivityWMK": quantity(L("Thermal conductivity", "Wärmeleitfähigkeit"), "thermal", "W/(m·K)", 0.001, 3),
        "densityKgM3": quantity(L("Density", "Rohdichte"), "thermal", "kg/m³", 1, 0),
        "specificHeatJKgK": quantity(L("Specific heat capacity", "Spezifische Wärmekapazität"), "thermal", "J/(kg·K)", 10, 0),
        "thermalAbsorptance": ratio(L("Thermal absorptance", "Thermischer Absorptionsgrad"), "optics", L("Long-wave absorptance, equal to the emissivity.", "Langwelliger Absorptionsgrad, gleich dem Emissionsgrad.")),
        "solarAbsorptance": ratio(L("Solar absorptance", "Solarer Absorptionsgrad"), "optics"),
        "visibleAbsorptance": ratio(L("Visible absorptance", "Sichtbarer Absorptionsgrad"), "optics"),
        "solarTransmittance": ratio(L("Solar transmittance", "Solarer Transmissionsgrad"), "optics"),
        "visibleTransmittance": ratio(L("Visible transmittance", "Lichttransmissionsgrad"), "optics"),
        "infraredEmissivityFront": ratio(L("Front infrared emissivity", "Infrarot-Emissionsgrad außen"), "optics", L("Long-wave emissivity of the outside-facing side.", "Langwelliger Emissionsgrad der nach außen gewandten Seite.")),
        "infraredEmissivityBack": ratio(L("Back infrared emissivity", "Infrarot-Emissionsgrad innen"), "optics", L("Long-wave emissivity of the inside-facing side.", "Langwelliger Emissionsgrad der nach innen gewandten Seite.")),
        "gas": choice(L("Fill gas", "Füllgas"), "thermal", GAS),
        "uValueWM2k": quantity(L("U-value", "U-Wert"), "thermal", "W/(m²·K)", 0.01, 2, L("Thermal transmittance of the whole window.", "Wärmedurchgangskoeffizient des gesamten Fensters.")),
        "shgc": ratio(L("Solar heat gain coefficient", "Gesamtenergiedurchlassgrad"), "optics", L("SHGC (g-value): share of incident solar energy that enters the room.", "g-Wert: Anteil der einfallenden Sonnenenergie, der in den Raum gelangt.")),
        "vlt": ratio(L("Visible transmittance", "Lichttransmissionsgrad"), "optics"),
        "areaM2": quantity(L("Aperture area", "Aperturfläche"), "geometry", "m²", 0.1, 2) if entity == "pv-system" else quantity(L("Area", "Fläche"), "geometry", "m²", 0.01, 2),
        "heightM": quantity(L("Height", "Höhe"), "geometry", "m", 0.01, 2),
        "sillHeightM": quantity(L("Sill height", "Brüstungshöhe"), "geometry", "m", 0.01, 2),
        "frameConductanceWK": quantity(L("Frame conductance", "Rahmen-Wärmeleitwert"), "thermal", "W/K", 0.01, 2),
        "dividerConductanceWK": quantity(L("Divider conductance", "Sprossen-Wärmeleitwert"), "thermal", "W/K", 0.01, 2),
        "overhangDepthM": quantity(L("Overhang depth", "Überstandstiefe"), "shading", "m", 0.01, 2, L("Projection of the horizontal overhang above the head; 0 means none.", "Auskragung des horizontalen Überstands über dem Sturz; 0 bedeutet keiner.")),
        "overhangOffsetM": quantity(L("Overhang offset", "Überstandsabstand"), "shading", "m", 0.01, 2, L("Height of the overhang above the window head.", "Höhe des Überstands über dem Fenstersturz.")),
        "finDepthM": quantity(L("Fin depth", "Seitenblendentiefe"), "shading", "m", 0.01, 2, L("Projection of the vertical fins beside the jambs; 0 means none.", "Auskragung der seitlichen Blenden an den Laibungen; 0 bedeutet keine.")),
        "finOffsetM": quantity(L("Fin offset", "Seitenblendenabstand"), "shading", "m", 0.01, 2, L("Distance of the fins from the window jambs.", "Abstand der Seitenblenden von den Fensterlaibungen.")),
        "peoplePerArea": quantity(L("People per area", "Personen pro Fläche"), "gains", "1/m²", 0.01, 3, L("Occupant density per m² floor area.", "Belegungsdichte je m² Geschossfläche.")),
        "sensibleFraction": percent(L("Sensible fraction", "Sensibler Anteil"), "gains", L("Share of the heat output released as sensible heat.", "Anteil der Wärmeabgabe als sensible Wärme.")),
        "latentFraction": percent(L("Latent fraction", "Latenter Anteil"), "gains", L("Share of the heat output released as moisture.", "Anteil der Wärmeabgabe als latente Wärme (Feuchte).")),
        "radiantFraction": percent(L("Radiant fraction", "Strahlungsanteil"), "gains", L("Share of the heat output released as long-wave radiation.", "Anteil der Wärmeabgabe als langwellige Strahlung.")),
        "visibleFraction": percent(L("Visible fraction", "Sichtbarer Anteil"), "gains", L("Share of the lighting power emitted as visible light.", "Anteil der Beleuchtungsleistung, der als sichtbares Licht abgegeben wird.")),
        "returnAirFraction": percent(L("Return-air fraction", "Rückluftanteil"), "gains", L("Share of the lighting heat carried off by the return air.", "Anteil der Beleuchtungswärme, der mit der Rückluft abgeführt wird.")),
        "wattsPerArea": quantity(L("Power per area", "Leistung pro Fläche"), "gains", "W/m²", 0.1, 1, L("Installed power density per m² floor area.", "Installierte Leistungsdichte je m² Geschossfläche.")),
        "heatingThrottleRangeK": quantity(L("Heating throttle range", "Heiz-Proportionalbereich"), "control", "K", 0.1, 1, L("Temperature band over which heating ramps from off to full output.", "Temperaturband, in dem die Heizleistung von null auf voll ansteigt.")),
        "coolingThrottleRangeK": quantity(L("Cooling throttle range", "Kühl-Proportionalbereich"), "control", "K", 0.1, 1, L("Temperature band over which cooling ramps from off to full output.", "Temperaturband, in dem die Kühlleistung von null auf voll ansteigt.")),
        "humidifyingThrottleRange": quantity(L("Humidifying throttle range", "Befeuchtungs-Proportionalbereich"), "control", None, 0.01, 0, L("Band of relative humidity (as a fraction of saturation, shown in %) over which humidification ramps from off to full.", "Band der relativen Feuchte (als Anteil der Sättigung, in % angezeigt), in dem die Befeuchtung von null auf voll ansteigt."), ("%", 100)),
        "dehumidifyingThrottleRange": quantity(L("Dehumidifying throttle range", "Entfeuchtungs-Proportionalbereich"), "control", None, 0.01, 0, L("Band of relative humidity (as a fraction of saturation, shown in %) over which dehumidification ramps from off to full.", "Band der relativen Feuchte (als Anteil der Sättigung, in % angezeigt), in dem die Entfeuchtung von null auf voll ansteigt."), ("%", 100)),
        "kind": choice(L("Control law", "Regelgesetz"), "control", SPM_KIND_OPTIONS),
        "lowOutdoorC": temperature(L("Low outdoor temperature", "Untere Außentemperatur"), "control", L("Lower end of the outdoor temperature range; 0 unless the law is OutdoorAirReset.", "Unteres Ende des Außentemperaturbereichs; 0 außer beim Regelgesetz OutdoorAirReset.")),
        "highOutdoorC": temperature(L("High outdoor temperature", "Obere Außentemperatur"), "control", L("Upper end of the outdoor temperature range, above the lower end; 0 unless the law is OutdoorAirReset.", "Oberes Ende des Außentemperaturbereichs, über dem unteren Ende; 0 außer beim Regelgesetz OutdoorAirReset.")),
        "lowSetpointC": temperature(L("Setpoint at low outdoor temperature", "Sollwert bei unterer Außentemperatur"), "control", L("0 unless the law is OutdoorAirReset.", "0 außer beim Regelgesetz OutdoorAirReset.")),
        "highSetpointC": temperature(L("Setpoint at high outdoor temperature", "Sollwert bei oberer Außentemperatur"), "control", L("0 unless the law is OutdoorAirReset.", "0 außer beim Regelgesetz OutdoorAirReset.")),
        "schedulePresent": toggle(L("Use schedule", "Zeitplan verwenden"), "schedule", L("Off means no setpoint schedule; its id is then 0.", "Aus bedeutet kein Sollwertzeitplan; dessen ID ist dann 0.")),
        "maxHeatingSupplyAirTempC": temperature(L("Maximum heating supply air temperature", "Maximale Heizzulufttemperatur"), "control"),
        "minCoolingSupplyAirTempC": temperature(L("Minimum cooling supply air temperature", "Minimale Kühlzulufttemperatur"), "control"),
        "capacityPresent": capacity_present(kind, name),
        "maxHeatingCapacityPresent": capacity_present(kind, name),
        "maxCoolingCapacityPresent": capacity_present(kind, name),
        "maxHeatingCapacityW": power(L("Maximum heating capacity", "Maximale Heizleistung"), "capacity", L("Only while the heating capacity is limited.", "Nur solange die Heizleistung begrenzt ist.")),
        "maxCoolingCapacityW": power(L("Maximum cooling capacity", "Maximale Kühlleistung"), "capacity", L("Only while the cooling capacity is limited.", "Nur solange die Kühlleistung begrenzt ist.")),
        "outdoorAirPerPersonM3S": quantity(L("Outdoor air per person", "Außenluftrate pro Person"), "ventilation", "m³/s", M3H1, 1, None, ("m³/h", PER_HOUR)),
        "outdoorAirPerAreaM3SM2": quantity(L("Outdoor air per floor area", "Außenluftrate pro Geschossfläche"), "ventilation", "m³/(s·m²)", 0.1 / PER_HOUR, 2, None, ("m³/(h·m²)", PER_HOUR)),
        "equipmentType": choice(L("Equipment type", "Gerätetyp"), "equipment", EQUIPMENT_TYPE),
        "priority": stepper(L("Priority", "Priorität"), "equipment", description=L("Load allocation order; 1 is served first.", "Reihenfolge der Lastverteilung; 1 wird zuerst bedient."), softMin=1),
        "heatingCapacityW": power(L("Heating capacity", "Heizleistung"), "capacity"),
        "coolingCapacityW": power(L("Cooling capacity", "Kühlleistung"), "capacity"),
        "supplyNodeId": stepper(L("Supply air node", "Zuluftknoten"), "network", description=L("Node number of the supply air outlet.", "Knotennummer des Zuluftaustritts.")),
        "returnNodeId": stepper(L("Return air node", "Rückluftknoten"), "network", description=L("Node number of the return air inlet.", "Knotennummer des Rücklufteintritts.")),
        "designSupplyAirFlowM3S": airflow(L("Design supply air flow", "Bemessungszuluftvolumenstrom"), "ventilation"),
        "loopType": choice(L("Loop type", "Kreislauftyp"), "plant", LOOP_TYPE),
        "supplyTemperatureC": temperature(L("Supply temperature", "Vorlauftemperatur"), "plant"),
        "returnTemperatureC": temperature(L("Return temperature", "Rücklauftemperatur"), "plant"),
        "designFlowKgS": quantity(L("Design mass flow", "Bemessungsmassenstrom"), "plant", "kg/s", 0.01, 2),
        "minOaFlowM3S": airflow(L("Minimum outdoor air flow", "Minimaler Außenluftvolumenstrom"), "ventilation"),
        "economizerEnabled": toggle(L("Economizer", "Economizer (freie Kühlung)"), "ventilation", L("Raises the outdoor air share when outdoor air can cool.", "Erhöht den Außenluftanteil, wenn Außenluft kühlen kann.")),
        "designFlowM3S": airflow(L("Design supply flow", "Bemessungsvolumenstrom"), "ventilation"),
        "fanTotalEfficiency": efficiency(L("Fan total efficiency", "Ventilatorgesamtwirkungsgrad"), "ventilation"),
        "fanDeltaPressurePa": quantity(L("Fan pressure rise", "Ventilatordruckerhöhung"), "ventilation", "Pa", 10, 0),
        "method": choice(L("Method", "Methode"), "infiltration", INFILTRATION_METHOD, L("Which of the parameters below drive the infiltration flow.", "Welche der folgenden Parameter den Infiltrationsvolumenstrom bestimmen.")),
        "designFlowAch": quantity(L("Design air change rate", "Bemessungsluftwechsel"), "infiltration", "1/h", 0.05, 2),
        "flowPerExteriorAreaM3SM2": quantity(L("Flow per exterior area", "Volumenstrom pro Außenfläche"), "infiltration", "m³/(s·m²)", 0.1 / PER_HOUR, 2, None, ("m³/(h·m²)", PER_HOUR)),
        "effectiveLeakageAreaM2": quantity(L("Effective leakage area", "Effektive Leckagefläche"), "infiltration", "m²", 0.0001, 0, None, ("cm²", 10000)),
        "dischargeCoefficient": quantity(L("Discharge coefficient", "Durchflussbeiwert"), "infiltration", None, 0.01, 2, L("Orifice discharge coefficient of the leakage path.", "Durchflussbeiwert der Leckageöffnung.")),
        "stackHeightM": quantity(L("Stack height", "Kaminhöhe"), "infiltration", "m", 0.1, 2, L("Height difference that drives the stack effect.", "Höhenunterschied, der den thermischen Auftrieb bewirkt.")),
        "constantTermCoefficient": quantity(L("Constant term coefficient A", "Konstanter Koeffizient A"), "infiltration", None, 0.001, 4),
        "temperatureTermCoefficient": quantity(L("Temperature term coefficient B", "Temperaturkoeffizient B"), "infiltration", "1/K", 0.001, 4),
        "velocityTermCoefficient": quantity(L("Wind velocity term coefficient C", "Linearer Windkoeffizient C"), "infiltration", "s/m", 0.001, 4),
        "velocitySquaredTermCoefficient": quantity(L("Wind velocity squared term coefficient D", "Quadratischer Windkoeffizient D"), "infiltration", "s²/m²", 0.001, 4),
        "dcCapacityW": power(L("DC rated power", "DC-Nennleistung"), "capacity", display="kWp"),
        "tiltDeg": ui("slider", L("Tilt", "Neigung"), "orientation", L("Angle from horizontal: 0 is flat, 90 is vertical.", "Winkel zur Horizontalen: 0 ist waagerecht, 90 senkrecht."), unit="deg", step=1, precision=0, softMin=0, softMax=90, snaps=TILT_SNAPS),
        "azimuthDeg": ui("dial", L("Azimuth", "Azimut"), "orientation", L("Compass bearing the collector faces: 0 north, 90 east, 180 south, 270 west.", "Himmelsrichtung der Ausrichtung: 0 Nord, 90 Ost, 180 Süd, 270 West."), unit="deg", step=1, precision=0, softMin=0, softMax=360, snaps=COMPASS_SNAPS),
        "moduleEfficiency": efficiency(L("Module efficiency", "Modulwirkungsgrad"), "efficiency"),
        "inverterEfficiency": efficiency(L("Inverter efficiency", "Wechselrichterwirkungsgrad"), "efficiency"),
        "capacityKwh": quantity(L("Storage capacity", "Speicherkapazität"), "capacity", "kWh", 0.5, 1),
        "maxChargeW": power(L("Maximum charge power", "Maximale Ladeleistung"), "capacity"),
        "maxDischargeW": power(L("Maximum discharge power", "Maximale Entladeleistung"), "capacity"),
        "roundTripEfficiency": efficiency(L("Round-trip efficiency", "Zyklenwirkungsgrad"), "efficiency"),
        "heaterCapacityW": power(L("Heater capacity", "Heizleistung"), "capacity"),
        "storageVolumeM3": quantity(L("Storage volume", "Speichervolumen"), "capacity", "m³", 0.01, 0, None, ("L", 1000)),
        "setpointC": temperature(L("Tank setpoint", "Speichersolltemperatur"), "control"),
        "collectorAreaM2": quantity(L("Collector area", "Kollektorfläche"), "geometry", "m²", 0.1, 2),
        "efficiency": efficiency(L("Collector efficiency", "Kollektorwirkungsgrad"), "efficiency"),
        "caseCount": stepper(L("Display cases", "Anzahl Kühlmöbel"), "capacity"),
        "designLoadW": power(L("Design load", "Auslegungslast"), "capacity"),
        "fixtureCount": stepper(L("Fixtures", "Anzahl Entnahmestellen"), "capacity"),
        "peakFlowLS": quantity(L("Peak flow", "Spitzenvolumenstrom"), "capacity", "L/s", 0.01, 2),
        "faultType": choice(L("Fault type", "Fehlertyp"), "fault", FAULT_TYPE),
        "severity": percent(L("Severity", "Schweregrad"), "fault", L("0 is harmless, 100 % is the fault at full strength.", "0 ist wirkungslos, 100 % ist der Fehler in voller Stärke.")),
        "sizingType": choice(L("Sizing type", "Auslegungsart"), "sizing", SIZING),
        "designDayType": choice(L("Design day type", "Auslegungstagtyp"), "sizing", DESIGN_DAY),
        "illuminanceTargetLux": quantity(L("Illuminance target", "Soll-Beleuchtungsstärke"), "daylight", "lx", 10, 0),
        "glareLimit": ui("slider", L("Glare limit", "Blendungsgrenzwert"), "daylight", L("Largest acceptable simplified glare index, 0 to 1; higher means more glare.", "Größter zulässiger vereinfachter Blendungsindex, 0 bis 1; höher bedeutet mehr Blendung."), step=0.01, precision=2, softMin=0.01, softMax=1),
        "windowTransmittance": ratio(L("Window transmittance", "Fenstertransmissionsgrad"), "daylight", L("Visible transmittance of the daylight apertures.", "Lichttransmissionsgrad der Tageslichtöffnungen.")),
        "model": choice(L("Room air model", "Raumluftmodell"), "airflow", ROOM_AIR),
        "latitudeDeg": quantity(L("Latitude", "Breitengrad"), "site", "deg", 0.001, 3, L("North positive, −90 to 90.", "Nord positiv, −90 bis 90."), softMin=-90, softMax=90),
        "longitudeDeg": quantity(L("Longitude", "Längengrad"), "site", "deg", 0.001, 3, L("East positive, −180 to 180.", "Ost positiv, −180 bis 180."), softMin=-180, softMax=180),
        "elevationM": quantity(L("Elevation", "Höhe über NN"), "site", "m", 1, 0),
        "timeZoneHours": quantity(L("Time zone", "Zeitzone"), "site", "h", 0.5, 1, L("Offset from UTC.", "Abweichung von UTC."), softMin=-12, softMax=14),
        "northAxisDeg": ui("dial", L("North axis", "Nordachse"), "site", L("Angle from true north to the building's north axis, clockwise.", "Winkel von geografisch Nord zur Nordachse des Gebäudes, im Uhrzeigersinn."), unit="deg", step=1, precision=1, softMin=0, softMax=360, snaps=COMPASS_SNAPS),
        "buildingSurfaceC": listing(L("Building surface ground temperatures", "Erdreichtemperaturen unter dem Gebäude"), "ground", L("Twelve monthly values, January first.", "Zwölf Monatswerte, Januar zuerst."), unit="°C"),
        "shallowC": listing(L("Shallow ground temperatures", "Oberflächennahe Erdreichtemperaturen"), "ground", L("Twelve monthly values, January first.", "Zwölf Monatswerte, Januar zuerst."), unit="°C"),
        "deepC": temperature(L("Deep ground temperature", "Erdreichtemperatur in der Tiefe"), "ground"),
        "startMonth": calendar("Start month", "Startmonat", "calendar", 12),
        "startDay": calendar("Start day", "Starttag", "calendar", 31),
        "endMonth": calendar("End month", "Endmonat", "calendar", 12),
        "endDay": calendar("End day", "Endtag", "calendar", 31),
        "month": calendar("Month", "Monat", "calendar", 12),
        "day": calendar("Day", "Tag", "calendar", 31),
        "year": stepper(L("Year", "Jahr"), "calendar", description=L("Calendar year that fixes the weekdays.", "Kalenderjahr, das die Wochentage festlegt.")) if kind == "update-run-period" else stepper(L("Year", "Jahr"), "calendar"),
        "dayIndex": stepper(L("Weekday slot", "Wochentag"), "calendar", description=L("Slot of the week, 0 to 6.", "Tag der Woche, 0 bis 6."), softMin=0, softMax=6),
        "from": stepper(L("From position", "Von Position"), "order", description=L("Current position of the rule; 0 is first.", "Aktuelle Position der Regel; 0 ist die erste.")),
        "to": stepper(L("To position", "Nach Position"), "order", description=L("New position of the rule; 0 is first.", "Neue Position der Regel; 0 ist die erste.")),
        "hourlyValues": listing(L("Hourly values", "Stundenwerte"), "values", L("Twenty-four values, one per hour of the day.", "Vierundzwanzig Werte, einer je Tagesstunde.")),
        "interpolation": choice(L("Interpolation", "Interpolation"), "values", INTERPOLATION),
        "limitsMin": quantity(L("Lower limit", "Untergrenze"), "values", None, 0.01, 2, L("Clamps every value; set both limits or neither.", "Begrenzt jeden Wert; beide Grenzen oder keine setzen.")),
        "limitsMax": quantity(L("Upper limit", "Obergrenze"), "values", None, 0.01, 2, L("Clamps every value; set both limits or neither.", "Begrenzt jeden Wert; beide Grenzen oder keine setzen.")),
        "values": listing(L("Values", "Werte"), "values", L("One value per timestep.", "Ein Wert je Zeitschritt.")),
        "timestepSeconds": stepper(L("Timestep", "Zeitschritt"), "values", "s", step=60, description=L("Duration each value holds.", "Dauer, für die jeder Wert gilt.")),
        "value": quantity(L("Value", "Wert"), "values", None, 0.01, 2),
        "present": toggle(L("Airflow network", "Luftströmungsnetz"), "network", L("Off removes the network; on replaces it with the lists below.", "Aus entfernt das Netz; ein ersetzt es durch die folgenden Listen.")),
        "outdoorNodeId": stepper(L("Outdoor node", "Außenknoten"), "network", description=L("Node number of the outdoor environment.", "Knotennummer der Außenumgebung.")),
    }
    if base not in table:
        raise SystemExit(f"{kind}: no annotation for input {name}")
    return table[base]


def ordered(annotation, order):
    body = dict(annotation, order=order)
    return {key: body[key] for key in KEY_ORDER if key in body}


RUST_FIELD = re.compile(r"pub (\w+):\s*([^,\n]+),")
UNSIGNED = {"u8": 255, "u16": 65535, "u32": None, "crate::model::EntityId": None, "crate::model::ScheduleId": None}


def camel(snake):
    head, *rest = snake.split("_")
    return head + "".join(part[:1].upper() + part[1:] for part in rest)


def rust_types(leaf_dir):
    source = open(os.path.join(REPO, leaf_dir, "🦀️.rs"), encoding="utf-8").read()
    struct = re.search(r"pub struct \w+\s*\{(.*?)\n\}", source, re.S)
    semantics = re.search(r'verb: "([^"]*)", entity: "([^"]*)", kind: "([^"]*)"', source)
    fields = {camel(name): kind.strip() for name, kind in RUST_FIELD.findall(struct.group(1))} if struct else {}
    return fields, semantics.groups()


def bounds_of(rust):
    inner = re.fullmatch(r"(?:Option|Vec)<(.+)>", rust)
    core = inner.group(1) if inner else rust
    if core not in UNSIGNED:
        return None
    maximum = UNSIGNED[core]
    return {"minimum": 0, **({"maximum": maximum} if maximum is not None else {})}


OWNED_CONSTRAINTS = ("minimum", "maximum", "exclusiveMinimum", "exclusiveMaximum", "pattern", "minItems", "maxItems", "uniqueItems", "anyOf")
POSITIVE = {"exclusiveMinimum": 0}
NONNEG = {"minimum": 0}
FRACTION = {"minimum": 0, "maximum": 1}
OPEN_FRACTION = {"exclusiveMinimum": 0, "maximum": 1}
NONZERO = {"minimum": 1}
NAME = {"pattern": "[^\\t\\n\\u000B\\f\\r \\u0085\\u00A0\\u1680\\u2000-\\u200A\\u2028\\u2029\\u202F\\u205F\\u3000]"}
ARTIFACT_URI = {"pattern": "^[^!]+![^@]+@[\\s\\S]+/[^/]+$"}
SPM_KINDS = ["Scheduled", "OutdoorAirReset", "WarmestZone", "ColdestZone"]
POLYGON = {"minItems": 3}


def span(low, high):
    return {"minimum": low, "maximum": high}


def exactly(count):
    return {"minItems": count, "maxItems": count}


def ascending_ids():
    return {"uniqueItems": True, "items": NONZERO}


RULES = {
    "change-air-loop-supply-node": {"newSupplyNodeId": NONZERO},
    "change-air-loop-return-node": {"newReturnNodeId": NONZERO},
    "change-battery-max-charge": {"newMaxChargeW": POSITIVE},
    "change-battery-max-discharge": {"newMaxDischargeW": POSITIVE},
    "change-material-solar-absorptance": {"newSolarAbsorptance": FRACTION},
    "change-plant-loop-supply-temperature": {"newSupplyTemperatureC": span(-100, 300)},
    "change-plant-loop-return-temperature": {"newReturnTemperatureC": span(-100, 300)},
    "change-pv-system-inverter-efficiency": {"newInverterEfficiency": OPEN_FRACTION},
    "change-material-specific-heat": {"newSpecificHeatJKgK": POSITIVE},
    "change-material-density": {"newDensityKgM3": POSITIVE},
    "create-plant-loop": {"name": NAME, "supplyTemperatureC": span(-100, 300), "returnTemperatureC": span(-100, 300), "designFlowKgS": POSITIVE, "equipmentIds": ascending_ids()},
    "change-pv-system-dc-capacity": {"newDcCapacityW": POSITIVE},
    "change-equipment-gain-watts-per-area": {"newWattsPerArea": NONNEG},
    "change-solar-thermal-system-azimuth": {"newAzimuthDeg": span(0, 360)},
    "change-zone-multiplier": {"newMultiplier": NONZERO},
    "change-fenestration-height": {"newHeightM": POSITIVE},
    "change-fenestration-sill-height": {"newSillHeightM": NONNEG},
    "change-infiltration-constant-term-coefficient": {"newConstantTermCoefficient": NONNEG},
    "change-infiltration-temperature-term-coefficient": {"newTemperatureTermCoefficient": NONNEG},
    "change-infiltration-velocity-term-coefficient": {"newVelocityTermCoefficient": NONNEG},
    "change-infiltration-velocity-squared-term-coefficient": {"newVelocitySquaredTermCoefficient": NONNEG},
    "create-humidistat": {"humidifyingThrottleRange": POSITIVE, "dehumidifyingThrottleRange": POSITIVE},
    "change-fenestration-shgc": {"newShgc": FRACTION},
    "change-fenestration-vlt": {"newVlt": FRACTION},
    "update-site": {"latitudeDeg": span(-90, 90), "longitudeDeg": span(-180, 180), "timeZoneHours": span(-12, 14)},
    "change-fenestration-u-value": {"newUValueWM2k": POSITIVE},
    "change-people-gain-sensible-fraction": {"newSensibleFraction": FRACTION},
    "change-lighting-gain-radiant-fraction": {"newRadiantFraction": FRACTION},
    "change-equipment-gain-radiant-fraction": {"newRadiantFraction": FRACTION},
    "update-ground-temperature": {"buildingSurfaceC": exactly(12), "shallowC": exactly(12)},
    "bind-weather-file": {"targetUri": ARTIFACT_URI},
    "connect-referenced-model": {"targetUri": ARTIFACT_URI},
    "change-humidistat-humidifying-throttle-range": {"newHumidifyingThrottleRange": POSITIVE},
    "change-humidistat-dehumidifying-throttle-range": {"newDehumidifyingThrottleRange": POSITIVE},
    "change-infiltration-flow-per-exterior-area": {"newFlowPerExteriorAreaM3SM2": NONNEG},
    "create-outdoor-air-system": {"minOaFlowM3S": NONNEG},
    "create-shading-surface": {"name": NAME, "verticesM": POLYGON},
    "change-fault-severity": {"newSeverity": FRACTION},
    "change-air-loop-design-supply-air-flow": {"newDesignSupplyAirFlowM3S": POSITIVE},
    "change-zone-equipment-cooling-capacity": {"newCoolingCapacityW": NONNEG},
    "change-zone-equipment-heating-capacity": {"newHeatingCapacityW": NONNEG},
    "change-shw-system-heater-capacity": {"newHeaterCapacityW": POSITIVE},
    "change-mechanical-ventilation-fan-delta-pressure": {"newFanDeltaPressurePa": NONNEG},
    "add-annual-schedule-holiday": {"month": span(1, 12), "day": span(1, 31)},
    "change-lighting-gain-return-air-fraction": {"newReturnAirFraction": FRACTION},
    "change-pv-system-module-efficiency": {"newModuleEfficiency": OPEN_FRACTION},
    "change-zone-equipment-priority": {"newPriority": NONZERO},
    "change-thermostat-heating-throttle-range": {"newHeatingThrottleRangeK": POSITIVE},
    "change-thermostat-cooling-throttle-range": {"newCoolingThrottleRangeK": POSITIVE},
    "change-fenestration-overhang-offset": {"newOverhangOffsetM": NONNEG},
    "change-fenestration-overhang-depth": {"newOverhangDepthM": NONNEG},
    "change-fenestration-fin-offset": {"newFinOffsetM": NONNEG},
    "change-fenestration-fin-depth": {"newFinDepthM": NONNEG},
    "change-solar-thermal-system-efficiency": {"newEfficiency": OPEN_FRACTION},
    "change-refrigeration-system-design-load": {"newDesignLoadW": POSITIVE},
    "create-zone": {"name": NAME, "volumeM3": POSITIVE, "multiplier": NONZERO},
    "change-infiltration-stack-height": {"newStackHeightM": NONNEG},
    "change-shw-system-setpoint": {"newSetpointC": span(0, 100)},
    "change-material-visible-absorptance": {"newVisibleAbsorptance": FRACTION},
    "change-people-gain-people-per-area": {"newPeoplePerArea": NONNEG},
    "change-mechanical-ventilation-fan-total-efficiency": {"newFanTotalEfficiency": OPEN_FRACTION},
    "change-equipment-gain-latent-fraction": {"newLatentFraction": FRACTION},
    "change-people-gain-latent-fraction": {"newLatentFraction": FRACTION},
    "update-run-period": {"startMonth": span(1, 12), "endMonth": span(1, 12), "startDay": span(1, 31), "endDay": span(1, 31)},
    "change-pv-system-tilt": {"newTiltDeg": span(0, 90)},
    "add-output-variable": {"name": NAME},
    "create-setpoint-manager": {"name": NAME, "kind": {"enum": SPM_KINDS}},
    "replace-setpoint-manager-kind": {"newKind": {"enum": SPM_KINDS}},
    "change-material-thickness": {"newThicknessM": POSITIVE},
    "insert-annual-schedule-rule": {"startMonth": span(1, 12), "endMonth": span(1, 12), "startDay": span(1, 31), "endDay": span(1, 31)},
    "change-people-gain-radiant-fraction": {"newRadiantFraction": FRACTION},
    "change-zone-volume": {"newVolumeM3": POSITIVE},
    "change-surface-multiplier": {"newMultiplier": NONZERO},
    "change-infiltration-design-flow-ach": {"newDesignFlowAch": NONNEG},
    "change-lighting-gain-visible-fraction": {"newVisibleFraction": FRACTION},
    "change-material-thermal-absorptance": {"newThermalAbsorptance": FRACTION},
    "change-lighting-gain-watts-per-area": {"newWattsPerArea": NONNEG},
    "change-model-version": {"newVersion": NAME},
    "change-material-conductivity": {"newConductivityWMK": POSITIVE},
    "add-plant-loop-equipment": {"equipmentId": NONZERO},
    "create-daylight-zone": {"illuminanceTargetLux": POSITIVE, "glareLimit": POSITIVE, "windowTransmittance": FRACTION},
    "change-ideal-loads-system-outdoor-air-per-area": {"newOutdoorAirPerAreaM3SM2": NONNEG},
    "change-ideal-loads-system-outdoor-air-per-person": {"newOutdoorAirPerPersonM3S": NONNEG},
    "change-ideal-loads-system-max-heating-supply-air-temp": {"newMaxHeatingSupplyAirTempC": span(-100, 200)},
    "change-ideal-loads-system-min-cooling-supply-air-temp": {"newMinCoolingSupplyAirTempC": span(-100, 200)},
    "replace-fenestration-vertices": {"newVerticesM": {"anyOf": [{"type": "array", "maxItems": 0}, {"type": "array", "minItems": 3}]}},
    "change-glazing-material-thickness": {"newThicknessM": POSITIVE},
    "change-glazing-material-conductivity": {"newConductivityWMK": POSITIVE},
    "change-glazing-material-solar-transmittance": {"newSolarTransmittance": FRACTION},
    "change-glazing-material-visible-transmittance": {"newVisibleTransmittance": FRACTION},
    "change-glazing-material-infrared-emissivity": {"newInfraredEmissivityFront": FRACTION, "newInfraredEmissivityBack": FRACTION},
    "replace-surface-vertices": {"newVerticesM": POLYGON},
    "replace-shading-surface-vertices": {"newVerticesM": POLYGON},
    "change-solar-thermal-system-tilt": {"newTiltDeg": span(0, 90)},
    "change-pv-system-azimuth": {"newAzimuthDeg": span(0, 360)},
    "replace-daily-schedule-hourly-values": {"newHourlyValues": exactly(24)},
    "change-weekly-schedule-day": {"dayIndex": span(0, 6)},
    "replace-time-series-schedule-values": {"newValues": {"minItems": 1}},
    "change-time-series-schedule-timestep": {"newTimestepSeconds": NONZERO},
    "create-daily-schedule": {"hourlyValues": exactly(24)},
    "create-weekly-schedule": {"dailyScheduleIds": exactly(7)},
    "create-time-series-schedule": {"values": {"minItems": 1}, "timestepSeconds": NONZERO},
    "change-infiltration-effective-leakage-area": {"newEffectiveLeakageAreaM2": NONNEG},
    "change-infiltration-discharge-coefficient": {"newDischargeCoefficient": POSITIVE},
    "change-daylight-zone-glare-limit": {"newGlareLimit": POSITIVE},
    "change-daylight-zone-illuminance-target": {"newIlluminanceTargetLux": POSITIVE},
    "change-daylight-zone-window-transmittance": {"newWindowTransmittance": FRACTION},
    "change-fenestration-frame-conductance": {"newFrameConductanceWK": NONNEG},
    "change-fenestration-divider-conductance": {"newDividerConductanceWK": NONNEG},
    "change-fenestration-area": {"newAreaM2": POSITIVE},
    "change-refrigeration-system-case-count": {"newCaseCount": NONZERO},
    "change-water-system-fixture-count": {"newFixtureCount": NONZERO},
    "change-water-system-peak-flow": {"newPeakFlowLS": POSITIVE},
    "change-plant-loop-design-flow": {"newDesignFlowKgS": POSITIVE},
    "change-mechanical-ventilation-design-flow": {"newDesignFlowM3S": NONNEG},
    "create-air-loop": {"name": NAME, "supplyNodeId": NONZERO, "returnNodeId": NONZERO, "designSupplyAirFlowM3S": POSITIVE, "terminalZoneIds": {"uniqueItems": True}},
    "create-zone-equipment": {"priority": NONZERO, "heatingCapacityW": NONNEG, "coolingCapacityW": NONNEG},
    "change-shw-system-storage-volume": {"newStorageVolumeM3": POSITIVE},
    "change-gas-material-thickness": {"newThicknessM": POSITIVE},
    "change-pv-system-area": {"newAreaM2": POSITIVE},
    "change-solar-thermal-system-collector-area": {"newCollectorAreaM2": POSITIVE},
    "change-solar-thermal-system-storage-volume": {"newStorageVolumeM3": POSITIVE},
    "create-surface": {"name": NAME, "verticesM": POLYGON, "multiplier": NONZERO},
    "change-battery-round-trip-efficiency": {"newRoundTripEfficiency": OPEN_FRACTION},
    "change-battery-capacity": {"newCapacityKwh": POSITIVE},
    "change-outdoor-air-system-min-oa-flow": {"newMinOaFlowM3S": NONNEG},
    "change-space-floor-area": {"newFloorAreaM2": NONNEG},
    "create-thermostat": {"heatingThrottleRangeK": POSITIVE, "coolingThrottleRangeK": POSITIVE},
    "create-space": {"name": NAME, "floorAreaM2": NONNEG},
    "create-fenestration": {"name": NAME, "uValueWM2k": POSITIVE, "shgc": FRACTION, "vlt": FRACTION, "areaM2": POSITIVE, "heightM": POSITIVE, "sillHeightM": NONNEG},
    "create-ideal-loads-system": {"maxHeatingSupplyAirTempC": span(-100, 200), "minCoolingSupplyAirTempC": span(-100, 200), "outdoorAirPerPersonM3S": NONNEG, "outdoorAirPerAreaM3SM2": NONNEG},
    "change-simulation-settings": {"zoneTimestepMinutes": span(1, 60), "systemTimestepMinutes": span(1, 60), "warmupDays": span(0, 365)},
    **{kind: {"newName": NAME} for kind in ("rename-fenestration", "rename-shading-surface", "rename-zone", "rename-surface", "rename-model", "rename-air-loop", "rename-plant-loop", "rename-gas-material", "rename-space", "rename-setpoint-manager", "rename-electrical-load-center", "rename-thermal-enclosure", "rename-glazing-material", "rename-material", "rename-construction", "rename-space-list")},
}


def when(field, value, then, otherwise=None):
    rule = {"if": {"type": "object", "properties": {field: {"const": value}}}, "then": {"type": "object", "properties": then}}
    return {**rule, "else": {"type": "object", "properties": otherwise}} if otherwise is not None else rule


def capacity(present, value):
    return when(present, False, {value: {"const": 0}}, {value: {"type": "number", "exclusiveMinimum": 0}})


def reset_limits(kind, limits):
    return {"if": {"type": "object", "properties": {kind: {"const": "OutdoorAirReset"}}}, "else": {"type": "object", "properties": {name: {"const": 0} for name in limits}}}


def both_limits(low, high):
    return when(low, None, {high: {"type": "null"}}, {high: {"type": "number"}})


def partner(boundary, surface):
    return when(boundary, "Interzone", {surface: {"type": "integer"}}, {surface: {"type": "null"}})


CONDITIONS = {
    "change-ideal-loads-system-max-heating-capacity": [capacity("newCapacityPresent", "newMaxHeatingCapacityW")],
    "change-ideal-loads-system-max-cooling-capacity": [capacity("newCapacityPresent", "newMaxCoolingCapacityW")],
    "create-ideal-loads-system": [capacity("maxHeatingCapacityPresent", "maxHeatingCapacityW"), capacity("maxCoolingCapacityPresent", "maxCoolingCapacityW")],
    "create-setpoint-manager": [reset_limits("kind", ("lowOutdoorC", "highOutdoorC", "lowSetpointC", "highSetpointC")), when("schedulePresent", False, {"scheduleId": {"const": 0}})],
    "replace-setpoint-manager-kind": [reset_limits("newKind", ("newLowOutdoorC", "newHighOutdoorC", "newLowSetpointC", "newHighSetpointC"))],
    "change-setpoint-manager-schedule": [when("newSchedulePresent", False, {"newScheduleId": {"const": 0}})],
    "create-surface": [partner("boundary", "interzoneSurfaceId")],
    "change-surface-boundary-condition": [partner("newBoundary", "newInterzoneSurfaceId")],
    "create-daily-schedule": [both_limits("limitsMin", "limitsMax")],
    "change-daily-schedule-limits": [both_limits("newLimitsMin", "newLimitsMax")],
    "replace-airflow-network": [when("present", False, {"zoneIds": {"type": "array", "maxItems": 0}, "linkIds": {"type": "array", "maxItems": 0}})],
}


def invariant(identifier, en, de):
    return {"id": identifier, "description": L(en, de)}


RESET_RANGE = invariant("outdoor-reset-range", "Under OutdoorAirReset the high outdoor temperature lies above the low one.", "Beim Regelgesetz OutdoorAirReset liegt die obere Außentemperatur über der unteren.")
LIMITS_ORDERED = invariant("limits-ordered", "When both limits are set, the lower limit does not exceed the upper one.", "Sind beide Grenzen gesetzt, überschreitet die Untergrenze die Obergrenze nicht.")
INVARIANTS = {
    "create-plant-loop": [invariant("equipment-ids-ascending", "Plant component ids are listed in strictly ascending order.", "Die Komponenten-IDs sind streng aufsteigend sortiert.")],
    "create-air-loop": [invariant("terminal-zone-ids-ascending", "Served zone ids are listed in strictly ascending order.", "Die IDs der versorgten Zonen sind streng aufsteigend sortiert.")],
    "connect-surfaces": [invariant("distinct-surfaces", "Surface A and surface B are two different surfaces.", "Oberfläche A und Oberfläche B sind zwei verschiedene Oberflächen.")],
    "replace-airflow-network": [invariant("zone-node-pairs", "Zones and zone nodes pair one to one: both lists have the same length.", "Zonen und Zonenknoten sind paarweise zugeordnet: beide Listen sind gleich lang.")],
    "create-setpoint-manager": [RESET_RANGE],
    "replace-setpoint-manager-kind": [RESET_RANGE],
    "create-daily-schedule": [LIMITS_ORDERED],
    "change-daily-schedule-limits": [LIMITS_ORDERED],
}


def constrained(prop, rust, rules):
    body = {key: value for key, value in prop.items() if key not in OWNED_CONSTRAINTS}
    items = body.get("items")
    if isinstance(items, dict) and items.get("type") == "integer":
        body["items"] = {key: value for key, value in items.items() if key not in ("minimum", "maximum")}
    bounds = bounds_of(rust) or {}
    if rust.startswith("Vec<") and bounds:
        body["items"] = {**body["items"], **bounds}
    else:
        body.update(bounds)
    for key, value in rules.items():
        if key == "items":
            body["items"] = {**body["items"], **value}
        else:
            body[key] = value
    return body


def annotate(path):
    leaf_dir = path.rsplit("/🧬️schema/🔣️.json", 1)[0]
    fields, (verb, entity, kind) = rust_types(leaf_dir)
    schema = json.load(open(os.path.join(REPO, path), encoding="utf-8"))
    rules = RULES.get(kind, {})
    unknown = set(rules) - set(schema["properties"])
    if unknown:
        raise SystemExit(f"{kind}: rules for undeclared inputs {sorted(unknown)}")
    properties = {}
    inputs = [name for name, prop in schema["properties"].items() if "const" not in prop]
    for name, prop in schema["properties"].items():
        if "const" in prop:
            properties[name] = prop
            continue
        position = inputs.index(name) + 1
        rust = fields.get(name)
        if rust is None:
            raise SystemExit(f"{kind}: input {name} has no Rust payload field")
        body = {key: value for key, value in prop.items() if key != "x-semio-ui"}
        properties[name] = {**constrained(body, rust, rules.get(name, {})), "x-semio-ui": ordered(value_ui(kind, verb, entity, name), position * 10)}
    rest = {key: value for key, value in schema.items() if key not in ("properties", "allOf", "x-semio-invariant")}
    rebuilt = {}
    for key, value in rest.items():
        rebuilt[key] = value
        if key == "required":
            rebuilt["properties"] = properties
    if "properties" not in rebuilt:
        rebuilt["properties"] = properties
    if kind in CONDITIONS:
        rebuilt["allOf"] = CONDITIONS[kind]
    if kind in INVARIANTS:
        rebuilt["x-semio-invariant"] = INVARIANTS[kind]
    return rebuilt


def main():
    check = "--check" in sys.argv
    paths = sorted(glob.glob(f"{SCOPE}/**/🧬️mutations/*/🧬️schema/🔣️.json", root_dir=REPO, recursive=True))
    written = 0
    for path in paths:
        if any(f"/{leaf}/" in path for leaf in HAND_EDITED):
            continue
        schema = annotate(path)
        rendered = json.dumps(schema, indent=2, ensure_ascii=False) + "\n"
        target = os.path.join(REPO, path)
        if open(target, encoding="utf-8").read() != rendered:
            written += 1
            if not check:
                with open(target, "w", encoding="utf-8") as handle:
                    handle.write(rendered)
    print(f"leaves={len(paths)} changed={written}{' (check only)' if check else ''}")


if __name__ == "__main__":
    main()
