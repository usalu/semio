# Staged PDF Integration

Current native and browser builds own the shared source capture. The staged changes below have focused verification. Their integrated builds and runtime checks remain pending.

| Change | Candidate | Evidence |
| --- | --- | --- |
| Mixed sampled images and Type1 text | `mixed/document.rs`, `mixed/images.rs`, and `mixed/fixture.json` | [Mixed resources](📋️mixed-resources.md) |
| Raw CFF and selected text encoding | `text-font-edit/codec.rs`, `simple-cff-stage/cff.rs`, `simple-cff-stage/🧫️contracts.json`, and `simple-cff-stage/painter.rs` | [CFF outlines](📋️cff-outlines.md) |
| Font edit, text changes and insertion font selection | `text-font-edit/{metrics,content-ops,object-edits}.rs` | [Font reassignment](📋️font-reassignment.md) |
| Registered mixed resources and font action regressions | `text-font-edit/editor-tests.rs` includes the mixed test, with `text-font-edit/natural-file-fixture.json` | [Font reassignment](📋️font-reassignment.md) |
| Page boundary coordinate acknowledgment | `boundary-draft-narrow/overlay.json` records its ten source paths | [Boundary drafts](📐️page-boundary-draft-fix.md) |
| Product browser startup | Live product-owned Vite and entry, with the one-line manifest binding staged | [Mounted editor](📋️mounted-pdf-e2e.md) |

Paths in the candidate column are below the ticket’s generated directory. Staging scripts re-read current source and preserve independent edits when regenerating the candidate. The CFF contract must follow the simple CFF stage before the text-font stage copies the reader. The registered font test stage must follow mixed editor test staging. The two advanced browser cases transpile and await the component build. The PNG overlay passed focused type, behavior and combined closure checks. Its actual browser scenario awaits integration.

- [Mixed resource stage](📜️mixed-resource-stage.py)
- [Mixed registered test stage](📜️mixed-resource-editor-stage.py)
- [Raw CID CFF stage](📜️raw-cff-stage.py)
- [Raw name-keyed CFF stage](📜️simple-cff-stage.py)
- [CFF contract stage](📜️cff-contract-stage.py)
- [Font mutation stage](📜️font-reassignment-stage.py)
- [Registered font test stage](📜️font-reassignment-editor-stage.py)
- [Advanced browser smoke staging](📜️pdf-browser-advanced-stage.py)

The boundary draft tests also passed with the Rust normalizer in the WGPU event owner. The WASI components keep their shared Rust UI contract; the schema carries the named policy through its existing text field. The native retained widget selects the same Commit trigger. The React renderer and its shared TypeScript helper use the same binary64 normalization.

- [Narrow boundary overlay](📐️boundary-draft-narrow.py)
- [Digest-guarded combined overlay](📜️prepare-pdf-overlay.py)
- [PNG picker and mutation staging](image-authoring-stage.md)

## Integrated Source Release

The digest-checked overlay applied 22 files. The PNG picker, three Cargo edges and seven matching workspace locks landed next. The real PDF development contribution is now authored in the PDF manifest. The independent cold image and idempotent stream fixes landed before the integrated WASI capture. The current source includes each of these changes.

Live closure verification measured the document owner at six files and 782 active lines. The page editor had 22 files and 3,994 active test lines. The controller had 24 test files, 3,981 active lines and 259,135 active bytes. All seven PDF-bearing locks parse and agree with their normal manifest edges. The path check measured up to 189 UTF-16 units on the 22-path overlay.

- [Live image source and dependency verifier](image-authoring-stage.py)
- [Cold image and stream regressions](cold-dct-import.md)
- [Mounted component acceptance](📋️mounted-pdf-e2e.md)

## Follow-up Stages

The controlled JPEG overlay remains staged in four source owners. Its verified preimage and output identities can be prepared and released independently with the reviewed release script. The active production capture still refers to the original coordinated epoch. A separate advanced browser runner covers mixed-resource save and reopen, exact font reassignment, and the primary runner's cancellation case. The font verifier can now check the actual downloaded PDF directly.

- [Controlled JPEG release](controlled-dct-release.py)
- [Mixed and font browser staging](📜️pdf-browser-advanced-stage.py)
- [Font download oracle](📜️font-reassignment-oracle.py)
- [Warm multi-component retry](📋️component-retry.md)

## Reviewed Follow-up

The next source batch has 15 controlled font and JPEG paths and one PDF progress controller path. Their exact preimages and outputs are recorded by the reviewed release helpers. The first part has passed the combined graph, semantic admission, content and native writer checks. The progress body type-checked against the real schema-5, page, and document metadata emitted by the current WASI build. The focused probe retained the same operation-progress reexport warning already present in the Raster editor.

The terminal-copy candidate separately charges one inline prepared-file body and one snapshot body for the ownership transfers at publication. The receiving audit identifies this as logical accounting. Its exact below-limit and at-limit behavior under the original mounted grant still needs the governing test.

The active browser and native captures are kept on the existing source epoch. The follow-up checks are staged and do not substitute for mounted Open, edit, Save and reopen.

- [Combined font and JPEG release](controlled-font-release.py)
- [Embedded font evidence](controlled-embedded-font.md)
- [PNG progress and cancellation](📋️registered-png-progress.md)
- [Original receiving audit](📥️release-receiving-audit.md)
- [Terminal quote release](📜️pdf-terminal-copy-stage.py)

The emitted WASI layout gives the staged terminal quote 964 bytes, including 116 bytes for the prepared file and 848 for the snapshot. The actual guard compiled to reject 963 and admit 964 under positive item and depth authority. The normal 32 KiB copy grant fits. Issuance and final settlement remain for the mounted gate. [Terminal quote evidence](terminal-copy-quote.md)

## Reviewed Product Release

At 20:21 UTC on 2026-11-20, the combined guarded release applied 21 PDF paths. It includes the qualified original JPEG path, physical text codes and embedded OpenType/CFF font custody, the image operation progress and cancellation control, the terminal ownership-transfer quote, and the four bilingual mutation and snapshot schema inputs. A second SHA-256 check of every live postimage matched the reviewed roster. The longest affected absolute source path measured 189 UTF-16 units. The release adds no Cargo dependency edges.

Repository closure measurements passed on the macOS, Linux, Windows, and WASI target configurations. The largest line closure in the font and JPEG batch was 3,948 lines in the lift tests. The largest source file count was 32 inferences test files. The native Page editor's current linked test binary passed the independent JPEG render comparison before this controlled large-file extension landed.

The w8jCBpl browser invocation ended with no linked component. Along with unrelated stale BREP consumers, final Hub PDF assembly exposed a removed plugin builder entry point and an undeclared direct dependency used by generated app bindings. The product browser gate must capture the Hub fixes and the 21 released PDF postimages in its next actual component run. The two active native requests had reached source admission and framework dependencies when the PDF changes landed, before they announced a PDF compilation. Their emitted unit receipts will determine the source they actually exercised.

- [Closed-loop browser acceptance](📋️mounted-pdf-e2e.md)
- [Native registered acceptance](📋️native-editor-gate.md)
- [Controlled PDF batch receipt](🗑️generated/pdf-reviewed-postimages.json)
