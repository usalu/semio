# Architecture Course End-to-End Verification

Updated 2026-11-05T06:33:08.133Z. Focused fixtures and source audits are complete where listed below; canonical gates and live browser proof remain separate.

| Requirement | Verified Evidence | Remaining Work |
| --- | --- | --- |
| Repository interface | Native install 4cbfc649…; paired-root resolution, cold import output 42, registered Go suites, and actual namespace no-write checks passed | Dependency-backed workspace oracles |
| Rust assets | Rustup 1.29.0 and nightly-2026-07-20 installed; all eight components and active rustc verified | Actual compiler/proctor build with MSVC |
| Windows compiler | Checksum-pinned vendor extraction succeeded through owned ASCII paths | Existing OS consent dialog, linker/SDK verification, and actual build |
| JavaScript dependencies | Single writer 33389 retained; Source fences passed, installed-tree traversal and another ownership pass completed; replay advanced into an owned Bun child | Successful terminal result and ready publication |
| Storage preflight | Meaningful neutral red/green cases and actual Windows journaled exchange/fold-safe probes passed | Registered portable Ajv/Python gate |
| Managed Python test route | Portable owner, eight bound-tool cases, nine managed-result cases, and UTF-16 boundaries passed; independent source review complete | Managed uv resolution and actual Ajv/Python execution |
| Publications | All four localized PDF producers required by four consumers; neutral graph contract passed; both runbook tables corrected | Ajv suite and actual PDF transport |
| Native setup and browser output | Focused command, checksum, compiler, temporary-path, argument, and artifact cases passed; maximum supported artifact path 254 UTF-16 units | Managed Ajv/Noble/Vitest suites and real browser artifacts |
| Proctor/course gates | Registered routes, prerequisites, and Windows process lifecycle audited | Build, typecheck, unit/check, and isolated development/rehearsal E2E |
| Learner journeys | Existing tests identified for identity, scoring, reload, second device, presence, short interruption, and phone interaction | Actual browser execution and four PDF GET checks |
| Running normal stack | Authored development target uses site port 6061 and proctor port 8791 | Start after gates, verify URLs, and leave the owned stack running |

The website/server runtime and complete end-to-end result are not yet verified. The existing Windows UAC dialog requires human action; see [UI handoff](nov05-ui-handoff.md). No public deployment is claimed.

The independent finite path audit covered 72 owned source files (maximum 140 UTF-16 units), 62 ticket-root inputs/reports (maximum 145), and eight authored cold fixture inputs (maximum 209), with no violation in that set. Course execution separately checked 240 generated entries and removed one stale initial WinGet metadata file at 257 units while preserving live installer files, aliases, and processes. These bounded checks do not describe unrelated concurrent changes.

The lengthy replay was investigated without intervention. Raw replay activity comes from repeated producer progress calls, although identical labels cannot identify a receipt or pass. The implementation repeats immutable-input fences around scope operations. Current repository diagnostics found no queued waiters, lock cycles, or violations; a short low-CPU process sample alone did not establish a hang. Subsequent logs showed an owned Bun child.

Details: [bootstrap](nov05-bootstrap.md), [oracle run plan](nov05-oracles.md), [course](nov05-course-audit.md), [preflight](nov05-preflight.md), [Source fence](dependency-fence.md), [semantic work](dependency-stage-work.md), [independent audit](nov05-web-audit.md), and [state ledger](nov05-state.md).

Full release readiness also requires the existing actual Linux/x64 image, Docker image/stack/bundle and release deploy-check gates. The current host has no installed WSL and no Docker client; no configured existing Linux worker has been proven. See [Linux environment inventory](nov05-linux-environment.md). The default devcontainer uses Ubuntu 24.04 and its full lifecycle modifies Git, so it will not be launched unchanged. A Store distro name alone does not prove the Debian 12 production ABI baseline. Selective prerequisite preparation is in progress.

The source action audit distinguishes local readiness from remote changes: course :publish only stages local Pages artifacts, :deploy-check validates local release artifacts and disposable stacks, and :docker-stack-bundle creates a local offline bundle. Registry pushes, Pages deployment, and production-host Compose startup are separate remote actions, which have not been performed. No Git mutation will be used to manufacture a clean-commit publication label.
