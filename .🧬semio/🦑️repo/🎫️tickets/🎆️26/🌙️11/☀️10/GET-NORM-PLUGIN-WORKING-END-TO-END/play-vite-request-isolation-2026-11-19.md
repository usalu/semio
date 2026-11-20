# Play Vite Request Error Isolation

Play calls the in-process `serveVite` wrapper for both the interactive server and the end-to-end server. The wrapper now carries each Vite transform or warmup rejection as a settled result until optimizer work and the lease check finish and compiler capacity is released. Only then does it return the original rejection to Vite. A resource-control, capacity, or optimizer-drain failure still closes admission and interrupts the listener. A drain failure takes precedence when the request also failed.

The request queue remains capped at 32 pending operations. The regression fills the queue while optimizer processing is outstanding, checks that the next admission is refused, then verifies that the pending slots are released after the drain. It also checks that a request error leaves admission open and that a concurrent drain error interrupts the service.

## Verification

- The focused regression failed against the original implementation because a request error entered the service failure list.
- The repository-wrapped drain, artifact-input, environment, compiler-capacity, and acquisition-order suites passed 17 tests across five files.
- The existing isolated HTTP listener test now includes a fixture for a missing JavaScript module returning HTTP 404 followed by a valid module returning HTTP 200. Its native run reached private workspace preparation, which failed before binding a listener with `process.nativeBuild.image-host-free-insufficient`. macOS reported 27.2 GiB available and 36.1 GiB required. The HTTP fixture remains unverified on this host.
- The shared Play listener was neither restarted nor used as a test target.

## Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🏗️compiler/💧️drain/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/💾️storage/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/💾️storage/🧪️tests/🔣️.json`
