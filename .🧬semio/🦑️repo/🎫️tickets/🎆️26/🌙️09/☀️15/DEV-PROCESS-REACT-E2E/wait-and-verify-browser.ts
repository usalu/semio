import { expect } from "./browser-tooling";

const target = new URL(process.argv[2] ?? "http://127.0.0.1:6022/");
console.log("[DEBUG] Waiting for the process editor server");
await expect.poll(async () => {
  try {
    const response = await fetch(target, { signal: AbortSignal.timeout(3000) });
    await response.body?.cancel();
    return response.ok;
  } catch {
    return false;
  }
}, { intervals: [1000, 5000, 10000], timeout: 14400000, message: "The Process 3D dev server is reachable" }).toBe(true);
console.log("[DEBUG] Process editor server responded. Running the current browser probe.");
await import("./verify-browser.ts");
