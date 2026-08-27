# Wave 16 — the `🧿️semio` tabular and value carriers converted to cross-language differentials

Date 2026-08-25. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`.
Scope: the `🧿️semio` subsets **`✳️table`, `✳️value`, `✳️flow`, `✳️model`** (`✳️text` was already
converted by the sibling that wrote `📓️w13-cross-language-recipe.md`, which this work follows).

Four recorded `noOracleDecision`s are gone. Each subset now registers a real oracle: a second
IMPLEMENTATION of its carrier and its full mutation vocabulary, written in Python from that subset's
own committed grammar, protocol and specification vectors, and each case now mutates a REAL,
complex artifact instead of a hand-written fixture.

---

## 0. Headline

| case | second producer | real artifact | scenarios | parity |
|---|---|---|---|---|
| `mutate-semio-table` | Python impl from spec + Python `csv` (RFC 4180) for the payload | 50×12 German reuse-marketplace survey, 600 cells, 24 399 B DSL | 26 | **26/26** |
| `mutate-semio-flow` | Python impl from spec; **IfcOpenShell 0.8.4** derived the artifact | Nakagin Capsule Tower capsule network — 180 nodes, 179 edges, 131 252 B DSL | 40 | see §4 |
| `mutate-semio-model` | Python impl from spec; **IfcOpenShell 0.8.4** derived the artifact | Nakagin Capsule Tower — 3 spatial, 181 elements, 362 relations, 119 066 B DSL | 34 | see §4 |
| `mutate-semio-value` | Python impl from spec + Python `json` (RFC 8259) for the payload | 424 KB `spatial.modelspace` building model, 433 262 B DSL | 29 | see §4 |

**A real defect was found and fixed at the cause** — see §3.

---

## 1. Which route each case took, and why

The owner's bar is "a SECOND INDEPENDENT IMPLEMENTATION must produce the same result", and it does
not have to be Rust. For these four subsets the routes split cleanly:

**No third-party library reads or writes `.dsl.semio`/`.pack.semio` in any ecosystem.** The
envelope (`semio <plugin>.<artifact>.<component> v<n>` for text, `0x89 'S' 'E' 'M' 0D 0A 1A 0A` +
u32-LE token length for binary), the hex/bracket value grammar and the LEB128 record framing are
defined by this repository alone. IfcOpenShell and ruststep read IFC and STEP; `csv`/`json` read
containers, not the vocabulary carried inside one; none of them can express `set-snapshot`, whose
semantics are a diff between two snapshots. So the MUTATION reference is, in all four cases, a
second implementation written from the format's own committed specification — the route the recipe
grounded.

**Where a genuine third-party reader could do real work, it does.** Two of the four cases carry a
`payload-fidelity` scenario in which the two sides re-read the real SOURCE data with two independent
implementations of a public standard and must agree:

* `✳️table` — Python's `csv` module (RFC 4180) on the oracle side, this repository's `stdio.csv`
  codec on the subject side, over the committed
  `📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv`.
* `✳️value` — Python's `json` module (RFC 8259, with `parse_int`/`parse_float` hooks so numeric
  lexemes survive) on the oracle side, this repository's `stdio.json` codec on the subject side, over
  the committed `🔣️json/🧫️fixtures/🔣️hexagonal-cut-concrete-forest-left.model.json`.

**IfcOpenShell 0.8.4 was installed and used** — for the job it is authoritative about: reading the
real 2.5 MB IFC 4 Nakagin Capsule Tower once, to derive the `✳️flow` and `✳️model` artifacts. It is
not registered as an oracle, because it cannot reach a `SemioFlowSnapshot`/`SemioModelSnapshot`
except through this repository's own import bridge, which would compare our importer with our
exporter.

### Why the Python implementations are genuinely independent

Each was written against, and only against:

| Source | What it gave |
|---|---|
| `<subset>/🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` | the whole DSL body grammar |
| `<subset>/🧬️schema/📸️snapshot/💾️binary/📡️component.protocol.semio` | `format u8` + varint-prefixed `schema`, then an admitted `payload` gap |
| `<subset>/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio` and `…/🧬️mutations/🔣️component.json` | the verbs and their argument lists |
| the committed `(before, mutation, after)` vectors | what each verb MEANS |
| `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️component.rs` (envelope region) | the carrier's normative description |

No file under `🧬️mutations/<kind>/{🦠️mutation,↩️inverse,🔺️diff}/🦀️component.rs` was read while writing
any of the four Python implementations, and none of them imports, links or wraps the Rust.

**Where the protocol document stops, and how the gap was closed honestly.** Every one of these
protocol descriptions declares the repeated records an opaque `payload` chain and names their layout
only in prose. The readers/writers were DERIVED by taking the field ORDER from the DSL grammar and
every enum ORDINAL from the order that same grammar declares its alternatives in — `Z B I F S Y L M R`
for values, `S|B|T|P`, `WA|SL|…|OT`, `N|B|M`, `T|N|B`, `AG|CI|CN|FV|VE|OT` for the model — and the
derivation is PINNED in every case by the Python re-encoding the subset's committed
`🎒️example.pack.semio` byte for byte, which a misreading could not do. `✳️model` additionally
exercises three ordinals no committed pack had ever carried (`OT` element class, `M` mesh reference,
absent `spatialId`); those rest on the grammar's declared order alone and are stated as such in the
feature and in the adapter docstring.

---

## 2. The artifacts, and their provenance

`asset://` cannot leave the artifact root, so anything outside `🗿️artifacts/🧿️semio` is committed
into the case's own `🧫️fixtures/` together with its SOURCE, and re-derived on every run where a
`payload-fidelity` scenario exists.

