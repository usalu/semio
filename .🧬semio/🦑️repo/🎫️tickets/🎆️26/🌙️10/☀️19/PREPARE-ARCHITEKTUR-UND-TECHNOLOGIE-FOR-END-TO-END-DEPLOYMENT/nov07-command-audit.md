# November 7 Command Registration Audit

Read-only configuration and documentation inspection; no IDE, dashboard, build, test or process operation was used. Existing ticket metadata requests executable-command launch registration. Its historical [release audit](current-release-gate-audit.md) records sixteen course entries at an earlier moment, but that is not current evidence.

At this audit, direct directory read of `D:/semio/.vscode` returned ENOENT, so its previously reported `launch.json` cannot currently be inspected for course/proctor entries, ordering, grouping or naming. Read-only `repo tools exec git -- ls-files -- .vscode/launch.json 🎓️teaching/🏛️architecture/📚️course/**/launch.json 🎓️teaching/🛂️proctor/**/launch.json` produced no tracked matches; an unrestricted `*launch.json` query produced many ticket artifacts and was truncated, so it cannot prove global absence. This does not establish whether another untracked catalog exists or why the root file is absent. In particular, registration of the new `test-launcher` in an actual launch catalog is unverified. Do not infer a need to restore a historical IDE file from this observation alone.

The current course target manifest `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json` declares `preflight`, `ci-browser-prepare`, `dev`, `dev-site`, `build`, `publish`, `test`, `test-e2e`, `typecheck`, `check`, five docker targets, `deploy-check`, four assignment producers and `test-assignment`. The proctor manifest `🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json` declares `build`, `test`, `test-launcher`, `dev`, `check`, `rebuild`, `health`, `backup`, `restore`, `erase`, `prune`, `capacity` and `serve-release`. These are actual target registrations, not evidence that every command appears in a separate launch catalog or has executed.

The current manifest still has all four PDF producers: `build-parametrization-en`, `build-parametrization-de`, `build-typology-en`, and `build-typology-de`. Each directly depends on Print `fonts`, `deps-tectonic`, `deps-tex`, and `generate`; course `build`, `dev` and `test-e2e` depend on all four. Dev and E2E additionally depend on proctor build.

The current course README explicitly gives both browser calls:

```text
repo run @teaching/architecture-course:test-e2e -- --topology dev
repo run @teaching/architecture-course:test-e2e --configuration=release -- --topology rehearsal
```

The `test-e2e` manifest forwards arguments and declares `both`, `dev`, and `rehearsal` topology choices. These are two invocations of the same target, not separate target names. Neither declarations nor this audit establish execution, generated PDFs, browser acceptance or a running stack.

## Focused Convention Follow-up

The completed untruncated tracked-path query `repo tools exec git -- ls-files -- ':(glob)**/launch.json' 'launch.json' ':(exclude).🧬semio/**' ':(exclude)**/.cache/**'` returned zero paths, exit 0. It covers tracked root and nested files with that exact basename outside ticket and cache exclusions; it says nothing about ignored or untracked files or other catalog names.

A completed untruncated filename-only `repo tools exec git -- grep -l -i -e launch.json` over the repo build library, dashboard module and caching library returned five files, all under test or fixture paths. Dashboard `🧪️tests/🧭️authority/🟦️.ts` explicitly asserts root `.vscode/launch.json` and `.vscode/🧩️launch.seed.jsonc` absent and root `generate-launch` and `generate-launch-check` targets undefined, while validating owner project declarations against the registry. This is source evidence of the present intended projection policy, not an executed passing test. The dashboard cold-start test's filename is a generated receipt; its authority fixture contains scenario data. Caching's prepared-worker asset-publication test still reads both old editor files, and its containers lifecycle fixture contains a path reference. That test is a conflicting stale reference under this policy, not evidence that a live generator exists. No non-test generator/reference to the literal `launch.json` appeared in these three scoped tracked-source trees. Other modules, untracked content, differently named generators, and dynamic construction remain outside this check.

The bounded evidence supports current project manifests/registry as the declared command surface and does not establish any current native launch.json catalog or a requirement to recreate one. This scopes and corrects the earlier catalog finding; it does not settle all repository-wide configuration.
