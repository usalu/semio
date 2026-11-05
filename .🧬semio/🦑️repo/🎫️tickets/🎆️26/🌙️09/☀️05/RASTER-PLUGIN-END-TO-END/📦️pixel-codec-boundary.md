# Pixel Codec Source Boundary

The same pinned compiler cfg and production Rust source-closure reader measured the original pixels package at 264,341 bytes over nineteen files. Its unchanged 262,144-byte declaration failed before compilation. The read-only neutral closure check reproduced that failure before source edits; the affine file reported by the build was merely the next file when the cumulative allowance ran out.

Image codecs now compile in an internal first-party pixels-codecs crate that mounts the existing authored source files. The pixels facade retains editing, compositing, coverage, affine sampling and mask retirement. Shared extent validation/progress/error vocabulary and the image lease move to the codec/image contract so PNG encoding has no dependency back onto editing. The original public image and lease types are explicit reexports of their actual new owner. Source files were neither copied to the ticket nor duplicated, and no Source bound was increased.

The first post-split read-only closure check passes both production and testkit selections: facade 120,657 bytes/eight files, codecs 145,257 bytes/fourteen files. Active lines remain below 4,000. This is structural source evidence, separate from the supported compiler admission and native codec tests, which still need their own results. The codec feature forwards the actual testkit exports and the framework workspace names the new real package.

The check now also compares every closure's total against independent filesystem stats and verifies the declared three bounds with Ajv. The neutral fixture has a closed schema. Seven changed Rust source files parse successfully. Codec-only dependencies were removed from the facade; editing/compositing dependencies were not introduced into the codec leaf. Native compilation and the existing independent PNG/image oracle suite remain required.

The PNG decoder's extent check was also pointed directly at the shared image contract; it no longer reaches back into the editing module. Production search found no other codec dependency on editing, coverage, compositing or affine sampling. This is still subject to compiler verification.

The census uses the pinned Linux x86_64 compiler cfg observed for these actual runs and resolves the source reader from the repository working directory. It does not hardcode the workstation's absolute checkout path. Native compilation on other targets remains distinct evidence; product Cargo wiring and source paths use relative platform-neutral paths.

The live hub check ended at a separate missing Raster test dependency alias before compiling; that manifest alias was repaired and all twenty-six Raster manifests now pass the inherited-dependency audit. A separate supported native check selects just the actual pixels-codecs and pixels libraries in their real framework workspace, avoiding the still-changing application metadata interface. It is a single fresh request after the previous hub request terminated.

That isolated compiler request (40213) completed successfully with exit 0: both semio-framework-pixels-codecs and semio-framework-pixels library selections passed the prototype native check through repo cargo. Existing style warnings did not prevent compilation. Independent codec behavior tests and checks on other target platforms remain separate work.
