#!/usr/bin/env python3
"""🪪️ Prepared patch (rule 20, apply after W2's `--packages all`): a Scenario Outline row's scenario id is
`<base>-<id cell>`, and the parser silently fell back to the row's 1-based INDEX when a table carried no `id` column.
Exactly three features in the repo had no `id` column (7 032 other rows all carry a kebab-case one), and all three were
red in BOTH roles: bestest (`case-parameters-1` vs the adapters' `case-parameters-600`), epJSON (`schema-validity-1`)
and procedural io (`read-1` vs `read-stl`). Root fix: the platform refuses an outline row without a kebab-case `id`
cell (an index renames every later row when one is inserted), the three features name their rows, and the two energy
adapters (Rust + Python each) register the kebab id of the ASHRAE 140 case (`600ff` for `600FF`).
Registering the bestest rows exposed what the index names had hidden in both roles: every `case-parameters-*` row
asserted a 10 °C ground temperature, yet no surface of any committed case touches the ground — the committed models
AND the committed EnergyPlus translation (`⚡️model.epJSON`) expose the floor to outdoor air with neither sun nor wind
(the raised floor of the current §5.2 base case), and carry no ground-temperature object. The model's 18 °C is the
engine default for a boundary nothing uses, and the Rust subject's projection never carried the member. The reference
now states what §5.2 does say about the floor (every surface outdoor-air, the floor sheltered from sun and wind) and
projects exactly the Rust subject's members.
Usage: patch.py --dry-run | --write [--root <dir>]"""
import re
import sys
from pathlib import Path

root = Path(sys.argv[sys.argv.index("--root") + 1]) if "--root" in sys.argv else Path("/Users/ueli/Documents/semio")
write = "--write" in sys.argv
PLATFORM = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test"
ENERGY = "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests"
PROCEDURAL = "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🚪️io-procedural-3d-1"
edits, problems = {}, []


def replace(path, old, new, count=1):
    """✂️ Replaces `old` exactly `count` times in the pending text of `path`, recording a problem otherwise."""
    source = edits.get(path) or (root / path).read_text(encoding="utf-8")
    if source.count(old) != count:
        problems.append(f"{path}: expected {count} of {old[:80]!r}, found {source.count(old)}")
        return
    edits[path] = source.replace(old, new)


def tables(path, transform):
    """📋️ Rewrites every Examples table of a feature through `transform(header, rows) -> (header, rows)` and re-pads
    each column to its widest cell."""
    lines = (edits.get(path) or (root / path).read_text(encoding="utf-8")).split("\n")
    at, changed = 0, 0
    while at < len(lines):
        if lines[at].strip() != "Examples:":
            at += 1
            continue
        start = end = at + 1
        while end < len(lines) and lines[end].strip().startswith("|"):
            end += 1
        indent = lines[start][: len(lines[start]) - len(lines[start].lstrip())]
        cells = [[cell.strip() for cell in line.strip()[1:-1].split("|")] for line in lines[start:end]]
        header, rows = transform(cells[0], cells[1:])
        grid = [header] + rows
        widths = [max(len(row[column]) for row in grid) for column in range(len(header))]
        lines[start:end] = [indent + "| " + " | ".join(cell.ljust(width) for cell, width in zip(row, widths)) + " |" for row in grid]
        changed += 1
        at = start + len(grid)
    edits[path] = "\n".join(lines)
    return changed


def case_ids(header, rows):
    """🏛️ `| case |` → `| id | case |`: the kebab id of the ASHRAE 140 case keys the scenario, the case keeps naming
    the committed model directory (`🏛️bestest-600FF`)."""
    if header != ["case"]:
        problems.append(f"energy table header {header}")
        return header, rows
    return ["id", "case"], [[row[0].lower(), row[0]] for row in rows]


def format_ids(header, rows):
    """🚪️ `format` IS the row's id (`read-stl`, `round-trip-obj`): the column is renamed, nothing else moves."""
    if header[0] != "format":
        problems.append(f"procedural table header {header}")
        return header, rows
    return ["id"] + header[1:], rows


replace(f"{PLATFORM}/🟦️.ts", """  const row = example?.row ?? {};
  const id = example === null ? baseId : `${baseId}-${row.id ?? String(example.index + 1)}`;
""", """  const row = example?.row ?? {};
  if (example !== null && !/^[a-z0-9]+(?:-[a-z0-9]+)*$/.test(row.id ?? "")) {
    errors.push(`Scenario Outline ${where} Examples row ${example.index + 1} needs a kebab-case \\`id\\` cell: its scenario id is @id-${baseId}-<id>, and a row index would rename every later row when one is inserted`);
    return [];
  }
  const id = example === null ? baseId : `${baseId}-${row.id}`;
""")
replace(f"{PLATFORM}/🧪️tests/🧪️test-platform/🟦️.ts", """  test("duplicate scenario ids are rejected", () => {""", """  test("an outline row without a kebab-case id cell is an error rather than an index-named scenario", () => {
    const outline = (header: string, row: string): string => `@capability-x @no-oracle-y @comparison-ordered-json-v1\\nFeature: F\\n  @id-read @level-quick @mode-differential\\n  Scenario Outline: Read <${header}>\\n    Given <${header}>\\n    Examples:\\n      | ${header} |\\n      | ${row} |\\n`;
    for (const [header, row] of [["format", "stl"], ["id", "600FF"], ["id", ""]] as const) {
      const feature = parseFeature(outline(header, row));
      expect(feature.scenarios).toHaveLength(0);
      expect(feature.errors.some((error) => error.includes("needs a kebab-case `id` cell"))).toBe(true);
    }
    expect(parseFeature(outline("id", "stl")).scenarios.map((scenario) => scenario.id)).toEqual(["read-stl"]);
  });

  test("duplicate scenario ids are rejected", () => {""")
