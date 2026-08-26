# Wave 14 — converting the fifteen 📕️norm no-oracle cases to a real differential

Date 2026-08-25. Ticket `26/08/23/END-TO-END-TESTING-REFACTOR`. Successor to
`📓️w13-cross-language-recipe.md`, whose recipe this follows. Every command quoted below was actually
run and every exit code is the tool's own, never a pipe's.

---

## 0. Headline

All **15** `📕️norm` cases now dispatch a real oracle. The `noOracleDecision` is gone from all fifteen
subset manifests.

```
before:  oracle exhaustive --owner 📕️norm  → cases=15 executed=0   … not-exercised=15   exit 0
after:   oracle exhaustive --owner 📕️norm  → cases=15 executed=799 passed=795 failed=4  exit 1
```

**`parity` is still 0/0 and the reason is not this work.** `semio-s-plugin-norm` does not compile —
671 errors at the time of writing, in ~2000 files a concurrent session is actively rewriting — so
there is no Rust subject to compare the oracle against. The conversion is complete and the moment
that crate is green `parity` produces a real number with no further change to any case.

The 4 red scenarios are findings, kept rather than removed. They are listed in §4.

---

## 1. What a 📕️norm artifact is, and whether anything third-party speaks it

Answered before assuming a reimplementation was needed, as instructed.

The fifteen are compliance-calculation artifacts: the ten Eurocodes EN 1990–EN 1999, DIN 4108,
DIN EN 16798, DIN V 18599, VDI 3805 and ISO 16757. **Twelve of them have no file format at all in the
outside world** — EN 1990 and DIN 4108 are calculation standards, not interchange standards, and our
artifact is a bag of typed design inputs carried in this repository's own `.dsl.semio`/`.pack.semio`
envelope. VDI 3805 and ISO 16757 name real external data formats, and DIN V 18599 a real calculation.

Checked against PyPI over the network rather than assumed:

| queried | result |
|---|---|
| `eurocode`, `vdi3805`, `iso16757`, `din18599` | **404 — no such distribution** |
| `structuralcodes` 0.7.1 | real; implements design-code MODELS (fib/EN 1992 material laws). No file format. |
| `concreteproperties` 0.8.0, `sectionproperties` 3.10.2, `anastruct` 1.7.0, `steelpy` 1.1.1 | real; cross-section and frame analysis. No file format. |

**No third-party library in any ecosystem reads or writes a `s.norm.*` artifact, and none could be
authoritative over its mutation vocabulary.** The reference had to be a second IMPLEMENTATION.

## 2. What that second implementation was written FROM

The obvious source — the subset's own committed grammar — turned out not to exist:

* **All fifteen `🧬️schema/📸️snapshot/📝️text/📖️component.grammar.semio` files are byte-identical
  boilerplate** whose whole body is `document = header body / header = "schema" SP "stdio.json" NL /
  payload = OCTET+`. None of them describes the format the committed artifacts are actually in. (The
  `🗄️stdio`/`🧿️semio` grammar w13 worked from is a REAL grammar; these are placeholders.)
* The `🧬️schema/📸️snapshot/🔣️component.json` JSON Schemas are shallow or stale — `en1990`'s still
  declares `qK` an array of entries, which it has not been since the composed-child migration — and
  `🧬️schema/🧬️mutations/🔣️component.json` is a **verbatim copy of the snapshot schema**, not a
  description of any mutation.
* The framework's DSL notation module (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🖋️notation`)
  publishes no grammar document either.

What DOES exist, and is a real written specification, is the mutation semantics:

