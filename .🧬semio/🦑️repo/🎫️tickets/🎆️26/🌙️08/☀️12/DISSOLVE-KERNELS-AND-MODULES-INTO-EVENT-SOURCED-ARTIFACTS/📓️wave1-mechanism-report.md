# W1 — Mechanism report

Executed by the coordinator directly (the dispatched Sonnet agent was killed by a session usage limit at its baseline step, having landed zero edits — verified, see "Agent-loss verification" below).

## What changed

One file: `🧰️framework/🛍️products/💻️os/🔨️modules/⚙️engine/🦀️component.rs` (crate `semio-framework-os-kernel`, mounted at `💻️os/📦️packages/🦀️rust/📦️glue.rs:257`).

### 1. `//#region 🔖️EngineRep` — the doctrine tier-(d) marker

```rust
pub trait EngineRep<P>: Sized {
    fn build(snapshot: &P) -> Self;
}
```

Docstring states the contract: built only inside a `🔺️diff` constructor or an `InferredField::{plan,dep_input,compute}` body; dropped when that function returns; never a durable struct field, never `thread_local!`, never carried across a mutation-dispatch boundary; deterministic; wholly derived.

**`build` is deliberately the only constructor.** No incremental or seeded variant exists, because a representation grown from a previous representation is no longer recoverable from the snapshot — which is exactly how a cache becomes hidden authoritative state. The docstring names the "warm rebuild during a continuous gesture" optimisation as the specific thing this forbids, and points at `DraftEngineSession` as the sanctioned way to avoid the rebuild instead of seeding one.

### 2. `//#region 🔖️DraftEngineSession` — the one sanctioned tier-(d) cache

Holds exactly one `EngineRep` for the span of a continuous draft gesture (fillet-radius drag, gumball), so the representation rebuilds once per distinct base rather than once per draft mutation.

```rust
pub struct DraftBaseHash([u8; 32]);            // minted only by of_bytes()
pub struct DraftEngineSessionStats { pub reuses: u64, pub rebuilds: u64 }
pub struct DraftEngineSession<P, R: EngineRep<P>> { … }
    pub fn new() / rep(&mut self, base: &P, base_hash: DraftBaseHash) -> &R / clear() / stats()
```

### How the type enforces the invariant

The binding invariant, negotiated with the ARTIFACTS-ONLY-PLUGIN-ARCHITECTURE session and quoted in the docstring:

> Drop the session at any instant and rebuild from the draft base; if a user would notice anything other than a pause, the session is holding state it shouldn't.

APA subsequently sharpened it: content-hashing the base makes *invalidation* correct, but the invariant is that nothing is *unrecoverable by rebuild*, and those come apart the moment a `Rep` is constructed from something other than the base. Answering their three questions directly:

1. **Can a caller obtain a `Rep` not built purely from a base?** No. `EngineRep::build(&P)` is the trait's only method and the only call site is inside `rep()`. There is no incremental constructor, no `From<PreviousRep>`, no way to insert or replace a held `Rep` from outside.
2. **Can a caller mutate a cached `Rep` in place?** No. `rep()` returns `&R`; there is no `&mut R` accessor anywhere on the type.
3. **Can any user-supplied value be stored?** No. The struct has three fields — `Option<(DraftBaseHash, R)>`, the stats counters, and a `PhantomData`. There is no field a tolerance, selection, or pending edit could occupy.

Two supporting choices: `DraftBaseHash` is a newtype minted only by `of_bytes` (blake3 over the encoded base), so it cannot be confused with a generation counter or an arbitrary integer — a stale-but-equal number would silently defeat invalidation. And a hash mismatch **rebuilds rather than patches**, so a stale representation cannot survive a base change.

**Deliberate rejection, recorded per instruction:** no warm/incremental rebuild fast path. Its cost is a full `build()` on every base change during a gesture. That is the price of the invariant holding by construction rather than by discipline, and the standard adopted across sessions today is *unwritable beats discouraged*.

### 3. `EngineCache` scope-narrowing

Behaviour **unchanged** — other sessions depend on it. Added a docstring recording the narrowed contract: it survives only at the wasm guest↔host boundary (`engine-derive`/`engine-read`), is no longer a general kernel cache, and points derived values at `💡️inference`/`DepHash` and ephemeral representations at `EngineRep`. Names `policyEngineCacheScopeBreaches` as the W6 enforcement.

### 4. Tests added (6, all in the file's existing `mod tests`)

| Test | Law |
|---|---|
| `engine_rep_build_is_deterministic` | `build(s) == build(s)` |
| `draft_base_hash_tracks_content_not_length` | equal-length different-content bases hash differently |
| `draft_session_rep_equals_cold_build` | **transparency**: a reused rep equals a cold rebuild |
| `draft_session_reuses_while_base_unchanged` | one gesture over one base ⇒ exactly 1 rebuild, 4 reuses |
| `draft_session_rebuilds_when_base_changes` | base change ⇒ rebuild, never a stale hit |
| `dropping_the_session_at_any_instant_costs_only_a_rebuild` | **APA's invariant, asserted directly**: clear mid-gesture, rebuild, get a byte-identical rep, cost exactly one extra rebuild |

