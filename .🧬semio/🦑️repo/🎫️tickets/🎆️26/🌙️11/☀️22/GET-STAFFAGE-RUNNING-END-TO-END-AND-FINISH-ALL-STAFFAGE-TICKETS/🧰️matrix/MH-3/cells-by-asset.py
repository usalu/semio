"""🗂️ Summary of a matrix output folder per asset: `python3 cells-by-asset.py <dir>` prints for every asset of `matrix.json` the cells by status and, per failing check, the cells by scale entry and view with the count, the worst delta and the first message."""
import collections, json, sys
cells = json.load(open(sys.argv[1] + "/matrix.json"))["cells"]
for asset in sorted({cell["asset"] for cell in cells}):
    own = [cell for cell in cells if cell["asset"] == asset]
    status = collections.Counter(cell["status"] for cell in own)
    print(f"{asset}: {len(own)} cells: " + ", ".join(f"{count} {name}" for name, count in sorted(status.items())))
    groups = collections.defaultdict(lambda: collections.defaultdict(list))
    for cell in own:
        for check, failure in (cell.get("failures") or {}).items():
            groups[check][(cell["scale"], cell["view"])].append(failure)
    for check, by in sorted(groups.items()):
        print(f"   {check} {sum(len(v) for v in by.values())} cells")
        for (scale, view), items in sorted(by.items()):
            failure = items[0]
            worst = failure.get("worst") or {}
            first = (failure.get("first") or [{}])[0]
            print(f"      {scale:>14} {view:<9} ×{len(items)} modes n={failure.get('count')} worst {worst.get('deltaMm', 0):.0f} mm @ {worst.get('subject', first.get('subject', ''))} :: {str(first.get('message', ''))[:150]}")
