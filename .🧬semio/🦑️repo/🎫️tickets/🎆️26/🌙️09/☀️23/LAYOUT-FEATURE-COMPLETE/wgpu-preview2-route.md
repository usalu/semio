# WGPU Preview 2 Shim Route

On the original port 6179 listener, the Layout host module crawl received HTTP 404 for `/🔌️plugin-modules/🪞️vendor/🤝️bytecode-alliance/🪟️preview2-shim/cli.js`. The prototype browser products already contained the patched shim and its publication receipt. The WGPU server placed its font mount at `/🔌️plugin-modules/🪞️vendor` ahead of the general plugin module mount. The static directory middleware treats an absent file under a matching prefix as a terminal 404, so it looked for `cli.js` under the fonts publication.

The WGPU server now mounts the specific Preview 2 vendor tree from the configured plugin module root before the font mount, following the existing React host precedence. The neutral route fixture exercises first-match ordering for the exact failing path in both profiles. A Bun run of that fixture reproduced the font mount match before the fix and passed for both profiles afterward. The registered WGPU route suite passed all five cases with Vitest native module import enabled. The Vite runner otherwise failed before loading tests with `Captured inventory Source module load failed`.

The configured WGPU listener has file watching disabled, so a coordinated serve-only restart replaced PID 24734 with PID 36282 on port 6179. The exact percent-encoded URL that had returned 404 then returned HTTP 200 with `text/javascript` and 4,814 bytes. Its SHA-256 was `281118847862a8d4c4aeae1a5316cf1dfec634490033f9a18db2567f407bcde9`. The HTTP probe compared all 15 shim responses and the guestslim font response byte-for-byte with the published sources. A missing shim file returned 404. No renderer or guest compilation was invoked during the restart.

## Runtime Graph Proof Coordinates

The runtime graph verifier calls the mount constructor with the completed framework roots. That projection now carries the same workspace, build profile, and plugin module root that the live config uses. The direct calls passed for prototype and release and verified all three exact mount paths. The registered route suite passed all seven tests, including the two new completed-root cases.
