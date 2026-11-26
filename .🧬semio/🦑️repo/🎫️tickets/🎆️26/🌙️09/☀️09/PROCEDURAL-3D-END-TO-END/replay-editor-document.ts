import { DocumentArchiveLoadHost, documentArchiveLoadReplyV1 } from "../../../../../../../🧰️framework/🔨️modules/🧾️app-channel/🗃️archive-load-host/🟦️.ts";
import type { AppCommandValue, AppFrameValue, DocumentArchivePack } from "@semio-tech/framework/wire-contracts";

/** 🗃️ Applies an actual emitted archive through its native admission, observation and acknowledgment commands. */
export async function replayEditorDocument(
  archive: DocumentArchivePack, next: () => number,
  exchange: (command: AppCommandValue, seq: number) => Promise<AppFrameValue[]>,
  advance: () => Promise<void>,
): Promise<void> {
  const host = new DocumentArchiveLoadHost(archive);
  let previous = "";
  for (let i = 0; i < 20000; i++) {
    const step = host.step(next);
    if (step.kind === "finished") {
      if (step.outcome.kind !== "ready") throw Error("Archive ended " + JSON.stringify(step.outcome));
      return;
    }
    const frames = await exchange(step.command, step.seq);
    const reply = documentArchiveLoadReplyV1(frames, step.seq);
    if (!reply) throw Error("Archive command has no correlated native response " + step.seq);
    const answer = host.answer(step.seq, reply);
    if (answer.kind !== "status") throw Error("Archive command refused " + JSON.stringify(answer));
    const status = answer.status;
    if (!status) continue;
    const detail = JSON.stringify([status.state, status.completed, status.total, status.ahead]);
    if (detail !== previous) console.log("[DEBUG] Archive progress", i, detail);
    previous = detail;
    if (status.state !== "ready" && status.state !== "fault" && status.state !== "cancelled") await advance();
  }
  throw Error("Native archive did not settle within the probe limit");
}
