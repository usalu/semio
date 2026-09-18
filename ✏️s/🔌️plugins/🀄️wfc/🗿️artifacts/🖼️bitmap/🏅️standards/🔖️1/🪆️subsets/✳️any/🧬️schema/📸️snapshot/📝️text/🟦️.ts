/** 📜️ Text facet declaration for `s.wfc.bitmap` snapshots — the normative grammar lives beside this
 * file as `📖️.grammar.semio`; the TS twin never re-implements the DSL parser, it only names the
 * carrier the Rust facet speaks. */
export type BitmapSnapshotText = string;
export const BITMAP_SNAPSHOT_GRAMMAR_ID = "wfcbitmap.snapshot";
export const BITMAP_SNAPSHOT_EXTENSION = "wfcbitmap";
