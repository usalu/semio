import { createHash } from "node:crypto";
import { chmodSync, copyFileSync, existsSync, mkdirSync, readFileSync, readdirSync, rmSync, writeFileSync } from "node:fs";
import { join } from "node:path";
import { spawnSync } from "node:child_process";

export type MicroCommitLevel = "prepare-only" | "prepare-and-commit" | "prepare-and-commit-and-push";

type Contributor = { alias: string; emoji: string; name: string; email: string; emails?: string[] };

const COUNTER_RE = /^(.+🎆\d{2}🌙\d{2}☀️\d{2})🚩(\d+)$/;
const TICKET_JSON_RE = /^\.repo\/🎫\/.+\/ticket\.json$/;
export function digestMicroCommitMessage(message: string): string {
  return createHash("sha256").update(message.replace(/\r\n/g, "\n").trimEnd()).digest("hex");
}

function preparedDigestPath(root: string): string {
  return join(gitDir(root), "semio-micro-commit-digest");
}

const GK_TEMPLATE_BASENAME = "gkcommittemplate";

function git(root: string, args: string[]): { ok: boolean; out: string } {
  const r = spawnSync("git", args, { cwd: root, encoding: "utf8" });
  if (r.status !== 0) return { ok: false, out: (r.stderr ?? r.stdout ?? "").trim() };
  return { ok: true, out: (r.stdout ?? "").trim() };
}

function gitCachedNames(root: string, extra: string[] = []): string[] {
  const r = spawnSync("git", ["diff", "--cached", "--name-only", "-z", ...extra], { cwd: root });
  if (r.status !== 0) return [];
  const raw = (r.stdout ?? Buffer.alloc(0)).toString("utf8");
  if (!raw) return [];
  return raw.split("\0").filter(Boolean);
}

function branchAllowed(root: string): boolean {
  const b = git(root, ["branch", "--show-current"]).out;
  return b.includes("⛳wip") || b.includes("🏗️dev");
}

function gitEmail(root: string): string {
  return git(root, ["config", "user.email"]).out;
}

function findContributor(root: string): Contributor | null {
  const email = gitEmail(root).toLowerCase();
  if (!email) return null;
  const dir = join(root, ".repo", "🧑‍💻");
  if (!existsSync(dir)) return null;
  for (const name of readdirSync(dir, { withFileTypes: true })) {
    if (!name.isDirectory()) continue;
    const path = join(dir, name.name, "contributor.json");
    if (!existsSync(path)) continue;
    const c = JSON.parse(readFileSync(path, "utf8")) as Contributor & { emails?: string[] };
    const emails = [c.email, ...(c.emails ?? [])].filter((e): e is string => typeof e === "string" && e.length > 0).map((e) => e.toLowerCase());
    if (emails.includes(email)) return c;
  }
  return null;
}

function loadLevel(root: string, contributor: Contributor, segments: string[]): MicroCommitLevel {
  const token = segments.join(" ").toLowerCase();
  if (/\b(gp|gpush|push!|\+push)\b/.test(token)) return "prepare-and-commit-and-push";
  if (/\b(gc|commit!|\+commit)\b/.test(token)) return "prepare-and-commit";
  if (/\b(g\.|gprepare|prepare!|\+prepare)\b/.test(token)) return "prepare-only";
  const path = join(root, ".repo", "🧑‍💻", contributor.alias, "micro-commit.json");
  if (existsSync(path)) {
    const j = JSON.parse(readFileSync(path, "utf8")) as { level?: string };
    if (j.level === "prepare-and-commit" || j.level === "prepare-and-commit-and-push" || j.level === "prepare-only") {
      return j.level;
    }
  }
  return "prepare-only";
}

function pad2(n: number): string {
  return String(n).padStart(2, "0");
}

function pad3(n: number): string {
  return String(n).padStart(3, "0");
}

const PLAIN_COUNTER_RE = /^(\d+)$/;
const COUNTER_LOG_DEPTH = 40;

