/** 💧 `change-water-flow-length` wire twin: the leaf payload `ChangeWaterFlowLength`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWaterFlowLength {
  mutation: "changeWaterFlowLength";
  id: string;
  newFlowLength: number;
}

export const parseChangeWaterFlowLength: NormWireReader<ChangeWaterFlowLength> = normWireObject<ChangeWaterFlowLength>({ mutation: normWireRequired(normWireLiteral("changeWaterFlowLength")), id: normWireRequired(normWireString), newFlowLength: normWireRequired(normWireNumber) });
