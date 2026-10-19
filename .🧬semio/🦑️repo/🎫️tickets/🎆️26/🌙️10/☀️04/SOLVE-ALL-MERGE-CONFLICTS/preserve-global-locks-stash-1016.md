# Source-Only Shared Lock Preservation

No Cargo, resolver, build, compiler, install or tests were executed. All existing HEAD-resolved records remain. Existing authored stash lock records and dependency edges are reconciled against changed local manifests, mapping dependencies by explicit package/path/workspace identity. This preserves source-side intent without making a build correctness claim. Manifest input was sampled while source lanes continue; a final refresh remains required.

## `Cargo.lock`

Packages 1426; changed package edges 10; added records [].

- `semio-framework-machine`: add ['semio-framework-value']; remove []
- `semio-framework-actor`: add ['semio-framework-pack-json']; remove []
- `semio-framework-value`: add ['blake3', 'semio-framework-hash']; remove []
- `semio-framework-repo-dashboard-cli`: add ['semio-framework-ui']; remove []
- `semio-framework-os-infinite-board-normal-fill`: add ['semio-framework-action-bus']; remove []
- `semio-framework-plugin`: add ['semio-framework-value-snapshot-read']; remove []
- `semio-framework-os-renderer-wgpu`: add ['semio-framework-plugin-ownership']; remove []
- `semio-framework-repo-dashboard-terminal`: add ['semio-framework-repo-dashboard-cli']; remove []
- `semio-framework-repo-dashboard-repo-domain`: add ['semio-framework-repo-identity', 'semio-framework-repo-todos']; remove []
- `semio-framework-os-inference`: add ['semio-framework-value-snapshot-read']; remove []

Unresolved source mappings: []

## `🧰️framework/Cargo.lock`

Packages 924; changed package edges 3; added records ['semio-framework-math-dense', 'semio-framework-math-rank'].

- `semio-framework-machine`: add ['semio-framework-value']; remove []
- `semio-framework-actor`: add ['semio-framework-pack-json']; remove []
- `semio-framework-value`: add ['blake3', 'semio-framework-hash']; remove []

Unresolved source mappings: []

## `🌎️hub/Cargo.lock`

Packages 2559; changed package edges 43; added records ['semio-s-plugin-energy-epw'].

