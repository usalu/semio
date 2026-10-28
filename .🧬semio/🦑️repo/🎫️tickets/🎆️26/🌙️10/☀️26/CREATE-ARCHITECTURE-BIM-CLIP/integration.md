# BIM Clip Integration

## Plan and ownership

- Main chat coordinates integration and verification. Four agent slots total: two High execution agents, one low read-only exploration agent followed by a second low independent content/standards auditor.
- Content agent authors six bilingual scenes and a native semantic/visual interface. Test agent writes schema-first language-neutral cases, Python tests, NumPy and Ajv oracles.
- Existing curriculum and presentation navigation register BIM as the sixth lesson with six chapters. Fixed energy-only cardinality and path constraints become architecture-wide.
- Source renders propagate explicit content language and include language in source fingerprints, output directories and canonical speech lookup. English and German cannot reuse each other's rendered text or recordings.

## Ticket infrastructure

The repository MCP ticket_open returned an open in-memory record without creating a local folder. The same logical ticket was materialized with repo ticket open in the shared checkout. No Git mutations, worktrees, AGENTS edits, goals or quiz content changes are part of this work.

## Verification

Schema-first red test recorded by the test agent in bim-tests.md. Primary-source conceptual audit recorded in content-audit.md. Further test and render results will be recorded here before closing.

## Six independent outputs

User clarified that the six conceptual topics need separate videos. The curriculum now contains six independently selectable BIM video identities and wrappers, each with its own localized title, source scene, presentation entry and output file. The shared BIM entry renders the six-video playlist and does not register a combined lesson. Source renders remain canonical and reusable.

Shared runtime and hierarchy suite: 35 tests passed, one existing opt-in test skipped. Initial German render exposed three captions exceeding the native two-line limit; the content and test agents are shortening those clauses and adding exhaustive bilingual caption checks before rerendering.

## Final source verification

Fifteen BIM contract tests cover identity, inheritance/composition, event-derived representations, same-schema partial models, one-to-many FEM translation and loss, six independent routing targets, explicit locales, source audio/media/trace isolation, and native caption bounds. All98 caption texts were validated under both audience policies (196 layout calls). Ajv independently validates the semantic model and five invalid mutations, plus the full curriculum and presentation JSON Schemas.

The shared runtime/media and hierarchy suite passed35 tests with one existing opt-in skip before the six-way split. After splitting, all three affected curriculum/routing/discovery contracts passed. A newly added concurrent presentation choreography test concerning JSON key ordering failed once; its implementation is outside this ticket.

All narration is authored in English and German. No voice recordings are provided by this task; previews therefore contain animations and Speaker captions, with the existing runtime ready to attach canonical clause recordings when supplied.

## Final integration refinements

The BIM suite now contains20 contracts. The final focused run passed28 tests (18 BIM and10 affected shared contracts), followed by the new sources-only browser regression. The browser metadata path now bundles its adapter to retain complete source provenance, but defers adapter evaluation and Playwright loading until actual rendering. An isolated loader that throws if evaluated demonstrates the sources-only path is read-only. No package was added and no success-only publication guard was relaxed.

The selected-source manifest interface keeps full validation by default, allows metadata enumeration without opening unrelated source files, and validates selected wrapper/segment assets and clause keys strictly. This allows BIM rendering while other contributors author newly registered lessons; global containment, identifiers and clause contract validation remain unconditional.

The final preview runs use the documented authoring command through a lightweight orchestration process. Previous attempts correctly rejected snapshots changed by ongoing shared-catalog/runtime edits. The final source state is frozen for the completed-media check.

## Completed media and cleanup

The final four-worker run completed24/24 jobs with no failures. PyAV decoded all1942 frames of640×360@1fps previews. Every within-language Speaker/Spectator pair had identical frame counts. All12 Speaker samples contained caption glyphs, all12 Spectator samples contained none, and all6 English/German caption comparisons passed. Source and representative composed-frame visual audits passed. Maximum generated path length was172. See media-verification.md and media-audience.md.

The resolved generated-directory target was verified within this exact ticket and workspace, then removed. All authored input scripts, schemas/cases, source files and Markdown reports were retained. No other ticket or shared dependency cache was removed.

## Ticket closure

Native repo ticket close completed and local metadata is closed. The first MCP close attempt failed during its management Git inspection outside the checkout; the management-free MCP retry closed its record successfully. Final verification confirmed the local ticket closed, summary retained and generated subtree absent. All64 source/metadata/report/input paths were recorded at closure.
