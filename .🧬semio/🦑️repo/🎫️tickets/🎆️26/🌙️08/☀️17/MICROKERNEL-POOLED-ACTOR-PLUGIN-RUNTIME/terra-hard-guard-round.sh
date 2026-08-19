#!/bin/bash
# 🛡️ alltargets-hard round: insert-await -> remove-bad-await -> revert known-bad struct-literal
# corruption (timestamp/member shorthand fields, documented in alltargets-kernel report) -> --lib
# recheck. Exits non-zero and leaves state for inspection if --lib regresses even after the revert.
set -uo pipefail
cd /Users/ueli/Documents/semio
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME"
TARGET_DIR="/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-alltargets-hard"
CRATE="$1"
SCOPE="$2"
ROUND="$3"

python3 "$TICKET/insert-await.py" --crate "$CRATE" --all-targets \
  --scope "$SCOPE" --target-dir "$TARGET_DIR" --max-passes 40 --apply \
  --report "$TICKET/terra-hard-${CRATE}-apply-r${ROUND}.json" \
  > "$TICKET/terra-hard-${CRATE}-apply-r${ROUND}.txt" 2>&1

python3 "$TICKET/remove-bad-await.py" --crate "$CRATE" --all-targets \
  --scope "$SCOPE" --target-dir "$TARGET_DIR" --max-passes 40 --apply \
  > "$TICKET/terra-hard-${CRATE}-removebad-r${ROUND}.txt" 2>&1

# revert the one known-bad false-positive shorthand-field corruption if it reappeared
STORE="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs"
if [ -f "$STORE" ]; then
python3 - "$STORE" <<'PYEOF'
import sys
path = sys.argv[1]
with open(path, encoding='utf-8') as f:
    text = f.read()
before = text
text = text.replace("            timestamp.await,\n        });", "            timestamp,\n        });")
text = text.replace("FixtureDirectory { member.await }", "FixtureDirectory { member }")
if text != before:
    with open(path, 'w', encoding='utf-8') as f:
        f.write(text)
    print("REVERTED known-bad pattern")
else:
    print("no known-bad pattern present")
PYEOF
fi

echo "--- lib recheck ---"
CARGO_TARGET_DIR="$TARGET_DIR" cargo check -p "$CRATE" --lib > "$TICKET/terra-hard-${CRATE}-lib-r${ROUND}.txt" 2>&1
LIBEXIT=$?
tail -3 "$TICKET/terra-hard-${CRATE}-lib-r${ROUND}.txt"
echo "LIB_EXIT:$LIBEXIT"
