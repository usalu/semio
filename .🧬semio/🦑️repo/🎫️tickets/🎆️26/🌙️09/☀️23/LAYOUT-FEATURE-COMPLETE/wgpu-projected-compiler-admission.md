# Projected WGPU Compiler Admission

The WGPU dev entry selects the authored `projected-no-optimizer` compiler policy for its privately projected composition. The existing compiler capacity owner admits exactly one core for Vite startup and separately for each uncached module transform or explicit warmup request. Other Vite callers retain counted admission. A ten-core host's counted request prefers five cores and requires three.

The service resolves the owned inline Vite configuration before initializing environments. The reduced request requires a private projection, no implicit configuration loader or Vite watcher, and disabled dependency discovery, optimizer includes, eager pretransforms, and warmup in every resolved environment. The WGPU configuration already supplies those settings. Its owned activation receipt watcher remains intact. It sends a browser reload when a stage publishes and does not restart the Vite listener. The existing capture, cancellation, drain, and private input identity checks still govern the service.

## Focused verification

- The staged six-file change passed Git's read-only patch and whitespace check. TypeScript parsed the four TypeScript files. The admission fixture covers the exact and counted requests, capture prerequisites, and the behavior with a single available core.
- The real Vite resolver accepted the optimizer-free profile and refused changed watcher, config file, client and SSR discovery, explicit optimizer include, and warmup settings. The test uses resolved state after plugin execution and starts no server.
- In a private lease namespace, the actual admission owner occupied every core except one. Both cold-operation scopes obtained that last permit with one-core custody. The following operation acquired it again after the preceding scope ended.
- Applied-source validation passed four Bun tests across the new admission suite and existing Vite service-capacity suite, with 20 assertions.

## Strict listener observation

The previous strict listener, PID 94097, had passed capture and held the shared host-capacity gate for 4,305 seconds without a core. The Layout guest held cores 0–4, the Office guest held 5–7, and the XLSX test held 8. Port 6179 refused TCP. The owner retired that wait and restarted only the canonical 6179 listener.

The replacement, PID 96320, announced readiness on port 6179 before the ten-second progress message. Its first `/@vite/client` and `/@vite/env` requests returned HTTP 200 JavaScript in 246.5 and 26.6 milliseconds. Two distinct query transforms also returned 200. During a transform the actual repository lease registry sampled exactly core 9 and the shared gate for PID 96320, while other processes held cores 0–8. After the requests it sampled only fixed image holds for the listener. The captured samples contain no strict lease-order violations for that PID.

The exact lease snapshot and HTTP digest receipt are in `🗑️generated/wgpu-single-core-review-leases.json` and `🗑️generated/wgpu-single-core-live-http.json`. The server log is `🗑️generated/serve-layout-wgpu-single-core.log`. This run checked Vite readiness and cold transforms while the guest build was still in progress.
