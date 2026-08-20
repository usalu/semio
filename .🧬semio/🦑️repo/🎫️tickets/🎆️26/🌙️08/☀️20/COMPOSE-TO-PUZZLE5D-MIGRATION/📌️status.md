# Compose → Puzzle5d migration — status

Pinned to `5904ebe289a4d149e659b23e1f728895ad8de4e8` (verified `HEAD ==` pin at session start).

## Wave status

| wave | state | evidence |
|---|---|---|
| 0 · pin & baseline | ✅ done | `📓️w0-baseline.md`, `📓️w0-cargo-check.txt` |
| 1 · read-only census | ✅ done | `📓️census/` (8 reports) + `📓️w1-coordinator-verification.md` |
| 2 · contract freeze | ✅ done | `📓️w2-contract.md` — 11 frozen decisions |
| 3 · foundation | ⚠️ partial | fixture-lint + fixture tree landed; codec generation **blocked** (see below) |
| 4 · atomic mutations | ✅ **28/28 covered** | `fixtures lint` → `28 covered · 0 uncovered` |
| 5 · compose algorithms | ⛔ blocked | needs Wave 3 |
| 6 · asset translation | ✅ manifest done | `🗺️migration-manifest.json` — 68 entries, **0 unaccounted of 128 tracked files** |
| 7 · consumer migration | ⛔ blocked | needs Wave 3 |
| 8 · integration & parity | ⛔ blocked | |
| 9 · cleanup | ⛔ blocked | scope now known to be small — contract D9 |
| 10 · final red team | ⛔ blocked | |

## Landed and verified

