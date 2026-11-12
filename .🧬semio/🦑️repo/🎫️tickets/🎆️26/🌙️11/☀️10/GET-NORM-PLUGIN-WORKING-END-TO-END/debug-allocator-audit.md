# 🐞️ Debug and Allocator Audit (audit3)

Read-only audit of the repo after the upstream merge (commits 737–740). No source edited, no builds run. Raw outputs: `🗑️generated/audit3-*`.

## 📊️ Counts

| Check | Count |
|---|---|
| `[DEBUG]` hits in .rs/.ts/.go/.py/.sh (excluding .cache, node_modules, target, ticket tree, .venv) | 51 |
| - temporary log, test scope | 32 |
| - temporary log in production (non-test) code | 2 |
| - contract string (tests or prod code depend on the tag) | 6 |
| - legitimate (tooling, tests asserting absence) | 10 |
| - third-party (`.venv`, fontTools docstring) | 1 |
| `#[global_allocator]` declarations (.rs) | 126 |
| - (a) testing allocator own declaration | 1 |
| - (b) production binary or allocator crate | 0 |
| - (c) orphan, not mounted by any Cargo target | 4 |
| - (d) duplicate test allocator (remove in favour of `semio-framework-value-testing-allocator`) | 121 |
| `#[path]` includes of an allocator file | 5 |

Caveat: (b) is 0 because every declaration is `#[cfg(test)]`-gated, sits in a test target, or is a lib root of a `tests`/`testing` crate. Production-path code has no allocator declaration.

## 1. `[DEBUG]` hits

| # | file:line | class | verdict / reason |
|---|---|---|---|
| 1 | `.venv/lib/python3.14/site-packages/fontTools/misc/loggingTools.py:49` | third-party | vendored fontTools docstring example, not ours |
| 2 | `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py:186` | temporary-log | print/stream [DEBUG] line in teaching tooling script (not framework); review |
| 3 | `🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/📚️collection/🐍️.py:200` | temporary-log | print/stream [DEBUG] line in teaching tooling script (not framework); review |
| 4 | `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🚶️circulation/🐍️.py:479` | temporary-log | print [DEBUG] in teaching test/fixture |
| 5 | `🎓️teaching/🏛️architecture/🏢️nakagin/🧪️tests/🪟️unit/🐍️.py:107` | temporary-log | print [DEBUG] in teaching test/fixture |
| 6 | `🎓️teaching/🏛️architecture/🦗️grasshopper/🎬️clip/🧪️tests/🎭️presentation/🐍️.py:200` | temporary-log | print [DEBUG] in teaching test/fixture |
| 7 | `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔗️consistency/🧪️tests/🔄️lifecycle/🐍️.py:37` | temporary-log | print [DEBUG] in teaching test/fixture |
| 8 | `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🔗️consistency/🧪️tests/🔄️lifecycle/🐍️.py:112` | temporary-log | print [DEBUG] in teaching test/fixture |
| 9 | `🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧪️tests/🎭️presentation/🐍️.py:69` | temporary-log | print [DEBUG] in teaching test/fixture |
| 10 | `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts:64` | legitimate | test asserting leftover tag is absent / fixture input |
| 11 | `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧪️deploy/🌐️sealed-publication-and-bundle/🟦️.ts:64` | legitimate | test asserting leftover tag is absent / fixture input |
| 12 | `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🌐️site/🟦️.ts:107` | legitimate | deploy check that rejects leftover tag |
| 13 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️supersession/🧪️tests/🦀️.rs:10` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 14 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🔁️replay/🧬️supersession/🧪️tests/🦀️.rs:13` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 15 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🧵️work/🧪️tests/🦀️.rs:31` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 16 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🖥️host/📤️publication/🧵️work/🧪️tests/🦀️.rs:65` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 17 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️testing/🧪️artifact-app-laws/🏗️registered-app-construction-and-typed-operation-settlement/🦀️.rs:303` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 18 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🛬️receiving/📥️pack/🧪️tests/🦀️.rs:9` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 19 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎟️birth/🧪️tests/🦀️.rs:60` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 20 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎟️birth/🧪️tests/🦀️.rs:60` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 21 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👷️job/🎟️birth/🧪️tests/🦀️.rs:60` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 22 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/♻️cancellation/🦀️.rs:282` | temporary-log-PRODUCTION | Drop assert message with force_capture backtrace in production cancellation lease |
| 23 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⏪️time-travel/🧵️instance/📖️ref-reads/🦀️.rs:44` | temporary-log-PRODUCTION | eprintln [DEBUG] in non-test production code (time-travel ref-reads) |
| 24 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs:32` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 25 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🎟️registration/🧪️tests/🦀️.rs:32` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 26 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/🌱️initialization/🫧️ephemeral/🧪️tests/🦀️.rs:9` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 27 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🌱️process-arena/🦀️.rs:32` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 28 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🌱️process-arena/🦀️.rs:32` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 29 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:152` | legitimate | census tooling that detects the tag |
| 30 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:155` | legitimate | census tooling that detects the tag |
| 31 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:157` | legitimate | census tooling that detects the tag |
| 32 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:163` | legitimate | census tooling that detects the tag |
| 33 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:168` | legitimate | census tooling that detects the tag |
| 34 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/🎯️acceptance/📋️orchestration/📝️docstring-and-debug-census/🟦️.ts:182` | legitimate | census tooling that detects the tag |
| 35 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts:336` | contract-string | parses '[DEBUG] Fixed UDRW image' producer line in prod code; debug-tagged protocol |
| 36 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🟦️.ts:336` | contract-string | parses '[DEBUG] Fixed UDRW image' producer line in prod code; debug-tagged protocol |
| 37 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts:60` | temporary-log | console.log [DEBUG] graph-root trace in test |
| 38 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts:60` | temporary-log | console.log [DEBUG] graph-root trace in test |
| 39 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts:60` | temporary-log | console.log [DEBUG] graph-root trace in test |
| 40 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🔒️enforcement/🔌️nx/📦️graph/🧪️tests/🎛️operation/🟦️.ts:63` | temporary-log | console.log [DEBUG] graph-root trace in test |
| 41 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/🟦️.ts:21` | contract-string | tag is a stderr contract asserted by test; still debug-tagged, review |
| 42 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/🟦️.ts:23` | contract-string | tag is a stderr contract asserted by test; still debug-tagged, review |
| 43 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/🟦️.ts:23` | contract-string | tag is a stderr contract asserted by test; still debug-tagged, review |
| 44 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🗂️workspaces/🦀️cargo/🛠️preparation/🧾️custody/🧪️tests/🌊️output/👷️child/📜️script.ts:9` | contract-string | tag is a stderr contract asserted by test; still debug-tagged, review |
| 45 | `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏃️process/🦀️cargo/📥️dependencies/🧪️tests/📥️consumption/🟦️.ts:41` | legitimate | test asserting source does not contain the tag |
| 46 | `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️differential/🦀️.rs:74` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 47 | `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️differential/🦀️.rs:75` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 48 | `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️differential/🦀️.rs:76` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 49 | `🧰️framework/🔨️modules/🧊️3d/📐️brep/🛠️operations/🔀️boolean/🧪️tests/🔬️differential/🦀️.rs:83` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 50 | `🧰️framework/🔨️modules/🌱️value/🛬️decode/🏭️construction/📦️effect/🧪️tests/retained-outcome.rs:85` | temporary-log | [DEBUG] eprintln/assert message in test code |
| 51 | `🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🧪️tests/original-loan.rs:116` | temporary-log | [DEBUG] eprintln/assert message in test code |

