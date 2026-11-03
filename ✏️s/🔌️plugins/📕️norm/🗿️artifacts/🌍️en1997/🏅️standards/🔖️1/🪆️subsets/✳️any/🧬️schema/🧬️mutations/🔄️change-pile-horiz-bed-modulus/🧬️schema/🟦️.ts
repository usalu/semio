/** 🔄️ `change-pile-horiz-bed-modulus` wire twin: the leaf payload `ChangePileHorizBedModulus`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileHorizBedModulus {
  mutation: "changePileHorizBedModulus";
  id: string;
  newHorizontalSubgradeModulus: number;
}

export const parseChangePileHorizBedModulus: NormWireReader<ChangePileHorizBedModulus> = normWireObject<ChangePileHorizBedModulus>({ mutation: normWireRequired(normWireLiteral("changePileHorizBedModulus")), id: normWireRequired(normWireString), newHorizontalSubgradeModulus: normWireRequired(normWireNumber) });
