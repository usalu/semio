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

function clearGitCommitTemplate(root: string): void {
  const configured = git(root, ["config", "--local", "--get", "commit.template"]).out;
  if (!configured) return;
  const normalized = configured.replace(/\\/g, "/");
  if (normalized.endsWith("gkcommittemplate.txt") || normalized.includes("/.git/gkcommittemplate.txt")) {
    git(root, ["config", "--local", "--unset", "commit.template"]);
  }
}

function git(root: string, args: string[]): { ok: boolean; out: string } {
  const r = spawnSync("git", args, { cwd: root, encoding: "utf8" });
  if (r.status !== 0) return { ok: false, out: (r.stderr ?? r.stdout ?? "").trim() };
  return { ok: true, out: (r.stdout ?? "").trim() };
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

function emojiForPath(path: string): string {
  const p = path.toLowerCase();
  if (p.includes("script.ts") || p.includes("script.sh")) return "📜";
  if (p.includes("test") || p.includes("spec") || p.includes(".test.")) return "🧪";
  if (p.startsWith("framework/") || p.includes("presentation/")) return "🖼️";
  if (p.startsWith("semio/")) return "🏘️";
  if (p.startsWith("puzzle/") || p.startsWith("elements/")) return "🧩";
  if (p.startsWith("repo/")) return "🧰";
  if (p.startsWith("coda/")) return "🗃️";
  if (p.startsWith(".agents/")) return "🫡";
  if (p.includes("hook")) return "🔄";
  return "✏️";
}

function summarizeGroup(top: string, files: string[]): string {
  if (files.length === 1) {
    const base = files[0].split("/").pop() ?? files[0];
    return `Update ${top === "." ? base : `${top}/${base}`}`;
  }
  return `Update ${top} (${files.length} files)`;
}

function listCachedPaths(root: string): string[] {
  const out = git(root, ["diff", "--cached", "--name-only"]).out;
  if (!out) return [];
  return out.split("\n").filter(Boolean);
}

function listAddedTicketPaths(root: string): string[] {
  const added = git(root, ["diff", "--cached", "--name-only", "--diff-filter=A"]).out;
  const paths = added ? added.split("\n").filter(Boolean) : [];
  return paths.filter((p) => TICKET_JSON_RE.test(p));
}

function diffBullets(root: string, ticketPaths: Set<string>): string[] {
  const files = listCachedPaths(root).filter((p) => !ticketPaths.has(p));
  const groups = new Map<string, string[]>();
  for (const f of files) {
    const top = f.includes("/") ? f.split("/")[0] : f;
    const list = groups.get(top) ?? [];
    list.push(f);
    groups.set(top, list);
  }
  const bullets: string[] = [];
  for (const [top, paths] of groups) {
    if (bullets.length >= 8) break;
    bullets.push(`- ${emojiForPath(paths[0])} ${summarizeGroup(top, paths)}`);
  }
  if (files.length > 8 && bullets.length > 0) bullets.push("- …");
  return bullets;
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

export function buildMicroCommitMessage(root: string, contributor: Contributor): string {
  const { line1Base, nnn } = nextCounter(root, contributor);
  const now = new Date();
  const ticketPaths = new Set(listAddedTicketPaths(root));
  const bullets = [...ticketBullets(root), ...diffBullets(root, ticketPaths)].slice(0, 8);
  if (bullets.length === 0) bullets.push(`- ✏️ WIP checkpoint`);
  const lines = [
    `${line1Base}🚩${nnn}`,
    formatSecond(now),
    ...bullets,
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
  const templateAbs = join(dir, "gkcommittemplate.txt");
  clearGitCommitTemplate(root);
  for (const p of [templateAbs, join(dir, "COMMIT_EDITMSG")]) {
    try {
      rmSync(p, { force: true });
    } catch {
      /* ignore */
    }
  }
  writeFileSync(templateAbs, message);
  writeFileSync(join(dir, "COMMIT_EDITMSG"), message);
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
  const message = buildMicroCommitMessage(root, contributor);
  writeFileSync(msgFile, message);
  writeFileSync(preparedDigestPath(root), `${digestMicroCommitMessage(message)}\n`);
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
  for (const p of [join(dir, "COMMIT_EDITMSG"), join(dir, "gkcommittemplate.txt"), preparedDigestPath(root)]) {
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
  if (cmd !== "prepare") {
    console.error("[micro-commit] usage: bun ./script.ts micro-commit prepare [level override tokens]");
    process.exit(1);
  }

  const level = loadLevel(root, contributor, segments.slice(1));
  const staged = git(root, ["add", "-A"]);
  if (!staged.ok) {
    console.error(staged.out || "git add -A failed");
    process.exit(1);
  }
  const message = buildMicroCommitMessage(root, contributor);
  writeMicroCommitTemplates(root, message);
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
