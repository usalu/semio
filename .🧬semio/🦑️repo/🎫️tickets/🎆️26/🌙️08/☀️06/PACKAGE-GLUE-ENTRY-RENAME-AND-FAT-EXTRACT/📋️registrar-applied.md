# Registrar applied after glue-rename agent

- Removed 37 stale OS implementation members deleted mid-flight by OS eradication.
- Added `🛢️db/📦️packages` member; retargeted db-* workspace deps to `semio-framework-os-kernel-db`.
- Retargeted flow/infinite/playbook workspace aliases → os-kernel (components exist; wiring owned by OS agent).
- Retargeted consumer path deps (wgpu, plugins) off deleted folded impl sandwiches → kernel/db packages.
- `cargo metadata --no-deps` green (72 packages) at final4.
