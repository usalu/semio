# reference-syntax-unsupported / frozen-coordinate-evidence-unowned — Report

Full census with families, examples and per-family disposition: `📓️goal-refsyn-census.md` (same
folder). This file is the required-format summary: what changed, what was verified, what remains.

## Verification (real, pasted output)

```
B=$(git rev-parse HEAD)   # bb06c41f73f0122fbed315b7487428b976f99921 — this ticket's fixed baseline
bun ./📜️script.ts clean taxonomy plan --ticket 26/08/17/END-TO-END-TAXONOMY-NORMALIZATION \
  --scope "🧰️framework/🔨️modules" --baseline "$B" \
  --plan "$T/🗑️temp/🔣️refsyn-plan.json" --workers 8

[clean taxonomy plan] moves=2158 roots=0 relocations=0 symlinks=0 removals=1 edits=4170 regenerations=9 unresolved=1193
```

| | handed off | after our work |
|---|---:|---:|
| `moves` | 2082 | 2158 |
| `reference-syntax-unsupported` | 397 | 439 |
| `frozen-coordinate-evidence-unowned` | 109 | 125 |
| `collision-*` (5 codes) | 220 (44×5, seen on our first read) | 0 |

**Neither target class visibly shrank — say so plainly, and why.** This is a live, four-worker
session; `🔣️taxonomy.json` is owned by another worker and both our concrete fixes for these classes
are *taxonomy-data* changes we are not allowed to apply ourselves this session:

- **rust-path-join** (155 rows, dominant): the actual code fix (below) is real, tested and landed in
  `🧹️normalization/🟦️.ts`, but the specific 16-file population that dominates this count all call a
  second, genuinely-suspicious construct (a real macro invocation) that the fix correctly leaves
  alone — so this population's row count is unchanged by design. See census Family A1 for the
  precise, verified mechanism and the concrete (unimplemented) follow-up that would very likely
  clear it.
- **the fixture family** (~80 of the 125 `frozen-coordinate-evidence-unowned` rows): fix is a
  2-line addition to `🔣️taxonomy.json`, designed and verified against the real file and the real
  validator/loader functions (5 tests, all passing), but **not applied** — not our file to edit.
  Exact patch is in the census under Family B1; ready for the taxonomy.json owner to drop in.
- `collision-*` hitting exactly 0 is **not our work** — another worker's fix, landed between our
  first read and this run. Flagging per the brief rather than claiming it.
- The `397→439`/`109→125` rise is consistent with `moves` rising `2082→2158`: more of the plan
  surfaced as other workers' concurrent moves/edits landed, not a regression from anything here.

## What was actually changed (2 files, both compile, both test suites pass)

**`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts`** — exported the
existing, already-tested Rust tokenizer `rustTokens` (was module-private). No behavior change.

**`🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts`** — added
`rustCodeOnlyText()` (reuses the tokenizer to strip comments) and used it in
`rustFiniteManifestTargets`'s ancestor-attribute/macro guard, instead of testing raw file text. Root
cause: every plugin's crate-root `📦️glue.rs` opens with a `//!` doc comment (this repo's own
mandated docstring convention), and the guard's `/[#!]/u.test(...)` matched the bare `!` in the `//!`
comment marker itself — a false positive completely unrelated to the guard's real purpose (catching
macros/`std`/`env`/glob-imports that could hide dynamic module structure from the static prover).
Purely subtractive on the guard's input (comments never affect Rust compilation) — can only remove
false positives, never admit a file the guard previously, correctly rejected.

Diagnosed by instrumenting every early-return guard in `rustFiniteManifestTargets` directly (per the
brief's own instruction), not by hypothesis: confirmed guards #1–#3 (§8's suspects) do **not** fire
today; the real culprit is deeper, in the per-ancestor attribute/macro scan. Full guard-by-guard
trace is in the census; instrumentation was removed before finishing (verified via grep, none left).

## Tests (both new, both fail-before/pass-after verified)

- `🧪️tests/🧪️rust-finite-target-consumption/🟦️.ts` — added case `ancestor-doc-comment-noise`
  (condition `parent-doc-comment`): a crate root whose only "suspicious" content is a `//!` doc
  comment mentioning `!important`/`#heading` in prose. Confirmed it fails without the fix (both Bun
  and TypeScript compiler oracles: `physicalInterpretation` comes back `undefined` instead of
  `"rust-finite-manifest-targets"`) and passes with it. Also extended the strict-TypeScript
  declarations test and the sandboxed-extraction harness (`functions`/`dependencies`/schema `enum`
  + `minItems`) to cover the new dependency. 76/76 tests pass.
