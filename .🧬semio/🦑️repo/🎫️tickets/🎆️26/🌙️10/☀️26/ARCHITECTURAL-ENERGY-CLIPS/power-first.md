# Leistung Before Arbeit

Request: explain Leistung before Arbeit/Energie in the Physikalische Grundlagen clip.

The existing ticket is already open. The repo MCP resource reads returned an empty index, and ticket_reopen resolved its path under the Codex installation directory instead of the workspace. The repo CLI confirmed the actual ticket is open; work continues within that ticket through the repo interface.

Plan: add a portable expected sequence regression, establish Leistung independently at the opening, follow with Arbeit and kWh, and keep the canonical manifest, scene classes, section composition, narration and recap in the same order.

## Verification

- Red: the new portable sequence test failed on the old energy-first manifest.
- Green: the sequence test passed with the real Manim scene classes and NumPy array oracle.
- Canonical clip contract module: 31 tests, 6 existing opt-in integration skips, no failures (11.332 s).
- git diff --check passed for the changed physics source, curriculum, test contract and concepts.
- Existing narration directory contains only its authoring script, so no stale recorded audio assets require reassignment.
- A real render caught an unsupported math spacing command in the new unit introduction; it was corrected to the existing supported command and rendering was restarted.
- A temporary project-local Python environment created by the first test invocation was removed after the test exited. All subsequent commands use the existing shared Python environment and ticket-local output.

## Changed Files

- Canonical curriculum manifest: physics segment order and scene names.
- Physics scene source: Leistung first, then Arbeit/Energie; independent W/kW opening, adjusted transitions, matching recap.
- Physics section composition: same order and numbered audio associations.
- Shared intro source: the physics topic description names Leistung before Arbeit.
- Concepts guide: matching lesson order and correct distinction between power and diagram area.
- Portable clip cases and Python regression: order across manifest, scene roster, section composition, narration and recap, validated with existing Manim and NumPy.
- This report, power-first-render.md and power-first-check.py: retained verification record and reproduction input.

Concurrent edits in the shared checkout were preserved. Other sessions already have output below the same ticket generated directory; cleanup will remove only this follow-up own power-first subtree.

The complete Leistung scene rendered successfully (356 frames at 640×360, nominal 2 fps; 135.08 s verification time). Its measured trace orders rate < unit < area. Sampled frames of the P/W opening and W/kW unit definition were visually inspected: readable captions, no overlap or clipping.

The work scene exposed a live section-composite updater changing the Manim family during FadeIn. Both architectural cut figures in that scene have static opaque contours, so their unused updaters are cleared before fading; animated windows keep their explicit animations. Verification was restarted on the corrected scene. A separate process rendered and decoded the recap successfully (167 frames, 43.56 s). Concurrent ticket cleanup removed some verification media after creation; retained frame counts and inspected-image evidence are preserved here.

Final clip-contract regression after the intro text and static-figure correction: 31 tests, 6 existing opt-in skips, zero failures (17.048 s). Diff whitespace validation passed again. The full curriculum movie is not regenerated or published by this source-edit task; the three affected beats are verified with temporary real renders.

## Completion

Leistung now comes first in the canonical curriculum, the source class order and BEATS roster, the standalone composition and numbered speech generation, the intro wording, the recap and the concepts guide. The first explanation introduces P, W and kW before the power-time area and the following dedicated Arbeit/Energie beat. Existing third-party Manim and NumPy validate the portable regression; no runtime dependency was added.

All three affected beats have complete temporary render/decode evidence: Leistung 356 frames, Arbeit/Energie 374 frames, recap 167 frames, all 640×360 at nominal 2 fps. See power-first-render.md for timings, sampled-image review and limits. The complete work scene includes the final architectural/window sequences. Verification outputs belonging to this follow-up were removed after inspection; other sessions output under the shared ticket is preserved. Reproduction input power-first-check.py is retained and takes one scene name per invocation, matching the production renderer process boundary.

The repo MCP is rooted at the Codex binary directory instead of this workspace. Resource reads and ticket operations were attempted through MCP; the repo CLI is the workspace-correct fallback for ticket lifecycle.
