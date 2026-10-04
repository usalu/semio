# Current Replication Native Deletion Scope Audit

Static read-only analysis; no Cargo/rustc/native/build commands, no workspace edits or generated wrapper. Current root has115 literal Cargo members,26 under whole OS. Counts come from exact quoted members input; source list below retains all actual bytes.

## Registered command versus neutral closure

Proposed CommandIngressNativeScript uses runCargoTestsV1 with exact Replication package manifest, packages=[semio-framework-replication], --lib --no-fail-fast, explicit caller Cargo policy. Helper build phase runs cargo nextest list with manifest-path/-p and then executes retained binary metadata; it does not build a standalone workspace, prune absent members, or prepare generated source wrappers. Existing Replication script does not yet register this proposal gate in current workspace. Source-ready registration is distinct from actual executable publication.

Replication manifest explicitly declares workspace=../../../../.., inherited rust-version.workspace/lints.workspace and semio-framework-value.workspace. Actual root includes26 literal OS members. Deleting wholeOS therefore leaves declared absent member manifests; static expected Cargo workspace loading blocker persists despite -p filtering. Native worker must verify actual command behavior; this audit has not invoked Cargo. Current root member roster does not include ✏️s entries, so deleting ✏️s alone is not this exact Cargo-members blocker, while Node workspace/GUI preparation still requires independent authority review.

Proof A (bounded package deletion): retain actual neutral Replication normal AND dev package/source closure and exact tests/actor fixture; create a ticket-owned standalone neutral workspace with explicit required inherited package/lint/dependency authority, resolving all inherited paths, and no OS/plugin/s sources. Include dev dependency Pack JSON/async macros and their full needed transitive/build/proc-macro/include contexts. Verify actual compiler metadata/binds and whole Replication laws with sole worker. Do not call ten-package normal closure alone a native-test deletion proof; dev/build/proc-macro/test inputs can extend it.

Proof B (actual repository deletion): run ordinary registered Bun→Nx gate against an actual root registration model that remains valid with optional wholeOS removed. The current literal roster does not provide that contract. Root workspace registration must be canonical and allow owner absence while rejecting a present malformed owner, with schema/independent tests. A ticket-only standalone proof A cannot silently claim B; do not hand-edit production root merely for proof. Preserve authored root bytes and record this registration blocker.

General native wrapper proof requires explicit retained workspace/config/test policy and exact current source include/cfg/macro binds. Neither source AST overlay nor foreign-source patch preservation establishes compiler origin. No generated wrapper binds were created/compiled by this lane; sole-worker receipts must provide those identities.

## Exact wholeOS member blockers

```json
[
  "🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🧩️component/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust",
  "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust"
]
```

## Full current/proposal helper receipts

### Cargo.toml

SHA-256 `542604e8a827dcc38c2e0c24922fb46eaeafb3c1b11638aaa1d6dbfb18579d77`; 32897 bytes.

```
cargo-features = ["trim-paths"]

[workspace]
resolver = "2"
members = [
    "🧰️framework/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏪️time-travel/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/✍️editor/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎒️pack/⚠️error/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎒️pack/🔤️json/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📏️intrinsic-size/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📚️compiler/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔄️machine/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔄️machine/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🔲️pixels/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🌐️locale/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗜️deflate/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗣️dsl/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗣️dsl/🧬️schema/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🚪️io/🔤️base64/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🚪️io/🧬️schema/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🚪️io/🪶️sqlite-snapshot/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust",
    "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/❓️quiz/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🔤️fonts/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🏃️run/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🧩️component/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/💻️os/🧫️fixtures/⚖️scale/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/⌨️cli/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🌳️tree/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎫️tickets/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎯️goals/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏃️test-runner/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🏠️workspace/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📊️metrics/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📐️model/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📜️statutes/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📝️todos/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/📡️events/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔌️mcp/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔎️search/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🔗️graphql/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🖥️server/🎛️coordinator/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🗂️codebase/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🗣️languages/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🚚️move/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧑️contributors/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧩️providers/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧾️yaml/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🪝️hooks/📦️packages/🦀️rust",
    "🧰️framework/🛍️products/🦑️repo/🔨️modules/🪪️identity/📦️packages/🦀️rust",
]
exclude = ["**/🏅️standards/**", "**/🔮️oracles/**", "**/👽️guest/**"]

[workspace.metadata.semio.repository]
schema-version = 1
member-manifests = ["🧰️framework/**/📦️packages/🦀️rust/Cargo.toml"]
owner-manifests = ["[!.]*/Cargo.toml"]

[workspace.package]
version = "0.1.0"
edition = "2021"
rust-version = "1.95"

[workspace.dependencies]
semio-framework-schema-state = { path = "🧰️framework/🔨️modules/🧬️schema/📶️state/📦️packages/🦀️rust" }
semio-framework-schema-composition = { path = "🧰️framework/🔨️modules/🧬️schema/🧩️composition/📦️packages/🦀️rust" }
semio-framework-schema-validator = { path = "🧰️framework/🔨️modules/🧬️schema/✅️validator/📦️packages/🦀️rust" }
semio-framework-plugin-host-fixture = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧫️fixtures/🧩️component/📦️packages/🦀️rust" }
semio-framework-plugin-host = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/📦️packages/🦀️rust" }
semio-repo-test-host = { path = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🦀️rust" }
semio-framework-os-renderer-wgpu = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📦️packages/🦀️rust" }
semio-framework-artifact-infinite-dag = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🗿️artifacts/🕸️dag/📦️packages/🦀️rust" }
semio-framework-artifact-flow-flow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🗿️artifacts/🌊️flow/📦️packages/🦀️rust" }
semio-framework-artifact-playbook-playbook = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/📦️packages/🦀️rust" }
semio-framework-artifact-workflow-workflow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🔁️workflow/📦️packages/🦀️rust" }
semio-framework-artifact-space-space = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🪐️space/📦️packages/🦀️rust" }
semio-framework-artifact-space-collection = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🪐️space/🗿️artifacts/🗂️collection/📦️packages/🦀️rust" }
semio-framework-artifact-workflow-run = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔁️workflow/🗿️artifacts/🏃️run/📦️packages/🦀️rust" }
semio-framework-math = { path = "🧰️framework/🔨️modules/🧮️math/📦️packages/🦀️rust" }
semio-framework-number = { path = "🧰️framework/🔨️modules/🔢️number/📦️packages/🦀️rust" }
semio-framework-geometry = { path = "🧰️framework/🔨️modules/📐️geometry/📦️packages/🦀️rust" }
semio-framework-raster = { path = "🧰️framework/🔨️modules/🖌️raster/📦️packages/🦀️rust" }
semio-framework-typeset = { path = "🧰️framework/🔨️modules/🔤️typeset/📦️packages/🦀️rust" }
semio-framework-graph = { path = "🧰️framework/🔨️modules/🕸️graph/📦️packages/🦀️rust" }
semio-framework-graph-layout-run = { path = "🧰️framework/🔨️modules/🕸️graph/⏯️layout-run/📦️packages/🦀️rust" }
semio-framework-actor = { path = "🧰️framework/🔨️modules/🎭️actor/📦️packages/🦀️rust" }
semio-framework-replication = { path = "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust" }
semio-framework-value = { path = "🧰️framework/🔨️modules/🌱️value/📦️packages/🦀️rust" }
semio-framework-value-resident = { path = "🧰️framework/🔨️modules/🌱️value/💾️resident/📦️packages/🦀️rust" }
semio-framework-pack = { path = "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust" }
pack = { path = "🧰️framework/🔨️modules/🎒️pack/📦️packages/🦀️rust", package = "semio-framework-pack" }
semio-framework-server = { path = "🧰️framework/🛍️products/🖥️server/📦️packages/🦀️rust" }
semio-framework-quiz = { path = "🧰️framework/🛍️products/❓️quiz/📦️packages/🦀️rust" }
semio-framework-async = { path = "🧰️framework/🔨️modules/⏳️async/📦️packages/🦀️rust" }
semio-framework-async-macros = { path = "🧰️framework/🔨️modules/⏳️async/✨️macros/📦️packages/🦀️rust" }
semio-framework-trace = { path = "🧰️framework/🔨️modules/⏱️trace/📦️packages/🦀️rust" }
semio-framework-job = { path = "🧰️framework/🔨️modules/🧵️job/📦️packages/🦀️rust" }
semio-framework-tool-run = { path = "🧰️framework/🔨️modules/⏯️tool-run/📦️packages/🦀️rust" }
semio-framework-time-travel = { path = "🧰️framework/🔨️modules/⏪️time-travel/📦️packages/🦀️rust" }
semio-framework-tool-machine = { path = "🧰️framework/🔨️modules/🛠️tool-machine/📦️packages/🦀️rust" }
semio-framework-dispatch-macros = { path = "🧰️framework/🔨️modules/🔀️dispatch/📦️packages/🦀️rust" }
semio-framework-hash = { path = "🧰️framework/🔨️modules/🔏️hash/📦️packages/🦀️rust" }
semio-framework-mesh-engine = { path = "🧰️framework/🔨️modules/🏗️mesh-engine/📦️packages/🦀️rust" }
semio-framework-schema = { path = "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust" }
semio-framework-schema-registry = { path = "🧰️framework/🔨️modules/🧬️schema/📇️registry/📦️packages/🦀️rust" }
schema = { path = "🧰️framework/🔨️modules/🧬️schema/📦️packages/🦀️rust", package = "semio-framework-schema" }
semio-framework-value-derive = { path = "🧰️framework/🔨️modules/🌱️value/✨️derive/📦️packages/🦀️rust" }
semio-framework-os-services = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛎️services/📦️packages/🦀️rust" }
semio-framework-plugin-describe = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖨️describe/📦️packages/🦀️rust" }
# 🌐️ Canonical versions for the highest-fanout external deps, chosen as the newest
# explicit requirement string already used somewhere in the 630 manifests (matches what
# Cargo.lock already resolves to, so adopting `.workspace = true` later is a no-op for
# resolution). Deps below the ~10-manifest survey bar are left alone (see ticket report).
serde = { version = "1.0.228", features = ["derive"] }
serde_json = { version = "1.0.149", features = ["raw_value"] }
wasm-bindgen = "0.2.106"
js-sys = "0.3.83"
tokio = { version = "1" }

# 🧭️ Internal path deps for crates that exist TODAY at their current location, surveyed
# by counting `path = "…"` dependency references across all Cargo.toml files (>5 other
# manifests). Not wired to any member yet — a later wave can adopt `.workspace = true` as
# a drop-in, or repoint ONE line here when a crate merges/moves instead of editing every
# consumer. `# N refs` is the survey count. Grouped: os-kernel/core, math, ui, plugin-internal.
# ---- core (24) ----
semio-framework = { path = "🧰️framework/📦️packages/🦀️rust" }  # 58 refs
semio-framework-os = { path = "🧰️framework/🛍️products/💻️os/🖥️host/📦️packages/🦀️rust" }
semio-framework-os-kernel = { path = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }
semio-s-kernel-flow-extension-brep = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
semio-framework-os-kernel-db-state = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
semio-framework-os-kernel-db-storage = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
semio-framework-os-kernel-db-wal = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
semio-framework-os-kernel-flow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
semio-framework-os-kernel-infinite-board-port-directed-dag = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }
semio-framework-os-kernel-infinite-canvas = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }
semio-framework-kernel-infinite-world = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust", package = "semio-framework-os-infinite" }
semio-framework-os-infinite = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust" }
semio-s-kernel-flow-extension-wasm = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust", package = "semio-framework-os-flow" }
semio-framework-os-flow = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/📦️packages/🦀️rust" }
semio-framework-os-kernel-neural-engine = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🧠️neural/⚙️engine/📦️packages/🦀️rust" }
semio-framework-os-kernel-db = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🛢️db/📦️packages/🦀️rust", package = "semio-framework-os-kernel-db" }
# 🧭️ Keep alias on OLD impl until W8c plugin cut-over deletes the sandwich (packages path already exists on disk).
semio-framework-plugin = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📦️packages/🦀️rust" }
semio-framework-os-config = { path = "🧰️framework/🛍️products/💻️os/🎚️config/📦️packages/🦀️rust" }
semio-framework-os-shell = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🖥️shell/📦️packages/🦀️rust" }
semio-framework-os-mcp = { path = "🧰️framework/🛍️products/💻️os/🔨️modules/🌉️mcp/📦️packages/🦀️rust" }
# REMOVED missing: semio-s-kernel-flow-extension-wasm = { path = "🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust" }  # 10 refs

# ---- math (13) ----

# ---- ui (10) ----
semio-framework-ui-styling = { path = "🧰️framework/🔨️modules/🖱️ui/🎨️styling/📦️packages/🦀️rust" }  # 15 refs
semio-framework-ui-contract = { path = "🧰️framework/🔨️modules/🖱️ui/🧬️contract/📦️packages/🦀️rust" }
semio-framework-ui-render = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/📦️packages/🦀️rust" }
semio-framework-ui-runtime = { path = "🧰️framework/🔨️modules/🖱️ui/🧠️runtime/📦️packages/🦀️rust" }
semio-framework-ui-scene = { path = "🧰️framework/🔨️modules/🖱️ui/🎬️scene/📦️packages/🦀️rust" }
semio-framework-ui-viewport = { path = "🧰️framework/🔨️modules/🖱️ui/🪟️viewport/📦️packages/🦀️rust" }
semio-framework-ui-backend-webgpu = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🧊️webgpu/📦️packages/🦀️rust" }
semio-framework-ui-backend-metal = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🍎️metal/📦️packages/🦀️rust" }
semio-framework-ui-backend-d3d12 = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🪟️d3d12/📦️packages/🦀️rust" }
semio-framework-ui-backend-vulkan = { path = "🧰️framework/🔨️modules/🖱️ui/🖌️render/🎯️targets/🌋️vulkan/📦️packages/🦀️rust" }
semio-framework-ui-host = { path = "🧰️framework/🔨️modules/🖱️ui/🖥️host/📦️packages/🦀️rust" }
semio-framework-ui = { path = "🧰️framework/🔨️modules/🖱️ui/📦️packages/🦀️rust" }
semio-framework-surface = { path = "🧰️framework/🔨️modules/🗺️surface/📦️packages/🦀️rust" }

# ---- plugin (62) ----
semio-framework-2d = { path = "🧰️framework/🔨️modules/◻️2d/📦️packages/🦀️rust" }
semio-framework-3d = { path = "🧰️framework/🔨️modules/🧊️3d/📦️packages/🦀️rust" }
[profile.dev]
debug = false
incremental = false

[profile.dev.build-override]
opt-level = 3

# ⚡️ The plugin runtime every native dev host embeds (the semio MCP gateway, the hub, native shells) is
# optimized even in the dev profile. At `opt-level = 0` the wasmtime component-model machinery around
# every guest crossing and host call, and cranelift compiling a 129 MB wasm-dev guest, dominated the
# guest's own work. Measured on the wfc genesis `inference_run` over the semio MCP: 112.7 s with this
# set unoptimized vs 63.9 s optimized (ticket 26/09/23, `📓️wp-g5.md`), and a first solve after a guest
# rebuild waited ~9 min for its unoptimized cranelift compile. Only these crates change, so a rebuild
# costs their own compile plus a relink of their dependents (1 min 44 s measured).
[profile.dev.package.wasmtime]
opt-level = 3

