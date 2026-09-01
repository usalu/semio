# TopicContribution seam — migrated to `DslValue`

## Headline

`ExtensionBundle::contributes_topic` and `TopicContribution.payload` are now
`semio_framework_os_kernel::DslValue`, not `serde_json::Value`. **`TopicContribution` itself is
serde-free at the type level** (its `payload` field no longer forces `serde_json`); the struct
still derives `Serialize`/`Deserialize` for the framework-internal host↔plugin `contributionsJson`
wire text, which is legitimate because `DslValue` already implements both (the pre-existing
`🌱️value/🔀️serde` bridge) — no new serde dependency introduced anywhere.

## Why `DslValue`, not `pack::json::Value`

The task offered a choice. `DslValue` won for three concrete reasons, not just consistency:

1. **The rest of the seam's own neighborhood had already committed to it.** Before I touched
   anything, `🏭️process`'s 4 extensions and `🪵️sourcing`'s 3 extensions were ALREADY calling
   `bundle.contributes_topic("...", semio_framework_os_kernel::DslValue::object([...]))` — written
   by a concurrent batch anticipating this exact migration. Those files could not have compiled
   under the old `serde_json::Value` signature. Picking `pack::json::Value` instead would have
   broken 7 already-written call sites instead of fixing them.
2. **`DslValue`'s API is a closer drop-in for the existing `serde_json::Value` call sites.**
   `DslValue` has `Index<&str>`/`Index<usize>`, `.as_str()`, `.as_object()`, `.get()` — the CAD
   extensions' existing test assertions (`payload["computersJson"].as_str()`) needed almost no
   reshaping. `pack::json::Value` has NO `Index` impl (only `.get()`), which would have forced
   every indexing call site to a different shape for no benefit.
3. **It is what `Mutation`/`MutationDiff`/`CompositeMutationKind` already migrated to.**
   `TopicContribution` is conceptually the same kind of thing — an open, schema-erased payload a
   plugin hands the host — so putting it on the same first-party value type as every other
   generic-payload seam in this ticket is the coherent long-term shape, not a one-off.

`pack::json::Value` remains the right type for STRING-shaped embedded JSON (`computersJson`,
`machinesJson`, `typologyJson`, `kindsJson` — fields whose value is JSON *text*, stored as a
`DslValue::String`), via `semio_framework_os_kernel::json::{to_json_string, from_json_str}`. Both
types are used, each for the shape it actually is — `DslValue` for the wire-tree, `pack::json` for
JSON text embedded as one string field inside that tree.

## Framework changes

| file | change |
|---|---|
| `🧰️framework/🔨️modules/🛂️manifest/🦀️component.rs` | `TopicContribution.payload: serde_json::Value` → `DslValue`; `TopicContribution::new` param type same change; `decode<T: serde::de::DeserializeOwned>(&self) -> Result<T, serde_json::Error>` → `decode<T: FromValue>(&self) -> Result<T, ValueError>` calling `T::from_value(self.payload.clone())`. `#[derive(Serialize, Deserialize)]` kept — legitimate, see above. |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs` | `ExtensionBundle::contributes_topic(topic, payload: Value)` → `payload: DslValue` (the local `Value` alias in that scope was `serde_json::Value`; `DslValue` was already in scope via the parent `app` module's `use dsl::{to_dsl_value, DslValue};`, no new import needed). |
| `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/📦️glue.rs` | **New**: `pub use pack::json;` added inside `os_pack` (alongside the existing `pub use pack::{async_, codec, format, http, io, source};`). This is what makes `semio_framework_os_kernel::json::{Value, to_json_string, from_json_str, dsl_to_json, json_to_dsl, parse, object, array, to_string}` reachable — **it did not exist before this session**. Several already-written call sites (`process`'s 4 extensions, `sourcing`'s 3 extensions) called `semio_framework_os_kernel::json::to_json_string(...)` as if this mount already existed; it did not, so those crates were silently broken until this edit. Not gated behind `#[cfg(not(wasm32))]` (unlike `pack::io`) — `json` has no OS dependency, confirmed against the standalone `semio-framework-pack` crate's own glue.rs, which mounts it unconditionally too. |

