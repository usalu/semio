# Existing Owner Dialect Witnesses

A bounded read-only scan inspected up to32 nearby owner test JSON files per excluded schema, each at most1MiB, at most3000 nested values/file and depth8. Current exact schema SHA was checked against the proof cohort before loading. Ajv2020 accepted 35 authentic existing fixture values for 35 of52 unproved targets across44 schemas with coverage. The separate excluded schema lacking coverage was not scanned. No Source payloads were copied; generated/schema-dialect-owner-witnesses.json contains only paths, hashes, target pointers, witness pointers and scan counts.

These establish positive witnesses under the current dialect only. Root must add the references to its SHA-bound dual-dialect proof and recheck both validators, per-export nonvacuity and all negative vectors before treating any candidate as eligible. Fixture membership under a refusal scenario is not itself a rejection: a nested Stamp or Hex may satisfy its local target while its surrounding scenario is intentionally refused.

| Schema | Target | Existing Data | Data Pointer |
|---|---|---|---|
| ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪛️utilities/🎬️actions/🧬️schema/🔣️.json | root | ✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🪛️utilities/🎬️actions/🧪️tests/🧫️fixtures/🔣️.json | root |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/📸️snapshot/🔣️.json | root | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧪️tests/intrinsic.json | /valid |
| ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧬️schema/🔣️.json | root | ✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📚️idml/🧪️tests/intrinsic.json | /valid |
| 🧰️framework/🔨️modules/⏯️tool-run/♻️retirement/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/⏯️tool-run/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/📊️progress/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🛬️receive/📊️progress/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧬️schema/🔣️.json | /definitions/Corpus | 🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧪️tests/🔣️.json | root |
| 🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧬️schema/🔣️.json | /definitions/Memory | 🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧪️tests/🔣️.json | /memory/0 |
| 🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧬️schema/🔣️.json | /definitions/View | 🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🧪️tests/🔣️.json | /view |
| 🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/🌱️value/✨️derive/🧵️canonical/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🔨️modules/🌱️value/📣️publication/🌳️root/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/🌱️value/📣️publication/🌳️root/🧪️tests/🔣️.json | root |
| 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🎟️demand/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/🌱️value/🧬️retained-clone/♻️close/🎟️demand/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🔨️modules/🌱️value/🫴️receiving/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/🌱️value/🫴️receiving/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🔨️modules/🎒️pack/📐️format/🏷️symbols/🎮️decode/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/🎒️pack/📐️format/🏷️symbols/🎮️decode/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🔬️fingerprint/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🔬️fingerprint/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🎮️management/🕰️history/🧬️schema/📈️paging.json | root | 🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🎮️management/🕰️history/🧪️tests/📈️paging/🔣️.json | root |
| 🧰️framework/🔨️modules/📡️replication/⚙️codec/🛬️retained/📦️bytes/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/📡️replication/⚙️codec/🛬️retained/📦️bytes/🧪️tests/🔣️.json | root |
| 🧰️framework/🔨️modules/📡️replication/🚪️io/💾️binary/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/📡️replication/🚪️io/💾️binary/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🔨️modules/📡️replication/🚪️io/📥️bootstrap/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/📡️replication/🧪️tests/🧫️fixtures/🚀️artifact-bootstrap/🔣️.json | root |
| 🧰️framework/🔨️modules/📡️replication/🚪️io/🧬️schema/🏛️ownership/🔣️.json | root | 🧰️framework/🔨️modules/📡️replication/🚪️io/🧪️tests/🧫️fixtures/🏛️ownership/🔣️.json | root |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/🔗️binding/🧬️schema/🔣️.json | root | 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️prepared/🔗️binding/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🧬️schema/🔣️.json | root | 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/📜️history/🛬️decode/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧬️schema/🪪️document-identity/🔣️.json | root | 🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🧫️fixtures/🪪️document-identity-wire/🔣️.json | /cases/0/identity |
| 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧬️schema/🔣️gumball-component-events/🔣️.json | root | 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🌐️World3dHost/🧪️tests/🧫️fixtures/🛠️gumball-component-events.json | root |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/💼️job/📤️receipt/🧬️schema/🔣️.json | root | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🎟️context/🌉️call/💼️job/📤️receipt/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization/♻️retirement/🧬️schema/🔣️.json | root | 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌱️initialization/♻️retirement/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧳️retained/🧬️schema/🔣️.json | root | 🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🧳️retained/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⌨️usage/🧬️schema/🔣️.json | root | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/⌨️usage/🧪️tests/🔣️.json | root |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎛️control-policy/🧩️products/🧬️schema/🔣️.json | root | 🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎛️control-policy/🧩️products/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧬️schema/🔣️.json | /definitions/Slot | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🧫️fixtures/🔣️.json | /ranks/0/slots/0 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧬️schema/🔣️.json | /definitions/Key | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🧫️fixtures/🔣️.json | /ranks/0/key |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧬️schema/🔣️.json | /definitions/SlotState | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔧️tools/🏗️bazel/🧵️sessions/🧪️tests/🧫️fixtures/🔣️.json | /ranks/1/states/s0 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔭️coverage/🧬️schema/🔣️.json | /definitions/Stamp | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🔭️coverage/🧪️tests/🔣️.json | /refusals/4/stamp |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧬️schema/🔣️.json | /definitions/Hex | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧪️tests/🌊️stream/🔣️.json | /owners/7/node/sha256 |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧬️schema/🔣️.json | root | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🕸️dependencies/🧭️direction/🏗️construction/🧪️tests/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📤️arguments/🧬️schema/🔣️.json | root | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/📤️arguments/🧪️tests/🧫️fixtures/🔣️.json | /accepted/0 |

