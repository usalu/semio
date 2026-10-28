OPS = {}
OPS["⏱️eta"] = dict(doc=[
 ("export type LeaseEtaV1", "⏲️ One estimate: the seconds left, whether it came from the holder's own counter or from earlier holds, and whether the hold is overdue."),
 ("export type LeaseEtaInputV1", "🧾️ The evidence of an estimate: elapsed times, the holder's own counter and the durations of earlier holds."),
])
OPS["⚖️priority"] = dict(doc=[
 ("export type LeasePriorityContextV1", "🖥️ What the class of a process depends on besides the environment: whether it runs inside an agent session and whether a person sits at its terminal."),
 ("export type LeaseJobV1", "📋️ One job of the queue simulation: its arrival and service time in seconds."),
 ("export type LeaseJobOutcomeV1", "🏁️ What the queue simulation reports for one job: when it started and how long it waited, in seconds."),
])
OPS["📈️standing/🟦️.ts"] = dict(
 emo={21:"📝",25:"🧬",65:"🔬",72:"🔧",75:"♻",77:"📡",86:"🎖",91:"📸",134:"🪝",136:"🔍",140:"⛔"},
 doc=[
 ("export type LeaseHolderStateV1", "🚦️ The judgement of a holder: legacy, progressing, silent, stuck or stale, with the silent time or the ids of the moved components."),
 ("export type LeaseProgressLineV1", "📜️ One progress line of this process: its stage or the resource of its current activity."),
 ("export type LeaseHolderStateInputV1", "📥️ What judging a holder needs: the clock, the hold start, the resource, the holder's standing and the requester's identity with the components that bind the resource."),
])
OPS["🤝️yield"] = dict(emo={71:"🧷"}, doc=[
 ("export type LeaseReclaimModeV1", "🛟️ How far a waiter may go against a holder that is stale or stuck: only report it, ask it to yield, or stop it."),
 ("export type LeaseYieldAcceptInputV1", "📬️ What a holder weighs before honouring a request: the request, its own identity and hold, and the components that bind and moved."),
 ("export type LeaseTerminateInputV1", "🔪️ What the decision to stop a holder weighs: the mode, the holder's state, the request time, progress since, both start times and whether the process may be signalled."),
 ("export type LeaseHolderNoticeV1", "📢️ What the queue notice of a waiter says about the holder: its state, phase, counters and the minutes left."),
 ("export type LeaseObserveInputV1", "🔭️ What observing the holder at the head of the queue needs: the lease directory, the resource, the head's pid, mode and arrival, and the injectable clock, environment, root and ports."),
])
OPS["🪟️windows"] = dict(emo={9:"🏠",23:"🗄",41:"🔂"})

OPS["📜️changes/🟦️.ts"] = dict(
 emo={17:"🙅",26:"📁",84:"📥",209:"💾",52:"🔡",238:"🔦",217:"🫀",128:"📚",177:"🆕",229:"⛔"},
 doc=[("export const CHANGE_JOURNAL_POLICY_V1", "📐️ The declared constants of the change journal, read from its schema and frozen.")])
OPS["📜️changes/🌳️tree"] = dict(emo={8:"🆚"})
OPS["🧭️client"] = dict(
 emo={16:"🛰",38:"🏗",18:"🚷+",20:"📍+",22:"🔎+"},
 doc=[("export function readChangeJournalSwitchV1", "🔌️ Reads the switch of the change journal from the environment: whether the consumers may ask it at all.")])
