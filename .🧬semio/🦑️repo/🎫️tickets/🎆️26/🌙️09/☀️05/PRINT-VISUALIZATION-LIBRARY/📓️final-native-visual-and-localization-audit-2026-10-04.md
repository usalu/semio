# Native Visual And Localization Audit

Read-only audit of corrected native family output on 2026-10-04. No production sources, Git state, tickets or goals were modified. The PDF skill was applied with bundled Poppler. Source remains under concurrent correction; this is a bounded inspection of actual identified outputs, not a final catalogue certification.

## Actual Output Inspected

The catalogue owner supplied `🗑️generated/native-family-containment-green2/{light,dark}/🧪️probe-out/cases.pdf`. Both PDFs were copied inside ticket-generated `final-native-visual-audit` and Poppler rendered pages 2 through 7 at 110 dpi. All twelve resulting full-page images were actually viewed.

- Light PDF: 30,401 bytes, SHA256 EA9A0C2D7D4BED56E33C02B3F8F44E50CE21B7D66AC09CAE192788A9D1B93BDB.
- Dark PDF: 30,375 bytes, SHA256 CB88A1CF4E43DFEC5593F8346CE02953D11EAE98D2DB62DD220D634C43A802EA.

| PDF Page | Actual Visible Observation In Both Themes |
| --- | --- |
| 2 molecule-default | Molecular bonds, wedge and O/N labels stay inside the figure frame. The previous below-frame N label is absent. |
| 3 molecule-negative | Authored negative-coordinate molecule is visibly fitted inside the frame, including both atom labels and double bond. |
| 4 reaction-symbol | Arrow and triangular reaction annotation are visible; no missing-glyph box appears. Molecule remains contained. |
| 5 feynman-labels | Incoming electron charge labels, gamma and outgoing mu charge labels are visible and contained. |
| 6 board-six | Four K/Q/R/N markers lie inside actual six-by-six cells; R and N no longer sit outside the board. |
| 7 process-horizontal | Requestdata, Submitform and Complete? fit their rendered nodes. Remaining edge captions are compact but this inspected image provides no new definite clipping blocker. |

Scientific fixtures intentionally occupy part of the canvas; unused space alone is not classified as a defect. This inspection does not measure font readability at final physical reproduction size. Compiler log searches found no `Missing character`, Overfull or Underfull warnings in either green2 case log. The only lowercase warning match was the informational infwarerr package description.

## Identified Compiled Source Revision

Hashes below identify the actual green2 staged `.semio-library/semio-latex` files used by this PDF revision, rather than unversioned current workspace files:

| Source | SHA256 |
| --- | --- |
| semio-viz-scientific-field.sty | 3087D2FF69C4CB68DE6AD32B8B8AE8E7F8D5E23C13A9B503E23FD87F33FEB62F |
| semio-viz-scientific-chemistry.sty | D5049E9C4121B6C2A92E76AC53F4D810558E727C5B0CB2284005497B1A259D3B |
| semio-viz-scientific-physics.sty | 9F37C3E89A237EDA877E9E374BCE8EFC8D500A02EB94DE7CE1511D31B5642A39 |
| semio-viz-diagram-process.sty | B4E830D9A6AE944A7326A1088B1C78D15ED757F1651549483DED56DB06594CB7 |
| semio.cls | DC34C8DCDC3F40AAE3C3F1EFCC2FF5BD950F8C236DA20677DED6B5E048E53D92 |
| semio-core.sty | 654FD134F3B08D7964C405DF4627B2BA5F73953C1528FC69B55F3981BD99BCFF |

The owner reported the green2 remaining Delta extraction assertion as standard Type1 Delta mapping to U+2206; the current neutral fixture now expects that extracted character. Raster inspection supports the physical triangle glyph only. This lane did not independently rerun that revised extraction assertion.

## Localization Source Contract

Current canonical chart schema requires `language` alongside width, height and layers; `LocalizedText` requires both nonempty en and de strings and rejects additional properties. Its TypeScript twin exposes `{en:string;de:string}` and languages en/de without a default. Native inference explicitly rejects any chart language outside en/de and sets the local document language for emitted figures. Numerical rendering throws if localized text or formatted guide labels lack explicit language.

A concrete pre-existing broader print document discrepancy exists: `🖋️latex/semio.cls:16` declares `DeclareStringOption[de]{language}`, and `semio-core.sty:35` initializes the language token to de. These native standalone document owners therefore retain a German default although canonical chart admission has no default. The six-family probe explicitly requests language=en; its successful output does not test omitted-language rejection or German family output. This discrepancy was sent to the root owner for scope determination; no production edit was made here.

The authored family glyph fixture covers O/N, a reaction triangle, gamma/mu and four Latin board markers. It cannot certify arbitrary Unicode glyph support or all German captions. No unsupported-glyph finding was inferred without compiler evidence.

## Remaining Boundaries

Green2 has six cases and does not contain the new crowded parallel-sets caption fixture. The owner is compiling green3 with that seventh case and subsequent native gates. Final catalogue rendering and final source hashing remain owned by the catalogue/root lanes. Prior graph and font geometry PASS reports are evidence for their tested revisions, not claims that all current edits have completed. No new material visual blocker was observed in the twelve inspected corrected pages.

## Explicit Visualization Document Language Audit

The root determined the generic standalone class default is outside this visualization API scope and is not a visualization release blocker. Read-only catalogue consumption inspected every `viz-*` document metadata entry, resolved its actual texPath, and matched the real documentclass declaration against explicit language=en/de. All 81 entries (80 numbered galleries plus viz-api) have explicit language; zero implicit visualization document declarations were found. The canonical native emission, directed-graph and physical-font compiler probes each explicitly pass type=paper,language=en. Family containment passes language=en in both themes and its authored invalid-board probe. The generic probe helper can construct article documents without language and is not itself an implicit semio visualization path.

## Final Corrected Crowded Flow Output

The catalogue owner subsequently published green5 at `🗑️generated/native-family-containment-green5/{light,dark}/🧪️probe-out/cases.pdf`. Both final flow pages were rendered with Poppler at 150 dpi and all four images were actually viewed. Page7 `flow-captions` contains explicit authored 97:2:1 Renew/Cancel/Buy targets; all three labels are visibly vertically separated with leader placement and remain in frame. Page8 `flow-adjacent` contains the stock multistage example; Paid/Renew/Cancel caption columns are visibly separated and contained. Dense intermediate captions are small, but no definite clipping or caption collision was observed in these actual pages. Light and dark themes both preserve the text and flow contrast.

Exact final inspected PDF hashes:

- green5 light: ED7260CE5CE1F884BB1A0043DEC62BBF3B27052DFBFFB56148A9443C2F714DFA.
- green5 dark: 5B2D189B4CCF4A2EF8EC7E740E7F2019A72525BA05617C0BA7557F7D03800093.

This supersedes the prior waiting boundary for crowded flow visual inspection. Actual fixture assertions and invalid-board compiler rejection remain owned by the catalogue/native runtime lane; this read-only visual lane does not assert their terminal status.
Green5 compiled staged semio-viz-flow-sankey.sty SHA256: B1B2BE7403221473633DDFF60B66D7C555DF4CC1E74BE84A264D7C66B81AFC03. Final inspected PDF sizes are 37,234 bytes light and 37,199 bytes dark.
