# 📇️/📡️ `directory` + `spr` serde removal — production/test split, conversions, and declines

Scope: `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/**` and `…/📡️spr/**`, both mounted inside
`semio-framework-os-kernel`. Measured by hand (line-range around each file's own `#[cfg(test)]`
boundary), not by a blind repo-wide grep — the ticket's own 68/62 estimate and this file's counts
differ because the estimate did not exclude `#[cfg(test)] mod` bodies inside otherwise-production
files.

## Production vs test split (before)

| file | total `serde` refs | production | test-only |
|---|---|---|---|
| `📇️directory/🦀️component.rs` | 2 | 0 | 2 (fixture decode) |
| `📇️directory/🪪️identity/🦀️component.rs` | 7 | 4 | 3 |
| `📇️directory/🔌️client/🦀️component.rs` | 26 | 19 | 7 |
| `📇️directory/🧬️schema/🦀️component.rs` | 41 | 36 | 5 |
| `📇️directory/🔌️client/🪪️runtime/🧪️tests/🦀️.rs` | 2 | 0 | 2 |
| **directory total** | **78** | **59** | **19** |
| `📡️spr/📜️history/🦀️component.rs` | 2 | 2 | 0 |
| `📡️spr/🎮️command/🦀️component.rs` | 50 | 22 (14 real code + 8 doc-prose) | 28 |
| `📡️spr/🧵️channel/🦀️component.rs` | 23 | 16 (15 real code + 1 doc-prose) | 7 |
| `📡️spr/🧪️testkit/**`, `📡️spr/🎮️command/🧪️tests/**` etc. | ~58 | 0 | ~58 (mutation-law oracle suites) |
| **spr total** | **133** | **31 real code** (+ 8 doc-prose) | **~93** |

## What got converted

