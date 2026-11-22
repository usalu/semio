# Descriptor 19:13 Reuse Evidence

The existing release/Darwin product in `.cache/b/p/m/browser/release/tools/descriptor-emitter/darwin` contains the executable, artifact manifest and retained compilation inventory published at 19:13. Its successful receipt records a release Cargo build with opt-level 3, 16 codegen units, LTO disabled, and explicit build-override settings. The inventory names `.cache/b/r/m/u/l/q53457c` as build directory and `q53457c/t/e9cc5d5674728c39b40b17644bee3d28` as target directory. Both paths are absent now, so that exact lane cannot warm the new attempt.

A read-only SHA-256 comparison of 454 recorded current repository files selected by Store/PNG and root manifest names found four changed Store files:

- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🌱️fresh-snapshot-target-and-vcs-authority/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/📦️codec/🧬️owned-schema/🧮️decode-owner-bundle-and-bounded-array-authority/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🔁️replay/🧬️supersession/🦀️.rs`
- `🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🧾️document/📜️history/💧️hydration/📥️fold/🦀️.rs`

No PNG-named file appears among the inventory's current-file rows. The later profile constraints file is also not among those rows; the receipt's build flags alone do not prove its compatibility with today's configuration. This was a selected-file comparison, not a complete revalidation of the compilation or Cargo unit graph, and the shared tree remains live.

The existing Play pipeline unconditionally runs the emitter build job; the consumer's retained artifact reader validates current compiler inputs and the compilation witness. The 19:13 artifact's presence alone is not authorization or proof for bypassing that job, and the changed recorded inputs preclude treating its retained observation as a matching current snapshot. Existing cache paths may still provide individual matching units where available, but this audit does not establish a hit or time saving. No Cargo, describe, build or lease operation was started.
