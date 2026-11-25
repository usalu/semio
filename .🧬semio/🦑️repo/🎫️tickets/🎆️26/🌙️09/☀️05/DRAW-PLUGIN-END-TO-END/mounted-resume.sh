#!/bin/sh
# 🧪️ Resumes the mounted laws from test 6 once the r46 browser build is gone; compiles once, then runs the built binary directly.
cd "$(dirname "$0")" || exit 1
G="$PWD/🗑️generated"
while pgrep -f "draw-e2e-2026-11-23.sh r4[6]" >/dev/null; do sleep 15; done
names="${TMPDIR:-/tmp}/mounted-names.txt"
cat > "$names" <<'N'
mounted_vector_geometry_cancellation_owns_partial_children_until_return
mounted_vector_geometry_failed_slot_is_not_owned_by_its_previous_instance
mounted_vector_geometry_reconciliation_cancels_the_actual_superseded_private_owner
mounted_vector_geometry_real_edit_replaces_only_complete_live_geometry
mounted_painted_scene_holds_actual_source_until_complete_cache_publication
mounted_painted_scene_refuses_real_edit_during_cache_and_retires_before_source_return
mounted_cache_cursor_survives_short_borrows_and_refuses_same_source_replacement_build
mounted_pointer_gestures_borrow_real_cache_and_refuse_rebuilt_scene_before_publication
mounted_publication_closes_the_previous_picture_so_only_the_last_visual_stays
N
first=$(head -n 1 "$names")
sh ./draw-native-tests.sh r6-mounted semio-s-artifact-draw-drawing-scene-mounted -- --exact "tests::$first" --nocapture > /dev/null 2>&1
echo "6 $first $(grep -a '^test result' "$G/native-r6-mounted.txt" | head -1 | cut -c1-80) $(grep -a 'panicked at' -A1 "$G/native-r6-mounted.txt" | grep -av 'controlled\|panicked\|^--' | head -1 | cut -c1-200)" >> "$G/mounted-individual.txt"
bin=$(grep -ao "Running unittests [^(]*([^)]*)" "$G/native-r6-mounted.txt" | tail -1 | sed 's/.*(\(.*\))/\1/')
i=6
for n in $(tail -n +2 "$names"); do
  i=$((i+1))
  if [ -x "$bin" ]; then "$bin" --exact "tests::$n" --nocapture > "$G/native-r$i-mounted.txt" 2>&1
  else sh ./draw-native-tests.sh r$i-mounted semio-s-artifact-draw-drawing-scene-mounted -- --exact "tests::$n" --nocapture > /dev/null 2>&1; fi
  echo "$i $n $(grep -a '^test result' "$G/native-r$i-mounted.txt" | head -1 | cut -c1-80) $(grep -a 'panicked at' -A1 "$G/native-r$i-mounted.txt" | grep -av 'controlled\|panicked\|^--' | head -1 | cut -c1-200)" >> "$G/mounted-individual.txt"
done
echo FINISHED >> "$G/mounted-individual.txt"
