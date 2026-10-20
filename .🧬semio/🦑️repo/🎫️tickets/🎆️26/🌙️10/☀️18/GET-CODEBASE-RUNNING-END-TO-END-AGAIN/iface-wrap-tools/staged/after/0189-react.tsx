/** ✍️ The editable retained views: input with its commit draft and picker release, select, toggle, slider, number stepper, ring and icon select.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import {
  useEffect,
  useRef,
  useState,
  type KeyboardEvent as ReactKeyboardEvent,
  type RefObject
} from "../../⚛️react-port/🟦️.ts";
import { INPUT_COMMIT_CAPACITY_V1, inputCommitLaneV1 } from "../🎯️commit-lane/🟦️.ts";
import { inputCommitReceiptMatchesPublicationV1, type InputOutcomeV1 } from "../../🏛️ShellHost/🎯️input-ledger/🟦️.ts";
import {
  Button,
  IconSelector,
  Input,
  Popover,
  PopoverContent,
  PopoverTrigger,
  Ring,
  Select,
  SelectContent,
  SelectItem,
  SelectTrigger,
  SelectValue,
  Slider,
  Stepper,
  Textarea,
  Toggle,
  ToggleGroup,
  TreeCheckbox,
  classifyIconSelectorMode,
  useLabel
} from "@semio-tech/ui-react";
import { type ContinuousGestureLane } from "@semio-tech/framework";
import { type Component, type UiNodeRecord, type UiValue } from "@semio-tech/framework";
import { uiAccessibilityValueV1 } from "@semio-tech/ui-contract/presentation/accessibility";
import { formatUiNumber, formatUiNumberFixed } from "@semio-tech/ui-contract/presentation/number-format";
import {
  uiNumberCrossedBound,
  uiNumberDisplayText,
  uiNumberFieldKey,
  uiNumberKeyValue,
  uiNumberTypedValue,
  uiTextInputKey,
  uiTextInputKeyOf
} from "@semio-tech/ui-contract/component";
import { interpLabel, type UiInterpreterContext } from "../🧷️interpreter-context/🟦️.ts";
import { dispatchTrigger, resolveControlIconNode, toUiValue, useContinuousTriggerLane } from "../⚡️action-dispatch/🟦️.tsx";
import { nodeDomId } from "../🪪️stable-dom-ids/🟦️.ts";

//#region 🔖️UiInterpreter
//#region ComponentRenderers
/** 🖊️ Queues commits for one authored target and preserves newer drafts across acknowledgements.
 * External changes retain a conflicting draft until discarded; a target change starts a fresh draft. */