- `semio-framework-machine`: add ['semio-framework-value']; remove []
- `semio-s-artifact-fem-2d-load`: add ['semio-s-artifact-fem-2d-mutations', 'serde_json']; remove []
- `semio-s-artifact-fem-2d-editor-session`: add ['semio-framework-ui-scene', 'semio-s-fem-app-surface']; remove []
- `semio-s-plugin-energy-engine-sim`: add ['semio-framework-os-kernel', 'semio-framework-trace', 'semio-s-plugin-energy-engine-bestest', 'serde_json']; remove []
- `semio-s-artifact-fem-3d-load`: add ['semio-s-artifact-fem-3d-mutations', 'serde_json']; remove []
- `semio-framework-actor`: add ['semio-framework-pack-json']; remove []
- `semio-s-plugin-energy-engine-precompute`: add ['semio-framework-trace', 'semio-s-plugin-energy-engine-bestest']; remove []
- `semio-s-artifact-fem-2d-material`: add ['semio-s-artifact-fem-2d-mutations', 'serde_json']; remove []
- `semio-s-artifact-fem-2d-editor-interaction`: add ['semio-framework-geometry']; remove []
- `semio-s-artifact-fem-2d-mesh-topology`: add ['semio-s-artifact-fem-2d-mutations', 'serde_json']; remove []
- `semio-framework-value`: add ['blake3', 'semio-framework-hash']; remove []
- `semio-s-artifact-fem-2d-analysis`: add ['semio-s-artifact-fem-2d-mutations', 'serde_json']; remove []
- `semio-s-artifact-puzzle-3d-retained-action`: add ['semio-s-artifact-puzzle-3d-mutations-object']; remove []
- `semio-s-artifact-energy-model-session`: add ['semio-framework-trace']; remove []
- `semio-s-artifact-fem-3d-editor`: add ['semio-s-fem-analyses-model']; remove []
- `semio-s-plugin-energy-epw`: add ['csv']; remove []
- `semio-s-artifact-fem-2d-mesh`: add ['semio-s-artifact-fem-2d-mutations', 'serde_json']; remove []
- `semio-s-artifact-puzzle-3d`: add ['semio-framework-async', 'serde']; remove []
- `semio-s-artifact-fem-3d-analysis`: add ['semio-s-artifact-fem-3d-mutations', 'serde_json']; remove []
- `semio-s-plugin-puzzle-jobs`: add ['semio-framework', 'serde_json']; remove []
- `semio-framework-os-infinite-board-normal-fill`: add ['semio-framework-action-bus']; remove []
- `semio-s-artifact-fem-2d-mutations`: add ['semio-s-artifact-fem-2d-engine', 'semio-s-fem-model']; remove []
- `semio-s-artifact-puzzle-2d`: add ['semio-framework-plugin-ownership']; remove []
- `semio-s-artifact-fem-2d-editor`: add ['semio-s-fem-analyses-model']; remove []
- `semio-s-artifact-fem-3d-editor-session`: add ['semio-s-artifact-fem-3d-editor-results-window', 'semio-s-artifact-fem-3d-scene', 'semio-s-fem-app-surface']; remove []
- `semio-s-artifact-puzzle-5d`: add ['semio-framework-job-publication']; remove ['js-sys']
- `semio-framework-plugin`: add ['semio-framework-value-snapshot-read']; remove []
- `semio-s-artifact-fem-3d-boundary`: add ['semio-s-artifact-fem-3d-mutations', 'serde_json']; remove []
- `semio-s-artifact-fem-3d-material`: add ['semio-s-artifact-fem-3d-mutations', 'serde_json']; remove []
- `semio-s-artifact-fem-2d-boundary`: add ['semio-s-artifact-fem-2d-mutations', 'serde_json']; remove []
- `semio-s-artifact-fem-2d`: add ['csv', 'semio-framework-async', 'semio-framework-geometry', 'semio-framework-trace']; remove []
- `semio-s-artifact-fem-2d-editor-panels`: add ['semio-s-fem-analyses-model']; remove []
- `semio-s-artifact-fem-3d-editor-results-window`: add ['semio-s-artifact-fem-3d-editor-model-window', 'semio-s-fem-analyses-model']; remove []
- `semio-s-artifact-fem-3d-mesh`: add ['semio-s-artifact-fem-3d-mutations', 'serde_json']; remove []
- `semio-s-artifact-fem-2d-editor-results-window`: add ['semio-s-artifact-fem-2d-editor-session', 'semio-s-artifact-fem-2d-editor-terminology', 'semio-s-fem-analyses-model']; remove []
- `semio-s-artifact-fem-3d-mesh-topology`: add ['semio-s-artifact-fem-3d-mutations', 'serde_json']; remove []
- `semio-s-artifact-fem-2d-editor-model-window`: add ['semio-s-fem-analyses-model']; remove []
- `semio-framework-os-inference`: add ['semio-framework-value-snapshot-read']; remove []
- `semio-s-artifact-fem-3d-editor-visual-job`: add ['semio-s-artifact-fem-3d-scene']; remove []
- `semio-s-artifact-puzzle-3d-editor`: add []; remove ['js-sys']
- `semio-s-artifact-energy-model-editor-results`: add ['semio-framework-trace']; remove []
- `semio-s-artifact-energy-model`: add ['csv', 'semio-framework-trace', 'semio-s-plugin-energy-engine-sim']; remove []
- `semio-s-artifact-energy-model-mutations`: add ['semio-s-plugin-energy-engine-model']; remove []

