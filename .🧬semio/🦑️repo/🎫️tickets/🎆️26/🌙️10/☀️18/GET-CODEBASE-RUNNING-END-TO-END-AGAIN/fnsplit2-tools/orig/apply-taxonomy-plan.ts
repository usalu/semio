import { report } from "../🏃️operation/🟦️.ts";
import { loadTaxonomy } from "../🔣️taxonomy/🟦️.ts";
import { normalizeRelative, splitLeadingEmoji, sourceRelative } from "../🛣️path/🟦️.ts";
import { sha256, assertLexicalInputOutsideOpaque, lstatOrNull, assertNoFollowAncestors } from "../📁️input/🟦️.ts";
import { execFileSync } from "node:child_process";
import { randomUUID } from "node:crypto";
import { lstatSync, mkdirSync, readFileSync, readdirSync, readlinkSync } from "node:fs";
import { canonicalJson } from "../../🧾️serialization/🔣️json/🟦️.ts";
import { basename, dirname, join, posix, relative, resolve } from "node:path";
import { PLAN_COMMIT_ID, PLAN_HASH, type ReferenceEdit, type TaxonomyApplyOptions, type TaxonomyApplyResult, type TaxonomyPlan, requireTaxonomyValue } from "../📜️plan-contracts/🟦️.ts";
import { absolutePath, cancellationRequested, checkCancellation, isExcluded, resolveFileKind } from "../🏷️file-kind-matching/🟦️.ts";
import { generatorPathCompare } from "../🧾️plan-parsing/🟦️.ts";
import { JOURNAL_OPERATION_ARRAYS, type MutableJournalRecord, durableRemove, durableRename, durableSymlink, durablySyncGeneratorRecords, fsyncDirectory, fsyncFile, journalSnapshot, persistJournal, publishCanonicalJsonCandidate, readJournal } from "../💾️durable-journal/🟦️.ts";
import { type TaxonomyBackupExpectedPreimage, TaxonomyGeneratorInputDriftError, TaxonomyMoveSourceInputDriftError, TaxonomyStartedRegenerationPartialError, assertJournalBackupAuthority, assertJournalPhaseMembership, assertJournalPlanMembership } from "../🩹️recovery-authorities/🟦️.ts";
import { generatorNodeRecord, generatorTreeInventory } from "../🏭️generator-inputs/🟦️.ts";
import { incomingEmbeddedReferences, logicalRepositorySymlinkTargetPath } from "../➰️symlink-targets/🟦️.ts";
import { assertDirectoryOnlyTree, assertLeafPreimage, assertPlanOutsideTransaction, assertTicketImportantExactRemovalAuthority, assertTicketImportantRemovalAuthority, assertWritableAncestors, canonicalDirectoryName, canonicalKindOnlyFilename, canonicalScopedKindOnlyFilename, leafPathPreimage, leafPreimage, pathsOverlap, removalAuthorityPaths, removalIncomingIgnoredSourceRoots, retargetedMovePreimage } from "../🧐️preimage-assertions/🟦️.ts";
import { assertTransactionRepositoryPath, assertTransactionRepositoryWitness, captureTransactionRepositoryAuthority, isTransactionRepositoryAuthorityError, transactionRepositoryBootstrapPaths, transactionRepositoryFinally } from "../🏛️repository-authority/🟦️.ts";
import { noFollowTreeDigest, transactionTreeDigest } from "../🌳️tree-digests/🟦️.ts";
import { emptySourceParents, injectFailure, inventoryAfterSourceParentPruning, reconcileRollbackTuples } from "../⏪️rollback-reconciliation/🟦️.ts";
import { TRANSACTION_LEASE_TOKEN, type TransactionLeaseHandle, type TransactionLeaseRecord, parseTransactionLease, readTransactionLease, releaseTransactionLease, transactionLeaseProcessIsAlive } from "../🔐️transaction-lease/🟦️.ts";
import { applyReferenceEditAtomically, assertRecoveryRootNames, backupPath, reconcileTransactionOwnedTuples, recoverReferenceEditPreparations, recoverRestorePreparations, recoverTransactionBackups, validateResumeTuples } from "../🛟️recovery/🟦️.ts";
import { assertActiveTransactionEvidence, cleanupCommittedTransaction } from "../🚿️transaction-cleanup/🟦️.ts";
import { reconcileJournalWal, recoverCanonicalJsonCandidates } from "../🔄️journal-reconciliation/🟦️.ts";
import { acquireTransactionLease } from "../🔑️transaction-lease-acquisition/🟦️.ts";
import { actualAffectedDigest, actualAffectedPreDigest, taxonomyPlanDigest, validateLeasePreparationEvidence } from "../🗃️affected-state/🟦️.ts";
import { parseTaxonomyPlan } from "../📃️plan-document/🟦️.ts";
import { artifactProjectionPostApplyViolations, projectionStaleViolations } from "../♻️projection-staleness/🟦️.ts";
import { cleanupRolledBackTransaction, rollbackTransaction } from "../↩️rollback-transaction/🟦️.ts";
import { removalReferenceProjection } from "../🗑️removal-references/🟦️.ts";
import { type PreflightReferenceBasis, capturePreflightReferenceBasis, embeddedTargetPaths, evidenceRemovalIncomingReferences, lexicalEmbeddedIncomingReferences, lexicalTargetIncomingReferences, validatePreflightReferenceBasis } from "../🛫️embedded-roots/🟦️.ts";
import { inventoryTaxonomy, inventoryTaxonomyWithSourceParentPruning, inventoryWithoutTransactionEvidence } from "../🔍️inventory-planning/🟦️.ts";
import { planTaxonomy } from "../🧠️planning/🟦️.ts";

