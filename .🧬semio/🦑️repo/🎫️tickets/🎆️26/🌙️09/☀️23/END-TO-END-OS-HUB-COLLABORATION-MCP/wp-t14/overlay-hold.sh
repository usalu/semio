#!/bin/zsh
# 🪞️ T14 session-14b resumable overlay hold (one overlay-lane hold, hard 28-min deadline via deadline.py): runs the not-yet-done
# steps of the window-3 candidate proof on the synced overlay (F9 + id recorder + G12 + P8 orphan + H9-L + 5b A/B1[/B2] + item 6
# already applied by `overlay-apply.sh`), kernel first. It stops when KERNEL, SDK or MAP is red or an OWNERS step does not compile
# (the id map needs the owners built); every law and N/W group step is recorded and the hold continues, so one hold shows every red. A step killed at the deadline stays pending; its compiled units survive in the private
# build-dir, so the next hold resumes it. State: `$T14_STATE` or `$L/s14b-state.txt` (`STEP rc HH:MM:SS`). usage: overlay-hold.sh <run-tag> [step…]
O="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-t14-overlay"
L="/Users/ueli/Documents/semio/.🧬semio/🌐hub/s14-t14-logs"
T=/Users/ueli/Documents/semio/.tmp-ticket/wp-t14
TAG="$1"; shift
STEPS=(KERNEL SDK OWNERS-1 OWNERS-1C OWNERS-1P OWNERS-1X MAP OWNERS-2 OWNERS-2R OWNERS-3P MAP2 OWNERS-V OWNERS-VR OWNERS-VP KERNEL-LAWS HOST-LAW WFC3D-LAW HUB-LAW VALUE-LAW ORPHAN-VERDICT ORPHAN-LAWS SDK-REDS TS-ORACLE N1 N2 N3 W1 W2); (( $# )) && STEPS=("$@")
export CARGO_INCREMENTAL=0 RUST_MIN_STACK=33554432 CARGO_BUILD_BUILD_DIR="$O/.t14-build" CARGO_TARGET_DIR="$O/.t14-target" NX_DAEMON=false
source $T/s14b-groups.zsh
DEADLINE=$(( $(date +%s) + 28 * 60 ))
STATE=${T14_STATE:-$L/s14b-state.txt}; touch $STATE
OWNERS=(-p semio-s-artifact-architect-program -p semio-s-artifact-remodel-remodeling -p semio-s-artifact-note-note -p semio-s-artifact-block-3d -p semio-s-artifact-process-process3d -p semio-s-artifact-norm-din18599 -p semio-s-artifact-sequence-sequence -p semio-s-artifact-raster-raster -p semio-s-artifact-sourcing-curation -p semio-s-artifact-gis-gisterrain)
OWNERS_FAST=(-p semio-s-artifact-architect-program -p semio-s-artifact-note-note -p semio-s-artifact-block-3d -p semio-s-artifact-process-process3d -p semio-s-artifact-norm-din18599 -p semio-s-artifact-sequence-sequence -p semio-s-artifact-raster-raster -p semio-s-artifact-sourcing-curation -p semio-s-artifact-gis-gisterrain)
OWNER_FEATURES=semio-s-artifact-block-3d/component-app-assembly,semio-s-artifact-gis-gisterrain/component-app-assembly
run() { python3 $T/deadline.py $DEADLINE nice -n 15 "$@"; }
step() {
  case "$1" in
    KERNEL) run cargo check -p semio-framework-os-kernel --lib --message-format short ;;
    SDK) run cargo check -p semio-framework-plugin --lib --message-format short ;;
    OWNERS-1) rm -f $L/s14b-f9-map-1.tsv; T14_F9_RECORD=$L/s14b-f9-map-1.tsv run cargo test --no-fail-fast ${OWNERS[@]} --features $OWNER_FEATURES --lib ;;
    OWNERS-1C) T14_F9_RECORD=$L/s14b-f9-map-1.tsv run cargo test --no-fail-fast -p semio-s-artifact-sourcing-curation --lib ;;
    OWNERS-1P) T14_F9_RECORD=$L/s14b-f9-map-1.tsv run cargo test --no-fail-fast -p semio-s-artifact-process-process3d --lib -- --skip undo_after_add_step_restores_the_steps_handle ;;
    OWNERS-1X) T14_F9_RECORD=$L/s14b-f9-map-1.tsv run cargo test --no-fail-fast -p semio-s-artifact-writer-writer -p semio-s-artifact-animate-presentation -p semio-s-artifact-playbook-playbook -p semio-s-artifact-imperative-procedure --lib ;;
    MAP) python3 $T/f9/apply-map.py --map $L/s14b-f9-map-1.tsv --root "$O" --write ;;
    OWNERS-2) rm -f $L/s14b-f9-map-2.tsv; T14_F9_RECORD=$L/s14b-f9-map-2.tsv run cargo test --no-fail-fast ${OWNERS_FAST[@]} --features $OWNER_FEATURES --lib ;;
    OWNERS-2R) T14_F9_RECORD=$L/s14b-f9-map-2.tsv run cargo test --no-fail-fast -p semio-s-artifact-remodel-remodeling --lib -- adversarial_feature_match_and_track_worker_steps_stay_fuel_bounded every_bounded_unit_stays_under_the_interactive_ceiling_on_every_example inverse_restores_the_before_document produces_committed_diff reaches_the_committed_after_document remodel_window_ownership_runtime_isolates_renders_and_reopens_six_windows switching_the_selected_table_changes_the_rendered_columns ;;
    OWNERS-3P) rm -f $L/s14b-f9-map-3.tsv; T14_F9_RECORD=$L/s14b-f9-map-3.tsv run cargo test --no-fail-fast -p semio-s-artifact-process-process3d --lib ;;
    MAP2) python3 $T/f9/apply-map.py --map $L/s14b-f9-map-1.tsv --map $L/s14b-f9-map-2.tsv --map $L/s14b-f9-map-3.tsv --root "$O" --write ;;
    OWNERS-V) run cargo test --no-fail-fast ${OWNERS_FAST[@]} --features $OWNER_FEATURES --lib ;;
    OWNERS-VP) run cargo test --no-fail-fast -p semio-s-artifact-process-process3d --lib ;;
    OWNERS-VR) run cargo test --no-fail-fast -p semio-s-artifact-remodel-remodeling --lib -- adversarial_feature_match_and_track_worker_steps_stay_fuel_bounded every_bounded_unit_stays_under_the_interactive_ceiling_on_every_example inverse_restores_the_before_document produces_committed_diff reaches_the_committed_after_document remodel_window_ownership_runtime_isolates_renders_and_reopens_six_windows switching_the_selected_table_changes_the_rendered_columns ;;
    KERNEL-LAWS) run cargo test --no-fail-fast -p semio-framework-os-kernel --lib -- content_id_is_the_specified_sha256_prefix shared_artifact_addressing ;;
    HOST-LAW) run cargo test --no-fail-fast -p semio-framework-os --features os-host-full --lib -- owned_artifact_kind_formats_survive_host_registry_projection ;;
    WFC3D-LAW) run cargo test --no-fail-fast -p semio-s-artifact-wfc-3d --lib -- the_os_artifact_kind_is_the_dimension_namespace ;;
    HUB-LAW) run cargo test --no-fail-fast -p semio-hub --lib -- linked_consumer_descriptors_bind_their_actual_compiled_stdio_dependency_and_catalog ;;
    VALUE-LAW) run cargo test --no-fail-fast -p semio-framework-replication --lib -- dsl_value_literal_matches_serde_json_and_keeps_written_order ;;
    ORPHAN-VERDICT) run cargo test --no-fail-fast -p semio-framework-plugin --features artifact-app-testing --lib -- declared_verb_verdict ;;
    ORPHAN-LAWS) run cargo test --no-fail-fast -p semio-s-artifact-reasoning-wires -p semio-s-artifact-flow-flow -p semio-s-artifact-architect-program --lib -- declared_verb ;;
    SDK-REDS) run cargo test --no-fail-fast -p semio-framework-plugin --lib -- activated_tool_factory_keys_are_an_exact_bijection_with_migrated_declarations a_scene_that_declares_no_lanes_still_publishes_one_childless_surface ;;
    TS-ORACLE) run bun ./📜️script.ts verify shared-artifact-addressing oracle ;;
    N1) run cargo check --keep-going ${N1_P[@]} --features $N1_F --lib --tests --message-format short ;;
    N2) run cargo check --keep-going ${N2_P[@]} --features $N2_F --lib --tests --message-format short ;;
    N3) run cargo check --keep-going ${N3_P[@]} --features $N3_F --lib --tests --message-format short ;;
    W1) run cargo check --keep-going ${W1_P[@]} --lib --target wasm32-wasip2 --message-format short ;;
    W2) run cargo check --keep-going ${W2_P[@]} --lib --target wasm32-wasip2 --message-format short ;;
    *) echo "unknown step $1"; return 2 ;;
  esac
}
cd "$O" || exit 2
echo "HOLD $TAG $(date +%T)"
[ -f $L/s14b-overlay-ready ] || { echo "NOT-READY (no $L/s14b-overlay-ready) $(date +%T)"; exit 3; }
for s in $STEPS; do
  if /usr/bin/grep -q "^$s " $STATE; then continue; fi
  if [ $(date +%s) -ge $DEADLINE ]; then echo "DEADLINE $(date +%T) next=$s"; break; fi
  step $s > $L/s14b-$TAG-$s.txt 2>&1; rc=$?
  echo "$s $rc $(date +%T)"
  if [ $rc -eq 124 ]; then echo "DEADLINE-KILLED $s $(date +%T)"; break; fi
  echo "$s $rc $(date +%T) $TAG" >> $STATE
  if [ $rc -ne 0 ] && { [[ $s == (KERNEL|SDK|MAP) ]] || { [[ $s == OWNERS-* ]] && /usr/bin/grep -q -E 'could not compile|^error: (package|none of the selected|failed to (load|parse|select)|no such|invalid)' $L/s14b-$TAG-$s.txt; }; }; then echo "STOP-RED $s $(date +%T)"; break; fi
done
echo "END $TAG $(date +%T)"