OPS["🎚️machine"] = dict(emo={50:"📜",62:"🧾",108:"🗺"})
OPS["🧮️engine-digest"] = dict(emo={37:"❓"})
OPS["🧊️memo"] = dict(emo={34:"🧬"})
OPS["🎚️store-bound/🏃️run/🟦️.ts"] = dict(emo={28:"🧮"})
OPS["🏊️pool"] = dict(emo={23:"🪣",64:"🎒"})
OPS["🪟️native"] = dict(emo={14:"🕰"})
OPS["🏗️addon/🟦️.ts"] = dict(emo={22:"🧰",44:"🧭",61:"📍"})
OPS["📚️libc"] = dict(doc=[("export type { RuntimeNativeLoadsV1 }", "🫙️ What a libc candidate resolves to once it is loaded, re-exported from the module that owns it.")])
OPS["🪜️phases"] = dict(doc=[("export type HostPhaseV1", "🪪️ The name of one phase of a run: assembly, plan, rebuild or execution.")])
OPS["👷️workers"] = dict(doc=[
 ("export type { TestRunnerIdV1", "🪪️ The types of the test worker plan, re-exported from the module that owns them: runner identity, bounds, facts, options, plan and policy."),
 ("export type { TestWorkerLearningV1", "📐️ What the test worker memory learned, re-exported from the module that owns it."),
])
OPS["🔁️incremental"] = dict(emo={89:"🛰"})
OPS["🕰️generations/✂️cutoff/🟨️.d.mts"] = dict(doc=[
 ("export const CUTOFF_POLICY_V1", "📐️ The declared constants of the derived fact cutoff."),
 ("export function isDerivedRowV1", "🏷️ Whether a row is a derived fact: the answer of a question, never a raw fact that can be checked against the disk."),
 ("export function isDerivedOwnerV1", "👻️ Whether an owner name is the pseudo owner that holds a derived fact together with its support."),
 ("export function derivedOwnerNameV1", "🪪️ The pseudo owner of one derived fact, named by the whole row."),
 ("export function derivedRowIdsV1", "🧾️ The ids of the derived rows of an index."),
 ("export function verifiableRowIdsV1", "🚫️ Drops the derived rows from the row ids that are checked against the disk."),
 ("export function scopeWithDerivedV1", "🎯️ Adds to a set of project names the pseudo owners that support the derived facts those projects hold."),
 ("export function splitStaleConsumersV1", "🔀️ Splits the consumers of the stale raw rows into projects that read a stale row themselves and pseudo owners whose derived fact has to be asked again."),
 ("export function holdersOfDerivedV1", "👥️ The projects that hold the derived fact of each pseudo owner."),
 ("export function settleSuspectsV1", "❓️ Asks every suspect question again: an equal answer holds, a different answer or a failing computation changes the fact."),
 ("export function derivedUpsertsV1", "🧱️ The rows of the pseudo owners a build writes: the fresh facts with their support and the supports of the questions that still held."),
 ("export function orphanedDerivedOwnersV1", "🧹️ The pseudo owners whose derived fact no project holds any more."),
 ("export function unsupportedDerivedV1", "🚨️ The derived rows that projects hold and no pseudo owner supports."),
])
OPS["🌿️frontier-walk/🟨️.d.mts"] = dict(doc=[
 ("export const FRONTIER_WALK_POLICY_V1", "📐️ The declared constants of the frontier walk: the parallel listings, the racy window and the root directory."),
 ("export interface FrontierWalkEntryV1", "🗒️ One entry of a directory listing: its name and whether it is a directory, a file or something else."),
 ("export interface FrontierWalkStatV1", "📊️ The stamp of one path: kind, modification time, inode and mode."),
 ("export interface FrontierWalkPortsV1", "🔌️ What a walk asks of the world: listing, stat, the descend and member rules and the cancellation signal."),
 ("export interface FrontierWalkV1", "🌿️ A walked frontier: every listed directory with its stamp and every member with its mode."),
])
OPS["🛌️yield/🟨️.d.mts"] = dict(doc=[
 ("export type GraphQueueYieldPolicyV1", "📋️ The closed numbers, variable names, exit code and EN/DE texts of the graph queue yield."),
 ("export function graphQueueYieldPolicyV1", "📜️ The closed numbers, variable names and EN/DE texts of the graph queue yield, exactly as the policy declares them."),
 ("export function graphQueueRequestedAtV1", "🕰️ The time the first start of this command asked for the graph; undefined for a first start."),
 ("export function graphQueueYieldRoundV1", "🔢️ How often this command has already yielded the graph queue."),
 ("export function graphQueueYieldsV1", "⚖️ Whether a command that has queued for the declared time for the graph builder yields now."),
 ("export function yieldGraphQueueV1", "🛌️ Ends the waiting process: writes the ticket, tells the user in both languages and exits with the yield code."),
 ("export function waitGraphQueueYieldV1", "⏳️ The front door's wait after a yielded run: polls the pointer until a newer generation was published or the wake time passed."),
])
OPS["🧩️claims/🟨️.d.mts"] = dict(doc=[
 ("export const CLAIMS_POLICY_V1", "📐️ The declared constants of the claim registry."),
 ("export type ClaimMessageKeyV1", "🔑️ The keys of the bilingual prefetch texts."),
 ("export const CLAIM_MESSAGES_V1", "🗣️ The bilingual texts of the prefetch."),
 ("export function renderClaimMessageV1", "🖨️ Renders one prefetch message in the language the caller resolved."),
 ("export type SeenClaimV1", "👀️ A claim as one command sees it: with whether the lease of a running claim is held."),
 ("export function readClaimV1", "🛂️ Admits a closed claim document or refuses it."),
 ("export interface ChunkChoiceV1", "📦️ What one command computes next: the open owners, the ones it takes, the ones a peer is running and how many are prepared or covered."),
 ("export function chunkOfV1", "🧮️ Chooses what one command computes next from the claims it sees."),
 ("export function slotCountV1", "🎟️ The number of compute slots: the environment value in range, else the memory share divided by the slot size."),
 ("export interface ClaimLeaseHoldV1", "✋️ A held lease that is given back by `release`."),
 ("export interface ClaimLeasePortV1", "🔐️ The lease operations the registry needs: try, hold and wait for a resource."),
 ("export interface ClaimFilePortV1", "🗄️ The file operations the registry needs: read, exclusive write, list and remove."),
 ("export interface ClaimPrefetchOptionsV1", "🛠️ What one prefetch needs: the plan, the wanted owners, the computation and the optional progress, notice and cancellation ports."),
 ("export interface ClaimRegistryV1", "📒️ The registry of claims: prefetch, sweep, list and the names of its lease resources."),
 ("export function createClaimRegistryV1", "🏗️ Creates the claim registry of the generation documents under `root` over a file port and a lease port."),
])
OPS["🎯️test-owners/🟨️.d.mts"] = dict(emo={}, doc=[
 ("export interface TestOwnerScopeV1", "🎯️ Which test owners a partial inference discovers again: all of them, or the ones under the roots that are not known yet."),
 ("export const compareTestOwnersV1", "🔢️ The code point order of test owner directories."),
 ("export const SEPARATE_TEST_OWNERS_V1", "📏️ How many test owners are discovered one by one before the rest is discovered in one call."),
 ("export interface TestOwnerDiscoveryV1", "📦️ What discovering test owners answers: the contributions, the rows by project and the derived facts."),
]); OPS["🎯️test-owners/🟨️.d.mts"]["emo"] = {7:"🔭"}
OPS["🎯️test-owners/🟨️.mjs"] = dict(emo={18:"📏",22:"🔭"})
OPS["🧩️claims/🟨️.mjs"] = dict(emo={19:"🖨",72:"🏗"})
OPS["🌳️owner-view/🟨️.d.mts"] = dict(emo={3:"🔒",5:"✂"})
OPS["🌳️owner-view/🟨️.mjs"] = dict(emo={7:"🔒",12:"✂"})
OPS["witness/🟨️.mjs"] = dict(emo={211:"🔦"}, sub=[
 ("} catch { /* a runtime without bun:sqlite has nothing to observe */ }", "} catch {}"),
 ("The function is self-contained: its source text is the preload of the planner child. */", "The function is self-contained: its source text is the preload of the planner child. A runtime without `bun:sqlite` has nothing to observe there, so its absence is ignored. */"),
])
OPS["witness/🟨️.d.mts"] = dict(emo={7:"🔬"}, doc=[
 ("export interface WorldWitnessVerifyOptionsV1", "🛠️ How a witness is checked: cancellation, parallelism, progress, the caller's checks and the environment."),
 ("export function readWorldWitnessSwitchV1", "🎚️ Reads the plan reuse switch; an unknown value is a refusal, never a silent default."),
 ("export function renderWorldWitnessTextV1", "💬️ Renders a message or a reason in one declared language; there is no hidden default language."),
 ("export function installWorldWitnessV1", "📸️ Records, inside the process that runs it, every fact of the world one computation read and names what it cannot record as tainted."),
 ("export function worldWitnessPreloadV1", "📜️ The source text of the preload that installs the witness in a child process."),
 ("export function worldWitnessEnvironmentV1", "🌍️ The environment a child needs to run under the witness; the output file receives the finished witness."),
 ("export function worldWitnessV1", "👁️ The witness of the process that runs here, when it was started under the preload; otherwise nothing."),
 ("export function readWorldWitnessV1", "📖️ Admits a witness file the planner child wrote; anything malformed is refused."),
 ("export function verifyWorldWitnessV1", "⚖️ Tells whether every observation of a witness still holds, stopping at the first that differs."),
 ("export function worldWitnessGitDigestV1", "🧮️ Digests the tracked-file listing and the ignore files a git universe depends on."),
 ("export function summarizeWorldWitnessV1", "🗂️ The names the observations of a witness group under, with their counts."),
 ("export function worldWitnessPathV1", "🔒️ Resolves a path the way the witness records it."),
])

