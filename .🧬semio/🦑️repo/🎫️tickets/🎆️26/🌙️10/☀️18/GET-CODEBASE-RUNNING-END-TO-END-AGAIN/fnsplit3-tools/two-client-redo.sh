#!/bin/zsh
# rebuilds the split of the two-client document e2e from the snapshot taken before any edit
. /Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN/fnsplit3-tools/paths.sh
T="$C/fnsplit3-tools"; D="${F8%/*}"
cd $R
for x in 🌱️space-and-artifact-setup 📐️scenario-types 🔌️sockets-presence-and-commands 🛑️shutdown-and-crash-recovery 🤖️agent-delegation-and-resume 🧭️workspace-paths 🧰️scenario-harness; do rm -rf "$D/$x"; done
cp "$T/orig/two-client.ts" "$F8"
python3 "$T/two-client-prep.py" "$F8" | tail -1
sed -i '' 's/import { findWorkspaceRoot, hubRoot, pick, repoRoot }/import { hubRoot, pick, repoRoot }/' "$F8"
python3 - "$F8" <<'PY'
import sys
p=sys.argv[1]
s=open(p,encoding='utf8').read()
s=s.replace("const TEST_TIMEOUT_MS = 2_400_000;\n\n\n","const TEST_TIMEOUT_MS = 2_400_000;\n\n")
open(p,'w',encoding='utf8').write(s)
PY
L=$(command grep -n "receiveScriptProcessInvocation(process.env,invocation=>" "$F8" | cut -d: -f1)
sed -i '' "s/\"scopeLine\": [0-9]*/\"scopeLine\": $L/" "$T/specs/two-client-letbox.json"
node "$T/letbox.cjs" "$T/specs/two-client-letbox.json" APPLY | tail -1
node "$T/bx.cjs" "$T/specs/two-client.json" APPLY | tail -1
python3 "$T/nest.py" "$D" "$D/📐️scenario-types=$D/🧰️scenario-harness/📐️scenario-types"
