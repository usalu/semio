/** 🧪️ Laws of the range-text operation (`semio.ui.scene.text-splice.v1`): the schema admits the language-neutral fixture, the TS
 * reference replays every vector exactly (edit → splice, application + inverse, concurrent folds in hub order, host rebase,
 * UTF-8 ↔ scalar offsets), jsdiff (third party) derives the same changed run for every edit, and no concurrent insert-only
 * workload loses a typed scalar. */
import { describe, expect, test } from "bun:test";
import Ajv from "ajv";
import { diffChars } from "diff";
import fixture from "../../🧫️fixtures/✂️text-splice/🔣️.json";
import schema from "../../🧬️schema/✂️text-splice/🔣️.json";
import { applyTextSpliceV1, locateTextSpliceV1, rebaseTextEditsV1, receiveTextEditorSceneV1, refuseTextEditorSpliceV1, scalarOfUtf8OffsetV1, sendTextEditorSpliceV1, TEXT_SPLICE_CONTEXT_SCALARS, TEXT_SPLICE_MIN_TWO_SIDED_SCALARS, textEditorAppliedSpliceV1, textEditorSpliceHostV1, textEditorTypingV1, textSpliceFromEditV1, utf8OffsetOfScalarV1, type TextEditorSpliceHostV1, type TextSpliceV1 } from "../../✂️text-splice/🟦️.ts";

