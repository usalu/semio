/** 🏗️ `change-stage-duration-days` wire twin: the leaf payload `ChangeStageDurationDays`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeStageDurationDays {
  id: string;
  newDurationDays: number;
}

export const parseChangeStageDurationDays: NormWireReader<ChangeStageDurationDays> = normWireObject<ChangeStageDurationDays>({ id: normWireRequired(normWireString), newDurationDays: normWireRequired(normWireNumber) });
