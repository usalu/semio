/** 🚪️ IO s.wfc.bitmap (1/✳️any) — the foreign hops this subset registers, mirrored for the
 * cross-language oracle. Both hops are `Exact`: json is a typed field-for-field projection and txt
 * is the artifact's own DSL text. */
export const BITMAP_IMPORT_STDIO_KINDS = ["stdio.txt", "stdio.json"] as const;
export const BITMAP_EXPORT_STDIO_KINDS = ["stdio.txt", "stdio.json"] as const;
export const BITMAP_NATIVE_EXTENSION = "wfcbitmap";
export const BITMAP_NATIVE_LANGUAGE_IDS = ["wfc.bitmap", "wfc.bitmap.op", "wfc.bitmap.pack", "wfc.bitmap.spr"] as const;
