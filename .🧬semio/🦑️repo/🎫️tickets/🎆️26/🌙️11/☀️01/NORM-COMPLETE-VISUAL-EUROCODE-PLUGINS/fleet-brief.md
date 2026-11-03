# 🏗️ Fleet Brief — Norm-Complete Visual Eurocode Plugins

Ticket: `26/11/01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS` (goal `norm`).
Ticket folder (TICKET): `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️11/☀️01/NORM-COMPLETE-VISUAL-EUROCODE-PLUGINS`

## Objective
The Eurocode plugins (EN 1990–EN 1999) are ad hoc. Make them:
1. **Norm complete**: every verification and clause of every part relevant to the norm, including the German national annex (NA-DE) NDPs.
2. **Visual**: 3D wherever possible (members, sections, joints, foundations, load paths, utilisation heat maps, failure modes), and 2D diagrams otherwise (interaction curves, M-N diagrams, stress blocks, spectra, S-N curves).
3. **Rich, immediate, real-time editors**: direct manipulation in viewports, live re-evaluation on every edit, inline utilisation feedback, remedies, no raw JSON.

## Where things live
- Norm plugin root: `✏️s/🔌️plugins/📕️norm` (read its `AGENTS.md`).
- Eurocode artifacts: `✏️s/🔌️plugins/📕️norm/🗿️artifacts/<emoji>enXXXX` with the emojis `⚖️en1990 🏋️en1991 🏛️en1992 🔩️en1993 🧩️en1994 🪵️en1995 🪨️en1996 🌍️en1997 🫨️en1998 🪶️en1999`.
- Per artifact: `🏅️standards/🔖️1/🪆️subsets/✳️any/` → `🧬️schema/` (`📸️snapshot` model, `💡️inferences`, `🧬️mutations`, `🔺️diff`), `✏️editor/` (commands, field-meta, panels, `🎭️modes/✏️edit/🪟️windows/{📥️inputs,📊️results}`), `👁️viewer/` (`📊️report` window), `🚪️io`, `🔮️oracles` (Python third-party oracles), `🧪️tests`, `📚️examples`.
- Shared norm UI surface: `✏️s/🔌️plugins/📕️norm/📇️registry/🧬️contract/🖥️app-surface` (the current editors render the document as JSON through `render_document_editor`).
- Shared compliance kernel: `✏️s/🔌️plugins/📕️norm/⚖️compliance`.
- Framework: `🧰️framework`; other plugins with 3D: `✏️s/🔌️plugins/{🏗️fem,🧱️block,💠️lowpoly,📐️cad,🏙️bim,🪐️space}`.

## Repo rules (from /AGENTS.md — mandatory)
- Never run modifying git commands (commit/stash/checkout/reset…). Others work on the same files simultaneously; do not revert their changes; keep going.
- Build/test only through the repo MCP tools (`mcp__repo__run`, `mcp__repo__test`, …) or the `repo` binary, never cargo/nx/bazel directly.
- Schema-first, test-driven, at least one language-agnostic test per feature, validate against a third-party library (Python oracles).
- No runtime external libraries; no comments inside definitions; docstrings start with a unique emoji; concise code; English + German labels (`LocalizedLabel::native(en, de)`).
- Event-sourced CQRS, no CRUD; no legacy, no compatibility layers, no migration scripts.
- File paths must stay below 256 characters.
- Write every research/summary result as markdown in the ticket folder; generated logs go to `TICKET/🗑️generated/`.
