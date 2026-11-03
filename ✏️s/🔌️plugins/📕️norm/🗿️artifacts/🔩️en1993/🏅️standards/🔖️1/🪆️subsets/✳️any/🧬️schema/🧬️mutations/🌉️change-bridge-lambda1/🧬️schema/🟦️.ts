/** 🌉️ `change-bridge-lambda1` wire twin: the leaf payload `ChangeBridgeLambda1`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeLambda1 {
  id: string;
  newLambda1: number;
}

export const parseChangeBridgeLambda1: NormWireReader<ChangeBridgeLambda1> = normWireObject<ChangeBridgeLambda1>({ id: normWireRequired(normWireString), newLambda1: normWireRequired(normWireNumber) });
