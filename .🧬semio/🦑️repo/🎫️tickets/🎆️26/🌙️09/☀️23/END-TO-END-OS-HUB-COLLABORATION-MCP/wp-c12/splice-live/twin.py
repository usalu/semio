# ✂️ The splice-typing host twin (TS reference) drops the unapplied splices of a reincarnated window; a late refusal of a dropped
# run is a no-op. Fixture row + seeded reincarnation simulation law.
edit(TWIN, """/** @emoji 🧾️ One splice-typing host's knowledge of one window (the Jupiter client model, no CRDT): `remote` — the text the
 * window last published, `base` — the text the window holds once it applied every splice this host sent (the text the next
 * splice is computed against), `seq` — the last splice sent, `unapplied` — the sent splices no scene acknowledged yet, oldest
 * first. */
export type TextEditorSpliceHostV1 = {
  readonly remote: string;
  readonly base: string;
  readonly seq: number;
  readonly unapplied: readonly { readonly seq: number; readonly splice: TextSpliceV1 }[];
};""", """/** @emoji 🧾️ One splice-typing host's knowledge of one window (the Jupiter client model, no CRDT): `remote` — the text the
 * window last published, `base` — the text the window holds once it applied every splice this host sent (the text the next
 * splice is computed against), `seq` — the last splice sent, `acknowledged` — the last splice a scene said the window applied (a
 * scene naming a lower one comes from a reincarnated window), `unapplied` — the sent splices no scene acknowledged yet, oldest
 * first. */
export type TextEditorSpliceHostV1 = {
  readonly remote: string;
  readonly base: string;
  readonly seq: number;
  readonly acknowledged: number;
  readonly unapplied: readonly { readonly seq: number; readonly splice: TextSpliceV1 }[];
};""", "twin host acknowledged")
edit(TWIN, "  return { remote: buffer, base: buffer, seq: applied, unapplied: [] };", "  return { remote: buffer, base: buffer, seq: applied, acknowledged: applied, unapplied: [] };", "twin host constructor")
edit(TWIN, "  return { host: { remote, base, seq: host.seq, unapplied }, show: view.text === local ? null : view };", "  return { host: { ...host, remote, base, unapplied }, show: view.text === local ? null : view };", "twin show keeps the host")
edit(TWIN, """ * so neither a lagging echo nor a collaborator's run ever reverts or scrambles what this host typed. */
export function receiveTextEditorSceneV1(host: TextEditorSpliceHostV1, remote: string, applied: number, local: string, selection: { readonly anchor: number; readonly caret: number }): { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 } {
  return textEditorSpliceShowV1({ ...host, seq: Math.max(host.seq, applied) }, remote, host.unapplied.filter((entry) => entry.seq > applied), local, selection, textSpliceFromEditV1(host.base, local));
}""", """ * so neither a lagging echo nor a collaborator's run ever reverts or scrambles what this host typed. A scene naming a splice BELOW
 * the one the window already acknowledged comes from a reincarnated window (the document reopened, silently on a hub-ordered
 * reorder too): this host's unapplied splices never reached it, so they are dropped instead of being folded onto its text as if
 * still in flight (which duplicated them once the reopened document carried them); the numbering continues. */
export function receiveTextEditorSceneV1(host: TextEditorSpliceHostV1, remote: string, applied: number, local: string, selection: { readonly anchor: number; readonly caret: number }): { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 } {
  const unapplied = applied < host.acknowledged ? [] : host.unapplied.filter((entry) => entry.seq > applied);
  return textEditorSpliceShowV1({ ...host, seq: Math.max(host.seq, applied), acknowledged: applied }, remote, unapplied, local, selection, textSpliceFromEditV1(host.base, local));
}""", "twin receive drops a reincarnated window's runs")
edit(TWIN, """ * saved; the caret collapses where the refused run was (located by the run's own context, which is still there). */
export function refuseTextEditorSpliceV1(host: TextEditorSpliceHostV1, seq: number, local: string, selection: { readonly anchor: number; readonly caret: number }): { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 } {
  const refused = host.unapplied.find((entry) => entry.seq === seq);
  const shown = textEditorSpliceShowV1(host, host.remote, host.unapplied.filter((entry) => entry.seq !== seq), local, selection, null);
  if (refused === undefined || shown.show === null) return shown;""", """ * saved; the caret collapses where the refused run was (located by the run's own context, which is still there). A refusal of a
 * run the host no longer holds (dropped when its window reincarnated) changes nothing. */
export function refuseTextEditorSpliceV1(host: TextEditorSpliceHostV1, seq: number, local: string, selection: { readonly anchor: number; readonly caret: number }): { readonly host: TextEditorSpliceHostV1; readonly show: TextEditorSpliceViewV1 } {
  const refused = host.unapplied.find((entry) => entry.seq === seq);
  if (refused === undefined) return { host, show: null };
  const shown = textEditorSpliceShowV1(host, host.remote, host.unapplied.filter((entry) => entry.seq !== seq), local, selection, null);
  if (shown.show === null) return shown;""", "twin refusal of a dropped run")