for name, expected in (("🏛️simulate-bestest-energyplus", 5), ("🏛️export-epjson-runs-in-energyplus", 3)):
    feature = f"{ENERGY}/{name}/🥒️.feature"
    if tables(feature, case_ids) != expected:
        problems.append(f"{feature}: expected {expected} Examples tables")
feature = f"{PROCEDURAL}/🥒️.feature"
if tables(feature, format_ids) != 2:
    problems.append(f"{feature}: expected 2 Examples tables")
source = edits.get(feature, "")
if source.count("<format>") != 4:
    problems.append(f"{feature}: expected 4 <format> placeholders (2 names, 2 doc strings), found {source.count('<format>')}")
edits[feature] = source.replace("<format>", "<id>")
BASES = {"🏛️simulate-bestest-energyplus": ("case-parameters", "annual-energy", "peak-load", "free-float", "hourly-temperature"), "🏛️export-epjson-runs-in-energyplus": ("schema-validity", "nothing-dropped", "energyplus-run")}
for name, bases in BASES.items():
    for base in bases:
        replace(f"{ENERGY}/{name}/🦀️.rs", f'&format!("{base}-{{case}}")', f'&format!("{base}-{{}}", case.to_ascii_lowercase())', count=len(re.findall(re.escape(f'&format!("{base}-{{case}}")'), (root / ENERGY / name / "🦀️.rs").read_text(encoding="utf-8"))) or 1)
replace(f"{ENERGY}/🏛️simulate-bestest-energyplus/🐍️.py", '{case}", ', '{case.lower()}", ', count=5)
replace(f"{ENERGY}/🏛️export-epjson-runs-in-energyplus/🐍️.py", '-%s" % case,', '-%s" % case.lower(),', count=3)
BESTEST = f"{ENERGY}/🏛️simulate-bestest-energyplus"
replace(f"{BESTEST}/🐍️.py", "WINDOW_U_W_M2K = 3.0\nGROUND_TEMPERATURE_C = 10.0\n", "WINDOW_U_W_M2K = 3.0\n")
replace(f"{BESTEST}/🐍️.py", '        "groundTemperatureC": model["ground_temperature"]["building_surface_c"][0],\n', "")
replace(f"{BESTEST}/🐍️.py", '''def assert_matches_the_standard(case, derived):
    """🏛️ The independent half: hold the derived quantities against §5.2's own published values."""
''', '''def assert_matches_the_standard(case, derived, model):
    """🏛️ The independent half: hold the derived quantities against §5.2's own published values, and the floor to
    §5.2's raised floor — exposed to outdoor air, sheltered from sun and wind, no surface coupled to the ground."""
''')
replace(f"{BESTEST}/🐍️.py", '''    _close(derived["groundTemperatureC"], GROUND_TEMPERATURE_C, 1e-9, f"case {case} ground temperature")
''', '''    for surface in model["surfaces"]:
        assert surface["outside_boundary_condition"] == "OutdoorAir", f"case {case} {surface['name']}: §5.2 couples no surface to the ground, found {surface['outside_boundary_condition']!r}"
    floors = [surface for surface in model["surfaces"] if surface["class"] == "Floor"]
    assert len(floors) == 1 and not floors[0]["sun_exposed"] and not floors[0]["wind_exposed"], f"case {case}: §5.2's one raised floor sees neither sun nor wind, found {floors!r}"
''')
replace(f"{BESTEST}/🐍️.py", "        assert_matches_the_standard(case, derived)\n", "        assert_matches_the_standard(case, derived, model)\n")
replace(f"{BESTEST}/🥒️.feature", "    Then the areas, air-to-air U-values, glazing, infiltration rate, internal gain and ground temperature agree\n", "    Then the areas, air-to-air U-values, glazing, infiltration rate and internal gain agree, and the floor is a raised floor over outdoor air\n")
print(f"files={len(edits)} problems={len(problems)} write={write}")
for problem in problems:
    print("PROBLEM", problem)
if write and not problems:
    for path, text in edits.items():
        (root / path).write_text(text, encoding="utf-8")