[profile.dev.package.wasmtime-environ]
opt-level = 3

[profile.dev.package.wasmtime-internal-core]
opt-level = 3

[profile.dev.package.wasmtime-internal-cranelift]
opt-level = 3

[profile.dev.package.wasmtime-internal-fiber]
opt-level = 3

[profile.dev.package.wasmtime-internal-unwinder]
opt-level = 3

[profile.dev.package.wasmtime-internal-cache]
opt-level = 3

[profile.dev.package.wasmtime-internal-component-util]
opt-level = 3

[profile.dev.package.wasmtime-internal-jit-debug]
opt-level = 3

[profile.dev.package.wasmtime-internal-jit-icache-coherence]
opt-level = 3

[profile.dev.package.wasmtime-wasi]
opt-level = 3

[profile.dev.package.wasmtime-wasi-io]
opt-level = 3

[profile.dev.package.cranelift-codegen]
opt-level = 3

[profile.dev.package.cranelift-frontend]
opt-level = 3

[profile.dev.package.cranelift-entity]
opt-level = 3

[profile.dev.package.cranelift-bforest]
opt-level = 3

[profile.dev.package.cranelift-bitset]
opt-level = 3

[profile.dev.package.cranelift-control]
opt-level = 3

[profile.dev.package.cranelift-native]
opt-level = 3

[profile.dev.package.cranelift-codegen-shared]
opt-level = 3

[profile.dev.package.cranelift-assembler-x64]
opt-level = 3

[profile.dev.package.regalloc2]
opt-level = 3

[profile.dev.package.wasmparser]
opt-level = 3

[profile.dev.package.pulley-interpreter]
opt-level = 3

[profile.dev.package.gimli]
opt-level = 3

[profile.dev.package.object]
opt-level = 3

[profile.dev.package.wit-parser]
opt-level = 3

# 🧮️ The owned wasm interpreter (`semio-framework-plugin-host::interpreter`) runs every trusted-catalog
# verification `codec.pack-schema-hash` on the hub's startup path. At `opt-level = 0` a debug hub spent
# >11 min of one core interpreting writer/draw/puzzle during catalog load and the candidate readiness
# wait gave up (ticket 26/09/23 W1 §4.6, sampled: 100 % in `CoreInstance::execute_machine`).
[profile.dev.package.semio-framework-plugin-host]
opt-level = 3

# 🔐️ The repository's own SHA-256 carries every hub credential check: a sign-in derives PBKDF2-HMAC-SHA256
# at 210 000 iterations. At `opt-level = 0` one derivation took ~2 s on an idle debug hub and 8.5 s on hub
# 7800 under load (ticket 26/09/23 H9 session 12), all inside the compression function; optimizing this
# one small crate makes a dev hub's sign-in cost what a release hub's does.
[profile.dev.package.semio-framework-hash]
opt-level = 3

# 🛡️ WASI component links alone select this mitigation for rust-lld's ElemSection crash.
# Native dev retains Cargo's parallel codegen policy; publication stays wasm-release.
[profile.wasm-dev]
inherits = "dev"
codegen-units = 1
# 🧾️ A component's described bytes must be its shipped bytes: `describe` and `component-dev` link the same
# `cargo rustc --crate-type cdylib` unit, and `incremental` is part of that unit's profile identity, so an
# inherited `incremental = true` split it by the caller's `CARGO_INCREMENTAL` into two compiles with two
# different wasm hashes (ticket 26/09/23 W1: descriptor `ce48…`/`2a5c…` vs staged `a740…`).
incremental = false

# 🎚️ The plugin guest must meet the framework's 8 ms interactive-step contract even in the dev
# profile: at `opt-level = 0` a lowpoly render turn tessellates its seeded mesh in 10-11.5 ms and
# the runtime traps it with `plugin.internal.interactive-ceiling`, so the window renders a fault
# instead of geometry. Scoped to this package rather than raised on the whole profile because the
# io layer (`semio-s-plugin-stdio`) is not on the render path and is far more expensive to compile.
[profile.wasm-dev.package.semio-framework-os-flow]
opt-level = 2

# 🎚️ `Evaluator::evaluate_channels_budgeted` — the topological dag walk itself, run once per hop.
[profile.wasm-dev.package.semio-framework-os-kernel-neural-engine]
opt-level = 2

# 🎚️ `DslValue`/`OrderedMap` — the per-neuron `Dictionary` every walk clones, merges and hashes.
[profile.wasm-dev.package.semio-framework-replication]
opt-level = 2

# 🎚️ `pack::json` — the codec that serializes each node's input/output payload on the same hop.
[profile.wasm-dev.package.semio-framework-pack]
opt-level = 2

# 🎚️ Answers `capability: evaluate`/`tessellate` for the BREP operators the example drives.
[profile.release]
opt-level = 3
lto = "thin"
codegen-units = 1
strip = "debuginfo"
incremental = false
trim-paths = "object"

# 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT. wasm32-wasip2 plugin components only — os-dev's
# 📜️script.ts passes `--profile wasm-release` when building plugin crates. Inherits ship-oriented
# `[profile.release]` above, then overrides for wasm size/runtime (see below).
#
# - opt-level "s" (not "z"): "z" measurably slows hot numeric loops in geometry/FEM solvers for a
#   few extra percent of size; the wasm-opt -Oz post-pass (see buildPlugin/transpilePluginComponent)
#   gets the "z"-class shrink without paying that runtime cost in every plugin.
# - lto "thin" (not "fat"): fat LTO re-links ~25 plugin cdylibs individually — 2-4x slower per plugin
#   for a low single-digit percent size gain over thin. Thin still gets full cross-crate inlining.
# - codegen-units = 1: maximizes cross-crate dedup/inlining and also sidesteps the LLVM-22
#   ElemSection::writeBody crash noted above (a stable low CGU count, same fix as `store`'s override).
# - strip = "symbols": drops the wasm `name` custom section, which is pure debug/dev-tooling weight
#   (measured ~14MB on the largest single plugin, ~87MB across the built fleet) never used at runtime.
# - incremental = false: deterministic output; release units are rebuilt whole anyway, so incremental
#   session state would only cost disk in the shared build-dir (`.cargo/config.toml` `build.build-dir`).
# - trim-paths = "object": strips absolute build-host cargo-registry paths retained in panic
#   `Location` strings (panic = "unwind" is intentionally NOT overridden here — wasm32-wasip2's
#   target spec already defaults to panic-strategy "abort", so plugins already abort-on-panic and
#   `catch_unwind` already never catches on this target; setting it explicitly would be a no-op).
[profile.wasm-release]
inherits = "release"
opt-level = "s"
lto = "thin"
codegen-units = 1
strip = "symbols"
incremental = false
trim-paths = "object"

# Explicit: profile inheritance does NOT inherit package-specific overrides from the parent
# profile, so without this `store` would fall back to workspace defaults under `wasm-release`.
[profile.wasm-release.package.semio-framework-os-kernel]
codegen-units = 1

# 🧹️ RUST-WIDE-CLEAN-REFACTOR-CAMPAIGN baseline. Kept at "warn" (never "deny") in
# the manifest so live concurrent edits never hard-break; zero-warning is enforced
# at verification gates via `cargo clippy -- -D warnings`. NEVER set RUSTFLAGS to
# add -D warnings — it replaces (not merges) .cargo/config.toml's rustflags.
[workspace.lints.rust]
future_incompatible = { level = "warn", priority = -1 }
rust_2018_idioms = { level = "warn", priority = -1 }
unsafe_op_in_unsafe_fn = "warn"
unused_lifetimes = "warn"
unused_qualifications = "warn"

[workspace.lints.clippy]
all = { level = "warn", priority = -1 }
cloned_instead_of_copied = "warn"
inefficient_to_string = "warn"
map_unwrap_or = "warn"
needless_pass_by_value = "warn"
semicolon_if_nothing_returned = "warn"
unnecessary_wraps = "warn"
redundant_clone = "warn"
# phase B (enable after the T2/T3 waves land): unwrap_used = "warn"

```

### .cargo/config.toml

SHA-256 `79757ec0ca6505f9caceb2b5646c3b7b2bb93d8f1950c3992aba01881fc1f0ee`; 5327 bytes.

```
# ⚡️ One shared compiler cache for every workspace, crate, profile, target, agent and dev. Intermediates live in
# `build-dir`, uplifted deliverables in `target-dir`; both sit under the repo cache root, so disk is bounded by the
# codebase and its build history (pruned by `bun nx run repo:cache-prune`) instead of by the
# number of concurrent builders. `fine-grain-locking` locks per compilation unit instead of per directory, so
# concurrent cargo invocations share every already-built unit and only wait on units another invocation is building.
# `checksum-freshness` keys freshness on content, so Nx cache restores and checkouts that only touch mtimes rebuild nothing.
# A private `CARGO_TARGET_DIR` only diverts the small uplifted deliverables; intermediates stay shared.
# https://doc.rust-lang.org/nightly/cargo/reference/unstable.html#fine-grain-locking
[build]
target-dir = ".🧬semio/🦑️repo/⚡️cache/cargo/target"
build-dir = ".🧬semio/🦑️repo/⚡️cache/cargo/build"
rustflags = ["-Z", "threads=8"]

[unstable]
no-embed-metadata = true
fine-grain-locking = true
build-dir-new-layout = true
checksum-freshness = true

# 🧊️ The browser renderer's shadow stack. LLVM's wasm-ld default is 1 MiB, and a debug build of the wgpu
# renderer overruns it during ordinary construction — `<UiSurfaceRegistry as Default>::default` alone
# materialises `[Option<UiSurfaceSlot>; 64]` on the stack — which traps as a bare `RuntimeError: memory
# access out of bounds` with no Rust panic, since the overflow writes below the stack rather than aborting.
# Measured: every `semioWgpuWorkerBootstrap` died there until this flag was set. 16 MiB is double the
# 8 MiB the plugin components carry (`[target.wasm32-wasip2]` below), because the renderer's debug
# frames are far larger than a plugin's.
[target.wasm32-unknown-unknown]
rustflags = ['--cfg', 'getrandom_backend="wasm_js"', '-C', 'link-arg=-zstack-size=16777216']

# 🪶️ REDUCE-DEMONSTRATOR-IDLE-MEMORY-FOOTPRINT. Plugin components declare no memory `maximum`,
# so V8 reserves the full 4GiB guard region per module per worker (~20 plugins/workers at boot).
# 512MiB is uniform across all plugins on purpose — per-plugin rustflags would churn cargo
# fingerprints across the shared dep graph on every sequential `cargo build -p` in the fleet.
# Target-scoped rustflags REPLACE (not merge with) [build].rustflags, so `-Z threads=8` is repeated.
#
# 🧊️ `-zstack-size=8388608` is the plugin guest's shadow stack, and it belongs HERE — with the memory
# maximum it is carved out of — rather than in whichever build command happens to produce a component.
# It used to be passed per invocation by `🔌️plugin/🏗️build/📋️plan/🟦️.ts`'s `pluginCargoArgs`, so a plain
# `cargo build -p <plugin> --target wasm32-wasip2 --profile wasm-dev` wrote a component with wasm-ld's
# DEFAULT 1 MiB stack into the same shared target directory the dev host and the MCP gateway both read
# from. At `opt-level = 0` that tape runs out inside `Event::InstanceOpen`: `UiPatchApplyArena::default`
# alone spends ~654 KiB of it zero-initialising a 93.5 KiB static, and the guest traps below address 0
# (ticket 26/09/18 slice A2, `📓️a2-mcp-plugin-host-instance-open.md` §3). A component's stack size is
# part of its ABI, not of one builder's argv.
[target.wasm32-wasip2]
rustflags = ["-Z", "threads=8", "-C", "link-arg=--max-memory=536870912", "-C", "link-arg=-zstack-size=8388608"]

# 🔗️ Every glibc Linux host links with the `rust-lld` the pinned toolchain ships (`lib/rustlib/<host>/bin/gcc-ld`),
# so no distribution package decides whether a native build links: `mold` was required here, installed only on
# apt-get systems, and every other distribution failed at the first build script's link step. x86_64 already
# defaults to the self-contained LLD; aarch64 still defaults to GNU ld, so both hosts state it explicitly. Measured
# link times against mold and GNU ld: ticket 26/09/23/END-TO-END-OS-HUB-COLLABORATION-MCP, `📓️wp-z2.md`.
# Target-scoped rustflags REPLACE [build].rustflags, so `-Z threads=8` is repeated.
[target.'cfg(all(target_os = "linux", target_env = "gnu"))']
rustflags = ["-Z", "threads=8", "-C", "linker-features=+lld", "-C", "link-self-contained=+linker", "-Z", "unstable-options"]

# 🧵️ The NATIVE test thread's stack. `libtest` spawns every test on a thread whose default stack is
# 2 MiB, and an unoptimised build of an artifact app overruns it before the first assertion: the async
# state machines of `context::app_with_registry` → `new_app_with_registry` → `VcsArtifactApp::open`
# are materialised inline by `block_on`, measured at 325 KiB, 227 KiB and 221 KiB of frame each, and
# one `procedural generation3d` fixture boot alone lands on 2_096_176 bytes — 976 bytes under the
# default. A stack overflow ABORTS the process instead of failing the test, so the run dies under
# whichever test the allocator happened to be in: that is the whole of the "9 generation3d `--lib`
# reds that swap between runs" (ticket 26/09/09/PROCEDURAL-3D-END-TO-END,
# `📓️editor-verbs-cancel-undo-2026-09-13.md`). Thread stacks are reserved, not committed, so the
# headroom costs address space only. Same defect class as the wasm `-zstack-size` bump above.
[env]
RUST_MIN_STACK = "67108864"

```

### 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/Cargo.toml

SHA-256 `4501c4bf584cfc1928d7c97608ca1356270f1aadc1a73f1db26637bdba6f3ffd`; 5910 bytes.

```
[package]
workspace = "../../../../.."
name = "semio-framework-replication"
version = "0.1.0"
edition = "2021"
rust-version.workspace = true
description = "Product-neutral replication contract: lane-tagged client/server frames, causal mutation envelopes, the mutation trait family, conflict vocabulary, the .spr record format and the pack codec primitives beneath them. Spoken identically by the optimistic local replica and the authoritative server; depends on neither."

[package.metadata.semio]
role = "framework"
id = "replication"

[lints]
workspace = true

[lib]
name = "protocol"
path = "🦀️.rs"

[features]
default = []
# 🗜️ Deflate segment compression (`CodecId(1)`); off by default so the pure contract stays dependency-light.
deflate = ["dep:semio-framework-deflate"]
# 🧪️ Cross-crate opt-in for `OrderedSet`'s hand-written `Serialize`/`Deserialize`
# (🌱️value/🗂️ordered/🧺️set/🦀️.rs's `ArrayWire` region). Plain `#[cfg(test)]` only covers THIS
# crate's own oracle test — a downstream crate's `#[cfg(test)]` build never activates a dependency
# crate's own `#[cfg(test)]` code, so a real cross-crate consumer (`os-flow`'s
# `Widget`/`FlowPreviewGui`, whose own `#[cfg_attr(test, derive(Serialize, Deserialize))]` needs
# this bound) must enable this feature explicitly instead.
ordered-set-serde = ["semio-framework-value/ordered-set-serde"]

