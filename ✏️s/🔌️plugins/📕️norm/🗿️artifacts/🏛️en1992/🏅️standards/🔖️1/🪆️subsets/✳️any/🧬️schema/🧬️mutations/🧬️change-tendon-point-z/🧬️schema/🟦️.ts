/** 🧬 `change-tendon-point-z` wire twin: the leaf payload `ChangeTendonPointZ`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTendonPointZ {
  memberId: string;
  tendonPointId: string;
  newZ: number;
}

export const parseChangeTendonPointZ: NormWireReader<ChangeTendonPointZ> = normWireObject<ChangeTendonPointZ>({ memberId: normWireRequired(normWireString), tendonPointId: normWireRequired(normWireString), newZ: normWireRequired(normWireNumber) });
