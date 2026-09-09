# Native Taxonomy Compile-Path Audit

Snapshot: 2026-09-09.

## Scope

Read-only preventive audit for the remaining default/parent package gates in `🗑️generated/native-library-recovery-plan.json` and the current recovery TSV. The accepted leaf gates `semio-s-artifact-process-process3d` and `semio-s-artifact-vcs-vcs` were excluded; 37 plan gates remained in scope.

The source inventory covers outstanding owner plugins, every non-Semio stdio artifact owner, all multi-artifact plugin owners, and the framework artifact owners used by those packages: Flow, Infinite DAG, Playbook, Space/Collection, and Workflow/Run. It excludes both Semio trees (`🧬️semio` and `🧿️semio`), generated/build directories, targets, node modules, and test source directories. Test modules mounted from a production source are classified by the resolver, but do not create an actionable default `cargo check` result.

## Result

No concrete moved-fixture or module reference remains in this scope. The scan resolved all 13,347 literal references from 3,544 Rust sources:

| Reference | Resolved | Missing |
| --- | ---: | ---: |
| `#[path = "…"] mod …;` | 9,301 | 0 |
| `include_str!("…")` | 3,983 | 0 |
| `include_bytes!("…")` | 63 | 0 |

The resolver ignores comments and string contents. It resolves `#[path]` external modules against the active Rust module directory and propagates an inline module's path directory, including the common `#[path = "."]` form. Literal include macros resolve from their physical containing source file. This avoids treating an inline module path as an external-file reference or using the enclosing file directory after an inline path reset.

No counterpart candidate is reported because there is no missing target. The remaining native failures/retries should be diagnosed from their emitted compiler or graph evidence, rather than preemptively changing taxonomy references.

## Evidence and Limit

The bounded receipt is [native-taxonomy-reference-audit.json](🗑️generated/native-taxonomy-reference-audit.json); it records the owner directories, plan package set, counts, exclusions, and empty missing-reference list. Its fragment receipts retain the raw per-source resolution output.

No Cargo, Nx, formatter, source change, or runtime test was intentionally run. During report generation, an unquoted shell heredoc accidentally invoked an unscoped `cargo check`; it was terminated after about 25 seconds without a terminal result and is not acceptance evidence. This static audit does not evaluate computed macro paths.

## Test-Tree Extension

The production census above deliberately excluded `🧪️tests` sources. At the coordinator's direction, I extended the same literal resolver to the scoped test tree. This is a separate static capture rather than a Cargo result: 2,932 Rust test sources carried 15,125 literal references, of which 204 were absent at capture time.

| Reference | Resolved at capture | Missing at capture |
| --- | ---: | ---: |
| `#[path = "…"] mod …;` | 664 | 24 |
| `include_str!("…")` | 14,172 | 180 |
| `include_bytes!("…")` | 85 | 0 |
| Total | 14,921 | 204 |

All 204 missing targets have an existing canonical counterpart confirmed from the physical source-file parent:

- Mathematical Equation: 66 stale fixture literals in three Any-subset tests. The canonical remaps were proven in [native-taxonomy-equation-fixture-remap.json](🗑️generated/native-taxonomy-equation-fixture-remap.json), then repaired and checked by the coordinator.
- Lowpoly: 85 stale fixture literals in 17 Any-subset tests. The target is the same mutation/case tail under `🧫️fixtures/🧬️mutations`, reached with five ascents from each physical test-file parent. GIS owns this repair.
- Energy Model: 28 BESTEST asset literals in 14 test modules. The target is `../../../../🖼️assets/🏛️bestest-<case>/🗣️.dsl.semio`, with four ascents from the physical test-file parent. A three-ascent candidate was rejected before source modification; the coordinator owns the corrected remap.
- Norm: 23 `#[path]` registrations across EN1997, EN1991, EN1994, EN1998, EN1999, and EN1995 omit the `🧪️tests` path component before their case directory. The existing test module is the counterpart in every case. Norm received the exact case list.
- stdio IFC: one equivalent missing `🧪️tests` component for the set-snapshot fixture module.
- Note: one window test literal needs four ascents to the existing subset-level `🧫️fixtures/🔣️.json`.

The test receipt is [native-taxonomy-test-reference-audit.json](🗑️generated/native-taxonomy-test-reference-audit.json). It records every source, literal, resolved missing target, and a physically resolved counterpart candidate. Its eight fragment receipts are the raw scanner outputs. Semio remains excluded. This captures static references only and does not establish a native compile or test result.

Receipt update: 2026-09-09T13:11:16+02:00. Individual capture timestamps were not measured; the eight test fragments were written from 2026-09-09T13:01:02+02:00 through 2026-09-09T13:07:51+02:00.

### Narrow Live Recheck

Completed at 2026-09-09T13:11:43+02:00, the resolver rechecked the 42 unique source files responsible for the initial 204 missing targets. Of 352 current literal references in those files, 23 remain unresolved and all 23 are the notified Norm `#[path]` omissions. The original Equation, Lowpoly, Energy, IFC, and Note targets now resolve. This is a narrow source-level confirmation after concurrent repairs, not a full second test-tree census and not a Cargo or test result. Evidence: [native-taxonomy-reference-current-affected-audit.json](🗑️generated/native-taxonomy-reference-current-affected-audit.json).
