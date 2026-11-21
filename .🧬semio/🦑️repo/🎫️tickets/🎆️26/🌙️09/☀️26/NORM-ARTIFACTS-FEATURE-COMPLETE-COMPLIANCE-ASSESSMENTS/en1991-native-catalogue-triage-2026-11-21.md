# EN 1991 native catalogue triage

The first run reached both packages and compiled. The checks package reported 38 passing and seven failing tests, so Cargo exited before it ran the companion package. The companion package passed all eight targeted catalogue cases on its separate run. The native compliance integration test for an unset National Annex parameter passed separately.

## Corrected calculations

- German floor-area reductions now use the published equations (6.1a DE) and (6.1b DE). Area divides 10 m² directly. The exempt and eligible occupancy categories use different factors. The floor-load check applies the area factor for a secondary member. The storey factor for vertical members is available separately. Six German annex constants have verified source records. [DIN EN 1991-1-1/NA:2010-12, printed pages 8–9](https://www.niedersachsen.de/download/80784/Nds._MBl._Nr._37_2012_vom_30.10.2012_Anlagenband_01.pdf)
- The snow load return-period factor uses the published Eq. (D.1) coefficients. It requires a return period of at least five years. The construction-stage calculation still reports the recommended short return period and wind factor and flags a separate project assessment in place of a two-year snow value. [DIN EN 1991-1-3 Annex D](https://www.niedersachsen.de/download/80801/Nds._MBl._Nr._37_2012_vom_30.10.2012_Anlagenband_02.pdf) · [EN 1991-1-6 Table 3.1](https://www.phd.eng.br/wp-content/uploads/2015/12/en.1991.1.6.2005.pdf)
- The crane wheel-load calculation accepts zero for an isolated dead-weight or hoist contribution and rejects negative contributions and fractional wheel counts.
- Catalogue explanations delegate evidence-status reporting to the shared check builder, which has access to the active annex and registry.

## Further findings

The missing Python interpreter is a test environment requirement. The SEMIO_PYTHON variable was absent and a repository virtual environment was unavailable in the build snapshot. The information-row test expected Pass for a row based on an unverified table entry. A separate check of zero utilization and preservation of the reported value is needed.

The German fire-design National Annex permits the annex AA model instead of EN Annex A and requires BB in place of E. It excludes the equivalent duration from EN Annex F. The EN parametric examples and their dedicated fixtures now select the EN basis. German nominal documents retain the nominal gas and flux checks and report the fire-load assessment with its BB basis. German parametric documents surface separately that Annex AA is needed. [DIN EN 1991-1-2/NA:2015-09](https://www.niedersachsen.de/download/140002/Nds._MBl._Nr._3_2019_vom_24.01.2019_Anlagenband_1-Teil_1.pdf)

The 21 language-neutral fire document outcomes matched the updated Python oracle. A separate full native retry reached the shared Store engine during concurrent transport work and failed on its evolving APIs and an undeclared JSON workspace dependency. The bundle now uses the same repository-relative codec dependency as adjacent Store packages. Native checks on the current source are running.
