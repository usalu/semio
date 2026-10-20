/** 🔤️ The display-only retained views: text, button, key-value list, image, progress, extension and unknown component, the busy shell and the activity border.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import { useEffect, useRef, useState, type ReactNode } from "../../⚛️react-port/🟦️.ts";
import {
  Button,
  cn,
  elementSkeleton,
  loadingBorderElementClass,
  waitingBorderElementClass,
  type ElementSkeletonKind
} from "@semio-tech/ui-react";
import { type Component, type UiNodeRecord } from "@semio-tech/framework";
import { uiAccessibilityValueV1, uiProgressFractionV1 } from "@semio-tech/ui-contract/presentation/accessibility";
import { shellLabel } from "../../🛠️ShellHelpers/🏷️labels/🟦️.tsx";
import { ShellFaultBoundary } from "../../🐚️Shell/🧯️fault-boundary/🟦️.tsx";
import { resolveExtensionHost } from "../🗺️element-host-registry/🟦️.ts";
import { type UiInterpreterContext } from "../🧷️interpreter-context/🟦️.ts";
import { accessibilityAriaProps } from "../📐️layout-accessibility/🟦️.tsx";
import { dispatchTrigger, resolveControlIconNode } from "../⚡️action-dispatch/🟦️.tsx";
import { nodeDomId } from "../🪪️stable-dom-ids/🟦️.ts";

//#region 🔖️UiInterpreter
//#region ComponentRenderers
export function activityBorderClass(record: UiNodeRecord): string | undefined {
  if (record.activity === "loading") return loadingBorderElementClass;
  if (record.activity === "waiting") return waitingBorderElementClass;
  return undefined;
}

export function TextView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "text" }>;
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, `node-${record.id}`);
  return (
    <p className={cn("text-foreground", component.emphasize ? "font-semibold" : "text-sm")} data-ui-node-id={record.id} data-ui-node-key={record.key} {...aria}>
      {describedBy}
      {component.value}
    </p>
  );
}

export function ButtonView({ record, context }: { readonly record: UiNodeRecord; readonly context: UiInterpreterContext }) {
  const component = record.component as Extract<Component, { type: "button" }>;
  const id = nodeDomId(context.store, record, context.domScope);
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, id);
  const [, render] = useState(0);
  const mounted = useRef(true);
  const pending = useRef({ store: context.store, owner: context.localDocumentOwner, key: record.key, busy: false });
  if (pending.current.store !== context.store || pending.current.owner !== context.localDocumentOwner || pending.current.key !== record.key) pending.current = { store: context.store, owner: context.localDocumentOwner, key: record.key, busy: false };
  const state = pending.current;
  useEffect(() => { mounted.current = true; return () => { mounted.current = false; }; }, []);
  const activate = () => {
    const current = context.store.getNodeSnapshot(record.id);
    if (state.busy || !current || current.key !== record.key || current.component.type !== "button" || current.disabled) return;
    state.busy = true;
    render((value) => value + 1);
    const finish = () => { if (pending.current !== state || !mounted.current) return; state.busy = false; render((value) => value + 1); };
    try { const completion = dispatchTrigger(context, current, "activate"); if (completion instanceof Promise) void completion.then(finish, finish); else finish(); }
    catch (error) { finish(); throw error; }
  };
  return (
    <>
      <Button
        id={id}
        data-ui-node-id={record.id} data-ui-node-key={record.key}
        data-tone={record.style.tone ?? undefined}
        text={component.label}
        icon={resolveControlIconNode(component.icon)}
        disabled={record.disabled || state.busy}
        aria-label={record.accessibility.label ?? undefined}
        aria-describedby={aria["aria-describedby"] as string | undefined}
        className={activityBorderClass(record)}
        aria-busy={state.busy || record.activity === "loading" || record.activity === "waiting" || undefined}
        onClick={activate}
      />
      {describedBy}
    </>
  );
}

export function KeyValueListView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "keyValueList" }>;
  return (
    <dl className="grid grid-cols-[auto_1fr] gap-x-single gap-y-single text-xs" data-ui-node-id={record.id} data-ui-node-key={record.key}>
      {component.entries.map((entry, index) => (
        <div key={`${entry.label}:${index}`} className="contents">
          <dt className="text-muted-foreground">{entry.label}</dt>
          <dd className="tabular-nums">{entry.value}</dd>
        </div>
      ))}
    </dl>
  );
}

export function ImageView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "image" }>;
  return <img id={`node-${record.id}`} src={component.src} alt={component.alt ?? ""} className="max-h-64 max-w-full rounded-md object-contain" data-ui-node-id={record.id} data-ui-node-key={record.key} />;
}

/** 📶️ `Component::Progress` → a real `role="progressbar"`. Determinate bars carry
 * `aria-valuemin`/`max`/`now`/`valuetext` and a fill sized by the shared fraction law; indeterminate bars
 * carry only `aria-busy` and a centred sweep that pulses only when the user has not asked for reduced
 * motion (`motion-safe:`). */