* `26/08/12/SEMANTIC-MUTATIONS-OVERHAUL/📓️taxonomy.md` — the closed verb table with each verb's
  canonical arguments and inverse partner, the **naming mechanics** ("New-value fields are
  `new_<field>`; address fields are bare") and the **addressing convention** ("Inverse always computed
  from `base`", "removed/modified indices are BASE-state; inserted indices are FINAL-state",
  "`reorder`'s inverse is `reorder{from: min(to, len-1), to: from}`", "Missing target ⇒ `inverse`
  returns `Vec::new()`").
* the same ticket's `📓️derivation-rules.md` — the shape rules.
* each subset's committed `mutationCatalogs[].kinds`, for the closed list.
* the committed specification vectors, for each payload's JSON wire spelling.

So the Python implementation was written from those, and resolves the document field a `new*`
argument names **by normalised spelling against the document's own keys** — which is exactly what the
naming mechanic states, and is why one implementation reads both the snake_case (`new_g_k`) and the
camelCase (`newTOpC`) payload spellings the fifteen subsets committed. Nothing is imported from the
Rust and no mapping table is transcribed out of `🧬️mutations/**`.

### The number that says this is not fitted to the answer

The engine was run against **all 392 committed `(before, mutation, after, outcome)` vectors before any
of them had been inspected**. First run, from the two documents alone:

```
📕️din4108   22/22   📗️din16798 62/62   📘️en1991 32/32   📘️en1992 35/35   📘️en1993 17/17
📘️en1994    22/22   📘️en1995   20/20   📘️en1996 22/22   📘️en1997 22/22   📘️en1998 49/49
📘️en1999    26/26   📘️en1990    9/10   📙️din18599 12/13
📓️iso16757   1/21   📔️vdi3805   4/19
                                                   → 331 / 392 on the first run
```

Thirteen subsets are pure `change-<field>`/`update-<facet>` vocabularies and the two documented
mechanics reproduced them outright. `📓️iso16757` and `📔️vdi3805` are rule-2 id-keyed-collection
vocabularies over nested containers; their container resolution was added afterwards, by reading the
snapshot SHAPE (which `📓️derivation-rules.md` directs an implementer to do) and refined over several
runs against the committed vectors. **That is a weaker kind of evidence than the thirteen and it is
recorded in their feature files, not levelled up by silence.**

## 3. Shape of each converted case

Per subset, `mutate-<kind>` and `inverse-<kind>` for every declared kind plus `identity-round-trip`.
Both implementations read the SAME committed bytes and both read them from the FEATURE — every
`(before, mutation, after, outcome)` path is a declared `asset://` fixture — so neither side holds a
transcription that could drift from what the other read.

Each side asserts three laws in role: the applied document must BE the committed after-snapshot; an
`applied` vector must move the document and a `rejected` one must leave it bit-identical; and the
mutation followed by its OWN computed inverse must restore the before-snapshot. `inverse-` projects
**both** the mutated and the restored document — projecting only the restored one would make every row
of the table project the same value and the differential vacuous.

`identity-round-trip` compares the two implementations at the CARRIER level — `{preamble, lines,
dslDigest, dslLength}`, where `digest` is the coordinator's own sha256 and is therefore directly
comparable across the two languages. It deliberately does not map carrier tokens onto the JSON
snapshot's enum spellings (`annex=en` ⇄ `"En"`, `masonry-class=class2` ⇄ `"Class2"`, `unit=clay` ⇄
`"clay"`): that mapping is stated nowhere, it is not even consistent between subsets, and inferring it
would be reverse-engineering rather than a second reading.

The Rust adapters were reduced to the SUBJECT half — their oracle registrations are gone, because
keeping them would put this repository's own answer on both sides of the comparison.

## 4. The four red scenarios — all findings, all kept

### 4.1 `📘️en1990` `mutate-` / `inverse-insert-variable-action` — a document that cannot be reproduced

`En1990Snapshot.q_k` is not an inline list but a composed `s.stdio.semio.table` CHILD slot. The
persisted document carries only a handle, and:

* the handle's `childId` is `format!("en1990-qk-{:016x}", DefaultHasher(serde_json::to_string(entries)))`
  — a value Rust's own documentation declares **unspecified and unstable across releases**;
* the entries themselves live in a `thread_local!` scratch cache (`EN1990_QK_SCRATCH`) that is **not
  part of the persisted document at all**.

So no second implementation in any language can mint that identity or read those entries: our answer
to `insert-variable-action` is unreproducible by construction. The other four collection kinds agree,
because their committed outcome is `rejected` and both sides refuse them.

**This is a defect in the codec, not in the reference.** A content-addressed identity that is written
into a persisted document has to use a specified hash.

### 4.2 The two subsets disagree with each other about composed slots

`📙️din18599`'s `update-climate` addresses the same kind of composed child slot and its committed
outcome is **`rejected`** — both implementations refuse it and agree. `📘️en1990`'s
`insert-variable-action` on the same kind of slot is committed as **`applied`**. Two norm subsets take
opposite decisions on the same construct.

### 4.3 `📓️iso16757` / `📔️vdi3805` `identity-round-trip` — a carrier with no grammar

These two carriers nest records and tables and flatten nested records into `key=key=value` runs with
no delimiter (`catalogue=id=cat.demo metadata=edition-profile=fullPublished names=short-name=Demo …`).
With the committed grammar being the `payload = OCTET+` placeholder and the framework publishing none
either, reconstructing those bytes would mean inferring a grammar from one committed example. The
Python side **refuses, with that explanation as the failure message**, rather than handing back the
bytes it was given and calling it a round trip. Their mutation vocabularies are unaffected — 42 of 43
and 38 of 39 scenarios pass.

## 5. Findings that are NOT red yet, because nothing can run them

### 5.1 All nine committed `.pack.semio` twins in 📕️norm are fabricated placeholders

