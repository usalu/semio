/** 🗼️ `change-tower-perm-amplitude-ratio` wire twin: the leaf payload `ChangeTowerPermAmplitudeRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerPermAmplitudeRatio {
  id: string;
  newPermittedAmplitudeRatio: number;
}

export const parseChangeTowerPermAmplitudeRatio: NormWireReader<ChangeTowerPermAmplitudeRatio> = normWireObject<ChangeTowerPermAmplitudeRatio>({ id: normWireRequired(normWireString), newPermittedAmplitudeRatio: normWireRequired(normWireNumber) });
