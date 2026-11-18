import * as React from "react";
import { createRoot } from "react-dom/client";
import { Engagement } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🎮️Engagement/🟦️.tsx";
import { Search } from "../../../../../../../🧰️framework/🔨️modules/🖱️ui/🧱️elements/🔍️WindowSearch/🟦️.tsx";
import { UiMobileProvider, UiI18nProvider, createShellI18nInstance, Popover, PopoverContent, PopoverTrigger, Input, Textarea, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, Toggle, Tree, Button, Window, Layout, Mode, Navbar, Footer, LayeredOverview, OverviewCard, OverviewCardAction, SharedBorderRow, SharedBorderRowCell, Icon, uiDataLabel, type LayeredPane } from "@semio-tech/ui-react";

const g = globalThis as unknown as { __r3?: { root: ReturnType<typeof createRoot>; host: HTMLElement } };
const i18n = createShellI18nInstance("en");
const opts = [{ id: "a", icon: "check", label: "Apply" }, { id: "b", icon: "x", label: "Cancel" }] as never;
const box = (children: React.ReactNode) => <div style={{ position: "absolute", left: 0, top: 0, width: 320 }}>{children}</div>;
const status = [{ id: "s", content: "Ready" }];
const panes: LayeredPane[] = ["one", "two", "three"].map((id, i) => ({ id, label: id, icon: "info" as never, render: () => <div className="p-large">{id}</div> }));
const labels = { grid: "Grid", overview: "Overview", waiting: (p: LayeredPane) => p.label, failed: (p: LayeredPane) => p.label, neighbour: (p: LayeredPane) => p.label };
const items = [{ value: "x", label: uiDataLabel("Alpha") }, { value: "y", label: uiDataLabel("Beta") }] as never;

const parts: Record<string, React.ComponentType> = {
  engagementFull: () => box(<Engagement sessionActive options={opts} status={status} control={{ kind: "stepper", label: "Height", value: 3, min: 0, max: 10, step: 1 } as never} controls={[{ kind: "select", id: "sel", label: "Mode", value: "x", items: [{ id: "x", value: "x", label: "Alpha" }], onChange: () => {} } as never]} />),
  engagementOptions: () => box(<Engagement options={opts} />),
  search: () => box(<Search sessionActive input={{ value: "", placeholder: "Search", onChange: () => {} } as never} />),
  window: () => <div style={{ height: "100vh" }}><Layout canvas={<Mode windows={[{ id: "w", iconId: "app-window", title: uiDataLabel("W"), engagement: { options: opts } as never, children: <div>body</div> } as never]} activeWindowId="w" />} /></div>,
  treeFiles: () => box(<><Tree.Files title="Files A" nodes={[{ title: "a.ts", path: "a.ts" }, { title: "src", path: "src", isFolder: true, children: [{ title: "b.ts", path: "src/b.ts" }] }] as never} /><Tree.Files title="Files B" nodes={[{ title: "c.ts", path: "c.ts" }] as never} /></>),
  overview: () => <div className="relative" style={{ height: "100vh" }}><LayeredOverview panes={panes} cells={{ one: { column: 0, row: 0 }, two: { column: 1, row: 0 }, three: { column: 2, row: 0 } }} labels={labels as never} routing="none" openedId="two" renderCard={((p: LayeredPane) => <OverviewCard as="section" slot="c" headingId={p.id} headingLevel={3} title={p.label} footerRight={<OverviewCardAction primary>Open</OverviewCardAction>}><p>x</p></OverviewCard>) as never} /></div>,
  popover: () => <div style={{ padding: 120 }}><Popover open><PopoverTrigger asChild><Button icon="info" text="Trigger" /></PopoverTrigger><PopoverContent side="bottom" align="start"><div className="p-single">Body</div></PopoverContent></Popover></div>,
  select: () => <div style={{ padding: 120, width: 300 }}><Select defaultOpen defaultValue="x"><SelectTrigger aria-label="s"><SelectValue /></SelectTrigger><SelectContent><SelectItem value="x">Alpha</SelectItem><SelectItem value="y">Beta</SelectItem></SelectContent></Select></div>,
  toggle: () => <div style={{ padding: 120, width: 300 }}><Toggle kind="dropdown" items={items} defaultValue="x" open onOpenChange={() => {}} /></div>,
  fields: () => box(<div className="flex flex-col"><Input id="i1" aria-label="idle" defaultValue="idle" /><Input id="i2" aria-label="invalid" aria-invalid defaultValue="invalid" /><Textarea id="t1" aria-label="ta" defaultValue="text" /><Textarea id="t2" aria-label="tai" aria-invalid defaultValue="bad" /></div>),
  bands: () => box(<div className="flex flex-col"><div className="p-large">editor</div><SharedBorderRow edges={["top"]}><SharedBorderRowCell className="flex-1 text-xs text-destructive">failure</SharedBorderRowCell><Button icon="x" variant="outline" text="Discard" /></SharedBorderRow><SharedBorderRow edges={["top"]} className="flex-wrap justify-end"><SharedBorderRowCell className="mr-auto text-xs">Applying</SharedBorderRowCell><SharedBorderRow framed={false}><Button icon="undo-2" variant="outline" text="Discard" /><Button icon="check" text="Apply" /></SharedBorderRow></SharedBorderRow><SharedBorderRow edges={["top"]}><SharedBorderRowCell className="text-xs">[error] diag</SharedBorderRowCell></SharedBorderRow></div>),
};
export function show(part: string) {
  g.__r3?.root.unmount(); g.__r3?.host.remove();
  for (const el of document.body.children) if (el instanceof HTMLElement && el.id === "root") el.style.display = "none";
  const host = document.createElement("div"); host.id = "r3fw-live"; host.style.cssText = "position:fixed;inset:0;overflow:auto;background:var(--background,#fff);z-index:0";
  document.body.append(host);
  const root = createRoot(host);
  g.__r3 = { root, host };
  const P = parts[part];
  if (!P) throw new Error("unknown part " + part);
  root.render(<UiI18nProvider instance={i18n}><UiMobileProvider mobile={innerWidth < 768}><P /></UiMobileProvider></UiI18nProvider>);
}
export const partNames = Object.keys(parts);
