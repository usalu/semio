# Architecture Parametric Assignment Integration

The Fingerübung is owned by the Architecture course's Parametric topic at:

`🎓️teaching/🏛️architecture/📐️parametric/📝️assignment`

That leaf contains the six authored TeX/resource-plan inputs and five content/provenance test inputs. Its localized metadata is registered in the existing course registry. It is absent from the generic Print template catalog.

## Build and Publication

The course owns these standard producers:

- `repo run @teaching/architecture-course:build-parametrization-en`
- `repo run @teaching/architecture-course:build-parametrization-de`

Normal course development and builds depend on these producers; publication consumes the course build. The shared Print product retains the reusable assignment class, matrix styling, Diagram/Transitions silhouettes, source projection and managed compiler/publication machinery.

Published document identities are `course/architecture-parametric-parametrization-en` and `course/architecture-parametric-parametrization-de`, with matching ID.pdf filenames and explicit course producer receipts. The course validates those identities and serves the same authenticated PDF transport for development and static publication.

Canonical Metabolism SVG paths and SHA256s remain authored inputs. The existing Sharp dependency is wrapped behind an owned byte-oriented build interface, which creates bounded 2048-pixel PNG compiler resources in managed scratch and honors cancellation. Generated capsule PDFs were removed from source. The university footer is explicitly selected by this assignment.

## Verification

| Check | Result |
| --- | --- |
| Generic course TypeScript suite | 595 passed |
| Generic Print assignment and intent/resource suites | 29 passed |
| Course PDF publication/transport suite | 12 passed |
| Cross-layer integration verifier | Passed |
| Generic page chrome / original logo oracles | Passed |
| Canonical SVG provenance and actual rendering | All 16 passed |
| Native English and German LaTeX compiles | Both three A4 pages |
| Native PDF layout | Zero overflow/underflow; both tables on one page; centered capsules |
| Scoped whitespace review | Passed |

The source contracts were tested red before implementation. Independent checks use AJV, SQLite, D3, mathjs, lxml, image decoding and PDF parsers. Native source bindings were rechecked after use, and rendered pages were visually reviewed.

The focused course Vitest run uses the retained portable ticket config `assignment-transport.config.ts` to execute the actual course test module without unrelated workspace bootstrap. It does not replace the normal course configuration.

## Remaining Repository Blockers

Managed course build and Rust verification were attempted through standard repo targets, but source admission failed before target execution:

`ENOENT: lstat C:\git\semio\.cache\b\l\fixed.dir`

No managed publication receipt was produced. Native QA compiles succeeded through the registered pinned compiler interface; no capabilities, receipts or cache directories were fabricated.

The normal course Vitest configuration also stops during ownership bootstrap with `Bun published workspace patterns drift: 🎓️teaching/package.json`. The broader lightweight Print source verifier stops on existing missing `@semio-tech/framework-transient` resolution in UI styling. These limits are recorded without changing other ongoing work.

## Output and Git Index Policy

The three task-created root `output/pdf` files and all sixteen generated capsule PDFs were removed from the working tree. The root output directory is absent and ignored. The canonical assignment leaf contains no compiled PDF/PNG. Native QA copies, images, fonts, PDFs, logs and renders are deleted from the ticket generated folder at completion; authored scripts, fixtures and reports are retained.

Nineteen generated PDF additions were already staged before this integration. Their working-tree state is now `AD`: absent on disk, still present in the existing index. User instructions explicitly prohibit Git mutations, so the index was not changed and these staged bytes still need unstaging before any commit. The exact paths are in [integration-final-audit.md](integration-final-audit.md).

## Supporting Records

- [Build implementation and file inventory](integration-build.md)
- [Course content integration](integration-course.md)
- [Framework contract verification](integration-framework.md)
- [Native PDF measurements and hashes](native-course-qa.md)
- [Ownership audit and exact existing index hazard](integration-final-audit.md)