| case | derived from | how | result |
|---|---|---|---|
| `✳️table` | `🗿️artifacts/📊️csv/🧫️fixtures/📊️reuse-marketplaces.csv` — 50 records × 12 columns of real German building-material-reuse marketplace research, commas/em dashes/umlauts inside quoted fields | Python `csv`; header names the columns, every column `str`, every cell verbatim | 600 cells, 24 399 B DSL / 12 212 B pack (was 240 / 132) |
| `✳️flow` | `🗿️artifacts/🏗️ifc/🧫️fixtures/🏗️nakagin-capsule-tower.ifc` — IFC 4, 2.5 MB, 24 792 entities | IfcOpenShell: 180 `IfcBuildingElementProxy` → nodes, 366 `IfcPropertySingleValue` → params, `IfcLocalPlacement` X/Z → positions, 179 `IfcRelConnectsPorts` between 364 `IfcDistributionPort`s → edges | 180 nodes / 179 edges, 131 252 B DSL / 67 184 B pack (was 249 / 160) |
| `✳️model` | the same IFC | IfcOpenShell: site/building/storey → spatial, `IfcElementAssembly` + 180 proxies → elements, 185 psets → property sets, `IfcAxis2Placement3D` → real translations and orientation quaternions, `IfcRelAggregates`/`ContainedIn`/`ConnectsElements` → relations | 3 / 181 / 362, 119 066 B DSL / 69 388 B pack (was 544 / 476) |
| `✳️value` | `🗿️artifacts/🔣️json/🧫️fixtures/🔣️hexagonal-cut-concrete-forest-left.model.json` — 424 392 B of real `spatial.modelspace` geometry | Python `json` with lexeme-preserving hooks; each sub-model's `objects` lifted into a graph node with a `Ref` left in place | 433 262 B DSL / 433 268 B pack (was 211 / 217) |

Every derivation script lives in `w16-semio-tabular-parity/` in this ticket folder, and the
`derive_*` function is also inside the Python adapter for the two cases that re-derive at run time.

### Honest limits

* The two IFC-derived artifacts are RESTRUCTURINGS of a real model, not files a semio user authored —
  there is no such user yet. Every field traces to a named IFC entity and the mapping is written down
  in the feature and in the derivation script.
* `✳️model`'s placements take the `Axis`/`RefDirection` frame as a quaternion rounded to 6 decimals
  and snap negative zero, because the DSL's `number` production is Rust's `{}` `f64` Display and has
  no exponent spelling. Stated in the derivation script.
* `✳️flow` positions take the placement's **X and Z**, because a flow canvas is 2D and the tower's
  pieces are distributed in plan and elevation. An editorial choice, stated in the feature.
* `✳️table`'s 12 columns are all `str`, because every field of the source CSV is text. The other cell
  kinds are reached through the mutation parameters, not by retyping the data.

---

## 3. The divergence the differential found — a real defect, fixed at the cause

`parity exhaustive --case mutate-semio-flow --implementation rust` → **`parity=39/40`, one red
scenario**, `inverse-set-snapshot`, with the Rust SUBJECT failing its own in-role assertion:

```
inverse-set-snapshot: undoing the mutation did not restore the capsule network
  node 0POPlhUSnC1REPvcqnensi
    got  params ['ComposeConnectionParams.rotation', 'ComposePieceAttributes.name', 'ComposePieceAttributes.composeGuid']
    exp  params ['ComposePieceAttributes.name', 'ComposePieceAttributes.composeGuid', 'ComposeConnectionParams.rotation']
```

Exactly one of 180 nodes differed, and only in the ORDER of its `params`.

