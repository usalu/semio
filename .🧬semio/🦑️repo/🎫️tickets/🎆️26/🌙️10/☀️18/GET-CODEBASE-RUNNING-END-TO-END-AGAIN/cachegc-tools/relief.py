"""[cachegc] Immediate relief for the full graph optional cache: removes ONLY the sparse pressure leaves that optional-cache test runs left in throw-away dataset namespaces of the live store (logical size >= 1 MiB, zero allocated blocks, every optional leaf of their `contexts/<id>` group sparse, group untouched for 10 minutes). Real records are never sparse and are never touched. Dry run by default; `--apply` unlinks. The next graph writer re-measures the optional ledger on open (used bytes drop to the real records). usage: python3 -I relief.py <repo root> [--apply]"""
import json, os, re, sys, time
root = sys.argv[1]; apply = "--apply" in sys.argv
schema = json.load(open(os.path.join(root, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/💾️storage/🧬️schema/🔣️.json"), encoding="utf8"))
pattern = re.compile(schema["definitions"]["OptionalQuota"]["optionalPath"]); base = os.path.join(root, ".cache/b/g/m/inference"); contexts = os.path.join(base, "contexts")
total = [0, 0]; removed = [0, 0]
for d, _, names in os.walk(base):
    for n in names:
        p = os.path.join(d, n)
        if pattern.match(os.path.relpath(p, base)): s = os.lstat(p); total[0] += s.st_size; total[1] += s.st_blocks * 512
plan = []
for ident in sorted(os.listdir(contexts)) if os.path.isdir(contexts) else []:
    group = os.path.join(contexts, ident); leaves = []
    for d, _, names in os.walk(group):
        for n in names:
            p = os.path.join(d, n)
            if pattern.match(os.path.relpath(p, base)): leaves.append((p, os.lstat(p)))
    if not leaves or any(s.st_blocks or s.st_size < (1 << 20) or s.st_nlink != 1 for _, s in leaves) or time.time() - max(s.st_mtime for _, s in leaves) < 600 and max(s.st_mtime for _, s in leaves) > 86400: continue
    plan += leaves
print(json.dumps({"optionalLogicalBytes": total[0], "optionalAllocatedBytes": total[1], "limit": schema["definitions"]["OptionalQuota"]["policy"]["maximumBytes"], "lowWater": schema["definitions"]["OptionalQuota"]["policy"]["lowWaterBytes"], "sparseTestLeaves": len(plan), "sparseLogicalBytes": sum(s.st_size for _, s in plan), "afterLogicalBytes": total[0] - sum(s.st_size for _, s in plan), "apply": apply}))
for p, s in plan:
    print(("removed " if apply else "would remove ") + os.path.relpath(p, base), s.st_size, time.strftime("%m-%d %H:%M", time.localtime(s.st_mtime)))
    if apply: os.unlink(p)
