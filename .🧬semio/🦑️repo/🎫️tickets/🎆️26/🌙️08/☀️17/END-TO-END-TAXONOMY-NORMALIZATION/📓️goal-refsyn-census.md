# reference-syntax-unsupported / frozen-coordinate-evidence-unowned — Census

Scope worked: `🧰️framework/🔨️modules`, baseline `bb06c41f73f0122fbed315b7487428b976f99921` (this ticket's
fixed baseline throughout). Source data: `$T/🗑️temp/🔣️vocab-plan.json` (handed off, later renamed to
`🔣️vocab-plan-3.json` by a concurrent worker before we could re-read it — see caveat in the report)
and a fresh `$T/🗑️temp/🔣️refsyn-plan-after-rustjoin-fix.json` produced after our own engine edit.

Given counts drift run to run (live, concurrently-edited repo — four other workers touching
`🔣️taxonomy.json`, plugin sources and the engine this session): **397/109 as handed off** →
**448/117 on our first read** → **440/125 after our fix** (see report for the reconciliation). All
families below are keyed by structure, not by a specific row count, so they hold regardless of which
snapshot you're looking at.

## `reference-syntax-unsupported` families

By message kind (`unsupported-path-syntax` / `rust-path-join` / `rust-path-join-unproven`), then by
population (production source vs ticket-owned vs `.cursor/plans`):

| message kind | production | ticket | cursor-plan |
|---|---:|---:|---:|
| `unsupported-path-syntax` | ~109 | ~153 | 0 |
| `rust-path-join` | 155 | 0 | 0 |
| `rust-path-join-unproven` | 34 | 7 | 0 |

### Family A1 — `rust-path-join`, mutation-root self-verification tests (155 rows, 16 files) — FIXED

Every row is inside a plugin's `🧬️schema/🧬️mutations/🦀️component.rs` (or `.rs`) mutation-root test —
the same 13–16-file population §7/§8 already found and twice misdiagnosed. Instrumented
`rustFiniteManifestTargets`'s early guards directly (guard-by-guard, per the coordinator's
instruction) on the real repo state:

- Guards #1–#3 (the ones §8 suspected) do **not** fire for these files today.
- The function proceeds into the module-graph proof, and dies at what we labelled guard#8:
  ```ts
  const withoutPathAttributes = text.replace(/#\s*\[\s*path\s*=\s*"[^"\\]*"\s*\]/gu, "");
  if (/[#!]/u.test(withoutPathAttributes) || /\bmacro\b/u.test(text) || ...) return result;
  ```
  applied to every OTHER `.rs` file in the crate's proof chain (i.e. the crate root
  `📦️glue.rs`, never the file under test itself). `text` is the **raw file text, comments
  included**. Every plugin's `📦️glue.rs` opens with a multi-line `//!` module doc comment (this
  repo's own docstring convention — CLAUDE.md mandates emoji-prefixed native docstrings on every
  definition) — and `//!` itself contains a `!`, so `/[#!]/u.test(...)` fires on **every** such
  crate root regardless of content, before the regex ever looks at real code.

**Fix implemented** (`🧹️normalization/🟦️.ts`): added `rustCodeOnlyText()`, which reuses the
already-existing, already-tested Rust tokenizer (`rustTokens` in `🔍️discovery/🟦️component.ts` —
exported it, previously private) to reconstruct the file with every `//`/`/* */` comment discarded,
strings/chars/raw-strings kept atomic. Guard#8 now tests that comment-free text instead of the raw
one. This is purely subtractive on the input the check sees (comments can never affect Rust
compilation), so it can only ever remove false positives, never admit a genuinely dangerous file it
previously rejected.

**Real-world effect, measured**: for the one file we instrumented end-to-end
(`✏️s/🔌️plugins/🎬️sequence/…/🧬️mutations/🦀️component.rs`), the fix does **not** by itself clear the
row — its crate root also calls a genuine macro, `semio_framework_plugin::plugin_exports!(...)`,
which trips the (correct, unrelated) macro guard. A full-scope run before/after the fix showed the
155-row population **unchanged in count and membership** — every one of these 16 crate roots calls
`plugin_exports!` too, so the comment bug was never the sole blocker for this specific population.
The fix is still real, correct, and load-bearing for any OTHER Rust file whose only "suspicious"
content is comment prose (verified with a new regression case, see report) — it just isn't the fix
for *this* population's row count.

