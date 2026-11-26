# Web surface ownership

## Architecture course

The client has one base surface. The layered overview and its opened quiz pages let the base show through their scrollports, card bodies, lists, task rows and inputs. Neutral segments and inline fields share the same paint. Selection, warning and answer-feedback colors identify state. Sticky leaderboard cells use the inherited surface token to cover content scrolling underneath.

The scrollbar thumb, presence chips, availability chip and reference captions use the shared next-level rule. The workbook grid and Grasshopper figure body inherit the surrounding quiz surface. Native figure semantics and authored diagram and exercise highlighting remain. Portalled course dialogs paint one glass surface behind all dialog content and use the shared veil behind it. The card stack and shared-border compositor own adjacent seams.

## Other web products

The Staffage archive root owns the base surface. Browse and detail pages, viewer and playground controls and ordinary sections inherit it. The portalled Display settings paints one glass menu. Neutral options stay transparent and share their dividers. Timeline markers are glass chips above their authored tracks. The checkerboard on asset imagery indicates transparency.

Hub admin panels and rows inherit the standalone admin base. The separate session status shell uses a window and title chip, with its command set as code on that window's surface.

Play and the Mit Bestand demonstrator now use the shared navbar and footer glass directly. Their acceptance assertions check the shared band level and glass class. Their cards rely on the common layered overview and card stack for ownership and seams.

## Verification

- Chrome through Playwright opened all ten real Architecture Course quiz pages at 1420 × 960 in light and dark. Every page and its four card bodies resolved to base and painted transparent. Each adjacent card pair met at exactly 0 px. Title and action chips resolved to window glass with 0.88 alpha and 8 px blur. Screenshots covered Physics, Excel, Grasshopper and BIM.
- The course browser registered an anonymous learner locally, verified the identity dialog and exercised 12 questions in each appearance across Physics, Energy Demand, Excel and Grasshopper. Chrome resolved the dialog to dialog glass and all ordinary classification, matching and sorting rows and editable guess fields to transparency. Both kinds of reference figure inherited base and their caption chips used window glass.
- The actual Staffage site rendered at 390 × 844 in both appearances. Its root owned base and its menu owned menu glass at 0.40 alpha and 40 px blur. Inactive menu options painted transparent. Keyboard tests checked focus into and out of the menu.
- Course component tests passed 97 assertions covering challenge views, mockups, shared borders, quiz card actions, the course shell and contrast states. The four added task-surface assertions had failed against the previous classification row paint. Staffage passed 47 assertions across viewpoints, settings focus and language selection. Hub admin passed 24 assertions.
- The repository course dev target spent more than seven minutes building a project graph and was cancelled. The browser ran the actual site entries with their owned Vite configs and local application material. The diagnostic loader excluded the course PDF transport plugin so the browser could use the existing offline deputy without provisioning assignment PDFs or a proctor. Assignment documents and server synchronization were outside this browser check. The Play and demonstrator acceptance suites were updated. A fresh release or wasm build for those products is unverified.
- The web-levels-serve.mjs and web-levels-check.mjs scripts reproduce the Chrome checks. The checks write screenshots in the ticket's generated directory.

## Changed paths

Course product 🧰️framework/🛍️products/📚️course

- 🎯️targets/⚛️react/🔨️modules/📚️app/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/🪟️chrome/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/🔓️access/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/👥️presence/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/🧷️scroll-thumb/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/🧩️task/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/🗂️classification/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/↕️sorting/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/🃏️matching/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/🖥️mockup/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/🎬️clips/🟦️.tsx
- 🎯️targets/⚛️react/🔨️modules/📝️assignments/🟦️.tsx
- 🎯️targets/⚛️react/🎨️styles/🏠️chrome-and-home/🎨️.css
- 🎯️targets/⚛️react/🎨️styles/🏁️results-and-preferences/🎨️.css
- 🎯️targets/⚛️react/🎨️styles/🗃️record-tables/🎨️.css
- 🎯️targets/⚛️react/🎨️styles/🖥️mockups/🎨️.css
- 🧪️tests/🖥️mockups/🟦️.tsx
- 🧪️tests/🪜️challenge-views/🟦️.tsx
- 🧪️tests/🧫️fixtures/🌈️surface/🔣️.json

Staffage React target 🎓️teaching/🏛️architecture/🧍️staffage/🎯️targets/⚛️react

- 📦️packages/🟦️typescript/📚️app/🟦️.tsx
- 🔨️modules/🪟️chrome/🟦️.tsx
- 🔨️modules/🥞️levels/🟦️.tsx
- 🔨️modules/🗃️browse/🟦️.tsx
- 🔨️modules/⚖️compare/🟦️.tsx
- 🔨️modules/🎛️viewer/🟦️.tsx
- 🔨️modules/🎛️viewer/🌬️air/🟦️.tsx
- 🔨️modules/🎛️viewer/🧾️reading/🟦️.tsx
- 🔨️modules/🧪️playground/🟦️.tsx
- 🔨️modules/🧪️playground/🎛️asset/🟦️.tsx
- 🔨️modules/🧪️playground/🎞️timeline/🖼️panel/🟦️.tsx
- 🔨️modules/⏯️transport/🟦️.tsx
- 🔨️modules/🔬️detail/🎞️clips/🟦️.tsx
- 🔨️modules/🔬️detail/🧱️tables/🟦️.tsx
- 🔨️modules/🔬️detail/📤️export-panel/🟦️.tsx
- 🧪️tests/⚙️settings-focus/🟦️.tsx
- 🧪️tests/⚙️settings-focus/🥒️.feature

Standalone shells

- 🌎️hub/🔨️modules/🛡️admin/🧱️elements/🛡️AdminApp/🟦️.tsx
- 🌎️hub/🔨️modules/🛡️admin/🧱️elements/🔑️AdminSession/🟦️.tsx
- 🏢️semio-tech/🎡️play/🏠️play-landing.tsx
- 🏢️semio-tech/🎡️play/🧪️tests/🎭️acceptance/🟦️.ts
- ♻️mit-bestand/🧺️demonstrator/🟦️.tsx
- ♻️mit-bestand/🧺️demonstrator/🧪️tests/🎭️acceptance/🟦️.ts

Ticket evidence files are web-levels.md, web-levels-serve.mjs and web-levels-check.mjs.


## Landing page verification

Chromium checked the real Play and Mit Bestand landing pages in light and dark. All 150 Play cards and eight demonstrator cards in the floating overview used dialog glass and menu chips, with a 40 px menu blur in both appearances. The browser used each site's owned production Vite composition in prototype mode with its actual React tree and styles. The development launchers require activation receipts for the embedded applications. The landing test does not cover starting all 158 applications. Reproduce with `web-levels-serve.mjs` for both sites and `landing-levels.mjs`.
