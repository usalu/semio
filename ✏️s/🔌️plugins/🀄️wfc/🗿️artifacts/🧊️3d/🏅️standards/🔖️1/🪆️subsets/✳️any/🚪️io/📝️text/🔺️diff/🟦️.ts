/** 🔺️ The wfc3d DIFF has no carrier of its own: a delta travels as its JSON/pack value inside the
 * mutation lane, and the `wfc3d.diff` language slot exists only so the diff facet has a registered
 * role. Stated here rather than left implicit. */
export type Wfc3dDiffText = string;

export const languageId = "wfc3d.diff";
export const hasDedicatedCarrier = false;
