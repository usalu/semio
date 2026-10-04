/** 🎙️ Ticket tool of work package C1a: `@semio-tech/pets` with `advance` and `frameOf` wrapped, so the harness page can read back what the layer handed the stage (`window.petRecord`, every event but `ticked`, `pointed` and `glanced`) and the last frame (`window.petFrame`). */
import { advance as advanceStage, frameOf as frameOfStage, type Frame, type Menagerie, type Stage, type StageEvent } from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/🟦️.ts";

export * from "../../../../../../../../🧰️framework/🛍️products/🐾️pets/📦️packages/🟦️typescript/🟦️.ts";

const QUIET_KINDS = new Set(["ticked", "pointed", "glanced"]);
const record: StageEvent[] = [];
Object.assign(window, { petRecord: record, petFrame: null });

/** 📼️ The stage's own `advance`, after noting every event worth reading back. */
export function advance(menagerie: Menagerie, stage: Stage, events: readonly StageEvent[]): Stage {
  for (const event of events) if (!QUIET_KINDS.has(event.kind)) record.push(event);
  return advanceStage(menagerie, stage, events);
}

/** 🖼️ The stage's own `frameOf`, after keeping the frame for the page. */
export function frameOf(menagerie: Menagerie, stage: Stage): Frame {
  const frame = frameOfStage(menagerie, stage);
  Object.assign(window, { petFrame: frame });
  return frame;
}