OPS["read-oracle/🍎️seatbelt"] = dict(

 sub=[
  ('import {captureV1} from "../🏃️run/🟦️.ts";', 'import {captureV1, readOracleBothV1} from "../🏃️run/🟦️.ts";'),
  ('reason: `Seatbelt needs macOS, this is ${platform}`', 'reason: readOracleBothV1("seatbeltPlatform", {platform}, " / ")'),
  ('reason: `sandbox-exec failed: ${(probe.stderr || String(probe.error ?? "")).trim().slice(0, 200)}`', 'reason: readOracleBothV1("seatbeltProbe", {reason: (probe.stderr || String(probe.error ?? "")).trim().slice(0, 200)}, " / ")'),
 ])
OPS["read-oracle/🐧️linux/🟦️.ts"] = dict(

 sub=[
  ('import {captureV1} from "../🏃️run/🟦️.ts";', 'import {captureV1, readOracleBothV1} from "../🏃️run/🟦️.ts";'),
  ('reason: `Landlock needs Linux, this is ${platform}`', 'reason: readOracleBothV1("landlockPlatform", {platform}, " / ")'),
 ])
OPS["read-oracle/🏃️run/🟦️.ts"] = dict(

 sub=[
  ('const TAIL_BYTES = 65536;\n', '''const TAIL_BYTES = document.policy.tailBytes;

/** 🌐️ Fills the placeholders of one message of the document in one language; an unknown placeholder stays visible. */
export function readOracleMessageV1(language: "en" | "de", key: keyof typeof document.messages, values: Readonly<Record<string, string | number>> = {}): string {
  return document.messages[key][language].replace(/\\{(\\w+)\\}/gu, (_, name: string) => String(values[name] ?? `{${name}}`));
}

/** 🌍️ Renders one message in both languages, English first, joined by `separator`: for refusals raised before a language is known and for reasons that travel inside other messages. */
export function readOracleBothV1(key: keyof typeof document.messages, values: Readonly<Record<string, string | number>> = {}, separator = "\\n"): string {
  return (["en", "de"] as const).map((language) => readOracleMessageV1(language, key, values)).join(separator);
}
'''),
 ])