Unresolved source mappings: [{"package": "semio-s-artifact-fem-3d", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-session", "dependency": "semio-s-artifact-fem-2d-results-state", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-session", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-session", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-editor-commands", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-editor-commands", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-io", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-io", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🚪️io/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-plugin-energy-engine-precompute", "dependency": "semio-framework-math-dense", "path": "✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🧠️precompute/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-material", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-model", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-model", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-mesh-topology", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-energy-model-session", "dependency": "semio-framework-math-rank", "path": "✏️s/🔌️plugins/🔋️energy/🗿️artifacts/🔋️model/🏅️standards/🔖️1/🪆️subsets/✳️any/🧵️simulation-session/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-editor-panels", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-editor-panels", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-commands", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-commands", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎮️commands/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-hub-raster", "dependency": "semio-s-artifact-raster-raster-viewer", "path": "🌎️hub/🧩️compositions/🖨️raster/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-hub-raster", "dependency": "semio-s-artifact-raster-raster-editor", "path": "🌎️hub/🧩️compositions/🖨️raster/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-mesh", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-plugin-energy-engine-num", "dependency": "semio-framework-math-dense", "path": "✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🔢️num/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-scene", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🎬️scene/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor", "dependency": "semio-s-artifact-fem-2d-results-state", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-material", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🧱️material/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-panels", "dependency": "semio-s-artifact-fem-2d-results-state", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-panels", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-panels", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-mutations", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-mutations", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-mesh", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/🧬️mutations/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-model", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-model", "dependency": "semio-s-fem-material", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🌐️any/🧬️schema/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-results-window", "dependency": "semio-s-artifact-fem-2d-results-state", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-viewer", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/👁️viewer/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-3d-mesh-topology", "dependency": "semio-s-fem-node", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/🧊️3d/🏅️standards/🔖️1/🪆️subsets/🕸️mesh/🧬️schema/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-plugin-energy-engine-kernel", "dependency": "semio-framework-math-dense", "path": "✏️s/🔌️plugins/🔋️energy/🔨️modules/⚡️simulation/⚙️engine/🌰️kernel/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}]

## Authored Record Transfer