edit(TWIN, """/** @emoji 🚫️ The window refused splice `seq`""", """/** @emoji ✅️ The round trip of splice `seq` settled APPLIED — the window's typed operation completed, so the window holds it and
 * every splice this host sent before it, whether or not a scene said so yet: they leave the unapplied set, so no later scene (a
 * reincarnated window's included, which may carry them from the hub) folds them onto a text that already holds them. */
export function settleTextEditorSpliceV1(host: TextEditorSpliceHostV1, seq: number): TextEditorSpliceHostV1 {
  return { ...host, unapplied: host.unapplied.filter((entry) => entry.seq > seq) };
}

/** @emoji 🚫️ The window refused splice `seq`""", "twin settle")

REINCARNATION_ROW = {
    "id": "a-reincarnated-window-drops-the-runs-it-never-saw",
    "note": "the document reopened while run 2 was in flight: the new window (applied 0 < acknowledged 1) never saw it, so the host drops it instead of folding it onto the reopened text, a late refusal of it changes nothing, and the next run is numbered 3",
    "init": {"buffer": "ab", "applied": 0},
    "events": [
        {"local": "abX", "caret": 3},
        {"send": {"seq": 1, "splice": {"start": 2, "deleted": "", "insert": "X", "before": "ab", "after": ""}}},
        {"scene": "abX", "applied": 1, "show": None, "unapplied": []},
        {"local": "abXY", "caret": 4},
        {"send": {"seq": 2, "splice": {"start": 3, "deleted": "", "insert": "Y", "before": "abX", "after": ""}}},
        {"scene": "", "applied": 0, "show": {"text": "", "anchor": 0, "caret": 0}, "unapplied": []},
        {"scene": "abX", "applied": 0, "show": {"text": "abX", "anchor": 0, "caret": 0}, "unapplied": []},
        {"refuse": 2, "show": None, "unapplied": []},
        {"local": "abXZ", "caret": 4},
        {"send": {"seq": 3, "splice": {"start": 3, "deleted": "", "insert": "Z", "before": "abX", "after": ""}}},
        {"scene": "abXZ", "applied": 3, "show": None, "unapplied": []},
    ],
}
SETTLE_ROW = {
    "id": "a-settled-run-never-folds-twice",
    "note": "the round trip of run 1 settled applied before any scene acknowledged it; the window then reincarnated (applied 0, nothing to compare with) and the reopened document carries X from the hub: X shows once, never folded onto it again",
    "init": {"buffer": "ab", "applied": 0},
    "events": [
        {"local": "abX", "caret": 3},
        {"send": {"seq": 1, "splice": {"start": 2, "deleted": "", "insert": "X", "before": "ab", "after": ""}}},
        {"settle": 1, "unapplied": []},
        {"scene": "", "applied": 0, "show": {"text": "", "anchor": 0, "caret": 0}, "unapplied": []},
        {"scene": "abX", "applied": 0, "show": {"text": "abX", "anchor": 0, "caret": 0}, "unapplied": []},
    ],
}
fixture_text = read(TWIN_FIXTURE)
if fixture_text is not None:
    fixture = json.loads(fixture_text)
    if json.dumps(fixture, indent=2, ensure_ascii=False) + "\n" != fixture_text:
        problems.append(("twin fixture", "format"))
    else:
        for row in (REINCARNATION_ROW, SETTLE_ROW):
            if any(entry["id"] == row["id"] for entry in fixture["hostTyping"]):
                plan.append("present twin fixture row " + row["id"])
            else:
                fixture["hostTyping"].append(row)
                plan.append("fixture twin row " + row["id"])
        files[TWIN_FIXTURE] = json.dumps(fixture, indent=2, ensure_ascii=False) + "\n"

edit(TWIN_SCHEMA, "`scene` = the window published this text having applied the host's splices up to `applied`; `refuse` = the window refused splice `refuse`.",
     "`scene` = the window published this text having applied the host's splices up to `applied` (below an earlier `applied`: a reincarnated window, the document reopened); `refuse` = the window refused splice `refuse`; `settle` = the round trip of splice `settle` settled applied (the window's typed operation completed).", "twin schema reincarnation comment")