/** 🛠️ Applies a digest-verified plan through two-phase staging, journaled backups, verification, cancellation, resume and rollback. */
export function applyTaxonomyPlan(plan: TaxonomyPlan, options: TaxonomyApplyOptions): TaxonomyApplyResult {
  plan = parseTaxonomyPlan(plan);
  const repoRoot = resolve(options.repoRoot);
  if (!PLAN_COMMIT_ID.test(options.expectedBaselineCommit) || options.expectedBaselineCommit !== plan.baselineCommit) throw new Error("Plan baselineCommit does not match expectedBaselineCommit authority");
  if (options.workers !== undefined && (!Number.isSafeInteger(options.workers) || options.workers < 1)) throw new Error("workers must be a positive integer");
  const digest = taxonomyPlanDigest(plan);
  if (plan.planDigest !== digest) throw new Error("Plan digest does not match canonical plan bytes");
  if (options.expectedPlanDigest !== undefined && options.expectedPlanDigest !== digest) throw new Error("Plan digest does not match expectedPlanDigest");
  if (plan.unresolved.some((entry) => entry.severity === "error")) throw new Error("Plan has unresolved blocking violations");
  const bootstrap = transactionRepositoryBootstrapPaths(plan, options, repoRoot);
  const repositoryAuthority = captureTransactionRepositoryAuthority(repoRoot);
  for (const access of bootstrap.accesses) assertTransactionRepositoryPath(repositoryAuthority, access.path, access.role, access.label);
  const taxonomy = loadTaxonomy({ repoRoot, taxonomyPath: absolutePath(repoRoot, bootstrap.taxonomyPath) });
  const transactionProbe = (phase: string, path?: string): void => report(options.progress, "apply", phase, 0, 0, path);
  const ticketRelative = bootstrap.ticketDir;
  if (isExcluded(ticketRelative, taxonomy)) throw new Error(`Ticket directory is opaque: ${ticketRelative}`);
  const ticketRoot = absolutePath(repoRoot, ticketRelative);
  const transactionDirectory = canonicalDirectoryName(taxonomy, "taxonomy-transaction", "taxonomy-transaction");
  const digestDirectory = canonicalDirectoryName(taxonomy, "transaction-digest", digest, "taxonomy-transaction");
  const transactionRootRelative = normalizeRelative(`${ticketRelative}/${transactionDirectory}`);
  assertTransactionRepositoryPath(repositoryAuthority, transactionRootRelative, "subtree", "Taxonomy transaction executor");
  const transactionRelative = normalizeRelative(`${transactionRootRelative}/${digestDirectory}`);
  const attemptsDirectory = canonicalDirectoryName(taxonomy, "transaction-attempts", "attempts", "transaction-digest");
  const stageDirectory = canonicalDirectoryName(taxonomy, "transaction-stage", "stage", "transaction-attempt");
  const backupDirectory = canonicalDirectoryName(taxonomy, "transaction-backup", "backup", "transaction-attempt");
  const leaseDirectory = canonicalDirectoryName(taxonomy, "transaction-lease", "lease", "transaction-attempt");
  const journalWriteDirectory = canonicalDirectoryName(taxonomy, "transaction-journal-write", "journal", "transaction-stage");
  const attemptPreparationName = (ordinal: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-attempt-preparation", `prepare-${ordinal}-${pid}-${token}`, "transaction-attempts");
  const leasePreparationName = (pid: number, token: string, state: "preparing" | "stale"): string => canonicalDirectoryName(taxonomy, "transaction-lease-preparation", `lease-${pid}-${token}-${state}`, "transaction-backup");
  const journalJsonWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-json-write-preparation", `write-${pid}-${token}`, "transaction-journal-write");
  const leaseJsonWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-json-write-preparation", `write-${pid}-${token}`, "transaction-lease-preparation");
  const backupPreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-backup-preparation", `backup-${identity}-${pid}-${token}`, "transaction-backup");
  const backupWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-backup-write-preparation", `write-${pid}-${token}`, "transaction-backup-preparation");
  const restorePreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-restore-preparation", `restore-${identity}-${pid}-${token}`, "transaction-backup");
  const editPreparationName = (identity: string, pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-edit-preparation", `edit-${identity}-${pid}-${token}`, "transaction-stage");
  const editWritePreparationName = (pid: number, token: string): string => canonicalDirectoryName(taxonomy, "transaction-edit-write-preparation", `write-${pid}-${token}`, "transaction-edit-preparation");
  const attemptsRelative = normalizeRelative(`${transactionRelative}/${attemptsDirectory}`);
  const journalFilename = canonicalKindOnlyFilename(taxonomy, "json", ".json");
  const jsonPreviousName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-json-previous", "transaction-json-write-preparation", ".json");
  const backupWriteCandidateName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-backup-write-candidate", "transaction-backup-write-preparation", ".backup");
  const editWriteCandidateName = canonicalScopedKindOnlyFilename(taxonomy, "transaction-edit-write-candidate", "transaction-edit-write-preparation", ".edit");
  const planBytes = Buffer.from(`${canonicalJson(plan)}\n`);
  const planAuthority = (() => {
    const candidateRelative = bootstrap.planArtifactPath;
    if (options.planArtifactPath) assertLexicalInputOutsideOpaque(repoRoot, options.planArtifactPath, "planArtifactPath", true);
    const candidate = absolutePath(repoRoot, candidateRelative);
    if (!options.planArtifactPath) assertNoFollowAncestors(repoRoot, candidate, "canonical plan artifact", true);
    const stat = lstatOrNull(candidate);
    if (options.planArtifactPath && (!stat?.isFile() || stat.isSymbolicLink() || !readFileSync(candidate).equals(planBytes))) throw new Error("planArtifactPath must be a regular no-follow file containing the exact canonical plan bytes");
    return stat?.isFile() && !stat.isSymbolicLink() && readFileSync(candidate).equals(planBytes) ? { path: candidateRelative, bytes: planBytes } : undefined;
  })();
  const resumeRelative = bootstrap.resumeJournal;
  assertPlanOutsideTransaction(plan, transactionRootRelative, taxonomy, repoRoot);
  if (options.cancelFile) {
    const cancelAbsolute = assertLexicalInputOutsideOpaque(repoRoot, options.cancelFile, "cancelFile", true);
    const cancelRelative = normalizeRelative(relative(repoRoot, cancelAbsolute).replaceAll("\\", "/"));
    const mutationPaths = [
      ...plan.moves.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.embeddedTicketRoots.flatMap((entry) => [entry.sourceMetadataRoot, entry.sourceTicketRoot, entry.canonicalTicketRoot]),
      ...plan.embeddedTicketRootRelocations.flatMap((entry) => [entry.sourcePath, entry.destinationPath]),
      ...plan.evidenceRemovals.flatMap((entry) => [entry.sourcePath, ...removalAuthorityPaths(entry.authority)]),
      ...plan.symlinkTargetEdits.flatMap((entry) => [entry.sourcePath, entry.finalPath, entry.logicalTargetSourcePath, entry.logicalTargetFinalPath]),
      ...plan.edits.map((entry) => entry.path),
      ...plan.regenerations.flatMap((entry) => [entry.cwd, ...entry.outputRoots, ...entry.inputs.map((input) => input.path), ...entry.preOutputs.map((output) => output.path), ...entry.outputs.map((output) => output.path), ...entry.staleRemovals]),
    ];
    if (pathsOverlap(cancelRelative, transactionRootRelative) || mutationPaths.some((path) => pathsOverlap(cancelRelative, path))) throw new Error(`cancelFile overlaps transaction or mutation authority: ${cancelRelative}`);
  }
  const existingAttempts: { readonly ordinal: string; readonly attemptRelative: string; readonly journal: MutableJournalRecord; readonly journalRelative: string }[] = [];
  const unpublishedAttempts: { readonly ordinal: string; readonly pid: number; readonly token: string; readonly path: string }[] = [];
  const attemptsAbsolute = absolutePath(repoRoot, attemptsRelative);
  assertNoFollowAncestors(repoRoot, attemptsAbsolute, "transaction attempts root", true);
  const transactionRootAbsolute = absolutePath(repoRoot, transactionRootRelative);
  const transactionRootStat = lstatOrNull(transactionRootAbsolute);
  if (transactionRootStat) {
    if (!transactionRootStat.isDirectory() || transactionRootStat.isSymbolicLink()) throw new Error("Taxonomy transaction root must be a no-follow directory");
    for (const name of readdirSync(transactionRootAbsolute).sort(generatorPathCompare)) {
      const childDigest = splitLeadingEmoji(name).rest;
      if (!PLAN_HASH.test(childDigest) || name !== canonicalDirectoryName(taxonomy, "transaction-digest", childDigest, "taxonomy-transaction")) throw new Error(`Unexpected taxonomy transaction-root entry: ${name}`);
      const stat = lstatOrNull(join(transactionRootAbsolute, name));
      if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction digest must be a no-follow directory: ${name}`);
    }
  }
  const digestAbsolute = absolutePath(repoRoot, transactionRelative);
  const digestStat = lstatOrNull(digestAbsolute);
  if (digestStat) {
    if (!digestStat.isDirectory() || digestStat.isSymbolicLink()) throw new Error("Selected transaction digest must be a no-follow directory");
    const digestChildren = readdirSync(digestAbsolute).sort(generatorPathCompare);
    if (digestChildren.some((name) => name !== attemptsDirectory)) throw new Error("Selected transaction digest contains an unexpected artifact");
  }
  const attemptsStat = lstatOrNull(attemptsAbsolute);
  if (attemptsStat) {
    if (!attemptsStat.isDirectory() || attemptsStat.isSymbolicLink()) throw new Error("Transaction attempts authority must be a no-follow directory");
    for (const name of readdirSync(attemptsAbsolute).sort(generatorPathCompare)) {
      const childSlug = splitLeadingEmoji(name).rest;
      const preparation = /^prepare-([0-9]{6})-([1-9][0-9]*)-([0-9a-f-]+)$/u.exec(childSlug);
      if (preparation) {
        const ordinal = requireTaxonomyValue(preparation[1],"preparation[1]"), pid = Number.parseInt(requireTaxonomyValue(preparation[2],"preparation[2]"), 10), token = requireTaxonomyValue(preparation[3],"preparation[3]");
        if (!Number.isSafeInteger(pid) || !TRANSACTION_LEASE_TOKEN.test(token) || name !== attemptPreparationName(ordinal, pid, token)) throw new Error(`Unexpected transaction attempt preparation: ${name}`);
        const path = join(attemptsAbsolute, name);
        const stat = lstatOrNull(path);
        if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Transaction attempt preparation must be a no-follow directory: ${name}`);
        unpublishedAttempts.push({ ordinal, pid, token, path });
        continue;
      }
      const ordinal = childSlug;
      if (!/^[0-9]{6}$/u.test(ordinal) || name !== canonicalDirectoryName(taxonomy, "transaction-attempt", ordinal, "transaction-attempts")) throw new Error(`Unexpected transaction attempt entry: ${name}`);
      const attemptRelative = normalizeRelative(`${attemptsRelative}/${name}`);
      const attemptAbsolute = absolutePath(repoRoot, attemptRelative);
      const attemptStat = lstatOrNull(attemptAbsolute);
      if (!attemptStat?.isDirectory() || attemptStat.isSymbolicLink()) throw new Error(`Transaction attempt must be a no-follow directory: ${attemptRelative}`);
      const childNames = readdirSync(attemptAbsolute).sort(generatorPathCompare);
      if (childNames.some((child) => child !== journalFilename && child !== stageDirectory && child !== backupDirectory && child !== leaseDirectory)) throw new Error(`Transaction attempt contains an unexpected artifact: ${attemptRelative}`);
      const attemptJournalRelative = normalizeRelative(`${attemptRelative}/${journalFilename}`);
      const attemptJournalAbsolute = absolutePath(repoRoot, attemptJournalRelative);
      const journalStat = lstatOrNull(attemptJournalAbsolute);
      const expectedStaging = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
      const expectedBackup = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
      if (journalStat && (!journalStat.isFile() || journalStat.isSymbolicLink())) throw new Error(`Transaction attempt journal must be a regular no-follow file: ${attemptJournalRelative}`);
      const walRoot = join(absolutePath(repoRoot, expectedStaging), journalWriteDirectory);
      const prospectiveJournal = lstatOrNull(walRoot) ? recoverCanonicalJsonCandidates(walRoot, journalFilename, jsonPreviousName, journalJsonWritePreparationName, (candidatePath) => {
        const candidate = readJournal(candidatePath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
        if (candidate.planDigest !== digest || candidate.attemptOrdinal !== ordinal || candidate.stagingRoot !== expectedStaging || candidate.backupRoot !== expectedBackup) throw new Error(`Transaction attempt WAL identity does not match its canonical path: ${attemptJournalRelative}`);
        assertJournalPlanMembership(plan, candidate);
        assertJournalPhaseMembership(plan, candidate);
        assertJournalBackupAuthority(plan, candidate);
      }, true, true, attemptJournalAbsolute) : journalStat ? attemptJournalAbsolute : undefined;
      if (!prospectiveJournal) throw new Error(`Transaction attempt has no recoverable durable journal: ${attemptRelative}`);
      const attemptJournal = readJournal(journalStat ? attemptJournalAbsolute : prospectiveJournal, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
      if (attemptJournal.planDigest !== digest || attemptJournal.attemptOrdinal !== ordinal) throw new Error(`Transaction attempt identity does not match its canonical path: ${attemptJournalRelative}`);
      if (attemptJournal.stagingRoot !== expectedStaging || attemptJournal.backupRoot !== expectedBackup) throw new Error(`Transaction attempt roots do not match ordinal ${ordinal}`);
      const stageStat = lstatOrNull(absolutePath(repoRoot, expectedStaging));
      const backupStat = lstatOrNull(absolutePath(repoRoot, expectedBackup));
      const leaseStat = lstatOrNull(join(attemptAbsolute, leaseDirectory));
      if (stageStat && (!stageStat.isDirectory() || stageStat.isSymbolicLink()) || backupStat && (!backupStat.isDirectory() || backupStat.isSymbolicLink())) throw new Error(`Transaction attempt stage/backup must be direct no-follow directories: ${ordinal}`);
      if (leaseStat && (!leaseStat.isDirectory() || leaseStat.isSymbolicLink())) throw new Error(`Transaction attempt lease must be a direct no-follow directory: ${ordinal}`);
      if (attemptJournal.state !== "rolled-back" && attemptJournal.state !== "committed" && (!stageStat || !backupStat)) throw new Error(`Active transaction attempt is missing stage/backup roots: ${ordinal}`);
      assertJournalPhaseMembership(plan, attemptJournal);
      assertJournalBackupAuthority(plan, attemptJournal);
      existingAttempts.push({ ordinal, attemptRelative, journal: attemptJournal, journalRelative: attemptJournalRelative });
    }
  }
  for (let index = 0; index < existingAttempts.length; index++) {
    if (requireTaxonomyValue(existingAttempts[index],"existingAttempts[index]").ordinal !== String(index + 1).padStart(6, "0")) throw new Error("Transaction attempt ordinals are not contiguous");
    if (index < existingAttempts.length - 1 && requireTaxonomyValue(existingAttempts[index],"existingAttempts[index]").journal.state !== "rolled-back") throw new Error("Only rolled-back attempts may precede another transaction attempt");
  }
  const unpublishedOrdinals = new Set<string>();
  const publishedOrdinals = new Set(existingAttempts.map((entry) => entry.ordinal));
  const exactNextUnpublishedOrdinal = String(existingAttempts.length + 1).padStart(6, "0");
  for (const preparation of unpublishedAttempts) {
    if (publishedOrdinals.has(preparation.ordinal)) throw new Error(`Transaction attempt preparation collides with canonical ordinal ${preparation.ordinal}`);
    if (preparation.ordinal !== exactNextUnpublishedOrdinal) throw new Error(`Transaction attempt preparation ordinal ${preparation.ordinal} is not exact next ordinal ${exactNextUnpublishedOrdinal}`);
    if (unpublishedOrdinals.has(preparation.ordinal)) throw new Error(`Transaction attempt preparations duplicate ordinal ${preparation.ordinal}`);
    unpublishedOrdinals.add(preparation.ordinal);
  }
  const validateUnpublishedAttempt = (preparation: (typeof unpublishedAttempts)[number]): void => {
    if (transactionLeaseProcessIsAlive(preparation.pid)) throw new Error(`Transaction attempt preparation is active for pid ${preparation.pid}`);
    const allowed = new Set([stageDirectory, backupDirectory, leaseDirectory, journalFilename]);
    const children = readdirSync(preparation.path).sort(generatorPathCompare);
    if (children.some((name) => !allowed.has(name))) throw new Error(`Dead transaction attempt preparation contains unexpected evidence: ${basename(preparation.path)}`);
    const finalAttemptDirectory = canonicalDirectoryName(taxonomy, "transaction-attempt", preparation.ordinal, "transaction-attempts");
    const finalAttempt = normalizeRelative(`${attemptsRelative}/${finalAttemptDirectory}`);
    const assertPreparedIdentity = (prepared: MutableJournalRecord, path: string): void => {
      if (prepared.revision !== 0 || prepared.state !== "prepared" || prepared.planDigest !== digest || prepared.attemptOrdinal !== preparation.ordinal || prepared.stagingRoot !== normalizeRelative(`${finalAttempt}/${stageDirectory}`) || prepared.backupRoot !== normalizeRelative(`${finalAttempt}/${backupDirectory}`)) throw new Error(`Dead attempt preparation journal identity is invalid: ${path}`);
      assertJournalPlanMembership(plan, prepared);
      assertJournalPhaseMembership(plan, prepared);
      assertJournalBackupAuthority(plan, prepared);
    };
    for (const name of children) {
      const path = join(preparation.path, name);
      const stat = lstatOrNull(path);
      if (name === journalFilename) {
        if (!stat?.isFile() || stat.isSymbolicLink()) throw new Error(`Dead attempt preparation journal is not a no-follow file: ${path}`);
        assertPreparedIdentity(readJournal(path, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), path);
      } else {
        if (!stat?.isDirectory() || stat.isSymbolicLink()) throw new Error(`Dead attempt preparation child is not a no-follow directory: ${path}`);
        if (name === leaseDirectory) {
          recoverCanonicalJsonCandidates(path, journalFilename, jsonPreviousName, leaseJsonWritePreparationName, (candidate) => { const lease = parseTransactionLease(candidate, digest, preparation.ordinal); if (lease.pid !== preparation.pid) throw new Error(`Dead attempt preparation lease pid is invalid: ${path}`); }, false, true);
          if (lstatOrNull(join(path, journalFilename))) {
            const lease = parseTransactionLease(join(path, journalFilename), digest, preparation.ordinal);
            if (lease.pid !== preparation.pid) throw new Error(`Dead attempt preparation lease pid is invalid: ${path}`);
          }
        } else if (name === stageDirectory && readdirSync(path).length > 0) {
          const nested = readdirSync(path).sort(generatorPathCompare);
          if (canonicalJson(nested) !== canonicalJson([journalWriteDirectory])) throw new Error(`Dead attempt preparation stage contains unexpected evidence: ${path}`);
          const wal = join(path, journalWriteDirectory);
          const walStat = lstatOrNull(wal);
          if (!walStat?.isDirectory() || walStat.isSymbolicLink()) throw new Error(`Dead attempt preparation WAL is invalid: ${wal}`);
          recoverCanonicalJsonCandidates(wal, journalFilename, jsonPreviousName, journalJsonWritePreparationName, (candidate) => assertPreparedIdentity(readJournal(candidate, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), candidate), false, true);
          const walChildren = readdirSync(wal).sort(generatorPathCompare);
          if (walChildren.length > 0) {
            if (children.includes(journalFilename)) throw new Error(`Dead attempt preparation has both a durable journal and a pending initial WAL: ${wal}`);
            const walPath = join(wal, journalFilename);
            if (lstatOrNull(walPath)) {
              if (lstatSync(walPath).isSymbolicLink()) throw new Error(`Dead attempt preparation WAL snapshot is not a no-follow file: ${walPath}`);
              assertPreparedIdentity(readJournal(walPath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName), walPath);
            }
          }
        } else if (readdirSync(path).length > 0) throw new Error(`Dead attempt preparation executor root is not empty: ${path}`);
      }
    }
  };
  for (const preparation of unpublishedAttempts) validateUnpublishedAttempt(preparation);
  const validateTerminalAttempt = (attempt: (typeof existingAttempts)[number], ownedLease?: TransactionLeaseRecord): void => {
    if (attempt.journal.state !== "rolled-back" && attempt.journal.state !== "committed") return;
    const attemptAbsolute = absolutePath(repoRoot, attempt.attemptRelative);
    const stageAbsolute = absolutePath(repoRoot, attempt.journal.stagingRoot);
    const backupAbsolute = absolutePath(repoRoot, attempt.journal.backupRoot);
    const leaseAbsolute = join(attemptAbsolute, leaseDirectory);
    const leaseStat = lstatOrNull(leaseAbsolute);
    if (leaseStat) {
      const lease = readTransactionLease(leaseAbsolute, journalFilename, digest, attempt.ordinal);
      if (transactionLeaseProcessIsAlive(lease.pid) && (!ownedLease || canonicalJson(lease) !== canonicalJson(ownedLease))) throw new Error(`Terminal transaction attempt is leased by active pid ${lease.pid}`);
    }
    const stageStat = lstatOrNull(stageAbsolute), backupStat = lstatOrNull(backupAbsolute);
    if (!stageStat && !backupStat) {
      if (attempt.journal.state === "committed" && actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed before stale lease recovery: ${attempt.ordinal}`);
      return;
    }
    if (stageStat && (!stageStat.isDirectory() || stageStat.isSymbolicLink()) || backupStat && (!backupStat.isDirectory() || backupStat.isSymbolicLink()) || stageStat && !backupStat) throw new Error(`Terminal transaction attempt has an invalid executor tree: ${attempt.ordinal}`);
    assertRecoveryRootNames(repoRoot, plan, attempt.journal, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName);
    validateLeasePreparationEvidence(backupAbsolute, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attempt.ordinal);
    recoverTransactionBackups(repoRoot, plan, attempt.journal, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, true);
    recoverRestorePreparations(repoRoot, plan, attempt.journal, restorePreparationName, true);
    if (stageStat) {
      recoverReferenceEditPreparations(repoRoot, plan, attempt.journal, editPreparationName, editWritePreparationName, editWriteCandidateName, true);
      reconcileJournalWal(repoRoot, absolutePath(repoRoot, attempt.journalRelative), attempt.journal, plan, taxonomy, true);
    }
    const transient = [...(stageStat ? readdirSync(stageAbsolute) : []), ...readdirSync(backupAbsolute)].some((name) => splitLeadingEmoji(name).rest.startsWith("edit-") || splitLeadingEmoji(name).rest.startsWith("backup-") || splitLeadingEmoji(name).rest.startsWith("restore-") || splitLeadingEmoji(name).rest.startsWith("lease-") || name === journalWriteDirectory);
    if (!transient) {
      if (attempt.journal.state === "rolled-back") cleanupRolledBackTransaction(repoRoot, attempt.journal, plan, true);
      else {
        if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed: ${attempt.ordinal}`);
        cleanupCommittedTransaction(repoRoot, attempt.journal, plan, ticketRoot, true);
      }
    }
  };
  for (const attempt of existingAttempts) validateTerminalAttempt(attempt);
  const closeTerminalAttempt = (attempt: (typeof existingAttempts)[number]): void => {
    if (attempt.journal.state !== "rolled-back" && attempt.journal.state !== "committed") return;
    let terminalJournal = attempt.journal;
    const attemptAbsolute = absolutePath(repoRoot, attempt.attemptRelative);
    const stageAbsolute = absolutePath(repoRoot, attempt.journal.stagingRoot);
    const backupAbsolute = absolutePath(repoRoot, attempt.journal.backupRoot);
    const leaseAbsolute = join(attemptAbsolute, leaseDirectory);
    const hasResidue = Boolean(lstatOrNull(stageAbsolute) || lstatOrNull(backupAbsolute));
    let terminalLease: TransactionLeaseHandle | undefined;
    let createdLeaseBackup = false;
    if (hasResidue || lstatOrNull(leaseAbsolute)) {
      if (!hasResidue && attempt.journal.state === "committed" && actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed before stale lease recovery: ${attempt.ordinal}`);
      if (!lstatOrNull(backupAbsolute)) {
        mkdirSync(backupAbsolute);
        fsyncDirectory(attemptAbsolute);
        createdLeaseBackup = true;
      }
      terminalLease = acquireTransactionLease(repositoryAuthority, attempt.attemptRelative, attempt.journal.backupRoot, leaseDirectory, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attempt.ordinal, (owned) => validateTerminalAttempt(attempt, owned), transactionProbe);
      if (!hasResidue && createdLeaseBackup) durableRemove(backupAbsolute, true);
      terminalJournal = reconcileJournalWal(repoRoot, absolutePath(repoRoot, attempt.journalRelative), terminalJournal, plan, taxonomy);
    }
    transactionRepositoryFinally(() => {
      if (terminalJournal.state === "rolled-back") cleanupRolledBackTransaction(repoRoot, terminalJournal, plan);
      else {
        if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error(`Committed attempt post-state changed: ${attempt.ordinal}`);
        cleanupCommittedTransaction(repoRoot, terminalJournal, plan, ticketRoot);
      }
    }, () => {
      if (terminalLease) releaseTransactionLease(repositoryAuthority, `${attempt.attemptRelative}/${leaseDirectory}`, terminalLease);
    });
    if (canonicalJson(readdirSync(attemptAbsolute).sort(generatorPathCompare)) !== canonicalJson([journalFilename])) throw new Error(`Terminal transaction attempt is not closed: ${attempt.ordinal}`);
  };
  let attemptOrdinal: string;
  let journalRelative: string;
  let selectedAttempt: (typeof existingAttempts)[number] | undefined;
  if (resumeRelative) {
    const match = existingAttempts.find((entry) => entry.journalRelative === resumeRelative);
    if (!match) throw new Error(`Resume journal is not an exact existing canonical attempt for plan ${digest}`);
    if (match.journal.state === "rolled-back") throw new Error("Cannot resume journal in state rolled-back");
    selectedAttempt = match;
    attemptOrdinal = match.ordinal;
    journalRelative = match.journalRelative;
  } else {
    const active = existingAttempts.find((entry) => entry.journal.state !== "rolled-back" && entry.journal.state !== "committed");
    if (active) throw new Error(`Transaction attempt ${active.ordinal} is active and must be resumed`);
    if (existingAttempts.some((entry) => entry.journal.state === "committed")) throw new Error("Plan already has a committed transaction attempt");
    const next = existingAttempts.length === 0 ? 1 : Math.max(...existingAttempts.map((entry) => Number.parseInt(entry.ordinal, 10))) + 1;
    if (next > 999999) throw new Error("Transaction attempt ordinal space is exhausted");
    attemptOrdinal = String(next).padStart(6, "0");
    const attemptDirectory = canonicalDirectoryName(taxonomy, "transaction-attempt", attemptOrdinal, "transaction-attempts");
    journalRelative = normalizeRelative(`${attemptsRelative}/${attemptDirectory}/${journalFilename}`);
  }
  const attemptRelative = posix.dirname(journalRelative);
  const journalPath = absolutePath(repoRoot, journalRelative);
  let leaseHandle: TransactionLeaseHandle | undefined;
  const acquireLease = (backupRoot: string, beforePublish?: (owned?: TransactionLeaseRecord) => void): void => {
    leaseHandle = acquireTransactionLease(repositoryAuthority, attemptRelative, backupRoot, leaseDirectory, leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attemptOrdinal, beforePublish, transactionProbe);
  };
  const releaseLease = (): void => {
    if (!leaseHandle) return;
    releaseTransactionLease(repositoryAuthority, `${attemptRelative}/${leaseDirectory}`, leaseHandle);
    leaseHandle = undefined;
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
  execFileSync("git", ["cat-file", "-e", `${plan.baselineCommit}^{commit}`], { cwd: repoRoot, stdio: "ignore" });
  if (plan.excludedTreeDigests.length > 0) throw new Error("Opaque digest filesystem access is disabled; replan with empty excludedTreeDigests");
  if (!options.resumeJournal) checkCancellation(repoRoot, options.cancelFile);
  let preflightReferenceBasis: PreflightReferenceBasis | undefined;
  if (!options.resumeJournal) {
    assertTransactionRepositoryWitness(repositoryAuthority, captureTransactionRepositoryAuthority(repoRoot).indexRows);
    for (const access of bootstrap.accesses) assertTransactionRepositoryPath(repositoryAuthority, access.path, access.role, access.label);
    assertTransactionRepositoryPath(repositoryAuthority, transactionRootRelative, "subtree", "Taxonomy transaction executor");
    if (actualAffectedPreDigest(repoRoot, plan) !== plan.expectedAffectedPreStateDigest) throw new Error("Affected pre-state digest does not match plan expectation");
    for (const path of [...plan.moves.map((entry) => entry.destinationPath), ...plan.embeddedTicketRootRelocations.map((entry) => entry.destinationPath), ...plan.symlinkTargetEdits.map((entry) => entry.finalPath), ...plan.edits.map((entry) => entry.path), ...plan.regenerations.flatMap((entry) => entry.outputRoots)]) assertWritableAncestors(repoRoot, path);
    const referenceInventory = inventoryTaxonomy({ repoRoot, scope: plan.scope, ticketDir: options.explicitTicketDir, taxonomyPath: options.taxonomyPath, cancelFile: options.cancelFile });
    const moveSources = new Set(plan.moves.map((move) => move.sourcePath));
    for (const move of plan.moves) {
      const source = absolutePath(repoRoot, move.sourcePath);
      assertLeafPreimage(repoRoot, move.sourcePath, move.sourcePreimage);
      if (lstatOrNull(absolutePath(repoRoot, move.destinationPath)) && !moveSources.has(move.destinationPath)) throw new Error(`Move destination is occupied: ${move.destinationPath}`);
    }
    for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
      assertLeafPreimage(repoRoot, relocation.sourcePath, relocation.preimage);
      if (lstatOrNull(absolutePath(repoRoot, relocation.destinationPath))) throw new Error(`Embedded relocation destination is occupied: ${relocation.destinationPath}`);
    }
    for (const removal of plan.evidenceRemovals) assertLeafPreimage(repoRoot, removal.sourcePath, removal.preimage);
    for (const removal of plan.evidenceRemovals) {
      if (removal.authority.kind === "byte-and-mode-identical") for (const member of removal.authority.members) assertLeafPreimage(repoRoot, member.sourcePath, member.preimage);
      else if (removal.authority.kind === "exact-path-mutation") assertTicketImportantExactRemovalAuthority(repoRoot, removal);
      else assertTicketImportantRemovalAuthority(repoRoot, removal, taxonomy);
    }
    for (const edit of plan.symlinkTargetEdits) {
      const link = absolutePath(repoRoot, edit.sourcePath);
      if (!lstatOrNull(link)?.isSymbolicLink() || readlinkSync(link) !== edit.oldTarget) throw new Error(`Symlink target preimage changed: ${edit.sourcePath}`);
      const logical = lstatOrNull(absolutePath(repoRoot, edit.logicalTargetSourcePath));
      if ((edit.logicalTargetPreimage.state === "absent" && logical) || (edit.logicalTargetPreimage.state === "directory" && !logical?.isDirectory()) || ((edit.logicalTargetPreimage.state === "file" || edit.logicalTargetPreimage.state === "symlink") && (!logical || canonicalJson(leafPathPreimage(absolutePath(repoRoot, edit.logicalTargetSourcePath))) !== canonicalJson(edit.logicalTargetPreimage)))) throw new Error(`Logical symlink target preimage changed: ${edit.logicalTargetSourcePath}`);
    }
    for (const root of plan.embeddedTicketRoots) {
      if (canonicalJson(transactionTreeDigest(repositoryAuthority, root.sourceMetadataRoot, root.sourceMetadataRoot, [])) !== canonicalJson(root.sourceTreeDigest)) throw new Error(`Embedded root tree preimage changed: ${root.sourceMetadataRoot}`);
      const children = [
        ...plan.embeddedTicketRootRelocations.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath),
        ...plan.evidenceRemovals.filter((entry) => entry.embeddedTicketRootId === root.operationId).map((entry) => entry.sourcePath),
      ].sort(generatorPathCompare);
      if (children.some((path) => !path.startsWith(`${root.sourceTicketRoot}/`) || !path.startsWith(`${root.sourceMetadataRoot}/`)) || canonicalJson(transactionTreeDigest(repositoryAuthority, root.sourceMetadataRoot, root.sourceMetadataRoot, children)) !== canonicalJson(root.residualTreeDigest)) throw new Error(`Embedded root residual authority changed: ${root.sourceMetadataRoot}`);
      const incoming = incomingEmbeddedReferences(referenceInventory, root.sourceMetadataRoot).filter((row) => {
        const source = row.split("\u0000")[1];
        if (!planAuthority || source !== planAuthority.path) return true;
        const stat = lstatOrNull(absolutePath(repoRoot, source));
        return !stat?.isFile() || stat.isSymbolicLink() || !readFileSync(absolutePath(repoRoot, source)).equals(planAuthority.bytes);
      });
      if (sha256(`sha256-taxonomy-reference-set-v1\u0000${canonicalJson(incoming)}`) !== root.incomingReferenceDigest || incoming.length > 0) throw new Error(`Embedded root incoming reference set changed: ${root.sourceMetadataRoot}`);
      preflightReferenceBasis ??= capturePreflightReferenceBasis(repoRoot, taxonomy, options.explicitTicketDir, transactionRootRelative, plan, options.cancelFile, options.progress);
      const lexicalIncoming = lexicalTargetIncomingReferences(repoRoot, embeddedTargetPaths(plan, root), [root.sourceMetadataRoot], taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, plan, options.cancelFile, options.progress, undefined, preflightReferenceBasis);
      if (lexicalIncoming.length > 0) throw new Error(`Embedded root structured incoming reference set changed: ${root.sourceMetadataRoot}`);
    }
    for (const removal of plan.evidenceRemovals) {
      preflightReferenceBasis ??= capturePreflightReferenceBasis(repoRoot, taxonomy, options.explicitTicketDir, transactionRootRelative, plan, options.cancelFile, options.progress);
      const project = removal.authority.kind === "nested-cargo-generated-source" || removal.authority.kind === "exact-owner-generated-source" ? removalReferenceProjection(repoRoot, plan) : undefined;
      const incoming = lexicalTargetIncomingReferences(repoRoot, new Set([removal.sourcePath]), removalIncomingIgnoredSourceRoots(removal), taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, plan, options.cancelFile, options.progress, project, preflightReferenceBasis);
      if (incoming.length > 0) throw new Error(`Evidence-removal structured incoming reference set changed: ${removal.sourcePath}`);
    }
    for (const regeneration of plan.regenerations) {
      const actualInputs = regeneration.inputs.map((input) => generatorNodeRecord(repoRoot, input.path, taxonomy));
      if (canonicalJson(actualInputs) !== canonicalJson(regeneration.inputs)) throw new Error(`Regeneration input preimage changed: ${regeneration.id}`);
      const actualOutputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
      if (canonicalJson(actualOutputs) !== canonicalJson(regeneration.preOutputs)) throw new Error(`Regeneration output preimage changed: ${regeneration.id}`);
    }
    checkCancellation(repoRoot, options.cancelFile);
    {
      const authorityInventory = inventoryWithoutTransactionEvidence(referenceInventory, transactionRootRelative, planAuthority?.path);
      const authorityPlan = planTaxonomy(authorityInventory, { baselineCommit: plan.baselineCommit, excludedTreeDigests: [], cancelFile: options.cancelFile, progress: options.progress });
      const operationSets = [
        ["source-tree digest", plan.sourceTreeDigest, authorityPlan.sourceTreeDigest],
        ["affected pre-state digest", plan.expectedAffectedPreStateDigest, authorityPlan.expectedAffectedPreStateDigest],
        ["affected post-state digest", plan.expectedPostStateDigest, authorityPlan.expectedPostStateDigest],
        ["moves", plan.moves, authorityPlan.moves],
        ["embedded roots", plan.embeddedTicketRoots, authorityPlan.embeddedTicketRoots],
        ["embedded relocations", plan.embeddedTicketRootRelocations, authorityPlan.embeddedTicketRootRelocations],
        ["symlink target edits", plan.symlinkTargetEdits, authorityPlan.symlinkTargetEdits],
        ["evidence removals", plan.evidenceRemovals, authorityPlan.evidenceRemovals],
        ["destination ancestor preimages", plan.destinationAncestorPreimages, authorityPlan.destinationAncestorPreimages],
        ["reference edits", plan.edits, authorityPlan.edits],
        ["regenerations", plan.regenerations, authorityPlan.regenerations],
        ["unresolved findings", plan.unresolved, authorityPlan.unresolved],
      ] as const;
      const mismatch = operationSets.find(([, submitted, derived]) => canonicalJson(submitted) !== canonicalJson(derived));
      if (mismatch) throw new Error(`Plan ${mismatch[0]} cannot be rederived exactly from current schema-owned authority`);
    }
  }
  const validateSelectedResumeSnapshot = (ownedLease?: TransactionLeaseRecord): MutableJournalRecord | undefined => {
    if (!selectedAttempt || selectedAttempt.journal.state === "committed" || selectedAttempt.journal.state === "rolled-back") return selectedAttempt?.journal;
    const selectedLeaseRoot = join(absolutePath(repoRoot, selectedAttempt.attemptRelative), leaseDirectory);
    if (lstatOrNull(selectedLeaseRoot)) {
      const selectedLease = readTransactionLease(selectedLeaseRoot, journalFilename, digest, selectedAttempt.ordinal);
      if (transactionLeaseProcessIsAlive(selectedLease.pid) && (!ownedLease || canonicalJson(selectedLease) !== canonicalJson(ownedLease))) throw new Error(`Transaction attempt is leased by active pid ${selectedLease.pid}`);
    }
    const canonicalJournal = absolutePath(repoRoot, selectedAttempt.journalRelative);
    const selectedWal = join(absolutePath(repoRoot, selectedAttempt.journal.stagingRoot), journalWriteDirectory);
    const selectedProspective = lstatOrNull(selectedWal) ? recoverCanonicalJsonCandidates(selectedWal, journalFilename, jsonPreviousName, journalJsonWritePreparationName, (candidatePath) => {
      const candidate = readJournal(candidatePath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
      if (candidate.planDigest !== digest || candidate.attemptOrdinal !== selectedAttempt!.ordinal || candidate.stagingRoot !== selectedAttempt!.journal.stagingRoot || candidate.backupRoot !== selectedAttempt!.journal.backupRoot) throw new Error("Resume WAL identity differs from the selected canonical attempt");
      assertJournalPlanMembership(plan, candidate);
      assertJournalPhaseMembership(plan, candidate);
      assertJournalBackupAuthority(plan, candidate);
    }, true, true, canonicalJournal) : lstatOrNull(canonicalJournal) ? canonicalJournal : undefined;
    if (!selectedProspective) throw new Error("Selected resume attempt has no recoverable durable journal");
    const current = readJournal(lstatOrNull(canonicalJournal) ? canonicalJournal : selectedProspective, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
    const durable = reconcileJournalWal(repoRoot, canonicalJournal, current, plan, taxonomy, true);
    assertRecoveryRootNames(repoRoot, plan, durable, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName);
    validateLeasePreparationEvidence(absolutePath(repoRoot, durable.backupRoot), leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, selectedAttempt.ordinal);
    recoverTransactionBackups(repoRoot, plan, durable, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, true);
    recoverRestorePreparations(repoRoot, plan, durable, restorePreparationName, true);
    recoverReferenceEditPreparations(repoRoot, plan, durable, editPreparationName, editWritePreparationName, editWriteCandidateName, true);
    const tupleProbe: MutableJournalRecord = { ...durable, ...Object.fromEntries(JOURNAL_OPERATION_ARRAYS.map((key) => [key, [...durable[key]]])), backups: { ...durable.backups } } as MutableJournalRecord;
    try {
      if (durable.state === "rolling-back") reconcileRollbackTuples(repoRoot, plan, tupleProbe, taxonomy);
      else reconcileTransactionOwnedTuples(repoRoot, plan, tupleProbe, taxonomy);
    } catch (error) {
      if (!(error instanceof TaxonomyStartedRegenerationPartialError)) throw error;
    }
    for (const root of plan.embeddedTicketRoots) {
      const incoming = lexicalEmbeddedIncomingReferences(repoRoot, plan, root, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: embedded incoming references ${root.sourceMetadataRoot}`);
    }
    for (const removal of plan.evidenceRemovals) {
      const incoming = evidenceRemovalIncomingReferences(repoRoot, removal, plan, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, tupleProbe, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: evidence-removal incoming references ${removal.sourcePath}`);
    }
    return durable;
  };
  validateSelectedResumeSnapshot();
  let lockedResumeJournal: MutableJournalRecord | undefined;
  try {
    if (selectedAttempt && selectedAttempt.journal.state !== "committed") {
      acquireLease(selectedAttempt.journal.backupRoot, validateSelectedResumeSnapshot);
      lockedResumeJournal = validateSelectedResumeSnapshot(leaseHandle?.record);
    }
    for (const preparation of unpublishedAttempts) durableRemove(preparation.path, true);
    for (const attempt of existingAttempts) closeTerminalAttempt(attempt);
  } catch (error) {
    if (isTransactionRepositoryAuthorityError(error)) throw error;
    releaseLease();
    throw error;
  }
  if (options.resumeJournal && selectedAttempt?.journal.state === "committed") {
    if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error("Committed resume post-state digest changed");
    const terminalRoot = absolutePath(repoRoot, selectedAttempt.attemptRelative);
    if (canonicalJson(readdirSync(terminalRoot).sort(generatorPathCompare)) !== canonicalJson([journalFilename])) throw new Error("Committed resume attempt is not terminal journal-only evidence");
    return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
  }
  let journal: MutableJournalRecord;
  if (options.resumeJournal) {
    try {
    if (!selectedAttempt || !lockedResumeJournal) throw new Error("Resume journal has no selected owned-lease snapshot");
    journal = lockedResumeJournal;
    journal = reconcileJournalWal(repoRoot, journalPath, journal, plan, taxonomy);
    journal.probe = transactionProbe;
    if (lockedResumeJournal && canonicalJson(journalSnapshot(journal)) !== canonicalJson(journalSnapshot(lockedResumeJournal))) throw new Error("Resume journal changed after its owned-lease snapshot");
    if (journal.state !== "committed" && journal.state !== "rolled-back") {
      const recovered: MutableJournalRecord = { ...journal, startedRegenerationIds: [...journal.startedRegenerationIds], backups: { ...journal.backups } };
      const recoveredBackups = recoverTransactionBackups(repoRoot, plan, recovered, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
      recoverRestorePreparations(repoRoot, plan, recovered, restorePreparationName);
      recoverReferenceEditPreparations(repoRoot, plan, recovered, editPreparationName, editWritePreparationName, editWriteCandidateName);
      journal = reconcileJournalWal(repoRoot, journalPath, journal, plan, taxonomy);
      if (recoveredBackups) {
        let changed = false;
        for (const [path, backup] of Object.entries(recovered.backups)) {
          if (journal.backups[path]) {
            if (canonicalJson(journal.backups[path]) !== canonicalJson(backup)) throw new Error(`Recovered backup differs from promoted journal authority: ${path}`);
          } else { journal.backups[path] = backup; changed = true; }
        }
        for (const id of recovered.startedRegenerationIds) if (!journal.startedRegenerationIds.includes(id)) { journal.startedRegenerationIds.push(id); changed = true; }
        if (changed) persistJournal(repoRoot, journalPath, journal);
      }
    }
    assertJournalBackupAuthority(plan, journal);
    const expectedStagingRoot = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
    const expectedBackupRoot = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
    if (journal.attemptOrdinal !== attemptOrdinal || journal.stagingRoot !== expectedStagingRoot || journal.backupRoot !== expectedBackupRoot) throw new Error("Resume journal attempt identity and transaction roots do not match the canonical plan attempt");
    assertJournalPlanMembership(plan, journal);
    assertJournalPhaseMembership(plan, journal);
    if (journal.planDigest !== digest) throw new Error("Resume journal belongs to a different plan");
    if (journal.state !== "committed" && journal.state !== "rolled-back") assertActiveTransactionEvidence(repoRoot, plan, journal, false);
    if (journal.state === "committed") {
      if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error("Committed resume post-state digest changed");
      cleanupCommittedTransaction(repoRoot, journal, plan, ticketRoot);
      return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
    }
    if (journal.state === "rolled-back") throw new Error(`Cannot resume journal in state ${journal.state}`);
    if (journal.state === "rolling-back") {
      transactionRepositoryFinally(() => { rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options); }, () => { releaseLease(); });
      return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
    }
    let partialOutput: TaxonomyStartedRegenerationPartialError | undefined;
    try { if (reconcileTransactionOwnedTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal); }
    catch (error) {
      if (!(error instanceof TaxonomyStartedRegenerationPartialError)) throw error;
      partialOutput = error;
    }
    if (partialOutput || cancellationRequested(repoRoot, options.cancelFile)) {
      if (partialOutput) {
        journal.state = "rolling-back";
        journal.error = partialOutput.message;
        persistJournal(repoRoot, journalPath, journal);
      }
      transactionRepositoryFinally(() => { rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options); }, () => { releaseLease(); });
      return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
    }
    for (const root of plan.embeddedTicketRoots) {
      const incoming = lexicalEmbeddedIncomingReferences(repoRoot, plan, root, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: embedded incoming references ${root.sourceMetadataRoot}`);
    }
    for (const removal of plan.evidenceRemovals) {
      const incoming = evidenceRemovalIncomingReferences(repoRoot, removal, plan, taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, journal, undefined, options.progress);
      if (incoming.length > 0) throw new Error(`resume-state-drift: evidence-removal incoming references ${removal.sourcePath}`);
    }
    try {
      if (validateResumeTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
    } catch (error) {
      if (!(error instanceof TaxonomyStartedRegenerationPartialError) && !(error instanceof TaxonomyGeneratorInputDriftError) && !(error instanceof TaxonomyMoveSourceInputDriftError)) throw error;
      journal.state = "rolling-back";
      journal.error = error.message;
      persistJournal(repoRoot, journalPath, journal);
      transactionRepositoryFinally(() => { rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options); }, () => { releaseLease(); });
      return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
    }
    assertActiveTransactionEvidence(repoRoot, plan, journal, true);
    } catch (error) {
      if (isTransactionRepositoryAuthorityError(error)) throw error;
      releaseLease();
      throw error;
    }
  } else {
    const stagingRoot = normalizeRelative(`${attemptRelative}/${stageDirectory}`);
    const backupRoot = normalizeRelative(`${attemptRelative}/${backupDirectory}`);
    journal = { schemaVersion: 2, revision: 0, planDigest: digest, attemptOrdinal, state: "prepared", stagingRoot, backupRoot, journalWriteDirectory, jsonWritePreparationName: journalJsonWritePreparationName, jsonPreviousName, probe: transactionProbe, preparedMoveIds: [], stagedMoveIds: [], installedMoveIds: [], preparedEmbeddedRelocationIds: [], stagedEmbeddedRelocationIds: [], installedEmbeddedRelocationIds: [], preparedEvidenceRemovalIds: [], stagedEvidenceRemovalIds: [], preparedEmbeddedRootIds: [], stagedEmbeddedRootIds: [], preparedSymlinkTargetEditIds: [], stagedSymlinkTargetEditIds: [], installedSymlinkTargetEditIds: [], appliedEditPaths: [], startedRegenerationIds: [], completedRegenerationIds: [], sourceParentPrunePaths: [], backups: {} };
    checkCancellation(repoRoot, options.cancelFile);
    assertNoFollowAncestors(repoRoot, absolutePath(repoRoot, attemptsRelative), "transaction attempts root", true);
    const allocationAncestors = [attemptsAbsolute, absolutePath(repoRoot, transactionRelative), transactionRootAbsolute].map((path) => ({ path, existed: Boolean(lstatOrNull(path)) }));
    const preparationToken = randomUUID();
    const preparationRelative = normalizeRelative(`${attemptsRelative}/${attemptPreparationName(attemptOrdinal, process.pid, preparationToken)}`);
    const preparationRoot = absolutePath(repoRoot, preparationRelative);
    const preparationStage = join(preparationRoot, stageDirectory);
    const preparationBackup = join(preparationRoot, backupDirectory);
    const preparationLease = join(preparationRoot, leaseDirectory);
    const leaseRecord: TransactionLeaseRecord = { schemaVersion: 1, planDigest: digest, attemptOrdinal, token: randomUUID(), pid: process.pid };
    try {
      mkdirSync(attemptsAbsolute, { recursive: true });
      fsyncDirectory(dirname(attemptsAbsolute));
      try { mkdirSync(preparationRoot); }
      catch (error) {
        if ((error as NodeJS.ErrnoException).code === "EEXIST") throw new Error(`Transaction attempt preparation collision at ${preparationRelative}`);
        throw error;
      }
      fsyncDirectory(attemptsAbsolute);
      transactionProbe("transaction-attempt-preparation-mkdir", preparationRelative);
      mkdirSync(preparationStage);
      mkdirSync(preparationBackup);
      mkdirSync(preparationLease);
      transactionProbe("transaction-attempt-preparation-children", preparationRelative);
      publishCanonicalJsonCandidate(preparationLease, journalFilename, jsonPreviousName, leaseRecord, leaseJsonWritePreparationName, undefined, transactionProbe, "transaction-initial-lease-json");
      transactionProbe("transaction-initial-lease-prepared", preparationRelative);
      const initialWalRoot = join(preparationStage, journalWriteDirectory);
      const initialWal = join(initialWalRoot, journalFilename);
      mkdirSync(initialWalRoot);
      transactionProbe("transaction-initial-wal-mkdir", preparationRelative);
      publishCanonicalJsonCandidate(initialWalRoot, journalFilename, jsonPreviousName, journalSnapshot(journal), journalJsonWritePreparationName, undefined, transactionProbe, "transaction-initial-journal");
      durableRename(initialWal, join(preparationRoot, journalFilename));
      transactionProbe("transaction-initial-journal-canonical", preparationRelative);
      durableRemove(initialWalRoot, true);
      fsyncFile(join(preparationRoot, journalFilename));
      fsyncDirectory(preparationStage);
      fsyncDirectory(preparationBackup);
      fsyncDirectory(preparationRoot);
      durableRename(preparationRoot, absolutePath(repoRoot, attemptRelative));
    } catch (error) {
      if (isTransactionRepositoryAuthorityError(error)) throw error;
      if (lstatOrNull(preparationRoot)) durableRemove(preparationRoot, true);
      for (const ancestor of allocationAncestors) if (!ancestor.existed && lstatOrNull(ancestor.path)?.isDirectory() && readdirSync(ancestor.path).length === 0) durableRemove(ancestor.path, true);
      if ((error as NodeJS.ErrnoException).code === "EEXIST" || (error as NodeJS.ErrnoException).code === "ENOTEMPTY") throw new Error(`Transaction attempt allocation race at ${attemptRelative}`);
      throw error;
    }
    leaseHandle = { root: absolutePath(repoRoot, `${attemptRelative}/${leaseDirectory}`), filename: journalFilename, record: leaseRecord };
  }
  const sourceSet = new Set(plan.moves.map((move) => move.sourcePath));
  try {
    if (!options.resumeJournal) transactionProbe("transaction-attempt-canonical-published", attemptRelative);
    checkCancellation(repoRoot, options.cancelFile);
    for (const move of plan.moves) {
      if (journal.stagedMoveIds.includes(move.operationId)) {
        const candidates = journal.installedMoveIds.includes(move.operationId)
          ? [absolutePath(repoRoot, move.destinationPath), join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), absolutePath(repoRoot, move.sourcePath)]
          : [join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), absolutePath(repoRoot, move.sourcePath)];
        const resumedPath = candidates.find((path) => lstatOrNull(path));
        if (!resumedPath) throw new Error(`Resume move state is invalid: ${move.operationId}`);
        const installedLink = plan.symlinkTargetEdits.find((edit) => edit.sourcePath === move.sourcePath && edit.finalPath === move.destinationPath && journal.installedSymlinkTargetEditIds.includes(edit.operationId));
        if (!journal.appliedEditPaths.includes(move.destinationPath) && canonicalJson(leafPreimage(resumedPath)) !== canonicalJson(retargetedMovePreimage(move, installedLink))) throw new Error(`Resume move preimage changed: ${move.operationId}`);
        continue;
      }
      const source = absolutePath(repoRoot, move.sourcePath);
      const destination = absolutePath(repoRoot, move.destinationPath);
      const sourceStat = lstatOrNull(source);
      if (!sourceStat) throw new Error(`Move source is missing: ${move.sourcePath}`);
      if (canonicalJson(leafPreimage(source)) !== canonicalJson(move.sourcePreimage)) throw new Error(`Move source preimage changed: ${move.sourcePath}`);
      if (lstatOrNull(destination) && !sourceSet.has(move.destinationPath)) throw new Error(`Move destination is occupied: ${move.destinationPath}`);
    }
    if (preflightReferenceBasis) validatePreflightReferenceBasis(preflightReferenceBasis, options.cancelFile, options.progress);
    journal.state = "staging";
    persistJournal(repoRoot, journalPath, journal);
    for (const [index, move] of plan.moves.entries()) {
      checkCancellation(repoRoot, options.cancelFile);

      const stage = join(absolutePath(repoRoot, journal.stagingRoot), move.operationId);
      if (!journal.preparedMoveIds.includes(move.operationId)) {
        journal.preparedMoveIds.push(move.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      if (!lstatOrNull(stage) && !journal.installedMoveIds.includes(move.operationId)) {
        mkdirSync(dirname(stage), { recursive: true });
        durableRename(absolutePath(repoRoot, move.sourcePath), stage);
      }
      if (!journal.stagedMoveIds.includes(move.operationId)) {
        journal.stagedMoveIds.push(move.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "staging", index + 1, plan.moves.length, move.sourcePath);
    }
    for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const stage = join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`);
      if (!journal.preparedEmbeddedRelocationIds.includes(relocation.operationId)) { journal.preparedEmbeddedRelocationIds.push(relocation.operationId); persistJournal(repoRoot, journalPath, journal); }
      if (!lstatOrNull(stage) && !journal.installedEmbeddedRelocationIds.includes(relocation.operationId)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(absolutePath(repoRoot, relocation.sourcePath), stage); }
      if (!journal.stagedEmbeddedRelocationIds.includes(relocation.operationId)) { journal.stagedEmbeddedRelocationIds.push(relocation.operationId); persistJournal(repoRoot, journalPath, journal); }
      report(options.progress, "apply", "staging-embedded-relocations", index + 1, plan.embeddedTicketRootRelocations.length, relocation.sourcePath);
      checkCancellation(repoRoot, options.cancelFile);
    }
    for (const [index, removal] of plan.evidenceRemovals.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const stage = join(absolutePath(repoRoot, journal.stagingRoot), `removal-${removal.operationId}`);
      if (!journal.preparedEvidenceRemovalIds.includes(removal.operationId)) { journal.preparedEvidenceRemovalIds.push(removal.operationId); persistJournal(repoRoot, journalPath, journal); }
      if (!lstatOrNull(stage) && !journal.stagedEvidenceRemovalIds.includes(removal.operationId)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(absolutePath(repoRoot, removal.sourcePath), stage); }
      if (!journal.stagedEvidenceRemovalIds.includes(removal.operationId)) { journal.stagedEvidenceRemovalIds.push(removal.operationId); persistJournal(repoRoot, journalPath, journal); }
      report(options.progress, "apply", "staging-evidence-removals", index + 1, plan.evidenceRemovals.length, removal.sourcePath);
      checkCancellation(repoRoot, options.cancelFile);
    }
    injectFailure(options, "after-staging");
    journal.state = "disposing";
    persistJournal(repoRoot, journalPath, journal);
    for (const [index, root] of plan.embeddedTicketRoots.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const stage = join(absolutePath(repoRoot, journal.stagingRoot), `root-${root.operationId}`);
      if (!journal.preparedEmbeddedRootIds.includes(root.operationId)) { journal.preparedEmbeddedRootIds.push(root.operationId); persistJournal(repoRoot, journalPath, journal); }
      if (!lstatOrNull(stage) && !journal.stagedEmbeddedRootIds.includes(root.operationId)) { assertDirectoryOnlyTree(absolutePath(repoRoot, root.sourceMetadataRoot)); if (canonicalJson(noFollowTreeDigest(repoRoot, root.sourceMetadataRoot)) !== canonicalJson(root.residualTreeDigest)) throw new Error(`Embedded root residual tree differs from frozen structure: ${root.sourceMetadataRoot}`); mkdirSync(dirname(stage), { recursive: true }); durableRename(absolutePath(repoRoot, root.sourceMetadataRoot), stage); }
      if (!journal.stagedEmbeddedRootIds.includes(root.operationId)) { journal.stagedEmbeddedRootIds.push(root.operationId); persistJournal(repoRoot, journalPath, journal); }
      report(options.progress, "apply", "disposing-embedded-roots", index + 1, plan.embeddedTicketRoots.length, root.sourceMetadataRoot);
      checkCancellation(repoRoot, options.cancelFile);
    }
    injectFailure(options, "after-embedded-root-staging");
    journal.state = "installing";
    persistJournal(repoRoot, journalPath, journal);
    for (const [index, move] of plan.moves.entries()) {
      checkCancellation(repoRoot, options.cancelFile);

      const destination = absolutePath(repoRoot, move.destinationPath);
      const installed = journal.installedMoveIds.includes(move.operationId);
      if (!lstatOrNull(destination)) {
        mkdirSync(dirname(destination), { recursive: true });
        durableRename(join(absolutePath(repoRoot, journal.stagingRoot), move.operationId), destination);
      }
      if (!installed) {
        journal.installedMoveIds.push(move.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "moves", index + 1, plan.moves.length, move.destinationPath);
    }
    injectFailure(options, "after-moves");
    for (const [index, relocation] of plan.embeddedTicketRootRelocations.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const destination = absolutePath(repoRoot, relocation.destinationPath);
      if (!journal.installedEmbeddedRelocationIds.includes(relocation.operationId)) {
        if (!lstatOrNull(destination)) { mkdirSync(dirname(destination), { recursive: true }); durableRename(join(absolutePath(repoRoot, journal.stagingRoot), `relocation-${relocation.operationId}`), destination); }
        journal.installedEmbeddedRelocationIds.push(relocation.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "installing-embedded-relocations", index + 1, plan.embeddedTicketRootRelocations.length, relocation.destinationPath);
    }
    injectFailure(options, "after-relocations");
    journal.state = "retargeting";
    persistJournal(repoRoot, journalPath, journal);
    for (const [index, edit] of plan.symlinkTargetEdits.entries()) {
      checkCancellation(repoRoot, options.cancelFile);
      const link = absolutePath(repoRoot, edit.finalPath);
      const stage = join(absolutePath(repoRoot, journal.stagingRoot), `symlink-${edit.operationId}`);
      if (!journal.preparedSymlinkTargetEditIds.includes(edit.operationId)) { journal.preparedSymlinkTargetEditIds.push(edit.operationId); persistJournal(repoRoot, journalPath, journal); }
      if (!journal.stagedSymlinkTargetEditIds.includes(edit.operationId)) {
        if (!lstatOrNull(stage)) { mkdirSync(dirname(stage), { recursive: true }); durableRename(link, stage); }
        journal.stagedSymlinkTargetEditIds.push(edit.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      if (!journal.installedSymlinkTargetEditIds.includes(edit.operationId)) {
        if (!lstatOrNull(link)) durableSymlink(edit.newTarget, link, process.platform === "win32" ? edit.windowsLinkType : undefined);
        if (readlinkSync(link) !== edit.newTarget) throw new Error(`Symlink retarget verification failed: ${edit.finalPath}`);
        journal.installedSymlinkTargetEditIds.push(edit.operationId);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "retargeting-symlinks", index + 1, plan.symlinkTargetEdits.length, edit.finalPath);
      checkCancellation(repoRoot, options.cancelFile);
    }
    injectFailure(options, "after-symlink-retargeting");
    journal.state = "editing";
    persistJournal(repoRoot, journalPath, journal);
    const editGroups = new Map<string, ReferenceEdit[]>();
    for (const edit of plan.edits) editGroups.set(edit.path, [...(editGroups.get(edit.path) ?? []), edit]);
    const sortedEditGroups = [...editGroups.entries()].sort(([a], [b]) => a.localeCompare(b));
    for (const [index, [path, edits]] of sortedEditGroups.entries()) {
      checkCancellation(repoRoot, options.cancelFile);

      if (!journal.appliedEditPaths.includes(path)) {
        const preimages = new Map(edits.map((edit) => [canonicalJson(edit.preimage), edit.preimage]));
        const preimage = [...preimages.values()][0];
        if (preimages.size !== 1 || !preimage || canonicalJson(leafPreimage(absolutePath(repoRoot, path))) !== canonicalJson(preimage)) throw new Error(`Reference edit preimage changed: ${path}`);
        backupPath(repoRoot, path, absolutePath(repoRoot, journal.backupRoot), journal, preimage, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
        persistJournal(repoRoot, journalPath, journal);
        applyReferenceEditAtomically(repoRoot, plan, journal, path, editPreparationName, editWritePreparationName, editWriteCandidateName);
        journal.appliedEditPaths.push(path);
        persistJournal(repoRoot, journalPath, journal);
      }
      report(options.progress, "apply", "edits", index + 1, sortedEditGroups.length, path);
    }
    checkCancellation(repoRoot, options.cancelFile);
    injectFailure(options, "after-edits");
    journal.state = "regenerating";
    persistJournal(repoRoot, journalPath, journal);
    for (const [index, regeneration] of plan.regenerations.entries()) {

      checkCancellation(repoRoot, options.cancelFile);
      if (validateResumeTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
      if (journal.completedRegenerationIds.includes(regeneration.id)) {
        if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.outputs)) throw new Error(`Completed regeneration output changed: ${regeneration.id}`);
        if (regeneration.verifyCommand) execFileSync(regeneration.verifyCommand[0], [...regeneration.verifyCommand.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), env: { ...process.env }, stdio: "inherit" });
        report(options.progress, "apply", "regenerations", index + 1, plan.regenerations.length, regeneration.contractId);
        continue;
      }
      if (canonicalJson(generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy)) !== canonicalJson(regeneration.preOutputs)) throw new Error(`Regeneration output preimage changed before execution: ${regeneration.id}`);
      if (!journal.startedRegenerationIds.includes(regeneration.id)) {
        for (const output of regeneration.preOutputs) if (output.nodeKind !== "directory") backupPath(repoRoot, output.path, absolutePath(repoRoot, journal.backupRoot), journal, output as TaxonomyBackupExpectedPreimage, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
        journal.startedRegenerationIds.push(regeneration.id);
        persistJournal(repoRoot, journalPath, journal);
      }
      execFileSync(regeneration.command[0], [...regeneration.command.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), env: { ...process.env }, stdio: "inherit" });
      checkCancellation(repoRoot, options.cancelFile);
      const actualOutputs = generatorTreeInventory(repoRoot, regeneration.outputRoots, taxonomy);
      if (canonicalJson(actualOutputs) !== canonicalJson(regeneration.outputs)) throw new Error(`Regeneration ${regeneration.id} produced missing, stale, unexpected, byte-different, or mode-different output`);
      durablySyncGeneratorRecords(repoRoot, actualOutputs);
      if (regeneration.verifyCommand) execFileSync(regeneration.verifyCommand[0], [...regeneration.verifyCommand.slice(1)], { cwd: absolutePath(repoRoot, regeneration.cwd), env: { ...process.env }, stdio: "inherit" });
      checkCancellation(repoRoot, options.cancelFile);
      journal.completedRegenerationIds.push(regeneration.id);
      persistJournal(repoRoot, journalPath, journal);
      report(options.progress, "apply", "regenerations", index + 1, plan.regenerations.length, regeneration.contractId);
    }
    injectFailure(options, "after-regenerations");
    checkCancellation(repoRoot, options.cancelFile);
    if ((plan.regenerations.length > 0 || plan.moves.some((move) => move.sourceAuthority !== undefined)) && validateResumeTuples(repoRoot, plan, journal, taxonomy)) persistJournal(repoRoot, journalPath, journal);
    journal.state = "verifying";
    persistJournal(repoRoot, journalPath, journal);
    injectFailure(options, "before-verify");
    const prunableSourceParents = emptySourceParents(repoRoot, plan, ticketRoot);
    const projectionState = artifactProjectionPostApplyViolations(repoRoot, plan, taxonomy, new Set(prunableSourceParents));
    if (projectionState.length > 0) throw new Error(`Projection verification failed: ${requireTaxonomyValue(projectionState[0],"projectionState[0]").code} at ${requireTaxonomyValue(projectionState[0],"projectionState[0]").path}`);
    const staleProjectionTokens = projectionStaleViolations(repoRoot, plan, taxonomy, undefined, options.explicitTicketDir);
    if (staleProjectionTokens.length > 0) throw new Error(`Projection verification found ${staleProjectionTokens.length} stale old-hierarchy token(s): ${requireTaxonomyValue(staleProjectionTokens[0],"staleProjectionTokens[0]").path}`);
    if (actualAffectedDigest(repoRoot, plan, taxonomy) !== plan.expectedPostStateDigest) throw new Error("Post-state digest does not match plan expectation");
    const oldTargets = new Set<string>([...plan.moves.map((entry) => entry.sourcePath), ...plan.evidenceRemovals.map((entry) => entry.sourcePath)]);
    for (const root of plan.embeddedTicketRoots) for (const path of embeddedTargetPaths(plan, root)) oldTargets.add(path);
    const staleTransactionReferences = lexicalTargetIncomingReferences(repoRoot, oldTargets, [], taxonomy, options.explicitTicketDir, planAuthority, transactionRootRelative, plan, options.cancelFile, options.progress);
    if (staleTransactionReferences.length > 0) throw new Error(`Post-state contains ${staleTransactionReferences.length} structured reference(s) to disposed source paths`);
    const postInventoryRaw = inventoryTaxonomyWithSourceParentPruning({ repoRoot, scope: plan.scope, ticketDir: options.explicitTicketDir, taxonomyPath: options.taxonomyPath, cancelFile: options.cancelFile }, new Set(prunableSourceParents));
    const exactPlanArtifact = (() => {
      if (!planAuthority) return false;
      const stat = lstatOrNull(absolutePath(repoRoot, planAuthority.path));
      return Boolean(stat?.isFile() && !stat.isSymbolicLink() && readFileSync(absolutePath(repoRoot, planAuthority.path)).equals(planAuthority.bytes));
    })();
    const postInventory = inventoryAfterSourceParentPruning(inventoryWithoutTransactionEvidence(postInventoryRaw, transactionRootRelative, exactPlanArtifact ? planAuthority!.path : undefined), prunableSourceParents);
    const postPlan = planTaxonomy(postInventory, { baselineCommit: plan.baselineCommit, excludedTreeDigests: [], cancelFile: options.cancelFile });
    const pendingPostOperations = postPlan.moves.length + postPlan.embeddedTicketRoots.length + postPlan.embeddedTicketRootRelocations.length + postPlan.symlinkTargetEdits.length + postPlan.evidenceRemovals.length + postPlan.edits.length + postPlan.regenerations.length;
    if (pendingPostOperations > 0 || postPlan.unresolved.some((entry) => entry.severity === "error")) throw new Error(`Post-state does not converge to an empty plan: ${pendingPostOperations} operation(s), ${postPlan.unresolved.length} finding(s)${postPlan.unresolved[0] ? `; first ${postPlan.unresolved[0].code} at ${postPlan.unresolved[0].path}` : ""}`);
    checkCancellation(repoRoot, options.cancelFile);
    if (canonicalJson(emptySourceParents(repoRoot, plan, ticketRoot)) !== canonicalJson(prunableSourceParents)) throw new Error("Exact source-parent pruning set changed before commit");
    journal.sourceParentPrunePaths = [...prunableSourceParents].sort(generatorPathCompare);
    journal.state = "committed";
    persistJournal(repoRoot, journalPath, journal);
    transactionProbe("transaction-source-parent-pruning-committed", sourceRelative(relative(repoRoot, journalPath)));
    cleanupCommittedTransaction(repoRoot, journal, plan, ticketRoot);
    const appliedOperations = plan.moves.length + plan.embeddedTicketRoots.length + plan.embeddedTicketRootRelocations.length + plan.symlinkTargetEdits.length + plan.evidenceRemovals.length + plan.edits.length + plan.regenerations.length;
    report(options.progress, "apply", "complete", appliedOperations, appliedOperations);
    releaseLease();
    return { planDigest: digest, journalPath, state: "committed", appliedMoves: plan.moves.length, appliedEmbeddedTicketRootRelocations: plan.embeddedTicketRootRelocations.length, appliedSymlinkTargetEdits: plan.symlinkTargetEdits.length, appliedEvidenceRemovals: plan.evidenceRemovals.length, appliedEdits: plan.edits.length, appliedRegenerations: plan.regenerations.length };
  } catch (error) {
    if (isTransactionRepositoryAuthorityError(error)) throw error;
    const failureMessage = error instanceof Error ? error.message : String(error);
    const committedFailure = {};
    try {
      let durable = (() => {
        if (lstatOrNull(journalPath)) return readJournal(journalPath, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
        const walRoot = join(absolutePath(repoRoot, journal.stagingRoot), journalWriteDirectory);
        const walStat = lstatOrNull(walRoot);
        if (!walStat?.isDirectory() || walStat.isSymbolicLink()) throw new Error("Taxonomy journal recovery has no canonical journal or direct WAL authority");
        const preparations = readdirSync(walRoot).filter((name) => name !== journalFilename);
        if (preparations.length !== 1) throw new Error("Taxonomy journal recovery has ambiguous previous-image authority");
        const previous = join(walRoot, requireTaxonomyValue(preparations[0],"preparations[0]"), jsonPreviousName);
        const previousStat = lstatOrNull(previous);
        if (!previousStat?.isFile() || previousStat.isSymbolicLink()) throw new Error("Taxonomy journal recovery previous-image authority is unavailable");
        const recovered = readJournal(previous, journalWriteDirectory, journalJsonWritePreparationName, jsonPreviousName);
        if (recovered.planDigest !== digest || recovered.attemptOrdinal !== attemptOrdinal || recovered.stagingRoot !== journal.stagingRoot || recovered.backupRoot !== journal.backupRoot) throw new Error("Taxonomy journal recovery previous-image identity differs from the active attempt");
        assertJournalPlanMembership(plan, recovered);
        assertJournalPhaseMembership(plan, recovered);
        assertJournalBackupAuthority(plan, recovered);
        return recovered;
      })();
      durable.probe = transactionProbe;
      if (durable.state === "committed") throw committedFailure;
      assertRecoveryRootNames(repoRoot, plan, durable, backupPreparationName, restorePreparationName, editPreparationName, leasePreparationName);
      validateLeasePreparationEvidence(absolutePath(repoRoot, durable.backupRoot), leasePreparationName, leaseJsonWritePreparationName, journalFilename, jsonPreviousName, digest, attemptOrdinal);
      const prospective = reconcileJournalWal(repoRoot, journalPath, durable, plan, taxonomy, true);
      recoverTransactionBackups(repoRoot, plan, prospective, backupPreparationName, backupWritePreparationName, backupWriteCandidateName, true);
      recoverRestorePreparations(repoRoot, plan, prospective, restorePreparationName, true);
      recoverReferenceEditPreparations(repoRoot, plan, prospective, editPreparationName, editWritePreparationName, editWriteCandidateName, true);
      durable = reconcileJournalWal(repoRoot, journalPath, durable, plan, taxonomy);
      if (durable.state === "committed") throw committedFailure;
      const recovered: MutableJournalRecord = { ...durable, ...Object.fromEntries(JOURNAL_OPERATION_ARRAYS.map((key) => [key, [...durable[key]]])), backups: { ...durable.backups } } as MutableJournalRecord;
      const recoveredBackups = recoverTransactionBackups(repoRoot, plan, recovered, backupPreparationName, backupWritePreparationName, backupWriteCandidateName);
      recoverRestorePreparations(repoRoot, plan, recovered, restorePreparationName);
      recoverReferenceEditPreparations(repoRoot, plan, recovered, editPreparationName, editWritePreparationName, editWriteCandidateName);
      let recoveredTuples = false;
      try { recoveredTuples = reconcileTransactionOwnedTuples(repoRoot, plan, recovered, taxonomy); }
      catch (resumeError) { if (!(resumeError instanceof TaxonomyStartedRegenerationPartialError)) throw resumeError; }
      if (recoveredBackups || recoveredTuples || canonicalJson(journalSnapshot(durable)) !== canonicalJson(journalSnapshot(recovered))) {
        durable = recovered;
        persistJournal(repoRoot, journalPath, durable);
      }
      journal = durable;
      journal.error = failureMessage;
      rollbackTransaction(repoRoot, plan, journalPath, journal, taxonomy, options);
    } catch (rollbackError) {
      if (isTransactionRepositoryAuthorityError(rollbackError)) throw rollbackError;
      if (rollbackError === committedFailure) {
        releaseLease();
        throw error;
      }
      journal.error = `${failureMessage}; rollback failed: ${rollbackError instanceof Error ? rollbackError.message : String(rollbackError)}`;
      try { persistJournal(repoRoot, journalPath, journal); } catch (persistError) { if (isTransactionRepositoryAuthorityError(persistError)) throw persistError; }
      releaseLease();
      throw new Error(journal.error);
    }
    report(options.progress, "apply", "rolled-back", 0, plan.moves.length + plan.embeddedTicketRoots.length + plan.embeddedTicketRootRelocations.length + plan.symlinkTargetEdits.length + plan.evidenceRemovals.length + plan.edits.length + plan.regenerations.length);
    releaseLease();
    return { planDigest: digest, journalPath, state: "rolled-back", appliedMoves: 0, appliedEmbeddedTicketRootRelocations: 0, appliedSymlinkTargetEdits: 0, appliedEvidenceRemovals: 0, appliedEdits: 0, appliedRegenerations: 0 };
  }
}
