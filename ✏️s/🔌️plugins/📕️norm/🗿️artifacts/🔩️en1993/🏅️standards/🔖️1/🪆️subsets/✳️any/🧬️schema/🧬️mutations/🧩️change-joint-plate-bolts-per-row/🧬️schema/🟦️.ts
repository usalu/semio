/** 🧩️ `change-joint-plate-bolts-per-row` wire twin: the leaf payload `ChangeJointPlateBoltsPerRow`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireInteger, normWireObject, normWireRange, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeJointPlateBoltsPerRow {
  id: string;
  newBoltsPerRow: number;
}

export const parseChangeJointPlateBoltsPerRow: NormWireReader<ChangeJointPlateBoltsPerRow> = normWireObject<ChangeJointPlateBoltsPerRow>({ id: normWireRequired(normWireString), newBoltsPerRow: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
