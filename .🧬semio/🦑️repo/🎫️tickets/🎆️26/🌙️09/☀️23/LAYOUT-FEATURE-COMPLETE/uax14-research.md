# Unicode Line Break Owner Research

Target: Unicode 16.0.0, UAX #14 revision 53 (2024-09-02). Primary inputs are saved in this ticket as LineBreak-16.0.0.txt, EastAsianWidth-16.0.0.txt, LineBreakTest-16.0.0.txt and uax14-53.html. They are research/test inputs, not generated outputs.

The existing pinned Unicode owner supplies general category, combining, bidi, joining, grapheme and script properties, but no Line_Break or East_Asian_Width property. A separate admitted line-break range record will provide those two additional properties. The owner will preserve borrowed source identity, bound property lookup and rule-context work, publish byte offsets, and retire owned cells/results through the current controlled interface. Official positive vectors and explicit malformed/cancelled source refusals will test it before paragraph integration.

Sources: [UAX #14 revision 53](https://www.unicode.org/reports/tr14/tr14-53.html), [Unicode 16 LineBreak data](https://www.unicode.org/Public/16.0.0/ucd/LineBreak.txt), [Unicode 16 EastAsianWidth data](https://www.unicode.org/Public/16.0.0/ucd/EastAsianWidth.txt), [Unicode 16 LineBreakTest](https://www.unicode.org/Public/16.0.0/ucd/auxiliary/LineBreakTest.txt).

The whitespace/hyphen/CJK paragraph tailoring is temporary and must be removed when the new owner is mounted. Native compilation remains unverified through the current repo frontend.

Implemented and verified current default UAX14 owner; see [uax14.md](uax14.md) for complete source ownership, pinned provenance, executed official/independent tests and native limitations.
