/** 🧪️ Language-agnostic bitmap mount contract — the TypeScript half. It reads the SAME committed
 *  statement `../../🧫️fixtures/🧩️mount-contract/🔣️.json` the Rust and Python halves read, so a
 *  drift in any surface id, window roster or example row fails in every language at once. */

import contract from "../../🧫️fixtures/🧩️mount-contract/🔣️.json" with { type: "json" };

export interface BitmapMountInput {
  readonly editorAppId: string;
  readonly viewerAppId: string;
  readonly artifactKindId: string;
  readonly inferenceToolId: string;
  readonly windowKindIds: readonly string[];
  readonly mutationKinds: readonly string[];
  readonly examples: ReadonlyArray<{
    readonly id: string;
    readonly label: { readonly en: string; readonly de: string };
    readonly inputWidth: number;
    readonly inputHeight: number;
    readonly paletteSize: number;
  }>;
}

export const expected = contract;

/** 🧵️ Lists are compared as sequences, not as sets: the window roster's ORDER is what the layout
 *  binds, so a swap is a real drift and not a cosmetic one. */
function sameSequence(left: readonly string[], right: readonly string[]): boolean {
  return left.length === right.length && left.every((entry, index) => entry === right[index]);
}

/** ✅️ Throws on the FIRST disagreement, naming the field — a silent partial match would let a
 *  renamed window kind through on the strength of the ids around it. */
export function assertBitmapMountContract(input: BitmapMountInput): void {
  const fields: ReadonlyArray<readonly [string, string]> = [
    ["editorAppId", input.editorAppId],
    ["viewerAppId", input.viewerAppId],
    ["artifactKindId", input.artifactKindId],
    ["inferenceToolId", input.inferenceToolId],
  ];
  for (const [field, value] of fields) {
    if (value !== (expected as Record<string, unknown>)[field]) throw new Error(`${field}: ${value}`);
  }
  if (!sameSequence(input.windowKindIds, expected.windowKindIds)) throw new Error(`windowKindIds: ${input.windowKindIds.join(", ")}`);
  if (!sameSequence(input.mutationKinds, expected.mutationKinds)) throw new Error(`mutationKinds: ${input.mutationKinds.join(", ")}`);
  for (const row of expected.examples) {
    const got = input.examples.find((candidate) => candidate.id === row.id);
    if (!got) throw new Error(`missing example ${row.id}`);
    if (got.label.en !== row.label.en || got.label.de !== row.label.de) throw new Error(`example label ${row.id}`);
    if (got.inputWidth !== row.inputWidth || got.inputHeight !== row.inputHeight) throw new Error(`example extent ${row.id}`);
    if (got.paletteSize !== row.paletteSize) throw new Error(`example palette ${row.id}`);
  }
}
