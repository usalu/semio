# Outer and Inner Cycle Content and English Native QA

The German and English content sources remain frozen. Both already instruct students to complete the twenty marked directed edges, reverse numeric signs and categorical arrows on return, and check a closed cycle. The final four selected edges now form the additional clockwise inner cycle 1→5→10→13→1. No production text edits were necessary.

## Frozen Source Verification

| Source | SHA-256 |
| --- | --- |
| German | 6c48d0b5f438dcb8a65cb3c434689db506ea175824ff80519ecab7fb11e6e07d |
| English | f163181945152117051c072a0b9f743b3897467be11db9793d70784a95877b84 |

Independent source checks confirm six Form captions and four two-line vector captions, two explicit page breaks, one main section and two subsections, both capsule matrices and one selected ledger. There is no frontmatter. Box and pyramid retain (2.5,1.5,2), while cylinder retains (3.5,2.5,3); each numeric row therefore has two shared values and one distinct value. The directed K3 vector and categorical changes remain correct.

## Native Verification

The original English main compiled successfully through registered Tectonic 0.16.9 using the canonical reference-only compiler input, pinned local bundle, product classes and verified fonts. The final PDF has three pages, 133617 bytes, title Finger Exercise: Parametrization, author Architecture, en-US metadata, and zero overfull or underfull boxes. Both frozen content hashes remained identical after native compilation and rendering.

The retained inner-content-check.py independently expects all twenty ordered labels, including 16→1 and the four inner-cycle arrows. It verifies English metadata, preserved content, blank writing regions, at least 3 mm inter-column space, Anta ledger digits and CMSY arrow glyphs, no box diagnostics, and no extracted characters beyond page bounds.

The latest v11 directed PDF oracle passes all 240 base cubic arcs, twenty selected directions and 266 native arrowheads including K3. The visible graph height is 125.603 mm, with vertical vertex radius 53 mm. The ordered ledger contains the complete outer cycle followed by 1→5, 5→10, 10→13 and 13→1.

The independent matrix and mixed-form PDF oracles pass. Both nine-by-nine matrices remain on page two, with 6.14 mm writing rows and artwork minimum padding 2.13–2.23 mm. Worked sample values, native box/pyramid/cylinder geometry, the nominal 20 mm header plus rule, six directed K3 transitions and four signed vector captions are verified.

The independent English checker passes all twenty blank writing rules and ordered labels. Minimum blank rule length is 64.60 mm; minimum inter-column gutter is 4.08 mm. All nine unique PDF font resources embed their actual fonts, including descendant descriptors for composite fonts. Ledger digits use Anta-Regular and arrows use CMSY10. No extracted text lies beyond a page.

## Individual Full-Page Visual Review

All three freshly rendered PDFium PNGs were individually inspected. Page one retains the three correct sample values and shapes, all six readable categorical captions, four complete two-line vectors and the three criteria. Page two has both complete compact capsule matrices, centered vector symbols and eight blank writing rows each.

Page three clearly shows two clockwise closed cycles: outer 1→2→…→16→1 and inner 1→5→10→13→1. Each selected arrowhead is visible, and the last four ledger labels match the inner cycle in order. The complete faint directed base graph remains. All twenty writing rules and the final cycle prompt fit, without clipping or collisions.

Only ticket inputs/report were authored by this role: inner-content-check.py and inner-content.md. No production content or geometry was modified. Temporary compiler outputs, logs, cache and review PNGs remain under the ticket generated directory for parent integration and cleanup.
