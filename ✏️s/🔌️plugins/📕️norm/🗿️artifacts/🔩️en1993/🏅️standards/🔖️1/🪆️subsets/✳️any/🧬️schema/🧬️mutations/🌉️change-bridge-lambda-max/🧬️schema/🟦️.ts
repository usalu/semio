/** 🌉️ `change-bridge-lambda-max` wire twin: the leaf payload `ChangeBridgeLambdaMax`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBridgeLambdaMax {
  id: string;
  newLambdaMax: number;
}

export const parseChangeBridgeLambdaMax: NormWireReader<ChangeBridgeLambdaMax> = normWireObject<ChangeBridgeLambdaMax>({ id: normWireRequired(normWireString), newLambdaMax: normWireRequired(normWireNumber) });
