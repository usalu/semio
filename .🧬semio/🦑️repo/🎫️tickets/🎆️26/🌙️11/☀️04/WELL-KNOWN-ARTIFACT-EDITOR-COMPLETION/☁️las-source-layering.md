# LAS Source Layering

The actual lower mutation crate mounted the LAS editor, whose native codec called an upper engine module absent from that crate. The snapshot crate also mounted editable-main rendering, whose scene path called the same absent upper engine. A schema/inference move alone would create a snapshot-to-schema-to-snapshot cycle.

Root moved the read-only editor-main mount from snapshot to schema beside viewer-main. Mutations reexports both main surfaces from schema. Pure native codec code now has one source at IO/natural, mounted by existing schema inferences; editor codecs call that lower owner explicitly. Scene geometry lives in existing schema inferences and both window sources call it there. Upper IO explicitly reexports the same functions. No new crate or runtime dependency was introduced. The existing first-party LAS test oracle is now an explicit dev dependency of the mutation crate, whose own editor unit tests already call it.

The neutral JSON owner law started red on the unwanted snapshot UI mount and passed after the move (1test/1Ajv assertion). Existing actual ArtifactEditor native open/edit/save/refusal/independent LAS library tests remain in the editor suite; no duplicate tests were created. Native LAS execution is queued in the medium agent sequential repair plan and is not yet green. Existing sample metadata was inspected independently:267bytes, LASF, version1.2,2points,0VLRs,20byte point records.

Changed source/input files: IO root, new IO/natural leaf, schema/inferences, snapshot package mount, schema package mount, mutation package reexports, editor codec calls, edit/view main scene paths, neutral IO test fixture, and ticket las-layering.test.ts input. All paths are below256UTF16 units.

Read-only independent audit confirmed both source path anchors, no remaining snapshot::editor consumers across authored Rust, and acyclic package direction snapshot←schema←mutations. The remaining editor-unit scene call was also changed to the canonical lower inference owner, because that test is compiled in the mutation lib test target without an upper engine. Real native compilation/test execution remains required.

The independent LAS oracle compilation also exposed its missing VLR inverse-wire function. The oracle now retains raw VLR octets through vlr_to_wire, with a permanent non-UTF-8 neutral vector and an actual las::raw::Vlr write/read test. Projection text remains distinct. A native retry is queued; successful execution remains pending.

The next native retry compiled the oracle repair and exposed a real testkit dependency closure failure: demo_diff_cases was enabled for testkit, while its base_point/base_vlr helpers were limited to cfg(test). Both private helper gates now match their testkit caller. Actual next retry remains pending; no broader runtime API or feature widening was introduced.

The next native retry reached test consumers. DiffText/DiffBinary and WindowKit imports are restored at the actual unit-test scopes; editor for_each callbacks now consume their MutationOutcome as statements. The owned SQLite receiving tests retain and check RetainedWorkOutcome.work through scoped observers, with zero work on denied/canceled probes and bounded work on paid close. Five changed physical test files parse successfully; the next full LAS retry remains pending.
