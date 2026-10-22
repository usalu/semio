#!/usr/bin/env python3
"""Re-bases the rexec-inhouse edit list on the live text: the cache-gc command delegate takes the remote-execution owner line after the `graph-inference` one (cap-gc), and the caching project manifest keeps its dependsOn names on one line.
usage: rexec-cap-gc.py <edits.py>"""
import sys

path = sys.argv[1]
text = open(path, encoding="utf8").read()
start = text.index('text(B + "/\U0001F9F9️cache-gc/⌨️command/\U0001F7E6️.ts", ')
end = text.index("\n\n", start)
anchor = '   if(owner==="graph"&&id==="graph-inference")return graphReclaim(root,dryRun,language,phaseControl,leases,opening);\n'
remote = '   if(owner==="remote-execution")return import("../../\U0001F310️remote-execution/\U0001F477️worker/\U0001F7E6️.ts").then(module=>module.collectRemoteExecutionForLedgerV1(root,dryRun,phaseControl.signal)).catch(error=>"failed: "+String((error as Error).message));\n'
row = 'text(B + "/\U0001F9F9️cache-gc/⌨️command/\U0001F7E6️.ts", ' + repr(anchor) + ", " + repr(anchor + remote) + ")"
text = text[:start] + row + text[end:]
old = "sub(CACHING, '''        \"remote-execution-probe-platform\",\n        \"remote-execution-probe-descriptor\",\n        \"remote-execution-probe-compose\",\n        \"remote-execution-probe-fanout\"''', '''        \"remote-execution-probe-platform\",\n        \"remote-execution-probe-fanout\"''')"
assert text.count(old) == 1
text = text.replace(old, "sub(CACHING, '\"remote-execution-probe-platform\",\"remote-execution-probe-descriptor\",\"remote-execution-probe-compose\",\"remote-execution-probe-fanout\"', '\"remote-execution-probe-platform\",\"remote-execution-probe-fanout\"')")
open(path, "w", encoding="utf8").write(text)
print("rexec-cap-gc: edit list re-based")
