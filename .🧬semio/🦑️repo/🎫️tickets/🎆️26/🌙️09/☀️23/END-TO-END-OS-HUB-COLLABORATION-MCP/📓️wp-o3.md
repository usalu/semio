# WP-O3 — Zero-touch hub + browser collaboration

## Verdict (in progress)

- Zero-touch primary `dev s` / local hub path: wired (ensureDevLocalHub + launch `S_HUB_URL` + `🛠️dev🪐️space⚛️react🔒local-only`).
- Collab hub boot: fixed (local-bootstrap FD3 + admin-relay session + small `db4-fs` catalog seed).
- Collab e2e: not green yet — last full attempt timed out waiting for `.semio-table-host` because serve reclaimed the collab hub via `ensureDevLocalHub` with an empty token; run9 sets `S_LOCAL_ONLY=1` on user servers to avoid that.

## Files changed

- `🌎️hub/🚀️local-bootstrap/🏃️execution/🟦️.ts` — `adminToken` / `adminSubjects` passthrough for e2e admin.
- `🧑‍💻dev/🚀️local-hub/🏃️execution/🟦️.ts` — `ensureDevLocalHub` (zero-touch loopback hub + local-bootstrap session).
- `🧑‍💻dev/♻️activation/🌐️serve/🟦️.ts` — calls ensure before Vite.
- `🧑‍💻dev/🔌️vite-plugins/🟦️.ts`, `🧑‍💻dev/🏗️builder/🌐️vite/🟦️.ts` — `/_semio/dev/local-session` middleware.
- `🏛️ShellHost/🟦️.tsx` — claims local-session when token env present.
- `.vscode/🧩️launch.seed.jsonc` (+ regenerated `launch.json`) — primary `🛠️dev🪐️space⚛️react` gets `S_HUB_URL`/`S_DATA_DIR`; new `🛠️dev🪐️space⚛️react🔒local-only` with `S_LOCAL_ONLY=1`.
- `🧑‍💻dev/🧪️tests/🤝️collaboration/🟦️.ts` — hub via `startLocalHub` + admin-relay capability; prefer `db4-fs` catalog seed; `S_LOCAL_ONLY=1` on user serves; `S_COLLAB_HUB_BINARY` override.
- `🔌️plugin/🏗️build/📦️materialization/🟦️.ts` — guestslim fonts from `semio-framework-os-font-assets` / seed copy (not broken `infinite`+`render`).
- `🧑‍💻dev/♻️activation/🔐️lease/🟦️.ts` — reclaim same-pid incomplete plugin-build lease (PID reuse deadlock).

## Commands / results

| Command | Result |
| --- | --- |
| Manual hub without FD3 | Abort: tokio Bad FD / `local bootstrap transport unavailable` |
| `startLocalHub` + `db4-fs` catalog + admin-relay | `/readyz` ready ~3s; `/admin/api/overview` 200 with issued session |
| `bun nx run @semio-tech/framework-os-dev:collab-e2e` (run4) | Hub ready; fonts dump failed (`infinite` `--features render`) |
| collab-e2e (run5) | Fonts OK; taxonomy mid-edit failure (WP-O1 race) |
| collab-e2e (run8) | Hub + activate OK; Vite up; timeout on `.semio-table-host` (empty local-hub reclaim) |
| collab-e2e (run9) | In flight after `S_LOCAL_ONLY=1` on user servers |

## Collab e2e pass log

Pending run9. Prior best: hub provisioned + ready, plugins activated, two Vite servers listening; failed before STEP 1 UI.

