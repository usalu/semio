/** 🧪️ Language-agnostic bitmap mount contract — the TypeScript reader. It reads the SAME committed statement
 *  `shared://🧩️mount-contract/🔣️.json` the Rust and Python halves read and checks its canonical forms, so a drift in any
 *  surface id, window roster or example row fails in every language at once. SUBJECT role: a second reading of the
 *  statement, never a reference for the Rust half, which measures the real surfaces. */

import { defineTestAdapter } from "../../../../../../../../../../../🧰️framework/🛍️products/🦑️repo/🔨️modules/🧪️test/📦️packages/🟦️typescript/🟦️.ts";

//#region 🧬️Statement
type MountContract = {
  readonly editorAppId: string;
  readonly viewerAppId: string;
  readonly dialect: string;
  readonly artifactKindId: string;
  readonly inferenceToolId: string;
  readonly windowKindIds: readonly string[];
  readonly mutationKinds: readonly string[];
  readonly examples: ReadonlyArray<{ readonly id: string; readonly label: { readonly en: string; readonly de: string }; readonly inputWidth: number; readonly inputHeight: number; readonly paletteSize: number }>;
};

const STATEMENT = "shared://🧩️mount-contract/🔣️.json";
const CANONICAL_IDS = { editorAppId: "s.wfc.bitmap@1/*#editor", viewerAppId: "s.wfc.bitmap@1/*#viewer", artifactKindId: "2d.wfcbitmap", inferenceToolId: "s.wfc.bitmap.solve" } as const;
const CANONICAL_WINDOWS = ["wfc-bitmap-input", "wfc-bitmap-output"] as const;
//#endregion 🧬️Statement

//#region 🧭️Adapter
/** 🧵️ Lists compare as sequences: the window roster's ORDER is what the layout binds. */
function sameSequence(left: readonly string[], right: readonly string[]): boolean {
  return left.length === right.length && left.every((entry, index) => entry === right[index]);
}

export default defineTestAdapter({
  implementation: "typescript",
  scenarios: {
    "surface-ids": {
      subject: (ctx) => {
        const contract = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(STATEMENT))) as MountContract;
        for (const [field, value] of Object.entries(CANONICAL_IDS)) if (contract[field as keyof typeof CANONICAL_IDS] !== value) throw new Error(`surface-ids: ${field} is ${contract[field as keyof typeof CANONICAL_IDS]}`);
        if (contract.dialect !== "s.wfc.bitmap@1/*") throw new Error(`surface-ids: dialect is ${contract.dialect}`);
        return { projection: { ...CANONICAL_IDS } };
      },
    },
    "window-kinds": {
      subject: (ctx) => {
        const contract = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(STATEMENT))) as MountContract;
        if (!sameSequence(contract.windowKindIds, CANONICAL_WINDOWS)) throw new Error(`window-kinds: ${contract.windowKindIds.join(", ")}`);
        return { projection: { editor: [...contract.windowKindIds], viewer: [...contract.windowKindIds] } };
      },
    },
    "mutation-vocabulary": {
      subject: (ctx) => {
        const contract = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(STATEMENT))) as MountContract;
        if (new Set(contract.mutationKinds).size !== contract.mutationKinds.length) throw new Error("mutation-vocabulary: the roster names a kind twice");
        return { projection: { mutationKinds: [...contract.mutationKinds] } };
      },
    },
    examples: {
      subject: (ctx) => {
        const contract = JSON.parse(new TextDecoder().decode(ctx.fixtureBytes(STATEMENT))) as MountContract;
        for (const row of contract.examples) if (row.inputWidth <= 0 || row.inputHeight <= 0 || row.paletteSize < 2 || !row.label.en || !row.label.de) throw new Error(`examples: ${row.id} is degenerate, single-coloured or not localized`);
        return { projection: { examples: contract.examples.map(({ id, inputWidth, inputHeight, paletteSize }) => ({ id, inputWidth, inputHeight, paletteSize })) } };
      },
    },
  },
});
//#endregion 🧭️Adapter
