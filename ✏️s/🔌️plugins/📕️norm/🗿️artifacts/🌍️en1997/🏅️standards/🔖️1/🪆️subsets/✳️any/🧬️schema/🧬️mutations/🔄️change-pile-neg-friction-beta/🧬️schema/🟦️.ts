/** 🔄️ `change-pile-neg-friction-beta` wire twin: the leaf payload `ChangePileNegFrictionBeta`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileNegFrictionBeta {
  mutation: "changePileNegFrictionBeta";
  id: string;
  newNegativeFrictionBeta: number;
}

export const parseChangePileNegFrictionBeta: NormWireReader<ChangePileNegFrictionBeta> = normWireObject<ChangePileNegFrictionBeta>({ mutation: normWireRequired(normWireLiteral("changePileNegFrictionBeta")), id: normWireRequired(normWireString), newNegativeFrictionBeta: normWireRequired(normWireNumber) });
