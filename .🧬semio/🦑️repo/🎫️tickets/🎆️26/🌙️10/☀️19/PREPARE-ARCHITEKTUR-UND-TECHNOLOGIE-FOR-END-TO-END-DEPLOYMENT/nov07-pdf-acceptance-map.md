# November 7 PDF Acceptance Map

Read-only current-source and file-existence inspection; no PDF, browser, or target was run. No concrete missing authored file or naming mismatch was found within the inspected roster.

| Assignment | Locale | Declared producer suffix | Print document ID and exported filename |
| --- | --- | --- | --- |
| parametrization | en | build-parametrization-en | architecture-parametric-parametrization-en.pdf |
| parametrization | de | build-parametrization-de | architecture-parametric-parametrization-de.pdf |
| typology | en | build-typology-en | architecture-parametric-typology-en.pdf |
| typology | de | build-typology-de | architecture-parametric-typology-de.pdf |

The course `🔣️.json` declares both localized titles/descriptions and these four `course` namespace documents, each with producer `@teaching/architecture-course:<suffix>`, ID equal to its filename without `.pdf`. The current script router binds each suffix to the same assignment ID and locale. The assignment builder selects the declared document and passes its namespace, ID and producer to Print, with `dark:false`. Current Print `publishPrintArtifact` names its light publication `intent.id + ".pdf"`; the four outputs therefore match the declared files by construction. This is source mapping, not proof of artifacts.

The builder references `en.tex`, `de.tex`, both localized content leaves, and for parametrization `geometry.tex` and `icons.json`; all exist. Its 16 declared workspace SVG icon sources all exist. The typology builder's six declared cover/logo/diagram images and both localized TeX pairs exist. Both TeX entrypoints use the shared `assignment` class with matching language; that class exists in Print's `🖋️latex`. Print's `semio-fonts.sty` names Anta, Share Tech Mono, Noto Emoji and Noto Sans CJK SC; the current font catalog maps each bare TeX filename to its authored emoji-prefixed file, and all four source files exist. These checks do not establish font materialization, resource digests, TeX compilation or visual correctness.

The Vite document contract requires `file === id + ".pdf"`, deduplicates by filename and rejects conflicting producer declarations. The retained Print transport reads each exact published document, checks signature and the 16 MiB limit, and serves it privately at `/pdfs/<file>`; the Vite plugin exposes `/assignments/<file>` in development and writes that same relative path in static builds. Its middleware sends PDF content type and permits GET/HEAD. This agrees with the course declarations and browser link expectation; no live HTTP response was checked.

The current `🧪️tests/🗣️both-languages/🟦️.ts` creates one PDF test per locale, loops over both authored assignments, enters author preview, checks each exact visible link, download name and `/assignments/<file>`, then fetches and checks 200, PDF content type, signature, nonempty body and the 16 MiB bound. The actual Playwright configuration selects that file in the `desktop` project. The session manifest includes desktop in the shared shard for both dev and rehearsal stacks; the E2E runner lists both topologies and runs the selected stack's waves. This establishes static discovery under the default full wave, subject to user-supplied project/test filters, not a Playwright discovery or execution result. Genuine generated documents and both browser gates remain required.