## Extended Authored Data Scan

Current proof: 287 eligible,16 excluded,18,219 observations with35 authored references. Review found no blocker in authored pointer decoding (~1 before ~0), empty-root handling, matching candidate SHA, fixture SHA, ordinary single-link bounded file checks, or explicit acceptance by both validators. Predicate vocabulary and exclusions remain unchanged. This is finite comparison evidence, not exhaustive instance proof.

A second bounded scan searched whole owning subtrees rather than test folders alone: maximum128 JSON files/schema,4MiB/file,10,000 values/file, depth12. Six additional existing values passed the current Ajv2020 target. Exact path/hash references are in generated/schema-dialect-extra-witnesses.json; no source values were copied. Root must dual-validate these references before expanding eligibility.

| Schema | Target | Existing Data | Pointer |
|---|---|---|---|
| ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/🧬️schema/🔣️.json | root | ✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/⚙️engine/🎬️scene/🔤️text-fonts/📦️catalog/🔣️.json | root |
| 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/↩️line-break/📦️data/🔣️provenance.json | root |
| 🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🎮️management/🔏️fingerprint/🧬️schema/🔣️.json | root | 🧰️framework/🔨️modules/📁️filesystem/📝️workspace/🎮️management/🔏️fingerprint/🧫️fixtures/🔣️.json | root |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🪁️typecheck/🧬️schema/🎯️project.json | root | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🪁️typecheck/🧫️fixtures/🎯️project.json | /accepted/0/selection |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🪁️typecheck/🧬️schema/🎯️units.json | root | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🪁️typecheck/🧫️fixtures/🎯️units.json | /accepted/0/selection |
| 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🧬️schema/🔣️.json | /definitions/Claim | 🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🧪️tests/🔣️.json | /refusals/9/claim |

No impossible schema was proved among the unresolved targets. Several describe runtime-produced custody claims, manifests, pins, archives or run receipts; lack of a matching checked-in JSON witness is not evidence that those schemas are unsatisfiable. Energy birth consumes x-semio-energy-row-birth authored metadata from enclosing row schemas, rather than a local test corpus. Wider enclosing-owner lookup or exact references from existing tests are the appropriate next step; inventing a minimal value merely to force eligibility is not recommended.
