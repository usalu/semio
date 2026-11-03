/** ⚖️ `CheckReport` wire twin: the compliance report every norm artifact's checks produce, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `repo ./📜️script.ts schema twins generate`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireAny, normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireOptional, normWireRange, normWireRef, normWireRequired, normWireString, type NormWireReader } from "../../📇️registry/🧬️contract/🟦️.ts";

export interface CheckReport {
  summary: CheckReportSummary;
  checks: CheckResult[];
}

export type QuantityKind = "Dimensionless" | "Length" | "Area" | "Volume" | "Mass" | "Time" | "Temperature" | "Force" | "Pressure" | "Stress" | "Moment" | "Energy" | "Power" | "ThermalConductivity" | "ThermalResistance" | "HeatTransferCoefficient" | "AirPermeability" | "VentilationRate" | "Acceleration" | "Velocity";

export interface Quantity {
  kind: QuantityKind;
  value: number;
}

export interface ClauseId {
  family: string;
  part: string;
  section: string;
}

export interface LocalizedCopy {
  en: string;
  de: string;
}

export interface SubjectRef {
  entityId: string;
  path: string;
  label: LocalizedCopy;
}

export type CheckStatus = "Pass" | "Warning" | "Fail" | "NotApplicable" | "UnverifiedBasis" | "Info";

export type AnnexChoice = "En" | "De";

export type RemedyBound = "AtLeast" | "AtMost" | "Exactly" | "OneOf" | "Remove" | "Insert";

export interface Remedy {
  target: SubjectRef;
  current: Quantity;
  required: Quantity;
  bound: RemedyBound;
  options: string[];
  action: LocalizedCopy;
  applicable: boolean;
}

export interface CheckResult {
  id: string;
  part: string;
  clause: ClauseId;
  subject: SubjectRef;
  status: CheckStatus;
  title: LocalizedCopy;
  explanation: LocalizedCopy;
  computed: Quantity;
  limit: Quantity;
  utilization: number;
  annex: AnnexChoice;
  remedies: Remedy[];
  ndps: string[];
  stations: Station[];
  stationField: string;
}

export interface Station {
  at: number[];
  utilization: number;
  values: Quantity[];
}

export interface PartVerdict {
  part: string;
  pass: number;
  warning: number;
  fail: number;
  notApplicable: number;
  unverified: number;
  worstUtilization: number;
  complies: boolean;
  info: number;
}

export interface CheckReportSummary {
  total: number;
  pass: number;
  warning: number;
  fail: number;
  notApplicable: number;
  unverified: number;
  worstUtilization: number;
  complies: boolean;
  parts: PartVerdict[];
  info: number;
}

export type NdpStatus = "verified" | "unverified";

export type NdpValue = null | number | string;

export interface NdpSource {
  document: string;
  edition: string;
  locator: string;
}

export interface NdpAnnexVerification {
  status: NdpStatus;
  evidence: string;
}

export interface NdpAnnexStatus {
  en: NdpAnnexVerification;
  naDe: NdpAnnexVerification;
}

export interface NdpEntry {
  id: string;
  code: string;
  part: string;
  clause: string;
  symbol: string;
  unit: string;
  specRef: string;
  asWritten: string;
  recommended: NdpValue;
  naDe: NdpValue;
  source: NdpSource;
  status: NdpStatus;
  evidence: string;
  annexStatus?: NdpAnnexStatus;
}

export interface NdpRegistry {
  entries: NdpEntry[];
}

export const parseCheckReport: NormWireReader<CheckReport> = normWireObject<CheckReport>({ summary: normWireRequired(normWireRef(() => parseCheckReportSummary)), checks: normWireRequired(normWireArray(normWireRef(() => parseCheckResult))) });
export const parseQuantityKind: NormWireReader<QuantityKind> = normWireLiteral("Dimensionless", "Length", "Area", "Volume", "Mass", "Time", "Temperature", "Force", "Pressure", "Stress", "Moment", "Energy", "Power", "ThermalConductivity", "ThermalResistance", "HeatTransferCoefficient", "AirPermeability", "VentilationRate", "Acceleration", "Velocity");
export const parseQuantity: NormWireReader<Quantity> = normWireObject<Quantity>({ kind: normWireRequired(normWireRef(() => parseQuantityKind)), value: normWireRequired(normWireNumber) });
export const parseClauseId: NormWireReader<ClauseId> = normWireObject<ClauseId>({ family: normWireRequired(normWireString), part: normWireRequired(normWireString), section: normWireRequired(normWireString) });
export const parseLocalizedCopy: NormWireReader<LocalizedCopy> = normWireObject<LocalizedCopy>({ en: normWireRequired(normWireString), de: normWireRequired(normWireString) });
export const parseSubjectRef: NormWireReader<SubjectRef> = normWireObject<SubjectRef>({ entityId: normWireRequired(normWireString), path: normWireRequired(normWireString), label: normWireRequired(normWireRef(() => parseLocalizedCopy)) });
export const parseCheckStatus: NormWireReader<CheckStatus> = normWireLiteral("Pass", "Warning", "Fail", "NotApplicable", "UnverifiedBasis", "Info");
export const parseAnnexChoice: NormWireReader<AnnexChoice> = normWireLiteral("En", "De");
export const parseRemedyBound: NormWireReader<RemedyBound> = normWireLiteral("AtLeast", "AtMost", "Exactly", "OneOf", "Remove", "Insert");
export const parseRemedy: NormWireReader<Remedy> = normWireObject<Remedy>({ target: normWireRequired(normWireRef(() => parseSubjectRef)), current: normWireRequired(normWireRef(() => parseQuantity)), required: normWireRequired(normWireRef(() => parseQuantity)), bound: normWireRequired(normWireRef(() => parseRemedyBound)), options: normWireRequired(normWireArray(normWireString)), action: normWireRequired(normWireRef(() => parseLocalizedCopy)), applicable: normWireRequired(normWireBoolean) });
export const parseCheckResult: NormWireReader<CheckResult> = normWireObject<CheckResult>({ id: normWireRequired(normWireString), part: normWireRequired(normWireString), clause: normWireRequired(normWireRef(() => parseClauseId)), subject: normWireRequired(normWireRef(() => parseSubjectRef)), status: normWireRequired(normWireRef(() => parseCheckStatus)), title: normWireRequired(normWireRef(() => parseLocalizedCopy)), explanation: normWireRequired(normWireRef(() => parseLocalizedCopy)), computed: normWireRequired(normWireRef(() => parseQuantity)), limit: normWireRequired(normWireRef(() => parseQuantity)), utilization: normWireRequired(normWireNumber), annex: normWireRequired(normWireRef(() => parseAnnexChoice)), remedies: normWireRequired(normWireArray(normWireRef(() => parseRemedy))), ndps: normWireRequired(normWireArray(normWireString)), stations: normWireRequired(normWireArray(normWireRef(() => parseStation))), stationField: normWireRequired(normWireString) });
export const parseStation: NormWireReader<Station> = normWireObject<Station>({ at: normWireRequired(normWireArray(normWireNumber, 1, 3)), utilization: normWireRequired(normWireRange(normWireNumber, {"minimum":0})), values: normWireRequired(normWireArray(normWireRef(() => parseQuantity))) });
export const parsePartVerdict: NormWireReader<PartVerdict> = normWireObject<PartVerdict>({ part: normWireRequired(normWireString), pass: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), warning: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), fail: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), notApplicable: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), unverified: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), worstUtilization: normWireRequired(normWireNumber), complies: normWireRequired(normWireBoolean), info: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseCheckReportSummary: NormWireReader<CheckReportSummary> = normWireObject<CheckReportSummary>({ total: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), pass: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), warning: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), fail: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), notApplicable: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), unverified: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), worstUtilization: normWireRequired(normWireNumber), complies: normWireRequired(normWireBoolean), parts: normWireRequired(normWireArray(normWireRef(() => parsePartVerdict))), info: normWireRequired(normWireRange(normWireInteger, {"minimum":0})) });
export const parseNdpStatus: NormWireReader<NdpStatus> = normWireLiteral("verified", "unverified");
export const parseNdpValue: NormWireReader<NdpValue> = normWireAny<[null, number, string]>(normWireLiteral(null), normWireNumber, normWireString);
export const parseNdpSource: NormWireReader<NdpSource> = normWireObject<NdpSource>({ document: normWireRequired(normWireString), edition: normWireRequired(normWireString), locator: normWireRequired(normWireString) });
export const parseNdpAnnexVerification: NormWireReader<NdpAnnexVerification> = normWireObject<NdpAnnexVerification>({ status: normWireRequired(normWireRef(() => parseNdpStatus)), evidence: normWireRequired(normWireString) });
export const parseNdpAnnexStatus: NormWireReader<NdpAnnexStatus> = normWireObject<NdpAnnexStatus>({ en: normWireRequired(normWireRef(() => parseNdpAnnexVerification)), naDe: normWireRequired(normWireRef(() => parseNdpAnnexVerification)) });
export const parseNdpEntry: NormWireReader<NdpEntry> = normWireObject<NdpEntry>({ id: normWireRequired(normWireString), code: normWireRequired(normWireString), part: normWireRequired(normWireString), clause: normWireRequired(normWireString), symbol: normWireRequired(normWireString), unit: normWireRequired(normWireString), specRef: normWireRequired(normWireString), asWritten: normWireRequired(normWireString), recommended: normWireRequired(normWireRef(() => parseNdpValue)), naDe: normWireRequired(normWireRef(() => parseNdpValue)), source: normWireRequired(normWireRef(() => parseNdpSource)), status: normWireRequired(normWireRef(() => parseNdpStatus)), evidence: normWireRequired(normWireString), annexStatus: normWireOptional(normWireRef(() => parseNdpAnnexStatus)) });
export const parseNdpRegistry: NormWireReader<NdpRegistry> = normWireObject<NdpRegistry>({ entries: normWireRequired(normWireArray(normWireRef(() => parseNdpEntry))) });
