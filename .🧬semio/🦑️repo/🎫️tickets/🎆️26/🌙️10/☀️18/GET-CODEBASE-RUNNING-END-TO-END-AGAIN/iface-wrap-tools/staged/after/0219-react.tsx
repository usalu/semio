/** 🎚️ Window panes, window measures: the sliders, selects, toggles and numbers a window declares as measures, their tree items, overlay and chrome and the dom id qualification. */
import { type ActionDescriptor, partitionWindowMeasures, type WindowMeasure } from "@semio-tech/framework";
import { Icon, type IconName, Slider, type TreeDataItem, uiDataLabel, WindowMeasuresTree, WindowMeasureTreeGroup, WindowMeasureTreeLeaf } from "@semio-tech/ui-react";
import { type ReactNode, useMemo } from "../../../⚛️react-port/🟦️.ts";
import { WindowMeasureNumber, WindowMeasureSelect, WindowMeasureToggle } from "../../🎚️measure-controls/🟦️.tsx";
import { createDirectionalAsyncDispatcher } from "../../🛑️extension-io/🟦️.ts";

//#region ShellHelpers
/** 🎚️ Whether any measure (including nested group children) declares `id`. */
export function windowMeasureTreeContainsId(measures: readonly WindowMeasure[], id: string): boolean {
  for (const measure of measures) {
    if (measure.id === id) return true;
    if (measure.kind === "group" && windowMeasureTreeContainsId(measure.children, id)) return true;
  }
  return false;
}

/** 📊️ Probability weights (0–1 simplex sliders) read out as whole-percent labels, not raw fractions. */
function windowMeasureUsesProbabilityReadout(measure: Extract<WindowMeasure, { kind: "slider" }>): boolean {
  const step = measure.step ?? 1;
  return measure.min === 0 && measure.max <= 1 && step < 1;
}

function windowMeasureProbabilityReadout(value: number): string {
  return `${Math.round(value * 100)}%`;
}

/** 🎚️ Keeps a measure slider live without accumulating stale document actions behind the pointer. */
function WindowMeasureSlider({ measure, onAction }: { readonly measure: Extract<WindowMeasure, { kind: "slider" }>; readonly onAction: (action: ActionDescriptor) => unknown }) {
  const dispatchValue = useMemo(
    () => createDirectionalAsyncDispatcher((value) => onAction({ ...measure.onChange, args: { ...(measure.onChange.args as object | undefined), value } })),
    [measure.onChange, onAction],
  );
  const formatDisplayValue = windowMeasureUsesProbabilityReadout(measure) ? windowMeasureProbabilityReadout : undefined;
  const disabled = measure.disabled === true;

  return (
    <Slider
      id={measure.id}
      data-published-value={String(measure.value)}
      value={[measure.value]}
      min={measure.min}
      max={measure.max}
      ready={measure.ready}
      loading={measure.loading === true}
      waiting={measure.waiting === true}
      step={measure.step}
      disabled={disabled}
      formatDisplayValue={formatDisplayValue}
      onValueChange={(values) => {
        if (disabled) return;
        dispatchValue(values[0] ?? measure.value);
      }}
    />
  );
}

function windowMeasureGroupHeaderSlider(measure: Extract<WindowMeasure, { kind: "group" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode | undefined {
  if (measure.value === undefined || measure.onChange === undefined) return undefined;
  const sliderMeasure: Extract<WindowMeasure, { kind: "slider" }> = {
    kind: "slider",
    id: `${measure.id}.header-slider`,
    label: undefined,
    value: measure.value,
    min: measure.min ?? 0,
    max: measure.max ?? 1,
    step: measure.step,
    ready: measure.ready,
    loading: measure.loading,
    waiting: measure.waiting,
    onChange: measure.onChange,
  };
  return <WindowMeasureSlider measure={sliderMeasure} onAction={onAction} />;
}

function windowMeasureSelectControl(measure: Extract<WindowMeasure, { kind: "select" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  return <WindowMeasureSelect measure={measure} onAction={onAction} />;
}

function windowMeasureToggleControl(measure: Extract<WindowMeasure, { kind: "toggle" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  return <WindowMeasureToggle measure={measure} onAction={onAction} />;
}

function windowMeasureNumberControl(measure: Extract<WindowMeasure, { kind: "number" }>, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  return <WindowMeasureNumber measure={measure} onAction={onAction} />;
}

function windowMeasureToggleIcon(measure: Extract<WindowMeasure, { kind: "toggle" }>): ReactNode {
  return <Icon icon={measure.iconId as IconName} size={12} />;
}

/**
 * 🌲️ Maps window measures to native panel-tree rows — same chrome as left-corner trees (label left, control right, guide lines).
 * Pre-reverses top-level measures so bottom-anchored `direction="up"` panels read Count at the bottom, Distribution above.
 */
export function windowMeasuresToTreeItems(measures: readonly WindowMeasure[], onAction: (action: ActionDescriptor) => unknown, reverseForUpPanel = true): TreeDataItem[] {
  const ordered = reverseForUpPanel ? [...measures].reverse() : [...measures];
  const mapMeasure = (measure: WindowMeasure): TreeDataItem => {
    if (measure.kind === "group") {
      return {
        id: measure.id,
        label: measure.label,
        defaultOpen: measure.defaultOpen,
        control: windowMeasureGroupHeaderSlider(measure, onAction),
        items: measure.children.length > 0 ? windowMeasuresToTreeItems(measure.children, onAction, false) : undefined,
      };
    }
    if (measure.kind === "slider") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: <WindowMeasureSlider measure={measure} onAction={onAction} />,
        loading: measure.loading,
        waiting: measure.waiting,
      };
    }
    if (measure.kind === "select") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: windowMeasureSelectControl(measure, onAction),
      };
    }
    if (measure.kind === "number") {
      return {
        id: measure.id,
        label: measure.label ?? "",
        control: windowMeasureNumberControl(measure, onAction),
        loading: measure.loading,
        waiting: measure.waiting,
      };
    }
    return {
      id: measure.id,
      label: measure.label ?? measure.text ?? "",
      icon: windowMeasureToggleIcon(measure),
      control: windowMeasureToggleControl(measure, onAction),
    };
  };
  return ordered.map(mapMeasure);
}

