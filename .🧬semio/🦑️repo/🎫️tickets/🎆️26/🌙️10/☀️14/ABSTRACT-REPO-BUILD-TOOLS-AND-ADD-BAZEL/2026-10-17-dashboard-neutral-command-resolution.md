# Dashboard Neutral Command Resolution

Read-only Source resolution; no dashboard/daemon/task command executed. Root package aliases and actual dashboard project targets/parser prove these replacements:

| Existing guide | Exact neutral spelling | Source behavior |
| --- | --- | --- |
| bun run dashboard | `bun repo run @semio-tech/repo-dashboard-rs:run --output=stream` | Exact root alias expansion; run target forwards native arguments. |
| bun run dashboard:install | `bun repo run @semio-tech/repo-dashboard-rs:build --exclude-dependencies --skip-cache --output=stream` | Exact root alias expansion; build owner compiles/installs. Do not replace it with a guessed install target behavior. |
| bun run dashboard daemon stop | `bun repo run @semio-tech/repo-dashboard-rs:daemon -- stop` | Declared daemon target has forwardAllArgs:true; DaemonScript adds native daemon prefix. Separator keeps stop as forwarded native DATA. |

`dashboardInvocation` in dashboard installation recognizes neutral run namespace targets run/daemon/workflow/preferences, removes the stream/cache transport options, and appends arguments after --. Thus the exact daemon spelling resolves to native argv [daemon,stop]. `mayRunStale` explicitly permits that pair to stop an installed daemon even when Source is stale, preserving the stated maintenance behavior. RunScript forwards segments unchanged; DaemonScript prepends daemon.

The project is `🎛️dashboard/📦️packages/🦀️rust/📋️project.json`; script is adjacent 📜️script.ts. Root package repo is the necessary Bun bootstrap, not a public Nx backend. Existing GUI registry parameter on daemon target declares action stop. Parsed root launch.json has no literal dedicated daemon target configuration; registry-backed generic launch selection supplies declared targets. Do not invent a named launch or claim a live GUI test from this Source inspection.

README also currently claims no editor/agent launch file exists, contradicting actual required launch registration. Root can remove that stale categorical sentence while preserving its real shared daemon-control-plane description. This report does not authorize editing AGENTS. Exact command behavior is Source-proven, not runtime qualified here.

## Root Documentation Change Review

Current README and clean skill now contain exactly the resolved neutral spellings above, and README describes editor launch selection of the same declared commands. Source forwarding remains exact; no daemon/task was executed. Current reviewed bytes:

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md`: 27564 bytes, SHA256 c09dbfefaab086d4c3163c920d1b3bf15b3aa333e447b6dc1e1f5faf6d39e5f8.
- `.agents/skills/clean/SKILL.md`: 2942 bytes, SHA256 0af7764572861a8ec2358f051fc5560dbe38e72ac2830df9087639fc5a3e0ee0.
