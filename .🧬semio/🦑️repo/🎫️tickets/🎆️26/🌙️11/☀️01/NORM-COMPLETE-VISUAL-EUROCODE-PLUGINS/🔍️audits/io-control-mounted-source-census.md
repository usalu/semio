# IO control mounted source census

At review the manifest lib points to package-local 🦀️.rs, which mounts control/🦀️.rs. That source mounts failure and snapshot; failure mounts codec and its own cfg(test) tests, while snapshot mounts its cfg(test) tests. The codec currently has no separate test module declaration.

The existing source collector cannot enumerate the closure: control/🦀️.rs mounts 📦️octets/🦀️.rs, which is currently absent (the directory contains only tests). This may be an in-progress shared edit. No complete totals or Cargo result can be asserted until the source is present. No build was run.

## Rerun

The octets module now exists. The source collector instead refuses the current io/control/🦀️.rs with Unbalanced Rust source delimiter during shared extraction, so a complete census still cannot be asserted. Descriptor was notified.

## Current collector

The original collector now resolves the mounted IO control source under Linux cfg: production 8 files, 163,928 bytes, 1,007 active lines; test 25 files, 242,455 bytes, 1,320 active lines. This is a source census, not actual Cargo admission or compilation.
