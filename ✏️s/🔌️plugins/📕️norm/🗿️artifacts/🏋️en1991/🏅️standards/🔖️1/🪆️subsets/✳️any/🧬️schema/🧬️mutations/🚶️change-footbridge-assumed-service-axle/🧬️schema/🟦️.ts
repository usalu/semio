/** 🚶 `change-footbridge-assumed-service-axle` wire twin: the leaf payload `ChangeFootbridgeAssumedServiceAxle`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFootbridgeAssumedServiceAxle {
  id: string;
  newAssumedServiceAxle: number;
}

export const parseChangeFootbridgeAssumedServiceAxle: NormWireReader<ChangeFootbridgeAssumedServiceAxle> = normWireObject<ChangeFootbridgeAssumedServiceAxle>({ id: normWireRequired(normWireString), newAssumedServiceAxle: normWireRequired(normWireNumber) });
