# Fragment-Aware Independent Ray Oracle

## Current status

The current independent portable oracle has **11 tests passing in 2.514 s with exact endpoint maps unchanged**. Strict AJV accepts six paint, five warped-mesh and four convex-material scenarios and rejects eight malformed contracts. No production renderer, physical model or kernel was edited by this oracle work. Actual latest-source gates await new final exports: GH’s later actual-plane/singleton renderer save supersedes earlier renderer snapshots for final acceptance.

The completed earlier capital snapshot has a fully green independent query: 938 preserved pixels / 23,450 rays, zero nearest/painter/layer/unsupported-source errors and zero Cairo centre mismatches across all three stages. Its exact old producer provenance is preserved below; this is not relabelled as a latest-renderer capture. Capsule attempts exposed oracle preparation/diagnostic bugs, not completed passes; both are now protected by red/green portable tests. New actual queries are on hold until GH supplies current final exports.

## Earlier implementation and portable chronology

The physical nearest scene derives only from immutable original Item polygons, independently ear-clipped by the existing numerical geometry oracle. Explicit paint-fragment rows are a separate triangle scene. The point painter uses actual stage/rank/piece rows, unions compound pieces once per paint rank for alpha, and aliases source/polygon identities back to original physical sources. It never falls back to one scalar rank per original source.

Six schema-first portable cases protect source IDs repeated at different paint ranks, concave and disconnected material, compound transparent pieces, and a deliberately invented bridge across an original gap. The last case must disagree with physical nearest geometry, proving paint fragments cannot manufacture physical truth. Initial red execution failed with six missing-interface errors; the first six tests passed after implementation.

Current exact export contract: `coverage-paint-fragments.json` flat rows with `stage`, `rank`, `piece`, `source`, `polygon`, `world_vertices`; stage/rank/piece is unique. Original opacity and semantic source identity come from immutable `coverage-items.json`. Source IDs are not fragment IDs.

`run-fragment-surface-audit.py` queries each pixel centre and a 5×5 grid, includes every new source-ID raster difference and explicit earlier ROI, records all original/paint/camera/raster/oracle endpoint hashes, and diagnoses physical-nearest, unsupported-source and transparent-layer order differences separately. No production V/M/kernel edit or duplicate Cairo capture is part of this task. Actual producer output is pending.

## Current portable proof

Seven tests pass in 0.092 s with exact source endpoints unchanged. They include independent NumPy plane solutions and Pillow rank-union masks. Strict AJV accepts six portable fragment-paint scenarios and rejects four malformed exports. Compound pieces sharing a rank contribute alpha once, even on their common boundary; transparent layers at distinct actual ranks remain distinct. Duplicate `(stage,rank,piece)` identities and unknown source/polygon references reject.

Actual producer queries remain pending. Explicit preserved ROIs are prepared: capital938 pixels, capsule256319 prior pixels including (174,182), and capsule5123 prior changed pixels. Every fresh source-ID difference will be added automatically, so these are starting ROIs rather than fixed limits.

