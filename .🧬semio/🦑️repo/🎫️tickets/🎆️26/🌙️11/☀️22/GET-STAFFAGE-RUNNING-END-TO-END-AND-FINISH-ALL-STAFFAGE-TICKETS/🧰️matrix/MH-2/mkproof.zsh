# 🧾️ Writes the three proof jobs of the assets given into $GEN/jobs (source it after ../MR/env.sh with BATCH=MH-2): `mkproof <prefix> <id>…` writes <prefix>-gate.job (800 MB: the repair once more with `fit2.ts`, which must change nothing, the gate over every subject `../MR/gate.ts --twins`, the official scan), <prefix>-check.job (1000 MB: the check CLI per asset) and <prefix>-tests.job (2500 MB: ONE vitest run per asset over the six real test files, filtered by the asset id and the two manifest cases); logs in $GEN/proof and $GEN/vt.
mkproof() {
  local pre="$1" d="$GEN/proof" P="$S/📦️packages/🟦️typescript" J="$GEN/jobs" a; shift; mkdir -p "$d" "$GEN/vt" "$J"
  { echo "# 800 14400"; for a in "$@"; do
    echo "cd \"$T\"; echo \"=== A $a \$(date +%H:%M)\""
    echo "\"$B\" \"$W/fit2.ts\" \"$a\" --ops \"$W/ops/$a.json\" > \"$d/$a-refit.log\" 2>&1; echo \"refit exit \$? \$(tail -1 \"$d/$a-refit.log\" | cut -c1-160)\""
    echo "\"$B\" \"$M/gate.ts\" \"$a\" --twins > \"$d/$a-gate.log\" 2>&1; echo \"gate exit \$? \$(tail -1 \"$d/$a-gate.log\")\""
    echo "\"$B\" 🔎️scan.ts --language en \"$a\" > \"$d/$a-scan.log\" 2>&1; echo \"scan exit \$? \$(tail -1 \"$d/$a-scan.log\" | cut -c1-200)\""
  done; } > "$J/$pre-gate.job"
  { echo "# 1000 7200"; echo "cd \"$S\""; for a in "$@"; do
    echo "\"$B\" \"🔨️modules/🗂️catalog/🩺️check/🟦️.ts\" --asset \"$a\" > \"$d/$a-check.log\" 2>&1; echo \"check $a exit \$?: \$(grep -m1 -E '^(✓|✗)' \"$d/$a-check.log\" | cut -c1-160)\""
  done; } > "$J/$pre-check.job"
  { echo "# 2500 14400"; echo "cd \"$P\""; for a in "$@"; do
    echo "\"$B\" x vitest run --config \"../../🧪️tests/🎚️config/🟦️.ts\" \"../../🧪️tests/🧊️catalog-silhouette/🟦️.ts\" \"../../🧪️tests/🧊️three-views/🟦️.ts\" \"../../🧪️tests/🌡️catalog-physics/🟦️.ts\" \"../../🧪️tests/🫁️breathing-catalog/🟦️.ts\" \"../../🧪️tests/✅️asset-validation/🟦️.ts\" \"../../🧪️tests/⚡️asset-power/🟦️.ts\" -t \"(accepts the sample figure|validates the manifest|$a)\" > \"$GEN/vt/$a-tests.log\" 2>&1"
    echo "echo \"tests $a exit \$? \$(grep -E '^ *(Test Files|Tests) ' \"$GEN/vt/$a-tests.log\" | tr -s ' ' | tr '\\n' ';') \$(grep -E '^ *(❯|×|FAIL) ' \"$GEN/vt/$a-tests.log\" | tr -s ' ' | tr '\\n' ';' | cut -c1-600)\""
  done; } > "$J/$pre-tests.job"
}
