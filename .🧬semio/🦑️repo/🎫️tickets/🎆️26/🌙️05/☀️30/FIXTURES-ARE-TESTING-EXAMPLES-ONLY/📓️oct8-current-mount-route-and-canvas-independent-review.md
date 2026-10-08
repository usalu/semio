# Current Mount Route and Canvas Independent Review

WGPU dev/release server selects corresponding wasm-dev/wasm-release compiler root and explicitly static-mounts /renderer-modules/wgpu plus boot/library/worker roots. Exact staged JS/WASM raw candidates require current Trunk receipts and HTTP length/hash equality, not dist presence. React dev/release uses profile-matched aliases/fsallow but no corresponding raw WGPU static mount; Vite JavaScript may transform. Do not apply raw stagedJS hash to a transformed response. Genuine asset provider mounts for CAD/Puzzle are shared source declarations and support separate raw JSON/PNG candidates after current publication.

Current generated maps still select /cad-assets CAD example assets and /infinite-assets Infinite assets; these are asset roots, not fixture collections. Existing candidates/current physical path observations:

```json
[
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🖼️assets/✏️sketch/🖼️.png",
    "exists": true,
    "file": true,
    "sha256": "cb1c934dff7b26101b3126ac1d63ab0865f96ccb319faa72a85e791f3ec079ad"
  },
  {
    "path": "🧰️framework/🔨️modules/🖼️canvas/🖼️assets/✏️sketch/🖼️.png",
    "exists": false,
    "file": false,
    "sha256": null
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/📦️packages/🦀️rust/dist/fonts",
    "exists": true,
    "file": false,
    "sha256": null
  },
  {
    "path": "🧰️framework/🔨️modules/🖼️canvas/🎨️react-renderer/📦️packages/🟦️typescript/🟦️.tsx",
    "exists": true,
    "file": true,
    "sha256": "9bdbdae252eea9d8180e6d882edb1379cb578132d78bfe39e09d4dbb7485ad6a"
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/♾️infinite/🌍️world/🎨️r3f/📦️packages/🟦️typescript/🟦️.tsx",
    "exists": true,
    "file": true,
    "sha256": "9bdbdae252eea9d8180e6d882edb1379cb578132d78bfe39e09d4dbb7485ad6a"
  },
  {
    "path": "✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/📚️examples/🖼️assets/🎮️play/🌲️hexagonal-cut-concrete-forest-right.model.json",
    "exists": true,
    "file": true,
    "sha256": "d9ca072461dae38701f44ce093e76c684becadbad200b7a0923ebaa68de2bb56"
  }
]
```

Current React config names both new canvas-react-renderer alias and existing infinite-world-r3f alias, and fonts under Infinite Rust dist. Generated puzzle/aggregator asset route still names Infinite assets. Do not guess a Canvas replacement from nearby moved files: actual original asset/font roots and owner targets must agree at materialization. Absent generated fonts is a producer prerequisite, not authored fixture authority. This source snapshot supplies no HTTP/producer proof or writer attribution. No server, compiler, publication or lifecycle command executed.
