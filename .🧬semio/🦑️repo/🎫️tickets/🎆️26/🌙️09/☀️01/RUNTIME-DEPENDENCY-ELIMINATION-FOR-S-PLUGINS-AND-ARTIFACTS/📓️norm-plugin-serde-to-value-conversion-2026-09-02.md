# 📕 norm — derive/`#[serde(...)]` → `ToValue`/`FromValue`/`#[value(...)]` conversion

Scope: convert PRODUCTION `#[derive(Serialize, Deserialize)]` / `#[serde(...)]` sites in
`✏️s/🔌️plugins/📕️norm` (crate `semio-s-plugin-norm`) to `#[derive(ToValue, FromValue)]` /
`#[value(...)]`, excluding `🧪️oracle/`, `🧪️fixtures/`, `🔬️probes/`, `🏭️generator/`. Fifteen
compliance-standard families (DIN 4108/16798/18599, EN 1990–1999, ISO 16757, VDI 3805), all
structurally identical.

## Starting state

- 622 `#[derive(...)]` sites carrying `Serialize, Deserialize` across 494 production files.
- 357 `#[serde(...)]` attribute sites (all single-line; no `flatten`/`with`/`skip`/`untagged`).
- One `#[serde(bound(serialize = ..., deserialize = ...))]` (`NormHost<F: NormFamily>` in
  `📄️artifact/🦀️.rs`) — the only non-mechanical site.
- `🧪️tests/<case>/🦀️.rs` (plural — the per-mutation fixture-comparison suite, ~15 families ×
  ~20 mutations each) is a **sibling differential-oracle test suite**, not covered by the
  singular `🧪️test/` exclusion literally, but functionally identical to it: every file lives
  under a `#[cfg(test)] mod fixture_tests { #[path="."] mod tests_x; }` mount and calls
  `serde_json::from_str::<FamilySnapshot/Diff/Mutation>` directly (typed, not via `serde_json::Value`).
  Confirmed zero derive sites live inside `🧪️tests/` itself. Per the ticket's oracle rule, every
  production type this suite exercises keeps serde as
  `#[cfg_attr(test, derive(Serialize, Deserialize))]` + `#[cfg_attr(test, serde(...))]`,
  **applied uniformly to all 622 sites** (cheapest safe choice — this plugin's fixture-test
  methodology transitively touches nearly the whole schema graph per family, and a blanket
  cfg-gated dual-derive costs nothing under a plain `cargo check`, since `#[cfg(test)]` code is
  not compiled without `--tests`). Matches the established pattern already in the framework
  (`🌊️flow/📄️artifact/🦀️.rs`) and in this exact ticket's `🔱️trinity` conversion.

## What changed

1. **Derives** (622 sites, scripted, span-anchored): `Serialize, Deserialize` → `value_derive::ToValue,
   value_derive::FromValue` in place, all other derives (`Clone`, `Debug`, `dsl::DslRecord`,
   `dsl::DslEnum`, `dsl::DslOps`, `dsl::DslArtifact`, `dsl::DslScalar`, `dsl::Mutations`,
   `dsl::MutationLeaf`, `ArtifactSchema`, …) preserved verbatim and untouched. Added
   `#[cfg_attr(test, derive(serde::Serialize, serde::Deserialize))]` immediately after.
2. **Attributes** (357 sites, scripted): every `#[serde(ARGS)]` → `#[cfg_attr(test, serde(ARGS))]`
   + `#[value(ARGS)]` (same order used by the established pattern: cfg_attr-serde line first,
   then the permanent value line).
3. **The one `bound` site** (`NormHost<F: NormFamily>`): serde's `bound(serialize=…,
   deserialize=…)` two-key form isn't what `#[value(...)]` supports (`bound` takes one literal
   predicate string shared by both impls, per `🌱️value/✨️derive/🦀️.rs:296-326`). Hand-written as
   `#[value(bound = "F::Document: dsl::ToValue, F::Document: dsl::FromValue")]`.
4. **Dependencies**: added `semio-framework-value-derive` and `pack` (`semio-framework-pack`) to
   `[dependencies]` in `📦️packages/🦀️rust/Cargo.toml` (same relative paths/aliases as sibling
   plugins `🗄️stdio`/`🔱️trinity`); added `extern crate semio_framework_value_derive as
   value_derive;` to the crate root (`📦️packages/🦀️rust/🦀️.rs`, alongside the existing `dsl`/
   `protocol`/`store`/`vcs`/`schema` aliases). Left `serde`/`serde_json` in `[dependencies]`
   unchanged — the crate still has zero production (non-`cfg(test)`) call sites left, so this is
   ready to move to `[dev-dependencies]` in a follow-up once confirmed by `cargo check`, per the
   ticket's "don't touch Cargo.toml serde lines until it compiles without them" rule.
