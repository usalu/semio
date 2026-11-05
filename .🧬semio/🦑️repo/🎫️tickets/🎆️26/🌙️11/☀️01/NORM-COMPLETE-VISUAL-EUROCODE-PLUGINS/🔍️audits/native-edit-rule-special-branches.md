# Native Family Edit-Rule Special Branches

Bounded read-only inventory of all15 actual family edit rules and DIN4108 actual editor caller. No new planner/build. Base for each relative path below: ✏️s/🔌️plugins/📕️norm/🗿️artifacts/<family>/🏅️standards/🔖️1/🪆️subsets/✳️any/.

| Family | Actual Function | Constructor Preparation Outside Generic Rule |
| --- | --- | --- |
| 🏭️vdi3805 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| 🪨️en1996 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| 🏋️en1991 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| ⚡️din18599 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| 🌍️en1997 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| 📇️iso16757 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| 🏛️en1992 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| ⚖️en1990 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| 🪵️en1995 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| 🪶️en1999 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| 🫨️en1998 | mutations/🧭️edit-rules/🦀️.rs resolve_edit | None: EDIT_RULES.resolve_with_id_removals(ID_REMOVALS,document,edit). Keep explicit ID-removal table behavior. |
| 🧱️din4108 | ✏️editor/🦀️.rs:200 inspector_resolve | None: directly calls aggregate EDIT_RULES.resolve; no family resolve_edit export. |
| 🧩️en1994 | mutations/🧭️edit-rules/🦀️.rs:181 resolve_edit | SetField passes conform(document,path,value) before generic resolve; other edits retain ID-removal resolution. |
| 🔩️en1993 | mutations/🧭️edit-rules/🦀️.rs:25 resolve | Numeric SetField members[…].sectionId invokes choose_section; all other edits generic resolve. |
| 🌬️din16798 | mutations/🧭️edit-rules/🦀️.rs:75 resolve_edit | RemoveItem zones or ventSystems resolves actual row ID and constructs concrete RemoveZone/RemoveVentSystem; all other edits generic resolve. |

Exact source prefix for mutations above is 🧬️schema/🧬️mutations. These three special branches must survive a generic kind/payload receiver:

- EN1993 choose_section (line12) chooses rolled HEB catalogue section nearest requested height by abs(h-height). If document.sections lacks its ID, prepends InsertSection {index:None,section}; then resolves original member sectionId path with selected string ID. Preserve ordered multi-operation result, optional insert-section followed by normal change-member-section-id setter (actual table 🧭️edit-rules/📚️table/🦀️.rs, newSectionId with id selector), and the fact the section pick depends on the actual document/catalogue. Enum token leaves remain direct.
- EN1994 conform (line170) projects current actual document leaf: numeric-to-bool threshold>=0.5; numeric to nearest catalogue designation for *.steel.designation (table kind change-beam-steel-designation); otherwise coerce_like current numeric kind and fallback original supplied value. This is preprocessing, not an independently different mutation kind. Preserve current count/integer coercion and bool handling, not just the named steel special case.
- DIN16798 row_position (line61) accepts collection root+supplied index, bracket numeric index or [id=…]; validates actual existing row. Root zones creates remove-zone with zoneId, root ventSystems creates remove-vent-system with ventId. Unknown ID/index errors are real refusal behavior. Avoid replacing these ID-addressed deletion payloads with generic positional fields.

## Descriptor Corpus Runtime Input Ownership

Arg-schema actual authored project is 🧰️framework/🔨️modules/🛂️manifest/🧮️arg-schema/📦️packages/🦀️rust/📋️project.json, @semio-tech/manifest-arg-schema-rs. Exact shared corpus is 🧰️framework/🔨️modules/🛂️manifest/🧪️tests/🧫️fixtures/🧫️mutation-inputs/🔣️.json. Register this canonical file under that project's namedInputs.nativeSources with {workspaceRoot}/ prefix. Existing build/📥️inputs/🟨️.mjs:322 explicitly folds declarations.nativeSources into nativeTestSources; project-assembly/🟨️.mjs:70 chooses nativeTestSources for native test targets. This keeps actual JSON hashed when include_str is removed from Rust source closure.

Canonical runtime reader semio_framework_test_fixtures::repo_text!(full workspace-relative path) returns &'static str; fixture_text! instead anchors relative path at call-site CARGO_MANIFEST_DIR. For arg-schema package, relative corpus path is ../../../../🧪️tests/🧫️fixtures/🧫️mutation-inputs/🔣️.json. Existing native-input-cases.rs now uses repo_text! through corpus(), as independently observed during this pass. Same canonical file must be declared in each separate authored compiler/live-editor/SDK test owner's actual nativeSources if its runtime law reads it; declarations on arg-schema alone do not automatically prove another owner hashes runtime data. No fabricated source receipt or runtime reader substitution is required. Sent exact path/API evidence to descriptor_recovery.

## Source-Proven Inferred Project Identity

Current build inference 📦️project-nodes/🟨️.mjs:198-210 iterates Cargo manifests, skips an already authored root at205, and uses manifest.package.name directly at210;216 emits that exact project name/root. The actual live-editor Cargo package name is semio-s-plugin-norm-app-surface-live-editor. Therefore an authored 📋️project.json at its existing package root with that exact name preserves its inferred identity while allowing explicit nativeSources. Adding an @semio-tech prefix would change identity; it cannot be called preservation based on naming convention. Arg-schema already has authored project name @semio-tech/manifest-arg-schema-rs. Shared runtime corpus input must be declared at each actual owner as detailed above. Descriptor recovery received this exact source proof.

Bounded matrix check: one live process1274547 matched all15 norm run-many build,check,test configuration prototype parallel1. Existing native-all15-current.log at2026-11-04T23:36:18Z showed genuine capacity queue at217s, requested5632MB with2790MB spare above floor,2 running,0 ahead. No duplicate matching matrix process observed; no new planner or build launched by this audit. This is a point-in-time queue observation, not a terminal result.
