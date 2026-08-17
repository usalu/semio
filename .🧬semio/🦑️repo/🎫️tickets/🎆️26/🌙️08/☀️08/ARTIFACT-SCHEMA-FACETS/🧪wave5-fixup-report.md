# W5-fixup — shared cross-plugin surfaces

Unlike the mutations refactor, whose wave 4 left 10 of 32 crates broken, this fan-out left only
**3 of 62** plugin crates failing `cargo check`, and all three failures were pre-existing rot that
the new gates merely exposed for the first time.

## Sweep baseline

`🧪wave5-fixup-check-sweep.txt` — `cargo check` across all 62 `semio-s-plugin-*` crates: 59 OK, 3 FAIL.
`🧪wave5-fixup-test-sweep.txt` — `cargo test --lib` across all 62: 58 green, 4 test-build failures.

## Fixed here

| Crate | Root cause | Fix |
| --- | --- | --- |
| `trinity-jack-lsp` | `dsl_lsp` root no longer re-exports the LSP entry points; they moved under the kernel's `lsp` module | glue shim imports `dsl_lsp::lsp::{handle_json_rpc, LanguageSession}` |
| `playbook-procedural` | `crate::playbook` never existed in the extension crate; the playbook domain is `flow::playbook` | corrected the import |
| `playbook-procedural` | `DslOps` no longer emits `OpText`/`OpBinary` (derive comment: "P6: DslOps emits DslVariants only") so `DocumentApp::Command`'s `OpBinary + Send` bound failed | handcrafted `OpBinary` for `Command`, delegating to the shared `dsl::variants_binary` helpers, matching the sibling `ModulePayloadMutation` impl in the same file |
| `playbook-procedural` tests | `module_bundle`/`module_labels` renamed; `handle_action` lost its `ViewModel` parameter; `ViewModel::locale` is now `Locale`, not `Option<String>`; `store::test_support` moved under `os_store` | retargeted to `module_extension_bundle`, `ModuleLabels::labels(Locale, Terminology)`, 3-arg `handle_action`, `store::os_store::test_support` |
| `playbook-procedural` | `envelope_id()` returned the one-segment `procmodule`, which makes `SemioEnvelope::from_envelope_id` fail `InvalidPreamble` at runtime | pinned the two-segment `playbook.procedural` |
| `space` / `os-host-full` | delegated — see `🧪wave5-fixup-os-host-full-report.md` | |

## Dead tests resurrected

Four crates compiled but their test targets did not, so **86 tests had never once run**. Fixing the
build surfaced genuine stale assertions behind them:

- `flow-extension-brep`: a stray `}` closed `mod tests` before its last test, so 17 tests were
  unreachable. Once compiled, `box_emits_geometry_handle` failed and poisoned the shared serial
  mutex, cascading `PoisonError` into 16 siblings. Two real defects: `test_serial()` panicked on a
  poisoned lock instead of recovering (so one failure hid every other result), and the handle
  assertions still expected a `solid-N`/`curve-N` counter scheme, whereas `Brep::mint` has moved to
  content addressing (a blake3 hex digest, with the kind carried in the separate `kind` field).
  18 tests now pass.
- `flow-extension-draw`: `install_extension_bundle` now returns `()`. Behind that, two stale tests:
  the same content-addressed-handle expectation (`drawing-` prefix vs the hex-encoded 32-byte
  content key), and `dispose_drawing_removes_the_handle` taking a *read* guard while mutating the
  process-wide drawing kernel — with content addressing its `make_rect(0,0,5,5)` handle is
  byte-identical to a sibling test's, so the sibling could revive the disposed handle. 42 pass.
- `draw-fsm`: `extern crate self as fsm;` sat in the domain file, but that binding is only effective
  at the crate root, and glue mounts the domain as `mod component` — so every `statechart!`
  expansion naming `fsm::…` was unresolvable. Moved to the glue file, which already owns
  `extern crate` declarations by convention. Separately, 12 `use super::…` paths inside nested test
  modules were one level short. 26 pass.
- `trinity-jack-shell` is binary-only; `--lib` has no target there. Not a failure.

## Deliberately out of scope

`cargo check --workspace` still reports 81 errors, all in two crates that have **no local
modifications** and were already broken at `HEAD`:

- `semio-framework-os-kernel-db` (57) — unresolved `db_storage`/`db_state`/`db_index`/… glue mounts.
  This is the `🛢️db` read-model, which the plan explicitly excludes from the `Projection` sweep.
- `semio-compose-rs` (22) — unresolved `dsl`/`vcs` extern-crate aliases. compose is a separate
  technology that `AGENTS.md` forbids mixing into this work, and the plan lists it out of scope.

Neither is reachable from any `semio-s-plugin-*` crate and neither is caused by this refactor.