function useCommitDraft(published: string, scope: string, guard: string, revision: string | null, send: (value: string, active: () => boolean, subscribeActive: (listener: () => void) => () => void) => ReturnType<UiInterpreterContext["onIntent"]>, publication: (value: string) => string, lifetime?: object | null): [string, (next: string) => void, (value: string) => void, () => void, boolean] {
  type Pending = { value: string; baseRevision: string | null; cancelled: boolean; settled: boolean; outcome: InputOutcomeV1 | undefined };
  const [, render] = useState(0);
  const create = () => ({ scope, lifetime, listeners: new Set<() => void>(), guard, revision, published, draft: published, committed: published, queue: [] as { value: string; publication: string }[], awaiting: null as Pending | null, busy: false, conflicted: false });
  const owner = useRef(create());
  const mounted = useRef(true);
  const sender = useRef(send);
  sender.current = send;
  useEffect(() => { mounted.current = true; return () => { mounted.current = false; owner.current.queue = []; for (const listener of owner.current.listeners) listener(); }; }, []);
  if (owner.current.scope !== scope || owner.current.lifetime !== lifetime) { const previous = owner.current; owner.current = create(); for (const listener of previous.listeners) listener(); }
  const state = owner.current;
  const acknowledge = (): boolean => {
    const pending = state.awaiting;
    if (!pending || !pending.settled) return false;
    const matches = pending.baseRevision === null ? pending.value === state.published : pending.outcome !== undefined && inputCommitReceiptMatchesPublicationV1(pending.outcome, state.revision) && pending.value === state.published;
    if (matches) { state.awaiting = null; return true; }
    if (pending.baseRevision !== null && (!pending.outcome || pending.outcome.kind !== "applied" || !pending.outcome.commit || state.revision !== pending.baseRevision)) state.conflicted = true;
    return false;
  };
  if (state.published !== published || state.guard !== guard || state.revision !== revision) {
    const previous = state.published;
    state.published = published;
    state.guard = guard;
    state.revision = revision;
    const accepted = acknowledge();
    if (state.draft === previous && state.queue.length === 0) { state.draft = published; state.committed = published; }
    else if (!accepted && !state.awaiting) state.conflicted = state.draft !== published;
  }
  const update = () => { if (mounted.current && owner.current === state) render((value) => value + 1); };
  const drain = async () => {
    if (state.busy || state.awaiting !== null || state.queue.length === 0 || state.conflicted) return;
    state.busy = true;
    try {
      while (mounted.current && owner.current === state && state.queue.length > 0 && !state.conflicted && state.awaiting === null) {
        const next = state.queue.shift()!;
        const pending: Pending = { value: next.publication, baseRevision: state.revision, cancelled: false, settled: false, outcome: undefined };
        state.awaiting = pending;
        const outcome = await sender.current(next.value, () => mounted.current && owner.current === state && !pending.cancelled, (listener) => { state.listeners.add(listener); return () => { state.listeners.delete(listener); }; });
        if (!mounted.current || owner.current !== state) return;
        if (pending.cancelled) { state.awaiting = null; continue; }
        if (outcome?.kind === "refused" && outcome.draftDisposition === "discard") {
          state.awaiting = null;
          state.queue = [];
          state.committed = state.published;
          if (!pending.cancelled && state.draft === next.value) state.draft = state.published;
          state.conflicted = state.draft !== state.published;
          break;
        }
        if (outcome && outcome.kind !== "applied") throw new Error(outcome.kind);
        pending.outcome = outcome || undefined;
        pending.settled = true;
        acknowledge();
      }
    } catch {
      const cancelled = state.awaiting?.cancelled === true;
      state.awaiting = null;
      if (!cancelled) { state.queue = []; state.committed = state.published; state.conflicted = true; }
    } finally { state.busy = false; update(); }
  };
  useEffect(() => { void drain(); });
  return [state.draft, (next) => { state.draft = next; update(); }, (value) => {
    if (state.conflicted || value === state.committed) return;
    if (state.queue.length + Number(state.awaiting !== null) >= INPUT_COMMIT_CAPACITY_V1) { state.queue = []; state.committed = state.published; state.conflicted = true; update(); return; }
    state.committed = value;
    state.queue.push({ value, publication: publication(value) });
    void drain();
  }, () => { state.queue = []; if (state.awaiting?.settled && state.conflicted) state.awaiting = null; else if (state.awaiting) state.awaiting.cancelled = true; state.draft = state.published; state.committed = state.published; state.conflicted = false; for (const listener of state.listeners) listener(); update(); }, state.conflicted];
}

/** 🪪️ Authored semantic identity separates a control target from changing command guards. */
function inputDraftScope(record: UiNodeRecord): string {
  const input = record.component as Extract<Component, { type: "input" }>;
  return input.draftTarget ?? JSON.stringify(record.bindings);
}

/** 🎨️ The release of a colour field's press: the native `change` a colour picker fires when it commits (React's `onChange`
 * is the per-move `input`, so it never sees it). A `display: contents` wrapper catches the bubbling event; the release is
 * deferred past React's own handler of the same event (which offers the value when the picker reports only `change`) and
 * names no value — the field is controlled, so its DOM value may already be the published one again; the lane releases
 * the press on the value last offered (design §13.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING). */
function usePickerRelease(enabled: boolean, lane: ContinuousGestureLane<UiValue>): RefObject<HTMLSpanElement | null> {
  const ref = useRef<HTMLSpanElement | null>(null);
  useEffect(() => {
    const element = ref.current;
    if (!enabled || element === null) return undefined;
    const release = () => queueMicrotask(() => lane.commit());
    element.addEventListener("change", release);
    return () => element.removeEventListener("change", release);
  }, [enabled, lane]);
  return ref;
}

