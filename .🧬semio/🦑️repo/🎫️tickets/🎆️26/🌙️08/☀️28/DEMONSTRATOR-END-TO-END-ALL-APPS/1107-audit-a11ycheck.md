# 1107 audit: a11ycheck lane

Scope: staged a11y patch `1107-a11y/` (core.py, localize.py, check.py, reach.py) against the live tree. Read-only. Nothing was applied, no `--write`, no cargo, no scripts or tree edited, no modifying git commands (only `git status`, `git diff HEAD`).

## Script review (read before running)

- `core.py`: dry run is the default. `W="--write" in sys.argv`; files are written only when `W and not bad`. Stale-anchor check runs first. Apply gate in the docstring: `C/🗑️generated/1105b/hub-ok-1107`. That file does not exist yet, so apply must wait.
- `localize.py`: dry run is the default. Writes only when `--write` is passed (`WRITE` flag, inside the per-file loop).
- `check.py`, `reach.py`: read-only.

## Runs

| Script | Mode | Result |
|---|---|---|
| `core.py` | dry run | 82 edits (81 text anchors + 1 fn on `semantic-cells`), 44 files, **0 stale**. Every anchor occurs exactly its declared count. |
| `localize.py` | dry run | 44 files, 64 regex sites, **0 unmatched/blocked**, no UNMAPPED, no NEEDS-IMPORT. 2 FALLBACK-IMPORT hits in `🧪️tests/🔬️host-unit/📝️dsl-and-op-text-laws/🦀️.rs` (assumes `semio-ui-locale` is in scope; not verified against that crate's Cargo.toml). |
| `check.py` | baseline | **66 violations** (last run recorded 59; +7 new sites from the re-imports). Output is capped at 40 lines by the script. |
| `reach.py` | cycle check | `semio-framework-pack-json` does not reach `semio-ui-locale` (normal deps: 54 visited; with dev-deps: 141 visited). `semio-ui-locale` reaches `pack-json` only via dev-deps (`ui-locale > value-native-receiving-original > … > dsl-record > pack-json`). The new normal dep `ui-locale -> pack-json` is acyclic. |

In-memory simulation (no write): applying localize's substitutions to the tree leaves 0 plain-literal port labels and 0 plain breadcrumbs. The 2 non-literal port labels that `check.py` flags (`🏃️run/…/🦀️.rs` `label: spec_id.into()` and `🔁️workflow/…/🧪️tests/🔁️workflow/🦀️.rs` `label: id.into()`) are covered by core.py (edits for both). So after apply, `check.py` should reach 0 if both run cleanly.

## Stale or unmatched anchors

None stale. Anchors still match, but two anchored regions were rewritten by a peer in the same lines:

1. `🧰️framework/🔨️modules/🖱️ui/🌐️locale/🏷️label/♻️retirement/🦀️.rs` (core edit "cells: ManuallyDrop<[[…]]>" at line 8). Peer change (uncommitted, `M `): `#[doc(hidden)] pub struct LocalizedLabelRetirement`, `retirement` replaced by `retirement_seed`, `close_step` now takes a `RetainedAllocationEffect` and calls `record_retirement_work`. The anchor text survives, but the surrounding code is new. The patch's `LabelRow` change targets the same struct, so expect compile breakage here.
2. `🧰️framework/🔨️modules/🖱️ui/🌐️locale/🏷️label/🛬️receiving/🦀️.rs` (core edit `Self{cells:[[Cow::Owned(value.native.en),…` at line 11). Peer rewrote `mapping_demand`/`map_step` into `fn from_native(value: NativeLabelTerminologies) -> Self`. The anchor sits in the new `from_native` body. Same risk: the patch was written against the old mapping API.

Peer hunks in the 86-file set that do not intersect any anchor line (anchors still match): `🎒️pack/🔤️json/🛫️encode/🧭️tree/🦀️.rs` (core edit 61), `🖱️ui/🌐️locale/📦️packages/🦀️rust/Cargo.toml` (core edit 60; peer added three dev-deps, lines after the anchor), `🔁️workflow/…/🚪️io/🪶️sqlite/📸️snapshot/🦀️.rs` (core edits 243–257, peer hunk elsewhere), `🔌️plugin/🦀️.rs`, `🔌️plugin/🧪️testing/🦀️.rs`, `🔌️plugin/🎞️media/📤️snapshot/🦀️.rs` (localize target).

Tree churn during this audit: the first diff showed 6 tracked hunks in the 86-set, the second showed 8 status entries. Treat the counts as a snapshot; re-run the anchor check at apply time.

Not checkable by these scripts: the "28 files re-imported", "tool-machine split" and "store ports" moves. The anchor-to-hunk test covers only files that the patch touches.

## Default-language leak not handled

`workflow_node_for_app` (`🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🕸️graph/🧱️graph-model/🦀️.rs:88`) sets `label: app.label.resolve(Terminology::Native, Locale::En).to_string()`. It stores the English cell only. The patch does not touch this file (0 mentions in core.py, localize.py, check.py). `check.py` does not detect it either, so a green gate would not catch it. Needs a separate edit: keep the matrix (or resolve per active locale) in the `WorkflowNode.label`, plus its schema and fixture.

## Hygiene

- My first command redirected a dry-run to `/tmp/null` (deleted).
- A temporary `git status` snapshot was written into the ticket root and deleted in the same command.
- No file under `1107-a11y/` or the tree was modified.

## Verdict

Anchors: 82 core + 64 localize sites; 0 stale. Patch is not applied and not compiled. Blockers before apply: the hub gate file is missing; two anchored regions in the ui-locale label module were rewritten by a peer; `workflow_node_for_app` is unhandled.
