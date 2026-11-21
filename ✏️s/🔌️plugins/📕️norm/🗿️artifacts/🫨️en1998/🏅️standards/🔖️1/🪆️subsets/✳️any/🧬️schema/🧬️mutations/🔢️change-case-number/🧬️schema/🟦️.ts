/** 🔢️ `change-case-number` wire twin: the leaf payload `ChangeCaseNumber`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";

export interface ChangeCaseNumber {
  mutation: "changeCaseNumber";
  caseId: string;
  key: string;
  value?: number | null;
}

export const parseChangeCaseNumber: NormWireReader<ChangeCaseNumber> = normWireObject<ChangeCaseNumber>({ mutation: normWireRequired(normWireLiteral("changeCaseNumber")), caseId: normWireRequired(normWireString), key: normWireRequired(normWireString), value: normWireOptional(normWireNullable(normWireNumber)) });
