# Root Workspace Diagnostic Declaration Audit

Read-only scoped causal review of actual Part21-visible Cargo101 failure; backend owns repair. No Cargo, broad registry crawl, Source edits or workspace mutation.

Composition publication manifest is framework/products/os/modules/plugin/composition/publication/packages/rust/Cargo.toml (actual emoji path retained in root members). Its explicit package.workspace resolves to /home/deck/semio, and root Cargo.toml explicitly lists that exact publication member. Therefore inherited dependencies resolve against ROOT workspace.dependencies, not the nearer framework workspace. This is explicit existing ownership, not evidence the member should move workspaces.

Its inherited dependency set is semio-framework-diagnostic, semio-framework-artifact-reference and semio-framework-trace (trace also dev dependency). Root already declares artifact-reference and trace canonical path entries, but diagnostic is missing. The reported failure is consistent with that concrete missing root declaration; no production Source/type implementation diagnosis is needed at this parse stage.

Canonical diagnostic manifest exists at 🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust/Cargo.toml, package name semio-framework-diagnostic, lib name semio_framework_diagnostic/path 🦀️.rs. Its own package.workspace resolves to framework, which does not prevent another workspace from depending on it by canonical path. Clean correction is ROOT [workspace.dependencies]:

~~~toml
semio-framework-diagnostic = { path = "🧰️framework/🔨️modules/⚠️diagnostic/📦️packages/🦀️rust" }
~~~

Do not change publication ownership or replace its intentional workspace dependency merely to avoid the missing declaration. This audit verifies this same-chain mismatch only; successful manifest parsing/compilation after repair remains actual owner execution evidence, and no Part21 pass follows from this read. Backend notified.
