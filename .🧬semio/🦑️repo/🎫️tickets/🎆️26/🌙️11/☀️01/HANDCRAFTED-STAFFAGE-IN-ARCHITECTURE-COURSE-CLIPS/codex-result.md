# Proper Staffage in Course Clips

## Scope and Current Evidence

The current archive contains 152 authored assets. All 97 course casts resolve to a catalog asset, an authored scale, nonempty geometry for the selected projection, and an existing pose or animation. The course source audit covers all 219 Python sources in its declared series and all 186 canonical clip source files across energy, optimization, BIM, Excel and Grasshopper. It found no omitted active clip series or forbidden staffage helpers.

## Repairs

- Animated fitting now measures the entire animation rather than its first frame. The regression failed with unit 2.0 instead of 0.4 before the fix; all three fitting tests subsequently passed against independent SVG and Manim rectangle oracles.
- Cooling comparison scenes select the winter or summer table and desk consistently, including the selected table height used to place the toaster.
- Catalog index pose lists now match their authored documents.
- Shared language-neutral coverage vectors and their Python/TypeScript twins reject missing, ambiguous, unauthored, malformed and invalid scale/view/scenario combinations. The TypeScript twin additionally uses Ajv as an independent schema oracle.
- Media provenance checkpoints refresh catalog, environment, print-profile, schema and exporter identities during a long-lived render run. Six edit scenarios failed before the fix and passed afterward; stale media cannot pass the existing pre/post publication identity comparison.
- Normal authoring verification now includes the course coverage and media freshness gates. Obsolete nonthermal exceptions were removed; the conduction material-control-volume diagram has one exact documented exception.

## Executed Verification

- Python course coverage: 5 passed.
- TypeScript course coverage: 59 passed, including independent schema-oracle checks.
- Physical scale and invariance: 9 passed.
- Six representative canonical casts exported successfully and were visually inspected: elevation/front/plan people, plan car, thermal person and thermal heat-pump.
- Three actual course casts exported with asset provenance and authored 1:50 selection; the corresponding elevation, plan and thermal front render proof passed and was visually inspected.
- Animated fitting: red/green confirmed; Fit class 3 passed.
- Schema bridge acceptance: 3 passed after provisioning its declared jsonschema dependency.
- Complete bridge acceptance: all 61 cases have passing evidence (57 passed in the full run; the three missing-dependency schema cases and the cache-input sensitivity case passed in targeted reruns after dependency and redundant-memoization repairs). No failed case was omitted or weakened.
- Complete staffage-only source guard: all 39 tests passed in 427.489 seconds, with zero failures or skips.
- Media source checkpoint freshness: 2 passed, including six shared edit vectors and real scene dependency closure.

- Video timing: all 5 tests passed with real rasterization, encoding and independent decoding. MP4 and VP9 WebM each contain 14 frames at 12 fps; RGBA decoding verifies both transparent and opaque WebM pixels. The first-party MP4 path independently decodes its exact 5 frames at 10 fps.

All selected checks are terminal and successful. The earlier missing-tool and stale-fixture failures in the detailed reports are historical and superseded by these results. The only remaining watt-comparison pictogram is the scientific sun symbol; all physical device and person comparisons use canonical casts. No missing catalog asset required an additional drawing in this pass.

## Boundaries

Verification ran on native Windows through the repository tool interface. No macOS, Linux or devcontainer execution is claimed. No narration or TTS was changed. The source and cache-publication contracts were verified; this pass does not claim a full narrated course video rebuild. Full archive tests unrelated to the changed staffage integration were not rerun. Research, source audits and detailed commands are retained in codex-coverage.md, codex-fit.md, codex-media.md, codex-clips-audit.md, codex-final-audit.md and codex-staffage-audit.md.

## Infrastructure and Collaboration

The installed repo launcher and native MCP executable were recovered without editing AGENTS.md, modifying Git state or creating a worktree. The complete repo://goals resource was read through native MCP; the matching existing open ticket remains associated with Running Drawing. Work used four available parallel slots, with GPT 6.1 Sol medium execution agents and low-effort audit agents. Only task-specific changes were made alongside other concurrent workspace work.

## Cleanup

All owned generated runtimes, binaries, proof media and logs were removed after the final live sessions completed. Reusable dependency manifests, portable configs, proof manifests/scripts and Markdown evidence remain in the ticket.

Final course guard: 39/39 passed, without skipped cases. Ticket closed through native repo MCP with all 33 changed source and retained evidence/input paths recorded. Final filesystem audit confirms closed status and absence of generated output.