**Framework-wide fix first** (`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs`): the derive
silently treated `#[value(rename_all_fields = "…")]` as a no-op, always reusing container
`rename_all` for enum-variant field casing too. Real defect — several types in my own slice
(`📇️directory/🧬️schema`'s `DirectoryCommand`/`DirectoryEventBody`/`DirectoryStreamMessage`, and per
a docstring in `🔌️plugin/🖥️host/🦀️component.rs` at least one peer's `OwnedPollInput` bridge) use a
DIFFERENT case for the tag than for the fields (`tag = "kind", rename_all = "kebab-case",
rename_all_fields = "camelCase"`), which the derive would have silently mis-cased. Added
`ContainerAttrs::rename_all_fields` + `field_rename_all()` (`rename_all_fields` when set, else
falls back to `rename_all`), rewired the 7 enum-variant-field `field_wire_name` call sites (both
`to_value`/`from_value`, all three enum representations) to use it. Struct-level field naming is
untouched (serde's `rename_all_fields` is enum-only). Verified: `cargo check -p
semio-framework-value-derive` clean, 2 pre-existing warnings only.

- **`📇️directory/🪪️identity/🦀️component.rs`**: `Identity` (4 refs) → `#[derive(ToValue, FromValue)]`
  + `#[value(rename_all = "camelCase")]`; `cache::load`/`save` → `os_pack::json::from_json_str`/
  `to_json_string`. Safe because this is a **local-only** cache file (`🪪️identity.json`) nothing but
  this same process ever reads back — the `.0`-suffix-on-integers behavior of the `DslValue`
  bridge (see below) is harmless here.
- **`📡️spr/🎮️command/🦀️component.rs`**: `TouchedPaths`, `SemanticDescriptor`, `IndexedTripleDiff<V,
  Patch>`, `MutationDescriptor`, the local `Canonical<'a>` helper inside `descriptor_fingerprint`,
  `MutationEvent` (`payload: serde_json::Value` → `payload: protocol::value::DslValue`),
  `CommandOutcome<Diff>` — all converted to `ToValue`/`FromValue`. One test rewritten
  (`operation_event_serde_round_trip`) to build the `DslValue` payload directly and round-trip
  through `os_pack::json::to_json_string`/`from_json_str` instead of `serde_json`.
- **`📡️spr/🧵️channel/🦀️component.rs`**: `CommandPageCursor`, `CommandIngressStatus` → derived
  (the latter is exactly the `rename_all` vs `rename_all_fields`-mismatch shape the macro fix
  above unblocks). `FixedCommandPage`'s hand-rolled `serde::Serialize`/`Deserialize` (a
  length-prefixed tuple over its `[u8; 4096]` backing array) replaced with hand-written
  `ToValue`/`FromValue` emitting a plain `DslValue::Array` of just the live `len` bytes (no
  blanket `[T; N]` walk over the unused tail, no redundant length field — the array's own length
  is the count).

Native check (`cargo check -p semio-framework-os-kernel --message-format=short`): **0 errors, 33
warnings**, `Finished … in 14.52s`. Guest check (`cargo build --lib --target wasm32-wasip2 -p
semio-s-plugin-draw-fsm`): **0 errors**, `Finished … in 35.78s` (after a 33-warning, 0-error
os-kernel rebuild for that target). Both verbatim tails below.

## Production serde counts, before → after

| module | real-code production sites | before | after |
|---|---|---|---|
| `📇️directory` | struct/fn conversions | 59 | 55 |
| `📡️spr` | struct/fn conversions | 31 | 4 |

(`📇️directory`'s small delta is real: almost its entire production surface is one frozen hub wire
contract, declined in full — see below. `📡️spr`'s 31→4 is the honest yield once the two frozen-byte
carve-outs are subtracted.)

## Declined — wire/disk-format risk, not converted

**Root cause common to every decline below**: `protocol::value::DslValue::Number` is a single
`f64` variant, framework-wide (`🧰️framework/🔨️modules/🌱️value/🦀️component.rs:23`). The
`pack::json` bridge (`os_pack::json::to_json_string`/`DslValueBridge::from_dsl_value`) therefore
**always** emits a trailing `.0` on a whole-number float when writing JSON text
(`write_float`'s own doc: "a whole-number float always gets an explicit `.0`… so it never
collapses onto its integer twin on the wire" — deliberate at the `pack::json::Value` level, but
lossy once something started as an `DslValue`-erased integer). A `serde_json`-based reader on the
other end that expects a genuine JSON integer (Rust/serde `u64`/`i32`/etc. field, no
`arbitrary_precision`) rejects a `.0`-suffixed literal outright. Decode direction is always safe
(text → `DslValue` → typed field tolerates either spelling); only the **encode** direction is at
risk, and only where the OLD code produced literal JSON *text* consumed by something outside this
same process/build.

1. **`📇️directory/🧬️schema/🦀️component.rs`** (36 production refs) and **`📇️directory/🔌️client/
   🦀️component.rs`**'s `🔖️Wire` region (19 production refs) — **declined in full**.
   `DirectoryCommand`/`DirectoryEvent`/`DirectoryStreamMessage`/`SessionMintResponse`/etc. are the
   Rust twin of a **real external hub server's** HTTP/WS contract (`contract §C2/§C6`,
   `POST /directory/commands`, `GET /directory/ws`, …) plus a byte-identical TypeScript client
   (`🟦️component.ts`) and a golden fixture (`🧫️fixtures/📇️directory/🧾️events.json`) diffed against
   it. Concretely: `DirectoryCommand::CreateInvite.ttl_secs: u64` posted via
   `client.command()` → `serde_json::to_vec(command)` would switch from `"ttlSecs":3600` to
   `"ttlSecs":3600.0` on the wire — a real Rust/serde hub (`🌎️hub/📦️bin.rs`'s sibling types are
   named in this same file's own comments) rejects that as `invalid type: floating point 3600,
   expected u64`. `DirectoryEvent`/`*View`'s many `_ms`/`_count`/`seq` fields carry the identical
   risk. Converting would need either an integer-preserving `DslValue::Number` variant (a
   framework-wide change far outside this slice, and actively contended — 4 other agents live in
   adjacent os-kernel modules) or a bespoke `pack::json::Value`-targeting (not `DslValue`-
   targeting) hand-written codec per wire type, duplicating ~20 types' worth of derive by hand.
   Neither is safe to land unreviewed in one session. Left on serde/serde_json entirely; no type
   here ends up deriving both (they never gained `ToValue`/`FromValue` in the first place).
2. **`📡️spr/📜️history/🦀️component.rs`** lines 813/850 (`write_op_meta`/`read_op_meta`,
   `serde_json::to_string(&meta.origin)` / `from_str`) — **declined**. This is literally embedded
   inside the **frozen `.spr` binary format** (`HistoryOpMeta`'s canonical-JSON-encoded `origin`
   field, `.🦑️repo/🎫️tickets/26/07/27/PROTOCOL-BINARY-OP-LOG-LAYER/contract.md`). `MutationOrigin`
   itself is numeric-field-free (already confirmed: it and its `SchemaId`/`ForeignTarget` fields
   are hand-written `ToValue`/`FromValue` in `📡️replication`, string-only) — but its `Contributed`
   variant carries `payload_hash: PayloadHash([u8; 32])`, whose **existing** hand-written
   `ToValue` impl (`🆔️ids/🦀️.rs:114-118`) produces a `DslValue::Array` of 32 `u8::to_value()`
   numbers. Routed through the same `f64`-erasing bridge, a persisted `.spr` file's `Contributed`-
   origin op-meta entries would switch from `[18,52,…]` to `[18.0,52.0,…]` — a byte change to a
   format already on disk. Declined; left on `serde_json` for these two call sites only.
3. **`📡️spr/🎮️command/🦀️component.rs`**'s `NamedTripleDiff<K, V, Patch>` (2 real-code refs,
   lines 276-277) — **declined, different reason: cross-crate peer dependency, not a byte risk**.
   Its `modified: Vec<ItemPatch<K, Patch>>` field needs `ItemPatch<K, Patch>: ToValue +
   FromValue`; `ItemPatch` is defined in `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️component.rs`
   (still serde-only — `vcs` is explicitly another agent's slice per this ticket's ownership
   table). Deriving `ToValue` on `NamedTripleDiff` today is a genuine, immediate `E0277` (the
   auto-synthesized generic bound covers `K`/`V`/`Patch` themselves, never the compound
   `ItemPatch<K, Patch>` the `modified` field actually needs) — not a monomorphization-time risk,
   a guaranteed compile failure. Re-attempt once `vcs::ItemPatch` gains `ToValue`/`FromValue`.
   `IndexedTripleDiff<V, Patch>` right next to it has no such dependency (raw tuples, no
   `ItemPatch`) and WAS converted.

No type in this slice ends up deriving both `serde` and `ToValue`/`FromValue` — every decline
above left the ORIGINAL serde-only derive/impl untouched rather than adding a redundant second one.

## Verbatim check tails

```
$ cargo check -p semio-framework-os-kernel --message-format=short
...
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 14.52s
```

```
$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm --message-format=short
...
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
   Compiling semio-s-plugin-draw-fsm v0.1.0 (.../✏️s/🔌️plugins/🖍️draw/.../🔄️fsm/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 35.78s
```

An intermediate `cargo check -p semio-framework-os-kernel` run (between the two above) briefly
showed 8 `E0277`s in `🏪️store/🧬️schema/🧬️mutations/{commit-space-checkpoint,create-space-
alternative}/🦀️.rs` (`SpaceCheckpoint`/`SpaceAlternative: serde::Serialize`/`Deserialize` not
satisfied) — confirmed unrelated to this slice (neither file references `directory` or `spr`;
`store` is another agent's live area per the ticket's ownership table) and gone by the very next
run minutes later, consistent with a peer's in-flight edit rather than anything introduced here.

## Files touched

- `🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs` — `rename_all_fields` support (framework-wide fix)
- `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🪪️identity/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🦀️component.rs`

## What remains (for whoever picks this up next)

- `📇️directory/🧬️schema` + `📇️directory/🔌️client`'s wire region: genuinely blocked until either
  `DslValue` gets an integer-preserving number variant, or someone hand-writes ~20
  `pack::json::Value`-targeting (not `DslValue`-targeting) codecs. Not a short follow-up.
- `📡️spr/📜️history`'s two `MutationOrigin` canonical-JSON call sites: same blocker, narrower (one
  type, one rare variant, only bites if `PayloadHash` gains a non-array or the bridge changes).
- `📡️spr/🎮️command`'s `NamedTripleDiff`: trivial once `vcs::ItemPatch` converts — flag to whichever
  agent owns `vcs` (13 refs, per the ticket's table).
- The crate's own ~150 direct `serde`/`serde_json` usages elsewhere in `os-kernel` (hand-written
  `impl Serialize for ArtifactEnvelope`/`ArtifactCursor`, etc.) are explicitly a later wave per
  `📓️verified-outcomes.md` — `directory`/`spr`'s own Cargo.toml dependency line is the crate-wide
  `os-kernel` one and was correctly left untouched (removing it needs every file in the crate
  converted, not just these two modules).
