# Browser First-Genesis Folder Archive

The staged Vite middleware derives the folder from the configured `S_DATA_DIR` and the SHA-256 of the document id. It reserves a fresh directory, retains the exact actor, client instance, worker epoch, accepted source and Store frontier, and issues a short-lived capability. Page, publish and retire requests use that capability. A retired worker epoch cannot reopen the same authority within the lease lifetime. The route accepts same-origin development requests and derives the path from the configured local document root.

The developer serve owner stages a platform-native `semio-folder-journal` before entering the Vite phase. The helper calls the existing native Store writer for ordinary folder PUTs. For first genesis it retains one ordered 4 KiB page at a time through the staged native spool and validates the memberless format-1 archive inside the existing 4 MiB limit. The native side writes the unchanged kind-4 record and verifies its FNV64 checksum, extent and SHA-256 under the same file lock and descriptor. A second process attempting either ordinary or first-genesis publication while the lock is held is refused. Cancellation closes the private request and lets the native writer roll back any incomplete append before retiring the spool.

The focused HTTP law used the actual Browser host-port client, the staged Vite handler and the native stdio companion. It sent 2,130,150 canonical archive bytes, received the exact Store frontier, retired the capability and reopened identical bytes from disk. Tests also covered duplicate reserve and page receipts, a foreign path and origin, stale epochs, oversized pages, retirement and physical writer contention. Python independently decoded kind 4, verified FNV64 and SHA-256, and read the same archive through SQLite. The earlier direct Node spool law separately measured a 4,096-byte file-read and write maximum and forced cancellation after bytes reached the journal. That direct Node capture is a fixture. The staged Vite route uses the Rust writer.

The native executable in the focused law compiled the actual staged companion, receipt, first-genesis capture and physical Store methods. Its ordinary archive decoder used a fixture implementation. The native Store closure and HTTP provider passed aggregate patch apply-check. The TypeScript native backend, HTTP route and bound provider passed strict type checking. A full Cargo product build and a mounted Browser editor journey remain separate acceptance work. The route’s authority starts at the trusted local developer origin and accepts the binding and worker coordinates from the host port.

- [Combined Browser provider stage](browser-first-archive-stage.py)
- [Native companion](browser-folder-stdio.rs)
- [Native executable provisioning](browser-folder-stdio-provision.ts)
- [Browser host-port client](browser-first-archive-client.ts)
- [Native companion build and contention checks](browser-folder-stdio-check.py)
- [Browser client to native HTTP law](browser-first-archive-http-law.ts)
- [Independent on-disk oracle](browser-first-archive-oracle.py)
