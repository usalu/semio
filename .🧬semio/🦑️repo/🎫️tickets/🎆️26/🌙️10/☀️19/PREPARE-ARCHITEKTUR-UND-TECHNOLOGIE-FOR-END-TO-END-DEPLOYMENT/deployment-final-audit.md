# Deployment Final Audit

Read-only audit on 2026-10-19 of the current architecture-course deployment sources. Code was under concurrent edits; locations refer to the observed source. No build, Docker runtime test or production deployment was executed by this audit. Only this report was written.

## Findings

### P2: Commit rollback tags are mutable despite the documented immutable contract

Location: `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🟦️.ts:496-502` (`publishCourseImage`).

The publisher derives `sha-<first 12 commit characters>` and unconditionally tags and pushes the current local image to it. It never checks whether that reference already exists in the registry or whether its digest differs. A second build of the same source commit can therefore replace the rollback image while the docstring and workflow promise that the tag is never moved. Commit identity alone does not establish image-byte identity, particularly for compiler/build metadata or changed external producer inputs. Refuse an existing differing immutable reference, or publish and document content-addressed digest rollback references. Keep version/latest updates separate.

### P2: Offline bundle mixes target architectures for an admitted ARM build

Locations: `🚀️deploy/🟦️.ts:456`, `:472`, `:1036`, `:1078` relative to the same course directory.

`buildCourseImage` explicitly accepts native Linux arm64 and packages its arm64 executable, but `stageCourseStackBundle` pulls Caddy with `--platform linux/amd64` when absent, saves that alongside the proctor, and emits instructions declaring the host to be linux/amd64. It never checks the proctor platform. Thus an arm64 operator can successfully stage a bundle whose proctor cannot run on the documented amd64 host without emulation, while an arm64 destination receives the wrong Caddy variant. Derive one supported target platform from the inspected proctor image, select matching Caddy, and emit matching host instructions; alternatively reject arm64 bundle inputs explicitly if offline deployment is intentionally amd64 only.

## Consistency Observations

The observed Dockerfile is runtime-only and receives a finite tar of the authenticated executable, selected course JSON, stack configuration and writable data directory. It validates the selected course using the executable inside the final runtime. The readiness ordering includes drift, typecheck, course validation, site publication, image build, direct image checks, TLS proxy stack checks, and E2E when registered. The workflow uses the configured canonical runner with release mode and does not silently substitute a hosted compiler runner. These are source observations, not runtime success claims.

The compose header still contains an obsolete direct `docker build` example against a clone even though the runbook states that the checkout is not a supported Docker build context (`compose.yaml:14-16`, course `README.md:238`). This is a documentation correction rather than a separate blocking finding.

The known native Linux release execution limitation is outside these findings: this audit does not establish current production runtime readiness.
