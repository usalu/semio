# VCS Identity Source Boundary

The entity identity implementation now belongs to `semio-os-vcs-entity-identity`, at its existing taxonomy location. VCS reexports that component under its existing public module. Its implementations and language-neutral fixtures remain in place; the existing VCS companion owns the original identity and control laws, including physical allocation and retirement checks. No source policy limits were raised.

The new workspace member is `🧰️framework/🛍️products/💻️os/🔨️modules/🌿️vcs/🚪️io/💾️binary/🪪️entity-identity/📦️packages/🦀️rust`. Its source boundary is 65536 bytes, 8 files, and 2000 active lines. The VCS boundary remains 262144 bytes, 32 files, and 4000 active lines.

The `semio-os-vcs`, `semio-os-vcs-entity-identity`, and `semio-os-vcs-tests` package tests were started through the repository Cargo ingress. Their full output is captured in the ticket's generated directory while the run is active. A separate original artifact source admission run is coordinated with the artifact verification lane.

The bitmap original native gate (session 97887) admitted `semio-os-vcs` as source unit 373/530 and `semio-os-vcs-entity-identity` as source unit 374/530, then entered native compilation. This establishes source admission, not a test verdict.

The first selected Cargo run exited 101 while compiling the companion: nine existing test call sites used outdated retained-work and source-admission APIs. They were updated to account for work and use staged admission while retaining the fixture and allocation assertions. The second run encountered an unrelated IO type inference error, which the IO owner fixed. The third run passed all four VCS unit tests and 40 of 41 companion tests, including the entity identity laws; the remaining Python oracle test could not find its fixture under the companion manifest. Its fixture and oracle paths have been corrected. A concurrent workspace manifest edit also caused an extent refusal during retirement of the third run. The fourth run encountered eight compiler errors in the shared value-retirement ticket selection during a concurrent `OriginalRetirementFailure` migration, before the tests could run again.

The earlier reported compiler failure in history retirement was already addressed in the shared checkout by accounting for the returned work via `tally`.