OPS["read-oracle/🟦️.ts"] = dict(

 sub=[
  ('import {captureV1, type CapturedV1} from "./🏃️run/🟦️.ts";', 'import {captureV1, readOracleBothV1, readOracleMessageV1, type CapturedV1} from "./🏃️run/🟦️.ts";'),
  ('  return document.messages[key][language].replace(/\\{(\\w+)\\}/gu, (_, name: string) => String(values[name] ?? `{${name}}`));', '  return readOracleMessageV1(language, key, values);'),
  ('throw new Error(`Read oracle request refused: ${failures.slice(0, 3).join("; ")}`);', 'throw new Error(readOracleBothV1("requestRefused", {failures: failures.slice(0, 3).join("; ")}));'),
  ('ports.memoryLimitMegabytes ?? 500;', 'ports.memoryLimitMegabytes ?? document.policy.memoryLimitMegabytes;'),
 ])
OPS["read-oracle/🚪️cli"] = dict(

 sub=[
  ('import {readOracleTextV1, runReadOracleV1, windowsPlanV1, type ReadOracleReportV1} from "../🟦️.ts";', 'import {readOracleTextV1, runReadOracleV1, windowsPlanV1, type ReadOracleReportV1} from "../🟦️.ts";\nimport {readOracleBothV1} from "../🏃️run/🟦️.ts";'),
  ('throw new Error(`unexpected argument: ${name}`);', 'throw new Error(readOracleBothV1("argumentUnexpected", {name}));'),
  ('throw new Error("--language, --root and --allow-list are required");', 'throw new Error(readOracleBothV1("argumentsRequired"));'),
 ])
