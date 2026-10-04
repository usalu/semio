# DAG Owned Parent SQLite Staging

The actual DAG parent has exactly two persisted fields: `schema: String` and `content: ArtifactChild<SemioGraphSnapshot>`. Its root marker is `dag.dag` and the authored child dialect is `s.stdio.semio@v1/graph`. The plugin AGENTS domain describes a left-to-right directed acyclic graph; the current concrete snapshot composes that graph through its typed child. Rich node, edge, port, preview, scalar, camera and other working-scene state are not extra persisted parent fields.

## Handwritten contract

Artifact root: `✏️s/🔌️plugins/🕸️dag/🗿️artifacts/🕸️dag`. Under the actual Snapshot owner at `🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot`, the independently handwritten `🪶️sqlite/🗄️.sql` has two tables. `dag_document` stores the root marker. `dag_content_child` stores the document foreign key and five literal child/target/dialect strings. SQL IDs are structural surrogates. The local child alias and target artifact ID remain independent, unresolved, empty, reserved-delimiter, NUL and Unicode strings. Only the existing authored child dialect is a semantic constraint.

`🧫️fixtures/🪶️sqlite/🔣️.json` specifies three complete native cases, both table widths/counts, and wrong-dialect negatives. There are exactly two semantic rows per snapshot. No JSON/native byte carrier, semantic identity inference, global materialization lookup, or child-entity flattening is part of this parent schema.

## Actual native seam audit

The current handwritten Native Text/Pack codec lowers the attached local scene into a framework-owned graph snapshot, then recreates a content-addressed child on decode. This is an inspected literal-address fidelity gap, not a native runtime result yet. The attached scene is explicitly ephemeral in the owner code. Parent export must preserve the existing child address rather than require or persist that local materialization. The actual declared foreign JSON leaves already project the parent's typed fields; their literal-address roundtrip baseline is staged independently.

The concrete Snapshot has a generated controlled ToValue path and a manually authored ordinary FromValue implementation. Its controlled FromValue constructor is not yet supplied. Its SQLite provider/capability and strict native input/output hooks are absent. The Native test fixture guard retires the actual composed child using the canonical owned retirement factory and terminal-empty cursor. It does not invent a DagSnapshot retirement trait or use a generic no-op lifecycle fallback.

## Baselines and execution

Nine Native laws are mounted in `🧪️tests/🪶️sqlite/🦀️.rs`: bare capability; actual public `any::io::io()` declaration; ordinary Binary and Text literal-child fidelity; independence from an attached empty local scene; genuine controlled Value construction; actual declared JSON leaf fidelity; independent Bun SQLite interpretation of both tables; actual erased Binary/Text parent SQL with all five child columns. Rustfmt parsed the file. No Cargo/native run was started by this worker, and the Native provider remains unmounted pending an individual real assertion baseline in the coordinator's sole lane.

The registered uncached Source baseline genuinely executed **six laws: five passed and one failed**, in a 10.4s Nx invocation. The one authentic feature failure is absent actual Snapshot SQLite projection/reconstruction exports. Three complete literal-child cases, wrong-dialect validation and independently interpreted two-table SQLite passed. This proves the existing canonical Source Snapshot already preserves the actual parent fields, while its provider is absent. Log: `🗑️generated/dag-snapshot-source-baseline-first.log`. The main coordinator subsequently assigned this worker the Source provider and public package after this authentic missing-export RED. The canonical Source persisted model is reused unchanged.

## Owned routes

`📦️packages/🦀️rust/📜️script.ts` preserves the existing document contract command and adds the canonical shared snapshot runner with the explicit Source contract file. `📋️project.json` registers `@semio-tech/dag-dag-rs:test-snapshot-sqlite`, `test-snapshot-sqlite-native` and `test-snapshot-sqlite-source`. The actual package has no assembly feature gate to request. Both launch catalogs contain these commands at 408.743–408.745, preserving all concurrent entries and existing finite quick-level budgets.

Next Native baseline: `SEMIO_TEST_LEVEL=quick bun nx run @semio-tech/dag-dag-rs:test-snapshot-sqlite-native --skip-nx-cache`. It is staged in the coordinator’s subsequent eighteen-owner queue; its own runtime baseline has not yet been reported. Compilation, capability failure, native field fidelity and eventual repair must be reported separately; no universal completion claim is made.

## Files

- Snapshot handwritten SQL, language-neutral fixture, Native baseline and independent Source contract suite.
- Snapshot root: cfg(test) module mount only.
- Owning Rust package script/project registration.
- Both seed/current launch catalogs: narrow DAG commands only.
- This report and the generated registered-run log.

## Source Semantic Owner and Public Package

Mounted the explicitly handwritten two-table Source projection/reconstruction beside the SQL contract and exported it from the actual canonical Snapshot and new artifact root facade. Both persisted fields and all five literal child components survive; positive SQL row IDs are independent structural surrogates, including independently renumbered document 9/child 11. The provider rejects unowned rows, broken parent relationships, wrong dialects, malformed scalar storage and extra local scene fields. Exact row/value/schema/table/column frontiers and known interior Unicode projection/reconstruction cancellation are covered. The initial implementation registered suite passed 13/13 laws, 71 assertions (119 ms assertions, 8.0 s uncached Nx).

