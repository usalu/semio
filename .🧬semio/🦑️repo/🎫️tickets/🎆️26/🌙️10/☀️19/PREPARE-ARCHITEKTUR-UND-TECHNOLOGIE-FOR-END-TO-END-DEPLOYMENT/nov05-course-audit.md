# Course Execution Audit

The course is owned by `@teaching/architecture-course` at `🎓️teaching/🏛️architecture/📚️course`. The existing gate drives desktop, phone, adaptive layout, pets, shared presence, connection shortages, offline results, and repeated proctor failures under development and release rehearsal topologies.

The E2E target declares the two parametrization PDF producers but omits both typology producers, although the course and authenticated PDF transport retain all four documents. A clean checkout therefore lacks required typology publications. The fix will derive required producers from the course declaration and verify consumer target dependencies with an independent Ajv oracle.

Native startup currently lacks Rust and MSVC. The complete bootstrap changes Git configuration and installs unrelated applications, so a schema-declared selective `rust` command will invoke the existing pinned Rust and native compiler installation primitives on Windows, macOS, and Linux. It will finish before the full setup branch.

Playwright v1.57.0 trims the relative file and test title component to 60 characters, then appends project/retry/repeat suffixes. The existing long run and shard paths risk exceeding the 256-character workspace limit; compact generated run and report names will be tested against default diagnostic paths. Sources: [Playwright testInfo.ts](https://raw.githubusercontent.com/microsoft/playwright/v1.57.0/packages/playwright/src/worker/testInfo.ts), [Playwright util.ts](https://raw.githubusercontent.com/microsoft/playwright/v1.57.0/packages/playwright/src/util.ts).

Root owns dependency synchronization. All temporary diagnostics and installer downloads for this ticket will be under `🗑️generated` and removed when done.

## Test-First Status

The neutral native bootstrap command fixture and regression tests were authored before implementation. The first targeted test invocation was corrected to use an explicit `./` operand, as required by the Bun test guard. The bounded 512 MB test then finished with `Cannot find package ajv`; no regression assertion ran. A separate public `repo setup native rust` baseline check is waiting for its 512 MB admission before any production bootstrap code changes. The existing PowerShell command parameter currently accepts only `setup`, so the baseline is expected to reject `rust` before any machine or Git side effect.

The localized PDF consumer contract has also been added before changing the E2E manifest. Required producer IDs come from the course document declarations, and Ajv independently checks array containment. Full course unit and browser verification awaits managed dependency synchronization and native prerequisites.

## Selective Rust Implementation

The expected red was observed through `repo ./📜️script.ts setup native rust`: PowerShell rejected `rust` at its original `ValidateSet(setup)` parameter before any side effect. Top-level `repo setup` is unavailable in both the launcher and modern native CLI; the declared graph entry is `repo run workspace:setup-native -- rust`.

The schema now declares `setup` and `rust`; the TypeScript dispatcher rejects unknown or extra arguments before spawning. Windows selects an early Rust branch before Git configuration and uses the existing BuildTools installation plus checksum-pinned rustup and the repository Rust toolchain. Unix invokes only the existing C/C++ compiler, Rust toolchain, and user-tool links. The default complete setup reuses the factored Rust helpers. A scoped 1024 MB installation has started through the declared target, with TEMP/TMP under this ticket’s generated directory.

## PDF Producer Regression

The language-neutral consumer fixture was executed directly against the authored course and project JSON while the managed test runner waits for Ajv. It failed with exactly two missing producers: `test-e2e` lacked `build-typology-en` and `build-typology-de`. Both prerequisite edges were added. The same assertion then passed for all four consumers and all four localized publications. The committed Vitest regression also compares this result to independent Ajv containment validation once managed dependencies are available.

## Native Setup Review

Review found an existing Visual Studio product can lack the VCTools workload. Rust setup now probes the actual architecture-specific MSVC linker, modifies an existing installation when it is missing, and verifies it after installation instead of trusting the WinGet product ID. The installed `setup.exe modify` operation uses an owned synchronous hidden process; bootstrapper-only `--wait` is retained only for the WinGet bootstrapper. Modification may require Windows administrative authorization, which is not bypassed. Official references: [Visual Studio command-line parameters](https://learn.microsoft.com/en-us/visualstudio/install/use-command-line-parameters-to-install-visual-studio?view=vs-2022), [command-line examples](https://learn.microsoft.com/en-us/visualstudio/install/command-line-parameter-examples).

Neutral cases cover an existing complete compiler, an existing product missing the linker, first installation, and a failed repair. Rustup discovery checks its owned user directory before PATH; the standalone Unix entry now rejects extra arguments as the TypeScript dispatcher does.

## Verified Native Entry

The declared graph target reached source admission but was canceled deliberately before installer execution because its default argument forwarding had not yet been proven; dropping `rust` would select unrelated full setup. The verified direct root entry is `repo ./📜️script.ts setup native rust`, which already produced the intended red and preserves the original repository invocation. The actual installation now uses this entry under the same 1024 MB reservation. Source comments document only this verified route.

## Focused Green Validation

The retained `check-course-bootstrap.ts` input harness executed the production bootstrap parser against 5 neutral command cases, the actual Windows compiler preparation function against 4 mocked installation states, and the actual E2E path helpers against 10 report cases. All passed. The longest authored screenshot/error/trace path, including `retry1` and `repeat2`, was 250 UTF-16 characters; the previous default screenshot path was 273. These checks do not claim the full Ajv/Vitest suites or a real browser run.

## Owned Files

- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧪️tests/🧰️toolchain/🔣️.json`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🧪️tests/🧰️toolchain/🟦️.ts`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🟦️.ts` (new)
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🔵️.ps1`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/🐚️.sh`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🎛️commands/🔧️setup/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/📖️assignment-pdfs/🧫️cases.json`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/📖️assignment-pdfs/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`
- `🎓️teaching/🏛️architecture/📚️course/🧫️fixtures/🎭️e2e-output/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🧱️local-stack/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/README.md`
- Ticket inputs: `check-course-bootstrap.ts`; audit: `nov05-course-audit.md`.

## Locked Browser Revision

The current `bun.lock` pins Playwright, playwright-core, and @playwright/test to 1.64.1, although package.json allows ^1.57.0. The locked release preserves the same 60-character test stem plus project/retry/repeat naming. This was verified against its primary source: [v1.64.1 testInfo.ts](https://raw.githubusercontent.com/microsoft/playwright/v1.64.1/packages/playwright/src/worker/testInfo.ts), [v1.64.1 util.ts](https://raw.githubusercontent.com/microsoft/playwright/v1.64.1/packages/playwright/src/util.ts). The earlier 1.57.0 check is therefore consistent with the actual lock, not the sole evidence.

## Browser Output Ownership

The forwarded-argument corpus reproduced a second red: the public plan accepted `--output outside`, which could override the gate’s owned ticket output. The plan now rejects output/configuration/reporter/last-run-file overrides, preserves option-looking grep values, limits retries and repeat-each to three digits, and pins the last-run file within each owned wave directory. Fourteen neutral argument cases passed in the focused harness. The fixture now includes the maximum supported `retry999` and `repeat999` suffixes. The CLI flag surface was checked against the [locked Playwright v1.64.1 program](https://raw.githubusercontent.com/microsoft/playwright/v1.64.1/packages/playwright/src/program.ts).

## Installer Failure and Path Probe

The actual Rust profile reached the VS BuildTools bootstrapper. WinGet downloaded and verified it, but its decompressor failed with exit 42 / 0x8007002a before Rust installation. Its log remains under generated/native. The longest observed WinGet temporary path was 250 characters. A read-only GetShortPathName P/Invoke probe could not compile its small C# wrapper into the Unicode temporary path (`directory name invalid`), providing a second concrete path-compatibility failure. The probe is therefore using the Windows FileSystemObject ShortPath property to inspect existing 8.3 aliases without a compiler or changing global state.

## Final Focused Browser Check

The final focused harness passed 5 native command cases, 4 mocked compiler installation cases, 14 forwarded-argument ownership cases, and 10 report-path cases. With the maximum supported `retry999` and `repeat999` suffixes, the longest retained artifact path is 254 UTF-16 characters. The full managed Ajv/Vitest and real browser gates remain pending dependencies and machine prerequisites.

Retained probe inputs: `inspect-native-temp.ps1` and `inspect-native-temp.ts`. Windows FileSystemObject.ShortPath returned a non-ASCII path, so existing 8.3 names do not solve this ticket’s installer temporary path.

## Owned Native Temporary Port

The schema now declares a 128-code-unit temporary-root budget and ASCII alias plan. The new `native/bootstrap/📁️temporary/🟦️.ts` port selects a direct ASCII root, tries unique owned user-state junctions, and falls back to an owned user DOS-device mapping when machine-state roots contain Unicode or are unavailable. It verifies the exact physical target by directory identity, enforces ticket-generated containment, waits for the synchronous installer body, and removes only its exact alias nonrecursively. It never removes the physical target.

`check-native-temp.ts` passed six neutral planning cases and real Windows ownership checks for both a junction and forced DOS-device fallback. Both could write/read a probe in the ticket-generated physical directory, both aliases disappeared after close, and the target remained. The actual installer retry is using `TEMP/TMP` only in its child environment, pointing through this port to the ticket-generated root, which reserves 128 characters for installer-relative files. No global TEMP setting or node_modules binding changed.

Additional owned source: `🧰️framework/🛍️products/🦑️repo/🔨️modules/🔩️native/🥾️bootstrap/📁️temporary/🟦️.ts`. Additional retained input: `check-native-temp.ts`.

## Exact Vendor Launch Diagnosis

Windows decoded exit 42 as an unknown Win32 code (0x2a); HRESULT 0x8007002A was generic, and 0x8007013D explained that no message text was available. The extraction log explicitly used an ASCII junction path on the second failed run. WinGet’s own diagnostic, however, showed it launched the bootstrapper from the canonical physical Unicode ticket path. This distinguishes executable launch path from TEMP path.

The cached manifest identifies the downloaded executable as SHA256 `ea54b67bfac576b6dd097f84eeaf6893780d2a613f90448ff038e8a2777a0afb`. A bounded isolated input (`install-msvc-dos.ts`) rehashed the same bytes, launched them through an owned ASCII DOS-device executable path, and extraction succeeded. It then reached the real Windows consent process titled `Visual Studio Installer erfordert Ihre Berechtigung`. That process remains live; no permission click, credential entry, bypass, dismissal, or compiler-readiness claim was made. The parent informed the user of the OS prompt.

The nested DOS-root check initially failed because Bun realpath preserves DOS device names. The fix accepts the canonical generated root only after exact nonzero directory identity proves equality, then applies the physical path budget. Unknown/outside aliases remain refused. A real nested-port check now passes for both junction and DOS aliases, and exact nonrecursive cleanup still preserves the physical target.

## Independent Rust Progress

Pinned Rust provisioning initially failed because the host lacks the optional Get-FileHash cmdlet. Save-VerifiedDownload now streams through system .NET SHA256, with independent existing @noble/hashes vectors in the full test suite. The focused harness passed all four correct/tampered checksum cases. The retry installed pinned rustup 1.29.0 and is synchronizing the repository’s nightly-2026-07-20 MSVC toolchain. The existing unattended `-y` behavior warns about missing MSVC without starting another VS installer ([pinned rustup source](https://raw.githubusercontent.com/rust-lang/rustup/1.29.0/src/cli/self_update/windows.rs)). Full native preflight continues to require the real linker.

Additional retained inputs: `install-msvc-dos.ts`, `install-rust-assets.ps1`, `install-rust-assets.ts`, `inspect-native-process.ts`, and `retry-native-dos.ts`.

## Rust Assets Verified

The independent Rust provisioning finished successfully: all eight components of `nightly-2026-07-20-x86_64-pc-windows-msvc` are installed and active through `D:/semio/rust-toolchain.toml`. The registered `rustup --version` query confirms rustup 1.29.0 and active rustc 1.99.0-nightly (`9f36de775`, 2026-07-19). A separate documented `tools exec --light rustc -vV` query was classified as a 3584 MB Cargo task despite being read-only; it was canceled as redundant after the authoritative rustup query succeeded. No resource-policy bypass or compiler build occurred.

The actual Microsoft installer remains at its previously observed OS consent dialog. Full native compiler readiness still requires its real MSVC linker and SDK; Rust asset installation alone does not satisfy that gate.

## Controlled Compiler Bootstrapper Integration

The schema-owned `MsvcBootstrap` pin records the observed bootstrapper release 18.11.3, exact Microsoft URL and SHA256, owned `cpp.exe` filename, and required VCTools workload. This is a bootstrapper release identifier, not an assertion about installed Visual Studio edition or MSVC toolset version. Fresh setup now uses Save-VerifiedDownload, then launches that exact owned ASCII executable through the existing awaited command interface. It does not invoke WinGet’s canonicalizing installation path or replay failed installers. Existing-instance modification and generic vswhere/linker discovery remain. Installer results 0 and 3010 are accepted without an automatic reboot; functional readiness still requires the real compiler/proctor build.

The focused composition checks passed both successful verification/launch and failed-verification/no-launch, with exact argument and success-code contracts and scratch cleanup. The final focused report now includes 5 command cases, 4 checksum cases, 2 compiler-bootstrap composition cases, 4 compiler-state cases, 14 browser argument cases, and 10 report cases; retained browser paths remain bounded at 254 UTF-16 characters. Full managed third-party-oracle suites still await dependency readiness.

### Portable Syntax After Ticket Temp Integration

The direct root native setup automatically uses the active ticket generated directory for Rust TEMP/TMP on Windows and TMPDIR on Unix. Full setup retains its existing environment. The retained `check-course-syntax.ts` input passed Bun syntax parsing for seven changed TypeScript sources and the native PowerShell parser for the bootstrap script. This is syntax evidence only; full dependency-backed tests and compiler readiness remain pending. Native Git runtime has no bundled Bash executable at its conventional bin/usr paths, so no Bash syntax result is claimed.

The focused actual-function harness was repeated after the final temp-routing edit. It passed 5 command cases, 4 checksum cases, 2 verified bootstrap composition cases, 4 MSVC discovery/repair cases, 14 E2E forwarding cases and 10 report-name cases, with maximum retained artifact path 254 UTF-16 units. An initial invocation lacked SEMIO_TICKET_DIR and stopped before checksum probes; the correctly ticket-scoped invocation passed. Conventional native Git Bash locations under Program Files, user Programs and the bundled Git runtime are absent on this machine; Unix shell syntax remains unverified here.

### Finite Physical Path Audit

A read-only traversal of 240 generated entries found one stale initial WinGet cache `versionData.mszyml` output at 257 UTF-16 units. That individual generated cache file was deleted; no parent directory, installer executable, active DOS alias or running process was touched. No remaining generated entry exceeds 256. The controlled pinned bootstrapper path replaces WinGet launch and avoids this cache tree in future selective installs.

The real temporary-port proof was repeated after final integration: all six neutral cases passed; an owned ASCII junction and a forced DOS fallback (`Y:` because the existing installer owns `Z:`) both resolved to the exact ticket generated directory. The probe cleaned only its own junction/DOS aliases and preserved the physical target and live installer alias.

The native module README now documents the verified direct root Rust setup route, compiler/linker postcondition, ticket-contained ASCII temporary aliases and normal Windows elevation boundary. It does not document the unverified generic target argument-forwarding route.

The published course `test` and `typecheck` targets both require the framework/UI Rust generation and actor type-generation closure before executing the TypeScript checks. The published dev and test-e2e targets also require the proctor executable and all four assignment PDFs. Consequently the full public course gates cannot establish readiness before the actual MSVC compiler/linker installation completes; dependency publication alone enables focused library-oracle regressions.

### Execution Gate Ledger

| Gate | Current Evidence |
| --- | --- |
| Narrow native command red | Existing direct root router rejected `rust` before selective profile implementation |
| Selective native logic | Focused actual-function harness passes command/compiler/verification/failure contracts |
| Independent library oracle | Ajv and Noble source tests authored; await published JavaScript dependencies |
| Native temporary aliases | Neutral corpus plus actual junction/DOS identity, physical-write and exact-cleanup proofs pass |
| Rust assets | Rustup 1.29.0 and all eight pinned nightly components installed |
| MSVC linker | Existing installer session 72311 awaits normal Windows elevation, no readiness claim |
| Source syntax | Seven TypeScript files and native PowerShell parser pass |
| Unix shell | Source implemented; actual Bash unavailable on this Windows host |
| PDF dependency closure | Neutral manifest assertion red then green for four consumers and four assignment documents |
| E2E artifact bounds | Neutral gate functions pass, maximum supported retained path 254 UTF-16 units |
| Public course unit/typecheck | Await real native Rust codegen prerequisites and JavaScript publication |
| Live website/server/browser E2E | Pending prerequisites; no browser success claimed |

### Bounded Live Dependency Writer Counters

The retained `inspect-writer.ts`/`.ps1` input sampled only known writer PID 25716 twice, 3.052 seconds apart, through the existing repo process interface. PID identity remained Bun with start time 2026-11-05 05:00:37 UTC. Total CPU increased from 1,586,187.500 ms to 1,586,234.375 ms: 46.875 ms over 3,052.123 ms (about 1.54% of one core during this short sample). Working set was approximately 947.1 MB, private bytes 1,265.4 MB, handles 164. Both instantaneous snapshots grouped 24 threads as Wait. These samples show little CPU activity during this interval; they cannot distinguish filesystem waits from sleeping, lock waits or expensive sparse replay, and they do not establish dependency progress/completion. No attach, debugger, security UI, process termination, endpoint walk or UAC action occurred. Raw generated counters: `🗑️generated/writer-process.json`; retained inputs remain in ticket.

### Immediate Dependency Child I/O Sample

The retained `inspect-writer-children.ts`/`.ps1` input confirmed parent Bun PID 25716 and queried only its immediate children, using bounded native CIM queries. Its sole child was managed Bun PID 9892, created 2026-11-05 06:31:35.840 UTC. Command lines were reduced to booleans: frozen-lockfile=true, ignore-scripts=true, backend-copyfile=true; no full command lines were emitted. Two snapshots at 06:43:25.891 and 06:43:28.979 UTC showed read bytes increasing by 12,151,198 and write bytes increasing by 12,151,198 (about 3.93 MB/s each across this interval), read operations +10, other operations +60, write operations unchanged at 1992. CPU stayed 22,765.625 ms, working set about 75.4 MB and private bytes about 179.8 MB. This directly establishes child I/O activity during the sample despite quiet CPU. It does not prove publication or full readiness, and native transfer counters need not correspond exactly to completed filesystem writes. No debugger, attach, cancellation, security UI, endpoint traversal or unrelated process inventory occurred. Raw generated evidence: `🗑️generated/writer-children.json`.

At the dependency child-to-input-replay phase transition, the requested single installer check found session 72311 still live at 7,786 seconds with the original bootstrapper PID 20876 and original consent PID 8804 showing the same Windows permission dialog. No terminal install outcome or linker readiness is claimed. No replay, process mutation or OS consent interaction occurred.

### Cross-Platform Ticket E2E Path Regression

New language-neutral fixtures use the exact current ticket under D:/semio, the default devcontainer /workspaces/semio, and a native macOS home clone /Users/anna/semio. The retained actual-helper regression reproduced the current guard refusing all ten waves on Linux/macOS before production changes. The minimal fix removes redundant nx/e2e nesting beneath ticket generated and uses exclusive six-character mkdtemp run names. Wave names, retry/repeat limits, diagnostics, physical containment and the unchanged 108-character guard reserve remain intact. Afterward actual report guards pass all waves: maximum retained artifact paths are 246 Windows and 255 Linux/macOS UTF-16 units. Longer checkout/ticket prefixes still receive the existing explicit guard rather than exceeding 256. No source copies or physical-path aliases are used for E2E artifacts. Vitest cases retain independent Ajv UTF-16 array validation; full dependency-backed execution remains pending publication.

After the compact-layout change, source syntax validation passed again for all seven changed TypeScript sources and PowerShell. The focused actual-function suite also passed again with maximum Windows artifact path updated from 254 to 246. The cross-platform retained helper input additionally creates two real six-character run directories directly beneath ticket generated, verifies distinct ownership and exact parent containment, and passes all three host prefix guards.

Independent source review of the compact layout confirmed that dirname(ticketOutput(...)) selects the authenticated ticket’s generated root, mkdtempSync(join(root, sep)) keeps a trailing separator and creates a six-character child rather than a sibling, all wave mappings remain distinct, and the 108-unit reserve and existing caps remain unchanged. No new actionable defect was found in this narrow change.

## November 5 Continuation: Native State

A short read-only repo tools exec --light query of the retained native process adapter returned no matching Visual Studio bootstrapper, setup, consent, or winget process. A second Windows process query found no matching vendor or consent process; the logical-drive query listed only C: and D:, and recursive Visual Studio linker discovery returned none. The query path was validated by observing the querying PowerShell process. The prior installer and Z: alias are therefore no longer observed, but no installer completion status or working MSVC linker has been established. No installer replay or security interaction was performed.

A separate tools verify rust msvc attempt failed before tool verification during source admission: the Nx authorizer closure exceeded its 16,777,216-byte cap at 16,788,269 bytes across 1,831 files. This is an infrastructure failure, not compiler evidence. Read-only Bun resolution also found Ajv, Vitest and @noble/hashes absent from the workspace; third-party-oracle tests remain blocked.

## Physical Diagnostic Containment

The old dependency writer PID 25716 and wrapper PID 16752 were absent from an exact Win32_Process query; stale queue observations alone do not establish live ownership. A neutral actual-filesystem regression showed that an existing generated-directory junction redirected E2E output outside the selected ticket. The shared ticketOutput helper now verifies each physical directory before proceeding to the next, and rejects redirected generated or nx directories. The original generated-junction case failed before the fix; three direct/generated-junction/nx-junction cases now pass using real Windows junctions, with the outside directory unchanged. The Ajv-backed local-stack test and existing injected-helper test were updated; the dependency-backed suite remains pending. These path-based checks reject pre-existing redirections but do not prove safety against concurrent adversarial replacement between operations. Retained input: check-output-containment.ts.

The three modified TypeScript sources for the containment change parse successfully with Bun. A single ticket-scoped selective Rust/MSVC setup retry is session 49088, admitted through the normal 1024 MB / one-core capacity queue. Its initial state is queued behind existing work; no installer or prompt has started.

## Fresh Selective Installer Attempt

One normal 1024 MB / one-core capacity request, session 49088, waited approximately 309 seconds and was admitted. The verified selective root setup launched the pinned MSVC 18.11.3 bootstrapper from owned ASCII junction C:/Users/Anna/.semio/native-temp/nt-mW4KK0/t/semio-cb83e16bfa4a4438ac2bb445382a7410/cpp.exe with --wait --quiet --norestart and the VCTools workload. Read-only Win32 inspection observed cpp.exe PID 15584 parent 15084 and consent.exe PID 31088 parent 10276. Only C: and D: logical drives were present; no DOS alias. Windows authorization is required and the user was informed. No prompt manipulation, automatic reboot, or duplicate installer occurred. The actual linker remains unverified.

The wrapper terminated exit 1 because PowerShell strict mode observed an undefined LASTEXITCODE: a GUI bootstrapper launched asynchronously through the call operator. The detached cpp and consent processes initially remained live, and the scratch executable had already been removed; no new installer was launched. The launch contract test failed before the fix and passed after switching to Start-Process with Wait, PassThru and Hidden; mocked exit 0 and 3010 succeed, 42 fails, and checksum failure prevents launch. The full retained focused harness passed; the portable syntax harness passed seven TypeScript sources and PowerShell. A later exact OS query found neither detached PID, but their result remains unknown. A recursive Visual Studio linker query returned no linker. The next attempt must not assume prior installation success.

## Corrected Awaited Setup Attempt

A broad preflight process query found no vendor, installer, consent, or downstream setup processes; matches were only the read-only query chain. No MSVC linker was found. Exactly one corrected managed attempt, session 76423, was admitted and ran the pinned verified MSVC bootstrapper through the awaited process path. It returned 1602 and the setup exited 1. The short prompt interval was not observed; no security interaction or additional retry occurred. Follow-up native process and recursive Visual Studio linker queries returned no results. Compiler readiness and the downstream course runtime remain unverified.

The OS Win32Exception message for 1602 says the installation was canceled by the user. No observation identifies the actor or mechanism that caused this result.
