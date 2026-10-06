/** 🧹️ Explicit contributed conformance eligibility with caller-owned continuation controls. */
export type ConformanceInputStatus = "ready" | "missing" | "empty" | "stub" | "invalid";
export interface ConformanceInput {
  readonly id: string;
  readonly ownerPresent: boolean;
  readonly facet: "grammar" | "protocol";
  readonly specimen: "text" | "binary";
  readonly specificationStatus: ConformanceInputStatus;
  readonly specimenStatus: ConformanceInputStatus;
}
export type ConformanceOutcome = "ready" | "owner-absent" | "kind-mismatch" | `specification-${Exclude<ConformanceInputStatus,"ready">}` | `specimen-${Exclude<ConformanceInputStatus,"ready">}`;
export interface ConformanceAdmission {readonly input: ConformanceInput;readonly outcome: ConformanceOutcome;}
export interface ConformanceOperationControl {
  readonly signal: AbortSignal;
  readonly onProgress: (completed: number,total: number)=>void;
  readonly yieldContinuation: ()=>Promise<void>;
}
export class ConformanceControlRefusal extends Error {
  constructor(readonly code: "cancelled" | "budget",readonly completed: number){super(code);}
}
export function admitConformanceInput(input: ConformanceInput): ConformanceAdmission {
  const outcome: ConformanceOutcome = !input.ownerPresent ? "owner-absent" : (input.facet === "grammar") !== (input.specimen === "text") ? "kind-mismatch" : input.specificationStatus !== "ready" ? `specification-${input.specificationStatus}` : input.specimenStatus !== "ready" ? `specimen-${input.specimenStatus}` : "ready";
  return {input,outcome};
}
export class ConformanceCursor {
  #completed = 0;
  constructor(readonly inputs: readonly ConformanceInput[]){}
  get completed(): number {return this.#completed;}
  get complete(): boolean {return this.#completed === this.inputs.length;}
  async advance(maximumUnits: number,control: ConformanceOperationControl,receive: (admission: ConformanceAdmission)=>void): Promise<boolean> {
    if(!Number.isSafeInteger(maximumUnits) || maximumUnits <= 0)throw new ConformanceControlRefusal("budget",this.#completed);
    for(let units = 0;units < maximumUnits && !this.complete;units++){
      if(control.signal.aborted)throw new ConformanceControlRefusal("cancelled",this.#completed);
      control.onProgress(this.#completed,this.inputs.length);
      await control.yieldContinuation();
      if(control.signal.aborted)throw new ConformanceControlRefusal("cancelled",this.#completed);
      receive(admitConformanceInput(this.inputs[this.#completed]!));
      this.#completed++;
    }
    return this.complete;
  }
}
