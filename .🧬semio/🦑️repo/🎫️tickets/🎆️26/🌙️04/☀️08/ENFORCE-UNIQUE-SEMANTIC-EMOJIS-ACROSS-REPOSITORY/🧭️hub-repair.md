# Hub Hand-Reviewed Emoji Repair

## Declared Admin HTML Entry

The shared Vite owner is removing unwanted output aliases `index.html` and `404.html`. Hub had a real non-Vite dependency on `index.html`: its `/admin` handler and SPA fallback. Those two exact references and their current documentation now name the declared `🌐️.html` entry. No alias file was removed in this lane and no alternate basename was retained.

A language-neutral `adminPageRoutes` boundary fixture and actual production Axum router test cover `/admin`, the trailing-slash root, nested SPA fallback, encoded emoji HTML and module paths, hostile traversal, HTML/JavaScript content types and bytes, absent alias files, and an unbuilt directory's existing 503 behavior. The focused source-contract check failed on the old `index.html` fallback, then passed both exact declared coordinates after the patch. Native session 48096 was originally queued as a red run but had not executed before the source patch, so it is not an observed native red. Its actual final outcome is a pass: one selected production-router test in 0.364 seconds with 36 tests filtered. The real HTTP fixture explicitly runs without either alias file. The styling owner independently reports its actual Vite build/preview alias-removal test passed.

## Public Extension Route Cutover

The OS owner declared the new public root `/🧩️extension-modules`; Hub's two extension listing/asset routes and their current documentation were updated to that exact root, without retaining `/extensions` aliases. A new language-neutral boundary fixture covers encoded listing, an encoded emoji-named module asset, both old routes, traversal, and an unknown extension. Its native test creates real installation metadata, starts the actual production Axum router, and sends raw HTTP over TCP.

Native red session 52707 reproduced the precise runtime defect after compiling dependencies: the percent-encoded root `/%F0%9F%A7%A9%EF%B8%8Fextension-modules` returned 404 rather than 200 when registered as literal Unicode. Axum 0.8.9's installed primary source matches `parts.uri.path()` without percent-decoding static route segments. The two route registrations now use the exact encoded ASCII path; public documentation retains the human-readable root. Native retry 43439 completed with exit 1 before tests because Stdio's main TXT mount was read during the coordinated `🦅️txt` to `🔤️txt` cutover. After that batch became coherent, retry 96650 passed the one selected production-router test in 0.218 seconds, with 36 tests filtered. Exact listing and emoji-module response bodies, JavaScript MIME type, old-route rejection, unknown extension and traversal rejection were exercised over raw HTTP. The initial attempted Nx `-E` filter lost shell quoting and ran no tests; the plain exact test-name filter worked. This is a targeted route pass, not a full Hub native-suite pass.

Status: 32 individual handpicked moves complete; Rust verification remains incomplete. No Git mutation or automatic emoji selection is used.

## Scope and Decisions

Every source-controlled hub directory and file is being inspected. Tool-owned `node_modules`, macOS `.DS_Store`, generated `📤️dist`, and runtime `.semio` data are not renamed. Fixed package/compiler configuration names remain literal. Unique format-marker leaves (`🦀️.rs`, `🟦️.ts`, `🔣️.json`) keep their existing schema-first meaning; no format-wide churn is introduced.

Handpicked repairs:

