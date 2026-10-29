# Final Sweep: Excel Program (Beat3_Program)

Series: `📊️excel`. Scene: `C:/git/semio/🎓️teaching/🏛️architecture/📊️excel/🎬️clip/📋️program/🐍️.py`, class `Beat3_Program`.
Frames: `T/🗑️generated/frames/📊️excel/Beat3_Program/{en,de}/`, 25 PNGs per language, read in NARRATION order (24 clauses, hook to recap).
Freshness: all 24 clause frames are newer than the scene file (05:03:18). `fill.png` (both languages, 02:07) is an orphan that is not a NARRATION clause and is older than the scene file by about 3 hours. It is not used in the video and should be removed from the frame set.

## Verdict: FIX

### Major

1. **update** (en and de), caption contradicts the picture. The caption says counts, free floors and bars update at once. The frame shows the counts and bars changing, but the free floors (5.14, 4.42, 3.26, 5.14 m²) do not change.
   - Fix en: "Counts and bars update at once. The total grows, because type A has more free floor than type C."
   - Fix de: "Anzahlen und Balken aktualisieren sich sofort. Die Summe wächst, weil Typ A mehr freie Fläche hat als Typ C."

### Minor (student would barely notice; fix if cheap)

2. **xlookup** (en and de): the expanded two-line formula bar overlaps the row 20 headers. "Count"/"Anzahl" and "Free"/"Frei" sit on the bottom edge of the formula box. Fix: shorten the formula display or collapse the formula bar before capture.
3. **countifs** (en and de): the "Net area"/"Nettogrundfläche" header in the matrix's right column has no values under it. It belongs to the towers stage. Fix: drop that header from the countifs stage.
4. **typo** (de): the "Marke = Leerzeichen" label runs into the "139 ≠ 140" chip with no gap. Fix: give the left label a shorter text or more gap.
5. **typo** (en and de): the caption says the capsule turns grey. At this resolution I could not confirm the grey fill, so check by eye on the rendered video.

### Checked and correct

- Numbers: counts 36/35/34/35 (sum 140), Tower A 54 and Tower B 86, net 8.74 m² (shown 8.7), volume 18.354 (shown 18.4), free floors 5.14/4.42/3.26/5.14, SUMPRODUCT 630.5 m², totals 185.0/154.7/110.8/179.9, towers 472.0 + 751.6 = 1,223.6 m², update 632.4 (+1.9), recap 632.4 m² free. All match the `🏢️nakagin` model.
- Check and typo: 140 = 140 TRUE/WAHR, 139 FALSE/FALSCH, fix back to TRUE/WAHR.
- German: narration and captions read correctly. UI names are German in the de frames (Als Tabelle formatieren, Einbauten, Kernseite, Zählenwenns, Summenprodukt, Summewenn). No stray English in the de frames except the Nakagin and Capsule Tower names.
- No `###` on any value, no clipped body text, no empty stage.
