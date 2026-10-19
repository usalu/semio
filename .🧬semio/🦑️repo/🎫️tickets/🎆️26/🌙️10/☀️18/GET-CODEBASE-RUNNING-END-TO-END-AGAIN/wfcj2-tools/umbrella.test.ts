import { test } from "bun:test";
const m = await import(process.env.UMB!);
test("umbrella", () => {
  try { console.log("[RESULT]", m.toolJobCoverageSelfTests()); } catch (e) { console.log("[FAILMSG]", (e as Error).message.slice(0, 800)); }
});
