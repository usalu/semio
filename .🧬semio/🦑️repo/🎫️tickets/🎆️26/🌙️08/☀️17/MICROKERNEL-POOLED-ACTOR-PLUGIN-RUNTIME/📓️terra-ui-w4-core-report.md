# 📓️ terra — packet `ui-w4-core` report

Executor `terra`, packet `ui-w4-core`. Path scope: `🖱️ui/🧬️contract/**` and `🖱️ui/🧠️runtime/**`. Ran
every cargo command myself, foreground, `CARGO_TARGET_DIR=/private/tmp/claude-501/-Users-ueli-Documents-semio/40ab938a-57cf-4d17-94a3-77c54a12536e/scratchpad/target-ui`, per this packet's own BUILD SETUP
instruction (which supersedes the historical U4 "executors don't run cargo" convention recorded in
`UITICKET/📓️status.md` — that ruling predates this packet's explicit brief).

## Item 1 — field-targeted `UiPatchOp` setters: ALREADY LANDED, not a scaffold

**Which is true, definitively determined by reading the actual source, not a report:** the op enum
already has all 11 variants (`Upsert | SetComponent | SetLayout | SetActivity | SetChildren | SetStyle |
SetAccessibility | SetBindings | SetMenu | Remove | SetRoot`) in
`🧬️contract/📦️packages/🦀️rust/🦀️document.rs`, `apply_patch`/`op_text_bytes` in `🦀️limits.rs` handle all
eight mutate-based ops exhaustively, and `SurfaceReconciler::diff_existing` in
`🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs` diffs all eight field groups independently and emits the
single narrowest op when exactly one group changed (no byte comparison — deterministic), falling back to
a byte-compared `Upsert` only when **more than one** group changed at once. Both files are already
committed at HEAD (`git log` shows commit `6cf8d6c858`, `git diff --stat HEAD` is empty for both) — this
is packet `contract-patch-ops`'/`runtime-reconcile`'s completed work, done before this packet started. The
ticket brief's premise ("their status says style/accessibility/bindings/menu fall back to whole-node
Upsert") describes packet `runtime-reconcile`'s OWN report from when it landed the scaffold, dated before
`contract-patch-ops` closed that exact gap — a stale status, not the current code.

Tests already present and passing prove the property: `reconcile.rs`'s
`changing_only_style_emits_exactly_one_set_style_not_upsert` /
`changing_only_accessibility_emits_exactly_one_set_accessibility_not_upsert` /
`changing_only_bindings_emits_exactly_one_set_bindings_not_upsert` /
`changing_only_menu_emits_exactly_one_set_menu_not_upsert`, plus
`changing_several_groups_at_once_prefers_a_single_upsert_over_many_targeted_ops` for the multi-group
fallback and a full round-trip property test. **No code change needed or made for item 1** — verified by
running the suite (see Acceptance below), not assumed.

## Item 2 — `ImageBuilder` typestate: IMPLEMENTED

`🧬️contract/📦️packages/🦀️rust/🦀️builder.rs`, region `🖼️Image`. Replaced the runtime
`panic!("image(\"{src}\") needs .alt(..) or .decorative()...")` with a phantom-typed typestate:

```rust
pub struct NoAlt;
pub struct HasAlt;
pub struct ImageBuilder<State = NoAlt> { base: NodeBase, src: String, alt: Option<ImageAlt>, _state: PhantomData<State> }

pub fn image(src: impl Into<String>) -> ImageBuilder<NoAlt>;
impl<State> ImageBuilder<State> {
    pub fn alt(self, alt: impl Into<Label>) -> ImageBuilder<HasAlt>;
    pub fn decorative(self) -> ImageBuilder<HasAlt>;
}
impl<State> HasBase for ImageBuilder<State> { .. }   // chainable methods work in either state
impl From<ImageBuilder<HasAlt>> for BuiltNode { .. } // the ONLY From impl — NoAlt has none
```

`ImageBuilder<NoAlt>` does not implement `Into<BuiltNode>`, so the blanket `impl<T: Into<BuiltNode>>
Buildable for T` never applies to it — `.build()` is **absent from its method set**, not present-and-
panicking. `ImageBuilder<HasAlt>` gets `.build()` for free from that same blanket impl once `.alt(..)`/
`.decorative()` transitions it. `HasBase` (the id/style/tone/... chainable vocabulary) is implemented
generically over `ImageBuilder<State>` so it works identically before or after the alt decision.

**Negative case, proven by the compiler itself, not asserted in prose**: a `compile_fail` rustdoc test on
`ImageBuilder`'s own doc comment (built into rustdoc, no `trybuild`/extra dev-dependency needed — the
`Cargo.toml` registrar file was not touched):

