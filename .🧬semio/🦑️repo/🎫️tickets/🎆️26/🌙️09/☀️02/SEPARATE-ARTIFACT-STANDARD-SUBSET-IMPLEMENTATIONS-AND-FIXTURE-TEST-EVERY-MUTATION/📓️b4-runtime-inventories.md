# B4 — Runtime inventories + leftover mutation vocabularies

Shard B4 of `SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
Scope: `runtime-inventory-missing` (63 assigned subsets) and `unregistered-mutation-vocabulary`
(the 8–10 new leftovers from this ticket's splits, plus re-verifying A9's 33 pre-existing ones).

## Headline result

**Breach 1 (`runtime-inventory-missing`) could not be advanced to real, measured inventories this
session.** Not because the measurement design is unsolved (it is — see below, and it is verified
against real source, not guessed) but because the native Rust build that EVERY bridge must run
through is currently broken repo-wide, for reasons entirely outside this shard's territory, caused
by another session's in-flight migration. Zero fabricated/hand-typed inventory files were written —
per the ticket's own rule, doing that would defeat the entire point ("runtime completeness is a
measurement, not a claim"), so the honest outcome is zero produced rather than sixty-three faked.

**Breach 2 (`unregistered-mutation-vocabulary`) is fully investigated: 0 safe fixes, 43 documented.**
The brief's premise for the "8 new" ones ("now empty of mutations… remove the leftover directory")
does not hold against the current tree — I checked all ten candidates file-by-file and every one
still holds live, in-use code (the artifact's shared aggregate mutation enum plus its wire-codec
files). Deleting any of them would delete production code. The 33 pre-existing ones were
re-verified against A9's analysis and the structural blocker A9 found is still exactly in place.

## Before / after

| id | before (session start) | after |
| --- | --- | --- |
| `runtime-inventory-missing`, my 63 paths | 63 | **63 (unchanged — all 63 confirmed still present, byte-identical scope list)** |
| `runtime-inventory-missing`, repo total | 63 | 163 (100 new ones appeared from OTHER shards splitting more subsets since ticket start — not mine, not touched, listed for transparency) |
| `unregistered-mutation-vocabulary` | 43 | 43 (unchanged) |
| `runtime-only-mutation` | 0 | 0 |
| `manifest-only-mutation` | 0 | 0 |
| `mutation-outcome-mismatch` | 0 | 0 |
| `mutation-variant-mismatch` | 0 | 0 |

The four inventory-comparison ids are 0/0 both times because `compareInventories` only ever
populates them once a runtime inventory has actually been produced (`readRuntimeInventory !==
null`) — with zero inventories produced, there is nothing yet for them to disagree with. That is
the honest number, not a hidden success.

Judge: `bun ./📜️script.ts test contract`, run once at session start (read from the ticket's own
baseline `🗑️generated/breach-runtime-inventory-missing.json` / `breach-unregistered-mutation-
vocabulary.json` plus a fresh read of `.🧬semio/🦑️repo/⚡️cache/breaches/testing.json`) and once at
the end (`🗑️generated/b4-test-contract-final.txt`, full output, non-zero exit as always expected —
`testing.json` fully rewritten regardless, confirmed 2130-entry complete dump, not a partial/crashed
run).

## Part 1 — Breach 1: why zero inventories were produced, and what would produce them

### The invocation gap (a real, separate finding)

The rule's own remediation text says `test inventory --artifact <id> --standard <v> --subset <s>`.
Run literally as written — `bun ./📜️script.ts test inventory --artifact … `, i.e. through the ROOT
script — this does **not** reach `InventoryScript` at all. `TestScript.run` (root `📜️script.ts:18996`)
only routes a `test <phase>` call into the testing domain's own router when `phase` is a member of
`taxonomy.testPhases` (`🔣️taxonomy.json:13197`). That list is `["discover", "doctor", "contract",
"oracle", "subject", "parity", "run", "report", "metrics", "clean", "dependency", "nx"]` —
**`"inventory"` is not in it.** So `test inventory …` silently falls through to the root's default
branch, which runs `contract` and then a full `nx run-many` sweep — I hit this on the first try (see
`🗑️generated/b4-step-cc6-inventory.txt`, a full breach dump instead of an inventory run).

The command **does** work when the test module's own `📜️script.ts` is invoked directly, exactly the
way the root's own `TestScript` invokes every other phase internally:

```
bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts inventory --artifact <id> --standard <v> --subset <s>
```

This is a genuine framework gap (the rule's documented remediation text names a command surface that
does not resolve through the root entrypoint it is written for) — worth a one-line fix to
`taxonomy.json`'s `testPhases` by whoever owns that file next; not touched here since it is outside
this shard's assigned scope and `🔣️taxonomy.json` is a taxonomy SSOT other shards may also be
mid-editing.

### The build blocker (confirmed, reproduced, not mine)

Running the corrected command against the one subset that already has a bridge —
`s.stdio.step@ap214/cc6` (bridge at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/
🪆️subsets/✳️cc6/🏭️bridge/`, pre-existing, not written by this shard) — reaches the bridge and fails
to compile it:

