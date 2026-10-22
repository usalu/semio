/** 🎯️ The attempt selection and lease handling of the taxonomy plan apply. */
import { normalizeRelative } from "../../🛣️path/🟦️.ts";
import { sha256 } from "../../📁️input/🟦️.ts";

import { canonicalJson } from "../../../🧾️serialization/🔣️json/🟦️.ts";
import { posix } from "node:path";
import { type TaxonomyApplyOptions, requireTaxonomyValue } from "../../📜️plan-contracts/🟦️.ts";
import { absolutePath, checkCancellation, resolveFileKind } from "../../🏷️file-kind-matching/🟦️.ts";
import { type MutableJournalRecord } from "../../💾️durable-journal/🟦️.ts";
import { logicalRepositorySymlinkTargetPath } from "../../➰️symlink-targets/🟦️.ts";
import { canonicalDirectoryName } from "../../🧐️preimage-assertions/🟦️.ts";
import { type TransactionLeaseHandle, type TransactionLeaseRecord, releaseTransactionLease } from "../../🔐️transaction-lease/🟦️.ts";
import { acquireTransactionLease } from "../../🔑️transaction-lease-acquisition/🟦️.ts";
import { type PreflightReferenceBasis } from "../../🛫️embedded-roots/🟦️.ts";
import type { bindTransactionNames, checkPlanAuthority } from "../📜️plan-checks/🟦️.ts";
import { runToolSyncV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";

/** 🧭 Selects the resumed attempt or allocates the next attempt ordinal and binds its journal paths. */
export function selectAttempt({ existingAttempts, resumeRelative, digest, taxonomy, attemptsRelative, journalFilename, repoRoot }: Pick<ReturnType<typeof checkPlanAuthority>, "existingAttempts" | "resumeRelative"> & Pick<ReturnType<typeof bindTransactionNames>, "digest" | "taxonomy" | "attemptsRelative" | "journalFilename" | "repoRoot">) {
  const txn = {} as { attemptOrdinal: string; journalRelative: string; selectedAttempt: (typeof existingAttempts)[number] | undefined; leaseHandle: TransactionLeaseHandle | undefined; preflightReferenceBasis: PreflightReferenceBasis | undefined; lockedResumeJournal: MutableJournalRecord | undefined };
  if (resumeRelative) {
    const match = existingAttempts.find((entry) => entry.journalRelative === resumeRelative);
    if (!match) throw new Error(`Resume journal is not an exact existing canonical attempt for plan ${digest}`);
    if (match.journal.state === "rolled-back") throw new Error("Cannot resume journal in state rolled-back");
    txn.selectedAttempt = match;
    txn.attemptOrdinal = match.ordinal;
    txn.journalRelative = match.journalRelative;
  } else {
    const active = existingAttempts.find((entry) => entry.journal.state !== "rolled-back" && entry.journal.state !== "committed");
    if (active) throw new Error(`Transaction attempt ${active.ordinal} is active and must be resumed`);
    if (existingAttempts.some((entry) => entry.journal.state === "committed")) throw new Error("Plan already has a committed transaction attempt");
    const next = existingAttempts.length === 0 ? 1 : Math.max(...existingAttempts.map((entry) => Number.parseInt(entry.ordinal, 10))) + 1;
    if (next > 999999) throw new Error("Transaction attempt ordinal space is exhausted");
    txn.attemptOrdinal = String(next).padStart(6, "0");
    const attemptDirectory = canonicalDirectoryName(taxonomy, "transaction-attempt", txn.attemptOrdinal, "transaction-attempts");
    txn.journalRelative = normalizeRelative(`${attemptsRelative}/${attemptDirectory}/${journalFilename}`);
  }
  const attemptRelative = posix.dirname(txn.journalRelative);
  const journalPath = absolutePath(repoRoot, txn.journalRelative);
  return { txn, attemptRelative, journalPath };
}

/** 🔑 Defines the lease acquisition and release and checks the symlink targets, baseline commit and cancellation. */
export function defineLeaseAndPreChecks({ txn, repositoryAuthority, attemptRelative, leaseDirectory, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, transactionProbe, plan, repoRoot, taxonomy, options }: Pick<ReturnType<typeof selectAttempt>, "txn" | "attemptRelative"> & Pick<ReturnType<typeof bindTransactionNames>, "repositoryAuthority" | "leaseDirectory" | "leasePreparationName" | "leaseJsonWritePreparationName" | "journalFilename" | "jsonPreviousName" | "digest" | "transactionProbe" | "plan" | "repoRoot" | "taxonomy"> & { readonly options: TaxonomyApplyOptions }) {
  const acquireLease = (backupRoot: string, beforePublish?: (owned?: TransactionLeaseRecord) => void): void => {
    txn.leaseHandle = acquireTransactionLease(repositoryAuthority, attemptRelative, backupRoot, leaseDirectory, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, txn.attemptOrdinal, beforePublish, transactionProbe);
  };
  const releaseLease = (): void => {
    if (!txn.leaseHandle) return;
    releaseTransactionLease(repositoryAuthority, `${attemptRelative}/${leaseDirectory}`, txn.leaseHandle);
    txn.leaseHandle = undefined;
  };
  for (const edit of plan.symlinkTargetEdits) {
    const localTarget = logicalRepositorySymlinkTargetPath(repoRoot, edit.sourcePath, edit.oldTarget);
    if (localTarget !== edit.logicalTargetSourcePath) throw new Error(`Symlink target authority is not repository-local or does not match its logical source: ${edit.sourcePath}`);
    const linkMoves = plan.moves.filter((move) => move.sourcePath === edit.sourcePath);
    const targetMoves = plan.moves.filter((move) => move.sourcePath === edit.logicalTargetSourcePath);
    const expectedFinalPath = linkMoves.length === 0 ? edit.sourcePath : linkMoves.length === 1 ? requireTaxonomyValue(linkMoves[0],"linkMoves[0]").destinationPath : "";
    const expectedTargetFinalPath = targetMoves.length === 0 ? edit.logicalTargetSourcePath : targetMoves.length === 1 ? requireTaxonomyValue(targetMoves[0],"targetMoves[0]").destinationPath : "";
    if (edit.finalPath !== expectedFinalPath || edit.logicalTargetFinalPath !== expectedTargetFinalPath) throw new Error(`Symlink target projection does not match exact plan moves: ${edit.sourcePath}`);
    const expectedTarget = posix.relative(posix.dirname(expectedFinalPath), expectedTargetFinalPath);
    if (!expectedTarget || expectedTarget !== edit.newTarget || expectedTarget.startsWith("/") || posix.normalize(posix.join(posix.dirname(expectedFinalPath), expectedTarget)) !== expectedTargetFinalPath) throw new Error(`Symlink relative target does not resolve to its frozen logical target: ${edit.sourcePath}`);
    if (edit.logicalTargetPreimage.state === "directory" || edit.windowsLinkType !== "file") throw new Error(`Symlink directory target lacks recursive no-follow authority: ${edit.sourcePath}`);
    if (edit.logicalTargetPreimage.state === "absent" && !resolveFileKind(edit.logicalTargetSourcePath, taxonomy, undefined, []).kind) throw new Error(`Broken symlink target kind cannot be proven: ${edit.sourcePath}`);
    const targetDigestible = { sourcePath: edit.sourcePath, finalPath: edit.finalPath, oldTarget: edit.oldTarget, newTarget: edit.newTarget, logicalTargetSourcePath: edit.logicalTargetSourcePath, logicalTargetFinalPath: edit.logicalTargetFinalPath, logicalTargetPreimage: edit.logicalTargetPreimage };
    if (edit.sourceTargetDigest !== sha256(canonicalJson(targetDigestible))) throw new Error(`Symlink source-target authority digest changed: ${edit.sourcePath}`);
  }
  runToolSyncV1("git", ["cat-file", "-e", `${plan.baselineCommit}^{commit}`], { check: true, cwd: repoRoot, stdio: "ignore" }).stdout;
  if (plan.excludedTreeDigests.length > 0) throw new Error("Opaque digest filesystem access is disabled; replan with empty excludedTreeDigests");
  if (!options.resumeJournal) checkCancellation(repoRoot, options.cancelFile);
  return { acquireLease, releaseLease };
}
