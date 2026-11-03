/** 🧬 `change-tendon-point-x` wire twin: the leaf payload `ChangeTendonPointX`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTendonPointX {
  memberId: string;
  tendonPointId: string;
  newX: number;
}

export const parseChangeTendonPointX: NormWireReader<ChangeTendonPointX> = normWireObject<ChangeTendonPointX>({ memberId: normWireRequired(normWireString), tendonPointId: normWireRequired(normWireString), newX: normWireRequired(normWireNumber) });
