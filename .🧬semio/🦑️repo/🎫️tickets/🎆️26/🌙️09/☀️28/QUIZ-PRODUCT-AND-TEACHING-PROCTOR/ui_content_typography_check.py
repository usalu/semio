"""🔤️ Number and unit typography of the four energy quiz content files: check, or apply the mechanical rules with --fix.

One convention per language, the one `Intl.NumberFormat` prints in the quiz UI:

- U+00A0 between a number and its unit symbol or unit word, and next to × ÷ ≈ = ≤ inside a formula.
- Percent: "100%" in English, "100" + U+00A0 + "%" in German.
- Thousands: comma in English, dot in German, never a space; years, standards and names stay ungrouped.
- U+2212 as minus, " – " (en dash) as the dash, never an em dash.
- English and German state the same numbers (two pairs word one differently on purpose: "before 1979" / "bis 1978" and
  "24-hour operation" / "rund um die Uhr"); German uses the informal address.
- The explanations of the `energies` sorting task name the SI-prefixed value of the card first.
- No-break spaces are written as the JSON escape so that they stay visible in the source.

--fix applies the space, percent and dash rules; thousands separators and wording are edited by hand.

Usage: python ui_content_typography_check.py [--fix]

See https://www.bipm.org/en/publications/si-brochure (space between value and unit).
"""

from __future__ import annotations

import json
import re
import sys
from collections import Counter
from decimal import ROUND_HALF_UP, Decimal
from pathlib import Path

ROOT = Path(__file__).resolve().parents[7]
TOPICS = ("physics", "heating", "cooling", "demand")
LANGUAGES = ("en", "de")
NBSP = chr(0xA0)
NBSP_ESCAPE = chr(92) + "u00a0"
OTHER_SPACES = "".join(map(chr, (0x2007, 0x2008, 0x2009, 0x202F)))
SUPERSCRIPTS = "⁰¹²³⁴⁵⁶⁷⁸⁹"
OPERATORS = "×÷≈=≤"
LETTERS = "A-Za-zÀ-ÖØ-öø-ÿ"
SYMBOLS = (
    "K", "°C", "h", "s", "g", "kg", "l", "m", "cm", "mm", "km", "m²", "m³", "W", "kW", "MW", "GW", "TW", "PW", "Wh", "kWh",
    "MWh", "GWh", "TWh", "PWh", "J", "kJ", "MJ", "PJ", "EJ", "kcal", "kWp", "mAh", "met", "PS", "V", "A", "€", "ct", "Pa",
)
UNIT_WORDS = {
    "en": ("full-load hours", "air changes", "hours", "hour", "minutes", "minute", "litres", "litre"),
    "de": ("Kühl-Volllaststunden", "Volllaststunden", "Luftwechsel", "Stunden", "Stunde", "Minuten", "Minute", "Liter"),
}
SI_PREFIXES = {-30: "q", -27: "r", -24: "y", -21: "z", -18: "a", -15: "f", -12: "p", -9: "n", -6: "µ", -3: "m", 0: "", 3: "k", 6: "M", 9: "G", 12: "T", 15: "P", 18: "E", 21: "Z", 24: "Y", 27: "R", 30: "Q"}
CARD_TASKS = ("energies",)
DATE = "04.12.2020"
KNOWN_NUMBER_DIFFERENCES = ({Decimal(1978), Decimal(1979)}, {Decimal(24)})
TEXT = re.compile(r'("(en|de)":\s*")((?:[^"\\]|\\.)*)(")')
NUMBER_END = rf"(?:(?<=[0-9])|(?<=[0-9][{SUPERSCRIPTS}])|(?<=[0-9][{SUPERSCRIPTS}][{SUPERSCRIPTS}]))"
RUN = re.compile(r"(?<![0-9.,])[0-9]{4,}")
STANDARD = re.compile(r"(?:DIN(?: EN| V)?|EN|ISO|IEC|VDI) $")
NUMBER = {
    "en": re.compile(r"(?<![0-9.,])(?:[0-9]{1,3}(?:,[0-9]{3})+|[0-9]+)(?:\.[0-9]+)?"),
    "de": re.compile(r"(?<![0-9.,])(?:[0-9]{1,3}(?:\.[0-9]{3})+|[0-9]+)(?:,[0-9]+)?"),
}
FORMAL = re.compile(r"\b(?:Sie|Ihr|Ihre|Ihren|Ihrem|Ihrer|Ihres|Ihnen)\b")
HYPHEN_MINUS = re.compile(rf"(?<![0-9{LETTERS}{SUPERSCRIPTS})\]_])-[0-9]| - ")
SPACED_THOUSANDS = re.compile(rf"[0-9][ {NBSP}{OTHER_SPACES}][0-9]{{3}}(?![0-9])")
OPERATOR_GAP = re.compile(rf"(?<=[{OPERATORS}]) | (?=[{OPERATORS}])")


def quiz_files() -> list[Path]:
    """📂️ The four quiz content files, found by topic folder so that no emoji path is typed twice."""
    files = sorted(path for path in ROOT.glob("*teaching/*architecture/*energy/*/*quiz/*.json") if path.parents[1].name.endswith(TOPICS))
    if len(files) != len(TOPICS):
        raise SystemExit(f"expected {len(TOPICS)} quiz files, found {len(files)}")
    return files


