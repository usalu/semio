# Dashboard Exploration — 19 October 2026

Read-only source inspection; no tests or runtime launch performed by this exploration agent.

## Existing Architecture

The dashboard already implements the requested overall shape. Its module is `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard`. Native Rust crates separate CLI, terminal UI, registry, inventory, preferences, repo-domain actions, invocation catalog, launch planning, daemon and wire protocol. The terminal launcher accepts typed command parameters; the daemon owns task processes, retained output, restart/stop/kill, reconnect and readiness. CLI and interactive UI share the registry and daemon. The README explicitly declares the dashboard the only developer control plane.

Registry sources are owner `📋️project.json` targets and dashboard metadata, published graph inventory, playground metadata, ticket `🎮️commands.json`, and in-process repo-domain actions. Root package scripts deliberately are not another registry source. Canonical graph publication is `.cache/b/g/m/canonical.json`.

## Already Preserved Workflows

`💻️client/🛠️developer-tools` has actual schema, fixture, implementation, owner manifest and tests for source opening, system terminals, Node/Python/GDB/LLDB debugging and read-only Git status/diff/history/show/blame. The dashboard README also maps setup/native setup, build help, affected tests, individual tests, Bun test selection, TypeScript project checking, verify/lint/format, build/publish, preview opening, repo ticket/goal actions, run-many and authored scripts to registry entries. Do not duplicate these features merely because the task wording asks to preserve IDE workflows.

The source opener intentionally needs a capable selected editor (or `SEMIO_SOURCE_OPENER`) to support line navigation; system opener with a line rejects. Windows GDB currently rejects Unicode paths. Those are documented capability limits, not evidence that the entire workflow is absent.

## Concrete Remaining Integration Checks

1. Actual startup and test execution on this macOS checkout remain unverified. Another agent owns bootstrap/execution, which is the highest-value next step.
2. The root AGENTS instructions retain the explicit launch.json-only developer rule. They are protected from editing. The authority contract therefore must explicitly represent that constraint while guaranteeing no generated IDE control-plane artifacts; main agent owns that policy integration.
3. Existing authority coverage is extensive, so fix failing source/contract cases rather than introduce another inventory mechanism. It checks active owner commands, all active source directories, mutable developer instructions, no editor projection route, retained headless clients, devcontainer lifecycle and registry declarations.
4. `.vscode` is absent in rg inventory. `.cursor/plans` remains historical planning content and is intentionally allowed by the authority policy. `.devcontainer` is runtime infrastructure with Bun repo lifecycle tasks and no editor customizations; retain it.

## Existing Verification Entrypoints

Owner `@semio-tech/repo-dashboard-rs` exposes `test-authority`, `test-execution`, `test-unit`, `test-quick`, `test-long`, `test-exhaustive`, `test-coverage`, `test-journeys`, `test-windows`, plus run/build. Use `bun repo run <owner>:<target>`.

The authority test at `🧪️tests/🧭️authority/🟦️.ts` independently validates schema using AJV, source inventory using fast-glob, terminal widths using string-width, shell aliases using Python shlex and UI properties using JSDOM/lodash. It imports source-discovery, journey-source, executable and dispatch/reuse suites. Native journeys have an independent vt100 screen model with real PTY/ConPTY support. Existing schema-first language-agnostic contracts and third-party oracles should be extended rather than replaced.
