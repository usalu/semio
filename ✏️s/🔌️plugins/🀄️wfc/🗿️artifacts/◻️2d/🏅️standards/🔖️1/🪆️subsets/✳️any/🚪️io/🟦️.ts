// 🚪️ IO mirror for `s.wfc.wfc2d@1/*`. This subset declares NO foreign interchange hop: a WFC problem
// spec has no external format to bridge to — its own DSL and pack envelope ARE the format — so the
// Rust `io()`'s `entries` is empty and so is this mirror. What it does declare is the five native
// languages, which is what a TS caller needs to name a channel.

export interface IoEntryDescriptorMirror {
  from: string;
  into: string;
  fidelity: "Exact" | "Canonical" | "Semantic" | "Lossy";
  sniffs: boolean;
}

export const WFC_2D_DIALECT = "s.wfc.wfc2d@1/*";

/** 🗣️ The five native languages, by `dsl::LanguageRole`. */
export const WFC_2D_NATIVE_LANGUAGES = {
  document: "wfc.wfc2d",
  ops: "wfc.wfc2d.op",
  diff: "wfc.wfc2d.diff",
  pack: "wfc.wfc2d.pack",
  spr: "wfc.wfc2d.spr",
} as const;

/** 🚪️ Deliberately empty — see this module's own header. */
export const WFC_2D_IO_ENTRIES: readonly IoEntryDescriptorMirror[] = [];
