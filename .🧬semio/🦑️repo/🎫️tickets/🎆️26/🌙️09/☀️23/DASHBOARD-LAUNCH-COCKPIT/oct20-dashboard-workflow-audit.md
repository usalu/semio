# Dashboard Human Workflow Audit

This is a bounded source audit on October 20, not a native viewer pass. The current dashboard binary has not yet been successfully rebuilt/installed in this task. The controls owner is executing the nine-library native compiler gate; Core and canonical dashboard installation follow it.

| Workflow | Actual authored route | Evidence and current limit |
| --- | --- | --- |
| Command catalog and palette | Ctrl+B n opens Launcher; registry-backed command tree, search and typed localized forms | `🎛️dashboard/🖥️terminal/🚀️launcher/🦀️.rs`; real PTY journeys include search/start, hover, required forms and quoted Unicode arguments. Pending native journey execution. |
| Workspace shell | `@semio-tech/repo-developer-tools:terminal` typed directory/shell parameters | `💻️client/🛠️developer-tools/🟦️.ts` plans fixed shell argv, physical workspace paths and supervised process; PTY scenario “A workspace terminal streams shell input and exits inside the dashboard” uses the production operation, echo probe, exit0 and durable receipt. Pending actual test execution. |
| Debugging | `:debuggers`, `:setup-debugger`, `:debug` with fixed Node Inspector/Pdb/GDB/LLDB profiles | Native target, working directory and literal bounded args are typed. Debugger stays supervised and the task terminal accepts breakpoint/step commands. Existing developer-tools tests exercise actual LLDB breakpoint/run/print/continue and setup no-install behavior; no dedicated debugger PTY journey was found. Pending runtime suite execution. |
| Source/file navigation | `:open-source` takes required file and optional line | The only explicit source-opening control currently launches system file association or passive VS Code/Codium/Sublime. It does not navigate/edit a document in the dashboard. The dashboard window Body enum contains only Overview, Settings, Help, Launcher and Output; no file browser or document body exists. |
| Source editing | No built-in authored editing control found | Existing launcher input widgets edit command parameters; they are not source editors. `:open-source` delegates to external applications. A workspace shell could run an installed terminal editor, but there is no typed declared editor route, availability/setup, file search/listing control or real editing/save journey. It would be inaccurate to claim a zero-touch source-editing workflow from that shell possibility. |

## Concrete Gap

Current root AGENTS requires humans to use the dashboard and forbids IDE use. The advertised open-source route still offers VS Code/Codium/system open, and the README tells the developer to edit externally and return. Thus the source-edit/navigation part of a sole dashboard developer workspace is currently incomplete. This is an actual reachable-control/source contract gap, not a packaging/documentation issue. A clean completion needs a dashboard-owned source navigation/editing workflow, with safe save/conflict semantics, language-neutral vectors and a real PTY edit→save→independent-read journey. Native process lifecycle/build/test/debug controls should stay in the existing registry and supervisor.

## Exact Sources Read

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🪟️windows/🦀️.rs`: `Body`/`Kind` exhaustively list current view bodies.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🖥️terminal/🚀️launcher/🦀️.rs`: actual registry-backed launcher.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧪️tests/🧭️journeys/🥒️.feature`: existing terminal/catalog/lifecycle journeys; no source-document editing or debugger breakpoint journey.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📋️project.json`: actual terminal/open-source/debug declarations and editor choices.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🟦️.ts`: actual physical-path checks, fixed process plans and external editor dispatch.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧪️tests/🟦️.ts`: current debugger runtime tests and source-open port checks.
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md`: current external-edit instruction and task controls.

No production files were edited for this audit. No frontend runtime claim is derived from source alone.
