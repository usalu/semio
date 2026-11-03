import json
import os
import re
import sys
import time
from pathlib import Path

ROOT = Path(r"C:\git\semio")
ARCH = ROOT / "🎓️teaching" / "🏛️architecture"
MANIFEST = ARCH / "⚡️energy" / "🏗️authoring" / "🎨️visuals" / "🧍️staffage" / "🔣️.json"
APPLY = "--apply" in sys.argv

GROUPS = {
    "shared-person-winter": ["heating-fundamentals-occupant", "heating-internal-gains-standing-adult", "demand-system-losses-person", "demand-regulations-person", "demand-power-person", "demand-primary-energy-person", "demand-certificate-person", "demand-certificate-walker", "demand-practice-person", "demand-recap-person", "physics-fundamentals-person", "cooling-heating-comparison-occupant", "heating-conduction-person", "heating-calculation-person"],
    "shared-person-neutral": ["demand-practice-talker", "demand-certificate-tenant", "shared-watt-person"],
    "shared-person-pathlines": ["ventilation-need-occupant", "ventilation-natural-occupant", "ventilation-volume-occupant", "ventilation-mechanical-occupant"],
    "shared-person-fanning": ["cooling-heating-comparison-occupant-fanning", "cooling-summer-strategy-occupant"],
    "shared-person-relaxed-line": ["bim-consistency-reviewer", "bim-exchange-imposed-person"],
    "shared-lamp-winter": ["heating-internal-gains-pendant-lamp", "demand-regulations-lamp", "demand-power-lamp", "demand-certificate-lamp", "demand-recap-lamp", "physics-fundamentals-lamp", "demand-practice-lamp-on", "cooling-heating-comparison-lamp", "heating-calculation-lamp"],
    "shared-lamp-summer": ["cooling-internal-gains-lamp", "cooling-heating-comparison-lamp-summer"],
    "shared-lamp-neutral": ["optimization-room-lamp-on", "optimization-index-lamp"],
    "shared-lamp-off-winter": ["demand-practice-lamp-off"],
    "shared-lamp-off-neutral": ["optimization-room-lamp-off"],
    "shared-desk": ["cooling-internal-gains-desk", "heating-internal-gains-desk", "demand-recap-desk", "cooling-heating-comparison-desk", "heating-calculation-desk"],
    "shared-radiator-front-winter": ["heating-fundamentals-panel-radiator", "optimization-extrema-radiator", "demand-primary-energy-radiator", "demand-practice-radiator", "demand-certificate-radiator", "physics-fundamentals-radiator-front"],
    "shared-radiator-winter": ["demand-system-losses-radiator", "demand-regulations-radiator", "demand-power-radiator", "demand-recap-radiator", "physics-fundamentals-radiator"],
    "shared-radiator-thermostat-front": ["heating-conduction-radiator", "heating-calculation-radiator-front"],
    "shared-laptop-winter": ["heating-internal-gains-laptop", "demand-recap-laptop"],
    "shared-blind-lowered": ["physics-fundamentals-blind", "heating-solar-gains-blind"],
    "shared-toaster-winter": ["cooling-heating-comparison-toaster", "heating-conduction-toaster"],
    "shared-mvhr-normal": ["ventilation-need-mvhr", "ventilation-volume-mvhr"],
    "shared-boiler-heating-cycle": ["demand-system-losses-boiler", "demand-primary-energy-boiler", "demand-practice-boiler", "heating-calculation-boiler"],
    "shared-tank-charge-up": ["demand-system-losses-tank", "heating-calculation-tank"],
    "shared-pump-run-high": ["demand-primary-energy-pump", "demand-practice-pump"],
    "shared-gas-meter-run": ["demand-system-losses-gas-meter", "demand-primary-energy-gas-meter", "demand-practice-gas-meter"],
    "shared-electricity-meter-run": ["demand-system-losses-electricity-meter", "demand-primary-energy-electricity-meter"],
    "shared-heat-pump-heating-sweep": ["demand-system-losses-heat-pump", "demand-primary-energy-heat-pump"],
}
NEUTRAL = {"ventilation-mechanical-occupant", "ventilation-volume-cook", "ventilation-volume-hood", "ventilation-volume-operating-table", "physics-fundamentals-vacuum", "ventilation-natural-radiator-on", "ventilation-natural-radiator-off"}
MAP = {old: new for new, olds in GROUPS.items() for old in olds}

