# 🗺️ PDF Artifact Editor: Reconciled Feature Inventory and Coverage Matrix (round 10, 2026-11-27)

## Product and scope

- **Product under test**: the PDF 1.7 editor `s.stdio.pdf@1.7/*#editor`, the PDF artifact editor with a dev contribution and playground row `stdio-pdf-1-7` (react port 6260, `✏️s/🧑‍💻dev/🎭️variants/📘️stdio-pdf`), run through the real shell, plugin runtime, wasm guest and local backbone persistence (`serve-pdf-native.sh`).
- **Same engine, different controllers**: the subset editors `a`, `e`, `h`, `ua`, `vt`, `x` expand the same kit macro (`🧰️kit/🧾️pdf17`) with 112 of the 115 actions. They carry no action delta (`🗺️ui-surfaces-static.md` §5). Only the image picker branch differs, which is a defect (A-IMG-3). The 1.4 editors (43 actions) and the viewers (8 actions) are separate apps and are out of this matrix.
- **Not current product requirements**: no context menus, toolbars or menu bars are rendered (live census `🗑️generated/ui-inventory/ui-inventory.json`: zero `menu`, `menuitem` and `toolbar` roles). Presence (multi-user overlays) needs a second participant. There is no documentation or feature flag for the editor (static audit).

## Sources reconciled

| Source | Where | Notes |
|---|---|---|
| Descriptor (115 actions, 5 window kinds, 5 keybindings, 1 dialog, History tab) | `🌎️hub/…/📘️pdf/🧩️component/🔣️.json` | dumped to `🗑️generated/inv/editor17.json` |
| Live UI census (rail per window, tabs, headings, Object pane, Details pane) | `probe-ui-inventory.ts` | adds the 10 page-window view actions and the 6 Details actions that the controller roster does not list |
| Static code audit (window kinds, inspector, authoring, roster-only verbs) | `🗺️ui-surfaces-static.md` | |
| Argument semantics per action | `🗺️action-args-a.md`, `🗺️action-args-b.md` | |
| Rust tests per action | same two files, "Rust test" columns | |
| Previous journey (42 checks) | `pdf-pane-journey.ts`, `🌐️pdf-browser-journey.md` | re-run green as baseline `🗑️generated/r10-baseline` (42 of 42) |

## Coverage matrix

Status: ✅ verified E2E (UI → visible → save → third-party oracle → reopen), 🔧 fixed this round, ❌ failing, ⏳ in progress, ➖ out of scope (with reason).
Owners: V = views/history worker (`sweep-views-history.ts`), A = content and page (`sweep-content.ts`), B = document and transfer (`sweep-document.ts`), C = annotations, forms and resources (`sweep-annotations.ts`), J = baseline journey (`pdf-pane-journey.ts`).

### Files and lifecycle
| ID | Feature | Entry | Owner | Status | Evidence |
|---|---|---|---|---|---|
| F-1 | New blank document (one Letter page) | app start | J | ✅ | r10-baseline en/de |
| F-2 | Save File (native PDF, compact) | Transfer › Save File | J | ✅ | r10-baseline `*-edited.pdf` + oracle |
| F-3 | Open File (real PDF) | Transfer › Open File… | J | ✅ | r10-baseline open-en/de |
| F-4 | Incremental Save of an opened PDF (original bytes kept) | Save File | J | ✅ | r10-baseline open-* |
| F-5 | Reopen saved files | Open File… | J | ✅ | r10-baseline |
| F-6 | Export Document / Import Document (archive) | Transfer | J, B | ✅ / ⏳ failure path | r10-baseline transfer-* |
| F-7 | Export PDF File (`exportMediaFile`) | ? | B | ⏳ | |
| F-8 | Open corrupt / non-PDF / encrypted / large PDF | Open File… | B | ⏳ | |
| F-9 | Repo fixtures indirect-length, repair-stream | Open → Save | B | ⏳ | |

### Content and page actions (A)
insert-text, insert-rectangle, insert-line, set-text, move, resize, delete, set-fill, set-stroke, set-font, set-font-size, set-image, insert-image (PNG picker), replace-image, import-local-image (host callback), insert-page, remove-page, move-page, set-page-size, set-page-rotation, set-page-box, set-page-user-unit, set-page-label, set-page-transition, set-page-extra, replace-page, replace-page-pick, merge-document, merge-pick, extract-pages, set-page (agent page-text command, not in the rail: verified through its native law only if not reachable), copy, cut, paste, clearSelection, selectAll, setSelectionMode, setInteractionGranularity, canvas selection, Object-pane object fields. Baseline-verified already: insert-text, insert-page, insert-rectangle, set-page-rotation, move-page (keyboard on the thumbnail tree). Detail: `🧭️sweep-content.md`.

### Document-level actions (B)
set-info, set-info-field, set-language, set-page-layout, set-page-mode, set-outline, set-mark-info, set-metadata, set-viewer-preferences, set-open-action, set-document-id, set-named-destination, remove-named-destination, set-embedded-file, remove-embedded-file, set-encryption, set-output-intent, set-catalog-entry, set-trailer-entry, set-indirect-object, set-entry, set-extra-entry, set-optional-content, Object-pane document fields and entry rows, Text-layer title and language. Baseline-verified already: set-info-field title through the Object pane. Detail: `🧭️sweep-document.md`.

### Annotations, forms and resources (C)
create-annotation (7 kinds, rail and Object-pane buttons), set-annotation, set-annotation-style, set-annotation-border, set-annotation-markup, set-annotation-kind, set-annotation-appearance, set-form-field, fill-form-field, set-field-data, set-form-settings, set-form-content, set-resource-detail, set-graphics-state, set-pattern, set-color-space, set-properties, set-font-metrics, set-font-reference, set-image-mask, set-glyph, set-mesh-reference. Detail: `🧭️sweep-annotations.md`.

### Views, history, Details, shell (V)
zoom-in, zoom-out, fit-page, fit-width, actual-size, go-to-page, set-query, toggle-match-case, next-hit, previous-hit; undo and redo (rail and mod+z / mod+shift+z); commitCheckpoint, createAlternative, switchAlternative, checkoutCheckpoint, revertToCommand, historyEdit* and the finalizeHistoryEdit dialog, History panel; Details setSnapshotValue, insertSnapshotValue, removeSnapshotValue, moveSnapshotValue, renameSnapshotKey, replaceSnapshotSource and inline buttons; Editor/Viewer toggle; Command palette; Tasks and cancelTypedOperation. Baseline-verified already: Details setSnapshotValue (title and unknown-field refusal), a11y of headings and thumbnails. Detail: `🧭️sweep-views-history.md`.

### Cross-cutting
| ID | Feature | Owner | Status |
|---|---|---|---|
| X-1 | Localized UI en and de | all | ✅ baseline, ⏳ sweeps |
| X-2 | Localized and announced refusal notices | all | ✅ baseline (range, unknown-field), ⏳ per action |
| X-3 | Accessibility: headings, tree, keyboard moves | J | ✅ |
| X-4 | Edit → undo → redo → save → reopen sequences | all | ⏳ |
| X-5 | Third-party render and fidelity of every saved or exported PDF | all | ⏳ |
| X-6 | No console errors beyond known noise | all | ✅ baseline |

## Known static defects (to verify and fix)
Listed in `🗺️action-args-a.md` (N1–N20) and `🗺️action-args-b.md` (Notes 1–10). They are assigned to the owning sweep.
