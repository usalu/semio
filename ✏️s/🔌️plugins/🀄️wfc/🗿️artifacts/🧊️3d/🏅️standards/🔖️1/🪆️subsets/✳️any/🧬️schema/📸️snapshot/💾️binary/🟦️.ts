/** 📦️ The wfc3d document's BINARY carrier — the envelope-wrapped pack body. Opaque bytes on this
 * side: the normative layout is `📡️.protocol.semio` beside it. */
export type Wfc3dSnapshotBinary = Uint8Array;

export const envelopeId = "wfc.wfc3d";
export const protocolId = "wfc3d.snapshot";
export const languageId = "wfc3d.pack";
