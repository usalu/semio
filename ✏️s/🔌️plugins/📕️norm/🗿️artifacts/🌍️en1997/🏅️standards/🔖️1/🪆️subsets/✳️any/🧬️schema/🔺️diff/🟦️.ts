/** 🔺️ `En1997Diff` wire twin: the sparse field delta a mutation raises, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireInteger, normWireNullable, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "@semio-tech/norm-js/registry/contract";
import { type ApproachChoice, parseApproachChoice, type CharacteristicParameter, parseCharacteristicParameter, type EarthPressureMode, parseEarthPressureMode, type EmbeddedWallKind, parseEmbeddedWallKind, type En1997Anchor, parseEn1997Anchor, type En1997AnchorTest, parseEn1997AnchorTest, type En1997AnnexChoice, parseEn1997AnnexChoice, type En1997CaseNumber, parseEn1997CaseNumber, type En1997CaseRow, parseEn1997CaseRow, type En1997CaseText, parseEn1997CaseText, type En1997CharacteristicBasis, parseEn1997CharacteristicBasis, type En1997EmbeddedWall, parseEn1997EmbeddedWall, type En1997FoundationLoadCase, parseEn1997FoundationLoadCase, type En1997Investigation, parseEn1997Investigation, type En1997InvestigationReading, parseEn1997InvestigationReading, type En1997LabPoint, parseEn1997LabPoint, type En1997LabTest, parseEn1997LabTest, type En1997LayerWater, parseEn1997LayerWater, type En1997Pile, parseEn1997Pile, type En1997PileGroup, parseEn1997PileGroup, type En1997PileTestProfile, parseEn1997PileTestProfile, type En1997Placement, parseEn1997Placement, type En1997RetainingWall, parseEn1997RetainingWall, type En1997Slope, parseEn1997Slope, type En1997SoilLayer, parseEn1997SoilLayer, type En1997SpreadFoundation, parseEn1997SpreadFoundation, type En1997UpliftCase, parseEn1997UpliftCase, type InvestigationKind, parseInvestigationKind, type LabTestKind, parseLabTestKind, type ParameterBasis, parseParameterBasis, type PileConcreteClass, parsePileConcreteClass, type PileType, parsePileType, type SituationChoice, parseSituationChoice, type SoilType, parseSoilType, type TendonType, parseTendonType, type WallMovement, parseWallMovement } from "../📸️snapshot/🟦️.ts";

export interface En1997Diff {
  /** @state artifact */
  structureId?: string | null;
  /** @state artifact */
  geotechnicalCategory?: number | null;
  /** @state artifact */
  designSituation?: SituationChoice | null;
  /** @state artifact */
  designApproach?: ApproachChoice | null;
  /** @state artifact */
  annex?: En1997AnnexChoice | null;
  /** @state artifact */
  groundwaterLevel?: number | null;
  /** @state artifact */
  investigationDepth?: number | null;
  /** @state artifact */
  layers?: En1997LayersRows | null;
  /** @state artifact */
  footings?: En1997FootingsRows | null;
  /** @state artifact */
  piles?: En1997PilesRows | null;
  /** @state artifact */
  retainingWalls?: En1997RetainingWallsRows | null;
  /** @state artifact */
  slopes?: En1997SlopesRows | null;
  /** @state artifact */
  upliftCases?: En1997UpliftCasesRows | null;
  /** @state artifact */
  anchors?: En1997AnchorsRows | null;
  /** @state artifact */
  embeddedWalls?: En1997EmbeddedWallsRows | null;
  /** @state artifact */
  pileGroups?: En1997PileGroupsRows | null;
  /** @state artifact */
  labTests?: En1997LabTestsRows | null;
  /** @state artifact */
  layerWaters?: En1997LayerWatersRows | null;
  /** @state artifact */
  placements?: En1997PlacementsRows | null;
  /** @state artifact */
  cases: CaseRowDelta;
  /** @state artifact */
  investigations?: En1997InvestigationsRows | null;
  /** @state artifact */
  characteristicBases?: En1997CharacteristicBasesRows | null;
}

export interface CaseNumberDelta {
  removed: CaseNumberRemoval[];
  inserted: CaseNumberInsertion[];
  moved: CaseNumberRelocation[];
  modified: CaseNumberModification[];
}

export interface CaseNumberInsertion {
  index: number;
  row: En1997CaseNumber;
}

export interface CaseNumberModification {
  id: string;
  patch: CaseNumberPatch;
}

export interface CaseNumberPatch {
  value?: number | null;
}

export interface CaseNumberRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseNumberRemoval {
  id: string;
  index: number;
}

export interface CaseRowDelta {
  removed: CaseRowRemoval[];
  inserted: CaseRowInsertion[];
  moved: CaseRowRelocation[];
  modified: CaseRowModification[];
}

export interface CaseRowInsertion {
  index: number;
  row: En1997CaseRow;
}

export interface CaseRowModification {
  id: string;
  patch: CaseRowPatch;
}

export interface CaseRowPatch {
  labelEn?: string | null;
  labelDe?: string | null;
  entryId?: string | null;
  numbers?: CaseNumberDelta;
  texts?: CaseTextDelta;
}

export interface CaseRowRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseRowRemoval {
  id: string;
  index: number;
}

export interface CaseTextDelta {
  removed: CaseTextRemoval[];
  inserted: CaseTextInsertion[];
  moved: CaseTextRelocation[];
  modified: CaseTextModification[];
}

export interface CaseTextInsertion {
  index: number;
  row: En1997CaseText;
}

export interface CaseTextModification {
  id: string;
  patch: CaseTextPatch;
}

export interface CaseTextPatch {
  value?: string | null;
}

export interface CaseTextRelocation {
  id: string;
  from: number;
  to: number;
}

export interface CaseTextRemoval {
  id: string;
  index: number;
}

export interface En1997AnchorsInserted {
  index: number;
  row: En1997Anchor;
}

export interface En1997AnchorsModified {
  id: string;
  patch: En1997AnchorsPatch;
}

export interface En1997AnchorsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997AnchorsPatch {
  wallId?: string | null;
  tendon?: TendonType | null;
  headDepth?: number | null;
  inclinationDeg?: number | null;
  freeLength?: number | null;
  fixedLength?: number | null;
  fixedDiameter?: number | null;
  bondStress?: number | null;
  designForce?: number | null;
  tendonYieldForce?: number | null;
  minFreeLength?: number | null;
  testFactor?: number | null;
  tests?: En1997AnchorsTestsRows;
}

export interface En1997AnchorsRemoved {
  id: string;
  index: number;
}

export interface En1997AnchorsRows {
  removed: En1997AnchorsRemoved[];
  inserted: En1997AnchorsInserted[];
  moved: En1997AnchorsMoved[];
  modified: En1997AnchorsModified[];
}

export interface En1997AnchorsTestsInserted {
  index: number;
  row: En1997AnchorTest;
}

export interface En1997AnchorsTestsModified {
  id: string;
  patch: En1997AnchorsTestsPatch;
}

export interface En1997AnchorsTestsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997AnchorsTestsPatch {
  pullOutResistance?: number | null;
}

export interface En1997AnchorsTestsRemoved {
  id: string;
  index: number;
}