```
[inventory] s.stdio.step@ap214/cc6: bridge exited 1 — error: could not compile `semio-framework-plugin`
(lib) due to 6 previous errors; 108 warnings emitted
```

Full build output: `🗑️generated` (captured, not committed — deleted per house rules before close).
The six errors are all `E0277` trait-bound failures at
`🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust/../../🦀️.rs:29878` and
neighbouring lines: `semio_framework::manifest::ActionInvocation`, `CommandInvocation`,
`MediaFingerprint` and `dsl::io_schema::IoPayload` are passed to `serde_json::from_str`/
`serde_json::to_string`-shaped calls that require `serde::Deserialize`/`Serialize`, but those types
no longer derive serde — they derive this repo's own `ToValue`/`FromValue` instead
(`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs:1330`, whose own docstring says why: "ticket
`26/09/01/RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS`: `arguments` used to be keyed
to `serde_json::Value`… `DslValue` carries the exact same schema-less-JSON shape without the serde
dependency"). That ticket is **mid-flight, in a different session**: `git log --date=iso` on
`🧰️framework/🔨️modules/🛂️manifest/🦀️.rs` shows its newest commit at `2026-09-02 15:18:09 +0200`,
fifteen minutes before I ran the build. The producer side of the migration (the struct) has landed;
the consumer side (`🔌️plugin`'s own `serde_json::from_str::<ManifestActionInvocation>(...)` call
site) has not caught up yet. Confirmed **not** feature-flag-avoidable:
`semio-s-plugin-stdio`'s own `Cargo.toml` lists `semio-framework-plugin` as an unconditional
dependency (`✏️s/🔌️plugins/🗄️stdio/📦️packages/🦀️rust/Cargo.toml:44`, outside the `[features]`
block), and the cc6 bridge's `default-features = false` (already set, pre-existing) does not remove
it.

Confirmed this is **repo-wide, not stdio-specific**: the same `could not compile
semio-framework-plugin (lib) due to 6 previous errors` fired identically building
`semio-s-plugin-mathematical`'s own lib (`✏️s/🔌️plugins/➗️mathematical/📦️packages/🦀️rust`, a
completely separate plugin) and `semio-s-plugin-sequence`'s lib, both against an isolated
`CARGO_TARGET_DIR` with `RUSTC_WRAPPER=""` (per house rules, so this is not sccache serialization
or shared-target contention — it is the same six real compile errors both times). `fem`/`architect`/
`draw` checks were still compiling (unrelated warnings only, no errors yet reached) when a 2-minute
bound was hit — consistent with the same shared dependency graph, not contradicting it. Every one of
the 24 artifacts this shard would need a bridge for links this exact plugin-host crate, so this
blocks all of them simultaneously, not just the one I happened to test first.

