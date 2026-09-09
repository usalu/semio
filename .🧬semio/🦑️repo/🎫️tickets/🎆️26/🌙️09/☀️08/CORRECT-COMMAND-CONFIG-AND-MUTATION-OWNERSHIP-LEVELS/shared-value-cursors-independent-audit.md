# Shared Value Cursor Independent Audit

## Scope and method

This is a read-only review of the new framework `DslValue` clone cursor and the
OS Store canonical JSON bridge, their contract/fixture/native tests, and the
two prerequisite notes:

- `shared-value-retained-cursors.md`;
- `wires-retained-move-audit.md`.

No source, test, generated output, or Git state was changed. The ticket's
reported native receipts are accepted as prior execution evidence: clone has
three green native tests and the bridge has one. No Cargo build was run here.

The ownership split itself is correct. The owned-copy primitive belongs under
framework Value, and the `ArtifactCanonicalJson` implementation belongs beside
the Store traversal that consumes it. The clone source retains one domain-owned
root and returns that exact owner only after value transfer or terminal close;
the Store bridge only borrows indexed values and does not create another owner.

## Confirmed behaviour

1. The clone contract explicitly requires a retained immutable source, at most
   64 indexed lookups, one structural item or 256 copied UTF-8 bytes per step,
   payload-capacity accounting, ordered duplicate keys, and bounded
   cancellation: `🧰️framework/🔨️modules/🌱️value/🧬️clone/🤝️contract.json:3-16`.
   The implementation selects source paths only by array/object index at
   `🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs:92-102`, preserves `Object`
   entry order while attaching copied children at `:123-137`, and UTF-8 backs
   chunk ends to a character boundary at `:84-90`.
2. Allocation capacity is checked before a `String`/`Vec` reserve at
   `🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs:58-73`, then retained capacity
   is accumulated at `:143-157` and `:165-176`. The close walk removes exactly
   one key, child edge, allocation, or frame per call at `:203-226`.
3. Completion cannot return the source until the caller transfers `output`:
   `take_source` admits it only after `take_value`, or after the closed phase,
   at `🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs:190-201`. The existing tests
   exercise exact `Arc` identity after normal transfer, after ten cancellation
   checkpoints, and after limits failures at
   `🧰️framework/🔨️modules/🌱️value/🧬️clone/🧪️tests/🔬️unit/🦀️.rs:12-93`.
4. The Store bridge performs indexed reads only: it maps node kind/length in
   `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🌱️value/🦀️.rs:20-34`
   and reads an object key by its entry index at `:36-39`. The fixed-state
   encoder emits no more than the supplied output length capped at 256 bytes,
   `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:534-546`.
   This is the right non-owning abstraction for Store sealing.

## Actionable defects

### P1 — The clone has no caller grant, so a retained parent cannot honor a smaller work budget

`DslValueCloneCursor::advance` takes no grant and always invokes an internal
256-byte copy limit (`🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs:114-120`,
`:84-90`, `:160-162`, `:174-176`). A caller with an exact one- or seven-byte
turn grant therefore cannot prevent a 256-byte copy. Structural setup can also
make a complete permitted payload allocation in one call through
`reserve_string`/`reserve_vec` (`:58-73`, `:148-150`, `:168-170`), even though
the caller may need to account for that work before entering the turn.

This satisfies the current standalone contract's fixed maxima, but it does not
satisfy the Wires design requirement that each nested cursor consume the
retained operation's actual grant. The Wires audit already requires all
candidate copy and retirement stages to be bounded by the admitting capacity
and grants (`wires-retained-move-audit.md:158-164`, `:241-244`). This is an
integration blocker, not an excuse to wrap `advance()` in a loop.

Change the API to accept an explicit clone grant with an item permit and a
maximum copied-byte allowance. A zero grant must report `Blocked` without
state change; a positive grant must cap `copy_chunk` by the smaller of its
allowance and 256. Keep `DslValueCloneLimits.maximum_retained_bytes` as the
whole-operation retained-capacity ceiling, separate from per-turn copied work.
Before a structural reserve, charge/admit that allocation under the parent
operation's stated policy rather than silently relying on the fixed 1 MiB test
limit.

Add a language-neutral grant matrix and native test that drive the large UTF-8
fixture with 0, 1, 7, and 256-byte grants. For every step, assert: no progress
under zero; copied-byte delta is at most the supplied grant and 256; one or
fewer structural items; retained capacity never exceeds the operation ceiling;
and cancellation after every matrix position reaches terminal-empty and
returns the identical source. The current clone test checks only the fixed
256-byte limit (`🧰️framework/🔨️modules/🌱️value/🧬️clone/🧪️tests/🔬️unit/🦀️.rs:16-30`).

