/** 🗼️ `change-tower-permitted-amplitude-ratio` wire twin: the leaf payload `ChangeTowerPermittedAmplitudeRatio`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTowerPermittedAmplitudeRatio {
  id: string;
  newPermittedAmplitudeRatio: number;
}

export const parseChangeTowerPermittedAmplitudeRatio: NormWireReader<ChangeTowerPermittedAmplitudeRatio> = normWireObject<ChangeTowerPermittedAmplitudeRatio>({ id: normWireRequired(normWireString), newPermittedAmplitudeRatio: normWireRequired(normWireNumber) });
