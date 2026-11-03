/** 👁️ `change-supervision-level` wire twin: the leaf payload `ChangeSupervisionLevel`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type En1990SupervisionLevel, parseEn1990SupervisionLevel } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeSupervisionLevel {
  mutation: "changeSupervisionLevel";
  newSupervisionLevel: En1990SupervisionLevel;
}

export const parseChangeSupervisionLevel: NormWireReader<ChangeSupervisionLevel> = normWireObject<ChangeSupervisionLevel>({ mutation: normWireRequired(normWireLiteral("changeSupervisionLevel")), newSupervisionLevel: normWireRequired(parseEn1990SupervisionLevel) });
