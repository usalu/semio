#!/bin/sh
# 🔁 [isolate] rebuilds staged batches from the CURRENT live bytes (anchored edits exit 2 on a lost anchor): `rebuild.sh helper` replays the support/optional-cache edits into mirror is1 and restages; `rebuild.sh fence` replays the fence edits of the existing files (new files come from mirror is1) and restages; `rebuild.sh <conversion batch>` re-runs the codemod + manual edits from live and restages (expected.json keeps the measured failure sets). usage: rebuild.sh <batch>...
T="$(cd "$(dirname "$0")" && pwd)"; L=/Users/ueli/Documents/semio; M=/private/tmp/m/is1
for b in "$@"; do
 if [ "$b" = helper ]; then python3 -I "$T/edits/helper_support.py" "$L" "$M" && python3 -I "$T/edits/helper_optional.py" "$L" "$M" || exit 2; fi
 if [ "$b" = fence ]; then python3 -I "$T/edits/fence_existing.py" "$L" "$M" || exit 2; fi
 python3 -I "$T/make_batch.py" "$b" || exit 2
done
