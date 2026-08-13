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

## Precise closing framing — 0 pre-existing, 1 transitional (not owned by this ticket)

**Correction to the headline below, made by DKM (#owner of `DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`) and adopted verbatim because it's more accurate than my own first framing:**

> *"That directory was created during your dissolution and is genuinely new, so your census was accurate when you took it and is accurate now — but the count is a moving target while my ticket is live... it may be worth phrasing as '0 pre-existing, 1 transitional owned by #DKM with a stated dissolution path', so a later reader doesn't see the number go 0 → 1 and conclude your work regressed."*

`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️brep/🧬️schema/⚙️engine/` exists (created during, not before, this ticket's work) — a deliberate, documented, correctly-mounted temporary forward edge (`stdio → semio-framework-3d`) for DKM's "brep flip", explicitly scoped to let ~35 framework-3d brep subdirs migrate one at a time without an atomic cross-crate change. **Investigated, not dissolved**: reading its own docstring before acting is what distinguished it from the 95 genuine fossils this ticket cleared — those were abandoned rushed relocations; this is live, owned, in-flight architecture from a different ticket's mandate.

DKM's stated end state is stronger than "dissolves as subdirs peel": the `BrepKernel` trait itself is slated to stop existing once its ~16,000 LOC of algorithms land in `📸️snapshot/💡️inferences` compute subdirs — "the correct outcome is not move `⚙️engine` somewhere legal, it's the trait stops existing." My forbidding rule (`policyArtifactEngineFacetForbiddenBreaches`, at `low`) will correctly report it until then; DKM has committed to treating a surviving `⚙️engine` at their own close time as a failure state, not a resting one.

**A second finding from the same exchange, worth recording for whoever next touches the taxonomy walker**: `policyTaxonomyDirsBreaches`'s `NestedFacetWalk` (📜️script.ts:4076+) only descends when an artifact's direct child is literally `🧬️schema` or `🚪️io` — but `artifactChildDirs` doesn't include `🏅️standards`, and every artifact in this repo is new-shape (child = `🏅️standards`). **Measured: 0 hits for "not a recognized artifact/representation dir" repo-wide** — not because the tree is clean, but because the walker's else-branch at `🏅️standards` never descends to subset facets at all. This ticket's own forbidding rule may be reached via a different code path (confirmed reporting 95, then correctly tracking the burn-down to 0, so it does NOT share this blind spot) — but the general shape (a rule that looks installed and never fires) is the same failure class as an earlier finding tonight about the artifact-io dispatch bridge. UCAS is retargeting `artifactChildDirs` at `🏅️standards` in their own W6.

## Re-verification after a 9.5-hour gap and a reported disk-full incident

My last "stdio compiles clean" check before this section was taken at **11:23**, clustered tightly with three other checks from the same sweep (11:12–11:24). A peer session reported the machine hit 100% disk (257 MB free) later that evening (~19:30), producing `No space left on device` errors that read exactly like real compile failures — they watched one plugin's error count swing 94 → 16 → 116 before diagnosing it, and 202 GB was freed to fix it.

**Given nearly 10 hours and a disk incident between my claim and this record, the claim was re-verified fresh rather than trusted**: `RUSTC_WRAPPER="" cargo check -p semio-s-plugin-stdio --all-targets` against a brand-new `CARGO_TARGET_DIR`, disk confirmed healthy (168 GB free) immediately before running. **Result: `Finished` in 4m25s, exit 0, zero errors — reconfirmed.**

**11 cross-plugin consumer crates were checked directly** (not merely grep-verified, per the wave-2 media/semio agent's own flagged gap): `fem`, `cad`, `layout`, `puzzle`, `raster`, `shooting`, `gis` all compile clean. `draw` and `lowpoly` each show one error — both confirmed **out of scope**, not this ticket's: draw's is an `E0716` borrow-lifetime issue in a file last touched by another session at 18:52, unrelated to engine dissolution; lowpoly's is `semio_framework_number` unresolved inside DKM's own in-flight `semio_framework_3d::brep` work, the same live dissolution flagged above. `trinity` timed out on build-lock contention after 6m40s — no verdict claimed either way.

## FINAL STATE — 95 → 0

```
artifact ⚙️engine directories:  0   (started at 95)
dangling #[path] mounts:        0   repo-wide
semio-s-plugin-stdio:           Finished in 1m35s, exit 0, ZERO errors
```

Every artifact-tree `⚙️engine` directory in the repository — including all 41 of stdio's, the largest and most cross-coupled block — is dissolved. The last wave ran as 5 concurrent agents plus direct hand-repair of the errors they surfaced; `🧿️semio`, the artifact that had already defeated one full session earlier tonight, was resolved this time by working symbol-by-symbol from the compiler's own error text rather than by file-wide edits.

**218 remaining `artifacts::X::engine::` references, enumerated rather than trusted.** All are legitimate: the 10 deliberately-protected imperative `engine::register()` plugin-root calls (a separate `dsl::registry::register_schema_spec` registry `ArtifactDeclaration` has no field for) and external consumers (`remodel`'s jpg import, etc.) — both resolving through inline `engine`-named compatibility barrels that several packets kept as a Rust module identifier after deleting the `⚙️engine` **directory** it used to name. The taxonomy governs filesystem facets, not arbitrary internal module names — this is compliant, and the fact that stdio compiles clean with all 218 present is the proof, not an assumption.

**Module engines (60) and app engines (51) both remain, as required.** Only artifact engines were ever abolished; `taxonomyLeafParentDirs` keeps the former legal, `appChildDirs` requires the latter.

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

## Simultaneous agent-fleet failure and hand repair

All six dispatched agents (5 stdio blocks + the norm regression repair + the cad TS remainder) were killed **at the same moment** by an API session limit, mid-edit. Damage assessment before doing anything else: **zero dangling `#[path]` mounts** despite the simultaneous kill — the individual edits that had landed were each internally consistent, just incomplete as a set.

Repaired by hand rather than re-dispatching into an unknown-duration outage, working error-by-error against `RUSTC_WRAPPER="" CARGO_TARGET_DIR=target/stdio_final cargo check -p semio-s-plugin-stdio --all-targets`:

1. **Two mid-file `//!` doc-comment blocks** (`☁️las/…/🚪️io`, `🎥️mp4/…/🚪️io`) — `//!` is only valid as the very first thing in a file/module; agents had appended large explanatory blocks mid-file after `//#region` markers. `E0753`. Converted to `//`.
2. **`docx`'s `📥️import/📤️export` real component files were never mounted** in glue — only their nested cross-format bridge submodules (`artifacts::zip::…`, `artifacts::xml::…`) were, so the direct file holding `decode_docx`/`encode_docx`/`build_minimal_docx` was orphaned. Added the missing `mod component; pub use component::*;` before each nested block.
3. **`pptx`/`xlsx` were missing their compatibility `engine` shim** — their real engine module still exists at `standards::v_ecma_376::engine` (their dissolution hadn't started), but the top-level `pptx::engine`/`xlsx::engine` passthrough that ~15 internal call sites expect was never added. Restored it.
4. **`docx`'s `engine` shim pointed at a module APA had already deleted** — repointed to the real new locations (`io::import::deserializers`, `io::export::serializers`, `io::io_registry`) rather than the stale `engine` path, since 16 in-crate call sites plus one cross-plugin consumer (`✒️writer`) still used it.
5. **Two path-nesting mistakes surfaced only once earlier errors cleared**: `docx`'s `✳️transitional` subset called `…::engine::sync_main_part` (real home: `…::io::export::serializers::sync_main_part`); `html`'s schema file aliased `use …::engine as engine;` where the real function lived two levels deeper than assumed (`io::import::deserializers::sniff_real_bytes`, not `io::sniff_real_bytes` — discovered by reading the actual nesting in the source file, not by inference).

**Verified, not assumed, at each step**: re-ran the full `cargo check` after every fix rather than trusting an error count from memory; one re-check surfaced 19 errors where the prior read showed 3 — not a regression, but earlier errors had been blocking rustc from reaching the code underneath.

**Result: `semio-s-plugin-stdio` — `Finished` in 24.97s, exit 0, zero errors.** `semio-s-plugin-norm` (the earlier regression) reconfirmed fully green (`Finished` 20.60s, exit 0) now that its dependency compiles. `✒️writer` (a docx consumer) still shows 20 errors — checked and **none touch docx or the engine dissolution**: they're an untouched `pdf` artifact (`PageDoc` unresolved — pdf's dissolution hasn't started) and a `WriterMutation` variant mismatch on a file last modified at commit 497, well before this session — confirmed pre-existing via `git log`, not fixed.

**Remaining stdio scope, honestly stated: 29 of 41 artifact engines are untouched** (directories and content both, not merely unmounted) — the repair above only fixed the handful whose deletion had already landed before the agents died. That work has not restarted.

## Workspace-wide re-verification after the stdio fix

Ran `cargo check --workspace --all-targets --keep-going` against a dedicated target dir (`target/ws_final`) to avoid the shared-lock contention that killed the earlier attempt. **Confirmed complete this time** — tail shows the run's final crates (`norm`, `stdio`) finishing with warnings only, no truncation.

**Result: still 23 failing crates — same count, but not the same set.** Diffed against the pre-repair baseline:
```
FIXED: semio-s-plugin-norm       (the regression this session repaired)
NEW:   semio-s-plugin-remodel
```

**The headline finding: fixing stdio did NOT reduce the overall failure count**, because most of the 23 failures were never caused by stdio being uncompilable — they carry independent, pre-existing bugs (mutation-vocabulary renames, an untouched `pdf` artifact, unrelated type errors) that show up whether or not stdio compiles. Spot-checked `block` directly: identical 8 errors, same as before the stdio fix, none touching declaration/io_registry/composers. `✒️writer`'s 20 errors are similarly unrelated (see above). **A green stdio does not imply a green workspace** — it only removes stdio itself from the blocking set.

**`remodel` appearing as newly-broken was investigated, not assumed.** Its own artifact IO file was clean; the actual break was in `🎛️apps/📸️remodel/⚙️engine/🎥️video/🦀️component.rs`, which imports `avi_engine`/`mp4_engine` aliased from `semio_s_plugin_stdio::artifacts::{avi,mp4}::standards::…::engine`. Both of stdio's `avi` and `mp4` engines had been dissolved (mp4 by this session's own repair; avi by an untracked concurrent write — its directory was gone with no corresponding entry in this session's edit history) **without their external consumer being updated** — exactly the cross-plugin-consumer pattern flagged earlier in this document. Repointed both aliases to `subsets::any::io` (their real new home, verified by finding `decode_avi`/`encode_avi`/`decode_mp4`/`encode_mp4` as top-level `pub fn`s there). **`semio-s-plugin-remodel` — `Finished` in 3.69s, exit 0, confirmed.**

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