Retried once more near the end of the session (same six errors, same file). Per the ticket's own
guidance ("if a repo-wide cargo failure looks unrelated to your files, check whether it precedes
your edits before blaming yourself… poll rather than chase") and the coordinating brief's explicit
anticipation of exactly this scenario, I did not chase it further or attempt to fix
`🔌️plugin`'s call site myself — it is another session's active migration, in a file this shard has
no ticket authority over, and hand-patching it mid-migration risks fighting that session's own
in-progress edits.

### The measurement design that IS verified (ready to run once the build is healthy)

This was worked out from real source, not guessed, and is considerably simpler than the pre-existing
`cc6` bridge's hand-written `every_variant()` approach:

1. **Every aggregate mutation enum already carries a generic, compiler-derived kind roster.**
   `#[derive(dsl::Mutations)]` (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:1692`)
   generates `impl protocol::SemanticMutation<Snapshot> for AggregateEnum { fn kinds() -> &'static
   [SemanticDescriptor] { … } }` — a **type-associated function**, callable without constructing any
   instance, returning one `{ verb, entity, kind, record }` row per variant, in declaration order,
   validated at compile time (`assert_eq!(SEMANTICS.kind, expected_kebab)` inside the derive itself).
2. **`protocol` is only a private in-crate alias for a real, independently-dependable crate.** Every
   plugin does `extern crate semio_framework_os_kernel as protocol;` at its own crate root
   (confirmed for `➗️mathematical`, `🗄️stdio`) — an unqualified `extern crate … as X` binding is
   crate-private, so an external bridge cannot reach `protocol::SemanticMutation` through the
   plugin's own alias. But `semio-framework-os-kernel` is a perfectly ordinary path dependency
   (`{ path = "…/🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust", package = "semio-framework-os-kernel"
   }`) that a bridge crate can add **directly**, then call
   `<AggregateEnum as semio_framework_os_kernel::SemanticMutation<Snapshot>>::kinds()` itself. No
   `every_variant()` construction, no field-default guessing, needed at all.
3. **Subset attribution is a real, compile-validated fact, not a guess.** Every mutation leaf carries
   a committed sidecar `<kind>/🔣️.json` (e.g.
   `✏️s/🔌️plugins/➗️mathematical/…/✳️equation/🧬️schema/🧬️mutations/🔄️change-coefficient/🔣️.json`)
   whose `"owner"` field is the leaf's own full source directory path, and
   `dsl::MutationLeaf`'s derive (`🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs:595`)
   **rejects the build** if that string does not match the file's real location
   (`if owner != authority.owner { return Err("descriptor owner does not exactly match source
   owner") }`). Reading `owner` back out of the kind's own `🔣️.json` and taking the `✳️<subset>`
   segment is therefore reading a fact the compiler already enforces, not trusting an
   independently-maintained label. Verified this against every kind under the artifacts in this
   shard's 63; the mapping is unambiguous everywhere I checked (mathematical's equation/geometry/
   graph split, fem2d's mesh/material/boundary/load/analysis split, etc.).
4. **Outcome classes need a translation, and the honest limitation is disclosed rather than hidden.**
   Each leaf's sidecar `🔣️.json` also declares `outcomeClasses` from the FRAMEWORK's generic severity
   vocabulary (`applied|info|warning|error|fatal`,
   `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🦀️.rs:339`) — a **different axis** from the
   domain vocabulary `compareInventories` actually checks against
   (`MUTATION_OUTCOME_CLASSES = ["applied","no-op","empty","disjoint","rejected"]`,
   `🟦️.ts:2742`). No compiled artifact anywhere states the domain classes generically; the
   pre-existing `cc6` bridge hand-wrote them per kind from reading the real guard logic
   (`outcomes_of()`), which is the fully-rigorous approach but does not scale to ~600 mutation kinds
   across 24 artifacts in one session. The scoped, disclosed compromise I verified is defensible
   rather than fabricated: translate `applied→applied`, `error→rejected`, `fatal→rejected`,
   `warning→applied`, `info→no-op`, embed it per-kind in the generated bridge from the sidecar JSON
   (still real compiled data, just translated by a fixed, documented rule), and **flag every
   resulting outcome mismatch as a candidate finding for manual confirmation** rather than silently
   suppressing it. Spot-checked this rule against `mathematical`'s `change-coefficient`: sidecar
   `outcomeClasses = [error, info, applied]` → translates to `{rejected, no-op, applied}`; the v2
   manifest currently declares only `{rejected, applied}` (missing `no-op`) — and the crate's own
   `#[cfg(test)]` suite already has `change_coefficient_at_an_unknown_label_is_a_no_op`, i.e. the
   translation surfaced a **real, pre-existing gap** the manifest is missing, exactly the kind of
   finding this rule exists to catch. This is strong evidence the translation rule produces signal,
   not noise — but it is disclosed as approximate, not claimed as exact, and no fixture-comparison
   breach was filed from it since no inventory was ever actually produced this session.
5. **One bridge per artifact (not per subset) is idiomatic here, not an invented shortcut.**
   `mutationBridgeFor` (`🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts:333`) walks from
   the subset owner up through every ancestor looking for `🏭️bridge/📜️script.ts`, and its own
   comment says why: "so a subset inherits the one its artifact publishes instead of every subset
   needing its own copy." One bridge crate per artifact, filtering `kinds()`'s full roster by the
   sidecar-JSON-derived subset per invocation, needs 24 Cargo crates instead of 63 (mathematical:
   1 crate serves equation/geometry/graph; gif: 1 crate serves both 87a and 89a standards; etc.) —
   less duplication, matches the framework's own stated intent, and is what I would generate first
   once native builds are healthy again.

### Concrete per-artifact facts collected, ready to generate from

| artifact | crate | module | Snapshot | aggregate enum |
| --- | --- | --- | --- | --- |
| `s.mathematical.mathematical` | `semio-s-plugin-mathematical` | `mathematical` | `MathematicalSnapshot` | `MathematicalMutation` (15 kinds, split equation/geometry/graph) |
| `s.sequence.sequence` | `semio-s-plugin-sequence` | `sequence` | `SequenceSnapshot` | `SequenceMutation` |
| `s.fem.2d` | `semio-s-plugin-fem` | `fem2d` | `Fem2dSnapshot` | `Fem2dMutation` (25 kinds, all still owned by `✳️any`'s v2 manifest despite mesh/material/boundary/load/analysis subset dirs already existing) |
| `s.fem.3d` | `semio-s-plugin-fem` | `fem3d` | `Fem3dSnapshot` | `Fem3dMutation` |
| `s.architect.program` | `semio-s-plugin-architect` | `program` | `ProgramSnapshot` | `ProgramMutation` (268 kinds — by far the largest) |
| `s.draw.draw` | `semio-s-plugin-draw` | `draw` | `DrawSnapshot` | `DrawMutation` |
| `s.note.note` | `semio-s-plugin-note` | `note` | `NoteSnapshot` | `NoteMutation` |
| 17 stdio artifacts (`las`,`gif`,`svg`,`bcf`,`pdf`,`step`,`docx`,`xml`,`jpg`,`png`,`avi`,`json`,`dxf`,`bmp`,`tiff`,`gltf`,`obj`,`semio`) | `semio-s-plugin-stdio` | `artifacts::<name>` | per-subset | **already split per-subset** (each subset owns its own aggregate enum, e.g. `StepCc1Mutation`…`StepCc6Mutation`) — one bridge per artifact importing every subset's own type and switching on the `--subset` CLI arg |

None of this was compiled or executed — it is source-verified design, explicitly not claimed as a
working measurement. No inventory cache files were written under
`.🧬semio/🦑️repo/⚡️cache/tests/results/🏭️inventory/`.

## Part 2 — Breach 2: `unregistered-mutation-vocabulary` (43 total)

### The 8–10 "new" ones the brief expected to be empty — verified NOT empty, NOT deleted

The brief's premise: this ticket's subset splits emptied the old artifact-wide `✳️any/🧬️schema/
🧬️mutations` directory of `draw`, `fem2d`, `fem3d`, `mathematical`, `note`, `sequence`, and it
should be confirmed-empty and removed. I checked every one, file by file
(`find <dir> -type f | wc -l` plus reading the content):

| directory | files | disposition |
| --- | --- | --- |
| `draw` `✳️any/🧬️schema/🧬️mutations` | 6 | **not empty** — holds the live `DrawMutation` aggregate enum + wire schema (`.graphql`/`.proto`/`.ts`/`.grammar.semio`) |
| `draw` `✳️any/🚪️io/🧬️mutations` | 14 | **not empty** — real `💾️binary`/`📝️text` codec implementations for that same enum |
| `fem2d` `✳️any/🧬️schema/🧬️mutations` | 20 | **not empty** — live `Fem2dMutation` (25-variant) aggregate enum, still the ONLY v2-manifested owner of all 25 kinds |
| `fem3d` `✳️any/🧬️schema/🧬️mutations` | 20 | **not empty** — same pattern as fem2d |
| `mathematical` `✳️any/🧬️schema/🧬️mutations` | 6 | **not empty** — live `MathematicalMutation` aggregate enum (imports all 15 leaves from equation/geometry/graph, see Part 1) |
| `mathematical` `✳️any/🚪️io/🧬️mutations` | 14 | **not empty** — codec for the same enum |
| `note` `✳️any/🧬️schema/🧬️mutations` | 6 | **not empty** — live `NoteMutation` aggregate enum |
| `note` `✳️any/🚪️io/🧬️mutations` | 14 | **not empty** — codec |
| `sequence` `✳️any/🧬️schema/🧬️mutations` | 5 | **not empty** — live `SequenceMutation` aggregate enum |
| `sequence` `✳️any/🚪️io/🧬️mutations` | 14 | **not empty** — codec |

Deleting any of these would delete production code — in several cases (fem2d/fem3d/mathematical) the
exact enum this shard's own bridge design in Part 1 needs to call. **None were touched.**

The actual reason they trip the walker: `unregistered-mutation-vocabulary`
(`🟦️.ts:1793`) walks every directory literally named `🧬️mutations` and checks whether
`owner = dirname(dirname(rel))` (i.e. the `✳️any` subset itself) has ANY contribution with
`mutationCatalogs.length > 0` — a **v1** concept, checked independent of whether a v2
`mutationManifest` exists. Confirmed directly: `mathematical`'s `✳️any/🧪️oracle/🔣️.json` has
`mutationCatalogs: []` (zero) despite the aggregate enum's own docstring claiming "this catalog
stays in `✳️any`" — that comment is now stale; whatever v1 catalog it referred to was removed by an
earlier shard's edit and nothing replaced it. `fem2d`'s `✳️any/🧪️oracle/🔣️.json` is the same: zero
v1 `mutationCatalogs`, one v2 `mutationManifest` (subset `any`, 25 mutations) — while its own
`✳️mesh` subset (already split out, has its own `🧪️oracle/🔣️.json`) claims 11 of those 25 kinds via
a v1 catalog with **no v2 manifest at all**. That is an active, partially-completed migration
straddling v1/v2 concepts across `✳️any` and five real split subsets (`mesh`/`material`/`boundary`/
`load`/`analysis`) — not something a one-line catalog registration can resolve cleanly without
either re-claiming kinds `✳️mesh` already claims (→ `duplicate-mutation-owner`/`mutation-catalog-
unclaimed`) or guessing at a split this shard was not asked to perform (that is A8's/wave-2's
territory). For `mathematical`/`draw`/`note`/`sequence` the same v1-catalog registration is also
explicitly reserved for shard B1 (per this shard's own brief: "Do NOT edit `🧪️oracle/🔣️.json`
`mutationCatalogs` blocks for note/draw/mathematical/sequence/step") — confirmed B1 is actively
mid-edit on exactly these files already: the live `test contract` output right now shows `Unknown
mutation catalog @mutations-{draw,fem2d,fem3d,mathematical,note}-1-any` on each artifact's
artifact-level feature file, the same v1/v2 seam from a different angle.

**Left unregistered, all ten**, with this reasoning on record. Not a "could not be bothered" — a
"the honest disposition is documented-and-deferred to the shard(s) whose territory the actual fix
lives in, because it is entangled with their in-flight work and a partial catalog would trade this
breach for a worse one."

### The 33 pre-existing ones — re-verified, A9's disposition stands unchanged

Read `📓️a9-mutation-catalog-integrity.md` in full before touching anything, per the brief. Current
breach list matches A9's exactly: the same 3 gis editor-state owners
(`🏔️gisterrain/…/✏️editor/🎚️config`, `🗺️gismap/…/✏️editor/👥️presence`,
`🗺️gismap/…/✏️editor/🎚️config`) and the same 30 framework `os`/`replication` module fixture trees.

Re-verified the specific mechanism A9 found rather than re-deriving from scratch: `mutationCatalog
Problems` (`🟦️.ts:658`) computes `profiled = owner.includes(PROFILE_MARKER)` — any owner whose path
contains `/🏅️standards/` anywhere is forced into profiled mode, which then requires
`owner.endsWith('/🏅️standards/${std}/🪆️subsets/${subset}')`. The three gis owners' paths contain
`/🏅️standards/` (they are nested under an artifact's standards/subsets tree) but end in
`/✏️editor/🎚️config` or `/✏️editor/👥️presence`, past the subset root — an `endsWith` anchored at
the subset root can never match a path with trailing segments past it. Confirmed this line is
unchanged since A9's read (still at `🟦️.ts:658` with the identical `owner.includes(PROFILE_MARKER)`/
`endsWith` logic) — **no compliant v1 catalog can be registered at the walker-computed owner for
these three, full stop**, exactly as A9 found. The 30 framework ones remain real, populated,
non-empty Rust test-fixture trees with zero `.feature` files anywhere in their module trees (A9's
own count, spot-confirmed unchanged) — registering a catalog with no claiming feature would trade
this breach for `mutation-catalog-unclaimed`, which is strictly worse under the same honesty metric
this shard is measured on.

**Left unregistered, all 33**, per A9's already-thorough investigation — nothing has changed on
disk or in the framework rule that would let a fresh attempt succeed where A9's did not.

## Files touched

**None.** Zero production files edited, zero files deleted, zero cache/inventory files written. Every
finding above is read-only investigation plus two real, reproduced, captured build failures (kept
under `🗑️generated`, deleted at ticket close per house rules).

## Handoff for whoever resumes Breach 1

1. Confirm `semio-framework-plugin` compiles clean (`cd 🧰️framework/🛍️products/💻️os/🔨️modules/
   🔌️plugin/📦️packages/🦀️rust && cargo check --offline`, isolated `CARGO_TARGET_DIR`,
   `RUSTC_WRAPPER=""`) — or that the `🔌️plugin` call sites at
   `…/🦀️.rs:29878` and neighbours have been updated to the `ToValue`/`FromValue`-based decode path
   the manifest types now use.
2. Invoke `bun 🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📜️script.ts inventory --artifact <id>
   --standard <v> --subset <s>` directly (not through the root `📜️script.ts` — see the invocation
   gap above) for the 63 subsets this shard was assigned (list: `🗑️generated/breach-runtime-
   inventory-missing.json`, or re-derive via the python one-liner in the coordinator's brief).
3. For the 24 artifacts with no bridge yet, generate one `🏭️bridge/{Cargo.toml,📜️script.ts,🦀️.rs}`
   per artifact root (not per subset) using the design in Part 1: depend on the plugin crate plus
   `semio-framework-os-kernel` directly, call `<Enum as SemanticMutation<Snapshot>>::kinds()`,
   attribute each kind to its subset via the leaf's own sidecar `🔣️.json` `"owner"` field, translate
   `outcomeClasses` via the disclosed rule, and flag (do not silently absorb) every resulting
   outcome/variant mismatch.
4. `s.stdio.step@ap214/cc6` already has a bridge (pre-existing, not this shard's) — just needs (2)
   run once the build is healthy; it should be the fastest one to close.
