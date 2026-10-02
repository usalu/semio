/** ⚖️ `CheckReport` wire twin: the compliance report every norm artifact's checks produce, exactly as `./🦀️.rs` writes it. Generated from `./🔣️.json`
 * by `🧪️s2-norm-ts-twins.ts`; readers judge structure, the schema's bounds stay Ajv's.
 * @see ./🔣️.json */
import { normWireArray, normWireBoolean, normWireInteger, normWireLiteral, normWireNumber, normWireObject, normWireRange, type NormWireReader, normWireRef, normWireRequired, normWireString } from "../../📇️registry/🧬️contract/🟦️.ts";

export interface CheckReport {
  summary: CheckReportSummary;
  checks: CheckResult[];
}

export type QuantityKind = "Dimensionless" | "Length" | "Area" | "Volume" | "Mass" | "Time" | "Temperature" | "Force" | "Pressure" | "Stress" | "Moment" | "Energy" | "Power" | "ThermalConductivity" | "ThermalResistance" | "HeatTransferCoefficient" | "AirPermeability" | "VentilationRate" | "Acceleration";

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

export type CheckStatus = "Pass" | "Warning" | "Fail" | "NotApplicable";

export type AnnexChoice = "En" | "De";

export type RemedyBound = "AtLeast" | "AtMost" | "Exactly" | "OneOf";

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
}

export interface PartVerdict {
  part: string;
  pass: number;
  warning: number;
  fail: number;
  notApplicable: number;
  worstUtilization: number;
  complies: boolean;
}

export interface CheckReportSummary {
  total: number;
  pass: number;
  warning: number;
  fail: number;
  notApplicable: number;
  worstUtilization: number;
  complies: boolean;
  parts: PartVerdict[];
}

export const parseCheckReport: NormWireReader<CheckReport> = normWireObject<CheckReport>({ summary: normWireRequired(normWireRef(() => parseCheckReportSummary)), checks: normWireRequired(normWireArray(normWireRef(() => parseCheckResult))) });
export const parseQuantityKind: NormWireReader<QuantityKind> = normWireLiteral("Dimensionless", "Length", "Area", "Volume", "Mass", "Time", "Temperature", "Force", "Pressure", "Stress", "Moment", "Energy", "Power", "ThermalConductivity", "ThermalResistance", "HeatTransferCoefficient", "AirPermeability", "VentilationRate", "Acceleration");
export const parseQuantity: NormWireReader<Quantity> = normWireObject<Quantity>({ kind: normWireRequired(normWireRef(() => parseQuantityKind)), value: normWireRequired(normWireNumber) });
export const parseClauseId: NormWireReader<ClauseId> = normWireObject<ClauseId>({ family: normWireRequired(normWireString), part: normWireRequired(normWireString), section: normWireRequired(normWireString) });
export const parseLocalizedCopy: NormWireReader<LocalizedCopy> = normWireObject<LocalizedCopy>({ en: normWireRequired(normWireString), de: normWireRequired(normWireString) });
export const parseSubjectRef: NormWireReader<SubjectRef> = normWireObject<SubjectRef>({ entityId: normWireRequired(normWireString), path: normWireRequired(normWireString), label: normWireRequired(normWireRef(() => parseLocalizedCopy)) });
export const parseCheckStatus: NormWireReader<CheckStatus> = normWireLiteral("Pass", "Warning", "Fail", "NotApplicable");
export const parseAnnexChoice: NormWireReader<AnnexChoice> = normWireLiteral("En", "De");
export const parseRemedyBound: NormWireReader<RemedyBound> = normWireLiteral("AtLeast", "AtMost", "Exactly", "OneOf");
export const parseRemedy: NormWireReader<Remedy> = normWireObject<Remedy>({ target: normWireRequired(normWireRef(() => parseSubjectRef)), current: normWireRequired(normWireRef(() => parseQuantity)), required: normWireRequired(normWireRef(() => parseQuantity)), bound: normWireRequired(normWireRef(() => parseRemedyBound)), options: normWireRequired(normWireArray(normWireString)), action: normWireRequired(normWireRef(() => parseLocalizedCopy)), applicable: normWireRequired(normWireBoolean) });
export const parseCheckResult: NormWireReader<CheckResult> = normWireObject<CheckResult>({ id: normWireRequired(normWireString), part: normWireRequired(normWireString), clause: normWireRequired(normWireRef(() => parseClauseId)), subject: normWireRequired(normWireRef(() => parseSubjectRef)), status: normWireRequired(normWireRef(() => parseCheckStatus)), title: normWireRequired(normWireRef(() => parseLocalizedCopy)), explanation: normWireRequired(normWireRef(() => parseLocalizedCopy)), computed: normWireRequired(normWireRef(() => parseQuantity)), limit: normWireRequired(normWireRef(() => parseQuantity)), utilization: normWireRequired(normWireNumber), annex: normWireRequired(normWireRef(() => parseAnnexChoice)), remedies: normWireRequired(normWireArray(normWireRef(() => parseRemedy))) });
export const parsePartVerdict: NormWireReader<PartVerdict> = normWireObject<PartVerdict>({ part: normWireRequired(normWireString), pass: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), warning: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), fail: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), notApplicable: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), worstUtilization: normWireRequired(normWireNumber), complies: normWireRequired(normWireBoolean) });
export const parseCheckReportSummary: NormWireReader<CheckReportSummary> = normWireObject<CheckReportSummary>({ total: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), pass: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), warning: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), fail: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), notApplicable: normWireRequired(normWireRange(normWireInteger, {"minimum":0})), worstUtilization: normWireRequired(normWireNumber), complies: normWireRequired(normWireBoolean), parts: normWireRequired(normWireArray(normWireRef(() => parsePartVerdict))) });
