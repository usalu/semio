# 🧪️ Writes one test job of the assets given into $GEN/jobs (source it after ../MR/env.sh with BATCH=MH-2): `mktests <name> <megabytes> <id>…` writes <name>.job: one vitest run per real test file (one worker, all the assets in one name filter, verbose report), logs $GEN/vt/<name>-<suite>.log, one line with counts and failures per suite.
mktests() {
  local n="$1" mb="$2" P="$S/📦️packages/🟦️typescript" J="$GEN/jobs"; shift 2; local ids="${(j:|:)@}"
  { echo "# $mb 14400"; echo "cd \"$P\""; for s in 🧊️catalog-silhouette 🧊️three-views 🌡️catalog-physics 🫁️breathing-catalog ✅️asset-validation ⚡️asset-power; do
    l="$GEN/vt/$n-${s//[^a-z-]/}.log"
    echo "\"$B\" x vitest run --config \"../../🧪️tests/🎚️config/🟦️.ts\" --no-file-parallelism --maxWorkers=1 --reporter=verbose \"../../🧪️tests/$s/🟦️.ts\" -t \"(accepts the sample figure|validates the manifest|$ids)\" > \"$l\" 2>&1; echo \"tests $s exit \$? \$(date +%H:%M) \$(cat \"$J/$n.peak\") \$(grep -E '^ *(Test Files|Tests) ' \"$l\" | tr -s ' ' | tr '\\n' ';') \$(grep -E '^ *(❯|×|FAIL) ' \"$l\" | tr -s ' ' | tr '\\n' ';' | cut -c1-900)\""
  done; } > "$J/$n.job"
}
