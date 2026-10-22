/** 🧾️ Row actions: the tree and table row action builders, the table row action button and the admission controller.
 *
 * 🧱️ Child module of {@link ../🟦️.tsx | Interpreter}; importers import this module directly.
 */

import {
  useContext,
  useEffect,
  useId,
  useMemo,
  useRef,
  useSyncExternalStore,
  type ReactElement
} from "../../⚛️react-port/🟦️.ts";
import { rowActionAdmissionKeyV1, rowActionAdmissionRegistryV1, type RowActionAdmissionOwnerV1 } from "../🎬️row-action-admission/🟦️.ts";
import { Button, DisabledReasonHint } from "@semio-tech/ui-react";
import { type ActionBinding } from "@semio-tech/framework";
import { rowActionBinding } from "../../📃️UiDocumentStore/🟦️.tsx";
import { type Component, type RowAction, type RowTarget, type UiNodeRecord } from "@semio-tech/framework";
import { wireLabel } from "../🌉️scene-host-contract/🟦️.ts";
import { RowActionRefusalsContext } from "../👥️presence-overlay/🟦️.ts";
import { type UiInterpreterContext } from "../🧷️interpreter-context/🟦️.ts";
import { resolveControlIconNode } from "../⚡️action-dispatch/🟦️.tsx";

//#region 🔖️UiInterpreter
//#region ComponentRenderers
//#region Tree
/** 🎬️ A tree row's actions as `TreeDataItem` buttons — each its verb on the row's ONE target, dispatched exactly as a table
 * row with the same target dispatches it; a disabled action renders disabled and its dispatch is refused. */
export function rowTreeActions(record: UiNodeRecord, props: Extract<Component, { type: "treeItem" }>, context: UiInterpreterContext, admission?: RowActionAdmissionControllerV1) {
  const target = props.target;
  if (!target || (props.rowActions ?? []).length === 0) return undefined;
  return (props.rowActions ?? []).map((action, actionIndex) => {
    const busy = admission?.isPending(target, action) ?? false;
    const unavailable = admission ? !admission.active : false;
    const refused = admission?.refusedBy(action);
    return { kind: "button" as const, icon: resolveControlIconNode(action.icon, 12), title: action.label ? wireLabel(action.label) : undefined, placement: action.placement ?? "row", busy, disabled: action.disabled === true || refused !== undefined || busy || unavailable, reason: action.disabled && action.reason ? wireLabel(action.reason) : refused, onClick: () => admission ? admission.dispatch(record.id, record.key, actionIndex, rowActionAdmissionKeyV1(target, action)) : dispatchCurrentRowAction(record.id, record.key, actionIndex, context, rowActionAdmissionKeyV1(target, action)) };
  });
}

/** 🎬️ One table row action, named `"<label>: <row name>"`. Only a retired document's action is natively disabled. A
 * disabled one — and one whose own dispatch is still pending, which is busy — stays focusable — `aria-disabled`, never run —
 * names its reason through `aria-describedby` and shows it as visible text while hovered, focused or pressed
 * (`DisabledReasonHint`), exactly as a tree row's action (`🌳️Tree`) and the wgpu renderer do (`💬️row-semantics`). */
export function TableRowActionButton({ action, name, busy, unavailable, refused, onRun }: { readonly action: RowAction; readonly name: string; readonly busy: boolean; readonly unavailable: boolean; readonly refused: string | undefined; readonly onRun: () => void }): ReactElement {
  const reasonId = `${useId()}-reason`;
  const label = action.label ? `${wireLabel(action.label)}: ${name}` : name;
  const reason = action.disabled === true && action.reason ? wireLabel(action.reason) : refused;
  const disabled = action.disabled === true || refused !== undefined || busy || unavailable;
  const button = (
    <Button
      type="button"
      variant="ghost"
      data-table-row-action=""
      tabIndex={0}
      icon={resolveControlIconNode(action.icon)}
      aria-label={label}
      title={reason ? undefined : label}
      disabled={unavailable}
      aria-disabled={disabled ? true : undefined}
      aria-busy={busy || undefined}
      aria-describedby={reason ? reasonId : undefined}
      className={disabled ? "cursor-not-allowed opacity-50" : undefined}
      onClick={() => {
        if (!disabled) onRun();
      }}
    />
  );
  return (
    <DisabledReasonHint id={reasonId} reason={reason}>
      {button}
    </DisabledReasonHint>
  );
}

