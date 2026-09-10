/** 🎯️ The shape of one example's committed expected-outcome vector (`🔣️.json` beside each example). */
export type AssemblyExampleOutcome = {
  readonly schema: "s.procedural.assembly.example-outcome/v1";
  readonly example: string;
  readonly seed: number;
  readonly slots: number;
  readonly edges: number;
  readonly modules: readonly string[];
  readonly rules: number;
  readonly satisfiable: boolean;
  readonly assignments: Readonly<Record<string, string>>;
};
