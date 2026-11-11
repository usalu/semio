# Throwaway construction admissions

Rule: `admit_original_construction_settled` (and the tool-shaped form) continue a partly funded construction over the caller's resident `OriginalConstructionAdmission`; a fresh local or `&mut Default::default()` drops that in-flight state and panics ("original effect producer requires transfer or paid retirement"). Each owner keeps its own `construction` admission, retires it in `close_step` (before its binding clears) and counts it in `terminal_is_empty`. Owner births take the caller's admission and move a non-empty residue into the new owner.

## Fixed

| Site | Resident admission |
| --- | --- |
| decl-app `👷️job/🎮️command/🧬️op-binary` producer (`built`, `built_with`, PAGES, SEAL, take_into) | `OpBinaryProducer.construction` + `unit` marker, `unbuilt` in close; `admit_op_binary_producer` and `ArtifactOriginalCommandProducerFactory::admit` take the caller's admission (command crate passes its source `construction`; Energy factory ignores it, it uses the unsettled API) |
| decl-app `🧩️app/🛬️lifted/♻️retirement/🌱️birth` completion birth source | `OriginalCompletionBirth.construction`, closed in `close_step` |
| decl-app `📨️emission/🛂️metadata/🔗️view/routing.rs` (`paid`, `prepare_event`) | `OriginalHostEventRoutes.construction` + `unit`, closed in `close_step` |
| decl-app `…/🔗️view/🦀️.rs` `admit_routing_alias` | takes the caller's admission; callers: routes, command `PendingLatestWinsCommand.construction`, host-event delivery (`OriginalHostEventDelivery.construction`), typing (`OriginalStoreBirth::resident`) |
| instance staging `🏷️reserved` `ReservedPreparation::handoff` | its existing `wire_construction` |
| window transient `🦀️.rs` `built`, `🎟️capture`, `🌱️registration`, `🧾️issuer::admit_original_mutation` | capture and registration own `construction`; trait `capture_original_header` and `admit_original_mutation` take the caller's admission (retained-command emission, instance window registration via `OriginalStoreBirth::resident`, raster `PaintStrokeWork.construction`) |
| text-splice native composer (`admit`, `advance`, `take_into`) | `OriginalTextSpliceComposer.construction` + `unit`; writer fold passes its own `construction` |

## Not throwaway (no admission API)
Wrappers that use the unsettled `admit_original_construction`, effect-based `built` (tool-run operations, branch) or resident `OriginalReplayResident`/`OriginalStoreBirth` were checked and left alone.

## Remaining (need a signature ripple through trait or manual-init owners)
See the end of this file once verification finished.

### Verification status
- `check -p semio-plugin-decl-emission -p semio-plugin-decl-app --lib`: passes. `check -p semio-plugin-command` lib compiles; decl-app lib test target compiled after the Drop fix.
- Other touched crates (text-splice native, window-transient, instance, retained-command, raster, writer) were not re-checked after the edits.
- `test -p semio-plugin-command --lib`: the drop panic is gone, but the 6 runtime producer tests still fail in fixture setup: producer birth returns `Ok(false)` forever (fuel drops 9 per turn, grant items 11, copy 26704), so the settled birth never completes in the fixture grant. Fixture grant now quotes `Box<dyn ArtifactOriginalCommandProducer>` and the decoder admits its return table; root cause of the non-completing birth is not found.

### Remaining throwaway sites (not fixed)
Need caller-resident admissions threaded through traits or manual-init owners:
- decl-app `admit_reserved_work` (via `ArtifactEditor/Viewer::build_reserved_tool_job`, 16 editor overrides) and instance `ReservedPreparation::admit` slot birth (`pending` has manual field init)
- tool-run `🎞️tick/🛬️native` (state retires through controlled effects, no `cx` for `unbuilt`)
- `admit_original_tool_construction_settled` with `&mut Default::default()`: `👷️job/🏛️reserved` `admit_original_reserved_producer`, inference `🫴️receiving` (2), infinite `fill` (4), retained-command-job source binding (1); tests in `🧬️mutation-fixtures-job-close` and `⚖️slot-and-publication-fairness`
