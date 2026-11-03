/** 💧 `change-water-layer-id` wire twin: the leaf payload `ChangeWaterLayerId`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeWaterLayerId {
  mutation: "changeWaterLayerId";
  id: string;
  newLayerId: string;
}

export const parseChangeWaterLayerId: NormWireReader<ChangeWaterLayerId> = normWireObject<ChangeWaterLayerId>({ mutation: normWireRequired(normWireLiteral("changeWaterLayerId")), id: normWireRequired(normWireString), newLayerId: normWireRequired(normWireString) });
