import {spawn, spawnSync} from "node:child_process";

/** 📟️ What a captured command did: exit status (124 on timeout), the tail of stdout and stderr and its wall time. */
export type CapturedV1 = {readonly exit: number; readonly output: string; readonly seconds: number; readonly timedOut: boolean};

const TAIL_BYTES = 65536;

/** ⏲️ Runs a command with bounded memory (only the output tail is kept) and a deadline; on timeout the whole process tree is killed (a process group on POSIX, `taskkill /T` on Windows). */
export function captureV1(command: readonly string[], options: {readonly cwd: string; readonly env: Readonly<Record<string, string | undefined>>; readonly timeoutMs: number; readonly signal?: AbortSignal}): Promise<CapturedV1> {
  return new Promise((resolve) => {
    const started = Date.now(), windows = process.platform === "win32";
    const child = spawn(command[0]!, command.slice(1), {cwd: options.cwd, env: options.env as NodeJS.ProcessEnv, detached: !windows, stdio: ["ignore", "pipe", "pipe"]});
    let tail = "", timedOut = false;
    const keep = (chunk: Buffer) => { tail = (tail + chunk.toString("utf8")).slice(-TAIL_BYTES); };
    child.stdout.on("data", keep);
    child.stderr.on("data", keep);
    const stop = () => {
      timedOut = true;
      if (windows) spawnSync("taskkill", ["/T", "/F", "/PID", String(child.pid)]);
      else try { process.kill(-child.pid!, "SIGKILL"); } catch { child.kill("SIGKILL"); }
    };
    const timer = setTimeout(stop, options.timeoutMs);
    options.signal?.addEventListener("abort", stop, {once: true});
    const finish = (exit: number, extra = "") => { clearTimeout(timer); resolve({exit: timedOut ? 124 : exit, output: tail + extra, seconds: Math.round((Date.now() - started) / 100) / 10, timedOut}); };
    child.on("error", (error) => finish(127, `\n${error.message}`));
    child.on("close", (code, signal) => finish(code ?? (signal ? 128 : 1)));
  });
}