Priority: the two `temporary-log-PRODUCTION` rows are real leftovers in shipped code (`ref-reads/🦀️.rs:44` eprintln; `♻️cancellation/🦀️.rs:282` Drop assertion with `force_capture` backtrace). Remove both. The `contract-string` rows are tests or prod code that depend on the tag; decide whether to keep the tag before removing it. The rest of the test-scope `[DEBUG]` lines should be removed by their owners.

## 2. `#[global_allocator]` declarations

| # | file (repo-relative) | line | class | reason |
|---|---|---|---|---|
| 1 | `✏️s/🧑‍💻dev/🧩️composition/🧪️tests/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/🔬️unit/🦀️.rs` | 316 | (c) | no Cargo target mounts it |
| 2 | `✏️s/🔌️plugins/🪵️sourcing/🗿️artifacts/🗂️curation/🏅️standards/🔖️1/🪆️subsets/✳️any/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/💰️backing/🦀️.rs` | 12 | (d) | chain-gated test-path; crate=semio-s-artifact-sourcing-curation kinds=lib |
| 3 | `✏️s/🔌️plugins/🔱️trinity/🗿️artifacts/🔌️jack/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/💡️inferences/🧪️tests/🪆️query/🦀️.rs` | 7 | (d) | chain-gated test-path; crate=semio-s-artifact-trinity-jack-graph-query kinds=lib |
| 4 | `✏️s/🔌️plugins/🧱️block/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/🧪️tests/⚖️retirement-law/🦀️.rs` | 2 | (c) | no Cargo target mounts it |
| 5 | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🔨️modules/🏠️host/🧰️owned/🧪️tests/🔬️retained-mutation-authority/🧬️snapshot-laws/🦀️.rs` | 271 | (d) | test-path; crate=semio-s-artifact-draw-drawing-host-tests kinds=lib |
| 6 | `✏️s/🔌️plugins/🖍️draw/🗿️artifacts/🖍️drawing/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🎬️scene/🎨️paint/🧪️tests/🖊️stroke/🦀️.rs` | 10 | (d) | chain-gated test-path; crate=semio-s-artifact-draw-drawing-scene-paint kinds=lib |
| 7 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/🔬️membership/🦀️.rs` | 5 | (d) | test-path; crate=semio-s-artifact-energy-model-mut-membership-tests kinds=lib |
| 8 | `✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🧪️tests/🦀️.rs` | 16 | (d) | test-path; crate=semio-s-artifact-energy-model-snapshot-tests kinds=lib |
| 9 | `✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/🦀️.rs` | 53 | (d) | cfg(test)-gated; crate=semio-s-plugin-energy-engine-precompute kinds=lib |
| 10 | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/📦️packages/🦀️rust/🦀️.rs` | 7 | (d) | cfg(test)-gated; crate=semio-s-artifact-remodel-remodeling-model-diff kinds=lib |
| 11 | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🧪️tests/⚙️retained-params/🦀️.rs` | 5 | (c) | no Cargo target mounts it |
| 12 | `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🪙️params-2/🦀️.rs` | 7 | (d) | cfg(test)-gated; crate=semio-s-artifact-remodel-remodeling-mut-params-2 kinds=lib |
| 13 | `✏️s/🔌️plugins/🗄️stdio/📇️registry/🧬️contract/✏️editing/🦀️.rs` | 4 | (d) | cfg(test)-gated; crate=semio-s-artifact-stdio-contract-editing kinds=lib |
| 14 | `🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🦀️.rs` | 34 | (d) | cfg(test)-gated; crate=semio-os-kernel kinds=lib |
| 15 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/🚪️native-io/📋️rows/📦️packages/🦀️rust/🦀️.rs` | 3 | (d) | cfg(test)-gated; crate=semio-os-services-native-paths kinds=lib |
| 16 | `🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/🧵️channel/🧪️tests/🪤️companion/🦀️.rs` | 5 | (d) | cfg(test)-gated test-path; crate=semio-os-channel-tests kinds=lib |
| 17 | `🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/🧪️tests/🥪️companion/🦀️.rs` | 8 | (d) | cfg(test)-gated test-path; crate=semio-os-dsl-tests kinds=lib |
| 18 | `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🦀️.rs` | 5 | (d) | cfg(test)-gated; crate=semio-framework-artifact-space-collection kinds=lib |
| 19 | `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/🧪️tests/⚖️retirement-law/🦀️.rs` | 2 | (d) | chain-gated test-path; crate=semio-framework-artifact-space-collection kinds=test |
| 20 | `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🦀️.rs` | 5 | (d) | cfg(test)-gated; crate=semio-framework-artifact-space-space kinds=lib |
| 21 | `🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/🧪️tests/⚖️retirement-law/🦀️.rs` | 2 | (d) | chain-gated test-path; crate=semio-framework-artifact-space-space kinds=test |
| 22 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/💰️backing/🦀️.rs` | 12 | (d) | chain-gated test-path; crate=semio-framework-artifact-workflow-workflow-schema-laws kinds=lib |
| 23 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🦀️.rs` | 79 | (d) | cfg(test)-gated; crate=semio-os-store-replay kinds=lib |
| 24 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📬️command/♻️retirement/🧪️tests/🚪️public/🦀️.rs` | 3 | (d) | chain-gated test-path; crate=semio-os-kernel kinds=test |
| 25 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🪶️snapshot-capability/🪶️native-decoding/🧪️tests/🚪️public/🦀️.rs` | 20 | (d) | chain-gated test-path; crate=semio-os-kernel kinds=test |
| 26 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧩️composition/🌳️graph/🤝️group/🧪️tests/🔬️isolated/🦀️.rs` | 13 | (c) | no Cargo target mounts it |
| 27 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📜️space-history/🦀️.rs` | 5 | (d) | cfg(test)-gated; crate=semio-os-store-space-history kinds=lib |
| 28 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/👥️presence/📸️snapshot/🦀️.rs` | 21 | (d) | cfg(test)-gated; crate=semio-os-presence-native-snapshot kinds=lib |
| 29 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/🦀️.rs` | 713 | (d) | cfg(test)-gated; crate=semio-os-store-ledger kinds=lib |
| 30 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📒️ledger/📨️messages/🦀️.rs` | 605 | (d) | cfg(test)-gated; crate=semio-os-store-message-ledger kinds=lib |
| 31 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️backbone/🦀️.rs` | 605 | (d) | cfg(test)-gated; crate=semio-os-store-backbone kinds=lib |
| 32 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🏗️initialization/🏃️runtime/🧪️tests/🔬️unit/🦀️.rs` | 4 | (d) | chain-gated test-path; crate=semio-os-store-initialization-runtime kinds=lib |
| 33 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎮️one-item/📬️publication/📥️lifted/🧵️operation-wire/🎯️target/🦀️.rs` | 4 | (d) | chain-gated; crate=semio-os-store-one-item-publication kinds=test |
| 34 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/♻️retirement/🦀️.rs` | 9 | (d) | cfg(test)-gated; crate=semio-os-store-retirement kinds=lib |
| 35 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🦀️.rs` | 15 | (d) | cfg(test)-gated; crate=semio-os-store-engine kinds=lib |
| 36 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🌿️branch/🦀️.rs` | 89 | (d) | cfg(test)-gated; crate=semio-os-store-engine-branch kinds=lib |
| 37 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/⚙️engine/🗄️stage/📥️lifted/🗄️durable-group/🧵️binding/🧪️tests/🦀️.rs` | 3 | (d) | test-path; crate=semio-os-store-engine-stage kinds=lib |
| 38 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔗️read/🧪️tests/🔬️unit/🦀️.rs` | 12 | (d) | chain-gated test-path; crate=semio-os-store-read kinds=lib |
| 39 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🫧️ephemeral/🦀️.rs` | 41 | (d) | cfg(test)-gated; crate=semio-os-store-ephemeral kinds=lib |
| 40 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧬️schema/🧬️mutations/🧪️tests/🔬️unit/🦀️.rs` | 119 | (d) | chain-gated test-path; crate=semio-os-store-space-schema kinds=lib |
| 41 | `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧵️canonical-edit/🦀️.rs` | 493 | (d) | cfg(test)-gated; crate=semio-os-store-canonical-json kinds=lib |
| 42 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🦀️.rs` | 16 | (d) | cfg(test)-gated; crate=semio-os-vcs kinds=lib |
| 43 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🧪️tests/🥣️companion/🦀️.rs` | 10 | (d) | cfg(test)-gated test-path; crate=semio-os-vcs-tests kinds=lib |
| 44 | `🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🧬️generation/🧪️tests/🧬️generation/🦀️.rs` | 15 | (d) | chain-gated test-path; crate=semio-framework-artifact-playbook-playbook kinds=lib |
| 45 | `🧰️framework/🛍️products/💻️os/🔨️modules/🧬️semio/🦀️.rs` | 9 | (d) | cfg(test)-gated; crate=semio-os-semio kinds=lib |
| 46 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🗄️storage/🦀️.rs` | 58 | (d) | cfg(test)-gated; crate=semio-os-kernel-db-storage kinds=lib |
| 47 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🦀️.rs` | 34 | (d) | cfg(test)-gated; crate=semio-os-kernel-db-sync kinds=lib |
| 48 | `🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/🔄️sync/🧪️tests/🔬️laws/🦀️.rs` | 14 | (d) | cfg(test)-gated test-path; crate=semio-os-kernel-db-sync-laws kinds=lib |
| 49 | `🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/⏱️camera/📦️packages/🦀️rust/🦀️.rs` | 7 | (d) | cfg(test)-gated; crate=semio-renderer-camera kinds=lib |
| 50 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧾️document/🦀️.rs` | 18 | (d) | cfg(test)-gated; crate=semio-framework-artifact-flow-flow-document kinds=lib |
| 51 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/🧵️retained/🧪️tests/🧬️fields/🦀️.rs` | 13 | (d) | chain-gated test-path; crate=semio-framework-artifact-flow-flow-copy kinds=lib |
| 52 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🧪️tests/⚖️retirement-law/🦀️.rs` | 2 | (d) | chain-gated test-path; crate=semio-os-flow kinds=test |
| 53 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/🦀️.rs` | 11 | (d) | cfg(test)-gated; crate=semio-plugin-job kinds=lib |
| 54 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host/🦀️.rs` | 9 | (d) CONFLICT | cfg(test)-gated; crate=semio-plugin-host-configuration kinds=lib |
| 55 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/🔬️app-typed-command-full-operation/🧩️child-operations/⚖️laws/🦀️.rs` | 9 | (d) | chain-gated test-path; crate=semio-plugin-child-group-wire-laws kinds=test |
| 56 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧪️tests/📑️command-page/🦀️.rs` | 3 | (d) | chain-gated test-path; crate=semio-plugin-command-page-laws kinds=test |
| 57 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🦀️.rs` | 10 | (d) CONFLICT | cfg(test)-gated; crate=semio-plugin-runtime-ops kinds=lib |
| 58 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/👁️view/staged-form/🦀️.rs` | 26 | (d) | cfg(test)-gated; crate=semio-plugin-decl-staged-form kinds=lib |
| 59 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📜️declaration/🧩️app/🧪️tests/🥣️companion/🦀️.rs` | 3 | (d) | cfg(test)-gated test-path; crate=semio-plugin-decl-app-tests kinds=lib |
| 60 | `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️retained-command/🧪️tests/🥣️companion/🦀️.rs` | 3 | (d) | cfg(test)-gated test-path; crate=semio-plugin-retained-command-tests kinds=lib |
| 61 | `🧰️framework/🛍️products/💻️os/🔨️modules/🚪️io/📤️serialization/📦️owned/🧪️tests/🦀️.rs` | 9 | (d) | chain-gated test-path; crate=semio-os-io kinds=test |
| 62 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust/🦀️.rs` | 6 | (d) | cfg(test)-gated; crate=semio-os-mcp kinds=lib |
| 63 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/⚙️plugin-runtime/🦀️.rs` | 6 | (d) | cfg(test)-gated; crate=semio-os-mcp-plugin-runtime kinds=lib |
| 64 | `🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/🧬️schema/🧪️tests/💡️borrowed-wire/🦀️.rs` | 4 | (d) | chain-gated test-path; crate=semio-os-mcp-schema kinds=test |
| 65 | `🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/🚪️io/🪶️sqlite/📸️snapshot/🧪️tests/🧮️properties/🦀️.rs` | 16 | (d) | chain-gated test-path; crate=semio-framework-artifact-infinite-dag kinds=lib |
| 66 | `🧰️framework/🔨️modules/🗜️deflate/🧪️tests/🔬️unit/🛫️controlled/🦀️.rs` | 57 | (d) | chain-gated test-path; crate=semio-framework-deflate kinds=lib |
| 67 | `🧰️framework/🔨️modules/🗺️surface/🕸️node-graph/🦀️.rs` | 118 | (d) | cfg(test)-gated; crate=semio-framework-surface-node-graph kinds=lib |
| 68 | `🧰️framework/🔨️modules/⚠️diagnostic/🧾️retained/🧪️tests/🦀️.rs` | 3 | (d) | chain-gated test-path; crate=semio-framework-diagnostic kinds=lib |
| 69 | `🧰️framework/🔨️modules/🖼️canvas/🎬️scene/🦀️.rs` | 275 | (d) | cfg(test)-gated; crate=semio-framework-canvas-scene kinds=lib |
| 70 | `🧰️framework/🔨️modules/🖼️canvas/♻️retirement/🎬️scene/🧪️tests/🦀️.rs` | 1 | (d) | chain-gated test-path; crate=semio-framework-canvas-scene kinds=test |
| 71 | `🧰️framework/🔨️modules/🧵️job/🏃️work/🎟️grant/🧪️tests/funding.rs` | 8 | (d) | chain-gated test-path; crate=semio-framework-job-work-grant kinds=test |
| 72 | `🧰️framework/🔨️modules/🧵️job/🔭️preview/🎟️drive/🦀️.rs` | 117 | (d) | cfg(test)-gated; crate=semio-framework-job-preview kinds=lib |
| 73 | `🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🧪️tests/🦀️.rs` | 4 | (d) | chain-gated test-path; crate=semio-framework-job-publication kinds=test |
| 74 | `🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🪙️escrow/🧪️tests/🦀️.rs` | 3 | (d) | chain-gated test-path; crate=semio-framework-job-return-escrow-publication kinds=lib |
| 75 | `🧰️framework/🔨️modules/🧵️job/📬️outcome/📤️publication/🪙️escrow/🧪️tests/🌱️birth/🦀️.rs` | 4 | (d) | chain-gated test-path; crate=semio-framework-job-return-escrow-publication kinds=test |
| 76 | `🧰️framework/🔨️modules/⏱️trace/📈️percentile/🔐️gate/🧪️tests/🦀️.rs` | 4 | (d) | chain-gated test-path; crate=semio-framework-trace kinds=lib |
| 77 | `🧰️framework/🔨️modules/🧊️3d/🧪️tests/🧰️support/🦀️.rs` | 25 | (d) | test-path; crate=semio-framework-3d-testing-support kinds=lib |
| 78 | `🧰️framework/🔨️modules/⏯️tool-run/🦀️.rs` | 201 | (d) | cfg(test)-gated; crate=semio-framework-tool-run kinds=lib |
| 79 | `🧰️framework/🔨️modules/⏯️tool-run/🧪️tests/🥣️companion/🦀️.rs` | 13 | (d) | cfg(test)-gated test-path; crate=semio-framework-tool-run-tests kinds=lib |
| 80 | `🧰️framework/🔨️modules/⏯️tool-run/👥️entities/🦀️.rs` | 190 | (d) | cfg(test)-gated; crate=semio-framework-tool-run-entities kinds=lib |
| 81 | `🧰️framework/🔨️modules/⏯️tool-run/🎞️tick/🦀️.rs` | 53 | (d) | cfg(test)-gated; crate=semio-framework-tool-run-tick-wire kinds=lib |
| 82 | `🧰️framework/🔨️modules/⏯️tool-run/📊️progress/🪟️presentation/🦀️.rs` | 51 | (d) | cfg(test)-gated; crate=semio-framework-tool-run-presentation kinds=lib |
| 83 | `🧰️framework/🔨️modules/🧮️math/📶️rank/🦀️.rs` | 113 | (d) | cfg(test)-gated; crate=semio-framework-math-rank kinds=lib |
| 84 | `🧰️framework/🔨️modules/◻️2d/🛤️path/📏️flatten/🧪️tests/🎟️work/🦀️.rs` | 6 | (d) | chain-gated test-path; crate=semio-framework-2d-path-flatten kinds=lib |
| 85 | `🧰️framework/🔨️modules/◻️2d/🛤️path/🖊️stroke/🧪️tests/🎟️work/🦀️.rs` | 6 | (d) | chain-gated test-path; crate=semio-framework-2d-path-stroke kinds=lib |
| 86 | `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🧪️tests/🦀️.rs` | 5 | (d) | chain-gated test-path; crate=semio-framework-2d-text-font-shape kinds=lib |
| 87 | `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️shape/🌐️unicode/🧪️tests/🦀️.rs` | 5 | (d) | chain-gated test-path; crate=semio-framework-2d-text-font-shape-unicode kinds=lib |
| 88 | `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧪️tests/🦀️.rs` | 4 | (d) | chain-gated test-path; crate=semio-framework-2d-text-font kinds=lib |
| 89 | `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/📇️catalog/🧪️tests/🦀️.rs` | 5 | (d) | chain-gated test-path; crate=semio-framework-2d-text-font-catalog kinds=lib |
| 90 | `🧰️framework/🔨️modules/◻️2d/📝️text/🔤️font/🧵️run/🧪️tests/🦀️.rs` | 5 | (d) | chain-gated test-path; crate=semio-framework-2d-text-font-run kinds=lib |
| 91 | `🧰️framework/🔨️modules/🏗️mesh-engine/🎯️component-references/🧪️tests/🦀️.rs` | 34 | (d) | chain-gated test-path; crate=semio-framework-mesh-component-references kinds=lib |
| 92 | `🧰️framework/🔨️modules/🎒️pack/⚠️error/🦀️.rs` | 6 | (d) | cfg(test)-gated; crate=semio-framework-pack-error kinds=lib |
| 93 | `🧰️framework/🔨️modules/🎒️pack/🌱️value/🫳️preflight/🦀️.rs` | 10 | (d) | cfg(test)-gated; crate=semio-framework-pack-record-preflight kinds=lib |
| 94 | `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🦀️.rs` | 13 | (d) | cfg(test)-gated; crate=semio-framework-pack-record-encode kinds=lib |
| 95 | `🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/⏳️cursor/🦀️.rs` | 16 | (d) | cfg(test)-gated; crate=semio-framework-pack-record-encode-cursor kinds=lib |
| 96 | `🧰️framework/🔨️modules/🎒️pack/🌱️intrinsic/🦀️.rs` | 9 | (d) | cfg(test)-gated; crate=semio-framework-pack-intrinsic kinds=lib |
| 97 | `🧰️framework/🔨️modules/🎒️pack/🔤️json/📥️retained/🧬️schema/🦀️.rs` | 34 | (d) | cfg(test)-gated; crate=semio-framework-pack-json-retained-schema kinds=lib |
| 98 | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️mesh-unit/🦀️.rs` | 419 | (d) | chain-gated test-path; crate=semio-ui-scene-mesh kinds=lib |
| 99 | `🧰️framework/🔨️modules/🖱️ui/🎬️scene/🧪️tests/🔬️world3d-modelling-unit/🦀️.rs` | 342 | (d) | chain-gated test-path; crate=semio-ui-scene-modelling kinds=lib |
| 100 | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️tests/🔎️observed-allocator/🦀️.rs` | 23 | (d) | chain-gated test-path; crate=semio-ui-host kinds=lib |
| 101 | `🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️tests/🥣️companion/🔎️observed-allocator/🦀️.rs` | 23 | (d) | test-path; crate=semio-ui-host-tests kinds=lib |
| 102 | `🧰️framework/🔨️modules/🖱️ui/🖼️raster/🧵️ownership/🦀️.rs` | 10 | (d) | cfg(test)-gated; crate=semio-ui-raster-owner kinds=lib |
| 103 | `🧰️framework/🔨️modules/🖱️ui/🔤️typography/🦀️.rs` | 9 | (d) | cfg(test)-gated; crate=semio-ui-text kinds=lib |
| 104 | `🧰️framework/🔨️modules/⏳️async/🧪️tests/🥣️companion/🦀️.rs` | 41 | (d) | cfg(test)-gated test-path; crate=semio-framework-async-tests kinds=lib |
| 105 | `🧰️framework/🔨️modules/🛂️manifest/🔡️mutation-inputs/🦀️.rs` | 932 | (d) | cfg(test)-gated; crate=semio-framework-manifest-mutation-inputs kinds=lib |
| 106 | `🧰️framework/🔨️modules/🛂️manifest/🖼️view-model/🦀️.rs` | 469 | (d) | cfg(test)-gated; crate=semio-framework-manifest-view-model kinds=lib |
| 107 | `🧰️framework/🔨️modules/🌱️value/🧪️testing/💾️allocator/🦀️.rs` | 49 | (a) | testing allocator own declaration |
| 108 | `🧰️framework/🔨️modules/🌱️value/💾️resident/🧪️tests/💾️resident/🦀️.rs` | 58 | (d) | chain-gated test-path; crate=semio-framework-value-resident kinds=lib |
| 109 | `🧰️framework/🔨️modules/🌱️value/🛬️decode/🧳️owned/🌳️value/🎟️reservation/🧪️tests/🦀️.rs` | 5 | (d) | chain-gated test-path; crate=semio-framework-value-native-reservation kinds=lib |
| 110 | `🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust/🦀️.rs` | 11 | (d) | cfg(test)-gated; crate=semio-framework-schema-validator kinds=lib |
| 111 | `🧰️framework/🔨️modules/⏪️time-travel/📦️packages/🦀️rust/🦀️.rs` | 9 | (d) | cfg(test)-gated; crate=semio-framework-time-travel kinds=lib |
| 112 | `🧰️framework/🔨️modules/📡️replication/⚠️error/🦀️.rs` | 4 | (d) | cfg(test)-gated; crate=semio-framework-replication-error kinds=lib |
| 113 | `🧰️framework/🔨️modules/📡️replication/📡️wire/🎮️command/📥️ingress/🦀️.rs` | 257 | (d) | cfg(test)-gated; crate=semio-framework-replication-command-ingress kinds=lib |
| 114 | `🧰️framework/🔨️modules/📡️replication/📡️wire/🏠️local-interaction/🦀️.rs` | 466 | (d) | cfg(test)-gated; crate=semio-framework-replication-local-interaction kinds=lib |
| 115 | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔺️diff/🪢️sequence/🧪️tests/🦀️.rs` | 10 | (d) | chain-gated test-path; crate=semio-framework-replication kinds=test |
| 116 | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧳️retained/🔎️key/📦️packages/🦀️rust/🦀️.rs` | 9 | (d) | cfg(test)-gated; crate=semio-framework-replication-retained-key kinds=lib |
| 117 | `🧰️framework/🔨️modules/📡️replication/🎮️mutation/✏️edit/🦀️.rs` | 420 | (d) | cfg(test)-gated; crate=semio-framework-replication-edit kinds=lib |
| 118 | `🧰️framework/🔨️modules/📡️replication/⚙️codec/📦️packages/🦀️rust/🦀️.rs` | 7 | (d) | cfg(test)-gated; crate=semio-framework-replication-codec kinds=lib |
| 119 | `🧰️framework/🔨️modules/📡️replication/📐️format/🛫️encode/🫳️borrowed/🧪️tests.rs` | 3 | (d) | chain-gated; crate=semio-framework-replication-format kinds=lib |
| 120 | `🧰️framework/🔨️modules/📡️replication/🧾️wire/📦️packages/🦀️rust/🦀️.rs` | 7 | (d) | cfg(test)-gated; crate=semio-framework-replication-wire-core kinds=lib |
| 121 | `🧰️framework/🔨️modules/📡️replication/🔗️causal/📦️packages/🦀️rust/🦀️.rs` | 7 | (d) | cfg(test)-gated; crate=semio-framework-replication-causal kinds=lib |
| 122 | `🧰️framework/🔨️modules/🚪️io/🔤️base64/🧪️tests/🥣️companion/🦀️.rs` | 7 | (d) | cfg(test)-gated test-path; crate=semio-framework-io-base64-tests kinds=lib |
| 123 | `🧰️framework/🔨️modules/🚪️io/🔢️binary/📥️source/🧪️tests/🦀️.rs` | 8 | (d) | chain-gated test-path; crate=semio-framework-io-binary-source kinds=lib |
| 124 | `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️testing/💰️backing/🦀️.rs` | 67 | (d) | test-path; crate=semio-framework-io-sqlite-snapshot-testing-backing kinds=lib |
| 125 | `🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/🧪️tests/🌅️companion/🦀️.rs` | 5 | (d) | cfg(test)-gated test-path; crate=semio-framework-io-sqlite-snapshot-tests kinds=lib |
| 126 | `🧰️framework/🔨️modules/🚪️io/⏱️control/🧪️testing/🧪️tests/🦀️.rs` | 3 | (d) | chain-gated test-path; crate=semio-framework-io-control-testing kinds=lib |

