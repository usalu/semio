# W1d — `📕️norm` (`semio-s-plugin-norm`): `.setup()` elimination

## Result: `.setup()` is GONE from this plugin. Zero residue.

Norm's one `.setup()` reason — `crate::config::schema::register_app_schema`, self-registering the
shared `NormConfig` config+presence `AppSchemaDescriptor` under the fixed id `"s.norm.norm"` — is
category-1 app-scope schema, exactly the reason `ArtifactApp::app_schema()` + `register_document_app`
(W1c) already closed for `🗒️note` and (this pass, per the framework agent's report) for `🧩️puzzle`'s
three play apps. Norm's own wrinkle, flagged in the task: **fifteen apps, ONE shared config type**
(`NormConfig`, documented in `🎚️config/🦀️component.rs` as deliberately not per-app). The shared-schema
question resolves cleanly: the field already covers it, because the field is per-*app*, not
per-*plugin*, and nothing stops fifteen apps from returning the identical descriptor.

## What changed

- **`🎚️config/🧬️schema/🦀️component.rs`** — `pub fn register_app_schema()` (self-registering, called
  `::schema::register_app_schema_descriptor(...)` inline) → `pub fn app_schema_descriptor() ->
  ::schema::AppSchemaDescriptor` (returns the identical struct literal, unregistered), mirroring
  `🗒️note`'s `app_schema_descriptor()` exactly. No field of the literal changed — same five
  `include_str!` leaves for `config`, same five for `presence`, same `id: "s.norm.norm"`.
- **All fifteen `🎛️apps/*/🦀️component.rs`** (`din4108`, `din16798`, `din18599`, `en1990`…`en1999`,
  `iso16757`, `vdi3805`) — each `ArtifactApp` impl gained:
  ```rust
  fn app_schema() -> Option<::schema::AppSchemaDescriptor> {
      Some(crate::config::schema::app_schema_descriptor())
  }
  ```
  placed directly after `config_schema()`, before `initial_snapshot()`. All fifteen return the exact
  same descriptor value (same `id`, same five+five `include_str!` leaves) — this is correct, not
  duplication-as-bug: `register_document_app` (framework `🔌️plugin/🦀️component.rs:7178`) calls
  `A::app_schema()` once per `.register_document_app::<A>(...)` in `plugin()`, i.e. fifteen times, each
  feeding `::schema::register_app_schema_descriptor` → `register_kernel_app_schema_descriptor`, which
  is a plain `HashMap<&'static str, KernelAppSchemaDescriptor>::insert` keyed by `descriptor.id`
  (`📡️spr/🧾️wire/🦀️component.rs:355-361`, read this pass to confirm). Fifteen inserts of the same key
  with byte-identical content is an idempotent overwrite, not a race or a conflict — unlike puzzle's
  B2 (OS media-bridge, non-deterministic under concurrent *different* registrants), there is no
  divergent-content case here to be non-deterministic about.
- **`🦀️component.rs`** (plugin root) — `.setup(crate::config::schema::register_app_schema)` line
  deleted; `plugin()`'s doc comment rewritten to state `.setup()` is gone entirely and explain why the
  fifteen-fold `app_schema()` repetition is correct.

No other file touched. `🎚️config`/`👥️presence`/`📄️artifact`/`🖥️app-surface` root dirs left alone per
the task's instruction (previously verified genuinely shared).

## Verification

**`grep -rn 'register_app_schema\b' ✏️s/🔌️plugins/📕️norm/`** — zero matches (was 2: the fn def and
the one `.setup()` call site). **`grep -n '\.setup(' ✏️s/🔌️plugins/📕️norm/🦀️component.rs`** — one
hit, inside a doc-comment sentence explaining `.setup()` is gone, zero live calls.

**`#[path]` resolution** — every `#[path = "..."]` in `📦️packages/🦀️rust/📦️glue.rs` resolved against
the file's own directory: 2319 entries, 0 missing (script-verified, not eyeballed).

**`include_str!`/`include_bytes!` resolution** — every occurrence across all `.rs` files under
`✏️s/🔌️plugins/📕️norm/` resolved relative to its containing file: 580 entries, 0 missing (includes the
ten `include_str!` leaves inside `app_schema_descriptor()`, now called from fifteen call sites instead
of one — same five+five files, no new leaves added).

**`cargo metadata --no-deps`**:
```
RUSTC_WRAPPER="" CARGO_TARGET_DIR=…/🎯️target cargo metadata --no-deps --format-version 1
→ exit 0, empty stderr
```
Log: `scratch-w1d-norm-cargo-metadata.txt` / `scratch-w1d-norm-cargo-metadata-stderr.txt`.

**`cargo check -p semio-s-plugin-norm --all-targets`** — two runs, ~9 minutes apart:

- **First run** (`scratch-w1d-norm-check.txt`): **exit 101, 12 errors**, all inside
  `semio-s-plugin-stdio` (`error[E0433]: cannot find inferences in schema` × 11,
  `error[E0425]: cannot find function register_artifact_inferences in this scope` × 1) — the same
  error class the framework agent's own report already caught mid-flight for the same crate. Grep-
  verified zero error or `-->` lines mention any `📕️norm` path; `Compiling semio-s-plugin-norm` never
  appears in the log — cargo died in the shared upstream `stdio` dependency before reaching norm's own
  crate at all. `stat -f '%Sm'` on the failing stdio file (`🎵️mp3/…/mpeg1-layer3/…/🦀️component.rs`)
  reported `Aug 12 23:42:30`, 9 minutes before this run — inside `🗄️stdio`'s documented "not frozen,
  actively edited" window. Classified **(c) upstream**, not touched (`🗄️stdio` is off-limits per the
  hard rules).
- **Retry** (`scratch-w1d-norm-check-retry.txt`, run ~9 min later): **exit 0.**
  ```
  Finished `dev` profile [unoptimized] target(s) in 2m 16s
  ```
  0 `error` lines, 0 `error[E...]` lines (grep-verified over the full 10730-line log). 264 lib
  warnings + 306 test warnings, all pre-existing style/lint noise (`unused import`, similar), none
  touching `app_schema`, `AppSchemaDescriptor`, or any file this pass edited — spot-checked by name,
  none match. **GREEN, both the `Finished` line and exit status 0 present**, per the ticket's own bar.

## Classification of every error observed this pass

(a) caused by me: **none**. (b) pre-existing/unrelated: none observed in norm's own paths. (c)
upstream: the first run's 12 `semio-s-plugin-stdio` errors — named crate (`semio-s-plugin-stdio`),
quoted (`E0433: cannot find inferences in schema`, `E0425: cannot find function
register_artifact_inferences`), resolved on retry without any change on my part, consistent with
`🗄️stdio` being another session's live in-progress edit, not a stable break.

## Answer to the task's specific question

**Norm's app-scope schema IS fully expressible by `ArtifactApp::app_schema()`, per-app, with no field
change and no invented shape.** The "shared by fifteen" property needed no special handling — a
`fn() -> Option<AppSchemaDescriptor>` override is already per-app by construction, and returning the
same value from all fifteen overrides is exactly as valid as returning fifteen different values,
because the sink (`register_app_schema_descriptor`, keyed by the descriptor's own `id` field) is
idempotent on identical re-registration. No residue.
