import { writeSync } from "node:fs";
import { isAbsolute, join, sep } from "node:path";

import { buildMicroCommitMessage, clearStaleTemplatesBeforePrepare, handlePrepareCommitMsg, wipeAfterCommit, writeMicroCommitTemplates } from "../📄️prepared-message/🟦️.ts";
import { git, gitDir, gitEmail, gitRepoRoot } from "../🌳️git/🟦️.ts";
import { installMicroCommitGitHooks } from "../🪝️hooks/🟦️.ts";
import { branchValidationError, currentBranch, findContributor, loadLevel } from "../👤️contributor/🟦️.ts";
import { listCachedPaths, readDiffBulletsInput, validateBulletEmojis, validateBulletsAgainstStaged, validateBulletSpacing, writePreparedBullets } from "../🔸️bullets/🟦️.ts";
import { gitSpawnEnv } from "../../../🏃️process/🌿️environment/🌳️git/🟦️.ts";
import { runToolSyncV1 } from "../../../../../../../🔨️modules/🏃️process/📦️packages/🟦️typescript/🧰️tools/🟦️.ts";
import {sayCommitV1,reportCommitErrorV1} from "../📖️texts/🟦️.ts";

export function resetMicroCommitTemplates(root: string): void {
  wipeAfterCommit(root);
}

function emitPrepareStdout(message: string): void {
  writeSync(1, Buffer.from(message.endsWith("\n") ? message : `${message}\n`));
}

function stageMicroCommitChanges(root: string): ReturnType<typeof git> {
  const excludedPaths = new Set<string>();
  for (;;) {
    const args = ["add", "-A"];
    if (excludedPaths.size > 0) args.push("--", ".", ...[...excludedPaths].map((path) => `:(exclude)${path}`));
    const staged = git(root, args);
    if (staged.ok) return staged;
    const rejectedPaths = [...staged.out.matchAll(/^error: '(.+)' does not have a commit checked out$/gm)]
      .map((match) => match[1]!)
      .filter((path) => !isAbsolute(path) && path !== "." && !path.startsWith(`..${sep}`));
    if (rejectedPaths.length === 0 || rejectedPaths.every((path) => excludedPaths.has(path))) return staged;
    for (const path of rejectedPaths) excludedPaths.add(path);
  }
}

export function runMicroCommit(root: string, segments: string[]): void {
  root = gitRepoRoot(root);
  const cmd = segments[0] ?? "prepare";
  if (cmd === "reset") {
    resetMicroCommitTemplates(root);
    process.exit(0);
  }
  if (cmd === "install-hooks") {
    installMicroCommitGitHooks(root);
    process.exit(0);
  }
  if (cmd === "prepare-commit-msg") {
    const msgFile = segments[1];
    if (!msgFile) process.exit(1);
    handlePrepareCommitMsg(root, msgFile, segments[2] ?? "");
    process.exit(0);
  }
  const branchError = branchValidationError("micro-commit", currentBranch(root));
  if (branchError) {
    console.error(branchError);
    process.exit(1);
  }
  const contributor = findContributor(root);
  if (!contributor) {
    sayCommitV1("micro.no-contributor-for-git-user",{email:String(gitEmail(root) || "(unset)")},"error");
    process.exit(1);
  }
  if (cmd === "stage") {
    clearStaleTemplatesBeforePrepare(root);
    const staged = stageMicroCommitChanges(root);
    if (!staged.ok) {
      console.error(staged.out || "git add -A failed");
      process.exit(1);
    }
    process.exit(0);
  }
  if (cmd === "diff") {
    const patch = git(root, ["diff", "--cached"]);
    if (!patch.ok) {
      console.error(patch.out || "git diff --cached failed");
      process.exit(1);
    }
    process.stdout.write(patch.out ? `${patch.out}\n` : "");
    process.exit(0);
  }
  if (cmd !== "prepare") {
    sayCommitV1("micro.usage-repo-script-ts-stage",{},"error");
    process.exit(1);
  }

  const dash = segments.indexOf("--");
  const levelSegments = dash >= 0 ? segments.slice(1, dash) : segments.slice(1);
  let bulletsFile = dash >= 0 ? (segments[dash + 1] ?? null) : null;
  if (!bulletsFile) {
    const last = segments[segments.length - 1];
    if (last && (last.endsWith(".txt") || last.includes("/") || last.includes("\\"))) {
      bulletsFile = last;
    }
  }

  const level = loadLevel(root, contributor, levelSegments);
  clearStaleTemplatesBeforePrepare(root);
  const stagedPaths = listCachedPaths(root);
  if (stagedPaths.length === 0) {
    sayCommitV1("micro.nothing-is-staged-stage-the",{},"error");
    process.exit(1);
  }

  const diffBullets = readDiffBulletsInput(root, bulletsFile);
  if (diffBullets.length === 0) {
    for (const p of stagedPaths) console.error(p);
    console.error("");
    const patch = git(root, ["diff", "--cached"]);
    if (patch.out) console.error(patch.out);
    sayCommitV1("micro.analyze-the-staged-paths-and",{},"error");
    process.exit(1);
  }

  validateBulletSpacing(diffBullets);
  validateBulletEmojis(diffBullets);
  validateBulletsAgainstStaged(diffBullets, stagedPaths);
  writePreparedBullets(root, diffBullets);
  let message: string;
  try {
    message = buildMicroCommitMessage(root, contributor, diffBullets);
  } catch (e) {
    reportCommitErrorV1(e);
    process.exit(1);
  }
  writeMicroCommitTemplates(root, message);
  emitPrepareStdout(message);

  if (level === "prepare-only") process.exit(0);

  const dir = gitDir(root);
  const commit = runToolSyncV1("git", ["commit", "-S", "-F", join(dir, "COMMIT_EDITMSG")], { cwd: root, encoding: "utf8", env: gitSpawnEnv() });
  if (commit.status !== 0) {
    console.error((commit.stderr ?? commit.stdout ?? "git commit failed").trim());
    process.exit(commit.status ?? 1);
  }
  wipeAfterCommit(root);
  if (level === "prepare-and-commit") process.exit(0);

  const push = runToolSyncV1("git", ["push"], { cwd: root, encoding: "utf8", env: gitSpawnEnv() });
  if (push.status !== 0) {
    console.error((push.stderr ?? push.stdout ?? "git push failed").trim());
    process.exit(push.status ?? 1);
  }
  process.exit(0);
}
