# Canonical Image Fixture Identities

Updated 53 JSON fixture files across transform-image, fill-region, apply-filter, paint-stroke, and fill-selection. The requested approximate census was 54; the actual obsolete-key census was 53 files with 20 distinct asset identities. All asset map keys, image references, child target artifact IDs, and child content IDs now agree with the canonical `image-<16hex>` formatter.

The historical suffixes were not safe to preserve. For example, `pixels-paint-95df513e979c7d93` carried raw decoded pixels whose current canonical SHA-256 prefix is `1ea5d2b9882458fd`; its historical `raster-asset-30b6da2098545c62` child ID was also stale. All 20 identities were recomputed rather than renamed mechanically.

The retained read-only `image-fixture-audit.ts` uses Node's independent SHA-256 implementation with the canonical Rust field framing: an eight-byte little-endian field length followed by field bytes, including source schema, dimensions, colorspace, bit depth, frame count, delays and pixels, ICC presence/content, and ordered metadata. It agrees with the independent existing Unicode/profile oracle `6a5b078c041617b1`. Eighteen decoded images are present directly in diff fixtures. The two source-only wall images are reconstructed from the authored Rust case tables: a 6×4 transparent RGBA image with column 2 painted either `[204,51,26,128]` or `[255,0,0,255]`. Their digests are `9f5dde706e415d92` and `e99486b5afaab5a1`.

The final read-only audit completed successfully through the repository script interface: 262 files inspected (261 JSON files and one `.absent` marker), 20 raw image identities and 20 child handles, zero obsolete identities, zero digest conflicts, and zero target mismatches. All JSON parsed. The exhaustive extension census found no checked-in text or pack twins anywhere under this mutation fixture tree, so no physical text/pack twin files required edits. Existing Rust fixture emitters and codec tests remain the authority for regenerating those representations; their execution remains blocked by the source admission failure reported by the parent. No Rust test, codec round trip, or mounted runtime pass is claimed.

No migration script or compatibility alias remains: the ticket script only reads and audits the final fixtures.

Changed fixture files:

- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/🫧️smooths/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/🫧️smooths/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/🫧️smooths/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/📏️resizes/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/📏️resizes/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/📏️resizes/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/↪️rotates-right/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/↪️rotates-right/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/↪️rotates-right/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/✂️crops/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/✂️crops/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/✂️crops/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/📍️outside/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/📍️outside/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/↩️rotates-left/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/↩️rotates-left/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🔄️transform-image/↩️rotates-left/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🪣️fill-region/✂️clips/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🪣️fill-region/✂️clips/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🪣️fill-region/🎭️masks/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🪣️fill-region/🎭️masks/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🪣️fill-region/🧱️walls/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🪣️fill-region/🧱️walls/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🪣️fill-region/🧱️walls/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🪣️fill-region/🪣️fills/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🪣️fill-region/🪣️fills/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/🌫️blurs/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/🌫️blurs/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/🌫️blurs/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/✂️clips/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/✂️clips/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/✂️clips/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/🔆️brightens/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/🔆️brightens/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/🔆️brightens/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/↔️flips/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/↔️flips/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/↔️flips/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/🌈️inverts/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/🌈️inverts/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🌈️apply-filter/🌈️inverts/📸️snapshot/⬅️before/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️paint-stroke/✂️clips/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️paint-stroke/✂️clips/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️paint-stroke/🎭️masks/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️paint-stroke/🎭️masks/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️paint-stroke/🖌️paints/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🖌️paint-stroke/🖌️paints/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🫗️fill-selection/🫗️fills/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🫗️fill-selection/🫗️fills/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🫗️fill-selection/✂️clips/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🫗️fill-selection/✂️clips/📸️snapshot/➡️after/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🫗️fill-selection/🎭️masks/🔺️diff/🔣️.json`
- `✏️s/🔌️plugins/🖨️raster/🗿️artifacts/🖨️raster/🏅️standards/🔖️1/🪆️subsets/✳️any/🧪️tests/🧫️fixtures/🧬️mutations/🫗️fill-selection/🎭️masks/📸️snapshot/➡️after/🔣️.json`