export interface En1997AnchorsTestsRows {
  removed: En1997AnchorsTestsRemoved[];
  inserted: En1997AnchorsTestsInserted[];
  moved: En1997AnchorsTestsMoved[];
  modified: En1997AnchorsTestsModified[];
}

export interface En1997EmbeddedWallsInserted {
  index: number;
  row: En1997EmbeddedWall;
}

export interface En1997EmbeddedWallsModified {
  id: string;
  patch: En1997EmbeddedWallsPatch;
}

export interface En1997EmbeddedWallsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997EmbeddedWallsPatch {
  kind?: EmbeddedWallKind | null;
  retainedHeight?: number | null;
  embedment?: number | null;
  propDepth?: number | null;
  thickness?: number | null;
  backfillGamma?: number | null;
  phiDeg?: number | null;
  deltaActiveDeg?: number | null;
  deltaPassiveDeg?: number | null;
  surcharge?: number | null;
  undrainedStrength?: number | null;
}

export interface En1997EmbeddedWallsRemoved {
  id: string;
  index: number;
}

export interface En1997EmbeddedWallsRows {
  removed: En1997EmbeddedWallsRemoved[];
  inserted: En1997EmbeddedWallsInserted[];
  moved: En1997EmbeddedWallsMoved[];
  modified: En1997EmbeddedWallsModified[];
}

export interface En1997FootingsInserted {
  index: number;
  row: En1997SpreadFoundation;
}

export interface En1997FootingsLoadCasesInserted {
  index: number;
  row: En1997FoundationLoadCase;
}

export interface En1997FootingsLoadCasesModified {
  id: string;
  patch: En1997FootingsLoadCasesPatch;
}

export interface En1997FootingsLoadCasesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997FootingsLoadCasesPatch {
  designSituation?: SituationChoice | null;
  verticalPermanent?: number | null;
  verticalVariable?: number | null;
  horizontalPermanent?: number | null;
  horizontalVariable?: number | null;
  momentPermanent?: number | null;
  momentVariable?: number | null;
}

export interface En1997FootingsLoadCasesRemoved {
  id: string;
  index: number;
}

export interface En1997FootingsLoadCasesRows {
  removed: En1997FootingsLoadCasesRemoved[];
  inserted: En1997FootingsLoadCasesInserted[];
  moved: En1997FootingsLoadCasesMoved[];
  modified: En1997FootingsLoadCasesModified[];
}

export interface En1997FootingsModified {
  id: string;
  patch: En1997FootingsPatch;
}

export interface En1997FootingsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997FootingsPatch {
  width?: number | null;
  length?: number | null;
  embedment?: number | null;
  baseInclinationDeg?: number | null;
  settlementLimit?: number | null;
  loadCases?: En1997FootingsLoadCasesRows;
}

export interface En1997FootingsRemoved {
  id: string;
  index: number;
}

export interface En1997FootingsRows {
  removed: En1997FootingsRemoved[];
  inserted: En1997FootingsInserted[];
  moved: En1997FootingsMoved[];
  modified: En1997FootingsModified[];
}

export interface En1997LabTestsInserted {
  index: number;
  row: En1997LabTest;
}

export interface En1997LabTestsModified {
  id: string;
  patch: En1997LabTestsPatch;
}

export interface En1997LabTestsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997LabTestsPatch {
  layerId?: string | null;
  kind?: LabTestKind | null;
  voidRatio?: number | null;
  preloadStress?: number | null;
  drainageHeight?: number | null;
  consolidationTime?: number | null;
  compressionIndex?: number | null;
  swellingIndex?: number | null;
  stressIncrement?: number | null;
  secondaryCompressionIndex?: number | null;
  designLife?: number | null;
  points?: En1997LabTestsPointsRows;
}

export interface En1997LabTestsPointsInserted {
  index: number;
  row: En1997LabPoint;
}

export interface En1997LabTestsPointsModified {
  id: string;
  patch: En1997LabTestsPointsPatch;
}

export interface En1997LabTestsPointsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997LabTestsPointsPatch {
  normalStress?: number | null;
  shearStress?: number | null;
}

export interface En1997LabTestsPointsRemoved {
  id: string;
  index: number;
}

export interface En1997LabTestsPointsRows {
  removed: En1997LabTestsPointsRemoved[];
  inserted: En1997LabTestsPointsInserted[];
  moved: En1997LabTestsPointsMoved[];
  modified: En1997LabTestsPointsModified[];
}

export interface En1997LabTestsRemoved {
  id: string;
  index: number;
}

export interface En1997LabTestsRows {
  removed: En1997LabTestsRemoved[];
  inserted: En1997LabTestsInserted[];
  moved: En1997LabTestsMoved[];
  modified: En1997LabTestsModified[];
}

export interface En1997LayerWatersInserted {
  index: number;
  row: En1997LayerWater;
}

export interface En1997LayerWatersModified {
  id: string;
  patch: En1997LayerWatersPatch;
}

export interface En1997LayerWatersMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997LayerWatersPatch {
  layerId?: string | null;
  piezometricDepth?: number | null;
  permeability?: number | null;
  headDifference?: number | null;
  flowLength?: number | null;
  wellRadius?: number | null;
  wellWaterDepth?: number | null;
  influenceRadius?: number | null;
  wellCapacity?: number | null;
}

export interface En1997LayerWatersRemoved {
  id: string;
  index: number;
}

export interface En1997LayerWatersRows {
  removed: En1997LayerWatersRemoved[];
  inserted: En1997LayerWatersInserted[];
  moved: En1997LayerWatersMoved[];
  modified: En1997LayerWatersModified[];
}

export interface En1997PlacementsInserted {
  index: number;
  row: En1997Placement;
}

export interface En1997PlacementsModified {
  id: string;
  patch: En1997PlacementsPatch;
}

export interface En1997PlacementsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997PlacementsPatch {
  x?: number | null;
  y?: number | null;
  z?: number | null;
  yaw?: number | null;
}

export interface En1997PlacementsRemoved {
  id: string;
  index: number;
}

export interface En1997PlacementsRows {
  removed: En1997PlacementsRemoved[];
  inserted: En1997PlacementsInserted[];
  moved: En1997PlacementsMoved[];
  modified: En1997PlacementsModified[];
}

export interface En1997LayersInserted {
  index: number;
  row: En1997SoilLayer;
}

export interface En1997LayersModified {
  id: string;
  patch: En1997LayersPatch;
}

export interface En1997LayersMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997LayersPatch {
  soilType?: SoilType | null;
  depthTop?: number | null;
  depthBottom?: number | null;
  gamma?: number | null;
  gammaPrime?: number | null;
  phiPrimeDeg?: number | null;
  cohesionEffective?: number | null;
  cohesionUndrained?: number | null;
  oedometricModulus?: number | null;
  poissonRatio?: number | null;
  cptQc?: number | null;
  sptN?: number | null;
  cptNkt?: number | null;
}

export interface En1997LayersRemoved {
  id: string;
  index: number;
}

export interface En1997LayersRows {
  removed: En1997LayersRemoved[];
  inserted: En1997LayersInserted[];
  moved: En1997LayersMoved[];
  modified: En1997LayersModified[];
}

