/** ⚖️ One wfc3d mutation's BINARY state-patch encoding. The tag is the operation's position in the
 * variant table the text keywords come from, so a kind can never carry one tag on the wire and
 * another in the grammar. */
export type Wfc3dMutationBinary = Uint8Array;

export const languageId = "wfc3d.spr";
export const protocolId = "wfc3d.mutations";
