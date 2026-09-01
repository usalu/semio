# 🧱️ Everything remaining is ONE blocker, and it is now fully sized

Three things were being reported as separate remainders: 13 mutations without an external oracle,
`runtimeMutationCoverage` at 0.00%, and ~550 unwritten mutation leaves. They are one blocker.

## Subject-side is 0% for a reason that is now verified, not assumed

`runtimeMutationCoverage` is not "tests were run". `RuntimeMutationInventory`
(`🧪️test/📦️packages/🟦️typescript/📦️index.ts:2792`) is JSON that the BUILT implementation emits —
`{artifact, standard, subset, bridgeVersion, mutations: [{id, variant, verb, entity, record, outcomes}]}` —
and the dimension asks whether each subset's runtime ids match what its manifest declares. It is the
bridge between the manifest and the code.

**No inventory file exists anywhere in the repository**, because nothing can emit one:

* 64 plugin crates. **35 depend on `semio-s-plugin-stdio`** — every artifact-owning one.
* The 29 that do not are all EXTENSIONS (`flow-extension-*`, `process-*`, `cad-aec-*`,
  `imperative-*`, `sourcing-*`, `draw-fsm`, `playbook-procedural`). None owns a mutation manifest.

So subject-side execution is gated entirely on `semio-s-plugin-stdio`'s 60 `E0046`. The same crate
gates `mathematical` and `sequence`, which is the other 13. **One blocker, three symptoms.**

## Why the leaf migration is a vocabulary change — the one-grep proof

`#[derive(dsl::Mutations)]` asserts, per variant:

```rust
assert!(is_approved_verb(#kind::SEMANTICS.verb), "Mutations requires an approved semantic verb");
```

Measured against `APPROVED_VERBS`
(`🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🎮️command/🦀️component.rs:169`):

| verb | approved |
|---|---|
| `set` | ✅ |
| `replace` | ✅ |
| `set-snapshot` | ❌ |
| `no-mutation` | ❌ |

And **all 60** failing aggregates carry `NoMutation`.

So `NoMutation` and `SetSnapshot` **cannot become leaves**. Migrating any aggregate means DELETING them
from its vocabulary — which is exactly what `✳️drawing`, the migrated reference, did ("replacing the
old hand-rolled setter/whole-document-replace/bare-no-op-variant shape"), under an SMO ruling recorded
in its module docs.

That is why this is not the mechanical substitution the byte-identical wire format made it look like.
The wire format of a MIGRATED variant is unchanged — that was measured and holds — but two variants
stop existing, and every site that constructs or matches them has to say something else.

## The size, counted

| | count |
|---|---|
| aggregates failing `E0046` | **60** |
| leaves to author | **~550** |
| aggregates carrying `NoMutation` | **60 of 60** |
| `::NoMutation` sites (repo-wide) | **790** |
| `::SetSnapshot` sites (repo-wide) | **574** |
| total call sites to rewrite | **~1364** |

Each of the ~550 leaves additionally needs an approved `verb`, an `entity`, a past-tense `record`, and
`invertibility`/`diffParticipation`/`outcomeClasses`/`composition` in its `🔣️.json`. Some of that is
derivable from the existing match arms (a variant whose `inverse()` arm returns a real value is
`explicit-mutation`; one returning `NoMutation` is not), but the vocabulary question — what replaces
`NoMutation` and `SetSnapshot` in each of 60 artifacts — is a design decision per artifact, and
`✳️drawing` needed a ruling to answer it once.

## What this means for the ticket

Nothing in this ticket's own scope is blocked by it. `externalOracleCoverage` and
`oracleEvidenceCoverage` both read **601/614 (97.88%)** and are limited only by the 13, which trace to
the same crate. The fixture corpora, the oracle registrations and the 121 harness checks all stand
independently, because every fixture is authored third-party bytes that never pass through our
exporters.

What IS blocked is the end-to-end half: running our implementation against those fixtures. That is one
task, already filed, and it is the highest-value thing left in the repository — it unblocks 13
mutations, `runtimeMutationCoverage` for all 40 manifests, and the subject side of every corpus built
here.
