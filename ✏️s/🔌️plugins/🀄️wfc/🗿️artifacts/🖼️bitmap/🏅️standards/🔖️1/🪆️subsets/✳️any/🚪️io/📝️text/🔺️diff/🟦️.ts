/** 📜️ Text facet declaration for `s.wfc.bitmap` diffs. Declaration-only on both sides: a diff is
 * transported as its own record, never as an authored text document, so no grammar is registered
 * for it. */
export type BitmapDiffText = string;
export const BITMAP_DIFF_GRAMMAR_ID = "wfc.bitmap.diff";
