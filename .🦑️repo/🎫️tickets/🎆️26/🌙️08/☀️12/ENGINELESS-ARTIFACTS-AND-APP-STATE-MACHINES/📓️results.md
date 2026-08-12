# Results — engineless artifacts and app state machines

## The thesis, and what it cost to establish

**An artifact is a `🧬️schema` (snapshot, diff, mutations, inferences) plus a `🚪️io` system — never an engine.** Behaviour belongs to the app that edits the artifact; pure algorithms belong one level up in a module. Only **artifact** engines were abolished — `appChildDirs` still *requires* `⚙️engine`, and `taxonomyLeafParentDirs` keeps module engines globally legal.

The trait the whole directory class existed to serve **never shipped**: `grep -rn "trait ArtifactEngine"` → **0** repo-wide, `impl … ArtifactEngine for …` → **0**, no string or runtime registry names those types, and every sampled `*Engine` struct had **zero construction sites**. 95 directories and ~153k LOC were built around a placeholder.

## The taxonomy now enforces the architecture

Confirmed on disk after IIF's `💡️inferences` flip landed — the four keys F1 changed survived it untouched:

```
schemaChildDirs        = ['📸️snapshot', '🔺️diff', '🧬️mutations', '💡️inferences']
artifactChildDirs      = ['🧬️schema', '🚪️io', '📚️examples']      ← no engine
subsetChildDirs        = ['🧬️schema', '🚪️io', '📚️examples']      ← no engine
appChildDirs           contains ⚙️engine                          ← apps REQUIRE one
taxonomyLeafParentDirs contains ⚙️engine                          ← module engines legal
```

**An artifact is a schema — snapshot, diff, mutations, inferences — plus an io system, and never an engine.** That is now a property of the SSOT rather than a convention.

IIF's flip was the strongest available proof the inference fan-out is real: `policySchemaRepresentationBreaches` is allowlist-free and hard-gating, so the instant it landed it demanded the full inference tree on all 112 owning subsets — and high breaches went **24802 → 24801 (down by one)**, with zero new representation breaches.

## Scoreboard

| | |
|---|---|
| artifact `⚙️engine` directories | **95 → 45** |
| non-stdio | **50 of 54 dissolved** (`📕️norm` ×2, `📐️cad`, `💡️reasoning` remain) |
| `🗄️stdio` | 41 — deliberately last, see below |
| dangling `#[path]` mounts | **0** repo-wide (peaked at 20, self-inflicted) |
| artifact→app inversions | **0** in dissolved plugins (one found and fixed) |
| bare `io_registry::entries()` | **0** of 45 sites, against 44 live shadows |

**Workstream F1** (repeal) and **workstream B** (`🔄️machine`, Rust 31 tests + TS twin 30 tests, both green) are complete.

## Four ways to get a confident, well-formed, WRONG answer

Every one returns a *short, clean* result rather than an obvious failure. All four bit someone tonight.

| instrument | blind to |
|---|---|
| `cargo check` without `--all-targets` | tests/benches/examples — exactly where a vocabulary rename lands |
| `cargo check` without `RUSTC_WRAPPER=""` | stale sccache; and on `Operation not permitted` cargo **exits 0, compiles nothing, names no crate** |
| `cargo check --workspace` without `--keep-going` | **every crate after the first failure** — reported **3** failing crates where the truth was **27 of 96 / 804 errors** |
| grep-derived counts | the difference between *mentions* and *defects* |

And one that defeats all of them: a relocated **unqualified** path. **44 of 45** artifact roots carry a shadowing `io_registry` whose `entries()` returns `&[&ComposerEntry]` versus the engine's `&[ComposerEntry]`. A bare call rebinds silently — no error, green build, **wrong function**. Only reading the call site catches it.

