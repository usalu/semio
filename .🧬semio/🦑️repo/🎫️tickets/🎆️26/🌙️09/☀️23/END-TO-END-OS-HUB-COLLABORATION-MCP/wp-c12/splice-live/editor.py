# 📮️ The React TextEditor's outbox reads the editor's state when a delivery goes out (every typing mode); a splice integration law
# over the actual React editor (`📮️delivery` fixture `spliceCases`, engine-contract harness).
DELIVERY_FIXTURE = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧫️fixtures/📮️delivery/🔣️.json"
DELIVERY_SCHEMA = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/✏️TextEditor/🧬️schema/📮️delivery/🔣️.json"
ENGINE_LAWS = ROOT + "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧪️tests/🔬️engine-contract/🟦️.ts"

edit(EDITOR, 'import { createCoalescingActionDispatcher, shellLabel } from "../🛠️ShellHelpers/🟦️.tsx";', 'import { shellLabel } from "../🛠️ShellHelpers/🟦️.tsx";', "editor import")
edit(EDITOR, """/** 📮️ What the editor owes the guest: its latest text and selection (byte offsets of the session). */
type TextEditorOutboxV1 = { readonly text: string; readonly start: number; readonly end: number };
""", """/** 📮️ What the editor owes the guest: its latest text and selection (byte offsets of the session). */
type TextEditorOutboxV1 = { readonly text: string; readonly start: number; readonly end: number };

/** 📮️ The editor's outbox: at most ONE round trip in flight, and each delivery carries the editor's state AS IT IS WHEN THE
 * DELIVERY GOES OUT (`read`), never a snapshot taken when it was owed — a scene that folded a collaborator's run into the editor
 * in between is part of what goes out (ticket 26/09/23 C12, live wave p33: a splice computed from a snapshot older than that fold
 * deleted the collaborator's run and re-inserted everything around it, so both editors duplicated and lost runs). `notify` after
 * every change; a state equal to the last one delivered is not sent again; a refused round trip frees the lane like a settled one.
 * @see ../🧫️fixtures/📮️delivery/🔣️.json */
export function createTextEditorOutboxV1<T>(read: () => T | null, deliver: (value: T) => unknown, isEqual: (a: T, b: T) => boolean): () => void {
  let inFlight = false;
  let owed = false;
  let lastSent: T | undefined;
  const flush = () => {
    if (inFlight || !owed) return;
    owed = false;
    const next = read();
    if (next === null || (lastSent !== undefined && isEqual(lastSent, next))) return;
    lastSent = next;
    inFlight = true;
    const release = () => {
      inFlight = false;
      flush();
    };
    void Promise.resolve(deliver(next)).then(release, release);
  };
  return () => {
    owed = true;
    flush();
  };
}
""", "editor outbox")
edit(EDITOR, """  /** 📮️ ONE round trip in flight per editor, latest state wins: a typed run is delivered as the newest full text (plus the
   * selection that goes with it) whenever the previous delivery settled, never one `textEdit` + one `textSelect` per key —""",
     """  /** 📮️ ONE round trip in flight per editor, latest state wins: a typed run is delivered as the editor's text and selection AT
   * DELIVERY (`createTextEditorOutboxV1`) whenever the previous delivery settled, never one `textEdit` + one `textSelect` per key —""", "editor outbox doc")
edit(EDITOR, """  const deliver = useMemo(
    () =>
      createCoalescingActionDispatcher<TextEditorOutboxV1>(
        async (next) => {""", """  const deliver = useMemo(
    () =>
      createTextEditorOutboxV1<TextEditorOutboxV1>(
        () => {
          const session = sessionRef.current;
          return session === null ? null : { text: session.text(), start: session.anchor(), end: session.caret() };
        },
        async (next) => {""", "editor outbox reads the session")
edit(EDITOR, """  const sendEdit = useCallback(
    (text: string) => {
      const session = sessionRef.current;
      deliver({ text, start: session?.anchor() ?? text.length, end: session?.caret() ?? text.length });
    },
    [deliver],
  );

""", "", "editor sendEdit removed")
edit(EDITOR, """  const emitSelection = useCallback(() => {
    const session = sessionRef.current;
    if (!session) return;
    deliver({ text: session.text(), start: session.anchor(), end: session.caret() });
    publishCaretPresence();
  }, [deliver, publishCaretPresence]);""", """  const emitSelection = useCallback(() => {
    deliver();
    publishCaretPresence();
  }, [deliver, publishCaretPresence]);""", "editor emitSelection notifies the outbox")

