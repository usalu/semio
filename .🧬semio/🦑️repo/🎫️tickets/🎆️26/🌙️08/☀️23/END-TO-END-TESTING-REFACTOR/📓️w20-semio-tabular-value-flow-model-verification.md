# Wave 20 — independent verification of the `🧿️semio` `✳️table` / `✳️text` / `✳️value` / `✳️flow` / `✳️model` differentials

Date 2026-08-25/26. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`.
Scope as assigned: the `🧿️semio` subsets **`✳️table`, `✳️text`, `✳️value`, `✳️flow`, `✳️model`**.
Verbatim logs in `w20-semio-tabular-verify/`.

---

## 0. Headline

All five cases were **already converted** off their `noOracleDecision` — `✳️text` by the sibling that
wrote `📓️w13-cross-language-recipe.md`, the other four by `📓️w16-semio-tabular-parity.md`. This wave
is therefore an independent AUDIT plus the verification runs w16 never produced, and it found three
real defects, all now fixed at the cause.

| case | second producer | real artifact | scenarios | parity |
|---|---|---|---|---|
| `mutate-semio-table` | Python impl from spec + Python `csv` (RFC 4180) for the payload | 50×12 German reuse-marketplace survey, 24 399 B DSL | 26 | **26/26** |
| `mutate-semio-text` | Python impl from spec | 3-run committed note, 203 B DSL | 22 | **22/22** |
| `mutate-semio-value` | Python impl from spec + Python `json` (RFC 8259) for the payload | 424 KB `spatial.modelspace` model, 433 262 B DSL | 29 | **29/29** |
| `mutate-semio-flow` | Python impl from spec; artifact derived with IfcOpenShell 0.8.4 | 180-node capsule network, 131 252 B DSL | 40 | **40/40** |
| `mutate-semio-model` | Python impl from spec; artifact derived with IfcOpenShell 0.8.4 | Nakagin Capsule Tower, 181 elements / 362 relations, 119 066 B DSL | 34 | **34/34** |

151 scenarios, 151/151 parity, every one against a second implementation in another language.
**`mutate-semio-model` had never had a parity number before this wave, because its Rust subject had
never compiled.**

---

## 1. The audit, and what it checked

| Question | Answer |
|---|---|
| Is a real second producer registered per subset? | Yes — five `*-python-independent` oracle entries, `ecosystem: "python"`, `package: ""`. |
| Does any `noOracleDecision` or `@no-oracle-` tag survive in scope? | No. All five features carry `@oracle-…`; the only surviving mentions of the old decision ids are prose inside the new `rationale` strings, explaining what was replaced. |
| Do the Python adapters import or wrap the Rust? | No. Their entire import list is `json` (all five), `csv`/`io` (`✳️table`), `struct` (`✳️model`) and the framework host `semio_repo_test`. No `subprocess`, no FFI, no path into the plugin crate. |
| Does any Rust adapter still register oracles? | No — `grep -c '\.oracle('` is **0** in all five `🦀️component.rs`. Subject only, so our answer is never on both sides. |
| Was any evidence weakened? | No. Zero `ignoreKeys`, zero tolerance settings in any feature, adapter or manifest in scope. The profile is `ordered-json-v1` — "array order significant; key order never" (`📦️index.ts:66`) — no key exclusions, no numeric tolerance. |
| Do the catalogs still cover the whole vocabulary? | Yes. Each `mutationCatalogs[].kinds` is character-for-character the subset's own `pub const KINDS` — 8 / 7 / 9 / 13 / 11 for table/text/value/flow/model. |
| Are the artifacts real? | Yes, with checkable provenance. `📊️reuse-marketplaces.csv` and `🔣️hexagonal-cut-concrete-forest-left.model.json` in the case fixtures are **byte-identical** to the committed originals under `🗿️artifacts/📊️csv` and `🗿️artifacts/🔣️json` (`diff` exit 0, both). The two IFC-derived artifacts trace to the committed 2 496 437-byte `🏗️nakagin-capsule-tower.ifc`. |
| Does every declared kind have all three laws? | Yes — `mutate-<kind>`, `inverse-<kind>` and `spec-vector-<kind>` for every kind, plus `identity-round-trip`, plus `payload-fidelity` for `table`/`value`. The arithmetic matches the executed counts exactly. |

## 2. Defect 1 — `mutate-semio-model`'s Rust subject had never compiled

`📓️w16-semio-tabular-parity.md` §0 promised a parity figure for `✳️model` under "see §4"; §4 shows
only the ORACLE phase. The reason:

```
[test] ✏️s/…/🧪️tests/mutate-semio-model: rust subject host exited 101 without emitting results
error[E0277]: `semio_repo_test_host::Json` doesn't implement `std::fmt::Display`
   --> …/mutate-semio-model/🦀️component.rs:402:55
