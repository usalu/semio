import os
import sys
from pathlib import Path

E = Path(r"C:\git\semio\🎓️teaching\🏛️architecture\⚡️energy")
IMPORT_ANCHOR = "set_vo_language = _manim_module.set_vo_language\n"


def patch(rel: str, pairs: list[tuple[str, str]], names=("tr",)) -> None:
    path = next(E.glob(rel))
    raw = path.read_bytes().decode("utf-8")
    crlf = "\r\n" in raw
    s = raw.replace("\r\n", "\n")
    adds = "".join(f"{n} = _manim_module.{n}\n" for n in names if f"\n{n} = _manim_module.{n}\n" not in s)
    assert s.count(IMPORT_ANCHOR) == 1, (rel, "anchor")
    s = s.replace(IMPORT_ANCHOR, IMPORT_ANCHOR + adds)
    for old, new in pairs:
        assert s.count(old) == 1, (rel, old, s.count(old))
        s = s.replace(old, new)
    tmp = path.with_name("lc-tmp.txt")
    tmp.write_bytes((s.replace("\n", "\r\n") if crlf else s).encode("utf-8"))
    os.replace(tmp, path)
    print("patched", rel, len(pairs))


patch("*cooling/*clip/*summer-strategy/*.py", [
    ('Text("Solarkollektor",', 'Text(tr("Solar collector", "Solarkollektor"),'),
    ('Text("Solarwärme → Heizregister",', 'Text(tr("Solar heat → heating coil", "Solarwärme → Heizregister"),'),
])

patch("*cooling/*clip/*internal-gains/*.py", [
    ('title="Körperwärme"', 'title=tr("Body heat", "Körperwärme")'),
    ('title="Gesamtwärme"', 'title=tr("Total heat", "Gesamtwärme")'),
    ('Text("Einzelperson (120 W)",', 'Text(f"{tr(\'Single person\', \'Einzelperson\')} ({unit_text(\'120\', \'W\')})",'),
], names=("tr", "unit_text"))

patch("*cooling/*clip/*solar-radiation/*.py", [
    ('title="Bestrahlungsstärke je m²"', 'title=tr("Irradiance per m²", "Bestrahlungsstärke je m²")'),
])

patch("*cooling/*clip/*heating-comparison/*.py", [
    ('title="Gerät")', 'title=tr("Device", "Gerät"))'),
])

patch("*ventilation/*clip/*mechanical/*.py", [
    ('Text("WRG Zentral",', 'Text(tr("Central HRV", "WRG Zentral"),'),
    ('legend(["70 s Abluft", "(Speichern)"],', 'legend([tr("70 s exhaust", "70 s Abluft"), tr("(storing)", "(Speichern)")],'),
    ('legend(["70 s Zuluft", "(16 °C)"],', 'legend([tr("70 s supply", "70 s Zuluft"), f"({unit_text(\'16\', \'°C\')})"],'),
], names=("tr", "unit_text"))

patch("*ventilation/*clip/*need/*.py", [
    ('body_text("Zuluft", font_size=LABEL_FONT_SIZE, color=AIR_COLOR)', 'body_text(tr("Supply air", "Zuluft"), font_size=LABEL_FONT_SIZE, color=AIR_COLOR)'),
    ('body_text("Abluft", font_size=LABEL_FONT_SIZE, color=AIR_COLOR)', 'body_text(tr("Exhaust air", "Abluft"), font_size=LABEL_FONT_SIZE, color=AIR_COLOR)'),
])

patch("*heating/*clip/*solar-gains/*.py", [
    ('Text("Raffstore",', 'Text(tr("Venetian blind", "Raffstore"),'),
])

patch("*heating/*clip/*internal-gains/*.py", [
    ('Text("Personen",', 'Text(tr("People", "Personen"),'),
    ('Text("Geräte",', 'Text(tr("Devices", "Geräte"),'),
    ('Text("Licht",', 'Text(tr("Light", "Licht"),'),
    ('title="eine sitzende Person"', 'title=tr("a seated person", "eine sitzende Person")'),
    ('rf"\\text{{Gerät}}\\;{de_num(dev_w.get_value())}\\,\\mathrm{{W}}"', 'rf"\\text{{Server}}\\;{label_number(dev_w.get_value())}\\,\\mathrm{{W}}"'),
    ('rf"\\text{{Laptop}}\\;{de_num(lap_w.get_value())}\\,\\mathrm{{W}}"', 'rf"\\text{{Laptop}}\\;{label_number(lap_w.get_value())}\\,\\mathrm{{W}}"'),
], names=("tr", "label_number"))

