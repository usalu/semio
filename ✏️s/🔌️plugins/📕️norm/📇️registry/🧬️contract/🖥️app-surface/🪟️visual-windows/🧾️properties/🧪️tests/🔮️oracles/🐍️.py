"""🔮️ Independent oracle of the typed inspector's unit conversion and locale number text.

Written from the specification only (display = si * factor, locale decimal mark, trailing zeros trimmed) with
exact rational arithmetic from the standard library; it imports nothing from the Rust it judges. Run it to
regenerate `🔣️.json`, which `🔬️unit/🦀️.rs` compares byte for byte with the Rust output.
"""

import json
import pathlib
from fractions import Fraction

FACTORS = {
    ("m", "mm"): Fraction(1000),
    ("m", "cm"): Fraction(100),
    ("N", "kN"): Fraction(1, 1000),
    ("Pa", "MPa"): Fraction(1, 1000000),
    ("N·m", "kN·m"): Fraction(1, 1000),
    ("m²", "cm²"): Fraction(10000),
    ("m⁴", "cm⁴"): Fraction(100000000),
}

CASES = [
    ("m", "mm", 0.0015, 3),
    ("m", "cm", 2.5, 2),
    ("N", "kN", 125000.0, 3),
    ("Pa", "MPa", 355000000.0, 1),
    ("N·m", "kN·m", 48250.0, 3),
    ("m²", "cm²", 0.0054, 2),
    ("m⁴", "cm⁴", 0.0000019, 0),
]


def fixed(value: Fraction, precision: int) -> str:
    scaled = round(value * 10**precision)
    sign = "-" if scaled < 0 else ""
    digits = str(abs(scaled)).rjust(precision + 1, "0")
    text = f"{digits[:-precision]}.{digits[-precision:]}" if precision else digits
    if "." in text:
        text = text.rstrip("0").rstrip(".")
    return sign + text if text != "0" else "0"


def main() -> None:
    rows = []
    for si, shown, value, precision in CASES:
        shown_value = Fraction(value) * FACTORS[(si, shown)]
        text = fixed(shown_value, precision)
        rows.append({
            "si": si,
            "symbol": shown,
            "siValue": value,
            "precision": precision,
            "en": f"{text} {shown}",
            "de": f"{text.replace('.', ',')} {shown}",
        })
    out = pathlib.Path(__file__).with_name("🔣️.json")
    out.write_text(json.dumps(rows, ensure_ascii=False, indent=1) + "\n", encoding="utf-8")


if __name__ == "__main__":
    main()
