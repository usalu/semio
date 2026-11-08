# 🔎️ 1107 Audit Review 9 — Store Backbone, Child, Canonical JSON, Space Schema, Action Bus

Lane `review-9`. Read-only audit. No sub-agents, no cargo, no git modifying commands, no edits outside this report. Nothing was compiled or run; every finding below comes from reading the source.

## Scope

Base paths (`S` = `/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store`, `A` = `/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🎯️action-bus`). File paths below are relative to those bases.

| Crate (Cargo `name`) | Location | Status |
|---|---|---|
| `semio-os-store-backbone` | `S/🔗️backbone` | read in full (lib, `♻️retirement`, `📤️original-outbox`) |
| `semio-os-store-child` | `S/🪆️child` | read in full (lib, receive, retained-clone, owner, owner schema, read, lifted retirement) |
| `semio-os-store-canonical-json` | `S/🧵️canonical-edit` | `🧳️source`, `📖️reader`, `🪪️source-identity` (grep and partial), `🌱️value/🪪️admission` (spot checks); `🧵️borrowed`, `🔏️borrowed-digest` only grep |
| `semio-os-store-space-schema` | `S/🧬️schema` | `📜️space-history` read in full; alternative mutations read; checkpoint/snapshot lift only grep |
| `semio-os-action-bus` | not found | No Cargo.toml with `name = "semio-os-action-bus"` exists anywhere under `/Users/ueli/Documents/semio` (the background search for it finished with no matches, excluding `target/` directories). I audited `A` (Cargo name `semio-framework-action-bus`) as the closest match: `🦀️.rs`, `🏛️registered`, `🚪️admission`, `📨️wire/🌱️original`. Tests not read. |

Value-crate contracts used to judge findings were read where cited: `RetirementPollExit::settle/pay/resume` (`🧰️framework/🔨️modules/🌱️value/♻️retirement/🧬️contract/🦀️.rs:10-29`), `factory_retire_snapshot` (`…/♻️retirement/🏭️factory/🦀️.rs:251-258`), and the job context `consume_retained` / `consume_fuel` (`🧰️framework/🔨️modules/🧵️job/⏱️context/🦀️.rs:258-264, 415-417`). `consume_retained` does not debit fuel, so `consume_fuel` is the only fuel debit in the paths reviewed.

## Findings, most severe first

### 1. High: a channel backbone whose actor installed its wake can never finish retiring

- `S/🔗️backbone/♻️retirement/🦀️.rs:68-70`: `demands()` returns `Err(UnsupportedOwner)` whenever the `wake` OnceLock already holds a callback.
- `:101`: `close_step` calls `self.demands()?` before it reaches the wake branch at `:125-130`.
- `:144`: a `Channel` backbone's `outbound_wake` is moved into `self.wake`, so the next close step hits `:101` and fails.
- Production installs the callback: `S/🔄️sync/🖥️native/🎭️actor/⏲️actor-runner-scheduling-terminal-and-spawn/🦀️.rs:386` runs `actor.remote.set_outbound_wake(schedule.clone())`. No test in the tree installs a wake (grep for `set_outbound_wake` finds only this call site).

Concrete failure: the store closes a channel backbone. The actor drops its `ChannelBackboneRemote` and the queues detach (`:142`). The wake then moves into `self.wake` (`:144`). Every later close step returns `Failed(UnsupportedOwner)`. The owner stays non-terminal, and its `Drop` (`:161`) asserts if the caller drops it. The error text says the wake "has no admitted retirement authority", so this is a missing paid release path, not a legitimate refusal. Fix direction: give the wake callback a paid release (it is an `Arc<OnceLock<Arc<dyn Fn>>>`, no external work) instead of refusing it.

### 2. Medium-high: close waits on an external owner with zero progress and no bounded refusal

- `S/🔗️backbone/♻️retirement/🦀️.rs:142-143`: when `detach_queue` returns `None` (`:81-87`, because `Arc::strong_count != 1`), the step returns `Progress(default)`. The step has no work, no progress, and no `Blocked` reason.
- The holder is the actor's `ChannelBackboneRemote` (`S/🔗️backbone/🦀️.rs` fields `inbound`/`outbound`). The only release helper, `ChannelBackboneRemote::close_alias_step` (`🔗️backbone/🦀️.rs:525`), has zero callers in the repo (grep on `close_alias_step` finds only its definition).
- Concrete failure: if the actor never drops its remote during shutdown, the store close returns zero-work Complete(Progress) forever. Each job turn re-enters with nothing recorded. The same shape applies to `MemoryBackbone` pairs. I did not verify whether the actor drops the remote on shutdown.

