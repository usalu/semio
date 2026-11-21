# EN 1993-1-9 fatigue assessment and detail tables

## Edition and scope

The checks in this work use first-generation EN 1993-1-9:2005 with AC:2009. Its §1.1 excludes fracture-mechanics assessment. §3 defines the damage-tolerant and safe-life approaches, Annex A gives fatigue assessment for variable amplitude, and Annex B defines geometric hot-spot resistance. The 2025 edition also excludes fatigue design based on fracture mechanics and assigns Annexes C and D to effective notch stresses and geometric stress concentration. The trace now assigns V-506 to the applicable design-method and inspection provisions of §3. The inspection and maintenance evidence remains an explicit gap. [2005 edition](https://www.phd.eng.br/wp-content/uploads/2015/12/en.1993.1.9.2005-1.pdf#page=8) · [2025 edition preview](https://webstore.ansi.org/preview-pages/BSI/preview_30397910.pdf)

## Production behavior

The document fatigue checks and diagrams now select the Table 3.1 resistance factor using both the recorded method and the consequence class. V-502 reaches the fatigue-limit, spectrum-damage and load-model stress-range checks. The bridge checks and diagram share the same selector for EN 1993-2:2006 and retain the existing unverified German bridge factor pending the application-specific annex classification. The general German annex confirms the four Table 3.1 values for cases outside the application-specific Parts 2 to 6. [Table 3.1](https://www.niedersachsen.de/download/80810/nds._mbl._nr._37_2012_vom_30.10.2012_anlagenband_37i.pdf#page=264) · [DIN EN 1993-1-9/NA:2010-12](https://www.niedersachsen.de/download/80810/nds._mbl._nr._37_2012_vom_30.10.2012_anlagenband_37i.pdf#page=296) · [EN 1993-2:2006 §9.3](https://www.phd.eng.br/wp-content/uploads/2015/12/en.1993.2.2006.pdf)

C93-16 covers the seven weld-toe categories in Table B.1. It checks fabrication evidence, inclusion of misalignment in the supplied structural stress, the applicable maximum toe angle, the equivalent 2-million-cycle fatigue range, and the frequent-load direct stress limit. The case requires a project-supplied structural stress. C93-17 applies the k₁ factor from Tables 4.1 and 4.2 to the supplied pinned-model stress range. It covers all 24 cells, including the four unavailable K-joint verticals, with the corrected geometry limits and a project-supplied qualified Table 8.7 detail category. [Table B.1](https://www.niedersachsen.de/download/80810/nds._mbl._nr._37_2012_vom_30.10.2012_anlagenband_37i.pdf#page=291) · [Tables 4.1 and 4.2](https://www.niedersachsen.de/download/80810/nds._mbl._nr._37_2012_vom_30.10.2012_anlagenband_37i.pdf#page=264)

## Independent checks

- `check-en1993-fatigue-factors.py` ran production Rust factor selection and damage equations against SQLite. All 320 combinations of method, consequence, annex, stress range and repetitions matched.
- `check-en1993-fatigue-catalogue.py` ran the two production evaluator functions against independent SQLite calculations. All 31 table cells and 265 cases matched, including documented scope, missing table values and angle and geometry limits.
- The document test now drives all four factors through both annexes. Two new catalogue tests use the seeded UI input definitions and shared report route. The full native run and new browser verification are pending.
- V-505 still requires the nominal-stress detail conditions in Tables 8.1–8.10 and V-609 still requires the advanced fire method.
