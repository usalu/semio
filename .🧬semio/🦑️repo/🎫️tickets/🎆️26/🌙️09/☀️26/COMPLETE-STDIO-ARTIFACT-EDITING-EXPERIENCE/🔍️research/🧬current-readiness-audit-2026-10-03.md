# Current Stdio Editor Readiness Audit — 2026-10-03

> **Root correction after integration inspection:** the shipping section below inspects only the base plugin. Current `🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions` contains nine family packages; the Office package registers DOCX, PPTX and XLSX editor/viewers and activation events. Therefore “no DOCX user-facing route” is not established by this audit. Family descriptors, publication, component linking and browser activation require validation; see the correction at the end.

## Scope and method

This is a read-only source and retained-log audit of the active ticket. It
revalidates `🧬retained-paged-mounted-audit-2026-09-28.md` and
`🧬bounded-typed-snapshot-copy-design.md` against the current checkout. No
Cargo command, test, runtime session, or Git operation was started by this
audit. Consequently, all current-state conclusions below are source-backed,
not a claim that the code compiles or works at runtime.

The Native 9 log is a historical compiler result. It is not a validation of
the moved current sources.

## Facts changed since the September reports

| September premise | Current verified state | Evidence |
| --- | --- | --- |
| Store owned `🧬️retained-clone`. | The reusable ownership model moved to the neutral Value module. Store now owns a `snapshot-clone` preparation adapter. | `🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust/🦀️.rs:11-17`; `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs:13-16`; `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:24-28` |
| Paged retention was unmounted. | `PagedList` is mounted beneath Value's retained-clone module and has current unit-law sources. Its mount alone does not connect it to a Stdio artifact route. | `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs:13-16`; `🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🦀️.rs:62-122` |
| The DOCX owner limit was 2 KiB / 128 item units. | Store's one-item maximum is now 1,048,576 bytes. DOCX reserves 4,096 turn bytes, yielding 1,044,480 owner bytes and 65,280 measured item units. The preparation remains one whole-snapshot clone and mutation turn. | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:15957`; `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs:11-17,269-325` |
| Native 9 had not obtained a compiler result. | Native 9 ran to compilation and exited 101 with two source errors in the old Store path. Those sources have since moved, so the result cannot validate the current architecture. | `🗑️generated/retained-clone-kernel-native-9.log:120,130-149,154-180` |

## Current ownership architecture

### The generic route is present, but is internal Store infrastructure

Value exposes retained-clone traits, source admission, progress admission, and
retirement admission (`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/🦀️.rs:21-48,125-145,167-211`).
Store's `RetainedClonePreparationFactory` begins a source cursor, copies it in
advance ticks, applies the bounded edit, seals it, and closes each stage on
cancellation or failure (`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:43-98,251-335,350-445`).

Both the Store module mount and the factory are `pub(crate)`
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🦀️.rs:24-25`; `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️snapshot-clone/🦀️.rs:43-57`). A Stdio
plugin cannot name this factory directly across the crate boundary. A real
artifact integration therefore needs a Store-owned public assembly seam or a
Store builder that mounts the generic factory internally; merely deriving a
Value trait in DOCX does not expose an editor route.

Store has source tests for the generic lifecycle and contiguous text rejection,
but they were not executed for this audit
(`🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧪️tests/🔬️unit/🦀️.rs:3467-3657`).

### Paged cursor abandonment remains an ownership-integration blocker

`PagedListCursor` owns state through `ManuallyDrop`. Its `Drop` implementation
asserts that the cursor is terminal (except during unwinding), and only then
drops the state (`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🦀️.rs:62-76,104-110`).
The current direct-abandonment law partially copies a 64-probe source, catches
the resulting panic, and verifies zero payload destructors
(`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:401-422`).

This is a material improvement over an unchecked drop: ordinary direct loss
does not synchronously destroy partially retained payload. It is not recovery.
The state remains in `ManuallyDrop`, and no inspected Store or Stdio route
transfers a nonterminal cursor into a driver-owned close queue. The next
integration must make that transfer explicit before a cursor can leave the
preparation driver. The existing source tests for normal close, cancellation,
and over-budget paths show the intended close protocol but are not test results
from this audit (`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:254-399`).

## DOCX and OPC have not moved to a paged owner

`DocxSnapshot` still directly owns `OpcPackage` and
`Vec<DocxXmlPart>` (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/🧬️schema/📸️snapshot/🦀️.rs:126-187`).
`OpcPackage` still owns a `Vec<OpcPart>`, an `OpcContentTypes` value, a
`BTreeMap<String, Vec<OpcRelationship>>`, and a string comment
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️.rs:455-473`); content-type
defaults and overrides are also a `Vec`
(`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🎒️zip/📦️opc/🦀️.rs:182-186`).

No DOCX base source inspected imports or implements `RetainedClone` or
`PagedList`. The DOCX editor's preflight measures the entire OPC collections
and XML vector (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs:105-137`), then phase zero performs
`base.clone()`, mutates the clone, measures its result and inverse, and reports
a single 4 KiB turn allowance (`✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📜️docx/🏅️standards/🔖️ecma-376/🪆️subsets/🧱️base/✏️editor/📬️preparation/🦀️.rs:269-325`). The current
limit rejects oversized documents, but it does not make XML or OPC copying
paged.

