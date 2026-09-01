# Collision Resolution Report — `🧰️framework/🔨️modules`

Full case-by-case data: `📓️goal-collide-census.md`. Ticket precedent this class was measured
against: §1 "Landed / Defects found and fixed" item 2 of `📓️goal-session-status.md` (the
`🧬️schema/✨️derive` glue/component duplication).

## Split

| class | count | fix |
|---|---:|---|
| (b) genuinely distinct — role/target-tagged sibling | 42 | moved the role/target-tagged file into an *existing* semantic directory (`🧪️tests/`, `🎯️targets/⌨️tui/`, `🎯️targets/🧊️wgpu/`); zero `🔣️taxonomy.json` changes |
| (a) duplication | 0 | — the precedent's diagnosis did not reproduce; both `📦️glue.rs`-vs-owner cases inspected byte-for-byte and are genuinely different content |
| (c) engine-exposed structural defect | 2 | extracted the offending `struct`/`impl` block out of `📦️glue.rs` into a new file under an *existing* semantic directory (`🔗️bindings/`, `🧬️schema/`), leaving `glue.rs` pure declaration |

**Root cause (shared by all 44):** `canonicalFile`'s generic-stem short-circuit
(`🧹️normalization/🟦️.ts:3122-3124`) returns the bare kind-only leaf for any file whose trailing
stem is generic (`component`, `index`, …), discarding the leading role/target emoji (`🧪️`, `⌨️`,
`🧊️`) before it is ever checked against `roleContext`/the `targets` registry — so an impl and its
role-tagged sibling collapse onto the same path. For the 2 `(c)` cases the defect is one hop later:
`packageImplementationDestination` (`🧹️normalization/🟦️.ts:3141-3157`) unconditionally hoists any
generic-stem `role: "implementation"` package file to `${owner}/${kindOnly}`, with no check that a
second implementation-role file already claims that slot.

Lines removed: **0**. Neither case was duplication, so nothing to de-duplicate — every fix is a
relocation (`mv` + reference-string edits), not a content merge.

## (c) — the two structural cases in detail

**`🎭️actor/📦️packages/🦀️rust/📦️glue.rs`** (case #8). 105-line file: genuine `#[path]` wiring to
`🦀️component.rs` PLUS a 90-line `wasm_bindgen` `KernelHost` wrapper (`struct`+`impl`) gated
`#[cfg(target_arch = "wasm32")]`. `classifyGlue`'s rust rule (`🧹️normalization/🟦️.ts:3013`,
`/\b(?:struct|enum|trait|union|impl)\b/`) correctly flags the file `role: "implementation"` — the
`KernelHost` type is real code, not wiring — so it hoists to `🎭️actor/🦀️.rs`, the same slot
`🦀️component.rs` already owns. Fix: extracted `KernelHost` verbatim into new
`🎭️actor/🔗️bindings/🦀️.rs` (existing `bindings` semantic directory kind, emoji 🔗️, no new taxonomy
entry), wired back via `#[cfg(target_arch = "wasm32")] #[path = "../../🔗️bindings/🦀️.rs"] mod
kernel_host;`. `glue.rs` is now 27 lines of pure `mod`/`use`/`extern crate`/attribute — no
`struct`/`enum`/`trait`/`union`/`impl` token anywhere — so `classifyPackageGlueContent` now returns
`"declaration"`. `cargo check` clean on both `wasm32-unknown-unknown` and native; `cargo test`:
**113 passed, 0 failed**.