> **Rules earned, not assumed:**
> - **A verification is a timestamp, not a property.** This ticket broadcast a "stdio is green" that was true when measured and false an hour later, to four sessions.
> - **A ceiling is a timestamp too.** A dissolution *transiently raises* counts mid-move; a shrink-only ceiling recorded at a trough gates the tree over work reducing the number it protects.
> - **Grep to find, enumerate to count.** A 22-reference "breakage" enumerated to 1 real case; 53 "inversions" to 1; SMO's 672 highs to 0.
> - **Every unqualified path in a moved body is a hazard until proven otherwise.**
> - **A stopped pass does not leave nothing behind.** Halting a pass mid-flight still stranded two `pub fn` widenings.
> - **After any operation creating or removing a directory referenced by `📦️glue.rs`, resolve every `#[path]` against disk before reporting done.** Five instances tonight across four sessions.
> - **An exemption list built from intent is a guess; one built from two readings is a fact.** APA instructed an agent that `plugin-purity` was safe to ceiling; it measured 116 → 118 → 125 and exempted it *against the instruction*. Correctly.

## Check the shim, not the shape — directory structure is NOT module path

Third occurrence of the `en1990` shape tonight, and the first where the exception was **invisible from the filesystem entirely**. Inside one plugin (`🗄️stdio`):

```rust
// md / json — engine declared BESIDE subsets
pub mod v_commonmark {
    #[path = "…/🪆️subsets/✳️any/⚙️engine/🦀️component.rs"]
    pub mod engine;                          → artifacts::md::standards::v_commonmark::engine
    pub mod subsets { pub mod any { … } }
}
// csv / xml — engine declared INSIDE subsets
                                             → artifacts::csv::standards::v_rfc4180::subsets::any::engine
```

**Every one of those four engines sits at `🪆️subsets/✳️any/⚙️engine/` on disk.** `#[path]` exists precisely so a module can be *declared* in one place and *sourced* from another, and this repo uses that freedom inconsistently. A `find` cannot distinguish them.

**How this ticket got it wrong.** This session diagnosed a peer's `E0433` as "`.composers(…)` is missing `::subsets::any`" and offered the on-disk location as proof. The original path was correct; the "fix" was applied across nine artifacts and *generated* the error it claimed to cure, and the subsequent revert over-reached and broke two files that were right.

> **The category error: an *inference dressed as a measurement*.** Every other claim exchanged between sessions tonight was re-runnable — counts, greps, `find` output, quoted compiler errors. This one had the same shape and was not. Peer verification did not catch it because the peer checked *the same insufficient evidence*: **two people verifying the same wrong thing is not corroboration, it is the same error twice.**

**The agents already had it right.** Two independently applied the correct method while both coordinating sessions reasoned from directory listings:
- `🪵️sourcing`'s, unprompted: *"engine was mounted at `standards::v1`, NOT under `subsets::any`, so a naive path swap would have been wrong."*
- the peer's own `md` agent: hit the `E0433`, diagnosed it from the glue, corrected itself, re-verified — **before** the coordinator intervened and broke it again.

> **Standing method: parse `📦️glue.rs`'s `pub mod` nesting to derive the authoritative module path per artifact, and match each `declaration()` against its own artifact's real path. Never copy a path between siblings. Where a working line exists in the same file (e.g. an already-resolving `.inferences(…)` prefix), copy from that — it is evidence; a directory name is not.**

This ticket's own wave is clear of the defect: of five dissolved plugins carrying a `.composers(…)` path, four compile clean and the fifth fails on unrelated mutation-derive and type errors, with **zero `E0433`** touching composers or `io_registry`.

## Defects this ticket caused, and fixed

