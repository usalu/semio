const root = process.argv[2]!;
const { createScriptProcessEnvelope, withScriptProcessEnvelope } = await import(`${root}/🧰️framework/🔨️modules/🏃️process/🧭️routing/📥️invocation/🏃️process/🟦️.ts`);
const { startNativeProgress } = await import(`${root}/🧰️framework/🔨️modules/🏃️process/🎛️owned-execution/🟦️.ts`);
const stop = startNativeProgress("demo:wait", 700);
await withScriptProcessEnvelope(createScriptProcessEnvelope({ version: 1, owner: "demo", maximumElapsedMilliseconds: 0 }, {}, Date.now()), async invocation => {
  await new Promise<void>((_done, fail) => invocation.control.signal.addEventListener("abort", () => fail(invocation.control.signal.reason)));
});
stop();
