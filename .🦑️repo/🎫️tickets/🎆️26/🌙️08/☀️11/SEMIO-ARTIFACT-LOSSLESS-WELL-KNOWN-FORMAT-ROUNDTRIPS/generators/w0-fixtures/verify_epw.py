#!/usr/bin/env python3
"""Re-parse the EPW fixture: confirm exactly 8 header lines with expected
leading keywords, then 24 data records each with exactly 35 comma-separated
fields, and spot-check a few field ranges for plausibility."""
import sys

path = sys.argv[1]
with open(path, "r", newline="") as fh:
    raw = fh.read()

lines = raw.split("\r\n")
if lines and lines[-1] == "":
    lines = lines[:-1]

EXPECTED_HEADER_KEYWORDS = [
    "LOCATION", "DESIGN CONDITIONS", "TYPICAL/EXTREME PERIODS",
    "GROUND TEMPERATURES", "HOLIDAYS/DAYLIGHT SAVINGS",
    "COMMENTS 1", "COMMENTS 2", "DATA PERIODS",
]

header = lines[:8]
records = lines[8:]

print(f"Total lines: {len(lines)} (8 header + {len(records)} records)")
for i, (line, kw) in enumerate(zip(header, EXPECTED_HEADER_KEYWORDS), start=1):
    starts_ok = line.startswith(kw + ",") or line == kw
    print(f"  header line {i}: starts_with('{kw}')={starts_ok}  -> {line[:60]}...")
    assert starts_ok, f"header line {i} does not start with expected keyword {kw}"

loc_fields = header[0].split(",")
print(f"LOCATION fields ({len(loc_fields)}): {loc_fields}")
assert len(loc_fields) == 10, f"LOCATION must have 10 fields, got {len(loc_fields)}"

assert len(records) == 24, f"expected 24 hourly records, got {len(records)}"

for idx, rec in enumerate(records):
    fields = rec.split(",")
    assert len(fields) == 35, f"record {idx} (hour field {fields[3] if len(fields)>3 else '?'}) has {len(fields)} fields, expected 35"
    hour = int(fields[3])
    assert hour == idx + 1, f"record {idx} hour field is {hour}, expected {idx+1}"
    dry_bulb = float(fields[6])
    rh = float(fields[8])
    assert -50 < dry_bulb < 60, f"implausible dry_bulb_temp {dry_bulb} at hour {hour}"
    assert 0 <= rh <= 100, f"implausible relative_humidity {rh} at hour {hour}"
    ghr = float(fields[13])
    assert ghr >= 0

print(f"\nAll {len(records)} records have exactly 35 columns; hour sequence 1..24 confirmed; "
      f"dry-bulb/RH/radiation ranges plausible.")
print("\nALL EPW STRUCTURAL ASSERTIONS PASSED")
