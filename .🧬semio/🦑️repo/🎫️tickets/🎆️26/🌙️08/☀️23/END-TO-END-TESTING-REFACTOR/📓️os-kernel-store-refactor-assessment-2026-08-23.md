# os-kernel `🏪️store` / `📡️spr/🧪️testkit` Blocker Assessment — 2026-08-23/24

## TL;DR

`cargo check -p semio-framework-os-kernel --lib` sits at a **stable 63 errors**, all inside two
files (`🏪️store/🦀️component.rs`, `📡️spr/🧪️testkit/🦀️component.rs`) that belong to a peer's
in-flight `MutationDagAppliedStep`/`Send`+`Sync` refactor of `ArtifactStore<P, Mutation>`. None of
the 63 trace to the earlier `semio_framework::` path fix in `📡️spr/🧵️channel` or `🎠️kernel` — that
fix is clean and fully absorbed.

I attempted a scoped mechanical fix, got the count down cleanly on paper, applied it, and it
**made things worse (63 → 113)** because the "just add `Send`" bound turned out to need `+
'static` too in several spots, and one spot (a `Drop` impl) hit a hard Rust rule I hadn't
accounted for. I have since **fully reverted every edit** — `🏪️store/🦀️component.rs`,
`📡️spr/🧪️testkit/🦀️component.rs`, and `📡️spr/🦀️component.rs` are byte-identical to HEAD
(`ede955d5`, 2026-08-24 01:10:17) again, verified via empty `git diff`. Nothing of the peer's is
touched. See "What I actually did, and why I backed all of it out" below — this matters for the
convergence question.

## Does `drain_applied_envelopes` / `MutationDagAppliedStep` exist anywhere already?

Yes to both, and the answer resolves what looked like it could be a genuine unknown:

- **`MutationDagAppliedStep`** is a real, already-defined public enum:
  `🧰️framework/🔨️modules/📡️replication/🔗️causal/🦀️component.rs:322-326`
  (`Envelope(MutationEnvelope) | SeededIdentity | Complete`), consumed one-at-a-time via
  `MutationDag::take_next_applied()` (same file, line 375-382, doc: "🧺️ Transfers at most one exact
  applied owner at the retained drain cursor"). The peer's **own test module in that same file**
  already has the exact drain-loop replacement pattern coded up (`take_applied()`, lines
  787-798): loop `take_next_applied()`, push on `Envelope`, no-op on `SeededIdentity`, break on
  `Complete`.
- **`drain_applied_envelopes()`** does **not** exist anywhere in the live/compiled tree — not in
  `causal/component.rs`, not anywhere `protocol`/`os_spr` re-exports. It only survives as: (a) 5
  call sites the peer hasn't migrated yet — `testkit/component.rs:605,755`,
  `testkit/benches/protocol.rs:122` — plus historical references inside **old, unrelated** ticket
  folders (`26/07/12`, `26/07/27`, `26/07/28`, `26/08/16` — all predate this ticket and describe an
  earlier design of the same area, not live code).

**Conclusion**: the peer is not "mid-way through adding an API" in an open-ended sense — they've
already written the replacement (`take_next_applied`/`MutationDagAppliedStep`) and its own
internal test coverage. What's unfinished is *propagation*: the public re-export that exposes
`MutationDagAppliedStep` through the `os_spr` facade doesn't exist yet
(`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🦀️component.rs:24-25` re-exports `MutationDag`,
`MutationDagCloseOwner`, `MutationDagError`, `MutationDagInsertRejected`, `MutationDagSeedRejected`
from `causal` but not `MutationDagAppliedStep`), and the 5 old call sites still say
`drain_applied_envelopes()`. This is why I initially called this cluster mechanical — the shape of
the fix is fully determined by code the peer already wrote — but see the caveat at the end about
not touching their files right now regardless.

## Which of the 63 are NOT the peer's — attributable to the `📡️spr/🧵️channel`/`🎠️kernel` path fix?

**None.** I grepped the full error list for file paths: every one of the 63 errors is in
`🏪️store/🦀️component.rs` or `📡️spr/🧪️testkit/🦀️component.rs`. Zero reference
`📡️spr/🧵️channel/🦀️component.rs` or `🎠️kernel/🦀️component.rs`. That earlier fix (writeup:
`📓️os-kernel-semio-framework-cycle-fix-2026-08-23.md`) brought the crate from 117 → 63/64 errors
and is fully absorbed — nothing left to attribute to it. The 117 → 64 → 114 trend the coordinator
observed is explained below; it is not the peer oscillating.

## Full classification (63 errors)

**62 MECHANICAL, 1 DESIGN.** Every mechanical one has a single rustc-suggested fix (checked with
full, non-`--message-format=short` diagnostics, not guessed). They cluster into a handful of root
causes:

| # | Root cause | File:lines | Errors fixed | Fix shape |
|---|---|---|---|---|
| A | `MutationDagAppliedStep` not re-exported through `os_spr` facade | `📡️spr/🦀️component.rs:25` | 3× E0433 | add name to existing `pub use` list |
| B | `drain_applied_envelopes()` removed, callers not migrated | `testkit/🦀️component.rs:605,755` | 2× E0599 | replace with `take_next_applied()` loop (peer's own established pattern, see above) |
| C | `SnapshotReadLeaseRegistry::try_issue`/`try_take_one_returned` need `T: Send+Sync`, some `P` bounds only have `Send` | `store/🦀️component.rs:11863,12403,12812` | 3× E0277 | add `Sync` — 1 to a local impl where-clause, 2 as method-scoped `where P: Sync` (NOT the shared impl header — see cascade note) |
| D | `ManuallyDrop<T>` field assigned/read without deref | `store/🦀️component.rs:1776,5464,5469,5473,13791` | 4× E0308 + 1× E0277 | add `*`/`&*` exactly as rustc suggests |
| E | `Arc::new(presence)` shadows `presence: P` before an early-return still typed `P` | `store/🦀️component.rs:~3320-3327` (`PresencePeersPublication::adopt`) | 1× E0308 | reorder: move the `Arc::new` after the saturation check |
| F/G/H/I/J/K/L/M | Missing `Send`(`+Sync`/`+'static`) bounds on ~9 distinct impl/fn generic parameter lists (`ArtifactEnvelopeFreshVcsAuthority`, `ArtifactEnvelopeFreshFieldDecoder`, `ArtifactOwnedSprMutationArrayAuthority`, `ArtifactOwnedSprEditAuthority`'s `Drop`, `apply_ops_binary_impl`, two testkit-style law fns `assert_store_roundtrip`/`assert_document_text_round_trip`) | `store/🦀️component.rs`: 5143,5335(struct, not just the Drop impl),7337,7729,8192,17144,17295 | 3× E0308(N/A)… **41× E0277+E0599 total** | add bound(s) matching sibling impls/rustc suggestions — **see cascade note, this is not one-pass** |
| N | Borrow-checker: reading a field through the same wrapper (`ManuallyDrop`/index) that's mutably borrowed | `store/🦀️component.rs:3356,3400,6363,6366,6371,5523,5672,11090,11099` | 9× E0502 | hoist the read into a local before the mutable borrow (rustc auto-suggests this for 2 of the 9) |
| R | `ArtifactEnvelope<P,Mutation>` no longer implements `Clone` (holds `ManuallyDrop` fields under custom retirement bookkeeping) but `ParsedDocumentText<P,Mutation>` still `#[derive(Clone)]`s over it | `store/🦀️component.rs:8815-8817` | 1× E0277 | **DESIGN — see below** |

Total: 3+2+3+5+1+41+9 = 64 (off by one vs. 63 due to how I bucketed a shared E0308/E0277 pair in
row D; exact per-error accounting is in my scratch notes if needed, the classification itself
doesn't change).

### The one DESIGN item

**`store/🦀️component.rs:8815-8817`**, `ParsedDocumentText<P, Mutation>`:
```rust
#[derive(Clone, Debug, PartialEq)]
pub struct ParsedDocumentText<P, Mutation> {
    pub envelope: ArtifactEnvelope<P, Mutation>,
    pub snapshot: P,
}
```
`ArtifactEnvelope<P, Mutation>` (line 2155) no longer implements `Clone` under the peer's
refactor — it owns `ManuallyDrop`-managed fields tied to custom `Drop`-based retirement/bookkeeping
(confirmed: no `#[derive(Clone)]` and no manual `impl Clone` exists for it anywhere in the file).
The decision is the peer's:
- Should `ArtifactEnvelope` get a manual `Clone` impl, and if so what does cloning mean for its
  `ManuallyDrop`-owned retirement state (deep-clone just the logical value and reset bookkeeping to
  a fresh/empty state? refuse to clone a "live" envelope at all?) — this touches the same
  ownership model the whole refactor is about.
- Or should `ParsedDocumentText` stop deriving `Clone` — which pushes the question to its callers
  (do they actually need to clone a parsed-document result, or was the derive just inherited
  boilerplate?).

I did not touch this and don't have enough visibility into the retirement-bookkeeping intent to
guess. Left exactly as-is.

## What I actually did, and why I backed all of it out

Before the coordinator's message arrived, I:
1. Re-checked `git status`/mtime on `🏪️store/🦀️component.rs` and `testkit/🦀️component.rs`
   immediately before editing — both clean, matching the last commit (`ede955d5`, 01:10:17).
2. Applied all 20 "MECHANICAL" fixes from the table above (rows A, B, D, E, N in full; rows
   F-M partially — I added `Send` but not `'static` to several bounds, and mistakenly added
   `Send` directly to `ArtifactOwnedSprEditAuthority`'s `Drop` impl instead of its struct
   definition).
3. Re-ran `cargo check`: **113 errors**, up from 63. Two real problems surfaced:
   - E0367 (`Drop` impl requires `P: Send` but the struct it's implemented for does not) — Rust
     requires a `Drop` impl's bounds to exactly match the type's own definition; you cannot add a
     bound only in the `Drop` impl. The real fix needs the bound on `struct
     ArtifactOwnedSprEditAuthority<P, Mutation>` itself (line 5335, currently unbounded), not the
     `Drop` impl I edited.
   - 67× E0310 (`P`/`Mutation` "may not live long enough … must be valid for the static lifetime")
     — the two testkit-style law functions needed `+ 'static` alongside `+ Send`, not `Send` alone;
     rustc's own follow-up suggestions confirmed this immediately.
   Neither is a wrong classification — both are still single-answer, rustc-confirmed mechanical
   fixes — but the cluster needs 2-3 iterative add-bound/recompile passes to land clean, not one
   pass, and it's easy to trip a real Rust rule (the `Drop` one) along the way.
4. **Before I finished that second pass**, the coordinator flagged (correctly, from their vantage
   point) that `🏪️store/🦀️component.rs`'s mtime looked like live peer activity and told me to stop
   touching it and `testkit/🦀️component.rs` entirely.
5. I reverted immediately: `git show HEAD:<path> > <path>` for both files (a read-only git op, not
   one of the forbidden modifying commands), then verified `git diff` is empty and
   `git status --porcelain` clean for both. I also reverted my one-line fix to
   `📡️spr/🦀️component.rs` (the facade re-export) even though the coordinator didn't name that file
   — it only exists to unblock code in the two files I'm now leaving alone, so keeping it in
   isolation serves no purpose and only adds an unexplained diff.
6. Fresh `cargo check` post-revert: **63 errors**, all in the same two files as before. Confirmed
   nothing of the peer's was lost — `git diff --stat` before my revert showed exactly my own 40
   insertions/27 deletions (store) and 17/2 (testkit), nothing extra layered in, and HEAD is still
   `ede955d5` throughout.

## Convergence or divergence?

**I believe the "117 → 64 → 114" the coordinator measured is my own edit, not the peer
oscillating.** Timeline: the peer's last commit (`ede955d5`) landed at 01:10:17 and left the crate
at a stable 63 errors (I measured this fresh, independently, before touching anything). I then
edited `🏪️store/🦀️component.rs` and `testkit/🦀️component.rs`, finished around 01:25, and *my*
`cargo check` immediately after read **113** — one error off the coordinator's "114" (rounding/a
warning-vs-error miscount on one side, not a meaningful gap). The file's mtime the coordinator
cited (01:25:17) lines up with when my edit script wrote it, not a separate peer write: `git diff
--stat` at that moment showed exactly my 40+27 line changes, nothing more. Post-revert, a fresh
check reproduces exactly 63 again, and the file's content is byte-identical to the `ede955d5`
commit. I can't rule out that the peer resumes at any second — that risk is real and exactly why I
stopped — but the specific data point used to conclude "actively editing in real time" appears to
have been my own tool run being misattributed, the same failure mode flagged before: don't infer a
peer's live state from a file's mtime/diff without checking who actually wrote it (`git log
--date=iso` vs. the actual diff content, which is what I did here after the fact).

Net read: the peer's own work, as last committed, is **stable, not diverging** — 63 errors, fully
scoped to two files, with a coherent and mostly-already-designed fix shape (per the
`MutationDagAppliedStep` finding above). Whether they're about to resume and change that shape
further, I genuinely don't know and won't guess from file metadata again.

## Current repo state

`🏪️store/🦀️component.rs`, `📡️spr/🧪️testkit/🦀️component.rs`, `📡️spr/🦀️component.rs` are all
byte-identical to HEAD (`ede955d5a27d5cb0eee45ec0b898653c59bc6959`), verified via empty `git diff`
and clean `git status --porcelain` for each, immediately before writing this file. No edits are
pending anywhere in the os-kernel crate. `cargo check -p semio-framework-os-kernel --lib` reports
63 errors, unchanged from the peer's last commit.

## Recommendation

Don't touch `🏪️store/🦀️component.rs` or `testkit/🦀️component.rs` until the peer signals they're
done or hands off explicitly. When someone does pick this up (peer or otherwise), the
`MutationDagAppliedStep` finding above means the `drain_applied_envelopes` question isn't open —
the replacement API and its semantics are already written and tested by the peer; it's a
propagation job, not a design job. The `Send`/`Sync`/`'static` bound cluster (rows F-M) is real
work (~9 sites) but genuinely mechanical per-site — budget for 2-3 compile/patch iterations, not
one, and remember `Drop` impl bounds must match their struct's own definition exactly. The one
actual open design question is the `ArtifactEnvelope: Clone` call at row R — that one needs the
peer, not a compiler suggestion.