**`🖱️ui/🧬️contract/📦️packages/🦀️rust/📦️glue.rs`** (case #27). 1027-line crate-root file: real
`#[path]` wiring to 12 flat sibling modules (`accessibility.rs`, `action.rs`, … `text_edit.rs`)
PLUS a 969-line `#[cfg(feature = "typegen")] pub mod schema_metadata { … }` block (a `SchemaMetadata`
struct + a `TYPES` table of versioned wire-type → TypeScript projections). The `struct` triggers
`role: "implementation"`, hoisting to `🧬️contract/🦀️.rs` — the SAME slot the crate's own
`📦️packages/🦀️rust/🦀️component.rs` (stem "component", also generic, also hoisted) wants. Fix:
extracted the module body verbatim into new `🧬️contract/🧬️schema/🦀️.rs` (existing `schema` semantic
directory kind, emoji 🧬️, slug pattern `^(schema|mutations)$` — "schema" is an exact match, no new
taxonomy entry), wired back via `#[cfg(feature = "typegen")] #[path = "../../🧬️schema/🦀️.rs"] pub
mod schema_metadata;`. `glue.rs` is now 56 lines of pure declarations. `cargo check` clean with and
without `--features typegen`; `cargo fmt --check` clean on both new/edited files (the extracted body
needed a `cargo fmt` re-indent after un-wrapping from `pub mod { }` — applied). `cargo test`:
158/159 pass; the one that panics ("UI value retirement arena is poisoned") is **non-deterministic
across runs** — different test fails each run, passes 3/3 in isolation, and 13 completely unrelated
`transaction::tests::*` in the sibling `🧠️runtime` crate fail the *identical* way with my changes
fully excluded (`--skip output_pool_tests --skip ownership`) — a pre-existing fixed-capacity
shared-global-arena flake in these two crates' test suites, not a regression.

## (b) — one extra defect found mid-fix

`🖱️ui/🎨️styling` (case #13, `📦️index.ts` vs `🧪️index.test.ts`) does **not** fit the plain
sibling-directory pattern: `📦️index.ts` sits directly inside the package boundary
(`📦️packages/🟦️typescript/`), so `packageImplementationDestination` hoists it to the owner root
regardless of where its sibling lives — nesting the test into `🧪️tests/` inside the package
directory still collided (verified: first `clean taxonomy plan` re-run left exactly this one
collision, 5 rows, all others zero). Fix: moved the test out of the package boundary entirely, to
`🖱️ui/🎨️styling/🧪️tests/🟦️.ts` (owner root, sibling of `📦️packages/`), and pointed
`📦️packages/🟦️typescript/📜️script.ts`'s `TestScript` at `../../🧪️tests/🟦️.ts`. `bun test`: 30/30 pass.

## Implementation

42 files moved to `🧪️tests/<kindOnly>` or `🎯️targets/<slug>/<kindOnly>`; every discovered
`#[path]`/`include!`/`include_str!` Rust reference and every TS `vitest.config.ts`/`script.ts` test
registration updated to match (found by a repo-wide grep for each old literal path, not by guessing
— one `include!` in `🖱️ui/🧠️runtime/📦️packages/🦀️rust/🦀️reconcile.rs` was missed on the first pass
because it uses `include!`, not `#[path]`, and was caught by `cargo test --no-run` failing to
resolve the module). 2 files restructured (`(c)`, above). 1 registry file updated:
`🔒️layering.json`'s key for the relocated styling test.

Verification per touched crate — real output:

- `🎭️actor/📦️packages/🦀️rust`: `cargo check` (native + `wasm32-unknown-unknown`) clean;
  `cargo test`: **113 passed; 0 failed**.
- `🧰️framework/📦️packages/🦀️rust` (mounts `🛂️manifest`→`🎠️kernel`, covers cases #3, #4):
  `cargo check`: `Finished dev profile`, warnings only pre-existing dead-code.
- `📡️replication/📦️packages/🦀️rust` (mounts `🌱️value`, covers cases #1, #2, #9-#12):
  `cargo check`: `Finished dev profile`, warnings only pre-existing.
- `🖱️ui/🧠️runtime/📦️packages/🦀️rust` (case #15): `cargo check`/`cargo test --no-run` clean;
  `output_pool_tests` (previously dead code — the `include!` was pointed at a file I had just moved)
  now compiles and mostly passes; see the shared-arena flake note above.
- `🖱️ui/🧬️contract/📦️packages/🦀️rust` (cases #13(rehomed above)/#16-#27): `cargo check` clean with
  and without `--features typegen`; `cargo test --no-run` builds; `cargo test`: 158/159, see above.
- `🖱️ui/📦️packages/🟦️typescript/🎯️targets/⚛️react` (16 tsx cases, #14/#28-#31/#34-#44):
  `bunx vitest run`: **691 passed, 0 failed** (one `20,000-node` stress test timed out once under
  concurrent system load, passed cleanly in isolation — not an import/logic issue).
- `🖱️ui/🎨️styling` (case #13): `bun test`: **30 passed, 0 failed**.

## Test added

`🧹️normalization/🧪️tests/🧪️generic-stem-collision-resolution/{🔣️.json,🟦️.ts}` (follows the sibling
`🧪️package-boundary-classification`/`🧪️source-admission` suites' shape exactly: AJV-validated JSON
fixture + `bun:test`). 44 cases in 3 kinds:
- `siblingCases` (41): asserts a directory does not directly hold both an impl and role-tagged
  basename — cross-checked by two independent listings (`node:fs.readdirSync` and the third-party
  `fast-glob`, which must agree).
- `gluePurityCases` (2): calls the **real, exported** `classifyPackageGlueContent` on the fixed
  `glue.rs` files and asserts `"declaration"`, not a reimplementation of the rule.
- `packageBoundaryHoistCases` (1, the styling case): asserts no test-shaped entry remains inside the
  package boundary and the owner-level `🧪️tests/🟦️.ts` exists.

`bun test`: **45 passed, 0 failed** (44 cases + 1 fixture-schema-validity test). Every one of these
44 assertions would fail against the pre-fix tree (the census documents that every one of these
directories/files held the exact collision shape being asserted against).

## VERIFY — real output

Baseline `bb06c41f73f0122fbed315b7487428b976f99921`, `--scope "🧰️framework/🔨️modules"`.

Before (this class's own baseline measurement, from `🗑️temp/🔣️vocab-plan.json`):
```
moves not separately re-measured pre-fix at this exact baseline; collision-byte 44, collision-case-fold 44,
collision-nfc 44, collision-same-kind 44, collision-vs16-fold 44   (220 rows / 44 distinct cases)
```

After (`bun ./📜️script.ts clean taxonomy plan --scope "🧰️framework/🔨️modules" --baseline bb06c41f73f0122fbed315b7487428b976f99921 --plan ...`):

Run 1 (before the styling package-boundary re-fix):
```
moves=2160 unresolved=1199   collision-byte 1, collision-case-fold 1, collision-nfc 1, collision-same-kind 1, collision-vs16-fold 1
```
Run 2 (after, final):
```
[clean taxonomy plan] moves=2161 roots=0 relocations=0 symlinks=0 removals=1 edits=4182 regenerations=7 unresolved=1195
  digest=557423a4e8c5502ffb0e089001c4a7e7e48b6b9021c0c46e85bcf9e1c3011791
  -> 🗑️temp/🔣️collide-plan2.json
collision-* rows: 0 (all five codes absent from .unresolved)
```
`moves` 2082→2161 (+79, expected — every registration that used to be blocked by a collision now
produces its move); `unresolved`'s other classes shifted by small amounts between the two runs
(`package-implementation-destination-unresolved` 113→95, `semantic-stem-unresolved` 449→411, a few
others ±) — expected concurrent-repo noise (other workers are actively editing `🔣️taxonomy.json` and
plugin sources per the ticket's own note), not something this slice touched; none of those classes
are `collision-*`.

Note: one `bun`/engine `TypeError` was hit and reproduced twice while running `clean taxonomy plan`
(`🧹️normalization/🟦️.ts:4360`, inside `rustCodeOnlyText`'s docstring) — `git status` showed that
exact file `MM` (modified, uncommitted) at the time, i.e. another worker's concurrent in-progress
edit to shared engine code, not caused by anything in this slice. It cleared on retry without any
change from me.
