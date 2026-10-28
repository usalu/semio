"""🧾️ Independent Python oracle of what a waiting request stands behind (`hostBlockersV1`): the runs of ours that hold the resource its reason names (memory: what they commit, load: the cores they book, ramp: the grants ramping in a slot, each with the requests admitted before it in the same evaluation; quiet-limit: the quiet grants) or the requests served before it that wait (queue order), largest first, at most three, never its own or a lent grant; nothing for a reason that is the host's own. It reads the stored worlds of the upgrade oracle (decisions and agent-session worlds), decides them with that oracle's port of the decision and names the blockers of every waiting request. Run `python3 🐍️.py` to regenerate `../🧫️fixtures/🔣️.json`, `python3 🐍️.py --check` to prove the stored vectors are what the oracle produces."""
import importlib.util, json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
TARGET = os.path.join(HERE, "..", "🧫️fixtures", "🔣️.json")
UPGRADE = os.path.join(HERE, "..", "..", "🔼️upgrade")
DECISION = os.path.join(HERE, "..", "..", "🧫️fixtures", "🔣️.json")
spec = importlib.util.spec_from_file_location("upgrade_oracle", os.path.join(UPGRADE, "🐍️oracle", "🐍️.py"))
upgrade = importlib.util.module_from_spec(spec); spec.loader.exec_module(upgrade)


def name(row, fallback_id):
    return row.get("label") or "%s %s" % (row["operation"], fallback_id[:6])


def blockers(policy, ledger, waiters, evaluation, ident, now, limit=3):
    decisions = evaluation["decisions"]
    position = next((index for index, row in enumerate(decisions) if row["id"] == ident), -1)
    if position < 0 or decisions[position]["verdict"] != "wait":
        return []
    reason = decisions[position]["reason"]
    def ramping(g): return now - g.get("resumedAtMs", g["admittedAtMs"]) < g.get("rampMilliseconds", policy["memory"]["rampMilliseconds"])
    def committed(g):
        if ramping(g): return max(g["reservedMegabytes"], g["observedMegabytes"] or 0)
        return g["observedMegabytes"] if g["observedMegabytes"] is not None else g["reservedMegabytes"]
    growing = {w["request"]["id"] for w in waiters if w.get("upgrade") is True}
    grants = [g for g in ledger if g.get("lentSinceMs") is None and g["id"] != ident]
    def of_grant(g): return {"label": name(g, g["id"]), "state": ("growing" if g["id"] in growing else "quiet") if g.get("quiet") is True else "running", "jobs": g["jobs"], "megabytes": committed(g)}
    ahead = []
    for row in decisions[:position]:
        waiter = next((w for w in waiters if w["request"]["id"] == row["id"]), None) if row["verdict"] == "wait" else None
        if waiter is not None:
            r = waiter["request"]
            ahead.append({"label": name(r, r["id"]), "state": "waiting", "jobs": r["minimumJobs"], "megabytes": r["fixedMegabytes"] + r["minimumJobs"] * r["perJobMegabytes"]})
    admitted = []
    for row in decisions[:position]:
        waiter = next((w for w in waiters if w["request"]["id"] == row["id"]), None) if row["verdict"] == "admit" and not any(g["id"] == row["id"] for g in ledger) else None
        if waiter is not None:
            r = waiter["request"]
            admitted.append(({"label": name(r, r["id"]), "state": "running", "jobs": row["jobs"], "megabytes": row["reservedMegabytes"]}, r.get("rampSlot") is True))
    if reason == "memory": rows = [of_grant(g) for g in grants] + [row for row, _ in admitted]
    elif reason == "load": rows = [of_grant(g) for g in grants if g["cpu"] > 0] + [row for row, _ in admitted]
    elif reason == "ramp": rows = [of_grant(g) for g in grants if g.get("rampSlot") is True and ramping(g)] + [row for row, slot in admitted if slot]
    elif reason == "quiet-limit": rows = [of_grant(g) for g in grants if g.get("quiet") is True]
    elif reason == "queue-order": rows = ahead
    else: rows = []
    key = (lambda row: row["jobs"]) if reason == "load" else (lambda row: row["megabytes"])
    return sorted(rows, key=lambda row: (-key(row), row["label"]))[:limit]


def generate():
    policy = json.load(open(DECISION, encoding="utf8"))["policy"]
    stored = json.load(open(os.path.join(UPGRADE, "🧫️fixtures", "🔣️.json"), encoding="utf8"))
    vectors, coverage = [], {}
    for world in stored["decisions"] + stored["agents"]:
        variant = {**policy, "memory": {**policy["memory"], "maximumQuiet": world["policy"]["maximumQuiet"]}} if "policy" in world else policy
        evaluation = upgrade.evaluate(variant, world["host"], world["ledger"], world["queue"], world["nowMs"])
        assert evaluation == world["expect"], world["name"]
        for row in evaluation["decisions"]:
            if row["verdict"] != "wait": continue
            expect = blockers(variant, world["ledger"], world["queue"], evaluation, row["id"], world["nowMs"])
            vectors.append({"world": world["name"], "id": row["id"], "expect": expect})
            key = row["reason"] + (":named" if expect else ":host")
            coverage[key] = coverage.get(key, 0) + 1
    return {"vectors": vectors, "coverage": dict(sorted(coverage.items()))}


if __name__ == "__main__":
    value = generate(); text = json.dumps(value, separators=(",", ":"), ensure_ascii=False) + "\n"
    if sys.argv[1:] == ["--check"]:
        stored = open(TARGET, encoding="utf8").read()
        print("vectors equal the oracle" if stored == text else "STALE vectors"); sys.exit(0 if stored == text else 1)
    open(TARGET, "w", encoding="utf8").write(text)
    print("wrote", len(value["vectors"]), "blocker vectors", value["coverage"])