- `🧪️tests/🧪️frozen-coordinate-wildcard-coverage/🟦️.test.ts` (new directory) — proves, against the
  real fixture bytes and the real `frozenCoordinateEvidenceCoordinates`/
  `validateFrozenCoordinateEvidenceContracts`, that (a) today's 5-coordinate registration leaves all
  40 real offending offsets unowned, (b) the proposed 2-coordinate patch resolves all 40 without
  touching row 29's other four fields, (c) a *full* wildcard on the destination-path column would be
  unsound (one row has a null value there) — the reason that column stays a narrow, explicit
  addition instead of a second wildcard. 5/5 pass.
- Ran the neighbouring `🧪️rust-writable-path-authority`, `🧪️rust-physical-reference-context`,
  `🧪️rust-divergence-callback`, `🧪️historical-document-evidence` and
  `🧪️historical-package-owner-identity` suites as regression checks: all pass except 2 pre-existing
  failures in the last one (`"has no exact unescaped JSON value span"` expected vs `"...declared
  JSON value encoding"` actual) — confirmed unrelated: different code path
  (`frozenCoordinateEvidenceCoordinates`'s error text), not touched by us, message text already
  read exactly as the failing test expects-it-not-to when we first opened the file, so it's drift
  from a concurrent worker's edit to that message, not a regression we introduced.

## Diagnosed, not implemented (see census for full detail + exact reasoning)

- **`plugin_exports!` macro allowlist** — would likely clear most of the remaining rust-path-join
  rows; macro body verified (grep) to contain zero `mod` tokens, so provably transparent to the
  module-graph proof. A second, judgment-heavy edit to the same guard; left as a precise follow-up
  rather than a third same-session hypothesis in this exact area.
- **Ticket-embedded-package collateral blocking** — a real Cargo debug crate sitting at a ticket
  root (`FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG`) disqualifies *every* direct-child file, including
  30+ unrelated `📓️` reports, from historical-evidence protection, because
  `ticketPackageBoundaryOwns` doesn't distinguish "shares a directory with a manifest" from "is
  plausible source for that manifest's ecosystem." Concrete fix sketched (extension-based
  narrowing); not applied — another edit to the same contested file, for a policy question
  (how narrow should the ecosystem-extension list be) better answered with the coordinator.
- **Non-`📓️`-prefixed nested reports** (`📓️wave1-reports/a1-framework-core-report.md` and ~8
  siblings across two closed tickets) — fixable by renaming to add the `📓️` leaf prefix, but 5
  citing docstrings in live production `.rs` files need updating in lockstep; deferred to be done
  together with the sibling `wave2/3b/5-reports` family rather than piecemeal.
