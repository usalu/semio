# Image Previews in Evaluated Values

## Producer and presentation

Flow’s builtin image producer places the authored data URI in a dictionary with `schema: "image"` and a `dataUrl` string. The Flow evaluation bridge serializes the dictionary under the channel in `in` or `out`. The Values tree previously presented that object through its generic metadata and text rows.

The shared Values presenter now adds a semantic Image under a localized preview row when it receives that dictionary with an embedded image source. The original dictionary members remain separately expandable, including the raw data URI. Media-looking strings and untyped metadata continue through their text and field rows. The picture’s visible branch and its preview row use the original directional graph handle during hover. The root channel row keeps its existing graph selection binding.

The image has a stable node key derived from its windowed tree path. TreeWindows charges the image and carrier records to the body ledger. Mounted windows use the shared resource cursor when the source would exceed the tree’s ordinary carrier allowance. This allows the same tree to host short images and valid larger sources without holding every large source record at once. A preview with no admissible body budget exposes its localized status while the metadata remains expandable.

React lays out the image as full-width tree content and forwards the row’s hover handlers over the picture. The native retained tree gives the same Image record multiple content lines and uses the existing Image slot and pointer ancestry. The whole preview is one indexed row with a fixed visual child, and the fixed child’s carrier does not add a second tree window.

## Verification

- The [portable image fixture](../../../../../../../✏️s/🔌️plugins/🌀️procedural/🫀️foundation/🖼️semantic-ui/📊️channel-values/🧪️tests/🖼️images.json) contains the Flow object, an exact PNG, expected pixels, and negative schema cases. PNGJS independently decoded the expected pixels.
- The [compact Rust driver](channel-image-native.ts) compiled the actual Values presenter, retained tree image builder and image resource publisher against the cached Pack JSON crate. Its panel and builder stand-ins exercised the real projection law. It verified the exact input and output hover bindings, distinct image keys, preserved data members, ledger debits and all three exact pages of a valid 256×256 PNG. The small image used the inline source path.
- The [mounted Chromium driver](channel-image-browser.ts) used the real React Interpreter, retained store and TreeWindow scheduler. The fixture body publisher used the native-authored resource descriptors. The test expanded the output, decoded its original PNG at 256×256, compared the SHA-256 from the browser Blob, opened the data URL metadata row, hovered the displayed picture to receive the exact output handle, collapsed the branch and checked that the Blob URL had been revoked. The separate input rendered the short PNG and emitted its exact input handle when hovered.
- The original 25-page retained Image transport regression and the whitespace check passed. The full linked native WGPU host and browser-to-plugin actor refresh are outside these checks.

Run the compact driver before the Chromium driver from the checkout with `repo tools exec --light bun --`. The native driver prepares the browser resource descriptors under the ticket’s generated directory.
