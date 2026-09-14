#!/usr/bin/env python3
"""🔧️ Registers the `-twin` capabilities of the TypeScript-twin test cases on their d3 oracles.

The twin cases measure `@semio-tech/print-viz-kernel` against the same oracle as the LaTeX case they
mirror, so every oracle needs the mirrored capability string before the platform will plan the case.
Idempotent; preserves key order and the decimal notation of the tolerance numbers.
"""
import json
import os
import sys
from collections import OrderedDict

TWIN = {
    "d3-scale": ["viz-scale-color-twin", "viz-scale-continuous-twin", "viz-scale-discrete-twin", "viz-scale-temporal-twin"],
    "d3-interpolate": ["viz-scale-color-twin"],
    "d3-color": ["viz-scale-color-twin"],
    "d3-time": ["viz-scale-temporal-twin"],
    "d3-format": ["viz-format-number-twin"],
    "d3-time-format": ["viz-format-time-twin"],
    "d3-array": ["viz-transform-bin-twin", "viz-transform-statistics-twin"],
    "d3-regression": ["viz-transform-statistics-twin"],
    "d3-shape": ["viz-transform-stack-twin", "viz-shape-arc-twin", "viz-shape-curves-twin", "viz-shape-link-twin", "viz-shape-symbol-twin"],
    "d3-chord": ["viz-shape-link-twin", "network-chord-layout-twin"],
    "d3-hierarchy": ["viz-hierarchy-aggregates-twin", "viz-hierarchy-pack-twin", "viz-hierarchy-partition-twin", "viz-hierarchy-sunburst-twin", "viz-hierarchy-tree-twin", "viz-hierarchy-cluster-twin", "viz-hierarchy-treemap-twin"],
    "d3-force": ["network-force-layout-twin"],
    "d3-sankey": ["flow-sankey-layout-twin"],
    "d3-geo": ["geo-projection-twin", "viz-geo-path-twin"],
    "d3-contour": ["viz-layout-contour-twin"],
    "d3-delaunay": ["viz-spatial-delaunay-twin", "viz-spatial-hull-twin"],
    "d3-hexbin": ["viz-spatial-hexbin-twin"],
}
DECISIONS = {"kernel-density": ["viz-transform-kde-twin"]}


def main() -> int:
    os.chdir(sys.argv[1] if len(sys.argv) > 1 else "C:/git/semio/🧰️framework/🛍️products/📓️print/🔮️oracle")
    with open("🔣️.json", encoding="utf8") as handle:
        text = handle.read()
    registry = json.loads(text, object_pairs_hook=OrderedDict)
    added = 0
    for entry in registry["oracles"]:
        for capability in TWIN.get(entry["id"], []):
            if capability not in entry["capabilities"]:
                entry["capabilities"].append(capability)
                added += 1
    for entry in registry["noOracleDecisions"]:
        for capability in DECISIONS.get(entry["id"], []):
            if capability not in entry["capabilities"]:
                entry["capabilities"].append(capability)
                added += 1
    with open("🔣️.json", "w", encoding="utf8", newline="\n") as handle:
        json.dump(registry, handle, ensure_ascii=False, indent=2)
        handle.write("\n")
    print(f"twin capabilities added: {added}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