Imported uniquely identified local math/FEM resolved records from other existing authored workspace locks; no dependency versions were resolved. [{"path": "🌎️hub\\Cargo.lock", "imported": [{"name": "semio-s-fem-material", "source": "✏️s/🔌️plugins/🏗️fem/⚙️engine/🧱️material/Cargo.lock"}, {"name": "semio-s-fem-node", "source": "✏️s/🔌️plugins/🏗️fem/⚙️engine/📍node/Cargo.lock"}, {"name": "semio-framework-math-dense", "source": "🧰️framework/Cargo.lock"}, {"name": "semio-framework-math-rank", "source": "🧰️framework/Cargo.lock"}], "sourceEdgesPreserved": 34, "unresolvedAuthoredRecords": [{"package": "semio-s-artifact-fem-2d-editor-session", "dependency": "semio-s-artifact-fem-2d-results-state", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-hub-raster", "dependency": "semio-s-artifact-raster-raster-viewer", "path": "🌎️hub/🧩️compositions/🖨️raster/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-hub-raster", "dependency": "semio-s-artifact-raster-raster-editor", "path": "🌎️hub/🧩️compositions/🖨️raster/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor", "dependency": "semio-s-artifact-fem-2d-results-state", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-panels", "dependency": "semio-s-artifact-fem-2d-results-state", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}, {"package": "semio-s-artifact-fem-2d-editor-results-window", "dependency": "semio-s-artifact-fem-2d-results-state", "path": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/📦️packages/🦀️rust/Cargo.toml", "reason": "no existing resolved record"}]}]

The three remaining dependency names have no authored resolved record in any of182existing workspace lockfiles. Their incoming source manifest entries remain preserved verbatim. Completing lock resolution or repairing those authored paths is compilation work outside the requested scope.

## Final Source Manifest Refresh

After all execution lanes completed, re-read changed local manifests and compare them against immutable HEAD. All82changed package manifests parse; refreshed the three shared locks with any additional known authored dependency records/edges. Existing resolved package records remain, including HEAD dissolved crates; no Cargo resolution or compiler ran.

[
  {
    "path": "Cargo.lock",
    "packages": 1426,
    "sourceChangedPackages": 82,
    "addedRecords": [],
    "newEdgeChanges": [
      {
        "package": "semio-framework-pack-record-encode",
        "add": [
          "semio-framework-hash",
          "semio-framework-pack-record-preflight"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-replication-mutation",
        "add": [
          "semio-framework-dsl-record"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-replication-mutation-messages",
        "add": [
          "semio-framework-trace-memory-testing"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-os-store-replay",
        "add": [
          "semio-framework-os-store-snapshot-clone"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-os-renderer-wgpu-kernel-protocol",
        "add": [
          "semio-framework-job-worker",
          "semio-framework-os-renderer-wgpu-host-plumbing"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-os-renderer-wgpu-host-plumbing",
        "add": [
          "semio-framework-os-services"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-os-renderer-wgpu-asset-decode",
        "add": [
          "semio-framework-os-renderer-wgpu-host-plumbing",
          "semio-framework-plugin-ownership"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-plugin-decl-app",
        "add": [
          "semio-framework-actor"
        ],
        "remove": []
      }
    ],
    "remainingAuthoredNames": [],
    "unresolved": []
  },
  {
    "path": "🧰️framework/Cargo.lock",
    "packages": 924,
    "sourceChangedPackages": 82,
    "addedRecords": [],
    "newEdgeChanges": [
      {
        "package": "semio-framework-pack-record-encode",
        "add": [
          "semio-framework-hash",
          "semio-framework-pack-record-preflight"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-replication-mutation",
        "add": [
          "semio-framework-dsl-record"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-replication-mutation-messages",
        "add": [
          "semio-framework-trace-memory-testing"
        ],
        "remove": []
      }
    ],
    "remainingAuthoredNames": [],
    "unresolved": []
  },
  {
    "path": "🌎️hub/Cargo.lock",
    "packages": 2564,
    "sourceChangedPackages": 82,
    "addedRecords": [
      {
        "name": "semio-framework-trace-memory-testing",
        "origin": "🧰️framework/Cargo.lock"
      }
    ],
    "newEdgeChanges": [
      {
        "package": "semio-framework-pack-record-encode",
        "add": [
          "semio-framework-hash",
          "semio-framework-pack-record-preflight"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-replication-mutation",
        "add": [
          "semio-framework-dsl-record"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-replication-mutation-messages",
        "add": [
          "semio-framework-trace-memory-testing"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-os-store-replay",
        "add": [
          "semio-framework-os-store-snapshot-clone"
        ],
        "remove": []
      },
      {
        "package": "semio-framework-plugin-decl-app",
        "add": [
          "semio-framework-actor"
        ],
        "remove": []
      }
    ],
    "remainingAuthoredNames": [
      "semio-s-artifact-fem-2d-results-state",
      "semio-s-artifact-raster-raster-editor",
      "semio-s-artifact-raster-raster-viewer"
    ],
    "unresolved": [
      {
        "package": "semio-s-artifact-fem-2d-editor-results-window",
        "dependency": "semio-s-artifact-fem-2d-results-state",
        "source": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🎭️modes/✏️edit/🪟️windows/📊️results/📦️packages/🦀️rust/Cargo.toml",
        "reason": "authored source has no existing resolved record"
      },
      {
        "package": "semio-s-artifact-fem-2d-editor-panels",
        "dependency": "semio-s-artifact-fem-2d-results-state",
        "source": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📌️panels/📦️packages/🦀️rust/Cargo.toml",
        "reason": "authored source has no existing resolved record"
      },
      {
        "package": "semio-s-artifact-fem-2d-editor",
        "dependency": "semio-s-artifact-fem-2d-results-state",
        "source": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/📦️packages/🦀️rust/Cargo.toml",
        "reason": "authored source has no existing resolved record"
      },
      {
        "package": "semio-s-artifact-fem-2d-editor-session",
        "dependency": "semio-s-artifact-fem-2d-results-state",
        "source": "✏️s/🔌️plugins/🏗️fem/🗿️artifacts/◻️2d/🏅️standards/🔖️1/🪆️subsets/🌐️any/✏️editor/🧵️session/📦️packages/🦀️rust/Cargo.toml",
        "reason": "authored source has no existing resolved record"
      },
      {
        "package": "semio-hub-raster",
        "dependency": "semio-s-artifact-raster-raster-viewer",
        "source": "🌎️hub/🧩️compositions/🖨️raster/📦️packages/🦀️rust/Cargo.toml",
        "reason": "authored source has no existing resolved record"
      },
      {
        "package": "semio-hub-raster",
        "dependency": "semio-s-artifact-raster-raster-editor",
        "source": "🌎️hub/🧩️compositions/🖨️raster/📦️packages/🦀️rust/Cargo.toml",
        "reason": "authored source has no existing resolved record"
      }
    ]
  }
]
