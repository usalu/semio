# Math Native Validation

The root agent already reviewed this tree's naming. No file, folder, API, or test was modified during this follow-up.

A native `cargo test -p semio-framework-math --lib` check through Nx failed on seven preexisting test compile errors: three assertions at sampling lines 8857, 8859, and 8860 compare the asynchronous `counts.count(...)` return directly to integers, generating six E0369/E0277 diagnostics; another test awaits a synchronous geometry `Rng` constructor. These are existing async-test/API inconsistencies, not naming or moved-source errors. They were not weakened or rewritten.

The check log is temporary evidence in `🗑️generated/metabolism-glb/math-tests.log`. The next assigned physical review is 2D and 3D, not a repeated math naming pass.
