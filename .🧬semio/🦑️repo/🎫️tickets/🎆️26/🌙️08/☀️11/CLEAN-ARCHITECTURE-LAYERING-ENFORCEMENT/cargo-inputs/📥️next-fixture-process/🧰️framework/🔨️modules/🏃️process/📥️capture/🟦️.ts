import {openSync,writeSync,closeSync,readFileSync} from "node:fs";
import {spawn} from "node:child_process";
import {terminateOwnedProcessTree} from "../🪓️termination/🟦️.ts";

export type OwnedProcessInput = Readonly<{source:Uint8Array;maximumBytes:number}>;
export type OwnedProcessInputReceipt = Readonly<{queuedBytes:number;state:"complete"|"closed"|"refused"}>;
export type OwnedProcessCaptureResult = { status: number | null; signal: string | null; stdout: string; stderr: string } & { input?:OwnedProcessInputReceipt;reason?: "exit" | "timeout" | "cancelled" | "output-limit" | "spawn-error" };
export type OwnedProcessCaptureOptions = {
  cwd: string;
  env: Readonly<Record<string, string | undefined>>;
  budgetMs: number;
  maxOutputBytes: number;
  stdoutPath: string;
  stderrPath: string;
  cancelled: () => boolean;
  input?: OwnedProcessInput;
};
/** 📥️ Captures a process into caller-owned evidence files; zero disables its deadline while retaining cancellation and output limits. */
export async function captureOwnedProcess(command: string, args: string[], options: OwnedProcessCaptureOptions): Promise<OwnedProcessCaptureResult> {
  if(options.input!==undefined&&(options.input===null||typeof options.input!=="object"||!(options.input.source instanceof Uint8Array)||Object.keys(options.input).some(key=>key!=="source"&&key!=="maximumBytes")||!Number.isSafeInteger(options.input.maximumBytes)||options.input.maximumBytes<0||options.input.source.byteLength>options.input.maximumBytes))throw Error("Owned process input exceeds finite admission");
  const stdout = openSync(options.stdoutPath, "wx", 0o600);
  let stderr: number;
  try { stderr = openSync(options.stderrPath, "wx", 0o600); }
  catch (error) { closeSync(stdout); throw error; }
  if (options.cancelled()) {
    closeSync(stdout); closeSync(stderr);
    return { status: null, signal: null, reason: "cancelled", stdout: "", stderr: "" };
  }
  return await new Promise((resolveResult) => {
    let reason: OwnedProcessCaptureResult["reason"] = "exit";
    let count = 0;
    let finished = false;
    const child = spawn(command, args, { cwd: options.cwd, env: options.env, stdio: [options.input ? "pipe" : "ignore", "pipe", "pipe"], detached: process.platform !== "win32", windowsHide: true });
    let inputStopped=false,inputComplete=false,inputQueuedBytes=0,wakeInput:(()=>void)|null=null;
    const stopInput=():void=>{inputStopped=true;child.stdin?.destroy();wakeInput?.();wakeInput=null;};
    const terminate = (cause: NonNullable<OwnedProcessCaptureResult["reason"]>): void => {
      if (finished || reason !== "exit") return;
      reason = cause;
      stopInput();
      if (child.pid) terminateOwnedProcessTree(child.pid);
    };
    const append = (descriptor: number, bytes: Buffer): void => {
      const remaining = Math.max(0, options.maxOutputBytes - count);
      if (remaining) writeSync(descriptor, bytes.subarray(0, remaining));
      count += bytes.length;
      if (count > options.maxOutputBytes) terminate("output-limit");
    };
    child.stdin?.on("error",(error:NodeJS.ErrnoException)=>{stopInput();if(error.code!=="EPIPE"&&error.code!=="ERR_STREAM_DESTROYED")terminate("spawn-error");});
    child.stdout?.on("data", (bytes) => append(stdout, bytes));
    child.stderr?.on("data", (bytes) => append(stderr, bytes));
    const timer = options.budgetMs > 0 ? setTimeout(() => terminate("timeout"), options.budgetMs) : undefined;
    const cancel = setInterval(() => {
      if (options.cancelled()) terminate("cancelled");
    }, 100);
    child.on("error", (error) => {
      if(reason=== "exit")reason = "spawn-error";
      stopInput();
      append(stderr, Buffer.from(error.message));
    });
    const inputTask=(async()=>{
      const source=options.input?.source;if(!source||!child.stdin)return;
      for(let offset=0;offset<source.byteLength;){
        if(inputStopped||options.cancelled()){if(options.cancelled())terminate("cancelled");return;}
        const end=Math.min(offset+4096,source.byteLength),chunk=Buffer.from(source.subarray(offset,end));
        const writable=child.stdin.write(chunk);inputQueuedBytes+=chunk.byteLength;offset=end;
        if(!writable)await new Promise<void>(resume=>{const finish=():void=>{child.stdin?.off("drain",finish);wakeInput=null;resume();};wakeInput=finish;child.stdin!.once("drain",finish);if(inputStopped)finish();});
        else await new Promise<void>(resume=>setImmediate(resume));
      }
      if(!inputStopped)await new Promise<void>(resume=>{const finish=():void=>{wakeInput=null;inputComplete=!inputStopped;resume();};wakeInput=()=>resume();child.stdin!.end(finish);if(inputStopped)resume();});
    })().catch(()=>{stopInput();terminate("spawn-error");});
    child.on("close", async(status, signal) => {
      if (finished) return;
      finished = true;
      clearTimeout(timer);
      clearInterval(cancel);
      stopInput();await inputTask;
      closeSync(stdout);
      closeSync(stderr);
      resolveResult({ status, signal, reason, ...(options.input?{input:{queuedBytes:inputQueuedBytes,state:inputComplete?"complete":inputQueuedBytes?"closed":"refused"}}:{}), stdout: readFileSync(options.stdoutPath, "utf8"), stderr: readFileSync(options.stderrPath, "utf8") });
    });
  });
}
