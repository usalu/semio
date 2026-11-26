# Browser font preparation and Cargo admission

Procedural run `w0c7udV` waited before its Cargo unit graph with three free jobserver tokens. Its own host `font-assets` child had no compiler descendant and was waiting for host compiler capacity while the FEM/GIS guest advanced. The driver intentionally waits before Cargo for jobs that are not assigned to a named output stage. The two release plans placed only `support` under browser publication, leaving two fonts-only jobs classified as pre-Cargo.

The font packer creates the vendor font seed. Its downstream `fonts` job publishes that seed, and `support` installs it with the browser vendor and shard files. No Rust source or Cargo manifest reads the generated `guestslim-typst-fonts.bin` directly. The root and Play manifests now explicitly list `font-assets` → `fonts` → `support` in browser publication dependencies. The existing job dependencies remain authoritative.

The portable release fixture failed on the original direct browser job inventory. The production TypeScript prerequisite planner and the independent Python graphlib implementation both passed after the edit and emitted identical JSON for both actual plans. The root plan has no pre-Cargo jobs in this release job set. Play retains its nine independent pre-Cargo jobs. Already active processes keep their originally captured plans and complete their existing jobs.
