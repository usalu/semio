# Claims verified (read-only audit)

Scope: `✏️s/🔌️plugins/📕️norm` (abbreviated `norm/`), `🧰️framework`, and the hub composition `🌎️hub/🧩️compositions/📕️norm` for shell-hook wiring. Method: literal grep and file walks, no builds, no git writes. "Wired" means the symbol is called or invoked from a test, editor, viewer or composition, not just defined. Artifact paths are shortened to `🗿️artifacts/<id>/…/`.

## Missing or unwired (read this first)

1. **MECHANICS: `analyse_thin_walled` is NOT used in en1993 or en1999.** It is defined at `norm/🧮️mechanics/🔲️section/🦀️.rs:174` and re-exported at `norm/🧮️mechanics/🦀️.rs:40`, but its only callers are the mechanics unit tests (`norm/🧮️mechanics/🧪️tests/🔲️section/🦀️.rs:253,277,288-290`). Zero hits in every artifact. The claim as written is false.
2. **LIVE-UNITS: `incremental_laws!` is not invoked in en1993.** The macro is invoked in 9 artifacts (en1990, 1991, 1992, 1994, 1995, 1996, 1997, 1998, 1999). en1993's incremental unit test (`🗿️artifacts/🔩️en1993/…/✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs`) calls `evaluate_units` directly and does not use the macro.
3. **SHARED v3: `project_whole` is absent from en1991 and en1992.** en1991 uses `project_parts` (`🗿️artifacts/🏋️en1991/…/🎬️scene/🦀️.rs:25`). en1992 uses `canonical_order` and `merge` (`🗿️artifacts/🏛️en1992/…/🎬️scene/🦀️.rs:61`). Present in en1990 (`🎬️scene/🦀️.rs:61`) and en1993 to en1999.
4. **I18N: `german_scene_leaks` has no artifact call site.** Definition at `norm/🎬️structural-scene/🎯️projection/🗣️description/🦀️.rs:90`. Callers are the framework's own projection tests (`🎯️projection/🧪️tests/🦀️.rs:295,297`) and `assert_scene_laws` in `norm/🎬️structural-scene/🧰️projection-kit/🧪️test-support/🦀️.rs:26`. That helper is called only by en1990, en1991 and en1992.
5. **I18N: `assert_scene_laws` and `assert_diagram_laws` are not called in en1993 to en1999.** Calls exist only in en1990 (34 and 20 hits), en1991 (39 and 5) and en1992 (19 and 19). `assert_handle_laws` is called in all ten artifacts.
6. **DESIGN CASES: `full-catalogue demo documents` are only partial.** `demo-document` directories exist in en1990, en1991, en1992, en1996 and en1997 (three in en1997). en1993, en1994, en1995, en1998 and en1999 have none under that name. The `pub cases` field is present in the snapshot code of en1990, en1991, en1992, en1995, en1996 and en1997.

Everything else in the list below is present and wired.

## Verification table

