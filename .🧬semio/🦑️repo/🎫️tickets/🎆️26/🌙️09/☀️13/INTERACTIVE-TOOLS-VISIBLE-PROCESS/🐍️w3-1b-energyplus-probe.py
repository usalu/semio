"""🔬️ W3-1b EnergyPlus bisection probe: native ASHRAE 140 IDF → physics variants → hourly/timestep JSON.

usage: python3 🐍️w3-1b-energyplus-probe.py <native-run-dir> <out.json> [--drop windows,gains,infiltration,hvac] [--timesteps N]
       [--epw-edit zero-solar|zero-wind|const-temp:<C>] [--var "Name@Frequency" ...] [--set "Object|Name|field_index|value" ...]
"""

import argparse
import json
import os
import re
import sqlite3
import subprocess
from pathlib import Path

ROOT = Path("/Users/ueli/Documents/semio")
EPLUS = ROOT / ".🧬semio/🦑️repo/⚡️cache/oracles/openstudio-3.11.0-darwin-arm64/EnergyPlus"
EPW = ROOT / "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🌦️denver-tmy/🌦️.epw"

DROPS = {
    "windows": ["FenestrationSurface:Detailed"],
    "gains": ["ElectricEquipment"],
    "infiltration": ["ZoneInfiltration:DesignFlowRate"],
    "hvac": ["ZoneControl:Thermostat", "ThermostatSetpoint:DualSetpoint", "ZoneHVAC:EquipmentConnections", "ZoneHVAC:IdealLoadsAirSystem", "ZoneHVAC:EquipmentList", "NodeList"],
}


def parse(text):
    text = re.sub(r"!.*", "", text)
    return [[field.strip() for field in chunk.split(",")] for chunk in text.split(";") if chunk.strip()]


def write(objects):
    return "\n".join(",\n  ".join(fields) + ";" for fields in objects) + "\n"


def edit_epw(source, mode, target):
    lines = source.read_text().splitlines()
    out = []
    for index, line in enumerate(lines):
        if index < 8:
            out.append(line)
            continue
        fields = line.split(",")
        if mode == "zero-solar":
            for column in (13, 14, 15, 16, 17, 18, 19):
                fields[column] = "0"
        elif mode == "zero-wind":
            fields[21] = "0.0"
        elif mode.startswith("const-temp:"):
            fields[6] = mode.split(":")[1]
            fields[7] = str(float(mode.split(":")[1]) - 10.0)
        elif mode == "const-ir:300":
            fields[12] = "300"
        out.append(",".join(fields))
    target.write_text("\n".join(out) + "\n")


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("base")
    parser.add_argument("out")
    parser.add_argument("--drop", default="")
    parser.add_argument("--timesteps", type=int, default=None)
    parser.add_argument("--epw-edit", action="append", default=[])
    parser.add_argument("--var", action="append", default=[])
    parser.add_argument("--set", action="append", default=[])
    args = parser.parse_args()
    objects = parse((Path(args.base) / "in.idf").read_text())
    dropped = {kind for name in filter(None, args.drop.split(",")) for kind in DROPS[name]}
    objects = [fields for fields in objects if fields[0] not in dropped and fields[0] != "Output:Variable"]
    for fields in objects:
        if fields[0] == "Timestep" and args.timesteps:
            fields[1] = str(args.timesteps)
    for spec in args.set:
        kind, name, index, value = spec.split("|")
        for fields in objects:
            if fields[0] == kind and (name == "*" or fields[1] == name):
                while len(fields) <= int(index):
                    fields.append("")
                fields[int(index)] = value
    variables = args.var or ["Zone Mean Air Temperature@Hourly", "Zone Ideal Loads Supply Air Total Heating Energy@Hourly", "Zone Ideal Loads Supply Air Total Cooling Energy@Hourly"]
    for spec in variables:
        name, frequency = spec.split("@")
        objects.append(["Output:Variable", "*", name, frequency])
    work = Path(args.out).with_suffix(".run")
    work.mkdir(parents=True, exist_ok=True)
    (work / "in.idf").write_text(write(objects))
    epw = EPW
    for index, mode in enumerate(args.epw_edit):
        target = work / f"edited-{index}.epw"
        edit_epw(epw, mode, target)
        epw = target
    result = subprocess.run([str(EPLUS / "energyplus"), "-w", str(epw), "-i", str(EPLUS / "Energy+.idd"), "-d", str(work), str(work / "in.idf")], capture_output=True, text=True)
    if result.returncode != 0:
        print(result.stdout[-3000:], (work / "eplusout.err").read_text()[-3000:] if (work / "eplusout.err").exists() else "")
        raise SystemExit(result.returncode)
    connection = sqlite3.connect(work / "eplusout.sql")
    rows = connection.execute(
        "select d.Name, d.KeyValue, d.ReportingFrequency, r.Value from ReportData r join ReportDataDictionary d on r.ReportDataDictionaryIndex = d.ReportDataDictionaryIndex join Time t on r.TimeIndex = t.TimeIndex where t.EnvironmentPeriodIndex = (select max(EnvironmentPeriodIndex) from EnvironmentPeriods) order by r.TimeIndex"
    ).fetchall()
    document = {}
    for name, key, frequency, value in rows:
        document.setdefault(f"{name}|{key}|{frequency}", []).append(value)
    Path(args.out).write_text(json.dumps(document))
    summary = {key: round(sum(values) / 3.6e6, 3) if key.endswith("Energy|ZONE_ONE IDEAL LOADS AIR SYSTEM|Hourly") else (round(min(values), 3), round(max(values), 3), round(sum(values) / len(values), 3)) for key, values in document.items()}
    for key, value in summary.items():
        print(key, value)


if __name__ == "__main__":
    main()
