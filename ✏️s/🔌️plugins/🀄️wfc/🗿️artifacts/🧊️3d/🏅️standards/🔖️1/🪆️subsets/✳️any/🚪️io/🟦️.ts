/** 🚪️ wfc3d IO — the native DSL/pack codec plus the one foreign hop this subset registers,
 * `s.stdio.txt@utf-8`, which carries the document's own DSL text losslessly in both directions. */
export const dialect = { artifactKind: "s.wfc.wfc3d", standard: "1", subset: "*" } as const;
export const importStdioKinds = ["stdio.txt"] as const;
export const exportStdioKinds = ["stdio.txt"] as const;
export const nativeLanguages = ["wfc3d.snapshot", "wfc3d.mutations", "wfc3d.diff", "wfc3d.pack", "wfc3d.spr"] as const;
