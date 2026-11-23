"""🧾️ Condenses a verbose vitest log of the three-views guard: per failing asset the number of findings per check code and scale, and the first findings in full."""
import collections
import re
import sys

FINDING = re.compile(r"^\s*[-+]?\s*\[([a-z-]+)\] 1:(\d+) (\S+) (.*)$")


def condense(path: str, shown: int) -> None:
    """📋️ Print one block per asset of the log at `path` with at most `shown` full findings."""
    counts: dict[str, collections.Counter] = collections.defaultdict(collections.Counter)
    first: dict[str, list[str]] = collections.defaultdict(list)
    for line in open(path, encoding="utf-8"):
        match = FINDING.match(line)
        if match is None:
            continue
        code, scale, asset, rest = match.groups()
        counts[asset][f"{code}@{scale}"] += 1
        if len(first[asset]) < shown:
            first[asset].append(f"[{code}] 1:{scale} {rest.strip()}")
    for asset in sorted(counts):
        print(f"## {asset}: {sum(counts[asset].values())} findings: " + ", ".join(f"{key} {value}" for key, value in sorted(counts[asset].items())))
        for finding in first[asset]:
            print(f"  {finding}")


if __name__ == "__main__":
    condense(sys.argv[1], int(sys.argv[2]) if len(sys.argv) > 2 else 2)
