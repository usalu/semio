# Source Owner Workspace Metadata

The new framework source owners and separate Pixels native owner were covered by the root workspace glob but missing from Bun's lock metadata. Root captured the full preceding lock and used ordinary `bun install --lockfile-only --ignore-scripts` to synchronize current package declarations. This installs no dependencies and runs no lifecycle hooks. The command completed successfully and recorded87workspace entries, previously74; full predecessor/current/inverse is retained in `workspace-source-owner-metadata-inputs/lock-current-inverse-1.json`.

The thirteen new entries include the compute runner, Mesh attribute owner, Flow/Plugin/host compute owners, OS facade and Pixels native owner, plus current foreign framework package declarations. No metadata was hand-edited or inferred from source test success. Generated Bun output is under `🗑️generated/source-owner-workspace-metadata`. Independent JSON5 and JSONC parser agreement and normalized package/workspace changes are retained in `lock-parser-parity-1.json`. A frozen lock-only validation is executing against the synchronized lock.

The frozen lock-only command completed successfully. Independent parser receipts show no changed original workspace entry, no removed package and precisely thirteen added first-party workspace packages. No external package was added by this synchronization.
