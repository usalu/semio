/** 📐 `change-beta-computed` wire twin: the leaf payload `ChangeBetaComputed`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeBetaComputed {
  mutation: "changeBetaComputed";
  newBetaComputed: number;
}

export const parseChangeBetaComputed: NormWireReader<ChangeBetaComputed> = normWireObject<ChangeBetaComputed>({ mutation: normWireRequired(normWireLiteral("changeBetaComputed")), newBetaComputed: normWireRequired(normWireNumber) });