### P1 — Canonical bytes diverge from `os_pack::json` for duplicate object keys

`DslValue` intentionally represents an object as a raw ordered vector and the
clone contract intentionally preserves duplicate keys
(`🧰️framework/🔨️modules/🌱️value/🦀️.rs:103-110`; clone contract `:11`). The
Store bridge declares the raw entry count and returns each raw key by index
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🌱️value/🦀️.rs:29-39`),
so the canonical cursor emits every duplicate entry (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs:481-518`).

The production first-party JSON route instead converts `DslValue` through
`from_dsl_value` at `🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:540-552`.
Its collection construction calls `Object::insert` (`:234-240`), which replaces
an existing key's value (`:190-196`), and `to_json_string` always uses that
conversion (`:1436-1438`). Thus a raw `[("z", 1), ("a", 2), ("z", 3)]` is
canonically streamed as three entries but is produced by `os_pack::json` as
the two-entry, last-value-wins object. That produces different Store bytes and
therefore different content hashes/identities for semantically equivalent
input.

The current bridge test does not cover this case. Its injected adversarial
object has distinct `z`, `a`, and `m` keys
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🌱️value/🧪️tests/🔬️unit/🦀️.rs:37-41`),
whereas the clone test does exercise duplicate preservation
(`🧰️framework/🔨️modules/🌱️value/🧬️clone/🧪️tests/🔬️unit/🦀️.rs:50-52`).

Choose and publish one representation rule before Wires reuses this bridge:

- Recommended for a Store canonical hash: canonical JSON sources must be
  duplicate-free. Keep the raw-value clone contract, but remove the direct
  `ArtifactCanonicalJson for DslValue` admission path. Introduce a bounded,
  source-owning duplicate-key validation cursor that produces a
  duplicate-free canonical-source wrapper; Wires drives it with the same
  grant/cancel/close rules before sealing. Do not scan whole objects inside
  `canonical_json_node`/`canonical_json_key`, because the trait explicitly
  forbids scans (`canonical-edit/🦀️.rs:48-60`).
- If duplicate-preserving JSON is instead intended canonical wire behaviour,
  change the pack bridge and its `Object` query semantics together so
  `os_pack::json::to_json_string` preserves the raw sequence. A one-line
  bypass of `Object::insert` is unsound: `Object::get` currently returns the
  first duplicate (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:178-184`) while
  the documented public insertion semantics are last-value-wins.

The required regression must use an actual duplicate object, assert the chosen
bytes at 1, 7, and 256-byte chunks, and compare the same chosen representation
with `os_pack::json::to_json_string`. The test must fail before the selected
fix, rather than merely testing valid JSON parse equivalence.

## Lifecycle obligation before Wires adoption

This is not a standalone functional failure under the written contract, but it
is an enforceable Wires integration obligation. `cancel()` only changes phase
(`🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs:201`); source and partial output
remain owned until repeated `close_step()` calls drain them (`:203-226`) and a
separate `take_source()` transfers the owner (`:197-199`). The type has no
custom `Drop`. Dropping a partially advanced cursor therefore drops its source
and nested output through ordinary Rust destruction, bypassing both bounded
retirement and the exact `SnapshotRead` return path planned for Wires.

Do not add a destructor that recursively drains: that would hide unbounded work
in `Drop`. Instead, the Wires retained preparation must own the cursor in an
explicit close state, drive one `close_step` only when granted, take the exact
source only after terminal close, and make its own `terminal_is_empty` reject
any remaining cursor/source. Improve this API by returning the source as the
terminal close result (and by making the cursor `#[must_use]` plus a debug
terminal assertion) so callers cannot accidentally report terminal before the
second transfer. Add a Wires lifecycle regression that cancels at each clone
stage and proves the exact base `SnapshotRead` returns to the registry before
the live authority is dismissed. This follows the release ordering already
specified in `wires-retained-move-audit.md:158-164` and `:236-239`.

## Non-findings

- The UTF-8 chunk boundary algorithm is correct for the fixed 256-byte limit:
  every target length remains a character boundary and 256 exceeds the maximum
  UTF-8 code-point width (`🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs:84-90`).
- The normal completion and explicit-close state transition does preserve source
  ownership; `terminal_is_empty` intentionally remains false until the source
  is actually transferred (`:228-230`).
- The canonical bridge's indexed lookup implementation has no hidden object
  key scan and belongs at its present Store boundary.
