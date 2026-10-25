#!/usr/bin/env python3
"""Independent Python model of the metrics module: UTC time bucketing (datetime/calendar), git path unquoting and rename
resolution, bucket classification and duration parsing.  Writes (or with --check verifies)
🧪️tests/🧫️fixtures/🔬️unit-vectors.json of the metrics module."""
import calendar
import datetime
import json
import re
import sys
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
MODULE = ROOT / "🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics"
ORACLE_SCRIPT = str(Path(__file__).resolve().relative_to(ROOT))
TARGET = MODULE / "🧪️tests/🧫️fixtures/🔬️unit-vectors.json"
EPOCH = datetime.datetime(1970, 1, 1)


def utc(unix):
    return EPOCH + datetime.timedelta(seconds=unix)


def stamp(moment):
    return int(calendar.timegm(moment.timetuple()))


def keys(unix):
    moment = utc(unix)
    iso_year, iso_week, iso_day = moment.isocalendar()
    day_start = moment.replace(hour=0, minute=0, second=0, microsecond=0)
    week_start = day_start - datetime.timedelta(days=iso_day - 1)
    return {
        "unix": unix,
        "commit": moment.strftime("%Y-%m-%dT%H:%M:%SZ"),
        "hour": moment.strftime("%Y-%m-%dT%HZ"),
        "day": moment.strftime("%Y-%m-%d"),
        "week": "%04d-W%02d" % (iso_year, iso_week),
        "month": moment.strftime("%Y-%m"),
        "year": moment.strftime("%Y"),
        "starts": {
            "commit": unix,
            "hour": stamp(moment.replace(minute=0, second=0)),
            "day": stamp(day_start),
            "week": stamp(week_start),
            "month": stamp(moment.replace(day=1, hour=0, minute=0, second=0)),
            "year": stamp(moment.replace(month=1, day=1, hour=0, minute=0, second=0)),
        },
        "weekday": iso_day,
        "civil": [moment.year, moment.month, moment.day],
        "days": (day_start - EPOCH).days,
    }


def lcg(seed):
    state = seed
    while True:
        state = (state * 1103515245 + 12345) & 0x7FFFFFFF
        yield state


INSTANTS = [-2208988800, -1, 0, 1, 86399, 86400, 951782400, 951868799, 1234567890, 1609459199, 1609459200, 1609718400, 1704067199, 1735603199, 1735689600, 1767225600, 1769904000, 4102444799, 4102444800, 253402300799]
# ISO-week edges: 2020-12-31 (W53), 2021-01-03 (W53), 2021-01-04 (W01), 2024-12-30 (2025-W01), 2026-01-01 (W01), 2027-01-01 (2026-W53)
for text in ("2020-12-31T12:00:00", "2021-01-03T23:59:59", "2021-01-04T00:00:00", "2024-12-30T00:00:01", "2026-01-01T00:00:00", "2027-01-01T08:00:00", "2100-02-28T23:59:59", "2100-03-01T00:00:00", "2000-02-29T10:00:00", "1999-12-31T23:00:00"):
    INSTANTS.append(stamp(datetime.datetime.fromisoformat(text)))
generator = lcg(2026)
for _ in range(260):
    INSTANTS.append(next(generator) % 4_300_000_000 - 2_200_000_000)

QUOTED = [
    '"plain.ts"', '"\\360\\237\\247\\254\\357\\270\\217alpha.ts"', '"a\\tb"', '"a\\nb"', '"quote\\"inside"', '"back\\\\slash"', '"\\303\\274ber.go"', '"\\0"', '"\\1x"', '"\\1234"', 'unquoted.ts', '"', '""', '"trailing\\"',
    '  "padded.ts"  ', '"\\a\\b\\f\\v\\r"', '"\\q"', '"\\377"', '"nested/\\360\\237\\246\\200/x.rs"',
]


