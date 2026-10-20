// 🧪️ Behavioural equivalence of the wasi module before and after the ECMAScript-private-fields fix: the same scripted scenario against both files must trace identically.
const [original, fixed] = process.argv.slice(2) as [string, string];
async function trace(path: string): Promise<unknown[]> {
  const {createBrowserWasiActivation} = await import(path), out: unknown[] = [], writes: unknown[] = [];
  let failing = false, clock = 0;
  const activation = createBrowserWasiActivation({nowNs: () => BigInt(clock += 1_000_000), wallNs: () => 1_700_000_000_000_000_000n, write(channel: string, bytes: Uint8Array) {if (failing) throw new Error("boom"); writes.push([channel, [...bytes]]);}, exit() {}} as never);
  const i = activation.imports as Record<string, any>;
  const stdout = i["wasi:cli/stdout@0.2.0"].getStdout(), stderr = i["wasi:cli/stderr@0.2.0"].getStderr();
  out.push(["permit", String(stdout.checkWrite())]);
  stdout.write(new Uint8Array([1, 2, 3])); stdout.flush();
  await stdout.blockingWriteAndFlush(new Uint8Array([4, 5]));
  try { stdout.write(new Uint8Array(3)); } catch (error) { out.push(["permit-refusal", String((error as Error).message)]); }
  const pollable = stdout.subscribe(); out.push(["ready-after-flush", pollable.ready()]);
  const timer = i["wasi:clocks/monotonic-clock@0.2.0"].subscribeDuration(0n); out.push(["timer-ready", timer.ready()]);
  const timer2 = i["wasi:clocks/monotonic-clock@0.2.0"].subscribeDuration(10_000_000n); out.push(["timer2-ready-before", timer2.ready()]);
  failing = true; stderr.checkWrite();
  try { stderr.write(new Uint8Array([9])); } catch (error: any) { out.push(["sink-failure", error.tag, error.val?.toDebugString?.()]); }
  try { stderr.checkWrite(); } catch (error: any) { out.push(["after-failure", error.tag]); }
  out.push(["writes", writes], ["progress", activation.progress()]);
  await activation.close(); out.push(["closed", activation.progress()]);
  try { stdout.checkWrite(); } catch (error) { out.push(["after-close", String((error as Error).message)]); }
  return out;
}
const a = JSON.stringify(await trace(original)), b = JSON.stringify(await trace(fixed));
console.log(a === b ? "IDENTICAL " + a.length + " bytes of trace" : "DIFFERENT\n" + a + "\n" + b);
process.exit(a === b ? 0 : 1);
