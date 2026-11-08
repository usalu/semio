/** 📍️ `change-pier-base-soil-vs-m-s` wire twin: the leaf payload `ChangePierBaseSoilVsMS`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangePierBaseSoilVsMS {
  mutation: "changePierBaseSoilVsMS";
  id: string;
  newSoilVsMS: number;
}

export const parseChangePierBaseSoilVsMS: NormWireReader<ChangePierBaseSoilVsMS> = normWireObject<ChangePierBaseSoilVsMS>({ mutation: normWireRequired(normWireLiteral("changePierBaseSoilVsMS")), id: normWireRequired(normWireString), newSoilVsMS: normWireRequired(normWireNumber) });
