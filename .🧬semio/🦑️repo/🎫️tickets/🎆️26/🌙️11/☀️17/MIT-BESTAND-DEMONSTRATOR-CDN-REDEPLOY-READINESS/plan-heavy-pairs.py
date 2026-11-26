"""🧮️ Compare retained release Cargo closures, RSS and completed space observations."""
from collections import Counter, defaultdict
from functools import cache
from hashlib import sha256
from itertools import combinations
import json
from pathlib import Path
from statistics import quantiles
import sys


def owner(unit):
    return unit["pkg_id"].rsplit("#", 1)[-1].split("@", 1)[0]


def closure(graph, root):
    units = graph["units"]

    @cache
    def identity(index):
        row = units[index]
        edges = [{**edge, "index": identity(edge["index"])} for edge in row["dependencies"]]
        value = {**row, "dependencies": sorted(edges, key=lambda edge: json.dumps(edge, sort_keys=True))}
        return sha256(json.dumps(value, sort_keys=True).encode()).hexdigest()

    pending, seen = [root], set()
    while pending:
        index = pending.pop()
        if index not in seen:
            seen.add(index)
            pending.extend(edge["index"] for edge in units[index]["dependencies"])
    return Counter(identity(index) for index in seen)


def indices(graph, root):
    units = graph["units"]
    pending, seen = [root], set()
    while pending:
        index = pending.pop()
        if index not in seen:
            seen.add(index)
            pending.extend(edge["index"] for edge in units[index]["dependencies"])
    return seen


def footprint_kind(crate, kinds):
    if "cdylib" in kinds:
        return "component"
    if "proc-macro" in kinds:
        return "procMacro"
    return "buildScript" if crate.startswith("build_script_") else "library"


def percentile(values, fraction):
    ordered = sorted(values)
    if not ordered:
        return None
    rank = (len(ordered) - 1) * fraction
    lo = int(rank)
    hi = min(lo + 1, len(ordered) - 1)
    return ordered[lo] + (ordered[hi] - ordered[lo]) * (rank - lo)


def paired_footprints(path, selected, names, history):
    driver = Path("🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build")
    admission = json.loads((driver / "🎟️admission/🧬️schema/🔣️.json").read_bytes())["definitions"]["Policy"]["const"]
    budget = json.loads((driver / "🧩️component-pass/🧬️schema/🔣️.json").read_bytes())["definitions"]["Policy"]["const"]
    quantile = admission["quantile"]
    graph = json.loads(path.read_bytes())
    units = graph["units"]
    roots = {names[owner(units[root])]: indices(graph, root) for root in graph["roots"] if owner(units[root]) in names}
    measured, pools = defaultdict(list), defaultdict(list)
    for row in history:
        key = row["key"]
        package, _, identity = key.partition("@")
        crate, target, profile, kinds = identity.split("|")
        size = row["outputBytes"]
        measured[key].append(size)
        pools[footprint_kind(crate, kinds.split(","))].append(size)

    def weight(index):
        row = units[index]
        target = row["target"]
        crate = target["name"].replace("-", "_")
        kinds = sorted(target["crate_types"])
        key = f"{owner(row)}@{crate}|{row['platform'] or 'host'}|release|{','.join(kinds)}"
        values = measured[key] if row["mode"] != "run-custom-build" else []
        if values:
            return percentile(values, quantile), True
        kind = "buildScript" if row["mode"] == "run-custom-build" or "custom-build" in target["kind"] else footprint_kind(crate, kinds)
        return percentile(pools[kind], quantile) or budget["unitFootprintBoundsBytes"][kind], False

    def capacity(closure):
        charges = [weight(index) for index in closure]
        return sum(size for size, _ in charges), sum(observed for _, observed in charges)

    reserve = budget["storageReserveBytes"]
    for plugin in selected:
        if plugin in roots:
            bytes_, measured_count = capacity(roots[plugin])
            print(f"{plugin:14s} singleton {len(roots[plugin]):4} units; logical output {readable(bytes_)} ({measured_count} measured units); with reserve {readable(bytes_+reserve)}")
    for left, right in combinations(selected, 2):
        if left not in roots or right not in roots:
            continue
        a, b = roots[left], roots[right]
        joint = a | b
        bytes_, measured_count = capacity(joint)
        print(f"{left:14s} {right:14s} {len(a & b):4}/{len(joint):<4} shared; logical output {readable(bytes_)} ({measured_count} measured units); with reserve {readable(bytes_+reserve)}")


def q90(values):
    if len(values) < 2:
        return values[-1] if values else None
    return quantiles(values[-8:], n=10, method="inclusive")[8]


def readable(amount):
    return "unobserved" if amount is None else f"{amount / 2**30:.2f} GiB"


def rows(path):
    for line in path.read_text().splitlines():
        try:
            yield json.loads(line)
        except ValueError:
            continue


def main():
    arguments = sys.argv[1:]
    joint = Path(arguments.pop(0).split("=", 1)[1]) if arguments and arguments[0].startswith("--joint=") else None
    selected = tuple(arguments)
    native = Path(".cache/b/r/m/components")
    names = {f"semio-hub-{plugin}-component": plugin for plugin in selected}
    captures = {}
    for path in sorted(native.glob("runs/*/l/unit-graph-release-*.json"), key=lambda file: file.stat().st_mtime, reverse=True):
        if path.stem.endswith("-plan"):
            continue
        graph = json.loads(path.read_bytes())
        for index in graph["roots"]:
            plugin = names.get(owner(graph["units"][index]))
            if plugin and plugin not in captures:
                captures[plugin] = (path.parts[-3], closure(graph, index))

    growth, rss = defaultdict(list), defaultdict(list)
    for row in rows(native / "storage-growth.jsonl"):
        if row["key"].startswith("release|wasm32-wasip2|"):
            growth[row["key"].split("|")[2]].append(row["bytes"])
    history = list(rows(native / "build-telemetry/units.jsonl"))
    for row in history:
        package, separator, identity = row.get("key", "").partition("@")
        if separator and "|wasm32-wasip2|release|" in identity:
            rss[package].append(row["peakRssBytes"])

    for plugin in selected:
        package = f"semio-hub-{plugin}"
        raw = captures.get(plugin)
        capture = "unobserved" if raw is None else f"{raw[0]}, {sum(raw[1].values())} closure units"
        print(f"{plugin:14s} capture {capture}; growth {readable(q90(growth[package + '-component']))}; lib RSS {readable(q90(rss[package]))}; component RSS {readable(q90(rss[package + '-component']))}")
    for left, right in combinations(selected, 2):
        if left not in captures or right not in captures:
            continue
        a, b = captures[left][1], captures[right][1]
        shared, union = (sum(x.values()) for x in (a & b, a | b))
        print(f"{left:14s} {right:14s} {shared:4}/{union:<4} {shared/union:6.1%} signature overlap")

    if joint:
        print("Joint captured unit estimates before materialized-lane credit or later growth learning:")
        paired_footprints(joint, selected, names, history)


if __name__ == "__main__":
    main()