- **`fixtures lint`** — `✏️s/🔌️plugins/🧩️puzzle/📦️packages/🦀️rust/📜️script.ts`, region `🔖️FixtureLint`.
  Reads the mutation vocabulary **from the schema** (enum variants + each leaf's `#[dsl(keyword)]`),
  never from a hand-maintained list, and enforces contract D1/D6: schema variants == mutation leaves
  == test subjects; complete codec set per case; rejected-case substitution; exactly-one-of
  inline-or-reference snapshots; reference fields free of path traversal.
  Registered as nx target `fixtures-lint` in `📋️project.json`.
  Run output (real, not projected):
  ```
  🧬️ puzzle5d: 28 schema mutations · 28 leaves · 0 covered · 28 uncovered
  ❌️ 28 error(s)
  ```
  It correctly reports zero variant/leaf mismatches — the 28↔28 mapping is clean — and 28 missing
  test suites. This is the Wave-4 work list, mechanically derived.

- **`🗺️migration-manifest.json`** — every one of the 128 tracked `compose/fixture` files carries a
  terminal status. Accounting was checked programmatically against `git ls-files`: **0 unaccounted**.

## ⛔ Blocker — peer session's de-async sweep

`cargo check -p semio-s-plugin-puzzle` fails at the pin: **927 errors in `semio-framework-os-infinite`**
(`expected Vec3, found future`). Attribution settled by live predicate, not inference:
`git status` on that crate is **clean**, so it is broken in the committed tree, while the unstaged
`🧰️framework/🔨️modules/🔄️machine/✨️derive/🦀️component.rs` and the staged
`26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYNC-REPAIR-SWEEP` files show a peer
actively repairing it. Re-checked once during this session — still 927, unchanged.

This blocks, specifically:
- generating `tower.pack.semio` / `tower.spr.semio` / `tower.json` (needs the real codecs — a
  hand-written TS re-encoder would be a forbidden parallel implementation)
- the binary diff codec (D7.1)
- `FixtureHarness` and every Rust fixture
- and the sweep is **rewriting the very signatures** the fixtures bind to
  (`async fn diff/inverse/label` on all 28 leaves)

No "tests pass" claim is made for any Rust fixture. Nothing was written against signatures that are
mid-rewrite.

## Plan corrections found during census — act on these, not on the raw plan

1. **`flatten.cases.compose.json` has NO expected output** (plan §6.1/§11 assume it is authoritative).
   It is `{name, kit, designPath}` only. Flatten's oracle is indirect and parity must be **numeric**,
   not hash-based — entity-type labels feed compose's merkle hash, so the rename changes every digest
   by construction.
2. **The binary diff codec is a 5-line stub, not merely unwired.** Real `encode`/`decode` must be
   authored (snapshot equivalent is 60 lines).
3. **TypeScript parity is vacuous** — all 28 `🟦️component.ts` files are literally `export {};`.
4. **Compose has zero source consumers outside `compose/`** — census's `erased_compose` hit was a
   name collision. Cleanup is a subtree delete plus 7 manifest edits.
5. **`inverse` must stay `Vec<Mutation>`**, not the plan's singular return (D4).
6. **Nakagin already matches**: puzzle5d example = 180 parts / 179 fasteners = compose's 180/179.

## Needs a decision from the dev

**TypeScript scope.** The plan's fixture contract, parity assertion, and CI shard 7 all assume
maintained TS implementations. There are none. Contract D2 currently records all 28 mutations as
Rust-only and strikes the TS gates as vacuous rather than shipping always-green checks. Writing 28
real TS implementations is new scope the plan neither asks for nor budgets. Confirm D2, or extend
scope.

---

# Wave 4 — every mutation now has a test

`bun ./📜️script.ts fixtures lint`:
```
🧬️ puzzle5d: 28 schema mutations · 28 leaves · 28 covered · 0 uncovered
⚠️ 252 derived-encoding gap(s) pending `fixtures generate` (contract D1 target; run with --full to fail on them)
✅️ fixture contract satisfied
```

## What landed
28 test cases, one per mutation, under
`…/🧬️schema/🧬️mutations/<mutation>/🧪️tests/<case>/`, each carrying the hand-authored core set
(before/after snapshot JSON, mutation JSON, outcome JSON) and a Rust `🦀️component.rs` with **four
assertions**:

1. `applies_to_committed_after` — the mutation carries `before` to exactly the committed `after`.
2. `inverse_restores_before` — forward then inverse restores `before` exactly.
3. `committed_json_is_canonical` — decode→encode is a fixed point for both snapshots and the mutation.
4. `declared_outcome_holds` — the declared status matches real behavior; a rejected case must leave
   the snapshot untouched.

All 28 are wired into `📦️glue.rs` as `#[cfg(test)] #[path = …] mod tests_<case>;` beside each
mutation's existing `mutation`/`diff`/`inverse` modules.

The fixtures share one small base snapshot — two parts (`part-a` fixed with grips `grip-1`,
`grip-spare`; `part-b` derived with `grip-2`), one fastener `fast-1`, one kind-compatibility row —
small enough that every expected `after` is reviewable by eye, rich enough to exercise the two
cascades (`🗑delete-part` and `➖remove-part-grip` both sever `fast-1`).

Every `after` state was derived from a **direct read of that mutation's own diff builder**, not from
the docstring — including the two cascade rules, `🧊replace-part2d-geometry`'s clearing of `radius`
when `newRadius` is null, and `📚replace-kind-catalogs`'s content-addressed no-op.

## Verification actually performed
| check | result |
|---|---|
| `fixtures lint` core tier | ✅ 28 covered, 0 uncovered, 0 errors |
| referential integrity of all 56 snapshots (no dangling `part:grip`, no duplicate ids) | ✅ 0 problems |
| all 28 serde mutation tags distinct and in the declared vocabulary | ✅ |
| declared no-op ⟺ `after == before` | ✅ consistent on all 28 |
| Rust syntax of all 28 test files (`rustfmt` parse) | ✅ 28/28 |
| `📦️glue.rs` parses with the 28 new modules | ✅ |
| all 112 `include_str!` targets resolve | ✅ 0 missing |
| all 28 glue `#[path]` targets resolve | ✅ 0 missing |
| **`cargo test` — tests actually executed** | ❌ **NOT RUN — workspace does not compile** |

## ⛔ Still blocked, and what it means for the above
`cargo check -p semio-s-plugin-puzzle --tests` at this moment:
```
error: could not compile `semio-framework-os-infinite` (lib) due to 873 previous errors
error: could not compile `semio-s-plugin-stdio` (lib) due to 4921 previous errors
```
During this session `infinite` went 927 → compiled → 873 errors as the peer's de-async sweep moved
through it; `stdio` (739 files staged by that peer, also a puzzle dependency) is mid-sweep.

**Therefore: these 28 tests have never been executed.** They are syntax-checked, their every file
reference resolves, and their expected states were derived from the real diff builders — but no
claim is made that they pass. They are written in the de-async target style (plain calls, no
`.await`), matching the existing committed example tests and the direction the sweep is heading.

The 252 derived-encoding gaps are deliberate, not an oversight — see contract D12: a hand-forged
`.pack.semio` would be a parallel implementation of the codec it is meant to test.

## Remaining to close the migration
1. Run `cargo test -p semio-s-plugin-puzzle` once the sweep lands; fix whatever the 28 tests surface.
2. Implement `fixtures generate` (contract D12.1) and fill the 252 derived encodings.
3. Author the binary diff codec (D7.1) — prerequisite for `.patch.spr.semio`.
4. Generate `tower.pack.semio` / `tower.json` for the Nakagin example (D7).
5. Waves 5, 7, 8, 9, 10.
