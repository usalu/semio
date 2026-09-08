# WP4e — `🧰️framework/🔨️modules/🖱️ui/**` + `🌱️value/🧬️schema/**` rows 145, 149, 151

Partition: `🧰️framework/🔨️modules/🖱️ui/**` and the new `🧰️framework/🔨️modules/🌱️value/🧬️schema/**`,
plus the exact consumer lines in other framework modules that address the collapsed scalars (§5.2).
Continuation of `📓️wp4d-framework-ui.md` (rows 92, 96) and `📓️wp4d-framework-modules.md` §2.2/§6.1.
Rows worked: `📋️cross-partition-requests.md` **145**, **149**, **151**.

## 1. Result per row

| row | request | result |
|---|---|---|
| 145 | one owner for `👥️presence-overlay.json`: `framework.ui.contract/ContractFixture`; `framework.ui` drops `PresenceOverlayFixture`; consumers `$ref` the owner; resolve the other three `📓️wp4d-framework-ui.md` open questions the same way | **done** — W10g had landed the schema/Rust/TS halves before it was cut off; this pass audited the whole surface, found no residue, and re-ran the consumer assertions (§2, §4.5) |
| 149 | `🎟️admission` `definitions.u64` → `$defs.U64`; `🪪️root:7` `$ref` follows | **done, superseded by row 151** — W10g's intermediate `framework.ui.host.input.admission/$defs.DecimalU64` is deleted; admission and root now address `framework.value` (§3) |
| 151 | shared scalar primitives are exports of exactly one scope, `framework.value`; the two `U64` copies promoted by WP4d collapse onto it | **done** — new `framework.value` scope owns `U64` + `NonZeroU64`; three copies deleted, 26 `$ref`s repointed, 0 residual (§3) |

## 2. Row 145 — what was already landed, and what this pass verified

