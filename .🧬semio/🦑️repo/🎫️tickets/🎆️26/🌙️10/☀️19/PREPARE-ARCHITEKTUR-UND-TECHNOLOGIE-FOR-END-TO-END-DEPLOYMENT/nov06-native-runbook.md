# Native Course Startup and Acceptance Runbook — Draft

Status on November 6: native Windows compiler and UV proof passed. JavaScript synchronization 55996 has not reported Ready; Python synchronization, Print, course gates and browser acceptance remain unrun. No website is claimed running. This is a plan, not execution evidence. Confirm the current publisher receipt before proceeding; do not start a second dependency writer.

## Prepare and verify

Run these registered targets from the repository root after the shared JavaScript publisher is actually Ready. These steps remain unexecuted; record each result and stop on failure. Preserve the declared graph and Rust generation prerequisites.

    repo run workspace:deps-python --parallel=1 --lang=en
    repo run repo:test-build-preflight --parallel=1 --lang=en
    repo run @semio-tech/print:deps-tectonic --parallel=1 --lang=en
    repo run @semio-tech/print:fonts --parallel=1 --lang=en
    repo run @semio-tech/print:deps-tex --parallel=1 --lang=en
    repo run @semio-tech/print:generate --parallel=1 --lang=en
    repo run @teaching/proctor:test-launcher --parallel=1 --lang=en
    repo run @teaching/proctor:build --parallel=1 --lang=en
    repo run @teaching/architecture-course:test --parallel=1 --lang=en -- quick
    repo run @teaching/architecture-course:typecheck --parallel=1 --lang=en
    repo run @teaching/architecture-course:check --parallel=1 --lang=en
    repo run @teaching/architecture-course:build --parallel=1 --lang=en
    repo run workspace:deps-browsers --parallel=1 --lang=en
    repo run @teaching/architecture-course:test-e2e --parallel=1 --lang=en -- --topology dev
    repo run @teaching/architecture-course:test-e2e --configuration=release --parallel=1 --lang=en -- --topology rehearsal

The build and browser targets include four PDF producers: parametrization and typology, each in English and German. Each declares Print fonts, Tectonic, TeX bundle and generation prerequisites. The end-to-end target can also prepare its browser revision. The course ci-browser-prepare target installs Debian system libraries and is not native Windows browser preparation. Keep temporary test outputs under the ticket generated directory; use process-local temporary-directory settings as described in [browser plan](nov05-browser-plan.md).

For both topologies, record real browser gate results for language selection, introduction, identity, scoring, reload persistence, second-device recovery, presence, short outage and recovery, and phone interaction. In author preview check all four actual localized assignment responses: HTTP 200, PDF content type, PDF signature and the declared 16 MiB bound. These checks require the real compiled proctor and produced PDFs.

## Normal local stack

After successful prerequisites, start the normal stack in an owned retained session:

    repo run @teaching/architecture-course:dev --parallel=1 --lang=en

Wait for reported readiness and verify the actual URL. Defaults are site http://127.0.0.1:6061 and proctor http://127.0.0.1:8791; environment overrides change them. Open the reported site in a real browser, verify a learner journey and responsive layout, and leave the requested stack running. The stack may reuse an already responding site or proctor and explicitly does not own or stop reused processes; verify what it reports before treating them as the newly built stack.

When stopping an owned stack is requested, use Ctrl+C in its retained session and wait for cleanup and exit; its current implementation stops its owned site and proctor. Do not terminate a reused or unidentified process. Standalone proctor cancellation has a current source change and requires the registered launcher test and actual verification before claiming it works; the earlier [source audit](nov06-proctor-native-audit.md) records the original defect.

The local release rehearsal is native browser and server evidence only. The image build and deployment gate retain their native Linux x64 guard and remain unverified on this Windows host. Do not infer a Linux or public deployment pass from a Windows rehearsal.

Authorities: [course README](../../../../../../../🎓️teaching/🏛️architecture/📚️course/README.md), [proctor README](../../../../../../../🎓️teaching/🛂️proctor/README.md), [course targets](../../../../../../../🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json), [proctor targets](../../../../../../../🎓️teaching/🛂️proctor/📦️packages/🦀️rust/📋️project.json), [prerequisite audit](nov06-course-prereqs.md), and [browser plan](nov05-browser-plan.md). Source declarations and this draft do not substitute for actual gate results.
