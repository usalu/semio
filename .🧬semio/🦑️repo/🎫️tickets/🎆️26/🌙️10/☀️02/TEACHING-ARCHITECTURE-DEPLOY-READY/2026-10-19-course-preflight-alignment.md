# Course Preflight Alignment After Shared Platform Rewrite

## Measured Boundary

The private captured Debian 12 x64 OS/Bun route completed the newly authored bare generic `bun repo preflight`, with genuine original authority and actual directory exchange/case probes. Its report is `2026-10-19-builder-os.md`. The new generic policy checks architectures x64/arm64, 8 GiB free, cc/pkg-config and Linux directory exchange/case semantics. It deliberately does not carry an owner scope or course ABI policy. The earlier owner-scoped Debian/loop result belongs to a different dated Source snapshot and must not be relabeled as current policy.

## Pre-Alignment Source Mismatches

These observations preceded the completed alignment below.

- `.github/workflows/architecture-course.yml` lines 78, 107 and 150 still invoke `bun repo preflight --scope @teaching/architecture-course`, which the current original entry demonstrably rejects. Its opening comments still state 72 GiB and seven loops, and step names claim capabilities/ABI checks that generic preflight no longer performs.
- Course `📦️packages/🟦️typescript/📜️script.ts` line 28 already calls `runBuildPreflightV1(...,["preflight"],...)`; it has been adapted by another contributor. Its docstring still describes course-authored release requirements, while its body currently performs only the generic check.
- Course `📦️packages/🟦️typescript/📋️project.json` retains `metadata.semio.preflight` with Debian 12, x64, 72 GiB, loop count and the old filesystem/Docker tools. The current generic preflight does not read that project metadata, so retaining it alone is not enforcement.
- Course `README.md` lines 396–409 still describe that metadata as the owner's actual enforced preflight, requiring CAP_SYS_ADMIN, loop-control/ext4 mounts, seven loops and 72 GiB. The runner and cross-architecture sections still require Debian 12 x64 and publish linux/amd64. These statements must be reconciled with the new directory-backed storage owner rather than removed or silently attributed to generic preflight.
- Course `🚀️deploy/📦️image/🟦️.ts` currently requires a retained native Linux release executable and accepts producer architectures x64 or arm64. It checks ELF64 little-endian and matching machine 62/183 before packaging. This is broader than the current documented amd64-only deployment contract. Its JSON-form runtime `proctor check` remains the actual pinned-runtime ABI/content validation during image construction. No such image construction or runtime check ran in the OS probe.

## Legitimate Follow-Up Boundaries

The registered course PreflightScript is an existing domain owner that can carry genuine course-specific release requirements in addition to generic host preflight. The image owner is an existing authority that checks the real retained producer and actual runtime ABI/content. The workflow needs a valid generic early check before Store opening, and any course-specific check must be described and invoked through its actual owner before claiming the Debian/x64/runtime contract. A generic success alone does not prove those requirements.

No compatibility flags, restored obsolete generic scope API, spoofed OS facts, removed image/ELF checks or undocumented raw compiler route is proposed. The proper final image platform contract (current documented amd64 versus the new owner accepting both native Linux architectures) requires explicit coordination with the shared image/platform change. This worker has not edited these current course/preflight/platform files during the OS probe; parent was notified of their exact state for the next aligned fix.

## Completed Alignment And Test Evidence

The actual public `buildCourseImage` entry still requires process platform linux and architecture x64. The lower-level context's matching ELF62/183 support does not broaden that public command. The bundle still selects linux/amd64 Caddy, and the final pinned Debian runtime executes the real JSON-form proctor/course check. Those guards were inspected and preserved unchanged. Debian 12 x64 remains the configured runner baseline for this course release; generic preflight does not claim to enforce its ABI.

Changes now completed:

- `.github/workflows/architecture-course.yml`: all three early calls use bare `bun repo preflight`; their names and workflow comments describe generic tools/capacity/filesystem admission. Existing genuine release mode, selected runner, pinned setup actions and original controlled command wrappers remain intact.
- Course `README.md`: describes the actual 8 GiB generic Linux architecture/tool/exchange/case checks and the distinct public Linux x64/ELF/pinned-runtime validation; removes obsolete loop/ext4/capability/72 GiB claims; retains actual per-partition capacity/physical-host considerations and legitimate CI Chromium package preparation.
- Course project `📋️project.json`: removes dead `metadata.semio.preflight`, which no current owner reads. The registered dashboard preflight target remains.
- Course `📦️packages/🟦️typescript/📜️script.ts`: corrects the preflight owner's docstring to describe its real generic behavior.
- Readiness fixture `🧫️fixtures/🚀️readiness/🔣️.json`: updates all expected CI calls and adds portable generic/runtime guard expectations.
- New `🧪️tests/🩺️preflight/🟦️.ts`: checks actual YAML commands against the real pure argument-parser function, compares authored policy against independent Ajv, inspects the dashboard owner with TypeScript, verifies truthful documentation and confirms retained actual image/ELF/runtime guards.
- Existing deployment suite `🧪️tests/🧪️deploy/🟦️.ts`: integrates that focused helper into the normal registered course test suite. Its explicit Vitest include list loads this helper through the deployment suite rather than treating the helper as an empty suite.

The new source test first failed on the actual obsolete scoped workflow arguments. After the changes it passed with console evidence. Because the real course Vitest runtime is Node, the helper executes the actual pure parser function from its TypeScript source instead of importing the whole generic preflight module's Bun FFI dependency into Node. The focused helper then passed on both Bun and actual Node (`node --experimental-strip-types`), with no build, dependency preparation, compiler process, Store or OS command. Logs are `🗑️generated/2026-10-19-course-preflight-red.log`, `...-green.log`, `...-final.log` and `...-node.log`. This is scoped source evidence; full managed course/CI and image/learner gates have not run.

Runtime's Source capture worker was notified when edits completed and again after the Node-only helper correction. It retains its real full hash fence and will record or refuse any later divergence rather than silently omit this change. No further authored source edits are planned by this worker during private OS extension.