```rust
/// ```compile_fail
/// use semio_framework_ui_contract::*;
/// let _ = image("atlas://logo").build();  // E0599: no method named `build`
/// ```
```

Ran as part of `cargo test -p semio-framework-ui-contract` (doc-tests section) — **passed** (see below),
i.e. rustc/rustdoc confirmed this snippet genuinely fails to compile. A second, ordinary doctest on the
same doc comment shows the fix (`.alt(..).build()` / `.decorative().build()`) compiling and running
clean. The old runtime `#[should_panic]` test was removed (nothing left to call — there is no `.build()`
on `ImageBuilder<NoAlt>` to invoke) and replaced with a one-line test asserting `image(..)`'s return type
is `ImageBuilder<NoAlt>`, with a comment pointing at the compile-fail doctest as the actual negative case.

## Item 3 — `PresenceHub`: ALREADY READY, verified standalone, API documented below

`🧠️runtime/📦️packages/🦀️rust/🦀️presence.rs` is already a complete, committed (HEAD, no working-tree
diff), standalone implementation — not a scaffold. Confirmed by reading the file and its imports: it
depends on nothing but `std::collections::{BTreeMap, BTreeSet, HashMap}` and
`ui_contract::{OwnPresence, PeerMark, PresenceUpdate, SurfaceId}` — **no `EntityStore`, no `UiRuntime`**,
exactly the standalone requirement the reactor packet needs. `record_own`/`record_peer` take
`(surface, node_key, ..)` and coalesce same-key writes to the LAST value between two `flush()` calls;
`expire(now_ms)` drops peer marks whose deadline is `<= now_ms` (own presence is never touched by
`expire` — it has no TTL) and marks the shrunk key dirty; `flush()` drains dirty keys into one
`PresenceUpdate` each and garbage-collects any entry that went fully empty. Already exported at crate
root via `📦️glue.rs`'s `pub use presence::*;`.

**Tests item 3 asked for already exist and pass**: `presence_entries_expire_exactly_at_their_ttl_and_a_flush_after_expiry_omits_them`
(TTL expiry with no goodbye message — a peer that never sends one still ages out at the exact TTL
boundary) and `a_burst_of_same_key_peer_writes_coalesces_to_one_update` /
`a_burst_of_same_key_own_presence_writes_coalesces_to_the_newest_value` (burst coalescing to ONE
update). No code change needed or made for item 3.

**Exact API the reactor sibling should call** (forwarding verbatim per your request):

```rust
use semio_framework_ui_runtime::PresenceHub;
use semio_framework_ui_contract::{OwnPresence, PeerMark, PresenceUpdate, SurfaceId};

// one PresenceHub per actor, thread-local, alongside the patch tracker:
thread_local! { static PRESENCE: std::cell::RefCell<PresenceHub> = std::cell::RefCell::new(PresenceHub::new()); }

impl PresenceHub {
    pub fn new() -> Self;
    pub fn record_own(&mut self, surface: SurfaceId, node_key: impl Into<String>, own: OwnPresence, ttl_ms: u32);
    pub fn record_peer(&mut self, surface: SurfaceId, node_key: impl Into<String>, mark: PeerMark, ttl_ms: u32, now_ms: u64);
    pub fn expire(&mut self, now_ms: u64);
    pub fn flush(&mut self) -> Vec<PresenceUpdate>;  // pack-encode each element onto the WIT turn-result
}
```

Call `record_own`/`record_peer` after each render this turn produced one; call `expire(now)` then
`flush()` exactly once per turn, after rendering — `flush()`'s `Vec<PresenceUpdate>` is what gets
pack-encoded onto the WIT turn-result. No other accessor was needed; nothing blocked this item.

## Item 4 — `SurfaceProps` scaffold: REPLACED wholesale

`🧬️contract/📦️packages/🦀️rust/🦀️surface.rs` replaced entirely (was genuinely a scaffold — its own header
said so). Final shape, exactly as decided:

```rust
pub struct SurfaceProps {
    pub kind: SurfaceKind,
    pub doc_schema: String,            // "<kind>@<version>", e.g. "world3d@1"
    pub doc: SurfaceDoc,               // opaque pack-encoded bytes; contract NEVER parses them
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bindings: Vec<crate::ActionBinding>,
}
pub struct SurfaceDoc { pub bytes: Vec<u8> }
```

Dropped from the scaffold: `surface_id`, `controller_id`, `pane_id`, `binding_id`, `domain_id`,
`domain_granularity_id`. Checked before dropping them: `semio-framework-ui-render`'s own
`🦀️surface.rs` (a sibling crate, not my scope) already imports only `SurfaceId`/`SurfaceKind` from this
crate, never those six fields, and owns its own independent `SurfacePlacement`/`AnySurface` placement
model — so nothing downstream I can see references the dropped fields. `builder.rs`'s `SurfaceBuilder`
wraps a caller-built `SurfaceProps` verbatim (never touches individual fields), so it needed no change.

