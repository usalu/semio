# Final Assignment Source And Test Verification

Verified on 2026-10-27 through the repository tool-independent ingress. This independent review did not compile documents or alter unrelated files. The parent ticket owner compiles and visually inspects actual LaTeX output.

## Passing Results

| Check | Result | Independent oracle |
| --- | --- | --- |
| Reusable assignment suite | 16 passed, 0 failed, 45 assertions | AJV2020 declaration validity agrees with owned validator |
| Shared parametrization content | All semantic/source checks passed | AJV2020, mathjs, d3-array |
| Course core | 591 passed in 19 files | Existing full course configuration and its oracle adapters |
| PDF publication and transport | 10 passed in one file | AJV schema agreement and actual authenticated HTTP byte checks |
| Course assignment corpus | Seven language-neutral scenarios passed | AJV versus owned validation and CourseView projection |
| Localized assignment presentation | Passed | Existing React server renderer and TypeScript transpiler |
| Canonical icon PDF assets | All 16 passed | pypdf content streams, metadata and MediaBox, XML source dimensions and SHA256 |

The icon verifier checked exact original Metabolism SVG hashes, one vector-only PDF page per asset, no image XObjects, nonempty vector geometry, exact source dimensions, source provenance metadata and registration in both language documents. Content checks also retain two eight-specimen tables, two complete K8 graphs and their full 28-edge ledgers per language, shared geometry and explicit language mains.

## Metadata Correction

Actual parent-produced PDF bytes exposed missing document Title and Lang metadata. Before implementation, two new language-neutral metadata vectors and matching source tests were added: the first run had 14 passes and two expected failures. The reusable assignment package now sets PDF title and author from the final authored LaTeX declarations at document start, after the document preamble has supplied them. It maps only explicit English to en-US and German to de-DE. The assignment suite then passed all 16 tests. No font, palette, spacing, geometry or runtime dependency changed for this correction.

Actual metadata in the compiled final PDFs must be checked independently by the parent with pypdf. These source tests intentionally do not claim actual PDF compilation or final page appearance.

## Exact Commands

All commands ran from C:/git/semio. Tool arguments follow the mandatory -- separator.

- repo tools exec --light bun -- test ./🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🟦️.ts
- repo tools exec --light bun -- -e "import { verifyParametrizationAssignmentContentV1 } from ./...; verifyParametrizationAssignmentContentV1();" (the actual import was ./🧰️framework/🛍️products/📓️print/🧪️tests/📝️parametrization-assignment/🟦️.ts)
- repo tools exec --light node -- node_modules/vitest/vitest.mjs run --config 🧰️framework/🛍️products/📚️course/🧪️tests/🎚️config/🟦️.ts
- repo tools exec --light node -- node_modules/vitest/vitest.mjs run --config .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/assignment-vitest.config.ts
- repo tools exec --light node -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/assignment-conformance.mjs
- repo tools exec --light node -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/assignment-view.mjs
- repo tools exec --light python -- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/verify-assignment-icons.py --library C:/Users/Ueli/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/Lib/site-packages

The Python command selected the preexisting bundled interpreter using SEMIO_TOOL_PYTHON=C:/Users/Ueli/.cache/codex-runtimes/codex-primary-runtime/dependencies/python/python.exe. No package installation occurred.

One preliminary Bun command omitted the leading ./ and searched instead of selecting the unconventional filename; it selected no tests and failed. The corrected explicit-path invocation above ran the intended file and passed. This is distinct from the expected metadata test-driven red run.

## Exact Files Changed By This Verification

- 🧰️framework/🛍️products/📓️print/🖋️latex/semio-assignment.sty
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🟦️.ts
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🔣️.json
- 🧰️framework/🛍️products/📓️print/🧪️tests/📝️assignment/🥒️.feature
- .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️26/ADD-PARAMETRIZATION-ASSIGNMENT-CLASS/latex-final-tests.md

For the exact earlier LaTeX class, icon-resource and paragraph-fix inventory, see latex-product.md in this ticket. For prior course and publication integration inventories, see course-assignment.md and assignment-engine.md. No AGENTS file, git state or worktree was changed. This verification created no retained tool output files.

