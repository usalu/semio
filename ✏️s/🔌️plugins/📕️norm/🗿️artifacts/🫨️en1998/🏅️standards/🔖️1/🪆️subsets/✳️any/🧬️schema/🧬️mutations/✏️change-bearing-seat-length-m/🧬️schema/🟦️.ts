/** ✏️ `change-bearing-seat-length-m` wire twin: the leaf payload `ChangeBearingSeatLengthM`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBearingSeatLengthM {
  mutation: "changeBearingSeatLengthM";
  id: string;
  newSeatLengthM: number;
}

export const parseChangeBearingSeatLengthM: NormWireReader<ChangeBearingSeatLengthM> = normWireObject<ChangeBearingSeatLengthM>({ mutation: normWireRequired(normWireLiteral("changeBearingSeatLengthM")), id: normWireRequired(normWireString), newSeatLengthM: normWireRequired(normWireNumber) });
