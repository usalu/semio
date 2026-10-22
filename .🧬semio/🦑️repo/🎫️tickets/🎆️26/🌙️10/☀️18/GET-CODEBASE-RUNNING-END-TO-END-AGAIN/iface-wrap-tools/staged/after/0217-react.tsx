/** 📝️ Window panes, staged args: the staged argument controls (reference, color, vector, number) and the form gate of an action. */
import { type ActionArgControl, type ActionDefinition, argControl, actionArgNumberFacets, type ActionArgNumberFacets, referenceIdText, referenceIdValue, type ReferenceIdType } from "@semio-tech/framework";
import { Icon, IconSelector, Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Slider, Stepper, Textarea, Toggle, ToggleGroup, uiDataLabel, type UIDialogFieldBinding } from "@semio-tech/ui-react";
import { type ReactElement, useState } from "../../../⚛️react-port/🟦️.ts";
import { parseUiColorHex, uiColorHex, uiNumberCrossedBound, uiNumberDisplay, uiNumberDisplayText, uiNumberFieldKey, uiNumberKeyValue, uiNumberTypedValue } from "@semio-tech/ui-contract/component";
import type { InteractionState } from "@semio-tech/framework-interaction";
import { actionArgStringOptions, currentShellLocaleV1, type ResolvedActionArgDef, shellLabel } from "../../🏷️labels/🟦️.tsx";

//#region ShellHelpers
//#region 🧰️WindowActionPane
/** 🎛️ What a staged field reads from the shell beyond its own value: the live interaction selection of a domain (every
 * domain's when the reference names none) a reference input's "use current selection" takes. */
export type StagedArgContextV1 = { readonly selection?: (domain: string | undefined) => readonly string[] };

/** 🕹️ The ids selected in `domain` of an interaction state, or in every domain when `domain` is absent. */
export function interactionSelectionIdsV1(state: InteractionState, domain: string | undefined): readonly string[] {
  return domain === undefined ? Object.values(state.selection).flatMap((entry) => entry.ids) : (state.selection[domain]?.ids ?? []);
}

/** 🪧️ A staged number as the human reads it: `stored × displayFactor` at `precision` (`uiNumberDisplayText`), followed by
 * the display unit (else the stored unit). */
export function stagedNumberDisplayText(value: number, facets: Pick<ActionArgNumberFacets, "precision" | "displayFactor" | "displayUnit" | "unit">): string {
  const text = uiNumberDisplayText(value, facets.displayFactor, facets.precision);
  const unit = facets.displayUnit ?? facets.unit;
  return unit ? `${text} ${unit}` : text;
}

/** 📐️ The UI-contract number facets of a staged number, stepper, slider, dial or vector input — `actionArgNumberFacets`, the
 * twin of the Rust `ActionArgDef::number_facets` the wgpu shell's staged rows read (`🛂️manifest/🧫️fixtures/🧫️number-facets`):
 * key range, step, look, axis, unit symbols, display factor, precision, admitted detents and the hard limits whose
 * refusals read in the shell's current language. */
export function stagedNumberFacetsV1(def: ResolvedActionArgDef): ActionArgNumberFacets {
  const facets = actionArgNumberFacets(def, currentShellLocaleV1());
  if (facets === null) throw new Error(`Staged input ${def.id} has no number facets`);
  return facets;
}

/** 🧷️ The ids a reference input holds, whether it takes many or one, as the text a selection spells them in. */
function stagedReferenceIds(value: unknown): readonly string[] {
  return (Array.isArray(value) ? value : [value]).flatMap((id) => referenceIdText(id) ?? []);
}

/** 🧷️ A staged reference input with the W1-E `reference_list` recipe semantics: one chip per referenced id that removes
 * it on activation (announced as "Remove <id>"), an empty line while there is none, and a "use current selection"
 * button that takes the live selection (capped at `maxItems`, the first id for a single reference). */