/** 🔢Reads micro-commit counter from subject: full `…🚩NNN` line or legacy plain number only. */
export function extractCounterFromSubject(subject: string): { nnn: number; line1Base: string | null } | null {
  const s = subject.trim();
  const formatted = COUNTER_RE.exec(s);
  if (formatted) return { nnn: Number.parseInt(formatted[2], 10), line1Base: formatted[1] };
  const plain = PLAIN_COUNTER_RE.exec(s);
  if (plain) return { nnn: Number.parseInt(plain[1], 10), line1Base: null };
  return null;
}

/** 🎆Bumps counter from recent subjects (newest first); plain `33` or `…🚩NNN` in that window. */
export function bumpCounterFromHistory(
  subjectsNewestFirst: string[],
  contributor: Contributor,
  now = new Date(),
): { line1Base: string; nnn: string } {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  const fresh = `${contributor.emoji}${contributor.alias}🎆${yy}🌙${mm}☀️${dd}`;
  let max = 0;
  let line1Base: string | null = null;
  for (const subject of subjectsNewestFirst) {
    const hit = extractCounterFromSubject(subject);
    if (!hit) continue;
    max = Math.max(max, hit.nnn);
    if (!line1Base && hit.line1Base) line1Base = hit.line1Base;
  }
  if (max > 0) return { line1Base: line1Base ?? fresh, nnn: pad3(max + 1) };
  return { line1Base: fresh, nnn: "001" };
}

export function bumpCounterFromSubject(
  subject: string,
  contributor: Contributor,
  now = new Date(),
): { line1Base: string; nnn: string } {
  return bumpCounterFromHistory([subject], contributor, now);
}

function nextCounter(root: string, contributor: Contributor): { line1Base: string; nnn: string } {
  const log = git(root, ["log", "--format=%s", `-${COUNTER_LOG_DEPTH}`]).out;
  const subjects = log ? log.split("\n").filter(Boolean) : [];
  return bumpCounterFromHistory(subjects, contributor);
}

function formatSecond(now: Date): string {
  const yy = pad2(now.getFullYear() % 100);
  const mm = pad2(now.getMonth() + 1);
  const dd = pad2(now.getDate());
  const hh = pad2(now.getHours());
  const min = pad2(now.getMinutes());
  const ss = pad2(now.getSeconds());
  return `🎆${yy}🌙${mm}☀️${dd}⏰${hh}⌚${min}⏱️${ss}`;
}

function preparedBulletsPath(root: string): string {
  return join(gitDir(root), "semio-micro-commit-bullets");
}

/** 📝Normalizes LLM-authored bullet lines (`- emoji text`). */
export function normalizeBulletLines(text: string): string[] {
  return text
    .split("\n")
    .map((l) => l.trim())
    .filter((l) => l.length > 0 && !l.startsWith("#"))
    .map((l) => (l.startsWith("- ") ? l : `- ${l}`))
    .slice(0, 8);
}

function writePreparedBullets(root: string, bullets: string[]): void {
  writeFileSync(preparedBulletsPath(root), `${bullets.join("\n")}\n`);
}

function readPreparedBullets(root: string): string[] {
  const path = preparedBulletsPath(root);
  if (!existsSync(path)) return [];
  return normalizeBulletLines(readFileSync(path, "utf8"));
}

function counterFromSubjectLine(message: string): string {
  const subject = message.split("\n")[0] ?? "";
  const m = COUNTER_RE.exec(subject.trim());
  return m?.[2] ?? "000";
}

function validateBulletsAgainstStaged(bullets: string[], staged: string[]): void {
  if (staged.length === 0) return;
  const text = bullets.join("\n").toLowerCase();
  const significant = staged.filter(
    (p) => !/\/micro-commit\.ts$/.test(p) && !p.endsWith("SKILL.md") && !/\/index\.test\.ts$/.test(p),
  );
  if (significant.length === 0) return;
  const covered = significant.filter((p) => {
    const segments = p.toLowerCase().split(/[/._-]+/).filter((s) => s.length >= 5);
    return segments.some((s) => text.includes(s));
  });
  if (covered.length === 0) {
    console.error("micro-commit: bullets do not match staged changes — read `micro-commit diff` again");
    for (const p of staged) console.error(`  ${p}`);
    process.exit(1);
  }
}

