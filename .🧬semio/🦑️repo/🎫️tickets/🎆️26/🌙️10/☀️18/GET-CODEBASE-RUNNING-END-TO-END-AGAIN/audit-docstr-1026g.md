# Docstring, comment, language and policy audit of wave 1026g

Auditor: `aud-docstr-g` (read-only). Scope: new production modules added by waves 1025f and 1026g. Repo root: `/Users/ueli/Documents/semio`. Ticket: `C = .🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️10/☀️18/GET-CODEBASE-RUNNING-END-TO-END-AGAIN`.

## Scope and method

- Candidate set: staged-added production files under `🧰️framework` (`git diff --cached --diff-filter=A`, 377 framework adds, 122 non-test). `git ls-files --others --exclude-standard -- 🧰️framework` returned no untracked file. Tests (`🧪️tests`, `*.test.*`, `*_test.*`, `*.spec.*`) were excluded.
- Attribution to waves: a file counts as wave 1025f/1026g when a `result-*-f.md` or `result-*-g.md` mentioning `1025f` or `1026g` names its directory and filename (84 result files scanned). Owner lane = the one or two lanes whose result files name the file most often. `unattributed` means no wave result names the file, so the owner must be assigned by the coordinator.
- Scope in numbers: 86 code files (68 `.ts`, 8 `.mjs`, 8 `.mts`, 1 `.rs`, 1 `.py`), 6308 lines in total; 34 JSON schema/policy/text files were checked for EN/DE pairing; 2 markdown files were not in scope for code rules.
- Checks: (1) exports without a docstring directly above; (2) docstrings whose first character is not an emoji (ZWJ/keycap sequences accepted); (3) the same leading emoji on two or more docstrings in one file; (4) comments inside bodies (brace depth at least 1 for line and block comments, depth at least 2 for docstrings); (5) user-facing text without a DE counterpart; (6) policy values hard-coded in code instead of JSON/schema. Tooling: `🗑️generated/aud-docstr-g/audit_docstr.py` (streaming, peak RSS 14 MB) plus grep/awk cross-checks; every finding below was confirmed by reading the line.

## Summary

| Check | Findings |
|---|---|
| Exported declaration without docstring (TS/MJS/MTS) | 90 |
| Docstring not starting with an emoji | 3 (plus 1 keycap false positive, see section 8) |
| Same leading emoji reused inside one file | 190 docstrings in 69 file/emoji groups |
| Comments inside bodies | 1 (TS/MJS), 0 in Rust, 0 in Python bodies |
| User-facing text without DE counterpart | 18 confirmed rows (14 messages, 4 progress tokens), plus EN-only internal invariant throws in about 15 files (listed in section 5) |
| Policy values hard-coded in code | 13 confirmed |
| Python: missing docstrings on module functions | 2 (`refuse`, `kernel`) |
| Rust: `pub` items without `///` | 0 (FFI `extern` block declarations, lines 18-54, have none) |

## 1. Exported declarations without a docstring

Rule: every exported function, type, interface, class and value needs a `/** 🎯️ ... */` docstring directly above (AGENTS: docstrings start with a unique emoji). The `.d.mts` files are the type surface of their `.mjs` siblings, so they need the same docstrings.

Re-exports (`export type { ... } from`, 3 rows) are listed because the rule covers every exported name; the barrel file may take the docstring instead.

