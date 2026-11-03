/** 🪵️ `change-pile-driving-stress` wire twin: the leaf payload `ChangePileDrivingStress`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileDrivingStress {
  id: string;
  newDrivingStress: number;
}

export const parseChangePileDrivingStress: NormWireReader<ChangePileDrivingStress> = normWireObject<ChangePileDrivingStress>({ id: normWireRequired(normWireString), newDrivingStress: normWireRequired(normWireNumber) });
