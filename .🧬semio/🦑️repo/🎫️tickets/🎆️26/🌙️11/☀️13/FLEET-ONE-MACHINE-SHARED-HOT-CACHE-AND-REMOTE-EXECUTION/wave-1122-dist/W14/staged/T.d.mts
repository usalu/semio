export type TakeOutcomeV1 = "present" | "fetched" | "stale" | "miss" | "fault";
export type TakePolicyV1 = Readonly<{ concurrency: number; lookFloorConcurrency: number; fetchConcurrency: number; claimStaleMilliseconds: number; claimPollMilliseconds: number; pollFirstMilliseconds: number; pollGrowthPercent: number; pollMaximumMilliseconds: number; pollBatch: number; faultRetries: number; faultWaitMilliseconds: number; progressMilliseconds: number; passBoundMilliseconds: number; releaseMilliseconds: number; noProgressMilliseconds: number; tailReportMilliseconds: number; watchMilliseconds: number; strongMemoryMegabytes: number; strongCores: number; cpuProgressPercent: number; hotRetryMilliseconds: number; adoptConcurrency: number; hotNamesMilliseconds: number; adoptReportMilliseconds: number; adoptStaleMilliseconds: number; adoptRetryMilliseconds: number; prefetchRecordsPerAsk: number; prefetchAsksInFlight: number; lookMaximumMilliseconds: number; abandonedLooksMaximum: number }>;
export type TakeCountsV1 = Readonly<{ total: number; present: number; fetched: number; here: number; stale: number; running: number; wanted: number; done: number; missing: number }>;
/** ⚙️ The authored numbers of the take pass. */
export const TAKE_POLICY_V1: TakePolicyV1;
/** 🪢️ Waits for another lane without taking custody of its look. */
export function joinTakeLookV1<Outcome>(running: Promise<Outcome>, signal: AbortSignal): Promise<Outcome>;
/** ⏱️ The pause before look number `round` at the cache. */
export function takePollWaitV1(round: number, policy?: TakePolicyV1): number;
/** 📒️ What a reader knows about its owners while it resolves them. */
export function createTakeLedgerV1(total: number): Readonly<{ mark(name: string, outcome: TakeOutcomeV1 | "inferred"): void; want(names: Iterable<string>): void; run(delta: number): void; state(name: string): string | undefined; counts(): TakeCountsV1; line(language: string): string; waveLine(language: string): string; staleLine(language: string, name: string): string; staleRowLine(language: string, name: string, row: Readonly<{ kind: string; path: string; fact: string; recorded?: string; current?: string; why?: string }>): string }>;
export function staleRowEventV1(name: string, source: "take" | "infer", row: Readonly<{ kind: string; path: string; fact?: string; recorded?: string; current?: string; why?: string }>, ledger: Readonly<{ staleRowLine(language: string, name: string, row: never): string }>): Readonly<Record<string, unknown>>;
/** 💪️ Whether a machine is large enough never to wait for another. */
export function takeStrongMachineV1(input: Readonly<{ memoryMegabytes: number; cores: number }>, policy?: TakePolicyV1): boolean;
/** 🚦️ A watch over the progress of a read. */
export function createProgressWatchV1(input?: Readonly<{ now?(): number; windowMilliseconds?: number }>): Readonly<{ check(sign: unknown): Readonly<{ sinceMilliseconds: number }> | undefined }>;
export function takeCpuMillisecondsV1(text: string): number;
export function takeTreeCpuMillisecondsV1(rows: readonly Readonly<{ pid: number; parent: number; milliseconds: number }>[], root: number): number;
export function takeChildrenCpuV1(root?: number, run?: (...input: any[]) => unknown): Promise<number | undefined>;
export function createCpuProgressV1(input: Readonly<{ sample(): Promise<number | undefined> | number | undefined; now?(): number; policy?: Readonly<{ cpuProgressPercent: number }> }>): Readonly<{ tick(): Promise<number> }>;
/** 🗣️ The plain text of a read that stopped for lack of progress. */
export function takeNoProgressTextV1(language: string, fields: Readonly<Record<string, string | number>>): string;
/** 🔢 The looks a lane runs at once under the cut of the process. */
/** 🔒️ The claims of the owners that loads of one checkout infer at the same time. */
export function createOwnerClaimsV1(input: Readonly<{ directory: string; pid?: number; now?(): number; alive?(pid: number): boolean; staleMilliseconds?: number }>): Readonly<{ claim(stem: string): "granted" | "running"; release(stem: string): void }>;
/** 🔒️ Holds the claim of an owner or takes the record the claimant stores. */
export function takeClaimedV1(input: Readonly<{ name: string; stem: string; claims: { claim(stem: string): "granted" | "running"; release(stem: string): void }; look(name: string): Promise<string | undefined>; signal?: AbortSignal; sleep?: (milliseconds: number, signal?: AbortSignal) => Promise<unknown>; pollMilliseconds?: number }>): Promise<Readonly<{ outcome?: string; release?(): void }>>;
/** 🔑️ The key of the globals a pool wave published its records under. */
export function fleetGlobalsKeyV1(globals: unknown): string;
export function takeConcurrencyV1(cutConcurrency?: number, policy?: TakePolicyV1): number;
/** 🔢 The looks a lane runs at once: the cut of the process, never fewer than the declared floor. */
export function takeLookConcurrencyV1(cutConcurrency?: number, policy?: TakePolicyV1): number;
export function takeFetchConcurrencyV1(cutConcurrency?: number, policy?: TakePolicyV1): number;
/** 🚦 A look admitted by the footprint gate of the process. */
export function takeAdmittedV1<Name, Outcome>(attempt: (name: Name, signal?: AbortSignal) => Promise<Outcome>, gate: () => Readonly<{ admit(signal?: AbortSignal): Promise<unknown> }> | undefined, signal?: AbortSignal, state?: { flying: number }): (name: Name, signal?: AbortSignal) => Promise<Outcome>;
/** 🔍️ Looks at every name once, and again those the cache did not answer. */
export function takeLanesV1(names: readonly string[], isPresent: (name: string) => boolean, isPublished?: (name: string) => boolean): string[][];
/** 🐘️ The names of a pass in the light group and the heavy group (looked at one at a time, after the light ones). */
export function takeSplitV1(names: readonly string[], isHeavy: (name: string) => boolean): Readonly<{ light: string[]; heavy: string[] }>;
/** ⏳️ The outcome of one look, or "miss" once it has not answered within the bound while fewer than `maximum` set-aside looks run. */
export function takeBoundedLookV1(look: Promise<TakeOutcomeV1>, milliseconds: number, state?: { count: number }, maximum?: number): Promise<TakeOutcomeV1>;
export function takePassV1(input: Readonly<{ names: readonly string[]; attempt(name: string, signal?: AbortSignal): Promise<TakeOutcomeV1>; concurrency?: number; faultRetries?: number; faultWaitMilliseconds?: number; lookMaximumMilliseconds?: number; abandoned?: { count: number }; abandonedMaximum?: number; sleep?(milliseconds: number): Promise<void>; signal?: AbortSignal; mark?(name: string, outcome: TakeOutcomeV1): void }>): Promise<ReadonlyMap<string, TakeOutcomeV1>>;
/** 🔁️ Looks again at the cache for the owners that are still missing, with a growing pause. */
/** 📣️ The names the pools of the fleet say they stored. */
export function fleetPublishedNamesV1(store: string | undefined, now?: number, policy?: Readonly<{ publishedMaximumAgeMilliseconds: number; publishedScanConcurrency: number }>, globals?: unknown): Promise<Set<string>>;
export function takePollV1(input: Readonly<{ hot?(): ReadonlySet<string> | undefined; hotOnly?: boolean; now?(): number; concurrency?: number; queue(): readonly string[]; attempt(name: string, signal?: AbortSignal): Promise<TakeOutcomeV1>; arrived(name: string, outcome: TakeOutcomeV1): void | Promise<void>; stop(): boolean; signal?: AbortSignal; sleep?(milliseconds: number, signal?: AbortSignal): Promise<void>; policy?: TakePolicyV1 }>): Promise<Readonly<{ looks: number; found: number }>>;
/** 🚀️ Runs the take pass and the inference of the owners beside each other: the inference is released when the pass ends or after `releaseMilliseconds`, never later, and the warm pass is stopped then. */
export function runTakeBesideInferV1(input: Readonly<{ take(): Promise<unknown>; warm?(): Promise<unknown>; stopWarm?(): void; release(): void; infer(): Promise<unknown>; end?(): void; releaseMilliseconds?: number; sleep?(milliseconds: number, signal?: AbortSignal): Promise<void> }>): Promise<void>;