[dependencies]
semio-framework-diagnostic = { path = "../../../⚠️diagnostic/📦️packages/🦀️rust" }
semio-framework-schema-registry = { path = "../../../🧬️schema/📇️registry/📦️packages/🦀️rust" }
semio-framework-schema-state = { path = "../../../🧬️schema/📶️state/📦️packages/🦀️rust" }
semio-framework-value.workspace = true
semio-framework-hash = { path = "../../../🔏️hash/📦️packages/🦀️rust" }
semio-framework-deflate = { path = "../../../🗜️deflate/📦️packages/🦀️rust", optional = true }
semio-framework-io-base64 = { path = "../../../🚪️io/🔤️base64/📦️packages/🦀️rust" }
semio-framework-value-derive = { path = "../../../🌱️value/✨️derive/📦️packages/🦀️rust", package = "semio-framework-value-derive" }
# 🚨️ RUNTIME-DEPENDENCY-ELIMINATION-FOR-S-PLUGINS-AND-ARTIFACTS (26/09/01, TENTH seam pass): STILL
# NOT movable to `[dev-dependencies]`, for three remaining reasons — see `📓️orderedmap-tenth-seam.md`.
# The ninth-seam pass's blocker (1) (the `to_dsl_value`/`from_dsl_value` and `encode_wire_serialized`/
# `decode_wire_serialized` bridges) stays resolved, unchanged from before. Blocker (2)'s FOURTH
# item — `🌱️value/🗂️ordered/🦀️.rs`'s `OrderedMap<V>: Serialize` (the real `os-kernel` neural-engine
# `Dictionary` caller) — is now ALSO resolved this pass: `Dictionary`, `Value`, `Atom`, `Neuron`,
# `Tree`, `FieldSpec`, `Schema`, `ChannelSpec`, `OperatorInfo`, `VariadicSpec`, `Cardinality` (all in
# `💻️os/🧠️neural/⚙️engine/🦀️.rs`) and `🌊️flow/🧩️extensions/🕸️wasm/🦀️.rs`'s `FlowExtension*` family
# moved to `ToValue`/`FromValue` + `pack::json`; `OrderedMap<V>: Serialize` is now `#[cfg(test)]`-only.
# THREE items of blocker (2) remain live and unconverted, each independently sufficient to keep
# `serde`/`serde_json` here — confirmed still present by direct read, not carried forward stale:
# - `🌱️value/🦀️.rs`'s `impl serde::Serialize/Deserialize for DslValue` (a deliberate, still-live
#   transitional bridge for OTHER serde-deriving types that hold a `DslValue` field, e.g. `ui_wgpu`'s
#   `ActionDescriptor`) and `impl From<&DslValue> for serde_json::Value` (real, actively-called
#   JSON-export/UI-boundary bridges, confirmed via `✏️s/🔌️plugins/🗄️stdio`/`🏭️process` and the
#   `wgpu` UI target's own callers).
# - `🌱️value/🗂️ordered/🧺️set/🦀️.rs`'s `OrderedSet: Serialize + Deserialize` (real callers in
#   `💻️os/🔨️modules/🌊️flow/**` and the `🌀️procedural` plugins).
# - `InteractionState` and its `🕹️interaction/**` siblings, hit DIRECTLY via
#   `serde_json::to_string`/`from_str`/`from_slice`/`from_value`
#   (`💻️os/🔨️modules/🔌️plugin/🦀️.rs:9860,9864,9875`, `🕹️interaction/📃️query`/`📡️live`), never
#   through `encode_wire_serialized` — separate follow-up work, not covered by this pass.
# Do not attempt further per-type removal by grep-for-`serde_json`-near-the-name alone — the
# `MutationDescriptor` finding in `📓️replication-serde-removal.md` proved that method misses
# derive-based fan-out in `os-kernel`/`os` consumer structs; only a real `cargo check
# -p semio-framework-os-kernel` after the edit proves a type is safe to drop. This pass's own
# fan-out (`FieldSpec`/`Schema`/`ChannelSpec`/`OperatorInfo`, and `os-flow`'s `FlowExtension*`
# family) was found the same way — by removing the impl and reading the compiler, not by grepping.
serde = { version = "1.0.219", features = ["derive"] }
serde_json = "1.0.140"

# 🌉️ `⚠️diagnostic`'s `fault_to_js`/`result_fault_to_js` hand a `Fault` across the JS boundary as a
# `JsValue`; `🧩️puzzle`'s `🌉️wasm` bridge calls them on ~20 lines, so the dep is retired together
# with those two functions and their callers, not ahead of them. Gated `not(target_env = "p2")`
# because `target_arch = "wasm32"` is TRUE for the WASI component target too — the same exclusion
# `💻️os`'s own browser-bridge block uses, and the reason those two functions carry the same cfg.
[target.'cfg(all(target_arch = "wasm32", not(target_env = "p2")))'.dependencies]
wasm-bindgen = "0.2.106"

[dev-dependencies]
semio-framework-pack-json = { path = "../../../🎒️pack/🔤️json/📦️packages/🦀️rust" }
blake3 = "1.8.2"
semio-framework-async-macros = { path = "../../../⏳️async/✨️macros/📦️packages/🦀️rust" }

```

### 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts

SHA-256 `1cb22fecd35f0f35f5f98b1da9d44db924ff6f7d4d4f9395b8fcd784ed465da1`; 27283 bytes.

```
#!/usr/bin/env bun
import { runExactCargoLaws } from "../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🖥️ `semio-framework-replication` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

import { buildCargoArtifacts , readCargoArtifactBuildPolicyV1 } from "../../../🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import { blake3Hex } from "../../../🔏️hash/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-replication"],cwd:this.root,extraArgs:rest },readCargoTestPolicyV1(process.env));
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildCargoArtifacts(`${this.root}/Cargo.toml`, segments, readCargoArtifactBuildPolicyV1(process.env,this.root));
  }
}

class SourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../../🌱️value/🗂️ordered/🧪️tests/🧪️source-contract/🟦️.ts");
  }
}

class LocalInteractionSourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../📡️wire/🏠️local-interaction/🧪️tests/🧪️source-contract/🟦️.ts");
  }
}

class LocalInteractionNativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-replication"],cwd:this.root,extraArgs:["--lib","local_interaction_",...rest] },readCargoTestPolicyV1(process.env));
  }
}

/** 🔬️ Independent test-only SPR grammar shared by framing and retained-owner neutral laws. */
export function inspectRetainedSprNeutral(input: Buffer, checksum: (bytes: Uint8Array) => number, hash: (bytes: Buffer) => Buffer, limits = { fileBytes: 67108864, frameBodyBytes: 1048576, records: 8192 }): { end: number; sequence: number; frames: number; tail: number } {
  const fail = (reason: string): never => { throw new Error(reason); };
  if (input.length > limits.fileBytes) fail("capacity");
  if (input.length < 32 || !input.subarray(0, 8).equals(Buffer.from([137,83,80,82,13,10,26,10])) || input.readUInt16LE(8) !== 1 || input.readUInt16LE(10) !== 0
    || input.readUInt32LE(12) !== 1 || input.subarray(24, 32).some(byte => byte !== 0) || checksum(input.subarray(0, 20)) !== input.readUInt32LE(20)) fail("header");
  let cursor = 32; let committed = 32; let sequence = 0; let lastOffset = 0; let frames = 0; let committedFrames = 0;
  let pendingBytes = 0; const pending: Buffer[] = []; let chain = hash(input.subarray(0, 32));
  while (cursor < input.length) {
    const start = cursor; let length = 0n; let complete = false;
    for (let index = 0; index < 10; index++) {
      if (cursor === input.length) break;
      const byte = input[cursor++]!; if (index === 9 && byte > 1) fail("frame");
      length |= BigInt(byte & 127) << BigInt(7 * index);
      if (byte < 128) { if (index && byte === 0) fail("frame"); complete = true; break; }
    }
    if (!complete) break;
    if (length < 2n) fail("frame"); if (length > BigInt(limits.frameBodyBytes)) fail("capacity");
    const bodyStart = cursor; const bodyEnd = cursor + Number(length); const end = bodyEnd + 8;
    if (end > input.length) break;
    if (checksum(input.subarray(bodyStart, bodyEnd)) !== input.readUInt32LE(bodyEnd) || input.readUInt32LE(bodyEnd + 4) !== end - start) fail("frame");
    if (input[bodyStart] === 12 && input[bodyStart + 1] !== 2) fail("commit");
    const flags = input[bodyStart + 1]!; if ((flags & ~31) !== 0 || Boolean(flags & 1) !== Boolean(flags & 28)) fail("frame");
    if (input[bodyStart + 1]! & 1) {
      let raw = bodyStart + 2; let complete = false;
      for (let index = 0; index < 10; index++) {
        if (raw === bodyEnd) fail("frame");
        const byte = input[raw++]!; if (index === 9 && byte > 1) fail("frame");
        if (byte < 128) { if (index && byte === 0) fail("frame"); complete = true; break; }
      }
      if (!complete) fail("frame");
    }
    frames++; if (frames > limits.records) fail("capacity");
    if (input[bodyStart] === 12) {
      const payload = input.subarray(bodyStart + 2, bodyEnd);
      if (input[bodyStart + 1] !== 2 || payload.length !== 64 || end - start !== 75) fail("commit");
      const nextChain = hash(Buffer.concat([chain, ...pending]));
      if (payload.readBigUInt64LE(0) !== BigInt(sequence + 1) || payload.readBigUInt64LE(8) !== BigInt(lastOffset)
        || payload.readBigUInt64LE(16) !== BigInt(pendingBytes) || payload.readUInt32LE(24) !== pending.length
        || payload.subarray(28, 32).some(byte => byte !== 0) || !payload.subarray(32, 64).equals(nextChain)) fail("commit");
      committed = end; sequence++; lastOffset = start; committedFrames = frames; chain = nextChain; pending.length = 0; pendingBytes = 0;
    } else { pending.push(hash(input.subarray(start, end))); pendingBytes += end - start; }
    cursor = end;
  }
  return { end: committed, sequence, frames: committedFrames, tail: input.length - committed };
}

/** 🔎️ The retained-verification fixture: header, commit frames, resume cuts and every hostile denial. */
type RetainedVerificationFixture = {
  readonly schema: string;
  readonly profile: {
    readonly major: number;
    readonly minor: number;
    readonly requiredFlags: number;
    readonly canonicalVarints: boolean;
    readonly signed: boolean;
    readonly encrypted: boolean;
  };
  readonly limits: {
    readonly fileBytes: number;
    readonly frameBodyBytes: number;
    readonly records: number;
  };
  readonly headerHex: string;
  readonly commits: readonly {
    readonly sequence: number;
    readonly previousOffset: number;
    readonly offset: number;
    readonly end: number;
    readonly records: readonly {
      readonly kind: number;
      readonly flags: number;
      readonly payloadHex: string;
    }[];
    readonly coveredBytes: number;
    readonly recoveredFrames: number;
  }[];
  readonly fuelGrants: readonly number[];
  readonly resume: {
    readonly cuts: readonly number[];
    readonly record: {
      readonly kind: number;
      readonly flags: number;
      readonly payloadHex: string;
    };
    readonly addedBytes: number;
    readonly wrongSinkOffsets: readonly number[];
    readonly exhaustedSequence: string;
  };
  readonly negative: readonly (
    | { readonly id: string; readonly operation: "replace-first-length"; readonly hex: string; readonly error: string }
    | { readonly id: string; readonly operation: "record-limit"; readonly value: number; readonly error: string }
    | { readonly id: string; readonly operation: "file-limit"; readonly value: number; readonly error: string }
    | {
        readonly id: string;
        readonly operation: "header-xor" | "frame-xor" | "commit-xor" | "second-commit-xor";
        readonly offset: number;
        readonly value: number;
        readonly repairCrc: boolean;
        readonly error: string;
      }
  )[];
  readonly compressed: readonly (
    | { readonly id: string; readonly kind: number; readonly flags: number; readonly rawLengthHex: string; readonly storedHex: string; readonly rawHex: string; readonly error: null }
    | { readonly id: string; readonly kind: number; readonly flags: number; readonly rawLengthHex: string; readonly storedHex: string; readonly error: string }
  )[];
};

/** 🧾️ The retained-record fixture: one observation window per framed record, with its corruption outcome. */
type RetainedRecordFixture = {
  readonly schema: string;
  readonly grants: readonly number[];
  readonly headerHex: string;
  readonly cases: readonly {
    readonly id: string;
    readonly kind: number;
    readonly flags: number;
    readonly rawHex: string;
    readonly payloadHex: string;
    readonly repeat: number;
    readonly at: number;
    readonly cancel: boolean;
    readonly corruptCrc: boolean;
    readonly error: null | string;
    readonly observation: {
      readonly frameStart: number;
      readonly payloadStart: number;
      readonly payloadEnd: number;
      readonly frameEnd: number;
      readonly kind: number;
      readonly flags: number;
      readonly rawBytes: null | number;
    };
  }[];
};

