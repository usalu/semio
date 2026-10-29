# Final Sweep: Excel, Beat4_Energy (read-only)

Verdict: **FIX**

Scope: all 24 clauses in both languages, read in narration order from `🗑️generated/frames/📊️excel/Beat4_Energy/{en,de}/`. Scene file `🎓️teaching/🏛️architecture/📊️excel/🎬️clip/⚡️energy/🐍️.py` last written 05:10:59; every PNG is newer (05:11 to 05:27), so no stale-frame note applies.

Numbers checked against the `🏢️nakagin` model (authoritative): net floor 8.74 m², window U 2.33 W/(m²·K), b 0.50 (assumed), 140 capsules (36/35/34/35), HDD 1,586 K·d, H_T 30.92 W/K, H_V 3.12 W/K, Φ 681 W, Q 1,296 kWh/a, 148 kWh/(m²·a), building 182,901 kWh/a, what-if 173,115 kWh/a (-5.4 %), type C 153 kWh/(m²·a). All consistent. Captions match the pictures in both languages, and the German video contains no English beyond Excel UI names.

## Defects (blockers and majors)

1. **ht, en and de. MAJOR, wrong value on a factor.** The formula note reads "b for the back wall: 1.88 W/K" (de: "b für die Rückwand: 1,88 W/K"). b is dimensionless and is 0.50 on the Climate sheet. 1.88 W/K is the back wall's U·A.
   - Fix: in `🐍️.py` line 558 (`note = self.tr(...)`), replace the value with the factor: "b = 0.50 for the back wall" / "b = 0,50 für die Rückwand", or drop the value.

2. **compare, en and de. MAJOR, value without object.** Under each capsule the bare "1.3 m" (de "1,3 m") has no label. A student cannot tell it is the window diameter.
   - Fix: in `_types_row` (line ~531), label the value "window ⌀ 1.3 m" / "Fenster ⌀ 1,3 m", or add one "window ⌀" caption at the row's left edge.

3. **compare, en and de. MAJOR, text outside its panel.** The y-axis unit "kWh/(m²·a)" starts at about x 812 px, while the white chart panel starts at about x 837 px. Half the label sits on the dark background.
   - Fix: keep the axis unit inside the `xl_bar_chart` panel bounds (visuals module), for example by moving it under the title.

4. **trace, en and de. MAJOR, overlapping text.** The dependency connectors run straight through the row-4 values: the dashed arrow crosses "36" in B4, and the blue arrow strikes "1,296 kWh/a" in C4 and "46,638 kWh/a" in D4 (de: "1.296 kWh/a", "46.638 kWh/a"). Same overlap in **whatif, frozen and repaired, en and de**, where "1,233 kWh/a" / "1.233 kWh/a" and "44,402" / "44.402" are struck.
   - Fix: in `xl_trace`, route the B4→C4 and C4→D4 connectors above or below the text baseline (about half a row offset), or draw them behind a background-coloured plate behind each value.

## Verdict

FIX: four items above, each in both languages (ht, compare, trace, whatif/frozen/repaired). No other blocker or major found in the 24 clauses.
