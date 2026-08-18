# 📓️ terra-sdk-witbindgen — wit-bindgen 0.36.0 → 0.57.1 (guest SDK)

## delivered

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/Cargo.toml:32` — `wit-bindgen = { version = "0.36.0", … }` → `wit-bindgen = { version = "0.57.1", … }`. **The only line changed in the entire packet.**
- `🦀️component.rs`'s `pub mod component { … generate!({ world: "actor", path: "../../🧬️schema" }); … }` block (lines 9–79), `⚛️reactor/🦀️component.rs`, and `🌐host/🦀️component.rs` — **all untouched**. Empirically confirmed (see below) that the version bump needed zero source changes: `world actor`'s exported/imported shape and every generated-path alias survive the jump from 0.36.0 to 0.57.1 unchanged.
- `Cargo.lock` was **not hand-edited** (registrar-only path respected) — `cargo check`/`cargo build`/`cargo test` rewrote it as an ordinary side effect of resolving the new pin. `wit-bindgen 0.57.1` was already present in the lockfile before I touched anything (pulled in transitively by the `wasip2` crate), so this was not a fresh-download risk.

## generated-path migration table

**No migration needed — verified empirically, not assumed.** Every alias A2b established for 0.36.0 resolved identically under 0.57.1; `cargo check --target wasm32-wasip2 --features component-guest` went green with the exact same import lines still in place:

| interface | 0.36.0 path (A2b, unchanged) | 0.57.1 path (this packet, confirmed) | file:line |
|---|---|---|---|
| `reactor`/`jobs`/`checkpoint`/`describe` (world-exported) | `crate::component::component::exports::semio::framework::<interface>` | **identical** | `🦀️component.rs:24-27`, `⚛️reactor/🦀️component.rs:102-105,110,267-268,335-336,351-352,385-386` |
| `effects`/`events`/`types`/`ui` (nested, not directly exported) | `crate::component::component::semio::framework::<interface>` (no `exports::` prefix) | **identical** | `⚛️reactor/🦀️component.rs:102-105` (the four `wit_effects`/`wit_events`/`wit_types`/`wit_ui` aliases) |
| `pure` (import) | `crate::component::component::semio::framework::pure::{log,now_ms,trace_span}` | **identical** | `🌐host/🦀️component.rs:344,355,365` |
| `PluginError` | `semio::framework::types::PluginError` (inside `pub mod component`, one level shallower than the crate-root aliases above) | **identical** | `🦀️component.rs:28` |

No aliases moved, no new ones were needed, and the WIT's one `resource surface;` marker (declared but referenced by no function signature, per its own doc comment in `ui.wit`) generated without incident under 0.57.1 exactly as it did under 0.36.0.

## commands + exit codes

All foreground, single Bash call, `CARGO_TARGET_DIR=<TICKET_DIR>/🎯️target-witb`, `-p` only, verbatim from the saved logs in this ticket folder. Timeline note: the first three attempts at the wasip2 gate hit a **transient, unrelated** blocker in `semio-framework-os-kernel` (a live peer's in-flight `dsl`/`pack`/`spr` module-consolidation refactor, not anything of mine — see `witb-check1.txt` through `witb-check3.txt`, `witb-oskernel-baseline.txt`, `witb-note-build1.txt`/`witb-note-build2.txt`). That refactor landed mid-session; the fourth attempt onward is clean:

```
$ cargo check -p semio-framework-plugin --target wasm32-wasip2 --features component-guest   (witb-check4.txt)
    Checking semio-framework v0.1.0 …
    Checking semio-framework-plugin v0.1.0 …
    Finished `dev` profile [unoptimized] target(s) in 6.48s
EXIT:0
```

```
$ cargo check -p semio-framework-plugin --lib   (witb-check-lib-final.txt)
    Checking semio-framework-plugin v0.1.0 …
    Finished `dev` profile [unoptimized] target(s) in 44.09s
EXIT:0
```

```
$ cargo test -p semio-framework-plugin --lib   (witb-test-final.txt)
test result: FAILED. 242 passed; 5 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.02s
EXIT:101   (all 5 failures on the known baseline named-set — see below; not a real failure of this packet)
```

```
$ cargo build -p semio-s-plugin-note --target wasm32-wasip2   (witb-note-build-final.txt)
   Compiling wit-bindgen v0.57.1
   … (43 pre-existing unused-code/unnecessary-qualification warnings, none new) …
    Finished `dev` profile [unoptimized] target(s) in 2m 42s
EXIT:0
```
Note: the packet brief's suggested run-the-real-thing command was `cargo build -p semio-s-plugin-note --target wasm32-wasip2 --features component-guest`, which fails immediately with `error: the package 'semio-s-plugin-note' does not contain this feature: component-guest` (`witb-note-build1.txt`, EXIT:101) — the exact same brief mistake `📓️status.md` already recorded for E2. `semio-s-plugin-note` has no `component-guest` feature of its own; it unconditionally enables it on its `semio-framework-plugin` dependency. Dropping the flag is the correct command and is what actually ran above.

```
$ cargo build -p semio-framework-plugin-describe   (witb-describe-build.txt)
    Finished `dev` profile [unoptimized] target(s) in 5m 55s
