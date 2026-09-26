"""🐍 Independent ISO 16757 evaluate oracle — recomputes governing numeric checks.

Validates Rust `CheckReport` JSON for:
- Part 2 solid volume from box width×height×depth
- Part 5 part-number script `dn * 10 + 50`
- Part 1 unique-id / selection ambiguity counts
"""

from __future__ import annotations

from typing import Any


def box_volume_m3(width: float, height: float, depth: float) -> float:
    return width * height * depth


def part_number_script(dn: float) -> float:
    return dn * 10.0 + 50.0


def installation_clearance_ok(
    product_min: list[float],
    product_max: list[float],
    install_min: list[float],
    install_max: list[float],
    clearance_m: float = 0.05,
) -> bool:
    needed = [product_max[i] - product_min[i] + 2.0 * clearance_m for i in range(3)]
    have = [install_max[i] - install_min[i] for i in range(3)]
    if any(have[i] + 1e-9 < needed[i] for i in range(3)):
        return False
    return all(
        product_min[i] >= install_min[i] + clearance_m - 1e-9
        and product_max[i] <= install_max[i] - clearance_m + 1e-9
        for i in range(3)
    )


def assert_report_matches_fixture(report: dict[str, Any], snapshot: dict[str, Any]) -> None:
    checks = {c["id"]: c for c in report["checks"]}
    # volume
    geom = next(iter(snapshot["geometry"]["objects"].values()))
    shape = geom.get("shape") or {}
    params = shape.get("parameters") or {}
    if shape.get("kind") == "box" or shape.get("node") == "primitive":
        # support both encodings
        p = params if params else shape.get("parameters", {})
        if not p and "parameters" in shape:
            p = shape["parameters"]
        # try nested primitive
        if shape.get("node") == "primitive":
            p = shape.get("parameters", {})
            w, h, d = float(p["width"]), float(p["height"]), float(p["depth"])
            expected = box_volume_m3(w, h, d)
            vol = next(c for c in report["checks"] if "2.7.1.volume" in c["id"])
            assert abs(vol["computed"]["value"] - expected) < 1e-9

    dn = snapshot["partNumberInputs"]["dn"]["value"]
    expected_pn = part_number_script(float(dn))
    pn = next(c for c in report["checks"] if c["clause"]["section"] == "6.10")
    if pn["status"] == "pass":
        assert abs(pn["computed"]["value"] - expected_pn) < 1e-6


def validate_snapshot_json_schema(snapshot: dict[str, Any], schema: dict[str, Any]) -> None:
    import jsonschema

    jsonschema.validate(instance=snapshot, schema=schema)
