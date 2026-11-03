import sys
p = sys.argv[1]
s = open(p, encoding="utf-8", newline="").read()
nl = "\r\n" if "\r\n" in s else "\n"
s = s.replace("\r\n", "\n")
def sub(a, b):
    global s
    assert s.count(a) == 1, a
    s = s.replace(a, b)
sub("import importlib\nimport sys\nfrom functools", "import importlib\nimport os\nimport sys\nfrom functools")
sub('''#region Watt anchors
_WATT_DEVICES: dict[str, tuple[str, int]] = {
    "laptop": ("Laptop", 60),
    "toaster": ("Toaster", 900),
    "heater": ("Heizgerät", 2000),
    "person": ("stehende Person", 120),
    "seated": ("sitzende Person", 120),
    "sun": ("volle Sonne", 1000),
}''', '''#region Label language
NARROW_SPACE: str = "\u202f"


def language() -> str:
    """🌐 Explicit on-screen label language from the runtime (``SEMIO_TEACHING_LANGUAGE``); never inferred from the host.

    @see ../../../../🏗️bim/🎬️clip/🎨️visuals/🐍️.py — the BIM scene base this mirrors
    """
    selected = os.environ.get("SEMIO_TEACHING_LANGUAGE")
    if selected not in {"en", "de"}:
        raise ValueError("Set SEMIO_TEACHING_LANGUAGE to en or de")
    return selected


def tr(en: str, de: str) -> str:
    """🗣️ Pick the authored label of the explicit language, the same language the narration is recorded in."""
    return en if language() == "en" else de


def unit_text(value: str, unit: str) -> str:
    """📏 A plain-text quantity with a narrow no-break space before its unit, as ``\\,`` does in typeset math."""
    return f"{value}{NARROW_SPACE}{unit}"
#endregion


#region Watt anchors
_WATT_DEVICES: dict[str, tuple[tuple[str, str], int]] = {
    "laptop": (("Laptop", "Laptop"), 60),
    "toaster": (("Toaster", "Toaster"), 900),
    "heater": (("Heater", "Heizgerät"), 2000),
    "person": (("standing person", "stehende Person"), 120),
    "seated": (("seated person", "sitzende Person"), 120),
    "sun": (("full sun", "volle Sonne"), 1000),
}''')
sub('''    name, device_w = _WATT_DEVICES[kind]
    unit = _WATT_UNITS.get(kind, "W")''', '''    names, device_w = _WATT_DEVICES[kind]
    name = tr(*names)
    unit = _WATT_UNITS.get(kind, "W")
    point = tr(".", ",")''')
sub('''        compare_line = f"≈ 1× {name} ({device_w} {unit})"
    elif ratio < 1:
        compare_line = f"≈ {ratio:.1f}× {name} ({device_w} {unit})".replace(".", ",")
    else:
        compare_line = f"≈ {ratio:.0f}× {name} ({device_w} {unit})"''', '''        compare_line = f"≈ 1× {name} ({unit_text(str(device_w), unit)})"
    elif ratio < 1:
        compare_line = f"≈ {ratio:.1f}× {name} ({unit_text(str(device_w), unit)})".replace(".", point)
    else:
        compare_line = f"≈ {ratio:.0f}× {name} ({unit_text(str(device_w), unit)})"''')
sub('''    value = fonts.body_text(f"{int(watts) if watts == int(watts) else watts} W",''', '''    value = fonts.body_text(unit_text(str(int(watts) if watts == int(watts) else watts), "W"),''')
open(p, "w", encoding="utf-8", newline="").write(s.replace("\n", nl))