editor_text = read(EDITOR)
if editor_text is not None:
    import re as _re
    pairs = _re.findall(r"\n([ ]+)sendEdit\(session\.text\(\)\);\n\1emitSelection\(\);", editor_text)
    if pairs:
        files[EDITOR] = _re.sub(r"\n([ ]+)sendEdit\(session\.text\(\)\);\n(\1emitSelection\(\);)", r"\n\2", editor_text)
        plan.append(f"edit    editor keystrokes notify once ×{len(pairs)}")
    elif "sendEdit(" in editor_text.replace("const sendEdit", ""):
        problems.append(("editor sendEdit call without emitSelection", editor_text.count("sendEdit(")))
    else:
        plan.append("present editor keystrokes notify once")
replace_count = 0
for old, new in (("  }, [emitSelection, sendEdit]);", "  }, [emitSelection]);"), ("    [emitSelection, sendEdit],", "    [emitSelection],")):
    text = read(EDITOR)
    if text is not None and old in text:
        files[EDITOR] = text.replace(old, new)
        replace_count += text.count(old)
plan.append(f"edit    editor dependency lists ×{replace_count}")

edit(EDITOR, """function refusalReason(outcome: unknown): string | null {""", """/** ✅️ Whether a dispatched action's typed outcome says the guest APPLIED it — its typed operation completed. */
function outcomeApplied(outcome: unknown): boolean {
  return typeof outcome === "object" && outcome !== null && (outcome as { readonly kind?: unknown }).kind === "applied";
}

function refusalReason(outcome: unknown): string | null {""", "editor outcome applied")
edit(EDITOR, """              const reason = refusalReason(await onAction({ controllerId, action: textEditorActions.splice, args: { surfaceId, ...sent.splice, seq: sent.seq, anchor: next.start, caret: next.end } }));
              if (reason === null) return;""", """              const outcome = await onAction({ controllerId, action: textEditorActions.splice, args: { surfaceId, ...sent.splice, seq: sent.seq, anchor: next.start, caret: next.end } });
              const reason = refusalReason(outcome);
              if (reason === null) {
                if (outcomeApplied(outcome)) spliceHostRef.current = settleTextEditorSpliceV1(spliceHostRef.current ?? sent.host, sent.seq);
                return;
              }""", "editor settles an applied splice")
edit(EDITOR, "sendTextEditorSpliceV1, TEXT_EDITOR_SCENE_LANES,", "sendTextEditorSpliceV1, settleTextEditorSpliceV1, TEXT_EDITOR_SCENE_LANES,", "editor import settle")

SPLICE_CASES = [
    {
        "id": "a-collaborator-run-folded-in-while-a-splice-is-in-flight",
        "note": "the editor types X (splice 1 in flight), then Y; the window publishes its text with splice 1 AND a collaborator's C folded in; when splice 1 settles the next splice goes out against what the editor shows NOW: it inserts Y only, never deleting C",
        "initial": "ab",
        "typedFirst": ["X"],
        "typedWhileInFlight": ["Y"],
        "published": {"buffer": "CabX", "applied": 1},
        "expected": ["textSplice:1:2::X:ab:", "textSplice:2:4::Y:CabX:"],
        "finalText": "CabXY",
    }
]
fixture_text = read(DELIVERY_FIXTURE)
if fixture_text is not None:
    fixture = json.loads(fixture_text)
    if json.dumps(fixture, indent=2, ensure_ascii=False) + "\n" != fixture_text:
        problems.append(("delivery fixture", "format"))
    elif fixture.get("spliceCases") == SPLICE_CASES:
        plan.append("present delivery fixture splice cases")
    else:
        fixture["spliceCases"] = SPLICE_CASES
        files[DELIVERY_FIXTURE] = json.dumps(fixture, indent=2, ensure_ascii=False) + "\n"
        plan.append("fixture delivery splice cases")
schema_text = read(DELIVERY_SCHEMA)
if schema_text is not None:
    schema = json.loads(schema_text)
    splice_schema = {
        "type": "array",
        "minItems": 1,
        "description": "Splice-typing delivery over the actual React editor (`settingsJson.typing.mode = splice`): keys typed, one splice in flight, a published scene that folds a collaborator's run in, then the delivery that follows — `expected` lists every `textSplice` as `textSplice:<seq>:<start>:<deleted>:<insert>:<before>:<after>`.",
        "items": {
            "type": "object",
            "additionalProperties": False,
            "required": ["id", "note", "initial", "typedFirst", "typedWhileInFlight", "published", "expected", "finalText"],
            "properties": {
                "id": {"type": "string", "minLength": 1},
                "note": {"type": "string", "minLength": 1},
                "initial": {"type": "string"},
                "typedFirst": {"type": "array", "minItems": 1, "items": {"type": "string", "minLength": 1}},
                "typedWhileInFlight": {"type": "array", "minItems": 1, "items": {"type": "string", "minLength": 1}},
                "published": {"type": "object", "additionalProperties": False, "required": ["buffer", "applied"], "properties": {"buffer": {"type": "string"}, "applied": {"type": "integer", "minimum": 0}}},
                "expected": {"type": "array", "minItems": 1, "items": {"type": "string", "pattern": "^textSplice:"}},
                "finalText": {"type": "string"},
            },
        },
    }
    if schema["properties"].get("spliceCases") == splice_schema:
        plan.append("present delivery schema splice cases")
    else:
        schema["properties"]["spliceCases"] = splice_schema
        files[DELIVERY_SCHEMA] = json.dumps(schema, ensure_ascii=False) + "\n"
        plan.append("schema delivery splice cases")

