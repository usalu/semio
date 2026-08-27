# Flow Compiler and Grant Repair

## Scope

Flow owns this source packet. The coordinator exclusively owns native compilation and task runners. The original native diagnostic contains 212 errors. This packet preserves the admitted 16,384-byte domain and production one-item, 4,096-byte grants.

## Compiler Surface

Added the missing owned framework and job dependencies, corrected plugin app type imports, and removed decorative asynchronous declarations from production synchronous trait implementations and helpers. Actual boxed composition futures and asynchronous tests remain asynchronous. Source whitespace validation passed; native compilation remains coordinator-owned and is not claimed here.

## Grant Findings

The original direct-work close path requires a 16,384-byte grant even for a tiny retained preview vector. Its reported release bytes are the envelope rather than actual released bytes. The original generic Store preparation clones and transforms an entire root, builds full inverses, and seals the complete edit in one advance while reporting one byte. Cancellation drops whole owned structures. These are concrete frontier violations, not classification issues.

The existing Store authority serializes the entire canonical edit to derive its digest. A separate executor is implementing a Store-owned incremental canonical JSON sealer with private authority; Flow will supply typed traversal and concrete preparation recipes rather than app-generated digests or accumulated credit. Nested Flow widget retirement also requires owned dictionary traversal, which the coordinator authorized in the neural module.

## Verification Status

- Compiler-ready source checkpoint sent to the coordinator.
- The coordinator subsequently granted the exclusive native compiler lease. Canonical Bun/Nx Flow checks are now running; no rustfmt or workspace-wide compile has been run.
- Grant-frontier tests and runtime cursor implementation are in progress.

## Source Implementation Checkpoint

The direct-work close path now transfers preview vectors, individual mutations, widget dictionaries, nested neural trees, GUI maps, and text into a linked owned retirement cursor. Each step removes at most one structural owner or truncates at most the supplied byte grant. String allocation ownership is moved, not cloned. UTF-8 strings are retired as raw byte buffers so one-byte grants can make progress. Restore rejects nonempty owners instead of dropping retained vectors. The neural dictionary now exposes borrowed ordered iteration and owned single-entry extraction; the ownership test checks that nested string allocation addresses remain unchanged.

Live Config preparation now has separate base admission, post-copy, targeted inverse-copy, actor-copy, edit construction, and Store-sealing phases. Each copied byte is validated incrementally as UTF-8; conversion does not rescan the completed string. Config canonical traversal borrows all 15 mutation variants directly and uses fixed field ordinals, with no JSON value staging. The Store owns canonical escaping, digest calculation, authority, and cancellation. The previous full-root snapshot inverse is replaced by the exact affected field except for an actual Snapshot mutation. Preparation and close account for actual traversed/released payload bytes rather than the allowed envelope. The fixed edit-ID construction atom is below 64 bytes; full preparation tests use grants of 64 and 4,096, while copying and retirement fixtures also cover 1 and 3 bytes.

Two typed proof catalogs replace the sentinel factory catalog: 15 DirectStore routes and six HostEffect routes, with concrete factory-type witnesses and matching execution contracts. Publication-lane declarations are unchanged.

The canonical native check is now registered through the Flow package script, Nx project, and launch configuration. A coordinator-authorized `.nxignore` exclusion prevents ticket-local generated fixtures from becoming duplicate Nx projects. `nx show projects` returned 183 projects, includes the real Flow project, and excludes transaction-generator-fixture; no fixtures were removed.

## Current Evidence

- `🎯️grant-frontier.json` and its strict schema define six byte-frontier cases, including actual 16 KiB semantic text, four-byte UTF-8 scalars, and escaped control characters.
- Bun/Ajv source fixture validation: six cases and seven hostile rejections pass. TextEncoder and Node Buffer agree on byte counts; JSON encode/decode agrees on semantic text.
- Rust tests authored: direct production close, nonempty restore rejection, nested dictionary ownership, all fixture byte retirement, Config copy/targeted inverse, canonical serde parity for all mutation variants, and real Store publication/cancel/retry/terminal-empty loops at 64/4,096 bytes. These are not yet claimed passing.
- Native r2 was stopped after discovering the plain target path was cold. Native r3 reached upstream stdio and reported eight errors, then current-source inspection found the exact TIFF/JPG/PNG corrections already made by a peer. Native r4/r5 were blocked by transient Nx fixture project collisions; `.nxignore` and the successful project-list evidence address that blocker. Native r6 is compiling using the warm `🧱️cargo-target-cad` directory.

