/** 🪵 EN 1995 snapshot TypeScript mirror (members ⊃ characteristic actions, connections ⊃ connection actions). */
export type AnnexChoice = "en" | "de";
export type MemberRole = "beam" | "column" | "floor" | "bridge";
export type SupportType = "simplySupported" | "cantilever" | "continuousTwoSpan";
export interface CharacteristicAction { id: string; kind: string; category: string; loadDuration: string; qLineNPerM: number; fPointN: number; mKNm: number; vKN: number; nKN: number; nTKN: number; fC90KN: number; }
export interface ConnectionAction { id: string; kind: string; loadDuration: string; fKN: number; }
export interface TimberMember { id: string; labelEn: string; labelDe: string; role: MemberRole; strengthClass: string; serviceClass: number; support: SupportType; bM: number; hM: number; spanM: number; supportLengthM: number; bearingLengthM: number; bucklingLengthYM: number; bucklingLengthZM: number; lateralRestraintSpacingM: number; notchDepthM: number; notchDistanceM: number; mCritNm: number; massKgPerM: number; massKgPerM2: number; dampingXi: number; fireDurationS: number; bridgeNObs: number; bridgeTLYears: number; bridgeBeta: number; bridgeA: number; bridgeB: number; bridgeCrowdPerM2: number; actions: CharacteristicAction[]; }
export interface TimberConnection { id: string; labelEn: string; labelDe: string; fastenerType: string; strengthClass: string; serviceClass: number; diameterM: number; number: number; rows: number; spacingM: number; edgeDistanceM: number; endDistanceM: number; t1M: number; t2M: number; steelPlate: boolean; steelPlateThicknessM: number; shearPlanes: number; fUK: number; actions: ConnectionAction[]; }
export interface En1995Snapshot { annex: AnnexChoice; members: TimberMember[]; connections: TimberConnection[]; }
