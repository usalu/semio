# Serde/serde_json Fan-Out — 🌊️flow Extensions (9 manifests)

Batch: the nine `✏️s/🔌️plugins/🌊️flow/🧩️extensions/*/📦️packages/🦀️rust/Cargo.toml` manifests
(`🏗️bim`, `📃️list`, `📐️brep`, `📖️dictionary`, `📝️text`, `🔤️primitive`, `🖍️draw`, `🧠️logic`,
`🧮️math`). The `🌊️flow` plugin itself (`✏️s/🔌️plugins/🌊️flow/📦️packages/🦀️rust/Cargo.toml`) was
explicitly out of scope and untouched.

## Headline

All 9 manifests: **`serde`/`serde_json` deleted from `[dependencies]`, zero third-party
entries remain.** All 9 source files (`🦀️component.rs`): **zero remaining `serde`/`serde_json`
usage** (grep-verified; only doc-comment prose mentioning "serde" as a concept survives).

None of these 9 crates implement `Mutation`/`MutationDiff` (the trait-bound root cause the pilot
fixed) — they are operator-registry + WIT extension-bundle glue crates, not artifact/mutation
types. So this batch did **not** use the `ToValue`/`FromValue` derive machinery
(`🌱️value/🔁️codec`, `🌱️value/✨️derive`) the ticket's foundation section pointed at. Instead it
used the plain first-party JSON tree (`pack::json::Value`, `semio-framework-pack`) plus two new
small helpers added to the shared `flow_extension_sdk` (`semio-framework-os-flow`, a
`🧰️framework/**` crate — sanctioned to carry `serde` itself, see plan.md's Definition of Done).

## What the serde usage actually was, per file — and what it became

Every one of the 9 files shared the same four-shape template (confirmed near-identical across
all nine before editing):

1. **`EvaluateRequest { operator_id, input_json }`** — `#[derive(serde::Deserialize)]
   #[serde(rename_all = "camelCase")]`, decoded from the WIT `extension::invoke` "evaluate"
   capability's raw request bytes. Appeared twice per file (once inline in a `mod tests` bundle
   test, once in `mod extension_guest`'s real `bundle()`).
2. **`flow_extension_contribution(app_id, manifest_json) -> serde_json::Value`** — built the
   `{"appId","extensionId","label","iconId","manifestJson"}` topic-contribution payload via
   `serde_json::json!(...)`, called twice per file (flow-play, procedural3d-play).
3. **`bundle_identity_matches_catalogue_fixture`** — parsed the shared
   `🌊️flow/🧩️extensions/🧪️fixtures/🔣️package-identities.json` fixture with
   `serde_json::from_str::<serde_json::Value>`, compared against `serde_json::to_value(&bundle.manifest)`
   and `contribution.decode::<serde_json::Value>()`.
4. A handful of files (`bim`, `list`, `dictionary`, `text`, `primitive`, `logic`, `math`) also had
   one `evaluate_json_*` test that round-tripped a `Dictionary` through JSON text via
   `serde_json::to_string`/`from_str`. `brep` and `draw` additionally had many small
   `let x: serde_json::Value = serde_json::from_str(&some_json_returning_helper(...))` reads
   against already-JSON-text-returning framework helpers (`export_solid_json`, `import_solid_json`,
   `export_svg_json`, `export_pdf_json`, `render_scene_json`, `trace_bitmap_json`,
   `boolean_segments_json`). `brep` alone also had a `TessellateRequest { handle, tolerance }`
   struct with `#[serde(default = "...")]`, and `math` had one `FlowExtensionSetting.default:
   serde_json::json!(1)` literal.

### Rewrite strategy (framework extension, not a plugin workaround)

Rather than reproduce shapes 1–3 nine times, I extended the SDK all nine already depend on
(`flow_extension_sdk` = `semio-framework-os-flow`, a `🧰️framework/**` crate, file
`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️component.rs`) with two new
functions, mirroring the pattern the imperative-fanout agent already proved at
`✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️component.rs::evaluate_invoke`/
`imperative_module_topic_contribution`:

- `pub fn evaluate_invoke_json(registry: &Registry, request: &[u8]) -> Result<Vec<u8>, String>` —
  owns the `EvaluateRequest` decode + dispatch, so no extension crate needs
  `serde`/`serde::Deserialize` at all for shape 1.
- `pub fn flow_extension_topic_contribution(app_id, extension_id, label, icon_id, manifest_json)
  -> semio_framework::TopicContribution` — owns shape 2's payload construction.
- `pub fn integer_setting_default(value: i64) -> serde_json::Value` — the one-off for `math`'s
  `FlowExtensionSetting.default`.

**Mid-batch collision (expected, handled per the ticket's own rules):** a concurrent agent (the
pilot, per status.md, "redirected to repair os-kernel") edited this same SDK file while I was
mid-batch and changed `TopicContribution`'s `payload` field from `serde_json::Value` to `DslValue`
(and `TopicContribution::decode` from `serde::de::DeserializeOwned` to `FromValue`), and
`ExtensionBundle::contributes_topic`'s `payload` parameter to `DslValue` to match. My
`flow_extension_topic_contribution` body was overwritten by their edit to build a `DslValue`
directly instead of `serde_json::json!(...)` — I did not revert it (CLAUDE.md: never revert a
peer). I re-read the live file, confirmed the new `TopicContribution { payload: DslValue }` shape,
and fixed the one place in each of my 9 files that assumed the old `serde_json::Value` shape
(`contribution.payload.to_string()` → `contribution.payload.get("extensionId").and_then(|v|
v.as_str())`, since `DslValue` has no `Display` impl but does have `.get`/`.as_str()` directly —
this is actually simpler than my original `pack::json` round-trip).

Shape 3's fixture parse itself uses `pack::json::parse` (first-party, `semio-framework-pack`) — the
fixture file is plain static JSON, unrelated to `Mutation`/`DslValue`.

Shape 4's JSON-text round-trips (`evaluate_json`, `export_solid_json`/etc.) were rewritten to build
input literals and parse output with `pack::json::object`/`to_string`/`parse`/`parse_bytes` — these
never touch `Dictionary`'s own (still-serde) `Serialize`/`Deserialize` impl; they hand-build the
wire JSON that mirrors the schema/value shape (`{"$schema":"number","value":N}`) directly, since the
test authors already fully controlled that shape. `brep`'s `TessellateRequest` was replaced by
direct `pack::json::Value` field reads plus `tessellate_geometry_json_for_wasm` (an
already-existing JSON-text-returning framework function this file wasn't using yet, which
sidesteps needing `serde_json::to_vec(&mesh)` entirely).

## Per-manifest detail

| crate | Cargo.toml change | source rewrite | resolve-checked path dep |
|---|---|---|---|
| `🏗️bim` | `-serde -serde_json +pack` | full template + `evaluate_json_wall` JSON literal | yes |
| `📃️list` | same | template + `sample_list`-shaped JSON literal | yes |
| `📖️dictionary` | same | template + `sample_dict`-shaped JSON literal | yes |
| `📝️text` | same | template + text-value JSON literal | yes |
| `🔤️primitive` | same | template + bare-atom JSON literal (`{"value":"hi"}`) | yes |
| `🧠️logic` | same | template + boolean-result JSON literal | yes |
| `🧮️math` | same | template + `integer_setting_default` (×2 call sites) + sum JSON literal | yes |
| `🖍️draw` | same | template + 16 generic `serde_json::Value` reads → `pack::json::parse` swap | yes |
| `📐️brep` | same | template + 9 generic reads → `pack::json::parse` swap + `TessellateRequest` → `pack::json` + `tessellate_geometry_json_for_wasm` | yes |

Every `pack = { path = "...🎒️pack/📦️packages/🦀️rust", package = "semio-framework-pack" }` line uses
the same `../../../../../../../` (7-level) prefix every sibling dependency in that same manifest
already uses — copied verbatim from the neighbouring `neural_engine`/`flow_extension_sdk` lines,
not re-derived. Resolved with `ls -d <manifest-dir>/<relative-path>` for all 9 before editing the
source (see method note in the ticket prompt) — all 9 resolved on the first try.

## Framework surface touched (in scope per plan.md — `🧰️framework/**` is the platform layer)

`🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧩️extensions/🕸️wasm/🦀️component.rs`
(`semio-framework-os-flow`, already in every one of the 9 manifests' `[dependencies]`, already
carries `serde`+`serde_json` itself and is unaffected by the goal): added
`evaluate_invoke_json`, `flow_extension_topic_contribution`, `integer_setting_default`. Purely
additive — no existing function signature changed by me. (A concurrent edit by another agent did
change `flow_extension_topic_contribution`'s body and `TopicContribution`'s shape after I added
it, described above; I adapted my callers rather than fighting it.)

## Verification — honest

**Machine state during this batch:** `ps aux | grep 'cargo check'` repeatedly showed 15-25
concurrent `cargo`/`rustc` processes from other sessions throughout, including another agent
already running `cargo check --target wasm32-wasip2 --lib -p semio-s-plugin-flow-extension-brep
-p semio-s-plugin-flow-extension-dictionary -p semio-s-plugin-flow-extension-draw -p
semio-s-plugin-flow-extension-list -p semio-s-plugin-flow-extension-logic -p
semio-s-plugin-flow-extension-math -p semio-s-plugin-flow-extension-primitive -p
semio-s-plugin-flow-extension-text ...` (i.e. 8 of my 9 crates) alongside many others.

I ran, foreground, one at a time, per the ticket's rules:
- `cargo check -p semio-framework-os-flow --message-format=short` — sat at "Blocking waiting for
  file lock on build directory" for the full ~10 minutes before I killed it to reduce contention
  (verbatim tail: `Blocking waiting for file lock on build directory`).
- `cargo check -p semio-s-plugin-flow-extension-logic --message-format=short` (foreground, 600s
  timeout) — moved to background by the harness after 600s with **zero CPU time accrued**
  (`ps -o etime,time,%cpu` showed `13:32 elapsed / 0:00.44 cpu / 0.0%`), i.e. still purely
  lock-blocked, not compiling.

Neither `CARGO_TARGET_DIR` override nor `run_in_background`/Monitor were used, per the ticket's
explicit ban.

**Verdict: WRITTEN BUT UNVERIFIED for all 9 crates by a passing/failing compiler run.** I did not
observe a single compiler diagnostic naming any of my 9 crates or `semio-framework-os-flow` — the
only observed failure mode was the build-directory lock, which the ticket explicitly says to
record and move past rather than chase.

What I verified by other means (not a substitute for a compiler run, but real):
- Every new/changed call site was checked against the **live, just-re-read** signatures of
  `ExtensionBundle::contributes_topic`, `TopicContribution::new`/`::payload`, `DslValue::get`/
  `::as_str`/`::object`/`::String`, `pack::json::{Value, object, array, parse, parse_bytes,
  to_string}`, `Fault::new`, and `flow_extension_sdk::{evaluate_invoke_json,
  flow_extension_topic_contribution, integer_setting_default}` — including re-reading after the
  concurrent agent's mid-batch edit to `TopicContribution`/`contributes_topic`, and fixing every
  one of the 9 `bundle_identity_matches_catalogue_fixture` tests to match the new `DslValue`
  payload shape.
- `grep -n serde` on all 9 `🦀️component.rs` files and all 9 `Cargo.toml` files: zero code hits
  (only doc-comment prose survives, e.g. "instead of `Dictionary`'s own `serde` codec").
- All 9 new `pack` path dependencies resolve on disk (`ls -d`, all 9 passed).

## Recommendation for whoever verifies next

Run, foreground, one at a time, once the machine quiets down:
```
cargo check -p semio-framework-os-flow --message-format=short
cargo check -p semio-s-plugin-flow-extension-bim --message-format=short
cargo check -p semio-s-plugin-flow-extension-list --message-format=short
cargo check -p semio-s-plugin-flow-extension-dictionary --message-format=short
cargo check -p semio-s-plugin-flow-extension-text --message-format=short
cargo check -p semio-s-plugin-flow-extension-primitive --message-format=short
cargo check -p semio-s-plugin-flow-extension-logic --message-format=short
cargo check -p semio-s-plugin-flow-extension-math --message-format=short
cargo check -p semio-s-plugin-flow-extension-draw --message-format=short
cargo check -p semio-s-plugin-flow-extension-brep --message-format=short
```
`semio-framework-os-flow` first — every one of the 9 depends on it and on its two new functions.

## Verification update — a real compile run landed

The `cargo check -p semio-s-plugin-flow-extension-logic --message-format=short` run above
(started foreground, moved to background by the harness after its 600s timeout, completed on its
own ~15 minutes later once the build-directory lock cleared) **did execute**, unlike the earlier
attempts. Full pipeline observed, in order: `semio-framework-os-kernel` (33 pre-existing
warnings, unrelated to this batch — redundant-pattern-field and similar lints), `semio-framework-3d`,
`semio-framework-graph`, `semio-framework-schema`, `semio-framework-ui`/`semio-framework-ui-backend-webgpu`
(135 pre-existing warnings, all in `🎯️targets/🧊️wgpu/*.rs` — unused imports/dead code, unrelated),
`semio-framework-compiler`, then **`semio-framework-2d`**, which failed:

```
🧰️framework/🔨️modules/◻2d/📦️packages/🦀️rust/../../../../🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs:114:20:
error[E0433]: cannot find module or crate `semio_framework_hash` in this scope:
use of unresolved module or unlinked crate `semio_framework_hash`
error: could not compile `semio-framework-2d` (lib) due to 1 previous error
warning: build failed, waiting for other jobs to finish...
```

This is `🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs` (the content-addressed engine-cache
module `semio-framework-2d` mounts), calling `semio_framework_hash::hash(...)` without that crate
declared as a dependency wherever this file's owning manifest lives — a dependency-declaration
break in the `🔢️hash`/W1 area, **not touched by this batch, not naming any of my 9 crates or the
`flow_extension_sdk` file I edited**. Per the ticket's own rule ("If a check is blocked by errors
in `semio-framework-os-kernel` or `semio-framework-replication`, that is another agent's in-flight
work, NOT yours... record the exact error... and move on"), this is the same category of blocker,
just in `semio-framework-2d`/`⚙️engine` instead. `semio-s-plugin-flow-extension-logic` depends on
`flow_extension_sdk` (`semio-framework-os-flow`), which depends on `semio-framework-2d` directly —
so **every one of my 9 crates is transitively blocked by this one unrelated error** until someone
fixes that dependency declaration. Without `--keep-going`, `cargo check` stops at the first hard
error, so my own crate's compilation was never reached.

**Net effect on the verdict:** still **WRITTEN BUT UNVERIFIED** by a passing/failing check of any
of the 9 crates themselves or of `semio-framework-os-flow` — but now for a concrete, named,
unrelated reason rather than pure lock contention, and with positive evidence that a large swath
of the framework upstream of this batch (kernel, 3d, graph, schema, ui, compiler) compiles clean
with only pre-existing warnings. Re-run the same command list once `⚙️engine/🦀️component.rs`'s
missing `semio-framework-hash` dependency is fixed by whoever owns that area.