## Explicitly Unfinished Boundaries

The artifact Store recipe still clones/transforms the complete bounded scene and creates its content handle monolithically. The BatchOnly duplicate lifecycle's CancelDuplicateWidget Config mutation retains its original arbitrary-JSON parser branch; a faithful incremental parser is still required before that branch can be certified. Direct command preparation still contains larger string/vector work in preview and widget scanning paths. None of these boundaries was hidden by shrinking the 16 KiB input domain, relabeling a live route, or claiming source fixture checks prove runtime behavior.

## Remaining Work

Complete native compiler iteration, byte-bounded preparation and retirement, schema-first cancellation/replay fixtures, third-party oracle parity, and coordinator-executed Rust runtime tests. Until those finish, Flow is not certified native-clear or grant-correct.

## Authored Slider Label Checkpoint

The framework `Widget::InputSlider`, GUI `NodeChrome::Slider`, and incoming widget descriptor now require an explicit string label. There is no serde default or English display fallback. The authored value flows into DAG `name` and abbreviation, the Flow child node's typed label, GUI chrome, neural round trips, and generated Playbook blocks. Unlabelled raw neural numbers remain neuron widgets instead of inventing a slider title. Catalogue drags initialize the new content from the selected catalogue name; direct command creation explicitly initializes empty content.

All inspected exact Rust constructors and the framework/Procedural2d/Procedural3d DSL encoders/decoders were updated. Mounted binary float field ordinals now follow `id, label, value, min, max, step`; the label occupies the retained second string slot. Mutation string copying and digest observation include it. Four Knob JSON fixtures and nine authored DSL example files were hand-updated, including the exact existing Column Height / Profile Radius / Side Count labels. Value-edit reconstruction preserves the existing label.

The app-private scene copier now includes label bytes, as does recursive Widget/NodeChrome retirement. It owns immutable Arc-rooted projections and native dictionary/map/set iterators, not ordinal `nth` or successor-key rescans. New Rust tests cover 16 KiB actual label content at fixture grants, exact copied/released bytes, and cancellation of a nested large-key map across worker transfer with a pinned allocation witness. These tests are authored, not yet executed; the scene copier is not yet connected to the live Artifact Store recipe.

Validation: corrected canonical Nx source target passed six grant fixtures / ten hostile cases plus five label fixtures / three hostile cases, using strict Ajv and JSON/UTF-8 byte oracles. `git diff --check` passed for the Flow, procedural, framework Flow, and renderer scopes. `🧪️flow-slider-label-source-2026-08-27.txt` is the passing log. The earlier file named `🧪️flow-slider-label-red-2026-08-27.txt` failed from a package-relative source lookup, not the label assertion; it is not functional RED evidence. Native JSON→DAG/chrome and child round-trip regression tests are authored but await the coordinator's compiler queue.

Native r6 completed upstream stdio RED (two already concurrently corrected JPG collection visibility diagnostics); it did not reach Flow. Compiler lease was returned to the coordinator. No Flow native-clear or runtime success claim is made here.

## Canonical Scene Identity and Borrowed Visitor

The coordinator approved replacing the unspecified DefaultHasher identity with portable repository-owned streaming SHA-256. The exact format is `flow-content-sha256-` followed by lowercase SHA-256 of UTF-8 `semio.flow.scene.sha256.v1`, one NUL byte, then the exact canonical typed scene JSON. Field order is widgets, synapses, layout; variant and record fields follow the typed serde contract. Number spelling and escaping are pinned by literal canonical JSON fixtures, including floating-point `.0`. Camera remains parent content; the local scene owner is never wire identity.

Both the ordinary helper and the retained `SceneHash` use that projection. The ordinary helper remains explicitly non-interactive and may clone its borrowed inputs. The retained cursor delegates byte encoding to Store's generic frozen reader, absorbs the domain and JSON under actual grants, and transfers the exact prepared scene Arc into the child handle without a second scene clone. This cursor is not yet wired into live Artifact publication.

