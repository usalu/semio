# Architecture Course Redeploy Readiness

Status on 2026-11-11: **not yet verified end to end.** The TypeScript side is green when run directly; everything that
goes through `repo run` is blocked on this Mac because the Xcode command line tools are not installed (no linker, so
no Rust binary: graph service, proctor, native `repo`, MCP servers).

This folder was created by hand: `ticket_open` was unavailable because the `repo` MCP server could not start. Register
it with `ticket_open`/`ticket_reopen` once the native `repo` exists.

## Host state

- Fresh macOS 27 arm64 machine, no Xcode command line tools, no Node, no Docker.
- `repo` launcher installed from the checkout (`~/.local/share/semio/launcher/bin/repo`), Bun 1.4.2 provisioned by it.
- `repo preflight` refuses: `missing executable: developer-tools`.
- `repo dependencies sync --all` succeeds (after the fix below).
- `repo run …` fails: the graph service compiles `semio-repo-watch`, linking with `cc` fails.

## Test results (run directly with `bun x vitest`, under Bun, not through `repo run`)

| Suite | Result |
|---|---|
| `@semio-tech/course` (core) | 595 pass |
| `@semio-tech/course-react` (renderer) | 1100 pass |
| `@teaching/architecture-course` (site, deploy files) | 847 pass |

The site suite needs the pinned Node 24.15.0 on PATH for its port probe; the native bootstrap (`🔩️native/🥾️bootstrap/🐚️.sh
setup`) installed it, together with rustup and the pinned nightly, and then stopped at the missing developer tools.
A failed language switch now raises a notice in the shown language with retry and dismiss, and is not persisted.

Not run at all: every Rust test, `typecheck`, `check` (`proctor check`), `build`, `publish`, `dev`, `test-e2e`,
the Docker image and stack targets, and nothing under the pinned Node 24.15.0.

## Changes made

### Build tool
- Dependency storage (`🦑️repo/…/📦️dependencies/💾️storage/🟦️.ts`): the stamp of a symbolic link no longer counts
  ctime (`javascriptDependencyStampV1`). On this Mac something bumped the ctime of the workspace `node_modules` links
  after creation and the sync refused with "Store Source fence changed". The process that changes the ctime was not
  identified. Fence tests: 6 pass, 3 could not run here (Python oracle, invocation envelope).

### Manifests
- `🛂️proctor/📦️packages/🦀️rust/📋️project.json` `proctorRust`: added `🚪️io/⏱️failure-owner` and the twelve
  `🧵️job` crates (52 → 76 globs, the Cargo closure the site test pins).
- `📚️course/📦️packages/🟦️typescript/📋️project.json` `proctorCargoManifests`: the same crates.

### Site
- Typology assignment completed in fixtures and schemas (dashboard contract, publication fixture, assignment producers):
  four PDFs instead of two.

### Renderer (`@semio-tech/course-react`)
- Locale chunks and screen chunks retry at most four times; a bilingual fallback with "Try again now" and
  "Reload page", or a "new version available" state with a reload, replaces the blank page and the endless spinner.
- Outbox: a failing settle callback or listener can no longer break delivery; `pump` cannot reject.
- Presence socket removes its abort listener when the socket ends.
- An abandoned drag releases its window listeners.
- Spoken durations are owned for en and de (", " join) instead of delegated to the engine's `Intl.ListFormat`.
- Home-grid tests and README follow the framework's `LayeredOverview`: hover centres, press clears the glass.

### Framework
- `🚪️io/🌐️fetch-timeout`: the deadline now covers reading the response body. Other callers were not re-tested.
- `⏳️async/🔁️jittered-backoff`: optional `maxAttempts`.
- `🖱️ui/🧱️elements/🗂️WindowChrome/shared-border.ts`: `isElement` was used but never defined (ReferenceError in every
  MutationObserver callback); defined, with a regression test.

### Content and documentation
- `$schema` added to the excel and grasshopper quizzes (their revisions change; open runs are voided).
- Parametric quiz emoji matches its badge.
- READMEs of teaching, architecture, course site, proctor and the course product corrected against manifests and code.

## Open findings, not changed

- Sorting tasks in the physics and ventilation quizzes contain equal values across pool variants; whether that is
  allowed is for `proctor check` to say.
- Physics quiz: 48 items share one of two generic explanations without value or source.
- `🚀️deploy/🔣️.json` `site.legal` is empty: no imprint or privacy link on the public site.
- Quiz solutions ship to the browser (documented design: the device-side deputy).
- `repo run` reports "graph service could not answer (timeout)" instead of the linker error that caused it.
- `🎓️teaching/Cargo.lock` lists `semio-framework-value-retained-clone-protocol`, which no manifest defines.
- Proctor: `open()` runs the schema before refusing a foreign database file; CORS preflights are not throttled.

## Remaining steps

1. Install the Xcode command line tools (`xcode-select --install`), then `repo preflight`.
2. `repo run @teaching/architecture-course:test`, `:typecheck`, `:check`, `@teaching/proctor:test`.
3. `repo run @teaching/architecture-course:dev` and `:test-e2e -- --topology dev`.
4. Release gate and image (`deploy-check`, `docker-image-build`, `docker-image-publish`) only on Linux x64 with Docker;
   the workflow `architecture-course.yml` needs the `ARCHITECTURE_COURSE_RUNNER` variable and the default branch.
