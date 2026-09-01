# 🔌️🏪️ `🔌️plugin` + `🏪️store` — `ToValue`/`FromValue` added alongside serde (derive-level wave)

## Headline

Guardrail held throughout: `cargo check -p semio-framework-os-kernel --message-format=short` was
**0 errors** before this wave started and is **0 errors, 33 warnings** (identical warning count —
no new lint) after it, verified fresh multiple times, including after every batch of edits.
`cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm` re-verified clean (0 errors,
14.47s) — no regression on the one plugin this ticket has already proven for the shipped target.

This wave is the **additive** phase only, per the ticket's own instruction ("add alongside first,
remove serde only once the whole module is converted"). Raw `serde`-substring reference counts
therefore did not drop (`🔌️plugin` 786 → 786, `🏪️store` 419 → 423 — the +4 is prose in new
docstrings mentioning "serde", not new code) — the metric that moved is **derive sites converted**:

| module | derive-eligible `Serialize`+no-`ToValue` sites at start | converted this wave | remaining (all deliberately not derived, see below) |
|---|---|---|---|
| `🏪️store` | 40 | 33 by script + 3 hand-written (`serialize_with`/`deserialize_with`) | 9 |
| `🔌️plugin` | 82 (79 real + 3 identical inside one unused test-only macro) | 78 by script | 4 (1 doc-comment false-positive + 3 in the unused macro) |

`serde`/`serde_json` were **not removed** from either `Cargo.toml` — correct per the ticket: the
call-site population (`serde_json::` — 434 refs/53 files in `🔌️plugin`, 215 refs/15 files in
`🏪️store`) is untouched and is why both crates still need serde. See "What remains" below.

## Method

Wrote a Python script (`🔬️verification-plugin-store-derive/convert.py`, this ticket folder) that,
per `#[derive(...)]` line containing `Serialize`/`serde::Serialize` without `ToValue`:
1. locates the struct/enum body via brace counting,
2. skips tuple structs, and skips (flags, does not touch) any item using an attribute key outside
   the derive's supported set (`rename_all`, `tag`, `content`, `default`, `deny_unknown_fields`,
   `transparent`, `bound`, `rename`, `skip_serializing_if`, `serialize_with`, `deserialize_with`) —
   in particular bare `#[serde(skip)]` and `#[serde(untagged)]`,
3. otherwise inserts `ToValue`/`FromValue` into the derive list and mirrors every `#[serde(...)]`
   line as an adjacent `#[value(...)]` line with the same (translated) key list.

Verified current derive macro support directly from
`🧰️framework/🔨️modules/🌱️value/✨️derive/🦀️component.rs`'s own docstring/parser (more current than
the ticket's playbook doc, which predates generic-bound synthesis and `serialize_with`/
`deserialize_with` support): generic structs/enums now get an automatic per-parameter `ToValue`/
`FromValue` bound, so `ArtifactCommand<Mutation>` derives cleanly with zero hand-written impl.

## Two accidental conflicts, found and reverted

The script's "does this struct already have `ToValue`?" check only looks at the struct's own
`#[derive(...)]` line — it has no way to know a **hand-written** `impl ToValue for X` exists
elsewhere in the file. Two pre-existing hand-written impls were double-derived this way, both
already flagged by name in this ticket's own prior research:

1. **`ArtifactEnvelopeRead<'a, P, Mutation>`** (`🏪️store/🦀️component.rs`) — the borrowed struct
   `os-kernel-tovalue-cascade.md` explicitly says NOT to convert (its `capture_read()` fallibility
   discussion). Reverted to `#[derive(Serialize)]` + plain `#[serde(...)]`, no `#[value(...)]`.
2. **`SpaceHistoryDiff`** (`🏪️store/🦀️component.rs`) — already had a hand-written
   `impl ToValue for SpaceHistoryDiff` (routes through the serde bridge because
   `add_checkpoint`/`add_alternative` are `Option<foreign-crate-type>`, orphan rule). Reverted to
   `#[derive(Serialize, Deserialize)]`, no `#[value(...)]`.

