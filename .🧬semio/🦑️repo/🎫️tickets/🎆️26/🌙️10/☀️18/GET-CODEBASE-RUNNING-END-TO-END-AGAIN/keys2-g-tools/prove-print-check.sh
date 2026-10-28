#!/bin/sh
# prove-print-check.sh <engine-dir> <tag> : derive the print:check declaration NOW, run the six tsc invocations unsandboxed (baseline) and under the read oracle (Seatbelt, the new module's CLI) with exactly the derived files, compare verdict and diagnostics.
cd /Users/ueli/Documents/semio || exit 1
C=".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN"; G="$C/🗑️generated/keys2-g"; T="$C/keys2-g-tools"
ENGINE="$1"; TAG="$2"
BUN=$(bun -e 'console.log(process.execPath)')
nice -n 10 bun "$T/derive-rule.ts" "$ENGINE" "$T/rules/print-check.json" @semio-tech/print check "🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📋️project.json" "$G/pc-$TAG-derived.json" > "$G/pc-$TAG-derive.log" 2>&1
python3 - "$G/pc-$TAG-derived.json" "$G/pc-$TAG.allow" <<'PY'
import json, sys
r = json.load(open(sys.argv[1]))["results"][0]
open(sys.argv[2], "w").write("\n".join(sorted(set(r["fileList"] + ["bunfig.toml"]))) + "\n")
PY
export SEMIO_RO_BUN="$BUN"
sh -c "sh '$T/print-check-tsc.sh' > '$G/pc-$TAG-baseline.txt' 2>&1; echo \$? > '$G/pc-$TAG-baseline.rc'"
R="🧰️framework/🛍️products/🦑️repo"
nice -n 10 bun "$T/stage/🔭️read-oracle/🚪️cli/🟦️.ts" --language en --root "$PWD" --allow-list "$G/pc-$TAG.allow" --allow-name package.json --allow-tree "$T" --allow-tree "$R/🔨️modules/🚀️launcher" --allow-tree "$R/tsconfig.json" --allow-tree "$G" --rounds 4 --timeout 580 --backend seatbelt --env SEMIO_RO_BUN="$BUN" --env PROOF_OUT="$PWD/$G/pc-$TAG-sandbox.txt" --out "$G/pc-$TAG-report.json" -- sh -c "sh '$PWD/$T/print-check-tsc.sh' > \"\$PROOF_OUT\" 2>&1" > "$G/pc-$TAG-cli.log" 2>&1
echo done > "$G/pc-$TAG.done"
