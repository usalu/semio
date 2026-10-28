#!/bin/sh
# oracle-leg.sh <tag> <rule.json> <project> <project.json> <target> -- <command...> : declares the target with the staged engine (rule files + kept explicit globs), runs ONE leg of its command under the Seatbelt read oracle (the new module's CLI), writes <tag>-report.json / -allow / -cli.log in the generated folder.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/keys2-g"; T="$C/keys2-g-tools"; R="🧰️framework/🛍️products/🦑️repo"
TAG="$1"; RULE="$2"; PROJECT="$3"; PJ="$4"; TARGET="$5"; shift 6
ENGINE="$T/stage/🎯️precision"
nice -n 10 bun "$T/derive-rule.ts" "$ENGINE" "$RULE" "$PROJECT" "$TARGET" "$PJ" "$G/$TAG-derived.json" > "$G/$TAG-derive.log" 2>&1
DERIVED_JSON="$G/$TAG-derived.json" nice -n 10 bun "$T/declared-set.ts" "$ENGINE" "$RULE" "$PROJECT" "$PJ" "$TARGET" "$G/$TAG.allow" > "$G/$TAG-declared.log" 2>&1
echo bunfig.toml >> "$G/$TAG.allow"
nice -n 10 bun "$T/stage/🔭️read-oracle/🚪️cli/🟦️.ts" --language en --root "$PWD" --allow-list "$G/$TAG.allow" --allow-name package.json --allow-tree "$T" --allow-tree "$R/🔨️modules/🚀️launcher" --allow-tree "$R/tsconfig.json" --allow-tree "$G" --rounds 40 --timeout 900 --backend auto --compare exit --out "$G/$TAG-report.json" -- "$@" > "$G/$TAG-cli.log" 2>&1
echo $? > "$G/$TAG.done"