**Not implemented, proposed**: `plugin_exports!`'s own definition
(`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:31447-31628`, macro_rules!, ~180
lines) contains **zero** `mod` tokens anywhere in its body — verified by grep over the full macro
extent — so no invocation of it can ever expand to a `mod` item; it is provably transparent to the
module-graph proof. A narrow, name-based allowlist (treat a call to this one specific,
framework-owned macro as non-hazardous for guard#8, while leaving *any other* macro invocation
exactly as suspicious as today) would very likely clear all or nearly all of the remaining 155+34+7
rows in this population. Not implemented here: it is a second, judgment-heavy change to the same
guard, for a population that (per §8) has already survived three wrong diagnoses, and CLAUDE.md /
the ticket brief are explicit not to restructure this area again on a hypothesis without dedicated
verification. Flagging as a precise, ready-to-implement follow-up rather than rushing it.

### Family A2 — `unsupported-path-syntax`, bare string-literal path tokens in production config/glue (~109 rows, ~33 files)

The task's own example (`.dependency-cruiser.cjs`) generalizes to a real, heterogeneous population —
confirmed diverse, not one shape:

- **TS/JS config object literals**: `.dependency-cruiser.cjs` (dependency rules), every
  `🎯️targets/⚛️react/🧪️vitest.config.ts` (relative `resolve`/`test.include` string arrays, several
  modules), `🟨️frame-worker.js` (a **generated/bundled** JS file — its literal is a JSON-escaped
  `\uD83E\uDDF0️...` surrogate-pair encoding of an asset path, produced by a bundler, not authored).
- **Rust glue files**: `🧬️contract/📦️packages/🦀️rust/📦️glue.rs` and similar — bare filename
  literals (`"🦀️action.rs"`, `"📦️glue.rs"`) inside non-`mod`/non-`#[path]` contexts (e.g. test
  assertions, doc examples).
- **A repo-owned `📜️script.ts`** (the CLAUDE.md-mandated permanent script) carries a large literal
  lookup table of `🦀️component.rs` paths (27 rows) — likely a registry/manifest table, not an
  `import`.
- **Prose files**: `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md` (a live, non-ticket prompt document —
  *not* covered by `historicalDocumentEvidencePopulations`, which is ticket/`.cursor`-scoped only)
  and a Gherkin `.feature` test scenario mentioning asset paths in `Given`/`When` steps.
- **TS component files** (`📡️replication/🟦️component.ts`, `🛂️manifest/🟦️component.ts`, …) with
  bare-string fixture-directory or sibling-module references outside `import`.

Common thread: every one of these is a genuine reference to a real, currently-existing file (the
tool resolves a target for each), but the **adapter's structured parser doesn't recognize the
surrounding syntax as a provably-rewritable form** (an arbitrary string inside a config object
literal, a doc-comment example, a lookup-table entry, free prose) — so the *generic heuristic*
scanner (`unsupportedReferenceTokens`) surfaces it for review instead of silently rewriting it,
exactly as the mechanism is designed to do (§ goal-frozen-report: "deliberately permissive because
it exists to surface candidates for review, not to certify them").

**Disposition, per sub-family** (not implemented — this is genuinely per-adapter surface area, not
one fix):
- `🟨️frame-worker.js`: **generated** — the fix belongs in whatever bundles it, not in the output;
  do not touch the emitted file.
- `.dependency-cruiser.cjs` / `🧪️vitest.config.ts` / `📜️script.ts`'s lookup table: legitimate
  rewritable references in an unsupported *form* — the TS/JS reference adapter would need to learn
  a few more structurally-safe shapes (string literals inside a known config call's array/object
  argument, e.g. `resolve.alias`, `test.include`, `.dependency-cruiser.cjs`'s `forbidden[].from/to`,
  and a plain top-level array-of-strings literal like the `📜️script.ts` table) to parse and rewrite
  them exactly the way `import`/`require` already are. Concrete, bounded, but multi-shape work
  deserving its own pass with one test per shape — not attempted here to avoid a rushed, partially
  wrong adapter change to a heavily shared file this late in the session.
- `.🧬semio/🦑️repo/💬️prompts/🐙️ueli.md`: this is a **live prompt document**, not a ticket artifact —
  outside `historicalDocumentEvidencePopulations`'s scope by design (that population is
  ticket/`.cursor`-only). Two honest options: extend the population definition to a fourth,
  narrowly-scoped kind for `.🧬semio/🦑️repo/💬️prompts/*.md` (same "document kind, not lifecycle"
  reasoning as tickets), or teach the Markdown adapter to rewrite the reference like any other prose
  mention. Not decided here — a repo-wide decision about what `💬️prompts` is (live spec vs.
  narrative), not ours to make unilaterally.