```json
{
  "tests": 7,
  "success": true,
  "failures": 0,
  "errors": 0,
  "before": {
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-nearest-surface.py": "808a9be4d6faef3805884beb21d99c34acfb676d9688ffc415f973e4c71af88a",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-ray-cases.json": "c4a11dd80dc539e96fe3ca33bb99594a6c49e0b16abfeb69a6e1b59c1438a8af",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-paint-fragments.schema.json": "0403971dfb2bf6abdcfcd8960777978f38fc0f43c7b2b33d2f992ba463512761",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-pixel-nearest-surface.py": "d004dccfa0dcc35ed94db8e65dc2e4a18c4c5e11a9ac5b6c84ca12adaee44969",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-paint-export.cjs": "dab4e0aa7de9500f1f4bf2afb21fe61a820ed0e0963c6a78b072218e96b8fa97",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\run-fragment-surface-audit.py": "83ce2151ae4291ad75e2620e364034bd12f84c2db67d3fea956120de1eff31b8",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-fragment-ray-oracle.py": "27fc761881088a91155ff39ea9ad476fdfdd50a7b28ca66e29d58ca3638c86cd",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883"
  },
  "after": {
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-nearest-surface.py": "808a9be4d6faef3805884beb21d99c34acfb676d9688ffc415f973e4c71af88a",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-ray-cases.json": "c4a11dd80dc539e96fe3ca33bb99594a6c49e0b16abfeb69a6e1b59c1438a8af",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-paint-fragments.schema.json": "0403971dfb2bf6abdcfcd8960777978f38fc0f43c7b2b33d2f992ba463512761",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-pixel-nearest-surface.py": "d004dccfa0dcc35ed94db8e65dc2e4a18c4c5e11a9ac5b6c84ca12adaee44969",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-paint-export.cjs": "dab4e0aa7de9500f1f4bf2afb21fe61a820ed0e0963c6a78b072218e96b8fa97",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\run-fragment-surface-audit.py": "83ce2151ae4291ad75e2620e364034bd12f84c2db67d3fea956120de1eff31b8",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-fragment-ray-oracle.py": "27fc761881088a91155ff39ea9ad476fdfdd50a7b28ca66e29d58ca3638c86cd",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883"
  },
  "unchanged": true
}
```

## Explicit stage and end-to-end controls

The sixth portable scenario supplies deliberately different full/stable/culled fragment streams for the same two original sources. The oracle must preserve the bad historical full/stable order and report the good fragmented candidate order, never falling back to a scalar Item rank. Current durable suite is 7 tests in 0.101 s with identical endpoint maps.

`probe-fragment-driver.py` creates a finite analytic crossing and Pillow ID inputs, then invokes the actual generic audit driver. It performs no Cairo capture. The seven-pixel/175-ray query identifies 150 wrong full/stable samples and zero wrong candidate samples; all six bad historical ID pixel centres are identified, while candidate centre and paint/source-ID agreement is exact. Input endpoints stay unchanged. This is a driver regression, not acceptance evidence for actual capital or capsule exports.

```json
{
  "tests": 7,
  "success": true,
  "failures": 0,
  "errors": 0,
  "before": {
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-nearest-surface.py": "808a9be4d6faef3805884beb21d99c34acfb676d9688ffc415f973e4c71af88a",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-ray-cases.json": "949f394bd026dc3eed9199a0cc3b541927ea7406b27b81df9b79973ef2c15358",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-paint-fragments.schema.json": "0403971dfb2bf6abdcfcd8960777978f38fc0f43c7b2b33d2f992ba463512761",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-pixel-nearest-surface.py": "3f3a5416cfadb5b951e6686a4183afa8545110ba18bd8162caae6c8e2ebb8835",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-paint-export.cjs": "44e55e603b7b15d750e92a8b068e326575cd0d6308d1026d42c262343264c983",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\run-fragment-surface-audit.py": "83ce2151ae4291ad75e2620e364034bd12f84c2db67d3fea956120de1eff31b8",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-fragment-ray-oracle.py": "27fc761881088a91155ff39ea9ad476fdfdd50a7b28ca66e29d58ca3638c86cd",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883"
  },
  "after": {
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-nearest-surface.py": "808a9be4d6faef3805884beb21d99c34acfb676d9688ffc415f973e4c71af88a",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-ray-cases.json": "949f394bd026dc3eed9199a0cc3b541927ea7406b27b81df9b79973ef2c15358",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-paint-fragments.schema.json": "0403971dfb2bf6abdcfcd8960777978f38fc0f43c7b2b33d2f992ba463512761",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-pixel-nearest-surface.py": "3f3a5416cfadb5b951e6686a4183afa8545110ba18bd8162caae6c8e2ebb8835",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-paint-export.cjs": "44e55e603b7b15d750e92a8b068e326575cd0d6308d1026d42c262343264c983",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\run-fragment-surface-audit.py": "83ce2151ae4291ad75e2620e364034bd12f84c2db67d3fea956120de1eff31b8",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-fragment-ray-oracle.py": "27fc761881088a91155ff39ea9ad476fdfdd50a7b28ca66e29d58ca3638c86cd",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883"
  },
  "unchanged": true
}
```

