# Temporary generator: flat snapshot -> metabolism.new block-hashtag kit bundle JSON (run from repo root helpers).

from __future__ import annotations

import copy
import json
import uuid
from pathlib import Path

try:
    import blake3
except ImportError as e:
    raise SystemExit("pip install blake3") from e

HASH = "\u2026"
SCHEMA = "\U0001f38626\U0001f31906\u2b06\ufe0f1"


def digest_kit_blob_wire(wire: str) -> str:
    return blake3.blake3(wire.encode("utf-8")).hexdigest()


def transform_obj(d: dict) -> dict:
    out: dict = {}
    for k, v in d.items():
        out[k] = transform_value(v, k)
    if isinstance(out.get("id"), str) and "hash" not in out:
        out["hash"] = HASH
    return out


def transform_value(val, key: str | None):
    if isinstance(val, dict):
        return transform_obj(val)
    if isinstance(val, list):
        inner = [transform_obj(x) if isinstance(x, dict) else x for x in val]
        if key == "items":
            return inner
        if len(inner) == 0:
            return {"hash": HASH, "items": inner}
        if isinstance(inner[0], dict):
            return {"hash": HASH, "items": inner}
        return inner
    return val


def hoist_blobs_from_kit(kit: dict, seen_digest: set[str], blob_items: list) -> None:
    fv = kit.get("files")
    if fv is None:
        return
    if isinstance(fv, dict):
        arr = fv.get("items") or []
    elif isinstance(fv, list):
        arr = fv
    else:
        return
    for f in arr:
        if not isinstance(f, dict):
            continue
        wire = f.pop("blob", None)
        if wire is None or not isinstance(wire, str):
            continue
        d = digest_kit_blob_wire(wire)
        f["blobHash"] = d
        if d in seen_digest:
            continue
        seen_digest.add(d)
        blob_items.append({"hash": d, "blob": wire})


def hl(items=None):
    return {"hash": HASH, "items": [] if items is None else items}


def main() -> None:
    repo = Path(__file__).resolve().parents[6]
    snap_path = repo / "compose" / "assets" / "compose" / "metabolism.kit.snapshot.compose.json"
    out_path = repo / "compose" / "assets" / "compose" / "metabolism.kit.compose.json"

    full = json.loads(snap_path.read_text(encoding="utf-8"))
    shaped = transform_obj(copy.deepcopy(full))

    stripped = copy.deepcopy(shaped)
    blob_items: list = []
    seen_digest: set[str] = set()
    hoist_blobs_from_kit(stripped, seen_digest, blob_items)
    frozen = copy.deepcopy(stripped)

    kit_id = stripped["id"]
    ckpt_id = str(uuid.uuid4())
    draft_id = str(uuid.uuid4())
    ts = stripped.get("updatedAt", "2025-11-23T01:10:21.03Z")

    checkpoint = {
        "id": ckpt_id,
        "hash": HASH,
        "timestamp": ts,
        "message": "initial import from metabolism.kit.snapshot.compose.json",
        "authors": hl(),
        "changes": hl(),
        "frozenRoot": frozen,
    }

    draft = {
        "id": draft_id,
        "hash": HASH,
        "checkpoint": {"id": ckpt_id, "hash": HASH},
        "transactions": hl(),
    }

    wip = {
        "id": kit_id,
        "hash": HASH,
        "authors": hl(),
        "root": stripped,
        "checkpoints": hl([checkpoint]),
        "alternatives": hl(),
        "drafts": hl([draft]),
    }

    empty_root = {
        "hash": HASH,
        "name": "",
        "types": {"hash": HASH, "items": []},
        "designs": {"hash": HASH, "items": []},
    }

    auth_stage = {
        "id": kit_id,
        "hash": HASH,
        "authors": hl(),
        "root": empty_root,
        "checkpoints": hl(),
        "alternatives": hl(),
        "drafts": hl(),
    }

    bundle = {
        "schema": SCHEMA,
        "wip": wip,
        "authoritative": auth_stage,
        "stage": auth_stage,
        "conflicts": hl(),
        "blobs": hl(blob_items),
    }

    if out_path.exists():
        out_path.unlink()

    out_path.write_text(json.dumps(bundle, ensure_ascii=False, separators=(",", ":")) + "\n", encoding="utf-8")
    print("ok", "blob_rows", len(blob_items), "bytes", out_path.stat().st_size)


if __name__ == "__main__":
    main()