### 2a. Three-way conflicts: crate depends on `semio-framework-value-testing-allocator` and also declares its own `#[global_allocator]`

- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/👷️job/📨️emission/🏛️host/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🏃️runtime/🧰️ops/🦀️.rs`
- `🧰️framework/🔨️modules/🌱️value/♻️retirement/🔗️arc-custody/🧪️tests/🦀️.rs`

These cannot link two global allocators into one test binary. The owning crates should drop the local declaration.

## 3. `#[path]` includes of allocator files

| including file | line | included file | verdict |
|---|---|---|---|
| `✏️s/🔌️plugins/📏️layout/🗿️artifacts/📏️layout/🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔺️diff/🧳️retained/🪡️apply/👁️final-graph/🧪️tests.rs` | 5 | testing allocator | legitimate consumer (a) |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🧪️tests/🎬️companion-scene/🦀️.rs` | 38 | `🧪️tests/♻️physical-job-close/⏱️allocator/🦀️.rs` | helper, no `GlobalAlloc` impl found |
| `🧰️framework/🔨️modules/🖱️ui/🎯️targets/🧊️wgpu/⚙️engine/🧪️tests/🥣️companion/🦀️.rs` | 41 | same helper | helper, no `GlobalAlloc` impl found |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/🦀️.rs` | 26 | `🧪️tests/🔎️observed-allocator/🦀️.rs` | (d) duplicate, see table 2 |
| `🧰️framework/🔨️modules/🖱️ui/🖥️host/🧪️tests/🥣️companion/🦀️.rs` | 36 | `🔎️observed-allocator/🦀️.rs` (companion copy) | (d) duplicate; the two copies DIFFER |