OPS["read-oracle/🪞️mirror"] = dict()
OPS["read-oracle/🪢️policy"] = dict()
OPS["read-oracle/🐧️linux/🐍️.py"] = dict(sub=[
 ('def refuse(text):\n    sys.stderr.write(f"landlock-unavailable: {text}\\n")\n', 'def refuse(english, german):\n    """🚫️ Ends the launcher with exit code 125 and the reason in English and German on standard error, where the read oracle picks it up."""\n    sys.stderr.write(f"landlock-unavailable: {english} / {german}\\n")\n'),
 ('refuse(f"needs Linux, this is {sys.platform}")', 'refuse(f"needs Linux, this is {sys.platform}", f"braucht Linux, dies ist {sys.platform}")'),
 ('def kernel(number, *args):\n', 'def kernel(number, *args):\n    """⚙️ Calls one Landlock system call and returns its result with the error number."""\n'),
 ('refuse(f"landlock_create_ruleset version probe failed: {os.strerror(error)}")', 'refuse(f"landlock_create_ruleset version probe failed: {os.strerror(error)}", f"Versionsabfrage von landlock_create_ruleset fehlgeschlagen: {os.strerror(error)}")'),
 ('refuse(f"landlock_create_ruleset failed: {os.strerror(error)}")', 'refuse(f"landlock_create_ruleset failed: {os.strerror(error)}", f"landlock_create_ruleset fehlgeschlagen: {os.strerror(error)}")'),
 ('refuse(f"landlock_add_rule failed for {path}: {os.strerror(error)}")', 'refuse(f"landlock_add_rule failed for {path}: {os.strerror(error)}", f"landlock_add_rule fehlgeschlagen für {path}: {os.strerror(error)}")'),
 ('refuse("prctl(PR_SET_NO_NEW_PRIVS) failed")', 'refuse("prctl(PR_SET_NO_NEW_PRIVS) failed", "prctl(PR_SET_NO_NEW_PRIVS) fehlgeschlagen")'),
 ('refuse(f"landlock_restrict_self failed: {os.strerror(error)}")', 'refuse(f"landlock_restrict_self failed: {os.strerror(error)}", f"landlock_restrict_self fehlgeschlagen: {os.strerror(error)}")'),
])
D = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️inputs/🔭️read-oracle"
OPS[D + "/🔣️.json"] = dict(sub=[
 ('  "version": 1,\n  "messages": {', '  "version": 1,\n  "policy": {\n    "memoryLimitMegabytes": 500,\n    "tailBytes": 65536\n  },\n  "messages": {'),
 ('    }\n  },\n  "windows": {', '''    },
    "argumentUnexpected": {
      "en": "unexpected argument: {name}",
      "de": "unerwartetes Argument: {name}"
    },
    "argumentsRequired": {
      "en": "--language, --root and --allow-list are required",
      "de": "--language, --root und --allow-list sind erforderlich"
    },
    "requestRefused": {
      "en": "Read oracle request refused: {failures}",
      "de": "Anfrage an das Lese-Orakel abgelehnt: {failures}"
    },
    "seatbeltPlatform": {
      "en": "Seatbelt needs macOS, this is {platform}",
      "de": "Seatbelt braucht macOS, dies ist {platform}"
    },
    "seatbeltProbe": {
      "en": "sandbox-exec failed: {reason}",
      "de": "sandbox-exec ist fehlgeschlagen: {reason}"
    },
    "landlockPlatform": {
      "en": "Landlock needs Linux, this is {platform}",
      "de": "Landlock braucht Linux, dies ist {platform}"
    }
  },
  "windows": {'''),
])
OPS[D + "/🧬️schema/🔣️.json"] = dict(sub=[
 ('    "version",\n    "messages",\n    "windows"\n  ],', '    "version",\n    "policy",\n    "messages",\n    "windows"\n  ],'),
 ('    "messages": {\n      "$ref": "#/definitions/Messages"\n    },', '    "policy": {\n      "$ref": "#/definitions/Policy"\n    },\n    "messages": {\n      "$ref": "#/definitions/Messages"\n    },'),
 ('    "Text": {\n      "type": "object",', '''    "Policy": {
      "type": "object",
      "additionalProperties": false,
      "required": [
        "memoryLimitMegabytes",
        "tailBytes"
      ],
      "properties": {
        "memoryLimitMegabytes": {
          "type": "integer",
          "minimum": 64,
          "maximum": 65536
        },
        "tailBytes": {
          "type": "integer",
          "minimum": 1024,
          "maximum": 16777216
        }
      }
    },
    "Text": {
      "type": "object",'''),
 ('        "outputDiffers",\n        "batchCommand"\n      ],', '        "outputDiffers",\n        "batchCommand",\n        "argumentUnexpected",\n        "argumentsRequired",\n        "requestRefused",\n        "seatbeltPlatform",\n        "seatbeltProbe",\n        "landlockPlatform"\n      ],'),
 ('        "batchCommand": {\n          "$ref": "#/definitions/Text"\n        }\n      }', '        "batchCommand": {\n          "$ref": "#/definitions/Text"\n        },\n        "argumentUnexpected": {\n          "$ref": "#/definitions/Text"\n        },\n        "argumentsRequired": {\n          "$ref": "#/definitions/Text"\n        },\n        "requestRefused": {\n          "$ref": "#/definitions/Text"\n        },\n        "seatbeltPlatform": {\n          "$ref": "#/definitions/Text"\n        },\n        "seatbeltProbe": {\n          "$ref": "#/definitions/Text"\n        },\n        "landlockPlatform": {\n          "$ref": "#/definitions/Text"\n        }\n      }'),
])

# ---- wave 2 of conv-g: sealed store, journal, ci-report, native, policy constants, bilingual refusals
def ext(alias, **kw):
    entry = OPS.setdefault(alias, {})
    for key, value in kw.items():
        if isinstance(value, dict): entry.setdefault(key, {}).update(value)
        else: entry.setdefault(key, []).extend(value)
FW = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
SEAL = FW + "🏗️build/🏛️sealed/"

ext("🏛️sealed/🟦️.ts",
 emo={40:"🧱",148:"📏",46:"🪝",94:"🧲",167:"⚖",50:"🪪",96:"📄",56:"🛑",98:"⛔",85:"🗺",90:"📁",115:"🔓"},
 sub=[
  ('readonly maximumBytes:number;', 'readonly maximumBytes:number;readonly maximumTextCharacters:number;readonly maximumPathCharacters:number;readonly maximumCommandLineCharacters:number;'),
  ('const text=(value:unknown,maximum=256):value is string=>', 'const text=(value:unknown,maximum=sealedPolicyV1.maximumTextCharacters):value is string=>'),
 ],
 doc=[
  ("export type ImplementationModeV1", "🔀️ The implementation a command runs from: the sealed snapshot, the live tree, or a promotion of the live tree to a new snapshot."),
  ("export type SealedTextV1", "🌍️ One message of the policy in every declared language."),
  ("export interface SealedPolicyV1", "📐️ The closed policy of the store exactly as its schema declares it: modes, languages, bounds, timings, lock, check and the bilingual texts."),
 ])
