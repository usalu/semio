# Wave 2 — Frozen contract and decision log

Frozen against `5904ebe289a4d149e659b23e1f728895ad8de4e8`. Every downstream agent obeys this file.
Deviations require a new entry here, signed by the coordinator — not a local judgement call.

---

## D1 · Test-directory contract (FROZEN)

```
<mutation>/
  🧪️tests/
    <test>/
      🦠️mutation/   🔧️component.op.semio   📡️component.spr.semio   🔣️component.json
      🔺️diff/       🩹️component.patch.semio 📡️component.patch.spr.semio 🔣️component.json
      📸️snapshot/
        ⬅️before/    🗣️component.dsl.semio  🎒️component.pack.semio  🔣️component.json
        ➡️after/     (same three, or 🔗️component.ref.json)
      🎯️outcome/    🔣️component.json
      🦀️component.rs
```

**Emoji prefixes are mandatory** and follow the codebase's existing per-format convention, observed
from the real assets: `🗣️` dsl · `🎒️` pack · `🔧️` op · `📡️` spr · `🔣️` json · `🩹️` patch ·
`🦠️` mutation · `🔺️` diff · `📸️` snapshot · `🧪️` tests. The plan's bare `component.patch.semio`
names are rendered in this scheme. `🩹️` is newly introduced for patch-text; verified unused
elsewhere in the repo.

**`🟦️component.ts` is OMITTED from every test directory.** See D2.

### D1.1 · Extension ruling
`.dsl.semio`, `.op.semio`, `.pack.semio`, `.spr.semio` already exist repo-wide with established
producers/consumers — reused as-is. `.patch.semio` and `.patch.spr.semio` do not exist anywhere and
are introduced by this migration.

---

## D2 · TypeScript is out of scope — all 28 mutations are Rust-only targets (FROZEN)

Every `🦠️mutation/🟦️component.ts` in the tree is literally `export {};` — one line, empty module.
There is no TS apply/diff/inverse anywhere.

Ruling:
- Each test descriptor records `targets: &[Target::Rust]`.
- The plan's "Rust/TypeScript parity" assertion row and CI shard 7 are struck as **vacuous**, not
  implemented as always-green.
- Authoring 28 real TS implementations is new scope, is not budgeted by the plan, and is **not
  undertaken**. Recorded as a known gap in the final report.

---

## D3 · Oracle precedence, applied per family (FROZEN)

The plan's §6.1 precedence is kept, but "committed expected fixture asset" only outranks
implementations **where such an asset exists**. Survey result:

| family | oracle | rank used |
|---|---|---|
| flatten | ⚠️ **none committed** | falls to Rust solver + Go operation at the pin, cross-checked by flatten-merkle |
| export-representation | ⚠️ **none committed** | `.gltf`/`.ifc` adopted as de-facto expected assets |
| quality-sum | fixture `2349.53` — **conflicts with Rust stub `0.0`** | fixture wins; **Go is the spec** |
| flatten-merkle, design-with-diff, copy-paste, delete, filter-kit, find-replaceable-types, architect, hash | committed expected assets | fixture wins |

### D3.1 · flatten parity is NUMERIC, not hash-based
Entity-type labels feed compose's merkle hash (`merkle_node_str(&["Position", …])`), so renaming
Piece→Part changes every digest by construction. Hash equality across the rename is impossible.
Flatten parity therefore compares **poses numerically** under the legacy tolerance. The
flatten-merkle 11-row sensitivity matrix is retained as a **behavioral** oracle (which input channel
moves which output channel) because that property survives renaming.

### D3.2 · flatten splits across two layers
- `GeometrySolver` ← Rust `flatten_design_positions` (`compose/client/lib/rs/lib.rs:1393`), pure poses.
- `flatten` semantic mutation ← Go `FlattenDesignDiff` (`compose/client/lib/go/main.go:14209`):
  solve poses, **remove every fastener**, promote all parts to anchored.

---

## D4 · `inverse` returns `Vec`, not a single mutation (FROZEN)

The plan's §5.2 sketch has `fn inverse(…) -> Result<Puzzle5dMutation>` (singular). Both the existing
puzzle5d code and compose return a **collection**:
- puzzle5d: `async fn inverse(&self, base) -> Vec<Puzzle5dMutation>`
- compose: `Operation::to_backwards(&kit) -> Result<Vec<Operation>>` (`lib.rs:10165`)

Singular would be a regression: `DeletePart`'s inverse must recreate the part **and** reconnect
every fastener it cascaded away. The trait keeps `Vec`.

Note also that compose inverts at the **operation** level, not the diff level — architecturally
identical to puzzle5d. The plan's §6 "no diff-level inversion oracle" concern dissolves: the
mutation-level inverse is the oracle, and it is well-precedented.