export interface En1997PileGroupsInserted {
  index: number;
  row: En1997PileGroup;
}

export interface En1997PileGroupsModified {
  id: string;
  patch: En1997PileGroupsPatch;
}

export interface En1997PileGroupsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997PileGroupsPatch {
  pileId?: string | null;
  rows?: number | null;
  columns?: number | null;
  spacing?: number | null;
  capVertical?: number | null;
  capMomentX?: number | null;
  capMomentY?: number | null;
  singleResistance?: number | null;
  shaftCohesionSum?: number | null;
  baseCohesion?: number | null;
}

export interface En1997PileGroupsRemoved {
  id: string;
  index: number;
}

export interface En1997PileGroupsRows {
  removed: En1997PileGroupsRemoved[];
  inserted: En1997PileGroupsInserted[];
  moved: En1997PileGroupsMoved[];
  modified: En1997PileGroupsModified[];
}

export interface En1997PilesInserted {
  index: number;
  row: En1997Pile;
}

export interface En1997PilesModified {
  id: string;
  patch: En1997PilesPatch;
}

export interface En1997PilesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997PilesPatch {
  pileType?: PileType | null;
  diameter?: number | null;
  length?: number | null;
  count?: number | null;
  alphaS?: number | null;
  unitShaftResistance?: number | null;
  unitBaseResistance?: number | null;
  compressionPermanent?: number | null;
  compressionVariable?: number | null;
  tensionPermanent?: number | null;
  tensionVariable?: number | null;
  bendingStiffnessEi?: number | null;
  horizontalSubgradeModulus?: number | null;
  lateralHeadLoad?: number | null;
  elasticModulus?: number | null;
  settlingThickness?: number | null;
  negativeFrictionBeta?: number | null;
  dynamicTestCount?: number | null;
  dynamicResistanceMean?: number | null;
  dynamicResistanceMin?: number | null;
  hammerWeight?: number | null;
  dropHeight?: number | null;
  setPerBlow?: number | null;
  elasticCompression?: number | null;
  hammerEfficiency?: number | null;
  concreteClass?: PileConcreteClass | null;
  testProfiles?: En1997PilesTestProfilesRows;
}

export interface En1997PilesRemoved {
  id: string;
  index: number;
}

export interface En1997PilesRows {
  removed: En1997PilesRemoved[];
  inserted: En1997PilesInserted[];
  moved: En1997PilesMoved[];
  modified: En1997PilesModified[];
}

export interface En1997PilesTestProfilesInserted {
  index: number;
  row: En1997PileTestProfile;
}

export interface En1997PilesTestProfilesModified {
  id: string;
  patch: En1997PilesTestProfilesPatch;
}

export interface En1997PilesTestProfilesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997PilesTestProfilesPatch {
  shaftResistance?: number | null;
  baseResistance?: number | null;
}

export interface En1997PilesTestProfilesRemoved {
  id: string;
  index: number;
}

export interface En1997PilesTestProfilesRows {
  removed: En1997PilesTestProfilesRemoved[];
  inserted: En1997PilesTestProfilesInserted[];
  moved: En1997PilesTestProfilesMoved[];
  modified: En1997PilesTestProfilesModified[];
}

export interface En1997RetainingWallsInserted {
  index: number;
  row: En1997RetainingWall;
}

export interface En1997RetainingWallsModified {
  id: string;
  patch: En1997RetainingWallsPatch;
}

export interface En1997RetainingWallsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997RetainingWallsPatch {
  height?: number | null;
  embedment?: number | null;
  baseWidth?: number | null;
  stemThickness?: number | null;
  backfillPhiDeg?: number | null;
  backfillGamma?: number | null;
  wallFrictionDeg?: number | null;
  earthPressureMode?: EarthPressureMode | null;
  wallMovement?: WallMovement | null;
  ocr?: number | null;
  concreteGamma?: number | null;
  surcharge?: number | null;
  verticalPermanent?: number | null;
  horizontalPermanent?: number | null;
  seismicKh?: number | null;
  seismicKv?: number | null;
  reinforcementSpacing?: number | null;
  reinforcementWidth?: number | null;
  reinforcementFriction?: number | null;
}

export interface En1997RetainingWallsRemoved {
  id: string;
  index: number;
}

export interface En1997RetainingWallsRows {
  removed: En1997RetainingWallsRemoved[];
  inserted: En1997RetainingWallsInserted[];
  moved: En1997RetainingWallsMoved[];
  modified: En1997RetainingWallsModified[];
}

export interface En1997SlopesInserted {
  index: number;
  row: En1997Slope;
}

export interface En1997SlopesModified {
  id: string;
  patch: En1997SlopesPatch;
}

export interface En1997SlopesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997SlopesPatch {
  angleDeg?: number | null;
  height?: number | null;
  length?: number | null;
  governingLayerId?: string | null;
  circleCentreX?: number | null;
  circleCentreZ?: number | null;
  circleRadius?: number | null;
  width?: number | null;
}

export interface En1997SlopesRemoved {
  id: string;
  index: number;
}

export interface En1997SlopesRows {
  removed: En1997SlopesRemoved[];
  inserted: En1997SlopesInserted[];
  moved: En1997SlopesMoved[];
  modified: En1997SlopesModified[];
}

export interface En1997UpliftCasesInserted {
  index: number;
  row: En1997UpliftCase;
}

export interface En1997UpliftCasesModified {
  id: string;
  patch: En1997UpliftCasesPatch;
}

export interface En1997UpliftCasesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997UpliftCasesPatch {
  permanentStabilizing?: number | null;
  permanentDestabilizing?: number | null;
  variableDestabilizing?: number | null;
  porePressure?: number | null;
  totalStress?: number | null;
}

export interface En1997UpliftCasesRemoved {
  id: string;
  index: number;
}

export interface En1997UpliftCasesRows {
  removed: En1997UpliftCasesRemoved[];
  inserted: En1997UpliftCasesInserted[];
  moved: En1997UpliftCasesMoved[];
  modified: En1997UpliftCasesModified[];
}

export interface En1997InvestigationsInserted {
  index: number;
  row: En1997Investigation;
}

export interface En1997InvestigationsModified {
  id: string;
  patch: En1997InvestigationsPatch;
}

export interface En1997InvestigationsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997InvestigationsRemoved {
  id: string;
  index: number;
}

export interface En1997InvestigationsRows {
  removed: En1997InvestigationsRemoved[];
  inserted: En1997InvestigationsInserted[];
  moved: En1997InvestigationsMoved[];
  modified: En1997InvestigationsModified[];
}

export interface En1997InvestigationsReadingsInserted {
  index: number;
  row: En1997InvestigationReading;
}

export interface En1997InvestigationsReadingsModified {
  id: string;
  patch: En1997InvestigationsReadingsPatch;
}

export interface En1997InvestigationsReadingsMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997InvestigationsReadingsRemoved {
  id: string;
  index: number;
}

export interface En1997InvestigationsReadingsRows {
  removed: En1997InvestigationsReadingsRemoved[];
  inserted: En1997InvestigationsReadingsInserted[];
  moved: En1997InvestigationsReadingsMoved[];
  modified: En1997InvestigationsReadingsModified[];
}

