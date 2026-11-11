# Reactor lifecycle and runtime migration

## Done
- `semio-plugin-reactor-lifecycle` compiles (`check --lib`, 0 errors; log reactor-lifecycle-3.log). Fix: `⚛️reactor/🏃️runtime/📮️publication/🦀️.rs:56` `owner.step(child.maximum_copy_bytes,&mut effect.nested(child))` (body argument, retained-command precedent).
- decl-emission `📨️emission/🛂️metadata/🔗️view/routing.rs:46`: one `paid` call lacked `(&mut self.construction,&mut self.unit)` (reported to coordinator; not touched again).
- instance-stores `🏪️stores/📥️staging/📮️action-ingress/🦀️.rs`: `close_granted(effect)->OriginalRetirementPoll` (+`completed`, `retired`, `retire_source`), `close_original_step` via `close_original_effect_step`, lifecycle close via `plugin_effect_close`; `🚪️ingress/🦀️.rs`: checkpoint-composition close via `retired_poll`/`begun` with `step_granted(body,effect)`/`begin_granted(pins,effect)`; `advance_local_interaction_query_one(cx,grant,width)` drives `query.close_step(effect)` inside `cx.with_original_effect` (caller in `🔌️plugin-app-impl` updated). Instance agent pinned `ValueError` at :186.
- instance-history `📥️staging/📨️dispatch/🦀️.rs:120`: `Ok::<_,ValueError>` inference pin.
- runtime-instances `🚪️opening/🦀️.rs`: local `built`/`retired` replaced by `semio_plugin_job::original_steps`; `PendingNativeCreation.authority_construction` (new field in `♻️reactor-lifecycle/🧯️cleanup-pumps`) funds the factory flag and authority builds, retired by `unbuilt` and in the closing branch; source/metadata closes via `retired_poll` with `step(body,effect)`.
- runtime-ops `🧰️ops/🖥️surface-render/🦀️.rs`: `plugin_drive_displaced_surfaces(runtime,cx)` (one `retire_displaced_step` per live instance per funded turn), called from `🔄️turn/🌀️poll-kernel-turn/🦀️.rs` before the clock read and fed into `more_work` (`typed_operation_runnable`); `retire_displaced_step` also follows both `update_view` calls (l.120, l.275).
- Test added (not yet run): `removed_surface_view_retires_within_the_next_funded_turns` in `🧰️ops/🧪️tests/original-view-read.rs`.

## Open (check 7, `reactor-runtime-ops-7.log`)
`semio-plugin-runtime-ops` 184 errors (source-only edits of the earlier caller agent never compiled), `semio-plugin-instance-children` 3 errors:
- instance-children `📥️staging/🤝️group/🦀️.rs:68`: `close_empty_backing_step(grant)` now returns `Result<RetainedCloneStep,Fault>`, not an outcome.
- runtime-ops `📥️ingress/🦀️.rs`: l.316 derive `RetireOwned` on `PluginExchangeOutput` needs `RetireOwned` for `ArtifactStoreConstructorKind/Phase`, `OperationId` (24); l.175 `semio_plugin_decl_app::MediaArtifact` moved to `semio_plugin_document_media`; l.320-322.
- `🎞️command/📥️custody/🛂️manifest/{🦀️,metadata,window-view}.rs` (about 60): unresolved crate `semio_framework_value_native_receiving_original` (missing Cargo dependency `semio-framework-value-native-receiving-original`), plus old admit/step signatures.
- `🧾️typed-operations/{intent,response}.rs` (about 20): `selected.step(g).result`, `begin_granted(..,g)` need effect style.
- `🎞️media/🦀️.rs:26`: `begin_document_archive_load(operation,archive)` now takes `(&mut archive, &mut OriginalDocumentArchiveAdmission, cx)`.
Tests of runtime-ops and instance-dispatch not run; instance-dispatch unchecked past children/history.
