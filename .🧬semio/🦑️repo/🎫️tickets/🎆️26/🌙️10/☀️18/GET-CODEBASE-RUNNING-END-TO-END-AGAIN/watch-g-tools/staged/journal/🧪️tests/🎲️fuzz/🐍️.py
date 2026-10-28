#!/usr/bin/env python3
"""Fuzz peer of the change journal: reuses the deterministic peer of the churn harness (renames, deletes, atomic rewrites, torn writes, swaps, file/directory replacement, directory moves) and applies its whole schedule at once, then waits for its helper threads.

  🐍️.py make --peer PEER.py --work DIR
  🐍️.py ops  --peer PEER.py --work DIR --seed N --count N
  🐍️.py diff SNAPSHOTS.json           (independent delta of {before, after} stamp maps)
"""
import argparse
import importlib.util
import json
import sys

sys.dont_write_bytecode = True


def load(path):
    spec = importlib.util.spec_from_file_location("churn_peer", path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("command", choices=["make", "ops", "diff"])
    parser.add_argument("file", nargs="?")
    parser.add_argument("--peer")
    parser.add_argument("--work")
    parser.add_argument("--seed", type=int, default=0)
    parser.add_argument("--count", type=int, default=6)
    args = parser.parse_args()
    if args.command == "diff":
        data = json.load(open(args.file, encoding="utf8"))
        before, after = data["before"], data["after"]
        print(json.dumps({
            "added": sorted(p for p in after if p not in before),
            "removed": sorted(p for p in before if p not in after),
            "changed": sorted(p for p in after if p in before and after[p] != before[p]),
        }))
        return
    peer = load(args.peer)
    if args.command == "make":
        peer.make_workspace(args.work)
        return
    actor = peer.Peer(args.work, args.seed, "sync", args.count)
    for op in actor.ops:
        actor.apply(op)
    for thread in actor.threads:
        thread.join()
    print(json.dumps({"applied": [row["kind"] for row in actor.applied]}))


if __name__ == "__main__":
    main()
