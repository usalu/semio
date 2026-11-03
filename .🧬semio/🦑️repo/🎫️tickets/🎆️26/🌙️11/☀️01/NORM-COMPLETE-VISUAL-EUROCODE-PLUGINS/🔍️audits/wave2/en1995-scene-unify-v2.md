# EN1995-SCENE-UNIFY v2 (wave 2)

Status: UNCOMPILED (no cargo/rustc/`repo run`; no freeze on en1995, only build-en1990). `- ready for rebuild: en1995 scene unify v2` appended to `build-en1995/latest.md`.

## Finding: the premise is stale
The 22 "hand-rolled" cases were already moved onto the shared forms by FORMS-V2 (`forms-v2.md` section 3): `entities::illustrates`/`ILLUSTRATED`/`Layout::kernel_case` are gone (grep: none), every case body is a form `Solid`, and `cases::project`, `Layout`, `handles` all go through `forms::draw_with/handles_with/origins_with` with `entities::case_extras` (`scene/🧱️entities/🧩️cases/🦀️.rs`). Prism (T-30..32), laminate (T-97/107), char/front/residual (T-101/102/106/107), mode (T-63..66/120/124), assembly (T-90/93/94/95/96/121/122), GripKind grips (count/spacing/gauge) and `dim_*` annotation roles are all shared-form features. The remaining kernel-specific code is only the data supplier `case_extras` (fire depths, mode shapes, assembly layouts), which needs the timber kernel and cannot be domain-neutral; the typed entities keep `rows/special` (used by the typed rows, not by cases). No new shape was needed in case-kit.
The check-to-entity highlight is wired generically in `fragment` (`projection_kit::highlight_owners` over cases and entities alike).

## Added (the missing proofs for the 22)
- `🧪️tests/🗂️cases/🦀️.rs`: `the_twenty_two_kernel_cases_are_coloured_by_their_worst_check_and_carry_grips_on_their_inputs` (utilisation binding + Utilisation colour channel + at least one grip for every one of the 22 entries present in the demo).
- `🧪️tests/🔬️live/🦀️.rs`: `incremental_equals_full_when_the_inputs_of_the_shared_form_cases_change` (40 random input edits and placements over the whole catalogue demo, incremental scene equals full scene). The existing live test only edited members.

## Open
- Written by reading, not run: `scene::ColourChannel::Utilisation`, `norm::CheckStatus` paths in the cases test; `Rng`/`place`/`report_of` reused in the live test.
- Entries without a demo case are skipped by the new cases test (`let Some(case) ... else continue`).
- Goldens still need `SEMIO_UPDATE_GOLDEN=1` after the rebuild; handle-paths/scene-law checks via heavy.sh not run (no cargo).
- Language-agnostic oracle for these forms exists already (forms oracle: taper, apex, plies, char, mode, assembly, dimension).
