/** ✏️ `change-foundation-pile-diameter-m` wire twin: the leaf payload `ChangeFoundationPileDiameterM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPileDiameterM {
  mutation: "changeFoundationPileDiameterM";
  id: string;
  newPileDiameterM: number;
}

export const parseChangeFoundationPileDiameterM: NormWireReader<ChangeFoundationPileDiameterM> = normWireObject<ChangeFoundationPileDiameterM>({ mutation: normWireRequired(normWireLiteral("changeFoundationPileDiameterM")), id: normWireRequired(normWireString), newPileDiameterM: normWireRequired(normWireNumber) });
