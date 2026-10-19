# Store Bundle and Replay Preservation, Repeat 695

The17 delegated semantic groups are resolved in the current component-bundle and replay owners. No old Store monolith was recreated. No builds/compiler/typechecker/tests/runtime/resolver/generator executed; five changed Rust owners passed masked delimiter guards before fresh-byte guarded atomic writes.

## Composition

The two large methods were reconstructed from immutable B/H/S functions. Original B/S receiver bodies were explicitly rebased to current StoreParts component fields and current helper functions; the conservative whole-function three-way against immutable H then had zero conflicting spans. Current declare_edit_ids ownership was retained with incoming original batch_ids, and current component split/control bodies retained. This replaced earlier partial blocks that still used self in a free function.

Publication validates the original Store snapshot registry, forwards the original receiving body/control parameters through the existing facade, publishes exact prepared diagnostics through a StoreParts helper, preflights/commits the retained lane map and returns the combined lane/diagnostic physical receipt. The current ArtifactStore diagnostic facade delegates to that helper.

Replay observes incoming and ledger edits through the actual authored OriginalReplayEdits source, now mounted in the bundle owner. The original borrowed conflict observation happens before moving batch edits into history. An unchanged B/H copied-edit map and the duplicate old copy-backed degraded-conflict block are retired: immutable S had introduced the original borrowed block but still contained those old blocks. Keeping both would repeat conflict creation, retain cloned operations and refer to already-moved batch input. Current H strict pending-report declaration and replay/candidate retirement remain. The existing independently used public edits_for_ids API remains; these incoming borrowed inspections call original_edits_for_ids.

Replay uses the incoming NativeP mutation bound for effective folds, admission/refusal before applying an outcome with current qualified diff retirement, cold mutation/clone bounds on the named fold functions, and crate visibility for the replay prefix cursor.

| Operation | Authored Delta | Actual Receiving Owner |
| --- | --- | --- |
| 476 | `if !publication.original_registry.as_ref().is_some_and(\|registry\|registry.ptr_eq(&self.snapshot_read_leases)){return Err(VcsError::InverseRefused(ValueError::literal(semio_framework_value::ValueRefusalKind::InvariantViolated,"batched publication requires its original Store issuer")));}` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 479 | `let base = self.batch_item_base(staged_root)?;` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 485 | `let diagnostic_receipt=self.publish_prepared_diagnostics(&mut diagnostics,grant.retained_grant())?;` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 487 | `let lane_diagnostic_copy=publication.stage.as_ref().filter(\|stage\|stage.diagnostics.row_count()!=0).map_or(0,\|_\|ArtifactStoreDiagnosticStage::transfer_copy_demand());                 let lane_grant=RetainedCloneGrant{maximum_copy_bytes:grant.maximum_copy_bytes.saturating_sub(lane_diagnostic_copy),..grant.retained_grant()};                 if let Some(owner)=publication.lane_insert.as_ref(){if !owner.commit_ready(&self.envelope.lanes,lane_grant).map_err(VcsError::InverseRefused)?{return Ok(ArtifactStoreOneItemAdvance::Blocked)}}                 let mut lane_progress=RetainedCloneProgress::default();` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 489 | `let diagnostic_receipt=self.publish_prepared_diagnostics(&mut diagnostics,grant.retained_grant())?;` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 491 | `if let Some(owner)=publication.lane_insert.as_mut(){let(slot,progress)=owner.commit_controlled(&mut self.envelope.lanes,lane_grant,\|lane\|lane).expect("preflight validated the unchanged original lane map");assert!(slot.is_some(),"preflight owns the exact unchanged vacant lane rank");lane_progress=progress;}` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 492 | `Ok(ArtifactStoreOneItemAdvance::Published(receipt,RetainedCloneProgress{copied_items:diagnostic_receipt.copied_items.max(lane_progress.copied_items),copied_bytes:diagnostic_receipt.copied_bytes+lane_progress.copied_bytes,retained_capacity_bytes:diagnostic_receipt.retained_capacity_bytes+lane_progress.retained_capacity_bytes,released_bytes:diagnostic_receipt.released_bytes+lane_progress.released_bytes}))` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 503 | `let replay = StoreReplay::<P, Mutation>::replay_suffix_partitioned(base, &order, k, &OriginalReplayEdits::new(&self.envelope.vcs.edits,&batch), &self.log.envelope.schema, &self.live.supersessions, self.log.merge_policy);` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 504 | `Original edit move/retirement replaces copied scratch ownership.` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 506 | `let  original_edits=OriginalReplayEdits::new(&self.envelope.vcs.edits,&batch);             let quarantined_edits = Self::edits_for_ids(&quarantined_ids, &original_edits)?;` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 509 | `self.insert_reserved_edit_history(reservation, edit)?;             }else{retire_scratch_edits::<P,Mutation>([edit]);` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 510 | `self.replay.pending_report.edit_ids = Some(batch_ids.iter().filter(\|id\| committed_ids.contains(id)).cloned().collect());` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` |
| 601 | `Mutation: self::Mutation<NativeP> + OpBinary,` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧮️effective-operations-and-folds/🦀️.rs` |
| 607 | `let outcome = match outcome.admit() {         protocol::MutationAdmission::Admitted(outcome) => outcome,         protocol::MutationAdmission::Refused(outcome) => {             let (diff, messages) = outcome.into_parts();             D::retire_cold(diff);             return (base.clone(), crate::os_spr::MutationOutcome::new(D::default()).absorb_messages(messages));         }     };` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧮️effective-operations-and-folds/🦀️.rs` |
| 609 | `P: Clone,     Mutation: ColdMutation<P>,` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧮️effective-operations-and-folds/🦀️.rs` |
| 613 | `P: Clone,     Mutation: self::Mutation<P> + ColdMutation<P> + OpBinary,` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧮️effective-operations-and-folds/🦀️.rs` |
| 615 | `pub(crate) prefix_until: usize,` | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧱️replay-owned-state-and-result-types/🦀️.rs` |

## Changed Sources

| Owner | Before SHA256 | After SHA256 |
| --- | --- | --- |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🧳️bundle/🦀️.rs` | `707777dd0c9ea0a3f23e43257df49ec8b0db7d1813da68040475aa66bf5ea59d` | `b2dc885a6c034af36631a0de7542af33cf335cb41664bebda6f77ab40e5d29f2` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🦀️.rs` | `2e7252c003905771523477cbd0764042e90802c0b8d18ba1b8b626f887d0e59c` | `a6b6379f3d16c1f6d710824901a2f828f273995404dc2fbe3fc18e794aeb49fd` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/📥️lifted/🏪️store/🚦️cursor-admission-dispatch-and-apply-batch/🦀️.rs` | `558e71d25d9aed2740645a5b04ef3ab3c5d6a6618db59f6a99c805f6562bc125` | `8ea2db2516c79b77670ab6fd7c49b70451d03bf26bc1bf0557697b29be593853` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧮️effective-operations-and-folds/🦀️.rs` | `63d33e7d32a9fdf1cf275f50a1f746a9e38ce186996b0070efcad89befbe852b` | `bc32feb7f99f0114c3cbd56235e63c6afa17f1de5870da66df37079e79bc763f` |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧱️replay-owned-state-and-result-types/🦀️.rs` | `179502a257d9e2431b5a93abac2dfa5c0a7088d5195aed3e2bbed36d1a99edc2` | `23a10b5458921dc974da260f738c717fac235a0e8cb8aa5f58c190a0d245fccb` |
