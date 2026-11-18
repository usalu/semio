import * as React from "react";
import { ShellScopeProvider, createShellScope, UiI18nProvider } from "@semio-tech/ui-react";
import { createMemoryStoragePort } from "@semio-tech/framework";
import { buildFrameworkSyncUtilities } from "@semio-tech/framework/backbone";
import { StagedActionDialog } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/📝️staging/🟦️.tsx";
import { OwnedShellDialog } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🏛️ShellHost/🗨️dialog-origin/🌐️browser/🟦️.tsx";
import { UtilityTree, groupUtilityNodesByCategory } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎛️UtilityTree/🟦️.tsx";
import { SelectionUtilityOptions } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🛠️ShellHelpers/🪟️window-panes/🪛️utility-bar-options/🟦️.tsx";
import { TaskManagerPanel, TaskManagerTasksPanel } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🧵️TaskManager/🟦️.tsx";
import { AgentChatPanel } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/💬️AgentChatPanel/🟦️.tsx";
import { SyncAttachCard } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔄️ShellSync/🟦️.tsx";
import { UISearch, UIFind, UIFindProvider, useUIFind } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🔎️ShellSearch/🟦️.tsx";
import { HubFirstRun } from "../../../../../../../🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/🧑‍🎨engine/🧱️elements/🎓️HubFirstRun/🟦️.tsx";

const noop = () => {};
const scope = createShellScope({ storage: createMemoryStoragePort(), initialLocale: "en" });
const Scoped = ({ children }: { children: React.ReactNode }) => {
  const root = React.useRef<HTMLDivElement>(null);
  const layer = React.useRef<HTMLDivElement>(null);
  const [ready, setReady] = React.useState(false);
  React.useLayoutEffect(() => { scope.rootRef.current = root.current; scope.portalLayerRef.current = layer.current; setReady(true); }, []);
  return <div ref={root} className="semio-scope relative" style={{ height: "100vh" }}><div ref={layer} data-slot="portal-layer" />{ready ? <ShellScopeProvider scope={scope}><UiI18nProvider instance={scope.i18n}>{children}</UiI18nProvider></ShellScopeProvider> : null}</div>;
};
const panelBox = (children: React.ReactNode, width = 420) => <div style={{ height: "100vh", display: "flex", alignItems: "flex-start" }}><div data-shared-border-frame="" className="flex min-h-0 flex-col" style={{ width: "min(100%, " + width + "px)" }}>{children}</div></div>;

const stagedHost = (state: unknown) => ({ subscribe: () => noop, snapshot: () => state, cancel: noop, execute: noop }) as never;
const dialogOwner = { openingId: 1, dialogId: "createArtifact", origin: {} as never, seedArgs: { name: "Facade" } };
const dialogDef = { id: "createArtifact", title: { native: { en: "Create Artifact", de: "Artefakt erstellen" } }, args: [{ id: "name", label: { native: { en: "Name", de: "Name" } }, required: true, schema: { kind: "string", options: [] } }, { id: "kind", label: { native: { en: "Kind", de: "Art" } }, required: false, schema: { kind: "string", options: [] } }], submitAction: "createArtifact", submitLabel: { native: { en: "Create", de: "Erstellen" } }, cancelAction: "cancelArtifact" } as never;

const utilities = [
  { id: "sel", kind: "collection", iconId: "mouse-pointer", label: "Selection", category: "selection", children: [{ id: "sel.marquee", kind: "toggle", iconId: "box-select", label: "Marquee", pressed: true, category: "selection", onChange: { controllerId: "w", action: "set" } }, { id: "sel.lasso", kind: "toggle", iconId: "lasso", label: "Lasso", category: "selection", onChange: { controllerId: "w", action: "set" } }] },
  { id: "u.measure", kind: "button", iconId: "ruler", label: "Measure", category: "utilities", onPress: { controllerId: "w", action: "measure" } },
  { id: "u.grid", kind: "toggle", iconId: "grid-3x3", label: "Grid", pressed: false, category: "utilities", onChange: { controllerId: "w", action: "grid" } },
] as never;

const tmRows = [
  { actorId: "plugin.cad#1", packageId: "s.cad", lane: "interactive", status: "active", stage: "healthy", shard: 0, wallUsP95: 120, mailboxLen: 0, turns: 4012, traps: 0, restarts: 0 },
  { actorId: "plugin.flow#2", packageId: "s.flow", lane: "background", status: "suspended", stage: "throttled", shard: 1, wallUsP95: null, mailboxLen: null, turns: null, traps: null, restarts: null },
] as never;
const tmTasks = [
  { id: "t1", lane: "job", title: "Generate mesh", owner: "s.cad", startedAtMs: Date.now() - 4000, steps: 12, progress: null, suspendable: true, state: "running" },
  { id: "t2", lane: "export", title: "Render video", owner: "os", startedAtMs: Date.now() - 9000, steps: null, progress: { completed: 3, total: 10, text: "Frame 3 of 10" }, suspendable: false, state: "running" },
] as never;
const conversation = [
  { kind: "userMessage", id: "u1", text: "Add a column to the facade.", atMs: 1 },
  { kind: "toolCall", id: "c1", toolName: "artifact.create", args: '{"kind":"column"}', state: "running", summary: null, atMs: 2 },
  { kind: "toolCall", id: "c2", toolName: "artifact.move", args: "{}", state: "ok", summary: "Moved 1 item", atMs: 3 },
  { kind: "approval", id: "a1", summary: "Delete 3 items", state: "pending", decision: null, withdrawal: null, atMs: 4 },
  { kind: "agentMessage", id: "m1", text: "Done. The column was added.", state: "complete", inReplyTo: null, atMs: 5 },
] as never;

