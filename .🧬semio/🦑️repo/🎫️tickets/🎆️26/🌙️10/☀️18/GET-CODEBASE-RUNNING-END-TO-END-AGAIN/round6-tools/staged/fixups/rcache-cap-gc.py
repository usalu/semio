#!/usr/bin/env python3
"""Re-bases the rcache-server edit list on the live text after cap-gc (apply3 + apply4) and the repo-launcher docs sweep: the cache-gc command delegate (`graph-inference`) takes the remote-cache owner line, README anchors carry `repo` instead of `bun repo`.
usage: rcache-cap-gc.py <edits.py>"""
import sys

path = sys.argv[1]
text = open(path, encoding="utf8").read()
start = text.index('text(B + "/\U0001F9F9️cache-gc/⌨️command/\U0001F7E6️.ts", ')
end = text.index("\n\n", start)
anchor = '   if(owner==="graph"&&id==="graph-inference")return graphReclaim(root,dryRun,language,phaseControl,leases,opening);\n'
remote = '   if(owner==="remote-cache")return import("../../\U0001F310️remote-cache/⌨️command/\U0001F7E6️.ts").then(module=>module.collectRemoteCacheForLedgerV1(root,dryRun,phaseControl.signal)).catch(error=>"failed: "+String((error as Error).message));\n'
row = 'text(B + "/\U0001F9F9️cache-gc/⌨️command/\U0001F7E6️.ts", ' + repr(anchor) + ", " + repr(anchor + remote) + ")"
text = text[:start] + row + text[end:]
lines = []
for line in text.split("\n"):
    if line.startswith(("sub(README", "region(README", "text(README")):
        line = line.replace("bun repo", "repo")
    lines.append(line)
open(path, "w", encoding="utf8").write("\n".join(lines))
print("rcache-cap-gc: edit list re-based")