| Claim | Expected | Found (file:line) | Wired | Verdict |
|---|---|---|---|---|
| SCENE: `every_handle_binding_of_the_default_document_resolves_to_a_typed_kind` | Test in live-acceptance macro | `norm/📇️registry/🧬️contract/🖥️app-surface/🧪️tests/🔬️live-acceptance/🦀️.rs:133` (inside `norm_live_acceptance!`, macro starts `:5`) | yes, via 10 editor invocations | VERIFIED |
| SCENE: `a_synthetic_placement_drag_previews_the_complete_row_commits_one_event_and_reloads_to_the_same_pose` | Test in live-acceptance macro | `…/🔬️live-acceptance/🦀️.rs:69` (inside macro) | yes, via 10 editor invocations | VERIFIED |
| SCENE: `norm_live_acceptance!` stamped into editors | 10 editors invoke it | `🗿️artifacts/<each of 10>/…/✏️editor/🦀️.rs:250-256` (en1990 `:255`, en1994 `:250`) | yes | VERIFIED |
| SCENE: module `📍️placement-drag` | Module declared and used | `norm/📇️registry/🧬️contract/🖥️app-surface/🕹️direct-manipulation/🦀️.rs:20-21` (`#[path]` mod plus `pub use`); files `…/📍️placement-drag/🦀️.rs` and `…/🧪️tests/🔬️unit/🦀️.rs` | yes | VERIFIED |
| REALTIME-QA: `norm_latency_bench!` in all ten editors | Macro defined, invoked in 10 editors | defined `norm/📇️registry/🧬️contract/🖥️app-surface/🧪️tests/⏱️latency-bench/🦀️.rs:273`; invoked at each `🗿️artifacts/<id>/…/✏️editor/🦀️.rs` (`:251-257`, e.g. en1990 `:256`) | yes, 10 of 10 | VERIFIED |
| REALTIME-QA: `viewport::full_scene_of` | Defined per artifact, used by bench | `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs:34` in all 10 artifacts; passed as `full = viewport::full_scene_of` in each bench call | yes | VERIFIED |
| REALTIME-QA: `standing_pose` | Defined, used | `norm/…/🕹️direct-manipulation/📍️placement-drag/🦀️.rs:24`; used in `…/🫧️live-lanes/🎛️editor-wiring/🦀️.rs:231` and in placement-drag unit tests | yes | VERIFIED |
| REALTIME-QA: `derived_yaw` | Defined, used | `…/📍️placement-drag/🦀️.rs:14`; used in `…/🫧️live-lanes/🎛️editor-wiring/🦀️.rs:4,231` and `…/🧪️tests/🔬️live-acceptance/🦀️.rs:109` | yes | VERIFIED |
| SHARED v3: `SceneKit` | Trait impl per artifact | `🗿️artifacts/<id>/…/🎬️scene/🦀️.rs` (en1990 `:85`, en1991 `:44`, en1993 `:43`, en1994 `:93`) in all 10 | yes | VERIFIED |
| SHARED v3: `Kit<K>` | Used by live scene | `live_scene!(pub mod live = Kit<…>)` in each `🎬️scene/🦀️.rs` (en1994 `:120`, en1996 `:148`) and `🎬️scene/🧪️tests/🔬️live/🦀️.rs:25` | yes | VERIFIED |
| SHARED v3: `project_whole` | Used in each artifact | Used in en1990 (`:61`) and en1993 to en1999; not in en1991 (uses `project_parts`) or en1992 (uses `canonical_order`, `merge`) | yes where used | PARTIAL (see missing item 3) |
| LIVE-SCENE: `live_scene_of` in each editor and viewer `scene_of` | Defined per artifact, called from editor and viewer | Defined in `🎬️scene/🦀️.rs` of all 10 (en1990 `:114`); en1990 editor `…/✏️editor/🎭️modes/✏️edit/🪟️windows/🧊️viewport/🦀️.rs:30` and viewer `…/👁️viewer/🎭️modes/👁️view/🪟️windows/🧊️viewport/🦀️.rs:32` | yes | VERIFIED (checked en1990 callers; the other nine hit 5 times each by count) |
| LIVE-SCENE: `🔬️live` tests in each artifact | Test module per artifact | `🗿️artifacts/<id>/…/🎬️scene/🧪️tests/🔬️live/🦀️.rs` in all 10 | yes (module tests) | VERIFIED |
| LIVE-UNITS: `with_every_collection` | Defined, used in `incremental_laws!` | Defined `norm/⚖️compliance/⚡️incremental-evaluation/🔍️read-tracker/🦀️.rs:169`; used in `…/🧪️laws/🦀️.rs:9,17,27,38` inside the macro (macro at `:5`) | yes | VERIFIED |
| LIVE-UNITS: `incremental_laws!` used in each artifact | Invoked in all 10 | Invoked in 9 artifacts at `…/✏️editor/🧮️incremental/🧪️tests/🔬️unit/🦀️.rs:9` (en1995 `:11`); none in en1993 | 9 of 10 | MISSING for en1993 (missing item 2) |
| SHELL-HOOKS: `register_shell_hooks` | Defined in framework, called | Defined `🧰️framework/🔨️modules/🛂️manifest/🧱️app-surface/📱️shell-hooks/🦀️.rs:155`; re-exported `🧰️framework/📦️packages/🦀️rust/🦀️.rs:16,137`; called in `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:190` and in `norm/…/🔌️shell-hooks/🧪️tests/🦀️.rs:126` | yes | VERIFIED |
| SHELL-HOOKS: `PluginBuilder::shell_hooks` | Method called by hub | Defined `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️builder/🦀️.rs:189`; called `🌎️hub/🧩️compositions/📕️norm/🦀️.rs:80` | yes | VERIFIED |
| SHELL-HOOKS: `NormShellHooks` registration in hub | Registered in `🌎️hub/🧩️compositions/📕️norm` | Defined `norm/…/🖥️app-surface/🔌️shell-hooks/🦀️.rs:112` (impl `:114`); registered `🌎️hub/🧩️compositions/📕️norm/🦀️.rs:80` | yes | VERIFIED |
| TRACEABILITY: `norm_compliance::spec_trace` | Module exposed by compliance crate | `norm/⚖️compliance/🦀️.rs:29` `pub mod spec_trace`; codegen at `norm/⚖️compliance/🧭️spec-trace/🧰️codegen/🐍️.py` | yes | VERIFIED |
| TRACEABILITY: `assert_traced` in each artifact's tests | Called in each artifact | Defined `norm/⚖️compliance/🧭️spec-trace/🦀️.rs:198`; used in 10 artifact test files (1 to 2 per artifact), e.g. `🗿️artifacts/🪨️en1996/…/🧪️tests/🧭️spec-trace/🦀️.rs:3`, `🗿️artifacts/🔩️en1993/…/🧪️tests/🗂️catalogue-trace/🦀️.rs:6` | yes | VERIFIED |
| TRACEABILITY: `spec_trace_table` in app-surface | Defined in app-surface, used by artifact catalogues | Defined `norm/📇️registry/🧬️contract/🖥️app-surface/📚️catalogue-and-inspection/🦀️.rs:71`; called once per artifact (e.g. en1991 `🗿️artifacts/🏋️en1991/…/📌️panels/📚️catalogue/🦀️.rs:25`, en1994 `:26`, en1996 `:36`) | yes, 10 of 10 | VERIFIED |
| MECHANICS: `analyse_section` | Defined, used in en1993 and en1999 | Defined `norm/🧮️mechanics/🔲️section/🦀️.rs` (re-export `norm/🧮️mechanics/🦀️.rs:40`); en1993 `…/⚖️checks/🔗️special-element-checks/📐️parametric-checks/🦀️.rs:33` and oracle-wiring `…/🎯️oracle-wiring/🦀️.rs:134,303-308`; en1999 `…/⚖️checks/🪢️free-outline-torsion/🦀️.rs:21` | yes | VERIFIED |
| MECHANICS: `analyse_thin_walled` | Used in en1993 and en1999 | Only in mechanics tests (`norm/🧮️mechanics/🧪️tests/🔲️section/🦀️.rs:253,277,288-290`) | no | MISSING (missing item 1) |
| NDP: `ndp_cell` | Defined, used | Defined `norm/⚖️compliance/📜️annex-and-tables/🦀️.rs:58`; used in en1996 `…/📚️masonry-tables/📚️normative-tables/🦀️.rs:85,86,120`, plus 1 hit in en1992 and 1 in en1999 | yes | VERIFIED |
| NDP: `ndp_interpolate` | Defined, used | Defined `…/📜️annex-and-tables/🦀️.rs:73`; used en1993 `…/⚖️checks/🧮️clauses/🔬️stainless/🦀️.rs:105` | yes | VERIFIED |
| NDP: `annex_parameter_rows` | Defined, used | Defined `…/📜️annex-and-tables/🦀️.rs:111`; used in app-surface `…/🪟️visual-windows/🧾️properties/🦀️.rs:303`; compliance unit test `norm/⚖️compliance/🧪️tests/🔬️unit/🦀️.rs:413-421` | yes | VERIFIED |
| I18N: `german_scene_leaks` | Used by artifacts or the test-support helper | Defined `norm/🎬️structural-scene/🎯️projection/🗣️description/🦀️.rs:90`; used by `assert_scene_laws` (`🧰️projection-kit/🧪️test-support/🦀️.rs:26`); no artifact calls it directly | only via scene-laws in en1990/1991/1992 | PARTIAL (missing item 4) |
| I18N: `assert_handle_laws` | Defined, called per artifact | Defined `norm/🎬️structural-scene/🧰️projection-kit/🧪️test-support/🦀️.rs:30`; called in all 10 (en1990 8, en1991 9, en1992 14, en1993 6, en1994 4, en1995 14, en1996 4, en1997 5, en1998 6, en1999 6 hits) | yes | VERIFIED |
| I18N: `assert_scene_laws` | Called per artifact | Defined `…/test-support/🦀️.rs:16`; called only in en1990 (34), en1991 (39), en1992 (19) | only 3 of 10 | PARTIAL (missing item 5) |
| I18N: `assert_diagram_laws` | Called per artifact | Defined `…/test-support/🦀️.rs:54`; called only in en1990 (20), en1991 (5), en1992 (19) | only 3 of 10 | PARTIAL (missing item 5) |
| DESIGN CASES: `cases` in the en1990, 1991, 1992, 1995, 1996, 1997 snapshots | `pub cases: Vec<CaseRow>` in snapshot code | `🗿️artifacts/⚖️en1990/…/🧬️schema/📸️snapshot/🦀️.rs:87` (`cases` also in `🛰️.proto:92` and `🔗️.graphql:151`); same field present in en1991, en1992, en1995, en1996 and en1997 (en1997 has two) | yes (schema snapshot, used by snapshot code) | VERIFIED |
| DESIGN CASES: full-catalogue demo documents | Demo documents in each artifact | `demo-document` dirs in en1990, 1991, 1992, 1996, 1997 (3); en1997 `🧫️fixtures/📇️catalogue/📚️demo-document/🔣️.json` | partly | PARTIAL (missing item 6) |

## Notes

- Counts come from literal-string scans across the artifact trees (`matrix.py` in the scratchpad). The `Kit<`, `SceneKit`, `live_scene_of` and `assert_handle_laws` counts are per artifact and were not each traced to a call site. The call-site checks above cover the claims marked "wired".
- The `cases` grep on the whole artifact tree is too broad (13k hits in the framework), so the snapshot check was narrowed to the `pub cases: Vec<CaseRow>` field.
- `wave2` reports themselves were not re-read. This audit checks only the symbols the lead listed.