ext("🏛️sealed/📤️promotion", emo={56:"🥾",49:"📍"}, doc=[
  ("export type SealedStatusV1", "🚥️ How a resolution ended: the snapshot was fresh, refreshed or promoted, a stored one was served, the promotion is blocked, the tree is unstable or the lock is busy."),
 ])
ext("🏛️sealed/🚦️trampoline", emo={9:"🏷"})
ext("🏛️sealed/🚪️entry", emo={18:"🏷"})
ext("🏛️sealed/🔁️promoter",
 sub=[
  ('import{sealedEntryOfV1}from"../🟦️.ts";', 'import{sealedEntryOfV1,sealedPolicyV1}from"../🟦️.ts";'),
  ('join(workspace,row.path).length>256', 'join(workspace,row.path).length>sealedPolicyV1.maximumPathCharacters'),
 ])
ext("🏛️sealed/📥️execution",
 sub=[
  ('sealedLanguagesV1,sealedTextV1}from"../🟦️.ts"', 'sealedLanguagesV1,sealedPolicyV1,sealedTextV1}from"../🟦️.ts"'),
  ('.length+1>32767', '.length+1>sealedPolicyV1.maximumCommandLineCharacters'),
 ])
ext("🏛️sealed/🔣️.json", sub=[
  ('  "maximumBytes": 536870912,\n', '  "maximumBytes": 536870912,\n  "maximumTextCharacters": 256,\n  "maximumPathCharacters": 256,\n  "maximumCommandLineCharacters": 32767,\n'),
 ])
ext("🏛️sealed/🧬️schema/🔣️.json", sub=[
  ('"maximumBytes", "settleMilliseconds"', '"maximumBytes", "maximumTextCharacters", "maximumPathCharacters", "maximumCommandLineCharacters", "settleMilliseconds"'),
  ('    "maximumBytes": { "type": "integer", "minimum": 67108864 },\n', '    "maximumBytes": { "type": "integer", "minimum": 67108864 },\n    "maximumTextCharacters": { "type": "integer", "minimum": 1 },\n    "maximumPathCharacters": { "type": "integer", "minimum": 1 },\n    "maximumCommandLineCharacters": { "type": "integer", "minimum": 1 },\n'),
 ])

ext("📜️journal/🟦️.ts",
 emo={15:"🏭"},
 sub=[
  ('throw Error("The change journal cannot vouch: "+reason);', 'throw Error("The change journal cannot vouch: "+reason+" / Der Änderungsjournal-Dienst kann nicht einstehen: "+reason);'),
  ('setTimeout(resolve,20)', 'setTimeout(resolve,policy.startPollMilliseconds)'),
 ])
ext("📜️changes/🟦️.ts", sub=[('startTimeoutMilliseconds:number;', 'startTimeoutMilliseconds:number;startPollMilliseconds:number;')])
ext("📜️changes/🧬️schema/🔣️.json", sub=[('"startTimeoutMilliseconds": 8000,', '"startTimeoutMilliseconds": 8000,\n        "startPollMilliseconds": 20,')])

ext("🗂️suites/🟦️.ts", emo={73:"🔒",88:"🛤",61:"🛂",131:"🥅",140:"🧮",122:"🧭",155:"🌡",169:"🛑"})
ext("🟢️node/🧲️native/🦀️.rs", emo={85:"🔣",95:"🎁",125:"🔓"})

ext("🔏️authorizers/🧬️schema/🔣️.json", sub=[
  ('        "maximumDeclaredBytes": 16777216\n      }\n    },\n', '        "maximumDeclaredBytes": 16777216\n      }\n    },\n    "MemoPolicy": {\n      "const": {\n        "maximumFileBytes": 4194304,\n        "concurrency": 24\n      }\n    },\n'),
 ])
ext("🧊️memo", sub=[
  ('import{dirname,join,resolve}from"node:path";', 'import{dirname,join,resolve}from"node:path";\nimport authorizerSchema from"../🧬️schema/🔣️.json"with{type:"json"};'),
  ('const MAXIMUM_FILE_BYTES=4_194_304n,CONCURRENCY=24;', 'const MEMO_POLICY=authorizerSchema.$defs.MemoPolicy.const,MAXIMUM_FILE_BYTES=BigInt(MEMO_POLICY.maximumFileBytes),CONCURRENCY=MEMO_POLICY.concurrency;'),
 ])

