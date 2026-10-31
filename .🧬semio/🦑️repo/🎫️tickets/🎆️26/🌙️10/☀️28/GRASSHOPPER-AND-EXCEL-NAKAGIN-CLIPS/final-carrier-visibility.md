# Current Carrier-Aware Finite Visibility Gate

The corrected independent forward oracle passes its 13-test NumPy/Pillow/Rhino/SciPy suite and all three assigned finite source queries. Across 1,260 pixel regions and 31,500 centre/subpixel rays, full, stable and culled paint each match independent original physical nearest material. There are zero unsupported-source, layer inversion or removed-nearest samples. Capital has zero visible steel in this closed north/west view. Every producer and query endpoint map is unchanged.

This is a finite sampling result, not exhaustive raster coverage or a universal closed-head rule: south louvre openings can legitimately expose internal material in other views. No Cairo capture was duplicated, no colour tolerance was applied, and no source-rank fallback was used.

## Conditioning Regression

The actual baseline silently dropped a positive triangle of area 6.211953942447299e-11: zero paint triangles were emitted. The source-bound baseline is paint-lineage/red.json, SHA-256 9360146cfe6def3015b4528583cbb01d5838c363a7536b01e16843d6159c1e87. The correction independently reconstructs canonical original carriers, validates the exported stable paint plane against original source geometry, and projects/solves depth through that carrier. Three-vertex positive material cannot silently become empty. Unmatched planes explicitly reject. Physical nearest and facing still derive only from original canonical geometry and winding.

The corrected witness retains one exact triangle and agrees with an independent original-source SVD plane to 5.551115123125783e-17. Green record SHA-256: 0afb164c5ac9869c00e2557cf6cba4a600dec03f3c10c79a5cf15d6fa2709430. The permanent suite tests wrong-normal/off-carrier rejection and a scaled normal, plus concave/disconnected material, transparency, compound alpha, canonical shortest-diagonal warped faces, true physical facing and positive tiny fragments. Its 13 tests passed with exact unchanged source endpoints; result SHA-256 683a5eb6e818ab3ac5abfa435ed288f6c32cd679b84bfe9d656db8c0301ffd91.

Runtime witness: Python 3.12.11, NumPy 2.5.4; project oracle hash 9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470. Schema controls: six paint/five mesh/four convex cases, ten malformed contracts rejected; separate carrier case accepts its exact original parent/triangle and rejects three malformed controls.

## Unchanged Cairo Boundary Ownership

Capsule256 retains two identical full/stable/culled Cairo centre-ID differences from ideal point geometry:

| Pixel | Cairo source in every stage | Actual centre nearest | Projected shared-edge distance | 25-sample nearest counts |
|---|---:|---:|---:|---|
| (52,84) | 63 | 42 | 0.0014365413187 px | 42:13, 63:12 |
| (78,90) | 64 | 63 | 0.0039859360375 px | 63:13, 64:12 |

All owners are retained opaque shell material. Sources42/63 share the exact world edge x=1.3,z=2.25,y=0.1499999999999999..3.8499999999999996. Sources63/64 share x=1.226776695296637,z=2.426776695296637 over the same y range. Cairo's selected adjacent owner has no actual centre ray hit; these are not claimed coplanar depth ties. Every one of the 25 point-painter samples per pixel is correct in every stage. The unchanged Cairo ownership is preserved explicitly; no claim of perfect Cairo centre attribution or AA waiver is made.

## Exact Current Results

