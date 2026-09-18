// 🪆️ Subset `s.wfc.wfc2d@1/*` — the TypeScript view of what this subset declares.

export * as schema from "./🧬️schema/🟦️.ts";
export * as io from "./🚪️io/🟦️.ts";
export * as editor from "./✏️editor/🟦️.ts";
export * as viewer from "./👁️viewer/🟦️.ts";

/** 📚️ The three bundled problems, in the order the example picker offers them. */
export const WFC_2D_EXAMPLE_IDS = ["two-room-corridor", "wall-roof-facade-strip", "hex-ring", "terrain-ring"] as const;