export class RetainedVerificationScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("retained-verification-check accepts only --oracle-only");
    const { default: Ajv } = await import("ajv/dist/2020.js");
    const { default: crc } = await import("crc-32/crc32c.js");
    const { inflateRawSync } = await import("node:zlib");
    const leb = await import("@webassemblyjs/leb128");
    const owner = join(this.root, "../../📐️format/🔎️verification");
    const fixture: RetainedVerificationFixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true }); const validate = ajv.compile(schema);
    assert(validate(fixture), ajv.errorsText(validate.errors));
    const checksum = (bytes: Uint8Array): number => crc.buf(bytes) >>> 0;
    const hash = (bytes: Uint8Array): Buffer => Buffer.from(blake3Hex(bytes), "hex");
    assert.equal(checksum(Buffer.from("123456789")), 0xe3069283);
    assert.equal(blake3Hex(Buffer.from("abc")), "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85");
    const header = Buffer.alloc(32); Buffer.from([137,83,80,82,13,10,26,10]).copy(header);
    header.writeUInt16LE(1, 8); header.writeUInt32LE(1, 12); header.writeUInt32LE(1, 16);
    header.writeUInt32LE(checksum(header.subarray(0, 20)), 20);
    assert.equal(header.toString("hex"), fixture.headerHex);
    const frame = (kind: number, flags: number, payload: Buffer): Buffer => {
      const body = Buffer.concat([Buffer.from([kind, flags]), payload]);
      const size = Buffer.from(leb.encodeU32(body.length)); const tail = Buffer.alloc(8);
      tail.writeUInt32LE(checksum(body)); tail.writeUInt32LE(size.length + body.length + 8, 4);
      return Buffer.concat([size, body, tail]);
    };
    const parts: Buffer[] = [header]; let chain = hash(header); let offset = 32; let previous = 0; let recovered = 0;
    for (const commit of fixture.commits) {
      const records = commit.records.map((record: { kind: number; flags: number; payloadHex: string }) => frame(record.kind, record.flags, Buffer.from(record.payloadHex, "hex")));
      const covered = records.reduce((count: number, record: Buffer) => count + record.length, 0);
      assert.equal(covered, commit.coveredBytes); assert.equal(offset + covered, commit.offset); assert.equal(previous, commit.previousOffset);
      chain = hash(Buffer.concat([chain, ...records.map(hash)]));
      const payload = Buffer.alloc(64); payload.writeBigUInt64LE(BigInt(commit.sequence)); payload.writeBigUInt64LE(BigInt(previous), 8);
      payload.writeBigUInt64LE(BigInt(covered), 16); payload.writeUInt32LE(records.length, 24); chain.copy(payload, 32);
      const encoded = frame(12, 2, payload); assert.equal(encoded.length, 75);
      parts.push(...records, encoded); offset += covered + encoded.length; recovered += records.length + 1;
      assert.equal(offset, commit.end); assert.equal(recovered, commit.recoveredFrames); previous = commit.offset;
    }
    const bytes = Buffer.concat(parts);
    const inspect = (input: Buffer, limits = fixture.limits) => inspectRetainedSprNeutral(input, checksum, hash, limits);
    assert.deepEqual(inspect(bytes), { end: 255, sequence: 2, frames: 5, tail: 0 });
    for (const cut of fixture.resume.cuts) {
      const prior = inspect(bytes.subarray(0, cut));
      const last = fixture.commits.filter((row: { end: number }) => row.end <= cut).at(-1);
      const prefix = bytes.subarray(0, prior.end);
      const priorChain = last ? prefix.subarray(last.offset + 35, last.offset + 67) : hash(header);
      const record = fixture.resume.record;
      const encoded = frame(record.kind, record.flags, Buffer.from(record.payloadHex, "hex"));
      const payload = Buffer.alloc(64);
      payload.writeBigUInt64LE(BigInt(prior.sequence + 1)); payload.writeBigUInt64LE(BigInt(last?.offset ?? 0), 8);
      payload.writeBigUInt64LE(BigInt(encoded.length), 16); payload.writeUInt32LE(1, 24);
      hash(Buffer.concat([priorChain, hash(encoded)])).copy(payload, 32);
      const resumed = Buffer.concat([prefix, encoded, frame(12, 2, payload)]);
      assert.equal(resumed.length - prefix.length, fixture.resume.addedBytes);
      assert.deepEqual(resumed.subarray(0, prefix.length), prefix);
      assert.deepEqual(inspect(resumed), { end: resumed.length, sequence: prior.sequence + 1, frames: prior.frames + 2, tail: 0 });
    }
    let recoveryCases = 0;
    for (let end = 32; end <= bytes.length; end++) {
      const commit = fixture.commits.filter((row: { end: number }) => row.end <= end).at(-1);
      assert.deepEqual(inspect(bytes.subarray(0, end)), { end: commit?.end ?? 32, sequence: commit?.sequence ?? 0, frames: commit?.recoveredFrames ?? 0, tail: end - (commit?.end ?? 32) }); recoveryCases++;
    }
    const ids = new Set<string>();
    for (const row of fixture.negative) {
      assert(!ids.has(row.id)); ids.add(row.id); let mutated = Buffer.from(bytes); const limits = { ...fixture.limits };
      if (row.operation === "replace-first-length") mutated = Buffer.concat([mutated.subarray(0, 32), Buffer.from(row.hex, "hex"), mutated.subarray(33)]);
      else if (row.operation === "record-limit") limits.records = row.value;
      else if (row.operation === "file-limit") limits.fileBytes = row.value;
      else {
        const offset = row.operation === "commit-xor" ? 94 + row.offset : row.operation === "second-commit-xor" ? 183 + row.offset : row.offset; mutated[offset] ^= row.value;
        if (row.repairCrc) {
          if (row.operation === "header-xor") mutated.writeUInt32LE(checksum(mutated.subarray(0, 20)), 20);
          else if (row.operation === "second-commit-xor") mutated.writeUInt32LE(checksum(mutated.subarray(181, 247)), 247);
          else mutated.writeUInt32LE(checksum(mutated.subarray(92, 158)), 158);
        }
      }
      assert.throws(() => inspect(mutated, limits), new RegExp(`^Error: ${row.error}$`), row.id);
    }
    const extra = JSON.parse(JSON.stringify(fixture)) as { readonly commits: readonly { readonly records: readonly Record<string, unknown>[] }[] };
    extra.commits[0]!.records[0]!.unowned = true;
    assert(!validate(extra));
    for (const row of fixture.compressed) {
      assert(!ids.has(row.id)); ids.add(row.id);
      const rawLength = Buffer.from(row.rawLengthHex, "hex"); const stored = Buffer.from(row.storedHex, "hex");
      const encoded = frame(row.kind, row.flags, Buffer.concat([rawLength, stored]));
      const payload = Buffer.alloc(64); payload.writeBigUInt64LE(1n); payload.writeBigUInt64LE(BigInt(encoded.length), 16); payload.writeUInt32LE(1, 24);
      hash(Buffer.concat([hash(header), hash(encoded)])).copy(payload, 32);
      const committed = Buffer.concat([header, encoded, frame(12, 2, payload)]);
      if (row.error === null) {
        const raw = Buffer.from(row.rawHex, "hex"); assert.deepEqual(inflateRawSync(stored), raw);
        assert.deepEqual(rawLength, Buffer.from(leb.encodeU32(raw.length)));
        assert.deepEqual(inspect(committed), { end: committed.length, sequence: 1, frames: 2, tail: 0 });
      } else assert.throws(() => inspect(committed), new RegExp(`^Error: ${row.error}$`), row.id);
    }
    process.stdout.write(`independent retained SPR oracle: 2 commits, ${recoveryCases} exact LastCommit prefixes, ${fixture.negative.length} strict hostile denials, ${fixture.compressed.length} compressed grammar cases; no typed history publication\n`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const laws = ["retained_spr_verification_matches_neutral_commits_and_torn_prefixes", "retained_spr_verification_rejects_hostile_frames_without_publication", "retained_spr_resume_preserves_exact_prefix_and_commit_chain"];
    for (const law of laws) assert(source.includes(`fn ${law}(`), `missing retained SPR law ${law}`);
    assert(source.includes("fn verify_compressed_fixture(") && source.includes("verify_compressed_fixture(&fixture).await;"), "native compressed raw-length parity law is missing");
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub async fn resume_verified("), "protocol-owned verified writer resume is missing");
    assert.equal(BigInt(fixture.resume.exhaustedSequence) + 1n, 1n << 64n);
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("let next_commit_seq = self.next_commit_seq.checked_add(1)"), "commit sequence must reject exhaustion before writing");
    console.log(`retained SPR resume oracle: ${fixture.resume.cuts.length} exact prefixes, next sequence/previous offset/hash chain preserved`);
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub mod retained;"), "retained SPR module is not mounted; native selection cannot run");
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: laws.map(law => `format::retained::tests::${law}`) }] });
    assert.equal(receipts[0]!.assertions, laws.length);
  }
}

class RetainedRecordObservationScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("retained-record-observation-check accepts only --oracle-only");
    const { default: Ajv } = await import("ajv/dist/2020.js");
    const { default: crc } = await import("crc-32/crc32c.js");
    const leb = await import("@webassemblyjs/leb128");
    const owner = join(this.root, "../../📐️format/🔎️verification/🧾️record");
    const fixture: RetainedRecordFixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true }); const validate = ajv.compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), ajv.errorsText(validate.errors)); const ids = new Set<string>(); let observed = 0;
    const checksum = (bytes: Uint8Array): number => crc.buf(bytes) >>> 0;
    for (const row of fixture.cases) {
      assert(!ids.has(row.id)); ids.add(row.id);
      const body = Buffer.concat([Buffer.from([row.kind, row.flags]), Buffer.from(row.rawHex, "hex"), Buffer.from(row.payloadHex, "hex"), Buffer.alloc(row.repeat, 97)]);
      const length = Buffer.from(leb.encodeU32(body.length)); const tail = Buffer.alloc(8); tail.writeUInt32LE((checksum(body) ^ Number(row.corruptCrc)) >>> 0); tail.writeUInt32LE(length.length + body.length + 8, 4);
      const bytes = Buffer.concat([Buffer.from(fixture.headerHex, "hex"), length, body, tail]);
      const bodyStart = 32 + length.length; const payloadEnd = bodyStart + body.length;
      assert(row.at <= bytes.length); let readyAt = bodyStart + 2; let rawBytes: number | null = null;
      if (row.flags & 1) {
        const raw = Buffer.from(row.rawHex, "hex"); let value = 0n;
        for (let index = 0; index < raw.length; index++) value |= BigInt(raw[index]! & 127) << BigInt(7 * index);
        rawBytes = Number(value); assert.deepEqual(raw, Buffer.from(leb.encodeU32(rawBytes))); readyAt += raw.length;
      }
      let error: string | null = null;
      if (row.at >= bodyStart + 2 && ((row.flags & ~31) !== 0 || Boolean(row.flags & 1) !== Boolean(row.flags & 28))) error = "frame";
      if (row.at === bytes.length) {
        try { const span = inspectRetainedSprNeutral(bytes, checksum, value => Buffer.from(blake3Hex(value), "hex")); assert.equal(span.sequence, 0); assert.equal(span.end, 32); }
        catch (failure) { error = failure instanceof Error ? failure.message : "unknown"; }
      }
      if (row.cancel && error === null) error = "cancelled";
      const observation = error === null && row.at >= readyAt && row.at < bytes.length
        ? { frameStart: 32, payloadStart: readyAt, payloadEnd, frameEnd: bytes.length, kind: row.kind, flags: row.flags, rawBytes } : null;
      assert.equal(error, row.error, row.id); assert.deepEqual(observation, row.observation, row.id); if (observation) observed++;
    }
    const extra = JSON.parse(JSON.stringify(fixture)) as { readonly cases: readonly Record<string, unknown>[] };
    extra.cases[0]!.authority = true;
    assert(!validate(extra));
    console.log(`[TRACE] retained SPR observation oracle: ${fixture.cases.length} exact rows, ${observed} scalar observations; compressed raw-length/empty payload/clear/error/cancel; zero commit or input authority`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const law = "retained_record_observation_uses_the_existing_framing_state_without_authority";
    assert(source.includes('include_str!("🧫️fixtures/🔣️.json")') && source.includes(`fn ${law}(`));
    assert(source.includes("impl RetainedSprVerification") && !source.includes("fn push("), "metadata must observe the existing scanner, not parse a second framing grammar");
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub mod record;"), "retained record observation remains unmounted");
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: [`format::retained::record::tests::${law}`] }] });
    assert.equal(receipts[0]!.assertions, 1);
  }
}

/** 🛡️ Cross-language hostile-input oracle for the exact bounded PresencePeer codec. */
class PresencePeerCodecScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("presence-peer-codec-check accepts only --oracle-only");
    const { default: Ajv } = await import("ajv");
    const owner = join(this.root, "../../🧫️fixtures/👥️presence-peer-codec-v1");
    const fixture = JSON.parse(readFileSync(join(owner, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(this.root, "../../🧬️schema/🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/PresencePeerCodecFixture`)!;
    assert(validate(fixture), validate.errors?.map(error => `${error.instancePath} ${error.message}`).join("; "));
    const codec = await import(join(this.root, "../../🟦️.ts"));
    assert.deepEqual(codec.PRESENCE_PEER_WIRE_LIMITS_V1, fixture.limits);
    const ids = new Set<string>();
    for (const row of fixture.cases) {
      assert(!ids.has(row.id), `duplicate fixture id ${row.id}`); ids.add(row.id);
      const bytes = Buffer.concat([Buffer.from(row.prefixHex, "hex"), Buffer.alloc(row.repeatCount, Number.parseInt(row.repeatHex, 16)), Buffer.from(row.suffixHex, "hex")]);
      const position: [number] = [0];
      if (row.accepted) {
        const peer = codec.decodePresencePeer(bytes, position);
        assert.equal(position[0], bytes.length, row.id);
        assert.equal(Buffer.from(codec.encodePresencePeer(peer)).toString("hex"), row.canonicalHex, row.id);
        const semantic = JSON.parse(JSON.stringify(peer));
        if (peer.presencePack !== undefined) semantic.presencePack = Buffer.from(peer.presencePack).toString("base64");
        if (peer.interaction !== undefined) { semantic.interaction.appId = peer.interaction.app_id; delete semantic.interaction.app_id; }
        assert.deepEqual(semantic, row.expected, row.id);
      } else {
        assert.throws(() => codec.decodePresencePeer(bytes, position), Error, row.id);
        assert.equal(position[0], 0, `${row.id} advanced the caller cursor`);
      }
    }
    const extra = JSON.parse(JSON.stringify(fixture)) as { readonly cases: readonly Record<string, unknown>[] };
    extra.cases[0]!.authority = true;
    assert(!validate(extra));
    const source = readFileSync(join(this.root, "../../📡️wire/🦀️.rs"), "utf8");
    const tests = readFileSync(join(this.root, "../../📡️wire/🧪️tests/🔬️presence-codec/🦀️.rs"), "utf8");
    const laws = ["presence_peer_decoder_matches_neutral_bounded_exact_corpus", "presence_peer_decoder_rejects_hostile_counts_before_allocation"];
    for (const law of laws) assert(tests.includes(`fn ${law}(`), `missing native presence codec law ${law}`);
    assert(source.includes("PRESENCE_PEER_WIRE_LIMITS_V1") && source.includes("reader.position != bytes.len()"), "Rust bounded exact decoder is absent");
    console.log(`presence peer codec oracle: ${fixture.cases.length} neutral Rust/TypeScript vectors, ${fixture.cases.filter((row: { accepted: boolean }) => !row.accepted).length} hostile inputs rejected exactly`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: laws.map(law => `wire::frames::presence_codec_tests::${law}`) }] });
    assert.equal(receipts[0]!.assertions, laws.length);
  }
}

const router = new ScriptRouter(import.meta.dir).register("test", TestScript).register("build", BuildScript).register("test-source", SourceTestScript).register("test-local-interaction-source", LocalInteractionSourceTestScript).register("test-local-interaction-native", LocalInteractionNativeTestScript).register("retained-verification-check", RetainedVerificationScript).register("retained-record-observation-check", RetainedRecordObservationScript).register("presence-peer-codec-check", PresencePeerCodecScript);

if (import.meta.main) await runScriptMain(router, { defaultCommand: "test" });

```

### 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🟦️.ts

SHA-256 `6d122cc1463e8430cdf5c2b8730e97c3504a39e54c5a0615fa2a6bd79dd79048`; 10835 bytes.

```
import { readFileSync, mkdirSync, mkdtempSync, writeFileSync, rmSync } from "node:fs";
import { join, isAbsolute } from "node:path";
import { StringDecoder } from "node:string_decoder";
import { validateJsonSchemaSubset } from "../../../🧬️schema/✅️validator/🟦️.ts";
import { TEST_LEVELS, isTestLevel, type TestLevel } from "../🎚️budget/🟦️.ts";
import { runBudgetedTestCommand } from "../🎛️execution/🟦️.ts";
import { startNativeProgress } from "../../🎛️owned-execution/🟦️.ts";

