#!/bin/zsh
# 🧪️ The real tests of assets with a name filter, a chosen set of suites per call so that one grant carries them (reservation figures of MM, takeover brief 2026-11-25 20:20: `🧊️three-views` 2500 MB, `🫁️breathing-catalog` 1950, `⚡️asset-power` and `✅️asset-validation` 1900, `🧊️catalog-silhouette` 1600, `🌡️catalog-physics` 700): tests.sh "<suite key>…" <id>… with the keys silhouette, three-views, physics, breathing, validation, power; one summary line per suite and asset with the md5 of the file and the worker peak, appended to $GEN/proof/summary.log, logs in $GEN/vt (run it inside a grant: a job of lane.sh)
export BATCH=MH-1; source "${0:A:h}/../MR/env.sh"; mkdir -p "$GEN/vt" "$GEN/proof"; keys="$1"; shift
typeset -A suite filter
suite=(silhouette "🧊️catalog-silhouette" three-views "🧊️three-views" physics "🌡️catalog-physics" breathing "🫁️breathing-catalog" validation "✅️asset-validation" power "⚡️asset-power")
for a in "$@"; do
  filter=(silhouette "($a)( |$)" three-views "($a)( |$)" physics "($a)( |$)" breathing "($a):" validation "(accepts the sample figure|validates the manifest|$a)" power "($a):")
  for k in ${=keys}; do
    log="$GEN/vt/$a-$k.log"
    (cd "$S/📦️packages/🟦️typescript" && /usr/bin/time -l -o "$log.time" "$B" x vitest run --config "../../🧪️tests/🎚️config/🟦️.ts" "../../🧪️tests/${suite[$k]}/🟦️.ts" -t "${filter[$k]}" > "$log" 2>&1)
    echo "$(date +%d\ %H:%M) $a $(md5 -q "$S/🗂️catalog/👤️humans/$a/🔣️.json" | cut -c1-8) ${suite[$k]} exit $? $(grep -E '^ *(Test Files|Tests) ' "$log" | tr -s ' ' | tr '\n' ';') peak $(awk '/maximum resident/{printf "%d MB", $1/1048576}' "$log.time")" | tee -a "$GEN/proof/summary.log"
  done
done
