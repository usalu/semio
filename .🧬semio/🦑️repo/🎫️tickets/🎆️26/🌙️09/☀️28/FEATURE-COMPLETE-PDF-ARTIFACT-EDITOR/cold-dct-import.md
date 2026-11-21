# Cold PDF image stream normalization

The source-only rehearsal reproduced the cold import failure using the same authored JFIF and Adobe RGB inputs as the MuPDF pixel regression. The bounded byte graph retains DCT as an opaque reference stream with the original Filter dictionary and empty typed filters. The cold clear-file reader passed that graph directly to semantic image lift, which interpreted the JPEG bytes as packed samples and rejected their geometry.

The cold import overlay reuses the reference stream normalizer on image dictionaries still carrying a raw filter. This limits work to pending image streams. Typed streams have already processed their filter and predictor. Non-image reference streams keep their raw dictionaries and payloads, and unsupported image filters continue to preserve raw custody. The separate xref overlay gives an existing typed pipeline precedence over a new raw decode while recursively normalizing the stream dictionary.

The repository Rust toolchain ran the before and after source-only probes against the real Resolver, cold clear-file reader, retained graph cursor, filter pipeline and content parser. The Adobe and mirrored JFIF image objects then matched the Resolver outputs in stream bytes, dictionary and typed DCT parameters. Plain and predicted Flate streams decoded once and retained their typed provenance through repeated normalization, including in a nested stream dictionary. The non-image reference stream and unknown image codec preserved their original bytes and filter dictionaries in both runs. The content parser stripped the inline ASCIIHex wrapper exactly once and admitted the original grayscale JPEG bytes, width, height, precision and Decode range.

The graph projection owner measured 16 source inputs and 3,483 active lines for library mode and 28 inputs and 3,954 active lines for test mode on macOS, Linux and Windows. The cold import is mounted by PDF mutation family 11, whose production closure contains 13 inputs. The cold patch adds no inputs to its existing 34-input test closure. The visible concurrent editor fixture overlay touches a separate file.

Rehearse with `before` and `after` from the repository root. Apply the exact guarded source replacements by passing `apply` after coordinated source release.

```sh
TICKET='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR'
BASE='✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/📖️pdf/🏅️standards/7️⃣1.7/🪆️subsets/🧱️base'
SEMIO_HOST_CAPACITY=off repo tools exec python -- "$TICKET/retained-stream-probe.py" after "$BASE"
```

The applied live-source probe passed all seven regression checks by mounting the current Resolver directly. The independently regenerated PyMuPDF 1.26.7 receipt matched the three authored fixtures byte for byte. For a repeat run on the applied source, pass `live` in place of `after`.

The staged changes are in `retained-stream-pipeline.patch` and `cold-dct-import.patch`. The Rust canvas regression still requires replay against the updated governed binary after release.
