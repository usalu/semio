/** ⚗️ `change-material-ramberg-osgood-n` wire twin: the leaf payload `ChangeMaterialRambergOsgoodN`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeMaterialRambergOsgoodN {
  id: string;
  newRambergOsgoodN: number;
}

export const parseChangeMaterialRambergOsgoodN: NormWireReader<ChangeMaterialRambergOsgoodN> = normWireObject<ChangeMaterialRambergOsgoodN>({ id: normWireRequired(normWireString), newRambergOsgoodN: normWireRequired(normWireNumber) });
