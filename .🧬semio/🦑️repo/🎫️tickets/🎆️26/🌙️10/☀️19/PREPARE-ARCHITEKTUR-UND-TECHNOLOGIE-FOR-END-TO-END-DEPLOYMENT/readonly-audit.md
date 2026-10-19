# Current Course Read-Only Audit

Observed 2026-10-19. Source/file inspection only: no build, test, browser, allocation, distro startup or external mutation performed. Findings describe the observed tree and may be superseded by concurrent fixes.

## Deployment Conclusion

Current deployability is unproved and the observed router/deploy sources contain immediate loading/type/runtime blockers. Existing tests and Oct14 generated assets are authored or historical evidence, not a current successful release.

## Concrete Blockers

- Course package `📜️script.ts` imports absent typecheck and Vitest paths through `🧰️framework/📦️packages/🟦️typescript/…` (lines 3,11; literal path existence checked).
- That router imports `checkCourse`, `publishCourseSite`, `buildCourseImage`, `checkCourseDeployment`, and `courseReleaseBuildPlan` from deploy, but the inspected deploy file exports none of these. Its corresponding names remain `checkQuizCatalog`, `publishQuizSite`, `buildQuizImage`, `checkQuizDeployment`.
- Router imports `runCourseEndToEnd`, while E2E exports `runQuizEndToEnd` (line322). Calls in router still use the latter without importing it.
- Router uses `join` without importing `node:path`, and `buildViteArtifact` without an import/definition.
- Deploy module imports `scriptCompilerAdmissionControlV1` twice (lines1,48).
- Deploy readiness step list uses `course valid in the Rust core`; dispatch object uses `catalog valid in the Rust core` (lines1078,1104). The selected key therefore has no callable handler.
- Deploy readiness calls `checkQuizImage` and `checkQuizStack`; definitions are now `checkCourseImage` (line701) and `checkCourseStack` (line843).

## Authored Experience and Gates

- Apex site: `architektur-und-technologie.de`; proctor: `semio.iek.uni-hannover.de`.
- Course has five energy quizzes (physics, ventilation, heating, cooling, demand). Parametrics, Grasshopper, Excel, BIM and Optimization topics currently contain no quizzes.
- Source describes English/German onboarding, anonymous/pseudonym/name identities, four challenge levels, local play/outbox recovery, badges, live leaderboard/presence, optional pets and legal footer links.
- Playwright config authors boot, desktop, phone (375×812), adaptive layout, presence, shortage, proctor-away, proctor faults and pets projects; one worker, zero retries; source tests reject unexpected console/page/request errors. This audit did not execute these journeys.
- `deploy-check` intends drift/typecheck/Rust validation/CDN artifact/image/image-runtime/stack/E2E gates. Current source inconsistencies prevent treating that definition as proof of execution.

## CI and Operator Evidence Still Required

- `.github/workflows/architecture-course.yml` is manual only and gates release jobs on the repository event's default branch.
- Mandatory `ARCHITECTURE_COURSE_RUNNER` variable provides the runner label, with no fallback. Shared unit/typecheck/rehearsal job gates both Pages and GHCR publication.
- Course preflight metadata requires Linux x64 Debian12, minimum available storage 77,309,411,328 bytes (72GiB), seven free loops, and fallocate/mkfs.ext4/mount/umount/losetup/cc/pkg-config/docker. README says native capability/storage/ABI proof remains necessary.
- Repository variable value, actual runner readiness, GitHub Pages domain/account verification, GHCR package public state, DNS/TLS and live learner behavior were not queried here.
- `site.legal` is `{}`. Source treats missing imprint/privacy notice as warnings; no legal URLs are authored.
- Retained package `dist/prototype` files observed dated Oct14; its CNAME contains the apex host. Those assets are not a build of the Oct19 source and establish no current runtime health.

## Evidence Files

- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📜️script.ts`
- `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🎭️e2e/🎚️config/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🔣️.json`
- `🎓️teaching/🏛️architecture/📚️course/README.md`
- `.github/workflows/architecture-course.yml`

## Follow-Up API and Dependency Audit

- Current repo-lib public dev entry is `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/📦️packages/🟦️typescript/🎮️playground/🟦️.ts`: exports `withViteArtifactV1` at line4 and `buildViteArtifact` at line12. Package exports `./dev` to that entry.
- Vite implementation `…/📚️library/⚡️caching/🌐️vite/🟦️.ts:35` defines `ViteArtifactBuild`: mandatory invocation, control, root, workspace, config, owner; optional mode/environment/artifactInputs/signal/timeouts. `buildViteArtifact` line52 checks exact issued control and original caller mode/workspace.
- Public Cargo entry `…/📚️library/📦️packages/🟦️typescript/🦀️cargo/🟦️.ts:3` exports `workspaceCargoVersion`, implemented at `…/⚡️caching/📦️artifacts/🏗️native-build/🟦️.ts:22`.
- Process routing package publicly exports `requireIssuedScriptInvocationV1`; deploy uses it line164 without import. Deploy also uses `readBuildMode` lines415/429 without importing it.
- `checkQuizCatalog` calls an obsolete `buildProctor` signature and undefined `CATALOG`. Current bootstrap signature line245 is `(repoRoot, invocation, env = process.env, signal?, requestedMode?) => Promise<ProctorArtifact>`; artifact offers `.launch(args, env?, log?)` and `.withExecutable(callback)` under custody.
- Inspected `🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts` buildProctor body is itself malformed around245–267: undefined `profile`; a preparation expression with `env: launch.cwd, env: launch.sources:`; unmatched `finally`/closures and absent definitions of `metadata`/`built`. This is a current dependency source blocker requiring repair before any course runtime claim.
