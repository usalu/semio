# Tool Configuration Ownership — Independent Audit

> **Current status — accepted.** The final isolated, cache-skipped registered route is 7/131 in 10.03 seconds (Nx 10.4 seconds), after the final semantic Vite-runner selector repair. Earlier 7/127 and pending-registration wording below is historical evidence retained for chronology; [Final scoped acceptance](#final-scoped-acceptance) is authoritative.

## Scope

This read-only review covers the selected 13 legacy configuration leaves, their 11 anonymous semantic owners, and the two retired Tailwind package re-exports. It does not execute Vite applications, Playwright, browser sessions, ESLint over the repository, dependency-cruiser, or the VS Code extension host.

## Current structural evidence

The portable fixture identifies 11 unique owner paths: four builder-owned Vite configurations, one WGPU server configuration, a Demonstrator E2E configuration, a Tailwind styling configuration, two ESLint configurations, a dependency-boundary configuration, and a VS Code extension-test configuration. Each owner is an anonymous implementation leaf outside `📦️packages`; the five Vite owners belong respectively to their build/server concerns rather than the package that supplies an effective `root`. This is the correct ownership boundary.

The source roles match their taxonomy contexts:

- Vite files own product builder or server configuration.
- The Demonstrator Playwright file is an E2E test configuration and retains its `PLAYWRIGHT_BASE_URL`/Nx launch guard.
- Tailwind owns styling theme/build configuration; the package re-export leaves are removed.
- Root and React ESLint configurations own lint policy.
- Dependency-cruiser owns repository dependency boundaries.
- VS Code test CLI configuration belongs to the extension test concern.

Current direct consumers already include product script routers, project manifests, `.vscode/settings.json` for root ESLint, and the VS Code package test router. These must be represented as source-data and registered-input relationships, not inferred solely from physical relocation.

## Repaired source-data closure

The initial ownership test had only AJV/map and owner/predecessor/shim checks. The repaired live test now binds 19 exact command, editor, test and fixture source-data consumers; eight external project-input bindings; five Vite native-loader configurations; the protected Playwright list route; Tailwind; both ESLint configurations; dependency-cruiser; and VS Code test-config loading. It asserts the two retired Tailwind shims and the removal of five fixed taxonomy exemptions. The package command, Nx target, package script, seed and derived launch registrations are explicit.

## Required final evidence

- Exact old paths absent and the two Tailwind re-exports absent.
- Every current consumer (package router, Nx manifest, VS Code selector, lint and dependency-boundary caller) selects the semantic owner.
- The registered ownership target declares every file it reads as an input and has a launch route.
- Tool loaders are tested only at their safe exported configuration boundary. No result should imply a Vite application build, Playwright/browser run, VS Code host launch, or repository-wide lint/graph success.


## Independent static control

I independently read the current fixture and project data without executing a Vite app, Playwright browser run, lint sweep, dependency graph, or VS Code extension host. The ticket-local control checked all 11 owner existence/predecessor absences, both shim absences, 19 required consumer tokens, eight exact `namedInputs.default` bindings, and the registered target/launch command. It produced zero failures: `🗑️generated/terra-tool-configuration-audit/result.json`.

Historical checkpoint: the executor’s focused package gate was green at 7/127 in 6.35 seconds. Its then-pending registered result is superseded by the accepted isolated Nx evidence in the final section.

## Final scoped acceptance

I independently inspected the current fixture, schema, ownership test, package router, project target, named inputs, launch seed and derived launch. My ticket-local static control found zero failures across all 11 semantic owners, the 19 exact command/editor/test/source-data consumers, eight owner input bindings, the two retired Tailwind re-exports, the target, and both launch registrations.

The executor then ran the same current target in an isolated, cache-skipped Nx environment: `@semio-tech/repo-lib:test-tool-configuration-ownership` exited successfully with 7 tests and 127 assertions in 4.41 seconds; Nx completed in 4.8 seconds. Its installed-tool boundary suite loads all five Vite owners using Vite's native loader at the actual sixth argument position, and separately exercises the protected Playwright list route, Tailwind import, two ESLint configuration readers, dependency-cruiser configuration, and VS Code test-config loader. The earlier bundled-loader result is retained only as historical evidence; the final native-loader result is the acceptance evidence.

I accept this bounded extraction. It proves configuration selection, source-data consumers, registered cache-input bindings, anonymous semantic ownership, and the listed safe loader boundaries. It does not claim that a Vite product build, Playwright browser session, VS Code extension host, repository lint, or dependency-cruiser graph completed.

## Final selector delta

The final owner closure repaired the shared `runViteBunxDev` fallback so configuration is required and all three callers provide a semantic owner, including the OS runtime configuration. The portable map consequently has 11 owners, 20 source-data consumers, and eight project-input bindings. Current direct evidence is 7/131 in 12.20 seconds; the current isolated cache-skipped Nx target is 7/131 in 10.03 seconds (Nx 10.4 seconds). The earlier independent 19-consumer static control remains an accurately labelled pre-delta checkpoint; it is not evidence for the additional final consumer.
