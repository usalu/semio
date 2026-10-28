"""🐍️ Independent model of the abandoned journal state rule: a state directory goes when no service holds it and its newest change is older than the abandoned age (the quiet time when its worktree is gone). usage: python3 -I oracle.py < {policy, cases}  ->  {"abandoned": [...]}; a case is {held, rootExists (true|false|null), newestMs} or, for the directory law, {lock: none|dead|young|old, rootExists, agesMs: [..]} with `now` fixed."""
import json, sys

def abandoned(held, root_exists, newest_ms, now, policy):
    if held:
        return False
    limit = policy["rootGoneQuietMilliseconds"] if root_exists is False else policy["abandonedAfterMilliseconds"]
    return now - newest_ms >= limit

def main():
    request = json.load(sys.stdin)
    policy, now = request["policy"], request["now"]
    out = []
    for case in request["cases"]:
        if "lock" in case:
            held = case["lock"] == "young"
            newest = now - min(case["agesMs"]) if case["agesMs"] else 0
            out.append(abandoned(held, case["rootExists"], newest, now, policy))
        else:
            out.append(abandoned(case["held"], case["rootExists"], case["newestMs"], now, policy))
    json.dump({"abandoned": out}, sys.stdout)

main()
