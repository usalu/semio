/** 🏢 `change-floor-assumed-partitions` wire twin: the leaf payload `ChangeFloorAssumedPartitions`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeFloorAssumedPartitions {
  id: string;
  newAssumedPartitions: number;
}

export const parseChangeFloorAssumedPartitions: NormWireReader<ChangeFloorAssumedPartitions> = normWireObject<ChangeFloorAssumedPartitions>({ id: normWireRequired(normWireString), newAssumedPartitions: normWireRequired(normWireNumber) });
