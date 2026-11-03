/** 🧪 `change-test-required` wire twin: the leaf payload `ChangeTestRequired`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeTestRequired {
  mutation: "changeTestRequired";
  id: string;
  newRequired: number;
}

export const parseChangeTestRequired: NormWireReader<ChangeTestRequired> = normWireObject<ChangeTestRequired>({ mutation: normWireRequired(normWireLiteral("changeTestRequired")), id: normWireRequired(normWireString), newRequired: normWireRequired(normWireNumber) });
