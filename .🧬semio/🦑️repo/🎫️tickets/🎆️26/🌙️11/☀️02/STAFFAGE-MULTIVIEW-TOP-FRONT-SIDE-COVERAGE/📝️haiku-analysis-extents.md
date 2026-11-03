# 📝️ haiku-analysis-audit: Extent Failure Classification (relayed by the coordinator)

This report covers 211 `extent-*` occurrences across 53 assets in `🗑️generated/contracts/failures.json`. It is static: real-world sizes come from memory and were not web-verified.

The four classes:
- (a) the drawing is off;
- (b) the declared dimension is implausible;
- (c) a held item, attachment, plate or lid is counted in the extent;
- (d) noise, below 2 %.

| Class | Occurrences | Assets | Examples |
|---|---|---|---|
| (a) drawing off | 131 | ~40 | Humans are about +4 % lateral and −5 % forward. The drawings of jogger, runner-with-blade-prosthesis and person-with-umbrella differ between bands, which is a defect. date-palm and person-with-rollator are off by 2–3 %. hornbeam-hedge is drawn 7 % thicker. |
| (b) declared value implausible | 22 | 6 | sleeping-person `widthM` 1.9; miscanthus `heightM` 1.75 against the 1.6 in its own summary (the drawing matches 1.6); retractable-bollard `depthM` 0.4 against its 0.20 summary; lombardy-poplar widths (check before changing); roe-deer depth (low confidence) |
| (c) semantic | 58 | ~20 | umbrella, crutches, cane, hand truck, box, toddler; steel-column base and head plates (+30–40 %); bicycle-stand footing (+41 %); room-thermostat wall plate (+63 %); waste-bin lid; charcoal-grill handle |
| (d) under 2 % | 0 | 0 | — |

A suspicious pattern: many assets measure exactly about 20 mm over the declared size (0.520 against 0.50 on tour-guide, walking-commuter, operating-table and waste-bin). Contracts must confirm that extents are measured on the geometry, not on the stroke.

## Coordinator rule D18

- **`dimensions`** is the overall physical envelope of everything that belongs to the asset in its rest pose: held items, attachments, plates, flanges, lids and footings. It excludes stroke width, ground shading, plumes and annotations.
  - For class (c), correct the declared `dimensions` to the envelope.
  - The body-only measure may be recorded in the asset's summary text.
- **Tolerance:** an extent passes when |measured − declared| ≤ max(2 % of declared, 15 mm). This must be stated exactly in the validator message.
- **Class (a):** fix the drawing. Drawings of the same view in different bands must agree in extent within the tolerance.
- **Class (b):** change the declared value only with physical evidence (web source or catalog data), cited in the asset's `sources` and references.