export function InputView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "input" }>;
  const commitOnBlur = component.commit === "blur";
  const draftScope = inputDraftScope(record);
  const draftConflictLabel = useLabel("ui.host.inputDraftConflict");
  const draftDiscardLabel = useLabel("ui.host.inputDraftDiscard");
  const factor = component.kind === "number" ? (component.displayFactor ?? null) : null;
  const storedNumber = Number(component.value);
  const shownText = (stored: number): string => (factor == null ? (component.precision == null ? formatUiNumber(stored) : formatUiNumberFixed(stored, component.precision)) : uiNumberDisplayText(stored, factor, component.precision));
  const published = factor != null && component.value.trim() !== "" && Number.isFinite(storedNumber) ? shownText(storedNumber) : component.value;
  const [refusal, setRefusal] = useState<{ readonly message: string | null } | null>(null);
  const lane = useContinuousTriggerLane(context, record);
  /** 🚧️ The stored number a typed text means, or the refusal it earns: unreadable text, or a value crossing a hard bound (the
   * field's `limits`, else its inclusive `min`/`max`) — a refused value is never dispatched and the draft is kept. */
  const typedNumber = (raw: string): number | { readonly refused: string | null } => {
    const typed = Number(raw.trim());
    if (raw.trim() === "" || !Number.isFinite(typed)) return { refused: null };
    const stored = uiNumberTypedValue(typed, factor, component.precision, [storedNumber, ...(component.snaps ?? [])]);
    const crossed = uiNumberCrossedBound(stored, component.min, component.max, component.limits);
    return crossed ? { refused: crossed.refusal ?? null } : stored;
  };
  const draftPublication = (raw: string): string => {
    const number = component.kind === "number" ? typedNumber(raw) : null;
    return typeof number === "number" ? factor == null ? String(number) : shownText(number) : raw;
  };
  const [draft, setDraft, commitDraft, discardDraft, draftConflict] = useCommitDraft(published, JSON.stringify([context.localDocumentOwner?.id, draftScope]), JSON.stringify(record.bindings), component.publicationRevision ?? null, (raw, active, subscribeActive) => {
    const read = () => {
      const current = context.store.getNodeSnapshot(record.id);
      if (!current || current.disabled || current.component.type !== "input" || inputDraftScope(current) !== draftScope) return null;
      const input = current.component;
      return { target: JSON.stringify([draftScope, input.kind, input.displayFactor, input.precision]), value: factor != null && input.value.trim() !== "" && Number.isFinite(Number(input.value)) ? shownText(Number(input.value)) : input.value, revision: input.publicationRevision ?? "" };
    };
    const send = () => {
      const current = context.store.getNodeSnapshot(record.id);
      if (!active() || !read() || !current) throw new Error("Input command target changed");
      const number = component.kind === "number" ? typedNumber(raw) : null;
      return dispatchTrigger(context, current, "commit", toUiValue(typeof number === "number" ? number : raw));
    };
    return context.localDocumentOwner && component.publicationRevision != null
      ? inputCommitLaneV1(context.localDocumentOwner).submit({ read, send, active, expected: draftPublication(raw), subscribe: (listener) => { const publication = context.store.subscribeRevision(listener); const lifecycle = subscribeActive(listener); return () => { publication(); lifecycle(); }; } })
      : send();
  }, draftPublication, context.localDocumentOwner);
  const continuous = (component.kind === "number" || component.kind === "color") && !commitOnBlur;
  const picker = continuous && component.kind === "color";
  const pickerRef = usePickerRelease(picker, lane);
  const commitValue = (raw: string) => {
    const number = component.kind === "number" ? typedNumber(raw) : null;
    if (number !== null && typeof number !== "number") {
      setRefusal({ message: number.refused });
      return;
    }
    setRefusal(null);
    if (commitOnBlur) { commitDraft(raw); return; }
    const value: UiValue = typeof number === "number" ? toUiValue(number) : toUiValue(raw);
    if (continuous) {
      lane.offer(value);
      return;
    }
    dispatchTrigger(context, record, commitOnBlur ? "commit" : "change", value);
  };
  /** ⌨️ A draft's keys follow the shared text-control law (design §22.7, `uiTextInputKey`, corpus `🧫️text-controls`): Escape
   * drops the draft; a single-line field commits on Enter without waiting for focus to leave — the gesture a keyboard-only
   * user has; a multi-line field keeps Enter for its line break and commits on Ctrl/⌘+Enter. A key pressed while an input
   * method composes belongs to the composition. */
  const draftKey = (event: ReactKeyboardEvent<HTMLInputElement | HTMLTextAreaElement>) => {
    const pressed = event.nativeEvent.isComposing ? null : uiTextInputKeyOf(event);
    const action = pressed === null ? null : uiTextInputKey(component.kind ?? "text", pressed.key, pressed.modifiers);
    if (action === "revert") { event.preventDefault(); event.stopPropagation(); discardDraft(); setRefusal(null); return; }
    if (action !== "commit") return;
    event.preventDefault();
    commitValue((event.target as HTMLInputElement | HTMLTextAreaElement).value);
    (event.target as HTMLInputElement | HTMLTextAreaElement).blur();
  };
  /** 📌️ A number field's keys follow the shared keyboard law (design §18, `uiNumberFieldKey`): arrows one step (Shift: ten)
   * then the detent within tolerance, page keys the adjacent detent (`snaps`) else ten steps, Home/End a bound the field has —
   * staged in the draft of a blur-committed field, offered on the lane of a continuous one. */
  const lawKey = (event: ReactKeyboardEvent<HTMLInputElement | HTMLTextAreaElement>): boolean => {
    const typed = Number((event.target as HTMLInputElement).value);
    const key = component.kind === "number" ? uiNumberFieldKey(event.key, event.shiftKey, component.min, component.max) : null;
    if (key === null || !Number.isFinite(typed)) return false;
    event.preventDefault();
    const current = uiNumberTypedValue(typed, factor, component.precision, [storedNumber, ...(component.snaps ?? [])]);
    const next = uiNumberKeyValue(current, component.min ?? null, component.max ?? null, component.step ?? 0, component.precision ?? null, factor, component.snaps ?? [], key.key, key.large);
    const text = shownText(next);
    if (commitOnBlur) setDraft(text);
    else commitValue(text);
    return true;
  };
  const draftConflictId = `${nodeDomId(context.store, record, context.domScope)}-draft-conflict`;
  const recovery = draftConflict ? <>
    <span id={draftConflictId} role="alert" className="sr-only">{draftConflictLabel}</span>
    <Popover>
      <PopoverTrigger asChild><Button id={`${draftConflictId}-explain`} icon={resolveControlIconNode("triangle-alert")} aria-label={draftConflictLabel} title={draftConflictLabel} className="shrink-0 text-destructive" /></PopoverTrigger>
      <PopoverContent aria-label={draftConflictLabel} className="max-w-sm whitespace-normal">
        <p className="text-sm">{draftConflictLabel}</p>
        <Button id={`${draftConflictId}-discard`} icon={resolveControlIconNode("undo-2")} text={draftDiscardLabel} onClick={() => { discardDraft(); setRefusal(null); }} />
      </PopoverContent>
    </Popover>
  </> : null;
  if (component.kind === "longText") {
    return (<>
      <Textarea
        id={nodeDomId(context.store, record, context.domScope)}
        data-ui-node-id={record.id} data-ui-node-key={record.key}
        aria-label={record.accessibility.label ?? undefined}
        aria-invalid={draftConflict || undefined}
        aria-describedby={draftConflict ? draftConflictId : undefined}
        disabled={record.disabled}
        className="min-h-[4.5rem] w-full min-w-0"
        value={commitOnBlur ? draft : component.value}
        placeholder={component.placeholder ?? undefined}
        onChange={commitOnBlur ? (event) => setDraft(event.target.value) : (event) => commitValue(event.target.value)}
        onKeyDown={commitOnBlur ? draftKey : undefined}
        onBlur={commitOnBlur ? (event) => commitValue(event.target.value) : undefined}
      />
      {recovery}
    </>);
  }
  const inputType = component.kind === "number" ? "number" : component.kind === "date" ? "date" : component.kind === "color" ? "color" : component.kind === "file" ? "file" : "text";
  const shown = (value: number | null | undefined): number | undefined => (value == null ? undefined : factor == null ? value : value * factor);
  const refusalId = `${nodeDomId(context.store, record, context.domScope)}-refusal`;
  const input = (
    <Input
      id={nodeDomId(context.store, record, context.domScope)}
      data-ui-node-id={record.id} data-ui-node-key={record.key}
      aria-label={record.accessibility.label ?? undefined}
      aria-invalid={refusal || draftConflict ? true : undefined}
      aria-describedby={[refusal?.message ? refusalId : null, draftConflict ? draftConflictId : null].filter(Boolean).join(" ") || undefined}
      disabled={record.disabled}
      type={inputType}
      className="h-[var(--tree-inline-control-height,var(--size-medium))] w-full min-w-0"
      value={component.kind === "file" ? undefined : commitOnBlur ? draft : published}
      placeholder={component.placeholder ?? undefined}
      min={shown(component.min)}
      max={shown(component.max)}
      step={shown(component.step) ?? (component.kind === "number" && component.precision != null ? 10 ** -component.precision : undefined)}
      accept={component.kind === "file" ? (component.accept ?? undefined) : undefined}
      onChange={commitOnBlur && component.kind !== "file" ? (event) => setDraft(event.target.value) : (event) => commitValue(component.kind === "file" ? (event.target.files?.[0]?.name ?? "") : event.target.value)}
      onKeyDown={(event) => { if (!lawKey(event) && commitOnBlur) draftKey(event); }}
      onBlur={commitOnBlur ? (event) => commitValue(component.kind === "file" ? (event.target.files?.[0]?.name ?? "") : event.target.value) : continuous ? () => lane.commit() : undefined}
    />
  );
  const field = (
    <>
      {input}
      {recovery}
      {refusal?.message ? (
        <span id={refusalId} role="alert" data-slot="input-refusal" className="text-destructive block w-full text-xs leading-tight">
          {refusal.message}
        </span>
      ) : null}
    </>
  );
  return picker ? <span ref={pickerRef} className="contents">{field}</span> : field;
}

