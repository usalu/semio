#!/usr/bin/env python3
"""🐍️ `s.wfc.grid2d` reference — an INDEPENDENT second implementation of the document model and of all
fourteen mutations' diff/apply/inverse semantics, and the replay the `🥒️.feature` scenarios are
measured against.

Nothing here reads the Rust crate: the shapes are transcribed from the normative JSON Schema leaves
under `../../🧬️schema/`, and `apply`/`inverse` re-derive the same answers the Rust `🔺️diff`/`↩️inverse`
leaves compute. Run it directly (`python3 🐍️.py`) to replay every committed quintet under
`../../🧫️fixtures/🧬️mutations/`; it exits non-zero on the first divergence.

Canonical JSON form (asserted by the Rust `every_committed_vector_is_canonical_json` test):
  * keys in Rust struct declaration order, camelCase;
  * every float carries a decimal point (`5` is never canonical for a float-typed field);
  * a tagged enum leads with `kind`;
  * `Option` fields with `skip_serializing_if = "Option::is_none"` are omitted when unset;
  * a diff states EVERY lane, including the untouched ones.
"""
from __future__ import annotations

import pathlib
import sys

import copy
import json
from typing import Any, Callable

SCHEMA = "s.wfc.grid2d"
DIRECTIONS = ("LEFT", "RIGHT", "TOP", "BOTTOM")


#region 🔢 canonical floats
class Float(float):
    """🔢 A float that always prints with a decimal point — the fixed point the canonical test asserts."""


class CanonicalEncoder(json.JSONEncoder):
    def iterencode(self, o, _one_shot=False):
        for chunk in super().iterencode(o, _one_shot):
            yield chunk


def _format_float(value: float) -> str:
    if value != value or value in (float("inf"), float("-inf")):
        raise ValueError("non-finite floats are not canonical document content")
    text = repr(float(value))
    if "e" in text or "E" in text:
        return text
    if "." not in text:
        text += ".0"
    return text


def dumps(value: Any) -> str:
    """🖨️ Canonical pretty JSON: two-space indent, floats with an explicit decimal point."""
    return _dump(value, 0) + "\n"


def _dump(value: Any, depth: int) -> str:
    pad = "  " * depth
    inner = "  " * (depth + 1)
    if isinstance(value, Float):
        return _format_float(value)
    if isinstance(value, bool):
        return "true" if value else "false"
    if value is None:
        return "null"
    if isinstance(value, float):
        return _format_float(value)
    if isinstance(value, int):
        return str(value)
    if isinstance(value, str):
        return json.dumps(value, ensure_ascii=False)
    if isinstance(value, list):
        if not value:
            return "[]"
        rows = ",\n".join(inner + _dump(item, depth + 1) for item in value)
        return "[\n" + rows + "\n" + pad + "]"
    if isinstance(value, dict):
        if not value:
            return "{}"
        rows = ",\n".join(f"{inner}{json.dumps(key, ensure_ascii=False)}: {_dump(item, depth + 1)}" for key, item in value.items())
        return "{\n" + rows + "\n" + pad + "}"
    raise TypeError(f"not canonical JSON content: {value!r}")
#endregion


#region 🧬 document model
def color(r: int, g: int, b: int, a: int = 255) -> dict:
    return {"r": r, "g": g, "b": b, "a": a}


def point(x: float, y: float) -> dict:
    return {"x": Float(x), "y": Float(y)}


def vector_media(paths: list[dict]) -> dict:
    return {"kind": "vector", "paths": paths}


def bitmap_media(width: int, height: int, palette: list[dict], pixels: str) -> dict:
    return {"kind": "bitmap", "width": width, "height": height, "palette": palette, "pixels": pixels}


def path(segments: list[dict], fill: dict | None = None, stroke: dict | None = None, stroke_width: float = 0.2) -> dict:
    out: dict[str, Any] = {"segments": segments}
    if fill is not None:
        out["fill"] = fill
    if stroke is not None:
        out["stroke"] = stroke
    out["strokeWidth"] = Float(stroke_width)
    return out


def tile(tile_id: str, weight: float, media: dict, label: str | None = None) -> dict:
    out: dict[str, Any] = {"id": tile_id}
    if label is not None:
        out["label"] = label
    out["weight"] = Float(weight)
    out["media"] = media
    return out


