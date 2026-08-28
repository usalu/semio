# 📓️ Why 5 of 2 147 mutations are externally oracled, and exactly what unblocks the rest

The target is that **every mutation of every artifact is predicted by a third-party library, scoped to
the smallest semantic subset**. Today 5 kinds meet that (0.2%). This is the measured chain of why, with
the command that produces each number, so the remaining work is enumerable rather than estimated.

Nothing here is an estimate. Every figure comes from running the command beside it.

## The chain

```
payload schema                1 394 of 1 710 leaves have none          ← test manifest scaffold
  └→ leaf descriptor          1 689 of 1 710 not derivable  (1.2% are)  ← test manifest scaffold
       └→ mutation manifest   0 owners ready                            ← test manifest
            └→ runtime inventory  0 of 1 manifests have one             ← test inventory
                 └→ fixtures       121, all for one subset              ← test fixture audit
                      └→ coverage  5 of 2 147 kinds  (0.2%)             ← test gap
```

And, independently of that chain:

```
1 568 of 2 147 kinds have NO qualifying third-party oracle              ← test gap
   1 299 backed only by a second implementation written in this repository
     269 backed by nothing at all
```

Both fronts must move. Neither is a bottleneck the other unblocks.

## Front 1 — the vocabulary is largely undeclared

A v2 manifest must state the OUTCOME CLASSES each mutation can reach, and that is the one field nobody
can honestly invent from outside the implementation. It already has a home: the `dsl::Mutations` derive
reads a fourteen-field JSON descriptor per leaf at macro-expansion time
(`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️component.rs`, `parse_mutation_leaf_descriptor`).
That descriptor is declarative and language-neutral, so a manifest generated from it is generated from
production's own record rather than restated beside it.

**6 of 1 944 leaves in the tree carry one.** That is why the workspace does not compile: the derive
requires the descriptor and almost no leaf has it.

`test manifest scaffold` derives descriptors from the leaves themselves and REFUSES any field it cannot
cite a file and line for. Over the 1 710 leaves reachable from a contribution:

| Field refused | Leaves | Why |
| --- | --- | --- |
| `payloadSchema` | 1 394 | the leaf carries neither `🧬️schema/🔣️.json` nor `🔣️payload.schema.json` — the mutation's payload has no contract at all |
| `binaryTag` | 1 366 | the owner's `💾️binary/📡️component.protocol.semio` does not declare this kind |
| `outcomeClasses` | 292 | no `🔺️diff` implementation to read the semantics from |
| `aggregateVariant` | 50 | no `pub struct` in the leaf's Rust |

`payloadSchema` is the deepest: **you cannot write a fixture for a mutation whose payload has no
contract**, so 1 394 leaves are unreachable by any amount of testing effort until that is authored.

The `binaryTag` refusals are not all absence — some are DRIFT. `s.cad.cad`'s binary protocol declares
14 records and every one of them is a verb retired in ticket
`26/08/12/UNIFIED-COMPOSABLE-ARTIFACT-SYSTEM` wave 3 (`add-object`, `patch-object`,
`translate-objects`, …). None of the twenty kinds the enum actually dispatches appears. The scaffolder
refuses rather than inventing a wire number nothing on the wire agrees with.

**4 owners are fully derivable today** — `✒️writer` (4 leaves), `🌿️vcs` (6), `🎬️sequence` (8),
`🗂️curate` (3) — and **none of the four has a qualifying oracle**, so a manifest for any of them would
declare a requirement nothing can discharge. The two fronts meet here.

## Front 2 — 1 568 kinds have no third-party reference

`test gap` reports this per owner. The largest single category is 1 299 kinds whose only registered
"oracle" is a second implementation written inside this repository from this repository's own schemas.
Those read as oracles in the registry and discharge nothing: both halves read the same specification,
so they catch a transcription error and cannot catch a misreading.

This is not a labelling problem to be fixed by relabelling. For a Semio-native document vocabulary
there IS no third party that implements it, and the honest route is the one the BRep pilot takes: pick
a STANDARD CARRIER the mutation's result can be expressed in — STEP, glTF, PDF, OOXML, PNG — and let a
third-party reader of that carrier predict and verify the result. That is a per-owner research and
integration job, and `test gap --json` is its worklist.

## What was built so this is mechanical rather than exploratory

| Command | Answers |
| --- | --- |
| `test gap` | per owner: covered / manifestable / supplemental-only / un-oracled, with what is owed |
| `test manifest` | which owners can have a manifest generated, and precisely what blocks the rest |
| `test manifest scaffold [--write]` | derives leaf descriptors with citable evidence; refuses rather than guesses |
| `test inventory` | runs the owner's production bridge; still blocked by the workspace not compiling |
| `test matrix --enforce` | the sixteen dimensions and the six release gates |

The scaffolder writes an owner ALL-OR-NOTHING. A partial descriptor set would let a manifest be
generated over a denominator that silently omits the undescribed leaves — coverage of a smaller
vocabulary, reported as coverage.

## The honest summary

The BRep pilot is complete and is the worked example: one subset, 121 STEP fixtures across all three
fixture classes and all five outcome classes, two qualifying oracles on different engine families, a
gating mesh comparison on a third, and 92 adversarial checks proving the gates catch what they claim.

Generalising it to 2 147 mutations is blocked on 1 394 missing payload contracts and 1 568 missing
third-party references. Both are now counted, attributed per owner, and reachable by a command.
