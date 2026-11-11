#!/bin/bash
C="/Users/ueli/Documents/semio/.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️28/DEMONSTRATOR-END-TO-END-ALL-APPS"
G="$C/🗑️generated/1105b"
i=0
for t in cold_replay::cold_replay_inverse_rows_restore_every_pre_state cold_replay::cold_replay_meters_the_base_snapshot_clone_as_the_bounded_path_does cold_replay::cold_replay_of_n_saved_edits_reopens_to_the_same_snapshot cold_replay::cold_replay_private_semantics_preserve_diagnostics_and_refuse_before_apply config_apply_reaches_prepared_with_independent_scalar_owner long::mutation_apply_admits_large_immutable_documents_with_bounded_changed_paths mutation_apply_prepares_structural_edits_with_ordered_inverse_rows paged_invalid_edit_receipt_retires_returned_inverse_with_original_issuer paged_preparation_preserves_original_admitted_diagnostics_until_typed_retirement quick::mutation_apply_diagnostics_gate_atomic_publication quick::mutation_apply_cancellation_after_decision_preserves_base quick::mutation_apply_recovers_pending_original_replies_exactly_once quick::mutation_apply_refuses_semantic_errors_and_closes quick::mutation_apply_inverse_cancellation_uses_original_factory quick::mutation_apply_rejects_same_data_different_method_table_inverse_issuer; do
 i=$((i+1)); L="W-mutapply6-law$i"
 "$C/1105b-tools/runtime/tst.sh" "$L" Cargo.toml native -p semio-os-store-one-item-batch --test mutation_apply -- --test-threads=1 --exact "$t"
 n=$(grep -n "^running 1 test" "$G/$L.txt" | tail -1 | cut -d: -f1)
 { echo "=== $i $t rc=$(cat $G/$L.rc)"; [ -n "$n" ] && sed -n "$n,\$p" "$G/$L.txt" | grep -v "^[0-9]* [|+-]" | grep -v "^ *at \|^ *[0-9]*: " | head -12 | cut -c1-500; } >> "$G/W-mutapply6-laws.out"
done
echo done >> "$G/W-mutapply6-laws.done"