schema_text = read(TWIN_SCHEMA)
if schema_text is not None:
    schema = json.loads(schema_text)
    branches = schema["definitions"]["hostTyping"]["items"]["properties"]["events"]["items"]["oneOf"] if "hostTyping" in schema.get("definitions", {}) else schema["properties"]["hostTyping"]["items"]["properties"]["events"]["items"]["oneOf"]
    settle_branch = {"type": "object", "required": ["settle", "unapplied"], "additionalProperties": False, "properties": {"settle": {"type": "integer", "minimum": 1}, "unapplied": {"type": "array", "items": {"type": "integer", "minimum": 1}}}}
    if settle_branch in branches:
        plan.append("present twin schema settle event")
    else:
        branches.append(settle_branch)
        files[TWIN_SCHEMA] = json.dumps(schema, indent=2, ensure_ascii=False) + "\n"
        plan.append("schema twin settle event")

REINCARNATION_LAW_ANCHOR = """      for (const author of [0, 1]) {
        expect(hosts[author]!.local, `session ${session}: host ${author} converged`).toBe(final);
        expect(Array.from(final).filter((char) => char === letters[author]).length, `session ${session}: every ${letters[author]} survives`).toBe(hosts[author]!.typed.length);
      }
    }
  });
"""
REINCARNATION_LAW = """
  test("a window that reincarnates (its document reopened) while both hosts type: the hosts still converge on the hub order, no scalar is ever duplicated, and only runs no window applied are lost (seeded, 300 sessions)", () => {
    let seed = 0xc12_29a;
    const random = () => ((seed = (Math.imul(seed ^ (seed >>> 15), 0x2c1b3c6d) + 0x6d2b79f5) >>> 0) / 2 ** 32);
    const pick = <T,>(items: readonly T[]): T => items[Math.floor(random() * items.length)]!;
    type Op = { readonly author: number; readonly seq: number; readonly splice: TextSpliceV1 };
    const count = (text: string, letter: string) => Array.from(text).filter((char) => char === letter).length;
    let reincarnations = 0, lostAcknowledgements = 0;
    for (let session = 0; session < 300; session += 1) {
      const initial = Array.from({ length: Math.floor(random() * 16) }, () => pick(Array.from("ab c\\n😀"))).join("");
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
        replicas[author]!.unpublished = false;
        show(author, receiveTextEditorSceneV1(hosts[author]!.host, replicaText(author), replicas[author]!.applied, hosts[author]!.local, { anchor: hosts[author]!.caret, caret: hosts[author]!.caret }));
      };
      const refuse = (author: number, op: Op) => {
        const view = hosts[author]!;
        if (view.host.unapplied.some((entry) => entry.seq === op.seq)) view.lost += count(textSpliceFromEditV1(view.host.base, view.local)?.insert ?? "", letters[author]!);
        show(author, refuseTextEditorSpliceV1(view.host, op.seq, view.local, { anchor: view.caret, caret: view.caret }));
      };
      const send = (author: number) => {
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
        const op = replicas[author]!.settles.shift();
        if (op !== undefined) hosts[author]!.host = settleTextEditorSpliceV1(hosts[author]!.host, op.seq);
      };
      const reopen = (author: number) => {
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
    expect(reincarnations, "the seeded sessions reincarnate windows").toBeGreaterThan(300);
    expect(lostAcknowledgements, "some windows reincarnate before acknowledging an applied run").toBeGreaterThan(50);
  });
"""
edit(TWIN_LAWS, REINCARNATION_LAW_ANCHOR, REINCARNATION_LAW_ANCHOR + REINCARNATION_LAW, "twin reincarnation simulation law")
edit(TWIN_LAWS, """          host = sent!.host;
        } else {
          const received =""", """          host = sent!.host;
        } else if (event.settle !== undefined) {
          host = settleTextEditorSpliceV1(host, event.settle as number);
          expect(host.unapplied.map((entry) => entry.seq), `${row.id} #${index} unapplied`).toEqual(event.unapplied as number[]);
        } else {
          const received =""", "twin host law settle event")
edit(TWIN_LAWS, "scalarOfUtf8OffsetV1, sendTextEditorSpliceV1, TEXT_SPLICE_CONTEXT_SCALARS", "scalarOfUtf8OffsetV1, sendTextEditorSpliceV1, settleTextEditorSpliceV1, TEXT_SPLICE_CONTEXT_SCALARS", "twin laws import settle")

edit(TWIN_LAWS, "utf8OffsetOfScalarV1, type TextEditorSpliceHostV1, type TextSpliceV1 } from \"../../✂️text-splice/🟦️.ts\";",
     "utf8OffsetOfScalarV1, type TextEditorSpliceHostV1, type TextEditorSpliceViewV1, type TextSpliceV1 } from \"../../✂️text-splice/🟦️.ts\";", "twin laws import view type")
edit(TWIN_LAWS, " * UTF-8 ↔ scalar offsets), jsdiff (third party) derives the same changed run for every edit, and no concurrent insert-only\n * workload loses a typed scalar. */",
     " * UTF-8 ↔ scalar offsets), jsdiff (third party) derives the same changed run for every edit, no concurrent insert-only\n * workload loses a typed scalar, and a reincarnated window never makes a host duplicate one. */", "twin laws header")
