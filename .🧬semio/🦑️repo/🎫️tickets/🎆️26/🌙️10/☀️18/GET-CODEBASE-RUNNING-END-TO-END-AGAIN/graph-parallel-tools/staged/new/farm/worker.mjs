import { readFileSync } from "node:fs";
import { deserialize } from "node:v8";

/** 👷️ The worker process of the graph inference farm: loads the service module its parent names, opens one session and runs jobs one at a time, answering every job with plain data. */
const MEGABYTE = 1048576;
const send = message => { try { process.send(message); } catch {} };
let service;

process.on("disconnect", () => process.exit(0));
process.on("message", async message => {
  try {
    if (message?.type === "init") {
      const url = new URL(message.service);
      if (url.protocol !== "file:") throw Error("File-addressed farm service required");
      service = await import(message.service);
      const identity = await service.open(deserialize(readFileSync(message.sessionFile)));
      send({ type: "ready", pid: process.pid, identity });
    } else if (message?.type === "run") {
      try {
        const reply = await service.run(message.job);
        send({ type: "result", id: message.id, ...reply, rssMegabytes: process.memoryUsage().rss / MEGABYTE });
      } catch (error) {
        send({ type: "failure", id: message.id, name: String(error?.name ?? "Error"), message: String(error?.message ?? error), code: typeof error?.code === "string" ? error.code : null });
      }
    } else if (message?.type === "shutdown") {
      await service?.close?.();
      process.exit(0);
    }
  } catch (error) {
    send({ type: "fatal", name: String(error?.name ?? "Error"), message: String(error?.message ?? error) });
    process.exit(1);
  }
});
