# IO Architecture Exploration

Read-only inspection on 2026-09-30. Read root AGENTS.md and ✏️s/AGENTS.md. No runtime tests were run and no code was changed by this exploration.

## Universal Snapshot Seam

The most complete dialect-aware seam is `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs`, `app::declarations::commit_artifact_declarations` around line 38924. It walks every artifact, standard and subset, with both `subset.dialect` and `subset.io.native.codec` in scope. It collects codecs and IO batches, registers codecs/formats under an assembly guard, releases that guard, and calls `io_mechanism::io_register` once per subset. Framework-supplied SQLite routes here can cover headless and editor artifacts without hand-editing each plugin.

The codec registry itself is lower and more universal for older declarations, but has insufficient dialect information: `🏪️store/🦀️.rs:10627` `ArtifactCodec` carries a schema string, DSL extension, schema hash and functions for compile DSL, print mirror and apply/replay mutations. It has no artifact/standard/subset coordinate. Registration around 11128–11158 accepts codecs and indexes them by schema. Thus deriving a dialect from schema is an assumption to avoid. `ArtifactCodec::of<P,Mutation>` around 10891 does have the concrete snapshot type and both ArtifactPack and ArtifactDsl bounds; it is the appropriate place for a typed erased snapshot encode/decode bridge if needed.

Older registration uses `🚪️io/🦀️.rs:1877` `ArtifactAssemblyRegistryPlan`, whose document codecs arrive through `DocumentCodecSpec` in plugin around 3600–3690. Older bare headless codecs and app codecs share that path. `commit_artifact_assembly_registry_plan` around 1916 preflights store, composer, validator and format registries then commits. New `io_mechanism` routes are currently not part of this plan. Adding universal handling at route resolution/run, based on all known native dialects, avoids incomplete coverage tied only to the new declaration tree.

## Framework IO Mechanism

`🧰️framework/🔨️modules/🚪️io/🦀️.rs:1993` owns the active newer mechanism. `IoEntry` around 2046 has static dialects, fidelity, optional sniff and bare fn-pointer run; the registry stores static entry pointers keyed by owned `(from,into)` dialects. `io_register` around 2142 uses a store assembly barrier, idempotently accepts identical entries and rejects conflicts. Routes around 2234 are deterministic, cycle-free, prefer fidelity then length then coordinates, and clamp to three hops. `io_run` around 2254 folds hops; `io_identify` around 2279 sniffs carrier-origin entries; `io_entries` around 2291 projects owned descriptors.

Typed constructors: `serializer_entry` at 2314 and `deserializer_entry` at 2355 decode/encode native binary through ArtifactPack. `_text` twins at 2332/2371 use ArtifactDsl for text-native snapshots. Every native snapshot can therefore be represented by its existing native payload without a lossy JSON conversion. A SQLite wrapper should preserve native payload type, native bytes/text and full dialect identity, reject importing into another dialect, and declare Exact fidelity.

Carrier dialects and wire types live in `🚪️io/🧬️schema/🦀️.rs` and mirrored JSON, proto, GraphQL and TypeScript schemas. TypeScript `🟦️.ts` only mirrors dialect/entry/route parsing; there is no TypeScript artifact conversion dispatcher implementation in this module. Browser bundle host/materialization exposes guest `io-run`. A new cross-platform codec should live under framework IO and operate on bytes without runtime filesystem or SQLite-library dependencies.

## Whole-Document UI Path

Every editor receives framework-owned Export/Import Document actions from `🛂️manifest/🦀️.rs:3053` and TypeScript constants around 1346. Existing transfer is a recursive native document archive, separate from the IO route mechanism.

React shell `📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🟦️.tsx:7970` intercepts export action. At 10505 `exportDocumentArchive` reads the focused plugin's `readAppDocumentArchive`, encodes with `encodeDocumentArchiveBytes`, then downloads. Import at 10521 opens a picker, decodes bytes, creates a fresh instance, then calls `loadAppDocumentArchive` with cancellation and progress before focusing it. Filename/media constants are in `🛠️ShellHelpers/🟦️.tsx:3075`: `.semio-archive`, `application/vnd.semio.document-archive`.

Native renderer matching transfer is in `🌉️ProgramBridge/🎯️targets/🧊️wgpu/🦀️.rs:836–847` and `🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs` around 10941 and 21807. Whole-document archive bytes are encoded/decoded in `📡️spr/🧵️channel/🦀️.rs:3542/3605` and mirrored in `💻️os/🟦️.ts:2905/2913`. Those archive bytes have a version byte and recursive closure with parent pack/spr and member identity/history; they are not SQLite. Changing snapshot IO should avoid accidentally treating whole-document archive as only one head snapshot.

## Coverage and Tests

The repo has 51 files named artifact-definition.json under plugins; this is only one inventory subset and does not prove total artifact count. A textual scan finds 132 Rust files with old document_codec declarations, including tests. Exact artifact coverage should be proven against registered dialects/declaration subsets rather than a hardcoded artifact list.

Existing IO law suite is `🚪️io/🧪️tests/🔬️io-mechanism-laws/🦀️.rs`; native constructor conformance, determinism, cycle and hop bounds are tested there. Plugin declaration fixture tests are included at plugin around 39001 from `🧪️tests/🔬️app-declarations-fixture/🦀️.rs`; existing assertions around 9413 inspect live IO routes and around 9436 prove failed declarations leave IO unchanged. Existing carrier contract tests in sequence also enumerate routes.

UI IO matrix fixture and Playwright harness live in `💻️os/🔨️modules/🧑‍💻dev/🧫️fixtures/🚪️io-matrix.json` and `🧪️tests/🚪️io-matrix/🟦️.ts`. The fixture says every editor exercises Export/Import Document and per-artifact rows add foreign formats. `exportDocumentArchive` test helper at 483 and roundtrip at 495 exercise the current archive behavior.

Useful independent SQLite oracle is builtin `bun:sqlite` behind test-only code (already used extensively in repo) or Python sqlite3. An interoperability law should open output as a real SQLite database, query expected schema/native payload, generate matching SQLite with the independent engine and import it, and cover overflow pages, empty text/binary, non-ASCII dialect/text, malformed tables/payload and dialect mismatch. Ensure generated outputs remain under ticket 🗑️generated and delete after verification.
