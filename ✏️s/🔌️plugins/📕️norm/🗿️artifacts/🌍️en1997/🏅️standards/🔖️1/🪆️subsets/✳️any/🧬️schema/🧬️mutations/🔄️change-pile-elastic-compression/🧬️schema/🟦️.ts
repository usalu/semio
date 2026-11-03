/** 🔄️ `change-pile-elastic-compression` wire twin: the leaf payload `ChangePileElasticCompression`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileElasticCompression {
  mutation: "changePileElasticCompression";
  id: string;
  newElasticCompression: number;
}

export const parseChangePileElasticCompression: NormWireReader<ChangePileElasticCompression> = normWireObject<ChangePileElasticCompression>({ mutation: normWireRequired(normWireLiteral("changePileElasticCompression")), id: normWireRequired(normWireString), newElasticCompression: normWireRequired(normWireNumber) });
