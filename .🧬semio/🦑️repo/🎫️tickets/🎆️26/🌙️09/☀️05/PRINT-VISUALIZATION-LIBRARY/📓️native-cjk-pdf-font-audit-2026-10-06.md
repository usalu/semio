# Current Native PDF CJK Font Resource Audit

Read-only bundled pypdf inspection ran through Bun 1.3.14 + Nx exec selecting the existing print project. Actual session 75309 exited 0. An earlier session 55217 exited 1 because Windows/Nx stripped nested double quotes from the Python evaluator; that infrastructure failure produced no PDF evidence. No native compiler or product edit was performed.

Four exact PDFs were inspected, with SHA-256 and full font resources/text runs retained in `📥️authored-inputs/native-cjk-pdf-font-observation-2026-10-06.json`:

| Existing suite | Themes | Pages each | Embedded fonts |
|---|---|---:|---|
| critical-cjk-canonical-green-2026-10-06 | light, dark | 12 | /IHAIZV+Anta-Regular; /BZIVMZ+ShareTechMono-Regular; /IXFAWU+NotoSansCJKsc-Regular-Identity-H |
| forest-ordinal-current-green-2026-10-06 | light, dark | 24 | /IHAIZV+Anta-Regular; /BZIVMZ+ShareTechMono-Regular |

Both critical PDFs select the embedded Noto font for 根, 子, 葉 on pages 4 and 5, including reversed node order. Their exact ToUnicode bfchar mappings are `<3BF9> <5B50>` (子), `<52AF> <6839>` (根), `<8717> <8449>` (葉). These are actual PDF mappings and selected text-run fonts, not an inference from extracted text alone.

A read-only bounded sfnt cmap format-12 inspection of the exact tracked Noto OTF found both Unicode platform 0/encoding 4 and Windows platform 3/encoding 10 tables mapping 根 to nonzero glyph 21167 (0x52AF), 子 to 15353 (0x3BF9), and 葉 to 34583 (0x8717). The source glyph IDs match those PDF character codes. The OTF signature is OTTO; its byte hash and mappings are retained in `📥️authored-inputs/native-cjk-cmap-observation-2026-10-06.json`. Thus the observed three native Han runs use the embedded tracked-family font, with source glyph coverage and matching IDs; no system fallback font is selected for these observed runs. This does not prove coverage of untested Unicode scalars.

All four matching TeX logs contain the exact NotoSansCJKsc-Regular.otf and Anta-Regular.ttf file names and zero `Missing character` warnings. Exact log hashes and counts are retained in `📥️authored-inputs/native-cjk-font-log-observation-2026-10-06.json`.

Latin restoration is visible across node/page boundaries: after page 4 Han runs, page 5 heading/caption runs select Anta; after page 5 Han runs, page 6 heading, captions and B/L/A node runs select Anta. Page footers select ShareTechMono. These fixtures do not contain a mixed Latin/Han/Latin node caption, so within-one-caption transition restoration remains untested by this PDF set and should receive a genuine mixed-caption fixture. The forest PDFs contain no extracted Han captions and no Noto font resource, so they contribute no CJK glyph evidence despite loading the fallback font in their logs.

This is narrowly scoped font resource/glyph/run evidence from the four hash-bound artifacts. It does not establish overall critical/forest phase completion, current whole-source equality, metric fallback success, or visual layout correctness. Generated inspection console is `🗑️generated/native-cjk-pdf-font-audit2.log`. Parent phase receipts remain authoritative for overall status.