def unquote(raw):
    trimmed = raw.strip()
    if len(trimmed) < 2 or not (trimmed.startswith('"') and trimmed.endswith('"')):
        return raw
    inner = trimmed[1:-1]
    simple = {"n": 10, "t": 9, "r": 13, "a": 7, "b": 8, "f": 12, "v": 11, '"': 34, "\\": 92}
    out = bytearray()
    index = 0
    while index < len(inner):
        current = inner[index]
        if current != "\\":
            out += current.encode("utf8")
            index += 1
            continue
        index += 1
        if index >= len(inner):
            break
        escape = inner[index]
        index += 1
        if escape in simple:
            out.append(simple[escape])
        elif escape in "01234567":
            value = int(escape, 8)
            taken = 1
            while taken < 3 and index < len(inner) and inner[index] in "01234567":
                value = value * 8 + int(inner[index], 8)
                index += 1
                taken += 1
            out.append(value & 0xFF)
        else:
            out += escape.encode("utf8")
    return out.decode("utf8", errors="replace")


RENAMES = [
    "plain.ts", "a.ts => b.rs", "src/{old => new}/tail.ts", "{ => sub}/file.ts", "{sub => }/file.ts", "pkg/{a => b}", "{a => b}", "x/{a/b => c}/y.go", "\"\\360\\237\\247\\254\\357\\270\\217alpha.ts\" => \"src/\\360\\237\\247\\254\\357\\270\\217beta.ts\"",
    "dir/{\"\\303\\274\" => plain}/f.ts", "a => b => c", "{a => b}/{c => d}.ts", "no arrow {braces}.ts", "trailing\r",
]


def collapse(path):
    return re.sub(r"/{2,}", "/", path)


def resolve_path(raw):
    field = raw.rstrip("\r")
    open_at, close_at = field.find("{"), field.find("}")
    if 0 <= open_at < close_at:
        middle = field[open_at + 1:close_at]
        arrow = middle.find(" => ")
        if arrow >= 0:
            prefix, suffix = field[:open_at], field[close_at + 1:]
            return [unquote(collapse(prefix + middle[arrow + 4:] + suffix)), unquote(collapse(prefix + middle[:arrow] + suffix))]
    arrow = field.find(" => ")
    if arrow >= 0:
        return [unquote(field[arrow + 4:].strip()), unquote(field[:arrow].strip())]
    return [unquote(field), None]


MARKUP = {".html", ".htm", ".xhtml", ".md", ".markdown", ".mdown", ".mkd", ".mdx", ".mdc", ".svx", ".svxtheme"}
DATA = {".json", ".jsonc", ".yaml", ".yml", ".toml", ".csv", ".xml", ".ini", ".cfg", ".conf", ".properties", ".editorconfig", ".gitattributes", ".gitmodules"}
CODE = {".ts": "TypeScript", ".tsx": "TypeScript", ".cts": "TypeScript", ".mts": "TypeScript", ".mtsx": "TypeScript", ".go": "Go", ".cs": "C#", ".py": "Python", ".rs": "Rust"}
FINE = {
    ".ts": "TypeScript", ".tsx": "TypeScript", ".cts": "TypeScript", ".mts": "TypeScript", ".mtsx": "TypeScript", ".js": "JavaScript", ".mjs": "JavaScript", ".cjs": "JavaScript", ".go": "Go", ".cs": "C#", ".py": "Python", ".rs": "Rust",
    ".sh": "Shell", ".bash": "Shell", ".zsh": "Shell", ".ps1": "PowerShell", ".css": "CSS", ".scss": "CSS", ".sass": "CSS", ".sql": "SQL", ".html": "HTML", ".htm": "HTML", ".xhtml": "HTML", ".md": "Markdown", ".markdown": "Markdown",
    ".mdown": "Markdown", ".mkd": "Markdown", ".mdx": "Markdown", ".mdc": "Markdown", ".svx": "Markdown", ".tex": "TeX", ".sty": "TeX", ".cls": "TeX", ".ltx": "TeX", ".bib": "TeX", ".json": "JSON", ".jsonc": "JSON", ".yaml": "YAML",
    ".yml": "YAML", ".toml": "TOML", ".csv": "CSV", ".xml": "XML",
}
PATHS = [
    "a.ts", "a.TSX", "dir/b.go", "c.cs", "d.py", "e.rs", "f.md", "g.MDX", "h.json", "i.jsonc", "j.yaml", "k.toml", "l.csv", "m.xml", "n.ini", "o.cfg", "p.conf", "q.properties", ".editorconfig", ".gitattributes", "x/.gitmodules", "noext", ".hidden",
    "a.js", "a.mjs", "run.sh", "run.PS1", "s.scss", "t.sql", "u.html", "v.tex", "w.bib", "Dockerfile", "Dockerfile.dev", "dockerfile", "Makefile", "justfile", "dir\\win.ts", "./rel/a.go", "a.b.c.rs", "trailing.", "a.mtsx", "z.svxtheme",
]