---

## D5 · Terminology map (FROZEN, additions to the plan's table)

The plan's 18 rows stand. Confirmed from the real 5d DSL header, plus rows the plan omitted:

| compose | puzzle5d | evidence |
|---|---|---|
| Kit | kind catalogs (handle `child_id`+`target`) | `🗣️tower.dsl.semio:6` |
| Design | `Puzzle5dSnapshot` | schema |
| Type | part kind | `parts [… part-kind:REF …]` |
| Piece | part | `parts` instance block |
| Connector | grip template | `grips [id code label order compatible-with …]` |
| Port | grip kind | as above |
| Connection | fastener | `fasteners [id source target fastener-kind gap shift rise rotation turn tilt x y]` |
| **(none)** | **rope / rope kind** | `ropes [id label default-fastener-kind:REF]` — **puzzle5d-only, no compose analogue** |
| Plane + Center | `part-3d` (`origin`/`orientation`/`scale`) + `part-2d` (`x`/`y`) | part row |
| fixed piece | `anchor` = `fixed` | part row literal |

**`ropes` has no compose counterpart.** Migration must not invent compose fixtures for it; it is
covered by native puzzle5d fixtures only. Recorded so no agent tries to back-fill it from compose.

---

## D6 · Rejected-mutation encoding (FROZEN)

Per plan §2.3. Error-code vocabulary is exactly **two** codes across all 28 mutations
(census H00, spot-verified):
- `mutation.target-missing` — 22 mutations
- `mutation.duplicate-id` — 2 mutations (`CreatePart`, `AddPartGrip`)

`🎯️outcome/🔣️component.json` is `{"status":"applied"}` or
`{"status":"rejected","code":"mutation.target-missing","path":[…]}`.
Rejected tests carry `🔺️diff/🚫️component.absent` and an after-snapshot equal to (or referencing)
before.

---

## D7 · Wave-3 prerequisite chain (FROZEN — this reorders the plan)

The plan lets S00-S07 run in parallel. They **cannot**. The proof fixture has a hard chain:

```
S06 generate tower.pack.semio + tower.json from tower.dsl.semio   ← MUST BE FIRST
   └─> S03 binary diff codec (encode/decode + wiring)
          └─> S00 fixture harness + lint
                 └─> S08 flatten proof fixture
```

Reason: `…/🏗️nakagin-capsule-tower/🖼️assets/` today holds a real 168 KB DSL but
`🎒️tower.pack.semio` (270 B) and `📡️tower.spr.semio` (267 B) are **header + zero padding** — empty
containers, not encodings of the 180-part design. And `🔧️tower.op.semio` contains
`add-vertex` / `set-face` / `transform-mesh` / `merge-solid` — **vocabulary from a mesh artifact,
none of which is a Puzzle5d mutation.** There is no `tower.json` at all.

The existing test only asserts `len() > 64`, which the zero-padded stubs satisfy — a vacuous gate
that currently reads as green. Any shared-snapshot `🔗️component.ref.json` pointing at this example
would resolve to one real encoding, not three, until S06 lands.

### D7.1 · S03 scope correction
The binary diff codec is **not** merely unwired. `🔺️diff/💾️binary/🦀️component.rs` is a 5-line
stub with no `encode`/`decode` (vs 60 real lines in the snapshot equivalent). S03 authors the codec,
confirms `📡️component.protocol.semio` describes it, then flips `binary: None` → `Some(&langs[N])`.

---

## D8 · Existing test infrastructure is EXTENDED, not replaced (FROZEN)

`os_store::test_support` already provides `assert_dsl_round_trip(&projection)` and
`assert_dsl_pack_equivalence(&projection)`, used by the example tests. `FixtureHarness` builds on
these rather than reimplementing round-trip logic.

---

## D9 · Compose deletion is a clean subtree removal (FROZEN)

**Zero** Rust/TS/Go source references to compose exist outside `compose/`. (Census H52's
`erased_compose` hit is a name collision with a generic framework helper — unrelated.) Coupling is
build-manifest only:

`Cargo.toml:115-117` (3 members) · `go.work:8` · `pyproject.toml:12,37` · `Monorepo.sln` (9 entries)
· `package.json:164,165,244` · `📜️script.ts` (13 orchestration targets) · `Cargo.lock` · `bun.lock`

Wave 9 = delete `compose/`, edit those seven manifests, regenerate two lockfiles.

---

## D10 · The legacy embedded-catalog path may already be dead (OPEN → assigned)

