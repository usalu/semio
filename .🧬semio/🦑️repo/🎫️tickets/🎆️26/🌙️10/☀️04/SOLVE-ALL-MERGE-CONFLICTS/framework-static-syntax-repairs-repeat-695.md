# Static Syntax Provenance

Source preservation audit; no build, compiler, typecheck, resolver, tests, or runtime. The incoming Pack document cursor schema was already malformed; restored its exact authored stash bytes and retained the issue as a pre-existing authored defect. Duplicate dependency definitions introduced by source merge were removed only where provenance confirms both source sides parse individually.

[
  {
    "path": "🌎️hub/🧩️compositions/🖨️raster/📦️packages/🦀️rust/Cargo.toml",
    "provenance": {
      "base": "parses",
      "head": "parses",
      "stash": "parses"
    }
  },
  {
    "path": "🧰️framework/🛍️products/💻️os/🔨️modules/💡️inference/📦️packages/🦀️rust/Cargo.toml",
    "provenance": {
      "base": "parses",
      "head": "parses",
      "stash": "parses"
    }
  },
  {
    "path": "🧰️framework/🔨️modules/🎒️pack/🌱️value/🛫️encode/🫳️borrowed/🎒️document/⏳️cursor/🧬️schema/🔣️.json",
    "provenance": {
      "base": "absent",
      "head": "absent",
      "stash": "Expecting ',' delimiter: line 2 column 1 (char 630)"
    }
  }
]

TypeScript parse audit found two merge-introduced syntax defects with B/H/S all parsing: combined independent alias-regex edits were reconciled as an introduction alias retaining the current Unicode flag, and the World3dHost framework import retained incoming InteractionTarget with a restored import separator and type marker.

## Lexical Delimiter Preservation

- 🧰️framework/🛍️products/💻️os/🔨️modules/🏪️store/🎲️space-members-macro/🦀️.rs: restored immutable current macro; unrelated concrete Store method alignment rejected.
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🎯️targets/🧊️wgpu/📐️surface-lane/🦀️.rs: retained full authored incoming funded close_step with current StepContext flow.
- 🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️wgpu-surface-lane-unit/🦀️.rs: removed merge-introduced unmatched call suffix.
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🧵️instance/📨️dispatch/📥️staging/⌨️typed-command/🦀️.rs: retained full incoming producer_request staging span while keeping current dispatch caller/failure wrappers outside span.
- 🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🪝️mounts/👥️wp3-owners/🧪️test-hosts/🧵️retained-command/🦀️.rs: removed current duplicate headers preceding incoming authored funded test-host continuation definitions.