| Existing node | Handpicked node | Meaning |
| --- | --- | --- |
| `🔐️local-bootstrap` | `🚀️local-bootstrap` | Starts and provisions the local hub; distinct from authentication. |
| `🗂️chunk-cas` | `🧱️chunk-cas` | Immutable content-addressed chunks are the storage building blocks. |
| `🗂️trusted-catalog` | `🔏️trusted-catalog` | Authenticated, trusted package catalog. |
| `🗂️📇️native-openable-provider` | `📇️native-openable-provider` | Provider of native-openable catalog records; removes stacked prefix. |
| Directory `🧪️fixtures` beside `🧪️tests` | `🧫️fixtures` | Input samples are distinct from executable tests. |
| `🔴️ConnectionsPage` | `🔗️ConnectionsPage` | Live client connections, not an arbitrary color. |
| `🧬️admin-live-journey-v1` | `🚶️admin-live-journey-v1` | End-to-end administrator journey. |
| `🧬️admin-intent-v1` | `🎯️admin-intent-v1` | Administrative intent contract. |
| `🧬️public-space-detail-v1` | `🏛️public-space-detail-v1` | Public space details. |
| `🧬️authority-adapter` | `🔌️authority-adapter` | Authority integration adapter. |
| `🧬️canonical-authority` | `🏛️canonical-authority` | Canonical authority contract. |
| `🧬️artifact-chunk-cas` | `🧱️artifact-chunk-cas` | Chunk-storage fixture contract. |
| `🧬️pipe-v1` fixture | `🚇️pipe-v1` | Local bootstrap pipe protocol. |
| `🧬️idle-admission-v1` | `⏳️idle-admission-v1` | Admission following idle time. |
| `🧬️canonical-pair` | `🪢️canonical-pair` | Canonical pack/SPR pair. |
| `🧬️lag-rebootstrap` | `🛟️lag-rebootstrap` | Recovery after replication lag. |
| `🧬️capability-v1` | `🔑️capability-v1` | Authentication capabilities. |
| Native provider fixture `🧬️v1` | `🪪️v1` | Validated provider identity contract. |
| `🧬️two-package` | `👥️two-package` | Two distinct packages sharing a trusted catalog. |
| `🧬️hub-boundaries` | `🚧️hub-boundaries` | Hub protocol and isolation boundaries. |
| `🧬️gis-inference-job-v1` | `🗺️gis-inference-job-v1` | Geographic inference job. |
| `🔣️share-token-vectors.json` | `🔑️share-token-vectors.json` | Share capability vectors. |
| `🔣️artifact-checkpoint-projection.json` | `📸️artifact-checkpoint-projection.json` | Checkpoint projection vectors. |
| `🧪️index.test.ts` | `🤝️index.test.ts` | Hub collaboration integration test. |
| `🧪️admin.test.tsx` | `🛡️admin.test.tsx` | Administrative UI test. |
| `📦️bin.rs` | `🚀️bin.rs` | Executable server entry point. |
| `📦️index.tsx` | `🚪️index.tsx` | Browser application entry point. |
| `🔣️entry-graph.json` | `🚪️entry-graph.json` | Entry dependency graph fixture. |
| `🔣️entry-graph.schema.json` | `📐️entry-graph.schema.json` | Entry graph structural contract. |
| `🔣️stylesheet-graph.json` | `🎨️stylesheet-graph.json` | Stylesheet dependency graph fixture. |
| `🔣️stylesheet-graph.schema.json` | `🧵️stylesheet-graph.schema.json` | Stylesheet wiring contract. |

## Retained Names Reviewed

- Hub roots: `📇️directory` denotes identity/tenancy records, `🔨️modules` implementation modules, `🔐️auth` authentication, `🗿️artifact-authority` artifact publication, `💡️inference` inferred results, `🛰️lag-rebootstrap` remote replication recovery, `🧪️fixtures` test input contracts, and `📦️packages` language packaging. All are distinct siblings.
- Database implementations: `🐘️postgres`, `🪶️sqlite`, and `🌐️neo4j` identify PostgreSQL, lightweight SQLite, and the graph backend. `🧪️tests` remains the executable test/data-vector group, distinct from handpicked `🧫️fixtures`.
- Administrator modules and elements: `🛡️admin`, `🧱️elements`, `🛡️AdminApp`, `📃️DocumentsPage`, `📰️EventsPage`, `📚️I18n`, `🏠️OverviewPage`, `🔑️AdminSession`, `🙋️UsersPage`, and `🏛️SpacesPage` describe their respective app, document, event, localization, overview, session, user, and space surfaces. Connections now uses its reviewed link symbol.
- Language packaging: `🦀️rust`, `🟦️typescript`, single `🦀️.rs`, `🟦️.ts`, and `🟦️.tsx` leaves retain the repository's schema-first format identity. `📜️script.ts` denotes the mandated executable router, `📋️project.json` the Nx project descriptor, `⚙️vite.config.ts` the Vite configuration, `🌐️.html` browser markup, and `🎨️.css` styles. There are no repeated format-marker leaf emojis among siblings.
- Contracts: `🧬️schema`, `🧬️.schema.json`, and `🔣️.json` distinguish schema structures from JSON vectors. `🧪️oracle` is the independent validator. Bootstrap contracts retain `🩺️readiness-v1`, `🚇️pipe-v1`, and `📨️credential-envelope-v1`; their diagnostic, transport, and credential-envelope purposes are distinct.
- The empty `📇️directory/🧪️tests/🧬️public-space-detail-v1` directory was retained and renamed to `🏛️public-space-detail-v1`; no unexplained input directory was deleted.