`normalize_kind_catalogs_for_snapshot_value` exists to convert legacy embedded catalogs to the
handle shape. The Nakagin example is **already** on the handle shape
(`kind-catalogs=child_id=… target="…"`). If no committed asset uses the embedded shape, the function
is dead code deletable in Wave 3 rather than Wave 7. Assigned: audit all three examples' catalogs
before writing any fixture that depends on the legacy path.

---

## D11 · Blocked on peer session — de-async sweep (STANDING)

The workspace does not compile at the pin: `semio-framework-os-infinite` has 927 errors from a peer
session's in-flight de-async refactor (ticket `26/08/20/INTERACTIVE-JOB-RUNTIME-REFACTOR/PHASE-1-5-DE-ASYNC-REPAIR-SWEEP`).
`semio-s-plugin-puzzle` depends on it, so **no Rust change in this migration can be compile-verified
yet**, and every mutation leaf's `async fn diff/inverse/label` signature is being rewritten under us.

Ruling:
- Asset-only and script-only work proceeds now (unblocked, no signature coupling).
- Rust code work is sequenced after the sweep lands, or written signature-agnostic.
- **No "tests pass" claim may be made for any Rust fixture until the workspace compiles.**

---

## D12 · Fixture source-of-truth vs derived encodings (FROZEN — amends D1)

D1's nine-file case remains the target. It is now split by **who authors it**:

| tier | files | authored by |
|---|---|---|
| **core** (hand-authored) | `📸️snapshot/{⬅️before,➡️after}/🔣️component.json`, `🦠️mutation/🔣️component.json`, `🎯️outcome/🔣️component.json`, `🦀️component.rs` | a human/agent, reviewed |
| **derived** (generated) | `🗣️.dsl.semio`, `🎒️.pack.semio`, `🔧️.op.semio`, `📡️.spr.semio`, `🩹️.patch.semio`, `📡️.patch.spr.semio`, `🔺️diff/🔣️component.json` | `fixtures generate`, from the core files, via the real codecs |

Rationale: a hand-forged `.pack.semio` or `.spr.semio` would be a **parallel implementation of the
very codec the fixture exists to test** — it would pass by construction and prove nothing. Derived
encodings must come from the production codec or not exist.

`fixtures lint` therefore reports two tiers: core gaps are **errors**, derived gaps are **warnings**
naming `fixtures generate`. `fixtures lint --full` promotes derived gaps to errors and is the gate
that must pass before the migration is declared complete.

### D12.1 · `fixtures generate` — specification for the next session
Not yet implemented: it is Rust, and the workspace does not compile (D11). Writing it blind against
codec APIs this session did not audit would very likely be wrong. Its contract:

- Read `📸️snapshot/⬅️before/🔣️component.json` → `Puzzle5dSnapshot` and `🦠️mutation/🔣️component.json`
  → `Puzzle5dMutation`.
- Recompute the diff, apply it, and **assert the result equals the committed `➡️after` JSON** —
  generation must never silently overwrite a reviewed expectation.
- Emit `.dsl.semio` + `.pack.semio` for both snapshot sides, `.op.semio` + `.spr.semio` for the
  mutation, `.patch.semio` + `.patch.spr.semio` + `🔺️diff/🔣️component.json` for the diff.
- Re-decode every emitted file and assert typed equality with its source (plan §8.2 steps 6-7).
- Run twice into separate directories and assert byte-identical output (plan §13 determinism gate).
- Blocked on: `🔺️diff/💾️binary/🦀️component.rs` gaining real `encode`/`decode` (D7.1) — the
  `.patch.spr.semio` half cannot be produced until then.

---

## D13 · Census correction — there is a THIRD message code

Census H00 reported two codes. Direct verification of all 28 diff builders found **three**, and also
corrected the severity split:

| code | severity | mutations |
|---|---|---|
| `mutation.target-missing` | `error` (not `fatal`) | 22 (`➖remove-part-grip` and `🔌replace-part-grip` raise it twice — part missing, then grip missing) |
| `mutation.duplicate-id` | `fatal` | 1 — `🌱create-part` only |
| `mutation.no-op` | `warn` | at least `🧊replace-part2d-geometry`, `🔗connect-grips`, `📚replace-kind-catalogs` |

Six mutations are **total functions** that cannot reject at all: `🌐change-domain`,
`🏷rename-puzzle5d`, `📚replace-kind-catalogs`, `📝change-description`, `🔗connect-grips`,
`🤝connect-kind-compatibility`.

`MutationOutcome::error` and `::fatal` both force `diff = D::default()` and differ only in message
severity (`🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️component.rs:228-235`). A `warn`
no-op is **applied with an empty diff**, NOT rejected — so `🎯️outcome` gains an optional
`messages: [{level, code}]` array to record it, and the applied/rejected binary is unchanged.
