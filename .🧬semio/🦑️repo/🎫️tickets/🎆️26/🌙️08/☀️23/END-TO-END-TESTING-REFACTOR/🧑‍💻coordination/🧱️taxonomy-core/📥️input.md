# Core Testing Taxonomy Exact Inputs

```json
[
  {
    "sourcePath": "🧰️framework/🔨️modules/⏳️async/🟦️.ts",
    "destPath": "🧰️framework/🔨️modules/⏳️async/🧪️tests/🧱️boxed-fixed-slots/🟦️.ts",
    "sourceBeforeSha256": "7a2b83e0ad520b642189e065f856745f64b8174061b1c9dd32f43f0e22c3af12"
  },
  {
    "sourcePath": "🧰️framework/🔨️modules/⏳️async/🪃️continuation/🟦️.ts",
    "destPath": "🧰️framework/🔨️modules/⏳️async/🪃️continuation/🧪️tests/🪃️scheduler/🟦️.ts",
    "sourceBeforeSha256": "32ada42f806d69a07fdea0f0ad69b4f2206ca7d7ab465e1bf946fa4cb8e9a8d2"
  },
  {
    "sourcePath": "🧰️framework/🔨️modules/🎠️kernel/🟦️.ts",
    "destPath": "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🔬️scope-contributions/🟦️.ts",
    "sourceBeforeSha256": "6386a4f76a9d18dd9376e9608c4853ef4c25d43b6b91d81043de2b0860c86366"
  },
  {
    "updated": "🧰️framework/🔨️modules/⏳️async/📦️packages/🟦️typescript/vitest.config.ts"
  },
  {
    "updated": "🧰️framework/🔨️modules/🎠️kernel/📦️packages/🟦️typescript/vitest.config.ts"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🎠️kernel/🧪️tests/🔬️scope-contributions/vectors.json",
    "newPath": "🧰️framework/🔨️modules/🎠️kernel/🧫️fixtures/🔬️scope-contributions/🔣️.json",
    "sha256": "ebd16951714d2ba81b97e727c6ae12c8143f4db07623eefe5481ff8814e75760"
  },
  {
    "oldPath": "🧰️framework/🔨️modules/🎭️actor/📦️packages/🟦️typescript/🧫️fixtures/📨️effect-wire-routes.json",
    "newPath": "🧰️framework/🔨️modules/🎭️actor/🧫️fixtures/📨️effect-wire-routes/🔣️.json",
    "sha256": "41f93a0108b7232915b057ce5df63e556a1b40e1aa6a218ab73a877730130310"
  },
  {
    "oldPath": "♻️mit-bestand/📋️bericht/🧫️tests/📄️pdf.json",
    "newPath": "♻️mit-bestand/📋️bericht/🧫️fixtures/📄️pdf.json",
    "sha256": "40699ed145a594928a9d9ef056e174a28a544e29d3067eb0c55f5806cb30ea1e"
  },
  {
    "oldPath": "♻️mit-bestand/📋️bericht/🧫️tests/🔣️report-family.json",
    "newPath": "♻️mit-bestand/📋️bericht/🧫️fixtures/🔣️report-family.json",
    "sha256": "632a554d6f41e7bf491e09710eb2974ef098d197527bfc9ca2df7070abd317ba"
  },
  {
    "oldPath": "♻️mit-bestand/📋️bericht/🧫️tests/🧭️commands.json",
    "newPath": "♻️mit-bestand/📋️bericht/🧫️fixtures/🧭️commands.json",
    "sha256": "cc87ca102d9c807edb956f3d47a392f5d8f713f8cb3c72f39cd2ed95f2565afc"
  },
  {
    "updated": "🧰️framework/🔨️modules/🎭️actor/🧪️tests/📨️effect-wire-routes/🟦️.ts"
  },
  {
    "updated": "♻️mit-bestand/📋️bericht/📦️packages/🟦️typescript/📜️script.ts"
  }
]
```

## Direct Native Fixture Readers

```json
{
  "source": "🧰️framework/🔨️modules/⏳️async/🦀️.rs",
  "reads": [
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-renderer-kernel-runtime-semantic-document/🦀️.rs",
      "fixture": "🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json",
      "include": "../../../../../../../🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs",
      "fixture": "🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json",
      "include": "../../../../../../../../../🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎞️Scenes/🧪️tests/🔬️wgpu-admitted-surface-map/🦀️.rs",
      "fixture": "🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json",
      "include": "../../../../../../../../../🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs",
      "fixture": "🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json",
      "include": "../../../../../../../🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧪️tests/🔬️renderer-opaque-scene-retirement/🦀️.rs",
      "fixture": "🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json",
      "include": "../../../../../../../🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json"
    },
    {
      "path": "🧰️framework/🔨️modules/🖱️ui/🧪️tests/🔬️targets-wgpu-engine-unit/🦀️.rs",
      "fixture": "🧰️framework/🔨️modules/⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json",
      "include": "../../../⏳️async/🧫️fixtures/🧱️boxed-fixed-slots/🔣️.json"
    }
  ]
}
```
