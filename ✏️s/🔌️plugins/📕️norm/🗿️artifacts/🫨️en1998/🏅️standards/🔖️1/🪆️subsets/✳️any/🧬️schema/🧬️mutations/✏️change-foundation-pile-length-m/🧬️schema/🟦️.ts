/** ✏️ `change-foundation-pile-length-m` wire twin: the leaf payload `ChangeFoundationPileLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationPileLengthM {
  mutation: "changeFoundationPileLengthM";
  id: string;
  newPileLengthM: number;
}

export const parseChangeFoundationPileLengthM: NormWireReader<ChangeFoundationPileLengthM> = normWireObject<ChangeFoundationPileLengthM>({ mutation: normWireRequired(normWireLiteral("changeFoundationPileLengthM")), id: normWireRequired(normWireString), newPileLengthM: normWireRequired(normWireNumber) });
