/** 👥️ Runtime smoke of shared presence across languages: a private copy of the built Rust `proctor` binary serves the
 * fixture catalog, and two Bun WebSocket clients speak `semio.presence.v1` through the TypeScript twin's
 * `presenceSocketUrl` and frame codec. Usage: `bun proctor_presence_smoke.ts <proctor executable>`. */
import { copyFileSync, mkdirSync, rmSync } from "node:fs";
import { join, resolve } from "node:path";
import { PRESENCE_PROTOCOL, decodePresenceFrame, encodePresenceFrame, presenceSocketUrl, type PresenceFrame } from "../../../../../../../🧰️framework/🛍️products/🖥️server/🟦️.ts";

const repo = resolve(import.meta.dir, "../../../../../../..");
const generated = join(import.meta.dir, "🗑️generated", "proctor", "presence-smoke");
const port = "18793";
const base = `http://127.0.0.1:${port}`;
const origin = "http://localhost:6061";

rmSync(generated, { recursive: true, force: true });
mkdirSync(join(generated, "data"), { recursive: true });
const executable = join(generated, process.platform === "win32" ? "proctor.exe" : "proctor");
copyFileSync(process.argv[2]!, executable);
const proctor = Bun.spawn([executable, "serve"], {
  env: { ...process.env, PROCTOR_PORT: port, PROCTOR_DATA: join(generated, "data"), PROCTOR_CATALOG: join(repo, "🎓️teaching/🛂️proctor/🧫️fixtures/📚️catalog/🔣️.json"), PROCTOR_PRESENCE_TICK_MS: "50" },
  stderr: "pipe",
});

async function listening(): Promise<void> {
  const reader = proctor.stderr.getReader();
  let log = "";
  while (!log.includes("listening")) {
    const { value, done } = await reader.read();
    if (done) throw new Error(`proctor exited: ${log}`);
    log += new TextDecoder().decode(value);
  }
  console.log(`[DEBUG] ${log.trim().split("\n").join("\n[DEBUG] ")}`);
  reader.releaseLock();
}

class Client {
  readonly frames: PresenceFrame[] = [];
  private waiting: (() => void)[] = [];
  readonly socket: WebSocket;

  constructor(readonly name: string, scope: string, surface: string) {
    this.socket = new WebSocket(presenceSocketUrl(base, scope, surface), { protocols: [PRESENCE_PROTOCOL], headers: { origin } } as unknown as string[]);
    this.socket.onmessage = (event) => {
      const frame = decodePresenceFrame(JSON.parse(String(event.data)));
      console.log(`[DEBUG] ${this.name} <- ${JSON.stringify(encodePresenceFrame(frame))}`);
      this.frames.push(frame);
      for (const wake of this.waiting.splice(0)) wake();
    };
  }

  async until(found: (frame: PresenceFrame) => boolean, label: string): Promise<PresenceFrame> {
    const deadline = Date.now() + 5000;
    for (;;) {
      const hit = this.frames.find(found);
      if (hit) return hit;
      if (Date.now() > deadline) throw new Error(`${this.name} never saw ${label}`);
      await Promise.race([new Promise<void>((wake) => this.waiting.push(wake)), Bun.sleep(100)]);
    }
  }

  share(state: unknown): void {
    this.socket.send(JSON.stringify(encodePresenceFrame({ type: "state", state })));
  }
}

try {
  await listening();
  const ada = new Client("ada", "proctor-fixture", "home");
  const welcome = await ada.until((frame) => frame.type === "welcome", "welcome");
  if (welcome.type !== "welcome" || ada.socket.protocol !== PRESENCE_PROTOCOL) throw new Error(`bad welcome ${ada.socket.protocol}`);
  const grace = new Client("grace", "proctor-fixture", "home");
  const graceWelcome = await grace.until((frame) => frame.type === "welcome", "welcome");
  if (graceWelcome.type !== "welcome" || graceWelcome.roster.length !== 2) throw new Error("grace's roster lacks ada");
  const atHome = { tag: "0a1b2c3d", identity: { kind: "pseudonym", handle: "Ada" }, place: { screen: "home" }, active: true };
  ada.share(atHome);
  await grace.until((frame) => frame.type === "batch" && frame.entries.some((entry) => entry.session === welcome.session && Bun.deepEquals(entry.state, atHome)), "ada at home");
  grace.share({ tag: "0a1b2c3d", identity: { kind: "anonymous" }, place: { screen: "run", quiz: "cooling" }, active: true });
  const refused = await grace.until((frame) => frame.type === "refused", "a refusal");
  if (refused.type !== "refused" || refused.reason !== "quiz-unknown /place/quiz") throw new Error(`unexpected refusal ${JSON.stringify(refused)}`);
  const thinking = "proctor-fixture/quiz/power/thinking";
  grace.socket.send(JSON.stringify(encodePresenceFrame({ type: "watch", scopes: [thinking], intervalMs: 200 })));
  await grace.until((frame) => frame.type === "watched" && frame.scope === thinking && frame.snapshot === true && frame.entries.length === 0, "an empty thinking snapshot");
  const thinker = new Client("thinker", thinking, "run");
  const thinkerWelcome = await thinker.until((frame) => frame.type === "welcome", "welcome");
  if (thinkerWelcome.type !== "welcome") throw new Error("no welcome");
  const draft = { tag: "0a1b2c3d", answers: { sources: { kind: "matching", values: { hours: { "rooftop-pv": 950 } } } } };
  thinker.share(draft);
  await grace.until((frame) => frame.type === "watched" && frame.scope === thinking && !frame.snapshot && frame.entries.some((entry) => entry.session === thinkerWelcome.session && Bun.deepEquals(entry.state, draft)), "the thinker's draft, watched");
  thinker.socket.close();
  await grace.until((frame) => frame.type === "watched" && frame.left.includes(thinkerWelcome.session), "the thinker leaving, watched");
  ada.socket.close();
  await grace.until((frame) => frame.type === "batch" && frame.left.includes(welcome.session), "ada leaving");
  grace.socket.close();
  console.log("[DEBUG] presence smoke passed");
} finally {
  proctor.kill();
  await proctor.exited;
  rmSync(executable, { force: true });
}