The new borrowed Flow visitor covers all ten mutation variants, all nine Widget variants, every NodeChrome variant, neural trees/dictionaries, GUI maps, preview bindings, layouts and synapses. Native map iterators replace ordinal scans. Strict schema-first fixtures cover those shapes; Rust tests compare exact borrowed bytes to serde, not just parsed equality.

Five content-identity fixtures cover independent label, nested Dictionary, layout and synapse changes. The Node crypto oracle pinned five distinct SHA-256 values and verifies chunking at 1, 64 and 4,096 bytes. The initial all-zero fixture RED and corrected source GREEN logs are retained as `🧪️flow-content-identity-red-2026-08-27.txt` and `🧪️flow-content-identity-source-2026-08-27.txt`. Rust tests comparing ordinary and retained streams, exact byte totals, Arc identity adoption, and terminal-empty cleanup are authored but unrun.

The remaining ordered-map comparison hole and the approved shared primitive design are recorded separately in `📓️flow-ordered-collection-seam-2026-08-27.md`. Live Artifact recipes remain unfinished until that seam is implemented and adopted; the five routes have not been relabelled or declared grant-correct.

## Parameter Intent and Shared Collection Checkpoint

The new schema-first `set-graph-parameter` leaf accepts only nonempty `widgetId`, a finite numeric `value`, and optional transport-only `surfaceId`. It rejects `documentJson`, `fixtureJson`, arbitrary operations, nonnumeric values, and unknown fields. No widget-ID length cap was introduced: the language-neutral fixture includes an actual 8,192-byte Unicode identifier. The typed payload and two native contract tests are mounted in the Flow crate. This leaf is not yet registered as a live command or factory route; retained selected-widget copying and Artifact Store publication must be completed first, so the current renderer's new action still has an explicit Flow integration gap.

The existing host expands slider min/max/step when an input falls outside its range. That behavior was communicated to the framework Flow owner for a shared fixed-size Widget helper; a retained command must preserve it along with the authored label, not silently adopt a clamp-only or value-only alternative.

Canonical Flow source validation passed again, adding four parameter-intent cases and nine hostile rejections to the previous grant, label, canonical-shape, and SHA-256 cases. The independent parameter encoding oracle is existing third-party `fast-json-stable-stringify`. Log: `🧪️flow-parameter-intent-source-2026-08-27.txt`. Old mutation-fixture prose now correctly describes the new domain-separated SHA-256 identity instead of DefaultHasher.

The shared ordered-map primitive now includes strict explicit-owner drop guards, one-byte retained lookup, exact last-owner payload handoff, fixed AVL-height/rank-iteration metadata bounds, cold-only serde/convenience documentation, and eleven authored native laws. Its own canonical source target passed three operation fixtures, two long-key lookups, and eight hostile rejections. Native validation and adoption remain coordinator-gated. The authoritative launch seed was updated and registry regeneration passed; both seed and generated launch configuration retain all four Flow/Ordered validation entries.

## Delete-Cascade Inverse Ordering

A new semantic fixture exposes an existing inverse error independently of grant accounting. Starting with edges `e0,e1,e2,e3,e4`, deleting a widget severs `e1` and `e3`. Re-inserting at original indices 3 then 1 gives `e0,e1,e2,e4,e3`, not the original order. Inverse operations replay in their stored order, so reconnecting must use ascending original indices 1 then 3 after the widget and its layout are restored.

Both the schema inverse and the app Store preparation recipe now iterate severed edges in ascending source order. The schema helper also no longer allocates an intermediate index vector. The schema-first fixture includes an actual 4,800-byte authored slider label, three surviving edges, and two nonadjacent severed edges. Existing third-party Immer generates an independent inverse patch stream; the language-neutral planned inverse matches its exact restored scene, while the old descending plan is explicitly shown to differ. Strict validation rejects three hostile fixture changes.

The canonical Flow source target passed (`🧪️flow-delete-cascade-source-2026-08-27.txt`). A native test now checks both ordinary schema inverse and preparation inverse, exact restored edge order, and full semantic scene equality including the large label. That native test is authored but unrun; this semantic correction does not certify the still-monolithic Artifact Store recipe.