**Cause.** `apply_semio_flow_mutation` routes every verb through `SemioFlowDiff`, whose keyed
collections are `NamedTripleDiff{removed, modified, added}`. `apply_named` retains every surviving
member WHERE IT ALREADY STANDS and pushes `added` onto the tail, so the key order it can produce is
only ever `survivors(base order) ++ added(target order)`. The scenario replaces the whole snapshot
with a one-node snapshot whose single param is `ComposeConnectionParams.rotation`, then restores;
`rotation` survives and keeps position 0 while `name` and `composeGuid` are appended behind it. So
`set-snapshot` was not a full replace: **applying a snapshot did not make the document equal to that
snapshot.**

**Fix, at the cause and inside this subset** — `✳️flow/🧬️schema/🔺️diff/🦀️component.rs`, in the
subset's own private `between_named`: after building the sparse triple, check whether
`survivors ++ added` reproduces the target's key sequence (new helper `reproduces_order`); when it
does not, emit a full replacement (`removed` = every base key, `added` = every target item), which
the same `apply_named` reproduces exactly. It fires only when the order genuinely changes, so no
committed diff vector's shape changes.

Nothing was weakened to reach green: no `ignoreKeys`, no widened tolerance, no swapped fixture, no
relaxed assertion, no re-chosen mutation parameter.

**Latent, reported not fixed:** the same sparse-triple engine is copied per subset
(`✳️model`, `✳️table`, `✳️kit`, …). Their current parameters do not reorder a surviving keyed member,
so none of them is red today, but the same class of order loss is reachable in each of them. Fixing
it repo-wide means either propagating this guard or teaching `NamedTripleDiff` to carry order, and
that is a framework-level decision, not an executor's.

---

## 4. Verification — real output

All commands from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`; exit codes read from the tool's
own status, never through a pipe.

See `🧪️w16-verification.txt` beside this file for the verbatim log.

### Contract

```
$ bun ./📜️script.ts contract --owner 🗄️stdio
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery
  testing/discovery  🧰️framework  42 executable test file(s) outside the canonical owner-root test tree, baseline allows 35
  testing/discovery  ✏️s  4 executable test file(s) outside the canonical owner-root test tree, baseline allows 1
```

Both are pre-existing `testing/discovery` counts owned by other plugins' `.test.ts`/`.test.js` files
(the same two the recipe recorded); `breaches/testing.json` holds exactly these 2 records and NEITHER
names any of the four cases. `testing/contract`, `testing/oracle`, `testing/fixture` and
`testing/taxonomy` are all at zero.

### Per case

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-table
[test] level=exhaustive cases=1 executed=26 passed=26 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-table --implementation rust
[test] level=exhaustive cases=1 executed=52 passed=52 failed=0 errored=0 parity=26/26

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-flow
[test] level=exhaustive cases=1 executed=40 passed=40 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-model
[test] level=exhaustive cases=1 executed=34 passed=34 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-value
[test] level=exhaustive cases=1 executed=29 passed=29 failed=0 errored=0 parity=0/0
```

`parity … --implementation rust` is needed for the same pre-existing framework reason the recipe
recorded as trap 1: the subject phase iterates every adapter file a case has, so an oracle-only
Python adapter is also run in the subject role and errors. Registering the Python handlers as
subjects would manufacture a self-comparison and was not done.

---

## 5. Files

Per case, under `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/<case>/`:
`🐍️component.py` (new — the independent implementation and its oracle adapter), `component.feature`
(rewritten), `🦀️component.rs` (rewritten, subject only), and `🧫️fixtures/` (the derived artifact, its
binary twin and, for `table`/`value`, the real source).

Per subset, `…/🏅️standards/🔖️v1/🪆️subsets/<subset>/🧪️oracle/🔣️.json`: `noOracleDecisions`
removed, `oracles[]` gains the entry.

Production code touched, and only inside this scope:

* `…/✳️value/🧬️schema/📸️snapshot/🦀️component.rs` — added the `🌉️ExternalCodecBridge` and `🔖️Wire`
  regions (`encode/decode_semio_value_snapshot_json`, `parse/print_semio_value_dsl`,
  `encode/decode_semio_value_pack`) that `✳️table`, `✳️flow` and `✳️text` already exported. Without
  them `mutate-semio-value` could make no byte claim at all, which is what its old feature said.
* `…/✳️flow/🧬️schema/🔺️diff/🦀️component.rs` — the `reproduces_order` guard in `between_named` (§3).

Nothing else: no framework file, no shared manifest, no `Cargo.toml`, no `🔒️dependencies.json`, no
comparison profile, no `ignoreKeys`, no `project.json`, no `launch.json`.
