# Wave 3 — App family sweep (parallel, one agent per family, zero file overlap)

Prerequisite: Wave 2b merged, full workspace builds green, root `script.ts` has
`POLICY_PROTOCOL_COMPLETENESS_ALLOWLIST` seeded with every file that needs this wave's work (get
the exact list from Wave 2's final report or by reading `POLICY_PROTOCOL_COMPLETENESS_ALLOWLIST`
in `/Users/ueli/Documents/semio/script.ts` directly).

## Per-family agent instructions (fill in `<FAMILY>` = your assigned crate(s))

Your job: for every `*.rs` test file in `<FAMILY>` that currently calls
`vcs::test_support::assert_dsl_pack_equivalence(` or `assert_document_pack_round_trip(`, add a
call to `vcs::test_support::assert_document_protocol_round_trip(` and
`assert_ops_protocol_equivalence(` right beside it (same test function, same fixture/store — these
asserts take the same `&DocumentVcsStore<P, Operation>` your existing pack asserts already have in
scope). Then remove your file(s) from `POLICY_PROTOCOL_COMPLETENESS_ALLOWLIST` in root
`/Users/ueli/Documents/semio/script.ts` (this is a **shared file** — make ONLY the deletion of your
own family's entries, re-read the current allowlist content first since other family agents are
editing it concurrently; if your edit conflicts, re-read and retry rather than overwriting).

Do not touch any file outside `<FAMILY>` except that one allowlist deletion in `script.ts`. Do not
change business logic, do not add new features, do not touch pack-related asserts that already
pass — this is purely "add the protocol-binary sibling assertion next to the existing pack
assertion" per the repo rule "extend existing test files, never create new ones."

## Verification

`cargo test -p <FAMILY's package name(s)>` passes, including your new asserts. Confirm your
file(s) no longer appear in `POLICY_PROTOCOL_COMPLETENESS_ALLOWLIST`.

## Report back

Files touched, test results, confirmation of the allowlist edit.

---

## Family assignments (fill in from the actual allowlist before dispatch — the ~49 document kinds
group by technology directory, mirroring the pack rollout's wave-2 grouping): puzzle/{2d,3d,5d},
flow, cad, draw, note, writer, layout, forms, playbook (renamed), procedural, process, sourcing,
sequence, imperative, trinity, remodel, reasoning/mindmap, shooting, raster, lowpoly, animate,
norm/* (grouped, ~20 crates as one or a few agents), architect, fem, gis, mathematical, s, compose,
framework/editor, framework/renderer/wgpu, vcs/plugin. Adjust grouping to keep each agent's file
set non-overlapping and roughly balanced; it's fine for one agent to cover several small families.
