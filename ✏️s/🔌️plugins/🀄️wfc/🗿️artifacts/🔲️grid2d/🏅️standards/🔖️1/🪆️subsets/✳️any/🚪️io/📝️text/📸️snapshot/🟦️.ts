/** 📜️ Text facet declaration for `s.wfc.grid2d` snapshots — the normative grammar lives beside
 * this file as `📖️.grammar.semio`; the TS twin never re-implements the DSL parser, it only names
 * the carrier the Rust facet speaks. */
export type Grid2dSnapshotText = string;
export const GRID2D_SNAPSHOT_GRAMMAR_ID = "wfc.grid2d.snapshot";
export const GRID2D_SNAPSHOT_EXTENSION = "wfcgrid2d";
