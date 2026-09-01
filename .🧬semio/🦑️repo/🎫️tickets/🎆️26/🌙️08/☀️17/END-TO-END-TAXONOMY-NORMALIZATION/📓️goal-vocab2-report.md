# 📓️ Vocabulary-Gap Closure Report (round 2)

Full detail: `📓️goal-vocab2-census.md` (same folder). Verified with `bun ./📜️script.ts clean
taxonomy inventory --scope ...` per module (fast, local-only phase — no `plan`/`apply` run), never
in the same class as the 10-15 min repo-wide scan.

**Registered in `🔣️taxonomy.json` (`semanticDirectoryKinds`, all reusing confirmed on-disk emoji):**
`limits` (emoji fixed 🚧️→📐️, was unused anywhere), `ledger` (📒️), `contention` (🧬️, scoped
`parentKindIds:["clock"]`), `resident-admission` (📨️, scoped `["resident",
"members-of-members-of-modules"]` — third conflicting-emoji "admission" variant besides the existing
`🏘️`/`🎟️` ones), `resident-fixture-member` (🧪️, narrow pattern `^(fixture|schema)$`, scoped to
`["members-of-members-of-modules","resident-admission"]`), `test-tube-fixtures` (🧪️, the minority-
but-real `🧪️fixtures` convention, 312 refs repo-wide — parented with an exact copy of `test-case`'s
own ~80-entry parent list so it wins exactly where `test-case` used to wrongly swallow it, and loses
nowhere `test-case` didn't already claim it), `shared-owner` (📤️, scoped
`["test-tube-fixtures"]`). Extended `fixture-case.parentKindIds` and `test-case`/
`test-fixture-member.slugPattern` (added `|fixtures$|examples$` exclusion — a directory literally
named "fixtures"/"examples" must never resolve as an individual test case or member).
**`fixedFilenameContracts`:** `font-family-libertinus-ofl-license`, `font-family-noto-ofl-license`
(SIL OFL license text, family-name-mandated, scoped to the `fonts` directory kind).

**Collision traps hit and fixed before landing:** first draft added `test-tube-fixtures`/
`resident-admission` context to `test-fixture-member` directly — re-created the exact
`json-fixture-case`-vs-`fixture-case` overlap the coordinator had just fixed, because the no-emoji
branch ignores a kind's own `emoji` field and matches on parent+pattern alone. Caught by re-deriving
`matchDirectoryKind` by hand before landing, not by a plan run; fixed by giving `test-tube-fixtures`
member-resolution to `fixture-case` only, and `resident-fixture-member` its own narrow
`^(fixture|schema)$` pattern instead of widening an existing catch-all.

**Not registered, on purpose:** `🕸️graph`/`🎭️actor`/`🖼️assets` `🤖️generated/*` (28 rows) — generator
output filenames, not domain vocabulary; would violate the taxonomy's own no-synonym rule. Flagged
as a spawned task (needs a `generatorContracts` fix, not word registration). `🔀️dispatch`'s 2 bare
Rust test filenames (`mixed_receivers.rs`, `mut_receiver.rs`) — same "genuine per-crate name, not
cross-cutting vocabulary" class the round-1 slice already established. `🖱️ui`'s 79 rows — pre-owned
by `📓️goal-ui-report.md`, untouched.

**Tests:** extended `🧪️package-language-kind-handoff/🔣️.json` with 7 new cases (one per new/changed
kind, plus a `test-tube-fixtures`→`fixture-case` regression guard). Real run:
`bun test .../🧪️index.test.ts -t "package language semantic handoff"` → **6 pass, 0 fail, 1213
expect() calls**.

**Before → after** (`clean taxonomy inventory`, per-module, real pasted output):

| module | before | after |
|---|---:|---:|
| `🌉️abi` | 2 | **0** |
| `⏱️trace` | 1 | **0** |
| `🌱️value` | 15 | **0** |
| `📚️compiler` | 2 | **0** |
| `🛂️manifest` | 6 (this class) | **0** |

19/22 modules now census clean on `semantic-stem-unresolved`/`-ambiguous`/`directory-kind-unresolved`.
The 2 that don't (`🖱️ui`, `🤖️generated` cluster) are explicitly out-of-class, not gaps I left
unaddressed. No `moves`/`unresolved`-total number claimed — that needs your `plan`/`apply` drive.
