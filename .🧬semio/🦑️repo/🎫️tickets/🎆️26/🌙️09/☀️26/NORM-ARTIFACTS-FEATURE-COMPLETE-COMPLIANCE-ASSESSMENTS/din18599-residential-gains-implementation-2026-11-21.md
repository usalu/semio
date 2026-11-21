# DIN V 18599 residential internal heat gains

DIN V 18599-10:2018-09 § 5, Table 4 gives internal heat gains of 45 Wh/(m² NGF·day) for buildings with one or two dwellings and 90 Wh/(m² NGF·day) for buildings with three or more dwellings. The calculation multiplies this rate by the residential zone’s net floor area and the standard monthly calendar days, then converts Wh to kWh. [dena, section 5.1.1 and Table 6](https://www.dena.de/fileadmin/dena/Publikationen/PDFs/2023/LEITFADEN_ENERGETISCHE_GEBAEUDEBILANZIERUNG_NACH_DIN_V_18599.pdf) · [BAFA formula collection](https://www.bafa.de/SharedDocs/Downloads/DE/Energie/qpeb_formelsammlung.pdf?__blob=publicationFile&v=7)

The existing Use class editor field and change event now represent both dwelling categories explicitly. The snapshot schema, TypeScript readers and native and TypeScript SQLite codecs round-trip the two values. The monthly calculation applies the corresponding Table 4 gain in residential zones in both the actual and reference balance. Office and school zones continue to use their entered gain input. The report fails the residential input check when the building has a residential zone but lacks a dwelling classification in the detailed monthly method. This state retains the entered gains for the balance and identifies that basis in the report and chart. The monthly table exposes Q_i independently from solar gains.

| Dwelling classification | Average W/m² | January kWh at 100 m² | February kWh at 100 m² | Annual kWh at 100 m² |
| --- | ---: | ---: | ---: | ---: |
| One or two | 1.875 | 139.5 | 126 | 1642.5 |
| Three or more | 3.75 | 279 | 252 | 3285 |

The language-neutral worked cases are checked against SQLite arithmetic and the compiled production Rust monthly balance. The SQLite schema accepts both profiles and rejects a value outside the enum. The NumPy scene oracle also reproduced the monthly and annual gains. The lightweight harness also checks that residential gains are gated by the building category, method and zone profile. The Bun SQLite suite passed 38 tests, including round-trips for both new use classes, rejection of unknown inputs and the independent physical schema-byte measurement. The native crate test through the plain Cargo tool reached compiler admission and was refused because that route lacked the repository launcher and authentic enforcing store. The repository includes the full report tests for the authorized native build.

This evidence covers the 2018 internal-gain input and monthly accounting. Full Part 2 heat-balance compliance, the complete Part 10 profile tables and the later DIN/TS edition require separate evidence.
