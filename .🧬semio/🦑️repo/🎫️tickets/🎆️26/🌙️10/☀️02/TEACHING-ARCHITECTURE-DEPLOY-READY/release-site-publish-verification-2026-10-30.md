# Release Site Publish Verification

Date: 2026-10-30. Ticket `26/10/02/TEACHING-ARCHITECTURE-DEPLOY-READY` was already open (`ticket_reopen`: ticket is already open) under `🎯runningframework🎯runningproducts`. `repo://goals` was read before the run.

## How publish stages the site

`@teaching/architecture-course:publish` is `repo ./📜️script.ts publish`. It depends on assets, styling tokens, and `@teaching/architecture-course:build`. `publish` builds nothing: `publishCourseSite` requires release mode, reads the immutable Vite product owned by `architecture-course:site`, rechecks it, and refuses unless `siteArtifactProblems` is empty. It then copies the public files to `📦️packages/🟦️typescript/dist/release/pages/course` and writes `_headers`.

The baked proctor origin is `PROCTOR_URL`, otherwise `https://semio.iek.uni-hannover.de` from `🚀️deploy/🔣️.json`. That origin selects the `publish` size topology (`scriptGzipBytes.publish` = 250,000). `totalBytes` is 5,000,000. The site host in `CNAME` is `architektur-und-technologie.de`.

The heading link in `CourseCard` calls `onOpen` and `preventDefault` only for an unmodified primary click (`button === 0` and no meta, ctrl, shift, or alt). Modified clicks keep the browser's own navigation. That module is an input of the site build.

## Run

Refreshed after the `CourseCard` heading-link `onClick` update in `chrome/🟦️.tsx`. The earlier staging in this note (on-disk 4,991,595 bytes, file-list sha256 `daf1b566bf8f7814ee3e936916cda486c7cd8d8915cf18f580c8c80e010196d3`) is replaced by the figures below.

Command, from the workspace root, started `2026-10-30T22:35:30Z`:

`SEMIO_BUILD_MODE=release NX_DAEMON=false SEMIO_LANGUAGE=en repo run @teaching/architecture-course:publish --configuration=release --parallel=1 --output=stream`

Nx ran `publish:release` and the 13 tasks it depends on. Cache was 0/14. Run duration was 26m 11s. Critical path was 9m 44s. The log ends with `NX Successfully ran target publish`. Exit code 0.

Publisher console:

- `staging the site built for https://architektur-und-technologie.de against https://semio.iek.uni-hannover.de`
- `staged 86 files in .../dist/release/pages/course`

No rehearsal warning was printed, so this artifact is the production proctor build.

## Staged artifact

`verify-staged-site.ts` ran `siteArtifactProblems` on the staged directory (including `_headers`), with proctor origin `https://semio.iek.uni-hannover.de`. Result: **0 problems**.

| Check | Result | Budget |
| --- | --- | --- |
| Problems | 0 | — |
| Files | 86 | — |
| On-disk bytes | 4,991,648 | 5,000,000 |
| Topology | `publish` | — |
| Document scripts, gzip | 222,244 | 250,000 |
| Document stylesheets, gzip | 41,479 | 60,000 |
| Baked proctor | `https://semio.iek.uni-hannover.de` | — |
| `CNAME` | `architektur-und-technologie.de` | — |
| `index.html` equals `404.html` | yes | — |
| `_headers` | present | — |

Margin under the 5,000,000-byte budget: 8,352 bytes. Script gzip is 27,756 bytes under 250,000. Stylesheet gzip is 18,521 bytes under 60,000.

The staged scripts contain `course-card-link` and the plain-click guard (`metaKey` together with `preventDefault`), so the `CourseCard` heading-link change is in this release bundle.

Verifier: `verify-staged-site.ts` in this ticket. File-list sha256: `ffdaf4c31edf64232a0c7df046ce1ed20364c29ce1f099209df077f1be1971c1`.