/** 🔽️ A select as its `appearance` says: a menu that opens its options, or — `segmented` — every option at once as a radio
 * group (`ToggleGroup` under radio semantics): one checked option, arrow keys move the choice, the chosen one never presses
 * off. Both dispatch the picked value on `Trigger::Change`. */
export function SelectView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "select" }>;
  if (component.appearance === "segmented") {
    return (
      <ToggleGroup
        id={nodeDomId(context.store, record, context.domScope)}
        kind="single"
        semantics="radio"
        data-ui-node-id={record.id}
        data-ui-node-key={record.key}
        aria-label={record.accessibility.label ?? undefined}
        disabled={record.disabled}
        value={component.value}
        onValueChange={(value) => dispatchTrigger(context, record, "change", toUiValue(value))}
        items={component.items.map((item) => ({ value: item.value, icon: "circle-dot" as const, text: item.label }))}
      />
    );
  }
  return (
    <Select id={`${nodeDomId(context.store, record, context.domScope)}-select`} disabled={record.disabled} value={component.value || undefined} onValueChange={(value) => dispatchTrigger(context, record, "change", toUiValue(value))}>
      <SelectTrigger id={nodeDomId(context.store, record, context.domScope)} aria-label={record.accessibility.label ?? undefined} data-ui-node-id={record.id} data-ui-node-key={record.key} className="h-[var(--tree-inline-control-height,var(--size-medium))] w-full min-w-0" size="sm">
        <SelectValue placeholder={component.placeholder ?? interpLabel("ui.common.select")} />
      </SelectTrigger>
      <SelectContent>
        {component.items.map((item, index) => (
          <SelectItem key={`${record.id}:${index}:${item.value}`} value={item.value}>
            {item.label}
          </SelectItem>
        ))}
      </SelectContent>
    </Select>
  );
}

