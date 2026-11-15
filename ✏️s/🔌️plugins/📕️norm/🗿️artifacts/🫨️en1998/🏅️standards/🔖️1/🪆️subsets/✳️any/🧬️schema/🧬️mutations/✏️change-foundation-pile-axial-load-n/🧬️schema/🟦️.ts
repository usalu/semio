/** ✏️ `change-foundation-pile-axial-load-n` wire twin: the leaf payload `ChangeFoundationPileAxialLoadN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPileAxialLoadN {
  mutation: "changeFoundationPileAxialLoadN";
  id: string;
  newPileAxialLoadN: number;
}

export const parseChangeFoundationPileAxialLoadN: NormWireReader<ChangeFoundationPileAxialLoadN> = normWireObject<ChangeFoundationPileAxialLoadN>({ mutation: normWireRequired(normWireLiteral("changeFoundationPileAxialLoadN")), id: normWireRequired(normWireString), newPileAxialLoadN: normWireRequired(normWireNumber) });
