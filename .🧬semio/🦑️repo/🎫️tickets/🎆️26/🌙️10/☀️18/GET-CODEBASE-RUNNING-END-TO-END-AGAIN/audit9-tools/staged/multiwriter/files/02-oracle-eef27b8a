"""🐍 Independent model of the generation merge decision: reads {"cases": [...]} on stdin and prints {"results": [...]}.

A case names three generations (base, head = what peers published, mine = what this writer built on base) as
{env, global, membership, owners: {name: row}, rows: {name: [row path]}, covered: [names] | null, at: ms | null}
and the writer's index patch {drop: [names], upserts: {name: [paths]}}. The merge is refused when the head moved the
environment, a peer changed an owner's stub (presence, root, kind, targets, config, native root, tag) or the writer changed
one of its owners' stubs while a peer changed any owner, an owner the
writer writes was changed by a peer (manifest row or fact rows), both changed the global fragment or the membership
differently, or a merged owner keeps edges of another environment. Otherwise the merged owners are the head's with
the writer's owners replaced, pseudo owners (the environment and derived facts) keep the union of both row sets, and
the stamp covers the writer's covered owners without the peers' plus the head's covered owners at the older time."""
import json
import sys

STUB = ["root", "kind", "targets", "config", "nativeRoot", "tag"]


def stub(row):
    return None if row is None else [(field, row[field]) for field in STUB if field in row]


def pseudo(name):
    return name == "@env" or name.startswith("@derived:")


def merge(case):
    base, head, mine, patch = case["base"], case["head"], case["mine"], case["patch"]
    if head["env"] != base["env"]:
        return {"conflict": "environment"}
    theirs, ours = set(), set()
    for name in dict.fromkeys([*base["owners"], *head["owners"], *mine["owners"]]):
        before = base["owners"].get(name)
        if head["owners"].get(name) != before:
            if stub(head["owners"].get(name)) != stub(before):
                return {"conflict": "structure", "owner": name}
            theirs.add(name)
        if mine["owners"].get(name) != before:
            ours.add(name)
    for name in [*patch["drop"], *patch["upserts"]]:
        if not pseudo(name):
            ours.add(name)
    if theirs:
        for name in sorted(ours):
            if stub(mine["owners"].get(name)) != stub(base["owners"].get(name)):
                return {"conflict": "structure", "owner": name}
    for name in sorted(ours):
        if name in theirs or sorted(set(base["rows"].get(name, []))) != sorted(set(head["rows"].get(name, []))):
            return {"conflict": "owner", "owner": name}
    if head["global"] == base["global"]:
        merged_global = mine["global"]
    elif mine["global"] in (base["global"], head["global"]):
        merged_global = head["global"]
    else:
        return {"conflict": "global"}
    if mine["membership"] == base["membership"]:
        membership = head["membership"]
    elif head["membership"] in (base["membership"], mine["membership"]):
        membership = mine["membership"]
    else:
        return {"conflict": "membership"}
    owners = dict(head["owners"])
    for name in ours:
        if name in mine["owners"]:
            owners[name] = mine["owners"][name]
        else:
            owners.pop(name, None)
    if any(row["edgesEnv"] != mine["env"] for row in owners.values()):
        return {"conflict": "environment"}
    upserts = {name: sorted(set(rows) | (set(head["rows"].get(name, [])) if pseudo(name) else set())) for name, rows in patch["upserts"].items()}
    covered = None
    if mine["covered"] is not None:
        names = {name for name in mine["covered"] if name not in theirs}
        at = mine["at"]
        if head["covered"] is not None:
            names |= set(head["covered"])
            at = min(at, head["at"])
        covered = {"names": sorted(name for name in names if name in owners), "at": at}
    return {"conflict": None, "owners": {name: owners[name] for name in sorted(owners)}, "global": merged_global, "membership": membership,
            "upserts": {name: upserts[name] for name in sorted(upserts)}, "theirs": sorted(theirs), "ours": sorted(ours), "covered": covered}


def main():
    json.dump({"results": [merge(case) for case in json.load(sys.stdin)["cases"]]}, sys.stdout)


main()