LINE = re.compile(r'^(\s*)"([^"]+)":\s*(\{.*\})(,?)\s*$')


def fmt(entry):
    return "{ " + ", ".join(f'"{k}": {json.dumps(v, ensure_ascii=False)}' for k, v in entry.items()) + " }"


def merge_manifest():
    text = MANIFEST.read_bytes().decode("utf-8")
    eol = "\r\n" if "\r\n" in text else "\n"
    lines = text.split(eol)
    entries = {}
    for line in lines:
        m = LINE.match(line)
        if m:
            entries[m.group(2)] = json.loads(m.group(3))
    out, placed, last = [], set(), None
    for index, line in enumerate(lines):
        m = LINE.match(line)
        if not m:
            out.append(line)
            continue
        cid = m.group(2)
        entry = dict(entries[cid])
        if cid in NEUTRAL:
            entry["environment"] = "office-neutral"
        if cid in MAP:
            new = MAP[cid]
            if new in placed:
                continue
            placed.add(new)
            if entry.get("fps") == 15:
                entry.pop("fps")
            out.append((m.group(1), new, entry))
        else:
            out.append((m.group(1), cid, entry))
    rows = [i for i, x in enumerate(out) if isinstance(x, tuple)]
    result = []
    for i, x in enumerate(out):
        if isinstance(x, tuple):
            comma = "," if i != rows[-1] else ""
            result.append(f'{x[0]}"{x[1]}": {fmt(x[2])}{comma}')
        else:
            result.append(x)
    return eol.join(result), text


def token(old):
    return re.compile(r"(?<![\w-])" + re.escape(old) + r"(?![\w-])")


ANY = re.compile("|".join(re.escape(old) for old in sorted(MAP, key=len, reverse=True)))
PATTERNS = [(token(old), new) for old, new in sorted(MAP.items(), key=lambda kv: -len(kv[0]))]


def files():
    for base, dirs, names in os.walk(ARCH):
        dirs[:] = [d for d in dirs if d not in {"node_modules", "__pycache__", ".git", "🗑️generated"} and not (Path(base) == ARCH and d == "🧍️staffage")]
        for n in names:
            if n.endswith((".py", ".json", ".ts", ".tsx", ".md")):
                p = Path(base) / n
                if p != MANIFEST:
                    yield p


def main():
    new_manifest, old_manifest = merge_manifest()
    json.loads(new_manifest)
    changed = []
    for p in files():
        data = p.read_bytes()
        try:
            text = data.decode("utf-8")
        except UnicodeDecodeError:
            continue
        if not ANY.search(text):
            continue
        new = text
        for pattern, repl in PATTERNS:
            new = pattern.sub(repl, new)
        if new != text:
            changed.append(p)
            if APPLY:
                for attempt in range(20):
                    try:
                        p.write_bytes(new.encode("utf-8"))
                        break
                    except OSError as error:
                        print("retry", p.name, error)
                        time.sleep(1.0)
    if APPLY:
        current = MANIFEST.read_bytes().decode("utf-8")
        if current != old_manifest:
            print("manifest changed under me; rerun")
            return 1
        MANIFEST.write_bytes(new_manifest.encode("utf-8"))
    for p in changed:
        print(p.relative_to(ARCH).as_posix())
    print(len(changed), "files;", len(MAP), "old ids;", len(GROUPS), "new ids;", "applied" if APPLY else "dry run")
    return 0


sys.exit(main())
