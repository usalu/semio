# Physical Draft Caption Preflight

The previously failing long caption is no longer present. Current physics fundamentals source line1841 reads: Elektroheizstab: am Haus genau 1. Brennstoffe verlieren den Rest mit dem Abgas; Strom kann anderswo Abgas verursachen. It is118 characters, shorter than the reported original.

Shared caption policy remains maximum2 lines, width11.5 scene units, CAPTION_FONT_SIZE24 (canonical textLgPx16 times1.5), fixed line gap UI_SPACING and bottom padding4*UI_SPACING. Exact wrapping depends on rendered Anta glyph widths and word boundaries, so character count alone cannot prove a caption failure.

Static candidate list above110 characters:

- line1163 power prefixes:123 characters.
- line1181 design load vs annual energy:115.
- line1808 heatpump transfers cold-air energy:134.
- line1841 heater/flue-gas distinction:118.
- line2215 long-wave glass/Low-E:122.
- line2227 internal shading:122.
- line2899 condensation at cold surface:134.

These are review candidates, not confirmed blockers. No longer string appears in the simple single-line German narration tuple scan. The134-character heatpump/condensation captions are the most likely next wrap boundaries if the actual full draft fails. No duplicate native rendering or source edits were performed; the running complete draft is the authoritative check.