const searchItems = [{ id: "i1", label: "Open file", description: "Open a document", category: "File", onSelect: noop }, { id: "i2", label: "Save", description: "Save the document", category: "File", onSelect: noop }, { id: "i3", label: "Toggle theme", category: "View", onSelect: noop }];
function FindSeed() { const f = useUIFind(); React.useEffect(() => { f.setFindItems(searchItems as never); }, []); return null; }

export const moreParts: Record<string, () => React.ReactElement> = {
  "staged-dialog": () => <StagedActionDialog host={stagedHost({ locale: "en", controls: null, pending: false, executing: false, closing: false, error: "The surface changed. Try again.", ready: true })} />,
  "staged-dialog-pending": () => <StagedActionDialog host={stagedHost({ locale: "de", controls: null, pending: true, executing: false, closing: false, error: null, ready: false })} />,
  "owned-dialog": () => <Scoped><OwnedShellDialog owner={dialogOwner as never} dialog={dialogDef} renderField={(_d: never, value: unknown, onChange: (v: unknown) => void, field: { id: string; labelledBy: string; required: boolean }) => <input id={field.id} aria-labelledby={field.labelledBy} required={field.required} value={String(value ?? "")} onChange={e => onChange(e.target.value)} />} isCurrent={() => true} close={() => true} dispatch={noop} /></Scoped>,
  "utility-tree": () => <Scoped><div style={{ padding: 0 }}><UtilityTree utilities={groupUtilityNodesByCategory(utilities, ["selection", "utilities"])} onAction={noop} /></div></Scoped>,
  "utility-selection": () => <Scoped><div style={{ width: 420 }}><SelectionUtilityOptions activeUtilityId="selectMarquee" windowId="w1" generation={1} onAction={noop} /></div></Scoped>,
  "task-manager": () => <Scoped>{panelBox(<><TaskManagerPanel rows={tmRows} onAction={noop} /><TaskManagerTasksPanel tasks={tmTasks} controls={{ onCancel: noop, onSuspend: noop, onResume: noop }} /></>, 640)}</Scoped>,
  "task-manager-empty": () => <Scoped>{panelBox(<><TaskManagerPanel rows={[] as never} onAction={noop} runtimeAttached={false} /><TaskManagerTasksPanel tasks={[]} controls={{ onCancel: noop, onSuspend: noop, onResume: noop }} /></>, 640)}</Scoped>,
  "agent-chat": () => <Scoped>{panelBox(<AgentChatPanel status="open" presence={{ active: true, label: "Builder", invocationId: "c1" }} conversation={conversation} onSendMessage={() => true} onCancelToolCall={() => true} onResolveApproval={noop} />, 420)}</Scoped>,
  "agent-chat-empty": () => <Scoped>{panelBox(<AgentChatPanel status="closed" presence={{ active: false, label: "", invocationId: null }} conversation={[]} onSendMessage={() => false} />, 420)}</Scoped>,
  "sync-attach": () => <Scoped><div style={{ height: "100vh", paddingTop: 400 }}><SyncAttachCard activeUri="file:///project/doc.json" cardKind="file" draftPath="/project/doc.json" syncUtilities={buildFrameworkSyncUtilities("file:///project/doc.json") as never} status={null} quarantinedConflicts={[]} onAction={noop} onDraftPathChange={noop} onClose={noop} onAttach={noop} onDetach={noop} onBrowsePath={noop} /></div></Scoped>,
  "sync-attach-remote": () => <Scoped><div style={{ height: "100vh", paddingTop: 400 }}><SyncAttachCard activeUri={null} cardKind="remote" draftPath="" syncUtilities={buildFrameworkSyncUtilities(null) as never} status={null} quarantinedConflicts={[]} onAction={noop} onDraftPathChange={noop} onClose={noop} onAttach={noop} onDetach={noop} /></div></Scoped>,
  "hub-first-run": () => <Scoped><div style={{ height: "100vh" }}><HubFirstRun locale="en" state={{ signedIn: false, spaceCount: 0, openSpaceId: null, canInvite: false }} onDismiss={noop} initialStepIndex={0} /></div></Scoped>,
  "hub-first-run-1": () => <Scoped><div style={{ height: "100vh" }}><HubFirstRun locale="en" state={{ signedIn: false, spaceCount: 0, openSpaceId: null, canInvite: false }} onDismiss={noop} initialStepIndex={1} /></div></Scoped>,
  "hub-first-run-3": () => <Scoped><div style={{ height: "100vh" }}><HubFirstRun locale="en" state={{ signedIn: false, spaceCount: 0, openSpaceId: null, canInvite: false }} onDismiss={noop} initialStepIndex={3} /></div></Scoped>,
  "hub-first-run-de": () => <Scoped><div style={{ height: "100vh" }}><HubFirstRun locale="de" state={{ signedIn: false, spaceCount: 0, openSpaceId: null, canInvite: false }} onDismiss={noop} initialStepIndex={0} /></div></Scoped>,
  "palette-scoped": () => <Scoped><UISearch items={searchItems} open onOpenChange={noop} placeholder="Search commands" emptyMessage="None" /></Scoped>,
  "find-scoped": () => <Scoped><UIFindProvider><FindSeed /><UIFind open onOpenChange={noop} placeholder="Find" emptyMessage="None" /></UIFindProvider></Scoped>,
  "hub-first-run-last": () => <Scoped><div style={{ height: "100vh" }}><HubFirstRun locale="de" state={{ signedIn: true, spaceCount: 1, openSpaceId: "s", canInvite: true }} onDismiss={noop} /></div></Scoped>,
};
