"""One-off: wrap piece-level plane+center into pose { plane, center } in compose JSON assets."""
from __future__ import annotations

import json
import sys
from pathlib import Path


def is_piece_like(obj: dict) -> bool:
    if "id" not in obj or not isinstance(obj.get("id"), str):
        return False
    if "plane" not in obj and "center" not in obj:
        return False
    # Exclude obvious non-pieces (connections have sides; planes alone on connectors differ)
    if "connected" in obj and "connecting" in obj:
        return False
    if "gap" in obj and "shift" in obj and "rise" in obj and "rotation" in obj:
        return False
    return "type" in obj or "name" in obj or "isHidden" in obj or "props" in obj or "mirrorPlane" in obj


def migrate_obj(obj):
    if isinstance(obj, dict):
        if is_piece_like(obj) and ("plane" in obj or "center" in obj):
            plane = obj.pop("plane", None)
            center = obj.pop("center", None)
            if plane is not None or center is not None:
                obj["pose"] = {"plane": plane, "center": center}
        for k, v in list(obj.items()):
            migrate_obj(v)
    elif isinstance(obj, list):
        for item in obj:
            migrate_obj(item)


def main() -> int:
    here = Path(__file__).resolve()
    root: Path | None = None
    for anc in here.parents:
        cand = anc / "compose" / "assets" / "compose"
        if cand.is_dir():
            root = cand
            break
    if root is None:
        print("could not find compose/assets/compose from", here, file=sys.stderr)
        return 1
    for path in sorted(root.rglob("*.json")):
        text = path.read_text(encoding="utf-8")
        try:
            data = json.loads(text)
        except json.JSONDecodeError as e:
            print("skip invalid json", path, e, file=sys.stderr)
            continue
        migrate_obj(data)
        path.write_text(json.dumps(data, indent=2, ensure_ascii=False) + "\n", encoding="utf-8")
        print("ok", path.relative_to(root))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
