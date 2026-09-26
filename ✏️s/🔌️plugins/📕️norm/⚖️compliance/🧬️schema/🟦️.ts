/** ⚖️ Norm compliance CheckReport wire types */

export type QuantityKind =
  | "Dimensionless"
  | "Length"
  | "Area"
  | "Volume"
  | "Mass"
  | "Time"
  | "Temperature"
  | "Force"
  | "Pressure"
  | "Stress"
  | "Moment"
  | "Energy"
  | "Power"
  | "ThermalConductivity"
  | "ThermalResistance"
  | "HeatTransferCoefficient"
  | "AirPermeability"
  | "VentilationRate"
  | "Acceleration";

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

export interface CheckReport {
  summary: CheckReportSummary;
  checks: CheckResult[];
}