| File:line | Declaration | Owner lane |
|---|---|---|
| `🧰️framework/📦️packages/🟦️typescript/⚙️runtime/🧲️native/📚️libc/🟦️.ts:3` | `export type { RuntimeNativeLoadsV1 } from "../../../../../🔨️modules/⚙️runtime/🧲️nativ` | unattributed |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🪜️phases/🟦️.ts:7` | `export type HostPhaseV1=(typeof HOST_PHASES_V1)[number];` | phase-g |
| `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/👷️workers/🟦️.ts:3` | `export type { TestRunnerIdV1, TestRunnerV1, TestWorkerBoundV1, TestWorkerFactsV1, Tes` | testmem-g |
| `🧰️framework/🔨️modules/🏃️process/📦️packages/🟦️typescript/🧪️testing/👷️workers/🟦️.ts:5` | `export type { TestWorkerLearningV1 } from "../../../../🧪️testing/👷️workers/📐️learning` | testmem-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/⏱️eta/🟦️.ts:4` | `export type LeaseEtaV1=Readonly<{seconds:number;source:"progress"\|"history";overdue:` | lease-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/⏱️eta/🟦️.ts:5` | `export type LeaseEtaInputV1=Readonly<{elapsedMs:number;phaseElapsedMs:number;done?:nu` | lease-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/⚖️priority/🟦️.ts:6` | `export type LeasePriorityContextV1=Readonly<{agentSession:boolean;interactiveTerminal` | lease-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/⚖️priority/🟦️.ts:25` | `export type LeaseJobV1=Readonly<{id:string;class:LeasePriorityV1;arrival:number;servi` | lease-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/⚖️priority/🟦️.ts:26` | `export type LeaseJobOutcomeV1=Readonly<{id:string;class:LeasePriorityV1;arrival:numbe` | lease-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts:14` | `export type LeaseHolderStateV1=Readonly<{state:"legacy"\|"progressing"\|"silent"\|"st` | zt-g, route-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts:15` | `export type LeaseProgressLineV1=Readonly<{stage?:string;activity?:Readonly<{resource:` | zt-g, route-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts:110` | `export type LeaseHolderStateInputV1=Readonly<{nowMs:number;holdSinceMs:number;resourc` | zt-g, route-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts:13` | `export type LeaseReclaimModeV1="report"\|"request"\|"terminate";` | rev-g, lease-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts:33` | `export type LeaseYieldAcceptInputV1=Readonly<{request:LeaseYieldRequestV1;pid:number;` | rev-g, lease-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts:48` | `export type LeaseTerminateInputV1=Readonly<{mode:LeaseReclaimModeV1;state:LeaseHolder` | rev-g, lease-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts:99` | `export type LeaseHolderNoticeV1=Readonly<{holderState:LeaseHolderStateV1["state"];hol` | rev-g, lease-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts:100` | `export type LeaseObserveInputV1=Readonly<{directory:string;resource:string;headPid:nu` | rev-g, lease-g |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts:7` | `export const CHANGE_JOURNAL_POLICY_V1:ChangeJournalPolicyV1=Object.freeze(JSON.parse(` | unattributed |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts:28` | `export function readChangeJournalSwitchV1(environment:Readonly<Record<string,string\|` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📤️promotion/🟦️.ts:25` | `export type SealedStatusV1="fresh"\|"refreshed"\|"promoted"\|"served"\|"blocked"\|"un` | hyg2-g, self-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📤️promotion/🟦️.ts:113` | `export async function withPromotionLockV1<T>(entry:SealedEntryV1,options:Readonly<{po` | hyg2-g, self-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📤️promotion/🟦️.ts:135` | `export function collectSealedStoreV1(workspace:string,entry:SealedEntryV1,pointer:Sea` | hyg2-g, self-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📤️promotion/🟦️.ts:163` | `export async function resolveSealedImplementationV1(workspace:string,entry:SealedEntr` | hyg2-g, self-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts:9` | `export type ImplementationModeV1="sealed"\|"live"\|"promote";` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts:10` | `export type SealedTextV1=Readonly<Record<SealedLanguageV1,string>>;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts:11` | `export interface SealedPolicyV1{readonly version:1;readonly modeVariable:string;reado` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts:78` | `export function straceCommandV1(log: string, command: readonly string[]): string[] {` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts:83` | `export function linuxAvailableV1(launcher: string, python: string, platform: NodeJS.P` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts:90` | `export function* fileLinesV1(path: string): Generator<string> {` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts:106` | `export type LinuxToolsV1 = {readonly python: string; readonly launcher: string; reado` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts:109` | `export class LandlockRefusedV1 extends Error {` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts:117` | `export async function runLinuxV1(plan: ReadPlanV1, run: ReadRunV1, tools: LinuxToolsV` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🪞️mirror/🟦️.ts:58` | `export async function runMirrorV1(plan: ReadPlanV1, allowed: readonly string[], run: ` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:15` | `export interface WorldWitnessVerifyOptionsV1 { readonly signal?: AbortSignal; readonl` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:16` | `export function readWorldWitnessSwitchV1(environment?: Readonly<Record<string, string` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:17` | `export function renderWorldWitnessTextV1(table: "Messages" \| "Reasons", language: "e` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:18` | `export function installWorldWitnessV1(configText: string): WorldWitnessRecorderV1;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:19` | `export function worldWitnessPreloadV1(): string;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:20` | `export function worldWitnessEnvironmentV1(options: Readonly<{ out: string; root: stri` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:21` | `export function worldWitnessV1(): WorldWitnessRecorderV1 \| undefined;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:22` | `export function readWorldWitnessV1(text: string): WorldWitnessV1;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:23` | `export function verifyWorldWitnessV1(witness: WorldWitnessV1, options: WorldWitnessVe` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:24` | `export function worldWitnessGitDigestV1(listing: string \| Uint8Array): string;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:25` | `export function summarizeWorldWitnessV1(witness: WorldWitnessV1): Readonly<Record<str` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts:26` | `export function worldWitnessPathV1(value: string): string;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🟨️.d.mts:1` | `export interface TestOwnerScopeV1{readonly all:boolean;readonly roots:ReadonlySet<str` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🟨️.d.mts:4` | `export const compareTestOwnersV1:(a:string,b:string)=>number;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🟨️.d.mts:5` | `export const SEPARATE_TEST_OWNERS_V1:number;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🟨️.d.mts:6` | `export interface TestOwnerDiscoveryV1<Row>{readonly contributions:[string,{projects?:` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:4` | `export const CUTOFF_POLICY_V1:CutoffPolicyV1;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:9` | `export function isDerivedRowV1(row:GenerationRowV1):boolean;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:10` | `export function isDerivedOwnerV1(name:string):boolean;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:11` | `export function derivedOwnerNameV1(row:GenerationRowV1):string;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:12` | `export function derivedRowIdsV1(index:FactIndexV1):ReadonlySet<number>;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:13` | `export function verifiableRowIdsV1(index:FactIndexV1,ids:readonly number[]):number[];` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:14` | `export function scopeWithDerivedV1(index:FactIndexV1,names:ReadonlySet<string>):Set<s` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:15` | `export function splitStaleConsumersV1(index:FactIndexV1,classes:readonly number[],sta` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:16` | `export function holdersOfDerivedV1(index:FactIndexV1,pseudoOwners:Iterable<string>):M` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:17` | `export function settleSuspectsV1(options:{index:FactIndexV1;suspects:ReadonlySet<stri` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:18` | `export function derivedUpsertsV1(derived:readonly DerivedFactV1[],held?:ReadonlyMap<s` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:19` | `export function orphanedDerivedOwnersV1(index:FactIndexV1):string[];` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/✂️cutoff/🟨️.d.mts:20` | `export function unsupportedDerivedV1(index:FactIndexV1):GenerationRowV1[];` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.d.mts:1` | `export const FRONTIER_WALK_POLICY_V1:Readonly<{concurrency:number;racyMilliseconds:nu` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.d.mts:2` | `export interface FrontierWalkEntryV1{readonly name:string;readonly kind:"directory"\|` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.d.mts:3` | `export interface FrontierWalkStatV1{readonly directory:boolean;readonly file:boolean;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.d.mts:4` | `export interface FrontierWalkPortsV1{list(directory:string):Promise<readonly Frontier` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.d.mts:5` | `export interface FrontierWalkV1{directories:[string,number,number][];members:[string,` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.mjs:3` | `export const FRONTIER_WALK_POLICY_V1=Object.freeze({concurrency:64,racyMilliseconds:2` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🟨️.d.mts:1` | `export type GraphQueueYieldPolicyV1=Readonly<{version:1;yieldAfterMilliseconds:number` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🟨️.d.mts:3` | `export function graphQueueYieldPolicyV1():GraphQueueYieldPolicyV1;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🟨️.d.mts:4` | `export function graphQueueRequestedAtV1(environment?:Env):number\|undefined;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🟨️.d.mts:5` | `export function graphQueueYieldRoundV1(environment?:Env):number;` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🟨️.d.mts:6` | `export function graphQueueYieldsV1(options:{queuedMs:number;environment?:Env;yieldAft` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🟨️.d.mts:7` | `export function yieldGraphQueueV1(options:{generation:number;pointer:string;requested` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🛌️yield/🟨️.d.mts:8` | `export function waitGraphQueueYieldV1(options?:{environment?:Env;now?:()=>number;slee` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:3` | `export const CLAIMS_POLICY_V1:ClaimsPolicyV1;` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:4` | `export type ClaimMessageKeyV1="joined"\|"computing"\|"slotsBusy"\|"served"\|"failed"\` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:5` | `export const CLAIM_MESSAGES_V1:Readonly<Record<"en"\|"de",Readonly<Record<ClaimMessag` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:6` | `export function renderClaimMessageV1(key:ClaimMessageKeyV1,fields:Record<string,strin` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:9` | `export type SeenClaimV1=ClaimV1&{readonly live:boolean};` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:10` | `export function readClaimV1(value:unknown):ClaimV1;` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:11` | `export interface ChunkChoiceV1{readonly open:string[];readonly take:string[];readonly` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:12` | `export function chunkOfV1(options:{wanted:readonly string[];stamps?:Readonly<Record<s` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:13` | `export function slotCountV1(options:{memoryMegabytes:number;environment?:Record<strin` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:14` | `export interface ClaimLeaseHoldV1{release():void}` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:15` | `export interface ClaimLeasePortV1{tryHold(resource:string):Promise<ClaimLeaseHoldV1\|` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:16` | `export interface ClaimFilePortV1{read(path:string):Promise<Uint8Array\|undefined>;wri` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:17` | `export interface ClaimPrefetchOptionsV1{base:number;implementation:string;wanted:read` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:18` | `export interface ClaimRegistryV1{prefetch(options:ClaimPrefetchOptionsV1):Promise<{co` | tsfix-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.d.mts:19` | `export function createClaimRegistryV1(options:{root:string;fs:ClaimFilePortV1;lease:C` | tsfix-g |

## 2. Docstrings not starting with an emoji

| File:line | Found | Owner lane | Note |
|---|---|---|---|
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts:18` | 'A' | unattributed | Member docstring of an interface (depth 1): replace the first word with an emoji. |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts:20` | 'T' | unattributed | Member docstring of an interface (depth 1): replace the first word with an emoji. |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts:22` | 'T' | unattributed | Member docstring of an interface (depth 1): replace the first word with an emoji. |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts:32` | `#️⃣` (keycap) | unattributed | False positive: a keycap emoji (`#` + FE0F + U+20E3) is a valid emoji start. No change needed. |

## 3. Same emoji on two or more docstrings within one file

Rule: the leading emoji of each docstring must be unique within its file. Each row lists the emoji and all lines that reuse it. Counts: 190 docstrings in 69 file/emoji groups.

| File | Emoji | Lines | Owner lane |
|---|---|---|---|
| `🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🟦️.ts` | 🎯️ | 16,44 | nodeffi-g |
| `🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🟦️.ts` | 🔌️ | 20,22 | nodeffi-g |
| `🧰️framework/🔨️modules/⚙️runtime/🟢️node/🧲️native/🏗️addon/🟦️.ts` | 🗺️ | 18,61 | nodeffi-g |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🔁️incremental/🟦️.ts` | 🛎️ | 86,89 | hyg2-g, ts2-g |
| `🧰️framework/🔨️modules/🏃️process/📦️artifacts/🏗️native-build/💾️storage/🌐️platform/🥾️boot/🩹️recovery/🕰️clock/🪟️native/🟦️.ts` | 🪟️ | 7,14 | unattributed |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts` | 🌡️ | 63,65,77 | zt-g, route-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts` | 🏷️ | 84,86 | zt-g, route-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts` | 📈️ | 12,91 | zt-g, route-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts` | 🔌️ | 17,21,72,75 | zt-g, route-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts` | 🚪️ | 130,134,136 | zt-g, route-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts` | 🛑️ | 138,140 | zt-g, route-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🟦️.ts` | 🪪️ | 10,25 | zt-g, route-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/📈️standing/🤝️yield/🟦️.ts` | 🔌️ | 63,71 | rev-g, lease-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🪟️windows/🟦️.ts` | 🔁️ | 38,41 | hyg2-g, xplat-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🪟️windows/🟦️.ts` | 🚧️ | 17,23 | hyg2-g, xplat-g |
| `🧰️framework/🔨️modules/🏃️process/🔒️leases/🪟️windows/🟦️.ts` | 🪟️ | 6,9 | hyg2-g, xplat-g |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🌳️tree/🟦️.ts` | 🌳️ | 6,8 | unattributed |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts` | ✍️ | 168,177 | unattributed |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts` | 🎯️ | 59,238 | unattributed |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts` | 💓️ | 94,217 | unattributed |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts` | 📖️ | 126,128 | unattributed |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts` | 🗂️ | 19,26,84,209 | unattributed |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts` | 🚫️ | 15,17,229 | unattributed |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🟦️.ts` | 🧬️ | 50,52 | unattributed |
| `🧰️framework/🔨️modules/📁️filesystem/🧾️observation/📜️changes/🧭️client/🟦️.ts` | 🧭️ | 9,16,38 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts` | 💰️ | 15,62 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts` | 🧭️ | 13,50 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts` | 🧮️ | 21,107 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🔌️nx-plugin/🧮️engine-digest/🟦️.ts` | 🌱️ | 10,37 | nxperf-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧊️memo/🟦️.ts` | 🪪️ | 30,34 | hyg2-g, rev2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🟦️.ts` | 🔭️ | 6,28 | nxret-g, cache-f |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts` | 🏊️ | 9,64 | lane-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts` | 📐️ | 11,23 | lane-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📤️promotion/🟦️.ts` | 🔬️ | 33,49 | hyg2-g, self-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📤️promotion/🟦️.ts` | 🧪️ | 23,56 | hyg2-g, self-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🚦️trampoline/🟦️.ts` | 🚦️ | 9,17 | self-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🚪️entry/🟦️.ts` | 🚪️ | 18,29 | self-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts` | 🌳️ | 18,40,148 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts` | 👈️ | 20,46,94 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts` | 📖️ | 100,115 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts` | 🗂️ | 81,85,90 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts` | 🚫️ | 28,56,98 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts` | 🧮️ | 22,167 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts` | 🧾️ | 24,50,96 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🟦️.ts` | 🔭️ | 13,15 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🟨️.d.mts` | 🌳️ | 1,3,5 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🌳️owner-view/🟨️.mjs` | 🌳️ | 1,7,12 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🍎️seatbelt/🟦️.ts` | 🍎️ | 11,20,32,39,46,54,59 | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🏃️run/🟦️.ts` | 🏃️ | 6,33 | nxret-g, cache-f |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts` | 🐧️ | 7,10,25,39,46,76,81,88,104,107,115 | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts` | 🚪️ | 5,23 | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🟦️.ts` | 🔭️ | 13,15,17,19,21,23,29,34,39,46,55 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🪞️mirror/🟦️.ts` | 🪞️ | 6,9,19,39,44,56 | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🪢️policy/🟦️.ts` | 🪢️ | 1,4,7,10,13,16,19,24,58,63,70,85 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.d.mts` | 🔭️ | 1,7 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.mjs` | 🔭️ | 9,211 | nodeffi-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🟨️.d.mts` | 🧪️ | 2,7 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🟨️.mjs` | 🔢️ | 15,18 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🎯️test-owners/🟨️.mjs` | 🧪️ | 3,21 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.mjs` | 🗣️ | 9,19 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🧩️claims/🟨️.mjs` | 🧩️ | 7,71 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts` | 🎯️ | 78,131 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts` | 🏛️ | 22,73 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts` | 📖️ | 43,61 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts` | 📚️ | 40,88 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts` | 🔢️ | 28,140 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts` | 🗺️ | 26,122 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts` | 🚩️ | 34,169 | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/🟦️.ts` | 🧠️ | 30,155 | unattributed |

Also in Rust (`🧲️native/🦀️.rs`): `🔢️` at lines 74, 85, 95 and `📚️` at lines 102, 125. Owner: nodeffi-g.

## 4. Comments inside function or class bodies

| File:line | Comment | Owner lane |
|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔭️witness/🟨️.mjs:168` | `block comment at depth 2: /* a runtime without bun:sqlite has nothing to observe */` | nodeffi-g |
Rule: no comments inside definitions. The comment at line 168 only says why the `catch` is empty, so the reason belongs in the docstring of the enclosing function or in a named `catch` helper. Rust (`🦀️.rs`) and Python (`🐍️.py`) have no comments inside bodies; their only `//` lines are `///` docstrings and `#[cfg]` attributes.

## 5. User-facing strings without a DE counterpart

Confirmed user-visible (refusals, notices, progress, CLI errors). JSON catalogues in this scope are all paired EN/DE (checked: 0 unpaired nodes), so these are inline strings that bypass the catalogue.

| File:line | Kind | Text (EN only) | Owner lane |
|---|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts:11` | CLI error | `unexpected argument: ${name}` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🚪️cli/🟦️.ts:15` | CLI error | `--language, --root and --allow-list are required` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🟦️.ts:42` | refusal | `Read oracle request refused: ${failures}` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🍎️seatbelt/🟦️.ts:41` | availability reason | `Seatbelt needs macOS, this is ${platform}` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🍎️seatbelt/🟦️.ts:43` | availability reason | `sandbox-exec failed: ...` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🟦️.ts:84` | availability reason | `Landlock needs Linux, this is ${platform}` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts:32` | policy error (user-edited policy file) | `Closed machine disk budget required: ...` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts:42` | policy error (user-edited policy file) | `Closed machine disk budget required: weights add up to ..., not 10000` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts:43` | policy error (user-edited policy file) | `Closed machine disk budget required: at most one loose store` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/💾️storage/🧾️ledger/🎚️machine/🟦️.ts:45` | policy error (user-edited policy file) | `Closed machine disk budget required: floors exceed the default total` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🟦️.ts:74` | refusal | `The change journal cannot vouch: ${reason}` | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🟦️.ts:40` | progress (raw token) | `nx-tooling:census:${seen}` | nxret-g, cache-f |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🟦️.ts:55` | progress (raw token) | `nx-tooling:probe:${probed}/${total}` | nxret-g, cache-f |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🟦️.ts:62` | progress (raw token) | `nx-tooling:wait:${seconds}s` | nxret-g, cache-f |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🟦️.ts:64` | progress (raw token) | `nx-tooling:retire:${index}/${total}:${identity}` | nxret-g, cache-f |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🧾️ci-report/🗂️suites/📜️script.ts:93` | pass-through error | `${(error as Error).message} printed as is` | testmem-g, route-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🐍️.py:14` | python stderr refusal | `landlock-unavailable: ${text}` | hyg2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🐧️linux/🐍️.py:19` | python stderr refusal | `needs Linux, this is ${sys.platform}` | hyg2-g |

Notes: `🧭️client/🟦️.ts:32` and `🧊️memo/🟦️.ts:49` and `🔭️witness/🟨️.mjs:16` already carry `EN / DE` inline (compliant, but a catalogue key would match the rest of the repo). The progress tokens at run/🟦️.ts lines 40-64 are keys that the renderer must localise; no DE text exists in this file, so the renderer owner must confirm. Internal invariant throws (EN only) are not listed one by one: `🔁️incremental/🟦️.ts:44,52`, `📸️snapshot/🟦️.ts:32`, `🪶️sparse/🟦️.ts:4`, `🔁️promoter/🟦️.ts:18,19`, `📤️retention/🟦️.ts:69,110`, `🔭️witness/🟨️.mjs:22,219,221,222`, `🧩️claims/🟨️.mjs:22,29-39,76,121`, `✂️cutoff/🟨️.mjs:77`, `🌿️frontier-walk/🟨️.mjs:26,37`, `📜️changes/🟦️.ts:80`. These reach a user only through a crash or a bug report, so they are low priority; the frontier-walk "removed during discovery" messages are the candidates to upgrade because they surface as build failures.

## 6. Policy constants hard-coded in code

Rule: policy values come from a closed JSON policy or schema (AGENTS: schema-first). Unit conversions (1024, 1_000_000, 10000 basis points) and schema invariants are accepted; the rows below are operating limits and timings.

| File:line | Value | Should live in | Owner lane |
|---|---|---|---|
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🟦️.ts:58` | `?? 500` memory limit in MB (the 500 MB script abort rule) | read-oracle policy JSON | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🔏️authorizers/🧊️memo/🟦️.ts:29` | `MAXIMUM_FILE_BYTES=4_194_304n`, `CONCURRENCY=24` | authorizer memo policy JSON | hyg2-g, rev2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle/🏃️run/🟦️.ts:9` | `TAIL_BYTES = 65536` | read-oracle policy JSON | nxret-g, cache-f |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🔁️incremental/🟦️.ts:22` | `TYPESCRIPT_CHECK_RETENTION_V1 = {retainedPrograms:2, maximumHeapBytes:2*1024**3, idleMilliseconds:120000}` (an object literal, not loaded) | typescript-check policy JSON | hyg2-g, ts2-g |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🟦️typescript/🔁️incremental/🟦️.ts:44` | range `1..8` for retainedPrograms | schema bounds | hyg2-g, ts2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/🟨️.mjs:3` | `FRONTIER_WALK_POLICY_V1 = {concurrency:64, racyMilliseconds:2000}` | frontier-walk policy JSON | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🚀️bootstrap/🛠️tools/🧹️retention/🎚️store-bound/🏃️run/🔌️host/🟦️.ts:9` | `admissionPatienceMs:1000` | host-capacity policy JSON | nxret-g |
| `🧰️framework/🔨️modules/🏃️process/🏗️build/🌡️host-capacity/🔼️upgrade/🟦️.ts:88` | jitter `0.75 + Math.random()*0.5` on the wait | host-capacity policy JSON | preadmit-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/⚡️caching/🦀️cargo/🗃️unit-cache/🧵️lane/🏊️pool/🟦️.ts:78` | backoff factor `poll*1.5` | lane pool policy JSON (its sibling `pollMaximumMilliseconds` already is policy) | lane-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏡️resident/📜️journal/🟦️.ts:127` | `setTimeout(resolve,20)` poll interval | journal policy JSON | unattributed |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🔁️promoter/🟦️.ts:18` | `join(workspace,row.path).length>256` (path limit) | sealed policy JSON (AGENTS 256-character rule) | self-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/📥️execution/🟦️.ts:20` | `32767` Windows command-line cap | sealed policy JSON or platform table | self-g, lane2-g |
| `🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🏛️sealed/🟦️.ts:35` | `maximum = 256` default text length | sealed schema default | unattributed |

Also: `🐍️.py` line 10 `UNAVAILABLE = 125` (exit code) is fine as a constant, but it is not in the schema that the TypeScript side reads. Policy values that already come from schema, for reference: `📸️snapshot` concurrency, `📥️execution` `maximumSnapshotBytes`, `🧩️claims` `P.*`, `🎚️machine` budget policy JSON.

## 7. Checked and clean

- All 34 JSON schema, policy and text files in scope: every bilingual node has both `en` and `de`.
- Rust `🦀️.rs`: the two `pub` exports (`napi_register_module_v1` line 282, `node_api_module_get_api_version_v1` line 295) have `///` docstrings starting with an emoji. No comment inside a function body.
- Python `🐍️.py`: the module docstring starts with an emoji and is in EN; `refuse` and `kernel` lack docstrings (section 1 rows).
- Test files are excluded from the scope by rule.

## 8. Limitations and false positives

- The keycap at `sealed/🟦️.ts:32` (`#️⃣`) is valid; the scanner reports it because it looks at `#` plus FE0F only.
- Comment detection is a tokenizer that skips strings, templates and comments; regex literals containing quotes can confuse it. Confirmed with grep: no inline `//` comments exist in the TS files; the one block comment at depth 2 is the only hit.
- The `POLICY-CONST?` scan produced 336 raw candidates; most are unit conversions or local arithmetic. Only the rows above were confirmed by reading the line. The scanner is `🗑️generated/aud-docstr-g/audit_docstr.py`; rerun it on the code list to reproduce.
- Owner lanes come from result-file mentions. Files marked `unattributed` (34 of 86) are named by no wave result; the coordinator must assign them.
- The 39 staged production files not referenced by any 1025f/1026g result were out of scope (they came from earlier waves or are not yet claimed).

## Changed-file tracking

Nothing was edited in source and no git command that modifies state was run. The only written files are this report and the scanner scripts (`audit_docstr.py`, `make_report.py`, `langcheck.py`) in `🗑️generated/aud-docstr-g/`. Intermediate scan outputs were deleted after the report was built.

