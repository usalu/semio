# Shell bridge: live-shell side of the semio MCP server

Status: NOT verified live. The shell could not be started within about 80 minutes on this machine (host contention), so ui_focus, ui_reveal, the ui resources and conversation_reply `shells:1` were not exercised against a real shell.

## How a developer starts the live shell through `repo`
- Browser shell (React): `repo ./📜️script.ts dev playground <variant> react prototype` (for example `note`). Its Vite dev server carries `semioAgentBridgeRendezvousVitePlugin` (`🧑‍💻dev/🔌️vite-plugins/🛰️agent-bridge-rendezvous`), which writes `<rendezvous>/sessions/<pid>.json` and serves the newest live offer at `/__semio/agent-bridge`. The page polls it and dials `/bridge`.
- `dev s` in the README means the s variants in `✏️s/🧑‍💻dev`; there is no `.claude/launch.json` entry for a shell with the bridge plugin. Entries such as `stdio-pdf-react` use ticket-owned scripts that need a prior activation (receipt under `.cache/b/p/m/browser/prototype/runtime/react/<variant>/activation`); without it the config load fails with ENOENT on `🔣️receipt.json`.
- A suitable harness exists: `✏️s/🧑‍💻dev/🎭️variants/🌍️gis/💡️services/🧪️tests/🤖️live-agent-loop/🟦️.ts` (real gateway + real browser shell, S_OS_MCP_LIVE_SHELL_URL).

## What blocked the run
`repo ./📜️script.ts dev playground note react prototype` first has to build prerequisites (`@semio-tech/draw-drawing-rs:graph-generate`, the project graph slice). With other agents building, the slice workers were busy ("2 of 2 workers busy") and then "waiting for host capacity (not enough free memory)", under 1 GB available. No dev server port opened; I stopped the build and removed my temp files.

## Isolation needed when several gateways run
`liveAgentBridgeOfferFor` serves the NEWEST live local offer. At the time of the run three other live gateways (pids 2291, 7106, 94109, other agents) were publishing offers in `~/.semio/agent/bridge/offers`, so a shell started with the default directory would attach to whichever published last, not to a given probe. Pin both sides with `S_AGENT_BRIDGE_DIR=<dir>` (dev server and gateway) to test one gateway against one shell.

## Stale offer hygiene (verified by code reading and tests)
- Gateway: `publish_offer_in` sweeps offers of dead pids (`🛰️rendezvous/🦀️.rs`, covered by `publishing_an_offer_sweeps_the_offers_of_gateways_that_are_gone`); the offer file is removed on drop.
- Shell side: `liveAgentBridgeOfferFor` ignores non-parseable records and dead pids (`processPidExistsV1`). This had no test on real files. Added `🧰️framework/🛍️products/💻️os/🔨️modules/🧑‍💻dev/🔌️vite-plugins/🛰️agent-bridge-rendezvous/🧪️tests/🟦️.ts` (3 bun tests: dead newer offer loses to a live older one; dead, garbage, wrong-version and non-json files give no offer; absent directory gives no offer). Ran with `bun test ./🟦️.ts`: 3 pass. Not yet run through `repo test` (graph queue).
- Residual risk: pid reuse makes a dead gateway's offer look live until its file is swept; a dial then simply fails.

## Tooling
`🐍️probe-shell.py` extends `🐍️probe.py` with `{"sleep": s}` lines and `"until_ok_within": s` to retry a tool call until the shell has attached. Usage: `S_AGENT_BRIDGE_DIR=<dir> python3 🐍️probe-shell.py calls.jsonl 1800`.

## Next steps once the host is idle
1. `mkdir -p /tmp/sb/offers; S_AGENT_BRIDGE_DIR=/tmp/sb repo ./📜️script.ts dev playground note react prototype`
2. Open the printed URL in the built-in browser.
3. `S_AGENT_BRIDGE_DIR=/tmp/sb python3 🐍️probe-shell.py calls.jsonl 1800` with ui_focus (until_ok_within 120), ui_reveal, resources/read for semio://window, semio://ui/active-context, semio://ui/agent-messages, semio://ui/selection, and conversation_reply expecting shells:1.