function StagedReferenceField({ id, labelledBy, value, many, maxItems, domain, idType, disabled, selection, onChange }: { readonly id: string; readonly labelledBy?: string; readonly value: unknown; readonly many: boolean; readonly maxItems?: number; readonly domain?: string; readonly idType?: ReferenceIdType; readonly disabled?: boolean; readonly selection?: StagedArgContextV1["selection"]; readonly onChange: (value: unknown) => void }): ReactElement {
  const ids = stagedReferenceIds(value);
  const removeLabel = (item: string) => String(shellLabel("ui.referenceList.remove", { item }));
  const commit = (next: readonly string[]) => {
    const values = next.flatMap((text) => referenceIdValue(idType, text) ?? []);
    onChange(many ? values : (values[0] ?? (idType === "integer" ? null : "")));
  };
  return (
    <div id={id} role="group" aria-labelledby={labelledBy} data-staged-reference="" className="flex w-full min-w-0 flex-col gap-tiny">
      {ids.length === 0 ? (
        <span data-staged-reference-empty="" className="text-xs text-muted-foreground">{shellLabel("ui.referenceList.empty")}</span>
      ) : (
        <div role="toolbar" aria-labelledby={labelledBy} className="flex flex-wrap gap-tiny">
          {ids.map((item) => (
            <button key={item} type="button" data-staged-reference-chip={item} aria-label={removeLabel(item)} title={removeLabel(item)} disabled={disabled} className="inline-flex items-center gap-tiny rounded-sm border px-single text-xs" onClick={() => commit(ids.filter((other) => other !== item))}>
              {item}
              <Icon icon="x" size="small" />
            </button>
          ))}
        </div>
      )}
      <button type="button" data-staged-reference-use-selection="" disabled={disabled || selection === undefined} className="self-start text-xs underline" onClick={() => {
        const picked = [...new Set(selection?.(domain) ?? [])];
        commit(many ? picked.slice(0, maxItems ?? picked.length) : picked.slice(0, 1));
      }}>
        {shellLabel("ui.referenceList.useSelection")}
      </button>
    </div>
  );
}

/** 🎨️ A staged colour with the W1-E `color_input` recipe semantics: the swatch (the platform's colour picker, named by
 * the field), the hex field — typed hex is staged once it parses (`parseUiColorHex`), on Enter or blur, keeping the
 * alpha when the text carries none — and, for a colour with alpha, the opacity slider over 0..1. sRGB components in
 * 0..1, straight alpha. */
