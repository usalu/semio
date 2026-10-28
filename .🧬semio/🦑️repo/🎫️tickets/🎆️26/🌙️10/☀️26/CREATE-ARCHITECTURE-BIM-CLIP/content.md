# Six Architecture BIM Videos

## Completed Content

Six standalone videos each own one original topic: data and information; object orientation; representations; partial models and use-case translations; interfaces and exchange; consistency. The six domain entries delegate to the shared curriculum renderer. The original clip-level scene sources remain canonical so every video uses one common wall fixture and one native visual interface.

English and German narration contain 49 clauses each, with explicit language selection and no fallback. All clauses were tightened to no more than 115 characters before exhaustive native subtitle validation. The standards audit in `content-audit.md` covers schema/instance distinctions, application methods versus declarative IFC, chosen analytical schemas, mesh semantics and identity versus synchronization.

The wall is 6 m × 3 m × 0.30 m (18 m², 5.4 m³). A pure thickness event changes it to 0.40 m (7.2 m³) without changing its teaching identifier. Layers scale proportionally in this example. Representations derive from the same geometry and visibly regenerate after the event. The external analytical link deliberately remains stale, producing separate source-revision and thickness consistency issues.

The object-oriented chapter distinguishes inheritance from contained material layers. The models chapter independently contrasts multiple models using the same vocabulary with an explicit translation to a chosen FEM schema: one wall maps to four shell elements, with nodes, assumed material behavior, supports and recorded losses. The exchange chapter explains standards/interfaces and geometric mesh transfer. The consistency chapter updates the linked revision only after regeneration/review.

## Verification

The test agent reported schema/Ajv validation and all semantic tests green before the standalone split. Independent NumPy geometry checks cover volume and the four shell areas. The shared render runner is validating all six videos in both languages and both audiences. Root orchestration owns the final render result and visual QA record.

An initial direct probe accidentally selected the long project-local Python environment and failed inside SciPy before loading scene content. The supported shared cache environment is used by the actual repository runner; root real renders successfully loaded the content. No new dependency was added.

## Owned Files

- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/README.md`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🔣️.json`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🎨️visuals/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧬️objects/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/👁️representations/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧩️models/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔌️exchange/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔗️consistency/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/📊️information/🎬️clip/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🧬️objects/🎬️clip/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/👁️representations/🎬️clip/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🧩️models/🎬️clip/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🔌️exchange/🎬️clip/🐍️.py`
- `🎓️teaching/🏛️architecture/🏗️bim/🔗️consistency/🎬️clip/🐍️.py`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-BIM-CLIP/content.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/CREATE-ARCHITECTURE-BIM-CLIP/probe.py`

Root owns playlist entry integration, the manifest, presentation titles and shared runtime changes. The schema/cases/tests are owned by the test agent. All generated probe/render artifacts remain under the ticket's generated folder for root cleanup after final verification.

## Final Handoff Audit

- All six README standalone commands and the playlist use actual registered IDs: `bim_information`, `bim_objects`, `bim_representations`, `bim_models`, `bim_exchange`, `bim_consistency`. Each manifest wrapper exists and declares the matching VIDEO_ID.
- English and German descriptions match all 49 source clauses (7 information, 9 objects, 7 representations, 9 models, 9 exchange, 8 consistency). The models video independently includes the chosen FEM schema, one-to-many mapping, assumptions and losses.
- No obsolete combined VIDEO_ID, manifest id or --video bim command remains in the BIM source documentation/tests. The root entry exposes the six-ID playlist.
- All nonempty Python docstring openings start with an emoji. New scene modules use the native BIM interface; that interface isolates the existing Manim runtime. The semantic model uses only Python standard-library modules. No runtime dependency manifest was introduced.
- Longest source path under BIM is 90 UTF-16 code units, below the 256-character limit. No AGENTS file was edited.
- No source files were changed during this audit, preserving the active render fingerprints. No content or documentation blocker was found; final decoded media QA and generated-output cleanup are owned by root.

## Read-only Render Cancellation Audit

At the audit snapshot, the native repository process-table interface found no process command containing this ticket's unique slug and therefore no remaining attributable render descendants. Both language render logs ended with a source-fingerprint failure while presenting the objects video. Four machine capacity grants existed, but generic grant PIDs are not evidence that those grants belong to this ticket; they must not be terminated or released based only on operation names.

The ticket runner uses synchronous Node spawn through a Windows command shell and does not install its own cancellation handler. The official authoring script does install SIGINT/SIGTERM handlers and owns exactly one uv child. Its Windows handler runs taskkill with that owned child PID and /T /F; its POSIX handler signals the child's detached process group. Python's ToolSession also stops only its tracked children when cancellation reaches it.

For these particular executions, the safe sequence is to interrupt only exec sessions 74815 and 17558, then inspect the native process table for this ticket's exact render.mjs/authoring command, explicit language and output-root path. If a verified dedicated authoring uv subtree survives, use the same PID-scoped tree termination as the official handler, after immediately revalidating ancestry and command identity. Never target executable names, all Python/Node/Chromium processes, generic capacity PIDs, or other agents' sessions. Do not reuse an old PID without a fresh ownership check. Capacity records of exited owners are reclaimed by the existing governor; there is no need to edit capacity state.

No process was killed, signaled or otherwise modified during this audit, and no source file was changed. Because no attributable process remained, no additional termination was indicated by the snapshot.
