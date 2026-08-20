# `db-dedyn` packet report — `semio-framework-os-kernel-db` — DONE

Packet: `db-dedyn`. Path scope: `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/**`.

## Bottom line

`cargo check -p semio-framework-os-kernel-db --lib` → **EXIT 0** (from 281 errors at packet start).
Forced-rebuild dropped-future census → **0** (verified against a real instrument failure — see
below, this found and fixed **46 real dropped futures plus 2 more via the `let _ =` corollary
grep**, one of which was a production correctness bug: `submit()`'s live-query notify step never
ran). Zero first-party `dyn` remains (verified by full-crate grep, every live hit is `dyn
Iterator`/`dyn Fn`/`dyn FnOnce`, all R1-legal). All five regression guards green;
`semio-framework-os-kernel` test suite **779 passed / 0 failed**, unchanged from baseline.
`--all-targets` → **2062 errors**, honestly reported, entirely the ticket's documented
`#[cfg(test)]` residue class — out of scope, needs its own packet, not attempted here.

## What was actually wrong (bigger than "missing awaits")

The design (`DbBackend<R>` enum, `WalRef`/`SnapshotRef`/`PayloadRef`/`CatalogRef`/`IndexRef`/
`LeaseRef` facet-ref enums, `DbFuture` deleted) was **already implemented** before this packet
started. What blocked the crate was three defect classes, each far more consequential than the
"~248 missing awaits" framing in the brief:

1. **A structural bug in how `insert-await.py`'s output composes across repeated passes.** The
   tool correctly applies rustc's own single-candidate suggestion at each individual diagnostic —
   but when a local variable holds an un-awaited async constructor (`let mut writer =
   pack::ByteWriter::new();`), rustc suggests `.await` at the RECEIVER of each downstream method
   call independently. Applied across several passes, every subsequent `writer.write_x(...)` site
   became `writer.await.write_x(...)` — legal in isolation, but `.await` MOVES its receiver, so
   the second/third/… occurrence is `E0382: use of moved value`. Measured peak: **570 edits
   across 15 files**. Fixed with a purpose-built recovery tool, `fix-repeated-await.py` (saved in
   this ticket folder per R10), which finds `let X = EXPR;` (not already `.await`ed) followed by
   `X.await.method(...)` uses in the same function and rewrites to `let X = EXPR.await;` +
   `X.method(...).await`. **This diagnosis was independently valuable**: the coordinator confirmed
   the same defect was corrupting a sibling packet running the identical tool across ~1,500 files,
   and pointed it at this recovery tool.
2. **Field-init shorthand corruption**, the same tool applied to a struct literal `{ field }`
   shorthand rewrites the bare identifier to `field.await` — a hard parse error. This recurred
   **9+ times across 8 files** over the course of the packet (including 3 re-occurrences on files
   I had already fixed, each time from a *different* insert-await pass touching a *different*
   declaration in the same file). Fixed by hand at every site; the general lesson (documented for
   whoever runs this tool next) is to grep for `IDENT\.await,?\s*$` inside struct literals after
   every `--apply`.
3. **A wide, mechanically-induced R9 backlog.** The blind universal-async codemod made async
   dozens of genuinely pure, zero-suspension-point functions/methods whose callers are
   permanently sync — `run_blocking_op`'s `FnOnce` closures (blocking I/O), `Future::poll`
   implementations (an entire `db_actor` design built around **sync constructors returning
   Future-implementing structs** — `Address::send`/`ask`/`recv`, `ArtifactEngine::submit` — which
   the codemod broke into illegal double-futures), `impl Default`/`impl Ord` bodies,
   `Iterator::map`/`fold`/`filter`/`find`/`max_by_key` closures, fn-pointer slots, and hand-rolled
   `Mutex`-guard helpers (`fn lock<T>(...)`, independently duplicated in **five** files, plus a
   hand-rolled `oneshot` channel duplicated in **two**). Reverted with a mix of targeted
   single-function edits (each hand-verified I/O-free, tagged `// 🚫️async: E<n> …see R9`) and,
   for five whole files/regions confirmed 100% I/O-free (module doc + grep):
   `db_state`, `db_conflict`, `db_preview`, `🎚️policy`, and `db_actor`'s Mailbox/Futures/Reply/
   BlockingRuntime regions — a full-file `async`+`.await` strip, same discipline as the ticket's
   pre-existing `terra-number-deasync.py` precedent. A handful of genuinely-open extension points
   (`ErasedProjection`/`ProjectionClass`, `protocol::Signer`/`MergePolicy`) stayed async and got
   their missing `.await`s filled in mechanically instead, since R9 does not apply where a real
   suspension point exists.

