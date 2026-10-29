# Fresh Dev Bootstrap Audit — 2026-10-28

Read-only source comparison; no builds, service starts, probes or source edits. No missing seed/catalog preparation was found in the inspected normal-dev versus browser-harness startup chain. This is not an observed successful fresh launch.

## Backend Defaults And Initialization

Normal dev calls proctorDevelopmentEnvironment (🎓️teaching/🛂️proctor/🏗️bootstrap/🟦️.ts:59–65), supplying port8791, absolute disposable data directory .🧬semio/🎓️teaching/proctor-dev, and the actual architecture course JSON. Existing PROCTOR_* overrides intentionally win. The course stack invokes developmentDataSettled before qualified product acquisition and launch (course 🧱️stack/🟦️.ts:281–287). An absent database needs no operator seed; incompatible launcher-owned development data is set aside, whereas explicitly configured foreign data is refused rather than silently replaced (proctor bootstrap:150–174).

The real process entry delegates to CLI serve. proctor 🔨️modules/⌨️cli/🦀️.rs:120–149 loads config, loads the selected course, assembles the server with embedded data storage, prepares projectors, reconciles/catches up, then binds and serves. loaded at612–622 calls load_course, not an E2E-only fixture catalog. The architecture material is therefore ordinary startup input, not seeded through a hidden harness endpoint. Proctor::assemble in 🔨️modules/🧩️instance/🦀️.rs:480–515 mounts core routes, registers all query kinds under course.id(), assigns anonymous learner/presence and service policy, and creates the router. No prior manual actor/policy enrollment is required by this assembly.

Configuration infers development posture for loopback when PROCTOR_MODE is absent (🔨️modules/🎚️config/🦀️.rs:217–223); normal dev does not need the harness explicit development variable in a fresh environment. The proctor-owned SQLite implementation documents directory/database/schema creation in 🔨️modules/🗄️storage/🦀️.rs:156–169; actual server assembly now passes StorageProfile::Embedded through the framework builder, so that older local Database helper alone is not evidence for every current framework storage path. Actual fresh embedded-server creation remains a runtime check.

## What The Harness Adds

Course 🎭️e2e/🟦️.ts:205–210 strips inherited PROCTOR_* and supplies fresh per-stack data, the same PROCTOR_DEV_COURSE and selected port. It creates the data directory at267, selects explicit development or production/origin posture at268, and launches the same serve command at279. Its control endpoint only stops/restarts that process (212–215); it does not initialize course data or register learners. Harness warmup loads the site; it is not a backend seed. Production rehearsal additionally bakes the separate proctor origin, builds/validates a static release artifact and sets allowed site origin; these are topology differences, not prerequisites omitted by local proxy development.

Normal dev preserves intentional PROCTOR_* overrides, while E2E sanitizes them. Therefore a passing E2E does not prove startup under an arbitrary inherited public bind, production mode, foreign course, data directory or allowed-origin override. Use the documented fresh/default environment for local availability, and record any deliberate overrides; no new config layer is needed.

## First Visit

The actual architecture site entry 🟦️.ts:16–34 mounts the renderer with tenant architecture and loadArchitectureCourseMaterial. Development uses the empty baked proctor origin, so gateway requests go through the Vite same-origin proxy (🏗️builder/🌐️vite/🟦️.ts:24–25,180–183). Material is supplied by the site itself for local-first use, rather than injected into browser storage by tests.

The browser harness openDevice (🎭️e2e/🚶️learner/🟦️.ts:215–240) creates a fresh context, selects the browser locale and installs only security/error observation. It does not preload localStorage, cookies, authenticated state or learner records; the fixture explicitly promises empty storage at243. The site-boot spec drives the ordinary introduction for both languages, while first-visit specs drive registration through the UI. Author-preview is explicitly enabled only in content journeys by URL parameter, as recorded in runtime-coverage-audit; it is not required to boot the introduction or register a learner.

## Actionable Boundary

No harness-only seed or manual catalog prerequisite was identified. After current compilation/gates, start the supported managed dev target, retain its owner session, and verify actual introduction plus /instance readiness on fresh/default data and a fresh browser context. Do not substitute the E2E fresh-state pass for verification of a reused backend: runDevStack may reuse an already answering process and merely announce a contract mismatch (🧱️stack/🟦️.ts:420–435). Persistent usable availability remains the distinct acceptance step.
