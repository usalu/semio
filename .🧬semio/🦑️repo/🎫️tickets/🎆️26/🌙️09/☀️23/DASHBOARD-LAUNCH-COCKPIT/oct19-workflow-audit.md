# Current Dashboard Workflow Audit

Read-only source and installation audit on 2026-10-19. No tests, native builds, daemon starts, source openers, or modifying Git operations were executed. Existing test declarations are evidence of intended coverage only.

## Current Architecture

The native Rust dashboard owns an in-workspace daemon, command registry, typed launcher forms, retained output, task restart/stop/kill, reconnect, readiness probes, URL opening, preferences and terminal layout. Registry inputs include project targets, authored dashboard declarations, playgrounds, ticket tools and in-process repository actions. Human daily operations are documented in the dashboard README.

The developer-tools owner offers typed source opening and read-only Git status/diff/history/show/blame. Interactive debugging is currently documented as starting a supervised shell and manually invoking an already installed debugger. Source editing and debugger interaction are not native dashboard interfaces.

## Current Installation Blocker

The only installation record found under `.🧬semio/🦑️repo/⚡️cache/🎛️dashboard` is root `installed.json`, version 2, pointing at the former cache/tools location. Current installation code expects a version 3 mode-specific record and executable below `.cache/b/t/m/dashboard-cli/{mode}/{digest}`. `semio` is absent from PATH. Consequently existing records cannot establish that the current source launches successfully. A current admitted build/install must precede runtime claims.

## Actionable Gaps

1. Installation errors in `📦️installation/🟦️.ts` recommend removed `bun run dashboard:install` and `bun run dashboard` bootstrap scripts. Replace with the actual supported dashboard runner route.
2. Dashboard README says editor launch configurations select declared commands, despite also stating no IDE workspace/task/launch configuration is generated or required.
3. Smoke feature prose says the real workspace is mirrored; its TypeScript implementation now uses a named daemon instance on the actual workspace. Keep the language-agnostic contract accurate.
4. Real PTY terminal interaction has a journey scenario. Source opening and Git helpers have process-port tests; this audit did not find a real debugger launch/stop/breakpoint journey. Shell availability alone does not prove friendly debugging.
5. Source planning rejects `editor=system` plus a line before examining `SEMIO_SOURCE_OPENER`. A custom capable opener therefore cannot honor a line when the form retains the default editor.

## Verification Plan

1. Build/install the current native dashboard through its admitted owner and verify the mode-specific immutable installation record.
2. Execute the current binary's help, command catalog check, representative dry runs and passive daemon status.
3. Run the isolated real-workspace smoke suite: finite execution tests, native UI build with retained log, HTTP-ready development server and stop with refused port.
4. Run real PTY journeys for launcher forms, supervised shell input, task cancellation and reconnect, output and navigation.
5. Add or run an isolated real debugger smoke if debugger support is part of completion: launch a fixture program, hit a breakpoint, inspect state, continue and stop descendants through the dashboard.

## Source Locations

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md`
- Dashboard `📦️installation/🟦️.ts`, `📦️packages/🦀️rust/📜️script.ts`, `🧭️cli/🦀️.rs`
- Dashboard `🧪️tests/🧭️journeys/🥒️.feature`, `💨️smoke/🥒️.feature`, `💨️smoke/🟦️.ts`, `🧰️support/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🟦️.ts`, project and tests