## Call-site inventory (construction + read sites, all fixed)

The task briefing quoted "26 call sites" from an earlier agent's grep of `contributes_topic(` alone.
The real blast radius is larger once **decode-side** call sites (`.decode::<T>()`, raw
`payload["key"]` reads) are counted — this section lists every one actually touched.

### Construction sites (`contributes_topic(...)` / `TopicContribution::new(...)`)

| site | before | after |
|---|---|---|
| `🧰️framework/…/🌊️flow/🧩️extensions/🕸️wasm/🦀️component.rs` `flow_extension_topic_contribution` (feeds all 9 `flow` extension crates) | `serde_json::json!({...})` | `DslValue::object([...])` |
| `✏️s/🔨️modules/📜️imperative/🧩️extension_sdk/🦀️component.rs` `imperative_module_topic_contribution` (feeds all 5 `imperative` extension crates) | `serde_json::json!({...})` | `DslValue::object([...])` |
| 4× `✏️s/🔌️plugins/📐️cad/🧩️extensions/{🏛️aec-building-structure,🏢️aec-building,📐️spatial-shape,🔥️aec-building-energy}/🦀️component.rs` | `serde_json::json!({...})` | `DslValue::object([...])` |
| `🏭️process`'s `🔩️metal`/`🪵️wood`/`🧱️concrete`/`🤖️robotic` extensions | already `DslValue::object([...])` but with a stray trailing `.into()` | `.into()` removed (harmless identity conversion, dead weight) |
| `🪵️sourcing`'s `🪵️beams`/`🪟️windows`/`🧱️slabs` extensions | already `DslValue::object([...])` but with a stray trailing `.into()` | same cleanup |
| `🧰️framework/…/📖️playbook/🦀️component.rs` (2 test fixtures) | `serde_json::json!({...})` | `dsl::DslValue::object([...])` |
| `🧰️framework/…/🖥️host/🦀️component.rs` (1 test fixture) | `serde_json::json!({...})` | `semio_framework::DslValue::object([...])` |
| `✏️s/🔌️plugins/🏭️process/…/✏️editor/🦀️component.rs` (3 test fixtures, incl. `WorkshopMachine` catalogs) | `serde_json::json!({...})` + `serde_json::to_string(&machines)` | `DslValue::object([...])` + `semio_framework_os_kernel::json::to_json_string(&machines)` |
| `✏️s/🔌️plugins/🪵️sourcing/…/🧬️schema/🦀️component.rs` (2 test fixtures) | `serde_json::json!({...})` | `DslValue::object([...])` + `json::to_json_string` |
| `✏️s/🔌️plugins/📋️forms/…/✏️editor/🦀️component.rs` (3 test fixtures, byte-identical duplicated block) | `serde_json::json!({...})` | `DslValue::object([...])` |
| `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🦀️component.rs` `module_extension_bundle` | `serde_json::json!({...})` | `DslValue::object([...])` |
| `✏️s/🔌️plugins/📖️playbook/…/🏗️builder/🦀️component.rs` (1 test fixture) | `serde_json::json!({...})` | `DslValue::object([...])` |

### Read sites (`.decode::<T>()`, needs `T: FromValue`)

Every payload-shape struct below moved `#[derive(Deserialize)]`/`#[serde(rename_all = "…")]` →
`#[derive(FromValue)]`/`#[value(rename_all = "…")]`. Two (`ProcessMachinesTopicPayload`,
`FormsQuestionKindTopicPayload`, `PlaybookBlockKindTopicPayload`) had an `IconName`-typed field —
`IconName` has no `ToValue`/`FromValue` impl anywhere in the repo, so each was changed to decode as
plain `String` and converted to `IconName` (`IconName: From<&str>`, already exists) at the call
site instead of inside the payload struct — avoids adding a speculative framework-wide `IconName`
value impl for a single-field need.

