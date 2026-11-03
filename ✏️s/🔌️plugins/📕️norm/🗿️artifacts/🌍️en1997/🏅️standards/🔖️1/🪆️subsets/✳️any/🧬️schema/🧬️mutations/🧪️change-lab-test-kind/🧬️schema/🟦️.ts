/** 🧪️ `change-lab-test-kind` wire twin: the leaf payload `ChangeLabTestKind`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type LabTestKind, parseLabTestKind } from "../../../📸️snapshot/🟦️.ts";

export interface ChangeLabTestKind {
  mutation: "changeLabTestKind";
  id: string;
  newKind: LabTestKind;
}

export const parseChangeLabTestKind: NormWireReader<ChangeLabTestKind> = normWireObject<ChangeLabTestKind>({ mutation: normWireRequired(normWireLiteral("changeLabTestKind")), id: normWireRequired(normWireString), newKind: normWireRequired(parseLabTestKind) });
