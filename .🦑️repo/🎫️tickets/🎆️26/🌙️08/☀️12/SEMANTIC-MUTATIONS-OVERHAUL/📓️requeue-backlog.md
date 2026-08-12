# Requeue backlog — open items before this ticket can close

Accumulated from lane reports. Nothing here is lost work; each item names its source and what
must happen. Ordered by blocking-ness.

## A. Coordinator rulings owed (mutation vocabulary is this ticket's to decide)

### A0. RULED — app COMMAND names are not mutation vocabulary and are out of scope

Raised independently by the remodel lane (`Set*`-named command structs in 3 `🎮️commands/` files,
"host wire surface with pinned ordinals, not mutations") and surfaced by the norm lane, whose apps
still contain `En1995Command::SetSnapshot(set_snapshot::ReplaceSnapshot { … })`.

**Ruling: the ban applies to mutation vocabulary, not to app command names.**

Rationale: the point of the taxonomy is that *mutation* intent survives into the diff, the
inverse, and the history — so undo labels and merges can recover what the user meant. An app
command is the host↔plugin wire surface; its name never enters history, and several carry pinned
ordinals that a rename would break. Renaming `setSnapshot` to something else buys no semantic
recovery and costs wire compatibility.

What *is* still banned in an app command is **constructing a banned mutation**. A command called
`setSnapshot` is fine when it routes to `ArtifactStore::reset` or decomposes into semantic
mutations; it is a violation only if it builds a whole-document mutation variant.

Verified for norm rather than taken on trust: `📘️en1995`'s dispatch enum contains only `Change*`
variants (`ChangeAnnex`, `ChangeMEdKnm`, …) and has no `set-snapshot` triad dir at all, so the
`SetSnapshot` *command* structurally cannot construct a whole-snapshot mutation — it must go
through the lane's `from_snapshot` decomposition. Same check must be applied to the other 14 norm
apps and to remodel's 3 command files before their hits are dismissed.

**Consequence for Wave R3**: the vocabulary policy rule currently greps raw tokens across
`🎮️commands/**`, which will produce false positives on legitimate command names. Refine it to
flag *constructions of banned mutation variants* rather than bare identifiers, or scope it to
`🧬️mutations/**` and let call-site correctness be enforced by the compiler (a banned variant that
no longer exists cannot be constructed).

### A1. remodel `replace-tracks` — the lane flagged this as worth challenging, and it was right

Source: `📓️waveM-reports/remodel-report.md`. `SetTracks` was kept as a single `replace-tracks`.
The lane's own evidence: tracks carry ids but have **zero per-track gestures**; the only writers
are a whole-run engine re-derivation and a `clearTracks` command.