def rule(rule_id: str, a: str, b: str, direction: str, allowed: bool) -> dict:
    assert direction in DIRECTIONS
    return {"id": rule_id, "tileAId": a, "tileBId": b, "direction": direction, "allowed": allowed}


def pinned(x: int, y: int, tile_id: str) -> dict:
    return {"x": x, "y": y, "tileId": tile_id}


def cell(x: int, y: int) -> dict:
    return {"x": x, "y": y}


def snapshot(*, seed: int, width: int, height: int, cell_width: float, cell_height: float, periodic_x: bool, periodic_y: bool, tiles: list[dict], rules: list[dict], pins: list[dict], masks: list[dict]) -> dict:
    return {
        "schema": SCHEMA,
        "seed": seed,
        "width": width,
        "height": height,
        "cellWidth": Float(cell_width),
        "cellHeight": Float(cell_height),
        "periodicX": periodic_x,
        "periodicY": periodic_y,
        "tiles": sorted(tiles, key=lambda row: row["id"]),
        "rules": sorted(rules, key=lambda row: row["id"]),
        "pinned": sorted(pins, key=lambda row: (row["y"], row["x"])),
        "masked": sorted(masks, key=lambda row: (row["y"], row["x"])),
    }


def cell_id(x: int, y: int) -> str:
    return f"{x},{y}"


EMPTY_DIFF = {
    "schema": None,
    "seed": None,
    "width": None,
    "height": None,
    "cellWidth": None,
    "cellHeight": None,
    "periodicX": None,
    "periodicY": None,
    "tilesRemoved": [],
    "tilesUpserted": [],
    "rulesRemoved": [],
    "rulesUpserted": [],
    "pinnedRemoved": [],
    "pinnedUpserted": [],
    "maskedRemoved": [],
    "maskedUpserted": [],
}


def diff(**lanes: Any) -> dict:
    out = copy.deepcopy(EMPTY_DIFF)
    for key, value in lanes.items():
        assert key in out, key
        out[key] = value
    return out
#endregion


#region 📐 canonical positions
def ordered_index(items: list[dict], key: str, value: str) -> int:
    for index, item in enumerate(items):
        if item[key] > value:
            return index
    return len(items)


def ordered_cell_index(items: list[dict], x: int, y: int) -> int:
    for index, item in enumerate(items):
        if (item["y"], item["x"]) > (y, x):
            return index
    return len(items)
#endregion


#region 🦠 mutations
class Outcome:
    """🎯 What a mutation's diff builder answers: a status, a diagnostic list, and the sparse delta."""

    def __init__(self, status: str, messages: list[dict], delta: dict) -> None:
        self.status = status
        self.messages = messages
        self.delta = delta

    def as_json(self) -> dict:
        out: dict[str, Any] = {"status": self.status}
        if self.messages:
            out["messages"] = self.messages
        return out


def applied(delta: dict, messages: list[dict] | None = None) -> Outcome:
    return Outcome("applied", messages or [], delta)


def fatal(code: str) -> Outcome:
    return Outcome("rejected", [{"level": "fatal", "code": code}], diff())


def rejected(code: str) -> Outcome:
    return Outcome("rejected", [{"level": "error", "code": code}], diff())


def no_op(code: str = "mutation.no-op") -> Outcome:
    return Outcome("no-op", [{"level": "warning", "code": code}], diff())


def _find(items: list[dict], key: str, value: str) -> dict | None:
    return next((item for item in items if item[key] == value), None)


def _find_cell(items: list[dict], x: int, y: int) -> dict | None:
    return next((item for item in items if item["x"] == x and item["y"] == y), None)


