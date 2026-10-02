/** ⚖️ `change-geg-qp-factor` wire twin: the leaf payload `ChangeGegQpFactor`, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRequired } from "../../../../../../../../../../📇️registry/🧬️contract/🟦️.ts";

export interface ChangeGegQpFactor {
  mutation: "changeGegQpFactor";
  newGegQpFactor: number;
}

export const parseChangeGegQpFactor: NormWireReader<ChangeGegQpFactor> = normWireObject<ChangeGegQpFactor>({ mutation: normWireRequired(normWireLiteral("changeGegQpFactor")), newGegQpFactor: normWireRequired(normWireRange(normWireNumber, {"exclusiveMinimum":0})) });