Analytic probe summary:

```json
{
  "pixels": 7,
  "rays": 175,
  "changed_id_pixels": {
    "full-stable": 0,
    "stable-culled": 6,
    "full-culled": 6
  },
  "summary": {
    "full_wrong_nearest_samples": 150,
    "full_unsupported_samples": 0,
    "full_layer_inversion_samples": 0,
    "stable_wrong_nearest_samples": 150,
    "stable_unsupported_samples": 0,
    "stable_layer_inversion_samples": 0,
    "culled_wrong_nearest_samples": 0,
    "culled_unsupported_samples": 0,
    "culled_layer_inversion_samples": 0,
    "physical_visible_steel_samples": 0,
    "physical_culling_nearest_changed_samples": 0,
    "full_cairo_physical_mismatch_centres": 6,
    "full_cairo_paint_mismatch_centres": 0,
    "stable_cairo_physical_mismatch_centres": 6,
    "stable_cairo_paint_mismatch_centres": 0,
    "culled_cairo_physical_mismatch_centres": 0,
    "culled_cairo_paint_mismatch_centres": 0
  },
  "source_before": {
    "🗑️generated\\fragment-driver-probe\\4-ids-culled.png": "c026c622a27440564382561893051f9a17249c9ca80b7fbc0447840ccb43990f",
    "🗑️generated\\fragment-driver-probe\\4-ids-full.png": "96f0e7974eed4e245616b88ddda425eff2cf9ce8c76f7ba88d8b8cf68a01b027",
    "🗑️generated\\fragment-driver-probe\\4-ids-stable.png": "96f0e7974eed4e245616b88ddda425eff2cf9ce8c76f7ba88d8b8cf68a01b027",
    "🗑️generated\\fragment-driver-probe\\coverage-camera.json": "50cbd788608b4cc83a1e8f5dc103cc864c0061f3b2a5743da83e5f5f420f60ac",
    "🗑️generated\\fragment-driver-probe\\coverage-items.json": "8c66f7cbd2687beadea3cae122fa506ac78478670e8907be1c71a9c293499abf",
    "🗑️generated\\fragment-driver-probe\\coverage-paint-fragments.json": "21f8a16b7eeab7846d35dbf7b4fbea2eae3ae2a4226b047981dbf34aaa6a7e98",
    "🗑️generated\\fragment-driver-probe\\coverage-provenance.json": "8c60d1b7189a20d77cfc534164baee631979d4d11aa572da1237dfab28af81c5",
    "🗑️generated\\fragment-driver-probe\\roi.json": "ad12fec49b39935aaf33c85070a897b103b1adeb9fcd31f9329b99bde059245e",
    "run-fragment-surface-audit.py": "83ce2151ae4291ad75e2620e364034bd12f84c2db67d3fea956120de1eff31b8",
    "pixel-nearest-surface.py": "808a9be4d6faef3805884beb21d99c34acfb676d9688ffc415f973e4c71af88a",
    "pixel-ray-cases.json": "949f394bd026dc3eed9199a0cc3b541927ea7406b27b81df9b79973ef2c15358",
    "pixel-paint-fragments.schema.json": "0403971dfb2bf6abdcfcd8960777978f38fc0f43c7b2b33d2f992ba463512761",
    "test-pixel-nearest-surface.py": "3f3a5416cfadb5b951e6686a4183afa8545110ba18bd8162caae6c8e2ebb8835",
    "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883"
  },
  "source_after": {
    "🗑️generated\\fragment-driver-probe\\4-ids-culled.png": "c026c622a27440564382561893051f9a17249c9ca80b7fbc0447840ccb43990f",
    "🗑️generated\\fragment-driver-probe\\4-ids-full.png": "96f0e7974eed4e245616b88ddda425eff2cf9ce8c76f7ba88d8b8cf68a01b027",
    "🗑️generated\\fragment-driver-probe\\4-ids-stable.png": "96f0e7974eed4e245616b88ddda425eff2cf9ce8c76f7ba88d8b8cf68a01b027",
    "🗑️generated\\fragment-driver-probe\\coverage-camera.json": "50cbd788608b4cc83a1e8f5dc103cc864c0061f3b2a5743da83e5f5f420f60ac",
    "🗑️generated\\fragment-driver-probe\\coverage-items.json": "8c66f7cbd2687beadea3cae122fa506ac78478670e8907be1c71a9c293499abf",
    "🗑️generated\\fragment-driver-probe\\coverage-paint-fragments.json": "21f8a16b7eeab7846d35dbf7b4fbea2eae3ae2a4226b047981dbf34aaa6a7e98",
    "🗑️generated\\fragment-driver-probe\\coverage-provenance.json": "8c60d1b7189a20d77cfc534164baee631979d4d11aa572da1237dfab28af81c5",
    "🗑️generated\\fragment-driver-probe\\roi.json": "ad12fec49b39935aaf33c85070a897b103b1adeb9fcd31f9329b99bde059245e",
    "run-fragment-surface-audit.py": "83ce2151ae4291ad75e2620e364034bd12f84c2db67d3fea956120de1eff31b8",
    "pixel-nearest-surface.py": "808a9be4d6faef3805884beb21d99c34acfb676d9688ffc415f973e4c71af88a",
    "pixel-ray-cases.json": "949f394bd026dc3eed9199a0cc3b541927ea7406b27b81df9b79973ef2c15358",
    "pixel-paint-fragments.schema.json": "0403971dfb2bf6abdcfcd8960777978f38fc0f43c7b2b33d2f992ba463512761",
    "test-pixel-nearest-surface.py": "3f3a5416cfadb5b951e6686a4183afa8545110ba18bd8162caae6c8e2ebb8835",
    "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883"
  },
  "unchanged": true
}
```

