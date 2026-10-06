# Hub Space Owned and Hosted Metadata Readback

Source-only independent readback; no gate run or runtime credit.

The current Plugin builder `🏗️builder/🦀️.rs:236` sends `.artifact` declarations to `artifacts`; `host_artifact` at248 sends foreign declarations to `hosted_artifacts`. At759–762 only `hosted_artifacts` contributes `manifest.hosted_artifact_kinds`; both vectors subsequently apply their actual declarations at763. Hub Space composition `🌎️hub/🧩️compositions/🪐️space/🦀️.rs:234,239` declares Home and Space through `.artifact`, not `.host_artifact`.

The existing guard `Plugin/🦀️.rs:4781–4801` also preserves owned kinds and moves only foreign surface kinds into hosted metadata. Therefore an empty foreign-host roster is compatible with two owned Home/Space definitions. The census should independently pin `hostedArtifactKinds: []` while preserving both real declaration binding/TypeId/SQL/installed-hook/lookup/two exact-route assertions. This finding does not establish those runtime assertions passed.