```json
[
  {
    "file": "capital-carrier-final-ray.json",
    "sha256": "2f73b60fbdb8d349ed0c661114d3dc367ff4bd50088fe78880aebbc28e59f68e",
    "pixels": 938,
    "rays": 23450,
    "source_items": 2602,
    "paint_piece_rows": 14843,
    "paint_unique_polygons": 5578,
    "physical_triangles": 2392,
    "paint_triangles": 11084,
    "elapsed_seconds": 20.95617460000358,
    "changed_id_pixels": {
      "full-stable": 0,
      "stable-culled": 0,
      "full-culled": 0
    },
    "summary": {
      "full_wrong_nearest_samples": 0,
      "full_unsupported_samples": 0,
      "full_layer_inversion_samples": 0,
      "stable_wrong_nearest_samples": 0,
      "stable_unsupported_samples": 0,
      "stable_layer_inversion_samples": 0,
      "culled_wrong_nearest_samples": 0,
      "culled_unsupported_samples": 0,
      "culled_layer_inversion_samples": 0,
      "physical_visible_steel_samples": 0,
      "physical_culling_nearest_changed_samples": 0,
      "full_cairo_physical_mismatch_centres": 0,
      "full_cairo_paint_mismatch_centres": 0,
      "stable_cairo_physical_mismatch_centres": 0,
      "stable_cairo_paint_mismatch_centres": 0,
      "culled_cairo_physical_mismatch_centres": 0,
      "culled_cairo_paint_mismatch_centres": 0
    },
    "progress_journal": "🗑️generated\\capital-carrier-final-ray.progress.jsonl",
    "progress_sha256": "de7e9a1de1d873b38f5c5f3ee613ab89c2a13e33dc3a5078877abf516b4dbff1"
  },
  {
    "file": "capsule-carrier-final-ray-256.json",
    "sha256": "9f756bf96ce3144b2efd85fccc3acab14633c91aad69af4c75c815f6f2fd26d9",
    "pixels": 319,
    "rays": 7975,
    "source_items": 2368,
    "paint_piece_rows": 7094,
    "paint_unique_polygons": 4126,
    "physical_triangles": 3728,
    "paint_triangles": 8128,
    "elapsed_seconds": 7.080220299998473,
    "changed_id_pixels": {
      "full-stable": 0,
      "stable-culled": 0,
      "full-culled": 0
    },
    "summary": {
      "full_wrong_nearest_samples": 0,
      "full_unsupported_samples": 0,
      "full_layer_inversion_samples": 0,
      "stable_wrong_nearest_samples": 0,
      "stable_unsupported_samples": 0,
      "stable_layer_inversion_samples": 0,
      "culled_wrong_nearest_samples": 0,
      "culled_unsupported_samples": 0,
      "culled_layer_inversion_samples": 0,
      "physical_visible_steel_samples": 0,
      "physical_culling_nearest_changed_samples": 0,
      "full_cairo_physical_mismatch_centres": 2,
      "full_cairo_paint_mismatch_centres": 2,
      "stable_cairo_physical_mismatch_centres": 2,
      "stable_cairo_paint_mismatch_centres": 2,
      "culled_cairo_physical_mismatch_centres": 2,
      "culled_cairo_paint_mismatch_centres": 2
    },
    "progress_journal": "🗑️generated\\capsule-carrier-final-ray-256.progress.jsonl",
    "progress_sha256": "2a1d1e12d2268253c0a6a5812a4c536c9e9c92547a86825e029e3c4dac2a2e36"
  },
  {
    "file": "capsule-carrier-final-ray-512.json",
    "sha256": "11db7f7a5ec037b63bb1a4d9601868647b2df19f5538ebd0415e611cfcbbbe3c",
    "pixels": 3,
    "rays": 75,
    "source_items": 2368,
    "paint_piece_rows": 6115,
    "paint_unique_polygons": 3752,
    "physical_triangles": 3728,
    "paint_triangles": 7377,
    "elapsed_seconds": 0.11724210000102175,
    "changed_id_pixels": {
      "full-stable": 0,
      "stable-culled": 0,
      "full-culled": 0
    },
    "summary": {
      "full_wrong_nearest_samples": 0,
      "full_unsupported_samples": 0,
      "full_layer_inversion_samples": 0,
      "stable_wrong_nearest_samples": 0,
      "stable_unsupported_samples": 0,
      "stable_layer_inversion_samples": 0,
      "culled_wrong_nearest_samples": 0,
      "culled_unsupported_samples": 0,
      "culled_layer_inversion_samples": 0,
      "physical_visible_steel_samples": 0,
      "physical_culling_nearest_changed_samples": 0,
      "full_cairo_physical_mismatch_centres": 0,
      "full_cairo_paint_mismatch_centres": 0,
      "stable_cairo_physical_mismatch_centres": 0,
      "stable_cairo_paint_mismatch_centres": 0,
      "culled_cairo_physical_mismatch_centres": 0,
      "culled_cairo_paint_mismatch_centres": 0
    },
    "progress_journal": "🗑️generated\\capsule-carrier-final-ray-512.progress.jsonl",
    "progress_sha256": "e1a5bee5c89aaa282a07da15329b1e80c79380e600462f19bc25d67b056d96ab"
  }
]
```

## Complete Unchanged Query Input Maps

The following union contains every exact producer artifact, camera, original polygon, actual paint-fragment, image, ROI, carrier contract and independent implementation/model input. Shared keys have identical values across queries; each complete start map equals its own end map.

