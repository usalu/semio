# framework-ext-surfaces

Crates: `semio-plugin-reactor-surfaces` (cold-pair, owns `📸️checkpoint`), `semio-plugin-extension` (owns `💡️inference/{🚪️gateway,🫴️receiving}`, `📨️invoke/⚠️failure`, `🚪️retirement`).

## reactor-surfaces (check --all-targets: 0 errors; test: 8 passed)
- checkpoint `admit_restored_actor`: `SharedUtf8::admit(text, grant, &mut actual)`.
- checkpoint `restore` no longer starts the archive load (it needs a StepContext). `RestoredCheckpoint.document_loads` is now `Vec<RestoredDocumentLoad>` (caller-resident archive plus its own `OriginalDocumentArchiveAdmission`, `begin_step(runtime, cx)` runs `plugin_begin_document_archive_load` until it begins). `RESTORED_DOCUMENT_LOADS` carries these.
- Cargo: new dep `semio-framework-job`; dev-dep `semio-plugin-reactor-lifecycle` feature `original-actor-context-testing` (for `NativeCloseKey::fixture`).
- Test fix: `a_newer_transfer...` now closes the ingress (`close_all`), the registry Drop asserts terminal close.
- OVERLAP: `⚛️reactor/🔄️turn/🧊️cold-pair-loads-and-process-pool` `step_restored_document_loads` edited to call `load.begin_step` before polling (reactor-turn crate, not compiled by me).

## extension (check --all-targets: 0 errors)
- `ExtensionInvocationCause`: manual `RetireOwned` replaced by derive.
- `RetainedExtensionFaultReply::advance`: pack cursor ops via `with_original_effect` (`pack` helper); controlled closes via `close_original_effect_step`.
- inference gateway: effect-shaped `close_step(effect)`, `close_ticket`, `admit(.., effect)`, `with_effect`; JSON decode step through `with_original_effect`; continuation resume/pause errors keep the continuation. Retirement caller uses `plugin_effect_close`.
- inference receiving: `step` is a settled construction over the owner's resident `construction` (`admit_original_construction_settled`, work quote excludes settle overhead); `close_step` via `close_original_effect_step`. Test work assertion now `work >= codec work` (settle overhead is charged on top).
- Removed crate-level `#[global_allocator]` (testing-allocator from job testkit now provides one; conflict).

## Open
- extension `receiving` tests (3 of 17): backtrace shows the panic "original effect producer requires transfer or paid retirement" inside `admit_original` (test line 33, the untouched settled admission with `&mut Default::default()`), in the framework producer drop on the settle path. The owner already retires `construction` in `close_step` and counts it in `terminal_is_empty`, so this is the decoder prerequisite symptom (same as job-receiving/command), not this crate. Stopped there.
- `check -p semio-plugin-reactor-turn --all-targets`: blocked by dependency `semio-framework-plugin-reactor-staged-forms` (`reactor/🔄️turn/📥️receiving/📝️forms/forms.rs:5` unresolved import `semio_framework_plugin_reactor_app`, a crate rename not in my crates). My `step_restored_document_loads` edit is therefore uncompiled; tests not run.
