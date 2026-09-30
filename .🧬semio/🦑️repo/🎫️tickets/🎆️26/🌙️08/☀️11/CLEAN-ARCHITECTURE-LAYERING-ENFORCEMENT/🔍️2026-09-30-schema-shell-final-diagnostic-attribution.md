# Schema Shell Final Diagnostic Attribution

Read-only analysis of the actual `🗑️generated/schema-check-shell-final.log`. No schema/catalog/filter/baseline edits and no duplicate check execution.

The uncached workspace:schema-check exited 1. Its summary reports 4266 modules, 3650 scopes and 9354 findings. Independently parsing the diagnostic JSON lines yields exactly 9354 findings. This repository-wide run failed; selected-domain zero counts do not make the whole check green.

Matched actual diagnostic paths against these authored domain prefixes. Every listed domain has zero findings:

| Domain | Exact path mapping used | Findings |
|---|---|---:|
| IO route | `🧰️framework/🔨️modules/🚪️io/🧬️schema` | 0 |
| JS direction | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️dependency-direction` | 0 |
| Cargo direction | same library schema root, `🧱️cargo-dependency-direction` | 0 |
| S composition | `✏️s/🧑‍💻dev/🧩️composition-laws/🧬️schema` | 0 |
| S Flow composition | `✏️s/🧑‍💻dev/🌊️flow/🧬️schema` | 0 |
| Dev contribution | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution` | 0 |
| Shared policy | same OS modules root, `📇️directory/🛡️access-policy` | 0 |
| Deployment | same OS modules root, `🔌️plugin/📇️registry/📦️deployment` | 0 |
| Media | same OS modules root, `📺️renderer/🧑‍🎨engine/🎬️media/🧬️schema` | 0 |
| Translation | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧬️schema` | 0 |
| Session lifetime | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧬️schema` | 0 |
| Mesh modeling/cuts | `🧰️framework/🔨️modules/🧊️3d/🥽️mesh` | 0 |
| Playbook lifetime | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧬️schema` | 0 |
| Plugin retirement | OS modules root, `🔌️plugin/🧫️fixtures/🧩️extension-retirement` | 0 |
| Source freshness | OS modules root, `🔌️plugin/🏗️build/🔍️freshness` | 0 |
| Root inference law source | library schema root, `🧱️root-inference-law-source` | 0 |
| Inference opening | OS modules root, `💡️inference/🚪️opening/🧬️schema` | 0 |
| Neural registry retirement | OS modules root, `🧠️neural/⚙️engine/📔️registry` | 0 |
| Plugin inference/action collection | OS modules root, `🔌️plugin/🧬️schema/🧰️ui-action-collection` | 0 |
| BREP retained lifetime fixture | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧫️fixtures/🚪️retirement` | 0 |
| S fixture sweep | `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧬️schema` | 0 |
| Neutral fixture sweep | OS modules root, `🗣️dsl/🧹️fixture-sweep/🧬️schema` | 0 |

The catalog is separately diagnosed once, at log line 9290: code `schema-catalog-stale`, path `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️schema-catalog.json`, detail “The catalog does not match the schema modules on disk.” It is included in the 9354 total, not excluded or attributed as a selected authored-domain failure. Parent observed this during shared handoff and plans regeneration after the source settles; this review confirms the actual diagnostic, not its causal timing. No generic scope exclusion was introduced.

The selected mapping is diagnostic attribution, not a schema-check suppression rule. Persistent reports remain; the generated log's cleanup belongs to final ticket coordination.

## Final Canonical Execution Handoff Check

Root executed the uncached final schema check after catalog/launcher and schema generation settled. It failed in 21.9s: 4,266 modules, 3,650 scopes and 9,333 actual diagnostic JSON rows, parsed independently. All twenty-two prior authored mappings plus the new canonical execution mapping have zero findings. This is attribution, not suppression; the whole repository check remains failing.

| Domain | Exact Path Prefix | Findings |
| --- | --- | ---: |
| IO route | `🧰️framework/🔨️modules/🚪️io/🧬️schema` | 0 |
| JS direction | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️dependency-direction` | 0 |
| Cargo direction | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️cargo-dependency-direction` | 0 |
| S composition | `✏️s/🧑‍💻dev/🧩️composition-laws/🧬️schema` | 0 |
| S Flow composition | `✏️s/🧑‍💻dev/🌊️flow/🧬️schema` | 0 |
| Dev contribution | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution` | 0 |
| Shared policy | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🛡️access-policy` | 0 |
| Deployment | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment` | 0 |
| Media | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎬️media/🧬️schema` | 0 |
| Translation | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧬️schema` | 0 |
| Session lifetime | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧬️schema` | 0 |
| Mesh modeling/cuts | `🧰️framework/🔨️modules/🧊️3d/🥽️mesh` | 0 |
| Playbook lifetime | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧬️schema` | 0 |
| Plugin retirement | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧩️extension-retirement` | 0 |
| Source freshness | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🔍️freshness` | 0 |
| Root inference law source | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️root-inference-law-source` | 0 |
| Inference opening | `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🚪️opening/🧬️schema` | 0 |
| Neural registry retirement | `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry` | 0 |
| Plugin inference/action collection | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🧰️ui-action-collection` | 0 |
| BREP retained lifetime fixture | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧫️fixtures/🚪️retirement` | 0 |
| S fixture sweep | `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧬️schema` | 0 |
| Neutral fixture sweep | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧬️schema` | 0 |
| Canonical execution | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏛️canonical-execution` | 0 |

