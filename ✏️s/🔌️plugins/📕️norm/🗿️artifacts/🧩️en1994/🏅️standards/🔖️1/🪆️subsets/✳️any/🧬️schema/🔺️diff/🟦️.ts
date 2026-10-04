/** 🔺️ `En1994Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireNullable, normWireNumber, normWireObject, type NormWireReader, normWireRequired, normWireString } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type AnnexChoice, type CompositeBeam, type CompositeColumn, type CompositeSlab, parseAnnexChoice, parseCompositeBeam, parseCompositeColumn, parseCompositeSlab } from "../📸️snapshot/🟦️.ts";
import { type En1994Artifact, parseEn1994Artifact } from "../🟦️.ts";

export interface En1994Diff {
  /** @state artifact */
  artifact: En1994Artifact | null;
  /** @state artifact */
  annex: AnnexChoice | null;
  /** @state artifact */
  structureKind: string | null;
  /** @state artifact */
  steelFYPa: number | null;
  /** @state artifact */
  beams: { values: CompositeBeam[]; } | null;
  /** @state artifact */
  columns: { values: CompositeColumn[]; } | null;
  /** @state artifact */
  slabs: { values: CompositeSlab[]; } | null;
  /** @state artifact */
  fireRating: string | null;
  /** @state artifact */
  insulationThicknessM: number | null;
  /** @state artifact */
  fatigueDetail: string | null;
}

export const parseEn1994Diff: NormWireReader<En1994Diff> = normWireObject<En1994Diff>({ artifact: normWireDefault(normWireNullable(parseEn1994Artifact), () => null), annex: normWireDefault(normWireNullable(parseAnnexChoice), () => null), structureKind: normWireDefault(normWireNullable(normWireString), () => null), steelFYPa: normWireDefault(normWireNullable(normWireNumber), () => null), beams: normWireDefault(normWireNullable(normWireObject<{ values: CompositeBeam[]; }>({ values: normWireRequired(normWireArray(parseCompositeBeam)) })), () => null), columns: normWireDefault(normWireNullable(normWireObject<{ values: CompositeColumn[]; }>({ values: normWireRequired(normWireArray(parseCompositeColumn)) })), () => null), slabs: normWireDefault(normWireNullable(normWireObject<{ values: CompositeSlab[]; }>({ values: normWireRequired(normWireArray(parseCompositeSlab)) })), () => null), fireRating: normWireDefault(normWireNullable(normWireString), () => null), insulationThicknessM: normWireDefault(normWireNullable(normWireNumber), () => null), fatigueDetail: normWireDefault(normWireNullable(normWireString), () => null) });
