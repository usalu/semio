# Assignment Engine

The reusable print document uses `semio` with `type=assignment`, a one-sided A4 article layout, an inline title and optional course metadata. Its assignment declaration schema requires explicit English/German language and light/dark theme. Plain-text metadata can be converted into a safe LaTeX preamble without external runtime dependencies. The reusable parameter table accepts arbitrary shape rows and customizable geometry; course-specific shape definitions remain with the course.

The shared JSON vectors specify accepted/rejected declarations and text escaping before implementation. The test adapter compares the owned validator against AJV, the registered third-party JSON Schema implementation. PDF compilation and visual review are coordinated by the parent agent with the course document.

## Verification

- Test-driven red run: the assignment suite first failed because the implementation module did not exist.
- Direct owned assignment suite: 13 passed, including schema/AJV agreement, inherited-property rejection, LaTeX text encoding and writable-table source contract.
- Focused architecture PDF suite: 10 passed. Finite authenticated runtime channel declarations agree with AJV; the actual HTTP request carries its token and preserves both byte bodies; GET/HEAD share media type and length, HEAD has no body; dev shutdown emits no static files; exact publications deduplicate without dropping another filename, and conflicting producers fail.
- TypeScript compiler API checked the existing architecture program: 97 diagnostics in the surrounding checkout, zero diagnostics in assignment-owned files at the time of the run. This is a bounded ownership check, not a claim that the complete workspace typechecks.
- Normal `repo run @semio-tech/print:test-assignment` and `repo dependencies lock --scope teaching` both failed during existing native source admission with `ENOENT .cache/b/l/fixed.dir`, before their authored command executed.
- The normal architecture Vitest configuration failed during workspace alias publication with existing teaching workspace-pattern drift. A ticket-owned focused Vitest configuration executes the actual new tests independently of that unrelated ownership scan.
- PDF compilation and page appearance remain coordinated by the parent ticket owner. No successful TeX/PDF compilation is claimed here.

## Publication

Architecture build/dev/dev-site explicitly depend on both declared print document targets. Their wrapper retains the exact print publications through compiler/listener settlement, reads them through the owned bounded material reader, and exposes an authenticated private loopback transport. Vite gets only finite runtime transport data, consumes bounded PDF bodies and writes/serves `assignments/*.pdf`. Missing transport/artifacts or invalid PDF bytes fail explicitly. Runtime transport is injected after Vite semantic compiler identity, following the existing service-channel pattern. The release artifact checker requires both PDFs; the rehearsal static origin returns `application/pdf`.

The teaching lock's existing architecture workspace record was synchronized manually with the new local `@semio-tech/print` dev dependency after the authorized dependency-lock command failed. The print workspace and package resolution were already present, so no external resolution changed. The root lock does not include the architecture workspace and required no change.

## Files Owned By This Agent

Print files: `🖋️latex/semio.cls`, `🖋️latex/semio-assignment.sty`, `🧾️template/📝️assignment/🟦️.ts`, `🧾️template/📝️assignment/🧬️schema/🔣️.json`, `🧪️tests/📝️assignment/{🟦️.ts,🔣️.json,🥒️.feature}`, `📦️packages/🟦️typescript/🧾️template/📝️assignment/🟦️.ts`, print TypeScript package/project manifests, `🧪️tests/🔬️script-commands/🟦️.ts`, quick pipeline verification, `README.md`.

Architecture files: Vite configuration plus `📖️assignments`, `📦️assignments` and `📃️documents` modules; site deploy module; TypeScript script/package/project manifests; static stack media map; `🧪️tests/📖️assignment-pdfs/{🟦️.ts,🧫️cases.json,🥒️.feature}`; test configuration; `🎓️teaching/bun.lock`.

Shared framework infrastructure: `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts` adds the optional owned runtime-channel input to artifact builds.

Retained ticket inputs: `assignment-vitest.config.ts`, `check-assignment-types.cjs`, and this report.

## Exact Ticket-Close File Inventory

- `🧰️framework/🛍️products/📓️print/🖋️latex/semio.cls`
- `🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧾️template/📝️assignment/🧬️schema/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🔣️.json`
- `🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🥒️.feature`
- `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/🧾️template/📝️assignment/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/package.json`
- `🧰️framework/🛍️products/📓️print/📦️packages/🟦️typescript/📋️project.json`
- `🧰️framework/🛍️products/📓️print/🧪️tests/🔬️script-commands/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/🎮️commands/🧪️print-pipeline-verification/🧪️tests/🖨️pipeline/⚡️quick-pipeline-verification/🟦️.ts`
- `🧰️framework/🛍️products/📓️print/README.md`
- `🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/📖️assignments/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/📦️assignments/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🏗️builder/🌐️vite/📃️documents/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🚀️deploy/🌐️site/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📜️script.ts`
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/package.json`
- `🎓️teaching/🏛️architecture/📚️course/📦️packages/🟦️typescript/📋️project.json`
- `🎓️teaching/🏛️architecture/📚️course/🧱️stack/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/📖️assignment-pdfs/🟦️.ts`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/📖️assignment-pdfs/🧫️cases.json`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/📖️assignment-pdfs/🥒️.feature`
- `🎓️teaching/🏛️architecture/📚️course/🧪️tests/🎚️config/🟦️.ts`
- `🎓️teaching/bun.lock`
- `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🌐️vite/🟦️.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/assignment-engine.md`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/assignment-vitest.config.ts`
- `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/check-assignment-types.cjs`
