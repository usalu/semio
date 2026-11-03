/** 🔄️ `change-pile-elastic-modulus` wire twin: the leaf payload `ChangePileElasticModulus`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileElasticModulus {
  mutation: "changePileElasticModulus";
  id: string;
  newElasticModulus: number;
}

export const parseChangePileElasticModulus: NormWireReader<ChangePileElasticModulus> = normWireObject<ChangePileElasticModulus>({ mutation: normWireRequired(normWireLiteral("changePileElasticModulus")), id: normWireRequired(normWireString), newElasticModulus: normWireRequired(normWireNumber) });
