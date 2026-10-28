# IFC Ring Differential Fuzz

Executed 2026-10-21 using ticket-only ifc4-rings-fuzz.mjs and existing Three public ShapeUtils/Vector2 oracle. No production source changed. The harness loads current shared geometry/🕳️rings/🟦️.ts by native type stripping to ticket generated .mjs, preserving actual source behavior.

## Deterministic Coverage

Seed 0x5e4d10; 80 valid ordered radial/simple outer rings (5–16 vertices), one to four disjoint rectangle holes within guaranteed interior radius, each across xy/yz/zx plus an oblique plane, uniform scales 1/1e-90/1e90, and mixed forward/reversed winding = 1,920 cases. The oracle checks filled area and no-Steiner triangle counts independently; the own result checks area and expected triangle count. This is structured deterministic differential testing, not a full arbitrary polygon proof.

## Failure Found and Resolution Verified

Original shared engine failed 64/1,920 cases, all on the oblique plane, with polygon-triangulation-failed. Three accepted all 64 with expected triangle counts. Axis-aligned versions passed. First neutral input is retained in ifc4-rings-fuzz-repro.json; reduced input in ifc4-rings-fuzz-minimal.json has five outer vertices and two rectangle holes. Oblique transform is [x/sqrt(2)-y/sqrt(6), x/sqrt(2)+y/sqrt(6), 2*y/sqrt(6)].

Root cause: repeated bridge vertex ids appear at multiple boundary occurrences, and nearest-visible-id selection ignored the occurrence local interior cone. A geometrically visible bridge could be inserted at the wrong occurrence, producing a bad weakly-simple boundary for ear clipping. Main added local interior checks at both candidate merged-ring occurrence and hole endpoint, and a permanent oblique regression. Fresh independent rerun after fix: **1,920 passed, 0 failures; maximum work 11,566**. No production fix was authored by this audit agent.

The retained harness is rerunnable with run(root,ticket). Repro JSON files are deliberate reusable test inputs, not logs. Generated transformed/debug modules were removed after completion.

## Shared Native Engine/Reader Static Audit

Files inspected: IFC common 📐️geometry/{🦀️.rs,📖️graph/🦀️.rs,🕳️rings/🦀️.rs}, IFC4 schema geometry facade and 📖️reader/{🦀️.rs,🟦️.ts}, IFC2x3 base schema geometry 📖️reader/🦀️.rs.

Concrete semantic issue surfaced to main/high: IFC4 reader sets constituent_count to complex.len().max(1) in Rust and complex?.length||1 in TypeScript, and constituent(index) skips primary whenever complex exists. IfcEntity.complex stores ADDITIONAL constituents beyond primary (snapshot docstring explicitly says this). Exactly one extra constituent is therefore reported as count one and bypasses unsupported-complex-product/representation refusal. Correct cardinality is 1+complex.len, constituent index0 must be primary and later index−1 extras. IFC2x3 Part21 instances store all constituents directly, so their raw entities.len is appropriate. This must not be blindly unified across readers. Reader source remained untouched.

No additional likely Rust type/lifetime blocker was established from static inspection. GeometryEntity explicitly implements unconditional Copy/Clone over borrowed slices, so entity-map copied() does not require V:Copy. GeometryGraph::Value is Sized by default, matching aggregate &[Self]. Borrowed reader lifetimes are tied to graph owner and projected entities are Copy descriptors, avoiding an engine self-borrow across recursive mutable methods. Rust rings stores tick in a Triangulator struct instead of competing closures over a mutable callback; local-cone parity matches corrected TypeScript. Common helper cross/subtract/dot are pub(super), so facade test imports are not the private-child visibility defect they initially appeared to risk.

No native builds were attempted because the unchanged Windows cachegate is known. Static audit is not native typecheck proof. Current generic engine facade and package registration should still be checked through the proper repo build once that blocker is resolved.
