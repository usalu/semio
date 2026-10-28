/** 🔭️ The switch of plan reuse. */
export const WORLD_WITNESS_SWITCH_V1: "SEMIO_REPO_PLAN_REUSE";
/** 🎛️ One reading of the switch: reuse a proven plan, never reuse, or plan completely and report when the reused plan would have differed. */
export type WorldWitnessSwitchV1 = "on" | "off" | "verify";
/** 🧾️ One fact of the world: its kind letter, its subject and the value it had. */
export type WorldObservationV1 = readonly [kind: string, subject: string, value: string];
/** 🔭️ Everything one computation looked at, or the reasons why that cannot be said completely. */
export interface WorldWitnessV1 { readonly version: 1; readonly observations: readonly WorldObservationV1[]; readonly racy: boolean; readonly unstable: readonly string[]; readonly tainted: readonly string[] }
/** 🕵️ The part of the witness a computation uses from inside the observed process. */
export interface WorldWitnessRecorderV1 { readonly version: 1; allow<T>(label: string, body: () => T): T; observe(kind: string, subject: string, value: string): void; taint(reason: string): void; finish(): void }
/** ✅️ The verdict of checking a witness against the present world. */
export type WorldWitnessVerdictV1 = Readonly<{ holds: true; checked: number }> | Readonly<{ holds: false; checked: number; kind: string; subject: string }>;
/** 🔌️ The checks only the caller can make: the git index listing and the output of a command. */
export interface WorldWitnessPortsV1 { git?(subject: string, expected: string): Promise<boolean>; runtime?(subject: string, expected: string): Promise<boolean> }
export interface WorldWitnessVerifyOptionsV1 { readonly signal?: AbortSignal; readonly concurrency?: number; readonly progress?: (checked: number) => Promise<void> | void; readonly ports?: WorldWitnessPortsV1; readonly environment?: Readonly<Record<string, string | undefined>> }
export function readWorldWitnessSwitchV1(environment?: Readonly<Record<string, string | undefined>>): WorldWitnessSwitchV1;
export function renderWorldWitnessTextV1(table: "Messages" | "Reasons", language: "en" | "de", name: string, values?: Readonly<Record<string, string | number>>): string;
export function installWorldWitnessV1(configText: string): WorldWitnessRecorderV1;
export function worldWitnessPreloadV1(): string;
export function worldWitnessEnvironmentV1(options: Readonly<{ out: string; root: string; workspace: string; writable: readonly string[]; racyMilliseconds?: number; hostProbes?: readonly Readonly<{ program: string; arguments: readonly string[] | null }>[]; hostProbeFiles?: readonly string[] }>): Record<string, string>;
export function worldWitnessV1(): WorldWitnessRecorderV1 | undefined;
export function readWorldWitnessV1(text: string): WorldWitnessV1;
export function verifyWorldWitnessV1(witness: WorldWitnessV1, options: WorldWitnessVerifyOptionsV1): Promise<WorldWitnessVerdictV1>;
export function worldWitnessGitDigestV1(listing: string | Uint8Array): string;
export function summarizeWorldWitnessV1(witness: WorldWitnessV1): Readonly<Record<string, number>>;
export function worldWitnessPathV1(value: string): string;
