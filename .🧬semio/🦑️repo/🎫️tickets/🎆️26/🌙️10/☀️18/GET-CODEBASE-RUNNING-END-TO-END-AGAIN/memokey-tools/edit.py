"""🔑️ memokey: the graph memo key ignores the per-run graph-yield handover and the granted job count (they changed the key of every identical request, so the memo never hit).
Usage: python3 -I edit.py <root> apply|check|revert   (backups in base/<n>, taken on the first apply)."""
import os, shutil, sys
HERE = os.path.dirname(os.path.abspath(__file__))
M = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧭️invocation/🕸️graph/🧠️memo/"
EDITS = [
    (M + "🧬️schema/🔣️.json", [(
        '          "SEMIO_BUILD_GRAPH_PLAN_OUT",\n          "SEMIO_BUILD_INVOCATION_PID",\n          "SEMIO_BUILD_INVOCATION_PIDS",\n          "SEMIO_BUILD_REUSE_GRAPH",\n',
        '          "SEMIO_BUILD_GRAPH_PLAN_OUT",\n          "SEMIO_BUILD_GRAPH_REQUESTED_AT",\n          "SEMIO_BUILD_GRAPH_YIELD_ROUND",\n          "SEMIO_BUILD_GRAPH_YIELD_TICKET",\n          "SEMIO_BUILD_INVOCATION_PID",\n          "SEMIO_BUILD_INVOCATION_PIDS",\n          "SEMIO_BUILD_PARALLEL",\n          "SEMIO_BUILD_REUSE_GRAPH",\n')]),
    (M + "🧪️tests/🟦️.ts", [(
        'SEMIO_REPO_PROFILE:"1",SEMIO_BUILD_GRAPH_MEMO_OUT:"y"}})).toBe(key);',
        'SEMIO_REPO_PROFILE:"1",SEMIO_BUILD_GRAPH_MEMO_OUT:"y",SEMIO_BUILD_PARALLEL:"3",SEMIO_BUILD_GRAPH_YIELD_TICKET:"/tmp/semio-graph-yield-16498.json",SEMIO_BUILD_GRAPH_REQUESTED_AT:"1793081919763",SEMIO_BUILD_GRAPH_YIELD_ROUND:"2"}})).toBe(key);')]),
]
root, mode = sys.argv[1], sys.argv[2]
status = 0
for n, (rel, pairs) in enumerate(EDITS):
    path, backup = os.path.join(root, rel), os.path.join(HERE, "base", str(n))
    text = open(path, encoding="utf-8").read()
    if mode == "revert":
        shutil.copyfile(backup, path); print("reverted", rel[-50:]); continue
    new = text
    for old, replacement in pairs:
        if replacement in new: continue
        if new.count(old) != 1: print("ANCHOR", rel[-50:], new.count(old)); status = 2; continue
        new = new.replace(old, replacement)
    if mode == "check": print("applied" if new == text else "pending", rel[-50:]); continue
    if new != text:
        if not os.path.exists(backup): shutil.copyfile(path, backup)
        tmp = path + ".memokey"; open(tmp, "w", encoding="utf-8").write(new); os.replace(tmp, path); print("applied", rel[-50:])
sys.exit(status)
