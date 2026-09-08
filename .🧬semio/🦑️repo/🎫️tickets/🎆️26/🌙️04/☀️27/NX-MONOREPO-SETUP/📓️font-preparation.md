# Font Preparation Producers

The Infinite Cargo package now has explicit `font-tool` and `fonts` targets. The first stages the native dump binary; the second consumes that completed binary, validates the packed extents and publishes only font bytes. Output ownership is disjoint; compilation remains in Cargo’s normal workspace store. Both new commands have editor entries.

The target-contract test failed before implementation and passed afterward. The actual producer completed and staged 17 fonts totaling 8,757,072 bytes (`font-assets-build.log`). A deletion/restoration probe with an independent Node/OpenType consumer is running. Existing development consumers still need to switch from the old existence-check/Cargo invocation to the prepared artifact.