describe("text splice", () => {
  test("the schema admits the fixture and pins the constants", () => {
    const validate = new Ajv().compile(schema);
    expect(validate(fixture), JSON.stringify(validate.errors)).toBe(true);
    expect(fixture.contextScalars).toBe(TEXT_SPLICE_CONTEXT_SCALARS);
    expect(fixture.minTwoSidedScalars).toBe(TEXT_SPLICE_MIN_TWO_SIDED_SCALARS);
  });

  for (const row of fixture.edits) {
    test(`edit ${row.id}: the splice and jsdiff agree on the one changed run`, () => {
      const splice = textSpliceFromEditV1(row.previous, row.next);
      expect(splice).toEqual(row.splice);
      if (splice === null) return expect(row.previous).toBe(row.next);
      const parts = diffChars(row.previous, row.next);
      expect(parts.filter((part) => part.removed).map((part) => part.value).join("")).toBe(splice.deleted);
      expect(parts.filter((part) => part.added).map((part) => part.value).join("")).toBe(splice.insert);
      expect(applyTextSpliceV1(row.previous, splice).text).toBe(row.next);
    });
  }

  for (const row of fixture.applications) {
    test(`apply ${row.id}: lands, results and inverts exactly`, () => {
      const applied = applyTextSpliceV1(row.text, row.splice);
      expect(locateTextSpliceV1(row.text, row.splice)).toEqual(row.located);
      expect(applied.text).toBe(row.result);
      expect(applied.inverse).toEqual(row.inverse);
      if (!row.located.clamped) expect(applyTextSpliceV1(applied.text, applied.inverse).text).toBe(row.text);
    });
  }

  for (const row of fixture.concurrent) {
    test(`concurrent ${row.id}: the hub-order fold is exact and every author's surviving run stays`, () => {
      let text = row.base;
      const clamped: number[] = [];
      for (const [index, entry] of row.order.entries()) {
        const applied = applyTextSpliceV1(text, entry.splice as TextSpliceV1);
        if (applied.located.clamped) clamped.push(index);
        const undone = applyTextSpliceV1(applied.text, applied.inverse);
        if (!applied.located.clamped) expect(undone.text, `${row.id} #${index} inverse`).toBe(text);
        text = applied.text;
      }
      expect(text).toBe(row.expected.text);
      expect(clamped).toEqual(row.expected.clamped);
      for (const author of new Set(row.order.map((entry) => entry.author))) {
        const inserted = row.order.filter((entry) => entry.author === author).map((entry) => entry.splice.insert).join("");
        for (const scalar of new Set(Array.from(inserted))) expect(Array.from(text).filter((char) => char === scalar).length, `${author}'s ${JSON.stringify(scalar)}`).toBeGreaterThanOrEqual(Array.from(inserted).filter((char) => char === scalar).length - row.order.map((entry) => Array.from(entry.splice.deleted).filter((char) => char === scalar).length).reduce((sum, count) => sum + count, 0));
      }
    });
  }

  for (const row of fixture.rebases) {
    test(`rebase ${row.id}: the host view folds its unapplied splices onto the guest's text`, () => {
      expect(rebaseTextEditsV1(row.remote, row.unapplied, row.local, row.selection)).toEqual(row.expected);
    });
  }

  test("a scene declares splice typing and echoes the applied splice seq", () => {
    const validate = new Ajv().addSchema(schema, "text-splice");
    expect(validate.validate({ $ref: "text-splice#/definitions/typing" }, { mode: "splice" })).toBe(true);
    expect(validate.validate({ $ref: "text-splice#/definitions/selectionEcho" }, { start: 3, end: 3, splice: 7 })).toBe(true);
    expect(textEditorTypingV1(JSON.stringify({ fontPx: 13, typing: { mode: "splice" } }))).toEqual({ mode: "splice" });
    expect(textEditorTypingV1(JSON.stringify({ readOnly: true }))).toBeNull();
    expect(textEditorTypingV1("not json")).toBeNull();
    expect(textEditorAppliedSpliceV1(JSON.stringify({ start: 1, end: 2, splice: 7 }))).toBe(7);
    expect(textEditorAppliedSpliceV1(JSON.stringify({ start: 1, end: 2 }))).toBe(0);
  });

  test("UTF-8 offsets and scalar indices convert both ways", () => {
    for (const row of fixture.offsets) {
      expect(scalarOfUtf8OffsetV1(row.text, row.utf8)).toBe(row.scalar);
      if (new TextEncoder().encode(Array.from(row.text).slice(0, row.scalar).join("")).length === row.utf8) expect(utf8OffsetOfScalarV1(row.text, row.scalar)).toBe(row.utf8);
    }
  });

  test("no concurrent insert-only typing loses a scalar (seeded, 400 workloads)", () => {
    let seed = 0x5eed_c12;
    const random = () => ((seed = (Math.imul(seed ^ (seed >>> 15), 0x2c1b3c6d) + 0x6d2b79f5) >>> 0) / 2 ** 32);
    const alphabet = Array.from("ab ab😀\nxyz");
    for (let workload = 0; workload < 400; workload += 1) {
      const base = Array.from({ length: Math.floor(random() * 24) }, () => alphabet[Math.floor(random() * alphabet.length)]).join("");
      const views = [{ text: base, caret: Math.floor(random() * (Array.from(base).length + 1)) }, { text: base, caret: Math.floor(random() * (Array.from(base).length + 1)) }];
      const committed: TextSpliceV1[] = [];
      const typed = [0, 0];
      for (let key = 0; key < 24; key += 1) {
        const author = random() < 0.5 ? 0 : 1;
        const view = views[author]!;
        const chars = Array.from(view.text);
        const next = [...chars.slice(0, view.caret), author === 0 ? "A" : "B", ...chars.slice(view.caret)].join("");
        committed.push(textSpliceFromEditV1(view.text, next)!);
        views[author] = { text: next, caret: view.caret + 1 };
        typed[author]! += 1;
        if (random() < 0.15) {
          const synced = committed.reduce((text, splice) => applyTextSpliceV1(text, splice).text, base);
          views[author] = { text: synced, caret: locateTextSpliceV1(synced, { start: views[author]!.caret, deleted: "", insert: "", before: Array.from(next).slice(Math.max(0, views[author]!.caret - 32), views[author]!.caret).join(""), after: Array.from(next).slice(views[author]!.caret, views[author]!.caret + 32).join("") }).start };
        }
      }
      const folded = Array.from(committed.reduce((text, splice) => applyTextSpliceV1(text, splice).text, base));
      expect(folded.filter((char) => char === "A").length, `workload ${workload}`).toBe(typed[0]);
      expect(folded.filter((char) => char === "B").length, `workload ${workload}`).toBe(typed[1]);
      expect(folded.length).toBe(Array.from(base).length + typed[0]! + typed[1]!);
    }
  });

  for (const row of fixture.hostTyping) {
    test(`host ${row.id}: sends one numbered splice per run, never reverts its own typing, drops a refused run`, () => {
      let host = textEditorSpliceHostV1(row.init.buffer, row.init.applied);
      let local = row.init.buffer, caret = Array.from(local).length;
      for (const [index, event] of (row.events as readonly Record<string, unknown>[]).entries()) {
        if (typeof event.local === "string") {
          local = event.local;
          caret = event.caret as number;
        } else if (event.send !== undefined) {
          const sent = sendTextEditorSpliceV1(host, local);
          expect(sent && { seq: sent.seq, splice: sent.splice }, `${row.id} #${index}`).toEqual(event.send as { seq: number; splice: TextSpliceV1 });
          host = sent!.host;
        } else {
          const received = typeof event.scene === "string" ? receiveTextEditorSceneV1(host, event.scene, event.applied as number, local, { anchor: caret, caret }) : refuseTextEditorSpliceV1(host, event.refuse as number, local, { anchor: caret, caret });
          expect(received.show, `${row.id} #${index} show`).toEqual(event.show as never);
          host = received.host;
          expect(host.unapplied.map((entry) => entry.seq), `${row.id} #${index} unapplied`).toEqual(event.unapplied as number[]);
          if (received.show !== null) ({ text: local, caret } = received.show);
        }
      }
    });
  }

  test("two splice-typing hosts over two replicas converge on the hub order and never lose or hide a typed scalar (seeded, 300 sessions)", () => {
    let seed = 0xc12_14c;
    const random = () => ((seed = (Math.imul(seed ^ (seed >>> 15), 0x2c1b3c6d) + 0x6d2b79f5) >>> 0) / 2 ** 32);
    const pick = <T,>(items: readonly T[]): T => items[Math.floor(random() * items.length)]!;
    type Op = { readonly author: number; readonly seq: number; readonly splice: TextSpliceV1 };
    for (let session = 0; session < 300; session += 1) {
      const initial = Array.from({ length: Math.floor(random() * 16) }, () => pick(Array.from("ab c\n😀"))).join("");
      const hub: Op[] = [];
      const replicas = [0, 1].map(() => ({ known: 0, inbox: [] as Op[], pending: [] as Op[], outbox: [] as Op[] }));
      const hosts = [0, 1].map(() => ({ host: textEditorSpliceHostV1(initial, 0) as TextEditorSpliceHostV1, local: initial, caret: Math.floor(random() * (Array.from(initial).length + 1)), typed: "" }));
      const letters = ["A", "B"];
      const replicaText = (author: number) => [...hub.slice(0, replicas[author]!.known), ...replicas[author]!.pending].reduce((text, op) => applyTextSpliceV1(text, op.splice).text, initial);
      const applied = (author: number) => Math.max(0, ...hub.slice(0, replicas[author]!.known).filter((op) => op.author === author).map((op) => op.seq), ...replicas[author]!.pending.map((op) => op.seq));
      const publish = (author: number) => {
        const view = hosts[author]!;
        const received = receiveTextEditorSceneV1(view.host, replicaText(author), applied(author), view.local, { anchor: view.caret, caret: view.caret });
        view.host = received.host;
        if (received.show !== null) ({ text: view.local, caret: view.caret } = received.show);
        for (const letter of Array.from(view.typed)) expect(Array.from(view.local).filter((char) => char === letter).length, `session ${session}: host ${author} shows every ${letter} it typed`).toBe(view.typed.length);
      };
      const step = (action: number) => {
        const author = action % 2, view = hosts[author]!, replica = replicas[author]!;
        switch (Math.floor(action / 2)) {
          case 0: {
            const chars = Array.from(view.local);
            view.local = [...chars.slice(0, view.caret), letters[author]!, ...chars.slice(view.caret)].join("");
            view.caret += 1;
            view.typed += letters[author]!;
            return;
          }
          case 1: {
            const sent = sendTextEditorSpliceV1(view.host, view.local);
            if (sent === null) return;
            view.host = sent.host;
            replica.inbox.push({ author, seq: sent.seq, splice: sent.splice });
            return;
          }
          case 2: {
            const op = replica.inbox.shift();
            if (op === undefined) return;
            replica.pending.push(op);
            replica.outbox.push(op);
            return publish(author);
          }
          case 3: {
            const op = replica.outbox.shift();
            if (op !== undefined) hub.push(op);
            return;
          }
          default: {
            if (replica.known === hub.length) return;
            const op = hub[replica.known]!;
            replica.known += 1;
            if (op.author === author) replica.pending = replica.pending.filter((entry) => entry.seq !== op.seq);
            return publish(author);
          }
        }
      };
      for (let turn = 0; turn < 60; turn += 1) step(Math.floor(random() * 10));
      for (let drain = 0; drain < 400 && (replicas.some((replica) => replica.inbox.length > 0 || replica.outbox.length > 0 || replica.known < hub.length) || hosts.some((view) => textSpliceFromEditV1(view.host.base, view.local) !== null)); drain += 1) step(2 + (drain % 8));
      const final = hub.reduce((text, op) => applyTextSpliceV1(text, op.splice).text, initial);
      for (const author of [0, 1]) {
        expect(hosts[author]!.local, `session ${session}: host ${author} converged`).toBe(final);
        expect(Array.from(final).filter((char) => char === letters[author]).length, `session ${session}: every ${letters[author]} survives`).toBe(hosts[author]!.typed.length);
      }
    }
  });
});
