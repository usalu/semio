/** 🔭️ The switch of plan reuse. */
export const WORLD_WITNESS_SWITCH_V1: "SEMIO_REPO_PLAN_REUSE";
/** 🎛️ One reading of the switch: reuse a proven plan, never reuse, or plan completely and report when the reused plan would have differed. */
export type WorldWitnessSwitchV1 = "on" | "off" | "verify";
/** 🧾️ One fact of the world: its kind letter, its subject and the value it had. */
export type WorldObservationV1 = readonly [kind: string, subject: string, value: string];
/** 🔬️ Everything one computation looked at, or the reasons why that cannot be said completely. */
export interface WorldWitnessV1 { readonly version: 1; readonly observations: readonly WorldObservationV1[]; readonly racy: string | null; readonly unstable: readonly string[]; readonly tainted: readonly string[] }
/** 🕵️ The part of the witness a computation uses from inside the observed process. */
export interface WorldWitnessRecorderV1 { readonly version: 1; allow<T>(label: string, body: () => T): T; observe(kind: string, subject: string, value: string): void; taint(reason: string): void; finish(): void }
/** ✅️ The verdict of checking a witness against the present world. */
export type WorldWitnessVerdictV1 = Readonly<{ holds: true; checked: number }> | Readonly<{ holds: false; checked: number; kind: string; subject: string }>;
/** 🔌️ The checks only the caller can make: the git index listing, git's answer about its ignore sources and the output of a command. */
export interface WorldWitnessPortsV1 { git?(subject: string, expected: string): Promise<boolean>; ignore?(subject: string, expected: string): Promise<boolean>; runtime?(subject: string, expected: string): Promise<boolean> }
/** 🛠️ How a witness is checked: cancellation, parallelism, progress, the caller's checks and the environment. */
export interface WorldWitnessVerifyOptionsV1 { readonly signal?: AbortSignal; readonly concurrency?: number; readonly progress?: (checked: number) => Promise<void> | void; readonly ports?: WorldWitnessPortsV1; readonly environment?: Readonly<Record<string, string | undefined>> }
/** 🎚️ Reads the plan reuse switch; an unknown value is a refusal, never a silent default. */
export function readWorldWitnessSwitchV1(environment?: Readonly<Record<string, string | undefined>>): WorldWitnessSwitchV1;
/** 💬️ Renders a message or a reason in one declared language; there is no hidden default language. */
export function renderWorldWitnessTextV1(table: "Messages" | "Reasons", language: "en" | "de", name: string, values?: Readonly<Record<string, string | number>>): string;
/** 📸️ Records, inside the process that runs it, every fact of the world one computation read and names what it cannot record as tainted. */
export function installWorldWitnessV1(configText: string): WorldWitnessRecorderV1;
/** 📜️ The source text of the preload that installs the witness in a child process. */
export function worldWitnessPreloadV1(): string;
/** 🌍️ The environment a child needs to run under the witness; the output file receives the finished witness. */
export function worldWitnessEnvironmentV1(options: Readonly<{ out: string; root: string; workspace: string; writable: readonly string[]; racyMilliseconds?: number; hostProbes?: readonly Readonly<{ program: string; arguments: readonly string[] }>[]; hostProbeFiles?: readonly string[] }>): Record<string, string>;
/** 👁️ The witness of the process that runs here, when it was started under the preload; otherwise nothing. */
export function worldWitnessV1(): WorldWitnessRecorderV1 | undefined;
/** 🌫️ Tells whether a name is an ambient host, agent or session variable that never is an input of a plan. */
export function isAmbientEnvironmentNameV1(name: string): boolean;
/** 📖️ Admits a witness file the planner child wrote; anything malformed is refused. */
export function readWorldWitnessV1(text: string): WorldWitnessV1;
/** ⚖️ Tells whether every observation of a witness still holds, stopping at the first that differs. */
export function verifyWorldWitnessV1(witness: WorldWitnessV1, options: WorldWitnessVerifyOptionsV1): Promise<WorldWitnessVerdictV1>;
/** 🧮️ Digests the tracked-file listing and the ignore files a git universe depends on. */
export function worldWitnessGitDigestV1(listing: string | Uint8Array): string;
/** 🗂️ The names the observations of a witness group under, with their counts. */
export function summarizeWorldWitnessV1(witness: WorldWitnessV1): Readonly<Record<string, number>>;
/** 🔒️ Resolves a path the way the witness records it. */
export function worldWitnessPathV1(value: string): string;