EXIT:0
```

```
$ cargo run -p semio-framework-plugin-describe -- describe <…>/semio_s_plugin_note.wasm --out <TICKET_DIR>/witb-note-descriptor   (witb-describe-run.txt)
described …/semio_s_plugin_note.wasm ("note", role=Plugin) -> …/witb-note-descriptor/🛂️descriptor.semio + 🔣️descriptor.json (wasm_sha256=ae9448d9b7f7140eb3cd2e5ce736a1e68d2485ac50e6d1d424395dc18ee67c73)
EXIT:0
```

Full logs, all kept in the ticket folder: `witb-check1.txt`..`witb-check4.txt`, `witb-check-lib-final.txt`, `witb-test-final.txt`, `witb-oskernel-baseline.txt`, `witb-note-build1.txt`/`witb-note-build2.txt`/`witb-note-build-final.txt`, `witb-describe-build.txt`, `witb-describe-run.txt`.

## named-set test comparison

`cargo test -p semio-framework-plugin --lib` → **242 passed, 5 failed** — matches the coordinator's stated baseline ("242-ish … 5 known failures") exactly by name, no substitutions:

- `component::app::artifact_definition_contract_tests::identities_and_locales_are_explicit_and_conflicts_do_not_overwrite` ✅ matches
- `component::app::artifact_definition_contract_tests::plural_definition_carries_every_artifact_capability_without_a_dispatch_edit` ✅ matches
- `component::app::artifact_definition_contract_tests::registry_rejects_duplicate_schema_dialect_codec_mime_and_extension_claims_atomically` ✅ matches
- `component::plugin_runtime::plugin_builder_contract_tests::merge_channel_commands_preserve_authoritative_policy_conflicts_and_payloads` ✅ matches
- `component::plugin_runtime::plugin_builder_contract_tests::a_child_survives_a_full_persist_and_reload_cycle_through_the_channel_frames` ✅ matches (the known suite-only flake)

**No different test failed. This is the same named set, not a coincidentally equal count** — the wit-bindgen bump introduces zero test regressions.

## real-component proof

Both prongs done, not just one:

1. **Magic-byte check.** `semio_s_plugin_note.wasm` (43 MB debug build) header: `00 61 73 6d 0d 00 01 00` — `\0asm` followed by version/layer `0d 00 01 00`, the WASM **component-model** binary marker (a plain core module would read `01 00 00 00`). This is a real component, not a bare core module.
2. **Describe CLI proof.** `semio-framework-plugin-describe`, which instantiates the component under **wasmtime 47.0.3** and calls its real exported `describe()`, ran clean against this exact artifact: `described …/semio_s_plugin_note.wasm ("note", role=Plugin) -> 🛂️descriptor.semio + 🔣️descriptor.json (wasm_sha256=ae9448…)`, `EXIT:0`. Both output files exist (`witb-note-descriptor/🛂️descriptor.semio` 63 930 bytes, `🔣️descriptor.json` 266 725 bytes) and contain real content (`"role": "plugin"`, `"pluginId": "note"`, a populated `apps[]` array with `s.note.note@1/*#editor`). This is the strongest possible proof available in this environment: guest (wit-bindgen 0.57.1) and host (wasmtime 47.0.3 `bindgen!`) are round-trip compatible on a real, non-trivial plugin, through the exact code path a production host would use.

## honest gaps

- **Transient blocker, now resolved, worth recording for the coordinator.** The first three `component-guest` gate attempts (`witb-check1.txt`-`witb-check3.txt`) and both note-plugin build attempts before the final one failed on `semio-framework-os-kernel`, not on anything of mine — a live peer's in-flight consolidation of `🗣️dsl`/`🎒️pack`/`📡️spr🎮️command` into new locations (including a brand-new `🧰️framework/🔨️modules/⚠️diagnostic/**`), landing through `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` (registrar-only, none of it mine to touch). Proven unrelated by an isolation run — `cargo check -p semio-framework-os-kernel --lib`, no target, no feature, same error — and by symptom drift between attempts (file-not-found → 41 unresolved-import errors → clean), which is what active mid-flight editing looks like, not permanent breakage. I made zero edits to any of those files and filed no lease, since nothing in my mission required touching them and the refactor was visibly moving. It landed on its own within roughly 30 minutes of first hitting it, and every acceptance command has been clean since.
- Beyond the known/expected 5-test baseline (all suite-level, pre-existing, named above), nothing else was skipped. `cargo test` ran the real `--lib` suite, not a filtered subset.
- The `Cargo.lock` diff produced by these commands has not been inspected line-by-line by me (registrar-only file) — I only confirmed it changed as an expected side effect of the version bump and did not hand-edit it.
- Did not attempt the async WIT syntax, did not touch `🧬️schema/📜️component.wit`, did not change `world actor`'s observable behavior — all explicitly out of scope, and none needed for the bump.

## lease-requests

None.
