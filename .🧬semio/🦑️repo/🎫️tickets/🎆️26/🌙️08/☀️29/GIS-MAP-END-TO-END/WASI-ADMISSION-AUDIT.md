# WASI Admission Audit

The current React launch log reports `semio-plugin-instance-app` at 263,166 bytes against a 262,144-byte source boundary. The admission implementation resolves source closures per workspace compilation unit with the actual compiler cfg and checks byte, file, and active-line limits. It stops at the first failure, so this log does not prove that later units pass. A file-size sum without the exact cfg and source traversal cannot establish later violations.

The ticket's generated WASM artifacts are `canvas2d-pkg/framework_surface_bg.wasm` and `canvas2d-optimized-pkg/framework_surface_bg.wasm`. These are surface test packages, not GIS component plugins, and do not provide a legitimate substitute for the requested plugin build.

The next reliable check is to rerun admission after the instance-app closure fix and observe the next result. Further artifact and graph inspection was curtailed because repository capacity admission was blocked by concurrent work.
