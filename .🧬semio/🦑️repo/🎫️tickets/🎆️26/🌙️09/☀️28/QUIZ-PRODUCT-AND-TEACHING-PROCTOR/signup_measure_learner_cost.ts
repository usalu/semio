/** ⚖️ Measures what one registered learner costs a proctor on disk and in memory, so the default registration cap
 * (`DEFAULT_LIMITS.learners`, `PROCTOR_MAX_LEARNERS`) is sized from numbers: registers `--count` learners per phase
 * against a private copy of a built `proctor` (anonymous, a short pseudonym, a name of the maximum length) over a
 * throw-away data directory and reports, per phase, the growth of the live database (file and write-ahead log), of a
 * compact copy (`proctor backup`) and of the resident memory.
 *
 * Run from the repo root: `bun <this file> --executable <proctor> [--count 20000] [--work <directory>]`. The edge
 * limits of the measured proctor are opened wide by environment; nothing else differs from a dev proctor. */
import { execFile, spawn } from "node:child_process";
import { copyFileSync, existsSync, mkdirSync, rmSync, statSync } from "node:fs";
import { connect, createServer, type Socket } from "node:net";
import { join, resolve } from "node:path";
import { commandEnvelope, newId } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🎯️targets/⚛️react/🔨️modules/🛂️proctor/🟦️.ts";
import { encodeCommandEnvelope } from "../../../../../../../🧰️framework/🛍️products/🖥️server/🟦️.ts";
import type { IdentityClaim } from "../../../../../../../🧰️framework/🛍️products/❓️quiz/🟦️.ts";

const RESERVED = new Set([6061, 8791, 6161, 6162, 8891, 8892, 6071, 8801]);
const args = process.argv.slice(2);
const value = (name: string): string | undefined => (args.indexOf(name) < 0 ? undefined : args[args.indexOf(name) + 1]);
const executable = resolve(value("--executable") ?? "");
const count = Number(value("--count") ?? 20_000);
const work = resolve(value("--work") ?? ".");
const pause = (milliseconds: number): Promise<void> => new Promise((done) => setTimeout(done, milliseconds));

async function freePort(): Promise<number> {
  for (;;) {
    const port = await new Promise<number>((found) => {
      const probe = createServer();
      probe.listen(0, "127.0.0.1", () => {
        const address = probe.address();
        probe.close(() => found(typeof address === "object" && address !== null ? address.port : 0));
      });
    });
    if (port !== 0 && !RESERVED.has(port)) return port;
  }
}

/** 🧵️ One kept-open connection posting one request after the other. */
class Wire {
  private socket: Socket | undefined;
  private received = Buffer.alloc(0);
  private answered: ((answer: { status: number; body: string }) => void) | undefined;

  constructor(private readonly port: number) {}

  post(path: string, body: string): Promise<{ status: number; body: string }> {
    return new Promise((answered) => {
      this.answered = answered;
      if (this.socket === undefined) {
        const socket = connect({ host: "127.0.0.1", port: this.port, noDelay: true });
        this.socket = socket;
        this.received = Buffer.alloc(0);
        socket.on("data", (chunk) => this.take(chunk));
        socket.on("error", () => undefined);
        socket.on("close", () => this.settle({ status: 0, body: "" }));
      }
      this.socket.write(`POST ${path} HTTP/1.1\r\nHost: 127.0.0.1:${this.port}\r\nContent-Type: application/json\r\nContent-Length: ${Buffer.byteLength(body)}\r\n\r\n${body}`);
    });
  }

  close(): void {
    this.socket?.destroy();
    this.socket = undefined;
  }

  private take(chunk: Buffer): void {
    this.received = Buffer.concat([this.received, chunk]);
    const end = this.received.indexOf("\r\n\r\n");
    if (end < 0) return;
    const head = this.received.subarray(0, end).toString("latin1").toLowerCase();
    const length = Number(/\r\ncontent-length: (\d+)/u.exec(head)?.[1] ?? 0);
    if (this.received.length < end + 4 + length) return;
    const body = this.received.subarray(end + 4, end + 4 + length).toString("utf8");
    this.received = this.received.subarray(end + 4 + length);
    this.settle({ status: Number(head.slice(9, 12)), body });
  }

  private settle(answer: { status: number; body: string }): void {
    const answered = this.answered;
    this.answered = undefined;
    answered?.(answer);
  }
}

function printed(command: string, argv: readonly string[], env?: NodeJS.ProcessEnv): Promise<string> {
  return new Promise((done) => execFile(command, [...argv], { encoding: "utf8", windowsHide: true, env }, (_error, stdout, stderr) => done(`${stdout ?? ""}${stderr ?? ""}`)));
}

async function residentKibibytes(pid: number): Promise<number> {
  if (process.platform === "win32") {
    const listed = await printed("tasklist", ["/FI", `PID eq ${pid}`, "/FO", "CSV", "/NH"]);
    return Number(/"([\d.,'   ]+) K"\s*$/mu.exec(listed.trim())?.[1]?.replace(/\D/gu, "") ?? 0);
  }
  return Number((await printed("ps", ["-o", "rss=", "-p", String(pid)])).trim());
}

