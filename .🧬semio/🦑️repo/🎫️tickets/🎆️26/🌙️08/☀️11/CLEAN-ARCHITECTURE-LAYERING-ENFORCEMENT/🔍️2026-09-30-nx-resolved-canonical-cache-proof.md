# Nx Resolved Canonical Cache Proof

Executed one read-only project graph query through installed Nx 23.2.0's `createProjectGraphAsync({exitOnError:false})`, using the actual repository emoji-project-json and test-cases graph plugins. No build/test/task execution was requested. Daemon was disabled and workspace graph cache/output redirected under the existing ticket's generated folder.

Actual API result: 24 resolved canonical-architecture targets; cacheNotFalse=[]; independentPolicyMismatches=[]. The independent JSON interpretation read each declared target and applied nx.json targetDefaults when its own cache property was absent. It produced the same 24 owners and false values. This was an inheritance interpretation check, not execution of a separate schema validator.

Every following owner resolved to cache:false:

- @semio-tech/cad-cad-rs
- @semio-tech/flow-extension-brep-rust
- @semio-tech/framework-job-rs
- @semio-tech/framework-os
- @semio-tech/framework-os-dev
- @semio-tech/framework-os-kernel
- @semio-tech/framework-plugin
- @semio-tech/framework-renderer-wgpu
- @semio-tech/layout-js
- @semio-tech/mit-bestand-demonstrator
- @semio-tech/mit-bestand-praesentation-projektetage
- @semio-tech/playbook-extension-procedural-rust
- @semio-tech/procedural-js
- @semio-tech/puzzle-plugin
- @semio-tech/raster-js
- @semio-tech/repo-lib
- @semio-tech/s-composition-laws-rs
- @semio-tech/s-fixture-sweep-rs
- @semio-tech/s-flow-composition-rs
- @semio-tech/s-spatial-kernel-semio-session-rs
- @semio-tech/sequence-sequence-rs
- @semio-tech/ui-react
- os-hub
- semio-framework-3d

This includes all formerly seven omitted explicit cache settings: presentation, demonstrator, sequence, raster, framework-os-dev, layout and procedural. They now inherit false from `nx.json:9–11`. Root script line 8959 additionally passes skip-nx-cache to the discovered run-many aggregate. The previously reported child-cache gap is resolved in the actual graph snapshot.

Full machine evidence was saved at `🗑️generated/nx-canonical-resolved-targets.json`; graph cache is under `🗑️generated/nx-graph-cache-audit`. Those transient outputs are subject to final ticket cleanup. This persistent report retains all resolved names and the query result. Actual uncached aggregate behavior remains the parent's separately executed runtime evidence.
