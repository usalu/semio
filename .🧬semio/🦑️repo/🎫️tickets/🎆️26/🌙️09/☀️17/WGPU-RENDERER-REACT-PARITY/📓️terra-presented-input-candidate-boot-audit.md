# Presented Input Candidate Boot Audit

## Evidence

The WGPU19 browser receipt contains only one renderer diagnostic:

- `🗑️generated/astra-runtime/checkpoint19-iab/wgpu-console.json`, 2026-09-21T15:59:48.279Z: `worker-frame-failed: presented input candidate generation exhausted`.

That text was not a reliable cause in the source snapshot that produced the receipt. The Chrome phase at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧊️renderer/🦀️.rs:15855` formerly converted every error returned by `ShellState::seal_presented_input_candidate` into this one text. The current source preserves the exact returned fault.

A real epoch exhaustion is not a credible ready-stage explanation. `ShellState::new` initializes `next_presented_input_epoch` to `Some(1)` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🐚️Shell/🎯️targets/🧊️wgpu/🦀️.rs:6131`; the only later write is `checked_add` in `seal_presented_input_candidate` at line 13002. All browser boot paths construct `ShellState::new`.

The seal can therefore fail for either of these UI-engine conditions:

1. A previous UI window still has `sealed_input_candidate`; `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1113-1114` returns false.
2. A visible, already presented UI window lacks a ready candidate; lines 1116-1117 return false.

## Confirmed abandoned-witness path

A successful Shell seal publishes one witness in Shell and in every eligible UI window. Before the repair, cancellation before the presenter could close and drop the retained frame owner without calling Shell discard. The next Chrome walk then hit UI condition 1, which the old renderer text mislabeled as epoch exhaustion.

The ownership boundary is now explicit:

- `FrameTransaction::discard_presented_input_candidate` at renderer lines 14197-14209 returns witnesses held by its build or after-Chrome carrier.
- `AppFramePreparation::discard_presented_input_candidate` at lines 14399-14401 returns the witness during preparation cancellation.
- `ActiveFrameBuild::retire_cancelled_phase` at `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/🧵️frame-job/🦀️.rs:230-239` performs this return before stepping the carrier close ladder.

This ordering matters. `FrameBuildCursor` and `AppFrameAfterChrome` can move a candidate into a nested preparation only while their close step is running. The active frame cancellation path returns the outer candidate before it invokes that close step, so recursion into the nested preparation is not required for the production path.

Existing native fail-first coverage is in `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-frame-job-unit/🦀️.rs:186-243`:

- superseded build carrier;
- cancelled preparation carrier;
- stale completed presentation carrier.

The required behavioral assertion is stronger than checking internal emptiness: seal witness N, cancel exactly one carrier, drain it terminally, then complete a new Chrome walk whose witness N+1 seals and acknowledges. That proves UI windows no longer retain N and the accepted input registry remains the last accepted registry until N+1 acknowledgement.

## Candidate-ready gate

An ordinary partially reconciled visible document cannot cause a completed Chrome walk to reach UI condition 2. `ShellState::render_main_window_step` returns false while `render_ui_document_step` is pending and its bounded opportunity count remains below `SHELL_WINDOW_PAINT_OPPORTUNITIES`; see Shell lines 23724-23739. It registers the body as visible only after document completion, a terminal document fault, or exhaustion at lines 23760-23767.

On normal completion, the UI reconcile path has already completed its candidate baseline, reconcile and interaction rebase before painting. `Ui::step_document_reconcile` establishes `candidate_ready` before it answers complete at `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🦀️.rs:1673-1722`. A clean presented document is rebuilt through that same candidate baseline on the next frame, so it is not a no-render exemption.

Condition 2 remains reachable only when Shell deliberately records a terminal document paint fault or exhaustion yet registers the window body for visible/accessibility ownership. That is a distinct document-paint failure and should surface its document fault, rather than be treated as acceptable bounded work. The WGPU19 console receipt has no UI-document fault or exhaustion line, so it does not establish this branch as the observed cause.

## Conclusion

The source proves the prior generic error text concealed the actual seal branch. The abandoned-witness path was concrete and has a narrow return-before-close repair and native carrier laws. The candidate-ready branch is a valid separate diagnostic only after a document terminal fault or exhaustion; it is not a normal busy/partial-render condition. The current receipt is insufficient to attribute the specific WGPU19 instance to one branch; preserve the exact Shell fault in the next browser capture to make that attribution.

No build or test was run for this audit.
