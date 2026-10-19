#!/bin/zsh
# rebuilds the split of the GIS document open tests from the original snapshot
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/fnsplit3-tools/paths.sh
T="$C/fnsplit3-tools"; D="${F1%/*}"
cd $R
for x in 🧱️cold-pair-fixtures 🪤️cold-pair-worker-fake 🔌️cold-pair-session-setup 🚚️transfer-and-render-phases ⚡️action-and-command-phases ↩️undo-owner-phases 📐️document-open-types; do rm -rf "$D/$x"; done
cp "$T/orig/gis-document-open.ts" "$F1"
node "$T/letbox.cjs" "$T/specs/doc-open-letbox.json" APPLY | tail -1
python3 "$T/doc-open-prep.py" "$F1"
node "$T/bx.cjs" "$T/specs/doc-open.json" APPLY | tail -1
python3 - "$D" <<'PY'
import sys,os
d=sys.argv[1]
def sub(rel,a,b):
    p=os.path.join(d,rel,"🟦️.ts")
    s=open(p,encoding="utf8").read()
    assert a in s,(rel,a)
    s=s.replace(a,b)
    open(p,"w",encoding="utf8").write(s)
sub("🚚️transfer-and-render-phases","await probe.reservation!.activate(state.socket);","await probe.reservation!.activate(state.socket!);")
sub("🚚️transfer-and-render-phases","baseline_frontier: state.frontier,","baseline_frontier: state.frontier!,")
sub("🚚️transfer-and-render-phases","required_tail_frontier: state.frontier,","required_tail_frontier: state.frontier!,")
sub("↩️undo-owner-phases","bindInferenceApprovalUndoToMountedPair(stateB, probe.reservation!, probe.owner);","bindInferenceApprovalUndoToMountedPair(stateB, probe.reservation!, probe.owner!);")
PY