1. **20 dangling `#[path]` mounts** (`🏛️architect` 11, `🧩️puzzle` 9). Agents deleted directories and left glue pointing at them — `os error 2`, crates unbuildable. Both agents then sat in wait-loops on compiles that *could never return*; one burned 386k tokens waiting. Repaired: architect by its own agent, puzzle by hand.
2. **One artifact→app inversion** — `✒️writer`'s schema test called `crate::apps::writer::register_writer_languages()`, the exact coupling this ticket removes. `🎬️sequence`'s agent had hit the identical case and rebuilt its test without the app; writer's hadn't. Test moved app-side, assertion intact.
3. **Energy's 50 simulation modules misfiled** into `💡️inferences/`. IIF proved it with an enumerator across all 112 families — energy's were the **only** non-emoji entries repo-wide. Relocated to `🔨️modules/⚡️simulation/⚙️engine/`; both sides' counts now agree at zero strays.

**A near-miss worth recording:** hand-encoding an emoji path produced `🫣️fill` instead of `🪣️fill` — a mount that looked right and didn't exist. Caught by the resolve-against-disk check, then fixed by reading the real directory name off disk. *Don't hand-write what you can measure.*

## Why `🗄️stdio` is last, and what it needs

Not caution — **measurement**. stdio's `⚙️engine` is the repo's de-facto codec library:

**15 plugins across 19 files** call into it — `🖨️raster`(14) `📸️remodel`(7) `🗒️note`(6) `📐️cad`(6) `🏗️fem`(6) `🔱️trinity`(4) `📏️layout`(3) `🌍️gis`(3) `🔋️energy`(2) `📜️imperative`(2) `🎥️shooting`(2) `🎞️animate`(2) `🧩️puzzle`(1) `🖍️draw`(1) `✒️writer`(1) — for `encode_png`/`decode_png`, `parse_markdown_blocks`, `encode_stl_ascii`, `decode_epw`, `encode_jpg`, `decode_pdf`.

So dissolving its 41 directories breaks 15 other plugins at ~60 call sites, and **`cargo check -p semio-s-plugin-stdio` would come back green while doing it.** Destination is unambiguous — these are codecs, so `🚪️io/{📥️import/🧩️deserializers,📤️export/🧵️serializers}`.

**Hard dependency:** APA has six agents converting stdio's *registration* right now, writing to artifact roots and the plugin root — the same files a dissolution touches. Sequenced behind them by agreement.

The same class exists outside stdio: the **OS renderer** depends on `puzzle::artifacts::puzzle2d::engine::BoardHost` (5 refs), and `📐️cad`'s engine is consumed by `🎪️demonstrator` and `💠️lowpoly`. **A packet can break crates its own `cargo check -p` never builds.**

## `🌿️vcs` — the packet that proved the thesis

Its demo dispatches `CheckoutCheckpoint` / `SwitchAlternative`, which change *what you are looking at*, never *what is*. They were never mutations — which is why `genesis() -> Vec<Mutation>` couldn't express them. The persisted `ArtifactEnvelope` carries `active_alternative_id` (machine state) and `cursor` (**presence** — a per-viewer caret, persisted into the artifact every collaborator shares), framework-wide across every artifact.

> **If an operation changes what you see rather than what is, it was never a mutation.**

Envelope surgery is deliberately **not** in this ticket — it needs the shared-file slot and lands after APA's conversion. Documented, claimed, parked.

## Verification status — stated plainly

`semio-s-plugin-stdio` was red for most of the night under another session's vocabulary migration (five distinct signatures across `✳️drawing` → `✳️brep` → `✳️mesh`). Every plugin depends on it, so most packets landed **structurally verified, not compiler-verified** — and were labelled that way rather than rounded up.

stdio is now **green** (`Finished`, exit 0, `--all-targets`). Individually confirmed since: `💠️lowpoly` (exit 0, 37 tests / 70 assertions preserved), `📕️norm` (exit 0), `🌍️gis` (3 errors, all pre-existing `crate::modules::terrain`, proven older via `git log` flag 487 vs this ticket's 492+), `🧱️block` (zero module-resolution errors — the failure mode a botched dissolution produces).

A full `--workspace --all-targets --keep-going` pass is pending: **16 concurrent cargo processes** are contending for the build lock. The first attempt was killed before compiling anything and reported "0 errors" — *because nothing had been built*. That reading was discarded, not published.