export function ProgressView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "progress" }>;
  const value = uiAccessibilityValueV1(component);
  const fraction = uiProgressFractionV1(component.completed, component.total);
  const { props: aria, describedBy } = accessibilityAriaProps(record.accessibility, `node-${record.id}`);
  return (
    <div
      role="progressbar"
      data-ui-node-id={record.id}
      data-ui-node-key={record.key}
      aria-valuemin={value.valueMin ?? undefined}
      aria-valuemax={value.valueMax ?? undefined}
      aria-valuenow={value.valueNow ?? undefined}
      aria-valuetext={value.valueText ?? undefined}
      aria-busy={value.busy ? true : undefined}
      {...aria}
      className="bg-muted h-tiny w-full min-w-0 overflow-hidden rounded-full"
    >
      {describedBy}
      <div data-slot="progress-fill" className={fraction === null ? "bg-accent mx-auto h-full w-1/3 motion-safe:animate-pulse" : "bg-accent h-full"} style={fraction === null ? undefined : { width: `${fraction * 100}%` }} />
    </div>
  );
}

export function ExtensionView({ record }: { readonly record: UiNodeRecord }) {
  const component = record.component as Extract<Component, { type: "extension" }>;
  const Host = resolveExtensionHost(component.extension);
  if (Host) return <Host value={component.props} nodeId={record.id} nodeKey={record.key} />;
  return (
    <ShellFaultBoundary boundaryId={`extension-${component.extension}`} fallbackLabel={shellLabel("ui.common.renderError")}>
      <p className="text-muted-foreground text-xs" data-ui-node-id={record.id} data-ui-node-key={record.key}>
        Extension unavailable: {component.extension}
      </p>
    </ShellFaultBoundary>
  );
}

/** 🚧️ An unregistered/unknown `Component::type` — never renders nothing (a silent blank is the
 * failure mode that makes a missing renderer look like a broken document, per the packet brief). */
export function UnknownComponentView({ record }: { readonly record: UiNodeRecord }) {
  const kind = (record.component as { readonly type: string }).type;
  console.error(`Interpreter: unknown component type ${JSON.stringify(kind)} on node ${record.id} ("${record.key}")`);
  return (
    <div role="alert" className="border-destructive text-destructive rounded-md border border-dashed p-single text-xs" data-ui-node-id={record.id} data-ui-node-key={record.key} data-unknown-component={kind}>
      Unrecognized component: {kind}
    </div>
  );
}
//#endregion ComponentRenderers

export function interpretUiNodeBusyShell(record: UiNodeRecord): ReactNode | null {
  if (record.component.type === "progress" || (record.activity !== "loading" && record.activity !== "waiting")) return null;
  return (
    <div data-ui-node-id={record.id} data-ui-node-key={record.key} data-ui-status={record.activity} className={cn("p-single w-full min-w-0", record.activity === "waiting" ? waitingBorderElementClass : loadingBorderElementClass)} role="status" aria-busy="true">
      {elementSkeleton(record.component.type as ElementSkeletonKind)}
    </div>
  );
}
//#endregion 🔖️UiInterpreter
