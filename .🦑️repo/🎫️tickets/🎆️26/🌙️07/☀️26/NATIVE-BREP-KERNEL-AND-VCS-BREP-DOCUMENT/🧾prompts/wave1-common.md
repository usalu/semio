# Wave 1 common rules

Repo root: /Users/ueli/Documents/semio
Ticket folder name: NATIVE-BREP-KERNEL-AND-VCS-BREP-DOCUMENT under .🦑️repo/🎫️tickets/

## Hard rules
- Edit ONLY your lane owned file under ✏️s/🔨️modules/️3d/📐️brep/<your-dir>/🦀️component.rs (replace stub).
- Read-only: existing foundation modules (vec/mat/tolerance/predicates/poly/bezier/bspline/curve/curve_ops/surface/surface_ops/arena/history/topo/euler/validate/error/oracle/engine types).
- Do NOT edit 📦️glue.rs, Cargo.toml, Cargo.lock, launch.json, project.json, script.ts, ⚙️engine, 🧰️kernel — append to ticket 📥️integration-requests.md instead.
- Do NOT create new files outside the ticket folder except your owned component.rs.
- Temporary logs/scripts go ONLY in the ticket folder.
- Docstrings start with a unique emoji; no comments inside function bodies; use // #region sections.
- Tests live inline in the same component.rs (quick/long/exhaustive tiers where appropriate).
- Build/test env: export SDKROOT=/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk DEVELOPER_DIR=/Library/Developer/CommandLineTools because Xcode license blocks the default SDK.
- Verification: cargo test -p semio-s-3d --lib <your_module_tests> then full lib if feasible; write 🧪lane-<id>-test-quick-run-1.txt and 🧾lane-<id>-scope-note.txt in the ticket; append a row to 🚦️lane-status.md; set your module to FROZEN in 📐️module-contracts.md.
- Reference-permitted: ~/.cargo/git/checkouts/brepkit-760d3602f95e00d3/d470b7c for algorithms. Do NOT add brepkit deps. Handcraft native code using crate::brep::*.
- Never claim tests pass without running them.
