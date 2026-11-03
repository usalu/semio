# Live Storybook Inspection

## Current Server

Read-only static UI/styling Storybook is running on `0.0.0.0:6010`, accessible locally at http://localhost:6010. PID: `215494`. It intentionally remains running for the user. It has 240 story variants and no development WebSockets, source watchers or file-writing endpoints. Live source editing requires restarting the development server instead.

Retained input: `serve-static-inspection.ts`. Runtime artifacts: `🗑️generated/s`, `🗑️generated/storybook-static-inspect.log`, and `🗑️generated/storybook-inspect.pid`. Static files are removed when this server stops normally. Remove remaining runtime logs/PID files after inspection; do not remove them while the user is using the server.

## Forwarding Investigation

The user reported their forwarding connection dropping with “lost connection to pod” when opening Storybook.

- The original loopback-only listener was reachable locally but not through the container network address. Rebinding to all IPv4 interfaces corrected that reachability difference. An intermediate restart selected 6011 while the old listener was closing; that process was stopped and strict 6010 selection subsequently verified.
- The user still experienced the forwarding failure. The development server remained alive, both root/index returned HTTP 200, pod uptime was uninterrupted, available memory was approximately 13 GB, and memory pressure averages were zero. There was no observed server crash or current memory exhaustion.
- Chromium opened the complete development manager through `10.0.2.2:6010`, rendered Button successfully and recorded no page/resource/WebSocket errors. Three development WebSockets were active; the largest observed incoming frame was 152,421 bytes. No auth tokens were retained in this report.
- Switched the same port to a genuine production static build to eliminate development WebSockets/module-request fanout and reduce server resource usage. This is an inspection workaround, not proof of the forwarding service's root cause.

## Static Validation

- Local and container-network index requests: HTTP 200.
- Root GET and index HEAD: HTTP 200.
- POST: HTTP 405; encoded directory traversal: HTTP 403.
- Development channel route: HTTP 404.
- Refreshed built index: 240 story variants, including BelowFloor. Follow-up visual repairs and complete dev/static validation are recorded in `visual-repair.md`.
- Chromium opened the full manager through the container-network address and waited for Button's finished render: no page/resource errors and **zero WebSockets**.
- Static server resident memory: approximately 93 MiB, compared with approximately 1.18 GiB for the previous development server.

The actual external forwarding client/control plane is not accessible from this session. Its “lost connection to pod” failure has not been reproduced or conclusively diagnosed here. Recreate the existing 6010 forward and refresh the browser to test the static endpoint. Keep the forwarded port private.
