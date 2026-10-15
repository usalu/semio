#!/bin/zsh
# usage: gates-run.sh <copy.ts> <logprefix>
G="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT/🪓️monolith-split/🗑️generated"
cp="$1"; log="$G/$2.log"; : > $log
cd /Users/ueli/Documents/semio/🌎️hub/📦️packages/🦀️rust
for fn in proveRetainedShortAdmin proveGisMapProposalApprovalFixture proveInviteRedemptionTransaction proveDirectorySpaceJourneyV1; do
  echo "=== $fn" >> $log
  perl -e 'alarm 600; exec @ARGV' -- bun -e "import * as m from '$cp'; const r = await m.$fn('/Users/ueli/Documents/semio'); console.log('RESULT', JSON.stringify(r));" >> $log 2>&1
  echo "exit $? $fn" >> $log
done
echo "=== orderedDirectoryPublicationOracle" >> $log
perl -e 'alarm 300; exec @ARGV' -- bun -e "import {orderedDirectoryPublicationOracle as o} from '/Users/ueli/Documents/semio/🌎️hub/📇️directory/📣️publication/🧪️tests/🧾️ordered-append-broadcast/🟦️.ts'; console.log('RESULT', o('/Users/ueli/Documents/semio'));" >> $log 2>&1
echo "exit $? ordered" >> $log
echo ALLDONE >> $log
