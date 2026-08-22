# P10z Owned Animate Markdown

## Outcome

Animate Present now keeps its existing public MarkdownHtmlCompiler seam while the default implementation is repository-owned. The internal schema is a closed document/block/inline AST; no parser or serializer type is exported.

The owned CommonMark/GFM presentation subset covers paragraphs, ATX headings, emphasis, strong text, inline code, fenced code with language classes, safe links and autolinks, ordered/unordered nested lists, aligned GFM tables, hard breaks, source escaping, raw HTML suppression, and deterministic malformed-input recovery.

Link serialization permits relative targets plus http, https, mailto, and tel. Control characters, backslash/network-path targets, javascript, data, and every other explicit scheme are rejected and rendered as plain label text.

## Differential Evidence

Before removing the installed compiler imports and manifest rows, fixtures were captured from the unified → remark-parse → remark-gfm → remark-rehype → rehype-stringify pipeline.

The owned tests match its exact HTML for representative prose/headings/emphasis/strong/inline-code/HTTPS links, fenced TypeScript, nested plus non-one-start ordered lists, and aligned tables. Escaping/raw-HTML and malformed-input outputs are also fixed in focused tests.

The URL-policy fixture intentionally differs from the installed pipeline: that pipeline emitted live javascript and data anchors; the owned compiler emits their labels without anchors while retaining mailto and relative links.

## Removed Identities

The Animate TypeScript manifest and production source no longer reference:

- unified
- remark-parse
- remark-gfm
- remark-rehype
- rehype-stringify

## Validation

- Focused Nx/Vitest run for owned markdown html compiler and compileMarkdownToHtml: PASS, 8 focused tests.
- Focused strict TypeScript compile of the owned module: PASS.
- Exact Animate source/manifest identity search: PASS, zero matches.

The full Animate suite, plugin/freeze audit, workspace parity suite, and Cargo gates were deferred as directed. P4 retains the Cargo lane.

## Files

- ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🔨️modules/📝️markdown-html-compiler/🟦️component.ts
- ✏️s/🔌️plugins/🎞️animate/🗿️artifacts/🎬️present/🏅️standards/🔖️1/🪆️subsets/✳️any/✏️editor/📺️renderer/⚛️react/🟦️component.tsx
- ✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/package.json
- ✏️s/🔌️plugins/🎞️animate/📦️packages/🟦️typescript/🧪️vitest.config.ts