| struct | file |
|---|---|
| `FlowExtensionTopicPayload` | `🧰️framework/…/🌊️flow/📔️registry/🦀️component.rs` |
| `BlockKindPayload` | `🧰️framework/…/📖️playbook/🦀️component.rs` |
| `ImperativeModuleTopicPayload` | `✏️s/🔨️modules/📜️imperative/📇️registry/🦀️component.rs` |
| `ProcessMachinesTopicPayload` | `✏️s/🔌️plugins/🏭️process/…/✏️editor/🦀️component.rs` |
| `CadComputerTopicPayload` | `✏️s/🔌️plugins/📐️cad/…/💡️inferences/🦀️component.rs` |
| `SourcingModuleTopicPayload` | `✏️s/🔌️plugins/🪵️sourcing/…/🧬️schema/🦀️component.rs` |
| `FormsQuestionKindTopicPayload` | `✏️s/🔌️plugins/📋️forms/…/✏️editor/🦀️component.rs` |
| `PlaybookBlockKindTopicPayload` | `✏️s/🔌️plugins/📖️playbook/…/🏗️builder/🦀️component.rs` |

One raw (non-`.decode`) read site had a real bug I fixed while migrating it:
`✏️s/🔌️plugins/🏭️process/🧩️extensions/🪵️wood/🦀️component.rs`'s test did
`topic_contribution.payload.as_object().expect(...)` then indexed the result (`payload["appId"]`)
— `.as_object()` on `DslValue` returns `&[(String, DslValue)]`, a slice, which has no string-index
operator. This would not have compiled. Fixed by indexing `&topic_contribution.payload` directly
(`DslValue`'s own `Index<&str>`) instead of going through `.as_object()` first. Sibling `sourcing`
extension tests had a related bug — `assert_eq!(topic.payload["appId"], HOST_APP_ID)` compares
`&DslValue` against `&str`, and `DslValue` has no `PartialEq<&str>` impl — fixed to
`.as_str()`/`Some(...)` comparisons in `🪵️beams`/`🪟️windows`/`🧱️slabs`.

### New `semio-framework-value-derive` dependency edges added (all resolve-checked with `ls -d`)

`🧰️framework/…/🌊️flow/📦️packages/🦀️rust/Cargo.toml` · `✏️s/🔨️modules/📜️imperative/📦️packages/🦀️rust/Cargo.toml`
· `✏️s/🔌️plugins/📐️cad/📦️packages/🦀️rust/Cargo.toml` · `✏️s/🔌️plugins/📋️forms/📦️packages/🦀️rust/Cargo.toml`
· `✏️s/🔌️plugins/📖️playbook/📦️packages/🦀️rust/Cargo.toml` (`🪵️sourcing`/`🏭️process`/cad-extensions already had it).

## Per-plugin manifest state — the 4 CAD extensions this seam unblocked

| extension | before | after |
|---|---|---|
| `📐️cad/🧩️extensions/🏛️aec-building-structure` | `serde_json = { workspace = true }` | **removed** — `[dependencies]` now has zero third-party entries |
| `📐️cad/🧩️extensions/🏢️aec-building` | `serde_json = { workspace = true }` | **removed** |
| `📐️cad/🧩️extensions/📐️spatial-shape` | `serde_json = { workspace = true }` | **removed** |
| `📐️cad/🧩️extensions/🔥️aec-building-energy` | `serde_json = { workspace = true }` | **removed** |

`serde` was already gone from all four before this session (prior batch). All four now have zero
third-party runtime dependencies — PROVEN BY GREP (`grep -E "^serde|^\[dependencies\]"` on each
manifest, pasted below), not yet by a passing `cargo check` of the extension crate itself (see
Verification section — blocked by unrelated concurrent breakage, not by anything in these files).

```
=== 🏛️aec-building-structure ===        === 🏢️aec-building ===        === 📐️spatial-shape ===        === 🔥️aec-building-energy ===
[dependencies]                          [dependencies]                [dependencies]                 [dependencies]
```

(each manifest's `[dependencies]` table now has zero third-party lines — `pack`/`semio-framework-*` path deps only)

## Verification

### PROVEN BY A PASSING CHECK

**`cargo check -p semio-framework-os-kernel --message-format=short`** — exit 0, 4m33s, warnings
only (33 pre-existing, none new). This is real: it compiles the `pub use pack::json;` glue.rs edit,
`🧰️framework/…/📖️playbook/🦀️component.rs`'s `BlockKindPayload` `#[derive(FromValue)]` conversion
and its two `DslValue::object(...)` test-fixture rewrites (all mounted inside this crate), and the
whole `DslValue`/`ToValue`/`FromValue`/`json` machinery the rest of the seam depends on.

**Standalone oracle crate**, built outside the repo (`/private/tmp/…/topic-contribution-verify`,
test source copied to this ticket's `🔬️verification-topic-contribution/oracle.rs`): a verbatim copy
of `🌱️value/{🦀️component.rs,🔁️codec/🦀️component.rs,🔀️serde/🦀️component.rs}` and `🎒️pack/🔤️json/🦀️component.rs`
(only edit: `protocol::value::` → `crate::value::`), plus 5 new tests exercising exactly the
patterns used at the real call sites, with `serde_json` as a `[dependencies]` oracle (not
`[dev-dependencies]` — needed by the copied library code itself, matching the real `🌱️value`
crate's own `Cargo.toml`):

```
running 5 tests
test from_value_decodes_string_fields_the_way_the_generated_derive_does ... ok
test dsl_value_indexing_and_as_str_match_the_real_call_sites ... ok
test pack_json_value_get_matches_serde_json_oracle_shape ... ok
test pack_json_bridge_round_trips_against_serde_json_oracle ... ok
test topic_contribution_round_trips_through_serde_json_wire_path ... ok

test result: ok. 5 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s
```

These 5 tests specifically prove: (1) a `TopicContribution`-shaped struct with `payload: DslValue`
round-trips through `serde_json::to_string`/`from_str` byte-for-byte (the `contributionsJson` host
wire path is intact), and the resulting JSON text parses under `serde_json::Value` too — i.e. it is
genuine, spec-shaped JSON, not some private encoding; (2) `DslValue`'s `Index<&str>`/`.as_str()`
match every real test assertion's usage; (3) hand-written `ToValue`/`FromValue` in the derive's
exact generated shape decode correctly; (4) `pack::json::{to_json_string, from_json_str}` produce
byte-identical JSON to `serde_json::to_string` for the same value and decode correctly in both
directions; (5) `pack::json::Value::get` (the `Index`-less replacement for `serde_json::Value`'s
`[...]`) matches the oracle.

Additionally, copying the three `🌱️value` files and `🎒️pack/🔤️json` verbatim also re-ran their
OWN pre-existing test suites (not authored by me) — **33 of 34 passed**:

```
test result: FAILED. 33 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

---- json::tests::differential_parse_matches_serde_json_on_arbitrary_values stdout ----
thread panicked at src/json_component.rs:1214:13:
case 37: structural mismatch; text=8322951083873004.0
mine=Number(Float(8322951083873004.0))
theirs=Number(8322951083873005.0)
```

**This 1 failure is real, pre-existing, deterministic (fixed RNG seed, reproduces every run), and
unrelated to `TopicContribution`** — a 1-ULP float-parsing disagreement between `pack::json`'s
hand-written number parser and `serde_json` near the 2^53 mantissa-precision boundary, in code that
existed before this session and that I did not author or touch. Flagged as a follow-up task
(`task_6de8b2c8`, spawned this session) rather than fixed here — out of this seam's scope, and deep
numerical-parser work deserves its own focused pass. It does NOT affect any of my own 5 tests (none
of my payloads carry large-magnitude floats) and does not affect any real `TopicContribution`
payload field in the repo today (`appId`/`moduleId`/`label`/`iconId`/`…Json` are all strings).

### WRITTEN BUT UNVERIFIED — and why

**No S-plugin crate's own `cargo check` completed green this session**, including the 4 CAD
extensions this seam exists to unblock. Not because of anything in this seam's own changes — two
unrelated, concurrent, in-flight edits by other live sessions currently leave large parts of the
dependency graph red:

1. **`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/`** — 26 files with an uncommitted diff as
   of this session (confirmed via `stat`/`git diff --stat`, last modified minutes before I checked),
   80 compile errors, all `expected &_, found <owned type>` mismatches in workflow's own mutation
   dispatch code. `cargo check -p semio-framework` fails with exactly these 80 errors and zero
   others — none reference `manifest`, `plugin`, `TopicContribution`, `contributes_topic`, or
   `DslValue` (checked by grep across the full error listing).
2. **`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs`'s `ArtifactApp::Snapshot`/
   `Mutation` bound migration** — the SAME file `contributes_topic` lives in, being migrated off
   `serde::Serialize`/`DeserializeOwned` by another concurrent session per this ticket's own
   briefing ("you may collide there... never revert it"). `cargo check -p
   semio-s-plugin-cad-spatial-shape` surfaces 864 errors, every one an `ArtifactApp::Snapshot`/
   `Mutation: Serialize`/`Deserialize` bound failure — zero reference `TopicContribution`,
   `contributes_topic`'s signature, or `DslValue`. Already flagged as its own follow-up task
   (`task_3b2e08fe`, pre-existing in the queue, not spawned by me).

Since `semio-framework` (the crate `TopicContribution`/`ProgramContributionEntry` live in) is
transitively depended on by every s-plugin, and it currently cannot compile for reasons unrelated
to this seam, **no full-crate check of any consumer is possible right now**. This is an honest
environmental limitation, not a gap in my own verification effort — the os-kernel green build plus
the standalone oracle crate are the strongest proof obtainable without either (a) fixing two other
sessions' in-flight work myself (explicitly against this ticket's instructions) or (b) waiting for
them to land.

## Explicitly left undone

1. **`➗️mathematical`** — untouched this session. Per the prior batch's own notes, still needs: the
   `MathematicalDiff`/`MathematicalMutation` dispatch enum, the two JSON test-bridge functions
   (`mathematical_mutation_report_json`/`mathematical_identity_report_json`), the content-hash
   function (`mathematical_scene_id`), the io/editor/presence/config files (~20 files), then
   deleting `serde`/`serde_json` from `Cargo.toml` once every file compiles.
2. **`🔋️energy`** — untouched this session, exactly as handed off ("not started", ~60 files
   surveyed at the byte level only).
3. **The `dsl_value_serde`/`to_dsl_value`/`from_dsl_value` bridge module** (`🌱️value/🔀️serde`) is
   still alive and load-bearing — NOT dead code, contrary to the playbook doc's aspiration that it
   become deletable once every composed-artifact plugin converts. It is what makes
   `#[derive(Serialize, Deserialize)]` + `payload: DslValue` work on `TopicContribution` at all.
   Deleting it would require giving `DslValue` up entirely, which is not this ticket's direction.
4. Two bugs found and NOT fixed here, each spawned as a separate follow-up task rather than
   patched inline (out of this seam's scope): `task_6de8b2c8` (pack::json float-parser 1-ULP
   defect, this session) and the pre-existing `task_3b2e08fe` (ArtifactApp::Snapshot bound,
   blocking every plugin's own `cargo check` right now, not spawned by me).
