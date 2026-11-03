/** 🧬 `change-tendon-point-y` wire twin: the leaf payload `ChangeTendonPointY`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTendonPointY {
  memberId: string;
  tendonPointId: string;
  newY: number;
}

export const parseChangeTendonPointY: NormWireReader<ChangeTendonPointY> = normWireObject<ChangeTendonPointY>({ memberId: normWireRequired(normWireString), tendonPointId: normWireRequired(normWireString), newY: normWireRequired(normWireNumber) });