## Verification

- Complete source-tree audit: 147 governed files/directories, zero missing emojis, zero multiple-emoji basenames, zero duplicate sibling emojis.
- Follow-up audit after concurrent inference additions: 159 governed files/directories, still zero missing, stacked, generic-policy, or duplicate-sibling findings. The new inference branch uses `📇️catalog` for verified catalog facts, `🧾️wal` for write-ahead evidence, `🪶️sqlite` for its backend, `🧬️schema` for language-neutral contracts, and `🧪️tests` for executable tests. Distinct singleton format leaves remain unchanged. This count excludes only the same previously documented tool-owned/runtime outputs and literal reserved configurations.
- Source-reference audit: all 81 directly resolvable relative imports, stylesheet/HTML dependencies, Rust module paths, and Rust embedded-file references exist.
- All 42 hub JSON files parse, including fixed package and TypeScript configuration files (38 governed JSON leaves plus four reserved configurations).
- `bun nx run os-hub-ts:test --skip-nx-cache`: 11 tests passed, one explicitly gated end-to-end test skipped. Rerun after the final five fixture moves passed again.
- `bun nx run os-hub-admin:test --skip-nx-cache -- long`: 18 tests passed across two files. Entry graph oracle validated four laws and the stylesheet graph oracle validated five laws. Initial cross-tree fixture import failure was handed to the framework owner and repaired there before this rerun.
- `bun nx run os-hub:gis-inference-ledger-oracle --skip-nx-cache`: passed nine lifecycle traces, eight hostile inputs, six independent hashes, geographic bounds, and Ajv/TypeScript schema checks at the handpicked fixture path. This is a fixture/oracle check, not an executor, route, or approval claim.
- `bun nx run os-hub:test --skip-nx-cache -- quick --lib`: both attempts exhausted the existing 1,200,000 ms nextest-list budget while compiling dependencies in an isolated target directory. The warmed retry reached the large stdio plugin dependency and emitted compiler warnings, but no source/compiler error preceded the timeout. No Rust tests executed and no Rust pass is claimed.

## Targeted Corruption Repairs

The existing readiness schema was malformed: four nested alternatives each lacked one closing brace. The first TypeScript test run reproduced the parse failure; four precise brace additions restored the JSON without changing the contract. The protocol test imported frame codecs from the OS facade after that facade stopped exporting them; its import now points directly at the existing replication implementation. Both failures are covered by the passing TypeScript suite.

## Coordinated Shared References

The parent agent owns shared registry and normalizer changes: the `🔗️ConnectionsPage` member name, accepting the distinct `🧫️fixtures` name, the root interactivity audit's `🚀️bin.rs` path, and the three changed package-entry paths in the remaining-package-purity authority fixture. No broad search/replace was run. Historical ticket reports and plans remain historical evidence rather than being rewritten wholesale.

The OS owner subsequently renamed its backbone worker to `🧵️backbone-worker.ts`. The hub browser smoke-check URL now points at that exact existing file; only the single literal was changed.

Further exact incoming OS references were updated: browser-document-open fixture `🌐️browser-document-open-v1.json`, its `🧬️browser-document-open-v1.schema.json`, open-plan fixture `🧭️document-open-plan-v1.json` in the router and five Rust includes, and the MCP launcher `🌉️mcp/🚀️bin.rs`. Every target was checked to exist. The parent subsequently updated administrator stylesheet references after the UI owner's `🧵️.css` move.

