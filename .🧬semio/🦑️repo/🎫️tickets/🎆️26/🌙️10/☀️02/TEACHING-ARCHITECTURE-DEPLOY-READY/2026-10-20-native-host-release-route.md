# Current Host Release Route Review

## Result

The current authoritative worktree can be tested through the existing native host repo run targets; it need not retain the frozen private26 implementation as the only Source route. However, current macOS ARM64 host execution cannot complete the full redeployable Linux x64 release through these course/proctor targets as currently authored. The decisive gap is an explicit Linux x64 process-platform guard in buildCourseImage, with no compiler/container delegation in the inspected target owners. A legitimate native Linux x64 repo execution owner is still required for final image production and its verified artifact cohort.

No command, builder observation, native recovery, build, Source edit or artifact production ran in this review. Only exact gate manifests, their routers, deployment platform/product ownership and the retained artifact contract were read. No broad scan was performed. Root dry2 remains Root-owned.

## Supported Native Commands

Use the current usable native repo frontend at the authoritative worktree root. The ordinary entry is repo run <project>:<target> --configuration=release. The existing exact manifests declare these test owners:

- @semio-tech/course:test
- @semio-tech/pets:test
- @semio-tech/course-react:test
- @semio-tech/pets-react:test
- @teaching/architecture-course:test
- @semio-tech/course-rs:test
- @semio-tech/pets-rs:test
- @teaching/proctor:test

@semio-tech/course-tests-runs-rs has its metadata-only project manifest and concrete Cargo owner; its test is inferred by the normal native graph rather than authored in that manifest. Preserve that separate ninth suite and verify the current inferred target through normal native graph admission before execution. No custom shim is needed or proposed.

The three additional product typechecks are @semio-tech/course-react:typecheck, @semio-tech/pets:typecheck and @semio-tech/pets-react:typecheck. Course React typecheck retains its6generator/assets prerequisites; architecture-course:typecheck has those same declared prerequisites and is scheduled by deploy-check. Their manifest commands are ordinary owned script targets and have no Linux/container delegation metadata. Host runs are structurally supported, but no actual test/typecheck pass is established here.

repo run @teaching/proctor:check --configuration=release remains the required native material validation. The proctor router uses acquire/runProctor under issued build authority. proctorBuildPlan produces --package teaching-proctor --bin proctor --profile <mode>, and the immutable executable reports process.platform/process.arch. The cached product receipt is platform/architecture specific. A successful macOS proctor check validates current material against the host-native Rust implementation; it is not a Linux executable artifact.

repo run @teaching/architecture-course:ci-browser-prepare --configuration=release exists and invokes the owned Playwright install-deps chromium command. Its documented purpose is Debian Chromium system preparation. The E2E owner separately selects exact installed Playwright Chromium/headless-shell revisions and installs missing browser binaries into repository tool storage, then exercises dev and release-rehearsal topologies. A host browser pass is useful current-worktree evidence; it does not supply Linux image production or Linux native tests.

## Hard Platform Gap

In 🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🐳️docker-image/🟦️.ts, buildCourseImage first requires original release mode and then refuses unless process.platform is linux AND process.arch is x64, with Image production requires the canonical release command on the authored native Linux x64 runner. This occurs before Docker server use and acquireProctor.

The same function acquires the original native release proctor, launches its check against actual course material, retains its executable and supplies a finite runtime context to docker build --platform linux/amd64. The platform flag packages/runs the already-authenticated native executable; it does not cross-compile a macOS proctor. Dockerfile is deliberately runtime-only, starting from a pinned distroless nonroot image, copying proctor/content/deploy/data and running the actual proctor check. No source/compiler/toolchain stage exists.

The exact architecture course manifest declares local repo:script commands, release-mode dashboard parameters and ordinary prerequisites. It has no container/remote Linux runner selector. Its deploy-check router directly calls checkCourseDeployment and that body's image-build step directly invokes buildCourseImage. Consequently native host repo run @teaching/architecture-course:deploy-check --configuration=release -- --tag <fresh-current-tag> can run early checks but must refuse at image production on this host. Starting that aggregate merely to rediscover the known platform guard would not complete the deliverable.

The lower course docker()/dockerOk() functions own shell-free Docker argv and environment merging, and image packaging uses the issued bounded command control. They are runtime/image tool ports, not a Linux native compiler execution service. No inspected owner provides automatic whole-repository delegation from the macOS host to Linux x64. This report makes no claim about an unconfigured generic remote backend; such a backend has not been established as this deployment owner's supported execution route.

## Offline Deliverable And Cohort

On a legitimate native Linux x64 execution owner with current Source and actual Docker/Compose/browser availability, the existing final route is:

~~~text
repo run @teaching/proctor:check --configuration=release
repo run @teaching/architecture-course:ci-browser-prepare --configuration=release
repo run @teaching/architecture-course:deploy-check --configuration=release -- --tag <fresh-current-tag>
repo run @teaching/architecture-course:docker-stack-bundle --configuration=release -- --tag <same-current-tag>
~~~

These supplement, rather than replace, the9individual suites and3product typechecks. deploy-check sequences drift, architecture typecheck, Rust material check, production-origin CDN site staging, image production, image runtime check, stack runtime check and both real browser topologies. It uploads/pushes nothing.

The separate same-tag stack bundle extracts and compares stack files from the actual image, pins a local digest tag for the exact authored Caddy image, writes pull_policy never, .env, certificates directory and operator README, and saves both services into proctor-image.tar. It may pull the exact linux/amd64 Caddy digest if absent; it is offline-redeployable after staging, not necessarily network-free during construction. A previously valid Linux image could be bundled from a different host with Docker access, but that would not create or validate the missing new Linux image/cohort.

Site output is the course package dist/release/pages/course; default offline stack output is dist/proctor. A move to the authoritative current worktree must produce fresh evidence and a coherent new tag/source identity. Do not combine current host passing tests with a frozen26/private image or assume the old planned architecture-20261019-8579fa82486d tag identifies newly changed Source. Record actual Source revision/dirty identity, proctor executable identity, image IDs/labels, matching tag, runtime/browser gates and final file/hash rosters. The retained artifact contract's production origin and export limits still apply to the final reviewable artifacts.

No compatibility runner, guest native-binary copy, Docker compiler shim or raw external command is proposed. Root should choose a legitimate native Linux x64 repo owner or an explicitly designed platform execution feature if no such owner is currently usable; the inspected host targets alone do not close this platform gap.