### 3. Medium: child retirement drops its local materialization without accounting

- `S/🪆️child/📥️lifted/♻️retirement/🦀️.rs:8`: `fn retirement_seed(self) -> … { (self.child_id, self.target).retirement_seed() }` moves only two fields. `self.local_owner` (an `Arc<dyn Any + Send + Sync>`) is dropped at the end of the function, unpaid and unreported.
- `:9` declares `controlled_retirement_supported() -> true`, so the controlled path accepts the child.
- The retained-clone path does this correctly with a paid `AliasRetirement` (`S/🪆️child/🧬️retained-clone/🦀️.rs:14-25`). The `RetireOwned` route bypasses it.
- Concrete failure: retiring a parent snapshot that holds a child with a large local text or mesh frees that materialization with no `work` and no `released_bytes`. Budget accounting undercounts the release.

### 4. Medium: funded construction is capped by a retry loop and a per-call constant (W2 violation), and one construction path records constant work

- `A/🚪️admission/🦀️.rs:96`: `settle` demand adds `items+3` and `copy_bytes + 64*size_of::<usize>()`. These are per-call constants standing in for a missing turn.
- `:99`: `for _ in 0..3 { … }` is the forbidden `for 0..N` retry loop.
- Concrete failure: the third iteration returns `Pending` while holding an output (`output.is_some()`). Control falls out of the loop and returns `OwnershipLimit`, "exceeds its funded construction turn" (after `:99`). A construction that needs one more turn is refused rather than continued next turn. The brief says chunk, do not cap.
- `:88` (`admit_original_tool_construction_scoped`) and `:125` (`admit_original_tool_construction_settled`): `work: u128::from(result.is_ok())`. Constant work 1 is charged regardless of how many actions the caller's `construct` closure ran. Only `admit_owner` reserves quotes and records no work. If the closure does more than one physical action, the work is unrecorded. Constant `work:1` is honest only for exactly one action (W2).

### 5. Medium: canonical source takes the publication-authority lease before paying, and drops it on refusal

- `S/🧵️canonical-edit/🧳️source/🦀️.rs:111`: `else if let Some(original) = self.pending_authority.take() { pay(effect,1)?; … }`. The `take()` runs before `pay`.
- Compare `:107`, `:109`, and `:152`, which pay first and then take.
- Concrete failure: when the work limit is hit at `:111`, `pay` returns `Err`. `original` (the `Arc<ArtifactStoreOneItemLiveAuthority>` strong alias) is dropped on the `?` return. That is an unpaid release of the authority alias. The sealer relies on that strong count (the comment at `:13` says so). I did not verify whether dropping `Live` has side effects beyond the decrement.

### 6. Medium (reachability unproven): canonical admission panics on an admit refusal

- `S/🧵️canonical-edit/🧳️source/🦀️.rs:90-91`: `ArtifactCanonicalEditAuthorityCursor::admit(…).unwrap_or_else(|_| unreachable!("canonical paid semantic aliases"))` and the identity variant.
- The gate before this (`advance`, `:148`) checks `permits(d, g)` against `next_demand()` (phase 2/3 nested `copy_bytes` only, `:140-141`). It does not check capacity, release, or work. An admit that needs capacity can be refused, and then `unreachable!` panics in a job turn after `pay(effect,1)?`. I did not check whether `admit` can refuse under a grant that passes `permits`.

### 7. Medium: a catalogue miss is reported as an internal invariant, not a dispatch refusal

- `A/📨️wire/🌱️original/🦀️.rs:24`: `inner.original_entries.get(self.row).else{ return Err(ValueError::invariant("original wire factory is not registered")) }`.
- The row walk advances on every key mismatch (`self.row += 1` inside `advance`). When no registered factory matches, `row == len`, so every unknown controller/tool id ends at this line.
- `begin_exact_wire` (`A/🦀️.rs` around line 682) returns `ToolDispatchError::UnknownController` for the same input.
- Concrete failure: a wire request naming an unregistered tool reaches an invariant violation (an internal-bug class) instead of a typed refusal. Callers that treat invariant errors as bugs will mis-handle user input.