**Rename**: `SurfaceKind::VirtualFileSystem`'s wire name fixed from the inconsistent camelCase
`"virtualFileSystem"` to kebab-case `"virtual-file-system"`, matching every sibling variant. No back-compat
shim (greenfield, no users — root `CLAUDE.md`).

**Unknown-`doc_schema` rule, implemented and tested**: this crate never validates `doc_schema` against a
known set (it structurally cannot own that set — every product crate that embeds a surface adds its own
kinds), so an unrecognised or malformed `doc_schema` never rejects a patch or fails validation — proved by
`validate_snapshot_never_rejects_an_unrecognised_doc_schema` and
`apply_patch_accepts_a_set_component_carrying_an_unrecognised_doc_schema`. Added
`parse_doc_schema(&str) -> Result<SurfaceSchema<'_>, SurfaceSchemaFault>` as the shared, tested building
block a renderer calls to implement "unknown schema → placeholder + logged fault, never a panic" — it
never panics on any input (empty, no `@`, empty kind half, non-numeric version), proved by
`parse_doc_schema_never_panics_and_returns_a_typed_fault_for_every_malformed_shape` exercising seven
malformed shapes. The actual placeholder-rendering + logging is `semio-framework-ui-render`'s job (out of
this packet's scope) — this crate only guarantees nothing here stops it from doing that safely.

**Opaque-blob diff, implemented and tested**: `SurfaceProps` derives plain structural `PartialEq`
(nothing schema-aware), so a one-byte change anywhere in `doc.bytes` makes the whole `SurfaceProps`
compare unequal — proved directly by `differing_only_in_one_doc_byte_makes_surface_props_unequal`. Since
`Component::Surface(SurfaceProps)` is diffed by the reconciler as part of the same `component` field
group every other `Component` variant uses (see item 1), a changed scene is naturally exactly one
`SetComponent`/`Upsert` op — no separate `SetSurface` op was needed or added.

**Exact signature the `🎬️scene` crate should build its typed `encode`/`decode` helpers against**
(forwarding verbatim per your request — that crate depends on this one, never the reverse):

```rust
fn encode<T: serde::Serialize>(kind: ui_contract::SurfaceKind, version: u32, value: &T) -> ui_contract::SurfaceProps;
fn decode<T: serde::de::DeserializeOwned>(props: &ui_contract::SurfaceProps) -> Result<T, DecodeFault>;
```

`encode` is expected to set `doc_schema` to a string of the form `format!("{kind_slug}@{version}")` — the
exact shape `ui_contract::parse_doc_schema` splits back apart. `decode` is expected to call
`parse_doc_schema` first and treat a `SurfaceSchemaFault` (or a recognised-but-unimplemented kind/version
pair) as its own `DecodeFault::UnknownSchema`-shaped case, never a panic. The scene crate owns
`DecodeFault`, the per-kind `kind_slug` strings, and the actual pack encode/decode of `T`; this crate
(`ui_contract::surface`) defines only the opaque envelope (`SurfaceProps`/`SurfaceDoc`) and the
schema-string convention (`SurfaceSchema`/`parse_doc_schema`/`SurfaceSchemaFault`), never a decoder.

## ts-rs regeneration needed

`🛂️manifest/🤖️generated/🟦️ui-contract.ts` needs regenerating (not hand-edited — gitignored, sol's own
generator). Changed wire types:

- **`SurfaceKind`** — one variant's rename (`virtualFileSystem` → `virtual-file-system`).
- **`SurfaceProps`** — shape changed: `surface_id`/`controller_id`/`pane_id`/`binding_id`/`domain_id`/
  `domain_granularity_id` removed; `bindings: ActionBinding[]` added.

`ImageBuilder`/`NoAlt`/`HasAlt` are Rust-only builder types, never `#[derive(TS)]`'d and not in
`tests/typegen_export.rs`'s export list — no TS impact from item 2. `PresenceHub` (item 3) is likewise a
Rust-only runtime type in `semio-framework-ui-runtime`, never wire-serialized itself (only the
`PresenceUpdate` it produces is, and that type is unchanged).

## Acceptance — every command run myself, foreground, output pasted

