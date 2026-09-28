/** 🧾️ C12 14c: derives the `hostTyping` vectors of the text-splice fixture from the TS reference (reviewed by hand before committing). */
import { receiveTextEditorSceneV1, refuseTextEditorSpliceV1, sendTextEditorSpliceV1, textEditorSpliceHostV1, type TextEditorSpliceHostV1 } from "./patch/tree/🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🟦️.ts";
type In = { local?: string; caret?: number; send?: true; scene?: string; applied?: number; refuse?: number };
const cases: { id: string; note: string; init: { buffer: string; applied: number }; events: In[] }[] = [
  { id: "an-echo-of-own-typing-shows-nothing", note: "the window echoes the host's own run: the editor keeps its text and caret", init: { buffer: "Hello", applied: 0 }, events: [{ local: "Hello there", caret: 11 }, { send: true }, { scene: "Hello there", applied: 1 }] },
  { id: "a-lagging-echo-never-reverts-in-flight-typing", note: "an echo of an older run while a newer one is in flight changes nothing", init: { buffer: "Hello", applied: 0 }, events: [{ local: "Hello there", caret: 11 }, { send: true }, { local: "Hello there!", caret: 12 }, { send: true }, { scene: "Hello there", applied: 1 }, { scene: "Hello there!", applied: 2 }] },
  { id: "a-collaborator-run-arrives-while-typing", note: "a collaborator's prefix arrives before the window applied the host's run: sent and unsent typing fold onto it, the caret follows", init: { buffer: "Hello", applied: 0 }, events: [{ local: "Hello there", caret: 11 }, { send: true }, { local: "Hello there you", caret: 15 }, { scene: "Oh. Hello", applied: 0 }, { scene: "Oh. Hello there", applied: 1 }] },
  { id: "both-type-at-the-same-point", note: "the window took the collaborator's Y first and then the host's X at the same point: the host shows both, its caret stays after its own X", init: { buffer: "ab", applied: 0 }, events: [{ local: "abX", caret: 3 }, { send: true }, { scene: "abY", applied: 0 }, { scene: "abXY", applied: 1 }] },
  { id: "a-refused-run-disappears", note: "the window refused the run: the editor shows the published text again", init: { buffer: "Hello", applied: 0 }, events: [{ local: "Hello there", caret: 11 }, { send: true }, { refuse: 1 }] },
  { id: "a-remounted-host-continues-the-window-sequence", note: "a host mounted on a window that applied splice 7 numbers its next run 8", init: { buffer: "abc", applied: 7 }, events: [{ local: "abcd", caret: 4 }, { send: true }, { scene: "abcd", applied: 8 }] },
];
const out = cases.map((row) => {
  let host: TextEditorSpliceHostV1 = textEditorSpliceHostV1(row.init.buffer, row.init.applied);
  let local = row.init.buffer, caret = Array.from(local).length;
  const events = row.events.map((event) => {
    if (event.local !== undefined) {
      local = event.local; caret = event.caret!;
      return { local, caret };
    }
    if (event.send) {
      const sent = sendTextEditorSpliceV1(host, local)!;
      host = sent.host;
      return { send: { seq: sent.seq, splice: sent.splice } };
    }
    const received = event.scene !== undefined ? receiveTextEditorSceneV1(host, event.scene, event.applied!, local, { anchor: caret, caret }) : refuseTextEditorSpliceV1(host, event.refuse!, local, { anchor: caret, caret });
    host = received.host;
    if (received.show) { local = received.show.text; caret = received.show.caret; }
    return { ...(event.scene !== undefined ? { scene: event.scene, applied: event.applied } : { refuse: event.refuse }), show: received.show, unapplied: host.unapplied.map((entry) => entry.seq) };
  });
  return { id: row.id, note: row.note, init: row.init, events };
});
console.log(JSON.stringify(out, null, 2));