def unit_symbols(quizzes: list[dict]) -> tuple[str, ...]:
    """📐️ The unit symbols of the rules: the fixed set plus every `unit` field of the quizzes — with every SI prefix
    when the quantity is `prefixed`, as a card may show any of them — longest first."""
    found: set[str] = set(SYMBOLS)

    def collect(node: object) -> None:
        if isinstance(node, dict):
            if isinstance(node.get("unit"), str):
                found.update(f"{prefix}{node['unit']}" for prefix in (SI_PREFIXES.values() if node.get("prefixed") else [""]))
            for value in node.values():
                collect(value)
        elif isinstance(node, list):
            for value in node:
                collect(value)

    for quiz in quizzes:
        collect(quiz)
    return tuple(sorted(found, key=lambda unit: (-len(unit), unit)))


def unit_pattern(symbols: tuple[str, ...], language: str, percent: bool) -> str:
    """🧩️ What may follow a number as its unit in `language`: a symbol not continued by a letter, or a unit word."""
    alternatives = [rf"{re.escape(symbol)}(?![{LETTERS}])" for symbol in symbols]
    alternatives += [rf"{re.escape(word)}\b" for word in UNIT_WORDS[language]]
    return "(?:" + "|".join(alternatives + (["%"] if percent else [])) + ")"


def rules(symbols: tuple[str, ...], language: str) -> tuple[tuple[str, re.Pattern[str], str], ...]:
    """🛠️ The mechanical rules of --fix as (name, pattern, replacement), applied in order."""
    return (
        ("percent", re.compile(r"(?<=[0-9]) (?=%)"), "" if language == "en" else NBSP),
        ("unit", re.compile(rf"{NUMBER_END} (?={unit_pattern(symbols, language, False)})"), NBSP),
        ("operator", OPERATOR_GAP, NBSP),
        ("dash", re.compile(" — "), " – "),
    )


def texts(node: object, path: str = "") -> list[tuple[str, dict[str, str]]]:
    """🗺️ Every `{en, de}` text of a quiz with its JSON path."""
    if isinstance(node, dict):
        if set(node) == set(LANGUAGES) and all(isinstance(value, str) for value in node.values()):
            return [(path, node)]
        return [entry for key, value in node.items() for entry in texts(value, f"{path}.{key}" if path else key)]
    if isinstance(node, list):
        return [entry for index, value in enumerate(node) for entry in texts(value, f"{path}[{index}]")]
    return []


def numbers(text: str, language: str) -> list[Decimal]:
    """🔢️ The numbers a text states, read with the separators of its language."""
    grouping, decimal = (",", ".") if language == "en" else (".", ",")
    return sorted(Decimal(match[0].replace(grouping, "").replace(decimal, ".")) for match in NUMBER[language].finditer(text.replace(DATE, "")))


