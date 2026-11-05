# Persistence and Runtime Audit

Read-only source audit; no runtime startup or persistence round trip executed. Root, s, and puzzle AGENTS.md read.

## Findings

- Disproved standalone missing-catalog defect: puzzle editors do not declare natural_file_codec at all. The entire puzzle Rust source has no NaturalFile/native-file overrides; ArtifactEditor default returns None (plugin declaration/🪟️surface/✏️artifact-editor-trait/🦀️.rs 469–478). Builder native_editor (builder/🌱️surfaces/🦀️.rs 12) only mounts paired Save/Open actions for Some(codec). Consequently there is currently no puzzle artifact:native Save output, extension, or byte representation. Host naturalFileFormatV1 returns null for these apps. The absence of puzzle format catalog descriptors therefore does not break an existing declared native Save round trip; implementing native support would require codecs and format claims together.
- Natural-file save starts a cancellable Tasks row but exportAppMedia(instanceId, artifact:native) receives no signal or progress callback. Cancellation is only observed before and after the awaited export. Expensive exports therefore run to completion after cancellation. Owner: framework OS renderer ShellHost/📤️document-transfer-tabs/🔶️scope/🟦️.tsx lines 140–162.
- Shell input funnel marks save/open actions applied immediately through void promises. Action receipts cannot establish successful file transfer. Owner: ShellHost/🖱️shell-actions/🧭️input-funnel/🟦️.ts lines 77–84.
- Snapshot import file-picker failures are swallowed by catch returning undefined. Owner: input-funnel lines 221–230.

## Existing Correctness and Test Boundaries

Natural-file open uses a fresh instance, imports artifact:native, and destroys failures via openNaturalFileOwnerV1. Existing archive import likewise creates a new instance and destroys failures; no proven data corruption identified. Shared host test: renderer engine/🧪️tests/💾️media-export-and-natural-files/🟦️.ts. Shared guest declaration test: plugin/🧪️tests/📄️natural-file-lifecycle/🦀️.rs.

Storybook 5D is explicitly a useState emulator which synthesizes history from parts. 3D fixtures intentionally omit mesh URLs and render placeholder boxes. These cannot establish real editor persistence or complete mesh integration.

Composition targets: repo run @semio-tech/s-puzzle-composition-tests:verify-browser-contribution; repo run @semio-tech/s-puzzle-composition-tests:verify-puzzle-fill-policy-self-tests.

## Verified Serve Entry

Framework dev script registers serve and ServeScript accepts serve <variant> react <prototype|release> [server options]. It automatically activates when no receipt exists. Specific catalog fixtures explicitly declare puzzle2d, puzzle3d and puzzle5d variants.

From repository root:

    repo ./🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/📦️packages/🟦️typescript/📜️script.ts serve puzzle2d react prototype

Replace puzzle2d with puzzle3d or puzzle5d. This invocation is source-verified, not runtime-verified. The continuous target repo run @semio-tech/framework-os-dev:serve-hold accepts --serve <url> [--hub <url>] [--variant <v>].

## Precise Puzzle Export Representation

All three own exportSnapshot actions emit application/json, UTF-8, .json filenames. 2D uses manifest id or puzzle-2d.json; 3D active example id or puzzle-3d.json; 5D slugged label or puzzle-5d.json. Owners are each editor/🎮️commands/(🧊️document/ where applicable)/📤️export-snapshot/🦀️.rs. These are distinct from the reserved artifact:native Save route. Opening these through generic host Open File routes by .json to stdio JSON; restoring puzzle content uses the puzzle own importSnapshot menu. This behavior is consistent with present declarations, so a new puzzle natural-file catalog alone is not an appropriate fix.