function readDiffBulletsInput(root: string, bulletsFile: string | null): string[] {
  if (bulletsFile) {
    const path = bulletsFile.startsWith("/") ? bulletsFile : join(root, bulletsFile);
    return normalizeBulletLines(readFileSync(path, "utf8"));
  }
  if (!process.stdin.isTTY) {
    return normalizeBulletLines(readFileSync(0, "utf8"));
  }
  return [];
}

function listCachedPaths(root: string): string[] {
  return gitCachedNames(root);
}

function listAddedTicketPaths(root: string): string[] {
  return gitCachedNames(root, ["--diff-filter=A"]).filter((p) => TICKET_JSON_RE.test(p));
}

function ticketBullets(root: string): string[] {
  const bullets: string[] = [];
  for (const rel of listAddedTicketPaths(root)) {
    const path = join(root, rel);
    const t = JSON.parse(readFileSync(path, "utf8")) as { emoji?: string; title?: string };
    if (!t.emoji || !t.title) continue;
    bullets.push(`- ${t.emoji}${t.title}`);
  }
  return bullets;
}

export function buildMicroCommitMessage(root: string, contributor: Contributor, diffBullets: string[] = []): string {
  const { line1Base, nnn } = nextCounter(root, contributor);
  const now = new Date();
  const authored = diffBullets.length > 0 ? normalizeBulletLines(diffBullets.join("\n")) : readPreparedBullets(root);
  const bullets = [...ticketBullets(root), ...authored].slice(0, 8);
  if (bullets.length === 0) {
    throw new Error("micro-commit: at least one description bullet is required");
  }
  const lines = [
    `${line1Base}🚩${nnn}`,
    "",
    formatSecond(now),
    ...bullets,
    "",
    `Signed-off-by: ${contributor.name} <${contributor.email}>`,
  ];
  return `${lines.join("\n")}\n`;
}

function gitDir(root: string): string {
  const out = git(root, ["rev-parse", "--git-dir"]).out;
  return out.startsWith("/") ? out : join(root, out);
}

export function writeMicroCommitTemplates(root: string, message: string): void {
  const dir = gitDir(root);
  const nnn = counterFromSubjectLine(message);
  const templateName = `${GK_TEMPLATE_BASENAME}-${nnn}.txt`;
  const templateAbs = join(dir, templateName);
  const legacyAbs = join(dir, `${GK_TEMPLATE_BASENAME}.txt`);

  for (const name of readdirSync(dir)) {
    if (name.startsWith(GK_TEMPLATE_BASENAME) && name !== templateName) {
      try {
        rmSync(join(dir, name), { force: true });
      } catch {
        /* ignore */
      }
    }
  }

  writeFileSync(templateAbs, message);
  writeFileSync(legacyAbs, message);
  writeFileSync(join(dir, "COMMIT_EDITMSG"), message);
  git(root, ["config", "--local", "commit.template", templateAbs]);
  writeFileSync(preparedDigestPath(root), `${digestMicroCommitMessage(message)}\n`);
}

export function shouldRefreshPreparedCommitMessage(current: string, preparedDigest: string | null): boolean {
  const trimmed = current.trim();
  if (!trimmed) return true;
  if (!preparedDigest) return false;
  return digestMicroCommitMessage(current) === preparedDigest.trim();
}

export function handlePrepareCommitMsg(root: string, msgFile: string, source: string): void {
  if (!branchAllowed(root)) return;
  const contributor = findContributor(root);
  if (!contributor) return;
  if (source === "merge" || source === "squash") return;
  const digestPath = preparedDigestPath(root);
  const preparedDigest = existsSync(digestPath) ? readFileSync(digestPath, "utf8") : null;
  const current = existsSync(msgFile) ? readFileSync(msgFile, "utf8") : "";
  if (!shouldRefreshPreparedCommitMessage(current, preparedDigest)) return;
  const staged = listCachedPaths(root);
  if (staged.length === 0) git(root, ["add", "-A"]);
  const message = buildMicroCommitMessage(root, contributor, readPreparedBullets(root));
  writeFileSync(msgFile, message);
  writeMicroCommitTemplates(root, message);
}

export function installMicroCommitGitHooks(root: string): void {
  const hooksDir = join(root, ".git", "hooks");
  mkdirSync(hooksDir, { recursive: true });
  for (const name of ["prepare-commit-msg", "post-commit"] as const) {
    copyFileSync(join(root, "repo", "hooks", name), join(hooksDir, name));
    chmodSync(join(hooksDir, name), 0o755);
  }
}

