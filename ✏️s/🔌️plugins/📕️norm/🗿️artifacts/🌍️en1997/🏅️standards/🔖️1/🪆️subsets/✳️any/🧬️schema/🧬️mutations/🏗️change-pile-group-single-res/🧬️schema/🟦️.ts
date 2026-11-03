/** 🏗️ `change-pile-group-single-res` wire twin: the leaf payload `ChangePileGroupSingleRes`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePileGroupSingleRes {
  mutation: "changePileGroupSingleRes";
  id: string;
  newSingleResistance: number;
}

export const parseChangePileGroupSingleRes: NormWireReader<ChangePileGroupSingleRes> = normWireObject<ChangePileGroupSingleRes>({ mutation: normWireRequired(normWireLiteral("changePileGroupSingleRes")), id: normWireRequired(normWireString), newSingleResistance: normWireRequired(normWireNumber) });
