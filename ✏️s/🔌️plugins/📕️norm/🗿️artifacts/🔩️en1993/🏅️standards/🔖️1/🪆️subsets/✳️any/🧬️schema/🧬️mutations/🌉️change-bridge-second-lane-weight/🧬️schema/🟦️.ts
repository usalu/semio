/** 🌉️ `change-bridge-second-lane-weight` wire twin: the leaf payload `ChangeBridgeSecondLaneWeight`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeSecondLaneWeight {
  id: string;
  newSecondLaneWeight: number;
}

export const parseChangeBridgeSecondLaneWeight: NormWireReader<ChangeBridgeSecondLaneWeight> = normWireObject<ChangeBridgeSecondLaneWeight>({ id: normWireRequired(normWireString), newSecondLaneWeight: normWireRequired(normWireNumber) });