## Completed immutable capital snapshot

Query57464 exited0 in34.6605s. It retained all15,012 actual paint rows, reusing5,548 exact identical geometric polygons across stages as10,904 ray triangles. There is no sample/face/material budget. The raw complete result is `🗑️generated/capital-fragment-ray-durable.json`; its append-only journal hash is b574bc0a6c2ddfb8cfe5d4bd2ea1cefa43841c95ac6408c38131cf15528b3e14. Every immutable original/paint/camera/raster/oracle input endpoint matched. The preceding66193 attempt ended unexpectedly after200/938 without traceback or result and remains incomplete/non-pass; no timeout/OOM cause is asserted. The tested exact-geometry reuse and durable per-pixel journal were added before the successful distinct run.

```json
{
  "pixels": 938,
  "rays": 23450,
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
  "changed_id_pixels": {
    "full-stable": 0,
    "stable-culled": 0,
    "full-culled": 0
  },
  "source_before": {
    "🗑️generated\\capital-fragment-final\\2048-material-culled.png": "b012b0159006ef0fe998ef4f114a5244d76ef1b1e850d85370442d1cb79bece9",
    "🗑️generated\\capital-fragment-final\\2048-material-full.png": "65a1bdde7e93e324dbb449a11cd4c2069dab6f70d57be4f2b508fb447074f530",
    "🗑️generated\\capital-fragment-final\\2048-material-stable.png": "b012b0159006ef0fe998ef4f114a5244d76ef1b1e850d85370442d1cb79bece9",
    "🗑️generated\\capital-fragment-final\\512-ids-culled.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
    "🗑️generated\\capital-fragment-final\\512-ids-full.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
    "🗑️generated\\capital-fragment-final\\512-ids-stable.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
    "🗑️generated\\capital-fragment-final\\512-material-culled.png": "adf6a88df7469df066222c269597674619fee1b2a85d02c076507d6466fcaddf",
    "🗑️generated\\capital-fragment-final\\512-material-full.png": "bfc7777ee7ed32696899342d0332963b0b71d68d0d639698d0557ef5f4168d0c",
    "🗑️generated\\capital-fragment-final\\512-material-stable.png": "e1bfea34303f200ba6c7643b74b7bb614f3289342d1f3a0e1f1eca154f309bb3",
    "🗑️generated\\capital-fragment-final\\coverage-aa-controls.json": "221ab695b8d7020344268643f79b4036e7c07235752a1f743600f43bc1f91084",
    "🗑️generated\\capital-fragment-final\\coverage-camera.json": "7cd6d7701b5b146fab71e78da1c3f672f15f27f8236f68c60b31a8f3c184c434",
    "🗑️generated\\capital-fragment-final\\coverage-items.json": "6796316d3733f814a49d842027e259a9097c93637c9ca5440add4d93db1cd8cf",
    "🗑️generated\\capital-fragment-final\\coverage-paint-fragments.json": "05e946ffc4c5e673326d6eadc6916c54235092f1b1aa214b3f2b05685ed45f76",
    "🗑️generated\\capital-fragment-final\\coverage-provenance.json": "61b93a8c11d2ffa3d6b915228d20bc852308c0ed4dd28dbd18a08e6247aa7d57",
    "run-fragment-surface-audit.py": "d82109e73edcd6abb43f278aa5f2ee3e05e4e21ba0e9f6ecc29eeca81ba39ca5",
    "pixel-nearest-surface.py": "183c89c0c50d4efb22019bc7e2e408ab36559828f22be05abdd562740fd32756",
    "pixel-ray-cases.json": "945165aee7d88faa036df86b65af83908fa46f0c16f37d3584d3e976eb77644c",
    "pixel-paint-fragments.schema.json": "0403971dfb2bf6abdcfcd8960777978f38fc0f43c7b2b33d2f992ba463512761",
    "test-pixel-nearest-surface.py": "df8c8223997544ba9d97e9600c24b3ed08e69dd96b8cf49b89dc98707cca2e32",
    "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
    "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
    "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
    "🗑️generated\\capital-fragment-roi.json": "08e7796732e6beaaf0596f3b887055f53f26c516cbf7f4e7b3762049d2dd503f"
  },
  "source_after": {
    "🗑️generated\\capital-fragment-final\\2048-material-culled.png": "b012b0159006ef0fe998ef4f114a5244d76ef1b1e850d85370442d1cb79bece9",
    "🗑️generated\\capital-fragment-final\\2048-material-full.png": "65a1bdde7e93e324dbb449a11cd4c2069dab6f70d57be4f2b508fb447074f530",
    "🗑️generated\\capital-fragment-final\\2048-material-stable.png": "b012b0159006ef0fe998ef4f114a5244d76ef1b1e850d85370442d1cb79bece9",
    "🗑️generated\\capital-fragment-final\\512-ids-culled.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
    "🗑️generated\\capital-fragment-final\\512-ids-full.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
    "🗑️generated\\capital-fragment-final\\512-ids-stable.png": "bd45078e4f1def6c72762f784f168a5097013770ba89c17b954bf3739c036867",
    "🗑️generated\\capital-fragment-final\\512-material-culled.png": "adf6a88df7469df066222c269597674619fee1b2a85d02c076507d6466fcaddf",
    "🗑️generated\\capital-fragment-final\\512-material-full.png": "bfc7777ee7ed32696899342d0332963b0b71d68d0d639698d0557ef5f4168d0c",
    "🗑️generated\\capital-fragment-final\\512-material-stable.png": "e1bfea34303f200ba6c7643b74b7bb614f3289342d1f3a0e1f1eca154f309bb3",
    "🗑️generated\\capital-fragment-final\\coverage-aa-controls.json": "221ab695b8d7020344268643f79b4036e7c07235752a1f743600f43bc1f91084",
    "🗑️generated\\capital-fragment-final\\coverage-camera.json": "7cd6d7701b5b146fab71e78da1c3f672f15f27f8236f68c60b31a8f3c184c434",
    "🗑️generated\\capital-fragment-final\\coverage-items.json": "6796316d3733f814a49d842027e259a9097c93637c9ca5440add4d93db1cd8cf",
    "🗑️generated\\capital-fragment-final\\coverage-paint-fragments.json": "05e946ffc4c5e673326d6eadc6916c54235092f1b1aa214b3f2b05685ed45f76",
    "🗑️generated\\capital-fragment-final\\coverage-provenance.json": "61b93a8c11d2ffa3d6b915228d20bc852308c0ed4dd28dbd18a08e6247aa7d57",
    "run-fragment-surface-audit.py": "d82109e73edcd6abb43f278aa5f2ee3e05e4e21ba0e9f6ecc29eeca81ba39ca5",
    "pixel-nearest-surface.py": "183c89c0c50d4efb22019bc7e2e408ab36559828f22be05abdd562740fd32756",
    "pixel-ray-cases.json": "945165aee7d88faa036df86b65af83908fa46f0c16f37d3584d3e976eb77644c",
    "pixel-paint-fragments.schema.json": "0403971dfb2bf6abdcfcd8960777978f38fc0f43c7b2b33d2f992ba463512761",
    "test-pixel-nearest-surface.py": "df8c8223997544ba9d97e9600c24b3ed08e69dd96b8cf49b89dc98707cca2e32",
    "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
    "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
    "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf",
    "🗑️generated\\capital-fragment-roi.json": "08e7796732e6beaaf0596f3b887055f53f26c516cbf7f4e7b3762049d2dd503f"
  },
  "producer_provenance": {
    "start_sources": {
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "065f2f4dd36ae06f263dde288349552404d4066435b2a1f0c7868008eac9dd28",
      "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "30427be32e606ff0e3ca9698273b4b719ea3458067fcf9bd8696ac0922f06411",
      "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "d7ed310a56fc7ba136d7730bfad2f841ac80a7090ad08cdadc567f8b0051c821",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "33bfac4445262b13112c9515313e62a2bd0aef8d0557de320ef0aa069e41b344"
    },
    "end_sources": {
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🎨️visuals\\🐍️.py": "065f2f4dd36ae06f263dde288349552404d4066435b2a1f0c7868008eac9dd28",
      "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🐍️.py": "30427be32e606ff0e3ca9698273b4b719ea3458067fcf9bd8696ac0922f06411",
      "🎓️teaching\\🏛️architecture\\🏗️bim\\🎬️clip\\🎨️visuals\\📐️drafting\\✂️fragments\\🔣️.schema.json": "d7ed310a56fc7ba136d7730bfad2f841ac80a7090ad08cdadc567f8b0051c821",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
      "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🔣️.json": "0835be979ddecc5133e62107ea3976c3f4714a61bbeb44aa3b1ba9a6707b3d68",
      ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\audit-opaque-coverage.py": "33bfac4445262b13112c9515313e62a2bd0aef8d0557de320ef0aa069e41b344"
    },
    "unchanged": true
  },
  "unchanged": true
}
```

