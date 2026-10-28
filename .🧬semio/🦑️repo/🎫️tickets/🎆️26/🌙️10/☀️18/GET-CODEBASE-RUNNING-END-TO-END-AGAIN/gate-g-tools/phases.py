"""⏱️ Phase split of official product logs: `phases.py [--json] <log>...` streams each `gate-g-tools/run/*.log` and prints where the wall time went.

The clock is the `[repository:command] running elapsedMs=N` heartbeat (10 s grain) plus the product.py wall from the sibling json; phases: door (before the command clock), facts (graph fact verification), closures (input-closure verification), stale (stale listing → refresh start), infer (project inference), publish (inferred → published), plan (assembly + Bazel planning), build (Bazel invocation → end)."""
import json, os, re, sys

BEAT = re.compile(r"^\[repository:command\] running elapsedMs=(\d+)")
MARKS = [
    ("facts", re.compile(r"Verifying graph facts: 0 of")),
    ("closures", re.compile(r"Verifying input closures: 0 of (\d+)")),
    ("stale", re.compile(r"^graph: \d+ stale facts|^graph: refreshing (\d+) projects")),
    ("infer", re.compile(r"^graph: asking the host|^graph: inferring projects|Inferring projects: 0 of")),
    ("publish", re.compile(r"projects inferred by \d+ worker|^graph: generation \d+ published")),
    ("plan", re.compile(r"Assembling the graph: 0 of|^graph: \d+ projects (planned|served)|^\[bazel\] session slot")),
    ("build", re.compile(r"^INFO: Invocation ID|^\[bazel\] plan reused")),
]
FACTS = {
    "refreshing": re.compile(r"^graph: refreshing (\d+) projects"),
    "stale": re.compile(r"^graph: (\d+) stale facts \((.*?)\) leave"),
    "published": re.compile(r"^graph: generation (\d+) published, (\d+) projects refreshed in (\d+) s"),
    "derived": re.compile(r"^graph: (\d+) derived results asked again after \d+ stale facts: (\d+) unchanged, (\d+) changed"),
    "closures": re.compile(r"Verifying input closures: 0 of (\d+)"),
    "workers": re.compile(r"^graph: worker processes were not used because (.*)$"),
    "inferred": re.compile(r"^graph: (\d+) of (\d+) projects inferred by (\d+) worker processes in (\d+) s"),
    "memo": re.compile(r"^graph: .*(served from the memo|memo cannot serve|refused).*$"),
    "plan": re.compile(r"^\[bazel\] (plan reused|planning again[^,]*|this plan cannot be reused later: [^:]*)"),
}


def split(path):
    clock, seen, facts = 0.0, {}, {}
    with open(path, encoding="utf8", errors="replace") as stream:
        for line in stream:
            beat = BEAT.match(line)
            if beat:
                clock = int(beat.group(1)) / 1000
                continue
            for name, pattern in MARKS:
                if name not in seen and pattern.search(line):
                    seen[name] = clock
            for name, pattern in FACTS.items():
                if name not in facts:
                    match = pattern.search(line)
                    if match:
                        facts[name] = list(match.groups())
    sidecar = os.path.splitext(path)[0] + ".json"
    wall = json.load(open(sidecar, encoding="utf8")).get("wall_s") if os.path.exists(sidecar) else None
    order = [name for name, _ in MARKS if name in seen]
    marks = [(name, seen[name]) for name in order]
    end = wall if wall is not None else clock
    spans, start = {}, marks[0][1] if marks else end
    spans["door+boot"] = round(start, 1)
    for index, (name, at) in enumerate(marks):
        until = marks[index + 1][1] if index + 1 < len(marks) else end
        spans[name] = round(max(0.0, until - at), 1)
    return {"log": os.path.basename(path), "wall_s": wall, "spans": spans, "facts": facts}


def main(arguments):
    as_json = "--json" in arguments
    for path in [a for a in arguments if a != "--json"]:
        row = split(path)
        if as_json:
            print(json.dumps(row, ensure_ascii=False))
            continue
        spans = " ".join(f"{k}={v}" for k, v in row["spans"].items())
        notes = "; ".join(f"{k}={'/'.join(v)}" for k, v in row["facts"].items() if k in ("refreshing", "closures", "published", "derived", "inferred", "plan"))
        print(f"{row['log']:<18} wall={row['wall_s']} {spans} | {notes}")


if __name__ == "__main__":
    main(sys.argv[1:])
