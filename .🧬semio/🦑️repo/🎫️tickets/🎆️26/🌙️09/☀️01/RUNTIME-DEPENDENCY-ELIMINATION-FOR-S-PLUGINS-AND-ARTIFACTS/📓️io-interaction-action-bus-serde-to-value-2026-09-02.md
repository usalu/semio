# io / interaction / action-bus — serde → ToValue/FromValue conversion (2026-09-02)

Scope: my 3 assigned modules only, all under `🧰️framework/🔨️modules/`:
`🚪️io` (+ `🚪️io/🧬️schema`, `🚪️io/🔤️base64`), `🕹️interaction` (+ `🧬️schema`), `🎯️action-bus`
(+ `🧹️wire-retirement`). Did not touch `🛂️manifest` or any file under `✏️s/🔌️plugins/`.

## Baseline

`cargo check -p semio-framework` at session start: 0 errors (only warnings), confirmed before any
edit. Assigned count: 48 production serde refs (io 38, interaction 7, action-bus 3).

## What changed (serde removed, ToValue/FromValue only)

**`🚪️io/🦀️.rs`** (the file with by far the most refs — now 0 real production `serde` code, only
3 doc-comment mentions):
- `IoDirection`, `IoKey` (old registry) — dropped serde, `ToValue`/`FromValue` only.
- `Confidence` (old 3-variant), `ComposeError`, `IoPayload` (old 2-variant), `IoFidelityClass`,
  `IoFidelityDeclaration`, `WireComposeSource`, `WireComposedArtifact`, `FormatDescriptor` — same.
- `encode_wire_json<T: Serialize>` → `<T: ToValue>`, body `serde_json::to_vec` →
  `dsl::os_pack::json::to_json_string(value).into_bytes()`. Added a symmetric `decode_wire_json<T:
  FromValue>` helper (utf8-validate then `dsl::os_pack::json::from_json_str`) and repointed all 3
  decode call sites (`wire_decode_composed_artifact`, `wire_artifact_compose`'s key + sources
  decode) at it — these are the real WIT-component-boundary JSON bytes, not test code.
- Test fixture at `wire_rejects_oversized_and_unbounded_dialect_inputs_before_interning`: swapped
  `serde_json::to_vec` for the same `pack::json` call.
- `mod laws`'s `JsonDeserializer`/`flag_non_object`/`conformance_runs_after_deserialize` (the block
  the ticket brief explicitly named, ~line 2879-2910): `serde_json::Value` → `dsl::DslValue` (which
  already has `impl store::ArtifactPack for DslValue` in `🏪️store/🦀️.rs`, so it's a drop-in for
  the `S: store::ArtifactPack` bound), `serde_json::from_str` → `dsl::os_pack::json::from_json_str`,
  `value.is_object()` → `matches!(value, DslValue::Object(_))`.
- Removed the now-unused top-level `use serde::{Deserialize, Serialize};`.

**`🚪️io/🧬️schema/🦀️.rs`**:
- `StandardId`/`SubsetId`/`Dialect` — serde derive dropped with NO replacement (nothing anywhere
  in the repo (de)serializes these `'static`-only compile-time types).
- `ArtifactRef`, `ArtifactKindId` — dropped serde, `ToValue`/`FromValue` only (verified zero
  `serde_json` usage anywhere, and the crates that embed them — `🏪️store`, `🌿️vcs` — already
  consume them via `FromValue`/`ToValue`, not serde).
- `Confidence` (new 4-variant), `IoFidelity`, `IoError` (added `ToValue`/`FromValue`, it had
  neither before), `IoEntryDescriptor`, `IoRoute` — dropped serde half of the existing dual derive.

**`🎯️action-bus/🦀️.rs`** + **`🧹️wire-retirement/🦀️.rs`**: the 3 raw JSON-fixture-parsing test
functions (`serde_json::Value` + `["key"]` indexing + `.as_u64()/.as_bool()/.as_array()`) now use
`dsl::os_pack::json::parse(...)` — `pack::json::Value` has the same `Index<&str>`/`Index<usize>`/
`as_u64`/`as_bool`/`as_array` API, byte-for-byte drop-in.

