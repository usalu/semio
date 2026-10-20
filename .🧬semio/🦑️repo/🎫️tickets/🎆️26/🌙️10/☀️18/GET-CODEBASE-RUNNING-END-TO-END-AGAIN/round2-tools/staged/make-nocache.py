#!/usr/bin/env python3
"""Derives the merged copy of bzl-nocache from the CURRENT original: the same apply.ts (absolute import of cache-fix-tools/jsonspan.ts, root from SEMIO_ROOT) and ops.json minus the op of
`@teaching/architecture-course:build`: the course build cache (store receipt, inputs, closure, publish ordering) is owned by x-site/x-proctor/reconcile in the gated course-B group; both sets rewrite
that target and whichever comes second refuses (x-site: hard anchor drifted; reconcile: target already has metadata). Run `prove.sh course` only after course-B."""
import json, os, shutil
HERE = os.path.dirname(os.path.abspath(__file__))
C = os.path.dirname(os.path.dirname(HERE))
SOURCE, TARGET = C + "/bzl-nocache-tools", HERE + "/bzl-nocache"
DROP = {"@teaching/architecture-course:build"}
ops = json.load(open(SOURCE + "/staged/ops.json", encoding="utf8"))
kept = [op for op in ops if op["id"] not in DROP]
assert len(ops) - len(kept) == len(DROP), "the course build op was not found exactly once"
os.makedirs(TARGET + "/staged", exist_ok=True)
text = open(SOURCE + "/apply.ts", encoding="utf8").read()
assert text.count('"../cache-fix-tools/jsonspan.ts"') == 1
open(TARGET + "/apply.ts", "w", encoding="utf8").write(text.replace('"../cache-fix-tools/jsonspan.ts"', '"%s/cache-fix-tools/jsonspan.ts"' % C))
shutil.copyfile(SOURCE + "/staged/plan.ts", TARGET + "/staged/plan.ts")
json.dump(kept, open(TARGET + "/staged/ops.json", "w", encoding="utf8"), ensure_ascii=False, indent=1)
print("merged nocache: %d of %d ops (course build dropped)" % (len(kept), len(ops)))