### 8. Medium: removing the active alternative leaves a dangling active id

- `S/🧬️schema/🧬️mutations/🧹️remove-space-alternative/🦀️.rs:35-37`: `diff` emits only `RemoveAlternative`, with no check and no `SetActive`.
- `S/🧬️schema/📜️space-history/🦀️.rs:165-170`: `step_into(RemoveAlternative)` removes the row and never touches `active_alternative_id`. (`SetActive` at `:171-178` validates existence; `RemoveAlternative` does not.)
- `:221-233` (`touches`) declares only region `alternatives` for this step, so the active region is not reported as touched either.
- Snapshot decode (`S/🧬️schema/📥️lifted/📸️snapshot/🦀️.rs:111`, `:176`) also accepts an active id without checking it names an existing alternative.
- Concrete failure: remove the active alternative. The snapshot keeps `active_alternative_id = Some(X)` with no row X. Later `SetActive(X)` or `Switch(X)` refuses ("does not exist"), while the active id still shows X. The inverse is consistent, so the fault is in the forward state.

### 9. Medium: the outbox loses a queued message on allocation failure and then wedges

- `S/🔗️backbone/📤️original-outbox/🦀️.rs:56`: `acknowledge_delivered` does `rows.get_mut(head).take()` and then `ControlledRetirement::new(bytes).map_err(|(e,_)| e)?`. On error the bytes are dropped, the row at `head` is now `None`, and `head` is not advanced.
- Consequence: `front()` returns `None` and `acknowledgement_demands` returns `invalid()` on every later call. Nothing can deliver or close the outbox cleanly, and `Drop` (`:67`) asserts when it is dropped.
- Same shape at `:64` (`close_step`): the row is popped, the bytes are dropped on `ControlledRetirement::new` error, and the error is returned.
- The module doc promises "Prepared original messages remain queued through failed transport attempts". This breaks that promise under allocation pressure.

### 10. Medium-low: persistence write errors are swallowed

- `S/🔗️backbone/🦀️.rs:228-233`: `LocalStorageBackbonePort::write` does `let _ = Box::pin(port.write(uri, payload)).await;` and then returns `Ok(())`.
- A failed host file or folder write is silently ignored after the in-memory write succeeds. The caller believes the write is durable.
- `:210-214` (read) falls back to stale memory or localStorage when the host read fails. This may be a design choice; I did not verify intent.

### 11. Low: validation gaps in registration and the registered alias

- `A/🦀️.rs:601-635` (`register_once`): no duplicate-key check within one factory's key list. `register` does this (`:583-589`, a `HashSet`); `register_once` does not, so duplicates are pushed twice into `original_entries`.
- `A/🏛️registered/🦀️.rs:47`: `RegisteredActionBusAlias::bus()` calls `expect("registered bus lease already returned")`. Any caller that uses the alias after close panics.

### 12. Low: zero-progress Pending at grant floor, and an unpaid read-owner drop

- `S/🪆️child/🧵️owner/♻️retirement/🪆️child/🦀️.rs:36` (`ArtifactChildRetirement::close_step`) and `S/🪆️child/🧵️owner/♻️retirement/🦀️.rs:13` (`ArtifactChildReadRetirement::close_step`): `if maximum_bytes < width { return Pending{0,0} }`. A caller whose grant never reaches `width` spins with zero progress and no refusal (W4).
- `S/🪆️child/🧵️owner/♻️retirement/🦀️.rs`: `ArtifactChildReadRetirement` has no `Drop` assertion, so an unfinished read owner is dropped without the paid release that `ArtifactChildRetirement` enforces (`🪆️child/🧵️owner/♻️retirement/🪆️child/🦀️.rs:43`).

### 13. Low: canonical reader panics on drop and discards the factory result

- `S/🧵️canonical-edit/📖️reader/🦀️.rs:180-191`: `Drop` panics if the reader is dropped before transfer or retirement (not while already panicking). Reachability depends on callers; I did not check them.
- `:11`: `let _ = factory.retire(root, effect, producer);`. If the factory returns without completing the reply, `reply.take()` is `None` and the poll returns `Pending{work: effect.performed_work()}`. Nothing bounds how often that repeats (W4). Unverified.

### 14. Low: quadratic inverse for a batched space-history diff