error[E0277]: the trait bound `&Vec<Vec<std::string::String>>: std::default::Default` is not satisfied
   --> …/mutate-semio-model/🦀️component.rs:377:37
error: could not compile `semio-test-host-mutate-semio-model` (bin "host") due to 2 previous errors
```

Corroboration that this was not a fresh regression: `⚡️cache/tests/results/…-mutate-semio-model-subject-rust/`
held projections dated **2026-08-25 11:15**, all ~978 bytes — the OLD two-node demo building, from
before the w16 rewrite — and no `📤️results.jsonl` at all.

**Fixed in the case's own adapter, without touching the framework:**

* `Json` has an inherent `to_string()` and **no `Display` impl** (verified: no `impl Display` anywhere
  in `🧬️protocol/🦀️component.rs`), so `disagreement` now calls `snapshot_json(got).to_string()`.
  `✳️table`/`✳️flow`/`✳️value` never hit this because their subsets export an
  `encode_*_snapshot_json` bridge returning `String`; `✳️model` exports none and its adapter builds
  the projection itself.
* `Context::data_table()` returns `Result<&Vec<Vec<String>>, String>` (`🏃️runner/🦀️component.rs:60`)
  and `&Vec<_>` is not `Default`, so `unwrap_or_default()` cannot type-check. Replaced with
  `if let Ok(rows) = ctx.data_table() { … }`.

First green run: `executed=68 passed=68 failed=0 errored=0 parity=34/34`, exit 0.

## 3. Defect 2 — `set-snapshot` did not reproduce a reordered collection

w16 fixed exactly this class in `✳️flow` with a `reproduces_order` guard and flagged it as latent in
the siblings. Code reading confirmed `✳️model`'s `between_named` was the pre-fix shape, so the
`set-snapshot` PARAMETER was strengthened to expose it — a replacement snapshot that drops the
building level and lists the storey BEFORE the site, two survivors in reversed relative order:

```
$ parity exhaustive --case mutate-semio-model --implementation rust        # exit 1
[test] level=exhaustive cases=1 executed=68 passed=67 failed=1 errored=0 parity=32/34
[test] parity failed: …::mutate-set-snapshot::rust::subject  (10 differences)
[test] parity failed: …::inverse-set-snapshot::rust::subject (1 differences)

