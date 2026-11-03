/** 🥶️ `change-cf-spring-factor` wire twin: the leaf payload `ChangeCfSpringFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCfSpringFactor {
  id: string;
  newSpringFactor: number;
}

export const parseChangeCfSpringFactor: NormWireReader<ChangeCfSpringFactor> = normWireObject<ChangeCfSpringFactor>({ id: normWireRequired(normWireString), newSpringFactor: normWireRequired(normWireNumber) });
