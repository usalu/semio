# Drawing PDF Native Reference Tests

The ticket's `drawing-reference-fixture.py` generates a private Cargo test workspace under `🗑️generated/pdfref`. It imports the production Drawing exporter and the original three PDF tests by source path. It uses the production Drawing dispatcher, the general PDF codec and the independent Hayro 0.4.0 renderer. The original test exercises exact PDF bytes, decoded page geometry and text, raster differences and missing canvas size.

The fixture seeds its generated lock from the canonical Stdio Semio workspace lock. Genuine Cargo offline resolution then writes the smaller graph. Using an unseeded lock selected 58 package and checksum rows absent from the canonical lock, including 48 different selected package versions. The seeded fixture has 775 selected native packages versus 830 for the public exporter test. The fixture package is the sole additional selected package and the sole additional lock row by name, version and checksum. The 56 packages removed include artifact application testing and the unrelated SVG reference path. Cargo resolved the same fixture in a second `--offline --locked` query.

The original full exporter test compiled through the PDF model on November 25, then was retired at 4.3 GiB disk availability to make room for the primary release. The private fixture's initial registry-exclusive lock-generation request was also retired gracefully, releasing its leases. The subsequent fixture plan used the repo tool registry for Cargo with the pinned native compiler for version probing and completed with no compilation. Local reference externs from earlier runs were reclaimed. The current active Demonstrator and Process run does not select the Drawing exporter or general PDF codec.

The behavior tests are pending. The earlier full test emitted 477 Cargo compilation lines and wrote about 3.6 GiB to its prototype target. Linear scaling to the fixture's package count suggests about 5.9 GiB before a margin for later crates and linking. Schedule the fixture when approximately 8 GiB remains available after the priority release reservations. From the workspace root, set `fixture` to the ticket's `🗑️generated/pdfref/Cargo.toml`, then use:

```sh
SEMIO_OFFLOAD=never repo cargo --store=shared --configuration=prototype --parallelism=1 -- test \
  --offline --locked --manifest-path "$fixture" --lib
```

The generated graph, cancellation and earlier full test logs are in `🗑️generated` beside the fixture.

After the later actor-input wire and text-registry changes, actual pinned Cargo refused the earlier generated lock. Offline resolution repaired only the generated private lock and a second offline locked query passed. The selected native package set is now 776. It added actor-input wire and four existing local testing owners and dropped four TXT leaf packages. External versions and checksums are identical. The 8 GiB estimate and primary-lane priority still apply. See `🗑️generated/pdfref-current-{preflight,refresh,locked,package-delta,selected-delta}.log`.

This fixture work updated the ticket input `drawing-reference-fixture.py`. It reads the existing repository test at `✏️s/🔌️plugins/🗄️stdio/🗿️artifacts/🧿️semio/🏅️standards/🔖️v1/🪆️subsets/🖊️drawing/🚪️io/📤️export/🧵️serializers/🗿️artifacts/📖️pdf/🔖️1.7/✳️any/🧪️tests/🔬️unit/🦀️.rs` without editing it.