function renderWindowMeasure(measure: WindowMeasure, onAction: (action: ActionDescriptor) => unknown): ReactNode {
  if (measure.kind === "group") {
    const headerSlider = windowMeasureGroupHeaderSlider(measure, onAction);
    return (
      <WindowMeasureTreeGroup key={measure.id} id={measure.id} label={measure.label} defaultOpen={measure.defaultOpen} headerControl={headerSlider}>
        {measure.children.map((child) => renderWindowMeasure(child, onAction))}
      </WindowMeasureTreeGroup>
    );
  }
  if (measure.kind === "select") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label === undefined ? undefined : uiDataLabel(measure.label)}>
        {windowMeasureSelectControl(measure, onAction)}
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "slider") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label === undefined ? undefined : uiDataLabel(measure.label)}>
        <WindowMeasureSlider measure={measure} onAction={onAction} />
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "number") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label === undefined ? undefined : uiDataLabel(measure.label)}>
        {windowMeasureNumberControl(measure, onAction)}
      </WindowMeasureTreeLeaf>
    );
  }
  if (measure.kind === "toggle") {
    return (
      <WindowMeasureTreeLeaf key={measure.id} label={measure.label ?? measure.text ? uiDataLabel(measure.label ?? measure.text ?? "") : undefined} icon={windowMeasureToggleIcon(measure)}>
        {windowMeasureToggleControl(measure, onAction)}
      </WindowMeasureTreeLeaf>
    );
  }
  return null;
}

function windowMeasuresOverlay(measures: readonly WindowMeasure[] | undefined, onAction: (action: ActionDescriptor) => unknown, direction: "up" | "down" = "down"): ReactNode | undefined {
  if (!measures || measures.length === 0) return undefined;
  return <WindowMeasuresTree direction={direction}>{measures.map((measure) => renderWindowMeasure(measure, onAction))}</WindowMeasuresTree>;
}

/** 🪟️ Public window-options tree for measures rails and tests — icon before label, checkbox for toggles. */
export function renderWindowMeasuresTree(measures: readonly WindowMeasure[], onAction: (action: ActionDescriptor) => unknown, direction: "up" | "down" = "down"): ReactNode | undefined {
  return windowMeasuresOverlay(measures, onAction, direction);
}

/** 🪪️ THE DOM identity of one window-measure control: the WINDOW INSTANCE it is rendered for, then the
 * program-authored measure id. Same rule and same separator as `uiNodeDomId` — a window kind's measure tree is
 * authored ONCE for the kind (`world3d_projection_measures` takes a kind-level `id_prefix`, and puzzle3d passes
 * the literal `"puzzle3d"`) and then rendered once per OPEN INSTANCE of that kind, so the authored id alone puts
 * `puzzle3d-measure-projection-orthographic-view` in the document once per pane the moment more than one
 * measures rail is unfolded. Duplicate ids are invalid HTML, they make the control unaddressable through
 * `<label for>`/`aria-labelledby`/automation, and they also collapse the rail's own fold state
 * (`WindowMeasureTreeGroup` keys `useTreeOpenState` on the group id), so unfolding "Parallel" in one pane
 * unfolded it in the other. */
export function windowMeasureDomId(windowId: string, measureId: string): string {
  return `${windowId}/${measureId}`;
}

/** 🪪️ {@link windowMeasureDomId} applied to a whole authored measure subtree — ids only, every other field
 * (labels, values, `reveal` group, `onChange`) verbatim, so the authored id stays the one integration key the
 * program, `windowMeasureTreeContainsId` and the `activeUtilityId` routing all speak. */
export function qualifyWindowMeasureIds(measures: readonly WindowMeasure[], windowId: string): WindowMeasure[] {
  return measures.map((measure) =>
    measure.kind === "group"
      ? { ...measure, id: windowMeasureDomId(windowId, measure.id), children: qualifyWindowMeasureIds(measure.children, windowId) }
      : { ...measure, id: windowMeasureDomId(windowId, measure.id) },
  );
}

export function windowMeasuresChrome(
  measures: readonly WindowMeasure[] | undefined,
  activeUtilityId: string | undefined,
  windowId: string,
  onAction: (action: ActionDescriptor) => unknown,
): { readonly measures: ReactNode | undefined; readonly utilityOptions: ReactNode | undefined } {
  const partitioned = partitionWindowMeasures(measures ?? [], activeUtilityId);
  const general = qualifyWindowMeasureIds(partitioned.general, windowId);
  const utilityOptions = qualifyWindowMeasureIds(partitioned.utilityOptions, windowId);
  const taggedOnAction = (action: ActionDescriptor) => onAction({ ...action, args: { ...(action.args as object | undefined), windowId } });
  return {
    measures: windowMeasuresOverlay(general, taggedOnAction),
    utilityOptions: windowMeasuresOverlay(utilityOptions, taggedOnAction, "up"),
  };
}
//#endregion ShellHelpers
