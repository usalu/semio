/** 💥 `change-accidental-ad` wire twin: the leaf payload `ChangeAccidentalAd`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeAccidentalAd {
  mutation: "changeAccidentalAd";
  id: string;
  newAd: number;
}

export const parseChangeAccidentalAd: NormWireReader<ChangeAccidentalAd> = normWireObject<ChangeAccidentalAd>({ mutation: normWireRequired(normWireLiteral("changeAccidentalAd")), id: normWireRequired(normWireString), newAd: normWireRequired(normWireNumber) });
