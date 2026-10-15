# Canonical BREP Owner Reconciliation

The former `semio-framework-3d-brep-operations` package no longer exists. Its actual physical manifest declares `semio-framework-3d-brep-construction`; boolean, sweep and offset have independent existing manifests and production Source. No package alias or forwarding module was introduced.

| Former module | Actual package | Public namespace |
| --- | --- | --- |
| euler, primitives, staged, sew, transform | semio-framework-3d-brep-construction | Original module name |
| boolean | semio-framework-3d-brep-boolean | Crate root |
| sweep | semio-framework-3d-brep-sweep | Crate root |
| offset | semio-framework-3d-brep-offset | Crate root |
| blend | semio-framework-3d-brep-offset | blend |

All 26 authored Rust files containing the old namespace were reconciled with these physical owners. This includes engine operations and mesh import, query tests, generation3d analysis/curve/intersection/boolean facets, STEP geometry, Semio BREP tests and Flow BREP. Documentation links now name the construction owner too. An independent repository-wide `rg` across authored Rust/Cargo/owner metadata subsequently returned no old namespace or package references.

The generation3d, STEP and Semio workspace dependency tables now declare the actual four packages. Generation3d analysis-check and brep-boolean use the boolean package directly; brep-curve and brep-intersect use construction. Its unused root package dependency was removed. Flow's direct boolean dependency points to the actual boolean manifest. Engine declares its three independently consumed boolean/sweep/offset production dependencies. Tessellation's boolean test dependency is explicit. STEP geometry declares construction directly. Semio BREP's four operation dependencies are scoped to its actual test consumers. The owner registry now names construction and includes the three real split leaves.

## Evidence

The private `brep-owners` script has a closed language-neutral nine-case corpus and schema. It checks module ownership against actual physical library entry files, independent SQLite primary-key/query behavior, Bun TOML against `@iarna/toml`, and every physical BREP dependency manifest identity. The initial actual Nx prototype gate was genuinely RED: 1 pass / 1 fail, 1812 assertions, with stale dependency/import/registry facts reported before production edits.

The first successor's assertions all passed, but its broad 30,318-file Source read took 6419 ms and exceeded Bun's unchanged 5000 ms test deadline. That attempt is a failure, not a behavior qualification. Source inspection is now constrained to the five explicit physical BREP consumer roots in the corpus; all 1783 authored manifests remain independently parsed. No deadline or capacity was increased.

Ingress's independent Original 51043 completed the real normal Nx → private script → owned offline Cargo `generate-lockfile` route for generation3d after the correction: exit 0, 809 resolved packages, roughly 7 seconds of task runtime. Its retained output is `🗑️generated/job-brep-lock.log`. The predecessor had rejected the nonexistent operations package. No compiler ran in this metadata/lock proof, and no production compilation result is claimed here. Lock ownership remained with Ingress; this lane did not edit lock files.

The final actual prototype and release successors both passed 2 tests / 1813 assertions (52.48 / 51.46 seconds end to end). Each independently parsed 1783 manifests, checked 68 actual physical BREP dependency edges and inspected 3215 Rust files under the five explicit Source roots. The assertions themselves took roughly 0.8 seconds; broad authored manifest discovery/parse setup remains roughly 51 seconds, so no fast test-inventory claim is made. The focused ES2022 strict/noUncheckedIndexedAccess gate passed in 947 ms. All Original handles are terminal.

The ownership and strict gates are registered in the ticket headroom Script/project and both prototype/release editor launch catalogs. All implementation changes are Rust/Cargo/owner metadata or ticket-private tests and dispatch; shared compiler, toolkit, graph runtime, Nx patch and SDK activation are unchanged. SDK/ordinary Rust pair ownership remains with this lane after Storage's coherent daemon checkpoint.
