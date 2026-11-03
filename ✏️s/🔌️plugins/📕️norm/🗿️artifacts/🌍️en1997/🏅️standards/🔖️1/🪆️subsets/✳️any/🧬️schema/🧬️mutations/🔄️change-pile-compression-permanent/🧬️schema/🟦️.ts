/** 🔄️ `change-pile-compression-permanent` wire twin: the leaf payload `ChangePileCompressionPermanent`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileCompressionPermanent {
  mutation: "changePileCompressionPermanent";
  id: string;
  newCompressionPermanent: number;
}

export const parseChangePileCompressionPermanent: NormWireReader<ChangePileCompressionPermanent> = normWireObject<ChangePileCompressionPermanent>({ mutation: normWireRequired(normWireLiteral("changePileCompressionPermanent")), id: normWireRequired(normWireString), newCompressionPermanent: normWireRequired(normWireNumber) });