## Capsule oracle corrections before current exports

1. Declared genuinely warped original quad semantics are shortest actual3D diagonal, tie0–2. Five portable cases (including actual entry handle) agree with NumPy/Rhino. The physical nearest scene uses these original-source facets; it does not infer a plane from a warped quad or take physical geometry from paint fragments.
2. Actual paint row full2646/piece0/source1071/polygon0 has five vertices, world area5.49770931734e−8m² and planarity residual2.90e−16m (64ULP bound2.84e−14m). It is a simple convex fragment, independently confirmed by NumPy edge turns and SciPy hull with all five vertices. The original ear clipper’s fixed1e−12 predicate rejected its near-adjacent last vertices. The repair verifies every normalized edge half-plane before a direct fan, preserves all positive material, and leaves concave boundaries to the proper independent ear clipper. No geometric predicate tolerance was widened. The separate positive tiny-triangle red test also removed the oracle’s fixed absolute ray determinant cutoff.
3. The next attempt stopped after25pixels when a source-boundary diagnostic assumed `full_rank` existed on an original physical Item. Original physical rows now correctly carry no scalar paint order. A portable red reproduced the KeyError; the canonical boundary diagnostic and physical schema now use only source geometry/identity. New fragment fixtures remove those fields and still pass all explicit-stage tests; no fallback rank is used. That partial attempt is not accepted.

