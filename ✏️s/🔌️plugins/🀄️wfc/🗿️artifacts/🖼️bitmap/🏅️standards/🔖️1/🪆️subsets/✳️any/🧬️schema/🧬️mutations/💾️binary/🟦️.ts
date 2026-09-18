/** ⚖️ Binary facet declaration for `s.wfc.bitmap` mutations. A kind's wire tag is its position in
 * the shared variant table, so the text keyword and the binary tag can never disagree. */
export type BitmapMutationBinary = Uint8Array;
export const BITMAP_MUTATIONS_PROTOCOL_ID = "wfcbitmap.mutations";
export const BITMAP_MUTATIONS_SPR_LANGUAGE_ID = "wfc.bitmap.spr";
