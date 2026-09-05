# IO and Compiler Hand Review

The exact scopes are framework `🚪️io` and `📚️compiler`; no scoped AGENTS files exist within them. Every physical entry was reviewed, including binary fonts and license texts. No generated or dependency subtree was omitted.

IO's existing names distinguish artifact/dialect schema from the strict RFC4648 Base64 codec. Its single Rust leaves and neutral JSON fixture are sibling-unique. Two stale documentation references to `🦀️component.rs` now name the actual schema leaf `🦀️.rs`; executable behavior did not change.

Compiler's root directories already distinguish syntax, font world, text shaping, mathematical layout, SVG output, and package glue. Its font directory contained five identical font emojis and two identical license emojis. The exact reserved-name contracts were checked before editing: `LibertinusOFL.txt` and `NotoOFL.txt` are already registered literal font-license names. No replacement license convention or unnecessary family-directory grouping was introduced.

| Old name under compiler/world/fonts | New name | Handpicked meaning |
| --- | --- | --- |
| 🔤️LibertinusMath-Regular.otf | 🧮️LibertinusMath-Regular.otf | Mathematical glyphs and OpenType MATH tables |
| 🔤️LibertinusMono-Regular.otf | ⌨️LibertinusMono-Regular.otf | Monospaced code/typewriter text |
| 🔤️LibertinusSerif-Regular.otf | 📖️LibertinusSerif-Regular.otf | Upright serif reading text |
| 🔤️LibertinusSerif-Italic.otf | 🖋️LibertinusSerif-Italic.otf | Italic emphasis/scripted letterforms |
| 📜️LibertinusOFL.txt | LibertinusOFL.txt | Existing reserved license filename |
| 📜️NotoOFL.txt | NotoOFL.txt | Existing reserved license filename |

The sole remaining `🔤️NotoColorEmoji-subset.ttf` keeps its meaningful font marker and is now sibling-unique. SFNT name tables confirmed all five actual font families/styles before the choices; Libertinus Math has MATH, while Noto has CBDT/CBLC bitmap-color tables. The two distinct copyright/license headers were inspected. All five font files and both license texts retain their original SHA-256 values across the six exact non-overwriting moves. Four exact `include_bytes!` paths changed; FontRole identities and embedded bytes did not. A stale syntax-documentation link was corrected to its existing `📖️.grammar.semio` sibling.

Final physical audit: IO 11 entries / 10 governed; compiler 24 / 21. Both have zero missing, multiple, generic, sibling-duplicate, or unresolved-directory-role findings. The two literal license filenames are reserved by their actual existing fonts-scope contracts, not a blanket exclusion.

Native verification through the workspace Nx runner passed 54 compiler tests and four Base64 tests. Compiler tests exercised the actual moved embedded fonts, glyph shaping/outlines, MATH layout, syntax, and SVG output. Base64 tests exercised the language-neutral RFC4648 corpus and independent third-party Base64 oracle. The broad IO dispatch suite was not separately rerun for its documentation-only change. Scoped whitespace checks passed. No font bytes, license text, public API, or test assertions changed; no modifying Git commands were used.
