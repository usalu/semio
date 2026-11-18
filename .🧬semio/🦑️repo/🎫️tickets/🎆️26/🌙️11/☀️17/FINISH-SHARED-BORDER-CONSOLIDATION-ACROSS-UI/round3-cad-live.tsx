import * as React from "react";
import { createRoot } from "react-dom/client";
import { Button, Input, Select, SelectContent, SelectItem, SelectTrigger, SelectValue, SharedBorderRow, SharedBorderRowCell, SharedBorderRowField, SharedBorderRowLabel, ToggleGroup, MenuItem, floatingPaneAsideClass, cn } from "@semio-tech/ui-react";
import { InferencePortPanel } from "../../../../../../../✏️s/🔌️plugins/🌍️gis/🗿️artifacts/🗺️gismap/💡️inference/🪟️presentation/🟦️.tsx";

const kinds = ["vertex", "edge", "wire", "face", "shell", "solid", "anchor", "object", "curve", "surface", "mesh"];
function Section({ title }: { title: string }) {
  const [on, setOn] = React.useState<string[]>(kinds.slice(0, 5));
  return (
    <SharedBorderRow orientation="vertical">
      <SharedBorderRowField className="text-foreground flex-none font-semibold"><input type="checkbox" aria-label={title} />{title}</SharedBorderRowField>
      <ToggleGroup kind="multiple" wrap aria-label={title} value={on} onValueChange={setOn} items={kinds.map((k) => ({ value: k, icon: on.includes(k) ? "check" : "square", text: k }))} />
    </SharedBorderRow>
  );
}
function Aside() {
  const [open, setOpen] = React.useState(true);
  return (
    <SharedBorderRow as="aside" orientation="vertical" data-level="pane" className={cn(floatingPaneAsideClass, "h-[560px] w-[360px]")}>
      <SharedBorderRowCell className="shrink-0"><strong className="text-sm font-semibold">Editor</strong></SharedBorderRowCell>
      <SharedBorderRow orientation="vertical" className="min-h-0 flex-1 overflow-auto [&>*]:shrink-0">
        <SharedBorderRow wrap>
          {["A move", "B copy", "C rotate", "D scale", "E mirror", "F array"].map((t) => <Button key={t} type="button" variant="outline" icon="arrow-right" className="h-auto px-single py-half text-xs">{t}</Button>)}
        </SharedBorderRow>
        <SharedBorderRow data-level="menu">
          <Input id="cmd" type="text" placeholder="Action" aria-label="Action" className="bg-transparent" />
          <Button type="button" variant="outline" icon={open ? "chevron-up" : "chevron-down"} className="flex-none p-0 text-2xs" aria-label="toggle" onClick={() => setOpen((o) => !o)} />
          {open ? (
            <div className="absolute inset-x-0 top-full z-[3]">
              <SharedBorderRow role="menu" orientation="vertical" edges={["right", "bottom", "left"]} className={floatingPaneAsideClass} data-level="menu">
                <SharedBorderRow orientation="vertical" className="max-h-40 overflow-y-auto [&>*]:shrink-0">
                  {["box", "line", "move"].map((k) => <MenuItem key={k} data-shared-border-cell="" className="rounded-none px-single py-single"><div className="flex items-center gap-single"><kbd className="text-muted-foreground inline-block min-w-6 text-center text-2xs font-bold uppercase">{k[0]}</kbd><span>{k}</span></div></MenuItem>)}
                </SharedBorderRow>
              </SharedBorderRow>
            </div>
          ) : null}
        </SharedBorderRow>
        <SharedBorderRow>
          <SharedBorderRowLabel htmlFor="md-trigger">Model definition</SharedBorderRowLabel>
          <Select id="md" value="a"><SelectTrigger className="h-medium w-full flex-1"><SelectValue /></SelectTrigger><SelectContent><SelectItem value="a">Shape (a)</SelectItem></SelectContent></Select>
        </SharedBorderRow>
        <SharedBorderRow>
          <SharedBorderRowLabel htmlFor="attr">Height</SharedBorderRowLabel>
          <Input id="attr" type="number" className="h-medium w-full" />
          <Button type="button" variant="outline" icon="x" className="flex-none h-auto px-half py-0 text-2xs">Clear</Button>
        </SharedBorderRow>
        <SharedBorderRow><SharedBorderRowLabel htmlFor="b">Flag</SharedBorderRowLabel><SharedBorderRowField><input id="b" type="checkbox" />Enabled</SharedBorderRowField></SharedBorderRow>
        <Section title="Primitives · Show" />
        <Section title="Primitives · Filter" />
        <SharedBorderRowCell className="text-muted-foreground block text-xs">Interaction x · state idle · rev 3</SharedBorderRowCell>
        <SharedBorderRowCell className="block text-xs"><strong>Last response</strong><pre className="text-2xs">{"{}"}</pre></SharedBorderRowCell>
      </SharedBorderRow>
    </SharedBorderRow>
  );
}
function App() {
  const status = { phase: "offered", jobId: "0123456789abcdef0123456789abcdef", cursor: 4, completed: 1, total: 3, proposalHash: "0123456789abcdef0123456789abcdef0123456789abcdef0123456789abcdef", cancelRequested: false, code: null } as never;
  return (
    <div className="semio-scope" style={{ padding: 0 }}>
      <Aside />
      <InferencePortPanel status={status} locale="en" onAction={() => {}} />
    </div>
  );
}
createRoot(document.getElementById("root")!).render(<App />);