function StagedColorField({ id, labelledBy, value, alpha, disabled, required, onChange }: { readonly id: string; readonly labelledBy?: string; readonly value: unknown; readonly alpha: boolean; readonly disabled?: boolean; readonly required?: boolean; readonly onChange: (value: unknown) => void }): ReactElement {
  const components = Array.isArray(value) ? value : [];
  const rgba = [0, 1, 2, 3].map((index) => {
    const component = Number(components[index] ?? (index === 3 ? 1 : 0));
    return Number.isFinite(component) ? Math.min(1, Math.max(0, component)) : 0;
  });
  const [draft, setDraft] = useState<string | null>(null);
  const commit = (next: readonly number[]) => onChange(alpha ? [...next] : next.slice(0, 3));
  const commitHex = (text: string) => {
    setDraft(null);
    const parsed = parseUiColorHex(text);
    if (parsed) commit(/^#?([0-9a-f]{4}|[0-9a-f]{8})$/i.test(text.trim()) ? parsed : [...parsed.slice(0, 3), rgba[3]!]);
  };
  return (
    <div id={id} role="group" aria-labelledby={labelledBy} data-staged-control="color" className="flex w-full min-w-0 flex-wrap items-center gap-single">
      <input
        id={`${id}.swatch`}
        type="color"
        aria-labelledby={labelledBy}
        className="h-medium w-medium shrink-0 cursor-pointer rounded-sm border"
        value={uiColorHex(rgba, false)}
        disabled={disabled}
        onChange={(event) => {
          const parsed = parseUiColorHex(event.target.value);
          if (parsed) commit([...parsed.slice(0, 3), rgba[3]!]);
        }}
      />
      <Input
        id={`${id}.hex`}
        aria-label={String(shellLabel("ui.colorInput.hex"))}
        required={required}
        type="text"
        spellCheck={false}
        className="h-medium w-[7rem] min-w-0 font-mono"
        value={draft ?? uiColorHex(rgba, alpha)}
        disabled={disabled}
        onChange={(event) => setDraft(event.target.value)}
        onBlur={(event) => commitHex(event.target.value)}
        onKeyDown={(event) => {
          if (event.key !== "Enter") return;
          event.preventDefault();
          commitHex(event.currentTarget.value);
        }}
      />
      {alpha ? <Slider id={`${id}.alpha`} aria-label={String(shellLabel("ui.colorInput.alpha"))} className="min-w-[6rem] flex-1" min={0} max={1} step={0.01} value={[rgba[3]!]} disabled={disabled} onValueChange={(values) => commit([...rgba.slice(0, 3), values[0] ?? rgba[3]!])} /> : null}
    </div>
  );
}

/** 🧲️ A staged vector with the W1-E `vector_input` recipe semantics: one {@link StagedNumberField} per component, named
 * "x (mm)" and so on, every axis sharing the vector's facets. */
function StagedVectorField({ id, labelledBy, value, dims, facets, disabled, required, onChange }: { readonly id: string; readonly labelledBy?: string; readonly value: unknown; readonly dims: number; readonly facets: ActionArgNumberFacets; readonly disabled?: boolean; readonly required?: boolean; readonly onChange: (value: unknown) => void }): ReactElement {
  const tuple = Array.isArray(value) && value.length >= dims ? (value as readonly number[]) : null;
  const axes = Array.from({ length: dims }, (_, index) => ["x", "y", "z", "w"][index] ?? String(index));
  const unit = facets.displayUnit ?? facets.unit;
  const commit = (index: number, next: number) => onChange(axes.map((_, component) => (component === index ? next : (tuple?.[component] ?? 0))));
  return (
    <div id={id} role="group" aria-labelledby={labelledBy} data-staged-control="vector" className="flex w-full min-w-0 flex-wrap gap-single">
      {axes.map((axis, index) => (
        <StagedNumberField key={`${id}.${axis}`} id={`${id}.${axis}`} label={unit ? `${axis} (${unit})` : axis} stored={tuple === null ? null : (tuple[index] ?? 0)} facets={facets} disabled={disabled} required={required} onCommit={(next) => commit(index, next)} />
      ))}
    </div>
  );
}

/** 🗜️ A staged number field — a number input or one vector axis — under the number-control law (design §18,
 * `🖱️ui/🧬️contract/🧫️fixtures/🧫️number-controls`) and its facets: it shows `stored × displayFactor` at the precision; the
 * keys `uiNumberFieldKey` names (Home/End only toward a key bound) move the stored value through `uiNumberKeyValue`; a typed
 * display value reads back through `uiNumberTypedValue` (an exact detent or the current value kept, else rounded half away
 * from zero and divided by the display factor) and is staged at once — unless it is unreadable or crosses a hard limit,
 * which keeps the draft, marks the field invalid and names the limit's refusal; an emptied field clears (`onClear`) where
 * the input may be unset. A stored value changed from outside (Reset, a default) replaces a draft that does not read back
 * to it. A lone field shows its unit beside it; an axis names it in its label. */
function StagedNumberField({ id, label, labelledBy, unit, stored, facets, disabled, required, onCommit, onClear }: { readonly id: string; readonly label?: string; readonly labelledBy?: string; readonly unit?: string; readonly stored: number | null; readonly facets: ActionArgNumberFacets; readonly disabled?: boolean; readonly required?: boolean; readonly onCommit: (next: number) => void; readonly onClear?: () => void }): ReactElement {
  const [draft, setDraft] = useState<string | null>(null);
  const [refusal, setRefusal] = useState<{ readonly message: string | null } | null>(null);
  const factor = facets.displayFactor ?? null;
  const precision = facets.precision ?? null;
  const shown = (value: number) => uiNumberDisplayText(value, factor, precision);
  const step = facets.step ?? (precision === null ? undefined : 10 ** -precision / (factor ?? 1));
  const refusalId = `${id}.refusal`;
  const [seen, setSeen] = useState(stored);
  if (seen !== stored) {
    setSeen(stored);
    const number = Number(draft?.trim() ?? "");
    const readBack = draft === null || draft.trim() === "" || !Number.isFinite(number) ? null : uiNumberTypedValue(number, factor, precision, [stored ?? 0, ...facets.snaps]);
    if (draft !== null && readBack !== stored) {
      setDraft(null);
      setRefusal(null);
    }
  }
  const typed = (text: string) => {
    setDraft(text);
    const number = Number(text.trim());
    if (text.trim() === "" && onClear !== undefined) {
      setRefusal(null);
      onClear();
      return;
    }
    if (text.trim() === "" || !Number.isFinite(number)) {
      setRefusal({ message: null });
      return;
    }
    const next = uiNumberTypedValue(number, factor, precision, [stored ?? 0, ...facets.snaps]);
    const crossed = uiNumberCrossedBound(next, facets.min, facets.max, facets.limits);
    if (crossed !== null) {
      setRefusal({ message: crossed.refusal ?? null });
      return;
    }
    setRefusal(null);
    onCommit(next);
  };
  const input = (
    <Input
      id={id}
      aria-labelledby={labelledBy}
      required={required}
      type="number"
      className="h-medium w-full min-w-0"
      value={draft ?? (stored === null ? "" : shown(stored))}
      min={facets.min === undefined ? undefined : uiNumberDisplay(facets.min, factor)}
      max={facets.max === undefined ? undefined : uiNumberDisplay(facets.max, factor)}
      step={step === undefined ? undefined : uiNumberDisplay(step, factor)}
      disabled={disabled}
      aria-invalid={refusal === null ? undefined : true}
      aria-describedby={refusal?.message ? refusalId : undefined}
      onKeyDown={(event) => {
        const key = uiNumberFieldKey(event.key, event.shiftKey, facets.min, facets.max);
        if (key === null) return;
        event.preventDefault();
        setDraft(null);
        setRefusal(null);
        onCommit(uiNumberKeyValue(stored ?? 0, facets.min ?? null, facets.max ?? null, facets.step ?? 0, precision, factor, facets.snaps, key.key, key.large));
      }}
      onChange={(event) => typed(event.target.value)}
      onBlur={() => {
        if (refusal === null) setDraft(null);
      }}
    />
  );
  return (
    <div className="flex min-w-[4rem] flex-1 flex-col gap-tiny">
      <div className="flex items-center gap-tiny text-xs text-muted-foreground">
        {label === undefined ? null : <label htmlFor={id}>{label}</label>}
        {input}
        {unit === undefined ? null : <span data-slot="staged-unit" className="shrink-0 tabular-nums">{unit}</span>}
      </div>
      {refusal?.message ? <span id={refusalId} role="alert" data-staged-refusal="" className="text-xs text-destructive">{refusal.message}</span> : null}
    </div>
  );
}

/**
 * 🎛️ Renders one {@link ActionArgControl} into a STAGED form field — the crucial difference from
 * `renderUiControl` in `ui-interpreter.tsx` is that this dispatches NOTHING globally; `onChange` only
 * writes to the caller's local staged buffer. `value` is the already-resolved effective value
 * (staged ?? default ?? unset). Stepper, dial (a detented slider read in display units), segmented choice (a radio group),
 * multi-line text (a textarea: Enter is its line break, the form's own submit commits), vector axes and reference chips
 * follow the W1-E contract recipes.
 */
export function renderStagedArgControl(def: ResolvedActionArgDef, value: unknown, onChange: (value: unknown) => void, disabled?: boolean, field?: UIDialogFieldBinding, context: StagedArgContextV1 = {}): ReactElement {
  const control = renderStagedArgValueControl(def, value, onChange, disabled, field, context);
  if (!def.nullable) return control;
  const cleared = value === null;
  return (
    <div className="flex w-full min-w-0 items-center gap-single" data-staged-nullable="">
      <div className="min-w-0 flex-1">{control}</div>
      <button type="button" aria-pressed={cleared} aria-describedby={field?.labelledBy} disabled={disabled} className="shrink-0 text-xs underline" onClick={() => onChange(cleared ? undefined : null)}>
        {shellLabel("ui.nullableInput.clear")}
      </button>
    </div>
  );
}

/** 🎛️ The value control of one staged argument, without the clear toggle a nullable input adds around it. */
function renderStagedArgValueControl(def: ResolvedActionArgDef, value: unknown, onChange: (value: unknown) => void, disabled?: boolean, field?: UIDialogFieldBinding, context: StagedArgContextV1 = {}): ReactElement {
  const control: ActionArgControl = argControl(def);
  const fieldId = field?.id ?? def.id;
  const labelledBy = field?.labelledBy;
  switch (control.kind) {
    case "text":
      return <Input id={fieldId} aria-labelledby={labelledBy} required={field?.required} type="text" className="h-medium w-full min-w-0" value={typeof value === "string" ? value : ""} placeholder={control.placeholder} disabled={disabled} onChange={(event) => onChange(event.target.value)} />;
    case "multiline":
      return <Textarea id={fieldId} aria-labelledby={labelledBy} required={field?.required} data-staged-control="multiline" className="min-h-[4.5rem] w-full min-w-0" value={typeof value === "string" ? value : ""} disabled={disabled} onChange={(event) => onChange(event.target.value)} />;
    case "stepper": {
      const facets = stagedNumberFacetsV1(def);
      const numeric = typeof value === "number" && Number.isFinite(value) ? value : undefined;
      return <Stepper id={fieldId} aria-labelledby={labelledBy} aria-valuetext={numeric === undefined ? undefined : stagedNumberDisplayText(numeric, facets)} value={numeric} defaultValue={facets.min ?? 0} min={facets.min} max={facets.max} step={facets.step ?? 1} precision={facets.precision} snapValues={facets.snaps} displayFactor={facets.displayFactor ?? null} unit={facets.displayUnit ?? facets.unit ?? null} limits={facets.limits} disabled={disabled} onChange={(next) => onChange(next)} />;
    }
    case "slider":
    case "dial": {
      const facets = stagedNumberFacetsV1(def);
      const numeric = typeof value === "number" && Number.isFinite(value) ? value : (facets.min ?? control.min);
      return <Slider id={fieldId} aria-labelledby={labelledBy} aria-valuetext={stagedNumberDisplayText(numeric, facets)} data-staged-control={control.kind} className="w-full min-w-0" min={facets.min ?? control.min} max={facets.max ?? control.max} step={facets.step} snapValues={facets.snaps} scale={facets.scale} appearance={facets.appearance} displayFactor={facets.displayFactor ?? null} precision={facets.precision ?? null} limits={facets.limits} formatDisplayValue={(shown) => stagedNumberDisplayText(shown, facets)} value={[numeric]} disabled={disabled} onValueChange={(values) => onChange(values[0] ?? numeric)} />;
    }
    case "number": {
      const facets = stagedNumberFacetsV1(def);
      return <StagedNumberField id={fieldId} labelledBy={labelledBy} unit={facets.displayUnit ?? facets.unit} stored={typeof value === "number" && Number.isFinite(value) ? value : null} facets={facets} disabled={disabled} required={field?.required} onCommit={(next) => onChange(next)} onClear={() => onChange(undefined)} />;
    }
    case "toggle":
      return <Toggle id={fieldId} aria-labelledby={labelledBy} icon="check" pressed={value === true} text={uiDataLabel(def.label)} disabled={disabled} onPressedChange={(pressed) => onChange(pressed)} />;
    case "segmented": {
      if (def.schema.kind !== "string") throw new Error("Segmented control requires a string argument schema");
      const options = actionArgStringOptions(def.schema);
      return (
        <ToggleGroup
          id={fieldId}
          kind="single"
          semantics="radio"
          aria-labelledby={labelledBy}
          data-staged-control="segmented"
          disabled={disabled || options.length === 0}
          value={typeof value === "string" && options.some((option) => option.value === value) ? value : ""}
          onValueChange={(next) => { if (next !== "") onChange(next); }}
          items={options.map((option) => ({ value: option.value, icon: "circle-dot", text: option.label }))}
        />
      );
    }
    case "artifactKind":
    case "surfaceApp":
    case "select": {
      if (def.schema.kind !== "string") throw new Error("Select control requires a string argument schema");
      return (
        <Select id={def.id} value={typeof value === "string" && actionArgStringOptions(def.schema).some(option => option.value === value) ? value : ""} disabled={disabled || actionArgStringOptions(def.schema).length === 0} onValueChange={(next) => onChange(next)}>
          <SelectTrigger id={fieldId} aria-labelledby={labelledBy} aria-required={field?.required} className="h-medium w-full min-w-0" size="sm">
            <SelectValue placeholder={def.label} />
          </SelectTrigger>
          <SelectContent>
            {actionArgStringOptions(def.schema).map((option, index) => (
              <SelectItem key={`${def.id}:${index}:${option.value}`} value={option.value}>
                {option.label}
              </SelectItem>
            ))}
          </SelectContent>
        </Select>
      );
    }
    case "reference":
      return <StagedReferenceField id={fieldId} labelledBy={labelledBy} value={value} many={control.many ?? false} maxItems={control.maxItems} domain={control.domain} idType={control.idType} disabled={disabled} selection={context.selection} onChange={onChange} />;
    case "vector":
      return <StagedVectorField id={fieldId} labelledBy={labelledBy} value={value} dims={control.dims} facets={stagedNumberFacetsV1(def)} disabled={disabled} required={field?.required} onChange={onChange} />;
    case "color":
      return <StagedColorField id={fieldId} labelledBy={labelledBy} value={value} alpha={control.alpha} disabled={disabled} required={field?.required} onChange={onChange} />;
    case "iconSelect":
      return <IconSelector id={fieldId} aria-labelledby={labelledBy} disabled={disabled} classifyIconSelectorMode={undefined} value={typeof value === "string" ? value : ""} uniform onChange={(next) => onChange(next)} />;
  }
}

/** 🧰️ True when an action carries a user-facing argument and therefore stages a form instead of firing immediately
 * (P1–P4). Hidden arguments are never entered by a user — the framework ToolRun verbs resolve their run or active
 * tool when a chord fires them bare — so an action whose every argument is hidden fires. */
export function actionRequiresStagedForm(action: Pick<ActionDefinition, "args">): boolean {
  return (action.args ?? []).some((arg) => arg.presentation?.kind !== "hidden");
}
//#endregion 🧰️WindowActionPane
//#endregion ShellHelpers