**Concrete owner blocker:** migrate the document/OPC collection ownership to a
retained paged representation, implement the Value retained-clone contract for
the document snapshot, and mount it through the Store integration seam above.
Until then, the bounded generic preparation cannot replace DOCX's whole clone
and in-place mutation.

## Shipped Stdio editor coverage is narrower than its semantic catalog

The deployed Hub Stdio plugin's `StdioApps` enum and registration sequence only
provide HTML, Markdown, CSV, TSV, plain text, JSON, and XML editor/viewer
variants (`🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/🦀️.rs:11-68`). It contains no
DOCX, XLSX, PPTX, ZIP, or media app. This is a user-facing production-route
blocker independent of the number of schemas or codecs present in the source
tree.

The catalog separately builds semantic rows from assemblies and codecs
(`🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇catalog/🦀️.rs:358-430`), and its
comment records a complete 36-assembly slice
(`🌎️hub/🧩️compositions/🗄️stdio/🔌️plugin/📇catalog/🦀️.rs:537-540`). That catalog
coverage does not instantiate an editor and must not be reported as 36 shipped
editors.

**Concrete shipping blocker:** after the DOCX bounded ownership route exists,
add its editor/viewer assembly to the Hub Stdio runtime app selection,
registration, and activation path, then validate the deployed composition. The
present plugin cannot expose a DOCX editing experience.

## Native 9 provenance and current relevance

The retained log shows that Native 9 reached `semio-framework-os-kernel`
compilation, then failed with status 101. The first error was a missing JSON
fixture included from the former Store paged-list test. The second was `E0618`:
a local `grant` binding shadowed the `grant` helper before it was called
(`🗑️generated/retained-clone-kernel-native-9.log:120,130-149,154-180`).

The old Store `🧬️retained-clone` source path is no longer the current owner.
The moved Value paged-list test now includes its fixture through
`../../🧫️fixtures/📦️copy/🔣️.json`
(`🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/📋️paged-list/🧪️tests/🔬️unit/🦀️.rs:233-252`).
Therefore Native 9 establishes neither a current compile failure nor a current
pass. A new validation must target the Value/Store/Hub paths after the source
blockers are addressed; this audit intentionally did not launch it.

## Handoff order

1. Define the Store-owned handoff for every nonterminal `PagedListCursor`, so
   cancellation and driver loss close retained state rather than relying on the
   intentional direct-drop panic.
2. Replace DOCX/OPC's whole `Vec`/`BTreeMap` snapshot ownership with the paged
   retained route, implement bounded cloning/editing, and mount it through a
   Store-accessible artifact assembly.
3. Register and activate the resulting DOCX editor/viewer in the Hub Stdio
   plugin. Catalog presence alone is insufficient.
4. Run fresh, targeted Value, Store, DOCX, and Hub Stdio validation from the
   current source tree. Do not reuse Native 9's old Store-path result.

XLSX and real media-export implementation work are outside this audit's owner
scope. No conclusion about their runtime correctness is implied here.

## Root Integration Correction: Family Composition Routes

`🌎️hub/🧩️compositions/🗄️stdio/🧩️extensions/💼️office/🦀️.rs` explicitly assembles eighteen editor/viewer apps across DOCX, PPTX and XLSX, with host artifacts, actions, capabilities and kind activation events. Sibling families are binary, image, media, mesh, semio, BIM, CAD and PDF. `.vscode/launch.json` registers family-named Office launch configurations. The base `StdioApps` roster alone cannot establish full shipped coverage.

The current `editor-component-check` in `🌎️hub/🧩️compositions/🗄️stdio/📦️packages/🦀️rust/📜️script.ts` now discovers and links each family package, transpiles it and checks the WebAssembly core. It accepts one absolute output directory; the September `--full-catalog` option no longer applies. Root started current family check 19 through its registered Nx target, output `🗑️generated/current-family-components`, log `🗑️generated/stdio-family-components-current-19.log`. This is a running validation, not proof of successful linking or browser reachability.
