# Value Admission and Clone Grant Independent Audit

## Scope and method

Read-only source audit of the framework Value clone cursor and Store canonical
Value admission only. I read the prerequisite ticket notes:

- `shared-value-cursors-independent-audit.md`;
- `shared-value-clone-grants.md`;
- `shared-value-retained-cursors.md`.

I did not edit production sources, run Cargo, Bun, Nx, or any test command, or
inspect unrelated ownership families. Root supplied execution evidence after
this source review began: canonical admission GREEN3 completed with four native
tests, Nx exit 0, in 8m19s. Clone GREEN4 remains queued on the shared Cargo
lock. Those are reported execution receipts, not work performed by this audit.

## Confirmed source properties

1. `DslValueCloneCursor` now honors independent per-call structural and byte
   permits while active. `advance` only increments checkpoint counters from its
   receipt (`🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs:188-209`), structural
   branches require an item permit (`:214-256`, `:287-303`), and byte copying
   uses the smaller of the caller allowance and 256 (`:131-148`, `:259-284`).
   The four-byte pending buffer preserves valid UTF-8 across one-byte grants.
   The source test covers grants 0/1/7/256, split item-only and byte-only work,
   and cancellation checkpoints (`🧪️tests/🔬️unit/🦀️.rs:97-149`).
2. Clone completion and bounded retirement transfer the exact owner correctly:
   `take_value` moves only the candidate value (`🦀️.rs:306-316`), and the final
   close step removes the retained source and returns it in the `Returned`
   receipt (`:324-370`). The close traversal pops a key, child edge, or frame
   per granted item; it does not recursively walk a nested value in `Drop`.
3. Store canonical JSON is no longer implemented directly for raw `DslValue`.
   Only the private-field `ArtifactCanonicalValue<R>` implements the encoder
   trait (`🧵️canonical-edit/🌱️value/🦀️.rs:24-43`), so a wrapper can only arise
   through admission. Admission visits tree children by index, hashes and
   compares key bytes in caller-sized chunks capped at 256, and rejects an
   identical key before it can expose that wrapper
   (`🪪️admission/🦀️.rs:81-185`). This correctly rejects raw duplicate entries,
   including nested objects, while retaining the raw clone primitive's
   duplicate-preserving behaviour.
4. The duplicate policy agrees with the first-party Pack writer. Pack's
   `Object::insert` replaces a prior key in place (`🧰️framework/🔨️modules/🎒️pack/🔤️json/🦀️.rs:190-196`),
   and collection invokes that insertion path (`:234-240`),
   `from_dsl_value` collects raw entries into that object (`:540-552`), and
   `to_json_string` uses that bridge (`:1436-1438`). The canonical fixture
   captures the resulting last-value-wins, insertion-order-preserving bytes for
   `[z:1, a:2, z:3]` and expects admission rejection
   (`🧫️fixtures/🔣️.json:22-37`).

## Findings

### P1 — A zero-grant close silently cancels an otherwise resumable owner

Both close APIs mutate phase before checking whether the caller supplied a
usable grant:

- clone calls `self.cancel()` at
  `🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs:325-331`, then reports
  `Blocked` for `maximum_items == 0`;
- canonical admission calls `self.cancel()` at
  `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🌱️value/🪪️admission/🦀️.rs:196-198`,
  then returns `false` when either limit is zero.

That contradicts the clone contract's zero-grant guarantee of no phase,
checkpoint, UTF-8, or ownership change
(`🧬️clone/🤝️contract.json:11-15`) and the admission contract's “Blocked
without mutation” rule (`🌱️value/🤝️contract.json:11-15`). For an active
cursor, a speculative close with a zero grant changes it to `Closing`; its next
`advance` then reports `closing` rather than continuing. The current tests only
try a zero close after an explicit `cancel` or after `take_value`, so they do
not observe this transition (`clone tests:34-52`; `admission tests:82-84`).

Check the grant before `cancel` in both `close_step` methods. Keep cancellation
as its explicit, ungranted state transition. Add the 0/1/7/256 matrix case that
calls `close_step(0)` while still active, asserts the complete checkpoint and
future `advance` result are unchanged, then drives the same cursor normally.

### P1 — Canonical cancellation completes without performing the promised owner transfer

`ArtifactCanonicalValueAdmission::close_step` marks the object `Closed` and
returns a bare `true` while its `ManuallyDrop<Option<R>>` still holds the source
(`🪪️admission/🦀️.rs:196-204`). The caller must make a separate ungranted
`take_source` call (`:206`) to transfer that owner. Until then,
`terminal_is_empty` is false (`:208`) and `Drop` asserts, or during unwinding
leaves the manually held source unreleased (`:211-214`).

This differs from the clone cursor, whose terminal close result contains the
source and is terminal-empty in that same granted transition
(`🧬️clone/🦀️.rs:358-370`). It also falls short of the admission contract's
statement that bounded close “returns the identical source”
(`🌱️value/🤝️contract.json:16`). A Wires parent can therefore record bounded
close completion before it has received its retained `SnapshotRead` equivalent,
and a missed immediate `take_source` loses the exact-owner handoff.

Replace the boolean close result and `take_source` split with a must-use close
receipt that includes `Returned { source, ... }`, plus `Progress`, `Blocked`,
and `Complete` variants, matching the clone shape. The receipt must report the
single structural retirement work that the parent has granted. Extend native
tests to require that the returned source is available only in the terminal
close receipt and that the cursor is terminal-empty immediately afterward.

### P2 — The advertised retained-byte limits are checked after allocator commitment, not a hard memory ceiling

Both primitives validate a requested capacity before calling
`try_reserve_exact`, but only verify the actual `Vec`/`String` capacity after
the allocation has occurred:

- clone: `reserve_vec` and `reserve_string` at
  `🧰️framework/🔨️modules/🌱️value/🧬️clone/🦀️.rs:97-119`;
- canonical numeric key table:
  `🧵️canonical-edit/🌱️value/🪪️admission/🦀️.rs:117-122`.

On an allocator that supplies more capacity than requested, the cursor has
already committed more than `maximum_retained_bytes` before it detects the
overage and enters closing. This makes the limit a requested-capacity policy,
not the real retained-memory ceiling claimed by both contracts
(`clone contract:5-10`; `admission contract:4-10`). Existing tests only cover
the pre-reservation rejection and ordinary successful allocations; none causes
an actual-capacity overage.

Either define these limits explicitly as requested allocation capacity or use a
reservation/allocation mechanism whose committed capacity can be guaranteed
against the caller's grant before the allocation becomes observable. In either
case, add an injected allocator/reservation test that demonstrates the chosen
contract at the post-reservation boundary. This finding does not claim a
failure on the current allocator; it identifies a source-level gap in the
promised hard-cap semantics.

## Lifecycle conclusion

After an explicit positive-grant cancellation path, neither primitive contains
an application-level recursive drain: clone removes one owned structural unit
per close call, and admission removes one numeric slot or traversal frame per
call before freeing its flat table. The normal clone source handoff is exact.
The two P1 lifecycle gaps above must be resolved before a retained Wires parent
can treat zero grants and terminal close receipts as authoritative ownership
boundaries.
