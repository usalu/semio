#!/usr/bin/env python3
"""🔎️ S19 one-off: replays every committed norm mutation fixture case (`🧫️fixtures/🧬️mutations/<leaf>/<case>/`) through
production (the emitter's `apply`): a case is VALID when its `before` and `mutation` decode, production applies it, and the
applied snapshot equals the committed `after` (both normalized through production decode). Read-only on the tree.
usage: s19-fixture-verify.py <emitter-binary> <root> [family …]"""
import json
import os
import subprocess
import sys

binary, root = sys.argv[1], sys.argv[2]
FAMILIES = {"din16798": "🌬️din16798", "din18599": "⚡️din18599", "din4108": "🧱️din4108", "en1990": "⚖️en1990", "en1991": "🏋️en1991", "en1992": "🏛️en1992", "en1993": "🔩️en1993", "en1994": "🧩️en1994", "en1995": "🪵️en1995", "en1996": "🪨️en1996", "en1997": "🌍️en1997", "en1998": "🫨️en1998", "en1999": "🪶️en1999", "iso16757": "📇️iso16757", "vdi3805": "🏭️vdi3805"}
process = subprocess.Popen([binary], stdin=subprocess.PIPE, stdout=subprocess.PIPE, text=True, encoding="utf-8")


def ask(request):
    process.stdin.write(json.dumps(request, ensure_ascii=False) + "\n")
    process.stdin.flush()
    reply = json.loads(process.stdout.readline())
    if "error" in reply:
        raise ValueError(reply["error"])
    return reply["ok"]


def load(path):
    return json.load(open(path, encoding="utf-8")) if os.path.exists(path) else None


for family in sys.argv[3:] or list(FAMILIES):
    base = os.path.join(root, "✏️s/🔌️plugins/📕️norm/🗿️artifacts", FAMILIES[family], "🏅️standards/🔖️1/🪆️subsets/✳️any/🧫️fixtures/🧬️mutations")
    counts, first = {"valid": 0, "stale": 0, "no-after": 0}, None
    for leaf in sorted(os.listdir(base)) if os.path.isdir(base) else []:
        for case in sorted(os.listdir(os.path.join(base, leaf))):
            folder = os.path.join(base, leaf, case)
            before, mutation, after = load(f"{folder}/📸️snapshot/⬅️before/🔣️.json"), load(f"{folder}/🦠️mutation/🔣️.json"), load(f"{folder}/📸️snapshot/➡️after/🔣️.json")
            if after is None:
                counts["no-after"] += 1
                continue
            try:
                applied = ask({"family": family, "op": "apply", "base": before, "mutation": mutation})
                valid = applied["applied"] and applied["after"] == ask({"family": family, "op": "normalize", "snapshot": after})
            except (ValueError, TypeError, KeyError):
                valid = False
            counts["valid" if valid else "stale"] += 1
            if first is None:
                first = (f"{leaf}/{case}", valid)
    print(f"{family}: {counts}; matrix-staged case {first}", flush=True)