export const TAKE_PROGRESS_KEYS_V1: readonly string[];
/** 🧮️ The sign of the progress watch, made of the declared progress facts only. */
export function takeProgressSignV1(row: Readonly<Record<string, unknown>>): string;
/** 🪧️ A reporter that speaks at most once per window of no change. */
export function createTailReporterV1(input?: Readonly<{ now?(): number; windowMilliseconds?: number }>): Readonly<{ check(sign: unknown): Readonly<{ sinceMilliseconds: number }> | undefined }>;
export type TakeMissingReasonV1 = "heavy" | "running" | "parked" | "fleet";
export type TakeMissingRowV1 = Readonly<{ name: string; reason: TakeMissingReasonV1; megabytes?: number; limitMegabytes?: number; seconds?: number }>;
/** 📋️ The reason of every missing owner. */
export function takeMissingRowsV1(names: readonly string[], input: Readonly<{ never: ReadonlySet<string>; running: ReadonlyMap<string, number>; deferred: ReadonlySet<string>; megabytes?(name: string): number | undefined; limitMegabytes?: number }>): readonly TakeMissingRowV1[];
/** ⚖️ Whether a stall is the typed heavy refusal or a generic stall. */
export function takeStallKindV1(rows: readonly TakeMissingRowV1[]): "heavy" | "stalled";
/** 🗣️ The plain text of the tail of a read, in one language. */
export function takeTailTextV1(language: string, input: Readonly<{ minutes: number; done: number; total: number; rows: readonly TakeMissingRowV1[]; pool?: Readonly<{ state: string; children: number; running: number; ownersPerMinute: number; text: Readonly<{ en: string; de: string }> | null }> }>, limit?: number): string;
/** 🎣️ The take pass beside an adopt loop that takes what the fleet lists as stored while the lanes still run. */
export function takeFetchFirstV1(input: Readonly<{ lanes: readonly (readonly string[])[]; heavy?: readonly string[]; concurrency?: number; fetchConcurrency?: number; fetches?(lane: readonly string[]): boolean; attempt(name: string, signal?: AbortSignal): Promise<TakeOutcomeV1>; adoptAttempt?(name: string, signal?: AbortSignal): Promise<TakeOutcomeV1>; signal?: AbortSignal; mark?(name: string, outcome: TakeOutcomeV1): void; hot?(): ReadonlySet<string> | undefined; queue(): readonly string[]; arrived(name: string, outcome: TakeOutcomeV1): void | Promise<void>; stop?(): boolean; now?(): number; sleep?(milliseconds: number, signal?: AbortSignal): Promise<void>; policy?: TakePolicyV1 }>): Promise<Readonly<{ looks: number; found: number }>>;