The later inference fixture census found `🪪️inference-server-identity-v1` colliding with sibling `🪪️inference-catalog-selection-v1`. The server identity fixture is now `🖥️inference-server-identity-v1`; its package script and both Rust includes use that exact coordinate. A focused `🌎️hub/🧪️fixtures` audit covers 16 files and 10 directories with every violation count zero.

The isolated native attempt with `SEMIO_BUILD_BUDGET_MS=3600000`, session 17624, and `🗑️generated/hub/rust-tests-settled.txt` reached Hub itself, then failed on a private `AuthSessionRecord` import. The catalog consumer now imports the existing public declaration directly from `directory::model::AuthSessionRecord`; no new re-export or compatibility facade was added. A warmed retry is pending. Earlier intermediate retries encountered in-progress shared kernel/renderer renames and were superseded after those owners repaired their references. No Rust test pass is claimed.

The settled native retry (session 33673) compiled successfully and executed 23 of 79 tests: 22 passed, one aborted with stack overflow in `trusted_catalog::tests::all_trust_failures_precede_activation_and_have_bounded_diagnostics`, and fail-fast left 56 unrun. A diagnostic retry with `RUST_MIN_STACK=16777216` is running in session 93388, using the same isolated ticket target directory. The ordinary-stack suite is not passing; no runtime behavior was changed to hide this failure.

The larger-stack diagnostic (session 93388) stopped before tests because concurrently moved styling generated Rust still had an old package entry and OS kernel reported an unavailable `semio_framework_schema` crate. The styling owner subsequently confirmed its exact entry repair. The OS manifest/source dependency is being inspected separately; this attempt says nothing about whether a larger stack resolves the observed test abort.

The later Hub census found `📇️directory/🧪️fixtures` colliding with sibling executable `🧪️tests`. The input collection is now the semantic `🧫️fixtures`, while `🧪️tests` remains unchanged. Its one exact consumer in the Hub Rust package script now points at `🧫️fixtures/📣️ordered-append-broadcast-v1`. The focused directory audit covers 19 files and 15 directories with every finding count zero, and no old coordinate remains.

The next settled native retry (session 36215, 16 MiB thread stack) compiled and ran 31 of the now 80 tests: 30 passed, one PostgreSQL test failed because the Docker daemon socket is unavailable, and fail-fast left 49 unrun. The previous OS schema compile failure no longer reproduces; no dependency was added. A final diagnostic run excludes the external PostgreSQL fixture group explicitly and disables fail-fast, so it can assess the remaining native tests without claiming the unavailable integration tests pass.

The final diagnostic (session 12463, 16 MiB thread stack, `--no-fail-fast`, explicit exclusion `not test(directory::postgres::tests::)`) executed all 76 selected tests: 73 passed, three failed, four PostgreSQL integration tests were skipped because Docker is unavailable. The previously stack-overflowing trust test passed under the larger diagnostic stack; this does not make the default-stack suite green.

The three remaining failures are not stale emoji references:

- `artifact_checkpoint_publication_is_atomic_bounded_idempotent_and_replayable`: the negative specimen appends `-altered` to a manifest locator, then its `ownership_plan` helper calls strict decoding with `expect`, panicking before the intended conflict assertion. The locator encoder/decoder prefix and byte format agree.
- `inference_wal_chain_rejects_crc_valid_tampering_and_exact_cross_segment_tip_mismatch`: an inference WAL test unwraps `Unavailable("db I/O task capacity exhausted")`.
- `inference_wal_proof_executes_literal_committed_transaction_scope_and_cancellation_traces`: the same I/O-capacity error at its database initialization.

These runtime/test-setup issues were reported to the parent without altering production behavior, broadening queue limits, weakening assertions, or rewriting concurrent agents' new inference work. The Hub native suite is not claimed fully passing.

## Changes Preserved

All moves preserve the current file bytes. Reference changes are exact reviewed paths in current files, never historical whole-file restoration.