/** 🦀️ Binds a Cargo invocation to its exact caller-owned execution policy. */
export type CargoTestPolicyV1 = Readonly<{version:1;manifestPath:string;targetDirectory:string;nextest:boolean;configPath:string;level:TestLevel;assertionBudgets:Readonly<Record<TestLevel,number>>;buildBudgetMs:number;assertionThreads:number;artifactDirectory:string;retainArtifacts:boolean;coverageEnabled:boolean;coveragePath:string|null;rustMinStack:string}>;
/** 📋️ Selects an exact manifest and package inventory without repository discovery. */
export type CargoTestRequestV1 = Readonly<{manifestPath:string;packages:readonly string[];cwd:string;extraArgs?:readonly string[];environment?:Readonly<Record<string,string|undefined>>;signal?:AbortSignal}>;
/** 🎬️ Describes a compiler, assertion, or report invocation under an explicit budget. */
export type CargoTestStepV1 = Readonly<{phase:"build"|"assert"|"report";args:string[];budgetMs:number;capture:boolean}>;
const policySchema=JSON.parse(readFileSync(new URL("./🧬️schema/🔣️.json",import.meta.url),"utf8"));

/** 🔐️ Refuses absent or malformed execution policies before starting a compiler. */
export function readCargoTestPolicyV1(environment:Readonly<Record<string,string|undefined>>):CargoTestPolicyV1 {
  const serialized=environment.SEMIO_CARGO_TEST_POLICY;
  if(!serialized) throw Error("Explicit Cargo test policy required");
  const policy=admitPolicy(JSON.parse(serialized)),level=isTestLevel(environment.SEMIO_TEST_LEVEL)?environment.SEMIO_TEST_LEVEL:policy.level;
  return {...policy,level,coverageEnabled:environment.SEMIO_COVERAGE===undefined?policy.coverageEnabled:environment.SEMIO_COVERAGE==="1"};
}
function admitPolicy(value:unknown):CargoTestPolicyV1 { const errors=validateJsonSchemaSubset(policySchema,value);if(errors.length)throw Error(`Invalid Cargo test policy: ${errors.join("; ")}`);return value as CargoTestPolicyV1; }

/** 🧪️ Preserves build selection on compilation and runtime filters on metadata execution. */
export function partitionNextestExecutionFilters(args: readonly string[]): { buildArgs: string[]; executionArgs: string[]; libtestArgs: string[] } {
  const valuedFilters = new Set(["-E", "--filter-expr", "--partition", "--run-ignored"]);
  const requiredBuildOptions = new Set([
    "-p",
    "--package",
    "--exclude",
    "--manifest-path",
    "--target",
    "--target-dir",
    "--features",
    "-F",
    "--jobs",
    "-j",
    "--build-jobs",
    "--cargo-profile",
    "--cargo-message-format",
    "--config",
    "-Z",
    "--color",
    "--profile",
    "-P",
    "--test",
    "--bin",
    "--bench",
    "--example",
    "--message-format",
    "-T",
    "--list-type",
    "--archive-file",
    "--archive-format",
    "--extract-to",
    "--cargo-metadata",
    "--workspace-remap",
    "--binaries-metadata",
    "--target-dir-remap",
    "--build-dir-remap",
    "--config-file",
    "--user-config-file",
    "--tool-config-file",
  ]);
  const optionalBuildOptions = new Set(["--timings"]);
  const joinedBuildOptions = ["-p", "-F", "-j", "-Z", "-P", "-T"];
  const buildArgs: string[] = [],
    executionArgs: string[] = [];
  const separator = args.indexOf("--");
  const cargoArgs = separator < 0 ? args : args.slice(0, separator);
  const libtestArgs = separator < 0 ? [] : args.slice(separator + 1);
  for (let index = 0; index < cargoArgs.length; index += 1) {
    const arg = cargoArgs[index]!;
    const key = arg.split("=", 1)[0]!;
    if (arg === "--ignore-default-filter" || arg === "--no-fail-fast" || (arg.startsWith("-E") && arg.length > 2)) {
      executionArgs.push(arg);
    } else if (valuedFilters.has(key)) {
      if (arg.includes("=")) {
        if (arg.endsWith("=")) throw new Error(`Nextest filter ${key} requires a value`);
        executionArgs.push(arg);
      } else {
        const value = cargoArgs[index + 1];
        if (value === undefined || value.length === 0 || value.startsWith("-")) throw new Error(`Nextest filter ${key} requires a value`);
        executionArgs.push(arg, value);
        index += 1;
      }
    } else {
      if (!arg.startsWith("-")) executionArgs.push(arg);
      else {
        buildArgs.push(arg);
        const inlineValue = arg.includes("=") || joinedBuildOptions.some((option) => arg.startsWith(option) && arg.length > option.length);
        const requiresValue = requiredBuildOptions.has(key);
        const allowsValue = optionalBuildOptions.has(key);
        if ((requiresValue || allowsValue) && arg.endsWith("=")) throw new Error(`Nextest build option ${key} requires a non-empty value`);
        if (!inlineValue && requiresValue) {
          const value = cargoArgs[index + 1];
          if (value !== undefined && value.length > 0 && !value.startsWith("-")) {
            buildArgs.push(value);
            index += 1;
          } else if (requiresValue) throw new Error(`Nextest build option ${key} requires a value`);
        }
      }
    }
  }
  return { buildArgs, executionArgs, libtestArgs };
}


/** 🧭️ Projects one exact Cargo request into bounded build, assertion and coverage operations. */
export function cargoTestPlanV1(request:CargoTestRequestV1, input:CargoTestPolicyV1, metadataPath:string):CargoTestStepV1[] {
  const policy=admitPolicy(input);
  if(request.manifestPath!==policy.manifestPath||!isAbsolute(policy.configPath)||policy.coveragePath!==null&&!isAbsolute(policy.coveragePath)||!isAbsolute(request.manifestPath)||!isAbsolute(request.cwd)||!isAbsolute(metadataPath)||request.packages.some(name=>!name || name.startsWith("-"))) throw Error("Cargo tests require exact absolute manifest, working directory and metadata paths");
  const packages=["--manifest-path",request.manifestPath,...request.packages.flatMap(name=>["-p",name])], split=partitionNextestExecutionFilters(request.extraArgs??[]), skip=TEST_LEVELS.slice(TEST_LEVELS.indexOf(policy.level)+1).flatMap(level=>["--skip",`${level}::`]), profile=["--config-file",policy.configPath,"--profile",policy.level];
  const step=(phase:CargoTestStepV1["phase"],args:string[],capture=false):CargoTestStepV1=>({phase,args,budgetMs:phase==="assert"?policy.assertionBudgets[policy.level]:policy.buildBudgetMs,capture});
  if(policy.coverageEnabled){
    if(!policy.coveragePath)throw Error("Coverage requires an explicit owner report path");
    const operation=policy.nextest?"nextest":"test", args=["--release","--no-report",...(policy.nextest?["--no-tests","fail",...profile]:[]),...packages,...split.buildArgs,...split.executionArgs,"--",...split.libtestArgs,...skip];
    return [step("build",policy.nextest?["llvm-cov",operation,"--no-run",...args]:["llvm-cov",operation,...args,"--list"]),step("assert",["llvm-cov",operation,"--no-clean",...args]),step("report",["llvm-cov","report","--release","--lcov",...packages,"--output-path",policy.coveragePath])];
  }
  if(policy.nextest) return [step("build",["nextest","list","--list-type","binaries-only","--message-format","json",...profile,...packages,...split.buildArgs],true),step("assert",["nextest","run","--binaries-metadata",metadataPath,"--no-tests","fail","--status-level","fail","--final-status-level","fail",...(policy.level==="fundamental"?["--test-threads",String(policy.assertionThreads)]:[]),...profile,"--manifest-path",request.manifestPath,...split.executionArgs,"--",...split.libtestArgs,...skip])];
  return [step("build",["build","--tests",...packages,...split.buildArgs]),step("assert",["test",...packages,...split.buildArgs,...split.executionArgs,"--",...split.libtestArgs,...skip])];
}

/** 🔌️ Supplies a caller-owned executable for the Cargo command grammar. */
export type CargoTestExecutionPortV1=Readonly<{command:string;args:readonly string[]}>;

/** 🏃️ Executes the admitted plan, retaining metadata only when its caller explicitly requests it. */
export async function runCargoTestsV1(request:CargoTestRequestV1,input:CargoTestPolicyV1,port:CargoTestExecutionPortV1={command:"cargo",args:[]}):Promise<void>{
  const policy=admitPolicy(input);
  if(!isAbsolute(policy.artifactDirectory)) throw Error("Cargo artifact directory must be absolute");
  mkdirSync(policy.artifactDirectory,{recursive:true});
  const directory=mkdtempSync(join(policy.artifactDirectory,"semio-nextest-")), metadata=join(directory,"binaries-metadata.json"), environment=request.environment??process.env, env={...environment,RUST_MIN_STACK:environment.RUST_MIN_STACK??policy.rustMinStack};
  try {for(const step of cargoTestPlanV1(request,policy,metadata)){
    const stop=startNativeProgress(`cargo:${step.phase}`), decoder=new StringDecoder("utf8");let output="";
    try {await runBudgetedTestCommand(port.command,[...port.args,...step.args],{cwd:request.cwd,env,signal:request.signal,budgetMs:step.budgetMs,throwOnFailure:true,...(step.capture?{captureStdout:{limitBytes:536870912,onChunk:(bytes:Uint8Array)=>{output+=decoder.write(Buffer.from(bytes));}}}:{})});if(step.capture)writeFileSync(metadata,output+decoder.end());}
    finally {stop();}
  }} finally {if(policy.retainArtifacts)console.error(`[TRACE] Nextest artifacts retained at ${directory}`);else rmSync(directory,{recursive:true,force:true});}
}

/** 🧹️ Projects exact owner Clippy selection while keeping warning policy out of compiler flags. */
export function cargoLintPlanV1(request:CargoTestRequestV1,input:CargoTestPolicyV1):CargoTestStepV1{
  const policy=admitPolicy(input);
  cargoTestPlanV1({...request,extraArgs:[]},policy,join(policy.artifactDirectory,"lint-metadata.json"));
  return {phase:"build",args:["clippy","--manifest-path",request.manifestPath,...request.packages.flatMap(name=>["-p",name]),"--all-targets",...(request.extraArgs??[]),"--","-D","warnings"],budgetMs:policy.buildBudgetMs,capture:false};
}

/** 🛠️ Executes one exact manifest's Clippy plan under its existing build budget. */
export async function runCargoLintV1(request:CargoTestRequestV1,policy:CargoTestPolicyV1,port:CargoTestExecutionPortV1={command:"cargo",args:[]}):Promise<void>{
  const step=cargoLintPlanV1(request,policy),stop=startNativeProgress("cargo:lint");
  try {await runBudgetedTestCommand(port.command,[...port.args,...step.args],{cwd:request.cwd,env:request.environment??process.env,signal:request.signal,budgetMs:step.budgetMs,throwOnFailure:true});}finally{stop();}
}

```

### 🧰️framework/🔨️modules/🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts

SHA-256 `1bdae6420e2034d98f3b26b11f56b501d1018670896f61d085fb6af4e538e0b3`; 16944 bytes.

```
import {captureOwnedProcess,type OwnedProcessCaptureOptions,type OwnedProcessCaptureResult} from "../../../📥️capture/🟦️.ts";
import { closeSync, existsSync, fstatSync, lstatSync, mkdirSync, mkdtempSync, openSync, readFileSync, readSync, readdirSync, realpathSync, rmSync, writeFileSync } from "node:fs";
import { isAbsolute, join, resolve, sep } from "node:path";
import { createHash } from "node:crypto";
import { buildBudgetMs } from "../../../⏱️budget/🟦️.ts";

export type ExactCargoLawGroup = {
  package: string;
  target: { kind: "lib"; name?: string } | { kind: "test" | "bin"; name: string };
  laws: readonly string[];
  cargoArgs?: readonly string[];
};
export type ExactCargoLawStage = "build" | "list" | "native";
/** 🔌️ Owned process/fingerprint port permits deterministic hostile runner laws without compiling Cargo. */
export type ExactCargoLawPort = {
  probe: (command: string, args: string[], options: OwnedProcessCaptureOptions) => Promise<OwnedProcessCaptureResult>;
  fingerprint: (path: string) => { path: string; sha256: string };
};
export type ExactCargoLawOptions = {
  cwd: string;
  groups: readonly ExactCargoLawGroup[];
  manifestPaths: Readonly<Record<string, string>>;
  cargoTargetDir: string;
  cargoArgs?: readonly string[];
  env?: Readonly<Record<string, string | undefined>>;
  nativeEnv?: Readonly<Record<string, string | undefined>>;
  artifactDir?: string;
  buildBudgetMs?: number;
  listBudgetMs?: number;
  lawBudgetMs?: number;
  cancelled?: () => boolean;
  progress?: (event: { stage: ExactCargoLawStage; package: string; law?: string; artifactDir: string }) => void;
};
export type ExactCargoLawReceipt = {
  package: string;
  target: ExactCargoLawGroup["target"];
  executable: string;
  sha256: string;
  laws: readonly string[];
  assertions: number;
  artifactDir: string;
  cargoTargetDir: string;
};

export const EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX = ".exact-cargo-laws-active-";
export const EXACT_CARGO_ACTIVE_LEASE_MANIFEST = "lease.json";
export const EXACT_CARGO_ACTIVE_LEASE_MAX_AGE_MS = 120_000;

/** 🛡️ Recognizes only a fresh lease owned by a live exact-Cargo runner process. */
export function exactCargoGeneratedOutputHasLiveLease(root: string): boolean {
  const stack = [{ path: root, depth: 0 }];
  while (stack.length > 0) {
    const current = stack.pop()!;
    let names: string[];
    try {
      names = readdirSync(current.path);
    } catch {
      continue;
    }
    for (const name of names) {
      const path = join(current.path, name);
      let state;
      try {
        state = lstatSync(path);
      } catch {
        continue;
      }
      if (!state.isDirectory() || state.isSymbolicLink()) continue;
      if (name.startsWith(EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX)) {
        const manifestPath = join(path, EXACT_CARGO_ACTIVE_LEASE_MANIFEST);
        try {
          const manifestState = lstatSync(manifestPath);
          if (!manifestState.isFile() || manifestState.isSymbolicLink() || manifestState.size > 128 || Date.now() - manifestState.mtimeMs > EXACT_CARGO_ACTIVE_LEASE_MAX_AGE_MS || manifestState.mtimeMs - Date.now() > 5_000) continue;
          const manifest = JSON.parse(readFileSync(manifestPath, "utf8")) as { version?: unknown; pid?: unknown };
          if (manifest.version !== 1 || !Number.isSafeInteger(manifest.pid) || Number(manifest.pid) < 1) continue;
          try {
            process.kill(Number(manifest.pid), 0);
            return true;
          } catch (error) {
            if ((error as NodeJS.ErrnoException).code === "EPERM") return true;
          }
        } catch {
          continue;
        }
      } else if (current.depth < 4) stack.push({ path, depth: current.depth + 1 });
    }
  }
  return false;
}