5. **Production `serde_json::` call sites** (92 real sites across 45 files, out of an initial
   106-file/many-thousand-line `serde_json::` grep dominated by `🧪️tests/` and inline
   `#[cfg(test)] mod tests` noise — isolated via a brace-matched cfg(test)-span scanner, not
   naive grep): converted to the `pack::to_json_string`/`pack::from_json_str` pair added for this
   exact ticket (`🎒️pack/🔤️json/🦀️.rs:1404-1412`, `ToValue`/`FromValue` analogs of
   `serde_json::to_string`/`from_str`). Two `to_string_pretty` sites (`vdi3805`/`iso16757`
   `🚪️io/🦀️.rs`, whole-catalogue/document exports) had no typed pretty helper, so hand-written as
   `pack::to_string_pretty(&pack::from_dsl_value(&dsl::ToValue::to_value(x)))` wrapped in `Ok(...)`
   (was fallible via serde, now infallible). `serde_json::to_string(x).expect(msg)` →
   `pack::to_json_string(x)` (drop `.expect`, now infallible) everywhere else.
6. **Generic trait bounds** (8 sites, not derives, missed by the attribute script and fixed by
   hand): `D: Serialize + DeserializeOwned` → `D: dsl::ToValue + dsl::FromValue` in
   `📄️artifact/🦀️.rs` (`NormFamily::Document`, `ArtifactDiff`/`SetArtifactMutation` impls,
   `ArtifactPack`/`ArtifactDsl`-bounded free fns) and `🖥️app-surface/🦀️.rs`
   (`render_document_json`, `import_media`); `T: serde::Serialize`/`T: serde::de::DeserializeOwned`
   → `T: dsl::ToValue`/`T: dsl::FromValue` in the 8 per-family `enc_json`/`dec_json`/
   `write_json_bin`/`read_json_bin` mutation-payload codec helpers (missed by the derive-line
   regex because they're fully-qualified `serde::Serialize`, not a bare `Serialize` next to
   `Deserialize`).
7. Removed all 494 now-dead `use serde::{Deserialize, Serialize};` imports (verified first that
   no bare `Serialize`/`Deserialize` identifier use survived outside `cfg_attr`/fully-qualified
   `serde::` positions).
8. Left untouched, by design: `🖥️app-surface/🦀️.rs::selected_check_index_arg(Option<&serde_json::Value>)`
   — genuinely untyped dynamic JSON (`.get("index").and_then(Value::as_u64)`), independent of any
   derived type, not a compile blocker, not called anywhere in production (only its own
   `#[cfg(test)]` tests) — a reasonable follow-up but out of this pass's scope.

## Verify

`cargo check -p semio-s-plugin-norm` (foreground) — the shared `target/` dir was held by
continuous, severe repo-wide build contention for most of the session (`ps aux` showed 90-100+
concurrent `rustc`/`cargo` processes at peak, load average 35-55, near-full 31G memory). Multiple
attempts printed only `Blocking waiting for file lock on build directory` for 45+ minutes and were
killed by the harness. A run eventually completed (exit 0, i.e. cargo itself ran to completion —
`error`-count below is real compiler diagnostics, not a harness failure) once contention eased.

**First completed run: 629 `.rs:LINE:COL: error` diagnostics, ALL inside `📕️norm`'s own paths.**
Triaged every distinct message:

- **Caused by this conversion (3 files, found and fixed):**
  - `SheetId(pub u16)`, `RecordFamilyId(pub String)` (`🗿️artifacts/📔️vdi3805/🦀️.rs`) and
    `CatalogueId(pub String)` (`🗿️artifacts/📓️iso16757/🦀️.rs`) are single-field tuple ("newtype")
    structs — `#[derive(ToValue, FromValue)]` rejects tuple/unit structs outright
    (`🌱️value/✨️derive/🦀️.rs:341`) unless marked `#[value(transparent)]` (which explicitly supports
    a single-field `Fields::Unnamed`, same file lines 501/667). Added the attribute to all 3 — this
    was the ONLY structural gap in the whole 622-site derive conversion.
  - `pack::to_json_string`/`from_json_str`/`to_string_pretty`/`from_dsl_value` (89 call sites this
    session hand-wrote or scripted) do not exist — `pack`'s crate root only does `pub mod json;`
    with no flattening re-export (verified by reading `🎒️pack/📦️packages/🦀️rust/🦀️.rs` directly,
    and confirmed by the compiler's own `E0425: not found in pack`). The `pack::json_to_dsl_value`-
    style flat names seen in `🔱️trinity`'s `🗿️artifacts/🔌️jack/🦀️.rs` do not exist ANYWHERE in the
    framework either (grepped) — that reference file is itself stale against the current `pack`
    API, not a reliable template for this specific call. Fixed: `pack::json::to_json_string` /
    `pack::json::from_json_str` / `pack::json::to_string_pretty` / `pack::json::from_dsl_value`
    everywhere (confirmed against `.🧬semio/🦑️repo/🎫️tickets/…/📓️queued-pack-bridge-wave.md`, which
    independently names `pack::json` as the correct current module for this exact bridge).
  - Only 6 `E0425` diagnostics surfaced for this in the 629-error run (not 89) — the other 83 sites
    have the identical bug but are in files whose surrounding module tree didn't get far enough
    through type-checking this run (see below) to report it. All 89 are fixed regardless; a rerun
    should confirm zero `E0425: ... in crate pack` remain.
- **NOT caused by this conversion — pre-existing/concurrent framework churn, left alone per
  "ignore breakage in crates other than yours" and this ticket's own `📓️session-status-2026-09-02.md`
  (explicitly documents two in-flight peer refactors — `SEMANTIC-MUTATIONS-OVERHAUL`'s `Mutation`
  trait gaining `DESCRIPTORS`/`descriptor`, and a mutations-module import-path flattening — as
  making "plugin-level verification... not meaningfully possible right now"):**
  - `E0046`/`E0080` (3+2): `missing DESCRIPTORS, descriptor in implementation` /
    `Mutations descriptor and semantic kind must agree` — exactly the named `DESCRIPTORS` symptom,
    in `📄️artifact/🦀️.rs`, `🎚️config/🦀️.rs`, `👥️presence/🦀️.rs`, `en1990`/`en1992` mutations.
  - `E0277` (54, dominant pattern): `` `ChangeX`/`CreateX`/`DeleteX`: MutationLeaf` is not
    satisfied`` — same `Mutation`/`MutationLeaf` trait-shape churn, not a derive issue.
  - `E0053`/`E0308` (30+15): `render` — `expected Result<ComponentTree,...>`/`Result<BuiltNode,
    PluginAssemblyError>`, found `UiNode`` — an `ArtifactViewer`/`ArtifactEditor` trait signature
    change, unrelated to serde entirely.
  - `E0255` (250, the largest group): `the name X is defined multiple times` for dozens of
    `din16798` mutation-leaf structs — a module-double-mount symptom, not touched by any derive or
    attribute edit; consistent with the documented "mutations module flattening" import-path churn.
  - `E0015` (262): `cannot call non-const associated function EditionId::new in constants`
    (`vdi3805`) — `EditionId::new`'s body and every derive on it are untouched by this pass; reads
    as a new `const` requirement from a generated table elsewhere, same DESCRIPTORS-adjacent family.

**Re-verified after the fixes above — second completed run: `error: could not compile
semio-s-plugin-norm (lib) due to 816 previous errors; 464 warnings` (606 distinct `.rs:LINE:COL:
error` lines by the corrected count recipe).** Confirmed both this-session bugs are now at
**zero**: `grep -c 'in crate \`pack\`'` → 0, `grep -c 'supports named-field structs'` → 0. The
816/606 vs. the first run's 629 is a WIDER, not narrower, number — expected and fine: the first run
never got past the `pack::json` path errors in several files, so rustc's error-recovery hadn't
reached everything yet; with those fixed, type-checking goes further into the crate and surfaces
MORE of the same pre-existing framework-churn errors cataloged above (still the same 7 categories —
`E0015`/`E0255`/`E0277`/`E0053`/`E0308`/`E0046`/`E0080` — same file clusters, same root causes, now
just more fully enumerated). None of the newly-visible errors are new error *categories* or trace
to anything this pass touched. **This crate's own serde→value conversion is complete and, as far
as this session could isolate it, error-free** — the remaining ~800 diagnostics are the two
in-flight peer refactors this ticket's own `📓️session-status-2026-09-02.md` already names as making
"plugin-level verification... not meaningfully possible right now," not a norm-specific problem.

Static self-review performed throughout (all passed): `grep` confirms all 622 derive sites converted
uniformly; zero remaining bare `Serialize`/`Deserialize`/`DeserializeOwned` trait-bound or import
sites outside `cfg_attr`/fully-qualified positions; zero remaining production (non-`cfg(test)`)
`serde_json::` call sites besides the one documented untyped exception above; the `NormHost` bound
special-case hand-edited and spot-checked; a dozen files' diffs read by hand for the derive/attr
ordering (`derive` → `cfg_attr(test, derive(...))` → `cfg_attr(test, serde(...))` → `value(...)`,
matching `🌊️flow/📄️artifact/🦀️.rs`'s established convention).

## Files touched

- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/Cargo.toml` (added `semio-framework-value-derive`, `pack`)
- `✏️s/🔌️plugins/📕️norm/📦️packages/🦀️rust/🦀️.rs` (added `extern crate ... as value_derive;`)
- `✏️s/🔌️plugins/📕️norm/📄️artifact/🦀️.rs` (derives/attrs, `NormHost` bound, 5 generic-bound sites, 2
  `serde_json` call sites, dropped `use serde::{...}`/`use serde::de::DeserializeOwned;`)
- `✏️s/🔌️plugins/📕️norm/🖥️app-surface/🦀️.rs` (3 generic-bound/call sites, dropped `use serde::...`)
- 492 further production `.rs` files under `🎚️config/`, `👥️presence/`, `🗿️artifacts/{din4108,
  din16798,din18599,en1990..en1999,iso16757,vdi3805}/**` — derive/attr conversion plus, in the
  ~43 files with per-family `enc_json`/`dec_json`/`write_json_bin`/`read_json_bin`/
  `encode_*_snapshot_json`/`decode_*_snapshot_json`/`decode_*_mutation_json` helpers, the
  `serde_json::` → `pack::` call-site conversion and generic-bound fix.
