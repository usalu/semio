/** 🔺️ The wfc3d DIFF has no dedicated binary protocol — see the `📝️text` sibling. A delta rides the
 * document's own pack encoding as a value; nothing frames it separately. */
export type Wfc3dDiffBinary = Uint8Array;

export const hasDedicatedCarrier = false;
