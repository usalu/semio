import os, re, sys, time, glob
C = sys.argv[1]
GEN = os.path.join(C, "🗑️generated", "1030")
OUT = os.path.join(C, "1030-audit-errcensus.md")
WINDOW = 30 * 60
now = time.time()
LANES = [("pasync", "async/value-retirement-owner"), ("prepl", "replication-mutation-retained-field"),
         ("psnap", "value-snapshot-read"), ("ppack", "pack-json"), ("pui", "ui-locale/ui-raster-owner"),
         ("pio", "io-control/io-sqlite-model")]
CC = re.compile(r"could not compile `([^`]+)`")
COMP = re.compile(r"^\s*Compiling (\S+) v")
ERR = re.compile(r"^error\[(E\d+)\]: (.*)")
LOC = re.compile(r"-->\s+(.+?):(\d+):(\d+)")
rows = {}
crate_hits = {}
files = [f for f in glob.glob(os.path.join(GEN, "*.txt")) if now - os.path.getmtime(f) <= WINDOW]
OWNERS = [
    ("pasync", ["framework-async", "value-retirement-owner", "tool-run-entities", "io-ports"]),
    ("prepl", ["replication-"]),
    ("psnap", ["value-snapshot-read"]),
    ("ppack", ["pack-json"]),
    ("pui", ["ui-locale", "ui-raster-owner", "value-snapshot-projection", "value-original-slot", "value-retained-map"]),
    ("pio", ["io-control", "io-sqlite-model"]),
    ("hubplug", ["value-edit"]),
    ("stdiofix", ["dsl-record-model"]),
    ("runtime", ["os-presence-native-snapshot", "plugin-instance", "decl-app", "window-config"]),
    ("fwfix", ["value-original-aliases"]),
]
def lane_of(crate):
    c = crate.replace("(in progress) ", "")
    for ln, pats in OWNERS:
        if any(p in c for p in pats):
            return ln
    return "UNASSIGNED"
def add(crate, lane, name, code, loc):
    lane = lane_of(crate)
    r = rows.setdefault((crate, lane), {"count": 0, "first": None, "logs": set(), "codes": set(), "sites": set()})
    r["logs"].add(name)
    site = (code, loc[0], loc[1])
    if site in r["sites"]:
        return
    r["sites"].add(site)
    r["count"] += 1
    r["codes"].add(code)
    r["logs"].add(name)
    if r["first"] is None:
        r["first"] = f"{loc[0]}:{loc[1]} ({code} in {name})"
for f in sorted(files):
    name = os.path.basename(f)
    cur = None
    buf = []
    pending = None
    with open(f, encoding="utf-8", errors="replace") as fh:
        for line in fh:
            line = line.rstrip("\n")
            m = COMP.match(line)
            if m: cur = m.group(1)
            m = CC.search(line)
            if m:
                crate = m.group(1).split(" ")[0]
                crate_hits.setdefault(crate, set()).add(name)
                for (code, loc) in buf:
                    add(crate, None, name, code, loc)
                buf = []
                continue
            m = ERR.match(line)
            if m:
                pending = m.group(1)
                continue
            if pending:
                m = LOC.search(line)
                if m:
                    buf.append((pending, (m.group(1), m.group(2))))
                    pending = None
    for (code, loc) in buf:
        add("(in progress) " + (cur or "UNKNOWN-CRATE"), None, name, code, loc)
ts = time.strftime("%Y-%m-%d %H:%M:%S %Z")
lines = [f"# 1030 Error Census (errcensus lane)", "", f"Timestamp: {ts}", f"Window: logs in 🗑️generated/1030 modified in last 30 min ({len(files)} files scanned)", ""]
lines.append("## Crate error table (UNASSIGNED first)")
lines.append("")
lines.append("| Crate | Owning lane | Unique error sites | Error codes | First error file:line | Logs |")
lines.append("|---|---|---|---|---|---|")
order = sorted(rows.items(), key=lambda kv: (kv[0][1] != "UNASSIGNED", kv[0][0]))
for (crate, lane), r in order:
    lines.append(f"| `{crate}` | {lane} | {r['count']} | {', '.join(sorted(r['codes']))} | {r['first']} | {', '.join(sorted(r['logs']))} |")
if not order:
    lines.append("| (none) | - | 0 | - | - | - |")
lines.append("")
lines.append("## could not compile hits (crate, logs)")
lines.append("")
for c in sorted(crate_hits):
    lines.append(f"- `{c}`: {', '.join(sorted(crate_hits[c]))}")
if not crate_hits:
    lines.append("- none")
lines.append("")
lines.append("Counts are unique (error code, file, line) sites per crate and lane; attribution uses the crate reported by the nearest `could not compile` line after the error, which is approximate when cargo interleaves parallel crates.")
lines.append("Owner map (by crate, coordinator 1030 update): pasync = async, value-retirement-owner, tool-run-entities, io-ports; prepl = replication-*; psnap = value-snapshot-read; ppack = pack-json; pui = ui-locale, ui-raster-owner, value-snapshot-projection, value-original-slot, value-retained-map; pio = io-control, io-sqlite-model; hubplug = value-edit; stdiofix = dsl-record-model; runtime = os-presence-native-snapshot, plugin-instance-*, decl-app, window-config; fwfix = value-original-aliases + integrator. Unmatched crates are UNASSIGNED.")
open(OUT, "w", encoding="utf-8").write("\n".join(lines) + "\n")
print("\n".join(lines))
