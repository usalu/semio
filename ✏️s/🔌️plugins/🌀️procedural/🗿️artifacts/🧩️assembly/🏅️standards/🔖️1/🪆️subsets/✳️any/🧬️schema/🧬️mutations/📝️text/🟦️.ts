/** ⚡️ Text representation for `procedural.assembly.mutations` — one operation per line. */
export type AssemblyMutationText = string;

/** 🏷️ Every mutation keyword this grammar admits, in the catalog's own declaration order. */
export const assemblyMutationKeywords = [
  "create-slot",
  "delete-slot",
  "create-rule",
  "delete-rule",
  "change-weight",
  "remove-weight",
  "connect-slots",
  "disconnect-slots",
  "change-seed",
] as const;

export type AssemblyMutationKeyword = (typeof assemblyMutationKeywords)[number];

/** 📖️ Reads the keyword one operation line declares, or `undefined` for a line no kind claims. */
export function assemblyMutationKeywordOf(line: AssemblyMutationText): AssemblyMutationKeyword | undefined {
  return assemblyMutationKeywords.find((keyword) => line === keyword || line.startsWith(`${keyword} `));
}