def build(base: dict, mutation: dict) -> Outcome:
    """🦠 The diff builder — one branch per `Grid2dMutation` variant, mirroring the Rust leaves."""
    (variant, payload), = mutation.items()
    if variant == "ChangeSeed":
        if base["seed"] == payload["seed"]:
            return no_op()
        return applied(diff(seed=payload["seed"]))
    if variant == "ResizeGrid":
        if payload["width"] == 0 or payload["height"] == 0:
            return fatal("mutation.invariant")
        if base["width"] == payload["width"] and base["height"] == payload["height"]:
            return no_op()
        outside = lambda row: row["x"] >= payload["width"] or row["y"] >= payload["height"]
        pins = [cell_id(row["x"], row["y"]) for row in base["pinned"] if outside(row)]
        masks = [cell_id(row["x"], row["y"]) for row in base["masked"] if outside(row)]
        messages = [{"level": "info", "code": "mutation.cascade"}] if pins or masks else []
        return applied(diff(width=payload["width"], height=payload["height"], pinnedRemoved=pins, maskedRemoved=masks), messages)
    if variant == "ChangeCellSize":
        if payload["cellWidth"] <= 0 or payload["cellHeight"] <= 0:
            return fatal("mutation.invariant")
        if base["cellWidth"] == payload["cellWidth"] and base["cellHeight"] == payload["cellHeight"]:
            return no_op()
        return applied(diff(cellWidth=Float(payload["cellWidth"]), cellHeight=Float(payload["cellHeight"])))
    if variant == "ChangePeriodicity":
        if base["periodicX"] == payload["periodicX"] and base["periodicY"] == payload["periodicY"]:
            return no_op()
        return applied(diff(periodicX=payload["periodicX"], periodicY=payload["periodicY"]))
    if variant == "CreateTile":
        new_tile = payload["tile"]
        if not new_tile["id"]:
            return fatal("mutation.invariant")
        if _find(base["tiles"], "id", new_tile["id"]):
            return fatal("mutation.duplicate-id")
        if not new_tile["weight"] > 0:
            return fatal("mutation.invariant")
        at = ordered_index(base["tiles"], "id", new_tile["id"])
        return applied(diff(tilesUpserted=[[at, new_tile]]))
    if variant == "DeleteTile":
        if not _find(base["tiles"], "id", payload["id"]):
            return rejected("mutation.target-missing")
        rules = [row["id"] for row in base["rules"] if payload["id"] in (row["tileAId"], row["tileBId"])]
        pins = [cell_id(row["x"], row["y"]) for row in base["pinned"] if row["tileId"] == payload["id"]]
        messages = [{"level": "info", "code": "mutation.cascade"}] if rules or pins else []
        return applied(diff(tilesRemoved=[payload["id"]], rulesRemoved=rules, pinnedRemoved=pins), messages)
    if variant == "ChangeTileWeight":
        found = _find(base["tiles"], "id", payload["id"])
        if not found:
            return rejected("mutation.target-missing")
        if not payload["weight"] > 0:
            return fatal("mutation.invariant")
        if found["weight"] == payload["weight"]:
            return no_op()
        index = base["tiles"].index(found)
        updated = copy.deepcopy(found)
        updated["weight"] = Float(payload["weight"])
        return applied(diff(tilesUpserted=[[index, updated]]))
    if variant == "ChangeTileMedia":
        found = _find(base["tiles"], "id", payload["id"])
        if not found:
            return rejected("mutation.target-missing")
        if found["media"] == payload["media"]:
            return no_op()
        index = base["tiles"].index(found)
        updated = copy.deepcopy(found)
        updated["media"] = copy.deepcopy(payload["media"])
        return applied(diff(tilesUpserted=[[index, updated]]))
    if variant == "CreateRule":
        new_rule = payload["rule"]
        if not new_rule["id"]:
            return fatal("mutation.invariant")
        if _find(base["rules"], "id", new_rule["id"]):
            return fatal("mutation.duplicate-id")
        for tile_id in (new_rule["tileAId"], new_rule["tileBId"]):
            if not _find(base["tiles"], "id", tile_id):
                return fatal("mutation.invariant")
        if any(row["tileAId"] == new_rule["tileAId"] and row["tileBId"] == new_rule["tileBId"] and row["direction"] == new_rule["direction"] for row in base["rules"]):
            return fatal("mutation.invariant")
        at = ordered_index(base["rules"], "id", new_rule["id"])
        return applied(diff(rulesUpserted=[[at, new_rule]]))
    if variant == "DeleteRule":
        if not _find(base["rules"], "id", payload["id"]):
            return rejected("mutation.target-missing")
        return applied(diff(rulesRemoved=[payload["id"]]))
    if variant == "PinCell":
        x, y = payload["x"], payload["y"]
        if x >= base["width"] or y >= base["height"]:
            return fatal("mutation.invariant")
        if not _find(base["tiles"], "id", payload["tileId"]):
            return fatal("mutation.invariant")
        if _find_cell(base["masked"], x, y):
            return fatal("mutation.invariant")
        row = pinned(x, y, payload["tileId"])
        existing = _find_cell(base["pinned"], x, y)
        if existing == row:
            return no_op()
        at = base["pinned"].index(existing) if existing else ordered_cell_index(base["pinned"], x, y)
        return applied(diff(pinnedUpserted=[[at, row]]))
    if variant == "UnpinCell":
        x, y = payload["x"], payload["y"]
        if not _find_cell(base["pinned"], x, y):
            return rejected("mutation.target-missing")
        return applied(diff(pinnedRemoved=[cell_id(x, y)]))
    if variant == "MaskCell":
        x, y = payload["x"], payload["y"]
        if x >= base["width"] or y >= base["height"]:
            return fatal("mutation.invariant")
        if _find_cell(base["masked"], x, y):
            return no_op()
        at = ordered_cell_index(base["masked"], x, y)
        pins = [cell_id(x, y)] if _find_cell(base["pinned"], x, y) else []
        messages = [{"level": "info", "code": "mutation.cascade"}] if pins else []
        return applied(diff(maskedUpserted=[[at, cell(x, y)]], pinnedRemoved=pins), messages)
    if variant == "UnmaskCell":
        x, y = payload["x"], payload["y"]
        if not _find_cell(base["masked"], x, y):
            return rejected("mutation.target-missing")
        return applied(diff(maskedRemoved=[cell_id(x, y)]))
    raise KeyError(variant)