The artifact previously had no actual TypeScript public package; its plugin’s existing TypeScript script only printed a trace and established no public owner proof. Authored the artifact-owned TypeScript package using the existing canonical runner, with build/check/test and explicit full standards consumer verification. Both launch catalogs register test/build/consumer checks at 408.746–408.748. The first public gate built 13 outputs but stopped at four test-only expected-value TypeScript literal-marker typing errors. Corrected only assertion generic typing, preserving every assertion and the canonical literal schema. The fresh registered public gate passed all 13 laws/71 assertions (271 ms assertions, 13.2 s Nx), six actual public exports and independent strict declaration consumer. Log: `🗑️generated/dag-snapshot-public-current.log`.

The whole owning Source consumer check then found a concrete existing inference parser gap: the authored depth map schema requires nonnegative integer values, while its parser returned unchecked unknown map values. This is a canonical consumer prerequisite, not a SQLite semantic failure. Added neutral valid maps (including reserved and Unicode keys), five malformed depth scalar cases and independent Ajv/schema laws before the narrow repair. Runtime RED and final whole-consumer verification are pending. Native provider, native address repair and controlled Native hooks remain unmounted awaiting the actual individual Native baseline.

## Final Source Consumer Verification

The inference negative law genuinely failed with unchecked depth `-1` (14 pass/1 fail, 77 assertions, 280 ms/17.6 s Nx). The narrow constructor now checks every authored map value as a nonnegative safe integer while preserving all literal keys through `Object.fromEntries`. Independent Ajv/schema tests cover reserved/Unicode keys, empty maps and all five invalid values. Fresh public package gate is **15/15 passed, 85 assertions**, 331 ms/7.6 s uncached Nx, with 13 outputs and six exports. Fresh whole owning Source check passed all **34 consumers**, 3.9 s uncached Nx. Logs: `dag-snapshot-public-final.log`, `dag-snapshot-consumers-final.log`. The Native two-domain-table oracle expects exactly two; the erased-file law expects three because central I/O attaches its required metadata table. Native owner assertion evidence remains pending; no capability or literal-address Native repair is mounted.

## Direct Codec Table-Count Correction

The preceding explanation of three erased tables was mistaken. These laws call the erased codec thunk directly; it returns the two domain tables. Central I/O attaches metadata outside that thunk. The independently executed Block5d direct-codec result confirmed this boundary (actual fifteen vs mistaken sixteen). Corrected DAG’s staged direct-codec expectation to two, preserving all literal field assertions; no DAG Native assertion has executed yet.

## Executed Native Failure and Literal Owner Repair

Fresh valid corpus genuinely executed nine laws, two passed/seven failed, Nextest `9a376d1f-a724-4c3b-9a11-c4a3fe9bd88d`, `root-parent-valid-corpus-native-baselines.log` lines 30534–30704. Three failures were missing actual capability/declaration/erased capability; three exposed literal child address replacement through the old graph-lowering Binary/Text codec (including an attached scene); one reached the strict missing controlled Value constructor.

Mounted the manually authored two-table Native provider, exact singleton/PK/parent/dialect validation, shared borrowed Projection/Reconstruction, explicit schema/row admission, generated literal controlled Native input/output, and actual Pack capability. The canonical Snapshot now owns a literal DslRecord with schema/content fields. Handwritten borrowed controlled FromValue guards and actual child retirement preserve partial ownership. Native Binary/Text now prints and parses that exact record, without materializing or deriving child identities.

The example's parent document is a literal child address. Its full graph initialization is a separately authored child-graph asset parsed through the actual framework Graph owner and attached explicitly by the demo constructor. The existing independent graph scene oracle remains, alongside parent address equality and an assertion that decoded literal parents do not acquire a local scene. Grammar/protocol names now identify dag.dag. Rustfmt parsed the mounted provider and codecs; fresh Native execution remains pending with Root. No whole-owner Native green is claimed.

The mounted selected DAG suite now has twelve laws: the original nine plus exact domain row admission/four actual copy phases, explicit malformed semantic graph cases, and independent Bun SQLite integrity/FK/count/edited literal Child reconstruction. The direct thunk expects two authored domain tables; metadata belongs to the central I/O boundary. Fresh Root execution remains pending.

## Current Native Route Readback

The actual Rust Nx project is `@semio-tech/dag-dag-rs`; `@semio-tech/trinity-dag-rs` is not this owner. Its existing `📦️packages/🦀️rust/📋️project.json` registers `test-snapshot-sqlite-native`, and its existing `📜️script.ts` delegates through `snapshotSqliteTests`. Both `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` already register combined, Native, and Source routes at 408.743–408.745. Correct next sole coordinator invocation is `bun nx run @semio-tech/dag-dag-rs:test-snapshot-sqlite-native --skip-nx-cache` with the existing quick level. No alias, duplicate command, or launch patch was necessary. The current parent queue's omission resulted from selecting the other project name; this readback establishes registration, not Native runtime success.