Every `identity-round-trip` Rust handler in this plugin decodes the committed binary twin and asserts
it describes the same document as the text artifact — and nine subsets' feature files state that as
evidence ("two separately committed files written by two separate codecs have to describe the same
document"). Measured:

| subset | bytes | payload after the 31-byte envelope |
|---|---|---|
| en1990, en1991, en1993, en1994, en1995, en1996, en1998, en1999 | 159 | **128 zero bytes** |
| en1992 | 168 | inner header fragment, the literal ASCII `annex=en tc2`, zeros, then `0xAA` filler |

`decode_pack` reads a format byte of `0x00` against `PACK_BINARY_FORMAT = 1` and fails. **Not one of
the nine is a real encoding of its document**, and the claim in nine feature files is false. It has
never been caught because `semio-s-plugin-norm` has never compiled. The other six subsets
(iso16757, vdi3805, din4108, din16798, en1997, din18599) commit no pack twin at all.

### 5.2 `semio-s-plugin-norm` does not compile — 671 errors

```
262 error[E0015]  cannot call non-const associated function in constants   (📔️vdi3805/🦀️component.rs:515)
315 error         method should be async or return a future, but it is synchronous
 45 error[E0609]  no field on type: field not available in ...
 31 error[E0277]  ... is not a future
 15 error[E0308]  mismatched types
  1 error[E0433]  unresolved module
```

Not touched: `git status` shows **1997 modified files under `✏️s/🔌️plugins/📕️norm/`**, the newest
written 13 minutes before this note. A concurrent session owns that refactor; `semio-s-plugin-stdio`
went from 30 errors to **exit 0** during this session, so it is progressing. Editing norm production
Rust would have collided with live work.

## 6. Verification — real output

From `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test`.

```
$ bun ./📜️script.ts oracle exhaustive --owner 📕️norm                                # exit 1
[test] level=exhaustive cases=15 executed=799 passed=795 failed=4 errored=0 parity=0/0

$ bun ./📜️script.ts oracle exhaustive --owner 📕️norm --case mutate-en1996-1         # exit 0
[test] level=exhaustive cases=1 executed=45 passed=45 failed=0 errored=0 parity=0/0

$ bun ./📜️script.ts contract --owner 📕️norm                                          # exit 1
2 high-priority breach(es) across 1 rule(s):
      2  testing/discovery
  testing/discovery  🧰️framework  42 executable test file(s) …, baseline allows 35
  testing/discovery  ✏️s           4 executable test file(s) …, baseline allows 1
```

`breaches/testing.json` read directly: 2 records, **zero name any 📕️norm path**; the
`testing/contract`, `testing/oracle`, `testing/fixture` and `testing/taxonomy` families are all at
zero. Both breaches are other plugins' `.test.ts`/`.test.js` counts, present before this work and
identical to the pair `📓️w13-cross-language-recipe.md` reported.

```
$ bun ./📜️script.ts dependency                                                        # exit 0
[dependency] ecosystems=4 entries=232 production-reachable=151 test-oracle=30
```

Unchanged. All fifteen oracle entries carry `"package": ""`, so they link nothing and contribute
nothing to the ratchet.

### Parity — the number that matters, and why it is 0/0

```
$ bun ./📜️script.ts parity exhaustive --owner 📕️norm --case mutate-en1996-1 --implementation rust
[test] level=exhaustive cases=1 executed=45 passed=45 failed=0 errored=0 parity=0/0
[test] …/mutate-en1996-1: rust subject host exited 101 without emitting results
[test] …/mutate-en1996-1: no result stream at …/mutate-en1996-1-subject-rust/📤️results.jsonl
                                                                                       # exit 1
```

45 oracle results, 0 subject results, so **0 comparisons**. `--implementation rust` is w13's trap-1
workaround and is not what limits this: the subject host cannot be built. During this session
`semio-s-plugin-stdio` went from 30 errors to `cargo check … --lib` **exit 0** — the peer refactor is
moving through the crates — while `semio-s-plugin-norm` stayed at **671**. Nothing in these fifteen
cases has to change for parity to light up; the oracle is registered, dispatched and green.

### The negative control — the differential fails when it should

`insert`'s FINAL-state index rule was temporarily removed from `📕️din4108`'s Python
(`items.insert(min(index, len(items)), element)` → `items.append(element)`) and the case re-run:

```
[test] level=exhaustive cases=1 executed=45 passed=42 failed=3 errored=0 parity=0/0     # exit 1
  mutate-insert-layer   the applied document does not match the committed after-snapshot
  inverse-insert-layer  undoing the mutation did not restore the before-snapshot
  inverse-remove-layer  undoing the mutation did not restore the before-snapshot
```

Reverted:

```
[test] level=exhaustive cases=1 executed=45 passed=45 failed=0 errored=0 parity=0/0     # exit 0
```

Three scenarios red from one rule, including the inverse of the SIBLING kind — the laws are asserted
in role, not merely projected.

## 7. Files

Per subset `<A>` of the fifteen under `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<A>/`:

| file | change |
|---|---|
| `🧪️tests/mutate-<slug>-1/🐍️component.py` | **new** — the independent Python implementation and its oracle adapter |
| `🧪️tests/mutate-<slug>-1/component.feature` | rewritten — `@oracle-<slug>-1-python-independent`, `@mode-differential`, every input a declared `asset://` fixture |
| `🧪️tests/mutate-<slug>-1/🦀️component.rs` | subject only; oracle registrations dropped, `inverse-` projects `{mutated, restored}`, `identity-round-trip` projects the carrier |
| `🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️oracle/🔣️component.json` | `noOracleDecisions` emptied, `oracles[]` gains the entry |

Nothing else was edited: no framework file, no shared manifest, no `Cargo.toml`, no
`🔒️dependencies.json`, no fixture, no comparison profile, no `ignoreKeys`, no production Rust.

Scratch: `🐍️w14-norm-spec-engine-probe.py` (the first-run probe of §2) in this ticket folder.