ENGINE_ANCHOR = """        expect(text).toBe(law.outcome === "accepted" ? law.initial + [...law.typed, ...typedAfterNewOwner].join("") : law.initial);
      } finally {
        view.unmount();
        factory.mockRestore();
        bounds.mockRestore();
      }
    });
"""
ENGINE_LAW = """
  for (const law of editorDeliveryFixture.spliceCases)
    it(`delivers splice typing against what the actual React editor shows at delivery: ${law.id}`, async () => {
      const { decodePackValue } = await import("@semio-tech/framework-os");
      let text = law.initial;
      let caret = text.length;
      const session = {
        attachCanvas: vi.fn(async () => {}),
        setCaretVisible: vi.fn(),
        setSize: () => {},
        renderFrame: () => {},
        syncFromSceneJson: () => {},
        setText: (value: string) => {
          text = value;
        },
        setSelectionRange: (_anchor: number, next: number) => {
          caret = next;
        },
        syncFromScenePack: (pack: Uint8Array) => {
          const scene = decodePackValue(pack) as { buffer?: string };
          if (scene.buffer !== undefined) text = scene.buffer;
        },
        text: () => text,
        caret: () => caret,
        anchor: () => caret,
        setCanvasThemeJson: () => {},
        free: () => {},
        insertText: (value: string) => {
          text = text.slice(0, caret) + value + text.slice(caret);
          caret += value.length;
        },
      };
      const factory = vi.spyOn(flowSessionLoader, "createEditorSession").mockResolvedValue(session as unknown as flowSessionLoader.EditorWasmSession);
      const bounds = vi.spyOn(HTMLElement.prototype, "getBoundingClientRect").mockReturnValue(new DOMRect(0, 0, 640, 480));
      const completions: Array<(outcome: unknown) => void> = [];
      const dispatched: string[] = [];
      const onAction = vi.fn((action: ActionDescriptor) => {
        if (action.action !== "textSplice") return Promise.resolve(undefined);
        const args = action.args as { readonly seq: number; readonly start: number; readonly deleted: string; readonly insert: string; readonly before: string; readonly after: string };
        dispatched.push(`textSplice:${args.seq}:${args.start}:${args.deleted}:${args.insert}:${args.before}:${args.after}`);
        return new Promise((resolve) => completions.push(resolve));
      });
      const node = (buffer: string, applied: number) => ({
        type: "componentScene",
        surfaceId: "writer.splice",
        controllerId: "writer",
        componentKind: "text-editor",
        textEditor: { buffer, selectionJson: JSON.stringify({ start: buffer.length, end: buffer.length, splice: applied }), settingsJson: JSON.stringify({ typing: { mode: "splice" } }) },
      });
      const view = render(createElement(TextEditorHost, { node: node(law.initial, 0), onAction } as never));
      try {
        await waitFor(() => expect(session.attachCanvas).toHaveBeenCalledOnce());
        await reactAct(async () => {
          await Promise.resolve();
        });
        const area = view.container.querySelector("textarea")!;
        for (const key of law.typedFirst) fireEvent.keyDown(area, { key });
        for (const key of law.typedWhileInFlight) fireEvent.keyDown(area, { key });
        await reactAct(async () => {
          view.rerender(createElement(TextEditorHost, { node: node(law.published.buffer, law.published.applied), onAction } as never));
          await Promise.resolve();
        });
        for (let turn = 0; turn < 16 && (completions.length > 0 || dispatched.length < law.expected.length); turn += 1)
          await reactAct(async () => {
            completions.shift()?.({ kind: "applied", inputSeq: 0 });
            await Promise.resolve();
          });
        await waitFor(() => expect(dispatched).toEqual(law.expected));
        expect(text).toBe(law.finalText);
      } finally {
        view.unmount();
        factory.mockRestore();
        bounds.mockRestore();
      }
    });
"""
edit(ENGINE_LAWS, ENGINE_ANCHOR, ENGINE_ANCHOR + ENGINE_LAW, "engine-contract splice delivery law")