def extension(path):
    base = re.split(r"[/\\]", path)[-1]
    index = base.rfind(".")
    if index >= 0 and (index > 0 or len(base) > 1):
        return base[index:].lower()
    return ""


def loc_bucket(path):
    ext = extension(path)
    if ext in CODE:
        return CODE[ext]
    if ext in MARKUP:
        return "Markup"
    if ext in DATA:
        return "Data"
    return None


def fine_language(path):
    rel = path.replace("\\", "/")
    rel = rel[2:] if rel.startswith("./") else rel
    base = rel.split("/")[-1].lower()
    if base == "dockerfile" or base.startswith("dockerfile."):
        return "Dockerfile"
    if base in ("makefile", "justfile"):
        return "Makefile"
    return FINE.get(extension(rel))


DURATIONS = ["0.412s", "1.204s", "12ms", "3.5ms", "250us", "250µs", "10ns", "7", " 2.5 s ", "1e3ms", "abc", "", "  ", "ms", "-3s", "1.0.0s", "0s", "5 ms", "1_000ns"]


def duration(text):
    stripped = text.strip()
    if not stripped:
        return None
    for suffix, factor in (("ms", 1e-3), ("us", 1e-6), ("µs", 1e-6), ("ns", 1e-9), ("s", 1.0)):
        if stripped.endswith(suffix):
            number, scale = stripped[: -len(suffix)], factor
            break
    else:
        number, scale = stripped, 1.0
    try:
        if "_" in number:
            return None
        return float(number.strip()) * scale
    except ValueError:
        return None


def build():
    return {
        "schema": "semio.repo.metrics.unit-vectors/1",
        "$comment": "🔬️ Expected answers for the cargo unit tests, computed by Python's datetime/calendar (the UTC arithmetic), `re` and plain models of git's path quoting and the extension tables. Regenerate with the oracle; never edit by hand.",
        "oracle": {"script": ORACLE_SCRIPT, "tool": "python3", "libraries": ["datetime", "calendar", "re"], "model": "UTC time bucketing, git path unquoting, rename resolution, bucket classification, duration parsing"},
        "instants": [keys(u) for u in INSTANTS],
        "unquote": [{"raw": raw, "path": unquote(raw)} for raw in QUOTED],
        "renames": [{"raw": raw, "path": resolve_path(raw)[0], "from": resolve_path(raw)[1]} for raw in RENAMES],
        "classify": [{"path": p, "extension": extension(p), "bucket": loc_bucket(p), "language": fine_language(p)} for p in PATHS],
        "durations": [{"raw": raw, "seconds": duration(raw)} for raw in DURATIONS],
    }


def main():
    text = json.dumps(build(), ensure_ascii=False, indent=1) + "\n"
    if "--check" in sys.argv:
        same = TARGET.exists() and TARGET.read_text(encoding="utf8") == text
        print("SAME" if same else "DIFFERENT")
        sys.exit(0 if same else 1)
    TARGET.write_text(text, encoding="utf8")
    print("written", TARGET, len(text))


main()