export function ToggleView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "toggle" }>;
  if (component.appearance === "checkbox") return <TreeCheckbox id={nodeDomId(context.store, record, context.domScope)} checked={component.on} disabled={record.disabled} ariaLabel={record.accessibility.label ?? component.text ?? undefined} onCheckedChange={(checked) => dispatchTrigger(context, record, "change", toUiValue(checked))} />;
  return <Toggle id={nodeDomId(context.store, record, context.domScope)} data-ui-node-id={record.id} data-ui-node-key={record.key} pressed={component.on} text={component.text ?? undefined} icon={resolveControlIconNode(component.icon)} disabled={record.disabled} aria-label={record.accessibility.label ?? component.text ?? undefined} onPressedChange={(pressed) => dispatchTrigger(context, record, "change", toUiValue(pressed))} />;
}

export function SliderView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "slider" }>;
  const lane = useContinuousTriggerLane(context, record);
  const slider = (
    <Slider
      id={nodeDomId(context.store, record, context.domScope)}
      data-ui-node-id={record.id} data-ui-node-key={record.key}
      aria-label={record.accessibility.label ?? undefined}
      aria-valuetext={uiAccessibilityValueV1(component).valueText ?? undefined}
      className="w-full min-w-0"
      max={component.max}
      min={component.min}
      step={component.step}
      snapValues={component.snaps}
      scale={component.scale ?? "linear"}
      appearance={component.appearance ?? "track"}
      displayFactor={component.displayFactor ?? null}
      precision={component.precision ?? null}
      limits={component.limits ?? null}
      value={[component.value]}
      onValueChange={(values) => lane.offer(toUiValue(values[0] ?? component.value))}
      onValueCommit={(values) => lane.commit(toUiValue(values[0] ?? component.value))}
      onPointerCancel={() => lane.abort("captureLost")}
      onBlur={() => lane.abort("blur")}
    />
  );
  const unit = component.displayUnit ?? component.unit;
  if (!unit) return slider;
  return (
    <div className="flex min-w-0 w-full items-center gap-single">
      {slider}
      <span data-slot="slider-unit" className="text-muted-foreground shrink-0 text-xs tabular-nums">
        {uiNumberDisplayText(component.value, component.displayFactor ?? null, component.precision ?? null)} {unit}
      </span>
    </div>
  );
}

