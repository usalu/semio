# Full Physical Source Audit

Snapshot: 2026-10-27T07:41:56.076814+00:00

Settings: {"language": "de", "quality": "-ql", "resolution": "640,360", "fps": 6.0, "audience": "spectator"}

8 physical scenes; 34 distinct dependency paths including the curriculum manifest. Hashes come from production segment_state, with a separate manifest hash.

Shared runtime, interaction, theme, audience, fonts, visuals and source-owned assets can invalidate all physical beats. Because asset enumeration is recursive under each dependency owner, edits to any qualifying asset under those owners can invalidate a source even when semantically unrelated. Catalog imports may also expose shared lesson changes. JSON changes outside the enumerated owner trees only affect this audit if explicitly imported or listed by source_files. The manifest is separately tracked for selection changes; it is not directly included in segment_state.

<!-- snapshot
{
  "time": "2026-10-27T07:41:56.076814+00:00",
  "settings": {
    "language": "de",
    "quality": "-ql",
    "resolution": "640,360",
    "fps": 6.0,
    "audience": "spectator"
  },
  "files": {
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️theme/🐍️.py": "a7367e0158b70f9d1807f45e1f6a29ca338370218e5e0698ddc49bcf14f346e9",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🐍️.py": "6ec450b3ea3f972bf1c6128767b546eb5649b81900616869d4d3e6090dffba9c",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🔣️.json": "4ce92675f24dca387f3c4919a127b1ff0cbc8dbc41dcc64b8b9579b4146fd90d",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🧬️schema/🔣️.schema.json": "ec7a3e8e7adcae62791283de47b71637eb0956cd59ff1eadc64e5987c2fda170",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🫟️silhouettes/🔣️.json": "78a4574cc406967ebeb1ffd80d917f7f831d1373a515f96606ccd75badd0ecfe",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎨️visuals/🫟️silhouettes/🧬️schema/🔣️.schema.json": "3b284b96469d9d884118776864572426f359a4ed4ec4a1873fd6347df6603336",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/⚙️runtime/🐍️.py": "30238edbfe1490db124f6a1f4541ae61e5373c50353751dff72d93158ff2c61a",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/👥️audience/🐍️.py": "3dc3aa7f26deabb079dac339cda0e838aec091ed0e575531e7f4041b0e0f2060",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/👥️audience/🔣️.json": "c9d90f1c6df6d3120a723c64df733a3439d1417ee722761fc50a71db6598e918",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/👥️audience/🧬️schema.json": "8b4e0d4815406032f0cb427f579023860d57ec420b6720d95abeebe1aafdb543",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/📁️output/🐍️.py": "8ae846987f52ec078bc6610da594b0dd98c2e29e084f9704207db0b642428a11",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/📁️output/🧬️schema/🔣️.schema.json": "32d3f195dca63f0f3e69e5001cf679f9130ee396bb06ce4862a998b731557ac4",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🔣️.json": "a9b3e5e2bf251cd4d5c1b9400afeb0b55d52398ed5409f3d8b63a5ac8ec8b91b",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🎬️scene/🐍️.py": "b59d881649612ef9857fadbc5efb41477c9248e0fdd9ce6753a449ad5ca29378",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🎬️scene/🔣️.schema.json": "3635993a13f98c1f3cfb354a9dc8c61f458da3444a407ecdc657f886ae0ca97b",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🐍️.py": "64c48bb463697ae2357323cd86c6c2e00a2b7397548e548aa52378fe974cae4e",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🔣️.json": "7cda6cd58bd1102885b9d70d84eb5392ef6388b000faf03baf22d4d64d6f47b3",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🎬️clip/🪟️presentation/🖱️interaction/🧬️schema.json": "e5f783a9aa0c7345933b80db5b36439c2ef0603a39794634916ff13287a91959",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔊️speech/🐍️.py": "fc729928d3c3d5a8bd648826fea3febbfc544737b5b8d92112380a16e8ea63c5",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🏗️authoring/🔤️fonts/🐍️.py": "e06d970ce78c3d91a0e5172d6a0d43e122d2e4bbc62b054dad78b7dcfa770ff3",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🧲️physics/🎬️clip/🧲️fundamentals/🐍️.py": "51db5096ee3fb98152112c826f01a62b32427a42526e003c98432830e2dae17b",
    "C:/git/semio/🧰️framework/🔨️modules/📚️compiler/🌍️world/🔤️fonts/🧮️LibertinusMath-Regular.otf": "ded714b0d3808af527438ed7a85f16933133f66a12104c4a83db4cc248888011",
    "C:/git/semio/🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json": "fbaecbd72b868c87bb1989837049bf2ba52f2f6eb58fddebf6cf6be9fab86927",
    "C:/git/semio/🧰️framework/🔨️modules/🖼️assets/👆️cursor/☑️selectable/🌙️dark.svg": "d6205fb1742c1486bdb9c7de449da593b065545db71476d91f5af84b3cc38ffe",
    "C:/git/semio/🧰️framework/🔨️modules/🖼️assets/👆️cursor/👆️pointer/🖋️dark-source.svg": "579b8cfac923f6ec5ac56077b14b0721dd7caa1f8b5577b1932669143a890fdf",
    "C:/git/semio/🧰️framework/🔨️modules/🖼️assets/👆️cursor/🖱️default/🌙️dark.svg": "3de1d11a8023823aa903f5f8a27a0b174841e2f6e80bb26680454f6088a998ff",
    "C:/git/semio/🧰️framework/🔨️modules/🖼️assets/🔣️icons/🔎️viewing/🔲️maximize-2.svg": "f3e009cb6920c2db79d622770724112c82fae7cccf7cdca26fb3822c073abadb",
    "C:/git/semio/🧰️framework/🔨️modules/🖼️assets/🔣️icons/🔎️viewing/🔳️minimize-2.svg": "b5aeb20665ae5f69bc090f4517762bd3f68ecd8c5b7b237dedf14ad0447ac212",
    "C:/git/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts/⌨️share-tech-mono/🏛️latin/📖️regular/🔤️outline.ttf": "31ff08942c8f4ff98276c050f039a2685d91d923867ea6c776c6a291a086fc13",
    "C:/git/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts/📇️catalog.json": "1f0d6d428fb071ec25ede8a6b2b0576ef4733323afd0de2b978250ed126610b0",
    "C:/git/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts/😀️noto-emoji/🌐️complete/📖️regular/🔤️outline.ttf": "eb77128c4f169a4790e48f8040b22ae310ae74ef9ef6bf8066664e375190a4f4",
    "C:/git/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🚀️anta/🏛️latin/📖️regular/🔤️outline.ttf": "6e9112853217f39bc753c94ab0f16fd7dc57fa56003cfb981b0bb7b0c514487f",
    "C:/git/semio/🧰️framework/🔨️modules/🖼️assets/🔤️fonts/🧱️kelly-slab/🏛️latin/📖️regular/🔤️outline.ttf": "33c29e564b169dbbc1a90be9f050fcde7fb204232bc4f3c52bf4b844e31dbba6",
    "C:/git/semio/🎓️teaching/🏛️architecture/⚡️energy/🎬️clip/🔣️.json": "01407e4ad8ab2a665496f1af1ec5e414a1586dc47921c5f8896aee592ac6dcfe"
  },
  "scenes": [
    "Beat1_Leistung",
    "Beat2_EnergieImAlltag",
    "Beat3_Energieerhaltung",
    "Beat4_Waermepumpe",
    "Beat5_Strahlung",
    "Beat6_ThermischeMasse",
    "Beat7_SensibelLatent",
    "Beat8_Zusammenfassung"
  ]
}
-->

## Comparison 2026-10-27T07:42:43.508889+00:00

No dependency hash changes across the physical source closure.

## Comparison 2026-10-27T07:43:51.145392+00:00

No dependency hash changes across the physical source closure.

## Output Environment Verification

Repeated comparison with SEMIO_TEACHING_OUTPUT_ROOT set to the executor CLI root, ticket 🗑️generated/full-physical. No dependency differences versus baseline. Canonical speech paths were not present in either closure. The executor reports child render roots append de/spectator; no parent speech override is set.

Font catalog edits at 🧰️framework/🔨️modules/🖼️assets/🔤️fonts/📇️catalog.json and framework styling at 🧰️framework/🔨️modules/🖱️ui/🎨️styling/🔣️.json are included and invalidate every physical source. Unrelated Python lesson files outside the physical source closure do not directly invalidate these source fingerprints.

## Comparison 2026-10-27T08:00:20.989472+00:00

No dependency hash changes across the physical source closure.
