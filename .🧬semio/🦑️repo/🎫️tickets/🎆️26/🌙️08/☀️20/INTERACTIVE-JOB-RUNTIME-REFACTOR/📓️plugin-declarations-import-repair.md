# Plugin Declarations Import Repair

The retained native RED identifies E0252 at `🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️component.rs:6969`: the public testkit already imports `declarations` with `ArtifactEditor`, `ArtifactViewer`, and `ViewerApp` near line 6919, while a later `#[cfg(test)] use super::declarations;` reimports it.

Only that redundant attribute/import pair was removed. The unconditional import and the declaration-helper bodies remain in the same region, so non-test helper signatures still resolve `declarations`.

Source SHA-256 before: `7f7b17b8beabde935b839e5c512bd6ffe91ac9862b52ebde3ee3d96b693181f8`.

Source SHA-256 after: `b115fb7e44e311352da1222292712817b17cb84e8422ddf61031c7da257f0d3e`.

No Cargo command was run. Native verification remains runtime-owned.
