# Editing Options Review

Static review of the current required scene carrier, native codecs, React controls, and mounted test found no additional concrete defect in the requested checks. The previously reported coordinate limit disagreement is fixed: native serde, native value decoding, fixture edge, and React inputs now admit 16384.

- `🌱️value/🌳️intrinsic/🦀️.rs` defines `DslValue::Bool(bool)` at line 115; the new plane test uses an existing variant.
- `ValueError::new(InvalidValue, message)` accepts the same `impl Into<String>` pattern already used by the world3d model. New options and grid native decoders reject unknown and duplicate keys before decoding required fields.
- Both fixture imports resolve to the same existing scene JSON fixture: the mounted test uses ten parent segments; the direct editing unit test uses nine. Checked through `options-review.ts` using native path resolution and existence checks.
- Editing options initialize from the shared fixture. The React active-layer effects no longer overwrite width or height. The wrapper applies semantic commands before republishing controlled props; the new mounted test deliberately bypasses that wrapper to check that commands alone cannot mutate authoritative published values.
- The mounted test's numeric changes satisfy their input constraints, and its expected commands match fixture string transport for grid fields. It republishes gamma/options before Apply, then waits for the prior operation to release the job before resize and crop. Its crop rectangle fits the unchanged 16×16 document. The initial 512×512 grid is intentionally replaced by the fixture's 5×4 grid before those operations.

These are source and import-resolution checks only. No Rust compilation, TypeScript typecheck, or mounted tests were executed because the retained source-admission descriptor-3 publication failure remains unresolved.
