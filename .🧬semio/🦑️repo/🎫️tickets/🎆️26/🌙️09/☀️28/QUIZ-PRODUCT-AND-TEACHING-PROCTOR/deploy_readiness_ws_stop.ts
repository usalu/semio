/** 🛑️ Deploy-readiness probe: does `docker stop` drain the proctor while a presence WebSocket is still open?
 * `bun deploy_readiness_ws_stop.ts [image] [port]` runs the image on a loopback port with a throw-away volume, opens one
 * presence socket as the site origin, stops the container and prints the drain time and exit code; everything it creates
 * is removed. */
import { spawnSync } from "node:child_process";

const image = process.argv[2] ?? "ghcr.io/usalu/architecture-quiz-proctor:latest";
const port = process.argv[3] ?? "18795";
const name = `deploy-readiness-ws-stop-${Date.now().toString(36)}`;
const docker = (...args: string[]): string => {
  const run = spawnSync("docker", args, { encoding: "utf8" });
  if (run.status !== 0) throw new Error(`docker ${args.join(" ")}: ${run.stderr}`);
  return run.stdout.trim();
};
const headers = { "x-forwarded-proto": "https", origin: "https://quizzes.architektur-und-technologie.de" };
try {
  docker("run", "--detach", "--name", name, "--publish", `127.0.0.1:${port}:8791`, "--volume", `${name}:/srv/quiz/data`, image);
  for (let attempt = 0; attempt < 60; attempt++) {
    const ready = await fetch(`http://127.0.0.1:${port}/instance`, { headers }).then((answer) => answer.status === 200, () => false);
    if (ready) break;
    await Bun.sleep(500);
  }
  const socket = new WebSocket(`ws://127.0.0.1:${port}/scopes/architecture/presence/ws?surface=probe`, { protocols: ["semio.presence.v1"], headers } as unknown as string[]);
  const closed = new Promise<string>((accept) => socket.addEventListener("close", (event) => accept(`close ${event.code} ${JSON.stringify(event.reason)}`)));
  await new Promise<void>((accept, reject) => {
    socket.addEventListener("open", () => accept(), { once: true });
    socket.addEventListener("error", () => reject(new Error("the presence socket did not open")), { once: true });
  });
  console.log("[DEBUG] presence socket open; stopping the container");
  const started = Date.now();
  docker("stop", "--time", "30", name);
  console.log(`[DEBUG] docker stop returned after ${Date.now() - started} ms, exit code ${docker("inspect", "--format", "{{.State.ExitCode}}", name)}`);
  console.log(`[DEBUG] socket: ${await Promise.race([closed, Bun.sleep(2000).then(() => "still open")])}`);
  console.log(`[DEBUG] log tail:\n${spawnSync("docker", ["logs", "--tail", "6", name], { encoding: "utf8" }).stderr}`);
} finally {
  spawnSync("docker", ["rm", "--force", name]);
  spawnSync("docker", ["volume", "rm", "--force", name]);
}