## 4. Findings and follow-ups

- The coordination plan (08:00 entry) says about 210 `#[global_allocator]` were removed. 126 declarations remain at HEAD, 121 of them duplicates. Either the upstream merge reintroduced them or the unification did not finish. Check against `test-allocator-unification.md`.
- Orphans (c): four files are never compiled. Delete them or mount them. The `retained-params` one is orphaned because the `#[path]` in `🔺️diff/🦀️.rs:649` points at a different copy (`🔺️diff/🧪️tests/⚙️retained-params/🦀️.rs`, no allocator). The other three are the `✏️editor/🧪️tests/🔬️unit`, `✏️editor/🧪️tests/⚖️retirement-law` and `group/🧪️tests/🔬️isolated` files.
- `alloc-static.py` reports 6 UNMAPPED files. Two of them (`sourcing/.../📸️snapshot/🧪️tests/💰️backing` and `workflow/.../📸️snapshot/🧪️tests/💰️backing`) are mounted through `#[path="\u{1f4b0}..."]` Rust unicode escapes, which the script does not decode. The script also does not follow plain `mod x;`. The mount analysis here decodes the escapes and follows `mod`, `#[path]`, and `include!`.
- `observed-allocator` exists twice (`🖥️host/🧪️tests/🔎️observed-allocator` and the `🥣️companion` copy). The two files differ. Keep one and make the other a consumer of the testing allocator.
- Method note: 'test scope' = `#[cfg(test)]` on the declaration or an enclosing mod (checked within the attribute block above the declaration and at file level), a Cargo test/example/bench target, or a `tests`/`testing` path component. Mount graph: every Cargo target (lib, bin, test, example, bench) traced through `mod`, `#[path]` and `include!`. Scripts and raw outputs are in `🗑️generated/audit3-*`.
