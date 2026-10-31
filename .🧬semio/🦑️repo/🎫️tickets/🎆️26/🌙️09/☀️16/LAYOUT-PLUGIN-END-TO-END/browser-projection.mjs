/** 🔁️ Observes a fresh asynchronous renderer projection until its required condition is true. */
export async function waitForProjection(label, read, accept, { now = Date.now, pause, timeout = 15000, interval = 50 } = {}) {
  const until = now() + timeout;
  let observed;
  while (now() < until) {
    observed = await read();
    if (await accept(observed)) return observed;
    await pause(interval);
  }
  throw Error(label + " projection timed out: " + JSON.stringify(observed));
}