**`🚪️io/🔤️base64`**: no change needed. Its own crate (`semio-framework-io-base64`) already carries
`serde_json` ONLY in `[dev-dependencies]` — the one remaining `serde_json::from_str` is inside
`#[cfg(test)] mod tests`, parsing a fixture; this crate ships zero runtime serde already. Left as-is.

## Blocked (documented in-source with `🚧️ BLOCKED` comments, NOT converted)

Every one of these is a genuine cross-crate compile requirement from a crate outside my scope —
confirmed by actually trying the conversion and reading the resulting `E0277`s, not by inspection
alone.

1. **`🕹️interaction/🦀️.rs`**: `InteractionDefinition`, `GranularityDefinition`, `InteractionRef`
   (all 3, i.e. all 7 of interaction's original refs). `🛂️manifest/🦀️.rs`'s `AppDefinition.
   interactions: Vec<InteractionDefinition>` and `WindowKindDefinition.interactions:
   Vec<InteractionRef>` derive plain unconditional `Serialize, Deserialize` — manifest is "another
   agent owns 🛂️manifest, do NOT touch" per the brief. Tried the conversion, got `E0277` on
   `LocalizedLabel: ToValue` cascading into `AppRole`/`TopicContribution`/`NonEmptyVec<...>`
   E0277s inside manifest.rs, reverted cleanly (functionally identical to HEAD, only added
   explanatory comments — diff is comments-only, confirmed via `git diff`).

2. **`🚪️io/🧬️schema/🦀️.rs`**: `ArtifactDialect`. `🛂️manifest/🦀️.rs`'s own `IoEntryDescriptor`
   (owner/counterpart), `ComposerEntryDescriptor` (writes/reads), and `AppDefinition.dialect` all
   embed `ArtifactDialect` under plain unconditional `Serialize, Deserialize`. Kept the existing
   (pre-existing, not introduced by me) dual `Serialize, Deserialize, ToValue, FromValue` derive —
   both families are genuinely load-bearing simultaneously right now, not a lazy shortcut.

3. **`🚪️io/🧬️schema/🦀️.rs`**: `IoPayload` (new 2-variant). `semio-framework-plugin`'s
   `🔌️plugin/🖥️host/🦀️.rs` test module (a SEPARATE crate — own `serde`/`serde_json` prod deps,
   actively mid-conversion by another agent this same ticket per its own Cargo.toml comments) still
   calls `serde_json::to_vec(&IoPayload::…)`/`serde_json::from_slice::<IoPayload>` directly at 5
   call sites (~8306-8414). `#[cfg_attr(test, derive(...))]` cannot fix this: that crate compiles
   `semio-framework` as an ordinary (non-`cfg(test)`) dependency even for ITS OWN test target, so
   the cfg never activates cross-crate. Production call sites in the SAME file (`🖥️host/🦀️.rs`
   lines ~4124, 4140, 5638) already use `dsl::os_pack::json::from_json_str` — only the 5 test call
   sites are stale. Did not touch that file (out of my 3 modules); left `IoPayload` dual-derived.

4. **`🎯️action-bus/🦀️.rs`**: `optional_json_to_dsl(args: Option<serde_json::Value>) -> Option
   <DslValue>` (line ~803). Per the brief's own explicit instruction for this exact function:
   listing callers instead of changing the signature. Found ~26 real callers (via
   `grep -rln optional_json_to_dsl`), 24 of them under `✏️s/🔌️plugins/**` (every one an
   `Option<serde_json::Value>`/`Option<Value>` argument via `use serde_json::Value;`), plus
   `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs` and `.../♾️infinite/🌍️world/🦀️.rs`.
   A signature change here is a genuine one-atomic-wave cross-crate edit touching ~26 files I do
   not own — left unchanged. Full caller list is reproducible with:
   `grep -rln optional_json_to_dsl --include='*.rs' ✏️s 🧰️framework/🛍️products`.

## Ref counts

Counting `grep -cE 'serde|Serialize|Deserialize'` after stripping `//` line comments (git HEAD vs
working tree), across all 8 files in my 3 modules:

| file | before | after |
|---|---:|---:|
| `🎯️action-bus/🦀️.rs` | 2 | 1 (blocked: `optional_json_to_dsl`) |
| `🎯️action-bus/🧹️wire-retirement/🦀️.rs` | 2 | 0 |
| `🕹️interaction/🦀️.rs` | 9 | 9 (blocked: manifest, unchanged) |
| `🕹️interaction/🧬️schema/🦀️.rs` | 0 | 0 |
| `🚪️io/🔤️base64/…/🦀️.rs` (packages) | 0 | 0 |
| `🚪️io/🔤️base64/🦀️.rs` | 1 | 1 (sanctioned test-only, already dev-dep-scoped) |
| `🚪️io/🦀️.rs` | 41 | 14 (all 14 are doc-comment prose; 0 real code) |
| `🚪️io/🧬️schema/🦀️.rs` | 17 | 4 (blocked: `ArtifactDialect` ×2 lines, `IoPayload` derive, `use serde` import) |
| **total** | **72** | **29** |

Real remaining *code* (not comments, not the sanctioned base64 test): `action-bus` 1 line +
`io_schema` 4 lines (1 import + `ArtifactDialect`'s derive+attr + `IoPayload`'s derive) = 5 lines,
all documented cross-crate blockers. `interaction`'s 9 (unchanged from HEAD) and base64's 1 are
pre-existing/sanctioned, not new debt.

## Errors — before/after, attributable vs peer churn

- Start-of-session baseline (`cargo check -p semio-framework --message-format short`): **0
  errors**, confirmed BEFORE any edit of mine.
- Every recheck during the session (after each file's edits) showed errors ONLY in
  `🛂️manifest/🦀️.rs`, its adjacent `../🎠️kernel/🦀️.rs`, and (once) `🔁️workflow/🦀️.rs` — all
  citing manifest-owned types (`AppRole`, `TopicContribution`, `NonEmptyVec<...>`, `MediaType`,
  `MediaWireFormat`, `MediaForm`, `PortMultiplicity`, `MediaPortSpec`) losing `Serialize`/
  `Deserialize`. Error count fluctuated 27 → 30 → 229 → 130 across the session with zero action
  from me in between rechecks — this is a concurrent session actively converting `🛂️manifest`
  (confirmed via `git log` showing a commit to `🔌️plugin/🖥️host/🦀️.rs` at 13:31:56 today, and the
  manifest error set changing shape between my checks). **Every recheck found 0 errors outside that
  manifest/kernel/workflow blast radius** — i.e. 0 errors attributable to my edits, at every point.
- Final state at report time: 130 total errors, all in `🛂️manifest`/`🎠️kernel`/`🔁️workflow`, 0 in
  `io`/`interaction`/`action-bus` or anywhere else. Did not attempt to fix manifest (out of scope,
  another agent's file, explicit instruction not to touch it).

## Files touched

- `🧰️framework/🔨️modules/🚪️io/🦀️.rs`
- `🧰️framework/🔨️modules/🚪️io/🧬️schema/🦀️.rs`
- `🧰️framework/🔨️modules/🕹️interaction/🦀️.rs` (comments only — reverted the functional change)
- `🧰️framework/🔨️modules/🎯️action-bus/🦀️.rs`
- `🧰️framework/🔨️modules/🎯️action-bus/🧹️wire-retirement/🦀️.rs`

No Cargo.toml edited (as instructed — `semio-framework`'s manifest lines stay until io_schema's
`ArtifactDialect`/`IoPayload` and interaction's 3 types are unblocked by `🛂️manifest` and
`semio-framework-plugin` migrating).