patch("*heating/*clip/*calculation/*.py", [
    ('_label("Speicher",', '_label(tr("Store", "Speicher"),'),
    ('_label("Wärme-",', '_label(tr("Heat", "Wärme-"),'),
    ('_label("erzeuger",', '_label(tr("generator", "erzeuger"),'),
    ('_label("unbeheizter",', '_label(tr("unheated", "unbeheizter"),'),
    ('_label("Keller",', '_label(tr("basement", "Keller"),'),
    ('_label("beheizt, 20 °C",', '_label(f"{tr(\'heated\', \'beheizt\')}, {unit_text(\'20\', \'°C\')}",'),
    ('_label("30 m Rohr",', '_label(tr("30 m pipe", "30 m Rohr"),'),
], names=("tr", "unit_text"))

patch("*heating/*clip/*conduction/*.py", [
    ('r"\\text{Toaster} \\approx 900\\,\\mathrm{W}"', 'rf"\\text{{{tr(\'Toaster\', \'Toaster\')}}} \\approx 900\\,\\mathrm{{W}}"'),
    ('r"\\text{Mensch in Ruhe} \\approx 100\\,\\mathrm{W}"', 'rf"\\text{{{tr(\'Person at rest\', \'Mensch in Ruhe\')}}} \\approx 100\\,\\mathrm{{W}}"'),
    ('r"\\text{Teelicht} \\approx 40\\,\\mathrm{W}"', 'rf"\\text{{{tr(\'Tealight\', \'Teelicht\')}}} \\approx 40\\,\\mathrm{{W}}"'),
    ('("toaster", 900.0, "Toaster", PASTEL_ORANGE), ("person", 100.0, "Mensch in Ruhe", PASTEL_RED),\n                 ("tealight", 40.0, "Teelicht", PASTEL_ORANGE)]',
     '("toaster", 900.0, tr("Toaster", "Toaster"), PASTEL_ORANGE), ("person", 100.0, tr("Person at rest", "Mensch in Ruhe"), PASTEL_RED),\n                 ("tealight", 40.0, tr("Tealight", "Teelicht"), PASTEL_ORANGE)]'),
    ('rf"{de_num(1.75, 2)}\\,\\mathrm{{m}}"', 'rf"{label_number(1.75, 2)}\\,\\mathrm{{m}}"'),
], names=("tr", "label_number"))

patch("*physics/*clip/*fundamentals/*.py", [
    ('r"\\text{Teelicht} \\approx 40\\,\\mathrm{W}"', 'rf"\\text{{{tr(\'Tealight\', \'Teelicht\')}}} \\approx 40\\,\\mathrm{{W}}"'),
    ('r"\\text{Staubsauger} \\approx 700\\,\\mathrm{W}"', 'rf"\\text{{{tr(\'Vacuum cleaner\', \'Staubsauger\')}}} \\approx 700\\,\\mathrm{{W}}"'),
    ('r"\\text{Teelicht} \\approx 0{,}15\\,\\mathrm{kWh}"', 'rf"\\text{{{tr(\'Tealight\', \'Teelicht\')}}} \\approx {label_number(0.15, 2)}\\,\\mathrm{{kWh}}"'),
    ('r"\\text{Staubsauger, 1,4 Stunden} \\approx 1\\,\\mathrm{kWh}"', 'rf"\\text{{{tr(\'Vacuum cleaner, 1.4 hours\', \'Staubsauger, 1,4 Stunden\')}}} \\approx 1\\,\\mathrm{{kWh}}"'),
    ('rf"\\text{{Licht}}\\;{_de(3 * f_l.get_value())}\\,\\mathrm{{W}}"', 'rf"\\text{{{tr(\'Light\', \'Licht\')}}}\\;{label_number(3 * f_l.get_value())}\\,\\mathrm{{W}}"'),
    ('rf"\\text{{Wärme}}\\;{_de(57 * f_h.get_value())}\\,\\mathrm{{W}}"', 'rf"\\text{{{tr(\'Heat\', \'Wärme\')}}}\\;{label_number(57 * f_h.get_value())}\\,\\mathrm{{W}}"'),
    ('r"\\text{Licht} \\to \\text{Wärme}\\;3\\,\\mathrm{W}"', 'rf"\\text{{{tr(\'Light\', \'Licht\')}}} \\to \\text{{{tr(\'Heat\', \'Wärme\')}}}\\;3\\,\\mathrm{{W}}"'),
    ('r"60\\,\\mathrm{W}\\;\\text{Wärme}"', 'rf"60\\,\\mathrm{{W}}\\;\\text{{{tr(\'Heat\', \'Wärme\')}}}"'),
    ('r"\\text{Heizwasser}\\;35\\,\\mathrm{°C}"', 'rf"\\text{{{tr(\'Heating water\', \'Heizwasser\')}}}\\;35\\,\\mathrm{{°C}}"'),
], names=("tr", "label_number"))
