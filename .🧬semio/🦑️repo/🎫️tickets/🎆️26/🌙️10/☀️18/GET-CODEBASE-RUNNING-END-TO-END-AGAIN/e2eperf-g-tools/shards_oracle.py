#!/usr/bin/env python3
"""Independent Python oracle of the shard planning of the course end-to-end gate (ticket tool, stdlib only).

Usage: shards_oracle.py <table.json> <fixtures.json> [--write]
Recomputes every derived section of the fixtures (arguments, matches, stacks, schedules, allocations, budgets, waves) from their inputs with its own implementation and
compares (or rewrites with --write) the file the TypeScript test reads.
"""
import fnmatch, json, re, sys

def matches(pattern, project):
    return re.fullmatch(".*".join(re.escape(part) for part in pattern.split("*")), project, re.IGNORECASE) is not None

def split_arguments(args):
    projects, rest, workers, i = [], [], None, 0
    while i < len(args):
        a = args[i]
        if a in ("--project", "-p"):
            while i + 1 < len(args) and not args[i + 1].startswith("-"):
                i += 1
                projects.append(args[i])
        elif a.startswith("--project="):
            projects.append(a[len("--project="):])
        elif a in ("--workers", "-j"):
            i += 1
            workers = args[i] if i < len(args) else None
        elif a.startswith("--workers="):
            workers = a[len("--workers="):]
        else:
            rest.append(a)
        i += 1
    return projects, workers, rest

def plan(table, topology, requested, split):
    layout = table["stacks"][topology]
    shards = {s["id"]: s for s in table["shards"]}
    known = [p for s in table["shards"] for p in s["projects"]]
    unknown = [r for r in requested if not any(matches(r, k) for k in known) and not matches(r, table["warmup"])]
    stacks = []
    for ids in layout:
        waves = []
        for sid in ids:
            keep = [p for p in shards[sid]["projects"] if not requested or any(matches(r, p) for r in requested)]
            if keep:
                waves.append({"shard": sid, "parallel": shards[sid]["parallel"], "projects": keep})
        stacks.append(waves)
    if unknown:
        stacks[0].append({"shard": "other", "parallel": False, "projects": unknown})
    used = [w for w in stacks if w]
    cut = used if split else [[w for ws in used for w in ws]]
    if not cut or all(not w for w in cut):
        cut = [[]]
    return [{"label": topology if len(cut) == 1 else topology + "-" + w[0]["shard"], "waves": w} for w in cut]

def schedule(table, topologies, requested, granted, serial):
    split = [s for t in topologies for s in plan(table, t, requested, True)]
    merged = [s for t in topologies for s in plan(table, t, requested, False)]
    if not serial and granted >= len(split):
        return split, len(split) > 1
    if not serial and granted >= len(merged):
        return merged, len(merged) > 1
    return merged, False

def allocate(parallel, budget, concurrent):
    lively = [i for i, waves in enumerate(parallel) if any(waves)]
    spare = max(0, budget - len(parallel)) if concurrent else 0
    out = []
    for i, waves in enumerate(parallel):
        if not concurrent:
            share = budget
        else:
            share = 1
            if i in lively:
                rank = lively.index(i)
                share += spare // len(lively) + (1 if rank < spare % len(lively) else 0)
        out.append([share if p else 1 for p in waves])
    return out

def budget_of(workers, memory, each, stacks):
    return max(1, min(workers, (memory - stacks) // max(1, each)))

def wave_arguments(forwarded, wave, workers, explicit):
    projects, count, rest = split_arguments(forwarded)
    return rest + [x for p in wave["projects"] for x in ("--project", p)] + ["--workers", explicit if wave["parallel"] and explicit is not None else str(workers)]

def main():
    table = json.load(open(sys.argv[1], encoding="utf8"))
    path = sys.argv[2]
    fixtures = json.load(open(path, encoding="utf8"))
    out = dict(fixtures)
    out["arguments"] = [dict(row, **dict(zip(("projects", "workers", "rest"), split_arguments(row["args"])))) for row in fixtures["arguments"]]
    out["matches"] = [dict(row, matches=matches(row["pattern"], row["project"])) for row in fixtures["matches"]]
    out["stacks"] = [dict(row, stacks=plan(table, row["topology"], row["requested"], row["split"])) for row in fixtures["stacks"]]
    schedules = []
    for row in fixtures["schedules"]:
        stacks, concurrent = schedule(table, row["topologies"], row["requested"], row["granted"], row["serial"])
        schedules.append(dict(row, concurrent=concurrent, labels=[s["label"] for s in stacks]))
    out["schedules"] = schedules
    out["allocations"] = [dict(row, workers=allocate(row["parallel"], row["budget"], row["concurrent"])) for row in fixtures["allocations"]]
    out["budgets"] = [dict(row, budget=budget_of(row["workers"], row["memoryMegabytes"], row["workerMegabytes"], row["stacksMegabytes"])) for row in fixtures["budgets"]]
    out["waves"] = [dict(row, args=wave_arguments(row["forwarded"], row["wave"], row["workers"], row["explicit"])) for row in fixtures["waves"]]
    if "--write" in sys.argv:
        lines = ["{"]
        for index, (key, rows) in enumerate(out.items()):
            body = ",\n".join("    " + json.dumps(row, ensure_ascii=False) for row in rows)
            lines.append('  %s: [\n%s\n  ]%s' % (json.dumps(key), body, "," if index < len(out) - 1 else ""))
        lines.append("}")
        open(path, "w", encoding="utf8").write("\n".join(lines) + "\n")
        print("written", path)
    elif out != fixtures:
        print("MISMATCH between the oracle and", path)
        sys.exit(1)
    else:
        print("oracle agrees with", path)

main()