export function NumberStepperView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "numberStepper" }>;
  const lane = useContinuousTriggerLane(context, record);
  return (
    <Stepper
      id={nodeDomId(context.store, record, context.domScope)}
      aria-label={record.accessibility.label ?? undefined}
      disabled={record.disabled}
      step={component.step}
      min={component.min ?? undefined}
      max={component.max ?? undefined}
      precision={component.precision ?? undefined}
      snapValues={component.snaps ?? []}
      displayFactor={component.displayFactor ?? null}
      unit={component.displayUnit ?? component.unit ?? null}
      limits={component.limits ?? null}
      aria-valuetext={uiAccessibilityValueV1(component).valueText ?? undefined}
      value={component.uniform ? component.value : undefined}
      mixed={!component.uniform}
      onChange={(value) => lane.offer(toUiValue(value))}
      onPointerUp={() => lane.commit()}
      onPointerCancel={() => lane.commit()}
      onDelta={(record.bindings ?? []).some((binding) => binding.trigger === "delta") ? (delta) => dispatchTrigger(context, record, "delta", toUiValue(delta)) : undefined}
    />
  );
}

/** 💍️ A dragged orb IS a continuous control (design §13.1 of ticket 26/09/30/NON-DESTRUCTIVE-HISTORY-EDITING): every
 * position rides the press lane, the pointer's release ends the press on the value last offered (deferred past `Ring`'s
 * own release handler, which reports the final position), and a cancelled pointer drops it with zero trace. */
export function RingView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "ring" }>;
  const lane = useContinuousTriggerLane(context, record);
  useEffect(() => {
    const release = () => queueMicrotask(() => {
      if (lane.open()) lane.commit();
    });
    const cancel = () => lane.abort("captureLost");
    window.addEventListener("pointerup", release);
    window.addEventListener("pointercancel", cancel);
    return () => {
      window.removeEventListener("pointerup", release);
      window.removeEventListener("pointercancel", cancel);
    };
  }, [lane]);
  return <Ring id={nodeDomId(context.store, record, context.domScope)} onOrbChange={(_orbId, _oldT, newT) => lane.offer(toUiValue(newT))} orbs={[{ disabled: record.disabled, id: component.orbId, selected: true, t: component.t }]} />;
}

/** 🖼️ An icon picker named by its record's label (its kind select and its field both read it) and locked while the record
 * is disabled — an editor control like any other, dispatching the picked icon on `Trigger::Change`. */
export function IconSelectView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "iconSelect" }>;
  const id = nodeDomId(context.store, record, context.domScope);
  const label = record.accessibility.label ?? null;
  return (
    <>
      {label === null ? null : (
        <span id={`${id}.label`} className="sr-only">
          {label}
        </span>
      )}
      <IconSelector
        aria-labelledby={label === null ? undefined : `${id}.label`}
        classifyIconSelectorMode={component.classifierKind === "puzzle2d" ? classifyIconSelectorMode : undefined}
        disabled={record.disabled}
        id={id}
        onChange={(next) => dispatchTrigger(context, record, "change", toUiValue(next))}
        uniform={component.uniform}
        value={component.value}
      />
    </>
  );
}
//#endregion ComponentRenderers
//#endregion 🔖️UiInterpreter
