/** 🔺️ `En1995Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireLiteral, normWireNullable, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { parseTimberConnection, parseTimberMember, type TimberConnection, type TimberMember } from "../📸️snapshot/🟦️.ts";
import { type En1995Artifact, parseEn1995Artifact } from "../🟦️.ts";

export interface En1995Diff {
  /** @state artifact */
  artifact: En1995Artifact | null;
  /** @state artifact */
  annex: ("En" | "De") | null;
  /** @state artifact */
  members: { values: TimberMember[]; } | null;
  /** @state artifact */
  connections: { values: TimberConnection[]; } | null;
}

export const parseEn1995Diff: NormWireReader<En1995Diff> = normWireObject<En1995Diff>({ artifact: normWireDefault(normWireNullable(parseEn1995Artifact), () => null), annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), members: normWireDefault(normWireNullable(normWireObject<{ values: TimberMember[]; }>({ values: normWireRequired(normWireArray(parseTimberMember)) })), () => null), connections: normWireDefault(normWireNullable(normWireObject<{ values: TimberConnection[]; }>({ values: normWireRequired(normWireArray(parseTimberConnection)) })), () => null) });
