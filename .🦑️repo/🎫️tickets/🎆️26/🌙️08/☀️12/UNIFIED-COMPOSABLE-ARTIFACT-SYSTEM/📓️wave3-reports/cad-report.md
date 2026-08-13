# W3 — `cad` composes stdio model/drawing

**ucas-status: complete (code); final green verification pending on stdio, see caveat**

Written by the orchestrator from on-disk evidence after the authoring agent was terminated by a session limit mid-verification.

## What changed

`✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🦀️component.rs` (413 lines):

- **Deleted one of the repo's four independent B-Rep topology models.** `CadEdge`, `CadWire`, `CadFace`, `CadShell`, `CadSolid`, `CadGeometry` — confirmed zero remaining references anywhere in the file (grep count: 0).
- **Replaced with typed composed children**:
  - `pub type CadModelChild = store::ArtifactChild<SemioModelSnapshot>` (`:43`)
  - `pub type CadDrawingChild = store::ArtifactChild<SemioDrawingSnapshot>` (`:49`)
  - Constructed via `store::ArtifactChild::new(...)` (`:354`).
- Retained plugin-specific state (view/projection UI, not duplicated content): `CadPaneId`, `CadCamera`, `CadProjectionDsl`, `CadReference`.
- `🎪️demonstrator`'s registration of cad's `3d.cad` kind was deliberately left in place per the earlier ruling — not relocated, not touched.

## Verification

- `cargo check -p semio-s-plugin-cad --all-targets`: **0 errors** at the agent's last successful check (before termination mid-final-verification).
- **Caveat — not independently re-verified end-to-end**, for the same reason as lowpoly: stdio itself is currently red from ticket #2553's live, in-flight `⚙️engine` deletion fan-out (spreading across png/pptx/xlsx/docx as of this writing — confirmed via `git log` to be commit 501, landed *after* this plugin's last clean check). The cad-specific code is stable; the blocker is entirely upstream and unrelated to this migration.

## sharedFileRequests

None.

## Concurrent-churn observations

cad's engine already consumed `SemioMeshSnapshot`/`SemioBrepSnapshot` before this migration (per the design doc's note that its conversion path partly pre-existed) — that path was reused rather than rewritten, consistent with the brief. No collisions observed with DKM's `✳️brep`/`✳️drawing`/`✳️mesh` mutation-vocabulary work; this plugin only consumes those subsets, never edits them.