ext("🔁️incremental", sub=[
  ('import {isAbsolute,join,resolve} from "node:path";', 'import {isAbsolute,join,resolve} from "node:path";\nimport schema from "../🧬️schema/🔣️.json" with {type:"json"};\n\nconst retentionPolicy=schema.definitions.RetentionPolicy.const;'),
  ('export const TYPESCRIPT_CHECK_RETENTION_V1:TypeScriptCheckRetentionV1=Object.freeze({retainedPrograms:2,maximumHeapBytes:2*1024**3,idleMilliseconds:120000});', 'export const TYPESCRIPT_CHECK_RETENTION_V1:TypeScriptCheckRetentionV1=Object.freeze({retainedPrograms:retentionPolicy.retainedPrograms,maximumHeapBytes:retentionPolicy.maximumHeapBytes,idleMilliseconds:retentionPolicy.idleMilliseconds});'),
  ('limits.retainedPrograms>8)throw new Error("Retained TypeScript programs must be an integer from 1 to 8");', 'limits.retainedPrograms>retentionPolicy.maximumRetainedPrograms)throw new Error("Retained TypeScript programs must be an integer from 1 to "+retentionPolicy.maximumRetainedPrograms);'),
 ])
ext("🏗️build/🟦️typescript/🧬️schema/🔣️.json", sub=[
  ('    "CompilerEnvironment": {', '    "RetentionPolicy": {\n      "const": {\n        "retainedPrograms": 2,\n        "maximumRetainedPrograms": 8,\n        "maximumHeapBytes": 2147483648,\n        "idleMilliseconds": 120000\n      }\n    },\n    "CompilerEnvironment": {'),
 ])

ext("🎚️store-bound/🟦️.ts", sub=[('idlePollMs:number;intervalMs:number}>', 'idlePollMs:number;probePatienceMs:number;intervalMs:number}>')])
ext("🎚️store-bound/🔣️policy.json", sub=[('  "idlePollMs": 2000\n', '  "idlePollMs": 2000,\n  "probePatienceMs": 1000\n')])
ext("🎚️store-bound/🧬️schema/🔣️.json", sub=[
  ('    "idlePollMs"\n  ],', '    "idlePollMs",\n    "probePatienceMs"\n  ],'),
  ('    "idlePollMs": {\n      "type": "integer",\n      "minimum": 100,\n      "maximum": 60000\n    }\n', '    "idlePollMs": {\n      "type": "integer",\n      "minimum": 100,\n      "maximum": 60000\n    },\n    "probePatienceMs": {\n      "type": "integer",\n      "minimum": 0,\n      "maximum": 60000\n    }\n'),
 ])
ext("🏃️run/🔌️host/🟦️.ts", sub=[
  ('import {acquireResourceLease}', 'import {nxToolingStoreBoundPolicyV1} from "../../🟦️.ts";\nimport {acquireResourceLease}'),
  ('admissionPatienceMs:1000', 'admissionPatienceMs:nxToolingStoreBoundPolicyV1().probePatienceMs', 2),
 ])

MP = lambda key, en, de, vars_="": '    "machine.policy.%s": { "en": "Closed machine disk budget required: %s", "de": "Geschlossenes Datenträger-Budget der Maschine erforderlich: %s" },\n' % (key, en, de)
ext("ledger/📖️texts/🔣️.json", sub=[
  ('"de": "Das Budget von {total} liegt unter den Untergrenzen der Speicher ({floors})" },\n',
   '"de": "Das Budget von {total} liegt unter den Untergrenzen der Speicher ({floors})" },\n' + "".join([
    MP("schema", "{reason}", "{reason}"), MP("store", "duplicate store {id}", "doppelter Speicher {id}"),
    MP("storeKind", "exactly one of partition and loose in {id}", "genau eines von partition und loose in {id}"),
    MP("partition", "unknown or shared partition {partition}", "unbekannte oder geteilte Partition {partition}"),
    MP("target", "duplicate dedupe target {id}", "doppeltes Dedupe-Ziel {id}"),
    MP("targetShape", "dedupe target {id} needs a store partition and a canonical directory", "das Dedupe-Ziel {id} braucht eine Speicher-Partition und ein kanonisches Verzeichnis"),
    MP("weights", "weights add up to {weights}, not 10000", "die Gewichte ergeben {weights}, nicht 10000"),
    MP("loose", "at most one loose store", "höchstens ein loser Speicher"),
    MP("orphan", "partition {partition} belongs to no store", "die Partition {partition} gehört zu keinem Speicher"),
    MP("floors", "floors exceed the default total", "die Untergrenzen übersteigen die Standardsumme"),
   ])),
 ])
