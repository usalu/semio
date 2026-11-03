/** 🌉️ `change-bridge-lambda` wire twin: the leaf payload `ChangeBridgeLambda`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeLambda {
  id: string;
  newLambda: number;
}

export const parseChangeBridgeLambda: NormWireReader<ChangeBridgeLambda> = normWireObject<ChangeBridgeLambda>({ id: normWireRequired(normWireString), newLambda: normWireRequired(normWireNumber) });
