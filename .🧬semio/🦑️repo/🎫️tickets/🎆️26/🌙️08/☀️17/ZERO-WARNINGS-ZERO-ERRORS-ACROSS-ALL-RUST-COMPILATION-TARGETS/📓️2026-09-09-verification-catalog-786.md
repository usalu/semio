# Verification Catalog

The retained exact-test catalog currently contains 439 selectors in 67 package/target groups. Completed group receipts below are historical evidence, not blanket claims for sources changed after their builds. The active focused run is laws782 (WFC and Writer); the full native fleet compiler check and strict/platform gates remain outstanding.

- semio-s-artifact-mathematical-equation: 20 exact tests; exact-cargo-laws-pPZ9pS; SHA-256 60c47b51ae010fb9c3170c7f922745c7986063cbe180be66cb80daee0c10c616
- semio-s-artifact-block-3d: 6 exact tests; exact-cargo-laws-hft8Lh; SHA-256 d025bb2e150e98210a1ec927035e9563eb639c8a34517b0f0135afde9c715876
- semio-s-plugin-writer: 5 exact tests; exact-cargo-laws-VlTWhm; SHA-256 7682b3d3679266e721329ea425c114688fce171056f6c5c48508a36dcbf3466a

## Build Group Scope Audit

{
  "groups": 67,
  "packages": 67,
  "nonLib": [
    {
      "package": "semio-s-plugin-gis",
      "target": {
        "kind": "test",
        "name": "native_codecs"
      }
    },
    {
      "package": "semio-s-plugin-vcs",
      "target": {
        "kind": "test",
        "name": "native_codecs"
      }
    }
  ],
  "extraArgs": [
    {
      "package": "semio-s-artifact-fem-3d",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-fem-3d/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-block-3d",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-block-3d/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-procedural-generation3d",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-procedural-generation3d/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-procedural-generation2d",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-procedural-generation2d/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-stdio-binary",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-stdio-binary/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-stdio-gif",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-stdio-gif/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-stdio-gltf",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-stdio-gltf/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-space-space",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-space-space/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-procedural-assembly",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-procedural-assembly/component-app-assembly"
      ]
    },
    {
      "package": "semio-framework-os-renderer-wgpu",
      "cargoArgs": [
        "--features",
        "semio-framework-os-renderer-wgpu/native-bin"
      ]
    },
    {
      "package": "semio-s-artifact-puzzle-3d",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-puzzle-3d/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-gis-gismap",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-gis-gismap/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-gis-gisterrain",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-gis-gisterrain/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-fem-2d",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-fem-2d/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-trinity-jack",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-trinity-jack/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-trinity-rewriting",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-trinity-rewriting/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-puzzle-2d",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-puzzle-2d/component-app-assembly"
      ]
    },
    {
      "package": "semio-s-artifact-puzzle-5d",
      "cargoArgs": [
        "--features",
        "semio-s-artifact-puzzle-5d/component-app-assembly"
      ]
    }
  ]
}

## Current Cargo Metadata Validation

Checked every catalog package, target, and explicitly configured feature against native800's fresh Cargo metadata. {
  "groups": 67,
  "issues": []
}