export interface En1997InvestigationsReadingsPatch {
  depth?: number | null;
  measurement?: number | null;
}

export interface En1997InvestigationsPatch {
  positionX?: number | null;
  positionY?: number | null;
  groundLevel?: number | null;
  depth?: number | null;
  kind?: InvestigationKind | null;
  readings?: En1997InvestigationsReadingsRows;
}

export interface En1997CharacteristicBasesInserted {
  index: number;
  row: En1997CharacteristicBasis;
}

export interface En1997CharacteristicBasesModified {
  id: string;
  patch: En1997CharacteristicBasesPatch;
}

export interface En1997CharacteristicBasesMoved {
  id: string;
  from: number;
  to: number;
}

export interface En1997CharacteristicBasesRemoved {
  id: string;
  index: number;
}

export interface En1997CharacteristicBasesRows {
  removed: En1997CharacteristicBasesRemoved[];
  inserted: En1997CharacteristicBasesInserted[];
  moved: En1997CharacteristicBasesMoved[];
  modified: En1997CharacteristicBasesModified[];
}

export interface En1997CharacteristicBasesPatch {
  layerId?: string | null;
  parameter?: CharacteristicParameter | null;
  basis?: ParameterBasis | null;
}

export const parseEn1997Diff: NormWireReader<En1997Diff> = normWireObject<En1997Diff>({ structureId: normWireOptional(normWireNullable(normWireString)), geotechnicalCategory: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":255}))), designSituation: normWireOptional(normWireNullable(parseSituationChoice)), designApproach: normWireOptional(normWireNullable(parseApproachChoice)), annex: normWireOptional(normWireNullable(parseEn1997AnnexChoice)), groundwaterLevel: normWireOptional(normWireNullable(normWireNumber)), investigationDepth: normWireOptional(normWireNullable(normWireNumber)), layers: normWireOptional(normWireNullable(normWireRef(() => parseEn1997LayersRows))), footings: normWireOptional(normWireNullable(normWireRef(() => parseEn1997FootingsRows))), piles: normWireOptional(normWireNullable(normWireRef(() => parseEn1997PilesRows))), retainingWalls: normWireOptional(normWireNullable(normWireRef(() => parseEn1997RetainingWallsRows))), slopes: normWireOptional(normWireNullable(normWireRef(() => parseEn1997SlopesRows))), upliftCases: normWireOptional(normWireNullable(normWireRef(() => parseEn1997UpliftCasesRows))), anchors: normWireOptional(normWireNullable(normWireRef(() => parseEn1997AnchorsRows))), embeddedWalls: normWireOptional(normWireNullable(normWireRef(() => parseEn1997EmbeddedWallsRows))), pileGroups: normWireOptional(normWireNullable(normWireRef(() => parseEn1997PileGroupsRows))), labTests: normWireOptional(normWireNullable(normWireRef(() => parseEn1997LabTestsRows))), layerWaters: normWireOptional(normWireNullable(normWireRef(() => parseEn1997LayerWatersRows))), placements: normWireOptional(normWireNullable(normWireRef(() => parseEn1997PlacementsRows))), cases: normWireRequired(normWireRef(() => parseCaseRowDelta)), investigations: normWireOptional(normWireNullable(normWireRef(() => parseEn1997InvestigationsRows))), characteristicBases: normWireOptional(normWireNullable(normWireRef(() => parseEn1997CharacteristicBasesRows))) });
export const parseCaseNumberDelta: NormWireReader<CaseNumberDelta> = normWireObject<CaseNumberDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseNumberModification))) });
export const parseCaseNumberInsertion: NormWireReader<CaseNumberInsertion> = normWireObject<CaseNumberInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997CaseNumber) });
export const parseCaseNumberModification: NormWireReader<CaseNumberModification> = normWireObject<CaseNumberModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseNumberPatch)) });
export const parseCaseNumberPatch: NormWireReader<CaseNumberPatch> = normWireObject<CaseNumberPatch>({ value: normWireOptional(normWireNullable(normWireNumber)) });
export const parseCaseNumberRelocation: NormWireReader<CaseNumberRelocation> = normWireObject<CaseNumberRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseNumberRemoval: NormWireReader<CaseNumberRemoval> = normWireObject<CaseNumberRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowDelta: NormWireReader<CaseRowDelta> = normWireObject<CaseRowDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseRowInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseRowRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseRowModification))) });
export const parseCaseRowInsertion: NormWireReader<CaseRowInsertion> = normWireObject<CaseRowInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997CaseRow) });
export const parseCaseRowModification: NormWireReader<CaseRowModification> = normWireObject<CaseRowModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseRowPatch)) });
export const parseCaseRowPatch: NormWireReader<CaseRowPatch> = normWireObject<CaseRowPatch>({ labelEn: normWireOptional(normWireNullable(normWireString)), labelDe: normWireOptional(normWireNullable(normWireString)), entryId: normWireOptional(normWireNullable(normWireString)), numbers: normWireOptional(normWireRef(() => parseCaseNumberDelta)), texts: normWireOptional(normWireRef(() => parseCaseTextDelta)) });
export const parseCaseRowRelocation: NormWireReader<CaseRowRelocation> = normWireObject<CaseRowRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseRowRemoval: NormWireReader<CaseRowRemoval> = normWireObject<CaseRowRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseTextDelta: NormWireReader<CaseTextDelta> = normWireObject<CaseTextDelta>({ removed: normWireRequired(normWireArray(normWireRef(() => parseCaseTextRemoval))), inserted: normWireRequired(normWireArray(normWireRef(() => parseCaseTextInsertion))), moved: normWireRequired(normWireArray(normWireRef(() => parseCaseTextRelocation))), modified: normWireRequired(normWireArray(normWireRef(() => parseCaseTextModification))) });
export const parseCaseTextInsertion: NormWireReader<CaseTextInsertion> = normWireObject<CaseTextInsertion>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997CaseText) });
export const parseCaseTextModification: NormWireReader<CaseTextModification> = normWireObject<CaseTextModification>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseCaseTextPatch)) });
export const parseCaseTextPatch: NormWireReader<CaseTextPatch> = normWireObject<CaseTextPatch>({ value: normWireOptional(normWireNullable(normWireString)) });
export const parseCaseTextRelocation: NormWireReader<CaseTextRelocation> = normWireObject<CaseTextRelocation>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCaseTextRemoval: NormWireReader<CaseTextRemoval> = normWireObject<CaseTextRemoval>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997AnchorsInserted: NormWireReader<En1997AnchorsInserted> = normWireObject<En1997AnchorsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997Anchor) });
export const parseEn1997AnchorsModified: NormWireReader<En1997AnchorsModified> = normWireObject<En1997AnchorsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997AnchorsPatch)) });
export const parseEn1997AnchorsMoved: NormWireReader<En1997AnchorsMoved> = normWireObject<En1997AnchorsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997AnchorsPatch: NormWireReader<En1997AnchorsPatch> = normWireObject<En1997AnchorsPatch>({ wallId: normWireOptional(normWireNullable(normWireString)), tendon: normWireOptional(normWireNullable(parseTendonType)), headDepth: normWireOptional(normWireNullable(normWireNumber)), inclinationDeg: normWireOptional(normWireNullable(normWireNumber)), freeLength: normWireOptional(normWireNullable(normWireNumber)), fixedLength: normWireOptional(normWireNullable(normWireNumber)), fixedDiameter: normWireOptional(normWireNullable(normWireNumber)), bondStress: normWireOptional(normWireNullable(normWireNumber)), designForce: normWireOptional(normWireNullable(normWireNumber)), tendonYieldForce: normWireOptional(normWireNullable(normWireNumber)), minFreeLength: normWireOptional(normWireNullable(normWireNumber)), testFactor: normWireOptional(normWireNullable(normWireNumber)), tests: normWireOptional(normWireRef(() => parseEn1997AnchorsTestsRows)) });
export const parseEn1997AnchorsRemoved: NormWireReader<En1997AnchorsRemoved> = normWireObject<En1997AnchorsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997AnchorsRows: NormWireReader<En1997AnchorsRows> = normWireObject<En1997AnchorsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997AnchorsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997AnchorsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997AnchorsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997AnchorsModified))) });
export const parseEn1997AnchorsTestsInserted: NormWireReader<En1997AnchorsTestsInserted> = normWireObject<En1997AnchorsTestsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997AnchorTest) });
export const parseEn1997AnchorsTestsModified: NormWireReader<En1997AnchorsTestsModified> = normWireObject<En1997AnchorsTestsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997AnchorsTestsPatch)) });
export const parseEn1997AnchorsTestsMoved: NormWireReader<En1997AnchorsTestsMoved> = normWireObject<En1997AnchorsTestsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997AnchorsTestsPatch: NormWireReader<En1997AnchorsTestsPatch> = normWireObject<En1997AnchorsTestsPatch>({ pullOutResistance: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997AnchorsTestsRemoved: NormWireReader<En1997AnchorsTestsRemoved> = normWireObject<En1997AnchorsTestsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997AnchorsTestsRows: NormWireReader<En1997AnchorsTestsRows> = normWireObject<En1997AnchorsTestsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997AnchorsTestsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997AnchorsTestsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997AnchorsTestsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997AnchorsTestsModified))) });
export const parseEn1997EmbeddedWallsInserted: NormWireReader<En1997EmbeddedWallsInserted> = normWireObject<En1997EmbeddedWallsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997EmbeddedWall) });
export const parseEn1997EmbeddedWallsModified: NormWireReader<En1997EmbeddedWallsModified> = normWireObject<En1997EmbeddedWallsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997EmbeddedWallsPatch)) });
export const parseEn1997EmbeddedWallsMoved: NormWireReader<En1997EmbeddedWallsMoved> = normWireObject<En1997EmbeddedWallsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997EmbeddedWallsPatch: NormWireReader<En1997EmbeddedWallsPatch> = normWireObject<En1997EmbeddedWallsPatch>({ kind: normWireOptional(normWireNullable(parseEmbeddedWallKind)), retainedHeight: normWireOptional(normWireNullable(normWireNumber)), embedment: normWireOptional(normWireNullable(normWireNumber)), propDepth: normWireOptional(normWireNullable(normWireNumber)), thickness: normWireOptional(normWireNullable(normWireNumber)), backfillGamma: normWireOptional(normWireNullable(normWireNumber)), phiDeg: normWireOptional(normWireNullable(normWireNumber)), deltaActiveDeg: normWireOptional(normWireNullable(normWireNumber)), deltaPassiveDeg: normWireOptional(normWireNullable(normWireNumber)), surcharge: normWireOptional(normWireNullable(normWireNumber)), undrainedStrength: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997EmbeddedWallsRemoved: NormWireReader<En1997EmbeddedWallsRemoved> = normWireObject<En1997EmbeddedWallsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997EmbeddedWallsRows: NormWireReader<En1997EmbeddedWallsRows> = normWireObject<En1997EmbeddedWallsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997EmbeddedWallsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997EmbeddedWallsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997EmbeddedWallsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997EmbeddedWallsModified))) });
export const parseEn1997FootingsInserted: NormWireReader<En1997FootingsInserted> = normWireObject<En1997FootingsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997SpreadFoundation) });
export const parseEn1997FootingsLoadCasesInserted: NormWireReader<En1997FootingsLoadCasesInserted> = normWireObject<En1997FootingsLoadCasesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997FoundationLoadCase) });
export const parseEn1997FootingsLoadCasesModified: NormWireReader<En1997FootingsLoadCasesModified> = normWireObject<En1997FootingsLoadCasesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997FootingsLoadCasesPatch)) });
export const parseEn1997FootingsLoadCasesMoved: NormWireReader<En1997FootingsLoadCasesMoved> = normWireObject<En1997FootingsLoadCasesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997FootingsLoadCasesPatch: NormWireReader<En1997FootingsLoadCasesPatch> = normWireObject<En1997FootingsLoadCasesPatch>({ designSituation: normWireOptional(normWireNullable(parseSituationChoice)), verticalPermanent: normWireOptional(normWireNullable(normWireNumber)), verticalVariable: normWireOptional(normWireNullable(normWireNumber)), horizontalPermanent: normWireOptional(normWireNullable(normWireNumber)), horizontalVariable: normWireOptional(normWireNullable(normWireNumber)), momentPermanent: normWireOptional(normWireNullable(normWireNumber)), momentVariable: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997FootingsLoadCasesRemoved: NormWireReader<En1997FootingsLoadCasesRemoved> = normWireObject<En1997FootingsLoadCasesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997FootingsLoadCasesRows: NormWireReader<En1997FootingsLoadCasesRows> = normWireObject<En1997FootingsLoadCasesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsLoadCasesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsLoadCasesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsLoadCasesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsLoadCasesModified))) });
export const parseEn1997FootingsModified: NormWireReader<En1997FootingsModified> = normWireObject<En1997FootingsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997FootingsPatch)) });
export const parseEn1997FootingsMoved: NormWireReader<En1997FootingsMoved> = normWireObject<En1997FootingsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997FootingsPatch: NormWireReader<En1997FootingsPatch> = normWireObject<En1997FootingsPatch>({ width: normWireOptional(normWireNullable(normWireNumber)), length: normWireOptional(normWireNullable(normWireNumber)), embedment: normWireOptional(normWireNullable(normWireNumber)), baseInclinationDeg: normWireOptional(normWireNullable(normWireNumber)), settlementLimit: normWireOptional(normWireNullable(normWireNumber)), loadCases: normWireOptional(normWireRef(() => parseEn1997FootingsLoadCasesRows)) });
export const parseEn1997FootingsRemoved: NormWireReader<En1997FootingsRemoved> = normWireObject<En1997FootingsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997FootingsRows: NormWireReader<En1997FootingsRows> = normWireObject<En1997FootingsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997FootingsModified))) });
export const parseEn1997LabTestsInserted: NormWireReader<En1997LabTestsInserted> = normWireObject<En1997LabTestsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997LabTest) });
export const parseEn1997LabTestsModified: NormWireReader<En1997LabTestsModified> = normWireObject<En1997LabTestsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997LabTestsPatch)) });
export const parseEn1997LabTestsMoved: NormWireReader<En1997LabTestsMoved> = normWireObject<En1997LabTestsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997LabTestsPatch: NormWireReader<En1997LabTestsPatch> = normWireObject<En1997LabTestsPatch>({ layerId: normWireOptional(normWireNullable(normWireString)), kind: normWireOptional(normWireNullable(parseLabTestKind)), voidRatio: normWireOptional(normWireNullable(normWireNumber)), preloadStress: normWireOptional(normWireNullable(normWireNumber)), drainageHeight: normWireOptional(normWireNullable(normWireNumber)), consolidationTime: normWireOptional(normWireNullable(normWireNumber)), compressionIndex: normWireOptional(normWireNullable(normWireNumber)), swellingIndex: normWireOptional(normWireNullable(normWireNumber)), stressIncrement: normWireOptional(normWireNullable(normWireNumber)), secondaryCompressionIndex: normWireOptional(normWireNullable(normWireNumber)), designLife: normWireOptional(normWireNullable(normWireNumber)), points: normWireOptional(normWireRef(() => parseEn1997LabTestsPointsRows)) });
export const parseEn1997LabTestsPointsInserted: NormWireReader<En1997LabTestsPointsInserted> = normWireObject<En1997LabTestsPointsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997LabPoint) });
export const parseEn1997LabTestsPointsModified: NormWireReader<En1997LabTestsPointsModified> = normWireObject<En1997LabTestsPointsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997LabTestsPointsPatch)) });
export const parseEn1997LabTestsPointsMoved: NormWireReader<En1997LabTestsPointsMoved> = normWireObject<En1997LabTestsPointsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997LabTestsPointsPatch: NormWireReader<En1997LabTestsPointsPatch> = normWireObject<En1997LabTestsPointsPatch>({ normalStress: normWireOptional(normWireNullable(normWireNumber)), shearStress: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997LabTestsPointsRemoved: NormWireReader<En1997LabTestsPointsRemoved> = normWireObject<En1997LabTestsPointsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997LabTestsPointsRows: NormWireReader<En1997LabTestsPointsRows> = normWireObject<En1997LabTestsPointsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997LabTestsPointsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997LabTestsPointsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997LabTestsPointsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997LabTestsPointsModified))) });
export const parseEn1997LabTestsRemoved: NormWireReader<En1997LabTestsRemoved> = normWireObject<En1997LabTestsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997LabTestsRows: NormWireReader<En1997LabTestsRows> = normWireObject<En1997LabTestsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997LabTestsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997LabTestsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997LabTestsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997LabTestsModified))) });
export const parseEn1997LayerWatersInserted: NormWireReader<En1997LayerWatersInserted> = normWireObject<En1997LayerWatersInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997LayerWater) });
export const parseEn1997LayerWatersModified: NormWireReader<En1997LayerWatersModified> = normWireObject<En1997LayerWatersModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997LayerWatersPatch)) });
export const parseEn1997LayerWatersMoved: NormWireReader<En1997LayerWatersMoved> = normWireObject<En1997LayerWatersMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997LayerWatersPatch: NormWireReader<En1997LayerWatersPatch> = normWireObject<En1997LayerWatersPatch>({ layerId: normWireOptional(normWireNullable(normWireString)), piezometricDepth: normWireOptional(normWireNullable(normWireNumber)), permeability: normWireOptional(normWireNullable(normWireNumber)), headDifference: normWireOptional(normWireNullable(normWireNumber)), flowLength: normWireOptional(normWireNullable(normWireNumber)), wellRadius: normWireOptional(normWireNullable(normWireNumber)), wellWaterDepth: normWireOptional(normWireNullable(normWireNumber)), influenceRadius: normWireOptional(normWireNullable(normWireNumber)), wellCapacity: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997LayerWatersRemoved: NormWireReader<En1997LayerWatersRemoved> = normWireObject<En1997LayerWatersRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997LayerWatersRows: NormWireReader<En1997LayerWatersRows> = normWireObject<En1997LayerWatersRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayerWatersRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayerWatersInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayerWatersMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayerWatersModified))) });
export const parseEn1997PlacementsInserted: NormWireReader<En1997PlacementsInserted> = normWireObject<En1997PlacementsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997Placement) });
export const parseEn1997PlacementsModified: NormWireReader<En1997PlacementsModified> = normWireObject<En1997PlacementsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997PlacementsPatch)) });
export const parseEn1997PlacementsMoved: NormWireReader<En1997PlacementsMoved> = normWireObject<En1997PlacementsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997PlacementsPatch: NormWireReader<En1997PlacementsPatch> = normWireObject<En1997PlacementsPatch>({ x: normWireOptional(normWireNullable(normWireNumber)), y: normWireOptional(normWireNullable(normWireNumber)), z: normWireOptional(normWireNullable(normWireNumber)), yaw: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997PlacementsRemoved: NormWireReader<En1997PlacementsRemoved> = normWireObject<En1997PlacementsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997PlacementsRows: NormWireReader<En1997PlacementsRows> = normWireObject<En1997PlacementsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997PlacementsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997PlacementsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997PlacementsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997PlacementsModified))) });
export const parseEn1997LayersInserted: NormWireReader<En1997LayersInserted> = normWireObject<En1997LayersInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997SoilLayer) });
export const parseEn1997LayersModified: NormWireReader<En1997LayersModified> = normWireObject<En1997LayersModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997LayersPatch)) });
export const parseEn1997LayersMoved: NormWireReader<En1997LayersMoved> = normWireObject<En1997LayersMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997LayersPatch: NormWireReader<En1997LayersPatch> = normWireObject<En1997LayersPatch>({ soilType: normWireOptional(normWireNullable(parseSoilType)), depthTop: normWireOptional(normWireNullable(normWireNumber)), depthBottom: normWireOptional(normWireNullable(normWireNumber)), gamma: normWireOptional(normWireNullable(normWireNumber)), gammaPrime: normWireOptional(normWireNullable(normWireNumber)), phiPrimeDeg: normWireOptional(normWireNullable(normWireNumber)), cohesionEffective: normWireOptional(normWireNullable(normWireNumber)), cohesionUndrained: normWireOptional(normWireNullable(normWireNumber)), oedometricModulus: normWireOptional(normWireNullable(normWireNumber)), poissonRatio: normWireOptional(normWireNullable(normWireNumber)), cptQc: normWireOptional(normWireNullable(normWireNumber)), sptN: normWireOptional(normWireNullable(normWireNumber)), cptNkt: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997LayersRemoved: NormWireReader<En1997LayersRemoved> = normWireObject<En1997LayersRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997LayersRows: NormWireReader<En1997LayersRows> = normWireObject<En1997LayersRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayersRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayersInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayersMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997LayersModified))) });
export const parseEn1997PileGroupsInserted: NormWireReader<En1997PileGroupsInserted> = normWireObject<En1997PileGroupsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997PileGroup) });
export const parseEn1997PileGroupsModified: NormWireReader<En1997PileGroupsModified> = normWireObject<En1997PileGroupsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997PileGroupsPatch)) });
export const parseEn1997PileGroupsMoved: NormWireReader<En1997PileGroupsMoved> = normWireObject<En1997PileGroupsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997PileGroupsPatch: NormWireReader<En1997PileGroupsPatch> = normWireObject<En1997PileGroupsPatch>({ pileId: normWireOptional(normWireNullable(normWireString)), rows: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), columns: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), spacing: normWireOptional(normWireNullable(normWireNumber)), capVertical: normWireOptional(normWireNullable(normWireNumber)), capMomentX: normWireOptional(normWireNullable(normWireNumber)), capMomentY: normWireOptional(normWireNullable(normWireNumber)), singleResistance: normWireOptional(normWireNullable(normWireNumber)), shaftCohesionSum: normWireOptional(normWireNullable(normWireNumber)), baseCohesion: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997PileGroupsRemoved: NormWireReader<En1997PileGroupsRemoved> = normWireObject<En1997PileGroupsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997PileGroupsRows: NormWireReader<En1997PileGroupsRows> = normWireObject<En1997PileGroupsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997PileGroupsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997PileGroupsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997PileGroupsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997PileGroupsModified))) });
export const parseEn1997PilesInserted: NormWireReader<En1997PilesInserted> = normWireObject<En1997PilesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997Pile) });
export const parseEn1997PilesModified: NormWireReader<En1997PilesModified> = normWireObject<En1997PilesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997PilesPatch)) });
export const parseEn1997PilesMoved: NormWireReader<En1997PilesMoved> = normWireObject<En1997PilesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997PilesPatch: NormWireReader<En1997PilesPatch> = normWireObject<En1997PilesPatch>({ pileType: normWireOptional(normWireNullable(parsePileType)), diameter: normWireOptional(normWireNullable(normWireNumber)), length: normWireOptional(normWireNullable(normWireNumber)), count: normWireOptional(normWireNullable(normWireRange(normWireInteger, {"minimum":0,"maximum":4294967295}))), alphaS: normWireOptional(normWireNullable(normWireNumber)), unitShaftResistance: normWireOptional(normWireNullable(normWireNumber)), unitBaseResistance: normWireOptional(normWireNullable(normWireNumber)), compressionPermanent: normWireOptional(normWireNullable(normWireNumber)), compressionVariable: normWireOptional(normWireNullable(normWireNumber)), tensionPermanent: normWireOptional(normWireNullable(normWireNumber)), tensionVariable: normWireOptional(normWireNullable(normWireNumber)), bendingStiffnessEi: normWireOptional(normWireNullable(normWireNumber)), horizontalSubgradeModulus: normWireOptional(normWireNullable(normWireNumber)), lateralHeadLoad: normWireOptional(normWireNullable(normWireNumber)), elasticModulus: normWireOptional(normWireNullable(normWireNumber)), settlingThickness: normWireOptional(normWireNullable(normWireNumber)), negativeFrictionBeta: normWireOptional(normWireNullable(normWireNumber)), dynamicTestCount: normWireOptional(normWireNullable(normWireNumber)), dynamicResistanceMean: normWireOptional(normWireNullable(normWireNumber)), dynamicResistanceMin: normWireOptional(normWireNullable(normWireNumber)), hammerWeight: normWireOptional(normWireNullable(normWireNumber)), dropHeight: normWireOptional(normWireNullable(normWireNumber)), setPerBlow: normWireOptional(normWireNullable(normWireNumber)), elasticCompression: normWireOptional(normWireNullable(normWireNumber)), hammerEfficiency: normWireOptional(normWireNullable(normWireNumber)), concreteClass: normWireOptional(normWireNullable(parsePileConcreteClass)), testProfiles: normWireOptional(normWireRef(() => parseEn1997PilesTestProfilesRows)) });
export const parseEn1997PilesRemoved: NormWireReader<En1997PilesRemoved> = normWireObject<En1997PilesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997PilesRows: NormWireReader<En1997PilesRows> = normWireObject<En1997PilesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesModified))) });
export const parseEn1997PilesTestProfilesInserted: NormWireReader<En1997PilesTestProfilesInserted> = normWireObject<En1997PilesTestProfilesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997PileTestProfile) });
export const parseEn1997PilesTestProfilesModified: NormWireReader<En1997PilesTestProfilesModified> = normWireObject<En1997PilesTestProfilesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997PilesTestProfilesPatch)) });
export const parseEn1997PilesTestProfilesMoved: NormWireReader<En1997PilesTestProfilesMoved> = normWireObject<En1997PilesTestProfilesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997PilesTestProfilesPatch: NormWireReader<En1997PilesTestProfilesPatch> = normWireObject<En1997PilesTestProfilesPatch>({ shaftResistance: normWireOptional(normWireNullable(normWireNumber)), baseResistance: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997PilesTestProfilesRemoved: NormWireReader<En1997PilesTestProfilesRemoved> = normWireObject<En1997PilesTestProfilesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997PilesTestProfilesRows: NormWireReader<En1997PilesTestProfilesRows> = normWireObject<En1997PilesTestProfilesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesTestProfilesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesTestProfilesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesTestProfilesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997PilesTestProfilesModified))) });
export const parseEn1997RetainingWallsInserted: NormWireReader<En1997RetainingWallsInserted> = normWireObject<En1997RetainingWallsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997RetainingWall) });
export const parseEn1997RetainingWallsModified: NormWireReader<En1997RetainingWallsModified> = normWireObject<En1997RetainingWallsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997RetainingWallsPatch)) });
export const parseEn1997RetainingWallsMoved: NormWireReader<En1997RetainingWallsMoved> = normWireObject<En1997RetainingWallsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997RetainingWallsPatch: NormWireReader<En1997RetainingWallsPatch> = normWireObject<En1997RetainingWallsPatch>({ height: normWireOptional(normWireNullable(normWireNumber)), embedment: normWireOptional(normWireNullable(normWireNumber)), baseWidth: normWireOptional(normWireNullable(normWireNumber)), stemThickness: normWireOptional(normWireNullable(normWireNumber)), backfillPhiDeg: normWireOptional(normWireNullable(normWireNumber)), backfillGamma: normWireOptional(normWireNullable(normWireNumber)), wallFrictionDeg: normWireOptional(normWireNullable(normWireNumber)), earthPressureMode: normWireOptional(normWireNullable(parseEarthPressureMode)), wallMovement: normWireOptional(normWireNullable(parseWallMovement)), ocr: normWireOptional(normWireNullable(normWireNumber)), concreteGamma: normWireOptional(normWireNullable(normWireNumber)), surcharge: normWireOptional(normWireNullable(normWireNumber)), verticalPermanent: normWireOptional(normWireNullable(normWireNumber)), horizontalPermanent: normWireOptional(normWireNullable(normWireNumber)), seismicKh: normWireOptional(normWireNullable(normWireNumber)), seismicKv: normWireOptional(normWireNullable(normWireNumber)), reinforcementSpacing: normWireOptional(normWireNullable(normWireNumber)), reinforcementWidth: normWireOptional(normWireNullable(normWireNumber)), reinforcementFriction: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997RetainingWallsRemoved: NormWireReader<En1997RetainingWallsRemoved> = normWireObject<En1997RetainingWallsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997RetainingWallsRows: NormWireReader<En1997RetainingWallsRows> = normWireObject<En1997RetainingWallsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997RetainingWallsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997RetainingWallsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997RetainingWallsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997RetainingWallsModified))) });
export const parseEn1997SlopesInserted: NormWireReader<En1997SlopesInserted> = normWireObject<En1997SlopesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997Slope) });
export const parseEn1997SlopesModified: NormWireReader<En1997SlopesModified> = normWireObject<En1997SlopesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997SlopesPatch)) });
export const parseEn1997SlopesMoved: NormWireReader<En1997SlopesMoved> = normWireObject<En1997SlopesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997SlopesPatch: NormWireReader<En1997SlopesPatch> = normWireObject<En1997SlopesPatch>({ angleDeg: normWireOptional(normWireNullable(normWireNumber)), height: normWireOptional(normWireNullable(normWireNumber)), length: normWireOptional(normWireNullable(normWireNumber)), governingLayerId: normWireOptional(normWireNullable(normWireString)), circleCentreX: normWireOptional(normWireNullable(normWireNumber)), circleCentreZ: normWireOptional(normWireNullable(normWireNumber)), circleRadius: normWireOptional(normWireNullable(normWireNumber)), width: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997SlopesRemoved: NormWireReader<En1997SlopesRemoved> = normWireObject<En1997SlopesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997SlopesRows: NormWireReader<En1997SlopesRows> = normWireObject<En1997SlopesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997SlopesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997SlopesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997SlopesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997SlopesModified))) });
export const parseEn1997UpliftCasesInserted: NormWireReader<En1997UpliftCasesInserted> = normWireObject<En1997UpliftCasesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997UpliftCase) });
export const parseEn1997UpliftCasesModified: NormWireReader<En1997UpliftCasesModified> = normWireObject<En1997UpliftCasesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997UpliftCasesPatch)) });
export const parseEn1997UpliftCasesMoved: NormWireReader<En1997UpliftCasesMoved> = normWireObject<En1997UpliftCasesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997UpliftCasesPatch: NormWireReader<En1997UpliftCasesPatch> = normWireObject<En1997UpliftCasesPatch>({ permanentStabilizing: normWireOptional(normWireNullable(normWireNumber)), permanentDestabilizing: normWireOptional(normWireNullable(normWireNumber)), variableDestabilizing: normWireOptional(normWireNullable(normWireNumber)), porePressure: normWireOptional(normWireNullable(normWireNumber)), totalStress: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997UpliftCasesRemoved: NormWireReader<En1997UpliftCasesRemoved> = normWireObject<En1997UpliftCasesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997UpliftCasesRows: NormWireReader<En1997UpliftCasesRows> = normWireObject<En1997UpliftCasesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997UpliftCasesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997UpliftCasesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997UpliftCasesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997UpliftCasesModified))) });
export const parseEn1997InvestigationsInserted: NormWireReader<En1997InvestigationsInserted> = normWireObject<En1997InvestigationsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997Investigation) });
export const parseEn1997InvestigationsModified: NormWireReader<En1997InvestigationsModified> = normWireObject<En1997InvestigationsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997InvestigationsPatch)) });
export const parseEn1997InvestigationsMoved: NormWireReader<En1997InvestigationsMoved> = normWireObject<En1997InvestigationsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997InvestigationsRemoved: NormWireReader<En1997InvestigationsRemoved> = normWireObject<En1997InvestigationsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997InvestigationsRows: NormWireReader<En1997InvestigationsRows> = normWireObject<En1997InvestigationsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997InvestigationsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997InvestigationsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997InvestigationsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997InvestigationsModified))) });
export const parseEn1997InvestigationsReadingsInserted: NormWireReader<En1997InvestigationsReadingsInserted> = normWireObject<En1997InvestigationsReadingsInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997InvestigationReading) });
export const parseEn1997InvestigationsReadingsModified: NormWireReader<En1997InvestigationsReadingsModified> = normWireObject<En1997InvestigationsReadingsModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997InvestigationsReadingsPatch)) });
export const parseEn1997InvestigationsReadingsMoved: NormWireReader<En1997InvestigationsReadingsMoved> = normWireObject<En1997InvestigationsReadingsMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997InvestigationsReadingsRemoved: NormWireReader<En1997InvestigationsReadingsRemoved> = normWireObject<En1997InvestigationsReadingsRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997InvestigationsReadingsRows: NormWireReader<En1997InvestigationsReadingsRows> = normWireObject<En1997InvestigationsReadingsRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997InvestigationsReadingsRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997InvestigationsReadingsInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997InvestigationsReadingsMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997InvestigationsReadingsModified))) });
export const parseEn1997InvestigationsReadingsPatch: NormWireReader<En1997InvestigationsReadingsPatch> = normWireObject<En1997InvestigationsReadingsPatch>({ depth: normWireOptional(normWireNullable(normWireNumber)), measurement: normWireOptional(normWireNullable(normWireNumber)) });
export const parseEn1997InvestigationsPatch: NormWireReader<En1997InvestigationsPatch> = normWireObject<En1997InvestigationsPatch>({ positionX: normWireOptional(normWireNullable(normWireNumber)), positionY: normWireOptional(normWireNullable(normWireNumber)), groundLevel: normWireOptional(normWireNullable(normWireNumber)), depth: normWireOptional(normWireNullable(normWireNumber)), kind: normWireOptional(normWireNullable(parseInvestigationKind)), readings: normWireOptional(normWireRef(() => parseEn1997InvestigationsReadingsRows)) });
export const parseEn1997CharacteristicBasesInserted: NormWireReader<En1997CharacteristicBasesInserted> = normWireObject<En1997CharacteristicBasesInserted>({ index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), row: normWireRequired(parseEn1997CharacteristicBasis) });
export const parseEn1997CharacteristicBasesModified: NormWireReader<En1997CharacteristicBasesModified> = normWireObject<En1997CharacteristicBasesModified>({ id: normWireRequired(normWireString), patch: normWireRequired(normWireRef(() => parseEn1997CharacteristicBasesPatch)) });
export const parseEn1997CharacteristicBasesMoved: NormWireReader<En1997CharacteristicBasesMoved> = normWireObject<En1997CharacteristicBasesMoved>({ id: normWireRequired(normWireString), from: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), to: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997CharacteristicBasesRemoved: NormWireReader<En1997CharacteristicBasesRemoved> = normWireObject<En1997CharacteristicBasesRemoved>({ id: normWireRequired(normWireString), index: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseEn1997CharacteristicBasesRows: NormWireReader<En1997CharacteristicBasesRows> = normWireObject<En1997CharacteristicBasesRows>({ removed: normWireRequired(normWireArray(normWireRef(() => parseEn1997CharacteristicBasesRemoved))), inserted: normWireRequired(normWireArray(normWireRef(() => parseEn1997CharacteristicBasesInserted))), moved: normWireRequired(normWireArray(normWireRef(() => parseEn1997CharacteristicBasesMoved))), modified: normWireRequired(normWireArray(normWireRef(() => parseEn1997CharacteristicBasesModified))) });
export const parseEn1997CharacteristicBasesPatch: NormWireReader<En1997CharacteristicBasesPatch> = normWireObject<En1997CharacteristicBasesPatch>({ layerId: normWireOptional(normWireNullable(normWireString)), parameter: normWireOptional(normWireNullable(parseCharacteristicParameter)), basis: normWireOptional(normWireNullable(parseParameterBasis)) });
