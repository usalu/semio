/** ⏱️ `change-reference-period-years` wire twin: the leaf payload `ChangeReferencePeriodYears`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, type NormWireReader, normWireRequired } from "@semio-tech/norm-js/registry/contract";

export interface ChangeReferencePeriodYears {
  mutation: "changeReferencePeriodYears";
  newReferencePeriodYears: number;
}

export const parseChangeReferencePeriodYears: NormWireReader<ChangeReferencePeriodYears> = normWireObject<ChangeReferencePeriodYears>({ mutation: normWireRequired(normWireLiteral("changeReferencePeriodYears")), newReferencePeriodYears: normWireRequired(normWireNumber) });