- **Ticket-workspace nesting depth** — confirmed via the existing test's own explicit case that
  "direct children only" is a deliberate, already-tested design choice from an earlier slice, not
  an oversight. Flagged as a policy question (this ticket's own subfolders do the same nesting),
  not changed unilaterally.

## Files touched

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔍️discovery/🟦️component.ts` (export only)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🟦️.ts` (real fix, guard#8)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️rust-finite-target-consumption/🟦️.ts`,
  `🔣️.json`, `🧬️schema/🔣️.json` (new case)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧪️frozen-coordinate-wildcard-coverage/🟦️.test.ts` (new)
- `🔣️taxonomy.json` — **not touched**, patch proposed in census Family B1 for its owner.
- This ticket's `📓️goal-refsyn-census.md`, `📓️goal-refsyn-report.md` (this file).
- `🗑️temp/` scratch: debug/verification JSON plan artifacts and a throwaway TS locate script,
  deleted before closing out (see below).

---

# Follow-up: taxonomy.json patch landed, macro allowlist landed, mesh signal measured

Coordinator freed `🔣️taxonomy.json` and asked for the wildcard patch to land synchronously, plus
the `plugin_exports!` follow-up and a `🔺️mesh` before/after as the bulk-apply go/no-go signal.

## Landed

- **`🔣️taxonomy.json`**: applied exactly the census's proposed patch to
  `frozenCoordinateEvidenceContracts.remaining-package-purity-history-v1` — dropped the now-redundant
  `/mappings/29/0`, added `/mappings/*/0` (source) and `/mappings/69/10` (destination). `bun -e`
  against `loadCatalogTaxonomy()`/`validateTaxonomy()`: 0 problems. Updated
  `🧪️frozen-coordinate-wildcard-coverage` (now asserts the *live* shape, not a hypothetical one) and
  fixed the one sibling test this broke (`🧪️historical-package-owner-identity`'s "no broader span
  ownership" — rewrote it to assert the 4 identity-test fields are untouched and the widening is
  exactly the 2 new entries, rather than exact-equality with the old 5). Both green.
- **`plugin_exports!` macro allowlist**: added `RUST_MODULE_STRUCTURE_TRANSPARENT_MACRO_INVOCATIONS`
  + `rustCodeOnlyTextForMacroTrust` in `🧹️normalization/🟦️.ts` — strips this one exact,
  fully-qualified, framework-owned macro invocation (verified present, identically, in all 32
  plugins' `📦️glue.rs`; verified by reading its full ~180-line `macro_rules!` body that it contains
  zero `mod` tokens, so it can never expand to hide module structure) before guard#8's
  attribute/macro scan. New regression case `ancestor-known-transparent-macro` in
  `🧪️rust-finite-target-consumption` (fails before, passes after — verified both ways). No other
  macro is treated any differently than before.

## Measured (real output)

```
🧰️framework/🔨️modules (full scope): unresolved 1193 → 1104
  frozen-coordinate-evidence-unowned: 125 → 37   (-88, the wildcard patch, confirmed live)
  reference-syntax-unsupported / rust-path-join: 155 → 155  (UNCHANGED — see below)

🧰️framework/🔨️modules/🔺️mesh (coordinator's go/no-go scope): unresolved 50 → 50 (UNCHANGED)
```

**`🔺️mesh`'s floor is not gone. Do not bulk-apply on this yet — say so plainly.** Re-instrumented
guard#8 on a mesh-blocking file (`✒️writer`) after landing the macro fix: it still fires, now for a
**different, more legitimate** reason than `plugin_exports!` — every one of the 33 plugins'
`📦️glue.rs` also contains multiple `#[cfg(test)] mod tests_*` blocks (confirmed: `grep -l` finds it
in all 33, zero exceptions) declaring sibling test modules unrelated to the mutation-root chain the
prover is trying to verify. Unlike a comment or a macro proven to emit no `mod` items, `#[cfg(...)]`
**can** legitimately gate whether a `mod` compiles at all — that is exactly the class of thing this
guard exists to catch, and I have not proven (the way I proved the comment and the one macro) that
ignoring it here is safe. The 155-count for this message kind is dominated entirely by this
population; whether it clears requires either accepting `#[cfg(test)]` on a *sibling, unrelated*
module is safe to ignore (a real design call, not mine to make unilaterally) or a per-declaration
check that only distrusts the guard for attributes actually on the proof chain's own modules (a
bigger, non-trivial rework). Flagging precisely rather than stripping it to force the row count down.

Also added, low-risk and unrelated to the count: nothing further attempted on the two
diagnosed-not-implemented items (ticket-embedded-package collateral blocking; non-`📓️`-prefixed
nested reports) — no time left in this turn after the above; both still fully diagnosed in the
census with exact reasoning, ready for a dedicated pass.

## Full regression check

`🧪️rust-finite-target-consumption` (78/78), `🧪️frozen-coordinate-wildcard-coverage` (5/5),
`🧪️historical-package-owner-identity` (24/26 — 2 pre-existing, unrelated, confirmed already failing
before this turn), `🧪️historical-document-evidence` (6/6), `🧪️rust-physical-reference-context`
(32/32). `🔣️taxonomy.json` validates with 0 problems via `validateTaxonomy`.

---

# Follow-up 2: cfg-gated mods included in the proof chain — landed; a new, different guard found underneath

Implemented exactly the coordinator's call: `#[cfg(...)]`/`#[cfg_attr(...)]` no longer disqualify an
ancestor. Checked against the code first, as asked: `inspectRustModuleGraphFacts`
(`🔍️discovery/🟦️component.ts`) already records a cfg-gated `mod` as an ordinary module fact
(`conditional: true`, never dropped), and the proof loop never inspects that flag — so a cfg-gated
`#[path]` mod was ALREADY a complete participant in the actual proof. Only guard#8's trust-scan
distrusted the *ancestor file* for merely containing `#[cfg(...)]` text. Fix:
`rustCodeOnlyTextForMacroTrust` (`🧹️normalization/🟦️.ts`) now also skips `#[cfg(...)]`/`#[cfg_attr(...)]`
spans (via the newly-exported `rustTokenPairs`, so nesting/strings are handled correctly, not a
fragile regex). While there, also added `#[allow(...)]`/`#[derive(...)]` — language-guaranteed inert
w.r.t. compilation/module structure for any argument, unlike `cfg`, so no per-case proof was needed.
Verified this cleared every non-cfg, non-path attribute actually present across all 33 plugins'
`glue.rs` (`cfg` 725, `allow` 43, `derive`/`dyn_enum` — both comment-only mentions, already inert).
New test `ancestor-cfg-gated-test-mod` in `🧪️rust-finite-target-consumption`: fails before, passes
after, both directions verified.

**Found the next, real guard underneath — different in kind, not yet proven safe.** Re-instrumented
after landing the above: `✒️writer`'s `glue.rs` now passes guard#8 cleanly (hashbang=false,
macro=false), but `rustFiniteManifestTargets` still returns empty because `parentImports` is true
(the mutation-root test file itself has `use super::*;`) AND the ancestor has a glob re-export
(`pub use component::*;`, present throughout every `glue.rs`'s nested wiring). This is the
`facts.get(source)!.uses.some(use => use.specifier.includes("*"))` branch — a genuinely different
concern (glob-import name-resolution ambiguity, not module-graph completeness) that I have not
verified is safe the way I verified `plugin_exports!`, `cfg`, or `allow`/`derive`. Not touched.

## Measured (real output)

```
🧰️framework/🔨️modules (full scope): moves=2156 unresolved=1104   (both unchanged from the last report)
🧰️framework/🔨️modules/🔺️mesh:       unresolved 50 → 50            (unchanged — same digest before/after)
```

**Go/no-go: still no-go on `🔺️mesh`.** The floor did not move because the ACTUAL universal blocker
for these 16 mutation-root files was never purely `#[cfg(test)]` — it's `use super::*;` in the test
file combined with `pub use component::*;` in every plugin's glue wiring. Next step, precisely: is a
target file's `use super::*;` plus an ancestor's `pub use X::*;` re-export actually unsafe for this
proof's purposes, or is it another case (like cfg) where a downstream mechanism already resolves it
completely and the guard is stale? I have not read that code path yet — flagging rather than guessing
a fourth time in the same guard.

Full regression: 147/149 pass (2 pre-existing, unrelated, confirmed before this turn).

---

# Follow-up 3: glob-import veto removed — mesh 50 → 1

Verified against code before touching it: `inspectRustManifestPathReferences`/
`inspectRustManifestPathCandidates`/`inspectRustJoinArgumentSpans` all require a string literal (or
a loop bound to string literals) to ever record a join argument — a glob import cannot inject a fake
literal into source text, so it structurally cannot hide a non-literal join from these extractors
regardless of the veto. Confirmed zero non-literal `.join(...)` calls exist in the 16 mutation-root
files or their `glue.rs` ancestors (443 literal joins, grep-verified). Removed only the glob-import
disjunct from guard#8 (`facts.get(source)!.uses.some(use => use.specifier.includes("*"))`); the
`std`/`env` half of that same condition is untouched. New test (`ancestor-glob-reexport-with-super-
star-target`): `use super::*;` in a nested test mod (matching the real files' own shape — a bare
top-level `use super::*` hits a *different*, pre-existing wildcard-bail inside
`inspectRustManifestPathCandidates` itself, unrelated to this guard, confirmed while building the
fixture) + an ancestor `pub use reader::*;`. Fails before, passes after, both directions verified.

**Note, not acted on**: `inspectRustJoinArgumentSpans`/`inspectRustManifestPathCandidates` silently
drop (never flag "unproven") a `.join(...)` argument that is a non-literal, non-loop identifier —
neither function emits any row for it. Pre-existing, orthogonal to this fix (confirmed unaffected by
it either way), not something I introduced or made worse. Flagging since you asked me to check for
exactly this risk.

## Measured (real output)

```
🧰️framework/🔨️modules/🔺️mesh:  unresolved 50 → 1     (GO signal, but not 0 — see below)
🧰️framework/🔨️modules (full): moves=2156 unresolved=1104 → 961   (-143)
```

**Not quite 0.** The 1 remaining row is a **different file, different cause**:
`✏️s/🔌️plugins/🗄️stdio/…/📚️examples/🏛️architectural/🧪️tests/🦀️test.rs`, still `rust-path-join`. Not
one of the original 13-16 mutation-root files. Have not instrumented it — out of time this turn, and
it looked like a different, unverified macro (`#[semio_framework_async_macros::async_test]`) rather
than any of the three things fixed today. Not guessed at; reporting the file and stopping there.

No new guard surfaced under this one beyond what's already named above.

---

# Follow-up 4: async_test attribute-path landed — mesh 1 → 1 (not 0, different file blocks it)

Verified the coordinator's proof against the code: `expand_async_test`
(`🧰️framework/🔨️modules/⏳️async/✨️macros/🦀️component.rs:26-73`) has zero `mod` tokens and parses
input as `syn::ItemFn` rejecting non-async/generic/parameterized fns, so it cannot structurally
emit one. Confirmed the trap: `mod tests {` at line 80 is the macro crate's *own* test module,
outside `expand_async_test` entirely (its function body ends at line ~75, well before line 80).

Implementation note: `#[semio_framework_async_macros::async_test]` is an ATTRIBUTE macro
(`#[path]`, no `!`, no parens, multi-segment path) — a different shape from `plugin_exports!`'s
statement-level `name!(args)`. Extended the mechanism with a new, generically-checked
`RUST_MODULE_STRUCTURE_TRANSPARENT_ATTRIBUTE_PATHS` (exact argument-free attribute paths, skipped
via `rustTokenPairs` bracket-matching same as `cfg`/`allow`/`derive`) rather than reusing the
macro-invocation string-replace, since that shape assumption (a `!`) doesn't hold here. New test
`ancestor-known-attribute-path`: fails before, passes after, both directions verified.

**`🔺️mesh`: still 1, unchanged — digest identical to before this fix.** The one remaining row moved
to a *different* file mid-investigation, unaffected by async_test: `✏️s/🔌️plugins/🗄️stdio/…/🧪️tests/🦀️test.rs`,
blocked by its own crate root `✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/📦️glue.rs`, which defines its
**own** `macro_rules! impl_serde_op_codec { … }` plus real `format!`/`unreachable!` invocations —
a plugin-specific macro I have not read the body of and have not verified is `mod`-free. Not
touched; this is exactly the "read the macro before trusting it" step, not yet done for this one.

## Measured (real output)

```
🔺️mesh:                unresolved 1 → 1   (NOT 0 — see stdio/glue.rs finding above)
🧰️framework/🔨️modules:  moves=2155 unresolved=961   (unchanged from Follow-up 3 — the one blocked
                                                       row moved file, net count didn't shift)
🖥️platform:  unresolved 51 → 2
⏳️async:     unresolved 52 → 3
🎯️action-bus: unresolved 84 → 35   (dominated by 32 `frozen-coordinate-evidence-unowned` on the
                                     SAME purity-authority fixture — rows outside our wildcard's
                                     column-0/row-69 scope, exactly the residual we already
                                     documented as deliberately not wildcarded on column 10)
```

No new guard beyond the stdio `macro_rules!` finding above.

**Recorded, not fixed — the known-remaining item from Follow-up 3:** `inspectRustManifestPathCandidates`
(`🔍️discovery/🟦️component.ts`) has its own, separate wildcard-bail: it returns `[]` for the entire
file whenever a bare (not `super::`-prefixed-and-depth-matched) glob `use` sits at a nesting depth
`moduleEnds.length` below the number of `super::` segments — concretely, a **top-level**
`use super::*;` (zero enclosing `mod {}` blocks) is treated as a real wildcard-macro-import hazard
and blanks the whole candidate set, while the *same* `use super::*;` one level deeper (inside a
`mod tests { … }`, matching every real file we've seen) is correctly recognized as `localParent` and
ignored. This is unrelated to the guard#8 mechanism fixed today (a different function entirely) and
was not touched. Discovered while building the `parent-glob-reexport` fixture; writing it down now
so it isn't rediscovered.

---

# Follow-up 5: ticket-workspace widened to nested depth; 🖼️assets vocabulary registered

## Nested ticket-workspace (Family A3 item 1 mechanism, item 2 depth policy)

`historicalDocumentEvidencePopulations.ticket-workspace.directoryPattern` widened from
`.../*/*` (direct children only) to `.../*/**` (any depth), mirroring `ticket-report`'s own
pattern shape exactly. Updated the matching hardcoded validator string in `🔍️discovery/🟦️component.ts`.
Both existing negatives are untouched and still fire — verified directly:
`.🧬semio/…/EMBEDDED/embedded-pkg/lib.rs` still returns `false` (still live) in the re-run integration
test (`🧪️historical-document-evidence`, "ticket narrative … a ticket-embedded package boundary is
never swept in", 6/6 pass, debug output still lists it under `blocked`).

**`📜️script.ts` distinguished, precisely**: `fixedFilenameContracts.root-script`'s `pathPattern` is
`**/📜️script.ts` — a double-star glob, so it matches a ticket-embedded copy exactly as it matches
the repo-root one. Verified directly (`pathMatcher.matches`): **both** return `true`, meaning the
existing "never exempt a fixedFilenameContracts match" negative excludes **both** identically — a
ticket's own `📜️script.ts` was never gated by directory depth, it is gated by matching this
contract, unconditionally, regardless of location. This directly contradicts my honest expectation
in Follow-up 3 that it would become exempt — it does not, and should not: CLAUDE.md's "permanent
command router" contract for that exact filename is deliberately global.

**Caveat confirmed real, not fixed**: `FIX-DEMONSTRATOR-END-TO-END-BOOT-HANG` has `Cargo.toml`
directly at its ticket root (a real, buildable debug crate) — `ticketPackageBoundaryOwns` disqualifies
its **direct-child** `📓️` reports too (pre-existing, my originally-diagnosed item, unrelated to
depth). So the biggest 3 files in the coordinator's list (56 of 143 rows) do **not** clear from this
fix alone. `INTERACTIVE-JOB-RUNTIME-REFACTOR` (no embedded package) verified clearing correctly.

## 🖼️assets vocabulary (3 families → became 5 registrations once verified against real files)

Traced `canonicalFile`'s exact resolution path rather than guessing:
- `📃️list` and `🪧️logos` resolve via the `members-of-assets` overlay (no exact kind owns their
  emoji+slug); `🔤️fonts` resolves to the literal `"fonts"` kind (exact id/slug match pre-empts any
  overlay) — so the font-family fix needed `parentKindIds: ["fonts"]`, not `"members-of-assets"`.
- Widened `asset-subject.parentKindIds` to `["assets", "members-of-assets"]` — this alone cleared
  every **emoji-less** stem (all the `.json` files: file-kind's own `🔣️` marker is stripped before
  semantic-stem extraction, leaving no leading emoji, so resolution goes through the no-emoji
  inference path).
- `📋️licenses.csv`, `📝️tags.txt`, `🎥️logo.mp4` carry **their own**, non-canonical semantic-evidence
  emoji (not each file-kind's own marker: `.csv`→📊️, `.txt`→🔤️, `.mp4`→🎬️) — these survive as
  *evidence* and go through the exact-emoji branch, unresolved by `asset-subject` alone. Added three
  narrow kinds (`asset-table-subject` 📋️, `asset-note-subject` 📝️, `asset-video-subject` 🎥️),
  `inferWithoutEmoji: false` on all three (**required** — without it they also entered the no-emoji
  pool and made every `.json` stem ambiguous against 4 candidates at once; caught and fixed via a
  real before/after plan diff, not assumed). `role: "asset"` file kinds (csv, mp4) resolve through
  the literal `"assets"` context, not `"members-of-assets"` — table-subject/video-subject need
  **both** in `parentKindIds`; note-subject (txt, role `documentation`) only needed the latter.
  Added `font-family` (😀️, `parentKindIds: ["fonts"]`) for `😀️noto-emoji`.

## Measured (real output)

```
🖼️assets scope:          moves=1074→1089, unresolved=32→16   (all vocabulary cleared; remaining
                                                                16 are reference-syntax-unsupported,
                                                                a separate, already-documented family
                                                                — NOT 0, said plainly)
🧰️framework/🔨️modules agg: moves=2177, unresolved=877  (was 961 before nested-ticket, 901 after it,
                                                          877 after assets vocabulary)
🔺️mesh / 🖥️platform / ⏳️async: unchanged (1/2/3) — their blockers are the stdio macro_rules!/other
                                                     plugin issues, untouched this round, not ticket-
                                                     or assets-related
```

taxonomy.json validates (`validateTaxonomy`: 0 problems) after every edit in this round.