- The Gherkin `.feature` file and the bare Rust glue-file literals: genuinely rewritable references
  in an unsupported form; same "teach the adapter one more safe shape" disposition as the TS config
  case.

### Family A3 — `unsupported-path-syntax`, ticket population (~153 rows) — split into two real sub-families

1. **Correctly-named, correctly-nested `📓️` reports that are STILL flagged, because their ticket
   root also holds a real, live Cargo package.** Example:
   `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️17/FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG/` has
   `Cargo.toml`/`Cargo.lock`/`🦀️main.rs`/`🦀️flow-pack-probe.rs` sitting **directly at the ticket
   root** (a real, buildable debugging crate, `procedural-ui-patch-probe`) alongside 30+ unrelated
   `📓️*.md` reports (some direct children, some one level down in
   `📥️worker-bootstrap/📓️*.md` — both shapes the population is supposed to admit).
   `ticketPackageBoundaryOwns` (`🧹️normalization/🟦️.ts`) walks from a candidate file's own
   directory up to the ticket root checking each directory's *own listing* for a package-manifest
   basename; for a **direct child**, that's one check — the ticket root itself — so if the ticket
   root directly contains `Cargo.toml`, **every** direct-child file is disqualified from
   `historicalDocumentEvidence`, not just the crate's own source. Verified: `procedural-ui-patch-probe`'s
   two `.rs` binaries are legitimately protected (their `path = "../../../…"` dependency lines are
   real, live references) — the bug is that the *unrelated* `📓️` reports next to them lose
   protection too, purely because they share a directory with a manifest that has nothing to do
   with them. `📜️script.ts` in the same ticket is correctly *not* exempt regardless (matches
   `fixedFilenameContracts`, same as `🎫️ticket.json`/`Cargo.toml` — working as designed, not part
   of this bug). Same shape recurs, smaller, in at least two other tickets whose roots hold stray
   build artifacts.
   **Disposition — propose, not implemented**: narrow `ticketPackageBoundaryOwns` so it only
   disqualifies a candidate file when the file's own extension is plausible source/config for the
   ecosystem that manifest actually declares (`.rs`/`Cargo.toml`/`Cargo.lock` for Cargo, etc.) —
   never for a `.md` report, which cannot be a Cargo/Node/Go source member under any manifest
   shape. This is the same core file (`🧹️normalization/🟦️.ts`) our rust-path-join fix already
   touched twice this session; a second edit to a different, also security-relevant boundary check
   is exactly the kind of change the ticket brief asks us not to rush. Diagnosis is complete and
   verified (real ticket, real manifest, real collateral files enumerated above); implementation is
   deliberately left as a precise follow-up.