def card(value: float, unit: str, language: str) -> str:
    """🃏️ The SI-prefixed value a sorting card shows: four significant digits, mantissa in [1, 1000)."""
    number = Decimal(repr(value))
    rounded = number.quantize(Decimal(1).scaleb(number.adjusted() - 3), rounding=ROUND_HALF_UP)
    exponent = max(-30, min(30, rounded.adjusted() // 3 * 3))
    mantissa = format(rounded.scaleb(-exponent).normalize(), "f")
    return f"{mantissa.replace('.', ',') if language == 'de' else mantissa}{NBSP}{SI_PREFIXES[exponent]}{unit}"


def context(text: str, start: int, end: int) -> str:
    """🔎️ A match with a little surrounding text, no-break spaces shown as ⍽."""
    return text[max(0, start - 18) : end + 18].replace(NBSP, "⍽")


def violations(text: str, language: str, symbols: tuple[str, ...]) -> list[str]:
    """🚨️ Every broken rule of one text."""
    units = unit_pattern(symbols, language, True)
    found: list[str] = []

    def scan(name: str, pattern: str | re.Pattern[str]) -> None:
        found.extend(f"{name}: …{context(text, match.start(), match.end())}…" for match in re.finditer(pattern, text))

    scan("plain space before unit", rf"[0-9{SUPERSCRIPTS}] {units}")
    scan("plain space at operator", OPERATOR_GAP)
    scan("space as thousands separator", SPACED_THOUSANDS)
    scan("percent", rf"[0-9][ {NBSP}{OTHER_SPACES}]%" if language == "en" else rf"[0-9]%|[0-9][ {OTHER_SPACES}]%")
    scan("em dash", "—")
    scan("hyphen as minus", HYPHEN_MINUS)
    scan("narrow or figure space", rf"[{OTHER_SPACES}]")
    if language == "de":
        scan("formal address", FORMAL)
    expected = rf"(?<=[{OPERATORS}]){NBSP}|{NBSP}(?=[{OPERATORS}])|{NUMBER_END}{NBSP}(?={unit_pattern(symbols, language, language == 'de')})"
    rest = re.sub(expected, "", text)
    found.extend(f"no-break space outside the rules: …{context(rest, match.start(), match.end())}…" for match in re.finditer(NBSP, rest))
    quantity_tail = re.compile(rf"(?:–[0-9.,]+)?[ {NBSP}]{units}|-mal")
    for match in RUN.finditer(text):
        year = len(match[0]) == 4 and 1800 <= int(match[0]) <= 2099 and not quantity_tail.match(text, match.end())
        if not year and not STANDARD.search(text, 0, match.start()):
            found.append(f"ungrouped quantity: …{context(text, match.start(), match.end())}…")
    return found


def card_findings(quiz: dict, language: str) -> tuple[list[str], list[str]]:
    """🃏️ Sorting explanations whose first value of the task's quantity is not the card's value: (violations, notes)."""
    failed: list[str] = []
    noted: list[str] = []
    for task_index, task in enumerate(quiz["tasks"]):
        quantity = task.get("quantity", {})
        if task["kind"] != "sorting" or not quantity.get("prefixed"):
            continue
        stated = re.compile(rf"[0-9][0-9.,]*{NBSP}[{''.join(SI_PREFIXES.values())}]?{re.escape(quantity['unit'])}(?![{LETTERS}/])")
        for item_index, item in enumerate(task["items"]):
            shown = card(item["value"], quantity["unit"], language)
            first = stated.search(item["explanation"][language])
            if first is None or first[0] != shown:
                line = f"tasks[{task_index}].items[{item_index}].explanation.{language} ({task['id']}/{item['id']}): card {shown} but explanation starts with {first[0] if first else 'no value'}".replace(NBSP, "⍽")
                (failed if task["id"] in CARD_TASKS else noted).append(line)
    return failed, noted


def fix(path: Path, symbols: tuple[str, ...]) -> Counter[str]:
    """✍️ Applies the mechanical rules to every text of one file, keeping everything else byte for byte. The file is
    overwritten in place: a running dev server keeps it memory-mapped, and Windows refuses to truncate a mapped file."""
    counts: Counter[str] = Counter()
    raw = path.read_text(encoding="utf-8", newline="")

    def replace(match: re.Match[str]) -> str:
        text = json.loads(f'"{match[3]}"')
        if json.dumps(text, ensure_ascii=False)[1:-1].replace(NBSP, NBSP_ESCAPE) != match[3]:
            raise SystemExit(f"{path.name}: a text does not round-trip: {match[3]}")
        for name, pattern, replacement in rules(symbols, match[2]):
            text, count = pattern.subn(replacement, text)
            counts[f"{match[2]} {name}"] += count
        return f"{match[1]}{json.dumps(text, ensure_ascii=False)[1:-1].replace(NBSP, NBSP_ESCAPE)}{match[4]}"

    fixed = TEXT.sub(replace, raw)
    if fixed != raw:
        data = fixed.encode("utf-8")
        with path.open("r+b") as file:
            file.write(data)
            if len(data) < path.stat().st_size:
                file.truncate()
    return counts


def main() -> int:
    """🚦️ Checks (or fixes, then checks) the four files; exit code 1 when a rule is broken."""
    sys.stdout.reconfigure(encoding="utf-8")
    files = quiz_files()
    symbols = unit_symbols([json.loads(path.read_text(encoding="utf-8")) for path in files])
    if "--fix" in sys.argv[1:]:
        for path in files:
            print(f"fixed {path.parents[1].name}: {dict(sorted(fix(path, symbols).items()))}")
    total = 0
    for path in files:
        topic = path.parents[1].name
        raw = path.read_text(encoding="utf-8", newline="")
        quiz = json.loads(raw)
        found = [f"real no-break space in the source (write {NBSP_ESCAPE})"] if NBSP in raw else []
        notes: list[str] = []
        longest = ""
        for json_path, text in texts(quiz):
            for language in LANGUAGES:
                found += [f"{json_path}.{language}: {violation}" for violation in violations(text[language], language, symbols)]
                longest = max([longest, *text[language].split(" ")], key=len)
            difference = set(Counter(numbers(text["en"], "en")) - Counter(numbers(text["de"], "de"))) | set(Counter(numbers(text["de"], "de")) - Counter(numbers(text["en"], "en")))
            if difference and difference not in KNOWN_NUMBER_DIFFERENCES:
                found.append(f"{json_path}: English and German numbers differ: {sorted(map(str, difference))}")
        for language in LANGUAGES:
            failed, noted = card_findings(quiz, language)
            found += failed
            notes += noted
        print(f"{topic}: {len(texts(quiz))} texts, {raw.count(NBSP_ESCAPE)} no-break spaces, longest unbreakable run {len(longest)} ({longest.replace(NBSP, '⍽')}), {len(found)} violations")
        for line in found:
            print(f"  VIOLATION {line}")
        for line in notes:
            print(f"  note {line}")
        total += len(found)
    print(f"unit symbols: {' '.join(symbols)}")
    print("OK" if total == 0 else f"FAILED: {total} violations")
    return 1 if total else 0


if __name__ == "__main__":
    sys.exit(main())
