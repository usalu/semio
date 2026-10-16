#!/bin/zsh
# 🚦️ usage: gates-run.sh <exported-copy.ts> <log>
cp="$1"; log="$2"; : > $log
cd /Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust
for fn in proveGisMapApprovalUndoFixture proveGisInferenceRetainedRuntimeFixture proveGisInferenceCheckpointControlFixture proveGisMapProposalApprovalFixture proveGisMapFrozenBindingFixture; do
  echo "=== $fn" >> $log
  perl -e 'alarm 300; exec @ARGV' -- bun -e "import * as m from '$cp'; const r = await m.$fn('/Users/ueli/Documents/semio'); console.log('RESULT', JSON.stringify(r));" >> $log 2>&1
  echo "exit $? $fn" >> $log
done