#endregion


#region 🩹 apply
def _apply_collection(base: list[dict], removed: list[str], upserted: list[list], key: Callable[[dict], str]) -> list[dict]:
    items = [item for item in base if key(item) not in removed]
    for index, value in upserted:
        existing = next((position for position, item in enumerate(items) if key(item) == key(value)), None)
        if existing is None:
            items.insert(index, copy.deepcopy(value))
        else:
            items[existing] = copy.deepcopy(value)
    return items


def apply(base: dict, delta: dict) -> dict:
    out = copy.deepcopy(base)
    for scalar in ("schema", "seed", "width", "height", "cellWidth", "cellHeight", "periodicX", "periodicY"):
        if delta[scalar] is not None:
            out[scalar] = delta[scalar]
    out["tiles"] = _apply_collection(out["tiles"], delta["tilesRemoved"], delta["tilesUpserted"], lambda row: row["id"])
    out["rules"] = _apply_collection(out["rules"], delta["rulesRemoved"], delta["rulesUpserted"], lambda row: row["id"])
    out["pinned"] = _apply_collection(out["pinned"], delta["pinnedRemoved"], delta["pinnedUpserted"], lambda row: cell_id(row["x"], row["y"]))
    out["masked"] = _apply_collection(out["masked"], delta["maskedRemoved"], delta["maskedUpserted"], lambda row: cell_id(row["x"], row["y"]))
    return out


