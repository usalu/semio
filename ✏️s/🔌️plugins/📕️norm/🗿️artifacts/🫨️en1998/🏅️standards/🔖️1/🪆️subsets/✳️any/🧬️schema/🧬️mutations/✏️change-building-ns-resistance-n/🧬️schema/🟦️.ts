/** ✏️ `change-building-ns-resistance-n` wire twin: the leaf payload `ChangeBuildingNsResistanceN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBuildingNsResistanceN {
  mutation: "changeBuildingNsResistanceN";
  id: string;
  newNonstructuralResistanceN: number;
}

export const parseChangeBuildingNsResistanceN: NormWireReader<ChangeBuildingNsResistanceN> = normWireObject<ChangeBuildingNsResistanceN>({ mutation: normWireRequired(normWireLiteral("changeBuildingNsResistanceN")), id: normWireRequired(normWireString), newNonstructuralResistanceN: normWireRequired(normWireNumber) });