ext("🎚️machine", sub=[
  ('throw Error("Closed machine disk budget required: "+errors[0])', 'refuse("machine.policy.schema",{reason:String(errors[0])})'),
  ('throw Error("Closed machine disk budget required: duplicate store "+store.id)', 'refuse("machine.policy.store",{id:store.id})'),
  ('throw Error("Closed machine disk budget required: exactly one of partition and loose in "+store.id)', 'refuse("machine.policy.storeKind",{id:store.id})'),
  ('throw Error("Closed machine disk budget required: unknown or shared partition "+store.partition)', 'refuse("machine.policy.partition",{partition:store.partition})'),
  ('throw Error("Closed machine disk budget required: duplicate dedupe target "+target.id)', 'refuse("machine.policy.target",{id:target.id})'),
  ('throw Error("Closed machine disk budget required: dedupe target "+target.id+" needs a store partition and a canonical directory")', 'refuse("machine.policy.targetShape",{id:target.id})'),
  ('throw Error("Closed machine disk budget required: weights add up to "+weights+", not 10000")', 'refuse("machine.policy.weights",{weights})'),
  ('throw Error("Closed machine disk budget required: at most one loose store")', 'refuse("machine.policy.loose")'),
  ('throw Error("Closed machine disk budget required: partition "+partition+" belongs to no store")', 'refuse("machine.policy.orphan",{partition})'),
  ('throw Error("Closed machine disk budget required: floors exceed the default total")', 'refuse("machine.policy.floors")'),
 ])

ext("🗃️unit-cache/🧬️schema/🔣️.json", sub=[
  ('"pollMaximumMilliseconds": 200, "maximumWaitMilliseconds": 1800000', '"pollMaximumMilliseconds": 200, "pollBackoffBasisPoints": 15000, "maximumWaitMilliseconds": 1800000'),
  ('"pollMaximumMilliseconds": 1000, "listedCrates": 16', '"pollMaximumMilliseconds": 1000, "pollBackoffBasisPoints": 15000, "listedCrates": 16'),
 ])
ext("🗃️unit-cache/🔒️lock/🟦️.ts", sub=[('Math.ceil(poll*1.5)', 'Math.ceil(poll*policy.lock.pollBackoffBasisPoints/10000)')])
ext("🏊️pool", sub=[('Math.ceil(poll*1.5)', 'Math.ceil(poll*rules.pollBackoffBasisPoints/10000)')])

FW2 = FW + "🏗️build/🕸️graph/🕰️generations/🌿️frontier-walk/"
ext("🌿️frontier-walk/🟨️.mjs",
 sub=[
  ('export const FRONTIER_WALK_POLICY_V1=Object.freeze({concurrency:64,racyMilliseconds:2000,rootDirectory:"."});', 'import schema from"./🧬️schema/🔣️.json"with{type:"json"};\n\nexport const FRONTIER_WALK_POLICY_V1=Object.freeze(structuredClone(schema.definitions.Policy.const));'),
 ],
 doc=[("export const FRONTIER_WALK_POLICY_V1", "🪜️ The declared limits of a walk: how many directories are listed at once, how recent a stamp must be to count as racy, and the name of the root directory; the schema file is the single source.")])
NEW = {FW2 + "🧬️schema/🔣️.json": '''{
 "$schema": "https://json-schema.org/draft/2020-12/schema",
 "title": "Walk and advance of the membership frontier",
 "description": "The limits of the frontier walk: how many directories are listed concurrently, the age below which an entry stamp is too recent to trust, and the name of the root directory.",
 "definitions": {
  "Policy": {
   "const": {
    "concurrency": 64,
    "racyMilliseconds": 2000,
    "rootDirectory": "."
   }
  }
 }
}
'''}

ext("🏗️build/🔣️policy.json", sub=[('      "lendAfterMilliseconds": 5000\n', '      "lendAfterMilliseconds": 5000,\n      "jitterBasisPoints": 2500\n')])
ext("🌡️host-capacity/🧬️schema/🔣️.json", sub=[
  ('            "agingMilliseconds",\n            "lendAfterMilliseconds"\n          ],', '            "agingMilliseconds",\n            "lendAfterMilliseconds",\n            "jitterBasisPoints"\n          ],'),
  ('            "lendAfterMilliseconds": {\n              "type": "integer",\n              "minimum": 1000,\n              "maximum": 3600000\n            }\n', '            "lendAfterMilliseconds": {\n              "type": "integer",\n              "minimum": 1000,\n              "maximum": 3600000\n            },\n            "jitterBasisPoints": {\n              "type": "integer",\n              "minimum": 0,\n              "maximum": 5000\n            }\n'),
 ])
ext("🌡️host-capacity/📏️decision/🟦️.ts", sub=[('agingMilliseconds:number;lendAfterMilliseconds:number}', 'agingMilliseconds:number;lendAfterMilliseconds:number;jitterBasisPoints:number}')])
ext("🌡️host-capacity/🧪️tests/🧫️fixtures/🔣️.json", sub=[('"lendAfterMilliseconds":30000}', '"lendAfterMilliseconds":30000,"jitterBasisPoints":2500}')])
ext("🌡️host-capacity/🔼️upgrade/🟦️.ts", sub=[('policy.wait.pollMilliseconds*(0.75+Math.random()*0.5)', 'policy.wait.pollMilliseconds*(1-policy.wait.jitterBasisPoints/10000+Math.random()*2*policy.wait.jitterBasisPoints/10000)')])
