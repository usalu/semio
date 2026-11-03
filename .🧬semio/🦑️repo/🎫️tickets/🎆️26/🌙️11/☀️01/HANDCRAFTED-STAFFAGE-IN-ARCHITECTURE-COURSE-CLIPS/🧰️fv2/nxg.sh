#!/usr/bin/env bash
W=/c/git/semio
T="$W/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/HANDCRAFTED-STAFFAGE-IN-ARCHITECTURE-COURSE-CLIPS"
G="$T/🗑️generated/fv2"
until grep -q DONE "$G/nxv.tsv" 2>/dev/null; do sleep 30; done
: > "$G/nxg.tsv"
cd "$W"
mod="🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip.🧪️tests.🎭️presentation.🐍️.GrasshopperPresentationContract"
for m in test_narration_is_bilingual_and_unique test_dispatch_and_step_keys test_declared_chapters_match test_imports_resolve test_provenance_files_exist test_language_is_explicit test_native_play_preserves_scene_sequence; do
  log="$G/logs/nxg-$m.log"
  start=$(date +%s)
  SEMIO_TEACHING_LANGUAGE=en timeout 1500 bash "$T/m9_tests.sh" "$mod.$m" > "$log" 2>&1 < /dev/null
  rc=$?
  printf '%s\t%s\t%s\t%s\t%ss\n' "$m" "$rc" "$(grep -E '^Ran [0-9]+ test' "$log" | tail -1 | awk '{print $2}')" "$(grep -E '^(OK|FAILED)' "$log" | tail -1)" "$(( $(date +%s) - start ))" >> "$G/nxg.tsv"
done
echo DONE >> "$G/nxg.tsv"
