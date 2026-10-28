# Named Delta Ledger Widths

Plain Anta text is suitable for the named changes. Keep the existing 9 pt font, 64.6 mm writing rules, 6.3 mm row pitch, and two-column exercise layout. No shared styling change or font reduction is needed.

## Measurement

`delta-width.verify.py` measures the existing Anta-Regular.ttf with Pillow FreeType at a 900-unit reference size and independently with ReportLab. The two measurements agree within 0.05 pt. The model preserves ordinary spaces and semicolons and substitutes a conservatively sized native PDF math-arrow advance. Using 9 PostScript points rather than the slightly smaller native TeX point makes the fit conservative as well.

At 9 pt, the longest German complete line is `L+1; B+1; H+1; Pyramide → ellipt. Zylinder`: 60.28 mm, leaving 4.32 mm within its rule. The longest English line is 58.20 mm. Negative entries are shorter with the plain native hyphen, and the two form-only entries are substantially shorter. All twelve lines fit without changing existing form phrases.

Use plain `L+1; B+1; H+1;` and `L-1; B-1; H-1;` before the existing form-change phrase; categorical-only entries omit the numeric prefix entirely. Retain the existing math arrow between form names. No production source was edited by this investigation.

The authored measurement script is retained at the ticket root. Its disposable numerical output is under `🗑️generated/delta-widths`. Final native PDF page-one inspection follows the root-owned build.