export type RowActionAdmissionControllerV1 = Readonly<{
  active: boolean;
  isPending(target: RowTarget, action: RowAction): boolean;
  refusedBy(action: RowAction): string | undefined;
  dispatch(recordId: UiNodeRecord["id"], recordKey: string, actionIndex: number, authoredKey: string): void;
}>;

function currentRowAction(recordId: UiNodeRecord["id"], recordKey: string, actionIndex: number, context: UiInterpreterContext, authoredKey?: string): { readonly record: UiNodeRecord; readonly target: RowTarget; readonly action: RowAction; readonly binding: ActionBinding } | null {
  const current = context.store.getNodeSnapshot(recordId);
  if (!current || current.disabled || current.key !== recordKey || (current.component.type !== "treeItem" && current.component.type !== "tableRow")) return null;
  const target = current.component.target;
  const action = current.component.rowActions?.[actionIndex];
  if (!target || !action || (authoredKey !== undefined && rowActionAdmissionKeyV1(target, action) !== authoredKey)) return null;
  const resolved = rowActionBinding(target, action);
  return resolved.ok ? { record: current, target, action, binding: resolved.binding } : null;
}

/** 🎬️ Dispatches from the Store's current record, never the callback's rendered target/action snapshot. */
function dispatchCurrentRowAction(recordId: UiNodeRecord["id"], recordKey: string, actionIndex: number, context: UiInterpreterContext, authoredKey?: string): ReturnType<UiInterpreterContext["onIntent"]> | undefined {
  const current = currentRowAction(recordId, recordKey, actionIndex, context, authoredKey);
  return current ? context.onIntent(context.store.buildIntent(current.record, current.binding)) : undefined;
}

export function useRowActionAdmissionV1(context: UiInterpreterContext): RowActionAdmissionControllerV1 {
  const fallback = useRef<(RowActionAdmissionOwnerV1 & { retire(): void }) | null>(null);
  if (!fallback.current) {
    const listeners = new Set<() => void>();
    let active = true;
    fallback.current = { get active() { return active; }, subscribeRetirement: (listener) => { if (!active) { listener(); return () => {}; } listeners.add(listener); return () => listeners.delete(listener); }, retire: () => { if (!active) return; active = false; for (const listener of [...listeners]) listener(); listeners.clear(); } };
  }
  useEffect(() => () => fallback.current?.retire(), []);
  const owner = context.localDocumentOwner ?? fallback.current;
  const registry = useMemo(() => rowActionAdmissionRegistryV1(owner, context.store), [owner, context.store]);
  const epoch = useSyncExternalStore(registry.subscribe, registry.snapshot, registry.snapshot);
  const refusals = useContext(RowActionRefusalsContext);
  return useMemo(() => ({
    active: registry.active,
    isPending: (target: RowTarget, action: RowAction) => registry.isPending(rowActionAdmissionKeyV1(target, action)),
    refusedBy: (action: RowAction) => refusals?.get(action.verb),
    dispatch: (recordId, recordKey, actionIndex, authoredKey) => {
      const current = currentRowAction(recordId, recordKey, actionIndex, context, authoredKey);
      if (!current || !registry.active || refusals?.has(current.action.verb)) return;
      const token = registry.begin(rowActionAdmissionKeyV1(current.target, current.action));
      if (!token) return;
      try {
        const completion = context.onIntent(context.store.buildIntent(current.record, current.binding));
        if (completion && typeof (completion as PromiseLike<unknown>).then === "function") void Promise.resolve(completion).then(token.finish, token.finish);
        else token.finish();
      } catch (error) {
        token.finish();
        throw error;
      }
    },
  }), [context, epoch, registry, refusals]);
}
//#endregion Tree
//#endregion ComponentRenderers
//#endregion 🔖️UiInterpreter
