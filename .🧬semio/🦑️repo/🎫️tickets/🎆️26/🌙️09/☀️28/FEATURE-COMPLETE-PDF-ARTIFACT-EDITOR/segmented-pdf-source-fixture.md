# Segmented PDF Source Fixture

The schema-first fixture defines an ordinary one-page PDF 1.7 source of 9,437,877 bytes. The 3072 × 3072 grayscale image uses four distinct sample values. The cross-reference table starts beyond the 8 MiB command ceiling so a source reader must reach the tail even to discover the page. Ajv validated the fixture. MuPDF opened the original without repairing it, reported the full image dimensions and independently rendered all four samples. A reference 90-degree rotation passed the saved-image, page-box and pixel checks.

The oracle is ready for the actual Browser and native deliveries. The current ordinary MediaIn route rejects this specific source before reading the body.

- [Fixture schema](🔣️segmented-pdf-source.schema.json)
- [Fixture data](🔣️segmented-pdf-source.json)
- [PDF generation and MuPDF oracle](📜️pdf-segmented-source-oracle.py)
