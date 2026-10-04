/** 🔺️ `En1993Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireDefault, normWireLiteral, normWireNullable, normWireObject, type NormWireReader, normWireRequired } from "../../../../../../../../📇️registry/🧬️contract/🟦️.ts";
import { type BridgeFatigue, type ColdFormedMember, type CraneRunway, type FatigueDetail, type FireExposure, type LoadCase, type MemberAction, parseBridgeFatigue, parseColdFormedMember, parseCraneRunway, parseFatigueDetail, parseFireExposure, parseLoadCase, parseMemberAction, parsePlatedPanel, parseSiloShell, parseSteelJoint, parseSteelMaterial, parseSteelMember, parseSteelPile, parseSteelSection, parseTensionComponent, parseTowerLeg, type PlatedPanel, type SiloShell, type SteelJoint, type SteelMaterial, type SteelMember, type SteelPile, type SteelSection, type TensionComponent, type TowerLeg } from "../📸️snapshot/🟦️.ts";
import { type En1993Artifact, parseEn1993Artifact } from "../🟦️.ts";

export interface En1993Diff {
  /** @state artifact */
  artifact: En1993Artifact | null;
  /** @state artifact */
  annex: ("En" | "De") | null;
  /** @state artifact */
  materials: { values: SteelMaterial[]; } | null;
  /** @state artifact */
  sections: { values: SteelSection[]; } | null;
  /** @state artifact */
  members: { values: SteelMember[]; } | null;
  /** @state artifact */
  loadCases: { values: LoadCase[]; } | null;
  /** @state artifact */
  memberActions: { values: MemberAction[]; } | null;
  /** @state artifact */
  joints: { values: SteelJoint[]; } | null;
  /** @state artifact */
  fatigueDetails: { values: FatigueDetail[]; } | null;
  /** @state artifact */
  fireExposures: { values: FireExposure[]; } | null;
  /** @state artifact */
  coldFormedMembers: { values: ColdFormedMember[]; } | null;
  /** @state artifact */
  platedPanels: { values: PlatedPanel[]; } | null;
  /** @state artifact */
  siloShells: { values: SiloShell[]; } | null;
  /** @state artifact */
  tensionComponents: { values: TensionComponent[]; } | null;
  /** @state artifact */
  bridgeFatigue: { values: BridgeFatigue[]; } | null;
  /** @state artifact */
  towerLegs: { values: TowerLeg[]; } | null;
  /** @state artifact */
  piles: { values: SteelPile[]; } | null;
  /** @state artifact */
  craneRunways: { values: CraneRunway[]; } | null;
}

export const parseEn1993Diff: NormWireReader<En1993Diff> = normWireObject<En1993Diff>({ artifact: normWireDefault(normWireNullable(parseEn1993Artifact), () => null), annex: normWireDefault(normWireNullable(normWireLiteral("En", "De")), () => null), materials: normWireDefault(normWireNullable(normWireObject<{ values: SteelMaterial[]; }>({ values: normWireRequired(normWireArray(parseSteelMaterial)) })), () => null), sections: normWireDefault(normWireNullable(normWireObject<{ values: SteelSection[]; }>({ values: normWireRequired(normWireArray(parseSteelSection)) })), () => null), members: normWireDefault(normWireNullable(normWireObject<{ values: SteelMember[]; }>({ values: normWireRequired(normWireArray(parseSteelMember)) })), () => null), loadCases: normWireDefault(normWireNullable(normWireObject<{ values: LoadCase[]; }>({ values: normWireRequired(normWireArray(parseLoadCase)) })), () => null), memberActions: normWireDefault(normWireNullable(normWireObject<{ values: MemberAction[]; }>({ values: normWireRequired(normWireArray(parseMemberAction)) })), () => null), joints: normWireDefault(normWireNullable(normWireObject<{ values: SteelJoint[]; }>({ values: normWireRequired(normWireArray(parseSteelJoint)) })), () => null), fatigueDetails: normWireDefault(normWireNullable(normWireObject<{ values: FatigueDetail[]; }>({ values: normWireRequired(normWireArray(parseFatigueDetail)) })), () => null), fireExposures: normWireDefault(normWireNullable(normWireObject<{ values: FireExposure[]; }>({ values: normWireRequired(normWireArray(parseFireExposure)) })), () => null), coldFormedMembers: normWireDefault(normWireNullable(normWireObject<{ values: ColdFormedMember[]; }>({ values: normWireRequired(normWireArray(parseColdFormedMember)) })), () => null), platedPanels: normWireDefault(normWireNullable(normWireObject<{ values: PlatedPanel[]; }>({ values: normWireRequired(normWireArray(parsePlatedPanel)) })), () => null), siloShells: normWireDefault(normWireNullable(normWireObject<{ values: SiloShell[]; }>({ values: normWireRequired(normWireArray(parseSiloShell)) })), () => null), tensionComponents: normWireDefault(normWireNullable(normWireObject<{ values: TensionComponent[]; }>({ values: normWireRequired(normWireArray(parseTensionComponent)) })), () => null), bridgeFatigue: normWireDefault(normWireNullable(normWireObject<{ values: BridgeFatigue[]; }>({ values: normWireRequired(normWireArray(parseBridgeFatigue)) })), () => null), towerLegs: normWireDefault(normWireNullable(normWireObject<{ values: TowerLeg[]; }>({ values: normWireRequired(normWireArray(parseTowerLeg)) })), () => null), piles: normWireDefault(normWireNullable(normWireObject<{ values: SteelPile[]; }>({ values: normWireRequired(normWireArray(parseSteelPile)) })), () => null), craneRunways: normWireDefault(normWireNullable(normWireObject<{ values: CraneRunway[]; }>({ values: normWireRequired(normWireArray(parseCraneRunway)) })), () => null) });
