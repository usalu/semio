# Norm Plugin End-to-End Audit

Date: 2026-11-04. Read-only implementation audit. No compilation or full test execution was performed.

## Confirmed Test Entry Blockers

The @semio-tech/norm-js:test target calls runTestScript in 🧪️tests/🔬️typescript-package-commands/🟦️.ts.

1. The runner unconditionally reads the plugin-root 🔣️.json app descriptor. That file does not exist. The retained-command-dispositions fixture exists, so the next descriptor read must fail with ENOENT before routes or examples run.
2. The runner unconditionally appends 🧪️tests/🪞️rust-parity/🟦️.ts to Bun test paths. That file is also absent. Repair both paths together.
3. Descriptor classification is fail-open: undefined interactiveJob skips classification, and zero classified rows only warns. Require every expected action to carry its expected migrated admission classification.

## Documented End-to-End Gaps

The shared README records that integrated editors and clause libraries were not compiled or run as a whole as of the 2026-11-02 audits. It states 520 NDP entries and zero verified values; this count was read from documentation, not recomputed.

The README documents unordered persisted selection versus picks, clearing picks leaving stored checks, read-only viewer picks not mirrored, multi-field drags committing separate setField commands, missing insertion ghosts and numeric dimension editing, missing inspector tiers and conditional visibility, scene positions absent from documents, incomplete configuration persistence, and absent per-station utilisation colours.

Artifact READMEs document further reachability gaps: EN 1990 catalogue-only reliability/test-series inputs; EN 1991 96-entry catalogue not reachable from a window; EN 1992 missing explicit bar/tendon/polygon fields; EN 1993 frame/joint/support wiring gaps; EN 1994 equivalent-tube geometry for encased columns; EN 1996 missing confined/prestressed entities; EN 1997 only 14 editable paths with 55 leaves not editable; EN 1998 missing capacity-design/modal/non-structural inputs; EN 1999 missing stiffener/torsion/local-load fields and exact tables. EN 1995 documentation contradicts itself: the introduction says catalogue cases cannot be created, while its Not covered section says cases are editable document entities. Reconcile with executed tests.

## Test Architecture Present

Schema-sync, wire-twin, mutation-label census, field-meta choices, entity deletion and document-contract test areas exist. The wire-twin command claims generated twin comparison with strict Ajv. The oracle contribution declares test-only independent Python libraries including structuralcodes, concreteproperties, sectionproperties, blue-prints, SymPy, groundhog, PyNiteFEA, anastruct and SciPy, numerical tolerance profiles and productionReachable=false. Declarations do not prove suite execution or independent coverage of every clause.

## Recommended Order

Repair the missing test-entry paths and require route classification, then run the entry target. Compile integrated artifacts and bridge, execute examples and language-neutral/twin/oracle witnesses, and replace historical not-run statements with successful run evidence. Track model reachability separately from clause-library existence and normative-value verification.

## Tooling Evidence

Active SEMIO_AGENT_SESSION plus repo tools exec --light bun permits inspection. The equivalent node invocation exits successfully with no output on this host, so its exit code alone does not prove execution. No implementation source changed.
