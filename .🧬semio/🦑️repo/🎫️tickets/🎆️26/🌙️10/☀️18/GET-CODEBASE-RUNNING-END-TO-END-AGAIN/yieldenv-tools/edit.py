#!/usr/bin/env python3
"""🛌️ yieldenv-tools: the captured Bazel planner child no longer inherits the graph-yield handover variables (ticket, requested-at, round of `🛌️yield/🔣️.json`), so a resumed or yielded run does not make the world witness of the next run differ; the earlier ignoredEnvironment entries for them are dropped (they only filtered the enumeration digest).
Usage: python3 -I edit.py <root> apply|check|revert. Anchored, all-or-nothing; backups under yieldenv-tools/base/<n>."""
import os, shutil, sys

HERE = os.path.dirname(os.path.abspath(__file__))
B = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build"
EDITS = [
 (B + "/🔧️tools/🏗️bazel/📜️script.ts", [
  ("import { readBuildGraphV1, requireLiveBuildGraphV1, resolveBuildTargetV1 } from \"../../🕸️graph/🟦️.ts\";",
   "import { readBuildGraphV1, requireLiveBuildGraphV1, resolveBuildTargetV1 } from \"../../🕸️graph/🟦️.ts\";\nimport {graphQueueYieldPolicyV1} from \"../../🕸️graph/🕰️generations/🛌️yield/🟨️.mjs\";"),
  ("env:{...scriptInvocationEnvironment(invocation,process.env),...(witnessed?worldWitnessEnvironmentV1(",
   "env:{...Object.fromEntries(Object.entries(scriptInvocationEnvironment(invocation,process.env)).filter(([name])=>!Object.values(graphQueueYieldPolicyV1().environment).includes(name))),...(witnessed?worldWitnessEnvironmentV1("),
 ]),
 (B + "/🔭️witness/🧬️schema/🔣️.json", [
  ("\"SEMIO_HOST_CAPACITY_PLAN\", \"SEMIO_BUILD_GRAPH_YIELD_TICKET\", \"SEMIO_BUILD_GRAPH_REQUESTED_AT\", \"SEMIO_BUILD_GRAPH_YIELD_ROUND\"]", "\"SEMIO_HOST_CAPACITY_PLAN\"]"),
 ]),
]


def main():
    root, mode = sys.argv[1], sys.argv[2]
    plans, bad = [], []
    for index, (rel, pairs) in enumerate(EDITS):
        path = os.path.join(root, rel)
        text = open(path, encoding="utf8").read()
        new = text
        for old, replacement in pairs:
            if mode == "revert":
                continue
            count = new.count(old)
            if replacement in new:
                continue
            if count == 1:
                new = new.replace(old, replacement)
            else:
                bad.append("%s: anchor %r found %d times" % (rel, old[:60], count))
        plans.append((index, rel, path, text, new))
    if mode == "revert":
        for index, rel, path, _, _ in plans:
            backup = os.path.join(HERE, "base", str(index))
            if os.path.exists(backup):
                shutil.copy2(backup, path)
                print("restored", rel)
        return
    for line in bad:
        print("DRIFT", line)
    if bad:
        sys.exit(2)
    for index, rel, path, text, new in plans:
        print(("pending " if new != text else "applied ") + rel)
        if mode == "apply" and new != text:
            os.makedirs(os.path.join(HERE, "base"), exist_ok=True)
            shutil.copy2(path, os.path.join(HERE, "base", str(index)))
            open(path, "w", encoding="utf8").write(new)


main()
