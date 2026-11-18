import * as React from "react";
import { createRoot } from "react-dom/client";
import { PresenceBar } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/👥️PresenceBar/🟦️.tsx";
import { Dialog, DialogContent, DialogHeader, DialogTitle, DialogDescription, DialogFooter } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/💬️Dialog/🟦️.tsx";
import { UIDialog } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📨️UIDialog/🟦️.tsx";
import { Popover, PopoverContent, PopoverTrigger } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🗨️Popover/🟦️.tsx";
import { Tabs, TabsList, TabsTrigger, TabsContent } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/📑️Tabs/🟦️.tsx";
import { App } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎨️Canvas/🟦️.tsx";
import { Button } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔘️Button/🟦️.tsx";
import { Input } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/✏️Input/🟦️.tsx";
import { PanelTreeSkeleton, elementSkeleton } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🦴️Skeletons/🟦️.tsx";
import { Tree } from "/Users/ueli/Documents/semio/🧰️framework/🔨️modules/🖱️ui/🧱️elements/🌳️Tree/🟦️.tsx";

const peers = [{ actor: "a", label: "Ada Lovelace", color: 1 }, { actor: "b", label: "Grace Hopper", color: 2, isAgent: true }, { actor: "c", label: "Linus T", color: 3, activity: { text: "Editing", badge: "E" } }, { actor: "d", label: "Four", color: 4 }];
const dialog = { id: "d", title: { native: { en: "Create" } }, body: { native: { en: "Choose." } }, args: [{ id: "name", label: { native: { en: "Name" } }, required: true, schema: { kind: "string", options: [] } }, { id: "kind", label: { native: { en: "Kind" } }, required: false, schema: { kind: "string", options: [] } }], submitAction: "s", submitLabel: { native: { en: "Create" } }, cancelAction: "c", cancelLabel: { native: { en: "Cancel" } } } as never;

function Showcase({ part }: { part: string }) {
  if (part === "presence") return <div style={{ padding: 40 }}><PresenceBar peers={peers} max={3} /></div>;
  if (part === "tabs") return <div style={{ padding: 40, width: 400 }}><Tabs defaultValue="a"><TabsList><TabsTrigger value="a">Alpha</TabsTrigger><TabsTrigger value="b">Beta</TabsTrigger></TabsList><TabsContent value="a"><div style={{ padding: 8 }}>Panel</div></TabsContent></Tabs></div>;
  if (part === "app") return <div style={{ height: 300, width: 500 }}><App modes={[{ id: "a", label: "A", children: <div /> }, { id: "b", label: "B", children: <div /> }]} activeModeId="a" onActiveModeChange={() => {}} /></div>;
  if (part === "skeleton") return <div style={{ padding: 40, width: 400 }}><PanelTreeSkeleton />{elementSkeleton("field")}{elementSkeleton("group")}</div>;
  if (part === "popover") return <div style={{ padding: 40 }}><Popover defaultOpen><PopoverTrigger>Open</PopoverTrigger><PopoverContent><Input aria-label="n" /><Button icon="check" text="Apply" /></PopoverContent></Popover></div>;
  if (part === "dialog") return <Dialog open><DialogContent><DialogHeader><DialogTitle>Title</DialogTitle><DialogDescription>Text</DialogDescription></DialogHeader><DialogFooter><Button icon="x" text="Cancel" /><Button icon="check" text="Save" /></DialogFooter></DialogContent></Dialog>;
  if (part === "uidialog") return <UIDialog dialog={dialog} renderField={(_d, v, c, f) => <Input id={f.id} aria-labelledby={f.labelledBy} value={String(v ?? "")} onChange={(e) => c(e.target.value)} />} onSubmit={() => {}} onChoose={() => {}} onCancel={() => {}} />;
  return <div style={{ padding: 40, width: 400 }}><Tree.Files title="Files" nodes={[{ name: "a.ts", path: "a.ts", type: "file" }] as never} /><Tree.Files title="More" nodes={[{ name: "b.ts", path: "b.ts", type: "file" }] as never} /></div>;
}

let root: ReturnType<typeof createRoot> | undefined;
let host: HTMLDivElement | undefined;
export function show(part: string) {
  root?.unmount(); host?.remove();
  for (const el of document.body.children) if (el instanceof HTMLElement && el.id === "root") el.style.display = "none";
  host = document.createElement("div"); host.id = "ui-static-live"; host.style.cssText = "position:fixed;inset:0;overflow:auto;background:var(--background,#fff);z-index:1";
  document.body.append(host);
  root = createRoot(host);
  root.render(<Showcase part={part} />);
}