## Verification — with an explicit honesty caveat

Real commands, real output, run at ~17:30–17:36 local:

```
CARGO_TARGET_DIR=<ticket>/🎯️target cargo check -p semio-framework-os-kernel
  → BASELINE (pre-edit): Finished dev profile; 0 errors, 49 warnings
  → POST-EDIT: 0 errors, 49 warnings (unchanged — no regression)

CARGO_TARGET_DIR=<ticket>/🎯️target cargo test -p semio-framework-os-kernel --lib engine
  → running 11 tests … test result: ok. 11 passed; 0 failed; 0 ignored; 818 filtered out
```

**What this does and does not cover — stated plainly rather than rounded up:**

- It covers the 11 tests in the `os_engine` module, which include all 6 new laws. The lib **test binary compiled and linked**, so the crate's test targets were genuinely reached, not skipped.
- It does **not** cover the crate's full lib suite. The 818 other tests were filtered out, not run. I have not verified them and do not claim them.
- An attempt to run the full lib suite immediately afterwards **failed for reasons outside this work**: first a transient workspace-manifest break (`✏️s/🔌️plugins/🖍️draw/🔄️fsm` was mid-move by another session; since resolved and confirmed by them), and then a machine-wide disk-full condition (`/System/Volumes/Data` at 100%, 1.2 GiB free, repo-root `target/` alone at 428 G).
- **Consequence: the two results above were obtained before the disk filled, and cannot currently be re-run.** They should be re-verified once the disk is resolved. Per the standard adopted across sessions today — *a green result only covers the targets the run actually reached, and `cargo check` is not a verification gate* — I am recording these as "green when measured, pending re-confirmation" rather than as a settled gate.

## Agent-loss verification (why this report has no partial edits behind it)

Four dispatched agents died simultaneously on a session usage limit. Two had reported reaching only their baseline step. Rather than assume, I verified zero edits landed:

- `stat -f '%Sm'` on both boundary files: `⚙️engine` **Aug 11 00:50**, `🖥️platform` **Aug 10 20:25** — both predating this session entirely.
- `grep -c "EngineRep\|DraftEngineSession"` on `⚙️engine` → **0**.
- `grep -c "fn set_"` on `🖥️platform` → **4** (unchanged).
- Ticket folder contained only the coordinator's own four files; no agent scratch or reports.

The bounded-rounds structure worked as designed: agents died at baseline, before any write, leaving nothing half-applied.

## sharedFileRequests

None. This wave touched exactly one file, claimed by this ticket, in no other session's table.

## Concurrent-churn observations

1. **Transient whole-workspace manifest break.** `✏️s/🔌️plugins/🖍️draw/🔄️fsm/` briefly vanished while root `Cargo.toml:66-67` and `✏️s/🔌️plugins/🖍️draw/📦️packages/🦀️rust/Cargo.toml:27` still referenced it, so cargo refused to load the workspace at all — no command worked repo-wide, not even `check -p`. Diagnosed, reported to the owning session, **not touched**. They confirmed it was an agent relocating `🔄️fsm`, discovering it is a separate crate with two workspace-member entries, and reverting; `🔄️fsm` stays in place. Generalisable lesson they've adopted into their packet rules: *moving a directory containing a `Cargo.toml` breaks the workspace for every session on the machine — a dangling `#[path]` mount is a local error, a dangling workspace member is a global one.*
2. **Disk full, machine-wide.** 100% capacity, 1.2 GiB free; root `target/` = 428 G, last modified 17.5 h ago with **zero files touched in the previous 2 hours** (i.e. genuinely stale — nothing writes to it, since repo policy is a per-ticket `CARGO_TARGET_DIR`). Escalated to the user rather than deleted; being handled externally. **All cargo evidence repo-wide in this window is untrustworthy.**
3. **A peer's `tempfile` finding appears stale.** A report of 144 errors in `🏪️store/🔄️sync/🦀️component.rs` from `tempfile` not being a dev-dependency does not match the tree: `💻️os/📦️packages/🦀️rust/Cargo.toml:61` carries `tempfile = "3.20.0"`, with a comment at :58 naming that exact test module. Corroborated by this wave's run, whose lib test binary linked successfully — impossible if that module carried 144 errors. Reported back for re-measurement; not acted on.
4. **`semio-framework-plugin` is red repo-wide** (E0499 `self.children`; E0560/E0609 from the `document`→`artifact` field rename reaching definitions but not two `#[cfg(test)]` call sites). Not ours, not touched. Ownership was initially misattributed by two peers as orphaned debt from a closed ticket; mtime evidence (`🛂️manifest` Aug 12 03:50 vs `🔌️plugin` Aug 12 17:33) showed it is one session's rename mid-propagation, and both peers retracted. This did not block W1, because `⚙️engine` lives in `semio-framework-os-kernel`, not the plugin SDK — but it will block the brep/mesh/2d lanes.

## Result

**PASS, with the verification caveat above.** The mechanism the later lanes migrate onto exists, is shaped so the tier-(d) invariant holds by construction, and its laws are asserted by tests that were observed passing. Re-run the two commands once the disk is clear before treating this as a settled gate.