Exact diagnosis artifacts: `🗑️generated/paint-polygon-diagnostic.json`, `🗑️generated/rejected-fragment-geometry.json`. No actual query was restarted after GH requested the latest-export hold.

## Current portable proof endpoints

```json
{
  "tests": 11,
  "success": true,
  "failures": 0,
  "errors": 0,
  "before": {
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-nearest-surface.py": "06d0daa1d40c3d14f84f22528248e680b38191d01374459c487b5bbf6393945b",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-ray-cases.json": "91debd590b41eb21a744e20a8c0a9bd22740c57c5809350d6ef5e388f0db4b82",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-paint-fragments.schema.json": "0403971dfb2bf6abdcfcd8960777978f38fc0f43c7b2b33d2f992ba463512761",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-source-export.schema.json": "11d2f200e1f98b5f4f2a722032045cd0a0281b86c80fdde8aabbb6ad77978ced",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-warped-mesh-cases.json": "97948da7260442b9c85a76518fbab34febefd1a04761a037a6256b4021b60adb",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-warped-mesh.schema.json": "fec275b03a13a952eaa99a297c773f87f9ecb483594e75ad8b3d2b54107fa3a2",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-pixel-nearest-surface.py": "97c5d832409dab658fc899a39acbcdb85d3364721d213ad8ee2ca503d28bd82e",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-paint-export.cjs": "af74281d7f4e5e882eb57800b7e1ae40f2d4874c109217a955ae17891ac83ea8",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\run-fragment-surface-audit.py": "8886b93473d4cd8bfa029f223c3fb3b86c349cdc85f296130382b8b98de63650",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-fragment-ray-oracle.py": "e5ade670e914503bd36c8810e34d0cd9a0598e83d97ac5e32496860b262b5c47",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf"
  },
  "after": {
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-nearest-surface.py": "06d0daa1d40c3d14f84f22528248e680b38191d01374459c487b5bbf6393945b",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-ray-cases.json": "91debd590b41eb21a744e20a8c0a9bd22740c57c5809350d6ef5e388f0db4b82",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-paint-fragments.schema.json": "0403971dfb2bf6abdcfcd8960777978f38fc0f43c7b2b33d2f992ba463512761",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-source-export.schema.json": "11d2f200e1f98b5f4f2a722032045cd0a0281b86c80fdde8aabbb6ad77978ced",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-warped-mesh-cases.json": "97948da7260442b9c85a76518fbab34febefd1a04761a037a6256b4021b60adb",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-warped-mesh.schema.json": "fec275b03a13a952eaa99a297c773f87f9ecb483594e75ad8b3d2b54107fa3a2",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-convex-fan-cases.json": "9356e3e9861170d5ce518a855bcee74dc41442843b84a52df650b34f917fd3d0",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\pixel-convex-fan.schema.json": "ed30d1701157a6dfcd53863874b16fbb8fc6f12b880d17eec4db9031877391ab",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-pixel-nearest-surface.py": "97c5d832409dab658fc899a39acbcdb85d3364721d213ad8ee2ca503d28bd82e",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\test-paint-export.cjs": "af74281d7f4e5e882eb57800b7e1ae40f2d4874c109217a955ae17891ac83ea8",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\run-fragment-surface-audit.py": "8886b93473d4cd8bfa029f223c3fb3b86c349cdc85f296130382b8b98de63650",
    ".🧬semio\\🦑️repo\\🎫️tickets\\🎆️26\\🌙️10\\☀️28\\GRASSHOPPER-AND-EXCEL-NAKAGIN-CLIPS\\validate-fragment-ray-oracle.py": "e5ade670e914503bd36c8810e34d0cd9a0598e83d97ac5e32496860b262b5c47",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🐍️.py": "b30bd02c8a08345aee394cf1daaed358762e2dc5a5e3ad353bb937aba5b80b19",
    "🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️cases\\🚶️circulation\\🔣️.json": "79b9134fdd527aba9b95f7636135d5fa0817fc0537cff2dbc8f3389f7219d3cf"
  },
  "unchanged": true
}
```
