import { readFileSync } from "node:fs";
import { deserialize } from "node:v8";

/** 👷️ The worker process of the graph farm: loads the service modules its parent names, opens one session per service kind (again for every later build) and runs jobs one at a time, answering every job with plain data. */
const MEGABYTE = 1048576;
const send = message => { try { process.send(message); } catch (error) { process.stderr.write("[graph-farm] a message could not be sent: " + String(error?.message ?? error) + "\n"); } };
const services = {};
const file = href => { if (new URL(href).protocol !== "file:") throw Error("File-addressed farm service required"); return href; };

process.on("disconnect", () => process.exit(0));
process.on("message", async message => {
  try {
    if (message?.type === "load") {
      for (const [kind, href] of Object.entries(message.services)) services[kind] ??= await import(file(href));
      send({ type: "loaded", pid: process.pid });
    } else if (message?.type === "open") {
      services[message.kind] ??= await import(file(message.service));
      const identity = await services[message.kind].open(deserialize(readFileSync(message.sessionFile)));
      send({ type: "opened", kind: message.kind, token: message.token, identity });
    } else if (message?.type === "run") {
      try {
        const reply = await services[message.kind].run(message.job);
        process.send({ type: "result", id: message.id, ...reply, rssMegabytes: process.memoryUsage().rss / MEGABYTE });
      } catch (error) {
        send({ type: "failure", id: message.id, name: String(error?.name ?? "Error"), message: String(error?.message ?? error), code: typeof error?.code === "string" ? error.code : null });
      }
    } else if (message?.type === "shutdown") {
      for (const service of Object.values(services)) await service.close?.();
      process.exit(0);
    }
  } catch (error) {
    send({ type: "fatal", name: String(error?.name ?? "Error"), message: String(error?.message ?? error) });
    process.exit(1);
  }
});
