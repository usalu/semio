#!/bin/sh
# dev_prepare.sh: patch scratch copies of the live authorizers and closure modules and write absolute-import dev copies.
set -e
cd /Users/ueli/Documents/semio
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"
S=/private/tmp/claude-501/-Users-ueli--claude-spares-spare-2BFGEq/a55b014a-59e5-4214-ad10-dbd1b1cb936d/scratchpad
F1="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🟦️.ts"
F2="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🟦️.ts"
mkdir -p $S/dev
cp "$F1" $S/authorizers.new.ts; python3 "$C/loader-own-tools/patch_authorizers.py" $S/authorizers.new.ts
cp "$F2" $S/closure.new.ts; python3 "$C/loader-own-tools/patch_closure.py" $S/closure.new.ts
python3 "$C/loader-own-tools/dev_rewrite.py" "$F1" $S/authorizers.new.ts $S/dev/authorizers.ts
python3 "$C/loader-own-tools/dev_rewrite.py" "$F2" $S/closure.new.ts $S/dev/closure.ts
