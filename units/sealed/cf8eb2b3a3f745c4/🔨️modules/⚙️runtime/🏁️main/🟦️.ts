import { realpathSync } from "node:fs";
import { fileURLToPath, pathToFileURL } from "node:url";

/** 🏁️ Tells whether the module at `metaUrl` is the process entry point. */
export function isMainModuleV1(metaUrl: string): boolean {
  const entry = process.argv[1];
  if (entry === undefined || entry === "") return false;
  const canonical = (path: string): string => { const real = pathToFileURL(realpathSync(path)).href; return process.platform === "win32" ? real.toLowerCase() : real; };
  try { return canonical(entry) === canonical(fileURLToPath(metaUrl)); } catch { return false; }
}

/** 🛑️ The signals that ask a process to stop. Node delivers `SIGINT` (Ctrl+C) everywhere and `SIGTERM` on POSIX hosts only; a Windows process hears the console instead: `SIGBREAK` (Ctrl+Break) and `SIGHUP` (the console window closes). An external `taskkill` without `/F` posts the same close event, so listening to all four is what makes cancellation reach a Windows child at all. */
export function terminationSignalsV1(platform: string = process.platform): readonly NodeJS.Signals[] {
  return platform === "win32" ? ["SIGINT", "SIGTERM", "SIGBREAK", "SIGHUP"] : ["SIGINT", "SIGTERM"];
}

/** 🛑️ Runs `handler` once when a stop signal arrives and answers the function that removes the listeners again. */
export function onTerminationV1(handler: (signal: NodeJS.Signals) => void, platform: string = process.platform, target: Pick<NodeJS.Process, "once" | "removeListener"> = process): () => void {
  const listeners = terminationSignalsV1(platform).map((signal) => [signal, () => handler(signal)] as const);
  for (const [signal, listener] of listeners) target.once(signal, listener);
  return () => { for (const [signal, listener] of listeners) target.removeListener(signal, listener); };
}

/** 🔇️ The error codes of a standard stream whose reader is gone (a closed pipe, a terminal that went away): the writer has nobody to tell, so it is not a failure of the program. */
const unreadableStreamCodesV1:ReadonlySet<string>=new Set(["EPIPE","ERR_STREAM_DESTROYED"]);

function standardStreamGuardV1(error:NodeJS.ErrnoException):void{if(!unreadableStreamCodesV1.has(error.code??""))throw error;}

/** 🛡️ Installs, once per stream, the one `error` listener that drops a write to a stream nobody reads any more (`EPIPE`), so a build driver whose output reader exited keeps building instead of dying in a log call; any other stream error is raised as it would be without the guard. Call it at the entry of a program, never at a write site. Returns whether this call installed it on any stream. */
export function guardStandardStreamsV1(streams:readonly Pick<NodeJS.WriteStream,"on"|"listeners">[]=[process.stdout,process.stderr]):boolean{
  let installed=false;
  for(const stream of streams){if(stream.listeners("error").includes(standardStreamGuardV1))continue;stream.on("error",standardStreamGuardV1);installed=true;}
  return installed;
}
