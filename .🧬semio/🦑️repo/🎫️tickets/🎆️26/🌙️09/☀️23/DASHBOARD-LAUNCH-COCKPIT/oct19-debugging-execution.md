# Typed Developer Debugging Execution

## Implemented Controls

The existing developer-tools domain now owns `@semio-tech/repo-developer-tools:debug` and `debuggers`. The debug target exposes localized English/German profile, workspace program, working-directory and literal argument-list fields. A profile must be selected; there is no language default. The argument field accepts a JSON list, up to 64 strings and 8192 total UTF-16 units; unknown fields, duplicate flags, NULs, excessive arguments and arbitrary debugger options are refused.

The availability control lists actual installed executable paths, missing optional programs, and the separate Windows GDB target-path constraint. Its wording explicitly distinguishes an installed frontend from support for the selected target. Missing programs produce actionable localized guidance to check availability, select an installed profile or complete tool setup. Windows GDB Unicode target/cwd requests are refused before process execution with guidance to check LLDB's installation before selecting it; missing LLDB is never described as ready.

Debugging remains in the dashboard task. The existing `runOwnedCommand` path retains streamed stdout/stderr, inherited task stdin, the original task cancellation signal, process-tree termination and original invocation budget. The debug target is continuous; it does not detach or invent a new operation deadline. Open-source navigation and all existing Git/terminal routes remain unchanged. There is no shell evaluation, IDE configuration, debug adapter, compatibility wrapper or new runtime dependency.

Program and directory requests pass the existing lexical and physical workspace-boundary checks, symlink-escape rejection and 256-character limits. The executed program argument is relative to its admitted cwd, with a leading ./ where needed to avoid option interpretation. This shortens argv without relocating programs, copying sources or bypassing custody.

## Profiles and Evidence

| Profile | Fixed debugger argv before target | Actual evidence on this Windows host |
| --- | --- | --- |
| node-inspect | node inspect --port=0 | Installed Node24.14.1 source confirms the CLI accepts port0 and discovers the assigned port. Actual owned process attached to a fresh local inspector port, paused the fixture at its first line and was cancelled with its child. |
| python-pdb | python -X utf8 -m pdb (python3 on macOS/Linux) | Installed Python CLI usage and UTF-8 mode verified. Actual Pdb evaluated6*7 as42, continued the fixture, preserved spaces, quote, trailing backslash, shell-looking text and emoji in exact debuggee argv, then quit successfully. UTF-8 mode fixes the observed cp1252 failure on Unicode workspace paths. |
| gdb | gdb --nx --quiet --args | Installed GNU CLI help verifies the flags. Actual Windows frontend evaluated42 and exited. A compiled native fixture under the Unicode ticket path could load symbols but its MinGW GDB canonicalization changed Unicode paths to question marks and could not launch it. The product therefore admits Windows GDB only for ASCII target/cwd paths, while preserving ASCII workspace artifact plans. Native breakpoint assertions are available in the POSIX fixture branch but were not executed on this host. |
| lldb | lldb --no-lldbinit -- | Absent on this host. Fixed argv and all three platform plan vectors pass. The [official LLVM LLDB manpage](https://lldb.llvm.org/man/lldb.html) documents disabling init files and placing the executable and its arguments after --. No actual LLDB process or macOS/Linux debugger session was claimed. |

The early `node inspect --help` probe unexpectedly started a debugger. The exact two newly owned processes were identified (parent121160, child84312) and terminated explicitly. No broad process sweep was used. Subsequent actual Node fixture tests use the repository-owned process runner and its cancellation path. Some Node cancellation runs printed an ECONNRESET from the installed CLI while both tracked processes exited; the tests require the expected owned cancellation, attachment and initial breakpoint evidence.

## Verification

- Schema-first shared JSON vectors initially exposed12 failures in absent debug dispatch/argument parsing; after implementation those tests passed.
- Final complete developer-tools suite, using its declared300000ms test timeout: **82 tests passed, 335 assertions, 0 failures**. This includes the existing source navigation, Git, terminal and localization tests plus debugging.
- Focused debugging/registry run: **26 tests passed, 172 assertions**. Includes Windows/macOS/Linux profile plans, Windows GDB Unicode rejection, positive ASCII target/root-cwd planning, schema admission, literal argv comparison through third-party string-argv, immutable arguments, actual symlink escape, cancellation, availability compared to Ajv and SQLite, and the actual Node/Python/GDB frontend processes.
- Strict TypeScript checks passed separately for the domain source, existing script entry and complete test source with repository TypeScript5.9.3 and explicit compiler flags. The first attempted --ignoreConfig flag was unsupported by this installed compiler; checks were rerun successfully without it.
- Read-only git diff --check passed after normalizing only the touched developer-tools files to LF.
- The full native dashboard PTY/form journey remains unverified because the admitted native installation is blocked by the previously reported orphan backing/receipt condition. No storage enforcement or cleanup policy was modified.

## Remaining LLDB Setup Requirement

The current native dependency/bootstrap implementation provisions cargo-nextest, cargo-llvm-cov and a Rust toolchain including llvm-tools-preview; it does not publish an LLDB executable generation. LLDB remains an optional probed system debugger. This task did not invoke external package installers, fabricate archive digests or claim zero-touch LLDB installation.

The appropriate future CLI-only setup is a pinned LLDB tool generation within the existing bounded tooling publisher: schema-owned platform/architecture archives and verified official digests, descriptor-bound downloads/extraction, existing Original resource controls and cancellation, sealed executable closure publication, and exposure through the existing owned tool environment/availability view. Relevant owners are `library/⚡️caching/🚀️bootstrap/🛠️tools` and the native dependency script `library/⚡️caching/🚀️bootstrap/📦️dependencies/🏗️native/📜️script.ts`. Actual publisher admission, installation and LLDB target launch require the native store to be available; they remain unverified setup work. No IDE bundle or debugger adapter is needed.

## Files Updated

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📜️script.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/📋️project.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧫️fixtures/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/💻️client/🛠️developer-tools/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/README.md`

The report itself is the only new authored ticket file in this slice. Generated debugger fixtures stayed in the ticket's generated/debugger subtree and can be retired with the parent's ticket output cleanup. AGENTS files, historical scripts, Git state and worktrees were not modified.