/** 📊️ The counters of the adopt and poll path of a read. */
export type AdoptCountsV1 = Readonly<{ listed: number; looks: number; adoptLooks: number; inferLooks: number; passLooks: number; fetched: number; present: number; adopted: number; stale: number; staleKinds: Readonly<Record<string, number>>; incomplete: number; miss: number; fault: number; skippedInferring: number; gateWaits: number; gateMilliseconds: number; gateSeconds: number; inferredListed: number }>;
export function createAdoptCountersV1(): Readonly<{ list(names: Iterable<string>): void; look(lane: "adopt" | "infer" | "pass" | "poll"): void; outcome(outcome: string): void; staleRow(kind: string | undefined): void; skipInferring(name: string): void; gateWait(milliseconds: number): void; inferredListed(): void; counts(): AdoptCountsV1; line(language: string): string }>;
/** 🔥️ The names the fleet lists as stored, scanned in the background at most once per interval after the last scan ended. */
export function createHotNamesV1(input: Readonly<{ read(): Promise<ReadonlySet<string>>; counters?: Readonly<{ list(names: Iterable<string>): void }>; now?(): number; policy?: TakePolicyV1 }>): (() => ReadonlySet<string>) & Readonly<{ settled(): Promise<void> }>;
/** 🥇️ The decision before a local inference: a listed owner is looked at first and adopted when its look serves the record. */
export function takeBeforeInferV1(input: Readonly<{ name: string; listed(name: string): boolean; taken: ReadonlySet<string>; look(name: string): Promise<string>; counters?: Readonly<{ inferredListed(): void }> }>): Promise<"present" | "fetched" | undefined>;
