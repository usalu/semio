"""🔎️ Scan the staffage-calling clip files for device state words and unit strings, printing file:line and the source line."""
import re
import sys
from pathlib import Path

ROOT = Path(r"C:\git\semio")
GEN = ROOT / ".🧬semio" / "🦑️repo" / "🎫️tickets" / "🎆️26" / "🌙️11" / "☀️01" / "HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS" / "🗑️generated" / "ct"
STATE = re.compile(r"""["'](an|aus|läuft|on|off|Ein|Aus|Betrieb|Standby)["'\s,)]|\b(an|aus|läuft|on|off)\b""")
UNIT = re.compile(r"""(\d|\})\s?(kW|kWh|W|°C|%|m²|m³|Wh)\b|\\mathrm\{(W|kWh|°C)\}""")
PLAIN_UNIT_NOSPACE = re.compile(r"""\d(kW|kWh|W|°C|%)\b""")


def main():
    files = [l.strip() for l in (GEN / "staffage-files.txt").read_text(encoding="utf-8").splitlines() if l.strip()]
    out = []
    for rel in files:
        for i, line in enumerate((ROOT / rel).read_text(encoding="utf-8").splitlines(), 1):
            s = line.strip()
            if s.startswith("#") or ('"' not in s and "'" not in s):
                continue
            hits = []
            if STATE.search(s):
                hits.append("STATE")
            if UNIT.search(s):
                hits.append("UNIT")
            if PLAIN_UNIT_NOSPACE.search(s):
                hits.append("NOSPACE")
            if hits:
                out.append(f"{','.join(hits)} {rel}:{i} | {s[:170]}")
    (GEN / "states.txt").write_text("\n".join(out), encoding="utf-8")
    print(len(out))


if __name__ == "__main__":
    sys.exit(main())
