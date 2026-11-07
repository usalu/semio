# UI Border Exploration

The general UI lives in 🧰️framework/🔨️modules/🖱️ui. Its AGENTS file describes pure, business-logic-free components.

🔨️modules/📏️border-presentation/🟦️.ts resolves exact collinear border spans, with deterministic owner and ID tie breaks and state precedence from normal to hover. 🧱️elements/🔲️WindowSilhouette/shared-border.ts projects and clips outline geometry and resolves junctions. 🧱️elements/🗂️WindowChrome/shared-border.ts implements a root-local compositor with SVG frame overlays, observers, focus and hover tracking, transforms and clipping. Adjacent schemas, cases and tests cover these mechanisms.

🎨️styling/🖌️ui/🎨️.css also implements shared cell dividers, hover precedence and suppression of underlying attached borders. 🧱️elements/⚡️ActionGroup/🟦️.tsx registers frames and marks shared cells. Cell CSS uses sibling relationships and separate dividers rather than registering those seams with the compositor. This distinction deserves end-to-end coverage where cell dividers meet another component or where their focus, status or hover state competes with neighboring edges.

The compositor discovers only marked frames and registered window silhouettes within its root. Unmarked application-level bordered surfaces and surfaces outside that root cannot participate automatically. Audit shell consumers and non-window controls for explicit registration and verify contact at their actual rendered boundaries, including clipping and scrolling. These are potential integration gaps, not confirmed visual defects.

## Scoped Consumer Audit

The semio-tech play TSX sources contain no literal border classes; 🏢️semio-tech/🎡️play/🟦️.tsx:200 and :243 use gaps but do not establish adjacent bordered controls. No actionable omitted border was confirmed in that consumer.

In OS renderer 🏘️SpaceBrowser/🟦️.tsx:268, each space row has its own rounded border; the list at :356 uses gap-1, so rows are deliberately separated, not coincident. Inputs at :93 and :100 are in separately gapped form columns (:90); they likewise do not establish coincident borders. 💬️AgentChatPanel/🟦️.tsx:162-166 has a header bottom border, followed by a feed with horizontal dividers; the panel parent at :161 has gap-single, so these are separated too. ✏️TextEditor/🟦️.tsx:661-676 uses separate top borders on conditionally rendered bands, but padding inside the bands separates their opposite boundaries. These scoped findings do not establish a missed coincident seam.

Other renderer overlays and sign-in form controls found during the scan have gaps, padding or absolute positioning; no omission should be inferred solely from a border class. Dynamic adjacency would require a rendered example or an explicit zero-gap layout.
