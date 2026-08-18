# 📓️ terra — D2-capability-claim-repairs — report

CARGO_TARGET_DIR used throughout: `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/MICROKERNEL-POOLED-ACTOR-PLUGIN-RUNTIME/🎯️target-d2`

## Summary

Row-level capability-claim fixes were made and manually cross-referenced against the exact SDK
derivation constants for all 7 plugins (fem×2, layout, playbook, trinity×2, puzzle×3, block×3 — 11
artifact roots). **Mechanical confirmation via `runtime_capability_requirements()` succeeded for 2 of
7 (fem, layout)** before a severe, unrelated environmental blocker (below) made every further cargo
build in this session fail. The remaining plugins' fixes are real (verified against Rust source, not
guessed) but **not mechanically build-confirmed** — reported as such, not claimed as done.

**0 of 7 were added to `DESCRIPTOR_MIGRATED_PLUGINS`.** `descriptor_is_fresh` never got a clean run
for any of the 7 this session (see environmental blocker). `🔌️plugin/🦀️component.rs` was **not
edited**.

## ⚠️ Environmental blocker (not caused by this packet, discovered ~21:58–22:15 CEST)

`🧧framework/🔨️modules/🎒️pack/**` (a shared dependency of `semio-framework-os-kernel`, which every
plugin transitively depends on) is **untracked in git** (`git status` shows `?? 🧰️framework/🔨️modules/
🎒️pack/`) and was being actively rewritten by an unidentified live peer session throughout this
window:

- `🦀️component.rs` inside it changed size **3 times** while I watched: 7973 → 7734 → 2882 bytes,
  mtimes moving from 20:06 to 22:02 to 22:05.
- 6 consecutive `cargo`/`bun describe` invocations against **fem** (once, before this got severe),
  **layout** (5×) and **playbook** (2×) failed with `couldn't read …/🎒️pack/{🦀️component.rs,⏳️async/
  🦀️component.rs}: No such file or directory`, each time on a *different* file inside that directory.
- A later attempt (**block**, 1×) got further and hit **44 real `error[…]` compile errors** inside
  that same directory (`🧪️testkit/🦀️component.rs:341` `no field 'frame_size' on type
  'history::EncodeOptions'`, plus `E0425`/`E0432`/`E0433`/`E0659`), confirming this is a genuine
  in-flight, currently-broken refactor, not a filesystem race.
- Cleared `🎯️target-d2/**/incremental/` mid-session (rule 5) between attempts — made no difference,
  ruling out my own incremental-cache staleness as the cause.
- System load average was **61–82** throughout (`uptime`), with 36–54 concurrent `cargo`/`rustc`
  processes observed — matches `important.md`'s documented "Concurrent Cargo Workspace Churn" pattern
  and the coordinator's own two 10-minute timeouts against the peer's fleet prebuild.

**Not in my `path_scope`, not touched.** I stopped retrying once the 44-error evidence made clear
this cannot resolve from my side; chasing it further would be exactly the "wake/idle" budget sink the
coordinator flagged. Whoever owns `🎒️pack` needs to land or revert it before *any* plugin's `describe`
or `--lib` test can run cleanly again.

## Per-plugin table

