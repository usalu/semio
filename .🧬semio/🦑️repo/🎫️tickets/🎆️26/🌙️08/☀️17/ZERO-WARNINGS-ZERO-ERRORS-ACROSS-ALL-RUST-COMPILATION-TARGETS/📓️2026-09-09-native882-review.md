# Native882 Diagnostic Review

{
  "unique": 514,
  "fault": 465,
  "retained": [
    {
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs",
      "line": 636,
      "text": "    fn push(&mut self, value: PendingResume) -> Result<(), PendingResume> {",
      "label": "the `Err`-variant is at least 312 bytes"
    },
    {
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/⚛️reactor/🦀️.rs",
      "line": 1022,
      "text": "            .with(|resumes| resumes.borrow_mut().push(pending))",
      "label": "the `Err`-variant is at least 312 bytes"
    },
    {
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "line": 17708,
      "text": "        pub async fn register_child(&mut self, slot: impl Into<String>, child_id: impl Into<String>, dialect: ArtifactDialect, mut member: M) -> Result<(), ChildMemberRegistrationError<M>> {",
      "label": "the `Err`-variant is at least 216 bytes"
    },
    {
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 266,
      "text": "    pub fn append_step(&mut self, input: &mut ui_wgpu::wgpu::PreparedRenderInput) -> Result<bool, World3dBuildRejected> {",
      "label": null
    },
    {
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 3061,
      "text": "    fn finish(self, state: &World3dState, generation: u64) -> Result<(WorldMarqueeGesture, WorldMarqueeResultPages), (Self, WorldInteractionStep)> {",
      "label": "the `Err`-variant is at least 280 bytes"
    },
    {
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 4474,
      "text": "    fn finish(mut self, state: &World3dState, generation: u64) -> Result<Option<WorldGumballGesture>, (Self, WorldInteractionStep)> {",
      "label": "the `Err`-variant is at least 128 bytes"
    },
    {
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 10953,
      "text": "    pub fn return_owner(&mut self, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {",
      "label": "the `Err`-variant is at least 640 bytes"
    },
    {
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 11002,
      "text": "    pub fn finish(&mut self, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {",
      "label": "the `Err`-variant is at least 640 bytes"
    },
    {
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 11086,
      "text": "pub fn return_world3d_asset(state: &mut World3dState, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {",
      "label": "the `Err`-variant is at least 640 bytes"
    },
    {
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 11105,
      "text": "pub fn finish_world3d_asset(state: &mut World3dState, owner: WorldAssetFetchOwner) -> Result<(), WorldAssetFetchOwner> {",
      "label": "the `Err`-variant is at least 640 bytes"
    }
  ],
  "other": [
    {
      "code": "clippy::map_unwrap_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🌐host/🦀️.rs",
      "line": 1052
    },
    {
      "code": "clippy::needless_as_bytes",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/📡️backbone/🔗️binding/🦀️.rs",
      "line": 120
    },
    {
      "code": "clippy::map_unwrap_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🫧️transient/🧵️publication/🦀️.rs",
      "line": 40
    },
    {
      "code": "clippy::type_complexity",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs",
      "line": 148
    },
    {
      "code": "clippy::derivable_impls",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🫧️transient/🦀️.rs",
      "line": 252
    },
    {
      "code": "clippy::type_complexity",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs",
      "line": 323
    },
    {
      "code": "clippy::derivable_impls",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪟️window/🎚️config/🦀️.rs",
      "line": 516
    },
    {
      "code": "clippy::too_many_arguments",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "line": 12132
    },
    {
      "code": "clippy::too_many_arguments",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "line": 12192
    },
    {
      "code": "clippy::question_mark",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "line": 20457
    },
    {
      "code": "clippy::unnecessary_wraps",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "line": 21052
    },
    {
      "code": "clippy::needless_pass_by_value",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "line": 28031
    },
    {
      "code": "clippy::await_holding_refcell_ref",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "line": 28062
    },
    {
      "code": "clippy::large_enum_variant",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "line": 28661
    },
    {
      "code": "clippy::large_enum_variant",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "line": 28669
    },
    {
      "code": "clippy::semicolon_if_nothing_returned",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🦀️.rs",
      "line": 29742
    },
    {
      "code": "clippy::manual_saturating_arithmetic",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 1142
    },
    {
      "code": "clippy::manual_saturating_arithmetic",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 1143
    },
    {
      "code": "clippy::map_unwrap_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 1270
    },
    {
      "code": "clippy::large_enum_variant",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 2471
    },
    {
      "code": "clippy::needless_range_loop",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 2752
    },
    {
      "code": "clippy::needless_range_loop",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 3130
    },
    {
      "code": "clippy::map_unwrap_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 4148
    },
    {
      "code": "clippy::map_unwrap_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 4388
    },
    {
      "code": "clippy::map_unwrap_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 4486
    },
    {
      "code": "clippy::unnecessary_unwrap",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 5253
    },
    {
      "code": "clippy::unnecessary_unwrap",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 5314
    },
    {
      "code": "clippy::needless_pass_by_value",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 6296
    },
    {
      "code": "clippy::too_many_arguments",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 6596
    },
    {
      "code": "clippy::unnecessary_lazy_evaluations",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 8536
    },
    {
      "code": "clippy::map_unwrap_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 8559
    },
    {
      "code": "clippy::map_unwrap_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 10408
    },
    {
      "code": "clippy::map_unwrap_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 10590
    },
    {
      "code": "clippy::map_unwrap_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 10596
    },
    {
      "code": "clippy::large_enum_variant",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 10657
    },
    {
      "code": "clippy::manual_saturating_arithmetic",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🦀️.rs",
      "line": 10742
    },
    {
      "code": "clippy::needless_pass_by_value",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🦀️.rs",
      "line": 62
    },
    {
      "code": "clippy::unnecessary_map_or",
      "file": "🧰️framework/🛍️products/💻️os/🔨️modules/📖️playbook/🗿️artifacts/📖️playbook/🦀️.rs",
      "line": 246
    },
    {
      "code": "clippy::unnecessary_wraps",
      "file": "🧰️framework/🛍️products/💻️os/🎚️config/🧬️schema/🦀️.rs",
      "line": 22
    }
  ]
}
