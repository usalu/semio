#!/usr/bin/env bash
# Skipped-animation smoke of every migrated ventilation scene under both profiles (one process at a time).
set -u
T="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
ROOT="$(cd "$T/../../../../../../.." && pwd)"
cd "$ROOT" || exit 2
PY=".🧬semio/🎓️teaching/⚡️cache/python/Scripts/python.exe"
export PYTHONPATH="$ROOT" PYTHONUTF8=1 PYTHONIOENCODING=utf-8 SEMIO_TEACHING_LANGUAGE=en SEMIO_TEST_MANIM=1
export SEMIO_TEACHING_CACHE_DIR="$ROOT/.🧬semio/🎓️teaching/⚡️cache/fonts" SEMIO_TEACHING_OUTPUT_ROOT="$T/🗑️generated/e2a"
OUT="$T/🗑️generated/e2a/smoke.log"
: > "$OUT"
while read -r clip scene; do
  for profile in ${PROFILES:-preview final}; do
    timeout 600 "$PY" "$T/clips-e2a/smoke_scene.py" "$clip" "$scene" "$profile" </dev/null >> "$OUT" 2>&1 || echo "FAIL $clip $scene $profile" >> "$OUT"
  done
done <<'EOF'
🪟️natural Beat1_Fensterregeln
🪟️natural Beat2_StossOderKipp
🪟️natural Beat3_Querlueftung
🪟️natural Beat4_Auftrieb
🪟️natural Beat5_Nachtlueftung
🪟️natural Beat6_GrenzenDerFreienLueftung
❓️need Beat1_Lueftungsbedarf
❓️need Beat2_Lueftungsweg
❓️need Beat3_Fahrplan
📦️volume Beat1_Innenvolumen
📦️volume Beat2_Luftwechselrate
📦️volume Beat3_Volumenstrom
EOF
echo DONE >> "$OUT"