export function resetMicroCommitTemplates(root: string): void {
  const dir = gitDir(root);
  const paths = [join(dir, "COMMIT_EDITMSG"), preparedDigestPath(root), preparedBulletsPath(root)];
  for (const name of readdirSync(dir)) {
    if (name.startsWith(GK_TEMPLATE_BASENAME)) paths.push(join(dir, name));
  }
  for (const p of paths) {
    try {
      rmSync(p, { force: true });
    } catch {
      /* ignore */
    }
  }
}

function emitPrepareStdout(message: string): void {
  process.stdout.write(message.endsWith("\n") ? message : `${message}\n`);
}

export function runMicroCommit(root: string, segments: string[]): void {
  if (!branchAllowed(root)) {
    console.error("micro-commit: branch must contain ⛳wip or 🏗️dev");
    process.exit(1);
  }
  const contributor = findContributor(root);
  if (!contributor) {
    console.error(`micro-commit: no contributor for git user.email ${gitEmail(root) || "(unset)"}`);
    process.exit(1);
  }

  const cmd = segments[0] ?? "prepare";
  if (cmd === "reset") {
    resetMicroCommitTemplates(root);
    process.exit(0);
  }
  if (cmd === "prepare-commit-msg") {
    const msgFile = segments[1];
    if (!msgFile) process.exit(1);
    handlePrepareCommitMsg(root, msgFile, segments[2] ?? "");
    process.exit(0);
  }
  if (cmd === "stage") {
    const staged = git(root, ["add", "-A"]);
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
    console.error(
      "[micro-commit] usage: bun ./script.ts micro-commit <stage|diff|prepare> [level tokens…] [-- bullets.txt]",
    );
    process.exit(1);
  }

  const dash = segments.indexOf("--");
  const levelSegments = dash >= 0 ? segments.slice(1, dash) : segments.slice(1);
  const bulletsFile = dash >= 0 ? (segments[dash + 1] ?? null) : null;

  const level = loadLevel(root, contributor, levelSegments);
  const staged = git(root, ["add", "-A"]);
  if (!staged.ok) {
    console.error(staged.out || "git add -A failed");
    process.exit(1);
  }

  const stagedPaths = listCachedPaths(root);
  const diffBullets = readDiffBulletsInput(root, bulletsFile);
  if (diffBullets.length === 0) {
    for (const p of stagedPaths) console.error(p);
    console.error("");
    const patch = git(root, ["diff", "--cached"]);
    if (patch.out) console.error(patch.out);
    console.error(
      "\nmicro-commit: analyze the staged paths and diff above; pass 1–8 semantic bullets on stdin (lines starting with '- ')",
    );
    process.exit(1);
  }

  validateBulletsAgainstStaged(diffBullets, stagedPaths);
  writePreparedBullets(root, diffBullets);
  let message: string;
  try {
    message = buildMicroCommitMessage(root, contributor, diffBullets);
  } catch (e) {
    console.error(e instanceof Error ? e.message : String(e));
    process.exit(1);
  }
  writeMicroCommitTemplates(root, message);
  console.error(`[micro-commit] GitKraken template: ${join(gitDir(root), `${GK_TEMPLATE_BASENAME}-${counterFromSubjectLine(message)}.txt`)}`);
  console.error(`[micro-commit] staged: ${stagedPaths.join(", ") || "(none)"}`);
  emitPrepareStdout(message);

  if (level === "prepare-only") process.exit(0);

  const dir = gitDir(root);
  const commit = spawnSync("git", ["commit", "-S", "-F", join(dir, "COMMIT_EDITMSG")], { cwd: root, encoding: "utf8" });
  if (commit.status !== 0) {
    console.error((commit.stderr ?? commit.stdout ?? "git commit failed").trim());
    process.exit(commit.status ?? 1);
  }
  if (level === "prepare-and-commit") process.exit(0);

  const push = spawnSync("git", ["push"], { cwd: root, encoding: "utf8" });
  if (push.status !== 0) {
    console.error((push.stderr ?? push.stdout ?? "git push failed").trim());
    process.exit(push.status ?? 1);
  }
  process.exit(0);
}
