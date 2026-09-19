"""🗺️ One-shot: bring the gismap `demo` example asset onto the live `MapFeature { id, data }` shape.

The asset's `positions=`/`routes=` lines are hex-encoded JSON arrays of *flat* feature objects
(`{id, lon, lat, …}`), which is the shape `MapFeature` carried before its payload moved behind the
opaque `data` escape hatch. `MapFeature` is `#[value(deny_unknown_fields)]`, so every flat key now
faults the parse (`0.unknown field `lon``) and `setActiveExample` refuses at boot.

The transform is exactly `gis_map_document_from_descriptor_json`
(`…/🧬️schema/🦀️.rs:250`): key by the entry's own `id`, keep the whole original object as the payload
— which also keeps `gis_map_descriptor_json` emitting the renderer's original flat array byte for byte.
"""

import binascii
import io
import json
import sys

ASSET = "/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/🏅️standards/🔖️1/🪆️subsets/✳️any/🖼️assets/🎬️demo/🗣️.dsl.semio"
COLLECTIONS = ("positions", "routes", "regions")


def migrate(entries):
    out = []
    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            raise SystemExit(f"entry {index} is not an object")
        if set(entry) == {"id", "data"}:
            out.append(entry)
            continue
        if "id" not in entry:
            raise SystemExit(f"entry {index} has no id")
        out.append({"id": entry["id"], "data": entry})
    return out


def main():
    lines = io.open(ASSET, encoding="utf-8").read().split("\n")
    changed = []
    for index, line in enumerate(lines):
        key, sep, raw = line.partition("=")
        if not sep or key not in COLLECTIONS:
            continue
        entries = json.loads(binascii.unhexlify(raw).decode("utf-8"))
        migrated = migrate(entries)
        if migrated == entries:
            continue
        payload = json.dumps(migrated, separators=(",", ":"), ensure_ascii=False)
        lines[index] = f"{key}={binascii.hexlify(payload.encode('utf-8')).decode('ascii')}"
        changed.append(f"{key}: {len(entries)} entries")
    if not changed:
        print("already migrated")
        return
    io.open(ASSET, "w", encoding="utf-8").write("\n".join(lines))
    print("migrated " + "; ".join(changed))


if __name__ == "__main__":
    sys.exit(main())
