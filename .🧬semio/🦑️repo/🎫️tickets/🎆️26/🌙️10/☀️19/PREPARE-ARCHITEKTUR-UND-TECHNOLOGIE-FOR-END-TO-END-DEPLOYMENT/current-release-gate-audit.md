# Current Release Gate Audit

Observed 2026-10-19 in the shared checkout. Read-only source audit; only this report was written. Root AGENTS.md applies; no nested teaching AGENTS.md was found. No Git mutation, native/compiler/store allocation, image operation or repeated test run was performed. The concurrent Rust companion/unit repairs are excluded.

## P2: Editor Launch Configuration Is Currently Absent

The actual `C:/git/semio/.vscode` directory and `.vscode/launch.json` are missing. Direct Get-Content and directory enumeration confirmed absence; `git status --short -- .vscode` reports staged deletion of launch.json along with the other editor files. This is a concrete current checkout regression, although attribution is outside this audit.

`🎓️teaching/🏛️architecture/📚️course/🧪️tests/🚀️launch/🟦️.ts:14` unconditionally reads that path at module initialization. Consequently the current checkout cannot expose its sixteen course commands through the required editor launcher, and the launch suite cannot collect against these bytes. The earlier root diagnostic of 12 files / 598 passing tests was supplied by the parent and predates this observation; it is not evidence that the currently missing launch file exists. Restore the owned editor configuration with its generic entries and course contract before final verification, coordinating with whoever owns this concurrent change.

## Other Inspected Release Sources

The actual course `📦️packages/🟦️typescript/📋️project.json` defines sixteen targets, routes each through its local `📜️script.ts`, disables cache for release/native checks and declares the native Linux x64 Debian 12 preflight. The router contains all sixteen registrations. Release site publication preserves original release-mode authority; the deployment gate routes typechecking, original build authority and serial end-to-end orchestration.

The runtime-only Dockerfile receives a finite authored image context, uses a digest-pinned Debian 12 distroless runtime and validates the selected course through the real proctor executable. The checked-in Compose stack keeps the proctor behind Caddy, persists SQLite separately, uses digest-pinned Caddy, bounds logs/resources and applies non-root/read-only/capability restrictions. The stack validator resolves actual Compose configuration, validates the extracted Caddyfile with the image, then checks runtime health, API/CORS/presence, backup/restore and certificates. These are inspected executable checks, not executed Docker proof.

The GitHub workflow requires the configured runner before release jobs, preserves release mode and preflights before dependency allocation. Its site output matches the course release Pages staging path. Proctor publication performs image/stack validation before push. The runner variable and zero registered runners remain the parent's acknowledged external prerequisite; this audit did not invent an alternate runner or perform a remote operation.

The course manifest declares its existing development oracles and runtime workspace sources, including tar-stream, AJV, yaml, picomatch, resolve.exports, terser and public styling/process sources. The teaching workspace roster and bun.lock are owned by their existing dependency scope; the ticket's dependency-lock.md records authored field parity. Physical installation and genuine native compilation remain unproven. This audit did not claim a fresh frozen-install result.

No additional concrete P1/P2 source defect was identified in those inspected release routing, Docker/Compose or workflow paths. This is a scoped static audit, not a guarantee about the excluded in-flight Rust repairs, native build, current dependency materialization, live production service or deployment. The parent-reported React diagnostic (38 files / 1032 tests) likewise does not establish native release readiness.

## Launch Finding Resolved in Current Shared State

Root restored the course-owned launch entries during this audit. A fresh direct read parses `.vscode/launch.json` with exactly sixteen total entries, all sixteen belonging to the course. Direct comparison against the neutral fixture confirms one entry per target, exact names/groups/orders and all required release modes. There is no present claim of twenty-eight entries or twelve restored generic commands. Earlier missing-file observations remain historical only; rerunning the course suite against the restored current state is root-owned.

## Output Boundaries and Public Course Aliases

A bounded recursive enumeration of this ticket found no file path longer than 256 characters. This audit created only the two Markdown reports inside the existing ticket; no generated log or copied source payload was created. Root owns cleanup of its shared generated diagnostics.

The current `@semio-tech/course` package exports its existing TypeScript source entry. All six explicit `@semio-tech/course-react` targets exist on disk: root, app, CSS, stylesheet, run module and results module. This confirms authored target reachability by direct manifest/file comparison, without claiming frozen dependency materialization or running a compiler. The source ui-axes public reexport fix remains companionHigh-owned; no new generated type requirement is inferred.