- `S/🧬️schema/📜️space-history/🦀️.rs:191-194`: `inverse` calls `undo_rows(base, &self.steps[..index])` for each step. `undo_rows` (`:113-135`) rescans `earlier` for `active()`, `length()` (two full counts), and every `find_map`. Total cost is O(n²) in the number of steps in one diff.
- `absorb` (`:210-217`) grows one diff per batched operation, so n is the batch size. Fine for small batches; quadratic for large ones.

### 15. Low: errors flattened from `Pending` in the child field drain

- `S/🪆️child/🧬️retained-clone/🦀️.rs:172` (`exit_failure`) and `:225-226`: a `Pending` from a child field close is converted into `InvariantViolated("… stayed pending on a synchronous recipient")`, and the Pending's own work is dropped (the work itself is still in the effect).
- For `String` and `ArtifactRef` leaves I believe `Pending` cannot occur, so this is probably unreachable. Listed because it turns a resumable state into a terminal failure if that assumption ever breaks.

### 16. Low: over-charge on an already-admitted backbone

- `S/🔗️backbone/♻️retirement/🦀️.rs:38`: `admit_attached` returns `(None, copied_items: 1)` when the backbone was already taken. A caller that pays `copied_items` charges one unit for nothing.

### 17. Side note (outside the five crates): inverse replay order disagrees between test support and the engine

- `S/🧰️test-support/🦀️.rs:56-60` applies `inverse` steps in listed order. The engine reverses them (`S/⚙️engine/…/📬️store-owners-batch-fold-and-history/🦀️.rs:417-418`, `back.reverse()`).
- `RemoveSpaceAlternative::inverse` returns `[SetActive(X), CreateSpaceAlternative(X)]`. In listed order the test helper applies `SetActive(X)` before X exists, and the step is refused with `missing-target`. The engine's reversed order is correct. This only affects the test helper.

## Checked and clean

- No `effect.reborrow(g, effect.maximum_work_units())` in these crates (grep). The two `maximum_work_units()` uses (`S/🪆️child/🧬️retained-clone/🦀️.rs:266`, `:284`) are guards, not ceilings.
- Progress without recorded work pays exactly one unit in the paths read (`RetirementPollExit::settle`, child `settle_outcome`, `paid_progress`). No double charge found there.
- Fuel: `consume_retained` does not debit fuel, so the `consume_fuel` calls in the wire admission and `ErasedToolJob::close_step` (`A/🦀️.rs:372`) are the single debit for those steps. I could not check that the job layer does not also charge them.
- `Pending{work:0}` loops in the child retained-clone path are not reachable with the grants read; the zero-progress returns in that path are grant-gated.
- Action bus `ErasedToolJob::close_step` maps a child `Complete` to `Pending` (`A/🦀️.rs:379-380`). This costs one extra turn, not a hang.

## Not verified

- The canonical-edit main file, `borrowed`, `borrowed-digest`, `source-identity` internals, `value/🪪️admission` close ordering beyond the `expect` at `:328` and the `assert` at `:341`.
- The space-history snapshot lift beyond the active-id decode, the create/remove checkpoint mutations, and their tests.
- `RetainedCloneClose` (`step_granted`, `begin_granted`), `close_original_effect_step`, `with_retained_loan`, and `OriginalConstructionAdmission` in the value and job crates. Several findings depend on how those behave.
- Whether the native actor ever drops its `ChannelBackboneRemote` on shutdown (finding 2), and whether any caller of `OriginalWireAdmission::close_step` runs outside a `native_receiving_identity()` window (`A/📨️wire/🌱️original/🦀️.rs:40`, which calls the file's `validate` helper first). `receiving_identity()` is `Some` only while a receipt is held (`🧰️framework/🔨️modules/🧵️job/🛬️receiving/🎟️state/🦀️.rs:36`). If the close runs after receive completes, every close step fails with "requires its initiating caller and decoder". Plausible, not confirmed.
- Action-bus tests were not read, so I cannot say whether they cover the wake, the catalogue-miss path, or the settled-construction cap.

## Top finding

Finding 1: a channel backbone whose actor has installed its wake can never finish retiring (`S/🔗️backbone/♻️retirement/🦀️.rs:69` refuses the wake, `:101` calls `demands()` before the wake branch, `:144` moves the wake in), and production installs that wake at `S/🔄️sync/🖥️native/🎭️actor/⏲️actor-runner-scheduling-terminal-and-spawn/🦀️.rs:386`.