```json
{
  "🗑️generated\\capital-final-repaired\\2048-geometric-culled.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
  "🗑️generated\\capital-final-repaired\\2048-geometric-full.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
  "🗑️generated\\capital-final-repaired\\2048-geometric-stable.png": "6e1753d4fa34882dd5b7ce604d96acc53d3272207249fb1387fed7df464968b0",
  "🗑️generated\\capital-final-repaired\\2048-material-culled.png": "61a2a2b3aceb4cf8a51f9ce07702004b8af54deb1a42083fb602818a26c083d2",
  "🗑️generated\\capital-final-repaired\\2048-material-full.png": "d8f8909a1f14d8e0533af73a369c73de35c1c91bece51b09ea519bb8a25c62d9",
  "🗑️generated\\capital-final-repaired\\2048-material-stable.png": "61a2a2b3aceb4cf8a51f9ce07702004b8af54deb1a42083fb602818a26c083d2",
  "🗑️generated\\capital-final-repaired\\2048-silhouette-culled.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
  "🗑️generated\\capital-final-repaired\\2048-silhouette-full.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
  "🗑️generated\\capital-final-repaired\\2048-silhouette-stable.png": "502e82c46dbad5d241004c788b8fe550706d4d3da93cb89c095ac2a36b2727a3",
  "🗑️generated\\capital-final-repaired\\512-aa-fill-culled.png": "ca8c312edaff39fa660b910f347829c2a318c73544f253bfc6bc933defda0917",
  "🗑️generated\\capital-final-repaired\\512-aa-fill-full.png": "89f6195a0c0c2ebd82e42884fb5bf272bdd16541ddd5e6c6f29f742f8582f8ef",
  "🗑️generated\\capital-final-repaired\\512-aa-fill-stable.png": "0b709781d8acbf9bf046252b93206cdc00592dbe215e7f034e49392dcb9130ef",
  "🗑️generated\\capital-final-repaired\\512-aa-strokes-culled.png": "fc1895120a606f3fe0a8655d350176f30091522066365824fa7b9e1fc07e7e5c",
  "🗑️generated\\capital-final-repaired\\512-aa-strokes-full.png": "26453b906d6b827195aedc08e05d85dfb2cff261ceaaf6d8387a971a2201befb",
  "🗑️generated\\capital-final-repaired\\512-aa-strokes-stable.png": "8aca4be3da8d42f790a180edd35be4a437512e992154fbf2665a82373fad6e27",
  "🗑️generated\\capital-final-repaired\\512-geometric-culled.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
  "🗑️generated\\capital-final-repaired\\512-geometric-full.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
  "🗑️generated\\capital-final-repaired\\512-geometric-stable.png": "9db6494696402f4a57294cbc379a93264ee06b15626d039ffb328f386a7e56ab",
  "🗑️generated\\capital-final-repaired\\512-ids-culled.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
  "🗑️generated\\capital-final-repaired\\512-ids-full.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
  "🗑️generated\\capital-final-repaired\\512-ids-stable.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
  "🗑️generated\\capital-final-repaired\\512-material-culled.png": "d88acb18ebacc1f6a40c5b95e1ca4b07ca7258868970146e1f46f71047d1769f",
  "🗑️generated\\capital-final-repaired\\512-material-full.png": "120e8b7b9e9ac5274e452a6d5559f9c75c4186ba403292e6d87a2c25b597e263",
  "🗑️generated\\capital-final-repaired\\512-material-stable.png": "65aee655dce05421f7db3c13937fffe023e63588e1ea117500f4d0749a8bedac",
  "🗑️generated\\capital-final-repaired\\512-silhouette-culled.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
  "🗑️generated\\capital-final-repaired\\512-silhouette-full.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
  "🗑️generated\\capital-final-repaired\\512-silhouette-stable.png": "9fdbda81f1d3f45c0cce2a8c299cae8d1c417e257d64fd8d22a148d3ab915e8a",
  "🗑️generated\\capital-final-repaired\\coverage-aa-controls.json": "42c9f518f77cab20e4ee8485fbadda7cfc420de8b84d3bbb244638ff00d32fee",
  "🗑️generated\\capital-final-repaired\\coverage-camera.json": "7cd6d7701b5b146fab71e78da1c3f672f15f27f8236f68c60b31a8f3c184c434",
  "🗑️generated\\capital-final-repaired\\coverage-items.json": "e077435b269eb957ad0abc17ca621de4411224770f9bf0470b1f11e1291433ba",
  "🗑️generated\\capital-final-repaired\\coverage-paint-fragments.json": "8b3b641a5fab3526e0a90002b2501c1ae16eaa2d8cb1bdb1a5763c03ffd2e57e",
  "🗑️generated\\capital-final-repaired\\coverage-provenance.json": "4a98f9e2163b18ed81b1fc9acfa222bb7a1d2c5d8fb68aec898491ede1d3cc4a",
  "run-fragment-surface-audit.py": "1e197edfde3bbf32b7a1a36844b05d28844427dbf5623bfc72e9944b72be2caa",
  "pixel-nearest-surface.py": "9635caee91ec33b846068a7c6dee1af33a40b3a933a0e88aa1b94460381dc470",
  "pixel-ray-cases.json": "ab0ff8ee0636275f4bcfdbf9a7ef8848592216517840addb3182406fcdc65200",
  "pixel-plane-lineage-cases.json": "20004a975344549ac5379a3dcad792c9faadcb94f03726d225a5ab0d334b7112",
  "pixel-plane-lineage.schema.json": "62cac0d810fa17c54bc239f47ef856fb822fd46de93cc395d4d26ea8ee06af4e",
  "pixel-paint-fragments.schema.json": "ed0183df51d2811483139e1fd91ab9593d575bc2c98990316c0ea4f0c5d744db",
  "pixel-source-export.schema.json": "d5e7b8761c162bcff3cec340f97b113890e63cf1ca70b5d70ab5770fd18c59cc",
  "test-pixel-nearest-surface.py": "381e557b2158893b0d76a0e46902ec733ba064bc252fa45bcd431bd11c916768",
  "pixel-warped-mesh-cases.json": "30aec9f23e04c592c94c60a981e1f98347903a07d813a4f0c503180be2b1023b",
  "pixel-warped-mesh.schema.json": "9166c9d3feaf8c72fa01e52be71c8fa7950dd947e8a23ead2176bccc5837bc1f",
  "pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
  "pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
  "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
  "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
  "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
  "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧬️schema\\🔣️.schema.json": "7a16cfd7438af18638c7ae5c1f884094f8a94df918dcd42f2eb4084a85a00514",
  "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
  "🗑️generated\\capital-fragment-roi.json": "08e7796732e6beaaf0596f3b887055f53f26c516cbf7f4e7b3762049d2dd503f",
  "🗑️generated\\capsule-final-repaired256-retry\\1024-geometric-culled.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
  "🗑️generated\\capsule-final-repaired256-retry\\1024-geometric-full.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
  "🗑️generated\\capsule-final-repaired256-retry\\1024-geometric-stable.png": "5edb6a15318aded874ca0d11754bf84d6e06fed8e538e5cb4dfa899e5da84d6d",
  "🗑️generated\\capsule-final-repaired256-retry\\1024-material-culled.png": "b2f2f295537ba3968caf9ae0af357c3304b3b2a6bcb3d0bcde6282a01bb7b413",
  "🗑️generated\\capsule-final-repaired256-retry\\1024-material-full.png": "3b7df958e7cdc024f9d034357e0918f6f385ea315107c48f88e5f7c7dda78bfe",
  "🗑️generated\\capsule-final-repaired256-retry\\1024-material-stable.png": "aabcf4e21ad1039502d0139a2470c1ffd12619908ea47f233babd66063d4430b",
  "🗑️generated\\capsule-final-repaired256-retry\\1024-silhouette-culled.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
  "🗑️generated\\capsule-final-repaired256-retry\\1024-silhouette-full.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
  "🗑️generated\\capsule-final-repaired256-retry\\1024-silhouette-stable.png": "d146a6577ef01b08eda9560bc518fcb7163440f758f37813974625a8454586f0",
  "🗑️generated\\capsule-final-repaired256-retry\\256-aa-fill-culled.png": "1e3eb33625a5ec2c0895a594315f85a5d4a3de7acaca0a33990035de3163da59",
  "🗑️generated\\capsule-final-repaired256-retry\\256-aa-fill-full.png": "63448597d7248b0cba26dce58969b43847ebc2c179bffd4626b5f5ebd57c58da",
  "🗑️generated\\capsule-final-repaired256-retry\\256-aa-fill-stable.png": "0d5cc519da9d8976a82a626eaa9d5bbf7ed174317943522073ade57d4461bb4e",
  "🗑️generated\\capsule-final-repaired256-retry\\256-aa-strokes-culled.png": "657eab6b93306dd5b380a5c5ec56d22da3679fa746a2b5dfea794f5a396d4650",
  "🗑️generated\\capsule-final-repaired256-retry\\256-aa-strokes-full.png": "3deba40daab0544f67c5b53c2d7f2e5dd82a5e5559eed2ddb517e5683c5be325",
  "🗑️generated\\capsule-final-repaired256-retry\\256-aa-strokes-stable.png": "ba33e2c171639361b897923cb3d9c4f814b808f9c6d0fb4ce418a01f6a57e834",
  "🗑️generated\\capsule-final-repaired256-retry\\256-geometric-culled.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
  "🗑️generated\\capsule-final-repaired256-retry\\256-geometric-full.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
  "🗑️generated\\capsule-final-repaired256-retry\\256-geometric-stable.png": "33e9473747679843bb94005f6f3860330eb7bf50943a164371070520adf087fb",
  "🗑️generated\\capsule-final-repaired256-retry\\256-ids-culled.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
  "🗑️generated\\capsule-final-repaired256-retry\\256-ids-full.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
  "🗑️generated\\capsule-final-repaired256-retry\\256-ids-stable.png": "f59ed41776afabcda60fa9d7411c9b3a8a86cce7fb6059f9c4c471a2ac39f60c",
  "🗑️generated\\capsule-final-repaired256-retry\\256-material-culled.png": "04533916618013a0086ba105d9f07ae1b8b51f01bf78eead398f2c7bbe4941dc",
  "🗑️generated\\capsule-final-repaired256-retry\\256-material-full.png": "494c712a7ab3f4a74da1c39de798705e5bb401af87048c33ff5070e225c3ccd8",
  "🗑️generated\\capsule-final-repaired256-retry\\256-material-stable.png": "10315d5b4d38f4ac53261ad018bbf53b4fdb8a3f43b9a923e5c6d4f3ced2667c",
  "🗑️generated\\capsule-final-repaired256-retry\\256-silhouette-culled.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
  "🗑️generated\\capsule-final-repaired256-retry\\256-silhouette-full.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
  "🗑️generated\\capsule-final-repaired256-retry\\256-silhouette-stable.png": "8e0c3a47109cc0e22af05d224c939f4723835bf19d818b348d33eeb6c7ff1b22",
  "🗑️generated\\capsule-final-repaired256-retry\\coverage-aa-controls.json": "af79d35598191e5451898b3aaae9c6d774d65239731e980fdde39460e951f20a",
  "🗑️generated\\capsule-final-repaired256-retry\\coverage-camera.json": "75b599f98ffd63699a2c24976ab73615db7b4c2cc1ba98d6251cb7bf3f744ee4",
  "🗑️generated\\capsule-final-repaired256-retry\\coverage-items.json": "dd3139922be2102d3baa61c2effddf64d0679d0b102c715ce0a2c336317703ce",
  "🗑️generated\\capsule-final-repaired256-retry\\coverage-paint-fragments.json": "e8e0b9d86b4d8564f7b86a229618ffc56651fc6d51e39debdaf7b9b64b50910d",
  "🗑️generated\\capsule-final-repaired256-retry\\coverage-provenance.json": "4a98f9e2163b18ed81b1fc9acfa222bb7a1d2c5d8fb68aec898491ede1d3cc4a",
  "🗑️generated\\capsule-fragment-roi-256.json": "ec164a488f13b3b172489909722578ce7d8acbb99d458c78c0004bdaac13da04",
  "🗑️generated\\capsule-final-repaired512\\2048-geometric-culled.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
  "🗑️generated\\capsule-final-repaired512\\2048-geometric-full.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
  "🗑️generated\\capsule-final-repaired512\\2048-geometric-stable.png": "47b45ee8686a9f149b43731f03a88b75eb6b87937d4dbaa663fd7161cd77a0d1",
  "🗑️generated\\capsule-final-repaired512\\2048-material-culled.png": "bf3865501102f40c3b3dbddca37256a2480a8f70f1e19d4978ae8ae453a17436",
  "🗑️generated\\capsule-final-repaired512\\2048-material-full.png": "e8ed9166c80c21bb8d207da43e19b2d0bde82dbea333daa487654795e8287851",
  "🗑️generated\\capsule-final-repaired512\\2048-material-stable.png": "856fb3418aaaf0c177fb56009be43604c9fa47d8bf84516715daad480bebdc3e",
  "🗑️generated\\capsule-final-repaired512\\2048-silhouette-culled.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
  "🗑️generated\\capsule-final-repaired512\\2048-silhouette-full.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
  "🗑️generated\\capsule-final-repaired512\\2048-silhouette-stable.png": "9d91728a585e598c3727baaad77b626a03815b457adf60d71044170aa0a85d20",
  "🗑️generated\\capsule-final-repaired512\\512-aa-fill-culled.png": "f11db3d7ddef75c98dd4522541a8d33d4ea48a4f830804c7ee4edbc9761e39b6",
  "🗑️generated\\capsule-final-repaired512\\512-aa-fill-full.png": "52ed1a21cb76a0676e60f048d41a7e8772037c2755d66a767eef8c49a6bf9a83",
  "🗑️generated\\capsule-final-repaired512\\512-aa-fill-stable.png": "c8b11818901ca85daf68d7949ef070c80407fdeec236c50f3128e1cc7426cf50",
  "🗑️generated\\capsule-final-repaired512\\512-aa-strokes-culled.png": "bcb4302ed4a274497a7b6152dce52bb5f45b316086aa3fe53c97bf9d3eda6cdc",
  "🗑️generated\\capsule-final-repaired512\\512-aa-strokes-full.png": "2fb5b4d1ef9a4ea6e1ce6aa8c76af7375257bf9da65fb2f3d1ebfeb9a7efba56",
  "🗑️generated\\capsule-final-repaired512\\512-aa-strokes-stable.png": "3912ec29c56a0d606a9ff7bd15b5791f09f73994957dc37dab10e5ae7b5833a3",
  "🗑️generated\\capsule-final-repaired512\\512-geometric-culled.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
  "🗑️generated\\capsule-final-repaired512\\512-geometric-full.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
  "🗑️generated\\capsule-final-repaired512\\512-geometric-stable.png": "4aec9eaa84f8c976387473ec464b2b68c8c1c2f92445ba9480c2437051355418",
  "🗑️generated\\capsule-final-repaired512\\512-ids-culled.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
  "🗑️generated\\capsule-final-repaired512\\512-ids-full.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
  "🗑️generated\\capsule-final-repaired512\\512-ids-stable.png": "215d36d30a466519e427000929c7fff71e6ed5270c94ce4c8f1dd88bace1dfa3",
  "🗑️generated\\capsule-final-repaired512\\512-material-culled.png": "574d1f7b4777229427d92c1b09b1d1d1cc7fdd342ba99cc8454341a6c96e2928",
  "🗑️generated\\capsule-final-repaired512\\512-material-full.png": "3c063106db0fca2ca8d2017eefeeae3f0c5ff2447a352b9ffb246fdc396b0f3e",
  "🗑️generated\\capsule-final-repaired512\\512-material-stable.png": "9a20fb5e927537561a84269e1e18a68f6c6ffd58993493e5d712ab17b9abd748",
  "🗑️generated\\capsule-final-repaired512\\512-silhouette-culled.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
  "🗑️generated\\capsule-final-repaired512\\512-silhouette-full.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
  "🗑️generated\\capsule-final-repaired512\\512-silhouette-stable.png": "5477028b59fecbf621d228a028615dc9eee3dd0f39d82ead4fd6d0e66c4a6856",
  "🗑️generated\\capsule-final-repaired512\\coverage-aa-controls.json": "1f32b8d5f44b37f2d094fd9e340ccdba33c3221858a66a993ed2a970eca00c15",
  "🗑️generated\\capsule-final-repaired512\\coverage-camera.json": "fb6d460aa4428bf58bef8f54ac09205922491ee4f2c7e30014edae893c4bf65c",
  "🗑️generated\\capsule-final-repaired512\\coverage-items.json": "e55f56d589783a458f0710462b2da7f76376d76369966f771a0fb3795a6876a1",
  "🗑️generated\\capsule-final-repaired512\\coverage-paint-fragments.json": "a0903170152349afd9d884ae7ac98cb1059f52d8ba61eb6e01ae19f17587b4c3",
  "🗑️generated\\capsule-final-repaired512\\coverage-provenance.json": "4a98f9e2163b18ed81b1fc9acfa222bb7a1d2c5d8fb68aec898491ede1d3cc4a",
  "🗑️generated\\capsule-fragment-roi-512.json": "0fce6a8ed99939f7818018a9010f762f6bbbafef28db0155b97796cd8fb524ed"
}
```