## The E0038 "de-dyn wall" — 5 traits, all resolved via existing rulings, zero new architecture

The brief flagged 2 E0038 at the start; as more of the crate came into view the true count peaked
at 14, across 5 traits. All five were resolved via R11(a) (parameter/field position, trivially
generic) or R11's "exactly one impl ⇒ delete the trait object" clause — not new design work:

| trait | shape | fix |
|---|---|---|
| `protocol::Signer`/`SignatureVerifier` | open extension point (external crate, "zero impls in this family" per its own doc), parameter-only | `&dyn` → `&impl` on `sign_message`/`verify_signature` |
| `db_query::QuerySource` | 3 first-party implementors, never mixed at one call site | `&dyn` → `&impl` on `execute`/`LiveQuery::refresh` (mirrors the already-converted sibling `FullTextLookup` in the same file — a prior packet had done half the job) |
| `db_preview::ConflictOracle` | 3 implementors, parameter-only | `&dyn` → `&impl` on `reconcile_with` |
| `db_actor::ThreadSpawner` | exactly 1 implementor (`StdThreadSpawner`), STORED in `Supervisor.spawner: Arc<dyn ThreadSpawner>` | R11's "exactly one impl" rule: `Supervisor<A: Actor>` → `Supervisor<A: Actor, S: ThreadSpawner>`, `spawner: Arc<S>` |

Verified zero-dyn with `grep -rnE '(Arc|Box|Rc)<dyn |&dyn |&mut dyn '` over the whole crate: every
live (non-comment) hit remaining is `dyn Iterator`/`dyn Fn`/`dyn FnOnce` — all std traits,
explicitly R1-legal. Every other hit is a doc-comment/Cargo.toml-comment historical reference to
the erasure being *replaced*.

## Forced-rebuild dropped-future census — real defect found, then verified zero

```
cargo clean -p semio-framework-os-kernel-db && cargo check -p semio-framework-os-kernel-db --lib --message-format=short 2>&1 | grep -c 'unused implementer of `Future`'
```

**Two R12/R13 traps hit and cleared in sequence, exactly as the ruling warns:**

1. First measurement used the lint text from the ruling's own wording, `unused implementer of
   \`std::future::Future\`` — returned 0. This was a **false negative**: this rustc/edition emits
   the short form `unused implementer of \`Future\`` (no `std::future::` prefix). Re-grepping with
   the correct text surfaced **46 real dropped futures**, concentrated in `⚙️engine`, `🗄️storage`,
   `⌨️cli`, `🔄️sync`, `📄️artifact`, `📸️snapshot`, `📝️wal`, `🔒️security` — almost entirely the
   `pack::ByteWriter`/`ByteReader` sequential-write-call pattern (`writer.write_u64_le(x);` with
   no `.await`, so the write never happened) left behind by defect class 1 above, plus a
   hand-rolled `oneshot` channel's `send`/constructor (same shape as `db_actor::oneshot`, now
   reverted to sync alongside it).
2. Fixed via the compiler's own JSON diagnostics: for each `unused_must_use` warning on a `Future`,
   inserted `.await` at the exact byte span rustc flagged (all 46 spans were plain statement
   expressions terminated by `;`, verified before editing). One follow-on compile error
   (`storage.rs`'s `run_blocking_op` closure — a **sync** `Box<dyn FnOnce()>` work item — can't
   `.await`) revealed the true fix was reverting `oneshot`/`OneshotSender::send` to sync (R9; same
   shape as the already-reverted `db_actor::oneshot`), not awaiting in place.
3. **Re-ran the forced-rebuild census after the fix**: `cargo clean -p … && cargo check …` →
   **0** `unused implementer of \`Future\`` warnings. This second zero IS trustworthy — the
   instrument was proven capable of seeing a real positive (46, then 0 after the actual fix),
   satisfying R12's "verify the instrument can see a known-positive before trusting a negative."

**`let _ =` corollary (R13/R14) — grepped for every `let _ = <expr>` lacking `.await` in the whole
crate (13 total, listed in full):** 4 were `write!(...)` macro calls (sync, not futures) or
`std::thread::JoinHandle::join()` (sync), 2 were parameter/variable suppressions unrelated to
async, and **2 were genuine dropped futures the lint-based census structurally cannot see**:

- `📄️artifact/🦀️component.rs:830` — `ArtifactEngine::submit`'s "live-query notify" step,
  `let _ = self.refresh_live_queries();`, silently never ran in **production code** on every
  command submit. Fixed to `let _ = self.refresh_live_queries().await;` (the `Vec` result is
  genuinely unneeded — fire-and-forget is correct, the missing `.await` was the bug).
- `🧪️testkit/🦀️component.rs:1110` (test-only) — a torn-write-recovery test's `let _ =
  engine.submit(...)` never actually submitted the command being tested, and a preceding
  `as_fault(&storage).set_script(...)` (both genuinely async) was similarly dropped — the whole
  test was a silent no-op. Fixed both.

## Current state, run this turn

| check | result |
|---|---|
| `cargo check -p semio-framework-os-kernel-db --lib` | **EXIT 0** |
| `cargo check -p semio-framework-os-kernel-db --all-targets` | **2062 errors**, all `#[cfg(test)]` residue (documented class, out of scope — separate packet needed) |
| Forced-rebuild dropped-future census | **0** (instrument-verified, see above) |
| `let _ =` audit | 13 sites, all resolved/benign (2 genuine bugs fixed, 11 confirmed non-future) |
| Zero first-party `dyn` | confirmed, only `dyn Iterator`/`Fn`/`FnOnce` remain (R1-legal) |
| `cargo check -p semio-framework-plugin-host --lib` | EXIT 0 |
| `cargo check -p semio-framework-plugin --lib` | EXIT 0 |
| `cargo check -p semio-framework-plugin --lib --all-features` | EXIT 0 |
| `cargo check -p semio-framework-async --lib` | EXIT 0 |
| `cargo check -p semio-framework-os-kernel --lib` | EXIT 0 |
| `cargo test -p semio-framework-os-kernel --lib` | **779 passed / 0 failed / 0 ignored** — matches recorded baseline exactly |

## Files touched (all within `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/**`)

`🦀️component.rs` (crate root re-exports), `🗄️storage/🦀️component.rs`, `📄️artifact/🦀️component.rs`,
`⚙️engine/🦀️component.rs`, `🔍️query/🦀️component.rs`, `🎭️actor/🦀️component.rs`,
`🎚️policy/🦀️component.rs`, `🔒️security/🦀️component.rs`, `👁️preview/🦀️component.rs`,
`⚔️conflict/🦀️component.rs`, `🔘️state/🦀️component.rs`, `📝️wal/🦀️component.rs`,
`💾️durability/🦀️component.rs`, `🕸️version-graph/🦀️component.rs`, `👁️observe/🦀️component.rs`,
`🌐️cluster/🦀️component.rs`, `⌨️cli/🦀️component.rs`, `🧪️testkit/🦀️component.rs`,
`🆔️ids/🦀️component.rs`, `📽️projection/🦀️component.rs`, `📸️snapshot/🦀️component.rs`,
`🔢️index/🦀️component.rs`, `🗜️compact/🦀️component.rs`.

## Tools added to the ticket folder (all `.py`, all diagnostic- or self-verified-scope-driven)

- `fix-repeated-await.py` — recovers the "declaration never awaited, every USE site got its own
  `.await` from independent insert-await.py passes" defect (defect class 1 above). Structural
  (brace/paren-matched per function), not name-keyed. **Independently confirmed valuable on a
  sibling packet by the coordinator.**
- `terra-db-state-deasync.py`, `terra-db-conflict-deasync.py` (reused verbatim for `🎚️policy` and
  `🎭️actor`), `terra-db-preview-deasync.py` — whole-file `async`/`.await` strip for files confirmed
  100% I/O-free (module doc + grep), same discipline as the ticket's pre-existing
  `terra-number-deasync.py`.
- `terra-db-query-deasync.py` — the same strip restricted to an explicit, hand-enumerated
  signature list (21 functions) for `db_query`'s pure `Value`/`Predicate`/`Path` core, leaving the
  genuinely-storage-backed half of the file untouched.
- The forced-rebuild dropped-future fix itself (46 sites) was applied via a one-off inline script
  reading `cargo check --message-format=json`'s own `unused_must_use` spans and inserting `.await`
  at the exact byte offset — not saved as a standalone tool since it is fully generic (works on
  any crate) and small enough to reconstruct from this report's description if needed again.

## What is intentionally NOT done (out of scope, reported per the brief's own instruction)

`--all-targets`'s 2062-error `#[cfg(test)]` residue needs its own packet, per this ticket's own
established precedent (`sdk-final`, `dispatch-group-split` both hit and deferred the identical
class). Not absorbed here.