const size = (path: string): number => (existsSync(path) ? statSync(path).size : 0);

if (!existsSync(executable)) throw new Error("--executable must name a built proctor");
rmSync(work, { recursive: true, force: true });
mkdirSync(join(work, "data"), { recursive: true });
const copy = join(work, `proctor-measure${process.platform === "win32" ? ".exe" : ""}`);
copyFileSync(executable, copy);
const port = await freePort();
const env: NodeJS.ProcessEnv = {
  ...process.env,
  PROCTOR_PORT: String(port),
  PROCTOR_BIND: "127.0.0.1",
  PROCTOR_DATA: join(work, "data"),
  PROCTOR_CATALOG: resolve("🎓️teaching/🏛️architecture/❓️quiz/🔣️.json"),
  PROCTOR_MAX_LEARNERS: "1000000",
  PROCTOR_LIMIT_COMMANDS_PER_SECOND: "1000000",
  PROCTOR_LIMIT_COMMANDS_BURST: "1000000",
  PROCTOR_LIMIT_QUERIES_PER_SECOND: "1000000",
  PROCTOR_LIMIT_QUERIES_BURST: "1000000",
  PROCTOR_LIMIT_SIGNUPS_PER_HOUR: "1000000",
  PROCTOR_LIMIT_SIGNUPS_BURST: "1000000",
};
const proctor = spawn(copy, ["serve"], { env, stdio: ["ignore", "ignore", "inherit"], windowsHide: true });
try {
  for (let waited = 0; ; waited += 200) {
    try {
      if ((await fetch(`http://127.0.0.1:${port}/instance`)).ok) break;
    } catch {}
    if (waited > 60_000) throw new Error("the measured proctor did not come up");
    await pause(200);
  }
  const database = join(work, "data", "proctor.sqlite");
  const state = async (label: string): Promise<{ file: number; wal: number; compact: number; resident: number }> => {
    await fetch(`http://127.0.0.1:${port}/queries`, { method: "POST", headers: { "content-type": "application/json" }, body: JSON.stringify({ queryId: newId(), kind: "quiz.leaderboard", version: 1, scope: "architecture", principal: { kind: "anonymous" }, arguments: Array.from(new TextEncoder().encode(JSON.stringify({ type: "leaderboard" }))), consistency: { kind: "authority" }, cursor: null }) });
    await pause(1_500);
    const backup = join(work, `backup-${label}.sqlite`);
    await printed(copy, ["backup", backup], env);
    return { file: size(database), wal: size(`${database}-wal`), compact: size(backup), resident: await residentKibibytes(proctor.pid!) };
  };
  const identities: [string, (index: number) => IdentityClaim][] = [
    ["anonymous", () => ({ kind: "anonymous" })],
    ["pseudonym (16 letters)", (index) => ({ kind: "pseudonym", handle: `learner ${String(index).padStart(8, "0")}` })],
    ["name (64 letters of three bytes)", (index) => ({ kind: "name", handle: `${"ẞ".repeat(56)}${String(index).padStart(8, "0")}` })],
  ];
  let before = await state("empty");
  console.log(`empty: file ${before.file} B, wal ${before.wal} B, compact copy ${before.compact} B, resident ${before.resident} KiB`);
  for (const [label, identity] of identities) {
    const started = performance.now();
    let next = 0;
    let refused = 0;
    await Promise.all(
      Array.from({ length: 32 }, async () => {
        const wire = new Wire(port);
        while (next < count) {
          const index = next++;
          const answer = await wire.post("/commands", JSON.stringify(encodeCommandEnvelope(commandEnvelope({ type: "identify-learner", id: newId(), learner: newId(), identity: identity(index) }, "architecture", Date.now()))));
          if (answer.status !== 200 || !answer.body.includes('"status":"accepted"')) {
            refused += 1;
            if (refused < 4) console.log(`  refused: ${answer.status} ${answer.body.slice(0, 200)}`);
          }
        }
        wire.close();
      }),
    );
    const seconds = (performance.now() - started) / 1000;
    const after = await state(label.split(" ")[0]!);
    const per = (now: number, then: number): string => ((now - then) / (count - refused)).toFixed(0);
    console.log(`${label}: ${count - refused} registered in ${seconds.toFixed(1)} s (${refused} refused); per registration: live file ${per(after.file + after.wal, before.file + before.wal)} B (file ${after.file} B, wal ${after.wal} B), compact copy ${per(after.compact, before.compact)} B (${after.compact} B), resident ${((after.resident - before.resident) * 1024 / (count - refused)).toFixed(0)} B (${after.resident} KiB)`);
    before = after;
  }
} finally {
  proctor.kill();
  await pause(500);
  rmSync(copy, { force: true, maxRetries: 20, retryDelay: 100 });
}
