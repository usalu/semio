/** 📜️ The boundary contracts of the transaction-v2 suite: fixture and snapshot types, goldens and phase tables. */
import { type ChildProcess } from "node:child_process";
import { appendFileSync, readFileSync } from "node:fs";
import { resolve } from "node:path";
import { getWorkspaceRoot } from "../../../🗂️workspaces/🟦️.ts";
import { type TaxonomyInventoryOptions, type TaxonomyPlan } from "../../../🧹️normalization/📜️plan-contracts/🟦️.ts";

export type Fixture = Readonly<{ baselineCommit: string; options: TaxonomyInventoryOptions; planCacheKey?: string; repoRoot: string; root: string; scope: string; ticketDir: string; workspace: string }>;

export type Snapshot = Readonly<Record<string, string>>;

export const TICKET_REL = ".🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️09/☀️01/KIND-ONLY-BASENAMES-ACROSS-THE-TAXONOMY-TREE";

export const SCHEMA_REL = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🔣️taxonomy.json";

export const TRANSACTION_LEDGER_BOUNDARIES_GOLDEN = resolve(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/📒️transaction-ledger-boundaries/🔣️.json");

export const TRANSACTION_PROTOCOL_GOLDEN = resolve(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧪️tests/🧫️fixtures/🤝️transaction-protocol/🔣️.json");

export const NORMALIZATION_MODULE = process.env.SEMIO_TRANSACTION_V2_MODULE ?? resolve(getWorkspaceRoot(), "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🧹️normalization/🚀️apply/🟦️.ts");

export const FIXTURE_SCHEMA = process.env.SEMIO_TRANSACTION_V2_SCHEMA ?? resolve(getWorkspaceRoot(), SCHEMA_REL);

export const FIXTURE_RUN_ID = process.env.SEMIO_TRANSACTION_V2_RUN_ID ?? `${process.pid}-${crypto.randomUUID()}`;

process.env.NX_DAEMON = "false";

export type BoundaryNode = Readonly<{ path: string; kind: "directory"; mode: string } | { path: string; kind: "symlink"; mode: string; target: string; targetHash: string } | { path: string; kind: "file"; mode: string; size: number; sha256: string; bytesBase64: string; canonicalJson?: boolean; json?: unknown }>;

type BoundaryGolden = Readonly<{ schemaVersion: 1; boundaries: Readonly<Record<string, Readonly<{ transaction: string; workspace: string }>>>; transactionLedgers: Readonly<Record<string, readonly BoundaryNode[]>>; workspaceLedgers: Readonly<Record<string, readonly BoundaryNode[]>> }>;

export type ProtocolGolden = Readonly<{ schemaVersion: 1; failureStages: readonly string[]; journalStates: readonly string[]; virtualPreimageNodes: readonly { path: string; state: string }[] }>;

export const transactionGolden = JSON.parse(readFileSync(TRANSACTION_LEDGER_BOUNDARIES_GOLDEN, "utf8")) as BoundaryGolden;

export const FAILURE_STAGES = ["after-staging", "after-embedded-root-staging", "after-moves", "after-relocations", "after-symlink-retargeting", "after-edits", "after-regenerations", "before-verify"] as const;

export const KILL_PHASES = ["transaction-attempt-preparation-mkdir", "transaction-attempt-preparation-children", "transaction-initial-lease-json-write-mkdir", "transaction-initial-lease-json-candidate-written", "transaction-initial-lease-json-canonical-exchanged", "transaction-initial-lease-prepared", "transaction-initial-wal-mkdir", "transaction-initial-journal-write-mkdir", "transaction-initial-journal-candidate-written", "transaction-initial-journal-canonical-exchanged", "transaction-initial-journal-canonical", "transaction-attempt-canonical-published", "transaction-journal-write-mkdir", "transaction-journal-candidate-written", "transaction-journal-previous-exchanged", "transaction-journal-canonical-exchanged", "transaction-wal-prepared", "transaction-backup-write-mkdir", "transaction-backup-write-mid", "transaction-backup-write-prepared", "transaction-backup-inner-exchange", "transaction-backup-exchange", "transaction-backup-retained", "transaction-edit-write-mkdir", "transaction-edit-write-mid", "transaction-edit-write-prepared", "transaction-edit-inner-exchange", "transaction-edit-exchange", "transaction-edit-canonical-exchange"] as const;

export const RESTORE_PHASES = ["transaction-restore-mkdir", "transaction-restore-prepared", "transaction-restore-exchange", "transaction-restore-canonical-exchange"] as const;

export const LEASE_PHASES = ["transaction-lease-stale-quarantined", "transaction-lease-preparation-mkdir", "transaction-lease-json-write-mkdir", "transaction-lease-json-candidate-written", "transaction-lease-json-canonical-exchanged", "transaction-lease-prepared", "transaction-lease-canonical-published"] as const;

export const LATE_KILL_PHASES = new Set<string>(KILL_PHASES.slice(12));

export const EXACT_BOUNDARY_KEYS = [
  ...FAILURE_STAGES.map((stage) => `rolledback:${stage}`),
  ...[...KILL_PHASES, ...RESTORE_PHASES, ...LEASE_PHASES].flatMap((phase) => [`killed:${phase}`, `recovered:${phase}`]),
  "killed:transaction-terminal-committed-stage-removed", "recovered:transaction-terminal-committed-stage-removed",
  "killed:transaction-terminal-rolled-back-stage-removed", "recovered:transaction-terminal-rolled-back-stage-removed",
  "killed:process-tree-mixed-generator", "recovered:process-tree-mixed-generator", "committed:process-tree-mixed-generator",
  "rolledback:cancellation", "rolledback:caught-attempt-canonical-published", "rolledback:caught-journal-previous-exchanged",
].sort();

export const fixtureTemplates = new Map<string, Readonly<{ baselineCommit: string; processOwned: boolean; root: string }>>();

export const fixturePlans = new Map<string, TaxonomyPlan>();

export const childEvidence = new WeakMap<ChildProcess, { output: string; error?: Error }>();

export function registerChild(childProcess: ChildProcess): ChildProcess {
  const evidence: { output: string; error?: Error } = { output: "" };
  childEvidence.set(childProcess, evidence);
  for (const stream of [childProcess.stdout, childProcess.stderr]) stream?.on("data", (bytes) => { evidence.output = (evidence.output + String(bytes)).slice(-32_768); });
  childProcess.once("error", (error) => { evidence.error = error; });
  const registry = process.env.SEMIO_TRANSACTION_V2_PID_REGISTRY;
  if (registry && childProcess.pid) appendFileSync(registry, `${childProcess.pid}\n`);
  return childProcess;
}