```
$ CARGO_TARGET_DIR=.../scratchpad/target-ui cargo check -p semio-framework-ui-contract --lib
    Finished `dev` profile [unoptimized] target(s) in 0.97s          EXIT=0, 0 warnings
$ CARGO_TARGET_DIR=.../scratchpad/target-ui cargo check -p semio-framework-ui-contract --all-targets
    Finished `dev` profile [unoptimized] target(s) in 2.42s          EXIT=0
$ CARGO_TARGET_DIR=.../scratchpad/target-ui cargo test -p semio-framework-ui-contract
running 85 tests ... test result: ok. 85 passed; 0 failed; 0 ignored
running 0 tests (tests/typegen_export.rs, feature-gated off)
Doc-tests: running 2 tests ... test result: ok. 2 passed; 0 failed        EXIT=0
$ CARGO_TARGET_DIR=.../scratchpad/target-ui cargo check -p semio-framework-ui-runtime --lib
    Finished in 0.92s                                                 EXIT=0
$ CARGO_TARGET_DIR=.../scratchpad/target-ui cargo check -p semio-framework-ui-runtime --all-targets
    Finished in 0.74s                                                 EXIT=0
$ CARGO_TARGET_DIR=.../scratchpad/target-ui cargo test -p semio-framework-ui-runtime
running 62 tests ... test result: ok. 62 passed; 0 failed; 0 ignored
Doc-tests: running 0 tests                                             EXIT=0
```

**R14 cross-target evidence** (both crates are consumed by wasm guests; I changed public types, so a
native-only check is not sufficient evidence):
```
cargo check -p semio-framework-ui-contract --lib --target wasm32-unknown-unknown   EXIT=0
cargo check -p semio-framework-ui-contract --lib --target wasm32-wasip2            EXIT=0
cargo check -p semio-framework-ui-runtime  --lib --target wasm32-wasip2            EXIT=0
```

**Test counts, named deltas** (contract crate — the only crate whose files I edited):
- Unit tests (`--lib`): **79 → 85** (+6, all in `surface.rs`: `surface_kind_wire_names_are_all_kebab_case`,
  `surface_props_round_trips_with_bindings_and_non_empty_doc`,
  `surface_props_omits_empty_bindings_on_the_wire`,
  `differing_only_in_one_doc_byte_makes_surface_props_unequal`, `parse_doc_schema_splits_kind_and_version`,
  `parse_doc_schema_never_panics_and_returns_a_typed_fault_for_every_malformed_shape`,
  `validate_snapshot_never_rejects_an_unrecognised_doc_schema`,
  `apply_patch_accepts_a_set_component_carrying_an_unrecognised_doc_schema` — that's 8 new minus 2 removed
  scaffold tests `surface_kind_verbatim_renames`/`surface_props_with_non_empty_doc_roundtrips` = net +6;
  `builder.rs`'s own count is unchanged, one runtime-panic test swapped 1-for-1 for one typestate-shape
  test). Before-count reconstructed from the original file contents read at the start of this session
  (2 tests in the old `surface.rs` scaffold, 1 relevant test in the old `builder.rs`), not guessed.
- Doctests: **0 → 2** (both new, on `ImageBuilder`: one `compile_fail`, one ordinary).
- `semio-framework-ui-runtime`: **62 → 62** (unchanged — no runtime-crate file was edited; confirmed via
  `git diff --stat HEAD` on `🦀️presence.rs`/`🦀️reconcile.rs` being empty before I started).
- **0 failures anywhere, by name: none.**

**Forced-rebuild dropped-future census**: not run — neither crate transitioned RED→GREEN this session
(both were already green before I started; item 1/3 needed no code change), so R12/R17's trigger
condition wasn't hit. Confirmed E6 compliance directly instead: `grep -n "async fn\|\.await"` over both
edited files (`🦀️surface.rs`, `🦀️builder.rs`) returns zero hits, and every new/changed `fn` carries the
`// 🚫️async: U1 …` tag (checked by eye against the file, region by region).

## Out of scope, not touched, FYI only

At session start, `git status` already showed `🧠️runtime/📦️packages/🦀️rust/🦀️present.rs` and
`🧠️runtime/📦️packages/🦀️rust/Cargo.toml` as modified in the working tree — not by me (I never opened
`present.rs` for writing and never touched any `Cargo.toml`). Left as-is; whichever session owns that
change should account for it separately. `Cargo.lock` and other repo-root files shown modified in the
initial `git status` are likewise untouched by this packet.

## Files touched

- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️builder.rs` (item 2 — `ImageBuilder` typestate)
- `🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust/🦀️surface.rs` (item 4 — wholesale replacement)
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/📓️terra-ui-w4-core-report.md` (this file, new)

No file outside `🖱️ui/🧬️contract/**`/`🖱️ui/🧠️runtime/**` was edited. No `Cargo.toml`/`project.json`/other
registrar-only file needed a change — both edits used only symbols/deps already available in the owned
crate.