`PresenceOverlayFixture` no longer exists anywhere in the tree (repo-wide grep: only this ticket's own
prose and the os renderer test's `🪦️`-style comment naming what it replaced). `framework.ui`'s `$defs`
is now `UIDialogModalFixture`, `RetainedCommandLimits`, `RetainedCommandRoute`, `RetainedCommandRoutes`,
`RetainedCommandCohort`, `RetainedCommandRoutesDocument` — the presence fixture export is gone.
`framework.ui.contract` carries `ConformanceCatalogFixture`, `ContractFixture`, `PresenceUpdate`.

The TypeScript consumer
(`🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts`)
imports the owner document at **line 57** and compiles the owner's export at **line 3542**:

```ts
const validate = new Ajv({ strict: true }).addKeyword("x-semio-formats").addSchema(uiContractSchema).compile({ $ref: `${uiContractSchema.$id}#/$defs/ContractFixture` });
```

Re-run standalone against the committed fixture (§4.5): the positive and all three negatives hold.

### 2.1 The other three open questions of `📓️wp4d-framework-ui.md` §7

| # | question | resolution |
|---|---|---|
| 2 | `ContractFixtureCase.update` is `crate::PresenceUpdate`, wider than the JSON | **closed the same way as row 145 — single owner, `$ref`.** `PresenceUpdate` is now a third `$defs` export of `framework.ui.contract`, spelled exactly as wide as the Rust struct (`peers` optional, `surface` a 512-byte `SurfaceId`); `ContractFixture.cases[].update` is `allOf: [$ref …#/$defs/PresenceUpdate, {narrowing}]`, so the fixture narrows the owner instead of restating it. The Rust half is the per-name `pub use crate::PresenceUpdate;` row 148 settled (`🧬️schema/🦀️.rs:113`), and a new law `presence_update_export_matches_the_wire_shape` pins the two together. Both halves now accept the same documents; the widening this question raised is gone. |
| 3 | the `♻️retirement` `SIGABRT` blocks any full-suite claim for `semio-framework-ui-contract` | **still open, still not ours.** Filtered runs are green (§4.3). The unfiltered run could not be attempted at the end of this pass for a *different*, newer reason: a peer removed `semio-framework-os-config` from the workspace root manifest's `workspace.dependencies` while this pass ran, so every cargo invocation now fails during manifest resolution (§4.6). Nothing in this pass touches Rust. |
| 4 | two `#[path]` mounts repaired by W10g belong to another ticket's refactor | **held.** `🪞️copy/🦀️.rs:540` → `🧪️tests/🔬️bytes/🦀️.rs` and `♻️retirement/🌳️typed/📃️document.rs:218` → `🧪️tests/🔬️document/🦀️.rs`, both still in the peer's own `#[path = "🧪️tests/<case>/🦀️.rs"]` convention. A third instance of the same fallout surfaced in this partition this pass and was repaired identically (§3.4). |

## 3. Rows 149 + 151 — one owner for the decimal-string u64

### 3.1 The owner: a new `framework.value` scope

`🧰️framework/🔨️modules/🌱️value/🧬️schema/` did not exist (`🌱️value` had `💾️resident`, `🔁️codec`,
`🗂️ordered`, `✨️derive` sub-scopes but no module scope of its own). Created:

* `🔣️.json` — `$id https://semio.tech/schema/framework/value/schema.json`, draft-07, two `$defs`.
* `🟦️.ts` — `export type U64` / `NonZeroU64` plus `parseU64()` / `parseNonZeroU64()` and a
  `ValueSchemaError`, the runtime entry point execution contract §A requires beside an erased type.

```json
"U64":        { "type": "string", "maxLength": 20, "pattern": "^(0|[1-9][0-9]{0,18}|…|18446744073709551615)$" },
"NonZeroU64": { "allOf": [{ "$ref": "#/$defs/U64" }, { "not": { "const": "0" } }] }
```

### 3.2 Why **two** exports and not one — measured, not assumed

The two copies row 151 names are **not** the same shape, and collapsing both onto one would have been a
silent widening:

| copy | admits `"0"`? |
|---|---|
| `framework.ui.host.input.admission/$defs.DecimalU64` (W10g's promotion of `definitions.u64`) | yes — pattern starts `^(?:0\|[1-9]…` |
| `framework.actor.page/$defs.Word` (kept per row 149) | yes |
| `framework.actor.lifetime/$defs.U64` | **no** — pattern starts `^([1-9]…`, zero has no alternative |
| `framework.actor.return/$defs.U64` | **no** — `allOf` of the lifetime one plus a no-op `not: [^0-9]` |

The exclusion is load-bearing, not an accident: `🎭️actor/🚪️lifetime/🧪️tests/🧪️actor-instance-close-fault-…/🟦️.ts:271`
asserts `["0", "-1", "01", "18446744073709551616"]` are all rejected for `activationGeneration` and
`guestLifetime`, and `:182` asserts `aliasCounter.afterReturn: "0"` is rejected. So `framework.value`
owns the base scalar **and** its one narrowing, and the actor family references the narrowing. Both
directions are pinned by an oracle (§4.4): `"0"` is `U64`-valid and `NonZeroU64`-invalid, `"-1"`/`"01"`/
`2⁶⁴` are invalid for both, `"1"` and `2⁶⁴−1` valid for both — the exact behaviour the copies had.

`framework.actor.return`'s `U64` was `allOf(lifetime.U64, not:{pattern:"[^0-9]"})`; the second branch is
a no-op over a pattern that is already anchored digits-only, so `return.U64 ≡ lifetime.U64 ≡ NonZeroU64`
and the collapse is behaviour-preserving, not a widening.

### 3.3 What moved — `wp4e-value-scalar-refs.py` (ticket input, kept)

`bun`-free transformer, explicit `(file, old ref, new ref)` table, no globbing, `--apply` to write.
26 `$ref`s repointed, 3 `$defs` deleted, `residual=0` (a regex sweep of every framework-module
`🔣️.json` for `DecimalU64` or `actor/(lifetime|return)/schema.json#/$defs/U64`):

```
🖱️ui/…/🎟️admission/🧬️schema/🔣️.json        2× #/$defs/DecimalU64                       → framework/value…#/$defs/U64
🖱️ui/…/🎟️admission/🪪️root/🧬️schema/🔣️.json  11× …/admission/schema.json#/$defs/DecimalU64 → framework/value…#/$defs/U64
🎭️actor/🚪️lifetime/🧬️schema/🔣️.json          4× #/$defs/U64                              → framework/value…#/$defs/NonZeroU64
🎭️actor/🚪️lifetime/🩹️patch/🧬️schema/🔣️.json  1× …/lifetime/schema.json#/$defs/U64         → framework/value…#/$defs/NonZeroU64
🎭️actor/🪪️activation/🚪️instance/📥️output/…   1× …/lifetime/schema.json#/$defs/U64         → framework/value…#/$defs/NonZeroU64
🎭️actor/📤️return/🧬️schema/🔣️.json            3× #/$defs/U64                              → framework/value…#/$defs/NonZeroU64
🎭️actor/📤️return/📨️response/🧬️schema/🔣️.json 3× …/return/schema.json#/$defs/U64           → framework/value…#/$defs/NonZeroU64
🎭️actor/📤️return/📨️response/🎟️credit/…        1× …/return/schema.json#/$defs/U64           → framework/value…#/$defs/NonZeroU64
deleted: 🎟️admission/$defs.DecimalU64, 🚪️lifetime/$defs.U64, 📤️return/$defs.U64
applied=26 residual=0
```

`🪪️root` had **no** other reference into `🎟️admission`, so that cross-scope edge disappears entirely;
its test no longer loads the parent document at all.

### 3.4 A dangling import repaired inside this partition

`🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts:9` still imported
`../../📥️input/🎟️admission/📜️script.ts`, a file the concurrent test-layout refactor had already moved
to `📥️input/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts` (the move is committed; the import was not
updated). The ui-host source oracle — the only runnable proof of the admission/root schema change —
could not start until it was repointed. Same class as the two `#[path]` mounts of §2.1 row 4, repaired
the same way: to the peer's own new location, nothing reverted.

### 3.5 Format coverage of the new scope, and a declared deviation

`framework.value` provides `🔣️jsonschema` + `🟦️typescript`; both exports exist in both, so execution
contract §B is satisfied without an `x-semio-formats` annotation (`schema check` confirms: **0** rows
name `framework.value` or `🌱️value/🧬️schema`, §4.2).

**The brief asked for a Rust twin as well; it is not in this change, deliberately.** `🌱️value/🦀️.rs`
is `#[path]`-mounted into **`semio-framework-replication`**
(`📡️replication/📦️packages/🦀️rust/🦀️.rs:33`), not into `semio-framework`. An honest
`🧬️schema/🦀️.rs` there must call `register_scope_schema_exports`, which means adding
`semio-framework-schema-registry` to that crate's `[dependencies]` — a manifest and module-graph change
to a crate whose own manifest carries a long, explicit comment block about staying dependency-light, and
which is two partitions away from this one. Writing an *unmounted* `🦀️.rs` instead would satisfy the
harness's textual `pub type` probe with dead code, i.e. exactly the false format claim WP4d §3.2 refused
to make. The two lines it needs are in §6.1; the annotation-free JSON+TS scope is correct in the
meantime, not a placeholder.

## 4. Verification — real output

Run from the repo root. Cargo confined to
`CARGO_TARGET_DIR=…/scratchpad/target-w10`, `RUSTC_WRAPPER=""`, name-filtered.
Logs in `🗑️generated/wp4e-*.txt`.

### 4.1 `bun wp4-framework-validate.mjs` (ajv draft-07, every framework-module schema)

```
modules=118 exports=232 badDialect=0 badId=0 noExports=0 problems=0
```

`problems=0` is the load-bearing number: the script adds every framework-module document to one strict
ajv instance and compiles **every** `$defs` pointer, so the eight documents that now `$ref` across into
`framework.value` resolve and compile. (116/231 at the end of WP4d → +1 module for `framework.value`,
+1 from a peer; +2 value exports, +1 `PresenceUpdate`, −2 deleted `U64` copies, −1 `DecimalU64` … net
232, with other partitions moving concurrently.)

A confirmation re-run 23 minutes later reads `modules=119 exports=232 … noExports=1 problems=1`, the
single problem being a module a peer created at 23:13 —
`🎭️actor/🪪️activation/🚪️instance/📥️output/🚪️retirement/🧬️schema/🔣️.json`, `no $defs exports` — which did
not exist during this pass's own run and is untouched by it (`git status` shows it as `A`, added by
the peer together with its `🧪️fixture/`). Every export this pass owns still compiles.

### 4.2 `bun 📜️script.ts schema check --json` (repo-wide)

```
[schema check] modules=3143 scopes=3098 findings=6956
[schema check] schema-catalog-stale=1
[schema check] schema-document-id-unaddressable=1
[schema check] schema-export-id-duplicate=39
[schema check] schema-export-id-invalid=286
[schema check] schema-export-incomplete=6337
[schema check] schema-fixture-defines-schema=59
[schema check] schema-module-id-missing=1
[schema check] schema-mutation-leaf-id=131
[schema check] schema-owner-ineligible=11
[schema check] schema-ref-unresolved=90
[schema check] shared-code-table=41 unshared-codes=0
```

Per-partition, which is the number that belongs to this pass:

```
rows under 🧰️framework/🔨️modules/🖱️ui      : 0
rows under 🧰️framework/🔨️modules (all)     : 0   ← includes schema-ref-unresolved = 0
rows naming framework.value / 🌱️value/🧬️schema : 0
```

Row 149's stated acceptance was "the last `schema-ref-unresolved` under `🧰️framework/🔨️modules`". The
whole framework-modules tree now reports **zero findings of any code**. The 90 remaining
`schema-ref-unresolved` are all in `✏️s/🔌️plugins/**` and `🧰️framework/🛍️products/**`.

### 4.3 `bun 📜️script.ts schema test --under … --json`

```
--under 🧰️framework/🔨️modules/🖱️ui        {"diagnostics": [], "fixtures": []}
                                          Test Files 3 passed (3) | Tests 27 passed (27)
                                          [schema test] harness exit=0, draft-07 oracle exit=0
--under 🧰️framework/🔨️modules/🌱️value     {"diagnostics": [], "fixtures": []}
                                          Test Files 3 passed (3) | Tests 27 passed (27)
                                          [schema test] harness exit=0, draft-07 oracle exit=0
```

Per-code counts: the `diagnostics` array is empty in both runs, so every `schema-*` code is 0. Note the
two-measurement caveat `📓️wp4d-framework-modules.md` §6.3 raised (row 150): `--under` reports
`schema-ref-*`/`schema-export-*` as 0 by construction; §4.2's repo-wide per-partition slice is the
measurement that actually covers those codes, and it is also 0.

`cargo test -p semio-framework-ui-contract`, name-filtered:

```
test schema_metadata::scope_schema_export_law::registers_and_resolves_exactly_the_declared_formats ... ok
test schema_metadata::scope_schema_export_law::restricted_formats_match_the_annotation ... ok
test schema_metadata::scope_schema_export_law::presence_update_export_matches_the_wire_shape ... ok
test result: ok. 3 passed; 0 failed; 0 ignored; 0 measured; 163 filtered out; finished in 0.00s

running 7 tests   (--lib presence)
test presence::tests::peer_mark_round_trips_and_omits_false_flags ... ok
test presence::tests::own_presence_default_serializes_to_empty_object ... ok
test presence::tests::presence_overlay_fixture_preserves_separate_own_flags ... ok
test presence::tests::presence_update_omits_empty_peers ... ok
test presence::tests::activity_defaults_to_idle_and_round_trips ... ok
test presence::tests::presence_update_round_trips_with_peers ... ok
test schema_metadata::scope_schema_export_law::presence_update_export_matches_the_wire_shape ... ok
test result: ok. 7 passed; 0 failed; 0 ignored; 0 measured; 159 filtered out; finished in 0.05s

running 6 tests   (--lib conformance::)
test conformance::tests::every_ui_patch_op_variant_appears_in_a_patch_case ... ok
test conformance::tests::patch_fixtures_apply_cleanly_and_match_their_expectations ... ok
test conformance::tests::snapshot_only_fixtures_are_valid_and_match_their_expectations ... ok
test conformance::tests::every_component_variant_appears_in_the_corpus ... ok
test conformance::tests::rejection_fixtures_are_rejected_with_the_named_violation_and_leave_state_unchanged ... ok
test conformance::tests::corpus_has_no_orphan_fixtures ... ok
test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 160 filtered out; finished in 0.23s
```

`cargo check -p semio-framework-replication --lib` (the crate that mounts `🌱️value/🦀️.rs`, i.e. the
"value" crate the brief names) — no warnings, no errors:

```
    Checking semio-framework-replication v0.1.0 (…/📡️replication/📦️packages/🦀️rust)
    Finished `dev` profile [unoptimized] target(s) in 5.13s
```

### 4.4 `bun wp4e-scalar-oracle.mjs` (ticket input, kept) — fixtures + the narrowing

Independent strict-ajv oracle mirroring each owning vitest case's `addSchema` set:

```
FAIL LifetimeFixture ← 🧰️framework/🔨️modules/🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json
    [{"instancePath":"/turnResults/3/hex","keyword":"maxLength","params":{"limit":150},…}]
pass PatchFixture      ← 🎭️actor/🚪️lifetime/🩹️patch/🧫️fixture/🔣️.json
pass ReturnFixture     ← 🎭️actor/📤️return/🧫️fixture/🔣️.json
pass ResponseFixture   ← 🎭️actor/📤️return/📨️response/🧪️fixture/🔣️.json
pass CreditFixture     ← 🎭️actor/📤️return/📨️response/🎟️credit/🧪️fixture/🔣️.json
pass AdmissionFixture  ← 🖱️ui/…/🎟️admission/🧪️tests/🔣️.json
pass RootFixture       ← 🖱️ui/…/🎟️admission/🪪️root/🧪️tests/🔣️.json
pass "0"                    U64=true  NonZeroU64=false
pass "-1"                   U64=false NonZeroU64=false
pass "01"                   U64=false NonZeroU64=false
pass "18446744073709551616" U64=false NonZeroU64=false
pass "1"                    U64=true  NonZeroU64=true
pass "18446744073709551615" U64=true  NonZeroU64=true
checks=13 failures=1
```

**The one failure is pre-existing and unrelated to this pass**, proven rather than asserted: the same
fixture validated against the **`HEAD` blob** of the lifetime schema — the version that still carried its
own `$defs.U64` — produces the byte-identical error.

```
HEAD (pre-change) LifetimeFixture valid = false
[{"instancePath":"/turnResults/3/hex","schemaPath":"#/properties/turnResults/items/properties/hex/maxLength",
  "keyword":"maxLength","params":{"limit":150},"message":"must NOT have more than 150 characters"}]
```

`turnResults[*].hex` lengths are 64/74/76/**152** against a `maxLength: 150`. Neither file is in this
partition and neither was touched here; see §6.3.

### 4.5 The ui-host source oracle and the row-145 consumer

`bun 📜️script.ts test source` in `🖱️ui/🖥️host/📦️packages/🦀️rust` (exit 0) — this is the command that
exercises the admission and root schemas end to end after the collapse:

```
[DEBUG] input admission oracle: 22 neutral cases, 7 schema hostiles, 3 logical-close frontiers over retained 64-byte backing; …
[DEBUG] input root oracle: 6 arithmetic vectors, 6 schema hostiles; …
[DEBUG] input writer oracle: 9 byte-copy frontiers, 3 retained-backing frontiers, 13 incremental UTF-8 vectors, 6 schema hostiles; …
[DEBUG] single-enqueue schema/Buffer oracle: 3 exact 24-byte tuples, 5 hostiles; …
[DEBUG] input commit observer format oracle: 3 exact 56-byte tuples, 3 declared phases, 5 schema hostiles; …
[DEBUG] watchdog tail oracle: 8 same-window vectors, 5 schema hostiles; …
```

Row 145's four consumer assertions, replayed standalone against the committed fixture:

```
ContractFixture(presence-overlay) = true
selectionJson injected rejected   = true
ttlMs:-1 rejected                 = true
own.selected:"true" rejected      = true
```

### 4.6 What could **not** be verified, and why

* `bun nx test @semio-tech/framework-actor` (`test quick`) reports
  `Test Files 5 failed | 5 passed (10) | Tests 16 failed | 190 passed (206)`, every failure the same
  `ReferenceError: Cannot access 'source2' before initialization` raised inside the concurrent
  test-layout refactor's extracted `registerTests(…, source)` wrappers. It fires **before** any schema
  is loaded (e.g. `🧪️ownedactorturnoutput/🟦️.ts:26`, in the first `it()`, above every line this pass
  touched) and it fires identically in `📮️shard-client`, a file this pass did not touch at all. §4.1
  and §4.4 are this change's substitute evidence; §6.2.
* The unfiltered `cargo test -p semio-framework-ui-contract` (open question 3) could not be attempted:
  from ~23:04 every cargo invocation fails during workspace manifest resolution —
  `` `dependency.semio-framework-os-config` was not found in `workspace.dependencies` `` via
  `…/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust/Cargo.toml``. The runs in §4.3 completed
  before that. Peer-owned root-manifest churn; nothing in this pass is Rust.

## 5. Files changed

### 5.1 Owned partition

* **new** `🧰️framework/🔨️modules/🌱️value/🧬️schema/🔣️.json` — `framework.value`, `$defs.U64` + `$defs.NonZeroU64`.
* **new** `🧰️framework/🔨️modules/🌱️value/🧬️schema/🟦️.ts` — `U64`/`NonZeroU64` types, `parseU64`/`parseNonZeroU64`, `ValueSchemaError`.
* `🖱️ui/🖥️host/📥️input/🎟️admission/🧬️schema/🔣️.json` — 2 `$ref`s repointed; `$defs.DecimalU64` deleted.
* `🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/🧬️schema/🔣️.json` — 11 `$ref`s repointed.
* `🖱️ui/🖥️host/📥️input/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts` — loads `🌱️value/🧬️schema/🔣️.json`, `addSchema(value)` before `compile(schema)`.
* `🖱️ui/🖥️host/📥️input/🎟️admission/🪪️root/🧪️tests/🔬️input-root/🟦️.ts` — the parent (`🎟️admission`) document is no longer referenced; loads `🌱️value/🧬️schema/🔣️.json` instead.
* `🖱️ui/🖥️host/📦️packages/🦀️rust/📜️script.ts:9` — dangling import repaired (§3.4).

### 5.2 Cross-module consumer lines (only these lines; nothing else in those files)

`$ref` strings:

| file | lines |
|---|---|
| `🎭️actor/🚪️lifetime/🧬️schema/🔣️.json` | 4 `$ref`s + `$defs.U64` removed |
| `🎭️actor/🚪️lifetime/🩹️patch/🧬️schema/🔣️.json` | 125 |
| `🎭️actor/🪪️activation/🚪️instance/📥️output/🧬️schema/🔣️.json` | 24 |
| `🎭️actor/📤️return/🧬️schema/🔣️.json` | 3 `$ref`s + `$defs.U64` removed |
| `🎭️actor/📤️return/📨️response/🧬️schema/🔣️.json` | 33, 54, 78 |
| `🎭️actor/📤️return/📨️response/🎟️credit/🧬️schema/🔣️.json` | 15 |

ajv registration (one import line + one `addSchema(value…)` per site — without them the strict instances
cannot resolve the new cross-document pointer):

| file | lines |
|---|---|
| `🎭️actor/📤️return/🧪️tests/🧪️actorreturn-codecs-…/🟦️.ts` | 93+96, 209+212, 272+275 |
| `🎭️actor/📤️return/📨️response/🧪️tests/🧪️actorreturnresponseframing-…/🟦️.ts` | 17+19, 229+233, 251+255 |
| `🎭️actor/🚪️lifetime/🧪️tests/🧪️actor-instance-close-fault-…/🟦️.ts` | 268 |
| `🎭️actor/🚪️lifetime/🩹️patch/🧪️tests/🧪️actor-ui-patch-receipt-…/🟦️.ts` | 13+14 |
| `🎭️actor/🪪️activation/🚪️instance/📥️output/🧪️tests/🧪️ownedactorturnoutput/🟦️.ts` | 75+78, 126+129 |
| `🎠️kernel/📤️return/📦️content/🧪️tests/🧪️kernelreturncontentframing-…/🟦️.ts` | 37+39 |
| `💻️os/🔨️modules/🔌️plugin/📤️return/🧪️tests/🧪️pluginreturnwit-…/🟦️.ts` | 55+57 |

### 5.3 Ticket folder

Inputs, kept: `wp4e-value-scalar-refs.py` (the transformer + residual sweep),
`wp4e-scalar-oracle.mjs` (fixture + narrowing oracle).
Generated, delete at ticket close: `🗑️generated/wp4e-schema-check.txt`,
`🗑️generated/wp4e-cargo-ui-contract.txt`, `🗑️generated/wp4e-cargo-replication.txt`.

## 6. Cross-partition requests

### 6.1 Whoever owns `📡️replication` — the Rust twin of `framework.value` (§3.5)

Two lines, then the leaf can be written honestly:

```toml
# 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml — [dependencies]
semio-framework-schema-registry = { path = "../../../🧬️schema/📇️registry/📦️packages/🦀️rust" }
```
```rust
// 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/🦀️.rs, beside `pub mod value;` (line 33)
#[path = "../../../🌱️value/🧬️schema/🦀️.rs"]
pub mod value_schema;
```

The leaf itself is then `pub type U64 = u64; pub type NonZeroU64 = core::num::NonZeroU64;` plus a
`register_scope_schema_exports(ScopeSchemaExports { scope: "framework.value", exports: &EXPORTS })`
with the three-format `FacetLeaves` (`rust`, `typescript`, `json_schema`). The registry crate is
dependency-free (`[dependencies]` is empty), which is exactly the case execution contract §C calls out:
"so every scope crate, including dependency-restricted ones, registers its own exports". Note that the
JSON `U64` is a *string* encoding, so a Rust `u64` twin is a naming artifact only — if the coordinator
would rather `framework.value` stay a JSON+TS scope, say so and this request is withdrawn, not deferred.

### 6.2 W-actor / W-kernel / test-layout peer — the `source2` TDZ regression (§4.6)

16 tests in `@semio-tech/framework-actor` fail with
`ReferenceError: Cannot access 'source2' before initialization` inside the extracted
`registerTests(vitest, dependencies, source)` wrappers, across five files including
`📮️shard-client`, `🪪️activation/🚪️instance/📥️output` and `📤️return`. Not caused by this pass (it fires
above every edited line and in untouched files), but it means the actor suite currently cannot confirm
*anyone's* schema change. Blocks the vitest half of §5.2.

### 6.3 W-actor — `LifetimeFixture.turnResults[3].hex` is 152 chars against `maxLength: 150` (§4.4)

Pre-existing at `HEAD`, proven by validating the HEAD blob. Either the vector or the bound is wrong;
both files (`🎭️actor/🚪️lifetime/🧪️fixture/🔣️.json`, `…/🧬️schema/🔣️.json`) are outside this partition.
Currently masked by §6.2, so it will surface the moment that regression is fixed.

### 6.4 W2w library — regenerate `📚️library/🔣️schema-catalog.json` / `📓️schema-catalog.md`

`schema check` reports `schema-catalog-stale=1`. Deltas this pass adds to WP4d §6.4's list:

| scope | change |
|---|---|
| `framework.value` | **new scope**, path `🧰️framework/🔨️modules/🌱️value/🧬️schema`, formats `{🔣️jsonschema: 🔣️.json, 🟦️typescript: 🟦️.ts}`, exports `U64`, `NonZeroU64` |
| `framework.actor.lifetime` | loses `U64`; gains `dependsOn: framework.value` |
| `framework.actor.return` | loses `U64`; `dependsOn` framework.value, no longer framework.actor.lifetime |
| `framework.ui.host.input.admission` | `DecimalU64` never reaches a released catalog; gains `dependsOn: framework.value` |
| `framework.ui.host.input.admission.root` | `dependsOn` framework.value, no longer …admission |
| `framework.actor.lifetime.patch`, `framework.actor.activation.instance.output`, `framework.actor.return.response`, `…response.credit`, `framework.kernel.return.content` | `dependsOn` gains framework.value |
| `framework.ui.contract` | `PresenceUpdate` (W10g) — source hashes of both `🔣️.json` and `🦀️.rs` changed again |

### 6.5 W2w library + whoever owns the test-layout refactor — the ui-host handoff fixture

`🦑️repo/📚️library/🧪️tests/🔬️workspace-contract/🟦️.ts:601` does
`read(expected.admission.scriptPath)` and asserts the snapshot is a file. The fixture
(`📚️library/🧪️tests/🤝️package-language-kind-handoff/🖥️ui-host-package/🔣️.json:39-41`, mirrored in
`🛂️schema/🔣️.json:78-79`) still pins

```
"scriptPath":  "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/📜️script.ts"   ← deleted by the refactor
"importSpecifier": "../../📥️input/🎟️admission/📜️script.ts"                             ← repointed here, §3.4
"fixturePath": "🧰️framework/🔨️modules/🖱️ui/🖥️host/📥️input/🎟️admission/🧪️test/🔣️s.json" ← never existed; the fixture is 🧪️tests/🔣️.json
```

and line 587 additionally asserts `🧪️fixture/🔣️.json` and `🧪️schema/🔣️.json` are *absent* next to the
script. The correct values are `…/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts`,
`../../📥️input/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts` and
`…/🎟️admission/🧪️tests/🔣️.json`.

**Paired change, deliberately not made unilaterally:** the same test does
`expect(project).toEqual(expected.project)` against
`🖱️ui/🖥️host/📦️packages/🦀️rust/📋️project.json` (in this partition), whose `namedInputs.default` lists
the same two dead paths at lines 10-11. Editing only the project.json half would turn one failing
assertion into a different failing assertion. Both halves must land together:

```
🖱️ui/🖥️host/📦️packages/🦀️rust/📋️project.json  namedInputs.default
-  "{workspaceRoot}/…/🎟️admission/📜️script.ts"
-  "{workspaceRoot}/…/🎟️admission/🧪️test/🔣️s.json"
+  "{workspaceRoot}/…/🎟️admission/🧪️tests/🔬️input-admission/🟦️.ts"
+  "{workspaceRoot}/…/🎟️admission/🧪️tests/🔣️.json"
```
plus the identical `namedInputs` block inside `🖥️ui-host-package/🔣️.json` (`:60-61`) and
`🖥️ui-host-package/🛂️schema/🔣️.json` (`:103-104`).

## 7. Open questions

1. **`🖱️ui/🧬️contract/🧪️fixtures/` is not a taxonomy directory name.** Execution contract §B: data
   collections are `🧫️fixtures` (`testFixturesDirName`), `🧪️fixtures` "is not a taxonomy name; wave-2
   partition owners rename it and rewire readers". This partition owns one
   (`🧬️contract/🧪️fixtures/👥️presence-overlay.json`) and `schema check` does not flag it, so no gate
   forces the rename today. Renaming it rewires the Rust `include_str!` in
   `🧬️contract/🧬️schema/🦀️.rs`, the presence law, and the os renderer test's line 56 import — a
   two-partition edit for zero measured findings. Left as-is; needs a coordinator call on whether the
   taxonomy rename is in scope for this ticket or a follow-up.
2. **`framework.actor.page/$defs.Word` is the same scalar as `framework.value/$defs.U64`.** Byte-identical
   pattern and `maxLength`, three referring scopes. Row 149 explicitly says to keep the two
   `…/actor/page/schema.json#/$defs/Word` refs the framework worker applied, so it was kept — but under
   row 151's own rule ("shared scalar primitives are exports of exactly one owning scope") `Word` is a
   fourth copy of the owner and should become `$ref …/framework/value/schema.json#/$defs/U64`, or keep
   its name as a `framework.actor.page` alias defined *as* an `allOf` of the owner. Not decided here:
   the two rows disagree and `📃️page` is not this partition.
3. **Three more private `definitions.u64` helpers exist in framework modules** —
   `🧵️job/⏱️budget/🧬️schema/🔣️.json` (`^(0|[1-9][0-9]{0,19})$`, a *looser* pattern that admits
   `19999999999999999999` > 2⁶⁴−1) and `🎠️kernel/📤️return/📦️content/📥️input/🧬️schema/🔣️.json` (same
   looser pattern). Both are module-internal, which contract §A declares legal and never a finding, so
   neither is in scope for row 151 as written. They are nonetheless two more spellings of the same
   scalar, one of them wrong at the boundary. (`🎠️kernel/📤️return/📦️content` already does the right
   thing: its `definitions.u64` is a `$ref` to the owner.)
4. **`I32`/`I64`/`U32` have no framework-module instance.** The brief asked for them "if present
   anywhere as helpers"; the only `I32`/`U64` declarations outside `🔨️modules` are integer-typed (not
   decimal-string) and live in `🧰️framework/🛍️products/💻️os/**` and `✏️s/🔌️plugins/**` — a different
   vocabulary in a different partition. Adding unused exports to `framework.value` would create dead
   exports, which contract §B says are deleted, so none were added.
