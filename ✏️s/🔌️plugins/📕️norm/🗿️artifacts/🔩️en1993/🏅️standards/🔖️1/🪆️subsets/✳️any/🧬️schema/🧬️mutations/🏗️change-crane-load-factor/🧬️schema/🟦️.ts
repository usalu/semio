/** 🏗️ `change-crane-load-factor` wire twin: the leaf payload `ChangeCraneLoadFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCraneLoadFactor {
  id: string;
  newLoadFactor: number;
}

export const parseChangeCraneLoadFactor: NormWireReader<ChangeCraneLoadFactor> = normWireObject<ChangeCraneLoadFactor>({ id: normWireRequired(normWireString), newLoadFactor: normWireRequired(normWireNumber) });
