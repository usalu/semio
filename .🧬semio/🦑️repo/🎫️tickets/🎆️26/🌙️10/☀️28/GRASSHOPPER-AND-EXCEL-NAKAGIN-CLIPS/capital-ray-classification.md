# Capital Surface Ray Classification

The immutable `capital-order-source` and `capital-after-dependency-order` exports have identical actual source geometry, opacity, retained flags and cameras. The independent NumPy ray oracle recovered their orthographic projection from source/pixel correspondences (maximum residual 1.819e-12 pixels). All source/camera/raster/oracle endpoint hashes remained identical throughout the query. No Cairo capture or production edit was performed.

The ROI is the union of every old and current visible `steel_truss` source-ID pixel and every old/current changed source-ID pixel: 938 pixels, including all 177 old changed pixels, with 25 rays per pixel (23,450 total). There are 2,602 exported source items, 957 retained, and 2,390 independently ear-clipped drawn triangles.

No sampled ray has steel as nearest material. All 938 old visible frame pixels were physically behind closed nearer material in this view; current Cairo has zero visible frame source IDs. This supports the closed-wall frame correction without making a universal claim about the actual south louvre openings.

## Remaining actual order discrepancies

The old point painter disagrees with physical nearest material at 18,140 samples; the current point painter at 106. Current Cairo pixel-centre ID differs at four ROI pixels: (322,220), (322,221), (351,226), (351,227). At the last two, retained west panel 816 paints over retained seam 1090 despite the seam being 22.871455 mm closer at the centre. This is a genuine physical depth difference, not an AA or zero-depth tie. The other two involve panel 894 and seam 1108. GH owns the ordering repair; no physical source change is required by this evidence.

All residual point-painter pairs and measured positive depth-deficit ranges (metres):

```json
{
  "816 -> 1090": {
    "samples": 20,
    "min": 0.009188501311040653,
    "max": 0.022871454792380064,
    "paint": 816,
    "nearest": [
      1090
    ]
  },
  "846 -> 1096": {
    "samples": 16,
    "min": 0.007350741138850481,
    "max": 0.021033694620182786,
    "paint": 846,
    "nearest": [
      1096
    ]
  },
  "870 -> 1102": {
    "samples": 20,
    "min": 0.0055129809666674134,
    "max": 0.01919593444800327,
    "paint": 870,
    "nearest": [
      1102
    ]
  },
  "894 -> 1108": {
    "samples": 20,
    "min": 0.003675220794473688,
    "max": 0.017358174275809546,
    "paint": 894,
    "nearest": [
      1108
    ]
  },
  "918 -> 1114": {
    "samples": 10,
    "min": 0.015520414103619373,
    "max": 0.015520414103630031,
    "paint": 918,
    "nearest": [
      1114
    ]
  },
  "1161 -> 1096": {
    "samples": 2,
    "min": 0.0016849031025643058,
    "max": 0.00852637984322513,
    "paint": 1161,
    "nearest": [
      1096
    ]
  },
  "1149 -> 882": {
    "samples": 2,
    "min": 0.001252541539844998,
    "max": 0.0018191070030049161,
    "paint": 1149,
    "nearest": [
      882
    ]
  },
  "1161 -> 846": {
    "samples": 1,
    "min": 0.0011756387043746486,
    "max": 0.0011756387043746486,
    "paint": 1161,
    "nearest": [
      846
    ]
  },
  "1120 -> 936": {
    "samples": 15,
    "min": 2.995498959990073e-7,
    "max": 2.9954990310443463e-7,
    "paint": 1120,
    "nearest": [
      936
    ]
  }
}
```

## Reproduction and limits

`query-capital-surfaces.py` consumes both immutable directories and the existing independent `pixel-nearest-surface.py` triangle/ray interface. Per sample it recomputes physical intersections and source point-painter order; it does not assign the centre pixel ID to every subpixel. Raw output is `🗑️generated/capital-ray-classification.json`, containing all samples, nearest sources, intervening opaque source IDs and exact endpoint maps. Finite ROI sampling does not establish exhaustive raster or AA correctness.

## Exact immutable input identities

The after-map is byte-identical to this before-map:

```json
{
  "🗑️generated\\capital-order-source\\512-ids-culled.png": "11a7655c5815cb1c3dbdbfc3c8c785efc4fe3aac883905f8af33759b82c8b7d5",
  "🗑️generated\\capital-order-source\\512-ids-full.png": "a0a41f45bd0d4e4ad0646604b8a3b216dd251d2ce54f445653e8ae4e793889f4",
  "🗑️generated\\capital-order-source\\512-ids-stable.png": "a0a41f45bd0d4e4ad0646604b8a3b216dd251d2ce54f445653e8ae4e793889f4",
  "🗑️generated\\capital-order-source\\coverage-aa-controls.json": "a12326948e9d60d0c1003b9dd0e51ebbc0d4475c370729bcb6c3ed0e108aa649",
  "🗑️generated\\capital-order-source\\coverage-camera.json": "7cd6d7701b5b146fab71e78da1c3f672f15f27f8236f68c60b31a8f3c184c434",
  "🗑️generated\\capital-order-source\\coverage-items.json": "f6f5042e0f0610a0dbd77d660c39e4b4ea29d85459a26b7f108f10bdd4d19982",
  "🗑️generated\\capital-order-source\\coverage-provenance.json": "74df96ab56d833eefb1f1ac6285df787d92ea7174a1e23b0e0a49c053638aacf",
  "🗑️generated\\capital-after-dependency-order\\512-ids-culled.png": "b469dc6f0125f0b4e84128e19c800bbd027f6f3e5acace3f6961798b3f6c42ef",
  "🗑️generated\\capital-after-dependency-order\\512-ids-full.png": "b469dc6f0125f0b4e84128e19c800bbd027f6f3e5acace3f6961798b3f6c42ef",
  "🗑️generated\\capital-after-dependency-order\\512-ids-stable.png": "b469dc6f0125f0b4e84128e19c800bbd027f6f3e5acace3f6961798b3f6c42ef",
  "🗑️generated\\capital-after-dependency-order\\coverage-aa-controls.json": "6459bc478e79f193e75552a2ce72f0c15fcf03ccde81294912e6179c208c2e42",
  "🗑️generated\\capital-after-dependency-order\\coverage-camera.json": "7cd6d7701b5b146fab71e78da1c3f672f15f27f8236f68c60b31a8f3c184c434",
  "🗑️generated\\capital-after-dependency-order\\coverage-items.json": "64d6818c8c4de3e568e94ddeb6240a35a0b412cb2e015ca9c2bdfde8b23a77a0",
  "🗑️generated\\capital-after-dependency-order\\coverage-provenance.json": "b5db4d463bd858307084662a0a7da9bac47adb0356a6e4c211616bdfbb3c8eb7",
  "query-capital-surfaces.py": "0727dbef685985ea9b100c65dd447de1b7216e4ef0883044843dd4795a5ebe39",
  "pixel-nearest-surface.py": "74be6398efa39b4b43afe9cf76ece2c39d6e750d85b7227a9a647de5fbf69da5",
  "C:\\git\\semio\\🎓️teaching\\🏛️architecture\\🏢️nakagin\\🧪️tests\\🚶️circulation\\🐍️.py": "832c2dc748ab2fae7d35903183fb5ba2e63244f45aec5bb65a92cce43da80883"
}
```