Catalog-stale diagnostics included in the total: 0. No causal timing inference is made from concurrent source changes. The final raw check output will be removed with ticket generated output; this exact summary remains.

## Actual Root Caller Final Refresh

After the process-option regression and reviewed constructor fingerprint update, schema catalog/docs generation passed in 1m13s and the plugin-registry byte freshness check passed in 26.6s. The subsequent uncached schema check failed: `[schema check] modules=4266 scopes=3650 findings=9332`. Independently parsed diagnostic rows: 9332. All 23 prior exact authored mappings remain zero. Catalog-stale count: 1. These counts include all diagnostics and do not make the global check green.

## Native Capture Schema Refresh

Actual schema generation/docs and registration freshness checks passed after the native capture change. The uncached global check exited 1 in 11.9s: [schema check] modules=4266 scopes=3650 findings=9336; independently parsed 9336 diagnostic JSON rows. Catalog-stale: 0. The 24 authored mappings below are attribution, not exclusions.

| Domain | Exact Path Prefix | Findings |
| --- | --- | ---: |
| IO route | `🧰️framework/🔨️modules/🚪️io/🧬️schema` | 0 |
| JS direction | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️dependency-direction` | 0 |
| Cargo direction | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️cargo-dependency-direction` | 0 |
| S composition | `✏️s/🧑‍💻dev/🧩️composition-laws/🧬️schema` | 0 |
| S Flow composition | `✏️s/🧑‍💻dev/🌊️flow/🧬️schema` | 0 |
| Dev contribution | `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🧩️contribution` | 0 |
| Shared policy | `🧰️framework/🛍️products/💻️os/🔨️modules/📇️directory/🛡️access-policy` | 0 |
| Deployment | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📇️registry/📦️deployment` | 0 |
| Media | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎬️media/🧬️schema` | 0 |
| Translation | `🧰️framework/🔨️modules/🖱️ui/🌐️i18n/🧬️schema` | 0 |
| Session lifetime | `✏️s/🔨️modules/🌐️spatial-kernel/⚙️engine/🧠️semio/🌊️session/🧬️schema` | 0 |
| Mesh modeling/cuts | `🧰️framework/🔨️modules/🧊️3d/🥽️mesh` | 0 |
| Playbook lifetime | `✏️s/🔌️plugins/📖️playbook/🧩️extensions/🌀️procedural/🧬️schema` | 0 |
| Plugin retirement | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧫️fixtures/🧩️extension-retirement` | 0 |
| Source freshness | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🏗️build/🔍️freshness` | 0 |
| Root inference law source | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🧱️root-inference-law-source` | 0 |
| Inference opening | `🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/🚪️opening/🧬️schema` | 0 |
| Neural registry retirement | `🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📔️registry` | 0 |
| Plugin inference/action collection | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧬️schema/🧰️ui-action-collection` | 0 |
| BREP retained lifetime fixture | `✏️s/🔌️plugins/🌊️flow/🧩️extensions/📐️brep/🧫️fixtures/🚪️retirement` | 0 |
| S fixture sweep | `✏️s/🧑‍💻dev/🧹️fixture-sweep/🧬️schema` | 0 |
| Neutral fixture sweep | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧹️fixture-sweep/🧬️schema` | 0 |
| Canonical execution | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🏛️canonical-execution` | 0 |
| Exact native admission | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧬️schema/🦀️exact-cargo-laws` | 0 |


## Preserved Harness and Launcher Refresh

Uncached schema generation/docs passed23.3s; whole-repository check failed11.8s with4,266modules,3,650scopes and9,336 independently parsed diagnostic JSON rows. Catalog-stale zero. All24 exact authored schema mappings above remain zero. The new required-bridge fixture belongs to the existing declared-verb test owner; this refresh introduces no whole-tree exclusion or baseline. Plugin catalog and launcher generation passed11.9s, independent check-generated20.9s.

## Owner-Context Intermediate Refresh

The actual uncached global check after the25-owner registry refresh failed with4,266modules,3,650scopes and9,341 independently parsed JSON diagnostics, including one catalog-stale finding. All24 previously mapped authored prefixes still have zero findings. The generation3d parent schema subtree contains42 findings; four specifically name new Generation3dIoAuthority missing-language exports. These four are root-owned and have been corrected by declaring its intended JSON-only source-test fixture format. The new Generation3dExportInputs and Generation3dRegistryTextRoundTrip fixture contracts already declare their supported JSON format; no findings name them in this run. The remaining parent-subtree findings are retained broader schema work. Final regeneration/check remains pending.

## Settled Prepared IO and Retained Profile Refresh

Actual uncached schema generation passed16.6s and docs passed11.3s, with3,650 scopes. The global check exited1 after11.5s. Independently parsed diagnostic JSON rows: 8911. Catalog-stale: 0. All24 existing exact authored-prefix mappings remain zero. New JSON-only source fixture contracts have zero named/path findings: Generation3dIoAuthority=0, Generation3dRegistryTextRoundTrip=0, Generation3dExportInputs=0, Retained Extrusion Fixture=0. The broader generation3d parent schema retains38 findings; they are not suppressed. These counts concern diagnostic attribution, and the whole-repository check is still failing.

## Final Settled Source Check

After the clean app source and all five final guest producers settled, plugin catalog regeneration passed in21.1s and byte freshness verification in22.0s. The final uncached schema check again exited1: 8911 independently parsed findings, catalog-stale 0. All24 exact authored prefixes and four new fixture contracts remain zero. Broader generation3d parent findings: 38. No catalog or schema filtering was introduced.
