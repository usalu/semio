"""Independent model of the owner row claims of the graph shaping (stdlib only): the naive scan the sorted range lookup must equal.

usage: oracle.py expect FIXTURE   prints, for every world, the rows of every owner (indices into the world's row list) and the rows of the environment"""
import json, sys


def key(row):
    return json.dumps([row["kind"], row["path"], row["fact"]], sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def prefix(workspace, root, separator="/"):
    return None if not root or root == "." else workspace + separator + separator.join(root.split("/")) + separator


def solve(world):
    workspace, rows = world["workspaceRoot"], world["rows"]
    keys = [key(row) for row in rows]
    claimed, owners = set(), {}
    entries = [(owner["name"], owner["nodeRows"], [owner["root"], owner.get("nativeRoot")]) for owner in world["owners"]]
    entries += [(owner["name"], [], [world["stubs"][owner["name"]]["root"], world["stubs"][owner["name"]].get("nativeRoot")]) for owner in world["edgeOnly"]]
    for name, base, roots in entries:
        seen = {keys[at] for at in base}
        found = list(base)
        prefixes = [p for p in (prefix(workspace, root) for root in roots if root) if p]
        if prefixes:
            for at in world["partRows"]:
                row = rows[at]
                if any(row["path"].startswith(p) for p in prefixes) and keys[at] not in seen:
                    seen.add(keys[at])
                    found.append(at)
        claimed |= seen
        owners[name] = found
    environment = [at for at in world["partRows"] if keys[at] not in claimed]
    return {"owners": owners, "environment": environment}


if __name__ == "__main__":
    fixture = json.load(open(sys.argv[2], encoding="utf8"))
    print(json.dumps([solve(world) for world in fixture["worlds"]], ensure_ascii=False))
