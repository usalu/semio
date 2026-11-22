# Cold playground dependency recovery

The previous React and WGPU launchers stopped without opening their listeners. Both owner PIDs and the browser watchers were absent. The dependency, Rust, and product image mount directories were empty, while their disk images still existed. The root node_modules symlink pointed into the detached dependency image. A fresh WGPU launch reached the root dev loader and failed to resolve the framework plugin build plan.

The root dependency gate now requires a reachable node_modules directory before accepting a workspace-owned or repository-owned invocation as already provisioned. Missing generations use the established repository dependency provisioning path. The existing lazy-load test now follows explicit named re-exports through the public package entries.

Both missing-generation fixtures reproduced the skipped provisioner, then passed with the fix. The command-registry suite passed 48 tests and 501 assertions, including a comparison with TypeScript's own directory check. Repository Nx setup restored the real dependency generation. The previous WGPU renderer publication survived the detach. The canonical WGPU launch resumed Cargo component preparation, and the browser watcher is active. React and live browser verification remain outstanding.