**Ruling — split it in two, and neither half is `replace-tracks`:**
1. **The `clearTracks` gesture is `clear-tracks`.** `clear` is in `APPROVED_VERBS` ("empty a
   collection/field wholesale"), and its inverse is defined: re-`create`/`add` every captured
   member from `base`. That is a real user gesture and gets real vocabulary.
2. **The whole-run engine re-derivation is not a mutation at all.** If tracks are recomputed from
   the run rather than authored, they are a *derived* value and belong in an `💡️inferences`
   facet — the same call APA and this session made for lowpoly's texture cache, and the same call
   DKM made for tessellation/measure/validate on brep. Authoring mutation vocabulary for a
   derived value means minting a diff and an inverse describing a cache fill.

A surviving `replace-tracks` would be a whole-collection setter, which the taxonomy forbids
outright. Requeue: confirm whether tracks are engine-derived; if yes, remove `replace-tracks` and
route re-derivation to an inference; keep `clear-tracks`.

### A2. remodel's two arguable `update` verbs
`update-camera-calibration` and `update-rig-extrinsic` are written up individually in the report.
Audit both against the `update` test: ≥2 fields, genuinely inseparable, never meaningfully set one
at a time. Three other sessions reached for `update` wrongly today; these two deserve the same
scrutiny before they stand.

## B. Verification gaps (block the ticket's exit criteria)

### B1. Framework law tests cannot build — FOREIGN, blocks everyone
`cargo check --workspace --all-targets` → `semio-framework-os-kernel` **lib test** fails with 144
errors, all in `🔨️modules/🏪️store/🔄️sync/🦀️component.rs`: `tempfile` is used but is not a
dev-dependency of the os-kernel crate, plus `DemoSnapshot`/`DemoMutation` fixtures failing
`ArtifactPack`/`OpText`/`OpBinary` bounds. Plus 1 error in `🧠️neural` (`Schema` has no field
`extension`).

Evidence it is not this ticket's: the `🔄️sync` module predates the ticket (commits 492/480/467),
this ticket never touches it, and this ticket is barred from editing `Cargo.toml`.

**Why it matters**: this blocks the framework's own `MiniMutation` fixture and the testkit law
helpers (`assert_mutation_inverse_law`, `assert_mutation_diff_absorb_law`,
`assert_diff_algebra_*_law`) — the mechanism's correctness argument. Broadcast to peers; owner
must add the dev-dependency.

### B2. Per-plugin law tests not yet run
`cargo check --workspace --all-targets` shows **zero plugin errors**, so plugin test code compiles.
Actual `cargo test` runs are queued behind heavy machine contention (80+ cargo processes across
five concurrent sessions). Confirmed so far: `🖨️raster` 66/0, `🌍️gis` 171/0.

### B3. `assert_op_text_binary_equivalence` sweep never run
Multiple lanes skipped it. Needs a pass once B1/B2 clear.

### B4. `impl DiffAlgebra` missing on several artifact diffs
Explicitly noted for `RemodelDiff`; likely others. Required before the final ratchet tightens
`Mutation::Diff` to `MutationDiff<P> + DiffAlgebra<P>`.

## C. Known-incomplete lane work

| item | source | state |
|---|---|---|
| `🧱️block` 3d/5d marked `partial`, never compiled by their lane | block reports | workspace check now shows 0 plugin errors, so they compile; law tests unrun |
| `🕸️dag` test build | was blocked by the panels rename (now fixed by another session) | re-run |
| `🔱️trinity`/`♻️rewrite` — zero compile verification by its lane | trinity report | workspace check clean; law tests unrun |
| `📖️playbook` — no clean confirming run | playbook report | workspace check clean; law tests unrun |
| stale `📡️component.protocol.semio` / `📖️component.grammar.semio` | remodel, block, others | Wave B honesty sweep |
| `Set*`-named app **command** structs in 3 remodel `🎮️commands/` files | remodel report | these are host wire surface with pinned ordinals, NOT mutations — decide whether they are in scope at all |
| schema description files left generic in `graphql`/`json`/`proto` | block reports | Wave B honesty sweep |

## D. Deferred by cross-ticket agreement

- **stdio, 53 facets** — behind UCAS's roster restructure; unstarted, brief ready.
  Includes the approved `set-primitive-geometry` → `replace-primitive-geometry` rename in
  `✳️mesh` (approved for DKM, absorbed into this lane to keep third parties out of stdio).
- **`🧿️semio ✳️any`** — 18-way union dispatch, migrate last, after all sub-subsets.
- **Framework kernel bridges** — `🌊️flow/🌿️vcs` (40 `CollectionMutation` hits) taken by DKM
  (#2550); `🪐️space` module still unowned. Both are the hard floor preventing full
  `CollectionMutation` elimination from the plugin side.

## E. Policy + ratchet (Waves R3/B), not started

`📜️script.ts` write slot is queued APA → UCAS-W6 → SMO → inference-family. Contents unchanged
from the plan: repoint 4 wrong-depth rules, extend ts-mirror to flag MISSING mirrors, widen the
vocabulary scan beyond `🧬️mutations`/`🎮️commands`, prune stale allowlist entries, add
grammar-coverage + DiffAlgebra-scope rules. Then the staged ratchet.