/** 💓 Holds a fresh process-bound lease until the exact Cargo run reaches a terminal result. */
function beginExactCargoLease(artifactRoot: string): () => void {
  const leaseRoot = mkdtempSync(join(artifactRoot, EXACT_CARGO_ACTIVE_LEASE_DIRECTORY_PREFIX));
  const manifestPath = join(leaseRoot, EXACT_CARGO_ACTIVE_LEASE_MANIFEST);
  const heartbeat = (): void => {
    try {
      writeFileSync(manifestPath, JSON.stringify({ version: 1, pid: process.pid }), { mode: 0o600 });
    } catch {}
  };
  heartbeat();
  const timer = setInterval(heartbeat, 10_000);
  timer.unref?.();
  return () => {
    clearInterval(timer);
    rmSync(leaseRoot, { recursive: true, force: true });
  };
}

/** 🚫️ Preserves the precise failing stage and actual child status independently of assertion parsing. */
export class ExactCargoLawError extends Error {
  constructor(
    readonly stage: ExactCargoLawStage,
    readonly status: number | null,
    readonly signal: string | null,
    readonly artifactDir: string,
    detail: string,
  ) {
    super(`exact Cargo law ${stage} failed: status=${status} signal=${signal ?? "none"} artifacts=${artifactDir}; ${detail}`);
  }
}

/** 🔬️ Fingerprints one retained executable descriptor with bounded streaming and cancellation. */
export function exactExecutableFingerprint(path: string, control: Readonly<{ cancelled?: () => boolean; progress?: (completed: number, total: number) => void }> = {}): { path: string; sha256: string; byteLength: number } {
  const check = () => {
    if (control.cancelled?.()) throw new Error("Executable fingerprint cancelled");
  };
  check();
  if (!isAbsolute(path) || lstatSync(path).isSymbolicLink()) throw new Error("Executable must be one absolute regular file");
  const canonical = realpathSync(path);
  const descriptor = openSync(canonical, "r");
  try {
    const before = fstatSync(descriptor);
    const same = (other: typeof before) => other.isFile() && !other.isSymbolicLink() && other.dev === before.dev && other.ino === before.ino && other.size === before.size && other.mtimeMs === before.mtimeMs && other.ctimeMs === before.ctimeMs;
    if (!before.isFile() || !same(lstatSync(canonical)) || before.size <= 0 || before.size > 8 * 1024 ** 3 || (process.platform !== "win32" && (before.mode & 0o111) === 0)) throw new Error("Executable size or type denied");
    const digest = createHash("sha256");
    const buffer = Buffer.alloc(64 * 1024);
    let count = 0;
    while (true) {
      check();
      const length = readSync(descriptor, buffer);
      if (length === 0) break;
      digest.update(buffer.subarray(0, length));
      count += length;
      if (count > before.size) throw new Error("Executable changed while hashing");
      control.progress?.(count, before.size);
    }
    const after = fstatSync(descriptor);
    if (count !== before.size || !same(after) || !same(lstatSync(canonical))) throw new Error("Executable changed while hashing");
    return { path: canonical, sha256: digest.digest("hex"), byteLength: count };
  } finally {
    closeSync(descriptor);
  }
}

/** 🧪️ Compiles each explicit target once and executes only its hash-bound, exact-listed native laws. */
export async function runExactCargoLaws(options: ExactCargoLawOptions, port: ExactCargoLawPort = { probe: captureOwnedProcess, fingerprint: exactExecutableFingerprint }): Promise<readonly ExactCargoLawReceipt[]> {
  const configuredEnv = options.env ?? process.env;
  const artifactRoot = options.artifactDir ?? configuredEnv.SEMIO_TEST_ARTIFACT_DIR;
  if (!artifactRoot || !isAbsolute(artifactRoot)) throw new Error("Exact Cargo laws require an absolute artifactDir or SEMIO_TEST_ARTIFACT_DIR");
  const cargoTargetDir = options.cargoTargetDir;
  if (!cargoTargetDir || !isAbsolute(cargoTargetDir)) throw new Error("Explicit absolute Cargo target directory required");
  const targetBoundary = process.platform === "win32" ? cargoTargetDir.toLowerCase() : cargoTargetDir;
  const sourceBoundary = process.platform === "win32" ? resolve(options.cwd).toLowerCase() : resolve(options.cwd);
  if (sourceBoundary === targetBoundary || sourceBoundary.startsWith(targetBoundary + sep)) throw new Error("Cargo target must not contain the source workspace");
  const env = { ...configuredEnv, CARGO_TARGET_DIR: cargoTargetDir };
  const nativeEnv: Record<string, string | undefined> = { ...env, ...options.nativeEnv, CARGO_TARGET_DIR: cargoTargetDir };
  delete nativeEnv.RUST_TEST_NOCAPTURE;
  if (!isAbsolute(options.cwd) || !options.groups.length || options.groups.length > 64) throw new Error("Exact Cargo laws require a bounded nonempty target list and absolute cwd");
  const groupKeys = options.groups.map((group) => JSON.stringify([group.package, group.target.kind, group.target.name ?? ""]));
  if (new Set(groupKeys).size !== groupKeys.length) throw new Error("Exact Cargo groups must combine laws for the same package/target");
  for (const group of options.groups) {
    if (!isAbsolute(options.manifestPaths[group.package] ?? "")) throw new Error("Explicit absolute Cargo manifest required for each package");
    if (!group.package || !group.laws.length || group.laws.length > 4096 || new Set(group.laws).size !== group.laws.length || group.laws.some((law) => !/^[A-Za-z_][A-Za-z0-9_:]*$/u.test(law)))
      throw new Error("Exact Cargo law identities must be nonempty and unique");
  }
  mkdirSync(artifactRoot, { recursive: true });
  const endLease = beginExactCargoLease(artifactRoot);
  try {
    const runRoot = mkdtempSync(join(artifactRoot, "exact-cargo-laws-"));
    const cancelled = options.cancelled ?? (() => false);
    const receipts: ExactCargoLawReceipt[] = [];
    const checkedBudget = (value: number, build: boolean): number => {
      if (!Number.isSafeInteger(value) || value < (build ? 0 : 1) || value > 24 * 60 * 60 * 1000) throw new Error("Exact Cargo budget must be finite and positive, or zero for builds");
      return value;
    };
    for (const [index, group] of options.groups.entries()) {
      const groupRoot = join(runRoot, String(index).padStart(2, "0"));
      mkdirSync(groupRoot);
      let stage: ExactCargoLawStage = "build";
      let last: OwnedProcessCaptureResult = { status: null, signal: null, stdout: "", stderr: "" };
      const fail = (detail: string): never => {
        throw new ExactCargoLawError(stage, last.status, last.signal, groupRoot, detail);
      };
      const checkpoint = (): void => {
        if (cancelled()) fail("cancelled");
      };
      const capture = async (next: ExactCargoLawStage, command: string, args: string[], budget: number, name: string): Promise<OwnedProcessCaptureResult> => {
        stage = next;
        checkpoint();
        options.progress?.({ stage, package: group.package, ...(next === "native" ? { law: args[0] } : {}), artifactDir: groupRoot });
        const stdoutPath = join(groupRoot, `${name}.stdout`);
        const stderrPath = join(groupRoot, `${name}.stderr`);
        last = await port.probe(command, args, { cwd: options.cwd, env: next === "build" ? env : nativeEnv, budgetMs: checkedBudget(budget, next === "build"), maxOutputBytes: next === "build" ? 256 * 1024 * 1024 : 8 * 1024 * 1024, stdoutPath, stderrPath, cancelled });
        if (!existsSync(stdoutPath)) writeFileSync(stdoutPath, last.stdout, { flag: "wx", mode: 0o600 });
        if (!existsSync(stderrPath)) writeFileSync(stderrPath, last.stderr, { flag: "wx", mode: 0o600 });
        writeFileSync(join(groupRoot, `${name}.json`), JSON.stringify({ command, args, cargoTargetDir, status: last.status, signal: last.signal, reason: last.reason ?? "exit" }), { flag: "wx", mode: 0o600 });
        checkpoint();
        return last;
      };
      const target = group.target.kind === "lib" ? ["--lib"] : [`--${group.target.kind}`, group.target.name];
      const cargoArgs = [...(options.cargoArgs ?? []), ...(group.cargoArgs ?? [])];
      if (
        cargoArgs.some(
          (arg) =>
            ["--", "--test", "--bin", "--lib", "-p", "--package", "--no-run", "--message-format", "--target-dir", "--manifest-path"].includes(arg) || ["--message-format=", "--target-dir=", "--manifest-path="].some((prefix) => arg.startsWith(prefix)),
        )
      )
        fail("Cargo target/control arguments are helper-owned");
      const built = await capture(
        "build",
        "cargo",
        ["test", "--manifest-path", options.manifestPaths[group.package]!, "-p", group.package, ...target, ...cargoArgs, "--no-run", "--message-format=json"],
        options.buildBudgetMs ?? buildBudgetMs(),
        "build",
      );
      const messages = built.stdout.split("\n").flatMap((line) => {
        try {
          return [JSON.parse(line)];
        } catch {
          return [];
        }
      });
      const errors = messages.filter((message) => message.reason === "compiler-message" && message.message?.level === "error").map((message) => message.message.rendered ?? message.message.message);
      if (built.status !== 0 || built.signal !== null || (built.reason && built.reason !== "exit")) fail(`${built.reason ?? "exit"}; ${(errors.length ? errors.slice(0, 3).join("\n") : built.stderr).slice(0, 6000)}`);
      const artifacts = messages.filter((message) => message.reason === "compiler-artifact" && message.profile?.test === true && typeof message.executable === "string");
      if (artifacts.length !== 1) fail(`expected one Cargo executable artifact, got ${artifacts.length}`);
      const artifact = artifacts[0];
      const packageId = String(artifact.package_id);
      const packageName = packageId.includes("#") ? packageId.slice(packageId.lastIndexOf("#") + 1).split("@")[0] : packageId.split(" ")[0];
      const kinds = artifact.target?.kind;
      if (
        packageName !== group.package ||
        !Array.isArray(kinds) ||
        !kinds.some((kind) => (group.target.kind === "lib" ? ["lib", "rlib", "dylib", "cdylib", "staticlib", "proc-macro"].includes(kind) : kind === group.target.kind)) ||
        (group.target.name && artifact.target?.name !== group.target.name) ||
        !isAbsolute(artifact.executable)
      )
        fail("Cargo executable package/target/path does not match the explicit group");
      const fingerprint = (): { path: string; sha256: string } => {
        try {
          return port.fingerprint(artifact.executable);
        } catch (error) {
          return fail(String(error));
        }
      };
      const initial = fingerprint();
      const verify = (): void => {
        checkpoint();
        const current = fingerprint();
        if (current.path !== initial.path || current.sha256 !== initial.sha256) fail("Cargo executable changed after its build receipt");
      };
      if (!isAbsolute(initial.path) || !/^[0-9a-f]{64}$/u.test(initial.sha256)) fail("Cargo executable fingerprint is invalid");
      writeFileSync(join(groupRoot, "executable.json"), JSON.stringify({ package: group.package, target: group.target, ...initial }), { flag: "wx", mode: 0o600 });
      verify();
      const listed = await capture("list", initial.path, ["--list"], options.listBudgetMs ?? 60_000, "list");
      verify();
      if (listed.status !== 0 || listed.signal !== null || (listed.reason && listed.reason !== "exit")) fail(`list ${listed.reason ?? "exit"}; ${listed.stderr.slice(0, 4000)}`);
      const discovered = listed.stdout
        .split(/\r?\n/u)
        .filter((line) => line.endsWith(": test"))
        .map((line) => line.slice(0, -6));
      const laws = group.laws.map((selector) => {
        const matches = discovered.filter((name) => name === selector || name.endsWith(`::${selector}`));
        if (matches.length !== 1) fail(`expected exactly one ${selector}, selected=${matches.length}`);
        return matches[0]!;
      });
      if (new Set(laws).size !== laws.length) fail("Law selectors resolve to the same native assertion");
      for (const [lawIndex, law] of laws.entries()) {
        verify();
        const result = await capture("native", initial.path, [law, "--exact", "--test-threads=1", "--show-output"], options.lawBudgetMs ?? 60_000, `law-${lawIndex}`);
        verify();
        const terminals = [...result.stdout.matchAll(/^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored;/gm)];
        if (
          result.status !== 0 ||
          result.signal !== null ||
          (result.reason && result.reason !== "exit") ||
          terminals.length !== 1 ||
          terminals[0]?.[1] !== "1" ||
          terminals[0]?.[2] !== "0" ||
          terminals[0]?.[3] !== "0" ||
          !result.stdout.split(/\r?\n/u).includes(`test ${law} ... ok`)
        )
          fail(`native assertion ${law} did not pass exactly once; ${(result.stdout + result.stderr).slice(-6000)}`);
      }
      const receipt = { package: group.package, target: group.target, executable: initial.path, sha256: initial.sha256, laws, assertions: laws.length, artifactDir: groupRoot, cargoTargetDir };
      writeFileSync(join(groupRoot, "receipt.json"), JSON.stringify(receipt), { flag: "wx", mode: 0o600 });
      receipts.push(receipt);
    }
    return receipts;
  } finally {
    endLease();
  }
}


```

### Proposed successor1 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📜️script.ts

```
#!/usr/bin/env bun
import { runExactCargoLaws } from "../../../🏃️process/🧪️testing/🦀️cargo/🎯️exact/🟦️.ts";
import { resolveTestLevel } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
/** 🖥️ `semio-framework-replication` task router: `bun ./📜️script.ts test [quick|long|exhaustive] [args…]`. */
import { runBudgetedTestCommand } from "../../../🏃️process/🧪️testing/🎛️execution/🟦️.ts";
import { testLevelBudgetMs } from "../../../🏃️process/🧪️testing/🎚️budget/🟦️.ts";
import { cmdBudgetMs } from "../../../🏃️process/⏱️budget/🟦️.ts";
import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import { join, resolve } from "node:path";
import { runCargoTestsV1, readCargoTestPolicyV1 } from "../../../🏃️process/🧪️testing/🦀️cargo/🟦️.ts";

import { BundleScript, ScriptRouter } from "../../../🏃️process/🧭️routing/🟦️.ts";
import { runScriptMain } from "../../../🏃️process/🧭️routing/🚪️entrypoint/🟦️.ts";

import { buildCargoArtifacts , readCargoArtifactBuildPolicyV1 } from "../../../🏃️process/📦️artifacts/🏗️native-build/🟦️.ts";
import { blake3Hex } from "../../../🔏️hash/🟦️.ts";

class TestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-replication"],cwd:this.root,extraArgs:rest },readCargoTestPolicyV1(process.env));
  }
}

class BuildScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    await buildCargoArtifacts(`${this.root}/Cargo.toml`, segments, readCargoArtifactBuildPolicyV1(process.env,this.root));
  }
}

class SourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../../🌱️value/🗂️ordered/🧪️tests/🧪️source-contract/🟦️.ts");
  }
}

class LocalInteractionSourceTestScript extends BundleScript {
  async run(): Promise<void> {
    await import("../../📡️wire/🏠️local-interaction/🧪️tests/🧪️source-contract/🟦️.ts");
  }
}

class LocalInteractionNativeTestScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    const { rest } = resolveTestLevel(segments);
    await runCargoTestsV1({ manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-replication"],cwd:this.root,extraArgs:["--lib","local_interaction_",...rest] },readCargoTestPolicyV1(process.env));
  }
}