Both caused `E0119: conflicting implementations`, caught by the very next `cargo check`, fixed
before moving on. No other conflicts found — a targeted `grep -rn "impl.*ToValue for "` over both
modules before and after confirms the only hand-written impls left standing
(`ArtifactChild<S>`, `SpaceHistoryMutation`, `CommitSpaceCheckpoint`, `CreateSpaceAlternative`,
`InteractionConfigMutation`, `SetInteractionState`) are untouched by the derive and none conflict.

## Converted by hand (derive can't reach these — `#[serde(with = "...")]`)

The derive supports `serialize_with`/`deserialize_with` (function pointers to
`fn(&T) -> DslValue` / `fn(DslValue) -> Result<T, ValueError>`) but not serde's `with = "module"`
shorthand (expands to two functions with serde's own `Serializer`/`Deserializer`-based signatures,
which don't match). Three enums in `🏪️store` used `with =`:

- `ArtifactActorMsg::LocalMutations.envelopes` and `ArtifactEvent::RemoteMutations.envelopes`
  (`🏪️store/🔄️sync/🦀️component.rs`) — both route through the same `envelope_serde` module. Added
  `envelope_serde::to_value`/`from_value` alongside the existing `serialize`/`deserialize`,
  identical `encode_envelopes`/`decode_envelopes` byte framing, wire shape
  `DslValue::Array` of `DslValue::Number` (one per byte) — the same shape `Vec<u8>`'s own
  blanket `ToValue` impl already produces.
- `ArtifactCommand::IngestRemote.envelope` (`🏪️store/🦀️component.rs`) — same treatment,
  `operation_envelope_serde::to_value`/`from_value` added, delegates to `ToValue::to_value(&bytes)`
  since it's a plain `Vec<u8>` (no manual byte-array construction needed there).

All three enums (`ArtifactActorMsg`, `ArtifactEvent`, `ArtifactCommand<Mutation>`) now derive
`ToValue, FromValue` with `#[value(tag = "kind", rename_all = "camelCase")]` mirroring their
existing internally-tagged serde shape; every other field in each enum's variants was confirmed to
already have `ToValue`/`FromValue` on its own type (`HistoryLane`, `UndoPolicy` — hand-written,
`Author`, `ArtifactSyncStatus`, `CommandAckOutcome`, `PresencePeer` — hand-written in
`📡️replication`, `MutationMessage` — hand-written in `📡️replication`) before adding the derive, so
no transitive gap was introduced.

## Not converted, deliberately — 9 in `🏪️store`, 4 in `🔌️plugin`

| type/site | why |
|---|---|
| `ArtifactEnvelopeRead` | explicitly out of scope per `os-kernel-tovalue-cascade.md` (reverted, see above) |
| `SpaceHistoryDiff` | hand-written impl already exists (reverted, see above) |
| `ArtifactChild<S>` | composed-child, hand-written impl already exists (untouched, correct) |
| `SpaceHistoryMutation` | hand-written impl already exists, routed through serde bridge (untouched) |
| `CommitSpaceCheckpoint`, `CreateSpaceAlternative` | each already has a hand-written `impl protocol::value::ToValue`/`FromValue` (`json_to_dsl`/`dsl_to_json` bridge) added concurrently by another session while this wave was in flight — confirmed via `git diff` showing only this wave's own 1-line import addition, the `ValueBridge` regions predate it. Left as-is; correct and compiling. |
| `MapValue` (`🏪️store/🧵️canonical-edit/🧵️borrowed/🧪️component.rs:10`) | `#[serde(untagged)]` — not derive-supported. **Test-only**: this whole file is `#[path=...] mod borrowed_tests` behind `#[cfg(test)]` in the parent, so it never reaches `cargo check`'s lib target. Deferred, not blocking the guardrail. |
| `MapMutation` (same file, line 14) | bare `#[serde(skip)]` on two fields (`Arc<MapLifetime>`, `bool`) — not derive-supported (no `skip` key at all in the macro). Same test-only file, same deferral. |
| `🔌️plugin/🦀️component.rs:10481` | inside a `///` doc-comment example, not real code — false positive, no action needed |
| `🔌️plugin/🦀️component.rs:10556/10631/10705` | inside `macro_rules! app_commands!`, whose own doc comment states "no real plugin adopts it yet" — confirmed only invoked from its own `#[cfg(test)] mod app_commands_tests`. Left on serde; low priority given zero production callers. |

## What remains (counts, for whoever picks this up)

1. **`serde_json::` call sites** — the bulk of the remaining work, untouched this wave:
   `🔌️plugin` 434 refs / 53 files, `🏪️store` 215 refs / 15 files. Top patterns by frequency:
   `from_str` (107+36), `Value` (84+65 — many as fixture/oracle types that should probably **stay**
   serde per this ticket's own "keep serde as a dev-dependency oracle" contract, needs per-site
   judgment not a blind sweep), `to_vec`/`to_string`/`from_slice`/`to_value`/`from_value`/the
   `json!` macro. Each needs the playbook's "JSON text vs `DslValue`" judgment call — not
   mechanical, hence not attempted in this pass.
2. **`app_commands!` macro** (`🔌️plugin/🦀️component.rs`, 3 near-identical `#[derive(...)]` lines
   inside the `macro_rules!` body) — currently serde-only, zero production callers. Converting the
   macro template itself (not just a call site) needs care since it must still work for its own
   test invocations; deferred rather than rushed.
3. **`MapValue`/`MapMutation`** (`🏪️store/🧵️canonical-edit/🧵️borrowed/🧪️component.rs`) — test-only,
   needs hand-written `ToValue`/`FromValue` (untagged-enum try-each-variant decode, and a
   `Default`-based reconstruction for `MapMutation`'s two skipped fields on `from_value`). Small,
   self-contained, ~20 lines of hand code — left for whoever next touches this file since it does
   not gate `cargo check`.
4. Once (1)-(3) are closed and every file in both crates is serde-free, remove `serde.workspace =
   true` / `serde_json = {...}` from `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/Cargo.toml`
   (shared by both modules, one crate) — **not done this wave**, correctly, since the crate is far
   from serde-free.

## Verbatim tails

```
$ cargo check -p semio-framework-os-kernel --message-format=short   (baseline, before this wave)
warning: `semio-framework-os-kernel` (lib) generated 33 warnings
    Finished `dev` profile [unoptimized] target(s) in 6.56s

$ cargo check -p semio-framework-os-kernel --message-format=short   (final, after this wave)
warning: `semio-framework-os-kernel` (lib) generated 33 warnings (run `cargo fix --lib -p semio-framework-os-kernel` to apply 33 suggestions)
    Finished `dev` profile [unoptimized] target(s) in 1.12s

$ cargo build --lib --target wasm32-wasip2 -p semio-s-plugin-draw-fsm --message-format=short
    Compiling semio-s-plugin-draw-fsm v0.1.0 (...)
    Finished `dev` profile [unoptimized] target(s) in 14.47s
```

## Files touched

`🏪️store`: `🦀️component.rs` (33 derive sites + `ArtifactCommand` hand fix + 2 reverts),
`🔄️sync/🦀️component.rs` (2 enum hand fixes + `envelope_serde::to_value`/`from_value`),
`🧬️schema/🧬️mutations/📌️commit-space-checkpoint/🦀️.rs` and `🌿️create-space-alternative/🦀️.rs`
(import only, struct itself already hand-converted concurrently), plus 30 other files under
`🏪️store/**` for the mechanical derive+attribute batch (`👥️presence`, `🧵️canonical-edit`,
`🧬️schema/🧬️mutations/**`).

`🔌️plugin`: `🦀️component.rs` plus 19 other files under `🔌️plugin/**` for the mechanical batch
(`⚛️reactor/**`, `🌐host`, `🖥️host/**`, `🕹️interaction/**`, `🧪️tests/**`), 9 of which also needed a
`use semio_framework_value_derive::{FromValue, ToValue};` import added (the rest already had it, or
use the codebase's fully-qualified `::semio_framework_value_derive::ToValue` convention inline).

No `Cargo.toml` was touched — both crates still declare `serde`/`serde_json` and must keep doing so
until the "What remains" items above are closed.
