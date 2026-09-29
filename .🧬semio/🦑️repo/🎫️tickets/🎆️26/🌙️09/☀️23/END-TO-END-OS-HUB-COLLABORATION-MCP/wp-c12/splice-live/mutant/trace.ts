import { applyTextSpliceV1, receiveTextEditorSceneV1, refuseTextEditorSpliceV1, sendTextEditorSpliceV1, settleTextEditorSpliceV1, textEditorSpliceHostV1, textSpliceFromEditV1, type TextEditorSpliceHostV1, type TextEditorSpliceViewV1, type TextSpliceV1 } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🎬️scene/✂️text-splice/🟦️.ts";
const log = (...args: unknown[]) => console.log(...args.map((a) => String(a).replaceAll("\n", "⏎")));
const expect = (actual: unknown, message?: string) => ({ toBe: (expected: unknown) => { if (actual !== expected) console.log("MISMATCH", message, JSON.stringify(actual), JSON.stringify(expected)); } });

    let seed = 0xc12_29a;
    const random = () => ((seed = (Math.imul(seed ^ (seed >>> 15), 0x2c1b3c6d) + 0x6d2b79f5) >>> 0) / 2 ** 32);
    const pick = <T,>(items: readonly T[]): T => items[Math.floor(random() * items.length)]!;
    type Op = { readonly author: number; readonly seq: number; readonly splice: TextSpliceV1 };
    const count = (text: string, letter: string) => Array.from(text).filter((char) => char === letter).length;
    let reincarnations = 0, lostAcknowledgements = 0;
    for (let session = 0; session < 2; session += 1) {
      const trace = session === 1;
      const initial = Array.from({ length: Math.floor(random() * 16) }, () => pick(Array.from("ab c\n😀"))).join("");
      const hub: Op[] = [];
      const replicas = [0, 1].map(() => ({ known: 0, applied: 0, unpublished: false, inbox: [] as Op[], pending: [] as Op[], outbox: [] as Op[], settles: [] as Op[] }));
      const hosts = [0, 1].map(() => ({ host: textEditorSpliceHostV1(initial, 0) as TextEditorSpliceHostV1, local: initial, caret: Math.floor(random() * (Array.from(initial).length + 1)), typed: 0, lost: 0 }));
      const letters = ["A", "B"];
      const replicaText = (author: number) => [...hub.slice(0, replicas[author]!.known), ...replicas[author]!.pending].reduce((text, op) => applyTextSpliceV1(text, op.splice).text, initial);
      const show = (author: number, update: { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 }) => {
        const view = hosts[author]!;
        view.host = update.host;
        if (update.show !== null) ({ text: view.local, caret: view.caret } = update.show);
      };
      const publish = (author: number) => {
        if (trace) log('publish', author, JSON.stringify({ remote: replicaText(author), applied: replicas[author]!.applied, unapplied: hosts[author]!.host.unapplied.map((e) => e.seq), base: hosts[author]!.host.base, local: hosts[author]!.local, lost: hosts[author]!.lost, typed: hosts[author]!.typed }));
        replicas[author]!.unpublished = false;
        show(author, receiveTextEditorSceneV1(hosts[author]!.host, replicaText(author), replicas[author]!.applied, hosts[author]!.local, { anchor: hosts[author]!.caret, caret: hosts[author]!.caret }));
      };
      const refuse = (author: number, op: Op) => {
        if (trace) log('refuse', author, op.seq, JSON.stringify({ local: hosts[author]!.local, unapplied: hosts[author]!.host.unapplied.map((e) => e.seq) }));
        const view = hosts[author]!, before = count(view.local, letters[author]!);
        show(author, refuseTextEditorSpliceV1(view.host, op.seq, view.local, { anchor: view.caret, caret: view.caret }));
        view.lost += Math.max(0, before - count(view.local, letters[author]!) - count(op.splice.insert, letters[author]!));
      };
      const send = (author: number) => {
        if (trace) log('send', author, JSON.stringify({ base: hosts[author]!.host.base, local: hosts[author]!.local }));
        const view = hosts[author]!, sent = sendTextEditorSpliceV1(view.host, view.local);
        if (sent === null) return;
        view.host = sent.host;
        replicas[author]!.inbox.push({ author, seq: sent.seq, splice: sent.splice });
      };
      const process = (author: number, publishNow: boolean) => {
        const replica = replicas[author]!, op = replica.inbox.shift();
        if (op === undefined) return;
        replica.pending.push(op);
        replica.outbox.push(op);
        replica.applied = op.seq;
        replica.settles.push(op);
        if (publishNow) publish(author);
        else replica.unpublished = true;
      };
      const settle = (author: number) => {
        if (trace) log('settle', author, replicas[author]!.settles[0]?.seq);
        const op = replicas[author]!.settles.shift();
        if (op !== undefined) hosts[author]!.host = settleTextEditorSpliceV1(hosts[author]!.host, op.seq);
      };
      const reopen = (author: number) => {
        if (trace) log('reopen', author, JSON.stringify({ inbox: replicas[author]!.inbox.map((op) => op.seq), settles: replicas[author]!.settles.map((op) => op.seq), unpublished: replicas[author]!.unpublished }));
        const view = hosts[author]!, replica = replicas[author]!, refused = replica.inbox;
        reincarnations += 1;
        if (replica.unpublished) lostAcknowledgements += 1;
        view.lost += refused.reduce((lost, op) => lost + count(op.splice.insert, letters[author]!), 0);
        const owed = replica.settles.map((op) => ({ op, applied: random() < 0.5 }));
        replica.inbox = [];
        replica.pending = [];
        replica.settles = [];
        replica.applied = 0;
        const answer = () => {
          for (const { op, applied } of owed) {
            if (applied) hosts[author]!.host = settleTextEditorSpliceV1(hosts[author]!.host, op.seq);
            else refuse(author, op);
          }
          for (const op of refused) refuse(author, op);
        };
        const answersFirst = random() < 0.5;
        if (answersFirst) answer();
        publish(author);
        if (!answersFirst) answer();
      };
      const step = (action: number) => {
        const author = action % 2, view = hosts[author]!, replica = replicas[author]!;
        if (replica.unpublished && random() < 0.5) return publish(author);
        if (replica.settles.length > 0 && random() < 0.4) return settle(author);
        switch (Math.floor(action / 2)) {
          case 0: {
            const chars = Array.from(view.local);
            view.local = [...chars.slice(0, view.caret), letters[author]!, ...chars.slice(view.caret)].join("");
            view.caret += 1;
            view.typed += 1;
            return;
          }
          case 1:
            return send(author);
          case 2:
            return process(author, random() < 0.7);
          case 3: {
            const op = replica.outbox.shift();
            if (op !== undefined) hub.push(op);
            return;
          }
          case 4: {
            if (replica.known === hub.length) return;
            const op = hub[replica.known]!;
            replica.known += 1;
            if (op.author === author) replica.pending = replica.pending.filter((entry) => entry.seq !== op.seq);
            return publish(author);
          }
          default:
            return random() < 0.3 ? reopen(author) : undefined;
        }
      };
      for (let turn = 0; turn < 80; turn += 1) step(Math.floor(random() * 12));
      const lastReopen = session % 3;
      if (lastReopen < 2) {
        send(lastReopen);
        process(lastReopen, false);
        reopen(lastReopen);
      }
      for (let drain = 0; drain < 600 && (replicas.some((replica) => replica.unpublished || replica.settles.length > 0 || replica.inbox.length > 0 || replica.outbox.length > 0 || replica.known < hub.length) || hosts.some((view) => textSpliceFromEditV1(view.host.base, view.local) !== null)); drain += 1) step(2 + (drain % 8));
      const final = hub.reduce((text, op) => applyTextSpliceV1(text, op.splice).text, initial);
      for (const author of [0, 1]) {
        expect(hosts[author]!.local, `session ${session}: host ${author} converged`).toBe(final);
        expect(count(final, letters[author]!), `session ${session}: no ${letters[author]} duplicated, only never-applied runs lost`).toBe(hosts[author]!.typed - hosts[author]!.lost);
      }
    }
