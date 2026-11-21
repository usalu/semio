# Controlled import of large PDFs with DCT images

The ticket-only overlay admits conventional indirect DeviceGray and DeviceRGB image XObjects with eight-bit samples and a single DCT filter. The byte graph treats JPEG as an identity transfer with one typed DCT stage. It preserves the encoded stream exactly and retains the supported ColorTransform parameter. Unknown, other image codecs and mixed DCT filter chains keep their existing raw reference custody.

The semantic image cursor reads the original stream through the borrowed object source. It incrementally hashes at most the current work grant's bytes and leaves the encoded JPEG exclusively in the native graph. The typed image and its provenance role receive separate reference strings. Their allocation and the terminal role transfer are charged against the existing grants. No JPEG pixel decoding or encoded-stream cloning occurs during semantic import. The completed graph remains the resource owner used by the existing exporter and canvas resolver.

PyMuPDF 1.26.7 opened and rendered both pages of an authored 44,486-byte PDF and independently extracted the exact native JPEG bodies. The RGB stream was 22,926 bytes and the grayscale stream was 20,387 bytes. Both declared 176 × 176 pixels and had the expected channel counts. The language-neutral output records byte lengths, native JPEG digests, stable resource identity digests and rendered RGB digests. Pillow 12.0.0 generates the two reference JPEGs.

The focused repository Rust probe composes the actual PDF byte graph cursor with the controlled document cursor over that input. The native byte graph completed in 154,302 one-work turns and retained 73,386 bytes. It produced both exact JPEG bodies with typed DCT filters. The semantic lift completed in 44,246 one-work turns and retained 12,956 bytes. Both image roles had the independently computed resource identity, source object number and page binding. The source rejected any attempted image clone. The probe uses the 32,768-byte copy ceiling, 128-byte minimum inline reserve and 1 MiB release ceiling. Graph cancellation at work 128, 4,096 and 22,000 released all backing. Semantic cancellation during the first encoded-stream hash left only the already admitted empty snapshot tables and metadata. Unknown and JPX controls retained original bytes and deferred at the resources boundary.

The staged image lowering and native writer emitted a PDF that the actual byte graph cursor reopened with identical JPEG bodies and typed DCT parameters. PyMuPDF reopened this emitted file independently without repair and matched both original JPEG digests, page pixel digests, dimensions and channel counts. The integration also ran the existing controlled sampled/Flate image admission and cancellation fixtures and the 48-page Type1 graph and semantic checks unchanged apart from applying the production empty-reservation grant in the test harness.

Snapshot Pack admission and storage already carry the artifact reference in the image and provenance roles, with the JPEG bytes and typed filter parameters in the separate native COS object rows. Save reuses equivalent retained objects or resolves the reference by its SHA-256 identity before image lowering. The patch uses the existing repository SHA-256 state and requires no edits to its owner, dependency manifests or lockfiles. Snapshot Pack round-trip execution and mounted browser verification still require governed runs after release.

The repository source closure census on macOS, Linux and Windows showed at most 31 inputs and 1,551 active lines for the image owner, and 32 inputs and 3,102 active lines for the inferences owner. The added native resource-identity regression lives in the existing image module test suite. This separately checks agreement between the bounded writer and the production compact filter serializer in a governed compile. The source-only rehearsal mounts the real Rust cursors and SHA implementation with narrow stand-ins for publication metadata and unrelated resource ports. The live canvas dispatches both cold and controlled Artifact references through the same NativePdfArtifactResources resolver. It borrows the snapshot graph to locate and hash the matching stream, then hands the first-party JPEG decoder a borrow of the resolver’s owned clone. No native or WASI branch, or page-owner edit, is needed. A governed native/browser run of the staged code and publication through the mounted job remain pending.

`controlled-dct-stage.py` materializes exact production and test overlays in `🗑️generated/dct-stage`. `controlled-dct-oracle.py` writes the language-neutral MuPDF receipt in `🗑️generated/dct-assets/expect.json`. `controlled-dct-graph.py` and `controlled-dct-integration.py` exercise the focused and integrated source-only paths. Live repository sources were left untouched.

## Mounted browser case prepared

The browser case uses the existing app shell and Open/Save file actions. It observes actual 176 × 176 PNG image draws on the Canvas2d host, checks that both contain nonuniform opaque pixels with distinct RGB and grayscale characteristics, and compares byte-identical canvas image sources after Save and a fresh mount followed by reopen. A separate MuPDF command checks the browser download against the authored JPEG digests, page rasters and metadata. The browser script transpiled successfully. It was left unrun to preserve the active source epoch.

With the controlled overlay released and the current shell mounted, run:

```sh
TICKET='.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️28/FEATURE-COMPLETE-PDF-ARTIFACT-EDITOR'
ASSETS="$TICKET/🗑️generated/dct-assets"
repo tools exec bun -- "$TICKET/controlled-dct-browser.ts" "$PDF_SHELL_URL" "$ASSETS"
repo tools exec uv -- run --no-project --python 3.12 --with pymupdf==1.26.7 --with pillow==12.0.0 "$TICKET/controlled-dct-oracle.py" verify-emitted "$ASSETS/browser-saved.pdf"
```

The canvas scene publishes its image-layer JSON through the existing paged `framework.scene.canvas2d.layers` carrier. The fixed-size surface document holds the spine and lane reference. Current React and native contracts reconstruct the layer payload before painting. This already covers scene payloads larger than the fixed document and needs no renderer wire change for the two-image fixture.