oracle  spatial ids: ['25h1tviqb5o97WsO89tzwZ', '3hePCnUzPDnQxT0FznTQjx']   # storey, site
subject spatial ids: ['3hePCnUzPDnQxT0FznTQjx', '25h1tviqb5o97WsO89tzwZ']   # site, storey
```

**Cause.** `apply_named` retains survivors where they stand and pushes `added` onto the tail, so the
only key sequence it can produce is `survivors(base order) ++ added(target order)`. `between_named`
emitted a sparse triple regardless. Applying a snapshot did not make the document equal to that
snapshot.

**Fix**, in this subset's own `✳️model/🧬️schema/🔺️diff/🦀️component.rs`: the `reproduces_order` guard,
degrading to a full replacement when the sparse triple cannot reproduce the target's key sequence.
Strictly stronger than flow's copy — flow returns `None` early when all three vectors are empty,
which leaves a same-keys-different-order snapshot a silent no-op; this version checks order first.

## 4. Defect 3 — the shared preflight contradicted the applier it guards

The fix above immediately produced a THIRD, deeper failure, in role on the subject side:

```
mutate-set-snapshot: the mutation was rejected:
  ["Fatal FaultCode(\"mutation.apply.invalid-add-key\"): add key \"25h1tviqb5o97WsO89tzwZ\"
    already exists or is duplicated"]
```

`validate_named_triple` in `…/✳️any/🧬️schema/🧰️triples/🦀️component.rs` tested each added key against
the raw base keys — `if base_keys.contains(&key) || added.contains(&key)` — **without excluding the
keys the same diff REMOVES**. But `apply_named` performs `retain(!removed)` FIRST and then pushes
`added`, so `removed + added` of one key is well defined, is the only spelling this container has for
"move this member", and leaves the key present exactly once. The preflight forbade what the applier
handles correctly, which made a whole-collection replacement — the only faithful diff for a
reordering `set-snapshot` — unrepresentable in every subset that validates its collections.

`✳️flow` never hit it because its guard fires on a node's nested `params`, which is not validated;
`✳️model`'s `spatial` is a validated top-level collection.

**Fix:** `if (base_keys.contains(&key) && !removed.contains(&key)) || added.contains(&key)`, with the
reason in the docstring and a new unit test
(`named_preflight_admits_a_removed_key_being_re_added`) pinning both the acceptance and the resulting
order. The change is strictly RELAXING — any diff that validated before still validates, and the
existing `named_preflight_rejects_missing_and_colliding_keys` test (whose `removed` is empty) still
fails exactly as it did.

After it: `executed=68 passed=68 failed=0 errored=0 parity=34/34`, exit 0, **with the strengthened
parameter kept**, so a regression is red again.

## 5. Verification — real output

Every command from `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`; exit codes read from the tool's
own status, never through a pipe.

### Contract (after all three fixes)

```
$ bun ./📜️script.ts contract --owner 🗄️stdio                                    # exit 1
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery
  testing/discovery  🧰️framework  42 executable test file(s) outside the canonical owner-root test tree, baseline allows 35
  testing/discovery  ✏️s  4 executable test file(s) outside the canonical owner-root test tree, baseline allows 1
```

`⚡️cache/breaches/testing.json` read directly: a 2-element array, both `id: "unmanaged-tests"`,
`kind: "testing/discovery"`, scopes `🧰️framework` and `✏️s` — the same two the recipe recorded, owned
by other plugins' `.test.ts`/`.test.js` files. A machine check for the substring `mutate-semio-` over
the whole breach set returns **False**. `testing/contract`, `testing/oracle`, `testing/fixture` and
`testing/taxonomy` are all at zero. The exit code is the repo-wide count, not this scope's.

### Oracle phase

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio --case mutate-semio-table  # exit 0
[test] level=exhaustive cases=1 executed=26 passed=26 failed=0 errored=0 parity=0/0
$ … --case mutate-semio-flow    → executed=40 passed=40                          # exit 0
$ … --case mutate-semio-model   → executed=34 passed=34                          # exit 0
$ … --case mutate-semio-value   → executed=29 passed=29                          # exit 0
$ … --case mutate-semio-text    → executed=22 passed=22                          # exit 0
```

### Parity — the number that matters

```
$ bun ./📜️script.ts parity exhaustive --owner 🗄️stdio --case mutate-semio-table --implementation rust
[test] level=exhaustive cases=1 executed=52 passed=52 failed=0 errored=0 parity=26/26   # exit 0
$ … --case mutate-semio-flow   → executed=80 passed=80 … parity=40/40                   # exit 0
$ … --case mutate-semio-value  → executed=58 passed=58 … parity=29/29                   # exit 0
$ … --case mutate-semio-text   → executed=44 passed=44 … parity=22/22                   # exit 0
$ … --case mutate-semio-model  → executed=68 passed=68 … parity=34/34                   # exit 0
```

The four non-model runs above were re-run AFTER the shared `🧰️triples` change and are the numbers
quoted. `--implementation rust` is required for the pre-existing framework reason the recipe recorded
as trap 1; without it the oracle-only Python adapter is also run in the subject role and errors.

### Owner-wide regression after the shared `🧰️triples` change

```
$ bun ./📜️script.ts oracle exhaustive --owner 🗄️stdio                            # exit 0
[test] level=exhaustive cases=101 executed=2013 passed=2013 failed=0 errored=0 parity=0/0 not-exercised=7
```

Zero failures across all 101 stdio cases with the relaxed preflight in place. The 7 not-exercised
are `mutate-binary-raw`, `mutate-dwg-ac1018`, `mutate-dwg-ac1024`, `mutate-jpg-jfif-1-01-baseline`,
`mutate-semio-any`, `mutate-tiff-6-0-baseline` and `mutate-txt-utf-8` — all recorded `noOracleDecision`s
belonging to other agents' scopes; **none of the five in this scope appears.** (The recipe's baseline
for comparison was `cases=101 executed=1343 … not-exercised=24`.)

### Dependency ratchet

```
$ bun ./📜️script.ts dependency                                                   # exit 0
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30
```

Unchanged from the recipe's baseline — the five `package: ""` oracle entries contribute nothing.

## 6. Negative controls — the differentials do fail when they should

Each Python oracle was perturbed in exactly one verb, the oracle phase re-run, and the perturbation
reverted; every revert was confirmed by re-running to green.

| case | perturbation | result |
|---|---|---|
| `✳️model` | `insertElement` appends → inserts at 0 | `executed=34 passed=31 failed=3` — `inverse-remove-element`, `spec-vector-insert-element`, `spec-vector-remove-element` |
| `✳️table` | `ReorderColumns` move → swap | `executed=26 passed=25 failed=1` — `spec-vector-reorder-columns` |
| `✳️flow` | `setNodePosition` swaps x and y | `executed=40 passed=38 failed=2` — `inverse-set-node-position`, `spec-vector-set-node-position` |
| `✳️value` | `insertListItem` at index → append | `executed=29 passed=25 failed=4` — `inverse-` and `spec-vector-` of `insert-list-item` AND `remove-list-item` |

