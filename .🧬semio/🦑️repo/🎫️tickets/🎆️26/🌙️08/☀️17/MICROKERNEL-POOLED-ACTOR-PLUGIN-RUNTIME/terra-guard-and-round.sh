#!/bin/bash
# 🛡️ One round of insert-await -> remove-bad-await -> revert-known-bad-pattern -> --lib recheck.
# Exits non-zero and leaves state for inspection if --lib regresses even after the revert.
set -uo pipefail
cd /Users/ueli/Documents/semio
TICKET=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME"
TARGET_DIR="/private/tmp/claude-501/-Users-ueli-Documents-semio/e6a44461-bab7-421f-8a53-65123a5e9482/scratchpad/target-at-kernel"
STORE="🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️component.rs"
ROUND="$1"

python3 "$TICKET/insert-await.py" --crate semio-framework-os-kernel --all-targets \
  --scope "🧰️framework/🛍️products/💻️os/🔨️modules" --target-dir "$TARGET_DIR" --max-passes 40 --apply \
  --report "$TICKET/terra-alltargets-kernel-apply-r${ROUND}.json" \
  > "$TICKET/terra-alltargets-kernel-apply-r${ROUND}.txt" 2>&1

python3 "$TICKET/remove-bad-await.py" --crate semio-framework-os-kernel --all-targets \
  --scope "🧰️framework/🛍️products/💻️os/🔨️modules" --target-dir "$TARGET_DIR" --max-passes 40 --apply \
  > "$TICKET/terra-alltargets-kernel-removebad-r${ROUND}.txt" 2>&1

# revert the one known-bad false-positive shorthand-field corruption if it reappeared
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

echo "--- lib recheck ---"
CARGO_TARGET_DIR="$TARGET_DIR" cargo check -p semio-framework-os-kernel --lib > "$TICKET/terra-alltargets-kernel-lib-r${ROUND}.txt" 2>&1
LIBEXIT=$?
tail -3 "$TICKET/terra-alltargets-kernel-lib-r${ROUND}.txt"
echo "LIB_EXIT:$LIBEXIT"
