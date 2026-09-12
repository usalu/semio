
## baseline Runtime

```json
{
  "flowDrawList": "pass"
}
```

## baseline Runtime

```json
{
  "passed": 34,
  "failed": 1,
  "files": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌳️wgpu-document-reconcile/🟦️.ts",
      "state": "pass"
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts",
      "state": "pass"
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖌️wgpu-document-owner-move/🟦️.ts",
      "state": "pass"
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔢️wgpu-u64-seam/🟦️.ts",
      "state": "fail"
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️wgpu-module-routes/🟦️.ts",
      "state": "pass"
    }
  ],
  "errors": []
}
```

## Cross-Runtime Error Contract Repair

The first five-suite baseline ran 34/35 assertions successfully. The failing assertion compared engine-specific BigInt TypeError message wording: Bun emitted a different message than V8. The neutral vector now declares the public error class TypeError and the actual test verifies that class, retaining the invalid-number and valid-BigInt checks. Updated before the move: `🔢️wgpu-u64-seam/🟦️.ts` and its `laws.json`.

## Exact Data Moves And Consumers

All thirteen moves preserved source bytes immediately; subsequent fixture provenance text updates are captured by final hashes. Executable case paths are unchanged.

```json
{
  "moves": [
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🌉️scene-bridge/🔣️.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🌉️scene-bridge/🔣️.json",
      "sha256": "51921edfdce13db114cf9ada661bde9d7e2b84b03f0c1d62369e8bd53d0919b1"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧪️tests/🎬️draw-list/📐️expected-draw-list.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/🎬️draw-list/📐️expected-draw-list.json",
      "sha256": "21822096bf63dd6fa435c241139552ac5e1805f8bbf3ceeeaf612787c457e650"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧪️tests/🎬️draw-list/🔣️.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/🎬️draw-list/🔣️.json",
      "sha256": "a8e74c13e21cd3896f4b41bfa06362db910fb8b86593d4cfb13887dfc5bf9f40"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧪️tests/🏷️label-fit/🔣️.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/🏷️label-fit/🔣️.json",
      "sha256": "600e32addd370bf9fd2edc182c5e6af8aae209a885dba32cf58fa80bd58243b0"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧪️tests/📷️camera-fit/🔣️.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json",
      "sha256": "97601e93b09c43072e6dbcf73b753c3bbe2b1574b1d6c6f6dc3f4e8cf592ef4a"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🔣️.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧫️fixtures/🎬️draw-list/🔣️.json",
      "sha256": "eb9fa066562e705ae932970c73fcaf1566ce85e798b91a805a5c45e88a7cd568"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌳️wgpu-document-reconcile/laws.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🌳️wgpu-document-reconcile/🔣️.json",
      "sha256": "5040455bfae825a21ae7f698b7ec9a935d58d75e77e8e293572ccb8464ba3446"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔢️wgpu-u64-seam/laws.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🔢️wgpu-u64-seam/🔣️.json",
      "sha256": "44661ade15b97b9e0edf9b76fe75af53f2ed38441504fb877294ec7c108a4d1b"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/laws.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🔬️wgpu-extension-dispatch/🔣️.json",
      "sha256": "5c1681ae4313b5caf59dce538592a962550da031c580c5c9ac0b70554f9aff57"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖌️wgpu-document-owner-move/laws.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🖌️wgpu-document-owner-move/🔣️.json",
      "sha256": "04d432a02f4848395782a4ab37ef2a38ef67e779d0c49db4c8eca6561dc6ec88"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️wgpu-module-routes/laws.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧩️wgpu-module-routes/🔣️.json",
      "sha256": "1d3c0c96a0130620273c9ef925f9f47bd7eaec62c936e999509faeebe6265c52"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🔣️.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph/🔣️.json",
      "sha256": "f8667b0878f1036b2a77cb144f07e821d5a3e5987f3609f798e3b3c54bfa1b74"
    },
    {
      "oldPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🔣️.json",
      "newPath": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧫️fixtures/🧩️wgpu-engine-surfaces/🔣️.json",
      "sha256": "7b458433b8973c7aae0acc2c3fc8f23393bc89b73af304fd71fec79e6a39c890"
    }
  ],
  "updated": [
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧪️tests/🎬️draw-list/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧪️tests/🏷️label-fit/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧪️tests/📷️camera-fit/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧪️tests/🎬️draw-list/🟨️.js",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌳️wgpu-document-reconcile/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔢️wgpu-u64-seam/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖌️wgpu-document-owner-move/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️wgpu-module-routes/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🕸️wgpu-node-graph/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧪️tests/🧩️wgpu-engine-surfaces/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧫️fixtures/🧩️wgpu-engine-surfaces/🔣️.json",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🎯️targets/🧊️wgpu/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🕸️NodeGraph/🟦️.tsx",
    "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧪️tests/🔬️unit/🦀️.rs",
    "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧫️fixtures/🎬️draw-list/🔣️.json",
    "✏️s/🔌️plugins/🌀️procedural/🗿️artifacts/🧊️generation3d/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🧪️tests/🧩️geometry/🦀️.rs"
  ],
  "finalHashes": [
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🧫️fixtures/🌉️scene-bridge/🔣️.json",
      "sha256": "51921edfdce13db114cf9ada661bde9d7e2b84b03f0c1d62369e8bd53d0919b1"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/🎬️draw-list/📐️expected-draw-list.json",
      "sha256": "21822096bf63dd6fa435c241139552ac5e1805f8bbf3ceeeaf612787c457e650"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/🎬️draw-list/🔣️.json",
      "sha256": "a8e74c13e21cd3896f4b41bfa06362db910fb8b86593d4cfb13887dfc5bf9f40"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/🏷️label-fit/🔣️.json",
      "sha256": "600e32addd370bf9fd2edc182c5e6af8aae209a885dba32cf58fa80bd58243b0"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️canvas/🧫️fixtures/📷️camera-fit/🔣️.json",
      "sha256": "97601e93b09c43072e6dbcf73b753c3bbe2b1574b1d6c6f6dc3f4e8cf592ef4a"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/🌊️flow/🕸️wasm/🧫️fixtures/🎬️draw-list/🔣️.json",
      "sha256": "68fe4a82dd0539802189074dbdd49f009551e3f305fcb43b01e92bc6087bc373"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🌳️wgpu-document-reconcile/🔣️.json",
      "sha256": "5040455bfae825a21ae7f698b7ec9a935d58d75e77e8e293572ccb8464ba3446"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🔢️wgpu-u64-seam/🔣️.json",
      "sha256": "44661ade15b97b9e0edf9b76fe75af53f2ed38441504fb877294ec7c108a4d1b"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🔬️wgpu-extension-dispatch/🔣️.json",
      "sha256": "5c1681ae4313b5caf59dce538592a962550da031c580c5c9ac0b70554f9aff57"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🖌️wgpu-document-owner-move/🔣️.json",
      "sha256": "04d432a02f4848395782a4ab37ef2a38ef67e779d0c49db4c8eca6561dc6ec88"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧫️fixtures/🧩️wgpu-module-routes/🔣️.json",
      "sha256": "1d3c0c96a0130620273c9ef925f9f47bd7eaec62c936e999509faeebe6265c52"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧫️fixtures/🕸️wgpu-node-graph/🔣️.json",
      "sha256": "f8667b0878f1036b2a77cb144f07e821d5a3e5987f3609f798e3b3c54bfa1b74"
    },
    {
      "path": "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/⚙️EngineCanvas/🧫️fixtures/🧩️wgpu-engine-surfaces/🔣️.json",
      "sha256": "a5a12c77dc906893cc9048956fded71b5d7e09e215f9696e8dc6c13360c3e5b4"
    }
  ]
}
```

## verify Runtime

```json
{
  "flowDrawList": "pass"
}
```

## verify Runtime

```json
{
  "passed": 35,
  "failed": 0,
  "files": [
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔢️wgpu-u64-seam/🟦️.ts",
      "state": "pass"
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🌳️wgpu-document-reconcile/🟦️.ts",
      "state": "pass"
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-extension-dispatch/🟦️.ts",
      "state": "pass"
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🖌️wgpu-document-owner-move/🟦️.ts",
      "state": "pass"
    },
    {
      "path": "/Users/ueli/Documents/semio/🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🧩️wgpu-module-routes/🟦️.ts",
      "state": "pass"
    }
  ],
  "errors": []
}
```

## node-oracle Runtime

```json
{
  "exitCode": 0,
  "stdout": "[DEBUG] Node V8 independently confirmed the u64 error class and successful BigInt lowering\n",
  "stderr": ""
}
```