After reverting all four: `26/26`, `40/40`, `29/29`, `34/34` executed-and-passed again.

**One observation worth keeping.** `✳️table`'s swap perturbation produced exactly ONE red scenario,
and it was a `spec-vector-` one. `inverse-reorder-columns` stayed green because a swap is its own
inverse — the metamorphic law alone cannot tell a move from a swap. Only the committed
`(before, mutation, after)` vector caught it. That is the concrete argument for w16's decision to ADD
`spec-vector-` rather than substitute it.

## 7. Environment — two things the coordinator should know

**A peer session's os-kernel refactor blocked every Rust subject phase for about an hour.** First 10
`Send`/`Sync` errors at `🏪️store/🦀️component.rs:9098`, then 3 `E0308`s at `📡️spr/📜️history/🦀️component.rs:1258`
(a half-applied `Fn(u64) -> Result<&str, ProtocolError> + Send + Sync` bound). 2 289 modified `.rs`
files in the working tree at the time. It cleared on its own; nothing here was changed for it.

**The machine ran out of disk, twice, hard enough that the Bash tool itself returned `ENOSPC`.**
`⚡️cache` was **357 GB** — `agents/local/cargo-test-hosts/debug/deps` alone 199 GB across 766 559
files, `debug/incremental` another 77 GB — against 13 GiB free on a 926 GB volume. I did **not**
delete any of it: 21 `cargo`/`rustc` processes were running and 24 incremental directories had been
written in the preceding 20 minutes, so a clean-up would have destroyed peers' in-flight builds.

**A related framework limit worth recording:** the runner's 900 000 ms budget on the subject
`cargo run` is shorter than a cold rebuild of `semio-s-plugin-stdio` under contention — this wave
measured **27m01s** and **17m15s** for two such builds — so a case can be reported as failed purely
because its compile did not fit. The workaround used here was to `cargo build` the generated host
manifest first, outside the budget, then run parity against the warm target dir. Five consecutive
`mutate-semio-model`/`-text` attempts failed on this before that workaround; none was a case defect.

## 8. Files this wave touched

| File | Change |
|---|---|
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-model/🦀️component.rs` | two compile fixes (§2) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🧪️tests/mutate-semio-model/component.feature` | `set-snapshot` parameter strengthened to reorder surviving spatial nodes, and a paragraph saying why (§3) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️model/🧬️schema/🔺️diff/🦀️component.rs` | `reproduces_order` guard in `between_named` (§3) |
| `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/✳️any/🧬️schema/🧰️triples/🦀️component.rs` | `validate_named_triple` no longer rejects a re-added removed key; docstring and a new unit test (§4) |

Nothing else. No framework file, no manifest, no fixture, no comparison profile, no `Cargo.toml`, no
`🔒️dependencies.json`, no `project.json`, no `launch.json`. The four Python perturbations of §6 were
reverted and the reverts verified by re-running.

**One shared-module edit, declared.** `🧰️triples` lives under `✳️any` and is used by every `🧿️semio`
subset. The change is relaxing-only, its end-to-end effect is measured (§4), the five in-scope cases
were each re-verified green after it, and the owner-wide sweep
(`oracle exhaustive --owner 🗄️stdio` → `cases=101 executed=2013 passed=2013 failed=0`) shows no
regression anywhere in `🗄️stdio`.

**Not run here:** the new `named_preflight_admits_a_removed_key_being_re_added` unit test. Rust unit
tests go through `cargo-nextest` over the ROOT workspace, a build this machine could not fit beside
the peers' churn; the generated host workspaces the parity runs use are deleted between runs. The
behaviour it pins is nevertheless demonstrated end to end — `mutate-set-snapshot` went from rejected
with `mutation.apply.invalid-add-key` to `parity=34/34` on the real Nakagin model.

## 9. Still open

* **The order-loss class is not swept repo-wide.** `✳️model` is fixed and `✳️flow` was fixed by w16;
  `✳️kit`, `✳️graph`, `✳️brep`, `✳️object` and the other subsets carry their own copies of
  `between_named` and are not audited here. `✳️table` is not exposed (no `set-snapshot` in its
  vocabulary, and its `reorder-*` verbs are green). `✳️flow`'s copy still returns `None` early when
  all three vectors are empty and would therefore still miss a pure reorder; `✳️model`'s does not.
* **`insert-*` carries no index in these vocabularies**, so undoing a removal can only re-append and
  removing a non-last member is not invertible. The features state this and address the last member.
  It is a vocabulary gap, not a codec bug, and fixing it is a schema decision.
* **`parity` still needs `--implementation rust`** until the framework stops running an oracle-only
  adapter in the subject role.
