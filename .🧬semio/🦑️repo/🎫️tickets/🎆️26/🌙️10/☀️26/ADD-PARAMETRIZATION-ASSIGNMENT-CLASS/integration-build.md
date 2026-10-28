# Course Assignment Build Integration

## Result

The parametrization finger exercise is a course-owned document under `🎓️teaching/🏛️architecture/📐️parametric/📝️assignment`. The generic Print catalog has no parametrization documents. The architecture course owns `build-parametrization-en` and `build-parametrization-de`, and normal dev, dev-site and build targets require these publications. Publish consumes the normal course build.

The shared schema requires an explicit producer identity for authored and generated publications, supports the course namespace, and uses the document ID for public PDF names. All active Print/report/Leutwiler publishers and retained readers were updated together. Historical ticket scripts retain their historical APIs as evidence.

Authored SVG resources are declared with canonical workspace paths, exact SHA256s, output names and widths. Build staging validates hashes, creates PNGs only in retained managed compiler scratch, and rechecks sources after use. The reusable Print renderer wraps the existing Sharp dependency; its first-party signature exposes only bytes, a width and an AbortSignal. It normalizes the authored viewBox to a bounded pixel viewport before opening the SVG, checks input/output bytes and pixels, refuses external references, and destroys its pipeline on cancellation. Sixteen PNGs are generated at 2048 pixels wide. No generated PNG or PDF was added to authored production sources.

Course transport accepts course-owned publications and verifies producer, ID and ID.pdf names. Its authenticated retained localhost transport, GET/HEAD media behavior and finite PDF byte checks remain intact. Course tests invoke the relocated semantic adapter. Existing pinned D3/mathjs versions were declared in the course devDependencies and corresponding teaching lock workspace record; no workspace membership was changed.

## Verification

- `repo tools exec --light bun -- test ./🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📋️intent/🧪️tests/🟦️.ts`: **10 passed**, including AJV/SQLite closed intent checks, resource pixel oracles, external-reference rejection, cancellation and tall/huge-physical-size viewport bounds.
- `repo tools exec --light bun -- ./.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/verify-build-integration.ts`: **passed**. Checks relocated assignment semantics against AJV/D3/mathjs, producer conflicts and course-only ownership, actual authenticated English/German PDF transport, publication slot projection, framework prerequisites and normal consumer dependencies.
- A direct loop using the production renderer over all **16 canonical SVGs** passed, with no generated files written by that loop.
- Root reran the real course assignment-pdfs Vitest suite using ticket `assignment-transport.config.ts`: **12 passed**. This focused config avoids unrelated workspace-bootstrap admission and does not replace the supported normal config.
- Native compiler QA coordinated by root/domain agent generated **three pages in each language**. Root reviewed all German pages and the English contact sheet/final page. See `native-course-qa.md` for the independently captured native results.
- Owned-file LF normalization and scoped `git diff --check` passed before handoff.

## Infrastructure Limits

`repo run @teaching/architecture-course:build-parametrization-en --exclude-dependencies` was attempted. Before target execution, source admission failed with ENOENT while lstat-ing `C:\git\semio\.cache\b\l\fixed.dir`. The course target therefore has no completed normal pipeline receipt from this session.

The lightweight generic Print source verifier import failed before executing assertions because `@semio-tech/framework-transient` was missing from the existing UI styling dependency resolution. The normal course Vitest configuration separately failed during workspace bootstrap with `Bun published workspace patterns drift: 🎓️teaching/package.json`. These unrelated infrastructure conditions were not bypassed with fake capabilities, receipts or new cache directories.

## Changed Files Owned by This Integration

- `🧰️framework/🛍️products/📓️print/README.md`
- `🧰️framework/🛍️products/📓️print/📦️artifacts/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📜️script.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📇️catalog/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📋️intent/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📋️intent/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📋️intent/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/📋️intent/🧪️tests/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🖼️resources/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🖼️resources/🧪️tests/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🖨️tectonic-template-compilation/🖼️resources/🧪️tests/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🔨️modules/🧪️viz-probe/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/📄️macro-and-pdf-verification/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/🛡️command-and-publication-verification/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/⚡️quick-pipeline-verification/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🧫️fixtures/🧫️merge-contract.json`
- `🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/📃️documents/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/📦️assignments/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🏗️builder/📖️assignments/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📜️script.ts`
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/package.json`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/📖️assignment-pdfs/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/📖️assignment-pdfs/🧫️cases.json`
- `♻️mit-bestand/📋️bericht/🔨️modules/📄️documents/🟦️.ts`
- `♻️mit-bestand/📋️bericht/🔨️modules/✅️report-verification/🟦️.ts`
- `👴️leutwiler/💤️realparts-of-powers-z-n/🟦️.ts`
- `🎓️teaching/bun.lock`
- `🎓️teaching/🏛️architecture/📚️course/README.md`

## Retained Ticket Inputs

- `verify-build-integration.ts` is an authored integration verifier input; keep it.
- This report records the final implementation and verification; keep it.
- Temporary generated compiler/render/test outputs belong under ticket `🗑️generated` and are cleaned by the ticket coordinator.
