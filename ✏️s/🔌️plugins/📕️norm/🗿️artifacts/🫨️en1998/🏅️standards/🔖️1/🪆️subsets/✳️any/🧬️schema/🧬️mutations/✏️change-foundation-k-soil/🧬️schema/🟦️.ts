/** ✏️ `change-foundation-k-soil` wire twin: the leaf payload `ChangeFoundationKSoil`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFoundationKSoil {
  mutation: "changeFoundationKSoil";
  id: string;
  newKSoil: number;
}

export const parseChangeFoundationKSoil: NormWireReader<ChangeFoundationKSoil> = normWireObject<ChangeFoundationKSoil>({ mutation: normWireRequired(normWireLiteral("changeFoundationKSoil")), id: normWireRequired(normWireString), newKSoil: normWireRequired(normWireNumber) });
