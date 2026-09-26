/** 🧬 EN 1993 snapshot TypeScript interfaces (SI). */
export interface DesignAction { n: number; vy: number; vz: number; my: number; mz: number; t: number; }
export interface LoadCase { id: string; name: string; kind: string; category: string; }
export interface MemberAction { id: string; memberId: string; loadCaseId: string; action: DesignAction; }
export interface ForceAction { id: string; loadCaseId: string; force: number; }
export interface JointForceAction { id: string; loadCaseId: string; shear: number; tension: number; }
export interface FatigueBand { id: string; deltaSigma: number; cycles: number; }
export interface SteelMaterial { id: string; grade: string; fy: number; fu: number; eModulus: number; gModulus: number; subgrade: string; kind: string; }
export interface SteelSection { id: string; designation: string; kind: string; h: number; b: number; tw: number; tf: number; r: number; area: number; shearAreaY: number; shearAreaZ: number; iy: number; iz: number; it: number; iw: number; wElY: number; wElZ: number; wPlY: number; wPlZ: number; areaNet: number; }
export interface SteelMember { id: string; label: string; memberType: string; sectionId: string; materialId: string; length: number; bucklingLengthY: number; bucklingLengthZ: number; ltbLength: number; ltbRestraintSpacing: number; loadApplication: string; endMomentRatioPsi: number; momentDiagram: string; deflectionLimitRatio: number; analysis: string; }
export interface SteelJoint { id: string; kind: string; memberId: string; boltClass: string; boltDiameter: number; boltRows: number; boltsPerRow: number; pitch: number; gauge: number; endDistance: number; edgeDistance: number; shearPlanes: number; plateThickness: number; plateFu: number; weldThroat: number; weldLength: number; weldFu: number; weldGrade: string; actions: JointForceAction[]; category: string; frictionMu: number; preloadForce: number; slipFactorKs: number; frictionSurfaces: number; }
export interface FatigueDetail { id: string; memberId: string; category: number; method: string; spectrum: FatigueBand[]; }
export interface FireExposure { id: string; memberId: string; rating: string; protectionThickness: number; sectionFactor: number; mu0: number; protectionConductivity: number; protectionDensity: number; protectionSpecificHeat: number; }
export interface ColdFormedMember { id: string; bBar: number; thickness: number; kSigma: number; psi: number; fy: number; grossResistance: number; actions: ForceAction[]; }
export interface PlatedPanel { id: string; a: number; b: number; thickness: number; fy: number; kSigma: number; actions: ForceAction[]; }
export interface SiloShell { id: string; thickness: number; radius: number; depth: number; k: number; gamma: number; fy: number; }
export interface TensionComponent { id: string; fUk: number; fK: number; actions: ForceAction[]; }
export interface BridgeFatigue { id: string; memberId: string; lambda: number; phi2: number; deltaSigmaP: number; category: number; method: string; }
export interface TowerLeg { id: string; memberId: string; forceCoefficient: number; dynamicFactor: number; actions: ForceAction[]; }
export interface SteelPile { id: string; sectionId: string; materialId: string; drivingStress: number; embeddedLength: number; shaftPerimeter: number; actions: ForceAction[]; }
export interface CraneRunway { id: string; memberId: string; wheelContactLength: number; dispersion: number; webThickness: number; fy: number; phi: number; actions: ForceAction[]; }
export interface En1993Snapshot { annex: string; materials: SteelMaterial[]; sections: SteelSection[]; members: SteelMember[]; loadCases: LoadCase[]; memberActions: MemberAction[]; joints: SteelJoint[]; fatigueDetails: FatigueDetail[]; fireExposures: FireExposure[]; coldFormedMembers: ColdFormedMember[]; platedPanels: PlatedPanel[]; siloShells: SiloShell[]; tensionComponents: TensionComponent[]; bridgeFatigue: BridgeFatigue[]; towerLegs: TowerLeg[]; piles: SteelPile[]; craneRunways: CraneRunway[]; }