| plugin | missing rows found | repaired? | describe exit code | descriptor committed? | ratcheted? | real error |
|---|---|---|---|---|---|---|
| 🏗️fem (2d+3d) | composer: missing native self-row (`s.fem2d@1/*`/`s.fem3d@1/*`); codec: wrong schema string (`fem.fem2d`/`fem.fem3d` should be `fem.2d`/`fem.3d`) | yes, both | 0 (ran successfully once, **before** the pack blocker) | yes, but `"assembly-failed"` — a **different, real** bug surfaced after the fix (see below) | no | **dialect collision**, not capability-claim: `"dialect:s.stdio.csv@rfc4180/* is already registered by s.fem2d.composer.csv"` — fem2d and fem3d are bundled into ONE plugin and both declare composer entries writing to the same 5 stdio export dialects (csv/md/json/stl/obj). Out of D2's charter (this is D1's *dialect-collision* class, same as procedural/gis, just intra-plugin instead of cross-plugin) |
| 📏️layout | composer: missing native self-row (`s.layout@1/*`) | yes | not run (blocked) | not run | no | describe blocked by the `🎒️pack` environmental issue, 5/5 attempts |
| 📖️playbook | composer: missing native self-row (`s.playbook@1/*`) | yes | not run (blocked) | not run | no | describe blocked by the `🎒️pack` environmental issue, 2/2 attempts |
| 🔱️trinity (jack+rewrite) | jack codec: wrong extension (`"jack"` should be `"trinity"`, matching `JackSnapshot::EXTENSION`). rewrite: **no gap found** — its codec row already matches `TrinityRewritePlayApp::DOCUMENT_SCHEMA`/`RewriteRuleSnapshot::EXTENSION` exactly | yes (jack only) | not run (never reached before pivoting to block/puzzle under time pressure) | not run | no | not yet build-confirmed |
| 🧱️block (2d+3d+5d) | codec: wrong extension on all three (generic `"block"` should be `"block2d"`/`"block3d"`/`"block5d"`, matching each `Block*dSnapshot::EXTENSION`) | yes, all 3 | not run (1 attempt reached further than layout/playbook but hit the 44-error `🎒️pack` break) | not run | no | see environmental blocker |
| 🧩️puzzle (2d+3d+5d) | codec: wrong extension on all three (`"puzzle2d"`/`"puzzle3d"`/`"puzzle5d"` should be `"puzzle2d-play"`/`"puzzle3d-play"`/`"puzzle5d-play"` — the required claim comes from each editor's real `Snapshot` associated type, `Puzzle*dPlaySnapshot`, not the base `Puzzle*dSnapshot`) | yes, all 3 | not attempted | not run | no | not attempted — see known trap below |
| 🗄️stdio | investigated, not isolated (see below) | no | not attempted | not run | no | not isolated within budget |

## `🧱️block` — "already has a committed descriptor" reconciled

`git show HEAD:✏️s/🔌️plugins/🧱️block/🔣️descriptor.json` contains `"pluginId": "assembly-failed"`,
`"label": "no declared codec capability owns the runtime claims"` — this is **D1's own placeholder**
(their report explicitly says it was generated then deleted, never meant to land). The auto-commit bot
evidently swept it into `HEAD` before D1's local deletion took effect; `git status` currently shows
both `🔣️descriptor.json` and `🛂️descriptor.semio` as `D` (deleted in the working tree relative to the
index) — i.e. the garbage is stuck in history but correctly absent from disk right now. **Not mine.**
I have not yet produced a real descriptor for block (blocked, see above) — once `describe` runs clean,
its output will overwrite this on the next auto-commit sweep.

## Where the capability-claim rule was NOT the real blocker

**🏗️fem is the clean example.** D1 classified it as pure capability-claim ("no declared composer
capability owns the runtime claims"). That was real and I fixed it (verified via an isolated
integration test — see below). But `describe` then ran the wasm component for real and hit a
**different, deeper bug**: fem2d and fem3d are two artifacts bundled into **one** `Plugin::builder
("fem")`, and both independently register composer entries for the same five stdio export dialects
(`csv`/`md`/`json`/`stl`/`obj`). The plugin-level composer registry rejects the second registration as
a duplicate dialect claim. This is D1's *other* failure class (the procedural/gis `dialect-collision`
pair), not a capability-claim mismatch — it just couldn't surface until the capability-claim check
upstream of it stopped firing first. **I did not fix this** — resolving it means deciding which
artifact keeps which export format (or disambiguating the dialect registry by owning artifact), which
is an architecture decision outside "repair rows so declared == runtime," not a row repair.

## Mechanism used (per plugin, before the environmental blocker hit)