/** 🔬️ Independent test-only SPR grammar shared by framing and retained-owner neutral laws. */
export function inspectRetainedSprNeutral(input: Buffer, checksum: (bytes: Uint8Array) => number, hash: (bytes: Buffer) => Buffer, limits = { fileBytes: 67108864, frameBodyBytes: 1048576, records: 8192 }): { end: number; sequence: number; frames: number; tail: number } {
  const fail = (reason: string): never => { throw new Error(reason); };
  if (input.length > limits.fileBytes) fail("capacity");
  if (input.length < 32 || !input.subarray(0, 8).equals(Buffer.from([137,83,80,82,13,10,26,10])) || input.readUInt16LE(8) !== 1 || input.readUInt16LE(10) !== 0
    || input.readUInt32LE(12) !== 1 || input.subarray(24, 32).some(byte => byte !== 0) || checksum(input.subarray(0, 20)) !== input.readUInt32LE(20)) fail("header");
  let cursor = 32; let committed = 32; let sequence = 0; let lastOffset = 0; let frames = 0; let committedFrames = 0;
  let pendingBytes = 0; const pending: Buffer[] = []; let chain = hash(input.subarray(0, 32));
  while (cursor < input.length) {
    const start = cursor; let length = 0n; let complete = false;
    for (let index = 0; index < 10; index++) {
      if (cursor === input.length) break;
      const byte = input[cursor++]!; if (index === 9 && byte > 1) fail("frame");
      length |= BigInt(byte & 127) << BigInt(7 * index);
      if (byte < 128) { if (index && byte === 0) fail("frame"); complete = true; break; }
    }
    if (!complete) break;
    if (length < 2n) fail("frame"); if (length > BigInt(limits.frameBodyBytes)) fail("capacity");
    const bodyStart = cursor; const bodyEnd = cursor + Number(length); const end = bodyEnd + 8;
    if (end > input.length) break;
    if (checksum(input.subarray(bodyStart, bodyEnd)) !== input.readUInt32LE(bodyEnd) || input.readUInt32LE(bodyEnd + 4) !== end - start) fail("frame");
    if (input[bodyStart] === 12 && input[bodyStart + 1] !== 2) fail("commit");
    const flags = input[bodyStart + 1]!; if ((flags & ~31) !== 0 || Boolean(flags & 1) !== Boolean(flags & 28)) fail("frame");
    if (input[bodyStart + 1]! & 1) {
      let raw = bodyStart + 2; let complete = false;
      for (let index = 0; index < 10; index++) {
        if (raw === bodyEnd) fail("frame");
        const byte = input[raw++]!; if (index === 9 && byte > 1) fail("frame");
        if (byte < 128) { if (index && byte === 0) fail("frame"); complete = true; break; }
      }
      if (!complete) fail("frame");
    }
    frames++; if (frames > limits.records) fail("capacity");
    if (input[bodyStart] === 12) {
      const payload = input.subarray(bodyStart + 2, bodyEnd);
      if (input[bodyStart + 1] !== 2 || payload.length !== 64 || end - start !== 75) fail("commit");
      const nextChain = hash(Buffer.concat([chain, ...pending]));
      if (payload.readBigUInt64LE(0) !== BigInt(sequence + 1) || payload.readBigUInt64LE(8) !== BigInt(lastOffset)
        || payload.readBigUInt64LE(16) !== BigInt(pendingBytes) || payload.readUInt32LE(24) !== pending.length
        || payload.subarray(28, 32).some(byte => byte !== 0) || !payload.subarray(32, 64).equals(nextChain)) fail("commit");
      committed = end; sequence++; lastOffset = start; committedFrames = frames; chain = nextChain; pending.length = 0; pendingBytes = 0;
    } else { pending.push(hash(input.subarray(start, end))); pendingBytes += end - start; }
    cursor = end;
  }
  return { end: committed, sequence, frames: committedFrames, tail: input.length - committed };
}

/** 🔎️ The retained-verification fixture: header, commit frames, resume cuts and every hostile denial. */
type RetainedVerificationFixture = {
  readonly schema: string;
  readonly profile: {
    readonly major: number;
    readonly minor: number;
    readonly requiredFlags: number;
    readonly canonicalVarints: boolean;
    readonly signed: boolean;
    readonly encrypted: boolean;
  };
  readonly limits: {
    readonly fileBytes: number;
    readonly frameBodyBytes: number;
    readonly records: number;
  };
  readonly headerHex: string;
  readonly commits: readonly {
    readonly sequence: number;
    readonly previousOffset: number;
    readonly offset: number;
    readonly end: number;
    readonly records: readonly {
      readonly kind: number;
      readonly flags: number;
      readonly payloadHex: string;
    }[];
    readonly coveredBytes: number;
    readonly recoveredFrames: number;
  }[];
  readonly fuelGrants: readonly number[];
  readonly resume: {
    readonly cuts: readonly number[];
    readonly record: {
      readonly kind: number;
      readonly flags: number;
      readonly payloadHex: string;
    };
    readonly addedBytes: number;
    readonly wrongSinkOffsets: readonly number[];
    readonly exhaustedSequence: string;
  };
  readonly negative: readonly (
    | { readonly id: string; readonly operation: "replace-first-length"; readonly hex: string; readonly error: string }
    | { readonly id: string; readonly operation: "record-limit"; readonly value: number; readonly error: string }
    | { readonly id: string; readonly operation: "file-limit"; readonly value: number; readonly error: string }
    | {
        readonly id: string;
        readonly operation: "header-xor" | "frame-xor" | "commit-xor" | "second-commit-xor";
        readonly offset: number;
        readonly value: number;
        readonly repairCrc: boolean;
        readonly error: string;
      }
  )[];
  readonly compressed: readonly (
    | { readonly id: string; readonly kind: number; readonly flags: number; readonly rawLengthHex: string; readonly storedHex: string; readonly rawHex: string; readonly error: null }
    | { readonly id: string; readonly kind: number; readonly flags: number; readonly rawLengthHex: string; readonly storedHex: string; readonly error: string }
  )[];
};

/** 🧾️ The retained-record fixture: one observation window per framed record, with its corruption outcome. */
type RetainedRecordFixture = {
  readonly schema: string;
  readonly grants: readonly number[];
  readonly headerHex: string;
  readonly cases: readonly {
    readonly id: string;
    readonly kind: number;
    readonly flags: number;
    readonly rawHex: string;
    readonly payloadHex: string;
    readonly repeat: number;
    readonly at: number;
    readonly cancel: boolean;
    readonly corruptCrc: boolean;
    readonly error: null | string;
    readonly observation: {
      readonly frameStart: number;
      readonly payloadStart: number;
      readonly payloadEnd: number;
      readonly frameEnd: number;
      readonly kind: number;
      readonly flags: number;
      readonly rawBytes: null | number;
    };
  }[];
};

export class RetainedVerificationScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("retained-verification-check accepts only --oracle-only");
    const { default: Ajv } = await import("ajv/dist/2020.js");
    const { default: crc } = await import("crc-32/crc32c.js");
    const { inflateRawSync } = await import("node:zlib");
    const leb = await import("@webassemblyjs/leb128");
    const owner = join(this.root, "../../📐️format/🔎️verification");
    const fixture: RetainedVerificationFixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true }); const validate = ajv.compile(schema);
    assert(validate(fixture), ajv.errorsText(validate.errors));
    const checksum = (bytes: Uint8Array): number => crc.buf(bytes) >>> 0;
    const hash = (bytes: Uint8Array): Buffer => Buffer.from(blake3Hex(bytes), "hex");
    assert.equal(checksum(Buffer.from("123456789")), 0xe3069283);
    assert.equal(blake3Hex(Buffer.from("abc")), "6437b3ac38465133ffb63b75273a8db548c558465d79db03fd359c6cd5bd9d85");
    const header = Buffer.alloc(32); Buffer.from([137,83,80,82,13,10,26,10]).copy(header);
    header.writeUInt16LE(1, 8); header.writeUInt32LE(1, 12); header.writeUInt32LE(1, 16);
    header.writeUInt32LE(checksum(header.subarray(0, 20)), 20);
    assert.equal(header.toString("hex"), fixture.headerHex);
    const frame = (kind: number, flags: number, payload: Buffer): Buffer => {
      const body = Buffer.concat([Buffer.from([kind, flags]), payload]);
      const size = Buffer.from(leb.encodeU32(body.length)); const tail = Buffer.alloc(8);
      tail.writeUInt32LE(checksum(body)); tail.writeUInt32LE(size.length + body.length + 8, 4);
      return Buffer.concat([size, body, tail]);
    };
    const parts: Buffer[] = [header]; let chain = hash(header); let offset = 32; let previous = 0; let recovered = 0;
    for (const commit of fixture.commits) {
      const records = commit.records.map((record: { kind: number; flags: number; payloadHex: string }) => frame(record.kind, record.flags, Buffer.from(record.payloadHex, "hex")));
      const covered = records.reduce((count: number, record: Buffer) => count + record.length, 0);
      assert.equal(covered, commit.coveredBytes); assert.equal(offset + covered, commit.offset); assert.equal(previous, commit.previousOffset);
      chain = hash(Buffer.concat([chain, ...records.map(hash)]));
      const payload = Buffer.alloc(64); payload.writeBigUInt64LE(BigInt(commit.sequence)); payload.writeBigUInt64LE(BigInt(previous), 8);
      payload.writeBigUInt64LE(BigInt(covered), 16); payload.writeUInt32LE(records.length, 24); chain.copy(payload, 32);
      const encoded = frame(12, 2, payload); assert.equal(encoded.length, 75);
      parts.push(...records, encoded); offset += covered + encoded.length; recovered += records.length + 1;
      assert.equal(offset, commit.end); assert.equal(recovered, commit.recoveredFrames); previous = commit.offset;
    }
    const bytes = Buffer.concat(parts);
    const inspect = (input: Buffer, limits = fixture.limits) => inspectRetainedSprNeutral(input, checksum, hash, limits);
    assert.deepEqual(inspect(bytes), { end: 255, sequence: 2, frames: 5, tail: 0 });
    for (const cut of fixture.resume.cuts) {
      const prior = inspect(bytes.subarray(0, cut));
      const last = fixture.commits.filter((row: { end: number }) => row.end <= cut).at(-1);
      const prefix = bytes.subarray(0, prior.end);
      const priorChain = last ? prefix.subarray(last.offset + 35, last.offset + 67) : hash(header);
      const record = fixture.resume.record;
      const encoded = frame(record.kind, record.flags, Buffer.from(record.payloadHex, "hex"));
      const payload = Buffer.alloc(64);
      payload.writeBigUInt64LE(BigInt(prior.sequence + 1)); payload.writeBigUInt64LE(BigInt(last?.offset ?? 0), 8);
      payload.writeBigUInt64LE(BigInt(encoded.length), 16); payload.writeUInt32LE(1, 24);
      hash(Buffer.concat([priorChain, hash(encoded)])).copy(payload, 32);
      const resumed = Buffer.concat([prefix, encoded, frame(12, 2, payload)]);
      assert.equal(resumed.length - prefix.length, fixture.resume.addedBytes);
      assert.deepEqual(resumed.subarray(0, prefix.length), prefix);
      assert.deepEqual(inspect(resumed), { end: resumed.length, sequence: prior.sequence + 1, frames: prior.frames + 2, tail: 0 });
    }
    let recoveryCases = 0;
    for (let end = 32; end <= bytes.length; end++) {
      const commit = fixture.commits.filter((row: { end: number }) => row.end <= end).at(-1);
      assert.deepEqual(inspect(bytes.subarray(0, end)), { end: commit?.end ?? 32, sequence: commit?.sequence ?? 0, frames: commit?.recoveredFrames ?? 0, tail: end - (commit?.end ?? 32) }); recoveryCases++;
    }
    const ids = new Set<string>();
    for (const row of fixture.negative) {
      assert(!ids.has(row.id)); ids.add(row.id); let mutated = Buffer.from(bytes); const limits = { ...fixture.limits };
      if (row.operation === "replace-first-length") mutated = Buffer.concat([mutated.subarray(0, 32), Buffer.from(row.hex, "hex"), mutated.subarray(33)]);
      else if (row.operation === "record-limit") limits.records = row.value;
      else if (row.operation === "file-limit") limits.fileBytes = row.value;
      else {
        const offset = row.operation === "commit-xor" ? 94 + row.offset : row.operation === "second-commit-xor" ? 183 + row.offset : row.offset; mutated[offset] ^= row.value;
        if (row.repairCrc) {
          if (row.operation === "header-xor") mutated.writeUInt32LE(checksum(mutated.subarray(0, 20)), 20);
          else if (row.operation === "second-commit-xor") mutated.writeUInt32LE(checksum(mutated.subarray(181, 247)), 247);
          else mutated.writeUInt32LE(checksum(mutated.subarray(92, 158)), 158);
        }
      }
      assert.throws(() => inspect(mutated, limits), new RegExp(`^Error: ${row.error}$`), row.id);
    }
    const extra = JSON.parse(JSON.stringify(fixture)) as { readonly commits: readonly { readonly records: readonly Record<string, unknown>[] }[] };
    extra.commits[0]!.records[0]!.unowned = true;
    assert(!validate(extra));
    for (const row of fixture.compressed) {
      assert(!ids.has(row.id)); ids.add(row.id);
      const rawLength = Buffer.from(row.rawLengthHex, "hex"); const stored = Buffer.from(row.storedHex, "hex");
      const encoded = frame(row.kind, row.flags, Buffer.concat([rawLength, stored]));
      const payload = Buffer.alloc(64); payload.writeBigUInt64LE(1n); payload.writeBigUInt64LE(BigInt(encoded.length), 16); payload.writeUInt32LE(1, 24);
      hash(Buffer.concat([hash(header), hash(encoded)])).copy(payload, 32);
      const committed = Buffer.concat([header, encoded, frame(12, 2, payload)]);
      if (row.error === null) {
        const raw = Buffer.from(row.rawHex, "hex"); assert.deepEqual(inflateRawSync(stored), raw);
        assert.deepEqual(rawLength, Buffer.from(leb.encodeU32(raw.length)));
        assert.deepEqual(inspect(committed), { end: committed.length, sequence: 1, frames: 2, tail: 0 });
      } else assert.throws(() => inspect(committed), new RegExp(`^Error: ${row.error}$`), row.id);
    }
    process.stdout.write(`independent retained SPR oracle: 2 commits, ${recoveryCases} exact LastCommit prefixes, ${fixture.negative.length} strict hostile denials, ${fixture.compressed.length} compressed grammar cases; no typed history publication\n`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const laws = ["retained_spr_verification_matches_neutral_commits_and_torn_prefixes", "retained_spr_verification_rejects_hostile_frames_without_publication", "retained_spr_resume_preserves_exact_prefix_and_commit_chain"];
    for (const law of laws) assert(source.includes(`fn ${law}(`), `missing retained SPR law ${law}`);
    assert(source.includes("fn verify_compressed_fixture(") && source.includes("verify_compressed_fixture(&fixture).await;"), "native compressed raw-length parity law is missing");
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub async fn resume_verified("), "protocol-owned verified writer resume is missing");
    assert.equal(BigInt(fixture.resume.exhaustedSequence) + 1n, 1n << 64n);
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("let next_commit_seq = self.next_commit_seq.checked_add(1)"), "commit sequence must reject exhaustion before writing");
    console.log(`retained SPR resume oracle: ${fixture.resume.cuts.length} exact prefixes, next sequence/previous offset/hash chain preserved`);
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub mod retained;"), "retained SPR module is not mounted; native selection cannot run");
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: laws.map(law => `format::retained::tests::${law}`) }] });
    assert.equal(receipts[0]!.assertions, laws.length);
  }
}

class RetainedRecordObservationScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("retained-record-observation-check accepts only --oracle-only");
    const { default: Ajv } = await import("ajv/dist/2020.js");
    const { default: crc } = await import("crc-32/crc32c.js");
    const leb = await import("@webassemblyjs/leb128");
    const owner = join(this.root, "../../📐️format/🔎️verification/🧾️record");
    const fixture: RetainedRecordFixture = JSON.parse(readFileSync(join(owner, "🧫️fixtures/🔣️.json"), "utf8"));
    const ajv = new Ajv({ strict: true }); const validate = ajv.compile(JSON.parse(readFileSync(join(owner, "🧬️schema/🔣️.json"), "utf8")));
    assert(validate(fixture), ajv.errorsText(validate.errors)); const ids = new Set<string>(); let observed = 0;
    const checksum = (bytes: Uint8Array): number => crc.buf(bytes) >>> 0;
    for (const row of fixture.cases) {
      assert(!ids.has(row.id)); ids.add(row.id);
      const body = Buffer.concat([Buffer.from([row.kind, row.flags]), Buffer.from(row.rawHex, "hex"), Buffer.from(row.payloadHex, "hex"), Buffer.alloc(row.repeat, 97)]);
      const length = Buffer.from(leb.encodeU32(body.length)); const tail = Buffer.alloc(8); tail.writeUInt32LE((checksum(body) ^ Number(row.corruptCrc)) >>> 0); tail.writeUInt32LE(length.length + body.length + 8, 4);
      const bytes = Buffer.concat([Buffer.from(fixture.headerHex, "hex"), length, body, tail]);
      const bodyStart = 32 + length.length; const payloadEnd = bodyStart + body.length;
      assert(row.at <= bytes.length); let readyAt = bodyStart + 2; let rawBytes: number | null = null;
      if (row.flags & 1) {
        const raw = Buffer.from(row.rawHex, "hex"); let value = 0n;
        for (let index = 0; index < raw.length; index++) value |= BigInt(raw[index]! & 127) << BigInt(7 * index);
        rawBytes = Number(value); assert.deepEqual(raw, Buffer.from(leb.encodeU32(rawBytes))); readyAt += raw.length;
      }
      let error: string | null = null;
      if (row.at >= bodyStart + 2 && ((row.flags & ~31) !== 0 || Boolean(row.flags & 1) !== Boolean(row.flags & 28))) error = "frame";
      if (row.at === bytes.length) {
        try { const span = inspectRetainedSprNeutral(bytes, checksum, value => Buffer.from(blake3Hex(value), "hex")); assert.equal(span.sequence, 0); assert.equal(span.end, 32); }
        catch (failure) { error = failure instanceof Error ? failure.message : "unknown"; }
      }
      if (row.cancel && error === null) error = "cancelled";
      const observation = error === null && row.at >= readyAt && row.at < bytes.length
        ? { frameStart: 32, payloadStart: readyAt, payloadEnd, frameEnd: bytes.length, kind: row.kind, flags: row.flags, rawBytes } : null;
      assert.equal(error, row.error, row.id); assert.deepEqual(observation, row.observation, row.id); if (observation) observed++;
    }
    const extra = JSON.parse(JSON.stringify(fixture)) as { readonly cases: readonly Record<string, unknown>[] };
    extra.cases[0]!.authority = true;
    assert(!validate(extra));
    console.log(`[TRACE] retained SPR observation oracle: ${fixture.cases.length} exact rows, ${observed} scalar observations; compressed raw-length/empty payload/clear/error/cancel; zero commit or input authority`);
    const source = readFileSync(join(owner, "🦀️.rs"), "utf8");
    const law = "retained_record_observation_uses_the_existing_framing_state_without_authority";
    assert(source.includes('include_str!("🧫️fixtures/🔣️.json")') && source.includes(`fn ${law}(`));
    assert(source.includes("impl RetainedSprVerification") && !source.includes("fn push("), "metadata must observe the existing scanner, not parse a second framing grammar");
    if (segments.includes("--oracle-only")) return;
    assert(readFileSync(join(owner, "../🦀️.rs"), "utf8").includes("pub mod record;"), "retained record observation remains unmounted");
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: [`format::retained::record::tests::${law}`] }] });
    assert.equal(receipts[0]!.assertions, 1);
  }
}

/** 🛡️ Cross-language hostile-input oracle for the exact bounded PresencePeer codec. */
class PresencePeerCodecScript extends BundleScript {
  async run(segments: string[]): Promise<void> {
    if (segments.some(segment => segment !== "--oracle-only")) throw new Error("presence-peer-codec-check accepts only --oracle-only");
    const { default: Ajv } = await import("ajv");
    const owner = join(this.root, "../../🧫️fixtures/👥️presence-peer-codec-v1");
    const fixture = JSON.parse(readFileSync(join(owner, "🔣️.json"), "utf8"));
    const schema = JSON.parse(readFileSync(join(this.root, "../../🧬️schema/🔣️.json"), "utf8"));
    const validate = new Ajv({ strict: true }).addSchema(schema).getSchema(`${schema.$id}#/$defs/PresencePeerCodecFixture`)!;
    assert(validate(fixture), validate.errors?.map(error => `${error.instancePath} ${error.message}`).join("; "));
    const codec = await import(join(this.root, "../../🟦️.ts"));
    assert.deepEqual(codec.PRESENCE_PEER_WIRE_LIMITS_V1, fixture.limits);
    const ids = new Set<string>();
    for (const row of fixture.cases) {
      assert(!ids.has(row.id), `duplicate fixture id ${row.id}`); ids.add(row.id);
      const bytes = Buffer.concat([Buffer.from(row.prefixHex, "hex"), Buffer.alloc(row.repeatCount, Number.parseInt(row.repeatHex, 16)), Buffer.from(row.suffixHex, "hex")]);
      const position: [number] = [0];
      if (row.accepted) {
        const peer = codec.decodePresencePeer(bytes, position);
        assert.equal(position[0], bytes.length, row.id);
        assert.equal(Buffer.from(codec.encodePresencePeer(peer)).toString("hex"), row.canonicalHex, row.id);
        const semantic = JSON.parse(JSON.stringify(peer));
        if (peer.presencePack !== undefined) semantic.presencePack = Buffer.from(peer.presencePack).toString("base64");
        if (peer.interaction !== undefined) { semantic.interaction.appId = peer.interaction.app_id; delete semantic.interaction.app_id; }
        assert.deepEqual(semantic, row.expected, row.id);
      } else {
        assert.throws(() => codec.decodePresencePeer(bytes, position), Error, row.id);
        assert.equal(position[0], 0, `${row.id} advanced the caller cursor`);
      }
    }
    const extra = JSON.parse(JSON.stringify(fixture)) as { readonly cases: readonly Record<string, unknown>[] };
    extra.cases[0]!.authority = true;
    assert(!validate(extra));
    const source = readFileSync(join(this.root, "../../📡️wire/🦀️.rs"), "utf8");
    const tests = readFileSync(join(this.root, "../../📡️wire/🧪️tests/🔬️presence-codec/🦀️.rs"), "utf8");
    const laws = ["presence_peer_decoder_matches_neutral_bounded_exact_corpus", "presence_peer_decoder_rejects_hostile_counts_before_allocation"];
    for (const law of laws) assert(tests.includes(`fn ${law}(`), `missing native presence codec law ${law}`);
    assert(source.includes("PRESENCE_PEER_WIRE_LIMITS_V1") && source.includes("reader.position != bytes.len()"), "Rust bounded exact decoder is absent");
    console.log(`presence peer codec oracle: ${fixture.cases.length} neutral Rust/TypeScript vectors, ${fixture.cases.filter((row: { accepted: boolean }) => !row.accepted).length} hostile inputs rejected exactly`);
    if (segments.includes("--oracle-only")) return;
    const receipts = await runExactCargoLaws({ manifestPaths: { "semio-framework-replication": resolve(this.root, "Cargo.toml") }, cargoTargetDir: readCargoTestPolicyV1(process.env).targetDirectory, cwd: this.repoRoot, groups: [{ package: "semio-framework-replication", target: { kind: "lib", name: "protocol" }, laws: laws.map(law => `wire::frames::presence_codec_tests::${law}`) }] });
    assert.equal(receipts[0]!.assertions, laws.length);
  }
}

/** 🏛️ Checks the complete neutral command ingress source and dependency contract. */
class CommandIngressOwnershipScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  if(segments.length)throw Error("test-command-ingress-ownership accepts no arguments");
  const source=resolve(this.root,"../../📡️wire/🎮️command/📥️ingress/🏛️ownership/🧪️tests/🟦️.ts");
  await runBudgetedTestCommand(process.execPath,[Bun.resolveSync("typescript/bin/tsc",this.root),"--noEmit","--strict","--skipLibCheck","--allowImportingTsExtensions","--esModuleInterop","--target","ESNext","--module","ESNext","--moduleResolution","bundler","--types","bun",source],{cwd:this.repoRoot,budgetMs:cmdBudgetMs(),throwOnFailure:true});
  await runBudgetedTestCommand(process.execPath,["test",source],{cwd:this.repoRoot,budgetMs:testLevelBudgetMs(),throwOnFailure:true});
 }
}
/** 📥️ Runs the complete registered neutral Replication native cohort. */
class CommandIngressNativeScript extends BundleScript {
 async run(segments:string[]):Promise<void>{
  const{rest}=resolveTestLevel(segments);
  if(rest.length)throw Error("test-command-ingress-native accepts only an execution level");
  await runCargoTestsV1({manifestPath:resolve(this.root,"Cargo.toml"),packages:["semio-framework-replication"],cwd:this.root,extraArgs:["--lib","--no-fail-fast"]},readCargoTestPolicyV1(process.env));
 }
}

const router = new ScriptRouter(import.meta.dir).register("test-command-ingress-ownership", CommandIngressOwnershipScript).register("test-command-ingress-native", CommandIngressNativeScript).register("test", TestScript).register("build", BuildScript).register("test-source", SourceTestScript).register("test-local-interaction-source", LocalInteractionSourceTestScript).register("test-local-interaction-native", LocalInteractionNativeTestScript).register("retained-verification-check", RetainedVerificationScript).register("retained-record-observation-check", RetainedRecordObservationScript).register("presence-peer-codec-check", PresencePeerCodecScript);

if (import.meta.main) await runScriptMain(router, { defaultCommand: "test" });

```

### Proposed successor1 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/📋️project.json

```
{
  "name": "@semio-tech/framework-replication-rs",
  "$schema": "../../../../../node_modules/nx/schemas/project-schema.json",
  "root": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
  "sourceRoot": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
  "projectType": "library",
  "tags": [
    "lang:rust",
    "role:framework",
    "family:replication"
  ],
  "namedInputs": {
    "default": [
      "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/**/*.rs",
      "{workspaceRoot}/🧰️framework/🔨️modules/🌱️value/**/*",
      "{projectRoot}/**/*",
      "{workspaceRoot}/🧰️framework/🔨️modules/🔏️hash/🟦️.ts"
    ]
  },
  "targets": {
    "presence-peer-codec-check": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts presence-peer-codec-check --oracle-only",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "presence-peer-codec-native-check": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts presence-peer-codec-check",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "retained-record-observation-check": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts retained-record-observation-check",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "retained-verification-check": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts retained-verification-check",
        "forwardAllArgs": true
      },
      "outputs": []
    },
    "test-local-interaction-native": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-local-interaction-native",
        "forwardAllArgs": true
      },
      "inputs": [
        "production",
        "^production",
        "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧫️fixtures/🛂️mutation-source-authority/🧭️domains.json",
        "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/🛂️mutation-source-authority/🧭️domains.json"
      ]
    },
    "test-local-interaction-source": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-local-interaction-source",
        "forwardAllArgs": true
      },
      "inputs": [
        "production",
        "^production",
        "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧫️fixtures/🛂️mutation-source-authority/🧭️domains.json",
        "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/🛂️mutation-source-authority/🧭️domains.json"
      ]
    },
    "test-source": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-source",
        "forwardAllArgs": true
      },
      "inputs": [
        "production",
        "^production",
        "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧫️fixtures/🛂️mutation-source-authority/🧭️domains.json",
        "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/🛂️mutation-source-authority/🧭️domains.json"
      ]
    },
    "build": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts build",
        "forwardAllArgs": true
      },
      "cache": true,
      "outputs": [
        "{projectRoot}/dist/build"
      ]
    },
    "test": {
      "executor": "nx:run-commands",
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test",
        "forwardAllArgs": true
      },
      "inputs": [
        "production",
        "^production",
        "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧫️fixtures/🛂️mutation-source-authority/🧭️domains.json",
        "{workspaceRoot}/🧰️framework/🔨️modules/📡️replication/🎮️mutation/🧬️schema/🛂️mutation-source-authority/🧭️domains.json"
      ]
    },
    "test-command-ingress-ownership": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-command-ingress-ownership",
        "forwardAllArgs": true
      },
      "inputs": [
        "default",
        "^default",
        "{workspaceRoot}/Cargo.toml",
        "{workspaceRoot}/🧰️framework/📦️packages/🦀️rust/Cargo.toml",
        "{workspaceRoot}/🧰️framework/🔨️modules/🎠️kernel/🦀️.rs",
        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/**/*",
        "{workspaceRoot}/🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/📥️command-ingress-pages/🔣️.json"
      ]
    },
    "test-command-ingress-native": {
      "executor": "nx:run-commands",
      "cache": false,
      "options": {
        "cwd": "🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust",
        "command": "bun ./📜️script.ts test-command-ingress-native",
        "forwardAllArgs": true
      },
      "inputs": [
        "default",
        "^default",
        "{workspaceRoot}/Cargo.toml",
        "{workspaceRoot}/🧰️framework/📦️packages/🦀️rust/Cargo.toml",
        "{workspaceRoot}/🧰️framework/🔨️modules/🎠️kernel/🦀️.rs",
        "{workspaceRoot}/🧰️framework/🛍️products/💻️os/🔨️modules/📡️spr/**/*",
        "{workspaceRoot}/🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/📥️command-ingress-pages/🔣️.json"
      ]
    }
  }
}

```

### Proposed successor1 🧰️framework/🔨️modules/📡️replication/📦️packages/🦀️rust/package.json

```
{
  "name": "@semio-tech/framework-replication-rs",
  "private": true,
  "scripts": {
    "test-command-ingress-ownership": "nx run @semio-tech/framework-replication-rs:test-command-ingress-ownership",
    "test-command-ingress-native": "nx run @semio-tech/framework-replication-rs:test-command-ingress-native"
  }
}

```