def inverse(base: dict, mutation: dict) -> list[dict]:
    """↩️ The inverse mutation list, mirroring the Rust `↩️inverse` leaves."""
    (variant, payload), = mutation.items()
    if variant == "ChangeSeed":
        return [{"ChangeSeed": {"seed": base["seed"]}}]
    if variant == "ResizeGrid":
        if base["width"] == payload["width"] and base["height"] == payload["height"]:
            return []
        outside = lambda row: row["x"] >= payload["width"] or row["y"] >= payload["height"]
        steps: list[dict] = [{"ResizeGrid": {"width": base["width"], "height": base["height"]}}]
        steps += [{"MaskCell": {"x": row["x"], "y": row["y"]}} for row in base["masked"] if outside(row)]
        steps += [{"PinCell": {"x": row["x"], "y": row["y"], "tileId": row["tileId"]}} for row in base["pinned"] if outside(row)]
        return steps
    if variant == "ChangeCellSize":
        return [{"ChangeCellSize": {"cellWidth": base["cellWidth"], "cellHeight": base["cellHeight"]}}]
    if variant == "ChangePeriodicity":
        return [{"ChangePeriodicity": {"periodicX": base["periodicX"], "periodicY": base["periodicY"]}}]
    if variant == "CreateTile":
        if _find(base["tiles"], "id", payload["tile"]["id"]):
            return []
        return [{"DeleteTile": {"id": payload["tile"]["id"]}}]
    if variant == "DeleteTile":
        found = _find(base["tiles"], "id", payload["id"])
        if not found:
            return []
        steps: list[dict] = [{"CreateTile": {"tile": copy.deepcopy(found)}}]
        steps += [{"CreateRule": {"rule": copy.deepcopy(row)}} for row in base["rules"] if payload["id"] in (row["tileAId"], row["tileBId"])]
        steps += [{"PinCell": {"x": row["x"], "y": row["y"], "tileId": row["tileId"]}} for row in base["pinned"] if row["tileId"] == payload["id"]]
        return steps
    if variant == "ChangeTileWeight":
        found = _find(base["tiles"], "id", payload["id"])
        return [{"ChangeTileWeight": {"id": payload["id"], "weight": found["weight"]}}] if found else []
    if variant == "ChangeTileMedia":
        found = _find(base["tiles"], "id", payload["id"])
        return [{"ChangeTileMedia": {"id": payload["id"], "media": copy.deepcopy(found["media"])}}] if found else []
    if variant == "CreateRule":
        if _find(base["rules"], "id", payload["rule"]["id"]):
            return []
        return [{"DeleteRule": {"id": payload["rule"]["id"]}}]
    if variant == "DeleteRule":
        found = _find(base["rules"], "id", payload["id"])
        return [{"CreateRule": {"rule": copy.deepcopy(found)}}] if found else []
    if variant == "PinCell":
        existing = _find_cell(base["pinned"], payload["x"], payload["y"])
        if existing and existing["tileId"] == payload["tileId"]:
            return []
        if existing:
            return [{"PinCell": {"x": existing["x"], "y": existing["y"], "tileId": existing["tileId"]}}]
        return [{"UnpinCell": {"x": payload["x"], "y": payload["y"]}}]
    if variant == "UnpinCell":
        existing = _find_cell(base["pinned"], payload["x"], payload["y"])
        return [{"PinCell": {"x": existing["x"], "y": existing["y"], "tileId": existing["tileId"]}}] if existing else []
    if variant == "MaskCell":
        if _find_cell(base["masked"], payload["x"], payload["y"]):
            return []
        steps: list[dict] = [{"UnmaskCell": {"x": payload["x"], "y": payload["y"]}}]
        existing = _find_cell(base["pinned"], payload["x"], payload["y"])
        if existing:
            steps.append({"PinCell": {"x": existing["x"], "y": existing["y"], "tileId": existing["tileId"]}})
        return steps
    if variant == "UnmaskCell":
        if not _find_cell(base["masked"], payload["x"], payload["y"]):
            return []
        return [{"MaskCell": {"x": payload["x"], "y": payload["y"]}}]
    raise KeyError(variant)
#endregion


#region 🧭️ adapter
LEAVES = ("before", "mutation", "diff", "outcome", "after")


def variant_of(kind: str) -> str:
    """🐫️ `change-cell-size` → `ChangeCellSize`, the externally tagged payload's single key."""
    return "".join(word.capitalize() for word in kind.split("-"))


def committed(ctx: Any) -> dict:
    """🧫️ The committed quintet of this scenario's row, read through the plan's declared fixtures."""
    spec = ctx.doc_json()
    if spec["kind"] != ctx.row():
        raise AssertionError(f"scenario {ctx.scenario['id']}: the doc string names {spec['kind']!r}")
    return {leaf: json.loads(ctx.fixture_bytes(spec[leaf]).decode("utf-8")) for leaf in LEAVES}