For fem and layout: a temporary integration test (`📦️packages/🦀️rust/tests/dbg_capability_claim_diff.rs`,
**not** `--lib`, to sidestep pre-existing unrelated `#[cfg(test)]` compile breakage inside these
crates — see below) built the same `ArtifactDeclarationBuilder` chain each artifact's real
`declaration()` uses, called `.runtime_capability_requirements()` on it (works before `try_build()`,
doesn't consult `definition_error`), and diffed the result against `definition()`'s own
`.capabilities()`. `pilot_languages()` was temporarily bumped `fn` → `pub fn` in each touched file (an
integration test lives in a separate crate and cannot see private items) to let the test call it;
**reverted to `fn` (private) again once the tests were deleted** — confirmed via `grep`, present in
all 12 touched files.

```
cargo test -p semio-s-plugin-fem --test dbg_capability_claim_diff -- --nocapture
  → [DEBUG] fem3d missing rows: [("codec", [("codec","fem.3d"),("extension","fem3d")])]
  → [DEBUG] fem2d missing rows: [("codec", [("codec","fem.2d"),("extension","fem2d")])]
  (first run, composer-only fix in place — codec fix not yet applied)
  test result: FAILED. 0 passed; 2 failed — exit reflects the two panics above

cargo test -p semio-s-plugin-fem --test dbg_capability_claim_diff -- --nocapture
  (after the codec fix too)
  test fem3d_capability_claim_diff ... ok
  test fem2d_capability_claim_diff ... ok
  test result: ok. 2 passed; 0 failed — REAL_EXIT: 0

cargo test -p semio-s-plugin-layout --test dbg_capability_claim_diff -- --nocapture
  test layout_capability_claim_diff ... ok
  test result: ok. 1 passed; 0 failed — REAL_EXIT: 0 (13m11s compile under heavy fleet load)
```

For playbook/trinity/puzzle/block, the same integration-test files were written and staged
identically, but every attempt to run them (playbook ×2, block ×1) hit the `🎒️pack` blocker before
reaching my test code, so I have **no printed diff output** for those four — the row fixes listed
above come from direct source cross-reference (reading the real `EXTENSION`/`DOCUMENT_SCHEMA`/`WRITES`
Rust constants each claim must equal, the same check the mechanism performs, just done by hand because
the mechanism couldn't run). Given fem's own codec bug was invisible to this same hand-check style
until the mechanical diff caught it, **these four should be re-verified mechanically** once the
`🎒️pack` blocker clears — I would not have caught fem's codec mismatch by inspection alone.

All temporary diff-test files removed before this report (`fem`×1, `layout`, `playbook`, `trinity`,
`puzzle`, `block` — 6 files, `tests/` dirs removed too where now empty). None kept as permanent parity
tests: each depends on private items (`pilot_languages()`) that had to stay private, so keeping them
would have meant leaving the visibility widening in place for no other reason.

## 🗄️stdio — investigated, not isolated

D1's error: `"no declared inference capability owns the runtime claims"`. stdio's channel is
JSON-data-driven (36 `artifact-definition.json` files under `📇️registry/`, one per format), unlike the
other 6 plugins' hand-written Rust literal rows. Traced the actual mechanism:

- `source.inferences: Vec<ExecutableLeaf>` is a **governance ledger** (id/status/executable_registration
  only) — `declared_capability()` builds these into claim-less `ArtifactCapability` rows on purpose;
  they are not meant to satisfy the runtime-claim check.
- The rows that actually need to match are `source.runtime_capabilities: Vec<RuntimeCapability>`
  (separate JSON array, carries real `category`+`claims`) — `runtime_capability()` builds these with
  real claims via `runtime_claims()`.
- Wrote a Python sweep over all 36 `artifact-definition.json` files checking "has a non-empty
  `inferences` ledger but no `runtime_capabilities` entry with `category == "inference"`": **zero
  hits**. Also checked for artifacts with zero `runtime_capabilities` at all — 10 hits (`binary`, `txt`,
  `ifc`, `gif`, `bmp`, `semio`, `wav`, `epw`, `tsv`, `html`), all confirmed legitimate
  `definition_only_assembly` (schema-only, no runtime facet, correctly has nothing to check).
- Manually, exhaustively verified the **first two** `runtime_assembly`-registered artifacts in
  `sources()`'s iteration order (`xml`, then `gltf`) — every claim (schema id, inference schema id,
  both composer dialects incl. the `/valid` subset, the subset-validator dialect, all 5 grammar ids,
  representation mime+extension, codec schema+extension) cross-checked against its real Rust-side
  constant (`XmlSnapshot::EXTENSION`, `DIALECT_VALID`, `XmlValidValidator::DIALECT`, etc.) and every
  single one matched exactly. **Neither is the broken artifact.**

Given `artifact_assemblies()` collects in iteration order and stops at the first `Err`, and the first
two candidates are both clean, the actual failing artifact is one of the remaining ~24
(`deflate`/`zip`/`json`/`csv`/`md`/`obj`/`stl`/`ply`/`las`/`step`/`ifc`/`dwg`/`dxf`/`svg`/`png`/`jpg`/
`tiff`/`pdf`/`docx`/`pptx`/`xlsx`/`bcf`/`mp4`/`avi`/`mp3`/`tsv`... — some already confirmed
definition-only above). Not isolated within budget; **no fix attempted, nothing changed in stdio.**

## Known trap confirmed but not exercised

`🧩️puzzle` — did not reach the point of running `cargo test -p semio-s-plugin-puzzle --lib` (blocked
upstream by the environmental issue before I got there). Per the ticket's own pre-registered trap
(packet Z1's 176 pre-existing unrelated compile errors), that command would not be expected to build
regardless — noted here rather than spending a build cycle proving it again.

## Peer-coexistence

- Liveness check before starting (per packet brief): `git log --date=iso --oneline -3` + mtime sweep
  on all 7 target dirs showed only the stale `🌙️06☀️04` history and no files touched in the last
  30–40 minutes. Proceeded on all 7.
- Mid-session: `🎒️pack` churn (above) — not one of my 7, not touched.
- `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🦀️component.rs`
  shows as staged-`M` in `git status` with **zero working-tree diff from me** — a pre-existing staged
  change from another session's edit, picked up by the auto-commit bot before I started. Not touched
  by this packet.
- `26/08/17/CLEAN-ARTIFACT-STANDARD-SUBSET-MECHANISM`: never touched a declaration channel
  (`.artifact(declaration())` / `.declare_artifact(artifact())`), never deleted a `definition()` row.
  All 7 plugins use the OLD `definition()`/`declaration()` pair exclusively (confirmed via grep for
  `fn artifact()` before starting — none found in fem/layout/playbook/trinity/puzzle/block). Repaired
  rows in place, in that same channel, per the HARD CONSTRAINT.

## Files touched

- **Edited** (row repairs, kept): `✏️s/🔌️plugins/🏗️fem/🗿️artifacts/{🧊️3d,◻2d}/🦀️component.rs`,
  `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🦀️component.rs`,
  `✏️s/🔌️plugins/📖️playbook/🗿️artifacts/📖️playbook/🦀️component.rs`,
  `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🦀️component.rs`,
  `✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/🦀️component.rs`,
  `✏️s/🔌️plugins/🧱️block/🗿️artifacts/{◻2d,🧊️3d,🖐️5d}/🦀️component.rs` — capability row fixes only,
  each with an inline `🐛️ D2-capability-claim-repairs` doc comment citing the real Rust constant that
  proves the claim value.
- **Touched then reverted to original** (net no-op, confirmed via grep): `pilot_languages()`
  visibility in all 12 files above plus `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/♻️rewrite/🦀️component.rs`
  (11 files touched for the fix + rewrite for symmetry checking = 12 total) — bumped `pub` for the
  temp integration tests, reverted to private after deleting them.
- **Created then deleted** (never left on disk): `tests/dbg_capability_claim_diff.rs` under each of
  fem/layout/playbook/trinity/puzzle/block's `📦️packages/🦀️rust/`.
- **Not touched**: `🔌️plugin/🦀️component.rs` (no ratchet entries added — none passed
  `descriptor_is_fresh`), `🗄️stdio/**` (investigated only), any declaration-channel file, any
  `🤖️generated/**`, registrar files.

## Lease-requests

None. All work stayed within path_scope.

## Acceptance

**0 of 7 fully converted** (repaired + describe-confirmed + ratcheted). Honest breakdown:
- **2 of 7 mechanically confirmed correct at the row level** (fem, layout) via a real
  `runtime_capability_requirements()` diff — but fem's `describe` then surfaced a *different*,
  out-of-charter dialect-collision bug, and layout's `describe` never got to run (environmental
  blocker).
- **4 of 7 repaired by exact source cross-reference, not yet mechanically confirmed** (playbook,
  trinity/jack, puzzle×3, block×3) — high confidence (same derivation mechanism, same constants
  checked), but per this ticket's own rule 7 ("never claim a test passed without pasting output"), I
  am not claiming these are proven, only that the fix is well-reasoned and the verification step is
  blocked, not skipped.
- **1 of 7 not fixed** (stdio) — investigated, root cause narrowed to "one of ~24 remaining artifacts'
  JSON data," not isolated.

A partial reported honestly: **the row repairs are real work product; the describe/ratchet
verification for 5 of 7 needs a re-run once `🧧framework/🔨️modules/🎒️pack` stops being actively
rewritten by its owning session.**
