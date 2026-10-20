# Native Course Product And Artifact Target Audit

Read-only source/manifest audit; no command ran. The old private Linux plan is not assumed to be a user requirement. The requirement that remains intrinsic to current product code is Linux x64 image production.

## Host And Artifact Conclusion

Native `repo run @teaching/architecture-course:deploy-check:release` on macOS can reach host typecheck/course/site steps, but cannot finish: buildCourseImage explicitly refuses process.platform!==linux or process.arch!==x64 before compiling or Docker building. Docker Desktop availability does not change that host gate. `@teaching/proctor:build:release` is host-native; product receipt validation requires host platform/architecture, so its macOS output cannot substitute. courseImageExecutable independently requires a genuine Linux release artifact and validates ELF64 little-endian magic and machine62 for x64 (183 for arm64); image build itself allows x64 only. No authored macOS cross-compilation or Linux delegation appears in these targets. A native Linux x64 execution environment is therefore necessary for the current image producer, while the particular frozen private environment is not prescribed by these sources.

## Declared Native Targets And Gates

Owner: `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`, @teaching/architecture-course. Native commands use repo run <project>:<target> and release configuration. build/publish/test-e2e depend on assets build and styling tokens; typecheck additionally depends on framework/UI generators; deploy-check declares typecheck plus assets/tokens and internally typechecks again. Pure test/typecheck/check targets emit verdicts; ordinary build is Vite output and is not the CDN artifact verifier.

The deploy-check implementation has eight ordered gates: deployment config drift; sources typechecked; course validated by acquired Rust proctor; release site built and CDN artifact verified; original native Linux x64 release proctor acquired/course-checked and packaged into image; image check; full Caddy stack check; browser end-to-end gate (both dev/rehearsal topologies serially). Release original-mode control is required. It stops at first failure and uploads/pushes nothing. It does not call the Vitest test target or proctor unit-test target; those remain distinct test gates for a full test mandate.

Image checks assert runtime labels/version/revision/time, nonroot user, health probe, expected ports/data volume, no shell, exact embedded stack files; run read-only/no capabilities/no-new-privileges; readiness, cleartext refusal, allowed/foreign-origin contract, body limit, two-learner presence, graceful drain/exit0 and database WAL folding. Stack check validates the extracted Compose/Caddy config and hardened/limited/log-rotated services, TLS/security headers and forwarded-proto behavior, origin/body/presence, serving backup readback, replacement availability/persistence, restore refusal while serving and correctness after stopped restore, requested learner erasure, and supplied-certificate override. Checks own throw-away resources.

The image producer uses a retained authenticated proctor artifact and bounded runtime tar context (Dockerfile, content, stack files, data directory); Docker is internal behind owned process APIs. Source revision/version/time become OCI labels. Native macOS proctor build cannot satisfy this packaging contract.

## Offline Bundle And Site

`publish:release` builds/verifies the CDN artifact and stages package dist/release/pages/course; it does not upload. Verification includes sealed document/CSP, entrypoints, hashed assets, no development leftovers and size budget, with baked proctor origin.

`docker-stack-bundle --tag <tag> --out <directory>` requires an already existing proctor image and exact stack-file match to checkout. It does not build the image and does not execute deploy-check. It stages Compose/Caddyfile, certificates directory, .env pinned tag, README and proctor-image.tar containing proctor plus exact pinned Caddy image (and portable local digest tag); staged Caddy pull_policy is never. It may pull missing Caddy linux/amd64 during staging; resulting bundle supports offline loading/start. The bundle README requires Linux/amd64 Docker Engine25+/Compose2.24+. Bundle creation has no macOS host refusal, so macOS with a suitable preexisting Linux image could stage it, but this does not prove acceptance gates. The CDN site is separate from this proctor-host directory and must also be retained for complete delivery.

`docker-image-publish:release` is separate registry publication, requires clean exact commit revision, reruns image/stack checks, confirms tag identity and pushes rollback/version/latest. It is unnecessary to make the offline redeploy artifact and should not be inferred as authorized external publication.

Primary inspected sources: course package manifest/router; deploy/⚖️readiness,🐳️docker-image,📦️image,🎁️stack-bundle,🌐️site,🩺️docker-image-check,🧱️docker-stack; proctor package manifest and 🏗️bootstrap/🧾️product. Source inspection does not prove any current product pass or artifact exists.
