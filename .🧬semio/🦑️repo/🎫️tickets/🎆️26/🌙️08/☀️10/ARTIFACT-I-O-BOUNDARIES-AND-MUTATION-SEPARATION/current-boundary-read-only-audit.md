# Current Boundary Audit — 2026-10-08

Read-only inspection; no production edits or Git mutations. This is a live shared-tree observation, not a complete execution closure.

## Current Execution

Registered `NX_DAEMON=false bun nx run @semio-tech/repo-lib:lint-artifact-io-ownership --skip-nx-cache`, with isolated ticket `nx-boundary-recovery` workspace data, ran once and exited 1. The guard completed 30,387 checkpoints; Nx reported 18.5 seconds for its task. Startup was observed with live Bun/Nx handles before completion. Evidence is `🗑️generated/ownership-current-audit.log`.

The guard reported these current findings:

- `artifact-io/schema-codec-dependency: ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️catalogue/🦀️.rs`
- `artifact-io/schema-codec-dependency: ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🛰️service/🦀️.rs`
- `artifact-io/schema-codec-dependency: ✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/📐️geometry/🦀️.rs`
- `artifact-io/schema-codec-dependency: ✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- `artifact-io/schema-codec-dependency: ✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs`
- `artifact-io/schema-codec-dependency: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs`
- `artifact-io/schema-codec-dependency: ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🦀️.rs`
- `artifact-io/facet-path: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/1️⃣cc1/🚪️io/🧬️mutations`
- `artifact-io/facet-path: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/5️⃣cc5/🚪️io/🧬️mutations`
- `artifact-io/facet-path: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/4️⃣cc4/🚪️io/🧬️mutations`
- `artifact-io/facet-path: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/3️⃣cc3/🚪️io/🧬️mutations`
- `artifact-io/facet-path: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/6️⃣cc6/🚪️io/🧬️mutations`
- `artifact-io/facet-path: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📐️step/🏅️standards/🔖️ap214/🪆️subsets/2️⃣cc2/🚪️io/🧬️mutations`
- `artifact-io/schema-codec-dependency: ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs`
- `artifact-io/schema-source: ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🖐️5d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `artifact-io/schema-source: ✏️s/🔌️plugins/🧩️puzzle/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🟦️.ts`
- `artifact-io/schema-codec-dependency: ✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🦀️.rs[0m`

## Prioritized Architectural Findings

1. **Semantic owners still execute physical codecs.** Procedural generation3d catalogue imports JSON parsing from `semio_framework_pack_json`; Architect program inference calls CSV and TSV encoders; PDF semantic diff imports the IO-root `carry_graph_edit`. These are actual executable dependencies, not merely spelling in comments. Move native admission/emission and any genuinely semantic graph operation into the correct owners instead of adding barrel aliases. Sources: [✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️catalogue/🦀️.rs:6](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🗂️catalogue/🦀️.rs:6), [✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:2001](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏛️architect/🗿️artifacts/🏛️program/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🦀️.rs:2001), [✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs:19](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base/🧬️schema/🔺️diff/🦀️.rs:19).

2. **Guard coverage cannot establish complete root or physical purity.** Owner-root Rust files only receive a schema-to-IO mount check; owner-root TypeScript files receive no check. Rust IO files get trait-name and semantic-function-name checks; TypeScript IO files get none. Rust semantic dependency detection is a finite token-name/string pattern set, not a resolved import graph. Representation matching checks codec trait implementations, not cross-representation imports. Sources: [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🟦️.ts:57](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🟦️.ts:57), [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🟦️.ts:198](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🟦️.ts:198), [🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🟦️.ts:213](/Users/ueli/Documents/semio/🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🚪️io/🏛️architecture/🟦️.ts:213).

3. **Concrete hidden facade and physical dependency examples remain outside that coverage.** ISO16757 root TypeScript reexports SQLite codecs; its Rust root publishes builder/analyzer/composer APIs from the IO root. Root facades may be intentional public surface, but they require explicit taxonomy policy and cannot be covered by the present root-mount-only rule. Its binary snapshot imports the text snapshot with a glob, even though the binary body delegates to ArtifactPack; this unnecessary physical dependency is not checked by the representation guard. Sources: [✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🟦️.ts:3](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🟦️.ts:3), [✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🦀️.rs:1720](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🦀️.rs:1720), [✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs:34](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📕️norm/🗿️artifacts/📇️iso16757/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/💾️binary/📸️snapshot/🦀️.rs:34).

4. **Six STEP AP214 subsets retain incorrect IO paths.** Mutation facets are directly beneath IO rather than beneath a declared physical representation; the fresh guard identifies cc1 through cc6. These path findings also short-circuit recursive inspection of the forbidden directory, so deeper purity is not certified.

5. **Rust derive hooks still couple semantic lifecycle to SQLite owner.** Layout snapshot has `retire_with` routed into `io::sqlite::snapshot::retire`; this is detected today and needs a semantic owner, not an exemption. Source: [✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:18](/Users/ueli/Documents/semio/✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🦀️.rs:18).

## Complementary Execution Notes

Existing source matrix ended RED: 16 of 48 source targets failed. FEM2D and FEM3D have stale assertions demanding SQLite codec functions on semantic snapshot modules even though their tests already import those codec functions directly from IO. The FEM2D log shows 14 functional checks passing and only this namespace assertion failing. Forms and CAD have additional functional failures and require separate diagnosis; they must not be dismissed as namespace-only. Source: [✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:38](/Users/ueli/Documents/semio/✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🟦️.ts:38). Evidence: `🗑️generated/complementary-source-current.log`, lines 495–538 and final failed-task list.

Existing native matrix ended RED: 44 of 48 native targets failed. Final kernel errors are three private apply-batch methods (`stage`, `adopt`, `abort`) and two `u32`/`usize` mismatches, not evidence of native codec semantic failure. They may reflect concurrent store extraction; root owns recovery. Evidence: `🗑️generated/complementary-native-current.log`, lines 920142–920197. No native success is claimed from this failed matrix.

No fixes, reruns, ticket closure, or goal closure were performed by this audit.