def adapter() -> Any:
    """🧭️ The platform entry point. The reference answers in the ORACLE role only, by Scenario Outline base id —
    registering it as a subject too would make it its own subject and manufacture a green self-comparison. Every law
    `replay()` checks is asserted in role, per row, before the parity phase compares the document it answers."""
    from semio_repo_test import Adapter, Outcome as HostOutcome

    def answer(document: dict) -> Any:
        return HostOutcome(document, raw=json.dumps(document, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))

    def mutate_oracle(ctx: Any) -> Any:
        kind, leaves = ctx.row(), committed(ctx)
        if list(leaves["mutation"]) != [variant_of(kind)]:
            raise AssertionError(f"mutate-{kind}: the committed payload declares {list(leaves['mutation'])!r}")
        rebuilt = build(leaves["before"], leaves["mutation"])
        if json.loads(dumps(rebuilt.delta)) != leaves["diff"]:
            raise AssertionError(f"mutate-{kind}: the reference diff differs from the committed 🔺️diff")
        if json.loads(dumps(rebuilt.as_json())) != leaves["outcome"]:
            raise AssertionError(f"mutate-{kind}: the reference outcome differs from the committed 🎯️outcome")
        after = apply(leaves["before"], leaves["diff"])
        if after != leaves["after"]:
            raise AssertionError(f"mutate-{kind}: the diff does not carry before to the committed after-snapshot")
        if after == leaves["before"] and rebuilt.status != "no-op":
            raise AssertionError(f"mutate-{kind}: the vector does not move the document and its outcome declares no no-op")
        return answer(after)

    def inverse_oracle(ctx: Any) -> Any:
        kind, leaves = ctx.row(), committed(ctx)
        restored = apply(leaves["before"], leaves["diff"])
        for step in inverse(leaves["before"], leaves["mutation"]):
            restored = apply(restored, build(restored, step).delta)
        if restored != leaves["before"]:
            raise AssertionError(f"inverse-{kind}: the reference's own inverse did not restore the before-snapshot")
        return answer(restored)

    return Adapter("python").oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle)
#endregion


#region 🔁️ replay
FIXTURES = pathlib.Path(__file__).resolve().parents[2] / "🧫️fixtures/🧬️mutations"

VECTORS = [
    ("change-seed", "🎲️change-seed", "🎲️reseeds-the-solve-from-7-to-99"),
    ("resize-grid", "📐️resize-grid", "📐️shrinks-the-board-and-drops-the-outside-cells"),
    ("change-cell-size", "📏️change-cell-size", "📏️widens-every-cell"),
    ("change-periodicity", "🔁️change-periodicity", "🔁️wraps-the-x-axis"),
    ("create-tile", "🌱️create-tile", "🌱️inserts-the-corner-tile-in-sorted-order"),
    ("delete-tile", "🗑️delete-tile", "🗑️removes-the-straight-tile-and-cascades-its-rule-and-pin"),
    ("change-tile-weight", "⚖️change-tile-weight", "⚖️biases-the-solve-towards-empty"),
    ("change-tile-media", "🎨️change-tile-media", "🎨️redraws-the-empty-tile-as-a-bitmap"),
    ("create-rule", "🚦️create-rule", "🚦️lets-two-straights-stack-vertically"),
    ("delete-rule", "❌delete-rule", "❌️forbids-the-straight-pair-again"),
    ("pin-cell", "📌️pin-cell", "📌️fixes-the-right-cell-to-the-straight-tile"),
    ("unpin-cell", "📍️unpin-cell", "📍️releases-the-pinned-straight-cell"),
    ("mask-cell", "🕳️mask-cell", "🕳️cuts-the-pinned-corner-out-of-the-problem"),
    ("unmask-cell", "🔳️unmask-cell", "🔳️puts-the-hole-back-into-the-problem"),

]


def load(root: pathlib.Path, *parts: str) -> object:
    return json.loads((root.joinpath(*parts)).read_text(encoding="utf-8"))


def replay() -> int:
    failures = 0
    for kind, directory, case in VECTORS:
        root = FIXTURES / directory / case
        before = load(root, "📸️snapshot/⬅️before/🔣️.json")
        after = load(root, "📸️snapshot/➡️after/🔣️.json")
        delta = load(root, "🔺️diff/🔣️.json")
        outcome = load(root, "🎯️outcome/🔣️.json")
        mutation = load(root, "🦠️mutation/🔣️.json")
        produced = apply(before, delta)
        if produced != after:
            print(f"FAIL {kind}/{case}: committed diff does not carry before to after")
            failures += 1
        rebuilt = build(before, mutation)
        if json.loads(dumps(rebuilt.delta)) != delta:
            print(f"FAIL {kind}/{case}: reference diff differs from the committed diff")
            failures += 1
        if json.loads(dumps(rebuilt.as_json())) != outcome:
            print(f"FAIL {kind}/{case}: reference outcome differs from the committed outcome")
            failures += 1
        restored = produced
        for step in inverse(before, mutation):
            restored = apply(restored, build(restored, step).delta)
        if restored != before:
            print(f"FAIL {kind}/{case}: inverse did not restore the before-snapshot")
            failures += 1
    print(f"{len(VECTORS)} vectors replayed, {failures} failure(s)")
    return failures
#endregion


if __name__ == "__main__":
    raise SystemExit(1 if replay() else 0)