2. **Reports genuinely outside both populations by document shape** — the already-known,
   already-dispositioned residue from `📓️goal-frozen-report.md` (giant/stalled 250MB `.md` files
   under `SEMANTIC-MUTATIONS-OVERHAUL`, absolute-path tokens, heading/indented/plain-prose spans the
   Markdown scanner deliberately doesn't admit) plus the **same non-`📓️`-prefixed nested-report
   shape found in classB** (`📓️wave1-reports/a1-framework-core-report.md` and four siblings under
   `UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, and the analogous `📓️wave2-reports/terrain-report.md` /
   `📓️wave3b-reports/surface-report.md` / `📓️wave5-reports/2d-store-deletion-report.md` under
   `DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS`, and `.py`/non-`.md` scratch files
   like `derive_obj.py`/`derive_ply.py` under `END-TO-END-TESTING-REFACTOR` sitting one level deep
   in a named subfolder — outside `ticket-workspace`'s direct-children-only `directoryPattern`).
   Same disposition as classB item 2 below — see there for the naming-vs-rule discussion.

## `frozen-coordinate-evidence-unowned` families

| family | rows | disposition |
|---|---:|---|
| `🧫️fixtures/🧪️remaining-package-purity-authority/🔣️.json` (production golden-census fixture) | ~80 | **fixed, proposed patch** — see below |
| Nested `.json` evidence snapshots, `INTERACTIVE-JOB-RUNTIME-REFACTOR` (open ticket, 2+ levels deep) | ~26 | naming/depth family — see below |
| `📓️wave1-reports/a1-framework-core-report.md` (`UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`, closed ticket) | ~5–9 | naming family — see below |

### Family B1 — the golden-census fixture (dominant, ~65-70% of the class) — FIXED, patch proposed

`🧰️framework/…/📦️packages/🟦️typescript/🧫️fixtures/🧪️remaining-package-purity-authority/🔣️.json` is
a **229-row, 241 KB genuine historical census snapshot** (verified: it records the exact pre-rename
`🖍️draw` editor-command paths from §11, i.e. it really is "what the census said before this
session's renames"), backing exactly one test —
`🧪️tests/🧪️historical-package-owner-identity/🟦️.test.ts` — which only ever asserted byte-identity
for `mappings[29]`'s five fields. `🔣️taxonomy.json`'s `remaining-package-purity-history-v1` entry
registers **only those 5** of the file's ~229×7 path-bearing cells. Once a file has *any* registered
frozen coordinate, `frozenEvidenceCoordinateAuthority` returns non-null and **every other candidate
token in that file that would otherwise need rewriting** is reported `frozen-coordinate-evidence-unowned`
instead of being silently rewritten (§10's "digest-bound coordinate authority" — a deliberate,
tested, file-wide governance gate, not a per-coordinate one). Because this fixture's `mappings[*][0]`
(`sourcePath`) column mentions real modules across the whole `🧰️framework` tree, *any* sufficiently
wide rename scope trips more of its untouched rows — this session's 2082-move
`🧰️framework/🔨️modules` scope tripped 40 distinct offsets (80 rows before de-duplicating a doubled
scan pass) across 39 of the 229 rows, all in column 0 except one in column 10 (`destinationPath`).

**Root cause is a genuine composition defect, not an engine bug**: the JSON family's freeze anchor
is a **whole-document** sha256, so protecting one row's byte-identity for one narrow test
incidentally locks the entire 229-row census against ever being auto-rewritten again, forcing manual
coordinate registration one collision at a time, forever, as future scopes touch new rows — the
mechanism working exactly as designed, applied to a fixture whose vast majority of content was never
meant to be under that governance.

**Fix designed and verified** (not applied — `🔣️taxonomy.json` is owned by another worker this
session): extend `remaining-package-purity-history-v1`'s `coordinates` with two more declarations,
using the family's existing `*`-wildcard pointer grammar instead of enumerating 229 rows by hand:
```json
{ "pointer": "/mappings/*/0", "kind": "source" },
{ "pointer": "/mappings/69/10", "kind": "destination" }
```
(drop the now-redundant explicit `/mappings/29/0`, since the wildcard already covers it — the family
throws on a duplicate pointer resolution otherwise). Column 0 is safe to wildcard unconditionally
(every one of the 229 rows has a non-empty string there, verified). Column 10 (`destinationPath`) is
**not** safe to wildcard — exactly one row (`mappings[200]`, `classifierRole: "unresolved"`) has
`destinationPath: null`, and the family's validator hard-throws on a non-string value anywhere the
wildcard reaches — so column 10 stays an explicit, narrow, one-row addition for the offset this
session's scope actually touched, not a standing promise to cover every future one.

Verified directly against the real function (`frozenCoordinateEvidenceCoordinates`,
`validateFrozenCoordinateEvidenceContracts`) and against the real file bytes on disk: the patch
validates, resolves to 234 coordinates, covers all 40 real offending offsets, and leaves row 29's
other four fields (3/4/5/6) untouched. See `🧪️tests/🧪️frozen-coordinate-wildcard-coverage/🟦️.test.ts`
(new, 5 tests, all passing) for the reproducible proof, including a negative test confirming a full
wildcard on column 10 would legitimately throw.

### Family B2 — nested ticket documents outside the current populations' exact shape

Two distinct sub-shapes, both **within the class B/A3 "ticket document, wrong shape" family**, not
new mechanisms:

1. **Leaf-naming**: `📓️wave1-reports/a1-framework-core-report.md` and 4 siblings
   (`a2-schema-composition-report.md`, `b1-spr-vcs-report.md`, `b2-store-composition-report.md`,
   `c1-plugin-composition-report.md`) live in a `📓️`-prefixed **folder** but their own **leaf**
   filenames don't carry the `📓️` prefix `ticket-report`'s `leafPattern` (`^📓️.+\.md$`) requires.
   These are cited by name, in prose, from **live production Rust docstrings**
   (`🏪️store/🦀️component.rs`, `🔌️plugin/🦀️component.rs`, `📡️replication/🎮️mutation/🦀️component.rs`)
   — e.g. `` /// `📓️wave1-reports/b2-store-composition-report.md`'s "Design decisions" `` — so
   renaming the leaves to add the `📓️` prefix (the taxonomy-conforming fix, matching every other
   report in the repo) requires updating those citing docstrings too, across files this ticket
   session does not own exclusively. Ticket is **closed**
   (`UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM`), so the files themselves are safe to touch; the blast
   radius is the handful of citing comments, enumerated via `grep -rn` (5 sites found). Not done
   here: touching four more production `.rs` files this late, for ~5-9 rows, was judged lower value
   than the two families we did fix, and CLAUDE.md's "you MUST manually fix all assets... all at
   once" argues for doing this together with the analogous
   `DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS` wave2/3b/5-reports family (Family A3
   item 2) in one pass, not piecemeal.
2. **Nesting depth**: `.json` evidence snapshots two-plus levels under an *open* ticket root
   (`INTERACTIVE-JOB-RUNTIME-REFACTOR/EVERY-TOOL-INTERACTIVE-JOB-MIGRATION/*.json`,
   `.../PHASE-2-RESUMABLE-JOB-AND-PROGRESS-PROTOCOL/*.json`,
   `.../RESUMABLE-FEM-JOB-GRAPH/*.json`) — `ticket-workspace`'s `directoryPattern` only admits
   **direct children** of the ticket root; this repo's own convention (including *this* ticket, see
   its own `📓️actor-bootstrap-declaration-launch-admission/` and sibling subdirectories at $T root)
   routinely nests scratch/evidence content one or more levels deeper. Confirmed via the existing
   test `🧪️historical-document-evidence/🟦️.ts`'s own explicit case
   (`".../nested/terra-number-deasync.py"` asserted `false`) that this is a **deliberate, tested**
   design choice from an earlier slice, not an oversight — so widening it is a real design reversal,
   not a bug fix, and needs the ticket coordinator's sign-off rather than a unilateral change here.
   **Disposition: flagged, not changed.** If widened, `ticket-workspace`'s existing negative guards
   (never a `fixedFilenameContracts` match, never inside a package boundary) already generalize to
   any depth with no extra work — the only open question is policy (how deep is "still workspace,
   not a smuggled real source tree"), not mechanism.

## Not investigated further

`semantic-stem-unresolved` (411/449), `package-implementation-destination-unresolved` (~95–117),
`semantic-stem-ambiguous` (~84–87), `directory-kind-unresolved` (~31–33), the `collision-*` rows and
`generator-preview-invalid` — all outside our two assigned classes (`reference-syntax-unsupported`,
`frozen-coordinate-evidence-unowned`), left to whichever slice owns them.
